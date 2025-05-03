// via https://github.com/rust-lang/rust-analyzer/blob/d8887c0758bbd2d5f752d5bd405d4491e90e7ed6/crates/parser/src/lib.rs
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

// https://www.scattered-thoughts.net/writing/babys-second-wasm-compiler/
//
// https://craftinginterpreters.com/parsing-expressions.html
//
// see: https://github.com/rust-lang/rust-analyzer/blob/master/crates/parser/src/parser.rs
// https://rust-analyzer.github.io/blog/2020/09/16/challeging-LR-parsing.html
// https://github.com/rust-lang/rust-analyzer/blob/master/docs/dev/syntax.md
// https://ericlippert.com/2012/06/08/red-green-trees/
// https://github.com/swiftlang/swift/tree/5e2c815edfd758f9b1309ce07bfc01c4bc20ec23/lib/Syntax
// https://swift-ast-explorer.com
// https://github.com/rust-lang/rust-analyzer/blob/cf156a7a43f822e71309e50470ac34363da26727/docs/dev/syntax.md
// https://github.com/kaleidawave/ezno/blob/8ce921e39c3d4e947063f206347b2932cee456ec/parser/src/lib.rs#L177
// https://kaleidawave.github.io/posts/sets-types-and-type-checking/
// https://github.com/m-novikov/tree-sitter-sql/tree/main
// https://github.com/withered-magic/starpls/tree/79f47e12dab8be650804ce7fa931ee5e1e116eae/crates/starpls_parser/src
// https://github.com/apache/datafusion-sqlparser-rs -- removes comments and whitespace :/

// rust analyzer has a builtin doc test like thing where you generate snapshot
// style tests from comments on top of grammar functions

use drop_bomb::DropBomb;
use event::Event;
use grammar::OPERATOR_FIRST;
use std::cell::Cell;
use token_set::TokenSet;
mod grammar;
mod token_set;

mod lexed_str;
mod shortcuts;
mod syntax_kind;

mod event;
mod input;
mod output;

#[cfg(test)]
mod test;

pub use crate::{
    lexed_str::LexedStr,
    // output::{Output, Step},
    shortcuts::StrStep,
    syntax_kind::SyntaxKind,
};

use crate::input::Input;
pub use crate::output::Output;

/// See [`Parser::start`].
pub(crate) struct Marker {
    pos: u32,
    bomb: DropBomb,
}

impl Marker {
    fn new(pos: u32) -> Marker {
        Marker {
            pos,
            bomb: DropBomb::new("Marker must be either completed or abandoned"),
        }
    }

    /// Finishes the syntax tree node and assigns `kind` to it,
    /// and mark the create a `CompletedMarker` for possible future
    /// operation like `.precede()` to deal with `forward_parent`.
    pub(crate) fn complete(mut self, p: &mut Parser<'_>, kind: SyntaxKind) -> CompletedMarker {
        self.bomb.defuse();
        let idx = self.pos as usize;
        match &mut p.events[idx] {
            Event::Start { kind: slot, .. } => {
                *slot = kind;
            }
            _ => unreachable!(),
        }
        p.push_event(Event::Finish);
        CompletedMarker::new(self.pos, kind)
    }

    /// Abandons the syntax tree node. All its children
    /// are attached to its parent instead.
    pub(crate) fn abandon(mut self, p: &mut Parser<'_>) {
        self.bomb.defuse();
        let idx = self.pos as usize;
        if idx == p.events.len() - 1 {
            match p.events.pop() {
                Some(Event::Start {
                    kind: SyntaxKind::TOMBSTONE,
                    forward_parent: None,
                }) => (),
                _ => unreachable!(),
            }
        }
    }
}

pub(crate) struct CompletedMarker {
    pos: u32,
    kind: SyntaxKind,
}

impl CompletedMarker {
    fn new(pos: u32, kind: SyntaxKind) -> Self {
        CompletedMarker { pos, kind }
    }

    /// This method allows to create a new node which starts
    /// *before* the current one. That is, parser could start
    /// node `A`, then complete it, and then after parsing the
    /// whole `A`, decide that it should have started some node
    /// `B` before starting `A`. `precede` allows to do exactly
    /// that. See also docs about
    /// [`Event::Start::forward_parent`](crate::event::Event::Start::forward_parent).
    ///
    /// Given completed events `[START, FINISH]` and its corresponding
    /// `CompletedMarker(pos: 0, _)`.
    /// Append a new `START` events as `[START, FINISH, NEWSTART]`,
    /// then mark `NEWSTART` as `START`'s parent with saving its relative
    /// distance to `NEWSTART` into `forward_parent`(=2 in this case);
    pub(crate) fn precede(self, p: &mut Parser<'_>) -> Marker {
        let new_pos = p.start();
        let idx = self.pos as usize;
        match &mut p.events[idx] {
            Event::Start { forward_parent, .. } => {
                *forward_parent = Some(new_pos.pos - self.pos);
            }
            _ => unreachable!(),
        }
        new_pos
    }

    /// Extends this completed marker *to the left* up to `m`.
    pub(crate) fn extend_to(self, p: &mut Parser<'_>, mut m: Marker) -> CompletedMarker {
        m.bomb.defuse();
        let idx = m.pos as usize;
        match &mut p.events[idx] {
            Event::Start { forward_parent, .. } => {
                *forward_parent = Some(self.pos - m.pos);
            }
            _ => unreachable!(),
        }
        self
    }

    pub(crate) fn kind(&self) -> SyntaxKind {
        self.kind
    }
}

pub fn parse(input: &Input) -> Output {
    let mut p = Parser::new(input);
    // 2. lex tokens to event vec via parser aka actually run the parser code,
    // it calls the methods on the parser to create a vector of events
    grammar::entry_point(&mut p);
    let events = p.finish();
    // 3. forward parents
    event::process(events)
}

pub(crate) struct Parser<'t> {
    inp: &'t Input,
    pos: usize,
    events: Vec<Event>,
    steps: Cell<u32>,
}

const PARSER_STEP_LIMIT: usize = 15_000_000;

enum TrivaBetween {
    NotAllowed,
    Allowed,
}

impl<'t> Parser<'t> {
    fn new(inp: &'t Input) -> Parser<'t> {
        Parser {
            inp,
            pos: 0,
            events: vec![],
            steps: Cell::new(0),
        }
    }

    /// Consume the next token if `kind` matches.
    pub(crate) fn eat(&mut self, kind: SyntaxKind) -> bool {
        if !self.at(kind) {
            return false;
        }
        let n_raw_tokens = match kind {
            SyntaxKind::COLON2
            | SyntaxKind::COLONEQ
            | SyntaxKind::NEQ
            | SyntaxKind::NEQB
            | SyntaxKind::LTEQ
            | SyntaxKind::FAT_ARROW
            | SyntaxKind::GTEQ => 2,
            SyntaxKind::SIMILAR_TO => {
                let m = self.start();
                self.bump(SyntaxKind::SIMILAR_KW);
                self.bump(SyntaxKind::TO_KW);
                m.complete(self, SyntaxKind::SIMILAR_TO);
                return true;
            }
            SyntaxKind::AT_TIME_ZONE => {
                let m = self.start();
                self.bump(SyntaxKind::AT_KW);
                self.bump(SyntaxKind::TIME_KW);
                self.bump(SyntaxKind::ZONE_KW);
                m.complete(self, SyntaxKind::AT_TIME_ZONE);
                return true;
            }
            SyntaxKind::IS_NOT_DISTINCT_FROM => {
                let m = self.start();
                self.bump(SyntaxKind::IS_KW);
                self.bump(SyntaxKind::NOT_KW);
                self.bump(SyntaxKind::DISTINCT_KW);
                self.bump(SyntaxKind::FROM_KW);
                m.complete(self, SyntaxKind::IS_NOT_DISTINCT_FROM);
                return true;
            }
            SyntaxKind::OPERATOR_CALL => {
                let m = self.start();
                self.bump(SyntaxKind::OPERATOR_KW);
                self.bump(SyntaxKind::L_PAREN);

                // database.
                if self.eat(SyntaxKind::IDENT) {
                    self.expect(SyntaxKind::DOT);
                }
                // schema.
                if self.eat(SyntaxKind::IDENT) {
                    self.expect(SyntaxKind::DOT);
                }

                // +, -, etc.
                match grammar::current_operator(self) {
                    Some(kind) => {
                        self.bump(kind);
                    }
                    None => {
                        self.error("expected operator");
                    }
                }

                self.expect(SyntaxKind::R_PAREN);
                m.complete(self, SyntaxKind::OPERATOR_CALL);
                return true;
            }
            SyntaxKind::IS_DISTINCT_FROM => {
                let m = self.start();
                self.bump(SyntaxKind::IS_KW);
                self.bump(SyntaxKind::DISTINCT_KW);
                self.bump(SyntaxKind::FROM_KW);
                m.complete(self, SyntaxKind::IS_DISTINCT_FROM);
                return true;
            }
            SyntaxKind::NOT_LIKE => {
                let m = self.start();
                self.bump(SyntaxKind::NOT_KW);
                self.bump(SyntaxKind::LIKE_KW);
                m.complete(self, SyntaxKind::NOT_LIKE);
                return true;
            }
            SyntaxKind::NOT_IN => {
                let m = self.start();
                self.bump(SyntaxKind::NOT_KW);
                self.bump(SyntaxKind::IN_KW);
                m.complete(self, SyntaxKind::NOT_IN);
                return true;
            }
            SyntaxKind::IS_NOT => {
                let m = self.start();
                self.bump(SyntaxKind::IS_KW);
                self.bump(SyntaxKind::NOT_KW);
                m.complete(self, SyntaxKind::IS_NOT);
                return true;
            }
            SyntaxKind::CUSTOM_OP => {
                let m = self.start();
                while !self.at(SyntaxKind::EOF) {
                    let is_joint = self.inp.is_joint(self.pos);
                    if self.at_ts(OPERATOR_FIRST) {
                        self.bump_any();
                    } else {
                        break;
                    }
                    if !is_joint {
                        break;
                    }
                }
                m.complete(self, SyntaxKind::CUSTOM_OP);
                return true;
            }
            _ => 1,
        };
        self.do_bump(kind, n_raw_tokens);
        true
    }

    fn at_composite2(&self, n: usize, k1: SyntaxKind, k2: SyntaxKind, triva: TrivaBetween) -> bool {
        let tokens_match =
            self.inp.kind(self.pos + n) == k1 && self.inp.kind(self.pos + n + 1) == k2;
        // We need to do this so we can say that:
        // 1 > > 2, is not the same as 1 >> 2
        match triva {
            TrivaBetween::Allowed => tokens_match,
            TrivaBetween::NotAllowed => {
                return tokens_match
                    && self.inp.is_joint(self.pos + n)
                    && self.next_not_joined_op(n + 1);
            }
        }
    }

    fn at_composite3(&self, n: usize, k1: SyntaxKind, k2: SyntaxKind, k3: SyntaxKind) -> bool {
        self.inp.kind(self.pos + n) == k1
            && self.inp.kind(self.pos + n + 1) == k2
            && self.inp.kind(self.pos + n + 2) == k3
    }

    fn at_composite4(
        &self,
        n: usize,
        k1: SyntaxKind,
        k2: SyntaxKind,
        k3: SyntaxKind,
        k4: SyntaxKind,
    ) -> bool {
        self.inp.kind(self.pos + n) == k1
            && self.inp.kind(self.pos + n + 1) == k2
            && self.inp.kind(self.pos + n + 2) == k3
            && self.inp.kind(self.pos + n + 3) == k4
    }

    fn next_not_joined_op(&self, n: usize) -> bool {
        let next = self.inp.kind(self.pos + n + 1);
        // next isn't an operator so we know we're not joined to it
        if !OPERATOR_FIRST.contains(next) {
            return true;
        }
        // current kind isn't joined
        if !self.inp.is_joint(self.pos + n) {
            return true;
        }
        false
    }

    /// Checks if the current token is in `kinds`.
    pub(crate) fn at_ts(&self, kinds: TokenSet) -> bool {
        kinds.contains(self.current())
    }

    /// Starts a new node in the syntax tree. All nodes and tokens
    /// consumed between the `start` and the corresponding `Marker::complete`
    /// belong to the same node.
    pub(crate) fn start(&mut self) -> Marker {
        let pos = self.events.len() as u32;
        self.push_event(Event::tombstone());
        Marker::new(pos)
    }

    /// Consume the next token. Panics if the parser isn't currently at `kind`.
    pub(crate) fn bump(&mut self, kind: SyntaxKind) {
        assert!(self.eat(kind));
    }

    /// Advances the parser by one token
    pub(crate) fn bump_any(&mut self) {
        let kind = self.nth(0);
        if kind == SyntaxKind::EOF {
            return;
        }
        self.do_bump(kind, 1);
    }

    /// Advances the parser by one token
    pub(crate) fn split_float(&mut self, mut marker: Marker) -> (bool, Marker) {
        assert!(self.at(SyntaxKind::FLOAT_NUMBER));
        // we have parse `<something>.`
        // `<something>`.0.1
        // here we need to insert an extra event
        //
        // `<something>`. 0. 1;
        // here we need to change the follow up parse, the return value will cause us to emulate a dot
        // the actual splitting happens later
        let ends_in_dot = !self.inp.is_joint(self.pos);
        if !ends_in_dot {
            let new_marker = self.start();
            let idx = marker.pos as usize;
            match &mut self.events[idx] {
                Event::Start {
                    forward_parent,
                    kind,
                } => {
                    *kind = SyntaxKind::FIELD_EXPR;
                    *forward_parent = Some(new_marker.pos - marker.pos);
                }
                _ => unreachable!(),
            }
            marker.bomb.defuse();
            marker = new_marker;
        };
        self.pos += 1;
        self.push_event(Event::FloatSplitHack { ends_in_dot });
        (ends_in_dot, marker)
    }

    /// Consume the next token if it is `kind` or emit an error
    /// otherwise.
    pub(crate) fn expect(&mut self, kind: SyntaxKind) -> bool {
        if self.eat(kind) {
            return true;
        }
        self.error(format!("expected {kind:?}"));
        false
    }

    /// Create an error node and consume the next token.
    pub(crate) fn err_and_bump(&mut self, message: &str) {
        self.err_recover(message, TokenSet::EMPTY);
    }

    /// Create an error node and consume the next token.
    pub(crate) fn err_recover(&mut self, message: &str, recovery: TokenSet) {
        // TODO: maybe we actually want this?
        // if matches!(self.current(), SyntaxKind::L_PAREN | SyntaxKind::R_PAREN) {
        //     self.error(message);
        //     return;
        // }

        if self.at_ts(recovery) {
            self.error(message);
            return;
        }

        let m = self.start();
        self.error(message);
        self.bump_any();
        m.complete(self, SyntaxKind::ERROR);
    }

    fn do_bump(&mut self, kind: SyntaxKind, n_raw_tokens: u8) {
        self.pos += n_raw_tokens as usize;
        self.steps.set(0);
        self.push_event(Event::Token { kind, n_raw_tokens });
    }

    fn push_event(&mut self, event: Event) {
        self.events.push(event);
    }

    fn finish(self) -> Vec<Event> {
        self.events
    }

    /// Emit error with the `message`
    /// FIXME: this should be much more fancy and support
    /// structured errors with spans and notes, like rustc
    /// does.
    pub(crate) fn error<T: Into<String>>(&mut self, message: T) {
        let msg = message.into();
        self.push_event(Event::Error { msg });
    }

    /// Checks if the current token is `kind`.
    #[must_use]
    pub(crate) fn at(&self, kind: SyntaxKind) -> bool {
        self.nth_at(0, kind)
    }

    /// Checks if the nth token is in `kinds`.
    #[must_use]
    pub(crate) fn nth_at_ts(&self, n: usize, kinds: TokenSet) -> bool {
        kinds.contains(self.nth(n))
    }

    #[must_use]
    pub(crate) fn nth_at(&self, n: usize, kind: SyntaxKind) -> bool {
        match kind {
            // =>
            SyntaxKind::FAT_ARROW => self.at_composite2(
                n,
                SyntaxKind::EQ,
                SyntaxKind::R_ANGLE,
                TrivaBetween::NotAllowed,
            ),
            // :=
            SyntaxKind::COLONEQ => self.at_composite2(
                n,
                SyntaxKind::COLON,
                SyntaxKind::EQ,
                TrivaBetween::NotAllowed,
            ),
            // ::
            SyntaxKind::COLON2 => self.at_composite2(
                n,
                SyntaxKind::COLON,
                SyntaxKind::COLON,
                TrivaBetween::NotAllowed,
            ),
            // !=
            SyntaxKind::NEQ => self.at_composite2(
                n,
                SyntaxKind::BANG,
                SyntaxKind::EQ,
                TrivaBetween::NotAllowed,
            ),
            // <>
            SyntaxKind::NEQB => self.at_composite2(
                n,
                SyntaxKind::L_ANGLE,
                SyntaxKind::R_ANGLE,
                TrivaBetween::NotAllowed,
            ),
            // is not
            SyntaxKind::IS_NOT => self.at_composite2(
                n,
                SyntaxKind::IS_KW,
                SyntaxKind::NOT_KW,
                TrivaBetween::Allowed,
            ),
            // is null
            SyntaxKind::IS_NULL => self.at_composite2(
                n,
                SyntaxKind::IS_KW,
                SyntaxKind::NULL_KW,
                TrivaBetween::Allowed,
            ),
            // not like
            SyntaxKind::NOT_LIKE => self.at_composite2(
                n,
                SyntaxKind::NOT_KW,
                SyntaxKind::LIKE_KW,
                TrivaBetween::Allowed,
            ),
            // not in
            SyntaxKind::NOT_IN => self.at_composite2(
                n,
                SyntaxKind::NOT_KW,
                SyntaxKind::IN_KW,
                TrivaBetween::Allowed,
            ),
            // at time zone
            SyntaxKind::AT_TIME_ZONE => self.at_composite3(
                n,
                SyntaxKind::AT_KW,
                SyntaxKind::TIME_KW,
                SyntaxKind::ZONE_KW,
            ),
            // is distinct from
            SyntaxKind::IS_DISTINCT_FROM => self.at_composite3(
                n,
                SyntaxKind::IS_KW,
                SyntaxKind::DISTINCT_KW,
                SyntaxKind::FROM_KW,
            ),
            // is not distinct from
            SyntaxKind::IS_NOT_DISTINCT_FROM => self.at_composite4(
                n,
                SyntaxKind::IS_KW,
                SyntaxKind::NOT_KW,
                SyntaxKind::DISTINCT_KW,
                SyntaxKind::FROM_KW,
            ),
            // similar to
            SyntaxKind::SIMILAR_TO => self.at_composite2(
                n,
                SyntaxKind::SIMILAR_KW,
                SyntaxKind::TO_KW,
                TrivaBetween::Allowed,
            ),
            // https://www.postgresql.org/docs/17/sql-expressions.html#SQL-EXPRESSIONS-OPERATOR-CALLS
            // TODO: is this right?
            SyntaxKind::OPERATOR_CALL => self.at_composite2(
                n,
                SyntaxKind::OPERATOR_KW,
                SyntaxKind::L_PAREN,
                TrivaBetween::Allowed,
            ),
            // <=
            SyntaxKind::LTEQ => self.at_composite2(
                n,
                SyntaxKind::L_ANGLE,
                SyntaxKind::EQ,
                TrivaBetween::NotAllowed,
            ),
            // <=
            SyntaxKind::GTEQ => self.at_composite2(
                n,
                SyntaxKind::R_ANGLE,
                SyntaxKind::EQ,
                TrivaBetween::NotAllowed,
            ),
            SyntaxKind::CUSTOM_OP => {
                // TODO: is this right?
                if self.at_ts(OPERATOR_FIRST) {
                    return true;
                }
                return false;
            }
            // TODO: we probably shouldn't be using a _ for this but be explicit for each type?
            _ => self.inp.kind(self.pos + n) == kind,
        }
    }

    /// Returns the kind of the current token.
    /// If parser has already reached the end of input,
    /// the special `EOF` kind is returned.
    #[must_use]
    pub(crate) fn current(&self) -> SyntaxKind {
        self.nth(0)
    }

    /// Lookahead operation: returns the kind of the next nth
    /// token.
    #[must_use]
    fn nth(&self, n: usize) -> SyntaxKind {
        assert!(n <= 3);

        let steps = self.steps.get();
        assert!(
            (steps as usize) < PARSER_STEP_LIMIT,
            "the parser seems stuck"
        );
        self.steps.set(steps + 1);

        self.inp.kind(self.pos + n)
    }
}
