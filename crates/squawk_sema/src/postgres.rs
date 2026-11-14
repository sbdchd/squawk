use std::{borrow::Cow, fmt, fmt::Write};

use squawk_syntax::identifier::{MAX_IDENT_BYTES, is_scanner_space, truncate_identifier};

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

pub(crate) const BOOL_INPUTS: [&str; 6] = ["true", "false", "yes", "no", "on", "off"];

// see: https://github.com/postgres/postgres/blob/b82d69abf64fc0c2fc6fdd491d7cecb8244680c2/src/backend/utils/adt/bool.c#L127
pub(crate) fn parse_bool(text: &str) -> Option<bool> {
    let text = text.trim_matches(is_scanner_space);
    let first = text.bytes().next()?;
    let is_prefix_of = |keyword: &str| {
        text.len() <= keyword.len() && keyword[..text.len()].eq_ignore_ascii_case(text)
    };
    match first.to_ascii_lowercase() {
        b't' if is_prefix_of("true") => Some(true),
        b'f' if is_prefix_of("false") => Some(false),
        b'y' if is_prefix_of("yes") => Some(true),
        b'n' if is_prefix_of("no") => Some(false),
        b'o' if text.len() >= 2 && is_prefix_of("on") => Some(true),
        b'o' if text.len() >= 2 && is_prefix_of("off") => Some(false),
        b'1' if text.len() == 1 => Some(true),
        b'0' if text.len() == 1 => Some(false),
        _ => None,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum IntType {
    Int2,
    Int4,
    Int8,
}

impl fmt::Display for IntType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            IntType::Int2 => "int2",
            IntType::Int4 => "int4",
            IntType::Int8 => "int8",
        })
    }
}

impl IntType {
    fn bits(self) -> u32 {
        match self {
            IntType::Int2 => 16,
            IntType::Int4 => 32,
            IntType::Int8 => 64,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum IntInputError {
    InvalidSyntax,
    OutOfRange,
}

// see: https://github.com/postgres/postgres/blob/b82d69abf64fc0c2fc6fdd491d7cecb8244680c2/src/backend/utils/adt/numutils.c#L388
pub(crate) fn parse_int(text: &str, ty: IntType) -> Result<i64, IntInputError> {
    let limit = 1u64 << (ty.bits() - 1);
    let mut rest = text.trim_start_matches(is_scanner_space).as_bytes();
    let neg = match rest {
        [b'-', tail @ ..] => {
            rest = tail;
            true
        }
        [b'+', tail @ ..] => {
            rest = tail;
            false
        }
        _ => false,
    };
    let (radix, digits) = match rest {
        [b'0', b'x' | b'X', digits @ ..] => (16, digits),
        [b'0', b'o' | b'O', digits @ ..] => (8, digits),
        [b'0', b'b' | b'B', digits @ ..] => (2, digits),
        _ => (10, rest),
    };
    let is_digit = |byte: u8| char::from(byte).is_digit(radix);
    let mut magnitude: u64 = 0;
    let mut i = 0;
    while let Some(&byte) = digits.get(i) {
        if let Some(digit) = char::from(byte).to_digit(radix) {
            if magnitude > limit / u64::from(radix) {
                return Err(IntInputError::OutOfRange);
            }
            magnitude = magnitude * u64::from(radix) + u64::from(digit);
            i += 1;
        } else if byte == b'_' {
            if radix == 10 && i == 0 {
                return Err(IntInputError::InvalidSyntax);
            }
            i += 1;
            if !digits.get(i).is_some_and(|&byte| is_digit(byte)) {
                return Err(IntInputError::InvalidSyntax);
            }
        } else {
            break;
        }
    }
    if i == 0
        || !digits[i..]
            .iter()
            .all(|&byte| is_scanner_space(char::from(byte)))
    {
        return Err(IntInputError::InvalidSyntax);
    }
    if neg {
        if magnitude > limit {
            return Err(IntInputError::OutOfRange);
        }
        return 0i64
            .checked_sub_unsigned(magnitude)
            .ok_or(IntInputError::OutOfRange);
    }
    if magnitude >= limit {
        return Err(IntInputError::OutOfRange);
    }
    i64::try_from(magnitude).map_err(|_| IntInputError::OutOfRange)
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
    use super::{
        IntInputError, IntType, make_array_type_name, make_multirange_type_name, parse_bool,
        parse_int,
    };

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

    #[test]
    fn parse_bools() {
        assert_eq!(parse_bool("t"), Some(true));
        assert_eq!(parse_bool("TRUE"), Some(true));
        assert_eq!(parse_bool(" yes\n"), Some(true));
        assert_eq!(parse_bool("of"), Some(false));
        assert_eq!(parse_bool("On"), Some(true));
        assert_eq!(parse_bool("n"), Some(false));
        assert_eq!(parse_bool("0"), Some(false));
        assert_eq!(parse_bool("1"), Some(true));
        assert_eq!(parse_bool(""), None);
        assert_eq!(parse_bool("o"), None);
        assert_eq!(parse_bool("01"), None);
        assert_eq!(parse_bool("truex"), None);
        assert_eq!(parse_bool("ture"), None);
        assert_eq!(parse_bool("é"), None);
    }

    #[test]
    fn parse_ints() {
        use IntInputError::{InvalidSyntax, OutOfRange};
        use IntType::{Int2, Int4, Int8};

        assert_eq!(parse_int(" 42 ", Int4), Ok(42));
        assert_eq!(parse_int("+1", Int4), Ok(1));
        assert_eq!(parse_int("1_000", Int4), Ok(1000));
        assert_eq!(parse_int("0x1F", Int4), Ok(31));
        assert_eq!(parse_int("0x_1F", Int4), Ok(31));
        assert_eq!(parse_int("0o17", Int4), Ok(15));
        assert_eq!(parse_int("0b101", Int4), Ok(5));
        assert_eq!(parse_int("-32768", Int2), Ok(-32768));
        assert_eq!(parse_int("-0x8000", Int2), Ok(-32768));
        assert_eq!(parse_int("-9223372036854775808", Int8), Ok(i64::MIN));
        assert_eq!(parse_int("9223372036854775807", Int8), Ok(i64::MAX));

        assert_eq!(parse_int("", Int4), Err(InvalidSyntax));
        assert_eq!(parse_int("-", Int4), Err(InvalidSyntax));
        assert_eq!(parse_int("- 1", Int4), Err(InvalidSyntax));
        assert_eq!(parse_int("_1", Int4), Err(InvalidSyntax));
        assert_eq!(parse_int("1_", Int4), Err(InvalidSyntax));
        assert_eq!(parse_int("1__0", Int4), Err(InvalidSyntax));
        assert_eq!(parse_int("0X", Int4), Err(InvalidSyntax));
        assert_eq!(parse_int("0b2", Int4), Err(InvalidSyntax));
        assert_eq!(parse_int("1.5", Int4), Err(InvalidSyntax));
        assert_eq!(parse_int("1e3", Int4), Err(InvalidSyntax));

        assert_eq!(parse_int("32768", Int2), Err(OutOfRange));
        assert_eq!(parse_int("0x8000", Int2), Err(OutOfRange));
        assert_eq!(parse_int("-2147483649", Int4), Err(OutOfRange));
        assert_eq!(parse_int("9223372036854775808", Int8), Err(OutOfRange));
        assert_eq!(parse_int("-9223372036854775809", Int8), Err(OutOfRange));
        assert_eq!(parse_int("99999999999999999999x", Int8), Err(OutOfRange));
    }
}
