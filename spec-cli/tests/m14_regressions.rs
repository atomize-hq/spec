use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use tempfile::TempDir;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("workspace root")
        .to_path_buf()
}

fn copy_dir_all(src: &Path, dst: &Path) {
    fs::create_dir_all(dst).unwrap();

    for entry in fs::read_dir(src).unwrap() {
        let entry = entry.unwrap();
        let entry_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if entry.file_type().unwrap().is_dir() {
            copy_dir_all(&entry_path, &dst_path);
        } else {
            fs::copy(&entry_path, &dst_path).unwrap();
        }
    }
}

fn run_spec(cwd: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_spec"))
        .current_dir(cwd)
        .args(args)
        .output()
        .unwrap()
}

fn assert_success(output: &Output, context: &str) {
    assert!(
        output.status.success(),
        "{context} failed\nstatus: {:?}\nstdout:\n{}\nstderr:\n{}",
        output.status.code(),
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn assert_exit_code(output: &Output, expected: i32, context: &str) {
    assert_eq!(
        output.status.code(),
        Some(expected),
        "{context} unexpected exit code\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn copied_ecommerce_fixture() -> (TempDir, PathBuf) {
    let temp_dir = TempDir::new().unwrap();
    let fixture_dst = temp_dir.path().join("ecommerce");
    copy_dir_all(&repo_root().join("examples/ecommerce"), &fixture_dst);
    (temp_dir, fixture_dst)
}

fn status_unit<'a>(status_json: &'a Value, id: &str) -> &'a Value {
    status_json["units"]
        .as_array()
        .unwrap()
        .iter()
        .find(|unit| unit["id"] == id)
        .unwrap()
}

fn exported_passport<'a>(bundle_json: &'a Value, id: &str) -> &'a Value {
    bundle_json["passports"]
        .as_array()
        .unwrap()
        .iter()
        .find(|passport| passport["id"] == id)
        .unwrap()
}

fn read_json(path: &Path) -> Value {
    serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap()
}

fn proof_coverage_surfaces<'a>(passport: &'a Value, coverage_id: &str) -> &'a Value {
    passport["proof_coverage"]
        .as_array()
        .unwrap()
        .iter()
        .find(|coverage| coverage["id"] == coverage_id)
        .map(|coverage| &coverage["surfaces"])
        .unwrap()
}

fn semantic_review_fixture_root(fixture_dst: &Path, name: &str) -> PathBuf {
    fixture_dst.join(".m15").join(name)
}

fn write_semantic_review_molecule(
    wedge_root: &Path,
    id: &str,
    intent: &str,
    body: &str,
) -> PathBuf {
    let path = wedge_root.join("units/pricing/discount_policy_semantic_review.test.spec");
    fs::write(
        &path,
        format!(
            "id: {id}\nspec_version: \"0.3.0\"\nintent:\n  why: {intent}\ncovers:\n  - pricing/discount_policy\nimports:\n  - rust_decimal::Decimal\nbody:\n  rust: |\n{body}\n"
        ),
    )
    .unwrap();
    path
}

fn assert_semantic_review(review: &Value, verdict: &str, reason_codes: &[&str], summary: &str) {
    assert_eq!(review["verdict"], verdict);
    if reason_codes.is_empty() {
        assert!(review.get("reason_codes").is_none() || review["reason_codes"].is_null());
    } else {
        assert_eq!(
            review["reason_codes"],
            serde_json::json!(reason_codes.to_vec())
        );
    }
    assert_eq!(review["summary"], summary);
    assert_eq!(review["evaluator_scope"], "supported_sum_surface");
}

#[test]
fn spec_build_does_not_clear_unit_staleness_without_retest() {
    let (_temp_dir, fixture_dst) = copied_ecommerce_fixture();
    let units_dir = fixture_dst.join("units");
    let output_dir = fixture_dst.join("src/generated");

    let test_output = run_spec(
        &fixture_dst,
        &[
            "test",
            units_dir.to_str().unwrap(),
            "--output",
            output_dir.to_str().unwrap(),
            "--crate-root",
            fixture_dst.to_str().unwrap(),
        ],
    );
    assert_success(&test_output, "spec test");

    let unit_path = fixture_dst.join("units/pricing/discount_policy.unit.spec");
    let source = fs::read_to_string(&unit_path).unwrap();
    fs::write(
        &unit_path,
        source.replace(
            "Represent mutually exclusive discount strategies for checkout pricing.",
            "Represent mutually exclusive discount strategies for checkout pricing with a revised authored truth.",
        ),
    )
    .unwrap();

    let build_output = run_spec(
        &fixture_dst,
        &[
            "build",
            units_dir.to_str().unwrap(),
            "--output",
            output_dir.to_str().unwrap(),
        ],
    );
    assert_success(&build_output, "spec build");

    let stored_passport =
        read_json(&fixture_dst.join("units/pricing/discount_policy.spec.passport.json"));

    let status_output = run_spec(
        &fixture_dst,
        &["status", fixture_dst.to_str().unwrap(), "--format", "json"],
    );
    assert_exit_code(&status_output, 1, "spec status");
    let status_json: Value = serde_json::from_slice(&status_output.stdout).unwrap();
    let unit = status_unit(&status_json, "pricing/discount_policy");

    assert_eq!(unit["status"], "stale");
    assert_eq!(unit["reason"], "authored truth changed since last test");
    assert_eq!(
        stored_passport["freshness"]["authored_truth_status"],
        "stale"
    );
    assert_eq!(
        stored_passport["freshness"]["backend_execution_status"],
        "fresh"
    );
    assert_eq!(stored_passport["escape_hatch_gate"]["status"], "open");
    assert_eq!(
        stored_passport["escape_hatch_gate"]["missing_surfaces"],
        serde_json::json!(["atom", "molecule"])
    );
}

#[test]
fn spec_export_matches_status_for_legacy_passports_missing_freshness() {
    let (_temp_dir, fixture_dst) = copied_ecommerce_fixture();
    let units_dir = fixture_dst.join("units");
    let output_dir = fixture_dst.join("src/generated");
    let passport_path = fixture_dst.join("units/pricing/discount_policy.spec.passport.json");

    let test_output = run_spec(
        &fixture_dst,
        &[
            "test",
            units_dir.to_str().unwrap(),
            "--output",
            output_dir.to_str().unwrap(),
            "--crate-root",
            fixture_dst.to_str().unwrap(),
        ],
    );
    assert_success(&test_output, "spec test");

    let mut passport_json: Value =
        serde_json::from_str(&fs::read_to_string(&passport_path).unwrap()).unwrap();
    passport_json.as_object_mut().unwrap().remove("freshness");
    passport_json
        .as_object_mut()
        .unwrap()
        .remove("freshness_anchor");
    fs::write(
        &passport_path,
        serde_json::to_string_pretty(&passport_json).unwrap(),
    )
    .unwrap();

    let unit_path = fixture_dst.join("units/pricing/discount_policy.unit.spec");
    let source = fs::read_to_string(&unit_path).unwrap();
    fs::write(
        &unit_path,
        source.replace(
            "Represent mutually exclusive discount strategies for checkout pricing.",
            "Represent mutually exclusive discount strategies for checkout pricing with a revised authored truth.",
        ),
    )
    .unwrap();

    let status_output = run_spec(
        &fixture_dst,
        &["status", fixture_dst.to_str().unwrap(), "--format", "json"],
    );
    assert_exit_code(&status_output, 1, "spec status");
    let status_json: Value = serde_json::from_slice(&status_output.stdout).unwrap();
    let status_unit = status_unit(&status_json, "pricing/discount_policy");

    let export_output = run_spec(&fixture_dst, &["export", fixture_dst.to_str().unwrap()]);
    assert_success(&export_output, "spec export");
    let export_json: Value = serde_json::from_slice(&export_output.stdout).unwrap();
    let passport = exported_passport(&export_json, "pricing/discount_policy");

    assert_eq!(status_unit["status"], "stale");
    assert_eq!(status_unit["freshness"]["authored_truth_status"], "stale");
    assert_eq!(
        status_unit["freshness"]["backend_execution_status"],
        "unknown"
    );
    assert_eq!(passport["freshness"]["authored_truth_status"], "stale");
    assert_eq!(passport["freshness"]["backend_execution_status"], "unknown");
}

#[test]
fn spec_plan_validate_accepts_valid_but_partial_add_plans() {
    let (_temp_dir, fixture_dst) = copied_ecommerce_fixture();
    let plan_path = fixture_dst.join("plans/refactors/add-tiered-rate.plan.spec");

    fs::create_dir_all(plan_path.parent().unwrap()).unwrap();
    fs::write(
        &plan_path,
        r#"id: add-tiered-rate
intent:
  why: "Add a tiered discount unit while keeping impact truthful."
changes:
  - unit: pricing/tiered_rate
    action: add
    acceptance:
      validate:
        - pricing/tiered_rate
      molecule_tests: []
      notes:
        - "new unit is intentionally unresolved in the current graph"
notes:
  - "M10 plans are local-library only."
"#,
    )
    .unwrap();

    let output = run_spec(
        &fixture_dst,
        &[
            "plan",
            "validate",
            plan_path.to_str().unwrap(),
            "--format",
            "json",
        ],
    );
    assert_success(&output, "spec plan validate");

    let json: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(json["status"], "valid");
    assert_eq!(json["computed_impact"]["status"], "partial");
    assert_eq!(
        json["computed_impact"]["unresolved"][0]["unit"],
        "pricing/tiered_rate"
    );
    assert_eq!(json["acceptance_closure"]["status"], "closed");
}

#[test]
fn canonical_escape_hatch_gate_closes_across_truth_surfaces() {
    let (_temp_dir, fixture_dst) = copied_ecommerce_fixture();
    let unit_path = fixture_dst.join("units/pricing/discount_policy.unit.spec");
    let molecule_path = fixture_dst.join("units/pricing/discount_policy_checkout_flow.test.spec");
    let passport_path = fixture_dst.join("units/pricing/discount_policy.spec.passport.json");

    let unit_test_output = run_spec(&fixture_dst, &["test", unit_path.to_str().unwrap()]);
    assert_success(&unit_test_output, "single-file unit spec test");

    let molecule_test_output = run_spec(&fixture_dst, &["test", molecule_path.to_str().unwrap()]);
    assert_success(&molecule_test_output, "single-file molecule spec test");

    let passport_json = read_json(&passport_path);
    assert_eq!(passport_json["markers"].as_array().unwrap().len(), 6);
    assert_eq!(passport_json["escape_hatch_gate"]["status"], "closed");
    assert_eq!(
        proof_coverage_surfaces(&passport_json, "variant.none"),
        &serde_json::json!(["atom", "molecule"])
    );
    assert_eq!(
        passport_json["escape_hatch_gate"]["required_surfaces"],
        serde_json::json!(["atom", "molecule"])
    );
    assert_eq!(
        passport_json["escape_hatch_gate"]["present_surfaces"],
        serde_json::json!(["atom", "molecule"])
    );
    assert_eq!(
        passport_json["escape_hatch_gate"]["missing_surfaces"],
        serde_json::json!([])
    );
    assert!(passport_json["escape_hatch_gate"]["reason"].is_null());

    let status_output = run_spec(
        &fixture_dst,
        &["status", fixture_dst.to_str().unwrap(), "--format", "json"],
    );
    assert_success(&status_output, "spec status");
    let status_json: Value = serde_json::from_slice(&status_output.stdout).unwrap();
    let discount_policy_status = status_unit(&status_json, "pricing/discount_policy");
    assert_eq!(discount_policy_status["status"], "valid");
    assert_eq!(
        discount_policy_status["escape_hatch_gate"]["status"],
        "closed"
    );
    assert_eq!(
        discount_policy_status["escape_hatch_gate"]["present_surfaces"],
        serde_json::json!(["atom", "molecule"])
    );

    let export_output = run_spec(&fixture_dst, &["export", fixture_dst.to_str().unwrap()]);
    assert_success(&export_output, "spec export");
    let export_json: Value = serde_json::from_slice(&export_output.stdout).unwrap();
    let exported = exported_passport(&export_json, "pricing/discount_policy");
    assert_eq!(exported["escape_hatch_gate"]["status"], "closed");
    assert_eq!(
        proof_coverage_surfaces(exported, "variant.none"),
        &serde_json::json!(["atom", "molecule"])
    );
    assert_eq!(
        exported["escape_hatch_gate"]["present_surfaces"],
        serde_json::json!(["atom", "molecule"])
    );

    let apply_tax_status = status_unit(&status_json, "pricing/apply_tax");
    assert!(apply_tax_status.get("escape_hatch_gate").is_none());
    let apply_tax_export = exported_passport(&export_json, "pricing/apply_tax");
    assert!(apply_tax_export.get("escape_hatch_gate").is_none());
}

#[test]
fn stale_marked_seam_reopens_gate_for_status_and_export() {
    let (_temp_dir, fixture_dst) = copied_ecommerce_fixture();
    let unit_path = fixture_dst.join("units/pricing/discount_policy.unit.spec");
    let molecule_path = fixture_dst.join("units/pricing/discount_policy_checkout_flow.test.spec");
    let passport_path = fixture_dst.join("units/pricing/discount_policy.spec.passport.json");

    let unit_test_output = run_spec(&fixture_dst, &["test", unit_path.to_str().unwrap()]);
    assert_success(&unit_test_output, "single-file unit spec test");

    let molecule_test_output = run_spec(&fixture_dst, &["test", molecule_path.to_str().unwrap()]);
    assert_success(&molecule_test_output, "single-file molecule spec test");

    let source = fs::read_to_string(&unit_path).unwrap();
    fs::write(
        &unit_path,
        source.replace(
            "Represent mutually exclusive discount strategies for checkout pricing.",
            "Represent mutually exclusive discount strategies for checkout pricing with a revised authored truth.",
        ),
    )
    .unwrap();

    let stored_passport = read_json(&passport_path);
    assert_eq!(stored_passport["escape_hatch_gate"]["status"], "closed");

    let status_output = run_spec(
        &fixture_dst,
        &["status", fixture_dst.to_str().unwrap(), "--format", "json"],
    );
    assert_exit_code(&status_output, 1, "spec status");
    let status_json: Value = serde_json::from_slice(&status_output.stdout).unwrap();
    let status_unit = status_unit(&status_json, "pricing/discount_policy");
    assert_eq!(status_unit["status"], "stale");
    assert_eq!(status_unit["escape_hatch_gate"]["status"], "open");
    assert_eq!(
        status_unit["escape_hatch_gate"]["present_surfaces"],
        serde_json::json!([])
    );
    assert_eq!(
        status_unit["escape_hatch_gate"]["missing_surfaces"],
        serde_json::json!(["atom", "molecule"])
    );

    let export_output = run_spec(&fixture_dst, &["export", fixture_dst.to_str().unwrap()]);
    assert_success(&export_output, "spec export");
    let export_json: Value = serde_json::from_slice(&export_output.stdout).unwrap();
    let exported = exported_passport(&export_json, "pricing/discount_policy");
    assert_eq!(exported["escape_hatch_gate"]["status"], "open");
    assert_eq!(
        proof_coverage_surfaces(exported, "variant.none"),
        &serde_json::json!(["implicit_only"])
    );
    assert_eq!(
        exported["escape_hatch_gate"]["present_surfaces"],
        serde_json::json!([])
    );
    assert_eq!(
        exported["escape_hatch_gate"]["missing_surfaces"],
        serde_json::json!(["atom", "molecule"])
    );
}

#[test]
fn single_file_unit_test_leaves_gate_open_when_molecule_proof_is_missing() {
    let (_temp_dir, fixture_dst) = copied_ecommerce_fixture();
    let unit_path = fixture_dst.join("units/pricing/discount_policy.unit.spec");
    let molecule_path = fixture_dst.join("units/pricing/discount_policy_checkout_flow.test.spec");
    let passport_path = fixture_dst.join("units/pricing/discount_policy.spec.passport.json");

    fs::remove_file(&molecule_path).unwrap();

    let unit_test_output = run_spec(&fixture_dst, &["test", unit_path.to_str().unwrap()]);
    assert_success(&unit_test_output, "single-file unit spec test");

    let passport_json = read_json(&passport_path);
    assert_eq!(passport_json["escape_hatch_gate"]["status"], "open");
    assert_eq!(
        passport_json["escape_hatch_gate"]["missing_surfaces"],
        serde_json::json!(["molecule"])
    );
    assert_eq!(
        passport_json["escape_hatch_gate"]["reason"],
        "missing required escape-hatch proof: molecule"
    );

    let status_output = run_spec(
        &fixture_dst,
        &["status", fixture_dst.to_str().unwrap(), "--format", "json"],
    );
    assert_exit_code(&status_output, 1, "spec status");
    let status_json: Value = serde_json::from_slice(&status_output.stdout).unwrap();
    let status_unit = status_unit(&status_json, "pricing/discount_policy");
    assert_eq!(status_unit["status"], "incomplete");
    assert_eq!(
        status_unit["reason"],
        "missing required escape-hatch proof: molecule"
    );
    assert_eq!(status_unit["escape_hatch_gate"]["status"], "open");

    let export_output = run_spec(&fixture_dst, &["export", fixture_dst.to_str().unwrap()]);
    assert_success(&export_output, "spec export");
    let export_json: Value = serde_json::from_slice(&export_output.stdout).unwrap();
    let exported = exported_passport(&export_json, "pricing/discount_policy");
    assert_eq!(exported["escape_hatch_gate"]["status"], "open");
    assert_eq!(
        proof_coverage_surfaces(exported, "variant.none"),
        &serde_json::json!(["atom"])
    );
    assert_eq!(
        exported["escape_hatch_gate"]["missing_surfaces"],
        serde_json::json!(["molecule"])
    );
}

#[test]
fn single_file_molecule_test_refreshes_covered_passport_gate() {
    let (_temp_dir, fixture_dst) = copied_ecommerce_fixture();
    let unit_path = fixture_dst.join("units/pricing/discount_policy.unit.spec");
    let molecule_path = fixture_dst.join("units/pricing/discount_policy_checkout_flow.test.spec");
    let passport_path = fixture_dst.join("units/pricing/discount_policy.spec.passport.json");

    let unit_test_output = run_spec(&fixture_dst, &["test", unit_path.to_str().unwrap()]);
    assert_success(&unit_test_output, "single-file unit spec test");
    let before = read_json(&passport_path);
    assert_eq!(before["escape_hatch_gate"]["status"], "open");
    assert_eq!(
        before["escape_hatch_gate"]["missing_surfaces"],
        serde_json::json!(["molecule"])
    );

    let molecule_test_output = run_spec(&fixture_dst, &["test", molecule_path.to_str().unwrap()]);
    assert_success(&molecule_test_output, "single-file molecule spec test");

    let after = read_json(&passport_path);
    assert_eq!(after["escape_hatch_gate"]["status"], "closed");
    assert_eq!(
        after["escape_hatch_gate"]["present_surfaces"],
        serde_json::json!(["atom", "molecule"])
    );
}

#[test]
fn marked_seam_without_atom_proof_reports_gate_open() {
    let (_temp_dir, fixture_dst) = copied_ecommerce_fixture();
    let units_dir = fixture_dst.join("units");
    let output_dir = fixture_dst.join("src/generated");
    let unit_path = fixture_dst.join("units/pricing/discount_policy.unit.spec");
    let passport_path = fixture_dst.join("units/pricing/discount_policy.spec.passport.json");

    let source = fs::read_to_string(&unit_path).unwrap();
    fs::write(
        &unit_path,
        source.replace(
            r#"local_tests:
  - id: variant_none
    expect: 'DiscountPolicy::None.discount_amount(rust_decimal::Decimal::new(1500, 2)) == rust_decimal::Decimal::ZERO && DiscountPolicy::None.discounted_subtotal(rust_decimal::Decimal::new(1500, 2)) == rust_decimal::Decimal::new(1500, 2)'
  - id: variant_percentage
    expect: DiscountPolicy::None.percentage_example_holds()
  - id: variant_fixed_amount
    expect: DiscountPolicy::None.fixed_amount_example_holds()
  - id: behavior_fixed_amount_capped
    expect: DiscountPolicy::None.fixed_amount_capped_behavior_holds()
"#,
            "local_tests: []\n",
        ),
    )
    .unwrap();

    let test_output = run_spec(
        &fixture_dst,
        &[
            "test",
            units_dir.to_str().unwrap(),
            "--output",
            output_dir.to_str().unwrap(),
            "--crate-root",
            fixture_dst.to_str().unwrap(),
        ],
    );
    assert_success(&test_output, "directory spec test");

    let passport_json = read_json(&passport_path);
    assert_eq!(passport_json["escape_hatch_gate"]["status"], "open");
    assert_eq!(
        passport_json["escape_hatch_gate"]["missing_surfaces"],
        serde_json::json!(["atom"])
    );
    assert_eq!(
        passport_json["escape_hatch_gate"]["reason"],
        "missing required escape-hatch proof: atom"
    );

    let status_output = run_spec(
        &fixture_dst,
        &["status", fixture_dst.to_str().unwrap(), "--format", "json"],
    );
    assert_exit_code(&status_output, 1, "spec status");
    let status_json: Value = serde_json::from_slice(&status_output.stdout).unwrap();
    let status_unit = status_unit(&status_json, "pricing/discount_policy");
    assert_eq!(status_unit["status"], "incomplete");
    assert_eq!(
        status_unit["escape_hatch_gate"]["missing_surfaces"],
        serde_json::json!(["atom"])
    );

    let export_output = run_spec(&fixture_dst, &["export", fixture_dst.to_str().unwrap()]);
    assert_success(&export_output, "spec export");
    let export_json: Value = serde_json::from_slice(&export_output.stdout).unwrap();
    let exported = exported_passport(&export_json, "pricing/discount_policy");
    assert_eq!(exported["escape_hatch_gate"]["status"], "open");
    assert_eq!(
        proof_coverage_surfaces(exported, "variant.none"),
        &serde_json::json!(["molecule", "implicit_only"])
    );
    assert_eq!(
        exported["escape_hatch_gate"]["missing_surfaces"],
        serde_json::json!(["atom"])
    );
}

#[test]
fn backend_only_drift_reprojects_truth_surfaces_consistently() {
    let (_temp_dir, fixture_dst) = copied_ecommerce_fixture();
    let units_dir = fixture_dst.join("units");
    let output_dir = fixture_dst.join("src/generated");
    let unit_path = fixture_dst.join("units/pricing/discount_policy.unit.spec");
    let passport_path = fixture_dst.join("units/pricing/discount_policy.spec.passport.json");

    let test_output = run_spec(
        &fixture_dst,
        &[
            "test",
            units_dir.to_str().unwrap(),
            "--output",
            output_dir.to_str().unwrap(),
            "--crate-root",
            fixture_dst.to_str().unwrap(),
        ],
    );
    assert_success(&test_output, "spec test");

    let source = fs::read_to_string(&unit_path).unwrap();
    fs::write(
        &unit_path,
        source.replace(
            "Self::Percentage { rate } => subtotal * *rate,",
            "Self::Percentage { rate } => {\n                      // backend-only drift for M14 regression coverage\n                      subtotal * *rate\n                  },",
        ),
    )
    .unwrap();

    let build_output = run_spec(
        &fixture_dst,
        &[
            "build",
            units_dir.to_str().unwrap(),
            "--output",
            output_dir.to_str().unwrap(),
        ],
    );
    assert_success(&build_output, "spec build");

    let stored_passport = read_json(&passport_path);
    let status_output = run_spec(
        &fixture_dst,
        &["status", fixture_dst.to_str().unwrap(), "--format", "json"],
    );
    assert_exit_code(&status_output, 1, "spec status");
    let status_json: Value = serde_json::from_slice(&status_output.stdout).unwrap();
    let status_unit = status_unit(&status_json, "pricing/discount_policy");

    let export_output = run_spec(&fixture_dst, &["export", fixture_dst.to_str().unwrap()]);
    assert_success(&export_output, "spec export");
    let export_json: Value = serde_json::from_slice(&export_output.stdout).unwrap();
    let exported = exported_passport(&export_json, "pricing/discount_policy");

    for surface in [&stored_passport, status_unit, exported] {
        assert_eq!(surface["freshness"]["authored_truth_status"], "fresh");
        assert_eq!(surface["freshness"]["backend_execution_status"], "stale");
        assert_eq!(surface["escape_hatch_gate"]["status"], "open");
        assert_eq!(
            surface["escape_hatch_gate"]["missing_surfaces"],
            serde_json::json!(["atom", "molecule"])
        );
    }
    assert_eq!(status_unit["status"], "stale");
    assert_eq!(
        status_unit["reason"],
        "backend execution changed since last test"
    );
}

#[test]
fn export_omits_molecule_proof_coverage_when_molecule_evidence_is_stale() {
    let (_temp_dir, fixture_dst) = copied_ecommerce_fixture();
    let units_dir = fixture_dst.join("units");
    let output_dir = fixture_dst.join("src/generated");
    let unit_path = fixture_dst.join("units/pricing/discount_policy.unit.spec");

    let test_output = run_spec(
        &fixture_dst,
        &[
            "test",
            units_dir.to_str().unwrap(),
            "--output",
            output_dir.to_str().unwrap(),
            "--crate-root",
            fixture_dst.to_str().unwrap(),
        ],
    );
    assert_success(&test_output, "spec test");

    let source = fs::read_to_string(&unit_path).unwrap();
    fs::write(
        &unit_path,
        source.replace(
            "Represent mutually exclusive discount strategies for checkout pricing.",
            "Represent mutually exclusive discount strategies for checkout pricing with stale molecule evidence.",
        ),
    )
    .unwrap();

    let build_output = run_spec(
        &fixture_dst,
        &[
            "build",
            units_dir.to_str().unwrap(),
            "--output",
            output_dir.to_str().unwrap(),
        ],
    );
    assert_success(&build_output, "spec build");

    let status_output = run_spec(
        &fixture_dst,
        &["status", fixture_dst.to_str().unwrap(), "--format", "json"],
    );
    assert_exit_code(&status_output, 1, "spec status");
    let status_json: Value = serde_json::from_slice(&status_output.stdout).unwrap();
    let status_unit = status_unit(&status_json, "pricing/discount_policy");

    let export_output = run_spec(&fixture_dst, &["export", fixture_dst.to_str().unwrap()]);
    assert_success(&export_output, "spec export");
    let export_json: Value = serde_json::from_slice(&export_output.stdout).unwrap();
    let exported = exported_passport(&export_json, "pricing/discount_policy");

    assert_eq!(
        proof_coverage_surfaces(exported, "variant.none"),
        &serde_json::json!(["implicit_only"])
    );
    assert_eq!(
        status_unit["escape_hatch_gate"]["missing_surfaces"],
        serde_json::json!(["atom", "molecule"])
    );
    assert_eq!(
        exported["escape_hatch_gate"]["missing_surfaces"],
        serde_json::json!(["atom", "molecule"])
    );
}

#[test]
fn export_omits_molecule_proof_coverage_when_molecule_evidence_failed() {
    let (_temp_dir, fixture_dst) = copied_ecommerce_fixture();
    let units_dir = fixture_dst.join("units");
    let output_dir = fixture_dst.join("src/generated");
    let evidence_path =
        fixture_dst.join("units/pricing/discount_policy_checkout_flow.test.evidence.json");

    let test_output = run_spec(
        &fixture_dst,
        &[
            "test",
            units_dir.to_str().unwrap(),
            "--output",
            output_dir.to_str().unwrap(),
            "--crate-root",
            fixture_dst.to_str().unwrap(),
        ],
    );
    assert_success(&test_output, "spec test");

    let mut evidence = read_json(&evidence_path);
    evidence["status"] = serde_json::json!("fail");
    evidence["reason"] = serde_json::json!("forced failure for regression coverage");
    fs::write(
        &evidence_path,
        serde_json::to_string_pretty(&evidence).unwrap(),
    )
    .unwrap();

    let status_output = run_spec(
        &fixture_dst,
        &["status", fixture_dst.to_str().unwrap(), "--format", "json"],
    );
    assert_exit_code(&status_output, 1, "spec status");
    let status_json: Value = serde_json::from_slice(&status_output.stdout).unwrap();
    let status_unit = status_unit(&status_json, "pricing/discount_policy");

    let export_output = run_spec(&fixture_dst, &["export", fixture_dst.to_str().unwrap()]);
    assert_success(&export_output, "spec export");
    let export_json: Value = serde_json::from_slice(&export_output.stdout).unwrap();
    let exported = exported_passport(&export_json, "pricing/discount_policy");

    assert_eq!(
        proof_coverage_surfaces(exported, "variant.none"),
        &serde_json::json!(["atom"])
    );
    assert_eq!(
        status_unit["escape_hatch_gate"]["missing_surfaces"],
        serde_json::json!(["molecule"])
    );
    assert_eq!(
        exported["escape_hatch_gate"]["missing_surfaces"],
        serde_json::json!(["molecule"])
    );
}

#[test]
fn canonical_semantic_review_wedge_projects_aligned_state() {
    let (_temp_dir, fixture_dst) = copied_ecommerce_fixture();
    let wedge_root = semantic_review_fixture_root(&fixture_dst, "aligned");
    let unit_path = wedge_root.join("units/pricing/discount_policy.unit.spec");
    let molecule_path = write_semantic_review_molecule(
        &wedge_root,
        "pricing/discount_policy_semantic_review_aligned",
        "Close the canonical aligned wedge by proving the authored discount semantics through a molecule test.",
        r#"    {
        let subtotal = Decimal::new(1500, 2);
        let none = crate::pricing::discount_policy::DiscountPolicy::None;
        assert_eq!(none.discount_amount(subtotal), Decimal::ZERO);
        assert_eq!(none.discounted_subtotal(subtotal), subtotal);

        let percentage = crate::pricing::discount_policy::DiscountPolicy::Percentage {
            rate: Decimal::new(10, 2),
        };
        assert_eq!(
            percentage.discount_amount(Decimal::new(10000, 2)),
            Decimal::new(1000, 2)
        );
        assert_eq!(
            percentage.discounted_subtotal(Decimal::new(10000, 2)),
            Decimal::new(9000, 2)
        );

        let capped = crate::pricing::discount_policy::DiscountPolicy::FixedAmount {
            amount: Decimal::new(2000, 2),
        };
        assert_eq!(capped.discount_amount(subtotal), subtotal);
        assert_eq!(capped.discounted_subtotal(subtotal), Decimal::ZERO);
    }"#,
    );
    let passport_path = wedge_root.join("units/pricing/discount_policy.spec.passport.json");

    let unit_test_output = run_spec(
        &fixture_dst,
        &[
            "test",
            unit_path.to_str().unwrap(),
            "--crate-root",
            fixture_dst.to_str().unwrap(),
        ],
    );
    assert_success(&unit_test_output, "aligned wedge unit test");

    let molecule_test_output = run_spec(
        &fixture_dst,
        &[
            "test",
            molecule_path.to_str().unwrap(),
            "--crate-root",
            fixture_dst.to_str().unwrap(),
        ],
    );
    assert_success(&molecule_test_output, "aligned wedge molecule test");

    let passport = read_json(&passport_path);
    assert_eq!(passport["escape_hatch_gate"]["status"], "closed");
    assert_semantic_review(
        &passport["semantic_review"],
        "aligned",
        &[],
        "authored semantics and executable lowering agree on the supported sum surface",
    );

    let status_output = run_spec(
        &fixture_dst,
        &["status", wedge_root.to_str().unwrap(), "--format", "json"],
    );
    assert_success(&status_output, "aligned wedge status");
    let status_json: Value = serde_json::from_slice(&status_output.stdout).unwrap();
    let status_unit = status_unit(&status_json, "pricing/discount_policy");
    assert_eq!(status_unit["status"], "valid");
    assert!(status_unit["reason"].is_null());
    assert_eq!(status_unit["escape_hatch_gate"]["status"], "closed");
    assert_semantic_review(
        &status_unit["semantic_review"],
        "aligned",
        &[],
        "authored semantics and executable lowering agree on the supported sum surface",
    );

    let export_output = run_spec(&fixture_dst, &["export", wedge_root.to_str().unwrap()]);
    assert_success(&export_output, "aligned wedge export");
    let export_json: Value = serde_json::from_slice(&export_output.stdout).unwrap();
    let exported = exported_passport(&export_json, "pricing/discount_policy");
    assert_eq!(exported["escape_hatch_gate"]["status"], "closed");
    assert_semantic_review(
        &exported["semantic_review"],
        "aligned",
        &[],
        "authored semantics and executable lowering agree on the supported sum surface",
    );
}

#[test]
fn contradictory_lowering_wedge_projects_backend_only_semantics_leaked() {
    let (_temp_dir, fixture_dst) = copied_ecommerce_fixture();
    let wedge_root = semantic_review_fixture_root(&fixture_dst, "semantic_drift");
    let unit_path = wedge_root.join("units/pricing/discount_policy.unit.spec");
    let molecule_path = write_semantic_review_molecule(
        &wedge_root,
        "pricing/discount_policy_semantic_review_semantic_drift",
        "Close the contradictory-lowering wedge so semantic review is the only failing signal.",
        r#"    {
        let subtotal = Decimal::new(1500, 2);
        let uncapped = crate::pricing::discount_policy::DiscountPolicy::FixedAmount {
            amount: Decimal::new(2000, 2),
        };

        assert_eq!(uncapped.discount_amount(subtotal), Decimal::new(2000, 2));
        assert_eq!(
            uncapped.discounted_subtotal(subtotal),
            Decimal::new(-500, 2)
        );
    }"#,
    );
    let passport_path = wedge_root.join("units/pricing/discount_policy.spec.passport.json");

    let unit_test_output = run_spec(
        &fixture_dst,
        &[
            "test",
            unit_path.to_str().unwrap(),
            "--crate-root",
            fixture_dst.to_str().unwrap(),
        ],
    );
    assert_success(&unit_test_output, "semantic drift wedge unit test");

    let molecule_test_output = run_spec(
        &fixture_dst,
        &[
            "test",
            molecule_path.to_str().unwrap(),
            "--crate-root",
            fixture_dst.to_str().unwrap(),
        ],
    );
    assert_success(&molecule_test_output, "semantic drift wedge molecule test");

    let passport = read_json(&passport_path);
    assert_eq!(passport["escape_hatch_gate"]["status"], "closed");
    assert_semantic_review(
        &passport["semantic_review"],
        "backend_only_semantics_leaked",
        &["method_body_missing_cap_behavior"],
        "executable lowering contradicts authored semantic claims",
    );

    let status_output = run_spec(
        &fixture_dst,
        &["status", wedge_root.to_str().unwrap(), "--format", "json"],
    );
    assert_exit_code(&status_output, 1, "semantic drift wedge status");
    let status_json: Value = serde_json::from_slice(&status_output.stdout).unwrap();
    let status_unit = status_unit(&status_json, "pricing/discount_policy");
    assert_eq!(status_unit["status"], "failing");
    assert_eq!(
        status_unit["reason"],
        "backend-only semantics leaked: executable lowering contradicts authored semantic claims"
    );
    assert_eq!(status_unit["escape_hatch_gate"]["status"], "closed");
    assert_semantic_review(
        &status_unit["semantic_review"],
        "backend_only_semantics_leaked",
        &["method_body_missing_cap_behavior"],
        "executable lowering contradicts authored semantic claims",
    );

    let export_output = run_spec(&fixture_dst, &["export", wedge_root.to_str().unwrap()]);
    assert_success(&export_output, "semantic drift wedge export");
    let export_json: Value = serde_json::from_slice(&export_output.stdout).unwrap();
    let exported = exported_passport(&export_json, "pricing/discount_policy");
    assert_eq!(exported["escape_hatch_gate"]["status"], "closed");
    assert_semantic_review(
        &exported["semantic_review"],
        "backend_only_semantics_leaked",
        &["method_body_missing_cap_behavior"],
        "executable lowering contradicts authored semantic claims",
    );
}

#[test]
fn under_specified_wedge_projects_incomplete_health_consistently() {
    let (_temp_dir, fixture_dst) = copied_ecommerce_fixture();
    let wedge_root = semantic_review_fixture_root(&fixture_dst, "under_specified");
    let unit_path = wedge_root.join("units/pricing/discount_policy.unit.spec");
    let molecule_path = write_semantic_review_molecule(
        &wedge_root,
        "pricing/discount_policy_semantic_review_under_specified",
        "Close the vague-authorship wedge so status reflects semantic under-specification rather than missing proof.",
        r#"    {
        let subtotal = Decimal::new(1500, 2);
        let capped = crate::pricing::discount_policy::DiscountPolicy::FixedAmount {
            amount: Decimal::new(2000, 2),
        };

        assert_eq!(capped.discount_amount(subtotal), subtotal);
        assert_eq!(capped.discounted_subtotal(subtotal), Decimal::ZERO);
    }"#,
    );
    let passport_path = wedge_root.join("units/pricing/discount_policy.spec.passport.json");

    let unit_test_output = run_spec(
        &fixture_dst,
        &[
            "test",
            unit_path.to_str().unwrap(),
            "--crate-root",
            fixture_dst.to_str().unwrap(),
        ],
    );
    assert_success(&unit_test_output, "under-specified wedge unit test");

    let molecule_test_output = run_spec(
        &fixture_dst,
        &[
            "test",
            molecule_path.to_str().unwrap(),
            "--crate-root",
            fixture_dst.to_str().unwrap(),
        ],
    );
    assert_success(&molecule_test_output, "under-specified wedge molecule test");

    let passport = read_json(&passport_path);
    assert_eq!(passport["escape_hatch_gate"]["status"], "closed");
    assert_semantic_review(
        &passport["semantic_review"],
        "under_specified",
        &["vague_unit_intent", "vague_method_intent"],
        "authored semantic surfaces are too weak for honest evaluation",
    );

    let status_output = run_spec(
        &fixture_dst,
        &["status", wedge_root.to_str().unwrap(), "--format", "json"],
    );
    assert_exit_code(&status_output, 1, "under-specified wedge status");
    let status_json: Value = serde_json::from_slice(&status_output.stdout).unwrap();
    let status_unit = status_unit(&status_json, "pricing/discount_policy");
    assert_eq!(status_unit["status"], "incomplete");
    assert_eq!(
        status_unit["reason"],
        "semantic under-specified: authored semantic surfaces are too weak for honest evaluation"
    );
    assert_eq!(status_unit["escape_hatch_gate"]["status"], "closed");
    assert_semantic_review(
        &status_unit["semantic_review"],
        "under_specified",
        &["vague_unit_intent", "vague_method_intent"],
        "authored semantic surfaces are too weak for honest evaluation",
    );

    let export_output = run_spec(&fixture_dst, &["export", wedge_root.to_str().unwrap()]);
    assert_success(&export_output, "under-specified wedge export");
    let export_json: Value = serde_json::from_slice(&export_output.stdout).unwrap();
    let exported = exported_passport(&export_json, "pricing/discount_policy");
    assert_eq!(exported["escape_hatch_gate"]["status"], "closed");
    assert_semantic_review(
        &exported["semantic_review"],
        "under_specified",
        &["vague_unit_intent", "vague_method_intent"],
        "authored semantic surfaces are too weak for honest evaluation",
    );
}
