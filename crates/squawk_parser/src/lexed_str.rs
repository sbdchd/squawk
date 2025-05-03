// based on https://github.com/rust-lang/rust-analyzer/blob/d8887c0758bbd2d5f752d5bd405d4491e90e7ed6/crates/parser/src/lexed_str.rs

use std::ops;

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
    token: u32,
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

    pub(crate) fn text(&self, i: usize) -> &str {
        self.range_text(i..i + 1)
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
    pub(crate) fn text_start(&self, i: usize) -> usize {
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

    pub fn errors(&self) -> impl Iterator<Item = (usize, &str)> + '_ {
        self.error
            .iter()
            .map(|it| (it.token as usize, it.msg.as_str()))
    }

    fn push(&mut self, kind: SyntaxKind, offset: usize) {
        self.kind.push(kind);
        self.start.push(offset as u32);
    }
}

struct Converter<'a> {
    res: LexedStr<'a>,
    offset: usize,
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
        }
    }

    fn finalize_with_eof(mut self) -> LexedStr<'a> {
        self.res.push(SyntaxKind::EOF, self.offset);
        self.res
    }

    fn push(&mut self, kind: SyntaxKind, len: usize, err: Option<&str>) {
        self.res.push(kind, self.offset);
        self.offset += len;

        if let Some(err) = err {
            let token = self.res.len() as u32;
            let msg = err.to_owned();
            self.res.error.push(LexError { msg, token });
        }
    }

    fn extend_token(&mut self, kind: &squawk_lexer::TokenKind, token_text: &str) {
        // A note on an intended tradeoff:
        // We drop some useful information here (see patterns with double dots `..`)
        // Storing that info in `SyntaxKind` is not possible due to its layout requirements of
        // being `u16` that come from `rowan::SyntaxKind`.
        let mut err = "";

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
                    // TODO: check for max identifier length
                    //
                    // see: https://www.postgresql.org/docs/16/sql-syntax-lexical.html#SQL-SYNTAX-IDENTIFIERS
                    // The system uses no more than NAMEDATALEN-1 bytes of an
                    // identifier; longer names can be written in commands, but
                    // they will be truncated. By default, NAMEDATALEN is 64 so
                    // the maximum identifier length is 63 bytes. If this limit
                    // is problematic, it can be raised by changing the
                    // NAMEDATALEN constant in src/include/pg_config_manual.h.
                    // see: https://github.com/postgres/postgres/blob/e032e4c7ddd0e1f7865b246ec18944365d4f8614/src/include/pg_config_manual.h#L29
                    SyntaxKind::from_keyword(token_text).unwrap_or(SyntaxKind::IDENT)
                }
                squawk_lexer::TokenKind::Literal { kind, .. } => {
                    self.extend_literal(token_text.len(), kind);
                    return;
                }
                squawk_lexer::TokenKind::Semi => SyntaxKind::SEMICOLON,
                squawk_lexer::TokenKind::Comma => SyntaxKind::COMMA,
                squawk_lexer::TokenKind::Dot => SyntaxKind::DOT,
                squawk_lexer::TokenKind::OpenParen => SyntaxKind::L_PAREN,
                squawk_lexer::TokenKind::CloseParen => SyntaxKind::R_PAREN,
                squawk_lexer::TokenKind::OpenBracket => SyntaxKind::L_BRACK,
                squawk_lexer::TokenKind::CloseBracket => SyntaxKind::R_BRACK,
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
                squawk_lexer::TokenKind::UnknownPrefix => {
                    err = "unknown literal prefix";
                    SyntaxKind::IDENT
                }
                squawk_lexer::TokenKind::Eof => SyntaxKind::EOF,
                squawk_lexer::TokenKind::Backtick => SyntaxKind::BACKTICK,
                squawk_lexer::TokenKind::Param => SyntaxKind::PARAM,
                squawk_lexer::TokenKind::QuotedIdent { terminated } => {
                    if !terminated {
                        err = "Missing trailing \" to terminate the quoted identifier"
                    }
                    SyntaxKind::IDENT
                }
            }
        };

        let err = if err.is_empty() { None } else { Some(err) };
        self.push(syntax_kind, token_text.len(), err);
    }

    fn extend_literal(&mut self, len: usize, kind: &squawk_lexer::LiteralKind) {
        let mut err = "";

        let syntax_kind = match *kind {
            squawk_lexer::LiteralKind::Int { empty_int, base: _ } => {
                if empty_int {
                    err = "Missing digits after the integer base prefix";
                }
                SyntaxKind::INT_NUMBER
            }
            squawk_lexer::LiteralKind::Float {
                empty_exponent,
                base: _,
            } => {
                if empty_exponent {
                    err = "Missing digits after the exponent symbol";
                }
                SyntaxKind::FLOAT_NUMBER
            }
            squawk_lexer::LiteralKind::Str { terminated } => {
                if !terminated {
                    err = "Missing trailing `'` symbol to terminate the string literal";
                }
                // TODO: rust analzyer checks for un-escaped strings, we should too
                SyntaxKind::STRING
            }
            squawk_lexer::LiteralKind::ByteStr { terminated } => {
                if !terminated {
                    err = "Missing trailing `'` symbol to terminate the hex bit string literal";
                }
                // TODO: rust analzyer checks for un-escaped strings, we should too
                SyntaxKind::BYTE_STRING
            }
            squawk_lexer::LiteralKind::BitStr { terminated } => {
                if !terminated {
                    err = "Missing trailing `\'` symbol to terminate the bit string literal";
                }
                // TODO: rust analzyer checks for un-escaped strings, we should too
                SyntaxKind::BIT_STRING
            }
            squawk_lexer::LiteralKind::DollarQuotedString { terminated } => {
                if !terminated {
                    // TODO: we could be fancier and say the ending string we're looking for
                    err = "Unterminated dollar quoted string literal";
                }
                // TODO: rust analzyer checks for un-escaped strings, we should too
                SyntaxKind::DOLLAR_QUOTED_STRING
            }
            squawk_lexer::LiteralKind::UnicodeEscStr { terminated } => {
                if !terminated {
                    err = "Missing trailing `'` symbol to terminate the unicode escape string literal";
                }
                // TODO: rust analzyer checks for un-escaped strings, we should too
                SyntaxKind::BYTE_STRING
            }
            squawk_lexer::LiteralKind::EscStr { terminated } => {
                if !terminated {
                    err = "Missing trailing `\'` symbol to terminate the escape string literal";
                }
                // TODO: rust analzyer checks for un-escaped strings, we should too
                SyntaxKind::ESC_STRING
            }
        };

        let err = if err.is_empty() { None } else { Some(err) };
        self.push(syntax_kind, len, err);
    }
}
