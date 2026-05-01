use rowan::TextSize;
use salsa::Database as Db;
use squawk_syntax::{SyntaxKind, ast::AstNode};

use crate::db::{File, parse};

use super::{ActionKind, CodeAction};

pub(super) fn rewrite_as_regular_string(
    db: &dyn Db,
    file: File,
    actions: &mut Vec<CodeAction>,
    offset: TextSize,
) -> Option<()> {
    let dollar_string = parse(db, file)
        .tree()
        .syntax()
        .token_at_offset(offset)
        .find(|token| token.kind() == SyntaxKind::DOLLAR_QUOTED_STRING)?;

    let replacement = dollar_quoted_to_string(dollar_string.text())?;
    actions.push(CodeAction {
        title: "Rewrite as regular string".to_owned(),
        edits: vec![squawk_linter::Edit::replace(
            dollar_string.text_range(),
            replacement,
        )],
        kind: ActionKind::RefactorRewrite,
    });

    Some(())
}

fn dollar_quoted_to_string(text: &str) -> Option<String> {
    debug_assert!(text.starts_with('$'));
    let (delimiter, content) = split_dollar_quoted(text)?;
    let boundary = format!("${}$", delimiter);

    if !text.starts_with(&boundary) || !text.ends_with(&boundary) {
        return None;
    }

    // quotes are escaped by using two of them in Postgres
    let escaped = content.replace('\'', "''");
    Some(format!("'{}'", escaped))
}

fn split_dollar_quoted(text: &str) -> Option<(String, &str)> {
    debug_assert!(text.starts_with('$'));
    let second_dollar = text[1..].find('$')?;
    // the `foo` in `select $foo$bar$foo$`
    let delimiter = &text[1..=second_dollar];
    let boundary = format!("${}$", delimiter);

    if !text.ends_with(&boundary) {
        return None;
    }

    let start = boundary.len();
    let end = text.len().checked_sub(boundary.len())?;
    let content = text.get(start..end)?;
    Some((delimiter.to_owned(), content))
}

#[cfg(test)]
mod test {
    use insta::assert_snapshot;

    use crate::code_actions::test_utils::{apply_code_action, code_action_not_applicable};

    use super::rewrite_as_regular_string;

    #[test]
    fn rewrite_dollar_string() {
        assert_snapshot!(apply_code_action(
            rewrite_as_regular_string,
            "select $$fo$0o$$;"),
            @"select 'foo';"
        );
    }

    #[test]
    fn rewrite_dollar_string_with_tag() {
        assert_snapshot!(apply_code_action(
            rewrite_as_regular_string,
            "select $tag$fo$0o$tag$;"),
            @"select 'foo';"
        );
    }

    #[test]
    fn rewrite_dollar_string_with_quote() {
        assert_snapshot!(apply_code_action(
            rewrite_as_regular_string,
            "select $$it'$0s fine$$;"),
            @"select 'it''s fine';"
        );
    }

    #[test]
    fn rewrite_dollar_string_not_applicable() {
        assert!(code_action_not_applicable(
            rewrite_as_regular_string,
            "select 'foo$0';"
        ));
    }
}
