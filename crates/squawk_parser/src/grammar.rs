// Modeled after rust-analyzer's grammar, but SQL instead of Rust!
// https://github.com/rust-lang/rust-analyzer/tree/d8887c0758bbd2d5f752d5bd405d4491e90e7ed6/crates/parser/src/grammar

use crate::{
    syntax_kind::{
        SyntaxKind::{self, *},
        ALL_KEYWORDS, BARE_LABEL_KEYWORDS, COLUMN_OR_TABLE_KEYWORDS, RESERVED_KEYWORDS,
        TYPE_KEYWORDS, UNRESERVED_KEYWORDS,
    },
    token_set::TokenSet,
    CompletedMarker, Marker, Parser,
};

const EXPR_RECOVERY_SET: TokenSet = TokenSet::new(&[
    R_PAREN, // is this bracket stuff right?
    R_BRACK, // guessing here
    SEMICOLON,
    // handles cases like:
    //   select 1 select 2
    // which should be written as:
    //   select 1; select 2;
    SELECT_KW,
]);

fn literal(p: &mut Parser<'_>) -> Option<CompletedMarker> {
    if !p.at_ts(LITERAL_FIRST) {
        return None;
    }
    let m = p.start();
    p.bump_any();
    Some(m.complete(p, LITERAL))
}

// array[1,2,3]
// array(select 1)
fn array_expr(p: &mut Parser<'_>, m: Option<Marker>) -> CompletedMarker {
    let m = m.unwrap_or_else(|| p.start());
    // `[` or `(`
    let closing = if p.eat(L_PAREN) {
        R_PAREN
    } else {
        p.expect(L_BRACK);
        R_BRACK
    };
    while !p.at(EOF) && !p.at(closing) {
        if p.at_ts(SELECT_FIRST) && (select_stmt(p, None).is_none() || p.at(EOF) || p.at(closing)) {
            break;
        }
        if expr(p).is_none() {
            break;
        }
        if p.at(COMMA) && p.nth_at(1, closing) {
            p.err_and_bump("unexpected trailing comma");
            break;
        }
        if !p.at(closing) && !p.expect(COMMA) {
            break;
        }
    }
    p.expect(closing);
    m.complete(p, ARRAY_EXPR)
}

fn paren_select(p: &mut Parser<'_>) -> Option<CompletedMarker> {
    let m = p.start();
    if !p.eat(L_PAREN) {
        m.abandon(p);
        return None;
    }
    while !p.at(EOF) && !p.at(R_PAREN) {
        // saw_expr = true;
        // we want to check for select stuff before we get the the expr stuff maybe? Although select is an expr so maybe fine? but it's not prefix or postfix so maybe right here is good?
        //
        if p.at_ts(SELECT_FIRST) && (select_stmt(p, None).is_none() || p.at(EOF) || p.at(R_PAREN)) {
            break;
        }
        if paren_select(p).is_none() {
            break;
        }
        if !p.at(R_PAREN) {
            break;
        }
    }
    p.expect(R_PAREN);
    Some(m.complete(p, PAREN_EXPR))
}

const SELECT_FIRST: TokenSet = TokenSet::new(&[SELECT_KW, TABLE_KW, WITH_KW, VALUES_KW]);

fn tuple_expr(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(L_PAREN) || p.at(ROW_KW));
    let m = p.start();
    p.eat(ROW_KW);
    p.expect(L_PAREN);
    let mut saw_comma = false;
    let mut saw_expr = false;
    if p.eat(COMMA) {
        p.error("expected expression in tuple_expr");
        saw_comma = true;
    }
    while !p.at(EOF) && !p.at(R_PAREN) {
        saw_expr = true;
        // we want to check for select stuff before we get the the expr stuff maybe? Although select is an expr so maybe fine? but it's not prefix or postfix so maybe right here is good?
        //
        if p.at_ts(SELECT_FIRST) && (select_stmt(p, None).is_none() || p.at(EOF) || p.at(R_PAREN)) {
            break;
        }
        if expr(p).is_none() {
            break;
        }
        if !p.at(R_PAREN) {
            saw_comma = true;
            p.expect(COMMA);
        }
    }
    p.expect(R_PAREN);
    let cm = m.complete(
        p,
        if saw_expr && !saw_comma {
            PAREN_EXPR
        } else {
            TUPLE_EXPR
        },
    );
    // TODO: needs to be reworked
    if p.at_ts(COMPOUND_SELECT_FIRST) {
        return compound_select(p, cm);
    }
    cm
}

// Define SQL-style CASE clause.
// - Full specification
// CASE WHEN a = b THEN c ... ELSE d END
// - Implicit argument
// CASE a WHEN b THEN c ... ELSE d END
//
// case_expr:  CASE case_arg when_clause_list case_default END_P
//
// when_clause_list:
//    when_clause
//  | when_clause_list when_clause
//
// when_clause:
//      WHEN a_expr THEN a_expr
fn case_expr(p: &mut Parser<'_>) -> CompletedMarker {
    let m = p.start();
    p.expect(CASE_KW);
    if !p.at(WHEN_KW) && expr(p).is_none() {
        p.error("expected an expression");
    }
    while !p.at(EOF) {
        when_clause(p);
        if !p.at(WHEN_KW) {
            break;
        }
    }
    // case_default
    //     | ELSE a_expr
    //     | /* empty */
    if p.eat(ELSE_KW) && expr(p).is_none() {
        p.error("expected an expression");
    }
    p.expect(END_KW);
    m.complete(p, CASE_EXPR)
}

// when_clause:
//      WHEN a_expr THEN a_expr
fn when_clause(p: &mut Parser<'_>) -> CompletedMarker {
    let m = p.start();
    p.expect(WHEN_KW);
    if expr(p).is_none() {
        p.error("expected an expression");
    }
    p.expect(THEN_KW);
    if expr(p).is_none() {
        p.error("expected an expression");
    }
    m.complete(p, WHEN_CLAUSE)
}

const EXTRACT_ARG_FIRST_: TokenSet =
    TokenSet::new(&[YEAR_KW, MONTH_KW, DAY_KW, HOUR_KW, MINUTE_KW, SECOND_KW]).union(STRING_FIRST);

// IDENT | YEAR_P | MONTH_P | DAY_P | HOUR_P | MINUTE_P | SECOND_P | Sconst
const EXTRACT_ARG_FIRST: TokenSet = IDENTS.union(EXTRACT_ARG_FIRST_);
fn extract_arg(p: &mut Parser<'_>) -> bool {
    if p.at_ts(EXTRACT_ARG_FIRST) {
        p.bump_any();
        true
    } else {
        p.error(format!(
            "expected ident, year, month, day, hour, minute, second, or string, got {:?}",
            p.current()
        ));
        false
    }
}

fn extract_fn(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(EXTRACT_KW));
    custom_fn(p, EXTRACT_KW, |p| {
        extract_arg(p);
        p.expect(FROM_KW);
        if expr(p).is_none() {
            p.error("expected an expression");
        }
    })
}

// | OVERLAY '(' overlay_list ')'
//     overlay_list:
//       | a_expr PLACING a_expr FROM a_expr FOR a_expr
//       | a_expr PLACING a_expr FROM a_expr
// | OVERLAY '(' func_arg_list_opt ')
fn overlay_fn(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(OVERLAY_KW));
    custom_fn(p, OVERLAY_KW, |p| {
        if p.at(R_PAREN) {
            return;
        }
        if expr(p).is_none() {
            p.error("expected an expression");
        }
        if p.eat(PLACING_KW) {
            if expr(p).is_none() {
                p.error("expected an expression");
            }
            p.expect(FROM_KW);
            if expr(p).is_none() {
                p.error("expected an expression");
            }
            if p.eat(FOR_KW) && expr(p).is_none() {
                p.error("expected an expression");
            }
        } else if p.eat(COMMA) {
            while !p.at(EOF) {
                if expr(p).is_none() {
                    p.error("expected an expression");
                }
                if !p.eat(COMMA) {
                    break;
                }
            }
        }
    })
}

// POSITION '(' position_list ')'
//  position_list:
//    b_expr IN_P b_expr
//
//  Presently, AND, NOT, IS, and IN are the a_expr keywords that would
//  cause trouble in the places where b_expr is used.
fn position_fn(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(POSITION_KW));
    fn b_expr(r: &mut Parser<'_>) -> Option<CompletedMarker> {
        expr_bp(
            r,
            1,
            &Restrictions {
                in_disabled: true,
                ..Restrictions::default()
            },
        )
    }
    custom_fn(p, POSITION_KW, |p| {
        if b_expr(p).is_none() {
            p.error("expected an expression");
        }
        p.expect(IN_KW);
        if b_expr(p).is_none() {
            p.error("expected an expression");
        }
    })
}

fn trim_fn(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(TRIM_KW));
    custom_fn(p, TRIM_KW, |p| {
        if p.eat(BOTH_KW) {
            if p.eat(FROM_KW) {
                if expr(p).is_none() {
                    p.error("expected an expression");
                }
            } else {
                if expr(p).is_none() {
                    p.error("expected an expression");
                }
                if p.eat(FROM_KW) && expr(p).is_none() {
                    p.error("expected an expression");
                }
            }
        } else if p.eat(LEADING_KW) || p.eat(TRAILING_KW) {
            if expr(p).is_none() {
                p.error("expected an expression");
            }
        } else if expr(p).is_none() {
            p.error("expected an expression");
        }
    })
}

// SUBSTRING '(' substr_list ')'
//   substr_list:
//     | a_expr FROM a_expr FOR a_expr
//     | a_expr FOR a_expr FROM a_expr
//     | a_expr FROM a_expr
//     | a_expr FOR a_expr
//     | a_expr SIMILAR a_expr ESCAPE a_expr
//
// SUBSTRING '(' func_arg_list_opt ')'
fn substring_fn(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(SUBSTRING_KW));
    custom_fn(p, SUBSTRING_KW, |p| {
        if expr(p).is_none() {
            p.error("expected an expression");
        }
        match p.current() {
            // FOR a_expr FROM a_expr
            // FOR a_expr
            FOR_KW => {
                p.bump(FOR_KW);
                if expr(p).is_none() {
                    p.error("expected an expression");
                }
                // [ from expr ]
                if p.eat(FROM_KW) && expr(p).is_none() {
                    p.error("expected an expression");
                }
            }
            // FROM a_expr
            // FROM a_expr FOR a_expr
            FROM_KW => {
                p.bump(FROM_KW);
                if expr(p).is_none() {
                    p.error("expected an expression");
                }
                // [ for expr ]
                if p.eat(FOR_KW) && expr(p).is_none() {
                    p.error("expected an expression");
                }
            }
            // SIMILAR a_expr ESCAPE a_expr
            SIMILAR_KW => {
                p.bump(SIMILAR_KW);
                if expr(p).is_none() {
                    p.error("expected an expression");
                }
                p.expect(ESCAPE_KW);
                if expr(p).is_none() {
                    p.error("expected an expression");
                }
            }
            _ if p.eat(COMMA) => {
                // normal function call
                while !p.at(EOF) {
                    if expr(p).is_none() {
                        p.error("expected an expression");
                    }
                    if !p.eat(COMMA) {
                        break;
                    }
                }
            }
            _ => {}
        }
    })
}

fn opt_json_encoding_clause(p: &mut Parser<'_>) {
    if p.eat(ENCODING_KW) {
        name_ref(p);
    }
}

// json_format_clause_opt:
//   json_format_clause
//   | /* EMPTY */
//
//   json_format_clause:
//     FORMAT_LA JSON ENCODING name
//     | FORMAT_LA JSON
fn opt_json_format_clause(p: &mut Parser<'_>) -> Option<CompletedMarker> {
    let m = p.start();
    if p.eat(FORMAT_KW) {
        p.expect(JSON_KW);
        opt_json_encoding_clause(p);
        Some(m.complete(p, JSON_FORMAT_CLAUSE))
    } else {
        m.abandon(p);
        None
    }
}

// json_returning_clause_opt:
//   RETURNING Typename json_format_clause_opt
//   | /* EMPTY */
fn opt_json_returning_clause(p: &mut Parser<'_>) -> Option<CompletedMarker> {
    let m = p.start();
    if p.eat(RETURNING_KW) {
        type_name(p);
        opt_json_format_clause(p);
        Some(m.complete(p, JSON_RETURNING_CLAUSE))
    } else {
        m.abandon(p);
        None
    }
}

// json_object_constructor_null_clause_opt:
//   | NULL_P ON NULL_P
//   | ABSENT ON NULL_P
//   | /* EMPTY */
fn opt_json_null_clause(p: &mut Parser<'_>) {
    let m = p.start();
    if p.at(NULL_KW) || p.at(ABSENT_KW) {
        p.bump_any();
        p.expect(ON_KW);
        p.expect(NULL_KW);
        m.complete(p, JSON_NULL_CLAUSE);
    } else {
        m.abandon(p);
    }
}

// json_key_uniqueness_constraint_opt:
//   | WITH UNIQUE KEYS
//   | WITH UNIQUE
//   | WITHOUT UNIQUE KEYS
//   | WITHOUT UNIQUE
//   | /* EMPTY */
fn opt_json_key_unique_constraint(p: &mut Parser<'_>) {
    if p.at(WITH_KW) || p.at(WITHOUT_KW) {
        let m = p.start();
        p.bump_any();
        p.expect(UNIQUE_KW);
        p.eat(KEYS_KW);
        m.complete(p, JSON_KEYS_UNIQUE_CLAUSE);
    }
}

// json_object( func_arg_list )
//   func_arg_list:  func_arg_expr
//     | func_arg_list ',' func_arg_expr
//
//     func_arg_expr:  a_expr
//       | param_name COLON_EQUALS a_expr
//       | param_name EQUALS_GREATER a_expr
//
//       param_name:
//         | type_function_name
//
// json_object(
//   json_name_and_value_list
//   json_object_constructor_null_clause_opt
//   json_key_uniqueness_constraint_opt
//   json_returning_clause_opt
// )
//   json_name_and_value_list:
//     | json_name_and_value
//     | json_name_and_value_list ',' json_name_and_value
//
// json_object( json_returning_clause_opt )
fn json_object_fn_arg_list(p: &mut Parser<'_>) {
    // json_object()
    if p.at(R_PAREN) {
        return;
    }
    // json_object(RETURNING Typename json_format_clause_opt)
    if p.at(RETURNING_KW) {
        opt_json_returning_clause(p);
        return;
    }
    while !p.at(EOF) && !p.at(R_PAREN) {
        // TODO: I think we need to be more strict here
        // json_object(c_expr ,
        // json_object(a_expr :
        // json_object(a_expr value
        if json_object_arg(p).is_none() {
            p.error("expected expression");
        }
        // if we're at a the end of the params or the start of the optional
        // null_clause break
        if p.at(R_PAREN)
            || p.at(NULL_KW)
            || p.at(ABSENT_KW)
            || p.at(WITH_KW)
            || p.at(WITHOUT_KW)
            || p.at(RETURNING_KW)
        {
            break;
        } else if p.at(COMMA) {
            // we're in a function arg
            //
            // we can't have trailing commas
            if p.nth_at(1, R_PAREN) {
                p.error("unexpected closing comma");
            }
            p.bump(COMMA);
            continue;
        } else {
            p.error("expected a comma");
        }
    }
    opt_json_null_clause(p);
    opt_json_key_unique_constraint(p);
    opt_json_returning_clause(p);
}

fn json_object_fn(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(JSON_OBJECT_KW));
    custom_fn(p, JSON_OBJECT_KW, |p| {
        json_object_fn_arg_list(p);
    })
}

/// <https://www.postgresql.org/docs/17/functions-json.html#FUNCTIONS-SQLJSON-TABLE>
fn json_table_fn(p: &mut Parser<'_>) -> CompletedMarker {
    custom_fn(p, JSON_TABLE_KW, |p| {
        json_table_arg_list(p);
    })
}

fn custom_fn(
    p: &mut Parser<'_>,
    kind: SyntaxKind,
    mut body: impl FnMut(&mut Parser<'_>),
) -> CompletedMarker {
    assert!(p.at(kind));
    let m = p.start();
    let name_ref = p.start();
    p.expect(kind);
    name_ref.complete(p, NAME_REF);
    let args = p.start();
    p.expect(L_PAREN);
    body(p);
    p.expect(R_PAREN);
    args.complete(p, ARG_LIST);
    m.complete(p, CALL_EXPR)
}

// JSON_TABLE (
//     context_item, path_expression [ AS json_path_name ] [ PASSING { value AS varname } [, ...] ]
//     COLUMNS ( json_table_column [, ...] )
//     [ { ERROR | EMPTY [ARRAY]} ON ERROR ]
// )
fn json_table_arg_list(p: &mut Parser<'_>) {
    // context_item
    if expr(p).is_none() {
        p.error("expected expression");
    }
    opt_json_format_clause(p);
    p.expect(COMMA);
    // path_expression
    if expr(p).is_none() {
        p.error("expected expression");
    }
    // [ AS json_path_name ]
    if p.eat(AS_KW) {
        name(p);
    }
    // [ PASSING { value AS varname } [, ...] ]
    if p.eat(PASSING_KW) {
        while !p.at(EOF) {
            // value
            if expr(p).is_none() {
                p.error("expected expression");
            }
            opt_json_format_clause(p);
            p.expect(AS_KW);
            col_label(p);
            if !p.eat(COMMA) {
                break;
            }
        }
    }
    // COLUMNS ( json_table_column [, ...] )
    if p.eat(COLUMNS_KW) {
        p.expect(L_PAREN);
        while !p.at(EOF) {
            json_table_column(p);
            if !p.eat(COMMA) {
                break;
            }
        }
        p.expect(R_PAREN);
    }
    // [ { ERROR | EMPTY [ARRAY]} ON ERROR ]
    if p.eat(ERROR_KW) {
        p.expect(ON_KW);
        p.expect(ERROR_KW);
    } else if p.eat(EMPTY_KW) {
        p.eat(ARRAY_KW);
        p.expect(ON_KW);
        p.expect(ERROR_KW);
    }
}

// where json_table_column is:
//   | name FOR ORDINALITY
//   | name type
//         [ FORMAT JSON [ENCODING UTF8]]
//         [ PATH path_expression ]
//         [ { WITHOUT | WITH { CONDITIONAL | [UNCONDITIONAL] } } [ ARRAY ] WRAPPER ]
//         [ { KEEP | OMIT } QUOTES [ ON SCALAR STRING ] ]
//         [ { ERROR | NULL | EMPTY { [ARRAY] | OBJECT } | DEFAULT expression } ON EMPTY ]
//         [ { ERROR | NULL | EMPTY { [ARRAY] | OBJECT } | DEFAULT expression } ON ERROR ]
//   | name type EXISTS [ PATH path_expression ]
//         [ { ERROR | TRUE | FALSE | UNKNOWN } ON ERROR ]
//   | NESTED [ PATH ] path_expression [ AS json_path_name ] COLUMNS ( json_table_column [, ...] )
fn json_table_column(p: &mut Parser<'_>) {
    // NESTED [ PATH ] path_expression [ AS json_path_name ] COLUMNS ( json_table_column [, ...] )
    if p.eat(NESTED_KW) {
        p.eat(PATH_KW);
        // path_expression
        if expr(p).is_none() {
            p.error("expected expression");
        }
        // [ AS json_path_name ]
        if p.eat(AS_KW) {
            name(p);
        }
        p.expect(COLUMNS_KW);
        p.expect(L_PAREN);
        while !p.at(EOF) {
            json_table_column(p);
            if !p.eat(COMMA) {
                break;
            }
        }
        p.expect(R_PAREN);
    } else {
        name(p);
        // FOR ORDINALITY
        if p.eat(FOR_KW) {
            p.expect(ORDINALITY_KW);
        } else {
            type_name(p);
            // name type EXISTS [ PATH path_expression ]
            if p.eat(EXISTS_KW) {
                // [ PATH path_expression ]
                if p.eat(PATH_KW) {
                    // path_expression
                    if expr(p).is_none() {
                        p.error("expected expression");
                    }
                }
                // [ { ERROR | TRUE | FALSE | UNKNOWN } ON ERROR ]
                if p.at(ERROR_KW) || p.at(TRUE_KW) || p.at(FALSE_KW) || p.at(UNKNOWN_KW) {
                    p.expect(ON_KW);
                    p.expect(ERROR_KW);
                }
            } else {
                // [ FORMAT JSON [ENCODING UTF8]]
                opt_json_format_clause(p);
                // [ PATH path_expression ]
                if p.eat(PATH_KW) {
                    // path_expression
                    if expr(p).is_none() {
                        p.error("expected expression");
                    }
                }
                // [ { WITHOUT | WITH { CONDITIONAL | [UNCONDITIONAL] } } [ ARRAY ] WRAPPER ]
                if p.at(WITHOUT_KW) || p.at(WITH_KW) {
                    if p.eat(WITH_KW) {
                        let _ = p.eat(CONDITIONAL_KW) || p.eat(UNCONDITIONAL_KW);
                    } else {
                        p.bump(WITHOUT_KW);
                    }
                    p.eat(ARRAY_KW);
                    p.expect(WRAPPER_KW);
                }
                // [ { KEEP | OMIT } QUOTES [ ON SCALAR STRING ] ]
                if p.eat(KEEP_KW) || p.eat(OMIT_KW) {
                    p.expect(QUOTES_KW);
                    if p.eat(ON_KW) {
                        p.expect(SCALAR_KW);
                        p.expect(STRING_KW);
                    }
                }
                // [ { ERROR | NULL | EMPTY { [ARRAY] | OBJECT } | DEFAULT expression } ON EMPTY ]
                // [ { ERROR | NULL | EMPTY { [ARRAY] | OBJECT } | DEFAULT expression } ON ERROR ]
                if p.at(ERROR_KW) || p.at(NULL_KW) || p.at(EMPTY_KW) || p.at(DEFAULT_KW) {
                    // EMPTY { [ARRAY] | OBJECT }
                    if p.eat(EMPTY_KW) {
                        let _ = p.eat(ARRAY_KW) || p.expect(OBJECT_KW);
                    // DEFAULT
                    } else if p.eat(DEFAULT_KW) {
                        if expr(p).is_none() {
                            p.error("expected an expression");
                        }
                    // ERROR | NULL
                    } else {
                        p.bump_any();
                    }
                    p.expect(ON_KW);
                    let _ = p.eat(EMPTY_KW) || p.expect(ERROR_KW);
                }
            }
        }
    }
}

// json_array (
//  [ { value_expression [ FORMAT JSON ] } [, ...] ]
//  [ { NULL | ABSENT } ON NULL ]
//  [ RETURNING data_type [ FORMAT JSON [ ENCODING UTF8 ] ] ]
// )
// json_array (
//  [ query_expression ]
//  [ RETURNING data_type [ FORMAT JSON [ ENCODING UTF8 ] ] ]
// )
fn json_array_fn_arg_list(p: &mut Parser<'_>) {
    // ()
    if p.at(R_PAREN) {
        return;
    }
    // 1, 2, 3, 4
    while !p.at(EOF) && !p.at(R_PAREN) && !p.at(RETURNING_KW) {
        if p.at_ts(SELECT_FIRST) {
            if select_stmt(p, None).is_none() || p.at(EOF) || p.at(R_PAREN) {
                break;
            }
            opt_json_format_clause(p);
        } else {
            if expr(p).is_none() {
                p.error("expected expression");
            }
            opt_json_format_clause(p);
        }
        if !p.eat(COMMA) {
            break;
        }
    }
    opt_json_null_clause(p);
    // (RETURNING Typename json_format_clause_opt)
    if opt_json_returning_clause(p).is_none() && opt_json_format_clause(p).is_none() {
        opt_json_encoding_clause(p);
    }
}

fn json_array_fn(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(JSON_ARRAY_KW));
    custom_fn(p, JSON_ARRAY_KW, |p| {
        json_array_fn_arg_list(p);
    })
}

/// <https://www.postgresql.org/docs/17/functions-comparisons.html#FUNCTIONS-COMPARISONS-ANY-SOME>
fn some_any_all_fn(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(SOME_KW) || p.at(ANY_KW) || p.at(ALL_KW));
    let m = p.start();
    // TODO: this can only be in the conext of a binary expression, so we should
    // have some validation for that.
    let m1 = p.start();
    // SOME | ANY | ALL
    p.bump_any();
    m1.complete(p, NAME_REF);
    // args
    p.expect(L_PAREN);
    if p.at_ts(SELECT_FIRST) {
        select_stmt(p, None);
    } else {
        if expr(p).is_none() {
            p.error("expected expression or select");
        }
    }
    p.expect(R_PAREN);
    m.complete(p, CALL_EXPR)
}

// literal, path, tuple, array
fn atom_expr(p: &mut Parser<'_>) -> Option<CompletedMarker> {
    if let Some(m) = literal(p) {
        return Some(m);
    }
    let done = match (p.current(), p.nth(1)) {
        (PARAM, _) => {
            let m = p.start();
            p.bump(PARAM);
            m.complete(p, LITERAL)
        }
        (VALUES_KW, _) => values_clause(p, None),
        (EXTRACT_KW, L_PAREN) => extract_fn(p),
        (JSON_EXISTS_KW, L_PAREN) => json_exists_fn(p),
        (JSON_ARRAY_KW, L_PAREN) => json_array_fn(p),
        (JSON_OBJECT_KW, L_PAREN) => json_object_fn(p),
        (JSON_QUERY_KW, L_PAREN) => json_query_fn(p),
        (JSON_SERIALIZE_KW, L_PAREN) => json_serialize_fn(p),
        (JSON_VALUE_KW, L_PAREN) => json_value_fn(p),
        (JSON_KW, L_PAREN) => json_fn(p),
        (SUBSTRING_KW, L_PAREN) => substring_fn(p),
        (POSITION_KW, L_PAREN) => position_fn(p),
        (OVERLAY_KW, L_PAREN) => overlay_fn(p),
        (TRIM_KW, L_PAREN) => trim_fn(p),
        (XMLROOT_KW, L_PAREN) => xmlroot_fn(p),
        (XMLSERIALIZE_KW, L_PAREN) => xmlserialize_fn(p),
        (XMLELEMENT_KW, L_PAREN) => xmlelement_fn(p),
        (XMLEXISTS_KW, L_PAREN) => xmlexists_fn(p),
        (XMLPARSE_KW, L_PAREN) => xmlparse_fn(p),
        (XMLPI_KW, L_PAREN) => xmlpi_fn(p),
        (SOME_KW | ALL_KW | ANY_KW, L_PAREN) => some_any_all_fn(p),
        (EXISTS_KW, L_PAREN) => exists_fn(p),
        _ if p.at_ts(NAME_REF_FIRST) => name_ref_(p)?,
        (L_PAREN, _) => tuple_expr(p),
        (ARRAY_KW, L_BRACK | L_PAREN) => {
            let m = p.start();
            p.bump(ARRAY_KW);
            array_expr(p, Some(m))
        }
        // nested array exprs:
        // array[[1,2],[3,4]]
        (L_BRACK, _) => array_expr(p, None),
        (ROW_KW, L_PAREN) => tuple_expr(p),
        (CASE_KW, _) => case_expr(p),
        _ => {
            p.err_and_bump("expected expression in atom_expr");
            return None;
        }
    };
    Some(done)
}

fn exists_fn(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(EXISTS_KW));
    custom_fn(p, EXISTS_KW, |p| {
        if p.at_ts(SELECT_FIRST) {
            select_stmt(p, None);
        } else {
            p.error("expected select")
        }
    })
}

// XMLPI '(' NAME_P ColLabel ',' a_expr ')'
// XMLPI '(' NAME_P ColLabel ')'
fn xmlpi_fn(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(XMLPI_KW));
    custom_fn(p, XMLPI_KW, |p| {
        p.expect(NAME_KW);
        col_label(p);
        if p.eat(COMMA) && expr(p).is_none() {
            p.error("expected expr");
        }
    })
}

// XMLPARSE '(' document_or_content a_expr xml_whitespace_option ')'
//   document_or_content:
//     | DOCUMENT_P
//     | CONTENT_P
//   xml_whitespace_option:
//     | PRESERVE WHITESPACE_P
//     | STRIP_P WHITESPACE_P
//     | /*EMPTY*/
fn xmlparse_fn(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(XMLPARSE_KW));
    custom_fn(p, XMLPARSE_KW, |p| {
        if p.at(DOCUMENT_KW) || p.at(CONTENT_KW) {
            p.bump_any();
        } else {
            p.error("expected DOCUMENT or CONTENT");
        }
        if expr(p).is_none() {
            p.error("expected expression");
        }
        if p.eat(PRESERVE_KW) || p.eat(STRIP_KW) {
            p.expect(WHITESPACE_KW);
        }
    })
}

fn opt_xml_passing_mech(p: &mut Parser<'_>) -> bool {
    if p.eat(BY_KW) {
        if !p.eat(REF_KW) && !p.eat(VALUE_KW) {
            p.error("expected REF or VALUE");
        }
        true
    } else {
        false
    }
}

// XMLEXISTS '(' c_expr xmlexists_argument ')'
//   xmlexists_argument:
//     | PASSING c_expr
//     | PASSING c_expr xml_passing_mech
//     | PASSING xml_passing_mech c_expr
//     | PASSING xml_passing_mech c_expr xml_passing_mech
//     xml_passing_mech:
//       | BY REF_P
//       | BY VALUE_P
fn xmlexists_fn(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(XMLEXISTS_KW));
    custom_fn(p, XMLEXISTS_KW, |p| {
        if expr(p).is_none() {
            p.error("expected expression");
        }
        p.expect(PASSING_KW);
        opt_xml_passing_mech(p);
        if expr(p).is_none() {
            p.error("expected expression");
        }
        opt_xml_passing_mech(p);
    })
}

// XMLELEMENT '(' NAME_P ColLabel ',' xml_attributes ',' expr_list ')'
// XMLELEMENT '(' NAME_P ColLabel ',' xml_attributes ')'
// XMLELEMENT '(' NAME_P ColLabe ',' expr_list ')'
// XMLELEMENT '(' NAME_P ColLabel ')'
//  xml_attributes:
//    XMLATTRIBUTES '(' xml_attribute_list ')'
//  xml_attribute_list:
//    | xml_attribute_el
//    | xml_attribute_list ',' xml_attribute_el
//    xml_attribute_el:
//      | a_expr AS ColLabel
//      | a_expr
fn xmlelement_fn(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(XMLELEMENT_KW));
    custom_fn(p, XMLELEMENT_KW, |p| {
        p.expect(NAME_KW);
        col_label(p);
        if p.eat(COMMA) {
            if p.eat(XMLATTRIBUTES_KW) {
                p.expect(L_PAREN);
                while !p.at(EOF) && !p.at(R_PAREN) {
                    if expr(p).is_none() {
                        p.error("expected expression");
                    }
                    if p.eat(AS_KW) {
                        col_label(p);
                    }
                    if !p.eat(COMMA) {
                        break;
                    }
                }
                p.expect(R_PAREN);
                if p.eat(COMMA) && !expr_list(p) {
                    p.error("expected expression list");
                }
            } else if !expr_list(p) {
                p.error("expected expression list");
            }
        }
    })
}

// XMLSERIALIZE '(' document_or_content a_expr AS SimpleTypename xml_indent_option ')'
// xml_indent_option:
//   | INDENT
//   | NO INDENT
//   | /*EMPTY*/
fn xmlserialize_fn(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(XMLSERIALIZE_KW));
    custom_fn(p, XMLSERIALIZE_KW, |p| {
        if p.at(DOCUMENT_KW) || p.at(CONTENT_KW) {
            p.bump_any();
        } else {
            p.error("expected DOCUMENT or CONTENT");
        }
        if expr(p).is_none() {
            p.error("expected expression");
        }
        p.expect(AS_KW);
        simple_type_name(p);
        if p.eat(NO_KW) {
            p.expect(INDENT_KW);
        } else {
            p.eat(INDENT_KW);
        }
    })
}

// XMLROOT '(' a_expr ',' xml_root_version opt_xml_root_standalone ')'
//   xml_root_version:
//     | VERSION_P a_expr
//     | VERSION_P NO VALUE_P
//   opt_xml_root_standalone:
//     | ',' STANDALONE_P YES_P
//     | ',' STANDALONE_P NO
//     | ',' STANDALONE_P NO VALUE_P
//     | /*EMPTY*/
fn xmlroot_fn(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(XMLROOT_KW));
    custom_fn(p, XMLROOT_KW, |p| {
        if expr(p).is_none() {
            p.error("expected expression");
        }
        p.expect(COMMA);
        p.expect(VERSION_KW);
        if p.eat(NO_KW) {
            p.expect(VALUE_KW);
        } else if expr(p).is_none() {
            p.error("expected expression");
        }
        if p.eat(COMMA) {
            p.expect(STANDALONE_KW);
            if p.eat(NO_KW) {
                p.eat(VALUE_KW);
            } else {
                p.expect(YES_KW);
            }
        }
    })
}

// JSON '(' json_value_expr json_key_uniqueness_constraint_opt ')'
fn json_fn(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(JSON_KW));
    custom_fn(p, JSON_KW, |p| {
        // json_value_expr
        if expr(p).is_none() {
            p.error("expected expression");
        }
        opt_json_format_clause(p);
        opt_json_key_unique_constraint(p);
    })
}

// JSON_VALUE '('
//   json_value_expr ',' a_expr json_passing_clause_opt
//   json_returning_clause_opt
//   json_behavior_clause_opt
// ')'
fn json_value_fn(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(JSON_VALUE_KW));
    custom_fn(p, JSON_VALUE_KW, |p| {
        // json_value_expr
        if expr(p).is_none() {
            p.error("expected expression");
        }
        opt_json_format_clause(p);
        p.expect(COMMA);
        if expr(p).is_none() {
            p.error("expected expression");
        }
        opt_json_passing_clause(p);
        opt_json_returning_clause(p);
        opt_json_behavior_clause(p);
    })
}

// JSON_SERIALIZE '(' json_value_expr json_returning_clause_opt ')'
fn json_serialize_fn(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(JSON_SERIALIZE_KW));
    custom_fn(p, JSON_SERIALIZE_KW, |p| {
        if expr(p).is_none() {
            p.error("expected expression");
        }
        opt_json_format_clause(p);
        opt_json_returning_clause(p);
    })
}

// JSON_QUERY (
//   context_item, path_expression
//   [ PASSING { value AS varname } [, ...]]
//   [ RETURNING data_type [ FORMAT JSON [ ENCODING UTF8 ] ] ]
//   [ { WITHOUT | WITH { CONDITIONAL | [UNCONDITIONAL] } } [ ARRAY ] WRAPPER ]
//   [ { KEEP | OMIT } QUOTES [ ON SCALAR STRING ] ]
//   [ { ERROR | NULL | EMPTY { [ ARRAY ] | OBJECT } | DEFAULT expression } ON EMPTY ]
//   [ { ERROR | NULL | EMPTY { [ ARRAY ] | OBJECT } | DEFAULT expression } ON ERROR ]
// )
fn json_query_fn(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(JSON_QUERY_KW));
    custom_fn(p, JSON_QUERY_KW, |p| {
        // context_item
        if expr(p).is_none() {
            p.error("expected expression");
        }
        opt_json_format_clause(p);
        p.expect(COMMA);
        // path_expression
        if expr(p).is_none() {
            p.error("expected expression");
        }
        opt_json_passing_clause(p);
        opt_json_returning_clause(p);
        opt_json_wrapper_behavior(p);
        opt_json_quotes_clause(p);
        opt_json_behavior_clause(p);
    })
}

fn opt_json_quotes_clause(p: &mut Parser<'_>) -> Option<CompletedMarker> {
    let m = p.start();
    if p.eat(KEEP_KW) || p.eat(OMIT_KW) {
        p.expect(QUOTES_KW);
        if p.eat(ON_KW) {
            p.expect(SCALAR_KW);
            p.expect(STRING_KW);
        }
        Some(m.complete(p, JSON_QUOTES_CLAUSE))
    } else {
        m.abandon(p);
        None
    }
}

fn opt_json_behavior_clause(p: &mut Parser<'_>) -> Option<CompletedMarker> {
    let m = p.start();
    if opt_json_behavior(p) {
        p.expect(ON_KW);
        if !p.eat(ERROR_KW) {
            p.expect(EMPTY_KW);
            if !opt_json_behavior(p) {
                p.error("expected json behavior");
            }
            p.expect(ON_KW);
            p.expect(ERROR_KW);
        }
        Some(m.complete(p, JSON_BEHAVIOR_CLAUSE))
    } else {
        m.abandon(p);
        None
    }
}

fn opt_json_wrapper_behavior(p: &mut Parser<'_>) -> Option<CompletedMarker> {
    let m = p.start();
    match (p.current(), p.nth(1)) {
        // WITHOUT WRAPPER
        // WITHOUT ARRAY
        // WITH WRAPPER
        (WITHOUT_KW, WRAPPER_KW) | (WITH_KW, WRAPPER_KW) | (WITHOUT_KW, ARRAY_KW) => {
            p.bump_any();
            p.bump_any();
        }
        // WITH ARRAY WRAPPER
        (WITH_KW, ARRAY_KW) => {
            p.bump_any();
            p.bump_any();
            p.expect(WRAPPER_KW);
        }
        // WITH UNCONDITIONAL ARRAY WRAPPER
        // WITH UNCONDITIONAL WRAPPER
        // WITH CONDITIONAL ARRAY WRAPPER
        // WITH CONDITIONAL WRAPPER
        (WITH_KW, UNCONDITIONAL_KW) | (WITH_KW, CONDITIONAL_KW) => {
            p.bump_any();
            p.bump_any();
            p.eat(ARRAY_KW);
            p.expect(WRAPPER_KW);
        }
        _ => {
            m.abandon(p);
            return None;
        }
    }
    Some(m.complete(p, JSON_WRAPPER_BEHAVIOR_CLAUSE))
}

// json_exists (
//   context_item,
//   path_expression
//   [ PASSING { value AS varname } [, ...]]
//   [{ TRUE | FALSE | UNKNOWN | ERROR } ON ERROR ]
// )
fn json_exists_fn(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(JSON_EXISTS_KW));
    custom_fn(p, JSON_EXISTS_KW, |p| {
        if expr(p).is_none() {
            p.error("expected expression");
        }
        opt_json_format_clause(p);
        p.expect(COMMA);
        if expr(p).is_none() {
            p.error("expected expression");
        }
        opt_json_passing_clause(p);
        opt_json_on_error_clause(p);
    })
}

fn opt_json_on_error_clause(p: &mut Parser<'_>) {
    let m = p.start();
    if opt_json_behavior(p) {
        p.expect(ON_KW);
        p.expect(ERROR_KW);
        m.complete(p, JSON_ON_ERROR_CLAUSE);
    } else {
        m.abandon(p);
    }
}

fn opt_json_behavior(p: &mut Parser<'_>) -> bool {
    match p.current() {
        DEFAULT_KW => {
            p.bump(DEFAULT_KW);
            if expr(p).is_none() {
                p.error("expected expression");
            }
        }
        ERROR_KW | NULL_KW | TRUE_KW | FALSE_KW | UNKNOWN_KW => {
            p.bump_any();
        }
        EMPTY_KW => {
            p.bump(EMPTY_KW);
            let _ = p.eat(ARRAY_KW) || p.eat(OBJECT_KW);
        }
        _ => return false,
    }
    true
}

fn json_args(p: &mut Parser<'_>) {
    while !p.at(EOF) {
        if expr(p).is_none() {
            p.error("expected expr");
        }
        opt_json_format_clause(p);
        p.expect(AS_KW);
        col_label(p);
        if !p.eat(COMMA) {
            break;
        }
    }
}

fn opt_json_passing_clause(p: &mut Parser<'_>) {
    let m = p.start();
    if p.eat(PASSING_KW) {
        json_args(p);
        m.complete(p, JSON_PASSING_CLAUSE);
    } else {
        m.abandon(p);
    }
}

// unary / prefix stuff
fn lhs(p: &mut Parser<'_>, r: &Restrictions) -> Option<CompletedMarker> {
    let m;
    let (kind, prefix_bp) = match p.current() {
        MINUS | PLUS => {
            m = p.start();
            p.bump_any();
            (PREFIX_EXPR, 13)
        }
        _ if p.at_ts(OPERATOR_FIRST) && p.at(CUSTOM_OP) => {
            m = p.start();
            p.bump(CUSTOM_OP);
            (PREFIX_EXPR, 7)
        }
        NOT_KW if !r.not_disabled => {
            m = p.start();
            p.bump_any();
            (PREFIX_EXPR, 3)
        }
        CAST_KW | TREAT_KW => {
            m = p.start();
            p.bump_any();
            p.expect(L_PAREN);
            if expr(p).is_none() {
                p.error("expected an expression");
            }
            p.expect(AS_KW);
            type_name(p);
            p.expect(R_PAREN);
            let cm = m.complete(p, CAST_EXPR);
            return Some(cm);
        }
        OPERATOR_KW if p.at(OPERATOR_CALL) => {
            m = p.start();
            p.expect(OPERATOR_CALL);
            (PREFIX_EXPR, 7)
        }
        _ => {
            let lhs = atom_expr(p)?;
            let cm = postfix_expr(p, lhs, true);
            return Some(cm);
        }
    };
    // parse the interior of the unary expression
    let _ = expr_bp(p, prefix_bp, &Restrictions::default());
    let cm = m.complete(p, kind);
    Some(cm)
}

fn postfix_expr(
    p: &mut Parser<'_>,
    mut lhs: CompletedMarker,
    allow_calls: bool,
) -> CompletedMarker {
    loop {
        lhs = match p.current() {
            | NOT_KW if p.nth_at(1, BETWEEN_KW) => between_expr(p, lhs),
            | BETWEEN_KW => between_expr(p, lhs),
            L_PAREN if allow_calls => call_expr_args(p, lhs),
            L_BRACK /* if allow_calls */ => index_expr(p, lhs),
            DOT => match postfix_dot_expr::<false>(p, lhs, allow_calls) {
                Ok(it) => it,
                Err(it) => {
                    lhs = it;
                    break;
                }
            },
            AT_KW if p.nth_at(1, LOCAL_KW) => {
                let m = lhs.precede(p);
                p.bump(AT_KW);
                p.bump(LOCAL_KW);
                lhs = m.complete(p, POSTFIX_EXPR);
                break;
            }
            ISNULL_KW => {
                let m = lhs.precede(p);
                p.bump(ISNULL_KW);
                lhs = m.complete(p, POSTFIX_EXPR);
                break;
            }
            _ => break,
        };
    }
    lhs
}

/// The `parser` passed this is required to at least consume one token if it returns `true`.
/// If the `parser` returns false, parsing will stop.
fn delimited(
    p: &mut Parser<'_>,
    bra: SyntaxKind,
    ket: SyntaxKind,
    delim: SyntaxKind,
    unexpected_delim_message: impl Fn() -> String,
    first_set: TokenSet,
    mut parser: impl FnMut(&mut Parser<'_>) -> bool,
) {
    p.bump(bra);
    while !p.at(ket) && !p.at(EOF) {
        if p.at(delim) {
            // Recover if an argument is missing and only got a delimiter,
            // e.g. `(a, , b)`.
            // Wrap the erroneous delimiter in an error node so that fixup logic gets rid of it.
            // FIXME: Ideally this should be handled in fixup in a structured way, but our list
            // nodes currently have no concept of a missing node between two delimiters.
            // So doing it this way is easier.
            let m = p.start();
            p.error(unexpected_delim_message());
            p.bump(delim);
            m.complete(p, ERROR);
            continue;
        }
        if !parser(p) {
            break;
        }
        // f(a,)
        //    ^
        if p.at(delim) && p.nth_at(1, ket) {
            p.err_and_bump("unexpected trailing comma");
            break;
        }
        if !p.eat(delim) {
            if p.at_ts(first_set) {
                p.error(format!("expected {delim:?}"));
            } else {
                break;
            }
        }
    }
    p.expect(ket);
}

fn name_ref(p: &mut Parser<'_>) -> Option<CompletedMarker> {
    opt_name_ref(p).or_else(|| {
        p.error("expected name");
        None
    })
}

fn opt_name_ref(p: &mut Parser<'_>) -> Option<CompletedMarker> {
    name_ref_(p)
}

fn name(p: &mut Parser<'_>) {
    if opt_name(p).is_none() {
        p.error("expected name");
    }
}

fn opt_name(p: &mut Parser<'_>) -> Option<CompletedMarker> {
    if !p.at_ts(NAME_FIRST) {
        return None;
    }
    let m = p.start();
    p.bump_any();
    Some(m.complete(p, NAME))
}

/// create type a . b as ();
///             ^ ^ ^ then name_ref
///             |   |
///                 | ^ then name
///                 |
fn path_segment(p: &mut Parser<'_>, kind: SyntaxKind) {
    let m = p.start();
    // TODO: does this need to be flagged?
    if current_operator(p).is_some() {
        // skip
    } else if p.at_ts(COL_LABEL_FIRST) {
        let m = p.start();
        p.bump_any();
        let kind = if p.at(DOT) { NAME_REF } else { kind };
        m.complete(p, kind);
    } else {
        p.error(format!("expected name, got {:?}", p.current()));
        m.abandon(p);
        return;
    }
    m.complete(p, PATH_SEGMENT);
}

fn opt_path(p: &mut Parser<'_>, kind: SyntaxKind) -> Option<CompletedMarker> {
    if !p.at_ts(COL_LABEL_FIRST) {
        return None;
    }
    let m = p.start();
    path_segment(p, kind);
    let qual = m.complete(p, PATH);
    Some(path_for_qualifier(p, qual, kind))
}

fn path_name(p: &mut Parser<'_>) {
    if opt_path(p, NAME).is_none() {
        p.error("expected path name");
    }
}

fn opt_path_name_ref(p: &mut Parser<'_>) -> Option<CompletedMarker> {
    opt_path(p, NAME_REF)
}

fn path_name_ref(p: &mut Parser<'_>) {
    if opt_path_name_ref(p).is_none() {
        p.error("expected path name");
    }
}

fn path_for_qualifier(
    p: &mut Parser<'_>,
    mut qual: CompletedMarker,
    kind: SyntaxKind,
) -> CompletedMarker {
    loop {
        if p.at(DOT) {
            let path = qual.precede(p);
            p.bump(DOT);
            path_segment(p, kind);
            let path = path.complete(p, PATH);
            qual = path;
        } else {
            return qual;
        }
    }
}

fn opt_percent_type(p: &mut Parser<'_>) -> Option<CompletedMarker> {
    let m = p.start();
    if p.eat(PERCENT) {
        p.expect(TYPE_KW);
        Some(m.complete(p, PERCENT_TYPE_CLAUSE))
    } else {
        m.abandon(p);
        None
    }
}

fn opt_array_index(p: &mut Parser<'_>) -> bool {
    if p.eat(L_BRACK) {
        if !p.at(R_BRACK) {
            let _ = expr(p);
        }
        p.expect(R_BRACK);
        true
    } else {
        false
    }
}

fn type_mods(
    p: &mut Parser<'_>,
    m: Marker,
    type_args_enabled: bool,
    kind: SyntaxKind,
) -> Option<CompletedMarker> {
    if opt_percent_type(p).is_some() {
        return Some(m.complete(p, PERCENT_TYPE));
    }
    if p.at(L_PAREN) && type_args_enabled {
        p.bump(L_PAREN);
        let type_args = p.start();
        while !p.at(EOF) && !p.at(R_PAREN) {
            let arg = p.start();
            if expr(p).is_none() {
                arg.abandon(p);
                break;
            }
            arg.complete(p, ARG);
            if !p.eat(COMMA) {
                break;
            }
        }
        p.expect(R_PAREN);
        type_args.complete(p, ARG_LIST);
    }
    let cm = m.complete(p, kind);
    if !p.at(L_BRACK) && !p.at(ARRAY_KW) {
        return Some(cm);
    }
    let m = cm.precede(p);
    // TODO: we should probably mimic the INDEX_EXPR but be stricter
    if p.eat(ARRAY_KW) {
        opt_array_index(p);
    } else {
        let mut found_one_array_bracks = false;
        while !p.at(EOF) && p.at(L_BRACK) {
            if opt_array_index(p) {
                found_one_array_bracks = true;
            }
        }
        if !found_one_array_bracks {
            p.error("expected L_BRACK for ARRAY_TYPE");
        }
    }
    Some(m.complete(p, ARRAY_TYPE))
}

fn char_type(p: &mut Parser<'_>) -> SyntaxKind {
    assert!(p.at(CHARACTER_KW) || p.at(CHAR_KW) || p.at(NCHAR_KW) || p.at(VARCHAR_KW));
    if p.eat(VARCHAR_KW) {
        return CHAR_TYPE;
    }
    p.bump_any();
    p.eat(VARYING_KW);
    CHAR_TYPE
}
/*
SimpleTypename ('[' (Iconst) ']')
SET OF SimpleTypename ('[' (Iconst) ']')
SimpleTypename array '[' Iconst ']'
SET OF SimpleTypename array '[' Iconst ']'
SimpleTypename array
SET OF SimpleTypename array

where
SimpleTypename is:
    GenericType
    | Numeric
    | Bit
    | Character
    | ConstDatetime
    | ConstInterval opt_interval
    | ConstInterval '(' Iconst ')'
    | JsonType

*/
#[must_use]
fn opt_type_name_with(p: &mut Parser<'_>, type_args_enabled: bool) -> Option<CompletedMarker> {
    let m = p.start();
    let wrapper_type = match p.current() {
        BIT_KW => {
            p.bump(BIT_KW);
            p.eat(VARYING_KW);
            BIT_TYPE
        }
        NATIONAL_KW if matches!(p.nth(1), CHAR_KW | CHARACTER_KW) => {
            p.bump(NATIONAL_KW);
            char_type(p)
        }
        CHARACTER_KW | CHAR_KW | NCHAR_KW | VARCHAR_KW => char_type(p),
        TIMESTAMP_KW | TIME_KW => {
            let name_ref = p.start();
            p.bump_any();
            name_ref.complete(p, NAME_REF);
            if p.eat(L_PAREN) {
                if expr(p).is_none() {
                    p.error("expected an expression");
                }
                p.expect(R_PAREN);
            }
            let m = p.start();
            if p.at(WITH_KW) || p.at(WITHOUT_KW) {
                let kind = if p.eat(WITH_KW) {
                    WITH_TIMEZONE
                } else {
                    p.bump(WITHOUT_KW);
                    WITHOUT_TIMEZONE
                };
                p.expect(TIME_KW);
                p.expect(ZONE_KW);
                m.complete(p, kind);
            } else {
                m.abandon(p);
            }
            TIME_TYPE
        }
        INTERVAL_KW => {
            p.bump(INTERVAL_KW);
            opt_interval_trailing(p);
            INTERVAL_TYPE
        }
        DOUBLE_KW => {
            p.bump(DOUBLE_KW);
            p.expect(PRECISION_KW);
            DOUBLE_TYPE
        }
        _ if p.at_ts(TYPE_KEYWORDS) || p.at(IDENT) => {
            path_name_ref(p);
            PATH_TYPE
        }
        _ => {
            m.abandon(p);
            return None;
        }
    };
    type_mods(p, m, type_args_enabled, wrapper_type)
}

fn opt_type_name(p: &mut Parser<'_>) -> bool {
    opt_type_name_with(p, true).is_some()
}

fn type_name(p: &mut Parser<'_>) {
    if !opt_type_name(p) {
        p.error("expected type name");
    }
}

fn simple_type_name(p: &mut Parser<'_>) {
    if opt_type_name_with(p, false).is_none() {
        p.error("expected simple type name");
    }
}

// json_name_and_value:
//   | c_expr VALUE_P json_value_expr
//   | a_expr ':' json_value_expr
//
//   json_value_expr:
//     a_expr json_format_clause_opt
fn json_key_value(p: &mut Parser<'_>, lhs: CompletedMarker) -> CompletedMarker {
    assert!(p.at(COLON) || p.at(VALUE_KW));
    let m = lhs.precede(p);
    p.bump_any();
    if expr(p).is_none() {
        p.error("expected expr");
    }
    opt_json_format_clause(p);
    m.complete(p, JSON_KEY_VALUE)
}

fn named_arg(p: &mut Parser<'_>, lhs: CompletedMarker) -> CompletedMarker {
    assert!(p.at(FAT_ARROW) || p.at(COLONEQ));
    let m = lhs.precede(p);
    if p.at(COLONEQ) {
        p.bump(COLONEQ);
    } else {
        p.bump(FAT_ARROW);
    }
    if expr(p).is_none() {
        p.error("expected expr");
    }
    m.complete(p, NAMED_ARG)
}

fn cast_expr(p: &mut Parser<'_>, lhs: CompletedMarker) -> CompletedMarker {
    assert!(p.at(COLON2));
    let m = lhs.precede(p);
    p.bump(COLON2);
    type_name(p);
    m.complete(p, CAST_EXPR)
}

fn arg_expr(p: &mut Parser<'_>) -> Option<CompletedMarker> {
    // https://www.postgresql.org/docs/17/typeconv-func.html
    p.eat(VARIADIC_KW);
    let r = Restrictions {
        order_by_allowed: true,
        ..Restrictions::default()
    };
    expr_bp(p, 1, &r)
}

fn arg_list(p: &mut Parser<'_>) {
    assert!(p.at(L_PAREN));
    let m = p.start();
    // sum(*), count(*), max(*)
    if p.nth_at(1, STAR) {
        p.bump(L_PAREN);
        p.expect(STAR);
        p.expect(R_PAREN);
        m.complete(p, ARG_LIST);
        return;
    }
    if p.nth_at(1, DISTINCT_KW) || p.nth_at(1, ALL_KW) {
        p.bump(L_PAREN);
        p.bump_any();
        if arg_expr(p).is_none() {
            p.error("expected expression");
        }
        p.expect(R_PAREN);
        m.complete(p, ARG_LIST);
        return;
    }
    delimited(
        p,
        L_PAREN,
        R_PAREN,
        COMMA,
        || "expected expression".into(),
        EXPR_FIRST.union(ATTRIBUTE_FIRST),
        |p| arg_expr(p).is_some(),
    );
    m.complete(p, ARG_LIST);
}

fn interval_second(p: &mut Parser<'_>) {
    p.expect(SECOND_KW);
    if p.eat(L_PAREN) {
        // TODO: int expr
        if expr(p).is_none() {
            p.error("expected an expression");
        }
        p.expect(R_PAREN);
    }
}

fn opt_interval_trailing(p: &mut Parser<'_>) {
    match (p.current(), p.nth(1)) {
        (DAY_KW, TO_KW) => {
            p.bump(DAY_KW);
            p.bump(TO_KW);
            match p.current() {
                HOUR_KW => {
                    p.bump(HOUR_KW);
                }
                MINUTE_KW => {
                    p.bump(MINUTE_KW);
                }
                SECOND_KW => {
                    interval_second(p);
                }
                _ => p.error("expected HOUR, MINUTE, or SECOND"),
            }
        }
        (DAY_KW, _) => p.bump(DAY_KW),
        (HOUR_KW, TO_KW) => {
            p.bump(HOUR_KW);
            p.bump(TO_KW);
            if !p.eat(MINUTE_KW) {
                interval_second(p);
            }
        }
        (HOUR_KW, _) => p.bump(HOUR_KW),
        (MINUTE_KW, TO_KW) => {
            p.bump(MINUTE_KW);
            p.bump(TO_KW);
            interval_second(p);
        }
        (MINUTE_KW, _) => p.bump(MINUTE_KW),
        (MONTH_KW, _) => p.bump(MONTH_KW),
        (YEAR_KW, TO_KW) => {
            p.bump(YEAR_KW);
            p.bump(TO_KW);
            p.expect(MONTH_KW);
        }
        (YEAR_KW, _) => {
            p.bump(YEAR_KW);
        }
        (SECOND_KW, _) => {
            interval_second(p);
        }
        _ => (),
    }
}

fn name_ref_(p: &mut Parser<'_>) -> Option<CompletedMarker> {
    if !p.at_ts(NAME_REF_FIRST) {
        return None;
    }
    let m = p.start();
    let mut is_interval_cast = false;
    if p.eat(COLLATION_KW) {
        p.expect(FOR_KW);
    // timestamp with time zone / time with time zone
    } else if p.eat(TIMESTAMP_KW) || p.eat(TIME_KW) {
        if p.eat(WITH_KW) {
            p.expect(TIME_KW);
            p.expect(ZONE_KW);
        }
    } else if p.eat(INTERVAL_KW) {
        is_interval_cast = true;
    } else {
        p.bump_any();
    }
    let cm = m.complete(p, NAME_REF);
    // A path followed by a literal is a type cast so we insert a CAST_EXPR
    // preceding it to wrap the previously parsed data.
    // e.g., `select numeric '12312'`
    if !p.at(NULL_KW) && !p.at(DEFAULT_KW) && literal(p).is_some() {
        if is_interval_cast {
            opt_interval_trailing(p);
        }
        Some(cm.precede(p).complete(p, CAST_EXPR))
    } else {
        Some(cm)
    }
}

fn between_expr(p: &mut Parser<'_>, lhs: CompletedMarker) -> CompletedMarker {
    assert!(p.at(NOT_KW) || p.at(BETWEEN_KW));
    // TODO: does this precede stuff matter?
    let m = lhs.precede(p);
    p.eat(NOT_KW);
    p.expect(BETWEEN_KW);
    p.eat(SYMMETRIC_KW);
    if bexpr(p).is_none() {
        p.error("expected an expression");
    }
    p.expect(AND_KW);
    if bexpr(p).is_none() {
        p.error("expected an expression");
    }
    m.complete(p, BETWEEN_EXPR)
}

fn call_expr_args(p: &mut Parser<'_>, lhs: CompletedMarker) -> CompletedMarker {
    assert!(p.at(L_PAREN));
    let m = lhs.precede(p);
    arg_list(p);
    // postgres has:
    // func_expr: func_application within_group_clause filter_clause over_clause
    if p.at(WITHIN_KW) {
        let m = p.start();
        p.expect(WITHIN_KW);
        p.expect(GROUP_KW);
        p.expect(L_PAREN);
        opt_order_by_clause(p);
        p.expect(R_PAREN);
        m.complete(p, WITHIN_CLAUSE);
    }
    if p.at(FILTER_KW) {
        let m = p.start();
        p.expect(FILTER_KW);
        p.expect(L_PAREN);
        p.expect(WHERE_KW);
        if expr(p).is_none() {
            p.error("expected an expression");
        }
        p.expect(R_PAREN);
        m.complete(p, FILTER_CLAUSE);
    }
    if p.at(OVER_KW) {
        // OVER window_name
        // OVER ( window_definition )
        let m = p.start();
        p.expect(OVER_KW);
        if p.eat(L_PAREN) {
            window_definition(p);
            p.expect(R_PAREN);
        } else {
            name_ref(p);
        }
        m.complete(p, OVER_CLAUSE);
    }
    m.complete(p, CALL_EXPR)
}

// foo[]
// foo[:b]
// foo[a:]
// foo[a:b]
// foo[:]
fn index_expr(p: &mut Parser<'_>, lhs: CompletedMarker) -> CompletedMarker {
    assert!(p.at(L_BRACK));
    let m = lhs.precede(p);
    p.bump(L_BRACK);
    if !p.eat(R_BRACK) {
        // foo[expr]
        // foo[:b]
        // foo[:]
        if p.eat(COLON) {
            // foo[:]
            if p.eat(R_BRACK) {
                return m.complete(p, INDEX_EXPR);
            } else {
                // foo[:b]
                if expr(p).is_none() {
                    p.error("expected an expression");
                }
                p.expect(R_BRACK);
                return m.complete(p, INDEX_EXPR);
            }
        }
        // foo[a]
        // foo[a:]
        // foo[a:b]
        if expr(p).is_none() {
            p.error("expected an expression");
        }
        if p.eat(COLON) {
            // foo[a:]
            if p.eat(R_BRACK) {
                return m.complete(p, INDEX_EXPR);
            }
            // foo[a:b]
            if expr(p).is_none() {
                p.error("expected an expression");
            }
        }
        p.expect(R_BRACK);
    }
    m.complete(p, INDEX_EXPR)
}

fn name_ref_or_index(p: &mut Parser<'_>) {
    assert!(p.at(IDENT) || p.at_ts(TYPE_KEYWORDS) || p.at_ts(ALL_KEYWORDS) || p.at(INT_NUMBER));
    let m = p.start();
    p.bump_any();
    m.complete(p, NAME_REF);
}

// TODO: do we need this float recovery stuff?
fn field_expr<const FLOAT_RECOVERY: bool>(
    p: &mut Parser<'_>,
    lhs: Option<CompletedMarker>,
    allow_calls: bool,
) -> Result<CompletedMarker, CompletedMarker> {
    if !FLOAT_RECOVERY {
        assert!(p.at(DOT));
    }
    let m = match lhs {
        Some(lhs) => lhs.precede(p),
        None => p.start(),
    };
    if !FLOAT_RECOVERY {
        p.bump(DOT);
    }
    if p.at(IDENT) || p.at_ts(TYPE_KEYWORDS) || p.at(INT_NUMBER) || p.at_ts(ALL_KEYWORDS) {
        name_ref_or_index(p);
    } else if p.at(FLOAT_NUMBER) {
        return match p.split_float(m) {
            (true, m) => {
                let lhs = m.complete(p, FIELD_EXPR);
                postfix_dot_expr::<true>(p, lhs, allow_calls)
            }
            (false, m) => Ok(m.complete(p, FIELD_EXPR)),
        };
    } else if p.eat(STAR) || opt_operator(p) {
        //
    } else {
        p.error(format!(
            "expected field name or number, got {:?}",
            p.current()
        ));
    }
    Ok(m.complete(p, FIELD_EXPR))
}

fn postfix_dot_expr<const FLOAT_RECOVERY: bool>(
    p: &mut Parser<'_>,
    lhs: CompletedMarker,
    allow_calls: bool,
) -> Result<CompletedMarker, CompletedMarker> {
    if !FLOAT_RECOVERY {
        assert!(p.at(DOT));
    }
    field_expr::<FLOAT_RECOVERY>(p, Some(lhs), allow_calls).map(|m| {
        // A field followed by a literal is a type cast so we insert a CAST_EXPR
        // preceding it to wrap the previously parsed data.
        if !p.at(NULL_KW) && !p.at(DEFAULT_KW) && literal(p).is_some() {
            m.precede(p).complete(p, CAST_EXPR)
        } else {
            m
        }
    })
}

#[must_use]
fn expr(p: &mut Parser) -> Option<CompletedMarker> {
    expr_bp(p, 1, &Restrictions::default())
}

// Based on the Postgres grammar b_expr, it's expr without `AND`, `NOT`, `IS`,
// and `IN`
#[must_use]
fn bexpr(p: &mut Parser<'_>) -> Option<CompletedMarker> {
    expr_bp(
        p,
        1,
        &Restrictions {
            in_disabled: true,
            is_disabled: true,
            not_disabled: true,
            and_disabled: true,
            ..Restrictions::default()
        },
    )
}

fn json_object_arg(p: &mut Parser) -> Option<CompletedMarker> {
    expr_bp(
        p,
        1,
        &Restrictions {
            json_field_arg_allowed: true,
            ..Restrictions::default()
        },
    )
}

enum Associativity {
    Left,
    Right,
}

/// Binding powers of operators for a Pratt parser.
///
/// See <https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html>
fn current_op(p: &Parser<'_>, r: &Restrictions) -> (u8, SyntaxKind, Associativity) {
    use Associativity::*;
    const NOT_AN_OP: (u8, SyntaxKind, Associativity) = (0, AT, Left);
    // For binding power, checkout:
    // https://www.postgresql.org/docs/17/sql-syntax-lexical.html#SQL-PRECEDENCE
    // https://github.com/postgres/postgres/blob/30f017626308a06cf0c0c82a706a1ba1b07df34a/src/backend/parser/gram.y#L817-L898
    match p.current() {
        // or
        OR_KW => (1, OR_KW, Left),
        // >=
        R_ANGLE if p.at(GTEQ) => (5, GTEQ, Left), // symbol
        // >
        R_ANGLE if p.next_not_joined_op(0) => (5, R_ANGLE, Left), // symbol
        // Later on we return a NAMED_ARG for this instead of BIN_EXPR
        // =>
        EQ if p.at(FAT_ARROW) => (7, FAT_ARROW, Right), // symbol
        // =
        EQ if p.next_not_joined_op(0) => (5, EQ, Right), // symbol
        // in
        IN_KW if !r.in_disabled => (6, IN_KW, Right),
        // <>
        L_ANGLE if p.at(NEQB) => (5, NEQB, Left), // symbol
        // <=
        L_ANGLE if p.at(LTEQ) => (5, LTEQ, Left), // symbol
        // <
        L_ANGLE if p.next_not_joined_op(0) => (5, L_ANGLE, Left), // symbol
        // +
        PLUS if p.next_not_joined_op(0) => (8, PLUS, Left), // symbol
        // overlaps
        OVERLAPS_KW => (7, OVERLAPS_KW, Left),
        // like
        LIKE_KW => (6, LIKE_KW, Left),
        // not like
        NOT_KW if !r.not_disabled && p.at(NOT_LIKE) => (6, NOT_LIKE, Left),
        // not in
        NOT_KW if !r.not_disabled && p.at(NOT_IN) => (6, NOT_IN, Left),
        // is distinct from
        IS_KW if !r.is_disabled && p.at(IS_DISTINCT_FROM) => (4, IS_DISTINCT_FROM, Left),
        // is not distinct from
        IS_KW if !r.is_disabled && p.at(IS_NOT_DISTINCT_FROM) => (4, IS_NOT_DISTINCT_FROM, Left),
        // at time zone
        AT_KW if p.at(AT_TIME_ZONE) => (11, AT_TIME_ZONE, Left),
        // similar to
        SIMILAR_KW if p.at(SIMILAR_TO) => (6, SIMILAR_TO, Left),
        // is not
        IS_KW if p.at(IS_NOT) => (4, IS_NOT, Left),
        // operator(pg_catalog.+)
        OPERATOR_KW if p.at(OPERATOR_CALL) => (7, OPERATOR_CALL, Left),
        // is
        IS_KW if !r.is_disabled => (4, IS_KW, Left),
        // ^
        CARET if p.next_not_joined_op(0) => (10, CARET, Left), // symbol
        // %
        PERCENT if p.next_not_joined_op(0) => (9, PERCENT, Left), // symbol
        // and
        AND_KW if !r.and_disabled => (2, AND_KW, Left),
        // /
        SLASH if p.next_not_joined_op(0) => (9, SLASH, Left), // symbol
        // *
        STAR if p.next_not_joined_op(0) => (9, STAR, Left), // symbol
        // !=
        BANG if p.at(NEQ) => (5, NEQ, Left), // symbol
        // collate
        COLLATE_KW => (12, COLLATE_KW, Left),
        // -
        MINUS if p.next_not_joined_op(0) => (8, MINUS, Left), // symbol
        // Later on we return a NAMED_ARG for this instead of BIN_EXPR
        // :=
        COLON if p.at(COLONEQ) => (5, COLONEQ, Right), // symbol
        // ::
        COLON if p.at(COLON2) => (15, COLON2, Left), // symbol
        // Only used in json_object, like json_object('a' value 1) instead of json_object('a': 1)
        // value
        VALUE_KW if r.json_field_arg_allowed => (7, VALUE_KW, Right),
        // Later on we return a FIELD_ARG instead of BIN_EXPR
        // a: b
        COLON if r.json_field_arg_allowed => (7, COLON, Right),
        _ if p.at_ts(OPERATOR_FIRST) => (7, CUSTOM_OP, Right),
        _ => NOT_AN_OP,
    }
}

// tokens thare in bin expr and also in bare_labels
const OVERLAPPING_TOKENS: TokenSet = TokenSet::new(&[OR_KW, AND_KW, IS_KW, COLLATE_KW]);

#[derive(Default)]
struct Restrictions {
    order_by_allowed: bool,
    json_field_arg_allowed: bool,
    in_disabled: bool,
    is_disabled: bool,
    not_disabled: bool,
    and_disabled: bool,
}

#[must_use]
fn expr_bp(p: &mut Parser<'_>, bp: u8, r: &Restrictions) -> Option<CompletedMarker> {
    let m = p.start();
    if !p.at_ts(EXPR_FIRST) {
        p.err_recover(
            &format!("expected an expression, found {:?}", p.current()),
            EXPR_RECOVERY_SET,
        );
        m.abandon(p);
        return None;
    }
    let mut lhs = match lhs(p, r) {
        Some(lhs) => lhs.extend_to(p, m),
        None => {
            m.abandon(p);
            return None;
        }
    };
    // if we're dealing with a bare column label, there's some operator keywords
    // that are allowed that can trip us up, e.g,
    //
    //    select 1 not;
    //
    // to solve this we check if the token following the possible operator looks
    // like an expr, in which case we assume we're dealing with a binary expr,
    // otherwise we assume it's a bare column label.
    if p.at_ts(OVERLAPPING_TOKENS)
        && !p.nth_at_ts(1, EXPR_FIRST)
        // could be start of `is distinct from`
        && !(p.at(IS_KW) && p.nth_at(1, DISTINCT_KW))
    {
        col_label(p);
        return Some(lhs);
    }
    if r.order_by_allowed && p.at(ORDER_KW) {
        opt_order_by_clause(p);
    }
    loop {
        let (op_bp, op, associativity) = current_op(p, r);
        if op_bp < bp {
            break;
        }
        match op {
            COLON2 => {
                lhs = cast_expr(p, lhs);
                continue;
            }
            FAT_ARROW | COLONEQ => {
                lhs = named_arg(p, lhs);
                continue;
            }
            COLON | VALUE_KW => {
                lhs = json_key_value(p, lhs);
                continue;
            }
            _ => {}
        }
        let m = lhs.precede(p);
        p.bump(op);
        let op_bp = match associativity {
            Associativity::Left => op_bp + 1,
            Associativity::Right => op_bp,
        };
        let _ = expr_bp(p, op_bp, r);
        lhs = m.complete(p, BIN_EXPR);
    }
    Some(lhs)
}

fn expr_list(p: &mut Parser) -> bool {
    let mut found_expr = false;
    while !p.at(COMMA) {
        if expr(p).is_none() {
            break;
        }
        found_expr = true;
        if !p.eat(COMMA) {
            break;
        }
    }
    found_expr
}

const COMPOUND_SELECT_FIRST: TokenSet = TokenSet::new(&[UNION_KW, INTERSECT_KW, EXCEPT_KW]);

// with_query_name [ ( column_name [, ...] ) ] AS [ [ NOT ] MATERIALIZED ] ( select | values | insert | update | delete | merge )
//     [ SEARCH { BREADTH | DEPTH } FIRST BY column_name [, ...] SET search_seq_col_name ]
//     [ CYCLE column_name [, ...] SET cycle_mark_col_name [ TO cycle_mark_value DEFAULT cycle_mark_default ] USING cycle_path_col_name ]
fn with_query(p: &mut Parser<'_>) -> Option<CompletedMarker> {
    let m = p.start();
    name(p);
    opt_column_list_with(p, ColumnDefKind::Name);
    p.expect(AS_KW);
    // [ [ NOT ] MATERIALIZED ]
    if p.eat(NOT_KW) {
        p.expect(MATERIALIZED_KW);
    } else {
        p.eat(MATERIALIZED_KW);
    }
    p.expect(L_PAREN);
    match p.current() {
        DELETE_KW => {
            delete_stmt(p, None);
        }
        SELECT_KW | TABLE_KW | VALUES_KW => {
            select_stmt(p, None);
        }
        INSERT_KW => {
            insert_stmt(p, None);
        }
        UPDATE_KW => {
            update_stmt(p, None);
        }
        MERGE_KW => {
            merge_stmt(p, None);
        }
        WITH_KW => {
            with_stmt(p, None);
        }
        _ => {
            p.error(format!(
                "expected DELETE, SELECT, TABLE, VALUES, INSERT, WITH, or UPDATE, got: {:?}",
                p.current()
            ));
        }
    }
    p.expect(R_PAREN);
    // [ SEARCH { BREADTH | DEPTH } FIRST BY column_name [, ...] SET search_seq_col_name ]
    if p.eat(SEARCH_KW) {
        if !p.eat(BREADTH_KW) {
            p.expect(DEPTH_KW);
        }
        p.expect(FIRST_KW);
        p.expect(BY_KW);
        while !p.at(EOF) && !p.at(COMMA) {
            name_ref(p);
            if !p.eat(COMMA) {
                break;
            }
        }
        p.expect(SET_KW);
        name_ref(p);
    }
    // [ CYCLE column_name [, ...] SET cycle_mark_col_name [ TO cycle_mark_value DEFAULT cycle_mark_default ] USING cycle_path_col_name ]
    if p.eat(CYCLE_KW) {
        while !p.at(EOF) && !p.at(COMMA) {
            name_ref(p);
            if !p.eat(COMMA) {
                break;
            }
        }
        p.expect(SET_KW);
        name_ref(p);
        if p.eat(TO_KW) {
            // TODO: we should limit this more
            if expr(p).is_none() {
                p.error("expected an expression");
            }
            p.expect(DEFAULT_KW);
            // TODO: we should limit this more
            if expr(p).is_none() {
                p.error("expected an expression");
            }
        }
        p.expect(USING_KW);
        name_ref(p);
    }
    Some(m.complete(p, WITH_TABLE))
}

const WITH_STMT_FOLLOW: TokenSet = TokenSet::new(&[
    DELETE_KW, SELECT_KW, TABLE_KW, INSERT_KW, UPDATE_KW, MERGE_KW,
]);

// [ WITH [ RECURSIVE ] with_query [, ...] ]
fn with_query_clause(p: &mut Parser<'_>) -> Option<CompletedMarker> {
    let m = p.start();
    p.expect(WITH_KW);
    p.eat(RECURSIVE_KW);
    while !p.at(EOF) {
        if with_query(p).is_none() {
            p.error("expected with_query");
        }
        if p.at(COMMA) && p.nth_at_ts(1, WITH_STMT_FOLLOW) {
            p.err_and_bump("unexpected comma");
            break;
        }
        if !p.eat(COMMA) {
            break;
        }
    }
    Some(m.complete(p, WITH_CLAUSE))
}

fn select_clause(p: &mut Parser<'_>) -> CompletedMarker {
    let m = p.start();
    // normal select stmts
    p.expect(SELECT_KW);
    // select 1 + 1;
    //        ^
    // select all
    //        ^^^
    // select distinct
    //        ^^^^^^^^
    opt_all_or_distinct(p);
    opt_target_list(p);
    m.complete(p, SELECT_CLAUSE)
}

fn compound_select(p: &mut Parser<'_>, cm: CompletedMarker) -> CompletedMarker {
    assert!(p.at_ts(COMPOUND_SELECT_FIRST));
    let m = cm.precede(p);
    p.bump_any();
    if !p.eat(ALL_KW) {
        p.eat(DISTINCT_KW);
    }
    select_stmt(p, None);
    m.complete(p, COMPOUND_SELECT)
}

// error recovery:
// - <https://youtu.be/0HlrqwLjCxA?feature=shared&t=2172>
/// <https://www.postgresql.org/docs/17/sql-select.html>
fn select_stmt(p: &mut Parser, m: Option<Marker>) -> Option<CompletedMarker> {
    assert!(p.at_ts(SELECT_FIRST));
    let m = m.unwrap_or_else(|| p.start());
    // table [only] name [*]
    if p.eat(TABLE_KW) {
        relation_name(p);
        return Some(m.complete(p, SELECT));
    }
    // with aka cte
    // [ WITH [ RECURSIVE ] with_query [, ...] ]
    if p.at(WITH_KW) {
        return with_stmt(p, Some(m));
    }
    if p.at(VALUES_KW) {
        let cm = values_clause(p, Some(m));
        if p.at_ts(COMPOUND_SELECT_FIRST) {
            return Some(compound_select(p, cm));
        } else {
            return Some(cm);
        }
    }
    select_clause(p);
    let is_select_into = opt_into_clause(p).is_some();
    opt_from_clause(p);
    opt_where_clause(p);
    opt_group_by_clause(p);
    opt_having_clause(p);
    opt_window_clause(p);
    if p.at_ts(COMPOUND_SELECT_FIRST) {
        let cm = m.complete(p, SELECT);
        return Some(compound_select(p, cm));
    }
    opt_order_by_clause(p);
    let mut has_locking_clause = false;
    while p.at(FOR_KW) {
        if opt_locking_clause(p).is_some() {
            has_locking_clause = true;
        }
    }
    opt_limit_clause(p);
    opt_offset_clause(p);
    opt_fetch_clause(p);
    if !has_locking_clause {
        while p.at(FOR_KW) {
            opt_locking_clause(p);
        }
    }
    Some(m.complete(
        p,
        if is_select_into {
            SELECT_INTO_STMT
        } else {
            SELECT
        },
    ))
}

// INTO [ TEMPORARY | TEMP | UNLOGGED ] [ TABLE ] new_table
fn opt_into_clause(p: &mut Parser<'_>) -> Option<CompletedMarker> {
    if p.at(INTO_KW) {
        let m = p.start();
        p.bump(INTO_KW);
        let _ = opt_temp(p) || p.eat(UNLOGGED_KW);
        p.eat(TABLE_KW);
        path_name(p);
        Some(m.complete(p, INTO_CLAUSE))
    } else {
        None
    }
}

fn lock_strength(p: &mut Parser<'_>) -> bool {
    // NO KEY UPDATE
    if p.eat(NO_KW) {
        p.expect(KEY_KW);
        p.expect(UPDATE_KW)
    // KEY SHARE
    } else if p.eat(KEY_KW) {
        p.expect(SHARE_KW)
    // SHARE
    } else if !p.eat(SHARE_KW) {
        // UPDATE
        p.expect(UPDATE_KW)
    } else {
        false
    }
}

fn opt_locking_clause(p: &mut Parser<'_>) -> Option<CompletedMarker> {
    let m = p.start();
    if !p.eat(FOR_KW) {
        m.abandon(p);
        return None;
    }
    lock_strength(p);
    if p.eat(OF_KW) {
        while !p.at(EOF) && !p.at(SEMICOLON) {
            if expr(p).is_none() {
                p.error("expected an expression");
            }
            if !p.eat(COMMA) {
                break;
            }
        }
    }
    if p.eat(SKIP_KW) {
        p.expect(LOCKED_KW)
    } else {
        p.eat(NOWAIT_KW)
    };
    Some(m.complete(p, LOCKING_CLAUSE))
}

// FETCH { FIRST | NEXT } [ count ] { ROW | ROWS } { ONLY | WITH TIES }
fn opt_fetch_clause(p: &mut Parser<'_>) -> bool {
    if !p.eat(FETCH_KW) {
        return false;
    }
    // { FIRST | NEXT }
    if p.at(FIRST_KW) || p.at(NEXT_KW) {
        p.bump_any();
    } else {
        p.error("expected first or next");
    }
    // [ count ]
    if expr(p).is_none() {
        p.error("expected an expression");
    }
    // { ROW | ROWS }
    if !p.eat(ROW_KW) {
        p.expect(ROWS_KW);
    }
    // { ONLY | WITH TIES }
    if p.eat(WITH_KW) {
        p.expect(TIES_KW);
    } else {
        p.expect(ONLY_KW);
    }
    true
}

fn opt_order_by_clause(p: &mut Parser<'_>) -> bool {
    let m = p.start();
    if !p.eat(ORDER_KW) {
        m.abandon(p);
        return false;
    }
    p.expect(BY_KW);
    while !p.at(EOF) {
        if expr(p).is_none() {
            p.error("expected an expression");
        }
        // ASC | DESC | USING operator
        if p.at(ASC_KW) || p.at(DESC_KW) {
            p.bump_any();
        } else if p.eat(USING_KW) {
            operator(p);
        }
        // NULLS {FIRST | LAST}
        if p.eat(NULLS_KW) {
            if p.at(FIRST_KW) || p.at(LAST_KW) {
                p.bump_any();
            } else {
                p.error("expected FIRST or LAST following NULLS");
            }
        }
        if !p.eat(COMMA) {
            break;
        }
    }
    m.complete(p, ORDER_BY_CLAUSE);
    true
}

const JOIN_TYPE_FIRST: TokenSet = TokenSet::new(&[INNER_KW, JOIN_KW, LEFT_KW, RIGHT_KW, FULL_KW]);

// where join_type is:
//   [ INNER ] JOIN
//   LEFT [ OUTER ] JOIN
//   RIGHT [ OUTER ] JOIN
//   FULL [ OUTER ] JOIN
fn join_type(p: &mut Parser<'_>) -> bool {
    assert!(p.at_ts(JOIN_TYPE_FIRST));
    if p.eat(INNER_KW) {
        p.expect(JOIN_KW)
    } else if p.eat(LEFT_KW) || p.eat(RIGHT_KW) || p.eat(FULL_KW) {
        p.eat(OUTER_KW);
        p.expect(JOIN_KW)
    } else {
        p.expect(JOIN_KW)
    }
}

const JOIN_FIRST: TokenSet = TokenSet::new(&[NATURAL_KW, CROSS_KW]).union(JOIN_TYPE_FIRST);

fn opt_from_clause(p: &mut Parser<'_>) -> bool {
    let m = p.start();
    if !p.eat(FROM_KW) {
        m.abandon(p);
        return false;
    }
    while !p.at(EOF) {
        if !opt_from_item(p) {
            m.complete(p, FROM_CLAUSE);
            return false;
        }
        // foo, bar, buzz
        //    ^
        if !p.eat(COMMA) {
            break;
        }
    }
    m.complete(p, FROM_CLAUSE);
    true
}

// https://github.com/postgres/postgres/blob/b3219c69fc1e161df8d380c464b3f2cce3b6cab9/src/backend/parser/gram.y#L18042
const COL_NAME_KEYWORD_FIRST: TokenSet = TokenSet::new(&[
    BETWEEN_KW,
    BIGINT_KW,
    BIT_KW,
    BOOLEAN_KW,
    CHAR_KW,
    CHARACTER_KW,
    COALESCE_KW,
    DEC_KW,
    DECIMAL_KW,
    EXISTS_KW,
    EXTRACT_KW,
    FLOAT_KW,
    GREATEST_KW,
    GROUPING_KW,
    INOUT_KW,
    INT_KW,
    INTEGER_KW,
    INTERVAL_KW,
    JSON_KW,
    JSON_ARRAY_KW,
    JSON_ARRAYAGG_KW,
    JSON_EXISTS_KW,
    JSON_OBJECT_KW,
    JSON_OBJECTAGG_KW,
    JSON_QUERY_KW,
    JSON_SCALAR_KW,
    JSON_SERIALIZE_KW,
    JSON_TABLE_KW,
    JSON_VALUE_KW,
    LEAST_KW,
    MERGE_ACTION_KW,
    NATIONAL_KW,
    NCHAR_KW,
    NONE_KW,
    NORMALIZE_KW,
    NULLIF_KW,
    NUMERIC_KW,
    OUT_KW,
    OVERLAY_KW,
    POSITION_KW,
    PRECISION_KW,
    REAL_KW,
    ROW_KW,
    SETOF_KW,
    SMALLINT_KW,
    SUBSTRING_KW,
    TIME_KW,
    TIMESTAMP_KW,
    TREAT_KW,
    TRIM_KW,
    VALUES_KW,
    VARCHAR_KW,
    XMLATTRIBUTES_KW,
    XMLCONCAT_KW,
    XMLELEMENT_KW,
    XMLEXISTS_KW,
    XMLFOREST_KW,
    XMLNAMESPACES_KW,
    XMLPARSE_KW,
    XMLPI_KW,
    XMLROOT_KW,
    XMLSERIALIZE_KW,
    XMLTABLE_KW,
]);

// https://github.com/postgres/postgres/blob/2421e9a51d20bb83154e54a16ce628f9249fa907/src/backend/parser/gram.y#L15798C13-L16258
// Generated via the above grammar, but we only take the keywords that are
// single items. So `CURRENT_DATE` but not `COLLATION FOR '(' a_expr ')'`
const FUNC_EXPR_COMMON_SUBEXPR_FIRST: TokenSet = TokenSet::new(&[
    CURRENT_DATE_KW,
    CURRENT_TIME_KW,
    CURRENT_TIMESTAMP_KW,
    LOCALTIME_KW,
    LOCALTIMESTAMP_KW,
    CURRENT_ROLE_KW,
    CURRENT_USER_KW,
    SESSION_USER_KW,
    SYSTEM_USER_KW,
    USER_KW,
    CURRENT_CATALOG_KW,
    CURRENT_SCHEMA_KW,
]);

const FROM_ITEM_KEYWORDS_FIRST: TokenSet = TokenSet::new(&[])
    .union(UNRESERVED_KEYWORDS)
    .union(COL_NAME_KEYWORD_FIRST)
    .union(FUNC_EXPR_COMMON_SUBEXPR_FIRST);

const FROM_ITEM_FIRST: TokenSet = TokenSet::new(&[
    ONLY_KW,    // optional
    IDENT,      // table_name, with_query_name, function_name
    L_PAREN,    // nested select stmt
    LATERAL_KW, // optional
    ROWS_KW,    // rows from
])
.union(FROM_ITEM_KEYWORDS_FIRST);

fn from_item_name(p: &mut Parser<'_>) {
    match name_ref_(p).map(|lhs| postfix_expr(p, lhs, true)) {
        Some(val) => match val.kind() {
            CALL_EXPR => {
                // [ WITH ORDINALITY ]
                //    [ [ AS ] alias [ ( column_alias [, ...] ) ] ]
                // [ AS ] alias ( column_definition [, ...] )
                // AS ( column_definition [, ...] )
                // TODO: we should use this to inform parsing down below
                if p.eat(WITH_KW) {
                    p.expect(ORDINALITY_KW);
                }
                opt_alias(p);
            }
            NAME_REF | FIELD_EXPR => {
                //  [ * ] [ [ AS ] alias [ ( column_alias [, ...] ) ] ]
                //              [ TABLESAMPLE sampling_method ( argument [, ...] ) [ REPEATABLE ( seed ) ] ]
                //
                //  [ [ AS ] alias [ ( column_alias [, ...] ) ] ]
                // we're at a table_name
                p.eat(STAR);
                opt_alias(p);
                // [ TABLESAMPLE sampling_method ( argument [, ...] ) [ REPEATABLE ( seed ) ] ]
                if p.eat(TABLESAMPLE_KW) {
                    call_expr(p);
                    if p.eat(REPEATABLE_KW) {
                        p.eat(R_PAREN);
                        if expr(p).is_none() {
                            p.error("expected a seed");
                        }
                        p.eat(L_PAREN);
                    }
                }
            }
            got => {
                p.error(format!("expected a name, got {:?}", got));
            }
        },
        None => p.error("expected name"),
    }
}

fn data_source(p: &mut Parser<'_>) {
    p.eat(ONLY_KW);
    p.eat(LATERAL_KW);
    match p.current() {
        L_PAREN => {
            p.bump(L_PAREN);
            // we're at the start of a nested select statement
            select_stmt(p, None);
            p.expect(R_PAREN);
            opt_alias(p);
        }
        JSON_TABLE_KW => {
            json_table_fn(p);
            opt_alias(p);
        }
        ROWS_KW => {
            p.bump(ROWS_KW);
            p.expect(FROM_KW);
            p.expect(L_PAREN);
            call_expr(p);
            // TODO: we should restrict this more
            opt_alias(p);
            p.expect(R_PAREN);
            // [ WITH ORDINALITY ]
            if p.eat(WITH_KW) {
                p.expect(ORDINALITY_KW);
            }
            // TODO: we should only alow col_alias, not def
            opt_alias(p);
        }
        IDENT => from_item_name(p),
        _ if p.at_ts(FROM_ITEM_KEYWORDS_FIRST) => from_item_name(p),
        _ => {}
    }
}

// USING data_source ON join_condition
fn merge_using_clause(p: &mut Parser<'_>) {
    let m1 = p.start();
    p.expect(USING_KW);
    data_source(p);
    p.expect(ON_KW);
    // join_condition
    if expr(p).is_none() {
        p.error("expected an expression");
    }
    m1.complete(p, USING_CLAUSE);
}

// where from_item can be one of:
//
//  [ LATERAL ] ( select ) [ [ AS ] alias [ ( column_alias [, ...] ) ] ]
//
//  [ ONLY ] table_name [ * ] [ [ AS ] alias [ ( column_alias [, ...] ) ] ]
//              [ TABLESAMPLE sampling_method ( argument [, ...] ) [ REPEATABLE ( seed ) ] ]
//
//  with_query_name [ [ AS ] alias [ ( column_alias [, ...] ) ] ]
//
//  [ LATERAL ] function_name ( [ argument [, ...] ] )
//      [ WITH ORDINALITY ]
//      [ [ AS ] alias [ ( column_alias [, ...] ) ] ]
//  [ LATERAL ] function_name ( [ argument [, ...] ] )
//      [ AS ] alias ( column_definition [, ...] )
//  [ LATERAL ] function_name ( [ argument [, ...] ] )
//      AS ( column_definition [, ...] )
//
//  [ LATERAL ] ROWS FROM(
//      function_name ( [ argument [, ...] ] ) [ AS ( column_definition [, ...] ) ] [, ...]
//    )
//    [ WITH ORDINALITY ]
//    [ [ AS ] alias [ ( column_alias [, ...] ) ] ]
//
//  from_item join_type from_item { ON join_condition | USING ( join_column [, ...] ) [ AS join_using_alias ] }
//  from_item NATURAL join_type from_item
//  from_item CROSS JOIN from_item
//
//  join_type is one of:
//
//    [ INNER ] JOIN
//    LEFT [ OUTER ] JOIN
//    RIGHT [ OUTER ] JOIN
//    FULL [ OUTER ] JOIN
//
#[must_use]
fn opt_from_item(p: &mut Parser<'_>) -> bool {
    if !p.at_ts(FROM_ITEM_FIRST) {
        return false;
    }
    data_source(p);
    while p.at_ts(JOIN_FIRST) {
        join(p);
    }
    true
}

// we have a from_item
//
//   join_type from_item { ON join_condition | USING ( join_column [, ...] ) [ AS join_using_alias ] }
//   NATURAL join_type from_item
//   CROSS JOIN from_item
//
// where join_type is:
//
//   [ INNER ] JOIN
//   LEFT [ OUTER ] JOIN
//   RIGHT [ OUTER ] JOIN
//   FULL [ OUTER ] JOIN
fn join(p: &mut Parser<'_>) {
    assert!(p.at_ts(JOIN_FIRST));
    let m = p.start();
    if p.eat(NATURAL_KW) {
        if !join_type(p) {
            p.error("expected join type");
        }
        if !opt_from_item(p) {
            p.error("expected from_item");
        }
    } else if p.eat(CROSS_KW) {
        p.expect(JOIN_KW);
        if !opt_from_item(p) {
            p.error("expected from_item");
        }
    } else {
        if !join_type(p) {
            p.error("expected join type");
        }
        if !opt_from_item(p) {
            p.error("expected from_item");
        }
        if p.eat(ON_KW) {
            if expr(p).is_none() {
                p.error("expected an expression");
            }
        } else {
            {
                let m = p.start();
                // USING ( join_column [, ...] )
                p.expect(USING_KW);
                if p.at(L_PAREN) {
                    tuple_expr(p);
                } else {
                    p.error("expected L_PAREN");
                }
                m.complete(p, USING_CLAUSE);
            }
            {
                let m = p.start();
                // [ AS join_using_alias ]
                if p.eat(AS_KW) {
                    name(p);
                    m.complete(p, ALIAS);
                } else {
                    m.abandon(p);
                }
            }
        }
    }
    m.complete(p, JOIN);
}

#[must_use]
fn opt_numeric_literal(p: &mut Parser<'_>) -> Option<CompletedMarker> {
    if p.at_ts(NUMERIC_FIRST) {
        let m = p.start();
        p.bump_any();
        Some(m.complete(p, LITERAL))
    } else if p.at(MINUS) && p.nth_at_ts(1, NUMERIC_FIRST) {
        // TODO: is this a good idea?
        expr(p)
    } else {
        None
    }
}

const SEQUENCE_OPTION_FIRST: TokenSet = TokenSet::new(&[
    AS_KW,
    CACHE_KW,
    INCREMENT_KW,
    SEQUENCE_KW,
    RESTART_KW,
    LOGGED_KW,
    UNLOGGED_KW,
    START_KW,
    OWNED_KW,
    OWNED_KW,
    MAXVALUE_KW,
    MINVALUE_KW,
    NO_KW,
    CYCLE_KW,
]);

// [ AS data_type ]
// [ CACHE cache ]
// [ INCREMENT [ BY ] increment ]
// [ SEQUENCE NAME name ]
// [ RESTART [ [ WITH ] NUMERIC ] ]
// [ LOGGED UNLOGGED ]
// [ START [ WITH ] start ]
// [ OWNED BY { table_name.column_name | NONE } ]
// [ MAXVALUE maxvalue ]
// [ MINVALUE maxvalue ]
// [ NO MINVALUE | NO CYCLE | NO MAXVALUE ]
// [ CYCLE ]
fn opt_sequence_option(p: &mut Parser<'_>) -> bool {
    if !p.at_ts(SEQUENCE_OPTION_FIRST) {
        return false;
    }
    match p.current() {
        AS_KW => {
            p.bump(AS_KW);
            type_name(p);
        }
        INCREMENT_KW => {
            p.bump(INCREMENT_KW);
            p.eat(BY_KW);
            if opt_numeric_literal(p).is_none() {
                p.error("expected numeric literal");
            }
        }
        SEQUENCE_KW => {
            p.bump(SEQUENCE_KW);
            p.expect(NAME_KW);
            name_ref(p);
        }
        RESTART_KW => {
            p.bump(RESTART_KW);
            if p.eat(WITH_KW) {
                if opt_numeric_literal(p).is_none() {
                    p.error("expected numeric");
                }
            } else {
                let _ = opt_numeric_literal(p);
            }
        }
        LOGGED_KW | UNLOGGED_KW => {
            p.bump_any();
        }
        START_KW => {
            p.bump(START_KW);
            p.eat(WITH_KW);
            if opt_numeric_literal(p).is_none() {
                p.error("expected numeric");
            }
        }
        OWNED_KW => {
            p.bump(OWNED_KW);
            p.expect(BY_KW);
            if !p.eat(NONE_KW) {
                path_name_ref(p);
            }
        }
        MINVALUE_KW | MAXVALUE_KW | CACHE_KW => {
            p.bump_any();
            if opt_numeric_literal(p).is_none() {
                p.error("expected numeric");
            }
        }
        NO_KW => {
            p.bump(NO_KW);
            if !p.eat(MINVALUE_KW) && !p.eat(CYCLE_KW) && !p.eat(MAXVALUE_KW) {
                p.error("expected MINVALUE, MAXVALUE, or CYCLE");
            }
        }
        CYCLE_KW => {
            p.bump(CYCLE_KW);
        }
        _ => return false,
    }
    true
}

fn opt_sequence_options(p: &mut Parser<'_>) -> bool {
    if p.at(L_PAREN) {
        let m = p.start();
        p.bump(L_PAREN);
        while !p.at(EOF) {
            // TODO: make sure we have at least one
            if !opt_sequence_option(p) {
                break;
            }
        }
        p.expect(R_PAREN);
        m.complete(p, SEQUENCE_OPTION_LIST);
        true
    } else {
        false
    }
}

// storage_parameter [= value]
fn storage_parameter(p: &mut Parser<'_>) -> bool {
    // storage_parameter
    path_name_ref(p);
    // [= value]
    if p.eat(EQ) && !def_arg(p) {
        p.error("expected a value for storage parameter");
        return false;
    }
    true
}

enum ColumnDefKind {
    Name,
    Ref,
    WithData,
}

// select * from f() as t(a, b);
//                       ^^^^^^
// select * from f() as t(a int, b text, c text collate foo.bar.buzz);
//                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
// [ ( column_name [, ... ] ) ]
fn opt_column_list_with(p: &mut Parser<'_>, kind: ColumnDefKind) -> bool {
    if !p.at(L_PAREN) {
        return false;
    }
    let m = p.start();
    p.expect(L_PAREN);
    while !p.at(EOF) && !p.at(R_PAREN) {
        if p.at(COMMA) {
            p.err_and_bump("missing column");
            continue;
        }
        if !p.at_ts(COLUMN_FIRST) {
            break;
        }
        column(p, &kind);
        if p.at(COMMA) && p.nth_at(1, R_PAREN) {
            p.err_and_bump("unexpected trailing comma");
        }
        if !p.eat(COMMA) {
            if p.at_ts(COLUMN_FIRST) && !(p.at(WITHOUT_KW) && p.nth_at(1, OVERLAPS_KW)) {
                p.error("expected COMMA");
            } else {
                break;
            }
        }
    }
    opt_without_overlaps(p);
    p.expect(R_PAREN);
    m.complete(p, COLUMN_LIST);
    return true;
}

fn column(p: &mut Parser<'_>, kind: &ColumnDefKind) -> CompletedMarker {
    assert!(p.at_ts(COLUMN_FIRST));
    let m = p.start();
    // https://www.depesz.com/2024/10/03/waiting-for-postgresql-18-add-temporal-foreign-key-contraints/
    // TODO: add validation to ensure this is in the right position
    p.eat(PERIOD_KW);
    match kind {
        ColumnDefKind::Name => name(p),
        ColumnDefKind::Ref => {
            // support parsing things like:
            // INSERT INTO tictactoe (game, board[1:3][1:3])
            name_ref(p).map(|lhs| postfix_expr(p, lhs, true));
        }
        ColumnDefKind::WithData => {
            name(p);
            if !p.at(COMMA) && !p.at(R_PAREN) {
                if !opt_type_name(p) {
                    return m.complete(p, COLUMN);
                }
                opt_collate(p);
            }
        }
    }
    m.complete(p, COLUMN)
}

// [ ( column_name [, ... ] ) ]
fn opt_column_list(p: &mut Parser<'_>) -> bool {
    opt_column_list_with(p, ColumnDefKind::Ref)
}

fn column_list(p: &mut Parser<'_>) {
    if !opt_column_list(p) {
        p.error("expected column list");
    }
}

fn opt_include_columns(p: &mut Parser<'_>) -> Option<CompletedMarker> {
    if p.at(INCLUDE_KW) {
        let m = p.start();
        p.bump(INCLUDE_KW);
        column_list(p);
        Some(m.complete(p, CONSTRAINT_INCLUDE_CLAUSE))
    } else {
        None
    }
}

// [ WITH ( storage_parameter [= value] [, ... ] ) ]
fn opt_with_params(p: &mut Parser<'_>) -> Option<CompletedMarker> {
    if p.at(WITH_KW) {
        let m = p.start();
        p.bump(WITH_KW);
        p.expect(L_PAREN);
        while !p.at(EOF) && !p.at(R_PAREN) {
            if !storage_parameter(p) {
                break;
            }
            if !p.eat(COMMA) {
                break;
            }
        }
        p.expect(R_PAREN);
        Some(m.complete(p, CONSTRAINT_STORAGE_PARAMS))
    } else {
        None
    }
}

// index_parameters in UNIQUE, PRIMARY KEY, and EXCLUDE constraints are:
// [ INCLUDE ( column_name [, ... ] ) ]
// [ WITH ( storage_parameter [= value] [, ... ] ) ]
// [ USING INDEX TABLESPACE tablespace_name ]
#[must_use]
fn opt_index_parameters(p: &mut Parser<'_>) -> bool {
    opt_include_columns(p);
    opt_with_params(p);
    if p.at(USING_KW) {
        let m = p.start();
        p.bump(USING_KW);
        p.expect(INDEX_KW);
        p.expect(TABLESPACE_KW);
        name_ref(p);
        m.complete(p, CONSTRAINT_INDEX_TABLESPACE);
    }
    true
}

// referential_action in a FOREIGN KEY/REFERENCES constraint is:
// { NO ACTION | RESTRICT | CASCADE | SET NULL [ ( column_name [, ... ] ) ] | SET DEFAULT [ ( column_name [, ... ] ) ] }
fn referential_action(p: &mut Parser<'_>) -> bool {
    if p.eat(NO_KW) {
        p.expect(ACTION_KW)
    } else if opt_cascade_or_restrict(p) {
        true
    } else if p.at(SET_KW) && p.nth_at(1, NULL_KW) {
        p.expect(SET_KW);
        p.expect(NULL_KW);
        opt_column_list(p);
        true
    } else if p.eat(SET_KW) {
        p.expect(DEFAULT_KW);
        if p.eat(L_PAREN) {
            // column_name [, ... ]
            while !p.at(EOF) && !p.at(COMMA) {
                name_ref(p);
                if !p.eat(COMMA) {
                    break;
                }
            }
            return p.expect(R_PAREN);
        }
        true
    } else {
        false
    }
}

const COLUMN_CONSTRAINT_FIRST: TokenSet = TokenSet::new(&[
    CONSTRAINT_KW,
    NOT_KW,
    NULL_KW,
    CHECK_KW,
    DEFAULT_KW,
    GENERATED_KW,
    UNIQUE_KW,
    PRIMARY_KW,
    REFERENCES_KW,
]);

// where column_constraint is:

// [ CONSTRAINT constraint_name ]
//
// { NOT NULL |
//   NULL |
//   CHECK ( expression ) [ NO INHERIT ] |
//   DEFAULT default_expr |
//   GENERATED ALWAYS AS ( generation_expr ) STORED |
//   GENERATED { ALWAYS | BY DEFAULT } AS IDENTITY [ ( sequence_options ) ] |
//   UNIQUE [ NULLS [ NOT ] DISTINCT ] index_parameters |
//   PRIMARY KEY index_parameters |
//   REFERENCES reftable [ ( refcolumn ) ] [ MATCH FULL | MATCH PARTIAL | MATCH SIMPLE ]
//     [ ON DELETE referential_action ] [ ON UPDATE referential_action ] }
//
// [ DEFERRABLE | NOT DEFERRABLE ] [ INITIALLY DEFERRED | INITIALLY IMMEDIATE ]
fn opt_column_constraint(p: &mut Parser<'_>) -> Option<CompletedMarker> {
    if !p.at_ts(COLUMN_CONSTRAINT_FIRST) {
        return None;
    }
    let m = p.start();
    if p.eat(CONSTRAINT_KW) {
        name_ref(p);
    }
    match opt_constraint_inner(p) {
        Some(kind) => {
            opt_constraint_options(p);
            Some(m.complete(p, kind))
        }
        None => {
            m.abandon(p);
            p.error("expected constraint type");
            None
        }
    }
}

// { NOT NULL |
//   NULL |
//   CHECK ( expression ) [ NO INHERIT ] |
//   DEFAULT default_expr |
//   GENERATED ALWAYS AS ( generation_expr ) STORED |
//   GENERATED { ALWAYS | BY DEFAULT } AS IDENTITY [ ( sequence_options ) ] |
//   UNIQUE [ NULLS [ NOT ] DISTINCT ] index_parameters |
//   PRIMARY KEY index_parameters |
//   REFERENCES reftable [ ( refcolumn ) ] [ MATCH FULL | MATCH PARTIAL | MATCH SIMPLE ]
//     [ ON DELETE referential_action ] [ ON UPDATE referential_action ] }
fn opt_constraint_inner(p: &mut Parser<'_>) -> Option<SyntaxKind> {
    let kind = match p.current() {
        NOT_KW => {
            p.bump(NOT_KW);
            p.expect(NULL_KW);
            NOT_NULL_CONSTRAINT
        }
        NULL_KW => {
            p.bump(NULL_KW);
            NULL_CONSTRAINT
        }
        CHECK_KW => {
            p.bump(CHECK_KW);
            p.expect(L_PAREN);
            // generation_expr
            // The generation expression can refer to other columns in the table,
            // but not other generated columns. Any functions and operators used
            // must be immutable. References to other tables are not allowed.
            if expr(p).is_none() {
                p.error("expected expression");
            }
            p.expect(R_PAREN);
            if p.eat(NO_KW) {
                p.expect(INHERIT_KW);
            }
            CHECK_CONSTRAINT
        }
        DEFAULT_KW => {
            p.bump(DEFAULT_KW);
            if expr(p).is_none() {
                p.error("expected expr for default");
            }
            DEFAULT_CONSTRAINT
        }
        GENERATED_KW => {
            p.bump(GENERATED_KW);
            // ALWAYS AS ( generation_expr ) STORED
            if p.at(ALWAYS_KW) && p.nth_at(1, AS_KW) && p.nth_at(2, L_PAREN) {
                p.expect(ALWAYS_KW);
                p.expect(AS_KW);
                p.expect(L_PAREN);
                if expr(p).is_none() {
                    p.error("expected an expression");
                }
                p.expect(R_PAREN);
                p.expect(STORED_KW);
                GENERATED_CONSTRAINT
            // { ALWAYS | BY DEFAULT } AS IDENTITY [ ( sequence_options ) ]
            } else if p.at(ALWAYS_KW) || p.at(BY_KW) {
                if p.eat(BY_KW) {
                    p.expect(DEFAULT_KW);
                } else {
                    p.expect(ALWAYS_KW);
                }
                p.expect(AS_KW);
                p.expect(IDENTITY_KW);
                opt_sequence_options(p);
                GENERATED_CONSTRAINT
            } else {
                p.error("expected generated type");
                return None;
            }
        }
        // UNIQUE [ NULLS [ NOT ] DISTINCT ] index_parameters
        UNIQUE_KW => {
            p.bump(UNIQUE_KW);
            if p.eat(NULLS_KW) {
                p.eat(NOT_KW);
                p.expect(DISTINCT_KW);
            }
            if !opt_index_parameters(p) {
                p.error("expected index parameters");
            }
            UNIQUE_CONSTRAINT
        }
        // PRIMARY KEY index_parameters
        PRIMARY_KW => {
            p.bump(PRIMARY_KW);
            p.expect(KEY_KW);
            if !opt_index_parameters(p) {
                p.error("expected index parameters");
            }
            PRIMARY_KEY_CONSTRAINT
        }
        // REFERENCES reftable [ ( refcolumn ) ] [ MATCH FULL | MATCH PARTIAL | MATCH SIMPLE ]
        //   [ ON DELETE referential_action ] [ ON UPDATE referential_action ] }
        REFERENCES_KW => {
            p.bump(REFERENCES_KW);
            path_name_ref(p);
            if p.eat(L_PAREN) {
                name_ref(p);
                p.expect(R_PAREN);
            }
            if p.eat(MATCH_KW) {
                if p.at(FULL_KW) || p.at(PARTIAL_KW) || p.at(SIMPLE_KW) {
                    p.bump_any();
                } else {
                    p.error("expected FULL, PARTIAL, or SIMPLE");
                }
            }
            // [ ON DELETE referential_action ]
            if p.at(ON_KW) && p.nth_at(1, DELETE_KW) {
                p.expect(ON_KW);
                p.expect(DELETE_KW);
                referential_action(p);
            }
            // [ ON UPDATE referential_action ]
            if p.at(ON_KW) && p.nth_at(1, UPDATE_KW) {
                p.expect(ON_KW);
                p.expect(UPDATE_KW);
                referential_action(p);
            }
            // [ ON DELETE referential_action ]
            if p.at(ON_KW) && p.nth_at(1, DELETE_KW) {
                p.expect(ON_KW);
                p.expect(DELETE_KW);
                referential_action(p);
            }
            REFERENCES_CONSTRAINT
        }
        _ => {
            return None;
        }
    };
    Some(kind)
}

const LIKE_OPTION: TokenSet = TokenSet::new(&[
    COMMENTS_KW,
    COMPRESSION_KW,
    CONSTRAINTS_KW,
    DEFAULTS_KW,
    GENERATED_KW,
    IDENTITY_KW,
    INDEXES_KW,
    STATISTICS_KW,
    STORAGE_KW,
    ALL_KW,
]);

// where like_option is:
//   { INCLUDING | EXCLUDING } { COMMENTS | COMPRESSION | CONSTRAINTS | DEFAULTS | GENERATED | IDENTITY | INDEXES | STATISTICS | STORAGE | ALL }
fn like_option(p: &mut Parser<'_>) -> bool {
    if p.at(INCLUDING_KW) || p.at(EXCLUDING_KW) {
        p.bump_any();
        if p.at_ts(LIKE_OPTION) {
            p.bump_any();
        } else {
            p.error("expected like option");
        }
        true
    } else {
        false
    }
}

// index_elem:
//  | ColId index_elem_options
//  | func_expr_windowless index_elem_options
//  | '(' a_expr ')' index_elem_options
fn index_elem(p: &mut Parser<'_>) {
    if p.eat(L_PAREN) {
        if expr(p).is_none() {
            p.error("expected an expression");
        }
        p.expect(R_PAREN);
    } else {
        if expr(p).is_none() {
            p.error("expected expression");
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum ColDefType {
    WithData,
    ColumnNameOnly,
}

fn opt_operator(p: &mut Parser<'_>) -> bool {
    let (power, kind, _) = current_op(p, &Restrictions::default());
    if power == 0 {
        if p.at_ts(OPERATOR_FIRST) {
            p.bump_any();
            return true;
        }
        return false;
    }
    p.eat(kind)
}

// optional schema supported
// >
// bar.>
// foo.bar.>
fn operator(p: &mut Parser<'_>) {
    opt_path_name_ref(p);
    if !opt_operator(p) {
        p.error(format!("expected operator, got {:?}", p.current()));
    }
}

pub(crate) fn current_operator(p: &Parser<'_>) -> Option<SyntaxKind> {
    let (power, kind, _) = current_op(p, &Restrictions::default());
    if power == 0 {
        None
    } else {
        Some(kind)
    }
}

fn using_index(p: &mut Parser<'_>) {
    let m = p.start();
    p.bump(USING_KW);
    p.expect(INDEX_KW);
    name_ref(p);
    m.complete(p, USING_INDEX);
}

const TABLE_CONSTRAINT_FIRST: TokenSet = TokenSet::new(&[
    CONSTRAINT_KW,
    CHECK_KW,
    UNIQUE_KW,
    PRIMARY_KW,
    EXCLUDE_KW,
    FOREIGN_KW,
]);

// and table_constraint is:
// [ CONSTRAINT constraint_name ]
// { CHECK ( expression ) [ NO INHERIT ] |
//   UNIQUE [ NULLS [ NOT ] DISTINCT ] ( column_name [, ... ] ) index_parameters |
//   PRIMARY KEY ( column_name [, ... ] ) index_parameters |
//   EXCLUDE [ USING index_method ] ( exclude_element WITH operator [, ... ] ) index_parameters [ WHERE ( predicate ) ] |
//   FOREIGN KEY ( column_name [, ... ] ) REFERENCES reftable [ ( refcolumn [, ... ] ) ]
//     [ MATCH FULL | MATCH PARTIAL | MATCH SIMPLE ] [ ON DELETE referential_action ] [ ON UPDATE referential_action ] }
// [ DEFERRABLE | NOT DEFERRABLE ] [ INITIALLY DEFERRED | INITIALLY IMMEDIATE ]
//
// and table_constraint_using_index is:
//   [ CONSTRAINT constraint_name ]
//   { UNIQUE | PRIMARY KEY } USING INDEX index_name
//   [ DEFERRABLE | NOT DEFERRABLE ] [ INITIALLY DEFERRED | INITIALLY IMMEDIATE ]
fn table_constraint(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at_ts(TABLE_CONSTRAINT_FIRST));
    let m = p.start();
    // [ CONSTRAINT constraint_name ]
    if p.eat(CONSTRAINT_KW) {
        name(p);
    }
    // CHECK ( expression ) [ NO INHERIT ]
    let kind = match p.current() {
        CHECK_KW => {
            p.bump(CHECK_KW);
            p.expect(L_PAREN);
            if expr(p).is_none() {
                p.error("expected expr");
            }
            p.expect(R_PAREN);
            if p.eat(NO_KW) {
                p.expect(INHERIT_KW);
            }
            CHECK_CONSTRAINT
        }
        // UNIQUE [ NULLS [ NOT ] DISTINCT ] ( column_name [, ... ] ) index_parameters
        // UNIQUE USING INDEX index_name
        UNIQUE_KW => {
            p.bump(UNIQUE_KW);
            // USING INDEX index_name
            if p.at(USING_KW) {
                using_index(p);
            // [ NULLS [ NOT ] DISTINCT ] ( column_name [, ... ] ) index_parameters
            } else {
                if p.eat(NULLS_KW) {
                    p.eat(NOT_KW);
                    p.eat(DISTINCT_KW);
                }
                column_list(p);
                if !opt_index_parameters(p) {
                    p.error("expected index parameters");
                }
            }
            UNIQUE_CONSTRAINT
        }
        // PRIMARY KEY ( column_name [, ... ] ) index_parameters
        // PRIMARY KEY USING INDEX index_name
        PRIMARY_KW => {
            p.bump(PRIMARY_KW);
            p.expect(KEY_KW);
            // USING INDEX index_name
            if p.at(USING_KW) {
                using_index(p);
            // ( column_name [, ... ] ) index_parameters
            } else {
                column_list(p);
                if !opt_index_parameters(p) {
                    p.error("expected index parameters");
                }
            }
            PRIMARY_KEY_CONSTRAINT
        }
        // EXCLUDE [ USING index_method ] ( exclude_element WITH operator [, ... ] ) index_parameters [ WHERE ( predicate ) ] |
        EXCLUDE_KW => {
            p.bump(EXCLUDE_KW);
            if p.at(USING_KW) {
                let m = p.start();
                p.bump(USING_KW);
                name_ref(p);
                m.complete(p, CONSTRAINT_INDEX_METHOD);
            }
            let m = p.start();
            p.expect(L_PAREN);
            while !p.at(EOF) && !p.at(R_PAREN) {
                index_elem(p);
                p.expect(WITH_KW);
                // support:
                // with >
                // with foo.bar.buzz.>
                operator(p);
                if !p.eat(COMMA) {
                    break;
                }
            }
            p.expect(R_PAREN);
            m.complete(p, CONSTRAINT_EXCLUSIONS);
            if !opt_index_parameters(p) {
                p.error("expected index parameters");
            }
            if p.at(WHERE_KW) {
                let m = p.start();
                p.bump(WHERE_KW);
                p.expect(L_PAREN);
                if expr(p).is_none() {
                    p.error("expected expr");
                }
                p.expect(R_PAREN);
                m.complete(p, CONSTRAINT_WHERE_CLAUSE);
            }
            EXCLUDE_CONSTRAINT
        }
        // FOREIGN KEY ( column_name [, ... ] ) REFERENCES reftable [ ( refcolumn [, ... ] ) ]
        //   [ MATCH FULL | MATCH PARTIAL | MATCH SIMPLE ] [ ON DELETE referential_action ] [ ON UPDATE referential_action ] }
        _ => {
            // must be in a foreign key constraint
            p.expect(FOREIGN_KW);
            p.expect(KEY_KW);
            column_list(p);
            p.expect(REFERENCES_KW);
            path_name_ref(p);
            if p.eat(L_PAREN) {
                while !p.at(EOF) && !p.at(COMMA) {
                    let found_period = p.eat(PERIOD_KW);
                    name_ref(p);
                    if found_period {
                        break;
                    }
                    if !p.eat(COMMA) {
                        break;
                    }
                }
                p.expect(R_PAREN);
            }
            if p.eat(MATCH_KW) {
                if p.at(FULL_KW) || p.at(PARTIAL_KW) || p.at(SIMPLE_KW) {
                    p.bump_any();
                } else {
                    p.error("expected FULL, PARTIAL, or SIMPLE");
                }
            }
            // [ ON DELETE referential_action ]
            if p.eat(ON_KW) {
                p.expect(DELETE_KW);
                referential_action(p);
            }
            // [ ON UPDATE referential_action ]
            if p.eat(ON_KW) {
                p.expect(UPDATE_KW);
                referential_action(p);
            }
            FOREIGN_KEY_CONSTRAINT
        }
    };
    opt_constraint_options(p);
    m.complete(p, kind)
}

fn opt_without_overlaps(p: &mut Parser<'_>) {
    if p.eat(WITHOUT_KW) {
        p.expect(OVERLAPS_KW);
    }
}

fn opt_deferrable_constraint_option(p: &mut Parser<'_>) -> Option<CompletedMarker> {
    let m = p.start();
    let kind = match (p.current(), p.nth(1)) {
        (DEFERRABLE_KW, _) => {
            p.bump(DEFERRABLE_KW);
            DEFERRABLE_CONSTRAINT_OPTION
        }
        (NOT_KW, DEFERRABLE_KW) => {
            p.bump(NOT_KW);
            p.bump(DEFERRABLE_KW);
            NOT_DEFERRABLE_CONSTRAINT_OPTION
        }
        _ => {
            m.abandon(p);
            return None;
        }
    };
    Some(m.complete(p, kind))
}

fn opt_initally_constraint_option(p: &mut Parser<'_>) -> Option<CompletedMarker> {
    let m = p.start();
    let kind = match (p.current(), p.nth(1)) {
        (INITIALLY_KW, DEFERRED_KW) => {
            p.bump(INITIALLY_KW);
            p.bump(DEFERRED_KW);
            INITALLY_DEFERRED_CONSTRAINT_OPTION
        }
        (INITIALLY_KW, IMMEDIATE_KW) => {
            p.bump(INITIALLY_KW);
            p.bump(IMMEDIATE_KW);
            INITIALLY_IMMEDIATE_CONSTRAINT_OPTION
        }
        _ => {
            m.abandon(p);
            return None;
        }
    };
    Some(m.complete(p, kind))
}

// [ NOT DEFERRABLE | [ DEFERRABLE ] [ INITIALLY IMMEDIATE | INITIALLY DEFERRED ] ]
fn opt_constraint_options(p: &mut Parser<'_>) {
    let m = p.start();
    let deferrable = opt_deferrable_constraint_option(p);
    let initially = opt_initally_constraint_option(p);

    match (deferrable, initially) {
        (None, None) => {
            m.abandon(p);
            return;
        }
        (Some(deferrable), Some(initially)) => {
            if deferrable.kind() == NOT_DEFERRABLE_CONSTRAINT_OPTION
                && initially.kind() == INITALLY_DEFERRED_CONSTRAINT_OPTION
            {
                p.error("constraint declared INITIALLY DEFERRED must be DEFERRABLE");
            }
        }
        (_, _) => (),
    }
    m.complete(p, CONSTRAINT_OPTION_LIST);
}

const COLUMN_NAME_KEYWORDS: TokenSet = TokenSet::new(&[
    BETWEEN_KW,
    BIGINT_KW,
    BIT_KW,
    BOOLEAN_KW,
    CHAR_KW,
    CHARACTER_KW,
    COALESCE_KW,
    DEC_KW,
    DECIMAL_KW,
    EXISTS_KW,
    EXTRACT_KW,
    FLOAT_KW,
    GREATEST_KW,
    GROUPING_KW,
    INOUT_KW,
    INT_KW,
    INTEGER_KW,
    INTERVAL_KW,
    JSON_KW,
    JSON_ARRAY_KW,
    JSON_ARRAYAGG_KW,
    JSON_EXISTS_KW,
    JSON_OBJECT_KW,
    JSON_OBJECTAGG_KW,
    JSON_QUERY_KW,
    JSON_SCALAR_KW,
    JSON_SERIALIZE_KW,
    JSON_TABLE_KW,
    JSON_VALUE_KW,
    LEAST_KW,
    MERGE_ACTION_KW,
    NATIONAL_KW,
    NCHAR_KW,
    NONE_KW,
    NORMALIZE_KW,
    NULLIF_KW,
    NUMERIC_KW,
    OUT_KW,
    OVERLAY_KW,
    POSITION_KW,
    PRECISION_KW,
    REAL_KW,
    ROW_KW,
    SETOF_KW,
    SMALLINT_KW,
    SUBSTRING_KW,
    TIME_KW,
    TIMESTAMP_KW,
    TREAT_KW,
    TRIM_KW,
    VALUES_KW,
    VARCHAR_KW,
    XMLATTRIBUTES_KW,
    XMLCONCAT_KW,
    XMLELEMENT_KW,
    XMLEXISTS_KW,
    XMLFOREST_KW,
    XMLNAMESPACES_KW,
    XMLPARSE_KW,
    XMLPI_KW,
    XMLROOT_KW,
    XMLSERIALIZE_KW,
    XMLTABLE_KW,
]);

// column_name data_type [ STORAGE { PLAIN | EXTERNAL | EXTENDED | MAIN | DEFAULT } ] [ COMPRESSION compression_method ] [ COLLATE collation ] [ column_constraint [ ... ] ]
//   { column_name data_type [ STORAGE { PLAIN | EXTERNAL | EXTENDED | MAIN | DEFAULT } ] [ COMPRESSION compression_method ] [ COLLATE collation ] [ column_constraint [ ... ] ]
//     | table_constraint
//     | LIKE source_table [ like_option ... ] }
fn col_def(p: &mut Parser<'_>, t: ColDefType) -> Option<CompletedMarker> {
    if !(p.at(LIKE_KW)
        || p.at_ts(TABLE_CONSTRAINT_FIRST)
        // ColId
        || p.at_ts(NAME_FIRST))
    {
        return None;
    }
    let m = p.start();
    // LIKE source_table [ like_option ... ]
    if t == ColDefType::WithData && p.eat(LIKE_KW) {
        path_name_ref(p);
        while !p.at(EOF) {
            if !like_option(p) {
                break;
            }
        }
        return Some(m.complete(p, LIKE_CLAUSE));
    }
    if p.at_ts(TABLE_CONSTRAINT_FIRST) {
        m.abandon(p);
        return Some(table_constraint(p));
    }
    name_ref(p);
    match t {
        ColDefType::WithData => {
            type_name(p);
            // [ STORAGE { PLAIN | EXTERNAL | EXTENDED | MAIN | DEFAULT } ]
            if p.eat(STORAGE_KW) && (p.at(DEFAULT_KW) || p.at(IDENT)) {
                p.bump_any();
            }
            // [ COMPRESSION compression_method ]
            if p.eat(COMPRESSION_KW) && (p.at(DEFAULT_KW) || p.at(IDENT)) {
                p.bump_any();
            }
            opt_collate(p);
        }
        ColDefType::ColumnNameOnly => {
            // [ WITH OPTIONS ]
            if p.eat(WITH_KW) {
                p.expect(OPTIONS_KW);
            }
        }
    }
    // [ column_constraint [ ... ] ]
    while !p.at(EOF) {
        if opt_column_constraint(p).is_none() {
            break;
        }
    }
    Some(m.complete(p, COLUMN))
}

// [ AS ] alias [ ( column_alias [, ...] ) ]
// [ AS ] alias ( column_definition [, ...] )
// AS ( column_definition [, ...] )
fn opt_alias(p: &mut Parser<'_>) -> Option<CompletedMarker> {
    // TODO: we should split this into opt_col_def and opt_col_alias
    if !(p.at(AS_KW) || p.at_ts(NAME_FIRST) || p.at(L_PAREN)) {
        return None;
    }
    let m = p.start();
    p.eat(AS_KW);
    // table_name(col1, col2, col3)
    opt_name(p);
    if p.at(L_PAREN) {
        if !opt_column_list_with(p, ColumnDefKind::WithData) {
            p.error("expected column list");
        }
    }
    Some(m.complete(p, ALIAS))
}

fn opt_where_clause(p: &mut Parser<'_>) -> bool {
    if !p.at(WHERE_KW) {
        return false;
    }
    let m = p.start();
    p.bump(WHERE_KW);
    if expr(p).is_none() {
        p.error("expected an expression");
    }
    m.complete(p, WHERE_CLAUSE);
    true
}

/// <https://www.postgresql.org/docs/current/sql-select.html#SQL-GROUPBY>
fn opt_group_by_clause(p: &mut Parser<'_>) -> bool {
    let m = p.start();
    if !p.eat(GROUP_KW) {
        m.abandon(p);
        return false;
    }
    p.expect(BY_KW);
    if p.at(ALL_KW) || p.at(DISTINCT_KW) {
        p.bump_any();
    }
    // From pg docs:
    // An expression used inside a grouping_element can be an input column name,
    // or the name or ordinal number of an output column (SELECT list item), or
    // an arbitrary expression formed from input-column values. In case of
    // ambiguity, a GROUP BY name will be interpreted as an input-column name
    // rather than an output column name.
    while !p.at(EOF) && !p.at(SEMICOLON) {
        // TODO: handle errors?
        p.eat(ROLLUP_KW);
        p.eat(CUBE_KW);
        if p.eat(GROUPING_KW) {
            p.expect(SETS_KW);
        }
        if expr(p).is_none() {
            p.error("expected an expression");
        }
        if !p.eat(COMMA) {
            break;
        }
    }
    m.complete(p, GROUP_BY_CLAUSE);
    true
}

/// <https://www.postgresql.org/docs/17/sql-select.html#SQL-HAVING>
fn opt_having_clause(p: &mut Parser<'_>) -> bool {
    if !p.at(HAVING_KW) {
        return false;
    }
    let m = p.start();
    p.bump(HAVING_KW);
    if expr(p).is_none() {
        p.error("expected an expression");
    }
    m.complete(p, HAVING_CLAUSE);
    true
}

// frame_start and frame_end can be one of
// UNBOUNDED PRECEDING
// UNBOUNDED FOLLOWING
// CURRENT ROW
// offset PRECEDING
// offset FOLLOWING
fn frame_start_end(p: &mut Parser<'_>) {
    if p.eat(UNBOUNDED_KW) {
        if p.at(PRECEDING_KW) || p.at(FOLLOWING_KW) {
            p.bump_any();
        } else {
            p.err_and_bump("expected preceding or following");
        }
    } else if p.eat(CURRENT_KW) {
        p.expect(ROW_KW);
    } else {
        if expr(p).is_none() {
            p.error("expected an expression");
        }
        if p.at(PRECEDING_KW) || p.at(FOLLOWING_KW) {
            p.bump_any();
        } else {
            p.err_and_bump("expected preceding or following");
        }
    }
}

// and frame_exclusion can be one of
// EXCLUDE CURRENT ROW
// EXCLUDE GROUP
// EXCLUDE TIES
// EXCLUDE NO OTHERS
fn opt_frame_exclusion(p: &mut Parser<'_>) -> bool {
    if !p.eat(EXCLUDE_KW) {
        return false;
    }
    if p.eat(CURRENT_KW) {
        p.expect(ROW_KW)
    } else if p.eat(NO_KW) {
        p.expect(OTHERS_KW)
    } else if p.at(GROUP_KW) || p.at(TIES_KW) {
        p.bump_any();
        true
    } else {
        p.err_and_bump("expected `group`, `current row`, `ties`, or `no others`");
        false
    }
}

const WINDOW_DEF_START: TokenSet =
    TokenSet::new(&[IDENT, PARTITION_KW, ORDER_KW, RANGE_KW, ROWS_KW, GROUPS_KW]);

// window_definition is
// [ existing_window_name ]
// [ PARTITION BY expression [, ...] ]
// [ ORDER BY expression [ ASC | DESC | USING operator ] [ NULLS { FIRST | LAST } ] [, ...] ]
// [ frame_clause ]
//
// The frame_clause can be one of
// { RANGE | ROWS | GROUPS } frame_start [ frame_exclusion ]
// { RANGE | ROWS | GROUPS } BETWEEN frame_start AND frame_end [ frame_exclusion ]
fn window_definition(p: &mut Parser<'_>) -> bool {
    if !p.at_ts(WINDOW_DEF_START) {
        return false;
    }
    let m = p.start();
    p.eat(IDENT);
    if p.eat(PARTITION_KW) {
        p.expect(BY_KW);
        if expr(p).is_none() {
            p.error("expected an expression");
        }
    }
    opt_order_by_clause(p);
    if p.at(RANGE_KW) || p.at(ROWS_KW) || p.at(GROUPS_KW) {
        p.bump_any();
        if p.eat(BETWEEN_KW) {
            frame_start_end(p);
            p.expect(AND_KW);
            frame_start_end(p);
            opt_frame_exclusion(p);
        } else {
            frame_start_end(p);
            opt_frame_exclusion(p);
        }
    }
    m.complete(p, WINDOW_DEF);
    true
}

/// <https://www.postgresql.org/docs/17/sql-select.html#SQL-WINDOW>
fn opt_window_clause(p: &mut Parser<'_>) -> bool {
    if !p.at(WINDOW_KW) {
        return false;
    }
    let m = p.start();
    p.bump(WINDOW_KW);
    p.expect(IDENT);
    p.expect(AS_KW);
    p.expect(L_PAREN);
    window_definition(p);
    p.expect(R_PAREN);
    m.complete(p, WINDOW_CLAUSE);
    true
}

// [ LIMIT { count | ALL } ]
fn opt_limit_clause(p: &mut Parser<'_>) -> bool {
    let m = p.start();
    if !p.eat(LIMIT_KW) {
        m.abandon(p);
        return false;
    }
    if !p.eat(ALL_KW) && expr(p).is_none() {
        p.error("expected an expression");
    }
    m.complete(p, LIMIT_CLAUSE);
    true
}

// [ OFFSET start [ ROW | ROWS ] ]
fn opt_offset_clause(p: &mut Parser<'_>) -> bool {
    let m = p.start();
    if !p.eat(OFFSET_KW) {
        m.abandon(p);
        return false;
    }
    if expr(p).is_none() {
        p.error("expected an expression");
    }
    if p.at(ROW_KW) || p.at(ROWS_KW) {
        p.bump_any();
    }
    m.complete(p, OFFSET_CLAUSE);
    true
}

/// all is the default, distinct removes duplicate rows
fn opt_all_or_distinct(p: &mut Parser) {
    // TODO: we probably don't want to be so specific here, we can be more
    // generous with parsing and handle error reporting later on.
    if p.at(ALL_KW) {
        p.bump(ALL_KW);
    } else if p.at(DISTINCT_KW) {
        let m = p.start();
        // ```
        // select DISTINCT [ ON ( expression [, ...] ) ]
        // ```
        //
        // for example:
        //
        //  ```
        //  select distinct name from users
        //  ```
        //
        //  or
        //
        //  ```
        //  select distinct on (url) url, request_duration
        //  from logs
        //  order by url, timestamp desc
        //  ```
        //
        // from: [pg docs](https://www.postgresql.org/docs/current/sql-select.html#SQL-DISTINCT)
        //
        // `SELECT DISTINCT ON ( expression [, ...] )` keeps only the first row of
        // each set of rows where the given expressions evaluate to equal. The
        // DISTINCT ON expressions are interpreted using the same rules as for
        // ORDER BY (see above). Note that the “first row” of each set is
        // unpredictable unless ORDER BY is used to ensure that the desired row
        // appears first.
        p.bump(DISTINCT_KW);
        if p.eat(ON_KW) && !paren_expr_list(p) {
            m.abandon(p);
            return;
        }
        m.complete(p, DISTINCT_CLAUSE);
    }
}

fn paren_expr_list(p: &mut Parser<'_>) -> bool {
    if !p.eat(L_PAREN) {
        return false;
    }
    if !expr_list(p) {
        p.error("expected expression in paren_expr_list");
        false
    } else {
        p.eat(R_PAREN);
        true
    }
}

/// All keywords
const COL_LABEL_FIRST: TokenSet = TokenSet::new(&[IDENT])
    .union(UNRESERVED_KEYWORDS)
    .union(COLUMN_NAME_KEYWORDS)
    .union(TYPE_FUNC_NAME_KEYWORDS)
    .union(RESERVED_KEYWORDS);

const NAME_FIRST: TokenSet = TokenSet::new(&[IDENT])
    .union(UNRESERVED_KEYWORDS)
    .union(COLUMN_NAME_KEYWORDS);

const BARE_COL_LABEL_FIRST: TokenSet = TokenSet::new(&[IDENT]).union(BARE_LABEL_KEYWORDS);

const TARGET_LIST_START: TokenSet = TokenSet::new(&[STAR])
    .union(COL_LABEL_FIRST)
    .union(EXPR_FIRST)
    .union(TYPE_KEYWORDS);

const LITERAL_FIRST: TokenSet = TokenSet::new(&[
    TRUE_KW, FALSE_KW, NULL_KW, // TODO: should default be in this set?
    DEFAULT_KW,
])
.union(NUMERIC_FIRST)
.union(STRING_FIRST);

const NUMERIC_FIRST: TokenSet = TokenSet::new(&[INT_NUMBER, FLOAT_NUMBER]);

const STRING_FIRST: TokenSet = TokenSet::new(&[
    STRING,
    BYTE_STRING,
    BIT_STRING,
    DOLLAR_QUOTED_STRING,
    ESC_STRING,
]);

// via https://www.postgresql.org/docs/17/sql-createoperator.html
// + - * / < > = ~ ! @ # % ^ & | ` ?
pub(crate) const OPERATOR_FIRST: TokenSet = TokenSet::new(&[
    PLUS, MINUS, STAR, SLASH, L_ANGLE, R_ANGLE, EQ, TILDE, BANG, AT, POUND, PERCENT, CARET, AMP,
    PIPE, BACKTICK, QUESTION,
]);

const LHS_FIRST: TokenSet = TokenSet::new(&[
    L_PAREN, L_BRACK, CAST_KW, NOT_KW, IS_KW, PARAM, CASE_KW, ARRAY_KW, ROW_KW, DEFAULT_KW,
])
.union(OPERATOR_FIRST)
.union(LITERAL_FIRST)
.union(TYPE_KEYWORDS)
.union(IDENTS);

const IDENTS: TokenSet = TokenSet::new(&[ANY_KW, ALL_KW, SOME_KW, IDENT]).union(FUNC_KEYWORDS);

const FUNC_KEYWORDS: TokenSet = TokenSet::new(&[
    CURRENT_DATE_KW,
    CURRENT_TIME_KW,
    CURRENT_TIMESTAMP_KW,
    LOCALTIME_KW,
    LOCALTIMESTAMP_KW,
    CURRENT_ROLE_KW,
    CURRENT_USER_KW,
    SESSION_USER_KW,
    SYSTEM_USER_KW,
    USER_KW,
    CURRENT_CATALOG_KW,
    CURRENT_SCHEMA_KW,
]);

const NAME_REF_FIRST: TokenSet = TYPE_KEYWORDS.union(IDENTS);

const EXPR_FIRST: TokenSet = LHS_FIRST;

// TODO: is this right? copied from rust analyzer
const ATTRIBUTE_FIRST: TokenSet = TokenSet::new(&[POUND, GROUP_KW]);

const TARGET_FOLLOW: TokenSet = TokenSet::new(&[
    FROM_KW, WHERE_KW, LIMIT_KW, ORDER_KW, OFFSET_KW, GROUP_KW, HAVING_KW, WINDOW_KW, HAVING_KW,
    FETCH_KW, FOR_KW, R_PAREN, R_BRACK,
])
.union(COMPOUND_SELECT_FIRST);

const TARGET_FIRST: TokenSet = EXPR_FIRST;

// target_el:
//   | a_expr AS ColLabel
//   | a_expr BareColLabel
//   | a_expr
//   | '*'
fn opt_target_el(p: &mut Parser) -> Option<CompletedMarker> {
    let m = p.start();
    if p.at_ts(TARGET_FOLLOW) {
        m.abandon(p);
        return None;
    } else if p.at(STAR) && !p.nth_at_ts(1, OPERATOR_FIRST) {
        p.bump(STAR);
        true
    } else if expr(p).is_some() {
        opt_as_col_label(p) || p.at(COMMA)
    } else {
        m.abandon(p);
        p.error(format!(
            "expected an expression in target_el, found {:?}",
            p.current()
        ));
        return None;
    };
    Some(m.complete(p, TARGET))
}

fn opt_as_col_label(p: &mut Parser<'_>) -> bool {
    if p.eat(AS_KW) {
        if p.at_ts(COL_LABEL_FIRST) {
            col_label(p);
            true
        } else {
            p.err_and_bump(&format!("expected column label, got {:?}", p.current()));
            false
        }
    } else {
        if p.at(FORMAT_KW) && p.nth_at(1, JSON_KW) {
            return true;
        }
        if p.at_ts(BARE_COL_LABEL_FIRST) {
            col_label(p);
        }
        true
    }
}

fn opt_target_list(p: &mut Parser) -> Option<CompletedMarker> {
    if !p.at_ts(TARGET_LIST_START) {
        return None;
    }
    let m = p.start();
    while !p.at(EOF) && !p.at(SEMICOLON) {
        if opt_target_el(p).is_some() {
            if p.at(COMMA) && matches!(p.nth(1), SEMICOLON | EOF) {
                p.err_and_bump("unexpected trailing comma");
                break;
            }
            if !p.eat(COMMA) {
                if p.at(FORMAT_KW) && p.nth_at(1, JSON_KW) {
                    break;
                }
                if p.at(RETURNING_KW) && p.nth_at(1, TEXT_KW) {
                    break;
                }
                if p.at_ts(TARGET_FIRST) {
                    p.error("missing comma");
                } else {
                    break;
                }
            }
        }
    }
    Some(m.complete(p, TARGET_LIST))
}

fn opt_if_not_exists(p: &mut Parser<'_>) -> Option<CompletedMarker> {
    if p.at(IF_KW) {
        let m = p.start();
        p.bump(IF_KW);
        p.expect(NOT_KW);
        p.expect(EXISTS_KW);
        Some(m.complete(p, IF_NOT_EXISTS))
    } else {
        None
    }
}

fn opt_if_exists(p: &mut Parser<'_>) -> Option<CompletedMarker> {
    if p.at(IF_KW) {
        let m = p.start();
        p.bump(IF_KW);
        p.expect(EXISTS_KW);
        Some(m.complete(p, IF_EXISTS))
    } else {
        None
    }
}

// DROP TABLE [ IF EXISTS ] name [, ...] [ CASCADE | RESTRICT ]
/// <https://www.postgresql.org/docs/17/sql-droptable.html>
fn drop_table_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(DROP_KW) && p.nth_at(1, TABLE_KW));
    let m = p.start();
    p.bump(DROP_KW);
    p.bump(TABLE_KW);
    opt_if_exists(p);
    while !p.at(EOF) {
        path_name_ref(p);
        if !p.eat(COMMA) {
            break;
        }
    }
    opt_cascade_or_restrict(p);
    m.complete(p, DROP_TABLE)
}

//   { column_name | ( expression ) }
//   [ COLLATE collation ]
//   [ opclass ]
//
// if we pass allow_extra_params:
//   { column_name | ( expression ) }
//   [ COLLATE collation ]
//   [ opclass [ ( opclass_parameter = value [, ... ] ) ] ]
//   [ ASC | DESC ]
//   [ NULLS { FIRST | LAST } ]
//
// part_elem:
//  | ColId opt_collate opt_qualified_name
//  | func_expr_windowless opt_collate opt_qualified_name
//  | '(' a_expr ')' opt_collate opt_qualified_name
fn part_elem(p: &mut Parser<'_>, allow_extra_params: bool) -> bool {
    // TODO: this can be more strict
    if expr(p).is_none() {
        p.error("expected expr")
    }
    opt_collate(p);
    // [ opclass ]
    p.eat(IDENT);
    if allow_extra_params {
        // [ ( opclass_parameter = value [, ... ] ) ]
        if p.eat(L_PAREN) {
            // TODO:
            while !p.at(EOF) {
                if !attribute_option(p, AttributeValue::Required) {
                    break;
                }
                if !p.eat(COMMA) {
                    break;
                }
            }
            p.expect(R_PAREN);
        }
        // [ ASC | DESC ]
        let _ = p.eat(ASC_KW) || p.eat(DESC_KW);
        // [ NULLS { FIRST | LAST } ]
        if p.eat(NULLS_KW) {
            let _ = p.eat(FIRST_KW) || p.expect(LAST_KW);
        }
    }
    true
}

fn table_args(p: &mut Parser<'_>, t: ColDefType) -> Option<CompletedMarker> {
    let m = p.start();
    match t {
        ColDefType::WithData => {
            p.expect(L_PAREN);
        }
        ColDefType::ColumnNameOnly => {
            if !p.eat(L_PAREN) {
                m.abandon(p);
                return None;
            }
        }
    }
    while !p.at(EOF) && !p.at(R_PAREN) {
        col_def(p, t);
        if p.at(COMMA) && p.nth_at(1, R_PAREN) {
            p.err_and_bump("unexpected trailing comma");
            break;
        }
        if !p.eat(COMMA) {
            break;
        }
    }
    p.expect(R_PAREN);
    Some(m.complete(p, TABLE_ARGS))
}

// { FOR VALUES partition_bound_spec | DEFAULT }
fn partition_option(p: &mut Parser<'_>) {
    if p.eat(FOR_KW) {
        p.expect(VALUES_KW);
        // FOR VALUES WITH (modulus 5, remainder 0)
        if p.eat(WITH_KW) {
            p.expect(L_PAREN);
            p.expect(IDENT);
            p.expect(INT_NUMBER);
            p.expect(COMMA);
            p.expect(IDENT);
            p.expect(INT_NUMBER);
            p.expect(R_PAREN);
        // FOR VALUES IN '(' expr_list ')'
        } else if p.eat(IN_KW) {
            p.expect(L_PAREN);
            if !expr_list(p) {
                p.error("expected expr list");
            }
            p.expect(R_PAREN);
        // FOR VALUES FROM '(' expr_list ')' TO '(' expr_list ')'
        } else if p.eat(FROM_KW) {
            p.expect(L_PAREN);
            if !expr_list(p) {
                p.error("expected expr list");
            }
            p.expect(R_PAREN);
            p.expect(TO_KW);
            p.expect(L_PAREN);
            if !expr_list(p) {
                p.error("expected expr list");
            }
            p.expect(R_PAREN);
        }
    // DEFAULT
    } else {
        p.expect(DEFAULT_KW);
    }
}

fn opt_inherits_tables(p: &mut Parser<'_>) {
    if p.eat(INHERITS_KW) {
        p.expect(L_PAREN);
        while !p.at(EOF) && !p.at(COMMA) {
            path_name_ref(p);
            if !p.eat(COMMA) {
                break;
            }
        }
        p.expect(R_PAREN);
    }
}

// CREATE [ [ GLOBAL | LOCAL ] { TEMPORARY | TEMP } | UNLOGGED ] TABLE [ IF NOT EXISTS ] table_name ( [
//   { column_name data_type [ STORAGE { PLAIN | EXTERNAL | EXTENDED | MAIN | DEFAULT } ] [ COMPRESSION compression_method ] [ COLLATE collation ] [ column_constraint [ ... ] ]
//     | table_constraint
//     | LIKE source_table [ like_option ... ] }
//     [, ... ]
// ] )
// [ INHERITS ( parent_table [, ... ] ) ]
// [ PARTITION BY { RANGE | LIST | HASH } ( { column_name | ( expression ) } [ COLLATE collation ] [ opclass ] [, ... ] ) ]
// [ USING method ]
// [ WITH ( storage_parameter [= value] [, ... ] ) | WITHOUT OIDS ]
// [ ON COMMIT { PRESERVE ROWS | DELETE ROWS | DROP } ]
// [ TABLESPACE tablespace_name ]
//
// CREATE [ [ GLOBAL | LOCAL ] { TEMPORARY | TEMP } | UNLOGGED ] TABLE [ IF NOT EXISTS ] table_name
//     OF type_name [ (
//   { column_name [ WITH OPTIONS ] [ column_constraint [ ... ] ]
//     | table_constraint }
//     [, ... ]
// ) ]
// [ PARTITION BY { RANGE | LIST | HASH } ( { column_name | ( expression ) } [ COLLATE collation ] [ opclass ] [, ... ] ) ]
// [ USING method ]
// [ WITH ( storage_parameter [= value] [, ... ] ) | WITHOUT OIDS ]
// [ ON COMMIT { PRESERVE ROWS | DELETE ROWS | DROP } ]
// [ TABLESPACE tablespace_name ]
//
// CREATE [ [ GLOBAL | LOCAL ] { TEMPORARY | TEMP } | UNLOGGED ] TABLE [ IF NOT EXISTS ] table_name
//     PARTITION OF parent_table [ (
//   { column_name [ WITH OPTIONS ] [ column_constraint [ ... ] ]
//     | table_constraint }
//     [, ... ]
// ) ] { FOR VALUES partition_bound_spec | DEFAULT }
// [ PARTITION BY { RANGE | LIST | HASH } ( { column_name | ( expression ) } [ COLLATE collation ] [ opclass ] [, ... ] ) ]
// [ USING method ]
// [ WITH ( storage_parameter [= value] [, ... ] ) | WITHOUT OIDS ]
// [ ON COMMIT { PRESERVE ROWS | DELETE ROWS | DROP } ]
// [ TABLESPACE tablespace_name ]
fn create_table_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(CREATE_KW));
    let m = p.start();
    p.expect(CREATE_KW);
    // [ [ GLOBAL | LOCAL ] { TEMPORARY | TEMP } | UNLOGGED ]
    if !p.eat(UNLOGGED_KW) {
        // [ GLOBAL | LOCAL ] { TEMPORARY | TEMP }
        let require_temp = p.eat(GLOBAL_KW) || p.eat(LOCAL_KW);
        if require_temp {
            if !opt_temp(p) {
                p.error("expected temp or temporary");
            }
        } else {
            opt_temp(p);
        }
    }
    p.expect(TABLE_KW);
    opt_if_not_exists(p);
    path_name(p);
    let mut col_def_t = ColDefType::WithData;
    let mut is_partition = false;
    // OF type_name
    if p.eat(OF_KW) {
        simple_type_name(p);
        col_def_t = ColDefType::ColumnNameOnly;
    // PARTITION OF parent_table
    } else if p.eat(PARTITION_KW) {
        p.expect(OF_KW);
        path_name_ref(p);
        col_def_t = ColDefType::ColumnNameOnly;
        is_partition = true;
    }
    if p.at(L_PAREN) {
        table_args(p, col_def_t);
    }
    if is_partition {
        partition_option(p);
    }
    // [ INHERITS ( parent_table [, ... ] ) ]
    if col_def_t == ColDefType::WithData {
        opt_inherits_tables(p);
    }
    // [ PARTITION BY { RANGE | LIST | HASH } ( { column_name | ( expression ) } [ COLLATE collation ] [ opclass ] [, ... ] ) ]
    if p.eat(PARTITION_KW) {
        p.expect(BY_KW);
        // name
        if p.at_ts(TYPE_KEYWORDS) || p.at(IDENT) {
            p.bump_any();
        }
        // (
        //   { column_name | ( expression ) }
        //   [ COLLATE collation ]
        //   [ opclass ]
        //   [, ... ]
        // )
        partition_items(p, false);
    }
    // [ USING method ]
    if p.eat(USING_KW) {
        path_name_ref(p);
    }
    // [ WITH ( storage_parameter [= value] [, ... ] ) | WITHOUT OIDS ]
    if opt_with_params(p).is_none() {
        if p.eat(WITHOUT_KW) {
            p.expect(OIDS_KW);
        }
    }
    // [ ON COMMIT { PRESERVE ROWS | DELETE ROWS | DROP } ]
    if p.eat(ON_KW) {
        p.expect(COMMIT_KW);
        if p.eat(PRESERVE_KW) || p.eat(DELETE_KW) {
            p.expect(ROWS_KW);
        } else if !p.eat(DROP_KW) {
            p.error("expected PRESERVE ROWS, DELETE ROWS, or DROP");
        }
    }
    // [ TABLESPACE tablespace_name ]
    if p.eat(TABLESPACE_KW) {
        name_ref(p);
    }
    // AS query
    // [ WITH [ NO ] DATA ]
    if p.eat(AS_KW) {
        // query
        if p.at_ts(SELECT_FIRST) {
            select_stmt(p, None);
        } else if p.at(EXECUTE_KW) {
            execute_stmt(p);
        } else {
            p.error("expected SELECT, TABLE, VALUES, or EXECUTE");
        }
        if p.eat(WITH_KW) {
            p.eat(NO_KW);
            p.expect(DATA_KW);
        }
        return m.complete(p, CREATE_TABLE_AS_STMT);
    }
    m.complete(p, CREATE_TABLE)
}

// COMMIT [ WORK | TRANSACTION ] [ AND [ NO ] CHAIN ]
// COMMIT PREPARED transaction_id
//
// https://www.postgresql.org/docs/17/sql-commit.html
fn commit_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(COMMIT_KW) || p.at(END_KW));
    let m = p.start();
    p.bump_any();
    // PREPARED transaction_id
    if p.eat(PREPARED_KW) {
        string_literal(p);
    } else {
        // [ WORK | TRANSACTION ] [ AND [ NO ] CHAIN ]
        let _ = p.eat(WORK_KW) || p.eat(TRANSACTION_KW);
        if p.eat(AND_KW) {
            p.eat(NO_KW);
            p.expect(CHAIN_KW);
        }
    }
    m.complete(p, COMMIT_STMT)
}

const TRANSACTION_MODE_FIRST: TokenSet =
    TokenSet::new(&[ISOLATION_KW, READ_KW, NOT_KW, DEFERRABLE_KW]);

// where transaction_mode is one of:
//     ISOLATION LEVEL { SERIALIZABLE | REPEATABLE READ | READ COMMITTED | READ UNCOMMITTED }
//     READ WRITE | READ ONLY
//     [ NOT ] DEFERRABLE
fn opt_transaction_mode(p: &mut Parser<'_>) -> bool {
    if !p.at_ts(TRANSACTION_MODE_FIRST) {
        return false;
    }
    // ISOLATION LEVEL { SERIALIZABLE | REPEATABLE READ | READ COMMITTED | READ UNCOMMITTED }
    if p.eat(ISOLATION_KW) {
        p.expect(LEVEL_KW);
        if p.eat(SERIALIZABLE_KW) {
            true
        } else if p.eat(REPEATABLE_KW) {
            p.expect(READ_KW)
        } else if p.eat(READ_KW) {
            p.eat(UNCOMMITTED_KW) || p.expect(COMMITTED_KW)
        } else {
            false
        }
    // READ WRITE | READ ONLY
    } else if p.eat(READ_KW) {
        p.eat(WRITE_KW) || p.expect(ONLY_KW)
    // [ NOT ] DEFERRABLE
    } else {
        p.eat(NOT_KW);
        p.expect(DEFERRABLE_KW)
    }
}

// [ transaction_mode [, ...] ]
fn opt_transaction_mode_list(p: &mut Parser<'_>) {
    while !p.at(EOF) {
        if !opt_transaction_mode(p) {
            break;
        }
        if !p.eat(COMMA) && !p.at_ts(TRANSACTION_MODE_FIRST) {
            break;
        }
    }
}

// BEGIN [ WORK | TRANSACTION ] [ transaction_mode [, ...] ]
//
// START TRANSACTION [ transaction_mode [, ...] ]
//
// where transaction_mode is one of:
//     ISOLATION LEVEL { SERIALIZABLE | REPEATABLE READ | READ COMMITTED | READ UNCOMMITTED }
//     READ WRITE | READ ONLY
//     [ NOT ] DEFERRABLE
//
// https://www.postgresql.org/docs/17/sql-begin.html
// https://www.postgresql.org/docs/17/sql-start-transaction.html
fn begin_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(BEGIN_KW) || p.at(START_KW));
    let m = p.start();
    // BEGIN [ WORK | TRANSACTION ] [ transaction_mode [, ...] ]
    if p.eat(BEGIN_KW) {
        // [ WORK | TRANSACTION ]
        let _ = p.eat(WORK_KW) || p.eat(TRANSACTION_KW);
        opt_transaction_mode_list(p);
    } else {
        // START TRANSACTION [ transaction_mode [, ...] ]
        p.bump(START_KW);
        p.expect(TRANSACTION_KW);
        opt_transaction_mode_list(p);
    }
    m.complete(p, BEGIN_STMT)
}

// Sconst
fn opt_string_literal(p: &mut Parser<'_>) -> Option<CompletedMarker> {
    if p.at_ts(STRING_FIRST) {
        let m = p.start();
        p.bump_any();
        Some(m.complete(p, LITERAL))
    } else {
        None
    }
}

fn string_literal(p: &mut Parser<'_>) {
    if opt_string_literal(p).is_none() {
        p.error("expected string literal");
    }
}

fn opt_bool_literal(p: &mut Parser<'_>) -> bool {
    let m = p.start();
    if p.eat(TRUE_KW) || p.eat(FALSE_KW) {
        m.complete(p, LITERAL);
        true
    } else {
        m.abandon(p);
        false
    }
}

// PREPARE TRANSACTION transaction_id
//
// https://www.postgresql.org/docs/17/sql-prepare-transaction.html
fn prepare_transaction_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(PREPARE_KW));
    let m = p.start();
    p.bump(PREPARE_KW);
    p.expect(TRANSACTION_KW);
    string_literal(p);
    m.complete(p, PREPARE_TRANSACTION_STMT)
}

// SAVEPOINT savepoint_name
//
// https://www.postgresql.org/docs/17/sql-savepoint.html
fn savepoint_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(SAVEPOINT_KW));
    let m = p.start();
    p.bump(SAVEPOINT_KW);
    if name_ref_(p).is_none() {
        p.error("expected a name");
    }
    m.complete(p, SAVEPOINT_STMT)
}

// RELEASE [ SAVEPOINT ] savepoint_name
//
// https://www.postgresql.org/docs/17/sql-release-savepoint.html
fn release_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(RELEASE_KW));
    let m = p.start();
    p.bump(RELEASE_KW);
    p.eat(SAVEPOINT_KW);
    if name_ref_(p).is_none() {
        p.error("expected a name");
    }
    m.complete(p, RELEASE_SAVEPOINT_STMT)
}

// ROLLBACK [ WORK | TRANSACTION ] [ AND [ NO ] CHAIN ]
// ABORT [ WORK | TRANSACTION ] [ AND [ NO ] CHAIN ]
// ROLLBACK [ WORK | TRANSACTION ] TO [ SAVEPOINT ] savepoint_name
// ROLLBACK PREPARED transaction_id
//
// https://www.postgresql.org/docs/17/sql-rollback.html
// https://www.postgresql.org/docs/17/sql-abort.html
// https://www.postgresql.org/docs/17/sql-rollback-to.html
// https://www.postgresql.org/docs/17/sql-rollback-prepared.html
fn rollback_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(ROLLBACK_KW) || p.at(ABORT_KW));
    let m = p.start();
    let is_rollback = p.at(ROLLBACK_KW);
    p.bump_any();
    if p.eat(PREPARED_KW) {
        string_literal(p);
        return m.complete(p, ROLLBACK_STMT);
    }
    let _ = p.eat(WORK_KW) || p.eat(TRANSACTION_KW);
    if is_rollback && p.eat(TO_KW) {
        p.eat(SAVEPOINT_KW);
        if name_ref_(p).is_none() {
            p.error("expected a name");
        }
    } else if p.eat(AND_KW) {
        p.eat(NO_KW);
        p.expect(CHAIN_KW);
    }
    m.complete(p, ROLLBACK_STMT)
}

struct StmtRestrictions {
    begin_end_allowed: bool,
}

fn stmt(p: &mut Parser, r: &StmtRestrictions) -> Option<CompletedMarker> {
    match (p.current(), p.nth(1)) {
        (ABORT_KW, _) => Some(rollback_stmt(p)),
        (ALTER_KW, AGGREGATE_KW) => Some(alter_aggregate_stmt(p)),
        (ALTER_KW, COLLATION_KW) => Some(alter_collation_stmt(p)),
        (ALTER_KW, CONVERSION_KW) => Some(alter_conversion_stmt(p)),
        (ALTER_KW, DATABASE_KW) => Some(alter_database_stmt(p)),
        (ALTER_KW, DEFAULT_KW) if p.nth_at(2, PRIVILEGES_KW) => {
            Some(alter_default_privileges_stmt(p))
        }
        (ALTER_KW, DOMAIN_KW) => Some(alter_domain_stmt(p)),
        (ALTER_KW, EVENT_KW) if p.nth_at(2, TRIGGER_KW) => Some(alter_event_trigger_stmt(p)),
        (ALTER_KW, EXTENSION_KW) => Some(alter_extension_stmt(p)),
        (ALTER_KW, FOREIGN_KW) if p.nth_at(2, DATA_KW) => Some(alter_foreign_data_wrapper_stmt(p)),
        (ALTER_KW, FOREIGN_KW) if p.nth_at(2, TABLE_KW) => Some(alter_foreign_table_stmt(p)),
        (ALTER_KW, FUNCTION_KW) => Some(alter_function_stmt(p)),
        (ALTER_KW, GROUP_KW) => Some(alter_group_stmt(p)),
        (ALTER_KW, INDEX_KW) => Some(alter_index_stmt(p)),
        (ALTER_KW, LARGE_KW) if p.nth_at(2, OBJECT_KW) => Some(alter_large_object_stmt(p)),
        (ALTER_KW, MATERIALIZED_KW) if p.nth_at(2, VIEW_KW) => {
            Some(alter_materialized_view_stmt(p))
        }
        (ALTER_KW, OPERATOR_KW) if p.nth_at(2, CLASS_KW) => Some(alter_operator_class_stmt(p)),
        (ALTER_KW, OPERATOR_KW) if p.nth_at(2, FAMILY_KW) => Some(alter_operator_family_stmt(p)),
        (ALTER_KW, OPERATOR_KW) => Some(alter_operator_stmt(p)),
        (ALTER_KW, POLICY_KW) => Some(alter_policy_stmt(p)),
        (ALTER_KW, PROCEDURAL_KW | LANGUAGE_KW) => Some(alter_language_stmt(p)),
        (ALTER_KW, PROCEDURE_KW) => Some(alter_procedure_stmt(p)),
        (ALTER_KW, PUBLICATION_KW) => Some(alter_publication_stmt(p)),
        (ALTER_KW, ROLE_KW) => Some(alter_role_stmt(p)),
        (ALTER_KW, ROUTINE_KW) => Some(alter_routine_stmt(p)),
        (ALTER_KW, RULE_KW) => Some(alter_rule_stmt(p)),
        (ALTER_KW, SCHEMA_KW) => Some(alter_schema_stmt(p)),
        (ALTER_KW, SEQUENCE_KW) => Some(alter_sequence_stmt(p)),
        (ALTER_KW, SERVER_KW) => Some(alter_server_stmt(p)),
        (ALTER_KW, STATISTICS_KW) => Some(alter_statistics_stmt(p)),
        (ALTER_KW, SUBSCRIPTION_KW) => Some(alter_subscription_stmt(p)),
        (ALTER_KW, SYSTEM_KW) => Some(alter_system_stmt(p)),
        (ALTER_KW, TABLE_KW) => Some(alter_table_stmt(p)),
        (ALTER_KW, TABLESPACE_KW) => Some(alter_tablespace_stmt(p)),
        (ALTER_KW, TEXT_KW) if p.nth_at(2, SEARCH_KW) => match p.nth(3) {
            CONFIGURATION_KW => Some(alter_text_search_configuration(p)),
            DICTIONARY_KW => Some(alter_text_search_dict_stmt(p)),
            PARSER_KW => Some(alter_text_search_parser_stmt(p)),
            TEMPLATE_KW => Some(alter_text_search_template_stmt(p)),
            _ => {
                p.error("expected TEMPLATE, CONFIGURATION, DICTIONARY, PARSER, or TEMPLATE");
                None
            }
        },
        (ALTER_KW, TRIGGER_KW) => Some(alter_trigger_stmt(p)),
        (ALTER_KW, TYPE_KW) => Some(alter_type_stmt(p)),
        (ALTER_KW, USER_KW) if p.nth_at(2, MAPPING_KW) => Some(alter_user_mapping_stmt(p)),
        (ALTER_KW, USER_KW) => Some(alter_user_stmt(p)),
        (ALTER_KW, VIEW_KW) => Some(alter_view_stmt(p)),
        (ANALYZE_KW | ANALYSE_KW, _) => Some(analyze_stmt(p)),
        (BEGIN_KW, _) if r.begin_end_allowed => Some(begin_stmt(p)),
        (CALL_KW, _) => Some(call_stmt(p)),
        (CHECKPOINT_KW, _) => Some(checkpoint_stmt(p)),
        (CLOSE_KW, _) => Some(close_stmt(p)),
        (CLUSTER_KW, _) => Some(cluster_stmt(p)),
        (COMMENT_KW, _) => Some(comment_stmt(p)),
        (COMMIT_KW, _) => Some(commit_stmt(p)),
        (COPY_KW, _) => Some(copy_stmt(p)),
        (CREATE_KW, ACCESS_KW) => Some(create_access_method_stmt(p)),
        (CREATE_KW, AGGREGATE_KW) => Some(create_aggregate_stmt(p)),
        (CREATE_KW, CAST_KW) => Some(create_cast_stmt(p)),
        (CREATE_KW, COLLATION_KW) => Some(create_collation_stmt(p)),
        (CREATE_KW, CONVERSION_KW | DEFAULT_KW) => Some(create_conversion_stmt(p)),
        (CREATE_KW, DATABASE_KW) => Some(create_database_stmt(p)),
        (CREATE_KW, DOMAIN_KW) => Some(create_domain_stmt(p)),
        (CREATE_KW, EVENT_KW) => Some(create_event_trigger_stmt(p)),
        (CREATE_KW, EXTENSION_KW) => Some(create_extension_stmt(p)),
        (CREATE_KW, FOREIGN_KW) => match p.nth(2) {
            DATA_KW => Some(create_foreign_data_wrapper_stmt(p)),
            _ => Some(create_foreign_table_stmt(p)),
        },
        (CREATE_KW, FUNCTION_KW) => Some(create_function_stmt(p)),
        (CREATE_KW, GROUP_KW) => Some(create_group_stmt(p)),
        (CREATE_KW, INDEX_KW | UNIQUE_KW) => Some(create_index_stmt(p)),
        (CREATE_KW, LANGUAGE_KW) => Some(create_language_stmt(p)),
        (CREATE_KW, MATERIALIZED_KW) => Some(create_materialized_view_stmt(p)),
        (CREATE_KW, OPERATOR_KW) => match p.nth(2) {
            CLASS_KW => Some(create_operator_class_stmt(p)),
            FAMILY_KW => Some(create_operator_family_stmt(p)),
            _ => Some(create_operator_stmt(p)),
        },
        (CREATE_KW, OR_KW) => {
            // CREATE OR REPLACE [ TEMP | TEMPORARY ] [ RECURSIVE ] VIEW
            // CREATE OR REPLACE CONSTRAINT
            // CREATE OR REPLACE TRANSFORM
            // CREATE OR REPLACE RULE
            // CREATE OR REPLACE AGGREGATE
            // ^0     ^1 ^2      ^3
            match p.nth(3) {
                AGGREGATE_KW => Some(create_aggregate_stmt(p)),
                CONSTRAINT_KW | TRIGGER_KW => Some(create_trigger_stmt(p)),
                PROCEDURAL_KW | TRUSTED_KW | LANGUAGE_KW => Some(create_language_stmt(p)),
                PROCEDURE_KW => Some(create_procedure_stmt(p)),
                RECURSIVE_KW | TEMP_KW | TEMPORARY_KW => Some(create_view_stmt(p)),
                RULE_KW => Some(create_rule_stmt(p)),
                TRANSFORM_KW => Some(create_transform_stmt(p)),
                _ => Some(create_function_stmt(p)),
            }
        }
        (CREATE_KW, POLICY_KW) => Some(create_policy_stmt(p)),
        (CREATE_KW, PROCEDURE_KW) => Some(create_procedure_stmt(p)),
        (CREATE_KW, PUBLICATION_KW) => Some(create_publication_stmt(p)),
        (CREATE_KW, RECURSIVE_KW | VIEW_KW) => Some(create_view_stmt(p)),
        (CREATE_KW, ROLE_KW) => Some(create_role_stmt(p)),
        (CREATE_KW, RULE_KW) => Some(create_rule_stmt(p)),
        (CREATE_KW, SCHEMA_KW) => Some(create_schema_stmt(p)),
        (CREATE_KW, SEQUENCE_KW) => Some(create_sequence_stmt(p)),
        (CREATE_KW, SERVER_KW) => Some(create_server_stmt(p)),
        (CREATE_KW, STATISTICS_KW) => Some(create_statistics_stmt(p)),
        (CREATE_KW, SUBSCRIPTION_KW) => Some(create_subscription_stmt(p)),
        (CREATE_KW, TABLE_KW | GLOBAL_KW | LOCAL_KW | UNLOGGED_KW) if !p.nth_at(2, SEQUENCE_KW) => {
            Some(create_table_stmt(p))
        }
        (CREATE_KW, TABLESPACE_KW) => Some(create_tablespace_stmt(p)),
        (CREATE_KW, TEMP_KW | TEMPORARY_KW) => {
            // CREATE TEMP [ RECURSIVE ] VIEW
            // CREATE TEMP TABLE
            // ^0     ^1   ^2
            match p.nth(2) {
                RECURSIVE_KW | VIEW_KW => Some(create_view_stmt(p)),
                SEQUENCE_KW => Some(create_sequence_stmt(p)),
                _ => Some(create_table_stmt(p)),
            }
        }
        (CREATE_KW, TEXT_KW) if p.nth_at(2, SEARCH_KW) => match p.nth(3) {
            CONFIGURATION_KW => Some(create_text_search_config_stmt(p)),
            DICTIONARY_KW => Some(create_text_search_dict_stmt(p)),
            PARSER_KW => Some(create_text_search_parser_stmt(p)),
            TEMPLATE_KW => Some(create_text_search_template_stmt(p)),
            _ => {
                p.error("expected TEMPLATE, CONFIGURATION, DICTIONARY, PARSER, or TEMPLATE");
                None
            }
        },
        (CREATE_KW, TRANSFORM_KW) => Some(create_transform_stmt(p)),
        (CREATE_KW, TYPE_KW) => Some(create_type_stmt(p)),
        (CREATE_KW, UNLOGGED_KW) if p.nth_at(2, SEQUENCE_KW) => Some(create_sequence_stmt(p)),
        (CREATE_KW, USER_KW) if p.nth_at(2, MAPPING_KW) => Some(create_user_mapping_stmt(p)),
        (CREATE_KW, USER_KW) => Some(create_user_stmt(p)),
        (CREATE_KW, CONSTRAINT_KW | TRIGGER_KW) => Some(create_trigger_stmt(p)),
        (DEALLOCATE_KW, _) => Some(deallocate_stmt(p)),
        (DECLARE_KW, _) => Some(declare_stmt(p)),
        (DELETE_KW, _) => Some(delete_stmt(p, None)),
        (DISCARD_KW, _) => Some(discard_stmt(p)),
        (DO_KW, _) => Some(do_stmt(p)),
        (DROP_KW, ACCESS_KW) => Some(drop_access_method_stmt(p)),
        (DROP_KW, AGGREGATE_KW) => Some(drop_aggregate_stmt(p)),
        (DROP_KW, CAST_KW) => Some(drop_cast_stmt(p)),
        (DROP_KW, COLLATION_KW) => Some(drop_collation_stmt(p)),
        (DROP_KW, CONVERSION_KW) => Some(drop_conversion_stmt(p)),
        (DROP_KW, DATABASE_KW) => Some(drop_database_stmt(p)),
        (DROP_KW, DOMAIN_KW) => Some(drop_domain_stmt(p)),
        (DROP_KW, EVENT_KW) => Some(drop_event_trigger_stmt(p)),
        (DROP_KW, EXTENSION_KW) => Some(drop_extension_stmt(p)),
        (DROP_KW, FOREIGN_KW) => match p.nth(2) {
            DATA_KW => Some(drop_foreign_data_stmt(p)),
            TABLE_KW => Some(drop_foreign_table_stmt(p)),
            _ => {
                p.error("expected DATA or TABLE");
                None
            }
        },
        (DROP_KW, FUNCTION_KW) => Some(drop_function_stmt(p)),
        (DROP_KW, GROUP_KW) => Some(drop_group_stmt(p)),
        (DROP_KW, INDEX_KW) => Some(drop_index_stmt(p)),
        (DROP_KW, LANGUAGE_KW | PROCEDURAL_KW) => Some(drop_language_stmt(p)),
        (DROP_KW, MATERIALIZED_KW) => Some(drop_materialized_view_stmt(p)),
        (DROP_KW, OPERATOR_KW) => match p.nth(2) {
            CLASS_KW => Some(drop_operator_class_stmt(p)),
            FAMILY_KW => Some(drop_operator_family_stmt(p)),
            _ => Some(drop_operator_stmt(p)),
        },
        (DROP_KW, OWNED_KW) => Some(drop_owned_stmt(p)),
        (DROP_KW, POLICY_KW) => Some(drop_policy_stmt(p)),
        (DROP_KW, PROCEDURE_KW) => Some(drop_procedure_stmt(p)),
        (DROP_KW, PUBLICATION_KW) => Some(drop_publication_stmt(p)),
        (DROP_KW, ROLE_KW) => Some(drop_role_stmt(p)),
        (DROP_KW, ROUTINE_KW) => Some(drop_routine_stmt(p)),
        (DROP_KW, RULE_KW) => Some(drop_rule_stmt(p)),
        (DROP_KW, SCHEMA_KW) => Some(drop_schema_stmt(p)),
        (DROP_KW, SEQUENCE_KW) => Some(drop_sequence_stmt(p)),
        (DROP_KW, SERVER_KW) => Some(drop_server_stmt(p)),
        (DROP_KW, STATISTICS_KW) => Some(drop_statistics_stmt(p)),
        (DROP_KW, SUBSCRIPTION_KW) => Some(drop_subscription_stmt(p)),
        (DROP_KW, TABLE_KW) => Some(drop_table_stmt(p)),
        (DROP_KW, TABLESPACE_KW) => Some(drop_tablespace_stmt(p)),
        (DROP_KW, TEXT_KW) if p.nth_at(2, SEARCH_KW) => match p.nth(3) {
            CONFIGURATION_KW => Some(drop_text_search_config_stmt(p)),
            DICTIONARY_KW => Some(drop_text_search_dict_stmt(p)),
            PARSER_KW => Some(drop_text_search_parser_stmt(p)),
            TEMPLATE_KW => Some(drop_text_search_template_stmt(p)),
            _ => {
                p.error("expected TEMPLATE, CONFIGURATION, DICTIONARY, PARSER, or TEMPLATE");
                None
            }
        },
        (DROP_KW, TRANSFORM_KW) => Some(drop_transform_stmt(p)),
        (DROP_KW, TRIGGER_KW) => Some(drop_trigger_stmt(p)),
        (DROP_KW, TYPE_KW) => Some(drop_type_stmt(p)),
        (DROP_KW, USER_KW) => {
            if p.nth_at(2, MAPPING_KW) {
                Some(drop_user_mapping_stmt(p))
            } else {
                Some(drop_user_stmt(p))
            }
        }
        (DROP_KW, VIEW_KW) => Some(drop_view_stmt(p)),
        (END_KW, _) if r.begin_end_allowed => Some(commit_stmt(p)),
        (EXECUTE_KW, _) => Some(execute_stmt(p)),
        (EXPLAIN_KW, _) => Some(explain_stmt(p)),
        (FETCH_KW, _) => Some(fetch_stmt(p)),
        (GRANT_KW, _) => Some(grant_stmt(p)),
        (IMPORT_KW, FOREIGN_KW) => Some(import_foreign_schema_stmt(p)),
        (INSERT_KW, _) => Some(insert_stmt(p, None)),
        (L_PAREN, L_PAREN | SELECT_KW) => {
            // can have select nested in parens, i.e., ((select 1));
            let cm = paren_select(p)?;
            let cm = if p.at_ts(COMPOUND_SELECT_FIRST) {
                compound_select(p, cm)
            } else {
                cm
            };
            // TODO: this needs to be rethinked
            if p.at(ORDER_KW) {
                opt_order_by_clause(p);
            }
            Some(cm)
        }
        (LISTEN_KW, _) => Some(listen_stmt(p)),
        (LOAD_KW, _) => Some(load_stmt(p)),
        (LOCK_KW, _) => Some(lock_stmt(p)),
        (MERGE_KW, _) => Some(merge_stmt(p, None)),
        (MOVE_KW, _) => Some(move_stmt(p)),
        (NOTIFY_KW, _) => Some(notify_stmt(p)),
        (PREPARE_KW, TRANSACTION_KW) => Some(prepare_transaction_stmt(p)),
        (PREPARE_KW, _) => Some(prepare_stmt(p)),
        (REASSIGN_KW, _) => Some(reassign_stmt(p)),
        (REFRESH_KW, _) => Some(refresh_stmt(p)),
        (REINDEX_KW, _) => Some(reindex_stmt(p)),
        (RELEASE_KW, _) => Some(release_stmt(p)),
        (RESET_KW, ROLE_KW) => Some(set_role_stmt(p)),
        (RESET_KW, SESSION_KW) => Some(set_session_auth_stmt(p)),
        (RESET_KW, _) => Some(reset_stmt(p)),
        (REVOKE_KW, _) => Some(revoke_stmt(p)),
        (ROLLBACK_KW, _) => Some(rollback_stmt(p)),
        (SAVEPOINT_KW, _) => Some(savepoint_stmt(p)),
        (SECURITY_KW, LABEL_KW) => Some(security_label_stmt(p)),
        (SELECT_KW | TABLE_KW | VALUES_KW, _) => select_stmt(p, None),
        (SET_KW, CONSTRAINTS_KW) => Some(set_constraints_stmt(p)),
        (SET_KW, LOCAL_KW) => match p.nth(2) {
            ROLE_KW => Some(set_role_stmt(p)),
            SESSION_KW => Some(set_session_auth_stmt(p)),
            _ => Some(set_stmt(p)),
        },
        (SET_KW, ROLE_KW) => Some(set_role_stmt(p)),
        (SET_KW, SESSION_KW) => match p.nth(2) {
            AUTHORIZATION_KW | SESSION_KW => Some(set_session_auth_stmt(p)),
            CHARACTERISTICS_KW => Some(set_transaction_stmt(p)),
            ROLE_KW => Some(set_role_stmt(p)),
            _ => Some(set_stmt(p)),
        },
        (SET_KW, TRANSACTION_KW) => Some(set_transaction_stmt(p)),
        (SET_KW, TIME_KW | _) => Some(set_stmt(p)),
        (SHOW_KW, _) => Some(show_stmt(p)),
        (START_KW, TRANSACTION_KW) => Some(begin_stmt(p)),
        (TRUNCATE_KW, _) => Some(truncate_stmt(p)),
        (UNLISTEN_KW, _) => Some(unlisten_stmt(p)),
        (UPDATE_KW, _) => Some(update_stmt(p, None)),
        (VACUUM_KW, _) => Some(vacuum_stmt(p)),
        (WITH_KW, _) => with_stmt(p, None),
        (command, _) => {
            // commands are outlined in:
            // https://www.postgresql.org/docs/17/sql-commands.html
            // TODO: see error recovery in rust-analyzer's expr_bp
            p.err_and_bump(&format!("expected command, found {:?}", command));
            // m.abandon(p);
            None
        }
    }
}

// ALTER STATISTICS name OWNER TO { new_owner | CURRENT_ROLE | CURRENT_USER | SESSION_USER }
// ALTER STATISTICS name RENAME TO new_name
// ALTER STATISTICS name SET SCHEMA new_schema
// ALTER STATISTICS name SET STATISTICS { new_target | DEFAULT }
fn alter_statistics_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(ALTER_KW) && p.nth_at(1, STATISTICS_KW));
    let m = p.start();
    p.bump(ALTER_KW);
    p.bump(STATISTICS_KW);
    path_name_ref(p);
    match p.current() {
        OWNER_KW => {
            p.bump(OWNER_KW);
            p.expect(TO_KW);
            role(p);
        }
        RENAME_KW => {
            p.bump(RENAME_KW);
            p.expect(TO_KW);
            name_ref(p);
        }
        SET_KW => {
            p.bump(SET_KW);
            if p.eat(SCHEMA_KW) {
                name_ref(p);
            } else if p.eat(STATISTICS_KW) {
                if !p.eat(DEFAULT_KW) {
                    if opt_numeric_literal(p).is_none() {
                        p.error("expected numeric literal");
                    }
                }
            } else {
                p.error("expected SCHEMA or STATISTICS");
            }
        }
        _ => {
            p.error("expected OWNER, RENAME, or SET");
        }
    }
    m.complete(p, ALTER_STATISTICS_STMT)
}

// ALTER SERVER name [ VERSION 'new_version' ]
//     [ OPTIONS ( [ ADD | SET | DROP ] option ['value'] [, ... ] ) ]
// ALTER SERVER name OWNER TO { new_owner | CURRENT_ROLE | CURRENT_USER | SESSION_USER }
// ALTER SERVER name RENAME TO new_name
fn alter_server_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(ALTER_KW) && p.nth_at(1, SERVER_KW));
    let m = p.start();
    p.bump(ALTER_KW);
    p.bump(SERVER_KW);
    name_ref(p);
    match p.current() {
        OWNER_KW => {
            p.bump(OWNER_KW);
            p.expect(TO_KW);
            role(p);
        }
        RENAME_KW => {
            p.bump(RENAME_KW);
            p.expect(TO_KW);
            name(p);
        }
        _ => {
            let mut found_option = false;
            if p.eat(VERSION_KW) {
                string_literal(p);
                found_option = true;
            }
            if p.eat(OPTIONS_KW) {
                found_option = true;
                p.expect(L_PAREN);
                alter_option(p);
                while !p.at(EOF) && p.eat(COMMA) {
                    alter_option(p);
                }
                p.expect(R_PAREN);
            }
            if !found_option {
                p.error("expected ALTER SERVER option");
            }
        }
    }
    m.complete(p, ALTER_SERVER_STMT)
}

// ALTER SEQUENCE [ IF EXISTS ] name
//     [ AS data_type ]
//     [ INCREMENT [ BY ] increment ]
//     [ MINVALUE minvalue | NO MINVALUE ] [ MAXVALUE maxvalue | NO MAXVALUE ]
//     [ START [ WITH ] start ]
//     [ RESTART [ [ WITH ] restart ] ]
//     [ CACHE cache ] [ [ NO ] CYCLE ]
//     [ OWNED BY { table_name.column_name | NONE } ]
// ALTER SEQUENCE [ IF EXISTS ] name SET { LOGGED | UNLOGGED }
// ALTER SEQUENCE [ IF EXISTS ] name OWNER TO { new_owner | CURRENT_ROLE | CURRENT_USER | SESSION_USER }
// ALTER SEQUENCE [ IF EXISTS ] name RENAME TO new_name
// ALTER SEQUENCE [ IF EXISTS ] name SET SCHEMA new_schema
fn alter_sequence_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(ALTER_KW) && p.nth_at(1, SEQUENCE_KW));
    let m = p.start();
    p.bump(ALTER_KW);
    p.bump(SEQUENCE_KW);
    opt_if_exists(p);
    path_name_ref(p);
    match p.current() {
        SET_KW => {
            p.bump(SET_KW);
            if p.eat(SCHEMA_KW) {
                name_ref(p);
            } else {
                if !p.eat(LOGGED_KW) && !p.eat(UNLOGGED_KW) {
                    p.error("LOGGED or UNLOGGED");
                }
            }
        }
        OWNER_KW => {
            p.bump(OWNER_KW);
            p.expect(TO_KW);
            role(p);
        }
        RENAME_KW => {
            p.bump(RENAME_KW);
            p.expect(TO_KW);
            name(p);
        }
        _ => {
            let mut found_option = false;
            while !p.at(EOF) {
                if !opt_sequence_option(p) {
                    break;
                }
                found_option = true;
            }
            if !found_option {
                p.error("expected ALTER SEQUENCE option");
            }
        }
    }
    m.complete(p, ALTER_SEQUENCE_STMT)
}

// ALTER SCHEMA name RENAME TO new_name
// ALTER SCHEMA name OWNER TO { new_owner | CURRENT_ROLE | CURRENT_USER | SESSION_USER }
fn alter_schema_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(ALTER_KW) && p.nth_at(1, SCHEMA_KW));
    let m = p.start();
    p.bump(ALTER_KW);
    p.bump(SCHEMA_KW);
    name_ref(p);
    match p.current() {
        RENAME_KW => {
            p.bump(RENAME_KW);
            p.expect(TO_KW);
            name(p);
        }
        OWNER_KW => {
            p.bump(OWNER_KW);
            p.expect(TO_KW);
            role(p);
        }
        _ => {
            p.error("expected RENAME or OWNER");
        }
    }
    m.complete(p, ALTER_SCHEMA_STMT)
}

// ALTER RULE name ON table_name RENAME TO new_name
fn alter_rule_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(ALTER_KW) && p.nth_at(1, RULE_KW));
    let m = p.start();
    p.bump(ALTER_KW);
    p.bump(RULE_KW);
    name_ref(p);
    p.expect(ON_KW);
    path_name_ref(p);
    p.expect(RENAME_KW);
    p.expect(TO_KW);
    name(p);
    m.complete(p, ALTER_RULE_STMT)
}

// ALTER ROUTINE name [ ( [ [ argmode ] [ argname ] argtype [, ...] ] ) ]
//     action [ ... ] [ RESTRICT ]
// ALTER ROUTINE name [ ( [ [ argmode ] [ argname ] argtype [, ...] ] ) ]
//     RENAME TO new_name
// ALTER ROUTINE name [ ( [ [ argmode ] [ argname ] argtype [, ...] ] ) ]
//     OWNER TO { new_owner | CURRENT_ROLE | CURRENT_USER | SESSION_USER }
// ALTER ROUTINE name [ ( [ [ argmode ] [ argname ] argtype [, ...] ] ) ]
//     SET SCHEMA new_schema
// ALTER ROUTINE name [ ( [ [ argmode ] [ argname ] argtype [, ...] ] ) ]
//     [ NO ] DEPENDS ON EXTENSION extension_name
// where action is one of:
//     IMMUTABLE | STABLE | VOLATILE
//     [ NOT ] LEAKPROOF
//     [ EXTERNAL ] SECURITY INVOKER | [ EXTERNAL ] SECURITY DEFINER
//     PARALLEL { UNSAFE | RESTRICTED | SAFE }
//     COST execution_cost
//     ROWS result_rows
//     SET configuration_parameter { TO | = } { value | DEFAULT }
//     SET configuration_parameter FROM CURRENT
//     RESET configuration_parameter
//     RESET ALL
fn alter_routine_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(ALTER_KW) && p.nth_at(1, ROUTINE_KW));
    let m = p.start();
    p.bump(ALTER_KW);
    p.bump(ROUTINE_KW);
    name_ref(p);
    opt_param_list(p);
    match p.current() {
        RENAME_KW => {
            p.bump(RENAME_KW);
            p.expect(TO_KW);
            name(p);
        }
        OWNER_KW => {
            p.bump(OWNER_KW);
            p.expect(TO_KW);
            role(p);
        }
        SET_KW if p.nth_at(1, SCHEMA_KW) => {
            p.bump(SET_KW);
            p.bump(SCHEMA_KW);
            name_ref(p);
        }
        NO_KW | DEPENDS_KW => {
            p.eat(NO_KW);
            p.expect(DEPENDS_KW);
            p.expect(ON_KW);
            p.expect(EXTENSION_KW);
            path_name_ref(p);
        }
        _ => {
            func_option_list(p);
        }
    }
    p.eat(RESTRICT_KW);
    m.complete(p, ALTER_ROUTINE_STMT)
}

// ALTER ROLE role_specification [ WITH ] option [ ... ]
// where option can be:
//       SUPERUSER | NOSUPERUSER
//     | CREATEDB | NOCREATEDB
//     | CREATEROLE | NOCREATEROLE
//     | INHERIT | NOINHERIT
//     | LOGIN | NOLOGIN
//     | REPLICATION | NOREPLICATION
//     | BYPASSRLS | NOBYPASSRLS
//     | CONNECTION LIMIT connlimit
//     | [ ENCRYPTED ] PASSWORD 'password' | PASSWORD NULL
//     | VALID UNTIL 'timestamp'
// ALTER ROLE name RENAME TO new_name
// ALTER ROLE { role_specification | ALL } [ IN DATABASE database_name ] SET configuration_parameter { TO | = } { value | DEFAULT }
// ALTER ROLE { role_specification | ALL } [ IN DATABASE database_name ] SET configuration_parameter FROM CURRENT
// ALTER ROLE { role_specification | ALL } [ IN DATABASE database_name ] RESET configuration_parameter
// ALTER ROLE { role_specification | ALL } [ IN DATABASE database_name ] RESET ALL
// where role_specification can be:
//     role_name
//   | CURRENT_ROLE
//   | CURRENT_USER
//   | SESSION_USER
fn alter_role_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(ALTER_KW) && p.nth_at(1, ROLE_KW));
    let m = p.start();
    p.bump(ALTER_KW);
    p.bump(ROLE_KW);
    if !p.eat(ALL_KW) {
        role(p);
    }
    match p.current() {
        RENAME_KW => {
            p.bump(RENAME_KW);
            p.expect(TO_KW);
            name(p);
        }
        IN_KW | SET_KW | RESET_KW => {
            if p.eat(IN_KW) {
                p.expect(DATABASE_KW);
                name_ref(p);
            }
            if p.at(SET_KW) {
                set_configuration_param(p);
            } else if p.eat(RESET_KW) {
                if !p.eat(ALL_KW) {
                    path_name_ref(p);
                }
            }
        }
        _ => {
            p.eat(WITH_KW);
            if !opt_role_option(p) {
                p.error("expected option");
            }
            while !p.at(EOF) && p.at_ts(ROLE_OPTION_FIRST) {
                opt_role_option(p);
            }
        }
    }
    m.complete(p, ALTER_ROLE_STMT)
}

// ALTER PUBLICATION name ADD publication_object [, ...]
// ALTER PUBLICATION name SET publication_object [, ...]
// ALTER PUBLICATION name DROP publication_object [, ...]
// ALTER PUBLICATION name SET ( publication_parameter [= value] [, ... ] )
// ALTER PUBLICATION name OWNER TO { new_owner | CURRENT_ROLE | CURRENT_USER | SESSION_USER }
// ALTER PUBLICATION name RENAME TO new_name
// where publication_object is one of:
//     TABLE [ ONLY ] table_name [ * ] [ ( column_name [, ... ] ) ] [ WHERE ( expression ) ] [, ... ]
//     TABLES IN SCHEMA { schema_name | CURRENT_SCHEMA } [, ... ]
fn alter_publication_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(ALTER_KW) && p.nth_at(1, PUBLICATION_KW));
    let m = p.start();
    p.bump(ALTER_KW);
    p.bump(PUBLICATION_KW);
    name_ref(p);
    match p.current() {
        ADD_KW | DROP_KW => {
            p.bump_any();
            publication_object(p);
            while !p.at(EOF) && p.eat(COMMA) {
                publication_object(p);
            }
        }
        SET_KW => {
            p.bump(SET_KW);
            if p.eat(L_PAREN) {
                while !p.at(EOF) && !p.at(R_PAREN) {
                    if !storage_parameter(p) {
                        break;
                    }
                    if !p.eat(COMMA) {
                        break;
                    }
                }
                p.expect(R_PAREN);
            } else {
                publication_object(p);
                while !p.at(EOF) && p.eat(COMMA) {
                    publication_object(p);
                }
            }
        }
        OWNER_KW => {
            p.bump(OWNER_KW);
            p.expect(TO_KW);
            role(p);
        }
        RENAME_KW => {
            p.bump(RENAME_KW);
            p.expect(TO_KW);
            name(p);
        }
        _ => {
            p.error("expected ADD, SET, DROP, OWNER, or RENAME");
        }
    }
    m.complete(p, ALTER_PUBLICATION_STMT)
}

// ALTER PROCEDURE name [ ( [ [ argmode ] [ argname ] argtype [, ...] ] ) ]
//     action [ ... ] [ RESTRICT ]
// ALTER PROCEDURE name [ ( [ [ argmode ] [ argname ] argtype [, ...] ] ) ]
//     RENAME TO new_name
// ALTER PROCEDURE name [ ( [ [ argmode ] [ argname ] argtype [, ...] ] ) ]
//     OWNER TO { new_owner | CURRENT_ROLE | CURRENT_USER | SESSION_USER }
// ALTER PROCEDURE name [ ( [ [ argmode ] [ argname ] argtype [, ...] ] ) ]
//     SET SCHEMA new_schema
// ALTER PROCEDURE name [ ( [ [ argmode ] [ argname ] argtype [, ...] ] ) ]
//     [ NO ] DEPENDS ON EXTENSION extension_name
//
// where action is one of:
//     [ EXTERNAL ] SECURITY INVOKER | [ EXTERNAL ] SECURITY DEFINER
//     SET configuration_parameter { TO | = } { value | DEFAULT }
//     SET configuration_parameter FROM CURRENT
//     RESET configuration_parameter
//     RESET ALL
fn alter_procedure_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(ALTER_KW) && p.nth_at(1, PROCEDURE_KW));
    let m = p.start();
    p.bump(ALTER_KW);
    p.bump(PROCEDURE_KW);
    path_name_ref(p);
    opt_param_list(p);
    match p.current() {
        RENAME_KW => {
            p.bump(RENAME_KW);
            p.expect(TO_KW);
            name(p);
        }
        OWNER_KW => {
            p.bump(OWNER_KW);
            p.expect(TO_KW);
            role(p);
        }
        SET_KW if p.nth_at(1, SCHEMA_KW) => {
            p.bump(SET_KW);
            p.bump(SCHEMA_KW);
            name_ref(p);
        }
        DEPENDS_KW | NO_KW => {
            p.eat(NO_KW);
            p.expect(DEPENDS_KW);
            p.expect(ON_KW);
            p.expect(EXTENSION_KW);
            path_name_ref(p);
        }
        _ => {
            func_option_list(p);
            p.eat(RESTRICT_KW);
        }
    }
    m.complete(p, ALTER_PROCEDURE_STMT)
}

// ALTER POLICY name ON table_name RENAME TO new_name
// ALTER POLICY name ON table_name
//     [ TO { role_name | PUBLIC | CURRENT_ROLE | CURRENT_USER | SESSION_USER } [, ...] ]
//     [ USING ( using_expression ) ]
//     [ WITH CHECK ( check_expression ) ]
fn alter_policy_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(ALTER_KW) && p.nth_at(1, POLICY_KW));
    let m = p.start();
    p.bump(ALTER_KW);
    p.bump(POLICY_KW);
    name_ref(p);
    p.expect(ON_KW);
    name_ref(p);
    if p.eat(RENAME_KW) {
        p.expect(TO_KW);
        name(p);
    } else {
        if p.eat(TO_KW) {
            role(p);
            while !p.at(EOF) && p.eat(COMMA) {
                role(p);
            }
        }
        if p.eat(USING_KW) {
            p.expect(L_PAREN);
            if expr(p).is_none() {
                p.error("expected expression");
            }
            p.expect(R_PAREN);
        }
        if p.eat(WITH_KW) {
            p.expect(CHECK_KW);
            p.expect(L_PAREN);
            if expr(p).is_none() {
                p.error("expected expression");
            }
            p.expect(R_PAREN);
        }
    }
    m.complete(p, ALTER_POLICY_STMT)
}

// ALTER OPERATOR FAMILY name USING index_method ADD
//   {  OPERATOR strategy_number operator_name ( op_type, op_type )
//               [ FOR SEARCH | FOR ORDER BY sort_family_name ]
//    | FUNCTION support_number [ ( op_type [ , op_type ] ) ]
//               function_name [ ( argument_type [, ...] ) ]
//   } [, ... ]
//
// ALTER OPERATOR FAMILY name USING index_method DROP
//   {  OPERATOR strategy_number ( op_type [ , op_type ] )
//    | FUNCTION support_number ( op_type [ , op_type ] )
//   } [, ... ]
//
// ALTER OPERATOR FAMILY name USING index_method
//     RENAME TO new_name
//
// ALTER OPERATOR FAMILY name USING index_method
//     OWNER TO { new_owner | CURRENT_ROLE | CURRENT_USER | SESSION_USER }
//
// ALTER OPERATOR FAMILY name USING index_method
//     SET SCHEMA new_schema
fn alter_operator_family_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(ALTER_KW) && p.nth_at(1, OPERATOR_KW) && p.nth_at(2, FAMILY_KW));
    let m = p.start();
    p.bump(ALTER_KW);
    p.bump(OPERATOR_KW);
    p.bump(FAMILY_KW);
    path_name_ref(p);
    p.expect(USING_KW);
    name_ref(p);
    match p.current() {
        ADD_KW => {
            p.bump_any();
            // TODO: need to add some validators to make this stricter
            operator_class_option(p);
            while !p.at(EOF) && p.eat(COMMA) {
                operator_class_option(p);
            }
        }
        DROP_KW => {
            p.bump_any();
            operator_drop_class_option(p);
            while !p.at(EOF) && p.eat(COMMA) {
                operator_drop_class_option(p);
            }
        }
        RENAME_KW => {
            p.bump(RENAME_KW);
            p.expect(TO_KW);
            name(p);
        }
        OWNER_KW => {
            p.bump(OWNER_KW);
            p.expect(TO_KW);
            role(p);
        }
        SET_KW => {
            p.bump(SET_KW);
            p.expect(SCHEMA_KW);
            name_ref(p);
        }
        _ => {
            p.error("expected ADD, DROP, RENAME, OWNER, or SET");
        }
    }
    m.complete(p, ALTER_OPERATOR_FAMILY_STMT)
}

// ALTER OPERATOR CLASS name USING index_method
//     RENAME TO new_name
// ALTER OPERATOR CLASS name USING index_method
//     OWNER TO { new_owner | CURRENT_ROLE | CURRENT_USER | SESSION_USER }
// ALTER OPERATOR CLASS name USING index_method
//     SET SCHEMA new_schema
fn alter_operator_class_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(ALTER_KW) && p.nth_at(1, OPERATOR_KW) && p.nth_at(2, CLASS_KW));
    let m = p.start();
    p.bump(ALTER_KW);
    p.bump(OPERATOR_KW);
    p.bump(CLASS_KW);
    path_name_ref(p);
    p.expect(USING_KW);
    name_ref(p);
    match p.current() {
        RENAME_KW => {
            p.bump(RENAME_KW);
            p.expect(TO_KW);
            name_ref(p);
        }
        OWNER_KW => {
            p.bump(OWNER_KW);
            p.expect(TO_KW);
            role(p);
        }
        SET_KW => {
            p.bump(SET_KW);
            p.expect(SCHEMA_KW);
            name_ref(p);
        }
        _ => {
            p.error("expected RENAME, OWNER, or SET");
        }
    }
    m.complete(p, ALTER_OPERATOR_CLASS_STMT)
}

// ALTER OPERATOR name ( { left_type | NONE } , right_type )
//     OWNER TO { new_owner | CURRENT_ROLE | CURRENT_USER | SESSION_USER }
// ALTER OPERATOR name ( { left_type | NONE } , right_type )
//     SET SCHEMA new_schema
// ALTER OPERATOR name ( { left_type | NONE } , right_type )
//     SET ( {  RESTRICT = { res_proc | NONE }
//            | JOIN = { join_proc | NONE }
//            | COMMUTATOR = com_op
//            | NEGATOR = neg_op
//            | HASHES
//            | MERGES
//           } [, ... ] )
fn alter_operator_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(ALTER_KW) && p.nth_at(1, OPERATOR_KW));
    let m = p.start();
    p.bump(ALTER_KW);
    p.bump(OPERATOR_KW);
    operator_sig(p);
    match p.current() {
        OWNER_KW => {
            p.bump(OWNER_KW);
            p.expect(TO_KW);
            role(p);
        }
        SET_KW => {
            p.bump(SET_KW);
            if p.eat(SCHEMA_KW) {
                name_ref(p);
            } else {
                p.expect(L_PAREN);
                if !attribute_option(p, AttributeValue::Either) {
                    p.error("expected option");
                }
                while !p.at(EOF) && p.eat(COMMA) {
                    if !attribute_option(p, AttributeValue::Either) {
                        p.error("expected option");
                    }
                }
                p.expect(R_PAREN);
            }
        }
        _ => {
            p.error("expected OWNER or SET");
        }
    }
    m.complete(p, ALTER_OPERATOR_STMT)
}

// ALTER MATERIALIZED VIEW [ IF EXISTS ] name
//     action [, ... ]
// ALTER MATERIALIZED VIEW name
//     [ NO ] DEPENDS ON EXTENSION extension_name
// ALTER MATERIALIZED VIEW [ IF EXISTS ] name
//     RENAME [ COLUMN ] column_name TO new_column_name
// ALTER MATERIALIZED VIEW [ IF EXISTS ] name
//     RENAME TO new_name
// ALTER MATERIALIZED VIEW [ IF EXISTS ] name
//     SET SCHEMA new_schema
// ALTER MATERIALIZED VIEW ALL IN TABLESPACE name [ OWNED BY role_name [, ... ] ]
//     SET TABLESPACE new_tablespace [ NOWAIT ]
//
// where action is one of:
//     ALTER [ COLUMN ] column_name SET STATISTICS integer
//     ALTER [ COLUMN ] column_name SET ( attribute_option = value [, ... ] )
//     ALTER [ COLUMN ] column_name RESET ( attribute_option [, ... ] )
//     ALTER [ COLUMN ] column_name SET STORAGE { PLAIN | EXTERNAL | EXTENDED | MAIN | DEFAULT }
//     ALTER [ COLUMN ] column_name SET COMPRESSION compression_method
//     CLUSTER ON index_name
//     SET WITHOUT CLUSTER
//     SET ACCESS METHOD new_access_method
//     SET TABLESPACE new_tablespace
//     SET ( storage_parameter [= value] [, ... ] )
//     RESET ( storage_parameter [, ... ] )
//     OWNER TO { new_owner | CURRENT_ROLE | CURRENT_USER | SESSION_USER }
fn alter_materialized_view_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(ALTER_KW) && p.nth_at(1, MATERIALIZED_KW) && p.nth_at(2, VIEW_KW));
    let m = p.start();
    p.bump(ALTER_KW);
    p.bump(MATERIALIZED_KW);
    p.bump(VIEW_KW);
    if p.eat(ALL_KW) {
        p.expect(IN_KW);
        p.expect(TABLESPACE_KW);
        name_ref(p);
        if p.eat(OWNED_KW) {
            p.expect(BY_KW);
            role(p);
            while !p.at(EOF) && p.eat(COMMA) {
                role(p);
            }
        }
        p.expect(SET_KW);
        p.expect(TABLESPACE_KW);
        name(p);
        p.eat(NOWAIT_KW);
    } else {
        opt_if_exists(p);
        path_name_ref(p);
        match p.current() {
            RENAME_KW => {
                p.bump(RENAME_KW);
                if p.eat(TO_KW) {
                    name(p);
                } else {
                    p.eat(COLUMN_KW);
                    name_ref(p);
                    p.expect(TO_KW);
                    name(p);
                }
            }
            SET_KW if p.nth_at(1, SCHEMA_KW) => {
                p.bump(SET_KW);
                p.bump(SCHEMA_KW);
                name_ref(p);
            }
            DEPENDS_KW | NO_KW => {
                p.eat(NO_KW);
                p.expect(DEPENDS_KW);
                p.expect(ON_KW);
                p.expect(EXTENSION_KW);
                name_ref(p);
            }
            ALTER_KW | CLUSTER_KW | SET_KW | RESET_KW | OWNER_KW => {
                // TODO: we should be robust to missing commas
                while !p.at(EOF) {
                    let action = p.start();
                    match alter_table_action(p) {
                        Some(action_kind) => {
                            action.complete(p, action_kind);
                        }
                        None => {
                            action.abandon(p);
                        }
                    };
                    if !p.eat(COMMA) {
                        break;
                    }
                }
            }
            _ => {
                p.error("Expected RENAME, SET SCHEMA, [NO] DEPENDS, or action (ALTER, CLUSTER, SET, RESET, OWNER)");
            }
        }
    }
    m.complete(p, ALTER_MATERIALIZED_VIEW_STMT)
}

// ALTER LARGE OBJECT large_object_oid OWNER TO { new_owner | CURRENT_ROLE | CURRENT_USER | SESSION_USER }
fn alter_large_object_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(ALTER_KW) && p.nth_at(1, LARGE_KW) && p.nth_at(2, OBJECT_KW));
    let m = p.start();
    p.bump(ALTER_KW);
    p.bump(LARGE_KW);
    p.bump(OBJECT_KW);
    if opt_numeric_literal(p).is_none() {
        p.error("expected numeric literal");
    }
    p.expect(OWNER_KW);
    p.expect(TO_KW);
    role(p);
    m.complete(p, ALTER_LARGE_OBJECT_STMT)
}

// ALTER [ PROCEDURAL ] LANGUAGE name RENAME TO new_name
// ALTER [ PROCEDURAL ] LANGUAGE name OWNER TO { new_owner | CURRENT_ROLE | CURRENT_USER | SESSION_USER }
fn alter_language_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(ALTER_KW) && (p.nth_at(1, PROCEDURAL_KW) || p.nth_at(1, LANGUAGE_KW)));
    let m = p.start();
    p.bump(ALTER_KW);
    p.eat(PROCEDURAL_KW);
    p.expect(LANGUAGE_KW);
    name_ref(p);
    match p.current() {
        RENAME_KW => {
            p.bump(RENAME_KW);
            p.expect(TO_KW);
            name(p);
        }
        OWNER_KW => {
            p.bump(OWNER_KW);
            p.expect(TO_KW);
            role(p);
        }
        _ => {
            p.error("expected RENAME or OWNER");
        }
    }
    m.complete(p, ALTER_LANGUAGE_STMT)
}

// ALTER INDEX ALL IN TABLESPACE name [ OWNED BY role_name [, ... ] ]
//     SET TABLESPACE new_tablespace [ NOWAIT ]
// ALTER INDEX name ATTACH PARTITION index_name
// ALTER INDEX name [ NO ] DEPENDS ON EXTENSION extension_name
// ALTER INDEX [ IF EXISTS ] name RENAME TO new_name
// ALTER INDEX [ IF EXISTS ] name SET TABLESPACE tablespace_name
// ALTER INDEX [ IF EXISTS ] name SET ( storage_parameter [= value] [, ... ] )
// ALTER INDEX [ IF EXISTS ] name RESET ( storage_parameter [, ... ] )
// ALTER INDEX [ IF EXISTS ] name ALTER [ COLUMN ] column_number
//     SET STATISTICS integer
fn alter_index_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(ALTER_KW) && p.nth_at(1, INDEX_KW));
    let m = p.start();
    p.bump(ALTER_KW);
    p.bump(INDEX_KW);
    if p.eat(ALL_KW) {
        p.expect(IN_KW);
        p.expect(TABLESPACE_KW);
        path_name_ref(p);
        if p.eat(OWNED_KW) {
            p.expect(BY_KW);
            role(p);
            while !p.at(EOF) && p.eat(COMMA) {
                role(p);
            }
        }
        p.expect(SET_KW);
        p.expect(TABLESPACE_KW);
        name_ref(p);
        p.eat(NOWAIT_KW);
    } else {
        opt_if_exists(p);
        path_name_ref(p);
        match p.current() {
            RENAME_KW => {
                p.bump(RENAME_KW);
                p.expect(TO_KW);
                path_name_ref(p);
            }
            SET_KW => {
                p.bump(SET_KW);
                if p.at(TABLESPACE_KW) {
                    p.bump(TABLESPACE_KW);
                    path_name_ref(p);
                } else {
                    p.expect(L_PAREN);
                    storage_parameter(p);
                    while !p.at(EOF) && p.eat(COMMA) {
                        storage_parameter(p);
                    }
                    p.expect(R_PAREN);
                }
            }
            ATTACH_KW => {
                p.bump(ATTACH_KW);
                p.expect(PARTITION_KW);
                path_name_ref(p);
            }
            DEPENDS_KW | NO_KW => {
                p.eat(NO_KW);
                p.bump(DEPENDS_KW);
                p.expect(ON_KW);
                p.expect(EXTENSION_KW);
                path_name_ref(p);
            }
            RESET_KW => {
                p.bump(RESET_KW);
                p.expect(L_PAREN);
                storage_parameter(p);
                while !p.at(EOF) && p.eat(COMMA) {
                    storage_parameter(p);
                }
                p.expect(R_PAREN);
            }
            ALTER_KW => {
                p.bump(ALTER_KW);
                p.eat(COLUMN_KW);
                if opt_numeric_literal(p).is_none() {
                    p.error("expected numeric literal");
                }
                p.expect(SET_KW);
                p.expect(STATISTICS_KW);
                if opt_numeric_literal(p).is_none() {
                    p.error("expected numeric literal");
                }
            }
            _ => {
                p.error("expected RENAME, SET, ATTACH, DEPENDS, RESET, or ALTER");
            }
        }
    }
    m.complete(p, ALTER_INDEX_STMT)
}

// ALTER GROUP role_specification ADD USER user_name [, ... ]
// ALTER GROUP role_specification DROP USER user_name [, ... ]
// where role_specification can be:
//     role_name
//   | CURRENT_ROLE
//   | CURRENT_USER
//   | SESSION_USER
// ALTER GROUP group_name RENAME TO new_name
fn alter_group_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(ALTER_KW) && p.nth_at(1, GROUP_KW));
    let m = p.start();
    p.bump(ALTER_KW);
    p.bump(GROUP_KW);
    role(p);
    match p.current() {
        ADD_KW | DROP_KW => {
            p.bump_any();
            p.expect(USER_KW);
            name_ref(p);
            while !p.at(EOF) && p.eat(COMMA) {
                name_ref(p);
            }
        }
        RENAME_KW => {
            p.bump(RENAME_KW);
            p.expect(TO_KW);
            name(p);
        }
        _ => {
            p.error("expected ADD, DROP, or RENAME");
        }
    }
    m.complete(p, ALTER_GROUP_STMT)
}

// ALTER FUNCTION name [ ( [ [ argmode ] [ argname ] argtype [, ...] ] ) ]
//     action [ ... ] [ RESTRICT ]
// ALTER FUNCTION name [ ( [ [ argmode ] [ argname ] argtype [, ...] ] ) ]
//     RENAME TO new_name
// ALTER FUNCTION name [ ( [ [ argmode ] [ argname ] argtype [, ...] ] ) ]
//     OWNER TO { new_owner | CURRENT_ROLE | CURRENT_USER | SESSION_USER }
// ALTER FUNCTION name [ ( [ [ argmode ] [ argname ] argtype [, ...] ] ) ]
//     SET SCHEMA new_schema
// ALTER FUNCTION name [ ( [ [ argmode ] [ argname ] argtype [, ...] ] ) ]
//     [ NO ] DEPENDS ON EXTENSION extension_name
// where action is one of:
//     CALLED ON NULL INPUT | RETURNS NULL ON NULL INPUT | STRICT
//     IMMUTABLE | STABLE | VOLATILE
//     [ NOT ] LEAKPROOF
//     [ EXTERNAL ] SECURITY INVOKER | [ EXTERNAL ] SECURITY DEFINER
//     PARALLEL { UNSAFE | RESTRICTED | SAFE }
//     COST execution_cost
//     ROWS result_rows
//     SUPPORT support_function
//     SET configuration_parameter { TO | = } { value | DEFAULT }
//     SET configuration_parameter FROM CURRENT
//     RESET configuration_parameter
//     RESET ALL
fn alter_function_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(ALTER_KW) && p.nth_at(1, FUNCTION_KW));
    let m = p.start();
    p.bump(ALTER_KW);
    p.bump(FUNCTION_KW);
    path_name_ref(p);
    opt_param_list(p);
    match p.current() {
        RENAME_KW => {
            p.bump(RENAME_KW);
            p.expect(TO_KW);
            name(p);
        }
        OWNER_KW => {
            p.bump(OWNER_KW);
            p.expect(TO_KW);
            role(p);
        }
        SET_KW if p.nth_at(1, SCHEMA_KW) => {
            p.bump(SET_KW);
            p.bump(SCHEMA_KW);
            name_ref(p);
        }
        DEPENDS_KW | NO_KW => {
            p.eat(NO_KW);
            p.expect(DEPENDS_KW);
            p.expect(ON_KW);
            p.expect(EXTENSION_KW);
            name_ref(p);
        }
        _ => {
            func_option_list(p);
        }
    }
    p.eat(RESTRICT_KW);
    m.complete(p, ALTER_FUNCTION_STMT)
}

// ALTER FOREIGN TABLE [ IF EXISTS ] [ ONLY ] name [ * ]
//     action [, ... ]
// ALTER FOREIGN TABLE [ IF EXISTS ] [ ONLY ] name [ * ]
//     RENAME [ COLUMN ] column_name TO new_column_name
// ALTER FOREIGN TABLE [ IF EXISTS ] name
//     RENAME TO new_name
// ALTER FOREIGN TABLE [ IF EXISTS ] name
//     SET SCHEMA new_schema
//
// where action is one of:
//     ADD [ COLUMN ] column_name data_type [ COLLATE collation ] [ column_constraint [ ... ] ]
//     DROP [ COLUMN ] [ IF EXISTS ] column_name [ RESTRICT | CASCADE ]
//     ALTER [ COLUMN ] column_name [ SET DATA ] TYPE data_type [ COLLATE collation ]
//     ALTER [ COLUMN ] column_name SET DEFAULT expression
//     ALTER [ COLUMN ] column_name DROP DEFAULT
//     ALTER [ COLUMN ] column_name { SET | DROP } NOT NULL
//     ALTER [ COLUMN ] column_name SET STATISTICS integer
//     ALTER [ COLUMN ] column_name SET ( attribute_option = value [, ... ] )
//     ALTER [ COLUMN ] column_name RESET ( attribute_option [, ... ] )
//     ALTER [ COLUMN ] column_name SET STORAGE { PLAIN | EXTERNAL | EXTENDED | MAIN | DEFAULT }
//     ALTER [ COLUMN ] column_name OPTIONS ( [ ADD | SET | DROP ] option ['value'] [, ... ])
//     ADD table_constraint [ NOT VALID ]
//     VALIDATE CONSTRAINT constraint_name
//     DROP CONSTRAINT [ IF EXISTS ]  constraint_name [ RESTRICT | CASCADE ]
//     DISABLE TRIGGER [ trigger_name | ALL | USER ]
//     ENABLE TRIGGER [ trigger_name | ALL | USER ]
//     ENABLE REPLICA TRIGGER trigger_name
//     ENABLE ALWAYS TRIGGER trigger_name
//     SET WITHOUT OIDS
//     INHERIT parent_table
//     NO INHERIT parent_table
//     OWNER TO { new_owner | CURRENT_ROLE | CURRENT_USER | SESSION_USER }
//     OPTIONS ( [ ADD | SET | DROP ] option ['value'] [, ... ])
fn alter_foreign_table_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(ALTER_KW) && p.nth_at(1, FOREIGN_KW) && p.nth_at(2, TABLE_KW));
    let m = p.start();
    p.bump(ALTER_KW);
    p.bump(FOREIGN_KW);
    p.bump(TABLE_KW);
    opt_if_exists(p);
    relation_name(p);
    match p.current() {
        RENAME_KW => {
            p.bump(RENAME_KW);
            if p.eat(TO_KW) {
                name(p);
            } else {
                p.eat(COLUMN_KW);
                name_ref(p);
                p.expect(TO_KW);
                name(p);
            }
        }
        SET_KW if p.nth_at(1, SCHEMA_KW) => {
            p.bump(SET_KW);
            p.bump(SCHEMA_KW);
            name_ref(p);
        }
        _ => {
            // TODO: we should be robust to missing commas
            while !p.at(EOF) {
                let action = p.start();
                match alter_table_action(p) {
                    Some(action_kind) => {
                        action.complete(p, action_kind);
                    }
                    None => {
                        action.abandon(p);
                    }
                };
                if !p.eat(COMMA) {
                    break;
                }
            }
        }
    }
    m.complete(p, ALTER_FOREIGN_TABLE_STMT)
}

// ALTER FOREIGN DATA WRAPPER name
//     [ HANDLER handler_function | NO HANDLER ]
//     [ VALIDATOR validator_function | NO VALIDATOR ]
//     [ OPTIONS ( [ ADD | SET | DROP ] option ['value'] [, ... ]) ]
// ALTER FOREIGN DATA WRAPPER name OWNER TO { new_owner | CURRENT_ROLE | CURRENT_USER | SESSION_USER }
// ALTER FOREIGN DATA WRAPPER name RENAME TO new_name
fn alter_foreign_data_wrapper_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(
        p.at(ALTER_KW)
            && p.nth_at(1, FOREIGN_KW)
            && p.nth_at(2, DATA_KW)
            && p.nth_at(3, WRAPPER_KW)
    );
    let m = p.start();
    p.bump(ALTER_KW);
    p.bump(FOREIGN_KW);
    p.bump(DATA_KW);
    p.bump(WRAPPER_KW);
    name_ref(p);
    let mut found_option = false;
    match p.current() {
        OWNER_KW => {
            p.bump(OWNER_KW);
            p.expect(TO_KW);
            role(p);
            found_option = true;
        }
        RENAME_KW => {
            p.bump(RENAME_KW);
            p.expect(TO_KW);
            name(p);
            found_option = true;
        }
        _ => {
            while !p.at(EOF) {
                if !opt_fdw_option(p) {
                    break;
                }
                found_option = true;
            }
        }
    }
    if !found_option {
        p.error("Missing alter foreign data wrapper option or action.")
    }
    m.complete(p, ALTER_FOREIGN_DATA_WRAPPER_STMT)
}

// ALTER EVENT TRIGGER name DISABLE
// ALTER EVENT TRIGGER name ENABLE [ REPLICA | ALWAYS ]
// ALTER EVENT TRIGGER name OWNER TO { new_owner | CURRENT_ROLE | CURRENT_USER | SESSION_USER }
// ALTER EVENT TRIGGER name RENAME TO new_name
fn alter_event_trigger_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(ALTER_KW) && p.nth_at(1, EVENT_KW) && p.nth_at(2, TRIGGER_KW));
    let m = p.start();
    p.bump(ALTER_KW);
    p.bump(EVENT_KW);
    p.bump(TRIGGER_KW);
    name_ref(p);
    match p.current() {
        DISABLE_KW => {
            p.bump(DISABLE_KW);
        }
        ENABLE_KW => {
            p.bump(ENABLE_KW);
            let _ = p.eat(REPLICA_KW) || p.eat(ALWAYS_KW);
        }
        OWNER_KW => {
            p.bump(OWNER_KW);
            p.expect(TO_KW);
            role(p);
        }
        RENAME_KW => {
            p.bump(RENAME_KW);
            p.expect(TO_KW);
            name(p);
        }
        _ => {
            p.error("expected DISABLE, ENABLE, OWNER, or RENAME");
        }
    }
    m.complete(p, ALTER_EVENT_TRIGGER_STMT)
}

// ALTER EXTENSION name UPDATE [ TO new_version ]
// ALTER EXTENSION name SET SCHEMA new_schema
// ALTER EXTENSION name ADD member_object
// ALTER EXTENSION name DROP member_object
//
// where member_object is:
//   ACCESS METHOD object_name |
//   AGGREGATE aggregate_name ( aggregate_signature ) |
//   CAST (source_type AS target_type) |
//   COLLATION object_name |
//   CONVERSION object_name |
//   DOMAIN object_name |
//   EVENT TRIGGER object_name |
//   FOREIGN DATA WRAPPER object_name |
//   FOREIGN TABLE object_name |
//   FUNCTION function_name [ ( [ [ argmode ] [ argname ] argtype [, ...] ] ) ] |
//   MATERIALIZED VIEW object_name |
//   OPERATOR operator_name (left_type, right_type) |
//   OPERATOR CLASS object_name USING index_method |
//   OPERATOR FAMILY object_name USING index_method |
//   [ PROCEDURAL ] LANGUAGE object_name |
//   PROCEDURE procedure_name [ ( [ [ argmode ] [ argname ] argtype [, ...] ] ) ] |
//   ROUTINE routine_name [ ( [ [ argmode ] [ argname ] argtype [, ...] ] ) ] |
//   SCHEMA object_name |
//   SEQUENCE object_name |
//   SERVER object_name |
//   TABLE object_name |
//   TEXT SEARCH CONFIGURATION object_name |
//   TEXT SEARCH DICTIONARY object_name |
//   TEXT SEARCH PARSER object_name |
//   TEXT SEARCH TEMPLATE object_name |
//   TRANSFORM FOR type_name LANGUAGE lang_name |
//   TYPE object_name |
//   VIEW object_name
//
// and aggregate_signature is:
// * |
// [ argmode ] [ argname ] argtype [ , ... ] |
// [ [ argmode ] [ argname ] argtype [ , ... ] ] ORDER BY [ argmode ] [ argname ] argtype [ , ... ]
fn alter_extension_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(ALTER_KW) && p.nth_at(1, EXTENSION_KW));
    let m = p.start();
    p.bump(ALTER_KW);
    p.bump(EXTENSION_KW);
    name_ref(p);
    match p.current() {
        UPDATE_KW => {
            p.bump(UPDATE_KW);
            if p.eat(TO_KW) {
                if p.at_ts(NON_RESERVED_WORD) {
                    p.bump_any();
                } else {
                    string_literal(p);
                }
            }
        }
        SET_KW => {
            p.bump(SET_KW);
            p.expect(SCHEMA_KW);
            name_ref(p);
        }
        ADD_KW | DROP_KW => {
            p.bump_any();
            match p.current() {
                SCHEMA_KW | DOMAIN_KW | TABLE_KW | TYPE_KW | EXTENSION_KW | PUBLICATION_KW
                | SERVER_KW | DATABASE_KW | ROLE_KW | SUBSCRIPTION_KW | TABLESPACE_KW => {
                    p.bump_any();
                    name_ref(p);
                }
                COLLATION_KW | CONVERSION_KW | SEQUENCE_KW | VIEW_KW | INDEX_KW | STATISTICS_KW => {
                    p.bump_any();
                    path_name_ref(p);
                }
                ACCESS_KW => {
                    p.bump(ACCESS_KW);
                    p.expect(METHOD_KW);
                    name_ref(p);
                }
                AGGREGATE_KW => {
                    p.bump(AGGREGATE_KW);
                    path_name_ref(p);
                    aggregate_arg_list(p);
                }
                CAST_KW => {
                    p.bump(CAST_KW);
                    source_type_as_target_type(p);
                }
                EVENT_KW => {
                    p.bump(EVENT_KW);
                    p.expect(TRIGGER_KW);
                    name_ref(p);
                }
                FOREIGN_KW => {
                    p.bump(FOREIGN_KW);
                    if p.eat(DATA_KW) {
                        p.expect(WRAPPER_KW);
                        name_ref(p);
                    } else {
                        p.expect(TABLE_KW);
                        path_name_ref(p);
                    }
                }
                FUNCTION_KW | PROCEDURE_KW | ROUTINE_KW => {
                    p.bump_any();
                    path_name_ref(p);
                    opt_param_list(p);
                }
                MATERIALIZED_KW => {
                    p.bump(MATERIALIZED_KW);
                    p.expect(VIEW_KW);
                    path_name_ref(p);
                }
                OPERATOR_KW => {
                    p.bump(OPERATOR_KW);
                    match p.current() {
                        CLASS_KW => {
                            p.bump(CLASS_KW);
                            name_ref(p);
                            p.expect(USING_KW);
                            name_ref(p);
                        }
                        FAMILY_KW => {
                            p.bump(FAMILY_KW);
                            name_ref(p);
                            p.expect(USING_KW);
                            name_ref(p);
                        }
                        _ => {
                            operator(p);
                            p.expect(L_PAREN);
                            type_name(p);
                            p.expect(COMMA);
                            type_name(p);
                            p.expect(R_PAREN);
                        }
                    }
                }
                LANGUAGE_KW | PROCEDURAL_KW => {
                    p.eat(PROCEDURAL_KW);
                    p.bump(LANGUAGE_KW);
                    name_ref(p);
                }
                TEXT_KW => {
                    p.bump(TEXT_KW);
                    p.expect(SEARCH_KW);
                    match p.current() {
                        CONFIGURATION_KW => {
                            p.bump(CONFIGURATION_KW);
                            path_name_ref(p);
                        }
                        DICTIONARY_KW => {
                            p.bump(DICTIONARY_KW);
                            path_name_ref(p);
                        }
                        PARSER_KW => {
                            p.bump(PARSER_KW);
                            path_name_ref(p);
                        }
                        TEMPLATE_KW => {
                            p.bump(TEMPLATE_KW);
                            path_name_ref(p);
                        }
                        _ => {
                            p.error("expected CONFIGURATION, DICTIONARY, PARSER, or TEMPLATE after TEXT SEARCH");
                        }
                    }
                }
                TRANSFORM_KW => {
                    p.bump(TRANSFORM_KW);
                    p.expect(FOR_KW);
                    name_ref(p);
                    p.expect(LANGUAGE_KW);
                    name_ref(p);
                }
                _ => {
                    p.error("expected valid extension member object type");
                }
            }
        }
        _ => {
            p.error("expected UPDATE, SET, ADD, or DROP");
        }
    }
    m.complete(p, ALTER_EXTENSION_STMT)
}

// ALTER DOMAIN name
//     { SET DEFAULT expression | DROP DEFAULT }
// ALTER DOMAIN name
//     { SET | DROP } NOT NULL
// ALTER DOMAIN name
//     ADD domain_constraint [ NOT VALID ]
// ALTER DOMAIN name
//     DROP CONSTRAINT [ IF EXISTS ] constraint_name [ RESTRICT | CASCADE ]
// ALTER DOMAIN name
//      RENAME CONSTRAINT constraint_name TO new_constraint_name
// ALTER DOMAIN name
//     VALIDATE CONSTRAINT constraint_name
// ALTER DOMAIN name
//     OWNER TO { new_owner | CURRENT_ROLE | CURRENT_USER | SESSION_USER }
// ALTER DOMAIN name
//     RENAME TO new_name
// ALTER DOMAIN name
//     SET SCHEMA new_schema
//
// where domain_constraint is:
// [ CONSTRAINT constraint_name ]
// { NOT NULL | CHECK (expression) }
fn alter_domain_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(ALTER_KW) && p.nth_at(1, DOMAIN_KW));
    let m = p.start();
    p.bump(ALTER_KW);
    p.bump(DOMAIN_KW);
    path_name_ref(p);
    alter_domain_action(p);
    m.complete(p, ALTER_DOMAIN_STMT)
}

fn alter_domain_action(p: &mut Parser<'_>) -> Option<CompletedMarker> {
    let m = p.start();
    let action = match p.current() {
        SET_KW => {
            p.bump(SET_KW);
            match p.current() {
                DEFAULT_KW => {
                    p.bump(DEFAULT_KW);
                    if expr(p).is_none() {
                        p.error("expected expression");
                    }
                    SET_DEFAULT
                }
                NOT_KW => {
                    p.bump(NOT_KW);
                    p.expect(NULL_KW);
                    SET_NOT_NULL
                }
                SCHEMA_KW => {
                    p.bump(SCHEMA_KW);
                    name_ref(p);
                    SET_SCHEMA
                }
                _ => {
                    p.error("expected DEFAULT, NOT, or SCHEMA");
                    m.abandon(p);
                    return None;
                }
            }
        }
        DROP_KW => {
            p.bump(DROP_KW);
            match p.current() {
                DEFAULT_KW => {
                    p.bump(DEFAULT_KW);
                    DROP_DEFAULT
                }
                NOT_KW => {
                    p.bump(NOT_KW);
                    p.expect(NULL_KW);
                    DROP_NOT_NULL
                }
                CONSTRAINT_KW => {
                    p.bump(CONSTRAINT_KW);
                    opt_if_exists(p);
                    name_ref(p);
                    opt_cascade_or_restrict(p);
                    DROP_CONSTRAINT
                }
                _ => {
                    p.error("expected DEFAULT, NOT, or CONSTRAINT");
                    m.abandon(p);
                    return None;
                }
            }
        }
        ADD_KW => {
            p.bump(ADD_KW);
            domain_constraint(p);
            if p.eat(NOT_KW) {
                p.expect(VALID_KW);
            }
            ADD_CONSTRAINT
        }
        RENAME_KW => {
            p.bump(RENAME_KW);
            if p.eat(CONSTRAINT_KW) {
                name_ref(p);
                p.expect(TO_KW);
                name(p);
                RENAME_CONSTRAINT
            } else {
                p.expect(TO_KW);
                name(p);
                RENAME_TO
            }
        }
        VALIDATE_KW => {
            p.bump(VALIDATE_KW);
            p.expect(CONSTRAINT_KW);
            name_ref(p);
            VALIDATE_CONSTRAINT
        }
        OWNER_KW => {
            p.bump(OWNER_KW);
            p.expect(TO_KW);
            role(p);
            OWNER_TO
        }
        _ => {
            p.error("expected SET, DROP, ADD, RENAME, VALIDATE, or OWNER");
            m.abandon(p);
            return None;
        }
    };
    Some(m.complete(p, action))
}

// [ CONSTRAINT constraint_name ]
// { NOT NULL | CHECK (expression) }
fn domain_constraint(p: &mut Parser<'_>) {
    let m = p.start();
    if p.eat(CONSTRAINT_KW) {
        name(p);
    }
    if p.eat(NOT_KW) {
        p.expect(NULL_KW);
        m.complete(p, NOT_NULL_CONSTRAINT);
    } else if p.eat(CHECK_KW) {
        p.expect(L_PAREN);
        if expr(p).is_none() {
            p.error("expected expression");
        }
        p.expect(R_PAREN);
        m.complete(p, CHECK_CONSTRAINT);
    } else {
        p.error("expected NOT NULL or CHECK constraint");
        m.abandon(p);
    }
}

// ALTER DEFAULT PRIVILEGES
//     [ FOR { ROLE | USER } target_role [, ...] ]
//     [ IN SCHEMA schema_name [, ...] ]
//     abbreviated_grant_or_revoke
//
// where abbreviated_grant_or_revoke is one of:
//   GRANT { { SELECT | INSERT | UPDATE | DELETE | TRUNCATE | REFERENCES | TRIGGER | MAINTAIN }
//       [, ...] | ALL [ PRIVILEGES ] }
//       ON TABLES
//       TO { [ GROUP ] role_name | PUBLIC } [, ...] [ WITH GRANT OPTION ]
//
//   GRANT { { USAGE | SELECT | UPDATE }
//       [, ...] | ALL [ PRIVILEGES ] }
//       ON SEQUENCES
//       TO { [ GROUP ] role_name | PUBLIC } [, ...] [ WITH GRANT OPTION ]
//
//   GRANT { EXECUTE | ALL [ PRIVILEGES ] }
//       ON { FUNCTIONS | ROUTINES }
//       TO { [ GROUP ] role_name | PUBLIC } [, ...] [ WITH GRANT OPTION ]
//
//   GRANT { USAGE | ALL [ PRIVILEGES ] }
//       ON TYPES
//       TO { [ GROUP ] role_name | PUBLIC } [, ...] [ WITH GRANT OPTION ]
//
//   GRANT { { USAGE | CREATE }
//       [, ...] | ALL [ PRIVILEGES ] }
//       ON SCHEMAS
//       TO { [ GROUP ] role_name | PUBLIC } [, ...] [ WITH GRANT OPTION ]
//
//   REVOKE [ GRANT OPTION FOR ]
//       { { SELECT | INSERT | UPDATE | DELETE | TRUNCATE | REFERENCES | TRIGGER | MAINTAIN }
//       [, ...] | ALL [ PRIVILEGES ] }
//       ON TABLES
//       FROM { [ GROUP ] role_name | PUBLIC } [, ...]
//       [ CASCADE | RESTRICT ]
//
//   REVOKE [ GRANT OPTION FOR ]
//       { { USAGE | SELECT | UPDATE }
//       [, ...] | ALL [ PRIVILEGES ] }
//       ON SEQUENCES
//       FROM { [ GROUP ] role_name | PUBLIC } [, ...]
//       [ CASCADE | RESTRICT ]
//
//   REVOKE [ GRANT OPTION FOR ]
//       { EXECUTE | ALL [ PRIVILEGES ] }
//       ON { FUNCTIONS | ROUTINES }
//       FROM { [ GROUP ] role_name | PUBLIC } [, ...]
//       [ CASCADE | RESTRICT ]
//
//   REVOKE [ GRANT OPTION FOR ]
//       { USAGE | ALL [ PRIVILEGES ] }
//       ON TYPES
//       FROM { [ GROUP ] role_name | PUBLIC } [, ...]
//       [ CASCADE | RESTRICT ]
//
//   REVOKE [ GRANT OPTION FOR ]
//       { { USAGE | CREATE }
//       [, ...] | ALL [ PRIVILEGES ] }
//       ON SCHEMAS
//       FROM { [ GROUP ] role_name | PUBLIC } [, ...]
//       [ CASCADE | RESTRICT ]
fn alter_default_privileges_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(ALTER_KW) && p.nth_at(1, DEFAULT_KW) && p.nth_at(2, PRIVILEGES_KW));
    let m = p.start();
    p.bump(ALTER_KW);
    p.bump(DEFAULT_KW);
    p.bump(PRIVILEGES_KW);
    // [ FOR { ROLE | USER } target_role [, ...] ]
    if p.eat(FOR_KW) {
        if !p.eat(ROLE_KW) && !p.eat(USER_KW) {
            p.error("expected ROLE or USER");
        }
        role(p);
        while !p.at(EOF) && p.eat(COMMA) {
            role(p);
        }
    }
    // [ IN SCHEMA schema_name [, ...] ]
    if p.eat(IN_KW) {
        p.expect(SCHEMA_KW);
        name_ref(p);
        while !p.at(EOF) && p.eat(COMMA) {
            name_ref(p);
        }
    }
    match p.current() {
        GRANT_KW => {
            p.bump(GRANT_KW);
            privileges(p);
            p.expect(ON_KW);
            privilege_target(p);
            p.expect(TO_KW);
            role(p);
            while !p.at(EOF) && p.eat(COMMA) {
                role(p);
            }
            if p.eat(WITH_KW) {
                p.expect(GRANT_KW);
                p.expect(OPTION_KW);
            }
        }
        REVOKE_KW => {
            p.bump(REVOKE_KW);
            if p.eat(GRANT_KW) {
                p.eat(OPTION_KW);
                p.eat(FOR_KW);
            }
            privileges(p);
            p.expect(ON_KW);
            privilege_target(p);
            p.expect(FROM_KW);
            role(p);
            while !p.at(EOF) && p.eat(COMMA) {
                role(p);
            }
            opt_cascade_or_restrict(p);
        }
        _ => {
            p.error("expected GRANT or REVOKE");
        }
    }
    m.complete(p, ALTER_DEFAULT_PRIVILEGES_STMT)
}

fn privilege_target(p: &mut Parser<'_>) {
    match p.current() {
        TABLES_KW | FUNCTIONS_KW | ROUTINES_KW | SEQUENCES_KW | TYPES_KW | SCHEMAS_KW => {
            p.bump_any();
        }
        _ => p.error(
            "expected privilege target, TABLES, FUNCTIONS, ROUTINES, SEQEUNCES, TYPES, SCHEMAS",
        ),
    }
}

// ALTER DATABASE name [ [ WITH ] option [ ... ] ]
// where option can be:
//     ALLOW_CONNECTIONS allowconn
//     CONNECTION LIMIT connlimit
//     IS_TEMPLATE istemplate
//
// ALTER DATABASE name RENAME TO new_name
// ALTER DATABASE name OWNER TO { new_owner | CURRENT_ROLE | CURRENT_USER | SESSION_USER }
// ALTER DATABASE name SET TABLESPACE new_tablespace
// ALTER DATABASE name REFRESH COLLATION VERSION
// ALTER DATABASE name SET configuration_parameter { TO | = } { value | DEFAULT }
// ALTER DATABASE name SET configuration_parameter FROM CURRENT
// ALTER DATABASE name RESET configuration_parameter
// ALTER DATABASE name RESET ALL
fn alter_database_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(ALTER_KW) && p.nth_at(1, DATABASE_KW));
    let m = p.start();
    p.bump(ALTER_KW);
    p.bump(DATABASE_KW);
    name_ref(p);
    match p.current() {
        RENAME_KW => {
            p.bump(RENAME_KW);
            p.expect(TO_KW);
            name(p);
        }
        OWNER_KW => {
            p.bump(OWNER_KW);
            p.expect(TO_KW);
            role(p);
        }
        SET_KW if p.nth_at(1, TABLESPACE_KW) => {
            p.bump(SET_KW);
            p.bump(TABLESPACE_KW);
            name_ref(p);
        }
        SET_KW => {
            set_configuration_param(p);
        }
        RESET_KW => {
            p.bump(RESET_KW);
            if !p.eat(ALL_KW) {
                path_name_ref(p);
            }
        }
        REFRESH_KW => {
            p.bump(REFRESH_KW);
            p.expect(COLLATION_KW);
            p.expect(VERSION_KW);
        }
        _ => {
            p.eat(WITH_KW);
            while !p.at(EOF) {
                if !opt_create_database_option(p) {
                    break;
                }
            }
        }
    }
    m.complete(p, ALTER_DATABASE_STMT)
}

// ALTER CONVERSION name RENAME TO new_name
// ALTER CONVERSION name OWNER TO { new_owner | CURRENT_ROLE | CURRENT_USER | SESSION_USER }
// ALTER CONVERSION name SET SCHEMA new_schema
fn alter_conversion_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(ALTER_KW) && p.nth_at(1, CONVERSION_KW));
    let m = p.start();
    p.bump(ALTER_KW);
    p.bump(CONVERSION_KW);
    path_name_ref(p);
    match p.current() {
        RENAME_KW => {
            p.bump(RENAME_KW);
            p.expect(TO_KW);
            name(p);
        }
        OWNER_KW => {
            p.bump(OWNER_KW);
            p.expect(TO_KW);
            role(p);
        }
        SET_KW => {
            p.bump(SET_KW);
            p.expect(SCHEMA_KW);
            name_ref(p);
        }
        _ => {
            p.error("expected RENAME, OWNER, or SET");
        }
    }
    m.complete(p, ALTER_CONVERSION_STMT)
}

// ALTER COLLATION name REFRESH VERSION
// ALTER COLLATION name RENAME TO new_name
// ALTER COLLATION name OWNER TO { new_owner | CURRENT_ROLE | CURRENT_USER | SESSION_USER }
// ALTER COLLATION name SET SCHEMA new_schema
fn alter_collation_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(ALTER_KW) && p.nth_at(1, COLLATION_KW));
    let m = p.start();
    p.bump(ALTER_KW);
    p.bump(COLLATION_KW);
    path_name_ref(p);
    match p.current() {
        REFRESH_KW => {
            p.bump(REFRESH_KW);
            p.expect(VERSION_KW);
        }
        RENAME_KW => {
            p.bump(RENAME_KW);
            p.expect(TO_KW);
            name(p);
        }
        OWNER_KW => {
            p.bump(OWNER_KW);
            p.expect(TO_KW);
            role(p);
        }
        SET_KW => {
            p.bump(SET_KW);
            p.expect(SCHEMA_KW);
            name_ref(p);
        }
        _ => {
            p.error("expected REFRESH, RENAME, OWNER, or SET");
        }
    }
    m.complete(p, ALTER_COLLATION_STMT)
}

// ALTER AGGREGATE name ( aggregate_signature ) RENAME TO new_name
// ALTER AGGREGATE name ( aggregate_signature )
//                 OWNER TO { new_owner | CURRENT_ROLE | CURRENT_USER | SESSION_USER }
// ALTER AGGREGATE name ( aggregate_signature ) SET SCHEMA new_schema
//
// where aggregate_signature is:
// * |
// [ argmode ] [ argname ] argtype [ , ... ] |
// [ [ argmode ] [ argname ] argtype [ , ... ] ] ORDER BY [ argmode ] [ argname ] argtype [ , ... ]
fn alter_aggregate_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(ALTER_KW) && p.nth_at(1, AGGREGATE_KW));
    let m = p.start();
    p.bump(ALTER_KW);
    p.bump(AGGREGATE_KW);
    path_name_ref(p);
    aggregate_arg_list(p);
    match p.current() {
        RENAME_KW => {
            p.bump(RENAME_KW);
            p.expect(TO_KW);
            path_name_ref(p);
        }
        OWNER_KW => {
            p.bump(OWNER_KW);
            p.expect(TO_KW);
            role(p);
        }
        SET_KW => {
            p.bump(SET_KW);
            p.expect(SCHEMA_KW);
            path_name_ref(p);
        }
        _ => {
            p.error("expected RENAME, OWNER, or SET");
        }
    }
    m.complete(p, ALTER_AGGREGATE_STMT)
}

// ALTER SUBSCRIPTION name CONNECTION 'conninfo'
// ALTER SUBSCRIPTION name SET PUBLICATION publication_name [, ...] [ WITH ( publication_option [= value] [, ... ] ) ]
// ALTER SUBSCRIPTION name ADD PUBLICATION publication_name [, ...] [ WITH ( publication_option [= value] [, ... ] ) ]
// ALTER SUBSCRIPTION name DROP PUBLICATION publication_name [, ...] [ WITH ( publication_option [= value] [, ... ] ) ]
// ALTER SUBSCRIPTION name REFRESH PUBLICATION [ WITH ( refresh_option [= value] [, ... ] ) ]
// ALTER SUBSCRIPTION name ENABLE
// ALTER SUBSCRIPTION name DISABLE
// ALTER SUBSCRIPTION name SET ( subscription_parameter [= value] [, ... ] )
// ALTER SUBSCRIPTION name SKIP ( skip_option = value )
// ALTER SUBSCRIPTION name OWNER TO { new_owner | CURRENT_ROLE | CURRENT_USER | SESSION_USER }
// ALTER SUBSCRIPTION name RENAME TO new_name
fn alter_subscription_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(ALTER_KW) && p.nth_at(1, SUBSCRIPTION_KW));
    let m = p.start();
    p.bump(ALTER_KW);
    p.bump(SUBSCRIPTION_KW);
    name_ref(p);
    match p.current() {
        CONNECTION_KW => {
            p.bump(CONNECTION_KW);
            string_literal(p);
        }
        SET_KW if p.nth_at(1, L_PAREN) => {
            p.bump(SET_KW);
            p.expect(L_PAREN);
            while !p.at(EOF) && !p.at(R_PAREN) {
                if !attribute_option(p, AttributeValue::Either) {
                    break;
                }
                if !p.eat(COMMA) {
                    break;
                }
            }
            p.expect(R_PAREN);
        }
        SET_KW | ADD_KW => {
            p.bump_any();
            p.expect(PUBLICATION_KW);
            name(p);
            while !p.at(EOF) && p.eat(COMMA) {
                name(p);
            }
            opt_with_options_list(p);
        }
        DROP_KW => {
            p.bump(DROP_KW);
            p.expect(PUBLICATION_KW);
            name_ref(p);
            while !p.at(EOF) && p.eat(COMMA) {
                name_ref(p);
            }
            opt_with_options_list(p);
        }
        REFRESH_KW => {
            p.bump(REFRESH_KW);
            p.expect(PUBLICATION_KW);
            opt_with_options_list(p);
        }
        ENABLE_KW | DISABLE_KW => {
            p.bump_any();
        }
        SKIP_KW => {
            p.bump(SKIP_KW);
            p.expect(L_PAREN);
            while !p.at(EOF) && !p.at(R_PAREN) {
                if !attribute_option(p, AttributeValue::Either) {
                    break;
                }
                if !p.eat(COMMA) {
                    break;
                }
            }
            p.expect(R_PAREN);
        }
        OWNER_KW => {
            p.bump(OWNER_KW);
            p.expect(TO_KW);
            role(p);
        }
        RENAME_KW => {
            p.bump(RENAME_KW);
            p.expect(TO_KW);
            name(p);
        }
        _ => {
            p.error(
            "expected CONNECTION, SET, ADD, DROP, REFRESH, ENABLE, DISABLE, SKIP, OWNER or RENAME",
        );
        }
    }
    m.complete(p, ALTER_SUBSCRIPTION_STMT)
}

fn opt_with_options_list(p: &mut Parser<'_>) {
    if p.eat(WITH_KW) {
        p.expect(L_PAREN);
        while !p.at(EOF) && !p.at(R_PAREN) {
            if !attribute_option(p, AttributeValue::Either) {
                break;
            }
            if !p.eat(COMMA) {
                break;
            }
        }
        p.expect(R_PAREN);
    }
}

// ALTER SYSTEM SET configuration_parameter { TO | = } { value [, ...] | DEFAULT }
// ALTER SYSTEM RESET configuration_parameter
// ALTER SYSTEM RESET ALL
fn alter_system_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(ALTER_KW) && p.nth_at(1, SYSTEM_KW));
    let m = p.start();
    p.bump(ALTER_KW);
    p.bump(SYSTEM_KW);
    if p.at(SET_KW) {
        set_configuration_param(p);
    } else if p.eat(RESET_KW) {
        if !p.eat(ALL_KW) {
            path_name_ref(p);
        }
    } else {
        p.error("expected SET or RESET after ALTER SYSTEM");
    }
    m.complete(p, ALTER_SYSTEM_STMT)
}

// ALTER TABLESPACE name RENAME TO new_name
// ALTER TABLESPACE name OWNER TO { new_owner | CURRENT_ROLE | CURRENT_USER | SESSION_USER }
// ALTER TABLESPACE name SET ( tablespace_option = value [, ... ] )
// ALTER TABLESPACE name RESET ( tablespace_option [, ... ] )
fn alter_tablespace_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(ALTER_KW) && p.nth_at(1, TABLESPACE_KW));
    let m = p.start();
    p.bump(ALTER_KW);
    p.bump(TABLESPACE_KW);
    path_name_ref(p);
    if p.eat(RENAME_KW) {
        p.expect(TO_KW);
        name(p);
    } else if p.eat(OWNER_KW) {
        p.expect(TO_KW);
        role(p);
    } else if p.eat(SET_KW) || p.eat(RESET_KW) {
        p.expect(L_PAREN);
        while !p.at(EOF) && !p.at(R_PAREN) {
            if !storage_parameter(p) {
                break;
            }
            if !p.eat(COMMA) {
                break;
            }
        }
        p.expect(R_PAREN);
    } else {
        p.error("expected RENAME, OWNER, SET, or RESET after tablespace name");
    }
    m.complete(p, ALTER_TABLESPACE_STMT)
}

// ALTER TEXT SEARCH PARSER name RENAME TO new_name
// ALTER TEXT SEARCH PARSER name SET SCHEMA new_schema
fn alter_text_search_parser_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(
        p.at(ALTER_KW) && p.nth_at(1, TEXT_KW) && p.nth_at(2, SEARCH_KW) && p.nth_at(3, PARSER_KW)
    );
    let m = p.start();
    p.bump(ALTER_KW);
    p.bump(TEXT_KW);
    p.bump(SEARCH_KW);
    p.bump(PARSER_KW);
    path_name_ref(p);
    if p.eat(RENAME_KW) {
        p.expect(TO_KW);
        name_ref(p);
    } else if p.eat(SET_KW) {
        p.expect(SCHEMA_KW);
        name_ref(p);
    } else {
        p.error("expected RENAME TO or SET SCHEMA");
    }
    m.complete(p, ALTER_TEXT_SEARCH_PARSER_STMT)
}

// ALTER TEXT SEARCH DICTIONARY name (
//     option [ = value ] [, ... ]
// )
// ALTER TEXT SEARCH DICTIONARY name RENAME TO new_name
// ALTER TEXT SEARCH DICTIONARY name OWNER TO { new_owner | CURRENT_ROLE | CURRENT_USER | SESSION_USER }
// ALTER TEXT SEARCH DICTIONARY name SET SCHEMA new_schema
fn alter_text_search_dict_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(
        p.at(ALTER_KW)
            && p.nth_at(1, TEXT_KW)
            && p.nth_at(2, SEARCH_KW)
            && p.nth_at(3, DICTIONARY_KW)
    );
    let m = p.start();
    p.bump(ALTER_KW);
    p.bump(TEXT_KW);
    p.bump(SEARCH_KW);
    p.bump(DICTIONARY_KW);
    path_name_ref(p);
    if p.eat(L_PAREN) {
        while !p.at(EOF) {
            if !attribute_option(p, AttributeValue::Either) {
                break;
            }
            if !p.eat(COMMA) {
                break;
            }
        }
        p.expect(R_PAREN);
    } else if p.eat(RENAME_KW) {
        p.expect(TO_KW);
        name(p);
    } else if p.eat(OWNER_KW) {
        p.expect(TO_KW);
        role(p);
    } else if p.eat(SET_KW) {
        p.expect(SCHEMA_KW);
        name(p);
    } else {
        p.error("expected '(', RENAME, OWNER, or SET");
    }
    m.complete(p, ALTER_TEXT_SEARCH_DICTIONARY_STMT)
}

// ALTER TEXT SEARCH CONFIGURATION name
//     ADD MAPPING FOR token_type [, ... ] WITH dictionary_name [, ... ]
// ALTER TEXT SEARCH CONFIGURATION name
//     ALTER MAPPING FOR token_type [, ... ] WITH dictionary_name [, ... ]
// ALTER TEXT SEARCH CONFIGURATION name
//     ALTER MAPPING REPLACE old_dictionary WITH new_dictionary
// ALTER TEXT SEARCH CONFIGURATION name
//     ALTER MAPPING FOR token_type [, ... ] REPLACE old_dictionary WITH new_dictionary
// ALTER TEXT SEARCH CONFIGURATION name
//     DROP MAPPING [ IF EXISTS ] FOR token_type [, ... ]
// ALTER TEXT SEARCH CONFIGURATION name
//     RENAME TO new_name
// ALTER TEXT SEARCH CONFIGURATION name
//     OWNER TO { new_owner | CURRENT_ROLE | CURRENT_USER | SESSION_USER }
// ALTER TEXT SEARCH CONFIGURATION name
//     SET SCHEMA new_schema
fn alter_text_search_configuration(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(
        p.at(ALTER_KW)
            && p.nth_at(1, TEXT_KW)
            && p.nth_at(2, SEARCH_KW)
            && p.nth_at(3, CONFIGURATION_KW)
    );
    let m = p.start();
    p.bump(ALTER_KW);
    p.bump(TEXT_KW);
    p.bump(SEARCH_KW);
    p.bump(CONFIGURATION_KW);
    path_name_ref(p);
    match p.current() {
        // ADD MAPPING FOR token_type
        ADD_KW => {
            p.bump(ADD_KW);
            p.expect(MAPPING_KW);
            p.expect(FOR_KW);
            name_ref_list(p);
            p.expect(WITH_KW);
            path_name_ref_list(p);
        }
        // ALTER MAPPING FOR
        // ALTER MAPPING REPLACE
        // ALTER MAPPING FOR ... REPLACE
        ALTER_KW => {
            p.bump(ALTER_KW);
            p.expect(MAPPING_KW);
            if p.eat(FOR_KW) {
                name_ref_list(p);
                if p.eat(WITH_KW) {
                    path_name_ref_list(p);
                } else if p.eat(REPLACE_KW) {
                    path_name_ref(p);
                    p.expect(WITH_KW);
                    path_name_ref(p);
                } else {
                    p.error("expected WITH or REPLACE");
                }
            } else if p.eat(REPLACE_KW) {
                path_name_ref(p);
                p.expect(WITH_KW);
                path_name_ref(p);
            } else {
                p.error("expected FOR or REPLACE");
            }
        }
        DROP_KW => {
            p.bump(DROP_KW);
            p.expect(MAPPING_KW);
            opt_if_exists(p);
            p.expect(FOR_KW);
            name_ref_list(p);
        }
        RENAME_KW => {
            p.bump(RENAME_KW);
            p.expect(TO_KW);
            path_name_ref(p);
        }
        OWNER_KW => {
            p.bump(OWNER_KW);
            p.expect(TO_KW);
            role(p);
        }
        SET_KW => {
            p.bump(SET_KW);
            p.expect(SCHEMA_KW);
            path_name_ref(p);
        }
        _ => {
            p.error("expected ADD, ALTER, DROP, RENAME, OWNER, or SET");
        }
    }
    m.complete(p, ALTER_TEXT_SEARCH_CONFIGURATION_STMT)
}

fn name_ref_list(p: &mut Parser<'_>) {
    name_ref(p);
    while !p.at(EOF) && p.eat(COMMA) {
        name_ref(p);
    }
}

fn path_name_ref_list(p: &mut Parser<'_>) {
    path_name_ref(p);
    while !p.at(EOF) && p.eat(COMMA) {
        path_name_ref(p);
    }
}

// ALTER TEXT SEARCH TEMPLATE name RENAME TO new_name
// ALTER TEXT SEARCH TEMPLATE name SET SCHEMA new_schema
fn alter_text_search_template_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(
        p.at(ALTER_KW)
            && p.nth_at(1, TEXT_KW)
            && p.nth_at(2, SEARCH_KW)
            && p.nth_at(3, TEMPLATE_KW)
    );
    let m = p.start();
    p.bump(ALTER_KW);
    p.bump(TEXT_KW);
    p.bump(SEARCH_KW);
    p.bump(TEMPLATE_KW);
    path_name_ref(p);
    if p.eat(RENAME_KW) {
        p.expect(TO_KW);
        name(p);
    } else if p.eat(SET_KW) {
        p.expect(SCHEMA_KW);
        name(p);
    } else {
        p.error("expected RENAME TO or SET SCHEMA");
    }
    m.complete(p, ALTER_TEXT_SEARCH_TEMPLATE_STMT)
}

// ALTER TRIGGER name ON table_name RENAME TO new_name
// ALTER TRIGGER name ON table_name [ NO ] DEPENDS ON EXTENSION extension_name
fn alter_trigger_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(ALTER_KW) && p.nth_at(1, TRIGGER_KW));
    let m = p.start();
    p.bump(ALTER_KW);
    p.bump(TRIGGER_KW);
    name_ref(p);
    p.expect(ON_KW);
    path_name_ref(p);
    if p.eat(RENAME_KW) {
        p.expect(TO_KW);
        name_ref(p);
    } else {
        p.eat(NO_KW);
        p.expect(DEPENDS_KW);
        p.expect(ON_KW);
        p.expect(EXTENSION_KW);
        name_ref(p);
    }
    m.complete(p, ALTER_TRIGGER_STMT)
}

fn alter_type_action(p: &mut Parser<'_>) {
    if p.eat(ADD_KW) {
        p.expect(ATTRIBUTE_KW);
        name(p);
        type_name(p);
        opt_collate(p);
        opt_cascade_or_restrict(p);
    } else if p.eat(DROP_KW) {
        p.expect(ATTRIBUTE_KW);
        opt_if_exists(p);
        name_ref(p);
        opt_cascade_or_restrict(p);
    } else {
        p.expect(ALTER_KW);
        p.expect(ATTRIBUTE_KW);
        name_ref(p);
        if p.eat(SET_KW) {
            p.eat(DATA_KW);
        }
        p.expect(TYPE_KW);
        type_name(p);
        opt_collate(p);
        opt_cascade_or_restrict(p);
    }
}

// ALTER TYPE name OWNER TO { new_owner | CURRENT_ROLE | CURRENT_USER | SESSION_USER }
// ALTER TYPE name RENAME TO new_name
// ALTER TYPE name SET SCHEMA new_schema
// ALTER TYPE name RENAME ATTRIBUTE attribute_name TO new_attribute_name [ CASCADE | RESTRICT ]
// ALTER TYPE name action [, ... ]
// ALTER TYPE name ADD VALUE [ IF NOT EXISTS ] new_enum_value [ { BEFORE | AFTER } neighbor_enum_value ]
// ALTER TYPE name RENAME VALUE existing_enum_value TO new_enum_value
// ALTER TYPE name SET ( property = value [, ... ] )
//
// where action is one of:
//     ADD ATTRIBUTE attribute_name data_type [ COLLATE collation ] [ CASCADE | RESTRICT ]
//     DROP ATTRIBUTE [ IF EXISTS ] attribute_name [ CASCADE | RESTRICT ]
//     ALTER ATTRIBUTE attribute_name [ SET DATA ] TYPE data_type [ COLLATE collation ] [ CASCADE | RESTRICT ]
fn alter_type_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(ALTER_KW) && p.nth_at(1, TYPE_KW));
    let m = p.start();
    p.bump(ALTER_KW);
    p.bump(TYPE_KW);
    path_name_ref(p);
    match p.current() {
        ADD_KW | DROP_KW | ALTER_KW if p.nth_at(1, ATTRIBUTE_KW) => {
            alter_type_action(p);
            while !p.at(EOF) && p.eat(COMMA) {
                alter_type_action(p);
            }
        }
        OWNER_KW => {
            p.bump(OWNER_KW);
            p.expect(TO_KW);
            role(p);
        }
        SET_KW => {
            p.bump(SET_KW);
            if p.eat(SCHEMA_KW) {
                name_ref(p);
            } else {
                p.expect(L_PAREN);
                while !p.at(EOF) {
                    if !attribute_option(p, AttributeValue::Either) {
                        break;
                    }
                    if !p.eat(COMMA) {
                        break;
                    }
                }
                p.expect(R_PAREN);
            }
        }
        RENAME_KW => {
            p.bump(RENAME_KW);
            if p.eat(TO_KW) {
                name(p);
            } else if p.eat(ATTRIBUTE_KW) {
                name_ref(p);
                p.expect(TO_KW);
                name(p);
                opt_cascade_or_restrict(p);
            } else if p.eat(VALUE_KW) {
                string_literal(p);
                p.expect(TO_KW);
                string_literal(p);
            } else {
                p.error("expected TO, ATTRIBUTE, or VALUE");
            }
        }
        ADD_KW => {
            p.bump(ADD_KW);
            if p.eat(VALUE_KW) {
                opt_if_not_exists(p);
                string_literal(p);
                if p.eat(BEFORE_KW) || p.eat(AFTER_KW) {
                    string_literal(p);
                }
            } else if p.eat(ATTRIBUTE_KW) {
                name(p);
                type_name(p);
                opt_collate(p);
                opt_cascade_or_restrict(p);
            } else {
                p.error("expected VALUE or ATTRIBUTE");
            }
        }
        _ => p.error("expected ALTER TYPE option"),
    }
    m.complete(p, ALTER_TYPE_STMT)
}

// ALTER USER role_specification [ WITH ] option [ ... ]
// where option can be:
//       SUPERUSER | NOSUPERUSER
//     | CREATEDB | NOCREATEDB
//     | CREATEROLE | NOCREATEROLE
//     | INHERIT | NOINHERIT
//     | LOGIN | NOLOGIN
//     | REPLICATION | NOREPLICATION
//     | BYPASSRLS | NOBYPASSRLS
//     | CONNECTION LIMIT connlimit
//     | [ ENCRYPTED ] PASSWORD 'password' | PASSWORD NULL
//     | VALID UNTIL 'timestamp'
//
// ALTER USER name RENAME TO new_name
// ALTER USER { role_specification | ALL } [ IN DATABASE database_name ] SET configuration_parameter { TO | = } { value | DEFAULT }
// ALTER USER { role_specification | ALL } [ IN DATABASE database_name ] SET configuration_parameter FROM CURRENT
// ALTER USER { role_specification | ALL } [ IN DATABASE database_name ] RESET configuration_parameter
// ALTER USER { role_specification | ALL } [ IN DATABASE database_name ] RESET ALL
//
// where role_specification can be:
//     role_name
//   | CURRENT_ROLE
//   | CURRENT_USER
//   | SESSION_USER
fn alter_user_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(ALTER_KW) && p.nth_at(1, USER_KW));
    let m = p.start();
    p.bump(ALTER_KW);
    p.bump(USER_KW);
    if !p.eat(ALL_KW) {
        role(p);
    }
    // be careful of the case where we're at the IN of IN DATABASE
    if p.eat(WITH_KW) || (p.at_ts(ROLE_OPTION_FIRST) && !p.nth_at(1, DATABASE_KW)) {
        opt_role_option(p);
        while !p.at(EOF) && p.at_ts(ROLE_OPTION_FIRST) {
            opt_role_option(p);
        }
        return m.complete(p, ALTER_USER_STMT);
    }
    // RENAME TO new_name
    if p.eat(RENAME_KW) {
        p.expect(TO_KW);
        name_ref(p);
        return m.complete(p, ALTER_USER_STMT);
    }
    if p.eat(IN_KW) {
        p.expect(DATABASE_KW);
        name_ref(p);
    }
    match p.current() {
        SET_KW => {
            set_configuration_param(p);
        }
        RESET_KW => {
            p.bump(RESET_KW);
            if !p.eat(ALL_KW) {
                name_ref(p);
            }
        }
        _ => p.error("expected SET or RESET"),
    }
    m.complete(p, ALTER_USER_STMT)
}

// ALTER USER MAPPING FOR { user_name | USER | CURRENT_ROLE | CURRENT_USER | SESSION_USER | PUBLIC }
//     SERVER server_name
//     OPTIONS ( [ ADD | SET | DROP ] option ['value'] [, ... ] )
fn alter_user_mapping_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(ALTER_KW) && p.nth_at(1, USER_KW) && p.nth_at(2, MAPPING_KW));
    let m = p.start();
    p.bump(ALTER_KW);
    p.bump(USER_KW);
    p.bump(MAPPING_KW);
    p.expect(FOR_KW);
    role(p);
    p.expect(SERVER_KW);
    name_ref(p);
    p.expect(OPTIONS_KW);
    p.expect(L_PAREN);
    alter_option(p);
    while !p.at(EOF) && p.eat(COMMA) {
        alter_option(p);
    }
    p.expect(R_PAREN);
    m.complete(p, ALTER_USER_MAPPING_STMT)
}

fn alter_option(p: &mut Parser<'_>) {
    let arg_required = match p.current() {
        DROP_KW => {
            p.bump(DROP_KW);
            false
        }
        ADD_KW | SET_KW => {
            p.bump_any();
            true
        }
        _ => true,
    };
    if arg_required {
        let _ = p.eat(ADD_KW) || p.eat(SET_KW);
    }
    col_label(p);
    if arg_required {
        string_literal(p);
    }
}

// ALTER VIEW [ IF EXISTS ] name ALTER [ COLUMN ] column_name SET DEFAULT expression
// ALTER VIEW [ IF EXISTS ] name ALTER [ COLUMN ] column_name DROP DEFAULT
// ALTER VIEW [ IF EXISTS ] name OWNER TO { new_owner | CURRENT_ROLE | CURRENT_USER | SESSION_USER }
// ALTER VIEW [ IF EXISTS ] name RENAME [ COLUMN ] column_name TO new_column_name
// ALTER VIEW [ IF EXISTS ] name RENAME TO new_name
// ALTER VIEW [ IF EXISTS ] name SET SCHEMA new_schema
// ALTER VIEW [ IF EXISTS ] name SET ( view_option_name [= view_option_value] [, ... ] )
// ALTER VIEW [ IF EXISTS ] name RESET ( view_option_name [, ... ] )
fn alter_view_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(ALTER_KW) && p.nth_at(1, VIEW_KW));
    let m = p.start();
    p.bump(ALTER_KW);
    p.bump(VIEW_KW);
    opt_if_exists(p);
    path_name_ref(p);
    match p.current() {
        ALTER_KW => {
            p.bump(ALTER_KW);
            p.eat(COLUMN_KW);
            name_ref(p);
            if p.eat(SET_KW) {
                p.expect(DEFAULT_KW);
                if expr(p).is_none() {
                    p.error("expected expression")
                }
            } else if p.eat(DROP_KW) {
                p.expect(DEFAULT_KW);
            } else {
                p.error("expected SET or DROP");
            }
        }
        OWNER_KW => {
            p.bump(OWNER_KW);
            p.expect(TO_KW);
            role(p);
        }
        RENAME_KW => {
            p.bump(RENAME_KW);
            if p.eat(TO_KW) {
                name(p);
            } else {
                p.eat(COLUMN_KW);
                name_ref(p);
                p.expect(TO_KW);
                name(p);
            }
        }
        SET_KW => {
            p.bump(SET_KW);
            if p.eat(SCHEMA_KW) {
                name_ref(p);
            } else {
                p.expect(L_PAREN);
                while !p.at(EOF) {
                    if !attribute_option(p, AttributeValue::Either) {
                        break;
                    }
                    if !p.eat(COMMA) {
                        break;
                    }
                }
                p.expect(R_PAREN);
            }
        }
        RESET_KW => {
            p.bump(RESET_KW);
            p.expect(L_PAREN);
            while !p.at(EOF) {
                if !attribute_option(p, AttributeValue::Either) {
                    break;
                }
                if !p.eat(COMMA) {
                    break;
                }
            }
            p.expect(R_PAREN);
        }
        _ => p.error("expected ALTER, OWNER, RENAME, or SET"),
    }
    m.complete(p, ALTER_VIEW_STMT)
}

// ANALYZE [ ( option [, ...] ) ] [ table_and_columns [, ...] ]
// where option can be one of:
//     VERBOSE [ boolean ]
//     SKIP_LOCKED [ boolean ]
//     BUFFER_USAGE_LIMIT size
// and table_and_columns is:
//     table_name [ ( column_name [, ...] ) ]
fn analyze_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(ANALYZE_KW) || p.at(ANALYSE_KW));
    let m = p.start();
    p.bump_any();
    if !p.eat(VERBOSE_KW) {
        if p.eat(L_PAREN) {
            cluster_option(p);
            while !p.at(EOF) && p.eat(COMMA) {
                cluster_option(p);
            }
            p.expect(R_PAREN);
        }
    }
    opt_relation_list(p);
    m.complete(p, ANALYZE_STMT)
}

// COMMENT ON
// {
//   ACCESS METHOD object_name |
//   AGGREGATE aggregate_name ( aggregate_signature ) |
//   CAST (source_type AS target_type) |
//   COLLATION object_name |
//   COLUMN relation_name.column_name |
//   CONSTRAINT constraint_name ON table_name |
//   CONSTRAINT constraint_name ON DOMAIN domain_name |
//   CONVERSION object_name |
//   DATABASE object_name |
//   DOMAIN object_name |
//   EXTENSION object_name |
//   EVENT TRIGGER object_name |
//   FOREIGN DATA WRAPPER object_name |
//   FOREIGN TABLE object_name |
//   FUNCTION function_name [ ( [ [ argmode ] [ argname ] argtype [, ...] ] ) ] |
//   INDEX object_name |
//   LARGE OBJECT large_object_oid |
//   MATERIALIZED VIEW object_name |
//   OPERATOR operator_name (left_type, right_type) |
//   OPERATOR CLASS object_name USING index_method |
//   OPERATOR FAMILY object_name USING index_method |
//   POLICY policy_name ON table_name |
//   [ PROCEDURAL ] LANGUAGE object_name |
//   PROCEDURE procedure_name [ ( [ [ argmode ] [ argname ] argtype [, ...] ] ) ] |
//   PUBLICATION object_name |
//   ROLE object_name |
//   ROUTINE routine_name [ ( [ [ argmode ] [ argname ] argtype [, ...] ] ) ] |
//   RULE rule_name ON table_name |
//   SCHEMA object_name |
//   SEQUENCE object_name |
//   SERVER object_name |
//   STATISTICS object_name |
//   SUBSCRIPTION object_name |
//   TABLE object_name |
//   TABLESPACE object_name |
//   TEXT SEARCH CONFIGURATION object_name |
//   TEXT SEARCH DICTIONARY object_name |
//   TEXT SEARCH PARSER object_name |
//   TEXT SEARCH TEMPLATE object_name |
//   TRANSFORM FOR type_name LANGUAGE lang_name |
//   TRIGGER trigger_name ON table_name |
//   TYPE object_name |
//   VIEW object_name
// } IS { string_literal | NULL }
//
// where aggregate_signature is:
//   * |
//   [ argmode ] [ argname ] argtype [ , ... ] |
//   [ [ argmode ] [ argname ] argtype [ , ... ] ] ORDER BY [ argmode ] [ argname ] argtype [ , ... ]
fn comment_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(COMMENT_KW));
    let m = p.start();
    p.bump(COMMENT_KW);
    p.expect(ON_KW);
    match p.current() {
        ACCESS_KW => {
            p.bump_any();
            p.expect(METHOD_KW);
            path_name_ref(p);
        }
        AGGREGATE_KW => {
            p.bump_any();
            path_name_ref(p);
            aggregate_arg_list(p);
        }
        CAST_KW => {
            p.bump_any();
            source_type_as_target_type(p);
        }
        COLLATION_KW | COLUMN_KW | CONVERSION_KW | DATABASE_KW | DOMAIN_KW | EXTENSION_KW
        | INDEX_KW | LANGUAGE_KW | PUBLICATION_KW | ROLE_KW | SCHEMA_KW | SEQUENCE_KW
        | SERVER_KW | STATISTICS_KW | SUBSCRIPTION_KW | TABLE_KW | TABLESPACE_KW | TYPE_KW
        | VIEW_KW => {
            p.bump_any();
            path_name_ref(p);
        }
        CONSTRAINT_KW => {
            p.bump_any();
            name_ref(p);
            p.expect(ON_KW);
            p.eat(DOMAIN_KW);
            path_name_ref(p);
        }
        EVENT_KW => {
            p.bump_any();
            p.expect(TRIGGER_KW);
            path_name_ref(p);
        }
        FOREIGN_KW if p.nth_at(1, DATA_KW) => {
            p.bump_any();
            p.bump(DATA_KW);
            p.expect(WRAPPER_KW);
            path_name_ref(p);
        }
        FOREIGN_KW => {
            p.bump_any();
            p.expect(TABLE_KW);
            path_name_ref(p);
        }
        FUNCTION_KW | PROCEDURE_KW | ROUTINE_KW => {
            p.bump_any();
            path_name_ref(p);
            opt_param_list(p);
        }
        LARGE_KW => {
            p.bump_any();
            p.expect(OBJECT_KW);
            if opt_numeric_literal(p).is_none() {
                p.error("expected object oid");
            }
        }
        MATERIALIZED_KW => {
            p.bump_any();
            p.expect(VIEW_KW);
            path_name_ref(p);
        }
        OPERATOR_KW if matches!(p.nth(1), CLASS_KW | FAMILY_KW) => {
            p.bump_any();
            p.bump_any();
            path_name_ref(p);
            if p.eat(USING_KW) {
                name_ref(p);
            }
        }
        OPERATOR_KW => {
            p.bump_any();
            operator(p);
            p.eat(L_PAREN);
            type_name(p);
            p.expect(COMMA);
            type_name(p);
            p.eat(R_PAREN);
        }
        POLICY_KW | RULE_KW | TRIGGER_KW => {
            p.bump_any();
            name_ref(p);
            p.expect(ON_KW);
            path_name_ref(p);
        }
        PROCEDURAL_KW => {
            p.bump_any();
            p.expect(LANGUAGE_KW);
            path_name_ref(p);
        }
        TEXT_KW => {
            p.bump_any();
            p.expect(SEARCH_KW);
            match p.current() {
                CONFIGURATION_KW | DICTIONARY_KW | PARSER_KW | TEMPLATE_KW => {
                    p.bump_any();
                    path_name_ref(p);
                }
                _ => p.error("expected CONFIGURATION, DICTIONARY, PARSER, or TEMPLATE"),
            }
        }
        TRANSFORM_KW => {
            p.bump_any();
            p.expect(FOR_KW);
            type_name(p);
            p.expect(LANGUAGE_KW);
            name_ref(p);
        }
        _ => p.err_and_bump("unexpected token"),
    }
    p.expect(IS_KW);
    if !p.eat(NULL_KW) && opt_string_literal(p).is_none() {
        p.error("expected string literal or NULL");
    }
    m.complete(p, COMMENT_STMT)
}

// CLUSTER [ ( option [, ...] ) ] [ table_name [ USING index_name ] ]
// where option can be one of:
//   VERBOSE [ boolean ]
fn cluster_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(CLUSTER_KW));
    let m = p.start();
    p.bump(CLUSTER_KW);
    if p.eat(VERBOSE_KW) {
        // pass
    } else if p.eat(L_PAREN) {
        cluster_option(p);
        while !p.at(EOF) && p.eat(COMMA) {
            cluster_option(p);
        }
        p.expect(R_PAREN);
    }
    let has_name = opt_path_name_ref(p).is_some();
    if has_name && p.eat(ON_KW) {
        path_name_ref(p);
    }
    if p.eat(USING_KW) {
        name_ref(p);
    }
    m.complete(p, CLUSTER_STMT)
}

fn cluster_option(p: &mut Parser<'_>) {
    // option name
    if p.at_ts(NON_RESERVED_WORD) || p.at(ANALYSE_KW) || p.at(ANALYZE_KW) || p.at(FORMAT_KW) {
        p.bump_any();
    } else {
        p.error("expected cluster option");
    }
    opt_option_value(p);
}

// CREATE ACCESS METHOD name
//     TYPE access_method_type
//     HANDLER handler_function
fn create_access_method_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(CREATE_KW));
    let m = p.start();
    p.bump(CREATE_KW);
    p.expect(ACCESS_KW);
    p.expect(METHOD_KW);
    path_name(p);
    p.expect(TYPE_KW);
    if !p.eat(TABLE_KW) && !p.eat(INDEX_KW) {
        p.error("expected TABLE or INDEX");
    }
    p.expect(HANDLER_KW);
    path_name(p);
    m.complete(p, CREATE_ACCESS_METHOD_STMT)
}

// CREATE [ OR REPLACE ] AGGREGATE name ( [ argmode ] [ argname ] arg_data_type [ , ... ] ) (
//     SFUNC = sfunc,
//     STYPE = state_data_type
//     [ , SSPACE = state_data_size ]
//     [ , FINALFUNC = ffunc ]
//     [ , FINALFUNC_EXTRA ]
//     [ , FINALFUNC_MODIFY = { READ_ONLY | SHAREABLE | READ_WRITE } ]
//     [ , COMBINEFUNC = combinefunc ]
//     [ , SERIALFUNC = serialfunc ]
//     [ , DESERIALFUNC = deserialfunc ]
//     [ , INITCOND = initial_condition ]
//     [ , MSFUNC = msfunc ]
//     [ , MINVFUNC = minvfunc ]
//     [ , MSTYPE = mstate_data_type ]
//     [ , MSSPACE = mstate_data_size ]
//     [ , MFINALFUNC = mffunc ]
//     [ , MFINALFUNC_EXTRA ]
//     [ , MFINALFUNC_MODIFY = { READ_ONLY | SHAREABLE | READ_WRITE } ]
//     [ , MINITCOND = minitial_condition ]
//     [ , SORTOP = sort_operator ]
//     [ , PARALLEL = { SAFE | RESTRICTED | UNSAFE } ]
// )
//
// CREATE [ OR REPLACE ] AGGREGATE name ( [ [ argmode ] [ argname ] arg_data_type [ , ... ] ]
//                         ORDER BY [ argmode ] [ argname ] arg_data_type [ , ... ] ) (
//     SFUNC = sfunc,
//     STYPE = state_data_type
//     [ , SSPACE = state_data_size ]
//     [ , FINALFUNC = ffunc ]
//     [ , FINALFUNC_EXTRA ]
//     [ , FINALFUNC_MODIFY = { READ_ONLY | SHAREABLE | READ_WRITE } ]
//     [ , INITCOND = initial_condition ]
//     [ , PARALLEL = { SAFE | RESTRICTED | UNSAFE } ]
//     [ , HYPOTHETICAL ]
// )
//
// or the old syntax
//
// CREATE [ OR REPLACE ] AGGREGATE name (
//     BASETYPE = base_type,
//     SFUNC = sfunc,
//     STYPE = state_data_type
//     [ , SSPACE = state_data_size ]
//     [ , FINALFUNC = ffunc ]
//     [ , FINALFUNC_EXTRA ]
//     [ , FINALFUNC_MODIFY = { READ_ONLY | SHAREABLE | READ_WRITE } ]
//     [ , COMBINEFUNC = combinefunc ]
//     [ , SERIALFUNC = serialfunc ]
//     [ , DESERIALFUNC = deserialfunc ]
//     [ , INITCOND = initial_condition ]
//     [ , MSFUNC = msfunc ]
//     [ , MINVFUNC = minvfunc ]
//     [ , MSTYPE = mstate_data_type ]
//     [ , MSSPACE = mstate_data_size ]
//     [ , MFINALFUNC = mffunc ]
//     [ , MFINALFUNC_EXTRA ]
//     [ , MFINALFUNC_MODIFY = { READ_ONLY | SHAREABLE | READ_WRITE } ]
//     [ , MINITCOND = minitial_condition ]
//     [ , SORTOP = sort_operator ]
// )
fn create_aggregate_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(CREATE_KW));
    let m = p.start();
    p.bump(CREATE_KW);
    opt_or_replace(p);
    p.expect(AGGREGATE_KW);
    path_name(p);
    let at_old_syntax = p.at(L_PAREN) && p.nth_at(1, IDENT) && p.nth_at(2, EQ);
    if !at_old_syntax {
        aggregate_arg_list(p);
    }
    p.expect(L_PAREN);
    while !p.at(EOF) {
        if !attribute_option(p, AttributeValue::Either) {
            break;
        }
        if !p.eat(COMMA) {
            break;
        }
    }
    p.expect(R_PAREN);
    m.complete(p, CREATE_AGGREGATE_STMT)
}

// CREATE CAST (source_type AS target_type)
//     WITH FUNCTION function_name [ (argument_type [, ...]) ]
//     [ AS ASSIGNMENT | AS IMPLICIT ]
//
// CREATE CAST (source_type AS target_type)
//     WITHOUT FUNCTION
//     [ AS ASSIGNMENT | AS IMPLICIT ]
//
// CREATE CAST (source_type AS target_type)
//     WITH INOUT
//     [ AS ASSIGNMENT | AS IMPLICIT ]
fn create_cast_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(CREATE_KW) && p.nth_at(1, CAST_KW));
    let m = p.start();
    p.bump(CREATE_KW);
    p.bump(CAST_KW);
    source_type_as_target_type(p);
    if p.eat(WITH_KW) {
        if !p.eat(INOUT_KW) {
            p.expect(FUNCTION_KW);
            path_name_ref(p);
            opt_param_list(p);
        }
    } else {
        p.expect(WITHOUT_KW);
        p.expect(FUNCTION_KW);
    }
    // [ AS ASSIGNMENT | AS IMPLICIT ]
    if p.eat(AS_KW) {
        if !p.eat(ASSIGNMENT_KW) && !p.eat(IMPLICIT_KW) {
            p.error("expected ASSIGNMENT or IMPLICIT");
        }
    }
    m.complete(p, CREATE_CAST_STMT)
}

// CREATE COLLATION [ IF NOT EXISTS ] name (
//     [ LOCALE = locale, ]
//     [ LC_COLLATE = lc_collate, ]
//     [ LC_CTYPE = lc_ctype, ]
//     [ PROVIDER = provider, ]
//     [ DETERMINISTIC = boolean, ]
//     [ RULES = rules, ]
//     [ VERSION = version ]
// )
// CREATE COLLATION [ IF NOT EXISTS ] name FROM existing_collation
fn create_collation_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(CREATE_KW) && p.nth_at(1, COLLATION_KW));
    let m = p.start();
    p.bump(CREATE_KW);
    p.bump(COLLATION_KW);
    opt_if_not_exists(p);
    path_name(p);
    if p.eat(FROM_KW) {
        path_name_ref(p);
    } else {
        p.expect(L_PAREN);
        while !p.at(EOF) {
            if !attribute_option(p, AttributeValue::Required) {
                break;
            }
            if !p.eat(COMMA) {
                break;
            }
        }
        p.expect(R_PAREN);
    }
    m.complete(p, CREATE_COLLATION_STMT)
}

// CREATE [ DEFAULT ] CONVERSION name
//     FOR source_encoding TO dest_encoding FROM function_name
fn create_conversion_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(CREATE_KW));
    let m = p.start();
    p.bump(CREATE_KW);
    p.eat(DEFAULT_KW);
    p.expect(CONVERSION_KW);
    path_name(p);
    p.expect(FOR_KW);
    string_literal(p);
    p.expect(TO_KW);
    string_literal(p);
    p.expect(FROM_KW);
    path_name(p);
    m.complete(p, CREATE_CONVERSION_STMT)
}

fn opt_option_value(p: &mut Parser<'_>) -> bool {
    if opt_numeric_literal(p).is_none()
        && opt_string_literal(p).is_none()
        && !opt_bool_literal(p)
        && !p.eat(DEFAULT_KW)
    {
        if p.at_ts(NON_RESERVED_WORD) {
            p.bump_any();
            return true;
        } else {
            return false;
        }
    }
    true
}

fn opt_create_database_option(p: &mut Parser<'_>) -> bool {
    p.eat(WITH_KW);
    // option name
    match p.current() {
        OWNER_KW | TEMPLATE_KW | ENCODING_KW | IDENT | TABLESPACE_KW => {
            p.bump_any();
        }
        CONNECTION_KW => {
            p.bump(CONNECTION_KW);
            p.expect(LIMIT_KW);
        }
        _ => return false,
    }
    p.eat(EQ);
    if !opt_option_value(p) {
        p.error("expected create database option value");
        return false;
    }
    true
}

// CREATE DATABASE name
//     [ WITH ] [ OWNER [=] user_name ]
//            [ TEMPLATE [=] template ]
//            [ ENCODING [=] encoding ]
//            [ STRATEGY [=] strategy ]
//            [ LOCALE [=] locale ]
//            [ LC_COLLATE [=] lc_collate ]
//            [ LC_CTYPE [=] lc_ctype ]
//            [ BUILTIN_LOCALE [=] builtin_locale ]
//            [ ICU_LOCALE [=] icu_locale ]
//            [ ICU_RULES [=] icu_rules ]
//            [ LOCALE_PROVIDER [=] locale_provider ]
//            [ COLLATION_VERSION = collation_version ]
//            [ TABLESPACE [=] tablespace_name ]
//            [ ALLOW_CONNECTIONS [=] allowconn ]
//            [ CONNECTION LIMIT [=] connlimit ]
//            [ IS_TEMPLATE [=] istemplate ]
//            [ OID [=] oid ]
fn create_database_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(CREATE_KW) && p.nth_at(1, DATABASE_KW));
    let m = p.start();
    p.bump(CREATE_KW);
    p.bump(DATABASE_KW);
    name(p);
    while !p.at(EOF) {
        if !opt_create_database_option(p) {
            break;
        }
    }
    m.complete(p, CREATE_DATABASE_STMT)
}

// CREATE DOMAIN name [ AS ] data_type
//     [ COLLATE collation ]
//     [ DEFAULT expression ]
//     [ domain_constraint [ ... ] ]
// where domain_constraint is:
// [ CONSTRAINT constraint_name ]
// { NOT NULL | NULL | CHECK (expression) }
fn create_domain_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(CREATE_KW) && p.nth_at(1, DOMAIN_KW));
    let m = p.start();
    p.bump(CREATE_KW);
    p.bump(DOMAIN_KW);
    name(p);
    p.eat(AS_KW);
    type_name(p);
    opt_collate(p);
    while !p.at(EOF) {
        // TODO: add validation to limit the types of constraints allowed
        if opt_column_constraint(p).is_none() {
            break;
        }
    }
    m.complete(p, CREATE_DOMAIN_STMT)
}

// filter_variable IN (filter_value [, ... ])
fn event_trigger_when(p: &mut Parser<'_>) {
    name_ref(p);
    p.expect(IN_KW);
    p.expect(L_PAREN);
    string_literal(p);
    while !p.at(EOF) && p.eat(COMMA) {
        string_literal(p);
    }
    p.expect(R_PAREN);
}

// CREATE EVENT TRIGGER name
//     ON event
//     [ WHEN filter_variable IN (filter_value [, ... ]) [ AND ... ] ]
//     EXECUTE { FUNCTION | PROCEDURE } function_name()
fn create_event_trigger_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(CREATE_KW) && p.nth_at(1, EVENT_KW));
    let m = p.start();
    p.bump(CREATE_KW);
    p.bump(EVENT_KW);
    p.expect(TRIGGER_KW);
    name(p);
    p.expect(ON_KW);
    p.expect(IDENT);
    if p.eat(WHEN_KW) {
        event_trigger_when(p);
        while !p.at(EOF) && p.eat(AND_KW) {
            event_trigger_when(p);
        }
    }
    p.expect(EXECUTE_KW);
    if !p.eat(FUNCTION_KW) && !p.eat(PROCEDURE_KW) {
        p.error("expected FUNCTION or PROCEDURE");
    }
    path_name_ref(p);
    p.expect(L_PAREN);
    p.expect(R_PAREN);
    m.complete(p, CREATE_EVENT_TRIGGER_STMT)
}

// CREATE FOREIGN TABLE [ IF NOT EXISTS ] table_name ( [
//   { column_name data_type [ OPTIONS ( option 'value' [, ... ] ) ] [ COLLATE collation ] [ column_constraint [ ... ] ]
//     | table_constraint }
//     [, ... ]
// ] )
// [ INHERITS ( parent_table [, ... ] ) ]
//   SERVER server_name
// [ OPTIONS ( option 'value' [, ... ] ) ]
//
// CREATE FOREIGN TABLE [ IF NOT EXISTS ] table_name
//   PARTITION OF parent_table [ (
//   { column_name [ WITH OPTIONS ] [ column_constraint [ ... ] ]
//     | table_constraint }
//     [, ... ]
// ) ]
// { FOR VALUES partition_bound_spec | DEFAULT }
//   SERVER server_name
// [ OPTIONS ( option 'value' [, ... ] ) ]
//
// where column_constraint is:
//   [ CONSTRAINT constraint_name ]
//   { NOT NULL |
//     NULL |
//     CHECK ( expression ) [ NO INHERIT ] |
//     DEFAULT default_expr |
//     GENERATED ALWAYS AS ( generation_expr ) STORED }
//
// and table_constraint is:
//   [ CONSTRAINT constraint_name ]
//   CHECK ( expression ) [ NO INHERIT ]
//
// and partition_bound_spec is:
//   IN ( partition_bound_expr [, ...] ) |
//   FROM ( { partition_bound_expr | MINVALUE | MAXVALUE } [, ...] )
//     TO ( { partition_bound_expr | MINVALUE | MAXVALUE } [, ...] ) |
//   WITH ( MODULUS numeric_literal, REMAINDER numeric_literal )
fn create_foreign_table_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(CREATE_KW) && p.nth_at(1, FOREIGN_KW));
    let m = p.start();
    p.bump(CREATE_KW);
    p.bump(FOREIGN_KW);
    p.expect(TABLE_KW);
    opt_if_not_exists(p);
    path_name(p);
    if p.eat(PARTITION_KW) {
        p.expect(OF_KW);
        path_name_ref(p);
        if p.eat(L_PAREN) {
            if p.at_ts(TABLE_CONSTRAINT_FIRST) {
                table_constraint(p);
            } else {
                name_ref(p);
                if p.eat(WITH_KW) {
                    p.expect(OPTIONS_KW);
                }
                while !p.at(EOF) && opt_column_constraint(p).is_some() {
                    // pass
                }
            }
            while !p.at(EOF) && p.eat(COMMA) {
                if p.at_ts(TABLE_CONSTRAINT_FIRST) {
                    table_constraint(p);
                } else {
                    name_ref(p);
                    if p.eat(WITH_KW) {
                        p.expect(OPTIONS_KW);
                    }
                    while !p.at(EOF) && opt_column_constraint(p).is_some() {
                        // pass
                    }
                }
            }
            p.expect(R_PAREN);
        }
        partition_option(p);
    } else {
        p.expect(L_PAREN);
        while !p.at(EOF) && !p.at(R_PAREN) {
            if p.at_ts(TABLE_CONSTRAINT_FIRST) {
                table_constraint(p);
            } else {
                name(p);
                type_name(p);
                opt_options_list(p);
                opt_collate(p);
                while !p.at(EOF) && opt_column_constraint(p).is_some() {}
            }
            if !p.eat(COMMA) {
                break;
            }
        }
        p.expect(R_PAREN);
        opt_inherits_tables(p);
    }
    p.expect(SERVER_KW);
    name_ref(p);
    opt_options_list(p);
    m.complete(p, CREATE_FOREIGN_TABLE_STMT)
}

// CREATE FOREIGN DATA WRAPPER name
//     [ HANDLER handler_function | NO HANDLER ]
//     [ VALIDATOR validator_function | NO VALIDATOR ]
//     [ OPTIONS ( option 'value' [, ... ] ) ]
fn create_foreign_data_wrapper_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(CREATE_KW) && p.nth_at(1, FOREIGN_KW) && p.nth_at(2, DATA_KW));
    let m = p.start();
    p.bump(CREATE_KW);
    p.bump(FOREIGN_KW);
    p.bump(DATA_KW);
    p.expect(WRAPPER_KW);
    name(p);
    while !p.at(EOF) {
        if !opt_fdw_option(p) {
            break;
        }
    }
    opt_options_list(p);
    m.complete(p, CREATE_FOREIGN_DATA_WRAPPER_STMT)
}

fn opt_fdw_option(p: &mut Parser<'_>) -> bool {
    match p.current() {
        OPTIONS_KW => {
            p.bump(OPTIONS_KW);
            p.expect(L_PAREN);
            alter_option(p);
            while !p.at(EOF) && p.eat(COMMA) {
                alter_option(p);
            }
            p.expect(R_PAREN);
            true
        }
        HANDLER_KW | VALIDATOR_KW => {
            p.bump_any();
            path_name_ref(p);
            true
        }
        NO_KW => {
            p.bump(NO_KW);
            if !p.eat(HANDLER_KW) && !p.eat(VALIDATOR_KW) {
                p.error("expected HANDLER or VALIDATOR")
            }
            true
        }
        _ => false,
    }
}

// CREATE GROUP name [ [ WITH ] option [ ... ] ]
// where option can be:
//       SUPERUSER | NOSUPERUSER
//     | CREATEDB | NOCREATEDB
//     | CREATEROLE | NOCREATEROLE
//     | INHERIT | NOINHERIT
//     | LOGIN | NOLOGIN
//     | REPLICATION | NOREPLICATION
//     | BYPASSRLS | NOBYPASSRLS
//     | CONNECTION LIMIT connlimit
//     | [ ENCRYPTED ] PASSWORD 'password' | PASSWORD NULL
//     | VALID UNTIL 'timestamp'
//     | IN ROLE role_name [, ...]
//     | IN GROUP role_name [, ...]
//     | ROLE role_name [, ...]
//     | ADMIN role_name [, ...]
//     | USER role_name [, ...]
//     | SYSID uid
fn create_group_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(CREATE_KW) && p.nth_at(1, GROUP_KW));
    let m = p.start();
    p.bump(CREATE_KW);
    p.bump(GROUP_KW);
    name(p);
    opt_role_option_list(p);
    m.complete(p, CREATE_GROUP_STMT)
}

// CREATE [ OR REPLACE ] [ TRUSTED ] [ PROCEDURAL ] LANGUAGE name
//   [ HANDLER call_handler [ INLINE inline_handler ] [ VALIDATOR valfunction ] ]
fn create_language_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(CREATE_KW));
    let m = p.start();
    p.bump(CREATE_KW);
    opt_or_replace(p);
    p.eat(TRUSTED_KW);
    p.eat(PROCEDURAL_KW);
    p.eat(LANGUAGE_KW);
    name(p);
    if p.eat(HANDLER_KW) {
        path_name_ref(p);
        if p.eat(INLINE_KW) {
            path_name_ref(p);
        }
        if p.eat(VALIDATOR_KW) {
            path_name_ref(p);
        }
    }
    m.complete(p, CREATE_LANGUAGE_STMT)
}

// CREATE MATERIALIZED VIEW [ IF NOT EXISTS ] table_name
//     [ ( column_name [, ...] ) ]
//     [ USING method ]
//     [ WITH ( storage_parameter [= value] [, ... ] ) ]
//     [ TABLESPACE tablespace_name ]
//     AS query
//     [ WITH [ NO ] DATA ]
fn create_materialized_view_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(CREATE_KW) && p.nth_at(1, MATERIALIZED_KW));
    let m = p.start();
    p.bump(CREATE_KW);
    p.bump(MATERIALIZED_KW);
    p.expect(VIEW_KW);
    opt_if_not_exists(p);
    path_name(p);
    opt_column_list_with(p, ColumnDefKind::Name);
    if p.eat(USING_KW) {
        name_ref(p);
    }
    opt_with_params(p);
    if p.eat(TABLESPACE_KW) {
        name_ref(p);
    }
    p.expect(AS_KW);
    // A SELECT, TABLE, or VALUES command.
    let statement = stmt(
        p,
        &StmtRestrictions {
            begin_end_allowed: false,
        },
    );
    match statement.map(|x| x.kind()) {
        Some(SELECT) => (),
        Some(kind) => {
            p.error(format!(
                "expected SELECT, TABLE, or VALUES statement, got {:?}",
                kind
            ));
        }
        None => {
            p.error("expected SELECT, TABLE, or VALUES statement");
        }
    }
    if p.eat(WITH_KW) {
        p.eat(NO_KW);
        p.expect(DATA_KW);
    }
    m.complete(p, CREATE_MATERIALIZED_VIEW_STMT)
}

// CREATE OPERATOR name (
//   {FUNCTION|PROCEDURE} = function_name
//   [, LEFTARG = left_type ] [, RIGHTARG = right_type ]
//   [, COMMUTATOR = com_op ] [, NEGATOR = neg_op ]
//   [, RESTRICT = res_proc ] [, JOIN = join_proc ]
//   [, HASHES ] [, MERGES ]
// )
fn create_operator_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(CREATE_KW) && p.nth_at(1, OPERATOR_KW));
    let m = p.start();
    p.bump(CREATE_KW);
    p.bump(OPERATOR_KW);
    operator(p);
    p.expect(L_PAREN);
    while !p.at(EOF) {
        if !attribute_option(p, AttributeValue::Either) {
            break;
        }
        if !p.eat(COMMA) {
            break;
        }
    }
    p.expect(R_PAREN);
    m.complete(p, CREATE_OPERATOR_STMT)
}

// CREATE OPERATOR CLASS name [ DEFAULT ] FOR TYPE data_type
//   USING index_method [ FAMILY family_name ] AS
//   {  OPERATOR strategy_number operator_name [ ( op_type, op_type ) ] [ FOR SEARCH | FOR ORDER BY sort_family_name ]
//    | FUNCTION support_number [ ( op_type [ , op_type ] ) ] function_name ( argument_type [, ...] )
//    | STORAGE storage_type
//   } [, ... ]
fn create_operator_class_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(CREATE_KW) && p.nth_at(1, OPERATOR_KW));
    let m = p.start();
    p.bump(CREATE_KW);
    p.bump(OPERATOR_KW);
    p.expect(CLASS_KW);
    path_name(p);
    p.eat(DEFAULT_KW);
    p.expect(FOR_KW);
    p.expect(TYPE_KW);
    type_name(p);
    p.expect(USING_KW);
    name_ref(p);
    if p.eat(FAMILY_KW) {
        path_name_ref(p);
    }
    p.expect(AS_KW);
    operator_class_option(p);
    while !p.at(EOF) && p.eat(COMMA) {
        operator_class_option(p);
    }
    m.complete(p, CREATE_OPERATOR_CLASS_STMT)
}

// | OPERATOR strategy_number operator_name [ ( op_type, op_type ) ] [ FOR SEARCH | FOR ORDER BY sort_family_name ]
// | FUNCTION support_number [ ( op_type [ , op_type ] ) ] function_name ( argument_type [, ...] )
// | STORAGE storage_type
fn operator_class_option(p: &mut Parser<'_>) {
    match p.current() {
        OPERATOR_KW => {
            p.bump(OPERATOR_KW);
            if opt_numeric_literal(p).is_none() {
                p.error("expected number");
            }
            operator(p);
            if p.eat(L_PAREN) {
                type_name(p);
                p.expect(COMMA);
                type_name(p);
                p.expect(R_PAREN);
            }
            if p.eat(FOR_KW) {
                if p.eat(ORDER_KW) {
                    p.expect(BY_KW);
                    path_name_ref(p);
                } else if p.eat(SEARCH_KW) {
                    // pass
                } else {
                    p.error("expected SEARCH or ORDER BY");
                }
            }
        }
        FUNCTION_KW => {
            p.bump(FUNCTION_KW);
            if opt_numeric_literal(p).is_none() {
                p.error("expected number");
            }
            if p.eat(L_PAREN) {
                type_name(p);
                if p.eat(COMMA) {
                    type_name(p);
                }
                p.expect(R_PAREN);
            }
            path_name_ref(p);
            opt_param_list(p);
        }
        STORAGE_KW => {
            p.bump(STORAGE_KW);
            type_name(p);
        }
        _ => p.error("expected OPERATOR, FUNCTION, or STORAGE"),
    }
}

// | OPERATOR strategy_number ( op_type [ , op_type ] )
// | FUNCTION support_number ( op_type [ , op_type ] )
fn operator_drop_class_option(p: &mut Parser<'_>) {
    match p.current() {
        OPERATOR_KW | FUNCTION_KW => {
            p.bump_any();
            if opt_numeric_literal(p).is_none() {
                p.error("expected number");
            }
            if p.eat(L_PAREN) {
                type_name(p);
                if p.eat(COMMA) {
                    type_name(p);
                }
                p.expect(R_PAREN);
            }
        }
        _ => p.error("expected OPERATOR, or FUNCTION"),
    }
}

// CREATE OPERATOR FAMILY name USING index_method
fn create_operator_family_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(CREATE_KW) && p.nth_at(1, OPERATOR_KW));
    let m = p.start();
    p.bump(CREATE_KW);
    p.bump(OPERATOR_KW);
    p.expect(FAMILY_KW);
    path_name(p);
    p.expect(USING_KW);
    name_ref(p);
    m.complete(p, CREATE_OPERATOR_FAMILY_STMT)
}

// CREATE POLICY name ON table_name
//     [ AS { PERMISSIVE | RESTRICTIVE } ]
//     [ FOR { ALL | SELECT | INSERT | UPDATE | DELETE } ]
//     [ TO { role_name | PUBLIC | CURRENT_ROLE | CURRENT_USER | SESSION_USER } [, ...] ]
//     [ USING ( using_expression ) ]
//     [ WITH CHECK ( check_expression ) ]
fn create_policy_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(CREATE_KW) && p.nth_at(1, POLICY_KW));
    let m = p.start();
    p.bump(CREATE_KW);
    p.bump(POLICY_KW);
    name(p);
    p.expect(ON_KW);
    path_name_ref(p);
    if p.eat(AS_KW) {
        p.expect(IDENT);
    }
    if p.eat(FOR_KW) {
        let _ = p.eat(ALL_KW)
            || p.eat(SELECT_KW)
            || p.eat(INSERT_KW)
            || p.eat(UPDATE_KW)
            || p.eat(DELETE_KW);
    }
    if p.eat(TO_KW) {
        role(p);
        while !p.at(EOF) && p.eat(COMMA) {
            role(p);
        }
    }
    if p.eat(USING_KW) {
        p.expect(L_PAREN);
        if expr(p).is_none() {
            p.error("expected expression");
        }
        p.expect(R_PAREN);
    }
    if p.eat(WITH_KW) {
        p.expect(CHECK_KW);
        p.expect(L_PAREN);
        if expr(p).is_none() {
            p.error("expected expression");
        }
        p.expect(R_PAREN);
    }
    m.complete(p, CREATE_POLICY_STMT)
}

// CREATE [ OR REPLACE ] PROCEDURE
//     name ( [ [ argmode ] [ argname ] argtype [ { DEFAULT | = } default_expr ] [, ...] ] )
//   { LANGUAGE lang_name
//     | TRANSFORM { FOR TYPE type_name } [, ... ]
//     | [ EXTERNAL ] SECURITY INVOKER | [ EXTERNAL ] SECURITY DEFINER
//     | SET configuration_parameter { TO value | = value | FROM CURRENT }
//     | AS 'definition'
//     | AS 'obj_file', 'link_symbol'
//     | sql_body
//   } ...
fn create_procedure_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(CREATE_KW));
    let m = p.start();
    p.bump(CREATE_KW);
    opt_or_replace(p);
    p.expect(PROCEDURE_KW);
    path_name(p);
    param_list(p);
    func_option_list(p);
    m.complete(p, CREATE_PROCEDURE_STMT)
}

// [ TABLE ] [ ONLY ] table_name [ * ] [ ( column_name [, ... ] ) ] [ WHERE ( expression ) ] [, ... ]
// TABLES IN SCHEMA { schema_name | CURRENT_SCHEMA } [ WHERE ( expression ) ]
// CURRENT_SCHEMA
fn publication_object(p: &mut Parser<'_>) {
    if p.eat(TABLES_KW) {
        p.expect(IN_KW);
        p.expect(SCHEMA_KW);
        if !p.eat(CURRENT_SCHEMA_KW) {
            name_ref(p);
        }
        while !p.at(EOF) && p.eat(COMMA) {
            if !p.eat(CURRENT_SCHEMA_KW) {
                name_ref(p);
            }
        }
    } else if p.eat(CURRENT_SCHEMA_KW) {
        return;
    } else {
        p.eat(TABLE_KW);
        p.eat(ONLY_KW);
        if p.eat(L_PAREN) {
            path_name_ref(p);
            p.expect(R_PAREN);
        } else {
            path_name_ref(p);
        }
        p.eat(STAR);
        opt_column_list(p);
        if p.eat(WHERE_KW) {
            p.expect(L_PAREN);
            if expr(p).is_none() {
                p.error("expected expression");
            }
            p.expect(R_PAREN);
        }
    }
}

// CREATE PUBLICATION name
//     [ FOR ALL TABLES
//       | FOR publication_object [, ... ] ]
//     [ WITH ( publication_parameter [= value] [, ... ] ) ]
//
// where publication_object is one of:
//     TABLE [ ONLY ] table_name [ * ] [ ( column_name [, ... ] ) ] [ WHERE ( expression ) ] [, ... ]
//     TABLES IN SCHEMA { schema_name | CURRENT_SCHEMA } [, ... ]
fn create_publication_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(CREATE_KW) && p.nth_at(1, PUBLICATION_KW));
    let m = p.start();
    p.bump(CREATE_KW);
    p.bump(PUBLICATION_KW);
    name(p);
    if p.eat(FOR_KW) {
        if p.eat(ALL_KW) {
            p.expect(TABLES_KW);
        } else {
            publication_object(p);
            while !p.at(EOF) && p.eat(COMMA) {
                publication_object(p);
            }
        }
    }
    opt_with_params(p);
    m.complete(p, CREATE_PUBLICATION_STMT)
}

// CREATE ROLE name [ [ WITH ] option [ ... ] ]
// where option can be:
//       SUPERUSER | NOSUPERUSER
//     | CREATEDB | NOCREATEDB
//     | CREATEROLE | NOCREATEROLE
//     | INHERIT | NOINHERIT
//     | LOGIN | NOLOGIN
//     | REPLICATION | NOREPLICATION
//     | BYPASSRLS | NOBYPASSRLS
//     | CONNECTION LIMIT connlimit
//     | [ ENCRYPTED ] PASSWORD 'password' | PASSWORD NULL
//     | VALID UNTIL 'timestamp'
//     | IN ROLE role_name [, ...]
//     | ROLE role_name [, ...]
//     | ADMIN role_name [, ...]
//     | SYSID uid
fn create_role_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(CREATE_KW) && p.nth_at(1, ROLE_KW));
    let m = p.start();
    p.bump(CREATE_KW);
    p.bump(ROLE_KW);
    name(p);
    opt_role_option_list(p);
    m.complete(p, CREATE_ROLE_STMT)
}

fn select_insert_delete_update_or_notify(p: &mut Parser<'_>) {
    // statement
    // Any SELECT, INSERT, UPDATE, DELETE, MERGE, or VALUES statement.
    let statement = stmt(
        p,
        &StmtRestrictions {
            begin_end_allowed: false,
        },
    );
    if let Some(statement) = statement {
        match statement.kind() {
            SELECT | INSERT_STMT | UPDATE_STMT | DELETE_STMT | NOTIFY_STMT => (),
            kind => {
                p.error(format!(
                    "expected SELECT, INSERT, UPDATE, DELETE, NOTIFY, or VALUES statement, got {:?}",
                    kind
                ));
            }
        }
    } else {
        p.error("expected SELECT, INSERT, UPDATE, DELETE, NOTIFY, or VALUES statement");
    }
}

// CREATE [ OR REPLACE ] RULE name AS ON event
//     TO table_name [ WHERE condition ]
//     DO [ ALSO | INSTEAD ] { NOTHING | command | ( command ; command ... ) }
// where event can be one of:
//     SELECT | INSERT | UPDATE | DELETE
fn create_rule_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(CREATE_KW) && matches!(p.nth(1), OR_KW | RULE_KW));
    let m = p.start();
    p.bump(CREATE_KW);
    opt_or_replace(p);
    p.expect(RULE_KW);
    name(p);
    p.expect(AS_KW);
    p.expect(ON_KW);
    if p.at(SELECT_KW) || p.at(INSERT_KW) || p.at(UPDATE_KW) || p.at(DELETE_KW) {
        p.bump_any();
    } else {
        p.error("expected SELECT, INSERT, UPDATE, or DELETE");
    }
    p.expect(TO_KW);
    path_name_ref(p);
    opt_where_clause(p);
    p.expect(DO_KW);
    let _ = p.eat(ALSO_KW) || p.eat(INSTEAD_KW);
    if p.eat(L_PAREN) {
        // ( command ; command ... )
        while !p.at(EOF) && !p.at(R_PAREN) {
            select_insert_delete_update_or_notify(p);
            if !p.eat(SEMICOLON) {
                break;
            }
        }
        p.expect(R_PAREN);
    } else if p.eat(NOTHING_KW) {
        // pass
    } else {
        select_insert_delete_update_or_notify(p);
    }
    m.complete(p, CREATE_RULE_STMT)
}

// CREATE [ { TEMPORARY | TEMP } | UNLOGGED ] SEQUENCE [ IF NOT EXISTS ] name
//     [ AS data_type ]
//     [ INCREMENT [ BY ] increment ]
//     [ MINVALUE minvalue | NO MINVALUE ] [ MAXVALUE maxvalue | NO MAXVALUE ]
//     [ START [ WITH ] start ] [ CACHE cache ] [ [ NO ] CYCLE ]
//     [ OWNED BY { table_name.column_name | NONE } ]
fn create_sequence_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(
        p.at(CREATE_KW) && matches!(p.nth(1), TEMPORARY_KW | TEMP_KW | UNLOGGED_KW | SEQUENCE_KW)
    );
    let m = p.start();
    p.bump(CREATE_KW);
    let _ = opt_temp(p) || p.eat(UNLOGGED_KW);
    p.expect(SEQUENCE_KW);
    opt_if_not_exists(p);
    path_name(p);
    while !p.at(EOF) {
        if !opt_sequence_option(p) {
            break;
        }
    }
    m.complete(p, CREATE_SEQUENCE_STMT)
}

// CREATE SERVER [ IF NOT EXISTS ] server_name [ TYPE 'server_type' ] [ VERSION 'server_version' ]
//     FOREIGN DATA WRAPPER fdw_name
//     [ OPTIONS ( option 'value' [, ... ] ) ]
fn create_server_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(CREATE_KW) && p.nth_at(1, SERVER_KW));
    let m = p.start();
    p.bump(CREATE_KW);
    p.bump(SERVER_KW);
    opt_if_not_exists(p);
    name(p);
    if p.eat(TYPE_KW) {
        string_literal(p);
    }
    if p.eat(VERSION_KW) {
        string_literal(p);
    }
    p.expect(FOREIGN_KW);
    p.expect(DATA_KW);
    p.expect(WRAPPER_KW);
    name_ref(p);
    opt_options_list(p);
    m.complete(p, CREATE_SERVER_STMT)
}

// CREATE STATISTICS [ [ IF NOT EXISTS ] statistics_name ]
//     ON ( expression )
//     FROM table_name
//
// CREATE STATISTICS [ [ IF NOT EXISTS ] statistics_name ]
//     [ ( statistics_kind [, ... ] ) ]
//     ON { column_name | ( expression ) }, { column_name | ( expression ) } [, ...]
//     FROM table_name
fn create_statistics_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(CREATE_KW) && p.nth_at(1, STATISTICS_KW));
    let m = p.start();
    p.bump(CREATE_KW);
    p.bump(STATISTICS_KW);
    opt_if_not_exists(p);
    if !p.at(L_PAREN) && !p.at(ON_KW) {
        path_name(p);
    }
    if p.eat(L_PAREN) {
        name_ref(p);
        while !p.at(EOF) && p.eat(COMMA) {
            name_ref(p);
        }
        p.expect(R_PAREN);
    }
    if p.eat(ON_KW) {
        if expr(p).is_none() {
            p.error("expected expression");
        }
        while !p.at(EOF) && p.eat(COMMA) {
            if expr(p).is_none() {
                p.error("expected expression");
            }
        }
    }
    p.expect(FROM_KW);
    path_name_ref(p);
    m.complete(p, CREATE_STATISTICS_STMT)
}

// CREATE SUBSCRIPTION subscription_name
//     CONNECTION 'conninfo'
//     PUBLICATION publication_name [, ...]
//     [ WITH ( subscription_parameter [= value] [, ... ] ) ]
fn create_subscription_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(CREATE_KW) && p.nth_at(1, SUBSCRIPTION_KW));
    let m = p.start();
    p.bump(CREATE_KW);
    p.bump(SUBSCRIPTION_KW);
    name(p);
    p.expect(CONNECTION_KW);
    string_literal(p);
    p.expect(PUBLICATION_KW);
    name_ref(p);
    while !p.at(EOF) && p.eat(COMMA) {
        name_ref(p);
    }
    opt_with_params(p);
    m.complete(p, CREATE_SUBSCRIPTION_STMT)
}

// CREATE TABLESPACE tablespace_name
//     [ OWNER { new_owner | CURRENT_ROLE | CURRENT_USER | SESSION_USER } ]
//     LOCATION 'directory'
//     [ WITH ( tablespace_option = value [, ... ] ) ]
fn create_tablespace_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(CREATE_KW) && p.nth_at(1, TABLESPACE_KW));
    let m = p.start();
    p.bump(CREATE_KW);
    p.bump(TABLESPACE_KW);
    name(p);
    if p.eat(OWNER_KW) {
        role(p);
    }
    p.expect(LOCATION_KW);
    string_literal(p);
    // TODO: we could have a validator to check these params
    opt_with_params(p);
    m.complete(p, CREATE_TABLESPACE_STMT)
}

// CREATE TEXT SEARCH PARSER name (
//     START = start_function ,
//     GETTOKEN = gettoken_function ,
//     END = end_function ,
//     LEXTYPES = lextypes_function
//     [, HEADLINE = headline_function ]
// )
fn create_text_search_parser_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(CREATE_KW) && p.nth_at(1, TEXT_KW));
    let m = p.start();
    p.bump(CREATE_KW);
    p.bump(TEXT_KW);
    p.bump(SEARCH_KW);
    p.bump(PARSER_KW);
    path_name(p);
    p.expect(L_PAREN);
    while !p.at(EOF) {
        if !attribute_option(p, AttributeValue::Required) {
            break;
        }
        if !p.eat(COMMA) {
            break;
        }
    }
    p.expect(R_PAREN);
    m.complete(p, CREATE_TEXT_SEARCH_PARSER_STMT)
}

// CREATE TEXT SEARCH DICTIONARY name (
//     TEMPLATE = template
//     [, option = value [, ... ]]
// )
fn create_text_search_dict_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(CREATE_KW) && p.nth_at(1, TEXT_KW));
    let m = p.start();
    p.bump(CREATE_KW);
    p.bump(TEXT_KW);
    p.bump(SEARCH_KW);
    p.bump(DICTIONARY_KW);
    path_name(p);
    p.expect(L_PAREN);
    while !p.at(EOF) {
        if !attribute_option(p, AttributeValue::Required) {
            break;
        }
        if !p.eat(COMMA) {
            break;
        }
    }
    p.expect(R_PAREN);
    m.complete(p, CREATE_TEXT_SEARCH_PARSER_STMT)
}

// CREATE TEXT SEARCH CONFIGURATION name (
//     PARSER = parser_name |
//     COPY = source_config
// )
fn create_text_search_config_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(CREATE_KW) && p.nth_at(1, TEXT_KW));
    let m = p.start();
    p.bump(CREATE_KW);
    p.bump(TEXT_KW);
    p.bump(SEARCH_KW);
    p.bump(CONFIGURATION_KW);
    path_name(p);
    p.expect(L_PAREN);
    while !p.at(EOF) {
        if !attribute_option(p, AttributeValue::Required) {
            break;
        }
        if !p.eat(COMMA) {
            break;
        }
    }
    p.expect(R_PAREN);
    m.complete(p, CREATE_TEXT_SEARCH_PARSER_STMT)
}

// CREATE TEXT SEARCH TEMPLATE name (
//     [ INIT = init_function , ]
//     LEXIZE = lexize_function
// )
fn create_text_search_template_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(CREATE_KW) && p.nth_at(1, TEXT_KW));
    let m = p.start();
    p.bump(CREATE_KW);
    p.bump(TEXT_KW);
    p.bump(SEARCH_KW);
    p.bump(TEMPLATE_KW);
    path_name(p);
    p.expect(L_PAREN);
    // definition in postgres grammar
    while !p.at(EOF) {
        if !attribute_option(p, AttributeValue::Required) {
            break;
        }
        if !p.eat(COMMA) {
            break;
        }
    }
    p.expect(R_PAREN);
    m.complete(p, CREATE_TEXT_SEARCH_PARSER_STMT)
}

// CREATE [ OR REPLACE ] TRANSFORM FOR type_name LANGUAGE lang_name (
//     FROM SQL WITH FUNCTION from_sql_function_name [ (argument_type [, ...]) ],
//     TO SQL WITH FUNCTION to_sql_function_name [ (argument_type [, ...]) ]
// );
fn create_transform_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(CREATE_KW));
    let m = p.start();
    p.bump(CREATE_KW);
    opt_or_replace(p);
    p.expect(TRANSFORM_KW);
    p.expect(FOR_KW);
    type_name(p);
    p.expect(LANGUAGE_KW);
    name_ref(p);
    p.expect(L_PAREN);
    p.expect(FROM_KW);
    p.expect(SQL_KW);
    p.expect(WITH_KW);
    p.expect(FUNCTION_KW);
    path_name_ref(p);
    opt_param_list(p);
    p.expect(COMMA);
    p.expect(TO_KW);
    p.expect(SQL_KW);
    p.expect(WITH_KW);
    p.expect(FUNCTION_KW);
    path_name_ref(p);
    opt_param_list(p);
    p.expect(R_PAREN);
    m.complete(p, CREATE_TRANSFORM_STMT)
}

// CREATE USER MAPPING [ IF NOT EXISTS ] FOR { user_name | USER | CURRENT_ROLE | CURRENT_USER | PUBLIC }
//     SERVER server_name
//     [ OPTIONS ( option 'value' [, ... ] ) ]
fn create_user_mapping_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(CREATE_KW) && p.nth_at(1, USER_KW) && p.nth_at(2, MAPPING_KW));
    let m = p.start();
    p.bump(CREATE_KW);
    p.bump(USER_KW);
    p.bump(MAPPING_KW);
    opt_if_not_exists(p);
    p.expect(FOR_KW);
    // role | USER
    if !p.eat(USER_KW) {
        role(p);
    }
    p.eat(SERVER_KW);
    // server_name
    name_ref(p);
    opt_options_list(p);
    m.complete(p, CREATE_USER_MAPPING_STMT)
}

const ROLE_OPTION_FIRST: TokenSet = TokenSet::new(&[
    WITH_KW,
    INHERIT_KW,
    CONNECTION_KW,
    ENCRYPTED_KW,
    PASSWORD_KW,
    VALID_KW,
    IN_KW,
    ROLE_KW,
    ADMIN_KW,
    USER_KW,
    SYSID_KW,
    IDENT,
]);

fn opt_role_option(p: &mut Parser<'_>) -> bool {
    if !p.at_ts(ROLE_OPTION_FIRST) {
        return false;
    }
    match p.current() {
        // SUPERUSER
        // NOSUPERUSER
        // CREATEDB
        // NOCREATEDB
        // CREATEROLE
        // NOCREATEROLE
        // NOINHERIT
        // LOGIN
        // NOLOGIN
        // REPLICATION
        // NOREPLICATION
        // BYPASSRLS
        // NOBYPASSRLS
        INHERIT_KW | IDENT => {
            p.bump_any();
        }
        CONNECTION_KW => {
            p.bump(CONNECTION_KW);
            p.expect(LIMIT_KW);
            if opt_numeric_literal(p).is_none() {
                p.error("expected number literal");
            }
        }
        ENCRYPTED_KW => {
            p.bump(ENCRYPTED_KW);
            p.expect(PASSWORD_KW);
            string_literal(p);
        }
        PASSWORD_KW => {
            p.bump(PASSWORD_KW);
            if !p.eat(NULL_KW) {
                string_literal(p);
            }
        }
        VALID_KW => {
            p.bump(VALID_KW);
            p.expect(UNTIL_KW);
            string_literal(p);
        }
        IN_KW => {
            p.bump(IN_KW);
            if p.at(GROUP_KW) || p.at(ROLE_KW) {
                p.bump_any();
            } else {
                p.error("expected GROUP or ROLE");
            }
            role(p);
            while !p.at(EOF) && p.eat(COMMA) {
                role(p);
            }
        }
        ROLE_KW | ADMIN_KW | USER_KW => {
            p.bump_any();
            role(p);
            while !p.at(EOF) && p.eat(COMMA) {
                role(p);
            }
        }
        SYSID_KW => {
            p.bump(SYSID_KW);
            if opt_numeric_literal(p).is_none() {
                p.error("expected string literal");
            }
        }
        _ => {
            p.err_and_bump("expected role option");
            return false;
        }
    }
    true
}

// CREATE USER name [ [ WITH ] option [ ... ] ]
// where option can be:
//     SUPERUSER | NOSUPERUSER
//   | CREATEDB | NOCREATEDB
//   | CREATEROLE | NOCREATEROLE
//   | INHERIT | NOINHERIT
//   | LOGIN | NOLOGIN
//   | REPLICATION | NOREPLICATION
//   | BYPASSRLS | NOBYPASSRLS
//   | CONNECTION LIMIT connlimit
//   | [ ENCRYPTED ] PASSWORD 'password' | PASSWORD NULL
//   | VALID UNTIL 'timestamp'
//   | IN ROLE role_name [, ...]
//   | IN GROUP role_name [, ...]
//   | ROLE role_name [, ...]
//   | ADMIN role_name [, ...]
//   | USER role_name [, ...]
//   | SYSID uid
fn create_user_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(CREATE_KW) && p.nth_at(1, USER_KW));
    let m = p.start();
    p.bump(CREATE_KW);
    p.bump(USER_KW);
    name(p);
    opt_role_option_list(p);
    m.complete(p, CREATE_USER_STMT)
}

fn opt_role_option_list(p: &mut Parser<'_>) {
    if p.at_ts(ROLE_OPTION_FIRST) {
        p.eat(WITH_KW);
        opt_role_option(p);
        while !p.at(EOF) && p.at_ts(ROLE_OPTION_FIRST) {
            opt_role_option(p);
        }
    }
}

// DROP [ PROCEDURAL ] LANGUAGE [ IF EXISTS ] name [ CASCADE| RESTRICT ]
fn drop_language_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(DROP_KW) && matches!(p.nth(1), LANGUAGE_KW | PROCEDURAL_KW));
    let m = p.start();
    p.bump(DROP_KW);
    p.eat(PROCEDURAL_KW);
    p.expect(LANGUAGE_KW);
    opt_if_exists(p);
    name_ref(p);
    opt_cascade_or_restrict(p);
    m.complete(p, DROP_LANGUAGE_STMT)
}

// DROP GROUP [ IF EXISTS ] name [, ...]
fn drop_group_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(DROP_KW) && p.nth_at(1, GROUP_KW));
    let m = p.start();
    p.bump(DROP_KW);
    p.bump(GROUP_KW);
    opt_if_exists(p);
    name_ref(p);
    while !p.at(EOF) && p.eat(COMMA) {
        name_ref(p);
    }
    m.complete(p, DROP_GROUP_STMT)
}

// DROP FUNCTION [ IF EXISTS ] name [ ( [ [ argmode ] [ argname ] argtype [, ...] ] ) ] [, ...]
//     [ CASCADE | RESTRICT ]
fn drop_function_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(DROP_KW) && p.nth_at(1, FUNCTION_KW));
    let m = p.start();
    p.bump(DROP_KW);
    p.bump(FUNCTION_KW);
    opt_if_exists(p);
    path_name_ref(p);
    opt_param_list(p);
    while !p.at(EOF) && p.eat(COMMA) {
        path_name_ref(p);
        opt_param_list(p);
    }
    opt_cascade_or_restrict(p);
    m.complete(p, DROP_FUNCTION_STMT)
}

// DROP FOREIGN DATA WRAPPER [ IF EXISTS ] name [, ...] [ CASCADE | RESTRICT ]
fn drop_foreign_data_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(
        p.at(DROP_KW) && p.nth_at(1, FOREIGN_KW) && p.nth_at(2, DATA_KW) && p.nth_at(3, WRAPPER_KW)
    );
    let m = p.start();
    p.bump(DROP_KW);
    p.bump(FOREIGN_KW);
    p.bump(DATA_KW);
    p.bump(WRAPPER_KW);
    opt_if_exists(p);
    name_ref(p);
    while !p.at(EOF) && p.eat(COMMA) {
        name_ref(p);
    }
    opt_cascade_or_restrict(p);
    m.complete(p, DROP_FOREIGN_DATA_WRAPPER_STMT)
}

// DROP FOREIGN TABLE [ IF EXISTS ] name [, ...] [ CASCADE | RESTRICT ]
fn drop_foreign_table_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(DROP_KW) && p.nth_at(1, FOREIGN_KW) && p.nth_at(2, TABLE_KW));
    let m = p.start();
    p.bump(DROP_KW);
    p.bump(FOREIGN_KW);
    p.bump(TABLE_KW);
    opt_if_exists(p);
    path_name_ref(p);
    while !p.at(EOF) && p.eat(COMMA) {
        path_name_ref(p);
    }
    opt_cascade_or_restrict(p);
    m.complete(p, DROP_FOREIGN_TABLE_STMT)
}

// DROP ACCESS METHOD [ IF EXISTS ] name [ CASCADE | RESTRICT ]
fn drop_access_method_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(DROP_KW) && p.nth_at(1, ACCESS_KW) && p.nth_at(2, METHOD_KW));
    let m = p.start();
    p.bump(DROP_KW);
    p.bump(ACCESS_KW);
    p.bump(METHOD_KW);
    opt_if_exists(p);
    name_ref(p);
    opt_cascade_or_restrict(p);
    m.complete(p, DROP_ACCESS_METHOD_STMT)
}

fn aggregate(p: &mut Parser<'_>) {
    let m = p.start();
    path_name_ref(p);
    aggregate_arg_list(p);
    m.complete(p, CALL_EXPR);
}

// DROP AGGREGATE [ IF EXISTS ] name ( aggregate_signature ) [, ...] [ CASCADE | RESTRICT ]
// where aggregate_signature is:
// * |
// [ argmode ] [ argname ] argtype [ , ... ] |
// [ [ argmode ] [ argname ] argtype [ , ... ] ] ORDER BY [ argmode ] [ argname ] argtype [ , ... ]
fn drop_aggregate_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(DROP_KW) && p.nth_at(1, AGGREGATE_KW));
    let m = p.start();
    p.bump(DROP_KW);
    p.bump(AGGREGATE_KW);
    opt_if_exists(p);
    aggregate(p);
    while !p.at(EOF) && p.eat(COMMA) {
        aggregate(p);
    }
    opt_cascade_or_restrict(p);
    m.complete(p, DROP_AGGREGATE_STMT)
}

fn source_type_as_target_type(p: &mut Parser<'_>) {
    p.expect(L_PAREN);
    type_name(p);
    p.expect(AS_KW);
    type_name(p);
    p.expect(R_PAREN);
}

// DROP CAST [ IF EXISTS ] (source_type AS target_type) [ CASCADE | RESTRICT ]
fn drop_cast_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(DROP_KW) && p.nth_at(1, CAST_KW));
    let m = p.start();
    p.bump(DROP_KW);
    p.bump(CAST_KW);
    opt_if_exists(p);
    source_type_as_target_type(p);
    opt_cascade_or_restrict(p);
    m.complete(p, DROP_CAST_STMT)
}

// DROP COLLATION [ IF EXISTS ] name [ CASCADE | RESTRICT ]
fn drop_collation_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(DROP_KW) && p.nth_at(1, COLLATION_KW));
    let m = p.start();
    p.bump(DROP_KW);
    p.bump(COLLATION_KW);
    opt_if_exists(p);
    path_name_ref(p);
    opt_cascade_or_restrict(p);
    m.complete(p, DROP_COLLATION_STMT)
}

// DROP CONVERSION [ IF EXISTS ] name [ CASCADE | RESTRICT ]
fn drop_conversion_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(DROP_KW) && p.nth_at(1, CONVERSION_KW));
    let m = p.start();
    p.bump(DROP_KW);
    p.bump(CONVERSION_KW);
    opt_if_exists(p);
    path_name_ref(p);
    opt_cascade_or_restrict(p);
    m.complete(p, DROP_CONVERSION_STMT)
}

// DROP DOMAIN [ IF EXISTS ] name [, ...] [ CASCADE | RESTRICT ]
fn drop_domain_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(DROP_KW) && p.nth_at(1, DOMAIN_KW));
    let m = p.start();
    p.bump(DROP_KW);
    p.bump(DOMAIN_KW);
    opt_if_exists(p);
    path_name_ref(p);
    while !p.at(EOF) && p.eat(COMMA) {
        path_name_ref(p);
    }
    opt_cascade_or_restrict(p);
    m.complete(p, DROP_DOMAIN_STMT)
}

// DROP EVENT TRIGGER [ IF EXISTS ] name [ CASCADE | RESTRICT ]
fn drop_event_trigger_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(DROP_KW) && p.nth_at(1, EVENT_KW) && p.nth_at(2, TRIGGER_KW));
    let m = p.start();
    p.bump(DROP_KW);
    p.bump(EVENT_KW);
    p.bump(TRIGGER_KW);
    opt_if_exists(p);
    name_ref(p);
    opt_cascade_or_restrict(p);
    m.complete(p, DROP_EVENT_TRIGGER_STMT)
}

// DROP EXTENSION [ IF EXISTS ] name [, ...] [ CASCADE | RESTRICT ]
fn drop_extension_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(DROP_KW) && p.nth_at(1, EXTENSION_KW));
    let m = p.start();
    p.bump(DROP_KW);
    p.bump(EXTENSION_KW);
    opt_if_exists(p);
    name_ref(p);
    while !p.at(EOF) && p.eat(COMMA) {
        name_ref(p);
    }
    opt_cascade_or_restrict(p);
    m.complete(p, DROP_EXTENSION_STMT)
}

// DROP MATERIALIZED VIEW [ IF EXISTS ] name [, ...] [ CASCADE | RESTRICT ]
fn drop_materialized_view_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(DROP_KW) && p.nth_at(1, MATERIALIZED_KW) && p.nth_at(2, VIEW_KW));
    let m = p.start();
    p.bump(DROP_KW);
    p.bump(MATERIALIZED_KW);
    p.bump(VIEW_KW);
    opt_if_exists(p);
    path_name_ref(p);
    while !p.at(EOF) && p.eat(COMMA) {
        path_name_ref(p);
    }
    opt_cascade_or_restrict(p);
    m.complete(p, DROP_MATERIALIZED_VIEW_STMT)
}

// DROP OPERATOR FAMILY [ IF EXISTS ] name USING index_method [ CASCADE | RESTRICT ]
fn drop_operator_family_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(DROP_KW) && p.nth_at(1, OPERATOR_KW) && p.nth_at(2, FAMILY_KW));
    let m = p.start();
    p.bump(DROP_KW);
    p.bump(OPERATOR_KW);
    p.bump(FAMILY_KW);
    opt_if_exists(p);
    path_name_ref(p);
    p.expect(USING_KW);
    name_ref(p); // index_method
    opt_cascade_or_restrict(p);
    m.complete(p, DROP_OPERATOR_FAMILY_STMT)
}

// DROP OPERATOR [ IF EXISTS ] name ( { left_type | NONE } , right_type ) [, ...] [ CASCADE | RESTRICT ]
fn drop_operator_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(DROP_KW) && p.nth_at(1, OPERATOR_KW));
    let m = p.start();
    p.bump(DROP_KW);
    p.bump(OPERATOR_KW);
    opt_if_exists(p);
    operator_sig(p);
    while !p.at(EOF) && p.eat(COMMA) {
        operator_sig(p);
    }
    opt_cascade_or_restrict(p);
    m.complete(p, DROP_OPERATOR_STMT)
}

// name ( { left_type | NONE } , right_type )
fn operator_sig(p: &mut Parser<'_>) {
    operator(p);
    p.expect(L_PAREN);
    if !p.eat(NONE_KW) {
        type_name(p);
    }
    p.expect(COMMA);
    type_name(p);
    p.expect(R_PAREN);
}

// DROP OPERATOR CLASS [ IF EXISTS ] name USING index_method [ CASCADE | RESTRICT ]
fn drop_operator_class_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(DROP_KW) && p.nth_at(1, OPERATOR_KW) && p.nth_at(2, CLASS_KW));
    let m = p.start();
    p.bump(DROP_KW);
    p.bump(OPERATOR_KW);
    p.bump(CLASS_KW);
    opt_if_exists(p);
    path_name_ref(p);
    p.expect(USING_KW);
    name_ref(p); // index_method
    opt_cascade_or_restrict(p);
    m.complete(p, DROP_OPERATOR_CLASS_STMT)
}

// DROP OWNED BY { name | CURRENT_ROLE | CURRENT_USER | SESSION_USER } [, ...] [ CASCADE | RESTRICT ]
fn drop_owned_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(DROP_KW) && p.nth_at(1, OWNED_KW));
    let m = p.start();
    p.bump(DROP_KW);
    p.bump(OWNED_KW);
    p.expect(BY_KW);
    role(p);
    while !p.at(EOF) && p.eat(COMMA) {
        role(p);
    }
    opt_cascade_or_restrict(p);
    m.complete(p, DROP_OWNED_STMT)
}

// DROP POLICY [ IF EXISTS ] name ON table_name [ CASCADE | RESTRICT ]
fn drop_policy_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(DROP_KW) && p.nth_at(1, POLICY_KW));
    let m = p.start();
    p.bump(DROP_KW);
    p.bump(POLICY_KW);
    opt_if_exists(p);
    name_ref(p);
    p.expect(ON_KW);
    path_name_ref(p);
    opt_cascade_or_restrict(p);
    m.complete(p, DROP_POLICY_STMT)
}

// DROP PROCEDURE [ IF EXISTS ] name [ ( [ [ argmode ] [ argname ] argtype [, ...] ] ) ] [, ...]
//     [ CASCADE | RESTRICT ]
fn drop_procedure_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(DROP_KW) && p.nth_at(1, PROCEDURE_KW));
    let m = p.start();
    p.bump(DROP_KW);
    p.bump(PROCEDURE_KW);
    opt_if_exists(p);
    path_name_ref(p);
    opt_param_list(p);
    while !p.at(EOF) && p.eat(COMMA) {
        path_name_ref(p);
        opt_param_list(p);
    }
    opt_cascade_or_restrict(p);
    m.complete(p, DROP_PROCEDURE_STMT)
}

// DROP PUBLICATION [ IF EXISTS ] name [, ...] [ CASCADE | RESTRICT ]
fn drop_publication_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(DROP_KW) && p.nth_at(1, PUBLICATION_KW));
    let m = p.start();
    p.bump(DROP_KW);
    p.bump(PUBLICATION_KW);
    opt_if_exists(p);
    name_ref(p);
    while !p.at(EOF) && p.eat(COMMA) {
        name_ref(p);
    }
    opt_cascade_or_restrict(p);
    m.complete(p, DROP_PUBLICATION_STMT)
}

// DROP ROLE [ IF EXISTS ] name [, ...]
fn drop_role_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(DROP_KW) && p.nth_at(1, ROLE_KW));
    let m = p.start();
    p.bump(DROP_KW);
    p.bump(ROLE_KW);
    opt_if_exists(p);
    name_ref(p);
    while !p.at(EOF) && p.eat(COMMA) {
        name_ref(p);
    }
    m.complete(p, DROP_ROLE_STMT)
}

// DROP ROUTINE [ IF EXISTS ] name [ ( [ [ argmode ] [ argname ] argtype [, ...] ] ) ] [, ...]
// [ CASCADE | RESTRICT ]
fn drop_routine_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(DROP_KW) && p.nth_at(1, ROUTINE_KW));
    let m = p.start();
    p.bump(DROP_KW);
    p.bump(ROUTINE_KW);
    opt_if_exists(p);
    path_name_ref(p);
    opt_param_list(p);
    while !p.at(EOF) && p.eat(COMMA) {
        path_name_ref(p);
        opt_param_list(p);
    }
    opt_cascade_or_restrict(p);
    m.complete(p, DROP_ROUTINE_STMT)
}

// DROP RULE [ IF EXISTS ] name ON table_name [ CASCADE | RESTRICT ]
fn drop_rule_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(DROP_KW) && p.nth_at(1, RULE_KW));
    let m = p.start();
    p.bump(DROP_KW);
    p.bump(RULE_KW);
    opt_if_exists(p);
    name_ref(p);
    p.expect(ON_KW);
    path_name_ref(p);
    opt_cascade_or_restrict(p);
    m.complete(p, DROP_RULE_STMT)
}

// DROP SEQUENCE [ IF EXISTS ] name [, ...] [ CASCADE | RESTRICT ]
fn drop_sequence_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(DROP_KW) && p.nth_at(1, SEQUENCE_KW));
    let m = p.start();
    p.bump(DROP_KW);
    p.bump(SEQUENCE_KW);
    opt_if_exists(p);
    path_name_ref(p);
    while !p.at(EOF) && p.eat(COMMA) {
        path_name_ref(p);
    }
    opt_cascade_or_restrict(p);
    m.complete(p, DROP_SEQUENCE_STMT)
}

// DROP SERVER [ IF EXISTS ] name [, ...] [ CASCADE | RESTRICT ]
fn drop_server_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(DROP_KW) && p.nth_at(1, SERVER_KW));
    let m = p.start();
    p.bump(DROP_KW);
    p.bump(SERVER_KW);
    opt_if_exists(p);
    name_ref(p);
    while !p.at(EOF) && p.eat(COMMA) {
        name_ref(p);
    }
    opt_cascade_or_restrict(p);
    m.complete(p, DROP_SERVER_STMT)
}

// DROP STATISTICS [ IF EXISTS ] name [, ...] [ CASCADE | RESTRICT ]
fn drop_statistics_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(DROP_KW) && p.nth_at(1, STATISTICS_KW));
    let m = p.start();
    p.bump(DROP_KW);
    p.bump(STATISTICS_KW);
    opt_if_exists(p);
    path_name_ref(p);
    while !p.at(EOF) && p.eat(COMMA) {
        path_name_ref(p);
    }
    opt_cascade_or_restrict(p);
    m.complete(p, DROP_STATISTICS_STMT)
}

// DROP SUBSCRIPTION [ IF EXISTS ] name [ CASCADE | RESTRICT ]
fn drop_subscription_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(DROP_KW) && p.nth_at(1, SUBSCRIPTION_KW));
    let m = p.start();
    p.bump(DROP_KW);
    p.bump(SUBSCRIPTION_KW);
    opt_if_exists(p);
    name_ref(p);
    opt_cascade_or_restrict(p);
    m.complete(p, DROP_SUBSCRIPTION_STMT)
}

// [ CASCADE | RESTRICT ]
fn opt_cascade_or_restrict(p: &mut Parser<'_>) -> bool {
    p.eat(CASCADE_KW) || p.eat(RESTRICT_KW)
}

// DROP TABLESPACE [ IF EXISTS ] name
fn drop_tablespace_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(DROP_KW) && p.nth_at(1, TABLESPACE_KW));
    let m = p.start();
    p.bump(DROP_KW);
    p.bump(TABLESPACE_KW);
    opt_if_exists(p);
    name_ref(p);
    m.complete(p, DROP_TABLESPACE_STMT)
}

// DROP TEXT SEARCH PARSER [ IF EXISTS ] name [ CASCADE | RESTRICT ]
fn drop_text_search_parser_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(
        p.at(DROP_KW) && p.nth_at(1, TEXT_KW) && p.nth_at(2, SEARCH_KW) && p.nth_at(3, PARSER_KW)
    );
    let m = p.start();
    p.bump(DROP_KW);
    p.bump(TEXT_KW);
    p.bump(SEARCH_KW);
    p.bump(PARSER_KW);
    opt_if_exists(p);
    name_ref(p);
    opt_cascade_or_restrict(p);
    m.complete(p, DROP_TEXT_SEARCH_PARSER_STMT)
}

// DROP TEXT SEARCH CONFIGURATION [ IF EXISTS ] name [ CASCADE | RESTRICT ]
fn drop_text_search_config_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(
        p.at(DROP_KW)
            && p.nth_at(1, TEXT_KW)
            && p.nth_at(2, SEARCH_KW)
            && p.nth_at(3, CONFIGURATION_KW)
    );
    let m = p.start();
    p.bump(DROP_KW);
    p.bump(TEXT_KW);
    p.bump(SEARCH_KW);
    p.bump(CONFIGURATION_KW);
    opt_if_exists(p);
    name_ref(p);
    opt_cascade_or_restrict(p);
    m.complete(p, DROP_TEXT_SEARCH_CONFIG_STMT)
}

// DROP TEXT SEARCH DICTIONARY [ IF EXISTS ] name [ CASCADE | RESTRICT ]
fn drop_text_search_dict_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(
        p.at(DROP_KW)
            && p.nth_at(1, TEXT_KW)
            && p.nth_at(2, SEARCH_KW)
            && p.nth_at(3, DICTIONARY_KW)
    );
    let m = p.start();
    p.bump(DROP_KW);
    p.bump(TEXT_KW);
    p.bump(SEARCH_KW);
    p.bump(DICTIONARY_KW);
    opt_if_exists(p);
    name_ref(p);
    opt_cascade_or_restrict(p);
    m.complete(p, DROP_TEXT_SEARCH_DICT_STMT)
}

// DROP TEXT SEARCH TEMPLATE [ IF EXISTS ] name [ CASCADE | RESTRICT ]
fn drop_text_search_template_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(
        p.at(DROP_KW) && p.nth_at(1, TEXT_KW) && p.nth_at(2, SEARCH_KW) && p.nth_at(3, TEMPLATE_KW)
    );
    let m = p.start();
    p.bump(DROP_KW);
    p.bump(TEXT_KW);
    p.bump(SEARCH_KW);
    p.bump(TEMPLATE_KW);
    opt_if_exists(p);
    name_ref(p);
    opt_cascade_or_restrict(p);
    m.complete(p, DROP_TEXT_SEARCH_TEMPLATE_STMT)
}

// DROP TRANSFORM [ IF EXISTS ] FOR type_name LANGUAGE lang_name [ CASCADE | RESTRICT ]
fn drop_transform_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(DROP_KW) && p.nth_at(1, TRANSFORM_KW));
    let m = p.start();
    p.bump(DROP_KW);
    p.bump(TRANSFORM_KW);
    opt_if_exists(p);
    p.expect(FOR_KW);
    type_name(p);
    p.expect(LANGUAGE_KW);
    name_ref(p);
    opt_cascade_or_restrict(p);
    m.complete(p, DROP_TRANSFORM_STMT)
}

// DROP USER [ IF EXISTS ] name [, ...]
fn drop_user_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(DROP_KW) && p.nth_at(1, USER_KW));
    let m = p.start();
    p.bump(DROP_KW);
    p.bump(USER_KW);
    opt_if_exists(p);
    name_ref(p);
    while !p.at(EOF) && p.eat(COMMA) {
        name_ref(p);
    }
    m.complete(p, DROP_USER_STMT)
}

// DROP USER MAPPING [ IF EXISTS ] FOR { user_name | USER | CURRENT_ROLE | CURRENT_USER | PUBLIC } SERVER server_name
fn drop_user_mapping_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(DROP_KW) && p.nth_at(1, USER_KW) && p.nth_at(2, MAPPING_KW));
    let m = p.start();
    p.bump(DROP_KW);
    p.bump(USER_KW);
    p.bump(MAPPING_KW);
    opt_if_exists(p);
    p.expect(FOR_KW);
    // role | USER
    if !p.eat(USER_KW) {
        role(p);
    }
    p.eat(SERVER_KW);
    // server_name
    name_ref(p);
    m.complete(p, DROP_USER_MAPPING_STMT)
}

// EXPLAIN [ANALYZE] [VERBOSE] query
// EXPLAIN [ ( option [, ...] ) ] statement
fn explain_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(EXPLAIN_KW));
    let m = p.start();
    p.bump(EXPLAIN_KW);
    let pre_pg_9_syntax = p.eat(ANALYZE_KW) || p.eat(VERBOSE_KW);
    if !pre_pg_9_syntax && p.eat(L_PAREN) {
        explain_option(p);
        while !p.at(EOF) && p.eat(COMMA) {
            explain_option(p);
        }
        p.expect(R_PAREN);
    }
    // statement is SELECT, INSERT, UPDATE, DELETE, MERGE, VALUES, EXECUTE, DECLARE, CREATE TABLE AS, or CREATE MATERIALIZED VIEW AS
    let statement = stmt(
        p,
        &StmtRestrictions {
            begin_end_allowed: false,
        },
    );
    if let Some(statement) = statement {
        match statement.kind() {
            SELECT
            | INSERT_STMT
            | UPDATE_STMT
            | DELETE_STMT
            | MERGE_STMT
            | EXECUTE_STMT
            | DECLARE_STMT
            | CREATE_TABLE_AS_STMT
            | CREATE_MATERIALIZED_VIEW_STMT
            // TODO: we need a validation to check inside this
            | PAREN_EXPR => (),
            kind => {
                p.error(format!(
                    "expected SELECT, INSERT, UPDATE, DELETE, MERGE, or VALUES statement, got {:?}",
                    kind
                ));
            }
        }
    } else {
        p.error("expected SELECT, INSERT, UPDATE, DELETE, MERGE, VALUES, EXECUTE, DECLARE, CREATE TABLE AS, or CREATE MATERIALIZED VIEW AS");
    }
    m.complete(p, EXPLAIN_STMT)
}

// where option can be one of:
//     ANALYZE [ boolean ]
//     VERBOSE [ boolean ]
//     COSTS [ boolean ]
//     SETTINGS [ boolean ]
//     GENERIC_PLAN [ boolean ]
//     BUFFERS [ boolean ]
//     SERIALIZE [ { NONE | TEXT | BINARY } ]
//     WAL [ boolean ]
//     TIMING [ boolean ]
//     SUMMARY [ boolean ]
//     MEMORY [ boolean ]
//     FORMAT { TEXT | XML | JSON | YAML }
fn explain_option(p: &mut Parser<'_>) {
    // TODO: we need a validation run for this since we're using IDENT
    match p.current() {
        ANALYZE_KW | VERBOSE_KW | IDENT | FORMAT_KW => {
            p.bump_any();
            //  WAL [ boolean ]
            if opt_bool_literal(p) {
                return;
            }
            // [ { NONE | TEXT | BINARY } ]
            if p.eat(NONE_KW) || p.eat(TEXT_KW) || p.eat(BINARY_KW) {
                return;
            }
            // { TEXT | XML | JSON | YAML }
            if p.eat(TEXT_KW) || p.eat(XML_KW) || p.eat(JSON_KW) || p.eat(IDENT) {
                return;
            }
        }
        _ => p.error("expected option name"),
    }
}

// TODO: I think we want something like deliminated where we give it a FIRST
// token set so we can be robust to missing commas
fn one_or_more(p: &mut Parser<'_>, mut parse: impl FnMut(&mut Parser<'_>)) {
    parse(p);
    while !p.at(EOF) && p.eat(COMMA) {
        parse(p);
    }
}

// [ OPTIONS ( option 'value' [, ... ] ) ]
fn opt_options_list(p: &mut Parser<'_>) {
    // [ OPTIONS ( option 'value' [, ... ] ) ]
    if p.eat(OPTIONS_KW) {
        p.expect(L_PAREN);
        one_or_more(p, |p| {
            col_label(p);
            string_literal(p);
        });
        p.expect(R_PAREN);
    }
}

// IMPORT FOREIGN SCHEMA remote_schema
//     [ { LIMIT TO | EXCEPT } ( table_name [, ...] ) ]
//     FROM SERVER server_name
//     INTO local_schema
//     [ OPTIONS ( option 'value' [, ... ] ) ]
fn import_foreign_schema_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(IMPORT_KW) && p.nth_at(1, FOREIGN_KW));
    let m = p.start();
    p.bump(IMPORT_KW);
    p.bump(FOREIGN_KW);
    p.expect(SCHEMA_KW);
    // remote_schema
    name_ref(p);
    // [ { LIMIT TO | EXCEPT } ( table_name [, ...] ) ]
    if p.at(LIMIT_KW) || p.at(EXCEPT_KW) {
        if p.eat(LIMIT_KW) {
            p.expect(TO_KW);
        } else {
            p.bump(EXCEPT_KW);
        }
        // ( table_name [, ...] )
        p.expect(L_PAREN);
        name_ref(p);
        while !p.at(EOF) && p.eat(COMMA) {
            name_ref(p);
        }
        p.expect(R_PAREN);
    }
    // FROM SERVER server_name
    p.expect(FROM_KW);
    p.expect(SERVER_KW);
    name_ref(p);
    // INTO local_schema
    p.expect(INTO_KW);
    name_ref(p);
    opt_options_list(p);
    m.complete(p, IMPORT_FOREIGN_SCHEMA)
}

// LOCK [ TABLE ] [ ONLY ] name [ * ] [, ...] [ IN lockmode MODE ] [ NOWAIT ]
// where lockmode is one of:
//     ACCESS SHARE | ROW SHARE | ROW EXCLUSIVE | SHARE UPDATE EXCLUSIVE
//     | SHARE | SHARE ROW EXCLUSIVE | EXCLUSIVE | ACCESS EXCLUSIVE
fn lock_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(LOCK_KW));
    let m = p.start();
    p.bump(LOCK_KW);
    // [ TABLE ]
    p.eat(TABLE_KW);
    // [ ONLY ] name [ * ] [, ...]
    relation_name(p);
    while !p.at(EOF) && p.eat(COMMA) {
        relation_name(p);
    }
    // [ IN lockmode MODE ]
    if p.eat(IN_KW) {
        match (p.current(), p.nth(1)) {
            // ACCESS SHARE | ROW SHARE
            (ACCESS_KW | ROW_KW, SHARE_KW) => {
                p.bump_any();
                p.bump(SHARE_KW);
            }
            // ACCESS EXCLUSIVE | ROW EXCLUSIVE
            (ACCESS_KW | ROW_KW, EXCLUSIVE_KW) => {
                p.bump_any();
                p.bump(EXCLUSIVE_KW);
            }
            // SHARE ROW EXCLUSIVE
            (SHARE_KW, ROW_KW) => {
                p.bump(SHARE_KW);
                p.bump(ROW_KW);
                p.expect(EXCLUSIVE_KW);
            }
            // SHARE UPDATE EXCLUSIVE
            (SHARE_KW, UPDATE_KW) => {
                p.bump(SHARE_KW);
                p.bump(UPDATE_KW);
                p.expect(EXCLUSIVE_KW);
            }
            // SHARE
            (SHARE_KW, _) => {
                p.bump(SHARE_KW);
            }
            // EXCLUSIVE
            (EXCLUSIVE_KW, _) => {
                p.bump(EXCLUSIVE_KW);
            }
            _ => p.error("expected lockmode"),
        }
        p.expect(MODE_KW);
    }
    // [ NOWAIT ]
    p.eat(NOWAIT_KW);
    m.complete(p, LOCK_STMT)
}

// [ WITH with_query [, ...] ]
// MERGE INTO [ ONLY ] target_table_name [ * ] [ [ AS ] target_alias ]
// USING data_source ON join_condition
// when_clause [...]
// [ RETURNING { * | output_expression [ [ AS ] output_name ] } [, ...] ]
//
// where data_source is:
// { [ ONLY ] source_table_name [ * ] | ( source_query ) } [ [ AS ] source_alias ]
//
// and when_clause is:
// { WHEN MATCHED [ AND condition ] THEN { merge_update | merge_delete | DO NOTHING } |
//   WHEN NOT MATCHED BY SOURCE [ AND condition ] THEN { merge_update | merge_delete | DO NOTHING } |
//   WHEN NOT MATCHED [ BY TARGET ] [ AND condition ] THEN { merge_insert | DO NOTHING } }
//
// and merge_insert is:
// INSERT [( column_name [, ...] )]
// [ OVERRIDING { SYSTEM | USER } VALUE ]
// { VALUES ( { expression | DEFAULT } [, ...] ) | DEFAULT VALUES }
//
// and merge_update is:
// UPDATE SET { column_name = { expression | DEFAULT } |
//              ( column_name [, ...] ) = [ ROW ] ( { expression | DEFAULT } [, ...] ) |
//              ( column_name [, ...] ) = ( sub-SELECT )
//            } [, ...]
//
// and merge_delete is:
// DELETE
fn merge_stmt(p: &mut Parser<'_>, m: Option<Marker>) -> CompletedMarker {
    assert!(p.at(MERGE_KW));
    let m = m.unwrap_or_else(|| p.start());
    p.bump(MERGE_KW);
    p.expect(INTO_KW);
    // [ ONLY ] target_table_name [ * ]
    relation_name(p);
    // [ [ AS ] target_alias ]
    opt_as_alias(p);
    // USING data_source ON join_condition
    merge_using_clause(p);
    merge_when_clause(p);
    while !p.at(EOF) && p.at(WHEN_KW) {
        merge_when_clause(p);
    }
    // [ RETURNING { * | output_expression [ [ AS ] output_name ] } [, ...] ]
    opt_returning_clause(p);
    m.complete(p, MERGE_STMT)
}

// where data_source is:
// { [ ONLY ] source_table_name [ * ] | ( source_query ) } [ [ AS ] source_alias ]
//
// and when_clause is:
// { WHEN MATCHED [ AND condition ] THEN { merge_update | merge_delete | DO NOTHING } |
//   WHEN NOT MATCHED BY SOURCE [ AND condition ] THEN { merge_update | merge_delete | DO NOTHING } |
//   WHEN NOT MATCHED [ BY TARGET ] [ AND condition ] THEN { merge_insert | DO NOTHING } }
//
// and merge_insert is:
// INSERT [ ( column_name [, ...] ) ]
// [ OVERRIDING { SYSTEM | USER } VALUE ]
// { VALUES ( { expression | DEFAULT } [, ...] ) | DEFAULT VALUES }
//
// and merge_update is:
// UPDATE SET { column_name = { expression | DEFAULT } |
//              ( column_name [, ...] ) = [ ROW ] ( { expression | DEFAULT } [, ...] ) |
//              ( column_name [, ...] ) = ( sub-SELECT )
//            } [, ...]
//
// and merge_delete is:
// DELETE
fn merge_when_clause(p: &mut Parser<'_>) {
    p.expect(WHEN_KW);
    match p.current() {
        MATCHED_KW => {
            p.bump(MATCHED_KW);
        }
        NOT_KW => {
            p.bump(NOT_KW);
            p.expect(MATCHED_KW);
            // TODO: need a validation to check these
            // BY SOURCE | BY TARGET
            if p.eat(BY_KW) {
                let _ = p.eat(SOURCE_KW) || p.eat(TARGET_KW);
            }
        }
        _ => p.error("expected MATCHED, or NOT MATCHED"),
    }
    // [ AND condition ]
    if p.eat(AND_KW) {
        if expr(p).is_none() {
            p.error("expected condition");
        }
    }
    p.expect(THEN_KW);
    // merge_update | merge_delete | merge_insert | DO NOTHING
    match p.current() {
        // merge_delete
        DELETE_KW => {
            p.bump(DELETE_KW);
        }
        // merge_update
        UPDATE_KW => {
            p.bump(UPDATE_KW);
            set_clause(p);
        }
        // merge_insert
        INSERT_KW => {
            p.bump(INSERT_KW);
            // [ ( column_name [, ...] ) ]
            opt_column_list(p);
            // [ OVERRIDING { SYSTEM | USER } VALUE ]
            if p.eat(OVERRIDING_KW) {
                if !p.eat(SYSTEM_KW) && !p.eat(USER_KW) {
                    p.error("expected SYSTEM or USER");
                }
                p.expect(VALUE_KW);
            }
            // { VALUES ( { expression | DEFAULT } [, ...] ) | DEFAULT VALUES }
            if p.at(VALUES_KW) {
                values_clause(p, None);
            } else if p.eat(DEFAULT_KW) {
                p.expect(VALUES_KW);
            } else {
                p.error("expected VALUES or DEFAULT VALUES");
            }
        }
        // DO NOTHING
        DO_KW => {
            p.bump(DO_KW);
            p.expect(NOTHING_KW);
        }
        _ => p.error("expected INSERT, UPDATE, DELETE, or DO NOTHING"),
    }
}

// REASSIGN OWNED BY { old_role | CURRENT_ROLE | CURRENT_USER | SESSION_USER } [, ...]
//                TO { new_role | CURRENT_ROLE | CURRENT_USER | SESSION_USER }
fn reassign_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(REASSIGN_KW));
    let m = p.start();
    p.bump(REASSIGN_KW);
    p.expect(OWNED_KW);
    p.expect(BY_KW);
    role(p);
    while !p.at(EOF) && p.eat(COMMA) {
        role(p);
    }
    p.expect(TO_KW);
    role(p);
    while !p.at(EOF) && p.eat(COMMA) {
        role(p);
    }
    m.complete(p, REASSIGN_STMT)
}

// REFRESH MATERIALIZED VIEW [ CONCURRENTLY ] name
//     [ WITH [ NO ] DATA ]
fn refresh_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(REFRESH_KW));
    let m = p.start();
    p.bump(REFRESH_KW);
    p.expect(MATERIALIZED_KW);
    p.expect(VIEW_KW);
    p.eat(CONCURRENTLY_KW);
    path_name_ref(p);
    if p.eat(WITH_KW) {
        p.eat(NO_KW);
        p.expect(DATA_KW);
    }
    m.complete(p, REFRESH_STMT)
}

// GRANT { { SELECT | INSERT | UPDATE | DELETE | TRUNCATE | REFERENCES | TRIGGER | MAINTAIN }
//     [, ...] | ALL [ PRIVILEGES ] }
//     ON { [ TABLE ] table_name [, ...]
//          | ALL TABLES IN SCHEMA schema_name [, ...] }
//     TO role_specification [, ...] [ WITH GRANT OPTION ]
//     [ GRANTED BY role_specification ]
fn grant_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(GRANT_KW));
    let m = p.start();
    p.bump(GRANT_KW);
    // TODO: we'll need a syntax validation for this since it uses specific
    // idents not purely keywords
    // TODO: we can cleanup this function a lot
    // TODO: might be able to dedupe with revoke since it's mostly copy paste

    // { { SELECT | INSERT | UPDATE | REFERENCES } ( column_name [, ...] )
    // [, ...] | ALL [ PRIVILEGES ] ( column_name [, ...] ) }
    // { { SELECT | INSERT | UPDATE | DELETE | TRUNCATE | REFERENCES | TRIGGER | MAINTAIN }
    //  [, ...] | ALL [ PRIVILEGES ] }
    // ALL [ PRIVILEGES ]
    if p.eat(ALL_KW) {
        p.eat(PRIVILEGES_KW);
    } else if !p.at(TO_KW) {
        revoke_command(p);
        while !p.at(EOF) && p.eat(COMMA) {
            revoke_command(p);
        }
    }
    // [ ( column_name [, ...] ) ]
    opt_column_list(p);
    // ON { [ TABLE ] table_name [, ...]
    //      | ALL TABLES IN SCHEMA schema_name [, ...] }
    // ON { SEQUENCE sequence_name [, ...]
    //      | ALL SEQUENCES IN SCHEMA schema_name [, ...] }
    // ON DATABASE database_name [, ...]
    // ON TABLESPACE tablespace_name [, ...]
    // ON { { FUNCTION | PROCEDURE | ROUTINE } function_name [ ( [ [ argmode ] [ arg_name ] arg_type [, ...] ] ) ] [, ...]
    //       | ALL { FUNCTIONS | PROCEDURES | ROUTINES } IN SCHEMA schema_name [, ...] }
    // ON PARAMETER configuration_parameter [, ...]
    if p.eat(ON_KW) {
        if p.eat(ALL_KW) {
            match p.current() {
                TABLES_KW | SEQUENCES_KW | FUNCTIONS_KW | PROCEDURES_KW | ROUTINES_KW => {
                    p.bump_any();
                    p.expect(IN_KW);
                    p.expect(SCHEMA_KW);
                    // schema_name [, ...]
                    name_ref(p);
                    while !p.at(EOF) && p.eat(COMMA) {
                        name_ref(p);
                    }
                }
                _ => p.error("expected TABLE"),
            }
        } else {
            match p.current() {
                PARAMETER_KW => {
                    p.bump(PARAMETER_KW);
                    name_ref(p);
                    while !p.at(EOF) && p.eat(COMMA) {
                        name_ref(p);
                    }
                }
                FUNCTION_KW | PROCEDURE_KW | ROUTINE_KW => {
                    p.bump_any();
                    // function_name [ ( [ [ argmode ] [ arg_name ] arg_type [, ...] ] ) ] [, ...]
                    path_name_ref(p);
                    opt_param_list(p);
                    while !p.at(EOF) && p.eat(COMMA) {
                        // function_name [ ( [ [ argmode ] [ arg_name ] arg_type [, ...] ] ) ] [, ...]
                        path_name_ref(p);
                        opt_param_list(p);
                    }
                }
                // TYPE type_name [, ...]
                TYPE_KW => {
                    p.bump(TYPE_KW);
                    type_name(p);
                    while !p.at(EOF) && p.eat(COMMA) {
                        type_name(p);
                    }
                }
                TABLE_KW | SEQUENCE_KW | DATABASE_KW | TABLESPACE_KW | SCHEMA_KW | LANGUAGE_KW
                | DOMAIN_KW => {
                    p.bump_any();
                    name_ref(p);
                    while !p.at(EOF) && p.eat(COMMA) {
                        name_ref(p);
                    }
                }
                FOREIGN_KW => {
                    p.bump(FOREIGN_KW);
                    if p.eat(DATA_KW) {
                        p.expect(WRAPPER_KW);
                    } else {
                        p.expect(SERVER_KW);
                    }
                    name_ref(p);
                    while !p.at(EOF) && p.eat(COMMA) {
                        name_ref(p);
                    }
                }
                LARGE_KW => {
                    p.bump(LARGE_KW);
                    p.expect(OBJECT_KW);
                    if opt_numeric_literal(p).is_none() {
                        p.error("expected large_object_oid")
                    }
                    while !p.at(EOF) && p.eat(COMMA) {
                        if opt_numeric_literal(p).is_none() {
                            p.error("expected large_object_oid")
                        }
                    }
                }
                // table_name [, ...]
                _ if p.at_ts(COL_LABEL_FIRST) => {
                    name_ref(p);
                    while !p.at(EOF) && p.eat(COMMA) {
                        name_ref(p);
                    }
                }
                _ => (),
            }
        }
    }
    // TO role_specification [, ...]
    p.expect(TO_KW);
    role(p);
    while !p.at(EOF) && p.eat(COMMA) {
        role(p);
    }
    // TODO: need more validation here
    // [ WITH GRANT OPTION ]
    // [ WITH { ADMIN | INHERIT | SET } { OPTION | TRUE | FALSE } ]
    if p.eat(WITH_KW) {
        match p.current() {
            ADMIN_KW | INHERIT_KW | SET_KW => {
                p.bump_any();
                if !(p.eat(OPTION_KW) || p.eat(TRUE_KW) || p.eat(FALSE_KW)) {
                    p.error("expected OPTION, TRUE, or FALSE")
                }
            }
            GRANT_KW => {
                p.bump(GRANT_KW);
                p.expect(OPTION_KW);
            }
            _ => p.error("expected WITH GRANT OPTION or WITH ADMIN/INHERIT/SET OPTION/TRUE/FALSE"),
        }
    }
    opt_granted_by(p);
    m.complete(p, GRANT_STMT)
}

// [ GRANTED BY role_specification ]
fn opt_granted_by(p: &mut Parser<'_>) {
    if p.eat(GRANTED_KW) {
        p.expect(BY_KW);
        role(p);
    }
}

// REVOKE [ GRANT OPTION FOR ]
//     { { SELECT | INSERT | UPDATE | DELETE | TRUNCATE | REFERENCES | TRIGGER | MAINTAIN }
//     [, ...] | ALL [ PRIVILEGES ] }
//     ON { [ TABLE ] table_name [, ...]
//          | ALL TABLES IN SCHEMA schema_name [, ...] }
//     FROM role_specification [, ...]
//     [ GRANTED BY role_specification ]
//     [ CASCADE | RESTRICT ]
fn revoke_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(REVOKE_KW));
    let m = p.start();
    p.bump(REVOKE_KW);
    // TODO: we'll need a syntax validation for this since it uses specific
    // idents not purely keywords
    // TODO: we can cleanup this function a lot
    // [ { ADMIN | INHERIT | SET } OPTION FOR ]
    // [ GRANT OPTION FOR ]
    match p.current() {
        ADMIN_KW | INHERIT_KW | GRANT_KW => {
            p.bump_any();
            p.expect(OPTION_KW);
            p.expect(FOR_KW);
        }
        SET_KW if p.nth_at(1, OPTION_KW) => {
            p.bump(SET_KW);
            p.bump(OPTION_KW);
            p.expect(FOR_KW);
        }
        _ => (),
    }
    privileges(p);
    // ON { [ TABLE ] table_name [, ...]
    //      | ALL TABLES IN SCHEMA schema_name [, ...] }
    // ON { SEQUENCE sequence_name [, ...]
    //      | ALL SEQUENCES IN SCHEMA schema_name [, ...] }
    // ON DATABASE database_name [, ...]
    // ON TABLESPACE tablespace_name [, ...]
    // ON { { FUNCTION | PROCEDURE | ROUTINE } function_name [ ( [ [ argmode ] [ arg_name ] arg_type [, ...] ] ) ] [, ...]
    //       | ALL { FUNCTIONS | PROCEDURES | ROUTINES } IN SCHEMA schema_name [, ...] }
    // ON PARAMETER configuration_parameter [, ...]
    if p.eat(ON_KW) {
        if p.eat(ALL_KW) {
            match p.current() {
                TABLES_KW | SEQUENCES_KW | FUNCTIONS_KW | PROCEDURES_KW | ROUTINES_KW => {
                    p.bump_any();
                    p.expect(IN_KW);
                    p.expect(SCHEMA_KW);
                    // schema_name [, ...]
                    name_ref(p);
                    while !p.at(EOF) && p.eat(COMMA) {
                        name_ref(p);
                    }
                }
                _ => p.error("expected TABLE"),
            }
        } else {
            match p.current() {
                PARAMETER_KW => {
                    p.bump(PARAMETER_KW);
                    name_ref(p);
                    while !p.at(EOF) && p.eat(COMMA) {
                        name_ref(p);
                    }
                }
                FUNCTION_KW | PROCEDURE_KW | ROUTINE_KW => {
                    p.bump_any();
                    path_name_ref(p);
                    opt_param_list(p);
                    while !p.at(EOF) && p.eat(COMMA) {
                        path_name_ref(p);
                        opt_param_list(p);
                    }
                }
                // TYPE type_name [, ...]
                TYPE_KW => {
                    p.bump(TYPE_KW);
                    type_name(p);
                    while !p.at(EOF) && p.eat(COMMA) {
                        type_name(p);
                    }
                }
                TABLE_KW | SEQUENCE_KW | DATABASE_KW | TABLESPACE_KW | SCHEMA_KW | LANGUAGE_KW
                | DOMAIN_KW => {
                    p.bump_any();
                    name_ref(p);
                    while !p.at(EOF) && p.eat(COMMA) {
                        name_ref(p);
                    }
                }
                FOREIGN_KW => {
                    p.bump(FOREIGN_KW);
                    if p.eat(DATA_KW) {
                        p.expect(WRAPPER_KW);
                    } else {
                        p.expect(SERVER_KW);
                    }
                    name_ref(p);
                    while !p.at(EOF) && p.eat(COMMA) {
                        name_ref(p);
                    }
                }
                LARGE_KW => {
                    p.bump(LARGE_KW);
                    p.expect(OBJECT_KW);
                    if opt_numeric_literal(p).is_none() {
                        p.error("expected large_object_oid")
                    }
                    while !p.at(EOF) && p.eat(COMMA) {
                        if opt_numeric_literal(p).is_none() {
                            p.error("expected large_object_oid")
                        }
                    }
                }
                // table_name [, ...]
                _ if p.at_ts(COL_LABEL_FIRST) => {
                    name_ref(p);
                    while !p.at(EOF) && p.eat(COMMA) {
                        name_ref(p);
                    }
                }
                _ => (),
            }
        }
    }
    // FROM role_specification [, ...]
    p.expect(FROM_KW);
    role(p);
    while !p.at(EOF) && p.eat(COMMA) {
        role(p);
    }
    // [ GRANTED BY role_specification ]
    opt_granted_by(p);
    opt_cascade_or_restrict(p);
    m.complete(p, REVOKE_STMT)
}

// { { SELECT | INSERT | UPDATE | REFERENCES } ( column_name [, ...] )
// [, ...] | ALL [ PRIVILEGES ] ( column_name [, ...] ) }
// { { SELECT | INSERT | UPDATE | DELETE | TRUNCATE | REFERENCES | TRIGGER | MAINTAIN }
//  [, ...] | ALL [ PRIVILEGES ] }
fn privileges(p: &mut Parser<'_>) {
    // ALL [ PRIVILEGES ]
    if p.eat(ALL_KW) {
        p.eat(PRIVILEGES_KW);
    } else if !p.at(FROM_KW) {
        revoke_command(p);
        while !p.at(EOF) && p.eat(COMMA) {
            revoke_command(p);
        }
    }
    // [ ( column_name [, ...] ) ]
    opt_column_list(p);
}

const REVOKE_COMMAND_FIRST: TokenSet = TokenSet::new(&[
    SELECT_KW,
    INSERT_KW,
    UPDATE_KW,
    DELETE_KW,
    TRUNCATE_KW,
    REFERENCES_KW,
    TRIGGER_KW,
    IDENT,
    ALL_KW,
    ALTER_KW,
    CREATE_KW,
    TEMPORARY_KW,
    TEMP_KW,
    EXECUTE_KW,
]);

fn revoke_command(p: &mut Parser<'_>) {
    if opt_role(p) {
        return;
    }
    if p.eat(ALTER_KW) {
        p.expect(SYSTEM_KW);
    } else if p.at_ts(REVOKE_COMMAND_FIRST) {
        p.bump_any();
    } else {
        p.error(format!("expected command name, got {:?}", p.current()))
    }
}

// where role_specification can be:
//  | [ GROUP ] role_name
//  | PUBLIC
//  | CURRENT_ROLE
//  | CURRENT_USER
//  | SESSION_USER
fn role(p: &mut Parser<'_>) {
    if !opt_role(p) {
        p.error(format!("expected role, got {:?}", p.current()))
    }
}

fn opt_role(p: &mut Parser<'_>) -> bool {
    match p.current() {
        GROUP_KW => {
            p.bump(GROUP_KW);
            if p.at_ts(NON_RESERVED_WORD)
                || p.at(CURRENT_ROLE_KW)
                || p.at(CURRENT_USER_KW)
                || p.at(SESSION_USER_KW)
            {
                p.bump_any();
            } else {
                p.error(format!("expected role_name, got {:?}", p.current()))
            }
        }
        CURRENT_ROLE_KW | CURRENT_USER_KW | SESSION_USER_KW => {
            p.bump_any();
        }
        ALTER_KW => {
            if !p.nth_at(1, SYSTEM_KW) {
                p.bump_any();
            } else {
                return false;
            }
        }
        _ if p.at_ts(NON_RESERVED_WORD) => {
            p.bump_any();
        }
        _ => return false,
    }
    true
}

// SECURITY LABEL [ FOR provider ] ON
// {
//   TABLE object_name |
//   COLUMN table_name.column_name |
//   AGGREGATE aggregate_name ( aggregate_signature ) |
//   DATABASE object_name |
//   DOMAIN object_name |
//   EVENT TRIGGER object_name |
//   FOREIGN TABLE object_name |
//   FUNCTION function_name [ ( [ [ argmode ] [ argname ] argtype [, ...] ] ) ] |
//   LARGE OBJECT large_object_oid |
//   MATERIALIZED VIEW object_name |
//   [ PROCEDURAL ] LANGUAGE object_name |
//   PROCEDURE procedure_name [ ( [ [ argmode ] [ argname ] argtype [, ...] ] ) ] |
//   PUBLICATION object_name |
//   ROLE object_name |
//   ROUTINE routine_name [ ( [ [ argmode ] [ argname ] argtype [, ...] ] ) ] |
//   SCHEMA object_name |
//   SEQUENCE object_name |
//   SUBSCRIPTION object_name |
//   TABLESPACE object_name |
//   TYPE object_name |
//   VIEW object_name
// } IS { string_literal | NULL }
//
// where aggregate_signature is:
// * |
// [ argmode ] [ argname ] argtype [ , ... ] |
// [ [ argmode ] [ argname ] argtype [ , ... ] ] ORDER BY [ argmode ] [ argname ] argtype [ , ... ]
fn security_label_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(SECURITY_KW) && p.nth_at(1, LABEL_KW));
    let m = p.start();
    p.bump(SECURITY_KW);
    p.bump(LABEL_KW);
    if p.eat(FOR_KW) {
        if p.at_ts(NON_RESERVED_WORD) {
            p.bump_any();
        } else {
            string_literal(p);
        }
    }
    p.expect(ON_KW);
    match p.current() {
        TABLE_KW | COLUMN_KW | DATABASE_KW | DOMAIN_KW | PUBLICATION_KW | ROLE_KW | SCHEMA_KW
        | SEQUENCE_KW | SUBSCRIPTION_KW | TABLESPACE_KW | TYPE_KW | VIEW_KW => {
            p.bump_any();
            path_name(p);
        }
        EVENT_KW => {
            p.bump(EVENT_KW);
            p.expect(TRIGGER_KW);
            path_name(p);
        }
        FOREIGN_KW => {
            p.bump(FOREIGN_KW);
            p.expect(TABLE_KW);
            path_name(p);
        }
        // [ PROCEDURAL ] LANGUAGE object_name
        PROCEDURAL_KW | LANGUAGE_KW => {
            p.eat(PROCEDURAL_KW);
            p.expect(LANGUAGE_KW);
            path_name(p);
        }
        // LARGE OBJECT large_object_oid
        LARGE_KW => {
            p.bump(LARGE_KW);
            p.expect(OBJECT_KW);
            if opt_numeric_literal(p).is_none() {
                p.error("expected large_object_oid")
            }
        }
        MATERIALIZED_KW => {
            p.bump(MATERIALIZED_KW);
            p.expect(VIEW_KW);
            path_name(p);
        }
        FUNCTION_KW | PROCEDURE_KW | ROUTINE_KW => {
            p.bump_any();
            path_name_ref(p);
            opt_param_list(p);
        }
        AGGREGATE_KW => {
            p.bump(AGGREGATE_KW);
            path_name(p);
            aggregate_arg_list(p);
        }
        _ => p.error("expected database object name"),
    }
    p.expect(IS_KW);
    if !p.eat(NULL_KW) {
        string_literal(p);
    }
    m.complete(p, SECURITY_LABEL_STMT)
}

fn agg_args(p: &mut Parser<'_>) {
    match p.current() {
        STAR => {
            p.bump(STAR);
        }
        // ORDER BY [ argmode ] [ argname ] argtype [ , ... ]
        ORDER_KW => {
            p.bump(ORDER_KW);
            p.expect(BY_KW);
            // TODO: generalize
            param(p);
            while !p.at(EOF) {
                if p.eat(COMMA) {
                    param(p);
                } else {
                    break;
                }
            }
        }
        _ => {
            param(p);
            while !p.at(EOF) {
                if p.eat(COMMA) {
                    param(p);
                } else {
                    break;
                }
            }
            // ORDER BY [ argmode ] [ argname ] argtype [ , ... ]
            if p.eat(ORDER_KW) {
                p.expect(BY_KW);
                // TODO: generalize
                param(p);
                while !p.at(EOF) {
                    if p.eat(COMMA) {
                        param(p);
                    } else {
                        break;
                    }
                }
            }
        }
    }
}

fn aggregate_arg_list(p: &mut Parser<'_>) {
    let m = p.start();
    p.expect(L_PAREN);
    agg_args(p);
    p.expect(R_PAREN);
    m.complete(p, PARAM_LIST);
}

// SET CONSTRAINTS { ALL | name [, ...] } { DEFERRED | IMMEDIATE }
fn set_constraints_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(SET_KW) && p.nth_at(1, CONSTRAINTS_KW));
    let m = p.start();
    p.bump(SET_KW);
    p.bump(CONSTRAINTS_KW);
    if !p.eat(ALL_KW) {
        // TODO: generalize
        path_name(p);
        while !p.at(EOF) {
            if p.eat(COMMA) {
                path_name(p);
            } else {
                break;
            }
        }
    }
    if !p.eat(DEFERRED_KW) && !p.eat(IMMEDIATE_KW) {
        p.error("expected DEFERRED or IMMEDIATE");
    }
    m.complete(p, SET_CONSTRAINTS_STMT)
}

// SET [ SESSION | LOCAL ] ROLE role_name
// SET [ SESSION | LOCAL ] ROLE NONE
// RESET ROLE
fn set_role_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(SET_KW) || p.at(RESET_KW));
    let m = p.start();
    if p.eat(RESET_KW) {
        p.expect(ROLE_KW);
    } else {
        p.bump(SET_KW);
        let _ = p.eat(SESSION_KW) || p.eat(LOCAL_KW);
        p.expect(ROLE_KW);
        if !p.eat(NONE_KW) && !p.eat(IDENT) && opt_string_literal(p).is_none() {
            p.error("expected NONE or role_name");
        }
    }
    m.complete(p, SET_ROLE_STMT)
}

// SET [ SESSION | LOCAL ] SESSION AUTHORIZATION user_name
// SET [ SESSION | LOCAL ] SESSION AUTHORIZATION DEFAULT
// RESET SESSION AUTHORIZATION
fn set_session_auth_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(SET_KW) || p.at(RESET_KW));
    let m = p.start();
    if p.at(RESET_KW) {
        p.bump(RESET_KW);
        p.expect(SESSION_KW);
        p.expect(AUTHORIZATION_KW);
    } else {
        p.bump(SET_KW);
        p.eat(LOCAL_KW);
        p.expect(SESSION_KW);
        p.eat(SESSION_KW);
        p.expect(AUTHORIZATION_KW);
        if p.at_ts(NON_RESERVED_WORD) || p.at(DEFAULT_KW) {
            p.bump_any();
        } else if opt_string_literal(p).is_none() {
            p.error("expected user_name or DEFAULT");
        }
    }
    m.complete(p, SET_SESSION_AUTH_STMT)
}

// SET TRANSACTION transaction_mode [, ...]
// SET TRANSACTION SNAPSHOT snapshot_id
// SET SESSION CHARACTERISTICS AS TRANSACTION transaction_mode [, ...]
//
// where transaction_mode is one of:
//     ISOLATION LEVEL { SERIALIZABLE | REPEATABLE READ | READ COMMITTED | READ UNCOMMITTED }
//     READ WRITE | READ ONLY
//     [ NOT ] DEFERRABLE
fn set_transaction_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(SET_KW));
    let m = p.start();
    p.bump(SET_KW);
    if p.eat(SESSION_KW) {
        p.expect(CHARACTERISTICS_KW);
        p.expect(AS_KW);
        p.expect(TRANSACTION_KW);
        // TODO: generalize
        // transaction_mode [, ...]
        while !p.at(EOF) {
            if !opt_transaction_mode(p) {
                p.error("expected transaction mode");
            }
            if !p.eat(COMMA) {
                break;
            }
        }
    } else {
        p.expect(TRANSACTION_KW);
        // [ SNAPSHOT snapshot_id ]
        if p.eat(SNAPSHOT_KW) {
            string_literal(p);
        } else {
            // TODO: generalize
            // transaction_mode [, ...]
            while !p.at(EOF) {
                if !opt_transaction_mode(p) {
                    break;
                }
                if !p.eat(COMMA) {
                    break;
                }
            }
        }
    }
    m.complete(p, SET_TRANSACTION_STMT)
}

// VALUES ( expression [, ...] ) [, ...]
//     [ ORDER BY sort_expression [ ASC | DESC | USING operator ] [, ...] ]
//     [ LIMIT { count | ALL } ]
//     [ OFFSET start [ ROW | ROWS ] ]
//     [ FETCH { FIRST | NEXT } [ count ] { ROW | ROWS } ONLY ]
fn values_clause(p: &mut Parser<'_>, m: Option<Marker>) -> CompletedMarker {
    let m = m.unwrap_or_else(|| p.start());
    p.bump(VALUES_KW);
    // TODO: generalize this
    while !p.at(EOF) {
        if !p.at(L_PAREN) {
            p.err_and_bump("expected L_PAREN");
            continue;
        }
        delimited(
            p,
            L_PAREN,
            R_PAREN,
            COMMA,
            || "expected expression".to_string(),
            EXPR_FIRST,
            |p| expr(p).is_some(),
        );
        if !p.eat(COMMA) {
            if p.at(L_PAREN) {
                p.error("expected COMMA");
            } else {
                break;
            }
        }
    }
    opt_order_by_clause(p);
    opt_limit_clause(p);
    opt_offset_clause(p);
    opt_fetch_clause(p);
    m.complete(p, SELECT)
}

// REINDEX [ ( option [, ...] ) ] { INDEX | TABLE | SCHEMA } [ CONCURRENTLY ] name
// REINDEX [ ( option [, ...] ) ] { DATABASE | SYSTEM } [ CONCURRENTLY ] [ name ]
//
// where option can be one of:
//     CONCURRENTLY [ boolean ]
//     TABLESPACE new_tablespace
//     VERBOSE [ boolean ]
fn reindex_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(REINDEX_KW));
    let m = p.start();
    p.bump(REINDEX_KW);
    // TODO: we need to general this stuff
    if p.eat(L_PAREN) {
        let mut found = false;
        while !p.at(EOF) {
            match p.current() {
                CONCURRENTLY_KW | VERBOSE_KW => {
                    p.bump_any();
                    opt_bool_literal(p);
                    found = true;
                }
                TABLESPACE_KW => {
                    p.bump_any();
                    name(p);
                    found = true;
                }
                kind => {
                    p.error(format!(
                        "expected CONCURRENTLY, TABLESPACE, or VERBOSE option, got {:?}",
                        kind
                    ));
                    break;
                }
            }
            if !p.eat(COMMA) {
                break;
            }
        }
        if !found {
            p.error("expected CONCURRENTLY, TABLESPACE, or VERBOSE option");
        }
        p.expect(R_PAREN);
    }
    let name_required = match p.current() {
        // { INDEX | TABLE | SCHEMA }
        INDEX_KW | TABLE_KW | SCHEMA_KW => {
            p.bump_any();
            true
        }
        // { DATABASE | SYSTEM }
        DATABASE_KW | SYSTEM_KW => {
            p.bump_any();
            false
        }
        _ => {
            p.error("expected INDEX, TABLE, SCHEMA, DATABASE, or SYSTEM");
            true
        }
    };
    p.eat(CONCURRENTLY_KW);
    if opt_name(p).is_none() && name_required {
        p.error("expected name");
    }
    m.complete(p, REINDEX_STMT)
}

// DROP VIEW [ IF EXISTS ] name [, ...] [ CASCADE | RESTRICT ]
fn drop_view_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(DROP_KW) && p.nth_at(1, VIEW_KW));
    let m = p.start();
    p.bump(DROP_KW);
    p.bump(VIEW_KW);
    opt_if_exists(p);
    // name [, ...]
    // TODO: we need to generalize this pattern
    path_name_ref(p);
    if p.eat(COMMA) {
        while !p.at(EOF) {
            path_name_ref(p);
            if !p.eat(COMMA) {
                break;
            }
        }
    }
    opt_cascade_or_restrict(p);
    m.complete(p, DROP_VIEW_STMT)
}

// CREATE [ OR REPLACE ] [ TEMP | TEMPORARY ] [ RECURSIVE ] VIEW name [ ( column_name [, ...] ) ]
//     [ WITH ( view_option_name [= view_option_value] [, ... ] ) ]
//     AS query
//     [ WITH [ CASCADED | LOCAL ] CHECK OPTION ]
fn create_view_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(CREATE_KW));
    let m = p.start();
    p.bump(CREATE_KW);
    opt_or_replace(p);
    opt_temp(p);
    p.eat(RECURSIVE_KW);
    p.expect(VIEW_KW);
    path_name(p);
    // [ ( column_name [, ...] ) ]
    opt_column_list(p);
    // [ WITH ( view_option_name [= view_option_value] [, ... ] ) ]
    // TODO: this can be more specific
    opt_with_params(p);
    p.expect(AS_KW);
    match stmt(
        p,
        &StmtRestrictions {
            begin_end_allowed: false,
        },
    ) {
        Some(statement) => match statement.kind() {
            SELECT | COMPOUND_SELECT => (),
            kind => p.error(format!("expected SELECT, got {:?}", kind)),
        },
        None => p.error("expected SELECT"),
    }
    // [ WITH [ CASCADED | LOCAL ] CHECK OPTION ]
    if p.eat(WITH_KW) {
        let _ = p.eat(CASCADED_KW) | p.eat(LOCAL_KW);
        p.expect(CHECK_KW);
        p.expect(OPTION_KW);
    }
    m.complete(p, CREATE_VIEW_STMT)
}

// EXECUTE name [ ( parameter [, ...] ) ]
fn execute_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(EXECUTE_KW));
    let m = p.start();
    p.bump(EXECUTE_KW);
    name_ref(p);
    // [ ( parameter [, ...] ) ]
    if p.at(L_PAREN) {
        arg_list(p);
    }
    m.complete(p, EXECUTE_STMT)
}

// PREPARE name [ ( data_type [, ...] ) ] AS statement
fn prepare_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(PREPARE_KW));
    let m = p.start();
    p.bump(PREPARE_KW);
    name(p);
    if p.eat(L_PAREN) {
        while !p.at(EOF) {
            type_name(p);
            if !p.eat(COMMA) {
                break;
            }
        }
        p.expect(R_PAREN);
    }
    p.expect(AS_KW);
    // statement
    // Any SELECT, INSERT, UPDATE, DELETE, MERGE, or VALUES statement.
    let statement = stmt(
        p,
        &StmtRestrictions {
            begin_end_allowed: false,
        },
    );
    if let Some(statement) = statement {
        match statement.kind() {
            SELECT | INSERT_STMT | UPDATE_STMT | DELETE_STMT | MERGE_STMT => (),
            kind => {
                p.error(format!(
                    "expected SELECT, INSERT, UPDATE, DELETE, MERGE, or VALUES statement, got {:?}",
                    kind
                ));
            }
        }
    } else {
        p.error("expected SELECT, INSERT, UPDATE, DELETE, MERGE, or VALUES statement");
    }
    m.complete(p, PREPARE_STMT)
}

// UNLISTEN { channel | * }
fn unlisten_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(UNLISTEN_KW));
    let m = p.start();
    p.bump(UNLISTEN_KW);
    if !p.eat(STAR) {
        name(p);
    }
    m.complete(p, UNLISTEN_STMT)
}

// CHECKPOINT
fn checkpoint_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(CHECKPOINT_KW));
    let m = p.start();
    p.bump(CHECKPOINT_KW);
    m.complete(p, CHECKPOINT_STMT)
}

// DEALLOCATE [ PREPARE ] { name | ALL }
fn deallocate_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(DEALLOCATE_KW));
    let m = p.start();
    p.bump(DEALLOCATE_KW);
    p.eat(PREPARE_KW);
    if !p.eat(ALL_KW) {
        name(p);
    }
    m.complete(p, DEALLOCATE_STMT)
}

// LOAD string
fn load_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(LOAD_KW));
    let m = p.start();
    p.bump(LOAD_KW);
    string_literal(p);
    m.complete(p, LOAD_STMT)
}

// LISTEN channel
fn listen_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(LISTEN_KW));
    let m = p.start();
    p.bump(LISTEN_KW);
    name(p);
    m.complete(p, LISTEN_STMT)
}

// NOTIFY channel [ , payload ]
fn notify_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(NOTIFY_KW));
    let m = p.start();
    p.bump(NOTIFY_KW);
    name(p);
    // [ , payload ]
    if p.eat(COMMA) {
        string_literal(p);
    }
    m.complete(p, NOTIFY_STMT)
}

fn reset_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(RESET_KW));
    let m = p.start();
    p.bump(RESET_KW);
    if !p.eat(ALL_KW) {
        name_ref(p);
    }
    m.complete(p, RESET_STMT)
}

// DISCARD { ALL | PLANS | SEQUENCES | TEMPORARY | TEMP }
fn discard_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(DISCARD_KW));
    let m = p.start();
    p.bump(DISCARD_KW);
    let _ = p.eat(ALL_KW) || p.eat(PLANS_KW) || p.eat(SEQUENCES_KW) || opt_temp(p);
    m.complete(p, DISCARD_STMT)
}

fn opt_temp(p: &mut Parser<'_>) -> bool {
    p.eat(TEMP_KW) || p.eat(TEMPORARY_KW)
}

// DO [ LANGUAGE lang_name ] code
fn do_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(DO_KW));
    let m = p.start();
    p.bump(DO_KW);
    if p.eat(LANGUAGE_KW) {
        if p.at_ts(NON_RESERVED_WORD) {
            p.bump_any();
        } else {
            string_literal(p);
        }
    }
    string_literal(p);
    m.complete(p, DO_STMT)
}

// DECLARE name [ BINARY ] [ ASENSITIVE | INSENSITIVE ] [ [ NO ] SCROLL ]
//     CURSOR [ { WITH | WITHOUT } HOLD ] FOR query
fn declare_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(DECLARE_KW));
    let m = p.start();
    p.bump(DECLARE_KW);
    name(p);
    // [ BINARY ]
    p.eat(BINARY_KW);
    // [ ASENSITIVE | INSENSITIVE ]
    let _ = p.eat(ASENSITIVE_KW) || p.eat(INSENSITIVE_KW);
    // [ [ NO ] SCROLL ]
    if p.eat(NO_KW) {
        p.expect(SCROLL_KW);
    } else {
        p.eat(SCROLL_KW);
    }
    p.expect(CURSOR_KW);
    // [ { WITH | WITHOUT } HOLD ]
    if p.eat(WITH_KW) || p.eat(WITHOUT_KW) {
        p.expect(HOLD_KW);
    }
    p.expect(FOR_KW);
    // select_stmt | values_stmt
    query(p);
    m.complete(p, DECLARE_STMT)
}

fn opt_direction(p: &mut Parser<'_>) -> bool {
    match p.current() {
        NEXT_KW | PRIOR_KW | FIRST_KW | LAST_KW | ALL_KW => {
            p.bump_any();
        }
        RELATIVE_KW | ABSOLUTE_KW => {
            p.bump_any();
            if opt_numeric_literal(p).is_none() {
                p.error("expected count")
            }
        }
        FORWARD_KW | BACKWARD_KW => {
            p.bump_any();
            if !p.eat(ALL_KW) {
                let _ = opt_numeric_literal(p);
            }
        }
        // count
        _ if p.at_ts(NUMERIC_FIRST) => {
            if opt_numeric_literal(p).is_none() {
                p.error("expected count")
            }
        }
        _ => return false,
    }
    true
}

// MOVE [ direction ] [ FROM | IN ] cursor_name
// where direction can be one of:
//     NEXT
//     PRIOR
//     FIRST
//     LAST
//     ABSOLUTE count
//     RELATIVE count
//     count
//     ALL
//     FORWARD
//     FORWARD count
//     FORWARD ALL
//     BACKWARD
//     BACKWARD count
//     BACKWARD ALL
fn move_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(MOVE_KW));
    let m = p.start();
    p.bump(MOVE_KW);
    opt_direction(p);
    let _ = p.eat(FROM_KW) || p.eat(IN_KW);
    // cursor_name
    name(p);
    m.complete(p, MOVE_STMT)
}

// FETCH [ direction ] [ FROM | IN ] cursor_name
// where direction can be one of:
//     NEXT
//     PRIOR
//     FIRST
//     LAST
//     ABSOLUTE count
//     RELATIVE count
//     count
//     ALL
//     FORWARD
//     FORWARD count
//     FORWARD ALL
//     BACKWARD
//     BACKWARD count
//     BACKWARD ALL
fn fetch_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(FETCH_KW));
    let m = p.start();
    p.bump(FETCH_KW);
    opt_direction(p);
    let _ = p.eat(FROM_KW) || p.eat(IN_KW);
    // cursor_name
    name(p);
    m.complete(p, FETCH_STMT)
}

// CLOSE { name | ALL }
fn close_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(CLOSE_KW));
    let m = p.start();
    p.bump(CLOSE_KW);
    if !p.eat(ALL_KW) {
        name(p);
    }
    m.complete(p, CLOSE_STMT)
}

// TRUNCATE [ TABLE ] [ ONLY ] name [ * ] [, ... ]
//   [ RESTART IDENTITY | CONTINUE IDENTITY ] [ CASCADE | RESTRICT ]
fn truncate_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(TRUNCATE_KW));
    let m = p.start();
    p.bump(TRUNCATE_KW);
    p.eat(TABLE_KW);
    while !p.at(EOF) {
        relation_name(p);
        if !p.eat(COMMA) {
            break;
        }
    }
    if p.eat(RESTART_KW) {
        p.expect(IDENTITY_KW);
    }
    if p.eat(CONTINUE_KW) {
        p.expect(IDENTITY_KW);
    }
    opt_cascade_or_restrict(p);
    m.complete(p, TRUNCATE_STMT)
}

// VACUUM [ ( option [, ...] ) ] [ table_and_columns [, ...] ]
//
// where option can be one of:
//     FULL [ boolean ]
//     FREEZE [ boolean ]
//     VERBOSE [ boolean ]
//     ANALYZE [ boolean ]
//     DISABLE_PAGE_SKIPPING [ boolean ]
//     SKIP_LOCKED [ boolean ]
//     INDEX_CLEANUP { AUTO | ON | OFF }
//     PROCESS_MAIN [ boolean ]
//     PROCESS_TOAST [ boolean ]
//     TRUNCATE [ boolean ]
//     PARALLEL integer
//     SKIP_DATABASE_STATS [ boolean ]
//     ONLY_DATABASE_STATS [ boolean ]
//     BUFFER_USAGE_LIMIT size
//
// and table_and_columns is:
//     table_name [ ( column_name [, ...] ) ]
//
// pre postgres 9 syntax:
//
// VACUUM [ FULL ] [ FREEZE ] [ VERBOSE ] [ ANALYZE ] [ table_and_columns [, ...] ]
fn vacuum_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(VACUUM_KW));
    let m = p.start();
    p.bump(VACUUM_KW);
    // [ FULL ]
    p.eat(FULL_KW);
    // [ FREEZE ]
    p.eat(FREEZE_KW);
    // [ VERBOSE ]
    p.eat(VERBOSE_KW);
    // [ ANALYZE ]
    let _ = p.eat(ANALYZE_KW) || p.eat(ANALYSE_KW);
    // [ ( option [, ...] ) ]
    if p.at(L_PAREN) {
        p.expect(L_PAREN);
        while !p.at(EOF) {
            vacuum_option(p);
            if !p.eat(COMMA) {
                break;
            }
        }
        p.expect(R_PAREN);
    }
    opt_relation_list(p);
    m.complete(p, VACUUM_STMT)
}

// [ table_and_columns [, ...] ]
// where table_and_coumns is:
//  table_name [ ( column_name [, ...] ) ]
fn opt_relation_list(p: &mut Parser<'_>) {
    while !p.at(EOF) {
        if opt_path_name_ref(p).is_none() {
            break;
        }
        opt_column_list(p);
        if !p.eat(COMMA) {
            break;
        }
    }
}

// where option can be one of:
//   FORMAT format_name
//   FREEZE [ boolean ]
//   DELIMITER 'delimiter_character'
//   NULL 'null_string'
//   DEFAULT 'default_string'
//   HEADER [ boolean | MATCH ]
//   QUOTE 'quote_character'
//   ESCAPE 'escape_character'
//   FORCE_QUOTE { ( column_name [, ...] ) | * }
//   FORCE_NOT_NULL { ( column_name [, ...] ) | * }
//   FORCE_NULL { ( column_name [, ...] ) | * }
//   ON_ERROR error_action
//   ENCODING 'encoding_name'
//   LOG_VERBOSITY verbosity
fn vacuum_option(p: &mut Parser<'_>) {
    // utility_option_name
    if p.at_ts(NON_RESERVED_WORD) || p.at(ANALYZE_KW) || p.at(ANALYSE_KW) || p.at(FORMAT_KW) {
        p.bump_any();
    }
    if p.at_ts(NON_RESERVED_WORD) || p.at(ON_KW) {
        p.bump_any();
        return;
    }
    // utility_option_arg
    if opt_numeric_literal(p).is_some() {
        return;
    }
    if opt_string_literal(p).is_some() {
        return;
    }
    if opt_bool_literal(p) {
        return;
    }
}

// copy_generic_opt_elem:
//       ColLabel copy_generic_opt_arg
//
// copy_generic_opt_arg:
//       opt_boolean_or_string
//       | NumericOnly
//       | '*'
//       | DEFAULT
//       | '(' copy_generic_opt_arg_list ')'
//       | /* EMPTY */
fn opt_copy_option(p: &mut Parser) -> bool {
    col_label(p);
    copy_option_arg(p);
    true
}

fn copy_option_arg(p: &mut Parser<'_>) {
    match p.current() {
        STAR | DEFAULT_KW => {
            p.bump_any();
        }
        L_PAREN => {
            copy_option_list(p);
        }
        ON_KW => {}
        _ => {
            if p.at_ts(NON_RESERVED_WORD) {
                p.bump_any();
                return;
            }
            if opt_numeric_literal(p).is_some() {
                return;
            }
            if opt_string_literal(p).is_some() {
                return;
            }
            if opt_bool_literal(p) {
                return;
            }
        }
    }
}

fn copy_option_list(p: &mut Parser<'_>) {
    p.expect(L_PAREN);
    while !p.at(EOF) {
        if !opt_copy_option(p) {
            p.error("expected copy option");
        }
        if !p.eat(COMMA) {
            break;
        }
    }
    p.expect(R_PAREN);
}

// COPY table_name [ ( column_name [, ...] ) ]
//     FROM { 'filename' | PROGRAM 'command' | STDIN }
//     [ [ WITH ] ( option [, ...] ) ]
//     [ WHERE condition ]
//
// COPY { table_name [ ( column_name [, ...] ) ] | ( query ) }
//     TO { 'filename' | PROGRAM 'command' | STDOUT }
//     [ [ WITH ] ( option [, ...] ) ]
//
// where option can be one of:
//  FORMAT format_name
//  FREEZE [ boolean ]
//  DELIMITER 'delimiter_character'
//  NULL 'null_string'
//  DEFAULT 'default_string'
//  HEADER [ boolean | MATCH ]
//  QUOTE 'quote_character'
//  ESCAPE 'escape_character'
//  FORCE_QUOTE { ( column_name [, ...] ) | * }
//  FORCE_NOT_NULL { ( column_name [, ...] ) | * }
//  FORCE_NULL { ( column_name [, ...] ) | * }
//  ON_ERROR error_action
//  ENCODING 'encoding_name'
//  LOG_VERBOSITY verbosity
//
// # Pre postgres 9 syntax:
//
// COPY table_name [ ( column_name [, ...] ) ]
//     FROM { 'filename' | STDIN }
//     [ [ WITH ]
//           [ BINARY ]
//           [ DELIMITER [ AS ] 'delimiter_character' ]
//           [ NULL [ AS ] 'null_string' ]
//           [ CSV [ HEADER ]
//                 [ QUOTE [ AS ] 'quote_character' ]
//                 [ ESCAPE [ AS ] 'escape_character' ]
//                 [ FORCE NOT NULL column_name [, ...] ] ] ]
//
// COPY { table_name [ ( column_name [, ...] ) ] | ( query ) }
//     TO { 'filename' | STDOUT }
//     [ [ WITH ]
//           [ BINARY ]
//           [ DELIMITER [ AS ] 'delimiter_character' ]
//           [ NULL [ AS ] 'null_string' ]
//           [ CSV [ HEADER ]
//                 [ QUOTE [ AS ] 'quote_character' ]
//                 [ ESCAPE [ AS ] 'escape_character' ]
//                 [ FORCE QUOTE { column_name [, ...] | * } ] ] ]
//
// pre postgres 7.3
//
// COPY [ BINARY ] table_name
//     FROM { 'filename' | STDIN }
//     [ [USING] DELIMITERS 'delimiter_character' ]
//     [ WITH NULL AS 'null_string' ]
//
// COPY [ BINARY ] table_name
//     TO { 'filename' | STDOUT }
//     [ [USING] DELIMITERS 'delimiter_character' ]
//     [ WITH NULL AS 'null_string' ]
fn copy_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(COPY_KW));
    let m = p.start();
    p.bump(COPY_KW);
    if p.eat(L_PAREN) {
        // query
        query(p);
        p.expect(R_PAREN);
    } else {
        // table_name
        path_name(p);
        // [ ( column_name [, ...] ) ]
        opt_column_list(p);
    }
    if p.eat(FROM_KW) {
        // STDIN
        if p.eat(STDIN_KW) {
            // PROGRAM 'command'
        } else if p.eat(PROGRAM_KW) {
            string_literal(p);
        // 'filename'
        } else {
            string_literal(p);
        }
    } else if p.eat(TO_KW) {
        if !p.eat(STDOUT_KW) {
            p.eat(PROGRAM_KW);
            string_literal(p);
        }
    }
    // [ [ WITH ] ( option [, ...] ) ]
    if p.eat(WITH_KW) || p.at(L_PAREN) {
        copy_option_list(p);
    }
    opt_where_clause(p);
    m.complete(p, COPY_STMT)
}

// https://www.postgresql.org/docs/17/sql-call.html
// CALL name ( [ argument ] [, ...] )
fn call_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(CALL_KW));
    let m = p.start();
    p.bump(CALL_KW);
    path_name(p);
    if p.at(L_PAREN) {
        arg_list(p);
    } else {
        p.error("expected L_PAREN");
    }
    m.complete(p, CALL_STMT)
}

// https://www.postgresql.org/docs/17/sql-createtrigger.html
// CREATE [ OR REPLACE ] [ CONSTRAINT ] TRIGGER name { BEFORE | AFTER | INSTEAD OF } { event [ OR ... ] }
//     ON table_name
//     [ FROM referenced_table_name ]
//     [ NOT DEFERRABLE | [ DEFERRABLE ] [ INITIALLY IMMEDIATE | INITIALLY DEFERRED ] ]
//     [ REFERENCING { { OLD | NEW } TABLE [ AS ] transition_relation_name } [ ... ] ]
//     [ FOR [ EACH ] { ROW | STATEMENT } ]
//     [ WHEN ( condition ) ]
//     EXECUTE { FUNCTION | PROCEDURE } function_name ( arguments )
//
// where event can be one of:
//     INSERT
//     UPDATE [ OF column_name [, ... ] ]
//     DELETE
//     TRUNCATE
fn create_trigger_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(CREATE_KW));
    let m = p.start();
    p.bump(CREATE_KW);
    opt_or_replace(p);
    p.eat(CONSTRAINT_KW);
    // TRIGGER name
    p.expect(TRIGGER_KW);
    name(p);
    // { BEFORE | AFTER | INSTEAD OF }
    if p.eat(INSTEAD_KW) {
        p.expect(OF_KW);
    } else if !p.eat(BEFORE_KW) && !p.eat(AFTER_KW) {
        p.error("expected BEFORE, AFTER, or INSTEAD OF");
    }
    // { event [ OR ... ] }
    while !p.at(EOF) {
        if p.eat(UPDATE_KW) {
            // [ OF column_name [, ... ] ]
            if p.eat(OF_KW) {
                while !p.at(EOF) {
                    name_ref(p);
                    if !p.eat(COMMA) {
                        break;
                    }
                }
            }
        } else if !(p.eat(INSERT_KW) || p.eat(DELETE_KW) || p.eat(TRUNCATE_KW)) {
            p.error("expected INSERT, UPDATE, DELETE, or TRUNCATE");
        }
        if !p.eat(OR_KW) {
            break;
        }
    }
    // ON table_name
    p.expect(ON_KW);
    path_name_ref(p);
    // [ FROM referenced_table_name ]
    if p.eat(FROM_KW) {
        path_name_ref(p);
    }
    // [ NOT DEFERRABLE | [ DEFERRABLE ] [ INITIALLY IMMEDIATE | INITIALLY DEFERRED ] ]
    opt_constraint_options(p);
    // [ REFERENCING { { OLD | NEW } TABLE [ AS ] transition_relation_name } [ ... ] ]
    if p.eat(REFERENCING_KW) {
        while !p.at(EOF) {
            if !opt_referencing_table(p) {
                break;
            }
        }
    }
    // [ FOR [ EACH ] { ROW | STATEMENT } ]
    if p.eat(FOR_KW) {
        p.eat(EACH_KW);
        if p.at(ROW_KW) || p.at(STATEMENT_KW) {
            p.bump_any();
        } else {
            p.error("expected ROW or STATEMENT");
        }
    }
    // [ WHEN ( condition ) ]
    if p.eat(WHEN_KW) {
        p.expect(L_PAREN);
        if expr(p).is_none() {
            p.error("expected expression");
        }
        p.expect(R_PAREN);
    }
    // EXECUTE { FUNCTION | PROCEDURE } function_name ( arguments )
    p.expect(EXECUTE_KW);
    if !p.eat(FUNCTION_KW) && !p.eat(PROCEDURE_KW) {
        p.error("expected FUNCTION or PROCEDURE");
    }
    // function_name ( arguments )
    call_expr(p);
    m.complete(p, CREATE_TRIGGER_STMT)
}

fn call_expr(p: &mut Parser<'_>) {
    match expr(p).map(|x| x.kind()) {
        Some(CALL_EXPR) => (),
        _ => p.error("expected call expression"),
    }
}

// { { OLD | NEW } TABLE [ AS ] transition_relation_name }
fn opt_referencing_table(p: &mut Parser<'_>) -> bool {
    if !(p.at(OLD_KW) || p.at(NEW_KW)) {
        return false;
    }
    p.bump_any();
    p.expect(TABLE_KW);
    p.eat(AS_KW);
    // transition_relation_name
    name_ref(p);
    true
}

// https://www.postgresql.org/docs/17/sql-dropschema.html
// DROP SCHEMA [ IF EXISTS ] name [, ...] [ CASCADE | RESTRICT ]
fn drop_schema_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(DROP_KW) && p.nth_at(1, SCHEMA_KW));
    let m = p.start();
    p.bump(DROP_KW);
    p.bump(SCHEMA_KW);
    opt_if_exists(p);
    let mut found_name = false;
    while !p.at(EOF) {
        name(p);
        found_name = true;
        if !p.eat(COMMA) {
            break;
        }
    }
    if !found_name {
        p.error("expected name");
    }
    opt_cascade_or_restrict(p);
    m.complete(p, DROP_SCHEMA_STMT)
}

fn opt_schema_auth(p: &mut Parser<'_>) -> bool {
    if p.eat(AUTHORIZATION_KW) {
        if !(p.eat(CURRENT_ROLE_KW) || p.eat(CURRENT_USER_KW) || p.eat(SESSION_USER_KW)) {
            if p.at_ts(UNRESERVED_KEYWORDS) || p.at(IDENT) {
                p.bump_any();
                return true;
            } else {
                p.error("expected user_name");
            }
        } else {
            return true;
        }
    }
    false
}

// An SQL statement defining an object to be created within the schema.
//
// Currently, only CREATE TABLE, CREATE VIEW, CREATE INDEX, CREATE SEQUENCE,
// CREATE TRIGGER and GRANT are accepted as clauses within CREATE SCHEMA. Other
// kinds of objects may be created in separate commands after the schema is
// created.
fn opt_schema_elements(p: &mut Parser<'_>) {
    while !p.at(EOF) {
        match (p.current(), p.nth(1)) {
            (CREATE_KW, TABLE_KW) => {
                create_table_stmt(p);
            }
            (CREATE_KW, VIEW_KW) => {
                create_view_stmt(p);
                return;
            }
            (CREATE_KW, SEQUENCE_KW) => {
                create_sequence_stmt(p);
                return;
            }
            (CREATE_KW, TRIGGER_KW) => {
                create_trigger_stmt(p);
                return;
            }
            _ => return,
        };
    }
}

// CREATE SCHEMA schema_name [ AUTHORIZATION role_specification ] [ schema_element [ ... ] ]
// CREATE SCHEMA AUTHORIZATION role_specification [ schema_element [ ... ] ]
// CREATE SCHEMA IF NOT EXISTS schema_name [ AUTHORIZATION role_specification ]
// CREATE SCHEMA IF NOT EXISTS AUTHORIZATION role_specification
// where role_specification can be:
//   | user_name
//   | CURRENT_ROLE
//   | CURRENT_USER
//   | SESSION_USER
// https://www.postgresql.org/docs/17/sql-createschema.html
fn create_schema_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(CREATE_KW) && p.nth_at(1, SCHEMA_KW));
    let m = p.start();
    p.bump(CREATE_KW);
    p.bump(SCHEMA_KW);
    let if_not_exists = opt_if_not_exists(p).is_some();
    match (if_not_exists, opt_schema_auth(p)) {
        // CREATE SCHEMA IF NOT EXISTS AUTHORIZATION role_specification
        //                                                             ^
        (true, true) => m.complete(p, CREATE_SCHEMA_STMT),
        // CREATE SCHEMA IF NOT EXISTS schema_name [ AUTHORIZATION role_specification ]
        //                             ^
        (true, false) => {
            name(p);
            opt_schema_auth(p);
            m.complete(p, CREATE_SCHEMA_STMT)
        }
        // CREATE SCHEMA AUTHORIZATION role_specification [ schema_element [ ... ] ]
        //                                                ^
        (false, true) => {
            opt_schema_elements(p);
            m.complete(p, CREATE_SCHEMA_STMT)
        }
        // CREATE SCHEMA schema_name [ AUTHORIZATION role_specification ] [ schema_element [ ... ] ]
        //               ^
        (false, false) => {
            name(p);
            opt_schema_auth(p);
            opt_schema_elements(p);
            m.complete(p, CREATE_SCHEMA_STMT)
        }
    }
}

fn query(p: &mut Parser<'_>) {
    // TODO: this needs to be more general
    if !p.at_ts(SELECT_FIRST) || select_stmt(p, None).is_none() {
        p.error("expected select stmt")
    }
}

// https://www.postgresql.org/docs/17/sql-insert.html
// [ WITH [ RECURSIVE ] with_query [, ...] ]
// INSERT INTO table_name [ AS alias ] [ ( column_name [, ...] ) ]
//     [ OVERRIDING { SYSTEM | USER } VALUE ]
//     { DEFAULT VALUES | VALUES ( { expression | DEFAULT } [, ...] ) [, ...] | query }
//     [ ON CONFLICT [ conflict_target ] conflict_action ]
//     [ RETURNING { * | output_expression [ [ AS ] output_name ] } [, ...] ]
//
// where conflict_target can be one of:
//     ( { index_column_name | ( index_expression ) } [ COLLATE collation ] [ opclass ] [, ...] ) [ WHERE index_predicate ]
//     ON CONSTRAINT constraint_name
//
// and conflict_action is one of:
//     DO NOTHING
//     DO UPDATE SET { column_name = { expression | DEFAULT } |
//                     ( column_name [, ...] ) = [ ROW ] ( { expression | DEFAULT } [, ...] ) |
//                     ( column_name [, ...] ) = ( sub-SELECT )
//                   } [, ...]
//               [ WHERE condition ]
fn insert_stmt(p: &mut Parser<'_>, m: Option<Marker>) -> CompletedMarker {
    assert!(p.at(INSERT_KW));
    let m = m.unwrap_or_else(|| p.start());
    p.bump(INSERT_KW);
    p.expect(INTO_KW);
    path_name_ref(p);
    // [ AS alias ]
    if p.at(AS_KW) {
        let m = p.start();
        p.bump(AS_KW);
        name(p);
        m.complete(p, ALIAS);
    }
    // [ ( column_name [, ...] ) ]
    opt_column_list(p);
    // [ OVERRIDING { SYSTEM | USER } VALUE ]
    if p.eat(OVERRIDING_KW) {
        let _ = p.eat(SYSTEM_KW) || p.expect(USER_KW);
        p.expect(VALUE_KW);
    }
    // { DEFAULT VALUES | VALUES ( { expression | DEFAULT } [, ...] ) [, ...] | query }
    if p.eat(DEFAULT_KW) {
        p.expect(VALUES_KW);
    } else if p.at(VALUES_KW) {
        values_clause(p, None);
    } else {
        query(p);
    }
    // [ ON CONFLICT [ conflict_target ] conflict_action ]
    if p.eat(ON_KW) {
        p.expect(CONFLICT_KW);
        // ON CONSTRAINT constraint_name
        if p.eat(ON_KW) {
            p.expect(CONSTRAINT_KW);
            name_ref(p);
        // ( { index_column_name | ( index_expression ) } [ COLLATE collation ] [ opclass ] [, ...] ) [ WHERE index_predicate ]
        } else if p.eat(L_PAREN) {
            while !p.at(EOF) {
                // ( index_expression )
                if p.eat(L_PAREN) {
                    // TODO: more strict?
                    if expr(p).is_none() {
                        p.error("expected index_expression");
                    }
                    // index_column_name
                }
                name_ref(p);
                opt_collate(p);
                // [ opclass ]
                p.eat(IDENT);
                // [, ...]
                if !p.eat(COMMA) {
                    break;
                }
            }
            p.expect(R_PAREN);
            // [ WHERE index_predicate ]
            // TODO: be more strict?
            opt_where_clause(p);
        }
        // conflict_action is one of:
        //   DO NOTHING
        //   DO UPDATE SET { column_name = { expression | DEFAULT } |
        //                   ( column_name [, ...] ) = [ ROW ] ( { expression | DEFAULT } [, ...] ) |
        //                   ( column_name [, ...] ) = ( sub-SELECT )
        //                 } [, ...]
        //             [ WHERE condition ]
        if p.eat(DO_KW) && !p.eat(NOTHING_KW) {
            p.expect(UPDATE_KW);
            set_clause(p);
            opt_where_clause(p);
        }
    }
    // [ RETURNING { * | output_expression [ [ AS ] output_name ] } [, ...] ]
    opt_returning_clause(p);
    m.complete(p, INSERT_STMT)
}

// SET { column_name = { expression | DEFAULT } |
//       ( column_name [, ...] ) = [ ROW ] ( { expression | DEFAULT } [, ...] ) |
//       ( column_name [, ...] ) = ( sub-SELECT )
//     } [, ...]
fn set_clause(p: &mut Parser<'_>) {
    p.expect(SET_KW);
    // TODO: generalize
    while !p.at(EOF) {
        // ( column_name [, ...] ) = [ ROW ] ( { expression | DEFAULT } [, ...] ) |
        // ( column_name [, ...] ) = ( sub-SELECT )
        if p.eat(L_PAREN) {
            while !p.at(EOF) {
                name_ref(p);
                if !p.eat(COMMA) {
                    break;
                }
            }
            p.expect(R_PAREN);
            p.expect(EQ);
            // [ ROW ] ( { expression | DEFAULT } [, ...] )
            // ( sub-SELECT )
            let found_row = p.eat(ROW_KW);
            if p.eat(L_PAREN) {
                // ( sub-SELECT )
                if p.at(SELECT_KW) && !found_row {
                    if select_stmt(p, None).is_none() {
                        p.error("expected sub-SELECT");
                    }
                } else {
                    // ( { expression | DEFAULT } [, ...] )
                    while !p.at(EOF) {
                        if !p.eat(DEFAULT_KW) && expr(p).is_none() {
                            p.error("expected expression");
                        }
                        if !p.eat(COMMA) {
                            break;
                        }
                    }
                }
                p.expect(R_PAREN);
            }
            // column_name = { expression | DEFAULT }
        } else {
            name_ref(p);
            p.expect(EQ);
            // { expression | DEFAULT }
            if !p.eat(DEFAULT_KW) && expr(p).is_none() {
                p.error("expected expression");
            }
        }
        if !p.eat(COMMA) {
            break;
        }
    }
}

fn opt_as_alias(p: &mut Parser<'_>) -> Option<CompletedMarker> {
    if p.at(AS_KW) {
        let m = p.start();
        p.bump(AS_KW);
        if p.at_ts(NAME_FIRST) {
            name(p);
        } else {
            p.error("col id")
        }
        Some(m.complete(p, ALIAS))
    } else if p.at_ts(NAME_FIRST) {
        let m = p.start();
        opt_name(p);
        Some(m.complete(p, ALIAS))
    } else {
        None
    }
}

// [ WITH [ RECURSIVE ] with_query [, ...] ]
// UPDATE [ ONLY ] table_name [ * ] [ [ AS ] alias ]
//     SET { column_name = { expression | DEFAULT } |
//           ( column_name [, ...] ) = [ ROW ] ( { expression | DEFAULT } [, ...] ) |
//           ( column_name [, ...] ) = ( sub-SELECT )
//         } [, ...]
//     [ FROM from_item [, ...] ]
//     [ WHERE condition | WHERE CURRENT OF cursor_name ]
//     [ RETURNING { * | output_expression [ [ AS ] output_name ] } [, ...] ]
//
// https://www.postgresql.org/docs/17/sql-update.html
fn update_stmt(p: &mut Parser<'_>, m: Option<Marker>) -> CompletedMarker {
    assert!(p.at(UPDATE_KW));
    let m = m.unwrap_or_else(|| p.start());
    p.bump(UPDATE_KW);
    relation_name(p);
    // postgres parser has the same setup, it assumes the alias can never be
    // named `SET`
    if !p.at(SET_KW) {
        // [ [ AS ] alias ]
        opt_as_alias(p);
    }
    set_clause(p);
    // [ FROM from_item [, ...] ]
    opt_from_clause(p);
    // [ WHERE condition | WHERE CURRENT OF cursor_name ]
    if p.at(WHERE_KW) {
        if p.nth_at(1, CURRENT_KW) {
            opt_where_current_of(p);
        } else {
            opt_where_clause(p);
        }
    }
    // [ RETURNING { * | output_expression [ [ AS ] output_name ] } [, ...] ]
    opt_returning_clause(p);
    m.complete(p, UPDATE_STMT)
}

fn with_stmt(p: &mut Parser<'_>, m: Option<Marker>) -> Option<CompletedMarker> {
    let m = m.unwrap_or_else(|| p.start());
    // with aka cte
    // [ WITH [ RECURSIVE ] with_query [, ...] ]
    with_query_clause(p);
    match p.current() {
        DELETE_KW => Some(delete_stmt(p, Some(m))),
        SELECT_KW | TABLE_KW => select_stmt(p, Some(m)),
        INSERT_KW => Some(insert_stmt(p, Some(m))),
        UPDATE_KW => Some(update_stmt(p, Some(m))),
        MERGE_KW => Some(merge_stmt(p, Some(m))),
        _ => {
            m.abandon(p);
            p.error(format!(
                "expected DELETE, SELECT, TABLE, UPDATE, or MERGE, got: {:?}",
                p.current()
            ));
            None
        }
    }
}

// [ WITH [ RECURSIVE ] with_query [, ...] ]
// DELETE FROM [ ONLY ] table_name [ * ] [ [ AS ] alias ]
//     [ USING from_item [, ...] ]
//     [ WHERE condition | WHERE CURRENT OF cursor_name ]
//     [ RETURNING { * | output_expression [ [ AS ] output_name ] } [, ...] ]
fn delete_stmt(p: &mut Parser<'_>, m: Option<Marker>) -> CompletedMarker {
    assert!(p.at(DELETE_KW));
    let m = m.unwrap_or_else(|| p.start());
    p.bump(DELETE_KW);
    p.expect(FROM_KW);
    relation_name(p);
    if p.eat(AS_KW) {
        p.expect(IDENT);
    } else {
        p.eat(IDENT);
    }
    {
        let m = p.start();
        if p.eat(USING_KW) {
            while p.at_ts(FROM_ITEM_FIRST) {
                if !opt_from_item(p) || !p.eat(COMMA) {
                    break;
                }
            }
            m.complete(p, USING_CLAUSE);
        } else {
            m.abandon(p);
        }
    }
    // [ WHERE condition | WHERE CURRENT OF cursor_name ]
    if p.at(WHERE_KW) {
        if p.nth_at(1, CURRENT_KW) {
            opt_where_current_of(p);
        } else {
            opt_where_clause(p);
        }
    }
    opt_returning_clause(p);
    m.complete(p, DELETE_STMT)
}

// WHERE CURRENT OF cursor_name
fn opt_where_current_of(p: &mut Parser<'_>) {
    if p.eat(WHERE_KW) {
        if p.eat(CURRENT_KW) {
            p.expect(OF_KW);
            p.expect(IDENT);
        }
    }
}

fn opt_returning_clause(p: &mut Parser<'_>) {
    if p.eat(RETURNING_KW) {
        while !p.at(EOF) {
            if !p.eat(STAR) {
                if expr(p).is_none() {
                    p.error("expected output expression");
                } else if p.eat(AS_KW) {
                    p.expect(IDENT);
                } else {
                    p.eat(IDENT);
                }
            }
            if !p.eat(COMMA) {
                break;
            }
        }
    }
}

// DROP TYPE [ IF EXISTS ] name [, ...] [ CASCADE | RESTRICT ]
// https://www.postgresql.org/docs/17/sql-droptype.html
fn drop_type_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(DROP_KW));
    let m = p.start();
    p.bump(DROP_KW);
    p.bump(TYPE_KW);
    opt_if_exists(p);
    // name [, ...]
    while !p.at(EOF) {
        path_name_ref(p);
        if !p.eat(COMMA) {
            break;
        }
    }
    opt_cascade_or_restrict(p);
    m.complete(p, DROP_TYPE_STMT)
}

// DROP TRIGGER [ IF EXISTS ] name ON table_name [ CASCADE | RESTRICT ]
//
// https://www.postgresql.org/docs/17/sql-droptrigger.html
fn drop_trigger_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(DROP_KW));
    let m = p.start();
    p.bump(DROP_KW);
    p.expect(TRIGGER_KW);
    opt_if_exists(p);
    // name
    path_name_ref(p);
    p.expect(ON_KW);
    // table_name
    path_name_ref(p);
    opt_cascade_or_restrict(p);
    m.complete(p, DROP_TRIGGER_STMT)
}

// DROP INDEX [ CONCURRENTLY ] [ IF EXISTS ] name [, ...] [ CASCADE | RESTRICT ]
//
// https://www.postgresql.org/docs/17/sql-dropindex.html
fn drop_index_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(DROP_KW));
    let m = p.start();
    // DROP INDEX
    p.bump(DROP_KW);
    p.expect(INDEX_KW);
    p.eat(CONCURRENTLY_KW);
    opt_if_exists(p);
    // name [, ...]
    while !p.at(EOF) {
        path_name_ref(p);
        if !p.eat(COMMA) {
            break;
        }
    }
    opt_cascade_or_restrict(p);
    m.complete(p, DROP_INDEX_STMT)
}

// DROP DATABASE [ IF EXISTS ] name [ [ WITH ] ( option [, ...] ) ]
//
// where option can be:
//
//     FORCE
// https://www.postgresql.org/docs/17/sql-dropdatabase.html
fn drop_database_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(DROP_KW));
    let m = p.start();
    p.bump(DROP_KW);
    p.expect(DATABASE_KW);
    opt_if_exists(p);
    name_ref(p);
    // [ [ WITH ] ( option [, ...] ) ]
    if p.at(L_PAREN) || p.eat(WITH_KW) {
        p.expect(L_PAREN);
        while !p.at(EOF) {
            p.expect(FORCE_KW);
            if !p.eat(COMMA) {
                break;
            }
        }
        p.expect(R_PAREN);
    }
    m.complete(p, DROP_DATABASE_STMT)
}

// CREATE [ UNIQUE ] INDEX [ CONCURRENTLY ] [ [ IF NOT EXISTS ] name ] ON [ ONLY ] table_name [ USING method ]
//   (
//     { column_name | ( expression ) }
//     [ COLLATE collation ]
//     [ opclass [ ( opclass_parameter = value [, ... ] ) ] ]
//     [ ASC | DESC ]
//     [ NULLS { FIRST | LAST } ]
//     [, ...]
//   )
//     [ INCLUDE ( column_name [, ...] ) ]
//     [ NULLS [ NOT ] DISTINCT ]
//     [ WITH ( storage_parameter [= value] [, ... ] ) ]
//     [ TABLESPACE tablespace_name ]
//     [ WHERE predicate ]
fn create_index_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(CREATE_KW));
    let m = p.start();
    p.bump(CREATE_KW);
    p.eat(UNIQUE_KW);
    p.bump(INDEX_KW);
    p.eat(CONCURRENTLY_KW);
    // [ [ IF NOT EXISTS ] name ]
    if opt_if_not_exists(p).is_some() {
        name(p);
    } else if p.at_ts(NAME_FIRST) {
        opt_name(p);
    }
    // ON
    p.expect(ON_KW);
    relation_name(p);
    // [ USING method ]
    if p.eat(USING_KW) {
        name_ref(p);
    }
    // (
    //   { column_name | ( expression ) }
    //   [ COLLATE collation ]
    //   [ opclass ]
    //   [, ... ]
    // )
    // (
    //   { column_name | ( expression ) }
    //   [ COLLATE collation ]
    //   [ opclass [ ( opclass_parameter = value [, ... ] ) ] ]
    //   [ ASC | DESC ]
    //   [ NULLS { FIRST | LAST } ]
    //   [, ...]
    // )
    let param_list = p.start();
    partition_items(p, true);
    param_list.complete(p, INDEX_PARAMS);
    // [ INCLUDE ( column_name [, ...] ) ]
    opt_include_columns(p);
    // [ NULLS [ NOT ] DISTINCT ]
    if p.eat(NULLS_KW) {
        p.eat(NOT_KW);
        p.expect(DISTINCT_KW);
    }
    // [ WITH ( storage_parameter [= value] [, ... ] ) ]
    opt_with_params(p);
    // [ TABLESPACE tablespace_name ]
    if p.eat(TABLESPACE_KW) {
        name_ref(p);
    }
    // [ WHERE predicate ]
    opt_where_clause(p);
    m.complete(p, CREATE_INDEX_STMT)
}

// (
//   { column_name | ( expression ) }
//   [ COLLATE collation ]
//   [ opclass ]
//   [, ... ]
// )
//
// if we pass allow_extra_params:
// (
//   { column_name | ( expression ) }
//   [ COLLATE collation ]
//   [ opclass [ ( opclass_parameter = value [, ... ] ) ] ]
//   [ ASC | DESC ]
//   [ NULLS { FIRST | LAST } ]
//   [, ...]
// )
fn partition_items(p: &mut Parser<'_>, allow_extra_params: bool) {
    p.expect(L_PAREN);
    while !p.at(EOF) && !p.at(R_PAREN) {
        if !part_elem(p, allow_extra_params) {
            break;
        }
        if !p.eat(COMMA) {
            break;
        }
    }
    p.expect(R_PAREN);
}

// [ argmode ]
fn opt_param_mode(p: &mut Parser<'_>) -> Option<CompletedMarker> {
    let m = p.start();
    let mode = match p.current() {
        VARIADIC_KW => {
            p.bump(VARIADIC_KW);
            PARAM_VARIADIC
        }
        IN_KW => {
            p.bump(IN_KW);
            if p.eat(OUT_KW) {
                PARAM_INOUT
            } else {
                PARAM_IN
            }
        }
        OUT_KW => {
            p.bump(OUT_KW);
            PARAM_OUT
        }
        INOUT_KW => {
            p.bump(INOUT_KW);
            PARAM_INOUT
        }
        _ => {
            m.abandon(p);
            return None;
        }
    };
    Some(m.complete(p, mode))
}

fn opt_param_default(p: &mut Parser<'_>) -> Option<CompletedMarker> {
    if p.at(DEFAULT_KW) || p.at(EQ) {
        let m = p.start();
        p.bump_any();
        if expr(p).is_none() {
            p.error("expected default expr for func arg");
        }
        Some(m.complete(p, PARAM_DEFAULT))
    } else {
        None
    }
}

/// see: <https://github.com/postgres/postgres/blob/29dfffae0a6db6cb880ae873169f5512ddab703d/src/backend/parser/gram.y#L18049>
const TYPE_FUNC_NAME_KEYWORDS: TokenSet = TokenSet::new(&[
    AUTHORIZATION_KW,
    BINARY_KW,
    COLLATION_KW,
    CONCURRENTLY_KW,
    CROSS_KW,
    CURRENT_SCHEMA_KW,
    FREEZE_KW,
    FULL_KW,
    ILIKE_KW,
    INNER_KW,
    IS_KW,
    ISNULL_KW,
    JOIN_KW,
    LEFT_KW,
    LIKE_KW,
    NATURAL_KW,
    NOTNULL_KW,
    OUTER_KW,
    OVERLAPS_KW,
    RIGHT_KW,
    SIMILAR_KW,
    TABLESAMPLE_KW,
    VERBOSE_KW,
]);

fn opt_param_name(p: &mut Parser<'_>) -> bool {
    if p.at(IDENT) || p.at_ts(UNRESERVED_KEYWORDS) || p.at_ts(TYPE_FUNC_NAME_KEYWORDS) {
        p.bump_any();
        true
    } else {
        false
    }
}

// [ argmode ] [ argname ] argtype [ { DEFAULT | = } default_expr ]
//
// func_arg:
//   | mode name type
//   | mode type
//   | name mode type
//   | name type
//   | type
fn param(p: &mut Parser<'_>) {
    let m = p.start();
    // [ argmode ]
    let param_mode_seen = opt_param_mode(p).is_some();
    let name_or_type = p.start();
    // [ argname ]
    let maybe_name = opt_param_name(p);
    if maybe_name {
        // Could have either parsed a name or a type, we know if it it's a type if:
        let at_type = match p.current() {
            // foo.bar%type
            //    ^
            DOT => true,
            // foo(8)
            //    ^
            L_PAREN => true,
            // float8 order by
            //        ^
            ORDER_KW => true,
            // we're at the end of the param, must be a type
            R_PAREN | EQ | DEFAULT_KW | COMMA => true,
            _ => false,
        };
        if at_type {
            name_or_type.complete(p, PATH_TYPE);
        } else {
            name_or_type.complete(p, NAME);
            if !param_mode_seen {
                opt_param_mode(p);
            }
            // argtype
            type_name(p);
        }
    } else {
        name_or_type.abandon(p);
        // argtype
        type_name(p);
    }
    opt_param_default(p);
    m.complete(p, PARAM);
}

// { LANGUAGE lang_name
//   | TRANSFORM { FOR TYPE type_name } [, ... ]
//   | WINDOW
//   | { IMMUTABLE | STABLE | VOLATILE }
//   | [ NOT ] LEAKPROOF
//   | { CALLED ON NULL INPUT | RETURNS NULL ON NULL INPUT | STRICT }
//   | { [ EXTERNAL ] SECURITY INVOKER | [ EXTERNAL ] SECURITY DEFINER }
//   | PARALLEL { UNSAFE | RESTRICTED | SAFE }
//   | COST execution_cost
//   | ROWS result_rows
//   | SUPPORT support_function
//   | SET configuration_parameter { TO value | = value | FROM CURRENT }
//   | AS 'definition'
//   | AS 'obj_file', 'link_symbol'
//   | sql_body
// } ...
//
//   sql_body:
//   | RETURN expression
//   | BEGIN ATOMIC
//       statement;
//       statement;
//       ...
//       statement;
//     END
fn opt_function_option(p: &mut Parser<'_>) -> bool {
    let m = p.start();
    let kind = match p.current() {
        // LANGUAGE lang_name
        LANGUAGE_KW => {
            p.bump(LANGUAGE_KW);
            // string for language is deprecated but let's support it
            if opt_string_literal(p).is_none() {
                if p.at_ts(UNRESERVED_KEYWORDS) || p.at(IDENT) {
                    p.bump_any();
                } else {
                    p.error(format!("expected a language name, got {:?}", p.current()));
                }
            }
            LANGUAGE_FUNC_OPTION
        }
        // TRANSFORM { FOR TYPE type_name } [, ... ]
        TRANSFORM_KW => {
            p.bump(TRANSFORM_KW);
            while !p.at(EOF) {
                p.expect(FOR_KW);
                p.expect(TYPE_KW);
                type_name(p);
                if !p.eat(COMMA) {
                    break;
                }
            }
            TRANSFORM_FUNC_OPTION
        }
        // WINDOW
        WINDOW_KW => {
            p.bump(WINDOW_KW);
            WINDOW_FUNC_OPTION
        }
        // { IMMUTABLE | STABLE | VOLATILE }
        IMMUTABLE_KW | STABLE_KW | VOLATILE_KW => {
            p.bump_any();
            VOLATILITY_FUNC_OPTION
        }
        // [ NOT ] LEAKPROOF
        NOT_KW | LEAKPROOF_KW => {
            p.eat(NOT_KW);
            p.expect(LEAKPROOF_KW);
            LEAKPROOF_FUNC_OPTION
        }
        // RESET configuration_parameter
        // RESET ALL
        RESET_KW => {
            p.bump(RESET_KW);
            if !p.eat(ALL_KW) {
                path_name_ref(p);
            }
            RESET_FUNC_OPTION
        }
        // { CALLED ON NULL INPUT | RETURNS NULL ON NULL INPUT | STRICT }
        CALLED_KW | RETURNS_KW | STRICT_KW => {
            if p.eat(CALLED_KW) {
                p.expect(ON_KW);
                p.expect(NULL_KW);
                p.expect(INPUT_KW);
            } else if p.eat(RETURNS_KW) {
                p.expect(NULL_KW);
                p.expect(ON_KW);
                p.expect(NULL_KW);
                p.expect(INPUT_KW);
            } else {
                p.expect(STRICT_KW);
            }
            STRICT_FUNC_OPTION
        }
        // { [ EXTERNAL ] SECURITY INVOKER | [ EXTERNAL ] SECURITY DEFINER }
        EXTERNAL_KW | SECURITY_KW => {
            p.eat(EXTERNAL_KW);
            p.expect(SECURITY_KW);
            let _ = p.eat(INVOKER_KW) || p.expect(DEFINER_KW);
            SECURITY_FUNC_OPTION
        }
        // PARALLEL { UNSAFE | RESTRICTED | SAFE }
        PARALLEL_KW => {
            p.bump(PARALLEL_KW);
            p.expect(IDENT);
            PARALLEL_FUNC_OPTION
        }
        // COST execution_cost
        COST_KW => {
            p.bump(COST_KW);
            if opt_numeric_literal(p).is_none() {
                p.error("expected numeric value for execution_cost");
            }
            COST_FUNC_OPTION
        }
        // ROWS result_rows
        ROWS_KW => {
            p.bump(ROWS_KW);
            if opt_numeric_literal(p).is_none() {
                p.error("expected numeric value for result_rows");
            }
            ROWS_FUNC_OPTION
        }
        // SUPPORT support_function
        SUPPORT_KW => {
            p.bump(SUPPORT_KW);
            path_name_ref(p);
            SUPPORT_FUNC_OPTION
        }
        // SET configuration_parameter { TO value | = value | FROM CURRENT }
        SET_KW => {
            set_configuration_param(p);
            SET_FUNC_OPTION
        }
        // AS 'definition'
        // AS 'obj_file', 'link_symbol'
        AS_KW => {
            p.bump(AS_KW);
            string_literal(p);
            if p.eat(COMMA) {
                string_literal(p);
            }
            AS_FUNC_OPTION
        }
        // RETURN expression
        RETURN_KW => {
            p.bump(RETURN_KW);
            if expr(p).is_none() {
                p.error("expected expression for return");
            }
            RETURN_FUNC_OPTION
        }
        // BEGIN ATOMIC
        //   statement;
        //   statement;
        //   ...
        //   statement;
        // END
        BEGIN_KW => {
            p.bump(BEGIN_KW);
            p.expect(ATOMIC_KW);
            while !p.at(EOF) && !p.at(END_KW) {
                if p.eat(SEMICOLON) {
                    continue;
                }
                stmt(
                    p,
                    &StmtRestrictions {
                        begin_end_allowed: false,
                    },
                );
                if p.at(END_KW) {
                    p.expect(SEMICOLON);
                }
            }
            p.expect(END_KW);
            BEGIN_FUNC_OPTION
        }
        _ => {
            m.abandon(p);
            return false;
        }
    };
    m.complete(p, kind);
    true
}

// SET configuration_parameter { TO | = } { value | DEFAULT }
// SET configuration_parameter FROM CURRENT
fn set_configuration_param(p: &mut Parser<'_>) {
    assert!(p.at(SET_KW));
    p.bump(SET_KW);
    // configuration_parameter
    path_name_ref(p);
    // { TO value | = value | FROM CURRENT }
    if p.eat(FROM_KW) {
        p.expect(CURRENT_KW);
    } else if (p.eat(TO_KW) || p.expect(EQ)) && !config_value(p) {
        p.error(format!("expected config value, got {:?}", p.current()));
    }
}

fn opt_ret_type(p: &mut Parser<'_>) {
    // [ RETURNS rettype
    //       | RETURNS TABLE ( column_name column_type [, ...] ) ]
    let m = p.start();
    if p.eat(RETURNS_KW) {
        if p.eat(TABLE_KW) {
            p.expect(L_PAREN);
            while !p.at(EOF) {
                // column_name
                name_ref(p);
                // column_type
                type_name(p);
                if !p.eat(COMMA) {
                    break;
                }
            }
            p.expect(R_PAREN);
        } else {
            p.eat(SETOF_KW);
            type_name(p);
        }
        m.complete(p, RET_TYPE);
    } else {
        m.abandon(p);
    }
}

fn func_option_list(p: &mut Parser<'_>) {
    let m = p.start();
    let mut seen_func_option = false;
    while !p.at(EOF) {
        if !opt_function_option(p) {
            if !seen_func_option {
                p.error("expected function option");
            }
            break;
        } else {
            seen_func_option = true
        }
    }
    if !seen_func_option {
        m.abandon(p);
        return;
    }
    m.complete(p, FUNC_OPTION_LIST);
}

// [ ( [ [ argmode ] [ argname ] argtype [ { DEFAULT | = } default_expr ] [, ...] ] ) ]
fn opt_param_list(p: &mut Parser<'_>) -> bool {
    if !p.at(L_PAREN) {
        return false;
    }
    param_list(p);
    true
}

fn param_list(p: &mut Parser<'_>) {
    let m = p.start();
    // ( [ [ argmode ] [ argname ] argtype [ { DEFAULT | = } default_expr ] [, ...] ] )
    if !p.expect(L_PAREN) {
        m.abandon(p);
        return;
    }
    while !p.at(EOF) && !p.at(R_PAREN) {
        param(p);
        if !p.eat(COMMA) {
            break;
        }
    }
    p.expect(R_PAREN);
    m.complete(p, PARAM_LIST);
}

// CREATE [ OR REPLACE ] FUNCTION
//     name ( [ [ argmode ] [ argname ] argtype [ { DEFAULT | = } default_expr ] [, ...] ] )
//     [ RETURNS rettype
//       | RETURNS TABLE ( column_name column_type [, ...] ) ]
//   { LANGUAGE lang_name
//     | TRANSFORM { FOR TYPE type_name } [, ... ]
//     | WINDOW
//     | { IMMUTABLE | STABLE | VOLATILE }
//     | [ NOT ] LEAKPROOF
//     | { CALLED ON NULL INPUT | RETURNS NULL ON NULL INPUT | STRICT }
//     | { [ EXTERNAL ] SECURITY INVOKER | [ EXTERNAL ] SECURITY DEFINER }
//     | PARALLEL { UNSAFE | RESTRICTED | SAFE }
//     | COST execution_cost
//     | ROWS result_rows
//     | SUPPORT support_function
//     | SET configuration_parameter { TO value | = value | FROM CURRENT }
//     | AS 'definition'
//     | AS 'obj_file', 'link_symbol'
//     | sql_body
//   } ...
// https://www.postgresql.org/docs/17/sql-createfunction.html
fn create_function_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(CREATE_KW));
    let m = p.start();
    // CREATE
    p.bump(CREATE_KW);
    opt_or_replace(p);
    p.expect(FUNCTION_KW);
    // name
    path_name(p);
    param_list(p);
    opt_ret_type(p);
    func_option_list(p);
    m.complete(p, CREATE_FUNCTION_STMT)
}

fn opt_or_replace(p: &mut Parser<'_>) -> Option<CompletedMarker> {
    // [ OR REPLACE ]
    if p.at(OR_KW) {
        let m = p.start();
        p.expect(OR_KW);
        p.expect(REPLACE_KW);
        Some(m.complete(p, OR_REPLACE))
    } else {
        None
    }
}

// CREATE TYPE name AS
//     ( [ attribute_name data_type [ COLLATE collation ] [, ... ] ] )
//
// CREATE TYPE name AS ENUM
//     ( [ 'label' [, ... ] ] )
//
// CREATE TYPE name AS RANGE (
//     SUBTYPE = subtype
//     [ , SUBTYPE_OPCLASS = subtype_operator_class ]
//     [ , COLLATION = collation ]
//     [ , CANONICAL = canonical_function ]
//     [ , SUBTYPE_DIFF = subtype_diff_function ]
//     [ , MULTIRANGE_TYPE_NAME = multirange_type_name ]
// )
//
// CREATE TYPE name (
//     INPUT = input_function,
//     OUTPUT = output_function
//     [ , RECEIVE = receive_function ]
//     [ , SEND = send_function ]
//     [ , TYPMOD_IN = type_modifier_input_function ]
//     [ , TYPMOD_OUT = type_modifier_output_function ]
//     [ , ANALYZE = analyze_function ]
//     [ , SUBSCRIPT = subscript_function ]
//     [ , INTERNALLENGTH = { internallength | VARIABLE } ]
//     [ , PASSEDBYVALUE ]
//     [ , ALIGNMENT = alignment ]
//     [ , STORAGE = storage ]
//     [ , LIKE = like_type ]
//     [ , CATEGORY = category ]
//     [ , PREFERRED = preferred ]
//     [ , DEFAULT = default ]
//     [ , ELEMENT = element ]
//     [ , DELIMITER = delimiter ]
//     [ , COLLATABLE = collatable ]
// )
//
// CREATE TYPE name
// https://www.postgresql.org/docs/17/sql-createtype.html
fn create_type_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(CREATE_KW));
    let m = p.start();
    p.bump(CREATE_KW);
    p.bump(TYPE_KW);
    path_name(p);
    if p.eat(AS_KW) {
        // AS ENUM
        if p.eat(ENUM_KW) {
            p.expect(L_PAREN);
            while !p.at(EOF) {
                if opt_string_literal(p).is_none() {
                    break;
                }
                if !p.eat(COMMA) {
                    break;
                }
            }
            p.expect(R_PAREN);
            // AS RANGE
        } else if p.eat(RANGE_KW) {
            p.expect(L_PAREN);
            while !p.at(EOF) {
                if !attribute_option(p, AttributeValue::Required) {
                    break;
                }
                if !p.eat(COMMA) {
                    break;
                }
            }
            p.expect(R_PAREN);
            // AS
        } else {
            p.expect(L_PAREN);
            while !p.at(EOF) && !p.at(R_PAREN) {
                // attribute_name
                name_ref(p);
                // data_type
                type_name(p);
                opt_collate(p);
                if !p.eat(COMMA) {
                    break;
                }
            }
            p.expect(R_PAREN);
        }
    } else if p.eat(L_PAREN) {
        while !p.at(EOF) {
            if !attribute_option(p, AttributeValue::Either) {
                break;
            }
            if !p.eat(COMMA) {
                break;
            }
        }
        p.expect(R_PAREN);
    }
    m.complete(p, CREATE_TYPE_STMT)
}

// CREATE EXTENSION [ IF NOT EXISTS ] extension_name
//     [ WITH ] [ SCHEMA schema_name ]
//              [ VERSION version ]
//              [ CASCADE ]
// https://www.postgresql.org/docs/17/sql-createextension.html
fn create_extension_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(CREATE_KW));
    let m = p.start();
    p.bump(CREATE_KW);
    p.bump(EXTENSION_KW);
    opt_if_not_exists(p);
    // extension_name
    name(p);
    p.eat(WITH_KW);
    if p.eat(SCHEMA_KW) {
        p.expect(IDENT);
    }
    if p.eat(VERSION_KW) {
        if opt_string_literal(p).is_none() && !p.eat(IDENT) {
            p.error("expected string literal or IDENT");
        }
    }
    p.eat(CASCADE_KW);
    m.complete(p, CREATE_EXTENSION_STMT)
}

// { value | 'value' | DEFAULT }
// where value can be specified as string constants, identifiers, numbers, or
// comma-separated lists of these
fn config_value(p: &mut Parser<'_>) -> bool {
    if p.eat(DEFAULT_KW) {
        return true;
    }
    let mut found_value = false;
    // ident, number or comma separated list of strings, idents, numbers
    while !p.at(EOF) {
        if opt_string_literal(p).is_none()
            && opt_numeric_literal(p).is_none()
            && !p.eat(IDENT)
            && !opt_bool_literal(p)
        {
            if p.at_ts(BARE_LABEL_KEYWORDS) {
                p.bump_any();
            } else {
                break;
            }
        }
        found_value = true;
        if !p.eat(COMMA) {
            break;
        }
    }
    found_value
}

// SET [ SESSION | LOCAL ] configuration_parameter { TO | = } { value | 'value' | DEFAULT }
// SET [ SESSION | LOCAL ] TIME ZONE { value | 'value' | LOCAL | DEFAULT }
//
// https://www.postgresql.org/docs/17/sql-set.html
fn set_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(SET_KW));
    let m = p.start();
    p.bump(SET_KW);
    let _ = p.eat(SESSION_KW) || p.eat(LOCAL_KW);
    // TIME ZONE { value | 'value' | LOCAL | DEFAULT }
    if p.eat(TIME_KW) {
        p.expect(ZONE_KW);
        if !p.eat(LOCAL_KW) && !config_value(p) {
            p.error(format!("expected config value, got {:?}", p.current()));
        }
    // configuration_parameter { TO | = } { value | 'value' | DEFAULT }
    } else {
        // configuration_parameter
        name_ref(p);
        // { TO | = }
        let _ = p.eat(TO_KW) || p.expect(EQ);
        // { value | 'value' | DEFAULT }
        if !config_value(p) {
            p.error(format!("expected config value, got {:?}", p.current()));
        }
    }
    m.complete(p, SET_STMT)
}

// SHOW name
// SHOW ALL
//
// https://www.postgresql.org/docs/17/sql-show.html
fn show_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(SHOW_KW));
    let m = p.start();
    p.bump(SHOW_KW);
    if !p.eat(ALL_KW) {
        name_ref(p);
    }
    m.complete(p, SHOW_STMT)
}

const COLUMN_FIRST: TokenSet = TokenSet::new(&[IDENT])
    .union(UNRESERVED_KEYWORDS)
    .union(COLUMN_OR_TABLE_KEYWORDS);

const NON_RESERVED_WORD: TokenSet = TokenSet::new(&[IDENT])
    .union(UNRESERVED_KEYWORDS)
    .union(COLUMN_NAME_KEYWORDS)
    .union(TYPE_FUNC_NAME_KEYWORDS);

fn relation_name(p: &mut Parser<'_>) {
    // [ ONLY ]
    if p.eat(ONLY_KW) {
        let trailing_paren = p.eat(L_PAREN);
        // name
        path_name_ref(p);
        if trailing_paren {
            p.expect(R_PAREN);
        }
    } else {
        path_name_ref(p);
        p.eat(STAR);
    }
}

// ALTER TABLE [ IF EXISTS ] [ ONLY ] name [ * ]
//     RENAME [ COLUMN ] column_name TO new_column_name
// ALTER TABLE [ IF EXISTS ] [ ONLY ] name [ * ]
//     RENAME CONSTRAINT constraint_name TO new_constraint_name
// ALTER TABLE [ IF EXISTS ] name
//     RENAME TO new_name
//
// ALTER TABLE [ IF EXISTS ] name
//     SET SCHEMA new_schema
// ALTER TABLE ALL IN TABLESPACE name [ OWNED BY role_name [, ... ] ]
//     SET TABLESPACE new_tablespace [ NOWAIT ]
//
// ALTER TABLE [ IF EXISTS ] name
//     ATTACH PARTITION partition_name { FOR VALUES partition_bound_spec | DEFAULT }
// ALTER TABLE [ IF EXISTS ] name
//     DETACH PARTITION partition_name [ CONCURRENTLY | FINALIZE ]
//
// ALTER TABLE [ IF EXISTS ] [ ONLY ] name [ * ]
//     action [, ... ]
//
// where action is one of:
//
//    ADD [ COLUMN ] [ IF NOT EXISTS ] column_name data_type [ COLLATE collation ] [ column_constraint [ ... ] ]
//    DROP [ COLUMN ] [ IF EXISTS ] column_name [ RESTRICT | CASCADE ]
//    ALTER [ COLUMN ] column_name [ SET DATA ] TYPE data_type [ COLLATE collation ] [ USING expression ]
//    ALTER [ COLUMN ] column_name SET DEFAULT expression
//    ALTER [ COLUMN ] column_name DROP DEFAULT
//    ALTER [ COLUMN ] column_name { SET | DROP } NOT NULL
//    ALTER [ COLUMN ] column_name SET EXPRESSION AS ( expression )
//    ALTER [ COLUMN ] column_name DROP EXPRESSION [ IF EXISTS ]
//    ALTER [ COLUMN ] column_name ADD GENERATED { ALWAYS | BY DEFAULT } AS IDENTITY [ ( sequence_options ) ]
//    ALTER [ COLUMN ] column_name { SET GENERATED { ALWAYS | BY DEFAULT } | SET sequence_option | RESTART [ [ WITH ] restart ] } [...]
//    ALTER [ COLUMN ] column_name DROP IDENTITY [ IF EXISTS ]
//    ALTER [ COLUMN ] column_name SET STATISTICS { integer | DEFAULT }
//    ALTER [ COLUMN ] column_name SET ( attribute_option = value [, ... ] )
//    ALTER [ COLUMN ] column_name RESET ( attribute_option [, ... ] )
//    ALTER [ COLUMN ] column_name SET STORAGE { PLAIN | EXTERNAL | EXTENDED | MAIN | DEFAULT }
//    ALTER [ COLUMN ] column_name SET COMPRESSION compression_method
//    ADD table_constraint [ NOT VALID ]
//    ADD table_constraint_using_index
//    ALTER CONSTRAINT constraint_name [ DEFERRABLE | NOT DEFERRABLE ] [ INITIALLY DEFERRED | INITIALLY IMMEDIATE ]
//    VALIDATE CONSTRAINT constraint_name
//    DROP CONSTRAINT [ IF EXISTS ]  constraint_name [ RESTRICT | CASCADE ]
//    DISABLE TRIGGER [ trigger_name | ALL | USER ]
//    ENABLE TRIGGER [ trigger_name | ALL | USER ]
//    ENABLE REPLICA TRIGGER trigger_name
//    ENABLE ALWAYS TRIGGER trigger_name
//    DISABLE RULE rewrite_rule_name
//    ENABLE RULE rewrite_rule_name
//    ENABLE REPLICA RULE rewrite_rule_name
//    ENABLE ALWAYS RULE rewrite_rule_name
//    DISABLE ROW LEVEL SECURITY
//    ENABLE ROW LEVEL SECURITY
//    FORCE ROW LEVEL SECURITY
//    NO FORCE ROW LEVEL SECURITY
//    CLUSTER ON index_name
//    SET WITHOUT CLUSTER
//    SET WITHOUT OIDS
//    SET ACCESS METHOD { new_access_method | DEFAULT }
//    SET TABLESPACE new_tablespace
//    SET { LOGGED | UNLOGGED }
//    SET ( storage_parameter [= value] [, ... ] )
//    RESET ( storage_parameter [, ... ] )
//    INHERIT parent_table
//    NO INHERIT parent_table
//    OF type_name
//    NOT OF
//    OWNER TO { new_owner | CURRENT_ROLE | CURRENT_USER | SESSION_USER }
//    REPLICA IDENTITY { DEFAULT | USING INDEX index_name | FULL | NOTHING }
fn alter_table_stmt(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(ALTER_KW));
    let m = p.start();
    // ALTER TABLE
    p.bump(ALTER_KW);
    p.expect(TABLE_KW);
    let mut all_in_tablespace = false;
    // [ ALL IN TABLESPACE]
    if p.eat(ALL_KW) {
        p.expect(IN_KW);
        p.expect(TABLESPACE_KW);
        all_in_tablespace = true;
    } else {
        opt_if_exists(p);
    }
    relation_name(p);
    // ALTER TABLE ALL IN TABLESPACE name [ OWNED BY role_name [, ... ] ]
    //     SET TABLESPACE new_tablespace [ NOWAIT ]
    if all_in_tablespace && p.eat(OWNED_KW) {
        p.expect(BY_KW);
        while !p.at(EOF) {
            // name
            name_ref(p);
            if !p.eat(COMMA) {
                break;
            }
        }
    }
    // TODO: we should be robust to missing commas
    while !p.at(EOF) {
        let action = p.start();
        match alter_table_action(p) {
            Some(action_kind) => {
                action.complete(p, action_kind);
            }
            None => {
                action.abandon(p);
            }
        };
        if !p.eat(COMMA) {
            break;
        }
    }
    m.complete(p, ALTER_TABLE)
}

fn alter_table_action(p: &mut Parser<'_>) -> Option<SyntaxKind> {
    let kind = match p.current() {
        // VALIDATE CONSTRAINT constraint_name
        VALIDATE_KW => {
            p.bump(VALIDATE_KW);
            p.expect(CONSTRAINT_KW);
            name_ref(p);
            VALIDATE_CONSTRAINT
        }
        // REPLICA IDENTITY { DEFAULT | USING INDEX index_name | FULL | NOTHING }
        REPLICA_KW => {
            p.bump(REPLICA_KW);
            p.expect(IDENTITY_KW);
            if !p.eat(DEFAULT_KW) && !p.eat(FULL_KW) && !p.eat(NOTHING_KW) {
                p.expect(USING_KW);
                p.expect(INDEX_KW);
                name_ref(p);
            }
            REPLICA_IDENTITY
        }
        // OF type_name
        OF_KW => {
            p.bump(OF_KW);
            simple_type_name(p);
            OF_TYPE
        }
        // NOT OF
        NOT_KW if p.nth_at(1, OF_KW) => {
            p.bump(NOT_KW);
            p.bump(OF_KW);
            NOT_OF
        }
        // FORCE ROW LEVEL SECURITY
        FORCE_KW => {
            p.bump(FORCE_KW);
            p.expect(ROW_KW);
            p.expect(LEVEL_KW);
            p.expect(SECURITY_KW);
            FORCE_RLS
        }
        // NO FORCE ROW LEVEL SECURITY
        NO_KW if p.nth_at(1, FORCE_KW) => {
            p.bump(NO_KW);
            p.bump(FORCE_KW);
            p.expect(ROW_KW);
            p.expect(LEVEL_KW);
            p.expect(SECURITY_KW);
            NO_FORCE_RLS
        }
        // INHERIT parent_table
        INHERIT_KW => {
            p.bump(INHERIT_KW);
            path_name_ref(p);
            INHERIT
        }
        // NO INHERIT parent_table
        NO_KW if p.nth_at(1, INHERIT_KW) => {
            p.bump(NO_KW);
            p.bump(INHERIT_KW);
            path_name_ref(p);
            NO_INHERIT
        }
        // ENABLE TRIGGER [ trigger_name | ALL | USER ]
        // ENABLE REPLICA TRIGGER trigger_name
        // ENABLE REPLICA RULE rewrite_rule_name
        // ENABLE ALWAYS TRIGGER trigger_name
        // ENABLE ALWAYS RULE rewrite_rule_name
        // ENABLE RULE rewrite_rule_name
        // ENABLE ROW LEVEL SECURITY
        ENABLE_KW => {
            p.bump(ENABLE_KW);
            match p.current() {
                TRIGGER_KW => {
                    p.bump(TRIGGER_KW);
                    if !p.eat(ALL_KW) && !p.eat(USER_KW) {
                        name_ref(p);
                    }
                    ENABLE_TRIGGER
                }
                REPLICA_KW => {
                    p.bump(REPLICA_KW);
                    if p.eat(TRIGGER_KW) {
                        name_ref(p);
                        ENABLE_REPLICA_TRIGGER
                    } else {
                        p.expect(RULE_KW);
                        name_ref(p);
                        ENABLE_REPLICA_RULE
                    }
                }
                ALWAYS_KW => {
                    p.bump(ALWAYS_KW);
                    if p.eat(TRIGGER_KW) {
                        name_ref(p);
                        ENABLE_ALWAYS_TRIGGER
                    } else {
                        p.expect(RULE_KW);
                        name_ref(p);
                        ENABLE_ALWAYS_RULE
                    }
                }
                RULE_KW => {
                    p.bump(RULE_KW);
                    name_ref(p);
                    ENABLE_RULE
                }
                ROW_KW => {
                    p.bump(ROW_KW);
                    p.expect(LEVEL_KW);
                    p.expect(SECURITY_KW);
                    ENABLE_RLS
                }
                _ => {
                    p.error("expected TRIGGER, REPLICA, ALWAYS, RULE, or ROW");
                    // TODO: just picking something for now
                    ENABLE_RLS
                }
            }
        }
        // DISABLE TRIGGER [ trigger_name | ALL | USER ]
        // DISABLE RULE rewrite_rule_name
        // DISABLE ROW LEVEL SECURITY
        DISABLE_KW => {
            p.bump(DISABLE_KW);
            match p.current() {
                TRIGGER_KW => {
                    p.bump(TRIGGER_KW);
                    if !p.eat(ALL_KW) && !p.eat(USER_KW) {
                        name_ref(p);
                    }
                    DISABLE_TRIGGER
                }
                ROW_KW => {
                    p.bump(ROW_KW);
                    p.expect(LEVEL_KW);
                    p.expect(SECURITY_KW);
                    DISABLE_RLS
                }
                RULE_KW => {
                    p.bump(RULE_KW);
                    name_ref(p);
                    DISABLE_RULE
                }
                _ => {
                    p.error("expected TRIGGER, ROW, or RULE");
                    // TODO: just picking something for now
                    DISABLE_RULE
                }
            }
        }
        // CLUSTER ON index_name
        CLUSTER_KW => {
            p.bump(CLUSTER_KW);
            p.bump(ON_KW);
            name_ref(p);
            DISABLE_CLUSTER
        }
        // OWNER TO { new_owner | CURRENT_ROLE | CURRENT_USER | SESSION_USER }
        OWNER_KW => {
            p.bump(OWNER_KW);
            p.bump(TO_KW);
            if !(p.eat(CURRENT_ROLE_KW) || p.eat(CURRENT_USER_KW) || p.eat(SESSION_USER_KW)) {
                name_ref(p);
            }
            OWNER_TO
        }
        DETACH_KW => {
            p.bump(DETACH_KW);
            p.expect(PARTITION_KW);
            // partition_name
            name_ref(p);
            // [ CONCURRENTLY | FINALIZE ]
            if !p.eat(CONCURRENTLY_KW) {
                p.eat(FINALIZE_KW);
            }
            DETACH_PARTITION
        }
        // DROP [ COLUMN ] [ IF EXISTS ] column_name [ RESTRICT | CASCADE ]
        // DROP CONSTRAINT [ IF EXISTS ] constraint_name [ RESTRICT | CASCADE ]
        DROP_KW => {
            p.bump(DROP_KW);
            // CONSTRAINT [ IF EXISTS ] constraint_name [ RESTRICT | CASCADE ]
            let kind = if p.eat(CONSTRAINT_KW) {
                opt_if_exists(p);
                name_ref(p);
                DROP_CONSTRAINT
            // [ COLUMN ] [ IF EXISTS ] column_name [ RESTRICT | CASCADE ]
            } else {
                p.eat(COLUMN_KW);
                opt_if_exists(p);
                name_ref(p);
                DROP_COLUMN
            };
            opt_cascade_or_restrict(p);
            kind
        }
        // ADD [ COLUMN ] [ IF NOT EXISTS ] column_name data_type [ COLLATE collation ] [ column_constraint [ ... ] ]
        // ADD table_constraint [ NOT VALID ]
        // ADD table_constraint_using_index
        //
        // and table_constraint is:
        //
        //   [ CONSTRAINT constraint_name ]
        //   { CHECK ( expression ) [ NO INHERIT ] |
        //     UNIQUE [ NULLS [ NOT ] DISTINCT ] ( column_name [, ... ] ) index_parameters |
        //     PRIMARY KEY ( column_name [, ... ] ) index_parameters |
        //     EXCLUDE [ USING index_method ] ( exclude_element WITH operator [, ... ] ) index_parameters [ WHERE ( predicate ) ] |
        //     FOREIGN KEY ( column_name [, ... ] ) REFERENCES reftable [ ( refcolumn [, ... ] ) ]
        //       [ MATCH FULL | MATCH PARTIAL | MATCH SIMPLE ] [ ON DELETE referential_action ] [ ON UPDATE referential_action ] }
        //   [ DEFERRABLE | NOT DEFERRABLE ] [ INITIALLY DEFERRED | INITIALLY IMMEDIATE ]
        //
        //
        // and table_constraint_using_index is:
        //
        //   [ CONSTRAINT constraint_name ]
        //   { UNIQUE | PRIMARY KEY } USING INDEX index_name
        //   [ DEFERRABLE | NOT DEFERRABLE ] [ INITIALLY DEFERRED | INITIALLY IMMEDIATE ]
        ADD_KW => {
            p.bump(ADD_KW);
            if p.at_ts(TABLE_CONSTRAINT_FIRST) {
                // at table_constraint or table_constraint_using_index
                table_constraint(p);
                opt_not_valid(p);
                ADD_CONSTRAINT
            } else {
                // [ COLUMN ] [ IF NOT EXISTS ] column_name data_type [ COLLATE collation ] [ column_constraint [ ... ] ]
                // otherwise we're expecting an add column
                p.eat(COLUMN_KW);
                opt_if_not_exists(p);
                // column_name
                name_ref(p);
                type_name(p);
                opt_collate(p);
                // [ column_constraint [ ... ] ]
                while !p.at(EOF) {
                    if opt_column_constraint(p).is_none() {
                        break;
                    }
                }
                ADD_COLUMN
            }
        }
        // ATTACH PARTITION partition_name { FOR VALUES partition_bound_spec | DEFAULT }
        ATTACH_KW => {
            p.bump(ATTACH_KW);
            p.expect(PARTITION_KW);
            // name
            name_ref(p);
            // { FOR VALUES partition_bound_spec | DEFAULT }
            partition_option(p);
            ATTACH_PARTITION
        }
        // SET SCHEMA new_schema
        // SET TABLESPACE new_tablespace [ NOWAIT ]
        // SET WITHOUT CLUSTER
        // SET WITHOUT OIDS
        // SET ACCESS METHOD { new_access_method | DEFAULT }
        // SET { LOGGED | UNLOGGED }
        // SET ( storage_parameter [= value] [, ... ] )
        SET_KW => {
            p.expect(SET_KW);
            // SET SCHEMA new_schema
            if p.eat(SCHEMA_KW) {
                // name
                name_ref(p);
                SET_SCHEMA
            // SET TABLESPACE new_tablespace [ NOWAIT ]
            } else if p.eat(TABLESPACE_KW) {
                // name
                name_ref(p);
                p.eat(NOWAIT_KW);
                SET_TABLESPACE
            // SET WITHOUT CLUSTER
            // SET WITHOUT OIDS
            } else if p.eat(WITHOUT_KW) {
                if p.eat(CLUSTER_KW) {
                    SET_WITHOUT_CLUSTER
                } else {
                    p.expect(OIDS_KW);
                    SET_WITHOUT_OIDS
                }
            // SET ACCESS METHOD { new_access_method | DEFAULT }
            } else if p.eat(ACCESS_KW) {
                p.eat(METHOD_KW);
                if !p.eat(DEFAULT_KW) {
                    // TODO: I think this can be stricter
                    // name
                    name_ref(p);
                }
                SET_ACCESS_METHOD
            // SET { LOGGED | UNLOGGED }
            } else if p.eat(LOGGED_KW) {
                SET_LOGGED
            } else if p.eat(UNLOGGED_KW) {
                SET_UNLOGGED
            // SET ( storage_parameter [= value] [, ... ] )
            } else {
                p.expect(L_PAREN);
                while !p.at(EOF) && !p.at(R_PAREN) {
                    if !storage_parameter(p) {
                        break;
                    }
                    if !p.eat(COMMA) {
                        break;
                    }
                }
                p.expect(R_PAREN);
                SET_STORAGE_PARAMS
            }
        }
        RESET_KW => {
            p.bump(RESET_KW);
            p.expect(L_PAREN);
            while !p.at(EOF) && !p.at(R_PAREN) {
                if !storage_parameter(p) {
                    break;
                }
                if !p.eat(COMMA) {
                    break;
                }
            }
            p.expect(R_PAREN);
            RESET_STORAGE_PARAMS
        }
        // RENAME CONSTRAINT constraint_name TO new_constraint_name
        // RENAME [ COLUMN ] column_name TO new_column_name
        // RENAME TO new_name
        RENAME_KW => {
            p.expect(RENAME_KW);
            // TO new_name
            if p.eat(TO_KW) {
                // name
                name_ref(p);
                RENAME_TABLE
            // CONSTRAINT
            } else if p.eat(CONSTRAINT_KW) {
                // name
                name_ref(p);
                // TO
                p.expect(TO_KW);
                // name
                name_ref(p);
                RENAME_CONSTRAINT
            // [ COLUMN ]
            } else {
                p.eat(COLUMN_KW);
                // name
                name_ref(p);
                // TO
                p.expect(TO_KW);
                // name
                name_ref(p);
                RENAME_COLUMN
            }
        }
        // ALTER [ COLUMN ] column_name [ SET DATA ] TYPE data_type [ COLLATE collation ] [ USING expression ]
        // ALTER [ COLUMN ] column_name SET DEFAULT expression
        // ALTER [ COLUMN ] column_name DROP DEFAULT
        // ALTER [ COLUMN ] column_name { SET | DROP } NOT NULL
        // ALTER [ COLUMN ] column_name SET EXPRESSION AS ( expression )
        // ALTER [ COLUMN ] column_name DROP EXPRESSION [ IF EXISTS ]
        // ALTER [ COLUMN ] column_name ADD GENERATED { ALWAYS | BY DEFAULT } AS IDENTITY [ ( sequence_options ) ]
        // ALTER [ COLUMN ] column_name { SET GENERATED { ALWAYS | BY DEFAULT } | SET sequence_option | RESTART [ [ WITH ] restart ] } [...]
        // ALTER [ COLUMN ] column_name DROP IDENTITY [ IF EXISTS ]
        // ALTER [ COLUMN ] column_name SET STATISTICS { integer | DEFAULT }
        // ALTER [ COLUMN ] column_name SET ( attribute_option = value [, ... ] )
        // ALTER [ COLUMN ] column_name RESET ( attribute_option [, ... ] )
        // ALTER [ COLUMN ] column_name SET STORAGE { PLAIN | EXTERNAL | EXTENDED | MAIN | DEFAULT }
        // ALTER [ COLUMN ] column_name SET COMPRESSION compression_method
        // ALTER CONSTRAINT constraint_name [ DEFERRABLE | NOT DEFERRABLE ] [ INITIALLY DEFERRED | INITIALLY IMMEDIATE ]
        ALTER_KW => {
            p.bump(ALTER_KW);
            // ALTER CONSTRAINT constraint_name [ DEFERRABLE | NOT DEFERRABLE ] [ INITIALLY DEFERRED | INITIALLY IMMEDIATE ]
            if p.eat(CONSTRAINT_KW) {
                // name
                name_ref(p);
                opt_constraint_options(p);
                ALTER_CONSTRAINT
            } else {
                p.eat(COLUMN_KW);
                if p.at_ts(COLUMN_FIRST) {
                    p.bump_any();
                } else {
                    p.error("expected column_name");
                }
                let m = p.start();
                if let Some(alter_kind) = alter_column_option(p) {
                    m.complete(p, alter_kind);
                } else {
                    p.error("expected alter column option");
                    m.abandon(p);
                }
                ALTER_COLUMN
            }
        }
        _ => return None,
    };
    Some(kind)
}

fn opt_not_valid(p: &mut Parser<'_>) -> Option<CompletedMarker> {
    if p.at(NOT_KW) {
        let m = p.start();
        p.bump(NOT_KW);
        p.expect(VALID_KW);
        Some(m.complete(p, NOT_VALID))
    } else {
        None
    }
}

// /* Column label --- allowed labels in "AS" clauses.
//  * This presently includes *all* Postgres keywords.
//  */
// ColLabel:  IDENT
//   | unreserved_keyword
//   | col_name_keyword
//   | type_func_name_keyword
//   | reserved_keyword
fn opt_col_label(p: &mut Parser<'_>) -> bool {
    if p.at_ts(COL_LABEL_FIRST) {
        let m = p.start();
        p.bump_any();
        m.complete(p, NAME);
        true
    } else {
        false
    }
}

fn col_label(p: &mut Parser<'_>) {
    if !opt_col_label(p) {
        p.error("expected label");
    }
}

enum AttributeValue {
    Either,
    Required,
    Disallowed,
}

// reloption_list:
//   | reloption_elem
//   | reloption_list ',' reloption_elem
// reloption_elem:
//   | ColLabel '=' def_arg
//   | ColLabel
//   | ColLabel '.' ColLabel '=' def_arg
//   | ColLabel '.' ColLabel
fn attribute_option(p: &mut Parser<'_>, option: AttributeValue) -> bool {
    if !opt_col_label(p) {
        return false;
    }
    if p.eat(DOT) && !opt_col_label(p) {
        p.error("expected column label")
    }
    match option {
        AttributeValue::Required => {
            p.expect(EQ);
            def_arg(p);
        }
        AttributeValue::Disallowed => {}
        AttributeValue::Either => {
            if p.eat(EQ) {
                def_arg(p);
            }
        }
    }
    true
}

// def_arg:
//   | func_type
//   | reserved_keyword
//   | qual_all_Op
//   | NumericOnly
//   | Sconst
//   | NONE
//
// qual_all_Op:
//   | all_Op
//   | OPERATOR '(' any_operator ')'
fn def_arg(p: &mut Parser<'_>) -> bool {
    if opt_bool_literal(p)
        || opt_string_literal(p).is_some()
        || opt_numeric_literal(p).is_some()
        || opt_operator(p)
        || p.eat(NONE_KW)
    {
        true
    } else if p.at_ts(RESERVED_KEYWORDS) {
        p.bump_any();
        true
    } else if p.eat(OPERATOR_KW) {
        p.expect(L_PAREN);
        operator(p);
        p.expect(R_PAREN);
        true
    } else {
        opt_type_name(p)
    }
}

fn generated_options(p: &mut Parser<'_>) {
    // {
    //  | SET GENERATED { ALWAYS | BY DEFAULT }
    //  | SET sequence_option
    //  | RESTART [ [ WITH ] restart ]
    // } [...]
    while !p.at(EOF) {
        let m = p.start();
        if p.eat(RESTART_KW) {
            if p.eat(WITH_KW) {
                if opt_numeric_literal(p).is_none() {
                    p.error("expected numeric literal");
                }
            } else {
                let _ = opt_numeric_literal(p);
            }
            m.complete(p, RESTART);
        } else if p.eat(SET_KW) {
            if opt_sequence_option(p) {
                m.complete(p, SET_SEQUENCE_OPTION);
                continue;
            } else {
                p.expect(GENERATED_KW);
                if !p.eat(ALWAYS_KW) {
                    p.expect(BY_KW);
                    p.expect(DEFAULT_KW);
                }
                m.complete(p, SET_GENERATED);
            }
        } else {
            m.abandon(p);
            break;
        }
    }
}

// [ COLUMN ] column_name [ SET DATA ] TYPE data_type [ COLLATE collation ] [ USING expression ]
// [ COLUMN ] column_name SET DEFAULT expression
// [ COLUMN ] column_name DROP DEFAULT
// [ COLUMN ] column_name { SET | DROP } NOT NULL
// [ COLUMN ] column_name SET EXPRESSION AS ( expression )
// [ COLUMN ] column_name DROP EXPRESSION [ IF EXISTS ]
// [ COLUMN ] column_name ADD GENERATED { ALWAYS | BY DEFAULT } AS IDENTITY [ ( sequence_options ) ]
// [ COLUMN ] column_name { SET GENERATED { ALWAYS | BY DEFAULT } | SET sequence_option | RESTART [ [ WITH ] restart ] } [...]
// [ COLUMN ] column_name DROP IDENTITY [ IF EXISTS ]
// [ COLUMN ] column_name SET STATISTICS { integer | DEFAULT }
// [ COLUMN ] column_name SET ( attribute_option = value [, ... ] )
// [ COLUMN ] column_name RESET ( attribute_option [, ... ] )
// [ COLUMN ] column_name SET STORAGE { PLAIN | EXTERNAL | EXTENDED | MAIN | DEFAULT }
// [ COLUMN ] column_name SET COMPRESSION compression_method
// [ COLUMN ] column_name OPTIONS ( [ ADD | SET | DROP ] option ['value'] [, ... ])
fn alter_column_option(p: &mut Parser<'_>) -> Option<SyntaxKind> {
    // DROP NOT NULL
    // DROP DEFAULT
    // DROP EXPRESSION [ IF EXISTS ]
    // DROP IDENTITY [ IF EXISTS ]
    let kind = match p.current() {
        DROP_KW => {
            p.bump(DROP_KW);
            if p.eat(DEFAULT_KW) {
                DROP_DEFAULT
            } else if p.eat(EXPRESSION_KW) {
                opt_if_exists(p);
                DROP_EXPRESSION
            } else if p.eat(IDENTITY_KW) {
                opt_if_exists(p);
                DROP_IDENTITY
            } else {
                p.expect(NOT_KW);
                p.expect(NULL_KW);
                DROP_NOT_NULL
            }
        }
        // RESTART [ [ WITH ] restart ]
        RESTART => {
            p.bump(RESTART_KW);
            if p.eat(WITH_KW) {
                p.expect(RESTART_KW);
            } else {
                p.eat(RESTART_KW);
            }
            RESTART
        }
        ADD_KW => {
            p.bump(ADD_KW);
            // ADD GENERATED { ALWAYS | BY DEFAULT } AS IDENTITY [ ( sequence_options ) ]
            p.expect(GENERATED_KW);
            if !p.eat(ALWAYS_KW) {
                p.expect(BY_KW);
                p.expect(DEFAULT_KW);
            }
            p.expect(AS_KW);
            p.expect(IDENTITY_KW);
            opt_sequence_options(p);
            ADD_GENERATED
        }
        // RESET ( attribute_option [, ... ] )
        RESET_KW => {
            p.bump(RESET_KW);
            p.expect(L_PAREN);
            while !p.at(EOF) && !p.at(R_PAREN) {
                if !attribute_option(p, AttributeValue::Disallowed) {
                    break;
                }
                if !p.eat(COMMA) {
                    break;
                }
            }
            p.expect(R_PAREN);
            RESET_OPTIONS
        }
        // TYPE data_type [ COLLATE collation ] [ USING expression ]
        TYPE_KW => {
            set_data_type(p);
            SET_TYPE
        }
        // SET DATA TYPE data_type [ COLLATE collation ] [ USING expression ]
        SET_KW if p.nth_at(1, DATA_KW) => {
            p.bump(SET_KW);
            p.bump(DATA_KW);
            set_data_type(p);
            SET_TYPE
        }
        // { SET GENERATED { ALWAYS | BY DEFAULT }
        SET_KW if p.nth_at(1, GENERATED_KW) => {
            generated_options(p);
            SET_GENERATED_OPTIONS
        }
        // SET sequence_option
        SET_KW if p.nth_at_ts(1, SEQUENCE_OPTION_FIRST) => {
            generated_options(p);
            SET_GENERATED_OPTIONS
        }
        // RESTART [ [ WITH ] restart ] } [...]
        RESTART_KW => {
            generated_options(p);
            SET_GENERATED_OPTIONS
        }
        // OPTIONS ( [ ADD | SET | DROP ] option ['value'] [, ... ])
        OPTIONS_KW => {
            p.bump(OPTIONS_KW);
            p.expect(L_PAREN);
            alter_option(p);
            while !p.at(EOF) && p.eat(COMMA) {
                alter_option(p);
            }
            p.expect(R_PAREN);
            SET_OPTIONS_LIST
        }
        // SET DEFAULT expression
        SET_KW if p.nth_at(1, DEFAULT_KW) => {
            p.bump(SET_KW);
            p.bump(DEFAULT_KW);
            if expr(p).is_none() {
                p.error("expected expr");
            }
            SET_DEFAULT
        }
        // SET EXPRESSION AS ( expression )
        SET_KW if p.nth_at(1, EXPRESSION_KW) => {
            p.bump(SET_KW);
            p.bump(EXPRESSION_KW);
            p.expect(AS_KW);
            p.expect(L_PAREN);
            if expr(p).is_none() {
                p.error("expected expr");
            }
            p.expect(R_PAREN);
            SET_EXPRESSION
        }
        // SET STATISTICS { integer | DEFAULT }
        SET_KW if p.nth_at(1, STATISTICS_KW) => {
            p.bump(SET_KW);
            p.bump(STATISTICS_KW);
            if !p.eat(DEFAULT_KW) {
                if opt_numeric_literal(p).is_none() {
                    p.error("expected numeric literal");
                }
            }
            SET_STATISTICS
        }
        // SET ( attribute_option = value [, ... ] )
        SET_KW if p.nth_at(1, L_PAREN) => {
            p.bump(SET_KW);
            p.bump(L_PAREN);
            while !p.at(EOF) && !p.at(R_PAREN) {
                if !attribute_option(p, AttributeValue::Either) {
                    break;
                }
                if !p.eat(COMMA) {
                    break;
                }
            }
            p.expect(R_PAREN);
            SET_OPTIONS
        }
        // SET STORAGE { PLAIN | EXTERNAL | EXTENDED | MAIN | DEFAULT }
        SET_KW if p.nth_at(1, STORAGE_KW) => {
            p.bump(SET_KW);
            p.bump(STORAGE_KW);
            if !p.eat(DEFAULT_KW) {
                if p.at_ts(COLUMN_FIRST) {
                    p.bump_any();
                } else {
                    p.error("expected name");
                }
            }
            SET_STORAGE
        }
        // SET COMPRESSION { ColId | DEFAULT }
        SET_KW if p.nth_at(1, COMPRESSION_KW) => {
            p.bump(SET_KW);
            p.bump(COMPRESSION_KW);
            if !p.eat(DEFAULT_KW) {
                if p.at_ts(COLUMN_FIRST) {
                    p.bump_any();
                } else {
                    p.error("expected name");
                }
            }
            SET_COMPRESSION
        }
        // SET NOT NULL
        SET_KW if p.nth_at(1, NOT_KW) => {
            p.bump(SET_KW);
            p.bump(NOT_KW);
            p.expect(NULL_KW);
            SET_NOT_NULL
        }
        _ => return None,
    };
    Some(kind)
}

fn opt_collate(p: &mut Parser<'_>) -> Option<CompletedMarker> {
    let m = p.start();
    if p.eat(COLLATE_KW) {
        path_name_ref(p);
        Some(m.complete(p, COLLATE))
    } else {
        m.abandon(p);
        None
    }
}

// TYPE data_type [ COLLATE collation ] [ USING expression ]
fn set_data_type(p: &mut Parser<'_>) {
    p.expect(TYPE_KW);
    type_name(p);
    opt_collate(p);
    if p.eat(USING_KW) && expr(p).is_none() {
        p.error("expected expression");
    }
}

pub(crate) fn entry_point(p: &mut Parser) {
    let m = p.start();
    while !p.at(EOF) {
        // handle things like: ;;;select 1
        if p.eat(SEMICOLON) {
            continue;
        }
        let parsed_stmt = stmt(
            p,
            &StmtRestrictions {
                begin_end_allowed: true,
            },
        );
        if !p.at(EOF) && parsed_stmt.is_some() {
            p.expect(SEMICOLON);
        }
    }
    m.complete(p, SOURCE_FILE);
}
