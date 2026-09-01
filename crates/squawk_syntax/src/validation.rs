// via: https://github.com/rust-lang/rust-analyzer/blob/d8887c0758bbd2d5f752d5bd405d4491e90e7ed6/crates/syntax/src/validation.rs

//! This module implements syntax validation that the parser doesn't handle.
//!
//! A failed validation emits a diagnostic.

use std::ops::Range;

use either::Either;

use crate::ast::{AstNode, CastKind, LitKind, PrefixOp};
use crate::unescape::{escape_unicode_esc_str, uescape_char};
use crate::{SyntaxNode, SyntaxToken, ast, match_ast, syntax_error::SyntaxError};
use rowan::{TextRange, TextSize};
use squawk_parser::SyntaxKind::*;
pub(crate) fn validate(root: &SyntaxNode, errors: &mut Vec<SyntaxError>) {
    for node in root.descendants() {
        match_ast! {
            match node {
                ast::Aggregate(it) => validate_aggregate_params(it.param_list(), errors),
                ast::AtomicBody(it) => validate_atomic_body(it, errors),
                ast::BinExpr(it) => validate_bin_expr(it, errors),
                ast::CastExpr(it) => validate_cast_expr(it, errors),
                ast::CreateAggregate(it) => {
                    validate_aggregate_params(it.param_list(), errors);
                    validate_aggregate_variadic_params(it.param_list(), errors);
                },
                ast::CreateFunction(it) => validate_create_function(it, errors),
                ast::CreateProcedure(it) => validate_create_procedure(it, errors),
                ast::CreateTable(it) => validate_create_table(it, errors),
                ast::CreateViewLike(it) => validate_non_empty_column_list(it.column_list(), errors),
                ast::CustomOp(it) => validate_custom_op_length(it, errors),
                ast::Do(it) => validate_do(it, errors),
                ast::FuncOptionList(it) => validate_func_option_list(it, errors),
                ast::FunctionSig(it) => validate_param_defaults(it.param_list(), errors),
                ast::FromAlias(it) => validate_non_empty_column_list(it.columns(), errors),
                ast::RetType(it) => validate_non_empty_column_list(it.return_table_arg_list(), errors),
                ast::WithTable(it) => validate_non_empty_column_list(it.column_list(), errors),
                ast::PrefixExpr(it) => validate_prefix_expr(it, errors),
                ast::ProcedureSig(it) => validate_param_defaults(it.param_list(), errors),
                ast::RoutineSig(it) => validate_param_defaults(it.param_list(), errors),
                ast::ArrayExpr(it) => validate_array_expr(it, errors),
                ast::JoinExpr(it) => validate_join_expr(it, errors),
                ast::Literal(it) => validate_literal(it, errors),
                ast::NonStandardParam(it) => validate_non_standard_param(it, errors),
                ast::RuleStmtList(it) => validate_rule_stmt_list(it, errors),
                ast::Select(it) => validate_select(it, errors),
                ast::SelectInto(it) => validate_select_into(it, errors),
                ast::SetSingleColumn(it) => validate_set_single_column(it, errors),
                ast::SourceFile(it) => validate_source_file(it, errors),
                ast::Type(it) => validate_type_modifiers(it, errors),
                _ => (),
            }
        }
    }
    for element in root.descendants_with_tokens() {
        if let Some(token) = element.into_token()
            && token.kind() == IDENT
        {
            validate_unicode_esc_ident(&token, errors);
        }
    }
}

// an empty column list is only valid for a composite type:
// `create type t as ()`
fn validate_non_empty_column_list(column_list: Option<impl AstNode>, acc: &mut Vec<SyntaxError>) {
    let Some(column_list) = column_list else {
        return;
    };
    let syntax = column_list.syntax();
    if syntax.children().next().is_some() {
        return;
    }
    acc.push(SyntaxError::new(
        "Expected at least one column",
        syntax.text_range(),
    ));
}

fn validate_atomic_body(it: ast::AtomicBody, acc: &mut Vec<SyntaxError>) {
    for option in it.routine_body_stmts() {
        let ast::RoutineBodyStmt::Stmt(stmt) = option else {
            continue;
        };
        let syntax = stmt.syntax();
        if syntax.kind() == EMPTY_STMT {
            continue;
        }
        let ends_with_semi = syntax.last_token().is_some_and(|t| t.kind() == SEMICOLON);
        if ends_with_semi {
            continue;
        }
        let end = syntax.text_range().end();
        acc.push(SyntaxError::new(
            "Missing semicolon after statement",
            TextRange::empty(end),
        ));
    }
}

fn validate_rule_stmt_list(it: ast::RuleStmtList, acc: &mut Vec<SyntaxError>) {
    let mut stmts = it.rule_stmts().peekable();
    while let Some(stmt) = stmts.next() {
        let syntax = stmt.syntax();
        if stmts.peek().is_none() {
            continue;
        }
        let ends_with_semi = syntax.last_token().is_some_and(|t| t.kind() == SEMICOLON);
        if ends_with_semi {
            continue;
        }
        let end = syntax.text_range().end();
        acc.push(SyntaxError::new(
            "Missing semicolon between statements",
            TextRange::empty(end),
        ));
    }
}

fn validate_source_file(it: ast::SourceFile, acc: &mut Vec<SyntaxError>) {
    let mut stmts = it.stmts().peekable();
    while let Some(stmt) = stmts.next() {
        let syntax = stmt.syntax();
        if syntax.kind() == EMPTY_STMT {
            continue;
        }
        let Some(next) = stmts.peek() else {
            continue;
        };
        let ends_with_semi = syntax.last_token().is_some_and(|t| t.kind() == SEMICOLON);
        if ends_with_semi || next.syntax().kind() == EMPTY_STMT {
            continue;
        }
        let end = syntax.text_range().end();
        acc.push(SyntaxError::new(
            "Missing semicolon between statements",
            TextRange::empty(end),
        ));
    }
}

fn validate_select(it: ast::Select, acc: &mut Vec<SyntaxError>) {
    let Some(from_clause) = it.from_clause() else {
        return;
    };
    if let Some(select_clause) = it.select_clause() {
        if from_clause.syntax().text_range().end() < select_clause.syntax().text_range().start() {
            // Postgres dialect doesn't support leading from clauses, e.g., `from t select c`
            acc.push(SyntaxError::new(
                "Leading from clauses are not supported in Postgres",
                from_clause.syntax().text_range(),
            ));
        }
    } else {
        // Postgres dialect doesn't support missing select clauses, e.g., `from t`
        acc.push(SyntaxError::new(
            "Missing select clause",
            TextRange::empty(from_clause.syntax().text_range().start()),
        ));
    }
}

fn validate_cast_expr(it: ast::CastExpr, acc: &mut Vec<SyntaxError>) {
    if it.kind() != Some(CastKind::TypeLiteral) {
        return;
    }
    let Some(literal) = it.literal().and_then(|literal| literal.kind()) else {
        return;
    };
    let (message, token) = match literal {
        LitKind::BitString(token) => ("Bit string literals cannot be used in type literals", token),
        LitKind::ByteString(token) => (
            "Hexadecimal string literals cannot be used in type literals",
            token,
        ),
        LitKind::NationalString(token) => (
            "National character string literals cannot be used in type literals",
            token,
        ),
        _ => return,
    };
    acc.push(SyntaxError::new(message, token.text_range()));
}

fn validate_type_modifiers(ty: ast::Type, acc: &mut Vec<SyntaxError>) {
    let Some(arg_list) = ty.arg_list() else {
        return;
    };

    for arg in arg_list.args() {
        if arg.variadic_token().is_some() {
            acc.push(SyntaxError::new(
                "Type modifiers must be simple constants or identifiers",
                arg.syntax().text_range(),
            ));
            continue;
        }
        if let Some(named_arg) = arg.named_arg() {
            acc.push(SyntaxError::new(
                "Type modifier cannot have parameter name",
                named_arg.syntax().text_range(),
            ));
            continue;
        }
        if let Some(order_by) = arg.order_by_clause() {
            acc.push(SyntaxError::new(
                "Type modifier cannot have ORDER BY",
                order_by.syntax().text_range(),
            ));
            continue;
        }
        let Some(expr) = arg.expr() else {
            continue;
        };
        if !is_simple_type_modifier(&expr) {
            acc.push(SyntaxError::new(
                "Type modifiers must be simple constants or identifiers",
                expr.syntax().text_range(),
            ));
        }
    }
}

fn is_simple_type_modifier(expr: &ast::Expr) -> bool {
    match expr {
        ast::Expr::Literal(literal) => matches!(
            literal.kind(),
            Some(
                LitKind::DollarQuotedString(_)
                    | LitKind::EscString(_)
                    | LitKind::IntNumber(_)
                    | LitKind::NumericNumber(_)
                    | LitKind::String(_)
                    | LitKind::UnicodeEscString(_)
            )
        ),
        ast::Expr::NameRef(_) => true,
        ast::Expr::PrefixExpr(prefix) => {
            matches!(prefix.op(), Some(PrefixOp::Minus(_)))
                && prefix.expr().is_some_and(|expr| {
                    matches!(
                        expr,
                        ast::Expr::Literal(ref literal)
                            if matches!(
                                literal.kind(),
                                Some(LitKind::IntNumber(_) | LitKind::NumericNumber(_))
                            )
                    )
                })
        }
        ast::Expr::ArrayExpr(_)
        | ast::Expr::BetweenExpr(_)
        | ast::Expr::BinExpr(_)
        | ast::Expr::CallExpr(_)
        | ast::Expr::CaseExpr(_)
        | ast::Expr::CastExpr(_)
        | ast::Expr::Collate(_)
        | ast::Expr::FieldExpr(_)
        | ast::Expr::IndexExpr(_)
        | ast::Expr::ParenExpr(_)
        | ast::Expr::PostfixExpr(_)
        | ast::Expr::SliceExpr(_)
        | ast::Expr::TupleExpr(_) => false,
    }
}

fn validate_select_into(it: ast::SelectInto, acc: &mut Vec<SyntaxError>) {
    for (child, ancestor) in it.syntax().ancestors().zip(it.syntax().ancestors().skip(1)) {
        let kind = ancestor.kind();
        if ast::ParenSelect::can_cast(kind) {
            continue;
        } else if let Some(compound_select) = ast::CompoundSelect::cast(ancestor) {
            if compound_select
                .lhs()
                .is_some_and(|lhs| lhs.syntax() == &child)
            {
                continue;
            }
            acc.push(SyntaxError::new(
                "INTO is only allowed on first SELECT of UNION/INTERSECT/EXCEPT",
                it.syntax().text_range(),
            ));
            return;
        } else if ast::Explain::can_cast(kind)
            || ast::Prepare::can_cast(kind)
            || ast::SourceFile::can_cast(kind)
        {
            return;
        }

        acc.push(SyntaxError::new(
            "SELECT ... INTO is not allowed here",
            it.syntax().text_range(),
        ));
        return;
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
                if let Some(col_name) = column.name()
                    && type_required
                    && column.ty().is_none()
                {
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
                        if matches!(
                            token.kind(),
                            STRING
                                | ESC_STRING
                                | BIT_STRING
                                | BYTE_STRING
                                | UNICODE_ESC_STRING
                                | NATIONAL_STRING
                        ) {
                            state = LookingFor::CloseString(token.text_range().end(), false);
                        }
                    }
                    LookingFor::CloseString(text_range_end, seen_new_line) => match token.kind() {
                        WHITESPACE => {
                            let seen_new_line = seen_new_line
                                || token.text().contains('\n')
                                || token.text().contains('\r');
                            state = LookingFor::CloseString(text_range_end, seen_new_line);
                        }
                        COMMENT if token.text().starts_with("--") => (),
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

    validate_unicode_esc_string(&lit, acc);
    validate_prefixed_strings(&lit, acc);
    validate_default_literal(&lit, acc);
}

fn validate_default_literal(lit: &ast::Literal, acc: &mut Vec<SyntaxError>) {
    if !matches!(lit.kind(), Some(LitKind::Default(_))) {
        return;
    }
    let node = lit.syntax();
    if is_valid_default_literal_position(node) {
        return;
    }
    acc.push(SyntaxError::new(
        "DEFAULT is not allowed in this context",
        node.text_range(),
    ));
}

fn is_valid_default_literal_position(literal: &SyntaxNode) -> bool {
    for ancestor in literal.ancestors().skip(1) {
        match ancestor.kind() {
            // unwrap parens
            PAREN_EXPR => continue,
            SET_EXPR => return true,
            ROW => return is_row_in_insert_values(&ancestor),
            _ => return false,
        }
    }
    false
}

fn is_row_in_insert_values(row: &SyntaxNode) -> bool {
    // row_list
    row.parent()
        // values
        .and_then(|n| n.parent())
        // insert / merge_insert
        .and_then(|n| n.parent())
        .is_some_and(|p| matches!(p.kind(), INSERT | MERGE_INSERT))
}

fn validate_set_single_column(it: ast::SetSingleColumn, acc: &mut Vec<SyntaxError>) {
    let Some(set_expr) = it.set_expr() else {
        return;
    };
    if set_expr.default_token().is_none() {
        return;
    }
    let Some(column_target) = it.column_target() else {
        return;
    };
    let Some(accessor) = column_target.accessors().next() else {
        return;
    };
    let message = match accessor {
        ast::Accessor::FieldAccessor(_) => "cannot set a subfield to DEFAULT",
        ast::Accessor::IndexAccessor(_) | ast::Accessor::SliceAccessor(_) => {
            "cannot set an array element to DEFAULT"
        }
    };
    acc.push(SyntaxError::new(
        message,
        column_target.syntax().text_range(),
    ));
}

#[derive(Clone, Copy)]
enum PrefixedKind {
    Bit,
    Byte,
    Esc,
}

fn validate_prefixed_strings(lit: &ast::Literal, acc: &mut Vec<SyntaxError>) {
    let mut continuation: Option<PrefixedKind> = None;
    for e in lit.syntax().children_with_tokens() {
        let Some(token) = e.into_token() else {
            continue;
        };
        match token.kind() {
            ESC_STRING => {
                let Some((inner, inner_start)) = prefixed_str_inner(&token, ['e', 'E']) else {
                    continue;
                };
                validate_escape_string_content(inner, inner_start, acc);
                continuation = Some(PrefixedKind::Esc);
            }
            BIT_STRING => {
                let Some((inner, inner_start)) = prefixed_str_inner(&token, ['b', 'B']) else {
                    continue;
                };
                validate_bit_string_content(inner, inner_start, acc);
                continuation = Some(PrefixedKind::Bit);
            }
            BYTE_STRING => {
                let Some((inner, inner_start)) = prefixed_str_inner(&token, ['x', 'X']) else {
                    continue;
                };
                validate_byte_string_content(inner, inner_start, acc);
                continuation = Some(PrefixedKind::Byte);
            }
            STRING => {
                let Some(continuation) = continuation else {
                    continue;
                };
                let Some(inner) = token
                    .text()
                    .strip_prefix('\'')
                    .and_then(|s| s.strip_suffix('\''))
                else {
                    continue;
                };
                let inner_start = token.text_range().start() + TextSize::new(1);
                match continuation {
                    PrefixedKind::Esc => validate_escape_string_content(inner, inner_start, acc),
                    PrefixedKind::Bit => validate_bit_string_content(inner, inner_start, acc),
                    PrefixedKind::Byte => validate_byte_string_content(inner, inner_start, acc),
                };
            }
            WHITESPACE | COMMENT => (),
            _ => continuation = None,
        }
    }
}

fn validate_bit_string_content(inner: &str, inner_start: TextSize, acc: &mut Vec<SyntaxError>) {
    for (i, c) in inner.char_indices() {
        if c != '0' && c != '1' {
            acc.push(SyntaxError::new(
                format!(r#""{c}" is not a valid binary digit"#),
                offset_range(inner_start, i..i + c.len_utf8()),
            ));
        }
    }
}

fn validate_byte_string_content(inner: &str, inner_start: TextSize, acc: &mut Vec<SyntaxError>) {
    for (i, c) in inner.char_indices() {
        if !c.is_ascii_hexdigit() {
            acc.push(SyntaxError::new(
                format!(r#""{c}" is not a valid hexadecimal digit"#),
                offset_range(inner_start, i..i + c.len_utf8()),
            ));
        }
    }
}

fn prefixed_str_inner(token: &SyntaxToken, prefix: [char; 2]) -> Option<(&str, TextSize)> {
    let inner = token
        .text()
        .strip_prefix(prefix)
        .and_then(|s| s.strip_prefix('\''))
        .and_then(|s| s.strip_suffix('\''))?;
    let inner_start = token.text_range().start() + TextSize::new(2);
    Some((inner, inner_start))
}

fn validate_escape_string_content(inner: &str, inner_start: TextSize, acc: &mut Vec<SyntaxError>) {
    let mut chars = inner.char_indices().peekable();
    while let Some((esc_start, c)) = chars.next() {
        if c != '\\' {
            continue;
        }
        let Some((next_pos, next_c)) = chars.next() else {
            return;
        };
        let (required, example) = match next_c {
            'u' => (4usize, r"\uXXXX"),
            'U' => (8usize, r"\UXXXXXXXX"),
            _ => continue,
        };
        let mut end = next_pos + next_c.len_utf8();
        let mut got_all = true;
        for _ in 0..required {
            match chars.peek() {
                Some(&(i, ch)) if ch.is_ascii_hexdigit() => {
                    end = i + ch.len_utf8();
                    chars.next();
                }
                _ => {
                    got_all = false;
                    break;
                }
            }
        }
        if !got_all {
            acc.push(SyntaxError::new(
                format!("Unicode escape requires {required} hex digits: {example}"),
                offset_range(inner_start, esc_start..end),
            ));
        }
    }
}

fn validate_unicode_esc_string(lit: &ast::Literal, acc: &mut Vec<SyntaxError>) {
    let mut unicode_esc = None;
    let mut continuations: Vec<SyntaxToken> = vec![];
    let mut seen_uescape = false;
    let mut escape_char = '\\';
    for e in lit.syntax().children_with_tokens() {
        let Some(token) = e.into_token() else {
            continue;
        };
        match token.kind() {
            UNICODE_ESC_STRING => unicode_esc = Some(token),
            UESCAPE_KW => seen_uescape = true,
            STRING if seen_uescape => {
                escape_char = match uescape_char(token.text()) {
                    Some(ch) => ch,
                    None => {
                        acc.push(SyntaxError::new(
                            "Invalid unicode escape character",
                            token.text_range(),
                        ));
                        return;
                    }
                };
                break;
            }
            STRING if unicode_esc.is_some() => continuations.push(token),
            _ => (),
        }
    }
    let Some(token) = unicode_esc else {
        return;
    };
    let Some(inner) = token
        .text()
        .strip_prefix(['u', 'U'])
        .and_then(|s| s.strip_prefix("&'"))
        .and_then(|s| s.strip_suffix('\''))
    else {
        return;
    };
    let inner_start = token.text_range().start() + TextSize::new(3);
    escape_unicode_esc_str(inner, escape_char, |range, result| {
        if let Err(err) = result {
            acc.push(SyntaxError::new(
                err.to_string(),
                offset_range(inner_start, range),
            ));
        }
    });
    for cont in continuations {
        let Some(cont_inner) = cont
            .text()
            .strip_prefix('\'')
            .and_then(|s| s.strip_suffix('\''))
        else {
            continue;
        };
        let cont_start = cont.text_range().start() + TextSize::new(1);
        escape_unicode_esc_str(cont_inner, escape_char, |range, result| {
            if let Err(err) = result {
                acc.push(SyntaxError::new(
                    err.to_string(),
                    offset_range(cont_start, range),
                ));
            }
        });
    }
}

fn validate_unicode_esc_ident(token: &SyntaxToken, acc: &mut Vec<SyntaxError>) {
    let Some(inner) = token
        .text()
        .strip_prefix(['u', 'U'])
        .and_then(|s| s.strip_prefix("&\""))
        .and_then(|s| s.strip_suffix('"'))
    else {
        return;
    };

    let mut escape_char = '\\';
    let mut seen_uescape = false;
    let mut next = token.next_sibling_or_token();
    while let Some(element) = next {
        match element.kind() {
            WHITESPACE | COMMENT => (),
            UESCAPE_KW => seen_uescape = true,
            STRING if seen_uescape => {
                if let Some(string_token) = element.as_token() {
                    escape_char = match uescape_char(string_token.text()) {
                        Some(ch) => ch,
                        None => {
                            acc.push(SyntaxError::new(
                                "Invalid unicode escape character",
                                string_token.text_range(),
                            ));
                            return;
                        }
                    };
                }
                break;
            }
            _ => break,
        }
        next = element.next_sibling_or_token();
    }

    let inner_start = token.text_range().start() + TextSize::new(3);
    escape_unicode_esc_str(inner, escape_char, |range, result| {
        if let Err(err) = result {
            acc.push(SyntaxError::new(
                err.to_string(),
                offset_range(inner_start, range),
            ));
        }
    });
}

fn offset_range(start: TextSize, range: Range<usize>) -> TextRange {
    let begin = start + TextSize::new(range.start as u32);
    let end = start + TextSize::new(range.end as u32);
    TextRange::new(begin, end)
}

fn validate_bin_expr(bin_expr: ast::BinExpr, acc: &mut Vec<SyntaxError>) {
    match bin_expr.op() {
        Some(ast::BinOp::In(_) | ast::BinOp::NotIn(_)) => validate_in_expr(&bin_expr, acc),
        Some(ast::BinOp::Overlaps(_)) => validate_overlaps_expr(&bin_expr, acc),
        _ => (),
    }
}

// see: https://christopher.xyz/2020/08/01/any-all-pg.html
fn validate_in_expr(bin_expr: &ast::BinExpr, acc: &mut Vec<SyntaxError>) {
    let rhs = match bin_expr.rhs() {
        None | Some(ast::Expr::ParenExpr(_)) => return,
        Some(ast::Expr::TupleExpr(tuple)) if tuple.row_token().is_none() => {
            if tuple.exprs().next().is_none() {
                acc.push(SyntaxError::new(
                    "Expected at least one expression in IN list.",
                    tuple.syntax().text_range(),
                ));
            }
            return;
        }
        Some(rhs) => rhs,
    };
    acc.push(SyntaxError::new(
        "Expected a parenthesized value list or subquery.",
        rhs.syntax().text_range(),
    ));
}

fn validate_overlaps_expr(bin_expr: &ast::BinExpr, acc: &mut Vec<SyntaxError>) {
    for operand in [bin_expr.lhs(), bin_expr.rhs()].into_iter().flatten() {
        match operand {
            ast::Expr::TupleExpr(tuple) if tuple.exprs().count() != 2 => acc.push(
                SyntaxError::new("wrong number of parameters", tuple.syntax().text_range()),
            ),
            ast::Expr::TupleExpr(_) => (),
            operand => acc.push(SyntaxError::new(
                "OVERLAPS operand must be a row expression",
                operand.syntax().text_range(),
            )),
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
            if join.join_condition().is_none() {
                let end = join_expr.syntax().text_range().end();
                acc.push(SyntaxError::new(
                    "Join missing condition.",
                    TextRange::new(end, end),
                ));
            }
        }
        NotAllowed => {
            if let Some(ast::JoinCondition::JoinUsingClause(using_clause)) = join.join_condition() {
                acc.push(SyntaxError::new(
                    format!("Join `using` clause is not allowed for {join_name} joins."),
                    using_clause.syntax().text_range(),
                ));
            }
        }
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

// NAMEDATALEN == 64 and idents and operators can be NAMEDATALEN - 1
const MAX_OPERATOR_LEN: TextSize = TextSize::new(63);
fn validate_custom_op_length(op: ast::CustomOp, acc: &mut Vec<SyntaxError>) {
    let range = op.syntax().text_range();
    if range.len() > MAX_OPERATOR_LEN {
        acc.push(SyntaxError::new("operator too long", range));
    }
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

fn validate_create_function(function: ast::CreateFunction, acc: &mut Vec<SyntaxError>) {
    validate_routine_body(function.option_list(), function.body(), acc);
    validate_variadic_params(function.param_list(), ParamContext::Func, acc);

    let returns_table = function
        .ret_type()
        .is_some_and(|ret_type| ret_type.table_token().is_some());
    if !returns_table {
        return;
    }

    let Some(params) = function.param_list() else {
        return;
    };
    for param in params.all_params() {
        let invalid_mode = match param.mode() {
            Some(ast::ParamMode::ParamOut(mode)) => Some(mode.syntax().text_range()),
            Some(ast::ParamMode::ParamInOut(mode)) => Some(mode.syntax().text_range()),
            Some(ast::ParamMode::ParamIn(_)) | Some(ast::ParamMode::ParamVariadic(_)) | None => {
                None
            }
        };
        if let Some(range) = invalid_mode {
            acc.push(SyntaxError::new(
                "OUT and INOUT arguments aren't allowed in TABLE functions",
                range,
            ));
        }
    }
}

fn validate_create_procedure(procedure: ast::CreateProcedure, acc: &mut Vec<SyntaxError>) {
    validate_routine_body(procedure.option_list(), procedure.body(), acc);
    validate_variadic_params(procedure.param_list(), ParamContext::Procedure, acc);
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ParamContext {
    Func,
    Agg,
    Procedure,
}

fn validate_variadic_params(
    params: Option<ast::ParamList>,
    context: ParamContext,
    acc: &mut Vec<SyntaxError>,
) {
    let Some(params) = params else {
        return;
    };
    validate_variadic_param_iter(params.all_params(), context, acc);
}

fn validate_aggregate_variadic_params(params: Option<ast::ParamList>, acc: &mut Vec<SyntaxError>) {
    let Some(params) = params else {
        return;
    };
    validate_variadic_param_iter(params.params(), ParamContext::Agg, acc);
    if let Some(order_by) = params.aggregate_order_by() {
        validate_variadic_param_iter(order_by.params(), ParamContext::Agg, acc);
    }
}

fn validate_variadic_param_iter(
    params: impl Iterator<Item = ast::Param>,
    context: ParamContext,
    acc: &mut Vec<SyntaxError>,
) {
    let mut seen_variadic = false;
    for param in params {
        if !seen_variadic {
            seen_variadic = matches!(param.mode(), Some(ast::ParamMode::ParamVariadic(_)));
            continue;
        }
        if matches!(param.mode(), Some(ast::ParamMode::ParamOut(_))) {
            if matches!(context, ParamContext::Func | ParamContext::Agg) {
                continue;
            }
            acc.push(SyntaxError::new(
                "VARIADIC param must be last.",
                param.syntax().text_range(),
            ));
            continue;
        }
        acc.push(SyntaxError::new(
            "VARIADIC param must be last input param.",
            param.syntax().text_range(),
        ));
    }
}

fn validate_param_defaults(params: Option<ast::ParamList>, acc: &mut Vec<SyntaxError>) {
    let Some(params) = params else {
        return;
    };
    for param in params.all_params() {
        if let Some(default) = param.param_default() {
            acc.push(SyntaxError::new(
                "Defaults are not allowed.",
                default.syntax().text_range(),
            ));
        }
    }
}

fn validate_aggregate_params(aggregate_params: Option<ast::ParamList>, acc: &mut Vec<SyntaxError>) {
    validate_param_defaults(aggregate_params.clone(), acc);
    if let Some(params) = aggregate_params {
        for p in params.all_params() {
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

const CONFLICTING_OPTIONS: &str = "Conflicting or redundant options.";

#[derive(Clone, Copy, PartialEq)]
enum FuncOptionGroup {
    As,
    Cost,
    Language,
    Leakproof,
    Parallel,
    Rows,
    Security,
    Strict,
    Support,
    Transform,
    Volatility,
    Window,
}

fn func_option_group(option: &ast::FuncOption) -> Option<FuncOptionGroup> {
    let group = match option {
        ast::FuncOption::AsFuncOption(_) => FuncOptionGroup::As,
        ast::FuncOption::CostFuncOption(_) => FuncOptionGroup::Cost,
        ast::FuncOption::LanguageFuncOption(_) => FuncOptionGroup::Language,
        ast::FuncOption::LeakproofFuncOption(_) | ast::FuncOption::NotLeakproofFuncOption(_) => {
            FuncOptionGroup::Leakproof
        }
        ast::FuncOption::ParallelFuncOption(_) => FuncOptionGroup::Parallel,
        ast::FuncOption::RowsFuncOption(_) => FuncOptionGroup::Rows,
        ast::FuncOption::SecurityDefinerFuncOption(_)
        | ast::FuncOption::SecurityInvokerFuncOption(_) => FuncOptionGroup::Security,
        ast::FuncOption::CalledOnNullInputFuncOption(_)
        | ast::FuncOption::ReturnsNullOnNullInputFuncOption(_)
        | ast::FuncOption::StrictFuncOption(_) => FuncOptionGroup::Strict,
        ast::FuncOption::SupportFuncOption(_) => FuncOptionGroup::Support,
        ast::FuncOption::TransformFuncOption(_) => FuncOptionGroup::Transform,
        ast::FuncOption::VolatilityFuncOption(_) => FuncOptionGroup::Volatility,
        ast::FuncOption::WindowFuncOption(_) => FuncOptionGroup::Window,
        ast::FuncOption::ResetFuncOption(_) | ast::FuncOption::SetFuncOption(_) => return None,
    };
    Some(group)
}

fn validate_func_option_list(option_list: ast::FuncOptionList, acc: &mut Vec<SyntaxError>) {
    let mut seen: Vec<FuncOptionGroup> = vec![];
    for option in option_list.options() {
        let Some(group) = func_option_group(&option) else {
            continue;
        };
        if seen.contains(&group) {
            acc.push(SyntaxError::new(
                CONFLICTING_OPTIONS,
                option.syntax().text_range(),
            ));
        } else {
            seen.push(group);
        }
    }
}

fn validate_routine_body(
    option_list: Option<ast::FuncOptionList>,
    body: Option<ast::RoutineBody>,
    acc: &mut Vec<SyntaxError>,
) {
    let Some(body) = body else {
        return;
    };
    let has_as_option = option_list
        .into_iter()
        .flat_map(|options| options.options())
        .any(|option| matches!(option, ast::FuncOption::AsFuncOption(_)));
    if has_as_option {
        acc.push(SyntaxError::new(
            "Duplicate function body.",
            body.syntax().text_range(),
        ));
    }
}

fn validate_do(do_: ast::Do, acc: &mut Vec<SyntaxError>) {
    let mut seen_language = false;
    let mut seen_body = false;
    for part in do_.language_and_body() {
        let (seen, range) = match part {
            Either::Left(language) => (&mut seen_language, language.syntax().text_range()),
            Either::Right(body) => (&mut seen_body, body.syntax().text_range()),
        };
        if *seen {
            acc.push(SyntaxError::new(CONFLICTING_OPTIONS, range));
        } else {
            *seen = true;
        }
    }
}
