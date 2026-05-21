use serde_json::{Value, json};
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
        let file_name = entry.file_name();
        if file_name
            .to_str()
            .is_some_and(|name| matches!(name, "target" | ".git"))
        {
            continue;
        }
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

fn run_git(cwd: &Path, args: &[&str]) -> Output {
    Command::new("git")
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

fn parse_stdout_json(output: &Output) -> Value {
    serde_json::from_slice(&output.stdout).unwrap()
}

fn init_git_repo(cwd: &Path) {
    assert_success(&run_git(cwd, &["init", "-b", "main"]), "git init");
    assert_success(
        &run_git(cwd, &["config", "user.email", "spec-tests@example.com"]),
        "git config user.email",
    );
    assert_success(
        &run_git(cwd, &["config", "user.name", "Spec Tests"]),
        "git config user.name",
    );
    assert_success(&run_git(cwd, &["add", "."]), "git add");
    assert_success(
        &run_git(cwd, &["commit", "-m", "test fixture"]),
        "git commit",
    );
}

const NORMALIZED_CONTRACT_VALUE: &str = "<normalized>";

fn read_contract_fixture(name: &str) -> Value {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/benchmarks")
        .join(name);
    serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap()
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

fn benchmark<'a>(json: &'a Value, benchmark_id: &str) -> &'a Value {
    json["benchmarks"]
        .as_array()
        .unwrap()
        .iter()
        .find(|benchmark| benchmark["benchmark_id"] == benchmark_id)
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

fn copy_service_benchmark_repo() -> (TempDir, PathBuf) {
    let temp_dir = TempDir::new().unwrap();
    let repo_dir = temp_dir.path().join("service-benchmark-repo");
    let examples_dir = repo_dir.join("examples");

    fs::create_dir_all(&examples_dir).unwrap();
    copy_dir_all(
        &repo_root().join("benchmarks"),
        &repo_dir.join("benchmarks"),
    );
    copy_dir_all(
        &repo_root().join("examples/ecommerce"),
        &examples_dir.join("ecommerce"),
    );
    copy_dir_all(
        &repo_root().join("examples/crosslib-app"),
        &examples_dir.join("crosslib-app"),
    );
    copy_dir_all(
        &repo_root().join("examples/shared-crate"),
        &examples_dir.join("shared-crate"),
    );
    copy_dir_all(
        &repo_root().join("examples/shared-spec"),
        &examples_dir.join("shared-spec"),
    );
    copy_dir_all(
        &repo_root().join("examples/service"),
        &examples_dir.join("service"),
    );
    init_git_repo(&repo_dir);

    (temp_dir, repo_dir)
}

fn write_service_readability_review(
    repo_dir: &Path,
    _projection: &Value,
    projection_digest: &str,
    generated_files: &[Value],
) {
    let review_dir = repo_dir.join("benchmarks/reviews");
    fs::create_dir_all(&review_dir).unwrap();
    fs::write(
        review_dir.join("BENCH-SERVICE.readability.review.json"),
        serde_json::to_vec_pretty(&json!({
            "benchmark_id": "BENCH-SERVICE",
            "projection_digest": projection_digest,
            "readability_generated_files": generated_files,
            "verdict": {
                "decision": "approved",
                "summary": "Generated service benchmark surfaces remain readable and structurally traceable."
            }
        }))
        .unwrap(),
    )
    .unwrap();
}

#[test]
fn rust_v1_service_status_contract_matches_frozen_fixture() {
    let (_temp_dir, repo_dir) = copy_service_benchmark_repo();

    let output = run_spec(
        &repo_dir,
        &["status", "examples/service/units", "--format", "json"],
    );
    assert_success(&output, "service benchmark-root status");
    let json = parse_stdout_json(&output);
    let benchmark = benchmark(&json, "BENCH-SERVICE");

    assert_eq!(benchmark["path_scope"], "full");
    assert_eq!(benchmark["benchmark_status"], "passing");
    assert_eq!(benchmark["gate_status"], "satisfied");
    assert_eq!(benchmark["summary"]["required_molecule_total"], 3);
    assert_eq!(benchmark["summary"]["positive_credit_cases"], 6);
    assert!(
        benchmark["cases"]
            .as_array()
            .unwrap()
            .iter()
            .all(|case| case["counts_as_supported_positive"] == Value::Bool(true))
    );

    assert_contract_matches_fixture(
        normalize_status_contract_json(json),
        "status-service-full.json",
    );
}

#[test]
fn rust_v1_service_export_contract_matches_frozen_fixture() {
    let (_temp_dir, repo_dir) = copy_service_benchmark_repo();

    let output = run_spec(&repo_dir, &["export", "examples/service/units"]);
    assert_success(&output, "service benchmark-root export");
    let json = parse_stdout_json(&output);
    let benchmark = benchmark(&json, "BENCH-SERVICE");

    assert_eq!(benchmark["path_scope"], "full");
    assert_eq!(benchmark["benchmark_status"], "passing");
    assert_eq!(benchmark["summary"]["required_molecule_total"], 3);
    assert_eq!(benchmark["summary"]["positive_credit_cases"], 6);
    assert!(
        benchmark["cases"]
            .as_array()
            .unwrap()
            .iter()
            .all(|case| case["counts_as_supported_positive"] == Value::Bool(true))
    );

    assert_contract_matches_fixture(
        normalize_export_contract_json(json),
        "export-service-full.json",
    );
}

#[test]
fn rust_v1_service_namespace_status_contract_matches_frozen_fixture() {
    let (_temp_dir, repo_dir) = copy_service_benchmark_repo();

    let output = run_spec(
        &repo_dir,
        &[
            "status",
            "examples/service/units/billing",
            "--format",
            "json",
        ],
    );
    assert_exit_code(&output, 1, "service namespace status should stay non-green");
    let json = parse_stdout_json(&output);
    let benchmark = benchmark(&json, "BENCH-SERVICE");

    assert_eq!(json["loader_errors"][0]["code"], "SPEC_NO_LIBRARY_ROOTS");
    assert_eq!(benchmark["path_scope"], "partial");
    assert_eq!(benchmark["accounting_status"], "partial_valid");
    assert!(
        benchmark["cases"]
            .as_array()
            .unwrap()
            .iter()
            .all(|case| case["counts_as_supported_positive"] == Value::Bool(false))
    );
    assert!(benchmark.get("benchmark_status").is_none());
    assert!(benchmark.get("projection_digest").is_none());
    assert!(benchmark.get("summary").is_none());

    assert_contract_matches_fixture(
        normalize_status_contract_json(json),
        "status-service-billing-partial-full.json",
    );
}

#[test]
fn rust_v1_service_namespace_export_contract_matches_frozen_fixture() {
    let (_temp_dir, repo_dir) = copy_service_benchmark_repo();

    let output = run_spec(&repo_dir, &["export", "examples/service/units/billing"]);
    assert_success(&output, "service namespace export");
    let json = parse_stdout_json(&output);
    let benchmark = benchmark(&json, "BENCH-SERVICE");

    assert_eq!(benchmark["path_scope"], "partial");
    assert_eq!(benchmark["accounting_status"], "partial_valid");
    assert!(
        benchmark["cases"]
            .as_array()
            .unwrap()
            .iter()
            .all(|case| case["counts_as_supported_positive"] == Value::Bool(false))
    );
    assert!(benchmark.get("benchmark_status").is_none());
    assert!(benchmark.get("projection_digest").is_none());
    assert!(benchmark.get("summary").is_none());

    assert_contract_matches_fixture(
        normalize_export_contract_json(json),
        "export-service-billing-partial-full.json",
    );
}

#[test]
fn rust_v1_service_repo_root_inventory_contract_matches_updated_fixture() {
    let (_temp_dir, repo_dir) = copy_service_benchmark_repo();

    let output = run_spec(&repo_dir, &["status", ".", "--format", "json"]);
    assert_exit_code(
        &output,
        1,
        "repo-root inventory should stay non-green when broad inventory includes untested work",
    );
    let json = parse_stdout_json(&output);
    let service = benchmark(&json, "BENCH-SERVICE");
    let ecom = benchmark(&json, "BENCH-ECOM");
    let crosslib = benchmark(&json, "BENCH-CROSSLIB");

    assert_eq!(json["scope_authority"], "inventory_only");
    assert_eq!(service["benchmark_status"], "passing");
    assert_eq!(service["summary"]["positive_credit_cases"], 6);
    assert_eq!(ecom["benchmark_status"], "passing");
    assert_eq!(ecom["readability_review_status"], "current");
    assert_eq!(crosslib["benchmark_status"], "passing");
    assert_eq!(crosslib["summary"]["positive_credit_cases"], 0);

    assert_contract_matches_fixture(
        normalize_status_contract_json(json),
        "status-repo-root-service-full.json",
    );
}

#[test]
fn rust_v1_service_is_non_passing_when_required_molecule_proof_is_missing() {
    let (_temp_dir, repo_dir) = copy_service_benchmark_repo();
    fs::remove_file(
        repo_dir.join("examples/service/units/billing/checkout_success_flow.test.evidence.json"),
    )
    .unwrap();

    let status_output = run_spec(
        &repo_dir,
        &["status", "examples/service/units", "--format", "json"],
    );
    assert_exit_code(
        &status_output,
        1,
        "service status with missing required molecule proof",
    );
    let status_json = parse_stdout_json(&status_output);
    let status_benchmark = benchmark(&status_json, "BENCH-SERVICE");
    assert_eq!(status_benchmark["benchmark_status"], "incomplete");
    assert_eq!(status_benchmark["gate_status"], "open");
    assert_eq!(
        required_molecule_proof(status_benchmark, "billing/checkout_success_flow")["status"],
        "untested"
    );

    let export_output = run_spec(&repo_dir, &["export", "examples/service/units"]);
    assert_success(
        &export_output,
        "service export with missing required molecule proof",
    );
    let export_json = parse_stdout_json(&export_output);
    let export_benchmark = benchmark(&export_json, "BENCH-SERVICE");
    assert_eq!(export_benchmark["benchmark_status"], "incomplete");
    assert_eq!(
        required_molecule_proof(export_benchmark, "billing/checkout_success_flow")["status"],
        "untested"
    );
}

#[test]
fn rust_v1_service_is_non_passing_when_required_molecule_proof_is_stale() {
    let (_temp_dir, repo_dir) = copy_service_benchmark_repo();

    let molecule_path =
        repo_dir.join("examples/service/units/billing/checkout_success_flow.test.spec");
    let source = fs::read_to_string(&molecule_path).unwrap();
    fs::write(
        &molecule_path,
        source.replace(
            "End-to-end service checkout pricing for the approved membership discount and regional fee path.",
            "End-to-end service checkout pricing for the approved membership discount and regional fee path after a fixture-only authored revision.",
        ),
    )
    .unwrap();

    let status_output = run_spec(
        &repo_dir,
        &["status", "examples/service/units", "--format", "json"],
    );
    assert_exit_code(
        &status_output,
        1,
        "service status with stale required molecule proof",
    );
    let status_json = parse_stdout_json(&status_output);
    let status_benchmark = benchmark(&status_json, "BENCH-SERVICE");
    assert_eq!(status_benchmark["benchmark_status"], "incomplete");
    assert_eq!(status_benchmark["gate_status"], "open");
    assert_eq!(
        required_molecule_proof(status_benchmark, "billing/checkout_success_flow")["status"],
        "stale"
    );

    let export_output = run_spec(&repo_dir, &["export", "examples/service/units"]);
    assert_success(
        &export_output,
        "service export with stale required molecule proof",
    );
    let export_json = parse_stdout_json(&export_output);
    let export_benchmark = benchmark(&export_json, "BENCH-SERVICE");
    assert_eq!(export_benchmark["benchmark_status"], "incomplete");
    assert_eq!(
        required_molecule_proof(export_benchmark, "billing/checkout_success_flow")["status"],
        "stale"
    );
}

#[test]
fn rust_v1_service_is_non_passing_when_required_molecule_proof_is_failing() {
    let (_temp_dir, repo_dir) = copy_service_benchmark_repo();

    let molecule_path =
        repo_dir.join("examples/service/units/billing/checkout_success_flow.test.spec");
    let source = fs::read_to_string(&molecule_path).unwrap();
    fs::write(
        &molecule_path,
        source.replace(
            "        assert!(quote.total() > quote.discounted_subtotal());\n",
            "        assert_eq!(quote.total(), Decimal::ZERO);\n",
        ),
    )
    .unwrap();

    let failing_test_output = run_spec(
        &repo_dir,
        &[
            "test",
            "examples/service/units/billing/checkout_success_flow.test.spec",
        ],
    );
    assert_exit_code(
        &failing_test_output,
        1,
        "failing service checkout_success_flow proof refresh",
    );

    let status_output = run_spec(
        &repo_dir,
        &["status", "examples/service/units", "--format", "json"],
    );
    assert_exit_code(
        &status_output,
        1,
        "service status with failing required molecule proof",
    );
    let status_json = parse_stdout_json(&status_output);
    let status_benchmark = benchmark(&status_json, "BENCH-SERVICE");
    assert_eq!(status_benchmark["benchmark_status"], "failing");
    assert_eq!(status_benchmark["gate_status"], "open");
    assert_eq!(
        required_molecule_proof(status_benchmark, "billing/checkout_success_flow")["status"],
        "failing"
    );

    let export_output = run_spec(&repo_dir, &["export", "examples/service/units"]);
    assert_success(
        &export_output,
        "service export with failing required molecule proof",
    );
    let export_json = parse_stdout_json(&export_output);
    let export_benchmark = benchmark(&export_json, "BENCH-SERVICE");
    assert_eq!(export_benchmark["benchmark_status"], "failing");
    assert_eq!(
        required_molecule_proof(export_benchmark, "billing/checkout_success_flow")["status"],
        "failing"
    );
}

#[test]
fn rust_v1_service_readability_review_becomes_stale_when_projection_digest_drifts() {
    let (_temp_dir, repo_dir) = copy_service_benchmark_repo();

    let baseline_output = run_spec(
        &repo_dir,
        &["status", "examples/service/units", "--format", "json"],
    );
    assert_success(&baseline_output, "service status before readability drift");
    let baseline_json = parse_stdout_json(&baseline_output);
    let baseline_benchmark = benchmark(&baseline_json, "BENCH-SERVICE");
    let generated_files = baseline_benchmark["readability_generated_files"]
        .as_array()
        .unwrap()
        .clone();

    write_service_readability_review(
        &repo_dir,
        baseline_benchmark,
        "sha256:not-the-current-projection",
        &generated_files,
    );

    let status_output = run_spec(
        &repo_dir,
        &["status", "examples/service/units", "--format", "json"],
    );
    assert_success(
        &status_output,
        "service status with stale readability review",
    );
    let status_json = parse_stdout_json(&status_output);
    let status_benchmark = benchmark(&status_json, "BENCH-SERVICE");
    assert_eq!(status_benchmark["benchmark_status"], "passing");
    assert_eq!(status_benchmark["readability_review_status"], "stale");

    let export_output = run_spec(&repo_dir, &["export", "examples/service/units"]);
    assert_success(
        &export_output,
        "service export with stale readability review",
    );
    let export_json = parse_stdout_json(&export_output);
    let export_benchmark = benchmark(&export_json, "BENCH-SERVICE");
    assert_eq!(export_benchmark["benchmark_status"], "passing");
    assert_eq!(export_benchmark["readability_review_status"], "stale");
}
