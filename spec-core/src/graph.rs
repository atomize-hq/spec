//! Minimal SpecGraph: captures the unit/test/edge world model.
//!
//! This module provides the foundation for M8's full graph layer.
//! It models units, molecule tests, and the edges between them (dep and covers).

use crate::types::{
    DepRef, LoadedMoleculeTest, LoadedSpec, NormalizedUnit, ResolvedSpec, UnitKind,
    ordered_unique_deps, type_name_for_unit_id,
};
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, VecDeque};

/// A node representing a unit in the spec graph
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnitNode {
    pub id: String,
    pub deps: Vec<String>,
}

/// A node representing a molecule test in the spec graph
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MoleculeTestNode {
    pub id: String,
    pub covers: Vec<String>,
}

/// An edge in the spec graph, either a dependency or a covers relationship
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpecEdge {
    /// A unit depends on another unit
    Dep { from: String, to: DepRef },
    /// A molecule test covers a unit
    Covers { test: String, unit: String },
}

impl SpecEdge {
    fn sort_key(&self) -> (u8, &str, String) {
        match self {
            Self::Dep { from, to } => (0, from.as_str(), to.authored()),
            Self::Covers { test, unit } => (1, test.as_str(), unit.clone()),
        }
    }
}

impl PartialOrd for SpecEdge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SpecEdge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.sort_key().cmp(&other.sort_key())
    }
}

/// The local declared blast radius for a seed unit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImpactSet {
    pub units: Vec<String>,
    pub molecule_tests: Vec<String>,
}

/// Kind-aware projection reused across graph, CLI, and molecule codegen call paths.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnitProjection {
    top_level_deps: Vec<String>,
    cover_imports: Vec<String>,
}

impl UnitProjection {
    fn new(top_level_deps: Vec<String>, cover_imports: Vec<String>) -> Self {
        Self {
            top_level_deps,
            cover_imports,
        }
    }

    pub fn top_level_deps(&self) -> &[String] {
        &self.top_level_deps
    }

    pub fn local_dep_ids(&self) -> Vec<String> {
        self.top_level_deps
            .iter()
            .filter_map(|dep| DepRef::parse(dep).ok())
            .filter(|dep| dep.library_alias().is_none())
            .map(|dep| dep.unit_id().to_string())
            .collect()
    }

    pub fn cover_imports(&self) -> &[String] {
        &self.cover_imports
    }
}

pub enum ProjectedUnitRef<'a> {
    Loaded(&'a LoadedSpec),
    Normalized(&'a NormalizedUnit),
}

pub fn project_unit(unit: ProjectedUnitRef<'_>) -> UnitProjection {
    match unit {
        ProjectedUnitRef::Loaded(spec) => match spec.spec.unit_kind() {
            Ok(UnitKind::Data) => UnitProjection::new(
                projected_data_method_deps(
                    spec.spec
                        .extensions
                        .methods
                        .iter()
                        .flat_map(|method| method.deps.iter().map(String::as_str)),
                ),
                vec![data_cover_import_path(&spec.spec.id)],
            ),
            _ => UnitProjection::new(
                spec.spec.deps.clone(),
                function_cover_import_paths(&spec.spec.id, &spec.spec.imports),
            ),
        },
        ProjectedUnitRef::Normalized(unit) => match unit {
            NormalizedUnit::Function(spec) => UnitProjection::new(
                spec.deps.clone(),
                function_cover_import_paths(&spec.id, &spec.imports),
            ),
            NormalizedUnit::Data(unit) => {
                UnitProjection::new(unit.deps.clone(), vec![data_cover_import_path(&unit.id)])
            }
        },
    }
}

fn projected_data_method_deps<'a>(deps: impl Iterator<Item = &'a str>) -> Vec<String> {
    ordered_unique_deps(deps)
}

fn function_cover_import_paths(unit_id: &str, imports: &[String]) -> Vec<String> {
    let mut cover_imports = imports.to_vec();
    cover_imports.push(ResolvedSpec::dep_to_use_path(unit_id));
    cover_imports
}

fn data_cover_import_path(unit_id: &str) -> String {
    format!(
        "crate::{}::{};",
        unit_id.replace('/', "::"),
        type_name_for_unit_id(unit_id)
    )
}

/// The full spec graph: units, molecule tests, and edges between them
#[derive(Debug, Clone)]
pub struct SpecGraph {
    units: Vec<UnitNode>,
    molecule_tests: Vec<MoleculeTestNode>,
    edges: Vec<SpecEdge>,
    rev_dep_index: HashMap<String, Vec<String>>,
    test_coverage_index: HashMap<String, Vec<String>>,
}

impl SpecGraph {
    /// Build a SpecGraph from loaded units and molecule tests.
    ///
    /// Assumes validated input: all dep IDs and covers IDs already exist in the
    /// loaded spec set. This constructor does not read `links.molecule_tests`;
    /// `.unit.spec` `deps` and `.test.spec` `covers` are the only relationship
    /// sources of truth in M8.
    pub fn build(units: &[LoadedSpec], molecule_tests: &[LoadedMoleculeTest]) -> Self {
        let mut unit_nodes: Vec<_> = units
            .iter()
            .map(|u| UnitNode {
                id: u.spec.id.clone(),
                deps: top_level_deps(u),
            })
            .collect();
        unit_nodes.sort_by(|left, right| left.id.cmp(&right.id));

        let mut molecule_test_nodes: Vec<_> = molecule_tests
            .iter()
            .map(|t| MoleculeTestNode {
                id: t.test.id.clone(),
                covers: t.test.covers.clone(),
            })
            .collect();
        molecule_test_nodes.sort_by(|left, right| left.id.cmp(&right.id));

        let mut edges = Vec::new();
        let mut rev_dep_index: HashMap<String, Vec<String>> = HashMap::new();
        let mut test_coverage_index: HashMap<String, Vec<String>> = HashMap::new();

        for u in units {
            for dep in top_level_deps(u) {
                let dep_ref =
                    DepRef::parse(&dep).expect("SpecGraph::build assumes validated dep refs");
                edges.push(SpecEdge::Dep {
                    from: u.spec.id.clone(),
                    to: dep_ref.clone(),
                });
                if dep_ref.library_alias().is_none() {
                    rev_dep_index
                        .entry(dep_ref.unit_id().to_string())
                        .or_default()
                        .push(u.spec.id.clone());
                }
            }
        }

        for t in molecule_tests {
            // Legacy `links.molecule_tests` on units is intentionally ignored.
            // Coverage edges come only from authored `.test.spec` `covers`.
            for unit_id in &t.test.covers {
                edges.push(SpecEdge::Covers {
                    test: t.test.id.clone(),
                    unit: unit_id.clone(),
                });
                test_coverage_index
                    .entry(unit_id.clone())
                    .or_default()
                    .push(t.test.id.clone());
            }
        }

        edges.sort();

        for dependents in rev_dep_index.values_mut() {
            dependents.sort();
            dependents.dedup();
        }

        for tests in test_coverage_index.values_mut() {
            tests.sort();
            tests.dedup();
        }

        Self {
            units: unit_nodes,
            molecule_tests: molecule_test_nodes,
            edges,
            rev_dep_index,
            test_coverage_index,
        }
    }

    pub fn units(&self) -> &[UnitNode] {
        &self.units
    }

    pub fn molecule_tests(&self) -> &[MoleculeTestNode] {
        &self.molecule_tests
    }

    pub fn edges(&self) -> &[SpecEdge] {
        &self.edges
    }

    pub fn reverse_deps(&self, unit_id: &str) -> Option<Vec<String>> {
        self.ensure_known_unit(unit_id)
            .then(|| self.rev_dep_index.get(unit_id).cloned().unwrap_or_default())
    }

    pub fn tests_covering(&self, unit_id: &str) -> Option<Vec<String>> {
        self.ensure_known_unit(unit_id).then(|| {
            self.test_coverage_index
                .get(unit_id)
                .cloned()
                .unwrap_or_default()
        })
    }

    pub fn impact(&self, unit_id: &str) -> Option<ImpactSet> {
        if !self.ensure_known_unit(unit_id) {
            return None;
        }

        let mut visited: HashSet<String> = HashSet::from([unit_id.to_string()]);
        let mut queue = VecDeque::from([unit_id.to_string()]);

        while let Some(current) = queue.pop_front() {
            if let Some(dependents) = self.rev_dep_index.get(&current) {
                for dependent in dependents {
                    if visited.insert(dependent.clone()) {
                        queue.push_back(dependent.clone());
                    }
                }
            }
        }

        let mut units: Vec<String> = visited.into_iter().collect();
        units.sort();

        let mut molecule_tests: Vec<String> = units
            .iter()
            .filter_map(|id| self.test_coverage_index.get(id))
            .flat_map(|tests| tests.iter().cloned())
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();
        molecule_tests.sort();

        Some(ImpactSet {
            units,
            molecule_tests,
        })
    }

    fn ensure_known_unit(&self, unit_id: &str) -> bool {
        self.units
            .binary_search_by(|node| node.id.as_str().cmp(unit_id))
            .is_ok()
    }
}

pub fn top_level_deps(spec: &LoadedSpec) -> Vec<String> {
    project_unit(ProjectedUnitRef::Loaded(spec))
        .top_level_deps()
        .to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{
        AuthoredBackends, AuthoredConstructor, AuthoredDataShape, AuthoredField, AuthoredMethod,
        AuthoredMethodLowering, AuthoredRustBackend, AuthoredRustMethodLowering, Body, Contract,
        Intent, Links, MoleculeTestSource, MoleculeTestStruct, SpecSource, SpecStruct,
        UnitExtensions,
    };
    use indexmap::IndexMap;

    fn make_loaded_spec(id: &str, deps: Vec<&str>) -> LoadedSpec {
        make_loaded_spec_with_links(id, deps, None)
    }

    fn make_loaded_spec_with_links(
        id: &str,
        deps: Vec<&str>,
        links: Option<Vec<&str>>,
    ) -> LoadedSpec {
        LoadedSpec {
            source: SpecSource {
                file_path: format!("{}.unit.spec", id),
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
                    rust: "{ }".to_string(),
                },
                local_tests: vec![],
                links: links.map(|molecule_tests| Links {
                    molecule_tests: molecule_tests.into_iter().map(str::to_string).collect(),
                }),
                spec_version: None,
                extensions: crate::types::UnitExtensions::default(),
            },
        }
    }

    fn make_loaded_molecule_test(id: &str, covers: Vec<&str>) -> LoadedMoleculeTest {
        LoadedMoleculeTest {
            source: MoleculeTestSource {
                file_path: format!("{}.test.spec", id),
                id: id.to_string(),
            },
            test: MoleculeTestStruct {
                id: id.to_string(),
                intent: Intent {
                    why: format!("Why {id}"),
                },
                covers: covers.into_iter().map(str::to_string).collect(),
                imports: None,
                body: Body {
                    rust: "{ assert!(true); }".to_string(),
                },
                spec_version: None,
            },
        }
    }

    fn make_loaded_data_seam(id: &str) -> LoadedSpec {
        LoadedSpec {
            source: SpecSource {
                file_path: format!("{id}.unit.spec"),
                id: id.to_string(),
            },
            spec: SpecStruct {
                id: id.to_string(),
                kind: "data".to_string(),
                intent: Intent {
                    why: format!("Why {id}"),
                },
                contract: None,
                deps: vec!["legacy/ignored".to_string()],
                imports: vec![],
                body: Body::default(),
                local_tests: vec![],
                links: None,
                spec_version: None,
                extensions: UnitExtensions {
                    data: Some(AuthoredDataShape {
                        fields: IndexMap::from([(
                            "subtotal".to_string(),
                            AuthoredField {
                                type_: "Decimal".to_string(),
                            },
                        )]),
                    }),
                    constructors: vec![AuthoredConstructor {
                        id: "new".to_string(),
                        intent: Intent {
                            why: "Create a quote".to_string(),
                        },
                        contract: Some(Contract {
                            inputs: Some(IndexMap::from([(
                                "subtotal".to_string(),
                                "Decimal".to_string(),
                            )])),
                            returns: None,
                            invariants: vec![],
                        }),
                        initializes: IndexMap::from([(
                            "subtotal".to_string(),
                            "subtotal".to_string(),
                        )]),
                    }],
                    methods: vec![
                        AuthoredMethod {
                            id: "discounted_subtotal".to_string(),
                            intent: Intent {
                                why: "Compute discounted subtotal".to_string(),
                            },
                            receiver: "shared_ref".to_string(),
                            contract: Some(Contract {
                                inputs: None,
                                returns: Some("Decimal".to_string()),
                                invariants: vec![],
                            }),
                            deps: vec!["pricing/apply_discount".to_string()],
                            lowering: Some(AuthoredMethodLowering {
                                rust: Some(AuthoredRustMethodLowering {
                                    body: "{ apply_discount(self.subtotal, Decimal::ZERO) }"
                                        .to_string(),
                                }),
                            }),
                        },
                        AuthoredMethod {
                            id: "total".to_string(),
                            intent: Intent {
                                why: "Compute total".to_string(),
                            },
                            receiver: "shared_ref".to_string(),
                            contract: Some(Contract {
                                inputs: None,
                                returns: Some("Decimal".to_string()),
                                invariants: vec![],
                            }),
                            deps: vec![
                                "pricing/apply_discount".to_string(),
                                "pricing/apply_tax".to_string(),
                            ],
                            lowering: Some(AuthoredMethodLowering {
                                rust: Some(AuthoredRustMethodLowering {
                                    body: "{ apply_tax(self.subtotal, Decimal::ONE) }".to_string(),
                                }),
                            }),
                        },
                    ],
                    backends: Some(AuthoredBackends {
                        rust: Some(AuthoredRustBackend {
                            derives: vec!["Clone".to_string()],
                        }),
                    }),
                },
            },
        }
    }

    #[test]
    fn build_graph_creates_dep_and_covers_edges() {
        let units = vec![
            make_loaded_spec("money/round", vec![]),
            make_loaded_spec("pricing/apply_discount", vec!["money/round"]),
        ];
        let molecule_tests = vec![
            make_loaded_molecule_test("pricing/z_test", vec!["pricing/apply_discount"]),
            make_loaded_molecule_test("pricing/a_test", vec!["money/round"]),
        ];

        let graph = SpecGraph::build(&units, &molecule_tests);

        assert_eq!(
            graph
                .units()
                .iter()
                .map(|node| node.id.as_str())
                .collect::<Vec<_>>(),
            vec!["money/round", "pricing/apply_discount"]
        );
        assert_eq!(
            graph
                .molecule_tests()
                .iter()
                .map(|node| node.id.as_str())
                .collect::<Vec<_>>(),
            vec!["pricing/a_test", "pricing/z_test"]
        );
        assert_eq!(
            graph.edges(),
            [
                SpecEdge::Dep {
                    from: "pricing/apply_discount".to_string(),
                    to: DepRef::local("money/round"),
                },
                SpecEdge::Covers {
                    test: "pricing/a_test".to_string(),
                    unit: "money/round".to_string(),
                },
                SpecEdge::Covers {
                    test: "pricing/z_test".to_string(),
                    unit: "pricing/apply_discount".to_string(),
                },
            ]
            .as_slice()
        );
    }

    #[test]
    fn top_level_deps_dedupes_reused_data_method_deps() {
        let seam = make_loaded_data_seam("pricing/checkout_quote");

        assert_eq!(
            top_level_deps(&seam),
            vec![
                "pricing/apply_discount".to_string(),
                "pricing/apply_tax".to_string(),
            ]
        );
    }

    #[test]
    fn build_graph_empty_inputs() {
        let graph = SpecGraph::build(&[], &[]);
        assert!(graph.units().is_empty());
        assert!(graph.molecule_tests().is_empty());
        assert!(graph.edges().is_empty());
        assert_eq!(graph.reverse_deps("missing/unit"), None);
        assert_eq!(graph.tests_covering("missing/unit"), None);
        assert_eq!(graph.impact("missing/unit"), None);
    }

    #[test]
    fn reverse_deps_returns_direct_dependents_sorted() {
        let graph = SpecGraph::build(
            &[
                make_loaded_spec("app/alpha", vec!["core/base"]),
                make_loaded_spec("app/beta", vec!["core/base"]),
                make_loaded_spec("app/gamma", vec!["app/alpha"]),
                make_loaded_spec("core/base", vec![]),
            ],
            &[],
        );

        assert_eq!(
            graph.reverse_deps("core/base"),
            Some(vec!["app/alpha".to_string(), "app/beta".to_string()])
        );
    }

    #[test]
    fn reverse_deps_unknown_unit_returns_none() {
        let graph = SpecGraph::build(&[make_loaded_spec("core/base", vec![])], &[]);

        assert_eq!(graph.reverse_deps("missing/unit"), None);
    }

    #[test]
    fn tests_covering_returns_multiple_tests_sorted() {
        let graph = SpecGraph::build(
            &[
                make_loaded_spec("core/base", vec![]),
                make_loaded_spec("other/unit", vec![]),
            ],
            &[
                make_loaded_molecule_test("tests/zeta", vec!["core/base"]),
                make_loaded_molecule_test("tests/alpha", vec!["core/base"]),
                make_loaded_molecule_test("tests/other", vec!["other/unit"]),
            ],
        );

        assert_eq!(
            graph.tests_covering("core/base"),
            Some(vec!["tests/alpha".to_string(), "tests/zeta".to_string()])
        );
    }

    #[test]
    fn tests_covering_unknown_unit_returns_none() {
        let graph = SpecGraph::build(&[make_loaded_spec("core/base", vec![])], &[]);

        assert_eq!(graph.tests_covering("missing/unit"), None);
    }

    #[test]
    fn impact_includes_seed_reverse_dep_closure_and_covering_tests() {
        let graph = SpecGraph::build(
            &[
                make_loaded_spec("core/base", vec![]),
                make_loaded_spec("service/a", vec!["core/base"]),
                make_loaded_spec("service/b", vec!["service/a"]),
            ],
            &[
                make_loaded_molecule_test("tests/base", vec!["core/base"]),
                make_loaded_molecule_test("tests/service_b", vec!["service/b"]),
            ],
        );

        assert_eq!(
            graph.impact("core/base"),
            Some(ImpactSet {
                units: vec![
                    "core/base".to_string(),
                    "service/a".to_string(),
                    "service/b".to_string(),
                ],
                molecule_tests: vec!["tests/base".to_string(), "tests/service_b".to_string()],
            })
        );
    }

    #[test]
    fn impact_includes_downstream_covering_tests_not_just_seed_tests() {
        let graph = SpecGraph::build(
            &[
                make_loaded_spec("core/base", vec![]),
                make_loaded_spec("service/a", vec!["core/base"]),
            ],
            &[make_loaded_molecule_test(
                "tests/service_a",
                vec!["service/a"],
            )],
        );

        assert_eq!(
            graph.impact("core/base"),
            Some(ImpactSet {
                units: vec!["core/base".to_string(), "service/a".to_string()],
                molecule_tests: vec!["tests/service_a".to_string()],
            })
        );
    }

    #[test]
    fn impact_deduplicates_diamond_reverse_deps() {
        let graph = SpecGraph::build(
            &[
                make_loaded_spec("core/base", vec![]),
                make_loaded_spec("service/left", vec!["core/base"]),
                make_loaded_spec("service/right", vec!["core/base"]),
                make_loaded_spec("service/top", vec!["service/left", "service/right"]),
            ],
            &[make_loaded_molecule_test("tests/top", vec!["service/top"])],
        );

        assert_eq!(
            graph.impact("core/base"),
            Some(ImpactSet {
                units: vec![
                    "core/base".to_string(),
                    "service/left".to_string(),
                    "service/right".to_string(),
                    "service/top".to_string(),
                ],
                molecule_tests: vec!["tests/top".to_string()],
            })
        );
    }

    #[test]
    fn build_graph_keeps_external_dep_edges_but_local_queries_ignore_them() {
        let graph = SpecGraph::build(
            &[
                make_loaded_spec("pricing/apply_discount", vec!["shared::money/round"]),
                make_loaded_spec("money/round", vec![]),
            ],
            &[],
        );

        assert_eq!(
            graph.edges(),
            [SpecEdge::Dep {
                from: "pricing/apply_discount".to_string(),
                to: DepRef::external("shared", "money/round"),
            }]
            .as_slice()
        );
        assert_eq!(graph.reverse_deps("money/round"), Some(vec![]));
        assert_eq!(
            graph.impact("money/round"),
            Some(ImpactSet {
                units: vec!["money/round".to_string()],
                molecule_tests: vec![],
            })
        );
    }

    #[test]
    fn build_graph_keeps_one_top_level_node_for_data_seam() {
        let graph = SpecGraph::build(
            &[
                make_loaded_spec("pricing/apply_discount", vec![]),
                make_loaded_spec("pricing/apply_tax", vec![]),
                make_loaded_data_seam("pricing/checkout_quote"),
            ],
            &[make_loaded_molecule_test(
                "pricing/checkout_quote_behavior",
                vec!["pricing/checkout_quote"],
            )],
        );

        assert_eq!(
            graph
                .units()
                .iter()
                .map(|node| (node.id.as_str(), node.deps.clone()))
                .collect::<Vec<_>>(),
            vec![
                ("pricing/apply_discount", vec![]),
                ("pricing/apply_tax", vec![]),
                (
                    "pricing/checkout_quote",
                    vec![
                        "pricing/apply_discount".to_string(),
                        "pricing/apply_tax".to_string(),
                    ],
                ),
            ]
        );
        assert_eq!(
            graph.edges(),
            [
                SpecEdge::Dep {
                    from: "pricing/checkout_quote".to_string(),
                    to: DepRef::local("pricing/apply_discount"),
                },
                SpecEdge::Dep {
                    from: "pricing/checkout_quote".to_string(),
                    to: DepRef::local("pricing/apply_tax"),
                },
                SpecEdge::Covers {
                    test: "pricing/checkout_quote_behavior".to_string(),
                    unit: "pricing/checkout_quote".to_string(),
                },
            ]
            .as_slice()
        );
    }

    #[test]
    fn build_ignores_links_molecule_tests_legacy_metadata() {
        let graph = SpecGraph::build(
            &[make_loaded_spec_with_links(
                "core/base",
                vec![],
                Some(vec!["legacy/test"]),
            )],
            &[],
        );

        assert!(
            graph
                .edges()
                .iter()
                .all(|edge| !matches!(edge, SpecEdge::Covers { .. }))
        );
        assert_eq!(graph.tests_covering("core/base"), Some(vec![]));
        assert_eq!(
            graph.impact("core/base"),
            Some(ImpactSet {
                units: vec!["core/base".to_string()],
                molecule_tests: vec![],
            })
        );
    }
}
