use squawk_syntax::{
    SyntaxKind,
    ast::{self, AstNode},
    quote::{strip_dollar_quotes, strip_prefixed_quotes, strip_quotes, strip_unicode_esc_prefix},
    unescape::{decode_esc_string, decode_plain_string, decode_unicode_esc_string, uescape_char},
};

pub(crate) fn binary_digits_to_hex(digits: &str) -> Option<String> {
    const HEX_DIGITS: &[u8; 16] = b"0123456789ABCDEF";

    if digits.is_empty() {
        return Some("".to_string());
    }

    let mut out = String::with_capacity(digits.len().div_ceil(4));
    let mut start = 0;

    while start < digits.len() {
        let chunk_len = if start == 0 {
            match digits.len() % 4 {
                0 => 4,
                n => n,
            }
        } else {
            4
        };
        let end = start + chunk_len;
        let value = u8::from_str_radix(&digits[start..end], 2).ok()?;
        out.push(HEX_DIGITS[value as usize] as char);
        start = end;
    }

    Some(out)
}

pub(crate) fn hex_digits_to_binary(digits: &str) -> Option<String> {
    const BINARY_DIGITS: [&str; 16] = [
        "0000", "0001", "0010", "0011", "0100", "0101", "0110", "0111", "1000", "1001", "1010",
        "1011", "1100", "1101", "1110", "1111",
    ];

    if digits.is_empty() {
        return Some("".to_string());
    }

    let mut out = String::with_capacity(digits.len() * 4);
    for ch in digits.chars() {
        let value = ch.to_digit(16)? as usize;
        out.push_str(BINARY_DIGITS[value]);
    }

    Some(out)
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum StringDecoding {
    BitOrByte,
    EscString,
    UnicodeEscString,
}

pub(crate) fn literal_string_value(literal: &ast::Literal) -> Option<String> {
    let escape_char = unicode_escape_char(literal);
    let mut out = String::with_capacity(literal.syntax().text().len().into());
    let mut decoding: Option<StringDecoding> = None;

    for element in literal.syntax().children_with_tokens() {
        let Some(token) = element.into_token() else {
            continue;
        };
        match token.kind() {
            SyntaxKind::ESC_STRING => {
                let inner = strip_prefixed_quotes(token.text(), ['e', 'E'])?;
                decode_esc_string(inner, &mut out);
                decoding = Some(StringDecoding::EscString);
            }
            SyntaxKind::UNICODE_ESC_STRING => {
                let inner = strip_unicode_esc_prefix(token.text())?;
                decode_unicode_esc_string(inner, escape_char, &mut out);
                decoding = Some(StringDecoding::UnicodeEscString);
            }
            SyntaxKind::BIT_STRING => {
                let inner = strip_prefixed_quotes(token.text(), ['b', 'B'])?;
                out.push_str(inner);
                decoding = Some(StringDecoding::BitOrByte);
            }
            SyntaxKind::BYTE_STRING => {
                let inner = strip_prefixed_quotes(token.text(), ['x', 'X'])?;
                out.push_str(inner);
                decoding = Some(StringDecoding::BitOrByte);
            }
            SyntaxKind::DOLLAR_QUOTED_STRING => {
                let inner = strip_dollar_quotes(token.text())?;
                out.push_str(inner);
                return Some(out);
            }
            SyntaxKind::STRING => {
                let inner = strip_quotes(token.text())?;
                match decoding {
                    Some(StringDecoding::EscString) => decode_esc_string(inner, &mut out),
                    Some(StringDecoding::UnicodeEscString) => {
                        decode_unicode_esc_string(inner, escape_char, &mut out)
                    }
                    Some(StringDecoding::BitOrByte) => out.push_str(inner),
                    None => decode_plain_string(inner, &mut out),
                }
            }
            SyntaxKind::UESCAPE_KW => break,
            _ => (),
        }
    }

    Some(out)
}

fn unicode_escape_char(literal: &ast::Literal) -> char {
    let mut seen_uescape = false;
    for element in literal.syntax().children_with_tokens() {
        let Some(token) = element.into_token() else {
            continue;
        };
        match token.kind() {
            SyntaxKind::UESCAPE_KW => seen_uescape = true,
            SyntaxKind::STRING if seen_uescape => {
                if let Some(ch) = uescape_char(token.text()) {
                    return ch;
                }
                return '\\';
            }
            _ => (),
        }
    }
    '\\'
}
