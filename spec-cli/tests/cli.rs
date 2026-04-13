use serde_json::Value;
use spec_core::AUTHORED_SPEC_VERSION;
use std::fs;
use std::io;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use walkdir::WalkDir;

fn bin() -> PathBuf {
    if let Ok(path) = std::env::var("CARGO_BIN_EXE_spec") {
        return path.into();
    }

    if let Ok(path) = std::env::var("CARGO_BIN_EXE_spec-cli") {
        return path.into();
    }

    let mut path = std::env::current_exe().expect("failed to locate test binary");
    path.pop();
    path.pop();
    path.push(if cfg!(windows) { "spec.exe" } else { "spec" });
    path
}

fn run(args: &[&str]) -> std::process::Output {
    Command::new(bin())
        .args(args)
        .output()
        .expect("failed to run spec")
}

fn run_in(cwd: &Path, args: &[&str]) -> std::process::Output {
    Command::new(bin())
        .current_dir(cwd)
        .args(args)
        .output()
        .expect("failed to run spec")
}

fn run_in_with_env(
    cwd: &Path,
    args: &[&str],
    envs: &[(&str, &std::ffi::OsStr)],
) -> std::process::Output {
    let mut command = Command::new(bin());
    command.current_dir(cwd).args(args);
    for (key, value) in envs {
        command.env(key, value);
    }
    command.output().expect("failed to run spec")
}

fn assert_output_success(context: &str, output: &std::process::Output) {
    if output.status.success() {
        return;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    panic!("{context}\n\nstdout:\n{stdout}\n\nstderr:\n{stderr}");
}

fn temp_repo_dir() -> tempfile::TempDir {
    tempfile::TempDir::new_in(std::env::current_dir().unwrap()).unwrap()
}

fn git_available() -> bool {
    Command::new("git").arg("--version").output().is_ok()
}

fn write_spec(dir: &Path, relative_path: &str, body: &str) {
    let path = dir.join(relative_path);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(path, body).unwrap();
}

fn write_file(dir: &Path, relative_path: &str, body: &str) {
    let path = dir.join(relative_path);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(path, body).unwrap();
}

fn run_git(cwd: &Path, args: &[&str]) -> std::process::Output {
    Command::new("git")
        .current_dir(cwd)
        .args(args)
        .output()
        .expect("failed to run git")
}

fn init_git_repo(cwd: &Path) -> String {
    assert_output_success("git init failed", &run_git(cwd, &["init", "-b", "main"]));
    assert_output_success(
        "git config user.email failed",
        &run_git(cwd, &["config", "user.email", "spec-tests@example.com"]),
    );
    assert_output_success(
        "git config user.name failed",
        &run_git(cwd, &["config", "user.name", "Spec Tests"]),
    );
    assert_output_success("git add failed", &run_git(cwd, &["add", "."]));
    assert_output_success(
        "git commit failed",
        &run_git(cwd, &["commit", "-m", "test fixture"]),
    );

    let rev_parse = run_git(cwd, &["rev-parse", "HEAD"]);
    assert_output_success("git rev-parse failed", &rev_parse);
    String::from_utf8(rev_parse.stdout)
        .unwrap()
        .trim()
        .to_string()
}

#[cfg(unix)]
fn write_executable_file(dir: &Path, relative_path: &str, body: &str) {
    let path = dir.join(relative_path);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(&path, body).unwrap();
    let mut perms = fs::metadata(&path).unwrap().permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&path, perms).unwrap();
}

fn parse_stdout_json(output: &std::process::Output) -> Value {
    serde_json::from_slice(&output.stdout).unwrap()
}

fn fixture_json(name: &str) -> Value {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name);
    serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap()
}

fn assert_stdout_json_matches_fixture(output: &std::process::Output, fixture: &str) {
    let actual = parse_stdout_json(output);
    let expected = fixture_json(fixture);
    assert_eq!(actual, expected);
}

fn rewrite_passport_generated_at(passport_path: &Path, generated_at: &str) {
    let mut passport: Value = serde_json::from_str(&fs::read_to_string(passport_path).unwrap())
        .expect("passport should be valid JSON");
    passport["generated_at"] = Value::String(generated_at.to_string());
    if let Some(evidence) = passport.get_mut("evidence") {
        evidence["observed_at"] = Value::String(generated_at.to_string());
    }
    fs::write(
        passport_path,
        serde_json::to_string_pretty(&passport).unwrap(),
    )
    .unwrap();
}

fn write_status_project(project_dir: &Path) -> PathBuf {
    let units_dir = project_dir.join("units");
    let pricing_dir = units_dir.join("pricing");
    let src_dir = project_dir.join("src");

    fs::create_dir_all(&pricing_dir).unwrap();
    fs::create_dir_all(&src_dir).unwrap();

    fs::write(
        project_dir.join("Cargo.toml"),
        "[package]\nname = \"pricing-project\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[workspace]\n",
    )
    .unwrap();
    fs::write(
        src_dir.join("main.rs"),
        "mod generated;\npub use generated::*;\nfn main() {}\n",
    )
    .unwrap();
    fs::write(
        pricing_dir.join("apply_discount.unit.spec"),
        r#"
id: pricing/apply_discount
kind: function
intent:
  why: Apply a discount.
spec_version: "0.3.0"
contract:
  returns: bool
body:
  rust: |
    { true }
local_tests:
  - id: happy_path
    expect: apply_discount() == true
"#,
    )
    .unwrap();

    pricing_dir.join("apply_discount.unit.spec")
}

#[test]
fn help_lists_validate_and_generate_commands() {
    let output = run(&["--help"]);
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("validate"));
    assert!(stdout.contains("generate"));
    assert!(stdout.contains("export"));
}

#[test]
fn version_reports_binary_version() {
    let output = run(&["--version"]);
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn validate_single_file_succeeds() {
    let temp_dir = temp_repo_dir();
    let spec_path = temp_dir.path().join("apply_discount.unit.spec");
    fs::write(
        &spec_path,
        r#"
id: pricing/apply_discount
kind: function
intent:
  why: Apply a discount.
body:
  rust: |
    { }
"#,
    )
    .unwrap();

    let output = run(&["validate", spec_path.to_str().unwrap()]);
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("1 unit valid"));
}

#[test]
fn generate_single_file_writes_output_tree() {
    let temp_dir = temp_repo_dir();
    let units_dir = temp_dir.path().join("units");
    let output_dir = temp_dir.path().join("generated/spec");
    write_spec(
        &units_dir,
        "pricing/apply_discount.unit.spec",
        r#"
id: pricing/apply_discount
kind: function
intent:
  why: Apply a discount.
body:
  rust: |
    { }
"#,
    );

    let output = run(&[
        "generate",
        units_dir.to_str().unwrap(),
        "--output",
        output_dir.to_str().unwrap(),
    ]);
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Generated 3 files"));
    assert!(output_dir.join(".spec-generated").exists());
    assert!(output_dir.join("pricing/apply_discount.rs").exists());
    assert!(output_dir.join("pricing/mod.rs").exists());
    assert!(output_dir.join("mod.rs").exists());
}

#[test]
fn generate_single_file_path_writes_gitignore_to_parent_dir() {
    // Regression: passing a .unit.spec file path to `spec generate` must
    // write .gitignore to the file's parent directory, not try to open
    // "foo.unit.spec/.gitignore" (which would be ENOTDIR).
    let temp_dir = temp_repo_dir();
    let units_dir = temp_dir.path().join("units");
    let output_dir = temp_dir.path().join("generated/spec");
    let spec_path = units_dir.join("pricing/apply_discount.unit.spec");
    write_spec(
        &units_dir,
        "pricing/apply_discount.unit.spec",
        r#"
id: pricing/apply_discount
kind: function
intent:
  why: Apply a discount.
body:
  rust: |
    { }
"#,
    );

    let output = run(&[
        "generate",
        spec_path.to_str().unwrap(),
        "--output",
        output_dir.to_str().unwrap(),
    ]);
    assert!(
        output.status.success(),
        "expected success for single-file generate, got:\nstdout: {}\nstderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
    // .gitignore must land next to the spec file, not as a child of it
    assert!(
        units_dir.join("pricing/.gitignore").exists(),
        "expected .gitignore in units/pricing/, not an ENOTDIR"
    );
}

#[test]
fn validate_strict_errors_on_missing_dep() {
    let temp_dir = temp_repo_dir();
    let units_dir = temp_dir.path().join("units");
    write_spec(
        &units_dir,
        "pricing/apply_discount.unit.spec",
        r#"
id: pricing/apply_discount
kind: function
intent:
  why: Apply a discount.
deps:
  - money/round
body:
  rust: |
    pub fn apply_discount() {}
"#,
    );

    let output = run(&["validate", units_dir.to_str().unwrap()]);
    assert!(!output.status.success());

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("❌ dep 'money/round' not found in this spec set"),
        "expected missing-dep message in stderr, got: {stderr}"
    );
}

#[test]
fn validate_no_strict_warns_on_missing_dep_and_exits_zero() {
    let temp_dir = temp_repo_dir();
    let units_dir = temp_dir.path().join("units");
    write_spec(
        &units_dir,
        "pricing/apply_discount.unit.spec",
        r#"
id: pricing/apply_discount
kind: function
intent:
  why: Apply a discount.
deps:
  - money/round
body:
  rust: |
    { }
"#,
    );

    let output = run(&["validate", units_dir.to_str().unwrap(), "--no-strict"]);
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stdout.contains("1 unit valid with 2 warnings"), "{stdout}");
    assert!(
        stderr.contains("⚠ dep 'money/round' not found in this spec set"),
        "{stderr}"
    );
}

#[test]
fn generate_strict_errors_on_missing_dep() {
    let temp_dir = temp_repo_dir();
    let units_dir = temp_dir.path().join("units");
    let output_dir = temp_dir.path().join("generated/spec");
    write_spec(
        &units_dir,
        "pricing/apply_discount.unit.spec",
        r#"
id: pricing/apply_discount
kind: function
intent:
  why: Apply a discount.
deps:
  - money/round
body:
  rust: |
    pub fn apply_discount() {}
"#,
    );

    let output = run(&[
        "generate",
        units_dir.to_str().unwrap(),
        "--output",
        output_dir.to_str().unwrap(),
    ]);
    assert!(!output.status.success());

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("❌ dep 'money/round' not found in this spec set"),
        "expected missing-dep message in stderr, got: {stderr}"
    );
    assert!(
        !output_dir.exists(),
        "expected output dir to not be created"
    );
    assert!(!output_dir.join(".spec-generated").exists());
}

#[test]
fn generate_rejects_no_strict_flag() {
    // --no-strict is not a valid flag for `generate` — clap rejects it at parse time
    let temp_dir = temp_repo_dir();
    let units_dir = temp_dir.path().join("units");
    let output_dir = temp_dir.path().join("generated/spec");

    let output = run(&[
        "generate",
        units_dir.to_str().unwrap(),
        "--output",
        output_dir.to_str().unwrap(),
        "--no-strict",
    ]);
    assert!(!output.status.success());

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("no-strict")
            && (stderr.contains("unexpected")
                || stderr.contains("unrecognized")
                || stderr.contains("found")),
        "expected clap unknown-argument error for --no-strict, got: {stderr}"
    );
    assert!(!output_dir.exists());
}

#[test]
fn validate_help_shows_path_description() {
    let output = run(&["validate", "--help"]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("PATH"),
        "expected PATH value_name in help, got: {stdout}"
    );
    assert!(
        stdout.contains(".unit.spec"),
        "expected .unit.spec in help description, got: {stdout}"
    );
}

#[test]
fn generate_help_does_not_show_no_strict() {
    let output = run(&["generate", "--help"]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        !stdout.contains("no-strict"),
        "expected --no-strict to be absent from generate help, got: {stdout}"
    );
}

#[test]
fn generate_multiple_units_with_deps_emits_imports() {
    let temp_dir = temp_repo_dir();
    let units_dir = temp_dir.path().join("units");
    let output_dir = temp_dir.path().join("generated/spec");
    write_spec(
        &units_dir,
        "money/round.unit.spec",
        r#"
id: money/round
kind: function
intent:
  why: Round monetary values.
contract:
  inputs:
    value: Decimal
  returns: Decimal
body:
  rust: |
    {
        value
    }
"#,
    );
    write_spec(
        &units_dir,
        "pricing/apply_discount.unit.spec",
        r#"
id: pricing/apply_discount
kind: function
intent:
  why: Apply a discount.
contract:
  returns: Decimal
deps:
  - money/round
body:
  rust: |
    {
        round(Decimal::ZERO)
    }
"#,
    );

    let output = run(&[
        "generate",
        units_dir.to_str().unwrap(),
        "--output",
        output_dir.to_str().unwrap(),
    ]);
    assert!(output.status.success());

    let apply_discount = fs::read_to_string(output_dir.join("pricing/apply_discount.rs")).unwrap();
    assert!(apply_discount.contains("use crate::money::round::round;"));
    assert!(apply_discount.contains("pub fn apply_discount() -> Decimal"));

    let root_mod = fs::read_to_string(output_dir.join("mod.rs")).unwrap();
    assert!(root_mod.contains("pub mod money;"));
    assert!(root_mod.contains("pub mod pricing;"));
}

#[test]
fn validate_duplicate_local_test_ids_fails() {
    let temp_dir = temp_repo_dir();
    let units_dir = temp_dir.path().join("units");
    write_spec(
        &units_dir,
        "pricing/apply_discount.unit.spec",
        r#"
id: pricing/apply_discount
kind: function
intent:
  why: Apply a discount.
body:
  rust: |
    { true }
local_tests:
  - id: happy_path
    expect: "apply_discount()"
  - id: happy_path
    expect: "apply_discount()"
"#,
    );

    let output = run(&["validate", units_dir.to_str().unwrap()]);
    assert!(!output.status.success());

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("duplicate local_tests id 'happy_path'"),
        "{stderr}"
    );
}

#[test]
fn generate_duplicate_local_test_ids_fails_before_writing_output() {
    let temp_dir = temp_repo_dir();
    let units_dir = temp_dir.path().join("units");
    let output_dir = temp_dir.path().join("generated/spec");
    write_spec(
        &units_dir,
        "pricing/apply_discount.unit.spec",
        r#"
id: pricing/apply_discount
kind: function
intent:
  why: Apply a discount.
body:
  rust: |
    { true }
local_tests:
  - id: happy_path
    expect: "apply_discount()"
  - id: happy_path
    expect: "apply_discount()"
"#,
    );

    let output = run(&[
        "generate",
        units_dir.to_str().unwrap(),
        "--output",
        output_dir.to_str().unwrap(),
    ]);
    assert!(!output.status.success());

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("duplicate local_tests id 'happy_path'"),
        "{stderr}"
    );
    assert!(!output_dir.exists());
}

// Regression: ISSUE-001 — duplicate ID across two files showed "1 file, 1 error"
// instead of "2 files, 1 error" because the composite key "file1 | file2" counted as one map entry.
// Found by /qa on 2026-04-01
// Report: .gstack/qa-reports/qa-report-spec-2026-04-01.md
#[test]
fn validate_duplicate_id_reports_correct_file_count() {
    let temp_dir = temp_repo_dir();
    let units_dir = temp_dir.path().join("units");
    write_spec(
        &units_dir,
        "pricing/a.unit.spec",
        r#"
id: pricing/foo
kind: function
intent:
  why: First definition.
body:
  rust: |
    pub fn foo() {}
"#,
    );
    write_spec(
        &units_dir,
        "pricing/b.unit.spec",
        r#"
id: pricing/foo
kind: function
intent:
  why: Duplicate definition.
body:
  rust: |
    pub fn foo() {}
"#,
    );

    let output = run(&["validate", units_dir.to_str().unwrap()]);
    assert!(!output.status.success());

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("2 files"),
        "expected '2 files' in error output, got: {stderr}"
    );
}

#[test]
fn validate_empty_directory_reports_zero_units() {
    let temp_dir = temp_repo_dir();
    let output = run(&["validate", temp_dir.path().to_str().unwrap()]);
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("0 units found, nothing to validate"));
}

#[test]
fn spec_validate_json_all_valid() {
    let temp_dir = temp_repo_dir();
    let units_dir = temp_dir.path().join("units");
    write_spec(
        &units_dir,
        "pricing/apply_discount.unit.spec",
        r#"
id: pricing/apply_discount
kind: function
intent:
  why: Apply a discount.
spec_version: "0.3.0"
body:
  rust: |
    { }
"#,
    );

    let output = run_in(temp_dir.path(), &["validate", "units", "--format", "json"]);
    assert!(output.status.success());
    assert!(
        output.stderr.is_empty(),
        "expected no stderr output, got: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    assert_stdout_json_matches_fixture(&output, "validate-valid.json");
}

#[test]
fn spec_validate_json_missing_dep() {
    let temp_dir = temp_repo_dir();
    let units_dir = temp_dir.path().join("units");
    write_spec(
        &units_dir,
        "pricing/apply_discount.unit.spec",
        r#"
id: pricing/apply_discount
kind: function
intent:
  why: Apply a discount.
spec_version: "0.3.0"
deps:
  - currency/convert
body:
  rust: |
    { }
"#,
    );

    let output = run_in(temp_dir.path(), &["validate", "units", "--format", "json"]);
    assert!(!output.status.success());
    assert!(
        output.stderr.is_empty(),
        "expected no stderr output, got: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    assert_stdout_json_matches_fixture(&output, "validate-invalid.json");
}

#[test]
fn spec_validate_json_contract_type_invalid() {
    let temp_dir = temp_repo_dir();
    let units_dir = temp_dir.path().join("units");
    write_spec(
        &units_dir,
        "pricing/apply_discount.unit.spec",
        r#"
id: pricing/apply_discount
kind: function
intent:
  why: Apply a discount.
spec_version: "0.3.0"
contract:
  inputs:
    weight: Vec<
  returns: i32
body:
  rust: |
    { }
"#,
    );

    let output = run(&["validate", units_dir.to_str().unwrap(), "--format", "json"]);
    assert!(!output.status.success());
    assert!(
        output.stderr.is_empty(),
        "expected no stderr output, got: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let json = parse_stdout_json(&output);
    assert_eq!(json["status"], "invalid");
    assert_eq!(json["errors"].as_array().unwrap().len(), 1);
    assert_eq!(json["errors"][0]["code"], "SPEC_CONTRACT_TYPE_INVALID");
    assert_eq!(json["errors"][0]["field"], "contract.inputs.weight");
    assert_eq!(json["errors"][0]["value"], "Vec<");
}

#[test]
fn spec_validate_json_no_human_text_on_stdout() {
    let temp_dir = temp_repo_dir();
    let units_dir = temp_dir.path().join("units");
    write_spec(
        &units_dir,
        "pricing/apply_discount.unit.spec",
        r#"
id: pricing/apply_discount
kind: function
intent:
  why: Apply a discount.
spec_version: "0.3.0"
body:
  rust: |
    { }
"#,
    );

    let output = run(&["validate", units_dir.to_str().unwrap(), "--format", "json"]);
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains(r#""status": "valid""#), "{stdout}");
    assert!(!stdout.contains("units found"), "{stdout}");
    assert!(!stdout.contains("unit valid"), "{stdout}");
    assert!(
        serde_json::from_str::<Value>(&stdout).is_ok(),
        "stdout must be parseable JSON: {stdout}"
    );
}

#[test]
fn spec_validate_json_zero_units() {
    let temp_dir = temp_repo_dir();
    let output = run(&[
        "validate",
        temp_dir.path().to_str().unwrap(),
        "--format",
        "json",
    ]);
    assert!(output.status.success());

    let json = parse_stdout_json(&output);
    assert_eq!(json["schema_version"], 2);
    assert_eq!(json["status"], "valid");
    assert_eq!(json["errors"], serde_json::json!([]));
    assert_eq!(json["warnings"], serde_json::json!([]));
}

#[test]
fn spec_validate_json_schema_version_is_2() {
    let temp_dir = temp_repo_dir();
    let units_dir = temp_dir.path().join("units");
    write_spec(
        &units_dir,
        "pricing/apply_discount.unit.spec",
        r#"
id: pricing/apply_discount
kind: function
intent:
  why: Apply a discount.
spec_version: "0.3.0"
body:
  rust: |
    { }
"#,
    );

    let output = run(&["validate", units_dir.to_str().unwrap(), "--format", "json"]);
    assert!(output.status.success());

    let json = parse_stdout_json(&output);
    assert_eq!(json["schema_version"], 2);
}

#[test]
fn generate_empty_directory_reports_zero_units() {
    let temp_dir = temp_repo_dir();
    let output_dir = temp_dir.path().join("generated/spec");

    let output = run(&[
        "generate",
        temp_dir.path().to_str().unwrap(),
        "--output",
        output_dir.to_str().unwrap(),
    ]);
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("0 units found, nothing to generate"));
}

#[test]
fn spec_export_emits_valid_json_bundle() {
    let temp_dir = temp_repo_dir();
    let units_dir = temp_dir.path().join("units");
    write_spec(
        &units_dir,
        "money/round.unit.spec",
        r#"
id: money/round
kind: function
intent:
  why: Round monetary values.
contract:
  inputs:
    value: i32
  returns: i32
body:
  rust: |
    { value }
"#,
    );
    write_spec(
        &units_dir,
        "pricing/apply_tax.unit.spec",
        r#"
id: pricing/apply_tax
kind: function
intent:
  why: Apply tax.
deps:
  - money/round
body:
  rust: |
    { round(1) }
local_tests:
  - id: basic
    expect: "true"
"#,
    );

    let output = run(&["export", units_dir.to_str().unwrap()]);
    assert!(output.status.success());

    let bundle: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(bundle["schema_version"], 1);
    assert_eq!(bundle["units"].as_array().unwrap().len(), 2);
    assert!(bundle.get("graph").is_some());
    assert!(bundle.get("warnings").is_some());
    assert!(String::from_utf8_lossy(&output.stderr).contains("spec_version not set"));
}

#[test]
fn spec_export_omits_top_level_provenance_outside_git() {
    let temp_dir = tempfile::TempDir::new().unwrap();
    let units_dir = temp_dir.path().join("units");
    write_spec(
        &units_dir,
        "money/round.unit.spec",
        r#"
id: money/round
kind: function
intent:
  why: Round monetary values.
spec_version: "0.3.0"
contract:
  inputs:
    value: i32
  returns: i32
body:
  rust: |
    { value }
"#,
    );

    let output = run_in(temp_dir.path(), &["export", "units"]);
    assert!(output.status.success());

    let bundle: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert!(bundle.get("provenance").is_none());
}

#[test]
fn spec_export_includes_top_level_provenance_when_git_available() {
    if !git_available() {
        return;
    }

    let temp_dir = temp_repo_dir();
    let units_dir = temp_dir.path().join("units");
    write_spec(
        &units_dir,
        "money/round.unit.spec",
        r#"
id: money/round
kind: function
intent:
  why: Round monetary values.
spec_version: "0.3.0"
contract:
  inputs:
    value: i32
  returns: i32
body:
  rust: |
    { value }
"#,
    );

    let sha = init_git_repo(temp_dir.path());
    let output = run_in(temp_dir.path(), &["export", "units"]);
    assert!(output.status.success());

    let bundle: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(bundle["provenance"]["git_commit_sha"], sha);
}

#[test]
fn spec_export_includes_graph_edges() {
    let temp_dir = temp_repo_dir();
    let units_dir = temp_dir.path().join("units");
    write_spec(
        &units_dir,
        "money/round.unit.spec",
        r#"
id: money/round
kind: function
intent:
  why: Round monetary values.
spec_version: "0.3.0"
body:
  rust: |
    { value }
contract:
  inputs:
    value: i32
  returns: i32
"#,
    );
    write_spec(
        &units_dir,
        "pricing/apply_discount.unit.spec",
        r#"
id: pricing/apply_discount
kind: function
intent:
  why: Apply a discount.
spec_version: "0.3.0"
deps:
  - money/round
body:
  rust: |
    { round(1) }
"#,
    );

    let output = run(&["export", units_dir.to_str().unwrap()]);
    assert!(output.status.success());

    let bundle: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(
        bundle["graph"]["edges"],
        serde_json::json!([{ "from": "pricing/apply_discount", "to": "money/round" }])
    );
}

#[test]
fn spec_export_includes_passports_if_present() {
    let temp_dir = temp_repo_dir();
    let units_dir = temp_dir.path().join("units");
    let output_dir = temp_dir.path().join("generated/spec");
    write_spec(
        &units_dir,
        "pricing/apply_tax.unit.spec",
        r#"
id: pricing/apply_tax
kind: function
intent:
  why: Apply tax.
spec_version: "0.3.0"
contract:
  inputs:
    subtotal: i32
  returns: i32
body:
  rust: |
    { subtotal }
"#,
    );

    let generate = run(&[
        "generate",
        units_dir.to_str().unwrap(),
        "--output",
        output_dir.to_str().unwrap(),
    ]);
    assert_output_success("generate before export", &generate);

    let output = run(&["export", units_dir.to_str().unwrap()]);
    assert!(output.status.success());

    let bundle: Value = serde_json::from_slice(&output.stdout).unwrap();
    let passports = bundle["passports"].as_array().unwrap();
    assert_eq!(passports.len(), 1);
    assert_eq!(passports[0]["id"], "pricing/apply_tax");
    assert!(bundle["warnings"].as_array().unwrap().is_empty());
}

#[test]
fn spec_export_partial_passports_marked_missing() {
    let temp_dir = temp_repo_dir();
    let units_dir = temp_dir.path().join("units");
    let output_dir = temp_dir.path().join("generated/spec");
    write_spec(
        &units_dir,
        "money/round.unit.spec",
        r#"
id: money/round
kind: function
intent:
  why: Round money.
spec_version: "0.3.0"
contract:
  inputs:
    value: i32
  returns: i32
body:
  rust: |
    { value }
"#,
    );
    write_spec(
        &units_dir,
        "pricing/apply_tax.unit.spec",
        r#"
id: pricing/apply_tax
kind: function
intent:
  why: Apply tax.
spec_version: "0.3.0"
deps:
  - money/round
contract:
  inputs:
    subtotal: i32
  returns: i32
body:
  rust: |
    { round(subtotal) }
"#,
    );

    let generate = run(&[
        "generate",
        units_dir.to_str().unwrap(),
        "--output",
        output_dir.to_str().unwrap(),
    ]);
    assert_output_success("generate before export", &generate);
    fs::remove_file(units_dir.join("pricing/apply_tax.spec.passport.json")).unwrap();

    let output = run(&["export", units_dir.to_str().unwrap()]);
    assert!(output.status.success());

    let bundle: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(bundle["passports"].as_array().unwrap().len(), 1);
    let warnings = bundle["warnings"].as_array().unwrap();
    assert_eq!(warnings.len(), 1);
    assert_eq!(warnings[0]["code"], "passport_missing");
    assert_eq!(warnings[0]["spec_id"], "pricing/apply_tax");
}

#[test]
fn spec_export_output_path_rejects_directory() {
    let temp_dir = temp_repo_dir();
    let units_dir = temp_dir.path().join("units");
    let export_dir = temp_dir.path().join("bundle-dir");
    fs::create_dir_all(&export_dir).unwrap();
    write_spec(
        &units_dir,
        "pricing/apply_tax.unit.spec",
        r#"
id: pricing/apply_tax
kind: function
intent:
  why: Apply tax.
spec_version: "0.3.0"
body:
  rust: |
    { 1 }
"#,
    );

    let output = run(&[
        "export",
        units_dir.to_str().unwrap(),
        "--output",
        export_dir.to_str().unwrap(),
    ]);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("--output must be a file path"));
}

#[test]
fn spec_export_output_parent_dir_missing_exits_cleanly() {
    let temp_dir = temp_repo_dir();
    let units_dir = temp_dir.path().join("units");
    let output_path = temp_dir.path().join("missing/bundle.json");
    write_spec(
        &units_dir,
        "pricing/apply_tax.unit.spec",
        r#"
id: pricing/apply_tax
kind: function
intent:
  why: Apply tax.
spec_version: "0.3.0"
body:
  rust: |
    { 1 }
"#,
    );

    let output = run(&[
        "export",
        units_dir.to_str().unwrap(),
        "--output",
        output_path.to_str().unwrap(),
    ]);
    assert!(!output.status.success());
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("output directory does not exist"),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn spec_export_empty_directory_emits_valid_empty_bundle() {
    let temp_dir = temp_repo_dir();
    let output = run(&["export", temp_dir.path().to_str().unwrap()]);
    assert!(output.status.success());

    let bundle: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(bundle["units"], serde_json::json!([]));
    assert_eq!(bundle["passports"], serde_json::json!([]));
    assert_eq!(bundle["graph"]["edges"], serde_json::json!([]));
    assert_eq!(bundle["warnings"], serde_json::json!([]));
}

#[test]
fn generate_rejects_non_empty_dir_without_marker() {
    let temp_dir = temp_repo_dir();
    let units_dir = temp_dir.path().join("units");
    let output_dir = temp_dir.path().join("src");
    fs::create_dir_all(&output_dir).unwrap();
    fs::write(output_dir.join("keep.txt"), "do not touch\n").unwrap();

    write_spec(
        &units_dir,
        "pricing/apply_discount.unit.spec",
        r#"
id: pricing/apply_discount
kind: function
intent:
  why: Apply a discount.
body:
  rust: |
    pub fn apply_discount() {}
"#,
    );

    let output = run(&[
        "generate",
        units_dir.to_str().unwrap(),
        "--output",
        output_dir.to_str().unwrap(),
    ]);
    assert!(!output.status.success());

    assert!(!output_dir.join(".spec-generated").exists());
    assert!(output_dir.join("keep.txt").exists());
    assert!(!output_dir.join("pricing/apply_discount.rs").exists());
}

#[test]
fn generate_rejects_path_outside_project_root() {
    let temp_dir = temp_repo_dir();
    let units_dir = temp_dir.path().join("units");
    write_spec(
        &units_dir,
        "pricing/apply_discount.unit.spec",
        r#"
id: pricing/apply_discount
kind: function
intent:
  why: Apply a discount.
body:
  rust: |
    pub fn apply_discount() {}
"#,
    );

    // This output directory is created outside the cargo test current_dir, so it must be rejected.
    let outside = tempfile::TempDir::new().unwrap();
    let output_dir = outside.path().join("generated");
    fs::create_dir_all(&output_dir).unwrap();

    let output = run(&[
        "generate",
        units_dir.to_str().unwrap(),
        "--output",
        output_dir.to_str().unwrap(),
    ]);
    assert!(!output.status.success());

    assert!(!output_dir.join(".spec-generated").exists());
    assert!(!output_dir.join("pricing/apply_discount.rs").exists());
}

#[test]
fn validate_default_config_rejects_unsafe_expect_expression() {
    let temp_dir = temp_repo_dir();
    write_spec(
        temp_dir.path(),
        "units/pricing/apply_discount.unit.spec",
        r#"
id: pricing/apply_discount
kind: function
intent:
  why: Apply a discount.
body:
  rust: |
    { true }
local_tests:
  - id: unsafe_attempt
    expect: "{ let ok = apply_discount(); ok }"
"#,
    );

    let output = run_in(temp_dir.path(), &["validate", "units"]);
    assert!(!output.status.success());

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("block, unsafe, closure"), "{stderr}");
}

#[test]
fn validate_trusted_config_allows_unsafe_expect_expression() {
    let temp_dir = temp_repo_dir();
    write_file(
        temp_dir.path(),
        "spec.toml",
        "[validation]\nallow_unsafe_local_test_expect = true\n",
    );
    write_spec(
        temp_dir.path(),
        "units/pricing/apply_discount.unit.spec",
        r#"
id: pricing/apply_discount
kind: function
intent:
  why: Apply a discount.
body:
  rust: |
    { true }
local_tests:
  - id: unsafe_attempt
    expect: "{ let ok = apply_discount(); ok }"
"#,
    );

    let output = run_in(temp_dir.path(), &["validate", "units/pricing"]);
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("1 unit valid"), "{stdout}");
}

#[test]
fn generate_trusted_config_allows_unsafe_expect_expression() {
    let temp_dir = temp_repo_dir();
    let units_dir = temp_dir.path().join("units");
    let output_dir = temp_dir.path().join("generated/spec");
    write_file(
        temp_dir.path(),
        "spec.toml",
        "[validation]\nallow_unsafe_local_test_expect = true\n",
    );
    write_spec(
        &units_dir,
        "pricing/apply_discount.unit.spec",
        r#"
id: pricing/apply_discount
kind: function
intent:
  why: Apply a discount.
body:
  rust: |
    { true }
local_tests:
  - id: unsafe_allowed
    expect: "{ let ok = apply_discount(); ok }"
"#,
    );

    let output = run_in(
        temp_dir.path(),
        &["generate", "units", "--output", "generated/spec"],
    );
    assert_output_success(
        "spec generate should honor trusted local test config",
        &output,
    );

    let generated = fs::read_to_string(output_dir.join("pricing/apply_discount.rs")).unwrap();
    assert!(generated.contains("assert!({ let ok = apply_discount(); ok });"));
}

#[test]
fn generate_without_trusted_config_rejects_unsafe_expect_expression() {
    let temp_dir = temp_repo_dir();
    let units_dir = temp_dir.path().join("units");
    write_spec(
        &units_dir,
        "pricing/apply_discount.unit.spec",
        r#"
id: pricing/apply_discount
kind: function
intent:
  why: Apply a discount.
body:
  rust: |
    { true }
local_tests:
  - id: unsafe_rejected
    expect: "{ let ok = apply_discount(); ok }"
"#,
    );

    let output = run_in(
        temp_dir.path(),
        &["generate", "units", "--output", "generated/spec"],
    );
    assert!(!output.status.success());

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("pricing/apply_discount"), "{stderr}");
    assert!(stderr.contains("unsafe_rejected"), "{stderr}");
    assert!(stderr.contains("block, unsafe, closure"), "{stderr}");
}

#[test]
fn validate_discovers_config_from_nested_unit_file_path() {
    let temp_dir = temp_repo_dir();
    write_file(
        temp_dir.path(),
        "spec.toml",
        "[validation]\nallow_unsafe_local_test_expect = true\n",
    );
    write_spec(
        temp_dir.path(),
        "units/pricing/apply_discount.unit.spec",
        r#"
id: pricing/apply_discount
kind: function
intent:
  why: Apply a discount.
body:
  rust: |
    { true }
local_tests:
  - id: unsafe_attempt
    expect: "{ let ok = apply_discount(); ok }"
"#,
    );

    let output = run_in(
        temp_dir.path(),
        &["validate", "units/pricing/apply_discount.unit.spec"],
    );
    assert!(output.status.success());
}

#[test]
fn validate_non_function_body_reports_explicit_error() {
    let temp_dir = temp_repo_dir();
    let units_dir = temp_dir.path().join("units");
    write_spec(
        &units_dir,
        "pricing/apply_discount.unit.spec",
        r#"
id: pricing/apply_discount
kind: function
intent:
  why: Apply a discount.
body:
  rust: |
    const APPLY_DISCOUNT: bool = true;
"#,
    );

    let output = run(&["validate", units_dir.to_str().unwrap()]);
    assert!(!output.status.success());

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("body.rust must be a Rust block expression"),
        "{stderr}"
    );
}

#[test]
#[cfg(unix)]
fn generate_skips_symlink_cycle_with_warning() {
    use std::os::unix::fs as unix_fs;

    let temp_dir = temp_repo_dir();
    let units_dir = temp_dir.path().join("units");
    let output_dir = temp_dir.path().join("generated/spec");
    write_spec(
        &units_dir,
        "pricing/apply_discount.unit.spec",
        r#"
id: pricing/apply_discount
kind: function
intent:
  why: Apply a discount.
body:
  rust: |
    { }
"#,
    );

    unix_fs::symlink(&units_dir, units_dir.join("loop")).unwrap();

    let output = run(&[
        "generate",
        units_dir.to_str().unwrap(),
        "--output",
        output_dir.to_str().unwrap(),
    ]);
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stdout.contains("Generated 3 files"), "{stdout}");
    assert!(stderr.contains("skipped symlink cycle"), "{stderr}");
    assert!(output_dir.join("pricing/apply_discount.rs").exists());
}

#[test]
fn generate_is_idempotent_for_same_spec_tree() {
    let temp_dir = temp_repo_dir();
    let units_dir = temp_dir.path().join("units");
    let output_dir = temp_dir.path().join("generated/spec");
    write_spec(
        &units_dir,
        "money/round.unit.spec",
        r#"
id: money/round
kind: function
intent:
  why: Round money.
body:
  rust: |
    { }
"#,
    );
    write_spec(
        &units_dir,
        "pricing/apply_discount.unit.spec",
        r#"
id: pricing/apply_discount
kind: function
intent:
  why: Apply a discount.
deps:
  - money/round
body:
  rust: |
    {
        round();
    }
"#,
    );

    let first = run(&[
        "generate",
        units_dir.to_str().unwrap(),
        "--output",
        output_dir.to_str().unwrap(),
    ]);
    assert_output_success("first spec generate run failed", &first);
    let first_snapshot = snapshot_tree(&output_dir);

    let second = run(&[
        "generate",
        units_dir.to_str().unwrap(),
        "--output",
        output_dir.to_str().unwrap(),
    ]);
    assert_output_success("second spec generate run failed", &second);
    let second_snapshot = snapshot_tree(&output_dir);

    assert_eq!(first_snapshot, second_snapshot);
}

#[test]
fn validate_detects_cycle_in_dep_graph() {
    let temp_dir = temp_repo_dir();
    let units_dir = temp_dir.path().join("units");
    write_spec(
        &units_dir,
        "a/foo.unit.spec",
        r#"
id: a/foo
kind: function
intent:
  why: First unit in cycle.
deps:
  - b/bar
body:
  rust: |
    { }
"#,
    );
    write_spec(
        &units_dir,
        "b/bar.unit.spec",
        r#"
id: b/bar
kind: function
intent:
  why: Second unit in cycle.
deps:
  - a/foo
body:
  rust: |
    { }
"#,
    );

    let output = run(&["validate", units_dir.to_str().unwrap()]);
    assert!(!output.status.success());

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("cycle detected"),
        "expected cycle error in stderr, got: {stderr}"
    );
    assert!(
        stderr.contains("a/foo"),
        "expected a/foo in cycle path: {stderr}"
    );
    assert!(
        stderr.contains("b/bar"),
        "expected b/bar in cycle path: {stderr}"
    );
}

fn cargo_available() -> bool {
    Command::new("cargo").arg("--version").output().is_ok()
}

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("spec-cli crate should have a parent directory (repo root)")
        .to_path_buf()
}

fn run_cargo(cwd: &Path, args: &[&str], cargo_target_dir: &Path) -> std::process::Output {
    Command::new("cargo")
        .current_dir(cwd)
        .env("CARGO_TARGET_DIR", cargo_target_dir)
        .env("CARGO_TERM_COLOR", "never")
        .args(args)
        .output()
        .expect("failed to run cargo")
}

#[test]
fn generate_cargo_check_on_ecommerce() {
    if !cargo_available() {
        return;
    }

    let root = repo_root();
    let ecommerce_dir = root.join("examples/ecommerce");

    let output = run_in(
        &root,
        &[
            "generate",
            "examples/ecommerce/units",
            "--output",
            "examples/ecommerce/src/generated",
        ],
    );
    assert_output_success("spec generate failed for ecommerce example", &output);

    let cargo_target_dir = tempfile::TempDir::new_in(root.join("target"))
        .expect("failed to create temp cargo target dir under repo target/");

    let output = run_cargo(
        &ecommerce_dir,
        &["check", "--locked"],
        cargo_target_dir.path(),
    );
    assert_output_success("cargo check failed for ecommerce example", &output);

    let output = run_cargo(
        &ecommerce_dir,
        &["test", "--locked"],
        cargo_target_dir.path(),
    );
    assert_output_success("cargo test failed for ecommerce example", &output);
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> io::Result<()> {
    if src.file_name().is_some_and(|name| name == "target") {
        return Ok(());
    }

    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let file_type = entry.file_type()?;
        let dest_path = dst.join(entry.file_name());

        if file_type.is_dir() {
            copy_dir_recursive(&path, &dest_path)?;
        } else if file_type.is_file() {
            fs::copy(&path, &dest_path)?;
        }
    }

    Ok(())
}

fn snapshot_tree(root: &Path) -> Vec<(PathBuf, Vec<u8>)> {
    let mut snapshot = Vec::new();
    for entry in WalkDir::new(root).sort_by_file_name() {
        let entry = entry.expect("failed to walk generated tree");
        if !entry.file_type().is_file() {
            continue;
        }

        let path = entry.path();
        let rel = path
            .strip_prefix(root)
            .expect("generated file should live under snapshot root")
            .to_path_buf();
        snapshot.push((rel, fs::read(path).expect("failed to read generated file")));
    }
    snapshot
}

#[test]
fn generate_cargo_check_test_failure_includes_cargo_stderr() {
    if !cargo_available() {
        return;
    }

    let root = repo_root();
    let temp_dir =
        tempfile::TempDir::new_in(root.join("target")).expect("failed to create temp dir");

    let src_ecommerce = root.join("examples/ecommerce");
    let dst_ecommerce = temp_dir.path().join("ecommerce");
    copy_dir_recursive(&src_ecommerce, &dst_ecommerce).expect("failed to copy ecommerce example");

    // Add a unit that will generate uncompilable Rust (stable error: cannot find type `NotAType`)
    write_spec(
        &dst_ecommerce.join("units"),
        "pricing/bad_type.unit.spec",
        r#"
id: pricing/bad_type
kind: function
intent:
  why: Force a compile error so we can assert cargo stderr is surfaced.
contract:
  returns: NotAType
body:
  rust: |
    {
        todo!()
    }
"#,
    );

    let output = run_in(
        &dst_ecommerce,
        &["generate", "units", "--output", "src/generated"],
    );
    assert_output_success("spec generate failed for temp ecommerce copy", &output);

    let cargo_target_dir = tempfile::TempDir::new_in(root.join("target"))
        .expect("failed to create temp cargo target dir under repo target/");
    let output = run_cargo(
        &dst_ecommerce,
        &["check", "--locked"],
        cargo_target_dir.path(),
    );
    assert!(
        !output.status.success(),
        "expected cargo check to fail for a unit with unknown type"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("E0412"),
        "expected E0412 (cannot find type) in cargo stderr, got: {stderr}"
    );
}

// Regression: ISSUE-QA-002 — no CLI integration test for spec_version warning.
// validate and generate must both surface a ⚠ warning when spec_version is absent
// and still exit 0.
// Found by /qa on 2026-04-04.
#[test]
fn validate_warns_on_missing_spec_version() {
    let temp_dir = temp_repo_dir();
    let units_dir = temp_dir.path().join("units");
    write_spec(
        &units_dir,
        "pricing/apply_discount.unit.spec",
        r#"
id: pricing/apply_discount
kind: function
intent:
  why: Apply a discount.
body:
  rust: |
    { }
"#,
    );

    let output = run(&["validate", units_dir.to_str().unwrap()]);
    assert!(
        output.status.success(),
        "validate should exit 0 when spec_version is missing"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("1 unit valid with 1 warning"),
        "expected warning count in stdout, got: {stdout}"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("spec_version not set"),
        "expected spec_version warning in stderr, got: {stderr}"
    );
    // The warning should keep recommending the authored spec format version.
    assert!(
        stderr.contains(AUTHORED_SPEC_VERSION),
        "expected authored spec version ({AUTHORED_SPEC_VERSION}) in warning, got: {stderr}"
    );
}

// Regression: ISSUE-QA-003 — no CLI integration test for passport file creation.
// spec generate must write a .spec.passport.json file co-located with each .unit.spec
// and add **/*.spec.passport.json to .gitignore in the spec root.
// Found by /qa on 2026-04-04.
#[test]
fn generate_emits_passport_json_and_updates_gitignore() {
    let temp_dir = temp_repo_dir();
    let units_dir = temp_dir.path().join("units");
    let output_dir = temp_dir.path().join("generated/spec");
    write_spec(
        &units_dir,
        "pricing/apply_discount.unit.spec",
        r#"
spec_version: "0.3.0"
id: pricing/apply_discount
kind: function
intent:
  why: Apply a discount to a subtotal.
contract:
  inputs:
    subtotal: Decimal
    rate: Decimal
  returns: Decimal
body:
  rust: |
    {
        subtotal * (Decimal::ONE - rate)
    }
"#,
    );

    let output = run(&[
        "generate",
        units_dir.to_str().unwrap(),
        "--output",
        output_dir.to_str().unwrap(),
    ]);
    assert!(output.status.success());

    // Passport is co-located with the source file, not in the output dir.
    let passport_path = units_dir.join("pricing/apply_discount.spec.passport.json");
    assert!(
        passport_path.exists(),
        "expected passport file at {}, not found",
        passport_path.display()
    );

    let passport_content = fs::read_to_string(&passport_path).unwrap();
    assert!(
        passport_content.contains("\"id\": \"pricing/apply_discount\""),
        "expected id in passport: {passport_content}"
    );
    assert!(
        passport_content.contains("\"spec_version\": \"0.3.0\""),
        "expected spec_version in passport: {passport_content}"
    );
    assert!(
        passport_content.contains("\"returns\": \"Decimal\""),
        "expected returns in passport: {passport_content}"
    );

    // .gitignore entry written to the spec root (units/), not per-namespace.
    let gitignore_path = units_dir.join(".gitignore");
    assert!(
        gitignore_path.exists(),
        "expected .gitignore in units/, not found"
    );
    let gitignore = fs::read_to_string(&gitignore_path).unwrap();
    assert!(
        gitignore.contains("**/*.spec.passport.json"),
        "expected passport glob in .gitignore: {gitignore}"
    );
}

// ── D1: Pipeline wrap (spec build / spec test) ────────────────────────────────

fn write_minimal_units_dir(units_dir: &Path) {
    write_spec(
        units_dir,
        "pricing/apply_discount.unit.spec",
        r#"spec_version: "0.3.0"
id: pricing/apply_discount
kind: function
intent:
  why: Apply a discount to a subtotal.
contract:
  inputs:
    subtotal: f64
    rate: f64
  returns: f64
body:
  rust: |
    {
        subtotal * (1.0 - rate)
    }
"#,
    );
}

#[test]
fn spec_build_validates_and_runs_cargo_build() {
    if !cargo_available() {
        return;
    }

    let root = repo_root();
    let ecommerce_dir = root.join("examples/ecommerce");
    let output = run_in(
        &root,
        &[
            "build",
            "examples/ecommerce/units",
            "--output",
            "examples/ecommerce/src/generated",
            "--crate-root",
            ecommerce_dir.to_str().unwrap(),
        ],
    );
    assert_output_success("spec build failed for ecommerce example", &output);
}

#[test]
fn spec_build_fails_on_validation_error_before_cargo() {
    let temp_dir = temp_repo_dir();
    let units_dir = temp_dir.path().join("units");
    // Write a spec with a Rust reserved keyword in the id — will fail validation
    write_spec(
        &units_dir,
        "pricing/type.unit.spec",
        r#"spec_version: "0.3.0"
id: pricing/type
kind: function
intent:
  why: Force a validation error.
body:
  rust: |
    { }
"#,
    );

    let output = run(&[
        "build",
        units_dir.to_str().unwrap(),
        "--output",
        temp_dir.path().join("out").to_str().unwrap(),
    ]);
    assert!(
        !output.status.success(),
        "spec build should exit 1 on validation error"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("reserved keyword") || stderr.contains("error"),
        "expected validation error in stderr, got: {stderr}"
    );
}

#[test]
fn spec_build_unavailable_cargo_exits_cleanly() {
    let temp_dir = temp_repo_dir();
    let units_dir = temp_dir.path().join("units");
    write_minimal_units_dir(&units_dir);

    // Point PATH to an empty dir so cargo cannot be found.
    let empty_path = temp_dir.path().join("empty_bin");
    fs::create_dir_all(&empty_path).unwrap();

    let output = Command::new(bin())
        .env("PATH", &empty_path)
        .args([
            "build",
            units_dir.to_str().unwrap(),
            "--output",
            temp_dir.path().join("out").to_str().unwrap(),
        ])
        .output()
        .expect("failed to run spec");

    assert!(
        !output.status.success(),
        "spec build should exit 1 when cargo is unavailable"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("cargo not found"),
        "expected 'cargo not found' in stderr, got: {stderr}"
    );
}

#[cfg(unix)]
#[test]
fn spec_build_respects_pipeline_timeout_secs() {
    let temp_dir = temp_repo_dir();
    let project_dir = temp_dir.path();
    let units_dir = project_dir.join("units");
    write_minimal_units_dir(&units_dir);
    write_file(
        project_dir,
        "Cargo.toml",
        "[package]\nname = \"timeout-fixture\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(project_dir, "spec.toml", "[pipeline]\ntimeout_secs = 1\n");

    let fake_bin_dir = project_dir.join("fake-bin");
    write_executable_file(
        &fake_bin_dir,
        "cargo",
        "#!/bin/sh\nif [ \"$1\" = \"--version\" ]; then\n  echo 'cargo 1.89.0'\n  exit 0\nfi\n/bin/sleep 2\n",
    );
    let mut path_override = std::ffi::OsString::from(fake_bin_dir.as_os_str());
    path_override.push(":");
    path_override.push(std::env::var_os("PATH").unwrap_or_default());

    let output = run_in_with_env(
        project_dir,
        &[
            "build",
            "units",
            "--output",
            "generated/spec",
            "--crate-root",
            project_dir.to_str().unwrap(),
        ],
        &[("PATH", path_override.as_os_str())],
    );

    assert!(!output.status.success(), "build should fail on timeout");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("timed out after 1s"), "{stderr}");
    assert!(stderr.contains("cargo build timed out"), "{stderr}");
}

#[test]
fn spec_test_runs_cargo_test() {
    if !cargo_available() {
        return;
    }

    let root = repo_root();
    let ecommerce_dir = root.join("examples/ecommerce");
    let output = run_in(
        &root,
        &[
            "test",
            "examples/ecommerce/units",
            "--output",
            "examples/ecommerce/src/generated",
            "--crate-root",
            ecommerce_dir.to_str().unwrap(),
        ],
    );
    assert_output_success("spec test failed for ecommerce example", &output);
}

#[test]
fn spec_test_forwards_cargo_stderr_on_failure() {
    if !cargo_available() {
        return;
    }

    let root = repo_root();
    let temp_dir =
        tempfile::TempDir::new_in(root.join("target")).expect("failed to create temp dir");

    let src_ecommerce = root.join("examples/ecommerce");
    let dst_ecommerce = temp_dir.path().join("ecommerce");
    copy_dir_recursive(&src_ecommerce, &dst_ecommerce).expect("failed to copy ecommerce example");

    // Add a unit that produces uncompilable Rust
    write_spec(
        &dst_ecommerce.join("units"),
        "pricing/broken.unit.spec",
        r#"spec_version: "0.3.0"
id: pricing/broken
kind: function
intent:
  why: Force a compile error.
contract:
  returns: NotARealType
body:
  rust: |
    {
        todo!()
    }
"#,
    );

    let output = Command::new(bin())
        .current_dir(&dst_ecommerce)
        .args([
            "build",
            "units",
            "--output",
            "src/generated",
            "--crate-root",
            dst_ecommerce.to_str().unwrap(),
        ])
        .output()
        .expect("failed to run spec");

    assert!(
        !output.status.success(),
        "spec build should exit 1 when cargo compilation fails"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("cargo build failed") || stderr.contains("error"),
        "expected cargo error in stderr, got: {stderr}"
    );
}

#[test]
fn spec_build_rejects_single_file_path() {
    let temp_dir = temp_repo_dir();
    let units_dir = temp_dir.path().join("units");
    write_minimal_units_dir(&units_dir);
    let single_file = units_dir.join("pricing/apply_discount.unit.spec");

    let output = run(&[
        "build",
        single_file.to_str().unwrap(),
        "--output",
        temp_dir.path().join("out").to_str().unwrap(),
    ]);
    assert!(
        !output.status.success(),
        "spec build should exit 1 for a single-file path"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("directory path"),
        "expected directory path error in stderr, got: {stderr}"
    );
}

#[test]
fn spec_build_crate_root_config_vs_flag_precedence() {
    if !cargo_available() {
        return;
    }

    let root = repo_root();
    let ecommerce_dir = root.join("examples/ecommerce");
    let temp_dir = temp_repo_dir();
    let units_dir = temp_dir.path().join("units");
    write_minimal_units_dir(&units_dir);

    // Write a spec.toml with a wrong crate_root; the --crate-root flag should override it.
    fs::write(
        temp_dir.path().join("spec.toml"),
        "[pipeline]\ncrate_root = \"/nonexistent_path_that_should_be_overridden\"\n",
    )
    .unwrap();

    let output = run(&[
        "build",
        units_dir.to_str().unwrap(),
        "--output",
        temp_dir.path().join("out").to_str().unwrap(),
        "--crate-root",
        ecommerce_dir.to_str().unwrap(),
    ]);
    // The flag overrides the config, so ecommerce builds successfully.
    assert_output_success(
        "spec build should use --crate-root flag over spec.toml config",
        &output,
    );
}

#[test]
fn spec_build_no_cargo_toml_exits_with_error() {
    let temp_dir = temp_repo_dir();
    let units_dir = temp_dir.path().join("units");
    write_minimal_units_dir(&units_dir);

    // No Cargo.toml anywhere under temp_dir — ancestor walk will fail.
    // We run spec from within temp_dir and don't pass --crate-root.
    let output = Command::new(bin())
        .current_dir(temp_dir.path())
        .args(["build", "units", "--output", "out"])
        .output()
        .expect("failed to run spec");

    assert!(
        !output.status.success(),
        "spec build should exit 1 when no Cargo.toml ancestor exists"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("could not find crate root"),
        "expected 'could not find crate root' in stderr, got: {stderr}"
    );
}

#[test]
fn spec_build_bare_crate_no_workspace_uses_package_toml() {
    if !cargo_available() {
        return;
    }

    // Use system temp dir (outside the spec repo) so the ancestor walk sees only our
    // synthetic [package] Cargo.toml and not the spec workspace root.
    let temp_dir = tempfile::TempDir::new().unwrap();
    let crate_dir = temp_dir.path().join("mybare");
    let units_dir = crate_dir.join("units");
    let src_dir = crate_dir.join("src");
    let generated_dir = crate_dir.join("src/generated");
    fs::create_dir_all(&src_dir).unwrap();

    // Bare Cargo.toml: [package] only, no [workspace]
    fs::write(
        crate_dir.join("Cargo.toml"),
        "[package]\nname = \"spec-test-bare\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    )
    .unwrap();

    // src/main.rs that re-exports the generated spec module
    fs::write(
        src_dir.join("main.rs"),
        "mod generated;\npub use generated::*;\nfn main() {}\n",
    )
    .unwrap();

    write_minimal_units_dir(&units_dir);

    // Run spec build with current_dir=crate_dir and WITHOUT --crate-root.
    // workspace_root_for walks ancestors from units_dir, finds no [workspace],
    // falls back to [package] at crate_dir. The output path is inside crate_dir,
    // satisfying safe_output_path's project-root check.
    let output = Command::new(bin())
        .current_dir(&crate_dir)
        .args([
            "build",
            units_dir.to_str().unwrap(),
            "--output",
            generated_dir.to_str().unwrap(),
        ])
        .output()
        .expect("failed to run spec");
    assert_output_success(
        "spec build bare crate (no [workspace]) should succeed via [package] fallback",
        &output,
    );
}

#[test]
fn spec_build_prints_crate_root_to_stderr() {
    if !cargo_available() {
        return;
    }

    let root = repo_root();
    let ecommerce_dir = root.join("examples/ecommerce");
    let output = run_in(
        &root,
        &[
            "build",
            "examples/ecommerce/units",
            "--output",
            "examples/ecommerce/src/generated",
            "--crate-root",
            ecommerce_dir.to_str().unwrap(),
        ],
    );
    assert_output_success("spec build failed for progress signal test", &output);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("spec: running cargo build in"),
        "expected progress signal in stderr, got: {stderr}"
    );
}

// ── End D1 ────────────────────────────────────────────────────────────────────

// Regression: ISSUE-QA-004 — no CLI integration test for contract input identifier validation.
// spec validate must reject parameter names that are not valid Rust identifiers with a clear error.
// Found by /qa on 2026-04-04.
#[test]
fn validate_rejects_invalid_contract_input_identifier() {
    let temp_dir = temp_repo_dir();
    let units_dir = temp_dir.path().join("units");
    write_spec(
        &units_dir,
        "pricing/bad_param.unit.spec",
        r#"
spec_version: "0.3.0"
id: pricing/bad_param
kind: function
intent:
  why: Test that hyphenated parameter names are rejected.
contract:
  inputs:
    my-param: Decimal
  returns: Decimal
body:
  rust: |
    { Decimal::ZERO }
"#,
    );

    let output = run(&["validate", units_dir.to_str().unwrap()]);
    assert!(
        !output.status.success(),
        "validate should fail for invalid contract input identifier"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("'my-param'") && stderr.contains("not a valid Rust identifier"),
        "expected identifier error in stderr, got: {stderr}"
    );
}

fn copy_ecommerce_example() -> (tempfile::TempDir, PathBuf) {
    let root = repo_root();
    let temp_dir =
        tempfile::TempDir::new_in(root.join("target")).expect("failed to create temp dir");
    let dst_ecommerce = temp_dir.path().join("ecommerce");
    copy_dir_recursive(&root.join("examples/ecommerce"), &dst_ecommerce)
        .expect("failed to copy ecommerce example");
    for entry in WalkDir::new(&dst_ecommerce) {
        let entry = entry.expect("failed to walk copied ecommerce example");
        if entry.file_type().is_file()
            && entry
                .file_name()
                .to_string_lossy()
                .ends_with(".spec.passport.json")
        {
            fs::remove_file(entry.path()).expect("failed to remove copied passport artifact");
        }
    }
    (temp_dir, dst_ecommerce)
}

fn read_passport(passport_path: &Path) -> String {
    fs::read_to_string(passport_path).unwrap()
}

fn read_passport_json(passport_path: &Path) -> Value {
    serde_json::from_str(&read_passport(passport_path)).unwrap()
}

fn write_pricing_project(project_dir: &Path, target_has_tests: bool) -> PathBuf {
    let units_dir = project_dir.join("units");
    let pricing_dir = units_dir.join("pricing");
    let src_dir = project_dir.join("src");

    fs::create_dir_all(&pricing_dir).unwrap();
    fs::create_dir_all(&src_dir).unwrap();

    fs::write(
        project_dir.join("Cargo.toml"),
        "[package]\nname = \"pricing-project\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[workspace]\n",
    )
    .unwrap();
    fs::write(
        src_dir.join("main.rs"),
        "mod generated;\npub use generated::*;\nfn main() {}\n",
    )
    .unwrap();

    let target_spec = if target_has_tests {
        r#"spec_version: "0.3.0"
id: pricing/apply_tax
kind: function
intent:
  why: Apply tax to a subtotal.
contract:
  inputs:
    subtotal: f64
    rate: f64
  returns: f64
body:
  rust: |
    {
        subtotal + rate
    }
local_tests:
  - id: happy_path
    expect: "true"
"#
    } else {
        r#"spec_version: "0.3.0"
id: pricing/apply_tax
kind: function
intent:
  why: Apply tax to a subtotal.
contract:
  inputs:
    subtotal: f64
    rate: f64
  returns: f64
body:
  rust: |
    {
        subtotal + rate
    }
"#
    };

    fs::write(pricing_dir.join("apply_tax.unit.spec"), target_spec).unwrap();
    fs::write(
        pricing_dir.join("apply_discount.unit.spec"),
        r#"spec_version: "0.3.0"
id: pricing/apply_discount
kind: function
intent:
  why: Apply a discount to a subtotal.
contract:
  inputs:
    subtotal: f64
    rate: f64
  returns: f64
body:
  rust: |
    {
        subtotal - rate
    }
local_tests:
  - id: happy_path
    expect: "true"
"#,
    )
    .unwrap();

    pricing_dir.join("apply_tax.unit.spec")
}

// ── D2: Runtime evidence in passports ───────────────────────────────────────

#[test]
fn spec_test_writes_evidence_to_passport() {
    if !cargo_available() {
        return;
    }

    let (_temp_dir, ecommerce_dir) = copy_ecommerce_example();
    let output = Command::new(bin())
        .current_dir(&ecommerce_dir)
        .args([
            "test",
            "units",
            "--output",
            "src/generated",
            "--crate-root",
            ecommerce_dir.to_str().unwrap(),
        ])
        .output()
        .expect("failed to run spec");
    assert_output_success("spec test should succeed for ecommerce example", &output);

    let passport = read_passport(&ecommerce_dir.join("units/pricing/apply_tax.spec.passport.json"));
    assert!(
        passport.contains("\"build_status\": \"pass\""),
        "{passport}"
    );
    assert!(passport.contains("\"id\": \"basic_tax\""), "{passport}");
    assert!(passport.contains("\"status\": \"pass\""), "{passport}");
    assert!(passport.contains("\"observed_at\": \""), "{passport}");
}

#[test]
fn spec_test_writes_provenance_to_passport_when_git_available() {
    if !cargo_available() || !git_available() {
        return;
    }

    let (_temp_dir, ecommerce_dir) = copy_ecommerce_example();
    let sha = init_git_repo(&ecommerce_dir);

    let output = Command::new(bin())
        .current_dir(&ecommerce_dir)
        .args([
            "test",
            "units",
            "--output",
            "src/generated",
            "--crate-root",
            ecommerce_dir.to_str().unwrap(),
        ])
        .output()
        .expect("failed to run spec");
    assert_output_success("spec test should succeed for ecommerce example", &output);

    let passport =
        read_passport_json(&ecommerce_dir.join("units/pricing/apply_tax.spec.passport.json"));
    assert_eq!(passport["evidence"]["provenance"]["git_commit_sha"], sha);
}

#[test]
fn spec_test_omits_provenance_outside_git() {
    if !cargo_available() {
        return;
    }

    let temp_dir = tempfile::TempDir::new().unwrap();
    let project_dir = temp_dir.path();
    write_pricing_project(project_dir, true);

    let output = Command::new(bin())
        .current_dir(project_dir)
        .args([
            "test",
            "units/pricing",
            "--output",
            "src/generated",
            "--crate-root",
            project_dir.to_str().unwrap(),
        ])
        .output()
        .expect("failed to run spec");
    assert_output_success("spec test should succeed outside git", &output);

    let passport =
        read_passport_json(&project_dir.join("units/pricing/apply_tax.spec.passport.json"));
    assert!(passport["evidence"].get("provenance").is_none());
}

#[test]
fn spec_test_writes_contract_hash_to_passport() {
    if !cargo_available() {
        return;
    }

    let (_temp_dir, ecommerce_dir) = copy_ecommerce_example();
    let output = Command::new(bin())
        .current_dir(&ecommerce_dir)
        .args([
            "test",
            "units",
            "--output",
            "src/generated",
            "--crate-root",
            ecommerce_dir.to_str().unwrap(),
        ])
        .output()
        .expect("failed to run spec");
    assert_output_success("spec test should succeed for ecommerce example", &output);

    let passport = read_passport(&ecommerce_dir.join("units/pricing/apply_tax.spec.passport.json"));
    assert!(
        passport.contains("\"contract_hash\": \"sha256:"),
        "{passport}"
    );
}

#[test]
fn spec_test_failure_writes_fail_status_to_passport() {
    if !cargo_available() {
        return;
    }

    let (_temp_dir, ecommerce_dir) = copy_ecommerce_example();
    fs::write(
        ecommerce_dir.join("units/pricing/apply_tax.unit.spec"),
        r#"id: pricing/apply_tax
kind: function
spec_version: "0.3.0"
intent:
  why: Add sales tax to a subtotal using a rate expressed as a decimal fraction.
contract:
  inputs:
    subtotal: Decimal
    rate: Decimal
  returns: Decimal
  invariants:
    - output >= subtotal
deps:
  - money/round
imports:
  - rust_decimal::Decimal
body:
  rust: |
    {
        let taxed = subtotal + subtotal * rate;
        round(taxed)
    }
local_tests:
  - id: basic_tax
    expect: "false"
links:
  molecule_tests:
    - pricing/discount_plus_tax
"#,
    )
    .unwrap();

    let output = Command::new(bin())
        .current_dir(&ecommerce_dir)
        .args([
            "test",
            "units",
            "--output",
            "src/generated",
            "--crate-root",
            ecommerce_dir.to_str().unwrap(),
        ])
        .output()
        .expect("failed to run spec");

    assert!(
        !output.status.success(),
        "spec test should exit non-zero for failing local test"
    );

    let passport = read_passport(&ecommerce_dir.join("units/pricing/apply_tax.spec.passport.json"));
    assert!(
        passport.contains("\"build_status\": \"pass\""),
        "{passport}"
    );
    assert!(passport.contains("\"status\": \"fail\""), "{passport}");
}

#[test]
fn spec_generate_writes_initial_contract_hash() {
    if !cargo_available() {
        return;
    }

    let (_temp_dir, ecommerce_dir) = copy_ecommerce_example();
    let output = Command::new(bin())
        .current_dir(&ecommerce_dir)
        .args(["generate", "units", "--output", "src/generated"])
        .output()
        .expect("failed to run spec");
    assert_output_success(
        "spec generate should succeed for ecommerce example",
        &output,
    );

    // spec generate must write an initial contract_hash baseline so that stale
    // detection fires if the contract changes before `spec test` is run.
    let passport = read_passport(&ecommerce_dir.join("units/pricing/apply_tax.spec.passport.json"));
    assert!(
        passport.contains("\"contract_hash\": \"sha256:"),
        "generate must write initial contract_hash baseline: {passport}"
    );
}

#[test]
fn spec_status_stale_after_generate_and_contract_change() {
    // Regression: units with contracts that were only generated (never tested)
    // must show as stale after the contract changes.  Before the fix,
    // spec generate wrote contract_hash=null, so the stale check was skipped
    // and status always showed "valid".
    let temp_dir = temp_repo_dir();
    let project_dir = temp_dir.path();
    write_status_project(project_dir);

    // Step 1: generate to establish the initial contract_hash baseline.
    let gen_output = run_in(
        project_dir,
        &["generate", "units", "--output", "src/generated"],
    );
    assert_output_success("spec generate should succeed", &gen_output);

    // Step 2: change the contract (returns: bool → returns: i32).
    fs::write(
        project_dir.join("units/pricing/apply_discount.unit.spec"),
        r#"
id: pricing/apply_discount
kind: function
intent:
  why: Apply a discount.
spec_version: "0.3.0"
contract:
  returns: i32
body:
  rust: |
    { 1 }
local_tests:
  - id: happy_path
    expect: apply_discount() == 1
"#,
    )
    .unwrap();

    // Step 3: spec status must detect the stale contract — no spec test needed.
    let output = run_in(project_dir, &["status", "units"]);
    assert!(
        !output.status.success(),
        "expected non-zero exit for stale unit"
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("stale"),
        "expected 'stale' in status output: {stdout}"
    );
}

#[test]
fn spec_status_stale_when_contract_added_after_test() {
    let temp_dir = temp_repo_dir();
    let project_dir = temp_dir.path();
    write_status_project(project_dir);

    fs::write(
        project_dir.join("units/pricing/apply_discount.unit.spec"),
        r#"
id: pricing/apply_discount
kind: function
intent:
  why: Apply a discount.
spec_version: "0.3.0"
body:
  rust: |
    { }
"#,
    )
    .unwrap();

    write_file(
        project_dir,
        "units/pricing/apply_discount.spec.passport.json",
        r#"{
  "spec_version": "0.3.0",
  "id": "pricing/apply_discount",
  "intent": "Apply a discount.",
  "deps": [],
  "local_tests": [],
  "generated_at": "2024-01-02T03:04:05Z",
  "source_file": "pricing/apply_discount.unit.spec",
  "evidence": {
    "build_status": "pass",
    "test_results": [],
    "observed_at": "2024-01-02T03:04:05Z"
  }
}"#,
    );

    let before_change = run_in(project_dir, &["status", "units", "--format", "json"]);
    assert!(
        before_change.status.success(),
        "no-contract unit should remain non-stale before contract is added"
    );
    let before_change_json = parse_stdout_json(&before_change);
    let before_change_units = before_change_json["units"].as_array().unwrap();
    assert_eq!(before_change_units[0]["status"], "valid");

    fs::write(
        project_dir.join("units/pricing/apply_discount.unit.spec"),
        r#"
id: pricing/apply_discount
kind: function
intent:
  why: Apply a discount.
spec_version: "0.3.0"
contract:
  returns: bool
body:
  rust: |
    { true }
local_tests:
  - id: happy_path
    expect: apply_discount() == true
"#,
    )
    .unwrap();

    let output = run_in(project_dir, &["status", "units", "--format", "json"]);
    assert!(!output.status.success(), "contract addition should mark unit stale");
    let json = parse_stdout_json(&output);
    let units = json["units"].as_array().unwrap();
    assert_eq!(units[0]["status"], "stale");
    assert_eq!(units[0]["reason"], "contract changed since last test");
    assert_eq!(units[0]["evidence_at"], "2024-01-02T03:04:05Z");
}

#[test]
fn spec_generate_preserves_passport_evidence_from_prior_test() {
    if !cargo_available() {
        return;
    }

    let temp_dir = temp_repo_dir();
    write_pricing_project(temp_dir.path(), true);

    // Seed evidence and contract_hash via spec test.
    let seed = Command::new(bin())
        .current_dir(temp_dir.path())
        .args([
            "test",
            "units/pricing",
            "--output",
            "src/generated",
            "--crate-root",
            temp_dir.path().to_str().unwrap(),
        ])
        .output()
        .expect("failed to seed spec passports");
    assert_output_success("spec test should seed passports", &seed);

    let passport_path = temp_dir
        .path()
        .join("units/pricing/apply_tax.spec.passport.json");
    let after_test = read_passport(&passport_path);
    assert!(
        after_test.contains("\"build_status\": \"pass\""),
        "expected evidence after spec test: {after_test}"
    );
    assert!(
        after_test.contains("\"contract_hash\": \"sha256:"),
        "expected contract_hash after spec test: {after_test}"
    );

    // Running spec generate must not erase the evidence or contract_hash.
    let output = Command::new(bin())
        .current_dir(temp_dir.path())
        .args(["generate", "units/pricing", "--output", "src/generated"])
        .output()
        .expect("failed to run spec generate");
    assert_output_success("spec generate should succeed after spec test", &output);

    let after_generate = read_passport(&passport_path);
    assert!(
        after_generate.contains("\"build_status\": \"pass\""),
        "spec generate must not erase evidence: {after_generate}"
    );
    assert!(
        after_generate.contains("\"contract_hash\": \"sha256:"),
        "spec generate must not erase contract_hash: {after_generate}"
    );
}

#[test]
fn spec_generate_preserves_passport_provenance_from_prior_test() {
    if !cargo_available() || !git_available() {
        return;
    }

    let temp_dir = temp_repo_dir();
    write_pricing_project(temp_dir.path(), true);
    let sha = init_git_repo(temp_dir.path());

    let seed = Command::new(bin())
        .current_dir(temp_dir.path())
        .args([
            "test",
            "units/pricing",
            "--output",
            "src/generated",
            "--crate-root",
            temp_dir.path().to_str().unwrap(),
        ])
        .output()
        .expect("failed to seed spec passports");
    assert_output_success("spec test should seed passports", &seed);

    let passport_path = temp_dir
        .path()
        .join("units/pricing/apply_tax.spec.passport.json");
    let after_test = read_passport_json(&passport_path);
    assert_eq!(after_test["evidence"]["provenance"]["git_commit_sha"], sha);

    let output = Command::new(bin())
        .current_dir(temp_dir.path())
        .args(["generate", "units/pricing", "--output", "src/generated"])
        .output()
        .expect("failed to run spec generate");
    assert_output_success("spec generate should succeed after spec test", &output);

    let after_generate = read_passport_json(&passport_path);
    assert_eq!(
        after_generate["evidence"]["provenance"]["git_commit_sha"],
        sha
    );
}

#[test]
fn spec_build_preserves_passport_evidence_from_prior_test() {
    if !cargo_available() {
        return;
    }

    let temp_dir = temp_repo_dir();
    write_pricing_project(temp_dir.path(), true);

    // Seed evidence and contract_hash via spec test.
    let seed = Command::new(bin())
        .current_dir(temp_dir.path())
        .args([
            "test",
            "units/pricing",
            "--output",
            "src/generated",
            "--crate-root",
            temp_dir.path().to_str().unwrap(),
        ])
        .output()
        .expect("failed to seed spec passports");
    assert_output_success("spec test should seed passports", &seed);

    let passport_path = temp_dir
        .path()
        .join("units/pricing/apply_tax.spec.passport.json");
    let after_test = read_passport(&passport_path);
    assert!(
        after_test.contains("\"build_status\": \"pass\""),
        "expected evidence after spec test: {after_test}"
    );
    assert!(
        after_test.contains("\"contract_hash\": \"sha256:"),
        "expected contract_hash after spec test: {after_test}"
    );

    // Running spec build must not erase the evidence or contract_hash.
    let output = Command::new(bin())
        .current_dir(temp_dir.path())
        .args([
            "build",
            "units/pricing",
            "--output",
            "src/generated",
            "--crate-root",
            temp_dir.path().to_str().unwrap(),
        ])
        .output()
        .expect("failed to run spec build");
    assert_output_success("spec build should succeed after spec test", &output);

    let after_build = read_passport(&passport_path);
    assert!(
        after_build.contains("\"build_status\": \"pass\""),
        "spec build must not erase evidence: {after_build}"
    );
    assert!(
        after_build.contains("\"contract_hash\": \"sha256:"),
        "spec build must not erase contract_hash: {after_build}"
    );
}

#[test]
fn spec_build_preserves_passport_provenance_from_prior_test() {
    if !cargo_available() || !git_available() {
        return;
    }

    let temp_dir = temp_repo_dir();
    write_pricing_project(temp_dir.path(), true);
    let sha = init_git_repo(temp_dir.path());

    let seed = Command::new(bin())
        .current_dir(temp_dir.path())
        .args([
            "test",
            "units/pricing",
            "--output",
            "src/generated",
            "--crate-root",
            temp_dir.path().to_str().unwrap(),
        ])
        .output()
        .expect("failed to seed spec passports");
    assert_output_success("spec test should seed passports", &seed);

    let passport_path = temp_dir
        .path()
        .join("units/pricing/apply_tax.spec.passport.json");
    let after_test = read_passport_json(&passport_path);
    assert_eq!(after_test["evidence"]["provenance"]["git_commit_sha"], sha);

    let output = Command::new(bin())
        .current_dir(temp_dir.path())
        .args([
            "build",
            "units/pricing",
            "--output",
            "src/generated",
            "--crate-root",
            temp_dir.path().to_str().unwrap(),
        ])
        .output()
        .expect("failed to run spec build");
    assert_output_success("spec build should succeed after spec test", &output);

    let after_build = read_passport_json(&passport_path);
    assert_eq!(after_build["evidence"]["provenance"]["git_commit_sha"], sha);
}

#[test]
fn spec_test_build_failure_writes_fail_build_status_to_passport() {
    if !cargo_available() {
        return;
    }

    let (_temp_dir, ecommerce_dir) = copy_ecommerce_example();
    write_spec(
        &ecommerce_dir.join("units"),
        "pricing/broken.unit.spec",
        r#"spec_version: "0.3.0"
id: pricing/broken
kind: function
intent:
  why: Force a compile error.
contract:
  returns: NotARealType
body:
  rust: |
    {
        todo!()
    }
"#,
    );

    let output = Command::new(bin())
        .current_dir(&ecommerce_dir)
        .args([
            "test",
            "units",
            "--output",
            "src/generated",
            "--crate-root",
            ecommerce_dir.to_str().unwrap(),
        ])
        .output()
        .expect("failed to run spec");

    assert!(
        !output.status.success(),
        "spec test should exit non-zero for compile failure"
    );

    let passport =
        read_passport(&ecommerce_dir.join("units/pricing/apply_discount.spec.passport.json"));
    assert!(
        passport.contains("\"build_status\": \"fail\""),
        "{passport}"
    );
    assert!(passport.contains("\"test_results\": []"), "{passport}");
}

#[test]
fn spec_test_build_failure_writes_provenance_when_git_available() {
    if !cargo_available() || !git_available() {
        return;
    }

    let (_temp_dir, ecommerce_dir) = copy_ecommerce_example();
    write_spec(
        &ecommerce_dir.join("units"),
        "pricing/broken.unit.spec",
        r#"spec_version: "0.3.0"
id: pricing/broken
kind: function
intent:
  why: Force a compile error.
contract:
  returns: NotARealType
body:
  rust: |
    {
        todo!()
    }
"#,
    );
    let sha = init_git_repo(&ecommerce_dir);

    let output = Command::new(bin())
        .current_dir(&ecommerce_dir)
        .args([
            "test",
            "units",
            "--output",
            "src/generated",
            "--crate-root",
            ecommerce_dir.to_str().unwrap(),
        ])
        .output()
        .expect("failed to run spec");
    assert!(
        !output.status.success(),
        "spec test should exit non-zero for compile failure"
    );

    let passport =
        read_passport_json(&ecommerce_dir.join("units/pricing/apply_discount.spec.passport.json"));
    assert_eq!(passport["evidence"]["provenance"]["git_commit_sha"], sha);
}

#[test]
fn spec_test_evidence_matches_non_default_output_module_name() {
    if !cargo_available() {
        return;
    }

    let (_temp_dir, ecommerce_dir) = copy_ecommerce_example();
    let main_rs_path = ecommerce_dir.join("src/main.rs");
    let main_rs = fs::read_to_string(&main_rs_path).unwrap();
    let rewritten = main_rs.replace(
        "mod generated;\npub use generated::*;",
        "mod atomized;\npub use atomized::*;",
    );
    fs::write(&main_rs_path, rewritten).unwrap();

    let output = Command::new(bin())
        .current_dir(&ecommerce_dir)
        .args([
            "test",
            "units",
            "--output",
            "src/atomized",
            "--crate-root",
            ecommerce_dir.to_str().unwrap(),
        ])
        .output()
        .expect("failed to run spec");
    assert_output_success(
        "spec test should succeed with non-default output module",
        &output,
    );

    let passport = read_passport(&ecommerce_dir.join("units/pricing/apply_tax.spec.passport.json"));
    assert!(passport.contains("\"status\": \"pass\""), "{passport}");
}

#[test]
fn spec_test_no_local_tests_produces_empty_evidence() {
    if !cargo_available() {
        return;
    }

    let temp_dir = tempfile::TempDir::new().unwrap();
    let crate_dir = temp_dir.path().join("nolocal");
    let units_dir = crate_dir.join("units");
    let src_dir = crate_dir.join("src");
    fs::create_dir_all(&src_dir).unwrap();
    fs::write(
        crate_dir.join("Cargo.toml"),
        "[package]\nname = \"nolocal\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    )
    .unwrap();
    fs::write(
        src_dir.join("main.rs"),
        "mod generated;\npub use generated::*;\nfn main() {}\n",
    )
    .unwrap();
    write_spec(
        &units_dir,
        "money/round.unit.spec",
        r#"spec_version: "0.3.0"
id: money/round
kind: function
intent:
  why: Echo the provided value.
contract:
  inputs:
    value: f64
  returns: f64
body:
  rust: |
    {
        value
    }
"#,
    );

    let output = Command::new(bin())
        .current_dir(&crate_dir)
        .args([
            "test",
            units_dir.to_str().unwrap(),
            "--output",
            "src/generated",
            "--crate-root",
            crate_dir.to_str().unwrap(),
        ])
        .output()
        .expect("failed to run spec");
    assert_output_success("spec test should succeed with no local tests", &output);

    let passport = read_passport(&units_dir.join("money/round.spec.passport.json"));
    assert!(
        passport.contains("\"build_status\": \"pass\""),
        "{passport}"
    );
    assert!(passport.contains("\"test_results\": []"), "{passport}");
}

#[test]
fn spec_test_writes_evidence_atomically() {
    if !cargo_available() {
        return;
    }

    let (_temp_dir, ecommerce_dir) = copy_ecommerce_example();
    let output = Command::new(bin())
        .current_dir(&ecommerce_dir)
        .args([
            "test",
            "units",
            "--output",
            "src/generated",
            "--crate-root",
            ecommerce_dir.to_str().unwrap(),
        ])
        .output()
        .expect("failed to run spec");
    assert_output_success(
        "spec test should succeed for atomic passport rewrite check",
        &output,
    );

    for entry in WalkDir::new(ecommerce_dir.join("units")) {
        let entry = entry.unwrap();
        if !entry.file_type().is_file() {
            continue;
        }
        if !entry
            .file_name()
            .to_string_lossy()
            .ends_with(".spec.passport.json")
        {
            continue;
        }

        let passport = read_passport(entry.path());
        assert!(
            passport.contains("\"id\": \""),
            "invalid passport at {}",
            entry.path().display()
        );
        assert!(
            passport.contains("\"evidence\": {"),
            "missing evidence at {}",
            entry.path().display()
        );
    }
}

// ── D4: Status command ─────────────────────────────────────────────────────

#[test]
fn spec_status_all_valid_no_evidence() {
    let (_temp_dir, ecommerce_dir) = copy_ecommerce_example();
    let output = run_in(&ecommerce_dir, &["status", "units"]);
    assert!(!output.status.success(), "untested units should exit 1");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("pricing/apply_tax"), "{stdout}");
    assert!(stdout.contains("—"), "{stdout}");
    assert!(stdout.contains("untested"), "{stdout}");
}

#[test]
fn spec_status_after_spec_test() {
    if !cargo_available() {
        return;
    }

    let (_temp_dir, ecommerce_dir) = copy_ecommerce_example();
    let test_output = Command::new(bin())
        .current_dir(&ecommerce_dir)
        .args([
            "test",
            "units",
            "--output",
            "src/generated",
            "--crate-root",
            ecommerce_dir.to_str().unwrap(),
        ])
        .output()
        .expect("failed to run spec test");
    assert_output_success("spec test should succeed before status check", &test_output);

    let output = run_in(&ecommerce_dir, &["status", "units"]);
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("evidence:"), "{stdout}");
    assert!(!stdout.contains("no-evidence"), "{stdout}");
}

#[test]
fn spec_status_invalid_unit() {
    let temp_dir = temp_repo_dir();
    let units_dir = temp_dir.path().join("units");
    write_spec(
        &units_dir,
        "pricing/bad.unit.spec",
        r#"
id: pricing/bad
kind: function
intent:
  why: Trigger a validation error.
body:
  rust: |
    {
        use std::fmt;
    }
"#,
    );

    let output = run(&["status", units_dir.to_str().unwrap()]);
    assert!(!output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("✗ pricing/bad"), "{stdout}");
    assert!(stdout.contains("invalid"), "{stdout}");
}

#[test]
fn spec_status_json_invalid_unit() {
    let temp_dir = temp_repo_dir();
    let units_dir = temp_dir.path().join("units");
    write_spec(
        &units_dir,
        "pricing/bad.unit.spec",
        r#"
id: pricing/bad
kind: function
intent:
  why: Trigger a validation error.
body:
  rust: |
    {
        use std::fmt;
    }
"#,
    );

    let output = run(&["status", units_dir.to_str().unwrap(), "--format", "json"]);
    assert!(!output.status.success());
    assert!(
        output.stderr.is_empty(),
        "expected no stderr, got: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let json = parse_stdout_json(&output);
    assert_eq!(json["schema_version"], 2);
    let units = json["units"].as_array().unwrap();
    assert_eq!(units.len(), 1);
    assert_eq!(units[0]["id"], "pricing/bad");
    assert_eq!(units[0]["status"], "invalid");
    assert!(
        !units[0]["errors"].as_array().unwrap().is_empty(),
        "errors array should be non-empty for invalid unit"
    );
    assert_eq!(units[0]["errors"][0]["code"], "SPEC_USE_STATEMENT_IN_BODY");
}

#[test]
fn spec_status_json_loader_error_surfaces_in_response() {
    let temp_dir = temp_repo_dir();
    let units_dir = temp_dir.path().join("units");
    std::fs::create_dir_all(&units_dir).unwrap();
    // Write a file that is not valid YAML — triggers a loader error, not a validation error.
    std::fs::write(
        units_dir.join("bad.unit.spec"),
        "not: valid: yaml: [unclosed",
    )
    .unwrap();

    let output = run(&["status", units_dir.to_str().unwrap(), "--format", "json"]);
    assert!(
        !output.status.success(),
        "should exit non-zero for loader error"
    );
    assert!(
        output.stderr.is_empty(),
        "no text diagnostics on stderr in JSON mode, got: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let json = parse_stdout_json(&output);
    assert_eq!(json["schema_version"], 2);
    let loader_errors = json["loader_errors"].as_array().unwrap();
    assert!(
        !loader_errors.is_empty(),
        "loader_errors must be present in JSON response when loader fails"
    );
    assert_eq!(loader_errors[0]["code"], "SPEC_YAML_PARSE");
}

#[test]
fn spec_status_stale_unit() {
    if !cargo_available() {
        return;
    }

    let temp_dir = temp_repo_dir();
    let project_dir = temp_dir.path();
    write_status_project(project_dir);

    let test_output = Command::new(bin())
        .current_dir(project_dir)
        .args([
            "test",
            "units",
            "--output",
            "src/generated",
            "--crate-root",
            project_dir.to_str().unwrap(),
        ])
        .output()
        .expect("failed to run spec test");
    assert_output_success("spec test should succeed before stale check", &test_output);

    rewrite_passport_generated_at(
        &project_dir.join("units/pricing/apply_discount.spec.passport.json"),
        "2024-01-02T03:04:05Z",
    );

    fs::write(
        project_dir.join("units/pricing/apply_discount.unit.spec"),
        r#"
id: pricing/apply_discount
kind: function
intent:
  why: Apply a discount.
spec_version: "0.3.0"
contract:
  returns: i32
body:
  rust: |
    { 1 }
local_tests:
  - id: happy_path
    expect: apply_discount() == true
"#,
    )
    .unwrap();

    let output = run_in(project_dir, &["status", "units"]);
    assert!(!output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("~ pricing/apply_discount"), "{stdout}");
    assert!(stdout.contains("stale"), "{stdout}");

    let json_output = run_in(project_dir, &["status", "units", "--format", "json"]);
    assert!(!json_output.status.success());
    assert_stdout_json_matches_fixture(&json_output, "status-stale.json");
}

#[test]
fn spec_status_stale_when_contract_removed_after_test() {
    if !cargo_available() {
        return;
    }

    let temp_dir = temp_repo_dir();
    let project_dir = temp_dir.path();
    write_status_project(project_dir);

    let test_output = Command::new(bin())
        .current_dir(project_dir)
        .args([
            "test",
            "units",
            "--output",
            "src/generated",
            "--crate-root",
            project_dir.to_str().unwrap(),
        ])
        .output()
        .expect("failed to run spec test");
    assert_output_success("spec test should succeed before removing contract", &test_output);

    rewrite_passport_generated_at(
        &project_dir.join("units/pricing/apply_discount.spec.passport.json"),
        "2024-01-02T03:04:05Z",
    );

    fs::write(
        project_dir.join("units/pricing/apply_discount.unit.spec"),
        r#"
id: pricing/apply_discount
kind: function
intent:
  why: Apply a discount.
spec_version: "0.3.0"
body:
  rust: |
    { true }
local_tests:
  - id: happy_path
    expect: apply_discount() == true
"#,
    )
    .unwrap();

    let output = run_in(project_dir, &["status", "units", "--format", "json"]);
    assert!(!output.status.success(), "contract removal should mark unit stale");
    let json = parse_stdout_json(&output);
    let units = json["units"].as_array().unwrap();
    assert_eq!(units[0]["status"], "stale");
    assert_eq!(units[0]["reason"], "contract changed since last test");
    assert_eq!(units[0]["evidence_at"], "2024-01-02T03:04:05Z");
}

#[test]
fn spec_status_valid_unit() {
    let temp_dir = temp_repo_dir();
    let project_dir = temp_dir.path();
    write_status_project(project_dir);

    let output = Command::new(bin())
        .current_dir(project_dir)
        .args([
            "test",
            "units",
            "--output",
            "src/generated",
            "--crate-root",
            project_dir.to_str().unwrap(),
        ])
        .output()
        .expect("failed to run spec test");
    assert_output_success("spec test should succeed before status check", &output);

    rewrite_passport_generated_at(
        &project_dir.join("units/pricing/apply_discount.spec.passport.json"),
        "2024-01-02T03:04:05Z",
    );

    let output = run_in(project_dir, &["status", "units", "--format", "json"]);
    assert!(output.status.success());

    assert_stdout_json_matches_fixture(&output, "status-valid.json");
}

#[test]
fn spec_status_malformed_passport_warns_not_aborts() {
    let temp_dir = temp_repo_dir();
    let units_dir = temp_dir.path().join("units");
    write_spec(
        &units_dir,
        "pricing/apply_discount.unit.spec",
        r#"
id: pricing/apply_discount
kind: function
intent:
  why: Apply a discount.
body:
  rust: |
    { }
"#,
    );
    write_file(
        &units_dir,
        "pricing/apply_discount.spec.passport.json",
        r#"{"spec_version":"0.3.0""#,
    );

    let output = run(&["status", units_dir.to_str().unwrap()]);
    assert!(!output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stdout.contains("—"), "{stdout}");
    assert!(stderr.contains("⚠ failed to read passport"), "{stderr}");
}

#[test]
fn spec_status_untested_unit() {
    let temp_dir = temp_repo_dir();
    let units_dir = temp_dir.path().join("units");
    write_spec(
        &units_dir,
        "pricing/apply_discount.unit.spec",
        r#"
id: pricing/apply_discount
kind: function
intent:
  why: Apply a discount.
body:
  rust: |
    { }
"#,
    );
    // No passport written — unit has no evidence.

    let output = run(&["status", units_dir.to_str().unwrap(), "--format", "json"]);
    assert!(!output.status.success(), "untested unit should exit 1");

    let json = parse_stdout_json(&output);
    let units = json["units"].as_array().unwrap();
    assert_eq!(units[0]["status"], "untested");
    assert_eq!(units[0]["reason"], "no evidence");
    assert!(units[0].get("evidence_at").is_none() || units[0]["evidence_at"].is_null());
    assert_stdout_json_matches_fixture(&output, "status-untested.json");
}

#[test]
fn spec_status_failing_build() {
    let temp_dir = temp_repo_dir();
    let units_dir = temp_dir.path().join("units");
    write_spec(
        &units_dir,
        "pricing/apply_discount.unit.spec",
        r#"
id: pricing/apply_discount
kind: function
intent:
  why: Apply a discount.
body:
  rust: |
    { }
"#,
    );
    write_file(
        &units_dir,
        "pricing/apply_discount.spec.passport.json",
        r#"{
  "spec_version": "0.3.0",
  "id": "pricing/apply_discount",
  "intent": "Apply a discount.",
  "deps": [],
  "local_tests": [],
  "generated_at": "2024-01-02T03:04:05Z",
  "source_file": "pricing/apply_discount.unit.spec",
  "evidence": {
    "build_status": "fail",
    "test_results": [],
    "observed_at": "2024-01-02T03:04:05Z"
  }
}"#,
    );

    let output = run(&["status", units_dir.to_str().unwrap(), "--format", "json"]);
    assert!(!output.status.success(), "failing unit should exit 1");

    let json = parse_stdout_json(&output);
    let units = json["units"].as_array().unwrap();
    assert_eq!(units[0]["status"], "failing");
    assert_eq!(units[0]["reason"], "build failed");
    assert_eq!(units[0]["evidence_at"], "2024-01-02T03:04:05Z");
    assert_stdout_json_matches_fixture(&output, "status-failing.json");
}

#[test]
fn spec_status_failing_test() {
    let temp_dir = temp_repo_dir();
    let units_dir = temp_dir.path().join("units");
    write_spec(
        &units_dir,
        "pricing/apply_discount.unit.spec",
        r#"
id: pricing/apply_discount
kind: function
intent:
  why: Apply a discount.
body:
  rust: |
    { }
"#,
    );
    write_file(
        &units_dir,
        "pricing/apply_discount.spec.passport.json",
        r#"{
  "spec_version": "0.3.0",
  "id": "pricing/apply_discount",
  "intent": "Apply a discount.",
  "deps": [],
  "local_tests": [],
  "generated_at": "2024-01-02T03:04:05Z",
  "source_file": "pricing/apply_discount.unit.spec",
  "evidence": {
    "build_status": "pass",
    "test_results": [
      {"id": "happy_path", "status": "fail"}
    ],
    "observed_at": "2024-01-02T03:04:05Z"
  }
}"#,
    );

    let output = run(&["status", units_dir.to_str().unwrap(), "--format", "json"]);
    assert!(!output.status.success(), "failing test should exit 1");

    let json = parse_stdout_json(&output);
    let units = json["units"].as_array().unwrap();
    assert_eq!(units[0]["status"], "failing");
    assert_eq!(units[0]["reason"], "1 test failed");
    assert_eq!(units[0]["evidence_at"], "2024-01-02T03:04:05Z");
}

#[test]
fn spec_status_incomplete() {
    let temp_dir = temp_repo_dir();
    let units_dir = temp_dir.path().join("units");
    write_spec(
        &units_dir,
        "pricing/apply_discount.unit.spec",
        r#"
id: pricing/apply_discount
kind: function
intent:
  why: Apply a discount.
body:
  rust: |
    { }
"#,
    );
    write_file(
        &units_dir,
        "pricing/apply_discount.spec.passport.json",
        r#"{
  "spec_version": "0.3.0",
  "id": "pricing/apply_discount",
  "intent": "Apply a discount.",
  "deps": [],
  "local_tests": [],
  "generated_at": "2024-01-02T03:04:05Z",
  "source_file": "pricing/apply_discount.unit.spec",
  "evidence": {
    "build_status": "pass",
    "test_results": [
      {"id": "happy_path", "status": "unknown"}
    ],
    "observed_at": "2024-01-02T03:04:05Z"
  }
}"#,
    );

    let output = run(&["status", units_dir.to_str().unwrap(), "--format", "json"]);
    assert!(!output.status.success(), "incomplete unit should exit 1");

    let json = parse_stdout_json(&output);
    let units = json["units"].as_array().unwrap();
    assert_eq!(units[0]["status"], "incomplete");
    assert_eq!(units[0]["reason"], "1 test not observed in cargo output");
    assert_eq!(units[0]["evidence_at"], "2024-01-02T03:04:05Z");
    assert_stdout_json_matches_fixture(&output, "status-incomplete.json");
}

#[test]
fn spec_status_failing_beats_stale() {
    let temp_dir = temp_repo_dir();
    let units_dir = temp_dir.path().join("units");
    write_spec(
        &units_dir,
        "pricing/apply_discount.unit.spec",
        r#"
id: pricing/apply_discount
kind: function
intent:
  why: Apply a discount.
contract:
  returns: i32
body:
  rust: |
    { 42 }
"#,
    );
    // Passport has a stale hash AND a build failure — failing should win.
    write_file(
        &units_dir,
        "pricing/apply_discount.spec.passport.json",
        r#"{
  "spec_version": "0.3.0",
  "id": "pricing/apply_discount",
  "intent": "Apply a discount.",
  "deps": [],
  "local_tests": [],
  "generated_at": "2024-01-02T03:04:05Z",
  "source_file": "pricing/apply_discount.unit.spec",
  "contract_hash": "0000000000000000000000000000000000000000000000000000000000000000",
  "evidence": {
    "build_status": "fail",
    "test_results": [],
    "observed_at": "2024-01-02T03:04:05Z"
  }
}"#,
    );

    let output = run(&["status", units_dir.to_str().unwrap(), "--format", "json"]);
    assert!(!output.status.success(), "failing unit should exit 1");

    let json = parse_stdout_json(&output);
    let units = json["units"].as_array().unwrap();
    assert_eq!(units[0]["status"], "failing", "failing should beat stale");
    assert_ne!(units[0]["status"], "stale", "stale should not win over failing");
}

#[test]
fn spec_status_single_file_path() {
    let temp_dir = temp_repo_dir();
    let spec_path = temp_dir.path().join("apply_discount.unit.spec");
    fs::write(
        &spec_path,
        r#"
id: pricing/apply_discount
kind: function
intent:
  why: Apply a discount.
body:
  rust: |
    { }
"#,
    )
    .unwrap();

    let output = run(&["status", spec_path.to_str().unwrap()]);
    assert!(!output.status.success(), "untested unit should exit 1");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("pricing/apply_discount"), "{stdout}");
    assert!(stdout.contains("—"), "{stdout}");
}

// ── D5: Single-file spec test scope ─────────────────────────────────────────

#[test]
fn spec_test_accepts_file_path() {
    if !cargo_available() {
        return;
    }

    let temp_dir = temp_repo_dir();
    let spec_path = write_pricing_project(temp_dir.path(), true);

    let output = Command::new(bin())
        .current_dir(temp_dir.path())
        .args([
            "test",
            spec_path.to_str().unwrap(),
            "--output",
            "src/generated",
            "--crate-root",
            temp_dir.path().to_str().unwrap(),
        ])
        .output()
        .expect("failed to run spec");

    assert_output_success("spec test should accept a file path", &output);

    let target_passport = temp_dir
        .path()
        .join("units/pricing/apply_tax.spec.passport.json");
    let sibling_passport = temp_dir
        .path()
        .join("units/pricing/apply_discount.spec.passport.json");

    assert!(
        target_passport.exists(),
        "expected target passport to be written"
    );
    assert!(
        !sibling_passport.exists(),
        "expected sibling passport to remain unwritten in file-path mode"
    );
}

#[test]
fn spec_test_file_path_only_writes_target_passport() {
    if !cargo_available() {
        return;
    }

    let temp_dir = temp_repo_dir();
    let spec_path = write_pricing_project(temp_dir.path(), true);

    let seed = Command::new(bin())
        .current_dir(temp_dir.path())
        .args([
            "test",
            "units/pricing",
            "--output",
            "src/generated",
            "--crate-root",
            temp_dir.path().to_str().unwrap(),
        ])
        .output()
        .expect("failed to seed spec passports");
    assert_output_success("spec test should seed pricing passports", &seed);

    let target_passport_path = temp_dir
        .path()
        .join("units/pricing/apply_tax.spec.passport.json");
    let sibling_passport_path = temp_dir
        .path()
        .join("units/pricing/apply_discount.spec.passport.json");
    let target_before = read_passport(&target_passport_path);
    let sibling_before = read_passport(&sibling_passport_path);

    let output = Command::new(bin())
        .current_dir(temp_dir.path())
        .args([
            "test",
            spec_path.to_str().unwrap(),
            "--output",
            "src/generated",
            "--crate-root",
            temp_dir.path().to_str().unwrap(),
        ])
        .output()
        .expect("failed to run spec");

    assert_output_success("spec test should succeed for file path mode", &output);

    let target_after = read_passport(&target_passport_path);
    let sibling_after = read_passport(&sibling_passport_path);
    assert_ne!(
        target_after, target_before,
        "expected target passport to be rewritten in file-path mode"
    );
    assert_eq!(
        sibling_after, sibling_before,
        "expected sibling passport to remain unchanged in file-path mode"
    );
}

#[test]
fn spec_test_file_path_nested_output_filter() {
    if !cargo_available() {
        return;
    }

    let temp_dir = temp_repo_dir();
    let units_dir = temp_dir.path().join("units/pricing");
    let src_dir = temp_dir.path().join("src");
    let generated_dir = src_dir.join("generated");

    fs::create_dir_all(&units_dir).unwrap();
    fs::create_dir_all(&generated_dir).unwrap();

    fs::write(
        temp_dir.path().join("Cargo.toml"),
        "[package]\nname = \"nested-output-project\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[workspace]\n",
    )
    .unwrap();
    fs::write(
        src_dir.join("main.rs"),
        "mod generated;\npub use generated::*;\nfn main() {}\n",
    )
    .unwrap();
    // Rust needs generated/mod.rs to resolve the nested `spec` module
    fs::write(generated_dir.join("mod.rs"), "pub mod spec;\n").unwrap();

    let spec_content = r#"spec_version: "0.3.0"
id: pricing/apply_tax
kind: function
intent:
  why: Apply tax to a subtotal.
contract:
  inputs:
    subtotal: f64
    rate: f64
  returns: f64
body:
  rust: |
    {
        subtotal + rate
    }
local_tests:
  - id: happy_path
    expect: "true"
"#;
    let spec_path = units_dir.join("apply_tax.unit.spec");
    fs::write(&spec_path, spec_content).unwrap();

    let crate_root = temp_dir.path().to_str().unwrap();

    // Seed: run on the full directory so all passports are written
    let seed = Command::new(bin())
        .current_dir(temp_dir.path())
        .args([
            "test",
            "units/pricing",
            "--output",
            "src/generated/spec",
            "--crate-root",
            crate_root,
        ])
        .output()
        .expect("failed to seed passports");
    assert_output_success("spec test should seed with nested output", &seed);

    let passport_path = temp_dir
        .path()
        .join("units/pricing/apply_tax.spec.passport.json");
    let before = read_passport(&passport_path);

    // Single-file run with nested output — cargo_test_filter_for must produce
    // "generated::spec::..." not just "spec::..." for the filter to match
    let output = Command::new(bin())
        .current_dir(temp_dir.path())
        .args([
            "test",
            spec_path.to_str().unwrap(),
            "--output",
            "src/generated/spec",
            "--crate-root",
            crate_root,
        ])
        .output()
        .expect("failed to run spec test single-file");
    assert_output_success(
        "spec test should pass for nested output single-file mode",
        &output,
    );

    let after = read_passport(&passport_path);
    assert_ne!(after, before, "expected passport to be rewritten");
    assert!(after.contains("\"status\": \"pass\""), "{after}");
}

#[test]
fn spec_test_zero_tests_matched_exits_nonzero() {
    if !cargo_available() {
        return;
    }

    let temp_dir = temp_repo_dir();
    let spec_path = write_pricing_project(temp_dir.path(), false);

    let output = Command::new(bin())
        .current_dir(temp_dir.path())
        .args([
            "test",
            spec_path.to_str().unwrap(),
            "--output",
            "src/generated",
            "--crate-root",
            temp_dir.path().to_str().unwrap(),
        ])
        .output()
        .expect("failed to run spec");

    assert!(
        !output.status.success(),
        "spec test should exit non-zero when the filter matches no tests"
    );

    let target_passport = temp_dir
        .path()
        .join("units/pricing/apply_tax.spec.passport.json");
    let sibling_passport = temp_dir
        .path()
        .join("units/pricing/apply_discount.spec.passport.json");
    assert!(
        !target_passport.exists(),
        "expected target passport not to be written when zero tests ran"
    );
    assert!(
        !sibling_passport.exists(),
        "expected sibling passport not to be written when zero tests ran"
    );
}

#[test]
fn spec_test_directory_path_unchanged() {
    if !cargo_available() {
        return;
    }

    let temp_dir = temp_repo_dir();
    write_pricing_project(temp_dir.path(), true);

    let output = Command::new(bin())
        .current_dir(temp_dir.path())
        .args([
            "test",
            "units/pricing",
            "--output",
            "src/generated",
            "--crate-root",
            temp_dir.path().to_str().unwrap(),
        ])
        .output()
        .expect("failed to run spec");

    assert_output_success(
        "spec test should still succeed for directory paths",
        &output,
    );

    let target_passport = read_passport(
        &temp_dir
            .path()
            .join("units/pricing/apply_tax.spec.passport.json"),
    );
    assert!(
        target_passport.contains("\"status\": \"pass\""),
        "{target_passport}"
    );
}

#[cfg(unix)]
#[test]
fn spec_test_respects_pipeline_timeout_secs() {
    let temp_dir = temp_repo_dir();
    let project_dir = temp_dir.path();
    let units_dir = project_dir.join("units");
    write_minimal_units_dir(&units_dir);
    write_file(
        project_dir,
        "Cargo.toml",
        "[package]\nname = \"timeout-fixture\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(project_dir, "spec.toml", "[pipeline]\ntimeout_secs = 1\n");

    let fake_bin_dir = project_dir.join("fake-bin");
    write_executable_file(
        &fake_bin_dir,
        "cargo",
        "#!/bin/sh\nif [ \"$1\" = \"--version\" ]; then\n  echo 'cargo 1.89.0'\n  exit 0\nfi\n/bin/sleep 2\n",
    );
    let mut path_override = std::ffi::OsString::from(fake_bin_dir.as_os_str());
    path_override.push(":");
    path_override.push(std::env::var_os("PATH").unwrap_or_default());

    let output = run_in_with_env(
        project_dir,
        &[
            "test",
            "units",
            "--output",
            "generated/spec",
            "--crate-root",
            project_dir.to_str().unwrap(),
        ],
        &[("PATH", path_override.as_os_str())],
    );

    assert!(!output.status.success(), "test should fail on timeout");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("timed out after 1s"), "{stderr}");
    assert!(
        stderr.contains("cargo") && stderr.contains("timed out"),
        "{stderr}"
    );

    // build_timeout_evidence must write a passport with build_status="timeout"
    let passport_path = project_dir.join("units/pricing/apply_discount.spec.passport.json");
    assert!(
        passport_path.exists(),
        "passport should be written on timeout: {}",
        passport_path.display()
    );
    let passport = fs::read_to_string(&passport_path).unwrap();
    assert!(
        passport.contains("\"build_status\": \"timeout\""),
        "passport should record timeout evidence: {passport}"
    );
}
