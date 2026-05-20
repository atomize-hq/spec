#![allow(dead_code)]

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

fn read_json(path: &Path) -> Value {
    serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap()
}

fn benchmark<'a>(status_json: &'a Value, id: &str) -> &'a Value {
    status_json["benchmarks"]
        .as_array()
        .unwrap()
        .iter()
        .find(|benchmark| benchmark["id"] == id)
        .unwrap()
}

fn copied_closure_fixture(lane: &str, fixture: &str) -> (TempDir, PathBuf) {
    let temp_dir = TempDir::new().unwrap();
    let fixture_dst = temp_dir.path().join(format!("{lane}_{fixture}"));
    copy_dir_all(
        &repo_root()
            .join("spec-cli/tests/fixtures/benchmarks/rust_v1_closure")
            .join(lane)
            .join(fixture),
        &fixture_dst,
    );
    (temp_dir, fixture_dst)
}

// Parent-owned suite prelude/helpers end here.
// Workers may edit only their named lane sections below.

// --- LANE A SECTION START ---
// Lane A owns only this section.
fn copied_lane_a_benchmark_fixture() -> (TempDir, PathBuf) {
    let temp_dir = TempDir::new().unwrap();
    let fixture_dst = temp_dir.path().join("lane_a_bench_ecom");
    fs::create_dir_all(fixture_dst.join("benchmarks")).unwrap();
    fs::write(fixture_dst.join(".git"), "gitdir: .git/modules/lane_a_bench_ecom\n").unwrap();
    fs::copy(
        repo_root().join("benchmarks/labels.json"),
        fixture_dst.join("benchmarks/labels.json"),
    )
    .unwrap();
    copy_dir_all(
        &repo_root().join("examples/ecommerce"),
        &fixture_dst.join("examples/ecommerce"),
    );
    (temp_dir, fixture_dst)
}

fn refresh_lane_a_required_benchmark_proofs(fixture_dst: &Path) {
    for (path, context) in [
        (
            "examples/ecommerce/units/pricing/pricing_quote.unit.spec",
            "pricing_quote unit proof refresh",
        ),
        (
            "examples/ecommerce/units/pricing/discount_strategy.unit.spec",
            "discount_strategy unit proof refresh",
        ),
        (
            "examples/ecommerce/units/pricing/discount_strategy_checkout_flow.test.spec",
            "discount_strategy_checkout_flow molecule proof refresh",
        ),
    ] {
        let output = run_spec(fixture_dst, &["test", path]);
        assert_success(&output, context);
    }
}

fn benchmark_status_json(fixture_dst: &Path) -> (Output, Value) {
    let output = run_spec(
        fixture_dst,
        &["status", "examples/ecommerce/units", "--format", "json"],
    );
    let json = serde_json::from_slice(&output.stdout).unwrap();
    (output, json)
}

fn benchmark_export_json(fixture_dst: &Path) -> (Output, Value) {
    let output = run_spec(fixture_dst, &["export", "examples/ecommerce/units"]);
    let json = serde_json::from_slice(&output.stdout).unwrap();
    (output, json)
}

fn lane_a_benchmark<'a>(status_json: &'a Value, id: &str) -> &'a Value {
    status_json["benchmarks"]
        .as_array()
        .unwrap()
        .iter()
        .find(|benchmark| benchmark["benchmark_id"] == id)
        .unwrap()
}

fn required_molecule_proof<'a>(benchmark_json: &'a Value, molecule_id: &str) -> &'a Value {
    benchmark_json["required_molecule_proofs"]
        .as_array()
        .unwrap()
        .iter()
        .find(|proof| proof["molecule_id"] == molecule_id)
        .unwrap()
}

#[test]
fn rust_v1_closure_lane_a_bench_ecom_passes_when_discount_strategy_checkout_flow_is_required_and_fresh(
) {
    let (_temp_dir, fixture_dst) = copied_lane_a_benchmark_fixture();
    refresh_lane_a_required_benchmark_proofs(&fixture_dst);

    let (status_output, status_json) = benchmark_status_json(&fixture_dst);
    assert_success(&status_output, "status with required discount strategy checkout flow");
    let status_benchmark = lane_a_benchmark(&status_json, "BENCH-ECOM");
    assert_eq!(status_benchmark["benchmark_status"], "passing");
    assert_eq!(status_benchmark["gate_status"], "satisfied");
    assert_eq!(status_benchmark["summary"]["required_molecule_total"], 3);
    assert_eq!(
        required_molecule_proof(status_benchmark, "pricing/discount_strategy_checkout_flow")
            ["status"],
        "valid"
    );

    let (export_output, export_json) = benchmark_export_json(&fixture_dst);
    assert_success(&export_output, "export with required discount strategy checkout flow");
    let export_benchmark = lane_a_benchmark(&export_json, "BENCH-ECOM");
    assert_eq!(export_benchmark["benchmark_status"], "passing");
    assert_eq!(export_benchmark["summary"]["required_molecule_total"], 3);
    assert_eq!(
        required_molecule_proof(export_benchmark, "pricing/discount_strategy_checkout_flow")
            ["status"],
        "valid"
    );
}

#[test]
fn rust_v1_closure_lane_a_bench_ecom_is_non_passing_when_required_discount_strategy_checkout_flow_proof_is_missing(
) {
    let (_temp_dir, fixture_dst) = copied_lane_a_benchmark_fixture();
    refresh_lane_a_required_benchmark_proofs(&fixture_dst);
    fs::remove_file(
        fixture_dst.join(
            "examples/ecommerce/units/pricing/discount_strategy_checkout_flow.test.evidence.json",
        ),
    )
    .unwrap();

    let (status_output, status_json) = benchmark_status_json(&fixture_dst);
    assert_exit_code(
        &status_output,
        1,
        "status with missing required discount strategy checkout flow proof",
    );
    let status_benchmark = lane_a_benchmark(&status_json, "BENCH-ECOM");
    assert_eq!(status_benchmark["benchmark_status"], "incomplete");
    assert_eq!(status_benchmark["gate_status"], "open");
    assert_eq!(
        required_molecule_proof(status_benchmark, "pricing/discount_strategy_checkout_flow")
            ["status"],
        "untested"
    );

    let (export_output, export_json) = benchmark_export_json(&fixture_dst);
    assert_success(
        &export_output,
        "export with missing required discount strategy checkout flow proof",
    );
    let export_benchmark = lane_a_benchmark(&export_json, "BENCH-ECOM");
    assert_eq!(export_benchmark["benchmark_status"], "incomplete");
    assert_eq!(
        required_molecule_proof(export_benchmark, "pricing/discount_strategy_checkout_flow")
            ["status"],
        "untested"
    );
}

#[test]
fn rust_v1_closure_lane_a_bench_ecom_is_non_passing_when_required_discount_strategy_checkout_flow_proof_is_stale(
) {
    let (_temp_dir, fixture_dst) = copied_lane_a_benchmark_fixture();
    refresh_lane_a_required_benchmark_proofs(&fixture_dst);

    let molecule_path =
        fixture_dst.join("examples/ecommerce/units/pricing/discount_strategy_checkout_flow.test.spec");
    let source = fs::read_to_string(&molecule_path).unwrap();
    fs::write(
        &molecule_path,
        source.replace(
            "Prove that the sum seam stays aligned with the pricing quote and tax flow.",
            "Prove that the sum seam stays aligned with the pricing quote and tax flow after a fixture-only authored revision.",
        ),
    )
    .unwrap();

    let (status_output, status_json) = benchmark_status_json(&fixture_dst);
    assert_exit_code(
        &status_output,
        1,
        "status with stale required discount strategy checkout flow proof",
    );
    let status_benchmark = lane_a_benchmark(&status_json, "BENCH-ECOM");
    assert_eq!(status_benchmark["benchmark_status"], "incomplete");
    assert_eq!(status_benchmark["gate_status"], "open");
    assert_eq!(
        required_molecule_proof(status_benchmark, "pricing/discount_strategy_checkout_flow")
            ["status"],
        "stale"
    );

    let (export_output, export_json) = benchmark_export_json(&fixture_dst);
    assert_success(
        &export_output,
        "export with stale required discount strategy checkout flow proof",
    );
    let export_benchmark = lane_a_benchmark(&export_json, "BENCH-ECOM");
    assert_eq!(export_benchmark["benchmark_status"], "incomplete");
    assert_eq!(
        required_molecule_proof(export_benchmark, "pricing/discount_strategy_checkout_flow")
            ["status"],
        "stale"
    );
}

#[test]
fn rust_v1_closure_lane_a_bench_ecom_is_non_passing_when_required_discount_strategy_checkout_flow_proof_is_failing(
) {
    let (_temp_dir, fixture_dst) = copied_lane_a_benchmark_fixture();
    refresh_lane_a_required_benchmark_proofs(&fixture_dst);

    let molecule_path =
        fixture_dst.join("examples/ecommerce/units/pricing/discount_strategy_checkout_flow.test.spec");
    let source = fs::read_to_string(&molecule_path).unwrap();
    fs::write(
        &molecule_path,
        source.replace(
            "        assert!(fixed_taxed > fixed_discounted);\n",
            "        assert_eq!(fixed_taxed, Decimal::ZERO);\n",
        ),
    )
    .unwrap();

    let failing_test_output = run_spec(
        &fixture_dst,
        &["test", "examples/ecommerce/units/pricing/discount_strategy_checkout_flow.test.spec"],
    );
    assert_exit_code(
        &failing_test_output,
        1,
        "failing discount strategy checkout flow proof refresh",
    );

    let (status_output, status_json) = benchmark_status_json(&fixture_dst);
    assert_exit_code(
        &status_output,
        1,
        "status with failing required discount strategy checkout flow proof",
    );
    let status_benchmark = lane_a_benchmark(&status_json, "BENCH-ECOM");
    assert_eq!(status_benchmark["benchmark_status"], "failing");
    assert_eq!(status_benchmark["gate_status"], "open");
    assert_eq!(
        required_molecule_proof(status_benchmark, "pricing/discount_strategy_checkout_flow")
            ["status"],
        "failing"
    );

    let (export_output, export_json) = benchmark_export_json(&fixture_dst);
    assert_success(
        &export_output,
        "export with failing required discount strategy checkout flow proof",
    );
    let export_benchmark = lane_a_benchmark(&export_json, "BENCH-ECOM");
    assert_eq!(export_benchmark["benchmark_status"], "failing");
    assert_eq!(
        required_molecule_proof(export_benchmark, "pricing/discount_strategy_checkout_flow")
            ["status"],
        "failing"
    );
}
// --- LANE A SECTION END ---

// --- LANE B SECTION START ---
// Lane B owns only this section.
fn lane_b_fixture_root(fixture: &str) -> (TempDir, PathBuf) {
    copied_closure_fixture("lane_b", fixture)
}

fn lane_b_benchmark<'a>(json: &'a Value, benchmark_id: &str) -> &'a Value {
    json["benchmarks"]
        .as_array()
        .unwrap()
        .iter()
        .find(|benchmark| benchmark["benchmark_id"] == benchmark_id)
        .unwrap()
}

fn lane_b_case<'a>(benchmark: &'a Value, case_id: &str) -> &'a Value {
    benchmark["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|case| case["case_id"] == case_id)
        .unwrap()
}

#[test]
fn rust_v1_closure_lane_b_status_keeps_active_companion_negative_missing_current_proof_incomplete(
) {
    let (_temp_dir, fixture_root) = lane_b_fixture_root("companion_negative_missing_current_proof");

    let output = run_spec(&fixture_root, &["status", "units", "--format", "json"]);
    assert_exit_code(&output, 1, "lane b crosslib status should stay incomplete");

    let json: Value = serde_json::from_slice(&output.stdout).unwrap();
    let benchmark = lane_b_benchmark(&json, "BENCH-CROSSLIB");
    let cases = benchmark["cases"].as_array().unwrap();

    assert_eq!(benchmark["benchmark_status"], "incomplete");
    assert_eq!(benchmark["summary"]["positive_credit_cases"], 0);
    assert_eq!(cases.len(), 4);
    assert!(
        cases.iter()
            .all(|case| case["counts_as_supported_positive"] == Value::Bool(false))
    );
    assert_eq!(
        lane_b_case(benchmark, "pricing/calculate_total")["status"],
        "untested"
    );
    assert_eq!(
        lane_b_case(benchmark, "pricing/checkout_nested_chain3")["status"],
        "untested"
    );
}

#[test]
fn rust_v1_closure_lane_b_export_never_counts_companion_negative_cases_as_positive_credit() {
    let (_temp_dir, fixture_root) = lane_b_fixture_root("companion_negative_missing_current_proof");

    let output = run_spec(&fixture_root, &["export", "units"]);
    assert_success(&output, "lane b crosslib export should succeed");

    let json: Value = serde_json::from_slice(&output.stdout).unwrap();
    let benchmark = lane_b_benchmark(&json, "BENCH-CROSSLIB");
    let cases = benchmark["cases"].as_array().unwrap();

    assert_eq!(benchmark["benchmark_status"], "incomplete");
    assert_eq!(benchmark["summary"]["positive_credit_cases"], 0);
    assert!(
        cases.iter()
            .all(|case| case["counts_as_supported_positive"] == Value::Bool(false))
    );
    assert_eq!(
        lane_b_case(benchmark, "pricing/calculate_total")["status"],
        "untested"
    );
    assert_eq!(
        lane_b_case(benchmark, "pricing/checkout_nested_chain3")["status"],
        "untested"
    );
}
// --- LANE B SECTION END ---

// --- LANE C SECTION START ---
// Lane C owns only this section.
fn lane_c_copy_fixture_file(fixture_root: &Path, src_relative: &str, dst_relative: &str) {
    let src = repo_root().join(src_relative);
    let dst = fixture_root.join(dst_relative);
    if let Some(parent) = dst.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::copy(src, dst).unwrap();
}

fn lane_c_write_benchmark_labels(
    fixture_root: &Path,
    benchmark_id: &str,
    cases: Vec<serde_json::Value>,
) {
    let benchmarks_dir = fixture_root.join("benchmarks");
    fs::create_dir_all(&benchmarks_dir).unwrap();
    fs::write(
        benchmarks_dir.join("labels.json"),
        serde_json::to_vec_pretty(&serde_json::json!({
            "schema_version": 1,
            "benchmarks": [
                {
                    "id": benchmark_id,
                    "kind": "positive",
                    "lifecycle": "active",
                    "required_for_v1": true,
                    "root": "units",
                    "generated_root": "src/generated",
                    "readability_scope": "supported_closure",
                    "required_molecule_ids": [],
                    "cases": cases
                }
            ]
        }))
        .unwrap(),
    )
    .unwrap();
}

fn lane_c_case_label(carrier_id: &str, classification: &str) -> serde_json::Value {
    serde_json::json!({
        "case_id": carrier_id,
        "carrier_kind": "unit",
        "carrier_id": carrier_id,
        "classification": classification
    })
}

fn lane_c_monotone_down_boundary_fixture() -> (TempDir, PathBuf) {
    let temp_dir = TempDir::new().unwrap();
    let fixture_root = temp_dir.path().join("lane_c_monotone_down_boundary");
    fs::create_dir_all(&fixture_root).unwrap();

    lane_c_copy_fixture_file(
        &fixture_root,
        "semantic-families/function.arithmetic_leaf.monotone_down_nonnegative.v1/fixtures/unsupported_near_miss/Cargo.toml",
        "Cargo.toml",
    );
    lane_c_copy_fixture_file(
        &fixture_root,
        "semantic-families/function.arithmetic_leaf.monotone_down_nonnegative.v1/fixtures/unsupported_near_miss/src/main.rs",
        "src/main.rs",
    );
    lane_c_copy_fixture_file(
        &fixture_root,
        "semantic-families/function.arithmetic_leaf.monotone_down_nonnegative.v1/fixtures/unsupported_near_miss/units/money/round_unsupported_near_miss.unit.spec",
        "units/money/round_unsupported_near_miss.unit.spec",
    );
    lane_c_copy_fixture_file(
        &fixture_root,
        "semantic-families/function.arithmetic_leaf.monotone_down_nonnegative.v1/fixtures/unsupported_near_miss/units/pricing/apply_discount_control_flow_unsupported_near_miss.unit.spec",
        "units/pricing/apply_discount_control_flow_unsupported_near_miss.unit.spec",
    );
    lane_c_write_benchmark_labels(
        &fixture_root,
        "BENCH-LANE-C-MONOTONE-DOWN",
        vec![
            lane_c_case_label(
                "pricing/apply_discount_control_flow_unsupported_near_miss",
                "supported",
            ),
            lane_c_case_label("money/round", "deferred"),
        ],
    );

    (temp_dir, fixture_root)
}

fn lane_c_monotone_up_boundary_fixture() -> (TempDir, PathBuf) {
    let temp_dir = TempDir::new().unwrap();
    let fixture_root = temp_dir.path().join("lane_c_monotone_up_boundary");
    fs::create_dir_all(&fixture_root).unwrap();

    lane_c_copy_fixture_file(
        &fixture_root,
        "semantic-families/function.arithmetic_leaf.monotone_up.v1/fixtures/unsupported_near_miss/Cargo.toml",
        "Cargo.toml",
    );
    lane_c_copy_fixture_file(
        &fixture_root,
        "semantic-families/function.arithmetic_leaf.monotone_up.v1/fixtures/unsupported_near_miss/src/main.rs",
        "src/main.rs",
    );
    lane_c_copy_fixture_file(
        &fixture_root,
        "semantic-families/function.arithmetic_leaf.monotone_up.v1/fixtures/unsupported_near_miss/units/pricing/apply_tax_control_flow_unsupported_near_miss.unit.spec",
        "units/pricing/apply_tax_control_flow_unsupported_near_miss.unit.spec",
    );
    lane_c_write_benchmark_labels(
        &fixture_root,
        "BENCH-LANE-C-MONOTONE-UP",
        vec![lane_c_case_label(
            "pricing/apply_tax_control_flow_unsupported_near_miss",
            "supported",
        )],
    );

    (temp_dir, fixture_root)
}

fn lane_c_wrapper_pipeline_boundary_fixture() -> (TempDir, PathBuf) {
    let temp_dir = TempDir::new().unwrap();
    let fixture_root = temp_dir.path().join("lane_c_wrapper_pipeline_boundary");
    fs::create_dir_all(&fixture_root).unwrap();

    lane_c_copy_fixture_file(
        &fixture_root,
        "semantic-families/function.wrapper.pipeline.v1/fixtures/unsupported_near_miss/Cargo.toml",
        "Cargo.toml",
    );
    lane_c_copy_fixture_file(
        &fixture_root,
        "semantic-families/function.wrapper.pipeline.v1/fixtures/unsupported_near_miss/src/main.rs",
        "src/main.rs",
    );
    lane_c_copy_fixture_file(
        &fixture_root,
        "semantic-families/function.wrapper.pipeline.v1/fixtures/unsupported_near_miss/units/pricing/pricing_discount_leaf_unsupported_near_miss.unit.spec",
        "units/pricing/pricing_discount_leaf_unsupported_near_miss.unit.spec",
    );
    lane_c_copy_fixture_file(
        &fixture_root,
        "semantic-families/function.wrapper.pipeline.v1/fixtures/unsupported_near_miss/units/pricing/pricing_tax_leaf_unsupported_near_miss.unit.spec",
        "units/pricing/pricing_tax_leaf_unsupported_near_miss.unit.spec",
    );
    lane_c_copy_fixture_file(
        &fixture_root,
        "semantic-families/function.wrapper.pipeline.v1/fixtures/unsupported_near_miss/units/pricing/pricing_total_wrapper_unsupported_near_miss.unit.spec",
        "units/pricing/pricing_total_wrapper_unsupported_near_miss.unit.spec",
    );
    lane_c_write_benchmark_labels(
        &fixture_root,
        "BENCH-LANE-C-WRAPPER",
        vec![
            lane_c_case_label("pricing/pricing_discount_leaf_unsupported_near_miss", "deferred"),
            lane_c_case_label("pricing/pricing_tax_leaf_unsupported_near_miss", "deferred"),
            lane_c_case_label("pricing/pricing_total_wrapper_unsupported_near_miss", "supported"),
        ],
    );

    (temp_dir, fixture_root)
}

fn lane_c_assert_supported_boundary_rejection(
    status_json: &Value,
    benchmark_id: &str,
    supported_case_id: &str,
) {
    let benchmark = status_json["benchmarks"]
        .as_array()
        .unwrap()
        .iter()
        .find(|benchmark| benchmark["benchmark_id"] == benchmark_id)
        .unwrap();
    let cases = benchmark["cases"].as_array().unwrap();
    let case = cases
        .iter()
        .find(|case| case["carrier_id"] == supported_case_id)
        .unwrap();

    assert_eq!(benchmark["path_scope"], "full");
    assert_eq!(benchmark["accounting_status"], "valid");
    assert_eq!(benchmark["benchmark_status"], "passing");
    assert_eq!(benchmark["gate_status"], "open");
    assert_eq!(benchmark["summary"]["supported_cases"], 1);
    assert_eq!(benchmark["summary"]["supported_valid_cases"], 1);
    assert_eq!(benchmark["summary"]["positive_credit_cases"], 0);
    assert!(
        benchmark["summary"]["unlabeled_loaded_carrier_ids"].is_null()
            || benchmark["summary"]["unlabeled_loaded_carrier_ids"]
                .as_array()
                .is_some_and(|values| values.is_empty())
    );

    assert_eq!(case["classification"], "supported");
    assert_eq!(case["status"], "valid");
    assert_eq!(case["semantic_support_status"], "unsupported");
    assert_eq!(case["counts_as_supported_positive"], Value::Bool(false));
}

#[test]
fn rust_v1_closure_lane_c_monotone_down_supported_boundary_is_rejected_from_positive_credit() {
    let (_temp_dir, fixture_root) = lane_c_monotone_down_boundary_fixture();

    let test_output = run_spec(
        &fixture_root,
        &["test", "units", "--output", "src/generated", "--crate-root", "."],
    );
    assert_success(
        &test_output,
        "Lane C monotone-down unsupported near-miss fixture test",
    );

    let status_output = run_spec(&fixture_root, &["status", ".", "--format", "json"]);
    assert_success(
        &status_output,
        "Lane C monotone-down unsupported near-miss fixture status",
    );
    let status_json: Value = serde_json::from_slice(&status_output.stdout).unwrap();

    lane_c_assert_supported_boundary_rejection(
        &status_json,
        "BENCH-LANE-C-MONOTONE-DOWN",
        "pricing/apply_discount_control_flow_unsupported_near_miss",
    );
}

#[test]
fn rust_v1_closure_lane_c_monotone_up_supported_boundary_is_rejected_from_positive_credit() {
    let (_temp_dir, fixture_root) = lane_c_monotone_up_boundary_fixture();

    let test_output = run_spec(
        &fixture_root,
        &["test", "units", "--output", "src/generated", "--crate-root", "."],
    );
    assert_success(
        &test_output,
        "Lane C monotone-up unsupported near-miss fixture test",
    );

    let status_output = run_spec(&fixture_root, &["status", ".", "--format", "json"]);
    assert_success(
        &status_output,
        "Lane C monotone-up unsupported near-miss fixture status",
    );
    let status_json: Value = serde_json::from_slice(&status_output.stdout).unwrap();

    lane_c_assert_supported_boundary_rejection(
        &status_json,
        "BENCH-LANE-C-MONOTONE-UP",
        "pricing/apply_tax_control_flow_unsupported_near_miss",
    );
}

#[test]
fn rust_v1_closure_lane_c_wrapper_pipeline_supported_boundary_is_rejected_from_positive_credit() {
    let (_temp_dir, fixture_root) = lane_c_wrapper_pipeline_boundary_fixture();

    let test_output = run_spec(
        &fixture_root,
        &["test", "units", "--output", "src/generated", "--crate-root", "."],
    );
    assert_success(
        &test_output,
        "Lane C wrapper-pipeline unsupported near-miss fixture test",
    );

    let status_output = run_spec(&fixture_root, &["status", ".", "--format", "json"]);
    assert_success(
        &status_output,
        "Lane C wrapper-pipeline unsupported near-miss fixture status",
    );
    let status_json: Value = serde_json::from_slice(&status_output.stdout).unwrap();

    lane_c_assert_supported_boundary_rejection(
        &status_json,
        "BENCH-LANE-C-WRAPPER",
        "pricing/pricing_total_wrapper_unsupported_near_miss",
    );
}
// --- LANE C SECTION END ---
