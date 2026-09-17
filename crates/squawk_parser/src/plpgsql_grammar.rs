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
            block(p);
        } else {
            temp_unknown(p, "expected a block");
        }
    }
    m.complete(p, PLPGSQL);
}

const BLOCK_FIRST: TokenSet = TokenSet::new(&[BEGIN_KW, DECLARE_KW]);

fn block(p: &mut Parser) {
    assert!(at_block_start(p));
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
        if at_var_decl(p) {
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
    grammar::func_type(p);
    grammar::opt_collate(p);
    opt_not_null(p);
    opt_var_init(p);
    p.expect(SEMICOLON);
    m.complete(p, PLPGSQL_VAR_DECL);
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
    if grammar::expr(p).is_none() {
        p.error("expected an expression");
    }
    m.complete(p, PLPGSQL_VAR_INIT);
}

#[derive(Clone, Copy, PartialEq)]
enum BodyKind {
    Block,
    ExceptionHandler,
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
        block(p);
    } else if p.at(NULL_KW) && p.nth_at(1, SEMICOLON) {
        let m = p.start();
        p.bump(NULL_KW);
        p.bump(SEMICOLON);
        m.complete(p, PLPGSQL_NULL_STMT);
    } else {
        temp_unknown(p, "expected a statement");
    }
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
    while p.at(WHEN_KW) {
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
    while p.eat(OR_KW) {
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

fn at_var_decl(p: &Parser) -> bool {
    at_name(p, 0)
        && !p.nth_at_contextual_kw(1, ALIAS_KW)
        && !p.nth_at(1, CURSOR_KW)
        && !p.nth_at(1, SCROLL_KW)
        && !p.nth_at(1, NO_KW)
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
    p.nth_at_ts(n, ALL_KEYWORDS) && !p.nth_at_ts(n, PLPGSQL_RESERVED_KEYWORDS)
}

fn at_body_end(p: &Parser, kind: BodyKind) -> bool {
    at_block_end(p)
        || p.nth_at_contextual_kw(0, EXCEPTION_KW)
        // TODO: do we need this kind param?
        || (kind == BodyKind::ExceptionHandler && p.at(WHEN_KW))
}

fn at_block_end(p: &Parser) -> bool {
    if !p.at(END_KW) {
        return false;
    }
    !p.nth_at(1, IF_KW) && !p.nth_at(1, CASE_KW) && !p.nth_at_contextual_kw(1, LOOP_KW)
}
