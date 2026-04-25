use rowan::TextSize;
use salsa::Database as Db;
use squawk_syntax::{SyntaxKind, ast::AstNode};

use crate::db::{File, parse};

use super::{ActionKind, CodeAction};

pub(super) fn rewrite_as_dollar_quoted_string(
    db: &dyn Db,
    file: File,
    actions: &mut Vec<CodeAction>,
    offset: TextSize,
) -> Option<()> {
    let string = parse(db, file)
        .tree()
        .syntax()
        .token_at_offset(offset)
        .find(|token| token.kind() == SyntaxKind::STRING)?;

    let replacement = string_to_dollar_quoted(string.text())?;
    actions.push(CodeAction {
        title: "Rewrite as dollar-quoted string".to_owned(),
        edits: vec![squawk_linter::Edit::replace(
            string.text_range(),
            replacement,
        )],
        kind: ActionKind::RefactorRewrite,
    });

    Some(())
}

fn string_to_dollar_quoted(text: &str) -> Option<String> {
    let normalized = normalize_single_quoted_string(text)?;
    let delimiter = dollar_delimiter(&normalized)?;
    let boundary = format!("${}$", delimiter);
    Some(format!("{boundary}{normalized}{boundary}"))
}

fn normalize_single_quoted_string(text: &str) -> Option<String> {
    let body = text.strip_prefix('\'')?.strip_suffix('\'')?;
    return Some(body.replace("''", "'"));
}

fn dollar_delimiter(content: &str) -> Option<String> {
    // We can't safely transform a trailing `$` i.e., `select 'foo $'` with an
    // empty delim, because we'll  `select $$foo $$$` which isn't valid.
    if !content.contains("$$") && !content.ends_with('$') {
        return Some("".to_owned());
    }

    let mut delim = "q".to_owned();
    // don't want to just loop forever
    for idx in 0..10 {
        if !content.contains(&format!("${}$", delim)) {
            return Some(delim);
        }
        delim.push_str(&idx.to_string());
    }
    None
}

#[cfg(test)]
mod test {
    use insta::assert_snapshot;

    use crate::code_actions::test_utils::{apply_code_action, code_action_not_applicable};

    use super::rewrite_as_dollar_quoted_string;

    #[test]
    fn rewrite_string() {
        assert_snapshot!(apply_code_action(
            rewrite_as_dollar_quoted_string,
            "select 'fo$0o';"),
            @"select $$foo$$;"
        );
    }

    #[test]
    fn rewrite_string_with_single_quote() {
        assert_snapshot!(apply_code_action(
            rewrite_as_dollar_quoted_string,
            "select 'it''s$0 nice';"),
            @"select $$it's nice$$;"
        );
    }

    #[test]
    fn rewrite_string_with_dollar_signs() {
        assert_snapshot!(apply_code_action(
            rewrite_as_dollar_quoted_string,
            "select 'foo $$ ba$0r';"),
            @"select $q$foo $$ bar$q$;"
        );
    }

    #[test]
    fn rewrite_string_when_trailing_dollar() {
        assert_snapshot!(apply_code_action(
            rewrite_as_dollar_quoted_string,
            "select 'foo $'$0;"),
            @"select $q$foo $$q$;"
        );
    }

    #[test]
    fn rewrite_string_not_applicable() {
        assert!(code_action_not_applicable(
            rewrite_as_dollar_quoted_string,
            "select 1 + $0 2;"
        ));
    }

    #[test]
    fn rewrite_prefix_string_not_applicable() {
        assert!(code_action_not_applicable(
            rewrite_as_dollar_quoted_string,
            "select b'foo$0';"
        ));
    }
}
