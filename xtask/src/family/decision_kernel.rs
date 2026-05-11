//! Compatibility-only passthrough for historical decision-contract call sites.
//! Semantic ownership lives in `crate::family::analysis_core::decision_contract`.

#[allow(unused_imports)]
pub(crate) use crate::family::analysis_core::decision_contract::{
    DerivedCorpusProgramDecision, basis_activates_helper_surface_follow_on,
    basis_snapshot_requires_helper_surface_follow_on, corpus_program_basis_snapshot,
    derive_corpus_program_decision_contract,
};
