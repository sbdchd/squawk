use crate::{Parser, syntax_kind::SyntaxKind::*, token_set::TokenSet};

pub(crate) fn plpgsql_entry_point(p: &mut Parser) {
    let m = p.start();
    while !p.at(EOF) {
        if p.at_ts(BLOCK_FIRST) {
            block(p);
        } else {
            temp_unknown(p, "expected a block");
        }
    }
    m.complete(p, PLPGSQL);
}

const BLOCK_FIRST: TokenSet = TokenSet::new(&[BEGIN_KW, DECLARE_KW]);

fn block(p: &mut Parser) {
    assert!(p.at_ts(BLOCK_FIRST));
    let m = p.start();
    opt_declare_section(p);
    p.expect(BEGIN_KW);
    body(p);
    p.expect(END_KW);
    // TODO: add validation, sometimes this is required
    p.eat(SEMICOLON);
    m.complete(p, PLPGSQL_BLOCK);
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
    if p.at_ts(BLOCK_FIRST) {
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

fn at_block_end(p: &Parser) -> bool {
    if !p.at(END_KW) {
        return false;
    }
    !p.nth_at(1, IF_KW) && !p.nth_at(1, CASE_KW) && !p.nth_at_contextual_kw(1, LOOP_KW)
}
