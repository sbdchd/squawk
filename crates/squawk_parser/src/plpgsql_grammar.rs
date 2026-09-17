use crate::{
    Parser, SyntaxKind,
    generated::token_sets::{
        ALL_KEYWORDS, PLPGSQL_RESERVED_CONTEXTUAL_KEYWORDS, PLPGSQL_RESERVED_KEYWORDS,
    },
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
    body(p);
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
        temp_unknown(p, "expected a declaration");
    }
    m.complete(p, PLPGSQL_DECLARE_SECTION);
}

fn body(p: &mut Parser) {
    let m = p.start();
    while !p.at(EOF) && !at_block_end(p) {
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

fn at_block_end(p: &Parser) -> bool {
    if !p.at(END_KW) {
        return false;
    }
    !p.nth_at(1, IF_KW) && !p.nth_at(1, CASE_KW) && !p.nth_at_contextual_kw(1, LOOP_KW)
}
