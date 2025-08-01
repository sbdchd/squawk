/// Postgres Identifiers are case insensitive unless they're quoted.
///
/// This type handles the casing rules for us to make comparisions easier.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct Identifier(String);

impl Identifier {
    // TODO: we need to handle more advanced identifiers like:
    // U&"d!0061t!+000061" UESCAPE '!'
    pub fn new(s: &str) -> Self {
        let normalized = if s.starts_with('"') && s.ends_with('"') {
            s[1..s.len() - 1].to_string()
        } else {
            s.to_lowercase()
        };
        Identifier(normalized)
    }
}

#[cfg(test)]
mod test {
    use crate::identifier::Identifier;

    #[test]
    fn case_folds_correctly() {
        // https://www.postgresql.org/docs/current/sql-syntax-lexical.html#SQL-SYNTAX-IDENTIFIERS
        // For example, the identifiers FOO, foo, and "foo" are considered the
        // same by PostgreSQL, but "Foo" and "FOO" are different from these
        // three and each other.
        assert_eq!(Identifier::new("FOO"), Identifier::new("foo"));
        assert_eq!(Identifier::new(r#""foo""#), Identifier::new("foo"));
        assert_eq!(Identifier::new(r#""foo""#), Identifier::new("FOO"));
    }
}
