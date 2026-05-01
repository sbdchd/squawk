// via https://github.com/rust-lang/rust-analyzer/blob/2efc80078029894eec0699f62ec8d5c1a56af763/crates/syntax/src/ptr.rs#L22C19-L22C19
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

//! In squawk, syntax trees are transient objects.
//!
//! That means that we create trees when we need them, and tear them down to
//! save memory. In this architecture, hanging on to a particular syntax node
//! for a long time is ill-advisable, as that keeps the whole tree resident.
//!
//! Instead, we provide a [`SyntaxNodePtr`] type, which stores information about
//! *location* of a particular syntax node in a tree. Its a small type which can
//! be cheaply stored, and which can be resolved to a real [`SyntaxNode`] when
//! necessary.

use std::{
    hash::{Hash, Hasher},
    marker::PhantomData,
};

use rowan::TextRange;

use crate::{AstNode, SyntaxNode, syntax_node::Sql};

/// A "pointer" to a [`SyntaxNode`], via location in the source code.
pub type SyntaxNodePtr = rowan::ast::SyntaxNodePtr<Sql>;

/// Like [`SyntaxNodePtr`], but remembers the type of node.
pub struct AstPtr<N: AstNode> {
    raw: SyntaxNodePtr,
    _ty: PhantomData<fn() -> N>,
}

impl<N: AstNode> std::fmt::Debug for AstPtr<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("AstPtr").field(&self.raw).finish()
    }
}

impl<N: AstNode> Copy for AstPtr<N> {}
impl<N: AstNode> Clone for AstPtr<N> {
    fn clone(&self) -> AstPtr<N> {
        *self
    }
}

impl<N: AstNode> Eq for AstPtr<N> {}

impl<N: AstNode> PartialEq for AstPtr<N> {
    fn eq(&self, other: &AstPtr<N>) -> bool {
        self.raw == other.raw
    }
}

impl<N: AstNode> Hash for AstPtr<N> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.raw.hash(state);
    }
}

impl<N: AstNode> AstPtr<N> {
    pub fn new(node: &N) -> AstPtr<N> {
        AstPtr {
            raw: SyntaxNodePtr::new(node.syntax()),
            _ty: PhantomData,
        }
    }

    pub fn to_node(&self, root: &SyntaxNode) -> N {
        let syntax_node = self.raw.to_node(root);
        N::cast(syntax_node).unwrap()
    }

    pub fn syntax_node_ptr(&self) -> SyntaxNodePtr {
        self.raw
    }

    pub fn text_range(&self) -> TextRange {
        self.raw.text_range()
    }

    pub fn cast<U: AstNode>(self) -> Option<AstPtr<U>> {
        if !U::can_cast(self.raw.kind()) {
            return None;
        }
        Some(AstPtr {
            raw: self.raw,
            _ty: PhantomData,
        })
    }

    pub fn kind(&self) -> squawk_parser::SyntaxKind {
        self.raw.kind()
    }

    pub fn upcast<M: AstNode>(self) -> AstPtr<M>
    where
        N: Into<M>,
    {
        AstPtr {
            raw: self.raw,
            _ty: PhantomData,
        }
    }

    /// Like `SyntaxNodePtr::cast` but the trait bounds work out.
    pub fn try_from_raw(raw: SyntaxNodePtr) -> Option<AstPtr<N>> {
        N::can_cast(raw.kind()).then_some(AstPtr {
            raw,
            _ty: PhantomData,
        })
    }

    pub fn wrap_left<R>(self) -> AstPtr<either::Either<N, R>>
    where
        either::Either<N, R>: AstNode,
    {
        AstPtr {
            raw: self.raw,
            _ty: PhantomData,
        }
    }

    pub fn wrap_right<L>(self) -> AstPtr<either::Either<L, N>>
    where
        either::Either<L, N>: AstNode,
    {
        AstPtr {
            raw: self.raw,
            _ty: PhantomData,
        }
    }
}

impl<N: AstNode> From<AstPtr<N>> for SyntaxNodePtr {
    fn from(ptr: AstPtr<N>) -> SyntaxNodePtr {
        ptr.raw
    }
}

#[test]
fn test_local_syntax_ptr() {
    use crate::{AstNode, ast};

    let file = ast::SourceFile::parse("create table t(c int8);")
        .ok()
        .unwrap();
    let field = file
        .syntax()
        .descendants()
        .find_map(ast::Column::cast)
        .unwrap();
    let ptr = SyntaxNodePtr::new(field.syntax());
    let field_syntax = ptr.to_node(file.syntax());
    assert_eq!(field.syntax(), &field_syntax);
}
