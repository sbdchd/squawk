use rowan::{Direction, NodeOrToken};
use squawk_line_index::UniversalNewlines;
use squawk_syntax::{
    SyntaxNode, SyntaxToken,
    ast::{self, AstToken},
};

pub(crate) fn preceding_comment(node: &SyntaxNode) -> Option<String> {
    let mut comments = vec![];

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

        // In the following case, we would skip the `-- foo` since it's not
        // connected to the function:
        //
        // -- foo
        //
        // create function foo() returns void
        //   as 'select 1' language sql;
        if let Some(ws) = ast::Whitespace::cast(token)
            && !ws.spans_multiple_lines()
        {
            continue;
        }

        break;
    }

    if comments.is_empty() {
        None
    } else {
        comments.reverse();
        Some(comments.join("\n"))
    }
}

fn normalize_comment(comment: &str) -> String {
    if let Some(content) = line_comment_content(comment) {
        return content.to_string();
    }

    if let Some(content) = block_comment_content(comment) {
        let normalized = content
            .universal_newlines()
            .map(|line| strip_block_decoration(line.as_str()))
            .collect::<Vec<_>>()
            .join("\n");

        return normalized.trim().to_string();
    }

    comment.trim().to_string()
}

pub(crate) fn line_comment_content(comment: &str) -> Option<&str> {
    comment.strip_prefix("--").map(str::trim)
}

pub(crate) fn block_comment_content(comment: &str) -> Option<&str> {
    comment.strip_prefix("/*")?.strip_suffix("*/")
}

pub(crate) fn strip_block_decoration(line: &str) -> &str {
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

pub(crate) fn line_comment_group(comment: &ast::Comment) -> Vec<ast::Comment> {
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

#[cfg(test)]
mod tests {
    use crate::db::{Database, File, parse};

    use insta::assert_snapshot;
    use squawk_syntax::ast::AstNode;

    #[must_use]
    fn preceding_comment(sql: &str) -> String {
        let db = Database::default();
        let file = File::new(&db, sql.to_string().into());
        let parse = parse(&db, file);
        assert_eq!(parse.errors(), vec![]);

        let stmt = parse.tree().stmts().next().unwrap();
        super::preceding_comment(stmt.syntax()).unwrap()
    }

    fn no_comment(sql: &str) {
        let db = Database::default();
        let file = File::new(&db, sql.to_string().into());
        let parse = parse(&db, file);
        assert_eq!(parse.errors(), vec![]);

        let stmt = parse.tree().stmts().next().unwrap();
        assert!(
            super::preceding_comment(stmt.syntax()).is_none(),
            "We shouldn't find a comment, if that's expected, use the preceding_comment instead"
        );
    }

    #[test]
    fn not_preceding_func() {
        no_comment(
            "
-- whitespace between so we don't count this

create function foo() returns int as $$ select 1 $$ language sql;
",
        );
    }

    #[test]
    fn not_preceding_func_with_cr_line_endings() {
        no_comment(
            "-- not connected\r\rcreate function foo() returns int as $$ select 1 $$ language sql;",
        );
    }

    #[test]
    fn preceding_func_line() {
        let comment = preceding_comment(
            "
-- whitespace between this and the following, so skip it

-- this is a doc comment
-- for foo
create function foo() returns int as $$ select 1 $$ language sql;
",
        );

        assert_snapshot!(comment, @r"
        this is a doc comment
        for foo
        ");
    }

    #[test]
    fn preceding_func_block() {
        let comment = preceding_comment(
            "
/** line 1 */
/* line 2 */
create function foo() returns int as $$ select 1 $$ language sql;
",
        );

        assert_snapshot!(comment, @"
        line 1
        line 2
        ");
    }
}
