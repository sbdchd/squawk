use rowan::{Direction, NodeOrToken};
use squawk_syntax::{
    SyntaxNode,
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
            && !ws.text().contains("\n\n")
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
    if let Some(comment) = comment.strip_prefix("--") {
        return comment.trim().to_string();
    }

    if let Some(comment) = comment
        .strip_prefix("/*")
        .and_then(|comment| comment.strip_suffix("*/"))
    {
        let normalized = comment
            .lines()
            .map(|line| line.trim_start().trim_start_matches('*').trim_start())
            .collect::<Vec<_>>()
            .join("\n");

        return normalized.trim().to_string();
    }

    comment.trim().to_string()
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
