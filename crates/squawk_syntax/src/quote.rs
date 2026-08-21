use crate::SyntaxNode;
use crate::generated::keywords::{AS_LABEL_KEYWORDS, RESERVED_KEYWORDS, TYPE_FUNC_NAME_KEYWORDS};

pub fn quote_string_literal(text: &str) -> String {
    format!("'{}'", text.replace('\'', "''"))
}

fn quote(text: &str) -> String {
    format!(r#""{}""#, text.replace('"', r#""""#))
}

pub fn quote_column_alias(text: &str) -> String {
    if needs_quoting(text) {
        quote(text)
    } else {
        text.to_string()
    }
}

pub fn quote_bare_column_alias(text: &str) -> String {
    if needs_quoting(text) || is_as_label_word(text) {
        quote(text)
    } else {
        text.to_string()
    }
}

pub fn quote_ident(text: &str) -> String {
    if needs_quoting(text) || is_reserved_word(text) || is_type_func_name_word(text) {
        quote(text)
    } else {
        text.to_string()
    }
}

pub fn unquote_ident(node: &SyntaxNode) -> Option<String> {
    let text = node.text().to_string();

    if !text.starts_with('"') || !text.ends_with('"') {
        return None;
    }

    let text = &text[1..text.len() - 1];

    if is_reserved_word(text) || is_type_func_name_word(text) {
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

pub fn needs_quoting(text: &str) -> bool {
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

pub fn is_reserved_word(text: &str) -> bool {
    RESERVED_KEYWORDS
        .binary_search(&text.to_ascii_lowercase().as_str())
        .is_ok()
}

fn is_type_func_name_word(text: &str) -> bool {
    TYPE_FUNC_NAME_KEYWORDS
        .binary_search(&text.to_ascii_lowercase().as_str())
        .is_ok()
}

fn is_as_label_word(text: &str) -> bool {
    AS_LABEL_KEYWORDS
        .binary_search(&text.to_ascii_lowercase().as_str())
        .is_ok()
}

pub fn strip_quotes(text: &str) -> Option<&str> {
    text.strip_prefix('\'')?.strip_suffix('\'')
}

pub fn strip_prefixed_quotes(text: &str, prefix: [char; 2]) -> Option<&str> {
    strip_quotes(text.strip_prefix(prefix)?)
}

pub fn strip_unicode_esc_prefix(text: &str) -> Option<&str> {
    strip_quotes(text.strip_prefix(['u', 'U'])?.strip_prefix('&')?)
}

pub fn dollar_quote_tag(text: &str) -> Option<&str> {
    text.strip_prefix('$')?.split_once('$').map(|(tag, _)| tag)
}

pub fn strip_dollar_quotes(text: &str) -> Option<&str> {
    let tag = dollar_quote_tag(text)?;
    let body = &text[tag.len() + 2..];
    let closing = format!("${tag}$");
    body.strip_suffix(&closing)
}

#[cfg(test)]
mod tests {
    use insta::assert_snapshot;

    use super::*;

    #[test]
    fn quote_string_literal_escapes_apostrophes() {
        assert_snapshot!(quote_string_literal("it's"), @"'it''s'");
    }

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

    #[test]
    fn quote_bare_column_alias_quotes_keywords_that_need_an_as() {
        assert_snapshot!(quote_bare_column_alias("filter"), @r#""filter""#);
        assert_snapshot!(quote_bare_column_alias("day"), @r#""day""#);
        // also reserved
        assert_snapshot!(quote_bare_column_alias("array"), @r#""array""#);
    }

    #[test]
    fn quote_bare_column_alias_doesnt_quote_bare_label_keywords() {
        assert_snapshot!(quote_bare_column_alias("between"), @"between");
        assert_snapshot!(quote_bare_column_alias("all"), @"all");
        assert_snapshot!(quote_bare_column_alias("left"), @"left");
        assert_snapshot!(quote_bare_column_alias("col_name"), @"col_name");
    }

    #[test]
    fn quote_ident_doesnt_quote_simple_identifiers() {
        assert_snapshot!(quote_ident("col_name"), @"col_name");
        assert_snapshot!(quote_ident("users"), @"users");
        assert_snapshot!(quote_ident("t2$"), @"t2$");
    }

    #[test]
    fn quote_ident_doesnt_quote_column_or_table_keywords() {
        // unreserved
        assert_snapshot!(quote_ident("data"), @"data");
        assert_snapshot!(quote_ident("value"), @"value");
        // col name
        assert_snapshot!(quote_ident("int"), @"int");
    }

    #[test]
    fn quote_ident_quotes_reserved_words() {
        assert_snapshot!(quote_ident("select"), @r#""select""#);
        assert_snapshot!(quote_ident("array"), @r#""array""#);
    }

    #[test]
    fn quote_ident_quotes_type_func_name_words() {
        assert_snapshot!(quote_ident("left"), @r#""left""#);
        assert_snapshot!(quote_ident("join"), @r#""join""#);
    }

    #[test]
    fn quote_ident_quotes_names_that_dont_fold_to_themselves() {
        assert_snapshot!(quote_ident("Mixed"), @r#""Mixed""#);
        assert_snapshot!(quote_ident("has space"), @r#""has space""#);
        assert_snapshot!(quote_ident(""), @r#""""#);
        assert_snapshot!(quote_ident(r#"foo"bar"#), @r#""foo""bar""#);
    }
}
