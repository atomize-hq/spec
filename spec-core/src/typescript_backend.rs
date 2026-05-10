//! Bounded M45 TypeScript backend generation.
//!
//! This module is intentionally narrow. It only renders the first-class M45
//! TypeScript lane for:
//! - `kind:function`
//! - compatibility key `function.arithmetic_leaf.monotone_up.v1`
//! - `deps: []`
//!
//! It does not promise generic TypeScript parity, molecule parity, seam-kind
//! support, dependency-bearing units, proof routing, or Bun execution.

use crate::syntax::validate_expect_expr;
use crate::types::{
    Body, Intent, LoadedSpec, LocalTest, NormalizedUnit, ResolvedSpec, SpecSource, SpecStruct,
    TYPESCRIPT_BUILD_ENTRY_PATH, TYPESCRIPT_LOCAL_TESTS_PATH, TYPESCRIPT_RUNTIME_HELPER_PATH,
    UnitExtensions,
};
use crate::validator::{
    TYPESCRIPT_KIND_UNSUPPORTED_MESSAGE, validate_typescript_execution_target_spec,
};
use crate::{Result, SpecError};
use std::collections::BTreeMap;
use std::path::{Component, Path, PathBuf};

pub fn generate_typescript_unit_module(spec: &ResolvedSpec) -> Result<String> {
    validate_typescript_resolved_spec(spec)?;
    render_typescript_unit_module(spec)
}

pub fn generate_typescript_tree(units: &[NormalizedUnit]) -> Result<BTreeMap<PathBuf, String>> {
    let mut specs = units
        .iter()
        .map(extract_typescript_spec)
        .collect::<Result<Vec<_>>>()?;
    specs.sort_by(|left, right| left.id.cmp(&right.id));

    let mut tree = BTreeMap::new();
    for spec in &specs {
        tree.insert(
            typescript_path_for_unit(spec),
            render_typescript_unit_module(spec)?,
        );
    }

    tree.insert(
        PathBuf::from(TYPESCRIPT_RUNTIME_HELPER_PATH),
        render_runtime_module(),
    );
    tree.insert(
        PathBuf::from(TYPESCRIPT_BUILD_ENTRY_PATH),
        render_build_entry_module(&specs),
    );
    tree.insert(
        PathBuf::from(TYPESCRIPT_LOCAL_TESTS_PATH),
        render_local_tests_module(&specs)?,
    );

    Ok(tree)
}

fn extract_typescript_spec(unit: &NormalizedUnit) -> Result<&ResolvedSpec> {
    match unit {
        NormalizedUnit::Function(spec) => {
            validate_typescript_resolved_spec(spec)?;
            Ok(spec)
        }
        _ => Err(SpecError::Generator {
            message: format!(
                "unit '{}' is not eligible for the bounded M45 TypeScript lane: {}",
                unit.id(),
                TYPESCRIPT_KIND_UNSUPPORTED_MESSAGE
            ),
        }),
    }
}

fn validate_typescript_resolved_spec(spec: &ResolvedSpec) -> Result<()> {
    if spec
        .body_typescript
        .as_deref()
        .map(str::trim)
        .filter(|body| !body.is_empty())
        .is_none()
    {
        return Err(SpecError::Generator {
            message: format!(
                "unit '{}' is not eligible for the bounded M45 TypeScript lane: body.typescript is required",
                spec.id
            ),
        });
    }

    let loaded = LoadedSpec {
        source: SpecSource {
            file_path: format!("{}.unit.spec", spec.id),
            id: spec.id.clone(),
        },
        spec: SpecStruct {
            id: spec.id.clone(),
            kind: "function".to_string(),
            intent: Intent {
                why: spec.intent_why.clone(),
            },
            contract: spec.contract.clone(),
            deps: spec.deps.clone(),
            imports: spec.imports.clone(),
            body: Body {
                rust: spec.body_rust.clone(),
                typescript: spec.body_typescript.clone(),
            },
            local_tests: spec.local_tests.clone(),
            links: spec.links.clone(),
            spec_version: spec.spec_version.clone(),
            extensions: UnitExtensions::default(),
        },
    };

    validate_typescript_execution_target_spec(&loaded)
}

fn render_typescript_unit_module(spec: &ResolvedSpec) -> Result<String> {
    let runtime_import =
        relative_import(&typescript_path_for_unit(spec), Path::new(TYPESCRIPT_RUNTIME_HELPER_PATH));
    let signature = render_typescript_signature(spec)?;
    let body = render_typescript_body(
        spec.body_typescript
            .as_deref()
            .expect("validated body.typescript should exist"),
    );

    let mut output = String::new();
    if let Some(doc_comment) = render_typescript_doc_comment(&spec.intent_why) {
        output.push_str(&doc_comment);
    }
    output.push_str(&format!(
        "import {{ Decimal }} from \"{runtime_import}\";\n\n{signature} {body}\n"
    ));
    Ok(output)
}

fn render_typescript_signature(spec: &ResolvedSpec) -> Result<String> {
    let params = spec
        .contract
        .as_ref()
        .and_then(|contract| contract.inputs.as_ref())
        .map(|inputs| {
            inputs
                .iter()
                .map(|(name, ty)| map_contract_type(ty).map(|ty| format!("{name}: {ty}")))
                .collect::<Result<Vec<_>>>()
                .map(|parts| parts.join(", "))
        })
        .transpose()?
        .unwrap_or_default();

    let returns = spec
        .contract
        .as_ref()
        .and_then(|contract| contract.returns.as_deref())
        .map(|ty| map_contract_type(ty).map(|ty| format!(": {ty}")))
        .transpose()?
        .unwrap_or_default();

    Ok(format!("export function {}({params}){returns}", spec.fn_name))
}

fn map_contract_type(ty: &str) -> Result<&'static str> {
    match ty.trim() {
        "Decimal" | "rust_decimal::Decimal" => Ok("Decimal"),
        other => Err(SpecError::Generator {
            message: format!(
                "unsupported TypeScript contract type '{}' in bounded M45 generation",
                other
            ),
        }),
    }
}

fn render_typescript_body(body: &str) -> String {
    let trimmed = body.trim();
    if trimmed.starts_with('{') {
        return trimmed.to_string();
    }

    let mut output = String::new();
    output.push_str("{\n");
    for line in trimmed.lines() {
        output.push_str("    ");
        output.push_str(line.trim());
        output.push('\n');
    }
    output.push('}');
    output
}

fn render_typescript_doc_comment(intent_why: &str) -> Option<String> {
    let trimmed = intent_why.trim();
    if trimmed.is_empty() {
        return None;
    }

    let mut output = String::new();
    output.push_str("/**\n");
    for line in trimmed.lines() {
        if line.trim().is_empty() {
            output.push_str(" *\n");
        } else {
            output.push_str(" * ");
            output.push_str(line);
            output.push('\n');
        }
    }
    output.push_str(" */\n");
    Some(output)
}

fn render_runtime_module() -> String {
    r#"export class Decimal {
    private constructor(
        private readonly value: bigint,
        private readonly scale: bigint,
    ) {}

    static new(value: bigint, scale: bigint): Decimal {
        if (scale < 0n) {
            throw new Error("Decimal scale must be non-negative in the bounded M45 runtime");
        }
        return Decimal.normalize(value, scale);
    }

    add(other: Decimal): Decimal {
        const scale = this.scale > other.scale ? this.scale : other.scale;
        const left = this.value * Decimal.pow10(scale - this.scale);
        const right = other.value * Decimal.pow10(scale - other.scale);
        return Decimal.normalize(left + right, scale);
    }

    mul(other: Decimal): Decimal {
        return Decimal.normalize(this.value * other.value, this.scale + other.scale);
    }

    eq(other: Decimal): boolean {
        const left = Decimal.normalize(this.value, this.scale);
        const right = Decimal.normalize(other.value, other.scale);
        return left.value === right.value && left.scale === right.scale;
    }

    private static normalize(value: bigint, scale: bigint): Decimal {
        if (value === 0n) {
            return new Decimal(0n, 0n);
        }

        let normalizedValue = value;
        let normalizedScale = scale;
        while (normalizedScale > 0n && normalizedValue % 10n === 0n) {
            normalizedValue /= 10n;
            normalizedScale -= 1n;
        }
        return new Decimal(normalizedValue, normalizedScale);
    }

    private static pow10(exponent: bigint): bigint {
        let value = 1n;
        let remaining = exponent;
        while (remaining > 0n) {
            value *= 10n;
            remaining -= 1n;
        }
        return value;
    }
}
"#
    .to_string()
}

fn render_build_entry_module(specs: &[&ResolvedSpec]) -> String {
    let build_entry_path = Path::new(TYPESCRIPT_BUILD_ENTRY_PATH);
    let mut output = String::new();
    output.push_str("import \"./runtime.ts\";\n");

    for spec in specs {
        let import_path = relative_import(build_entry_path, &typescript_path_for_unit(spec));
        output.push_str(&format!(
            "import {{ {} as {} }} from \"{import_path}\";\n",
            spec.fn_name,
            typescript_symbol_alias(spec)
        ));
    }

    if !specs.is_empty() {
        output.push('\n');
    }

    for spec in specs {
        output.push_str(&format!("void {};\n", typescript_symbol_alias(spec)));
    }

    output
}

fn render_local_tests_module(specs: &[&ResolvedSpec]) -> Result<String> {
    let local_tests_path = Path::new(TYPESCRIPT_LOCAL_TESTS_PATH);
    let mut output = String::new();
    output.push_str("import { Decimal } from \"./runtime.ts\";\n");

    let specs_with_tests = specs
        .iter()
        .copied()
        .filter(|spec| !spec.local_tests.is_empty())
        .collect::<Vec<_>>();
    for spec in &specs_with_tests {
        let import_path = relative_import(local_tests_path, &typescript_path_for_unit(spec));
        output.push_str(&format!(
            "import {{ {} as {} }} from \"{import_path}\";\n",
            spec.fn_name,
            typescript_symbol_alias(spec)
        ));
    }

    if !specs_with_tests.is_empty() {
        output.push('\n');
    }

    for spec in specs_with_tests {
        for local_test in &spec.local_tests {
            output.push_str(&render_local_test_block(spec, local_test)?);
            output.push('\n');
        }
    }

    Ok(output)
}

fn render_local_test_block(spec: &ResolvedSpec, local_test: &LocalTest) -> Result<String> {
    let expr = validate_expect_expr(local_test.expect.trim(), false).map_err(|err| {
        SpecError::Generator {
            message: format!(
                "failed to parse bounded TypeScript local test for unit '{}' test '{}': {}",
                spec.id,
                local_test.id,
                err.message()
            ),
        }
    })?;

    let syn::Expr::Binary(binary) = expr else {
        return Err(SpecError::Generator {
            message: format!(
                "bounded TypeScript local test shape unexpectedly changed for unit '{}' test '{}'",
                spec.id, local_test.id
            ),
        });
    };
    let syn::Expr::Call(call) = binary.left.as_ref() else {
        return Err(SpecError::Generator {
            message: format!(
                "bounded TypeScript local test call shape unexpectedly changed for unit '{}' test '{}'",
                spec.id, local_test.id
            ),
        });
    };

    let args = call
        .args
        .iter()
        .map(render_decimal_new_expr)
        .collect::<Result<Vec<_>>>()?;
    let expected = render_decimal_new_expr(binary.right.as_ref())?;
    let imported_name = typescript_symbol_alias(spec);

    Ok(format!(
        "const actual_{} = {}({});\nconst expected_{} = {};\nif (!actual_{}.eq(expected_{})) {{\n    throw new Error({:?});\n}}",
        local_test.id,
        imported_name,
        args.join(", "),
        local_test.id,
        expected,
        local_test.id,
        local_test.id,
        format!("bounded TypeScript local test failed: {}#{}", spec.id, local_test.id)
    ))
}

fn render_decimal_new_expr(expr: &syn::Expr) -> Result<String> {
    let syn::Expr::Call(call) = expr else {
        return Err(SpecError::Generator {
            message: "bounded TypeScript local test expected Decimal::new(...)".to_string(),
        });
    };
    if call.args.len() != 2 {
        return Err(SpecError::Generator {
            message: "bounded TypeScript local test expected Decimal::new(value, scale)"
                .to_string(),
        });
    }

    let mut args = call.args.iter();
    let value = render_integer_literal(args.next().expect("value present"))?;
    let scale = render_integer_literal(args.next().expect("scale present"))?;
    Ok(format!("Decimal.new({value}n, {scale}n)"))
}

fn render_integer_literal(expr: &syn::Expr) -> Result<String> {
    match expr {
        syn::Expr::Lit(syn::ExprLit {
            lit: syn::Lit::Int(int),
            ..
        }) => Ok(int.base10_digits().to_string()),
        syn::Expr::Unary(unary) if matches!(unary.op, syn::UnOp::Neg(_)) => {
            let syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Int(int),
                ..
            }) = unary.expr.as_ref()
            else {
                return Err(SpecError::Generator {
                    message: "bounded TypeScript local test expected integer literal"
                        .to_string(),
                });
            };
            Ok(format!("-{}", int.base10_digits()))
        }
        _ => Err(SpecError::Generator {
            message: "bounded TypeScript local test expected integer literal".to_string(),
        }),
    }
}

fn typescript_path_for_unit(spec: &ResolvedSpec) -> PathBuf {
    let mut path = PathBuf::new();
    if !spec.module_path.is_empty() {
        path.push(spec.module_path.replace('/', std::path::MAIN_SEPARATOR_STR));
    }
    path.push(format!("{}.ts", spec.fn_name));
    path
}

fn typescript_symbol_alias(spec: &ResolvedSpec) -> String {
    format!("__spec${}", spec.id.replace('/', "$"))
}

fn relative_import(from: &Path, to: &Path) -> String {
    let from_components = path_components(from.parent().unwrap_or_else(|| Path::new("")));
    let to_components = path_components(to);

    let mut shared = 0usize;
    while shared < from_components.len()
        && shared < to_components.len()
        && from_components[shared] == to_components[shared]
    {
        shared += 1;
    }

    let mut parts = Vec::new();
    for _ in shared..from_components.len() {
        parts.push("..".to_string());
    }
    for component in &to_components[shared..] {
        parts.push(component.clone());
    }

    let joined = parts.join("/");
    if joined.starts_with('.') {
        joined
    } else {
        format!("./{joined}")
    }
}

fn path_components(path: &Path) -> Vec<String> {
    path.components()
        .filter_map(|component| match component {
            Component::Normal(segment) => Some(segment.to_string_lossy().into_owned()),
            _ => None,
        })
        .collect()
}
