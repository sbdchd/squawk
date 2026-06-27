mod cursor;
mod token;
use cursor::{Cursor, EOF_CHAR};
pub use token::{Base, LiteralKind, Token, TokenKind};

// via: https://github.com/postgres/postgres/blob/db0c96cc18aec417101e37e59fcc53d4bf647915/src/backend/parser/scan.l#L346
// ident_start		[A-Za-z\200-\377_]
const fn is_ident_start(c: char) -> bool {
    matches!(c, 'a'..='z' | 'A'..='Z' | '_' | '\u{80}'..)
}

// ident_cont		[A-Za-z\200-\377_0-9\$]
const fn is_ident_cont(c: char) -> bool {
    matches!(c, 'a'..='z' | 'A'..='Z' | '_' | '0'..='9' | '$' | '\u{80}'..)
}

// see:
// - https://github.com/postgres/postgres/blob/db0c96cc18aec417101e37e59fcc53d4bf647915/src/backend/parser/scansup.c#L107-L128
// - https://github.com/postgres/postgres/blob/db0c96cc18aec417101e37e59fcc53d4bf647915/src/backend/parser/scan.l#L204-L229
const fn is_whitespace(c: char) -> bool {
    matches!(
        c,
        ' ' // space
        | '\t' // tab
        | '\n' // newline
        | '\r' // carriage return
        | '\u{000B}' // vertical tab
        | '\u{000C}' // form feed
    )
}

impl Cursor<'_> {
    // see: https://github.com/rust-lang/rust/blob/ba1d7f4a083e6402679105115ded645512a7aea8/compiler/rustc_lexer/src/lib.rs#L339
    pub(crate) fn advance_token(&mut self) -> Token {
        let Some(first_char) = self.bump() else {
            return Token::new(TokenKind::Eof, 0);
        };
        let token_kind = match first_char {
            // Slash, comment or block comment.
            '/' => match self.first() {
                '*' => self.block_comment(),
                _ => TokenKind::Slash,
            },
            '-' => match self.first() {
                '-' => self.line_comment(),
                _ => TokenKind::Minus,
            },

            // // Whitespace sequence.
            c if is_whitespace(c) => self.whitespace(),

            // https://www.postgresql.org/docs/current/sql-syntax-lexical.html#SQL-SYNTAX-STRINGS-UESCAPE
            'u' | 'U' => {
                if self.first() == '&' && matches!(self.second(), '\'' | '"') {
                    self.bump();
                    self.prefixed_string(
                        |terminated| LiteralKind::UnicodeEscStr { terminated },
                        true,
                        false,
                    )
                } else {
                    self.ident()
                }
            }
            // escaped strings
            'e' | 'E' => {
                self.prefixed_string(|terminated| LiteralKind::EscStr { terminated }, false, true)
            }

            // bit string
            'b' | 'B' => self.prefixed_string(
                |terminated| LiteralKind::BitStr { terminated },
                false,
                false,
            ),

            // hexadecimal byte string
            'x' | 'X' => self.prefixed_string(
                |terminated| LiteralKind::ByteStr { terminated },
                false,
                false,
            ),

            // national character string
            'n' | 'N' => match self.first() {
                '\'' => {
                    self.bump();
                    let terminated = self.single_quoted_string(false);
                    TokenKind::Literal {
                        kind: LiteralKind::NationalStr { terminated },
                    }
                }
                _ => self.ident(),
            },

            // Identifier (this should be checked after other variant that can
            // start as identifier).
            c if is_ident_start(c) => self.ident(),

            // Numeric literal.
            // see: https://www.postgresql.org/docs/current/sql-syntax-lexical.html#SQL-SYNTAX-CONSTANTS-NUMERIC
            c @ '0'..='9' => {
                let literal_kind = self.number(c);
                TokenKind::Literal { kind: literal_kind }
            }
            '.' => match self.first() {
                '0'..='9' => {
                    let literal_kind = self.number('.');
                    TokenKind::Literal { kind: literal_kind }
                }
                _ => TokenKind::Dot,
            },
            // One-symbol tokens.
            ';' => TokenKind::Semi,
            ',' => TokenKind::Comma,
            '(' => TokenKind::OpenParen,
            ')' => TokenKind::CloseParen,
            '[' => TokenKind::OpenBracket,
            ']' => TokenKind::CloseBracket,
            '{' => TokenKind::OpenCurly,
            '}' => TokenKind::CloseCurly,
            '@' => TokenKind::At,
            '#' => TokenKind::Pound,
            '~' => TokenKind::Tilde,
            '?' => TokenKind::Question,
            ':' => TokenKind::Colon,
            '$' => {
                if self.is_dollar_quote_start() {
                    self.dollar_quoted_string()
                } else {
                    // Parameters
                    while self.first().is_ascii_digit() {
                        self.bump();
                    }
                    let trailing_junk_start = self.pos_within_token();
                    self.eat_identifier();
                    TokenKind::PositionalParam {
                        trailing_junk_start,
                    }
                }
            }
            '`' => TokenKind::Backtick,
            '=' => TokenKind::Eq,
            '!' => TokenKind::Bang,
            '<' => TokenKind::Lt,
            '>' => TokenKind::Gt,
            '&' => TokenKind::And,
            '|' => TokenKind::Or,
            '+' => TokenKind::Plus,
            '*' => TokenKind::Star,
            '^' => TokenKind::Caret,
            '%' => TokenKind::Percent,

            // String literal
            '\'' => {
                let terminated = self.single_quoted_string(false);
                let kind = LiteralKind::Str { terminated };
                TokenKind::Literal { kind }
            }

            // Quoted indentifiers
            '"' => {
                let terminated = self.double_quoted_string();
                TokenKind::QuotedIdent {
                    terminated,
                    uescape: false,
                }
            }
            _ => TokenKind::Unknown,
        };
        let res = Token::new(token_kind, self.pos_within_token());
        self.reset_pos_within_token();
        res
    }
    pub(crate) fn ident(&mut self) -> TokenKind {
        self.eat_while(is_ident_cont);
        TokenKind::Ident
    }

    pub(crate) fn whitespace(&mut self) -> TokenKind {
        self.eat_while(is_whitespace);
        TokenKind::Whitespace
    }

    // see: https://github.com/postgres/postgres/blob/db0c96cc18aec417101e37e59fcc53d4bf647915/src/backend/parser/scan.l#L227
    // comment			("--"{non_newline}*)
    pub(crate) fn line_comment(&mut self) -> TokenKind {
        self.bump();

        self.eat_while(|c| c != '\n' && c != '\r');
        TokenKind::LineComment
    }

    // see: https://github.com/postgres/postgres/blob/db0c96cc18aec417101e37e59fcc53d4bf647915/src/backend/parser/scan.l#L324-L344
    pub(crate) fn block_comment(&mut self) -> TokenKind {
        self.bump();

        let mut depth = 1usize;
        while let Some(c) = self.bump() {
            match c {
                '/' if self.first() == '*' => {
                    self.bump();
                    depth += 1;
                }
                '*' if self.first() == '/' => {
                    self.bump();
                    depth -= 1;
                    if depth == 0 {
                        // This block comment is closed, so for a construction like "/* */ */"
                        // there will be a successfully parsed block comment "/* */"
                        // and " */" will be processed separately.
                        break;
                    }
                }
                _ => (),
            }
        }

        TokenKind::BlockComment {
            terminated: depth == 0,
        }
    }

    fn prefixed_string(
        &mut self,
        mk_kind: fn(bool) -> LiteralKind,
        allows_double: bool,
        backslash_escapes: bool,
    ) -> TokenKind {
        match self.first() {
            '\'' => {
                self.bump();
                let terminated = self.single_quoted_string(backslash_escapes);
                let kind = mk_kind(terminated);
                TokenKind::Literal { kind }
            }
            '"' if allows_double => {
                self.bump();
                let terminated = self.double_quoted_string();
                TokenKind::QuotedIdent {
                    terminated,
                    uescape: true,
                }
            }
            _ => self.ident(),
        }
    }

    fn number(&mut self, first_digit: char) -> LiteralKind {
        let mut base = Base::Decimal;
        if first_digit == '.' {
            return self.eat_fractional();
        }
        if first_digit == '0' {
            // Attempt to parse encoding base.
            match self.first() {
                // https://github.com/postgres/postgres/blob/db0c96cc18aec417101e37e59fcc53d4bf647915/src/backend/parser/scan.l#L403
                'b' | 'B' => {
                    base = Base::Binary;
                    self.bump();
                    let has_digits = self.eat_decimal_digits();
                    return self.finish_base_prefixed_int(base, has_digits);
                }
                // https://github.com/postgres/postgres/blob/db0c96cc18aec417101e37e59fcc53d4bf647915/src/backend/parser/scan.l#L402
                'o' | 'O' => {
                    base = Base::Octal;
                    self.bump();
                    let has_digits = self.eat_decimal_digits();
                    return self.finish_base_prefixed_int(base, has_digits);
                }
                // https://github.com/postgres/postgres/blob/db0c96cc18aec417101e37e59fcc53d4bf647915/src/backend/parser/scan.l#L401
                'x' | 'X' => {
                    base = Base::Hexadecimal;
                    self.bump();
                    let has_digits = self.eat_hexadecimal_digits();
                    return self.finish_base_prefixed_int(base, has_digits);
                }
                // Not a base prefix; consume additional digits.
                '0'..='9' | '_' => {
                    self.eat_decimal_digits();
                }

                // Also not a base prefix; nothing more to do here.
                '.' | 'e' | 'E' => {}

                // Just a 0.
                _ => {
                    let trailing_junk_start = self.pos_within_token();
                    self.eat_identifier();
                    return LiteralKind::Int {
                        base,
                        empty_int: false,
                        trailing_junk_start,
                    };
                }
            }
        } else {
            // No base prefix, parse number in the usual way.
            self.eat_decimal_digits();
        };

        match self.first() {
            '.' => {
                self.bump();
                self.eat_fractional()
            }
            'e' | 'E' => {
                let exponent_start = self.pos_within_token();
                self.bump();
                let empty_exponent_start = (!self.eat_numeric_exponent()).then_some(exponent_start);
                let trailing_junk_start = self.pos_within_token();
                self.eat_identifier();
                LiteralKind::Numeric {
                    empty_exponent_start,
                    trailing_junk_start,
                }
            }
            _ => {
                let trailing_junk_start = self.pos_within_token();
                self.eat_identifier();
                LiteralKind::Int {
                    base,
                    empty_int: false,
                    trailing_junk_start,
                }
            }
        }
    }

    fn single_quoted_string(&mut self, backslash_escapes: bool) -> bool {
        // Parse until either quotes are terminated or error is detected.
        loop {
            match self.first() {
                '\\' if backslash_escapes => {
                    // backslash
                    self.bump();
                    // escaped char
                    self.bump();
                }
                // Quotes might be terminated.
                '\'' => {
                    self.bump();

                    match self.first() {
                        // encountered an escaped quote ''
                        '\'' => {
                            self.bump();
                        }
                        // encountered terminating quote
                        _ => return true,
                    }
                }
                // End of file, stop parsing.
                EOF_CHAR if self.is_eof() => break,
                // Skip the character.
                _ => {
                    self.bump();
                }
            }
        }
        // String was not terminated.
        false
    }

    /// Eats double-quoted string and returns true
    /// if string is terminated.
    fn double_quoted_string(&mut self) -> bool {
        while let Some(c) = self.bump() {
            match c {
                '"' if self.first() == '"' => {
                    // Bump again to skip escaped character.
                    self.bump();
                }
                '"' => {
                    return true;
                }
                _ => (),
            }
        }
        // End of file reached.
        false
    }

    /// Check for `$$` and `$tag$`
    fn is_dollar_quote_start(&self) -> bool {
        let mut chars = self.chars();
        match chars.next() {
            // `$$...` -- empty tag
            Some('$') => true,
            // `$tag$...` -- tag chars terminated by `$`
            Some(c) if is_ident_start(c) => {
                for c in chars {
                    if c == '$' {
                        return true;
                    }
                    if !is_ident_cont(c) {
                        return false;
                    }
                }
                false
            }
            _ => false,
        }
    }

    // https://www.postgresql.org/docs/16/sql-syntax-lexical.html#SQL-SYNTAX-DOLLAR-QUOTING
    fn dollar_quoted_string(&mut self) -> TokenKind {
        // Get the start sequence of the dollar quote, i.e., 'foo' in
        // $foo$hello$foo$
        let mut start = vec![];
        while let Some(c) = self.bump() {
            match c {
                '$' => {
                    break;
                }
                _ => {
                    start.push(c);
                }
            }
        }

        // we have a dollar quoted string deliminated with `$$`
        if start.is_empty() {
            loop {
                self.eat_while(|c| c != '$');
                if self.is_eof() {
                    return TokenKind::Literal {
                        kind: LiteralKind::DollarQuotedString { terminated: false },
                    };
                }
                // eat $
                self.bump();
                if self.first() == '$' {
                    self.bump();
                    return TokenKind::Literal {
                        kind: LiteralKind::DollarQuotedString { terminated: true },
                    };
                }
            }
        } else {
            loop {
                self.eat_while(|c| c != '$');
                if self.is_eof() {
                    return TokenKind::Literal {
                        kind: LiteralKind::DollarQuotedString { terminated: false },
                    };
                }

                // Eat the leading '$' of a possible closing delimiter.
                self.bump();

                let mut matches_tag = true;
                for start_char in &start {
                    if self.first() == *start_char {
                        self.bump();
                    } else {
                        matches_tag = false;
                        break;
                    }
                }

                if matches_tag && self.first() == '$' {
                    self.bump();
                    return TokenKind::Literal {
                        kind: LiteralKind::DollarQuotedString { terminated: true },
                    };
                }
            }
        }
    }

    fn eat_decimal_digits(&mut self) -> bool {
        let mut has_digits = false;
        loop {
            match self.first() {
                '_' if self.second().is_ascii_digit() => {
                    self.bump();
                }
                '0'..='9' => {
                    has_digits = true;
                    self.bump();
                }
                _ => break,
            }
        }
        has_digits
    }

    fn finish_base_prefixed_int(&mut self, base: Base, has_digits: bool) -> LiteralKind {
        let trailing_junk_start = self.pos_within_token();
        self.eat_while(is_ident_cont);
        let has_trailing_junk = self.pos_within_token() > trailing_junk_start;
        LiteralKind::Int {
            base,
            empty_int: !has_digits && !has_trailing_junk,
            trailing_junk_start,
        }
    }

    fn eat_hexadecimal_digits(&mut self) -> bool {
        let mut has_digits = false;
        loop {
            match self.first() {
                '_' if self.second().is_ascii_hexdigit() => {
                    self.bump();
                }
                '0'..='9' | 'a'..='f' | 'A'..='F' => {
                    has_digits = true;
                    self.bump();
                }
                _ => break,
            }
        }
        has_digits
    }

    /// Eats the numeric exponent. Returns true if at least one digit was met,
    /// and returns false otherwise.
    fn eat_numeric_exponent(&mut self) -> bool {
        if self.first() == '-' || self.first() == '+' {
            if !self.second().is_ascii_digit() {
                return false;
            }
            self.bump();
        } else if !self.first().is_ascii_digit() {
            return false;
        }
        self.eat_decimal_digits()
    }

    fn eat_identifier(&mut self) {
        if is_ident_start(self.first()) {
            self.eat_while(is_ident_cont);
        }
    }

    pub(crate) fn eat_fractional(&mut self) -> crate::LiteralKind {
        let mut empty_exponent_start = None;
        if self.first().is_ascii_digit() {
            self.eat_decimal_digits();
        }
        match self.first() {
            'e' | 'E' => {
                let exponent_start = self.pos_within_token();
                self.bump();
                if !self.eat_numeric_exponent() {
                    empty_exponent_start = Some(exponent_start);
                }
            }
            _ => (),
        }
        let trailing_junk_start = self.pos_within_token();
        self.eat_identifier();
        LiteralKind::Numeric {
            empty_exponent_start,
            trailing_junk_start,
        }
    }
}

/// Creates an iterator that produces tokens from the input string.
pub fn tokenize(input: &str) -> impl Iterator<Item = Token> + '_ {
    let mut cursor = Cursor::new(input);
    std::iter::from_fn(move || {
        let token = cursor.advance_token();
        if token.kind != TokenKind::Eof {
            Some(token)
        } else {
            None
        }
    })
}

#[cfg(test)]
mod tests {
    use std::fmt;

    use super::*;
    use insta::assert_debug_snapshot;

    struct TokenDebug<'a> {
        content: &'a str,
        token: Token,
    }
    impl fmt::Debug for TokenDebug<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{:?} @ {:?}", self.content, self.token.kind)
        }
    }

    impl<'a> TokenDebug<'a> {
        fn new(token: Token, input: &'a str, start: u32) -> TokenDebug<'a> {
            TokenDebug {
                token,
                content: &input[start as usize..(start + token.len) as usize],
            }
        }
    }

    fn lex(input: &str) -> Vec<TokenDebug<'_>> {
        let mut tokens = vec![];
        let mut start = 0;

        for token in tokenize(input) {
            let length = token.len;
            tokens.push(TokenDebug::new(token, input, start));
            start += length;
        }
        tokens
    }
    #[test]
    fn lex_statement() {
        let result = lex("select 1;");
        assert_debug_snapshot!(result);
    }

    #[test]
    fn block_comment() {
        let result = lex(r#"
/*
 * foo
 * bar
*/"#);
        assert_debug_snapshot!(result);
    }

    #[test]
    fn block_comment_unterminated() {
        let result = lex(r#"
/*
 * foo
 * bar
 /*
*/"#);
        assert_debug_snapshot!(result);
    }

    #[test]
    fn line_comment() {
        let result = lex(r#"
-- foooooooooooo bar buzz
"#);
        assert_debug_snapshot!(result);
    }

    #[test]
    fn line_comment_cr_newline() {
        assert_debug_snapshot!(lex("select 1; -- comment\rselect 2;"), @r#"
        [
            "select" @ Ident,
            " " @ Whitespace,
            "1" @ Literal { kind: Int { base: Decimal, empty_int: false, trailing_junk_start: 1 } },
            ";" @ Semi,
            " " @ Whitespace,
            "-- comment" @ LineComment,
            "\r" @ Whitespace,
            "select" @ Ident,
            " " @ Whitespace,
            "2" @ Literal { kind: Int { base: Decimal, empty_int: false, trailing_junk_start: 1 } },
            ";" @ Semi,
        ]
        "#);
    }

    #[test]
    fn line_comment_whitespace() {
        assert_debug_snapshot!(lex(r#"
select 'Hello' -- This is a comment
' World';"#))
    }

    #[test]
    fn dollar_quoting() {
        assert_debug_snapshot!(lex(r#"
$$Dianne's horse$$
$SomeTag$Dianne's horse$SomeTag$

-- with dollar inside and matching tags
$foo$hello$world$bar$
"#))
    }

    #[test]
    fn dollar_strings_part2() {
        assert_debug_snapshot!(lex(r#"
DO $doblock$
end
$doblock$;"#))
    }

    #[test]
    fn dollar_quote_mismatch_tags_simple() {
        assert_debug_snapshot!(lex(r#"
-- dollar quoting with mismatched tags
$foo$hello world$bar$
"#));
    }

    #[test]
    fn dollar_quote_mismatch_tags_complex() {
        assert_debug_snapshot!(lex(r#"
-- with dollar inside but mismatched tags
$foo$hello$world$bar$
"#));
    }

    #[test]
    fn numeric() {
        assert_debug_snapshot!(lex(r#"
42
3.5
4.
.001
.123e10
5e2
1.925e-3
1e-10
1e+10
1e10
4664.E+5
"#))
    }

    #[test]
    fn numeric_non_decimal() {
        assert_debug_snapshot!(lex(r#"
0b100101
0B10011001
0o273
0O755
0x42f
0XFFFF
"#))
    }

    #[test]
    fn numeric_with_seperators() {
        assert_debug_snapshot!(lex(r#"
1_500_000_000
0b10001000_00000000
0o_1_755
0xFFFF_FFFF
1.618_034
"#))
    }

    #[test]
    fn numeric_leading_dot_with_separators() {
        assert_debug_snapshot!(lex(".1_2 .5_5 .1_2e3"), @r#"
        [
            ".1_2" @ Literal { kind: Numeric { empty_exponent_start: None, trailing_junk_start: 4 } },
            " " @ Whitespace,
            ".5_5" @ Literal { kind: Numeric { empty_exponent_start: None, trailing_junk_start: 4 } },
            " " @ Whitespace,
            ".1_2e3" @ Literal { kind: Numeric { empty_exponent_start: None, trailing_junk_start: 6 } },
        ]
        "#)
    }

    #[test]
    fn numeric_exponent_underscore_after_sign() {
        assert_debug_snapshot!(lex("1e+_2 1e-_2 1.0e+_2 .1e+_2"), @r#"
        [
            "1e" @ Literal { kind: Numeric { empty_exponent_start: Some(1), trailing_junk_start: 2 } },
            "+" @ Plus,
            "_2" @ Ident,
            " " @ Whitespace,
            "1e" @ Literal { kind: Numeric { empty_exponent_start: Some(1), trailing_junk_start: 2 } },
            "-" @ Minus,
            "_2" @ Ident,
            " " @ Whitespace,
            "1.0e" @ Literal { kind: Numeric { empty_exponent_start: Some(3), trailing_junk_start: 4 } },
            "+" @ Plus,
            "_2" @ Ident,
            " " @ Whitespace,
            ".1e" @ Literal { kind: Numeric { empty_exponent_start: Some(2), trailing_junk_start: 3 } },
            "+" @ Plus,
            "_2" @ Ident,
        ]
        "#)
    }

    #[test]
    fn select_with_period() {
        assert_debug_snapshot!(lex(r#"
select public.users;
"#))
    }

    #[test]
    fn bitstring() {
        assert_debug_snapshot!(lex(r#"
B'1001'
b'1001'
X'1FF'
x'1FF'
"#))
    }

    #[test]
    fn national_character_string() {
        assert_debug_snapshot!(lex("N'foo' n'bar' numeric'1'"), @r#"
        [
            "N'foo'" @ Literal { kind: NationalStr { terminated: true } },
            " " @ Whitespace,
            "n'bar'" @ Literal { kind: NationalStr { terminated: true } },
            " " @ Whitespace,
            "numeric" @ Ident,
            "'1'" @ Literal { kind: Str { terminated: true } },
        ]
        "#);
    }

    #[test]
    fn ident_prefix_then_string_is_consistent() {
        assert_debug_snapshot!(
            lex("N1'foo' E1'foo' B1'foo' X1'foo' U1'foo' uuid'00000000'"),
            @r#"
        [
            "N1" @ Ident,
            "'foo'" @ Literal { kind: Str { terminated: true } },
            " " @ Whitespace,
            "E1" @ Ident,
            "'foo'" @ Literal { kind: Str { terminated: true } },
            " " @ Whitespace,
            "B1" @ Ident,
            "'foo'" @ Literal { kind: Str { terminated: true } },
            " " @ Whitespace,
            "X1" @ Ident,
            "'foo'" @ Literal { kind: Str { terminated: true } },
            " " @ Whitespace,
            "U1" @ Ident,
            "'foo'" @ Literal { kind: Str { terminated: true } },
            " " @ Whitespace,
            "uuid" @ Ident,
            "'00000000'" @ Literal { kind: Str { terminated: true } },
        ]
        "#);
    }

    #[test]
    fn string() {
        assert_debug_snapshot!(lex(r#"
'Dianne''s horse'

select 'foo ''
bar';

select 'foooo'   
   'bar';


'foo \\ \n \tbar'

'forgot to close the string
"#))
    }

    #[test]
    fn params() {
        assert_debug_snapshot!(lex(r#"
select $1 + $2;

select $1123123123123;

select $;
"#))
    }

    #[test]
    fn string_with_escapes() {
        // https://www.postgresql.org/docs/current/sql-syntax-lexical.html#SQL-SYNTAX-STRINGS-ESCAPE

        assert_debug_snapshot!(lex(r#"
E'foo'

e'bar'

e'\b\f\n\r\t'

e'\0\11\777'

e'\x0\x11\xFF'

e'\uAAAA \UFFFFFFFF'

"#))
    }

    #[test]
    fn escape_string_with_backslash_escaped_quote() {
        assert_debug_snapshot!(lex(r"E'foo\'bar'"), @r#"
        [
            "E'foo\\'bar'" @ Literal { kind: EscStr { terminated: true } },
        ]
        "#);
    }

    #[test]
    fn escape_string_with_escaped_terminal_quote_is_unterminated() {
        assert_debug_snapshot!(lex(r"E'foo\';"), @r#"
        [
            "E'foo\\';" @ Literal { kind: EscStr { terminated: false } },
        ]
        "#);
    }

    #[test]
    fn escape_string_with_even_backslashes_before_quote_is_terminated() {
        assert_debug_snapshot!(lex(r"E'foo\\'"), @r#"
        [
            "E'foo\\\\'" @ Literal { kind: EscStr { terminated: true } },
        ]
        "#);
    }

    #[test]
    fn string_unicode_escape() {
        // https://www.postgresql.org/docs/current/sql-syntax-lexical.html#SQL-SYNTAX-STRINGS-UESCAPE

        assert_debug_snapshot!(lex(r#"
U&"d\0061t\+000061"

U&"\0441\043B\043E\043D"

u&'\0441\043B'

U&"d!0061t!+000061" UESCAPE '!'
"#))
    }

    #[test]
    fn quoted_ident() {
        assert_debug_snapshot!(lex(r#"
"hello &1 -world";


"hello-world
"#))
    }

    #[test]
    fn quoted_ident_with_escape_quote() {
        assert_debug_snapshot!(lex(r#"
"foo "" bar"
"#))
    }

    #[test]
    fn dollar_quoted_string() {
        assert_debug_snapshot!(lex("$$$$"), @r#"
        [
            "$$$$" @ Literal { kind: DollarQuotedString { terminated: true } },
        ]
        "#);
    }

    #[test]
    fn tagged_dollar_quote_requires_leading_dollar() {
        assert_debug_snapshot!(lex("select $foo$abcfoo$def$foo$;"), @r#"
        [
            "select" @ Ident,
            " " @ Whitespace,
            "$foo$abcfoo$def$foo$" @ Literal { kind: DollarQuotedString { terminated: true } },
            ";" @ Semi,
        ]
        "#);
    }

    #[test]
    fn unclosed_dollar_tag_does_not_swallow_rest_of_input() {
        assert_debug_snapshot!(lex("select $x;\ndrop table users;"), @r#"
        [
            "select" @ Ident,
            " " @ Whitespace,
            "$x" @ PositionalParam { trailing_junk_start: 1 },
            ";" @ Semi,
            "\n" @ Whitespace,
            "drop" @ Ident,
            " " @ Whitespace,
            "table" @ Ident,
            " " @ Whitespace,
            "users" @ Ident,
            ";" @ Semi,
        ]
        "#);
    }

    #[test]
    fn ident_non_ascii_above_latin1() {
        assert_debug_snapshot!(lex("ẞ Ā 漢字 𐐷"), @r#"
        [
            "ẞ" @ Ident,
            " " @ Whitespace,
            "Ā" @ Ident,
            " " @ Whitespace,
            "漢字" @ Ident,
            " " @ Whitespace,
            "𐐷" @ Ident,
        ]
        "#);
    }
}
