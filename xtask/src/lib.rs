mod family;

use clap::{Args, Parser, Subcommand};
use family::{
    certify, coverage, inventory, promotion_artifacts, prove, recommend, scaffold, smoke,
};
use std::ffi::OsString;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum XtaskError {
    #[error("{0}")]
    InvalidInput(String),
    #[error("{0}")]
    AlreadyExists(String),
    #[error("{0}")]
    ProveSuiteFailure(String),
    #[error("{0}")]
    CertifyProveFailure(String),
    #[error("{0}")]
    CertifySuiteFailure(String),
    #[error("{0}")]
    CertifyArtifactWriteFailure(String),
    #[error("{0}")]
    WriteFailure(String),
    #[error("{0}")]
    NotImplemented(String),
    #[error("{0}")]
    Internal(String),
}

impl XtaskError {
    pub fn exit_code(&self) -> i32 {
        match self {
            Self::InvalidInput(_) => 2,
            Self::AlreadyExists(_) | Self::ProveSuiteFailure(_) | Self::CertifyProveFailure(_) => 3,
            Self::WriteFailure(_) | Self::CertifySuiteFailure(_) => 4,
            Self::CertifyArtifactWriteFailure(_) => 5,
            Self::NotImplemented(_) | Self::Internal(_) => 1,
        }
    }

    fn safe_message(&self) -> &'static str {
        match self {
            Self::InvalidInput(_) => "invalid input",
            Self::AlreadyExists(_) => "resource already exists",
            Self::ProveSuiteFailure(_) => "prove suite failure",
            Self::CertifyProveFailure(_) => "family certify failed after prove",
            Self::CertifySuiteFailure(_) => "family certify failed one or more certify gates",
            Self::CertifyArtifactWriteFailure(_) => {
                "family certify could not write one or more certification artifacts"
            }
            Self::WriteFailure(_) => "write failure",
            Self::NotImplemented(_) => "not implemented",
            Self::Internal(_) => "internal error",
        }
    }
}

#[derive(Debug, Parser)]
#[command(bin_name = "xtask")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Family(FamilyArgs),
}

#[derive(Debug, Args)]
struct FamilyArgs {
    #[command(subcommand)]
    command: FamilyCommand,
}

#[derive(Debug, Subcommand)]
enum FamilyCommand {
    New {
        family: String,
    },
    Inventory {
        #[arg(long, default_value = "json")]
        format: String,
    },
    Coverage {
        #[arg(long, default_value = "json")]
        format: String,
    },
    Recommend {
        #[arg(long, default_value = "json")]
        format: String,
    },
    ValidateArtifact {
        path: String,
    },
    Smoke {
        family: String,
    },
    Prove {
        family: String,
    },
    Certify {
        family: String,
    },
}

pub fn run() -> i32 {
    let workspace_root = match std::env::current_dir() {
        Ok(path) => path,
        Err(error) => {
            eprintln!("failed to resolve current directory: {error}");
            return XtaskError::Internal("failed to resolve current directory".to_string())
                .exit_code();
        }
    };

    run_from(&workspace_root, std::env::args_os())
}

pub fn run_from<I, S>(workspace_root: &Path, args: I) -> i32
where
    I: IntoIterator<Item = S>,
    S: Into<OsString> + Clone,
{
    match dispatch(workspace_root, args) {
        Ok(()) => 0,
        Err(error) => {
            eprintln!("{}", error.safe_message());
            error.exit_code()
        }
    }
}

pub fn dispatch<I, S>(workspace_root: &Path, args: I) -> Result<(), XtaskError>
where
    I: IntoIterator<Item = S>,
    S: Into<OsString> + Clone,
{
    let cli =
        Cli::try_parse_from(args).map_err(|error| XtaskError::InvalidInput(error.to_string()))?;

    match cli.command {
        Command::Family(args) => match args.command {
            FamilyCommand::New { family } => scaffold::run(workspace_root, &family),
            FamilyCommand::Inventory { format } => inventory::run(workspace_root, &format),
            FamilyCommand::Coverage { format } => coverage::run(workspace_root, &format),
            FamilyCommand::Recommend { format } => recommend::run(workspace_root, &format),
            FamilyCommand::ValidateArtifact { path } => {
                promotion_artifacts::run_validate_artifact(workspace_root, &path)
            }
            FamilyCommand::Smoke { family } => smoke::run(workspace_root, &family),
            FamilyCommand::Prove { family } => prove::run(workspace_root, &family),
            FamilyCommand::Certify { family } => certify::run(workspace_root, &family),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::family::{
        certify,
        harness::{
            family_harness, family_harness_in, registered_harnesses_in_routing_order_from,
            require_family_harness_in, validate_suite_ownership, FamilyHarness, LockedManifestArgs,
            LockedManifestRouting, LockedManifestShape, ProveSuiteDefinition, ScaffoldDefinition,
            SmokeContract, StarterCaseDefinition, StarterTemplate, CHAIN3_CERTIFY_SUITES,
            CHAIN3_MUST_NOT_SHADOW, CHAIN3_PRECEDENCE, CHAIN3_PROVE_SUITES, CHAIN3_SUITE_SLUG,
            MONOTONE_DOWN_NONNEGATIVE_CERTIFY_SUITES, MONOTONE_DOWN_NONNEGATIVE_MUST_NOT_SHADOW,
            MONOTONE_DOWN_NONNEGATIVE_PRECEDENCE, MONOTONE_DOWN_NONNEGATIVE_PROVE_SUITES,
            MONOTONE_DOWN_NONNEGATIVE_SUITE_SLUG, MONOTONE_UP_CERTIFY_SUITES,
            MONOTONE_UP_MUST_NOT_SHADOW, MONOTONE_UP_PRECEDENCE, MONOTONE_UP_PROVE_SUITES,
            MONOTONE_UP_SUITE_SLUG, TERMINAL_UNSUPPORTED_CATCH_ALL,
            WRAPPER_PIPELINE_CERTIFY_SUITES, WRAPPER_PIPELINE_MUST_NOT_SHADOW,
            WRAPPER_PIPELINE_PRECEDENCE, WRAPPER_PIPELINE_PROVE_SUITES,
            WRAPPER_PIPELINE_SUITE_SLUG,
        },
        inventory,
        layout::validate_packet_layout,
        manifest::parse_manifest_file,
        manifest::Routing,
        paths::{
            FamilyId, PacketPaths, FAMILY_COVERAGE_LATEST_PATH,
            FAMILY_RECOMMENDATION_ANALYSIS_LATEST_PATH, REQUIRED_BUCKETS,
        },
        promotion_artifacts::{
            ApprovalRecord, ApprovalStatus, BlockerKind, BlockingStep, CandidateStatus,
            CommandRecord,
            ConfidenceLevel, DifficultyTier, FamilyRecommendationAnalysisArtifact,
            FamilyCoverageArtifact, FamilyRecommendationArtifact, GateStatus, GateSummary,
            HoldReason, MachineEvidence, MachineEvidenceKind, PromotionApprovals,
            PromotionArtifactKind,
            PromotionBlockerArtifact, PromotionExecutionArtifact, PromotionReadiness,
            RankedCandidate, RecommendationCandidateEntry, RecommendationConfidence,
            RecommendationDifficulty, RecommendationLeverage, RecommendationStatus,
            TargetLanguage, UnsupportedClusterEntry, RECOMMENDATION_ANALYSIS_SCHEMA_VERSION,
        },
        prove, recommend,
        report::{
            certification_report_path, run_suite, CommandOutput, CommandRunner, PassFail,
            SuiteDefinition, CERTIFY_ARTIFACT_NAME, PROVE_ARTIFACT_NAME,
        },
        routing::{
            locked_manifest_routing_in, locked_routing_order_with_terminal,
            locked_routing_order_with_terminal_from, routing_diagnostics_in, ManifestRoutingIssue,
            RegistryRoutingIssue,
        },
        scaffold, smoke,
    };
    use spec_core::loader::load_file;
    use spec_core::semantic_review::{
        UnsupportedFunctionReasonCode, evaluate_semantic_review, SemanticSupportStatus,
    };
    use spec_core::validator::validate_full;
    use std::cell::RefCell;
    use std::collections::{HashMap, VecDeque};
    use std::fs;
    use std::path::{Path, PathBuf};
    use tempfile::TempDir;

    const EMPTY_PROVE_SUITES: [ProveSuiteDefinition; 0] = [];
    const EMPTY_CERTIFY_SUITES: [SuiteDefinition; 0] = [];
    const SYNTHETIC_ALPHA_CASES: [StarterCaseDefinition; 1] = [StarterCaseDefinition {
        bucket: "aligned",
        path: "fixtures/aligned/units/alpha/alpha_wrapper_aligned.unit.spec",
    }];
    const SYNTHETIC_BETA_CASES: [StarterCaseDefinition; 1] = [StarterCaseDefinition {
        bucket: "aligned",
        path: "fixtures/aligned/units/beta/beta_wrapper_aligned.unit.spec",
    }];
    const SYNTHETIC_GAMMA_CASES: [StarterCaseDefinition; 1] = [StarterCaseDefinition {
        bucket: "aligned",
        path: "fixtures/aligned/units/gamma/gamma_wrapper_aligned.unit.spec",
    }];
    const SYNTHETIC_ALPHA_WITH_LEGACY_MUST_NOT_SHADOW: [&str; 3] = [
        "legacy.alpha.v1",
        "function.wrapper.pipeline.gamma.v1",
        "legacy.beta.v1",
    ];
    const SYNTHETIC_ALPHA_MUST_NOT_SHADOW: [&str; 1] = ["function.wrapper.pipeline.gamma.v1"];
    const SYNTHETIC_BETA_MUST_NOT_SHADOW: [&str; 2] = [
        "function.wrapper.pipeline.alpha.v1",
        "function.wrapper.pipeline.gamma.v1",
    ];
    const SYNTHETIC_BETA_MISSING_GAMMA_MUST_NOT_SHADOW: [&str; 1] =
        ["function.wrapper.pipeline.alpha.v1"];
    const SYNTHETIC_BETA_DUPLICATE_GAMMA_MUST_NOT_SHADOW: [&str; 3] = [
        "function.wrapper.pipeline.alpha.v1",
        "function.wrapper.pipeline.gamma.v1",
        "function.wrapper.pipeline.gamma.v1",
    ];
    const SYNTHETIC_BETA_OUT_OF_ORDER_MUST_NOT_SHADOW: [&str; 2] = [
        "function.wrapper.pipeline.gamma.v1",
        "function.wrapper.pipeline.alpha.v1",
    ];
    const SYNTHETIC_BETA_UNSUPPORTED_BEFORE_GAMMA_MUST_NOT_SHADOW: [&str; 3] = [
        "function.wrapper.pipeline.alpha.v1",
        TERMINAL_UNSUPPORTED_CATCH_ALL,
        "function.wrapper.pipeline.gamma.v1",
    ];
    const SYNTHETIC_BETA_DUPLICATE_UNSUPPORTED_MUST_NOT_SHADOW: [&str; 4] = [
        "function.wrapper.pipeline.alpha.v1",
        TERMINAL_UNSUPPORTED_CATCH_ALL,
        "function.wrapper.pipeline.gamma.v1",
        TERMINAL_UNSUPPORTED_CATCH_ALL,
    ];
    const SYNTHETIC_GAMMA_MUST_NOT_SHADOW: [&str; 1] = [TERMINAL_UNSUPPORTED_CATCH_ALL];
    const SYNTHETIC_ALPHA_HARNESS: FamilyHarness = FamilyHarness {
        family: "function.wrapper.pipeline.alpha.v1",
        summary: "alpha summary",
        suite_slug: "alpha_",
        scaffold: ScaffoldDefinition {
            unit_namespace: "alpha",
            template: StarterTemplate::GenericPlaceholder,
            starter_cases: &SYNTHETIC_ALPHA_CASES,
            smoke: SmokeContract {
                scaffold_exact_match_paths: &[],
                scaffold_file_contracts: &[],
            },
        },
        routing: LockedManifestRouting {
            precedence: 20,
            must_not_shadow: &SYNTHETIC_ALPHA_MUST_NOT_SHADOW,
        },
        shape: LockedManifestShape {
            dep_min: 1,
            dep_max: 1,
            control_flow: "straight_line_only",
            return_style: "let_then_return_or_direct_return",
            loops: false,
            branching: false,
            requires_supported_function_deps: true,
        },
        args: LockedManifestArgs {
            threading: "ordered_passthrough",
            allow_nested_argument_expressions: false,
            allow_literal_only_extra_args: false,
        },
        prove_suites: &EMPTY_PROVE_SUITES,
        certify_suites: &EMPTY_CERTIFY_SUITES,
    };
    const SYNTHETIC_BETA_HARNESS: FamilyHarness = FamilyHarness {
        family: "function.wrapper.pipeline.beta.v1",
        summary: "beta summary",
        suite_slug: "beta_",
        scaffold: ScaffoldDefinition {
            unit_namespace: "beta",
            template: StarterTemplate::GenericPlaceholder,
            starter_cases: &SYNTHETIC_BETA_CASES,
            smoke: SmokeContract {
                scaffold_exact_match_paths: &[],
                scaffold_file_contracts: &[],
            },
        },
        routing: LockedManifestRouting {
            precedence: 10,
            must_not_shadow: &SYNTHETIC_BETA_MUST_NOT_SHADOW,
        },
        shape: LockedManifestShape {
            dep_min: 1,
            dep_max: 1,
            control_flow: "straight_line_only",
            return_style: "let_then_return_or_direct_return",
            loops: false,
            branching: false,
            requires_supported_function_deps: true,
        },
        args: LockedManifestArgs {
            threading: "ordered_passthrough",
            allow_nested_argument_expressions: false,
            allow_literal_only_extra_args: false,
        },
        prove_suites: &EMPTY_PROVE_SUITES,
        certify_suites: &EMPTY_CERTIFY_SUITES,
    };
    const SYNTHETIC_GAMMA_HARNESS: FamilyHarness = FamilyHarness {
        family: "function.wrapper.pipeline.gamma.v1",
        summary: "gamma summary",
        suite_slug: "gamma_",
        scaffold: ScaffoldDefinition {
            unit_namespace: "gamma",
            template: StarterTemplate::GenericPlaceholder,
            starter_cases: &SYNTHETIC_GAMMA_CASES,
            smoke: SmokeContract {
                scaffold_exact_match_paths: &[],
                scaffold_file_contracts: &[],
            },
        },
        routing: LockedManifestRouting {
            precedence: 30,
            must_not_shadow: &SYNTHETIC_GAMMA_MUST_NOT_SHADOW,
        },
        shape: LockedManifestShape {
            dep_min: 1,
            dep_max: 1,
            control_flow: "straight_line_only",
            return_style: "let_then_return_or_direct_return",
            loops: false,
            branching: false,
            requires_supported_function_deps: true,
        },
        args: LockedManifestArgs {
            threading: "ordered_passthrough",
            allow_nested_argument_expressions: false,
            allow_literal_only_extra_args: false,
        },
        prove_suites: &EMPTY_PROVE_SUITES,
        certify_suites: &EMPTY_CERTIFY_SUITES,
    };
    const SYNTHETIC_MULTI_FAMILY_REGISTRY: [FamilyHarness; 3] = [
        SYNTHETIC_ALPHA_HARNESS,
        SYNTHETIC_GAMMA_HARNESS,
        SYNTHETIC_BETA_HARNESS,
    ];

    #[test]
    fn family_new_creates_locked_scaffold() {
        let temp_dir = workspace_root();
        let code = run_from(
            temp_dir.path(),
            [
                "xtask",
                "family",
                "new",
                "function.wrapper.pipeline.chain3.v1",
            ],
        );

        assert_eq!(code, 0);

        let family = FamilyId::parse("function.wrapper.pipeline.chain3.v1").unwrap();
        let paths = PacketPaths::new(temp_dir.path(), family);

        assert!(paths.root.is_dir());
        assert!(paths.candidate.is_file());
        assert!(paths.manifest.is_file());

        let manifest = fs::read_to_string(&paths.manifest).unwrap();
        assert!(manifest.contains("schema_version = 2"));
        assert!(manifest.contains("kind = \"function\""));
        assert!(manifest.contains(&format!("precedence = {CHAIN3_PRECEDENCE}")));
        assert!(manifest.contains("dep_min = 3"));
        assert!(manifest.contains("dep_max = 3"));
        for family_id in CHAIN3_MUST_NOT_SHADOW {
            assert_eq!(manifest.matches(family_id).count(), 1);
        }
        assert!(manifest.contains(
            "required_buckets = [\"aligned\", \"drift\", \"under_specified\", \"unsupported_near_miss\"]"
        ));

        let candidate = fs::read_to_string(&paths.candidate).unwrap();
        assert!(candidate.contains("## Aligned"));
        assert!(candidate.contains("## Drift"));
        assert!(candidate.contains("## Under Specified"));
        assert!(candidate.contains("## Unsupported Near Miss"));
        assert!(!candidate.contains("units/namespace/"));
        assert!(!candidate.contains("TODO: list each"));

        for bucket in REQUIRED_BUCKETS {
            let bucket_root = paths.fixtures.join(bucket);
            assert!(bucket_root.join("Cargo.toml").is_file());
            assert!(bucket_root.join("src/main.rs").is_file());
            assert!(bucket_root.join("units/pricing").is_dir());
            for relative_path in expected_chain3_scaffold_unit_paths(bucket) {
                let unit_path = paths.root.join(&relative_path);
                assert!(
                    unit_path.is_file(),
                    "missing scaffolded unit `{}`",
                    unit_path.display()
                );
                assert_candidate_lists_path_once(&candidate, &relative_path);
                assert_starter_spec_is_valid_and_non_proving(&unit_path);
            }
        }
    }

    #[test]
    fn family_new_creates_locked_monotone_down_nonnegative_scaffold() {
        let temp_dir = workspace_root();
        let family = "function.arithmetic_leaf.monotone_down_nonnegative.v1";

        assert_eq!(
            run_from(temp_dir.path(), ["xtask", "family", "new", family]),
            0
        );

        let family_id = FamilyId::parse(family).unwrap();
        let harness = family_harness(&family_id).unwrap();
        let paths = PacketPaths::new(temp_dir.path(), family_id);

        let manifest = fs::read_to_string(&paths.manifest).unwrap();
        assert!(manifest.contains("schema_version = 2"));
        assert!(manifest.contains(&format!(
            "precedence = {MONOTONE_DOWN_NONNEGATIVE_PRECEDENCE}"
        )));
        assert!(manifest.contains("dep_min = 0"));
        assert!(manifest.contains("dep_max = 1"));
        assert!(manifest.contains("requires_supported_function_deps = false"));
        for family_id in MONOTONE_DOWN_NONNEGATIVE_MUST_NOT_SHADOW {
            assert_eq!(manifest.matches(family_id).count(), 1);
        }

        let candidate = fs::read_to_string(&paths.candidate).unwrap();
        for case in harness.scaffold.starter_cases {
            let unit_path = paths.root.join(case.path);
            assert!(
                unit_path.is_file(),
                "missing monotone scaffold `{}`",
                unit_path.display()
            );
            assert_candidate_lists_path_once(&candidate, case.path);
        }

        let aligned = fs::read_to_string(
            paths
                .root
                .join("fixtures/aligned/units/pricing/apply_discount_aligned.unit.spec"),
        )
        .unwrap();
        assert!(aligned.contains("subtotal: Decimal"));
        assert!(aligned.contains("rate: Decimal"));
        assert!(aligned.contains("deps:\n  - money/round"));
        assert!(aligned.contains("round(discounted.max(Decimal::ZERO))"));

        let unsupported = fs::read_to_string(paths.root.join("fixtures/unsupported_near_miss/units/pricing/apply_discount_control_flow_unsupported_near_miss.unit.spec")).unwrap();
        assert!(unsupported.contains("if discounted < Decimal::ZERO"));
        assert!(!candidate.contains("TODO: replace"));
    }

    #[test]
    fn family_smoke_accepts_committed_monotone_down_nonnegative_scaffold_surfaces() {
        let temp_dir = workspace_root();
        let family = "function.arithmetic_leaf.monotone_down_nonnegative.v1";

        scaffold::run(temp_dir.path(), family).unwrap();

        assert_eq!(
            run_from(temp_dir.path(), ["xtask", "family", "smoke", family]),
            0
        );
    }

    #[test]
    fn family_smoke_rejects_committed_manifest_drift() {
        let temp_dir = workspace_root();
        let family =
            FamilyId::parse("function.arithmetic_leaf.monotone_down_nonnegative.v1").unwrap();
        let paths = PacketPaths::new(temp_dir.path(), family.clone());

        scaffold::run(temp_dir.path(), family.as_str()).unwrap();
        rewrite_manifest(&paths.manifest, "precedence = 3", "precedence = 33");

        let error = smoke::run(temp_dir.path(), family.as_str()).unwrap_err();
        assert!(matches!(error, XtaskError::InvalidInput(message)
            if message.contains("family smoke failed")
                && message.contains("committed scaffold exact-match file")
                && message.contains("family.toml")));
    }

    #[test]
    fn family_smoke_rejects_leaf_aligned_starter_shape_drift() {
        let committed_root = workspace_root();
        let scaffolded_root = workspace_root();
        let family =
            FamilyId::parse("function.arithmetic_leaf.monotone_down_nonnegative.v1").unwrap();

        scaffold::run(committed_root.path(), family.as_str()).unwrap();
        scaffold::run(scaffolded_root.path(), family.as_str()).unwrap();

        let scaffolded_paths = PacketPaths::new(scaffolded_root.path(), family.clone());
        let aligned_path = scaffolded_paths
            .root
            .join("fixtures/aligned/units/pricing/apply_discount_aligned.unit.spec");
        let aligned = fs::read_to_string(&aligned_path).unwrap();
        write_string(
            &aligned_path,
            &aligned.replacen("subtotal: Decimal", "amount: Decimal", 1),
        );

        let failures = smoke::collect_smoke_failures(
            &PacketPaths::new(committed_root.path(), family.clone()),
            &scaffolded_paths,
            *family_harness(&family).unwrap(),
        )
        .unwrap();

        assert!(failures.iter().any(|message| {
            message.contains("scaffolded smoke-contract file")
                && message.contains("subtotal: Decimal")
        }));
    }

    #[test]
    fn family_new_creates_locked_monotone_up_scaffold() {
        let temp_dir = workspace_root();
        let family = "function.arithmetic_leaf.monotone_up.v1";

        assert_eq!(
            run_from(temp_dir.path(), ["xtask", "family", "new", family]),
            0
        );

        let family_id = FamilyId::parse(family).unwrap();
        let harness = family_harness(&family_id).unwrap();
        let paths = PacketPaths::new(temp_dir.path(), family_id);

        let manifest = fs::read_to_string(&paths.manifest).unwrap();
        assert!(manifest.contains("schema_version = 2"));
        assert!(manifest.contains(&format!("precedence = {MONOTONE_UP_PRECEDENCE}")));
        assert!(manifest.contains("dep_min = 0"));
        assert!(manifest.contains("dep_max = 1"));
        assert!(manifest.contains("requires_supported_function_deps = false"));
        for family_id in MONOTONE_UP_MUST_NOT_SHADOW {
            assert_eq!(manifest.matches(family_id).count(), 1);
        }

        let candidate = fs::read_to_string(&paths.candidate).unwrap();
        for case in harness.scaffold.starter_cases {
            let unit_path = paths.root.join(case.path);
            assert!(
                unit_path.is_file(),
                "missing monotone-up scaffold `{}`",
                unit_path.display()
            );
            assert_candidate_lists_path_once(&candidate, case.path);
        }

        let aligned = fs::read_to_string(
            paths
                .root
                .join("fixtures/aligned/units/pricing/apply_tax_aligned.unit.spec"),
        )
        .unwrap();
        assert!(aligned.contains("subtotal: Decimal"));
        assert!(aligned.contains("rate: Decimal"));
        assert!(aligned.contains("- output >= subtotal"));
        assert!(aligned.contains("deps:\n  - money/round"));
        assert!(aligned.contains("let taxed = subtotal + subtotal * rate;"));
        assert!(aligned.contains("round(taxed)"));

        let unsupported = fs::read_to_string(paths.root.join("fixtures/unsupported_near_miss/units/pricing/apply_tax_control_flow_unsupported_near_miss.unit.spec")).unwrap();
        assert!(unsupported.contains("if rate == Decimal::ZERO"));
        assert!(!candidate.contains("TODO: replace"));
    }

    #[test]
    fn family_smoke_accepts_committed_monotone_up_scaffold_surfaces() {
        let temp_dir = workspace_root();
        let family = "function.arithmetic_leaf.monotone_up.v1";

        scaffold::run(temp_dir.path(), family).unwrap();

        assert_eq!(
            run_from(temp_dir.path(), ["xtask", "family", "smoke", family]),
            0
        );
    }

    #[test]
    fn family_smoke_rejects_monotone_up_aligned_starter_shape_drift() {
        let committed_root = workspace_root();
        let scaffolded_root = workspace_root();
        let family = FamilyId::parse("function.arithmetic_leaf.monotone_up.v1").unwrap();

        scaffold::run(committed_root.path(), family.as_str()).unwrap();
        scaffold::run(scaffolded_root.path(), family.as_str()).unwrap();

        let scaffolded_paths = PacketPaths::new(scaffolded_root.path(), family.clone());
        let aligned_path = scaffolded_paths
            .root
            .join("fixtures/aligned/units/pricing/apply_tax_aligned.unit.spec");
        let aligned = fs::read_to_string(&aligned_path).unwrap();
        write_string(
            &aligned_path,
            &aligned.replacen("subtotal: Decimal", "amount: Decimal", 1),
        );

        let failures = smoke::collect_smoke_failures(
            &PacketPaths::new(committed_root.path(), family.clone()),
            &scaffolded_paths,
            *family_harness(&family).unwrap(),
        )
        .unwrap();

        assert!(failures.iter().any(|message| {
            message.contains("scaffolded smoke-contract file")
                && message.contains("subtotal: Decimal")
        }));
    }

    #[test]
    fn locked_routing_helper_uses_registered_families_plus_terminal() {
        assert_eq!(
            locked_routing_order_with_terminal(),
            [
                "function.wrapper.pipeline.chain3.v1",
                "function.wrapper.pipeline.v1",
                "function.arithmetic_leaf.monotone_down_nonnegative.v1",
                "function.arithmetic_leaf.monotone_up.v1",
                "unsupported.function.v1",
            ]
        );
    }

    #[test]
    fn chain3_harness_contract_is_locked() {
        let temp_dir = workspace_root();
        let family = "function.wrapper.pipeline.chain3.v1";
        let family_id = FamilyId::parse(family).unwrap();
        let harness = family_harness(&family_id).expect("chain3 harness should be registered");

        assert_eq!(
            run_from(temp_dir.path(), ["xtask", "family", "new", family]),
            0
        );
        assert_eq!(CHAIN3_PROVE_SUITES.len(), 3);
        assert_eq!(CHAIN3_CERTIFY_SUITES.len(), 2);
        assert_eq!(
            harness
                .prove_suites
                .iter()
                .map(|definition| definition.gate)
                .collect::<Vec<_>>(),
            vec![
                crate::family::report::GateId::GateA,
                crate::family::report::GateId::GateC,
                crate::family::report::GateId::GateB,
            ]
        );
        assert_eq!(
            CHAIN3_PROVE_SUITES
                .iter()
                .map(|suite| suite.name)
                .collect::<Vec<_>>(),
            vec![
                "spec-core:m21_chain3_classifier_",
                "spec-cli:m21_chain3_truth_surface_",
                "spec-cli:m21_chain3_corpus_",
            ]
        );
        assert_eq!(
            CHAIN3_CERTIFY_SUITES
                .iter()
                .map(|suite| suite.name)
                .collect::<Vec<_>>(),
            vec![
                "spec-core:m21_chain3_regression_",
                "spec-cli:m21_chain3_regression_",
            ]
        );
        assert_eq!(harness.suite_slug, CHAIN3_SUITE_SLUG);
    }

    #[test]
    fn family_new_creates_locked_wrapper_pipeline_scaffold() {
        let temp_dir = workspace_root();
        let family = "function.wrapper.pipeline.v1";

        assert_eq!(
            run_from(temp_dir.path(), ["xtask", "family", "new", family]),
            0
        );

        let family_id = FamilyId::parse(family).unwrap();
        let harness = family_harness(&family_id).unwrap();
        let paths = PacketPaths::new(temp_dir.path(), family_id);

        let manifest = fs::read_to_string(&paths.manifest).unwrap();
        assert!(manifest.contains("schema_version = 2"));
        assert!(manifest.contains(&format!("precedence = {WRAPPER_PIPELINE_PRECEDENCE}")));
        assert!(manifest.contains("dep_min = 2"));
        assert!(manifest.contains("dep_max = 2"));
        assert!(manifest.contains("requires_supported_function_deps = true"));
        for family_id in WRAPPER_PIPELINE_MUST_NOT_SHADOW {
            assert_eq!(manifest.matches(family_id).count(), 1);
        }

        let candidate = fs::read_to_string(&paths.candidate).unwrap();
        for bucket in REQUIRED_BUCKETS {
            for relative_path in expected_wrapper_pipeline_scaffold_unit_paths(bucket) {
                let unit_path = paths.root.join(&relative_path);
                assert!(
                    unit_path.is_file(),
                    "missing wrapper scaffold `{}`",
                    unit_path.display()
                );
                assert_candidate_lists_path_once(&candidate, &relative_path);
                let loaded = load_file(&unit_path).unwrap();
                validate_full(&loaded).unwrap();
            }
        }

        let aligned = fs::read_to_string(
            paths
                .root
                .join("fixtures/aligned/units/pricing/pricing_total_wrapper_aligned.unit.spec"),
        )
        .unwrap();
        assert!(aligned.contains("discount_rate: Decimal"));
        assert!(aligned.contains("tax_rate: Decimal"));
        assert!(aligned.contains("pricing_discount_leaf_aligned(subtotal, discount_rate)"));
        assert!(aligned.contains("pricing_tax_leaf_aligned(discounted, tax_rate)"));

        let drift = fs::read_to_string(
            paths
                .root
                .join("fixtures/drift/units/pricing/pricing_total_wrapper_drift.unit.spec"),
        )
        .unwrap();
        assert!(drift.contains("let taxed = pricing_tax_leaf_drift(subtotal, tax_rate);"));
        assert!(drift.contains("pricing_discount_leaf_drift(taxed, discount_rate)"));

        let under_specified = fs::read_to_string(paths.root.join("fixtures/under_specified/units/pricing/pricing_total_wrapper_under_specified.unit.spec")).unwrap();
        assert!(under_specified
            .contains("why: Adjust the checkout total using the current pricing inputs."));

        let unsupported = fs::read_to_string(paths.root.join("fixtures/unsupported_near_miss/units/pricing/pricing_total_wrapper_unsupported_near_miss.unit.spec")).unwrap();
        assert!(unsupported.contains("tax_rate.max(Decimal::ZERO)"));
        assert!(!candidate.contains("TODO: replace"));
        assert_eq!(harness.suite_slug, WRAPPER_PIPELINE_SUITE_SLUG);
    }

    #[test]
    fn family_smoke_accepts_committed_wrapper_pipeline_scaffold_surfaces() {
        let temp_dir = workspace_root();
        let family = "function.wrapper.pipeline.v1";

        scaffold::run(temp_dir.path(), family).unwrap();

        assert_eq!(
            run_from(temp_dir.path(), ["xtask", "family", "smoke", family]),
            0
        );
    }

    #[test]
    fn family_smoke_rejects_wrapper_pipeline_aligned_starter_shape_drift() {
        let committed_root = workspace_root();
        let scaffolded_root = workspace_root();
        let family = FamilyId::parse("function.wrapper.pipeline.v1").unwrap();

        scaffold::run(committed_root.path(), family.as_str()).unwrap();
        scaffold::run(scaffolded_root.path(), family.as_str()).unwrap();

        let scaffolded_paths = PacketPaths::new(scaffolded_root.path(), family.clone());
        let aligned_path = scaffolded_paths
            .root
            .join("fixtures/aligned/units/pricing/pricing_total_wrapper_aligned.unit.spec");
        let aligned = fs::read_to_string(&aligned_path).unwrap();
        write_string(
            &aligned_path,
            &aligned.replacen("discount_rate: Decimal", "discount: Decimal", 1),
        );

        let failures = smoke::collect_smoke_failures(
            &PacketPaths::new(committed_root.path(), family.clone()),
            &scaffolded_paths,
            *family_harness(&family).unwrap(),
        )
        .unwrap();

        assert!(failures.iter().any(|message| {
            message.contains("scaffolded smoke-contract file")
                && message.contains("discount_rate: Decimal")
        }));
    }

    #[test]
    fn wrapper_pipeline_harness_contract_is_locked() {
        let temp_dir = workspace_root();
        let family = "function.wrapper.pipeline.v1";
        let family_id = FamilyId::parse(family).unwrap();
        let harness =
            family_harness(&family_id).expect("wrapper pipeline harness should be registered");

        assert_eq!(
            run_from(temp_dir.path(), ["xtask", "family", "new", family]),
            0
        );
        assert_eq!(WRAPPER_PIPELINE_PROVE_SUITES.len(), 3);
        assert_eq!(WRAPPER_PIPELINE_CERTIFY_SUITES.len(), 1);
        assert_eq!(
            harness
                .prove_suites
                .iter()
                .map(|definition| definition.gate)
                .collect::<Vec<_>>(),
            vec![
                crate::family::report::GateId::GateA,
                crate::family::report::GateId::GateC,
                crate::family::report::GateId::GateB,
            ]
        );
        assert_eq!(harness.shape.dep_min, 2);
        assert_eq!(harness.shape.dep_max, 2);
        assert!(harness.shape.requires_supported_function_deps);
        assert_eq!(
            harness.scaffold.smoke.scaffold_exact_match_paths,
            ["family.toml"]
        );
        assert_eq!(harness.scaffold.smoke.scaffold_file_contracts.len(), 1);
        assert_eq!(
            harness.scaffold.smoke.scaffold_file_contracts[0].path,
            "fixtures/aligned/units/pricing/pricing_total_wrapper_aligned.unit.spec"
        );
        assert_eq!(harness.suite_slug, WRAPPER_PIPELINE_SUITE_SLUG);
    }

    #[test]
    fn monotone_down_nonnegative_harness_contract_is_locked() {
        let temp_dir = workspace_root();
        let family = "function.arithmetic_leaf.monotone_down_nonnegative.v1";
        let family_id = FamilyId::parse(family).unwrap();
        let harness = family_harness(&family_id).expect("leaf harness should be registered");

        assert_eq!(
            run_from(temp_dir.path(), ["xtask", "family", "new", family]),
            0
        );
        assert_eq!(MONOTONE_DOWN_NONNEGATIVE_PROVE_SUITES.len(), 3);
        assert_eq!(MONOTONE_DOWN_NONNEGATIVE_CERTIFY_SUITES.len(), 2);
        assert_eq!(
            harness
                .prove_suites
                .iter()
                .map(|definition| definition.gate)
                .collect::<Vec<_>>(),
            vec![
                crate::family::report::GateId::GateA,
                crate::family::report::GateId::GateC,
                crate::family::report::GateId::GateB,
            ]
        );
        assert_eq!(harness.shape.dep_min, 0);
        assert_eq!(harness.shape.dep_max, 1);
        assert!(!harness.shape.requires_supported_function_deps);
        assert_eq!(
            harness.scaffold.smoke.scaffold_exact_match_paths,
            ["family.toml"]
        );
        assert_eq!(harness.scaffold.smoke.scaffold_file_contracts.len(), 1);
        assert_eq!(
            harness.scaffold.smoke.scaffold_file_contracts[0].path,
            "fixtures/aligned/units/pricing/apply_discount_aligned.unit.spec"
        );
        assert_eq!(harness.suite_slug, MONOTONE_DOWN_NONNEGATIVE_SUITE_SLUG);
    }

    #[test]
    fn monotone_up_harness_contract_is_locked() {
        let temp_dir = workspace_root();
        let family = "function.arithmetic_leaf.monotone_up.v1";
        let family_id = FamilyId::parse(family).unwrap();
        let harness = family_harness(&family_id).expect("monotone-up harness should be registered");

        assert_eq!(
            run_from(temp_dir.path(), ["xtask", "family", "new", family]),
            0
        );
        assert_eq!(MONOTONE_UP_PROVE_SUITES.len(), 3);
        assert_eq!(MONOTONE_UP_CERTIFY_SUITES.len(), 2);
        assert_eq!(
            harness
                .prove_suites
                .iter()
                .map(|definition| definition.gate)
                .collect::<Vec<_>>(),
            vec![
                crate::family::report::GateId::GateA,
                crate::family::report::GateId::GateC,
                crate::family::report::GateId::GateB,
            ]
        );
        assert_eq!(harness.shape.dep_min, 0);
        assert_eq!(harness.shape.dep_max, 1);
        assert!(!harness.shape.requires_supported_function_deps);
        assert_eq!(
            harness.scaffold.smoke.scaffold_exact_match_paths,
            ["family.toml"]
        );
        assert_eq!(harness.scaffold.smoke.scaffold_file_contracts.len(), 1);
        assert_eq!(
            harness.scaffold.smoke.scaffold_file_contracts[0].path,
            "fixtures/aligned/units/pricing/apply_tax_aligned.unit.spec"
        );
        assert_eq!(harness.suite_slug, MONOTONE_UP_SUITE_SLUG);
    }

    #[test]
    fn suite_ownership_rejects_suite_names_without_locked_slug() {
        let harness = family_harness(
            &FamilyId::parse("function.arithmetic_leaf.monotone_down_nonnegative.v1").unwrap(),
        )
        .unwrap();
        let suites = [SuiteDefinition {
            name: "spec-core:leaf_classifier_",
            command: &["cargo", "test"],
            expected_tests: &[
                "semantic_review::tests::monotone_down_nonnegative_classifier_aligned_fixture_routes_to_promoted_leaf",
            ],
        }];

        let error = validate_suite_ownership(harness, &suites, "family prove").unwrap_err();
        assert!(matches!(error, XtaskError::InvalidInput(message)
            if message.contains("suite names must include `monotone_down_nonnegative_`")));
    }

    #[test]
    fn suite_ownership_rejects_expected_tests_without_locked_slug() {
        let harness = family_harness(
            &FamilyId::parse("function.arithmetic_leaf.monotone_down_nonnegative.v1").unwrap(),
        )
        .unwrap();
        let suites = [SuiteDefinition {
            name: "spec-core:monotone_down_nonnegative_classifier_",
            command: &["cargo", "test"],
            expected_tests: &["semantic_review::tests::leaf_classifier_aligned_fixture"],
        }];

        let error = validate_suite_ownership(harness, &suites, "family prove").unwrap_err();
        assert!(matches!(error, XtaskError::InvalidInput(message)
            if message.contains("expected test names must include `monotone_down_nonnegative_`")));
    }

    #[test]
    fn monotone_up_suite_ownership_rejects_suite_names_without_locked_slug() {
        let harness =
            family_harness(&FamilyId::parse("function.arithmetic_leaf.monotone_up.v1").unwrap())
                .unwrap();
        let suites = [SuiteDefinition {
            name: "spec-core:leaf_classifier_",
            command: &["cargo", "test"],
            expected_tests: &[
                "semantic_review::tests::monotone_up_classifier_aligned_fixture_routes_to_promoted_leaf",
            ],
        }];

        let error = validate_suite_ownership(harness, &suites, "family prove").unwrap_err();
        assert!(matches!(error, XtaskError::InvalidInput(message)
            if message.contains("suite names must include `monotone_up_`")));
    }

    #[test]
    fn monotone_up_suite_ownership_rejects_expected_tests_without_locked_slug() {
        let harness =
            family_harness(&FamilyId::parse("function.arithmetic_leaf.monotone_up.v1").unwrap())
                .unwrap();
        let suites = [SuiteDefinition {
            name: "spec-core:monotone_up_classifier_",
            command: &["cargo", "test"],
            expected_tests: &["semantic_review::tests::leaf_classifier_aligned_fixture"],
        }];

        let error = validate_suite_ownership(harness, &suites, "family prove").unwrap_err();
        assert!(matches!(error, XtaskError::InvalidInput(message)
            if message.contains("expected test names must include `monotone_up_`")));
    }

    #[test]
    fn family_commands_reject_unregistered_families_as_not_implemented() {
        let temp_dir = workspace_root();
        let family = "function.wrapper.pipeline.chain4.v1";

        for args in [
            vec!["xtask", "family", "new", family],
            vec!["xtask", "family", "smoke", family],
            vec!["xtask", "family", "prove", family],
            vec!["xtask", "family", "certify", family],
        ] {
            let error = dispatch(temp_dir.path(), args).unwrap_err();
            assert!(
                matches!(error, XtaskError::NotImplemented(ref message)
                    if message.contains(family)
                        && message.contains("xtask/src/family/harness.rs")),
                "unexpected error variant for `{family}`"
            );
        }
    }

    #[test]
    fn synthetic_family_registry_supports_lookup_and_locked_manifest_routing() {
        let alpha = FamilyId::parse("function.wrapper.pipeline.alpha.v1").unwrap();
        let gamma = FamilyId::parse("function.wrapper.pipeline.gamma.v1").unwrap();
        let delta = FamilyId::parse("function.wrapper.pipeline.delta.v1").unwrap();

        assert_eq!(
            family_harness_in(&SYNTHETIC_MULTI_FAMILY_REGISTRY, &alpha)
                .map(|harness| harness.family),
            Some("function.wrapper.pipeline.alpha.v1")
        );
        assert_eq!(
            family_harness_in(&SYNTHETIC_MULTI_FAMILY_REGISTRY, &gamma)
                .map(|harness| harness.family),
            Some("function.wrapper.pipeline.gamma.v1")
        );

        let alpha_harness = require_family_harness_in(
            &SYNTHETIC_MULTI_FAMILY_REGISTRY,
            &alpha,
            "synthetic coverage",
        )
        .unwrap();
        assert_eq!(alpha_harness.routing.precedence, 20);
        assert_eq!(
            locked_manifest_routing_in(&SYNTHETIC_MULTI_FAMILY_REGISTRY, &gamma)
                .map(|routing| (routing.precedence, routing.must_not_shadow)),
            Some((30, &SYNTHETIC_GAMMA_MUST_NOT_SHADOW[..]))
        );
        assert!(matches!(
            require_family_harness_in(&SYNTHETIC_MULTI_FAMILY_REGISTRY, &delta, "synthetic coverage"),
            Err(XtaskError::NotImplemented(message))
                if message.contains("xtask/src/family/harness.rs")
                    && message.contains("family `function.wrapper.pipeline.delta.v1`")
        ));
    }

    #[test]
    fn synthetic_registry_routing_order_uses_registered_families_plus_terminal() {
        let routing_order =
            registered_harnesses_in_routing_order_from(&SYNTHETIC_MULTI_FAMILY_REGISTRY)
                .into_iter()
                .map(|harness| harness.family)
                .collect::<Vec<_>>();
        assert_eq!(
            routing_order,
            [
                "function.wrapper.pipeline.beta.v1",
                "function.wrapper.pipeline.alpha.v1",
                "function.wrapper.pipeline.gamma.v1",
            ]
        );
        assert_ne!(
            locked_routing_order_with_terminal_from(&SYNTHETIC_MULTI_FAMILY_REGISTRY),
            locked_routing_order_with_terminal()
        );
        assert_eq!(
            locked_routing_order_with_terminal_from(&SYNTHETIC_MULTI_FAMILY_REGISTRY),
            [
                "function.wrapper.pipeline.beta.v1",
                "function.wrapper.pipeline.alpha.v1",
                "function.wrapper.pipeline.gamma.v1",
                TERMINAL_UNSUPPORTED_CATCH_ALL,
            ]
        );
    }

    #[test]
    fn synthetic_registry_routing_diagnostics_ignore_unregistered_legacy_entries() {
        let alpha = FamilyId::parse("function.wrapper.pipeline.alpha.v1").unwrap();
        let registry = [
            FamilyHarness {
                routing: LockedManifestRouting {
                    precedence: SYNTHETIC_ALPHA_HARNESS.routing.precedence,
                    must_not_shadow: &SYNTHETIC_ALPHA_WITH_LEGACY_MUST_NOT_SHADOW,
                },
                ..SYNTHETIC_ALPHA_HARNESS
            },
            SYNTHETIC_GAMMA_HARNESS,
            SYNTHETIC_BETA_HARNESS,
        ];
        let routing = Routing {
            precedence: SYNTHETIC_ALPHA_HARNESS.routing.precedence,
            must_not_shadow: SYNTHETIC_ALPHA_WITH_LEGACY_MUST_NOT_SHADOW
                .iter()
                .map(|family_id| (*family_id).to_string())
                .collect(),
        };

        let diagnostics = routing_diagnostics_in(&registry, &alpha, &routing);

        assert!(diagnostics.manifest.passed);
        assert!(diagnostics.registry.passed);
    }

    #[test]
    fn synthetic_manifest_routing_diagnostics_reject_selected_family_unregistered_entry_mismatch() {
        let alpha = FamilyId::parse("function.wrapper.pipeline.alpha.v1").unwrap();
        let mismatched = Routing {
            precedence: SYNTHETIC_ALPHA_HARNESS.routing.precedence,
            must_not_shadow: SYNTHETIC_ALPHA_WITH_LEGACY_MUST_NOT_SHADOW
                .iter()
                .map(|family_id| (*family_id).to_string())
                .collect(),
        };

        let diagnostics =
            routing_diagnostics_in(&SYNTHETIC_MULTI_FAMILY_REGISTRY, &alpha, &mismatched);

        assert!(!diagnostics.manifest.passed);
        assert_eq!(
            diagnostics.manifest.issue,
            Some(ManifestRoutingIssue::MustNotShadowMismatch {
                expected: SYNTHETIC_ALPHA_MUST_NOT_SHADOW
                    .iter()
                    .map(|family_id| (*family_id).to_string())
                    .collect(),
                found: SYNTHETIC_ALPHA_WITH_LEGACY_MUST_NOT_SHADOW
                    .iter()
                    .map(|family_id| (*family_id).to_string())
                    .collect(),
            })
        );
        assert!(diagnostics.registry.passed);
    }

    #[test]
    fn synthetic_registry_routing_diagnostics_reject_duplicate_registered_family_ids() {
        let alpha = FamilyId::parse("function.wrapper.pipeline.alpha.v1").unwrap();
        let registry = [
            SYNTHETIC_ALPHA_HARNESS,
            SYNTHETIC_ALPHA_HARNESS,
            SYNTHETIC_GAMMA_HARNESS,
        ];
        let routing = Routing {
            precedence: SYNTHETIC_ALPHA_HARNESS.routing.precedence,
            must_not_shadow: SYNTHETIC_ALPHA_MUST_NOT_SHADOW
                .iter()
                .map(|family_id| (*family_id).to_string())
                .collect(),
        };

        let diagnostics = routing_diagnostics_in(&registry, &alpha, &routing);

        assert!(diagnostics.registry.issues.contains(
            &RegistryRoutingIssue::DuplicateRegisteredFamilyId {
                family: "function.wrapper.pipeline.alpha.v1".to_string(),
            }
        ));
    }

    #[test]
    fn synthetic_registry_routing_diagnostics_reject_duplicate_precedence() {
        let alpha = FamilyId::parse("function.wrapper.pipeline.alpha.v1").unwrap();
        let registry = [
            FamilyHarness {
                routing: LockedManifestRouting {
                    precedence: SYNTHETIC_ALPHA_HARNESS.routing.precedence,
                    ..SYNTHETIC_BETA_HARNESS.routing
                },
                ..SYNTHETIC_BETA_HARNESS
            },
            SYNTHETIC_ALPHA_HARNESS,
            SYNTHETIC_GAMMA_HARNESS,
        ];
        let routing = Routing {
            precedence: SYNTHETIC_ALPHA_HARNESS.routing.precedence,
            must_not_shadow: SYNTHETIC_ALPHA_MUST_NOT_SHADOW
                .iter()
                .map(|family_id| (*family_id).to_string())
                .collect(),
        };

        let diagnostics = routing_diagnostics_in(&registry, &alpha, &routing);

        assert!(diagnostics
            .registry
            .issues
            .contains(&RegistryRoutingIssue::DuplicatePrecedence {
                precedence: SYNTHETIC_ALPHA_HARNESS.routing.precedence,
                families: vec![
                    "function.wrapper.pipeline.beta.v1".to_string(),
                    "function.wrapper.pipeline.alpha.v1".to_string(),
                ],
            }));
    }

    #[test]
    fn synthetic_registry_routing_diagnostics_reject_missing_registered_successor() {
        let beta = FamilyId::parse("function.wrapper.pipeline.beta.v1").unwrap();
        let registry = [
            FamilyHarness {
                routing: LockedManifestRouting {
                    must_not_shadow: &SYNTHETIC_BETA_MISSING_GAMMA_MUST_NOT_SHADOW,
                    ..SYNTHETIC_BETA_HARNESS.routing
                },
                ..SYNTHETIC_BETA_HARNESS
            },
            SYNTHETIC_ALPHA_HARNESS,
            SYNTHETIC_GAMMA_HARNESS,
        ];
        let routing = Routing {
            precedence: SYNTHETIC_BETA_HARNESS.routing.precedence,
            must_not_shadow: SYNTHETIC_BETA_MISSING_GAMMA_MUST_NOT_SHADOW
                .iter()
                .map(|family_id| (*family_id).to_string())
                .collect(),
        };

        let diagnostics = routing_diagnostics_in(&registry, &beta, &routing);

        assert!(diagnostics.registry.issues.contains(
            &RegistryRoutingIssue::MissingRegisteredSuccessor {
                family: "function.wrapper.pipeline.beta.v1".to_string(),
                successor: "function.wrapper.pipeline.gamma.v1".to_string(),
            }
        ));
    }

    #[test]
    fn synthetic_registry_routing_diagnostics_reject_duplicate_registered_successor() {
        let beta = FamilyId::parse("function.wrapper.pipeline.beta.v1").unwrap();
        let registry = [
            FamilyHarness {
                routing: LockedManifestRouting {
                    must_not_shadow: &SYNTHETIC_BETA_DUPLICATE_GAMMA_MUST_NOT_SHADOW,
                    ..SYNTHETIC_BETA_HARNESS.routing
                },
                ..SYNTHETIC_BETA_HARNESS
            },
            SYNTHETIC_ALPHA_HARNESS,
            SYNTHETIC_GAMMA_HARNESS,
        ];
        let routing = Routing {
            precedence: SYNTHETIC_BETA_HARNESS.routing.precedence,
            must_not_shadow: SYNTHETIC_BETA_DUPLICATE_GAMMA_MUST_NOT_SHADOW
                .iter()
                .map(|family_id| (*family_id).to_string())
                .collect(),
        };

        let diagnostics = routing_diagnostics_in(&registry, &beta, &routing);

        assert!(diagnostics.registry.issues.contains(
            &RegistryRoutingIssue::DuplicateRegisteredSuccessor {
                family: "function.wrapper.pipeline.beta.v1".to_string(),
                successor: "function.wrapper.pipeline.gamma.v1".to_string(),
            }
        ));
    }

    #[test]
    fn synthetic_registry_routing_diagnostics_reject_registered_successors_out_of_order() {
        let beta = FamilyId::parse("function.wrapper.pipeline.beta.v1").unwrap();
        let registry = [
            FamilyHarness {
                routing: LockedManifestRouting {
                    must_not_shadow: &SYNTHETIC_BETA_OUT_OF_ORDER_MUST_NOT_SHADOW,
                    ..SYNTHETIC_BETA_HARNESS.routing
                },
                ..SYNTHETIC_BETA_HARNESS
            },
            SYNTHETIC_ALPHA_HARNESS,
            SYNTHETIC_GAMMA_HARNESS,
        ];
        let routing = Routing {
            precedence: SYNTHETIC_BETA_HARNESS.routing.precedence,
            must_not_shadow: SYNTHETIC_BETA_OUT_OF_ORDER_MUST_NOT_SHADOW
                .iter()
                .map(|family_id| (*family_id).to_string())
                .collect(),
        };

        let diagnostics = routing_diagnostics_in(&registry, &beta, &routing);

        assert!(diagnostics.registry.issues.contains(
            &RegistryRoutingIssue::RegisteredSuccessorsOutOfOrder {
                family: "function.wrapper.pipeline.beta.v1".to_string(),
                expected: vec![
                    "function.wrapper.pipeline.alpha.v1".to_string(),
                    "function.wrapper.pipeline.gamma.v1".to_string(),
                ],
                found: vec![
                    "function.wrapper.pipeline.gamma.v1".to_string(),
                    "function.wrapper.pipeline.alpha.v1".to_string(),
                ],
            }
        ));
    }

    #[test]
    fn synthetic_registry_routing_diagnostics_reject_unsupported_before_registered_successor() {
        let beta = FamilyId::parse("function.wrapper.pipeline.beta.v1").unwrap();
        let registry = [
            FamilyHarness {
                routing: LockedManifestRouting {
                    must_not_shadow: &SYNTHETIC_BETA_UNSUPPORTED_BEFORE_GAMMA_MUST_NOT_SHADOW,
                    ..SYNTHETIC_BETA_HARNESS.routing
                },
                ..SYNTHETIC_BETA_HARNESS
            },
            SYNTHETIC_ALPHA_HARNESS,
            SYNTHETIC_GAMMA_HARNESS,
        ];
        let routing = Routing {
            precedence: SYNTHETIC_BETA_HARNESS.routing.precedence,
            must_not_shadow: SYNTHETIC_BETA_UNSUPPORTED_BEFORE_GAMMA_MUST_NOT_SHADOW
                .iter()
                .map(|family_id| (*family_id).to_string())
                .collect(),
        };

        let diagnostics = routing_diagnostics_in(&registry, &beta, &routing);

        assert!(diagnostics.registry.issues.contains(
            &RegistryRoutingIssue::UnsupportedBeforeRegisteredSuccessor {
                family: "function.wrapper.pipeline.beta.v1".to_string(),
            }
        ));
    }

    #[test]
    fn synthetic_registry_routing_diagnostics_reject_duplicate_unsupported_terminal() {
        let beta = FamilyId::parse("function.wrapper.pipeline.beta.v1").unwrap();
        let registry = [
            FamilyHarness {
                routing: LockedManifestRouting {
                    must_not_shadow: &SYNTHETIC_BETA_DUPLICATE_UNSUPPORTED_MUST_NOT_SHADOW,
                    ..SYNTHETIC_BETA_HARNESS.routing
                },
                ..SYNTHETIC_BETA_HARNESS
            },
            SYNTHETIC_ALPHA_HARNESS,
            SYNTHETIC_GAMMA_HARNESS,
        ];
        let routing = Routing {
            precedence: SYNTHETIC_BETA_HARNESS.routing.precedence,
            must_not_shadow: SYNTHETIC_BETA_DUPLICATE_UNSUPPORTED_MUST_NOT_SHADOW
                .iter()
                .map(|family_id| (*family_id).to_string())
                .collect(),
        };

        let diagnostics = routing_diagnostics_in(&registry, &beta, &routing);

        assert!(diagnostics.registry.issues.contains(
            &RegistryRoutingIssue::DuplicateUnsupportedTerminal {
                family: "function.wrapper.pipeline.beta.v1".to_string(),
            }
        ));
    }

    #[test]
    fn family_new_rejects_invalid_family_id_without_writes() {
        let temp_dir = workspace_root();
        let code = run_from(temp_dir.path(), ["xtask", "family", "new", "../bad"]);

        assert_eq!(code, 2);
        assert!(fs::read_dir(temp_dir.path().join("semantic-families"))
            .unwrap()
            .next()
            .is_none());
    }

    #[test]
    fn family_new_rejects_existing_packet() {
        let temp_dir = workspace_root();
        let family = FamilyId::parse("function.wrapper.pipeline.chain3.v1").unwrap();
        let paths = PacketPaths::new(temp_dir.path(), family);
        fs::create_dir(paths.root).unwrap();

        let code = run_from(
            temp_dir.path(),
            [
                "xtask",
                "family",
                "new",
                "function.wrapper.pipeline.chain3.v1",
            ],
        );

        assert_eq!(code, 3);
    }

    #[cfg(unix)]
    #[test]
    fn family_new_rejects_packet_root_symlink() {
        use std::os::unix::fs::symlink;

        let temp_dir = workspace_root();
        let family = FamilyId::parse("function.wrapper.pipeline.chain3.v1").unwrap();
        let paths = PacketPaths::new(temp_dir.path(), family);
        let outside = TempDir::new().unwrap();
        symlink(outside.path(), &paths.root).unwrap();

        let code = run_from(
            temp_dir.path(),
            [
                "xtask",
                "family",
                "new",
                "function.wrapper.pipeline.chain3.v1",
            ],
        );

        assert_eq!(code, 2);
    }

    #[test]
    fn manifest_validation_accepts_locked_example() {
        let temp_dir = workspace_root();
        let family = FamilyId::parse("function.wrapper.pipeline.chain3.v1").unwrap();
        let paths = PacketPaths::new(temp_dir.path(), family.clone());
        write_string(
            &paths.root.join("family.toml"),
            r#"schema_version = 2
family = "function.wrapper.pipeline.chain3.v1"
kind = "function"
compatibility_key = "function.wrapper.pipeline.chain3.v1"
summary = "Straight-line three-call wrapper pipeline over supported function deps."

[routing]
precedence = 1
must_not_shadow = [
  "function.wrapper.pipeline.v1",
  "function.arithmetic_leaf.monotone_down_nonnegative.v1",
  "function.arithmetic_leaf.monotone_up.v1",
]

[shape]
dep_min = 3
dep_max = 3
control_flow = "straight_line_only"
return_style = "let_then_return_or_direct_return"
loops = false
branching = false
requires_supported_function_deps = true

[args]
threading = "ordered_passthrough"
allow_nested_argument_expressions = false
allow_literal_only_extra_args = false

[corpus]
required_buckets = ["aligned", "drift", "under_specified", "unsupported_near_miss"]
min_cases_per_bucket = 1

[truth_surface]
requires_refresh_via = ["spec test"]
preserve_only_via = ["spec build", "spec generate", "spec status", "spec export"]
requires_stale_demote = true

[gates]
gate_a = true
gate_b = true
gate_c = true
gate_d = true
"#,
        );

        parse_manifest_file(&paths.manifest, &family, family_harness(&family).unwrap()).unwrap();
    }

    #[test]
    fn manifest_validation_rejects_extra_top_level_keys() {
        let temp_dir = workspace_root();
        let family = FamilyId::parse("function.wrapper.pipeline.chain3.v1").unwrap();
        let paths = PacketPaths::new(temp_dir.path(), family.clone());
        write_string(
            &paths.root.join("family.toml"),
            r#"schema_version = 2
family = "function.wrapper.pipeline.chain3.v1"
kind = "function"
compatibility_key = "function.wrapper.pipeline.chain3.v1"
summary = "summary"
unexpected = true

[routing]
precedence = 1
must_not_shadow = ["function.wrapper.pipeline.v1"]

[shape]
dep_min = 3
dep_max = 3
control_flow = "straight_line_only"
return_style = "direct_return"
loops = false
branching = false
requires_supported_function_deps = true

[args]
threading = "ordered_passthrough"
allow_nested_argument_expressions = false
allow_literal_only_extra_args = false

[corpus]
required_buckets = ["aligned", "drift", "under_specified", "unsupported_near_miss"]
min_cases_per_bucket = 1

[truth_surface]
requires_refresh_via = ["spec test"]
preserve_only_via = ["spec build", "spec generate", "spec status", "spec export"]
requires_stale_demote = true

[gates]
gate_a = true
gate_b = true
gate_c = true
gate_d = true
"#,
        );

        let error = parse_manifest_file(&paths.manifest, &family, family_harness(&family).unwrap())
            .unwrap_err();
        assert!(matches!(error, XtaskError::InvalidInput(_)));
    }

    #[test]
    fn manifest_validation_rejects_wrong_bucket_contract() {
        let temp_dir = workspace_root();
        let family = FamilyId::parse("function.wrapper.pipeline.chain3.v1").unwrap();
        let paths = PacketPaths::new(temp_dir.path(), family.clone());
        write_string(
            &paths.root.join("family.toml"),
            r#"schema_version = 2
family = "function.wrapper.pipeline.chain3.v1"
kind = "function"
compatibility_key = "function.wrapper.pipeline.chain3.v1"
summary = "summary"

[routing]
precedence = 1
must_not_shadow = ["function.wrapper.pipeline.v1"]

[shape]
dep_min = 3
dep_max = 3
control_flow = "straight_line_only"
return_style = "direct_return"
loops = false
branching = false
requires_supported_function_deps = true

[args]
threading = "ordered_passthrough"
allow_nested_argument_expressions = false
allow_literal_only_extra_args = false

[corpus]
required_buckets = ["aligned", "drift"]
min_cases_per_bucket = 1

[truth_surface]
requires_refresh_via = ["spec test"]
preserve_only_via = ["spec build", "spec generate", "spec status", "spec export"]
requires_stale_demote = true

[gates]
gate_a = true
gate_b = true
gate_c = true
gate_d = true
"#,
        );

        let error = parse_manifest_file(&paths.manifest, &family, family_harness(&family).unwrap())
            .unwrap_err();
        assert!(matches!(error, XtaskError::InvalidInput(_)));
    }

    #[test]
    fn packet_layout_validation_accepts_locked_shape() {
        let temp_dir = workspace_root();
        let family = FamilyId::parse("function.wrapper.pipeline.chain3.v1").unwrap();
        let paths = PacketPaths::new(temp_dir.path(), family.clone());
        scaffold::run(temp_dir.path(), family.as_str()).unwrap();
        seed_valid_manifest(&paths.manifest, family.as_str());
        seed_valid_cases(&paths);

        let harness = family_harness(&family).unwrap();
        let manifest = parse_manifest_file(&paths.manifest, &family, harness).unwrap();
        let layout = validate_packet_layout(&paths.root, &manifest, harness).unwrap();

        assert_eq!(layout.case_filenames.len(), 16);
    }

    #[test]
    fn packet_layout_validation_rejects_non_unit_spec_files() {
        let temp_dir = workspace_root();
        let family = FamilyId::parse("function.wrapper.pipeline.chain3.v1").unwrap();
        let paths = PacketPaths::new(temp_dir.path(), family.clone());
        scaffold::run(temp_dir.path(), family.as_str()).unwrap();
        seed_valid_manifest(&paths.manifest, family.as_str());
        seed_valid_cases(&paths);
        write_string(
            &paths.fixtures.join("aligned/units/pricing/not-allowed.txt"),
            "bad",
        );

        let harness = family_harness(&family).unwrap();
        let manifest = parse_manifest_file(&paths.manifest, &family, harness).unwrap();
        let error = validate_packet_layout(&paths.root, &manifest, harness).unwrap_err();
        assert!(matches!(error, XtaskError::InvalidInput(_)));
    }

    #[cfg(unix)]
    #[test]
    fn packet_layout_validation_rejects_symlinks_anywhere_under_fixtures() {
        use std::os::unix::fs::symlink;

        let temp_dir = workspace_root();
        let family = FamilyId::parse("function.wrapper.pipeline.chain3.v1").unwrap();
        let paths = PacketPaths::new(temp_dir.path(), family.clone());
        scaffold::run(temp_dir.path(), family.as_str()).unwrap();
        seed_valid_manifest(&paths.manifest, family.as_str());
        seed_valid_cases(&paths);
        let outside = TempDir::new().unwrap();
        symlink(outside.path(), paths.fixtures.join("drift/src/linked")).unwrap();

        let harness = family_harness(&family).unwrap();
        let manifest = parse_manifest_file(&paths.manifest, &family, harness).unwrap();
        let error = validate_packet_layout(&paths.root, &manifest, harness).unwrap_err();
        assert!(matches!(error, XtaskError::InvalidInput(_)));
    }

    #[test]
    fn packet_layout_validation_rejects_duplicate_case_filenames() {
        let temp_dir = workspace_root();
        let family = FamilyId::parse("function.wrapper.pipeline.chain3.v1").unwrap();
        let paths = PacketPaths::new(temp_dir.path(), family.clone());
        scaffold::run(temp_dir.path(), family.as_str()).unwrap();
        seed_valid_manifest(&paths.manifest, family.as_str());
        seed_valid_cases(&paths);
        let original = paths
            .fixtures
            .join("aligned/units/pricing/checkout_chain3_aligned.unit.spec");
        let duplicate = paths
            .fixtures
            .join("aligned/units/bonus/checkout_chain3_aligned.unit.spec");
        write_string(&duplicate, &fs::read_to_string(&original).unwrap());

        let harness = family_harness(&family).unwrap();
        let manifest = parse_manifest_file(&paths.manifest, &family, harness).unwrap();
        let error = validate_packet_layout(&paths.root, &manifest, harness).unwrap_err();
        assert!(matches!(error, XtaskError::InvalidInput(_)));
    }

    #[test]
    fn packet_layout_validation_allows_helper_units_without_bucket_suffix() {
        let temp_dir = workspace_root();
        let family = FamilyId::parse("function.wrapper.pipeline.chain3.v1").unwrap();
        let paths = PacketPaths::new(temp_dir.path(), family.clone());
        scaffold::run(temp_dir.path(), family.as_str()).unwrap();
        seed_valid_manifest(&paths.manifest, family.as_str());
        seed_valid_cases(&paths);
        for bucket in REQUIRED_BUCKETS {
            write_string(
                &paths
                    .fixtures
                    .join(bucket)
                    .join("units/money/round.unit.spec"),
                "kind: function\n",
            );
        }

        let harness = family_harness(&family).unwrap();
        let manifest = parse_manifest_file(&paths.manifest, &family, harness).unwrap();
        let layout = validate_packet_layout(&paths.root, &manifest, harness).unwrap();
        assert_eq!(layout.case_filenames.len(), 16);
    }

    #[test]
    fn packet_layout_validation_rejects_hollow_packet_missing_locked_starter_case() {
        let temp_dir = workspace_root();
        let family = FamilyId::parse("function.wrapper.pipeline.chain3.v1").unwrap();
        let paths = PacketPaths::new(temp_dir.path(), family.clone());
        scaffold::run(temp_dir.path(), family.as_str()).unwrap();
        seed_valid_manifest(&paths.manifest, family.as_str());
        seed_valid_cases(&paths);
        fs::remove_file(
            paths
                .fixtures
                .join("aligned/units/pricing/checkout_chain3_aligned.unit.spec"),
        )
        .unwrap();
        write_string(
            &paths
                .fixtures
                .join("aligned/units/pricing/hollow_aligned.unit.spec"),
            "kind: function\n",
        );

        let harness = family_harness(&family).unwrap();
        let manifest = parse_manifest_file(&paths.manifest, &family, harness).unwrap();
        let error = validate_packet_layout(&paths.root, &manifest, harness).unwrap_err();
        assert!(matches!(error, XtaskError::InvalidInput(message)
            if message.contains("locked starter case")));
    }

    #[test]
    fn family_prove_writes_locked_report() {
        let temp_dir = workspace_root();
        let family = FamilyId::parse("function.wrapper.pipeline.chain3.v1").unwrap();
        let paths = PacketPaths::new(temp_dir.path(), family.clone());
        scaffold::run(temp_dir.path(), family.as_str()).unwrap();
        seed_valid_manifest(&paths.manifest, family.as_str());
        seed_valid_cases(&paths);
        seed_suite_sources(temp_dir.path(), false);

        let runner = prove_runner_with_suite_outputs(&[
            normalized_libtest_stdout(&expected_suite_test_names(CHAIN3_PROVE_SUITES[0])),
            normalized_libtest_stdout(&expected_suite_test_names(CHAIN3_PROVE_SUITES[1])),
            normalized_libtest_stdout(&expected_suite_test_names(CHAIN3_PROVE_SUITES[2])),
        ]);

        prove::run_with_runner(temp_dir.path(), family.as_str(), &runner).unwrap();

        let report_path = paths.artifacts.join(PROVE_ARTIFACT_NAME);
        let report = read_report(&report_path);
        assert_artifact_surface_matches_captured_chain3_baseline(
            &report_path,
            &report,
            PROVE_ARTIFACT_NAME,
        );
        assert_eq!(report["schema_version"], 3);
        assert_required_gates(&report, &["A", "B", "C"]);
        assert_eq!(report["phase_status"], "pass");
        assert_eq!(report["overall_status"], "pass");
        assert_eq!(report["gates"]["gate_a"]["status"], "pass");
        assert_eq!(report["gates"]["gate_b"]["status"], "pass");
        assert_eq!(report["gates"]["gate_c"]["status"], "pass");
        assert_eq!(report["gates"]["gate_d"]["status"], "fail");
        assert_eq!(report["suites"].as_array().unwrap().len(), 3);
        assert_report_status_invariants(&report);
    }

    #[test]
    fn family_prove_requires_attested_stdout_and_keeps_gate_mapping_explicit() {
        for (failing_index, failing_gate) in [(0, "gate_a"), (1, "gate_c"), (2, "gate_b")] {
            let temp_dir = workspace_root();
            let family = FamilyId::parse("function.wrapper.pipeline.chain3.v1").unwrap();
            let paths = PacketPaths::new(temp_dir.path(), family.clone());
            scaffold::run(temp_dir.path(), family.as_str()).unwrap();
            seed_valid_manifest(&paths.manifest, family.as_str());
            seed_valid_cases(&paths);
            seed_suite_sources(temp_dir.path(), false);

            let mut suite_outputs = vec![
                normalized_libtest_stdout(&expected_suite_test_names(CHAIN3_PROVE_SUITES[0])),
                normalized_libtest_stdout(&expected_suite_test_names(CHAIN3_PROVE_SUITES[1])),
                normalized_libtest_stdout(&expected_suite_test_names(CHAIN3_PROVE_SUITES[2])),
            ];
            suite_outputs[failing_index] = String::new();

            let runner = prove_runner_with_suite_outputs(&suite_outputs);
            let error =
                prove::run_with_runner(temp_dir.path(), family.as_str(), &runner).unwrap_err();
            assert!(matches!(error, XtaskError::ProveSuiteFailure(_)));

            let report = read_report(&paths.artifacts.join(PROVE_ARTIFACT_NAME));
            assert_eq!(report["schema_version"], 3);
            assert_eq!(report["phase_status"], "fail");
            assert_eq!(report["overall_status"], "fail");
            assert_eq!(report["gates"][failing_gate]["status"], "fail");
            for gate in ["gate_a", "gate_b", "gate_c"] {
                if gate != failing_gate {
                    assert_eq!(report["gates"][gate]["status"], "pass");
                }
            }
            assert_eq!(report["gates"]["gate_d"]["status"], "fail");
            assert_report_status_invariants(&report);
        }
    }

    #[test]
    fn family_certify_writes_success_report_only_on_full_success() {
        let temp_dir = workspace_root();
        let family = FamilyId::parse("function.wrapper.pipeline.chain3.v1").unwrap();
        let paths = PacketPaths::new(temp_dir.path(), family.clone());
        scaffold::run(temp_dir.path(), family.as_str()).unwrap();
        seed_valid_manifest(&paths.manifest, family.as_str());
        seed_valid_cases(&paths);
        seed_suite_sources(temp_dir.path(), true);

        let runner = success_certify_runner();

        certify::run_with_runner(temp_dir.path(), family.as_str(), &runner).unwrap();

        let certification_report = certification_report_path(&paths);
        let report = read_report(&certification_report);
        assert_artifact_surface_matches_captured_chain3_baseline(
            &certification_report,
            &report,
            CERTIFY_ARTIFACT_NAME,
        );
        assert_eq!(report["schema_version"], 3);
        assert_required_gates(&report, &["A", "B", "C", "D"]);
        assert_eq!(report["phase_status"], "pass");
        assert_eq!(report["overall_status"], "pass");
        assert_eq!(report["gates"]["gate_d"]["status"], "pass");
        assert_eq!(report["suites"].as_array().unwrap().len(), 5);
        assert_report_status_invariants(&report);
        let attempts = attempt_reports(&paths);
        assert_eq!(attempts.len(), 1);
        assert_attempt_artifact_name_matches_captured_baseline(&attempts[0]);
        let attempt = read_report(&attempts[0]);
        assert_artifact_surface_matches_captured_chain3_baseline(
            &attempts[0],
            &attempt,
            attempts[0]
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap(),
        );
        assert_eq!(attempt["schema_version"], 3);
        assert_required_gates(&attempt, &["A", "B", "C", "D"]);
        assert_report_status_invariants(&attempt);
    }

    #[test]
    fn family_certify_success_rewrites_prove_latest_with_truthful_v3_surface() {
        let temp_dir = workspace_root();
        let family = FamilyId::parse("function.wrapper.pipeline.chain3.v1").unwrap();
        let paths = PacketPaths::new(temp_dir.path(), family.clone());
        scaffold::run(temp_dir.path(), family.as_str()).unwrap();
        seed_valid_manifest(&paths.manifest, family.as_str());
        seed_valid_cases(&paths);
        seed_suite_sources(temp_dir.path(), true);

        let runner = success_certify_runner();

        certify::run_with_runner(temp_dir.path(), family.as_str(), &runner).unwrap();

        let prove_report = read_report(&paths.artifacts.join(PROVE_ARTIFACT_NAME));
        assert_eq!(prove_report["schema_version"], 3);
        assert_required_gates(&prove_report, &["A", "B", "C"]);
        assert_eq!(prove_report["phase_status"], "pass");
        assert_eq!(prove_report["overall_status"], "pass");
        assert_eq!(prove_report["gates"]["gate_d"]["status"], "fail");

        let attempt = read_report(&attempt_reports(&paths)[0]);
        assert_eq!(attempt["schema_version"], 3);
        let certification = read_report(&certification_report_path(&paths));
        assert_eq!(certification["schema_version"], 3);
    }

    #[test]
    fn scaffold_manifest_routing_matches_selected_registered_family_values() {
        let temp_dir = workspace_root();
        let family = FamilyId::parse("function.wrapper.pipeline.chain3.v1").unwrap();
        let expected = locked_manifest_routing_in(
            &[*family_harness(&family).expect("chain3 harness should be registered")],
            &family,
        )
        .expect("chain3 locked routing should exist");

        scaffold::run(temp_dir.path(), family.as_str()).unwrap();

        let manifest = parse_manifest_file(
            &PacketPaths::new(temp_dir.path(), family.clone()).manifest,
            &family,
            family_harness(&family).unwrap(),
        )
        .unwrap();
        assert_eq!(manifest.routing.precedence, expected.precedence);
        assert_eq!(
            manifest.routing.must_not_shadow,
            expected
                .must_not_shadow
                .iter()
                .map(|family| family.to_string())
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn family_certify_manifest_local_routing_mismatch_fails_gate_d_without_registry_failure() {
        let temp_dir = workspace_root();
        let family = FamilyId::parse("function.wrapper.pipeline.chain3.v1").unwrap();
        let paths = PacketPaths::new(temp_dir.path(), family.clone());
        scaffold::run(temp_dir.path(), family.as_str()).unwrap();
        seed_valid_manifest(&paths.manifest, family.as_str());
        seed_valid_cases(&paths);
        rewrite_manifest(&paths.manifest, "precedence = 1", "precedence = 2");
        seed_suite_sources(temp_dir.path(), true);

        let cert_report_path = certification_report_path(&paths);
        write_string(&cert_report_path, "{\"previous\":true}\n");

        let runner = success_certify_runner();
        let error =
            certify::run_with_runner(temp_dir.path(), family.as_str(), &runner).unwrap_err();

        assert!(matches!(error, XtaskError::CertifySuiteFailure(message)
                if message.contains("manifest-local routing mismatch")
                    && !message.contains("registry-global routing incoherence")));
        assert_eq!(
            fs::read_to_string(&cert_report_path).unwrap(),
            "{\"previous\":true}\n"
        );
        let attempts = attempt_reports(&paths);
        assert_eq!(attempts.len(), 1);
        let attempt = read_report(&attempts[0]);
        assert_eq!(attempt["schema_version"], 3);
        assert_required_gates(&attempt, &["A", "B", "C", "D"]);
        assert_eq!(attempt["overall_status"], "fail");
        assert_eq!(attempt["gates"]["gate_d"]["status"], "fail");
        let suites = attempt["suites"].as_array().unwrap();
        assert_eq!(suites.len(), 5);
        assert!(suites.iter().all(|suite| suite["status"] == "pass"));
        assert_report_status_invariants(&attempt);
    }

    #[test]
    fn family_certify_missing_shadow_entry_fails_gate_d_without_failing_suites() {
        let temp_dir = workspace_root();
        let family = FamilyId::parse("function.wrapper.pipeline.chain3.v1").unwrap();
        let paths = PacketPaths::new(temp_dir.path(), family.clone());
        scaffold::run(temp_dir.path(), family.as_str()).unwrap();
        seed_valid_manifest(&paths.manifest, family.as_str());
        seed_valid_cases(&paths);
        rewrite_manifest(
            &paths.manifest,
            "  \"function.arithmetic_leaf.monotone_up.v1\",\n",
            "",
        );
        seed_suite_sources(temp_dir.path(), true);

        let cert_report_path = certification_report_path(&paths);
        write_string(&cert_report_path, "{\"previous\":true}\n");

        let runner = success_certify_runner();
        let error =
            certify::run_with_runner(temp_dir.path(), family.as_str(), &runner).unwrap_err();

        assert!(matches!(error, XtaskError::CertifySuiteFailure(message)
                if message.contains("manifest-local routing mismatch")
                    && !message.contains("registry-global routing incoherence")));
        assert_eq!(
            fs::read_to_string(&cert_report_path).unwrap(),
            "{\"previous\":true}\n"
        );
        let attempts = attempt_reports(&paths);
        assert_eq!(attempts.len(), 1);
        let attempt = read_report(&attempts[0]);
        assert_eq!(attempt["schema_version"], 3);
        assert_required_gates(&attempt, &["A", "B", "C", "D"]);
        assert_eq!(attempt["overall_status"], "fail");
        assert_eq!(attempt["gates"]["gate_d"]["status"], "fail");
        let suites = attempt["suites"].as_array().unwrap();
        assert_eq!(suites.len(), 5);
        assert!(suites.iter().all(|suite| suite["status"] == "pass"));
        assert_report_status_invariants(&attempt);
    }

    #[test]
    fn family_certify_registry_global_routing_incoherence_fails_gate_d_without_manifest_mismatch() {
        let temp_dir = workspace_root();
        let family = FamilyId::parse("function.wrapper.pipeline.chain3.v1").unwrap();
        let paths = PacketPaths::new(temp_dir.path(), family.clone());
        let chain3 = *family_harness(&family).unwrap();
        let registry = [
            chain3,
            SYNTHETIC_BETA_HARNESS,
            SYNTHETIC_ALPHA_HARNESS,
            SYNTHETIC_GAMMA_HARNESS,
        ];
        scaffold::run(temp_dir.path(), family.as_str()).unwrap();
        seed_valid_manifest(&paths.manifest, family.as_str());
        seed_valid_cases(&paths);
        seed_suite_sources(temp_dir.path(), true);

        let cert_report_path = certification_report_path(&paths);
        write_string(&cert_report_path, "{\"previous\":true}\n");

        let runner = success_certify_runner();
        let error =
            certify::run_with_runner_in(&registry, temp_dir.path(), family.as_str(), &runner)
                .unwrap_err();

        assert!(matches!(error, XtaskError::CertifySuiteFailure(message)
                if message.contains("registry-global routing incoherence")
                    && !message.contains("manifest-local routing mismatch")));
        assert_eq!(
            fs::read_to_string(&cert_report_path).unwrap(),
            "{\"previous\":true}\n"
        );
        let attempts = attempt_reports(&paths);
        assert_eq!(attempts.len(), 1);
        let attempt = read_report(&attempts[0]);
        assert_eq!(attempt["schema_version"], 3);
        assert_required_gates(&attempt, &["A", "B", "C", "D"]);
        assert_eq!(attempt["gates"]["gate_d"]["status"], "fail");
        let suites = attempt["suites"].as_array().unwrap();
        assert_eq!(suites.len(), 5);
        assert!(suites.iter().all(|suite| suite["status"] == "pass"));
    }

    #[test]
    fn family_certify_combined_manifest_and_registry_routing_failures_report_both_scopes() {
        let temp_dir = workspace_root();
        let family = FamilyId::parse("function.wrapper.pipeline.chain3.v1").unwrap();
        let paths = PacketPaths::new(temp_dir.path(), family.clone());
        let chain3 = *family_harness(&family).unwrap();
        let registry = [
            chain3,
            SYNTHETIC_BETA_HARNESS,
            SYNTHETIC_ALPHA_HARNESS,
            SYNTHETIC_GAMMA_HARNESS,
        ];
        scaffold::run(temp_dir.path(), family.as_str()).unwrap();
        seed_valid_manifest(&paths.manifest, family.as_str());
        seed_valid_cases(&paths);
        rewrite_manifest(&paths.manifest, "precedence = 1", "precedence = 2");
        seed_suite_sources(temp_dir.path(), true);

        let cert_report_path = certification_report_path(&paths);
        write_string(&cert_report_path, "{\"previous\":true}\n");

        let runner = success_certify_runner();
        let error =
            certify::run_with_runner_in(&registry, temp_dir.path(), family.as_str(), &runner)
                .unwrap_err();

        assert!(matches!(error, XtaskError::CertifySuiteFailure(message)
                if message.contains("manifest-local routing mismatch")
                    && message.contains("registry-global routing incoherence")));
        assert_eq!(
            fs::read_to_string(&cert_report_path).unwrap(),
            "{\"previous\":true}\n"
        );
        let attempts = attempt_reports(&paths);
        assert_eq!(attempts.len(), 1);
        let attempt = read_report(&attempts[0]);
        assert_eq!(attempt["schema_version"], 3);
        assert_required_gates(&attempt, &["A", "B", "C", "D"]);
        assert_eq!(attempt["gates"]["gate_d"]["status"], "fail");
        let suites = attempt["suites"].as_array().unwrap();
        assert_eq!(suites.len(), 5);
        assert!(suites.iter().all(|suite| suite["status"] == "pass"));
    }

    #[test]
    fn family_certify_keeps_previous_success_report_on_failed_regression_suite() {
        let temp_dir = workspace_root();
        let family = FamilyId::parse("function.wrapper.pipeline.chain3.v1").unwrap();
        let paths = PacketPaths::new(temp_dir.path(), family.clone());
        scaffold::run(temp_dir.path(), family.as_str()).unwrap();
        seed_valid_manifest(&paths.manifest, family.as_str());
        seed_valid_cases(&paths);
        seed_suite_sources(temp_dir.path(), true);

        let success_runner = success_certify_runner();
        certify::run_with_runner(temp_dir.path(), family.as_str(), &success_runner).unwrap();

        let cert_report_path = certification_report_path(&paths);
        let previous_bytes = fs::read(&cert_report_path).unwrap();
        let attempts_before = attempt_reports(&paths);
        assert_eq!(attempts_before.len(), 1);

        let failing_runner = certify_runner_with_outputs(
            &[
                normalized_libtest_stdout(&expected_suite_test_names(CHAIN3_PROVE_SUITES[0])),
                normalized_libtest_stdout(&expected_suite_test_names(CHAIN3_PROVE_SUITES[1])),
                normalized_libtest_stdout(&expected_suite_test_names(CHAIN3_PROVE_SUITES[2])),
                normalized_libtest_stdout(&expected_suite_test_names(CHAIN3_CERTIFY_SUITES[0])),
                String::new(),
            ],
            "2026-04-27T18:20:00Z\n",
        );

        let error = certify::run_with_runner(temp_dir.path(), family.as_str(), &failing_runner)
            .unwrap_err();
        assert!(matches!(error, XtaskError::CertifySuiteFailure(_)));
        assert_eq!(fs::read(&cert_report_path).unwrap(), previous_bytes);
        let attempts_after = attempt_reports(&paths);
        assert_eq!(attempts_after.len(), 2);
        let new_attempts = attempts_after
            .iter()
            .filter(|path| !attempts_before.contains(path))
            .cloned()
            .collect::<Vec<_>>();
        assert_eq!(new_attempts.len(), 1);
        let failed_attempt = read_report(&new_attempts[0]);
        assert_required_gates(&failed_attempt, &["A", "B", "C", "D"]);
        assert_eq!(failed_attempt["phase_status"], "fail");
        assert_eq!(failed_attempt["overall_status"], "fail");
        assert_report_status_invariants(&failed_attempt);
        assert!(!paths
            .artifacts
            .join(CERTIFY_ARTIFACT_NAME)
            .with_extension("tmp")
            .exists());
    }

    #[test]
    fn suite_attestation_accepts_only_normalized_non_colored_libtest_output() {
        let suite = CHAIN3_PROVE_SUITES[0];
        let temp_dir = TempDir::new().unwrap();
        let names = expected_suite_test_names(suite);

        let success_runner = FakeRunner::new(&[suite_command_output(
            suite,
            0,
            &normalized_libtest_stdout(&names),
        )]);
        let success = run_suite(temp_dir.path(), &success_runner, suite);
        assert_eq!(success.status, PassFail::Pass);

        let colorized_runner = FakeRunner::new(&[suite_command_output(
            suite,
            0,
            &colorized_libtest_stdout(&names),
        )]);
        let colorized = run_suite(temp_dir.path(), &colorized_runner, suite);
        assert_eq!(colorized.status, PassFail::Fail);

        let variant_runner = FakeRunner::new(&[suite_command_output(
            suite,
            0,
            &format_variant_libtest_stdout(&names),
        )]);
        let variant = run_suite(temp_dir.path(), &variant_runner, suite);
        assert_eq!(variant.status, PassFail::Fail);
    }

    #[test]
    fn suite_attestation_rejects_empty_stdout_even_with_zero_exit() {
        let suite = CHAIN3_PROVE_SUITES[0];
        let temp_dir = TempDir::new().unwrap();
        let runner = FakeRunner::new(&[suite_command_output(suite, 0, "")]);
        let report = run_suite(temp_dir.path(), &runner, suite);
        assert_eq!(report.status, PassFail::Fail);
    }

    #[test]
    fn suite_attestation_rejects_zero_match_output() {
        let suite = CHAIN3_PROVE_SUITES[0];
        let temp_dir = TempDir::new().unwrap();
        let runner = FakeRunner::new(&[suite_command_output(
            suite,
            0,
            &normalized_libtest_stdout(&Vec::<String>::new()),
        )]);
        let report = run_suite(temp_dir.path(), &runner, suite);
        assert_eq!(report.status, PassFail::Fail);
    }

    #[test]
    fn suite_attestation_rejects_missing_expected_tests() {
        let suite = CHAIN3_PROVE_SUITES[0];
        let temp_dir = TempDir::new().unwrap();
        let names = expected_suite_test_names(suite);
        let runner = FakeRunner::new(&[suite_command_output(
            suite,
            0,
            &normalized_libtest_stdout(&names[..names.len() - 1]),
        )]);
        let report = run_suite(temp_dir.path(), &runner, suite);
        assert_eq!(report.status, PassFail::Fail);
    }

    #[test]
    fn suite_attestation_rejects_extra_matched_tests() {
        let suite = CHAIN3_PROVE_SUITES[0];
        let temp_dir = TempDir::new().unwrap();
        let mut names = expected_suite_test_names(suite);
        names.push("semantic_review::tests::m21_chain3_classifier_fake_extra".to_string());
        let runner = FakeRunner::new(&[suite_command_output(
            suite,
            0,
            &normalized_libtest_stdout(&names),
        )]);
        let report = run_suite(temp_dir.path(), &runner, suite);
        assert_eq!(report.status, PassFail::Fail);
    }

    #[test]
    fn suite_attestation_rejects_ignored_tests() {
        let suite = CHAIN3_PROVE_SUITES[0];
        let temp_dir = TempDir::new().unwrap();
        let names = expected_suite_test_names(suite);
        let runner = FakeRunner::new(&[suite_command_output(
            suite,
            0,
            &ignored_libtest_stdout(&names[0]),
        )]);
        let report = run_suite(temp_dir.path(), &runner, suite);
        assert_eq!(report.status, PassFail::Fail);
    }

    #[test]
    fn suite_attestation_rejects_non_zero_exit() {
        let suite = CHAIN3_PROVE_SUITES[0];
        let temp_dir = TempDir::new().unwrap();
        let names = expected_suite_test_names(suite);
        let runner = FakeRunner::new(&[suite_command_output(
            suite,
            101,
            &normalized_libtest_stdout(&names),
        )]);
        let report = run_suite(temp_dir.path(), &runner, suite);
        assert_eq!(report.status, PassFail::Fail);
    }

    #[test]
    fn suite_attestation_rejects_fake_printed_test_line_without_libtest_surface() {
        let suite = CHAIN3_PROVE_SUITES[0];
        let temp_dir = TempDir::new().unwrap();
        let names = expected_suite_test_names(suite);
        let runner = FakeRunner::new(&[suite_command_output(
            suite,
            0,
            &format!("test {} ... ok\n", names[0]),
        )]);
        let report = run_suite(temp_dir.path(), &runner, suite);
        assert_eq!(report.status, PassFail::Fail);
    }

    #[test]
    fn family_inventory_reports_locked_promoted_and_supported_unpromoted_families() {
        let temp_dir = workspace_root();
        seed_inventory_repo_truth(temp_dir.path());

        let inventory =
            inventory::collect_inventory_in(&pre_wrapper_promotion_registry(), temp_dir.path())
                .unwrap();

        assert_eq!(inventory.schema_version, 1);
        assert_eq!(inventory.generated_at, "1970-01-01T00:00:00Z");
        assert_eq!(
            inventory.promoted_families,
            vec![
                "function.wrapper.pipeline.chain3.v1",
                "function.arithmetic_leaf.monotone_down_nonnegative.v1",
                "function.arithmetic_leaf.monotone_up.v1",
            ]
        );
        assert_eq!(
            inventory.runtime_supported_routes,
            vec![
                "function.wrapper.pipeline.chain3.v1",
                "function.wrapper.pipeline.v1",
                "function.arithmetic_leaf.monotone_down_nonnegative.v1",
                "function.arithmetic_leaf.monotone_up.v1",
            ]
        );
        assert_eq!(inventory.supported_unpromoted_families.len(), 1);

        let wrapper = inventory.supported_unpromoted_families.first().unwrap();
        assert_eq!(wrapper.family, "function.wrapper.pipeline.v1");
        assert_eq!(
            wrapper.routing_predecessor.as_deref(),
            Some("function.wrapper.pipeline.chain3.v1")
        );
        assert_eq!(
            wrapper.routing_successors,
            vec![
                "function.arithmetic_leaf.monotone_down_nonnegative.v1",
                "function.arithmetic_leaf.monotone_up.v1",
                "unsupported.function.v1",
            ]
        );
        assert_eq!(
            wrapper.canonical_seed_paths,
            vec!["examples/ecommerce/units/pricing/calculate_total.unit.spec"]
        );
        assert_eq!(
            wrapper.existing_wedge_paths,
            vec!["spec-cli/tests/m14_regressions.rs"]
        );
        assert_eq!(wrapper.supporting_packet_paths.len(), 6);
    }

    #[test]
    fn family_inventory_snapshot_bytes_are_stable_and_hash_exact_stdout_bytes() {
        let temp_dir = workspace_root();
        seed_inventory_repo_truth(temp_dir.path());

        let first =
            inventory::render_snapshot_bytes_in(&pre_wrapper_promotion_registry(), temp_dir.path())
                .unwrap();
        let second =
            inventory::render_snapshot_bytes_in(&pre_wrapper_promotion_registry(), temp_dir.path())
                .unwrap();

        assert_eq!(first, second);
        assert!(first.ends_with(b"\n"));
        assert!(!first.ends_with(b"\n\n"));

        let rendered: serde_json::Value = serde_json::from_slice(&first).unwrap();
        assert!(rendered.get("ranked_candidates").is_none());
        assert!(rendered.get("approval").is_none());
        assert!(rendered.get("runtime_supported_families").is_none());
        assert!(rendered.get("families").is_none());

        let exact_hash = inventory::inventory_sha256_hex(&first);
        let without_trailing_newline = first[..first.len() - 1].to_vec();
        assert_ne!(
            exact_hash,
            inventory::inventory_sha256_hex(&without_trailing_newline)
        );
    }

    #[test]
    fn family_inventory_command_rejects_non_json_format() {
        let temp_dir = workspace_root();
        seed_inventory_repo_truth(temp_dir.path());

        let code = run_from(
            temp_dir.path(),
            ["xtask", "family", "inventory", "--format", "text"],
        );

        assert_eq!(code, 2);
    }

    #[test]
    fn artifact_schema_accepts_recommendation_with_exact_inventory_hash() {
        let temp_dir = workspace_root();
        seed_inventory_repo_truth(temp_dir.path());

        let run_id = "20260429T154500Z-function.wrapper.pipeline.v1";
        let inventory_path = write_inventory_snapshot(temp_dir.path(), run_id);
        let inventory_bytes = fs::read(temp_dir.path().join(&inventory_path)).unwrap();
        let recommendation_path = temp_dir
            .path()
            .join(".semantic-family-artifacts/family-promotion/recommendation.latest.json");

        write_json_file(
            &recommendation_path,
            &FamilyRecommendationArtifact {
                schema_version: 1,
                artifact_kind: PromotionArtifactKind::FamilyRecommendation,
                generated_at: "2026-04-29T15:45:00Z".to_string(),
                inventory_path: inventory_path.clone(),
                inventory_sha256: inventory::inventory_sha256_hex(&inventory_bytes),
                target_language: TargetLanguage::Rust,
                ranked_candidates: vec![
                    RankedCandidate {
                        family: "function.wrapper.pipeline.v1".to_string(),
                        evidence: vec![
                            "spec-core/src/semantic_review.rs".to_string(),
                            "examples/ecommerce/units/pricing/calculate_total.unit.spec"
                                .to_string(),
                        ],
                        expected_leverage:
                            "Broadens the promoted corpus from leaves to a two-step wrapper."
                                .to_string(),
                        expected_risks: vec![
                            "Routing order must stay between chain3 and the leaves.".to_string(),
                        ],
                    },
                    RankedCandidate {
                        family: "function.arithmetic_leaf.monotone_up.v1".to_string(),
                        evidence: vec!["spec-core/src/semantic_review.rs".to_string()],
                        expected_leverage: "Already promoted and therefore informational only."
                            .to_string(),
                        expected_risks: vec![
                            "Not approval-eligible from this artifact.".to_string()
                        ],
                    },
                ],
            },
        );

        let code = run_from(
            temp_dir.path(),
            [
                "xtask",
                "family",
                "validate-artifact",
                ".semantic-family-artifacts/family-promotion/recommendation.latest.json",
            ],
        );

        assert_eq!(code, 0);
    }

    #[test]
    fn artifact_schema_rejects_recommendation_when_inventory_hash_is_recomputed_from_different_bytes(
    ) {
        let temp_dir = workspace_root();
        seed_inventory_repo_truth(temp_dir.path());

        let run_id = "20260429T154500Z-function.wrapper.pipeline.v1";
        let inventory_path = write_inventory_snapshot(temp_dir.path(), run_id);
        let recommendation_path = temp_dir
            .path()
            .join(".semantic-family-artifacts/family-promotion/recommendation.latest.json");

        write_json_file(
            &recommendation_path,
            &FamilyRecommendationArtifact {
                schema_version: 1,
                artifact_kind: PromotionArtifactKind::FamilyRecommendation,
                generated_at: "2026-04-29T15:45:00Z".to_string(),
                inventory_path,
                inventory_sha256: "deadbeef".to_string(),
                target_language: TargetLanguage::Rust,
                ranked_candidates: vec![RankedCandidate {
                    family: "function.wrapper.pipeline.v1".to_string(),
                    evidence: vec!["spec-core/src/semantic_review.rs".to_string()],
                    expected_leverage: "Two-step wrapper promotion target.".to_string(),
                    expected_risks: vec![
                        "Inventory hash mismatch should fail validation.".to_string()
                    ],
                }],
            },
        );

        let code = run_from(
            temp_dir.path(),
            [
                "xtask",
                "family",
                "validate-artifact",
                ".semantic-family-artifacts/family-promotion/recommendation.latest.json",
            ],
        );

        assert_eq!(code, 2);
    }

    #[test]
    fn artifact_schema_accepts_schema_v2_recommendation_analysis_with_ready_first_candidate() {
        let temp_dir = workspace_root();
        let (coverage_path, coverage_sha256) =
            seed_recommendation_analysis_coverage(temp_dir.path());
        let analysis_path = temp_dir
            .path()
            .join(FAMILY_RECOMMENDATION_ANALYSIS_LATEST_PATH);

        write_json_file(
            &analysis_path,
            &FamilyRecommendationAnalysisArtifact {
                schema_version: RECOMMENDATION_ANALYSIS_SCHEMA_VERSION,
                artifact_kind: PromotionArtifactKind::FamilyRecommendationAnalysis,
                generated_at: "2026-04-29T15:45:00Z".to_string(),
                coverage_path,
                coverage_sha256,
                recommendation_status: RecommendationStatus::Ranked,
                ranked_candidates: vec![valid_recommendation_candidate(
                    PromotionReadiness::Ready,
                    Vec::new(),
                )],
            },
        );

        let code = run_from(
            temp_dir.path(),
            [
                "xtask",
                "family",
                "validate-artifact",
                FAMILY_RECOMMENDATION_ANALYSIS_LATEST_PATH,
            ],
        );

        assert_eq!(code, 0);
    }

    #[test]
    fn artifact_schema_rejects_ready_recommendation_candidate_with_hold_reasons() {
        let temp_dir = workspace_root();
        let (coverage_path, coverage_sha256) =
            seed_recommendation_analysis_coverage(temp_dir.path());
        let analysis_path = temp_dir
            .path()
            .join(FAMILY_RECOMMENDATION_ANALYSIS_LATEST_PATH);

        write_json_file(
            &analysis_path,
            &FamilyRecommendationAnalysisArtifact {
                schema_version: RECOMMENDATION_ANALYSIS_SCHEMA_VERSION,
                artifact_kind: PromotionArtifactKind::FamilyRecommendationAnalysis,
                generated_at: "2026-04-29T15:45:00Z".to_string(),
                coverage_path,
                coverage_sha256,
                recommendation_status: RecommendationStatus::NoStrongCandidate,
                ranked_candidates: vec![valid_recommendation_candidate(
                    PromotionReadiness::Ready,
                    vec![HoldReason::ThinRealExampleSupport],
                )],
            },
        );

        let code = run_from(
            temp_dir.path(),
            [
                "xtask",
                "family",
                "validate-artifact",
                FAMILY_RECOMMENDATION_ANALYSIS_LATEST_PATH,
            ],
        );

        assert_eq!(code, 2);
    }

    #[test]
    fn artifact_schema_rejects_held_recommendation_candidate_without_hold_reasons() {
        let temp_dir = workspace_root();
        let (coverage_path, coverage_sha256) =
            seed_recommendation_analysis_coverage(temp_dir.path());
        let analysis_path = temp_dir
            .path()
            .join(FAMILY_RECOMMENDATION_ANALYSIS_LATEST_PATH);

        write_json_file(
            &analysis_path,
            &FamilyRecommendationAnalysisArtifact {
                schema_version: RECOMMENDATION_ANALYSIS_SCHEMA_VERSION,
                artifact_kind: PromotionArtifactKind::FamilyRecommendationAnalysis,
                generated_at: "2026-04-29T15:45:00Z".to_string(),
                coverage_path,
                coverage_sha256,
                recommendation_status: RecommendationStatus::NoStrongCandidate,
                ranked_candidates: vec![valid_recommendation_candidate(
                    PromotionReadiness::Hold,
                    Vec::new(),
                )],
            },
        );

        let code = run_from(
            temp_dir.path(),
            [
                "xtask",
                "family",
                "validate-artifact",
                FAMILY_RECOMMENDATION_ANALYSIS_LATEST_PATH,
            ],
        );

        assert_eq!(code, 2);
    }

    #[test]
    fn artifact_schema_rejects_ranked_recommendation_analysis_when_first_candidate_is_held() {
        let temp_dir = workspace_root();
        let (coverage_path, coverage_sha256) =
            seed_recommendation_analysis_coverage(temp_dir.path());
        let analysis_path = temp_dir
            .path()
            .join(FAMILY_RECOMMENDATION_ANALYSIS_LATEST_PATH);

        write_json_file(
            &analysis_path,
            &FamilyRecommendationAnalysisArtifact {
                schema_version: RECOMMENDATION_ANALYSIS_SCHEMA_VERSION,
                artifact_kind: PromotionArtifactKind::FamilyRecommendationAnalysis,
                generated_at: "2026-04-29T15:45:00Z".to_string(),
                coverage_path,
                coverage_sha256,
                recommendation_status: RecommendationStatus::Ranked,
                ranked_candidates: vec![valid_recommendation_candidate(
                    PromotionReadiness::Hold,
                    vec![HoldReason::UnknownOverlapFamily],
                )],
            },
        );

        let code = run_from(
            temp_dir.path(),
            [
                "xtask",
                "family",
                "validate-artifact",
                FAMILY_RECOMMENDATION_ANALYSIS_LATEST_PATH,
            ],
        );

        assert_eq!(code, 2);
    }

    #[test]
    fn recommendation_policy_holds_unknown_overlap_hard_single_real_candidate() {
        let artifact = recommendation_analysis_from_clusters(vec![unsupported_cluster(
            "money-round-cluster",
            UnsupportedFunctionReasonCode::UnsupportedFunctionSurface,
            "unknown",
            1,
            1,
            0,
            CandidateStatus::Rankable,
            vec![
                "examples_ecommerce::money/round".to_string(),
                "m20_unsupported_truth_pack::money/round".to_string(),
            ],
        )]);

        assert_eq!(
            artifact.recommendation_status,
            RecommendationStatus::NoStrongCandidate
        );
        assert_eq!(artifact.ranked_candidates.len(), 1);

        let candidate = &artifact.ranked_candidates[0];
        assert_eq!(candidate.promotion_readiness, PromotionReadiness::Hold);
        assert_eq!(candidate.confidence.level, ConfidenceLevel::Low);
        assert_eq!(
            candidate.hold_reasons,
            vec![
                HoldReason::UnknownOverlapFamily,
                HoldReason::HardDifficulty,
                HoldReason::ThinRealExampleSupport,
                HoldReason::ThinRegressionSupport,
            ]
        );
    }

    #[test]
    fn recommendation_policy_returns_insufficient_real_corpus_when_no_discoverable_candidates() {
        let artifact = recommendation_analysis_from_clusters(vec![unsupported_cluster(
            "insufficient-cluster",
            UnsupportedFunctionReasonCode::UnsupportedWrapperBodyShape,
            "function.wrapper.pipeline*",
            0,
            1,
            0,
            CandidateStatus::InsufficientEvidence,
            vec!["m20_unsupported_truth_pack::pricing/checkout_total_bad_body_shape".to_string()],
        )]);

        assert_eq!(
            artifact.recommendation_status,
            RecommendationStatus::InsufficientRealCorpus
        );
        assert!(artifact.ranked_candidates.is_empty());
    }

    #[test]
    fn recommendation_policy_returns_no_strong_candidate_when_discoverable_candidates_are_held() {
        let artifact = recommendation_analysis_from_clusters(vec![unsupported_cluster(
            "held-cluster",
            UnsupportedFunctionReasonCode::UnsupportedWrapperBodyShape,
            "function.wrapper.pipeline*",
            0,
            1,
            0,
            CandidateStatus::Rankable,
            vec!["m20_unsupported_truth_pack::pricing/checkout_total_bad_body_shape".to_string()],
        )]);

        assert_eq!(
            artifact.recommendation_status,
            RecommendationStatus::NoStrongCandidate
        );
        assert_eq!(artifact.ranked_candidates.len(), 1);
        assert_eq!(
            artifact.ranked_candidates[0].promotion_readiness,
            PromotionReadiness::Hold
        );
        assert_eq!(
            artifact.ranked_candidates[0].hold_reasons,
            vec![
                HoldReason::ThinRealExampleSupport,
                HoldReason::ThinRegressionSupport,
            ]
        );
    }

    #[test]
    fn recommendation_policy_ranks_known_overlap_candidate_with_strong_evidence() {
        let artifact = recommendation_analysis_from_clusters(vec![unsupported_cluster(
            "strong-known-cluster",
            UnsupportedFunctionReasonCode::UnsupportedWrapperBodyShape,
            "function.wrapper.pipeline*",
            3,
            1,
            0,
            CandidateStatus::Rankable,
            vec!["examples_ecommerce::pricing/calculate_total".to_string()],
        )]);

        assert_eq!(artifact.recommendation_status, RecommendationStatus::Ranked);
        assert_eq!(artifact.ranked_candidates.len(), 1);

        let candidate = &artifact.ranked_candidates[0];
        assert_eq!(candidate.promotion_readiness, PromotionReadiness::Ready);
        assert!(candidate.hold_reasons.is_empty());
        assert_eq!(candidate.confidence.level, ConfidenceLevel::High);
    }

    #[test]
    fn recommendation_policy_sorts_ready_candidates_ahead_of_held_candidates() {
        let artifact = recommendation_analysis_from_clusters(vec![
            unsupported_cluster(
                "held-high-leverage",
                UnsupportedFunctionReasonCode::UnsupportedFunctionSurface,
                "unknown",
                5,
                5,
                0,
                CandidateStatus::Rankable,
                vec!["examples_ecommerce::money/round".to_string()],
            ),
            unsupported_cluster(
                "ready-medium-confidence",
                UnsupportedFunctionReasonCode::UnsupportedRequiredArgumentExpression,
                "function.wrapper.pipeline*",
                1,
                3,
                0,
                CandidateStatus::Rankable,
                vec!["examples_ecommerce::pricing/calculate_total".to_string()],
            ),
            unsupported_cluster(
                "ready-high-confidence",
                UnsupportedFunctionReasonCode::UnsupportedWrapperBodyShape,
                "function.wrapper.pipeline*",
                3,
                1,
                0,
                CandidateStatus::Rankable,
                vec!["examples_ecommerce::pricing/checkout_total".to_string()],
            ),
        ]);

        assert_eq!(artifact.recommendation_status, RecommendationStatus::Ranked);
        assert_eq!(
            artifact
                .ranked_candidates
                .iter()
                .map(|candidate| candidate.cluster_ids[0].as_str())
                .collect::<Vec<_>>(),
            vec![
                "ready-high-confidence",
                "ready-medium-confidence",
                "held-high-leverage",
            ]
        );
        assert_eq!(
            artifact.ranked_candidates[0].promotion_readiness,
            PromotionReadiness::Ready
        );
        assert_eq!(
            artifact.ranked_candidates[1].confidence.level,
            ConfidenceLevel::Medium
        );
        assert_eq!(
            artifact.ranked_candidates[2].promotion_readiness,
            PromotionReadiness::Hold
        );
    }

    #[test]
    fn recommendation_command_path_writes_same_bytes_and_locked_corpus_is_no_strong_candidate() {
        let temp_dir = workspace_root();
        seed_locked_recommendation_workspace(temp_dir.path());

        let mut stdout = Vec::new();
        recommend::run_with_writer(temp_dir.path(), "json", &mut stdout).unwrap();

        let artifact_path = temp_dir
            .path()
            .join(FAMILY_RECOMMENDATION_ANALYSIS_LATEST_PATH);
        let written_bytes = fs::read(&artifact_path).unwrap();
        assert_eq!(stdout, written_bytes);

        let recommendation: FamilyRecommendationAnalysisArtifact =
            serde_json::from_slice(&written_bytes).unwrap();
        let coverage: FamilyCoverageArtifact = serde_json::from_slice(
            &fs::read(temp_dir.path().join(FAMILY_COVERAGE_LATEST_PATH)).unwrap(),
        )
        .unwrap();

        assert_eq!(
            recommendation.recommendation_status,
            RecommendationStatus::NoStrongCandidate
        );

        let money_round_cluster = coverage
            .unsupported_clusters
            .iter()
            .find(|cluster| {
                cluster
                    .representative_unit_ids
                    .contains(&"examples_ecommerce::money/round".to_string())
            })
            .unwrap();
        let money_round_candidate = recommendation
            .ranked_candidates
            .iter()
            .find(|candidate| candidate.cluster_ids.contains(&money_round_cluster.cluster_id))
            .unwrap();

        assert_eq!(
            money_round_candidate.promotion_readiness,
            PromotionReadiness::Hold
        );
        assert!(money_round_candidate
            .hold_reasons
            .contains(&HoldReason::UnknownOverlapFamily));
        assert!(money_round_candidate
            .hold_reasons
            .contains(&HoldReason::HardDifficulty));
        assert!(money_round_candidate
            .hold_reasons
            .contains(&HoldReason::ThinRealExampleSupport));
    }

    #[test]
    fn artifact_schema_accepts_execution_report_with_real_proof_artifact_paths() {
        let temp_dir = workspace_root();
        seed_inventory_repo_truth(temp_dir.path());

        let run_id = "20260429T154500Z-function.wrapper.pipeline.v1";
        let recommendation_path = seed_valid_recommendation_artifact(temp_dir.path(), run_id);
        seed_promoted_wrapper_proof_artifacts(temp_dir.path());
        write_string(
            &temp_dir
                .path()
                .join("semantic-families/function.wrapper.pipeline.v1/candidate.md"),
            "# wrapper packet\n",
        );

        let report_path = temp_dir.path().join(format!(
            ".semantic-family-artifacts/family-promotion/function.wrapper.pipeline.v1/{run_id}/promotion.execution.json"
        ));
        write_json_file(
            &report_path,
            &PromotionExecutionArtifact {
                schema_version: 1,
                artifact_kind: PromotionArtifactKind::PromotionExecution,
                run_id: run_id.to_string(),
                family: "function.wrapper.pipeline.v1".to_string(),
                status: promotion_artifacts::ExecutionStatus::Green,
                recommendation_path,
                approvals: PromotionApprovals {
                    target_family: ApprovalRecord {
                        status: ApprovalStatus::Approved,
                    },
                    final_output: ApprovalRecord {
                        status: ApprovalStatus::Approved,
                    },
                },
                files_changed: vec![
                    "semantic-families/function.wrapper.pipeline.v1/candidate.md".to_string(),
                    "spec-cli/tests/cli.rs".to_string(),
                ],
                commands: vec![CommandRecord {
                    step: "prove".to_string(),
                    command: "cargo xtask family prove function.wrapper.pipeline.v1".to_string(),
                    exit_code: 0,
                    started_at: "2026-04-29T15:50:00Z".to_string(),
                    finished_at: "2026-04-29T15:51:00Z".to_string(),
                    artifact_path: Some(
                        ".semantic-family-artifacts/semantic-families/function.wrapper.pipeline.v1/prove.latest.json"
                            .to_string(),
                    ),
                }],
                referenced_proof_artifacts: vec![
                    ".semantic-family-artifacts/semantic-families/function.wrapper.pipeline.v1/prove.latest.json"
                        .to_string(),
                    ".semantic-family-artifacts/semantic-families/function.wrapper.pipeline.v1/attempt-20260429T155100Z.json"
                        .to_string(),
                    ".semantic-family-artifacts/semantic-families/function.wrapper.pipeline.v1/certification.report.json"
                        .to_string(),
                ],
                iterations: 1,
                gate_summary: GateSummary {
                    smoke: GateStatus::Pass,
                    prove: GateStatus::Pass,
                    certify: GateStatus::Pass,
                },
                notes: vec!["All hard gates are green.".to_string()],
            },
        );

        let code = run_from(
            temp_dir.path(),
            [
                "xtask",
                "family",
                "validate-artifact",
                &format!(
                    ".semantic-family-artifacts/family-promotion/function.wrapper.pipeline.v1/{run_id}/promotion.execution.json"
                ),
            ],
        );

        assert_eq!(code, 0);
    }

    #[test]
    fn artifact_schema_rejects_execution_report_when_proof_artifact_path_is_missing() {
        let temp_dir = workspace_root();
        seed_inventory_repo_truth(temp_dir.path());

        let run_id = "20260429T154500Z-function.wrapper.pipeline.v1";
        let recommendation_path = seed_valid_recommendation_artifact(temp_dir.path(), run_id);
        let report_path = temp_dir.path().join(format!(
            ".semantic-family-artifacts/family-promotion/function.wrapper.pipeline.v1/{run_id}/promotion.execution.json"
        ));
        write_json_file(
            &report_path,
            &PromotionExecutionArtifact {
                schema_version: 1,
                artifact_kind: PromotionArtifactKind::PromotionExecution,
                run_id: run_id.to_string(),
                family: "function.wrapper.pipeline.v1".to_string(),
                status: promotion_artifacts::ExecutionStatus::Green,
                recommendation_path,
                approvals: PromotionApprovals {
                    target_family: ApprovalRecord {
                        status: ApprovalStatus::Approved,
                    },
                    final_output: ApprovalRecord {
                        status: ApprovalStatus::Pending,
                    },
                },
                files_changed: vec![
                    "semantic-families/function.wrapper.pipeline.v1/candidate.md".to_string(),
                ],
                commands: vec![CommandRecord {
                    step: "certify".to_string(),
                    command: "cargo xtask family certify function.wrapper.pipeline.v1".to_string(),
                    exit_code: 1,
                    started_at: "2026-04-29T15:52:00Z".to_string(),
                    finished_at: "2026-04-29T15:53:00Z".to_string(),
                    artifact_path: None,
                }],
                referenced_proof_artifacts: vec![
                    ".semantic-family-artifacts/semantic-families/function.wrapper.pipeline.v1/prove.latest.json"
                        .to_string(),
                    ".semantic-family-artifacts/semantic-families/function.wrapper.pipeline.v1/attempt-20260429T155100Z.json"
                        .to_string(),
                    ".semantic-family-artifacts/semantic-families/function.wrapper.pipeline.v1/certification.report.json"
                        .to_string(),
                ],
                iterations: 2,
                gate_summary: GateSummary {
                    smoke: GateStatus::Pass,
                    prove: GateStatus::Pass,
                    certify: GateStatus::Fail,
                },
                notes: vec!["Missing proof artifacts should fail validation.".to_string()],
            },
        );

        let code = run_from(
            temp_dir.path(),
            [
                "xtask",
                "family",
                "validate-artifact",
                &format!(
                    ".semantic-family-artifacts/family-promotion/function.wrapper.pipeline.v1/{run_id}/promotion.execution.json"
                ),
            ],
        );

        assert_eq!(code, 2);
    }

    #[test]
    fn artifact_schema_accepts_blocker_report_with_locked_vocabulary() {
        let temp_dir = workspace_root();
        seed_inventory_repo_truth(temp_dir.path());

        let run_id = "20260429T154500Z-function.wrapper.pipeline.v1";
        let inventory_path = write_inventory_snapshot(temp_dir.path(), run_id);
        let report_path = temp_dir.path().join(format!(
            ".semantic-family-artifacts/family-promotion/function.wrapper.pipeline.v1/{run_id}/blocker.report.json"
        ));
        write_json_file(
            &report_path,
            &PromotionBlockerArtifact {
                schema_version: 1,
                artifact_kind: PromotionArtifactKind::PromotionBlocker,
                run_id: run_id.to_string(),
                family: "function.wrapper.pipeline.v1".to_string(),
                blocking_step: BlockingStep::Certify,
                blocker_kind: BlockerKind::CertifyRoutingConflict,
                summary: "Registry routing order still conflicts with the promoted wrapper family."
                    .to_string(),
                machine_evidence: vec![
                    MachineEvidence {
                        kind: MachineEvidenceKind::Command,
                        path: None,
                        command: Some(
                            "cargo xtask family certify function.wrapper.pipeline.v1".to_string(),
                        ),
                        exit_code: Some(1),
                        observed_at: "2026-04-29T15:55:00Z".to_string(),
                        note: "Certify failed during the routing gate.".to_string(),
                    },
                    MachineEvidence {
                        kind: MachineEvidenceKind::Artifact,
                        path: Some(inventory_path),
                        command: None,
                        exit_code: None,
                        observed_at: "2026-04-29T15:55:01Z".to_string(),
                        note: "Gate 1 inventory snapshot for this run.".to_string(),
                    },
                ],
                required_human_action:
                    "Confirm whether the routing conflict should be resolved in the packet or the runtime contract."
                        .to_string(),
                safe_next_actions: vec![
                    "Do not start wrapper-family edits under the stale routing contract."
                        .to_string(),
                ],
            },
        );

        let code = run_from(
            temp_dir.path(),
            [
                "xtask",
                "family",
                "validate-artifact",
                &format!(
                    ".semantic-family-artifacts/family-promotion/function.wrapper.pipeline.v1/{run_id}/blocker.report.json"
                ),
            ],
        );

        assert_eq!(code, 0);
    }

    #[test]
    fn artifact_schema_rejects_blocker_report_with_unknown_blocker_kind() {
        let temp_dir = workspace_root();
        seed_inventory_repo_truth(temp_dir.path());

        let run_id = "20260429T154500Z-function.wrapper.pipeline.v1";
        let report_path = temp_dir.path().join(format!(
            ".semantic-family-artifacts/family-promotion/function.wrapper.pipeline.v1/{run_id}/blocker.report.json"
        ));
        write_string(
            &report_path,
            r#"{
  "schema_version": 1,
  "artifact_kind": "promotion_blocker",
  "run_id": "20260429T154500Z-function.wrapper.pipeline.v1",
  "family": "function.wrapper.pipeline.v1",
  "blocking_step": "smoke",
  "blocker_kind": "unknown_kind",
  "summary": "unknown blocker kind",
  "machine_evidence": [
    {
      "kind": "command",
      "path": null,
      "command": "cargo xtask family smoke function.wrapper.pipeline.v1",
      "exit_code": 1,
      "observed_at": "2026-04-29T15:55:00Z",
      "note": "smoke failed"
    }
  ],
  "required_human_action": "decide next step",
  "safe_next_actions": ["do not proceed"]
}
"#,
        );

        let code = run_from(
            temp_dir.path(),
            [
                "xtask",
                "family",
                "validate-artifact",
                &format!(
                    ".semantic-family-artifacts/family-promotion/function.wrapper.pipeline.v1/{run_id}/blocker.report.json"
                ),
            ],
        );

        assert_eq!(code, 2);
    }

    fn workspace_root() -> TempDir {
        let temp_dir = TempDir::new().unwrap();
        fs::create_dir(temp_dir.path().join("semantic-families")).unwrap();
        temp_dir
    }

    fn write_string(path: &Path, contents: &str) {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(path, contents).unwrap();
    }

    fn write_json_file<T: serde::Serialize>(path: &Path, value: &T) {
        let contents = serde_json::to_string_pretty(value).unwrap();
        write_string(path, &format!("{contents}\n"));
    }

    fn seed_inventory_repo_truth(workspace_root: &Path) {
        write_string(
            &workspace_root.join("spec-core/src/semantic_review.rs"),
            r#"const SUPPORTED_FUNCTION_ROUTING_ORDER: [SupportedFunctionRoute; 4] = [
    SupportedFunctionRoute::WrapperPipelineChain3,
    SupportedFunctionRoute::WrapperPipeline,
    SupportedFunctionRoute::ArithmeticLeafMonotoneDownNonnegative,
    SupportedFunctionRoute::ArithmeticLeafMonotoneUp,
];
"#,
        );
        write_string(
            &workspace_root.join("examples/ecommerce/units/pricing/apply_discount.unit.spec"),
            "id: pricing/apply_discount\n",
        );
        write_string(
            &workspace_root.join("examples/ecommerce/units/pricing/apply_tax.unit.spec"),
            "id: pricing/apply_tax\n",
        );
        write_string(
            &workspace_root.join("examples/ecommerce/units/pricing/calculate_total.unit.spec"),
            "id: pricing/calculate_total\n",
        );
        write_string(
            &workspace_root.join("spec-cli/tests/m14_regressions.rs"),
            "fn wrapper_pipeline_existing_wedge() {}\n",
        );
        write_string(
            &workspace_root.join(
                "semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/aligned/units/pricing/checkout_chain3_aligned.unit.spec",
            ),
            "kind: function\n",
        );
        write_string(
            &workspace_root.join(
                "semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/aligned/units/pricing/pricing_discount_leaf_aligned.unit.spec",
            ),
            "kind: function\n",
        );
        write_string(
            &workspace_root.join(
                "semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/aligned/units/pricing/pricing_tax_leaf_aligned.unit.spec",
            ),
            "kind: function\n",
        );
        write_string(
            &workspace_root.join(
                "semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/aligned/units/pricing/pricing_total_wrapper_aligned.unit.spec",
            ),
            "kind: function\n",
        );
        write_string(
            &workspace_root.join(
                "semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/drift/units/pricing/pricing_total_wrapper_drift.unit.spec",
            ),
            "kind: function\n",
        );
        write_string(
            &workspace_root.join(
                "semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/under_specified/units/pricing/pricing_total_wrapper_under_specified.unit.spec",
            ),
            "kind: function\n",
        );
        write_string(
            &workspace_root.join(
                "semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/unsupported_near_miss/units/pricing/pricing_total_wrapper_unsupported_near_miss.unit.spec",
            ),
            "kind: function\n",
        );
        fs::create_dir_all(
            workspace_root
                .join("semantic-families/function.arithmetic_leaf.monotone_down_nonnegative.v1"),
        )
        .unwrap();
        fs::create_dir_all(
            workspace_root.join("semantic-families/function.arithmetic_leaf.monotone_up.v1"),
        )
        .unwrap();
    }

    fn write_inventory_snapshot(workspace_root: &Path, run_id: &str) -> String {
        let relative_path =
            format!(".semantic-family-artifacts/family-promotion/inventory/{run_id}.json");
        let absolute_path = workspace_root.join(&relative_path);
        let bytes =
            inventory::render_snapshot_bytes_in(&pre_wrapper_promotion_registry(), workspace_root)
                .unwrap();
        if let Some(parent) = absolute_path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(&absolute_path, bytes).unwrap();
        relative_path
    }

    fn pre_wrapper_promotion_registry() -> [FamilyHarness; 3] {
        [
            *family_harness(&FamilyId::parse("function.wrapper.pipeline.chain3.v1").unwrap())
                .unwrap(),
            *family_harness(
                &FamilyId::parse("function.arithmetic_leaf.monotone_down_nonnegative.v1").unwrap(),
            )
            .unwrap(),
            *family_harness(&FamilyId::parse("function.arithmetic_leaf.monotone_up.v1").unwrap())
                .unwrap(),
        ]
    }

    fn seed_valid_recommendation_artifact(workspace_root: &Path, run_id: &str) -> String {
        let inventory_path = write_inventory_snapshot(workspace_root, run_id);
        let inventory_bytes = fs::read(workspace_root.join(&inventory_path)).unwrap();
        let recommendation_path = workspace_root
            .join(".semantic-family-artifacts/family-promotion/recommendation.latest.json");
        write_json_file(
            &recommendation_path,
            &FamilyRecommendationArtifact {
                schema_version: 1,
                artifact_kind: PromotionArtifactKind::FamilyRecommendation,
                generated_at: "2026-04-29T15:45:00Z".to_string(),
                inventory_path,
                inventory_sha256: inventory::inventory_sha256_hex(&inventory_bytes),
                target_language: TargetLanguage::Rust,
                ranked_candidates: vec![RankedCandidate {
                    family: "function.wrapper.pipeline.v1".to_string(),
                    evidence: vec![
                        "spec-core/src/semantic_review.rs".to_string(),
                        "examples/ecommerce/units/pricing/calculate_total.unit.spec".to_string(),
                    ],
                    expected_leverage:
                        "Broadens the corpus from leaves to a dedicated wrapper family.".to_string(),
                    expected_risks: vec![
                        "Routing order must remain between chain3 and the leaves.".to_string(),
                    ],
                }],
            },
        );
        ".semantic-family-artifacts/family-promotion/recommendation.latest.json".to_string()
    }

    fn seed_recommendation_analysis_coverage(workspace_root: &Path) -> (String, String) {
        let coverage_path = FAMILY_COVERAGE_LATEST_PATH.to_string();
        write_string(
            &workspace_root.join(FAMILY_COVERAGE_LATEST_PATH),
            "{\n  \"analysis_fixture\": true\n}\n",
        );
        let coverage_bytes = fs::read(workspace_root.join(FAMILY_COVERAGE_LATEST_PATH)).unwrap();
        (
            coverage_path,
            inventory::inventory_sha256_hex(&coverage_bytes),
        )
    }

    fn valid_recommendation_candidate(
        promotion_readiness: PromotionReadiness,
        hold_reasons: Vec<HoldReason>,
    ) -> RecommendationCandidateEntry {
        RecommendationCandidateEntry {
            candidate_id: "a-unsupportedwrappershape-cluster-01".to_string(),
            cluster_ids: vec!["cluster-01".to_string()],
            primary_reason_code: spec_core::semantic_review::UnsupportedFunctionReasonCode::UnsupportedWrapperBodyShape,
            overlap_family: "function.wrapper.pipeline*".to_string(),
            promotion_readiness,
            hold_reasons,
            leverage: RecommendationLeverage {
                real_example_hits: 2,
                promotion_relevant_regression_hits: 3,
                boundary_only_hits: 0,
                total_units_in_cluster: 2,
            },
            difficulty: RecommendationDifficulty {
                tier: DifficultyTier::Adjacent,
                why: "This cluster is adjacent to the promoted wrapper pipeline family."
                    .to_string(),
            },
            confidence: RecommendationConfidence {
                level: ConfidenceLevel::Medium,
                why: "The cluster has enough support for validator coverage.".to_string(),
            },
            rationale: "Validator fixture candidate.".to_string(),
        }
    }

    fn recommendation_analysis_from_clusters(
        clusters: Vec<UnsupportedClusterEntry>,
    ) -> FamilyRecommendationAnalysisArtifact {
        recommend::build_recommendation_analysis_artifact(
            "2026-04-29T15:45:00Z".to_string(),
            FAMILY_COVERAGE_LATEST_PATH.to_string(),
            "coverage-sha".to_string(),
            &clusters,
        )
    }

    fn unsupported_cluster(
        cluster_id: &str,
        reason_code: UnsupportedFunctionReasonCode,
        overlap_family: &str,
        real_example_hits: usize,
        promotion_relevant_regression_hits: usize,
        boundary_only_hits: usize,
        candidate_status: CandidateStatus,
        representative_unit_ids: Vec<String>,
    ) -> UnsupportedClusterEntry {
        UnsupportedClusterEntry {
            cluster_id: cluster_id.to_string(),
            reason_code,
            shape_fingerprint: format!("shape::{cluster_id}"),
            representative_unit_ids,
            source_ids: vec!["synthetic_source".to_string()],
            real_example_hits,
            promotion_relevant_regression_hits,
            boundary_only_hits,
            overlap_family: overlap_family.to_string(),
            candidate_status,
        }
    }

    fn seed_locked_recommendation_workspace(workspace_root: &Path) {
        for relative_path in [
            "examples/ecommerce/units",
            "semantic-families/corpus/rust-function.toml",
            "semantic-families/function.wrapper.pipeline.chain3.v1",
            "semantic-families/function.arithmetic_leaf.monotone_down_nonnegative.v1",
            "semantic-families/function.arithmetic_leaf.monotone_up.v1",
            "spec-cli/tests/fixtures/m19/semantic_falsification_pack/units",
            "spec-cli/tests/fixtures/m20/unsupported_truth_pack/units",
            "spec-cli/tests/m14_regressions.rs",
            "spec-core/src/semantic_review.rs",
        ] {
            copy_path_from_repo(workspace_root, relative_path);
        }
    }

    fn copy_path_from_repo(destination_root: &Path, relative_path: &str) {
        let source = repo_workspace_root().join(relative_path);
        let destination = destination_root.join(relative_path);
        if source.is_dir() {
            copy_dir_recursive(&source, &destination);
        } else {
            if let Some(parent) = destination.parent() {
                fs::create_dir_all(parent).unwrap();
            }
            fs::copy(source, destination).unwrap();
        }
    }

    fn copy_dir_recursive(source: &Path, destination: &Path) {
        fs::create_dir_all(destination).unwrap();
        for entry in fs::read_dir(source).unwrap() {
            let entry = entry.unwrap();
            let source_path = entry.path();
            let destination_path = destination.join(entry.file_name());
            if source_path.is_dir() {
                copy_dir_recursive(&source_path, &destination_path);
            } else {
                if let Some(parent) = destination_path.parent() {
                    fs::create_dir_all(parent).unwrap();
                }
                fs::copy(source_path, destination_path).unwrap();
            }
        }
    }

    fn repo_workspace_root() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .to_path_buf()
    }

    fn seed_promoted_wrapper_proof_artifacts(workspace_root: &Path) {
        write_string(
            &workspace_root.join(
                ".semantic-family-artifacts/semantic-families/function.wrapper.pipeline.v1/prove.latest.json",
            ),
            "{}\n",
        );
        write_string(
            &workspace_root.join(
                ".semantic-family-artifacts/semantic-families/function.wrapper.pipeline.v1/attempt-20260429T155100Z.json",
            ),
            "{}\n",
        );
        write_string(
            &workspace_root.join(
                ".semantic-family-artifacts/semantic-families/function.wrapper.pipeline.v1/certification.report.json",
            ),
            "{}\n",
        );
    }

    fn seed_valid_manifest(manifest_path: &Path, family: &str) {
        write_string(
            manifest_path,
            &format!(
                r#"schema_version = 2
family = "{family}"
kind = "function"
compatibility_key = "{family}"
summary = "Straight-line three-call wrapper pipeline over supported function deps."

[routing]
precedence = 1
must_not_shadow = [
  "function.wrapper.pipeline.v1",
  "function.arithmetic_leaf.monotone_down_nonnegative.v1",
  "function.arithmetic_leaf.monotone_up.v1",
]

[shape]
dep_min = 3
dep_max = 3
control_flow = "straight_line_only"
return_style = "let_then_return_or_direct_return"
loops = false
branching = false
requires_supported_function_deps = true

[args]
threading = "ordered_passthrough"
allow_nested_argument_expressions = false
allow_literal_only_extra_args = false

[corpus]
required_buckets = ["aligned", "drift", "under_specified", "unsupported_near_miss"]
min_cases_per_bucket = 1

[truth_surface]
requires_refresh_via = ["spec test"]
preserve_only_via = ["spec build", "spec generate", "spec status", "spec export"]
requires_stale_demote = true

[gates]
gate_a = true
gate_b = true
gate_c = true
gate_d = true
"#
            ),
        );
    }

    fn seed_valid_cases(paths: &PacketPaths) {
        for bucket in REQUIRED_BUCKETS {
            for relative_path in expected_chain3_scaffold_unit_paths(bucket) {
                write_string(&paths.root.join(relative_path), "kind: function\n");
            }
        }
    }

    fn seed_suite_sources(workspace_root: &Path, include_cli_regression: bool) {
        write_string(
            &workspace_root.join("spec-core/src/semantic_review.rs"),
            "fn m21_chain3_classifier_alpha() {}\nfn m21_chain3_regression_alpha() {}\n",
        );
        write_string(
            &workspace_root.join("spec-cli/tests/cli.rs"),
            "fn m21_chain3_truth_surface_alpha() {}\n",
        );
        let cli_regression = if include_cli_regression {
            "fn m21_chain3_regression_alpha() {}\n"
        } else {
            ""
        };
        write_string(
            &workspace_root.join("spec-cli/tests/m14_regressions.rs"),
            &format!("fn m21_chain3_corpus_alpha() {{}}\n{cli_regression}"),
        );
    }

    fn success_certify_runner() -> FakeRunner {
        certify_runner_with_outputs(
            &[
                normalized_libtest_stdout(&expected_suite_test_names(CHAIN3_PROVE_SUITES[0])),
                normalized_libtest_stdout(&expected_suite_test_names(CHAIN3_PROVE_SUITES[1])),
                normalized_libtest_stdout(&expected_suite_test_names(CHAIN3_PROVE_SUITES[2])),
                normalized_libtest_stdout(&expected_suite_test_names(CHAIN3_CERTIFY_SUITES[0])),
                normalized_libtest_stdout(&expected_suite_test_names(CHAIN3_CERTIFY_SUITES[1])),
            ],
            "2026-04-27T18:10:00Z\n",
        )
    }

    fn prove_runner_with_suite_outputs(stdout_by_suite: &[String]) -> FakeRunner {
        assert_eq!(stdout_by_suite.len(), CHAIN3_PROVE_SUITES.len());
        let mut outputs = vec![
            command_output(&["git", "rev-parse", "HEAD"], 0, "abc123\n"),
            command_output(&["rustc", "--version"], 0, "rustc 1.89.0\n"),
            command_output(
                &["date", "-u", "+%Y-%m-%dT%H:%M:%SZ"],
                0,
                "2026-04-27T18:00:00Z\n",
            ),
        ];
        for (suite, stdout) in CHAIN3_PROVE_SUITES.iter().zip(stdout_by_suite.iter()) {
            outputs.push(suite_command_output(*suite, 0, stdout));
        }
        FakeRunner::new(&outputs)
    }

    fn certify_runner_with_outputs(
        stdout_by_suite: &[String],
        certify_generated_at: &str,
    ) -> FakeRunner {
        assert_eq!(
            stdout_by_suite.len(),
            CHAIN3_PROVE_SUITES.len() + CHAIN3_CERTIFY_SUITES.len()
        );
        let mut outputs = vec![
            command_output(&["git", "rev-parse", "HEAD"], 0, "abc123\n"),
            command_output(&["rustc", "--version"], 0, "rustc 1.89.0\n"),
            command_output(
                &["date", "-u", "+%Y-%m-%dT%H:%M:%SZ"],
                0,
                "2026-04-27T18:00:00Z\n",
            ),
        ];
        for (suite, stdout) in CHAIN3_PROVE_SUITES.iter().zip(stdout_by_suite.iter()) {
            outputs.push(suite_command_output(*suite, 0, stdout));
        }
        outputs.push(command_output(
            &["date", "-u", "+%Y-%m-%dT%H:%M:%SZ"],
            0,
            certify_generated_at,
        ));
        for (suite, stdout) in CHAIN3_CERTIFY_SUITES
            .iter()
            .zip(stdout_by_suite[CHAIN3_PROVE_SUITES.len()..].iter())
        {
            outputs.push(suite_command_output(*suite, 0, stdout));
        }
        FakeRunner::new(&outputs)
    }

    fn command_output(command: &[&str], exit_code: i32, stdout: &str) -> (String, CommandOutput) {
        (
            command.join("\u{1f}"),
            CommandOutput {
                exit_code,
                stdout: stdout.to_string(),
                stderr: String::new(),
            },
        )
    }

    fn suite_command_output(
        suite: SuiteDefinition,
        exit_code: i32,
        stdout: &str,
    ) -> (String, CommandOutput) {
        command_output(suite.command, exit_code, stdout)
    }

    fn expected_chain3_scaffold_unit_paths(bucket: &str) -> [String; 4] {
        [
            format!("fixtures/{bucket}/units/pricing/pricing_discount_leaf_{bucket}.unit.spec"),
            format!("fixtures/{bucket}/units/pricing/pricing_tax_leaf_{bucket}.unit.spec"),
            format!("fixtures/{bucket}/units/pricing/pricing_total_wrapper_{bucket}.unit.spec"),
            format!("fixtures/{bucket}/units/pricing/checkout_chain3_{bucket}.unit.spec"),
        ]
    }

    fn expected_wrapper_pipeline_scaffold_unit_paths(bucket: &str) -> [String; 3] {
        [
            format!("fixtures/{bucket}/units/pricing/pricing_discount_leaf_{bucket}.unit.spec"),
            format!("fixtures/{bucket}/units/pricing/pricing_tax_leaf_{bucket}.unit.spec"),
            format!("fixtures/{bucket}/units/pricing/pricing_total_wrapper_{bucket}.unit.spec"),
        ]
    }

    fn assert_candidate_lists_path_once(candidate: &str, relative_path: &str) {
        assert_eq!(candidate.matches(relative_path).count(), 1);
    }

    fn assert_starter_spec_is_valid_and_non_proving(path: &Path) {
        let loaded = load_file(path).unwrap();
        validate_full(&loaded).unwrap();
        let review = evaluate_semantic_review(&loaded)
            .expect("starter spec should produce a semantic review");
        assert_eq!(
            review.effective_support_status(),
            SemanticSupportStatus::Unsupported,
            "starter spec `{}` should remain outside the supported subset",
            path.display()
        );
    }

    fn rewrite_manifest(path: &Path, from: &str, to: &str) {
        let contents = fs::read_to_string(path).unwrap();
        assert!(
            contents.contains(from),
            "manifest rewrite anchor missing: {from}"
        );
        write_string(path, &contents.replacen(from, to, 1));
    }

    fn attempt_reports(paths: &PacketPaths) -> Vec<PathBuf> {
        let mut attempts = fs::read_dir(&paths.artifacts)
            .unwrap()
            .filter_map(|entry| {
                let path = entry.ok()?.path();
                let file_name = path.file_name()?.to_str()?;
                file_name.starts_with("attempt-").then_some(path)
            })
            .collect::<Vec<_>>();
        attempts.sort();
        attempts
    }

    fn read_report(path: &Path) -> serde_json::Value {
        serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap()
    }

    fn assert_required_gates(report: &serde_json::Value, expected: &[&str]) {
        let gates = report["required_gates"]
            .as_array()
            .expect("report required_gates should be an array")
            .iter()
            .map(|gate| {
                gate.as_str()
                    .expect("gate name should be a string")
                    .trim_start_matches("gate_")
                    .to_ascii_uppercase()
            })
            .collect::<Vec<_>>();
        assert_eq!(gates, expected);
    }

    fn assert_report_status_invariants(report: &serde_json::Value) {
        let required_gates = report["required_gates"]
            .as_array()
            .expect("report required_gates should be an array")
            .iter()
            .map(|gate| {
                let raw = gate.as_str().expect("gate name should be a string");
                if raw.starts_with("gate_") {
                    raw.to_string()
                } else {
                    format!("gate_{}", raw.to_ascii_lowercase())
                }
            })
            .collect::<Vec<_>>();
        if report["phase_status"] == "pass" {
            for gate in &required_gates {
                assert_eq!(
                    report["gates"][gate]["status"], "pass",
                    "phase_status=pass requires `{gate}` to pass"
                );
            }
        }
        if report["overall_status"] == "pass" {
            for gate in &required_gates {
                assert_eq!(
                    report["gates"][gate]["status"], "pass",
                    "overall_status=pass requires `{gate}` to pass"
                );
            }
        }
    }

    fn assert_artifact_surface_matches_captured_chain3_baseline(
        path: &Path,
        report: &serde_json::Value,
        expected_file_name: &str,
    ) {
        assert_eq!(
            path.file_name().and_then(|name| name.to_str()),
            Some(expected_file_name)
        );
        let mut actual_keys = report
            .as_object()
            .expect("report should be a JSON object")
            .keys()
            .map(String::as_str)
            .collect::<Vec<_>>();
        actual_keys.sort_unstable();

        let mut expected_keys = vec![
            "artifact_kind",
            "family",
            "fixture_digests",
            "gates",
            "generated_at",
            "git_commit_sha",
            "manifest_schema_version",
            "overall_status",
            "phase_status",
            "required_gates",
            "rust_toolchain",
            "schema_version",
            "suites",
        ];
        expected_keys.sort_unstable();

        assert_eq!(actual_keys, expected_keys);
    }

    fn assert_attempt_artifact_name_matches_captured_baseline(path: &Path) {
        let file_name = path
            .file_name()
            .and_then(|name| name.to_str())
            .expect("attempt artifact should have a file name");
        assert!(file_name.starts_with("attempt-"));
        assert!(file_name.ends_with(".json"));
        assert_ne!(file_name, "attempt-.json");
    }

    fn expected_suite_test_names(suite: SuiteDefinition) -> Vec<String> {
        suite
            .expected_tests
            .iter()
            .map(|name| (*name).to_string())
            .collect()
    }

    fn normalized_libtest_stdout(names: &[String]) -> String {
        let count = names.len();
        let noun = if count == 1 { "test" } else { "tests" };
        let mut stdout = format!("running {count} {noun}\n");
        for name in names {
            stdout.push_str(&format!("test {name} ... ok\n"));
        }
        stdout.push('\n');
        stdout.push_str(&format!(
            "test result: ok. {count} passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s\n"
        ));
        stdout
    }

    fn ignored_libtest_stdout(name: &str) -> String {
        format!(
            "running 1 test\ntest {name} ... ignored\n\ntest result: ok. 0 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.00s\n"
        )
    }

    fn colorized_libtest_stdout(names: &[String]) -> String {
        normalized_libtest_stdout(names).replace("test ", "\u{1b}[32mtest \u{1b}[0m")
    }

    fn format_variant_libtest_stdout(names: &[String]) -> String {
        let count = names.len();
        let noun = if count == 1 { "test" } else { "tests" };
        let mut stdout = format!("running {count} {noun}\n");
        for name in names {
            stdout.push_str(&format!("test {name} ... ok (0.00s)\n"));
        }
        stdout.push('\n');
        stdout.push_str(&format!(
            "test result: ok. {count} passed; 0 failed; 0 ignored; 0 measured; 0 filtered out\n"
        ));
        stdout
    }

    struct FakeRunner {
        outputs: RefCell<HashMap<String, VecDeque<CommandOutput>>>,
    }

    impl FakeRunner {
        fn new(entries: &[(String, CommandOutput)]) -> Self {
            let mut outputs = HashMap::<String, VecDeque<CommandOutput>>::new();
            for (command, output) in entries {
                outputs
                    .entry(command.clone())
                    .or_default()
                    .push_back(output.clone());
            }
            Self {
                outputs: RefCell::new(outputs),
            }
        }
    }

    impl CommandRunner for FakeRunner {
        fn run(&self, _cwd: &Path, command: &[String]) -> CommandOutput {
            let key = command.join("\u{1f}");
            self.outputs
                .borrow_mut()
                .get_mut(&key)
                .and_then(VecDeque::pop_front)
                .unwrap_or_else(|| panic!("unexpected command: {command:?}"))
        }
    }
}
