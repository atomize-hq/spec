//! Shared syntax helpers for validating local test expectation expressions.

use proc_macro2::{Delimiter, TokenStream, TokenTree};

pub(crate) const MAX_EXPECT_EXPR_DEPTH: usize = 128;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ExpectExprErrorKind {
    Parse(String),
    TooDeep { max_depth: usize },
    UnsafeForm,
}

impl ExpectExprErrorKind {
    pub(crate) fn message(&self) -> String {
        match self {
            Self::Parse(message) => message.clone(),
            Self::TooDeep { max_depth } => {
                format!("expect expression nesting exceeds maximum depth of {max_depth}")
            }
            Self::UnsafeForm => "expect must use only operators, function calls, and value access; block, unsafe, closure, and control-flow forms are not allowed".to_string(),
        }
    }
}

pub(crate) fn validate_expect_expr(
    expr_source: &str,
    allow_unsafe: bool,
) -> Result<syn::Expr, ExpectExprErrorKind> {
    preflight_expect_expr_depth(expr_source)?;

    let expr = syn::parse_str::<syn::Expr>(expr_source)
        .map_err(|err| ExpectExprErrorKind::Parse(err.to_string()))?;

    if !allow_unsafe && !is_safe_expect_expr(&expr) {
        return Err(ExpectExprErrorKind::UnsafeForm);
    }

    Ok(expr)
}

pub(crate) fn preflight_expect_expr_depth(expr_source: &str) -> Result<(), ExpectExprErrorKind> {
    let tokens = expr_source
        .parse::<TokenStream>()
        .map_err(|err| ExpectExprErrorKind::Parse(err.to_string()))?;

    ensure_group_depth_within_limit(&tokens, MAX_EXPECT_EXPR_DEPTH)
}

fn ensure_group_depth_within_limit(
    tokens: &TokenStream,
    max_depth: usize,
) -> Result<(), ExpectExprErrorKind> {
    let mut stack = vec![(tokens.clone().into_iter(), false)];
    let mut depth = 0usize;

    while let Some((iter, _counted_group)) = stack.last_mut() {
        match iter.next() {
            Some(TokenTree::Group(group)) => {
                let count_depth = group.delimiter() != Delimiter::None;
                if count_depth {
                    depth += 1;
                    if depth > max_depth {
                        return Err(ExpectExprErrorKind::TooDeep { max_depth });
                    }
                }
                stack.push((group.stream().into_iter(), count_depth));
            }
            Some(TokenTree::Ident(_) | TokenTree::Punct(_) | TokenTree::Literal(_)) => {}
            None => {
                let (_, counted_group) = stack.pop().expect("stack should not be empty");
                if counted_group {
                    depth = depth.saturating_sub(1);
                }
            }
        }
    }

    Ok(())
}

fn is_safe_expect_expr(expr: &syn::Expr) -> bool {
    is_safe_expect_expr_depth(expr, 0)
}

fn is_safe_expect_expr_depth(expr: &syn::Expr, depth: usize) -> bool {
    if depth >= MAX_EXPECT_EXPR_DEPTH {
        return false;
    }
    let next = depth + 1;
    match expr {
        syn::Expr::Binary(b) => {
            is_safe_expect_expr_depth(&b.left, next) && is_safe_expect_expr_depth(&b.right, next)
        }
        syn::Expr::Call(c) => {
            is_safe_expect_expr_depth(&c.func, next)
                && c.args.iter().all(|a| is_safe_expect_expr_depth(a, next))
        }
        syn::Expr::MethodCall(m) => {
            is_safe_expect_expr_depth(&m.receiver, next)
                && m.args.iter().all(|a| is_safe_expect_expr_depth(a, next))
        }
        syn::Expr::Field(f) => is_safe_expect_expr_depth(&f.base, next),
        syn::Expr::Index(i) => {
            is_safe_expect_expr_depth(&i.expr, next) && is_safe_expect_expr_depth(&i.index, next)
        }
        syn::Expr::Unary(u) => is_safe_expect_expr_depth(&u.expr, next),
        syn::Expr::Path(_) | syn::Expr::Lit(_) => true,
        syn::Expr::Paren(inner) => is_safe_expect_expr_depth(&inner.expr, next),
        syn::Expr::Cast(c) => is_safe_expect_expr_depth(&c.expr, next),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_expect_expr_accepts_safe_expression() {
        let result = validate_expect_expr("apply_discount() == Decimal::ZERO", false);
        assert!(result.is_ok());
    }

    #[test]
    fn validate_expect_expr_returns_too_deep_before_syn_parse() {
        let nested = format!("{}true{}", "(".repeat(200), ")".repeat(200));
        let result = validate_expect_expr(&nested, false);
        match result {
            Err(ExpectExprErrorKind::TooDeep { max_depth }) => {
                assert_eq!(max_depth, MAX_EXPECT_EXPR_DEPTH);
            }
            Err(other) => panic!("expected too-deep error, got {:?}", other),
            Ok(_) => panic!("expected too-deep error, got success"),
        }
    }

    #[test]
    fn validate_expect_expr_rejects_unsafe_forms_when_strict() {
        let result = validate_expect_expr("{ let ok = apply_discount(); ok }", false);
        assert!(matches!(result, Err(ExpectExprErrorKind::UnsafeForm)));
    }

    #[test]
    fn validate_expect_expr_allows_unsafe_forms_when_configured() {
        let result = validate_expect_expr("{ let ok = apply_discount(); ok }", true);
        assert!(result.is_ok());
    }
}
