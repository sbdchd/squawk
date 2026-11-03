//! Working with abstract syntax trees.
//!
//! In rowan, syntax trees are transient objects. That means that we create
//! trees when we need them, and tear them down to save memory. In this
//! architecture, hanging on to a particular syntax node for a long time is
//! ill-advisable, as that keeps the whole tree resident.
//!
//! Instead, we provide a [`SyntaxNodePtr`] type, which stores information about
//! the _location_ of a particular syntax node in a tree. It's a small type
//! which can be cheaply stored, and which can be resolved to a real
//! [`SyntaxNode`] when necessary.
//!
//! We also provide an [`AstNode`] trait for typed AST wrapper APIs over rowan
//! nodes.

use std::{
    fmt,
    hash::{Hash, Hasher},
    iter::successors,
    marker::PhantomData,
};

use crate::{Language, SyntaxNode, SyntaxNodeChildren, TextRange};

/// The main trait to go from untyped [`SyntaxNode`] to a typed AST. The
/// conversion itself has zero runtime cost: AST and syntax nodes have exactly
/// the same representation: a pointer to the tree root and a pointer to the
/// node itself.
pub trait AstNode {
    type Language: Language;

    fn can_cast(kind: <Self::Language as Language>::Kind) -> bool
    where
        Self: Sized;

    fn cast(node: SyntaxNode<Self::Language>) -> Option<Self>
    where
        Self: Sized;

    fn syntax(&self) -> &SyntaxNode<Self::Language>;

    fn clone_subtree(&self) -> Self
    where
        Self: Sized,
    {
        Self::cast(self.syntax().clone_subtree()).unwrap()
    }
}

/// A "pointer" to a [`SyntaxNode`], via location in the source code.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct SyntaxNodePtr<L: Language> {
    kind: L::Kind,
    range: TextRange,
}

impl<L: Language> SyntaxNodePtr<L> {
    /// Returns a [`SyntaxNodePtr`] for the node.
    pub fn new(node: &SyntaxNode<L>) -> Self {
        Self {
            kind: node.kind(),
            range: node.text_range(),
        }
    }

    /// Like [`Self::try_to_node`] but panics instead of returning `None` on
    /// failure.
    pub fn to_node(&self, root: &SyntaxNode<L>) -> SyntaxNode<L> {
        self.try_to_node(root)
            .unwrap_or_else(|| panic!("can't resolve {self:?} with {root:?}"))
    }

    /// "Dereferences" the pointer to get the [`SyntaxNode`] it points to.
    ///
    /// Returns `None` if the node is not found, so make sure that the `root`
    /// syntax tree is equivalent to (i.e. is build from the same text from) the
    /// tree which was originally used to get this [`SyntaxNodePtr`].
    ///
    /// Also returns `None` if `root` is not actually a root (i.e. it has a
    /// parent).
    ///
    /// The complexity is linear in the depth of the tree and logarithmic in
    /// tree width. As most trees are shallow, thinking about this as
    /// `O(log(N))` in the size of the tree is not too wrong!
    pub fn try_to_node(&self, root: &SyntaxNode<L>) -> Option<SyntaxNode<L>> {
        if root.parent().is_some() {
            return None;
        }
        successors(Some(root.clone()), |node| {
            node.child_or_token_at_range(self.range)?.into_node()
        })
        .find(|it| it.text_range() == self.range && it.kind() == self.kind)
    }

    /// Casts this to an [`AstPtr`] to the given node type if possible.
    pub fn cast<N: AstNode<Language = L>>(self) -> Option<AstPtr<N>> {
        if !N::can_cast(self.kind) {
            return None;
        }
        Some(AstPtr { raw: self })
    }

    /// Returns the kind of the syntax node this points to.
    pub fn kind(&self) -> L::Kind {
        self.kind
    }

    /// Returns the range of the syntax node this points to.
    pub fn text_range(&self) -> TextRange {
        self.range
    }
}

/// Like [`SyntaxNodePtr`], but remembers the type of node.
pub struct AstPtr<N: AstNode> {
    raw: SyntaxNodePtr<N::Language>,
}

impl<N: AstNode> AstPtr<N> {
    /// Returns an [`AstPtr`] for the node.
    pub fn new(node: &N) -> Self {
        Self {
            raw: SyntaxNodePtr::new(node.syntax()),
        }
    }

    /// Like `Self::try_to_node` but panics on failure.
    pub fn to_node(&self, root: &SyntaxNode<N::Language>) -> N {
        self.try_to_node(root)
            .unwrap_or_else(|| panic!("can't resolve {self:?} with {root:?}"))
    }

    /// Given the root node containing the node `n` that `self` is a pointer to,
    /// returns `n` if possible. See [`SyntaxNodePtr::try_to_node`].
    pub fn try_to_node(&self, root: &SyntaxNode<N::Language>) -> Option<N> {
        N::cast(self.raw.try_to_node(root)?)
    }

    /// Returns the underlying [`SyntaxNodePtr`].
    pub fn syntax_node_ptr(&self) -> SyntaxNodePtr<N::Language> {
        self.raw
    }

    /// Casts this to an [`AstPtr`] to the given node type if possible.
    pub fn cast<U: AstNode<Language = N::Language>>(self) -> Option<AstPtr<U>> {
        if !U::can_cast(self.raw.kind) {
            return None;
        }
        Some(AstPtr { raw: self.raw })
    }
}

impl<N: AstNode> fmt::Debug for AstPtr<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AstPtr").field("raw", &self.raw).finish()
    }
}

impl<N: AstNode> Clone for AstPtr<N> {
    fn clone(&self) -> Self {
        Self { raw: self.raw }
    }
}

impl<N: AstNode> PartialEq for AstPtr<N> {
    fn eq(&self, other: &AstPtr<N>) -> bool {
        self.raw == other.raw
    }
}

impl<N: AstNode> Eq for AstPtr<N> {}

impl<N: AstNode> Hash for AstPtr<N> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.raw.hash(state)
    }
}

impl<N: AstNode> From<AstPtr<N>> for SyntaxNodePtr<N::Language> {
    fn from(ptr: AstPtr<N>) -> SyntaxNodePtr<N::Language> {
        ptr.raw
    }
}

#[derive(Debug, Clone)]
pub struct AstChildren<N: AstNode> {
    inner: SyntaxNodeChildren<N::Language>,
    ph: PhantomData<N>,
}

impl<N: AstNode> AstChildren<N> {
    fn new(parent: &SyntaxNode<N::Language>) -> Self {
        AstChildren {
            inner: parent.children(),
            ph: PhantomData,
        }
    }
}

impl<N: AstNode> Iterator for AstChildren<N> {
    type Item = N;
    fn next(&mut self) -> Option<N> {
        self.inner.find_map(N::cast)
    }
}

pub mod support {
    use super::{AstChildren, AstNode};
    use crate::{Language, SyntaxNode, SyntaxToken};

    pub fn child<N: AstNode>(parent: &SyntaxNode<N::Language>) -> Option<N> {
        parent.children().find_map(N::cast)
    }

    pub fn children<N: AstNode>(parent: &SyntaxNode<N::Language>) -> AstChildren<N> {
        AstChildren::new(parent)
    }

    pub fn token<L: Language>(parent: &SyntaxNode<L>, kind: L::Kind) -> Option<SyntaxToken<L>> {
        parent
            .children_with_tokens()
            .filter_map(|it| it.into_token())
            .find(|it| it.kind() == kind)
    }
}
