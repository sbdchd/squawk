use la_arena::Idx;
use smol_str::SmolStr;
use squawk_syntax::SyntaxNodePtr;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct Name(pub(crate) SmolStr);

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Schema(pub(crate) Name);

impl Schema {
    pub(crate) fn new(name: impl Into<SmolStr>) -> Self {
        Schema(Name::new(name))
    }
}

impl Name {
    pub(crate) fn new(text: impl Into<SmolStr>) -> Self {
        let text = text.into();
        let normalized = normalize_identifier(&text);
        Name(normalized)
    }
}

fn normalize_identifier(text: &str) -> SmolStr {
    if text.starts_with('"') && text.ends_with('"') && text.len() >= 2 {
        text[1..text.len() - 1].into()
    } else {
        text.to_lowercase().into()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) enum SymbolKind {
    Table,
}

#[derive(Clone, Debug)]
pub(crate) struct Symbol {
    pub(crate) kind: SymbolKind,
    pub(crate) ptr: SyntaxNodePtr,
    pub(crate) schema: Schema,
}

pub(crate) type SymbolId = Idx<Symbol>;
