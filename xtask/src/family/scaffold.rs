use crate::XtaskError;
use crate::family::harness::{
    FamilyHarness, StarterCaseDefinition, StarterTemplate, require_family_harness,
};
use crate::family::paths::{FamilyId, PacketPaths, REQUIRED_BUCKETS, ensure_packet_path_safe};
use std::fs;
use std::path::Path;

pub fn run(workspace_root: &Path, raw_family: &str) -> Result<(), XtaskError> {
    let requested_family = FamilyId::parse(raw_family)?;
    let harness = require_family_harness(&requested_family, "family new")?;
    let family = harness_family_id(harness);
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

    write_file(&paths.candidate, &candidate_template(&family, harness))?;
    write_file(&paths.manifest, &manifest_template(&family, harness))?;
    fs::create_dir(&paths.fixtures).map_err(|error| {
        XtaskError::WriteFailure(format!(
            "failed to create fixtures directory `{}`: {error}",
            paths.fixtures.display()
        ))
    })?;

    for bucket in REQUIRED_BUCKETS {
        create_bucket(&paths, harness, bucket)?;
    }

    Ok(())
}

fn create_bucket(
    paths: &PacketPaths,
    harness: &FamilyHarness,
    bucket: &str,
) -> Result<(), XtaskError> {
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
    fs::create_dir_all(
        bucket_root
            .join("units")
            .join(harness.scaffold.unit_namespace),
    )
    .map_err(|error| {
        XtaskError::WriteFailure(format!(
            "failed to create bucket units directory `{}`: {error}",
            bucket_root
                .join("units")
                .join(harness.scaffold.unit_namespace)
                .display()
        ))
    })?;

    write_file(
        &bucket_root.join("Cargo.toml"),
        &bucket_cargo_toml(&harness_family_id(harness), bucket),
    )?;
    write_file(&bucket_root.join("src/main.rs"), bucket_main_rs())?;
    for case in harness.starter_cases_for_bucket(bucket) {
        let destination = paths.root.join(case.path);
        write_file(&destination, &starter_unit_spec(harness, case))?;
    }

    Ok(())
}

fn write_file(path: &Path, contents: &str) -> Result<(), XtaskError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| {
            XtaskError::WriteFailure(format!(
                "failed to create parent directory `{}`: {error}",
                parent.display()
            ))
        })?;
    }
    fs::write(path, contents).map_err(|error| {
        XtaskError::WriteFailure(format!("failed to write `{}`: {error}", path.display()))
    })
}

fn harness_family_id(harness: &FamilyHarness) -> FamilyId {
    FamilyId::parse(harness.family).expect("registered family harness ids must be valid")
}

fn candidate_template(family: &FamilyId, harness: &FamilyHarness) -> String {
    format!(
        "# {}\n\nSummary: {}\n\n## Aligned\n\n{}\n\n## Drift\n\n{}\n\n## Under Specified\n\n{}\n\n## Unsupported Near Miss\n\n{}\n",
        family.as_str(),
        harness.summary,
        render_markdown_path_list(&starter_paths_for_bucket(harness, "aligned")),
        render_markdown_path_list(&starter_paths_for_bucket(harness, "drift")),
        render_markdown_path_list(&starter_paths_for_bucket(harness, "under_specified")),
        render_markdown_path_list(&starter_paths_for_bucket(harness, "unsupported_near_miss")),
    )
}

fn manifest_template(family: &FamilyId, harness: &FamilyHarness) -> String {
    let must_not_shadow = render_toml_string_array(harness.routing.must_not_shadow);
    format!(
        r#"schema_version = 2
family = "{family_id}"
kind = "function"
compatibility_key = "{family_id}"
summary = "{summary}"

[routing]
precedence = {precedence}
must_not_shadow = [
{must_not_shadow}
]

[shape]
dep_min = {dep_min}
dep_max = {dep_max}
control_flow = "{control_flow}"
return_style = "{return_style}"
loops = {loops}
branching = {branching}
requires_supported_function_deps = {requires_supported_function_deps}

[args]
threading = "{threading}"
allow_nested_argument_expressions = {allow_nested_argument_expressions}
allow_literal_only_extra_args = {allow_literal_only_extra_args}

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
        summary = harness.summary,
        precedence = harness.routing.precedence,
        must_not_shadow = must_not_shadow,
        dep_min = harness.shape.dep_min,
        dep_max = harness.shape.dep_max,
        control_flow = harness.shape.control_flow,
        return_style = harness.shape.return_style,
        loops = harness.shape.loops,
        branching = harness.shape.branching,
        requires_supported_function_deps = harness.shape.requires_supported_function_deps,
        threading = harness.args.threading,
        allow_nested_argument_expressions = harness.args.allow_nested_argument_expressions,
        allow_literal_only_extra_args = harness.args.allow_literal_only_extra_args,
    )
}

fn render_markdown_path_list(paths: &[&str]) -> String {
    paths
        .iter()
        .map(|path| format!("- `{path}`"))
        .collect::<Vec<_>>()
        .join("\n")
}

fn render_toml_string_array(values: &[&str]) -> String {
    values
        .iter()
        .map(|value| format!("  \"{value}\","))
        .collect::<Vec<_>>()
        .join("\n")
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

fn starter_paths_for_bucket<'a>(harness: &'a FamilyHarness, bucket: &str) -> Vec<&'a str> {
    harness
        .starter_cases_for_bucket(bucket)
        .map(|definition| definition.path)
        .collect()
}

fn starter_unit_spec(harness: &FamilyHarness, case: StarterCaseDefinition) -> String {
    let filename = case
        .path
        .rsplit('/')
        .next()
        .expect("starter path must end in a filename");
    let unit_id = format!(
        "{}/{}",
        harness.scaffold.unit_namespace,
        filename.trim_end_matches(".unit.spec")
    );
    let callable_name = unit_id
        .split_once('/')
        .map(|(_, name)| name)
        .unwrap_or(unit_id.as_str());

    match harness.scaffold.template {
        StarterTemplate::GenericPlaceholder => generic_placeholder_starter(&unit_id, callable_name),
        StarterTemplate::WrapperPipelineTwoStep => {
            wrapper_pipeline_two_step_starter(case.bucket, &unit_id, callable_name)
        }
        StarterTemplate::ArithmeticLeafMonotoneDownNonnegative => {
            arithmetic_leaf_monotone_down_nonnegative_starter(case.bucket, &unit_id, callable_name)
        }
        StarterTemplate::ArithmeticLeafMonotoneUp => {
            arithmetic_leaf_monotone_up_starter(case.bucket, &unit_id, callable_name)
        }
    }
}

fn generic_placeholder_starter(unit_id: &str, callable_name: &str) -> String {
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

fn wrapper_pipeline_two_step_starter(bucket: &str, unit_id: &str, callable_name: &str) -> String {
    if unit_id.contains("pricing_discount_leaf_") {
        return wrapper_pipeline_discount_leaf_starter(unit_id, callable_name);
    }
    if unit_id.contains("pricing_tax_leaf_") {
        return wrapper_pipeline_tax_leaf_starter(unit_id, callable_name);
    }
    if unit_id.contains("pricing_total_wrapper_") {
        return wrapper_pipeline_total_wrapper_starter(bucket, unit_id, callable_name);
    }

    panic!("unexpected wrapper pipeline starter unit `{unit_id}`");
}

fn wrapper_pipeline_discount_leaf_starter(unit_id: &str, callable_name: &str) -> String {
    format!(
        r#"id: {unit_id}
kind: function
spec_version: "0.3.0"
intent:
  why: Return the running checkout subtotal after applying the loyalty discount rate and clamping at zero.
contract:
  inputs:
    subtotal: Decimal
    rate: Decimal
  returns: Decimal
  invariants:
    - output <= subtotal
    - output >= 0
imports:
  - rust_decimal::Decimal
body:
  rust: |
    {{
        (subtotal - subtotal * rate).max(Decimal::ZERO)
    }}
  typescript: |
    {{
        const discounted = subtotal - subtotal * rate;
        return discounted >= Decimal.ZERO ? discounted : Decimal.ZERO;
    }}
local_tests:
  - id: {callable_name}_basic
    expect: {callable_name}(Decimal::new(10000, 2), Decimal::new(10, 2)) == Decimal::new(9000, 2)
"#
    )
}

fn wrapper_pipeline_tax_leaf_starter(unit_id: &str, callable_name: &str) -> String {
    format!(
        r#"id: {unit_id}
kind: function
spec_version: "0.3.0"
intent:
  why: Return the running checkout subtotal after applying the surcharge rate.
contract:
  inputs:
    subtotal: Decimal
    rate: Decimal
  returns: Decimal
  invariants:
    - output >= subtotal
imports:
  - rust_decimal::Decimal
body:
  rust: |
    {{
        subtotal + subtotal * rate
    }}
  typescript: |
    {{
        return subtotal + subtotal * rate;
    }}
local_tests:
  - id: {callable_name}_basic
    expect: {callable_name}(Decimal::new(10000, 2), Decimal::new(10, 2)) == Decimal::new(11000, 2)
"#
    )
}

fn wrapper_pipeline_total_wrapper_starter(
    bucket: &str,
    unit_id: &str,
    callable_name: &str,
) -> String {
    let bucket_suffix = bucket;
    let (intent_why, body, typescript_body) = match bucket {
        "aligned" => (
            "Return the checkout total after discounting the subtotal and then applying tax.",
            format!(
                "{{\n        let discounted = pricing_discount_leaf_{bucket_suffix}(subtotal, discount_rate);\n        pricing_tax_leaf_{bucket_suffix}(discounted, tax_rate)\n    }}"
            ),
            format!(
                "{{\n        const discounted = pricing_discount_leaf_{bucket_suffix}(subtotal, discount_rate);\n        return pricing_tax_leaf_{bucket_suffix}(discounted, tax_rate);\n    }}"
            ),
        ),
        "drift" => (
            "Return the checkout total after discounting the subtotal and then applying tax.",
            format!(
                "{{\n        let taxed = pricing_tax_leaf_{bucket_suffix}(subtotal, tax_rate);\n        pricing_discount_leaf_{bucket_suffix}(taxed, discount_rate)\n    }}"
            ),
            format!(
                "{{\n        const taxed = pricing_tax_leaf_{bucket_suffix}(subtotal, tax_rate);\n        return pricing_discount_leaf_{bucket_suffix}(taxed, discount_rate);\n    }}"
            ),
        ),
        "under_specified" => (
            "todo",
            format!(
                "{{\n        let discounted = pricing_discount_leaf_{bucket_suffix}(subtotal, discount_rate);\n        pricing_tax_leaf_{bucket_suffix}(discounted, tax_rate)\n    }}"
            ),
            format!(
                "{{\n        const discounted = pricing_discount_leaf_{bucket_suffix}(subtotal, discount_rate);\n        return pricing_tax_leaf_{bucket_suffix}(discounted, tax_rate);\n    }}"
            ),
        ),
        "unsupported_near_miss" => (
            "Return the checkout total after discounting the subtotal and then applying tax.",
            format!(
                "{{\n        let discounted = pricing_discount_leaf_{bucket_suffix}(subtotal, discount_rate);\n        pricing_tax_leaf_{bucket_suffix}(discounted, tax_rate.max(Decimal::ZERO))\n    }}"
            ),
            format!(
                "{{\n        const discounted = pricing_discount_leaf_{bucket_suffix}(subtotal, discount_rate);\n        return pricing_tax_leaf_{bucket_suffix}(\n            discounted,\n            tax_rate >= Decimal.ZERO ? tax_rate : Decimal.ZERO\n        );\n    }}"
            ),
        ),
        other => panic!("unexpected wrapper pipeline bucket `{other}`"),
    };

    format!(
        r#"id: {unit_id}
kind: function
spec_version: "0.3.0"
intent:
  why: {intent_why}
contract:
  inputs:
    subtotal: Decimal
    discount_rate: Decimal
    tax_rate: Decimal
  returns: Decimal
deps:
  - pricing/pricing_discount_leaf_{bucket_suffix}
  - pricing/pricing_tax_leaf_{bucket_suffix}
imports:
  - rust_decimal::Decimal
body:
  rust: |
    {body}
  typescript: |
    {typescript_body}
local_tests:
  - id: {callable_name}_basic
    expect: {callable_name}(Decimal::new(10000, 2), Decimal::new(10, 2), Decimal::new(10, 2)) == Decimal::new(9900, 2)
"#
    )
}

fn arithmetic_leaf_monotone_down_nonnegative_starter(
    bucket: &str,
    unit_id: &str,
    callable_name: &str,
) -> String {
    match bucket {
        "aligned" => format!(
            r#"id: {unit_id}
kind: function
spec_version: "0.3.0"
intent:
  why: Apply a discount to a subtotal while keeping the result nonnegative.
contract:
  inputs:
    subtotal: Decimal
    rate: Decimal
  returns: Decimal
  invariants:
    - output <= subtotal
    - output >= 0
deps:
  - money/round
imports:
  - rust_decimal::Decimal
body:
  rust: |
    {{
        let discounted = subtotal - subtotal * rate;
        round(discounted.max(Decimal::ZERO))
    }}
local_tests:
  - id: {callable_name}_happy_path
    expect: {callable_name}(Decimal::new(10000, 2), Decimal::new(10, 2)) == Decimal::new(9000, 2)
"#
        ),
        "drift" => format!(
            r#"id: {unit_id}
kind: function
spec_version: "0.3.0"
intent:
  why: Apply a discount to a subtotal while keeping the result nonnegative.
contract:
  inputs:
    subtotal: Decimal
    rate: Decimal
  returns: Decimal
  invariants:
    - output <= subtotal
    - output >= 0
deps:
  - money/round
imports:
  - rust_decimal::Decimal
body:
  rust: |
    {{
        let discounted = subtotal + subtotal * rate;
        round(discounted.max(Decimal::ZERO))
    }}
local_tests:
  - id: {callable_name}_drift
    expect: {callable_name}(Decimal::new(10000, 2), Decimal::new(10, 2)) == Decimal::new(11000, 2)
"#
        ),
        "under_specified" => format!(
            r#"id: {unit_id}
kind: function
spec_version: "0.3.0"
intent:
  why: Adjust a subtotal.
contract:
  inputs:
    subtotal: Decimal
    rate: Decimal
  returns: Decimal
deps:
  - money/round
imports:
  - rust_decimal::Decimal
body:
  rust: |
    {{
        let discounted = subtotal - subtotal * rate;
        round(discounted.max(Decimal::ZERO))
    }}
local_tests:
  - id: {callable_name}_under_specified
    expect: {callable_name}(Decimal::new(10000, 2), Decimal::new(10, 2)) == Decimal::new(9000, 2)
"#
        ),
        "unsupported_near_miss" => format!(
            r#"id: {unit_id}
kind: function
spec_version: "0.3.0"
intent:
  why: Apply a discount to a subtotal while keeping the result nonnegative.
contract:
  inputs:
    subtotal: Decimal
    rate: Decimal
  returns: Decimal
  invariants:
    - output <= subtotal
    - output >= 0
deps:
  - money/round
imports:
  - rust_decimal::Decimal
body:
  rust: |
    {{
        let discounted = subtotal - subtotal * rate;
        if discounted < Decimal::ZERO {{
            Decimal::ZERO
        }} else {{
            round(discounted)
        }}
    }}
local_tests:
  - id: {callable_name}_unsupported_near_miss
    expect: {callable_name}(Decimal::new(10000, 2), Decimal::new(10, 2)) == Decimal::new(9000, 2)
"#
        ),
        other => panic!("unexpected arithmetic leaf bucket `{other}`"),
    }
}

fn arithmetic_leaf_monotone_up_starter(bucket: &str, unit_id: &str, callable_name: &str) -> String {
    match bucket {
        "aligned" => format!(
            r#"id: {unit_id}
kind: function
spec_version: "0.3.0"
intent:
  why: Add sales tax to a subtotal using a rate expressed as a decimal fraction.
contract:
  inputs:
    subtotal: Decimal
    rate: Decimal
  returns: Decimal
  invariants:
    - output >= subtotal
deps:
  - money/round
imports:
  - rust_decimal::Decimal
body:
  rust: |
    {{
        let taxed = subtotal + subtotal * rate;
        round(taxed)
    }}
  typescript: |
    {{
        const taxed = subtotal + subtotal * rate;
        return round(taxed);
    }}
local_tests:
  - id: {callable_name}_happy_path
    expect: {callable_name}(Decimal::new(10000, 2), Decimal::new(725, 4)) == Decimal::new(10725, 2)
"#
        ),
        "drift" => format!(
            r#"id: {unit_id}
kind: function
spec_version: "0.3.0"
intent:
  why: Add sales tax to a subtotal using a rate expressed as a decimal fraction.
contract:
  inputs:
    subtotal: Decimal
    rate: Decimal
  returns: Decimal
  invariants:
    - output >= subtotal
deps:
  - money/round
imports:
  - rust_decimal::Decimal
body:
  rust: |
    {{
        let taxed = subtotal - subtotal * rate;
        round(taxed.max(Decimal::ZERO))
    }}
  typescript: |
    {{
        const taxed = subtotal - subtotal * rate;
        return round(taxed >= Decimal.ZERO ? taxed : Decimal.ZERO);
    }}
local_tests:
  - id: {callable_name}_drift
    expect: {callable_name}(Decimal::new(10000, 2), Decimal::new(725, 4)) == Decimal::new(9275, 2)
"#
        ),
        "under_specified" => format!(
            r#"id: {unit_id}
kind: function
spec_version: "0.3.0"
intent:
  why: todo
contract:
  inputs:
    subtotal: Decimal
    rate: Decimal
  returns: Decimal
  invariants:
    - output >= subtotal
deps:
  - money/round
imports:
  - rust_decimal::Decimal
body:
  rust: |
    {{
        let taxed = subtotal + subtotal * rate;
        round(taxed)
    }}
  typescript: |
    {{
        const taxed = subtotal + subtotal * rate;
        return round(taxed);
    }}
local_tests:
  - id: {callable_name}_under_specified
    expect: {callable_name}(Decimal::new(10000, 2), Decimal::new(725, 4)) == Decimal::new(10725, 2)
"#
        ),
        "unsupported_near_miss" => format!(
            r#"id: {unit_id}
kind: function
spec_version: "0.3.0"
intent:
  why: Add sales tax to a subtotal using a rate expressed as a decimal fraction.
contract:
  inputs:
    subtotal: Decimal
    rate: Decimal
  returns: Decimal
  invariants:
    - output >= subtotal
deps:
  - money/round
imports:
  - rust_decimal::Decimal
body:
  rust: |
    {{
        let taxed = subtotal + subtotal * rate;
        if rate == Decimal::ZERO {{
            subtotal
        }} else {{
            round(taxed)
        }}
    }}
  typescript: |
    {{
        const taxed = subtotal + subtotal * rate;
        if (rate === Decimal.ZERO) {{
            return subtotal;
        }}
        return round(taxed);
    }}
local_tests:
  - id: {callable_name}_unsupported_near_miss
    expect: {callable_name}(Decimal::new(10000, 2), Decimal::new(725, 4)) == Decimal::new(10725, 2)
"#
        ),
        other => panic!("unexpected arithmetic leaf bucket `{other}`"),
    }
}
