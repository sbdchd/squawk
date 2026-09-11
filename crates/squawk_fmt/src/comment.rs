use rowan::Direction;
use squawk_line_index::find_newline;
use squawk_syntax::{SyntaxElement, SyntaxKind, SyntaxToken};
use tiny_pretty::Doc;

pub(crate) fn is_line_comment(token: &SyntaxToken) -> bool {
    token.text().starts_with("--")
}

pub(crate) fn build_comment<'a>(token: &SyntaxToken) -> Doc<'a> {
    let mut docs = vec![];
    let mut text = token.text();
    while let Some((position, line_ending)) = find_newline(text) {
        docs.push(Doc::text(
            text[..position].trim_end_matches([' ', '\t']).to_string(),
        ));
        docs.push(Doc::empty_line());
        text = &text[position + line_ending.len()..];
    }
    docs.push(Doc::text(text.trim_end_matches([' ', '\t']).to_string()));
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
    tokens
}

// TODO: remove this since it just aggregates tokens instead of giving us a Doc
pub(crate) fn comment_tokens_before(el: &(impl Into<SyntaxElement> + Clone)) -> Vec<SyntaxToken> {
    let mut tokens = comment_tokens(el, Direction::Prev);
    tokens.reverse();
    tokens
}

pub(crate) fn build_leading_comments<'a>(tokens: &[SyntaxToken]) -> Doc<'a> {
    let mut doc = Doc::nil();
    for token in tokens {
        doc = doc.append(build_comment(token));
        doc = doc.append(if is_line_comment(token) {
            Doc::hard_line()
        } else {
            Doc::space()
        });
    }
    doc
}

fn build_trailing_comments<'a>(tokens: &[SyntaxToken]) -> Doc<'a> {
    let mut doc = Doc::nil();
    let mut after_line_comment = false;
    for token in tokens {
        if !after_line_comment {
            doc = doc.append(Doc::space());
        }
        doc = doc.append(build_comment(token));
        after_line_comment = is_line_comment(token);
        if after_line_comment {
            doc = doc.append(Doc::hard_line());
        }
    }
    doc
}

pub(crate) fn leading_comments<'a>(el: &(impl Into<SyntaxElement> + Clone)) -> Doc<'a> {
    build_leading_comments(&comment_tokens_before(el))
}

pub(crate) fn comments_before<'a>(el: &(impl Into<SyntaxElement> + Clone)) -> Doc<'a> {
    build_trailing_comments(&comment_tokens_before(el))
}

pub(crate) fn trailing_comments<'a>(el: &(impl Into<SyntaxElement> + Clone)) -> Doc<'a> {
    build_trailing_comments(&comment_tokens(el, Direction::Next))
}
