use std::fmt::{self, Write as _};

use rowan::TextSize;
use squawk_syntax::ast::{self, AstNode, LitKind};

use crate::lower::literal_type_name;

pub(crate) struct LiteralHover {
    pub(crate) snippet: &'static str,
    pub(crate) comment: Option<String>,
}

pub(crate) fn hover_literal(literal: &ast::Literal) -> Option<LiteralHover> {
    let snippet = literal_type_name(literal)?;
    let comment = match literal.kind()? {
        LitKind::String(_)
        | LitKind::EscString(_)
        | LitKind::NationalString(_)
        | LitKind::UnicodeEscString(_)
        | LitKind::DollarQuotedString(_) => {
            let value = literal.string_value()?;
            let comment = match value.find(['\n', '\r']) {
                Some(newline) => format!(
                    "value of literal (truncated up to newline): {}",
                    InlineCode(&value[..newline])
                ),
                None => format!("value of literal: {}", InlineCode(&value)),
            };
            Some(comment)
        }
        LitKind::BitString(_) => {
            let binary = literal.string_value()?;
            let hex = binary_to_hex(&binary)?;
            Some(bit_comment(&hex, &binary))
        }
        LitKind::ByteString(_) => {
            let hex = literal.string_value()?;
            let binary = hex_to_binary(&hex)?;
            Some(bit_comment(&hex, &binary))
        }
        LitKind::IntNumber(_) => match literal.integer_value_u64() {
            Some(value) => Some(integer_comment(value)),
            None if literal.syntax().text_range().len() <= MAX_BIG_INTEGER_LEN => {
                Some(integer_comment(literal.integer_value()?))
            }
            None => None,
        },
        LitKind::NumericNumber(token) => numeric_value(token.text())
            .map(|value| format!("value of literal: {}", InlineCode(&value))),
        LitKind::False(_) | LitKind::True(_) => None,
        LitKind::Default(_)
        | LitKind::Null(_)
        | LitKind::Off(_)
        | LitKind::On(_)
        | LitKind::PositionalParam(_) => return None,
    };
    Some(LiteralHover { snippet, comment })
}

const MAX_BIG_INTEGER_LEN: TextSize = TextSize::new(1000);

fn integer_comment<T: fmt::Display + fmt::UpperHex + fmt::Binary>(value: T) -> String {
    format!(
        "value of literal: {}",
        InlineCode(&format!("{value} (0x{value:X}|0b{value:b})"))
    )
}

const NUMERIC_MAX_DISPLAY_SCALE: isize = 1000;

fn numeric_value(text: &str) -> Option<String> {
    let text = text.replace('_', "");
    let (mantissa, exponent) = match text.split_once(['e', 'E']) {
        Some((mantissa, exponent)) => (mantissa, exponent.parse::<isize>().ok()?),
        None => (text.as_str(), 0),
    };
    if exponent.abs() > NUMERIC_MAX_DISPLAY_SCALE {
        return None;
    }
    let (int_part, frac_part) = mantissa.split_once('.').unwrap_or((mantissa, ""));
    if !int_part
        .chars()
        .chain(frac_part.chars())
        .all(|c| c.is_ascii_digit())
    {
        return None;
    }
    let digits_len = int_part.len() + frac_part.len();
    let point = isize::try_from(int_part.len()).ok()? + exponent;
    let mut value = String::with_capacity(digits_len + exponent.unsigned_abs() + 2);
    let mut digits = int_part.chars().chain(frac_part.chars());
    if point <= 0 {
        value.push_str("0.");
        value.extend(std::iter::repeat_n('0', point.unsigned_abs()));
        value.extend(digits);
        return Some(value);
    }
    let point = point.unsigned_abs();
    value.extend(digits.by_ref().take(point).skip_while(|&c| c == '0'));
    let is_zero = value.is_empty();
    if is_zero {
        value.push('0');
    }
    if point > digits_len && !is_zero {
        value.extend(std::iter::repeat_n('0', point - digits_len));
    } else if point < digits_len {
        value.push('.');
        value.extend(digits);
    }
    Some(value)
}

fn bit_comment(hex: &str, binary: &str) -> String {
    format!(
        "value of literal: {}",
        InlineCode(&format!("x'{hex}'|b'{binary}'"))
    )
}

struct InlineCode<'a>(&'a str);

impl std::fmt::Display for InlineCode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut longest = 0;
        let mut run = 0;
        for ch in self.0.chars() {
            if ch == '`' {
                run += 1;
                longest = longest.max(run);
            } else {
                run = 0;
            }
        }
        let fence = "`".repeat(longest + 1);
        write!(f, "{fence} {} {fence}", self.0)
    }
}

fn binary_to_hex(binary: &str) -> Option<String> {
    let mut hex = String::with_capacity(binary.len().div_ceil(4));
    let first = match binary.len() % 4 {
        0 => 4,
        len => len,
    };
    let mut start = 0;
    let mut end = first.min(binary.len());
    while start < binary.len() {
        let digit = u32::from_str_radix(&binary[start..end], 2).ok()?;
        hex.push(char::from_digit(digit, 16)?.to_ascii_uppercase());
        start = end;
        end += 4;
    }
    Some(hex)
}

fn hex_to_binary(hex: &str) -> Option<String> {
    let mut binary = String::with_capacity(hex.len() * 4);
    for ch in hex.chars() {
        write!(binary, "{:04b}", ch.to_digit(16)?).unwrap();
    }
    Some(binary)
}
