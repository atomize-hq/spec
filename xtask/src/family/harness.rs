use crate::XtaskError;
use crate::family::paths::FamilyId;
use crate::family::report::{ArtifactKind, CertificationReport, GateId, SuiteDefinition};

pub(crate) const TERMINAL_UNSUPPORTED_CATCH_ALL: &str = "unsupported.function.v1";

pub const CHAIN3_PRECEDENCE: u64 = 1;
pub const CHAIN3_MUST_NOT_SHADOW: [&str; 3] = [
    "function.wrapper.pipeline.v1",
    "function.arithmetic_leaf.monotone_down_nonnegative.v1",
    "function.arithmetic_leaf.monotone_up.v1",
];

const CHAIN3_STARTER_CASE_STEMS: [&str; 4] = [
    "pricing_discount_leaf",
    "pricing_tax_leaf",
    "pricing_total_wrapper",
    "checkout_chain3",
];

pub(crate) const CHAIN3_PROVE_SUITES: [SuiteDefinition; 3] = [
    SuiteDefinition {
        name: "spec-core:m21_chain3_classifier_",
        command: &[
            "cargo",
            "test",
            "-p",
            "spec-core",
            "--lib",
            "m21_chain3_classifier_",
            "--",
            "--color",
            "never",
        ],
        expected_tests: &[
            "semantic_review::tests::m21_chain3_classifier_aligned_fixture_routes_to_chain3",
            "semantic_review::tests::m21_chain3_classifier_drift_fixture_reports_semantic_drift",
            "semantic_review::tests::m21_chain3_classifier_runtime_order_is_explicit",
            "semantic_review::tests::m21_chain3_classifier_under_specified_fixture_reports_vague_truth",
            "semantic_review::tests::m21_chain3_classifier_unsupported_near_miss_stays_unsupported",
        ],
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
            "--color",
            "never",
        ],
        expected_tests: &[
            "m21_chain3_truth_surface_command_matrix_preserves_until_spec_test_refresh",
            "m21_chain3_truth_surface_stale_status_and_export_preserve_last_proven_review",
        ],
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
            "--color",
            "never",
        ],
        expected_tests: &[
            "m21_chain3_corpus_aligned_fixture_projects_valid_state",
            "m21_chain3_corpus_drift_fixture_projects_failing_state",
            "m21_chain3_corpus_under_specified_fixture_projects_incomplete_state",
            "m21_chain3_corpus_unsupported_near_miss_stays_additive_only_and_neutral",
        ],
    },
];

pub(crate) const CHAIN3_CERTIFY_SUITES: [SuiteDefinition; 2] = [
    SuiteDefinition {
        name: "spec-core:m21_chain3_regression_",
        command: &[
            "cargo",
            "test",
            "-p",
            "spec-core",
            "--lib",
            "m21_chain3_regression_",
            "--",
            "--color",
            "never",
        ],
        expected_tests: &[
            "semantic_review::tests::m21_chain3_regression_family_a_variants_are_not_shadowed",
            "semantic_review::tests::m21_chain3_regression_family_b_is_not_shadowed",
            "semantic_review::tests::m21_chain3_regression_runtime_order_matches_locked_precedence",
        ],
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
            "--color",
            "never",
        ],
        expected_tests: &[
            "m21_chain3_regression_family_b_read_side_surfaces_are_not_shadowed",
            "m21_chain3_regression_unsupported_near_miss_stays_additive_only_and_neutral",
        ],
    },
];

const CHAIN3_PROVE_SUITE_DEFINITIONS: [ProveSuiteDefinition; 3] = [
    ProveSuiteDefinition {
        suite: CHAIN3_PROVE_SUITES[0],
        gate: GateId::GateA,
    },
    ProveSuiteDefinition {
        suite: CHAIN3_PROVE_SUITES[1],
        gate: GateId::GateC,
    },
    ProveSuiteDefinition {
        suite: CHAIN3_PROVE_SUITES[2],
        gate: GateId::GateB,
    },
];

const CHAIN3_HARNESS: FamilyHarness = FamilyHarness {
    family: "function.wrapper.pipeline.chain3.v1",
    scaffold: ScaffoldDefinition {
        unit_namespace: "pricing",
        starter_case_stems: &CHAIN3_STARTER_CASE_STEMS,
    },
    routing: LockedManifestRouting {
        precedence: CHAIN3_PRECEDENCE,
        must_not_shadow: &CHAIN3_MUST_NOT_SHADOW,
    },
    prove_suites: &CHAIN3_PROVE_SUITE_DEFINITIONS,
    certify_suites: &CHAIN3_CERTIFY_SUITES,
};

const FAMILY_REGISTRY: [FamilyHarness; 1] = [CHAIN3_HARNESS];

const PROVE_LATEST_REQUIRED_GATES: [GateId; 3] = [GateId::GateA, GateId::GateB, GateId::GateC];
const CERTIFY_REQUIRED_GATES: [GateId; 4] =
    [GateId::GateA, GateId::GateB, GateId::GateC, GateId::GateD];

#[derive(Debug, Clone, Copy)]
pub(crate) struct FamilyHarness {
    pub family: &'static str,
    pub scaffold: ScaffoldDefinition,
    pub routing: LockedManifestRouting,
    pub prove_suites: &'static [ProveSuiteDefinition],
    pub certify_suites: &'static [SuiteDefinition],
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct ScaffoldDefinition {
    pub unit_namespace: &'static str,
    pub starter_case_stems: &'static [&'static str],
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct ProveSuiteDefinition {
    pub suite: SuiteDefinition,
    pub gate: GateId,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct LockedManifestRouting {
    pub precedence: u64,
    pub must_not_shadow: &'static [&'static str],
}

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct GateResults {
    pub gate_a: bool,
    pub gate_b: bool,
    pub gate_c: bool,
    pub gate_d: bool,
}

impl GateResults {
    pub(crate) fn from_report(report: &CertificationReport) -> Self {
        Self {
            gate_a: report.gates.gate_a.status.is_pass(),
            gate_b: report.gates.gate_b.status.is_pass(),
            gate_c: report.gates.gate_c.status.is_pass(),
            gate_d: report.gates.gate_d.status.is_pass(),
        }
    }

    pub(crate) fn set(&mut self, gate: GateId, passed: bool) {
        match gate {
            GateId::GateA => self.gate_a = passed,
            GateId::GateB => self.gate_b = passed,
            GateId::GateC => self.gate_c = passed,
            GateId::GateD => self.gate_d = passed,
        }
    }

    pub(crate) fn is_pass(self, gate: GateId) -> bool {
        match gate {
            GateId::GateA => self.gate_a,
            GateId::GateB => self.gate_b,
            GateId::GateC => self.gate_c,
            GateId::GateD => self.gate_d,
        }
    }

    pub(crate) fn satisfies(self, artifact_kind: ArtifactKind) -> bool {
        let required_gates = match artifact_kind {
            ArtifactKind::ProveLatest => &PROVE_LATEST_REQUIRED_GATES[..],
            ArtifactKind::CertifyAttempt | ArtifactKind::Certification => {
                &CERTIFY_REQUIRED_GATES[..]
            }
        };
        required_gates
            .iter()
            .copied()
            .all(|gate| self.is_pass(gate))
    }
}

pub(crate) fn family_harness(family: &FamilyId) -> Option<&'static FamilyHarness> {
    FAMILY_REGISTRY
        .iter()
        .find(|definition| definition.family == family.as_str())
}

pub(crate) fn require_family_harness(
    family: &FamilyId,
    workflow: &'static str,
) -> Result<&'static FamilyHarness, XtaskError> {
    family_harness(family).ok_or_else(|| {
        XtaskError::NotImplemented(format!(
            "family `{}` is not registered for `{workflow}`; add an entry to `xtask/src/family/harness.rs` before running family new/prove/certify",
            family.as_str()
        ))
    })
}

pub(crate) fn registered_harnesses_in_routing_order() -> Vec<&'static FamilyHarness> {
    let mut harnesses = FAMILY_REGISTRY.iter().collect::<Vec<_>>();
    harnesses.sort_by_key(|definition| definition.routing.precedence);
    harnesses
}
