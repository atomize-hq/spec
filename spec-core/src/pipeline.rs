use anyhow::{Context, Result, bail};
use std::collections::HashMap;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;
use wait_timeout::ChildExt;

/// Exit code used when a cargo subprocess is killed due to timeout.
/// Matches the convention used by the POSIX `timeout(1)` command.
const TIMEOUT_EXIT_CODE: i32 = 124;

/// Controls whether `run_cargo_build` and `run_cargo_test` emit status lines
/// to stderr. Use `Normal` for interactive CLI invocations; use `Silent` when
/// the caller manages its own output format (e.g., `--format json` on build/test,
/// once those flags are added).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verbosity {
    /// Emit "spec: running cargo …" lines to stderr.
    Normal,
    /// Suppress all status output from the pipeline helpers.
    Silent,
}

pub struct CargoResult {
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
    pub timed_out: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedCargoTestResult {
    pub status: String,
    pub reason: Option<String>,
}

pub fn cargo_available() -> bool {
    Command::new("cargo").arg("--version").output().is_ok()
}

/// Find the crate root for a given path.
///
/// Prefers the nearest member crate over the workspace root so that
/// `spec build`/`spec test` scope to a single crate rather than building
/// the entire workspace.
///
/// Walk:
/// 1. Find the nearest ancestor `Cargo.toml` that has `[package]` but not
///    `[workspace]` — this is a workspace member or a bare crate. Return it.
/// 2. If no such ancestor exists, fall back to the nearest `[workspace]` root.
/// 3. Otherwise bail.
pub fn workspace_root_for(path: &Path) -> Result<PathBuf> {
    let start = if path.is_file() {
        path.parent().unwrap_or(path)
    } else {
        path
    };

    let mut workspace_root: Option<PathBuf> = None;

    for dir in start.ancestors() {
        let candidate = dir.join("Cargo.toml");
        if candidate.is_file() {
            let contents = std::fs::read_to_string(&candidate)
                .with_context(|| format!("Failed to read {}", candidate.display()))?;
            let has_workspace = contents.contains("[workspace]");
            let has_package = contents.contains("[package]");

            if has_package && !has_workspace {
                // Member crate or bare crate — prefer this over workspace root
                return Ok(dir.to_path_buf());
            }
            if has_workspace && workspace_root.is_none() {
                workspace_root = Some(dir.to_path_buf());
            }
        }
    }

    if let Some(root) = workspace_root {
        return Ok(root);
    }

    bail!(
        "❌ could not find crate root — run from inside a Cargo project, or pass --crate-root <path>"
    )
}

pub fn run_cargo_build(
    crate_root: &Path,
    cargo_target_dir: &Path,
    timeout: Option<Duration>,
    verbosity: Verbosity,
) -> Result<CargoResult> {
    if matches!(verbosity, Verbosity::Normal) {
        eprintln!("spec: running cargo build in {}", crate_root.display());
    }
    run_cargo(crate_root, &["build"], cargo_target_dir, timeout)
}

pub fn run_cargo_test(
    crate_root: &Path,
    cargo_target_dir: &Path,
    filter: Option<&str>,
    timeout: Option<Duration>,
    verbosity: Verbosity,
) -> Result<CargoResult> {
    if matches!(verbosity, Verbosity::Normal) {
        eprintln!("spec: running cargo test in {}", crate_root.display());
    }
    let args = cargo_test_args(filter);
    let arg_refs = args.iter().map(|arg| arg.as_str()).collect::<Vec<_>>();
    run_cargo(crate_root, &arg_refs, cargo_target_dir, timeout)
}

fn cargo_test_args(filter: Option<&str>) -> Vec<String> {
    let mut args = vec!["test".to_string()];
    if let Some(filter) = filter {
        args.push("--".to_string());
        args.push(filter.to_string());
    }
    args
}

/// Returns true when the filter matched no tests across all test binaries.
///
/// In a multi-binary crate, cargo emits one `test result:` summary per binary.
/// We return true only when at least one summary exists AND none of them show
/// a passed count > 0. This avoids false-positives where one binary matched
/// nothing while another binary ran the filtered tests.
pub fn zero_tests_ran(output: &str) -> bool {
    let mut has_any_result = false;
    for line in output.lines() {
        let Some(summary) = line.strip_prefix("test result: ") else {
            continue;
        };
        has_any_result = true;
        // If any binary ran at least one test the filter matched, return false.
        // The first `;`-delimited segment is "ok. N passed" or "FAILED. N passed";
        // strip the status prefix before parsing.
        let passed = summary.split(';').map(str::trim).find_map(|part| {
            let part = part
                .strip_prefix("ok. ")
                .or_else(|| part.strip_prefix("FAILED. "))
                .unwrap_or(part);
            part.strip_suffix(" passed")
                .and_then(|n| n.trim().parse::<u32>().ok())
        });
        if passed.is_some_and(|n| n > 0) {
            return false;
        }
    }
    has_any_result
}

pub fn parse_cargo_test_output(stdout: &str) -> HashMap<String, ParsedCargoTestResult> {
    let mut results: HashMap<String, ParsedCargoTestResult> = HashMap::new();

    for line in stdout.lines() {
        let Some(rest) = line.strip_prefix("test ") else {
            continue;
        };
        let Some((full_name, terminal_status)) = rest.split_once(" ... ") else {
            continue;
        };

        let parsed = match terminal_status.trim() {
            "ok" => ParsedCargoTestResult {
                status: "pass".to_string(),
                reason: None,
            },
            "FAILED" => ParsedCargoTestResult {
                status: "fail".to_string(),
                reason: None,
            },
            other => ParsedCargoTestResult {
                status: "error".to_string(),
                reason: Some(other.to_string()),
            },
        };

        match results.get_mut(full_name) {
            Some(existing) => {
                existing.status = "error".to_string();
                existing.reason = Some("multiple matching cargo results".to_string());
            }
            None => {
                results.insert(full_name.to_string(), parsed);
            }
        }
    }

    results
}

fn run_cargo(
    cwd: &Path,
    args: &[&str],
    cargo_target_dir: &Path,
    timeout: Option<Duration>,
) -> Result<CargoResult> {
    run_command(Path::new("cargo"), cwd, args, cargo_target_dir, timeout)
}

fn run_command(
    program: &Path,
    cwd: &Path,
    args: &[&str],
    cargo_target_dir: &Path,
    timeout: Option<Duration>,
) -> Result<CargoResult> {
    let mut child = Command::new(program)
        .current_dir(cwd)
        .env("CARGO_TARGET_DIR", cargo_target_dir)
        .env("CARGO_TERM_COLOR", "never")
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .with_context(|| format!("failed to spawn {}", program.display()))?;

    let stdout = child
        .stdout
        .take()
        .context("failed to capture child stdout")?;
    let stderr = child
        .stderr
        .take()
        .context("failed to capture child stderr")?;
    let stdout_handle = thread::spawn(move || read_pipe(stdout));
    let stderr_handle = thread::spawn(move || read_pipe(stderr));

    let (status, timed_out) = match timeout {
        Some(timeout) => match child.wait_timeout(timeout)? {
            Some(status) => (status, false),
            None => {
                let _ = child.kill();
                (child.wait()?, true)
            }
        },
        None => (child.wait()?, false),
    };

    // On timeout, do NOT join the pipe reader threads. cargo may have spawned
    // grandchildren (rustc, test binaries) that inherited the pipe write-ends.
    // Those grandchildren remain alive after we kill cargo, keeping the pipes
    // open and causing read_to_end to block indefinitely — defeating the timeout.
    // We abandon the threads here; they will be cleaned up when the process
    // exits (shortly after the caller bails on the timeout error).
    let (stdout, stderr) = if timed_out {
        drop(stdout_handle);
        drop(stderr_handle);
        let timeout_secs = timeout.expect("timed_out implies Some timeout").as_secs();
        let stderr_msg = format!(
            "spec: cargo {} timed out after {}s\n",
            args.join(" "),
            timeout_secs
        );
        (String::new(), stderr_msg)
    } else {
        let stdout =
            String::from_utf8_lossy(&join_pipe_reader(stdout_handle, "stdout")?).into_owned();
        let stderr =
            String::from_utf8_lossy(&join_pipe_reader(stderr_handle, "stderr")?).into_owned();
        (stdout, stderr)
    };

    Ok(CargoResult {
        exit_code: if timed_out {
            TIMEOUT_EXIT_CODE
        } else {
            status.code().unwrap_or(1)
        },
        stdout,
        stderr,
        timed_out,
    })
}

fn read_pipe<R: Read>(mut reader: R) -> std::io::Result<Vec<u8>> {
    let mut bytes = Vec::with_capacity(65536);
    reader.read_to_end(&mut bytes)?;
    Ok(bytes)
}

fn join_pipe_reader(
    handle: thread::JoinHandle<std::io::Result<Vec<u8>>>,
    name: &str,
) -> Result<Vec<u8>> {
    handle
        .join()
        .map_err(|_| anyhow::anyhow!("failed to join {name} reader thread"))?
        .with_context(|| format!("failed to read child {name}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    #[cfg(unix)]
    use std::os::unix::fs::PermissionsExt;
    use tempfile::TempDir;

    #[test]
    fn workspace_root_for_prefers_member_crate_over_workspace() {
        let tmp = TempDir::new().unwrap();
        let member = tmp.path().join("crates/foo");
        fs::create_dir_all(&member).unwrap();
        fs::write(
            tmp.path().join("Cargo.toml"),
            "[workspace]\nmembers = [\"crates/foo\"]\n",
        )
        .unwrap();
        fs::write(
            member.join("Cargo.toml"),
            "[package]\nname = \"foo\"\nversion = \"0.1.0\"\n",
        )
        .unwrap();

        // Should return the member crate, not the workspace root, so spec
        // build/test scopes to this crate rather than the whole workspace.
        let root = workspace_root_for(&member).unwrap();
        assert_eq!(root, member);
    }

    #[test]
    fn workspace_root_for_falls_back_to_workspace_when_no_member() {
        let tmp = TempDir::new().unwrap();
        let units = tmp.path().join("units");
        fs::create_dir_all(&units).unwrap();
        fs::write(
            tmp.path().join("Cargo.toml"),
            "[workspace]\nmembers = [\"crates/foo\"]\n",
        )
        .unwrap();

        // No member Cargo.toml in ancestors — falls back to workspace root.
        let root = workspace_root_for(&units).unwrap();
        assert_eq!(root, tmp.path());
    }

    #[test]
    fn workspace_root_for_falls_back_to_package_toml_for_bare_crate() {
        let tmp = TempDir::new().unwrap();
        let src = tmp.path().join("src");
        fs::create_dir_all(&src).unwrap();
        fs::write(
            tmp.path().join("Cargo.toml"),
            "[package]\nname = \"solo\"\nversion = \"0.1.0\"\n",
        )
        .unwrap();

        let root = workspace_root_for(&src).unwrap();
        assert_eq!(root, tmp.path());
    }

    #[test]
    fn workspace_root_for_errors_with_no_cargo_toml() {
        let tmp = TempDir::new().unwrap();
        let nested = tmp.path().join("a/b/c");
        fs::create_dir_all(&nested).unwrap();

        let err = workspace_root_for(&nested).unwrap_err().to_string();
        assert!(err.contains("could not find crate root"), "got: {err}");
    }

    #[test]
    fn parse_cargo_test_output_parses_pass_and_fail() {
        let stdout = "\
running 2 tests
test generated::pricing::apply_discount::tests::test_happy_path ... ok
test generated::pricing::apply_tax::tests::test_basic_tax ... FAILED
";

        let parsed = parse_cargo_test_output(stdout);
        assert_eq!(
            parsed.get("generated::pricing::apply_discount::tests::test_happy_path"),
            Some(&ParsedCargoTestResult {
                status: "pass".to_string(),
                reason: None,
            })
        );
        assert_eq!(
            parsed.get("generated::pricing::apply_tax::tests::test_basic_tax"),
            Some(&ParsedCargoTestResult {
                status: "fail".to_string(),
                reason: None,
            })
        );
    }

    #[test]
    fn parse_cargo_test_output_ignores_non_test_lines() {
        let stdout = "\
running 1 test

failures:

---- generated::pricing::apply_tax::tests::test_basic_tax stdout ----

thread 'generated::pricing::apply_tax::tests::test_basic_tax' panicked at src/lib.rs:10:9:
assertion failed: false
test result: FAILED. 0 passed; 1 failed
";

        let parsed = parse_cargo_test_output(stdout);
        assert!(parsed.is_empty(), "got: {parsed:?}");
    }

    #[test]
    fn parse_cargo_test_output_handles_duplicate_test_ids_across_units() {
        let stdout = "\
test generated::pricing::apply_discount::tests::test_happy_path ... ok
test generated::checkout::apply_discount::tests::test_happy_path ... FAILED
";

        let parsed = parse_cargo_test_output(stdout);
        assert_eq!(
            parsed.get("generated::pricing::apply_discount::tests::test_happy_path"),
            Some(&ParsedCargoTestResult {
                status: "pass".to_string(),
                reason: None,
            })
        );
        assert_eq!(
            parsed.get("generated::checkout::apply_discount::tests::test_happy_path"),
            Some(&ParsedCargoTestResult {
                status: "fail".to_string(),
                reason: None,
            })
        );
    }

    #[test]
    fn parse_cargo_test_output_marks_duplicate_full_names_as_error() {
        let stdout = "\
test generated::pricing::apply_discount::tests::test_happy_path ... ok
test generated::pricing::apply_discount::tests::test_happy_path ... FAILED
";

        let parsed = parse_cargo_test_output(stdout);
        assert_eq!(
            parsed.get("generated::pricing::apply_discount::tests::test_happy_path"),
            Some(&ParsedCargoTestResult {
                status: "error".to_string(),
                reason: Some("multiple matching cargo results".to_string()),
            })
        );
    }

    #[test]
    fn parse_cargo_test_output_marks_unrecognized_terminal_status_as_error() {
        let stdout = "test generated::pricing::apply_tax::tests::test_basic_tax ... IGNORED\n";

        let parsed = parse_cargo_test_output(stdout);
        assert_eq!(
            parsed.get("generated::pricing::apply_tax::tests::test_basic_tax"),
            Some(&ParsedCargoTestResult {
                status: "error".to_string(),
                reason: Some("IGNORED".to_string()),
            })
        );
    }

    #[test]
    fn parse_cargo_test_output_handles_real_corpus_shape() {
        let stdout = "\
running 3 tests
test generated::pricing::apply_discount::tests::test_happy_path ... ok
test generated::pricing::apply_tax::tests::test_basic_tax ... ok
test generated::pricing::calculate_total::tests::test_combined_flow ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
";

        let parsed = parse_cargo_test_output(stdout);
        assert_eq!(parsed.len(), 3);
        assert_eq!(
            parsed.get("generated::pricing::calculate_total::tests::test_combined_flow"),
            Some(&ParsedCargoTestResult {
                status: "pass".to_string(),
                reason: None,
            })
        );
    }

    #[test]
    fn test_zero_tests_ran_detects_empty_run() {
        let stdout = "\
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
";

        assert!(zero_tests_ran(stdout));
    }

    #[test]
    fn test_zero_tests_ran_false_for_passing_tests() {
        let stdout = "\
running 3 tests
test generated::pricing::apply_discount::tests::test_happy_path ... ok
test generated::pricing::apply_tax::tests::test_basic_tax ... ok
test generated::pricing::calculate_total::tests::test_combined_flow ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
";

        assert!(!zero_tests_ran(stdout));
    }

    #[test]
    fn test_zero_tests_ran_false_when_one_binary_matches_in_multi_binary_crate() {
        // One binary (integration tests) matched 0; the other (lib) matched the target.
        // Should return false — the filter DID match something.
        let stdout = "\
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

running 1 test
test generated::pricing::apply_tax::tests::test_happy_path ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
";
        assert!(!zero_tests_ran(stdout));
    }

    #[test]
    fn test_zero_tests_ran_true_when_all_binaries_match_nothing() {
        // Both binaries ran 0 matching tests — filter matched nothing anywhere.
        let stdout = "\
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s
";
        assert!(zero_tests_ran(stdout));
    }

    #[test]
    fn test_zero_tests_ran_false_for_no_result_lines() {
        // No test result lines at all (empty output / build-only) — not a zero-tests-ran situation.
        assert!(!zero_tests_ran(""));
        assert!(!zero_tests_ran(
            "Compiling spec-core v0.5.0\nFinished dev profile\n"
        ));
    }

    #[test]
    fn test_run_cargo_test_with_filter_appends_filter_arg() {
        assert_eq!(
            cargo_test_args(Some("generated::pricing::apply_tax::tests::")),
            vec![
                "test".to_string(),
                "--".to_string(),
                "generated::pricing::apply_tax::tests::".to_string(),
            ]
        );

        assert_eq!(cargo_test_args(None), vec!["test".to_string()]);
    }

    #[cfg(unix)]
    fn write_fake_command(dir: &TempDir, name: &str, body: &str) -> PathBuf {
        let path = dir.path().join(name);
        fs::write(&path, body).unwrap();
        let mut perms = fs::metadata(&path).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&path, perms).unwrap();
        path
    }

    #[cfg(unix)]
    #[test]
    fn run_command_marks_timeout_and_reports_it() {
        let tmp = TempDir::new().unwrap();
        // Command sleeps 10s so the 2s timeout fires first; as_secs() returns "2s" in message.
        let fake_cargo = write_fake_command(&tmp, "fake-cargo.sh", "#!/bin/sh\nsleep 10\n");

        let result = run_command(
            &fake_cargo,
            tmp.path(),
            &["build"],
            &tmp.path().join("target"),
            Some(Duration::from_secs(2)),
        )
        .unwrap();

        assert!(result.timed_out);
        assert_eq!(result.exit_code, TIMEOUT_EXIT_CODE);
        assert!(
            result.stderr.contains("timed out after 2s"),
            "{}",
            result.stderr
        );
    }

    #[cfg(unix)]
    #[test]
    fn run_command_preserves_output_without_timeout() {
        let tmp = TempDir::new().unwrap();
        let fake_cargo = write_fake_command(
            &tmp,
            "fake-cargo.sh",
            "#!/bin/sh\necho ok-stdout\necho ok-stderr >&2\n",
        );

        let result = run_command(
            &fake_cargo,
            tmp.path(),
            &["build"],
            &tmp.path().join("target"),
            Some(Duration::from_secs(1)),
        )
        .unwrap();

        assert!(!result.timed_out);
        assert_eq!(result.exit_code, 0);
        assert!(result.stdout.contains("ok-stdout"));
        assert!(result.stderr.contains("ok-stderr"));
    }
}
