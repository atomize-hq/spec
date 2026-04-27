use crate::XtaskError;
use crate::family::paths::{FamilyId, PacketPaths, REQUIRED_BUCKETS, ensure_packet_path_safe};
use std::fs;
use std::path::Path;

pub fn run(workspace_root: &Path, raw_family: &str) -> Result<(), XtaskError> {
    let family = FamilyId::parse(raw_family)?;
    let paths = PacketPaths::new(workspace_root, family.clone());
    ensure_packet_path_safe(workspace_root, &paths.root)?;

    match fs::symlink_metadata(&paths.root) {
        Ok(_) => {
            return Err(XtaskError::AlreadyExists(format!(
                "packet `{}` already exists at `{}`",
                family.as_str(),
                paths.root.display()
            )));
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => {
            return Err(XtaskError::WriteFailure(format!(
                "failed to inspect packet root `{}`: {error}",
                paths.root.display()
            )));
        }
    }

    let family_root = workspace_root.join("semantic-families");
    let family_root_metadata = fs::metadata(&family_root).map_err(|error| {
        XtaskError::WriteFailure(format!(
            "semantic family root `{}` is unavailable: {error}",
            family_root.display()
        ))
    })?;
    if !family_root_metadata.is_dir() {
        return Err(XtaskError::WriteFailure(format!(
            "semantic family root `{}` is not a directory",
            family_root.display()
        )));
    }

    fs::create_dir(&paths.root).map_err(|error| {
        XtaskError::WriteFailure(format!(
            "failed to create packet root `{}`: {error}",
            paths.root.display()
        ))
    })?;

    write_file(&paths.candidate, &candidate_template(&family))?;
    write_file(&paths.manifest, &manifest_template(&family))?;
    fs::create_dir(&paths.fixtures).map_err(|error| {
        XtaskError::WriteFailure(format!(
            "failed to create fixtures directory `{}`: {error}",
            paths.fixtures.display()
        ))
    })?;

    for bucket in REQUIRED_BUCKETS {
        create_bucket(&paths, &family, bucket)?;
    }

    Ok(())
}

fn create_bucket(paths: &PacketPaths, family: &FamilyId, bucket: &str) -> Result<(), XtaskError> {
    let bucket_root = paths.fixtures.join(bucket);
    fs::create_dir(&bucket_root).map_err(|error| {
        XtaskError::WriteFailure(format!(
            "failed to create bucket directory `{}`: {error}",
            bucket_root.display()
        ))
    })?;
    fs::create_dir(bucket_root.join("src")).map_err(|error| {
        XtaskError::WriteFailure(format!(
            "failed to create bucket src directory `{}`: {error}",
            bucket_root.join("src").display()
        ))
    })?;
    fs::create_dir_all(bucket_root.join("units/namespace")).map_err(|error| {
        XtaskError::WriteFailure(format!(
            "failed to create bucket units directory `{}`: {error}",
            bucket_root.join("units/namespace").display()
        ))
    })?;

    write_file(
        &bucket_root.join("Cargo.toml"),
        &bucket_cargo_toml(family, bucket),
    )?;
    write_file(&bucket_root.join("src/main.rs"), bucket_main_rs())?;

    Ok(())
}

fn write_file(path: &Path, contents: &str) -> Result<(), XtaskError> {
    fs::write(path, contents).map_err(|error| {
        XtaskError::WriteFailure(format!("failed to write `{}`: {error}", path.display()))
    })
}

fn candidate_template(family: &FamilyId) -> String {
    format!(
        "# {}\n\nSummary: TODO: replace with a one-line family summary.\n\n## Aligned\n\n- TODO: list each aligned fixture path exactly once.\n\n## Drift\n\n- TODO: list each drift fixture path exactly once.\n\n## Under Specified\n\n- TODO: list each under-specified fixture path exactly once.\n\n## Unsupported Near Miss\n\n- TODO: list each unsupported near miss fixture path exactly once.\n",
        family.as_str()
    )
}

fn manifest_template(family: &FamilyId) -> String {
    format!(
        r#"schema_version = 1
family = "{family_id}"
kind = "function"
compatibility_key = "{family_id}"
summary = "TODO: replace with a one-line family summary."

[routing]
precedence = 1
must_not_shadow = ["function.wrapper.pipeline.v1"]

[shape]
dep_count = 3
control_flow = "straight_line_only"
return_style = "let_then_return_or_direct_return"
loops = false
branching = false
requires_supported_function_deps = true

[args]
threading = "ordered_passthrough"
allow_nested_argument_expressions = false
allow_literal_only_extra_args = false

[corpus]
required_buckets = ["aligned", "drift", "under_specified", "unsupported_near_miss"]
min_cases_per_bucket = 1

[truth_surface]
requires_refresh_via = ["spec test"]
preserve_only_via = ["spec build", "spec generate", "spec status", "spec export"]
requires_stale_demote = true

[gates]
gate_a = true
gate_b = true
gate_c = true
gate_d = true
"#,
        family_id = family.as_str(),
    )
}

fn bucket_cargo_toml(family: &FamilyId, bucket: &str) -> String {
    format!(
        "[package]\nname = \"{}-{}\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[workspace]\n",
        family.crate_stem(),
        bucket.replace('_', "-")
    )
}

fn bucket_main_rs() -> &'static str {
    "mod generated;\npub use generated::*;\n\nfn main() {}\n"
}
