use crate::graph::ImpactSet;
use crate::types::{DepRef, Intent, LoadedMoleculeTest, LoadedSpec};
use crate::{Result, SpecError};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_yaml_bw::Value as YamlValue;
use std::collections::{BTreeSet, HashSet};
use std::sync::OnceLock;

const PLAN_SCHEMA_JSON: &str = include_str!("schema/plan.spec.json");
static COMPILED_PLAN_SCHEMA: OnceLock<jsonschema::Validator> = OnceLock::new();

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlanStruct {
    pub id: String,
    pub intent: Intent,
    pub changes: Vec<PlanChange>,
    #[serde(default)]
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlanChange {
    pub unit: String,
    pub action: PlanChangeAction,
    pub acceptance: PlanAcceptance,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlanAcceptance {
    #[serde(default)]
    pub validate: Vec<String>,
    #[serde(default)]
    pub molecule_tests: Vec<String>,
    #[serde(default)]
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum PlanChangeAction {
    Add,
    Modify,
    Remove,
}

impl PlanChangeAction {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Add => "add",
            Self::Modify => "modify",
            Self::Remove => "remove",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PlanSource {
    pub file_path: String,
    pub id: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LoadedPlan {
    pub source: PlanSource,
    pub plan: PlanStruct,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PlanComputedImpact {
    pub status: PlanComputedImpactStatus,
    pub units: Vec<String>,
    pub molecule_tests: Vec<String>,
    pub unresolved: Vec<PlanUnresolvedImpact>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum PlanComputedImpactStatus {
    Complete,
    Partial,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct PlanUnresolvedImpact {
    pub unit: String,
    pub action: PlanChangeAction,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlanChangeReport {
    pub unit: String,
    pub action: PlanChangeAction,
    pub impact: Option<ImpactSet>,
    pub unresolved_reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlanReport {
    pub plan_id: String,
    pub computed_impact: PlanComputedImpact,
    pub change_reports: Vec<PlanChangeReport>,
}

pub fn validate_raw_plan_yaml(yaml_value: &YamlValue, file_path: &str) -> Result<()> {
    let plan_json = serde_json::to_value(yaml_value).map_err(SpecError::Json)?;
    validate_plan_json_value(&plan_json, file_path)
}

pub fn build_plan_report(
    loaded_plan: &LoadedPlan,
    specs: &[LoadedSpec],
    molecule_tests: &[LoadedMoleculeTest],
) -> Result<PlanReport> {
    let graph = crate::graph::SpecGraph::build(specs, molecule_tests);
    let current_units: HashSet<&str> = specs.iter().map(|spec| spec.spec.id.as_str()).collect();
    let known_tests: HashSet<&str> = molecule_tests
        .iter()
        .map(|test| test.test.id.as_str())
        .collect();

    let mut seen_change_units = HashSet::new();
    let mut union_units = BTreeSet::new();
    let mut union_molecule_tests = BTreeSet::new();
    let mut unresolved = BTreeSet::new();
    let mut change_reports = Vec::with_capacity(loaded_plan.plan.changes.len());

    for change in &loaded_plan.plan.changes {
        if !seen_change_units.insert(change.unit.as_str()) {
            return Err(SpecError::PlanDuplicateChangeUnit {
                unit: change.unit.clone(),
                path: loaded_plan.source.file_path.clone(),
            });
        }

        validate_local_plan_unit_ref(&change.unit, &loaded_plan.source.file_path)?;

        for unit in &change.acceptance.validate {
            validate_local_plan_unit_ref(unit, &loaded_plan.source.file_path)?;
        }

        for test_id in &change.acceptance.molecule_tests {
            if !known_tests.contains(test_id.as_str()) {
                return Err(SpecError::PlanMoleculeTestNotFound {
                    test_id: test_id.clone(),
                    path: loaded_plan.source.file_path.clone(),
                });
            }
        }

        match change.action {
            PlanChangeAction::Modify | PlanChangeAction::Remove => {
                if !current_units.contains(change.unit.as_str()) {
                    return Err(SpecError::PlanUnitMissingForAction {
                        unit: change.unit.clone(),
                        action: change.action.as_str().to_string(),
                        path: loaded_plan.source.file_path.clone(),
                    });
                }

                let impact = graph.impact(&change.unit).expect("known units have impact");
                union_units.extend(impact.units.iter().cloned());
                union_molecule_tests.extend(impact.molecule_tests.iter().cloned());
                change_reports.push(PlanChangeReport {
                    unit: change.unit.clone(),
                    action: change.action.clone(),
                    impact: Some(impact),
                    unresolved_reason: None,
                });
            }
            PlanChangeAction::Add => {
                if current_units.contains(change.unit.as_str()) {
                    return Err(SpecError::PlanUnitAlreadyExistsForAdd {
                        unit: change.unit.clone(),
                        path: loaded_plan.source.file_path.clone(),
                    });
                }

                let reason = "current graph has no node for action=add".to_string();
                unresolved.insert(PlanUnresolvedImpact {
                    unit: change.unit.clone(),
                    action: change.action.clone(),
                    reason: reason.clone(),
                });
                change_reports.push(PlanChangeReport {
                    unit: change.unit.clone(),
                    action: change.action.clone(),
                    impact: None,
                    unresolved_reason: Some(reason),
                });
            }
        }
    }

    Ok(PlanReport {
        plan_id: loaded_plan.plan.id.clone(),
        computed_impact: PlanComputedImpact {
            status: if unresolved.is_empty() {
                PlanComputedImpactStatus::Complete
            } else {
                PlanComputedImpactStatus::Partial
            },
            units: union_units.into_iter().collect(),
            molecule_tests: union_molecule_tests.into_iter().collect(),
            unresolved: unresolved.into_iter().collect(),
        },
        change_reports,
    })
}

fn validate_local_plan_unit_ref(unit: &str, file_path: &str) -> Result<()> {
    let dep_ref = DepRef::parse(unit).map_err(|err| SpecError::SemanticValidation {
        message: err.to_string(),
        path: file_path.to_string(),
    })?;
    if dep_ref.library_alias().is_some() {
        return Err(SpecError::PlanCrossLibraryUnit {
            unit: unit.to_string(),
            path: file_path.to_string(),
        });
    }
    Ok(())
}

fn compiled_plan_schema() -> Result<&'static jsonschema::Validator> {
    if let Some(schema) = COMPILED_PLAN_SCHEMA.get() {
        return Ok(schema);
    }

    let schema_json: Value = serde_json::from_str(PLAN_SCHEMA_JSON).map_err(SpecError::Json)?;
    let schema =
        jsonschema::draft7::new(&schema_json).map_err(|e| SpecError::SchemaValidation {
            message: format!("Plan schema compilation failed: {e}"),
            path: "<plan.spec schema>".to_string(),
        })?;

    let _ = COMPILED_PLAN_SCHEMA.set(schema);

    Ok(COMPILED_PLAN_SCHEMA
        .get()
        .expect("COMPILED_PLAN_SCHEMA must be set after successful compilation"))
}

fn validate_plan_json_value(plan_json: &Value, file_path: &str) -> Result<()> {
    let schema = compiled_plan_schema()?;
    match schema.validate(plan_json) {
        Ok(()) => Ok(()),
        Err(error) => Err(SpecError::SchemaValidation {
            message: humanize_validation_error(&error),
            path: file_path.to_string(),
        }),
    }
}

fn humanize_validation_error(error: &jsonschema::ValidationError<'_>) -> String {
    use jsonschema::error::ValidationErrorKind;

    let field_path = error.instance_path.to_string();
    let field_label = if field_path.is_empty() || field_path == "/" {
        String::new()
    } else {
        format!(" at {field_path}")
    };

    match &error.kind {
        ValidationErrorKind::Required { property } => {
            format!("missing required field: {}{}", property, field_label)
        }
        ValidationErrorKind::AdditionalProperties { unexpected } => {
            let fields = unexpected.to_vec().join(", ");
            format!("unknown field{}: {}", field_label, fields)
        }
        ValidationErrorKind::Enum { .. } => {
            format!(
                "invalid value{}: {} — check allowed values",
                field_label, error
            )
        }
        ValidationErrorKind::Pattern { .. } => {
            format!("invalid format{}: {}", field_label, error)
        }
        _ => {
            if field_path.is_empty() {
                error.to_string()
            } else {
                format!("{} (at {})", error, field_path)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::loader::load_plan_file;
    use crate::types::{Body, MoleculeTestSource, MoleculeTestStruct, SpecSource, SpecStruct};
    use tempfile::TempDir;

    fn write_file(dir: &TempDir, rel: &str, body: &str) -> std::path::PathBuf {
        let path = dir.path().join(rel);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        std::fs::write(&path, body).unwrap();
        path
    }

    fn loaded_spec(id: &str, deps: Vec<&str>) -> LoadedSpec {
        LoadedSpec {
            source: SpecSource {
                file_path: format!("{id}.unit.spec"),
                id: id.to_string(),
            },
            spec: SpecStruct {
                id: id.to_string(),
                kind: "function".to_string(),
                intent: Intent {
                    why: format!("Why {id}"),
                },
                contract: None,
                deps: deps.into_iter().map(str::to_string).collect(),
                imports: vec![],
                body: Body {
                    rust: "{ true }".to_string(),
                },
                local_tests: vec![],
                links: None,
                spec_version: None,
            },
        }
    }

    fn loaded_test(id: &str, covers: Vec<&str>) -> LoadedMoleculeTest {
        LoadedMoleculeTest {
            source: MoleculeTestSource {
                file_path: format!("{id}.test.spec"),
                id: id.to_string(),
            },
            test: MoleculeTestStruct {
                id: id.to_string(),
                intent: Intent {
                    why: format!("Why {id}"),
                },
                covers: covers.into_iter().map(str::to_string).collect(),
                body: Body {
                    rust: "{ assert!(true); }".to_string(),
                },
                spec_version: None,
            },
        }
    }

    #[test]
    fn load_plan_file_rejects_unknown_fields() {
        let dir = TempDir::new().unwrap();
        let path = write_file(
            &dir,
            "plans/tax.plan.spec",
            r#"
id: checkout-tax-refactor
intent:
  why: "Refactor tax calculation."
changes:
  - unit: pricing/apply_tax
    action: modify
    acceptance:
      validate:
        - pricing/apply_tax
      molecule_tests:
        - pricing/checkout_flow
bogus: nope
"#,
        );

        let err = load_plan_file(&path).unwrap_err();
        match err {
            SpecError::SchemaValidation { message, .. } => {
                assert!(message.contains("unknown field"), "{message}");
            }
            other => panic!("expected schema validation error, got {other:?}"),
        }
    }

    #[test]
    fn build_plan_report_rejects_duplicate_change_units() {
        let loaded_plan = LoadedPlan {
            source: PlanSource {
                file_path: "dup.plan.spec".to_string(),
                id: "dup".to_string(),
            },
            plan: PlanStruct {
                id: "dup".to_string(),
                intent: Intent {
                    why: "duplicate".to_string(),
                },
                changes: vec![
                    PlanChange {
                        unit: "pricing/apply_tax".to_string(),
                        action: PlanChangeAction::Modify,
                        acceptance: PlanAcceptance {
                            validate: vec![],
                            molecule_tests: vec![],
                            notes: vec![],
                        },
                    },
                    PlanChange {
                        unit: "pricing/apply_tax".to_string(),
                        action: PlanChangeAction::Remove,
                        acceptance: PlanAcceptance {
                            validate: vec![],
                            molecule_tests: vec![],
                            notes: vec![],
                        },
                    },
                ],
                notes: vec![],
            },
        };

        let err = build_plan_report(&loaded_plan, &[loaded_spec("pricing/apply_tax", vec![])], &[])
            .unwrap_err();
        match err {
            SpecError::PlanDuplicateChangeUnit { unit, .. } => {
                assert_eq!(unit, "pricing/apply_tax");
            }
            other => panic!("expected duplicate change error, got {other:?}"),
        }
    }

    #[test]
    fn build_plan_report_rejects_missing_modify_unit() {
        let loaded_plan = LoadedPlan {
            source: PlanSource {
                file_path: "missing.plan.spec".to_string(),
                id: "missing".to_string(),
            },
            plan: PlanStruct {
                id: "missing".to_string(),
                intent: Intent {
                    why: "missing".to_string(),
                },
                changes: vec![PlanChange {
                    unit: "pricing/apply_tax".to_string(),
                    action: PlanChangeAction::Modify,
                    acceptance: PlanAcceptance {
                        validate: vec![],
                        molecule_tests: vec![],
                        notes: vec![],
                    },
                }],
                notes: vec![],
            },
        };

        let err = build_plan_report(&loaded_plan, &[], &[]).unwrap_err();
        match err {
            SpecError::PlanUnitMissingForAction { unit, action, .. } => {
                assert_eq!(unit, "pricing/apply_tax");
                assert_eq!(action, "modify");
            }
            other => panic!("expected missing-unit error, got {other:?}"),
        }
    }

    #[test]
    fn build_plan_report_rejects_existing_add_unit() {
        let loaded_plan = LoadedPlan {
            source: PlanSource {
                file_path: "exists.plan.spec".to_string(),
                id: "exists".to_string(),
            },
            plan: PlanStruct {
                id: "exists".to_string(),
                intent: Intent {
                    why: "exists".to_string(),
                },
                changes: vec![PlanChange {
                    unit: "pricing/tiered_rate".to_string(),
                    action: PlanChangeAction::Add,
                    acceptance: PlanAcceptance {
                        validate: vec![],
                        molecule_tests: vec![],
                        notes: vec![],
                    },
                }],
                notes: vec![],
            },
        };

        let err =
            build_plan_report(&loaded_plan, &[loaded_spec("pricing/tiered_rate", vec![])], &[])
                .unwrap_err();
        match err {
            SpecError::PlanUnitAlreadyExistsForAdd { unit, .. } => {
                assert_eq!(unit, "pricing/tiered_rate");
            }
            other => panic!("expected existing-add error, got {other:?}"),
        }
    }

    #[test]
    fn build_plan_report_dedupes_union_impact_and_marks_add_unresolved() {
        let loaded_plan = LoadedPlan {
            source: PlanSource {
                file_path: "mixed.plan.spec".to_string(),
                id: "mixed".to_string(),
            },
            plan: PlanStruct {
                id: "mixed".to_string(),
                intent: Intent {
                    why: "mixed".to_string(),
                },
                changes: vec![
                    PlanChange {
                        unit: "pricing/apply_tax".to_string(),
                        action: PlanChangeAction::Modify,
                        acceptance: PlanAcceptance {
                            validate: vec!["pricing/apply_tax".to_string()],
                            molecule_tests: vec!["pricing/checkout_flow".to_string()],
                            notes: vec![],
                        },
                    },
                    PlanChange {
                        unit: "pricing/tiered_rate".to_string(),
                        action: PlanChangeAction::Add,
                        acceptance: PlanAcceptance {
                            validate: vec!["pricing/tiered_rate".to_string()],
                            molecule_tests: vec![],
                            notes: vec![],
                        },
                    },
                ],
                notes: vec![],
            },
        };
        let specs = vec![
            loaded_spec("pricing/apply_tax", vec![]),
            loaded_spec("pricing/calculate_total", vec!["pricing/apply_tax"]),
        ];
        let tests = vec![loaded_test("pricing/checkout_flow", vec!["pricing/calculate_total"])];

        let report = build_plan_report(&loaded_plan, &specs, &tests).unwrap();

        assert_eq!(report.plan_id, "mixed");
        assert_eq!(report.computed_impact.status, PlanComputedImpactStatus::Partial);
        assert_eq!(
            report.computed_impact.units,
            vec!["pricing/apply_tax".to_string(), "pricing/calculate_total".to_string()]
        );
        assert_eq!(
            report.computed_impact.molecule_tests,
            vec!["pricing/checkout_flow".to_string()]
        );
        assert_eq!(report.computed_impact.unresolved.len(), 1);
        assert_eq!(report.computed_impact.unresolved[0].unit, "pricing/tiered_rate");
        assert_eq!(
            report.computed_impact.unresolved[0].reason,
            "current graph has no node for action=add"
        );
    }

    #[test]
    fn build_plan_report_rejects_missing_molecule_acceptance_target() {
        let loaded_plan = LoadedPlan {
            source: PlanSource {
                file_path: "missing-test.plan.spec".to_string(),
                id: "missing-test".to_string(),
            },
            plan: PlanStruct {
                id: "missing-test".to_string(),
                intent: Intent {
                    why: "missing test".to_string(),
                },
                changes: vec![PlanChange {
                    unit: "pricing/apply_tax".to_string(),
                    action: PlanChangeAction::Modify,
                    acceptance: PlanAcceptance {
                        validate: vec![],
                        molecule_tests: vec!["pricing/checkout_flow".to_string()],
                        notes: vec![],
                    },
                }],
                notes: vec![],
            },
        };

        let err =
            build_plan_report(&loaded_plan, &[loaded_spec("pricing/apply_tax", vec![])], &[])
                .unwrap_err();
        match err {
            SpecError::PlanMoleculeTestNotFound { test_id, .. } => {
                assert_eq!(test_id, "pricing/checkout_flow");
            }
            other => panic!("expected missing molecule test error, got {other:?}"),
        }
    }
}
