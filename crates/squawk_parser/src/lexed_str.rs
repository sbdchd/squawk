// based on https://github.com/rust-lang/rust-analyzer/blob/d8887c0758bbd2d5f752d5bd405d4491e90e7ed6/crates/parser/src/lexed_str.rs

use std::{num::IntErrorKind, ops};

use squawk_lexer::tokenize;

use crate::SyntaxKind;

pub struct LexedStr<'a> {
    text: &'a str,
    kind: Vec<SyntaxKind>,
    start: Vec<u32>,
    error: Vec<LexError>,
}

struct LexError {
    msg: String,
    range: ops::Range<u32>,
}

impl<'a> LexedStr<'a> {
    // TODO: rust-analyzer has an edition thing to specify things that are only
    // available in certain version, we can do that later
    pub fn new(text: &'a str) -> LexedStr<'a> {
        let mut conv = Converter::new(text);

        for token in tokenize(&text[conv.offset..]) {
            let token_text = &text[conv.offset..][..token.len as usize];

            conv.extend_token(&token.kind, token_text);
        }

        conv.finalize_with_eof()
    }

    // pub(crate) fn single_token(text: &'a str) -> Option<(SyntaxKind, Option<String>)> {
    //     if text.is_empty() {
    //         return None;
    //     }

    //     let token = tokenize(text).next()?;
    //     if token.len as usize != text.len() {
    //         return None;
    //     }

    //     let mut conv = Converter::new(text);
    //     conv.extend_token(&token.kind, text);
    //     match &*conv.res.kind {
    //         [kind] => Some((*kind, conv.res.error.pop().map(|it| it.msg))),
    //         _ => None,
    //     }
    // }

    // pub(crate) fn as_str(&self) -> &str {
    //     self.text
    // }

    pub(crate) fn len(&self) -> usize {
        self.kind.len() - 1
    }

    // pub(crate) fn is_empty(&self) -> bool {
    //     self.len() == 0
    // }

    pub(crate) fn kind(&self, i: usize) -> SyntaxKind {
        assert!(i < self.len());
        self.kind[i]
    }

    pub(crate) fn range_text(&self, r: ops::Range<usize>) -> &str {
        assert!(r.start < r.end && r.end <= self.len());
        let lo = self.start[r.start] as usize;
        let hi = self.start[r.end] as usize;
        &self.text[lo..hi]
    }

    // Naming is hard.
    pub fn text_range(&self, i: usize) -> ops::Range<usize> {
        assert!(i < self.len());
        let lo = self.start[i] as usize;
        let hi = self.start[i + 1] as usize;
        lo..hi
    }
    pub fn text_start(&self, i: usize) -> usize {
        assert!(i <= self.len());
        self.start[i] as usize
    }
    // pub(crate) fn text_len(&self, i: usize) -> usize {
    //     assert!(i < self.len());
    //     let r = self.text_range(i);
    //     r.end - r.start
    // }

    // pub(crate) fn error(&self, i: usize) -> Option<&str> {
    //     assert!(i < self.len());
    //     let err = self
    //         .error
    //         .binary_search_by_key(&(i as u32), |i| i.token)
    //         .ok()?;
    //     Some(self.error[err].msg.as_str())
    // }

    pub fn errors(&self) -> impl Iterator<Item = (&ops::Range<u32>, &str)> + '_ {
        self.error.iter().map(|it| (&it.range, it.msg.as_str()))
    }

    fn push(&mut self, kind: SyntaxKind, offset: usize) {
        self.kind.push(kind);
        self.start.push(offset as u32);
    }
}

struct Converter<'a> {
    res: LexedStr<'a>,
    offset: usize,
    prefixed_string_continuation: Option<PrefixedStringKind>,
}

#[derive(Clone, Copy)]
enum PrefixedStringKind {
    Bit,
    Byte,
    Escape,
}

fn is_empty_quoted_ident(token_text: &str, uescape: bool) -> bool {
    let inner = if uescape {
        token_text
            .strip_prefix(['u', 'U'])
            .and_then(|s| s.strip_prefix('&'))
    } else {
        Some(token_text)
    };
    inner == Some("\"\"")
}

impl<'a> Converter<'a> {
    fn new(text: &'a str) -> Self {
        Self {
            res: LexedStr {
                text,
                kind: Vec::new(),
                start: Vec::new(),
                error: Vec::new(),
            },
            offset: 0,
            prefixed_string_continuation: None,
        }
    }

    fn finalize_with_eof(mut self) -> LexedStr<'a> {
        self.res.push(SyntaxKind::EOF, self.offset);
        self.res
    }

    fn push(&mut self, kind: SyntaxKind, len: usize, err: Option<(&str, ops::Range<u32>)>) {
        let token_start = self.offset as u32;
        self.res.push(kind, self.offset);
        self.offset += len;

        if let Some((msg, err_range)) = err {
            self.res.error.push(LexError {
                msg: msg.to_owned(),
                range: token_start + err_range.start..token_start + err_range.end,
            });
        }
    }

    fn extend_token(&mut self, kind: &squawk_lexer::TokenKind, token_text: &str) {
        if !matches!(
            kind,
            squawk_lexer::TokenKind::Whitespace
                | squawk_lexer::TokenKind::LineComment
                | squawk_lexer::TokenKind::BlockComment { .. }
                | squawk_lexer::TokenKind::Literal { .. }
        ) {
            self.prefixed_string_continuation = None;
        }

        // A note on an intended tradeoff:
        // We drop some useful information here (see patterns with double dots `..`)
        // Storing that info in `SyntaxKind` is not possible due to its layout requirements of
        // being `u16` that come from `rowan::SyntaxKind`.
        let mut err = "";
        let mut err_range: Option<ops::Range<u32>> = None;

        let syntax_kind = {
            match kind {
                squawk_lexer::TokenKind::LineComment => SyntaxKind::COMMENT,
                squawk_lexer::TokenKind::BlockComment { terminated } => {
                    if !terminated {
                        err = "Missing trailing `*/` symbols to terminate the block comment";
                    }
                    SyntaxKind::COMMENT
                }

                squawk_lexer::TokenKind::Whitespace => SyntaxKind::WHITESPACE,
                squawk_lexer::TokenKind::Ident => {
                    SyntaxKind::from_keyword(token_text).unwrap_or(SyntaxKind::IDENT)
                }
                squawk_lexer::TokenKind::Literal { kind, .. } => {
                    self.extend_literal(token_text, kind);
                    return;
                }
                squawk_lexer::TokenKind::Semi => SyntaxKind::SEMICOLON,
                squawk_lexer::TokenKind::Comma => SyntaxKind::COMMA,
                squawk_lexer::TokenKind::Dot => SyntaxKind::DOT,
                squawk_lexer::TokenKind::OpenParen => SyntaxKind::L_PAREN,
                squawk_lexer::TokenKind::CloseParen => SyntaxKind::R_PAREN,
                squawk_lexer::TokenKind::OpenBracket => SyntaxKind::L_BRACK,
                squawk_lexer::TokenKind::CloseBracket => SyntaxKind::R_BRACK,
                squawk_lexer::TokenKind::OpenCurly => SyntaxKind::L_CURLY,
                squawk_lexer::TokenKind::CloseCurly => SyntaxKind::R_CURLY,
                squawk_lexer::TokenKind::At => SyntaxKind::AT,
                squawk_lexer::TokenKind::Pound => SyntaxKind::POUND,
                squawk_lexer::TokenKind::Tilde => SyntaxKind::TILDE,
                squawk_lexer::TokenKind::Question => SyntaxKind::QUESTION,
                squawk_lexer::TokenKind::Colon => SyntaxKind::COLON,
                squawk_lexer::TokenKind::Eq => SyntaxKind::EQ,
                squawk_lexer::TokenKind::Bang => SyntaxKind::BANG,
                squawk_lexer::TokenKind::Lt => SyntaxKind::L_ANGLE,
                squawk_lexer::TokenKind::Gt => SyntaxKind::R_ANGLE,
                squawk_lexer::TokenKind::Minus => SyntaxKind::MINUS,
                squawk_lexer::TokenKind::And => SyntaxKind::AMP,
                squawk_lexer::TokenKind::Or => SyntaxKind::PIPE,
                squawk_lexer::TokenKind::Plus => SyntaxKind::PLUS,
                squawk_lexer::TokenKind::Star => SyntaxKind::STAR,
                squawk_lexer::TokenKind::Slash => SyntaxKind::SLASH,
                squawk_lexer::TokenKind::Caret => SyntaxKind::CARET,
                squawk_lexer::TokenKind::Percent => SyntaxKind::PERCENT,
                squawk_lexer::TokenKind::Unknown => SyntaxKind::ERROR,
                squawk_lexer::TokenKind::Eof => SyntaxKind::EOF,
                squawk_lexer::TokenKind::Backtick => SyntaxKind::BACKTICK,
                squawk_lexer::TokenKind::PositionalParam {
                    trailing_junk_start,
                } => {
                    let digits = &token_text[1..*trailing_junk_start as usize];
                    if digits.is_empty() {
                        err = "missing parameter number";
                        err_range = Some(0..1);
                    } else if digits
                        .parse::<i32>()
                        .is_err_and(|err| matches!(err.kind(), IntErrorKind::PosOverflow))
                    {
                        err = "parameter number too large";
                        err_range = Some(0..*trailing_junk_start);
                    } else if (*trailing_junk_start as usize) < token_text.len() {
                        err = "trailing junk after positional parameter";
                        err_range = Some(*trailing_junk_start..token_text.len() as u32);
                    }
                    SyntaxKind::POSITIONAL_PARAM
                }
                squawk_lexer::TokenKind::QuotedIdent {
                    terminated,
                    uescape,
                } => {
                    if !terminated {
                        err = "Missing trailing \" to terminate the quoted identifier"
                    } else if is_empty_quoted_ident(token_text, *uescape) {
                        err = "empty delimited identifier";
                    }
                    SyntaxKind::IDENT
                }
            }
        };

        let err = if err.is_empty() { None } else { Some(err) };
        let err = err.map(|msg| (msg, err_range.unwrap_or(0..token_text.len() as u32)));
        self.push(syntax_kind, token_text.len(), err);
    }

    fn extend_literal(&mut self, token_text: &str, kind: &squawk_lexer::LiteralKind) {
        let mut err: Option<String> = None;
        let mut err_range: Option<ops::Range<u32>> = None;
        let continuation = self.prefixed_string_continuation.take();

        let syntax_kind = match *kind {
            squawk_lexer::LiteralKind::Int {
                empty_int,
                base,
                trailing_junk_start,
            } => {
                if empty_int {
                    err = Some("Missing digits after the integer base prefix".into());
                } else {
                    if matches!(base, squawk_lexer::Base::Binary | squawk_lexer::Base::Octal) {
                        let prefix_len = 2u32;
                        let digits = &token_text[prefix_len as usize..trailing_junk_start as usize];
                        let base = base as u32;
                        let token_start = self.offset as u32;
                        for (i, c) in digits.char_indices() {
                            if c != '_' && c.to_digit(base).is_none() {
                                let start = token_start + prefix_len + i as u32;
                                let end = start + c.len_utf8() as u32;
                                self.res.error.push(LexError {
                                    msg: format!("invalid digit for a base {base} literal"),
                                    range: start..end,
                                });
                            }
                        }
                    }
                    if (trailing_junk_start as usize) < token_text.len() {
                        err = Some("trailing junk after numeric literal".into());
                        err_range = Some(trailing_junk_start..token_text.len() as u32);
                    }
                }
                SyntaxKind::INT_NUMBER
            }
            squawk_lexer::LiteralKind::Numeric {
                empty_exponent_start,
                trailing_junk_start,
            } => {
                if let Some(exponent_start) = empty_exponent_start {
                    err = Some("Missing digits after the exponent symbol".into());
                    err_range = Some(exponent_start..exponent_start + 1);
                } else if (trailing_junk_start as usize) < token_text.len() {
                    err = Some("trailing junk after numeric literal".into());
                    err_range = Some(trailing_junk_start..token_text.len() as u32);
                }
                SyntaxKind::NUMERIC_NUMBER
            }
            squawk_lexer::LiteralKind::Str { terminated } => {
                if !terminated {
                    err =
                        Some("Missing trailing `'` symbol to terminate the string literal".into());
                } else if let Some(kind) = continuation {
                    self.validate_prefixed_string_content(token_text, 1, kind);
                    self.prefixed_string_continuation = Some(kind);
                }
                SyntaxKind::STRING
            }
            squawk_lexer::LiteralKind::NationalStr { terminated } => {
                if !terminated {
                    err = Some(
                        "Missing trailing `'` symbol to terminate the national character string literal"
                            .into(),
                    );
                }
                SyntaxKind::NATIONAL_STRING
            }
            squawk_lexer::LiteralKind::ByteStr { terminated } => {
                if !terminated {
                    err = Some(
                        "Missing trailing `'` symbol to terminate the hex bit string literal"
                            .into(),
                    );
                } else {
                    self.validate_prefixed_string_content(token_text, 2, PrefixedStringKind::Byte);
                    self.prefixed_string_continuation = Some(PrefixedStringKind::Byte);
                }
                SyntaxKind::BYTE_STRING
            }
            squawk_lexer::LiteralKind::BitStr { terminated } => {
                if !terminated {
                    err = Some(
                        "Missing trailing `'` symbol to terminate the bit string literal".into(),
                    );
                } else {
                    self.validate_prefixed_string_content(token_text, 2, PrefixedStringKind::Bit);
                    self.prefixed_string_continuation = Some(PrefixedStringKind::Bit);
                }
                SyntaxKind::BIT_STRING
            }
            squawk_lexer::LiteralKind::DollarQuotedString { terminated } => {
                if !terminated {
                    // TODO: we could be fancier and say the ending string we're looking for
                    err = Some("Unterminated dollar quoted string literal".into());
                }
                SyntaxKind::DOLLAR_QUOTED_STRING
            }
            squawk_lexer::LiteralKind::UnicodeEscStr { terminated } => {
                if !terminated {
                    err = Some(
                        "Missing trailing `'` symbol to terminate the unicode escape string literal"
                            .into(),
                    );
                }
                // validated in squawk_syntax
                SyntaxKind::UNICODE_ESC_STRING
            }
            squawk_lexer::LiteralKind::EscStr { terminated } => {
                if !terminated {
                    err = Some(
                        "Missing trailing `'` symbol to terminate the escape string literal".into(),
                    );
                } else {
                    self.validate_prefixed_string_content(
                        token_text,
                        2,
                        PrefixedStringKind::Escape,
                    );
                    self.prefixed_string_continuation = Some(PrefixedStringKind::Escape);
                }
                SyntaxKind::ESC_STRING
            }
        };

        let err = err
            .as_deref()
            .map(|msg| (msg, err_range.unwrap_or(0..token_text.len() as u32)));
        self.push(syntax_kind, token_text.len(), err);
    }

    fn validate_prefixed_string_content(
        &mut self,
        token_text: &str,
        inner_start: usize,
        kind: PrefixedStringKind,
    ) {
        let inner = &token_text[inner_start..token_text.len() - 1];
        match kind {
            PrefixedStringKind::Bit => {
                for (i, c) in inner.char_indices() {
                    if !matches!(c, '0' | '1') {
                        self.push_content_error(
                            format!(r#""{c}" is not a valid binary digit"#),
                            inner_start + i,
                            c.len_utf8(),
                        );
                    }
                }
            }
            PrefixedStringKind::Byte => {
                for (i, c) in inner.char_indices() {
                    if !c.is_ascii_hexdigit() {
                        self.push_content_error(
                            format!(r#""{c}" is not a valid hexadecimal digit"#),
                            inner_start + i,
                            c.len_utf8(),
                        );
                    }
                }
            }
            PrefixedStringKind::Escape => {
                let mut chars = inner.char_indices().peekable();
                while let Some((escape_start, c)) = chars.next() {
                    if c != '\\' {
                        continue;
                    }
                    let Some((next_pos, next_c)) = chars.next() else {
                        break;
                    };
                    let (required, example) = match next_c {
                        'u' => (4usize, r"\uXXXX"),
                        'U' => (8usize, r"\UXXXXXXXX"),
                        _ => continue,
                    };
                    let mut end = next_pos + next_c.len_utf8();
                    let mut got_all = true;
                    for _ in 0..required {
                        match chars.peek() {
                            Some(&(i, ch)) if ch.is_ascii_hexdigit() => {
                                end = i + ch.len_utf8();
                                chars.next();
                            }
                            _ => {
                                got_all = false;
                                break;
                            }
                        }
                    }
                    if !got_all {
                        self.push_content_error(
                            format!("Unicode escape requires {required} hex digits: {example}"),
                            inner_start + escape_start,
                            end - escape_start,
                        );
                    }
                }
            }
        }
    }

    fn push_content_error(&mut self, msg: String, start: usize, len: usize) {
        let token_start = self.offset as u32;
        let start = token_start + start as u32;
        self.res.error.push(LexError {
            msg,
            range: start..start + len as u32,
        });
    }
}

#[cfg(test)]
mod tests {
    use annotate_snippets::{AnnotationKind, Level, Renderer, Snippet, renderer::DecorStyle};
    use insta::{assert_debug_snapshot, assert_snapshot};

    use super::LexedStr;

    fn lex(text: &str) -> String {
        let lexed = LexedStr::new(text);
        let renderer = Renderer::plain().decor_style(DecorStyle::Unicode);
        let mut res = String::new();

        for (range, msg) in lexed.errors() {
            let span = range.start as usize..range.end as usize;
            let group = Level::ERROR.primary_title(msg).element(
                Snippet::source(text)
                    .fold(true)
                    .annotation(AnnotationKind::Primary.span(span)),
            );
            res.push_str(&renderer.render(&[group]).to_string());
            res.push('\n');
        }

        res
    }

    fn lex_errors(text: &str) -> Vec<(std::ops::Range<u32>, String)> {
        LexedStr::new(text)
            .errors()
            .map(|(range, msg)| (range.clone(), msg.to_owned()))
            .collect()
    }

    #[test]
    fn prefixed_string_content_errors() {
        assert_debug_snapshot!(lex_errors("B'102' X'1G' E'\\u00'"), @r#"
        [
            (
                4..5,
                "\"2\" is not a valid binary digit",
            ),
            (
                10..11,
                "\"G\" is not a valid hexadecimal digit",
            ),
            (
                15..19,
                "Unicode escape requires 4 hex digits: \\uXXXX",
            ),
        ]
        "#);
    }

    #[test]
    fn prefixed_string_continuations_use_the_initial_string_kind() {
        assert_debug_snapshot!(lex_errors("B'0'\n'2'"), @r#"
        [
            (
                6..7,
                "\"2\" is not a valid binary digit",
            ),
        ]
        "#);
        assert_debug_snapshot!(lex_errors("X'F'\n'G'"), @r#"
        [
            (
                6..7,
                "\"G\" is not a valid hexadecimal digit",
            ),
        ]
        "#);
        assert_debug_snapshot!(lex_errors("E'ok'\n'\\u0'"), @r#"
        [
            (
                7..10,
                "Unicode escape requires 4 hex digits: \\uXXXX",
            ),
        ]
        "#);
    }

    #[test]
    fn prefixed_string_continuation_state_resets() {
        assert_debug_snapshot!(
            lex_errors("B'01' || '2'; X'0F' N'x' 'G'; E'ok' $tag$x$tag$ '\\u0'"),
            @"[]"
        );
        assert_debug_snapshot!(lex_errors("B'01' 2 '2'; X'0F' 1.5 'G'"), @"[]");
    }

    #[test]
    fn empty_int_error() {
        assert_snapshot!(lex("select 0x;"), @"
        error: Missing digits after the integer base prefix
          ╭▸ 
        1 │ select 0x;
          ╰╴       ━━
        ");
    }

    #[test]
    fn empty_int_with_trailing_ident_error() {
        assert_snapshot!(lex("select 0xg;"), @"
        error: trailing junk after numeric literal
          ╭▸ 
        1 │ select 0xg;
          ╰╴         ━
        ");
    }

    #[test]
    fn invalid_octal_digits_error() {
        assert_snapshot!(lex("select 0o999;"), @"
        error: invalid digit for a base 8 literal
          ╭▸ 
        1 │ select 0o999;
          ╰╴         ━
        error: invalid digit for a base 8 literal
          ╭▸ 
        1 │ select 0o999;
          ╰╴          ━
        error: invalid digit for a base 8 literal
          ╭▸ 
        1 │ select 0o999;
          ╰╴           ━
        ");
    }

    #[test]
    fn invalid_binary_digits_error() {
        assert_snapshot!(lex("select 0b234;"), @"
        error: invalid digit for a base 2 literal
          ╭▸ 
        1 │ select 0b234;
          ╰╴         ━
        error: invalid digit for a base 2 literal
          ╭▸ 
        1 │ select 0b234;
          ╰╴          ━
        error: invalid digit for a base 2 literal
          ╭▸ 
        1 │ select 0b234;
          ╰╴           ━
        ");
    }

    #[test]
    fn invalid_octal_digits_after_valid_error() {
        assert_snapshot!(lex("select 0o7889;"), @"
        error: invalid digit for a base 8 literal
          ╭▸ 
        1 │ select 0o7889;
          ╰╴          ━
        error: invalid digit for a base 8 literal
          ╭▸ 
        1 │ select 0o7889;
          ╰╴           ━
        error: invalid digit for a base 8 literal
          ╭▸ 
        1 │ select 0o7889;
          ╰╴            ━
        ");
    }

    #[test]
    fn empty_exponent_error() {
        assert_snapshot!(lex("select 1e;"), @"
        error: Missing digits after the exponent symbol
          ╭▸ 
        1 │ select 1e;
          ╰╴        ━
        ");
    }

    #[test]
    fn unterminated_string_error() {
        assert_snapshot!(lex("select 'hello;"), @"
        error: Missing trailing `'` symbol to terminate the string literal
          ╭▸ 
        1 │ select 'hello;
          ╰╴       ━━━━━━━
        ");
    }

    #[test]
    fn unterminated_hex_bit_string_error() {
        assert_snapshot!(lex("select X'1F;"), @"
        error: Missing trailing `'` symbol to terminate the hex bit string literal
          ╭▸ 
        1 │ select X'1F;
          ╰╴       ━━━━━
        ");
    }

    #[test]
    fn unterminated_bit_string_error() {
        assert_snapshot!(lex("select B'101;"), @"
        error: Missing trailing `'` symbol to terminate the bit string literal
          ╭▸ 
        1 │ select B'101;
          ╰╴       ━━━━━━
        ");
    }

    #[test]
    fn unterminated_dollar_quoted_string_error() {
        assert_snapshot!(lex("select $tag$hello;"), @"
        error: Unterminated dollar quoted string literal
          ╭▸ 
        1 │ select $tag$hello;
          ╰╴       ━━━━━━━━━━━
        ");
    }

    #[test]
    fn unterminated_unicode_escape_string_error() {
        assert_snapshot!(lex("select U&'hello;"), @"
        error: Missing trailing `'` symbol to terminate the unicode escape string literal
          ╭▸ 
        1 │ select U&'hello;
          ╰╴       ━━━━━━━━━
        ");
    }

    #[test]
    fn unterminated_escape_string_error() {
        assert_snapshot!(lex("select E'hello;"), @"
        error: Missing trailing `'` symbol to terminate the escape string literal
          ╭▸ 
        1 │ select E'hello;
          ╰╴       ━━━━━━━━
        ");
    }
}
