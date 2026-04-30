use crate::XtaskError;
use crate::family::harness::{
    FamilyHarness, registered_harnesses_in_routing_order,
    registered_harnesses_in_routing_order_from,
};
use crate::family::paths::{FamilyId, PacketPaths};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::fs;
use std::io::{self, Write};
use std::path::{Component, Path, PathBuf};
use std::process::Command;

const INVENTORY_SCHEMA_VERSION: u64 = 1;
const TERMINAL_UNSUPPORTED_CATCH_ALL: &str = "unsupported.function.v1";
const RUNTIME_ROUTE_MARKERS: [(&str, &str); 4] = [
    (
        "WrapperPipelineChain3",
        "function.wrapper.pipeline.chain3.v1",
    ),
    ("WrapperPipeline", "function.wrapper.pipeline.v1"),
    (
        "ArithmeticLeafMonotoneDownNonnegative",
        "function.arithmetic_leaf.monotone_down_nonnegative.v1",
    ),
    (
        "ArithmeticLeafMonotoneUp",
        "function.arithmetic_leaf.monotone_up.v1",
    ),
];

const INVENTORY_METADATA: [InventoryFamilyMetadata; 4] = [
    InventoryFamilyMetadata {
        family: "function.wrapper.pipeline.chain3.v1",
        canonical_seed_paths: &[
            "semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/aligned/units/pricing/checkout_chain3_aligned.unit.spec",
        ],
        existing_wedge_paths: &["spec-core/src/semantic_review.rs"],
        supporting_packet_paths: &["semantic-families/function.wrapper.pipeline.chain3.v1"],
    },
    InventoryFamilyMetadata {
        family: "function.wrapper.pipeline.v1",
        canonical_seed_paths: &["examples/ecommerce/units/pricing/calculate_total.unit.spec"],
        existing_wedge_paths: &["spec-cli/tests/m14_regressions.rs"],
        supporting_packet_paths: &[
            "semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/aligned/units/pricing/pricing_discount_leaf_aligned.unit.spec",
            "semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/aligned/units/pricing/pricing_tax_leaf_aligned.unit.spec",
            "semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/aligned/units/pricing/pricing_total_wrapper_aligned.unit.spec",
            "semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/drift/units/pricing/pricing_total_wrapper_drift.unit.spec",
            "semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/under_specified/units/pricing/pricing_total_wrapper_under_specified.unit.spec",
            "semantic-families/function.wrapper.pipeline.chain3.v1/fixtures/unsupported_near_miss/units/pricing/pricing_total_wrapper_unsupported_near_miss.unit.spec",
        ],
    },
    InventoryFamilyMetadata {
        family: "function.arithmetic_leaf.monotone_down_nonnegative.v1",
        canonical_seed_paths: &["examples/ecommerce/units/pricing/apply_discount.unit.spec"],
        existing_wedge_paths: &["spec-cli/tests/m14_regressions.rs"],
        supporting_packet_paths: &[
            "semantic-families/function.arithmetic_leaf.monotone_down_nonnegative.v1",
        ],
    },
    InventoryFamilyMetadata {
        family: "function.arithmetic_leaf.monotone_up.v1",
        canonical_seed_paths: &["examples/ecommerce/units/pricing/apply_tax.unit.spec"],
        existing_wedge_paths: &["spec-cli/tests/m14_regressions.rs"],
        supporting_packet_paths: &["semantic-families/function.arithmetic_leaf.monotone_up.v1"],
    },
];

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub(crate) struct FamilyInventory {
    pub schema_version: u64,
    pub generated_at: String,
    pub promoted_families: Vec<String>,
    pub runtime_supported_routes: Vec<String>,
    pub supported_unpromoted_families: Vec<SupportedUnpromotedFamilyEntry>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub(crate) struct SupportedUnpromotedFamilyEntry {
    pub family: String,
    pub canonical_seed_paths: Vec<String>,
    pub existing_wedge_paths: Vec<String>,
    pub supporting_packet_paths: Vec<String>,
    pub routing_predecessor: Option<String>,
    pub routing_successors: Vec<String>,
}

#[derive(Debug, Clone, Copy)]
struct InventoryFamilyMetadata {
    family: &'static str,
    canonical_seed_paths: &'static [&'static str],
    existing_wedge_paths: &'static [&'static str],
    supporting_packet_paths: &'static [&'static str],
}

pub(crate) fn run(workspace_root: &Path, format: &str) -> Result<(), XtaskError> {
    if format != "json" {
        return Err(XtaskError::InvalidInput(format!(
            "family inventory only supports `--format json`, found `{format}`"
        )));
    }

    let bytes = render_snapshot_bytes(workspace_root)?;
    let mut stdout = io::stdout().lock();
    stdout
        .write_all(&bytes)
        .map_err(|error| XtaskError::WriteFailure(format!("failed to write inventory: {error}")))?;
    stdout
        .flush()
        .map_err(|error| XtaskError::WriteFailure(format!("failed to flush inventory: {error}")))
}

pub(crate) fn collect_inventory(workspace_root: &Path) -> Result<FamilyInventory, XtaskError> {
    let registry = registered_harnesses_in_routing_order()
        .into_iter()
        .copied()
        .collect::<Vec<_>>();
    collect_inventory_in(&registry, workspace_root)
}

pub(crate) fn collect_inventory_in(
    registry: &[FamilyHarness],
    workspace_root: &Path,
) -> Result<FamilyInventory, XtaskError> {
    let promoted_families = registered_harnesses_in_routing_order_from(registry)
        .into_iter()
        .filter_map(|harness| {
            let family = FamilyId::parse(harness.family)
                .expect("registered family harness ids must be valid");
            let paths = PacketPaths::new(workspace_root, family);
            paths.root.exists().then(|| harness.family.to_string())
        })
        .collect::<Vec<_>>();
    let runtime_supported_routes = runtime_supported_families_from_repo(workspace_root)?;
    let generated_at = inventory_generated_at(workspace_root)?;

    let promoted_set = promoted_families.iter().cloned().collect::<BTreeSet<_>>();
    let runtime_set = runtime_supported_routes
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();

    for family in &promoted_families {
        if !runtime_set.contains(family) {
            return Err(XtaskError::InvalidInput(format!(
                "inventory projection mismatch: promoted family `{family}` is not present in runtime-supported routing truth"
            )));
        }
    }

    let supported_unpromoted_families = runtime_supported_routes
        .iter()
        .enumerate()
        .filter(|(_, family)| !promoted_set.contains(*family))
        .map(|(index, family)| {
            let metadata = inventory_metadata(family).ok_or_else(|| {
                XtaskError::InvalidInput(format!(
                    "inventory metadata missing for runtime-supported family `{family}`"
                ))
            })?;
            validate_inventory_paths_exist(workspace_root, metadata)?;

            let mut canonical_seed_paths = metadata
                .canonical_seed_paths
                .iter()
                .map(|path| (*path).to_string())
                .collect::<Vec<_>>();
            canonical_seed_paths.sort();

            let mut existing_wedge_paths = metadata
                .existing_wedge_paths
                .iter()
                .map(|path| (*path).to_string())
                .collect::<Vec<_>>();
            existing_wedge_paths.sort();

            let mut supporting_packet_paths = metadata
                .supporting_packet_paths
                .iter()
                .map(|path| (*path).to_string())
                .collect::<Vec<_>>();
            supporting_packet_paths.sort();

            let mut routing_successors = runtime_supported_routes[index + 1..].to_vec();
            routing_successors.push(TERMINAL_UNSUPPORTED_CATCH_ALL.to_string());

            Ok(SupportedUnpromotedFamilyEntry {
                family: family.clone(),
                canonical_seed_paths,
                existing_wedge_paths,
                supporting_packet_paths,
                routing_predecessor: index
                    .checked_sub(1)
                    .and_then(|prior| runtime_supported_routes.get(prior).cloned()),
                routing_successors,
            })
        })
        .collect::<Result<Vec<_>, XtaskError>>()?;

    Ok(FamilyInventory {
        schema_version: INVENTORY_SCHEMA_VERSION,
        generated_at,
        promoted_families,
        runtime_supported_routes,
        supported_unpromoted_families,
    })
}

pub(crate) fn render_snapshot_bytes(workspace_root: &Path) -> Result<Vec<u8>, XtaskError> {
    let inventory = collect_inventory(workspace_root)?;
    let mut bytes = serde_json::to_vec_pretty(&inventory).map_err(|error| {
        XtaskError::WriteFailure(format!("failed to serialize inventory: {error}"))
    })?;
    bytes.push(b'\n');
    Ok(bytes)
}

#[cfg_attr(not(test), allow(dead_code))]
pub(crate) fn render_snapshot_bytes_in(
    registry: &[FamilyHarness],
    workspace_root: &Path,
) -> Result<Vec<u8>, XtaskError> {
    let inventory = collect_inventory_in(registry, workspace_root)?;
    let mut bytes = serde_json::to_vec_pretty(&inventory).map_err(|error| {
        XtaskError::WriteFailure(format!("failed to serialize inventory: {error}"))
    })?;
    bytes.push(b'\n');
    Ok(bytes)
}

pub(crate) fn inventory_sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}

fn runtime_supported_families_from_repo(workspace_root: &Path) -> Result<Vec<String>, XtaskError> {
    let source_path = workspace_root.join("spec-core/src/semantic_review.rs");
    let source = fs::read_to_string(&source_path).map_err(|error| {
        XtaskError::WriteFailure(format!(
            "failed to read runtime routing source `{}`: {error}",
            source_path.display()
        ))
    })?;

    let marker = "const SUPPORTED_FUNCTION_ROUTING_ORDER";
    let start = source.find(marker).ok_or_else(|| {
        XtaskError::InvalidInput(format!(
            "inventory projection mismatch: `{marker}` is missing from `{}`",
            source_path.display()
        ))
    })?;
    let ordered_block = &source[start..];
    let open_bracket = ordered_block.find('[').ok_or_else(|| {
        XtaskError::InvalidInput(format!(
            "inventory projection mismatch: failed to parse runtime routing order in `{}`",
            source_path.display()
        ))
    })?;
    let close_marker = "];";
    let close_bracket = ordered_block.find(close_marker).ok_or_else(|| {
        XtaskError::InvalidInput(format!(
            "inventory projection mismatch: failed to locate the end of runtime routing order in `{}`",
            source_path.display()
        ))
    })?;
    let routes_block = &ordered_block[open_bracket + 1..close_bracket];

    let mut ordered_families = Vec::new();
    for line in routes_block.lines() {
        let trimmed = line.trim();
        let Some(route) = trimmed.strip_prefix("SupportedFunctionRoute::") else {
            continue;
        };
        let route = route.trim_end_matches(',');
        let family = RUNTIME_ROUTE_MARKERS
            .iter()
            .find_map(|(marker, family)| (*marker == route).then_some(*family))
            .ok_or_else(|| {
                XtaskError::InvalidInput(format!(
                    "inventory projection mismatch: runtime route `{route}` is not mapped in xtask inventory"
                ))
            })?;
        ordered_families.push(family.to_string());
    }

    if ordered_families.is_empty() {
        return Err(XtaskError::InvalidInput(format!(
            "inventory projection mismatch: runtime routing order in `{}` is empty",
            source_path.display()
        )));
    }

    Ok(ordered_families)
}

fn inventory_metadata(family: &str) -> Option<&'static InventoryFamilyMetadata> {
    INVENTORY_METADATA
        .iter()
        .find(|metadata| metadata.family == family)
}

fn inventory_generated_at(workspace_root: &Path) -> Result<String, XtaskError> {
    let output = Command::new("git")
        .args(["show", "-s", "--format=%ct", "HEAD"])
        .current_dir(workspace_root)
        .output();

    match output {
        Ok(output) if output.status.success() => {
            let epoch = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if epoch.is_empty() {
                return Ok("1970-01-01T00:00:00Z".to_string());
            }

            let date_output = Command::new("date")
                .args(["-u", "-r", &epoch, "+%Y-%m-%dT%H:%M:%SZ"])
                .current_dir(workspace_root)
                .output();

            match date_output {
                Ok(date_output) if date_output.status.success() => {
                    let value = String::from_utf8_lossy(&date_output.stdout)
                        .trim()
                        .to_string();
                    if value.is_empty() {
                        Ok("1970-01-01T00:00:00Z".to_string())
                    } else {
                        Ok(value)
                    }
                }
                Ok(_) | Err(_) => Ok("1970-01-01T00:00:00Z".to_string()),
            }
        }
        Ok(_) | Err(_) => Ok("1970-01-01T00:00:00Z".to_string()),
    }
}

fn validate_inventory_paths_exist(
    workspace_root: &Path,
    metadata: &InventoryFamilyMetadata,
) -> Result<(), XtaskError> {
    for path in metadata
        .canonical_seed_paths
        .iter()
        .chain(metadata.existing_wedge_paths.iter())
        .chain(metadata.supporting_packet_paths.iter())
    {
        validate_repo_relative_path_exists(workspace_root, path)?;
    }
    Ok(())
}

fn validate_repo_relative_path_exists(
    workspace_root: &Path,
    raw_path: &str,
) -> Result<(), XtaskError> {
    let relative = Path::new(raw_path);
    if relative.as_os_str().is_empty() || relative.is_absolute() {
        return Err(XtaskError::InvalidInput(format!(
            "inventory metadata path `{raw_path}` must be a non-empty repo-relative path"
        )));
    }
    if relative
        .components()
        .any(|component| !matches!(component, Component::Normal(_)))
    {
        return Err(XtaskError::InvalidInput(format!(
            "inventory metadata path `{raw_path}` must contain only normal path components"
        )));
    }

    let absolute = workspace_root.join(PathBuf::from(relative));
    if !absolute.exists() {
        return Err(XtaskError::InvalidInput(format!(
            "inventory metadata path `{raw_path}` does not exist in the workspace"
        )));
    }
    Ok(())
}
