use std::fs;
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

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Generated 1 file"));
    assert!(output_dir.join(".spec-generated").exists());
    assert!(output_dir.join("pricing/apply_discount.rs").exists());
    assert!(output_dir.join("pricing/mod.rs").exists());
    assert!(output_dir.join("mod.rs").exists());
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
