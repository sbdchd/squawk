use crate::{
    Parser, SyntaxKind,
    generated::token_sets::{
        ALL_KEYWORDS, PLPGSQL_RESERVED_CONTEXTUAL_KEYWORDS, PLPGSQL_RESERVED_KEYWORDS,
    },
    grammar,
    syntax_kind::SyntaxKind::*,
    token_set::TokenSet,
};

pub(crate) fn plpgsql_entry_point(p: &mut Parser) {
    let m = p.start();
    while !p.at(EOF) {
        if at_block_start(p) {
            opt_block(p);
        } else {
            temp_unknown(p, "expected a block");
        }
    }
    m.complete(p, PLPGSQL);
}

const BLOCK_FIRST: TokenSet = TokenSet::new(&[BEGIN_KW, DECLARE_KW]);

fn opt_block(p: &mut Parser) {
    if !at_block_start(p) {
        return;
    }
    let m = p.start();
    opt_block_label(p);
    opt_declare_section(p);
    p.expect(BEGIN_KW);
    body(p, BodyKind::Block);
    opt_exception_section(p);
    p.expect(END_KW);
    opt_label_name_ref(p);
    // TODO: add validation, sometimes this is required
    p.eat(SEMICOLON);
    m.complete(p, PLPGSQL_BLOCK);
}

// <<foo>>
fn opt_block_label(p: &mut Parser) {
    if !at_block_label(p) {
        return;
    }
    let m = p.start();
    p.bump(LESS_LESS);
    name(p, PLPGSQL_LABEL_NAME);
    p.bump(GREATER_GREATER);
    m.complete(p, PLPGSQL_LABEL);
}

fn opt_label_name_ref(p: &mut Parser) {
    if !at_name(p, 0) {
        return;
    }
    name(p, PLPGSQL_LABEL_NAME_REF);
}

fn name(p: &mut Parser, kind: SyntaxKind) {
    let m = p.start();
    p.bump_any();
    m.complete(p, kind);
}

fn opt_declare_section(p: &mut Parser) {
    if !p.at(DECLARE_KW) {
        return;
    }
    let m = p.start();
    p.bump(DECLARE_KW);
    while !p.at(EOF) && !p.at(BEGIN_KW) {
        if at_alias_decl(p) {
            alias_decl(p);
        } else if at_cursor_decl(p) {
            cursor_decl(p);
        } else if at_var_decl(p) {
            var_decl(p);
        } else {
            temp_unknown(p, "expected a declaration");
        }
    }
    m.complete(p, PLPGSQL_DECLARE_SECTION);
}

fn var_decl(p: &mut Parser) {
    let m = p.start();
    name(p, PLPGSQL_VAR_NAME);
    if p.nth_at_contextual_kw(0, CONSTANT_KW) {
        p.bump_remap(CONSTANT_KW);
    }
    decl_datatype(p);
    grammar::opt_collate(p);
    opt_not_null(p);
    opt_var_init(p);
    p.expect(SEMICOLON);
    m.complete(p, PLPGSQL_VAR_DECL);
}

fn alias_decl(p: &mut Parser) {
    let m = p.start();
    name(p, PLPGSQL_VAR_NAME);
    p.bump_remap(ALIAS_KW);
    p.expect(FOR_KW);
    alias_target(p);
    p.expect(SEMICOLON);
    m.complete(p, PLPGSQL_ALIAS_DECL);
}

fn alias_target(p: &mut Parser) {
    let m = p.start();
    if at_path(p).is_some() {
        path_name_ref(p);
    } else {
        let kind = p.current();
        p.error(format!("expected an alias target, found {kind:?}"));
    }
    m.complete(p, PLPGSQL_ALIAS_TARGET);
}

fn cursor_decl(p: &mut Parser) {
    let m = p.start();
    name(p, PLPGSQL_VAR_NAME);
    grammar::opt_cursor_scroll(p);
    p.expect(CURSOR_KW);
    if p.at(L_PAREN) {
        cursor_arg_list(p);
    }
    if !p.eat(IS_KW) {
        p.expect(FOR_KW);
    }
    if grammar::stmt(p, &grammar::StmtRestrictions::default()).is_none() {
        p.error("expected a query");
    }
    p.expect(SEMICOLON);
    m.complete(p, PLPGSQL_CURSOR_DECL);
}

fn cursor_arg_list(p: &mut Parser) {
    assert!(p.at(L_PAREN));
    let m = p.start();
    // TODO: use delimited
    p.bump(L_PAREN);
    if p.at(R_PAREN) {
        p.error("expected at least one cursor argument");
    }
    while !p.at(EOF) && !p.at(R_PAREN) {
        cursor_arg(p);
        if !p.eat(COMMA) {
            break;
        }
    }
    p.expect(R_PAREN);
    m.complete(p, PLPGSQL_CURSOR_ARG_LIST);
}

fn cursor_arg(p: &mut Parser) {
    let m = p.start();
    if at_name(p, 0) {
        name(p, PLPGSQL_VAR_NAME);
        decl_datatype(p);
    } else {
        let kind = p.current();
        p.error(format!("expected a cursor argument name, found {kind:?}"));
    }
    m.complete(p, PLPGSQL_CURSOR_ARG);
}

fn decl_datatype(p: &mut Parser) {
    if !at_percent_datatype(p) {
        grammar::func_type(p);
        return;
    }
    let m = p.start();
    path_name_ref(p);
    let kind = if grammar::opt_percent_type(p).is_some() {
        PERCENT_TYPE
    } else {
        p.bump(PERCENT);
        p.bump_remap(ROWTYPE_KW);
        PLPGSQL_PERCENT_ROWTYPE
    };
    p.eat(ARRAY_KW);
    while !p.at(EOF) && grammar::opt_array_bound(p) {}
    m.complete(p, kind);
}

fn path_name_ref(p: &mut Parser) {
    assert!(at_path(p).is_some());
    let m = p.start();
    name(p, PATH_SEGMENT_REF);
    let mut path = m.complete(p, PATH_REF);
    while !p.at(EOF) && p.at(DOT) {
        let m = path.precede(p);
        p.bump(DOT);
        name(p, PATH_SEGMENT_REF);
        path = m.complete(p, PATH_REF);
    }
}

fn at_percent_datatype(p: &Parser) -> bool {
    let Some(n) = at_path(p) else {
        return false;
    };
    p.nth_at(n, PERCENT) && (p.nth_at(n + 1, TYPE_KW) || p.nth_at_contextual_kw(n + 1, ROWTYPE_KW))
}

fn at_path(p: &Parser) -> Option<usize> {
    let mut n = 0;
    while !p.nth_at(n, EOF) && at_name(p, n) {
        n += 1;
        if !p.nth_at(n, DOT) {
            return Some(n);
        }
        n += 1;
    }
    None
}

fn opt_not_null(p: &mut Parser) {
    if !p.at(NOT_KW) {
        return;
    }
    let m = p.start();
    p.bump(NOT_KW);
    p.expect(NULL_KW);
    m.complete(p, PLPGSQL_NOT_NULL);
}

fn opt_var_init(p: &mut Parser) {
    if !p.at(COLON_EQ) && !p.at(EQ) && !p.at(DEFAULT_KW) {
        return;
    }
    let m = p.start();
    if !p.eat(COLON_EQ) && !p.eat(EQ) {
        p.bump(DEFAULT_KW);
    }
    expr(p);
    m.complete(p, PLPGSQL_VAR_INIT);
}

fn expr(p: &mut Parser) {
    if grammar::expr(p).is_none() {
        p.error("expected an expression");
    }
}

#[derive(Clone, Copy, PartialEq)]
enum BodyKind {
    Block,
    ExceptionHandler,
    IfThen,
    IfElse,
    CaseWhen,
    CaseElse,
}

fn body(p: &mut Parser, kind: BodyKind) {
    let m = p.start();
    while !p.at(EOF) && !at_body_end(p, kind) {
        stmt(p);
    }
    m.complete(p, PLPGSQL_BODY);
}

fn stmt(p: &mut Parser) {
    if at_block_start(p) {
        opt_block(p);
    } else if p.at(CASE_KW) {
        case_stmt(p);
    } else if p.at(IF_KW) {
        if_stmt(p);
    } else if p.at(NULL_KW) && p.nth_at(1, SEMICOLON) {
        let m = p.start();
        p.bump(NULL_KW);
        p.bump(SEMICOLON);
        m.complete(p, PLPGSQL_NULL_STMT);
    } else {
        temp_unknown(p, "expected a statement");
    }
}

fn if_stmt(p: &mut Parser) {
    assert!(p.at(IF_KW));
    let m = p.start();
    p.bump(IF_KW);
    expr(p);
    p.expect(THEN_KW);
    body(p, BodyKind::IfThen);
    while !p.at(EOF) && opt_elsif_clause(p) {}
    opt_else_clause(p, BodyKind::IfElse);
    p.expect(END_KW);
    p.expect(IF_KW);
    p.expect(SEMICOLON);
    m.complete(p, PLPGSQL_IF_STMT);
}

fn case_stmt(p: &mut Parser) {
    assert!(p.at(CASE_KW));
    let m = p.start();
    p.bump(CASE_KW);
    if !p.at(WHEN_KW) {
        expr(p);
        if !p.at(WHEN_KW) {
            p.error("expected `when`");
        }
    }
    while !p.at(EOF) && p.at(WHEN_KW) {
        case_when(p);
    }
    opt_else_clause(p, BodyKind::CaseElse);
    p.expect(END_KW);
    p.expect(CASE_KW);
    p.expect(SEMICOLON);
    m.complete(p, PLPGSQL_CASE_STMT);
}

fn case_when(p: &mut Parser) {
    assert!(p.at(WHEN_KW));
    let m = p.start();
    p.bump(WHEN_KW);
    expr(p);
    while !p.at(EOF) && p.eat(COMMA) {
        expr(p);
    }
    p.expect(THEN_KW);
    body(p, BodyKind::CaseWhen);
    m.complete(p, PLPGSQL_CASE_WHEN);
}

fn opt_elsif_clause(p: &mut Parser) -> bool {
    if !at_elsif(p) {
        return false;
    }
    let m = p.start();
    if p.nth_at_contextual_kw(0, ELSEIF_KW) {
        p.bump_remap(ELSEIF_KW);
    } else {
        p.bump_remap(ELSIF_KW);
    }
    expr(p);
    p.expect(THEN_KW);
    body(p, BodyKind::IfThen);
    m.complete(p, PLPGSQL_ELSIF_CLAUSE);
    true
}

fn opt_else_clause(p: &mut Parser, kind: BodyKind) {
    if !p.at(ELSE_KW) {
        return;
    }
    let m = p.start();
    p.bump(ELSE_KW);
    body(p, kind);
    m.complete(p, PLPGSQL_ELSE_CLAUSE);
}

// TODO: remove this once we get all the ast nodes working
fn temp_unknown(p: &mut Parser, message: &str) {
    let m = p.start();
    let kind = p.current();
    p.error(format!("{message}, found {kind:?}"));

    let mut depth = 0;
    while !p.at(EOF) {
        if depth == 0 && p.at(SEMICOLON) {
            break;
        }
        if p.at(BEGIN_KW) {
            depth += 1;
        } else if p.at(END_KW) && depth > 0 {
            depth -= 1;
        }
        p.bump_any();
    }
    p.eat(SEMICOLON);
    m.complete(p, ERROR);
}

fn opt_exception_section(p: &mut Parser) {
    if !p.nth_at_contextual_kw(0, EXCEPTION_KW) {
        return;
    }
    let m = p.start();
    p.bump_remap(EXCEPTION_KW);
    while !p.at(EOF) && p.at(WHEN_KW) {
        exception_handler(p);
    }
    m.complete(p, PLPGSQL_EXCEPTION_SECTION);
}

fn exception_handler(p: &mut Parser) {
    assert!(p.at(WHEN_KW));
    let m = p.start();
    // TODO: use delimited
    p.bump(WHEN_KW);
    condition(p);
    while !p.at(EOF) && p.eat(OR_KW) {
        condition(p);
    }
    p.expect(THEN_KW);
    body(p, BodyKind::ExceptionHandler);
    m.complete(p, PLPGSQL_EXCEPTION_HANDLER);
}

fn condition(p: &mut Parser) {
    let m = p.start();
    if at_name(p, 0) {
        let sqlstate = p.nth_at_contextual_kw(0, SQLSTATE_KW);
        p.bump_any();
        if sqlstate {
            p.expect(STRING);
        }
    } else {
        let kind = p.current();
        p.error(format!("expected a condition name, found {kind:?}"));
    }
    m.complete(p, PLPGSQL_CONDITION);
}

fn at_alias_decl(p: &Parser) -> bool {
    at_name(p, 0) && p.nth_at_contextual_kw(1, ALIAS_KW)
}

fn at_cursor_decl(p: &Parser) -> bool {
    at_name(p, 0) && (p.nth_at(1, CURSOR_KW) || p.nth_at(1, SCROLL_KW) || p.nth_at(1, NO_KW))
}

fn at_var_decl(p: &Parser) -> bool {
    at_name(p, 0)
}

fn at_block_start(p: &Parser) -> bool {
    p.at_ts(BLOCK_FIRST) || at_block_label(p)
}

fn at_block_label(p: &Parser) -> bool {
    p.at(LESS_LESS) && at_name(p, 2) && p.nth_at(3, GREATER_GREATER) && p.nth_at_ts(5, BLOCK_FIRST)
}

fn at_name(p: &Parser, n: usize) -> bool {
    if p.nth_at(n, IDENT) {
        return !p.nth_at_contextual_ts(n, PLPGSQL_RESERVED_CONTEXTUAL_KEYWORDS);
    }
    if p.nth_at(n, POSITIONAL_PARAM) {
        return true;
    }
    p.nth_at_ts(n, ALL_KEYWORDS) && !p.nth_at_ts(n, PLPGSQL_RESERVED_KEYWORDS)
}

fn at_elsif(p: &Parser) -> bool {
    p.nth_at_contextual_kw(0, ELSIF_KW) || p.nth_at_contextual_kw(0, ELSEIF_KW)
}

fn at_body_end(p: &Parser, kind: BodyKind) -> bool {
    if at_block_end(p) {
        return true;
    }
    match kind {
        BodyKind::Block => p.nth_at_contextual_kw(0, EXCEPTION_KW),
        BodyKind::ExceptionHandler => p.nth_at_contextual_kw(0, EXCEPTION_KW) || p.at(WHEN_KW),
        BodyKind::IfThen => at_end_if(p) || p.at(ELSE_KW) || at_elsif(p),
        BodyKind::IfElse => at_end_if(p),
        BodyKind::CaseWhen => at_end_case(p) || p.at(WHEN_KW) || p.at(ELSE_KW),
        BodyKind::CaseElse => at_end_case(p),
    }
}

fn at_end_if(p: &Parser) -> bool {
    p.at(END_KW) && p.nth_at(1, IF_KW)
}

fn at_end_case(p: &Parser) -> bool {
    p.at(END_KW) && p.nth_at(1, CASE_KW)
}

fn at_block_end(p: &Parser) -> bool {
    if !p.at(END_KW) {
        return false;
    }
    !p.nth_at(1, IF_KW) && !p.nth_at(1, CASE_KW) && !p.nth_at_contextual_kw(1, LOOP_KW)
}
