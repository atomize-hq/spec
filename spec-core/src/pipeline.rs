use anyhow::{Context, Result, bail};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct CargoResult {
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
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

pub fn run_cargo_build(crate_root: &Path, cargo_target_dir: &Path) -> Result<CargoResult> {
    eprintln!("spec: running cargo build in {}", crate_root.display());
    run_cargo(crate_root, &["build"], cargo_target_dir)
}

pub fn run_cargo_test(
    crate_root: &Path,
    cargo_target_dir: &Path,
    filter: Option<&str>,
) -> Result<CargoResult> {
    eprintln!("spec: running cargo test in {}", crate_root.display());
    let args = cargo_test_args(filter);
    let arg_refs = args.iter().map(|arg| arg.as_str()).collect::<Vec<_>>();
    run_cargo(crate_root, &arg_refs, cargo_target_dir)
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

pub fn parse_cargo_test_output(stdout: &str) -> BTreeMap<String, ParsedCargoTestResult> {
    let mut results: BTreeMap<String, ParsedCargoTestResult> = BTreeMap::new();

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

fn run_cargo(cwd: &Path, args: &[&str], cargo_target_dir: &Path) -> Result<CargoResult> {
    let output = Command::new("cargo")
        .current_dir(cwd)
        .env("CARGO_TARGET_DIR", cargo_target_dir)
        .env("CARGO_TERM_COLOR", "never")
        .args(args)
        .output()
        .with_context(|| "failed to spawn cargo")?;

    Ok(CargoResult {
        exit_code: output.status.code().unwrap_or(1),
        stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
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
}
