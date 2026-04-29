use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use tempfile::TempDir;

const FUNCTION_FAMILY_A_DOWN_COMPATIBILITY_KEY: &str =
    "function.arithmetic_leaf.monotone_down_nonnegative.v1";
const FUNCTION_FAMILY_A_UP_COMPATIBILITY_KEY: &str = "function.arithmetic_leaf.monotone_up.v1";
const FUNCTION_FAMILY_B_COMPATIBILITY_KEY: &str = "function.wrapper.pipeline.v1";
const FUNCTION_FAMILY_CHAIN3_COMPATIBILITY_KEY: &str = "function.wrapper.pipeline.chain3.v1";

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

fn copied_m19_semantic_falsification_pack() -> (TempDir, PathBuf) {
    let temp_dir = TempDir::new().unwrap();
    let fixture_dst = temp_dir.path().join("semantic_falsification_pack");
    copy_dir_all(
        &repo_root().join("spec-cli/tests/fixtures/m19/semantic_falsification_pack"),
        &fixture_dst,
    );
    (temp_dir, fixture_dst)
}

fn copied_m21_chain3_fixture(bucket: &str) -> (TempDir, PathBuf) {
    let temp_dir = TempDir::new().unwrap();
    let fixture_dst = temp_dir.path().join(format!("m21_chain3_{bucket}"));
    copy_dir_all(
        &repo_root()
            .join("semantic-families/function.wrapper.pipeline.chain3.v1/fixtures")
            .join(bucket),
        &fixture_dst,
    );
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

fn write_checkout_quote_semantic_review_molecule(
    fixture_root: &Path,
    id: &str,
    intent: &str,
    body: &str,
) -> PathBuf {
    let path = fixture_root.join("units/pricing/checkout_quote_semantic_review.test.spec");
    fs::write(
        &path,
        format!(
            "id: {id}\nspec_version: \"0.3.0\"\nintent:\n  why: {intent}\ncovers:\n  - pricing/checkout_quote\nimports:\n  - rust_decimal::Decimal\n  - crate::pricing::apply_discount::apply_discount\n  - crate::pricing::apply_tax::apply_tax\n  - crate::pricing::calculate_total::calculate_total\n  - crate::pricing::checkout_quote::CheckoutQuote\nbody:\n  rust: |\n{body}\n"
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

fn assert_checkout_quote_semantic_review(
    review: &Value,
    verdict: &str,
    reason_codes: &[&str],
    summary: &str,
) {
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
    assert_eq!(review["evaluator_scope"], "supported_data_surface");
    assert_eq!(review["compatibility_key"], "data.checkout_quote.v1");
}

fn assert_function_semantic_review(
    review: &Value,
    compatibility_key: &str,
    verdict: &str,
    reason_codes: &[&str],
    summary: &str,
) {
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
    assert_eq!(review["evaluator_scope"], "supported_function_surface");
    assert_eq!(review["compatibility_key"], compatibility_key);
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

fn assert_unsupported_function_reason(review: &Value, reason: &str) {
    assert_unsupported_function_semantic_review(review);
    assert_eq!(review["unsupported_reason_codes"][0], reason, "{review}");
}

fn replace_in_file(path: &Path, from: &str, to: &str) {
    let source = fs::read_to_string(path).unwrap();
    assert!(
        source.contains(from),
        "expected {from:?} in {}",
        path.display()
    );
    fs::write(path, source.replacen(from, to, 1)).unwrap();
}

fn rewrite_apply_discount_as_drift(unit_path: &Path) {
    replace_in_file(
        unit_path,
        "    {\n        let discounted = subtotal - subtotal * rate;\n        round(discounted.max(Decimal::ZERO))\n    }\n",
        "    {\n        round(subtotal + subtotal * rate)\n    }\n",
    );
    replace_in_file(unit_path, "Decimal::new(9000, 2)", "Decimal::new(11000, 2)");
}

fn rewrite_apply_discount_as_under_specified(unit_path: &Path) {
    replace_in_file(
        unit_path,
        "Apply a discount to a subtotal while keeping the result nonnegative.",
        "todo",
    );
}

fn rewrite_apply_discount_as_clamp_drift(unit_path: &Path) {
    replace_in_file(
        unit_path,
        "round(discounted.max(Decimal::ZERO))",
        "round(discounted)",
    );
}

fn rewrite_apply_discount_as_unsupported_near_miss(unit_path: &Path) {
    replace_in_file(
        unit_path,
        "    {\n        let discounted = subtotal - subtotal * rate;\n        round(discounted.max(Decimal::ZERO))\n    }\n",
        "    {\n        let discounted = subtotal - subtotal * rate;\n        if discounted < Decimal::ZERO {\n            Decimal::ZERO\n        } else {\n            round(discounted)\n        }\n    }\n",
    );
}

fn rewrite_apply_tax_as_drift(unit_path: &Path) {
    replace_in_file(
        unit_path,
        "    {\n        let taxed = subtotal + subtotal * rate;\n        round(taxed)\n    }\n",
        "    {\n        round((subtotal - subtotal * rate).max(Decimal::ZERO))\n    }\n",
    );
    replace_in_file(unit_path, "Decimal::new(10725, 2)", "Decimal::new(9275, 2)");
}

fn rewrite_apply_tax_as_under_specified(unit_path: &Path) {
    replace_in_file(
        unit_path,
        "Add sales tax to a subtotal using a rate expressed as a decimal fraction.",
        "todo",
    );
}

fn rewrite_apply_tax_as_clamp_drift(unit_path: &Path) {
    replace_in_file(unit_path, "round(taxed)", "round(taxed.max(Decimal::ZERO))");
}

#[allow(dead_code)]
fn rewrite_apply_tax_as_unsupported_near_miss(unit_path: &Path) {
    replace_in_file(
        unit_path,
        "    {\n        let taxed = subtotal + subtotal * rate;\n        round(taxed)\n    }\n",
        "    {\n        let taxed = subtotal + subtotal * rate;\n        if taxed < subtotal {\n            subtotal\n        } else {\n            round(taxed)\n        }\n    }\n",
    );
}

fn rewrite_calculate_total_as_reversed_pipeline(unit_path: &Path) {
    replace_in_file(
        unit_path,
        "    {\n        let discounted = apply_discount(subtotal, discount_rate);\n        apply_tax(discounted, tax_rate)\n    }\n",
        "    {\n        let taxed_first = apply_tax(subtotal, tax_rate);\n        apply_discount(taxed_first, discount_rate)\n    }\n",
    );
}

fn rewrite_calculate_total_as_under_specified(unit_path: &Path) {
    replace_in_file(
        unit_path,
        "Combine discount and tax so a checkout flow can produce the final price.",
        "todo",
    );
}

fn rewrite_calculate_total_as_unsupported_near_miss(unit_path: &Path) {
    replace_in_file(
        unit_path,
        "    {\n        let discounted = apply_discount(subtotal, discount_rate);\n        apply_tax(discounted, tax_rate)\n    }\n",
        "    {\n        apply_tax(apply_discount(subtotal, discount_rate), tax_rate.max(Decimal::ZERO))\n    }\n",
    );
}

struct SupportedFunctionWedgeExpectation<'a> {
    unit_id: &'a str,
    compatibility_key: &'a str,
    verdict: &'a str,
    reason_codes: &'a [&'a str],
    summary: &'a str,
    expected_status: &'a str,
    expected_reason: Option<&'a str>,
}

fn run_supported_function_wedge_assertions(
    fixture_dst: &Path,
    passport_path: &Path,
    expectation: SupportedFunctionWedgeExpectation<'_>,
) {
    let passport = read_json(passport_path);
    assert_function_semantic_review(
        &passport["semantic_review"],
        expectation.compatibility_key,
        expectation.verdict,
        expectation.reason_codes,
        expectation.summary,
    );

    let status_output = run_spec(
        fixture_dst,
        &["status", fixture_dst.to_str().unwrap(), "--format", "json"],
    );
    match expectation.expected_status {
        "valid" => assert_success(&status_output, "supported function wedge status"),
        _ => assert_exit_code(&status_output, 1, "supported function wedge status"),
    }
    let status_json: Value = serde_json::from_slice(&status_output.stdout).unwrap();
    let status_unit = status_unit(&status_json, expectation.unit_id);
    assert_eq!(status_unit["status"], expectation.expected_status);
    match expectation.expected_reason {
        Some(reason) => assert_eq!(status_unit["reason"], reason),
        None => assert!(status_unit["reason"].is_null()),
    }
    assert_function_semantic_review(
        &status_unit["semantic_review"],
        expectation.compatibility_key,
        expectation.verdict,
        expectation.reason_codes,
        expectation.summary,
    );

    let export_output = run_spec(fixture_dst, &["export", fixture_dst.to_str().unwrap()]);
    assert_success(&export_output, "supported function wedge export");
    let export_json: Value = serde_json::from_slice(&export_output.stdout).unwrap();
    let exported = exported_passport(&export_json, expectation.unit_id);
    assert_function_semantic_review(
        &exported["semantic_review"],
        expectation.compatibility_key,
        expectation.verdict,
        expectation.reason_codes,
        expectation.summary,
    );
}

fn aligned_checkout_quote_molecule_body() -> &'static str {
    r#"    {
        let quote = CheckoutQuote::new(
            Decimal::new(10000, 2),
            Decimal::new(10, 2),
            Decimal::new(725, 4),
        );
        let total =
            calculate_total(Decimal::new(10000, 2), Decimal::new(10, 2), Decimal::new(725, 4));
        let rounding_sensitive_quote = CheckoutQuote::new(
            Decimal::new(1001, 2),
            Decimal::new(3333, 4),
            Decimal::new(725, 4),
        );
        let rounding_sensitive_total = calculate_total(
            Decimal::new(1001, 2),
            Decimal::new(3333, 4),
            Decimal::new(725, 4),
        );

        assert_eq!(
            quote.discounted_subtotal(),
            apply_discount(Decimal::new(10000, 2), Decimal::new(10, 2))
        );
        assert_eq!(quote.total(), total);
        assert_eq!(
            rounding_sensitive_quote.discounted_subtotal(),
            apply_discount(Decimal::new(1001, 2), Decimal::new(3333, 4))
        );
        assert_eq!(rounding_sensitive_quote.total(), rounding_sensitive_total);
    }"#
}

fn contradictory_checkout_quote_molecule_body() -> &'static str {
    r#"    {
        let quote = CheckoutQuote::new(
            Decimal::new(10000, 2),
            Decimal::new(10, 2),
            Decimal::new(725, 4),
        );
        let rounding_sensitive_quote = CheckoutQuote::new(
            Decimal::new(1001, 2),
            Decimal::new(3333, 4),
            Decimal::new(725, 4),
        );

        assert_eq!(
            quote.discounted_subtotal(),
            apply_discount(Decimal::new(10000, 2), Decimal::new(10, 2))
        );
        assert_eq!(
            quote.total(),
            apply_tax(Decimal::new(10000, 2), Decimal::new(725, 4))
        );
        assert_eq!(
            rounding_sensitive_quote.discounted_subtotal(),
            apply_discount(Decimal::new(1001, 2), Decimal::new(3333, 4))
        );
        assert_eq!(
            rounding_sensitive_quote.total(),
            apply_tax(Decimal::new(1001, 2), Decimal::new(725, 4))
        );
    }"#
}

fn remove_discount_policy_noise(fixture_root: &Path) {
    let _ = fs::remove_file(fixture_root.join("units/pricing/discount_policy.unit.spec"));
    let _ =
        fs::remove_file(fixture_root.join("units/pricing/discount_policy_checkout_flow.test.spec"));
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
        "backend_only_meaning_preserved",
        &["backend_only_execution_marker", "proof_helper_only_marker"],
        "backend-only execution markers are present without changing authored meaning",
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
        "backend_only_meaning_preserved",
        &["backend_only_execution_marker", "proof_helper_only_marker"],
        "backend-only execution markers are present without changing authored meaning",
    );

    let export_output = run_spec(&fixture_dst, &["export", wedge_root.to_str().unwrap()]);
    assert_success(&export_output, "aligned wedge export");
    let export_json: Value = serde_json::from_slice(&export_output.stdout).unwrap();
    let exported = exported_passport(&export_json, "pricing/discount_policy");
    assert_eq!(exported["escape_hatch_gate"]["status"], "closed");
    assert_semantic_review(
        &exported["semantic_review"],
        "backend_only_meaning_preserved",
        &["backend_only_execution_marker", "proof_helper_only_marker"],
        "backend-only execution markers are present without changing authored meaning",
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

#[test]
fn bool_domain_predicate_wedge_projects_under_specified_instead_of_false_green() {
    let (_temp_dir, fixture_dst) = copied_ecommerce_fixture();
    let wedge_root =
        semantic_review_fixture_root(&fixture_dst, "false_green_bool_domain_predicate");
    let unit_path = wedge_root.join("units/pricing/discount_policy.unit.spec");
    let molecule_path = write_semantic_review_molecule(
        &wedge_root,
        "pricing/discount_policy_semantic_review_false_green_bool_domain_predicate",
        "Close the bool-domain-predicate wedge so semantic review surfaces the extra authored method instead of treating it like proof glue.",
        r#"    {
        let subtotal = Decimal::new(1500, 2);
        let capped = crate::pricing::discount_policy::DiscountPolicy::FixedAmount {
            amount: Decimal::new(2000, 2),
        };

        assert_eq!(capped.discount_amount(subtotal), subtotal);
        assert_eq!(capped.discounted_subtotal(subtotal), Decimal::ZERO);
        assert!(capped.has_cap());
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
    assert_success(&unit_test_output, "bool-domain-predicate wedge unit test");

    let molecule_test_output = run_spec(
        &fixture_dst,
        &[
            "test",
            molecule_path.to_str().unwrap(),
            "--crate-root",
            fixture_dst.to_str().unwrap(),
        ],
    );
    assert_success(
        &molecule_test_output,
        "bool-domain-predicate wedge molecule test",
    );

    let passport = read_json(&passport_path);
    assert_eq!(passport["escape_hatch_gate"]["status"], "closed");
    assert_semantic_review(
        &passport["semantic_review"],
        "under_specified",
        &["outside_honest_supported_subset"],
        "authored semantic surfaces are too weak for honest evaluation",
    );

    let status_output = run_spec(
        &fixture_dst,
        &["status", wedge_root.to_str().unwrap(), "--format", "json"],
    );
    assert_exit_code(&status_output, 1, "bool-domain-predicate wedge status");
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
        &["outside_honest_supported_subset"],
        "authored semantic surfaces are too weak for honest evaluation",
    );

    let export_output = run_spec(&fixture_dst, &["export", wedge_root.to_str().unwrap()]);
    assert_success(&export_output, "bool-domain-predicate wedge export");
    let export_json: Value = serde_json::from_slice(&export_output.stdout).unwrap();
    let exported = exported_passport(&export_json, "pricing/discount_policy");
    assert_eq!(exported["escape_hatch_gate"]["status"], "closed");
    assert_semantic_review(
        &exported["semantic_review"],
        "under_specified",
        &["outside_honest_supported_subset"],
        "authored semantic surfaces are too weak for honest evaluation",
    );
}

#[test]
fn canonical_checkout_quote_semantic_review_wedge_projects_aligned_state() {
    let (_temp_dir, fixture_dst) = copied_ecommerce_fixture();
    remove_discount_policy_noise(&fixture_dst);
    let unit_path = fixture_dst.join("units/pricing/checkout_quote.unit.spec");
    let molecule_path = write_checkout_quote_semantic_review_molecule(
        &fixture_dst,
        "pricing/checkout_quote_semantic_review_aligned",
        "Close the canonical aligned checkout-quote wedge by proving the authored data semantics through a molecule test.",
        aligned_checkout_quote_molecule_body(),
    );
    let passport_path = fixture_dst.join("units/pricing/checkout_quote.spec.passport.json");

    let unit_test_output = run_spec(
        &fixture_dst,
        &[
            "test",
            unit_path.to_str().unwrap(),
            "--crate-root",
            fixture_dst.to_str().unwrap(),
        ],
    );
    assert_success(&unit_test_output, "aligned checkout quote wedge unit test");

    let molecule_test_output = run_spec(
        &fixture_dst,
        &[
            "test",
            molecule_path.to_str().unwrap(),
            "--crate-root",
            fixture_dst.to_str().unwrap(),
        ],
    );
    assert_success(
        &molecule_test_output,
        "aligned checkout quote wedge molecule test",
    );

    let passport = read_json(&passport_path);
    assert_eq!(passport["escape_hatch_gate"]["status"], "closed");
    assert_checkout_quote_semantic_review(
        &passport["semantic_review"],
        "backend_only_meaning_preserved",
        &["backend_only_execution_marker"],
        "backend-only execution markers are present without changing authored meaning",
    );

    let status_output = run_spec(
        &fixture_dst,
        &["status", fixture_dst.to_str().unwrap(), "--format", "json"],
    );
    assert_success(&status_output, "aligned checkout quote wedge status");
    let status_json: Value = serde_json::from_slice(&status_output.stdout).unwrap();
    let status_unit = status_unit(&status_json, "pricing/checkout_quote");
    assert_eq!(status_unit["status"], "valid");
    assert!(status_unit["reason"].is_null());
    assert_eq!(status_unit["escape_hatch_gate"]["status"], "closed");
    assert_checkout_quote_semantic_review(
        &status_unit["semantic_review"],
        "backend_only_meaning_preserved",
        &["backend_only_execution_marker"],
        "backend-only execution markers are present without changing authored meaning",
    );

    let export_output = run_spec(&fixture_dst, &["export", fixture_dst.to_str().unwrap()]);
    assert_success(&export_output, "aligned checkout quote wedge export");
    let export_json: Value = serde_json::from_slice(&export_output.stdout).unwrap();
    let exported = exported_passport(&export_json, "pricing/checkout_quote");
    assert_eq!(exported["escape_hatch_gate"]["status"], "closed");
    assert_checkout_quote_semantic_review(
        &exported["semantic_review"],
        "backend_only_meaning_preserved",
        &["backend_only_execution_marker"],
        "backend-only execution markers are present without changing authored meaning",
    );
}

#[test]
fn contradictory_checkout_quote_wedge_projects_failing_state() {
    let (_temp_dir, fixture_dst) = copied_ecommerce_fixture();
    remove_discount_policy_noise(&fixture_dst);
    let unit_path = fixture_dst.join("units/pricing/checkout_quote.unit.spec");
    let molecule_path = write_checkout_quote_semantic_review_molecule(
        &fixture_dst,
        "pricing/checkout_quote_semantic_review_semantic_drift",
        "Close the contradictory checkout-quote wedge so semantic review is the only failing signal.",
        contradictory_checkout_quote_molecule_body(),
    );
    let passport_path = fixture_dst.join("units/pricing/checkout_quote.spec.passport.json");
    let source = fs::read_to_string(&unit_path).unwrap();
    fs::write(
        &unit_path,
        source
            .replace(
                "apply_tax(self.discounted_subtotal(), self.tax_rate)",
                "apply_tax(self.subtotal, self.tax_rate)",
            )
            .replace(
                "rust_decimal::Decimal::new(96525, 3)",
                "rust_decimal::Decimal::new(107250, 3)",
            ),
    )
    .unwrap();

    let unit_test_output = run_spec(
        &fixture_dst,
        &[
            "test",
            unit_path.to_str().unwrap(),
            "--crate-root",
            fixture_dst.to_str().unwrap(),
        ],
    );
    assert_success(&unit_test_output, "checkout quote drift wedge unit test");

    let molecule_test_output = run_spec(
        &fixture_dst,
        &[
            "test",
            molecule_path.to_str().unwrap(),
            "--crate-root",
            fixture_dst.to_str().unwrap(),
        ],
    );
    assert_success(
        &molecule_test_output,
        "checkout quote drift wedge molecule test",
    );

    let passport = read_json(&passport_path);
    assert_eq!(passport["escape_hatch_gate"]["status"], "closed");
    assert_checkout_quote_semantic_review(
        &passport["semantic_review"],
        "backend_only_semantics_leaked",
        &["method_body_missing_cap_behavior"],
        "executable lowering contradicts authored semantic claims",
    );

    let status_output = run_spec(
        &fixture_dst,
        &["status", fixture_dst.to_str().unwrap(), "--format", "json"],
    );
    assert_exit_code(&status_output, 1, "checkout quote drift wedge status");
    let status_json: Value = serde_json::from_slice(&status_output.stdout).unwrap();
    let status_unit = status_unit(&status_json, "pricing/checkout_quote");
    assert_eq!(status_unit["status"], "failing");
    assert_eq!(
        status_unit["reason"],
        "backend-only semantics leaked: executable lowering contradicts authored semantic claims"
    );
    assert_eq!(status_unit["escape_hatch_gate"]["status"], "closed");
    assert_checkout_quote_semantic_review(
        &status_unit["semantic_review"],
        "backend_only_semantics_leaked",
        &["method_body_missing_cap_behavior"],
        "executable lowering contradicts authored semantic claims",
    );

    let export_output = run_spec(&fixture_dst, &["export", fixture_dst.to_str().unwrap()]);
    assert_success(&export_output, "checkout quote drift wedge export");
    let export_json: Value = serde_json::from_slice(&export_output.stdout).unwrap();
    let exported = exported_passport(&export_json, "pricing/checkout_quote");
    assert_eq!(exported["escape_hatch_gate"]["status"], "closed");
    assert_checkout_quote_semantic_review(
        &exported["semantic_review"],
        "backend_only_semantics_leaked",
        &["method_body_missing_cap_behavior"],
        "executable lowering contradicts authored semantic claims",
    );
}

#[test]
fn under_specified_checkout_quote_wedge_projects_incomplete_state() {
    let (_temp_dir, fixture_dst) = copied_ecommerce_fixture();
    remove_discount_policy_noise(&fixture_dst);
    let unit_path = fixture_dst.join("units/pricing/checkout_quote.unit.spec");
    let molecule_path = write_checkout_quote_semantic_review_molecule(
        &fixture_dst,
        "pricing/checkout_quote_semantic_review_under_specified",
        "Close the vague checkout-quote wedge so semantic review is the only incomplete signal.",
        aligned_checkout_quote_molecule_body(),
    );
    let passport_path = fixture_dst.join("units/pricing/checkout_quote.spec.passport.json");
    let source = fs::read_to_string(&unit_path).unwrap();
    let source = source.replace(
        "Quote a checkout total from subtotal plus discount and tax rates.",
        "todo",
    );
    fs::write(
        &unit_path,
        source.replace(
            "Return the final checkout total after discount and tax.",
            "todo",
        ),
    )
    .unwrap();

    let unit_test_output = run_spec(
        &fixture_dst,
        &[
            "test",
            unit_path.to_str().unwrap(),
            "--crate-root",
            fixture_dst.to_str().unwrap(),
        ],
    );
    assert_success(
        &unit_test_output,
        "checkout quote under-specified wedge unit test",
    );

    let molecule_test_output = run_spec(
        &fixture_dst,
        &[
            "test",
            molecule_path.to_str().unwrap(),
            "--crate-root",
            fixture_dst.to_str().unwrap(),
        ],
    );
    assert_success(
        &molecule_test_output,
        "checkout quote under-specified wedge molecule test",
    );

    let passport = read_json(&passport_path);
    assert_eq!(passport["escape_hatch_gate"]["status"], "closed");
    assert_checkout_quote_semantic_review(
        &passport["semantic_review"],
        "under_specified",
        &["vague_unit_intent", "vague_method_intent"],
        "authored semantic surfaces are too weak for honest evaluation",
    );

    let status_output = run_spec(
        &fixture_dst,
        &["status", fixture_dst.to_str().unwrap(), "--format", "json"],
    );
    assert_exit_code(
        &status_output,
        1,
        "checkout quote under-specified wedge status",
    );
    let status_json: Value = serde_json::from_slice(&status_output.stdout).unwrap();
    let status_unit = status_unit(&status_json, "pricing/checkout_quote");
    assert_eq!(status_unit["status"], "incomplete");
    assert_eq!(
        status_unit["reason"],
        "semantic under-specified: authored semantic surfaces are too weak for honest evaluation"
    );
    assert_eq!(status_unit["escape_hatch_gate"]["status"], "closed");
    assert_checkout_quote_semantic_review(
        &status_unit["semantic_review"],
        "under_specified",
        &["vague_unit_intent", "vague_method_intent"],
        "authored semantic surfaces are too weak for honest evaluation",
    );

    let export_output = run_spec(&fixture_dst, &["export", fixture_dst.to_str().unwrap()]);
    assert_success(
        &export_output,
        "checkout quote under-specified wedge export",
    );
    let export_json: Value = serde_json::from_slice(&export_output.stdout).unwrap();
    let exported = exported_passport(&export_json, "pricing/checkout_quote");
    assert_eq!(exported["escape_hatch_gate"]["status"], "closed");
    assert_checkout_quote_semantic_review(
        &exported["semantic_review"],
        "under_specified",
        &["vague_unit_intent", "vague_method_intent"],
        "authored semantic surfaces are too weak for honest evaluation",
    );
}

#[test]
fn monotone_down_nonnegative_truth_surface_command_matrix_preserves_until_spec_test_refresh() {
    let (_temp_dir, fixture_dst) = copied_ecommerce_fixture();
    let unit_path = fixture_dst.join("units/pricing/apply_discount.unit.spec");
    let passport_path = fixture_dst.join("units/pricing/apply_discount.spec.passport.json");

    let test_output = run_spec(
        &fixture_dst,
        &[
            "test",
            unit_path.to_str().unwrap(),
            "--crate-root",
            fixture_dst.to_str().unwrap(),
        ],
    );
    assert_success(
        &test_output,
        "monotone-down-nonnegative aligned fixture test",
    );

    let seeded_review = read_json(&passport_path)["semantic_review"].clone();
    assert_function_semantic_review(
        &seeded_review,
        FUNCTION_FAMILY_A_DOWN_COMPATIBILITY_KEY,
        "aligned",
        &[],
        "authored semantics and executable lowering agree on the supported function surface",
    );

    let status_output = run_spec(
        &fixture_dst,
        &["status", fixture_dst.to_str().unwrap(), "--format", "json"],
    );
    assert_success(
        &status_output,
        "monotone-down-nonnegative aligned fixture status",
    );
    let status_json: Value = serde_json::from_slice(&status_output.stdout).unwrap();
    let status_unit = status_unit(&status_json, "pricing/apply_discount");
    assert_eq!(status_unit["status"], "valid", "{status_json}");
    assert_eq!(
        status_unit["semantic_review"], seeded_review,
        "{status_json}"
    );

    let export_output = run_spec(&fixture_dst, &["export", fixture_dst.to_str().unwrap()]);
    assert_success(
        &export_output,
        "monotone-down-nonnegative aligned fixture export",
    );
    let export_json: Value = serde_json::from_slice(&export_output.stdout).unwrap();
    let exported = exported_passport(&export_json, "pricing/apply_discount");
    assert_eq!(exported["semantic_review"], seeded_review, "{export_json}");

    let generate_output = run_spec(
        &fixture_dst,
        &["generate", "units", "--output", "src/generated"],
    );
    assert_success(
        &generate_output,
        "monotone-down-nonnegative generate should preserve review",
    );
    assert_eq!(read_json(&passport_path)["semantic_review"], seeded_review);

    let build_output = run_spec(
        &fixture_dst,
        &[
            "build",
            "units",
            "--output",
            "src/generated",
            "--crate-root",
            ".",
        ],
    );
    assert_success(
        &build_output,
        "monotone-down-nonnegative build should preserve review",
    );
    assert_eq!(read_json(&passport_path)["semantic_review"], seeded_review);

    let refresh_output = run_spec(
        &fixture_dst,
        &[
            "test",
            "units",
            "--output",
            "src/generated",
            "--crate-root",
            ".",
        ],
    );
    assert_success(&refresh_output, "monotone-down-nonnegative refresh test");
    let refreshed_review = read_json(&passport_path)["semantic_review"].clone();
    assert_eq!(refreshed_review, seeded_review);
    assert_eq!(
        refreshed_review["compatibility_key"],
        FUNCTION_FAMILY_A_DOWN_COMPATIBILITY_KEY
    );
    assert_eq!(
        refreshed_review["evaluator_scope"],
        "supported_function_surface"
    );
}

#[test]
fn monotone_down_nonnegative_truth_surface_stale_status_and_export_preserve_last_proven_review() {
    let (_temp_dir, fixture_dst) = copied_ecommerce_fixture();
    let unit_path = fixture_dst.join("units/pricing/apply_discount.unit.spec");
    let passport_path = fixture_dst.join("units/pricing/apply_discount.spec.passport.json");

    let test_output = run_spec(
        &fixture_dst,
        &[
            "test",
            unit_path.to_str().unwrap(),
            "--crate-root",
            fixture_dst.to_str().unwrap(),
        ],
    );
    assert_success(
        &test_output,
        "monotone-down-nonnegative aligned fixture test",
    );

    let seeded_review = read_json(&passport_path)["semantic_review"].clone();
    assert_function_semantic_review(
        &seeded_review,
        FUNCTION_FAMILY_A_DOWN_COMPATIBILITY_KEY,
        "aligned",
        &[],
        "authored semantics and executable lowering agree on the supported function surface",
    );

    replace_in_file(
        &unit_path,
        "Apply a discount to a subtotal while keeping the result nonnegative.",
        "Apply a discount to a subtotal while keeping the result nonnegative with revised authored truth.",
    );

    let status_output = run_spec(
        &fixture_dst,
        &["status", fixture_dst.to_str().unwrap(), "--format", "json"],
    );
    assert_exit_code(
        &status_output,
        1,
        "monotone-down-nonnegative stale status should exit non-zero",
    );
    let status_json: Value = serde_json::from_slice(&status_output.stdout).unwrap();
    let status_unit = status_unit(&status_json, "pricing/apply_discount");
    assert_eq!(status_unit["status"], "stale", "{status_json}");
    assert_eq!(
        status_unit["reason"],
        "authored truth changed since last test"
    );
    assert_eq!(
        status_unit["semantic_review"], seeded_review,
        "{status_json}"
    );

    let export_output = run_spec(&fixture_dst, &["export", fixture_dst.to_str().unwrap()]);
    assert_success(
        &export_output,
        "monotone-down-nonnegative stale export should preserve prior review",
    );
    let export_json: Value = serde_json::from_slice(&export_output.stdout).unwrap();
    let exported = exported_passport(&export_json, "pricing/apply_discount");
    assert_eq!(exported["freshness"]["authored_truth_status"], "stale");
    assert_eq!(exported["semantic_review"], seeded_review, "{export_json}");
}

#[test]
fn monotone_down_nonnegative_corpus_aligned_fixture_projects_valid_state() {
    let (_temp_dir, fixture_dst) = copied_ecommerce_fixture();
    let unit_path = fixture_dst.join("units/pricing/apply_discount.unit.spec");
    let passport_path = fixture_dst.join("units/pricing/apply_discount.spec.passport.json");

    let unit_test_output = run_spec(
        &fixture_dst,
        &[
            "test",
            unit_path.to_str().unwrap(),
            "--crate-root",
            fixture_dst.to_str().unwrap(),
        ],
    );
    assert_success(&unit_test_output, "aligned apply_discount wedge unit test");

    run_supported_function_wedge_assertions(
        &fixture_dst,
        &passport_path,
        SupportedFunctionWedgeExpectation {
            unit_id: "pricing/apply_discount",
            compatibility_key: FUNCTION_FAMILY_A_DOWN_COMPATIBILITY_KEY,
            verdict: "aligned",
            reason_codes: &[],
            summary: "authored semantics and executable lowering agree on the supported function surface",
            expected_status: "valid",
            expected_reason: None,
        },
    );
}

#[test]
fn monotone_down_nonnegative_corpus_drift_fixture_projects_failing_state() {
    let (_temp_dir, fixture_dst) = copied_ecommerce_fixture();
    let unit_path = fixture_dst.join("units/pricing/apply_discount.unit.spec");
    let passport_path = fixture_dst.join("units/pricing/apply_discount.spec.passport.json");
    rewrite_apply_discount_as_drift(&unit_path);

    let unit_test_output = run_spec(
        &fixture_dst,
        &[
            "test",
            unit_path.to_str().unwrap(),
            "--crate-root",
            fixture_dst.to_str().unwrap(),
        ],
    );
    assert_success(&unit_test_output, "drift apply_discount wedge unit test");

    run_supported_function_wedge_assertions(
        &fixture_dst,
        &passport_path,
        SupportedFunctionWedgeExpectation {
            unit_id: "pricing/apply_discount",
            compatibility_key: FUNCTION_FAMILY_A_DOWN_COMPATIBILITY_KEY,
            verdict: "semantic_drift",
            reason_codes: &["function_body_contradicts_semantic_intent"],
            summary: "executable lowering contradicts authored semantic claims",
            expected_status: "failing",
            expected_reason: Some(
                "semantic drift: executable lowering contradicts authored semantic claims",
            ),
        },
    );
}

#[test]
fn monotone_down_nonnegative_corpus_under_specified_fixture_projects_incomplete_state() {
    let (_temp_dir, fixture_dst) = copied_ecommerce_fixture();
    let unit_path = fixture_dst.join("units/pricing/apply_discount.unit.spec");
    let passport_path = fixture_dst.join("units/pricing/apply_discount.spec.passport.json");
    rewrite_apply_discount_as_under_specified(&unit_path);

    let unit_test_output = run_spec(
        &fixture_dst,
        &[
            "test",
            unit_path.to_str().unwrap(),
            "--crate-root",
            fixture_dst.to_str().unwrap(),
        ],
    );
    assert_success(
        &unit_test_output,
        "under-specified apply_discount wedge unit test",
    );

    run_supported_function_wedge_assertions(
        &fixture_dst,
        &passport_path,
        SupportedFunctionWedgeExpectation {
            unit_id: "pricing/apply_discount",
            compatibility_key: FUNCTION_FAMILY_A_DOWN_COMPATIBILITY_KEY,
            verdict: "under_specified",
            reason_codes: &["vague_unit_intent"],
            summary: "authored semantic surfaces are too weak for honest evaluation",
            expected_status: "incomplete",
            expected_reason: Some(
                "semantic under-specified: authored semantic surfaces are too weak for honest evaluation",
            ),
        },
    );
}

#[test]
fn clamp_drift_apply_discount_wedge_projects_failing_state() {
    let (_temp_dir, fixture_dst) = copied_ecommerce_fixture();
    let unit_path = fixture_dst.join("units/pricing/apply_discount.unit.spec");
    let passport_path = fixture_dst.join("units/pricing/apply_discount.spec.passport.json");
    rewrite_apply_discount_as_clamp_drift(&unit_path);

    let unit_test_output = run_spec(
        &fixture_dst,
        &[
            "test",
            unit_path.to_str().unwrap(),
            "--crate-root",
            fixture_dst.to_str().unwrap(),
        ],
    );
    assert_success(
        &unit_test_output,
        "clamp drift apply_discount wedge unit test",
    );

    run_supported_function_wedge_assertions(
        &fixture_dst,
        &passport_path,
        SupportedFunctionWedgeExpectation {
            unit_id: "pricing/apply_discount",
            compatibility_key: FUNCTION_FAMILY_A_DOWN_COMPATIBILITY_KEY,
            verdict: "semantic_drift",
            reason_codes: &["function_body_contradicts_semantic_intent"],
            summary: "executable lowering contradicts authored semantic claims",
            expected_status: "failing",
            expected_reason: Some(
                "semantic drift: executable lowering contradicts authored semantic claims",
            ),
        },
    );
}

#[test]
fn monotone_down_nonnegative_corpus_unsupported_near_miss_stays_additive_only_and_neutral() {
    let (_temp_dir, fixture_dst) = copied_ecommerce_fixture();
    let unit_path = fixture_dst.join("units/pricing/apply_discount.unit.spec");
    let passport_path = fixture_dst.join("units/pricing/apply_discount.spec.passport.json");
    rewrite_apply_discount_as_unsupported_near_miss(&unit_path);

    let unit_test_output = run_spec(
        &fixture_dst,
        &[
            "test",
            unit_path.to_str().unwrap(),
            "--crate-root",
            fixture_dst.to_str().unwrap(),
        ],
    );
    assert_success(
        &unit_test_output,
        "unsupported near-miss monotone-down-nonnegative fixture unit test",
    );

    let passport = read_json(&passport_path);
    let seeded_review = passport["semantic_review"].clone();
    assert_unsupported_function_reason(&seeded_review, "unsupported_control_flow");

    let status_output = run_spec(
        &fixture_dst,
        &["status", fixture_dst.to_str().unwrap(), "--format", "json"],
    );
    assert_exit_code(
        &status_output,
        1,
        "unsupported near-miss monotone-down-nonnegative fixture status",
    );
    let status_json: Value = serde_json::from_slice(&status_output.stdout).unwrap();
    let status_unit = status_unit(&status_json, "pricing/apply_discount");
    assert_eq!(status_unit["status"], "valid");
    assert!(status_unit["reason"].is_null());
    assert_eq!(status_unit["semantic_review"], seeded_review);

    let export_output = run_spec(&fixture_dst, &["export", fixture_dst.to_str().unwrap()]);
    assert_success(
        &export_output,
        "unsupported near-miss monotone-down-nonnegative fixture export",
    );
    let export_json: Value = serde_json::from_slice(&export_output.stdout).unwrap();
    let exported = exported_passport(&export_json, "pricing/apply_discount");
    assert_eq!(exported["semantic_review"], seeded_review);
}

#[test]
fn monotone_down_nonnegative_regression_read_side_surfaces_are_not_shadowed() {
    monotone_down_nonnegative_corpus_aligned_fixture_projects_valid_state();
}

#[test]
fn monotone_down_nonnegative_regression_unsupported_near_miss_stays_additive_only_and_neutral() {
    monotone_down_nonnegative_corpus_unsupported_near_miss_stays_additive_only_and_neutral();
}

#[test]
fn monotone_up_truth_surface_command_matrix_preserves_until_spec_test_refresh() {
    let (_temp_dir, fixture_dst) = copied_ecommerce_fixture();
    let unit_path = fixture_dst.join("units/pricing/apply_tax.unit.spec");
    let passport_path = fixture_dst.join("units/pricing/apply_tax.spec.passport.json");

    let test_output = run_spec(
        &fixture_dst,
        &[
            "test",
            unit_path.to_str().unwrap(),
            "--crate-root",
            fixture_dst.to_str().unwrap(),
        ],
    );
    assert_success(&test_output, "monotone-up aligned fixture test");

    let seeded_review = read_json(&passport_path)["semantic_review"].clone();
    assert_function_semantic_review(
        &seeded_review,
        FUNCTION_FAMILY_A_UP_COMPATIBILITY_KEY,
        "aligned",
        &[],
        "authored semantics and executable lowering agree on the supported function surface",
    );

    let status_output = run_spec(
        &fixture_dst,
        &["status", fixture_dst.to_str().unwrap(), "--format", "json"],
    );
    assert_success(&status_output, "monotone-up aligned fixture status");
    let status_json: Value = serde_json::from_slice(&status_output.stdout).unwrap();
    let status_unit = status_unit(&status_json, "pricing/apply_tax");
    assert_eq!(status_unit["status"], "valid", "{status_json}");
    assert_eq!(
        status_unit["semantic_review"], seeded_review,
        "{status_json}"
    );

    let export_output = run_spec(&fixture_dst, &["export", fixture_dst.to_str().unwrap()]);
    assert_success(&export_output, "monotone-up aligned fixture export");
    let export_json: Value = serde_json::from_slice(&export_output.stdout).unwrap();
    let exported = exported_passport(&export_json, "pricing/apply_tax");
    assert_eq!(exported["semantic_review"], seeded_review, "{export_json}");

    let generate_output = run_spec(
        &fixture_dst,
        &["generate", "units", "--output", "src/generated"],
    );
    assert_success(
        &generate_output,
        "monotone-up generate should preserve review",
    );
    assert_eq!(read_json(&passport_path)["semantic_review"], seeded_review);

    let build_output = run_spec(
        &fixture_dst,
        &[
            "build",
            "units",
            "--output",
            "src/generated",
            "--crate-root",
            ".",
        ],
    );
    assert_success(&build_output, "monotone-up build should preserve review");
    assert_eq!(read_json(&passport_path)["semantic_review"], seeded_review);

    let refresh_output = run_spec(
        &fixture_dst,
        &[
            "test",
            "units",
            "--output",
            "src/generated",
            "--crate-root",
            ".",
        ],
    );
    assert_success(&refresh_output, "monotone-up refresh test");
    let refreshed_review = read_json(&passport_path)["semantic_review"].clone();
    assert_eq!(refreshed_review, seeded_review);
    assert_eq!(
        refreshed_review["compatibility_key"],
        FUNCTION_FAMILY_A_UP_COMPATIBILITY_KEY
    );
    assert_eq!(
        refreshed_review["evaluator_scope"],
        "supported_function_surface"
    );
}

#[test]
fn monotone_up_truth_surface_stale_status_and_export_preserve_last_proven_review() {
    let (_temp_dir, fixture_dst) = copied_ecommerce_fixture();
    let unit_path = fixture_dst.join("units/pricing/apply_tax.unit.spec");
    let passport_path = fixture_dst.join("units/pricing/apply_tax.spec.passport.json");

    let test_output = run_spec(
        &fixture_dst,
        &[
            "test",
            unit_path.to_str().unwrap(),
            "--crate-root",
            fixture_dst.to_str().unwrap(),
        ],
    );
    assert_success(&test_output, "monotone-up aligned fixture test");

    let seeded_review = read_json(&passport_path)["semantic_review"].clone();
    assert_function_semantic_review(
        &seeded_review,
        FUNCTION_FAMILY_A_UP_COMPATIBILITY_KEY,
        "aligned",
        &[],
        "authored semantics and executable lowering agree on the supported function surface",
    );

    replace_in_file(
        &unit_path,
        "Add sales tax to a subtotal using a rate expressed as a decimal fraction.",
        "Add sales tax to a subtotal using a rate expressed as a decimal fraction with revised authored truth.",
    );

    let status_output = run_spec(
        &fixture_dst,
        &["status", fixture_dst.to_str().unwrap(), "--format", "json"],
    );
    assert_exit_code(
        &status_output,
        1,
        "monotone-up stale status should exit non-zero",
    );
    let status_json: Value = serde_json::from_slice(&status_output.stdout).unwrap();
    let status_unit = status_unit(&status_json, "pricing/apply_tax");
    assert_eq!(status_unit["status"], "stale", "{status_json}");
    assert_eq!(
        status_unit["reason"],
        "authored truth changed since last test"
    );
    assert_eq!(
        status_unit["semantic_review"], seeded_review,
        "{status_json}"
    );

    let export_output = run_spec(&fixture_dst, &["export", fixture_dst.to_str().unwrap()]);
    assert_success(
        &export_output,
        "monotone-up stale export should preserve prior review",
    );
    let export_json: Value = serde_json::from_slice(&export_output.stdout).unwrap();
    let exported = exported_passport(&export_json, "pricing/apply_tax");
    assert_eq!(exported["freshness"]["authored_truth_status"], "stale");
    assert_eq!(exported["semantic_review"], seeded_review, "{export_json}");
}

#[test]
fn canonical_apply_tax_semantic_review_wedge_projects_aligned_state() {
    let (_temp_dir, fixture_dst) = copied_ecommerce_fixture();
    let unit_path = fixture_dst.join("units/pricing/apply_tax.unit.spec");
    let passport_path = fixture_dst.join("units/pricing/apply_tax.spec.passport.json");

    let unit_test_output = run_spec(
        &fixture_dst,
        &[
            "test",
            unit_path.to_str().unwrap(),
            "--crate-root",
            fixture_dst.to_str().unwrap(),
        ],
    );
    assert_success(&unit_test_output, "aligned apply_tax wedge unit test");

    run_supported_function_wedge_assertions(
        &fixture_dst,
        &passport_path,
        SupportedFunctionWedgeExpectation {
            unit_id: "pricing/apply_tax",
            compatibility_key: FUNCTION_FAMILY_A_UP_COMPATIBILITY_KEY,
            verdict: "aligned",
            reason_codes: &[],
            summary: "authored semantics and executable lowering agree on the supported function surface",
            expected_status: "valid",
            expected_reason: None,
        },
    );
}

#[test]
fn drift_apply_tax_wedge_projects_failing_state() {
    let (_temp_dir, fixture_dst) = copied_ecommerce_fixture();
    let unit_path = fixture_dst.join("units/pricing/apply_tax.unit.spec");
    let passport_path = fixture_dst.join("units/pricing/apply_tax.spec.passport.json");
    rewrite_apply_tax_as_drift(&unit_path);

    let unit_test_output = run_spec(
        &fixture_dst,
        &[
            "test",
            unit_path.to_str().unwrap(),
            "--crate-root",
            fixture_dst.to_str().unwrap(),
        ],
    );
    assert_success(&unit_test_output, "drift apply_tax wedge unit test");

    run_supported_function_wedge_assertions(
        &fixture_dst,
        &passport_path,
        SupportedFunctionWedgeExpectation {
            unit_id: "pricing/apply_tax",
            compatibility_key: FUNCTION_FAMILY_A_UP_COMPATIBILITY_KEY,
            verdict: "semantic_drift",
            reason_codes: &["function_body_contradicts_semantic_intent"],
            summary: "executable lowering contradicts authored semantic claims",
            expected_status: "failing",
            expected_reason: Some(
                "semantic drift: executable lowering contradicts authored semantic claims",
            ),
        },
    );
}

#[test]
fn under_specified_apply_tax_wedge_projects_incomplete_state() {
    let (_temp_dir, fixture_dst) = copied_ecommerce_fixture();
    let unit_path = fixture_dst.join("units/pricing/apply_tax.unit.spec");
    let passport_path = fixture_dst.join("units/pricing/apply_tax.spec.passport.json");
    rewrite_apply_tax_as_under_specified(&unit_path);

    let unit_test_output = run_spec(
        &fixture_dst,
        &[
            "test",
            unit_path.to_str().unwrap(),
            "--crate-root",
            fixture_dst.to_str().unwrap(),
        ],
    );
    assert_success(
        &unit_test_output,
        "under-specified apply_tax wedge unit test",
    );

    run_supported_function_wedge_assertions(
        &fixture_dst,
        &passport_path,
        SupportedFunctionWedgeExpectation {
            unit_id: "pricing/apply_tax",
            compatibility_key: FUNCTION_FAMILY_A_UP_COMPATIBILITY_KEY,
            verdict: "under_specified",
            reason_codes: &["vague_unit_intent"],
            summary: "authored semantic surfaces are too weak for honest evaluation",
            expected_status: "incomplete",
            expected_reason: Some(
                "semantic under-specified: authored semantic surfaces are too weak for honest evaluation",
            ),
        },
    );
}

#[test]
fn clamp_drift_apply_tax_wedge_projects_failing_state() {
    let (_temp_dir, fixture_dst) = copied_ecommerce_fixture();
    let unit_path = fixture_dst.join("units/pricing/apply_tax.unit.spec");
    let passport_path = fixture_dst.join("units/pricing/apply_tax.spec.passport.json");
    rewrite_apply_tax_as_clamp_drift(&unit_path);

    let unit_test_output = run_spec(
        &fixture_dst,
        &[
            "test",
            unit_path.to_str().unwrap(),
            "--crate-root",
            fixture_dst.to_str().unwrap(),
        ],
    );
    assert_success(&unit_test_output, "clamp drift apply_tax wedge unit test");

    run_supported_function_wedge_assertions(
        &fixture_dst,
        &passport_path,
        SupportedFunctionWedgeExpectation {
            unit_id: "pricing/apply_tax",
            compatibility_key: FUNCTION_FAMILY_A_UP_COMPATIBILITY_KEY,
            verdict: "semantic_drift",
            reason_codes: &["function_body_contradicts_semantic_intent"],
            summary: "executable lowering contradicts authored semantic claims",
            expected_status: "failing",
            expected_reason: Some(
                "semantic drift: executable lowering contradicts authored semantic claims",
            ),
        },
    );
}

#[test]
fn m21_chain3_regression_family_b_read_side_surfaces_are_not_shadowed() {
    let (_temp_dir, fixture_dst) = copied_ecommerce_fixture();
    let unit_path = fixture_dst.join("units/pricing/calculate_total.unit.spec");
    let passport_path = fixture_dst.join("units/pricing/calculate_total.spec.passport.json");

    let unit_test_output = run_spec(
        &fixture_dst,
        &[
            "test",
            unit_path.to_str().unwrap(),
            "--crate-root",
            fixture_dst.to_str().unwrap(),
        ],
    );
    assert_success(&unit_test_output, "aligned calculate_total wedge unit test");

    run_supported_function_wedge_assertions(
        &fixture_dst,
        &passport_path,
        SupportedFunctionWedgeExpectation {
            unit_id: "pricing/calculate_total",
            compatibility_key: FUNCTION_FAMILY_B_COMPATIBILITY_KEY,
            verdict: "aligned",
            reason_codes: &[],
            summary: "authored semantics and executable lowering agree on the supported function surface",
            expected_status: "valid",
            expected_reason: None,
        },
    );
}

#[test]
fn reversed_pipeline_calculate_total_wedge_projects_failing_state() {
    let (_temp_dir, fixture_dst) = copied_ecommerce_fixture();
    let unit_path = fixture_dst.join("units/pricing/calculate_total.unit.spec");
    let passport_path = fixture_dst.join("units/pricing/calculate_total.spec.passport.json");
    rewrite_calculate_total_as_reversed_pipeline(&unit_path);

    let unit_test_output = run_spec(
        &fixture_dst,
        &[
            "test",
            unit_path.to_str().unwrap(),
            "--crate-root",
            fixture_dst.to_str().unwrap(),
        ],
    );
    assert_success(
        &unit_test_output,
        "reversed pipeline calculate_total wedge unit test",
    );

    run_supported_function_wedge_assertions(
        &fixture_dst,
        &passport_path,
        SupportedFunctionWedgeExpectation {
            unit_id: "pricing/calculate_total",
            compatibility_key: FUNCTION_FAMILY_B_COMPATIBILITY_KEY,
            verdict: "semantic_drift",
            reason_codes: &["function_body_contradicts_semantic_intent"],
            summary: "executable lowering contradicts authored semantic claims",
            expected_status: "failing",
            expected_reason: Some(
                "semantic drift: executable lowering contradicts authored semantic claims",
            ),
        },
    );
}

#[test]
fn under_specified_calculate_total_wedge_projects_incomplete_state() {
    let (_temp_dir, fixture_dst) = copied_ecommerce_fixture();
    let unit_path = fixture_dst.join("units/pricing/calculate_total.unit.spec");
    let passport_path = fixture_dst.join("units/pricing/calculate_total.spec.passport.json");
    rewrite_calculate_total_as_under_specified(&unit_path);

    let unit_test_output = run_spec(
        &fixture_dst,
        &[
            "test",
            unit_path.to_str().unwrap(),
            "--crate-root",
            fixture_dst.to_str().unwrap(),
        ],
    );
    assert_success(
        &unit_test_output,
        "under-specified calculate_total wedge unit test",
    );

    run_supported_function_wedge_assertions(
        &fixture_dst,
        &passport_path,
        SupportedFunctionWedgeExpectation {
            unit_id: "pricing/calculate_total",
            compatibility_key: FUNCTION_FAMILY_B_COMPATIBILITY_KEY,
            verdict: "under_specified",
            reason_codes: &["vague_unit_intent"],
            summary: "authored semantic surfaces are too weak for honest evaluation",
            expected_status: "incomplete",
            expected_reason: Some(
                "semantic under-specified: authored semantic surfaces are too weak for honest evaluation",
            ),
        },
    );
}

#[test]
fn unsupported_near_miss_calculate_total_wedge_stays_additive_only_and_neutral() {
    let (_temp_dir, fixture_dst) = copied_ecommerce_fixture();
    let unit_path = fixture_dst.join("units/pricing/calculate_total.unit.spec");
    let passport_path = fixture_dst.join("units/pricing/calculate_total.spec.passport.json");
    rewrite_calculate_total_as_unsupported_near_miss(&unit_path);

    let unit_test_output = run_spec(
        &fixture_dst,
        &[
            "test",
            unit_path.to_str().unwrap(),
            "--crate-root",
            fixture_dst.to_str().unwrap(),
        ],
    );
    assert_success(
        &unit_test_output,
        "unsupported near-miss calculate_total wedge unit test",
    );

    let passport = read_json(&passport_path);
    let seeded_review = passport["semantic_review"].clone();
    assert_unsupported_function_semantic_review(&seeded_review);

    let status_output = run_spec(
        &fixture_dst,
        &["status", fixture_dst.to_str().unwrap(), "--format", "json"],
    );
    assert_exit_code(
        &status_output,
        1,
        "unsupported near-miss calculate_total wedge status",
    );
    let status_json: Value = serde_json::from_slice(&status_output.stdout).unwrap();
    let status_unit = status_unit(&status_json, "pricing/calculate_total");
    assert_eq!(status_unit["status"], "valid");
    assert!(status_unit["reason"].is_null());
    assert_eq!(status_unit["semantic_review"], seeded_review);

    let export_output = run_spec(&fixture_dst, &["export", fixture_dst.to_str().unwrap()]);
    assert_success(
        &export_output,
        "unsupported near-miss calculate_total wedge export",
    );
    let export_json: Value = serde_json::from_slice(&export_output.stdout).unwrap();
    let exported = exported_passport(&export_json, "pricing/calculate_total");
    assert_eq!(exported["semantic_review"], seeded_review);
}

#[test]
fn m21_chain3_corpus_aligned_fixture_projects_valid_state() {
    let (_temp_dir, fixture_dst) = copied_m21_chain3_fixture("aligned");
    let unit_path = fixture_dst.join("units/pricing/checkout_chain3_aligned.unit.spec");
    let passport_path =
        fixture_dst.join("units/pricing/checkout_chain3_aligned.spec.passport.json");

    let unit_test_output = run_spec(
        &fixture_dst,
        &[
            "test",
            unit_path.to_str().unwrap(),
            "--crate-root",
            fixture_dst.to_str().unwrap(),
        ],
    );
    assert_success(&unit_test_output, "M21 chain3 aligned fixture unit test");

    let passport = read_json(&passport_path);
    assert_function_semantic_review(
        &passport["semantic_review"],
        FUNCTION_FAMILY_CHAIN3_COMPATIBILITY_KEY,
        "aligned",
        &[],
        "authored semantics and executable lowering agree on the supported function surface",
    );

    let status_output = run_spec(
        &fixture_dst,
        &["status", fixture_dst.to_str().unwrap(), "--format", "json"],
    );
    assert_exit_code(&status_output, 1, "M21 chain3 aligned fixture status");
    let status_json: Value = serde_json::from_slice(&status_output.stdout).unwrap();
    let status_unit = status_unit(&status_json, "pricing/checkout_chain3_aligned");
    assert_eq!(status_unit["status"], "valid");
    assert!(status_unit["reason"].is_null());
    assert_function_semantic_review(
        &status_unit["semantic_review"],
        FUNCTION_FAMILY_CHAIN3_COMPATIBILITY_KEY,
        "aligned",
        &[],
        "authored semantics and executable lowering agree on the supported function surface",
    );

    let export_output = run_spec(&fixture_dst, &["export", fixture_dst.to_str().unwrap()]);
    assert_success(&export_output, "M21 chain3 aligned fixture export");
    let export_json: Value = serde_json::from_slice(&export_output.stdout).unwrap();
    let exported = exported_passport(&export_json, "pricing/checkout_chain3_aligned");
    assert_function_semantic_review(
        &exported["semantic_review"],
        FUNCTION_FAMILY_CHAIN3_COMPATIBILITY_KEY,
        "aligned",
        &[],
        "authored semantics and executable lowering agree on the supported function surface",
    );
}

#[test]
fn m21_chain3_corpus_drift_fixture_projects_failing_state() {
    let (_temp_dir, fixture_dst) = copied_m21_chain3_fixture("drift");
    let unit_path = fixture_dst.join("units/pricing/checkout_chain3_drift.unit.spec");
    let passport_path = fixture_dst.join("units/pricing/checkout_chain3_drift.spec.passport.json");

    let unit_test_output = run_spec(
        &fixture_dst,
        &[
            "test",
            unit_path.to_str().unwrap(),
            "--crate-root",
            fixture_dst.to_str().unwrap(),
        ],
    );
    assert_success(&unit_test_output, "M21 chain3 drift fixture unit test");

    run_supported_function_wedge_assertions(
        &fixture_dst,
        &passport_path,
        SupportedFunctionWedgeExpectation {
            unit_id: "pricing/checkout_chain3_drift",
            compatibility_key: FUNCTION_FAMILY_CHAIN3_COMPATIBILITY_KEY,
            verdict: "semantic_drift",
            reason_codes: &["function_body_contradicts_semantic_intent"],
            summary: "executable lowering contradicts authored semantic claims",
            expected_status: "failing",
            expected_reason: Some(
                "semantic drift: executable lowering contradicts authored semantic claims",
            ),
        },
    );
}

#[test]
fn m21_chain3_corpus_under_specified_fixture_projects_incomplete_state() {
    let (_temp_dir, fixture_dst) = copied_m21_chain3_fixture("under_specified");
    let unit_path = fixture_dst.join("units/pricing/checkout_chain3_under_specified.unit.spec");
    let passport_path =
        fixture_dst.join("units/pricing/checkout_chain3_under_specified.spec.passport.json");

    let unit_test_output = run_spec(
        &fixture_dst,
        &[
            "test",
            unit_path.to_str().unwrap(),
            "--crate-root",
            fixture_dst.to_str().unwrap(),
        ],
    );
    assert_success(
        &unit_test_output,
        "M21 chain3 under-specified fixture unit test",
    );

    run_supported_function_wedge_assertions(
        &fixture_dst,
        &passport_path,
        SupportedFunctionWedgeExpectation {
            unit_id: "pricing/checkout_chain3_under_specified",
            compatibility_key: FUNCTION_FAMILY_CHAIN3_COMPATIBILITY_KEY,
            verdict: "under_specified",
            reason_codes: &["vague_unit_intent"],
            summary: "authored semantic surfaces are too weak for honest evaluation",
            expected_status: "incomplete",
            expected_reason: Some(
                "semantic under-specified: authored semantic surfaces are too weak for honest evaluation",
            ),
        },
    );
}

#[test]
fn m21_chain3_corpus_unsupported_near_miss_stays_additive_only_and_neutral() {
    let (_temp_dir, fixture_dst) = copied_m21_chain3_fixture("unsupported_near_miss");
    let unit_path =
        fixture_dst.join("units/pricing/checkout_chain3_unsupported_near_miss.unit.spec");
    let passport_path =
        fixture_dst.join("units/pricing/checkout_chain3_unsupported_near_miss.spec.passport.json");

    let unit_test_output = run_spec(
        &fixture_dst,
        &[
            "test",
            unit_path.to_str().unwrap(),
            "--crate-root",
            fixture_dst.to_str().unwrap(),
        ],
    );
    assert_success(
        &unit_test_output,
        "M21 chain3 unsupported near-miss fixture unit test",
    );

    let passport = read_json(&passport_path);
    let seeded_review = passport["semantic_review"].clone();
    assert_unsupported_function_reason(&seeded_review, "unsupported_wrapper_body_shape");

    let status_output = run_spec(
        &fixture_dst,
        &["status", fixture_dst.to_str().unwrap(), "--format", "json"],
    );
    assert_exit_code(
        &status_output,
        1,
        "M21 chain3 unsupported near-miss fixture status",
    );
    let status_json: Value = serde_json::from_slice(&status_output.stdout).unwrap();
    let status_unit = status_unit(
        &status_json,
        "pricing/checkout_chain3_unsupported_near_miss",
    );
    assert_eq!(status_unit["status"], "valid");
    assert!(status_unit["reason"].is_null());
    assert_eq!(status_unit["semantic_review"], seeded_review);

    let export_output = run_spec(&fixture_dst, &["export", fixture_dst.to_str().unwrap()]);
    assert_success(
        &export_output,
        "M21 chain3 unsupported near-miss fixture export",
    );
    let export_json: Value = serde_json::from_slice(&export_output.stdout).unwrap();
    let exported = exported_passport(
        &export_json,
        "pricing/checkout_chain3_unsupported_near_miss",
    );
    assert_eq!(exported["semantic_review"], seeded_review);
}

#[test]
fn m21_chain3_regression_unsupported_near_miss_stays_additive_only_and_neutral() {
    m21_chain3_corpus_unsupported_near_miss_stays_additive_only_and_neutral();
}

#[test]
fn m19_drift_apply_membership_discount_fixture_projects_failing_state() {
    let (_temp_dir, fixture_dst) = copied_m19_semantic_falsification_pack();
    let unit_path = fixture_dst.join("units/billing/apply_membership_discount_drift.unit.spec");
    let passport_path =
        fixture_dst.join("units/billing/apply_membership_discount_drift.spec.passport.json");

    let unit_test_output = run_spec(
        &fixture_dst,
        &[
            "test",
            unit_path.to_str().unwrap(),
            "--crate-root",
            fixture_dst.to_str().unwrap(),
        ],
    );
    assert_success(
        &unit_test_output,
        "M19 drift apply_membership_discount fixture unit test",
    );

    run_supported_function_wedge_assertions(
        &fixture_dst,
        &passport_path,
        SupportedFunctionWedgeExpectation {
            unit_id: "billing/apply_membership_discount_drift",
            compatibility_key: FUNCTION_FAMILY_A_DOWN_COMPATIBILITY_KEY,
            verdict: "semantic_drift",
            reason_codes: &["function_body_contradicts_semantic_intent"],
            summary: "executable lowering contradicts authored semantic claims",
            expected_status: "failing",
            expected_reason: Some(
                "semantic drift: executable lowering contradicts authored semantic claims",
            ),
        },
    );
}

#[test]
fn m19_under_specified_apply_membership_discount_fixture_projects_incomplete_state() {
    let (_temp_dir, fixture_dst) = copied_m19_semantic_falsification_pack();
    let unit_path =
        fixture_dst.join("units/billing/apply_membership_discount_under_specified.unit.spec");
    let passport_path = fixture_dst
        .join("units/billing/apply_membership_discount_under_specified.spec.passport.json");

    let unit_test_output = run_spec(
        &fixture_dst,
        &[
            "test",
            unit_path.to_str().unwrap(),
            "--crate-root",
            fixture_dst.to_str().unwrap(),
        ],
    );
    assert_success(
        &unit_test_output,
        "M19 under-specified apply_membership_discount fixture unit test",
    );

    run_supported_function_wedge_assertions(
        &fixture_dst,
        &passport_path,
        SupportedFunctionWedgeExpectation {
            unit_id: "billing/apply_membership_discount_under_specified",
            compatibility_key: FUNCTION_FAMILY_A_DOWN_COMPATIBILITY_KEY,
            verdict: "under_specified",
            reason_codes: &["vague_unit_intent"],
            summary: "authored semantic surfaces are too weak for honest evaluation",
            expected_status: "incomplete",
            expected_reason: Some(
                "semantic under-specified: authored semantic surfaces are too weak for honest evaluation",
            ),
        },
    );
}

#[test]
fn m19_drift_apply_regional_fee_fixture_projects_failing_state() {
    let (_temp_dir, fixture_dst) = copied_m19_semantic_falsification_pack();
    let unit_path = fixture_dst.join("units/billing/apply_regional_fee_drift.unit.spec");
    let passport_path =
        fixture_dst.join("units/billing/apply_regional_fee_drift.spec.passport.json");

    let unit_test_output = run_spec(
        &fixture_dst,
        &[
            "test",
            unit_path.to_str().unwrap(),
            "--crate-root",
            fixture_dst.to_str().unwrap(),
        ],
    );
    assert_success(
        &unit_test_output,
        "M19 drift apply_regional_fee fixture unit test",
    );

    run_supported_function_wedge_assertions(
        &fixture_dst,
        &passport_path,
        SupportedFunctionWedgeExpectation {
            unit_id: "billing/apply_regional_fee_drift",
            compatibility_key: FUNCTION_FAMILY_A_UP_COMPATIBILITY_KEY,
            verdict: "semantic_drift",
            reason_codes: &["function_body_contradicts_semantic_intent"],
            summary: "executable lowering contradicts authored semantic claims",
            expected_status: "failing",
            expected_reason: Some(
                "semantic drift: executable lowering contradicts authored semantic claims",
            ),
        },
    );
}

#[test]
fn m19_under_specified_apply_regional_fee_fixture_projects_incomplete_state() {
    let (_temp_dir, fixture_dst) = copied_m19_semantic_falsification_pack();
    let unit_path = fixture_dst.join("units/billing/apply_regional_fee_under_specified.unit.spec");
    let passport_path =
        fixture_dst.join("units/billing/apply_regional_fee_under_specified.spec.passport.json");

    let unit_test_output = run_spec(
        &fixture_dst,
        &[
            "test",
            unit_path.to_str().unwrap(),
            "--crate-root",
            fixture_dst.to_str().unwrap(),
        ],
    );
    assert_success(
        &unit_test_output,
        "M19 under-specified apply_regional_fee fixture unit test",
    );

    run_supported_function_wedge_assertions(
        &fixture_dst,
        &passport_path,
        SupportedFunctionWedgeExpectation {
            unit_id: "billing/apply_regional_fee_under_specified",
            compatibility_key: FUNCTION_FAMILY_A_UP_COMPATIBILITY_KEY,
            verdict: "under_specified",
            reason_codes: &["vague_unit_intent"],
            summary: "authored semantic surfaces are too weak for honest evaluation",
            expected_status: "incomplete",
            expected_reason: Some(
                "semantic under-specified: authored semantic surfaces are too weak for honest evaluation",
            ),
        },
    );
}

#[test]
fn m19_drift_checkout_net_total_fixture_projects_failing_state() {
    let (_temp_dir, fixture_dst) = copied_m19_semantic_falsification_pack();
    let unit_path = fixture_dst.join("units/billing/checkout_net_total_drift.unit.spec");
    let passport_path =
        fixture_dst.join("units/billing/checkout_net_total_drift.spec.passport.json");

    let unit_test_output = run_spec(
        &fixture_dst,
        &[
            "test",
            unit_path.to_str().unwrap(),
            "--crate-root",
            fixture_dst.to_str().unwrap(),
        ],
    );
    assert_success(
        &unit_test_output,
        "M19 drift checkout_net_total fixture unit test",
    );

    run_supported_function_wedge_assertions(
        &fixture_dst,
        &passport_path,
        SupportedFunctionWedgeExpectation {
            unit_id: "billing/checkout_net_total_drift",
            compatibility_key: FUNCTION_FAMILY_B_COMPATIBILITY_KEY,
            verdict: "semantic_drift",
            reason_codes: &["function_body_contradicts_semantic_intent"],
            summary: "executable lowering contradicts authored semantic claims",
            expected_status: "failing",
            expected_reason: Some(
                "semantic drift: executable lowering contradicts authored semantic claims",
            ),
        },
    );
}

#[test]
fn m19_under_specified_checkout_net_total_fixture_projects_incomplete_state() {
    let (_temp_dir, fixture_dst) = copied_m19_semantic_falsification_pack();
    let unit_path = fixture_dst.join("units/billing/checkout_net_total_under_specified.unit.spec");
    let passport_path =
        fixture_dst.join("units/billing/checkout_net_total_under_specified.spec.passport.json");

    let unit_test_output = run_spec(
        &fixture_dst,
        &[
            "test",
            unit_path.to_str().unwrap(),
            "--crate-root",
            fixture_dst.to_str().unwrap(),
        ],
    );
    assert_success(
        &unit_test_output,
        "M19 under-specified checkout_net_total fixture unit test",
    );

    run_supported_function_wedge_assertions(
        &fixture_dst,
        &passport_path,
        SupportedFunctionWedgeExpectation {
            unit_id: "billing/checkout_net_total_under_specified",
            compatibility_key: FUNCTION_FAMILY_B_COMPATIBILITY_KEY,
            verdict: "under_specified",
            reason_codes: &["outside_honest_supported_subset"],
            summary: "supported semantic bodies fall outside the honest evaluator subset",
            expected_status: "incomplete",
            expected_reason: Some(
                "semantic under-specified: supported semantic bodies fall outside the honest evaluator subset",
            ),
        },
    );
}

#[test]
fn checkout_quote_and_discount_plus_tax_still_compose_with_supported_function_pair() {
    let (_temp_dir, fixture_dst) = copied_ecommerce_fixture();
    let apply_discount_unit_path = fixture_dst.join("units/pricing/apply_discount.unit.spec");
    let apply_tax_unit_path = fixture_dst.join("units/pricing/apply_tax.unit.spec");
    let checkout_quote_unit_path = fixture_dst.join("units/pricing/checkout_quote.unit.spec");
    let discount_plus_tax_path = fixture_dst.join("units/pricing/discount_plus_tax.test.spec");
    let checkout_flow_path = fixture_dst.join("units/pricing/checkout_flow.test.spec");

    let apply_discount_output = run_spec(
        &fixture_dst,
        &[
            "test",
            apply_discount_unit_path.to_str().unwrap(),
            "--crate-root",
            fixture_dst.to_str().unwrap(),
        ],
    );
    assert_success(
        &apply_discount_output,
        "supported pair apply_discount composition unit test",
    );

    let apply_tax_output = run_spec(
        &fixture_dst,
        &[
            "test",
            apply_tax_unit_path.to_str().unwrap(),
            "--crate-root",
            fixture_dst.to_str().unwrap(),
        ],
    );
    assert_success(
        &apply_tax_output,
        "supported pair apply_tax composition unit test",
    );

    let checkout_quote_output = run_spec(
        &fixture_dst,
        &[
            "test",
            checkout_quote_unit_path.to_str().unwrap(),
            "--crate-root",
            fixture_dst.to_str().unwrap(),
        ],
    );
    assert_success(
        &checkout_quote_output,
        "supported pair checkout_quote composition unit test",
    );

    let discount_plus_tax_output = run_spec(
        &fixture_dst,
        &[
            "test",
            discount_plus_tax_path.to_str().unwrap(),
            "--crate-root",
            fixture_dst.to_str().unwrap(),
        ],
    );
    assert_success(
        &discount_plus_tax_output,
        "supported pair discount_plus_tax composition molecule test",
    );

    let checkout_flow_output = run_spec(
        &fixture_dst,
        &[
            "test",
            checkout_flow_path.to_str().unwrap(),
            "--crate-root",
            fixture_dst.to_str().unwrap(),
        ],
    );
    assert_success(
        &checkout_flow_output,
        "supported pair checkout_flow composition molecule test",
    );
}
