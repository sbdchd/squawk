use rowan::{TextRange, TextSize};
use salsa::Database as Db;
use smallvec::SmallVec;
use squawk_syntax::{
    SyntaxKind, SyntaxNode, SyntaxToken,
    ast::{self, AstNode},
    sql_body::SqlBody,
};

use crate::db::parse;
use crate::file::{FilePosition, InFile};
use crate::ir::ItemRange;
use crate::semantic_index::SemanticIndex;

pub(crate) fn token_from_offset(db: &dyn Db, position: InFile<TextSize>) -> Option<SyntaxToken> {
    token_from_syntax_offset(parse(db, position.file_id).tree().syntax(), position.value)
}

pub(crate) fn token_from_syntax_offset(
    syntax: &SyntaxNode,
    position: TextSize,
) -> Option<SyntaxToken> {
    let mut token = syntax.token_at_offset(position).right_biased()?;
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

pub(crate) struct TokenAt {
    pub(crate) token: SyntaxToken,
    offset: TextSize,
    bodies: SmallVec<[SqlBody; 1]>,
}

impl TokenAt {
    pub(crate) fn new(file: &ast::SourceFile, offset: TextSize) -> Option<TokenAt> {
        let mut bodies: SmallVec<[SqlBody; 1]> = SmallVec::new();
        let mut root = file.syntax().clone();
        let mut offset = offset;
        loop {
            let token = token_from_syntax_offset(&root, offset)?;
            let Some(body) = token
                .parent()
                .and_then(ast::Literal::cast)
                .and_then(|literal| literal.sql_body())
            else {
                return Some(TokenAt {
                    token,
                    offset,
                    bodies,
                });
            };
            let Some(body_offset) = body.body_position(offset) else {
                return Some(TokenAt {
                    token,
                    offset,
                    bodies,
                });
            };
            root = body.syntax();
            offset = body_offset;
            bodies.push(body);
        }
    }

    pub(crate) fn range(&self) -> TextRange {
        self.original_range(self.token.text_range())
    }

    fn cursor(&self) -> Option<TextRange> {
        self.token
            .text_range()
            .contains_inclusive(self.offset)
            .then(|| self.original_range(TextRange::empty(self.offset)))
    }

    pub(crate) fn original_range(&self, range: TextRange) -> TextRange {
        self.bodies
            .iter()
            .rev()
            .fold(range, |range, body| body.source_range(range))
    }
}

pub(crate) fn item_at_position(
    index: &SemanticIndex,
    file: &ast::SourceFile,
    position: FilePosition,
) -> Option<InFile<ItemRange>> {
    let token = TokenAt::new(file, position.value)?;
    item_at_token(index, position, &token)
}

pub(crate) fn item_at_token(
    index: &SemanticIndex,
    position: FilePosition,
    token: &TokenAt,
) -> Option<InFile<ItemRange>> {
    let range = token.range();
    index.item_at_range(position.with_value(range)).or_else(|| {
        index
            .item_at_range(position.with_value(token.cursor()?))
            .filter(|item| range.contains_range(item.value.range))
    })
}
