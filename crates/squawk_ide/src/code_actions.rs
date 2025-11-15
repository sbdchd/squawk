use rowan::TextSize;
use squawk_linter::Edit;
use squawk_syntax::{
    SyntaxKind,
    ast::{self, AstNode},
};

#[derive(Debug, Clone)]
pub struct CodeAction {
    pub title: String,
    pub edits: Vec<Edit>,
}

pub fn code_actions(file: ast::SourceFile, offset: TextSize) -> Option<Vec<CodeAction>> {
    let mut actions = vec![];
    remove_else_clause(&mut actions, &file, offset);
    Some(actions)
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
}
