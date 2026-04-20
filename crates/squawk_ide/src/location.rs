use rowan::TextRange;
use squawk_syntax::SyntaxNode;

use crate::{classify::classify_def_node, db::File};

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

impl LocationKind {
    pub(crate) fn from_node(node: &SyntaxNode) -> Option<LocationKind> {
        classify_def_node(node)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Location {
    pub file: File,
    pub range: TextRange,
    pub kind: LocationKind,
}

impl Location {
    pub(crate) fn current(file: File, range: TextRange, kind: LocationKind) -> Location {
        Location { file, range, kind }
    }
}
