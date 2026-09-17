use crate::{Parser, syntax_kind::SyntaxKind::*};

pub(crate) fn plpgsql_entry_point(p: &mut Parser) {
    let m = p.start();
    while !p.at(EOF) {
        if p.at(NULL_KW) && p.nth_at(1, SEMICOLON) {
            let m = p.start();
            p.bump(NULL_KW);
            p.bump(SEMICOLON);
            m.complete(p, PLPGSQL_NULL_STMT);
        } else {
            let kind = p.current();
            p.err_and_bump(&format!("expected a statement, found {kind:?}"));
        }
    }
    m.complete(p, PLPGSQL);
}
