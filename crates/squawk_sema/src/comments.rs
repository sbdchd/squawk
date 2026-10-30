use rowan::{Direction, NodeOrToken};
use smallvec::SmallVec;
use squawk_line_index::UniversalNewlines;
use squawk_syntax::{
    SyntaxNode, SyntaxToken,
    ast::{self, AstToken},
};

pub fn line_comment_content(comment: &str) -> Option<&str> {
    comment.strip_prefix("--").map(str::trim)
}

pub fn block_comment_content(comment: &str) -> Option<&str> {
    comment.strip_prefix("/*")?.strip_suffix("*/")
}

pub fn strip_block_decoration(line: &str) -> &str {
    let line = line.trim();
    let Some(content) = line.strip_prefix('*') else {
        return line;
    };

    if content.is_empty() || content.chars().next().is_some_and(char::is_whitespace) {
        content.trim()
    } else {
        line
    }
}

pub fn line_comment_group(comment: &ast::Comment) -> Vec<ast::Comment> {
    let mut comments = vec![comment.clone()];

    let mut current = comment.clone();
    while let Some(prev) = adjacent_line_comment(&current, Direction::Prev) {
        comments.push(prev.clone());
        current = prev;
    }
    comments.reverse();

    let mut current = comment.clone();
    while let Some(next) = adjacent_line_comment(&current, Direction::Next) {
        comments.push(next.clone());
        current = next;
    }

    comments
}

fn line_break_count(text: &str) -> usize {
    text.universal_newlines()
        .filter(|line| line.line_ending().is_some())
        .count()
}

fn adjacent_line_comment(comment: &ast::Comment, direction: Direction) -> Option<ast::Comment> {
    let whitespace = ast::Whitespace::cast(adjacent_token(comment.syntax(), direction)?)?;
    if line_break_count(whitespace.text()) != 1 {
        return None;
    }
    let adjacent = ast::Comment::cast(adjacent_token(whitespace.syntax(), direction)?)?;
    adjacent.kind().is_line().then_some(adjacent)
}

fn adjacent_token(token: &SyntaxToken, direction: Direction) -> Option<SyntaxToken> {
    match direction {
        Direction::Next => token.next_token(),
        Direction::Prev => token.prev_token(),
    }
}

pub(crate) fn preceding_comment(node: &SyntaxNode) -> Option<String> {
    let mut comments: SmallVec<[String; 2]> = SmallVec::new();
    for element in node.siblings_with_tokens(Direction::Prev).skip(1) {
        let NodeOrToken::Token(token) = element else {
            break;
        };
        if let Some(comment) = ast::Comment::cast(token.clone()) {
            let comment = normalize_comment(comment.text());
            if !comment.is_empty() {
                comments.push(comment);
            }
            continue;
        }
        if let Some(whitespace) = ast::Whitespace::cast(token)
            && !whitespace.spans_multiple_lines()
        {
            continue;
        }
        break;
    }
    if comments.is_empty() {
        return None;
    }
    let mut text = String::with_capacity(comments.iter().map(|comment| comment.len() + 1).sum());
    for (i, comment) in comments.iter().rev().enumerate() {
        if i > 0 {
            text.push('\n');
        }
        text.push_str(comment);
    }
    Some(text)
}

fn normalize_comment(comment: &str) -> String {
    if let Some(content) = comment.strip_prefix("--") {
        return content.trim().to_string();
    }
    if let Some(content) = comment
        .strip_prefix("/*")
        .and_then(|comment| comment.strip_suffix("*/"))
    {
        let mut text = String::with_capacity(content.len());
        for (i, line) in content
            .universal_newlines()
            .map(|line| line.as_str())
            .enumerate()
        {
            if i > 0 {
                text.push('\n');
            }
            text.push_str(strip_block_decoration(line));
        }
        return text.trim().to_string();
    }
    comment.trim().to_string()
}
