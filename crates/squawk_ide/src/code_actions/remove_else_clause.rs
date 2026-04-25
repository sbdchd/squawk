use rowan::TextSize;
use salsa::Database as Db;
use squawk_linter::Edit;
use squawk_syntax::{
    SyntaxKind,
    ast::{self, AstNode},
};

use crate::db::{File, parse};

use super::{ActionKind, CodeAction};

pub(super) fn remove_else_clause(
    db: &dyn Db,
    file: File,
    actions: &mut Vec<CodeAction>,
    offset: TextSize,
) -> Option<()> {
    let source_file = parse(db, file).tree();

    let else_token = source_file
        .syntax()
        .token_at_offset(offset)
        .find(|x| x.kind() == SyntaxKind::ELSE_KW)?;
    let parent = else_token.parent()?;
    let else_clause = ast::ElseClause::cast(parent)?;

    let mut edits = vec![];
    edits.push(Edit::delete(else_clause.syntax().text_range()));
    if let Some(token) = else_token.prev_token()
        && token.kind() == SyntaxKind::WHITESPACE
    {
        edits.push(Edit::delete(token.text_range()));
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
    use insta::assert_snapshot;

    use crate::code_actions::test_utils::{apply_code_action, code_action_not_applicable};

    use super::remove_else_clause;

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
            "select case x when true then 1 e$0lse 2 end;"),
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
