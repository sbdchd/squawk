// via https://github.com/rust-lang/rust-analyzer/blob/d8887c0758bbd2d5f752d5bd405d4491e90e7ed6/crates/parser/src/shortcuts.rs
//
// Permission is hereby granted, free of charge, to any
// person obtaining a copy of this software and associated
// documentation files (the "Software"), to deal in the
// Software without restriction, including without
// limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following
// conditions:
//
// The above copyright notice and this permission notice
// shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
// ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
// TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
// PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
// SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

use std::mem;

use crate::{
    lexed_str::LexedStr,
    output::{Output, Step},
    syntax_kind::SyntaxKind,
};

#[derive(Debug)]
pub enum StrStep<'a> {
    Token { kind: SyntaxKind, text: &'a str },
    Enter { kind: SyntaxKind },
    Exit,
    Error { msg: &'a str, pos: usize },
}

enum State {
    PendingEnter,
    Normal,
    PendingExit,
}

struct Builder<'a, 'b> {
    lexed: &'a LexedStr<'a>,
    pos: usize,
    state: State,
    sink: &'b mut dyn FnMut(StrStep<'_>),
}

impl Builder<'_, '_> {
    fn token(&mut self, kind: SyntaxKind, n_tokens: u8) {
        match mem::replace(&mut self.state, State::Normal) {
            State::PendingEnter => unreachable!(),
            State::PendingExit => (self.sink)(StrStep::Exit),
            State::Normal => (),
        }
        self.eat_trivias();
        self.do_token(kind, n_tokens as usize);
    }

    fn float_split(&mut self, has_pseudo_dot: bool) {
        match mem::replace(&mut self.state, State::Normal) {
            State::PendingEnter => unreachable!(),
            State::PendingExit => (self.sink)(StrStep::Exit),
            State::Normal => (),
        }
        self.eat_trivias();
        self.do_float_split(has_pseudo_dot);
    }

    fn enter(&mut self, kind: SyntaxKind) {
        match mem::replace(&mut self.state, State::Normal) {
            State::PendingEnter => {
                (self.sink)(StrStep::Enter { kind });
                // No need to attach trivias to previous node: there is no
                // previous node.
                return;
            }
            State::PendingExit => (self.sink)(StrStep::Exit),
            State::Normal => (),
        }

        let n_trivias = (self.pos..self.lexed.len())
            .take_while(|&it| self.lexed.kind(it).is_trivia())
            .count();
        let leading_trivias = self.pos..self.pos + n_trivias;
        let n_attached_trivias = n_attached_trivias(
            kind,
            leading_trivias
                .rev()
                .map(|it| (self.lexed.kind(it), self.lexed.text(it))),
        );
        self.eat_n_trivias(n_trivias - n_attached_trivias);
        (self.sink)(StrStep::Enter { kind });
        self.eat_n_trivias(n_attached_trivias);
    }

    fn exit(&mut self) {
        match mem::replace(&mut self.state, State::PendingExit) {
            State::PendingEnter => unreachable!(),
            State::PendingExit => (self.sink)(StrStep::Exit),
            State::Normal => (),
        }
    }

    fn eat_trivias(&mut self) {
        while self.pos < self.lexed.len() {
            let kind = self.lexed.kind(self.pos);
            if !kind.is_trivia() {
                break;
            }
            self.do_token(kind, 1);
        }
    }

    fn eat_n_trivias(&mut self, n: usize) {
        for _ in 0..n {
            let kind = self.lexed.kind(self.pos);
            assert!(kind.is_trivia());
            self.do_token(kind, 1);
        }
    }

    fn do_token(&mut self, kind: SyntaxKind, n_tokens: usize) {
        let text = &self.lexed.range_text(self.pos..self.pos + n_tokens);
        self.pos += n_tokens;
        (self.sink)(StrStep::Token { kind, text });
    }

    fn do_float_split(&mut self, has_pseudo_dot: bool) {
        let text = &self.lexed.range_text(self.pos..self.pos + 1);

        match text.split_once('.') {
            Some((left, right)) => {
                assert!(!left.is_empty());
                (self.sink)(StrStep::Enter {
                    kind: SyntaxKind::NAME_REF,
                });
                (self.sink)(StrStep::Token {
                    kind: SyntaxKind::INT_NUMBER,
                    text: left,
                });
                (self.sink)(StrStep::Exit);

                // here we move the exit up, the original exit has been deleted in process
                (self.sink)(StrStep::Exit);

                (self.sink)(StrStep::Token {
                    kind: SyntaxKind::DOT,
                    text: ".",
                });

                if has_pseudo_dot {
                    assert!(right.is_empty(), "{left}.{right}");
                    self.state = State::Normal;
                } else {
                    assert!(!right.is_empty(), "{left}.{right}");
                    (self.sink)(StrStep::Enter {
                        kind: SyntaxKind::NAME_REF,
                    });
                    (self.sink)(StrStep::Token {
                        kind: SyntaxKind::INT_NUMBER,
                        text: right,
                    });
                    (self.sink)(StrStep::Exit);

                    // the parser creates an unbalanced start node, we are required to close it here
                    self.state = State::PendingExit;
                }
            }
            None => {
                // illegal float literal which doesn't have dot in form (like 1e0)
                // we should emit an error node here
                (self.sink)(StrStep::Error {
                    msg: "illegal float literal",
                    pos: self.pos,
                });
                (self.sink)(StrStep::Enter {
                    kind: SyntaxKind::ERROR,
                });
                (self.sink)(StrStep::Token {
                    kind: SyntaxKind::FLOAT_NUMBER,
                    text,
                });
                (self.sink)(StrStep::Exit);

                // move up
                (self.sink)(StrStep::Exit);

                self.state = if has_pseudo_dot {
                    State::Normal
                } else {
                    State::PendingExit
                };
            }
        }

        self.pos += 1;
    }
}

impl LexedStr<'_> {
    pub fn to_input(&self) -> crate::Input {
        let mut res = crate::Input::default();
        let mut was_joint = false;
        for i in 0..self.len() {
            let kind = self.kind(i);
            if kind.is_trivia() {
                was_joint = false;
                // skip over any triva since the parser shouldn't have to deal
                // with it
            }
            // else if kind == SyntaxKind::IDENT {
            //     let token_text = self.text(i);
            //     let contextual_kw =
            //         SyntaxKind::from_contextual_keyword(token_text).unwrap_or(SyntaxKind::IDENT);
            //     res.push_ident(contextual_kw);
            // }
            else {
                if was_joint {
                    res.was_joint();
                }
                res.push(kind);
                // Tag the token as joint if it is float with a fractional part
                // we use this jointness to inform the parser about what token split
                // event to emit when we encounter a float literal in a field access
                // if kind == SyntaxKind::FLOAT_NUMBER {
                //     if !self.text(i).ends_with('.') {
                //         res.was_joint();
                //     } else {
                //         was_joint = false;
                //     }
                // } else {
                was_joint = true;
                // }
            }
        }
        res
    }

    /// NB: only valid to call with Output from Reparser/TopLevelEntry.
    pub fn intersperse_trivia(&self, output: &Output, sink: &mut dyn FnMut(StrStep<'_>)) -> bool {
        let mut builder = Builder {
            lexed: self,
            pos: 0,
            state: State::PendingEnter,
            sink,
        };

        for event in output.iter() {
            match event {
                Step::Token {
                    kind,
                    n_input_tokens: n_raw_tokens,
                } => builder.token(kind, n_raw_tokens),
                Step::FloatSplit {
                    ends_in_dot: has_pseudo_dot,
                } => builder.float_split(has_pseudo_dot),
                Step::Enter { kind } => builder.enter(kind),
                Step::Exit => builder.exit(),
                Step::Error { msg } => {
                    let text_pos = builder.lexed.text_start(builder.pos);
                    (builder.sink)(StrStep::Error { msg, pos: text_pos });
                }
            }
        }

        match mem::replace(&mut builder.state, State::Normal) {
            State::PendingExit => {
                builder.eat_trivias();
                (builder.sink)(StrStep::Exit);
            }
            State::PendingEnter | State::Normal => unreachable!(),
        }

        // is_eof?
        builder.pos == builder.lexed.len()
    }
}

fn n_attached_trivias<'a>(
    kind: SyntaxKind,
    trivias: impl Iterator<Item = (SyntaxKind, &'a str)>,
) -> usize {
    match kind {
        // CONST | ENUM | FN | IMPL | MACRO_CALL | MACRO_DEF | MACRO_RULES | MODULE | RECORD_FIELD
        // | STATIC | STRUCT | TRAIT | TUPLE_FIELD | TYPE_ALIAS | UNION | USE | VARIANT
        // | EXTERN_CRATE
        SyntaxKind::CREATE_TABLE => {
            let mut res = 0;
            let mut trivias = trivias.enumerate().peekable();

            while let Some((i, (kind, text))) = trivias.next() {
                match kind {
                    SyntaxKind::WHITESPACE if text.contains("\n\n") => {
                        // we check whether the next token is a doc-comment
                        // and skip the whitespace in this case
                        if let Some((SyntaxKind::COMMENT, peek_text)) =
                            trivias.peek().map(|(_, pair)| pair)
                        {
                            if is_outer(peek_text) {
                                continue;
                            }
                        }
                        break;
                    }
                    SyntaxKind::COMMENT => {
                        if is_inner(text) {
                            break;
                        }
                        res = i + 1;
                    }
                    _ => (),
                }
            }
            res
        }
        _ => 0,
    }
}

fn is_outer(text: &str) -> bool {
    if text.starts_with("////") || text.starts_with("/***") {
        return false;
    }
    text.starts_with("///") || text.starts_with("/**")
}

fn is_inner(text: &str) -> bool {
    text.starts_with("//!") || text.starts_with("/*!")
}
