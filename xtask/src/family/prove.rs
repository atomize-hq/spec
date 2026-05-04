use crate::family::harness::{
    FamilyHarness, GateResults, registered_family_harnesses, require_family_harness_in,
    validate_suite_ownership,
};
use crate::family::layout::validate_packet_layout;
use crate::family::manifest::{FamilyManifest, parse_manifest_file};
use crate::family::paths::{FamilyId, PacketPaths, ensure_packet_path_safe};
use crate::family::report::{
    ArtifactKind, CertificationReport, CommandRunner, SystemRunner, collect_fixture_digests,
    failed_suite_names, prove_artifact_path, run_suite, set_gates, set_overall, write_report,
};
use crate::{FamilyTargetLanguage, XtaskError};
use std::path::Path;

pub fn run(
    workspace_root: &Path,
    raw_family: &str,
    target_language: FamilyTargetLanguage,
) -> Result<(), XtaskError> {
    run_with_runner(workspace_root, raw_family, target_language, &SystemRunner)
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
    pub harness: FamilyHarness,
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
    target_language: FamilyTargetLanguage,
    runner: &R,
) -> Result<(), XtaskError> {
    let execution = execute(workspace_root, raw_family, target_language, runner)?;
    write_report(&prove_artifact_path(&execution.paths), &execution.report)?;
    execution.finish_for_prove()
}

pub(crate) fn execute<R: CommandRunner>(
    workspace_root: &Path,
    raw_family: &str,
    target_language: FamilyTargetLanguage,
    runner: &R,
) -> Result<ProveExecution, XtaskError> {
    execute_in(
        registered_family_harnesses(),
        workspace_root,
        raw_family,
        target_language,
        runner,
    )
}

pub(crate) fn execute_in<R: CommandRunner>(
    registry: &[FamilyHarness],
    workspace_root: &Path,
    raw_family: &str,
    target_language: FamilyTargetLanguage,
    runner: &R,
) -> Result<ProveExecution, XtaskError> {
    let family = FamilyId::parse(raw_family)?;
    validate_target_language(&family, target_language, "family prove")?;
    let harness = *require_family_harness_in(registry, &family, "family prove")?;
    let paths = PacketPaths::new(workspace_root, family.clone());
    let mut report = crate::family::report::build_report(workspace_root, &family, runner);

    if let Err(error) = ensure_packet_path_safe(workspace_root, &paths.root) {
        set_gates(&mut report, false, false, false, false);
        set_overall(&mut report, false);
        return Ok(ProveExecution {
            family,
            harness,
            manifest: None,
            paths,
            report,
            outcome: ProveOutcome::InvalidInput(error.to_string()),
        });
    }

    report.fixture_digests = collect_fixture_digests(&paths)?;

    if let Err(error) = validate_suite_ownership(
        &harness,
        &harness
            .prove_suites
            .iter()
            .map(|definition| definition.suite)
            .collect::<Vec<_>>(),
        "family prove",
    ) {
        set_gates(&mut report, false, false, false, false);
        set_overall(&mut report, false);
        return Ok(ProveExecution {
            family,
            harness,
            manifest: None,
            paths,
            report,
            outcome: ProveOutcome::InvalidInput(error.to_string()),
        });
    }

    let manifest = match parse_manifest_file(&paths.manifest, &family, &harness) {
        Ok(manifest) => {
            report.manifest_schema_version = manifest.schema_version;
            manifest
        }
        Err(error) => {
            set_gates(&mut report, false, false, false, false);
            set_overall(&mut report, false);
            return Ok(ProveExecution {
                family,
                harness,
                manifest: None,
                paths,
                report,
                outcome: ProveOutcome::InvalidInput(error.to_string()),
            });
        }
    };

    let layout = match validate_packet_layout(&paths.root, &manifest, &harness) {
        Ok(layout) => layout,
        Err(error) => {
            set_gates(&mut report, false, false, false, false);
            set_overall(&mut report, false);
            return Ok(ProveExecution {
                family,
                harness,
                manifest: None,
                paths,
                report,
                outcome: ProveOutcome::InvalidInput(error.to_string()),
            });
        }
    };
    let _bucket_count = layout.bucket_cases.len();
    let _case_count = layout.case_filenames.len();

    let mut gates = GateResults::default();
    report.suites = harness
        .prove_suites
        .iter()
        .map(|definition| {
            let suite = run_suite(workspace_root, runner, definition.suite);
            gates.set(definition.gate, suite.status.is_pass());
            suite
        })
        .collect();
    set_gates(
        &mut report,
        gates.gate_a,
        gates.gate_b,
        gates.gate_c,
        gates.gate_d,
    );

    let prove_passed = gates.satisfies(ArtifactKind::ProveLatest);
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
        harness,
        manifest: Some(manifest),
        paths,
        report,
        outcome,
    })
}

fn validate_target_language(
    family: &FamilyId,
    target_language: FamilyTargetLanguage,
    command: &str,
) -> Result<(), XtaskError> {
    match target_language {
        FamilyTargetLanguage::Rust => Ok(()),
        FamilyTargetLanguage::Typescript
            if matches!(
                family.as_str(),
                "function.arithmetic_leaf.monotone_up.v1" | "function.wrapper.pipeline.v1"
            ) =>
        {
            Ok(())
        }
        FamilyTargetLanguage::Typescript => Err(XtaskError::InvalidInput(format!(
            "{command} supports --target-language typescript only for function.arithmetic_leaf.monotone_up.v1 and function.wrapper.pipeline.v1"
        ))),
    }
}
