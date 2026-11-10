use squawk_syntax::ast::NameLike;

use squawk_syntax::identifier::truncate_identifier;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub(crate) struct Name(String);

impl Name {
    pub(crate) fn from_node(node: &impl NameLike) -> Self {
        Self::from_string(node.text().into_owned())
    }

    pub(crate) fn from_string(mut text: String) -> Self {
        text.truncate(truncate_identifier(&text).len());
        Self(text)
    }

    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }

    pub(crate) fn is_public_schema(&self) -> bool {
        self.0 == "public"
    }
}
