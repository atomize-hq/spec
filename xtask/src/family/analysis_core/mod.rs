pub(crate) mod decision_contract;
pub(crate) mod helper_surface;
pub(crate) mod proof_fingerprint;

#[allow(unused_imports)]
pub(crate) use decision_contract::{
    DecisionContractStopStateTuple, DerivedCorpusProgramDecision,
    basis_activates_helper_surface_follow_on, basis_snapshot_requires_helper_surface_follow_on,
    corpus_program_basis_snapshot, decision_contract_stop_state_tuple,
    derive_corpus_program_decision_contract,
};
#[allow(unused_imports)]
pub(crate) use helper_surface::{
    HELPER_SURFACE_FINGERPRINT, HelperSurfaceDisposition, HelperSurfaceSignal,
    classify_helper_surface, durable_non_promotable_helper_surface_candidate_tuple,
    recommendation_matches_helper_surface_durable_hold_tuple,
    recommendation_uses_helper_surface_durable_hold_tuple,
};
#[allow(unused_imports)]
pub(crate) use proof_fingerprint::{
    normalized_corpus_program_decision_proof_fingerprint, normalized_coverage_proof_fingerprint,
    normalized_for_recommend_determinism, normalized_recommendation_proof_fingerprint,
};
