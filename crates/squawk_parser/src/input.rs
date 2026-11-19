// via https://github.com/rust-lang/rust-analyzer/blob/d8887c0758bbd2d5f752d5bd405d4491e90e7ed6/crates/parser/src/input.rs
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

//! See [`Input`].

use crate::SyntaxKind;

const JOINT_MASK: u16 = 1 << 15;
const _: () = assert!((SyntaxKind::__LAST as u16) < JOINT_MASK);

struct InputToken {
    kind: SyntaxKind,
    contextual_kind_and_joint: u16,
}

const _: () = assert!(std::mem::size_of::<InputToken>() == 4);

/// Input for the parser -- a sequence of tokens.
///
/// As of now, parser doesn't have access to the *text* of the tokens, and makes
/// decisions based solely on their classification. Unlike `LexerToken`, the
/// `Tokens` doesn't include whitespace and comments. Main input to the parser.
#[derive(Default)]
pub struct Input {
    tokens: Vec<InputToken>,
}

/// `pub` impl used by callers to create `Tokens`.
impl Input {
    pub(crate) fn with_capacity(capacity: usize) -> Self {
        Input {
            tokens: Vec::with_capacity(capacity),
        }
    }

    #[inline]
    pub(crate) fn push(&mut self, kind: SyntaxKind) {
        self.push_impl(kind, SyntaxKind::EOF);
    }
    #[inline]
    pub(crate) fn push_ident(&mut self, contextual_kind: SyntaxKind) {
        self.push_impl(SyntaxKind::IDENT, contextual_kind);
    }
    /// Sets jointness for the last token we've pushed.
    ///
    /// This is a separate API rather than an argument to the `push` to make it
    /// convenient both for textual and mbe tokens. With text, you know whether
    /// the *previous* token was joint, with mbe, you know whether the *current*
    /// one is joint. This API allows for styles of usage:
    ///
    /// ```
    /// // In text:
    /// tokens.was_joint(prev_joint);
    /// tokens.push(curr);
    ///
    /// // In MBE:
    /// token.push(curr);
    /// tokens.push(curr_joint)
    /// ```
    #[inline]
    pub(crate) fn was_joint(&mut self) {
        self.tokens.last_mut().unwrap().contextual_kind_and_joint |= JOINT_MASK;
    }
    #[inline]
    fn push_impl(&mut self, kind: SyntaxKind, contextual_kind: SyntaxKind) {
        self.tokens.push(InputToken {
            kind,
            contextual_kind_and_joint: contextual_kind as u16,
        });
    }
}

#[derive(Clone, Copy)]
pub(crate) struct InputView<'a> {
    tokens: &'a [InputToken],
}

impl InputView<'_> {
    pub(crate) fn limited(self, end: usize) -> Self {
        Self {
            tokens: &self.tokens[..end.min(self.tokens.len())],
        }
    }

    pub(crate) fn len(&self) -> usize {
        self.tokens.len()
    }

    pub(crate) fn kind(&self, idx: usize) -> SyntaxKind {
        self.tokens
            .get(idx)
            .map_or(SyntaxKind::EOF, |token| token.kind)
    }
    /// For an `IDENT`, the PL/pgSQL keyword it spells, or `IDENT` if it spells
    /// none. `EOF` for every other token.
    pub(crate) fn contextual_kind(&self, idx: usize) -> SyntaxKind {
        self.tokens.get(idx).map_or(SyntaxKind::EOF, |token| {
            (token.contextual_kind_and_joint & !JOINT_MASK).into()
        })
    }
    pub(crate) fn is_joint(&self, n: usize) -> bool {
        self.tokens[n].contextual_kind_and_joint & JOINT_MASK != 0
    }
}

impl Input {
    pub(crate) fn view(&self) -> InputView<'_> {
        InputView {
            tokens: &self.tokens,
        }
    }

    pub(crate) fn len(&self) -> usize {
        self.tokens.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_metadata_stays_independent() {
        let mut input = Input::with_capacity(0);
        assert_eq!(input.view().kind(0), SyntaxKind::EOF);
        assert_eq!(input.view().contextual_kind(0), SyntaxKind::EOF);
        for i in 0..130 {
            if i % 2 == 0 {
                input.push_ident(SyntaxKind::LOOP_KW);
            } else {
                input.push(SyntaxKind::PLUS);
            }
            if i % 3 == 0 {
                input.was_joint();
            }
        }
        let input = input.view();
        assert_eq!(input.len(), 130);
        for i in 0..input.len() {
            let (kind, contextual_kind) = if i % 2 == 0 {
                (SyntaxKind::IDENT, SyntaxKind::LOOP_KW)
            } else {
                (SyntaxKind::PLUS, SyntaxKind::EOF)
            };
            assert_eq!(input.kind(i), kind);
            assert_eq!(input.contextual_kind(i), contextual_kind);
            assert_eq!(input.is_joint(i), i % 3 == 0);
        }
        assert_eq!(input.kind(input.len()), SyntaxKind::EOF);
        assert_eq!(input.contextual_kind(input.len()), SyntaxKind::EOF);
        assert_eq!(input.kind(usize::MAX), SyntaxKind::EOF);
        assert_eq!(input.contextual_kind(usize::MAX), SyntaxKind::EOF);
    }
}
