use std::{borrow::Cow, fmt, fmt::Write};

use rowan::{TextRange, TextSize};

const MAX_IDENTIFIER_LEN: usize = 63;

// see: https://github.com/postgres/postgres/blob/b82d69abf64fc0c2fc6fdd491d7cecb8244680c2/src/backend/commands/indexcmds.c#L2644
pub(crate) fn choose_relation_name(
    name1: &str,
    name2: Option<&str>,
    label: &str,
    mut relation_exists: impl FnMut(&str) -> bool,
    mut constraint_exists: impl FnMut(&str) -> bool,
) -> String {
    choose_name(name1, name2, label, |name| {
        relation_exists(name) || constraint_exists(name)
    })
}

// see: https://github.com/postgres/postgres/blob/b82d69abf64fc0c2fc6fdd491d7cecb8244680c2/src/backend/catalog/pg_constraint.c#L513
pub(crate) fn choose_constraint_name(
    name1: &str,
    name2: Option<&str>,
    label: &str,
    constraint_exists: impl FnMut(&str) -> bool,
) -> String {
    choose_name(name1, name2, label, constraint_exists)
}

// see: https://github.com/postgres/postgres/blob/b82d69abf64fc0c2fc6fdd491d7cecb8244680c2/src/backend/commands/statscmds.c#L840
pub(crate) fn choose_extended_statistic_name(
    name1: &str,
    name2: &str,
    statistic_exists: impl FnMut(&str) -> bool,
) -> String {
    choose_name(name1, Some(name2), "stat", statistic_exists)
}

fn choose_name(
    name1: &str,
    name2: Option<&str>,
    label: &str,
    mut used: impl FnMut(&str) -> bool,
) -> String {
    let mut pass = 0;
    loop {
        let label = match pass {
            0 => Cow::Borrowed(label),
            _ => Cow::Owned(format!("{label}{pass}")),
        };
        let name = make_object_name(name1, name2, &label);
        if !used(&name) {
            return name;
        }
        pass += 1;
    }
}

// see: https://github.com/postgres/postgres/blob/b82d69abf64fc0c2fc6fdd491d7cecb8244680c2/src/backend/commands/indexcmds.c#L2556
fn make_object_name(name1: &str, name2: Option<&str>, label: &str) -> String {
    let overhead = usize::from(name2.is_some()) + label.len() + 1;
    let available = MAX_IDENTIFIER_LEN.saturating_sub(overhead);
    let mut len1 = name1.len();
    let mut len2 = name2.map_or(0, str::len);
    while len1 + len2 > available {
        if len1 > len2 {
            len1 -= 1;
        } else {
            len2 -= 1;
        }
    }
    let mut name = String::with_capacity(MAX_IDENTIFIER_LEN);
    name.push_str(clip_to_char_boundary(name1, len1));
    if let Some(name2) = name2 {
        name.push('_');
        name.push_str(clip_to_char_boundary(name2, len2));
    }
    name.push('_');
    name.push_str(label);
    name
}

// see: https://github.com/postgres/postgres/blob/b82d69abf64fc0c2fc6fdd491d7cecb8244680c2/src/backend/commands/indexcmds.c#L2767
pub(crate) fn join_names(names: impl Iterator<Item = impl fmt::Display>) -> String {
    let mut joined = String::new();
    for name in names {
        if !joined.is_empty() {
            joined.push('_');
        }
        let _ = write!(joined, "{name}");
    }
    joined
}

pub(crate) struct Identifier {
    pub(crate) name: String,
    pub(crate) range: TextRange,
}

// see: https://github.com/postgres/postgres/blob/b82d69abf64fc0c2fc6fdd491d7cecb8244680c2/src/backend/utils/adt/varlena.c#L2870
pub(crate) fn split_identifier_string(text: &str, separator: char) -> Option<Vec<Identifier>> {
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
fn is_scanner_space(c: char) -> bool {
    matches!(c, ' ' | '\t' | '\n' | '\r' | '\x0b' | '\x0c')
}

// see: https://github.com/postgres/postgres/blob/b82d69abf64fc0c2fc6fdd491d7cecb8244680c2/src/backend/parser/scansup.c#L81
pub(crate) fn truncate_identifier(text: &str) -> &str {
    if text.len() > MAX_IDENTIFIER_LEN {
        clip_to_char_boundary(text, MAX_IDENTIFIER_LEN)
    } else {
        text
    }
}

// see: https://github.com/postgres/postgres/blob/b82d69abf64fc0c2fc6fdd491d7cecb8244680c2/src/backend/catalog/pg_type.c#L948
pub(crate) fn make_multirange_type_name(range_type_name: &str) -> String {
    const SUFFIX: &str = "_multirange";
    let mut name = match range_type_name.find("range") {
        Some(start) => format!(
            "{}multi{}",
            &range_type_name[..start],
            &range_type_name[start..]
        ),
        None => {
            let len = range_type_name.len().min(MAX_IDENTIFIER_LEN - SUFFIX.len());
            format!("{}{SUFFIX}", clip_to_char_boundary(range_type_name, len))
        }
    };
    name.truncate(truncate_identifier(&name).len());
    name
}

fn clip_to_char_boundary(text: &str, mut len: usize) -> &str {
    while !text.is_char_boundary(len) {
        len -= 1;
    }
    &text[..len]
}

#[cfg(test)]
mod tests {
    use super::{make_multirange_type_name, split_identifier_string, truncate_identifier};

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

    #[test]
    fn multirange_type_names() {
        assert_eq!(make_multirange_type_name("int4range"), "int4multirange");
        assert_eq!(
            make_multirange_type_name("floatrange_x"),
            "floatmultirange_x"
        );
        assert_eq!(make_multirange_type_name("mood"), "mood_multirange");
        assert_eq!(
            make_multirange_type_name(&"a".repeat(60)),
            format!("{}_multirange", "a".repeat(52))
        );
        assert_eq!(
            make_multirange_type_name(&format!("{}range", "a".repeat(58))),
            format!("{}multi", "a".repeat(58))
        );
    }
}
