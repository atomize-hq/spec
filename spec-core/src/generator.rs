//! Generator module: Generate Rust code from ResolvedSpec
//!
//! Implements the M1 generation path from PLAN.md:
//! - prepend `use ...` imports for imports + deps
//! - write generated `.rs` files
//! - generate `mod.rs` contents
//! - owned-tree orphan cleanup with `.spec-generated` marker safety rails

use crate::graph::{ProjectedUnitRef, project_unit};
use crate::syntax::validate_expect_expr;
use crate::types::{
    DepRef, LocalTest, MethodReceiver, NormalizedDataSeam, NormalizedSumSeam, NormalizedUnit,
    ResolvedMoleculeTest, ResolvedSpec, RustDataSeamLowering, RustInherentMethodLowering,
    RustSumSeamLowering, TargetLanguage, has_callable_collision,
};
use crate::{Result, SpecError};
use indexmap::IndexMap;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs::{self, File};
use std::io::Write;
use std::path::{Component, Path, PathBuf};
use walkdir::WalkDir;

const GENERATED_MARKER: &str = ".spec-generated";

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct GenerateOptions {
    pub allow_unsafe_local_test_expect: bool,
}

fn build_named_inputs(inputs: &IndexMap<String, String>) -> String {
    inputs
        .iter()
        .map(|(name, ty)| format!("{name}: {ty}"))
        .collect::<Vec<_>>()
        .join(", ")
}

fn build_fn_signature(spec: &ResolvedSpec) -> String {
    let params = spec
        .contract
        .as_ref()
        .and_then(|c| c.inputs.as_ref())
        .map(build_named_inputs)
        .unwrap_or_default();

    let return_type = spec
        .contract
        .as_ref()
        .and_then(|c| c.returns.as_ref())
        .map(|r| format!(" -> {}", r));

    match return_type {
        Some(ret) => format!("pub fn {}({}){}", spec.fn_name, params, ret),
        None => format!("pub fn {}({})", spec.fn_name, params),
    }
}

fn build_doc_comment(intent_why: &str) -> Option<String> {
    let trimmed = intent_why.trim();
    if trimmed.is_empty() {
        return None;
    }

    let mut output = String::new();
    for line in trimmed.lines() {
        if line.trim().is_empty() {
            output.push_str("///\n");
        } else {
            output.push_str("/// ");
            output.push_str(line);
            output.push('\n');
        }
    }

    Some(output)
}

fn render_local_tests(
    unit_id: &str,
    local_tests: &[LocalTest],
    options: &GenerateOptions,
) -> Result<Option<String>> {
    if local_tests.is_empty() {
        return Ok(None);
    }

    let mut output = String::new();
    output.push_str("#[cfg(test)]\n");
    output.push_str("mod tests {\n");
    output.push_str("    use super::*;\n\n");

    for (index, local_test) in local_tests.iter().enumerate() {
        let expect = local_test.expect.trim();
        validate_expect_expr(expect, options.allow_unsafe_local_test_expect).map_err(|err| {
            SpecError::Generator {
                message: format!(
                    "invalid local test expect for unit '{}' test '{}': {}",
                    unit_id,
                    local_test.id,
                    err.message()
                ),
            }
        })?;
        output.push_str("    #[test]\n");
        output.push_str(&format!("    fn test_{}() {{\n", local_test.id));
        output.push_str(&format!("        assert!({expect});\n"));
        output.push_str("    }\n");

        if index + 1 != local_tests.len() {
            output.push('\n');
        }
    }

    output.push_str("}\n");
    Ok(Some(output))
}

pub fn generate_code(spec: &ResolvedSpec) -> Result<String> {
    generate_code_with_options(spec, &GenerateOptions::default())
}

pub fn generate_code_with_options(
    spec: &ResolvedSpec,
    options: &GenerateOptions,
) -> Result<String> {
    let (import_statements, dep_statements) = build_use_groups(spec)?;
    let mut output = String::new();

    for statement in import_statements {
        output.push_str(&statement);
        output.push('\n');
    }

    if !spec.imports.is_empty() && !spec.deps.is_empty() {
        output.push('\n');
    }

    for statement in dep_statements {
        output.push_str(&statement);
        output.push('\n');
    }

    if !spec.imports.is_empty() || !spec.deps.is_empty() {
        output.push('\n');
    }

    if let Some(doc_comment) = build_doc_comment(&spec.intent_why) {
        output.push_str(&doc_comment);
    }

    let signature = build_fn_signature(spec);
    let block = spec.body_rust.trim();
    output.push_str(&format!("{signature} {block}"));
    output.push('\n');

    if let Some(tests) = render_local_tests(&spec.id, &spec.local_tests, options)? {
        // One blank line between the generated unit body and the tests module.
        output.push('\n');
        output.push_str(&tests);
    }
    Ok(output)
}

pub fn lower_data_seam(unit: &NormalizedDataSeam) -> Result<RustDataSeamLowering> {
    let constructors = unit
        .constructors
        .iter()
        .map(|constructor| RustInherentMethodLowering {
            id: constructor.id.clone(),
            is_constructor: true,
            receiver: None,
            inputs: constructor.inputs.clone(),
            returns: Some("Self".to_string()),
            body_rust: render_constructor_body(unit, &constructor.initializes),
        })
        .collect::<Vec<_>>();

    let methods = unit
        .methods
        .iter()
        .map(|method| RustInherentMethodLowering {
            id: method.id.clone(),
            is_constructor: false,
            receiver: Some(method.receiver),
            inputs: method.contract.inputs.clone().unwrap_or_default(),
            returns: method.contract.returns.clone(),
            body_rust: method.rust_body.clone(),
        })
        .collect::<Vec<_>>();

    Ok(RustDataSeamLowering {
        id: unit.id.clone(),
        module_path: unit.module_path.clone(),
        struct_name: unit.type_name.clone(),
        fields: unit
            .fields
            .iter()
            .map(|field| crate::types::RustDataFieldLowering {
                name: field.name.clone(),
                type_: field.type_.clone(),
            })
            .collect(),
        constructors,
        methods,
        local_tests: unit.local_tests.clone(),
        deps: unit.deps.clone(),
        derives: unit.rust_backend.derives.clone(),
    })
}

pub fn lower_sum_seam(unit: &NormalizedSumSeam) -> Result<RustSumSeamLowering> {
    let methods = unit
        .methods
        .iter()
        .map(|method| RustInherentMethodLowering {
            id: method.id.clone(),
            is_constructor: false,
            receiver: Some(method.receiver),
            inputs: method.contract.inputs.clone().unwrap_or_default(),
            returns: method.contract.returns.clone(),
            body_rust: method.rust_body.clone(),
        })
        .collect::<Vec<_>>();

    Ok(RustSumSeamLowering {
        id: unit.id.clone(),
        module_path: unit.module_path.clone(),
        enum_name: unit.enum_name.clone(),
        variants: unit
            .variants
            .iter()
            .map(|variant| crate::types::RustSumVariantLowering {
                id: variant.id.clone(),
                variant_name: variant.variant_name.clone(),
                fields: variant
                    .fields
                    .iter()
                    .map(|field| crate::types::RustSumVariantFieldLowering {
                        name: field.name.clone(),
                        type_: field.type_.clone(),
                    })
                    .collect(),
            })
            .collect(),
        methods,
        local_tests: unit.local_tests.clone(),
        deps: unit.deps.clone(),
        derives: unit.rust_backend.derives.clone(),
    })
}

pub fn generate_data_seam_code(unit: &NormalizedDataSeam) -> Result<String> {
    generate_data_seam_code_with_options(unit, &GenerateOptions::default())
}

pub fn generate_sum_seam_code(unit: &NormalizedSumSeam) -> Result<String> {
    generate_sum_seam_code_with_options(unit, &GenerateOptions::default())
}

pub fn generate_data_seam_code_with_options(
    unit: &NormalizedDataSeam,
    options: &GenerateOptions,
) -> Result<String> {
    let lowering = lower_data_seam(unit)?;
    let dep_statements = build_dep_statements(&lowering.deps, &unit.id)?;
    validate_data_inherent_callables(unit, &lowering)?;
    validate_rust_derives(&format!("data seam '{}'", unit.id), &lowering.derives)?;
    let mut output = String::new();

    for statement in dep_statements {
        output.push_str(&statement);
        output.push('\n');
    }

    if !lowering.deps.is_empty() {
        output.push('\n');
    }

    if let Some(doc_comment) = build_doc_comment(&unit.intent_why) {
        output.push_str(&doc_comment);
    }

    if !lowering.derives.is_empty() {
        output.push_str(&format!("#[derive({})]\n", lowering.derives.join(", ")));
    }

    output.push_str(&render_data_struct(&lowering));
    output.push('\n');
    output.push('\n');
    output.push_str(&render_data_impl(unit, &lowering));
    output.push('\n');

    if let Some(tests) = render_local_tests(&unit.id, &lowering.local_tests, options)? {
        output.push('\n');
        output.push_str(&tests);
    }

    Ok(output)
}

pub fn generate_sum_seam_code_with_options(
    unit: &NormalizedSumSeam,
    options: &GenerateOptions,
) -> Result<String> {
    let lowering = lower_sum_seam(unit)?;
    let dep_statements = build_dep_statements(&lowering.deps, &unit.id)?;
    validate_sum_inherent_callables(unit, &lowering)?;
    validate_sum_variant_names(unit, &lowering)?;
    validate_rust_derives(&format!("sum seam '{}'", unit.id), &lowering.derives)?;
    let mut output = String::new();

    for statement in dep_statements {
        output.push_str(&statement);
        output.push('\n');
    }

    if !lowering.deps.is_empty() {
        output.push('\n');
    }

    if let Some(doc_comment) = build_doc_comment(&unit.intent_why) {
        output.push_str(&doc_comment);
    }

    if !lowering.derives.is_empty() {
        output.push_str(&format!("#[derive({})]\n", lowering.derives.join(", ")));
    }

    output.push_str(&render_sum_enum(&lowering));
    output.push('\n');
    output.push('\n');
    output.push_str(&render_sum_impl(unit, &lowering));
    output.push('\n');

    if let Some(tests) = render_local_tests(&unit.id, &lowering.local_tests, options)? {
        output.push('\n');
        output.push_str(&tests);
    }

    Ok(output)
}

fn validate_data_inherent_callables(
    unit: &NormalizedDataSeam,
    lowering: &RustDataSeamLowering,
) -> Result<()> {
    let callable_ids = lowering
        .constructors
        .iter()
        .map(|constructor| constructor.id.clone())
        .chain(lowering.methods.iter().map(|method| method.id.clone()))
        .collect::<Vec<_>>();

    if let Some((first, second, callable_name)) = has_callable_collision(&callable_ids) {
        return Err(SpecError::Generator {
            message: format!(
                "data seam '{}' contains duplicate inherent callable '{}': '{}' and '{}'",
                unit.id, callable_name, first, second
            ),
        });
    }

    Ok(())
}

fn validate_sum_inherent_callables(
    unit: &NormalizedSumSeam,
    lowering: &RustSumSeamLowering,
) -> Result<()> {
    let callable_ids = lowering
        .methods
        .iter()
        .map(|method| method.id.clone())
        .collect::<Vec<_>>();

    if let Some((first, second, callable_name)) = has_callable_collision(&callable_ids) {
        return Err(SpecError::Generator {
            message: format!(
                "sum seam '{}' contains duplicate inherent callable '{}': '{}' and '{}'",
                unit.id, callable_name, first, second
            ),
        });
    }

    Ok(())
}

fn validate_sum_variant_names(
    unit: &NormalizedSumSeam,
    lowering: &RustSumSeamLowering,
) -> Result<()> {
    let mut seen = HashMap::new();
    for (index, variant) in lowering.variants.iter().enumerate() {
        if variant.variant_name == lowering.enum_name {
            return Err(SpecError::Generator {
                message: format!(
                    "sum seam '{}' projects variant '{}' to '{}', which conflicts with enum name '{}'",
                    unit.id, variant.id, variant.variant_name, lowering.enum_name
                ),
            });
        }
        if let Some((first_id, first_index)) =
            seen.insert(variant.variant_name.clone(), (variant.id.clone(), index))
        {
            return Err(SpecError::Generator {
                message: format!(
                    "sum seam '{}' projects variants '{}' and '{}' to duplicate Rust variant name '{}' (indices {} and {})",
                    unit.id, first_id, variant.id, variant.variant_name, first_index, index
                ),
            });
        }
    }

    Ok(())
}

fn validate_rust_derives(owner: &str, derives: &[String]) -> Result<()> {
    for (index, derive) in derives.iter().enumerate() {
        syn::parse_str::<syn::Path>(derive).map_err(|err| SpecError::Generator {
            message: format!(
                "{owner} has invalid backends.rust.derives[{index}] '{derive}': {err}",
            ),
        })?;
    }

    Ok(())
}

pub fn generate_normalized_unit_code(unit: &NormalizedUnit) -> Result<String> {
    generate_normalized_unit_code_with_options_for_target(
        unit,
        &GenerateOptions::default(),
        TargetLanguage::Rust,
    )
}

pub fn generate_unit_code(unit: &NormalizedUnit) -> Result<String> {
    generate_normalized_unit_code(unit)
}

pub fn generate_normalized_unit_code_with_options(
    unit: &NormalizedUnit,
    options: &GenerateOptions,
) -> Result<String> {
    generate_normalized_unit_code_with_options_for_target(unit, options, TargetLanguage::Rust)
}

pub fn generate_unit_code_with_options(
    unit: &NormalizedUnit,
    options: &GenerateOptions,
) -> Result<String> {
    generate_normalized_unit_code_with_options(unit, options)
}

pub fn generate_normalized_unit_code_for_target(
    unit: &NormalizedUnit,
    target_language: TargetLanguage,
) -> Result<String> {
    generate_normalized_unit_code_with_options_for_target(
        unit,
        &GenerateOptions::default(),
        target_language,
    )
}

pub fn generate_normalized_unit_code_with_options_for_target(
    unit: &NormalizedUnit,
    options: &GenerateOptions,
    target_language: TargetLanguage,
) -> Result<String> {
    match target_language {
        TargetLanguage::Rust => match unit {
            NormalizedUnit::Function(spec) => generate_code_with_options(spec, options),
            NormalizedUnit::Data(unit) => generate_data_seam_code_with_options(unit, options),
            NormalizedUnit::Sum(unit) => generate_sum_seam_code_with_options(unit, options),
        },
        TargetLanguage::TypeScript => match unit {
            NormalizedUnit::Function(spec) => {
                crate::typescript_backend::generate_typescript_unit_module(spec)
            }
            _ => Err(SpecError::Generator {
                message: format!(
                    "unit '{}' is not eligible for the bounded M45 TypeScript lane: only kind:function units are supported",
                    unit.id()
                ),
            }),
        },
    }
}

pub fn generate_unit_code_for_target(
    unit: &NormalizedUnit,
    target_language: TargetLanguage,
) -> Result<String> {
    generate_normalized_unit_code_for_target(unit, target_language)
}

pub fn generate_typescript_output_tree(
    units: &[NormalizedUnit],
) -> Result<BTreeMap<PathBuf, String>> {
    crate::typescript_backend::generate_typescript_tree(units)
}

fn render_constructor_body(
    unit: &NormalizedDataSeam,
    initializes: &IndexMap<String, String>,
) -> String {
    if unit.fields.is_empty() {
        return "{\n        Self\n    }".to_string();
    }

    let assignments = unit
        .fields
        .iter()
        .map(|field| {
            let expr = initializes
                .get(&field.name)
                .map(String::as_str)
                .unwrap_or(field.name.as_str());
            format!("            {}: {},", field.name, expr)
        })
        .collect::<Vec<_>>()
        .join("\n");

    format!("{{\n        Self {{\n{assignments}\n        }}\n    }}")
}

fn render_data_struct(lowering: &RustDataSeamLowering) -> String {
    if lowering.fields.is_empty() {
        return format!("pub struct {};", lowering.struct_name);
    }

    let fields = lowering
        .fields
        .iter()
        .map(|field| format!("    pub {}: {},", field.name, field.type_))
        .collect::<Vec<_>>()
        .join("\n");

    format!("pub struct {} {{\n{}\n}}", lowering.struct_name, fields)
}

fn render_sum_enum(lowering: &RustSumSeamLowering) -> String {
    let variants = lowering
        .variants
        .iter()
        .map(|variant| {
            if variant.fields.is_empty() {
                format!("    {},", variant.variant_name)
            } else {
                let fields = variant
                    .fields
                    .iter()
                    .map(|field| format!("        {}: {},", field.name, field.type_))
                    .collect::<Vec<_>>()
                    .join("\n");
                format!("    {} {{\n{}\n    }},", variant.variant_name, fields)
            }
        })
        .collect::<Vec<_>>()
        .join("\n");

    format!("pub enum {} {{\n{}\n}}", lowering.enum_name, variants)
}

fn receiver_param(receiver: MethodReceiver) -> &'static str {
    match receiver {
        MethodReceiver::SharedRef => "&self",
    }
}

fn render_inherent_method_signature(method: &RustInherentMethodLowering) -> String {
    let mut params = Vec::new();
    if let Some(receiver) = method.receiver {
        params.push(receiver_param(receiver).to_string());
    }

    let named_inputs = build_named_inputs(&method.inputs);
    if !named_inputs.is_empty() {
        params.push(named_inputs);
    }

    let return_suffix = method
        .returns
        .as_ref()
        .map(|returns| format!(" -> {returns}"))
        .unwrap_or_default();

    format!(
        "pub fn {}({}){}",
        method.id,
        params.join(", "),
        return_suffix
    )
}

fn render_data_impl(unit: &NormalizedDataSeam, lowering: &RustDataSeamLowering) -> String {
    let mut items = Vec::new();

    for (source, lowered) in unit.constructors.iter().zip(&lowering.constructors) {
        let mut item = String::new();
        if let Some(doc_comment) = build_doc_comment(&source.intent_why) {
            item.push_str(&indent_block(&doc_comment, 4));
        }
        item.push_str("    ");
        item.push_str(&render_inherent_method_signature(lowered));
        item.push(' ');
        item.push_str(lowered.body_rust.trim());
        item.push('\n');
        items.push(item);
    }

    for (source, lowered) in unit.methods.iter().zip(&lowering.methods) {
        let mut item = String::new();
        if let Some(doc_comment) = build_doc_comment(&source.intent_why) {
            item.push_str(&indent_block(&doc_comment, 4));
        }
        item.push_str("    ");
        item.push_str(&render_inherent_method_signature(lowered));
        item.push(' ');
        item.push_str(lowered.body_rust.trim());
        item.push('\n');
        items.push(item);
    }

    if items.is_empty() {
        format!("impl {} {{\n}}", lowering.struct_name)
    } else {
        format!("impl {} {{\n{}\n}}", lowering.struct_name, items.join("\n"))
    }
}

fn render_sum_impl(unit: &NormalizedSumSeam, lowering: &RustSumSeamLowering) -> String {
    let mut items = Vec::new();

    for (source, lowered) in unit.methods.iter().zip(&lowering.methods) {
        let mut item = String::new();
        if let Some(doc_comment) = build_doc_comment(&source.intent_why) {
            item.push_str(&indent_block(&doc_comment, 4));
        }
        item.push_str("    ");
        item.push_str(&render_inherent_method_signature(lowered));
        item.push(' ');
        item.push_str(lowered.body_rust.trim());
        item.push('\n');
        items.push(item);
    }

    if items.is_empty() {
        format!("impl {} {{\n}}", lowering.enum_name)
    } else {
        format!("impl {} {{\n{}\n}}", lowering.enum_name, items.join("\n"))
    }
}

fn indent_block(block: &str, spaces: usize) -> String {
    let padding = " ".repeat(spaces);
    let mut output = String::new();
    for line in block.lines() {
        output.push_str(&padding);
        output.push_str(line);
        output.push('\n');
    }
    output
}

pub fn write_generated_file(output_path: &str, content: &str) -> Result<()> {
    let path = Path::new(output_path);

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|err| SpecError::OutputDir {
            message: format!(
                "Unable to create output directory {}: {}",
                parent.display(),
                err
            ),
        })?;
    } else {
        return Err(SpecError::Generator {
            message: format!(
                "Unable to write {}: missing parent directory",
                path.display()
            ),
        });
    }

    let parent_dir = path
        .parent()
        .ok_or_else(|| SpecError::Generator {
            message: format!(
                "Unable to write {}: missing parent directory",
                path.display()
            ),
        })?
        .to_path_buf();

    // Write to a temp file in the same directory and rename into place (per-file atomic).
    let mut tmp = tempfile::Builder::new()
        .prefix(".spec-tmp-")
        .suffix(".tmp")
        .tempfile_in(&parent_dir)
        .map_err(|err| SpecError::Generator {
            message: format!(
                "Unable to create temp file in {}: {}",
                parent_dir.display(),
                err
            ),
        })?;

    tmp.write_all(content.as_bytes())
        .map_err(|err| SpecError::Generator {
            message: format!("Unable to write temp file for {}: {}", path.display(), err),
        })?;

    if !content.ends_with('\n') {
        tmp.write_all(b"\n").map_err(|err| SpecError::Generator {
            message: format!(
                "Unable to finalize temp file for {}: {}",
                path.display(),
                err
            ),
        })?;
    }

    tmp.flush().map_err(|err| SpecError::Generator {
        message: format!("Unable to flush temp file for {}: {}", path.display(), err),
    })?;

    // On Windows, renaming over an existing file fails; remove it first.
    if cfg!(windows) && path.exists() {
        fs::remove_file(path).map_err(|err| SpecError::Generator {
            message: format!("Unable to remove existing {}: {}", path.display(), err),
        })?;
    }

    let tmp_path = tmp.into_temp_path();
    fs::rename(&tmp_path, path).map_err(|err| SpecError::Generator {
        message: format!(
            "Unable to rename temp file into {}: {}",
            path.display(),
            err
        ),
    })?;

    Ok(())
}

pub fn clean_output_dir(
    output_base: &Path,
    generated_rs_rel_paths: &HashSet<PathBuf>,
    project_root: &Path,
) -> Result<()> {
    let base = safe_output_path_with_project_root(output_base, project_root)?;

    let marker = base.join(GENERATED_MARKER);
    if !marker.exists() {
        return Err(SpecError::MissingMarker {
            path: base.display().to_string(),
        });
    }

    // Remove orphaned `.rs` files (anything not in the generated set).
    for entry in WalkDir::new(&base).follow_links(false) {
        let entry = entry.map_err(SpecError::from)?;
        if !entry.file_type().is_file() {
            continue;
        }
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("rs") {
            continue;
        }

        let rel = path
            .strip_prefix(&base)
            .map_err(|err| SpecError::Generator {
                message: format!(
                    "Unable to compute relative path for {}: {}",
                    path.display(),
                    err
                ),
            })?;

        if !generated_rs_rel_paths.contains(rel) {
            fs::remove_file(path).map_err(|err| SpecError::Generator {
                message: format!("Unable to remove {}: {}", path.display(), err),
            })?;
        }
    }

    // Remove empty directories bottom-up (but never remove the base itself).
    for entry in WalkDir::new(&base).follow_links(false).contents_first(true) {
        let entry = entry.map_err(SpecError::from)?;
        if !entry.file_type().is_dir() || entry.file_type().is_symlink() {
            continue;
        }
        let path = entry.path();
        if path == base {
            continue;
        }

        let mut entries = fs::read_dir(path).map_err(|err| SpecError::Generator {
            message: format!("Unable to read dir {}: {}", path.display(), err),
        })?;
        if entries.next().is_none() {
            fs::remove_dir(path).map_err(|err| SpecError::Generator {
                message: format!("Unable to remove dir {}: {}", path.display(), err),
            })?;
        }
    }

    File::create(&marker).map_err(|err| SpecError::Generator {
        message: format!("Unable to recreate marker {}: {}", marker.display(), err),
    })?;

    Ok(())
}

pub fn generate_mod_rs(
    unit_files: &[String],
    subdirs: &[String],
    has_molecule_tests: bool,
) -> Result<String> {
    let mut seen = HashSet::new();
    let mut unit_mods = Vec::new();
    let mut subdir_mods = Vec::new();

    for unit_file in unit_files {
        if let Some(name) = module_item_name(unit_file) {
            let decl = format!("pub mod {};", name);
            if seen.insert(decl.clone()) {
                unit_mods.push(decl);
            }
        }
    }

    for subdir in subdirs {
        if let Some(name) = module_item_name(subdir) {
            let decl = format!("pub mod {};", name);
            if seen.insert(decl.clone()) {
                subdir_mods.push(decl);
            }
        }
    }

    unit_mods.sort();
    subdir_mods.sort();

    let mut output = String::new();
    for line in &unit_mods {
        output.push_str(line);
        output.push('\n');
    }

    if !unit_mods.is_empty() && !subdir_mods.is_empty() {
        output.push('\n');
    }

    for line in &subdir_mods {
        output.push_str(line);
        output.push('\n');
    }

    if has_molecule_tests {
        if !unit_mods.is_empty() || !subdir_mods.is_empty() {
            output.push('\n');
        }
        output.push_str("#[cfg(test)]\npub mod molecule_tests;\n");
    }

    Ok(output)
}

/// Returns the path for a molecule_tests.rs file relative to the output base.
pub fn molecule_tests_file_path(output_base: &Path, module_path: &str) -> PathBuf {
    if module_path.is_empty() {
        output_base.join("molecule_tests.rs")
    } else {
        output_base
            .join(module_path.replace('/', std::path::MAIN_SEPARATOR_STR))
            .join("molecule_tests.rs")
    }
}

/// Generate the Rust source code for a group of molecule tests in one namespace.
///
/// `tests` must all belong to the same `module_path`.
/// `units_by_id` is used to look up imports for covered units.
pub fn generate_molecule_tests_code(
    tests: &[&ResolvedMoleculeTest],
    units_by_id: &HashMap<&str, &NormalizedUnit>,
) -> Result<String> {
    fn render_use_import(import: &str) -> String {
        format!("use {};", import.trim_end_matches(';'))
    }

    let mut import_seen: HashSet<String> = HashSet::new();
    let mut import_lines: Vec<String> = Vec::new();

    for test in tests {
        if let Some(imports) = &test.imports {
            for import in imports {
                let line = render_use_import(import);
                if import_seen.insert(line.clone()) {
                    import_lines.push(line);
                }
            }
        } else {
            for cover_id in &test.covers {
                let unit = units_by_id
                    .get(cover_id.as_str())
                    .ok_or_else(|| SpecError::Generator {
                        message: format!(
                            "covered unit '{}' not found in spec set (should have been caught by validation)",
                            cover_id
                        ),
                    })?;
                for import in project_unit(ProjectedUnitRef::Normalized(unit)).cover_imports() {
                    let line = render_use_import(import);
                    if import_seen.insert(line.clone()) {
                        import_lines.push(line);
                    }
                }
            }
        }
    }

    let mut output = String::new();

    for line in &import_lines {
        output.push_str(line);
        output.push('\n');
    }

    if !import_lines.is_empty() {
        output.push('\n');
    }

    // Emit test functions
    for (index, test) in tests.iter().enumerate() {
        let block = test.body_rust.trim();
        output.push_str("#[test]\n");
        output.push_str(&format!("fn test_{}() {}\n", test.fn_name, block));

        if index + 1 != tests.len() {
            output.push('\n');
        }
    }

    Ok(output)
}

/// Generate and write molecule_tests.rs files for all resolved molecule tests.
///
/// Groups tests by module_path and generates one molecule_tests.rs per group.
///
/// Returns the set of relative paths for generated molecule_tests.rs files
/// (for inclusion in `generated_rs_rel_paths` passed to `clean_output_dir`).
pub fn generate_and_write_molecule_tests(
    resolved_tests: &[ResolvedMoleculeTest],
    units_by_id: &HashMap<&str, &NormalizedUnit>,
    output_base: &Path,
) -> Result<HashSet<PathBuf>> {
    // Group tests by module_path (BTreeMap for deterministic iteration order)
    let mut by_module: BTreeMap<String, Vec<&ResolvedMoleculeTest>> = BTreeMap::new();
    for test in resolved_tests {
        by_module
            .entry(test.module_path.clone())
            .or_default()
            .push(test);
    }

    let mut generated_paths = HashSet::new();

    for (module_path, tests) in &by_module {
        // Generate molecule_tests.rs content
        let content = generate_molecule_tests_code(tests, units_by_id)?;
        let file_path = molecule_tests_file_path(output_base, module_path);
        write_generated_file(&file_path.to_string_lossy(), &content)?;

        // Track the relative path for clean_output_dir
        let rel: PathBuf = if module_path.is_empty() {
            PathBuf::from("molecule_tests.rs")
        } else {
            PathBuf::from(module_path.replace('/', std::path::MAIN_SEPARATOR_STR))
                .join("molecule_tests.rs")
        };
        generated_paths.insert(rel);
    }

    Ok(generated_paths)
}

fn build_use_groups(spec: &ResolvedSpec) -> Result<(Vec<String>, Vec<String>)> {
    if let Some((dep1, dep2)) = ResolvedSpec::has_dep_collision(&spec.deps) {
        return Err(SpecError::DepCollision {
            dep1: dep1.clone(),
            dep2: dep2.clone(),
            fn_name: ResolvedSpec::dep_fn_name(dep1).to_string(),
            path: spec.id.clone(),
        });
    }

    if let Some(dep) = spec.deps.iter().find(|dep| {
        DepRef::parse(dep)
            .map(|parsed| parsed.callable_name() == spec.fn_name)
            .unwrap_or(false)
    }) {
        return Err(SpecError::DepCollision {
            dep1: dep.clone(),
            dep2: spec.id.clone(),
            fn_name: spec.fn_name.clone(),
            path: spec.id.clone(),
        });
    }

    let mut import_seen = HashSet::new();
    let mut import_statements = Vec::new();

    for import_path in &spec.imports {
        if import_seen.insert(import_path.clone()) {
            import_statements.push(format!("use {};", import_path));
        }
    }

    let mut dep_seen = HashSet::new();
    let mut dep_statements = Vec::new();

    for dep in &spec.deps {
        if dep_seen.insert(dep.clone()) {
            dep_statements.push(build_dep_use_statement(dep)?);
        }
    }

    Ok((import_statements, dep_statements))
}

fn build_dep_statements(deps: &[String], path: &str) -> Result<Vec<String>> {
    if let Some((dep1, dep2)) = ResolvedSpec::has_dep_collision(deps) {
        return Err(SpecError::DepCollision {
            dep1: dep1.clone(),
            dep2: dep2.clone(),
            fn_name: ResolvedSpec::dep_fn_name(dep1).to_string(),
            path: path.to_string(),
        });
    }

    let mut dep_seen = HashSet::new();
    let mut dep_statements = Vec::new();

    for dep in deps {
        if dep_seen.insert(dep.clone()) {
            dep_statements.push(build_dep_use_statement(dep)?);
        }
    }

    Ok(dep_statements)
}

fn build_dep_use_statement(dep: &str) -> Result<String> {
    let dep_ref = DepRef::parse(dep).map_err(|err| SpecError::Generator {
        message: format!(
            "invalid dep '{}' reached generator after validation: {}",
            dep, err
        ),
    })?;

    let unit_path = dep_ref.unit_id().replace('/', "::");
    let callable_name = dep_ref.callable_name();
    let prefix = dep_ref.library_alias().unwrap_or("crate");
    Ok(format!("use {prefix}::{unit_path}::{callable_name};"))
}

fn module_item_name(fragment: &str) -> Option<String> {
    Path::new(fragment)
        .file_name()
        .and_then(|name| name.to_str())
        .map(|name| name.trim_end_matches(".rs").to_string())
        .filter(|name| !name.is_empty())
}

pub fn safe_output_path<P: AsRef<Path>>(path: P) -> Result<PathBuf> {
    let project_root = std::env::current_dir().map_err(|err| SpecError::OutputDir {
        message: format!("Unable to determine project root: {err}"),
    })?;
    safe_output_path_with_project_root(path, project_root)
}

pub fn safe_output_path_with_project_root<P: AsRef<Path>, R: AsRef<Path>>(
    path: P,
    project_root: R,
) -> Result<PathBuf> {
    let path = path.as_ref();
    let project_root =
        canonicalize_existing_path(&normalized_absolute_path(project_root.as_ref()))?;
    let output_base = canonicalize_output_path(path)?;

    if !output_base.starts_with(&project_root) {
        return Err(SpecError::OutputDir {
            message: format!(
                "Refusing to generate into {}: output path is outside the project root {}",
                output_base.display(),
                project_root.display()
            ),
        });
    }

    Ok(output_base)
}

pub fn normalized_absolute_path<P: AsRef<Path>>(path: P) -> PathBuf {
    let path = path.as_ref();
    let mut normalized = if path.is_absolute() {
        PathBuf::new()
    } else {
        std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
    };

    for component in path.components() {
        match component {
            Component::Prefix(prefix) => normalized.push(prefix.as_os_str()),
            Component::RootDir => normalized.push(component.as_os_str()),
            Component::CurDir => {}
            Component::ParentDir => {
                normalized.pop();
            }
            Component::Normal(segment) => normalized.push(segment),
        }
    }

    normalized
}

fn canonicalize_output_path(path: &Path) -> Result<PathBuf> {
    let absolute = normalized_absolute_path(path);
    if absolute.exists() {
        return canonicalize_existing_path(&absolute);
    }

    let mut current = absolute.as_path();
    let mut missing_segments = Vec::new();

    while !current.exists() {
        let segment = current.file_name().ok_or_else(|| SpecError::OutputDir {
            message: format!(
                "Unable to resolve output path {}: no existing ancestor found",
                absolute.display()
            ),
        })?;
        missing_segments.push(segment.to_os_string());
        current = current.parent().ok_or_else(|| SpecError::OutputDir {
            message: format!(
                "Unable to resolve output path {}: no existing ancestor found",
                absolute.display()
            ),
        })?;
    }

    let mut resolved = canonicalize_existing_path(current)?;
    for segment in missing_segments.iter().rev() {
        resolved.push(segment);
    }

    Ok(resolved)
}

fn canonicalize_existing_path(path: &Path) -> Result<PathBuf> {
    path.canonicalize().map_err(|err| SpecError::OutputDir {
        message: format!("Unable to canonicalize {}: {}", path.display(), err),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syntax::{ExpectExprErrorKind, validate_expect_expr};
    use crate::types::{
        AuthoredConstructor, AuthoredDataShape, AuthoredField, AuthoredMethod,
        AuthoredMethodLowering, AuthoredRustMethodLowering, Body, Contract, Intent, LocalTest,
        MethodReceiver, NormalizedConstructor, NormalizedDataField, NormalizedDataSeam,
        NormalizedMethod, NormalizedSumSeam, NormalizedSumVariant, NormalizedSumVariantField,
        NormalizedUnit, ResolvedMoleculeTest, ResolvedSpec, RustDataSeamBackend,
        RustSumSeamBackend, SpecStruct, TargetLanguage, UnitExtensions,
    };
    use indexmap::IndexMap;
    #[cfg(unix)]
    use std::os::unix::fs as unix_fs;
    use tempfile::TempDir;

    fn make_resolved_molecule_test(
        id: &str,
        covers: Vec<&str>,
        body_rust: &str,
    ) -> ResolvedMoleculeTest {
        let (module_path, fn_name) = id
            .rsplit_once('/')
            .map(|(m, f)| (m.to_string(), f.to_string()))
            .unwrap_or_else(|| (String::new(), id.to_string()));
        ResolvedMoleculeTest {
            id: id.to_string(),
            fn_name,
            module_path,
            intent_why: format!("Test {id}"),
            covers: covers.into_iter().map(str::to_string).collect(),
            imports: None,
            body_rust: body_rust.to_string(),
            spec_version: None,
        }
    }

    fn make_resolved_spec_with_imports(id: &str, imports: Vec<&str>) -> ResolvedSpec {
        ResolvedSpec::from_spec(SpecStruct {
            id: id.to_string(),
            kind: "function".to_string(),
            intent: Intent {
                why: format!("Why {id}"),
            },
            contract: None,
            deps: vec![],
            imports: imports.into_iter().map(str::to_string).collect(),
            body: Body {
                rust: "{ }".to_string(),
                typescript: None,
            },
            local_tests: vec![],
            links: None,
            spec_version: None,
            extensions: crate::types::UnitExtensions::default(),
        })
    }

    fn test_spec_with_intent(
        deps: Vec<&str>,
        imports: Vec<&str>,
        body: &str,
        intent_why: &str,
    ) -> ResolvedSpec {
        ResolvedSpec::from_spec(SpecStruct {
            id: "pricing/apply_discount".to_string(),
            kind: "function".to_string(),
            intent: Intent {
                why: intent_why.to_string(),
            },
            contract: None,
            deps: deps.into_iter().map(|dep| dep.to_string()).collect(),
            imports: imports
                .into_iter()
                .map(|import| import.to_string())
                .collect(),
            body: Body {
                rust: body.to_string(),
                typescript: None,
            },
            local_tests: vec![],
            links: None,
            spec_version: None,
            extensions: crate::types::UnitExtensions::default(),
        })
    }

    fn test_spec_with(deps: Vec<&str>, imports: Vec<&str>, body: &str) -> ResolvedSpec {
        test_spec_with_intent(deps, imports, body, " ")
    }

    fn test_spec(deps: Vec<&str>, body: &str) -> ResolvedSpec {
        test_spec_with(deps, vec![], body)
    }

    fn typescript_lane_spec(id: &str) -> ResolvedSpec {
        let fn_name = id.rsplit('/').next().unwrap_or(id);
        ResolvedSpec::from_spec(SpecStruct {
            id: id.to_string(),
            kind: "function".to_string(),
            intent: Intent {
                why: "Return the subtotal after applying the tax rate.".to_string(),
            },
            contract: Some(Contract {
                inputs: Some(IndexMap::from([
                    ("subtotal".to_string(), "rust_decimal::Decimal".to_string()),
                    ("rate".to_string(), "rust_decimal::Decimal".to_string()),
                ])),
                returns: Some("rust_decimal::Decimal".to_string()),
                invariants: vec!["output >= subtotal".to_string()],
            }),
            deps: vec![],
            imports: vec!["rust_decimal::Decimal".to_string()],
            body: Body {
                rust: "{ subtotal + subtotal * rate }".to_string(),
                typescript: Some("return subtotal.add(subtotal.mul(rate));".to_string()),
            },
            local_tests: vec![LocalTest {
                id: "taxes_subtotal".to_string(),
                expect: format!(
                    "{fn_name}(Decimal::new(1000, 2), Decimal::new(7, 2)) == Decimal::new(1070, 2)"
                ),
            }],
            links: None,
            spec_version: Some("0.3.0".to_string()),
            extensions: UnitExtensions::default(),
        })
    }

    fn test_data_seam() -> NormalizedDataSeam {
        NormalizedDataSeam {
            id: "pricing/pricing_quote".to_string(),
            intent_why: "Quote checkout totals.".to_string(),
            type_name: "PricingQuote".to_string(),
            module_path: "pricing".to_string(),
            fields: vec![
                NormalizedDataField {
                    name: "subtotal".to_string(),
                    type_: "rust_decimal::Decimal".to_string(),
                },
                NormalizedDataField {
                    name: "tax_rate".to_string(),
                    type_: "rust_decimal::Decimal".to_string(),
                },
            ],
            constructors: vec![NormalizedConstructor {
                id: "new".to_string(),
                intent_why: "Create a checkout quote.".to_string(),
                inputs: IndexMap::from([
                    ("subtotal".to_string(), "rust_decimal::Decimal".to_string()),
                    ("tax_rate".to_string(), "rust_decimal::Decimal".to_string()),
                ]),
                initializes: IndexMap::from([
                    ("subtotal".to_string(), "subtotal".to_string()),
                    ("tax_rate".to_string(), "tax_rate".to_string()),
                ]),
            }],
            methods: vec![NormalizedMethod {
                id: "total".to_string(),
                intent_why: "Compute the final total.".to_string(),
                receiver: MethodReceiver::SharedRef,
                contract: Contract {
                    inputs: None,
                    returns: Some("rust_decimal::Decimal".to_string()),
                    invariants: vec![],
                },
                deps: vec!["pricing/apply_tax".to_string()],
                rust_body: "{\n        apply_tax(self.subtotal, self.tax_rate)\n    }".to_string(),
            }],
            deps: vec!["pricing/apply_tax".to_string()],
            local_tests: vec![LocalTest {
                id: "happy_path".to_string(),
                expect: "CheckoutQuote::new(rust_decimal::Decimal::ONE, rust_decimal::Decimal::ONE).total() == apply_tax(rust_decimal::Decimal::ONE, rust_decimal::Decimal::ONE)".to_string(),
            }],
            links: None,
            spec_version: None,
            rust_backend: RustDataSeamBackend {
                derives: vec!["Clone".to_string(), "Debug".to_string()],
            },
        }
    }

    fn test_sum_seam() -> NormalizedSumSeam {
        NormalizedSumSeam {
            id: "pricing/checkout_status".to_string(),
            intent_why: "Track checkout status.".to_string(),
            enum_name: "CheckoutStatus".to_string(),
            module_path: "pricing".to_string(),
            variants: vec![
                NormalizedSumVariant {
                    id: "pending".to_string(),
                    variant_name: "Pending".to_string(),
                    fields: vec![],
                },
                NormalizedSumVariant {
                    id: "quoted_total".to_string(),
                    variant_name: "QuotedTotal".to_string(),
                    fields: vec![NormalizedSumVariantField {
                        name: "subtotal".to_string(),
                        type_: "rust_decimal::Decimal".to_string(),
                    }],
                },
            ],
            methods: vec![NormalizedMethod {
                id: "label".to_string(),
                intent_why: "Expose a stable label.".to_string(),
                receiver: MethodReceiver::SharedRef,
                contract: Contract {
                    inputs: None,
                    returns: Some("&'static str".to_string()),
                    invariants: vec![],
                },
                deps: vec![],
                rust_body: "{\n        match self {\n            Self::Pending => \"pending\",\n            Self::QuotedTotal { .. } => \"quoted_total\",\n        }\n    }".to_string(),
            }],
            deps: vec![],
            local_tests: vec![LocalTest {
                id: "happy_path".to_string(),
                expect: "CheckoutStatus::Pending.label() == \"pending\"".to_string(),
            }],
            links: None,
            spec_version: None,
            rust_backend: RustSumSeamBackend {
                derives: vec!["Clone".to_string(), "Debug".to_string()],
            },
        }
    }

    #[test]
    fn generate_code_includes_doc_comment_from_intent() {
        let spec = test_spec_with_intent(
            vec![],
            vec!["rust_decimal::Decimal"],
            "{\n    Decimal::ZERO\n}",
            "Apply a percentage discount.",
        );

        let code = generate_code(&spec).unwrap();
        assert_eq!(
            code,
            "use rust_decimal::Decimal;\n\n/// Apply a percentage discount.\npub fn apply_discount() {\n    Decimal::ZERO\n}\n"
        );
    }

    #[test]
    fn generate_code_multiline_intent_produces_multiline_doc_comment() {
        let spec = test_spec_with_intent(
            vec![],
            vec![],
            "{\n    Decimal::ZERO\n}",
            "\nFirst line.\n\nSecond line.\n",
        );

        let code = generate_code(&spec).unwrap();
        assert_eq!(
            code,
            "/// First line.\n///\n/// Second line.\npub fn apply_discount() {\n    Decimal::ZERO\n}\n"
        );
    }

    #[test]
    fn generate_code_omits_doc_comment_for_blank_intent() {
        let spec = test_spec_with_intent(vec![], vec![], "{\n    Decimal::ZERO\n}", "   \n  ");

        let code = generate_code(&spec).unwrap();
        assert_eq!(code, "pub fn apply_discount() {\n    Decimal::ZERO\n}\n");
    }

    #[test]
    fn generate_typescript_target_emits_authored_typescript_body() {
        let spec = typescript_lane_spec("pricing/apply_tax");

        let code = generate_unit_code_for_target(
            &NormalizedUnit::Function(spec),
            TargetLanguage::TypeScript,
        )
        .unwrap();

        assert!(code.contains("import { Decimal } from \"../__spec_ts/runtime.ts\";"));
        assert!(
            code.contains("export function apply_tax(subtotal: Decimal, rate: Decimal): Decimal")
        );
        assert!(code.contains("return subtotal.add(subtotal.mul(rate));"));
        assert!(!code.contains("subtotal + subtotal * rate"));
    }

    #[test]
    fn test_generate_code_prepends_use_statements() {
        let spec = test_spec(
            vec!["money/round", "utils/math/normalize"],
            "{\n    round(Decimal::ONE)\n}",
        );

        let code = generate_code(&spec).unwrap();
        assert_eq!(
            code,
            "use crate::money::round::round;\nuse crate::utils::math::normalize::normalize;\n\npub fn apply_discount() {\n    round(Decimal::ONE)\n}\n"
        );
    }

    #[test]
    fn test_generate_code_rejects_dep_collision() {
        let spec = test_spec(
            vec!["money/round", "utils/round"],
            "{\n    round(Decimal::ONE)\n}",
        );

        let err = generate_code(&spec).unwrap_err();
        assert!(err.to_string().contains("Dep fn_name collision"));
    }

    #[test]
    fn imports_field_generates_correct_use_statement() {
        let spec = test_spec_with(
            vec![],
            vec!["rust_decimal::Decimal"],
            "{\n    Decimal::ZERO\n}",
        );

        let code = generate_code(&spec).unwrap();
        assert_eq!(
            code,
            "use rust_decimal::Decimal;\n\npub fn apply_discount() {\n    Decimal::ZERO\n}\n"
        );
    }

    #[test]
    fn deps_unchanged_after_imports_split() {
        let spec = test_spec_with(
            vec!["money/round"],
            vec![],
            "{\n    round(Decimal::ZERO)\n}",
        );

        let code = generate_code(&spec).unwrap();
        assert!(code.contains("use crate::money::round::round;"));
    }

    #[test]
    fn imports_emitted_before_deps_in_use_statements() {
        let spec = test_spec_with(
            vec!["money/round"],
            vec!["rust_decimal::Decimal"],
            "{\n    round(Decimal::ZERO)\n}",
        );

        let code = generate_code(&spec).unwrap();
        assert_eq!(
            code,
            "use rust_decimal::Decimal;\n\nuse crate::money::round::round;\n\npub fn apply_discount() {\n    round(Decimal::ZERO)\n}\n"
        );
    }

    #[test]
    fn external_deps_generate_alias_use_statements() {
        let spec = test_spec_with(
            vec!["shared::money/round"],
            vec![],
            "{\n    round(Decimal::ZERO)\n}",
        );

        let code = generate_code(&spec).unwrap();
        assert_eq!(
            code,
            "use shared::money::round::round;\n\npub fn apply_discount() {\n    round(Decimal::ZERO)\n}\n"
        );
    }

    #[test]
    fn generate_code_rejects_local_external_dep_collision() {
        let spec = test_spec(
            vec!["money/round", "shared::math/round"],
            "{\n    round(Decimal::ONE)\n}",
        );

        let err = generate_code(&spec).unwrap_err().to_string();
        assert!(err.contains("Dep fn_name collision"), "{err}");
        assert!(err.contains("money/round"), "{err}");
        assert!(err.contains("shared::math/round"), "{err}");
    }

    #[test]
    fn generate_code_rejects_dep_collision_with_unit_callable_name() {
        let spec = ResolvedSpec::from_spec(SpecStruct {
            id: "money/round".to_string(),
            kind: "function".to_string(),
            intent: Intent {
                why: "Round money values.".to_string(),
            },
            contract: None,
            deps: vec!["shared::money/round".to_string()],
            imports: vec![],
            body: Body {
                rust: "{\n    round(value)\n}".to_string(),
                typescript: None,
            },
            local_tests: vec![],
            links: None,
            spec_version: None,
            extensions: crate::types::UnitExtensions::default(),
        });

        let err = generate_code(&spec).unwrap_err();
        match err {
            SpecError::DepCollision {
                dep1,
                dep2,
                fn_name,
                path,
            } => {
                assert_eq!(dep1, "shared::money/round");
                assert_eq!(dep2, "money/round");
                assert_eq!(fn_name, "round");
                assert_eq!(path, "money/round");
            }
            other => panic!("expected DepCollision, got {other:?}"),
        }
    }

    #[test]
    fn test_write_generated_file_creates_parent_dirs() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir
            .path()
            .join("generated/spec/pricing/apply_discount.rs");

        write_generated_file(
            file_path.to_str().unwrap(),
            "pub fn apply_discount() -> Decimal { Decimal::ZERO }\n",
        )
        .unwrap();

        let contents = fs::read_to_string(&file_path).unwrap();
        assert_eq!(
            contents,
            "pub fn apply_discount() -> Decimal { Decimal::ZERO }\n"
        );
    }

    #[test]
    fn test_generate_mod_rs_lists_units_and_subdirs() {
        let content = generate_mod_rs(
            &["apply_discount.rs".to_string(), "refund.rs".to_string()],
            &["taxes".to_string(), "discounts".to_string()],
            false,
        )
        .unwrap();

        assert_eq!(
            content,
            "pub mod apply_discount;\npub mod refund;\n\npub mod discounts;\npub mod taxes;\n"
        );
    }

    #[test]
    fn generate_local_tests_produces_cfg_test_block() {
        let mut spec = test_spec_with(
            vec![],
            vec!["rust_decimal::Decimal"],
            "{\n    Decimal::ZERO\n}",
        );
        spec.local_tests = vec![LocalTest {
            id: "happy_path".to_string(),
            expect: "apply_discount() == Decimal::ZERO".to_string(),
        }];

        let code = generate_code(&spec).unwrap();
        assert!(code.contains("#[cfg(test)]\nmod tests {"));
        assert!(code.contains("use super::*;"));
        assert!(code.contains("fn test_happy_path() {"));
        assert!(code.contains("assert!(apply_discount() == Decimal::ZERO);"));
    }

    #[test]
    fn generate_no_local_tests_produces_no_test_block() {
        let spec = test_spec_with(vec![], vec![], "{ }");
        let code = generate_code(&spec).unwrap();
        assert!(!code.contains("#[cfg(test)]"));
        assert!(!code.contains("mod tests {"));
    }

    #[test]
    fn generate_code_rejects_unsafe_expect_at_sink() {
        let mut spec = test_spec_with(vec![], vec![], "{ true }");
        spec.local_tests = vec![LocalTest {
            id: "unsafe_attempt".to_string(),
            expect: "{ let ok = apply_discount(); ok }".to_string(),
        }];

        let err = generate_code(&spec).unwrap_err().to_string();
        assert!(err.contains("pricing/apply_discount"), "{err}");
        assert!(err.contains("unsafe_attempt"), "{err}");
        assert!(err.contains("block, unsafe, closure"), "{err}");
    }

    #[test]
    fn generate_code_rejects_deeply_nested_expect_at_sink() {
        let mut spec = test_spec_with(vec![], vec![], "{ true }");
        spec.local_tests = vec![LocalTest {
            id: "deep".to_string(),
            expect: format!("{}true{}", "(".repeat(200), ")".repeat(200)),
        }];

        let err = generate_code(&spec).unwrap_err().to_string();
        assert!(err.contains("pricing/apply_discount"), "{err}");
        assert!(err.contains("deep"), "{err}");
        assert!(err.contains("maximum depth of 128"), "{err}");
    }

    #[test]
    fn generate_code_sink_guard_includes_unit_and_test_id_in_error() {
        let mut spec = test_spec_with(vec![], vec![], "{ true }");
        spec.local_tests = vec![LocalTest {
            id: "broken".to_string(),
            expect: "(".to_string(),
        }];

        let err = generate_code(&spec).unwrap_err().to_string();
        assert!(err.contains("pricing/apply_discount"), "{err}");
        assert!(err.contains("broken"), "{err}");
    }

    #[test]
    fn generate_code_with_options_preserves_escape_hatch() {
        let mut spec = test_spec_with(vec![], vec![], "{ true }");
        spec.local_tests = vec![LocalTest {
            id: "unsafe_allowed".to_string(),
            expect: "{ let ok = apply_discount(); ok }".to_string(),
        }];

        let code = generate_code_with_options(
            &spec,
            &GenerateOptions {
                allow_unsafe_local_test_expect: true,
            },
        )
        .unwrap();

        assert!(code.contains("assert!({ let ok = apply_discount(); ok });"));
    }

    #[test]
    fn lower_data_seam_preserves_constructor_and_method_semantics() {
        let seam = test_data_seam();

        let lowering = lower_data_seam(&seam).unwrap();

        assert_eq!(lowering.struct_name, "PricingQuote");
        assert_eq!(lowering.fields.len(), 2);
        assert_eq!(lowering.constructors.len(), 1);
        assert_eq!(lowering.constructors[0].id, "new");
        assert_eq!(lowering.constructors[0].receiver, None);
        assert_eq!(lowering.constructors[0].returns.as_deref(), Some("Self"));
        assert!(
            lowering.constructors[0].body_rust.contains("Self {"),
            "constructor should lower to a struct literal"
        );
        assert_eq!(lowering.methods.len(), 1);
        assert_eq!(lowering.methods[0].id, "total");
        assert_eq!(
            lowering.methods[0].receiver,
            Some(MethodReceiver::SharedRef)
        );
        assert_eq!(
            lowering.methods[0].returns.as_deref(),
            Some("rust_decimal::Decimal")
        );
    }

    #[test]
    fn generate_normalized_unit_code_emits_data_struct_impl_and_tests() {
        let code = generate_normalized_unit_code(&NormalizedUnit::Data(test_data_seam())).unwrap();

        assert!(code.contains("use crate::pricing::apply_tax::apply_tax;"));
        assert!(code.contains("/// Quote checkout totals."));
        assert!(code.contains("#[derive(Clone, Debug)]"));
        assert!(code.contains("pub struct PricingQuote {"));
        assert!(code.contains("pub subtotal: rust_decimal::Decimal,"));
        assert!(code.contains("pub tax_rate: rust_decimal::Decimal,"));
        assert!(code.contains("impl PricingQuote {"));
        assert!(code.contains(
            "pub fn new(subtotal: rust_decimal::Decimal, tax_rate: rust_decimal::Decimal) -> Self {"
        ));
        assert!(code.contains(
            "Self {\n            subtotal: subtotal,\n            tax_rate: tax_rate,\n        }"
        ));
        assert!(code.contains("pub fn total(&self) -> rust_decimal::Decimal {"));
        assert!(code.contains("apply_tax(self.subtotal, self.tax_rate)"));
        assert!(code.contains("#[cfg(test)]\nmod tests {"));
        assert!(code.contains("assert!(CheckoutQuote::new("));
    }

    #[test]
    fn generate_normalized_unit_code_emits_sum_enum_impl_and_tests() {
        let code = generate_normalized_unit_code(&NormalizedUnit::Sum(test_sum_seam())).unwrap();

        assert!(code.contains("/// Track checkout status."));
        assert!(code.contains("#[derive(Clone, Debug)]"));
        assert!(code.contains("pub enum CheckoutStatus {"));
        assert!(code.contains("Pending,"));
        assert!(code.contains("QuotedTotal {"));
        assert!(code.contains("subtotal: rust_decimal::Decimal,"));
        assert!(code.contains("impl CheckoutStatus {"));
        assert!(code.contains("pub fn label(&self) -> &'static str {"));
        assert!(code.contains("Self::Pending => \"pending\""));
        assert!(code.contains("Self::QuotedTotal { .. } => \"quoted_total\""));
        assert!(code.contains("#[cfg(test)]\nmod tests {"));
        assert!(code.contains("assert!(CheckoutStatus::Pending.label() == \"pending\");"));
    }

    #[test]
    fn generate_sum_seam_code_rejects_projected_variant_name_collision() {
        let mut seam = test_sum_seam();
        seam.variants.push(NormalizedSumVariant {
            id: "quoted__total".to_string(),
            variant_name: "QuotedTotal".to_string(),
            fields: vec![],
        });

        let err = generate_sum_seam_code(&seam).unwrap_err().to_string();
        assert!(
            err.contains("duplicate Rust variant name 'QuotedTotal'"),
            "{err}"
        );
        assert!(err.contains("quoted_total"), "{err}");
        assert!(err.contains("quoted__total"), "{err}");
    }

    #[test]
    fn generate_data_seam_code_rejects_dep_collision() {
        let mut seam = test_data_seam();
        seam.deps = vec![
            "pricing/apply_tax".to_string(),
            "shared::money/apply_tax".to_string(),
        ];

        let err = generate_data_seam_code(&seam).unwrap_err().to_string();
        assert!(err.contains("Dep fn_name collision"), "{err}");
        assert!(err.contains("pricing/apply_tax"), "{err}");
        assert!(err.contains("shared::money/apply_tax"), "{err}");
    }

    #[test]
    fn generate_data_seam_code_dedupes_identical_cross_method_deps_after_normalization() {
        let seam = NormalizedDataSeam::from_spec(SpecStruct {
            id: "pricing/pricing_quote".to_string(),
            kind: "data".to_string(),
            intent: Intent {
                why: "Quote checkout totals.".to_string(),
            },
            contract: None,
            deps: vec![],
            imports: vec![],
            body: Body::default(),
            local_tests: vec![],
            links: None,
            spec_version: None,
            extensions: UnitExtensions {
                data: Some(AuthoredDataShape {
                    fields: IndexMap::from([(
                        "subtotal".to_string(),
                        AuthoredField {
                            type_: "rust_decimal::Decimal".to_string(),
                        },
                    )]),
                }),
                constructors: vec![AuthoredConstructor {
                    id: "new".to_string(),
                    intent: Intent {
                        why: "Create a quote.".to_string(),
                    },
                    contract: Some(Contract {
                        inputs: Some(IndexMap::from([(
                            "subtotal".to_string(),
                            "rust_decimal::Decimal".to_string(),
                        )])),
                        returns: None,
                        invariants: vec![],
                    }),
                    initializes: IndexMap::from([("subtotal".to_string(), "subtotal".to_string())]),
                }],
                methods: vec![
                    AuthoredMethod {
                        id: "subtotal_with_tax".to_string(),
                        intent: Intent {
                            why: "Apply tax once.".to_string(),
                        },
                        receiver: "shared_ref".to_string(),
                        contract: Some(Contract {
                            inputs: None,
                            returns: Some("rust_decimal::Decimal".to_string()),
                            invariants: vec![],
                        }),
                        deps: vec!["pricing/apply_tax".to_string()],
                        lowering: Some(AuthoredMethodLowering {
                            rust: Some(AuthoredRustMethodLowering {
                                body: "{ apply_tax(self.subtotal, self.subtotal) }".to_string(),
                            }),
                        }),
                    },
                    AuthoredMethod {
                        id: "subtotal_with_tax_again".to_string(),
                        intent: Intent {
                            why: "Apply the same helper again.".to_string(),
                        },
                        receiver: "shared_ref".to_string(),
                        contract: Some(Contract {
                            inputs: None,
                            returns: Some("rust_decimal::Decimal".to_string()),
                            invariants: vec![],
                        }),
                        deps: vec!["pricing/apply_tax".to_string()],
                        lowering: Some(AuthoredMethodLowering {
                            rust: Some(AuthoredRustMethodLowering {
                                body: "{ apply_tax(self.subtotal, self.subtotal) }".to_string(),
                            }),
                        }),
                    },
                ],
                backends: None,
                sum: None,
            },
        })
        .unwrap();
        let code = generate_data_seam_code(&seam).unwrap();
        assert_eq!(
            code.matches("use crate::pricing::apply_tax::apply_tax;")
                .count(),
            1
        );
    }

    #[test]
    fn generate_data_seam_code_rejects_duplicate_inherent_callables() {
        let mut seam = test_data_seam();
        seam.methods[0].id = "new".to_string();

        let err = generate_data_seam_code(&seam).unwrap_err().to_string();
        assert!(err.contains("duplicate inherent callable 'new'"), "{err}");
        assert!(err.contains("pricing/pricing_quote"), "{err}");
    }

    #[test]
    fn generate_data_seam_code_rejects_invalid_derive_path() {
        let mut seam = test_data_seam();
        seam.rust_backend.derives = vec!["not valid rust".to_string()];

        let err = generate_data_seam_code(&seam).unwrap_err().to_string();
        assert!(
            err.contains("invalid backends.rust.derives[0] 'not valid rust'"),
            "{err}"
        );
        assert!(err.contains("pricing/pricing_quote"), "{err}");
    }

    #[test]
    fn generate_normalized_unit_code_dispatches_existing_function_path() {
        let spec = test_spec_with(
            vec!["money/round"],
            vec![],
            "{\n    round(Decimal::ZERO)\n}",
        );

        let direct = generate_code(&spec).unwrap();
        let dispatched = generate_normalized_unit_code(&NormalizedUnit::Function(spec)).unwrap();

        assert_eq!(dispatched, direct);
    }

    #[test]
    fn generate_typescript_output_tree_emits_unit_modules_and_frozen_helpers_once() {
        let specs = vec![
            NormalizedUnit::Function(typescript_lane_spec("pricing/apply_tax")),
            NormalizedUnit::Function(typescript_lane_spec("checkout/apply_vat")),
        ];

        let tree = generate_typescript_output_tree(&specs).unwrap();

        assert_eq!(tree.len(), 5);
        assert!(tree.contains_key(&PathBuf::from("pricing/apply_tax.ts")));
        assert!(tree.contains_key(&PathBuf::from("checkout/apply_vat.ts")));
        assert!(tree.contains_key(&PathBuf::from("__spec_ts/runtime.ts")));
        assert!(tree.contains_key(&PathBuf::from("__spec_ts/build_entry.ts")));
        assert!(tree.contains_key(&PathBuf::from("__spec_ts/local_tests.ts")));

        let build_entry = tree
            .get(&PathBuf::from("__spec_ts/build_entry.ts"))
            .unwrap();
        assert!(build_entry.contains("import \"./runtime.ts\";"));
        assert!(build_entry.contains(
            "import { apply_tax as __spec$pricing$apply_tax } from \"../pricing/apply_tax.ts\";"
        ));
        assert!(build_entry.contains(
            "import { apply_vat as __spec$checkout$apply_vat } from \"../checkout/apply_vat.ts\";"
        ));

        let local_tests = tree
            .get(&PathBuf::from("__spec_ts/local_tests.ts"))
            .unwrap();
        assert!(local_tests.contains("Decimal.new(1000n, 2n)"));
        assert!(local_tests.contains("__spec$pricing$apply_tax"));
        assert!(local_tests.contains("__spec$checkout$apply_vat"));
    }

    #[test]
    fn shared_expect_validation_reports_too_deep_before_syn_parse() {
        let result = validate_expect_expr(
            &format!("{}true{}", "(".repeat(200), ")".repeat(200)),
            false,
        );
        match result {
            Err(ExpectExprErrorKind::TooDeep { max_depth }) => assert_eq!(max_depth, 128),
            Err(other) => panic!("expected too-deep error, got {:?}", other),
            Ok(_) => panic!("expected too-deep error, got success"),
        }
    }

    #[test]
    fn clean_output_dir_removes_stale_module_from_prior_run() {
        let temp_dir = TempDir::new_in(std::env::current_dir().unwrap()).unwrap();
        let base = temp_dir.path().join("generated/spec");
        let pricing = base.join("pricing");
        let test_mod = base.join("test");
        fs::create_dir_all(&pricing).unwrap();
        fs::create_dir_all(&test_mod).unwrap();
        fs::write(base.join(GENERATED_MARKER), "").unwrap();

        // "Current run" generated files.
        fs::write(base.join("mod.rs"), "pub mod pricing;\n").unwrap();
        fs::write(pricing.join("apply_discount.rs"), "fn a() {}\n").unwrap();
        fs::write(pricing.join("mod.rs"), "pub mod apply_discount;\n").unwrap();

        // Stale module files from a prior run should be removed (and the empty dir pruned).
        fs::write(test_mod.join("foo.rs"), "fn stale() {}\n").unwrap();
        fs::write(test_mod.join("mod.rs"), "pub mod foo;\n").unwrap();

        let mut generated = HashSet::new();
        generated.insert(PathBuf::from("pricing/apply_discount.rs"));
        generated.insert(PathBuf::from("pricing/mod.rs"));
        generated.insert(PathBuf::from("mod.rs"));

        clean_output_dir(&base, &generated, temp_dir.path()).unwrap();

        assert!(pricing.join("apply_discount.rs").exists());
        assert!(pricing.join("mod.rs").exists());

        assert!(!test_mod.join("foo.rs").exists());
        assert!(!test_mod.join("mod.rs").exists());
        assert!(!test_mod.exists(), "stale module dir should be removed");

        assert!(base.join("mod.rs").exists());
        assert!(base.join(GENERATED_MARKER).exists());
    }

    #[test]
    fn test_clean_output_dir_refuses_without_marker() {
        let temp_dir = TempDir::new_in(std::env::current_dir().unwrap()).unwrap();
        let base = temp_dir.path().join("generated/spec");
        fs::create_dir_all(&base).unwrap();

        let generated = HashSet::new();
        let err = clean_output_dir(&base, &generated, temp_dir.path()).unwrap_err();
        assert!(matches!(err, SpecError::MissingMarker { .. }));
    }

    #[test]
    #[cfg(unix)]
    fn test_clean_output_dir_does_not_follow_symlink_dirs() {
        let temp_dir = TempDir::new_in(std::env::current_dir().unwrap()).unwrap();
        let base = temp_dir.path().join("generated/spec");
        let pricing = base.join("pricing");
        fs::create_dir_all(&pricing).unwrap();
        fs::write(base.join(GENERATED_MARKER), "").unwrap();
        fs::write(pricing.join("apply_discount.rs"), "fn a() {}\n").unwrap();

        let outside_dir = temp_dir.path().join("outside");
        fs::create_dir_all(&outside_dir).unwrap();
        let outside_rs = outside_dir.join("outside.rs");
        fs::write(&outside_rs, "fn outside() {}\n").unwrap();

        unix_fs::symlink(&outside_dir, pricing.join("link")).unwrap();

        let generated = HashSet::new();
        clean_output_dir(&base, &generated, temp_dir.path()).unwrap();

        assert!(!pricing.join("apply_discount.rs").exists());
        assert!(
            outside_rs.exists(),
            "clean_output_dir must not delete files through symlinks"
        );
        assert!(base.join(GENERATED_MARKER).exists());
    }

    #[test]
    fn safe_output_path_accepts_existing_path_inside_project_root() {
        let temp_dir = TempDir::new_in(std::env::current_dir().unwrap()).unwrap();
        let path = temp_dir.path().join("generated/spec");
        fs::create_dir_all(&path).unwrap();

        let resolved = safe_output_path(&path).unwrap();
        assert_eq!(resolved, path.canonicalize().unwrap());
    }

    #[test]
    fn safe_output_path_resolves_nonexistent_nested_path_from_existing_ancestor() {
        let temp_dir = TempDir::new_in(std::env::current_dir().unwrap()).unwrap();
        let base = temp_dir.path().join("generated");
        fs::create_dir_all(&base).unwrap();
        let path = base.join("spec/pricing");

        let resolved = safe_output_path(&path).unwrap();
        assert_eq!(
            resolved,
            base.canonicalize().unwrap().join("spec").join("pricing")
        );
    }

    #[test]
    #[cfg(unix)]
    fn safe_output_path_rejects_symlink_escape_in_nonexistent_path() {
        let temp_dir = TempDir::new_in(std::env::current_dir().unwrap()).unwrap();
        let outside = tempfile::TempDir::new().unwrap();
        let link = temp_dir.path().join("escape");
        unix_fs::symlink(outside.path(), &link).unwrap();

        let err = safe_output_path(link.join("generated/spec")).unwrap_err();
        assert!(err.to_string().contains("outside the project root"));
    }

    #[test]
    fn generate_molecule_tests_code_single_test_single_cover() {
        let test = make_resolved_molecule_test(
            "pricing/checkout",
            vec!["pricing/apply_tax"],
            "{ assert!(true); }",
        );
        let spec =
            make_resolved_spec_with_imports("pricing/apply_tax", vec!["rust_decimal::Decimal"]);
        let unit = NormalizedUnit::Function(spec);
        let units_by_id: HashMap<&str, &NormalizedUnit> =
            [("pricing/apply_tax", &unit)].into_iter().collect();

        let code = generate_molecule_tests_code(&[&test], &units_by_id).unwrap();

        assert!(
            code.contains("use rust_decimal::Decimal;"),
            "should emit spec imports"
        );
        assert!(
            code.contains("use crate::pricing::apply_tax::apply_tax;"),
            "should emit use crate path for covered unit"
        );
        assert!(code.contains("#[test]"), "should emit test attribute");
        assert!(
            code.contains("fn test_checkout()"),
            "should use fn_name with test_ prefix"
        );
        assert!(
            code.contains("{ assert!(true); }"),
            "should include body verbatim"
        );
    }

    #[test]
    fn generate_molecule_tests_code_deduplicates_shared_imports() {
        let test = make_resolved_molecule_test(
            "pricing/multi",
            vec!["pricing/apply_tax", "pricing/apply_discount"],
            "{ assert!(true); }",
        );
        let spec_tax =
            make_resolved_spec_with_imports("pricing/apply_tax", vec!["rust_decimal::Decimal"]);
        let spec_discount = make_resolved_spec_with_imports(
            "pricing/apply_discount",
            vec!["rust_decimal::Decimal"],
        );
        let unit_tax = NormalizedUnit::Function(spec_tax);
        let unit_discount = NormalizedUnit::Function(spec_discount);
        let units_by_id: HashMap<&str, &NormalizedUnit> = [
            ("pricing/apply_tax", &unit_tax),
            ("pricing/apply_discount", &unit_discount),
        ]
        .into_iter()
        .collect();

        let code = generate_molecule_tests_code(&[&test], &units_by_id).unwrap();

        let decimal_count = code.matches("use rust_decimal::Decimal;").count();
        assert_eq!(decimal_count, 1, "shared import should appear exactly once");
    }

    #[test]
    fn generate_molecule_tests_code_missing_cover_id_returns_error() {
        let test = make_resolved_molecule_test(
            "pricing/checkout",
            vec!["pricing/nonexistent"],
            "{ assert!(true); }",
        );
        let units_by_id: HashMap<&str, &NormalizedUnit> = HashMap::new();

        let err = generate_molecule_tests_code(&[&test], &units_by_id).unwrap_err();
        assert!(
            err.to_string().contains("pricing/nonexistent"),
            "error should name the missing cover id"
        );
    }

    #[test]
    fn generate_molecule_tests_code_multiple_tests_in_namespace() {
        let test_a = make_resolved_molecule_test(
            "pricing/flow_a",
            vec!["pricing/apply_tax"],
            "{ assert!(1 == 1); }",
        );
        let test_b = make_resolved_molecule_test(
            "pricing/flow_b",
            vec!["pricing/apply_tax"],
            "{ assert!(2 == 2); }",
        );
        let spec = make_resolved_spec_with_imports("pricing/apply_tax", vec![]);
        let unit = NormalizedUnit::Function(spec);
        let units_by_id: HashMap<&str, &NormalizedUnit> =
            [("pricing/apply_tax", &unit)].into_iter().collect();

        let code = generate_molecule_tests_code(&[&test_a, &test_b], &units_by_id).unwrap();

        assert!(
            code.contains("fn test_flow_a()"),
            "first test should be present"
        );
        assert!(
            code.contains("fn test_flow_b()"),
            "second test should be present"
        );
        // Use statement should appear only once despite two tests covering the same unit
        let use_count = code
            .matches("use crate::pricing::apply_tax::apply_tax;")
            .count();
        assert_eq!(use_count, 1, "use path should be deduplicated across tests");
    }

    #[test]
    fn generate_mod_rs_with_molecule_tests_appends_declaration() {
        let content = generate_mod_rs(&["apply_discount.rs".to_string()], &[], true).unwrap();

        assert!(
            content.contains("#[cfg(test)]\npub mod molecule_tests;"),
            "should declare molecule_tests module gated by cfg(test)"
        );
        assert!(
            content.contains("pub mod apply_discount;"),
            "should still include unit mods"
        );
    }

    #[test]
    fn generate_mod_rs_molecule_tests_only_namespace() {
        let content = generate_mod_rs(&[], &[], true).unwrap();
        assert_eq!(content, "#[cfg(test)]\npub mod molecule_tests;\n");
    }

    #[test]
    fn generate_molecule_tests_code_zero_covers_emits_test_fn_no_use_statements() {
        // A test with no covers should emit the #[test] function but no `use crate::` lines.
        let test =
            make_resolved_molecule_test("pricing/standalone_flow", vec![], "{ assert!(true); }");
        let units_by_id: HashMap<&str, &NormalizedUnit> = HashMap::new();

        let code = generate_molecule_tests_code(&[&test], &units_by_id).unwrap();

        assert!(code.contains("#[test]"), "should emit #[test] attribute");
        assert!(
            code.contains("fn test_standalone_flow()"),
            "should emit test function"
        );
        assert!(
            !code.contains("use crate::"),
            "no covers means no use crate:: imports"
        );
    }

    #[test]
    fn generate_molecule_tests_code_imports_data_seam_type() {
        let test = make_resolved_molecule_test(
            "pricing/data_checkout",
            vec!["pricing/pricing_quote"],
            "{ let _quote = PricingQuote::new(rust_decimal::Decimal::ONE, rust_decimal::Decimal::ONE); }",
        );
        let seam = test_data_seam();
        let unit = NormalizedUnit::Data(seam);
        let units_by_id: HashMap<&str, &NormalizedUnit> =
            [("pricing/pricing_quote", &unit)].into_iter().collect();

        let code = generate_molecule_tests_code(&[&test], &units_by_id).unwrap();

        assert!(code.contains("use crate::pricing::pricing_quote::PricingQuote;"));
    }

    #[test]
    fn generate_molecule_tests_code_explicit_imports_skip_cover_derived_imports() {
        let mut test = make_resolved_molecule_test(
            "pricing/discount_plus_tax",
            vec!["pricing/apply_discount", "pricing/apply_tax", "money/round"],
            "{ let discounted = apply_discount(Decimal::ONE, Decimal::ONE); let taxed = apply_tax(discounted, Decimal::ONE); assert!(taxed >= Decimal::ZERO); }",
        );
        test.imports = Some(vec![
            "rust_decimal::Decimal".to_string(),
            "crate::pricing::apply_discount::apply_discount".to_string(),
            "crate::pricing::apply_tax::apply_tax".to_string(),
        ]);
        let units_by_id: HashMap<&str, &NormalizedUnit> = HashMap::new();

        let code = generate_molecule_tests_code(&[&test], &units_by_id).unwrap();

        assert!(code.contains("use rust_decimal::Decimal;"));
        assert!(code.contains("use crate::pricing::apply_discount::apply_discount;"));
        assert!(code.contains("use crate::pricing::apply_tax::apply_tax;"));
        assert!(
            !code.contains("use crate::money::round::round;"),
            "explicit imports should not synthesize imports from semantic-only covers"
        );
    }

    #[test]
    fn molecule_tests_file_path_root_namespace() {
        let base = PathBuf::from("/tmp/generated");
        let path = molecule_tests_file_path(&base, "");
        assert_eq!(path, PathBuf::from("/tmp/generated/molecule_tests.rs"));
    }

    #[test]
    fn molecule_tests_file_path_nested_namespace() {
        let base = PathBuf::from("/tmp/generated");
        let path = molecule_tests_file_path(&base, "pricing/sub");
        assert_eq!(
            path,
            PathBuf::from("/tmp/generated/pricing/sub/molecule_tests.rs")
        );
    }
}
