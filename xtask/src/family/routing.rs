use crate::family::harness::{
    LockedManifestRouting, TERMINAL_UNSUPPORTED_CATCH_ALL, family_harness,
    registered_harnesses_in_routing_order,
};
use crate::family::manifest::Routing;
use crate::family::paths::FamilyId;

#[allow(dead_code)]
pub const CHAIN3_PRECEDENCE: u64 = crate::family::harness::CHAIN3_PRECEDENCE;
#[allow(dead_code)]
pub const CHAIN3_MUST_NOT_SHADOW: [&str; 3] = crate::family::harness::CHAIN3_MUST_NOT_SHADOW;

pub fn locked_routing_order_with_terminal() -> [&'static str; 5] {
    let harnesses = registered_harnesses_in_routing_order();
    debug_assert_eq!(
        harnesses.len(),
        1,
        "locked routing helper assumes one family"
    );
    debug_assert_eq!(
        harnesses[0].routing.must_not_shadow.len(),
        3,
        "locked routing helper assumes three must_not_shadow entries",
    );
    [
        harnesses[0].family,
        harnesses[0].routing.must_not_shadow[0],
        harnesses[0].routing.must_not_shadow[1],
        harnesses[0].routing.must_not_shadow[2],
        TERMINAL_UNSUPPORTED_CATCH_ALL,
    ]
}

pub fn locked_manifest_routing(family: &FamilyId) -> Option<LockedManifestRouting> {
    family_harness(family).map(|harness| harness.routing)
}

pub fn manifest_matches_locked_routing(family: &FamilyId, routing: &Routing) -> bool {
    let Some(expected) = locked_manifest_routing(family) else {
        return false;
    };

    routing.precedence == expected.precedence
        && routing
            .must_not_shadow
            .iter()
            .map(String::as_str)
            .eq(expected.must_not_shadow.iter().copied())
}

pub fn manifest_routing_mismatch_message(family: &FamilyId, routing: &Routing) -> Option<String> {
    let expected = locked_manifest_routing(family)?;
    if manifest_matches_locked_routing(family, routing) {
        return None;
    }

    let locked_order = locked_routing_order_with_terminal();
    Some(format!(
        "manifest routing mismatch for `{}`: locked routing order {:?}; expected precedence {} with must_not_shadow {:?}, found precedence {} with must_not_shadow {:?}",
        family.as_str(),
        locked_order,
        expected.precedence,
        expected.must_not_shadow,
        routing.precedence,
        routing.must_not_shadow
    ))
}
