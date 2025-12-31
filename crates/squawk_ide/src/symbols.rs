use la_arena::Idx;
use smol_str::SmolStr;
use squawk_syntax::SyntaxNodePtr;
use std::fmt;

use crate::quote::normalize_identifier;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct Name(pub(crate) SmolStr);

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Schema(pub(crate) Name);

impl Schema {
    pub(crate) fn new(name: impl Into<SmolStr>) -> Self {
        Schema(Name::new(name))
    }
}

impl fmt::Display for Schema {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.0)
    }
}

impl Name {
    pub(crate) fn new(text: impl Into<SmolStr>) -> Self {
        let text = text.into();
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
    Schema,
}

#[derive(Clone, Debug)]
pub(crate) struct Symbol {
    pub(crate) kind: SymbolKind,
    pub(crate) ptr: SyntaxNodePtr,
    pub(crate) schema: Schema,
}

pub(crate) type SymbolId = Idx<Symbol>;

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn name_case_insensitive_compare() {
        assert_eq!(Name::new("foo"), Name::new("FOO"));
    }

    #[test]
    fn name_quote_comparing() {
        assert_eq!(Name::new(r#""foo""#), Name::new("foo"));
    }
}
