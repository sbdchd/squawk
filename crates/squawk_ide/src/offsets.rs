use crate::db::parse;
use crate::file::InFile;
use rowan::TextSize;
use salsa::Database as Db;
use squawk_syntax::{SyntaxKind, SyntaxToken, ast::AstNode};

pub(crate) fn token_from_offset(db: &dyn Db, position: InFile<TextSize>) -> Option<SyntaxToken> {
    let mut token = parse(db, position.file_id)
        .tree()
        .syntax()
        .token_at_offset(position.value)
        .right_biased()?;
    // want to be lenient in case someone clicks:
    // - the trailing `;` of a line
    // - the `,` in a target list, like `select a, b, c`
    // - the `.` following a table/schema/column, like `select t.a from t`
    // - the `)` following a composite type, like `select (c).f from t`
    // - the `[` in `select c[1] from t`
    // - the `]` in `select c[a] from t`
    // - the `(` in `select foo()`
    if matches!(
        token.kind(),
        SyntaxKind::SEMICOLON
            | SyntaxKind::COMMA
            | SyntaxKind::DOT
            | SyntaxKind::R_PAREN
            | SyntaxKind::L_BRACK
            | SyntaxKind::R_BRACK
            | SyntaxKind::L_PAREN
    ) {
        token = token.prev_token()?;
    }
    Some(token)
}
