use squawk_syntax::ast::NameLike;

use crate::rules::identifier_too_long::MAX_IDENT_BYTES;

#[derive(Debug, Eq, Hash, PartialEq)]
pub(crate) struct Name(String);

impl Name {
    pub(crate) fn from_node(node: &impl NameLike) -> Self {
        Self::from_string(node.text())
    }

    pub(crate) fn from_string(mut text: String) -> Self {
        text.truncate(text.floor_char_boundary(MAX_IDENT_BYTES));
        Self(text)
    }

    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }
}
