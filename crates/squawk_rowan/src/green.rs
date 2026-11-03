use std::{fmt, hash::Hash, iter::FusedIterator, ops::Range, slice, sync::Arc};

use crate::{NodeOrToken, TextRange, TextSize};

/// SyntaxKind is a type tag for each token or node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SyntaxKind(pub u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct NodeId(pub(crate) u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct TokenId(pub(crate) u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum ElementId {
    Node(NodeId),
    Token(TokenId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct PackedElementId(u32);

impl PackedElementId {
    fn node(id: NodeId) -> Self {
        PackedElementId(id.0 << 1)
    }

    fn token(id: TokenId) -> Self {
        PackedElementId((id.0 << 1) | 1)
    }

    #[inline]
    fn unpack(self) -> ElementId {
        if self.0 & 1 == 0 {
            ElementId::Node(NodeId(self.0 >> 1))
        } else {
            ElementId::Token(TokenId(self.0 >> 1))
        }
    }
}

const NO_PARENT: NodeId = NodeId(u32::MAX);

const _: () = assert!(std::mem::size_of::<NodeData>() == 32);
const _: () = assert!(std::mem::size_of::<TokenData>() == 16);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct NodeData {
    kind: SyntaxKind,
    parent: NodeId,
    index_in_parent: u32,
    first_child: u32,
    child_count: u32,
    first_token: u32,
    nodes_end: u32,
    tokens_end: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct TokenData {
    kind: SyntaxKind,
    parent: NodeId,
    index_in_parent: u32,
    start: TextSize,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct GreenTree {
    source: Box<str>,
    nodes: Box<[NodeData]>,
    tokens: Box<[TokenData]>,
    children: Box<[PackedElementId]>,
}

impl GreenTree {
    #[inline]
    fn node(&self, id: NodeId) -> &NodeData {
        &self.nodes[id.0 as usize]
    }

    #[inline]
    fn token(&self, id: TokenId) -> &TokenData {
        &self.tokens[id.0 as usize]
    }

    #[inline]
    fn token_start(&self, index: u32) -> TextSize {
        self.tokens
            .get(index as usize)
            .map_or_else(|| TextSize::of(&*self.source), |token| token.start)
    }

    #[inline]
    fn node_range(&self, id: NodeId) -> TextRange {
        let node = self.node(id);
        TextRange::new(
            self.token_start(node.first_token),
            self.token_start(node.tokens_end),
        )
    }

    #[inline]
    fn token_range(&self, id: TokenId) -> TextRange {
        TextRange::new(self.token(id).start, self.token_start(id.0 + 1))
    }

    #[inline]
    fn children(&self, id: NodeId) -> &[PackedElementId] {
        let node = self.node(id);
        let start = node.first_child as usize;
        &self.children[start..start + node.child_count as usize]
    }
}

#[derive(Clone)]
pub struct GreenNode {
    tree: Arc<GreenTree>,
}

impl GreenNode {
    pub(crate) const ROOT: NodeId = NodeId(0);

    #[inline]
    pub(crate) fn node_kind(&self, id: NodeId) -> SyntaxKind {
        self.tree.node(id).kind
    }

    #[inline]
    pub(crate) fn node_range(&self, id: NodeId) -> TextRange {
        self.tree.node_range(id)
    }

    #[inline]
    pub(crate) fn node_parent(&self, id: NodeId) -> Option<NodeId> {
        let parent = self.tree.node(id).parent;
        (parent != NO_PARENT).then_some(parent)
    }

    #[inline]
    pub(crate) fn node_index(&self, id: NodeId) -> u32 {
        self.tree.node(id).index_in_parent
    }

    #[inline]
    pub(crate) fn node_descendants(&self, id: NodeId) -> Range<u32> {
        id.0..self.tree.node(id).nodes_end
    }

    #[inline]
    pub(crate) fn node_first_token(&self, id: NodeId) -> u32 {
        self.tree.node(id).first_token
    }

    #[inline]
    pub(crate) fn node_tokens(&self, id: NodeId) -> Range<u32> {
        let node = self.tree.node(id);
        node.first_token..node.tokens_end
    }

    #[inline]
    pub(crate) fn token_kind(&self, id: TokenId) -> SyntaxKind {
        self.tree.token(id).kind
    }

    #[inline]
    pub(crate) fn token_range(&self, id: TokenId) -> TextRange {
        self.tree.token_range(id)
    }

    #[inline]
    pub(crate) fn token_parent(&self, id: TokenId) -> NodeId {
        self.tree.token(id).parent
    }

    #[inline]
    pub(crate) fn token_index(&self, id: TokenId) -> u32 {
        self.tree.token(id).index_in_parent
    }

    #[inline]
    pub(crate) fn child_count(&self, id: NodeId) -> u32 {
        self.tree.node(id).child_count
    }

    #[inline]
    pub(crate) fn child(&self, id: NodeId, index: u32) -> Option<ElementId> {
        self.tree
            .children(id)
            .get(index as usize)
            .map(|it| it.unpack())
    }

    #[inline]
    pub(crate) fn element_range(&self, id: ElementId) -> TextRange {
        match id {
            ElementId::Node(id) => self.node_range(id),
            ElementId::Token(id) => self.token_range(id),
        }
    }

    pub(crate) fn child_at_range(&self, id: NodeId, range: TextRange) -> Option<ElementId> {
        let children = self.tree.children(id);
        let idx = children
            .binary_search_by(|it| TextRange::ordering(self.element_range(it.unpack()), range))
            .unwrap_or_else(|it| it.saturating_sub(1));
        children
            .get(idx)
            .map(|it| it.unpack())
            .filter(|it| self.element_range(*it).contains_range(range))
    }

    #[inline]
    pub(crate) fn text_at(&self, range: TextRange) -> &str {
        &self.tree.source[range]
    }

    #[inline]
    pub(crate) fn ptr_eq(&self, other: &GreenNode) -> bool {
        Arc::ptr_eq(&self.tree, &other.tree)
    }

    #[inline]
    pub(crate) fn addr(&self) -> usize {
        Arc::as_ptr(&self.tree) as usize
    }

    #[inline]
    pub(crate) fn node_ref(&self, id: NodeId) -> GreenNodeRef<'_> {
        GreenNodeRef {
            tree: &self.tree,
            id,
        }
    }

    #[inline]
    pub(crate) fn token_ref(&self, id: TokenId) -> GreenTokenRef<'_> {
        GreenTokenRef {
            tree: &self.tree,
            id,
        }
    }

    #[inline]
    pub fn as_node_ref(&self) -> GreenNodeRef<'_> {
        self.node_ref(Self::ROOT)
    }

    #[inline]
    pub fn kind(&self) -> SyntaxKind {
        self.as_node_ref().kind()
    }

    #[inline]
    pub fn text_len(&self) -> TextSize {
        self.as_node_ref().text_len()
    }

    #[inline]
    pub fn text(&self) -> &str {
        &self.tree.source
    }

    #[inline]
    pub fn children(&self) -> Children<'_> {
        self.as_node_ref().children()
    }
}

impl PartialEq for GreenNode {
    fn eq(&self, other: &GreenNode) -> bool {
        self.ptr_eq(other) || self.tree == other.tree
    }
}

impl Eq for GreenNode {}

impl Hash for GreenNode {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.tree.source.hash(state);
        self.tree.nodes.len().hash(state);
    }
}

impl fmt::Debug for GreenNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.as_node_ref(), f)
    }
}

impl fmt::Display for GreenNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.text())
    }
}

#[derive(Clone, Copy)]
pub struct GreenNodeRef<'a> {
    tree: &'a GreenTree,
    id: NodeId,
}

impl<'a> GreenNodeRef<'a> {
    #[inline]
    pub fn kind(&self) -> SyntaxKind {
        self.tree.node(self.id).kind
    }

    #[inline]
    pub fn text_len(&self) -> TextSize {
        self.tree.node_range(self.id).len()
    }

    #[inline]
    pub fn text(&self) -> &'a str {
        &self.tree.source[self.tree.node_range(self.id)]
    }

    #[inline]
    pub fn children(&self) -> Children<'a> {
        Children {
            tree: self.tree,
            iter: self.tree.children(self.id).iter(),
        }
    }
}

impl PartialEq for GreenNodeRef<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && std::ptr::eq(self.tree, other.tree)
    }
}

impl Eq for GreenNodeRef<'_> {}

impl fmt::Debug for GreenNodeRef<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GreenNode")
            .field("kind", &self.kind())
            .field("text_len", &self.text_len())
            .finish()
    }
}

impl fmt::Display for GreenNodeRef<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.text())
    }
}

#[derive(Clone, Copy)]
pub struct GreenTokenRef<'a> {
    tree: &'a GreenTree,
    id: TokenId,
}

impl<'a> GreenTokenRef<'a> {
    #[inline]
    pub fn kind(&self) -> SyntaxKind {
        self.tree.token(self.id).kind
    }

    #[inline]
    pub fn text(&self) -> &'a str {
        &self.tree.source[self.tree.token_range(self.id)]
    }

    #[inline]
    pub fn text_len(&self) -> TextSize {
        self.tree.token_range(self.id).len()
    }
}

impl PartialEq for GreenTokenRef<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && std::ptr::eq(self.tree, other.tree)
    }
}

impl Eq for GreenTokenRef<'_> {}

impl fmt::Debug for GreenTokenRef<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GreenToken")
            .field("kind", &self.kind())
            .field("text", &self.text())
            .finish()
    }
}

impl fmt::Display for GreenTokenRef<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.text())
    }
}

pub type GreenElementRef<'a> = NodeOrToken<GreenNodeRef<'a>, GreenTokenRef<'a>>;

impl GreenElementRef<'_> {
    #[inline]
    pub fn kind(&self) -> SyntaxKind {
        match self {
            NodeOrToken::Node(it) => it.kind(),
            NodeOrToken::Token(it) => it.kind(),
        }
    }

    #[inline]
    pub fn text_len(&self) -> TextSize {
        match self {
            NodeOrToken::Node(it) => it.text_len(),
            NodeOrToken::Token(it) => it.text_len(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Children<'a> {
    tree: &'a GreenTree,
    iter: slice::Iter<'a, PackedElementId>,
}

impl<'a> Children<'a> {
    #[inline]
    fn wrap(&self, id: PackedElementId) -> GreenElementRef<'a> {
        match id.unpack() {
            ElementId::Node(id) => NodeOrToken::Node(GreenNodeRef {
                tree: self.tree,
                id,
            }),
            ElementId::Token(id) => NodeOrToken::Token(GreenTokenRef {
                tree: self.tree,
                id,
            }),
        }
    }
}

impl<'a> Iterator for Children<'a> {
    type Item = GreenElementRef<'a>;

    #[inline]
    fn next(&mut self) -> Option<GreenElementRef<'a>> {
        let id = *self.iter.next()?;
        Some(self.wrap(id))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<GreenElementRef<'a>> {
        let id = *self.iter.nth(n)?;
        Some(self.wrap(id))
    }
}

impl DoubleEndedIterator for Children<'_> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        let id = *self.iter.next_back()?;
        Some(self.wrap(id))
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        let id = *self.iter.nth_back(n)?;
        Some(self.wrap(id))
    }
}

impl ExactSizeIterator for Children<'_> {}

impl FusedIterator for Children<'_> {}

struct OpenNode {
    id: NodeId,
    first_pending: usize,
}

/// A builder for a green tree.
#[derive(Default)]
pub struct GreenNodeBuilder {
    source: String,
    nodes: Vec<NodeData>,
    tokens: Vec<TokenData>,
    children: Vec<PackedElementId>,
    pending: Vec<PackedElementId>,
    stack: Vec<OpenNode>,
    root: Option<NodeId>,
    offset: TextSize,
}

impl GreenNodeBuilder {
    /// Creates new builder.
    pub fn new() -> GreenNodeBuilder {
        GreenNodeBuilder::default()
    }

    pub fn with_text_capacity(text_len: usize) -> GreenNodeBuilder {
        GreenNodeBuilder {
            source: String::with_capacity(text_len),
            ..GreenNodeBuilder::default()
        }
    }

    pub fn with_source(source: &str, nodes: usize, tokens: usize) -> GreenNodeBuilder {
        GreenNodeBuilder {
            source: source.to_owned(),
            nodes: Vec::with_capacity(nodes),
            tokens: Vec::with_capacity(tokens),
            children: Vec::with_capacity(nodes + tokens),
            ..GreenNodeBuilder::default()
        }
    }

    /// Adds new token to the current branch.
    #[inline]
    pub fn token(&mut self, kind: SyntaxKind, text: &str) {
        let parent = self.stack.last().expect("tokens must be inside a node");
        let id = TokenId(u32::try_from(self.tokens.len()).expect("too many tokens"));
        let len = TextSize::of(text);
        let index_in_parent =
            u32::try_from(self.pending.len() - parent.first_pending).expect("too many children");
        self.tokens.push(TokenData {
            kind,
            parent: parent.id,
            index_in_parent,
            start: self.offset,
        });
        self.pending.push(PackedElementId::token(id));
        self.source.push_str(text);
        self.offset += len;
    }

    #[inline]
    pub fn source_token(&mut self, kind: SyntaxKind, len: TextSize) {
        let parent = self.stack.last().expect("tokens must be inside a node");
        let id = TokenId(u32::try_from(self.tokens.len()).expect("too many tokens"));
        let index_in_parent =
            u32::try_from(self.pending.len() - parent.first_pending).expect("too many children");
        self.tokens.push(TokenData {
            kind,
            parent: parent.id,
            index_in_parent,
            start: self.offset,
        });
        self.pending.push(PackedElementId::token(id));
        self.offset += len;
    }

    /// Start new node and make it current.
    #[inline]
    pub fn start_node(&mut self, kind: SyntaxKind) {
        let id = NodeId(u32::try_from(self.nodes.len()).expect("too many nodes"));
        let parent = self.stack.last().map_or(NO_PARENT, |node| node.id);
        self.nodes.push(NodeData {
            kind,
            parent,
            index_in_parent: 0,
            first_child: 0,
            child_count: 0,
            first_token: u32::try_from(self.tokens.len()).expect("too many tokens"),
            nodes_end: 0,
            tokens_end: 0,
        });
        self.stack.push(OpenNode {
            id,
            first_pending: self.pending.len(),
        });
    }

    /// Finish current branch and restore previous
    /// branch as current.
    #[inline]
    pub fn finish_node(&mut self) {
        let open = self
            .stack
            .pop()
            .expect("`finish_node` called without a matching `start_node`");
        let first_child = u32::try_from(self.children.len()).expect("too many children");
        let child_count =
            u32::try_from(self.pending.len() - open.first_pending).expect("too many children");
        self.children
            .extend(self.pending.drain(open.first_pending..));
        let index_in_parent = self.stack.last().map_or(0, |parent| {
            u32::try_from(self.pending.len() - parent.first_pending).expect("too many children")
        });

        let nodes_end = u32::try_from(self.nodes.len()).expect("too many nodes");
        let tokens_end = u32::try_from(self.tokens.len()).expect("too many tokens");
        let node = &mut self.nodes[open.id.0 as usize];
        node.index_in_parent = index_in_parent;
        node.first_child = first_child;
        node.child_count = child_count;
        node.nodes_end = nodes_end;
        node.tokens_end = tokens_end;

        if self.stack.last().is_some() {
            self.pending.push(PackedElementId::node(open.id));
        } else {
            assert!(self.root.is_none(), "a green tree has exactly one root");
            self.root = Some(open.id);
        }
    }

    /// Complete tree building. Make sure that
    /// `start_node` and `finish_node` calls
    /// are paired!
    #[inline]
    pub fn finish(mut self) -> GreenNode {
        assert!(self.stack.is_empty(), "unfinished node");
        assert_eq!(self.root, Some(GreenNode::ROOT));
        self.source.truncate(self.offset.into());
        GreenNode {
            tree: Arc::new(GreenTree {
                source: self.source.into_boxed_str(),
                nodes: self.nodes.into_boxed_slice(),
                tokens: self.tokens.into_boxed_slice(),
                children: self.children.into_boxed_slice(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assert_send_sync() {
        fn f<T: Send + Sync>() {}
        f::<GreenNode>();
    }

    #[test]
    fn test_size_of() {
        use std::mem::size_of;

        eprintln!("GreenNode          {}", size_of::<GreenNode>());
        eprintln!("GreenNodeRef       {}", size_of::<GreenNodeRef<'_>>());
    }
}
