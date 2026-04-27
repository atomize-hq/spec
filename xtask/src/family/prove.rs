use crate::XtaskError;
use crate::family::layout::validate_packet_layout;
use crate::family::manifest::{FamilyManifest, parse_manifest_file};
use crate::family::paths::{FamilyId, PacketPaths, ensure_packet_path_safe};
use crate::family::report::{
    CertificationReport, CommandRunner, PROVE_SUITES, SystemRunner, collect_fixture_digests,
    failed_suite_names, prove_artifact_path, run_suite, set_gates, set_overall, write_report,
};
use std::path::Path;

pub fn run(workspace_root: &Path, raw_family: &str) -> Result<(), XtaskError> {
    run_with_runner(workspace_root, raw_family, &SystemRunner)
}

#[derive(Debug, Clone)]
pub(crate) enum ProveOutcome {
    Passed,
    InvalidInput(String),
    SuiteFailure(String),
}

#[derive(Debug, Clone)]
pub(crate) struct ProveExecution {
    pub family: FamilyId,
    pub manifest: Option<FamilyManifest>,
    pub paths: PacketPaths,
    pub report: CertificationReport,
    pub outcome: ProveOutcome,
}

impl ProveExecution {
    pub(crate) fn finish_for_prove(self) -> Result<(), XtaskError> {
        match self.outcome {
            ProveOutcome::Passed => Ok(()),
            ProveOutcome::InvalidInput(message) => Err(XtaskError::InvalidInput(message)),
            ProveOutcome::SuiteFailure(message) => Err(XtaskError::ProveSuiteFailure(message)),
        }
    }
}

pub(crate) fn run_with_runner<R: CommandRunner>(
    workspace_root: &Path,
    raw_family: &str,
    runner: &R,
) -> Result<(), XtaskError> {
    let execution = execute(workspace_root, raw_family, runner)?;
    write_report(&prove_artifact_path(&execution.paths), &execution.report)?;
    execution.finish_for_prove()
}

pub(crate) fn execute<R: CommandRunner>(
    workspace_root: &Path,
    raw_family: &str,
    runner: &R,
) -> Result<ProveExecution, XtaskError> {
    let family = FamilyId::parse(raw_family)?;
    let paths = PacketPaths::new(workspace_root, family.clone());
    let mut report = crate::family::report::build_report(workspace_root, &family, runner);

    if let Err(error) = ensure_packet_path_safe(workspace_root, &paths.root) {
        set_gates(&mut report, false, false, false, false);
        set_overall(&mut report, false);
        return Ok(ProveExecution {
            family,
            manifest: None,
            paths,
            report,
            outcome: ProveOutcome::InvalidInput(error.to_string()),
        });
    }

    report.fixture_digests = collect_fixture_digests(&paths)?;

    let manifest = match parse_manifest_file(&paths.manifest, &family) {
        Ok(manifest) => {
            report.manifest_schema_version = manifest.schema_version;
            manifest
        }
        Err(error) => {
            set_gates(&mut report, false, false, false, false);
            set_overall(&mut report, false);
            return Ok(ProveExecution {
                family,
                manifest: None,
                paths,
                report,
                outcome: ProveOutcome::InvalidInput(error.to_string()),
            });
        }
    };

    let layout = match validate_packet_layout(&paths.root, &manifest) {
        Ok(layout) => layout,
        Err(error) => {
            set_gates(&mut report, false, false, false, false);
            set_overall(&mut report, false);
            return Ok(ProveExecution {
                family,
                manifest: None,
                paths,
                report,
                outcome: ProveOutcome::InvalidInput(error.to_string()),
            });
        }
    };
    let _bucket_count = layout.bucket_cases.len();
    let _case_count = layout.case_filenames.len();

    report.suites = PROVE_SUITES
        .into_iter()
        .map(|suite| run_suite(workspace_root, runner, suite))
        .collect();

    let gate_a = report.suites[0].status.is_pass();
    let gate_c = report.suites[1].status.is_pass();
    let gate_b = report.suites[2].status.is_pass();
    set_gates(&mut report, gate_a, gate_b, gate_c, false);

    let prove_passed = gate_a && gate_b && gate_c;
    set_overall(&mut report, prove_passed);

    let outcome = if prove_passed {
        ProveOutcome::Passed
    } else {
        ProveOutcome::SuiteFailure(format!(
            "prove suite failure: {}",
            failed_suite_names(&report.suites)
        ))
    };

    Ok(ProveExecution {
        family,
        manifest: Some(manifest),
        paths,
        report,
        outcome,
    })
}
