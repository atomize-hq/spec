use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

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
    pub fn apply_discount() {}
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
    pub fn apply_discount() {}
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
    assert!(stdout.contains("Generated 1 file"));
    assert!(output_dir.join(".spec-generated").exists());
    assert!(output_dir.join("pricing/apply_discount.rs").exists());
    assert!(output_dir.join("pricing/mod.rs").exists());
    assert!(output_dir.join("mod.rs").exists());
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
    assert!(!output_dir.exists(), "expected output dir to not be created");
    assert!(!output_dir.join(".spec-generated").exists());
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
body:
  rust: |
    pub fn round(value: Decimal) -> Decimal {
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
deps:
  - money/round
body:
  rust: |
    pub fn apply_discount() -> Decimal {
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

#[test]
#[should_panic(expected = "E0412")]
fn generate_cargo_check_test_failure_includes_cargo_stderr() {
    if !cargo_available() {
        panic!("cargo is required for this test");
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
body:
  rust: |
    pub fn bad_type() -> NotAType {
        todo!()
    }
"#,
    );

    let output = run_in(&dst_ecommerce, &["generate", "units", "--output", "src/generated"]);
    assert_output_success("spec generate failed for temp ecommerce copy", &output);

    let cargo_target_dir = tempfile::TempDir::new_in(root.join("target"))
        .expect("failed to create temp cargo target dir under repo target/");
    let output = run_cargo(
        &dst_ecommerce,
        &["check", "--locked"],
        cargo_target_dir.path(),
    );
    assert_output_success(
        "expected cargo check to fail; this panic should include cargo stderr",
        &output,
    );
}
