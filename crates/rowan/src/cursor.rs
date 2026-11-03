//! Implementation of the cursors -- API for convenient access to syntax trees.
//!
//! A cursor (`SyntaxNode` or `SyntaxToken`) is a reference counted handle to
//! shared `Rc<GreenNode>` plus the id of an entry in its node or token table.

use std::{
    fmt,
    hash::{Hash, Hasher},
    iter,
    rc::Rc,
};

use crate::{
    green::{ElementId, GreenNodeBuilder, GreenNodeRef, GreenTokenRef, NodeId, TokenId},
    Direction, GreenNode, NodeOrToken, SyntaxKind, SyntaxText, TextRange, TextSize, TokenAtOffset,
    WalkEvent,
};

pub type SyntaxElement = NodeOrToken<SyntaxNode, SyntaxToken>;

#[derive(Clone)]
pub struct SyntaxNode {
    green: Rc<GreenNode>,
    id: NodeId,
}

#[derive(Clone)]
pub struct SyntaxToken {
    green: Rc<GreenNode>,
    id: TokenId,
}

#[inline]
fn element(green: &Rc<GreenNode>, id: ElementId) -> SyntaxElement {
    let green = green.clone();
    match id {
        ElementId::Node(id) => SyntaxNode { green, id }.into(),
        ElementId::Token(id) => SyntaxToken { green, id }.into(),
    }
}

#[inline]
fn sibling(
    green: &Rc<GreenNode>,
    parent: Option<NodeId>,
    index: Option<u32>,
) -> Option<SyntaxElement> {
    let id = green.child(parent?, index?)?;
    Some(element(green, id))
}

impl SyntaxNode {
    pub fn new_root(green: GreenNode) -> SyntaxNode {
        SyntaxNode {
            green: Rc::new(green),
            id: GreenNode::ROOT,
        }
    }

    pub fn clone_subtree(&self) -> SyntaxNode {
        let mut builder = GreenNodeBuilder::with_text_capacity(self.text_range().len().into());
        for event in self.preorder_with_tokens() {
            match event {
                WalkEvent::Enter(NodeOrToken::Node(node)) => builder.start_node(node.kind()),
                WalkEvent::Enter(NodeOrToken::Token(token)) => {
                    builder.token(token.kind(), token.text())
                }
                WalkEvent::Leave(NodeOrToken::Node(_)) => builder.finish_node(),
                WalkEvent::Leave(NodeOrToken::Token(_)) => (),
            }
        }
        SyntaxNode::new_root(builder.finish())
    }

    #[inline]
    fn node(&self, id: NodeId) -> SyntaxNode {
        SyntaxNode {
            green: self.green.clone(),
            id,
        }
    }

    #[inline]
    pub(crate) fn text_at(&self, range: TextRange) -> &str {
        self.green.text_at(range)
    }

    #[inline]
    pub fn kind(&self) -> SyntaxKind {
        self.green.node_kind(self.id)
    }

    #[inline]
    pub fn text_range(&self) -> TextRange {
        self.green.node_range(self.id)
    }

    #[inline]
    pub fn index(&self) -> usize {
        self.green.node_index(self.id) as usize
    }

    #[inline]
    pub fn text(&self) -> SyntaxText {
        SyntaxText::new(self.clone())
    }

    #[inline]
    pub fn green(&self) -> GreenNodeRef<'_> {
        self.green.node_ref(self.id)
    }

    #[inline]
    pub fn parent(&self) -> Option<SyntaxNode> {
        self.green.node_parent(self.id).map(|id| self.node(id))
    }

    #[inline]
    pub fn ancestors(&self) -> impl Iterator<Item = SyntaxNode> {
        iter::successors(Some(self.clone()), SyntaxNode::parent)
    }

    #[inline]
    pub fn tree_top(&self) -> SyntaxNode {
        self.ancestors().last().unwrap()
    }

    #[inline]
    pub fn children(&self) -> SyntaxNodeChildren {
        SyntaxNodeChildren {
            parent: self.clone(),
            next_index: 0,
        }
    }

    #[inline]
    pub fn children_with_tokens(&self) -> SyntaxElementChildren {
        SyntaxElementChildren {
            parent: self.clone(),
            next_index: 0,
        }
    }

    #[inline]
    pub fn first_child(&self) -> Option<SyntaxNode> {
        self.children().next()
    }
    #[inline]
    pub fn last_child(&self) -> Option<SyntaxNode> {
        let count = self.green.child_count(self.id);
        (0..count)
            .rev()
            .find_map(|index| match self.green.child(self.id, index)? {
                ElementId::Node(id) => Some(self.node(id)),
                ElementId::Token(_) => None,
            })
    }

    #[inline]
    pub fn first_child_or_token(&self) -> Option<SyntaxElement> {
        sibling(&self.green, Some(self.id), Some(0))
    }
    #[inline]
    pub fn last_child_or_token(&self) -> Option<SyntaxElement> {
        let index = self.green.child_count(self.id).checked_sub(1);
        sibling(&self.green, Some(self.id), index)
    }

    #[inline]
    pub fn next_sibling(&self) -> Option<SyntaxNode> {
        let parent = self.green.node_parent(self.id)?;
        let count = self.green.child_count(parent);
        let start = self.green.node_index(self.id) + 1;
        (start..count).find_map(|index| match self.green.child(parent, index)? {
            ElementId::Node(id) => Some(self.node(id)),
            ElementId::Token(_) => None,
        })
    }
    #[inline]
    pub fn prev_sibling(&self) -> Option<SyntaxNode> {
        let parent = self.green.node_parent(self.id)?;
        let end = self.green.node_index(self.id);
        (0..end)
            .rev()
            .find_map(|index| match self.green.child(parent, index)? {
                ElementId::Node(id) => Some(self.node(id)),
                ElementId::Token(_) => None,
            })
    }

    #[inline]
    pub fn next_sibling_or_token(&self) -> Option<SyntaxElement> {
        let index = self.green.node_index(self.id).checked_add(1);
        sibling(&self.green, self.green.node_parent(self.id), index)
    }
    #[inline]
    pub fn prev_sibling_or_token(&self) -> Option<SyntaxElement> {
        let index = self.green.node_index(self.id).checked_sub(1);
        sibling(&self.green, self.green.node_parent(self.id), index)
    }

    #[inline]
    pub fn first_token(&self) -> Option<SyntaxToken> {
        self.first_child_or_token()?.first_token()
    }
    #[inline]
    pub fn last_token(&self) -> Option<SyntaxToken> {
        self.last_child_or_token()?.last_token()
    }

    #[inline]
    pub fn siblings(&self, direction: Direction) -> impl Iterator<Item = SyntaxNode> {
        iter::successors(Some(self.clone()), move |node| match direction {
            Direction::Next => node.next_sibling(),
            Direction::Prev => node.prev_sibling(),
        })
    }

    #[inline]
    pub fn siblings_with_tokens(
        &self,
        direction: Direction,
    ) -> impl Iterator<Item = SyntaxElement> {
        let me: SyntaxElement = self.clone().into();
        iter::successors(Some(me), move |el| match direction {
            Direction::Next => el.next_sibling_or_token(),
            Direction::Prev => el.prev_sibling_or_token(),
        })
    }

    #[inline]
    pub fn descendants(&self) -> impl Iterator<Item = SyntaxNode> {
        self.preorder().filter_map(|event| match event {
            WalkEvent::Enter(node) => Some(node),
            WalkEvent::Leave(_) => None,
        })
    }

    #[inline]
    pub fn descendants_with_tokens(&self) -> impl Iterator<Item = SyntaxElement> {
        self.preorder_with_tokens().filter_map(|event| match event {
            WalkEvent::Enter(it) => Some(it),
            WalkEvent::Leave(_) => None,
        })
    }

    #[inline]
    pub fn preorder(&self) -> Preorder {
        Preorder::new(self.clone())
    }

    #[inline]
    pub fn preorder_with_tokens(&self) -> PreorderWithTokens {
        PreorderWithTokens::new(self.clone())
    }

    pub fn token_at_offset(&self, offset: TextSize) -> TokenAtOffset<SyntaxToken> {
        let range = self.text_range();
        assert!(
            range.start() <= offset && offset <= range.end(),
            "Bad offset: range {:?} offset {:?}",
            range,
            offset
        );
        if range.is_empty() {
            return TokenAtOffset::None;
        }

        let mut children = self.children_with_tokens().filter(|child| {
            let child_range = child.text_range();
            !child_range.is_empty()
                && (child_range.start() <= offset && offset <= child_range.end())
        });

        let left = children.next().unwrap();
        let right = children.next();
        assert!(children.next().is_none());

        if let Some(right) = right {
            match (left.token_at_offset(offset), right.token_at_offset(offset)) {
                (TokenAtOffset::Single(left), TokenAtOffset::Single(right)) => {
                    TokenAtOffset::Between(left, right)
                }
                _ => unreachable!(),
            }
        } else {
            left.token_at_offset(offset)
        }
    }

    pub fn covering_element(&self, range: TextRange) -> SyntaxElement {
        let mut res: SyntaxElement = self.clone().into();
        loop {
            assert!(
                res.text_range().contains_range(range),
                "Bad range: node range {:?}, range {:?}",
                res.text_range(),
                range,
            );
            res = match &res {
                NodeOrToken::Token(_) => return res,
                NodeOrToken::Node(node) => match node.child_or_token_at_range(range) {
                    Some(it) => it,
                    None => return res,
                },
            };
        }
    }

    #[inline]
    pub fn child_or_token_at_range(&self, range: TextRange) -> Option<SyntaxElement> {
        let id = self.green.child_at_range(self.id, range)?;
        Some(element(&self.green, id))
    }
}

impl SyntaxToken {
    #[inline]
    pub fn kind(&self) -> SyntaxKind {
        self.green.token_kind(self.id)
    }

    #[inline]
    pub fn text_range(&self) -> TextRange {
        self.green.token_range(self.id)
    }

    #[inline]
    pub fn index(&self) -> usize {
        self.green.token_index(self.id) as usize
    }

    #[inline]
    pub fn text(&self) -> &str {
        self.green.text_at(self.text_range())
    }

    #[inline]
    pub fn green(&self) -> GreenTokenRef<'_> {
        self.green.token_ref(self.id)
    }

    #[inline]
    pub fn parent(&self) -> Option<SyntaxNode> {
        Some(SyntaxNode {
            green: self.green.clone(),
            id: self.green.token_parent(self.id),
        })
    }

    #[inline]
    pub fn ancestors(&self) -> impl Iterator<Item = SyntaxNode> {
        std::iter::successors(self.parent(), SyntaxNode::parent)
    }

    #[inline]
    pub fn tree_top(&self) -> SyntaxNode {
        self.ancestors().last().unwrap()
    }

    #[inline]
    pub fn next_sibling_or_token(&self) -> Option<SyntaxElement> {
        let index = self.green.token_index(self.id).checked_add(1);
        sibling(&self.green, Some(self.green.token_parent(self.id)), index)
    }
    #[inline]
    pub fn prev_sibling_or_token(&self) -> Option<SyntaxElement> {
        let index = self.green.token_index(self.id).checked_sub(1);
        sibling(&self.green, Some(self.green.token_parent(self.id)), index)
    }

    #[inline]
    pub fn siblings_with_tokens(
        &self,
        direction: Direction,
    ) -> impl Iterator<Item = SyntaxElement> {
        let me: SyntaxElement = self.clone().into();
        iter::successors(Some(me), move |el| match direction {
            Direction::Next => el.next_sibling_or_token(),
            Direction::Prev => el.prev_sibling_or_token(),
        })
    }

    #[inline]
    pub fn next_token(&self) -> Option<SyntaxToken> {
        match self.next_sibling_or_token() {
            Some(element) => element.first_token(),
            None => self
                .ancestors()
                .find_map(|it| it.next_sibling_or_token())
                .and_then(|element| element.first_token()),
        }
    }
    #[inline]
    pub fn prev_token(&self) -> Option<SyntaxToken> {
        match self.prev_sibling_or_token() {
            Some(element) => element.last_token(),
            None => self
                .ancestors()
                .find_map(|it| it.prev_sibling_or_token())
                .and_then(|element| element.last_token()),
        }
    }
}

impl SyntaxElement {
    #[inline]
    pub fn text_range(&self) -> TextRange {
        match self {
            NodeOrToken::Node(it) => it.text_range(),
            NodeOrToken::Token(it) => it.text_range(),
        }
    }

    #[inline]
    pub fn index(&self) -> usize {
        match self {
            NodeOrToken::Node(it) => it.index(),
            NodeOrToken::Token(it) => it.index(),
        }
    }

    #[inline]
    pub fn kind(&self) -> SyntaxKind {
        match self {
            NodeOrToken::Node(it) => it.kind(),
            NodeOrToken::Token(it) => it.kind(),
        }
    }

    #[inline]
    pub fn parent(&self) -> Option<SyntaxNode> {
        match self {
            NodeOrToken::Node(it) => it.parent(),
            NodeOrToken::Token(it) => it.parent(),
        }
    }

    #[inline]
    pub fn ancestors(&self) -> impl Iterator<Item = SyntaxNode> {
        let first = match self {
            NodeOrToken::Node(it) => Some(it.clone()),
            NodeOrToken::Token(it) => it.parent(),
        };
        iter::successors(first, SyntaxNode::parent)
    }

    #[inline]
    pub fn tree_top(&self) -> SyntaxNode {
        match self {
            NodeOrToken::Node(it) => it.tree_top(),
            NodeOrToken::Token(it) => it.tree_top(),
        }
    }

    #[inline]
    pub fn first_token(&self) -> Option<SyntaxToken> {
        match self {
            NodeOrToken::Node(it) => it.first_token(),
            NodeOrToken::Token(it) => Some(it.clone()),
        }
    }
    #[inline]
    pub fn last_token(&self) -> Option<SyntaxToken> {
        match self {
            NodeOrToken::Node(it) => it.last_token(),
            NodeOrToken::Token(it) => Some(it.clone()),
        }
    }

    #[inline]
    pub fn next_sibling_or_token(&self) -> Option<SyntaxElement> {
        match self {
            NodeOrToken::Node(it) => it.next_sibling_or_token(),
            NodeOrToken::Token(it) => it.next_sibling_or_token(),
        }
    }
    #[inline]
    pub fn prev_sibling_or_token(&self) -> Option<SyntaxElement> {
        match self {
            NodeOrToken::Node(it) => it.prev_sibling_or_token(),
            NodeOrToken::Token(it) => it.prev_sibling_or_token(),
        }
    }

    fn token_at_offset(&self, offset: TextSize) -> TokenAtOffset<SyntaxToken> {
        assert!(self.text_range().start() <= offset && offset <= self.text_range().end());
        match self {
            NodeOrToken::Token(token) => TokenAtOffset::Single(token.clone()),
            NodeOrToken::Node(node) => node.token_at_offset(offset),
        }
    }
}

// region: impls

// Identity semantics for hash & eq
impl PartialEq for SyntaxNode {
    #[inline]
    fn eq(&self, other: &SyntaxNode) -> bool {
        self.id == other.id && self.green.ptr_eq(&other.green)
    }
}

impl Eq for SyntaxNode {}

impl Hash for SyntaxNode {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.green.addr().hash(state);
        self.id.hash(state);
    }
}

impl fmt::Debug for SyntaxNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SyntaxNode")
            .field("kind", &self.kind())
            .field("text_range", &self.text_range())
            .finish()
    }
}

impl fmt::Display for SyntaxNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.text_at(self.text_range()))
    }
}

// Identity semantics for hash & eq
impl PartialEq for SyntaxToken {
    #[inline]
    fn eq(&self, other: &SyntaxToken) -> bool {
        self.id == other.id && self.green.ptr_eq(&other.green)
    }
}

impl Eq for SyntaxToken {}

impl Hash for SyntaxToken {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.green.addr().hash(state);
        self.id.hash(state);
    }
}

impl fmt::Debug for SyntaxToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SyntaxToken")
            .field("kind", &self.kind())
            .field("text_range", &self.text_range())
            .finish()
    }
}

impl fmt::Display for SyntaxToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.text(), f)
    }
}

impl From<SyntaxNode> for SyntaxElement {
    #[inline]
    fn from(node: SyntaxNode) -> SyntaxElement {
        NodeOrToken::Node(node)
    }
}

impl From<SyntaxToken> for SyntaxElement {
    #[inline]
    fn from(token: SyntaxToken) -> SyntaxElement {
        NodeOrToken::Token(token)
    }
}

// endregion

// region: iterators

#[derive(Clone, Debug)]
pub struct SyntaxNodeChildren {
    parent: SyntaxNode,
    next_index: u32,
}

impl Iterator for SyntaxNodeChildren {
    type Item = SyntaxNode;
    #[inline]
    fn next(&mut self) -> Option<SyntaxNode> {
        while let Some(child) = self.parent.green.child(self.parent.id, self.next_index) {
            self.next_index += 1;
            if let ElementId::Node(id) = child {
                return Some(self.parent.node(id));
            }
        }
        None
    }
}

#[derive(Clone, Debug)]
pub struct SyntaxElementChildren {
    parent: SyntaxNode,
    next_index: u32,
}

impl Iterator for SyntaxElementChildren {
    type Item = SyntaxElement;
    #[inline]
    fn next(&mut self) -> Option<SyntaxElement> {
        let child = self.parent.green.child(self.parent.id, self.next_index)?;
        self.next_index += 1;
        Some(element(&self.parent.green, child))
    }
}

#[inline]
fn element_parent(green: &GreenNode, id: ElementId) -> Option<NodeId> {
    match id {
        ElementId::Node(id) => green.node_parent(id),
        ElementId::Token(id) => Some(green.token_parent(id)),
    }
}

#[inline]
fn element_index(green: &GreenNode, id: ElementId) -> u32 {
    match id {
        ElementId::Node(id) => green.node_index(id),
        ElementId::Token(id) => green.token_index(id),
    }
}

#[inline]
fn first_child_node(green: &GreenNode, parent: NodeId, from: u32) -> Option<NodeId> {
    let count = green.child_count(parent);
    (from..count).find_map(|index| match green.child(parent, index)? {
        ElementId::Node(id) => Some(id),
        ElementId::Token(_) => None,
    })
}

#[derive(Debug, Clone)]
pub struct Preorder {
    green: Rc<GreenNode>,
    start: NodeId,
    next: Option<WalkEvent<NodeId>>,
    skip_subtree: bool,
}

impl Preorder {
    #[inline]
    fn new(start: SyntaxNode) -> Preorder {
        Preorder {
            next: Some(WalkEvent::Enter(start.id)),
            start: start.id,
            green: start.green,
            skip_subtree: false,
        }
    }

    pub fn skip_subtree(&mut self) {
        self.skip_subtree = true;
    }

    #[cold]
    fn do_skip(&mut self) {
        let green = &self.green;
        self.next = self.next.take().map(|next| match next {
            WalkEvent::Enter(first_child) => {
                WalkEvent::Leave(green.node_parent(first_child).unwrap())
            }
            WalkEvent::Leave(parent) => WalkEvent::Leave(parent),
        })
    }

    #[inline]
    fn advance(&self, event: WalkEvent<NodeId>) -> Option<WalkEvent<NodeId>> {
        let green = &self.green;
        Some(match event {
            WalkEvent::Enter(node) => match first_child_node(green, node, 0) {
                Some(child) => WalkEvent::Enter(child),
                None => WalkEvent::Leave(node),
            },
            WalkEvent::Leave(node) => {
                if node == self.start {
                    return None;
                }
                let parent = green.node_parent(node)?;
                match first_child_node(green, parent, green.node_index(node) + 1) {
                    Some(sibling) => WalkEvent::Enter(sibling),
                    None => WalkEvent::Leave(parent),
                }
            }
        })
    }
}

impl Iterator for Preorder {
    type Item = WalkEvent<SyntaxNode>;

    #[inline]
    fn next(&mut self) -> Option<WalkEvent<SyntaxNode>> {
        if self.skip_subtree {
            self.do_skip();
            self.skip_subtree = false;
        }
        let next = self.next.take()?;
        self.next = self.advance(next);
        let green = &self.green;
        Some(next.map(|id| SyntaxNode {
            green: green.clone(),
            id,
        }))
    }
}

#[derive(Debug, Clone)]
pub struct PreorderWithTokens {
    green: Rc<GreenNode>,
    start: NodeId,
    next: Option<WalkEvent<ElementId>>,
    skip_subtree: bool,
}

impl PreorderWithTokens {
    #[inline]
    fn new(start: SyntaxNode) -> PreorderWithTokens {
        PreorderWithTokens {
            next: Some(WalkEvent::Enter(ElementId::Node(start.id))),
            start: start.id,
            green: start.green,
            skip_subtree: false,
        }
    }

    pub fn skip_subtree(&mut self) {
        self.skip_subtree = true;
    }

    #[cold]
    fn do_skip(&mut self) {
        let green = &self.green;
        self.next = self.next.take().map(|next| match next {
            WalkEvent::Enter(first_child) => {
                WalkEvent::Leave(ElementId::Node(element_parent(green, first_child).unwrap()))
            }
            WalkEvent::Leave(parent) => WalkEvent::Leave(parent),
        })
    }

    #[inline]
    fn advance(&self, event: WalkEvent<ElementId>) -> Option<WalkEvent<ElementId>> {
        let green = &self.green;
        Some(match event {
            WalkEvent::Enter(ElementId::Node(node)) => match green.child(node, 0) {
                Some(child) => WalkEvent::Enter(child),
                None => WalkEvent::Leave(ElementId::Node(node)),
            },
            WalkEvent::Enter(token @ ElementId::Token(_)) => WalkEvent::Leave(token),
            WalkEvent::Leave(ElementId::Node(node)) if node == self.start => return None,
            WalkEvent::Leave(el) => {
                let parent = element_parent(green, el)?;
                match green.child(parent, element_index(green, el) + 1) {
                    Some(sibling) => WalkEvent::Enter(sibling),
                    None => WalkEvent::Leave(ElementId::Node(parent)),
                }
            }
        })
    }
}

impl Iterator for PreorderWithTokens {
    type Item = WalkEvent<SyntaxElement>;

    #[inline]
    fn next(&mut self) -> Option<WalkEvent<SyntaxElement>> {
        if self.skip_subtree {
            self.do_skip();
            self.skip_subtree = false;
        }
        let next = self.next.take()?;
        self.next = self.advance(next);
        let green = &self.green;
        Some(next.map(|id| element(green, id)))
    }
}
// endregion
