use crate::XtaskError;
use crate::family::harness::{FamilyHarness, LockedManifestArgs, LockedManifestShape};
use crate::family::paths::FamilyId;
use crate::family::paths::REQUIRED_BUCKETS;
use serde::Deserialize;
use std::fs;
use std::path::Path;

const REQUIRES_REFRESH_VIA: [&str; 1] = ["spec test"];
const PRESERVE_ONLY_VIA: [&str; 4] = ["spec build", "spec generate", "spec status", "spec export"];

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FamilyManifest {
    pub schema_version: u64,
    pub family: String,
    pub kind: String,
    pub compatibility_key: String,
    pub summary: String,
    pub routing: Routing,
    pub shape: Shape,
    pub args: Args,
    pub corpus: Corpus,
    pub truth_surface: TruthSurface,
    pub gates: Gates,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Routing {
    pub precedence: u64,
    pub must_not_shadow: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Shape {
    pub dep_min: u64,
    pub dep_max: u64,
    pub control_flow: String,
    pub return_style: String,
    pub loops: bool,
    pub branching: bool,
    pub requires_supported_function_deps: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Args {
    pub threading: String,
    pub allow_nested_argument_expressions: bool,
    pub allow_literal_only_extra_args: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Corpus {
    pub required_buckets: Vec<String>,
    pub min_cases_per_bucket: u64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TruthSurface {
    pub requires_refresh_via: Vec<String>,
    pub preserve_only_via: Vec<String>,
    pub requires_stale_demote: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Gates {
    pub gate_a: bool,
    pub gate_b: bool,
    pub gate_c: bool,
    pub gate_d: bool,
}

impl FamilyManifest {
    pub fn validate(
        &self,
        expected_family: &FamilyId,
        harness: &FamilyHarness,
    ) -> Result<(), XtaskError> {
        if self.schema_version != 2 {
            return Err(XtaskError::InvalidInput(format!(
                "family.toml schema_version must be 2, found {}",
                self.schema_version
            )));
        }
        if self.family != expected_family.as_str() {
            return Err(XtaskError::InvalidInput(format!(
                "family.toml family `{}` must match packet directory `{}`",
                self.family,
                expected_family.as_str()
            )));
        }
        if self.kind != "function" {
            return Err(XtaskError::InvalidInput(format!(
                "family.toml kind must be `function`, found `{}`",
                self.kind
            )));
        }
        if self.compatibility_key != self.family {
            return Err(XtaskError::InvalidInput(
                "family.toml compatibility_key must equal family".to_string(),
            ));
        }
        if self.summary.is_empty() || self.summary.contains('\n') || self.summary.contains('\r') {
            return Err(XtaskError::InvalidInput(
                "family.toml summary must be a single non-empty line".to_string(),
            ));
        }

        validate_routing(&self.routing)?;
        validate_shape(&self.shape, harness.shape)?;
        validate_args(&self.args, harness.args)?;

        let required_buckets: Vec<String> = REQUIRED_BUCKETS
            .iter()
            .map(|bucket| bucket.to_string())
            .collect();
        if self.corpus.required_buckets != required_buckets {
            return Err(XtaskError::InvalidInput(format!(
                "family.toml corpus.required_buckets must be exactly {:?}",
                REQUIRED_BUCKETS
            )));
        }
        if self.corpus.min_cases_per_bucket < 1 {
            return Err(XtaskError::InvalidInput(
                "family.toml corpus.min_cases_per_bucket must be at least 1".to_string(),
            ));
        }

        if self.truth_surface.requires_refresh_via
            != REQUIRES_REFRESH_VIA
                .iter()
                .map(|value| value.to_string())
                .collect::<Vec<_>>()
        {
            return Err(XtaskError::InvalidInput(format!(
                "family.toml truth_surface.requires_refresh_via must be exactly {:?}",
                REQUIRES_REFRESH_VIA
            )));
        }
        if self.truth_surface.preserve_only_via
            != PRESERVE_ONLY_VIA
                .iter()
                .map(|value| value.to_string())
                .collect::<Vec<_>>()
        {
            return Err(XtaskError::InvalidInput(format!(
                "family.toml truth_surface.preserve_only_via must be exactly {:?}",
                PRESERVE_ONLY_VIA
            )));
        }
        if !self.truth_surface.requires_stale_demote {
            return Err(XtaskError::InvalidInput(
                "family.toml truth_surface.requires_stale_demote must be true".to_string(),
            ));
        }
        if !(self.gates.gate_a && self.gates.gate_b && self.gates.gate_c && self.gates.gate_d) {
            return Err(XtaskError::InvalidInput(
                "family.toml gates.gate_a through gate_d must all be true".to_string(),
            ));
        }

        Ok(())
    }
}

fn validate_routing(routing: &Routing) -> Result<(), XtaskError> {
    if routing.precedence == 0 {
        return Err(XtaskError::InvalidInput(
            "family.toml routing.precedence must be a positive integer".to_string(),
        ));
    }
    if routing.must_not_shadow.is_empty() {
        return Err(XtaskError::InvalidInput(
            "family.toml routing.must_not_shadow must be non-empty".to_string(),
        ));
    }
    if routing
        .must_not_shadow
        .iter()
        .any(|value| value.trim().is_empty())
    {
        return Err(XtaskError::InvalidInput(
            "family.toml routing.must_not_shadow entries must be non-empty".to_string(),
        ));
    }

    Ok(())
}

fn validate_shape(shape: &Shape, expected: LockedManifestShape) -> Result<(), XtaskError> {
    if shape.dep_min > shape.dep_max {
        return Err(XtaskError::InvalidInput(format!(
            "family.toml shape.dep_min must be less than or equal to shape.dep_max, found {} > {}",
            shape.dep_min, shape.dep_max
        )));
    }
    if shape.dep_min != expected.dep_min || shape.dep_max != expected.dep_max {
        return Err(XtaskError::InvalidInput(format!(
            "family.toml shape dep range must be {}..={}, found {}..={}",
            expected.dep_min, expected.dep_max, shape.dep_min, shape.dep_max
        )));
    }
    if shape.control_flow != expected.control_flow {
        return Err(XtaskError::InvalidInput(format!(
            "family.toml shape.control_flow must be `{}`, found `{}`",
            expected.control_flow, shape.control_flow
        )));
    }
    if shape.return_style != expected.return_style {
        return Err(XtaskError::InvalidInput(format!(
            "family.toml shape.return_style must be `{}`, found `{}`",
            expected.return_style, shape.return_style
        )));
    }
    if shape.loops != expected.loops {
        return Err(XtaskError::InvalidInput(format!(
            "family.toml shape.loops must be {}, found {}",
            expected.loops, shape.loops
        )));
    }
    if shape.branching != expected.branching {
        return Err(XtaskError::InvalidInput(format!(
            "family.toml shape.branching must be {}, found {}",
            expected.branching, shape.branching
        )));
    }
    if shape.requires_supported_function_deps != expected.requires_supported_function_deps {
        return Err(XtaskError::InvalidInput(format!(
            "family.toml shape.requires_supported_function_deps must be {}, found {}",
            expected.requires_supported_function_deps, shape.requires_supported_function_deps
        )));
    }

    Ok(())
}

fn validate_args(args: &Args, expected: LockedManifestArgs) -> Result<(), XtaskError> {
    if args.threading != expected.threading {
        return Err(XtaskError::InvalidInput(format!(
            "family.toml args.threading must be `{}`, found `{}`",
            expected.threading, args.threading
        )));
    }
    if args.allow_nested_argument_expressions != expected.allow_nested_argument_expressions {
        return Err(XtaskError::InvalidInput(format!(
            "family.toml args.allow_nested_argument_expressions must be {}, found {}",
            expected.allow_nested_argument_expressions, args.allow_nested_argument_expressions
        )));
    }
    if args.allow_literal_only_extra_args != expected.allow_literal_only_extra_args {
        return Err(XtaskError::InvalidInput(format!(
            "family.toml args.allow_literal_only_extra_args must be {}, found {}",
            expected.allow_literal_only_extra_args, args.allow_literal_only_extra_args
        )));
    }

    Ok(())
}

pub fn parse_manifest_file(
    manifest_path: &Path,
    expected_family: &FamilyId,
    harness: &FamilyHarness,
) -> Result<FamilyManifest, XtaskError> {
    let contents = fs::read_to_string(manifest_path).map_err(|error| {
        XtaskError::InvalidInput(format!(
            "failed to read family manifest `{}`: {error}",
            manifest_path.display()
        ))
    })?;

    let manifest = toml::from_str::<FamilyManifest>(&contents).map_err(|error| {
        XtaskError::InvalidInput(format!(
            "failed to parse family manifest `{}`: {error}",
            manifest_path.display()
        ))
    })?;

    manifest.validate(expected_family, harness)?;
    Ok(manifest)
}
