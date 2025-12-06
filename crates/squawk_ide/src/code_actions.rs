use rowan::TextSize;
use squawk_linter::Edit;
use squawk_syntax::{
    SyntaxKind,
    ast::{self, AstNode},
};

#[derive(Debug, Clone)]
pub enum ActionKind {
    QuickFix,
    RefactorRewrite,
}

#[derive(Debug, Clone)]
pub struct CodeAction {
    pub title: String,
    pub edits: Vec<Edit>,
    pub kind: ActionKind,
}

pub fn code_actions(file: ast::SourceFile, offset: TextSize) -> Option<Vec<CodeAction>> {
    let mut actions = vec![];
    rewrite_as_regular_string(&mut actions, &file, offset);
    rewrite_as_dollar_quoted_string(&mut actions, &file, offset);
    remove_else_clause(&mut actions, &file, offset);
    Some(actions)
}

fn rewrite_as_regular_string(
    actions: &mut Vec<CodeAction>,
    file: &ast::SourceFile,
    offset: TextSize,
) -> Option<()> {
    let dollar_string = file
        .syntax()
        .token_at_offset(offset)
        .find(|token| token.kind() == SyntaxKind::DOLLAR_QUOTED_STRING)?;

    let replacement = dollar_quoted_to_string(dollar_string.text())?;
    actions.push(CodeAction {
        title: "Rewrite as regular string".to_owned(),
        edits: vec![Edit::replace(dollar_string.text_range(), replacement)],
        kind: ActionKind::RefactorRewrite,
    });

    Some(())
}

fn rewrite_as_dollar_quoted_string(
    actions: &mut Vec<CodeAction>,
    file: &ast::SourceFile,
    offset: TextSize,
) -> Option<()> {
    let string = file
        .syntax()
        .token_at_offset(offset)
        .find(|token| token.kind() == SyntaxKind::STRING)?;

    let replacement = string_to_dollar_quoted(string.text())?;
    actions.push(CodeAction {
        title: "Rewrite as dollar-quoted string".to_owned(),
        edits: vec![Edit::replace(string.text_range(), replacement)],
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

fn remove_else_clause(
    actions: &mut Vec<CodeAction>,
    file: &ast::SourceFile,
    offset: TextSize,
) -> Option<()> {
    let else_token = file
        .syntax()
        .token_at_offset(offset)
        .find(|x| x.kind() == SyntaxKind::ELSE_KW)?;
    let parent = else_token.parent()?;
    let else_clause = ast::ElseClause::cast(parent)?;

    let mut edits = vec![];
    edits.push(Edit::delete(else_clause.syntax().text_range()));
    if let Some(token) = else_token.prev_token() {
        if token.kind() == SyntaxKind::WHITESPACE {
            edits.push(Edit::delete(token.text_range()));
        }
    }

    actions.push(CodeAction {
        title: "Remove `else` clause".to_owned(),
        edits,
        kind: ActionKind::RefactorRewrite,
    });
    Some(())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_utils::fixture;
    use insta::assert_snapshot;
    use rowan::TextSize;
    use squawk_syntax::ast;

    fn apply_code_action(
        f: impl Fn(&mut Vec<CodeAction>, &ast::SourceFile, TextSize) -> Option<()>,
        sql: &str,
    ) -> String {
        let (offset, sql) = fixture(sql);
        let parse = ast::SourceFile::parse(&sql);
        assert_eq!(parse.errors(), vec![]);
        let file: ast::SourceFile = parse.tree();

        let mut actions = vec![];
        f(&mut actions, &file, offset);

        assert!(
            !actions.is_empty(),
            "We should always have actions for `apply_code_action`. If you want to ensure there are no actions, use `code_action_not_applicable` instead."
        );

        let action = &actions[0];
        let mut result = sql.clone();

        let mut edits = action.edits.clone();
        edits.sort_by_key(|e| e.text_range.start());
        check_overlap(&edits);
        edits.reverse();

        for edit in edits {
            let start: usize = edit.text_range.start().into();
            let end: usize = edit.text_range.end().into();
            let replacement = edit.text.as_deref().unwrap_or("");
            result.replace_range(start..end, replacement);
        }

        let reparse = ast::SourceFile::parse(&result);
        assert_eq!(
            reparse.errors(),
            vec![],
            "Code actions shouldn't cause syntax errors"
        );

        result
    }

    // There's an invariant where the edits can't overlap.
    // For example, if we have an edit that deletes the full `else clause` and
    // another edit that deletes the `else` keyword and they overlap, then
    // vscode doesn't surface the code action.
    fn check_overlap(edits: &[Edit]) {
        for (edit_i, edit_j) in edits.iter().zip(edits.iter().skip(1)) {
            if let Some(intersection) = edit_i.text_range.intersect(edit_j.text_range) {
                assert!(
                    intersection.is_empty(),
                    "Edit ranges must not overlap: {:?} and {:?} intersect at {:?}",
                    edit_i.text_range,
                    edit_j.text_range,
                    intersection
                );
            }
        }
    }

    fn code_action_not_applicable(
        f: impl Fn(&mut Vec<CodeAction>, &ast::SourceFile, TextSize) -> Option<()>,
        sql: &str,
    ) -> bool {
        let (offset, sql) = fixture(sql);
        let parse = ast::SourceFile::parse(&sql);
        assert_eq!(parse.errors(), vec![]);
        let file: ast::SourceFile = parse.tree();

        let mut actions = vec![];
        f(&mut actions, &file, offset);
        actions.is_empty()
    }

    #[test]
    fn remove_else_clause_() {
        assert_snapshot!(apply_code_action(
            remove_else_clause,
            "select case x when true then 1 else$0 2 end;"),
            @"select case x when true then 1 end;"
        );
    }

    #[test]
    fn remove_else_clause_before_token() {
        assert_snapshot!(apply_code_action(
            remove_else_clause,
            "select case x when true then 1 $0else 2 end;"),
            @"select case x when true then 1 end;"
        );
    }

    #[test]
    fn remove_else_clause_not_applicable() {
        assert!(code_action_not_applicable(
            remove_else_clause,
            "select case x when true then 1 else 2 end$0;"
        ));
    }

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
