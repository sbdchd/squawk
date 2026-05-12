use std::fmt;
use std::ops::{Range, RangeInclusive};

pub(crate) enum UnicodeEscapeKind {
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

pub(crate) enum UnicodeEscError {
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

pub(crate) fn escape_unicode_esc_str<F>(text: &str, escape_char: char, mut callback: F)
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
}
