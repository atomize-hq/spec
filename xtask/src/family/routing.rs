use crate::family::harness::{
    FamilyHarness, LockedManifestRouting, TERMINAL_UNSUPPORTED_CATCH_ALL, family_harness,
    family_harness_in, registered_harnesses_in_routing_order_from,
};
use crate::family::manifest::Routing;
use crate::family::paths::FamilyId;
use std::collections::{BTreeMap, BTreeSet};

#[allow(dead_code)]
pub const CHAIN3_PRECEDENCE: u64 = crate::family::harness::CHAIN3_PRECEDENCE;
#[allow(dead_code)]
pub const CHAIN3_MUST_NOT_SHADOW: [&str; 4] = crate::family::harness::CHAIN3_MUST_NOT_SHADOW;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RoutingDiagnostics {
    pub locked_registry_order_with_terminal: Vec<String>,
    pub manifest: ManifestRoutingDiagnostic,
    pub registry: RegistryRoutingDiagnostic,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ManifestRoutingDiagnostic {
    pub passed: bool,
    pub issue: Option<ManifestRoutingIssue>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ManifestRoutingIssue {
    UnknownFamily,
    PrecedenceMismatch {
        expected: u64,
        found: u64,
    },
    MustNotShadowMismatch {
        expected: Vec<String>,
        found: Vec<String>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RegistryRoutingDiagnostic {
    pub passed: bool,
    pub issues: Vec<RegistryRoutingIssue>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum RegistryRoutingIssue {
    DuplicateRegisteredFamilyId {
        family: String,
    },
    DuplicatePrecedence {
        precedence: u64,
        families: Vec<String>,
    },
    MissingRegisteredSuccessor {
        family: String,
        successor: String,
    },
    DuplicateRegisteredSuccessor {
        family: String,
        successor: String,
    },
    RegisteredSuccessorsOutOfOrder {
        family: String,
        expected: Vec<String>,
        found: Vec<String>,
    },
    UnsupportedBeforeRegisteredSuccessor {
        family: String,
    },
    DuplicateUnsupportedTerminal {
        family: String,
    },
}

#[allow(dead_code)]
pub fn locked_routing_order_with_terminal() -> Vec<&'static str> {
    locked_routing_order_with_terminal_from(crate::family::harness::registered_family_harnesses())
}

#[cfg_attr(not(test), allow(dead_code))]
pub(crate) fn locked_routing_order_with_terminal_from(
    registry: &[FamilyHarness],
) -> Vec<&'static str> {
    let mut locked_order = registered_harnesses_in_routing_order_from(registry)
        .into_iter()
        .map(|harness| harness.family)
        .collect::<Vec<_>>();
    locked_order.push(TERMINAL_UNSUPPORTED_CATCH_ALL);
    locked_order
}

#[allow(dead_code)]
pub fn locked_manifest_routing(family: &FamilyId) -> Option<LockedManifestRouting> {
    family_harness(family).map(|harness| harness.routing)
}

#[cfg_attr(not(test), allow(dead_code))]
pub(crate) fn locked_manifest_routing_in(
    registry: &[FamilyHarness],
    family: &FamilyId,
) -> Option<LockedManifestRouting> {
    family_harness_in(registry, family).map(|harness| harness.routing)
}

#[allow(dead_code)]
pub fn manifest_matches_locked_routing(family: &FamilyId, routing: &Routing) -> bool {
    let Some(expected) = locked_manifest_routing(family) else {
        return false;
    };

    manifest_matches_expected_routing(expected, routing)
}

#[allow(dead_code)]
pub(crate) fn manifest_matches_locked_routing_in(
    registry: &[FamilyHarness],
    family: &FamilyId,
    routing: &Routing,
) -> bool {
    let Some(expected) = locked_manifest_routing_in(registry, family) else {
        return false;
    };

    manifest_matches_expected_routing(expected, routing)
}

#[allow(dead_code)]
pub(crate) fn routing_diagnostics(family: &FamilyId, routing: &Routing) -> RoutingDiagnostics {
    routing_diagnostics_in(
        crate::family::harness::registered_family_harnesses(),
        family,
        routing,
    )
}

#[cfg_attr(not(test), allow(dead_code))]
pub(crate) fn routing_diagnostics_in(
    registry: &[FamilyHarness],
    family: &FamilyId,
    routing: &Routing,
) -> RoutingDiagnostics {
    RoutingDiagnostics {
        locked_registry_order_with_terminal: locked_routing_order_with_terminal_from(registry)
            .into_iter()
            .map(ToString::to_string)
            .collect(),
        manifest: manifest_routing_diagnostic_in(registry, family, routing),
        registry: registry_routing_diagnostic_in(registry),
    }
}

#[allow(dead_code)]
fn manifest_matches_expected_routing(expected: LockedManifestRouting, routing: &Routing) -> bool {
    routing.precedence == expected.precedence
        && routing
            .must_not_shadow
            .iter()
            .map(String::as_str)
            .eq(expected.must_not_shadow.iter().copied())
}

fn manifest_routing_diagnostic_in(
    registry: &[FamilyHarness],
    family: &FamilyId,
    routing: &Routing,
) -> ManifestRoutingDiagnostic {
    let Some(expected) = locked_manifest_routing_in(registry, family) else {
        return ManifestRoutingDiagnostic {
            passed: false,
            issue: Some(ManifestRoutingIssue::UnknownFamily),
        };
    };

    if routing.precedence != expected.precedence {
        return ManifestRoutingDiagnostic {
            passed: false,
            issue: Some(ManifestRoutingIssue::PrecedenceMismatch {
                expected: expected.precedence,
                found: routing.precedence,
            }),
        };
    }

    let expected_must_not_shadow = expected
        .must_not_shadow
        .iter()
        .map(|family_id| (*family_id).to_string())
        .collect::<Vec<_>>();
    if routing.must_not_shadow != expected_must_not_shadow {
        return ManifestRoutingDiagnostic {
            passed: false,
            issue: Some(ManifestRoutingIssue::MustNotShadowMismatch {
                expected: expected_must_not_shadow,
                found: routing.must_not_shadow.clone(),
            }),
        };
    }

    ManifestRoutingDiagnostic {
        passed: true,
        issue: None,
    }
}

fn registry_routing_diagnostic_in(registry: &[FamilyHarness]) -> RegistryRoutingDiagnostic {
    let mut issues = Vec::new();

    let mut by_family = BTreeMap::<String, usize>::new();
    let mut by_precedence = BTreeMap::<u64, Vec<String>>::new();
    for harness in registry {
        *by_family.entry(harness.family.to_string()).or_insert(0) += 1;
        by_precedence
            .entry(harness.routing.precedence)
            .or_default()
            .push(harness.family.to_string());
    }

    for (family, count) in by_family {
        if count > 1 {
            issues.push(RegistryRoutingIssue::DuplicateRegisteredFamilyId { family });
        }
    }
    for (precedence, families) in by_precedence {
        if families.len() > 1 {
            issues.push(RegistryRoutingIssue::DuplicatePrecedence {
                precedence,
                families,
            });
        }
    }

    let ordered = registered_harnesses_in_routing_order_from(registry);
    let registered_families = ordered
        .iter()
        .map(|harness| harness.family)
        .collect::<Vec<_>>();
    let registered_family_set = registered_families.iter().copied().collect::<BTreeSet<_>>();

    for (index, harness) in ordered.iter().enumerate() {
        let family = harness.family.to_string();
        let expected_registered_successors = registered_families[index + 1..]
            .iter()
            .map(|family_id| (*family_id).to_string())
            .collect::<Vec<_>>();

        let observed_registered = harness
            .routing
            .must_not_shadow
            .iter()
            .copied()
            .filter(|family_id| registered_family_set.contains(family_id))
            .map(ToString::to_string)
            .collect::<Vec<_>>();
        let observed_registered_set = observed_registered.iter().cloned().collect::<BTreeSet<_>>();

        let mut seen_registered = BTreeSet::new();
        for successor in &observed_registered {
            if !seen_registered.insert(successor.clone()) {
                issues.push(RegistryRoutingIssue::DuplicateRegisteredSuccessor {
                    family: family.clone(),
                    successor: successor.clone(),
                });
            }
        }

        for successor in &expected_registered_successors {
            if !observed_registered_set.contains(successor) {
                issues.push(RegistryRoutingIssue::MissingRegisteredSuccessor {
                    family: family.clone(),
                    successor: successor.clone(),
                });
            }
        }

        let unique_observed_registered =
            observed_registered
                .iter()
                .fold(Vec::<String>::new(), |mut acc, successor| {
                    if acc.last() != Some(successor)
                        && !acc.iter().any(|existing| existing == successor)
                    {
                        acc.push(successor.clone());
                    }
                    acc
                });
        if unique_observed_registered != expected_registered_successors {
            issues.push(RegistryRoutingIssue::RegisteredSuccessorsOutOfOrder {
                family: family.clone(),
                expected: expected_registered_successors.clone(),
                found: unique_observed_registered,
            });
        }

        let unsupported_positions = harness
            .routing
            .must_not_shadow
            .iter()
            .enumerate()
            .filter_map(|(position, family_id)| {
                (*family_id == TERMINAL_UNSUPPORTED_CATCH_ALL).then_some(position)
            })
            .collect::<Vec<_>>();
        if unsupported_positions.len() > 1 {
            issues.push(RegistryRoutingIssue::DuplicateUnsupportedTerminal {
                family: family.clone(),
            });
        }
        if let Some(first_unsupported_position) = unsupported_positions.first().copied() {
            let has_registered_successor_after_unsupported = harness.routing.must_not_shadow
                [first_unsupported_position + 1..]
                .iter()
                .copied()
                .any(|family_id| registered_family_set.contains(family_id));
            if has_registered_successor_after_unsupported {
                issues.push(RegistryRoutingIssue::UnsupportedBeforeRegisteredSuccessor {
                    family: family.clone(),
                });
            }
        }
    }

    RegistryRoutingDiagnostic {
        passed: issues.is_empty(),
        issues,
    }
}
