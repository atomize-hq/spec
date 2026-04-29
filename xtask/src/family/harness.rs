use crate::XtaskError;
use crate::family::paths::FamilyId;
use crate::family::report::{ArtifactKind, CertificationReport, GateId, SuiteDefinition};

pub(crate) const TERMINAL_UNSUPPORTED_CATCH_ALL: &str = "unsupported.function.v1";

pub const CHAIN3_PRECEDENCE: u64 = 1;
pub const MONOTONE_DOWN_NONNEGATIVE_PRECEDENCE: u64 = 3;

pub const CHAIN3_MUST_NOT_SHADOW: [&str; 3] = [
    "function.wrapper.pipeline.v1",
    "function.arithmetic_leaf.monotone_down_nonnegative.v1",
    "function.arithmetic_leaf.monotone_up.v1",
];
pub const MONOTONE_DOWN_NONNEGATIVE_MUST_NOT_SHADOW: [&str; 1] =
    ["function.arithmetic_leaf.monotone_up.v1"];

pub const CHAIN3_SUITE_SLUG: &str = "m21_chain3_";
pub const MONOTONE_DOWN_NONNEGATIVE_SUITE_SLUG: &str = "monotone_down_nonnegative_";

const CHAIN3_SUMMARY: &str =
    "Straight-line three-call wrapper pipeline over supported function deps.";
const MONOTONE_DOWN_NONNEGATIVE_SUMMARY: &str =
    "Straight-line arithmetic leaf with zero-or-one helper dep and nonnegative clamp semantics.";

const CHAIN3_STARTER_CASES: [StarterCaseDefinition; 16] = [
    StarterCaseDefinition {
        bucket: "aligned",
        path: "fixtures/aligned/units/pricing/pricing_discount_leaf_aligned.unit.spec",
    },
    StarterCaseDefinition {
        bucket: "aligned",
        path: "fixtures/aligned/units/pricing/pricing_tax_leaf_aligned.unit.spec",
    },
    StarterCaseDefinition {
        bucket: "aligned",
        path: "fixtures/aligned/units/pricing/pricing_total_wrapper_aligned.unit.spec",
    },
    StarterCaseDefinition {
        bucket: "aligned",
        path: "fixtures/aligned/units/pricing/checkout_chain3_aligned.unit.spec",
    },
    StarterCaseDefinition {
        bucket: "drift",
        path: "fixtures/drift/units/pricing/pricing_discount_leaf_drift.unit.spec",
    },
    StarterCaseDefinition {
        bucket: "drift",
        path: "fixtures/drift/units/pricing/pricing_tax_leaf_drift.unit.spec",
    },
    StarterCaseDefinition {
        bucket: "drift",
        path: "fixtures/drift/units/pricing/pricing_total_wrapper_drift.unit.spec",
    },
    StarterCaseDefinition {
        bucket: "drift",
        path: "fixtures/drift/units/pricing/checkout_chain3_drift.unit.spec",
    },
    StarterCaseDefinition {
        bucket: "under_specified",
        path: "fixtures/under_specified/units/pricing/pricing_discount_leaf_under_specified.unit.spec",
    },
    StarterCaseDefinition {
        bucket: "under_specified",
        path: "fixtures/under_specified/units/pricing/pricing_tax_leaf_under_specified.unit.spec",
    },
    StarterCaseDefinition {
        bucket: "under_specified",
        path: "fixtures/under_specified/units/pricing/pricing_total_wrapper_under_specified.unit.spec",
    },
    StarterCaseDefinition {
        bucket: "under_specified",
        path: "fixtures/under_specified/units/pricing/checkout_chain3_under_specified.unit.spec",
    },
    StarterCaseDefinition {
        bucket: "unsupported_near_miss",
        path: "fixtures/unsupported_near_miss/units/pricing/pricing_discount_leaf_unsupported_near_miss.unit.spec",
    },
    StarterCaseDefinition {
        bucket: "unsupported_near_miss",
        path: "fixtures/unsupported_near_miss/units/pricing/pricing_tax_leaf_unsupported_near_miss.unit.spec",
    },
    StarterCaseDefinition {
        bucket: "unsupported_near_miss",
        path: "fixtures/unsupported_near_miss/units/pricing/pricing_total_wrapper_unsupported_near_miss.unit.spec",
    },
    StarterCaseDefinition {
        bucket: "unsupported_near_miss",
        path: "fixtures/unsupported_near_miss/units/pricing/checkout_chain3_unsupported_near_miss.unit.spec",
    },
];

const MONOTONE_DOWN_NONNEGATIVE_STARTER_CASES: [StarterCaseDefinition; 4] = [
    StarterCaseDefinition {
        bucket: "aligned",
        path: "fixtures/aligned/units/pricing/apply_discount_aligned.unit.spec",
    },
    StarterCaseDefinition {
        bucket: "drift",
        path: "fixtures/drift/units/pricing/apply_discount_drift.unit.spec",
    },
    StarterCaseDefinition {
        bucket: "under_specified",
        path: "fixtures/under_specified/units/pricing/apply_discount_under_specified.unit.spec",
    },
    StarterCaseDefinition {
        bucket: "unsupported_near_miss",
        path: "fixtures/unsupported_near_miss/units/pricing/apply_discount_control_flow_unsupported_near_miss.unit.spec",
    },
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

pub(crate) const MONOTONE_DOWN_NONNEGATIVE_PROVE_SUITES: [SuiteDefinition; 3] = [
    SuiteDefinition {
        name: "spec-core:monotone_down_nonnegative_classifier_",
        command: &[
            "cargo",
            "test",
            "-p",
            "spec-core",
            "--lib",
            "monotone_down_nonnegative_classifier_",
            "--",
            "--color",
            "never",
        ],
        expected_tests: &[
            "semantic_review::tests::monotone_down_nonnegative_classifier_aligned_fixture_routes_to_promoted_leaf",
            "semantic_review::tests::monotone_down_nonnegative_classifier_drift_fixture_reports_semantic_drift",
            "semantic_review::tests::monotone_down_nonnegative_classifier_under_specified_fixture_reports_vague_truth",
            "semantic_review::tests::monotone_down_nonnegative_classifier_unsupported_near_miss_stays_unsupported",
        ],
    },
    SuiteDefinition {
        name: "spec-cli:monotone_down_nonnegative_truth_surface_",
        command: &[
            "cargo",
            "test",
            "-p",
            "spec-cli",
            "--test",
            "m14_regressions",
            "monotone_down_nonnegative_truth_surface_",
            "--",
            "--color",
            "never",
        ],
        expected_tests: &[
            "monotone_down_nonnegative_truth_surface_command_matrix_preserves_until_spec_test_refresh",
            "monotone_down_nonnegative_truth_surface_stale_status_and_export_preserve_last_proven_review",
        ],
    },
    SuiteDefinition {
        name: "spec-cli:monotone_down_nonnegative_corpus_",
        command: &[
            "cargo",
            "test",
            "-p",
            "spec-cli",
            "--test",
            "m14_regressions",
            "monotone_down_nonnegative_corpus_",
            "--",
            "--color",
            "never",
        ],
        expected_tests: &[
            "monotone_down_nonnegative_corpus_aligned_fixture_projects_valid_state",
            "monotone_down_nonnegative_corpus_drift_fixture_projects_failing_state",
            "monotone_down_nonnegative_corpus_under_specified_fixture_projects_incomplete_state",
            "monotone_down_nonnegative_corpus_unsupported_near_miss_stays_additive_only_and_neutral",
        ],
    },
];

pub(crate) const MONOTONE_DOWN_NONNEGATIVE_CERTIFY_SUITES: [SuiteDefinition; 2] = [
    SuiteDefinition {
        name: "spec-core:monotone_down_nonnegative_regression_",
        command: &[
            "cargo",
            "test",
            "-p",
            "spec-core",
            "--lib",
            "monotone_down_nonnegative_regression_",
            "--",
            "--color",
            "never",
        ],
        expected_tests: &[
            "semantic_review::tests::monotone_down_nonnegative_regression_chain3_is_not_shadowed",
            "semantic_review::tests::monotone_down_nonnegative_regression_monotone_up_is_not_shadowed",
        ],
    },
    SuiteDefinition {
        name: "spec-cli:monotone_down_nonnegative_regression_",
        command: &[
            "cargo",
            "test",
            "-p",
            "spec-cli",
            "--test",
            "m14_regressions",
            "monotone_down_nonnegative_regression_",
            "--",
            "--color",
            "never",
        ],
        expected_tests: &[
            "monotone_down_nonnegative_regression_read_side_surfaces_are_not_shadowed",
            "monotone_down_nonnegative_regression_unsupported_near_miss_stays_additive_only_and_neutral",
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

const MONOTONE_DOWN_NONNEGATIVE_PROVE_SUITE_DEFINITIONS: [ProveSuiteDefinition; 3] = [
    ProveSuiteDefinition {
        suite: MONOTONE_DOWN_NONNEGATIVE_PROVE_SUITES[0],
        gate: GateId::GateA,
    },
    ProveSuiteDefinition {
        suite: MONOTONE_DOWN_NONNEGATIVE_PROVE_SUITES[1],
        gate: GateId::GateC,
    },
    ProveSuiteDefinition {
        suite: MONOTONE_DOWN_NONNEGATIVE_PROVE_SUITES[2],
        gate: GateId::GateB,
    },
];

const CHAIN3_HARNESS: FamilyHarness = FamilyHarness {
    family: "function.wrapper.pipeline.chain3.v1",
    summary: CHAIN3_SUMMARY,
    suite_slug: CHAIN3_SUITE_SLUG,
    scaffold: ScaffoldDefinition {
        unit_namespace: "pricing",
        template: StarterTemplate::GenericPlaceholder,
        starter_cases: &CHAIN3_STARTER_CASES,
    },
    routing: LockedManifestRouting {
        precedence: CHAIN3_PRECEDENCE,
        must_not_shadow: &CHAIN3_MUST_NOT_SHADOW,
    },
    shape: LockedManifestShape {
        dep_min: 3,
        dep_max: 3,
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
    prove_suites: &CHAIN3_PROVE_SUITE_DEFINITIONS,
    certify_suites: &CHAIN3_CERTIFY_SUITES,
};

const MONOTONE_DOWN_NONNEGATIVE_HARNESS: FamilyHarness = FamilyHarness {
    family: "function.arithmetic_leaf.monotone_down_nonnegative.v1",
    summary: MONOTONE_DOWN_NONNEGATIVE_SUMMARY,
    suite_slug: MONOTONE_DOWN_NONNEGATIVE_SUITE_SLUG,
    scaffold: ScaffoldDefinition {
        unit_namespace: "pricing",
        template: StarterTemplate::ArithmeticLeafMonotoneDownNonnegative,
        starter_cases: &MONOTONE_DOWN_NONNEGATIVE_STARTER_CASES,
    },
    routing: LockedManifestRouting {
        precedence: MONOTONE_DOWN_NONNEGATIVE_PRECEDENCE,
        must_not_shadow: &MONOTONE_DOWN_NONNEGATIVE_MUST_NOT_SHADOW,
    },
    shape: LockedManifestShape {
        dep_min: 0,
        dep_max: 1,
        control_flow: "straight_line_only",
        return_style: "let_then_return_or_direct_return",
        loops: false,
        branching: false,
        requires_supported_function_deps: false,
    },
    args: LockedManifestArgs {
        threading: "ordered_passthrough",
        allow_nested_argument_expressions: false,
        allow_literal_only_extra_args: false,
    },
    prove_suites: &MONOTONE_DOWN_NONNEGATIVE_PROVE_SUITE_DEFINITIONS,
    certify_suites: &MONOTONE_DOWN_NONNEGATIVE_CERTIFY_SUITES,
};

const FAMILY_REGISTRY: [FamilyHarness; 2] = [CHAIN3_HARNESS, MONOTONE_DOWN_NONNEGATIVE_HARNESS];

const PROVE_LATEST_REQUIRED_GATES: [GateId; 3] = [GateId::GateA, GateId::GateB, GateId::GateC];
const CERTIFY_REQUIRED_GATES: [GateId; 4] =
    [GateId::GateA, GateId::GateB, GateId::GateC, GateId::GateD];

#[derive(Debug, Clone, Copy)]
pub(crate) struct FamilyHarness {
    pub family: &'static str,
    pub summary: &'static str,
    pub suite_slug: &'static str,
    pub scaffold: ScaffoldDefinition,
    pub routing: LockedManifestRouting,
    pub shape: LockedManifestShape,
    pub args: LockedManifestArgs,
    pub prove_suites: &'static [ProveSuiteDefinition],
    pub certify_suites: &'static [SuiteDefinition],
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct ScaffoldDefinition {
    pub unit_namespace: &'static str,
    pub template: StarterTemplate,
    pub starter_cases: &'static [StarterCaseDefinition],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct StarterCaseDefinition {
    pub bucket: &'static str,
    pub path: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum StarterTemplate {
    GenericPlaceholder,
    ArithmeticLeafMonotoneDownNonnegative,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct LockedManifestShape {
    pub dep_min: u64,
    pub dep_max: u64,
    pub control_flow: &'static str,
    pub return_style: &'static str,
    pub loops: bool,
    pub branching: bool,
    pub requires_supported_function_deps: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct LockedManifestArgs {
    pub threading: &'static str,
    pub allow_nested_argument_expressions: bool,
    pub allow_literal_only_extra_args: bool,
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

impl FamilyHarness {
    pub(crate) fn starter_cases_for_bucket(
        self,
        bucket: &str,
    ) -> impl Iterator<Item = StarterCaseDefinition> {
        self.scaffold
            .starter_cases
            .iter()
            .copied()
            .filter(move |definition| definition.bucket == bucket)
    }
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

#[cfg_attr(not(test), allow(dead_code))]
pub(crate) fn family_harness(family: &FamilyId) -> Option<&'static FamilyHarness> {
    family_harness_in(registered_family_harnesses(), family)
}

pub(crate) fn registered_family_harnesses() -> &'static [FamilyHarness] {
    &FAMILY_REGISTRY
}

pub(crate) fn family_harness_in<'registry>(
    registry: &'registry [FamilyHarness],
    family: &FamilyId,
) -> Option<&'registry FamilyHarness> {
    registry
        .iter()
        .find(|definition| definition.family == family.as_str())
}

pub(crate) fn require_family_harness(
    family: &FamilyId,
    workflow: &'static str,
) -> Result<&'static FamilyHarness, XtaskError> {
    require_family_harness_in(registered_family_harnesses(), family, workflow)
}

pub(crate) fn require_family_harness_in<'registry>(
    registry: &'registry [FamilyHarness],
    family: &FamilyId,
    workflow: &'static str,
) -> Result<&'registry FamilyHarness, XtaskError> {
    family_harness_in(registry, family).ok_or_else(|| {
        XtaskError::NotImplemented(format!(
            "family `{}` is not registered for `{workflow}`; add an entry to `xtask/src/family/harness.rs` before running family new/smoke/prove/certify",
            family.as_str()
        ))
    })
}

pub(crate) fn validate_suite_ownership(
    harness: &FamilyHarness,
    suites: &[SuiteDefinition],
    workflow: &str,
) -> Result<(), XtaskError> {
    for suite in suites {
        if !suite.name.contains(harness.suite_slug) {
            return Err(XtaskError::InvalidInput(format!(
                "{workflow} suite `{}` is not owned by family `{}`; suite names must include `{}`",
                suite.name, harness.family, harness.suite_slug
            )));
        }
        for expected_test in suite.expected_tests {
            if !expected_test.contains(harness.suite_slug) {
                return Err(XtaskError::InvalidInput(format!(
                    "{workflow} suite `{}` includes expected test `{expected_test}` that is not owned by family `{}`; expected test names must include `{}`",
                    suite.name, harness.family, harness.suite_slug
                )));
            }
        }
    }
    Ok(())
}

#[allow(dead_code)]
pub(crate) fn registered_harnesses_in_routing_order() -> Vec<&'static FamilyHarness> {
    registered_harnesses_in_routing_order_from(registered_family_harnesses())
}

pub(crate) fn registered_harnesses_in_routing_order_from(
    registry: &[FamilyHarness],
) -> Vec<&FamilyHarness> {
    let mut harnesses = registry.iter().collect::<Vec<_>>();
    harnesses.sort_by_key(|definition| definition.routing.precedence);
    harnesses
}
