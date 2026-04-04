use std::fs;
use std::io;
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

#[test]
fn help_lists_validate_and_generate_commands() {
    let output = run(&["--help"]);
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("validate"));
    assert!(stdout.contains("generate"));
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
    pub fn apply_discount() {}
"#,
    );

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
        stderr.contains(
            "❌ --no-strict is not valid for spec generate — use spec validate to check without strict enforcement"
        ),
        "{stderr}"
    );
    assert!(!output_dir.exists());
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
    // The warning includes the current binary version as a suggestion.
    assert!(
        stderr.contains(env!("CARGO_PKG_VERSION")),
        "expected current version ({}) in warning, got: {stderr}",
        env!("CARGO_PKG_VERSION")
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
