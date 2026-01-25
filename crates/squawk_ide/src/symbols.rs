use la_arena::Idx;
use smol_str::SmolStr;
use squawk_syntax::{SyntaxNodePtr, ast};
use std::fmt;

use crate::quote::normalize_identifier;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct Name(pub(crate) SmolStr);

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Schema(pub(crate) Name);

impl Schema {
    pub(crate) fn new(name: impl Into<SmolStr>) -> Self {
        Schema(Name::from_string(name))
    }
}

impl fmt::Display for Schema {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.0)
    }
}

impl Name {
    pub(crate) fn from_string(text: impl Into<SmolStr>) -> Self {
        let text = text.into();
        let normalized = normalize_identifier(&text);
        Name(normalized.into())
    }
    pub(crate) fn from_node(node: &impl ast::NameLike) -> Self {
        let text = node.syntax().text().to_string();
        let normalized = normalize_identifier(&text);
        Name(normalized.into())
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) enum SymbolKind {
    Table,
    Index,
    Function,
    Aggregate,
    Procedure,
    Schema,
    Type,
    View,
    Sequence,
    Cursor,
    PreparedStatement,
    Channel,
    Tablespace,
    Database,
    Server,
    Extension,
    Trigger,
    EventTrigger,
    Role,
    Policy,
}

#[derive(Clone, Debug)]
pub(crate) struct Symbol {
    pub(crate) kind: SymbolKind,
    pub(crate) ptr: SyntaxNodePtr,
    pub(crate) schema: Option<Schema>,
    pub(crate) params: Option<Vec<Name>>,
    pub(crate) table: Option<Name>,
}

pub(crate) type SymbolId = Idx<Symbol>;

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn name_case_insensitive_compare() {
        assert_eq!(Name::from_string("foo"), Name::from_string("FOO"));
    }

    #[test]
    fn name_quote_comparing() {
        assert_eq!(Name::from_string(r#""foo""#), Name::from_string("foo"));
    }
}
