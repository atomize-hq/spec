use crate::XtaskError;
use crate::family::paths::{FamilyId, PacketPaths};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use walkdir::WalkDir;

pub(crate) const REPORT_SCHEMA_VERSION: u64 = 2;
pub(crate) const MANIFEST_SCHEMA_VERSION: u64 = 1;
pub(crate) const PROVE_ARTIFACT_NAME: &str = "prove.latest.json";
pub(crate) const CERTIFY_ARTIFACT_NAME: &str = "certification.report.json";

#[derive(Debug, Clone, Copy)]
pub(crate) struct SuiteDefinition {
    pub name: &'static str,
    pub command: &'static [&'static str],
    pub expected_tests: &'static [&'static str],
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

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub(crate) enum GateId {
    GateA,
    GateB,
    GateC,
    GateD,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ArtifactKind {
    ProveLatest,
    CertifyAttempt,
    Certification,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct SuiteReport {
    pub name: String,
    pub command: Vec<String>,
    pub exit_code: i32,
    pub status: PassFail,
    pub attested_tests: Vec<String>,
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
    pub artifact_kind: ArtifactKind,
    pub required_gates: Vec<GateId>,
    pub phase_status: PassFail,
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
            artifact_kind: ArtifactKind::ProveLatest,
            required_gates: ArtifactKind::ProveLatest.required_gates(),
            phase_status: PassFail::Fail,
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

    fn apply_runtime_invariants(&mut self) {
        self.schema_version = REPORT_SCHEMA_VERSION;
        self.required_gates = self.artifact_kind.required_gates();
        self.phase_status = PassFail::from_passed(
            self.required_gates
                .iter()
                .all(|gate| self.gates.status(*gate).is_pass()),
        );
        self.overall_status = PassFail::from_passed(
            [GateId::GateA, GateId::GateB, GateId::GateC, GateId::GateD]
                .into_iter()
                .all(|gate| self.gates.status(gate).is_pass()),
        );
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
    report.apply_runtime_invariants();
}

pub(crate) fn set_overall(report: &mut CertificationReport, _passed: bool) {
    report.apply_runtime_invariants();
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
    let attestation = attest_suite_output(suite, &output);

    SuiteReport {
        name: suite.name.to_string(),
        command,
        exit_code: output.exit_code,
        status: PassFail::from_passed(attestation.is_ok()),
        attested_tests: match attestation {
            Ok(attested_tests) | Err(AttestationFailure { attested_tests, .. }) => attested_tests,
        },
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

    let normalized = normalize_report_for_write(path, report)?;
    let contents = serde_json::to_string_pretty(&normalized).map_err(|error| {
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

impl GateStatuses {
    fn status(&self, gate: GateId) -> &PassFail {
        match gate {
            GateId::GateA => &self.gate_a.status,
            GateId::GateB => &self.gate_b.status,
            GateId::GateC => &self.gate_c.status,
            GateId::GateD => &self.gate_d.status,
        }
    }
}

impl ArtifactKind {
    fn required_gates(self) -> Vec<GateId> {
        match self {
            Self::ProveLatest => vec![GateId::GateA, GateId::GateB, GateId::GateC],
            Self::CertifyAttempt | Self::Certification => {
                vec![GateId::GateA, GateId::GateB, GateId::GateC, GateId::GateD]
            }
        }
    }
}

#[derive(Debug)]
struct AttestationFailure {
    attested_tests: Vec<String>,
}

fn normalize_report_for_write(
    path: &Path,
    report: &CertificationReport,
) -> Result<CertificationReport, XtaskError> {
    let artifact_kind = artifact_kind_for_path(path)?;
    let mut normalized = report.clone();
    normalized.artifact_kind = artifact_kind;
    normalized.required_gates = artifact_kind.required_gates();
    normalized.phase_status = PassFail::from_passed(
        normalized
            .required_gates
            .iter()
            .all(|gate| normalized.gates.status(*gate).is_pass()),
    );
    normalized.overall_status = PassFail::from_passed(
        [GateId::GateA, GateId::GateB, GateId::GateC, GateId::GateD]
            .into_iter()
            .all(|gate| normalized.gates.status(gate).is_pass()),
    );
    normalized.schema_version = REPORT_SCHEMA_VERSION;
    for suite in &mut normalized.suites {
        suite.attested_tests.sort();
    }
    Ok(normalized)
}

fn artifact_kind_for_path(path: &Path) -> Result<ArtifactKind, XtaskError> {
    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| {
            XtaskError::WriteFailure(format!(
                "failed to infer artifact kind from report path `{}`",
                path.display()
            ))
        })?;

    if file_name == PROVE_ARTIFACT_NAME {
        Ok(ArtifactKind::ProveLatest)
    } else if file_name == CERTIFY_ARTIFACT_NAME {
        Ok(ArtifactKind::Certification)
    } else if file_name.starts_with("attempt-") && file_name.ends_with(".json") {
        Ok(ArtifactKind::CertifyAttempt)
    } else {
        Err(XtaskError::WriteFailure(format!(
            "failed to infer artifact kind from report path `{}`",
            path.display()
        )))
    }
}

fn attest_suite_output(
    suite: SuiteDefinition,
    output: &CommandOutput,
) -> Result<Vec<String>, AttestationFailure> {
    let attested = parse_libtest_attestations(output);
    let attested_tests = attested
        .iter()
        .map(|entry| entry.name.clone())
        .collect::<Vec<_>>();
    let mut sorted_attested_tests = attested_tests.clone();
    sorted_attested_tests.sort();

    if output.exit_code != 0 {
        return Err(AttestationFailure {
            attested_tests: sorted_attested_tests,
        });
    }

    if attested.is_empty() {
        return Err(AttestationFailure {
            attested_tests: Vec::new(),
        });
    }

    let ignored = attested
        .iter()
        .filter(|entry| matches!(entry.result, LibtestResult::Ignored))
        .map(|entry| entry.name.as_str())
        .collect::<Vec<_>>();
    if !ignored.is_empty() {
        return Err(AttestationFailure {
            attested_tests: sorted_attested_tests,
        });
    }

    let failed = attested
        .iter()
        .filter(|entry| matches!(entry.result, LibtestResult::Failed))
        .map(|entry| entry.name.as_str())
        .collect::<Vec<_>>();
    if !failed.is_empty() {
        return Err(AttestationFailure {
            attested_tests: sorted_attested_tests,
        });
    }

    let actual = sorted_attested_tests
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    if actual.len() != sorted_attested_tests.len() {
        return Err(AttestationFailure {
            attested_tests: sorted_attested_tests,
        });
    }

    let expected = suite
        .expected_tests
        .iter()
        .map(|test| (*test).to_string())
        .collect::<BTreeSet<_>>();

    let missing = expected.difference(&actual).cloned().collect::<Vec<_>>();
    if !missing.is_empty() {
        return Err(AttestationFailure {
            attested_tests: sorted_attested_tests,
        });
    }

    let extra = actual.difference(&expected).cloned().collect::<Vec<_>>();
    if !extra.is_empty() {
        return Err(AttestationFailure {
            attested_tests: sorted_attested_tests,
        });
    }

    Ok(sorted_attested_tests)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct AttestedTest {
    name: String,
    result: LibtestResult,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LibtestResult {
    Ok,
    Failed,
    Ignored,
}

fn parse_libtest_attestations(output: &CommandOutput) -> Vec<AttestedTest> {
    output
        .stdout
        .lines()
        .chain(output.stderr.lines())
        .filter_map(parse_libtest_line)
        .collect()
}

fn parse_libtest_line(line: &str) -> Option<AttestedTest> {
    let body = line.trim().strip_prefix("test ")?;
    let (name, result) = body.split_once(" ... ")?;
    let result = match result.trim() {
        "ok" => LibtestResult::Ok,
        "FAILED" => LibtestResult::Failed,
        "ignored" => LibtestResult::Ignored,
        _ => return None,
    };
    Some(AttestedTest {
        name: name.trim().to_string(),
        result,
    })
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn attestation_extracts_exact_sorted_libtest_names() {
        let suite = SuiteDefinition {
            name: "suite",
            command: &[],
            expected_tests: &["alpha::test_b", "alpha::test_a"],
        };
        let output = CommandOutput {
            exit_code: 0,
            stdout: [
                "Compiling crate v0.1.0",
                "test alpha::test_b ... ok",
                "random cargo noise",
                "test alpha::test_a ... ok",
                "test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out",
            ]
            .join("\n"),
            stderr: String::new(),
        };

        let attested = attest_suite_output(suite, &output).unwrap();

        assert_eq!(
            attested,
            vec!["alpha::test_a".to_string(), "alpha::test_b".to_string()]
        );
    }

    #[test]
    fn attestation_rejects_extra_or_ignored_tests() {
        let suite = SuiteDefinition {
            name: "suite",
            command: &[],
            expected_tests: &["alpha::test_a"],
        };
        let output = CommandOutput {
            exit_code: 0,
            stdout: [
                "test alpha::test_a ... ok",
                "test alpha::test_b ... ignored",
            ]
            .join("\n"),
            stderr: String::new(),
        };

        let failure = attest_suite_output(suite, &output).unwrap_err();

        assert_eq!(
            failure.attested_tests,
            vec!["alpha::test_a".to_string(), "alpha::test_b".to_string()]
        );
    }

    #[test]
    fn normalize_report_for_write_derives_phase_and_overall_by_artifact_kind() {
        let family = FamilyId::parse("function.wrapper.pipeline.chain3.v1").unwrap();
        let mut report = CertificationReport::new(
            &family,
            "abc123".to_string(),
            "rustc 1.89.0".to_string(),
            "2026-04-27T18:00:00Z".to_string(),
        );
        set_gates(&mut report, true, true, true, false);

        let prove_report =
            normalize_report_for_write(Path::new(PROVE_ARTIFACT_NAME), &report).unwrap();
        assert_eq!(prove_report.artifact_kind, ArtifactKind::ProveLatest);
        assert_eq!(
            prove_report.required_gates,
            vec![GateId::GateA, GateId::GateB, GateId::GateC]
        );
        assert_eq!(prove_report.phase_status, PassFail::Pass);
        assert_eq!(prove_report.overall_status, PassFail::Fail);

        let certify_report =
            normalize_report_for_write(Path::new("attempt-20260427.json"), &report).unwrap();
        assert_eq!(certify_report.artifact_kind, ArtifactKind::CertifyAttempt);
        assert_eq!(
            certify_report.required_gates,
            vec![GateId::GateA, GateId::GateB, GateId::GateC, GateId::GateD]
        );
        assert_eq!(certify_report.phase_status, PassFail::Fail);
        assert_eq!(certify_report.overall_status, PassFail::Fail);
    }
}
