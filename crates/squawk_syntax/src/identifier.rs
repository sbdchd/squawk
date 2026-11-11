use squawk_rowan::{TextRange, TextSize};

// via: https://github.com/postgres/postgres/blob/228a1f9542792c6533ef74c2e7aefad0da1d9a7a/src/include/pg_config_manual.h#L39C6-L39C6
pub const MAX_IDENT_BYTES: usize = 63;

pub fn truncate_identifier(text: &str) -> &str {
    &text[..text.floor_char_boundary(MAX_IDENT_BYTES)]
}

pub struct Identifier {
    pub name: String,
    pub range: TextRange,
}

pub fn split_identifier_string(text: &str, separator: char) -> Option<Vec<Identifier>> {
    let mut names = vec![];
    let mut chars = text.char_indices().peekable();
    let offset = |chars: &mut std::iter::Peekable<std::str::CharIndices>| {
        TextSize::try_from(chars.peek().map_or(text.len(), |(i, _)| *i)).ok()
    };
    while chars.next_if(|(_, c)| is_scanner_space(*c)).is_some() {}
    if chars.peek().is_none() {
        return Some(names);
    }
    loop {
        let start = offset(&mut chars)?;
        let mut name = String::new();
        if chars.next_if(|(_, c)| *c == '"').is_some() {
            loop {
                match chars.next()?.1 {
                    '"' if chars.next_if(|(_, c)| *c == '"').is_some() => name.push('"'),
                    '"' => break,
                    c => name.push(c),
                }
            }
        } else {
            while let Some((_, c)) =
                chars.next_if(|(_, c)| *c != separator && !is_scanner_space(*c))
            {
                name.push(c.to_ascii_lowercase());
            }
            if name.is_empty() {
                return None;
            }
        }
        let end = offset(&mut chars)?;
        while chars.next_if(|(_, c)| is_scanner_space(*c)).is_some() {}
        name.truncate(truncate_identifier(&name).len());
        names.push(Identifier {
            name,
            range: TextRange::new(start, end),
        });
        match chars.next() {
            Some((_, c)) if c == separator => {
                while chars.next_if(|(_, c)| is_scanner_space(*c)).is_some() {}
            }
            None => return Some(names),
            Some(_) => return None,
        }
    }
}

// see: https://github.com/postgres/postgres/blob/b82d69abf64fc0c2fc6fdd491d7cecb8244680c2/src/backend/parser/scansup.c#L105
pub fn is_scanner_space(c: char) -> bool {
    matches!(c, ' ' | '\t' | '\n' | '\r' | '\x0b' | '\x0c')
}

#[cfg(test)]
mod tests {
    use super::{split_identifier_string, truncate_identifier};

    #[track_caller]
    fn split(text: &str) -> Option<Vec<String>> {
        split_identifier_string(text, ',').map(|names| {
            names
                .into_iter()
                .map(|identifier| identifier.name)
                .collect()
        })
    }

    #[test]
    fn split_identifier_ranges() {
        let text = r#" app . "My""Seq" "#;
        let ranges: Vec<&str> = split_identifier_string(text, '.')
            .unwrap()
            .into_iter()
            .map(|identifier| &text[identifier.range])
            .collect();
        assert_eq!(ranges, vec!["app", r#""My""Seq""#]);
    }

    #[test]
    fn split_identifiers() {
        assert_eq!(split(""), Some(vec![]));
        assert_eq!(split("  "), Some(vec![]));
        assert_eq!(
            split(r#" App , "My""Schema",public "#),
            Some(vec![
                "app".to_string(),
                r#"My"Schema"#.to_string(),
                "public".to_string()
            ])
        );
        assert_eq!(
            split("a,\t\x0bb"),
            Some(vec!["a".to_string(), "b".to_string()])
        );
    }

    #[test]
    fn split_identifiers_empty_quoted_name() {
        assert_eq!(
            split(r#""", a"#),
            Some(vec![String::new(), "a".to_string()])
        );
    }

    #[test]
    fn split_identifiers_non_scanner_whitespace() {
        assert_eq!(
            split("a,\u{a0}b"),
            Some(vec!["a".to_string(), "\u{a0}b".to_string()])
        );
    }

    #[test]
    fn split_identifiers_invalid() {
        assert_eq!(split("a,"), None);
        assert_eq!(split(",a"), None);
        assert_eq!(split("a b"), None);
        assert_eq!(split(r#""a"#), None);
    }

    #[test]
    fn split_identifiers_truncated() {
        let long = "a".repeat(70);
        assert_eq!(split(&long), Some(vec!["a".repeat(63)]));
    }

    #[test]
    fn truncate_identifiers() {
        assert_eq!(truncate_identifier("t"), "t");
        assert_eq!(truncate_identifier(&"a".repeat(63)), "a".repeat(63));
        assert_eq!(truncate_identifier(&"a".repeat(64)), "a".repeat(63));
        let multibyte = format!("{}é", "a".repeat(62));
        assert_eq!(truncate_identifier(&multibyte), "a".repeat(62));
    }
}
