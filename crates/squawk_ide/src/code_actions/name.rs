use smol_str::{SmolStr, StrExt};
use squawk_syntax::ast;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct Name(pub(crate) SmolStr);

impl Name {
    pub(crate) fn from_string(text: impl Into<SmolStr>) -> Self {
        let text = text.into();
        let text = if let Some(quoted) = text.strip_prefix('"').and_then(|t| t.strip_suffix('"')) {
            quoted.replace_smolstr(r#"""""#, "\"")
        } else if text.bytes().any(|b| b.is_ascii_uppercase()) {
            text.to_ascii_lowercase_smolstr()
        } else {
            text
        };
        Name(text)
    }

    pub(crate) fn from_node(node: &impl ast::NameLike) -> Self {
        let text = node.text();
        Name(text.into())
    }
}

fn debug_assert_normalized(text: &str) {
    debug_assert!(
        !text.bytes().any(|b| b.is_ascii_uppercase()),
        "un-normalized str used as a Name: {text:?}"
    );
}

impl PartialEq<str> for Name {
    fn eq(&self, other: &str) -> bool {
        debug_assert_normalized(other);
        self.0 == other
    }
}

impl PartialEq<Name> for str {
    fn eq(&self, other: &Name) -> bool {
        other == self
    }
}

impl PartialEq<&str> for Name {
    fn eq(&self, other: &&str) -> bool {
        self == *other
    }
}

impl PartialEq<Name> for &str {
    fn eq(&self, other: &Name) -> bool {
        other == *self
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn name_case_insensitive_compare() {
        assert_eq!(Name::from_string("foo"), Name::from_string(r#""foo""#));
    }

    #[test]
    fn name_quote_comparing() {
        assert_eq!(Name::from_string(r#""foo""#), Name::from_string("foo"));
    }

    #[test]
    fn name_str_comparing() {
        assert_eq!(Name::from_string("FOO"), "foo");
        assert_eq!("foo", Name::from_string(r#""foo""#));
        assert_ne!(Name::from_string(r#""FOO""#), "foo");
    }
}
