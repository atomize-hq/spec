use crate::XtaskError;
use crate::family::paths::{FamilyId, PacketPaths};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use walkdir::WalkDir;

pub(crate) const REPORT_SCHEMA_VERSION: u64 = 1;
pub(crate) const MANIFEST_SCHEMA_VERSION: u64 = 1;
pub(crate) const PROVE_ARTIFACT_NAME: &str = "prove.latest.json";
pub(crate) const CERTIFY_ARTIFACT_NAME: &str = "certification.report.json";

pub(crate) const PROVE_SUITES: [SuiteDefinition; 3] = [
    SuiteDefinition {
        name: "spec-core:m21_chain3_classifier_",
        command: &[
            "cargo",
            "test",
            "-p",
            "spec-core",
            "m21_chain3_classifier_",
            "--",
            "--nocapture",
        ],
        source_rel_path: "spec-core/src/semantic_review.rs",
        required_prefix: "fn m21_chain3_classifier_",
    },
    SuiteDefinition {
        name: "spec-cli:m21_chain3_truth_surface_",
        command: &[
            "cargo",
            "test",
            "-p",
            "spec-cli",
            "--test",
            "cli",
            "m21_chain3_truth_surface_",
            "--",
            "--nocapture",
        ],
        source_rel_path: "spec-cli/tests/cli.rs",
        required_prefix: "fn m21_chain3_truth_surface_",
    },
    SuiteDefinition {
        name: "spec-cli:m21_chain3_corpus_",
        command: &[
            "cargo",
            "test",
            "-p",
            "spec-cli",
            "--test",
            "m14_regressions",
            "m21_chain3_corpus_",
            "--",
            "--nocapture",
        ],
        source_rel_path: "spec-cli/tests/m14_regressions.rs",
        required_prefix: "fn m21_chain3_corpus_",
    },
];

pub(crate) const CERTIFY_SUITES: [SuiteDefinition; 2] = [
    SuiteDefinition {
        name: "spec-core:m21_chain3_regression_",
        command: &[
            "cargo",
            "test",
            "-p",
            "spec-core",
            "m21_chain3_regression_",
            "--",
            "--nocapture",
        ],
        source_rel_path: "spec-core/src/semantic_review.rs",
        required_prefix: "fn m21_chain3_regression_",
    },
    SuiteDefinition {
        name: "spec-cli:m21_chain3_regression_",
        command: &[
            "cargo",
            "test",
            "-p",
            "spec-cli",
            "--test",
            "m14_regressions",
            "m21_chain3_regression_",
            "--",
            "--nocapture",
        ],
        source_rel_path: "spec-cli/tests/m14_regressions.rs",
        required_prefix: "fn m21_chain3_regression_",
    },
];

#[derive(Debug, Clone, Copy)]
pub(crate) struct SuiteDefinition {
    pub name: &'static str,
    pub command: &'static [&'static str],
    pub source_rel_path: &'static str,
    pub required_prefix: &'static str,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub(crate) enum PassFail {
    Pass,
    Fail,
}

impl PassFail {
    pub(crate) fn from_passed(passed: bool) -> Self {
        if passed { Self::Pass } else { Self::Fail }
    }

    pub(crate) fn is_pass(&self) -> bool {
        matches!(self, Self::Pass)
    }
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct GateStatus {
    pub status: PassFail,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct GateStatuses {
    pub gate_a: GateStatus,
    pub gate_b: GateStatus,
    pub gate_c: GateStatus,
    pub gate_d: GateStatus,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct SuiteReport {
    pub name: String,
    pub command: Vec<String>,
    pub exit_code: i32,
    pub status: PassFail,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct FixtureDigest {
    pub bucket: String,
    pub path: String,
    pub sha256: String,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct CertificationReport {
    pub schema_version: u64,
    pub family: String,
    pub manifest_schema_version: u64,
    pub git_commit_sha: String,
    pub rust_toolchain: String,
    pub generated_at: String,
    pub overall_status: PassFail,
    pub gates: GateStatuses,
    pub suites: Vec<SuiteReport>,
    pub fixture_digests: Vec<FixtureDigest>,
}

impl CertificationReport {
    pub(crate) fn new(
        family: &FamilyId,
        git_commit_sha: String,
        rust_toolchain: String,
        generated_at: String,
    ) -> Self {
        Self {
            schema_version: REPORT_SCHEMA_VERSION,
            family: family.as_str().to_string(),
            manifest_schema_version: MANIFEST_SCHEMA_VERSION,
            git_commit_sha,
            rust_toolchain,
            generated_at,
            overall_status: PassFail::Fail,
            gates: GateStatuses {
                gate_a: GateStatus {
                    status: PassFail::Fail,
                },
                gate_b: GateStatus {
                    status: PassFail::Fail,
                },
                gate_c: GateStatus {
                    status: PassFail::Fail,
                },
                gate_d: GateStatus {
                    status: PassFail::Fail,
                },
            },
            suites: Vec::new(),
            fixture_digests: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct CommandOutput {
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
}

pub(crate) trait CommandRunner {
    fn run(&self, cwd: &Path, command: &[String]) -> CommandOutput;
}

pub(crate) struct SystemRunner;

impl CommandRunner for SystemRunner {
    fn run(&self, cwd: &Path, command: &[String]) -> CommandOutput {
        let mut process = Command::new(&command[0]);
        process.current_dir(cwd);
        process.args(&command[1..]);
        match process.output() {
            Ok(output) => CommandOutput {
                exit_code: output.status.code().unwrap_or(1),
                stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
                stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
            },
            Err(error) => CommandOutput {
                exit_code: 1,
                stdout: String::new(),
                stderr: error.to_string(),
            },
        }
    }
}

pub(crate) fn build_report<R: CommandRunner>(
    workspace_root: &Path,
    family: &FamilyId,
    runner: &R,
) -> CertificationReport {
    let git_commit_sha = first_line_or(
        runner.run(
            workspace_root,
            &["git", "rev-parse", "HEAD"]
                .into_iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>(),
        ),
        "unknown".to_string(),
    );
    let rust_toolchain = first_line_or(
        runner.run(
            workspace_root,
            &["rustc", "--version"]
                .into_iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>(),
        ),
        "unknown".to_string(),
    );
    let generated_at = first_line_or(
        runner.run(
            workspace_root,
            &["date", "-u", "+%Y-%m-%dT%H:%M:%SZ"]
                .into_iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>(),
        ),
        "1970-01-01T00:00:00Z".to_string(),
    );

    CertificationReport::new(family, git_commit_sha, rust_toolchain, generated_at)
}

pub(crate) fn refresh_generated_at<R: CommandRunner>(
    workspace_root: &Path,
    report: &mut CertificationReport,
    runner: &R,
) {
    report.generated_at = first_line_or(
        runner.run(
            workspace_root,
            &["date", "-u", "+%Y-%m-%dT%H:%M:%SZ"]
                .into_iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>(),
        ),
        report.generated_at.clone(),
    );
}

pub(crate) fn set_gates(
    report: &mut CertificationReport,
    gate_a: bool,
    gate_b: bool,
    gate_c: bool,
    gate_d: bool,
) {
    report.gates = GateStatuses {
        gate_a: GateStatus {
            status: PassFail::from_passed(gate_a),
        },
        gate_b: GateStatus {
            status: PassFail::from_passed(gate_b),
        },
        gate_c: GateStatus {
            status: PassFail::from_passed(gate_c),
        },
        gate_d: GateStatus {
            status: PassFail::from_passed(gate_d),
        },
    };
}

pub(crate) fn set_overall(report: &mut CertificationReport, passed: bool) {
    report.overall_status = PassFail::from_passed(passed);
}

pub(crate) fn run_suite<R: CommandRunner>(
    workspace_root: &Path,
    runner: &R,
    suite: SuiteDefinition,
) -> SuiteReport {
    let command = suite
        .command
        .iter()
        .map(|part| (*part).to_string())
        .collect::<Vec<_>>();
    let output = runner.run(workspace_root, &command);
    let prefix_present = suite_prefix_present(workspace_root, suite);
    let passed = output.exit_code == 0 && prefix_present;
    let _stderr = output.stderr;
    let _stdout = output.stdout;

    SuiteReport {
        name: suite.name.to_string(),
        command,
        exit_code: output.exit_code,
        status: PassFail::from_passed(passed),
    }
}

pub(crate) fn write_report(path: &Path, report: &CertificationReport) -> Result<(), XtaskError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| {
            XtaskError::WriteFailure(format!(
                "failed to create artifact directory `{}`: {error}",
                parent.display()
            ))
        })?;
    }

    let contents = serde_json::to_string_pretty(report).map_err(|error| {
        XtaskError::WriteFailure(format!("failed to serialize report: {error}"))
    })?;
    fs::write(path, format!("{contents}\n")).map_err(|error| {
        XtaskError::WriteFailure(format!(
            "failed to write report `{}`: {error}",
            path.display()
        ))
    })
}

pub(crate) fn collect_fixture_digests(
    paths: &PacketPaths,
) -> Result<Vec<FixtureDigest>, XtaskError> {
    if !paths.fixtures.exists() {
        return Ok(Vec::new());
    }

    let mut digests = Vec::new();
    for entry in WalkDir::new(&paths.fixtures).follow_links(false) {
        let entry = entry.map_err(|error| {
            XtaskError::WriteFailure(format!(
                "failed to walk fixture tree `{}`: {error}",
                paths.fixtures.display()
            ))
        })?;
        if entry.file_type().is_dir() {
            continue;
        }
        if !entry.file_type().is_file() {
            continue;
        }

        let path = entry.into_path();
        let relative_to_packet = path.strip_prefix(&paths.root).map_err(|_| {
            XtaskError::WriteFailure(format!(
                "fixture path `{}` escaped packet root `{}`",
                path.display(),
                paths.root.display()
            ))
        })?;
        let relative_to_fixtures = path.strip_prefix(&paths.fixtures).map_err(|_| {
            XtaskError::WriteFailure(format!(
                "fixture path `{}` escaped fixtures root `{}`",
                path.display(),
                paths.fixtures.display()
            ))
        })?;
        let bucket = relative_to_fixtures
            .components()
            .next()
            .and_then(|component| component.as_os_str().to_str())
            .ok_or_else(|| {
                XtaskError::WriteFailure(format!(
                    "fixture path `{}` is missing a bucket component",
                    path.display()
                ))
            })?;

        let bytes = fs::read(&path).map_err(|error| {
            XtaskError::WriteFailure(format!(
                "failed to read fixture `{}`: {error}",
                path.display()
            ))
        })?;
        let mut hasher = Sha256::new();
        hasher.update(bytes);
        digests.push(FixtureDigest {
            bucket: bucket.to_string(),
            path: normalize_path(relative_to_packet),
            sha256: hex_string(&hasher.finalize()),
        });
    }

    digests.sort_by(|left, right| left.path.cmp(&right.path));
    Ok(digests)
}

pub(crate) fn prove_artifact_path(paths: &PacketPaths) -> PathBuf {
    paths.artifacts.join(PROVE_ARTIFACT_NAME)
}

pub(crate) fn certify_attempt_path(paths: &PacketPaths, generated_at: &str) -> PathBuf {
    paths.artifacts.join(format!(
        "attempt-{}.json",
        generated_at
            .chars()
            .filter(|ch| ch.is_ascii_alphanumeric())
            .collect::<String>()
    ))
}

pub(crate) fn certification_report_path(paths: &PacketPaths) -> PathBuf {
    paths.artifacts.join(CERTIFY_ARTIFACT_NAME)
}

pub(crate) fn failed_suite_names(suites: &[SuiteReport]) -> String {
    suites
        .iter()
        .filter(|suite| !suite.status.is_pass())
        .map(|suite| suite.name.as_str())
        .collect::<Vec<_>>()
        .join(", ")
}

fn suite_prefix_present(workspace_root: &Path, suite: SuiteDefinition) -> bool {
    let source_path = workspace_root.join(suite.source_rel_path);
    match fs::read_to_string(&source_path) {
        Ok(contents) => contents.contains(suite.required_prefix),
        Err(_) => false,
    }
}

fn first_line_or(output: CommandOutput, fallback: String) -> String {
    if output.exit_code != 0 {
        return fallback;
    }

    output
        .stdout
        .lines()
        .next()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(ToString::to_string)
        .unwrap_or(fallback)
}

fn normalize_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

fn hex_string(bytes: &[u8]) -> String {
    let mut encoded = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        encoded.push_str(&format!("{byte:02x}"));
    }
    encoded
}
