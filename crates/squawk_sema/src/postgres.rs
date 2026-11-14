use std::{borrow::Cow, fmt, fmt::Write};

use squawk_syntax::identifier::{MAX_IDENT_BYTES, truncate_identifier};

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
        let name = make_object_name(name1, name2, Some(&label));
        if !used(&name) {
            return name;
        }
        pass += 1;
    }
}

// see: https://github.com/postgres/postgres/blob/b82d69abf64fc0c2fc6fdd491d7cecb8244680c2/src/backend/commands/indexcmds.c#L2556
fn make_object_name(name1: &str, name2: Option<&str>, label: Option<&str>) -> String {
    let overhead = usize::from(name2.is_some()) + label.map_or(0, |label| label.len() + 1);
    let available = MAX_IDENT_BYTES.saturating_sub(overhead);
    let mut len1 = name1.len();
    let mut len2 = name2.map_or(0, str::len);
    while len1 + len2 > available {
        if len1 > len2 {
            len1 -= 1;
        } else {
            len2 -= 1;
        }
    }
    let mut name = String::with_capacity(MAX_IDENT_BYTES);
    name.push_str(clip_to_char_boundary(name1, len1));
    if let Some(name2) = name2 {
        name.push('_');
        name.push_str(clip_to_char_boundary(name2, len2));
    }
    if let Some(label) = label {
        name.push('_');
        name.push_str(label);
    }
    name
}

// see: https://github.com/postgres/postgres/blob/b82d69abf64fc0c2fc6fdd491d7cecb8244680c2/src/backend/catalog/pg_type.c#L844
pub(crate) fn make_array_type_name(type_name: &str, mut used: impl FnMut(&str) -> bool) -> String {
    let mut name = make_object_name("", Some(type_name), None);
    let mut pass = 0;
    while used(&name) {
        pass += 1;
        name = make_object_name("", Some(type_name), Some(&pass.to_string()));
    }
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
            let len = range_type_name.len().min(MAX_IDENT_BYTES - SUFFIX.len());
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
    use super::{make_array_type_name, make_multirange_type_name};

    #[test]
    fn array_type_names() {
        assert_eq!(make_array_type_name("x", |_| false), "_x");
        assert_eq!(make_array_type_name("x", |name| name == "_x"), "_x_1");
        assert_eq!(
            make_array_type_name("x", |name| ["_x", "_x_1"].contains(&name)),
            "_x_2"
        );
        assert_eq!(make_array_type_name("_y", |_| false), "__y");
        let long = "a".repeat(63);
        assert_eq!(
            make_array_type_name(&long, |_| false),
            format!("_{}", "a".repeat(62))
        );
        assert_eq!(
            make_array_type_name(&long, |name| name.len() == 63 && !name.ends_with("_1")),
            format!("_{}_1", "a".repeat(60))
        );
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
