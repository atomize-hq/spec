pub mod analysis_core;
pub mod certify;
pub mod coverage;
pub mod harness;
pub mod inventory;
pub mod layout;
pub mod manifest;
pub mod paths;
pub mod promotion_artifacts;
pub mod prove;
pub mod recommend;
pub mod report;
pub mod routing;
pub mod scaffold;
pub mod smoke;
pub mod verify;

// Compatibility-only passthroughs for historical call sites.
// Semantic ownership remains in `analysis_core/*`.
pub mod decision_kernel;
pub mod helper_surface;
