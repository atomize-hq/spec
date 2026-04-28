use crate::XtaskError;
use crate::family::harness::GateResults;
use crate::family::prove;
use crate::family::report::{
    ArtifactKind, GateId, SystemRunner, certification_report_path, certify_attempt_path,
    failed_suite_names, refresh_generated_at, run_suite, set_gates, set_overall, write_report,
};
use crate::family::routing::manifest_routing_mismatch_message;
use std::path::Path;

pub fn run(workspace_root: &Path, raw_family: &str) -> Result<(), XtaskError> {
    run_with_runner(workspace_root, raw_family, &SystemRunner)
}

pub(crate) fn run_with_runner<R: crate::family::report::CommandRunner>(
    workspace_root: &Path,
    raw_family: &str,
    runner: &R,
) -> Result<(), XtaskError> {
    let prove_execution = match prove::execute(workspace_root, raw_family, runner) {
        Ok(execution) => execution,
        Err(error @ XtaskError::InvalidInput(_)) => return Err(error),
        Err(error @ XtaskError::WriteFailure(_)) => {
            return Err(XtaskError::CertifyArtifactWriteFailure(error.to_string()));
        }
        Err(other) => return Err(other),
    };

    let prove_latest_error = write_report(
        &crate::family::report::prove_artifact_path(&prove_execution.paths),
        &prove_execution.report,
    )
    .err();

    let mut report = prove_execution.report.clone();
    let family = prove_execution.family.clone();
    let harness = prove_execution.harness;
    let manifest = prove_execution.manifest.clone();
    let paths = prove_execution.paths.clone();
    report.artifact_kind = ArtifactKind::CertifyAttempt;
    refresh_generated_at(workspace_root, &mut report, runner);

    let prove_result = prove_execution.outcome.clone();
    let mut certify_error: Option<XtaskError> =
        prove_latest_error.map(|error| XtaskError::CertifyArtifactWriteFailure(error.to_string()));

    if certify_error.is_none() && matches!(prove_result, prove::ProveOutcome::Passed) {
        let manifest = manifest.ok_or_else(|| {
            XtaskError::Internal("prove passed without a parsed family manifest".to_string())
        })?;
        let prove_suite_count = harness.prove_suites.len();
        let extra_suites = harness
            .certify_suites
            .iter()
            .copied()
            .map(|suite| run_suite(workspace_root, runner, suite))
            .collect::<Vec<_>>();
        let routing_mismatch = manifest_routing_mismatch_message(&family, &manifest.routing);
        let mut gates = GateResults::from_report(&report);
        gates.set(
            GateId::GateD,
            extra_suites.iter().all(|suite| suite.status.is_pass()) && routing_mismatch.is_none(),
        );
        report.suites.extend(extra_suites);
        set_gates(
            &mut report,
            gates.gate_a,
            gates.gate_b,
            gates.gate_c,
            gates.gate_d,
        );
        let certify_passed = gates.satisfies(ArtifactKind::CertifyAttempt);
        set_overall(&mut report, certify_passed);
        if !certify_passed {
            let failure = report.suites[prove_suite_count..]
                .iter()
                .any(|suite| !suite.status.is_pass());
            certify_error = Some(XtaskError::CertifySuiteFailure(if failure {
                format!(
                    "certify suite failure: {}",
                    failed_suite_names(&report.suites[prove_suite_count..])
                )
            } else {
                format!(
                    "certify gate failure: gate_d{}",
                    routing_mismatch
                        .map(|message| format!(" ({message})"))
                        .unwrap_or_default()
                )
            }));
        }
    } else {
        set_overall(&mut report, false);
    }

    let attempt_path = certify_attempt_path(&paths, &report.generated_at);
    write_report(&attempt_path, &report)
        .map_err(|error| XtaskError::CertifyArtifactWriteFailure(error.to_string()))?;

    if let Some(error) = certify_error {
        return Err(error);
    }

    match prove_result {
        prove::ProveOutcome::Passed => {}
        prove::ProveOutcome::InvalidInput(message) => {
            return Err(XtaskError::InvalidInput(message));
        }
        prove::ProveOutcome::SuiteFailure(message) => {
            return Err(XtaskError::CertifyProveFailure(message));
        }
    }

    write_report(&certification_report_path(&paths), &report)
        .map_err(|error| XtaskError::CertifyArtifactWriteFailure(error.to_string()))?;
    Ok(())
}
