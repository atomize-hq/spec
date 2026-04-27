use crate::XtaskError;
use crate::family::paths::{FamilyId, PacketPaths, REQUIRED_BUCKETS, ensure_packet_path_safe};
use crate::family::routing::{CHAIN3_MUST_NOT_SHADOW, CHAIN3_PRECEDENCE};
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
    fs::create_dir_all(bucket_root.join("units/pricing")).map_err(|error| {
        XtaskError::WriteFailure(format!(
            "failed to create bucket units directory `{}`: {error}",
            bucket_root.join("units/pricing").display()
        ))
    })?;

    write_file(
        &bucket_root.join("Cargo.toml"),
        &bucket_cargo_toml(family, bucket),
    )?;
    write_file(&bucket_root.join("src/main.rs"), bucket_main_rs())?;
    for (filename, contents) in starter_unit_specs(bucket) {
        write_file(&bucket_root.join("units/pricing").join(filename), &contents)?;
    }

    Ok(())
}

fn write_file(path: &Path, contents: &str) -> Result<(), XtaskError> {
    fs::write(path, contents).map_err(|error| {
        XtaskError::WriteFailure(format!("failed to write `{}`: {error}", path.display()))
    })
}

fn candidate_template(family: &FamilyId) -> String {
    let aligned = starter_paths_for_bucket("aligned");
    let drift = starter_paths_for_bucket("drift");
    let under_specified = starter_paths_for_bucket("under_specified");
    let unsupported_near_miss = starter_paths_for_bucket("unsupported_near_miss");
    format!(
        "# {}\n\nSummary: TODO: replace with a one-line family summary.\n\n## Aligned\n\n{}\n\n## Drift\n\n{}\n\n## Under Specified\n\n{}\n\n## Unsupported Near Miss\n\n{}\n",
        family.as_str(),
        aligned
            .iter()
            .map(|path| format!("- `{path}`"))
            .collect::<Vec<_>>()
            .join("\n"),
        drift
            .iter()
            .map(|path| format!("- `{path}`"))
            .collect::<Vec<_>>()
            .join("\n"),
        under_specified
            .iter()
            .map(|path| format!("- `{path}`"))
            .collect::<Vec<_>>()
            .join("\n"),
        unsupported_near_miss
            .iter()
            .map(|path| format!("- `{path}`"))
            .collect::<Vec<_>>()
            .join("\n"),
    )
}

fn manifest_template(family: &FamilyId) -> String {
    let must_not_shadow = CHAIN3_MUST_NOT_SHADOW
        .iter()
        .map(|value| format!("  \"{value}\","))
        .collect::<Vec<_>>()
        .join("\n");
    format!(
        r#"schema_version = 1
family = "{family_id}"
kind = "function"
compatibility_key = "{family_id}"
summary = "TODO: replace with a one-line family summary."

[routing]
precedence = {precedence}
must_not_shadow = [
{must_not_shadow}
]

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
        precedence = CHAIN3_PRECEDENCE,
        must_not_shadow = must_not_shadow,
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

fn starter_paths_for_bucket(bucket: &str) -> [String; 4] {
    [
        format!("fixtures/{bucket}/units/pricing/pricing_discount_leaf_{bucket}.unit.spec"),
        format!("fixtures/{bucket}/units/pricing/pricing_tax_leaf_{bucket}.unit.spec"),
        format!("fixtures/{bucket}/units/pricing/pricing_total_wrapper_{bucket}.unit.spec"),
        format!("fixtures/{bucket}/units/pricing/checkout_chain3_{bucket}.unit.spec"),
    ]
}

fn starter_unit_specs(bucket: &str) -> Vec<(String, String)> {
    starter_paths_for_bucket(bucket)
        .into_iter()
        .map(|path| {
            let filename = path
                .rsplit('/')
                .next()
                .expect("starter path must end in a filename")
                .to_string();
            let unit_id = format!("pricing/{}", filename.trim_end_matches(".unit.spec"));
            let callable_name = unit_id
                .split_once('/')
                .map(|(_, name)| name)
                .unwrap_or(unit_id.as_str());
            (filename, starter_unit_spec(&unit_id, callable_name))
        })
        .collect()
}

fn starter_unit_spec(unit_id: &str, callable_name: &str) -> String {
    format!(
        r#"id: {unit_id}
kind: function
spec_version: "0.3.0"
intent:
  why: "TODO: replace this scaffolded placeholder with real authored behavior."
contract:
  inputs:
    amount: Decimal
  returns: Decimal
imports:
  - rust_decimal::Decimal
body:
  rust: |
    {{
        if amount > Decimal::ZERO {{
            amount
        }} else {{
            Decimal::ZERO
        }}
    }}
local_tests:
  - id: {callable_name}_placeholder
    expect: {callable_name}(Decimal::new(10000, 2)) == Decimal::new(10000, 2)
"#
    )
}
