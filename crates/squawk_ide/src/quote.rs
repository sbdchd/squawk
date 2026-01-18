use squawk_syntax::SyntaxNode;

use crate::generated::keywords::RESERVED_KEYWORDS;

pub(crate) fn quote_column_alias(text: &str) -> String {
    if needs_quoting(text) {
        format!(r#""{}""#, text.replace('"', r#""""#))
    } else {
        text.to_string()
    }
}

pub(crate) fn unquote_ident(node: &SyntaxNode) -> Option<String> {
    let text = node.text().to_string();

    if !text.starts_with('"') || !text.ends_with('"') {
        return None;
    }

    let text = &text[1..text.len() - 1];

    if is_reserved_word(text) {
        return None;
    }

    if text.is_empty() {
        return None;
    }

    let mut chars = text.chars();

    // see: https://www.postgresql.org/docs/18/sql-syntax-lexical.html#SQL-SYNTAX-IDENTIFIERS
    match chars.next() {
        Some(c) if c.is_lowercase() || c == '_' => {}
        _ => return None,
    }

    for c in chars {
        if c.is_lowercase() || c.is_ascii_digit() || c == '_' || c == '$' {
            continue;
        }
        return None;
    }

    Some(text.to_string())
}

fn needs_quoting(text: &str) -> bool {
    if text.is_empty() {
        return true;
    }

    // Column labels in AS clauses allow all keywords, so we don't need to check
    // for reserved words. See PostgreSQL grammar:
    // ColLabel: IDENT | unreserved_keyword | col_name_keyword | type_func_name_keyword | reserved_keyword

    let mut chars = text.chars();

    match chars.next() {
        Some(c) if c.is_lowercase() || c == '_' => {}
        _ => return true,
    }

    for c in chars {
        if c.is_lowercase() || c.is_ascii_digit() || c == '_' || c == '$' {
            continue;
        }
        return true;
    }

    false
}

pub(crate) fn is_reserved_word(text: &str) -> bool {
    RESERVED_KEYWORDS
        .binary_search(&text.to_lowercase().as_str())
        .is_ok()
}

pub(crate) fn normalize_identifier(text: &str) -> String {
    if text.starts_with('"') && text.ends_with('"') && text.len() >= 2 {
        text[1..text.len() - 1].to_string()
    } else {
        text.to_lowercase()
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_snapshot;

    use super::*;

    #[test]
    fn quote_column_alias_handles_embedded_quotes() {
        assert_snapshot!(quote_column_alias(r#"foo"bar"#), @r#""foo""bar""#);
    }

    #[test]
    fn quote_column_alias_doesnt_quote_reserved_words() {
        // Keywords are allowed as column labels in AS clauses
        assert_snapshot!(quote_column_alias("case"), @"case");
        assert_snapshot!(quote_column_alias("array"), @"array");
    }

    #[test]
    fn quote_column_alias_doesnt_quote_simple_identifiers() {
        assert_snapshot!(quote_column_alias("col_name"), @"col_name");
    }

    #[test]
    fn quote_column_alias_handles_special_column_name() {
        assert_snapshot!(quote_column_alias("?column?"), @r#""?column?""#);
    }
}
