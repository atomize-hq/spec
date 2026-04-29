use crate::XtaskError;
use crate::family::harness::{
    FamilyHarness, StarterTemplate, registered_family_harnesses, require_family_harness_in,
};
use crate::family::paths::{FAMILY_ROOT_DIR, FamilyId, PacketPaths, ensure_packet_path_safe};
use crate::family::scaffold;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

pub fn run(workspace_root: &Path, raw_family: &str) -> Result<(), XtaskError> {
    run_in(registered_family_harnesses(), workspace_root, raw_family)
}

pub(crate) fn run_in(
    registry: &[FamilyHarness],
    workspace_root: &Path,
    raw_family: &str,
) -> Result<(), XtaskError> {
    let family = FamilyId::parse(raw_family)?;
    let harness = *require_family_harness_in(registry, &family, "family smoke")?;
    let committed_paths = PacketPaths::new(workspace_root, family.clone());
    ensure_packet_path_safe(workspace_root, &committed_paths.root)?;

    let temp_dir = TempDir::new().map_err(|error| {
        XtaskError::WriteFailure(format!(
            "failed to create temporary workspace for `family smoke`: {error}"
        ))
    })?;
    fs::create_dir(temp_dir.path().join(FAMILY_ROOT_DIR)).map_err(|error| {
        XtaskError::WriteFailure(format!(
            "failed to create temporary semantic family root under `{}`: {error}",
            temp_dir.path().display()
        ))
    })?;

    scaffold::run(temp_dir.path(), family.as_str())?;
    let scaffolded_paths = PacketPaths::new(temp_dir.path(), family.clone());
    let failures = collect_smoke_failures(&committed_paths, &scaffolded_paths, harness)?;

    if failures.is_empty() {
        println!("family smoke passed for `{}`", family.as_str());
        return Ok(());
    }

    Err(XtaskError::InvalidInput(format!(
        "family smoke failed for `{}`:\n- {}",
        family.as_str(),
        failures.join("\n- ")
    )))
}

pub(crate) fn collect_smoke_failures(
    committed_paths: &PacketPaths,
    scaffolded_paths: &PacketPaths,
    harness: FamilyHarness,
) -> Result<Vec<String>, XtaskError> {
    let mut failures = Vec::new();

    let committed_manifest = read_file_bytes(&committed_paths.manifest, "committed family.toml")?;
    let scaffolded_manifest =
        read_file_bytes(&scaffolded_paths.manifest, "scaffolded family.toml")?;
    if committed_manifest != scaffolded_manifest {
        failures.push(format!(
            "committed `family.toml` at `{}` does not match scaffolded `family.toml` from `cargo xtask family new`",
            committed_paths.manifest.display()
        ));
    }

    for case in harness.scaffold.starter_cases {
        let path = scaffolded_paths.root.join(case.path);
        match fs::metadata(&path) {
            Ok(metadata) if metadata.is_file() => {}
            Ok(_) => failures.push(format!(
                "scaffolded starter case `{}` exists but is not a regular file",
                path.display()
            )),
            Err(error) => failures.push(format!(
                "scaffolded starter case `{}` is missing: {error}",
                path.display()
            )),
        }
    }

    failures.extend(template_specific_failures(scaffolded_paths, harness)?);

    Ok(failures)
}

fn template_specific_failures(
    scaffolded_paths: &PacketPaths,
    harness: FamilyHarness,
) -> Result<Vec<String>, XtaskError> {
    match harness.scaffold.template {
        StarterTemplate::GenericPlaceholder => Ok(Vec::new()),
        StarterTemplate::ArithmeticLeafMonotoneDownNonnegative => {
            arithmetic_leaf_aligned_failures(scaffolded_paths, harness)
        }
    }
}

fn arithmetic_leaf_aligned_failures(
    scaffolded_paths: &PacketPaths,
    harness: FamilyHarness,
) -> Result<Vec<String>, XtaskError> {
    let aligned_case = harness
        .starter_cases_for_bucket("aligned")
        .next()
        .ok_or_else(|| {
            XtaskError::Internal(format!(
                "family `{}` has no locked aligned starter case",
                harness.family
            ))
        })?;
    let aligned_path = scaffolded_paths.root.join(aligned_case.path);
    let aligned = read_file_string(&aligned_path, "scaffolded aligned starter spec")?;
    let mut failures = Vec::new();

    for needle in [
        "subtotal: Decimal",
        "rate: Decimal",
        "- output <= subtotal",
        "- output >= 0",
    ] {
        if !aligned.contains(needle) {
            failures.push(format!(
                "aligned starter spec `{}` is missing `{needle}`",
                aligned_path.display()
            ));
        }
    }

    if aligned.contains("deps:") && !aligned.contains("money/round") {
        failures.push(format!(
            "aligned starter spec `{}` mentions deps but does not include optional helper dep `money/round`",
            aligned_path.display()
        ));
    }

    Ok(failures)
}

fn read_file_bytes(path: &Path, label: &str) -> Result<Vec<u8>, XtaskError> {
    fs::read(path).map_err(|error| {
        XtaskError::InvalidInput(format!(
            "failed to read {label} `{}`: {error}",
            path.display()
        ))
    })
}

fn read_file_string(path: &Path, label: &str) -> Result<String, XtaskError> {
    fs::read_to_string(path).map_err(|error| {
        XtaskError::InvalidInput(format!(
            "failed to read {label} `{}`: {error}",
            path.display()
        ))
    })
}
