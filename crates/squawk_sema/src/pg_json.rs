use std::fmt;

use serde::de::IgnoredAny;
use squawk_rowan::{TextRange, TextSize};

use crate::pg_numeric::{self, NumericInputError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum JsonType {
    Json,
    Jsonb,
}

impl fmt::Display for JsonType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            JsonType::Json => "json",
            JsonType::Jsonb => "jsonb",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum JsonInputError {
    InvalidSyntax { range: Option<TextRange> },
    UnsupportedEscape { range: TextRange },
    NumericOverflow { range: TextRange },
}

// see: https://github.com/postgres/postgres/blob/b82d69abf64fc0c2fc6fdd491d7cecb8244680c2/src/common/jsonapi.c#L745
pub(crate) fn check_json(text: &str, ty: JsonType) -> Result<(), JsonInputError> {
    if serde_json::from_str::<IgnoredAny>(text).is_err() {
        return Err(JsonInputError::InvalidSyntax { range: None });
    }
    match ty {
        JsonType::Json => Ok(()),
        JsonType::Jsonb => check_jsonb(text),
    }
}

// see: https://github.com/postgres/postgres/blob/b82d69abf64fc0c2fc6fdd491d7cecb8244680c2/src/backend/utils/adt/jsonb.c#L371
fn check_jsonb(text: &str) -> Result<(), JsonInputError> {
    let bytes = text.as_bytes();
    let mut i = 0;
    while let Some(&byte) = bytes.get(i) {
        match byte {
            b'"' => i = check_jsonb_string(bytes, i + 1)?,
            b'-' | b'0'..=b'9' => {
                let start = i;
                while bytes.get(i).is_some_and(|byte| {
                    matches!(byte, b'0'..=b'9' | b'-' | b'+' | b'.' | b'e' | b'E')
                }) {
                    i += 1;
                }
                if pg_numeric::check_numeric(&text[start..i], None)
                    == Err(NumericInputError::FormatOverflow)
                {
                    return Err(JsonInputError::NumericOverflow {
                        range: text_range(start, i),
                    });
                }
            }
            _ => i += 1,
        }
    }
    Ok(())
}

// see: https://github.com/postgres/postgres/blob/b82d69abf64fc0c2fc6fdd491d7cecb8244680c2/src/common/jsonapi.c#L2013
fn check_jsonb_string(bytes: &[u8], mut i: usize) -> Result<usize, JsonInputError> {
    loop {
        match bytes.get(i) {
            Some(b'"') | None => return Ok(i + 1),
            Some(b'\\') => {
                let Some(code) = unicode_escape(bytes, i) else {
                    i += 2;
                    continue;
                };
                let range = text_range(i, i + 6);
                match code {
                    0 => return Err(JsonInputError::UnsupportedEscape { range }),
                    0xD800..=0xDBFF => {
                        if !unicode_escape(bytes, i + 6)
                            .is_some_and(|low| (0xDC00..=0xDFFF).contains(&low))
                        {
                            return Err(JsonInputError::InvalidSyntax { range: Some(range) });
                        }
                        i += 12;
                    }
                    0xDC00..=0xDFFF => {
                        return Err(JsonInputError::InvalidSyntax { range: Some(range) });
                    }
                    _ => i += 6,
                }
            }
            Some(_) => i += 1,
        }
    }
}

fn unicode_escape(bytes: &[u8], i: usize) -> Option<u16> {
    let [b'\\', b'u', digits @ ..] = bytes.get(i..i + 6)? else {
        return None;
    };
    u16::from_str_radix(std::str::from_utf8(digits).ok()?, 16).ok()
}

fn text_range(start: usize, end: usize) -> TextRange {
    TextRange::new(
        TextSize::try_from(start).unwrap(),
        TextSize::try_from(end).unwrap(),
    )
}

#[cfg(test)]
mod tests {
    use squawk_rowan::{TextRange, TextSize};

    use super::{
        JsonInputError::{InvalidSyntax, NumericOverflow, UnsupportedEscape},
        JsonType::{Json, Jsonb},
        check_json,
    };

    fn range(start: u32, end: u32) -> TextRange {
        TextRange::new(TextSize::new(start), TextSize::new(end))
    }

    #[test]
    fn check_jsons() {
        for text in [
            r#"{"a": [1, -0, 1.5e-3, 1E+2, true, false, null]}"#,
            r#"{"a": 1, "a": 2}"#,
            " \r\n\t1 ",
            r#""\/\b\f\n\r\t\"\\""#,
            r#""\ud83d\ude00""#,
            "\"\u{7f}é\"",
            &"1".repeat(500),
            &format!("0.{}1", "0".repeat(1000)),
            &format!("{}{}", "[".repeat(1_000_000), "]".repeat(1_000_000)),
        ] {
            assert_eq!(check_json(text, Json), Ok(()), "{text}");
            assert_eq!(check_json(text, Jsonb), Ok(()), "{text}");
        }

        for text in [
            "",
            " ",
            "01",
            "1.",
            ".5",
            "+1",
            "-",
            "1e",
            "NaN",
            "Infinity",
            "tru",
            "TRUE",
            "[1,]",
            r#"{"a": 1,}"#,
            "[1 2]",
            "{a: 1}",
            "'a'",
            "1 2",
            "[1]x",
            "// c\n1",
            "\u{c}1",
            "\u{a0}1",
            "\"a\tb\"",
            r#""\x41""#,
            r#""\v""#,
            r#""\u12""#,
            &"[".repeat(1_000_000),
        ] {
            assert_eq!(
                check_json(text, Json),
                Err(InvalidSyntax { range: None }),
                "{text}"
            );
            assert_eq!(
                check_json(text, Jsonb),
                Err(InvalidSyntax { range: None }),
                "{text}"
            );
        }
    }

    #[test]
    fn check_jsonb_only() {
        for text in [
            r#""\u0000""#,
            r#""\ud800""#,
            r#""\ud800x""#,
            r#""\udc00""#,
            r#""\ud800\u0041""#,
            "1e99999999999999999999",
        ] {
            assert_eq!(check_json(text, Json), Ok(()), "{text}");
        }

        assert_eq!(
            check_json(r#"["a", "\u0000"]"#, Jsonb),
            Err(UnsupportedEscape {
                range: range(7, 13)
            })
        );
        assert_eq!(
            check_json(r#""\ud800""#, Jsonb),
            Err(InvalidSyntax {
                range: Some(range(1, 7))
            })
        );
        assert_eq!(
            check_json(r#""a\ud800x""#, Jsonb),
            Err(InvalidSyntax {
                range: Some(range(2, 8))
            })
        );
        assert_eq!(
            check_json(r#""\udc00""#, Jsonb),
            Err(InvalidSyntax {
                range: Some(range(1, 7))
            })
        );
        assert_eq!(
            check_json(r#""\ud800\u0041""#, Jsonb),
            Err(InvalidSyntax {
                range: Some(range(1, 7))
            })
        );
        assert_eq!(check_json(r#"{"a\\u0000": 1}"#, Jsonb), Ok(()));
        assert_eq!(
            check_json(r#"{"a": 1e99999999999999999999}"#, Jsonb),
            Err(NumericOverflow {
                range: range(6, 28)
            })
        );
        assert_eq!(
            check_json("[1e-99999999999999999999]", Jsonb),
            Err(NumericOverflow {
                range: range(1, 24)
            })
        );
        assert_eq!(
            check_json(r#"{"1e99999999999999999999": 1}"#, Jsonb),
            Ok(())
        );
    }
}
