use std::{borrow::Cow, cmp::Ordering, fmt, fmt::Write};

use num_bigint::BigUint;
use squawk_rowan::{TextRange, TextSize};

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

const DEC_DIGITS: i64 = 4;
const NUMERIC_DSCALE_MAX: i64 = 0x3FFF;
const NUMERIC_MAX_DECIMAL_DIGITS: u32 = 131_072;
const NUMERIC_MAX_DECIMAL_BITS: u64 = 435_412;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct NumericTypmod {
    pub(crate) precision: i32,
    pub(crate) scale: i32,
}

impl NumericTypmod {
    pub(crate) fn new(precision: i64, scale: i64) -> Option<NumericTypmod> {
        if !(1..=1000).contains(&precision) || !(-1000..=1000).contains(&scale) {
            return None;
        }
        Some(NumericTypmod {
            precision: i32::try_from(precision).ok()?,
            scale: i32::try_from(scale).ok()?,
        })
    }

    pub(crate) fn max_digits(self) -> i32 {
        self.precision - self.scale
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NumericInputError {
    InvalidSyntax,
    FieldOverflow,
    InfiniteOverflow,
    FormatOverflow,
}

struct Decimal<'a> {
    text: &'a [u8],
    weight: i64,
    dscale: i64,
}

impl Decimal<'_> {
    fn digits(&self) -> impl Iterator<Item = (i64, u8)> + '_ {
        self.text
            .iter()
            .filter(|byte| byte.is_ascii_digit())
            .zip(0..)
            .map(|(byte, i)| (self.weight - i, byte - b'0'))
    }

    // see: https://github.com/postgres/postgres/blob/b82d69abf64fc0c2fc6fdd491d7cecb8244680c2/src/backend/utils/adt/numeric.c#L7584
    fn check_format(&self) -> Result<(), NumericInputError> {
        if self.dscale > NUMERIC_DSCALE_MAX {
            return Err(NumericInputError::FormatOverflow);
        }
        if let Some((exponent, _)) = self.digits().find(|(_, digit)| *digit != 0)
            && i16::try_from(exponent.div_euclid(DEC_DIGITS)).is_err()
        {
            return Err(NumericInputError::FormatOverflow);
        }
        Ok(())
    }

    // see: https://github.com/postgres/postgres/blob/b82d69abf64fc0c2fc6fdd491d7cecb8244680c2/src/backend/utils/adt/numeric.c#L1229
    fn check_typmod(&self, typmod: NumericTypmod) -> Result<(), NumericInputError> {
        let scale = i64::from(typmod.scale);
        let mut leading = None;
        let mut all_nines = true;
        let mut round_up = false;
        for (exponent, digit) in self.digits() {
            if exponent < -scale {
                round_up = exponent == -scale - 1 && digit >= 5;
                break;
            }
            if leading.is_none() && digit != 0 {
                leading = Some(exponent);
            }
            if leading.is_some() && digit != 9 {
                all_nines = false;
            }
        }
        let leading = match leading {
            Some(exponent) if round_up && all_nines => exponent + 1,
            Some(exponent) => exponent,
            None if round_up => -scale,
            None => return Ok(()),
        };
        if leading >= i64::from(typmod.max_digits()) {
            return Err(NumericInputError::FieldOverflow);
        }
        Ok(())
    }
}

// see: https://github.com/postgres/postgres/blob/b82d69abf64fc0c2fc6fdd491d7cecb8244680c2/src/backend/utils/adt/numeric.c#L626
pub(crate) fn check_numeric(
    text: &str,
    typmod: Option<NumericTypmod>,
) -> Result<(), NumericInputError> {
    let text = text.trim_start_matches(is_scanner_space).as_bytes();
    let unsigned = match text {
        [b'+' | b'-', rest @ ..] => rest,
        _ => text,
    };
    if !matches!(unsigned.first(), Some(b'0'..=b'9' | b'.')) {
        let starts_with = |text: &[u8], prefix: &[u8]| {
            text.get(..prefix.len())
                .is_some_and(|start| start.eq_ignore_ascii_case(prefix))
        };
        let (rest, infinite) = if starts_with(text, b"nan") {
            (&text[3..], false)
        } else if starts_with(unsigned, b"infinity") {
            (&unsigned[8..], true)
        } else if starts_with(unsigned, b"inf") {
            (&unsigned[3..], true)
        } else {
            return Err(NumericInputError::InvalidSyntax);
        };
        if !is_blank(rest) {
            return Err(NumericInputError::InvalidSyntax);
        }
        if infinite && typmod.is_some() {
            return Err(NumericInputError::InfiniteOverflow);
        }
        return Ok(());
    }
    let radix = match unsigned {
        [b'0', b'x' | b'X', ..] => 16,
        [b'0', b'o' | b'O', ..] => 8,
        [b'0', b'b' | b'B', ..] => 2,
        _ => 10,
    };
    let mut buf;
    let decimal = if radix == 10 {
        let (decimal, rest) = parse_decimal(unsigned)?;
        if !is_blank(rest) {
            return Err(NumericInputError::InvalidSyntax);
        }
        decimal
    } else {
        let (value, rest) = parse_non_decimal(&unsigned[2..], radix)?;
        if !is_blank(rest) {
            return Err(NumericInputError::InvalidSyntax);
        }
        match value {
            NonDecimal::Small(value) => {
                buf = [0u8; 39];
                decimal_from_u128(value, &mut buf)
            }
            NonDecimal::Big(digits) => return check_big_integer(digits, radix, typmod),
        }
    };
    decimal.check_format()?;
    match typmod {
        Some(typmod) => decimal.check_typmod(typmod),
        None => Ok(()),
    }
}

// see: https://github.com/postgres/postgres/blob/b82d69abf64fc0c2fc6fdd491d7cecb8244680c2/src/backend/utils/adt/numeric.c#L6817
fn parse_decimal(text: &[u8]) -> Result<(Decimal<'_>, &[u8]), NumericInputError> {
    let is_digit_at = |i: usize| text.get(i).is_some_and(u8::is_ascii_digit);
    let mut i = 0;
    let mut have_dp = false;
    if text.first() == Some(&b'.') {
        have_dp = true;
        i = 1;
    }
    if !is_digit_at(i) {
        return Err(NumericInputError::InvalidSyntax);
    }
    let mut int_digits: i64 = 0;
    let mut frac_digits: i64 = 0;
    while let Some(&byte) = text.get(i) {
        match byte {
            b'0'..=b'9' => {
                if have_dp {
                    frac_digits += 1;
                } else {
                    int_digits += 1;
                }
                i += 1;
            }
            b'.' => {
                if have_dp {
                    return Err(NumericInputError::InvalidSyntax);
                }
                have_dp = true;
                i += 1;
                if text.get(i) == Some(&b'_') {
                    return Err(NumericInputError::InvalidSyntax);
                }
            }
            b'_' => {
                i += 1;
                if !is_digit_at(i) {
                    return Err(NumericInputError::InvalidSyntax);
                }
            }
            _ => break,
        }
    }
    let mantissa = &text[..i];
    let mut exponent: i64 = 0;
    if let Some(b'e' | b'E') = text.get(i) {
        i += 1;
        let neg = match text.get(i) {
            Some(b'+') => {
                i += 1;
                false
            }
            Some(b'-') => {
                i += 1;
                true
            }
            _ => false,
        };
        if !is_digit_at(i) {
            return Err(NumericInputError::InvalidSyntax);
        }
        while let Some(&byte) = text.get(i) {
            match byte {
                b'0'..=b'9' => {
                    exponent = exponent * 10 + i64::from(byte - b'0');
                    if exponent > i64::from(i32::MAX / 2) {
                        return Err(NumericInputError::FormatOverflow);
                    }
                    i += 1;
                }
                b'_' => {
                    i += 1;
                    if !is_digit_at(i) {
                        return Err(NumericInputError::InvalidSyntax);
                    }
                }
                _ => break,
            }
        }
        if neg {
            exponent = -exponent;
        }
    }
    let decimal = Decimal {
        text: mantissa,
        weight: int_digits - 1 + exponent,
        dscale: (frac_digits - exponent).max(0),
    };
    Ok((decimal, &text[i..]))
}

enum NonDecimal<'a> {
    Small(u128),
    Big(&'a [u8]),
}

// see: https://github.com/postgres/postgres/blob/b82d69abf64fc0c2fc6fdd491d7cecb8244680c2/src/backend/utils/adt/numeric.c#L7047
fn parse_non_decimal(
    text: &[u8],
    radix: u32,
) -> Result<(NonDecimal<'_>, &[u8]), NumericInputError> {
    let is_digit_at = |i: usize| {
        text.get(i)
            .is_some_and(|&byte| char::from(byte).is_digit(radix))
    };
    let mut value = Some(0u128);
    let mut i = 0;
    while let Some(&byte) = text.get(i) {
        if let Some(digit) = char::from(byte).to_digit(radix) {
            value = value
                .and_then(|value| value.checked_mul(u128::from(radix)))
                .and_then(|value| value.checked_add(u128::from(digit)));
            i += 1;
        } else if byte == b'_' {
            i += 1;
            if !is_digit_at(i) {
                return Err(NumericInputError::InvalidSyntax);
            }
        } else {
            break;
        }
    }
    if i == 0 {
        return Err(NumericInputError::InvalidSyntax);
    }
    let value = match value {
        Some(value) => NonDecimal::Small(value),
        None => NonDecimal::Big(&text[..i]),
    };
    Ok((value, &text[i..]))
}

fn check_big_integer(
    digits: &[u8],
    radix: u32,
    typmod: Option<NumericTypmod>,
) -> Result<(), NumericInputError> {
    let mut values = Vec::with_capacity(digits.len());
    values.extend(
        digits
            .iter()
            .filter_map(|&byte| char::from(byte).to_digit(radix))
            .filter_map(|digit| u8::try_from(digit).ok()),
    );
    let value = BigUint::from_radix_be(&values, radix).ok_or(NumericInputError::InvalidSyntax)?;
    let ten = BigUint::from(10u32);
    let overflows = match value.bits().cmp(&NUMERIC_MAX_DECIMAL_BITS) {
        Ordering::Less => false,
        Ordering::Equal => value >= ten.pow(NUMERIC_MAX_DECIMAL_DIGITS),
        Ordering::Greater => true,
    };
    if overflows {
        return Err(NumericInputError::FormatOverflow);
    }
    let Some(typmod) = typmod else {
        return Ok(());
    };
    let Ok(max_digits) = u32::try_from(typmod.max_digits()) else {
        return Err(NumericInputError::FieldOverflow);
    };
    let half = match u32::try_from(-typmod.scale - 1) {
        Ok(exponent) => BigUint::from(5u32) * ten.pow(exponent),
        Err(_) => BigUint::ZERO,
    };
    if value + half >= ten.pow(max_digits) {
        return Err(NumericInputError::FieldOverflow);
    }
    Ok(())
}

fn decimal_from_u128(mut value: u128, buf: &mut [u8; 39]) -> Decimal<'_> {
    let mut start = buf.len();
    let mut weight = -1;
    loop {
        start -= 1;
        weight += 1;
        buf[start] = b'0' + (value % 10) as u8;
        value /= 10;
        if value == 0 {
            break;
        }
    }
    Decimal {
        text: &buf[start..],
        weight,
        dscale: 0,
    }
}

fn is_blank(text: &[u8]) -> bool {
    text.iter().all(|&byte| is_scanner_space(char::from(byte)))
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
    use num_bigint::BigUint;

    use super::{
        IntInputError, IntType, NUMERIC_MAX_DECIMAL_BITS, NUMERIC_MAX_DECIMAL_DIGITS,
        NumericInputError, NumericTypmod, check_numeric, make_array_type_name,
        make_multirange_type_name, parse_bool, parse_int, split_identifier_string,
        truncate_identifier,
    };

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

    #[test]
    fn check_numerics() {
        use NumericInputError::{FieldOverflow, FormatOverflow, InfiniteOverflow, InvalidSyntax};

        let typmod = |precision, scale| NumericTypmod::new(precision, scale);

        assert_eq!(check_numeric(" 1.5 ", None), Ok(()));
        assert_eq!(check_numeric("+.5", None), Ok(()));
        assert_eq!(check_numeric("1.", None), Ok(()));
        assert_eq!(check_numeric("1_000.000_1", None), Ok(()));
        assert_eq!(check_numeric("1.e5", None), Ok(()));
        assert_eq!(check_numeric("1e5_0", None), Ok(()));
        assert_eq!(check_numeric("0x_F", None), Ok(()));
        assert_eq!(check_numeric("-0b11", None), Ok(()));
        assert_eq!(check_numeric("nan", None), Ok(()));
        assert_eq!(check_numeric("-inf", None), Ok(()));
        assert_eq!(check_numeric("1e131071", None), Ok(()));
        assert_eq!(check_numeric("1e-16383", None), Ok(()));
        assert_eq!(check_numeric("0e1073741823", None), Ok(()));

        assert_eq!(check_numeric("", None), Err(InvalidSyntax));
        assert_eq!(check_numeric(".", None), Err(InvalidSyntax));
        assert_eq!(check_numeric("1.2.3", None), Err(InvalidSyntax));
        assert_eq!(check_numeric("1._5", None), Err(InvalidSyntax));
        assert_eq!(check_numeric("1_.5", None), Err(InvalidSyntax));
        assert_eq!(check_numeric("1e", None), Err(InvalidSyntax));
        assert_eq!(check_numeric("1e_5", None), Err(InvalidSyntax));
        assert_eq!(check_numeric("0x", None), Err(InvalidSyntax));
        assert_eq!(check_numeric("-NaN", None), Err(InvalidSyntax));
        assert_eq!(check_numeric("infinit", None), Err(InvalidSyntax));
        assert_eq!(check_numeric("- 1", None), Err(InvalidSyntax));

        assert_eq!(check_numeric("1e131072", None), Err(FormatOverflow));
        assert_eq!(check_numeric("1e-16384", None), Err(FormatOverflow));
        assert_eq!(check_numeric("0e-16384", typmod(5, 2)), Err(FormatOverflow));
        assert_eq!(check_numeric("0e1073741824", None), Err(FormatOverflow));

        assert_eq!(check_numeric("9.94", typmod(2, 1)), Ok(()));
        assert_eq!(check_numeric("9.96", typmod(2, 1)), Err(FieldOverflow));
        assert_eq!(check_numeric("0.006", typmod(2, 2)), Ok(()));
        assert_eq!(check_numeric("0.995", typmod(2, 2)), Err(FieldOverflow));
        assert_eq!(check_numeric("99949", typmod(3, -2)), Ok(()));
        assert_eq!(check_numeric("99950", typmod(3, -2)), Err(FieldOverflow));
        assert_eq!(check_numeric("0.0095", typmod(1, 3)), Err(FieldOverflow));
        assert_eq!(check_numeric("5", typmod(1, -4)), Ok(()));
        assert_eq!(check_numeric("0xFF", typmod(3, 0)), Ok(()));
        assert_eq!(check_numeric("0xFF", typmod(2, 0)), Err(FieldOverflow));
        assert_eq!(check_numeric("NaN", typmod(3, 1)), Ok(()));
        assert_eq!(
            check_numeric("Infinity", typmod(3, 1)),
            Err(InfiniteOverflow)
        );

        let big = "0x1FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF";
        assert_eq!(check_numeric(big, None), Ok(()));
        assert_eq!(check_numeric(big, typmod(39, 0)), Ok(()));
        assert_eq!(check_numeric(big, typmod(3, 0)), Err(FieldOverflow));
        assert_eq!(check_numeric(&format!("{big}z"), None), Err(InvalidSyntax));
        assert_eq!(
            check_numeric(&format!("0x{}", "F".repeat(108_800)), None),
            Ok(())
        );
        assert_eq!(
            check_numeric(&format!("0x{}", "F".repeat(108_854)), None),
            Err(FormatOverflow)
        );
        assert_eq!(
            check_numeric(&format!("0x8{}", "0".repeat(108_852)), None),
            Ok(())
        );
        assert_eq!(
            BigUint::from(10u32).pow(NUMERIC_MAX_DECIMAL_DIGITS).bits(),
            NUMERIC_MAX_DECIMAL_BITS
        );
    }
}
