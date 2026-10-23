use std::fmt::Write as _;

use squawk_syntax::ast::{self, LitKind};

pub(crate) struct LiteralHover {
    pub(crate) snippet: &'static str,
    pub(crate) comment: String,
}

pub(crate) fn hover_literal(literal: &ast::Literal) -> Option<LiteralHover> {
    let (snippet, comment) = match literal.kind()? {
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
            ("text", comment)
        }
        LitKind::BitString(_) => {
            let binary = literal.string_value()?;
            let hex = binary_to_hex(&binary)?;
            ("bit", bit_comment(&hex, &binary))
        }
        LitKind::ByteString(_) => {
            let hex = literal.string_value()?;
            let binary = hex_to_binary(&hex)?;
            ("bit", bit_comment(&hex, &binary))
        }
        LitKind::Default(_)
        | LitKind::False(_)
        | LitKind::IntNumber(_)
        | LitKind::Null(_)
        | LitKind::NumericNumber(_)
        | LitKind::Off(_)
        | LitKind::On(_)
        | LitKind::PositionalParam(_)
        | LitKind::True(_) => return None,
    };
    Some(LiteralHover { snippet, comment })
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
