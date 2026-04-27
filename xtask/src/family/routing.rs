use crate::family::manifest::Routing;
use crate::family::paths::FamilyId;

pub const ORDERED_SUPPORTED_ROUTING: [&str; 4] = [
    "function.wrapper.pipeline.chain3.v1",
    "function.wrapper.pipeline.v1",
    "function.arithmetic_leaf.monotone_down_nonnegative.v1",
    "function.arithmetic_leaf.monotone_up.v1",
];

pub const TERMINAL_UNSUPPORTED_CATCH_ALL: &str = "unsupported.function.v1";

pub const CHAIN3_PRECEDENCE: u64 = 1;
pub const CHAIN3_MUST_NOT_SHADOW: [&str; 3] = [
    "function.wrapper.pipeline.v1",
    "function.arithmetic_leaf.monotone_down_nonnegative.v1",
    "function.arithmetic_leaf.monotone_up.v1",
];

#[derive(Debug, Clone, Copy)]
pub struct LockedManifestRouting {
    pub precedence: u64,
    pub must_not_shadow: &'static [&'static str],
}

pub fn locked_routing_order_with_terminal() -> [&'static str; 5] {
    [
        ORDERED_SUPPORTED_ROUTING[0],
        ORDERED_SUPPORTED_ROUTING[1],
        ORDERED_SUPPORTED_ROUTING[2],
        ORDERED_SUPPORTED_ROUTING[3],
        TERMINAL_UNSUPPORTED_CATCH_ALL,
    ]
}

pub fn locked_manifest_routing(family: &FamilyId) -> Option<LockedManifestRouting> {
    match family.as_str() {
        "function.wrapper.pipeline.chain3.v1" => Some(LockedManifestRouting {
            precedence: CHAIN3_PRECEDENCE,
            must_not_shadow: &CHAIN3_MUST_NOT_SHADOW,
        }),
        _ => None,
    }
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
