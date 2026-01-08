pub use crate::ast::generated::nodes::*;
use crate::{
    SyntaxNode,
    ast::{self, AstNode, support},
};

// TODO: Initial attempt to try and unify the CreateTable and
// CreateForeignTable. Not sure this is the right approach, we may want to be
// more general, like TableSource, which can be a View, CTE, Table,
// ForeignTable, Subquery, etc.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateTableLike {
    pub(crate) syntax: SyntaxNode,
}
impl CreateTableLike {
    #[inline]
    pub fn path(&self) -> Option<ast::Path> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn table_arg_list(&self) -> Option<ast::TableArgList> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn inherits(&self) -> Option<ast::Inherits> {
        support::child(&self.syntax)
    }
}
impl AstNode for CreateTableLike {
    #[inline]
    fn can_cast(kind: ast::SyntaxKind) -> bool {
        matches!(
            kind,
            ast::SyntaxKind::CREATE_TABLE | ast::SyntaxKind::CREATE_FOREIGN_TABLE
        )
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
