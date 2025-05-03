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

#[allow(non_camel_case_types)]
type bits = u64;

/// Input for the parser -- a sequence of tokens.
///
/// As of now, parser doesn't have access to the *text* of the tokens, and makes
/// decisions based solely on their classification. Unlike `LexerToken`, the
/// `Tokens` doesn't include whitespace and comments. Main input to the parser.
///
/// Struct of arrays internally, but this shouldn't really matter.
#[derive(Default)]
pub struct Input {
    kind: Vec<SyntaxKind>,
    joint: Vec<bits>,
    // TODO: I think we can remove this
    contextual_kind: Vec<SyntaxKind>,
}

/// `pub` impl used by callers to create `Tokens`.
impl Input {
    #[inline]
    pub(crate) fn push(&mut self, kind: SyntaxKind) {
        self.push_impl(kind, SyntaxKind::EOF)
    }
    // #[inline]
    // pub(crate) fn push_ident(&mut self, contextual_kind: SyntaxKind) {
    //     self.push_impl(SyntaxKind::IDENT, contextual_kind)
    // }
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
        let n = self.len() - 1;
        let (idx, b_idx) = self.bit_index(n);
        self.joint[idx] |= 1 << b_idx;
    }
    #[inline]
    fn push_impl(&mut self, kind: SyntaxKind, contextual_kind: SyntaxKind) {
        let idx = self.len();
        if idx % (bits::BITS as usize) == 0 {
            self.joint.push(0);
        }
        self.kind.push(kind);
        self.contextual_kind.push(contextual_kind);
    }
}

/// pub(crate) impl used by the parser to consume `Tokens`.
impl Input {
    pub(crate) fn kind(&self, idx: usize) -> SyntaxKind {
        self.kind.get(idx).copied().unwrap_or(SyntaxKind::EOF)
    }
    // TODO: we may want to use this in the parser since we have a lot of
    // "keywords" that are actually contextual.
    // pub(crate) fn contextual_kind(&self, idx: usize) -> SyntaxKind {
    //     self.contextual_kind
    //         .get(idx)
    //         .copied()
    //         .unwrap_or(SyntaxKind::EOF)
    // }
    pub(crate) fn is_joint(&self, n: usize) -> bool {
        let (idx, b_idx) = self.bit_index(n);
        self.joint[idx] & 1 << b_idx != 0
    }
}

impl Input {
    fn bit_index(&self, n: usize) -> (usize, usize) {
        let idx = n / (bits::BITS as usize);
        let b_idx = n % (bits::BITS as usize);
        (idx, b_idx)
    }
    fn len(&self) -> usize {
        self.kind.len()
    }
}
