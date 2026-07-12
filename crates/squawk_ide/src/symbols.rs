use la_arena::Idx;
use squawk_syntax::SyntaxNodePtr;

pub(crate) use crate::name::{Name, Schema};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) enum SymbolKind {
    AccessMethod,
    Aggregate,
    Channel,
    Collation,
    Constraint,
    Conversion,
    Cursor,
    Database,
    EventTrigger,
    Extension,
    ForeignDataWrapper,
    Function,
    Index,
    Language,
    OperatorFamily,
    Policy,
    PreparedStatement,
    Procedure,
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
    TextSearchDictionary,
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
