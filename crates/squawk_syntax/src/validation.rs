// via: https://github.com/rust-lang/rust-analyzer/blob/d8887c0758bbd2d5f752d5bd405d4491e90e7ed6/crates/syntax/src/validation.rs

//! This module implements syntax validation that the parser doesn't handle.
//!
//! A failed validation emits a diagnostic.

use crate::ast::AstNode;
use crate::{ast, match_ast, syntax_error::SyntaxError, SyntaxNode};
use rowan::{TextRange, TextSize};
use squawk_parser::SyntaxKind::*;
pub(crate) fn validate(root: &SyntaxNode, errors: &mut Vec<SyntaxError>) {
    for node in root.descendants() {
        match_ast! {
            match node {
                ast::AlterAggregate(it) => validate_aggregate_params(it.aggregate().and_then(|x| x.param_list()), errors),
                ast::CreateAggregate(it) => validate_aggregate_params(it.param_list(), errors),
                ast::PrefixExpr(it) => validate_prefix_expr(it, errors),
                ast::ArrayExpr(it) => validate_array_expr(it, errors),
                ast::DropAggregate(it) => validate_drop_aggregate(it, errors),
                ast::Literal(it) => validate_literal(it, errors),
                _ => (),
            }
        }
    }
}

enum LookingFor {
    OpenString,
    CloseString(TextSize, bool),
}
fn validate_literal(lit: ast::Literal, acc: &mut Vec<SyntaxError>) {
    let mut state = LookingFor::OpenString;
    let mut maybe_errors = vec![];

    // Checking for string continuation issues, like comments between string
    // literals or missing new lines.
    for e in lit.syntax().children_with_tokens() {
        match e {
            rowan::NodeOrToken::Node(_) => {
                // not sure when this would occur
                state = LookingFor::OpenString;
            }
            rowan::NodeOrToken::Token(token) => {
                match state {
                    LookingFor::OpenString => {
                        if token.kind() == STRING {
                            state = LookingFor::CloseString(token.text_range().end(), false);
                        }
                    }
                    LookingFor::CloseString(text_range_end, seen_new_line) => match token.kind() {
                        WHITESPACE => {
                            let seen_new_line = token.text().contains("\n");
                            state = LookingFor::CloseString(text_range_end, seen_new_line);
                        }
                        COMMENT => {
                            maybe_errors.push(SyntaxError::new(
                                "Comments between string literals are not allowed.",
                                token.text_range(),
                            ));
                        }
                        STRING => {
                            // avoid warning twice for the same two string literals, so we check maybe_errors
                            if !seen_new_line && maybe_errors.is_empty() {
                                maybe_errors.push(SyntaxError::new(
                                    "Expected new line or comma between string literals",
                                    TextRange::new(text_range_end, token.text_range().start()),
                                ));
                            }
                            acc.append(&mut maybe_errors);
                            state = LookingFor::CloseString(token.text_range().end(), false);
                        }
                        _ => {
                            maybe_errors.clear();
                            state = LookingFor::OpenString;
                        }
                    },
                }
            }
        }
    }
}

fn validate_drop_aggregate(drop_agg: ast::DropAggregate, acc: &mut Vec<SyntaxError>) {
    for agg in drop_agg.aggregates() {
        validate_aggregate_params(agg.param_list(), acc);
    }
}

fn validate_array_expr(array_expr: ast::ArrayExpr, acc: &mut Vec<SyntaxError>) {
    if array_expr.array_token().is_none() {
        let parent_kind = array_expr.syntax().parent().map(|x| x.kind());
        if matches!(parent_kind, Some(ARRAY_EXPR)) {
            return;
        }
        let expr_range = array_expr.syntax().text_range();
        let range = TextRange::new(expr_range.start(), expr_range.start());
        acc.push(SyntaxError::new("Array missing ARRAY keyword.", range));
    }
}

fn validate_prefix_expr(prefix_expr: ast::PrefixExpr, acc: &mut Vec<SyntaxError>) {
    let Some(op) = prefix_expr
        .syntax()
        .children()
        .find_map(ast::CustomOp::cast)
    else {
        return;
    };
    validate_custom_op(op, acc);
}

// https://www.postgresql.org/docs/17/sql-createoperator.html
fn validate_custom_op(op: ast::CustomOp, acc: &mut Vec<SyntaxError>) {
    // TODO: there's more we can validate
    let mut found = 0;
    for node_or_token in op.syntax().children_with_tokens() {
        match node_or_token {
            rowan::NodeOrToken::Node(_) => (),
            rowan::NodeOrToken::Token(_) => {
                found += 1;
            }
        }
        if found >= 2 {
            return;
        }
    }
    let token = op.syntax().children_with_tokens().find_map(|x| match x {
        rowan::NodeOrToken::Node(_) => None,
        rowan::NodeOrToken::Token(tk) => Some(tk.kind()),
    });
    if let Some(STAR | SLASH | L_ANGLE | R_ANGLE | EQ | PERCENT | CARET) = token {
        acc.push(SyntaxError::new(
            "Invalid operator.",
            op.syntax().text_range(),
        ));
    }
}

fn validate_aggregate_params(aggregate_params: Option<ast::ParamList>, acc: &mut Vec<SyntaxError>) {
    if let Some(params) = aggregate_params {
        for p in params.params() {
            if let Some(mode) = p.mode() {
                match mode {
                    ast::ParamMode::ParamOut(param_out) => acc.push(SyntaxError::new(
                        "Out params are not allowed with aggregates.",
                        param_out.syntax().text_range(),
                    )),
                    ast::ParamMode::ParamInOut(param_in_out) => acc.push(SyntaxError::new(
                        "In Out params are not allowed with aggregates.",
                        param_in_out.syntax().text_range(),
                    )),
                    ast::ParamMode::ParamIn(_) | ast::ParamMode::ParamVariadic(_) => (),
                }
            }
        }
    }
}
