use serde_json::Value;
use spec_core::AUTHORED_SPEC_VERSION;
use spec_core::loader::{load_file, load_molecule_test_file};
use spec_core::molecule_evidence::{
    MoleculeEvidenceStatus, build_molecule_evidence, write_molecule_evidence,
};
use spec_core::passport::{
    PassportEvidence, PassportProjectionContext, PassportTestResult,
    apply_projected_passport_truth, build_passport_with_evidence, compute_contract_hash,
    project_passport_truth, read_passport as read_passport_record, write_passport,
};
use spec_core::semantic_review::{
    EvaluatorScope, SemanticProjectionMode, SemanticReasonCode, SemanticReview,
    SemanticSupportStatus, SemanticVerdict, UnsupportedFunctionReasonCode,
};
use std::collections::HashMap;
use std::fs;
use std::io;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
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

struct M9RepoFixture {
    _temp_dir: tempfile::TempDir,
    app_root: PathBuf,
    shared_root: PathBuf,
    payments_root: PathBuf,
}

fn setup_m9_repo_fixture() -> M9RepoFixture {
    let temp_dir = temp_repo_dir();
    let repo_root = temp_dir.path().join("repo");
    let app_root = repo_root.join("app-spec");
    let shared_root = repo_root.join("shared-spec");
    let payments_root = repo_root.join("payments-spec");

    fs::create_dir_all(app_root.join("units")).unwrap();
    fs::create_dir_all(shared_root.join("units")).unwrap();
    fs::create_dir_all(payments_root.join("units")).unwrap();
    fs::write(repo_root.join(".git"), "gitdir: .git/modules/spec-tests\n").unwrap();

    M9RepoFixture {
        _temp_dir: temp_dir,
        app_root,
        shared_root,
        payments_root,
    }
}

fn setup_isolated_m9_repo_fixture() -> M9RepoFixture {
    let temp_dir = tempfile::TempDir::new().unwrap();
    let repo_root = temp_dir.path().join("repo");
    let app_root = repo_root.join("app-spec");
    let shared_root = repo_root.join("shared-spec");
    let payments_root = repo_root.join("payments-spec");

    fs::create_dir_all(app_root.join("units")).unwrap();
    fs::create_dir_all(shared_root.join("units")).unwrap();
    fs::create_dir_all(payments_root.join("units")).unwrap();
    fs::write(repo_root.join(".git"), "gitdir: .git/modules/spec-tests\n").unwrap();

    M9RepoFixture {
        _temp_dir: temp_dir,
        app_root,
        shared_root,
        payments_root,
    }
}

fn write_m9_unit(dir: &Path, relative_path: &str, id: &str, deps: &[&str]) {
    let deps_yaml = if deps.is_empty() {
        String::new()
    } else {
        format!(
            "deps:\n{}\n",
            deps.iter()
                .map(|dep| format!("  - {dep}"))
                .collect::<Vec<_>>()
                .join("\n")
        )
    };

    write_spec(
        dir,
        relative_path,
        &format!(
            r#"
id: {id}
kind: function
intent:
  why: Exercise M9 validation.
spec_version: "{AUTHORED_SPEC_VERSION}"
{deps_yaml}body:
  rust: |
    {{
        true
    }}
"#
        ),
    );
}

fn write_m9_data_seam(dir: &Path, relative_path: &str, id: &str, deps: &[&str]) {
    let deps_yaml = if deps.is_empty() {
        String::new()
    } else {
        format!(
            "    deps:\n{}\n",
            deps.iter()
                .map(|dep| format!("      - {dep}"))
                .collect::<Vec<_>>()
                .join("\n")
        )
    };

    write_spec(
        dir,
        relative_path,
        &format!(
            r#"
id: {id}
kind: data
intent:
  why: Exercise M9 cross-library alias discovery for data seams.
spec_version: "{AUTHORED_SPEC_VERSION}"
data:
  fields:
    subtotal:
      type: i32
constructors:
  - id: new
    intent:
      why: Create a quote.
    contract:
      inputs:
        subtotal: i32
    initializes:
      subtotal: subtotal
methods:
  - id: total
    intent:
      why: Return the total.
    receiver: shared_ref
    contract:
      returns: i32
{deps_yaml}    lowering:
      rust:
        body: |
          {{
              round(self.subtotal)
          }}
local_tests:
  - id: happy_path
    expect: CheckoutQuote::new(5).total() == 5
backends:
  rust:
    derives:
      - Clone
      - Debug
      - PartialEq
"#
        ),
    );
}

fn write_m13_sum_seam(dir: &Path, relative_path: &str, id: &str, deps: &[&str]) {
    let deps_yaml = if deps.is_empty() {
        String::new()
    } else {
        format!(
            "    deps:\n{}\n",
            deps.iter()
                .map(|dep| format!("      - {dep}"))
                .collect::<Vec<_>>()
                .join("\n")
        )
    };

    write_spec(
        dir,
        relative_path,
        &format!(
            r#"
id: {id}
kind: sum
intent:
  why: Exercise M13 sum seam passport/export/status coverage.
spec_version: "{AUTHORED_SPEC_VERSION}"
sum:
  variants:
    pending: {{}}
    quoted_total:
      fields:
        subtotal:
          type: i32
methods:
  - id: rounded_total
    intent:
      why: Return the rounded subtotal for quoted totals.
    receiver: shared_ref
    contract:
      returns: i32
{deps_yaml}    lowering:
      rust:
        body: |
          {{
              match self {{
                  CheckoutStatus::Pending => 0,
                  CheckoutStatus::QuotedTotal {{ subtotal }} => round(*subtotal),
              }}
          }}
local_tests:
  - id: quoted_total_rounds
    expect: "CheckoutStatus::Pending.rounded_total() == 0"
backends:
  rust:
    derives:
      - Clone
      - Debug
      - PartialEq
"#
        ),
    );
}

fn write_m9_app_cargo_toml(app_root: &Path, dependency_aliases: &[&str]) {
    let dependency_lines = dependency_aliases
        .iter()
        .map(|alias| format!(r#"{alias} = {{ path = "../shared-crate" }}"#))
        .collect::<Vec<_>>()
        .join("\n");
    let dependencies = if dependency_lines.is_empty() {
        String::new()
    } else {
        format!("\n{dependency_lines}\n")
    };

    fs::write(
        app_root.join("Cargo.toml"),
        format!(
            r#"[package]
name = "app"
version = "0.1.0"
edition = "2024"

[dependencies]{dependencies}

[workspace]
"#
        ),
    )
    .unwrap();
}

fn write_invalid_m9_app_cargo_toml(app_root: &Path) {
    fs::write(
        app_root.join("Cargo.toml"),
        r#"[package]
name = "app"
version = "0.1.0"
edition = "2024"

[dependencies
shared = { path = "../shared-crate" }

[workspace]
"#,
    )
    .unwrap();
}

fn write_m9_shared_round_crate_fixture(fixture: &M9RepoFixture) {
    write_file(
        &fixture.app_root,
        "src/main.rs",
        "mod generated;\npub use generated::*;\nfn main() {}\n",
    );
    let shared_crate_root = fixture.app_root.parent().unwrap().join("shared-crate");
    write_file(
        &shared_crate_root,
        "Cargo.toml",
        r#"[package]
name = "shared"
version = "0.1.0"
edition = "2024"

[lib]
path = "src/lib.rs"
"#,
    );
    write_file(
        &shared_crate_root,
        "src/lib.rs",
        "pub mod money {\n    pub mod round {\n        pub fn round(value: i32) -> i32 {\n            value\n        }\n    }\n}\n",
    );
}

fn setup_apply_discount_unit() -> (tempfile::TempDir, PathBuf) {
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
    (temp_dir, units_dir)
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

fn status_roots(json: &Value) -> &Vec<Value> {
    json["roots"].as_array().unwrap()
}

fn status_units(json: &Value) -> &Vec<Value> {
    status_roots(json)[0]["units"].as_array().unwrap()
}

fn status_molecule_tests(json: &Value) -> &Vec<Value> {
    status_roots(json)[0]["molecule_tests"].as_array().unwrap()
}

fn fixture_json(name: &str) -> Value {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name);
    serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap()
}

// fixture flow:
// command JSON -> narrow normalization -> frozen fixture
const NORMALIZED_CONTRACT_VALUE: &str = "<normalized>";

fn read_contract_fixture(name: &str) -> Value {
    fixture_json(&format!("benchmarks/{name}"))
}

fn assert_stdout_json_matches_fixture(output: &std::process::Output, fixture: &str) {
    let actual = parse_stdout_json(output);
    let expected = fixture_json(fixture);
    assert_eq!(actual, expected);
}

fn normalize_status_contract_json(mut json: Value) -> Value {
    normalize_contract_json_value(&mut json, &mut Vec::new(), false);
    json
}

fn normalize_export_contract_json(mut json: Value) -> Value {
    normalize_contract_json_value(&mut json, &mut Vec::new(), true);
    json
}

fn normalize_contract_json_value(value: &mut Value, path: &mut Vec<String>, export_mode: bool) {
    match value {
        Value::Object(map) => {
            for (key, child) in map.iter_mut() {
                path.push(key.clone());
                normalize_contract_json_value(child, path, export_mode);
                path.pop();
            }
        }
        Value::Array(items) => {
            for (index, child) in items.iter_mut().enumerate() {
                path.push(index.to_string());
                normalize_contract_json_value(child, path, export_mode);
                path.pop();
            }
        }
        Value::String(text) => {
            let leaf = path.last().map(String::as_str);
            let should_normalize = Path::new(text).is_absolute()
                || leaf == Some("evidence_at")
                || leaf == Some("generated_at")
                || leaf == Some("observed_at")
                || (export_mode && leaf == Some("exported_at"))
                || (leaf == Some("git_commit_sha")
                    && path.iter().any(|segment| segment == "provenance"))
                || (leaf == Some("authored_truth_digest")
                    && path.iter().rev().nth(1).map(String::as_str) == Some("freshness"))
                || leaf == Some("label_digest")
                || leaf == Some("projection_digest");

            if should_normalize {
                *text = NORMALIZED_CONTRACT_VALUE.to_string();
            }
        }
        _ => {}
    }
}

fn assert_contract_matches_fixture(actual: Value, fixture: &str) {
    let expected = read_contract_fixture(fixture);
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

fn rewrite_json_field(path: &Path, field: &str, value: Value) {
    let mut json: Value = serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap();
    json[field] = value;
    fs::write(path, serde_json::to_string_pretty(&json).unwrap()).unwrap();
}

fn clear_passport_semantic_review_descriptor_id(passport_path: &Path) -> Value {
    let mut passport = read_passport_json(passport_path);
    passport["semantic_review"]
        .as_object_mut()
        .unwrap()
        .remove("descriptor_id");
    fs::write(
        passport_path,
        serde_json::to_string_pretty(&passport).unwrap(),
    )
    .unwrap();
    read_passport_json(passport_path)["semantic_review"].clone()
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
        pricing_dir.join("quote_total.unit.spec"),
        r#"
id: pricing/quote_total
kind: function
intent:
  why: Return a quoted total placeholder.
spec_version: "0.3.0"
contract:
  returns: bool
body:
  rust: |
    { true }
local_tests:
  - id: happy_path
    expect: quote_total() == true
"#,
    )
    .unwrap();

    pricing_dir.join("quote_total.unit.spec")
}

fn write_semantic_status_project(project_dir: &Path) -> PathBuf {
    let units_dir = project_dir.join("units");
    write_file(
        project_dir,
        "Cargo.toml",
        r#"[package]
name = "semantic-status-project"
version = "0.1.0"
edition = "2024"

[workspace]
"#,
    );
    write_file(
        project_dir,
        "src/main.rs",
        "mod generated;\npub use generated::*;\nfn main() {}\n",
    );
    write_spec(
        &units_dir,
        "pricing/discount_mode.unit.spec",
        r#"
id: pricing/discount_mode
kind: sum
intent:
  why: Represent discount modes that cap fixed discounts at the subtotal.
spec_version: "0.3.0"
sum:
  variants:
    none: {}
    fixed:
      fields:
        amount:
          type: i32
methods:
  - id: discount_amount
    intent:
      why: Return the capped discount amount to subtract from the subtotal.
    receiver: shared_ref
    contract:
      inputs:
        subtotal: i32
      returns: i32
    lowering:
      rust:
        body: |
          {
              match self {
                  Self::None => 0,
                  Self::Fixed { amount } => (*amount).min(subtotal),
              }
          }
  - id: capped_discount_example_holds
    intent:
      why: Support direct atom proof for the capped discount example.
    receiver: shared_ref
    contract:
      returns: bool
    lowering:
      rust:
        body: |
          {
              let fixed = Self::Fixed { amount: 10 };
              fixed.discount_amount(5) == 5
          }
local_tests:
  - id: capped_discount
    expect: "DiscountMode::None.capped_discount_example_holds()"
"#,
    );
    write_spec(
        &units_dir,
        "pricing/discount_mode_flow.test.spec",
        r#"
id: pricing/discount_mode_flow
spec_version: "0.3.0"
intent:
  why: Prove the capped discount seam behavior through molecule evidence.
covers:
  - pricing/discount_mode
body:
  rust: |
    {
        let fixed = crate::pricing::discount_mode::DiscountMode::Fixed { amount: 10 };
        assert_eq!(fixed.discount_amount(5), 5);
        let none = crate::pricing::discount_mode::DiscountMode::None;
        assert_eq!(none.discount_amount(5), 0);
    }
"#,
    );

    units_dir
}

fn seed_semantic_status_artifacts(units_dir: &Path) {
    const GENERATED_AT: &str = "2026-04-21T00:00:00Z";

    let unit_path = units_dir.join("pricing/discount_mode.unit.spec");
    let molecule_path = units_dir.join("pricing/discount_mode_flow.test.spec");

    let spec = load_file(&unit_path).unwrap();
    let molecule_test = load_molecule_test_file(&molecule_path).unwrap();
    let specs_by_id = HashMap::from([(spec.spec.id.clone(), spec.clone())]);
    let passport_evidence = PassportEvidence {
        build_status: "pass".to_string(),
        test_results: spec
            .spec
            .local_tests
            .iter()
            .map(|test| PassportTestResult {
                id: test.id.clone(),
                status: "pass".to_string(),
                reason: None,
            })
            .collect(),
        observed_at: GENERATED_AT.to_string(),
        provenance: None,
    };
    let molecule_evidence = build_molecule_evidence(
        &molecule_test,
        MoleculeEvidenceStatus::Pass,
        None,
        GENERATED_AT,
        &specs_by_id,
        None,
    );
    let molecule_evidence_by_id =
        HashMap::from([(molecule_test.test.id.clone(), molecule_evidence.clone())]);

    // Mirror the `spec test` write path closely enough for status/export reads
    // without spawning Cargo inside these integration tests.
    let mut passport = build_passport_with_evidence(
        &spec,
        GENERATED_AT,
        Some(passport_evidence),
        compute_contract_hash(&spec),
    );
    let projection_context = PassportProjectionContext {
        molecule_tests: std::slice::from_ref(&molecule_test),
        molecule_evidence_by_id: &molecule_evidence_by_id,
        specs_by_id: &specs_by_id,
        semantic_projection_mode: SemanticProjectionMode::Refresh,
    };
    let projected_truth = project_passport_truth(&spec, Some(&passport), &projection_context);
    apply_projected_passport_truth(&mut passport, projected_truth);

    write_passport(&passport, &unit_path).unwrap();
    write_molecule_evidence(&molecule_evidence, &molecule_path).unwrap();
}

fn supported_pricing_quote_semantic_review(
    verdict: SemanticVerdict,
    reason_codes: Vec<SemanticReasonCode>,
    summary: &str,
) -> SemanticReview {
    SemanticReview {
        verdict,
        compatibility_key: DATA_SEAM_COMPATIBILITY_KEY.to_string(),
        descriptor_id: Some("pricing_quote.ecommerce.v1".to_string()),
        support_status: None,
        unsupported_reason_codes: vec![],
        rewrite_hints: vec![],
        reason_codes,
        summary: summary.to_string(),
        authored_surfaces: vec![],
        executable_surfaces: vec![],
        evaluator_scope: EvaluatorScope::SupportedDataSurface,
    }
}

fn seed_supported_data_semantic_status_artifacts(
    units_dir: &Path,
    semantic_review: Option<SemanticReview>,
) {
    const GENERATED_AT: &str = "2026-04-21T00:00:00Z";

    let unit_path = units_dir.join("pricing/pricing_quote.unit.spec");
    let molecule_path = units_dir.join("pricing/pricing_quote_flow.test.spec");

    let spec = load_file(&unit_path).unwrap();
    let molecule_test = load_molecule_test_file(&molecule_path).unwrap();
    let specs_by_id = HashMap::from([(spec.spec.id.clone(), spec.clone())]);
    let passport_evidence = PassportEvidence {
        build_status: "pass".to_string(),
        test_results: spec
            .spec
            .local_tests
            .iter()
            .map(|test| PassportTestResult {
                id: test.id.clone(),
                status: "pass".to_string(),
                reason: None,
            })
            .collect(),
        observed_at: GENERATED_AT.to_string(),
        provenance: None,
    };
    let molecule_evidence = build_molecule_evidence(
        &molecule_test,
        MoleculeEvidenceStatus::Pass,
        None,
        GENERATED_AT,
        &specs_by_id,
        None,
    );
    let molecule_evidence_by_id =
        HashMap::from([(molecule_test.test.id.clone(), molecule_evidence.clone())]);

    let mut passport = build_passport_with_evidence(
        &spec,
        GENERATED_AT,
        Some(passport_evidence),
        compute_contract_hash(&spec),
    );
    let projection_context = PassportProjectionContext {
        molecule_tests: std::slice::from_ref(&molecule_test),
        molecule_evidence_by_id: &molecule_evidence_by_id,
        specs_by_id: &specs_by_id,
        semantic_projection_mode: SemanticProjectionMode::Refresh,
    };
    let projected_truth = project_passport_truth(&spec, Some(&passport), &projection_context);
    apply_projected_passport_truth(&mut passport, projected_truth);
    if let Some(review) = semantic_review {
        passport.semantic_review = Some(review);
    }

    write_passport(&passport, &unit_path).unwrap();
    write_molecule_evidence(&molecule_evidence, &molecule_path).unwrap();
}

fn write_supported_function_semantic_status_project(project_dir: &Path) -> PathBuf {
    let units_dir = project_dir.join("units");
    write_file(
        project_dir,
        "Cargo.toml",
        r#"[package]
name = "supported-function-semantic-status-project"
version = "0.1.0"
edition = "2024"

[dependencies]
rust_decimal = { version = "1.36", features = ["serde"] }

[workspace]
"#,
    );
    write_file(
        project_dir,
        "src/main.rs",
        "mod generated;\npub use generated::*;\nfn main() {}\n",
    );
    write_spec(
        &units_dir,
        "money/round.unit.spec",
        r#"
id: money/round
kind: function
intent:
  why: Round a decimal value to two fractional digits for pricing flows.
spec_version: "0.3.0"
contract:
  inputs:
    value: Decimal
  returns: Decimal
imports:
  - rust_decimal::Decimal
body:
  rust: |
    {
        value
    }
local_tests:
  - id: basic
    expect: "round(Decimal::new(1001, 2)) == Decimal::new(1001, 2)"
"#,
    );
    write_spec(
        &units_dir,
        "pricing/apply_discount.unit.spec",
        r#"
id: pricing/apply_discount
kind: function
intent:
  why: Apply a discount to a subtotal while keeping the result nonnegative.
spec_version: "0.3.0"
contract:
  inputs:
    subtotal: Decimal
    rate: Decimal
  returns: Decimal
  invariants:
    - output <= subtotal
    - output >= 0
deps:
  - money/round
imports:
  - rust_decimal::Decimal
body:
  rust: |
    {
        let discounted = subtotal - subtotal * rate;
        round(discounted.max(Decimal::ZERO))
    }
local_tests:
  - id: basic
    expect: "apply_discount(Decimal::new(10000, 2), Decimal::new(10, 2)) == Decimal::new(9000, 2)"
"#,
    );

    units_dir
}

fn write_supported_helper_function_semantic_status_project(project_dir: &Path) -> PathBuf {
    let units_dir = project_dir.join("units");
    write_file(
        project_dir,
        "Cargo.toml",
        r#"[package]
name = "supported-helper-function-semantic-status-project"
version = "0.1.0"
edition = "2024"

[dependencies]
rust_decimal = { version = "1.36", features = ["serde"] }

[workspace]
"#,
    );
    write_file(
        project_dir,
        "src/main.rs",
        "mod generated;\npub use generated::*;\nfn main() {}\n",
    );
    write_spec(
        &units_dir,
        "money/round.unit.spec",
        r#"
id: money/round
kind: function
intent:
  why: Round a decimal value to two fractional digits for pricing flows.
spec_version: "0.3.0"
contract:
  inputs:
    value: Decimal
  returns: Decimal
imports:
  - rust_decimal::Decimal
  - rust_decimal::RoundingStrategy
body:
  rust: |
    {
        value.round_dp_with_strategy(2, RoundingStrategy::MidpointAwayFromZero)
    }
local_tests:
  - id: rounds_half_up
    expect: "round(Decimal::new(12345, 3)) == Decimal::new(1235, 2)"
"#,
    );

    units_dir
}

const FUNCTION_FAMILY_A_COMPATIBILITY_KEY: &str =
    "function.arithmetic_leaf.monotone_down_nonnegative.v1";
const FUNCTION_FAMILY_A_UP_COMPATIBILITY_KEY: &str = "function.arithmetic_leaf.monotone_up.v1";
const FUNCTION_FAMILY_A_LEGACY_COMPATIBILITY_KEY: &str = "function.apply_discount.v1";
const FUNCTION_FAMILY_B_NORMALIZED_REQUIRED_ARG_COMPATIBILITY_KEY: &str =
    "function.wrapper.pipeline.normalized_required_arg.v1";
const FUNCTION_FAMILY_B_COMPATIBILITY_KEY: &str = "function.wrapper.pipeline.v1";
const FUNCTION_FAMILY_B_LEGACY_COMPATIBILITY_KEY: &str = "function.calculate_total.v1";
const FUNCTION_FAMILY_CHAIN3_COMPATIBILITY_KEY: &str = "function.wrapper.pipeline.chain3.v1";
const FUNCTION_HELPER_IDENTITY_PASSTHROUGH_COMPATIBILITY_KEY: &str =
    "function.helper.identity_passthrough.v1";
const DATA_SEAM_COMPATIBILITY_KEY: &str = "data.pricing_quote.v1";
const DATA_SEAM_LEGACY_COMPATIBILITY_KEY: &str = "data.checkout_quote.v1";
const SUM_SEAM_COMPATIBILITY_KEY: &str = "sum.discount_strategy.v1";
const SUM_SEAM_LEGACY_COMPATIBILITY_KEY: &str = "sum.discount_policy.v1";

fn load_unit_specs_by_id(units_dir: &Path) -> HashMap<String, spec_core::types::LoadedSpec> {
    WalkDir::new(units_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| {
            entry.file_type().is_file()
                && entry.file_name().to_string_lossy().ends_with(".unit.spec")
        })
        .map(|entry| {
            let spec = load_file(entry.path()).unwrap();
            (spec.spec.id.clone(), spec)
        })
        .collect()
}

fn seed_function_semantic_status_artifacts(
    units_dir: &Path,
    unit_relative_path: &str,
    semantic_review: Option<SemanticReview>,
) -> Option<SemanticReview> {
    const GENERATED_AT: &str = "2026-04-21T00:00:00Z";

    let unit_path = units_dir.join(unit_relative_path);
    let spec = load_file(&unit_path).unwrap();
    let specs_by_id = load_unit_specs_by_id(units_dir);
    let passport_evidence = PassportEvidence {
        build_status: "pass".to_string(),
        test_results: spec
            .spec
            .local_tests
            .iter()
            .map(|test| PassportTestResult {
                id: test.id.clone(),
                status: "pass".to_string(),
                reason: None,
            })
            .collect(),
        observed_at: GENERATED_AT.to_string(),
        provenance: None,
    };

    let mut passport = build_passport_with_evidence(
        &spec,
        GENERATED_AT,
        Some(passport_evidence),
        compute_contract_hash(&spec),
    );
    let molecule_evidence_by_id = HashMap::new();
    let projection_context = PassportProjectionContext {
        molecule_tests: &[],
        molecule_evidence_by_id: &molecule_evidence_by_id,
        specs_by_id: &specs_by_id,
        semantic_projection_mode: SemanticProjectionMode::Refresh,
    };
    let projected_truth = project_passport_truth(&spec, Some(&passport), &projection_context);
    apply_projected_passport_truth(&mut passport, projected_truth);
    let projected_review = passport
        .semantic_review
        .clone()
        .filter(|review| review.compatibility_key != "unsupported.function.v1");
    if let Some(review) = semantic_review {
        passport.semantic_review = Some(review);
    }

    write_passport(&passport, &unit_path).unwrap();
    projected_review
}

fn seed_supported_function_semantic_status_artifacts(
    units_dir: &Path,
    semantic_review: Option<SemanticReview>,
) -> Option<SemanticReview> {
    seed_function_semantic_status_artifacts(
        units_dir,
        "pricing/apply_discount.unit.spec",
        semantic_review,
    )
}

fn seed_supported_wrapper_function_semantic_status_artifacts(
    units_dir: &Path,
    semantic_review: Option<SemanticReview>,
) -> Option<SemanticReview> {
    seed_function_semantic_status_artifacts(
        units_dir,
        "pricing/calculate_total.unit.spec",
        semantic_review,
    )
}

fn write_supported_wrapper_function_semantic_status_project(project_dir: &Path) -> PathBuf {
    let units_dir = project_dir.join("units");
    write_file(
        project_dir,
        "Cargo.toml",
        r#"[package]
name = "supported-wrapper-function-semantic-status-project"
version = "0.1.0"
edition = "2024"

[dependencies]
rust_decimal = { version = "1.36", features = ["serde"] }

[workspace]
"#,
    );
    write_file(
        project_dir,
        "src/main.rs",
        "mod generated;\npub use generated::*;\nfn main() {}\n",
    );
    write_spec(
        &units_dir,
        "money/round.unit.spec",
        r#"
id: money/round
kind: function
intent:
  why: Round a decimal value to two fractional digits for pricing flows.
spec_version: "0.3.0"
contract:
  inputs:
    value: Decimal
  returns: Decimal
imports:
  - rust_decimal::Decimal
body:
  rust: |
    {
        value
    }
local_tests:
  - id: basic
    expect: "round(Decimal::new(1001, 2)) == Decimal::new(1001, 2)"
"#,
    );
    write_spec(
        &units_dir,
        "pricing/apply_discount.unit.spec",
        r#"
id: pricing/apply_discount
kind: function
intent:
  why: Apply a discount to a subtotal while keeping the result nonnegative.
spec_version: "0.3.0"
contract:
  inputs:
    subtotal: Decimal
    rate: Decimal
  returns: Decimal
  invariants:
    - output <= subtotal
    - output >= 0
deps:
  - money/round
imports:
  - rust_decimal::Decimal
body:
  rust: |
    {
        let discounted = subtotal - subtotal * rate;
        round(discounted.max(Decimal::ZERO))
    }
local_tests:
  - id: basic
    expect: "apply_discount(Decimal::new(10000, 2), Decimal::new(10, 2)) == Decimal::new(9000, 2)"
"#,
    );
    write_spec(
        &units_dir,
        "pricing/apply_tax.unit.spec",
        r#"
id: pricing/apply_tax
kind: function
intent:
  why: Add sales tax to a subtotal using a rate expressed as a decimal fraction.
spec_version: "0.3.0"
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
    expect: "apply_tax(Decimal::new(10000, 2), Decimal::new(725, 4)) == Decimal::new(10725, 2)"
"#,
    );
    write_spec(
        &units_dir,
        "pricing/calculate_total.unit.spec",
        r#"
id: pricing/calculate_total
kind: function
intent:
  why: Return the total after discounting the subtotal and then applying tax.
spec_version: "0.3.0"
contract:
  inputs:
    subtotal: Decimal
    discount_rate: Decimal
    tax_rate: Decimal
  returns: Decimal
deps:
  - pricing/apply_discount
  - pricing/apply_tax
imports:
  - rust_decimal::Decimal
body:
  rust: |
    {
        let discounted = apply_discount(subtotal, discount_rate);
        apply_tax(discounted, tax_rate)
    }
local_tests:
  - id: combined_flow
    expect: "calculate_total(Decimal::new(10000, 2), Decimal::new(10, 2), Decimal::new(725, 4)) == Decimal::new(96525, 3)"
"#,
    );

    units_dir
}

fn write_unsupported_function_semantic_status_project(project_dir: &Path) -> PathBuf {
    let units_dir = write_supported_wrapper_function_semantic_status_project(project_dir);
    let unit_path = units_dir.join("pricing/calculate_total.unit.spec");
    let source = fs::read_to_string(&unit_path).unwrap();
    fs::write(
        &unit_path,
        source.replace(
            "    {\n        let discounted = apply_discount(subtotal, discount_rate);\n        apply_tax(discounted, tax_rate)\n    }\n",
            "    {\n        apply_tax(\n            apply_discount(subtotal, discount_rate),\n            tax_rate.max(Decimal::ZERO).round_dp(4),\n        )\n    }\n",
        ),
    )
    .unwrap();
    units_dir
}

fn write_supported_normalized_required_arg_wrapper_function_semantic_status_project(
    project_dir: &Path,
) -> PathBuf {
    let units_dir = write_supported_wrapper_function_semantic_status_project(project_dir);
    let unit_path = units_dir.join("pricing/calculate_total.unit.spec");
    let source = fs::read_to_string(&unit_path).unwrap();
    fs::write(
        &unit_path,
        source.replace(
            "    {\n        let discounted = apply_discount(subtotal, discount_rate);\n        apply_tax(discounted, tax_rate)\n    }\n",
            "    {\n        let discounted = apply_discount(subtotal, discount_rate);\n        apply_tax(discounted, tax_rate.max(Decimal::ZERO))\n    }\n",
        ),
    )
    .unwrap();
    units_dir
}

fn unsupported_function_semantic_review(summary: &str) -> SemanticReview {
    SemanticReview {
        verdict: SemanticVerdict::UnderSpecified,
        compatibility_key: "unsupported.function.v1".to_string(),
        descriptor_id: None,
        support_status: Some(SemanticSupportStatus::Unsupported),
        unsupported_reason_codes: vec![
            UnsupportedFunctionReasonCode::UnsupportedRequiredArgumentExpression,
        ],
        rewrite_hints: vec![
            "pass bare parameter paths to supported wrappers instead of computed argument expressions"
                .to_string(),
        ],
        reason_codes: vec![SemanticReasonCode::UnsupportedSurface],
        summary: summary.to_string(),
        authored_surfaces: vec![],
        executable_surfaces: vec![],
        evaluator_scope: EvaluatorScope::UnsupportedSurface,
    }
}

fn supported_function_semantic_review(
    compatibility_key: &str,
    verdict: SemanticVerdict,
    reason_codes: Vec<SemanticReasonCode>,
    summary: &str,
) -> SemanticReview {
    SemanticReview {
        verdict,
        compatibility_key: compatibility_key.to_string(),
        descriptor_id: None,
        support_status: Some(SemanticSupportStatus::Supported),
        unsupported_reason_codes: vec![],
        rewrite_hints: vec![],
        reason_codes,
        summary: summary.to_string(),
        authored_surfaces: vec![],
        executable_surfaces: vec![],
        evaluator_scope: EvaluatorScope::SupportedFunctionSurface,
    }
}

fn seed_unsupported_function_semantic_status_artifacts(
    units_dir: &Path,
    semantic_review: Option<SemanticReview>,
) {
    seed_function_semantic_status_artifacts(
        units_dir,
        "pricing/calculate_total.unit.spec",
        semantic_review,
    );
}

fn assert_semantic_review_absent(review: &Value) {
    assert!(review.is_null(), "{review}");
}

fn assert_unsupported_function_semantic_review(review: &Value) {
    assert_eq!(review["verdict"], "under_specified");
    assert_eq!(review["evaluator_scope"], "unsupported_surface");
    assert_eq!(review["support_status"], "unsupported");
    assert_eq!(review["compatibility_key"], "unsupported.function.v1");
    assert_eq!(
        review["reason_codes"],
        serde_json::json!(["unsupported_surface"])
    );
    assert!(
        review["summary"]
            .as_str()
            .is_some_and(|summary| !summary.trim().is_empty()),
        "expected non-empty unsupported summary for {review}"
    );
    assert!(
        review["unsupported_reason_codes"]
            .as_array()
            .is_some_and(|codes| !codes.is_empty()),
        "expected unsupported_reason_codes for {review}"
    );
    assert!(
        review["rewrite_hints"]
            .as_array()
            .is_some_and(|hints| !hints.is_empty()),
        "expected rewrite_hints for {review}"
    );
}

fn assert_supported_function_semantic_review(review: &Value, compatibility_key: &str) {
    assert_eq!(review["evaluator_scope"], "supported_function_surface");
    assert_eq!(review["support_status"], "supported");
    assert_eq!(review["compatibility_key"], compatibility_key);
    assert!(
        review["unsupported_reason_codes"]
            .as_array()
            .is_none_or(|codes| codes.is_empty()),
        "expected empty unsupported_reason_codes for {review}"
    );
    assert!(
        review["rewrite_hints"]
            .as_array()
            .is_none_or(|hints| hints.is_empty()),
        "expected empty rewrite_hints for {review}"
    );
}

fn assert_unsupported_function_reason(review: &Value, reason: &str) {
    assert_unsupported_function_semantic_review(review);
    assert_eq!(review["unsupported_reason_codes"][0], reason, "{review}");
}

fn copy_m19_semantic_falsification_pack() -> (tempfile::TempDir, PathBuf) {
    let temp_dir = temp_repo_dir();
    let fixture_dir = temp_dir.path().join("semantic_falsification_pack");
    copy_dir_recursive(
        &repo_root().join("spec-cli/tests/fixtures/m19/semantic_falsification_pack"),
        &fixture_dir,
    )
    .expect("failed to copy M19 semantic falsification fixture");
    (temp_dir, fixture_dir)
}

fn copy_m20_unsupported_truth_pack() -> (tempfile::TempDir, PathBuf) {
    let temp_dir = temp_repo_dir();
    let fixture_dir = temp_dir.path().join("unsupported_truth_pack");
    copy_dir_recursive(
        &repo_root().join("spec-cli/tests/fixtures/m20/unsupported_truth_pack"),
        &fixture_dir,
    )
    .expect("failed to copy M20 unsupported truth fixture");
    (temp_dir, fixture_dir)
}

fn copy_m21_chain3_fixture(bucket: &str) -> (tempfile::TempDir, PathBuf) {
    let temp_dir = temp_repo_dir();
    let fixture_dir = temp_dir.path().join(format!("m21_chain3_{bucket}"));
    copy_dir_recursive(
        &repo_root()
            .join("semantic-families/function.wrapper.pipeline.chain3.v1/fixtures")
            .join(bucket),
        &fixture_dir,
    )
    .expect("failed to copy M21 chain3 fixture");
    (temp_dir, fixture_dir)
}

fn copy_m30_wrapper_fixture(bucket: &str) -> (tempfile::TempDir, PathBuf) {
    let temp_dir = temp_repo_dir();
    let fixture_dir = temp_dir.path().join(format!("m30_wrapper_{bucket}"));
    copy_dir_recursive(
        &repo_root()
            .join("semantic-families/function.wrapper.pipeline.v1/fixtures")
            .join(bucket),
        &fixture_dir,
    )
    .expect("failed to copy M30 wrapper fixture");
    (temp_dir, fixture_dir)
}

fn copy_typescript_local_supported_graph_fixture() -> (tempfile::TempDir, PathBuf) {
    let temp_dir = temp_repo_dir();
    let fixture_dir = temp_dir.path().join("typescript_local_supported_graph");
    copy_dir_recursive(
        &repo_root().join("spec-cli/tests/fixtures/typescript_local_supported_graph"),
        &fixture_dir,
    )
    .expect("failed to copy local supported-graph fixture");
    fs::write(
        temp_dir.path().join(".git"),
        "gitdir: .git/modules/spec-tests\n",
    )
    .expect("failed to mark local supported-graph temp repo root");
    (temp_dir, fixture_dir)
}

fn inject_typescript_body_if_missing(unit_path: &Path, typescript_body: &str) {
    let contents = fs::read_to_string(unit_path).unwrap();
    if contents.contains("\n  typescript: |\n") {
        return;
    }

    let rewritten = contents.replace(
        "\nlocal_tests:\n",
        &format!("\n  typescript: |\n{typescript_body}\nlocal_tests:\n"),
    );
    assert_ne!(
        rewritten,
        contents,
        "expected to inject a typescript body into `{}`",
        unit_path.display()
    );
    fs::write(unit_path, rewritten).unwrap();
}

fn remove_typescript_body(unit_path: &Path) {
    let contents = fs::read_to_string(unit_path).unwrap();
    let (before, after) = contents
        .split_once("\n  typescript: |\n")
        .expect("expected a typescript body block to remove");
    let (_, tail) = after
        .split_once("\nlocal_tests:\n")
        .expect("expected local_tests after the typescript body block");
    let rewritten = format!("{before}\nlocal_tests:\n{tail}");
    fs::write(unit_path, rewritten).unwrap();
}

fn replace_in_file(path: &Path, from: &str, to: &str) {
    let contents = fs::read_to_string(path).unwrap();
    let rewritten = contents.replace(from, to);
    assert_ne!(
        rewritten,
        contents,
        "expected fixture rewrite for `{}`",
        path.display()
    );
    fs::write(path, rewritten).unwrap();
}

fn setup_m12_data_seam_project() -> (tempfile::TempDir, PathBuf) {
    let temp_dir = temp_repo_dir();
    let project_dir = temp_dir.path().join("m12-data-seam");
    let units_dir = project_dir.join("units");

    write_file(
        &project_dir,
        "Cargo.toml",
        r#"[package]
name = "m12-data-seam"
version = "0.1.0"
edition = "2024"

[dependencies]
rust_decimal = { version = "1.36", features = ["serde"] }

[workspace]
"#,
    );
    write_file(
        &project_dir,
        "src/main.rs",
        "mod generated;\npub use generated::*;\nfn main() {}\n",
    );

    write_spec(
        &units_dir,
        "pricing/apply_discount.unit.spec",
        r#"
id: pricing/apply_discount
kind: function
spec_version: "0.3.0"
intent:
  why: Apply a discount.
contract:
  inputs:
    subtotal: Decimal
    discount_rate: Decimal
  returns: Decimal
imports:
  - rust_decimal::Decimal
body:
  rust: |
    {
        subtotal - subtotal * discount_rate
    }
"#,
    );
    write_spec(
        &units_dir,
        "pricing/apply_tax.unit.spec",
        r#"
id: pricing/apply_tax
kind: function
spec_version: "0.3.0"
intent:
  why: Apply tax to a subtotal.
contract:
  inputs:
    subtotal: Decimal
    tax_rate: Decimal
  returns: Decimal
imports:
  - rust_decimal::Decimal
body:
  rust: |
    {
        subtotal + subtotal * tax_rate
    }
"#,
    );
    write_spec(
        &units_dir,
        "pricing/pricing_quote.unit.spec",
        r#"
id: pricing/pricing_quote
kind: data
spec_version: "0.3.0"
intent:
  why: Quote a checkout total from subtotal plus discount and tax rates.
data:
  fields:
    subtotal:
      type: rust_decimal::Decimal
    discount_rate:
      type: rust_decimal::Decimal
    tax_rate:
      type: rust_decimal::Decimal
constructors:
  - id: new
    intent:
      why: Create a quote from explicit subtotal and rates.
    contract:
      inputs:
        subtotal: rust_decimal::Decimal
        discount_rate: rust_decimal::Decimal
        tax_rate: rust_decimal::Decimal
    initializes:
      subtotal: subtotal
      discount_rate: discount_rate
      tax_rate: tax_rate
methods:
  - id: discounted_subtotal
    intent:
      why: Return the discounted subtotal before tax.
    receiver: shared_ref
    contract:
      returns: rust_decimal::Decimal
    deps:
      - pricing/apply_discount
    lowering:
      rust:
        body: |
          {
              apply_discount(self.subtotal, self.discount_rate)
          }
  - id: total
    intent:
      why: Return the final checkout total after discount and tax.
    receiver: shared_ref
    contract:
      returns: rust_decimal::Decimal
    deps:
      - pricing/apply_tax
    lowering:
      rust:
        body: |
          {
              apply_tax(self.discounted_subtotal(), self.tax_rate)
          }
local_tests:
  - id: total_basic
    expect: PricingQuote::new(rust_decimal::Decimal::new(10000, 2), rust_decimal::Decimal::new(10, 2), rust_decimal::Decimal::new(725, 4)).total() == rust_decimal::Decimal::new(96525, 3)
backends:
  rust:
    derives:
      - Clone
      - Debug
      - PartialEq
"#,
    );
    write_spec(
        &units_dir,
        "pricing/pricing_quote_flow.test.spec",
        r#"
id: pricing/pricing_quote_flow
intent:
  why: Verify the generated checkout quote seam composes with pricing helpers.
covers:
  - pricing/pricing_quote
body:
  rust: |
    {
        let quote = PricingQuote::new(
            rust_decimal::Decimal::new(10000, 2),
            rust_decimal::Decimal::new(10, 2),
            rust_decimal::Decimal::new(725, 4),
        );
        assert_eq!(quote.total(), rust_decimal::Decimal::new(96525, 3));
    }
"#,
    );

    (temp_dir, project_dir)
}

fn setup_m13_sum_seam_project() -> (tempfile::TempDir, PathBuf) {
    let temp_dir = temp_repo_dir();
    let project_dir = temp_dir.path().join("m13-sum-seam");
    let units_dir = project_dir.join("units");

    write_file(
        &project_dir,
        "Cargo.toml",
        r#"[package]
name = "m13-sum-seam"
version = "0.1.0"
edition = "2024"

[workspace]
"#,
    );
    write_file(
        &project_dir,
        "src/main.rs",
        "mod generated;\npub use generated::*;\nfn main() {}\n",
    );

    write_spec(
        &units_dir,
        "pricing/apply_discount.unit.spec",
        r#"
id: pricing/apply_discount
kind: function
spec_version: "0.3.0"
intent:
  why: Apply a flat discount.
contract:
  inputs:
    subtotal: i32
    discount: i32
  returns: i32
body:
  rust: |
    {
        subtotal - discount
    }
"#,
    );
    write_spec(
        &units_dir,
        "pricing/apply_tax.unit.spec",
        r#"
id: pricing/apply_tax
kind: function
spec_version: "0.3.0"
intent:
  why: Apply a flat tax amount.
contract:
  inputs:
    subtotal: i32
    tax_rate: i32
  returns: i32
body:
  rust: |
    {
        subtotal + tax_rate
    }
"#,
    );
    write_spec(
        &units_dir,
        "pricing/checkout_quote.unit.spec",
        r#"
id: pricing/checkout_quote
kind: data
spec_version: "0.3.0"
intent:
  why: Capture a quoted subtotal and tax rate.
data:
  fields:
    subtotal:
      type: i32
    tax_rate:
      type: i32
constructors:
  - id: new
    intent:
      why: Create a quote.
    contract:
      inputs:
        subtotal: i32
        tax_rate: i32
    initializes:
      subtotal: subtotal
      tax_rate: tax_rate
methods:
  - id: total
    intent:
      why: Return the total.
    receiver: shared_ref
    contract:
      returns: i32
    deps:
      - pricing/apply_tax
    lowering:
      rust:
        body: |
          {
              apply_tax(self.subtotal, self.tax_rate)
          }
backends:
  rust:
    derives:
      - Clone
      - Debug
      - PartialEq
"#,
    );
    write_spec(
        &units_dir,
        "pricing/checkout_status.unit.spec",
        r#"
id: pricing/checkout_status
kind: sum
spec_version: "0.3.0"
intent:
  why: Track checkout state as a seam-owned enum.
sum:
  variants:
    pending: {}
    quoted_total:
      fields:
        subtotal:
          type: i32
        tax_rate:
          type: i32
    failed:
      fields:
        code:
          type: i32
methods:
  - id: label
    intent:
      why: Return a variant label.
    receiver: shared_ref
    contract:
      returns: "&'static str"
    deps:
      - pricing/apply_discount
    lowering:
      rust:
        body: |
          {
              match self {
                  Self::Pending => "pending",
                  Self::QuotedTotal { .. } => {
                      let _ = apply_discount(1, 0);
                      "quoted_total"
                  }
                  Self::Failed { .. } => "failed",
              }
          }
  - id: total
    intent:
      why: Return the checkout total for quoted totals.
    receiver: shared_ref
    contract:
      returns: i32
    deps:
      - pricing/apply_discount
      - pricing/apply_tax
    lowering:
      rust:
        body: |
          {
              match self {
                  Self::Pending => 0,
                  Self::QuotedTotal { subtotal, tax_rate } => {
                      let discounted = apply_discount(*subtotal, 1);
                      apply_tax(discounted, *tax_rate)
                  }
                  Self::Failed { .. } => 0,
              }
          }
local_tests:
  - id: quoted_total_total
    expect: "CheckoutStatus::Pending.total() == 0"
backends:
  rust:
    derives:
      - Clone
      - Debug
      - PartialEq
"#,
    );

    (temp_dir, project_dir)
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
fn generate_single_file_path_is_rejected_and_leaves_output_untouched() {
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

    fs::create_dir_all(&output_dir).unwrap();
    fs::write(output_dir.join(".spec-generated"), "").unwrap();
    let output = run(&[
        "generate",
        spec_path.to_str().unwrap(),
        "--output",
        output_dir.to_str().unwrap(),
    ]);
    assert!(
        !output.status.success(),
        "expected single-file generate to fail\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("spec generate requires a directory path"),
        "unexpected stderr: {stderr}"
    );
    assert!(
        !output_dir.join("pricing/apply_discount.rs").exists(),
        "single-file generate should not write output files"
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
fn build_help_requires_directory_path_description() {
    let output = run(&["build", "--help"]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Directory containing .unit.spec files"),
        "expected directory-scoped build help, got: {stdout}"
    );
    assert!(
        !stdout.contains("or a single .unit.spec file"),
        "build help should not advertise single-file support, got: {stdout}"
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
fn spec_validate_json_local_cycle_keeps_cyclic_dep_code() {
    let temp_dir = temp_repo_dir();
    let units_dir = temp_dir.path().join("units");
    write_spec(
        &units_dir,
        "a/foo.unit.spec",
        r#"
id: a/foo
kind: function
intent:
  why: Exercise local cycle JSON reporting.
spec_version: "0.3.0"
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
  why: Close the local cycle.
spec_version: "0.3.0"
deps:
  - a/foo
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

    let json = parse_stdout_json(&output);
    let errors = json["errors"].as_array().unwrap();
    assert!(
        errors
            .iter()
            .any(|error| error["code"] == "SPEC_CYCLIC_DEP"),
        "expected SPEC_CYCLIC_DEP, got: {errors:?}"
    );
    assert!(
        !errors
            .iter()
            .any(|error| error["code"] == "SPEC_CROSS_LIBRARY_CYCLE"),
        "unexpected SPEC_CROSS_LIBRARY_CYCLE, got: {errors:?}"
    );
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
    assert_eq!(json["schema_version"], 3);
    assert_eq!(json["status"], "valid");
    assert_eq!(json["errors"], serde_json::json!([]));
    assert_eq!(json["warnings"], serde_json::json!([]));
}

#[test]
fn spec_validate_json_schema_version_is_3() {
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
    assert_eq!(json["schema_version"], 3);
}

#[test]
fn generate_empty_directory_reports_zero_units() {
    let temp_dir = temp_repo_dir();
    let output_dir = temp_dir.path().join("generated/spec");
    fs::create_dir_all(output_dir.join("pricing")).unwrap();
    fs::write(output_dir.join(".spec-generated"), "").unwrap();
    fs::write(output_dir.join("mod.rs"), "pub mod pricing;\n").unwrap();
    fs::write(
        output_dir.join("pricing/mod.rs"),
        "pub mod molecule_tests;\n",
    )
    .unwrap();
    fs::write(
        output_dir.join("pricing/molecule_tests.rs"),
        "#[test]\nfn stale() { assert!(false, \"stale molecule\"); }\n",
    )
    .unwrap();

    let output = run(&[
        "generate",
        temp_dir.path().to_str().unwrap(),
        "--output",
        output_dir.to_str().unwrap(),
    ]);
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("0 units found, nothing to generate"));
    assert!(
        !output_dir.join("mod.rs").exists(),
        "stale root mod.rs should be removed"
    );
    assert!(
        !output_dir.join("pricing/molecule_tests.rs").exists(),
        "stale molecule_tests.rs should be removed"
    );
    assert!(
        !output_dir.join("pricing").exists(),
        "empty namespace directories should be removed"
    );
}

#[test]
fn generate_directory_with_only_molecule_tests_fails_and_cleans_stale_output() {
    let temp_dir = temp_repo_dir();
    let units_dir = temp_dir.path().join("units");
    let output_dir = temp_dir.path().join("src/generated");

    write_spec(
        &units_dir,
        "pricing/only.test.spec",
        r#"id: pricing/only
spec_version: "0.3.0"
intent:
  why: Exercise the molecule-only zero-unit path.
covers:
  - pricing/missing
body:
  rust: |
    {
        assert!(true);
    }
"#,
    );

    fs::create_dir_all(output_dir.join("pricing")).unwrap();
    fs::write(output_dir.join(".spec-generated"), "").unwrap();
    fs::write(output_dir.join("mod.rs"), "pub mod pricing;\n").unwrap();
    fs::write(
        output_dir.join("pricing/mod.rs"),
        "pub mod molecule_tests;\n",
    )
    .unwrap();
    fs::write(
        output_dir.join("pricing/molecule_tests.rs"),
        "#[test]\nfn stale() { assert!(false, \"stale molecule\"); }\n",
    )
    .unwrap();

    let output = run(&[
        "generate",
        units_dir.to_str().unwrap(),
        "--output",
        output_dir.to_str().unwrap(),
    ]);
    assert!(
        !output.status.success(),
        "generate should fail for a molecule-only tree"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("pricing/missing"),
        "expected missing cover diagnostic\nstderr: {stderr}"
    );
    assert!(
        !output_dir.join("mod.rs").exists(),
        "stale root mod.rs should be removed before failing"
    );
    assert!(
        !output_dir.join("pricing/molecule_tests.rs").exists(),
        "stale molecule_tests.rs should be removed before failing"
    );
    assert!(
        !output_dir.join("pricing").exists(),
        "empty namespace directories should be removed before failing"
    );
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
    assert_eq!(bundle["schema_version"], 5);
    assert_eq!(bundle["units"].as_array().unwrap().len(), 2);
    assert!(bundle.get("graph").is_some());
    assert!(bundle.get("molecule_tests").is_some());
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
        serde_json::json!([{
            "kind": "dep",
            "from": { "library": null, "id": "pricing/apply_discount" },
            "to": { "library": null, "id": "money/round" }
        }])
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

fn bun_available() -> bool {
    Command::new("bun").arg("--version").output().is_ok()
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

    let (_temp_dir, ecommerce_dir) = copy_ecommerce_example();

    let output = run_in(
        &ecommerce_dir,
        &["generate", "units", "--output", "src/generated"],
    );
    assert_output_success("spec generate failed for ecommerce example", &output);

    let cargo_target_dir = tempfile::TempDir::new_in(repo_root().join("target"))
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

#[test]
fn generate_cargo_check_on_cross_library_example() {
    if !cargo_available() {
        return;
    }

    let root = repo_root();
    let temp_dir = temp_repo_dir();
    let app_dir = temp_dir.path().join("crosslib-app");
    let shared_crate_dir = temp_dir.path().join("shared-crate");
    let shared_spec_dir = temp_dir.path().join("shared-spec");

    fs::write(
        temp_dir.path().join(".git"),
        "gitdir: .git/modules/spec-tests\n",
    )
    .unwrap();
    copy_dir_recursive(&root.join("examples/crosslib-app"), &app_dir).unwrap();
    copy_dir_recursive(&root.join("examples/shared-crate"), &shared_crate_dir).unwrap();
    copy_dir_recursive(&root.join("examples/shared-spec"), &shared_spec_dir).unwrap();

    let output = run_in(
        temp_dir.path(),
        &[
            "generate",
            "shared-spec/units",
            "--output",
            "shared-crate/src/generated",
        ],
    );
    assert_output_success("spec generate failed for shared example", &output);

    let output = run_in(
        &app_dir,
        &["generate", "units", "--output", "src/generated"],
    );
    assert_output_success(
        "spec generate failed for cross-library app example",
        &output,
    );

    let cargo_target_dir = tempfile::TempDir::new_in(root.join("target"))
        .expect("failed to create temp cargo target dir under repo target/");

    let output = run_cargo(&app_dir, &["check"], cargo_target_dir.path());
    assert_output_success("cargo check failed for cross-library app example", &output);

    let output = run_cargo(&app_dir, &["test"], cargo_target_dir.path());
    assert_output_success("cargo test failed for cross-library app example", &output);
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

fn copy_git_tracked_dir_from_repo(repo: &Path, src: &Path, dst: &Path) -> io::Result<()> {
    let relative_src = src
        .strip_prefix(repo)
        .expect("tracked fixture source should live under the repo root");
    let output = Command::new("git")
        .current_dir(repo)
        .args(["ls-files", "--", relative_src.to_str().unwrap()])
        .output()
        .expect("failed to run git ls-files for tracked fixture copy");

    assert!(
        output.status.success(),
        "git ls-files failed for {}.\nstdout:\n{}\nstderr:\n{}",
        relative_src.display(),
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let tracked_files = String::from_utf8_lossy(&output.stdout);
    assert!(
        !tracked_files.trim().is_empty(),
        "git ls-files returned no tracked files under {}",
        relative_src.display()
    );

    fs::create_dir_all(dst)?;
    for tracked_path in tracked_files.lines().filter(|line| !line.is_empty()) {
        let tracked_path = Path::new(tracked_path);
        let suffix = tracked_path
            .strip_prefix(relative_src)
            .unwrap_or_else(|_| panic!("{tracked_path:?} was not nested under {relative_src:?}"));
        let destination = dst.join(suffix);
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::copy(repo.join(tracked_path), destination)?;
    }

    Ok(())
}

fn copy_git_tracked_dir(src: &Path, dst: &Path) -> io::Result<()> {
    copy_git_tracked_dir_from_repo(&repo_root(), src, dst)
}

fn setup_detached_shared_example() -> (tempfile::TempDir, PathBuf, PathBuf) {
    let root = repo_root();
    let temp_dir = temp_repo_dir();
    let shared_crate_dir = temp_dir.path().join("shared-crate");
    let shared_spec_dir = temp_dir.path().join("shared-spec");

    fs::write(
        temp_dir.path().join(".git"),
        "gitdir: .git/modules/spec-tests\n",
    )
    .unwrap();
    copy_dir_recursive(&root.join("examples/shared-crate"), &shared_crate_dir).unwrap();
    copy_dir_recursive(&root.join("examples/shared-spec"), &shared_spec_dir).unwrap();
    fs::write(
        shared_spec_dir.join("spec.toml"),
        "[pipeline]\ncrate_root = \"../shared-crate\"\n",
    )
    .unwrap();

    (temp_dir, shared_spec_dir, shared_crate_dir)
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

    let (_temp_dir, ecommerce_dir) = copy_ecommerce_example();
    let output = run_in(
        &ecommerce_dir,
        &[
            "build",
            "units",
            "--output",
            "src/generated",
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

    let (_temp_dir, ecommerce_dir) = copy_ecommerce_example();
    let output = run_in(
        &ecommerce_dir,
        &[
            "test",
            "units",
            "--output",
            "src/generated",
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

    let (_fixture, ecommerce_dir) = copy_ecommerce_example();
    let temp_dir = temp_repo_dir();
    let units_dir = temp_dir.path().join("units");
    write_minimal_units_dir(&units_dir);
    let generated_dir = ecommerce_dir.join("src/generated");

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
        generated_dir.to_str().unwrap(),
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
    let temp_dir = tempfile::TempDir::new().unwrap();
    let units_dir = temp_dir.path().join("units");
    write_minimal_units_dir(&units_dir);

    // No Cargo.toml anywhere under temp_dir, and the fixture lives outside this repo,
    // so the ancestor walk has no crate root to find.
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

    let (_temp_dir, ecommerce_dir) = copy_ecommerce_example();
    let output = run_in(
        &ecommerce_dir,
        &[
            "build",
            "units",
            "--output",
            "src/generated",
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

#[test]
fn spec_build_resolves_relative_pipeline_crate_root_from_spec_toml() {
    if !cargo_available() {
        return;
    }

    let (_temp_dir, shared_spec_dir, _shared_crate_dir) = setup_detached_shared_example();
    let output = run_in(
        shared_spec_dir
            .parent()
            .expect("shared-spec fixture should have a parent"),
        &[
            "build",
            "shared-spec/units",
            "--output",
            "shared-crate/src/generated",
        ],
    );

    assert_output_success(
        "spec build should resolve relative [pipeline].crate_root from spec.toml",
        &output,
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("spec: running cargo build in") && stderr.contains("shared-crate"),
        "{stderr}"
    );
}

#[test]
fn spec_build_from_crate_subdir_infers_crate_root_without_empty_workdir() {
    if !cargo_available() {
        return;
    }

    let (_temp_dir, ecommerce_dir) = copy_ecommerce_example();
    let output = run_in(&ecommerce_dir, &["build", "units"]);

    assert_output_success(
        "spec build from crate subdir should infer the local crate root",
        &output,
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("spec: running cargo build in"), "{stderr}");
    assert!(
        !stderr.contains("failed to spawn cargo"),
        "crate-root inference regressed: {stderr}"
    );
}

#[test]
fn spec_build_from_shared_spec_subdir_allows_repo_relative_output() {
    if !cargo_available() {
        return;
    }

    let (_temp_dir, shared_spec_dir, _shared_crate_dir) = setup_detached_shared_example();
    let output = run_in(&shared_spec_dir, &["build", "units"]);

    assert_output_success(
        "spec build from shared-spec subdir should honor repo-relative crate_root",
        &output,
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("spec: running cargo build in") && stderr.contains("shared-crate"),
        "{stderr}"
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

fn copy_ecommerce_example_preserving_artifacts() -> (tempfile::TempDir, PathBuf) {
    let root = repo_root();
    let temp_dir =
        tempfile::TempDir::new_in(root.join("target")).expect("failed to create temp dir");
    let dst_ecommerce = temp_dir.path().join("ecommerce");
    copy_git_tracked_dir(&root.join("examples/ecommerce"), &dst_ecommerce)
        .expect("failed to copy ecommerce example");
    fs::write(
        temp_dir.path().join(".git"),
        "gitdir: .git/modules/spec-tests\n",
    )
    .expect("failed to mark ecommerce temp repo root");
    (temp_dir, dst_ecommerce)
}

fn remove_derived_artifacts(root: &Path) {
    for entry in WalkDir::new(root) {
        let entry = entry.expect("failed to walk copied example");
        if entry.file_type().is_file() {
            let file_name = entry.file_name().to_string_lossy();
            if file_name.ends_with(".spec.passport.json")
                || file_name.ends_with(".test.evidence.json")
            {
                fs::remove_file(entry.path()).expect("failed to remove copied derived artifact");
            }
        }
    }
}

fn copy_ecommerce_example() -> (tempfile::TempDir, PathBuf) {
    let (temp_dir, dst_ecommerce) = copy_ecommerce_example_preserving_artifacts();
    remove_derived_artifacts(&dst_ecommerce);
    (temp_dir, dst_ecommerce)
}

fn copy_benchmark_repo_fixture_from_root(root: &Path) -> (tempfile::TempDir, PathBuf) {
    let temp_dir = temp_repo_dir();
    let repo_dir = temp_dir.path().join("benchmark-repo");
    let examples_dir = repo_dir.join("examples");
    let semantic_families_dir = repo_dir.join("semantic-families");
    let spec_cli_fixtures_dir = repo_dir.join("spec-cli/tests/fixtures");

    fs::create_dir_all(&examples_dir).unwrap();
    copy_git_tracked_dir_from_repo(
        root,
        &root.join("examples/ecommerce"),
        &examples_dir.join("ecommerce"),
    )
    .expect("failed to copy ecommerce benchmark fixture");
    copy_git_tracked_dir_from_repo(
        root,
        &root.join("examples/crosslib-app"),
        &examples_dir.join("crosslib-app"),
    )
    .expect("failed to copy cross-library benchmark fixture");
    copy_git_tracked_dir_from_repo(
        root,
        &root.join("examples/shared-crate"),
        &examples_dir.join("shared-crate"),
    )
    .expect("failed to copy shared crate benchmark fixture");
    copy_git_tracked_dir_from_repo(
        root,
        &root.join("examples/shared-spec"),
        &examples_dir.join("shared-spec"),
    )
    .expect("failed to copy shared spec benchmark fixture");
    copy_git_tracked_dir_from_repo(
        root,
        &root.join("examples/service"),
        &examples_dir.join("service"),
    )
    .expect("failed to copy service benchmark fixture");
    copy_dir_recursive(&root.join("benchmarks"), &repo_dir.join("benchmarks"))
        .expect("failed to copy benchmark registry fixture");
    copy_dir_recursive(
        &root.join(
            "semantic-families/function.arithmetic_leaf.monotone_down_nonnegative.v1/fixtures",
        ),
        &semantic_families_dir
            .join("function.arithmetic_leaf.monotone_down_nonnegative.v1/fixtures"),
    )
    .expect("failed to copy monotone-down benchmark semantic fixtures");
    copy_dir_recursive(
        &root.join("semantic-families/function.arithmetic_leaf.monotone_up.v1/fixtures"),
        &semantic_families_dir.join("function.arithmetic_leaf.monotone_up.v1/fixtures"),
    )
    .expect("failed to copy monotone-up benchmark semantic fixtures");
    copy_dir_recursive(
        &root.join("semantic-families/function.helper.identity_passthrough.v1/fixtures"),
        &semantic_families_dir.join("function.helper.identity_passthrough.v1/fixtures"),
    )
    .expect("failed to copy helper benchmark semantic fixtures");
    copy_dir_recursive(
        &root.join("semantic-families/function.wrapper.pipeline.chain3.v1/fixtures"),
        &semantic_families_dir.join("function.wrapper.pipeline.chain3.v1/fixtures"),
    )
    .expect("failed to copy chain3 benchmark semantic fixtures");
    copy_dir_recursive(
        &root.join(
            "semantic-families/function.wrapper.pipeline.normalized_required_arg.v1/fixtures",
        ),
        &semantic_families_dir
            .join("function.wrapper.pipeline.normalized_required_arg.v1/fixtures"),
    )
    .expect("failed to copy normalized-required-arg benchmark semantic fixtures");
    copy_dir_recursive(
        &root.join("semantic-families/function.wrapper.pipeline.v1/fixtures"),
        &semantic_families_dir.join("function.wrapper.pipeline.v1/fixtures"),
    )
    .expect("failed to copy wrapper benchmark semantic fixtures");
    copy_dir_recursive(
        &root.join("spec-cli/tests/fixtures/m19/semantic_falsification_pack"),
        &spec_cli_fixtures_dir.join("m19/semantic_falsification_pack"),
    )
    .expect("failed to copy M19 benchmark semantic fixture pack");
    copy_dir_recursive(
        &root.join("spec-cli/tests/fixtures/m20/unsupported_truth_pack"),
        &spec_cli_fixtures_dir.join("m20/unsupported_truth_pack"),
    )
    .expect("failed to copy M20 benchmark semantic fixture pack");
    copy_dir_recursive(
        &root.join("spec-cli/tests/fixtures/typescript_local_supported_graph"),
        &spec_cli_fixtures_dir.join("typescript_local_supported_graph"),
    )
    .expect("failed to copy TypeScript supported-graph benchmark fixture pack");
    init_git_repo(&repo_dir);

    (temp_dir, repo_dir)
}

fn copy_benchmark_repo_fixture() -> (tempfile::TempDir, PathBuf) {
    copy_benchmark_repo_fixture_from_root(&repo_root())
}

fn setup_benchmark_source_repo_fixture() -> (tempfile::TempDir, PathBuf) {
    let root = repo_root();
    let temp_dir = temp_repo_dir();
    let source_root = temp_dir.path().join("source-repo");
    let examples_dir = source_root.join("examples");
    let semantic_families_dir = source_root.join("semantic-families");
    let spec_cli_fixtures_dir = source_root.join("spec-cli/tests/fixtures");

    fs::create_dir_all(&examples_dir).unwrap();
    copy_git_tracked_dir(
        &root.join("examples/ecommerce"),
        &examples_dir.join("ecommerce"),
    )
    .expect("failed to copy ecommerce benchmark source fixture");
    copy_git_tracked_dir(
        &root.join("examples/crosslib-app"),
        &examples_dir.join("crosslib-app"),
    )
    .expect("failed to copy cross-library benchmark source fixture");
    copy_git_tracked_dir(
        &root.join("examples/shared-crate"),
        &examples_dir.join("shared-crate"),
    )
    .expect("failed to copy shared crate benchmark source fixture");
    copy_git_tracked_dir(
        &root.join("examples/shared-spec"),
        &examples_dir.join("shared-spec"),
    )
    .expect("failed to copy shared spec benchmark source fixture");
    copy_git_tracked_dir(
        &root.join("examples/service"),
        &examples_dir.join("service"),
    )
    .expect("failed to copy service benchmark source fixture");
    copy_git_tracked_dir(&root.join("benchmarks"), &source_root.join("benchmarks"))
        .expect("failed to copy benchmark registry source fixture");
    copy_git_tracked_dir(
        &root.join(
            "semantic-families/function.arithmetic_leaf.monotone_down_nonnegative.v1/fixtures",
        ),
        &semantic_families_dir
            .join("function.arithmetic_leaf.monotone_down_nonnegative.v1/fixtures"),
    )
    .expect("failed to copy monotone-down semantic source fixtures");
    copy_git_tracked_dir(
        &root.join("semantic-families/function.arithmetic_leaf.monotone_up.v1/fixtures"),
        &semantic_families_dir.join("function.arithmetic_leaf.monotone_up.v1/fixtures"),
    )
    .expect("failed to copy monotone-up semantic source fixtures");
    copy_git_tracked_dir(
        &root.join("semantic-families/function.helper.identity_passthrough.v1/fixtures"),
        &semantic_families_dir.join("function.helper.identity_passthrough.v1/fixtures"),
    )
    .expect("failed to copy helper semantic source fixtures");
    copy_git_tracked_dir(
        &root.join("semantic-families/function.wrapper.pipeline.chain3.v1/fixtures"),
        &semantic_families_dir.join("function.wrapper.pipeline.chain3.v1/fixtures"),
    )
    .expect("failed to copy chain3 semantic source fixtures");
    copy_git_tracked_dir(
        &root.join(
            "semantic-families/function.wrapper.pipeline.normalized_required_arg.v1/fixtures",
        ),
        &semantic_families_dir
            .join("function.wrapper.pipeline.normalized_required_arg.v1/fixtures"),
    )
    .expect("failed to copy normalized-required-arg semantic source fixtures");
    copy_git_tracked_dir(
        &root.join("semantic-families/function.wrapper.pipeline.v1/fixtures"),
        &semantic_families_dir.join("function.wrapper.pipeline.v1/fixtures"),
    )
    .expect("failed to copy wrapper semantic source fixtures");
    copy_git_tracked_dir(
        &root.join("spec-cli/tests/fixtures/m19/semantic_falsification_pack"),
        &spec_cli_fixtures_dir.join("m19/semantic_falsification_pack"),
    )
    .expect("failed to copy M19 source fixture pack");
    copy_git_tracked_dir(
        &root.join("spec-cli/tests/fixtures/m20/unsupported_truth_pack"),
        &spec_cli_fixtures_dir.join("m20/unsupported_truth_pack"),
    )
    .expect("failed to copy M20 source fixture pack");
    copy_git_tracked_dir(
        &root.join("spec-cli/tests/fixtures/typescript_local_supported_graph"),
        &spec_cli_fixtures_dir.join("typescript_local_supported_graph"),
    )
    .expect("failed to copy TypeScript source fixture pack");
    init_git_repo(&source_root);

    (temp_dir, source_root)
}

fn copy_crosslib_typescript_example() -> (tempfile::TempDir, PathBuf, PathBuf, PathBuf) {
    let root = repo_root();
    let temp_dir = temp_repo_dir();
    let app_dir = temp_dir.path().join("crosslib-app");
    let shared_crate_dir = temp_dir.path().join("shared-crate");
    let shared_spec_dir = temp_dir.path().join("shared-spec");

    fs::write(
        temp_dir.path().join(".git"),
        "gitdir: .git/modules/spec-tests\n",
    )
    .unwrap();
    copy_dir_recursive(&root.join("examples/crosslib-app"), &app_dir).unwrap();
    copy_dir_recursive(&root.join("examples/shared-crate"), &shared_crate_dir).unwrap();
    copy_dir_recursive(&root.join("examples/shared-spec"), &shared_spec_dir).unwrap();
    remove_derived_artifacts(&app_dir);
    remove_derived_artifacts(&shared_spec_dir);

    (temp_dir, app_dir, shared_crate_dir, shared_spec_dir)
}

fn write_cross_library_chain3_spec(app_dir: &Path) -> PathBuf {
    write_spec(
        &app_dir.join("units"),
        "pricing/checkout_chain3.unit.spec",
        r#"
id: pricing/checkout_chain3
kind: function
spec_version: "0.3.0"
intent:
  why: Return the final checkout total by computing the taxed discounted subtotal, then applying a surcharge, then applying a loyalty discount.
contract:
  inputs:
    subtotal: Decimal
    discount_rate: Decimal
    tax_rate: Decimal
    surcharge_rate: Decimal
    loyalty_rate: Decimal
  returns: Decimal
deps:
  - pricing/calculate_total
  - shared::pricing/apply_tax
  - shared::pricing/apply_discount
imports:
  - rust_decimal::Decimal
body:
  rust: |
    {
        let base_total = calculate_total(subtotal, discount_rate, tax_rate);
        let surcharged_total = apply_tax(base_total, surcharge_rate);
        apply_discount(surcharged_total, loyalty_rate)
    }
  typescript: |
    {
        const base_total = calculate_total(subtotal, discount_rate, tax_rate);
        const surcharged_total = apply_tax(base_total, surcharge_rate);
        return apply_discount(surcharged_total, loyalty_rate);
    }
local_tests:
  - id: happy_path
    expect: checkout_chain3(Decimal::new(1000, 2), Decimal::new(10, 2), Decimal::new(7, 2), Decimal::new(5, 2), Decimal::new(5, 2)) == Decimal::new(96045, 4)
"#,
    );

    app_dir.join("units/pricing/checkout_chain3.unit.spec")
}

fn write_cross_library_nested_chain3_specs(app_dir: &Path, shared_spec_dir: &Path) -> PathBuf {
    write_spec(
        &shared_spec_dir.join("units"),
        "pricing/calculate_total.unit.spec",
        r#"
id: pricing/calculate_total
kind: function
spec_version: "0.3.0"
intent:
  why: Return the shared checkout total after discounting the subtotal and then applying tax.
contract:
  inputs:
    subtotal: Decimal
    discount_rate: Decimal
    tax_rate: Decimal
  returns: Decimal
deps:
  - pricing/apply_discount
  - pricing/apply_tax
imports:
  - rust_decimal::Decimal
body:
  rust: |
    {
        let discounted = apply_discount(subtotal, discount_rate);
        apply_tax(discounted, tax_rate)
    }
  typescript: |
    {
        const discounted = apply_discount(subtotal, discount_rate);
        return apply_tax(discounted, tax_rate);
    }
local_tests:
  - id: happy_path
    expect: calculate_total(Decimal::new(10000, 2), Decimal::new(10, 2), Decimal::new(725, 4)) == Decimal::new(9653, 2)
"#,
    );
    write_spec(
        &shared_spec_dir.join("units"),
        "pricing/base_nested_chain3.unit.spec",
        r#"
id: pricing/base_nested_chain3
kind: function
spec_version: "0.3.0"
intent:
  why: Return the shared nested chain3 subtotal before the app root applies its outer surcharge and loyalty discount.
contract:
  inputs:
    subtotal: Decimal
    discount_rate: Decimal
    tax_rate: Decimal
    surcharge_rate: Decimal
    loyalty_rate: Decimal
  returns: Decimal
deps:
  - pricing/calculate_total
  - pricing/apply_tax
  - pricing/apply_discount
imports:
  - rust_decimal::Decimal
body:
  rust: |
    {
        let base_total = calculate_total(subtotal, discount_rate, tax_rate);
        let surcharged_total = apply_tax(base_total, surcharge_rate);
        apply_discount(surcharged_total, loyalty_rate)
    }
  typescript: |
    {
        const base_total = calculate_total(subtotal, discount_rate, tax_rate);
        const surcharged_total = apply_tax(base_total, surcharge_rate);
        return apply_discount(surcharged_total, loyalty_rate);
    }
local_tests:
  - id: happy_path
    expect: base_nested_chain3(Decimal::new(1000, 2), Decimal::new(10, 2), Decimal::new(7, 2), Decimal::new(5, 2), Decimal::new(5, 2)) == Decimal::new(96045, 4)
"#,
    );
    write_spec(
        &app_dir.join("units"),
        "pricing/checkout_nested_chain3.unit.spec",
        r#"
id: pricing/checkout_nested_chain3
kind: function
spec_version: "0.3.0"
intent:
  why: Return the app checkout total by extending the shared nested chain3 with one outer surcharge and loyalty discount.
contract:
  inputs:
    subtotal: Decimal
    discount_rate: Decimal
    tax_rate: Decimal
    surcharge_rate: Decimal
    loyalty_rate: Decimal
  returns: Decimal
deps:
  - shared::pricing/base_nested_chain3
  - shared::pricing/apply_tax
  - shared::pricing/apply_discount
imports:
  - rust_decimal::Decimal
body:
  rust: |
    {
        let base_total = base_nested_chain3(subtotal, discount_rate, tax_rate, surcharge_rate, loyalty_rate);
        let surcharged_total = apply_tax(base_total, surcharge_rate);
        apply_discount(surcharged_total, loyalty_rate)
    }
  typescript: |
    {
        const base_total = base_nested_chain3(subtotal, discount_rate, tax_rate, surcharge_rate, loyalty_rate);
        const surcharged_total = apply_tax(base_total, surcharge_rate);
        return apply_discount(surcharged_total, loyalty_rate);
    }
local_tests:
  - id: happy_path
    expect: checkout_nested_chain3(Decimal::new(1000, 2), Decimal::new(10, 2), Decimal::new(7, 2), Decimal::new(5, 2), Decimal::new(5, 2)) == Decimal::new(95760, 4)
"#,
    );

    app_dir.join("units/pricing/checkout_nested_chain3.unit.spec")
}

fn read_passport(passport_path: &Path) -> String {
    fs::read_to_string(passport_path).unwrap()
}

fn read_passport_json(passport_path: &Path) -> Value {
    serde_json::from_str(&read_passport(passport_path)).unwrap()
}

fn seed_passport_semantic_review_compatibility_key(
    passport_path: &Path,
    compatibility_key: &str,
) -> Value {
    let mut passport = read_passport_json(passport_path);
    passport["semantic_review"]["compatibility_key"] = serde_json::json!(compatibility_key);
    if compatibility_key == DATA_SEAM_COMPATIBILITY_KEY {
        passport["semantic_review"]["descriptor_id"] =
            serde_json::json!("pricing_quote.ecommerce.v1");
    } else if compatibility_key == SUM_SEAM_COMPATIBILITY_KEY {
        passport["semantic_review"]["descriptor_id"] =
            serde_json::json!("discount_strategy.ecommerce.v1");
    }
    fs::write(
        passport_path,
        serde_json::to_string_pretty(&passport).unwrap(),
    )
    .unwrap();
    read_passport_json(passport_path)["semantic_review"].clone()
}

fn write_bounded_typescript_apply_tax_fixture(project_dir: &Path) -> PathBuf {
    let units_dir = project_dir.join("units");
    let spec_path = units_dir.join("pricing/apply_tax.unit.spec");
    write_spec(
        &units_dir,
        "pricing/apply_tax.unit.spec",
        r#"
id: pricing/apply_tax
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
imports:
  - rust_decimal::Decimal
body:
  rust: |
    {
        subtotal + subtotal * rate
    }
  typescript: |
    {
        return subtotal.add(subtotal.mul(rate));
    }
local_tests:
  - id: basic_tax
    expect: apply_tax(Decimal::new(10000, 2), Decimal::new(725, 4)) == Decimal::new(10725, 2)
"#,
    );
    spec_path
}

fn write_typescript_near_miss_fixture(project_dir: &Path) -> PathBuf {
    let units_dir = project_dir.join("units");
    let spec_path = units_dir.join("pricing/apply_discount.unit.spec");
    write_spec(
        &units_dir,
        "pricing/apply_discount.unit.spec",
        r#"
id: pricing/apply_discount
kind: function
spec_version: "0.3.0"
intent:
  why: Apply a discount to a subtotal while keeping the result nonnegative.
contract:
  inputs:
    subtotal: Decimal
    rate: Decimal
  returns: Decimal
  invariants:
    - output <= subtotal
    - output >= 0
imports:
  - rust_decimal::Decimal
body:
  rust: |
    {
        (subtotal - subtotal * rate).max(Decimal::ZERO)
    }
  typescript: |
    {
        return subtotal.add(subtotal.mul(Decimal.new(-1n, 0n).mul(rate)));
    }
local_tests:
  - id: happy_path
    expect: apply_discount(Decimal::new(10000, 2), Decimal::new(10, 2)) == Decimal::new(9000, 2)
"#,
    );
    spec_path
}

#[cfg(unix)]
fn path_with_fake_bun(project_dir: &Path, script_body: &str) -> std::ffi::OsString {
    let fake_bin_dir = project_dir.join("fake-bin");
    write_executable_file(&fake_bin_dir, "bun", script_body);
    let mut path_override = std::ffi::OsString::from(fake_bin_dir.as_os_str());
    path_override.push(":");
    path_override.push(std::env::var_os("PATH").unwrap_or_default());
    path_override
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

fn write_single_file_test_scope_project(project_dir: &Path) -> PathBuf {
    let units_dir = project_dir.join("units");
    let pricing_dir = units_dir.join("pricing");
    let src_dir = project_dir.join("src");

    fs::create_dir_all(&pricing_dir).unwrap();
    fs::create_dir_all(&src_dir).unwrap();

    fs::write(
        project_dir.join("Cargo.toml"),
        "[package]\nname = \"single-file-test-scope\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[workspace]\n",
    )
    .unwrap();
    fs::write(
        src_dir.join("main.rs"),
        "mod generated;\npub use generated::*;\nfn main() {}\n",
    )
    .unwrap();

    write_spec(
        &units_dir,
        "pricing/a.unit.spec",
        r#"spec_version: "0.3.0"
id: pricing/a
kind: function
intent:
  why: Return true so the single-file test path has one local test to run.
contract:
  returns: bool
body:
  rust: |
    {
        true
    }
local_tests:
  - id: happy_path
    expect: a() == true
"#,
    );
    write_spec(
        &units_dir,
        "pricing/bad.test.spec",
        r#"spec_version: "0.3.0"
id: pricing/bad
intent:
  why: Invalid molecule test used to prove single-file spec test stays scoped.
covers:
  - pricing/missing
body:
  rust: |
    {
        assert!(true);
    }
"#,
    );

    pricing_dir.join("a.unit.spec")
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
        project_dir.join("units/pricing/quote_total.unit.spec"),
        r#"
id: pricing/quote_total
kind: function
intent:
  why: Return a quoted total placeholder.
spec_version: "0.3.0"
contract:
  returns: i32
body:
  rust: |
    { 1 }
local_tests:
  - id: happy_path
    expect: quote_total() == 1
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
        project_dir.join("units/pricing/quote_total.unit.spec"),
        r#"
id: pricing/quote_total
kind: function
intent:
  why: Return a quoted total placeholder.
spec_version: "0.3.0"
body:
  rust: |
    { }
"#,
    )
    .unwrap();

    write_file(
        project_dir,
        "units/pricing/quote_total.spec.passport.json",
        r#"{
  "spec_version": "0.3.0",
  "id": "pricing/quote_total",
  "intent": "Return a quoted total placeholder.",
  "deps": [],
  "local_tests": [],
  "generated_at": "2024-01-02T03:04:05Z",
  "source_file": "pricing/quote_total.unit.spec",
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
        project_dir.join("units/pricing/quote_total.unit.spec"),
        r#"
id: pricing/quote_total
kind: function
intent:
  why: Return a quoted total placeholder.
spec_version: "0.3.0"
contract:
  returns: bool
body:
  rust: |
    { true }
local_tests:
  - id: happy_path
    expect: quote_total() == true
"#,
    )
    .unwrap();

    let output = run_in(project_dir, &["status", "units", "--format", "json"]);
    assert!(
        !output.status.success(),
        "contract addition should mark unit stale"
    );
    let json = parse_stdout_json(&output);
    let units = json["units"].as_array().unwrap();
    assert_eq!(units[0]["status"], "stale");
    assert_eq!(units[0]["reason"], "authored truth changed since last test");
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
fn spec_status_checked_in_ecommerce_example_is_green() {
    let (_temp_dir, ecommerce_dir) = copy_ecommerce_example_preserving_artifacts();
    let output = run_in(&ecommerce_dir, &["status", ".", "--format", "json"]);
    assert_output_success("checked-in ecommerce example should be green", &output);

    let json = parse_stdout_json(&output);
    assert!(
        status_units(&json)
            .iter()
            .all(|unit| unit["status"] == "valid"),
        "{json}"
    );
    assert_eq!(status_molecule_tests(&json).len(), 3, "{json}");
    assert!(
        status_molecule_tests(&json)
            .iter()
            .all(|test| test["status"] == "valid"),
        "{json}"
    );
}

#[test]
fn spec_status_checked_in_ecommerce_example_opens_marked_seam_gates_without_molecule_evidence() {
    let (_temp_dir, ecommerce_dir) = copy_ecommerce_example_preserving_artifacts();
    fs::remove_file(ecommerce_dir.join("units/pricing/checkout_flow.test.evidence.json")).unwrap();
    fs::remove_file(ecommerce_dir.join("units/pricing/discount_plus_tax.test.evidence.json"))
        .unwrap();
    fs::remove_file(
        ecommerce_dir.join("units/pricing/discount_strategy_checkout_flow.test.evidence.json"),
    )
    .unwrap();

    let output = run_in(&ecommerce_dir, &["status", ".", "--format", "json"]);
    assert!(
        !output.status.success(),
        "removing checked-in molecule evidence should make the example non-green"
    );

    let json = parse_stdout_json(&output);
    let pricing_quote = status_units(&json)
        .iter()
        .find(|unit| unit["id"] == "pricing/pricing_quote")
        .unwrap();
    assert_eq!(pricing_quote["status"], "incomplete", "{json}");
    assert_eq!(
        pricing_quote["reason"], "missing required escape-hatch proof: molecule",
        "{json}"
    );
    let discount_strategy = status_units(&json)
        .iter()
        .find(|unit| unit["id"] == "pricing/discount_strategy")
        .unwrap();
    assert_eq!(discount_strategy["status"], "incomplete", "{json}");
    assert_eq!(
        discount_strategy["reason"], "missing required escape-hatch proof: molecule",
        "{json}"
    );
    assert!(
        status_units(&json)
            .iter()
            .filter(|unit| unit["status"] == "valid")
            .count()
            >= 4,
        "{json}"
    );
    assert_eq!(status_molecule_tests(&json).len(), 3, "{json}");
    assert!(
        status_molecule_tests(&json)
            .iter()
            .all(|test| test["status"] == "untested"),
        "{json}"
    );
}

#[test]
fn spec_status_and_export_ignore_stale_checked_in_marked_seam_gate_claims() {
    let (_temp_dir, ecommerce_dir) = copy_ecommerce_example_preserving_artifacts();
    let passport_path = ecommerce_dir.join("units/pricing/discount_strategy.spec.passport.json");
    let mut seeded_passport = read_passport_json(&passport_path);
    seeded_passport["escape_hatch_gate"] = serde_json::json!({
        "status": "open",
        "required_surfaces": ["atom", "molecule"],
        "present_surfaces": ["atom"],
        "missing_surfaces": ["molecule"],
        "reason": "missing required escape-hatch proof: molecule"
    });
    fs::write(
        &passport_path,
        serde_json::to_string_pretty(&seeded_passport).unwrap(),
    )
    .unwrap();

    let status_output = run_in(&ecommerce_dir, &["status", ".", "--format", "json"]);
    assert_output_success(
        "status should ignore stale checked-in gate claims and stay green",
        &status_output,
    );
    let status_json = parse_stdout_json(&status_output);
    let status_unit = status_units(&status_json)
        .iter()
        .find(|unit| unit["id"] == "pricing/discount_strategy")
        .unwrap();
    assert_eq!(status_unit["status"], "valid", "{status_json}");
    assert_eq!(
        status_unit["escape_hatch_gate"]["status"], "closed",
        "{status_json}"
    );

    let export_output = run_in(&ecommerce_dir, &["export", ".", "--format", "json"]);
    assert_output_success(
        "export should ignore stale checked-in gate claims",
        &export_output,
    );
    let export_json = parse_stdout_json(&export_output);
    let exported_passport = export_json["passports"]
        .as_array()
        .unwrap()
        .iter()
        .find(|passport| passport["id"] == "pricing/discount_strategy")
        .unwrap();

    assert_eq!(
        exported_passport["escape_hatch_gate"],
        status_unit["escape_hatch_gate"]
    );
    assert_eq!(exported_passport["escape_hatch_gate"]["status"], "closed");
}

#[test]
fn spec_status_and_export_reproject_marked_seam_markers_and_proof_coverage_from_current_truth() {
    let (_temp_dir, ecommerce_dir) = copy_ecommerce_example_preserving_artifacts();
    let passport_path = ecommerce_dir.join("units/pricing/discount_strategy.spec.passport.json");
    let mut seeded_passport = read_passport_json(&passport_path);
    seeded_passport["markers"] = serde_json::json!([
        {
            "id": "backend_rust_derives",
            "path": "backends.rust.derives"
        }
    ]);
    seeded_passport["proof_coverage"] = serde_json::json!([
        {
            "id": "variant.none",
            "surfaces": ["implicit_only"]
        }
    ]);
    fs::write(
        &passport_path,
        serde_json::to_string_pretty(&seeded_passport).unwrap(),
    )
    .unwrap();

    let status_output = run_in(&ecommerce_dir, &["status", ".", "--format", "json"]);
    assert_output_success(
        "status should reproject current marked-seam markers and proof coverage",
        &status_output,
    );
    let status_json = parse_stdout_json(&status_output);
    let status_unit = status_units(&status_json)
        .iter()
        .find(|unit| unit["id"] == "pricing/discount_strategy")
        .unwrap();

    assert_ne!(
        status_unit["markers"], seeded_passport["markers"],
        "{status_json}"
    );
    assert_ne!(
        status_unit["proof_coverage"], seeded_passport["proof_coverage"],
        "{status_json}"
    );

    let export_output = run_in(&ecommerce_dir, &["export", ".", "--format", "json"]);
    assert_output_success(
        "export should reproject current marked-seam markers and proof coverage",
        &export_output,
    );
    let export_json = parse_stdout_json(&export_output);
    let exported_passport = export_json["passports"]
        .as_array()
        .unwrap()
        .iter()
        .find(|passport| passport["id"] == "pricing/discount_strategy")
        .unwrap();

    assert_eq!(status_unit["markers"], exported_passport["markers"]);
    assert_eq!(
        status_unit["proof_coverage"],
        exported_passport["proof_coverage"]
    );
    assert_eq!(
        status_unit["escape_hatch_gate"],
        exported_passport["escape_hatch_gate"]
    );
}

#[test]
fn seam_cli_status_and_export_preserve_legacy_semantic_reviews() {
    let (_temp_dir, ecommerce_dir) = copy_ecommerce_example_preserving_artifacts();
    let data_passport_path = ecommerce_dir.join("units/pricing/pricing_quote.spec.passport.json");
    let sum_passport_path =
        ecommerce_dir.join("units/pricing/discount_strategy.spec.passport.json");
    let seeded_data_review = seed_passport_semantic_review_compatibility_key(
        &data_passport_path,
        DATA_SEAM_COMPATIBILITY_KEY,
    );
    let seeded_sum_review = seed_passport_semantic_review_compatibility_key(
        &sum_passport_path,
        SUM_SEAM_COMPATIBILITY_KEY,
    );

    let status_output = run_in(&ecommerce_dir, &["status", ".", "--format", "json"]);
    assert_output_success(
        "status should preserve canonical seam semantic reviews from checked-in passports",
        &status_output,
    );
    let status_json = parse_stdout_json(&status_output);
    let pricing_quote = status_units(&status_json)
        .iter()
        .find(|unit| unit["id"] == "pricing/pricing_quote")
        .unwrap();
    assert_eq!(pricing_quote["status"], "valid", "{status_json}");
    assert_eq!(
        pricing_quote["semantic_review"], seeded_data_review,
        "{status_json}"
    );
    let discount_strategy = status_units(&status_json)
        .iter()
        .find(|unit| unit["id"] == "pricing/discount_strategy")
        .unwrap();
    assert_eq!(discount_strategy["status"], "valid", "{status_json}");
    assert_eq!(
        discount_strategy["semantic_review"], seeded_sum_review,
        "{status_json}"
    );

    let export_output = run_in(&ecommerce_dir, &["export", ".", "--format", "json"]);
    assert_output_success(
        "export should preserve canonical seam semantic reviews from checked-in passports",
        &export_output,
    );
    let export_json = parse_stdout_json(&export_output);
    let exported_pricing_quote = export_json["passports"]
        .as_array()
        .unwrap()
        .iter()
        .find(|passport| passport["id"] == "pricing/pricing_quote")
        .unwrap();
    assert_eq!(
        exported_pricing_quote["semantic_review"], seeded_data_review,
        "{export_json}"
    );
    let exported_discount_strategy = export_json["passports"]
        .as_array()
        .unwrap()
        .iter()
        .find(|passport| passport["id"] == "pricing/discount_strategy")
        .unwrap();
    assert_eq!(
        exported_discount_strategy["semantic_review"], seeded_sum_review,
        "{export_json}"
    );
}

#[test]
fn seam_cli_legacy_semantic_reviews_preserve_incomplete_health_semantics() {
    let (_temp_dir, ecommerce_dir) = copy_ecommerce_example_preserving_artifacts();
    let data_passport_path = ecommerce_dir.join("units/pricing/pricing_quote.spec.passport.json");
    let sum_passport_path =
        ecommerce_dir.join("units/pricing/discount_strategy.spec.passport.json");
    let seeded_data_review = seed_passport_semantic_review_compatibility_key(
        &data_passport_path,
        DATA_SEAM_COMPATIBILITY_KEY,
    );
    let seeded_sum_review = seed_passport_semantic_review_compatibility_key(
        &sum_passport_path,
        SUM_SEAM_COMPATIBILITY_KEY,
    );

    fs::remove_file(ecommerce_dir.join("units/pricing/checkout_flow.test.evidence.json")).unwrap();
    fs::remove_file(ecommerce_dir.join("units/pricing/discount_plus_tax.test.evidence.json"))
        .unwrap();
    fs::remove_file(
        ecommerce_dir.join("units/pricing/discount_strategy_checkout_flow.test.evidence.json"),
    )
    .unwrap();

    let status_output = run_in(&ecommerce_dir, &["status", ".", "--format", "json"]);
    assert!(
        !status_output.status.success(),
        "open marked-seam gates should keep status non-green even when canonical reviews are preserved"
    );
    let status_json = parse_stdout_json(&status_output);
    let pricing_quote = status_units(&status_json)
        .iter()
        .find(|unit| unit["id"] == "pricing/pricing_quote")
        .unwrap();
    assert_eq!(pricing_quote["status"], "incomplete", "{status_json}");
    assert_eq!(
        pricing_quote["reason"], "missing required escape-hatch proof: molecule",
        "{status_json}"
    );
    assert_eq!(
        pricing_quote["semantic_review"], seeded_data_review,
        "{status_json}"
    );
    let discount_strategy = status_units(&status_json)
        .iter()
        .find(|unit| unit["id"] == "pricing/discount_strategy")
        .unwrap();
    assert_eq!(discount_strategy["status"], "incomplete", "{status_json}");
    assert_eq!(
        discount_strategy["reason"], "missing required escape-hatch proof: molecule",
        "{status_json}"
    );
    assert_eq!(
        discount_strategy["semantic_review"], seeded_sum_review,
        "{status_json}"
    );

    let export_output = run_in(&ecommerce_dir, &["export", ".", "--format", "json"]);
    assert_output_success(
        "export should preserve canonical seam reviews while marked-seam gates are open",
        &export_output,
    );
    let export_json = parse_stdout_json(&export_output);
    let exported_pricing_quote = export_json["passports"]
        .as_array()
        .unwrap()
        .iter()
        .find(|passport| passport["id"] == "pricing/pricing_quote")
        .unwrap();
    assert_eq!(
        exported_pricing_quote["semantic_review"], seeded_data_review,
        "{export_json}"
    );
    let exported_discount_strategy = export_json["passports"]
        .as_array()
        .unwrap()
        .iter()
        .find(|passport| passport["id"] == "pricing/discount_strategy")
        .unwrap();
    assert_eq!(
        exported_discount_strategy["semantic_review"], seeded_sum_review,
        "{export_json}"
    );
}

#[test]
fn seam_cli_legacy_semantic_reviews_preserve_stale_health_semantics() {
    let (_temp_dir, ecommerce_dir) = copy_ecommerce_example_preserving_artifacts();
    let data_passport_path = ecommerce_dir.join("units/pricing/pricing_quote.spec.passport.json");
    let sum_passport_path =
        ecommerce_dir.join("units/pricing/discount_strategy.spec.passport.json");
    let seeded_data_review = seed_passport_semantic_review_compatibility_key(
        &data_passport_path,
        DATA_SEAM_COMPATIBILITY_KEY,
    );
    let seeded_sum_review = seed_passport_semantic_review_compatibility_key(
        &sum_passport_path,
        SUM_SEAM_COMPATIBILITY_KEY,
    );

    let pricing_quote_spec_path = ecommerce_dir.join("units/pricing/pricing_quote.unit.spec");
    let pricing_quote_source = fs::read_to_string(&pricing_quote_spec_path).unwrap();
    fs::write(
        &pricing_quote_spec_path,
        pricing_quote_source.replace(
            "Quote a checkout total from subtotal plus discount and tax rates.",
            "Quote a checkout total from subtotal plus discount and tax rates with revised authored intent.",
        ),
    )
    .unwrap();
    let discount_strategy_spec_path =
        ecommerce_dir.join("units/pricing/discount_strategy.unit.spec");
    let discount_strategy_source = fs::read_to_string(&discount_strategy_spec_path).unwrap();
    fs::write(
        &discount_strategy_spec_path,
        discount_strategy_source.replace(
            "Represent mutually exclusive discount strategies for checkout pricing.",
            "Represent mutually exclusive discount strategies for checkout pricing with revised authored intent.",
        ),
    )
    .unwrap();

    let status_output = run_in(&ecommerce_dir, &["status", ".", "--format", "json"]);
    assert!(
        !status_output.status.success(),
        "stale authored truth should keep status non-green even when canonical reviews are preserved"
    );
    let status_json = parse_stdout_json(&status_output);
    let pricing_quote = status_units(&status_json)
        .iter()
        .find(|unit| unit["id"] == "pricing/pricing_quote")
        .unwrap();
    assert_eq!(pricing_quote["status"], "stale", "{status_json}");
    assert_eq!(
        pricing_quote["reason"], "authored truth changed since last test",
        "{status_json}"
    );
    assert_eq!(
        pricing_quote["semantic_review"], seeded_data_review,
        "{status_json}"
    );
    let discount_strategy = status_units(&status_json)
        .iter()
        .find(|unit| unit["id"] == "pricing/discount_strategy")
        .unwrap();
    assert_eq!(discount_strategy["status"], "stale", "{status_json}");
    assert_eq!(
        discount_strategy["reason"], "authored truth changed since last test",
        "{status_json}"
    );
    assert_eq!(
        discount_strategy["semantic_review"], seeded_sum_review,
        "{status_json}"
    );

    let export_output = run_in(&ecommerce_dir, &["export", ".", "--format", "json"]);
    assert_output_success(
        "export should preserve canonical seam reviews when authored seam truth is stale",
        &export_output,
    );
    let export_json = parse_stdout_json(&export_output);
    let exported_pricing_quote = export_json["passports"]
        .as_array()
        .unwrap()
        .iter()
        .find(|passport| passport["id"] == "pricing/pricing_quote")
        .unwrap();
    assert_eq!(
        exported_pricing_quote["freshness"]["authored_truth_status"],
        "stale"
    );
    assert_eq!(
        exported_pricing_quote["semantic_review"], seeded_data_review,
        "{export_json}"
    );
    let exported_discount_strategy = export_json["passports"]
        .as_array()
        .unwrap()
        .iter()
        .find(|passport| passport["id"] == "pricing/discount_strategy")
        .unwrap();
    assert_eq!(
        exported_discount_strategy["freshness"]["authored_truth_status"],
        "stale"
    );
    assert_eq!(
        exported_discount_strategy["semantic_review"], seeded_sum_review,
        "{export_json}"
    );
}

#[test]
fn seam_cli_test_refresh_rewrites_legacy_semantic_review_keys_canonically() {
    if !cargo_available() {
        return;
    }

    let (_temp_dir, ecommerce_dir) = copy_ecommerce_example_preserving_artifacts();
    let data_passport_path = ecommerce_dir.join("units/pricing/pricing_quote.spec.passport.json");
    let sum_passport_path =
        ecommerce_dir.join("units/pricing/discount_strategy.spec.passport.json");
    let seeded_data_review = seed_passport_semantic_review_compatibility_key(
        &data_passport_path,
        DATA_SEAM_LEGACY_COMPATIBILITY_KEY,
    );
    let seeded_sum_review = seed_passport_semantic_review_compatibility_key(
        &sum_passport_path,
        SUM_SEAM_LEGACY_COMPATIBILITY_KEY,
    );

    let data_test_output = run_in(
        &ecommerce_dir,
        &[
            "test",
            "units/pricing/pricing_quote.unit.spec",
            "--crate-root",
            ".",
        ],
    );
    assert_output_success(
        "single-file data seam test should rewrite the legacy compatibility key canonically",
        &data_test_output,
    );
    let sum_test_output = run_in(
        &ecommerce_dir,
        &[
            "test",
            "units/pricing/discount_strategy.unit.spec",
            "--crate-root",
            ".",
        ],
    );
    assert_output_success(
        "single-file sum seam test should rewrite the legacy compatibility key canonically",
        &sum_test_output,
    );

    let refreshed_data_review = read_passport_json(&data_passport_path)["semantic_review"].clone();
    assert_ne!(refreshed_data_review, seeded_data_review);
    assert_eq!(
        refreshed_data_review["compatibility_key"],
        DATA_SEAM_COMPATIBILITY_KEY
    );
    let refreshed_sum_review = read_passport_json(&sum_passport_path)["semantic_review"].clone();
    assert_ne!(refreshed_sum_review, seeded_sum_review);
    assert_eq!(
        refreshed_sum_review["compatibility_key"],
        SUM_SEAM_COMPATIBILITY_KEY
    );

    let status_output = run_in(&ecommerce_dir, &["status", ".", "--format", "json"]);
    assert_output_success(
        "status should surface canonical seam semantic reviews after refresh",
        &status_output,
    );
    let status_json = parse_stdout_json(&status_output);
    let pricing_quote = status_units(&status_json)
        .iter()
        .find(|unit| unit["id"] == "pricing/pricing_quote")
        .unwrap();
    assert_eq!(
        pricing_quote["semantic_review"]["compatibility_key"],
        DATA_SEAM_COMPATIBILITY_KEY
    );
    let discount_strategy = status_units(&status_json)
        .iter()
        .find(|unit| unit["id"] == "pricing/discount_strategy")
        .unwrap();
    assert_eq!(
        discount_strategy["semantic_review"]["compatibility_key"],
        SUM_SEAM_COMPATIBILITY_KEY
    );

    let export_output = run_in(&ecommerce_dir, &["export", ".", "--format", "json"]);
    assert_output_success(
        "export should surface canonical seam semantic reviews after refresh",
        &export_output,
    );
    let export_json = parse_stdout_json(&export_output);
    let exported_pricing_quote = export_json["passports"]
        .as_array()
        .unwrap()
        .iter()
        .find(|passport| passport["id"] == "pricing/pricing_quote")
        .unwrap();
    assert_eq!(
        exported_pricing_quote["semantic_review"]["compatibility_key"],
        DATA_SEAM_COMPATIBILITY_KEY
    );
    let exported_discount_strategy = export_json["passports"]
        .as_array()
        .unwrap()
        .iter()
        .find(|passport| passport["id"] == "pricing/discount_strategy")
        .unwrap();
    assert_eq!(
        exported_discount_strategy["semantic_review"]["compatibility_key"],
        SUM_SEAM_COMPATIBILITY_KEY
    );
}

#[test]
fn spec_status_text_lists_units_even_without_semantic_review_story() {
    let temp_dir = temp_repo_dir();
    let project_dir = temp_dir.path();
    let units_dir = write_semantic_status_project(project_dir);
    seed_semantic_status_artifacts(&units_dir);

    let output = run_in(project_dir, &["status", units_dir.to_str().unwrap()]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("pricing/discount_mode"), "{stdout}");
}

#[test]
fn spec_status_text_stays_neutral_for_unsupported_surface_in_preserve_mode() {
    let temp_dir = temp_repo_dir();
    let project_dir = temp_dir.path();
    let units_dir = write_unsupported_function_semantic_status_project(project_dir);
    let passport_path = units_dir.join("pricing/calculate_total.spec.passport.json");
    let review = unsupported_function_semantic_review("seeded unsupported function review");
    seed_unsupported_function_semantic_status_artifacts(&units_dir, Some(review));
    let seeded_review = read_passport_json(&passport_path)["semantic_review"].clone();
    let seeded_summary = seeded_review["summary"]
        .as_str()
        .expect("expected seeded unsupported semantic review summary");
    let seeded_hint = seeded_review["rewrite_hints"][0]
        .as_str()
        .expect("expected seeded unsupported semantic review hint");

    let output = run_in(project_dir, &["status", units_dir.to_str().unwrap()]);
    assert!(
        !output.status.success(),
        "status stays non-green because helper units remain untested"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("pricing/calculate_total"), "{stdout}");
    assert!(!stdout.contains(seeded_summary), "{stdout}");
    assert!(stdout.contains(seeded_hint), "{stdout}");
}

#[test]
fn spec_status_json_and_export_include_compatibility_key_for_data_semantic_review() {
    let (_temp_dir, project_dir) = setup_m12_data_seam_project();
    let units_dir = project_dir.join("units");
    let passport_path = units_dir.join("pricing/pricing_quote.spec.passport.json");

    seed_supported_data_semantic_status_artifacts(&units_dir, None);
    let seeded_passport = read_passport_json(&passport_path);
    let seeded_review = seeded_passport["semantic_review"].clone();
    assert_eq!(
        seeded_review["compatibility_key"],
        DATA_SEAM_COMPATIBILITY_KEY
    );

    let status_output = run_in(&project_dir, &["status", "units", "--format", "json"]);
    assert!(
        !status_output.status.success(),
        "status stays non-green because sibling helper units remain untested"
    );
    let status_json = parse_stdout_json(&status_output);
    assert_eq!(status_json["schema_version"], 5);
    let unit = status_units(&status_json)
        .iter()
        .find(|unit| unit["id"] == "pricing/pricing_quote")
        .unwrap();
    assert_eq!(unit["status"], "valid", "{status_json}");
    assert_eq!(unit["semantic_review"], seeded_review, "{status_json}");

    let export_output = run_in(&project_dir, &["export", "units"]);
    assert_output_success("supported data export should succeed", &export_output);
    let export_json = parse_stdout_json(&export_output);
    assert_eq!(export_json["schema_version"], 5);
    let passport = export_json["passports"]
        .as_array()
        .unwrap()
        .iter()
        .find(|passport| passport["id"] == "pricing/pricing_quote")
        .unwrap();
    assert_eq!(passport["semantic_review"], seeded_review, "{export_json}");
}

#[test]
fn spec_status_demotes_supported_data_review_to_incomplete() {
    let (_temp_dir, project_dir) = setup_m12_data_seam_project();
    let units_dir = project_dir.join("units");
    let review = supported_pricing_quote_semantic_review(
        SemanticVerdict::UnderSpecified,
        vec![SemanticReasonCode::MissingSemanticMethods],
        "authored semantic surfaces are too weak for honest evaluation",
    );
    seed_supported_data_semantic_status_artifacts(&units_dir, Some(review.clone()));

    let status_output = run_in(&project_dir, &["status", "units", "--format", "json"]);
    assert!(
        !status_output.status.success(),
        "supported data semantic demotion should make status non-green"
    );
    let status_json = parse_stdout_json(&status_output);
    let unit = status_units(&status_json)
        .iter()
        .find(|unit| unit["id"] == "pricing/pricing_quote")
        .unwrap();
    assert_eq!(unit["status"], "incomplete", "{status_json}");
    assert_eq!(
        unit["reason"],
        "semantic under-specified: authored semantic surfaces are too weak for honest evaluation",
        "{status_json}"
    );
    assert_eq!(
        unit["semantic_review"]["compatibility_key"],
        DATA_SEAM_COMPATIBILITY_KEY
    );
    assert_eq!(unit["semantic_review"]["verdict"], "under_specified");
}

#[test]
fn spec_status_demotes_supported_data_review_to_failing() {
    let (_temp_dir, project_dir) = setup_m12_data_seam_project();
    let units_dir = project_dir.join("units");
    let review = supported_pricing_quote_semantic_review(
        SemanticVerdict::SemanticDrift,
        vec![SemanticReasonCode::MethodBodyMissingCapBehavior],
        "executable lowering contradicts authored semantic claims",
    );
    seed_supported_data_semantic_status_artifacts(&units_dir, Some(review.clone()));

    let status_output = run_in(&project_dir, &["status", "units", "--format", "json"]);
    assert!(
        !status_output.status.success(),
        "supported data semantic drift should make status non-green"
    );
    let status_json = parse_stdout_json(&status_output);
    let unit = status_units(&status_json)
        .iter()
        .find(|unit| unit["id"] == "pricing/pricing_quote")
        .unwrap();
    assert_eq!(unit["status"], "failing", "{status_json}");
    assert_eq!(
        unit["reason"], "semantic drift: executable lowering contradicts authored semantic claims",
        "{status_json}"
    );
    assert_eq!(
        unit["semantic_review"]["compatibility_key"],
        DATA_SEAM_COMPATIBILITY_KEY
    );
    assert_eq!(unit["semantic_review"]["verdict"], "semantic_drift");
}

#[test]
fn spec_status_keeps_stale_base_health_over_supported_data_semantic_review() {
    let (_temp_dir, project_dir) = setup_m12_data_seam_project();
    let units_dir = project_dir.join("units");
    let passport_path = units_dir.join("pricing/pricing_quote.spec.passport.json");
    let review = supported_pricing_quote_semantic_review(
        SemanticVerdict::SemanticDrift,
        vec![SemanticReasonCode::MethodBodyMissingCapBehavior],
        "executable lowering contradicts authored semantic claims",
    );
    seed_supported_data_semantic_status_artifacts(&units_dir, Some(review));
    let seeded_review = read_passport_json(&passport_path)["semantic_review"].clone();

    let unit_path = units_dir.join("pricing/pricing_quote.unit.spec");
    let source = fs::read_to_string(&unit_path).unwrap();
    fs::write(
        &unit_path,
        source.replace(
            "Quote a checkout total from subtotal plus discount and tax rates.",
            "Quote a checkout total from subtotal plus discount and tax rates with revised authored truth.",
        ),
    )
    .unwrap();

    let status_output = run_in(&project_dir, &["status", "units", "--format", "json"]);
    assert!(!status_output.status.success());
    let status_json = parse_stdout_json(&status_output);
    let unit = status_units(&status_json)
        .iter()
        .find(|unit| unit["id"] == "pricing/pricing_quote")
        .unwrap();
    assert_eq!(unit["status"], "stale", "{status_json}");
    assert_eq!(
        unit["reason"], "authored truth changed since last test",
        "{status_json}"
    );
    assert_eq!(unit["semantic_review"], seeded_review, "{status_json}");
}

#[test]
fn supported_data_semantic_review_command_matrix_preserves_or_refreshes_by_flow() {
    if !cargo_available() {
        return;
    }

    let (_temp_dir, project_dir) = setup_m12_data_seam_project();
    let units_dir = project_dir.join("units");
    let passport_path = units_dir.join("pricing/pricing_quote.spec.passport.json");
    let review = supported_pricing_quote_semantic_review(
        SemanticVerdict::Aligned,
        vec![],
        "seeded supported data review",
    );
    seed_supported_data_semantic_status_artifacts(&units_dir, Some(review));
    let seeded_review = read_passport_json(&passport_path)["semantic_review"].clone();

    let status_output = run_in(&project_dir, &["status", "units", "--format", "json"]);
    assert!(
        !status_output.status.success(),
        "status stays non-green because sibling helper units remain untested"
    );
    let status_json = parse_stdout_json(&status_output);
    let unit = status_units(&status_json)
        .iter()
        .find(|unit| unit["id"] == "pricing/pricing_quote")
        .unwrap();
    assert_eq!(unit["status"], "valid", "{status_json}");
    assert_eq!(unit["semantic_review"], seeded_review, "{status_json}");

    let export_output = run_in(&project_dir, &["export", "units"]);
    assert_output_success("supported data export should succeed", &export_output);
    let export_json = parse_stdout_json(&export_output);
    let exported_passport = export_json["passports"]
        .as_array()
        .unwrap()
        .iter()
        .find(|passport| passport["id"] == "pricing/pricing_quote")
        .unwrap();
    assert_eq!(
        exported_passport["semantic_review"], seeded_review,
        "{export_json}"
    );

    let generate_output = run_in(
        &project_dir,
        &["generate", "units", "--output", "src/generated"],
    );
    assert_output_success("supported data generate should succeed", &generate_output);
    assert_eq!(
        read_passport_json(&passport_path)["semantic_review"],
        seeded_review
    );

    let build_output = run_in(
        &project_dir,
        &[
            "build",
            "units",
            "--output",
            "src/generated",
            "--crate-root",
            ".",
        ],
    );
    assert_output_success("supported data build should succeed", &build_output);
    assert_eq!(
        read_passport_json(&passport_path)["semantic_review"],
        seeded_review
    );

    let test_output = run_in(
        &project_dir,
        &[
            "test",
            "units",
            "--output",
            "src/generated",
            "--crate-root",
            ".",
        ],
    );
    assert_output_success("supported data test should succeed", &test_output);
    let refreshed_review = read_passport_json(&passport_path)["semantic_review"].clone();
    assert_ne!(refreshed_review, seeded_review);
    assert_eq!(
        refreshed_review["compatibility_key"],
        DATA_SEAM_COMPATIBILITY_KEY
    );
    assert_eq!(
        refreshed_review["evaluator_scope"],
        "supported_data_surface"
    );
}

fn assert_supported_function_command_matrix(
    project_dir: &Path,
    unit_id: &str,
    passport_path: &Path,
    seeded_review: &Value,
    expected_refreshed_key: &str,
) {
    let status_output = run_in(project_dir, &["status", "units", "--format", "json"]);
    assert!(
        !status_output.status.success(),
        "status stays non-green because helper units remain untested"
    );
    let status_json = parse_stdout_json(&status_output);
    let unit = status_units(&status_json)
        .iter()
        .find(|unit| unit["id"] == unit_id)
        .unwrap();
    assert_eq!(unit["status"], "valid", "{status_json}");
    assert_eq!(unit["semantic_review"], *seeded_review, "{status_json}");

    let export_output = run_in(project_dir, &["export", "units"]);
    assert_output_success("supported function export should succeed", &export_output);
    let export_json = parse_stdout_json(&export_output);
    let exported_passport = export_json["passports"]
        .as_array()
        .unwrap()
        .iter()
        .find(|passport| passport["id"] == unit_id)
        .unwrap();
    assert_eq!(
        exported_passport["semantic_review"], *seeded_review,
        "{export_json}"
    );

    let generate_output = run_in(
        project_dir,
        &["generate", "units", "--output", "src/generated"],
    );
    assert_output_success(
        "supported function generate should succeed",
        &generate_output,
    );
    assert_eq!(
        read_passport_json(passport_path)["semantic_review"],
        *seeded_review
    );

    let build_output = run_in(
        project_dir,
        &[
            "build",
            "units",
            "--output",
            "src/generated",
            "--crate-root",
            ".",
        ],
    );
    assert_output_success("supported function build should succeed", &build_output);
    assert_eq!(
        read_passport_json(passport_path)["semantic_review"],
        *seeded_review
    );

    let test_output = run_in(
        project_dir,
        &[
            "test",
            "units",
            "--output",
            "src/generated",
            "--crate-root",
            ".",
        ],
    );
    assert_output_success("supported function test should succeed", &test_output);
    let refreshed_review = read_passport_json(passport_path)["semantic_review"].clone();
    assert_ne!(refreshed_review, *seeded_review);
    assert_eq!(
        refreshed_review["compatibility_key"],
        expected_refreshed_key
    );
    assert_eq!(
        refreshed_review["evaluator_scope"],
        "supported_function_surface"
    );
}

fn assert_function_review_drops_on_preserve_and_refreshes_on_test(
    project_dir: &Path,
    unit_id: &str,
    passport_path: &Path,
    expected_refreshed_key: &str,
) {
    let status_output = run_in(project_dir, &["status", "units", "--format", "json"]);
    assert!(
        !status_output.status.success(),
        "status stays non-green because helper units remain untested"
    );
    let status_json = parse_stdout_json(&status_output);
    let unit = status_units(&status_json)
        .iter()
        .find(|unit| unit["id"] == unit_id)
        .unwrap();
    assert_eq!(unit["status"], "valid", "{status_json}");
    assert_semantic_review_absent(&unit["semantic_review"]);

    let export_output = run_in(project_dir, &["export", "units"]);
    assert_output_success("supported function export should succeed", &export_output);
    let export_json = parse_stdout_json(&export_output);
    let exported_passport = export_json["passports"]
        .as_array()
        .unwrap()
        .iter()
        .find(|passport| passport["id"] == unit_id)
        .unwrap();
    assert_semantic_review_absent(&exported_passport["semantic_review"]);

    let generate_output = run_in(
        project_dir,
        &["generate", "units", "--output", "src/generated"],
    );
    assert_output_success(
        "supported function generate should succeed",
        &generate_output,
    );
    assert_semantic_review_absent(&read_passport_json(passport_path)["semantic_review"]);

    let build_output = run_in(
        project_dir,
        &[
            "build",
            "units",
            "--output",
            "src/generated",
            "--crate-root",
            ".",
        ],
    );
    assert_output_success("supported function build should succeed", &build_output);
    assert_semantic_review_absent(&read_passport_json(passport_path)["semantic_review"]);

    let test_output = run_in(
        project_dir,
        &[
            "test",
            "units",
            "--output",
            "src/generated",
            "--crate-root",
            ".",
        ],
    );
    assert_output_success("supported function test should succeed", &test_output);
    let refreshed_review = read_passport_json(passport_path)["semantic_review"].clone();
    assert_eq!(
        refreshed_review["compatibility_key"],
        expected_refreshed_key
    );
    assert_eq!(
        refreshed_review["evaluator_scope"],
        "supported_function_surface"
    );
}

#[test]
fn spec_status_demotes_supported_function_review_to_incomplete() {
    let temp_dir = temp_repo_dir();
    let project_dir = temp_dir.path();
    let units_dir = write_supported_function_semantic_status_project(project_dir);
    let review = supported_function_semantic_review(
        FUNCTION_FAMILY_A_COMPATIBILITY_KEY,
        SemanticVerdict::UnderSpecified,
        vec![SemanticReasonCode::OutsideHonestSupportedSubset],
        "supported semantic bodies fall outside the honest evaluator subset",
    );
    seed_supported_function_semantic_status_artifacts(&units_dir, Some(review));

    let status_output = run_in(project_dir, &["status", "units", "--format", "json"]);
    assert!(
        !status_output.status.success(),
        "supported function under-specification should make status non-green"
    );
    let status_json = parse_stdout_json(&status_output);
    let unit = status_units(&status_json)
        .iter()
        .find(|unit| unit["id"] == "pricing/apply_discount")
        .unwrap();
    assert_eq!(unit["status"], "incomplete", "{status_json}");
    assert_eq!(
        unit["reason"],
        "semantic under-specified: supported semantic bodies fall outside the honest evaluator subset",
        "{status_json}"
    );
    assert_eq!(
        unit["semantic_review"]["compatibility_key"],
        FUNCTION_FAMILY_A_COMPATIBILITY_KEY
    );
    assert_eq!(unit["semantic_review"]["verdict"], "under_specified");
}

#[test]
fn spec_status_demotes_supported_function_review_to_failing() {
    let temp_dir = temp_repo_dir();
    let project_dir = temp_dir.path();
    let units_dir = write_supported_function_semantic_status_project(project_dir);
    let review = supported_function_semantic_review(
        FUNCTION_FAMILY_A_COMPATIBILITY_KEY,
        SemanticVerdict::SemanticDrift,
        vec![SemanticReasonCode::FunctionBodyContradictsSemanticIntent],
        "executable lowering contradicts authored semantic claims",
    );
    seed_supported_function_semantic_status_artifacts(&units_dir, Some(review));

    let status_output = run_in(project_dir, &["status", "units", "--format", "json"]);
    assert!(
        !status_output.status.success(),
        "supported function semantic drift should make status non-green"
    );
    let status_json = parse_stdout_json(&status_output);
    let unit = status_units(&status_json)
        .iter()
        .find(|unit| unit["id"] == "pricing/apply_discount")
        .unwrap();
    assert_eq!(unit["status"], "failing", "{status_json}");
    assert_eq!(
        unit["reason"], "semantic drift: executable lowering contradicts authored semantic claims",
        "{status_json}"
    );
    assert_eq!(
        unit["semantic_review"]["compatibility_key"],
        FUNCTION_FAMILY_A_COMPATIBILITY_KEY
    );
    assert_eq!(unit["semantic_review"]["verdict"], "semantic_drift");
}

#[test]
fn spec_status_keeps_stale_base_health_over_supported_function_semantic_review() {
    let temp_dir = temp_repo_dir();
    let project_dir = temp_dir.path();
    let units_dir = write_supported_function_semantic_status_project(project_dir);
    let passport_path = units_dir.join("pricing/apply_discount.spec.passport.json");
    let review = supported_function_semantic_review(
        FUNCTION_FAMILY_A_COMPATIBILITY_KEY,
        SemanticVerdict::SemanticDrift,
        vec![SemanticReasonCode::FunctionBodyContradictsSemanticIntent],
        "executable lowering contradicts authored semantic claims",
    );
    seed_supported_function_semantic_status_artifacts(&units_dir, Some(review));
    let seeded_review = read_passport_json(&passport_path)["semantic_review"].clone();

    let unit_path = units_dir.join("pricing/apply_discount.unit.spec");
    let source = fs::read_to_string(&unit_path).unwrap();
    fs::write(
        &unit_path,
        source.replace("- output >= 0", "- output >= Decimal::ZERO"),
    )
    .unwrap();

    let status_output = run_in(project_dir, &["status", "units", "--format", "json"]);
    assert!(!status_output.status.success());
    let status_json = parse_stdout_json(&status_output);
    let unit = status_units(&status_json)
        .iter()
        .find(|unit| unit["id"] == "pricing/apply_discount")
        .unwrap();
    assert_eq!(unit["status"], "stale", "{status_json}");
    assert!(!unit["reason"].is_null(), "{status_json}");
    assert_eq!(unit["semantic_review"], seeded_review, "{status_json}");
}

#[test]
fn spec_status_keeps_failing_base_health_over_supported_function_semantic_review() {
    let temp_dir = temp_repo_dir();
    let project_dir = temp_dir.path();
    let units_dir = write_supported_function_semantic_status_project(project_dir);
    let review = supported_function_semantic_review(
        FUNCTION_FAMILY_A_COMPATIBILITY_KEY,
        SemanticVerdict::UnderSpecified,
        vec![SemanticReasonCode::OutsideHonestSupportedSubset],
        "supported semantic bodies fall outside the honest evaluator subset",
    );
    seed_supported_function_semantic_status_artifacts(&units_dir, Some(review));
    let unit_path = units_dir.join("pricing/apply_discount.unit.spec");

    let mut passport = read_passport_record(&unit_path).unwrap().unwrap();
    passport
        .evidence
        .as_mut()
        .expect("seeded passport evidence")
        .build_status = "fail".to_string();
    write_passport(&passport, &unit_path).unwrap();

    let status_output = run_in(project_dir, &["status", "units", "--format", "json"]);
    assert!(!status_output.status.success());
    let status_json = parse_stdout_json(&status_output);
    let unit = status_units(&status_json)
        .iter()
        .find(|unit| unit["id"] == "pricing/apply_discount")
        .unwrap();
    assert_eq!(unit["status"], "failing", "{status_json}");
    assert_eq!(unit["reason"], "build failed", "{status_json}");
    assert_eq!(
        unit["semantic_review"]["compatibility_key"],
        FUNCTION_FAMILY_A_COMPATIBILITY_KEY
    );
    assert_eq!(unit["semantic_review"]["verdict"], "under_specified");
}

#[test]
fn supported_function_semantic_review_command_matrix_preserves_or_refreshes_by_flow() {
    if !cargo_available() {
        return;
    }

    let temp_dir = temp_repo_dir();
    let project_dir = temp_dir.path();
    let units_dir = write_supported_function_semantic_status_project(project_dir);
    let passport_path = units_dir.join("pricing/apply_discount.spec.passport.json");
    let review = supported_function_semantic_review(
        FUNCTION_FAMILY_A_COMPATIBILITY_KEY,
        SemanticVerdict::Aligned,
        vec![],
        "seeded supported function review",
    );
    seed_supported_function_semantic_status_artifacts(&units_dir, Some(review));
    let seeded_review = read_passport_json(&passport_path)["semantic_review"].clone();
    assert_eq!(
        seeded_review["compatibility_key"],
        FUNCTION_FAMILY_A_COMPATIBILITY_KEY
    );

    assert_supported_function_command_matrix(
        project_dir,
        "pricing/apply_discount",
        &passport_path,
        &seeded_review,
        FUNCTION_FAMILY_A_COMPATIBILITY_KEY,
    );
}

#[test]
fn supported_wrapper_function_semantic_review_command_matrix_preserves_or_refreshes_by_flow() {
    if !cargo_available() {
        return;
    }

    let temp_dir = temp_repo_dir();
    let project_dir = temp_dir.path();
    let units_dir = write_supported_wrapper_function_semantic_status_project(project_dir);
    let passport_path = units_dir.join("pricing/calculate_total.spec.passport.json");
    let review = supported_function_semantic_review(
        FUNCTION_FAMILY_B_COMPATIBILITY_KEY,
        SemanticVerdict::Aligned,
        vec![],
        "seeded supported wrapper review",
    );
    let projected_review =
        seed_supported_wrapper_function_semantic_status_artifacts(&units_dir, Some(review));
    assert_eq!(
        projected_review.unwrap().compatibility_key,
        FUNCTION_FAMILY_B_COMPATIBILITY_KEY
    );
    let seeded_review = read_passport_json(&passport_path)["semantic_review"].clone();
    assert_eq!(
        seeded_review["compatibility_key"],
        FUNCTION_FAMILY_B_COMPATIBILITY_KEY
    );

    assert_supported_function_command_matrix(
        project_dir,
        "pricing/calculate_total",
        &passport_path,
        &seeded_review,
        FUNCTION_FAMILY_B_COMPATIBILITY_KEY,
    );
}

#[test]
fn wrapper_pipeline_truth_surface_command_matrix_preserves_until_spec_test_refresh() {
    supported_wrapper_function_semantic_review_command_matrix_preserves_or_refreshes_by_flow();
}

#[test]
fn normalized_required_arg_wrapper_truth_surface_command_matrix_preserves_until_spec_test_refresh()
{
    if !cargo_available() {
        return;
    }

    let temp_dir = temp_repo_dir();
    let project_dir = temp_dir.path();
    let units_dir =
        write_supported_normalized_required_arg_wrapper_function_semantic_status_project(
            project_dir,
        );
    let passport_path = units_dir.join("pricing/calculate_total.spec.passport.json");
    let review = supported_function_semantic_review(
        FUNCTION_FAMILY_B_NORMALIZED_REQUIRED_ARG_COMPATIBILITY_KEY,
        SemanticVerdict::Aligned,
        vec![],
        "seeded supported normalized wrapper review",
    );
    let projected_review =
        seed_supported_wrapper_function_semantic_status_artifacts(&units_dir, Some(review));
    assert_eq!(
        projected_review.unwrap().compatibility_key,
        FUNCTION_FAMILY_B_NORMALIZED_REQUIRED_ARG_COMPATIBILITY_KEY
    );
    let seeded_review = read_passport_json(&passport_path)["semantic_review"].clone();
    assert_eq!(
        seeded_review["compatibility_key"],
        FUNCTION_FAMILY_B_NORMALIZED_REQUIRED_ARG_COMPATIBILITY_KEY
    );

    assert_supported_function_command_matrix(
        project_dir,
        "pricing/calculate_total",
        &passport_path,
        &seeded_review,
        FUNCTION_FAMILY_B_NORMALIZED_REQUIRED_ARG_COMPATIBILITY_KEY,
    );
}

#[test]
fn helper_identity_passthrough_truth_surfaces_preserve_supported_semantic_review() {
    if !cargo_available() {
        return;
    }

    let temp_dir = temp_repo_dir();
    let project_dir = temp_dir.path();
    let units_dir = write_supported_helper_function_semantic_status_project(project_dir);
    let passport_path = units_dir.join("money/round.spec.passport.json");

    let test_output = run_in(
        project_dir,
        &[
            "test",
            "units",
            "--output",
            "src/generated",
            "--crate-root",
            ".",
        ],
    );
    assert_output_success("helper semantic review test should succeed", &test_output);

    let seeded_review = read_passport_json(&passport_path)["semantic_review"].clone();
    assert_supported_function_semantic_review(
        &seeded_review,
        FUNCTION_HELPER_IDENTITY_PASSTHROUGH_COMPATIBILITY_KEY,
    );
    assert_eq!(seeded_review["verdict"], "aligned");

    let status_output = run_in(project_dir, &["status", "units", "--format", "json"]);
    assert_output_success(
        "fresh helper semantic review status should stay green",
        &status_output,
    );
    let status_json = parse_stdout_json(&status_output);
    let unit = status_units(&status_json)
        .iter()
        .find(|unit| unit["id"] == "money/round")
        .unwrap();
    assert_eq!(unit["status"], "valid", "{status_json}");
    assert!(unit["reason"].is_null(), "{status_json}");
    assert_eq!(unit["semantic_review"], seeded_review, "{status_json}");

    let export_output = run_in(project_dir, &["export", "units"]);
    assert_output_success(
        "helper semantic review export should succeed",
        &export_output,
    );
    let export_json = parse_stdout_json(&export_output);
    let exported_passport = export_json["passports"]
        .as_array()
        .unwrap()
        .iter()
        .find(|passport| passport["id"] == "money/round")
        .unwrap();
    assert_eq!(
        exported_passport["semantic_review"], seeded_review,
        "{export_json}"
    );
}

#[test]
fn cross_library_monotone_up_truth_surfaces_preserve_supported_semantic_review() {
    if !cargo_available() {
        return;
    }

    let fixture = setup_isolated_m9_repo_fixture();
    fs::write(
        fixture.app_root.join("spec.toml"),
        "[libraries]\nshared = \"../shared-spec\"\n",
    )
    .unwrap();
    write_file(
        &fixture.app_root,
        "Cargo.toml",
        r#"[package]
name = "app"
version = "0.1.0"
edition = "2024"

[dependencies]
rust_decimal = { version = "1.36", features = ["serde"] }
shared = { path = "../shared-crate" }

[workspace]
"#,
    );
    write_file(
        &fixture.app_root,
        "src/main.rs",
        "mod generated;\npub use generated::*;\nfn main() {}\n",
    );
    let shared_crate_root = fixture.app_root.parent().unwrap().join("shared-crate");
    write_file(
        &shared_crate_root,
        "Cargo.toml",
        r#"[package]
name = "shared"
version = "0.1.0"
edition = "2024"

[dependencies]
rust_decimal = { version = "1.36", features = ["serde"] }

[lib]
path = "src/lib.rs"
"#,
    );
    write_file(
        &shared_crate_root,
        "src/lib.rs",
        "pub mod money {\n    pub mod round {\n        use rust_decimal::Decimal;\n\n        pub fn round(value: Decimal) -> Decimal {\n            value\n        }\n    }\n}\n",
    );
    write_spec(
        &fixture.app_root.join("units"),
        "pricing/apply_tax.unit.spec",
        r#"
id: pricing/apply_tax
kind: function
intent:
  why: Add sales tax to a subtotal using a rate expressed as a decimal fraction.
spec_version: "0.3.0"
contract:
  inputs:
    subtotal: Decimal
    rate: Decimal
  returns: Decimal
  invariants:
    - output >= subtotal
deps:
  - shared::money/round
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
    expect: "apply_tax(Decimal::new(10000, 2), Decimal::new(725, 4)) == Decimal::new(10725, 2)"
"#,
    );
    write_spec(
        &fixture.shared_root.join("units"),
        "money/round.unit.spec",
        r#"
id: money/round
kind: function
intent:
  why: Round a decimal value to two fractional digits for pricing flows.
spec_version: "0.3.0"
contract:
  inputs:
    value: Decimal
  returns: Decimal
imports:
  - rust_decimal::Decimal
body:
  rust: |
    {
        value
    }
local_tests:
  - id: basic
    expect: "round(Decimal::new(1001, 2)) == Decimal::new(1001, 2)"
"#,
    );

    let test_output = run_in(
        &fixture.app_root,
        &[
            "test",
            "units/pricing/apply_tax.unit.spec",
            "--crate-root",
            ".",
        ],
    );
    assert_output_success(
        "cross-library monotone-up spec test should succeed",
        &test_output,
    );

    let passport_path = fixture
        .app_root
        .join("units/pricing/apply_tax.spec.passport.json");
    let seeded_review = read_passport_json(&passport_path)["semantic_review"].clone();
    assert_supported_function_semantic_review(
        &seeded_review,
        FUNCTION_FAMILY_A_UP_COMPATIBILITY_KEY,
    );

    let status_output = run_in(&fixture.app_root, &["status", "units", "--format", "json"]);
    assert_output_success(
        "cross-library monotone-up status should stay green",
        &status_output,
    );
    let status_json = parse_stdout_json(&status_output);
    let status_unit = status_units(&status_json)
        .iter()
        .find(|unit| unit["id"] == "pricing/apply_tax")
        .unwrap();
    assert_eq!(status_unit["status"], "valid", "{status_json}");
    assert!(status_unit["reason"].is_null(), "{status_json}");
    assert_eq!(
        status_unit["semantic_review"], seeded_review,
        "{status_json}"
    );

    let export_output = run_in(&fixture.app_root, &["export", "units"]);
    assert_output_success(
        "cross-library monotone-up export should succeed",
        &export_output,
    );
    let export_json = parse_stdout_json(&export_output);
    let exported_passport = export_json["passports"]
        .as_array()
        .unwrap()
        .iter()
        .find(|passport| passport["id"] == "pricing/apply_tax")
        .unwrap();
    assert_eq!(
        exported_passport["semantic_review"], seeded_review,
        "{export_json}"
    );
}

#[test]
fn m21_chain3_truth_surface_command_matrix_preserves_until_spec_test_refresh() {
    if !cargo_available() {
        return;
    }

    let (_temp_dir, fixture_dir) = copy_m21_chain3_fixture("aligned");
    let unit_path = fixture_dir.join("units/pricing/checkout_chain3_aligned.unit.spec");
    let passport_path =
        fixture_dir.join("units/pricing/checkout_chain3_aligned.spec.passport.json");

    let test_output = run(&[
        "test",
        unit_path.to_str().unwrap(),
        "--crate-root",
        fixture_dir.to_str().unwrap(),
    ]);
    assert_output_success(
        "M21 chain3 aligned fixture test should succeed",
        &test_output,
    );

    let seeded_review = read_passport_json(&passport_path)["semantic_review"].clone();
    assert_supported_function_semantic_review(
        &seeded_review,
        FUNCTION_FAMILY_CHAIN3_COMPATIBILITY_KEY,
    );

    let status_output = run_in(&fixture_dir, &["status", "units", "--format", "json"]);
    assert!(
        !status_output.status.success(),
        "status stays non-green because helper units remain untested"
    );
    let status_json = parse_stdout_json(&status_output);
    let unit = status_units(&status_json)
        .iter()
        .find(|unit| unit["id"] == "pricing/checkout_chain3_aligned")
        .unwrap();
    assert_eq!(unit["status"], "valid", "{status_json}");
    assert_eq!(unit["semantic_review"], seeded_review, "{status_json}");

    let export_output = run_in(&fixture_dir, &["export", "units"]);
    assert_output_success("M21 chain3 export should succeed", &export_output);
    let export_json = parse_stdout_json(&export_output);
    let exported_passport = export_json["passports"]
        .as_array()
        .unwrap()
        .iter()
        .find(|passport| passport["id"] == "pricing/checkout_chain3_aligned")
        .unwrap();
    assert_eq!(
        exported_passport["semantic_review"], seeded_review,
        "{export_json}"
    );

    let generate_output = run_in(
        &fixture_dir,
        &["generate", "units", "--output", "src/generated"],
    );
    assert_output_success("M21 chain3 generate should succeed", &generate_output);
    assert_eq!(
        read_passport_json(&passport_path)["semantic_review"],
        seeded_review
    );

    let build_output = run_in(
        &fixture_dir,
        &[
            "build",
            "units",
            "--output",
            "src/generated",
            "--crate-root",
            ".",
        ],
    );
    assert_output_success("M21 chain3 build should succeed", &build_output);
    assert_eq!(
        read_passport_json(&passport_path)["semantic_review"],
        seeded_review
    );

    let refresh_output = run_in(
        &fixture_dir,
        &[
            "test",
            "units",
            "--output",
            "src/generated",
            "--crate-root",
            ".",
        ],
    );
    assert_output_success("M21 chain3 refresh test should succeed", &refresh_output);
    let refreshed_review = read_passport_json(&passport_path)["semantic_review"].clone();
    assert_eq!(refreshed_review, seeded_review);
    assert_eq!(
        refreshed_review["compatibility_key"],
        FUNCTION_FAMILY_CHAIN3_COMPATIBILITY_KEY
    );
    assert_eq!(
        refreshed_review["evaluator_scope"],
        "supported_function_surface"
    );
}

#[test]
fn m21_chain3_truth_surface_stale_status_and_export_preserve_last_proven_review() {
    if !cargo_available() {
        return;
    }

    let (_temp_dir, fixture_dir) = copy_m21_chain3_fixture("aligned");
    let unit_path = fixture_dir.join("units/pricing/checkout_chain3_aligned.unit.spec");
    let passport_path =
        fixture_dir.join("units/pricing/checkout_chain3_aligned.spec.passport.json");

    let test_output = run(&[
        "test",
        unit_path.to_str().unwrap(),
        "--crate-root",
        fixture_dir.to_str().unwrap(),
    ]);
    assert_output_success(
        "M21 chain3 aligned fixture test should succeed",
        &test_output,
    );

    let seeded_review = read_passport_json(&passport_path)["semantic_review"].clone();
    assert_supported_function_semantic_review(
        &seeded_review,
        FUNCTION_FAMILY_CHAIN3_COMPATIBILITY_KEY,
    );

    let source = fs::read_to_string(&unit_path).unwrap();
    fs::write(
        &unit_path,
        source.replace(
            "Return the final checkout total by computing the taxed discounted subtotal, then applying a surcharge, then applying a loyalty discount.",
            "Return the final checkout total by computing the taxed discounted subtotal, then applying a surcharge, then applying a loyalty discount with revised authored truth.",
        ),
    )
    .unwrap();

    let status_output = run(&["status", fixture_dir.to_str().unwrap(), "--format", "json"]);
    assert!(
        !status_output.status.success(),
        "chain3 stale status should exit non-zero"
    );
    let status_json = parse_stdout_json(&status_output);
    let unit = status_units(&status_json)
        .iter()
        .find(|unit| unit["id"] == "pricing/checkout_chain3_aligned")
        .unwrap();
    assert_eq!(unit["status"], "stale", "{status_json}");
    assert_eq!(unit["reason"], "authored truth changed since last test");
    assert_eq!(unit["semantic_review"], seeded_review, "{status_json}");

    let export_output = run(&["export", fixture_dir.to_str().unwrap()]);
    assert_output_success(
        "M21 chain3 stale export should preserve prior review",
        &export_output,
    );
    let export_json = parse_stdout_json(&export_output);
    let exported_passport = export_json["passports"]
        .as_array()
        .unwrap()
        .iter()
        .find(|passport| passport["id"] == "pricing/checkout_chain3_aligned")
        .unwrap();
    assert_eq!(
        exported_passport["freshness"]["authored_truth_status"],
        "stale"
    );
    assert_eq!(
        exported_passport["semantic_review"], seeded_review,
        "{export_json}"
    );
}

#[test]
fn wrapper_pipeline_truth_surface_stale_status_and_export_preserve_last_proven_review() {
    if !cargo_available() {
        return;
    }

    let temp_dir = temp_repo_dir();
    let project_dir = temp_dir.path();
    let units_dir = write_supported_wrapper_function_semantic_status_project(project_dir);
    let unit_path = units_dir.join("pricing/calculate_total.unit.spec");
    let passport_path = units_dir.join("pricing/calculate_total.spec.passport.json");
    let review = supported_function_semantic_review(
        FUNCTION_FAMILY_B_COMPATIBILITY_KEY,
        SemanticVerdict::Aligned,
        vec![],
        "seeded supported wrapper review",
    );
    seed_supported_wrapper_function_semantic_status_artifacts(&units_dir, Some(review));
    let seeded_review = read_passport_json(&passport_path)["semantic_review"].clone();
    assert_supported_function_semantic_review(&seeded_review, FUNCTION_FAMILY_B_COMPATIBILITY_KEY);

    let source = fs::read_to_string(&unit_path).unwrap();
    fs::write(
        &unit_path,
        source.replace(
            "Return the total after discounting the subtotal and then applying tax.",
            "Return the total after discounting the subtotal and then applying tax with revised authored truth.",
        ),
    )
    .unwrap();

    let status_output = run_in(project_dir, &["status", "units", "--format", "json"]);
    assert!(
        !status_output.status.success(),
        "wrapper pipeline stale status should exit non-zero"
    );
    let status_json = parse_stdout_json(&status_output);
    let unit = status_units(&status_json)
        .iter()
        .find(|unit| unit["id"] == "pricing/calculate_total")
        .unwrap();
    assert_eq!(unit["status"], "stale", "{status_json}");
    assert_eq!(unit["reason"], "authored truth changed since last test");
    assert_eq!(unit["semantic_review"], seeded_review, "{status_json}");

    let export_output = run_in(project_dir, &["export", "units"]);
    assert_output_success(
        "wrapper pipeline stale export should preserve prior review",
        &export_output,
    );
    let export_json = parse_stdout_json(&export_output);
    let exported_passport = export_json["passports"]
        .as_array()
        .unwrap()
        .iter()
        .find(|passport| passport["id"] == "pricing/calculate_total")
        .unwrap();
    assert_eq!(
        exported_passport["freshness"]["authored_truth_status"],
        "stale"
    );
    assert_eq!(
        exported_passport["semantic_review"], seeded_review,
        "{export_json}"
    );
}

#[test]
fn legacy_exact_id_leaf_function_review_drops_on_preserve_and_refreshes_on_test() {
    if !cargo_available() {
        return;
    }

    let temp_dir = temp_repo_dir();
    let project_dir = temp_dir.path();
    let units_dir = write_supported_function_semantic_status_project(project_dir);
    let passport_path = units_dir.join("pricing/apply_discount.spec.passport.json");
    let review = supported_function_semantic_review(
        FUNCTION_FAMILY_A_LEGACY_COMPATIBILITY_KEY,
        SemanticVerdict::Aligned,
        vec![],
        "seeded legacy exact-id function review",
    );
    seed_supported_function_semantic_status_artifacts(&units_dir, Some(review));
    assert_eq!(
        read_passport_json(&passport_path)["semantic_review"]["compatibility_key"],
        FUNCTION_FAMILY_A_LEGACY_COMPATIBILITY_KEY
    );

    assert_function_review_drops_on_preserve_and_refreshes_on_test(
        project_dir,
        "pricing/apply_discount",
        &passport_path,
        FUNCTION_FAMILY_A_COMPATIBILITY_KEY,
    );
}

#[test]
fn legacy_exact_id_wrapper_function_review_drops_on_preserve_and_refreshes_on_test() {
    if !cargo_available() {
        return;
    }

    let temp_dir = temp_repo_dir();
    let project_dir = temp_dir.path();
    let units_dir = write_supported_wrapper_function_semantic_status_project(project_dir);
    let passport_path = units_dir.join("pricing/calculate_total.spec.passport.json");
    let review = supported_function_semantic_review(
        FUNCTION_FAMILY_B_LEGACY_COMPATIBILITY_KEY,
        SemanticVerdict::Aligned,
        vec![],
        "seeded legacy exact-id wrapper review",
    );
    seed_supported_wrapper_function_semantic_status_artifacts(&units_dir, Some(review));
    assert_eq!(
        read_passport_json(&passport_path)["semantic_review"]["compatibility_key"],
        FUNCTION_FAMILY_B_LEGACY_COMPATIBILITY_KEY
    );

    assert_function_review_drops_on_preserve_and_refreshes_on_test(
        project_dir,
        "pricing/calculate_total",
        &passport_path,
        FUNCTION_FAMILY_B_COMPATIBILITY_KEY,
    );
}

#[test]
fn unsupported_near_miss_function_semantic_review_remains_additive_only_and_neutral() {
    let temp_dir = temp_repo_dir();
    let project_dir = temp_dir.path();
    let units_dir = write_unsupported_function_semantic_status_project(project_dir);
    let passport_path = units_dir.join("pricing/calculate_total.spec.passport.json");
    let review = unsupported_function_semantic_review("seeded unsupported function review");
    seed_unsupported_function_semantic_status_artifacts(&units_dir, Some(review));
    let seeded_review = read_passport_json(&passport_path)["semantic_review"].clone();
    assert_eq!(
        seeded_review["compatibility_key"],
        "unsupported.function.v1"
    );

    let status_output = run_in(project_dir, &["status", "units", "--format", "json"]);
    assert!(
        !status_output.status.success(),
        "status stays non-green because helper units remain untested"
    );
    let status_json = parse_stdout_json(&status_output);
    let unit = status_units(&status_json)
        .iter()
        .find(|unit| unit["id"] == "pricing/calculate_total")
        .unwrap();
    assert_eq!(unit["status"], "valid", "{status_json}");
    assert!(unit["reason"].is_null(), "{status_json}");
    assert_eq!(unit["semantic_review"], seeded_review, "{status_json}");
    assert_unsupported_function_semantic_review(&unit["semantic_review"]);
    assert_eq!(
        read_passport_json(&passport_path)["semantic_review"],
        seeded_review
    );
}

#[test]
fn wrapper_pipeline_truth_surface_unsupported_near_miss_command_matrix_stays_neutral() {
    if !cargo_available() {
        return;
    }

    let temp_dir = temp_repo_dir();
    let project_dir = temp_dir.path();
    let units_dir = write_unsupported_function_semantic_status_project(project_dir);
    let passport_path = units_dir.join("pricing/calculate_total.spec.passport.json");
    let review = unsupported_function_semantic_review("seeded unsupported wrapper review");
    seed_unsupported_function_semantic_status_artifacts(&units_dir, Some(review));
    let seeded_review = read_passport_json(&passport_path)["semantic_review"].clone();
    assert_unsupported_function_reason(&seeded_review, "unsupported_required_argument_expression");

    let status_output = run_in(project_dir, &["status", "units", "--format", "json"]);
    assert!(
        !status_output.status.success(),
        "status stays non-green because helper units remain untested"
    );
    let status_json = parse_stdout_json(&status_output);
    let unit = status_units(&status_json)
        .iter()
        .find(|unit| unit["id"] == "pricing/calculate_total")
        .unwrap();
    assert_eq!(unit["status"], "valid", "{status_json}");
    assert!(unit["reason"].is_null(), "{status_json}");
    assert_eq!(unit["semantic_review"], seeded_review, "{status_json}");

    let export_output = run_in(project_dir, &["export", "units"]);
    assert_output_success(
        "wrapper pipeline unsupported near-miss export should succeed",
        &export_output,
    );
    let export_json = parse_stdout_json(&export_output);
    let exported_passport = export_json["passports"]
        .as_array()
        .unwrap()
        .iter()
        .find(|passport| passport["id"] == "pricing/calculate_total")
        .unwrap();
    assert_eq!(
        exported_passport["semantic_review"], seeded_review,
        "{export_json}"
    );

    let build_output = run_in(
        project_dir,
        &[
            "build",
            "units",
            "--output",
            "src/generated",
            "--crate-root",
            ".",
        ],
    );
    assert_output_success(
        "wrapper pipeline unsupported near-miss build should succeed",
        &build_output,
    );
    assert_eq!(
        read_passport_json(&passport_path)["semantic_review"],
        seeded_review
    );

    let refresh_output = run_in(
        project_dir,
        &[
            "test",
            "units",
            "--output",
            "src/generated",
            "--crate-root",
            ".",
        ],
    );
    assert_output_success(
        "wrapper pipeline unsupported near-miss test should succeed",
        &refresh_output,
    );
    let refreshed_review = read_passport_json(&passport_path)["semantic_review"].clone();
    assert_unsupported_function_reason(
        &refreshed_review,
        "unsupported_required_argument_expression",
    );
}

#[test]
fn unsupported_function_review_preserves_when_fresh_drops_when_stale_and_refreshes_on_test() {
    if !cargo_available() {
        return;
    }

    let (_temp_dir, fixture_dir) = copy_m20_unsupported_truth_pack();
    let unit_path = fixture_dir.join("units/pricing/calculate_total.unit.spec");
    let passport_path = fixture_dir.join("units/pricing/calculate_total.spec.passport.json");

    let test_output = run_in(
        &fixture_dir,
        &[
            "test",
            "units",
            "--output",
            "src/generated",
            "--crate-root",
            ".",
        ],
    );
    assert_output_success("unsupported function test should succeed", &test_output);
    let seeded_review = read_passport_json(&passport_path)["semantic_review"].clone();
    assert_unsupported_function_semantic_review(&seeded_review);

    let status_output = run_in(&fixture_dir, &["status", "units", "--format", "json"]);
    assert_output_success(
        "fresh unsupported function status should stay green",
        &status_output,
    );
    let status_json = parse_stdout_json(&status_output);
    let unit = status_units(&status_json)
        .iter()
        .find(|unit| unit["id"] == "pricing/calculate_total")
        .unwrap();
    assert_eq!(unit["status"], "valid", "{status_json}");
    assert!(unit["reason"].is_null(), "{status_json}");
    assert_eq!(unit["semantic_review"], seeded_review, "{status_json}");

    let export_output = run_in(&fixture_dir, &["export", "units"]);
    assert_output_success("unsupported function export should succeed", &export_output);
    let export_json = parse_stdout_json(&export_output);
    let exported_passport = export_json["passports"]
        .as_array()
        .unwrap()
        .iter()
        .find(|passport| passport["id"] == "pricing/calculate_total")
        .unwrap();
    assert_eq!(
        exported_passport["semantic_review"], seeded_review,
        "{export_json}"
    );

    let generate_output = run_in(
        &fixture_dir,
        &["generate", "units", "--output", "src/generated"],
    );
    assert_output_success(
        "unsupported function generate should succeed",
        &generate_output,
    );
    assert_eq!(
        read_passport_json(&passport_path)["semantic_review"],
        seeded_review
    );

    let build_output = run_in(
        &fixture_dir,
        &[
            "build",
            "units",
            "--output",
            "src/generated",
            "--crate-root",
            ".",
        ],
    );
    assert_output_success("unsupported function build should succeed", &build_output);
    assert_eq!(
        read_passport_json(&passport_path)["semantic_review"],
        seeded_review
    );

    let source = fs::read_to_string(&unit_path).unwrap();
    fs::write(
        &unit_path,
        source.replace(
            "Return the total after discounting the subtotal and then applying tax.",
            "Return the total after discounting the subtotal and then applying tax with revised authored truth.",
        ),
    )
    .unwrap();

    let stale_status_output = run_in(&fixture_dir, &["status", "units", "--format", "json"]);
    assert!(
        !stale_status_output.status.success(),
        "stale unsupported function status should exit non-zero"
    );
    let stale_status_json = parse_stdout_json(&stale_status_output);
    let stale_unit = status_units(&stale_status_json)
        .iter()
        .find(|unit| unit["id"] == "pricing/calculate_total")
        .unwrap();
    assert_eq!(stale_unit["status"], "stale", "{stale_status_json}");
    assert_eq!(
        stale_unit["reason"], "authored truth changed since last test",
        "{stale_status_json}"
    );
    assert_semantic_review_absent(&stale_unit["semantic_review"]);

    let stale_export_output = run_in(&fixture_dir, &["export", "units"]);
    assert_output_success(
        "stale unsupported function export should succeed",
        &stale_export_output,
    );
    let stale_export_json = parse_stdout_json(&stale_export_output);
    let stale_exported_passport = stale_export_json["passports"]
        .as_array()
        .unwrap()
        .iter()
        .find(|passport| passport["id"] == "pricing/calculate_total")
        .unwrap();
    assert_semantic_review_absent(&stale_exported_passport["semantic_review"]);

    let stale_build_output = run_in(
        &fixture_dir,
        &[
            "build",
            "units",
            "--output",
            "src/generated",
            "--crate-root",
            ".",
        ],
    );
    assert_output_success(
        "stale unsupported function build should succeed",
        &stale_build_output,
    );
    assert_semantic_review_absent(&read_passport_json(&passport_path)["semantic_review"]);

    let refresh_output = run_in(
        &fixture_dir,
        &[
            "test",
            "units",
            "--output",
            "src/generated",
            "--crate-root",
            ".",
        ],
    );
    assert_output_success(
        "unsupported function refresh test should succeed",
        &refresh_output,
    );
    assert_unsupported_function_semantic_review(
        &read_passport_json(&passport_path)["semantic_review"],
    );
}

#[test]
fn m20_unsupported_truth_pack_whole_pack_status_and_export_cover_public_reason_matrix() {
    if !cargo_available() {
        return;
    }

    let (_temp_dir, fixture_dir) = copy_m20_unsupported_truth_pack();

    let test_output = run_in(
        &fixture_dir,
        &[
            "test",
            "units",
            "--output",
            "src/generated",
            "--crate-root",
            ".",
        ],
    );
    assert_output_success(
        "whole-pack unsupported truth test should succeed",
        &test_output,
    );

    let supported_cases = [
        (
            "pricing/apply_discount",
            FUNCTION_FAMILY_A_COMPATIBILITY_KEY,
        ),
        ("pricing/apply_tax", FUNCTION_FAMILY_A_UP_COMPATIBILITY_KEY),
        (
            "pricing/checkout_total",
            FUNCTION_FAMILY_B_COMPATIBILITY_KEY,
        ),
    ];
    let unsupported_cases = [
        (
            "pricing/apply_discount_control_flow",
            "unsupported_control_flow",
        ),
        (
            "pricing/calculate_total",
            "unsupported_required_argument_expression",
        ),
        (
            "pricing/checkout_total_bad_dep_topology",
            "unsupported_dep_topology",
        ),
        (
            "pricing/checkout_total_bad_body_shape",
            "unsupported_wrapper_body_shape",
        ),
        ("pricing/apply_tax_control_flow", "unsupported_control_flow"),
    ];

    let mut expected_reviews = HashMap::new();

    for (unit_id, compatibility_key) in supported_cases {
        let passport_path = fixture_dir
            .join("units")
            .join(format!("{unit_id}.spec.passport.json"));
        let review = read_passport_json(&passport_path)["semantic_review"].clone();
        assert_supported_function_semantic_review(&review, compatibility_key);
        expected_reviews.insert(unit_id, review);
    }

    for (unit_id, reason) in unsupported_cases {
        let passport_path = fixture_dir
            .join("units")
            .join(format!("{unit_id}.spec.passport.json"));
        let review = read_passport_json(&passport_path)["semantic_review"].clone();
        assert_unsupported_function_reason(&review, reason);
        expected_reviews.insert(unit_id, review);
    }

    let status_output = run_in(&fixture_dir, &["status", ".", "--format", "json"]);
    assert_output_success(
        "whole-pack unsupported truth status should stay green",
        &status_output,
    );
    let status_json = parse_stdout_json(&status_output);

    let export_output = run_in(&fixture_dir, &["export", "."]);
    assert_output_success(
        "whole-pack unsupported truth export should succeed",
        &export_output,
    );
    let export_json = parse_stdout_json(&export_output);

    for (unit_id, expected_review) in expected_reviews {
        let status_unit = status_units(&status_json)
            .iter()
            .find(|unit| unit["id"] == unit_id)
            .unwrap();
        assert_eq!(status_unit["status"], "valid", "{status_json}");
        assert!(status_unit["reason"].is_null(), "{status_json}");
        assert_eq!(
            status_unit["semantic_review"], expected_review,
            "{status_json}"
        );

        let exported_passport = export_json["passports"]
            .as_array()
            .unwrap()
            .iter()
            .find(|passport| passport["id"] == unit_id)
            .unwrap();
        assert_eq!(
            exported_passport["semantic_review"], expected_review,
            "{export_json}"
        );
    }
}

#[test]
fn supported_function_m19_stale_proof_after_semantic_edit_surfaces_on_read_side_commands() {
    if !cargo_available() {
        return;
    }

    let (_temp_dir, fixture_dir) = copy_m19_semantic_falsification_pack();
    let unit_path = fixture_dir.join("units/billing/apply_membership_discount.unit.spec");
    let passport_path =
        fixture_dir.join("units/billing/apply_membership_discount.spec.passport.json");

    let test_output = run(&[
        "test",
        unit_path.to_str().unwrap(),
        "--crate-root",
        fixture_dir.to_str().unwrap(),
    ]);
    assert_output_success("M19 supported function test should succeed", &test_output);

    let seeded_review = read_passport_json(&passport_path)["semantic_review"].clone();
    assert_eq!(
        seeded_review["compatibility_key"],
        FUNCTION_FAMILY_A_COMPATIBILITY_KEY
    );
    assert_eq!(seeded_review["verdict"], "aligned");
    assert_eq!(
        seeded_review["summary"],
        "authored semantics and executable lowering agree on the supported function surface",
    );
    assert_eq!(
        seeded_review["evaluator_scope"],
        "supported_function_surface"
    );

    let source = fs::read_to_string(&unit_path).unwrap();
    fs::write(
        &unit_path,
        source.replacen("output >= 0", "output >= Decimal::ZERO", 1),
    )
    .unwrap();

    let status_output = run(&["status", fixture_dir.to_str().unwrap(), "--format", "json"]);
    assert!(
        !status_output.status.success(),
        "status should surface stale proof after a semantic edit"
    );
    let status_json = parse_stdout_json(&status_output);
    let unit = status_units(&status_json)
        .iter()
        .find(|unit| unit["id"] == "billing/apply_membership_discount")
        .unwrap();
    assert_eq!(unit["status"], "stale", "{status_json}");
    assert_eq!(unit["reason"], "authored truth changed since last test");
    assert_eq!(unit["semantic_review"], seeded_review, "{status_json}");

    let export_output = run(&["export", fixture_dir.to_str().unwrap()]);
    assert_output_success(
        "M19 supported function export after semantic edit should succeed",
        &export_output,
    );
    let export_json = parse_stdout_json(&export_output);
    let exported_passport = export_json["passports"]
        .as_array()
        .unwrap()
        .iter()
        .find(|passport| passport["id"] == "billing/apply_membership_discount")
        .unwrap();
    assert_eq!(
        exported_passport["freshness"]["authored_truth_status"],
        "stale"
    );
    assert_eq!(
        exported_passport["semantic_review"], seeded_review,
        "{export_json}"
    );
}

fn assert_m19_unsupported_near_miss_command_matrix(unit_relative_path: &str, unit_id: &str) {
    let (_temp_dir, fixture_dir) = copy_m19_semantic_falsification_pack();
    let unit_path = fixture_dir.join(unit_relative_path);
    let passport_path =
        fixture_dir.join(unit_relative_path.replace(".unit.spec", ".spec.passport.json"));

    let test_output = run(&[
        "test",
        unit_path.to_str().unwrap(),
        "--crate-root",
        fixture_dir.to_str().unwrap(),
    ]);
    assert_output_success(
        "M19 unsupported near-miss test should succeed",
        &test_output,
    );
    let seeded_review = read_passport_json(&passport_path)["semantic_review"].clone();
    assert_unsupported_function_semantic_review(&seeded_review);

    let build_output = run(&[
        "build",
        fixture_dir.join("units").to_str().unwrap(),
        "--crate-root",
        fixture_dir.to_str().unwrap(),
    ]);
    assert_output_success(
        "M19 unsupported near-miss build should succeed",
        &build_output,
    );
    assert_eq!(
        read_passport_json(&passport_path)["semantic_review"],
        seeded_review
    );

    let status_output = run(&["status", fixture_dir.to_str().unwrap(), "--format", "json"]);
    assert!(
        !status_output.status.success(),
        "status stays non-green because sibling M19 fixtures remain untested"
    );
    let status_json = parse_stdout_json(&status_output);
    let unit = status_units(&status_json)
        .iter()
        .find(|unit| unit["id"] == unit_id)
        .unwrap();
    assert_eq!(unit["status"], "valid", "{status_json}");
    assert!(unit["reason"].is_null(), "{status_json}");
    assert_eq!(unit["semantic_review"], seeded_review, "{status_json}");

    let export_output = run(&["export", fixture_dir.to_str().unwrap()]);
    assert_output_success(
        "M19 unsupported near-miss export should succeed",
        &export_output,
    );
    let export_json = parse_stdout_json(&export_output);
    let exported_passport = export_json["passports"]
        .as_array()
        .unwrap()
        .iter()
        .find(|passport| passport["id"] == unit_id)
        .unwrap();
    assert_eq!(
        exported_passport["semantic_review"], seeded_review,
        "{export_json}"
    );

    let generate_output = run(&[
        "generate",
        fixture_dir.join("units").to_str().unwrap(),
        "--output",
        fixture_dir.join("src/generated").to_str().unwrap(),
    ]);
    assert_output_success(
        "M19 unsupported near-miss generate should succeed",
        &generate_output,
    );
    assert_eq!(
        read_passport_json(&passport_path)["semantic_review"],
        seeded_review
    );
}

#[test]
fn unsupported_near_miss_m19_family_a_down_command_matrix_stays_neutral() {
    if !cargo_available() {
        return;
    }

    assert_m19_unsupported_near_miss_command_matrix(
        "units/billing/apply_membership_discount_unsupported_near_miss.unit.spec",
        "billing/apply_membership_discount_unsupported_near_miss",
    );
}

#[test]
fn unsupported_near_miss_m19_family_a_up_command_matrix_stays_neutral() {
    if !cargo_available() {
        return;
    }

    assert_m19_unsupported_near_miss_command_matrix(
        "units/billing/apply_regional_fee_unsupported_near_miss.unit.spec",
        "billing/apply_regional_fee_unsupported_near_miss",
    );
}

#[test]
fn unsupported_near_miss_m19_family_b_command_matrix_stays_neutral() {
    if !cargo_available() {
        return;
    }

    assert_m19_unsupported_near_miss_command_matrix(
        "units/billing/checkout_net_total_unsupported_near_miss.unit.spec",
        "billing/checkout_net_total_unsupported_near_miss",
    );
}

#[test]
fn spec_status_drops_supported_sum_review_when_unit_becomes_function() {
    let temp_dir = temp_repo_dir();
    let project_dir = temp_dir.path();
    let units_dir = write_semantic_status_project(project_dir);
    seed_semantic_status_artifacts(&units_dir);

    write_spec(
        &units_dir,
        "pricing/discount_mode.unit.spec",
        r#"
id: pricing/discount_mode
kind: function
intent:
  why: Compute the discount amount directly for checkout pricing.
spec_version: "0.3.0"
contract:
  inputs:
    subtotal: i32
    amount: i32
  returns: i32
body:
  rust: |
    {
        subtotal - amount
    }
local_tests:
  - id: capped_discount
    expect: "discount_mode(10, 3) == 7"
"#,
    );

    let status_output = run_in(
        project_dir,
        &["status", units_dir.to_str().unwrap(), "--format", "json"],
    );
    assert!(!status_output.status.success());
    let status_json = parse_stdout_json(&status_output);
    let unit = status_units(&status_json)
        .iter()
        .find(|unit| unit["id"] == "pricing/discount_mode")
        .unwrap();
    assert_eq!(unit["status"], "stale", "{status_json}");
    assert_semantic_review_absent(&unit["semantic_review"]);
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
    assert_eq!(json["schema_version"], 5);
    let units = status_units(&json);
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
    assert_eq!(json["schema_version"], 5);
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
        &project_dir.join("units/pricing/quote_total.spec.passport.json"),
        "2024-01-02T03:04:05Z",
    );

    fs::write(
        project_dir.join("units/pricing/quote_total.unit.spec"),
        r#"
id: pricing/quote_total
kind: function
intent:
  why: Return a quoted total placeholder.
spec_version: "0.3.0"
contract:
  returns: i32
body:
  rust: |
    { 1 }
local_tests:
  - id: happy_path
    expect: quote_total() == 1
"#,
    )
    .unwrap();

    let output = run_in(project_dir, &["status", "units"]);
    assert!(!output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("~ pricing/quote_total"), "{stdout}");
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
    assert_output_success(
        "spec test should succeed before removing contract",
        &test_output,
    );

    rewrite_passport_generated_at(
        &project_dir.join("units/pricing/quote_total.spec.passport.json"),
        "2024-01-02T03:04:05Z",
    );

    fs::write(
        project_dir.join("units/pricing/quote_total.unit.spec"),
        r#"
id: pricing/quote_total
kind: function
intent:
  why: Return a quoted total placeholder.
spec_version: "0.3.0"
body:
  rust: |
    { true }
local_tests:
  - id: happy_path
    expect: quote_total() == true
"#,
    )
    .unwrap();

    let output = run_in(project_dir, &["status", "units", "--format", "json"]);
    assert!(
        !output.status.success(),
        "contract removal should mark unit stale"
    );
    let json = parse_stdout_json(&output);
    let units = json["units"].as_array().unwrap();
    assert_eq!(units[0]["status"], "stale");
    assert_eq!(units[0]["reason"], "authored truth changed since last test");
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
        &project_dir.join("units/pricing/quote_total.spec.passport.json"),
        "2024-01-02T03:04:05Z",
    );

    let output = run_in(project_dir, &["status", "units", "--format", "json"]);
    assert!(output.status.success());

    assert_stdout_json_matches_fixture(&output, "status-valid.json");
}

#[test]
fn spec_status_reports_molecule_failure_without_poisoning_unit_health() {
    if !cargo_available() {
        return;
    }

    let temp_dir = temp_repo_dir();
    let project_dir = temp_dir.path();
    let units_dir = project_dir.join("units");
    write_file(
        project_dir,
        "Cargo.toml",
        r#"[package]
name = "molecule-status-neutral-project"
version = "0.1.0"
edition = "2024"

[workspace]
"#,
    );
    write_file(
        project_dir,
        "src/main.rs",
        "mod generated;\npub use generated::*;\nfn main() {}\n",
    );
    write_spec(
        &units_dir,
        "pricing/quote_total.unit.spec",
        r#"
id: pricing/quote_total
kind: function
spec_version: "0.3.0"
intent:
  why: Return a quoted total placeholder for status tests.
contract:
  returns: bool
body:
  rust: |
    {
        true
    }
local_tests:
  - id: happy_path
    expect: quote_total() == true
"#,
    );
    write_spec(
        &units_dir,
        "pricing/calculate_total.unit.spec",
        r#"
id: pricing/calculate_total
kind: function
spec_version: "0.3.0"
intent:
  why: Return a calculated total placeholder for status tests.
contract:
  returns: bool
body:
  rust: |
    {
        true
    }
local_tests:
  - id: happy_path
    expect: calculate_total() == true
"#,
    );
    write_spec(
        &units_dir,
        "pricing/tax_and_discount.test.spec",
        r#"id: pricing/tax_and_discount
spec_version: "0.3.0"
intent:
  why: Verify tax and discount interact correctly.
covers:
  - pricing/calculate_total
  - pricing/quote_total
body:
  rust: |
    {
        assert!(true);
    }
"#,
    );

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
    assert_output_success(
        "spec test should seed evidence before status check",
        &output,
    );

    rewrite_json_field(
        &project_dir.join("units/pricing/tax_and_discount.test.evidence.json"),
        "status",
        Value::String("fail".to_string()),
    );

    let output = run_in(project_dir, &["status", "units", "--format", "json"]);
    assert!(
        !output.status.success(),
        "molecule failure should exit non-zero"
    );

    let json = parse_stdout_json(&output);
    let units = status_units(&json);
    assert!(
        units.iter().all(|unit| unit["status"] == "valid"),
        "unit plane should remain valid: {units:?}"
    );
    let molecule_tests = status_molecule_tests(&json);
    assert_eq!(molecule_tests.len(), 1);
    assert_eq!(molecule_tests[0]["id"], "pricing/tax_and_discount");
    assert_eq!(molecule_tests[0]["status"], "failing");
}

#[test]
fn spec_status_zero_roots_is_non_green() {
    let temp_dir = temp_repo_dir();
    let output = run_in(temp_dir.path(), &["status", ".", "--format", "json"]);
    assert!(
        !output.status.success(),
        "empty search root should exit non-zero"
    );

    let json = parse_stdout_json(&output);
    assert_eq!(json["schema_version"], 5);
    assert_eq!(json["roots"], serde_json::json!([]));
    let loader_errors = json["loader_errors"].as_array().unwrap();
    assert_eq!(loader_errors[0]["code"], "SPEC_NO_LIBRARY_ROOTS");
}

#[test]
fn spec_status_repo_root_discovers_multiple_library_roots() {
    let temp_dir = temp_repo_dir();
    let repo_root = temp_dir.path().join("repo");
    fs::create_dir_all(repo_root.join("alpha/units/pricing")).unwrap();
    fs::create_dir_all(repo_root.join("beta/units/money")).unwrap();
    fs::write(repo_root.join(".git"), "gitdir: .git/modules/repo\n").unwrap();
    write_spec(
        &repo_root.join("alpha/units"),
        "pricing/apply_discount.unit.spec",
        r#"
id: pricing/apply_discount
kind: function
intent:
  why: Alpha pricing function.
body:
  rust: |
    { true }
"#,
    );
    write_spec(
        &repo_root.join("beta/units"),
        "money/round.unit.spec",
        r#"
id: money/round
kind: function
intent:
  why: Beta money function.
body:
  rust: |
    { true }
"#,
    );

    let output = run_in(&repo_root, &["status", ".", "--format", "json"]);
    assert!(
        !output.status.success(),
        "untested multi-root status should exit non-zero"
    );

    let json = parse_stdout_json(&output);
    let roots = status_roots(&json);
    assert_eq!(roots.len(), 2);
    assert_eq!(roots[0]["root"], "alpha");
    assert_eq!(roots[1]["root"], "beta");
}

#[test]
fn spec_status_repo_root_honors_each_root_workspace_config() {
    let root = repo_root();
    let temp_dir = temp_repo_dir();
    let app_dir = temp_dir.path().join("crosslib-app");
    let ecommerce_dir = temp_dir.path().join("ecommerce");
    let shared_spec_dir = temp_dir.path().join("shared-spec");

    fs::write(
        temp_dir.path().join(".git"),
        "gitdir: .git/modules/spec-tests\n",
    )
    .unwrap();
    copy_git_tracked_dir(&root.join("examples/crosslib-app"), &app_dir).unwrap();
    copy_git_tracked_dir(&root.join("examples/ecommerce"), &ecommerce_dir).unwrap();
    copy_git_tracked_dir(&root.join("examples/shared-spec"), &shared_spec_dir).unwrap();

    let output = run_in(temp_dir.path(), &["status", ".", "--format", "json"]);
    assert!(
        !output.status.success(),
        "repo status should stay non-green when copied roots still include unproven shared library units"
    );

    let json = parse_stdout_json(&output);
    let roots = status_roots(&json);
    let crosslib_root = roots
        .iter()
        .find(|root| root["root"] == "crosslib-app")
        .expect("expected crosslib-app root in repo status");
    let units = crosslib_root["units"].as_array().unwrap();
    assert_eq!(units.len(), 4, "{json}");
    assert_eq!(units[0]["id"], "pricing/apply_discount");
    assert_eq!(units[0]["status"], "valid", "{json}");
    assert_eq!(
        units[0]["semantic_review"]["compatibility_key"], FUNCTION_FAMILY_A_COMPATIBILITY_KEY,
        "{json}"
    );
    assert_eq!(units[1]["id"], "pricing/apply_tax");
    assert_eq!(units[1]["status"], "valid", "{json}");
    assert_eq!(
        units[1]["semantic_review"]["compatibility_key"], FUNCTION_FAMILY_A_UP_COMPATIBILITY_KEY,
        "{json}"
    );
    assert_eq!(units[2]["id"], "pricing/calculate_total");
    assert_eq!(units[2]["status"], "valid", "{json}");
    assert_eq!(units[3]["id"], "pricing/checkout_nested_chain3");
    assert_eq!(units[3]["status"], "valid", "{json}");
    assert!(
        units.iter().all(|unit| {
            !unit["errors"]
                .as_array()
                .unwrap()
                .iter()
                .any(|error| error["code"] == "SPEC_UNKNOWN_LIBRARY_NAMESPACE")
        }),
        "{json}"
    );

    let ecommerce_root = roots
        .iter()
        .find(|root| root["root"] == "ecommerce")
        .expect("expected ecommerce root in repo status");
    let molecule_tests = ecommerce_root["molecule_tests"].as_array().unwrap();
    assert_eq!(molecule_tests.len(), 3, "{json}");
    assert!(
        molecule_tests.iter().all(|test| test["status"] == "valid"),
        "{json}"
    );
}

#[cfg(unix)]
#[test]
fn spec_status_repo_root_rejects_symlinked_external_library_root() {
    use std::os::unix::fs::symlink;

    let temp_dir = temp_repo_dir();
    let repo_root = temp_dir.path().join("repo");
    fs::create_dir_all(repo_root.join("alpha/units/pricing")).unwrap();
    fs::write(repo_root.join(".git"), "gitdir: .git/modules/repo\n").unwrap();
    write_spec(
        &repo_root.join("alpha/units"),
        "pricing/apply_discount.unit.spec",
        r#"
id: pricing/apply_discount
kind: function
intent:
  why: In-repo library root.
body:
  rust: |
    { true }
"#,
    );

    let outside_dir = tempfile::TempDir::new().unwrap();
    let outside_root = outside_dir.path().join("linked-lib");
    fs::create_dir_all(outside_root.join("units/money")).unwrap();
    write_spec(
        &outside_root.join("units"),
        "money/round.unit.spec",
        r#"
id: money/round
kind: function
intent:
  why: Outside-root library.
body:
  rust: |
    { true }
"#,
    );
    symlink(&outside_root, repo_root.join("linked-lib")).unwrap();

    let output = run_in(&repo_root, &["status", ".", "--format", "json"]);
    assert!(
        !output.status.success(),
        "symlinked external library root should fail status"
    );

    let json = parse_stdout_json(&output);
    let roots = status_roots(&json);
    assert_eq!(roots.len(), 1, "{json}");
    assert_eq!(roots[0]["root"], "alpha");
    let loader_errors = json["loader_errors"].as_array().unwrap();
    assert!(
        loader_errors
            .iter()
            .any(|error| error["code"] == "SPEC_PLAN_SYMLINK_ESCAPE"),
        "{json}"
    );
}

#[cfg(unix)]
#[test]
fn spec_status_rejects_symlinked_external_unit_in_root_library_graph() {
    use std::os::unix::fs::symlink;

    let temp_dir = temp_repo_dir();
    let project_dir = temp_dir.path();
    let units_dir = project_dir.join("units");
    write_spec(
        &units_dir,
        "pricing/apply_discount.unit.spec",
        r#"
id: pricing/apply_discount
kind: function
intent:
  why: In-root unit.
body:
  rust: |
    { true }
"#,
    );

    let outside_dir = tempfile::TempDir::new().unwrap();
    let rogue_spec = outside_dir.path().join("rogue.unit.spec");
    fs::write(
        &rogue_spec,
        r#"
id: pricing/rogue
kind: function
intent:
  why: Escape the local library graph.
body:
  rust: "{ true }"
"#,
    )
    .unwrap();
    symlink(&rogue_spec, units_dir.join("pricing/rogue.unit.spec")).unwrap();

    let output = run_in(project_dir, &["status", "units", "--format", "json"]);
    assert!(
        !output.status.success(),
        "status should fail when a root library graph escapes via symlink"
    );

    let json = parse_stdout_json(&output);
    let units = status_units(&json);
    assert_eq!(units.len(), 1, "{json}");
    assert_eq!(units[0]["id"], "pricing/apply_discount");
    let loader_errors = json["loader_errors"].as_array().unwrap();
    assert!(
        loader_errors
            .iter()
            .any(|error| error["code"] == "SPEC_PLAN_SYMLINK_ESCAPE"),
        "{json}"
    );
}

#[cfg(unix)]
#[test]
fn spec_status_rejects_symlinked_external_unit_in_imported_library_graph() {
    use std::os::unix::fs::symlink;

    let fixture = setup_m9_repo_fixture();
    fs::write(
        fixture.app_root.join("spec.toml"),
        "[libraries]\nshared = \"../shared-spec\"\n",
    )
    .unwrap();
    write_m9_unit(
        &fixture.app_root.join("units"),
        "pricing/apply_discount.unit.spec",
        "pricing/apply_discount",
        &["shared::money/round"],
    );
    write_m9_unit(
        &fixture.shared_root.join("units"),
        "money/round.unit.spec",
        "money/round",
        &[],
    );

    let outside_dir = tempfile::TempDir::new().unwrap();
    let rogue_spec = outside_dir.path().join("rogue.unit.spec");
    fs::write(
        &rogue_spec,
        r#"
id: money/rogue
kind: function
intent:
  why: Escape the imported library graph.
body:
  rust: "{ true }"
"#,
    )
    .unwrap();
    symlink(
        &rogue_spec,
        fixture.shared_root.join("units/money/rogue.unit.spec"),
    )
    .unwrap();

    let output = run_in(&fixture.app_root, &["status", ".", "--format", "json"]);
    assert!(
        !output.status.success(),
        "status should fail when an imported library graph escapes via symlink"
    );

    let json = parse_stdout_json(&output);
    let loader_errors = json["loader_errors"].as_array().unwrap();
    assert!(
        loader_errors
            .iter()
            .any(|error| error["code"] == "SPEC_PLAN_SYMLINK_ESCAPE"),
        "{json}"
    );
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
    let (_temp_dir, units_dir) = setup_apply_discount_unit();
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
    let (_temp_dir, units_dir) = setup_apply_discount_unit();
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
fn spec_status_failing_timeout() {
    let (_temp_dir, units_dir) = setup_apply_discount_unit();
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
    "build_status": "timeout",
    "test_results": [],
    "observed_at": "2024-01-02T03:04:05Z"
  }
}"#,
    );

    let output = run(&["status", units_dir.to_str().unwrap(), "--format", "json"]);
    assert!(!output.status.success(), "timed out unit should exit 1");

    let json = parse_stdout_json(&output);
    let units = json["units"].as_array().unwrap();
    assert_eq!(units[0]["status"], "failing");
    assert_eq!(units[0]["reason"], "build timed out");
    assert_eq!(units[0]["evidence_at"], "2024-01-02T03:04:05Z");
    assert_stdout_json_matches_fixture(&output, "status-failing-timeout.json");
}

#[test]
fn spec_status_failing_test() {
    let (_temp_dir, units_dir) = setup_apply_discount_unit();
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
fn spec_status_failing_tests_plural() {
    let (_temp_dir, units_dir) = setup_apply_discount_unit();
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
      {"id": "happy_path", "status": "fail"},
      {"id": "sad_path", "status": "fail"}
    ],
    "observed_at": "2024-01-02T03:04:05Z"
  }
}"#,
    );

    let output = run(&["status", units_dir.to_str().unwrap(), "--format", "json"]);
    assert!(
        !output.status.success(),
        "plural failing tests should exit 1"
    );

    let json = parse_stdout_json(&output);
    let units = json["units"].as_array().unwrap();
    assert_eq!(units[0]["status"], "failing");
    assert_eq!(units[0]["reason"], "2 tests failed");
    assert_eq!(units[0]["evidence_at"], "2024-01-02T03:04:05Z");
}

#[test]
fn spec_status_incomplete() {
    let (_temp_dir, units_dir) = setup_apply_discount_unit();
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
    let (_temp_dir, units_dir) = setup_apply_discount_unit();
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
    assert_ne!(
        units[0]["status"], "stale",
        "stale should not win over failing"
    );
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
fn spec_test_accepts_absolute_file_path_from_tmp_symlink_root() {
    if !cargo_available() {
        return;
    }

    let temp_dir = tempfile::Builder::new().tempdir_in("/tmp").unwrap();
    let spec_path = write_pricing_project(temp_dir.path(), true);

    let output = Command::new(bin())
        .current_dir(temp_dir.path())
        .args([
            "test",
            spec_path.to_str().unwrap(),
            "--crate-root",
            temp_dir.path().to_str().unwrap(),
        ])
        .output()
        .expect("failed to run spec");

    assert_output_success(
        "spec test should accept an absolute file path rooted under /tmp",
        &output,
    );
}

#[test]
fn spec_test_file_path_rejects_explicit_nested_output() {
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

    // Single-file runs now build inside an isolated internal output tree.
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
    assert!(
        !output.status.success(),
        "spec test should reject explicit output for a single file\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        String::from_utf8_lossy(&output.stderr)
            .contains("spec test does not accept --output for a single file"),
        "expected explicit-output rejection"
    );

    let after = read_passport(&passport_path);
    assert_eq!(
        after, before,
        "single-file rejection should not rewrite passports"
    );
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

#[test]
fn single_file_test_skips_sibling_molecule_tests() {
    if !cargo_available() {
        return;
    }

    let temp_dir = temp_repo_dir();
    let project_dir = temp_dir.path();
    let spec_path = write_single_file_test_scope_project(project_dir);

    let output = Command::new(bin())
        .current_dir(project_dir)
        .args([
            "test",
            spec_path.to_str().unwrap(),
            "--crate-root",
            project_dir.to_str().unwrap(),
        ])
        .output()
        .expect("failed to run spec");

    assert_output_success(
        "single-file spec test should ignore sibling molecule specs",
        &output,
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let combined = format!("{stdout}\n{stderr}");
    assert!(
        !combined.contains("pricing/missing"),
        "single-file spec test should not load sibling molecule specs\n{combined}"
    );
    assert!(
        !combined.contains("pricing/bad"),
        "single-file spec test should stay scoped to the target unit\n{combined}"
    );

    let target_passport = project_dir.join("units/pricing/a.spec.passport.json");
    let sibling_molecule_passport = project_dir.join("units/pricing/bad.spec.passport.json");
    assert!(
        target_passport.exists(),
        "expected target passport to be written"
    );
    assert!(
        !sibling_molecule_passport.exists(),
        "expected no passport for sibling molecule spec"
    );

    let target_passport = read_passport(&target_passport);
    assert!(
        target_passport.contains("\"status\": \"pass\""),
        "{target_passport}"
    );
}

#[test]
fn single_file_test_with_local_deps_succeeds() {
    if !cargo_available() {
        return;
    }

    let (_temp_dir, ecommerce_dir) = copy_ecommerce_example();
    let output = Command::new(bin())
        .current_dir(&ecommerce_dir)
        .args(["test", "units/pricing/apply_tax.unit.spec"])
        .output()
        .expect("failed to run spec");

    assert_output_success("single_file_test_with_local_deps_succeeds", &output);

    let passport = read_passport(&ecommerce_dir.join("units/pricing/apply_tax.spec.passport.json"));
    assert!(passport.contains("\"status\": \"pass\""), "{passport}");
}

#[test]
fn single_file_test_preserves_unrelated_generated_files() {
    if !cargo_available() {
        return;
    }

    let (_temp_dir, ecommerce_dir) = copy_ecommerce_example();
    let initial_build = run_in(
        &ecommerce_dir,
        &["build", "units", "--output", "src/generated"],
    );
    assert_output_success(
        "initial full build should succeed for preservation test",
        &initial_build,
    );
    let original_mod_rs =
        fs::read_to_string(ecommerce_dir.join("src/generated/pricing/mod.rs")).unwrap();

    let output = run_in(
        &ecommerce_dir,
        &[
            "test",
            "units/pricing/pricing_quote.unit.spec",
            "--crate-root",
            ".",
        ],
    );
    assert_output_success(
        "single-file spec test should preserve unrelated generated files",
        &output,
    );

    assert!(
        ecommerce_dir
            .join("src/generated/pricing/calculate_total.rs")
            .exists(),
        "single-file test should not prune unrelated generated siblings"
    );
    assert!(
        ecommerce_dir
            .join("src/generated/pricing/molecule_tests.rs")
            .exists(),
        "single-file test should not prune molecule_tests.rs from the shared output tree"
    );
    let updated_mod_rs =
        fs::read_to_string(ecommerce_dir.join("src/generated/pricing/mod.rs")).unwrap();
    assert_eq!(updated_mod_rs, original_mod_rs);
}

#[test]
fn single_file_test_rejects_explicit_output() {
    if !cargo_available() {
        return;
    }

    let (_temp_dir, ecommerce_dir) = copy_ecommerce_example();
    let output = run_in(
        &ecommerce_dir,
        &[
            "test",
            "units/pricing/pricing_quote.unit.spec",
            "--output",
            "src/generated",
            "--crate-root",
            ".",
        ],
    );
    assert!(
        !output.status.success(),
        "single_file_test_rejects_explicit_output\n\nstdout:\n{}\n\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        String::from_utf8_lossy(&output.stderr)
            .contains("spec test does not accept --output for a single file"),
        "expected explicit-output rejection"
    );
}

#[test]
fn single_file_test_failure_writes_failing_passport() {
    if !cargo_available() {
        return;
    }

    let (_temp_dir, ecommerce_dir) = copy_ecommerce_example();
    let spec_path = ecommerce_dir.join("units/pricing/apply_tax.unit.spec");
    let contents = fs::read_to_string(&spec_path).unwrap();
    fs::write(
        &spec_path,
        contents.replace(
            "apply_tax(Decimal::new(10000, 2), Decimal::new(725, 4)) == Decimal::new(10725, 2)",
            "apply_tax(Decimal::new(10000, 2), Decimal::new(725, 4)) == Decimal::new(99999, 2)",
        ),
    )
    .unwrap();

    let output = Command::new(bin())
        .current_dir(&ecommerce_dir)
        .args(["test", "units/pricing/apply_tax.unit.spec"])
        .output()
        .expect("failed to run spec");

    assert!(
        !output.status.success(),
        "expected failing local test to exit non-zero"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !stderr.contains("matched 0 tests"),
        "failing single-file test should not be misclassified as zero matches\n{stderr}"
    );

    let passport = read_passport(&ecommerce_dir.join("units/pricing/apply_tax.spec.passport.json"));
    assert!(passport.contains("\"status\": \"fail\""), "{passport}");

    let status_output = run_in(
        &ecommerce_dir,
        &[
            "status",
            "units/pricing/apply_tax.unit.spec",
            "--format",
            "json",
        ],
    );
    assert!(
        !status_output.status.success(),
        "failing unit status should exit non-zero"
    );
    let json = parse_stdout_json(&status_output);
    assert_eq!(json["units"][0]["status"], "failing");
}

#[test]
fn directory_test_still_loads_sibling_molecule_tests() {
    if !cargo_available() {
        return;
    }

    let temp_dir = temp_repo_dir();
    let project_dir = temp_dir.path();
    let units_dir = project_dir.join("units");
    write_single_file_test_scope_project(project_dir);

    let output = Command::new(bin())
        .current_dir(project_dir)
        .args([
            "test",
            units_dir.to_str().unwrap(),
            "--output",
            "src/generated",
            "--crate-root",
            project_dir.to_str().unwrap(),
        ])
        .output()
        .expect("failed to run spec");

    assert!(
        !output.status.success(),
        "directory spec test should still load sibling molecule specs"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("pricing/missing") || stderr.contains("pricing/bad"),
        "expected directory spec test to surface the invalid molecule spec\n{stderr}"
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

#[test]
fn spec_test_resolves_relative_pipeline_crate_root_from_spec_toml() {
    if !cargo_available() {
        return;
    }

    let (_temp_dir, shared_spec_dir, _shared_crate_dir) = setup_detached_shared_example();
    let output = run_in(
        shared_spec_dir
            .parent()
            .expect("shared-spec fixture should have a parent"),
        &[
            "test",
            "shared-spec/units",
            "--output",
            "shared-crate/src/generated",
        ],
    );

    assert_output_success(
        "shared example should resolve relative [pipeline].crate_root and pass cargo test",
        &output,
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("spec: running cargo build in")
            && stderr.contains("shared-crate")
            && stderr.contains("spec: running cargo test in"),
        "{stderr}"
    );
    assert!(!stderr.contains("failed to spawn cargo"), "{stderr}");
}

// ──────────────────────────────────────────────────────────────────────────────
// M7: molecule test (.test.spec) integration tests
// ──────────────────────────────────────────────────────────────────────────────

/// Helper: write a minimal .test.spec file
fn write_molecule_test_spec(units_dir: &Path, relative_path: &str, id: &str, covers: &[&str]) {
    let covers_yaml = if covers.is_empty() {
        "covers: []".to_string()
    } else {
        let items: Vec<String> = covers.iter().map(|c| format!("  - {c}")).collect();
        format!("covers:\n{}", items.join("\n"))
    };
    let content = format!(
        r#"id: {id}
spec_version: "0.3.0"
intent:
  why: Test molecule for {id}.
{covers_yaml}
body:
  rust: |
    {{
        assert!(true);
    }}
"#
    );
    write_spec(units_dir, relative_path, &content);
}

fn write_molecule_test_spec_with_imports(
    units_dir: &Path,
    relative_path: &str,
    id: &str,
    covers: &[&str],
    imports: &[&str],
) {
    let covers_yaml = if covers.is_empty() {
        "covers: []".to_string()
    } else {
        let items: Vec<String> = covers.iter().map(|c| format!("  - {c}")).collect();
        format!("covers:\n{}", items.join("\n"))
    };
    let imports_yaml = if imports.is_empty() {
        "imports: []".to_string()
    } else {
        let items: Vec<String> = imports
            .iter()
            .map(|import| format!("  - {import}"))
            .collect();
        format!("imports:\n{}", items.join("\n"))
    };
    let content = format!(
        r#"id: {id}
spec_version: "0.3.0"
intent:
  why: Test molecule for {id}.
{covers_yaml}
{imports_yaml}
body:
  rust: |
    {{
        assert!(true);
    }}
"#
    );
    write_spec(units_dir, relative_path, &content);
}

fn write_two_unit_molecule_fixture(units_dir: &Path) -> PathBuf {
    write_spec(
        units_dir,
        "pricing/a.unit.spec",
        r#"
id: pricing/a
kind: function
intent:
  why: Unit A.
spec_version: "0.3.0"
body:
  rust: |
    { }
"#,
    );
    write_spec(
        units_dir,
        "pricing/b.unit.spec",
        r#"
id: pricing/b
kind: function
intent:
  why: Unit B.
spec_version: "0.3.0"
body:
  rust: |
    { }
"#,
    );
    write_molecule_test_spec(
        units_dir,
        "pricing/ab.test.spec",
        "pricing/ab",
        &["pricing/a", "pricing/b"],
    );

    units_dir.join("pricing/a.unit.spec")
}

fn write_molecule_test_target_unit(units_dir: &Path) {
    write_spec(
        units_dir,
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
}

fn write_indexed_unsafe_molecule_test(units_dir: &Path) {
    let content = r#"id: pricing/unsafe_test
spec_version: "0.3.0"
intent:
  why: Unsafe test that should be rejected.
covers:
  - pricing/apply_discount
body:
  rust: |
    {
        let _x = [unsafe { std::mem::zeroed::<u8>() }][0];
    }
"#;
    write_spec(units_dir, "pricing/unsafe_test.test.spec", content);
}

#[test]
fn valid_molecule_test_validates() {
    let temp_dir = temp_repo_dir();
    let units_dir = temp_dir.path().join("units");

    // Write a unit spec that the molecule test covers
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

    // Write a valid molecule test
    write_molecule_test_spec(
        &units_dir,
        "pricing/discount_test.test.spec",
        "pricing/discount_test",
        &["pricing/apply_discount"],
    );

    let output = run(&["validate", units_dir.to_str().unwrap()]);
    assert_output_success("valid_molecule_test_validates", &output);
}

#[test]
fn molecule_test_unknown_covers_id_fails() {
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

    // Molecule test covers a unit that doesn't exist
    write_molecule_test_spec(
        &units_dir,
        "pricing/bad_test.test.spec",
        "pricing/bad_test",
        &["pricing/nonexistent_unit"],
    );

    let output = run(&["validate", units_dir.to_str().unwrap()]);
    assert!(
        !output.status.success(),
        "validate should fail when covers references unknown unit"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stderr.contains("nonexistent_unit") || stdout.contains("nonexistent_unit"),
        "error should mention the missing unit id\nstdout: {stdout}\nstderr: {stderr}"
    );
}

#[test]
fn molecule_test_generates_molecule_tests_rs() {
    let temp_dir = temp_repo_dir();
    let units_dir = temp_dir.path().join("units");
    let output_dir = temp_dir.path().join("generated");

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

    write_molecule_test_spec(
        &units_dir,
        "pricing/discount_test.test.spec",
        "pricing/discount_test",
        &["pricing/apply_discount"],
    );

    // Create marker so generate doesn't complain about non-empty dir
    fs::create_dir_all(&output_dir).unwrap();
    fs::write(output_dir.join(".spec-generated"), "").unwrap();

    let output = run(&[
        "generate",
        units_dir.to_str().unwrap(),
        "--output",
        output_dir.to_str().unwrap(),
    ]);
    assert_output_success("molecule_test_generates_molecule_tests_rs", &output);

    let molecule_tests_rs = output_dir.join("pricing/molecule_tests.rs");
    assert!(
        molecule_tests_rs.exists(),
        "pricing/molecule_tests.rs should be generated: {}",
        molecule_tests_rs.display()
    );

    let mod_rs = output_dir.join("pricing/mod.rs");
    assert!(mod_rs.exists(), "pricing/mod.rs should exist");
    let mod_content = fs::read_to_string(&mod_rs).unwrap();
    assert!(
        mod_content.contains("pub mod molecule_tests;"),
        "pricing/mod.rs should declare molecule_tests module\ncontent: {mod_content}"
    );

    let root_mod = output_dir.join("mod.rs");
    assert!(root_mod.exists(), "root mod.rs should exist");
    let root_mod_content = fs::read_to_string(&root_mod).unwrap();
    assert!(
        root_mod_content.contains("pub mod pricing;"),
        "root mod.rs should declare pricing module\ncontent: {root_mod_content}"
    );
}

#[test]
fn molecule_test_with_explicit_imports_keeps_transitive_cover_semantic_only() {
    let temp_dir = temp_repo_dir();
    let units_dir = temp_dir.path().join("units");
    let output_dir = temp_dir.path().join("generated");

    write_spec(
        &units_dir,
        "money/round.unit.spec",
        r#"
id: money/round
kind: function
intent:
  why: Round money values.
spec_version: "0.3.0"
body:
  rust: |
    { value }
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
imports:
  - rust_decimal::Decimal
body:
  rust: |
    { subtotal }
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
imports:
  - rust_decimal::Decimal
body:
  rust: |
    { subtotal }
"#,
    );
    write_molecule_test_spec_with_imports(
        &units_dir,
        "pricing/discount_plus_tax.test.spec",
        "pricing/discount_plus_tax",
        &["pricing/apply_discount", "pricing/apply_tax", "money/round"],
        &[
            "rust_decimal::Decimal",
            "crate::pricing::apply_discount::apply_discount",
            "crate::pricing::apply_tax::apply_tax",
        ],
    );
    fs::create_dir_all(&output_dir).unwrap();
    fs::write(output_dir.join(".spec-generated"), "").unwrap();

    let output = run(&[
        "generate",
        units_dir.to_str().unwrap(),
        "--output",
        output_dir.to_str().unwrap(),
    ]);
    assert_output_success(
        "molecule test with explicit imports should generate successfully",
        &output,
    );

    let generated = fs::read_to_string(output_dir.join("pricing/molecule_tests.rs")).unwrap();
    assert!(
        generated.contains("use crate::pricing::apply_discount::apply_discount;"),
        "expected explicit callable import\n{generated}"
    );
    assert!(
        generated.contains("use crate::pricing::apply_tax::apply_tax;"),
        "expected explicit callable import\n{generated}"
    );
    assert!(
        !generated.contains("use crate::money::round::round;"),
        "semantic-only transitive cover should not become an import\n{generated}"
    );
}

#[test]
fn molecule_only_namespace_generates_module_tree() {
    let temp_dir = temp_repo_dir();
    let units_dir = temp_dir.path().join("units");
    let output_dir = temp_dir.path().join("generated");

    write_molecule_test_target_unit(&units_dir);
    write_molecule_test_spec(
        &units_dir,
        "qa/sample.test.spec",
        "qa/sample",
        &["pricing/apply_discount"],
    );

    fs::create_dir_all(&output_dir).unwrap();
    fs::write(output_dir.join(".spec-generated"), "").unwrap();

    let output = run(&[
        "generate",
        units_dir.to_str().unwrap(),
        "--output",
        output_dir.to_str().unwrap(),
    ]);
    assert_output_success("molecule_only_namespace_generates_module_tree", &output);

    let root_mod_content = fs::read_to_string(output_dir.join("mod.rs")).unwrap();
    assert!(
        root_mod_content.contains("pub mod qa;"),
        "root mod.rs should declare qa module\ncontent: {root_mod_content}"
    );

    let qa_mod_content = fs::read_to_string(output_dir.join("qa/mod.rs")).unwrap();
    assert!(
        qa_mod_content.contains("pub mod molecule_tests;"),
        "qa/mod.rs should declare molecule_tests module\ncontent: {qa_mod_content}"
    );

    assert!(
        output_dir.join("qa/molecule_tests.rs").exists(),
        "qa/molecule_tests.rs should be generated"
    );
}

#[test]
fn nested_molecule_only_namespace_generates_parent_modules() {
    let temp_dir = temp_repo_dir();
    let units_dir = temp_dir.path().join("units");
    let output_dir = temp_dir.path().join("generated");

    write_molecule_test_target_unit(&units_dir);
    write_molecule_test_spec(
        &units_dir,
        "qa/sub/sample.test.spec",
        "qa/sub/sample",
        &["pricing/apply_discount"],
    );

    fs::create_dir_all(&output_dir).unwrap();
    fs::write(output_dir.join(".spec-generated"), "").unwrap();

    let output = run(&[
        "generate",
        units_dir.to_str().unwrap(),
        "--output",
        output_dir.to_str().unwrap(),
    ]);
    assert_output_success(
        "nested_molecule_only_namespace_generates_parent_modules",
        &output,
    );

    let root_mod_content = fs::read_to_string(output_dir.join("mod.rs")).unwrap();
    assert!(
        root_mod_content.contains("pub mod qa;"),
        "root mod.rs should declare qa module\ncontent: {root_mod_content}"
    );

    let qa_mod_content = fs::read_to_string(output_dir.join("qa/mod.rs")).unwrap();
    assert!(
        qa_mod_content.contains("pub mod sub;"),
        "qa/mod.rs should declare sub module\ncontent: {qa_mod_content}"
    );
    assert!(
        !qa_mod_content.contains("pub mod molecule_tests;"),
        "qa/mod.rs should not declare molecule_tests directly\ncontent: {qa_mod_content}"
    );

    let nested_mod_content = fs::read_to_string(output_dir.join("qa/sub/mod.rs")).unwrap();
    assert!(
        nested_mod_content.contains("pub mod molecule_tests;"),
        "qa/sub/mod.rs should declare molecule_tests module\ncontent: {nested_mod_content}"
    );

    assert!(
        output_dir.join("qa/sub/molecule_tests.rs").exists(),
        "qa/sub/molecule_tests.rs should be generated"
    );
}

#[test]
fn generate_removes_stale_molecule_module_declarations() {
    let temp_dir = temp_repo_dir();
    let units_dir = temp_dir.path().join("units");
    let output_dir = temp_dir.path().join("generated");

    write_molecule_test_target_unit(&units_dir);
    let qa_test_path = units_dir.join("qa/sample.test.spec");
    write_molecule_test_spec(
        &units_dir,
        "qa/sample.test.spec",
        "qa/sample",
        &["pricing/apply_discount"],
    );

    fs::create_dir_all(&output_dir).unwrap();
    fs::write(output_dir.join(".spec-generated"), "").unwrap();

    let first_output = run(&[
        "generate",
        units_dir.to_str().unwrap(),
        "--output",
        output_dir.to_str().unwrap(),
    ]);
    assert_output_success(
        "generate_removes_stale_molecule_module_declarations first pass",
        &first_output,
    );

    fs::remove_file(&qa_test_path).unwrap();

    let second_output = run(&[
        "generate",
        units_dir.to_str().unwrap(),
        "--output",
        output_dir.to_str().unwrap(),
    ]);
    assert_output_success(
        "generate_removes_stale_molecule_module_declarations second pass",
        &second_output,
    );

    let root_mod_content = fs::read_to_string(output_dir.join("mod.rs")).unwrap();
    assert!(
        !root_mod_content.contains("pub mod qa;"),
        "root mod.rs should not retain stale qa module\ncontent: {root_mod_content}"
    );

    assert!(
        !output_dir.join("qa/molecule_tests.rs").exists(),
        "qa/molecule_tests.rs should be removed after molecule test deletion"
    );
    assert!(
        !output_dir.join("qa/mod.rs").exists(),
        "qa/mod.rs should be removed after molecule test deletion"
    );
    assert!(
        !output_dir.join("qa").exists(),
        "empty qa directory should be removed after molecule test deletion"
    );
}

#[test]
fn export_includes_molecule_tests_and_covers_edges() {
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

    write_molecule_test_spec(
        &units_dir,
        "pricing/discount_test.test.spec",
        "pricing/discount_test",
        &["pricing/apply_discount"],
    );

    let output = run(&["export", units_dir.to_str().unwrap()]);
    assert_output_success("export_includes_molecule_tests_and_covers_edges", &output);

    let bundle: Value = serde_json::from_slice(&output.stdout).unwrap();

    let molecule_tests = bundle["molecule_tests"].as_array().unwrap();
    assert!(
        !molecule_tests.is_empty(),
        "molecule_tests array should be non-empty"
    );

    let covers_edges: Vec<&Value> = bundle["graph"]["edges"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|e| e["kind"] == "covers")
        .collect();
    assert!(
        !covers_edges.is_empty(),
        "graph.edges should have at least one covers edge"
    );
}

#[test]
fn export_preserves_molecule_test_imports_surface() {
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

    write_molecule_test_spec_with_imports(
        &units_dir,
        "pricing/discount_test.test.spec",
        "pricing/discount_test",
        &["pricing/apply_discount"],
        &[
            "rust_decimal::Decimal",
            "crate::pricing::apply_discount::apply_discount",
        ],
    );
    write_molecule_test_spec_with_imports(
        &units_dir,
        "pricing/empty_imports.test.spec",
        "pricing/empty_imports",
        &["pricing/apply_discount"],
        &[],
    );
    write_molecule_test_spec(
        &units_dir,
        "pricing/implicit_imports.test.spec",
        "pricing/implicit_imports",
        &["pricing/apply_discount"],
    );

    let output = run(&["export", units_dir.to_str().unwrap()]);
    assert_output_success("export_preserves_molecule_test_imports_surface", &output);

    let bundle: Value = serde_json::from_slice(&output.stdout).unwrap();
    let molecule_tests = bundle["molecule_tests"].as_array().unwrap();

    let explicit = molecule_tests
        .iter()
        .find(|test| test["id"] == "pricing/discount_test")
        .unwrap();
    assert_eq!(
        explicit["imports"],
        serde_json::json!([
            "rust_decimal::Decimal",
            "crate::pricing::apply_discount::apply_discount"
        ])
    );

    let empty = molecule_tests
        .iter()
        .find(|test| test["id"] == "pricing/empty_imports")
        .unwrap();
    assert_eq!(empty["imports"], serde_json::json!([]));

    let implicit = molecule_tests
        .iter()
        .find(|test| test["id"] == "pricing/implicit_imports")
        .unwrap();
    assert!(
        implicit.get("imports").is_none(),
        "omitted imports should stay omitted: {implicit}"
    );
}

#[test]
fn export_ecommerce_example_includes_authored_molecule_test_imports() {
    let (_temp_dir, ecommerce_dir) = copy_ecommerce_example();

    let output = run_in(&ecommerce_dir, &["export", "units"]);
    assert_output_success(
        "export_ecommerce_example_includes_authored_molecule_test_imports",
        &output,
    );

    let bundle = parse_stdout_json(&output);
    let molecule_tests = bundle["molecule_tests"].as_array().unwrap();

    let checkout_flow = molecule_tests
        .iter()
        .find(|test| test["id"] == "pricing/checkout_flow")
        .unwrap();
    assert_eq!(
        checkout_flow["imports"],
        serde_json::json!([
            "rust_decimal::Decimal",
            "crate::pricing::apply_discount::apply_discount",
            "crate::pricing::calculate_total::calculate_total",
            "crate::pricing::pricing_quote::PricingQuote"
        ])
    );

    let discount_plus_tax = molecule_tests
        .iter()
        .find(|test| test["id"] == "pricing/discount_plus_tax")
        .unwrap();
    assert_eq!(
        discount_plus_tax["imports"],
        serde_json::json!([
            "rust_decimal::Decimal",
            "crate::pricing::apply_discount::apply_discount",
            "crate::pricing::apply_tax::apply_tax"
        ])
    );
}

#[test]
fn single_file_validate_skips_sibling_molecule_tests() {
    let temp_dir = temp_repo_dir();
    let units_dir = temp_dir.path().join("units");
    let unit_path = write_two_unit_molecule_fixture(&units_dir);

    let output = run(&["validate", unit_path.to_str().unwrap()]);
    assert_output_success("single_file_validate_skips_sibling_molecule_tests", &output);

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !stdout.contains("pricing/b"),
        "stdout should stay scoped\n{stdout}"
    );
    assert!(
        !stderr.contains("pricing/b"),
        "stderr should stay scoped\n{stderr}"
    );
}

#[test]
fn single_file_validate_with_local_deps_succeeds() {
    let (_temp_dir, ecommerce_dir) = copy_ecommerce_example();

    let output = run_in(
        &ecommerce_dir,
        &[
            "validate",
            "units/pricing/apply_tax.unit.spec",
            "--format",
            "json",
        ],
    );
    assert_output_success("single_file_validate_with_local_deps_succeeds", &output);

    let json = parse_stdout_json(&output);
    assert_eq!(json["status"], "valid");
    assert_eq!(json["errors"], serde_json::json!([]));
}

#[test]
fn single_file_status_with_local_deps_stays_scoped() {
    let (_temp_dir, ecommerce_dir) = copy_ecommerce_example();

    let output = run_in(
        &ecommerce_dir,
        &[
            "status",
            "units/pricing/apply_tax.unit.spec",
            "--format",
            "json",
        ],
    );
    assert!(
        !output.status.success(),
        "single_file_status_with_local_deps_stays_scoped\n\nstdout:\n{}\n\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let json = parse_stdout_json(&output);
    let units = json["units"].as_array().unwrap();
    assert_eq!(units.len(), 1, "{json}");
    assert_eq!(units[0]["id"], "pricing/apply_tax");
    assert_eq!(units[0]["status"], "untested");
    assert_eq!(units[0]["errors"], serde_json::json!([]));
}

#[test]
fn single_file_generate_is_rejected() {
    let temp_dir = temp_repo_dir();
    let units_dir = temp_dir.path().join("units");
    let output_dir = temp_dir.path().join("generated");
    let unit_path = write_two_unit_molecule_fixture(&units_dir);

    fs::create_dir_all(&output_dir).unwrap();
    fs::write(output_dir.join(".spec-generated"), "").unwrap();

    let output = run(&[
        "generate",
        unit_path.to_str().unwrap(),
        "--output",
        output_dir.to_str().unwrap(),
    ]);
    assert!(
        !output.status.success(),
        "single_file_generate_is_rejected\n\nstdout:\n{}\n\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    assert!(
        String::from_utf8_lossy(&output.stderr).contains("spec generate requires a directory path"),
        "expected directory-path error"
    );
    assert!(!output_dir.join("pricing/a.rs").exists());
    assert!(!output_dir.join("pricing/molecule_tests.rs").exists());
}

#[test]
fn single_file_generate_with_local_deps_is_rejected() {
    let (_temp_dir, ecommerce_dir) = copy_ecommerce_example();
    let output_dir = ecommerce_dir.join("generated-single");

    let output = run_in(
        &ecommerce_dir,
        &[
            "generate",
            "units/pricing/apply_tax.unit.spec",
            "--output",
            output_dir.to_str().unwrap(),
        ],
    );
    assert!(
        !output.status.success(),
        "single_file_generate_with_local_deps_is_rejected\n\nstdout:\n{}\n\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    assert!(
        String::from_utf8_lossy(&output.stderr).contains("spec generate requires a directory path"),
        "expected directory-path error"
    );
    assert!(!output_dir.join("pricing/apply_tax.rs").exists());
    assert!(!output_dir.join("money/round.rs").exists());
    assert!(!output_dir.join("pricing/pricing_quote.rs").exists());
}

#[test]
fn single_file_generate_does_not_rewrite_existing_generated_tree() {
    if !cargo_available() {
        return;
    }

    let (_temp_dir, ecommerce_dir) = copy_ecommerce_example();
    let initial_build = run_in(
        &ecommerce_dir,
        &["build", "units", "--output", "src/generated"],
    );
    assert_output_success(
        "initial full build should succeed for generate preservation test",
        &initial_build,
    );
    let original_mod_rs =
        fs::read_to_string(ecommerce_dir.join("src/generated/pricing/mod.rs")).unwrap();

    let output = run_in(
        &ecommerce_dir,
        &[
            "generate",
            "units/pricing/pricing_quote.unit.spec",
            "--output",
            "src/generated",
        ],
    );
    assert!(
        !output.status.success(),
        "single-file generate should be rejected\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    assert!(
        ecommerce_dir
            .join("src/generated/pricing/calculate_total.rs")
            .exists(),
        "single-file generate should not prune unrelated generated siblings"
    );
    assert!(
        ecommerce_dir
            .join("src/generated/pricing/molecule_tests.rs")
            .exists(),
        "single-file generate should not prune molecule_tests.rs from the shared output tree"
    );
    let updated_mod_rs =
        fs::read_to_string(ecommerce_dir.join("src/generated/pricing/mod.rs")).unwrap();
    assert_eq!(updated_mod_rs, original_mod_rs);
}

#[test]
fn single_file_generate_from_nested_units_subdir_is_rejected() {
    let (_temp_dir, ecommerce_dir) = copy_ecommerce_example();
    let pricing_dir = ecommerce_dir.join("units/pricing");

    let output = run_in(
        &pricing_dir,
        &[
            "generate",
            "apply_tax.unit.spec",
            "--output",
            "../../src/generated",
        ],
    );
    assert!(
        !output.status.success(),
        "single-file generate from nested units dir should fail\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    assert!(
        String::from_utf8_lossy(&output.stderr).contains("spec generate requires a directory path"),
        "expected directory-path error"
    );
}

#[test]
fn single_file_test_from_nested_units_subdir_finds_ancestor_crate_root() {
    if !cargo_available() {
        return;
    }

    let (_temp_dir, ecommerce_dir) = copy_ecommerce_example();
    let pricing_dir = ecommerce_dir.join("units/pricing");

    let output = run_in(&pricing_dir, &["test", "apply_tax.unit.spec"]);
    assert_output_success(
        "single-file test from nested units dir should find ancestor Cargo.toml",
        &output,
    );

    let passport = read_passport(&ecommerce_dir.join("units/pricing/apply_tax.spec.passport.json"));
    assert!(passport.contains("\"status\": \"pass\""), "{passport}");
}

#[test]
fn single_file_export_skips_sibling_molecule_tests() {
    let temp_dir = temp_repo_dir();
    let units_dir = temp_dir.path().join("units");
    let unit_path = write_two_unit_molecule_fixture(&units_dir);

    let output = run(&["export", unit_path.to_str().unwrap()]);
    assert_output_success("single_file_export_skips_sibling_molecule_tests", &output);

    let bundle: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(bundle["units"].as_array().unwrap().len(), 1);
    assert_eq!(bundle["molecule_tests"].as_array().unwrap().len(), 0);

    let covers_edges: Vec<&Value> = bundle["graph"]["edges"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|e| e["kind"] == "covers")
        .collect();
    assert!(
        covers_edges.is_empty(),
        "single-file export should not include sibling covers edges"
    );
}

#[test]
fn single_file_export_with_local_deps_succeeds() {
    let (_temp_dir, ecommerce_dir) = copy_ecommerce_example();

    let output = run_in(
        &ecommerce_dir,
        &["export", "units/pricing/apply_tax.unit.spec"],
    );
    assert_output_success("single_file_export_with_local_deps_succeeds", &output);

    let bundle: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(bundle["units"].as_array().unwrap().len(), 1);
    assert_eq!(bundle["units"][0]["id"], "pricing/apply_tax");
    assert_eq!(bundle["molecule_tests"].as_array().unwrap().len(), 0);
}

#[test]
fn duplicate_molecule_test_id_rejected() {
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

    // Two test.spec files with the same id
    write_molecule_test_spec(
        &units_dir,
        "pricing/dupe_test_a.test.spec",
        "pricing/dupe_test",
        &["pricing/apply_discount"],
    );
    write_molecule_test_spec(
        &units_dir,
        "pricing/dupe_test_b.test.spec",
        "pricing/dupe_test",
        &["pricing/apply_discount"],
    );

    let output = run(&["validate", units_dir.to_str().unwrap()]);
    assert!(
        !output.status.success(),
        "validate should fail on duplicate molecule test IDs"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stderr.contains("dupe_test") || stdout.contains("dupe_test"),
        "error should mention the duplicate id\nstdout: {stdout}\nstderr: {stderr}"
    );
}

#[test]
fn duplicate_molecule_test_id_json_uses_stable_contract_code() {
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

    let file_a = units_dir.join("pricing/dupe_test_a.test.spec");
    let file_b = units_dir.join("pricing/dupe_test_b.test.spec");

    write_molecule_test_spec(
        &units_dir,
        "pricing/dupe_test_a.test.spec",
        "pricing/dupe_test",
        &["pricing/apply_discount"],
    );
    write_molecule_test_spec(
        &units_dir,
        "pricing/dupe_test_b.test.spec",
        "pricing/dupe_test",
        &["pricing/apply_discount"],
    );

    let output = run(&["validate", units_dir.to_str().unwrap(), "--format", "json"]);
    assert!(
        !output.status.success(),
        "validate should fail on duplicate molecule test IDs in JSON mode"
    );
    assert!(
        output.stderr.is_empty(),
        "expected no stderr output, got: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let json = parse_stdout_json(&output);
    assert_eq!(json["status"], "invalid");
    let errors = json["errors"].as_array().unwrap();
    assert_eq!(errors.len(), 1, "expected exactly one duplicate-id error");

    let error = &errors[0];
    assert_eq!(error["code"], "SPEC_DUPLICATE_MOLECULE_ID");
    assert_eq!(error["id"], "pricing/dupe_test");

    let path = error["path"].as_str().unwrap();
    let path2 = error["path2"].as_str().unwrap();
    let expected_a = file_a.to_str().unwrap();
    let expected_b = file_b.to_str().unwrap();

    assert!(
        (path == expected_a && path2 == expected_b) || (path == expected_b && path2 == expected_a),
        "duplicate paths should identify both files\npath: {path}\npath2: {path2}\nexpected_a: {expected_a}\nexpected_b: {expected_b}"
    );
}

#[test]
fn molecule_covers_collision_json_uses_stable_contract_code() {
    let temp_dir = temp_repo_dir();
    let units_dir = temp_dir.path().join("units");

    write_spec(
        &units_dir,
        "money/round.unit.spec",
        r#"
id: money/round
kind: function
intent:
  why: Round money values.
spec_version: "0.3.0"
body:
  rust: |
    { value }
"#,
    );
    write_spec(
        &units_dir,
        "utils/round.unit.spec",
        r#"
id: utils/round
kind: function
intent:
  why: Round utility values.
spec_version: "0.3.0"
body:
  rust: |
    { value }
"#,
    );
    write_molecule_test_spec(
        &units_dir,
        "pricing/rounding_flow.test.spec",
        "pricing/rounding_flow",
        &["money/round", "utils/round"],
    );

    let output = run(&["validate", units_dir.to_str().unwrap(), "--format", "json"]);
    assert!(
        !output.status.success(),
        "validate should fail on molecule covers collisions in JSON mode"
    );
    assert!(
        output.stderr.is_empty(),
        "expected no stderr output, got: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let json = parse_stdout_json(&output);
    assert_eq!(json["status"], "invalid");
    let warnings = json["warnings"].as_array().unwrap();
    assert_eq!(warnings.len(), 1, "expected one deprecation warning");
    assert_eq!(
        warnings[0]["code"],
        "SPEC_MOLECULE_IMPLICIT_IMPORTS_DEPRECATED"
    );
    assert_eq!(warnings[0]["id"], "pricing/rounding_flow");
    let errors = json["errors"].as_array().unwrap();
    assert_eq!(
        errors.len(),
        1,
        "expected exactly one covers-collision error"
    );

    let error = &errors[0];
    assert_eq!(error["code"], "SPEC_MOLECULE_COVERS_COLLISION");
    assert_eq!(error["id"], "pricing/rounding_flow");
    assert_eq!(
        error["path"],
        units_dir
            .join("pricing/rounding_flow.test.spec")
            .to_str()
            .unwrap()
    );
    assert_eq!(error["dep"], "money/round");
    assert_eq!(error["path2"], "utils/round");
    assert_eq!(error["value"], "round");
}

#[test]
fn generate_rejects_molecule_covers_collision_before_rust_codegen() {
    let temp_dir = temp_repo_dir();
    let units_dir = temp_dir.path().join("units");
    let output_dir = temp_dir.path().join("generated");

    write_spec(
        &units_dir,
        "money/round.unit.spec",
        r#"
id: money/round
kind: function
intent:
  why: Round money values.
spec_version: "0.3.0"
body:
  rust: |
    { value }
"#,
    );
    write_spec(
        &units_dir,
        "utils/round.unit.spec",
        r#"
id: utils/round
kind: function
intent:
  why: Round utility values.
spec_version: "0.3.0"
body:
  rust: |
    { value }
"#,
    );
    write_molecule_test_spec(
        &units_dir,
        "pricing/rounding_flow.test.spec",
        "pricing/rounding_flow",
        &["money/round", "utils/round"],
    );

    fs::create_dir_all(&output_dir).unwrap();
    fs::write(output_dir.join(".spec-generated"), "").unwrap();

    let output = run(&[
        "generate",
        units_dir.to_str().unwrap(),
        "--output",
        output_dir.to_str().unwrap(),
    ]);
    assert!(
        !output.status.success(),
        "generate should fail before writing duplicate Rust imports"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("money/round"),
        "error should mention first conflicting cover\nstderr: {stderr}"
    );
    assert!(
        stderr.contains("utils/round"),
        "error should mention second conflicting cover\nstderr: {stderr}"
    );
    assert!(
        stderr.contains("round"),
        "error should mention collided callable name\nstderr: {stderr}"
    );
    assert!(
        !output_dir.join("pricing/molecule_tests.rs").exists(),
        "generate should fail before molecule_tests.rs is written"
    );
}

#[test]
fn empty_covers_is_warning_not_error() {
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

    // Molecule test with no covers (should warn, not error)
    write_molecule_test_spec(
        &units_dir,
        "pricing/empty_covers_test.test.spec",
        "pricing/empty_covers_test",
        &[], // empty covers
    );

    let output = run(&["validate", units_dir.to_str().unwrap()]);
    assert_output_success("empty_covers_is_warning_not_error", &output);

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("no covered units") || stderr.contains("empty_covers_test"),
        "should emit a warning about no covered units\nstderr: {stderr}"
    );
}

#[test]
fn validate_single_test_spec_file_gives_directed_error() {
    let temp_dir = temp_repo_dir();
    let units_dir = temp_dir.path().join("units");

    write_molecule_test_spec(
        &units_dir,
        "pricing/checkout_flow.test.spec",
        "pricing/checkout_flow",
        &[],
    );

    let test_spec_path = units_dir.join("pricing/checkout_flow.test.spec");
    let output = run(&["validate", test_spec_path.to_str().unwrap()]);

    // Should fail — single .test.spec files are not valid input to validate
    assert!(
        !output.status.success(),
        "validate of single .test.spec file should fail"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("containing directory") || stderr.contains(".unit.spec"),
        "error should guide user to use directory path\nstderr: {stderr}"
    );
}

#[test]
fn molecule_body_with_unsafe_is_rejected() {
    let temp_dir = temp_repo_dir();
    let units_dir = temp_dir.path().join("units");

    write_molecule_test_target_unit(&units_dir);
    write_indexed_unsafe_molecule_test(&units_dir);

    let output = run(&["validate", units_dir.to_str().unwrap()]);
    assert!(
        !output.status.success(),
        "validate should fail for molecule test with unsafe body"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("unsafe") || stderr.contains("SPEC_MOLECULE_BODY_CONTAINS_UNSAFE"),
        "error should mention unsafe\nstderr: {stderr}"
    );
}

#[test]
fn molecule_body_with_unsafe_is_rejected_in_json_output() {
    let temp_dir = temp_repo_dir();
    let units_dir = temp_dir.path().join("units");

    write_molecule_test_target_unit(&units_dir);
    write_indexed_unsafe_molecule_test(&units_dir);

    let output = run(&["validate", units_dir.to_str().unwrap(), "--format", "json"]);
    assert!(
        !output.status.success(),
        "validate --format json should fail for molecule test with unsafe body"
    );
    assert!(
        output.stderr.is_empty(),
        "expected no stderr output, got: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let json = parse_stdout_json(&output);
    assert_eq!(json["status"], "invalid");
    let errors = json["errors"].as_array().unwrap();
    assert!(
        errors
            .iter()
            .any(|error| error["code"] == "SPEC_MOLECULE_BODY_CONTAINS_UNSAFE"),
        "expected SPEC_MOLECULE_BODY_CONTAINS_UNSAFE in errors: {errors:?}"
    );
}

#[test]
fn generate_rejects_molecule_body_with_nested_unsafe_expr() {
    let temp_dir = temp_repo_dir();
    let units_dir = temp_dir.path().join("units");
    let output_dir = temp_dir.path().join("generated");

    write_molecule_test_target_unit(&units_dir);
    write_indexed_unsafe_molecule_test(&units_dir);

    fs::create_dir_all(&output_dir).unwrap();
    fs::write(output_dir.join(".spec-generated"), "").unwrap();

    let output = run(&[
        "generate",
        units_dir.to_str().unwrap(),
        "--output",
        output_dir.to_str().unwrap(),
    ]);
    assert!(
        !output.status.success(),
        "generate should fail for molecule test with unsafe body"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("unsafe") || stderr.contains("SPEC_MOLECULE_BODY_CONTAINS_UNSAFE"),
        "error should mention unsafe\nstderr: {stderr}"
    );
}

#[test]
fn export_rejects_molecule_body_with_nested_unsafe_expr() {
    let temp_dir = temp_repo_dir();
    let units_dir = temp_dir.path().join("units");

    write_molecule_test_target_unit(&units_dir);
    write_indexed_unsafe_molecule_test(&units_dir);

    let output = run(&["export", units_dir.to_str().unwrap()]);
    assert!(
        !output.status.success(),
        "export should fail for molecule test with unsafe body"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("unsafe") || stderr.contains("SPEC_MOLECULE_BODY_CONTAINS_UNSAFE"),
        "error should mention unsafe\nstderr: {stderr}"
    );
}

#[test]
fn spec_test_with_molecule_tests_writes_molecule_evidence_not_passports() {
    if !cargo_available() {
        return;
    }

    let temp_dir = temp_repo_dir();
    let project_dir = temp_dir.path();
    write_pricing_project(project_dir, true);

    // Add a molecule test that covers one of the pricing units
    write_spec(
        &project_dir.join("units"),
        "pricing/tax_and_discount.test.spec",
        r#"id: pricing/tax_and_discount
spec_version: "0.3.0"
intent:
  why: Verify tax and discount interact correctly.
covers:
  - pricing/apply_tax
  - pricing/apply_discount
body:
  rust: |
    {
        assert!(true);
    }
"#,
    );

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
        .expect("failed to run spec");

    assert_output_success(
        "spec test should succeed when molecule tests are present",
        &output,
    );

    // Molecule evidence is written to a dedicated co-located artifact.
    let molecule_evidence = project_dir.join("units/pricing/tax_and_discount.test.evidence.json");
    assert!(
        molecule_evidence.exists(),
        "spec test should write molecule evidence for .test.spec files: {}",
        molecule_evidence.display()
    );
    let molecule_evidence_json: Value =
        serde_json::from_str(&fs::read_to_string(&molecule_evidence).unwrap()).unwrap();
    assert_eq!(molecule_evidence_json["id"], "pricing/tax_and_discount");
    assert_eq!(molecule_evidence_json["status"], "pass");

    // Molecule tests still do not use unit-passport artifacts.
    let molecule_passport =
        project_dir.join("units/pricing/tax_and_discount.test.spec.passport.json");
    assert!(
        !molecule_passport.exists(),
        "spec test must not write a passport for .test.spec files: {}",
        molecule_passport.display()
    );

    // Unit passports should still be written as normal
    assert!(
        project_dir
            .join("units/pricing/apply_tax.spec.passport.json")
            .exists(),
        "unit passport should still be written for apply_tax"
    );
}

#[test]
fn spec_test_accepts_molecule_file_path_and_writes_only_targeted_evidence() {
    if !cargo_available() {
        return;
    }

    let temp_dir = temp_repo_dir();
    let project_dir = temp_dir.path();
    write_pricing_project(project_dir, true);
    let units_dir = project_dir.join("units");

    write_spec(
        &units_dir,
        "pricing/tax_and_discount.test.spec",
        r#"id: pricing/tax_and_discount
spec_version: "0.3.0"
intent:
  why: Verify tax and discount interact correctly.
covers:
  - pricing/apply_tax
  - pricing/apply_discount
body:
  rust: |
    {
        assert!(true);
    }
"#,
    );
    write_spec(
        &units_dir,
        "pricing/discount_only.test.spec",
        r#"id: pricing/discount_only
spec_version: "0.3.0"
intent:
  why: Verify discount-only flow.
covers:
  - pricing/apply_discount
body:
  rust: |
    {
        assert!(true);
    }
"#,
    );

    let target_test_path = units_dir.join("pricing/tax_and_discount.test.spec");
    let output = Command::new(bin())
        .current_dir(project_dir)
        .args([
            "test",
            target_test_path.to_str().unwrap(),
            "--crate-root",
            project_dir.to_str().unwrap(),
        ])
        .output()
        .expect("failed to run spec");

    assert_output_success("spec test should accept a .test.spec file path", &output);

    let targeted_evidence = units_dir.join("pricing/tax_and_discount.test.evidence.json");
    let untouched_evidence = units_dir.join("pricing/discount_only.test.evidence.json");
    assert!(
        targeted_evidence.exists(),
        "expected targeted molecule evidence"
    );
    assert!(
        !untouched_evidence.exists(),
        "single-file molecule test should not write sibling evidence"
    );
    assert!(
        units_dir
            .join("pricing/apply_tax.spec.passport.json")
            .exists(),
        "single-file molecule test should refresh covered unit passports"
    );
    assert!(
        units_dir
            .join("pricing/apply_discount.spec.passport.json")
            .exists(),
        "single-file molecule test should refresh all covered unit passports"
    );
}

#[test]
fn spec_test_accepts_absolute_molecule_file_path_from_tmp_symlink_root() {
    if !cargo_available() {
        return;
    }

    let temp_dir = tempfile::Builder::new().tempdir_in("/tmp").unwrap();
    let project_dir = temp_dir.path();
    write_pricing_project(project_dir, true);
    let units_dir = project_dir.join("units");

    write_spec(
        &units_dir,
        "pricing/tax_and_discount.test.spec",
        r#"id: pricing/tax_and_discount
spec_version: "0.3.0"
intent:
  why: Verify tax and discount interact correctly.
covers:
  - pricing/apply_tax
  - pricing/apply_discount
body:
  rust: |
    {
        assert!(true);
    }
"#,
    );

    let target_test_path = units_dir.join("pricing/tax_and_discount.test.spec");
    let output = Command::new(bin())
        .current_dir(project_dir)
        .args([
            "test",
            target_test_path.to_str().unwrap(),
            "--crate-root",
            project_dir.to_str().unwrap(),
        ])
        .output()
        .expect("failed to run spec");

    assert_output_success(
        "spec test should accept an absolute molecule file path rooted under /tmp",
        &output,
    );
}

#[test]
fn spec_test_empty_directory_still_runs_cargo_tests() {
    if !cargo_available() {
        return;
    }

    let temp_dir = temp_repo_dir();
    let project_dir = temp_dir.path();
    let units_dir = project_dir.join("units");
    fs::create_dir_all(&units_dir).unwrap();
    fs::create_dir_all(project_dir.join("src")).unwrap();

    fs::write(
        project_dir.join("Cargo.toml"),
        "[package]\nname = \"empty-units-fixture\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[workspace]\n",
    )
    .unwrap();
    fs::write(
        project_dir.join("src/main.rs"),
        r#"fn main() {}

#[cfg(test)]
mod tests {
    #[test]
    fn unrelated_failure() {
        panic!("unrelated failing cargo test");
    }
}
"#,
    )
    .unwrap();

    let output = Command::new(bin())
        .current_dir(project_dir)
        .args([
            "test",
            "units",
            "--output",
            "generated/spec",
            "--crate-root",
            project_dir.to_str().unwrap(),
        ])
        .output()
        .expect("failed to run spec");

    assert!(
        !output.status.success(),
        "spec test should fail when cargo tests fail, even with zero generated unit specs"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let combined = format!("{stdout}\n{stderr}");
    assert!(
        combined.contains("unrelated_failure"),
        "expected cargo test output to mention the failing test\noutput: {combined}"
    );
    assert!(
        stderr.contains("cargo test failed"),
        "expected spec test to surface cargo test failure\nstderr: {stderr}"
    );
}

#[test]
fn reserved_unit_name_molecule_tests_is_rejected() {
    let temp_dir = temp_repo_dir();
    let units_dir = temp_dir.path().join("units");

    write_spec(
        &units_dir,
        "pricing/molecule_tests.unit.spec",
        r#"
id: pricing/molecule_tests
kind: function
intent:
  why: This ID is reserved.
spec_version: "0.3.0"
body:
  rust: |
    { }
"#,
    );

    let output = run(&["validate", units_dir.to_str().unwrap()]);
    assert!(
        !output.status.success(),
        "validate should fail for unit with reserved ID segment 'molecule_tests'"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("reserved") || stderr.contains("SPEC_RESERVED_UNIT_NAME"),
        "error should mention reserved name\nstderr: {stderr}"
    );
}

#[test]
fn reserved_molecule_test_name_molecule_tests_is_rejected() {
    let temp_dir = temp_repo_dir();
    let units_dir = temp_dir.path().join("units");
    let output_dir = temp_dir.path().join("generated");

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

    write_molecule_test_spec(
        &units_dir,
        "qa/molecule_tests/foo.test.spec",
        "qa/molecule_tests/foo",
        &["pricing/apply_discount"],
    );

    let output = run(&["validate", units_dir.to_str().unwrap()]);
    assert!(
        !output.status.success(),
        "validate should fail for molecule test with reserved ID segment 'molecule_tests'"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("reserved") || stderr.contains("SPEC_RESERVED_UNIT_NAME"),
        "error should mention reserved name\nstderr: {stderr}"
    );
    assert!(
        stderr.contains("qa/molecule_tests/foo.test.spec"),
        "error should point at the molecule test path\nstderr: {stderr}"
    );

    let json_output = run(&["validate", units_dir.to_str().unwrap(), "--format", "json"]);
    assert!(
        !json_output.status.success(),
        "validate --format json should fail for molecule test with reserved ID segment"
    );
    assert!(
        json_output.stderr.is_empty(),
        "expected no stderr output, got: {}",
        String::from_utf8_lossy(&json_output.stderr)
    );

    let json = parse_stdout_json(&json_output);
    assert_eq!(json["status"], "invalid");
    let errors = json["errors"].as_array().unwrap();
    assert!(
        errors.iter().any(|error| {
            let path = error["path"].as_str().unwrap_or_default();
            error["code"] == "SPEC_RESERVED_UNIT_NAME"
                && path.ends_with("/units/qa/molecule_tests/foo.test.spec")
                && error["value"] == "molecule_tests"
        }),
        "expected SPEC_RESERVED_UNIT_NAME for molecule test path, got: {errors:?}"
    );

    fs::create_dir_all(&output_dir).unwrap();
    fs::write(output_dir.join(".spec-generated"), "").unwrap();

    let generate_output = run(&[
        "generate",
        units_dir.to_str().unwrap(),
        "--output",
        output_dir.to_str().unwrap(),
    ]);
    assert!(
        !generate_output.status.success(),
        "generate should fail for molecule test with reserved ID segment 'molecule_tests'"
    );
    assert!(
        !output_dir.join("qa/molecule_tests.rs").exists(),
        "generate should fail before writing qa/molecule_tests.rs"
    );
    assert!(
        !output_dir.join("qa/molecule_tests/mod.rs").exists(),
        "generate should fail before writing qa/molecule_tests/mod.rs"
    );
}

#[test]
fn validate_accepts_valid_direct_cross_library_dep() {
    let fixture = setup_m9_repo_fixture();
    fs::write(
        fixture.app_root.join("spec.toml"),
        "[libraries]\nshared = \"../shared-spec\"\n",
    )
    .unwrap();
    write_m9_app_cargo_toml(&fixture.app_root, &["shared"]);
    write_m9_unit(
        &fixture.app_root.join("units"),
        "pricing/apply_discount.unit.spec",
        "pricing/apply_discount",
        &["shared::money/round"],
    );
    write_m9_unit(
        &fixture.shared_root.join("units"),
        "money/round.unit.spec",
        "money/round",
        &[],
    );

    let output = run_in(&fixture.app_root, &["validate", "units"]);
    assert!(
        output.status.success(),
        "validate should succeed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("2 unit"), "{stdout}");
}

#[test]
fn validate_ignores_unreferenced_broken_library() {
    let fixture = setup_m9_repo_fixture();
    fs::write(
        fixture.app_root.join("spec.toml"),
        "[libraries]\nshared = \"../shared-spec\"\npayments = \"../payments-spec\"\n",
    )
    .unwrap();
    write_m9_app_cargo_toml(&fixture.app_root, &["shared"]);
    write_m9_unit(
        &fixture.app_root.join("units"),
        "pricing/apply_discount.unit.spec",
        "pricing/apply_discount",
        &["shared::money/round"],
    );
    write_m9_unit(
        &fixture.shared_root.join("units"),
        "money/round.unit.spec",
        "money/round",
        &[],
    );
    write_file(
        &fixture.payments_root.join("units"),
        "money/charge.unit.spec",
        "not: valid: yaml: [unclosed",
    );

    let output = run_in(&fixture.app_root, &["validate", "units"]);
    assert_output_success(
        "validate should ignore unreferenced configured libraries",
        &output,
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("2 unit"), "{stdout}");
}

#[test]
fn validate_rejects_transitive_library_alias_without_loading_transitive_library() {
    let fixture = setup_m9_repo_fixture();
    fs::write(
        fixture.app_root.join("spec.toml"),
        "[libraries]\nshared = \"../shared-spec\"\npayments = \"../payments-spec\"\n",
    )
    .unwrap();
    write_m9_app_cargo_toml(&fixture.app_root, &["shared"]);
    write_m9_unit(
        &fixture.app_root.join("units"),
        "pricing/apply_discount.unit.spec",
        "pricing/apply_discount",
        &["shared::money/round"],
    );
    write_m9_unit(
        &fixture.shared_root.join("units"),
        "money/round.unit.spec",
        "money/round",
        &["payments::money/scale"],
    );
    write_file(
        &fixture.payments_root.join("units"),
        "money/scale.unit.spec",
        "not: valid: yaml: [unclosed",
    );

    let output = run_in(&fixture.app_root, &["validate", "units"]);
    assert!(!output.status.success(), "validate should fail");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("unknown library namespace 'payments'"),
        "{stderr}"
    );
    assert!(!stderr.contains("YAML parse error"), "{stderr}");
}

#[test]
fn validate_ignores_unreferenced_library_cycles() {
    let fixture = setup_m9_repo_fixture();
    fs::write(
        fixture.app_root.join("spec.toml"),
        "[libraries]\nshared = \"../shared-spec\"\npayments = \"../payments-spec\"\n",
    )
    .unwrap();
    write_m9_app_cargo_toml(&fixture.app_root, &["shared"]);
    write_m9_unit(
        &fixture.app_root.join("units"),
        "pricing/apply_discount.unit.spec",
        "pricing/apply_discount",
        &["shared::money/round"],
    );
    write_m9_unit(
        &fixture.shared_root.join("units"),
        "money/round.unit.spec",
        "money/round",
        &[],
    );
    write_m9_unit(
        &fixture.payments_root.join("units"),
        "money/charge.unit.spec",
        "money/charge",
        &["money/refund"],
    );
    write_m9_unit(
        &fixture.payments_root.join("units"),
        "money/refund.unit.spec",
        "money/refund",
        &["money/charge"],
    );

    let output = run_in(&fixture.app_root, &["validate", "units"]);
    assert_output_success(
        "validate should ignore cycles in unreferenced configured libraries",
        &output,
    );
}

#[test]
fn validate_preserves_referenced_broken_library_failures() {
    let fixture = setup_m9_repo_fixture();
    fs::write(
        fixture.app_root.join("spec.toml"),
        "[libraries]\nshared = \"../shared-spec\"\n",
    )
    .unwrap();
    write_m9_app_cargo_toml(&fixture.app_root, &["shared"]);
    write_m9_unit(
        &fixture.app_root.join("units"),
        "pricing/apply_discount.unit.spec",
        "pricing/apply_discount",
        &["shared::money/round"],
    );
    write_file(
        &fixture.shared_root.join("units"),
        "money/round.unit.spec",
        "not: valid: yaml: [unclosed",
    );

    let output = run_in(&fixture.app_root, &["validate", "units"]);
    assert!(!output.status.success(), "validate should fail");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("YAML parse error"), "{stderr}");
    assert!(stderr.contains("shared-spec"), "{stderr}");
}

#[test]
fn validate_rejects_missing_library_crate_alias() {
    let fixture = setup_m9_repo_fixture();
    fs::write(
        fixture.app_root.join("spec.toml"),
        "[libraries]\nshared = \"../shared-spec\"\n",
    )
    .unwrap();
    write_m9_app_cargo_toml(&fixture.app_root, &[]);
    write_m9_unit(
        &fixture.app_root.join("units"),
        "pricing/apply_discount.unit.spec",
        "pricing/apply_discount",
        &["shared::money/round"],
    );
    write_m9_unit(
        &fixture.shared_root.join("units"),
        "money/round.unit.spec",
        "money/round",
        &[],
    );

    let output = run_in(&fixture.app_root, &["validate", "units"]);
    assert!(!output.status.success(), "validate should fail");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("SPEC_LIBRARY_CRATE_ALIAS_MISSING"),
        "{stderr}"
    );
    assert!(stderr.contains("shared"), "{stderr}");
}

#[test]
fn validate_json_accepts_data_seam_cross_library_method_dep_with_cargo_alias() {
    let fixture = setup_m9_repo_fixture();
    fs::write(
        fixture.app_root.join("spec.toml"),
        "[libraries]\nshared = \"../shared-spec\"\n",
    )
    .unwrap();
    write_m9_app_cargo_toml(&fixture.app_root, &["shared"]);
    write_m9_data_seam(
        &fixture.app_root.join("units"),
        "pricing/checkout_quote.unit.spec",
        "pricing/checkout_quote",
        &["shared::money/round"],
    );
    write_spec(
        &fixture.shared_root.join("units"),
        "money/round.unit.spec",
        &format!(
            r#"
id: money/round
kind: function
intent:
  why: Round a subtotal.
spec_version: "{AUTHORED_SPEC_VERSION}"
contract:
  inputs:
    value: i32
  returns: i32
body:
  rust: |
    {{
        value
    }}
"#
        ),
    );

    let output = run_in(
        &fixture.app_root,
        &[
            "validate",
            "units/pricing/checkout_quote.unit.spec",
            "--format",
            "json",
        ],
    );
    assert_output_success(
        "validate should accept cross-library method deps for data seams",
        &output,
    );

    let json = parse_stdout_json(&output);
    assert_eq!(json["status"], "valid");
}

#[test]
fn validate_json_reports_missing_library_crate_alias_for_data_seam_method_dep() {
    let fixture = setup_m9_repo_fixture();
    fs::write(
        fixture.app_root.join("spec.toml"),
        "[libraries]\nshared = \"../shared-spec\"\n",
    )
    .unwrap();
    write_m9_app_cargo_toml(&fixture.app_root, &[]);
    write_m9_data_seam(
        &fixture.app_root.join("units"),
        "pricing/checkout_quote.unit.spec",
        "pricing/checkout_quote",
        &["shared::money/round"],
    );
    write_spec(
        &fixture.shared_root.join("units"),
        "money/round.unit.spec",
        &format!(
            r#"
id: money/round
kind: function
intent:
  why: Round a subtotal.
spec_version: "{AUTHORED_SPEC_VERSION}"
contract:
  inputs:
    value: i32
  returns: i32
body:
  rust: |
    {{
        value
    }}
"#
        ),
    );

    let output = run_in(
        &fixture.app_root,
        &[
            "validate",
            "units/pricing/checkout_quote.unit.spec",
            "--format",
            "json",
        ],
    );
    assert!(
        !output.status.success(),
        "validate should fail when the Cargo alias is missing"
    );
    assert!(
        output.stderr.is_empty(),
        "expected no stderr, got: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let json = parse_stdout_json(&output);
    assert_eq!(json["status"], "invalid");
    let errors = json["errors"].as_array().unwrap();
    assert!(
        errors
            .iter()
            .any(|error| error["code"] == "SPEC_LIBRARY_CRATE_ALIAS_MISSING"),
        "expected SPEC_LIBRARY_CRATE_ALIAS_MISSING, got: {errors:?}"
    );
    assert!(
        !errors
            .iter()
            .any(|error| error["code"] == "SPEC_UNKNOWN_LIBRARY_NAMESPACE"),
        "unexpected SPEC_UNKNOWN_LIBRARY_NAMESPACE, got: {errors:?}"
    );
}

#[test]
fn validate_json_reports_library_manifest_errors_without_alias_misdiagnosis() {
    let fixture = setup_m9_repo_fixture();
    fs::write(
        fixture.app_root.join("spec.toml"),
        "[libraries]\nshared = \"../shared-spec\"\n",
    )
    .unwrap();
    write_invalid_m9_app_cargo_toml(&fixture.app_root);
    write_m9_unit(
        &fixture.app_root.join("units"),
        "pricing/apply_discount.unit.spec",
        "pricing/apply_discount",
        &["shared::money/round"],
    );
    write_m9_unit(
        &fixture.shared_root.join("units"),
        "money/round.unit.spec",
        "money/round",
        &[],
    );

    let output = run_in(
        &fixture.app_root,
        &["validate", "units", "--format", "json"],
    );
    assert!(!output.status.success(), "validate should fail");
    assert!(
        output.stderr.is_empty(),
        "expected no stderr, got: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let json = parse_stdout_json(&output);
    let errors = json["errors"].as_array().unwrap();
    assert!(
        errors
            .iter()
            .any(|error| error["code"] == "SPEC_LIBRARY_CRATE_MANIFEST_ERROR"),
        "expected SPEC_LIBRARY_CRATE_MANIFEST_ERROR, got: {errors:?}"
    );
    assert!(
        !errors
            .iter()
            .any(|error| error["code"] == "SPEC_LIBRARY_CRATE_ALIAS_MISSING"),
        "unexpected alias-missing error, got: {errors:?}"
    );
    assert!(
        !String::from_utf8_lossy(&output.stdout).contains("<unresolved>/Cargo.toml"),
        "unexpected unresolved manifest path: {}",
        String::from_utf8_lossy(&output.stdout)
    );
}

#[test]
fn validate_rejects_unknown_library_namespace() {
    let fixture = setup_m9_repo_fixture();
    fs::write(
        fixture.app_root.join("spec.toml"),
        "[libraries]\nshared = \"../shared-spec\"\n",
    )
    .unwrap();
    write_m9_app_cargo_toml(&fixture.app_root, &["shared"]);
    write_m9_unit(
        &fixture.app_root.join("units"),
        "pricing/apply_discount.unit.spec",
        "pricing/apply_discount",
        &["payments::money/round"],
    );
    write_m9_unit(
        &fixture.shared_root.join("units"),
        "money/round.unit.spec",
        "money/round",
        &[],
    );

    let output = run_in(&fixture.app_root, &["validate", "units"]);
    assert!(!output.status.success(), "validate should fail");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("unknown library namespace 'payments'"),
        "{stderr}"
    );
}

#[test]
fn validate_rejects_missing_library_path() {
    let fixture = setup_m9_repo_fixture();
    fs::write(
        fixture.app_root.join("spec.toml"),
        "[libraries]\nshared = \"../missing-spec\"\n",
    )
    .unwrap();
    write_m9_unit(
        &fixture.app_root.join("units"),
        "pricing/apply_discount.unit.spec",
        "pricing/apply_discount",
        &[],
    );

    let output = run_in(&fixture.app_root, &["validate", "units"]);
    assert!(!output.status.success(), "validate should fail");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("SPEC_LIBRARY_PATH_NOT_FOUND"), "{stderr}");
}

#[test]
fn validate_json_surfaces_missing_library_path_as_machine_readable_error() {
    let fixture = setup_m9_repo_fixture();
    fs::write(
        fixture.app_root.join("spec.toml"),
        "[libraries]\nshared = \"../missing-spec\"\n",
    )
    .unwrap();
    write_m9_unit(
        &fixture.app_root.join("units"),
        "pricing/apply_discount.unit.spec",
        "pricing/apply_discount",
        &[],
    );

    let output = run_in(
        &fixture.app_root,
        &["validate", "units", "--format", "json"],
    );
    assert!(!output.status.success(), "validate should fail");
    assert!(
        output.stderr.is_empty(),
        "expected no stderr, got: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let json = parse_stdout_json(&output);
    assert_eq!(json["schema_version"], 3);
    assert_eq!(json["status"], "invalid");
    assert_eq!(json["warnings"], serde_json::json!([]));
    let errors = json["errors"].as_array().unwrap();
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0]["code"], "SPEC_LIBRARY_PATH_NOT_FOUND");
    assert_eq!(errors[0]["unit"], Value::Null);
    assert_eq!(errors[0]["path"], Value::String("spec.toml".to_string()));
    assert!(
        errors[0]["message"]
            .as_str()
            .unwrap()
            .contains("library 'shared' path does not exist"),
        "{errors:?}"
    );
}

#[test]
fn validate_rejects_library_path_outside_repo() {
    let fixture = setup_m9_repo_fixture();
    let outside_root = fixture
        .app_root
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("outside-spec");
    fs::create_dir_all(outside_root.join("units")).unwrap();
    fs::write(
        fixture.app_root.join("spec.toml"),
        "[libraries]\noutside = \"../../outside-spec\"\n",
    )
    .unwrap();
    write_m9_unit(
        &fixture.app_root.join("units"),
        "pricing/apply_discount.unit.spec",
        "pricing/apply_discount",
        &[],
    );

    let output = run_in(&fixture.app_root, &["validate", "units"]);
    assert!(!output.status.success(), "validate should fail");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("SPEC_LIBRARY_OUT_OF_ROOT"), "{stderr}");
}

#[test]
fn validate_rejects_library_alias_to_self() {
    let fixture = setup_m9_repo_fixture();
    fs::write(
        fixture.app_root.join("spec.toml"),
        "[libraries]\napp = \".\"\n",
    )
    .unwrap();
    write_m9_unit(
        &fixture.app_root.join("units"),
        "pricing/apply_discount.unit.spec",
        "pricing/apply_discount",
        &[],
    );

    let output = run_in(&fixture.app_root, &["validate", "units"]);
    assert!(!output.status.success(), "validate should fail");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("SPEC_LIBRARY_ALIAS_SELF"), "{stderr}");
}

#[test]
fn validate_rejects_duplicate_canonical_library_roots() {
    let fixture = setup_m9_repo_fixture();
    fs::write(
        fixture.app_root.join("spec.toml"),
        "[libraries]\nshared = \"../shared-spec\"\nshared_copy = \"../shared-spec/./\"\n",
    )
    .unwrap();
    write_m9_unit(
        &fixture.app_root.join("units"),
        "pricing/apply_discount.unit.spec",
        "pricing/apply_discount",
        &[],
    );

    let output = run_in(&fixture.app_root, &["validate", "units"]);
    assert!(!output.status.success(), "validate should fail");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("SPEC_DUPLICATE_LIBRARY_ROOT"), "{stderr}");
}

#[test]
fn validate_detects_direct_cross_library_cycle() {
    let fixture = setup_m9_repo_fixture();
    fs::write(
        fixture.app_root.join("spec.toml"),
        "[libraries]\nshared = \"../shared-spec\"\npayments = \"../payments-spec\"\n",
    )
    .unwrap();
    write_m9_app_cargo_toml(&fixture.app_root, &["shared", "payments"]);
    write_m9_unit(
        &fixture.app_root.join("units"),
        "pricing/apply_discount.unit.spec",
        "pricing/apply_discount",
        &["shared::money/round", "payments::money/scale"],
    );
    write_m9_unit(
        &fixture.shared_root.join("units"),
        "money/round.unit.spec",
        "money/round",
        &["payments::money/scale"],
    );
    write_m9_unit(
        &fixture.payments_root.join("units"),
        "money/scale.unit.spec",
        "money/scale",
        &["shared::money/round"],
    );

    let output = run_in(&fixture.app_root, &["validate", "units"]);
    assert!(!output.status.success(), "validate should fail");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("cycle detected"), "{stderr}");
    assert!(stderr.contains("shared::money/round"), "{stderr}");
    assert!(stderr.contains("payments::money/scale"), "{stderr}");
}

#[test]
fn validate_json_emits_cross_library_cycle_code() {
    let fixture = setup_m9_repo_fixture();
    fs::write(
        fixture.app_root.join("spec.toml"),
        "[libraries]\nshared = \"../shared-spec\"\npayments = \"../payments-spec\"\n",
    )
    .unwrap();
    write_m9_app_cargo_toml(&fixture.app_root, &["shared", "payments"]);
    write_m9_unit(
        &fixture.app_root.join("units"),
        "pricing/apply_discount.unit.spec",
        "pricing/apply_discount",
        &["shared::money/round", "payments::money/scale"],
    );
    write_m9_unit(
        &fixture.shared_root.join("units"),
        "money/round.unit.spec",
        "money/round",
        &["payments::money/scale"],
    );
    write_m9_unit(
        &fixture.payments_root.join("units"),
        "money/scale.unit.spec",
        "money/scale",
        &["shared::money/round"],
    );

    let output = run_in(
        &fixture.app_root,
        &["validate", "units", "--format", "json"],
    );
    assert!(!output.status.success(), "validate should fail");
    assert!(
        output.stderr.is_empty(),
        "expected no stderr, got: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let json = parse_stdout_json(&output);
    let errors = json["errors"].as_array().unwrap();
    assert!(
        errors
            .iter()
            .any(|error| error["code"] == "SPEC_CROSS_LIBRARY_CYCLE"),
        "expected SPEC_CROSS_LIBRARY_CYCLE, got: {errors:?}"
    );
    assert!(
        !errors
            .iter()
            .any(|error| error["code"] == "SPEC_CYCLIC_DEP"),
        "unexpected SPEC_CYCLIC_DEP, got: {errors:?}"
    );
}

#[test]
fn validate_json_rejects_dep_collision_with_unit_callable_name() {
    let fixture = setup_m9_repo_fixture();
    fs::write(
        fixture.app_root.join("spec.toml"),
        "[libraries]\nshared = \"../shared-spec\"\n",
    )
    .unwrap();
    write_m9_app_cargo_toml(&fixture.app_root, &["shared"]);
    write_m9_unit(
        &fixture.app_root.join("units"),
        "money/round.unit.spec",
        "money/round",
        &["shared::money/round"],
    );
    write_m9_unit(
        &fixture.shared_root.join("units"),
        "money/round.unit.spec",
        "money/round",
        &[],
    );

    let output = run_in(
        &fixture.app_root,
        &["validate", "units", "--format", "json"],
    );
    assert!(!output.status.success(), "validate should fail");
    assert!(
        output.stderr.is_empty(),
        "expected no stderr, got: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let json = parse_stdout_json(&output);
    assert_eq!(json["status"], "invalid");
    let errors = json["errors"].as_array().unwrap();
    assert_eq!(errors.len(), 1, "expected one collision error");

    let error = &errors[0];
    assert_eq!(error["code"], "SPEC_DEP_COLLISION");
    assert_eq!(error["unit"], "money/round");
    assert_eq!(error["path"], "units/money/round.unit.spec");
    assert_eq!(error["dep"], "shared::money/round");
    assert_eq!(error["value"], "round");
    assert_eq!(error["path2"], "money/round");
}

#[test]
fn validate_json_rejects_data_constructor_method_id_collision() {
    let temp_dir = temp_repo_dir();
    let spec_path = temp_dir
        .path()
        .join("units/pricing/checkout_quote.unit.spec");
    write_spec(
        temp_dir.path(),
        "units/pricing/checkout_quote.unit.spec",
        &format!(
            r#"
id: pricing/checkout_quote
kind: data
spec_version: "{AUTHORED_SPEC_VERSION}"
intent:
  why: Quote totals.
data:
  fields:
    subtotal:
      type: Decimal
constructors:
  - id: new
    intent:
      why: Create a quote.
    contract:
      inputs:
        subtotal: Decimal
    initializes:
      subtotal: subtotal
methods:
  - id: new
    intent:
      why: Duplicate the constructor callable.
    receiver: shared_ref
    contract:
      returns: Decimal
    lowering:
      rust:
        body: |
          {{
              self.subtotal
          }}
"#
        ),
    );

    let output = run(&["validate", spec_path.to_str().unwrap(), "--format", "json"]);
    assert!(!output.status.success(), "validate should fail");
    assert!(
        output.stderr.is_empty(),
        "expected no stderr, got: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let json = parse_stdout_json(&output);
    assert_eq!(json["status"], "invalid");
    let errors = json["errors"].as_array().unwrap();
    assert_eq!(errors.len(), 1, "expected one collision error");

    let error = &errors[0];
    assert_eq!(error["code"], "SPEC_SEMANTIC_VALIDATION");
    assert_eq!(error["unit"], "pricing/checkout_quote");
    assert_eq!(error["path"], spec_path.to_string_lossy().as_ref());
    assert!(
        error["message"]
            .as_str()
            .unwrap()
            .contains("constructors[0].id 'new' conflicts with methods[0].id 'new'"),
        "unexpected error payload: {error:?}"
    );
}

#[test]
fn validate_json_rejects_kind_data_without_constructors() {
    let temp_dir = temp_repo_dir();
    let spec_path = temp_dir
        .path()
        .join("units/pricing/checkout_quote.unit.spec");
    write_spec(
        temp_dir.path(),
        "units/pricing/checkout_quote.unit.spec",
        &format!(
            r#"
id: pricing/checkout_quote
kind: data
spec_version: "{AUTHORED_SPEC_VERSION}"
intent:
  why: Quote totals.
data:
  fields:
    subtotal:
      type: Decimal
methods:
  - id: total
    intent:
      why: Return the total.
    receiver: shared_ref
    contract:
      returns: Decimal
    lowering:
      rust:
        body: |
          {{
              self.subtotal
          }}
"#
        ),
    );

    let output = run(&["validate", spec_path.to_str().unwrap(), "--format", "json"]);
    assert!(!output.status.success(), "validate should fail");
    assert!(
        output.stderr.is_empty(),
        "expected no stderr, got: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let json = parse_stdout_json(&output);
    assert_eq!(json["status"], "invalid");
    let errors = json["errors"].as_array().unwrap();
    assert_eq!(errors.len(), 1, "expected one schema error");

    let error = &errors[0];
    assert_eq!(error["code"], "SPEC_SCHEMA_VALIDATION");
    assert!(error["unit"].is_null(), "{error:?}");
    assert_eq!(error["path"], spec_path.to_string_lossy().as_ref());
    assert!(
        error["message"]
            .as_str()
            .unwrap()
            .contains("missing required field: \"constructors\""),
        "unexpected error payload: {error:?}"
    );
}

#[test]
fn validate_json_rejects_kind_data_with_empty_constructors() {
    let temp_dir = temp_repo_dir();
    let spec_path = temp_dir
        .path()
        .join("units/pricing/checkout_quote.unit.spec");
    write_spec(
        temp_dir.path(),
        "units/pricing/checkout_quote.unit.spec",
        &format!(
            r#"
id: pricing/checkout_quote
kind: data
spec_version: "{AUTHORED_SPEC_VERSION}"
intent:
  why: Quote totals.
data:
  fields:
    subtotal:
      type: Decimal
constructors: []
methods:
  - id: total
    intent:
      why: Return the total.
    receiver: shared_ref
    contract:
      returns: Decimal
    lowering:
      rust:
        body: |
          {{
              self.subtotal
          }}
"#
        ),
    );

    let output = run(&["validate", spec_path.to_str().unwrap(), "--format", "json"]);
    assert!(!output.status.success(), "validate should fail");
    assert!(
        output.stderr.is_empty(),
        "expected no stderr, got: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let json = parse_stdout_json(&output);
    assert_eq!(json["status"], "invalid");
    let errors = json["errors"].as_array().unwrap();
    assert_eq!(errors.len(), 1, "expected one schema error");

    let error = &errors[0];
    assert_eq!(error["code"], "SPEC_SCHEMA_VALIDATION");
    assert!(error["unit"].is_null(), "{error:?}");
    assert_eq!(error["path"], spec_path.to_string_lossy().as_ref());
    assert!(
        error["message"]
            .as_str()
            .unwrap()
            .contains("[] has less than 1 item (at /constructors)"),
        "unexpected error payload: {error:?}"
    );
}

#[test]
fn validate_json_rejects_kind_data_with_empty_methods() {
    let temp_dir = temp_repo_dir();
    let spec_path = temp_dir
        .path()
        .join("units/pricing/checkout_quote.unit.spec");
    write_spec(
        temp_dir.path(),
        "units/pricing/checkout_quote.unit.spec",
        &format!(
            r#"
id: pricing/checkout_quote
kind: data
spec_version: "{AUTHORED_SPEC_VERSION}"
intent:
  why: Quote totals.
data:
  fields:
    subtotal:
      type: Decimal
constructors:
  - id: new
    intent:
      why: Create a quote.
    contract:
      inputs:
        subtotal: Decimal
    initializes:
      subtotal: subtotal
methods: []
"#
        ),
    );

    let output = run(&["validate", spec_path.to_str().unwrap(), "--format", "json"]);
    assert!(!output.status.success(), "validate should fail");
    assert!(
        output.stderr.is_empty(),
        "expected no stderr, got: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let json = parse_stdout_json(&output);
    assert_eq!(json["status"], "invalid");
    let errors = json["errors"].as_array().unwrap();
    assert_eq!(errors.len(), 1, "expected one schema error");

    let error = &errors[0];
    assert_eq!(error["code"], "SPEC_SCHEMA_VALIDATION");
    assert!(error["unit"].is_null(), "{error:?}");
    assert_eq!(error["path"], spec_path.to_string_lossy().as_ref());
    assert!(
        error["message"]
            .as_str()
            .unwrap()
            .contains("[] has less than 1 item (at /methods)"),
        "unexpected error payload: {error:?}"
    );
}

#[test]
fn validate_json_rejects_data_method_without_contract() {
    let temp_dir = temp_repo_dir();
    let spec_path = temp_dir
        .path()
        .join("units/pricing/checkout_quote.unit.spec");
    write_spec(
        temp_dir.path(),
        "units/pricing/checkout_quote.unit.spec",
        &format!(
            r#"
id: pricing/checkout_quote
kind: data
spec_version: "{AUTHORED_SPEC_VERSION}"
intent:
  why: Quote totals.
data:
  fields:
    subtotal:
      type: Decimal
constructors:
  - id: new
    intent:
      why: Create a quote.
    contract:
      inputs:
        subtotal: Decimal
    initializes:
      subtotal: subtotal
methods:
  - id: total
    intent:
      why: Return the total.
    receiver: shared_ref
    lowering:
      rust:
        body: |
          {{
              self.subtotal
          }}
"#
        ),
    );

    let output = run(&["validate", spec_path.to_str().unwrap(), "--format", "json"]);
    assert!(!output.status.success(), "validate should fail");
    assert!(
        output.stderr.is_empty(),
        "expected no stderr, got: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let json = parse_stdout_json(&output);
    assert_eq!(json["status"], "invalid");
    let errors = json["errors"].as_array().unwrap();
    assert_eq!(errors.len(), 1, "expected one semantic error");

    let error = &errors[0];
    assert_eq!(error["code"], "SPEC_SCHEMA_VALIDATION");
    assert!(error["unit"].is_null(), "{error:?}");
    assert_eq!(error["path"], spec_path.to_string_lossy().as_ref());
    assert!(
        error["message"]
            .as_str()
            .unwrap()
            .contains("missing required field: \"contract\""),
        "unexpected error payload: {error:?}"
    );
}

#[test]
fn validate_json_rejects_invalid_data_rust_backend_derive() {
    let temp_dir = temp_repo_dir();
    let spec_path = temp_dir
        .path()
        .join("units/pricing/checkout_quote.unit.spec");
    write_spec(
        temp_dir.path(),
        "units/pricing/checkout_quote.unit.spec",
        &format!(
            r#"
id: pricing/checkout_quote
kind: data
spec_version: "{AUTHORED_SPEC_VERSION}"
intent:
  why: Quote totals.
data:
  fields:
    subtotal:
      type: Decimal
constructors:
  - id: new
    intent:
      why: Create a quote.
    contract:
      inputs:
        subtotal: Decimal
    initializes:
      subtotal: subtotal
methods:
  - id: total
    intent:
      why: Return the total.
    receiver: shared_ref
    contract:
      returns: Decimal
    lowering:
      rust:
        body: |
          {{
              self.subtotal
          }}
backends:
  rust:
    derives:
      - not valid rust
"#
        ),
    );

    let output = run(&["validate", spec_path.to_str().unwrap(), "--format", "json"]);
    assert!(!output.status.success(), "validate should fail");
    assert!(
        output.stderr.is_empty(),
        "expected no stderr, got: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let json = parse_stdout_json(&output);
    assert_eq!(json["status"], "invalid");
    let errors = json["errors"].as_array().unwrap();
    assert_eq!(errors.len(), 1, "expected one semantic error");

    let error = &errors[0];
    assert_eq!(error["code"], "SPEC_SEMANTIC_VALIDATION");
    assert_eq!(error["unit"], "pricing/checkout_quote");
    assert_eq!(error["path"], spec_path.to_string_lossy().as_ref());
    assert!(
        error["message"]
            .as_str()
            .unwrap()
            .contains("backends.rust.derives[0] must be a valid Rust path"),
        "unexpected error payload: {error:?}"
    );
}

#[test]
fn validate_json_rejects_cross_method_dep_callable_collision_for_data_seam() {
    let temp_dir = temp_repo_dir();
    let spec_path = temp_dir
        .path()
        .join("units/pricing/checkout_quote.unit.spec");
    let units_dir = temp_dir.path().join("units");
    write_spec(
        temp_dir.path(),
        "units/pricing/checkout_quote.unit.spec",
        &format!(
            r#"
id: pricing/checkout_quote
kind: data
spec_version: "{AUTHORED_SPEC_VERSION}"
intent:
  why: Quote totals.
data:
  fields:
    subtotal:
      type: Decimal
constructors:
  - id: new
    intent:
      why: Create a quote.
    contract:
      inputs:
        subtotal: Decimal
    initializes:
      subtotal: subtotal
methods:
  - id: discount_total
    intent:
      why: Use the demo callable.
    receiver: shared_ref
    contract:
      returns: Decimal
    deps:
      - demo/foo
    lowering:
      rust:
        body: |
          {{
              foo(self.subtotal)
          }}
  - id: tax_total
    intent:
      why: Use the util callable.
    receiver: shared_ref
    contract:
      returns: Decimal
    deps:
      - util/foo
    lowering:
      rust:
        body: |
          {{
              foo(self.subtotal)
          }}
"#
        ),
    );
    write_spec(
        temp_dir.path(),
        "units/demo/foo.unit.spec",
        &format!(
            r#"
id: demo/foo
kind: function
spec_version: "{AUTHORED_SPEC_VERSION}"
intent:
  why: Demo callable.
body:
  rust: |
    {{
        value
    }}
"#
        ),
    );
    write_spec(
        temp_dir.path(),
        "units/util/foo.unit.spec",
        &format!(
            r#"
id: util/foo
kind: function
spec_version: "{AUTHORED_SPEC_VERSION}"
intent:
  why: Util callable.
body:
  rust: |
    {{
        value
    }}
"#
        ),
    );

    let output = run(&["validate", units_dir.to_str().unwrap(), "--format", "json"]);
    assert!(!output.status.success(), "validate should fail");
    assert!(
        output.stderr.is_empty(),
        "expected no stderr, got: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let json = parse_stdout_json(&output);
    assert_eq!(json["status"], "invalid");
    let errors = json["errors"].as_array().unwrap();
    assert_eq!(errors.len(), 1, "expected one dep collision error");

    let error = &errors[0];
    assert_eq!(error["code"], "SPEC_DEP_COLLISION");
    assert_eq!(error["unit"], "pricing/checkout_quote");
    assert_eq!(error["path"], spec_path.to_string_lossy().as_ref());
    assert_eq!(error["dep"], "demo/foo");
    assert_eq!(error["value"], "foo");
    assert_eq!(error["path2"], "util/foo");
}

#[test]
fn validate_rejects_cross_library_molecule_covers() {
    let fixture = setup_m9_repo_fixture();
    fs::write(
        fixture.app_root.join("spec.toml"),
        "[libraries]\nshared = \"../shared-spec\"\n",
    )
    .unwrap();
    write_m9_app_cargo_toml(&fixture.app_root, &["shared"]);
    write_m9_unit(
        &fixture.app_root.join("units"),
        "pricing/apply_discount.unit.spec",
        "pricing/apply_discount",
        &[],
    );
    write_file(
        &fixture.app_root.join("units"),
        "pricing/discount_flow.test.spec",
        &format!(
            r#"
id: pricing/discount_flow
intent:
  why: Cross-library covers stay out of scope in M9.
spec_version: "{AUTHORED_SPEC_VERSION}"
covers:
  - shared::money/round
body:
  rust: |
    {{
        assert!(true);
    }}
"#
        ),
    );

    let output = run_in(
        &fixture.app_root,
        &["validate", "units", "--format", "json"],
    );
    assert!(!output.status.success(), "validate should fail");
    assert!(
        output.stderr.is_empty(),
        "expected no stderr, got: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let json = parse_stdout_json(&output);
    assert_eq!(json["status"], "invalid");
    let errors = json["errors"].as_array().unwrap();
    assert_eq!(errors.len(), 1, "expected one cross-library cover error");

    let error = &errors[0];
    assert_eq!(
        error["code"],
        "SPEC_MOLECULE_CROSS_LIBRARY_COVERS_UNSUPPORTED"
    );
    assert_ne!(error["code"], "SPEC_SCHEMA_VALIDATION");
    assert_eq!(error["path"], "units/pricing/discount_flow.test.spec");
    assert_eq!(error["dep"], "shared::money/round");
    assert_eq!(error["id"], "pricing/discount_flow");
    assert!(
        error["message"]
            .as_str()
            .unwrap()
            .contains("cross-library molecule cover 'shared::money/round' is not supported in M9"),
        "unexpected message: {}",
        error["message"]
    );
}

#[test]
fn generate_rejects_missing_library_crate_alias_before_writing_output() {
    let fixture = setup_m9_repo_fixture();
    fs::write(
        fixture.app_root.join("spec.toml"),
        "[libraries]\nshared = \"../shared-spec\"\n",
    )
    .unwrap();
    write_m9_app_cargo_toml(&fixture.app_root, &[]);
    write_m9_unit(
        &fixture.app_root.join("units"),
        "pricing/apply_discount.unit.spec",
        "pricing/apply_discount",
        &["shared::money/round"],
    );
    write_m9_unit(
        &fixture.shared_root.join("units"),
        "money/round.unit.spec",
        "money/round",
        &[],
    );

    let output = run_in(
        &fixture.app_root,
        &["generate", "units", "--output", "src/generated"],
    );
    assert!(!output.status.success(), "generate should fail");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("SPEC_LIBRARY_CRATE_ALIAS_MISSING"),
        "{stderr}"
    );
    assert!(
        !fixture
            .app_root
            .join("src/generated/pricing/apply_discount.rs")
            .exists(),
        "generation should fail before writing output"
    );
}

#[test]
fn generate_accepts_data_seam_cross_library_method_dep_with_cargo_alias() {
    let fixture = setup_m9_repo_fixture();
    fs::write(
        fixture.app_root.join("spec.toml"),
        "[libraries]\nshared = \"../shared-spec\"\n",
    )
    .unwrap();
    write_m9_app_cargo_toml(&fixture.app_root, &["shared"]);
    write_m9_data_seam(
        &fixture.app_root.join("units"),
        "pricing/checkout_quote.unit.spec",
        "pricing/checkout_quote",
        &["shared::money/round"],
    );
    write_spec(
        &fixture.shared_root.join("units"),
        "money/round.unit.spec",
        &format!(
            r#"
id: money/round
kind: function
intent:
  why: Round a subtotal.
spec_version: "{AUTHORED_SPEC_VERSION}"
contract:
  inputs:
    value: i32
  returns: i32
body:
  rust: |
    {{
        value
    }}
"#
        ),
    );

    let output = run_in(
        &fixture.app_root,
        &["generate", "units", "--output", "src/generated"],
    );
    assert_output_success(
        "generate should accept cross-library method deps for data seams",
        &output,
    );
    assert!(
        fixture
            .app_root
            .join("src/generated/pricing/checkout_quote.rs")
            .exists(),
        "expected generated data seam output"
    );
}

#[test]
fn generate_rejects_invalid_library_manifest_before_writing_output() {
    let fixture = setup_m9_repo_fixture();
    fs::write(
        fixture.app_root.join("spec.toml"),
        "[libraries]\nshared = \"../shared-spec\"\n",
    )
    .unwrap();
    write_invalid_m9_app_cargo_toml(&fixture.app_root);
    write_m9_unit(
        &fixture.app_root.join("units"),
        "pricing/apply_discount.unit.spec",
        "pricing/apply_discount",
        &["shared::money/round"],
    );
    write_m9_unit(
        &fixture.shared_root.join("units"),
        "money/round.unit.spec",
        "money/round",
        &[],
    );

    let output = run_in(
        &fixture.app_root,
        &["generate", "units", "--output", "src/generated"],
    );
    assert!(!output.status.success(), "generate should fail");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("SPEC_LIBRARY_CRATE_MANIFEST_ERROR"),
        "{stderr}"
    );
    assert!(stderr.contains("Failed to parse"), "{stderr}");
    assert!(
        !stderr.contains("SPEC_LIBRARY_CRATE_ALIAS_MISSING"),
        "{stderr}"
    );
    assert!(!stderr.contains("<unresolved>/Cargo.toml"), "{stderr}");
    assert!(
        !fixture
            .app_root
            .join("src/generated/pricing/apply_discount.rs")
            .exists(),
        "generation should fail before writing output"
    );
}

#[test]
fn export_rejects_missing_library_crate_alias() {
    let fixture = setup_m9_repo_fixture();
    fs::write(
        fixture.app_root.join("spec.toml"),
        "[libraries]\nshared = \"../shared-spec\"\n",
    )
    .unwrap();
    write_m9_app_cargo_toml(&fixture.app_root, &[]);
    write_m9_unit(
        &fixture.app_root.join("units"),
        "pricing/apply_discount.unit.spec",
        "pricing/apply_discount",
        &["shared::money/round"],
    );
    write_m9_unit(
        &fixture.shared_root.join("units"),
        "money/round.unit.spec",
        "money/round",
        &[],
    );

    let output = run_in(&fixture.app_root, &["export", "units"]);
    assert!(!output.status.success(), "export should fail");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("SPEC_LIBRARY_CRATE_ALIAS_MISSING"),
        "{stderr}"
    );
    assert!(stderr.contains("shared"), "{stderr}");
}

#[test]
fn export_rejects_missing_library_crate_alias_before_writing_output() {
    let fixture = setup_m9_repo_fixture();
    fs::write(
        fixture.app_root.join("spec.toml"),
        "[libraries]\nshared = \"../shared-spec\"\n",
    )
    .unwrap();
    write_m9_app_cargo_toml(&fixture.app_root, &[]);
    write_m9_unit(
        &fixture.app_root.join("units"),
        "pricing/apply_discount.unit.spec",
        "pricing/apply_discount",
        &["shared::money/round"],
    );
    write_m9_unit(
        &fixture.shared_root.join("units"),
        "money/round.unit.spec",
        "money/round",
        &[],
    );

    let bundle_path = fixture.app_root.join("bundle.json");
    let output = run_in(
        &fixture.app_root,
        &["export", "units", "--output", "bundle.json"],
    );
    assert!(!output.status.success(), "export should fail");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("SPEC_LIBRARY_CRATE_ALIAS_MISSING"),
        "{stderr}"
    );
    assert!(
        !bundle_path.exists(),
        "export should fail before writing output bundle"
    );
}

#[test]
fn export_emits_schema_v3_bundle_for_valid_cross_library_dep() {
    let fixture = setup_m9_repo_fixture();
    fs::write(
        fixture.app_root.join("spec.toml"),
        "[libraries]\nshared = \"../shared-spec\"\n",
    )
    .unwrap();
    write_m9_app_cargo_toml(&fixture.app_root, &["shared"]);
    write_m9_unit(
        &fixture.app_root.join("units"),
        "pricing/apply_discount.unit.spec",
        "pricing/apply_discount",
        &["shared::money/round"],
    );
    write_m9_unit(
        &fixture.shared_root.join("units"),
        "money/round.unit.spec",
        "money/round",
        &[],
    );

    let output = run_in(&fixture.app_root, &["export", "units"]);
    assert!(output.status.success(), "export should succeed");

    let bundle: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(bundle["schema_version"], 5);
    let edges = bundle["graph"]["edges"].as_array().unwrap();
    assert!(
        edges.iter().any(|edge| {
            edge["kind"] == "dep"
                && edge["from"]["library"].is_null()
                && edge["from"]["id"] == "pricing/apply_discount"
                && edge["to"]["library"] == "shared"
                && edge["to"]["id"] == "money/round"
        }),
        "expected cross-library dep edge in export bundle, got: {edges:?}"
    );
}

#[test]
fn export_ignores_unreferenced_broken_library() {
    let fixture = setup_m9_repo_fixture();
    fs::write(
        fixture.app_root.join("spec.toml"),
        "[libraries]\nshared = \"../shared-spec\"\npayments = \"../payments-spec\"\n",
    )
    .unwrap();
    write_m9_app_cargo_toml(&fixture.app_root, &["shared"]);
    write_m9_unit(
        &fixture.app_root.join("units"),
        "pricing/apply_discount.unit.spec",
        "pricing/apply_discount",
        &["shared::money/round"],
    );
    write_m9_unit(
        &fixture.shared_root.join("units"),
        "money/round.unit.spec",
        "money/round",
        &[],
    );
    write_file(
        &fixture.payments_root.join("units"),
        "money/charge.unit.spec",
        "not: valid: yaml: [unclosed",
    );

    let output = run_in(&fixture.app_root, &["export", "units"]);
    assert!(output.status.success(), "export should succeed");

    let bundle: Value = serde_json::from_slice(&output.stdout).unwrap();
    let edges = bundle["graph"]["edges"].as_array().unwrap();
    assert!(
        edges.iter().any(|edge| {
            edge["kind"] == "dep"
                && edge["from"]["library"].is_null()
                && edge["from"]["id"] == "pricing/apply_discount"
                && edge["to"]["library"] == "shared"
                && edge["to"]["id"] == "money/round"
        }),
        "expected cross-library dep edge in export bundle, got: {edges:?}"
    );
}

#[test]
fn status_reports_valid_cross_library_unit_as_untested_without_dep_errors() {
    let fixture = setup_m9_repo_fixture();
    fs::write(
        fixture.app_root.join("spec.toml"),
        "[libraries]\nshared = \"../shared-spec\"\n",
    )
    .unwrap();
    write_m9_app_cargo_toml(&fixture.app_root, &["shared"]);
    write_m9_unit(
        &fixture.app_root.join("units"),
        "pricing/apply_discount.unit.spec",
        "pricing/apply_discount",
        &["shared::money/round"],
    );
    write_m9_unit(
        &fixture.shared_root.join("units"),
        "money/round.unit.spec",
        "money/round",
        &[],
    );

    let output = run_in(&fixture.app_root, &["status", "units"]);
    assert!(
        !output.status.success(),
        "untested status should exit non-zero until evidence exists"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stdout.contains("pricing/apply_discount"), "{stdout}");
    assert!(stdout.contains("untested"), "{stdout}");
    assert!(
        !stdout.contains("SPEC_CROSS_LIBRARY_DEP_NOT_FOUND"),
        "{stdout}"
    );
    assert!(stderr.is_empty(), "{stderr}");
}

#[test]
fn status_json_reports_cross_library_unit_as_untested_without_loader_errors() {
    let fixture = setup_m9_repo_fixture();
    fs::write(
        fixture.app_root.join("spec.toml"),
        "[libraries]\nshared = \"../shared-spec\"\n",
    )
    .unwrap();
    write_m9_app_cargo_toml(&fixture.app_root, &["shared"]);
    write_m9_unit(
        &fixture.app_root.join("units"),
        "pricing/apply_discount.unit.spec",
        "pricing/apply_discount",
        &["shared::money/round"],
    );
    write_m9_unit(
        &fixture.shared_root.join("units"),
        "money/round.unit.spec",
        "money/round",
        &[],
    );

    let output = run_in(&fixture.app_root, &["status", "units", "--format", "json"]);
    assert!(
        !output.status.success(),
        "untested status should exit non-zero until evidence exists"
    );
    assert!(
        output.stderr.is_empty(),
        "expected no stderr, got: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let json = parse_stdout_json(&output);
    let units = json["units"].as_array().unwrap();
    assert_eq!(units.len(), 1);
    assert_eq!(units[0]["id"], "pricing/apply_discount");
    assert_eq!(units[0]["status"], "untested");
    assert_eq!(units[0]["errors"].as_array().unwrap().len(), 0);
    assert!(
        json.get("loader_errors").is_none(),
        "expected no global loader_errors, got: {json:?}"
    );
}

#[test]
fn status_json_ignores_unreferenced_broken_library_loader_errors() {
    let fixture = setup_m9_repo_fixture();
    fs::write(
        fixture.app_root.join("spec.toml"),
        "[libraries]\nshared = \"../shared-spec\"\npayments = \"../payments-spec\"\n",
    )
    .unwrap();
    write_m9_app_cargo_toml(&fixture.app_root, &["shared"]);
    write_m9_unit(
        &fixture.app_root.join("units"),
        "pricing/apply_discount.unit.spec",
        "pricing/apply_discount",
        &["shared::money/round"],
    );
    write_m9_unit(
        &fixture.shared_root.join("units"),
        "money/round.unit.spec",
        "money/round",
        &[],
    );
    write_file(
        &fixture.payments_root.join("units"),
        "money/charge.unit.spec",
        "not: valid: yaml: [unclosed",
    );

    let output = run_in(&fixture.app_root, &["status", "units", "--format", "json"]);
    assert!(
        !output.status.success(),
        "untested status should exit non-zero until evidence exists"
    );

    let json = parse_stdout_json(&output);
    let units = json["units"].as_array().unwrap();
    assert_eq!(units.len(), 1);
    assert_eq!(units[0]["id"], "pricing/apply_discount");
    assert_eq!(units[0]["status"], "untested");
    assert_eq!(units[0]["errors"].as_array().unwrap().len(), 0);
    assert!(
        json.get("loader_errors").is_none(),
        "expected no loader errors from unreferenced libraries, got: {json:?}"
    );
}

#[test]
fn status_json_routes_direct_cross_library_cycles_to_loader_errors() {
    let fixture = setup_m9_repo_fixture();
    fs::write(
        fixture.app_root.join("spec.toml"),
        "[libraries]\nshared = \"../shared-spec\"\npayments = \"../payments-spec\"\n",
    )
    .unwrap();
    write_m9_app_cargo_toml(&fixture.app_root, &["shared", "payments"]);
    write_m9_unit(
        &fixture.app_root.join("units"),
        "pricing/apply_discount.unit.spec",
        "pricing/apply_discount",
        &["shared::money/round", "payments::money/scale"],
    );
    write_m9_unit(
        &fixture.shared_root.join("units"),
        "money/round.unit.spec",
        "money/round",
        &["payments::money/scale"],
    );
    write_m9_unit(
        &fixture.payments_root.join("units"),
        "money/scale.unit.spec",
        "money/scale",
        &["shared::money/round"],
    );

    let output = run_in(&fixture.app_root, &["status", "units", "--format", "json"]);
    assert!(!output.status.success(), "status should fail");
    let json = parse_stdout_json(&output);
    let units = json["units"].as_array().unwrap();
    assert_eq!(units.len(), 1);
    assert_eq!(units[0]["status"], "untested");
    assert_eq!(units[0]["errors"].as_array().unwrap().len(), 0);

    let loader_errors = json["loader_errors"].as_array().unwrap();
    assert!(
        loader_errors
            .iter()
            .any(|error| error["code"] == "SPEC_CROSS_LIBRARY_CYCLE"),
        "expected SPEC_CROSS_LIBRARY_CYCLE, got: {loader_errors:?}"
    );
    assert!(
        !loader_errors
            .iter()
            .any(|error| error["code"] == "SPEC_CYCLIC_DEP"),
        "unexpected SPEC_CYCLIC_DEP, got: {loader_errors:?}"
    );
}

#[test]
fn status_json_reports_transitive_library_alias_without_loading_transitive_library() {
    let fixture = setup_m9_repo_fixture();
    fs::write(
        fixture.app_root.join("spec.toml"),
        "[libraries]\nshared = \"../shared-spec\"\npayments = \"../payments-spec\"\n",
    )
    .unwrap();
    write_m9_app_cargo_toml(&fixture.app_root, &["shared"]);
    write_m9_unit(
        &fixture.app_root.join("units"),
        "pricing/apply_discount.unit.spec",
        "pricing/apply_discount",
        &["shared::money/round"],
    );
    write_m9_unit(
        &fixture.shared_root.join("units"),
        "money/round.unit.spec",
        "money/round",
        &["payments::money/scale"],
    );
    write_file(
        &fixture.payments_root.join("units"),
        "money/scale.unit.spec",
        "not: valid: yaml: [unclosed",
    );

    let output = run_in(&fixture.app_root, &["status", "units", "--format", "json"]);
    assert!(!output.status.success(), "status should fail");

    let json = parse_stdout_json(&output);
    let units = json["units"].as_array().unwrap();
    assert_eq!(units.len(), 1);
    assert_eq!(units[0]["id"], "pricing/apply_discount");
    assert_eq!(units[0]["status"], "untested");
    assert_eq!(units[0]["errors"].as_array().unwrap().len(), 0);

    let loader_errors = json["loader_errors"].as_array().unwrap();
    assert!(
        loader_errors
            .iter()
            .any(|error| error["code"] == "SPEC_UNKNOWN_LIBRARY_NAMESPACE"),
        "expected SPEC_UNKNOWN_LIBRARY_NAMESPACE, got: {loader_errors:?}"
    );
    assert!(
        !loader_errors
            .iter()
            .any(|error| error["code"] == "SPEC_YAML_PARSE"),
        "unexpected transitive loader error, got: {loader_errors:?}"
    );
}

#[test]
fn status_marks_missing_library_crate_alias_as_invalid() {
    let fixture = setup_m9_repo_fixture();
    fs::write(
        fixture.app_root.join("spec.toml"),
        "[libraries]\nshared = \"../shared-spec\"\n",
    )
    .unwrap();
    write_m9_app_cargo_toml(&fixture.app_root, &[]);
    write_m9_unit(
        &fixture.app_root.join("units"),
        "pricing/apply_discount.unit.spec",
        "pricing/apply_discount",
        &["shared::money/round"],
    );
    write_m9_unit(
        &fixture.shared_root.join("units"),
        "money/round.unit.spec",
        "money/round",
        &[],
    );

    let output = run_in(&fixture.app_root, &["status", "units", "--format", "json"]);
    assert!(!output.status.success(), "status should fail");
    let json = parse_stdout_json(&output);
    let units = json["units"].as_array().unwrap();
    assert_eq!(units[0]["status"], "invalid");
    let errors = units[0]["errors"].as_array().unwrap();
    assert!(
        errors
            .iter()
            .any(|error| error["code"] == "SPEC_LIBRARY_CRATE_ALIAS_MISSING"),
        "expected SPEC_LIBRARY_CRATE_ALIAS_MISSING, got: {errors:?}"
    );
    assert!(
        !errors
            .iter()
            .any(|error| error["code"] == "SPEC_CROSS_LIBRARY_DEP_NOT_FOUND"),
        "unexpected dep-not-found error, got: {errors:?}"
    );
}

#[test]
fn test_accepts_data_seam_cross_library_method_dep_with_cargo_alias() {
    if !cargo_available() {
        return;
    }

    let fixture = setup_m9_repo_fixture();
    fs::write(
        fixture.app_root.join("spec.toml"),
        "[libraries]\nshared = \"../shared-spec\"\n",
    )
    .unwrap();
    write_m9_app_cargo_toml(&fixture.app_root, &["shared"]);
    write_file(
        &fixture.app_root,
        "src/main.rs",
        "mod generated;\npub use generated::*;\nfn main() {}\n",
    );
    let shared_crate_root = fixture.app_root.parent().unwrap().join("shared-crate");
    write_file(
        &shared_crate_root,
        "Cargo.toml",
        r#"[package]
name = "shared"
version = "0.1.0"
edition = "2024"

[lib]
path = "src/lib.rs"
"#,
    );
    write_file(
        &shared_crate_root,
        "src/lib.rs",
        "pub mod money {\n    pub mod round {\n        pub fn round(value: i32) -> i32 {\n            value\n        }\n    }\n}\n",
    );
    write_m9_data_seam(
        &fixture.app_root.join("units"),
        "pricing/checkout_quote.unit.spec",
        "pricing/checkout_quote",
        &["shared::money/round"],
    );
    write_spec(
        &fixture.shared_root.join("units"),
        "money/round.unit.spec",
        &format!(
            r#"
id: money/round
kind: function
intent:
  why: Round a subtotal.
spec_version: "{AUTHORED_SPEC_VERSION}"
contract:
  inputs:
    value: i32
  returns: i32
body:
  rust: |
    {{
        value
    }}
"#
        ),
    );

    let output = run_in(
        &fixture.app_root,
        &[
            "test",
            "units",
            "--output",
            "src/generated",
            "--crate-root",
            ".",
        ],
    );
    assert_output_success(
        "test should accept cross-library method deps for data seams",
        &output,
    );
    assert!(
        fixture
            .app_root
            .join("units/pricing/checkout_quote.spec.passport.json")
            .exists(),
        "expected data seam passport after test"
    );
}

#[test]
fn single_file_test_accepts_data_seam_cross_library_method_dep_with_cargo_alias() {
    if !cargo_available() {
        return;
    }

    let fixture = setup_m9_repo_fixture();
    fs::write(
        fixture.app_root.join("spec.toml"),
        "[libraries]\nshared = \"../shared-spec\"\n",
    )
    .unwrap();
    write_m9_app_cargo_toml(&fixture.app_root, &["shared"]);
    write_m9_shared_round_crate_fixture(&fixture);
    write_m9_data_seam(
        &fixture.app_root.join("units"),
        "pricing/checkout_quote.unit.spec",
        "pricing/checkout_quote",
        &["shared::money/round"],
    );
    write_spec(
        &fixture.shared_root.join("units"),
        "money/round.unit.spec",
        &format!(
            r#"
id: money/round
kind: function
intent:
  why: Round a subtotal.
spec_version: "{AUTHORED_SPEC_VERSION}"
contract:
  inputs:
    value: i32
  returns: i32
body:
  rust: |
    {{
        value
    }}
"#
        ),
    );

    let output = run_in(
        &fixture.app_root,
        &[
            "test",
            "units/pricing/checkout_quote.unit.spec",
            "--crate-root",
            ".",
        ],
    );
    assert_output_success(
        "single-file spec test should accept cross-library method deps for data seams",
        &output,
    );
    assert!(
        fixture
            .app_root
            .join("units/pricing/checkout_quote.spec.passport.json")
            .exists(),
        "expected data seam passport after single-file test"
    );
}

#[test]
fn single_file_molecule_test_accepts_cross_library_data_seam_dep_with_cargo_alias() {
    if !cargo_available() {
        return;
    }

    let fixture = setup_m9_repo_fixture();
    fs::write(
        fixture.app_root.join("spec.toml"),
        "[libraries]\nshared = \"../shared-spec\"\n",
    )
    .unwrap();
    write_m9_app_cargo_toml(&fixture.app_root, &["shared"]);
    write_m9_shared_round_crate_fixture(&fixture);
    write_m9_data_seam(
        &fixture.app_root.join("units"),
        "pricing/checkout_quote.unit.spec",
        "pricing/checkout_quote",
        &["shared::money/round"],
    );
    write_spec(
        &fixture.app_root.join("units"),
        "pricing/checkout_flow.test.spec",
        r#"
id: pricing/checkout_flow
intent:
  why: Verify the checkout quote seam through a molecule test.
covers:
  - pricing/checkout_quote
body:
  rust: |
    {
        let quote = CheckoutQuote::new(5);
        assert_eq!(quote.total(), 5);
    }
"#,
    );
    write_spec(
        &fixture.shared_root.join("units"),
        "money/round.unit.spec",
        &format!(
            r#"
id: money/round
kind: function
intent:
  why: Round a subtotal.
spec_version: "{AUTHORED_SPEC_VERSION}"
contract:
  inputs:
    value: i32
  returns: i32
body:
  rust: |
    {{
        value
    }}
"#
        ),
    );

    let output = run_in(
        &fixture.app_root,
        &[
            "test",
            "units/pricing/checkout_flow.test.spec",
            "--crate-root",
            ".",
        ],
    );
    assert_output_success(
        "single-file molecule test should accept cross-library unit deps",
        &output,
    );
    assert!(
        fixture
            .app_root
            .join("units/pricing/checkout_flow.test.evidence.json")
            .exists(),
        "expected molecule evidence after single-file molecule test"
    );
}

#[test]
fn status_marks_missing_library_crate_alias_as_invalid_for_data_seam_method_dep() {
    let fixture = setup_m9_repo_fixture();
    fs::write(
        fixture.app_root.join("spec.toml"),
        "[libraries]\nshared = \"../shared-spec\"\n",
    )
    .unwrap();
    write_m9_app_cargo_toml(&fixture.app_root, &[]);
    write_m9_data_seam(
        &fixture.app_root.join("units"),
        "pricing/checkout_quote.unit.spec",
        "pricing/checkout_quote",
        &["shared::money/round"],
    );
    write_spec(
        &fixture.shared_root.join("units"),
        "money/round.unit.spec",
        &format!(
            r#"
id: money/round
kind: function
intent:
  why: Round a subtotal.
spec_version: "{AUTHORED_SPEC_VERSION}"
contract:
  inputs:
    value: i32
  returns: i32
body:
  rust: |
    {{
        value
    }}
"#
        ),
    );

    let output = run_in(&fixture.app_root, &["status", "units", "--format", "json"]);
    assert!(
        !output.status.success(),
        "status should fail when the Cargo alias is missing"
    );
    let json = parse_stdout_json(&output);
    let units = json["units"].as_array().unwrap();
    assert_eq!(units[0]["status"], "invalid");
    let errors = units[0]["errors"].as_array().unwrap();
    assert!(
        errors
            .iter()
            .any(|error| error["code"] == "SPEC_LIBRARY_CRATE_ALIAS_MISSING"),
        "expected SPEC_LIBRARY_CRATE_ALIAS_MISSING, got: {errors:?}"
    );
    assert!(
        !errors
            .iter()
            .any(|error| error["code"] == "SPEC_UNKNOWN_LIBRARY_NAMESPACE"),
        "unexpected SPEC_UNKNOWN_LIBRARY_NAMESPACE, got: {errors:?}"
    );
}

#[test]
fn status_json_surfaces_library_manifest_errors_globally() {
    let fixture = setup_m9_repo_fixture();
    fs::write(
        fixture.app_root.join("spec.toml"),
        "[libraries]\nshared = \"../shared-spec\"\n",
    )
    .unwrap();
    write_invalid_m9_app_cargo_toml(&fixture.app_root);
    write_m9_unit(
        &fixture.app_root.join("units"),
        "pricing/apply_discount.unit.spec",
        "pricing/apply_discount",
        &["shared::money/round"],
    );
    write_m9_unit(
        &fixture.shared_root.join("units"),
        "money/round.unit.spec",
        "money/round",
        &[],
    );

    let output = run_in(&fixture.app_root, &["status", "units", "--format", "json"]);
    assert!(!output.status.success(), "status should fail");

    let json = parse_stdout_json(&output);
    let units = json["units"].as_array().unwrap();
    assert_eq!(units[0]["status"], "untested");
    assert_eq!(units[0]["errors"].as_array().unwrap().len(), 0);

    let loader_errors = json["loader_errors"].as_array().unwrap();
    assert!(
        loader_errors
            .iter()
            .any(|error| error["code"] == "SPEC_LIBRARY_CRATE_MANIFEST_ERROR"),
        "expected SPEC_LIBRARY_CRATE_MANIFEST_ERROR, got: {loader_errors:?}"
    );
    assert!(
        !loader_errors
            .iter()
            .any(|error| error["code"] == "SPEC_LIBRARY_CRATE_ALIAS_MISSING"),
        "unexpected alias-missing error, got: {loader_errors:?}"
    );
    assert!(
        !String::from_utf8_lossy(&output.stdout).contains("<unresolved>/Cargo.toml"),
        "unexpected unresolved manifest path: {}",
        String::from_utf8_lossy(&output.stdout)
    );
}

#[test]
fn status_surfaces_imported_library_loader_errors_globally_without_misreporting_root_unit() {
    let fixture = setup_m9_repo_fixture();
    fs::write(
        fixture.app_root.join("spec.toml"),
        "[libraries]\nshared = \"../shared-spec\"\n",
    )
    .unwrap();
    write_m9_app_cargo_toml(&fixture.app_root, &["shared"]);
    write_m9_unit(
        &fixture.app_root.join("units"),
        "pricing/apply_discount.unit.spec",
        "pricing/apply_discount",
        &["shared::money/round"],
    );
    fs::create_dir_all(fixture.shared_root.join("units/money")).unwrap();
    fs::write(
        fixture.shared_root.join("units/money/round.unit.spec"),
        "not: valid: yaml: [unclosed",
    )
    .unwrap();

    let output = run_in(&fixture.app_root, &["status", "units", "--format", "json"]);
    assert!(!output.status.success(), "status should fail");
    let json = parse_stdout_json(&output);
    let units = json["units"].as_array().unwrap();
    assert_eq!(units.len(), 1);
    assert_eq!(units[0]["status"], "untested");
    assert_eq!(units[0]["errors"].as_array().unwrap().len(), 0);

    let loader_errors = json["loader_errors"].as_array().unwrap();
    assert!(
        loader_errors
            .iter()
            .any(|error| error["code"] == "SPEC_YAML_PARSE"),
        "expected imported loader error, got: {loader_errors:?}"
    );
    assert!(
        !loader_errors
            .iter()
            .any(|error| error["code"] == "SPEC_CROSS_LIBRARY_DEP_NOT_FOUND"),
        "unexpected dep-not-found global error, got: {loader_errors:?}"
    );
}

#[test]
fn status_json_surfaces_missing_library_path_as_loader_error() {
    let fixture = setup_m9_repo_fixture();
    fs::write(
        fixture.app_root.join("spec.toml"),
        "[libraries]\nshared = \"../missing-spec\"\n",
    )
    .unwrap();
    write_m9_unit(
        &fixture.app_root.join("units"),
        "pricing/apply_discount.unit.spec",
        "pricing/apply_discount",
        &[],
    );

    let output = run_in(&fixture.app_root, &["status", "units", "--format", "json"]);
    assert!(!output.status.success(), "status should fail");
    assert!(
        output.stderr.is_empty(),
        "expected no stderr, got: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let json = parse_stdout_json(&output);
    assert_eq!(json["schema_version"], 5);
    assert_eq!(json["units"], serde_json::json!([]));
    let loader_errors = json["loader_errors"].as_array().unwrap();
    assert_eq!(loader_errors.len(), 1);
    assert_eq!(loader_errors[0]["code"], "SPEC_LIBRARY_PATH_NOT_FOUND");
    assert_eq!(loader_errors[0]["unit"], Value::Null);
    assert_eq!(
        loader_errors[0]["path"],
        Value::String("spec.toml".to_string())
    );
    assert!(
        loader_errors[0]["message"]
            .as_str()
            .unwrap()
            .contains("library 'shared' path does not exist"),
        "{loader_errors:?}"
    );
}

#[test]
fn validate_ignores_broken_configured_library_when_root_has_no_cross_library_deps() {
    let fixture = setup_m9_repo_fixture();
    fs::write(
        fixture.app_root.join("spec.toml"),
        "[libraries]\npayments = \"../payments-spec\"\n",
    )
    .unwrap();
    write_m9_unit(
        &fixture.app_root.join("units"),
        "pricing/apply_discount.unit.spec",
        "pricing/apply_discount",
        &[],
    );
    write_file(
        &fixture.payments_root.join("units"),
        "money/charge.unit.spec",
        "not: valid: yaml: [unclosed",
    );

    let output = run_in(&fixture.app_root, &["validate", "units"]);
    assert_output_success(
        "validate should ignore configured libraries when root specs have no cross-library deps",
        &output,
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("1 unit"), "{stdout}");
}

#[test]
fn status_json_ignores_configured_library_when_root_has_no_cross_library_deps() {
    let fixture = setup_m9_repo_fixture();
    fs::write(
        fixture.app_root.join("spec.toml"),
        "[libraries]\npayments = \"../payments-spec\"\n",
    )
    .unwrap();
    write_m9_unit(
        &fixture.app_root.join("units"),
        "pricing/apply_discount.unit.spec",
        "pricing/apply_discount",
        &[],
    );
    write_file(
        &fixture.payments_root.join("units"),
        "money/charge.unit.spec",
        "not: valid: yaml: [unclosed",
    );

    let output = run_in(&fixture.app_root, &["status", "units", "--format", "json"]);
    assert!(
        !output.status.success(),
        "untested status should exit non-zero until evidence exists"
    );

    let json = parse_stdout_json(&output);
    let units = json["units"].as_array().unwrap();
    assert_eq!(units.len(), 1);
    assert_eq!(units[0]["id"], "pricing/apply_discount");
    assert_eq!(units[0]["status"], "untested");
    assert!(
        json.get("loader_errors").is_none(),
        "expected no loader errors from unused configured libraries, got: {json:?}"
    );
}

#[test]
fn export_ignores_configured_library_when_root_has_no_cross_library_deps() {
    let fixture = setup_m9_repo_fixture();
    fs::write(
        fixture.app_root.join("spec.toml"),
        "[libraries]\npayments = \"../payments-spec\"\n",
    )
    .unwrap();
    write_m9_unit(
        &fixture.app_root.join("units"),
        "pricing/apply_discount.unit.spec",
        "pricing/apply_discount",
        &[],
    );
    write_file(
        &fixture.payments_root.join("units"),
        "money/charge.unit.spec",
        "not: valid: yaml: [unclosed",
    );

    let output = run_in(&fixture.app_root, &["export", "units"]);
    assert!(output.status.success(), "export should succeed");

    let bundle: Value = serde_json::from_slice(&output.stdout).unwrap();
    let edges = bundle["graph"]["edges"].as_array().unwrap();
    assert!(
        !edges.iter().any(|edge| edge["to"]["library"] == "payments"),
        "did not expect unused configured libraries in export graph, got: {edges:?}"
    );
}

#[test]
fn export_rejects_transitive_library_alias_without_loading_transitive_library() {
    let fixture = setup_m9_repo_fixture();
    fs::write(
        fixture.app_root.join("spec.toml"),
        "[libraries]\nshared = \"../shared-spec\"\npayments = \"../payments-spec\"\n",
    )
    .unwrap();
    write_m9_app_cargo_toml(&fixture.app_root, &["shared"]);
    write_m9_unit(
        &fixture.app_root.join("units"),
        "pricing/apply_discount.unit.spec",
        "pricing/apply_discount",
        &["shared::money/round"],
    );
    write_m9_unit(
        &fixture.shared_root.join("units"),
        "money/round.unit.spec",
        "money/round",
        &["payments::money/scale"],
    );
    write_file(
        &fixture.payments_root.join("units"),
        "money/scale.unit.spec",
        "not: valid: yaml: [unclosed",
    );

    let output = run_in(&fixture.app_root, &["export", "units"]);
    assert!(!output.status.success(), "export should fail");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("unknown library namespace 'payments'"),
        "{stderr}"
    );
    assert!(!stderr.contains("YAML parse error"), "{stderr}");
}

#[test]
fn generate_ignores_unreferenced_broken_library() {
    let fixture = setup_m9_repo_fixture();
    fs::write(
        fixture.app_root.join("spec.toml"),
        "[libraries]\nshared = \"../shared-spec\"\npayments = \"../payments-spec\"\n",
    )
    .unwrap();
    write_m9_app_cargo_toml(&fixture.app_root, &["shared"]);
    write_m9_unit(
        &fixture.app_root.join("units"),
        "pricing/apply_discount.unit.spec",
        "pricing/apply_discount",
        &["shared::money/round"],
    );
    write_m9_unit(
        &fixture.shared_root.join("units"),
        "money/round.unit.spec",
        "money/round",
        &[],
    );
    write_file(
        &fixture.payments_root.join("units"),
        "money/charge.unit.spec",
        "not: valid: yaml: [unclosed",
    );

    let output = run_in(
        &fixture.app_root,
        &["generate", "units", "--output", "src/generated"],
    );
    assert_output_success(
        "generate should ignore unreferenced configured libraries",
        &output,
    );
    assert!(
        fixture
            .app_root
            .join("src/generated/pricing/apply_discount.rs")
            .exists(),
        "expected generated output for referenced root unit"
    );
}

#[test]
fn generate_ignores_configured_library_when_root_has_no_cross_library_deps() {
    let fixture = setup_m9_repo_fixture();
    fs::write(
        fixture.app_root.join("spec.toml"),
        "[libraries]\npayments = \"../payments-spec\"\n",
    )
    .unwrap();
    write_m9_unit(
        &fixture.app_root.join("units"),
        "pricing/apply_discount.unit.spec",
        "pricing/apply_discount",
        &[],
    );
    write_file(
        &fixture.payments_root.join("units"),
        "money/charge.unit.spec",
        "not: valid: yaml: [unclosed",
    );

    let output = run_in(
        &fixture.app_root,
        &["generate", "units", "--output", "src/generated"],
    );
    assert_output_success(
        "generate should ignore configured libraries when root has no cross-library deps",
        &output,
    );
    assert!(
        fixture
            .app_root
            .join("src/generated/pricing/apply_discount.rs")
            .exists(),
        "expected generated output for local root unit"
    );
}

#[test]
fn generate_rejects_transitive_library_alias_without_loading_transitive_library() {
    let fixture = setup_m9_repo_fixture();
    fs::write(
        fixture.app_root.join("spec.toml"),
        "[libraries]\nshared = \"../shared-spec\"\npayments = \"../payments-spec\"\n",
    )
    .unwrap();
    write_m9_app_cargo_toml(&fixture.app_root, &["shared"]);
    write_m9_unit(
        &fixture.app_root.join("units"),
        "pricing/apply_discount.unit.spec",
        "pricing/apply_discount",
        &["shared::money/round"],
    );
    write_m9_unit(
        &fixture.shared_root.join("units"),
        "money/round.unit.spec",
        "money/round",
        &["payments::money/scale"],
    );
    write_file(
        &fixture.payments_root.join("units"),
        "money/scale.unit.spec",
        "not: valid: yaml: [unclosed",
    );

    let output = run_in(
        &fixture.app_root,
        &["generate", "units", "--output", "src/generated"],
    );
    assert!(!output.status.success(), "generate should fail");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("unknown library namespace 'payments'"),
        "{stderr}"
    );
    assert!(!stderr.contains("YAML parse error"), "{stderr}");
}

const M10_MODIFY_PLAN: &str = r#"
id: checkout-tax-refactor
intent:
  why: "Refactor tax calculation to support tiered rates without losing checkout coverage."
changes:
  - unit: pricing/apply_tax
    action: modify
    acceptance:
      validate:
        - pricing/apply_tax
        - pricing/calculate_total
        - pricing/calculate_total_guarded_tax
        - pricing/pricing_quote
      molecule_tests:
        - pricing/checkout_flow
        - pricing/discount_plus_tax
        - pricing/discount_strategy_checkout_flow
      notes:
        - "current blast radius stays fully covered"
notes:
  - "M10 plans are local-library only."
"#;

const M10_MIXED_PLAN: &str = r#"
id: checkout-tax-refactor
intent:
  why: "Refactor tax calculation to support tiered rates without losing checkout coverage."
changes:
  - unit: pricing/apply_tax
    action: modify
    acceptance:
      validate:
        - pricing/apply_tax
      molecule_tests:
        - pricing/checkout_flow
      notes:
        - "tiered-rate behavior is covered by checkout_flow"
  - unit: pricing/tiered_rate
    action: add
    acceptance:
      validate:
        - pricing/tiered_rate
notes:
  - "M10 plans are local-library only."
"#;

const M10_REMOVE_PLAN: &str = r#"
id: remove-tax
intent:
  why: "Evaluate the current removal blast radius for apply_tax."
changes:
  - unit: pricing/apply_tax
    action: remove
    acceptance:
      validate:
        - pricing/apply_tax
        - pricing/calculate_total
        - pricing/calculate_total_guarded_tax
        - pricing/pricing_quote
      molecule_tests:
        - pricing/checkout_flow
        - pricing/discount_plus_tax
        - pricing/discount_strategy_checkout_flow
"#;

fn setup_m10_plan_fixture(
    plan_relative_path: &str,
    body: &str,
) -> (tempfile::TempDir, PathBuf, PathBuf) {
    let (temp_dir, ecommerce_dir) = copy_ecommerce_example();
    let plan_path = ecommerce_dir.join(plan_relative_path);
    if let Some(parent) = plan_path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(&plan_path, body).unwrap();
    (temp_dir, ecommerce_dir, plan_path)
}

fn add_hidden_scratch_units_copy(ecommerce_dir: &Path) {
    let scratch_units_dir = ecommerce_dir.join(".scratch/units");
    copy_dir_recursive(&ecommerce_dir.join("units"), &scratch_units_dir)
        .expect("failed to copy units into hidden scratch tree");
}

fn normalize_exported_at(mut json: Value) -> Value {
    json["exported_at"] = Value::String("<normalized>".to_string());
    json
}

#[test]
fn plan_validate_rejects_directory_input() {
    let (_temp_dir, ecommerce_dir, _plan_path) = setup_m10_plan_fixture(
        "plans/refactors/checkout-tax-refactor.plan.spec",
        M10_MIXED_PLAN,
    );

    let output = run_in(
        &ecommerce_dir,
        &["plan", "validate", "plans", "--format", "json"],
    );
    assert!(
        !output.status.success(),
        "plan validate should fail on directory input"
    );

    let json = parse_stdout_json(&output);
    assert_eq!(json["status"], "invalid");
    assert_eq!(json["errors"][0]["code"], "SPEC_PLAN_DIRECTORY_INPUT");
}

#[test]
fn plan_validate_nested_plan_path_matches_checked_in_fixture() {
    let (_temp_dir, ecommerce_dir) = copy_ecommerce_example();
    let plan_path = ecommerce_dir.join("plans/refactors/checkout-tax-refactor.plan.spec");

    let output = run_in(
        &ecommerce_dir,
        &[
            "plan",
            "validate",
            plan_path
                .strip_prefix(&ecommerce_dir)
                .unwrap()
                .to_str()
                .unwrap(),
            "--format",
            "json",
        ],
    );
    assert_output_success("plan validate should succeed for nested plan path", &output);
    assert_stdout_json_matches_fixture(&output, "plan-validate-valid-mixed.json");
}

#[test]
fn plan_validate_ignores_hidden_scratch_units_copy() {
    let (_temp_dir, ecommerce_dir) = copy_ecommerce_example();
    let plan_path = ecommerce_dir.join("plans/refactors/checkout-tax-refactor.plan.spec");
    add_hidden_scratch_units_copy(&ecommerce_dir);

    let output = run_in(
        &ecommerce_dir,
        &[
            "plan",
            "validate",
            plan_path
                .strip_prefix(&ecommerce_dir)
                .unwrap()
                .to_str()
                .unwrap(),
            "--format",
            "json",
        ],
    );
    assert_output_success(
        "plan validate should ignore hidden scratch units copies",
        &output,
    );
    assert_stdout_json_matches_fixture(&output, "plan-validate-valid-mixed.json");
}

#[test]
fn plan_validate_modify_plan_keeps_seed_unit_in_computed_impact() {
    let (_temp_dir, ecommerce_dir, plan_path) =
        setup_m10_plan_fixture("plans/modify-tax.plan.spec", M10_MODIFY_PLAN);

    let output = run_in(
        &ecommerce_dir,
        &[
            "plan",
            "validate",
            plan_path
                .strip_prefix(&ecommerce_dir)
                .unwrap()
                .to_str()
                .unwrap(),
            "--format",
            "json",
        ],
    );
    assert_output_success("plan validate should succeed for modify plan", &output);

    let json = parse_stdout_json(&output);
    let units = json["computed_impact"]["units"]
        .as_array()
        .unwrap()
        .iter()
        .map(|value| value.as_str().unwrap())
        .collect::<Vec<_>>();
    assert!(units.contains(&"pricing/apply_tax"), "{json}");
    assert!(units.contains(&"pricing/calculate_total"), "{json}");
    assert_eq!(json["computed_impact"]["status"], "complete");
}

#[test]
fn plan_validate_remove_plan_uses_current_graph_impact() {
    let (_temp_dir, ecommerce_dir, plan_path) =
        setup_m10_plan_fixture("plans/remove-tax.plan.spec", M10_REMOVE_PLAN);

    let output = run_in(
        &ecommerce_dir,
        &[
            "plan",
            "validate",
            plan_path
                .strip_prefix(&ecommerce_dir)
                .unwrap()
                .to_str()
                .unwrap(),
            "--format",
            "json",
        ],
    );
    assert_output_success("plan validate should succeed for remove plan", &output);

    let json = parse_stdout_json(&output);
    assert_eq!(json["computed_impact"]["status"], "complete");
    assert_eq!(
        json["computed_impact"]["molecule_tests"],
        serde_json::json!([
            "pricing/checkout_flow",
            "pricing/discount_plus_tax",
            "pricing/discount_strategy_checkout_flow"
        ])
    );
}

#[test]
fn plan_validate_rejects_missing_modify_unit() {
    let (_temp_dir, ecommerce_dir, plan_path) = setup_m10_plan_fixture(
        "plans/missing-modify.plan.spec",
        r#"
id: missing-modify
intent:
  why: "Should fail because modify targets must already exist."
changes:
  - unit: pricing/tiered_rate
    action: modify
    acceptance:
      validate:
        - pricing/tiered_rate
"#,
    );

    let output = run_in(
        &ecommerce_dir,
        &[
            "plan",
            "validate",
            plan_path
                .strip_prefix(&ecommerce_dir)
                .unwrap()
                .to_str()
                .unwrap(),
            "--format",
            "json",
        ],
    );
    assert!(
        !output.status.success(),
        "expected missing modify target to fail"
    );

    let json = parse_stdout_json(&output);
    assert_eq!(json["status"], "invalid");
    assert_eq!(
        json["errors"][0]["code"],
        "SPEC_PLAN_UNIT_MISSING_FOR_ACTION"
    );
    assert_eq!(json["errors"][0]["id"], "pricing/tiered_rate");
    assert_eq!(json["errors"][0]["value"], "modify");
}

#[test]
fn plan_validate_rejects_duplicate_change_units_in_json() {
    let (_temp_dir, ecommerce_dir, plan_path) = setup_m10_plan_fixture(
        "plans/duplicate-change.plan.spec",
        r#"
id: duplicate-change
intent:
  why: "Should fail because one plan cannot author the same unit twice."
changes:
  - unit: pricing/apply_tax
    action: modify
    acceptance:
      validate:
        - pricing/apply_tax
  - unit: pricing/apply_tax
    action: remove
    acceptance:
      validate:
        - pricing/apply_tax
"#,
    );

    let output = run_in(
        &ecommerce_dir,
        &[
            "plan",
            "validate",
            plan_path
                .strip_prefix(&ecommerce_dir)
                .unwrap()
                .to_str()
                .unwrap(),
            "--format",
            "json",
        ],
    );
    assert!(
        !output.status.success(),
        "expected duplicate plan change unit to fail"
    );

    let json = parse_stdout_json(&output);
    assert_eq!(json["status"], "invalid");
    assert_eq!(json["errors"][0]["code"], "SPEC_PLAN_DUPLICATE_CHANGE_UNIT");
    assert_eq!(json["errors"][0]["id"], "pricing/apply_tax");
}

#[test]
fn plan_validate_rejects_plan_outside_library_root() {
    let (temp_dir, ecommerce_dir) = copy_ecommerce_example();
    let outside_plan = temp_dir.path().join("outside.plan.spec");
    fs::write(&outside_plan, M10_MODIFY_PLAN).unwrap();

    let output = run_in(
        &ecommerce_dir,
        &[
            "plan",
            "validate",
            outside_plan.to_str().unwrap(),
            "--format",
            "json",
        ],
    );
    assert!(
        !output.status.success(),
        "expected outside-root plan to fail"
    );

    let json = parse_stdout_json(&output);
    assert_eq!(json["status"], "invalid");
    assert_eq!(json["errors"][0]["code"], "SPEC_PLAN_OUTSIDE_LIBRARY_ROOT");
}

#[cfg(unix)]
#[test]
fn plan_validate_rejects_symlink_escape() {
    use std::os::unix::fs::symlink;

    let (_temp_dir, ecommerce_dir, _plan_path) =
        setup_m10_plan_fixture("plans/local.plan.spec", M10_MODIFY_PLAN);
    let outside_dir = tempfile::TempDir::new().unwrap();
    let outside_plan = outside_dir.path().join("escape.plan.spec");
    fs::write(&outside_plan, M10_MODIFY_PLAN).unwrap();
    let symlink_path = ecommerce_dir.join("plans/escape.plan.spec");
    symlink(&outside_plan, &symlink_path).unwrap();

    let output = run_in(
        &ecommerce_dir,
        &[
            "plan",
            "validate",
            "plans/escape.plan.spec",
            "--format",
            "json",
        ],
    );
    assert!(!output.status.success(), "expected symlink escape to fail");

    let json = parse_stdout_json(&output);
    assert_eq!(json["errors"][0]["code"], "SPEC_PLAN_SYMLINK_ESCAPE");
}

#[cfg(unix)]
#[test]
fn plan_validate_rejects_symlinked_external_unit_in_library_graph() {
    use std::os::unix::fs::symlink;

    let (_temp_dir, ecommerce_dir, plan_path) =
        setup_m10_plan_fixture("plans/local.plan.spec", M10_MODIFY_PLAN);
    let outside_dir = tempfile::TempDir::new().unwrap();
    let rogue_spec = outside_dir.path().join("rogue.unit.spec");
    fs::write(
        &rogue_spec,
        r#"
id: pricing/rogue
kind: function
intent:
  why: Escape the local library graph.
body:
  rust: "{ true }"
"#,
    )
    .unwrap();
    symlink(
        &rogue_spec,
        ecommerce_dir.join("units/pricing/rogue.unit.spec"),
    )
    .unwrap();

    let output = run_in(
        &ecommerce_dir,
        &[
            "plan",
            "validate",
            plan_path
                .strip_prefix(&ecommerce_dir)
                .unwrap()
                .to_str()
                .unwrap(),
            "--format",
            "json",
        ],
    );
    assert!(
        !output.status.success(),
        "expected external unit symlink to fail"
    );

    let json = parse_stdout_json(&output);
    assert_eq!(json["status"], "invalid");
    assert_eq!(json["errors"][0]["code"], "SPEC_PLAN_SYMLINK_ESCAPE");
    assert!(json.get("computed_impact").is_none(), "{json}");
    assert!(
        json["errors"][0].get("unit").is_none(),
        "unexpected unit mapping for escaped file: {json}"
    );
}

#[cfg(unix)]
#[test]
fn plan_export_rejects_symlinked_external_unit_in_library_graph() {
    use std::os::unix::fs::symlink;

    let (_temp_dir, ecommerce_dir, plan_path) =
        setup_m10_plan_fixture("plans/local.plan.spec", M10_MODIFY_PLAN);
    let outside_dir = tempfile::TempDir::new().unwrap();
    let rogue_spec = outside_dir.path().join("rogue.unit.spec");
    fs::write(
        &rogue_spec,
        r#"
id: pricing/rogue
kind: function
intent:
  why: Escape the local library graph.
body:
  rust: "{ true }"
"#,
    )
    .unwrap();
    symlink(
        &rogue_spec,
        ecommerce_dir.join("units/pricing/rogue.unit.spec"),
    )
    .unwrap();

    let output = run_in(
        &ecommerce_dir,
        &[
            "plan",
            "export",
            plan_path
                .strip_prefix(&ecommerce_dir)
                .unwrap()
                .to_str()
                .unwrap(),
        ],
    );
    assert!(
        !output.status.success(),
        "expected external unit symlink to fail plan export"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stdout.trim().is_empty(), "unexpected stdout: {stdout}");
    assert!(
        stderr.contains("symlink"),
        "expected symlink escape diagnostics, got: {stderr}"
    );
}

#[cfg(unix)]
#[test]
fn plan_validate_rejects_symlinked_external_molecule_test_in_library_graph() {
    use std::os::unix::fs::symlink;

    let (_temp_dir, ecommerce_dir, plan_path) =
        setup_m10_plan_fixture("plans/local.plan.spec", M10_MODIFY_PLAN);
    let outside_dir = tempfile::TempDir::new().unwrap();
    let rogue_test = outside_dir.path().join("rogue.test.spec");
    fs::write(
        &rogue_test,
        r#"
id: pricing/rogue_flow
covers:
  - pricing/apply_tax
body:
  rust: |
    {
        assert!(true);
    }
"#,
    )
    .unwrap();
    symlink(
        &rogue_test,
        ecommerce_dir.join("units/pricing/rogue.test.spec"),
    )
    .unwrap();

    let output = run_in(
        &ecommerce_dir,
        &[
            "plan",
            "validate",
            plan_path
                .strip_prefix(&ecommerce_dir)
                .unwrap()
                .to_str()
                .unwrap(),
            "--format",
            "json",
        ],
    );
    assert!(
        !output.status.success(),
        "expected external molecule test symlink to fail"
    );

    let json = parse_stdout_json(&output);
    assert_eq!(json["status"], "invalid");
    assert_eq!(json["errors"][0]["code"], "SPEC_PLAN_SYMLINK_ESCAPE");
    assert!(json.get("computed_impact").is_none(), "{json}");
}

#[test]
fn plan_validate_json_wraps_local_molecule_loader_failures() {
    let (_temp_dir, ecommerce_dir, plan_path) =
        setup_m10_plan_fixture("plans/local.plan.spec", M10_MODIFY_PLAN);
    fs::write(
        ecommerce_dir.join("units/pricing/broken.test.spec"),
        "not: valid: yaml: [unclosed",
    )
    .unwrap();

    let output = run_in(
        &ecommerce_dir,
        &[
            "plan",
            "validate",
            plan_path
                .strip_prefix(&ecommerce_dir)
                .unwrap()
                .to_str()
                .unwrap(),
            "--format",
            "json",
        ],
    );
    assert!(
        !output.status.success(),
        "expected invalid molecule test YAML to fail"
    );

    let json = parse_stdout_json(&output);
    assert_eq!(json["status"], "invalid");
    assert_eq!(json["errors"][0]["code"], "SPEC_YAML_PARSE");
    assert!(json.get("computed_impact").is_none(), "{json}");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !stderr.contains("Failed to load molecule tests"),
        "expected JSON envelope instead of raw loader fallback, got: {stderr}"
    );
}

#[test]
fn plan_validate_rejects_cross_library_change_unit() {
    let (_temp_dir, ecommerce_dir, plan_path) = setup_m10_plan_fixture(
        "plans/crosslib.plan.spec",
        r#"
id: crosslib-plan
intent:
  why: "Should fail because M10 is local-library only."
changes:
  - unit: shared::money/round
    action: modify
    acceptance:
      validate:
        - shared::money/round
"#,
    );

    let output = run_in(
        &ecommerce_dir,
        &[
            "plan",
            "validate",
            plan_path
                .strip_prefix(&ecommerce_dir)
                .unwrap()
                .to_str()
                .unwrap(),
            "--format",
            "json",
        ],
    );
    assert!(
        !output.status.success(),
        "expected cross-library plan to fail"
    );

    let json = parse_stdout_json(&output);
    assert_eq!(json["errors"][0]["code"], "SPEC_PLAN_CROSS_LIBRARY_UNIT");
}

#[test]
fn plan_export_matches_checked_in_fixture_and_preserves_spec_export_surface() {
    let (_temp_dir, ecommerce_dir) = copy_ecommerce_example();
    let plan_path = ecommerce_dir.join("plans/refactors/checkout-tax-refactor.plan.spec");

    let output = run_in(
        &ecommerce_dir,
        &[
            "plan",
            "export",
            plan_path
                .strip_prefix(&ecommerce_dir)
                .unwrap()
                .to_str()
                .unwrap(),
        ],
    );
    assert_output_success("plan export should succeed", &output);
    let actual = normalize_exported_at(parse_stdout_json(&output));
    let expected = fixture_json("plan-export-valid-mixed.json");
    assert_eq!(actual, expected);

    let spec_export = run_in(&ecommerce_dir, &["export", "units"]);
    assert_output_success("spec export should remain unchanged", &spec_export);
    let spec_export_json = parse_stdout_json(&spec_export);
    assert_eq!(spec_export_json["schema_version"], 5);
    assert!(spec_export_json.get("plan").is_none(), "{spec_export_json}");
    assert!(
        spec_export_json.get("units").is_some(),
        "{spec_export_json}"
    );
    assert!(
        spec_export_json.get("graph").is_some(),
        "{spec_export_json}"
    );
}

#[test]
fn plan_export_ignores_hidden_scratch_units_copy() {
    let (_temp_dir, ecommerce_dir) = copy_ecommerce_example();
    let plan_path = ecommerce_dir.join("plans/refactors/checkout-tax-refactor.plan.spec");
    add_hidden_scratch_units_copy(&ecommerce_dir);

    let output = run_in(
        &ecommerce_dir,
        &[
            "plan",
            "export",
            plan_path
                .strip_prefix(&ecommerce_dir)
                .unwrap()
                .to_str()
                .unwrap(),
        ],
    );
    assert_output_success(
        "plan export should ignore hidden scratch units copies",
        &output,
    );
    let actual = normalize_exported_at(parse_stdout_json(&output));
    let expected = fixture_json("plan-export-valid-mixed.json");
    assert_eq!(actual, expected);
}

#[test]
fn status_benchmark_root_contract_matches_frozen_fixture() {
    let (_temp_dir, repo_dir) = copy_benchmark_repo_fixture();

    let output = run_in(
        &repo_dir,
        &["status", "examples/ecommerce/units", "--format", "json"],
    );
    assert_output_success("benchmark-root status should succeed", &output);
    let json = parse_stdout_json(&output);

    assert_eq!(json["schema_version"], 5);
    assert_eq!(json["benchmarks"][0]["path_scope"], "full");
    assert_eq!(json["benchmarks"][0]["benchmark_status"], "passing");
    assert!(
        json["benchmarks"][0]["cases"]
            .as_array()
            .unwrap()
            .iter()
            .all(|case| case["counts_as_supported_positive"] == Value::Bool(true))
    );

    assert_contract_matches_fixture(
        normalize_status_contract_json(json),
        "status-ecommerce-full.json",
    );
}

#[test]
fn status_namespace_contract_matches_frozen_fixture() {
    let (_temp_dir, repo_dir) = copy_benchmark_repo_fixture();

    let output = run_in(
        &repo_dir,
        &[
            "status",
            "examples/ecommerce/units/pricing",
            "--format",
            "json",
        ],
    );
    assert!(
        !output.status.success(),
        "namespace status should stay non-green when no library root is discovered"
    );
    let json = parse_stdout_json(&output);
    let benchmark = &json["benchmarks"][0];

    assert_eq!(json["loader_errors"][0]["code"], "SPEC_NO_LIBRARY_ROOTS");
    assert_eq!(benchmark["path_scope"], "partial");
    assert!(
        benchmark["cases"]
            .as_array()
            .unwrap()
            .iter()
            .all(|case| case["counts_as_supported_positive"] == Value::Bool(false))
    );
    assert!(benchmark.get("projection_digest").is_none());
    assert!(benchmark.get("readability_review_status").is_none());

    assert_contract_matches_fixture(
        normalize_status_contract_json(json),
        "status-ecommerce-pricing-partial-full.json",
    );
}

#[test]
fn status_single_file_contract_matches_frozen_fixture() {
    let (_temp_dir, repo_dir) = copy_benchmark_repo_fixture();

    let output = run_in(
        &repo_dir,
        &[
            "status",
            "examples/ecommerce/units/pricing/apply_discount.unit.spec",
            "--format",
            "json",
        ],
    );
    assert_output_success("single-file status should succeed", &output);
    let json = parse_stdout_json(&output);
    let benchmark = &json["benchmarks"][0];

    assert_eq!(benchmark["path_scope"], "partial");
    assert_eq!(benchmark["cases"].as_array().unwrap().len(), 1);
    assert_eq!(
        benchmark["cases"][0]["counts_as_supported_positive"],
        Value::Bool(false)
    );
    assert!(benchmark.get("benchmark_status").is_none());
    assert!(benchmark.get("projection_digest").is_none());
    assert!(benchmark.get("summary").is_none());

    assert_contract_matches_fixture(
        normalize_status_contract_json(json),
        "status-apply-discount-partial-full.json",
    );
}

fn normalized_repo_root_status_contract_from_fixture_root(root: &Path) -> Value {
    let (_temp_dir, repo_dir) = copy_benchmark_repo_fixture_from_root(root);
    let output = run_in(&repo_dir, &["status", ".", "--format", "json"]);
    assert!(
        !output.status.success(),
        "repo-root status should stay non-green when broad inventory includes untested work"
    );
    normalize_status_contract_json(parse_stdout_json(&output))
}

#[test]
fn status_repo_root_fixture_copy_ignores_untracked_shared_spec_pricing_passports() {
    let (_temp_dir, source_root) = setup_benchmark_source_repo_fixture();
    let clean_contract = normalized_repo_root_status_contract_from_fixture_root(&source_root);

    let shared_spec_dir = source_root.join("examples/shared-spec");
    let output = run_in(
        &shared_spec_dir,
        &["test", "units/pricing/apply_discount.unit.spec"],
    );
    assert_output_success(
        "shared-spec apply_discount test should generate an ignored pricing passport",
        &output,
    );

    let ignored_passport =
        source_root.join("examples/shared-spec/units/pricing/apply_discount.spec.passport.json");
    assert!(
        ignored_passport.exists(),
        "{ignored_passport:?} should exist"
    );

    let ignored_status = run_git(&source_root, &["status", "--short", "--ignored"]);
    assert_output_success(
        "git status --short --ignored should succeed for source fixture repo",
        &ignored_status,
    );
    let ignored_stdout = String::from_utf8_lossy(&ignored_status.stdout);
    assert!(
        ignored_stdout
            .contains("!! examples/shared-spec/units/pricing/apply_discount.spec.passport.json"),
        "expected ignored pricing passport to stay untracked, got:\n{ignored_stdout}"
    );

    let contaminated_contract =
        normalized_repo_root_status_contract_from_fixture_root(&source_root);
    assert_eq!(contaminated_contract, clean_contract);
}

#[test]
fn status_repo_root_contract_matches_frozen_fixture() {
    let (_temp_dir, repo_dir) = copy_benchmark_repo_fixture();

    let output = run_in(&repo_dir, &["status", ".", "--format", "json"]);
    assert!(
        !output.status.success(),
        "repo-root status should stay non-green when broad inventory includes untested work"
    );
    let json = parse_stdout_json(&output);

    assert_eq!(json["scope_authority"], "inventory_only");
    assert!(
        json["benchmarks"]
            .as_array()
            .unwrap()
            .iter()
            .any(|benchmark| benchmark["benchmark_id"] == "BENCH-SERVICE"),
        "repo-root inventory should preserve reserved benchmark visibility"
    );

    assert_contract_matches_fixture(
        normalize_status_contract_json(json),
        "status-repo-root-full.json",
    );
}

#[test]
fn export_benchmark_root_contract_matches_frozen_fixture() {
    let (_temp_dir, repo_dir) = copy_benchmark_repo_fixture();

    let output = run_in(&repo_dir, &["export", "examples/ecommerce/units"]);
    assert_output_success("benchmark-root export should succeed", &output);
    let json = parse_stdout_json(&output);

    assert_eq!(json["schema_version"], 5);
    assert_eq!(json["benchmarks"][0]["path_scope"], "full");
    assert_eq!(json["benchmarks"][0]["benchmark_status"], "passing");
    assert!(
        json["benchmarks"][0]["cases"]
            .as_array()
            .unwrap()
            .iter()
            .all(|case| case["counts_as_supported_positive"] == Value::Bool(true))
    );

    assert_contract_matches_fixture(
        normalize_export_contract_json(json),
        "export-ecommerce-full.json",
    );
}

#[test]
fn benchmark_read_surfaces_do_not_backfill_missing_supported_seam_descriptor_ids() {
    let (_temp_dir, repo_dir) = copy_benchmark_repo_fixture();
    let data_passport_path =
        repo_dir.join("examples/ecommerce/units/pricing/pricing_quote.spec.passport.json");
    let sum_passport_path =
        repo_dir.join("examples/ecommerce/units/pricing/discount_strategy.spec.passport.json");

    let stored_data_review = clear_passport_semantic_review_descriptor_id(&data_passport_path);
    let stored_sum_review = clear_passport_semantic_review_descriptor_id(&sum_passport_path);
    assert_eq!(stored_data_review["descriptor_id"], Value::Null);
    assert_eq!(stored_sum_review["descriptor_id"], Value::Null);

    let status_output = run_in(
        &repo_dir,
        &["status", "examples/ecommerce/units", "--format", "json"],
    );
    assert_output_success(
        "benchmark-root status should not backfill missing seam descriptor ids",
        &status_output,
    );
    let status_json = parse_stdout_json(&status_output);
    let status_benchmark = status_json["benchmarks"]
        .as_array()
        .unwrap()
        .iter()
        .find(|benchmark| benchmark["benchmark_id"] == "BENCH-ECOM")
        .unwrap();
    assert_eq!(status_benchmark["accounting_status"], "invalid");
    assert_eq!(status_benchmark["benchmark_status"], "invalid");

    for unit_id in ["pricing/pricing_quote", "pricing/discount_strategy"] {
        let status_unit = status_units(&status_json)
            .iter()
            .find(|unit| unit["id"] == unit_id)
            .unwrap();
        assert_eq!(status_unit["semantic_review"]["descriptor_id"], Value::Null);
        assert_eq!(
            status_unit["category_qualification"]["claim_status"],
            "unqualified"
        );
        assert_eq!(
            status_unit["category_qualification"]["reason_code"],
            "descriptor_id_missing"
        );

        let benchmark_case = status_benchmark["cases"]
            .as_array()
            .unwrap()
            .iter()
            .find(|case| case["carrier_id"] == unit_id)
            .unwrap();
        assert_eq!(
            benchmark_case["category_qualification"]["claim_status"],
            "unqualified"
        );
        assert_eq!(
            benchmark_case["category_qualification"]["reason_code"],
            "descriptor_id_missing"
        );
        assert_eq!(
            benchmark_case["counts_as_supported_positive"],
            Value::Bool(false)
        );
    }

    let export_output = run_in(&repo_dir, &["export", "examples/ecommerce/units"]);
    assert_output_success(
        "benchmark-root export should not backfill missing seam descriptor ids",
        &export_output,
    );
    let export_json = parse_stdout_json(&export_output);
    let export_benchmark = export_json["benchmarks"]
        .as_array()
        .unwrap()
        .iter()
        .find(|benchmark| benchmark["benchmark_id"] == "BENCH-ECOM")
        .unwrap();
    assert_eq!(export_benchmark["accounting_status"], "invalid");
    assert_eq!(export_benchmark["benchmark_status"], "invalid");

    for unit_id in ["pricing/pricing_quote", "pricing/discount_strategy"] {
        let projected_unit = export_json["projected_units"]
            .as_array()
            .unwrap()
            .iter()
            .find(|unit| unit["id"] == unit_id)
            .unwrap();
        assert_eq!(
            projected_unit["semantic_review"]["descriptor_id"],
            Value::Null
        );
        assert_eq!(
            projected_unit["category_qualification"]["claim_status"],
            "unqualified"
        );
        assert_eq!(
            projected_unit["category_qualification"]["reason_code"],
            "descriptor_id_missing"
        );

        let benchmark_case = export_benchmark["cases"]
            .as_array()
            .unwrap()
            .iter()
            .find(|case| case["carrier_id"] == unit_id)
            .unwrap();
        assert_eq!(
            benchmark_case["category_qualification"]["claim_status"],
            "unqualified"
        );
        assert_eq!(
            benchmark_case["category_qualification"]["reason_code"],
            "descriptor_id_missing"
        );
        assert_eq!(
            benchmark_case["counts_as_supported_positive"],
            Value::Bool(false)
        );
    }

    let snapshot_output = run_in(&repo_dir, &["benchmark", "snapshot", "BENCH-ECOM"]);
    assert_output_success(
        "benchmark snapshot should not backfill missing seam descriptor ids",
        &snapshot_output,
    );
    let snapshot: Value = serde_json::from_str(
        &fs::read_to_string(repo_dir.join("benchmarks/snapshots/BENCH-ECOM.snapshot.json"))
            .unwrap(),
    )
    .unwrap();
    let snapshot_projection = &snapshot["projection"];
    assert_eq!(snapshot_projection["accounting_status"], "invalid");
    assert_eq!(snapshot_projection["benchmark_status"], "invalid");

    for unit_id in ["pricing/pricing_quote", "pricing/discount_strategy"] {
        let benchmark_case = snapshot_projection["cases"]
            .as_array()
            .unwrap()
            .iter()
            .find(|case| case["carrier_id"] == unit_id)
            .unwrap();
        assert_eq!(
            benchmark_case["category_qualification"]["claim_status"],
            "unqualified"
        );
        assert_eq!(
            benchmark_case["category_qualification"]["reason_code"],
            "descriptor_id_missing"
        );
        assert_eq!(
            benchmark_case["counts_as_supported_positive"],
            Value::Bool(false)
        );
    }
}

#[test]
fn export_repo_root_contract_matches_frozen_fixture() {
    let (_temp_dir, repo_dir) = copy_benchmark_repo_fixture();

    let output = run_in(&repo_dir, &["export", "."]);
    assert!(
        !output.status.success(),
        "repo-root export should fail with a stable unsupported-scope contract"
    );
    let json = parse_stdout_json(&output);

    assert_eq!(json["errors"][0]["code"], "SPEC_UNSUPPORTED_SCOPE");
    assert!(json.get("benchmarks").is_none());
    assert!(json.get("units").is_none());
    assert!(json.get("passports").is_none());

    assert_contract_matches_fixture(
        normalize_export_contract_json(json),
        "export-repo-root-unsupported-scope.json",
    );
}

#[test]
fn benchmark_snapshot_writes_seeded_positive_negative_and_reserved_outputs() {
    let (_temp_dir, repo_dir) = copy_benchmark_repo_fixture();

    let ecom_output = run_in(&repo_dir, &["benchmark", "snapshot", "BENCH-ECOM"]);
    assert_output_success("BENCH-ECOM snapshot should succeed", &ecom_output);
    let crosslib_output = run_in(&repo_dir, &["benchmark", "snapshot", "BENCH-CROSSLIB"]);
    assert_output_success("BENCH-CROSSLIB snapshot should succeed", &crosslib_output);
    let service_output = run_in(&repo_dir, &["benchmark", "snapshot", "BENCH-SERVICE"]);
    assert_output_success("BENCH-SERVICE snapshot should succeed", &service_output);

    let ecom_snapshot: Value = serde_json::from_str(
        &fs::read_to_string(repo_dir.join("benchmarks/snapshots/BENCH-ECOM.snapshot.json"))
            .unwrap(),
    )
    .unwrap();
    assert_eq!(
        ecom_snapshot["projection"]["benchmark_status"],
        "incomplete"
    );
    assert_eq!(
        ecom_snapshot["projection"]["readability_review_status"],
        "stale"
    );

    let crosslib_snapshot: Value = serde_json::from_str(
        &fs::read_to_string(repo_dir.join("benchmarks/snapshots/BENCH-CROSSLIB.snapshot.json"))
            .unwrap(),
    )
    .unwrap();
    assert_eq!(
        crosslib_snapshot["projection"]["kind"],
        "companion_negative_proof"
    );
    assert!(
        crosslib_snapshot["projection"]["cases"]
            .as_array()
            .unwrap()
            .iter()
            .all(|case| case["counts_as_supported_positive"] == Value::Bool(false))
    );

    let service_snapshot: Value = serde_json::from_str(
        &fs::read_to_string(repo_dir.join("benchmarks/snapshots/BENCH-SERVICE.snapshot.json"))
            .unwrap(),
    )
    .unwrap();
    assert_eq!(
        service_snapshot["projection"]["benchmark_status"],
        "invalid"
    );
    assert_eq!(service_snapshot["projection"]["gate_status"], "open");
    assert_eq!(
        service_snapshot["projection"]["readability_review_status"],
        "stale"
    );
}

#[test]
fn benchmark_snapshot_preserves_existing_artifact_when_projection_is_unchanged() {
    let (_temp_dir, repo_dir) = copy_benchmark_repo_fixture();

    for benchmark_id in ["BENCH-ECOM", "BENCH-CROSSLIB"] {
        let output = run_in(&repo_dir, &["benchmark", "snapshot", benchmark_id]);
        assert_output_success("initial benchmark snapshot should succeed", &output);
    }

    let ecom_path = repo_dir.join("benchmarks/snapshots/BENCH-ECOM.snapshot.json");
    let crosslib_path = repo_dir.join("benchmarks/snapshots/BENCH-CROSSLIB.snapshot.json");
    let ecom_before = fs::read_to_string(&ecom_path).unwrap();
    let crosslib_before = fs::read_to_string(&crosslib_path).unwrap();

    sleep(Duration::from_secs(1));

    for benchmark_id in ["BENCH-ECOM", "BENCH-CROSSLIB"] {
        let output = run_in(&repo_dir, &["benchmark", "snapshot", benchmark_id]);
        assert_output_success("repeat benchmark snapshot should succeed", &output);
    }

    let ecom_after = fs::read_to_string(&ecom_path).unwrap();
    let crosslib_after = fs::read_to_string(&crosslib_path).unwrap();

    assert_eq!(ecom_after, ecom_before);
    assert_eq!(crosslib_after, crosslib_before);
}

#[test]
fn status_json_surfaces_invalid_benchmark_registry_root_machine_readably() {
    let (_temp_dir, repo_dir) = copy_benchmark_repo_fixture();
    replace_in_file(
        &repo_dir.join("benchmarks/labels.json"),
        "\"root\": \"examples/ecommerce/units\"",
        "\"root\": \"../outside\"",
    );

    let output = run_in(
        &repo_dir,
        &["status", "examples/ecommerce", "--format", "json"],
    );
    assert!(
        !output.status.success(),
        "invalid benchmark registry should make status non-green"
    );
    let json = parse_stdout_json(&output);
    let loader_errors = json["loader_errors"].as_array().unwrap();
    let mut normalized_loader_errors = json["loader_errors"].clone();
    normalized_loader_errors[0]["path"] =
        Value::String("/tmp/benchmark-fixture/benchmarks/labels.json".to_string());

    assert_eq!(json["schema_version"], 5);
    assert_eq!(loader_errors.len(), 1);
    assert_eq!(
        normalized_loader_errors,
        fixture_json("benchmarks/status-invalid-benchmark-registry-loader-errors.json")
    );
}

#[test]
fn validate_json_accepts_kind_data_without_placeholder_body() {
    let (_temp_dir, project_dir) = setup_m12_data_seam_project();

    let output = run_in(
        &project_dir,
        &[
            "validate",
            "units/pricing/pricing_quote.unit.spec",
            "--format",
            "json",
        ],
    );
    assert_output_success("validate should accept kind:data without body", &output);

    let json = parse_stdout_json(&output);
    assert_eq!(json["status"], "valid");
}

#[test]
fn validate_json_accepts_kind_data_with_empty_placeholder_body() {
    let (_temp_dir, project_dir) = setup_m12_data_seam_project();
    write_spec(
        &project_dir.join("units"),
        "pricing/checkout_quote.unit.spec",
        r#"
id: pricing/checkout_quote
kind: data
spec_version: "0.3.0"
intent:
  why: Quote a checkout total from subtotal plus discount and tax rates.
body: {}
data:
  fields:
    subtotal:
      type: rust_decimal::Decimal
    discount_rate:
      type: rust_decimal::Decimal
    tax_rate:
      type: rust_decimal::Decimal
constructors:
  - id: new
    intent:
      why: Create a quote from explicit subtotal and rates.
    contract:
      inputs:
        subtotal: rust_decimal::Decimal
        discount_rate: rust_decimal::Decimal
        tax_rate: rust_decimal::Decimal
    initializes:
      subtotal: subtotal
      discount_rate: discount_rate
      tax_rate: tax_rate
methods:
  - id: discounted_subtotal
    intent:
      why: Return the discounted subtotal before tax.
    receiver: shared_ref
    contract:
      returns: rust_decimal::Decimal
    deps:
      - pricing/apply_discount
    lowering:
      rust:
        body: |
          {
              apply_discount(self.subtotal, self.discount_rate)
          }
  - id: total
    intent:
      why: Return the final checkout total after discount and tax.
    receiver: shared_ref
    contract:
      returns: rust_decimal::Decimal
    deps:
      - pricing/apply_tax
    lowering:
      rust:
        body: |
          {
              apply_tax(self.discounted_subtotal(), self.tax_rate)
          }
local_tests:
  - id: discounted_subtotal_basic
    expect: CheckoutQuote::new(rust_decimal::Decimal::new(10000, 2), rust_decimal::Decimal::new(10, 2), rust_decimal::Decimal::new(725, 4)).discounted_subtotal() == rust_decimal::Decimal::new(9000, 2)
  - id: total_basic
    expect: CheckoutQuote::new(rust_decimal::Decimal::new(10000, 2), rust_decimal::Decimal::new(10, 2), rust_decimal::Decimal::new(725, 4)).total() == rust_decimal::Decimal::new(96525, 3)
links:
  molecule_tests:
    - pricing/checkout_flow
backends:
  rust:
    derives:
      - Clone
      - Debug
      - PartialEq
"#,
    );

    let output = run_in(
        &project_dir,
        &[
            "validate",
            "units/pricing/checkout_quote.unit.spec",
            "--format",
            "json",
        ],
    );
    assert_output_success(
        "validate should accept kind:data with empty placeholder body",
        &output,
    );

    let json = parse_stdout_json(&output);
    assert_eq!(json["status"], "valid");
}

#[test]
fn validate_json_rejects_kind_data_with_shared_body_as_semantic_error() {
    let (_temp_dir, project_dir) = setup_m12_data_seam_project();
    write_spec(
        &project_dir.join("units"),
        "pricing/checkout_quote.unit.spec",
        r#"
id: pricing/checkout_quote
kind: data
spec_version: "0.3.0"
intent:
  why: Quote a checkout total from subtotal plus discount and tax rates.
body:
  rust: |
    {
        unreachable!("escape hatch")
    }
data:
  fields:
    subtotal:
      type: rust_decimal::Decimal
    discount_rate:
      type: rust_decimal::Decimal
    tax_rate:
      type: rust_decimal::Decimal
constructors:
  - id: new
    intent:
      why: Create a quote from explicit subtotal and rates.
    contract:
      inputs:
        subtotal: rust_decimal::Decimal
        discount_rate: rust_decimal::Decimal
        tax_rate: rust_decimal::Decimal
    initializes:
      subtotal: subtotal
      discount_rate: discount_rate
      tax_rate: tax_rate
methods:
  - id: discounted_subtotal
    intent:
      why: Return the discounted subtotal before tax.
    receiver: shared_ref
    contract:
      returns: rust_decimal::Decimal
    deps:
      - pricing/apply_discount
    lowering:
      rust:
        body: |
          {
              apply_discount(self.subtotal, self.discount_rate)
          }
  - id: total
    intent:
      why: Return the final checkout total after discount and tax.
    receiver: shared_ref
    contract:
      returns: rust_decimal::Decimal
    deps:
      - pricing/apply_tax
    lowering:
      rust:
        body: |
          {
              apply_tax(self.discounted_subtotal(), self.tax_rate)
          }
"#,
    );

    let output = run_in(
        &project_dir,
        &[
            "validate",
            "units/pricing/checkout_quote.unit.spec",
            "--format",
            "json",
        ],
    );
    assert!(!output.status.success(), "validate should fail");

    let json = parse_stdout_json(&output);
    assert_eq!(json["status"], "invalid");
    let errors = json["errors"].as_array().unwrap();
    assert!(
        errors
            .iter()
            .any(|error| error["code"] == "SPEC_SEMANTIC_VALIDATION"),
        "{json}"
    );
    assert!(
        errors
            .iter()
            .all(|error| error["code"] != "SPEC_YAML_PARSE"),
        "{json}"
    );
}

#[test]
fn data_seam_single_file_test_writes_passport_and_leaves_gate_open_without_molecule_proof() {
    let (_temp_dir, project_dir) = setup_m12_data_seam_project();

    let output = run_in(
        &project_dir,
        &[
            "test",
            "units/pricing/pricing_quote.unit.spec",
            "--crate-root",
            ".",
        ],
    );
    assert_output_success("single-file data seam test should succeed", &output);

    let passport_path = project_dir.join("units/pricing/pricing_quote.spec.passport.json");
    assert!(passport_path.exists(), "expected data seam passport");
    let passport = read_passport_json(&passport_path);
    assert_eq!(passport["kind"], "data");
    assert_eq!(
        passport["data"]["fields"]["subtotal"]["type"],
        "rust_decimal::Decimal"
    );
    assert_eq!(passport["evidence"]["test_results"][0]["status"], "pass");
    assert_eq!(passport["escape_hatch_gate"]["status"], "open");
    assert_eq!(
        passport["escape_hatch_gate"]["missing_surfaces"],
        serde_json::json!(["molecule"])
    );
    assert!(
        passport["contract_hash"]
            .as_str()
            .unwrap()
            .starts_with("sha256:"),
        "{passport}"
    );

    let status_output = run_in(&project_dir, &["status", "units", "--format", "json"]);
    let status_json = parse_stdout_json(&status_output);
    let units = status_units(&status_json);
    let pricing_quote = units
        .iter()
        .find(|entry| entry["id"] == "pricing/pricing_quote")
        .expect("expected pricing_quote status row");
    assert_eq!(pricing_quote["status"], "incomplete");
    assert_eq!(
        pricing_quote["reason"],
        "missing required escape-hatch proof: molecule"
    );
}

#[test]
fn data_seam_status_stale_after_intent_change() {
    if !cargo_available() {
        return;
    }

    let (_temp_dir, project_dir) = setup_m12_data_seam_project();

    let output = run_in(
        &project_dir,
        &[
            "test",
            "units/pricing/pricing_quote.unit.spec",
            "--crate-root",
            ".",
        ],
    );
    assert_output_success("single-file data seam test should succeed", &output);

    let spec_path = project_dir.join("units/pricing/pricing_quote.unit.spec");
    let updated = fs::read_to_string(&spec_path).unwrap().replace(
        "Quote a checkout total from subtotal plus discount and tax rates.",
        "Quote a checkout total with updated intent wording.",
    );
    fs::write(&spec_path, updated).unwrap();

    let status_output = run_in(&project_dir, &["status", "units", "--format", "json"]);
    assert!(
        !status_output.status.success(),
        "status should be non-zero for stale data seam"
    );

    let status_json = parse_stdout_json(&status_output);
    let units = status_units(&status_json);
    let pricing_quote = units
        .iter()
        .find(|entry| entry["id"] == "pricing/pricing_quote")
        .expect("expected pricing_quote status row");
    assert_eq!(pricing_quote["status"], "stale");
    assert_eq!(
        pricing_quote["reason"],
        "authored truth changed since last test"
    );
}

#[test]
fn sum_seam_cli_validate_build_status_export_round_trip() {
    if !cargo_available() {
        return;
    }

    let (_temp_dir, project_dir) = setup_m13_sum_seam_project();

    let validate_output = run_in(
        &project_dir,
        &[
            "validate",
            "units/pricing/checkout_status.unit.spec",
            "--format",
            "json",
        ],
    );
    assert_output_success("validate should accept sum seam", &validate_output);

    let build_output = run_in(
        &project_dir,
        &[
            "build",
            "units",
            "--output",
            "src/generated",
            "--crate-root",
            ".",
        ],
    );
    assert_output_success(
        "build should succeed for mixed function/data/sum tree",
        &build_output,
    );

    let test_output = run_in(
        &project_dir,
        &[
            "test",
            "units",
            "--output",
            "src/generated",
            "--crate-root",
            ".",
        ],
    );
    assert_output_success(
        "directory test should succeed for mixed sum seam tree",
        &test_output,
    );

    let passport_path = project_dir.join("units/pricing/checkout_status.spec.passport.json");
    assert!(passport_path.exists(), "expected sum seam passport");
    let passport_raw = read_passport(&passport_path);
    let pending_pos = passport_raw.find("\"pending\": {").unwrap();
    let quoted_total_pos = passport_raw.find("\"quoted_total\": {").unwrap();
    let failed_pos = passport_raw.find("\"failed\": {").unwrap();
    assert!(
        pending_pos < quoted_total_pos && quoted_total_pos < failed_pos,
        "expected sum variants to preserve authored order: {passport_raw}"
    );
    let passport = read_passport_json(&passport_path);
    assert_eq!(passport["kind"], "sum");
    assert_eq!(
        passport["sum"]["variants"]["quoted_total"]["fields"]["subtotal"]["type"],
        "i32"
    );
    assert_eq!(passport["methods"][0]["id"], "label");
    assert_eq!(passport["methods"][1]["id"], "total");
    assert_eq!(
        passport["backends"]["rust"]["derives"],
        serde_json::json!(["Clone", "Debug", "PartialEq"])
    );
    assert_eq!(
        passport["deps"],
        serde_json::json!(["pricing/apply_discount", "pricing/apply_tax"])
    );
    assert_eq!(passport["evidence"]["test_results"][0]["status"], "pass");
    assert!(
        passport["contract_hash"]
            .as_str()
            .unwrap()
            .starts_with("sha256:"),
        "{passport}"
    );
    assert_eq!(passport["escape_hatch_gate"]["status"], "open");
    assert_eq!(
        passport["escape_hatch_gate"]["missing_surfaces"],
        serde_json::json!(["molecule"])
    );

    let status_output = run_in(&project_dir, &["status", "units", "--format", "json"]);
    let status_json = parse_stdout_json(&status_output);
    let sum_rows = status_units(&status_json)
        .iter()
        .filter(|entry| entry["id"] == "pricing/checkout_status")
        .collect::<Vec<_>>();
    assert_eq!(
        sum_rows.len(),
        1,
        "expected one status row for the sum seam"
    );
    assert_eq!(sum_rows[0]["status"], "incomplete");
    assert_eq!(
        sum_rows[0]["reason"],
        "missing required escape-hatch proof: molecule"
    );

    let export_output = run_in(&project_dir, &["export", "units"]);
    assert_output_success(
        "export should succeed for mixed sum seam project",
        &export_output,
    );
    let export_stdout = String::from_utf8_lossy(&export_output.stdout);
    let pending_pos = export_stdout.find("\"pending\": {").unwrap();
    let quoted_total_pos = export_stdout.find("\"quoted_total\": {").unwrap();
    let failed_pos = export_stdout.find("\"failed\": {").unwrap();
    assert!(
        pending_pos < quoted_total_pos && quoted_total_pos < failed_pos,
        "expected export sum variants to preserve authored order: {export_stdout}"
    );
    let export_json = parse_stdout_json(&export_output);
    let exported_units = export_json["units"].as_array().unwrap();
    let exported_sum_units = exported_units
        .iter()
        .filter(|entry| entry["id"] == "pricing/checkout_status")
        .collect::<Vec<_>>();
    assert_eq!(
        exported_sum_units.len(),
        1,
        "expected one export entry for the sum seam"
    );
    let exported_sum = exported_sum_units[0];
    assert_eq!(exported_sum["kind"], "sum");
    assert_eq!(
        exported_sum["sum"]["variants"]["quoted_total"]["fields"]["tax_rate"]["type"],
        "i32"
    );
    assert_eq!(
        exported_sum["deps"],
        serde_json::json!([
            {"library": null, "id": "pricing/apply_discount"},
            {"library": null, "id": "pricing/apply_tax"}
        ])
    );
    assert_eq!(
        export_json["passports"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|entry| entry["id"] == "pricing/checkout_status")
            .count(),
        1,
        "expected one passport entry for the sum seam in export"
    );
    let exported_passport = export_json["passports"]
        .as_array()
        .unwrap()
        .iter()
        .find(|entry| entry["id"] == "pricing/checkout_status")
        .unwrap();
    assert_eq!(exported_passport["escape_hatch_gate"]["status"], "open");
    assert_eq!(
        exported_passport["escape_hatch_gate"]["missing_surfaces"],
        serde_json::json!(["molecule"])
    );
}

#[test]
fn sum_seam_validate_rejects_projected_invalid_rust_identifier_as_semantic_error() {
    let (_temp_dir, project_dir) = setup_m13_sum_seam_project();
    write_spec(
        &project_dir.join("units"),
        "pricing/checkout_status.unit.spec",
        &format!(
            r#"
id: pricing/checkout_status
kind: sum
intent:
  why: Exercise M13 projected identifier validation.
spec_version: "{AUTHORED_SPEC_VERSION}"
sum:
  variants:
    self_: {{}}
    quoted_total:
      fields:
        subtotal:
          type: i32
methods:
  - id: rounded_total
    intent:
      why: Return the rounded subtotal for quoted totals.
    receiver: shared_ref
    contract:
      returns: i32
    lowering:
      rust:
        body: |
          {{
              match self {{
                  CheckoutStatus::Self => 0,
                  CheckoutStatus::QuotedTotal {{ subtotal }} => *subtotal,
              }}
          }}
local_tests:
  - id: quoted_total_rounds
    expect: "CheckoutStatus::QuotedTotal {{ subtotal: 2 }}.rounded_total() == 2"
backends:
  rust:
    derives:
      - Clone
      - Debug
      - PartialEq
"#
        ),
    );

    let output = run_in(
        &project_dir,
        &[
            "validate",
            "units/pricing/checkout_status.unit.spec",
            "--format",
            "json",
        ],
    );
    assert!(!output.status.success(), "validate should fail");
    assert!(
        output.stderr.is_empty(),
        "expected no stderr, got: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let json = parse_stdout_json(&output);
    assert_eq!(json["status"], "invalid");
    let errors = json["errors"].as_array().unwrap();
    assert_eq!(errors.len(), 1, "expected one semantic error");

    let error = &errors[0];
    assert_eq!(error["code"], "SPEC_SEMANTIC_VALIDATION");
    assert_eq!(error["unit"], "pricing/checkout_status");
    assert_eq!(error["path"], "units/pricing/checkout_status.unit.spec");
    let message = error["message"].as_str().unwrap();
    assert!(
        message.contains("sum.variants[0].id"),
        "unexpected error payload: {error:?}"
    );
    assert!(
        message.contains("'Self'"),
        "unexpected error payload: {error:?}"
    );
}

#[test]
fn sum_seam_single_file_test_accepts_cross_library_method_dep_with_cargo_alias() {
    if !cargo_available() {
        return;
    }

    let fixture = setup_m9_repo_fixture();
    fs::write(
        fixture.app_root.join("spec.toml"),
        "[libraries]\nshared = \"../shared-spec\"\n",
    )
    .unwrap();
    write_m9_app_cargo_toml(&fixture.app_root, &["shared"]);
    write_m9_shared_round_crate_fixture(&fixture);
    write_m13_sum_seam(
        &fixture.app_root.join("units"),
        "pricing/checkout_status.unit.spec",
        "pricing/checkout_status",
        &["shared::money/round"],
    );
    write_spec(
        &fixture.shared_root.join("units"),
        "money/round.unit.spec",
        &format!(
            r#"
id: money/round
kind: function
intent:
  why: Round a subtotal.
spec_version: "{AUTHORED_SPEC_VERSION}"
contract:
  inputs:
    value: i32
  returns: i32
body:
  rust: |
    {{
        value
    }}
"#
        ),
    );

    let output = run_in(
        &fixture.app_root,
        &[
            "test",
            "units/pricing/checkout_status.unit.spec",
            "--crate-root",
            ".",
        ],
    );
    assert_output_success(
        "single-file spec test should accept cross-library method deps for sum seams",
        &output,
    );
    assert!(
        fixture
            .app_root
            .join("units/pricing/checkout_status.spec.passport.json")
            .exists(),
        "expected sum seam passport after single-file test"
    );
}

#[test]
fn sum_seam_status_stale_after_intent_change() {
    if !cargo_available() {
        return;
    }

    let (_temp_dir, project_dir) = setup_m13_sum_seam_project();

    let output = run_in(
        &project_dir,
        &[
            "test",
            "units/pricing/checkout_status.unit.spec",
            "--crate-root",
            ".",
        ],
    );
    assert_output_success("single-file sum seam test should succeed", &output);

    let spec_path = project_dir.join("units/pricing/checkout_status.unit.spec");
    let updated = fs::read_to_string(&spec_path).unwrap().replace(
        "Track checkout state as a seam-owned enum.",
        "Track checkout state with updated intent wording.",
    );
    fs::write(&spec_path, updated).unwrap();

    let status_output = run_in(&project_dir, &["status", "units", "--format", "json"]);
    assert!(
        !status_output.status.success(),
        "status should be non-zero for stale sum seam"
    );

    let status_json = parse_stdout_json(&status_output);
    let checkout_status = status_units(&status_json)
        .iter()
        .find(|entry| entry["id"] == "pricing/checkout_status")
        .expect("expected checkout_status status row");
    assert_eq!(checkout_status["status"], "stale");
    assert_eq!(
        checkout_status["reason"],
        "authored truth changed since last test"
    );
}

#[test]
fn validate_json_reports_missing_data_method_dep() {
    let (_temp_dir, project_dir) = setup_m12_data_seam_project();
    write_spec(
        &project_dir.join("units"),
        "pricing/checkout_quote.unit.spec",
        r#"
id: pricing/checkout_quote
kind: data
spec_version: "0.3.0"
intent:
  why: Quote a checkout total from subtotal plus discount and tax rates.
data:
  fields:
    subtotal:
      type: rust_decimal::Decimal
constructors:
  - id: new
    intent:
      why: Create a quote from explicit subtotal.
    contract:
      inputs:
        subtotal: rust_decimal::Decimal
    initializes:
      subtotal: subtotal
methods:
  - id: total
    intent:
      why: Return the final checkout total.
    receiver: shared_ref
    contract:
      returns: rust_decimal::Decimal
    deps:
      - pricing/definitely_missing
    lowering:
      rust:
        body: |
          {
              self.subtotal
          }
"#,
    );

    let output = run_in(
        &project_dir,
        &[
            "validate",
            "units/pricing/checkout_quote.unit.spec",
            "--format",
            "json",
        ],
    );
    assert!(
        !output.status.success(),
        "validate should fail for missing data method dep"
    );
    let json = parse_stdout_json(&output);
    assert_eq!(json["status"], "invalid");
    let errors = json["errors"].as_array().unwrap();
    assert!(
        errors
            .iter()
            .any(|err| err["dep"] == "pricing/definitely_missing"),
        "{json}"
    );
}

#[test]
fn validate_json_reports_data_seam_cycle() {
    let temp_dir = temp_repo_dir();
    let project_dir = temp_dir.path().join("data-cycle");
    let units_dir = project_dir.join("units");

    write_file(
        &project_dir,
        "Cargo.toml",
        "[package]\nname = \"data-cycle\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[workspace]\n",
    );
    write_file(
        &project_dir,
        "src/main.rs",
        "mod generated;\npub use generated::*;\nfn main() {}\n",
    );
    write_spec(
        &units_dir,
        "pricing/alpha.unit.spec",
        r#"
id: pricing/alpha
kind: data
spec_version: "0.3.0"
intent:
  why: Alpha seam.
data:
  fields:
    subtotal:
      type: i32
constructors:
  - id: new
    intent:
      why: Create alpha.
    contract:
      inputs:
        subtotal: i32
    initializes:
      subtotal: subtotal
methods:
  - id: total
    intent:
      why: Return alpha.
    receiver: shared_ref
    contract:
      returns: i32
    deps:
      - pricing/beta
    lowering:
      rust:
        body: |
          {
              self.subtotal
          }
"#,
    );
    write_spec(
        &units_dir,
        "pricing/beta.unit.spec",
        r#"
id: pricing/beta
kind: data
spec_version: "0.3.0"
intent:
  why: Beta seam.
data:
  fields:
    subtotal:
      type: i32
constructors:
  - id: new
    intent:
      why: Create beta.
    contract:
      inputs:
        subtotal: i32
    initializes:
      subtotal: subtotal
methods:
  - id: total
    intent:
      why: Return beta.
    receiver: shared_ref
    contract:
      returns: i32
    deps:
      - pricing/alpha
    lowering:
      rust:
        body: |
          {
              self.subtotal
          }
"#,
    );

    let output = run_in(&project_dir, &["validate", "units", "--format", "json"]);
    assert!(
        !output.status.success(),
        "validate should fail for data seam cycle"
    );
    let json = parse_stdout_json(&output);
    assert_eq!(json["status"], "invalid");
    let errors = json["errors"].as_array().unwrap();
    assert!(
        errors
            .iter()
            .any(|error| error["code"] == "SPEC_CYCLIC_DEP"),
        "expected SPEC_CYCLIC_DEP, got: {errors:?}"
    );
    assert!(
        errors.iter().any(|err| err["cycle"]
            == serde_json::json!(["pricing/alpha", "pricing/beta", "pricing/alpha"])),
        "{json}"
    );
    assert!(
        !errors
            .iter()
            .any(|error| error["code"] == "SPEC_SCHEMA_VALIDATION"),
        "unexpected SPEC_SCHEMA_VALIDATION, got: {errors:?}"
    );
}

#[test]
fn export_additively_includes_data_seam_truth() {
    let (_temp_dir, project_dir) = setup_m12_data_seam_project();

    let build_output = run_in(
        &project_dir,
        &[
            "build",
            "units",
            "--output",
            "src/generated",
            "--crate-root",
            ".",
        ],
    );
    assert_output_success("build should succeed before export", &build_output);

    let output = run_in(&project_dir, &["export", "units"]);
    assert_output_success("export should succeed for data seam project", &output);

    let json = parse_stdout_json(&output);
    let units = json["units"].as_array().unwrap();
    let pricing_quote = units
        .iter()
        .find(|entry| entry["id"] == "pricing/pricing_quote")
        .expect("expected pricing_quote export unit");
    assert_eq!(pricing_quote["kind"], "data");
    assert_eq!(
        pricing_quote["data"]["fields"]["subtotal"]["type"],
        "rust_decimal::Decimal"
    );
    assert_eq!(pricing_quote["constructors"][0]["id"], "new");
    assert_eq!(pricing_quote["methods"][0]["id"], "discounted_subtotal");
    assert_eq!(pricing_quote["backends"]["rust"]["derives"][0], "Clone");
}

#[test]
fn validate_does_not_accept_target_language_flag() {
    let output = run(&[
        "validate",
        "examples/ecommerce/units",
        "--target-language",
        "typescript",
    ]);
    assert!(
        !output.status.success(),
        "validate should reject --target-language"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("--target-language"), "{stderr}");
}

#[test]
fn typescript_aligned_fixture_single_file_test_succeeds_and_status_is_target_specific() {
    if !bun_available() {
        return;
    }

    let temp_dir = temp_repo_dir();
    let spec_path = write_bounded_typescript_apply_tax_fixture(temp_dir.path());

    let test_output = run_in(
        temp_dir.path(),
        &[
            "test",
            spec_path
                .strip_prefix(temp_dir.path())
                .unwrap()
                .to_str()
                .unwrap(),
            "--target-language",
            "typescript",
        ],
    );
    assert_output_success(
        "bounded TypeScript fixture should pass end-to-end",
        &test_output,
    );

    let passport_path = temp_dir
        .path()
        .join("units/pricing/apply_tax.spec.passport.json");
    let passport = read_passport_json(&passport_path);
    assert!(passport.get("evidence").is_none(), "{passport}");
    assert_eq!(
        passport["target_proofs"]["typescript"]["evidence"]["build_status"],
        "pass"
    );
    assert_eq!(
        passport["target_proofs"]["typescript"]["evidence"]["test_results"][0]["status"],
        "pass"
    );

    let rust_status_output = run_in(temp_dir.path(), &["status", "units", "--format", "json"]);
    assert!(
        !rust_status_output.status.success(),
        "rust status should remain non-green without rust proof"
    );
    let rust_status_json = parse_stdout_json(&rust_status_output);
    let rust_unit = status_units(&rust_status_json)
        .iter()
        .find(|entry| entry["id"] == "pricing/apply_tax")
        .expect("expected apply_tax rust status row");
    assert_eq!(rust_unit["status"], "untested");

    let typescript_status_output = run_in(
        temp_dir.path(),
        &[
            "status",
            "units",
            "--target-language",
            "typescript",
            "--format",
            "json",
        ],
    );
    assert_output_success(
        "TypeScript status should use the TypeScript target proof",
        &typescript_status_output,
    );
    let typescript_status_json = parse_stdout_json(&typescript_status_output);
    let typescript_unit = status_units(&typescript_status_json)
        .iter()
        .find(|entry| entry["id"] == "pricing/apply_tax")
        .expect("expected apply_tax typescript status row");
    assert_eq!(typescript_unit["status"], "valid");
}

#[test]
fn typescript_status_detects_drift_after_typescript_body_change() {
    if !bun_available() {
        return;
    }

    let temp_dir = temp_repo_dir();
    let spec_path = write_bounded_typescript_apply_tax_fixture(temp_dir.path());

    let test_output = run_in(
        temp_dir.path(),
        &[
            "test",
            spec_path
                .strip_prefix(temp_dir.path())
                .unwrap()
                .to_str()
                .unwrap(),
            "--target-language",
            "typescript",
        ],
    );
    assert_output_success(
        "bounded TypeScript fixture should pass before drift",
        &test_output,
    );

    let updated = fs::read_to_string(&spec_path).unwrap().replace(
        "return subtotal.add(subtotal.mul(rate));",
        "return subtotal.add(subtotal.mul(rate)).add(Decimal.new(1, 2));",
    );
    fs::write(&spec_path, updated).unwrap();

    let status_output = run_in(
        temp_dir.path(),
        &[
            "status",
            "units",
            "--target-language",
            "typescript",
            "--format",
            "json",
        ],
    );
    assert!(
        !status_output.status.success(),
        "drifted TypeScript proof should be non-green"
    );
    let status_json = parse_stdout_json(&status_output);
    let unit = status_units(&status_json)
        .iter()
        .find(|entry| entry["id"] == "pricing/apply_tax")
        .expect("expected apply_tax status row after drift");
    assert_eq!(unit["status"], "stale");
    assert!(
        unit["reason"]
            .as_str()
            .unwrap_or_default()
            .contains("changed since last test"),
        "{status_json}"
    );
}

#[test]
fn typescript_monotone_down_fixture_executes_with_bun() {
    if !bun_available() {
        return;
    }

    let temp_dir = temp_repo_dir();
    let spec_path = write_typescript_near_miss_fixture(temp_dir.path());

    let output = run_in(
        temp_dir.path(),
        &[
            "test",
            spec_path
                .strip_prefix(temp_dir.path())
                .unwrap()
                .to_str()
                .unwrap(),
            "--target-language",
            "typescript",
        ],
    );
    assert_output_success(
        "monotone-down roots should execute in the M59 lane",
        &output,
    );
}

#[test]
fn typescript_example_apply_tax_single_file_test_succeeds() {
    if !bun_available() {
        return;
    }

    let (_temp_dir, ecommerce_dir) = copy_ecommerce_example();
    let output = run_in(
        &ecommerce_dir,
        &[
            "test",
            "units/pricing/apply_tax.unit.spec",
            "--target-language",
            "typescript",
        ],
    );
    assert_output_success(
        "example apply_tax should succeed in the bounded TypeScript lane",
        &output,
    );

    let passport =
        read_passport_json(&ecommerce_dir.join("units/pricing/apply_tax.spec.passport.json"));
    assert_eq!(
        passport["target_proofs"]["typescript"]["evidence"]["build_status"],
        "pass"
    );
    assert_eq!(
        passport["target_proofs"]["typescript"]["evidence"]["test_results"][0]["status"],
        "pass"
    );
}

#[test]
fn typescript_local_supported_graph_helper_root_executes_with_bun() {
    if !bun_available() {
        return;
    }

    let (_temp_dir, fixture_dir) = copy_typescript_local_supported_graph_fixture();
    let output = run_in(
        &fixture_dir,
        &[
            "test",
            "units/money/round.unit.spec",
            "--target-language",
            "typescript",
        ],
    );
    assert_output_success("local helper root should pass in the M59 lane", &output);

    let passport = read_passport_json(&fixture_dir.join("units/money/round.spec.passport.json"));
    assert_eq!(
        passport["target_proofs"]["typescript"]["evidence"]["build_status"],
        "pass"
    );
    assert_eq!(
        passport["target_proofs"]["typescript"]["evidence"]["test_results"][0]["status"],
        "pass"
    );
}

#[test]
fn typescript_local_supported_graph_monotone_down_root_executes_with_bun() {
    if !bun_available() {
        return;
    }

    let (_temp_dir, fixture_dir) = copy_typescript_local_supported_graph_fixture();
    let output = run_in(
        &fixture_dir,
        &[
            "test",
            "units/pricing/apply_discount.unit.spec",
            "--target-language",
            "typescript",
        ],
    );
    assert_output_success(
        "local monotone-down root should pass in the M59 lane",
        &output,
    );

    let passport =
        read_passport_json(&fixture_dir.join("units/pricing/apply_discount.spec.passport.json"));
    assert_eq!(
        passport["target_proofs"]["typescript"]["evidence"]["build_status"],
        "pass"
    );
    assert_eq!(
        passport["target_proofs"]["typescript"]["evidence"]["test_results"][0]["status"],
        "pass"
    );
}

#[test]
fn typescript_local_supported_graph_monotone_up_root_executes_with_bun() {
    if !bun_available() {
        return;
    }

    let (_temp_dir, fixture_dir) = copy_typescript_local_supported_graph_fixture();
    let output = run_in(
        &fixture_dir,
        &[
            "test",
            "units/pricing/apply_tax.unit.spec",
            "--target-language",
            "typescript",
        ],
    );
    assert_output_success(
        "local monotone-up root should pass in the M59 lane",
        &output,
    );

    let passport =
        read_passport_json(&fixture_dir.join("units/pricing/apply_tax.spec.passport.json"));
    assert_eq!(
        passport["target_proofs"]["typescript"]["evidence"]["build_status"],
        "pass"
    );
    assert_eq!(
        passport["target_proofs"]["typescript"]["evidence"]["test_results"][0]["status"],
        "pass"
    );
}

#[test]
fn typescript_local_supported_graph_wrapper_root_executes_with_bun() {
    if !bun_available() {
        return;
    }

    let (_temp_dir, fixture_dir) = copy_typescript_local_supported_graph_fixture();
    let output = run_in(
        &fixture_dir,
        &[
            "test",
            "units/pricing/calculate_total.unit.spec",
            "--target-language",
            "typescript",
        ],
    );
    assert_output_success("local wrapper root should pass in the M59 lane", &output);

    let passport =
        read_passport_json(&fixture_dir.join("units/pricing/calculate_total.spec.passport.json"));
    assert_eq!(
        passport["target_proofs"]["typescript"]["evidence"]["build_status"],
        "pass"
    );
    assert_eq!(
        passport["target_proofs"]["typescript"]["evidence"]["test_results"][0]["status"],
        "pass"
    );
}

#[test]
fn typescript_local_supported_graph_chain3_root_executes_with_bun() {
    if !bun_available() {
        return;
    }

    let (_temp_dir, fixture_dir) = copy_typescript_local_supported_graph_fixture();
    let output = run_in(
        &fixture_dir,
        &[
            "test",
            "units/pricing/checkout_total.unit.spec",
            "--target-language",
            "typescript",
        ],
    );
    assert_output_success("local chain3 root should pass in the M59 lane", &output);

    let passport =
        read_passport_json(&fixture_dir.join("units/pricing/checkout_total.spec.passport.json"));
    assert_eq!(
        passport["target_proofs"]["typescript"]["evidence"]["build_status"],
        "pass"
    );
    assert_eq!(
        passport["target_proofs"]["typescript"]["evidence"]["test_results"][0]["status"],
        "pass"
    );
    assert!(
        !fixture_dir
            .join("src/generated/pricing/display_total.rs")
            .exists(),
        "unrelated supported units should stay out of the generated local graph"
    );
}

#[cfg(unix)]
#[test]
fn typescript_local_supported_graph_reachable_shared_dep_rejects_before_bun_runs() {
    let temp_dir = tempfile::TempDir::new_in(repo_root().join("target")).unwrap();
    let fixture_dir = temp_dir.path().join("typescript_local_supported_graph");
    fs::write(
        temp_dir.path().join(".git"),
        "gitdir: .git/modules/spec-tests\n",
    )
    .expect("failed to mark local supported-graph temp repo root");
    copy_dir_recursive(
        &repo_root().join("spec-cli/tests/fixtures/typescript_local_supported_graph"),
        &fixture_dir,
    )
    .expect("failed to copy local supported-graph fixture");
    copy_dir_recursive(
        &repo_root().join("examples/shared-crate"),
        &temp_dir.path().join("shared-crate"),
    )
    .expect("failed to copy shared crate for local-graph rejection coverage");
    copy_dir_recursive(
        &repo_root().join("examples/shared-spec"),
        &temp_dir.path().join("shared-spec"),
    )
    .expect("failed to copy shared spec for local-graph rejection coverage");
    fs::write(
        fixture_dir.join("spec.toml"),
        "[libraries]\nshared = \"../shared-spec\"\n",
    )
    .unwrap();
    fs::write(
        fixture_dir.join("Cargo.toml"),
        "[package]\nname = \"typescript-local-supported-graph\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\nrust_decimal = { version = \"1.36\", features = [\"serde\"] }\nshared = { path = \"../shared-crate\" }\n\n[workspace]\n",
    )
    .unwrap();
    replace_in_file(
        &fixture_dir.join("units/pricing/apply_tax.unit.spec"),
        "deps:\n  - money/round\n",
        "deps:\n  - shared::money/round\n",
    );

    let marker_path = fixture_dir.join("bun-invoked.txt");
    let fake_bun = format!(
        "#!/bin/sh\nif [ \"$1\" = \"--version\" ]; then\n  echo '1.2.15'\n  exit 0\nfi\necho invoked > \"{}\"\nexit 99\n",
        marker_path.display()
    );
    let path_override = path_with_fake_bun(&fixture_dir, &fake_bun);

    let output = run_in_with_env(
        &fixture_dir,
        &[
            "test",
            "units/pricing/checkout_total.unit.spec",
            "--target-language",
            "typescript",
        ],
        &[("PATH", path_override.as_os_str())],
    );
    assert!(
        !output.status.success(),
        "reachable shared deps in the local graph should be rejected"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains(
            "TypeScript same-tree local target requires every reachable dep to stay same-tree local in M59"
        ),
        "{stderr}"
    );
    assert!(
        !marker_path.exists(),
        "same-tree local shared-dep rejection should happen before Bun"
    );
}

#[cfg(unix)]
#[test]
fn typescript_local_supported_graph_reachable_unsupported_member_rejects_before_bun_runs() {
    let (_temp_dir, fixture_dir) = copy_typescript_local_supported_graph_fixture();
    replace_in_file(
        &fixture_dir.join("units/pricing/apply_discount.unit.spec"),
        "        let discounted = subtotal - subtotal * rate;\n        round(discounted.max(Decimal::ZERO))\n",
        "        let discounted = subtotal - subtotal * rate;\n        if discounted < Decimal::ZERO {\n            Decimal::ZERO\n        } else {\n            round(discounted)\n        }\n",
    );

    let marker_path = fixture_dir.join("bun-invoked.txt");
    let fake_bun = format!(
        "#!/bin/sh\nif [ \"$1\" = \"--version\" ]; then\n  echo '1.2.15'\n  exit 0\nfi\necho invoked > \"{}\"\nexit 99\n",
        marker_path.display()
    );
    let path_override = path_with_fake_bun(&fixture_dir, &fake_bun);

    let output = run_in_with_env(
        &fixture_dir,
        &[
            "test",
            "units/pricing/checkout_total.unit.spec",
            "--target-language",
            "typescript",
        ],
        &[("PATH", path_override.as_os_str())],
    );
    assert!(
        !output.status.success(),
        "reachable unsupported local members should be rejected"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains(
            "TypeScript same-tree local target requires every reachable unit to classify to a supported semantic review in M59"
        ),
        "{stderr}"
    );
    assert!(stderr.contains("unsupported.function.v1"), "{stderr}");
    assert!(
        !marker_path.exists(),
        "unsupported-member rejection should happen before Bun"
    );
}

#[cfg(unix)]
#[test]
fn typescript_local_supported_graph_missing_typescript_body_rejects_before_bun_runs() {
    let (_temp_dir, fixture_dir) = copy_typescript_local_supported_graph_fixture();
    remove_typescript_body(&fixture_dir.join("units/pricing/apply_tax.unit.spec"));

    let marker_path = fixture_dir.join("bun-invoked.txt");
    let fake_bun = format!(
        "#!/bin/sh\nif [ \"$1\" = \"--version\" ]; then\n  echo '1.2.15'\n  exit 0\nfi\necho invoked > \"{}\"\nexit 99\n",
        marker_path.display()
    );
    let path_override = path_with_fake_bun(&fixture_dir, &fake_bun);

    let output = run_in_with_env(
        &fixture_dir,
        &[
            "test",
            "units/pricing/checkout_total.unit.spec",
            "--target-language",
            "typescript",
        ],
        &[("PATH", path_override.as_os_str())],
    );
    assert!(
        !output.status.success(),
        "reachable missing body.typescript should be rejected"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains(
            "TypeScript same-tree local target requires every reachable unit to author body.typescript in M59"
        ),
        "{stderr}"
    );
    assert!(
        !marker_path.exists(),
        "missing body.typescript rejection should happen before Bun"
    );
}

#[cfg(unix)]
#[test]
fn typescript_local_supported_graph_unsupported_topology_rejects_before_bun_runs() {
    let (_temp_dir, fixture_dir) = copy_typescript_local_supported_graph_fixture();
    replace_in_file(
        &fixture_dir.join("units/pricing/checkout_total.unit.spec"),
        "deps:\n  - pricing/calculate_total\n  - pricing/apply_tax\n  - pricing/apply_discount\n",
        "deps:\n  - pricing/calculate_total\n  - pricing/apply_tax\n  - pricing/apply_discount\n  - pricing/display_total\n",
    );

    let marker_path = fixture_dir.join("bun-invoked.txt");
    let fake_bun = format!(
        "#!/bin/sh\nif [ \"$1\" = \"--version\" ]; then\n  echo '1.2.15'\n  exit 0\nfi\necho invoked > \"{}\"\nexit 99\n",
        marker_path.display()
    );
    let path_override = path_with_fake_bun(&fixture_dir, &fake_bun);

    let output = run_in_with_env(
        &fixture_dir,
        &[
            "test",
            "units/pricing/checkout_total.unit.spec",
            "--target-language",
            "typescript",
        ],
        &[("PATH", path_override.as_os_str())],
    );
    assert!(
        !output.status.success(),
        "unsupported local dep topology should still be rejected"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("unsupported.function.v1"), "{stderr}");
    assert!(
        !marker_path.exists(),
        "unsupported-topology rejection should happen before Bun"
    );
}

#[cfg(unix)]
#[test]
fn typescript_local_supported_graph_cycle_rejects_before_bun_runs() {
    let (_temp_dir, fixture_dir) = copy_typescript_local_supported_graph_fixture();
    replace_in_file(
        &fixture_dir.join("units/pricing/apply_tax.unit.spec"),
        "deps:\n  - money/round\n",
        "deps:\n  - money/round\n  - pricing/checkout_total\n",
    );

    let marker_path = fixture_dir.join("bun-invoked.txt");
    let fake_bun = format!(
        "#!/bin/sh\nif [ \"$1\" = \"--version\" ]; then\n  echo '1.2.15'\n  exit 0\nfi\necho invoked > \"{}\"\nexit 99\n",
        marker_path.display()
    );
    let path_override = path_with_fake_bun(&fixture_dir, &fake_bun);

    let output = run_in_with_env(
        &fixture_dir,
        &[
            "test",
            "units/pricing/checkout_total.unit.spec",
            "--target-language",
            "typescript",
        ],
        &[("PATH", path_override.as_os_str())],
    );
    assert!(
        !output.status.success(),
        "local cycles should be rejected before Bun"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("cycle detected"), "{stderr}");
    assert!(
        !marker_path.exists(),
        "cycle rejection should happen before Bun"
    );
}

#[test]
fn typescript_cross_library_example_status_marks_apply_tax_valid_after_proof() {
    if !bun_available() {
        return;
    }

    let (_temp_dir, app_dir, _shared_crate_dir, _shared_spec_dir) =
        copy_crosslib_typescript_example();

    let test_output = run_in(
        &app_dir,
        &[
            "test",
            "units/pricing/apply_tax.unit.spec",
            "--target-language",
            "typescript",
        ],
    );
    assert_output_success(
        "cross-library apply_tax should pass in the bounded TypeScript lane",
        &test_output,
    );

    let status_output = run_in(
        &app_dir,
        &[
            "status",
            "units",
            "--target-language",
            "typescript",
            "--format",
            "json",
        ],
    );
    assert!(
        !status_output.status.success(),
        "status should stay non-green while other cross-library units remain untested"
    );
    let status_json = parse_stdout_json(&status_output);
    let apply_tax = status_units(&status_json)
        .iter()
        .find(|unit| unit["id"] == "pricing/apply_tax")
        .expect("expected pricing/apply_tax status row");
    assert_eq!(apply_tax["status"], "valid", "{status_json}");
    assert_eq!(
        apply_tax["semantic_review"]["compatibility_key"], FUNCTION_FAMILY_A_UP_COMPATIBILITY_KEY,
        "{status_json}"
    );
    assert_eq!(
        apply_tax["freshness"]["authored_truth_status"], "fresh",
        "{status_json}"
    );
}

#[test]
fn typescript_cross_library_wrapper_example_executes_with_bun() {
    if !bun_available() {
        return;
    }

    let (_temp_dir, app_dir, _shared_crate_dir, _shared_spec_dir) =
        copy_crosslib_typescript_example();
    let output = run_in(
        &app_dir,
        &[
            "test",
            "units/pricing/calculate_total.unit.spec",
            "--target-language",
            "typescript",
        ],
    );
    assert_output_success(
        "cross-library wrapper example should pass in the bounded TypeScript lane",
        &output,
    );

    let passport =
        read_passport_json(&app_dir.join("units/pricing/calculate_total.spec.passport.json"));
    assert_eq!(
        passport["target_proofs"]["typescript"]["evidence"]["build_status"],
        "pass"
    );
    assert_eq!(
        passport["target_proofs"]["typescript"]["evidence"]["test_results"][0]["status"],
        "pass"
    );
}

#[cfg(unix)]
#[test]
fn typescript_cross_library_chain3_root_executes_with_bun() {
    if !bun_available() {
        return;
    }

    let (_temp_dir, app_dir, _shared_crate_dir, _shared_spec_dir) =
        copy_crosslib_typescript_example();
    write_cross_library_chain3_spec(&app_dir);
    let output = run_in(
        &app_dir,
        &[
            "test",
            "units/pricing/checkout_chain3.unit.spec",
            "--target-language",
            "typescript",
        ],
    );
    assert_output_success(
        "cross-library chain3 root should pass in the bounded TypeScript lane",
        &output,
    );

    let passport =
        read_passport_json(&app_dir.join("units/pricing/checkout_chain3.spec.passport.json"));
    assert_eq!(
        passport["target_proofs"]["typescript"]["evidence"]["build_status"],
        "pass"
    );
    assert_eq!(
        passport["target_proofs"]["typescript"]["evidence"]["test_results"][0]["status"],
        "pass"
    );
}

#[cfg(unix)]
#[test]
fn typescript_cross_library_wrapper_wrong_dep_order_rejects_before_bun_runs() {
    let (temp_dir, app_dir, _shared_crate_dir, _shared_spec_dir) =
        copy_crosslib_typescript_example();
    replace_in_file(
        &app_dir.join("units/pricing/calculate_total.unit.spec"),
        "deps:\n  - shared::pricing/apply_discount\n  - shared::pricing/apply_tax\n",
        "deps:\n  - shared::pricing/apply_tax\n  - shared::pricing/apply_discount\n",
    );

    let marker_path = app_dir.join("bun-invoked.txt");
    let fake_bun = format!(
        "#!/bin/sh\nif [ \"$1\" = \"--version\" ]; then\n  echo '1.2.15'\n  exit 0\nfi\necho invoked > \"{}\"\nexit 99\n",
        marker_path.display()
    );
    let path_override = path_with_fake_bun(temp_dir.path(), &fake_bun);

    let output = run_in_with_env(
        &app_dir,
        &[
            "test",
            "units/pricing/calculate_total.unit.spec",
            "--target-language",
            "typescript",
        ],
        &[("PATH", path_override.as_os_str())],
    );
    assert!(
        !output.status.success(),
        "cross-library wrapper with wrong dep order should be rejected"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains(
            "TypeScript wrapper target requires direct deps to classify as function.arithmetic_leaf.monotone_down_nonnegative.v1 then function.arithmetic_leaf.monotone_up.v1 in M56"
        ),
        "{stderr}"
    );
    assert!(
        !marker_path.exists(),
        "cross-library wrapper dep-order rejection should happen before Bun"
    );
}

#[cfg(unix)]
#[test]
fn typescript_cross_library_chain3_wrong_dep_order_rejects_before_bun_runs() {
    let (temp_dir, app_dir, _shared_crate_dir, _shared_spec_dir) =
        copy_crosslib_typescript_example();
    write_cross_library_chain3_spec(&app_dir);
    replace_in_file(
        &app_dir.join("units/pricing/checkout_chain3.unit.spec"),
        "deps:\n  - pricing/calculate_total\n  - shared::pricing/apply_tax\n  - shared::pricing/apply_discount\n",
        "deps:\n  - shared::pricing/apply_tax\n  - pricing/calculate_total\n  - shared::pricing/apply_discount\n",
    );

    let marker_path = app_dir.join("bun-invoked.txt");
    let fake_bun = format!(
        "#!/bin/sh\nif [ \"$1\" = \"--version\" ]; then\n  echo '1.2.15'\n  exit 0\nfi\necho invoked > \"{}\"\nexit 99\n",
        marker_path.display()
    );
    let path_override = path_with_fake_bun(temp_dir.path(), &fake_bun);

    let output = run_in_with_env(
        &app_dir,
        &[
            "test",
            "units/pricing/checkout_chain3.unit.spec",
            "--target-language",
            "typescript",
        ],
        &[("PATH", path_override.as_os_str())],
    );
    assert!(
        !output.status.success(),
        "cross-library chain3 with wrong dep order should be rejected"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains(
            "TypeScript chain3 target requires direct deps to classify as function.wrapper.pipeline.v1, function.wrapper.pipeline.normalized_required_arg.v1, or function.wrapper.pipeline.chain3.v1 then function.arithmetic_leaf.monotone_up.v1 then function.arithmetic_leaf.monotone_down_nonnegative.v1 in M61"
        ),
        "{stderr}"
    );
    assert!(
        !marker_path.exists(),
        "cross-library chain3 dep-order rejection should happen before Bun"
    );
}

#[cfg(unix)]
#[test]
fn typescript_cross_library_wrapper_wrong_family_rejects_before_bun_runs() {
    let (temp_dir, app_dir, _shared_crate_dir, _shared_spec_dir) =
        copy_crosslib_typescript_example();
    replace_in_file(
        &app_dir.join("units/pricing/calculate_total.unit.spec"),
        "deps:\n  - shared::pricing/apply_discount\n  - shared::pricing/apply_tax\n",
        "deps:\n  - shared::money/round\n  - shared::pricing/apply_tax\n",
    );

    let marker_path = app_dir.join("bun-invoked.txt");
    let fake_bun = format!(
        "#!/bin/sh\nif [ \"$1\" = \"--version\" ]; then\n  echo '1.2.15'\n  exit 0\nfi\necho invoked > \"{}\"\nexit 99\n",
        marker_path.display()
    );
    let path_override = path_with_fake_bun(temp_dir.path(), &fake_bun);

    let output = run_in_with_env(
        &app_dir,
        &[
            "test",
            "units/pricing/calculate_total.unit.spec",
            "--target-language",
            "typescript",
        ],
        &[("PATH", path_override.as_os_str())],
    );
    assert!(
        !output.status.success(),
        "cross-library wrapper with wrong dep family should be rejected"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("unsupported.function.v1")
            || stderr.contains(
                "TypeScript wrapper target requires direct deps to classify as function.arithmetic_leaf.monotone_down_nonnegative.v1 then function.arithmetic_leaf.monotone_up.v1 in M56"
            ),
        "{stderr}"
    );
    assert!(
        !marker_path.exists(),
        "cross-library wrapper family rejection should happen before Bun"
    );
}

#[cfg(unix)]
#[test]
fn typescript_cross_library_chain3_wrong_family_rejects_before_bun_runs() {
    let (temp_dir, app_dir, _shared_crate_dir, _shared_spec_dir) =
        copy_crosslib_typescript_example();
    write_cross_library_chain3_spec(&app_dir);
    replace_in_file(
        &app_dir.join("units/pricing/checkout_chain3.unit.spec"),
        "deps:\n  - pricing/calculate_total\n  - shared::pricing/apply_tax\n  - shared::pricing/apply_discount\n",
        "deps:\n  - pricing/calculate_total\n  - shared::money/round\n  - shared::pricing/apply_discount\n",
    );

    let marker_path = app_dir.join("bun-invoked.txt");
    let fake_bun = format!(
        "#!/bin/sh\nif [ \"$1\" = \"--version\" ]; then\n  echo '1.2.15'\n  exit 0\nfi\necho invoked > \"{}\"\nexit 99\n",
        marker_path.display()
    );
    let path_override = path_with_fake_bun(temp_dir.path(), &fake_bun);

    let output = run_in_with_env(
        &app_dir,
        &[
            "test",
            "units/pricing/checkout_chain3.unit.spec",
            "--target-language",
            "typescript",
        ],
        &[("PATH", path_override.as_os_str())],
    );
    assert!(
        !output.status.success(),
        "cross-library chain3 with wrong dep family should be rejected"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains(
            "TypeScript chain3 target requires direct deps to classify as function.wrapper.pipeline.v1, function.wrapper.pipeline.normalized_required_arg.v1, or function.wrapper.pipeline.chain3.v1 then function.arithmetic_leaf.monotone_up.v1 then function.arithmetic_leaf.monotone_down_nonnegative.v1 in M61"
        ),
        "{stderr}"
    );
    assert!(
        !marker_path.exists(),
        "cross-library chain3 family rejection should happen before Bun"
    );
}

#[cfg(unix)]
#[test]
fn typescript_cross_library_wrapper_missing_typescript_body_rejects_before_bun_runs() {
    let (temp_dir, app_dir, _shared_crate_dir, shared_spec_dir) =
        copy_crosslib_typescript_example();
    remove_typescript_body(&shared_spec_dir.join("units/pricing/apply_tax.unit.spec"));

    let marker_path = app_dir.join("bun-invoked.txt");
    let fake_bun = format!(
        "#!/bin/sh\nif [ \"$1\" = \"--version\" ]; then\n  echo '1.2.15'\n  exit 0\nfi\necho invoked > \"{}\"\nexit 99\n",
        marker_path.display()
    );
    let path_override = path_with_fake_bun(temp_dir.path(), &fake_bun);

    let output = run_in_with_env(
        &app_dir,
        &[
            "test",
            "units/pricing/calculate_total.unit.spec",
            "--target-language",
            "typescript",
        ],
        &[("PATH", path_override.as_os_str())],
    );
    assert!(
        !output.status.success(),
        "cross-library wrapper with missing imported body.typescript should be rejected"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains(
            "TypeScript wrapper target requires direct deps to author body.typescript in M56"
        ),
        "{stderr}"
    );
    assert!(
        !marker_path.exists(),
        "cross-library wrapper missing-body rejection should happen before Bun"
    );
}

#[cfg(unix)]
#[test]
fn typescript_cross_library_chain3_missing_typescript_body_rejects_before_bun_runs() {
    let (temp_dir, app_dir, _shared_crate_dir, shared_spec_dir) =
        copy_crosslib_typescript_example();
    write_cross_library_chain3_spec(&app_dir);
    remove_typescript_body(&shared_spec_dir.join("units/pricing/apply_tax.unit.spec"));

    let marker_path = app_dir.join("bun-invoked.txt");
    let fake_bun = format!(
        "#!/bin/sh\nif [ \"$1\" = \"--version\" ]; then\n  echo '1.2.15'\n  exit 0\nfi\necho invoked > \"{}\"\nexit 99\n",
        marker_path.display()
    );
    let path_override = path_with_fake_bun(temp_dir.path(), &fake_bun);

    let output = run_in_with_env(
        &app_dir,
        &[
            "test",
            "units/pricing/checkout_chain3.unit.spec",
            "--target-language",
            "typescript",
        ],
        &[("PATH", path_override.as_os_str())],
    );
    assert!(
        !output.status.success(),
        "cross-library chain3 with missing imported body.typescript should be rejected"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains(
            "TypeScript chain3 target requires direct deps to author body.typescript in M56"
        ),
        "{stderr}"
    );
    assert!(
        !marker_path.exists(),
        "cross-library chain3 missing-body rejection should happen before Bun"
    );
}

#[cfg(unix)]
#[test]
fn typescript_cross_library_wrapper_unresolved_alias_rejects_before_bun_runs() {
    let (temp_dir, app_dir, _shared_crate_dir, _shared_spec_dir) =
        copy_crosslib_typescript_example();
    replace_in_file(
        &app_dir.join("units/pricing/calculate_total.unit.spec"),
        "shared::pricing/apply_discount",
        "missing::pricing/apply_discount",
    );

    let marker_path = app_dir.join("bun-invoked.txt");
    let fake_bun = format!(
        "#!/bin/sh\nif [ \"$1\" = \"--version\" ]; then\n  echo '1.2.15'\n  exit 0\nfi\necho invoked > \"{}\"\nexit 99\n",
        marker_path.display()
    );
    let path_override = path_with_fake_bun(temp_dir.path(), &fake_bun);

    let output = run_in_with_env(
        &app_dir,
        &[
            "test",
            "units/pricing/calculate_total.unit.spec",
            "--target-language",
            "typescript",
        ],
        &[("PATH", path_override.as_os_str())],
    );
    assert!(
        !output.status.success(),
        "cross-library wrapper with unresolved alias should be rejected"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("unknown library namespace 'missing'"),
        "{stderr}"
    );
    assert!(
        !marker_path.exists(),
        "cross-library wrapper unresolved-alias rejection should happen before Bun"
    );
}

#[cfg(unix)]
#[test]
fn typescript_cross_library_wrapper_missing_imported_unit_rejects_before_bun_runs() {
    let (temp_dir, app_dir, _shared_crate_dir, _shared_spec_dir) =
        copy_crosslib_typescript_example();
    replace_in_file(
        &app_dir.join("units/pricing/calculate_total.unit.spec"),
        "shared::pricing/apply_tax",
        "shared::pricing/missing_tax",
    );

    let marker_path = app_dir.join("bun-invoked.txt");
    let fake_bun = format!(
        "#!/bin/sh\nif [ \"$1\" = \"--version\" ]; then\n  echo '1.2.15'\n  exit 0\nfi\necho invoked > \"{}\"\nexit 99\n",
        marker_path.display()
    );
    let path_override = path_with_fake_bun(temp_dir.path(), &fake_bun);

    let output = run_in_with_env(
        &app_dir,
        &[
            "test",
            "units/pricing/calculate_total.unit.spec",
            "--target-language",
            "typescript",
        ],
        &[("PATH", path_override.as_os_str())],
    );
    assert!(
        !output.status.success(),
        "cross-library wrapper with missing imported unit should be rejected"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("cross-library dep 'shared::pricing/missing_tax' not found"),
        "{stderr}"
    );
    assert!(
        !marker_path.exists(),
        "cross-library wrapper missing-imported-unit rejection should happen before Bun"
    );
}

#[test]
fn typescript_example_calculate_total_single_file_test_succeeds() {
    if !bun_available() {
        return;
    }

    let (_temp_dir, ecommerce_dir) = copy_ecommerce_example();
    let output = run_in(
        &ecommerce_dir,
        &[
            "test",
            "units/pricing/calculate_total.unit.spec",
            "--target-language",
            "typescript",
        ],
    );
    assert_output_success(
        "example calculate_total should succeed in the bounded TypeScript wrapper lane",
        &output,
    );

    let wrapper_passport =
        read_passport_json(&ecommerce_dir.join("units/pricing/calculate_total.spec.passport.json"));
    assert_eq!(
        wrapper_passport["target_proofs"]["typescript"]["evidence"]["build_status"],
        "pass"
    );
    assert_eq!(
        wrapper_passport["target_proofs"]["typescript"]["evidence"]["test_results"][0]["status"],
        "pass"
    );
}

#[test]
fn typescript_aligned_wrapper_fixture_single_file_test_succeeds() {
    if !bun_available() {
        return;
    }

    let (_temp_dir, fixture_dir) = copy_m30_wrapper_fixture("aligned");
    let output = run_in(
        &fixture_dir,
        &[
            "test",
            "units/pricing/pricing_total_wrapper_aligned.unit.spec",
            "--target-language",
            "typescript",
        ],
    );
    assert_output_success(
        "aligned wrapper fixture should pass in the bounded TypeScript wrapper lane",
        &output,
    );

    let wrapper_passport = read_passport_json(
        &fixture_dir.join("units/pricing/pricing_total_wrapper_aligned.spec.passport.json"),
    );
    assert_eq!(
        wrapper_passport["target_proofs"]["typescript"]["evidence"]["build_status"],
        "pass"
    );
    assert_eq!(
        wrapper_passport["target_proofs"]["typescript"]["evidence"]["test_results"][0]["status"],
        "pass"
    );
}

#[cfg(unix)]
#[test]
fn typescript_molecule_test_is_rejected_before_bun_runs() {
    let (_temp_dir, ecommerce_dir) = copy_ecommerce_example();
    let marker_path = ecommerce_dir.join("bun-invoked.txt");
    let fake_bun = format!(
        "#!/bin/sh\nif [ \"$1\" = \"--version\" ]; then\n  echo '1.2.15'\n  exit 0\nfi\necho invoked > \"{}\"\nexit 99\n",
        marker_path.display()
    );
    let path_override = path_with_fake_bun(&ecommerce_dir, &fake_bun);

    let output = run_in_with_env(
        &ecommerce_dir,
        &[
            "test",
            "units/pricing/discount_plus_tax.test.spec",
            "--target-language",
            "typescript",
        ],
        &[("PATH", path_override.as_os_str())],
    );
    assert!(
        !output.status.success(),
        "TypeScript molecule tests should be rejected in M52"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains(
            ".test.spec is not supported for --target-language typescript in M52; molecule tests remain Rust-only"
        ),
        "{stderr}"
    );
    assert!(
        !marker_path.exists(),
        "molecule rejection should happen before Bun build/test execution"
    );
}

#[cfg(unix)]
#[test]
fn typescript_chain3_wrapper_executes_with_bun() {
    if !bun_available() {
        return;
    }

    let (_temp_dir, fixture_dir) = copy_m21_chain3_fixture("aligned");
    let output = run_in(
        &fixture_dir,
        &[
            "test",
            "units/pricing/checkout_chain3_aligned.unit.spec",
            "--target-language",
            "typescript",
        ],
    );
    assert_output_success(
        "aligned chain3 fixture should pass in the bounded same-tree TypeScript lane",
        &output,
    );

    let passport = read_passport_json(
        &fixture_dir.join("units/pricing/checkout_chain3_aligned.spec.passport.json"),
    );
    assert_eq!(
        passport["target_proofs"]["typescript"]["evidence"]["build_status"],
        "pass"
    );
    assert_eq!(
        passport["target_proofs"]["typescript"]["evidence"]["test_results"][0]["status"],
        "pass"
    );
}

#[cfg(unix)]
#[test]
fn typescript_nested_chain3_executes_with_bun() {
    if !bun_available() {
        return;
    }

    let (_temp_dir, fixture_dir) = copy_m21_chain3_fixture("aligned");
    let output = run_in(
        &fixture_dir,
        &[
            "test",
            "units/pricing/checkout_nested_chain3_aligned.unit.spec",
            "--target-language",
            "typescript",
        ],
    );
    assert_output_success(
        "aligned nested chain3 fixture should pass in the bounded same-tree TypeScript lane",
        &output,
    );

    let passport = read_passport_json(
        &fixture_dir.join("units/pricing/checkout_nested_chain3_aligned.spec.passport.json"),
    );
    assert_eq!(
        passport["target_proofs"]["typescript"]["evidence"]["build_status"],
        "pass"
    );
    assert_eq!(
        passport["target_proofs"]["typescript"]["evidence"]["test_results"][0]["status"],
        "pass"
    );
}

#[cfg(unix)]
#[test]
fn typescript_chain3_wrong_family_rejects_before_bun_runs() {
    let (_temp_dir, fixture_dir) = copy_m21_chain3_fixture("unsupported_near_miss");
    inject_typescript_body_if_missing(
        &fixture_dir.join("units/pricing/checkout_chain3_unsupported_near_miss.unit.spec"),
        "    {\n        const base_total = pricing_total_wrapper_unsupported_near_miss(subtotal, discount_rate, tax_rate);\n        const surcharged_total = pricing_tax_leaf_unsupported_near_miss(base_total, surcharge_rate);\n        return pricing_discount_leaf_unsupported_near_miss(surcharged_total, loyalty_rate);\n    }",
    );
    inject_typescript_body_if_missing(
        &fixture_dir.join("units/pricing/pricing_total_wrapper_unsupported_near_miss.unit.spec"),
        "    {\n        const discounted = pricing_discount_leaf_unsupported_near_miss(subtotal, discount_rate);\n        return pricing_tax_leaf_unsupported_near_miss(discounted, tax_rate);\n    }",
    );
    inject_typescript_body_if_missing(
        &fixture_dir.join("units/pricing/pricing_tax_leaf_unsupported_near_miss.unit.spec"),
        "    {\n        return subtotal.add(subtotal.mul(rate));\n    }",
    );
    inject_typescript_body_if_missing(
        &fixture_dir.join("units/pricing/pricing_discount_leaf_unsupported_near_miss.unit.spec"),
        "    {\n        const discounted = subtotal.add(subtotal.mul(Decimal.new(-1n, 0n).mul(rate)));\n        return discounted;\n    }",
    );

    let marker_path = fixture_dir.join("bun-invoked.txt");
    let fake_bun = format!(
        "#!/bin/sh\nif [ \"$1\" = \"--version\" ]; then\n  echo '1.2.15'\n  exit 0\nfi\necho invoked > \"{}\"\nexit 99\n",
        marker_path.display()
    );
    let path_override = path_with_fake_bun(&fixture_dir, &fake_bun);

    let output = run_in_with_env(
        &fixture_dir,
        &[
            "test",
            "units/pricing/checkout_chain3_unsupported_near_miss.unit.spec",
            "--target-language",
            "typescript",
        ],
        &[("PATH", path_override.as_os_str())],
    );
    assert!(
        !output.status.success(),
        "unsupported chain3-like TypeScript target should be rejected before Bun"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("unsupported.function.v1"), "{stderr}");
    assert!(
        !marker_path.exists(),
        "chain3 wrong-family rejection should happen before Bun build/test execution"
    );
}

#[cfg(unix)]
#[test]
fn typescript_nested_chain3_wrong_first_slot_family_reaches_bun_in_m59() {
    let (_temp_dir, fixture_dir) = copy_m21_chain3_fixture("aligned");
    write_spec(
        &fixture_dir.join("units"),
        "pricing/wrong_first_slot_tax_like_aligned.unit.spec",
        r#"
id: pricing/wrong_first_slot_tax_like_aligned
kind: function
spec_version: "0.3.0"
intent:
  why: Provide a distinct monotone-up leaf so nested chain3 slot-1 family rejection is exercised without duplicate dep collisions.
contract:
  inputs:
    subtotal: Decimal
    rate: Decimal
  returns: Decimal
  invariants:
    - output >= subtotal
imports:
  - rust_decimal::Decimal
body:
  rust: |
    {
        subtotal + subtotal * rate
    }
  typescript: |
    {
        return subtotal.add(subtotal.mul(rate));
    }
local_tests:
  - id: wrong_first_slot_tax_like_aligned_basic
    expect: wrong_first_slot_tax_like_aligned(Decimal::new(10000, 2), Decimal::new(10, 2)) == Decimal::new(11000, 2)
"#,
    );
    replace_in_file(
        &fixture_dir.join("units/pricing/checkout_nested_chain3_aligned.unit.spec"),
        "deps:\n  - pricing/base_nested_chain3_aligned\n  - pricing/pricing_tax_leaf_aligned\n  - pricing/pricing_discount_leaf_aligned\n",
        "deps:\n  - pricing/wrong_first_slot_tax_like_aligned\n  - pricing/pricing_tax_leaf_aligned\n  - pricing/pricing_discount_leaf_aligned\n",
    );

    let marker_path = fixture_dir.join("bun-invoked.txt");
    let fake_bun = format!(
        "#!/bin/sh\nif [ \"$1\" = \"--version\" ]; then\n  echo '1.2.15'\n  exit 0\nfi\necho invoked > \"{}\"\nexit 99\n",
        marker_path.display()
    );
    let path_override = path_with_fake_bun(&fixture_dir, &fake_bun);

    let output = run_in_with_env(
        &fixture_dir,
        &[
            "test",
            "units/pricing/checkout_nested_chain3_aligned.unit.spec",
            "--target-language",
            "typescript",
        ],
        &[("PATH", path_override.as_os_str())],
    );
    assert!(
        !output.status.success(),
        "nested chain3 targets with rewritten local deps should still fail under the fake Bun"
    );
    assert!(
        marker_path.exists(),
        "M59 same-tree local graph validation should allow this case far enough to reach Bun"
    );
}

#[cfg(unix)]
#[test]
fn typescript_chain3_missing_typescript_body_rejects_before_bun_runs() {
    let (_temp_dir, fixture_dir) = copy_m21_chain3_fixture("aligned");
    remove_typescript_body(
        &fixture_dir.join("units/pricing/pricing_total_wrapper_aligned.unit.spec"),
    );

    let marker_path = fixture_dir.join("bun-invoked.txt");
    let fake_bun = format!(
        "#!/bin/sh\nif [ \"$1\" = \"--version\" ]; then\n  echo '1.2.15'\n  exit 0\nfi\necho invoked > \"{}\"\nexit 99\n",
        marker_path.display()
    );
    let path_override = path_with_fake_bun(&fixture_dir, &fake_bun);

    let output = run_in_with_env(
        &fixture_dir,
        &[
            "test",
            "units/pricing/checkout_chain3_aligned.unit.spec",
            "--target-language",
            "typescript",
        ],
        &[("PATH", path_override.as_os_str())],
    );
    assert!(
        !output.status.success(),
        "chain3 targets with missing direct-dep TypeScript bodies should be rejected"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains(
            "TypeScript same-tree local target requires every reachable unit to author body.typescript in M59"
        ),
        "{stderr}"
    );
    assert!(
        !marker_path.exists(),
        "missing body.typescript rejection should happen before Bun build/test execution"
    );
}

#[cfg(unix)]
#[test]
fn typescript_nested_chain3_missing_nested_typescript_body_rejects_before_bun_runs() {
    let (_temp_dir, fixture_dir) = copy_m21_chain3_fixture("aligned");
    remove_typescript_body(&fixture_dir.join("units/pricing/base_nested_chain3_aligned.unit.spec"));

    let marker_path = fixture_dir.join("bun-invoked.txt");
    let fake_bun = format!(
        "#!/bin/sh\nif [ \"$1\" = \"--version\" ]; then\n  echo '1.2.15'\n  exit 0\nfi\necho invoked > \"{}\"\nexit 99\n",
        marker_path.display()
    );
    let path_override = path_with_fake_bun(&fixture_dir, &fake_bun);

    let output = run_in_with_env(
        &fixture_dir,
        &[
            "test",
            "units/pricing/checkout_nested_chain3_aligned.unit.spec",
            "--target-language",
            "typescript",
        ],
        &[("PATH", path_override.as_os_str())],
    );
    assert!(
        !output.status.success(),
        "nested chain3 targets with a missing nested body.typescript should be rejected"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("TypeScript same-tree local target requires every reachable unit to author body.typescript in M59"),
        "{stderr}"
    );
    assert!(
        !marker_path.exists(),
        "nested chain3 missing-body rejection should happen before Bun build/test execution"
    );
}

#[cfg(unix)]
#[test]
fn typescript_chain3_wrong_dep_order_reaches_bun_in_m59() {
    let (_temp_dir, fixture_dir) = copy_m21_chain3_fixture("aligned");
    replace_in_file(
        &fixture_dir.join("units/pricing/checkout_chain3_aligned.unit.spec"),
        "deps:\n  - pricing/pricing_total_wrapper_aligned\n  - pricing/pricing_tax_leaf_aligned\n  - pricing/pricing_discount_leaf_aligned\n",
        "deps:\n  - pricing/pricing_tax_leaf_aligned\n  - pricing/pricing_total_wrapper_aligned\n  - pricing/pricing_discount_leaf_aligned\n",
    );

    let marker_path = fixture_dir.join("bun-invoked.txt");
    let fake_bun = format!(
        "#!/bin/sh\nif [ \"$1\" = \"--version\" ]; then\n  echo '1.2.15'\n  exit 0\nfi\necho invoked > \"{}\"\nexit 99\n",
        marker_path.display()
    );
    let path_override = path_with_fake_bun(&fixture_dir, &fake_bun);

    let output = run_in_with_env(
        &fixture_dir,
        &[
            "test",
            "units/pricing/checkout_chain3_aligned.unit.spec",
            "--target-language",
            "typescript",
        ],
        &[("PATH", path_override.as_os_str())],
    );
    assert!(
        !output.status.success(),
        "chain3 targets with reordered local deps should still fail under the fake Bun"
    );
    assert!(
        marker_path.exists(),
        "M59 same-tree local graph validation should allow reordered local deps to reach Bun"
    );
}

#[cfg(unix)]
#[test]
fn typescript_nested_chain3_wrong_dep_order_reaches_bun_in_m59() {
    let (_temp_dir, fixture_dir) = copy_m21_chain3_fixture("aligned");
    replace_in_file(
        &fixture_dir.join("units/pricing/checkout_nested_chain3_aligned.unit.spec"),
        "deps:\n  - pricing/base_nested_chain3_aligned\n  - pricing/pricing_tax_leaf_aligned\n  - pricing/pricing_discount_leaf_aligned\n",
        "deps:\n  - pricing/pricing_tax_leaf_aligned\n  - pricing/base_nested_chain3_aligned\n  - pricing/pricing_discount_leaf_aligned\n",
    );

    let marker_path = fixture_dir.join("bun-invoked.txt");
    let fake_bun = format!(
        "#!/bin/sh\nif [ \"$1\" = \"--version\" ]; then\n  echo '1.2.15'\n  exit 0\nfi\necho invoked > \"{}\"\nexit 99\n",
        marker_path.display()
    );
    let path_override = path_with_fake_bun(&fixture_dir, &fake_bun);

    let output = run_in_with_env(
        &fixture_dir,
        &[
            "test",
            "units/pricing/checkout_nested_chain3_aligned.unit.spec",
            "--target-language",
            "typescript",
        ],
        &[("PATH", path_override.as_os_str())],
    );
    assert!(
        !output.status.success(),
        "nested chain3 targets with reordered local deps should still fail under the fake Bun"
    );
    assert!(
        marker_path.exists(),
        "M59 same-tree local graph validation should allow reordered nested local deps to reach Bun"
    );
}

#[cfg(unix)]
#[test]
fn typescript_nested_chain3_cross_library_recursive_first_dep_executes_with_bun() {
    if !bun_available() {
        return;
    }

    let (_temp_dir, app_dir, _shared_crate_dir, shared_spec_dir) =
        copy_crosslib_typescript_example();
    let target = write_cross_library_nested_chain3_specs(&app_dir, &shared_spec_dir);

    let output = run_in(
        &app_dir,
        &[
            "test",
            "units/pricing/checkout_nested_chain3.unit.spec",
            "--target-language",
            "typescript",
        ],
    );
    assert_output_success(
        "recursive cross-library nested chain3 roots should execute in the bounded TypeScript lane",
        &output,
    );

    let passport = read_passport_json(
        &target
            .parent()
            .unwrap()
            .join("checkout_nested_chain3.spec.passport.json"),
    );
    assert_eq!(
        passport["target_proofs"]["typescript"]["evidence"]["build_status"],
        "pass"
    );
    assert_eq!(
        passport["target_proofs"]["typescript"]["evidence"]["test_results"][0]["status"],
        "pass"
    );
}

#[cfg(unix)]
#[test]
fn typescript_recursive_cross_library_member_wrong_dep_order_rejects_before_bun_runs() {
    let (temp_dir, app_dir, _shared_crate_dir, shared_spec_dir) =
        copy_crosslib_typescript_example();
    write_cross_library_nested_chain3_specs(&app_dir, &shared_spec_dir);
    replace_in_file(
        &shared_spec_dir.join("units/pricing/base_nested_chain3.unit.spec"),
        "deps:\n  - pricing/calculate_total\n  - pricing/apply_tax\n  - pricing/apply_discount\n",
        "deps:\n  - pricing/apply_tax\n  - pricing/calculate_total\n  - pricing/apply_discount\n",
    );

    let marker_path = app_dir.join("bun-invoked.txt");
    let fake_bun = format!(
        "#!/bin/sh\nif [ \"$1\" = \"--version\" ]; then\n  echo '1.2.15'\n  exit 0\nfi\necho invoked > \"{}\"\nexit 99\n",
        marker_path.display()
    );
    let path_override = path_with_fake_bun(temp_dir.path(), &fake_bun);

    let output = run_in_with_env(
        &app_dir,
        &[
            "test",
            "units/pricing/checkout_nested_chain3.unit.spec",
            "--target-language",
            "typescript",
        ],
        &[("PATH", path_override.as_os_str())],
    );
    assert!(
        !output.status.success(),
        "recursive shared members with the wrong chain3 dep order should be rejected"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("requires direct deps to classify as"),
        "{stderr}"
    );
    assert!(
        !marker_path.exists(),
        "shared recursive wrong-order rejection should happen before Bun build/test execution"
    );
}

#[cfg(unix)]
#[test]
fn typescript_recursive_cross_library_member_missing_typescript_body_rejects_before_bun_runs() {
    let (temp_dir, app_dir, _shared_crate_dir, shared_spec_dir) =
        copy_crosslib_typescript_example();
    write_cross_library_nested_chain3_specs(&app_dir, &shared_spec_dir);
    remove_typescript_body(&shared_spec_dir.join("units/pricing/base_nested_chain3.unit.spec"));

    let marker_path = app_dir.join("bun-invoked.txt");
    let fake_bun = format!(
        "#!/bin/sh\nif [ \"$1\" = \"--version\" ]; then\n  echo '1.2.15'\n  exit 0\nfi\necho invoked > \"{}\"\nexit 99\n",
        marker_path.display()
    );
    let path_override = path_with_fake_bun(temp_dir.path(), &fake_bun);

    let output = run_in_with_env(
        &app_dir,
        &[
            "test",
            "units/pricing/checkout_nested_chain3.unit.spec",
            "--target-language",
            "typescript",
        ],
        &[("PATH", path_override.as_os_str())],
    );
    assert!(
        !output.status.success(),
        "recursive shared members without body.typescript should be rejected"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("body.typescript"), "{stderr}");
    assert!(
        !marker_path.exists(),
        "shared recursive missing-body rejection should happen before Bun build/test execution"
    );
}

#[cfg(unix)]
#[test]
fn typescript_recursive_cross_library_member_unresolved_dep_rejects_before_bun_runs() {
    let (temp_dir, app_dir, _shared_crate_dir, shared_spec_dir) =
        copy_crosslib_typescript_example();
    write_cross_library_nested_chain3_specs(&app_dir, &shared_spec_dir);
    replace_in_file(
        &shared_spec_dir.join("units/pricing/base_nested_chain3.unit.spec"),
        "  - pricing/calculate_total\n",
        "  - pricing/missing_total\n",
    );

    let marker_path = app_dir.join("bun-invoked.txt");
    let fake_bun = format!(
        "#!/bin/sh\nif [ \"$1\" = \"--version\" ]; then\n  echo '1.2.15'\n  exit 0\nfi\necho invoked > \"{}\"\nexit 99\n",
        marker_path.display()
    );
    let path_override = path_with_fake_bun(temp_dir.path(), &fake_bun);

    let output = run_in_with_env(
        &app_dir,
        &[
            "test",
            "units/pricing/checkout_nested_chain3.unit.spec",
            "--target-language",
            "typescript",
        ],
        &[("PATH", path_override.as_os_str())],
    );
    assert!(
        !output.status.success(),
        "recursive shared members with unresolved deps should be rejected"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("pricing/missing_total"), "{stderr}");
    assert!(
        !marker_path.exists(),
        "shared recursive unresolved-dep rejection should happen before Bun build/test execution"
    );
}

#[cfg(unix)]
#[test]
fn typescript_cross_library_helper_missing_typescript_body_rejects_before_bun_runs() {
    let root = repo_root();
    let temp_dir = temp_repo_dir();
    let app_dir = temp_dir.path().join("crosslib-app");
    let shared_crate_dir = temp_dir.path().join("shared-crate");
    let shared_spec_dir = temp_dir.path().join("shared-spec");

    fs::write(
        temp_dir.path().join(".git"),
        "gitdir: .git/modules/spec-tests\n",
    )
    .unwrap();
    copy_git_tracked_dir(&root.join("examples/crosslib-app"), &app_dir).unwrap();
    copy_git_tracked_dir(&root.join("examples/shared-crate"), &shared_crate_dir).unwrap();
    copy_git_tracked_dir(&root.join("examples/shared-spec"), &shared_spec_dir).unwrap();

    remove_typescript_body(&shared_spec_dir.join("units/money/round.unit.spec"));

    let marker_path = app_dir.join("bun-invoked.txt");
    let fake_bun = format!(
        "#!/bin/sh\nif [ \"$1\" = \"--version\" ]; then\n  echo '1.2.15'\n  exit 0\nfi\necho invoked > \"{}\"\nexit 99\n",
        marker_path.display()
    );
    let path_override = path_with_fake_bun(temp_dir.path(), &fake_bun);

    let output = run_in_with_env(
        &app_dir,
        &[
            "test",
            "units/pricing/apply_tax.unit.spec",
            "--target-language",
            "typescript",
        ],
        &[("PATH", path_override.as_os_str())],
    );
    assert!(
        !output.status.success(),
        "cross-library helper without body.typescript should be rejected"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains(
            "TypeScript target requires the direct helper dep to author body.typescript in M55"
        ),
        "{stderr}"
    );
    assert!(
        !marker_path.exists(),
        "shared-helper body rejection should happen before Bun build/test execution"
    );
}

#[test]
fn export_additively_preserves_rust_proof_and_includes_typescript_proof() {
    if !cargo_available() || !bun_available() {
        return;
    }

    let (_temp_dir, ecommerce_dir) = copy_ecommerce_example();

    let rust_test_output = run_in(
        &ecommerce_dir,
        &["test", "units/pricing/apply_tax.unit.spec"],
    );
    assert_output_success("rust proof should succeed before export", &rust_test_output);

    let typescript_test_output = run_in(
        &ecommerce_dir,
        &[
            "test",
            "units/pricing/apply_tax.unit.spec",
            "--target-language",
            "typescript",
        ],
    );
    assert_output_success(
        "TypeScript proof should succeed before export",
        &typescript_test_output,
    );

    let export_output = run_in(&ecommerce_dir, &["export", "units", "--format", "json"]);
    assert_output_success(
        "export should include additive TypeScript target proofs",
        &export_output,
    );
    let export_json = parse_stdout_json(&export_output);
    let passport = export_json["passports"]
        .as_array()
        .unwrap()
        .iter()
        .find(|entry| entry["id"] == "pricing/apply_tax")
        .expect("expected apply_tax passport in export");
    assert_eq!(passport["evidence"]["build_status"], "pass");
    assert_eq!(
        passport["target_proofs"]["rust"]["evidence"]["build_status"],
        "pass"
    );
    assert_eq!(
        passport["target_proofs"]["typescript"]["evidence"]["build_status"],
        "pass"
    );
}
