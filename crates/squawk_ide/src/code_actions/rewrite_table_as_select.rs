use rowan::TextSize;
use salsa::Database as Db;
use squawk_syntax::ast::{self, AstNode};

use crate::{db::File, offsets::token_from_offset};

use super::{ActionKind, CodeAction};

pub(super) fn rewrite_table_as_select(
    db: &dyn Db,
    file: File,
    actions: &mut Vec<CodeAction>,
    offset: TextSize,
) -> Option<()> {
    let token = token_from_offset(db, file, offset)?;
    let table = token.parent_ancestors().find_map(ast::Table::cast)?;

    let table_name = table.relation_name()?.syntax().text();

    let replacement = format!("select * from {}", table_name);

    actions.push(CodeAction {
        title: "Rewrite as `select`".to_owned(),
        edits: vec![squawk_linter::Edit::replace(
            table.syntax().text_range(),
            replacement,
        )],
        kind: ActionKind::RefactorRewrite,
    });

    Some(())
}

#[cfg(test)]
mod test {
    use insta::assert_snapshot;

    use crate::code_actions::test_utils::{apply_code_action, code_action_not_applicable};

    use super::rewrite_table_as_select;

    #[test]
    fn rewrite_table_as_select_simple() {
        assert_snapshot!(apply_code_action(
            rewrite_table_as_select,
            "tab$0le foo;"),
            @"select * from foo;"
        );
    }

    #[test]
    fn rewrite_table_as_select_qualified() {
        assert_snapshot!(apply_code_action(
            rewrite_table_as_select,
            "ta$0ble schema.foo;"),
            @"select * from schema.foo;"
        );
    }

    #[test]
    fn rewrite_table_as_select_after_keyword() {
        assert_snapshot!(apply_code_action(
            rewrite_table_as_select,
            "table$0 bar;"),
            @"select * from bar;"
        );
    }

    #[test]
    fn rewrite_table_as_select_on_table_name() {
        assert_snapshot!(apply_code_action(
            rewrite_table_as_select,
            "table fo$0o;"),
            @"select * from foo;"
        );
    }

    #[test]
    fn rewrite_table_as_select_not_applicable() {
        assert!(code_action_not_applicable(
            rewrite_table_as_select,
            "select * from foo$0;"
        ));
    }
}
