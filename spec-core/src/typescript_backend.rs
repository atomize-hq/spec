//! Bounded M46 TypeScript backend generation.
//!
//! This module is intentionally narrow. It only renders the first-class M46
//! TypeScript lane for:
//! - `kind:function`
//! - compatibility key `function.arithmetic_leaf.monotone_up.v1`
//! - `deps: []` or exactly one direct local helper dep
//!
//! It does not promise generic TypeScript parity, molecule parity, seam-kind
//! support, dependency-bearing units, proof routing, or Bun execution.

use crate::semantic_review::{
    SemanticReviewContext, SemanticSupportStatus, evaluate_semantic_review_with_context,
};
use crate::syntax::validate_expect_expr;
use crate::types::{
    Body, DepRef, Intent, LoadedSpec, LocalTest, NormalizedUnit, ResolvedSpec, SpecSource,
    SpecStruct, TYPESCRIPT_BUILD_ENTRY_PATH, TYPESCRIPT_LOCAL_TESTS_PATH,
    TYPESCRIPT_RUNTIME_HELPER_PATH, UnitExtensions,
};
use crate::validator::{
    TYPESCRIPT_HELPER_COMPATIBILITY_KEY, TYPESCRIPT_KIND_UNSUPPORTED_MESSAGE,
    TYPESCRIPT_TARGET_COMPATIBILITY_KEY, validate_typescript_execution_target_spec_with_specs,
};
use crate::{Result, SpecError};
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::path::{Component, Path, PathBuf};

pub fn generate_typescript_unit_module(spec: &ResolvedSpec) -> Result<String> {
    render_typescript_unit_module(spec)
}

pub fn generate_typescript_tree(units: &[NormalizedUnit]) -> Result<BTreeMap<PathBuf, String>> {
    let mut specs = units
        .iter()
        .map(extract_typescript_spec)
        .collect::<Result<Vec<_>>>()?;
    specs.sort_by(|left, right| left.id.cmp(&right.id));
    let specs_by_id = build_typescript_loaded_specs_by_id(&specs);

    for spec in &specs {
        validate_typescript_tree_spec(spec, &specs_by_id)?;
    }

    let mut tree = BTreeMap::new();
    for spec in &specs {
        tree.insert(
            typescript_path_for_unit(spec),
            render_typescript_unit_module_unchecked(spec)?,
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
        NormalizedUnit::Function(spec) => Ok(spec),
        _ => Err(SpecError::Generator {
            message: format!(
                "unit '{}' is not eligible for the bounded M46 TypeScript lane: {}",
                unit.id(),
                TYPESCRIPT_KIND_UNSUPPORTED_MESSAGE
            ),
        }),
    }
}

fn validate_typescript_resolved_spec(spec: &ResolvedSpec) -> Result<()> {
    let specs_by_id = HashMap::from([(spec.id.clone(), typescript_loaded_spec(spec))]);
    validate_typescript_resolved_spec_with_specs(spec, &specs_by_id)
}

fn validate_typescript_resolved_spec_with_specs(
    spec: &ResolvedSpec,
    specs_by_id: &HashMap<String, LoadedSpec>,
) -> Result<()> {
    if spec
        .body_typescript
        .as_deref()
        .map(str::trim)
        .filter(|body| !body.is_empty())
        .is_none()
    {
        return Err(SpecError::Generator {
            message: format!(
                "unit '{}' is not eligible for the bounded M46 TypeScript lane: body.typescript is required",
                spec.id
            ),
        });
    }

    validate_typescript_execution_target_spec_with_specs(&typescript_loaded_spec(spec), specs_by_id)
}

fn validate_typescript_tree_spec(
    spec: &ResolvedSpec,
    specs_by_id: &HashMap<String, LoadedSpec>,
) -> Result<()> {
    if spec
        .body_typescript
        .as_deref()
        .map(str::trim)
        .filter(|body| !body.is_empty())
        .is_none()
    {
        return Err(SpecError::Generator {
            message: format!(
                "unit '{}' is not eligible for the bounded M46 TypeScript lane: body.typescript is required",
                spec.id
            ),
        });
    }

    let loaded = typescript_loaded_spec(spec);
    let semantic_review_context = SemanticReviewContext::new(specs_by_id);
    let Some(review) = evaluate_semantic_review_with_context(&loaded, &semantic_review_context)
    else {
        return Err(SpecError::Generator {
            message: format!(
                "unit '{}' is not eligible for the bounded M46 TypeScript lane: missing supported semantic review",
                spec.id
            ),
        });
    };

    if review.effective_support_status() != SemanticSupportStatus::Supported {
        return Err(SpecError::Generator {
            message: format!(
                "unit '{}' is not eligible for the bounded M46 TypeScript lane: semantic review is {}",
                spec.id, review.compatibility_key
            ),
        });
    }

    match review.compatibility_key.as_str() {
        TYPESCRIPT_TARGET_COMPATIBILITY_KEY => {
            validate_typescript_execution_target_spec_with_specs(&loaded, specs_by_id)
        }
        TYPESCRIPT_HELPER_COMPATIBILITY_KEY if spec.deps.is_empty() => Ok(()),
        TYPESCRIPT_HELPER_COMPATIBILITY_KEY => Err(SpecError::Generator {
            message: format!(
                "unit '{}' is not eligible for the bounded M46 TypeScript lane: helper units must have deps: []",
                spec.id
            ),
        }),
        _ => Err(SpecError::Generator {
            message: format!(
                "unit '{}' is not eligible for the bounded M46 TypeScript lane: semantic family '{}' is unsupported",
                spec.id, review.compatibility_key
            ),
        }),
    }
}

fn build_typescript_loaded_specs_by_id(specs: &[&ResolvedSpec]) -> HashMap<String, LoadedSpec> {
    specs
        .iter()
        .map(|spec| (spec.id.clone(), typescript_loaded_spec(spec)))
        .collect()
}

fn typescript_loaded_spec(spec: &ResolvedSpec) -> LoadedSpec {
    LoadedSpec {
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
    }
}

fn render_typescript_unit_module(spec: &ResolvedSpec) -> Result<String> {
    validate_typescript_resolved_spec(spec)?;
    render_typescript_unit_module_unchecked(spec)
}

fn render_typescript_unit_module_unchecked(spec: &ResolvedSpec) -> Result<String> {
    let runtime_import = relative_import(
        &typescript_path_for_unit(spec),
        Path::new(TYPESCRIPT_RUNTIME_HELPER_PATH),
    );
    let helper_import = render_typescript_helper_import(spec)?;
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
        "import {{ Decimal }} from \"{runtime_import}\";\n"
    ));
    if let Some(helper_import) = helper_import {
        output.push_str(&helper_import);
        output.push('\n');
    }
    output.push_str(&format!("\n{signature} {body}\n"));
    Ok(output)
}

fn render_typescript_helper_import(spec: &ResolvedSpec) -> Result<Option<String>> {
    match spec.deps.as_slice() {
        [] => Ok(None),
        [dep] => {
            let parsed = DepRef::parse(dep).map_err(|err| SpecError::Generator {
                message: format!(
                    "invalid helper dep '{}' reached bounded TypeScript generator: {}",
                    dep, err
                ),
            })?;
            let helper_path = typescript_path_for_unit_id(parsed.unit_id());
            let import_path = relative_import(&typescript_path_for_unit(spec), &helper_path);
            Ok(Some(format!(
                "import {{ {} }} from \"{import_path}\";",
                parsed.callable_name()
            )))
        }
        deps => Err(SpecError::Generator {
            message: format!(
                "bounded TypeScript generator expected at most one direct helper dep; found {}",
                deps.len()
            ),
        }),
    }
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

    Ok(format!(
        "export function {}({params}){returns}",
        spec.fn_name
    ))
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
        format!(
            "bounded TypeScript local test failed: {}#{}",
            spec.id, local_test.id
        )
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
                    message: "bounded TypeScript local test expected integer literal".to_string(),
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
    typescript_path_for_unit_id(&spec.id)
}

fn typescript_path_for_unit_id(unit_id: &str) -> PathBuf {
    let mut path = PathBuf::new();
    if let Some((module_path, fn_name)) = unit_id.rsplit_once('/') {
        path.push(module_path.replace('/', std::path::MAIN_SEPARATOR_STR));
        path.push(format!("{fn_name}.ts"));
    } else {
        path.push(format!("{unit_id}.ts"));
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::semantic_review::evaluate_semantic_review_with_context;
    use crate::types::{
        AuthoredBackends, AuthoredConstructor, AuthoredDataShape, AuthoredField, AuthoredMethod,
        AuthoredMethodLowering, AuthoredRustBackend, AuthoredRustMethodLowering, Contract,
        LoadedSpec, UnitExtensions,
    };
    use indexmap::IndexMap;

    fn monotone_up_spec(id: &str, deps: Vec<&str>, typescript_body: &str) -> ResolvedSpec {
        let fn_name = id.rsplit('/').next().unwrap_or(id);
        ResolvedSpec::from_spec(SpecStruct {
            id: id.to_string(),
            kind: "function".to_string(),
            intent: Intent {
                why: format!("TypeScript fixture for {id}."),
            },
            contract: Some(Contract {
                inputs: Some(IndexMap::from([
                    ("subtotal".to_string(), "rust_decimal::Decimal".to_string()),
                    ("rate".to_string(), "rust_decimal::Decimal".to_string()),
                ])),
                returns: Some("rust_decimal::Decimal".to_string()),
                invariants: vec!["output >= subtotal".to_string()],
            }),
            deps: deps.into_iter().map(str::to_string).collect(),
            imports: vec!["rust_decimal::Decimal".to_string()],
            body: Body {
                rust: "{ subtotal + subtotal * rate }".to_string(),
                typescript: Some(typescript_body.to_string()),
            },
            local_tests: vec![LocalTest {
                id: "happy_path".to_string(),
                expect: format!(
                    "{fn_name}(Decimal::new(1000, 2), Decimal::new(7, 2)) == Decimal::new(1070, 2)"
                ),
            }],
            links: None,
            spec_version: Some("0.3.0".to_string()),
            extensions: UnitExtensions::default(),
        })
    }

    fn helper_spec(id: &str) -> ResolvedSpec {
        let fn_name = id.rsplit('/').next().unwrap_or(id);
        ResolvedSpec::from_spec(SpecStruct {
            id: id.to_string(),
            kind: "function".to_string(),
            intent: Intent {
                why: format!("TypeScript helper fixture for {id}."),
            },
            contract: Some(Contract {
                inputs: Some(IndexMap::from([(
                    "value".to_string(),
                    "rust_decimal::Decimal".to_string(),
                )])),
                returns: Some("rust_decimal::Decimal".to_string()),
                invariants: vec![],
            }),
            deps: vec![],
            imports: vec![
                "rust_decimal::Decimal".to_string(),
                "rust_decimal::RoundingStrategy".to_string(),
            ],
            body: Body {
                rust: "{ value.round_dp_with_strategy(2, RoundingStrategy::MidpointAwayFromZero) }"
                    .to_string(),
                typescript: Some("return value;".to_string()),
            },
            local_tests: vec![LocalTest {
                id: "happy_path".to_string(),
                expect: format!("{fn_name}(Decimal::new(1000, 2)) == Decimal::new(1000, 2)"),
            }],
            links: None,
            spec_version: Some("0.3.0".to_string()),
            extensions: UnitExtensions::default(),
        })
    }

    fn supported_data_seam_spec(id: &str) -> LoadedSpec {
        LoadedSpec {
            source: SpecSource {
                file_path: format!("{id}.unit.spec"),
                id: id.to_string(),
            },
            spec: SpecStruct {
                id: id.to_string(),
                kind: "data".to_string(),
                intent: Intent {
                    why: format!("TypeScript context seam fixture for {id}."),
                },
                contract: None,
                deps: vec![],
                imports: vec![],
                body: Body::default(),
                local_tests: vec![],
                links: None,
                spec_version: Some("0.3.0".to_string()),
                extensions: UnitExtensions {
                    data: Some(AuthoredDataShape {
                        fields: IndexMap::from([
                            (
                                "subtotal".to_string(),
                                AuthoredField {
                                    type_: "Decimal".to_string(),
                                },
                            ),
                            (
                                "discount_rate".to_string(),
                                AuthoredField {
                                    type_: "Decimal".to_string(),
                                },
                            ),
                            (
                                "tax_rate".to_string(),
                                AuthoredField {
                                    type_: "Decimal".to_string(),
                                },
                            ),
                        ]),
                    }),
                    constructors: vec![AuthoredConstructor {
                        id: "new".to_string(),
                        intent: Intent {
                            why: "Create a quote".to_string(),
                        },
                        contract: Some(Contract {
                            inputs: Some(IndexMap::from([
                                ("subtotal".to_string(), "Decimal".to_string()),
                                ("discount_rate".to_string(), "Decimal".to_string()),
                                ("tax_rate".to_string(), "Decimal".to_string()),
                            ])),
                            returns: None,
                            invariants: vec![],
                        }),
                        initializes: IndexMap::from([
                            ("subtotal".to_string(), "subtotal".to_string()),
                            ("discount_rate".to_string(), "discount_rate".to_string()),
                            ("tax_rate".to_string(), "tax_rate".to_string()),
                        ]),
                    }],
                    methods: vec![
                        AuthoredMethod {
                            id: "discounted_subtotal".to_string(),
                            intent: Intent {
                                why: "Compute discounted subtotal".to_string(),
                            },
                            receiver: "shared_ref".to_string(),
                            contract: Some(Contract {
                                inputs: None,
                                returns: Some("Decimal".to_string()),
                                invariants: vec![],
                            }),
                            deps: vec!["pricing/apply_discount".to_string()],
                            lowering: Some(AuthoredMethodLowering {
                                rust: Some(AuthoredRustMethodLowering {
                                    body: "{ apply_discount(self.subtotal, self.discount_rate) }"
                                        .to_string(),
                                }),
                            }),
                        },
                        AuthoredMethod {
                            id: "total".to_string(),
                            intent: Intent {
                                why: "Compute total".to_string(),
                            },
                            receiver: "shared_ref".to_string(),
                            contract: Some(Contract {
                                inputs: None,
                                returns: Some("Decimal".to_string()),
                                invariants: vec![],
                            }),
                            deps: vec!["pricing/apply_tax".to_string()],
                            lowering: Some(AuthoredMethodLowering {
                                rust: Some(AuthoredRustMethodLowering {
                                    body:
                                        "{ apply_tax(self.discounted_subtotal(), self.tax_rate) }"
                                            .to_string(),
                                }),
                            }),
                        },
                    ],
                    backends: Some(AuthoredBackends {
                        rust: Some(AuthoredRustBackend {
                            derives: vec![
                                "Clone".to_string(),
                                "Debug".to_string(),
                                "PartialEq".to_string(),
                            ],
                        }),
                    }),
                    sum: None,
                },
            },
        }
    }

    #[test]
    fn typescript_tree_renders_helper_imports_with_shared_context() {
        let helper = helper_spec("money/round");
        let leaf = monotone_up_spec(
            "pricing/deep/apply_tax",
            vec!["money/round"],
            "return round(subtotal.add(subtotal.mul(rate)));",
        );

        let tree = generate_typescript_tree(&[
            NormalizedUnit::Function(leaf),
            NormalizedUnit::Function(helper),
        ])
        .expect("helper-aware tree should generate");

        assert!(tree.contains_key(&PathBuf::from("money/round.ts")));
        let leaf_module = tree
            .get(&PathBuf::from("pricing/deep/apply_tax.ts"))
            .expect("leaf module should be emitted");
        assert!(leaf_module.contains("import { Decimal } from \"../../__spec_ts/runtime.ts\";"));
        assert!(leaf_module.contains("import { round } from \"../../money/round.ts\";"));

        let local_tests = tree
            .get(&PathBuf::from("__spec_ts/local_tests.ts"))
            .expect("local test harness should be emitted");
        assert!(local_tests
            .contains("import { apply_tax as __spec$pricing$deep$apply_tax } from \"../pricing/deep/apply_tax.ts\";"));
        assert!(local_tests.contains("__spec$pricing$deep$apply_tax"));
    }

    #[test]
    fn typescript_tree_preserves_zero_dep_unit_modules() {
        let leaf = monotone_up_spec(
            "pricing/apply_tax",
            vec![],
            "return subtotal.add(subtotal.mul(rate));",
        );

        let tree = generate_typescript_tree(&[NormalizedUnit::Function(leaf)])
            .expect("zero-dep tree should still generate");

        let leaf_module = tree
            .get(&PathBuf::from("pricing/apply_tax.ts"))
            .expect("leaf module should be emitted");
        assert!(leaf_module.contains("import { Decimal } from \"../__spec_ts/runtime.ts\";"));
        assert!(!leaf_module.contains("import { round }"));
    }

    #[test]
    fn typescript_validation_does_not_regress_with_supported_seam_context() {
        let helper = helper_spec("money/round");
        let leaf = monotone_up_spec(
            "pricing/apply_tax",
            vec!["money/round"],
            "return round(subtotal.add(subtotal.mul(rate)));",
        );
        let seam = supported_data_seam_spec("pricing/checkout_quote");
        let specs_by_id = HashMap::from([
            (leaf.id.clone(), typescript_loaded_spec(&leaf)),
            (helper.id.clone(), typescript_loaded_spec(&helper)),
            (seam.spec.id.clone(), seam.clone()),
        ]);
        let context = SemanticReviewContext::new(&specs_by_id);
        let seam_review = evaluate_semantic_review_with_context(&seam, &context)
            .expect("supported seam review expected in context");

        assert_eq!(seam_review.compatibility_key, "data.pricing_quote.v1");
        assert!(validate_typescript_tree_spec(&helper, &specs_by_id).is_ok());
        assert!(validate_typescript_tree_spec(&leaf, &specs_by_id).is_ok());
    }
}
