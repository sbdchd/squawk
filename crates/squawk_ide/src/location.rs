use rowan::TextRange;
use salsa::Database as Db;
use squawk_syntax::SyntaxNode;
use squawk_syntax::ast::AstNode;

use crate::{
    classify::classify_def_node,
    db::{FileId, embedded_body, parse},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocationKind {
    AccessMethod,
    Aggregate,
    CaseExpr,
    Channel,
    Collation,
    Column,
    CommitBegin,
    CommitEnd,
    Constraint,
    Conversion,
    Cursor,
    Database,
    EventTrigger,
    Extension,
    ElementTable,
    ForeignDataWrapper,
    Function,
    Index,
    JsonPath,
    Label,
    Language,
    NamedArgParameter,
    Operator,
    OperatorClass,
    OperatorFamily,
    Policy,
    PreparedStatement,
    PreparedTransaction,
    Procedure,
    Property,
    PropertyGraph,
    Publication,
    Role,
    Rule,
    Savepoint,
    Schema,
    Sequence,
    Server,
    Statistics,
    Subscription,
    Table,
    Tablespace,
    TextSearchConfiguration,
    TextSearchDictionary,
    TextSearchParser,
    TextSearchTemplate,
    Trigger,
    Type,
    View,
    Window,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Location {
    pub file: FileId,
    pub range: TextRange,
    pub kind: LocationKind,
}

impl Location {
    pub(crate) fn new(file: impl Into<FileId>, range: TextRange, kind: LocationKind) -> Location {
        Location {
            file: file.into(),
            range,
            kind,
        }
    }

    pub(crate) fn from_node(file: impl Into<FileId>, node: &SyntaxNode) -> Option<Location> {
        let kind = classify_def_node(node)?;
        Some(Location::new(file, node.text_range(), kind))
    }

    pub(crate) fn upmap(self, db: &dyn Db) -> Location {
        let mut location = self;
        loop {
            match location.file {
                FileId::File(_) | FileId::Completion(_) => return location,
                FileId::Embedded(embedded) => {
                    let Some(body) = embedded_body(db, embedded) else {
                        return location;
                    };
                    location = Location::new(
                        embedded.parent(db),
                        body.source_range(location.range),
                        location.kind,
                    );
                }
            }
        }
    }

    pub(crate) fn to_node(self, db: &dyn Db) -> Option<SyntaxNode> {
        let tree = parse(db, self.file).tree();
        match tree.syntax().covering_element(self.range) {
            rowan::NodeOrToken::Token(token) => token.parent(),
            rowan::NodeOrToken::Node(node) => Some(node.clone()),
        }
    }
}
