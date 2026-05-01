use rowan::TextRange;
use salsa::Database as Db;
use squawk_syntax::SyntaxNode;
use squawk_syntax::ast::AstNode;

use crate::{
    classify::classify_def_node,
    db::{File, parse},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocationKind {
    Aggregate,
    CaseExpr,
    Channel,
    Column,
    CommitBegin,
    CommitEnd,
    Cursor,
    Database,
    EventTrigger,
    Extension,
    Function,
    Index,
    NamedArgParameter,
    Policy,
    PreparedStatement,
    Procedure,
    PropertyGraph,
    Role,
    Schema,
    Sequence,
    Server,
    Table,
    Tablespace,
    Trigger,
    Type,
    View,
    Window,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Location {
    pub file: File,
    pub range: TextRange,
    pub kind: LocationKind,
}

impl Location {
    pub(crate) fn new(file: File, range: TextRange, kind: LocationKind) -> Location {
        Location { file, range, kind }
    }

    pub(crate) fn from_node(file: File, node: &SyntaxNode) -> Option<Location> {
        let kind = classify_def_node(node)?;
        Some(Location::new(file, node.text_range(), kind))
    }

    pub(crate) fn to_node(self, db: &dyn Db) -> Option<SyntaxNode> {
        let tree = parse(db, self.file).tree();
        match tree.syntax().covering_element(self.range) {
            rowan::NodeOrToken::Token(token) => token.parent(),
            rowan::NodeOrToken::Node(node) => Some(node.clone()),
        }
    }
}
