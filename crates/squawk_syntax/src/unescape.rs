use std::fmt;
use std::ops::{Range, RangeInclusive};

use rowan::TextSize;

use crate::decoded_text::DecodedText;

pub enum UnicodeEscapeKind {
    Extended,
    Short,
}

impl UnicodeEscapeKind {
    fn count(&self) -> u32 {
        match self {
            UnicodeEscapeKind::Extended => 6,
            UnicodeEscapeKind::Short => 4,
        }
    }
}

pub enum UnicodeEscError {
    InvalidEscape,
    InvalidSurrogatePair,
    OutOfRange,
    RequiresHexDigits {
        kind: UnicodeEscapeKind,
        escape_char: char,
    },
}

impl fmt::Display for UnicodeEscError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidEscape => f.write_str("Invalid Unicode escape sequence"),
            Self::InvalidSurrogatePair => f.write_str("Invalid Unicode surrogate pair"),
            Self::OutOfRange => f.write_str("Unicode escape value out of range"),
            Self::RequiresHexDigits { kind, escape_char } => {
                let required = kind.count();
                let plus = match kind {
                    UnicodeEscapeKind::Extended => "+",
                    UnicodeEscapeKind::Short => "",
                };
                let xs = "X".repeat(required as usize);
                write!(
                    f,
                    "Unicode escape requires {required} hex digits: {escape_char}{plus}{xs}"
                )
            }
        }
    }
}

pub fn escape_unicode_esc_str<F>(text: &str, escape_char: char, mut callback: F)
where
    F: FnMut(Range<usize>, Result<char, UnicodeEscError>),
{
    const HIGH_SURROGATE: RangeInclusive<u32> = 0xD800..=0xDBFF;
    const LOW_SURROGATE: RangeInclusive<u32> = 0xDC00..=0xDFFF;
    const MAX_CODEPOINT: u32 = 0x10FFFF;

    let mut chars = text.char_indices().peekable();
    let mut high_surrogate: Option<(Range<usize>, u32)> = None;

    while let Some((escape_start, c)) = chars.next() {
        if c != escape_char {
            if let Some((hi_range, _)) = high_surrogate.take() {
                callback(hi_range, Err(UnicodeEscError::InvalidSurrogatePair));
            }
            callback(escape_start..escape_start + c.len_utf8(), Ok(c));
            continue;
        }
        let kind = match chars.peek() {
            Some(&(_, c)) if c == escape_char => {
                chars.next();
                if let Some((hi_range, _)) = high_surrogate.take() {
                    callback(hi_range, Err(UnicodeEscError::InvalidSurrogatePair));
                }
                let end = escape_start + escape_char.len_utf8() * 2;
                callback(escape_start..end, Ok(escape_char));
                continue;
            }
            Some(&(_, '+')) => {
                chars.next();
                UnicodeEscapeKind::Extended
            }
            Some(&(_, c)) if c.is_ascii_hexdigit() => UnicodeEscapeKind::Short,
            _ => {
                let end = chars
                    .next()
                    .map(|(i, c)| i + c.len_utf8())
                    .unwrap_or(text.len());
                if let Some((hi_range, _)) = high_surrogate.take() {
                    callback(hi_range, Err(UnicodeEscError::InvalidSurrogatePair));
                }
                callback(escape_start..end, Err(UnicodeEscError::InvalidEscape));
                continue;
            }
        };
        let mut codepoint: u32 = 0;
        let mut got_all = true;
        let mut last_end = chars.peek().map(|&(i, _)| i).unwrap_or(text.len());
        for _ in 0..kind.count() {
            let radix = 16;
            let Some(&(i, ch)) = chars.peek() else {
                got_all = false;
                break;
            };
            let Some(d) = ch.to_digit(radix) else {
                got_all = false;
                break;
            };
            chars.next();
            codepoint = codepoint * radix + d;
            last_end = i + ch.len_utf8();
        }
        if !got_all {
            if let Some((hi_range, _)) = high_surrogate.take() {
                callback(hi_range, Err(UnicodeEscError::InvalidSurrogatePair));
            }
            callback(
                escape_start..last_end,
                Err(UnicodeEscError::RequiresHexDigits { kind, escape_char }),
            );
            continue;
        }
        if let Some((hi_range, hi_cp)) = high_surrogate.take() {
            if LOW_SURROGATE.contains(&codepoint) {
                let combined = 0x10000 + ((hi_cp - 0xD800) << 10) + (codepoint - 0xDC00);
                let ch = char::from_u32(combined).unwrap();
                callback(hi_range.start..last_end, Ok(ch));
                continue;
            }
            callback(
                hi_range.start..last_end,
                Err(UnicodeEscError::InvalidSurrogatePair),
            );
            continue;
        }
        if codepoint > MAX_CODEPOINT {
            callback(escape_start..last_end, Err(UnicodeEscError::OutOfRange));
        } else if HIGH_SURROGATE.contains(&codepoint) {
            high_surrogate = Some((escape_start..last_end, codepoint));
        } else if LOW_SURROGATE.contains(&codepoint) {
            callback(
                escape_start..last_end,
                Err(UnicodeEscError::InvalidSurrogatePair),
            );
        } else {
            let ch = char::from_u32(codepoint).unwrap();
            callback(escape_start..last_end, Ok(ch));
        }
    }
    if let Some((range, _)) = high_surrogate {
        callback(range, Err(UnicodeEscError::InvalidSurrogatePair));
    }
}

// https://github.com/postgres/postgres/blob/228a1f9542792c6533ef74c2e7aefad0da1d9a7a/src/backend/parser/parser.c#L350
const fn is_valid_uescape_char(byte: u8) -> bool {
    !byte.is_ascii_hexdigit()
        && byte != b'+'
        && byte != b'\''
        && byte != b'"'
        && !matches!(
            byte,
            b' ' | b'\t' | b'\n' | b'\r' | /* b'\v' */ 0x0B | /* b'\f' */ 0x0C
        )
}

pub fn uescape_char(text: &str) -> Option<char> {
    let inner = text.strip_prefix('\'')?.strip_suffix('\'')?;
    let &[byte] = inner.as_bytes() else {
        return None;
    };
    is_valid_uescape_char(byte).then(|| char::from(byte))
}

pub fn decode_plain_string(inner: &str, start_pos: TextSize, out: &mut DecodedText) {
    let mut chars = inner.char_indices().peekable();
    while let Some((i, c)) = chars.next() {
        let pos = start_pos + TextSize::new(i as u32);
        if c == '\'' && chars.peek().is_some_and(|&(_, next)| next == '\'') {
            chars.next();
        }
        out.push_char(c, pos);
    }
}

struct EscBuffer {
    bytes: Vec<u8>,
    pos: TextSize,
}

impl EscBuffer {
    fn new(pos: TextSize) -> Self {
        Self { bytes: vec![], pos }
    }

    fn push(&mut self, byte: u8, pos: TextSize) {
        if self.bytes.is_empty() {
            self.pos = pos;
        }
        self.bytes.push(byte);
    }

    fn push_char(&mut self, c: char, pos: TextSize) {
        if self.bytes.is_empty() {
            self.pos = pos;
        }
        let mut buf = [0; 4];
        self.bytes
            .extend_from_slice(c.encode_utf8(&mut buf).as_bytes());
    }

    fn drain(&mut self, out: &mut DecodedText) {
        if self.bytes.is_empty() {
            return;
        }
        match std::str::from_utf8(&self.bytes) {
            Ok(text) => out.push_str(text, self.pos),
            Err(err) if err.error_len().is_some() => {
                out.push_str(&String::from_utf8_lossy(&self.bytes), self.pos);
            }
            Err(_) => return,
        }
        self.bytes.clear();
    }

    fn flush(&mut self, out: &mut DecodedText) {
        if self.bytes.is_empty() {
            return;
        }
        out.push_str(&String::from_utf8_lossy(&self.bytes), self.pos);
        self.bytes.clear();
    }
}

pub fn decode_esc_string(inner: &str, start_pos: TextSize, out: &mut DecodedText) {
    let mut chars = inner.char_indices().peekable();
    let mut esc = EscBuffer::new(start_pos);

    while let Some((i, c)) = chars.next() {
        let pos = start_pos + TextSize::new(i as u32);

        if c == '\'' && chars.peek().is_some_and(|&(_, next)| next == '\'') {
            chars.next();
            esc.flush(out);
            out.push_char('\'', pos);
            continue;
        }
        if c != '\\' {
            esc.flush(out);
            out.push_char(c, pos);
            continue;
        }
        let Some(&(_, next)) = chars.peek() else {
            esc.flush(out);
            out.push_char('\\', pos);
            break;
        };
        match next {
            'b' => {
                chars.next();
                esc.push(b'\x08', pos);
            }
            'f' => {
                chars.next();
                esc.push(b'\x0C', pos);
            }
            'n' => {
                chars.next();
                esc.push(b'\n', pos);
            }
            'r' => {
                chars.next();
                esc.push(b'\r', pos);
            }
            't' => {
                chars.next();
                esc.push(b'\t', pos);
            }
            '0'..='7' => {
                let mut value: u32 = 0;
                for _ in 0..3 {
                    match chars.peek() {
                        Some(&(_, d)) if ('0'..='7').contains(&d) => {
                            chars.next();
                            value = value * 8 + d.to_digit(8).unwrap();
                        }
                        _ => break,
                    }
                }
                if value != 0 {
                    esc.push(value as u8, pos);
                }
            }
            'x' => {
                chars.next();
                let mut value: u8 = 0;
                let mut got_any = false;
                for _ in 0..2 {
                    match chars.peek() {
                        Some(&(_, d)) if d.is_ascii_hexdigit() => {
                            chars.next();
                            value = value * 16 + d.to_digit(16).unwrap() as u8;
                            got_any = true;
                        }
                        _ => break,
                    }
                }
                if got_any {
                    if value != 0 {
                        esc.push(value, pos);
                    }
                } else {
                    esc.push(b'x', pos);
                }
            }
            'u' | 'U' => {
                chars.next();
                let required = if next == 'u' { 4 } else { 8 };
                let mut value: u32 = 0;
                let mut got_all = true;
                for _ in 0..required {
                    match chars.peek() {
                        Some(&(_, d)) if d.is_ascii_hexdigit() => {
                            chars.next();
                            value = value * 16 + d.to_digit(16).unwrap();
                        }
                        _ => {
                            got_all = false;
                            break;
                        }
                    }
                }
                if got_all
                    && let Some(ch) = char::from_u32(value)
                    && ch != '\0'
                {
                    esc.push_char(ch, pos);
                }
            }
            _ => {
                chars.next();
                esc.push_char(next, pos);
            }
        }
        esc.drain(out);
    }

    esc.flush(out);
}

pub fn decode_unicode_esc_string(
    inner: &str,
    start_pos: TextSize,
    escape_char: char,
    out: &mut DecodedText,
) {
    let mut dequoted = DecodedText::new(start_pos);
    decode_plain_string(inner, start_pos, &mut dequoted);

    escape_unicode_esc_str(dequoted.text(), escape_char, |range, result| {
        if let Ok(ch) = result {
            out.push_char(ch, dequoted.source_pos(TextSize::new(range.start as u32)));
        }
    });
}

#[cfg(test)]
mod tests {
    use insta::assert_snapshot;

    use super::*;

    fn unicode_escape_events(text: &str, escape_char: char) -> String {
        let mut events = vec![];

        escape_unicode_esc_str(text, escape_char, |range, result| {
            let entry = match result {
                Ok(ch) => format!("{}..{} ok {ch:?}", range.start, range.end),
                Err(err) => format!("{}..{} err {err}", range.start, range.end),
            };
            events.push(entry);
        });

        events.join("\n")
    }

    fn decode_escape_string(inner: &str) -> String {
        let mut out = DecodedText::new(TextSize::new(0));
        decode_esc_string(inner, TextSize::new(0), &mut out);
        out.into_text()
    }

    fn decode_unicode_escape_string(inner: &str, escape_char: char) -> String {
        let mut out = DecodedText::new(TextSize::new(0));
        decode_unicode_esc_string(inner, TextSize::new(0), escape_char, &mut out);
        out.into_text()
    }

    #[test]
    fn ok() {
        assert_snapshot!(unicode_escape_events(r"hello world", '\\'), @"
        0..1 ok 'h'
        1..2 ok 'e'
        2..3 ok 'l'
        3..4 ok 'l'
        4..5 ok 'o'
        5..6 ok ' '
        6..7 ok 'w'
        7..8 ok 'o'
        8..9 ok 'r'
        9..10 ok 'l'
        10..11 ok 'd'
        ");
    }

    #[test]
    fn incomplete_unicode_escape_breaks_surrogate_pairing() {
        assert_snapshot!(unicode_escape_events(r"\D800\006\DC00", '\\'), @r"
        0..5 err Invalid Unicode surrogate pair
        5..9 err Unicode escape requires 4 hex digits: \XXXX
        9..14 err Invalid Unicode surrogate pair
        ");
    }

    #[test]
    fn invalid_unicode_escape_breaks_surrogate_pairing() {
        assert_snapshot!(unicode_escape_events(r"\D800\Q\DC00", '\\'), @r"
        0..5 err Invalid Unicode surrogate pair
        5..7 err Invalid Unicode escape sequence
        7..12 err Invalid Unicode surrogate pair
        ");
    }

    #[test]
    fn invalid_unicode_escape_does_not_emit_literal_char() {
        assert_snapshot!(unicode_escape_events(r"\0061\Q\0062", '\\'), @r"
        0..5 ok 'a'
        5..7 err Invalid Unicode escape sequence
        7..12 ok 'b'
        ");
    }

    #[test]
    fn invalid_unicode_escape_works_with_custom_escape_char() {
        assert_snapshot!(unicode_escape_events("!0061!Q!0062", '!'), @r"
        0..5 ok 'a'
        5..7 err Invalid Unicode escape sequence
        7..12 ok 'b'
        ");
    }

    #[test]
    fn valid_unicode_escape_after_high_surrogate_only_emits_error() {
        assert_snapshot!(unicode_escape_events(r"\D800\0061", '\\'), @r"
        0..10 err Invalid Unicode surrogate pair
        ");
    }

    #[test]
    fn decode_escape_string_hex_bytes_as_utf8() {
        assert_snapshot!(decode_escape_string(r"\xC3\xA9"), @"é");
    }

    #[test]
    fn decode_escape_string_skips_nul_byte() {
        assert_snapshot!(decode_escape_string(r"a\000b"), @"ab");
    }

    #[test]
    fn escape_string_incomplete_byte_escape() {
        assert_snapshot!(decode_escape_string(r"\xc3a"), @"�a");
        assert_snapshot!(decode_escape_string(r"\xc3"), @"�");
    }

    #[test]
    fn escape_string_trailing_backslash() {
        assert_snapshot!(decode_escape_string(r"a\"), @r"a\");
        assert_snapshot!(decode_escape_string(r"\xC3\"), @r"�\");
    }

    #[test]
    fn decode_unicode_string_collapses_doubled_quotes() {
        assert_snapshot!(decode_unicode_escape_string("a''b", '\\'), @"a'b");
    }
}
