use rowan::TextSize;
use salsa::Database as Db;
use squawk_syntax::ast::{self, AstNode};

use crate::{db::File, offsets::token_from_offset};

use super::{ActionKind, CodeAction};

pub(super) fn rewrite_from(
    db: &dyn Db,
    file: File,
    actions: &mut Vec<CodeAction>,
    offset: TextSize,
) -> Option<()> {
    let token = token_from_offset(db, file, offset)?;
    let select = token.parent_ancestors().find_map(ast::Select::cast)?;

    if select.select_clause().is_some() {
        return None;
    }

    select.from_clause()?;

    actions.push(CodeAction {
        title: "Insert leading `select *`".to_owned(),
        edits: vec![squawk_linter::Edit::insert(
            "select * ".to_owned(),
            select.syntax().text_range().start(),
        )],
        kind: ActionKind::QuickFix,
    });

    Some(())
}

#[cfg(test)]
mod test {
    use insta::assert_snapshot;

    use crate::code_actions::test_utils::{
        apply_code_action, code_action_not_applicable, code_action_not_applicable_with_errors,
    };

    use super::rewrite_from;

    #[test]
    fn rewrite_from_simple() {
        assert_snapshot!(apply_code_action(
            rewrite_from,
            "from$0 t;"),
            @"select * from t;"
        );
    }

    #[test]
    fn rewrite_from_qualified() {
        assert_snapshot!(apply_code_action(
            rewrite_from,
            "from$0 s.t;"),
            @"select * from s.t;"
        );
    }

    #[test]
    fn rewrite_from_on_name() {
        assert_snapshot!(apply_code_action(
            rewrite_from,
            "from t$0;"),
            @"select * from t;"
        );
    }

    #[test]
    fn rewrite_from_not_applicable_with_select() {
        assert!(code_action_not_applicable_with_errors(
            rewrite_from,
            "from$0 t select c;"
        ));
    }

    #[test]
    fn rewrite_from_not_applicable_on_normal_select() {
        assert!(code_action_not_applicable(
            rewrite_from,
            "select * from$0 t;"
        ));
    }
}
