use rowan::Direction;
use squawk_line_index::{UniversalNewlines, find_newline};
use squawk_syntax::{SyntaxElement, SyntaxKind, SyntaxToken};
use tiny_pretty::Doc;

pub(crate) fn is_line_comment(token: &SyntaxToken) -> bool {
    token.text().starts_with("--")
}

pub(crate) fn build_comment<'a>(token: &SyntaxToken) -> Doc<'a> {
    let align_stars = !is_line_comment(token)
        && find_newline(token.text()).is_some()
        && token
            .text()
            .universal_newlines()
            .skip(1)
            .all(|line| line.as_str().trim_start().starts_with('*'));
    let line = |text: &str| {
        let text = text.trim_end_matches([' ', '\t']);
        if is_line_comment(token) {
            if let Some(content) = text.strip_prefix("--") {
                if !content.is_empty() && !content.starts_with(' ') && !content.starts_with('\t') {
                    return Doc::text(format!("-- {content}"));
                }
            }
        } else if align_stars && text.trim_start().starts_with('*') {
            return Doc::text(format!(" {}", text.trim()));
        }
        Doc::text(text.to_string())
    };
    let mut docs = vec![];
    let mut text = token.text();
    while let Some((position, line_ending)) = find_newline(text) {
        docs.push(line(&text[..position]));
        docs.push(Doc::empty_line());
        text = &text[position + line_ending.len()..];
    }
    docs.push(line(text));
    Doc::list(docs)
}

fn comment_tokens(
    el: &(impl Into<SyntaxElement> + Clone),
    direction: Direction,
) -> Vec<SyntaxToken> {
    let sibling = |el: SyntaxElement| match direction {
        Direction::Next => el.next_sibling_or_token(),
        Direction::Prev => el.prev_sibling_or_token(),
    };
    let mut tokens = vec![];
    let mut curr = sibling(el.clone().into());
    while let Some(rowan::NodeOrToken::Token(token)) = curr {
        match token.kind() {
            SyntaxKind::COMMENT => tokens.push(token.clone()),
            SyntaxKind::WHITESPACE => (),
            _ => break,
        }
        curr = sibling(token.into());
    }
    if direction == Direction::Prev {
        tokens.reverse();
    }
    tokens
}

fn is_trailing_comment(token: &SyntaxToken) -> bool {
    match token.prev_token() {
        Some(prev) if prev.kind() == SyntaxKind::WHITESPACE => {
            find_newline(prev.text()).is_none() && prev.prev_token().is_some()
        }
        Some(_) => true,
        None => false,
    }
}

pub(crate) struct CommentRun {
    tokens: Vec<SyntaxToken>,
}

impl CommentRun {
    pub(crate) fn empty() -> Self {
        Self { tokens: vec![] }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.tokens.is_empty()
    }

    fn doc<'a>(&self) -> Doc<'a> {
        let mut docs = vec![];
        for (index, token) in self.tokens.iter().enumerate() {
            docs.push(build_comment(token));
            if index + 1 < self.tokens.len() {
                docs.push(if is_line_comment(token) {
                    Doc::hard_line()
                } else {
                    Doc::space()
                });
            }
        }
        Doc::list(docs)
    }

    pub(crate) fn separator_before<'a>(&self, separator: Doc<'a>) -> Doc<'a> {
        if self
            .tokens
            .first()
            .is_some_and(|token| is_line_comment(token) && is_trailing_comment(token))
        {
            Doc::space()
        } else {
            separator
        }
    }

    fn separator_after<'a>(&self, separator: Doc<'a>) -> Doc<'a> {
        if self.tokens.last().is_some_and(is_line_comment) {
            Doc::hard_line()
        } else {
            separator
        }
    }

    pub(crate) fn before_node<'a>(&self, separator: Doc<'a>) -> Doc<'a> {
        if self.is_empty() {
            return separator;
        }
        self.separator_before(separator)
            .append(self.doc())
            .append(self.separator_after(Doc::space()))
    }

    pub(crate) fn after_node<'a>(&self, separator: Doc<'a>) -> Doc<'a> {
        if self.is_empty() {
            return separator;
        }
        Doc::space()
            .append(self.doc())
            .append(self.separator_after(separator))
    }

    pub(crate) fn between_nodes<'a>(&self, separator: Doc<'a>) -> Doc<'a> {
        if self.is_empty() {
            return separator;
        }
        let before = if self.tokens.first().is_some_and(is_trailing_comment) {
            Doc::space()
        } else {
            separator.clone()
        };
        before
            .append(self.doc())
            .append(self.separator_after(separator))
    }

    pub(crate) fn before_keyword<'a>(&self, separator: Doc<'a>) -> Doc<'a> {
        if self.is_empty() {
            return Doc::space();
        }
        let before = if self.tokens.first().is_some_and(is_trailing_comment) {
            Doc::space()
        } else {
            separator
        };
        before
            .append(self.doc())
            .append(self.separator_after(Doc::space()))
    }

    pub(crate) fn split_trailing(self) -> (CommentRun, CommentRun) {
        let count = self
            .tokens
            .iter()
            .take_while(|token| is_trailing_comment(token))
            .position(is_line_comment)
            .map_or(0, |index| index + 1);
        let mut trailing = self.tokens;
        let rest = trailing.split_off(count);
        (Self { tokens: trailing }, Self { tokens: rest })
    }

    pub(crate) fn trailing<'a>(&self) -> Doc<'a> {
        if self.is_empty() {
            return Doc::nil();
        }
        Doc::space().append(self.doc())
    }

    pub(crate) fn leading<'a>(&self) -> Doc<'a> {
        if self.is_empty() {
            return Doc::nil();
        }
        self.doc().append(self.separator_after(Doc::space()))
    }

    pub(crate) fn before_closing_delimiter<'a>(&self, separator: Doc<'a>) -> (Doc<'a>, Doc<'a>) {
        (self.doc(), self.separator_after(separator))
    }
}

pub(crate) fn comment_run_before(el: &(impl Into<SyntaxElement> + Clone)) -> CommentRun {
    CommentRun {
        tokens: comment_tokens(el, Direction::Prev),
    }
}

pub(crate) fn comment_run_after(el: &(impl Into<SyntaxElement> + Clone)) -> CommentRun {
    CommentRun {
        tokens: comment_tokens(el, Direction::Next),
    }
}

pub(crate) fn leading_comments<'a>(el: &(impl Into<SyntaxElement> + Clone)) -> Doc<'a> {
    comment_run_before(el).leading()
}

pub(crate) fn comments_before<'a>(el: &(impl Into<SyntaxElement> + Clone)) -> Doc<'a> {
    comment_run_before(el).after_node(Doc::nil())
}

pub(crate) fn trailing_comments<'a>(el: &(impl Into<SyntaxElement> + Clone)) -> Doc<'a> {
    comment_run_after(el).after_node(Doc::nil())
}

pub(crate) fn space_or_comments_before<'a>(el: &(impl Into<SyntaxElement> + Clone)) -> Doc<'a> {
    let comments = comment_run_before(el);
    if comments.is_empty() {
        return Doc::space();
    }
    comments.after_node(Doc::nil())
}

pub(crate) fn space_before<'a>(el: &(impl Into<SyntaxElement> + Clone)) -> Doc<'a> {
    comment_run_before(el).before_node(Doc::space())
}

pub(crate) fn line_before<'a>(el: &(impl Into<SyntaxElement> + Clone)) -> Doc<'a> {
    comment_run_before(el).before_node(Doc::line_or_space())
}

pub(crate) fn hard_line_before<'a>(el: &(impl Into<SyntaxElement> + Clone)) -> Doc<'a> {
    comment_run_before(el).before_node(Doc::hard_line())
}

pub(crate) fn has_own_line_comments_before(el: &(impl Into<SyntaxElement> + Clone)) -> bool {
    comment_run_before(el)
        .tokens
        .first()
        .is_some_and(|token| !is_trailing_comment(token))
}

pub(crate) fn has_comments_before(el: &(impl Into<SyntaxElement> + Clone)) -> bool {
    !comment_run_before(el).is_empty()
}
