//! Minimal SpecGraph: captures the unit/test/edge world model.
//!
//! This module provides the foundation for M8's full graph layer.
//! It models units, molecule tests, and the edges between them (dep and covers).

use crate::types::{LoadedMoleculeTest, LoadedSpec};

/// A node representing a unit in the spec graph
#[derive(Debug, Clone)]
pub struct UnitNode {
    pub id: String,
    pub deps: Vec<String>,
}

/// A node representing a molecule test in the spec graph
#[derive(Debug, Clone)]
pub struct MoleculeTestNode {
    pub id: String,
    pub covers: Vec<String>,
}

/// An edge in the spec graph, either a dependency or a covers relationship
#[derive(Debug, Clone)]
pub enum SpecEdge {
    /// A unit depends on another unit
    Dep { from: String, to: String },
    /// A molecule test covers a unit
    Covers { test: String, unit: String },
}

/// The full spec graph: units, molecule tests, and edges between them
#[derive(Debug, Clone)]
pub struct SpecGraph {
    pub units: Vec<UnitNode>,
    pub molecule_tests: Vec<MoleculeTestNode>,
    pub edges: Vec<SpecEdge>,
}

impl SpecGraph {
    /// Build a SpecGraph from loaded units and molecule tests
    pub fn build(units: &[LoadedSpec], molecule_tests: &[LoadedMoleculeTest]) -> Self {
        let unit_nodes = units
            .iter()
            .map(|u| UnitNode {
                id: u.spec.id.clone(),
                deps: u.spec.deps.clone(),
            })
            .collect();

        let molecule_test_nodes = molecule_tests
            .iter()
            .map(|t| MoleculeTestNode {
                id: t.test.id.clone(),
                covers: t.test.covers.clone(),
            })
            .collect();

        let mut edges = Vec::new();

        for u in units {
            for dep in &u.spec.deps {
                edges.push(SpecEdge::Dep {
                    from: u.spec.id.clone(),
                    to: dep.clone(),
                });
            }
        }

        for t in molecule_tests {
            for unit_id in &t.test.covers {
                edges.push(SpecEdge::Covers {
                    test: t.test.id.clone(),
                    unit: unit_id.clone(),
                });
            }
        }

        Self {
            units: unit_nodes,
            molecule_tests: molecule_test_nodes,
            edges,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Body, Intent, MoleculeTestSource, MoleculeTestStruct, SpecSource, SpecStruct};

    fn make_loaded_spec(id: &str, deps: Vec<&str>) -> LoadedSpec {
        LoadedSpec {
            source: SpecSource {
                file_path: format!("{}.unit.spec", id.replace('/', "/")),
                id: id.to_string(),
            },
            spec: SpecStruct {
                id: id.to_string(),
                kind: "function".to_string(),
                intent: Intent { why: format!("Why {id}") },
                contract: None,
                deps: deps.into_iter().map(str::to_string).collect(),
                imports: vec![],
                body: Body { rust: "{ }".to_string() },
                local_tests: vec![],
                links: None,
                spec_version: None,
            },
        }
    }

    fn make_loaded_molecule_test(id: &str, covers: Vec<&str>) -> LoadedMoleculeTest {
        LoadedMoleculeTest {
            source: MoleculeTestSource {
                file_path: format!("{}.test.spec", id.replace('/', "/")),
                id: id.to_string(),
            },
            test: MoleculeTestStruct {
                id: id.to_string(),
                intent: Intent { why: format!("Why {id}") },
                covers: covers.into_iter().map(str::to_string).collect(),
                body: Body { rust: "{ assert!(true); }".to_string() },
                spec_version: None,
            },
        }
    }

    #[test]
    fn build_graph_creates_dep_and_covers_edges() {
        let units = vec![
            make_loaded_spec("pricing/apply_discount", vec!["money/round"]),
            make_loaded_spec("money/round", vec![]),
        ];
        let molecule_tests = vec![
            make_loaded_molecule_test("pricing/discount_test", vec!["pricing/apply_discount"]),
        ];

        let graph = SpecGraph::build(&units, &molecule_tests);

        assert_eq!(graph.units.len(), 2);
        assert_eq!(graph.molecule_tests.len(), 1);
        assert_eq!(graph.edges.len(), 2); // 1 dep + 1 covers

        let dep_edges: Vec<_> = graph.edges.iter().filter(|e| matches!(e, SpecEdge::Dep { .. })).collect();
        assert_eq!(dep_edges.len(), 1);

        let covers_edges: Vec<_> = graph.edges.iter().filter(|e| matches!(e, SpecEdge::Covers { .. })).collect();
        assert_eq!(covers_edges.len(), 1);
    }

    #[test]
    fn build_graph_empty_inputs() {
        let graph = SpecGraph::build(&[], &[]);
        assert!(graph.units.is_empty());
        assert!(graph.molecule_tests.is_empty());
        assert!(graph.edges.is_empty());
    }
}
