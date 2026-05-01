use la_arena::Idx;
use squawk_syntax::SyntaxNodePtr;

pub(crate) use crate::name::{Name, Schema};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) enum SymbolKind {
    Aggregate,
    Channel,
    Cursor,
    Database,
    EventTrigger,
    Extension,
    Function,
    Index,
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
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Symbol {
    pub(crate) kind: SymbolKind,
    pub(crate) ptr: SyntaxNodePtr,
    pub(crate) schema: Option<Schema>,
    pub(crate) params: Option<Vec<Name>>,
    pub(crate) table: Option<Name>,
}

pub(crate) type SymbolId = Idx<Symbol>;
