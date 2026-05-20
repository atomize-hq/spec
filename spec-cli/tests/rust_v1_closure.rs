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
// --- LANE C SECTION END ---
