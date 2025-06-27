// via: https://github.com/rust-lang/rust-analyzer/blob/d8887c0758bbd2d5f752d5bd405d4491e90e7ed6/crates/syntax/src/validation.rs

//! This module implements syntax validation that the parser doesn't handle.
//!
//! A failed validation emits a diagnostic.

use crate::ast::AstNode;
use crate::{SyntaxNode, ast, match_ast, syntax_error::SyntaxError};
use rowan::{TextRange, TextSize};
use squawk_parser::SyntaxKind::*;
pub(crate) fn validate(root: &SyntaxNode, errors: &mut Vec<SyntaxError>) {
    for node in root.descendants() {
        match_ast! {
            match node {
                ast::AlterAggregate(it) => validate_aggregate_params(it.aggregate().and_then(|x| x.param_list()), errors),
                ast::CreateAggregate(it) => validate_aggregate_params(it.param_list(), errors),
                ast::CreateTable(it) => validate_create_table(it, errors),
                ast::PrefixExpr(it) => validate_prefix_expr(it, errors),
                ast::ArrayExpr(it) => validate_array_expr(it, errors),
                ast::DropAggregate(it) => validate_drop_aggregate(it, errors),
                ast::JoinExpr(it) => validate_join_expr(it, errors),
                ast::Literal(it) => validate_literal(it, errors),
                ast::NonStandardParam(it) => validate_non_standard_param(it, errors),
                _ => (),
            }
        }
    }
}

fn validate_create_table(it: ast::CreateTable, acc: &mut Vec<SyntaxError>) {
    let Some(arg_list) = it.table_arg_list() else {
        return;
    };

    let type_required = it.partition_of().is_none() && it.of_type().is_none();

    for arg in arg_list.args() {
        match arg {
            ast::TableArg::Column(column) => {
                let Some(col_name) = column.name() else {
                    continue;
                };
                if type_required && column.ty().is_none() {
                    let end = col_name.syntax().text_range().end();
                    acc.push(SyntaxError::new(
                        "Missing column type",
                        TextRange::new(end, end),
                    ));
                }
            }
            ast::TableArg::LikeClause(_) => (),
            ast::TableArg::TableConstraint(_) => (),
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

fn validate_join_expr(join_expr: ast::JoinExpr, acc: &mut Vec<SyntaxError>) {
    let Some(join) = join_expr.join() else {
        return;
    };

    let Some(join_type) = join.join_type() else {
        return;
    };

    enum JoinClause {
        Required,
        NotAllowed,
    }
    use JoinClause::*;

    let join_clause = if join.natural_token().is_some() {
        NotAllowed
    } else {
        match join_type {
            ast::JoinType::JoinCross(_) => NotAllowed,
            ast::JoinType::JoinFull(_)
            | ast::JoinType::JoinInner(_)
            | ast::JoinType::JoinLeft(_)
            | ast::JoinType::JoinRight(_) => Required,
        }
    };

    let join_name = if join.natural_token().is_some() {
        "natural"
    } else {
        match join_type {
            ast::JoinType::JoinCross(_) => "cross",
            ast::JoinType::JoinFull(_) => "full",
            ast::JoinType::JoinInner(_) => "inner",
            ast::JoinType::JoinLeft(_) => "left",
            ast::JoinType::JoinRight(_) => "right",
        }
    };

    match join_clause {
        Required => {
            if join.on_clause().is_none() && join.using_clause().is_none() {
                let end = join_expr.syntax().text_range().end();
                acc.push(SyntaxError::new(
                    "Join missing condition.",
                    TextRange::new(end, end),
                ));
            }
        }
        NotAllowed => {
            if let Some(on_clause) = join.on_clause() {
                acc.push(SyntaxError::new(
                    format!("Join condition is not allowed for {join_name} joins."),
                    on_clause.syntax().text_range(),
                ));
            }
            if let Some(using_clause) = join.using_clause() {
                acc.push(SyntaxError::new(
                    format!("Join `using` clause is not allowed for {join_name} joins."),
                    using_clause.syntax().text_range(),
                ));
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

fn validate_non_standard_param(param: ast::NonStandardParam, acc: &mut Vec<SyntaxError>) {
    acc.push(SyntaxError::new(
        "Invalid parameter type. Use positional params like $1 instead.",
        param.syntax().text_range(),
    ))
}
