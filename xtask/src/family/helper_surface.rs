use spec_core::semantic_review::UnsupportedFunctionReasonCode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct HelperSurfaceSignal<'a> {
    pub(crate) primary_reason_code: UnsupportedFunctionReasonCode,
    pub(crate) overlap_family: &'a str,
    pub(crate) real_example_hits: usize,
    pub(crate) shape_fingerprint: &'a str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum HelperSurfaceDisposition {
    DurableNonPromotableHelperSurface,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct UnsupportedShapeFingerprint {
    schema_version: u64,
    function_dep_arity: usize,
    callable_dep_topology_class: String,
    contract_input_count: usize,
    has_return: bool,
    authored_body_kind: String,
}

pub(crate) fn classify_helper_surface(
    signal: &HelperSurfaceSignal<'_>,
) -> Option<HelperSurfaceDisposition> {
    if signal.primary_reason_code != UnsupportedFunctionReasonCode::UnsupportedFunctionSurface {
        return None;
    }
    if signal.overlap_family != "unknown" || signal.real_example_hits == 0 {
        return None;
    }
    if !matches_helper_surface_fingerprint(signal.shape_fingerprint) {
        return None;
    }
    Some(HelperSurfaceDisposition::DurableNonPromotableHelperSurface)
}

fn matches_helper_surface_fingerprint(shape_fingerprint: &str) -> bool {
    let Ok(fingerprint) = serde_json::from_str::<UnsupportedShapeFingerprint>(shape_fingerprint)
    else {
        return false;
    };
    fingerprint.schema_version == 1
        && fingerprint.function_dep_arity == 0
        && fingerprint.callable_dep_topology_class == "no_deps_or_helper"
        && fingerprint.contract_input_count == 1
        && fingerprint.has_return
        && fingerprint.authored_body_kind == "neither"
}

#[cfg(test)]
mod tests {
    use super::{HelperSurfaceDisposition, HelperSurfaceSignal, classify_helper_surface};
    use spec_core::semantic_review::UnsupportedFunctionReasonCode;

    const HELPER_FINGERPRINT: &str = "{\"schema_version\":1,\"function_dep_arity\":0,\"callable_dep_topology_class\":\"no_deps_or_helper\",\"contract_input_count\":1,\"has_return\":true,\"authored_body_kind\":\"neither\"}";

    #[test]
    fn helper_surface_classifies_durable_non_promotable_helper_surface() {
        let signal = HelperSurfaceSignal {
            primary_reason_code: UnsupportedFunctionReasonCode::UnsupportedFunctionSurface,
            overlap_family: "unknown",
            real_example_hits: 2,
            shape_fingerprint: HELPER_FINGERPRINT,
        };

        assert_eq!(
            classify_helper_surface(&signal),
            Some(HelperSurfaceDisposition::DurableNonPromotableHelperSurface)
        );
    }

    #[test]
    fn helper_surface_rejects_non_matching_signal() {
        let signal = HelperSurfaceSignal {
            primary_reason_code: UnsupportedFunctionReasonCode::UnsupportedFunctionSurface,
            overlap_family: "function.wrapper.pipeline*",
            real_example_hits: 2,
            shape_fingerprint: HELPER_FINGERPRINT,
        };

        assert_eq!(classify_helper_surface(&signal), None);
    }
}
