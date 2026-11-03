// via https://github.com/rust-lang/rust-analyzer/blob/d8887c0758bbd2d5f752d5bd405d4491e90e7ed6/crates/syntax/src/ast.rs
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

use squawk_rowan::Language;

use super::{AstChildren, AstNode, SyntaxKind, SyntaxNode, SyntaxToken};
use crate::SyntaxElement;
use crate::syntax_node::Sql;

#[inline]
pub(super) fn child<N: AstNode>(parent: &SyntaxNode) -> Option<N> {
    first_child_or_token(parent, N::can_cast)?
        .into_node()
        .and_then(N::cast)
}

#[inline]
pub(super) fn children<N: AstNode>(parent: &SyntaxNode) -> AstChildren<N> {
    AstChildren::new(parent)
}

#[inline]
pub(super) fn token(parent: &SyntaxNode, kind: SyntaxKind) -> Option<SyntaxToken> {
    parent.child_token_of_kind(kind)
}

// avoid allocation compared to:
//    parent
//        .children_with_tokens()
//        .filter_map(|it| it.into_token())
//        .find(|it| it.kind() == kind)
fn first_child_or_token(
    parent: &SyntaxNode,
    kind_matches: impl Fn(SyntaxKind) -> bool,
) -> Option<SyntaxElement> {
    let index = parent.green().children().position(|child| {
        child.as_node().is_some() && kind_matches(Sql::kind_from_raw(child.kind()))
    })?;
    parent.child_or_token(u32::try_from(index).unwrap())
}
