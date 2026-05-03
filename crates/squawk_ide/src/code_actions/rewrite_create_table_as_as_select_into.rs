use rowan::{TextRange, TextSize};
use salsa::Database as Db;
use squawk_linter::Edit;
use squawk_syntax::{
    SyntaxKind,
    ast::{self, AstNode},
};

use crate::{db::File, offsets::token_from_offset};

use super::{ActionKind, CodeAction};

pub(super) fn rewrite_create_table_as_as_select_into(
    db: &dyn Db,
    file: File,
    actions: &mut Vec<CodeAction>,
    offset: TextSize,
) -> Option<()> {
    let token = token_from_offset(db, file, offset)?;
    let create_table_as = token
        .parent_ancestors()
        .find_map(ast::CreateTableAs::cast)?;

    if create_table_as.if_not_exists().is_some()
        || create_table_as.using_method().is_some()
        || create_table_as.on_commit().is_some()
        || create_table_as.tablespace().is_some()
        || create_table_as.with_data().is_some()
        || create_table_as.with_no_data().is_some()
        || create_table_as.with_params().is_some()
        || create_table_as.without_oids().is_some()
    {
        return None;
    }

    // TODO: support more
    let ast::SelectVariant::Select(select) = create_table_as.query()? else {
        return None;
    };

    let delete_edit = {
        let as_token = create_table_as.as_token()?;
        let mut delete_end = as_token.text_range().end();
        if let Some(next) = as_token.next_sibling_or_token()
            && next.kind() == SyntaxKind::WHITESPACE
        {
            delete_end = next.text_range().end();
        }
        Edit::delete(TextRange::new(
            create_table_as.syntax().text_range().start(),
            delete_end,
        ))
    };

    let insert_edit = {
        let persistence_text = create_table_as
            .persistence()
            .map(|p| format!(" {}", p.syntax()))
            .unwrap_or_default();
        let into_text = format!(
            " into{} {}",
            persistence_text,
            create_table_as.path()?.syntax()
        );
        let select_end = select.select_clause()?.syntax().text_range().end();
        Edit::insert(into_text, select_end)
    };

    actions.push(CodeAction {
        title: "Rewrite as `select into`".to_owned(),
        edits: vec![delete_edit, insert_edit],
        kind: ActionKind::RefactorRewrite,
    });

    Some(())
}

#[cfg(test)]
mod test {
    use insta::assert_snapshot;

    use crate::code_actions::test_utils::{apply_code_action, code_action_not_applicable};

    use super::rewrite_create_table_as_as_select_into;

    #[test]
    fn simple() {
        assert_snapshot!(
            apply_code_action(
                rewrite_create_table_as_as_select_into,
                "create table$0 mytable as select a, b from foo;"
            ),
            @"select a, b into mytable from foo;"
        );
    }

    #[test]
    fn with_temp() {
        assert_snapshot!(
            apply_code_action(
                rewrite_create_table_as_as_select_into,
                "create temp table$0 mytable as select a, b from foo;"
            ),
            @"select a, b into temp mytable from foo;"
        );
    }

    #[test]
    fn with_temporary() {
        assert_snapshot!(
            apply_code_action(
                rewrite_create_table_as_as_select_into,
                "create temporary table$0 mytable as select a from foo;"
            ),
            @"select a into temporary mytable from foo;"
        );
    }

    #[test]
    fn qualified_name() {
        assert_snapshot!(
            apply_code_action(
                rewrite_create_table_as_as_select_into,
                "create table$0 myschema.mytable as select a from foo;"
            ),
            @"select a into myschema.mytable from foo;"
        );
    }

    #[test]
    fn with_where() {
        assert_snapshot!(
            apply_code_action(
                rewrite_create_table_as_as_select_into,
                "create table mytable$0 as select a, b from foo where a > 1;"
            ),
            @"select a, b into mytable from foo where a > 1;"
        );
    }

    #[test]
    fn with_cte() {
        assert_snapshot!(
            apply_code_action(
                rewrite_create_table_as_as_select_into,
                "create table mytable as$0 with cte as (select 1) select a from cte;"
            ),
            @"with cte as (select 1) select a into mytable from cte;"
        );
    }

    #[test]
    fn on_create_keyword() {
        assert_snapshot!(
            apply_code_action(
                rewrite_create_table_as_as_select_into,
                "creat$0e table mytable as select a from foo;"
            ),
            @"select a into mytable from foo;"
        );
    }

    #[test]
    fn not_applicable_with_if_not_exists() {
        assert!(code_action_not_applicable(
            rewrite_create_table_as_as_select_into,
            "create table if not exists$0 mytable as select a from foo;"
        ));
    }

    #[test]
    fn not_applicable_with_compound_select() {
        assert!(code_action_not_applicable(
            rewrite_create_table_as_as_select_into,
            "create table mytable as select a from foo$0 union all select b from bar;"
        ));
    }

    #[test]
    fn not_applicable_with_values() {
        assert!(code_action_not_applicable(
            rewrite_create_table_as_as_select_into,
            "create table mytable$0 as values (1, 2);"
        ));
    }

    #[test]
    fn not_applicable_with_paren_select() {
        assert!(code_action_not_applicable(
            rewrite_create_table_as_as_select_into,
            "create table mytable$0 as (select 1 a);"
        ));
    }

    #[test]
    fn not_applicable_with_table() {
        assert!(code_action_not_applicable(
            rewrite_create_table_as_as_select_into,
            "create table mytable$0 as table u;"
        ));
    }

    #[test]
    fn not_applicable_on_select_into() {
        assert!(code_action_not_applicable(
            rewrite_create_table_as_as_select_into,
            "select a int$0o mytable from foo;"
        ));
    }
}
