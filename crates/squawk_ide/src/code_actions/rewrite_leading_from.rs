use rowan::{TextRange, TextSize};
use salsa::Database as Db;
use squawk_syntax::{
    SyntaxKind,
    ast::{self, AstNode},
};

use crate::{db::File, offsets::token_from_offset};

use super::{ActionKind, CodeAction};

pub(super) fn rewrite_leading_from(
    db: &dyn Db,
    file: File,
    actions: &mut Vec<CodeAction>,
    offset: TextSize,
) -> Option<()> {
    let token = token_from_offset(db, file, offset)?;
    let select = token.parent_ancestors().find_map(ast::Select::cast)?;

    let from_clause = select.from_clause()?;
    let select_clause = select.select_clause()?;

    if from_clause.syntax().text_range().start() >= select_clause.syntax().text_range().start() {
        return None;
    }

    let select_text = select_clause.syntax().text().to_string();

    let mut delete_start = select_clause.syntax().text_range().start();
    if let Some(prev) = select_clause.syntax().prev_sibling_or_token()
        && prev.kind() == SyntaxKind::WHITESPACE
    {
        delete_start = prev.text_range().start();
    }
    let select_with_ws = TextRange::new(delete_start, select_clause.syntax().text_range().end());

    actions.push(CodeAction {
        title: "Swap `from` and `select` clauses".to_owned(),
        edits: vec![
            squawk_linter::Edit::delete(select_with_ws),
            squawk_linter::Edit::insert(
                format!("{} ", select_text),
                from_clause.syntax().text_range().start(),
            ),
        ],
        kind: ActionKind::QuickFix,
    });

    Some(())
}

#[cfg(test)]
mod test {
    use insta::assert_snapshot;

    use crate::code_actions::test_utils::{apply_code_action, code_action_not_applicable};

    use super::rewrite_leading_from;

    #[test]
    fn rewrite_leading_from_simple() {
        assert_snapshot!(apply_code_action(
            rewrite_leading_from,
            "from$0 t select c;"),
            @"select c from t;"
        );
    }

    #[test]
    fn rewrite_leading_from_multiple_cols() {
        assert_snapshot!(apply_code_action(
            rewrite_leading_from,
            "from$0 t select a, b;"),
            @"select a, b from t;"
        );
    }

    #[test]
    fn rewrite_leading_from_with_where() {
        assert_snapshot!(apply_code_action(
            rewrite_leading_from,
            "from$0 t select c where x = 1;"),
            @"select c from t where x = 1;"
        );
    }

    #[test]
    fn rewrite_leading_from_on_select() {
        assert_snapshot!(apply_code_action(
            rewrite_leading_from,
            "from t sel$0ect c;"),
            @"select c from t;"
        );
    }

    #[test]
    fn rewrite_leading_from_not_applicable_normal() {
        assert!(code_action_not_applicable(
            rewrite_leading_from,
            "sel$0ect c from t;"
        ));
    }
}
