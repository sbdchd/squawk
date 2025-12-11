use rowan::TextSize;
use squawk_syntax::{
    SyntaxKind, SyntaxToken,
    ast::{self, AstNode},
};

pub(crate) fn token_from_offset(file: &ast::SourceFile, offset: TextSize) -> Option<SyntaxToken> {
    let mut token = file.syntax().token_at_offset(offset).right_biased()?;
    // want to be lenient in case someone clicks the trailing `;` of a line
    // instead of an identifier
    // or if someone clicks the `,` in a target list, like `select a, b, c`
    if token.kind() == SyntaxKind::SEMICOLON || token.kind() == SyntaxKind::COMMA {
        token = token.prev_token()?;
    }
    return Some(token);
}
