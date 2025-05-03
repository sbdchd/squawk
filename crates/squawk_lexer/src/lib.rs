mod cursor;
mod token;
use cursor::{Cursor, EOF_CHAR};
pub use token::{Base, LiteralKind, Token, TokenKind};

// via: https://github.com/postgres/postgres/blob/db0c96cc18aec417101e37e59fcc53d4bf647915/src/backend/parser/scan.l#L346
// ident_start		[A-Za-z\200-\377_]
const fn is_ident_start(c: char) -> bool {
    matches!(c, 'a'..='z' | 'A'..='Z' | '_' | '\u{80}'..='\u{FF}')
}

// ident_cont		[A-Za-z\200-\377_0-9\$]
const fn is_ident_cont(c: char) -> bool {
    matches!(c, 'a'..='z' | 'A'..='Z' | '_' | '0'..='9' | '$' | '\u{80}'..='\u{FF}')
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
            'u' | 'U' => match self.first() {
                '&' => {
                    self.bump();
                    self.prefixed_string(
                        |terminated| LiteralKind::UnicodeEscStr { terminated },
                        true,
                    )
                }
                _ => self.ident_or_unknown_prefix(),
            },

            // escaped strings
            'e' | 'E' => {
                self.prefixed_string(|terminated| LiteralKind::EscStr { terminated }, false)
            }

            // bit string
            'b' | 'B' => {
                self.prefixed_string(|terminated| LiteralKind::BitStr { terminated }, false)
            }

            // hexadecimal byte string
            'x' | 'X' => {
                self.prefixed_string(|terminated| LiteralKind::ByteStr { terminated }, false)
            }

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
            '@' => TokenKind::At,
            '#' => TokenKind::Pound,
            '~' => TokenKind::Tilde,
            '?' => TokenKind::Question,
            ':' => TokenKind::Colon,
            '$' => {
                // Dollar quoted strings
                if is_ident_start(self.first()) || self.first() == '$' {
                    self.dollar_quoted_string()
                } else {
                    // Parameters
                    while self.first().is_ascii_digit() {
                        self.bump();
                    }
                    TokenKind::Param
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
                let terminated = self.single_quoted_string();
                let kind = LiteralKind::Str { terminated };
                TokenKind::Literal { kind }
            }

            // Quoted indentifiers
            '"' => {
                let terminated = self.double_quoted_string();
                TokenKind::QuotedIdent { terminated }
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

    fn ident_or_unknown_prefix(&mut self) -> TokenKind {
        // Start is already eaten, eat the rest of identifier.
        self.eat_while(is_ident_cont);
        // Known prefixes must have been handled earlier. So if
        // we see a prefix here, it is definitely an unknown prefix.
        match self.first() {
            '#' | '"' | '\'' => TokenKind::UnknownPrefix,
            _ => TokenKind::Ident,
        }
    }

    // see: https://github.com/postgres/postgres/blob/db0c96cc18aec417101e37e59fcc53d4bf647915/src/backend/parser/scan.l#L227
    // comment			("--"{non_newline}*)
    pub(crate) fn line_comment(&mut self) -> TokenKind {
        self.bump();

        self.eat_while(|c| c != '\n');
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
    ) -> TokenKind {
        match self.first() {
            '\'' => {
                self.bump();
                let terminated = self.single_quoted_string();
                let kind = mk_kind(terminated);
                TokenKind::Literal { kind }
            }
            '"' if allows_double => {
                self.bump();
                let terminated = self.double_quoted_string();
                let kind = mk_kind(terminated);
                TokenKind::Literal { kind }
            }
            _ => self.ident_or_unknown_prefix(),
        }
    }

    fn number(&mut self, first_digit: char) -> LiteralKind {
        let mut base = Base::Decimal;
        if first_digit == '0' {
            // Attempt to parse encoding base.
            match self.first() {
                // https://github.com/postgres/postgres/blob/db0c96cc18aec417101e37e59fcc53d4bf647915/src/backend/parser/scan.l#L403
                'b' | 'B' => {
                    base = Base::Binary;
                    self.bump();
                    if !self.eat_decimal_digits() {
                        return LiteralKind::Int {
                            base,
                            empty_int: true,
                        };
                    }
                }
                // https://github.com/postgres/postgres/blob/db0c96cc18aec417101e37e59fcc53d4bf647915/src/backend/parser/scan.l#L402
                'o' | 'O' => {
                    base = Base::Octal;
                    self.bump();
                    if !self.eat_decimal_digits() {
                        return LiteralKind::Int {
                            base,
                            empty_int: true,
                        };
                    }
                }
                // https://github.com/postgres/postgres/blob/db0c96cc18aec417101e37e59fcc53d4bf647915/src/backend/parser/scan.l#L401
                'x' | 'X' => {
                    base = Base::Hexadecimal;
                    self.bump();
                    if !self.eat_hexadecimal_digits() {
                        return LiteralKind::Int {
                            base,
                            empty_int: true,
                        };
                    }
                }
                // Not a base prefix; consume additional digits.
                '0'..='9' | '_' => {
                    self.eat_decimal_digits();
                }

                // Also not a base prefix; nothing more to do here.
                '.' | 'e' | 'E' => {}

                // Just a 0.
                _ => {
                    return LiteralKind::Int {
                        base,
                        empty_int: false,
                    }
                }
            }
        } else {
            // No base prefix, parse number in the usual way.
            self.eat_decimal_digits();
        };

        match self.first() {
            '.' => {
                // might have stuff after the ., and if it does, it needs to start
                // with a number
                self.bump();
                let mut empty_exponent = false;
                if self.first().is_ascii_digit() {
                    self.eat_decimal_digits();
                    match self.first() {
                        'e' | 'E' => {
                            self.bump();
                            empty_exponent = !self.eat_float_exponent();
                        }
                        _ => (),
                    }
                }
                LiteralKind::Float {
                    base,
                    empty_exponent,
                }
            }
            'e' | 'E' => {
                self.bump();
                let empty_exponent = !self.eat_float_exponent();
                LiteralKind::Float {
                    base,
                    empty_exponent,
                }
            }
            _ => LiteralKind::Int {
                base,
                empty_int: false,
            },
        }
    }

    fn single_quoted_string(&mut self) -> bool {
        // Parse until either quotes are terminated or error is detected.
        loop {
            match self.first() {
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
                '"' => {
                    return true;
                }
                '\\' if self.first() == '\\' || self.first() == '"' => {
                    // Bump again to skip escaped character.
                    self.bump();
                }
                _ => (),
            }
        }
        // End of file reached.
        false
    }

    // https://www.postgresql.org/docs/16/sql-syntax-lexical.html#SQL-SYNTAX-DOLLAR-QUOTING
    fn dollar_quoted_string(&mut self) -> TokenKind {
        // Get the start sequence of the dollar quote, i.e., 'foo' in
        // $foo$hello$foo$
        let mut start = vec![];
        while let Some(c) = self.bump() {
            match c {
                '$' => {
                    self.bump();
                    break;
                }
                _ => {
                    start.push(c);
                }
            }
        }

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
                self.eat_while(|c| c != start[0]);
                if self.is_eof() {
                    return TokenKind::Literal {
                        kind: LiteralKind::DollarQuotedString { terminated: false },
                    };
                }

                // might be the start of our start/end sequence
                let mut match_count = 0;
                for start_char in &start {
                    if self.first() == *start_char {
                        self.bump();
                        match_count += 1;
                    } else {
                        self.bump();
                        break;
                    }
                }

                // closing '$'
                if self.first() == '$' {
                    self.bump();
                    let terminated = match_count == start.len();
                    return TokenKind::Literal {
                        kind: LiteralKind::DollarQuotedString { terminated },
                    };
                }
            }
        }
    }

    fn eat_decimal_digits(&mut self) -> bool {
        let mut has_digits = false;
        loop {
            match self.first() {
                '_' => {
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

    fn eat_hexadecimal_digits(&mut self) -> bool {
        let mut has_digits = false;
        loop {
            match self.first() {
                '_' => {
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

    /// Eats the float exponent. Returns true if at least one digit was met,
    /// and returns false otherwise.
    fn eat_float_exponent(&mut self) -> bool {
        if self.first() == '-' || self.first() == '+' {
            self.bump();
        }
        self.eat_decimal_digits()
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

    fn lex(input: &str) -> Vec<TokenDebug> {
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
}
