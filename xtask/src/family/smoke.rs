use crate::XtaskError;
use crate::family::harness::{
    FamilyHarness, registered_family_harnesses, require_family_harness_in,
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

    failures.extend(collect_exact_match_failures(
        committed_paths,
        scaffolded_paths,
        harness,
    )?);

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

    failures.extend(collect_file_contract_failures(scaffolded_paths, harness)?);

    Ok(failures)
}

fn collect_exact_match_failures(
    committed_paths: &PacketPaths,
    scaffolded_paths: &PacketPaths,
    harness: FamilyHarness,
) -> Result<Vec<String>, XtaskError> {
    let mut failures = Vec::new();

    for relative_path in harness.scaffold.smoke.scaffold_exact_match_paths {
        let committed_path = committed_paths.root.join(relative_path);
        let scaffolded_path = scaffolded_paths.root.join(relative_path);
        let committed = read_file_bytes(&committed_path, "committed scaffold exact-match file")?;
        let scaffolded = read_file_bytes(&scaffolded_path, "scaffolded scaffold exact-match file")?;
        if committed != scaffolded {
            failures.push(format!(
                "committed scaffold exact-match file `{}` does not match scaffolded file from `cargo xtask family new`",
                committed_path.display()
            ));
        }
    }

    Ok(failures)
}

fn collect_file_contract_failures(
    scaffolded_paths: &PacketPaths,
    harness: FamilyHarness,
) -> Result<Vec<String>, XtaskError> {
    let mut failures = Vec::new();

    for contract in harness.scaffold.smoke.scaffold_file_contracts {
        let path = scaffolded_paths.root.join(contract.path);
        let contents = read_file_string(&path, "scaffolded smoke-contract file")?;

        for needle in contract.required_contents {
            if !contents.contains(needle) {
                failures.push(format!(
                    "scaffolded smoke-contract file `{}` is missing required content `{needle}`",
                    path.display()
                ));
            }
        }

        for needle in contract.forbidden_contents {
            if contents.contains(needle) {
                failures.push(format!(
                    "scaffolded smoke-contract file `{}` contains forbidden content `{needle}`",
                    path.display()
                ));
            }
        }
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
