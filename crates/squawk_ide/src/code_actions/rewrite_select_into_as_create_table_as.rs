use rowan::{TextRange, TextSize};
use salsa::Database as Db;
use squawk_linter::Edit;
use squawk_syntax::{
    SyntaxKind,
    ast::{self, AstNode},
};

use crate::{db::File, offsets::token_from_offset};

use super::{ActionKind, CodeAction};

pub(super) fn rewrite_select_into_as_create_table_as(
    db: &dyn Db,
    file: File,
    actions: &mut Vec<CodeAction>,
    offset: TextSize,
) -> Option<()> {
    let token = token_from_offset(db, file, offset)?;
    let select_into = token.parent_ancestors().find_map(ast::SelectInto::cast)?;

    let into_clause = select_into.into_clause()?;

    // temp, unlogged, etc.
    let persistence = into_clause
        .persistence()
        .map(|x| format!("{} ", x.syntax()))
        .unwrap_or_default();
    let create_prefix = format!(
        "create {}table {} as ",
        persistence,
        into_clause.path()?.syntax()
    );
    let stmt_start = {
        let mut start = select_into.syntax().text_range().start();
        for parent in select_into.syntax().parent()?.ancestors() {
            if ast::CompoundSelect::can_cast(parent.kind())
                || ast::ParenSelect::can_cast(parent.kind())
            {
                start = parent.text_range().start();
            } else {
                break;
            }
        }
        start
    };
    let insert_prefix = Edit::insert(create_prefix, stmt_start);

    let delete_into = {
        let range = into_clause.syntax().text_range();
        let mut start = range.start();
        if let Some(prev) = into_clause.syntax().prev_sibling_or_token()
            && prev.kind() == SyntaxKind::WHITESPACE
        {
            start = prev.text_range().start();
        }
        Edit::delete(TextRange::new(start, range.end()))
    };

    actions.push(CodeAction {
        title: "Rewrite as `create table as`".to_owned(),
        edits: vec![delete_into, insert_prefix],
        kind: ActionKind::RefactorRewrite,
    });

    Some(())
}

#[cfg(test)]
mod test {
    use insta::assert_snapshot;

    use crate::code_actions::test_utils::{apply_code_action, code_action_not_applicable};

    use super::rewrite_select_into_as_create_table_as;

    #[test]
    fn simple() {
        assert_snapshot!(
            apply_code_action(
                rewrite_select_into_as_create_table_as,
                "select$0 a, b into mytable from foo;"
            ),
            @"create table mytable as select a, b from foo;"
        );
    }

    #[test]
    fn with_temp() {
        assert_snapshot!(
            apply_code_action(
                rewrite_select_into_as_create_table_as,
                "select a, b int$0o temp mytable from foo;"
            ),
            @"create temp table mytable as select a, b from foo;"
        );
    }

    #[test]
    fn with_temporary() {
        assert_snapshot!(
            apply_code_action(
                rewrite_select_into_as_create_table_as,
                "select a int$0o temporary mytable from foo;"
            ),
            @"create temporary table mytable as select a from foo;"
        );
    }

    #[test]
    fn with_table_keyword() {
        assert_snapshot!(
            apply_code_action(
                rewrite_select_into_as_create_table_as,
                "select a into$0 table mytable from foo;"
            ),
            @"create table mytable as select a from foo;"
        );
    }

    #[test]
    fn qualified_name() {
        assert_snapshot!(
            apply_code_action(
                rewrite_select_into_as_create_table_as,
                "select a into$0 myschema.mytable from foo;"
            ),
            @"create table myschema.mytable as select a from foo;"
        );
    }

    #[test]
    fn with_where() {
        assert_snapshot!(
            apply_code_action(
                rewrite_select_into_as_create_table_as,
                "select a, b into$0 mytable from foo where a > 1;"
            ),
            @"create table mytable as select a, b from foo where a > 1;"
        );
    }

    #[test]
    fn with_cte() {
        assert_snapshot!(
            apply_code_action(
                rewrite_select_into_as_create_table_as,
                "with cte as (select 1) select a int$0o mytable from cte;"
            ),
            @"create table mytable as with cte as (select 1) select a from cte;"
        );
    }

    #[test]
    fn on_into_keyword() {
        assert_snapshot!(
            apply_code_action(
                rewrite_select_into_as_create_table_as,
                "select a, b int$0o mytable from foo;"
            ),
            @"create table mytable as select a, b from foo;"
        );
    }

    #[test]
    fn rewrite_select_into_parent_as_create_table_as() {
        assert_snapshot!(
            apply_code_action(
                rewrite_select_into_as_create_table_as,
                "((select 1 b int$0o t))"
            ),
            @"create table t as ((select 1 b))"
        );
    }

    #[test]
    fn compound_paren() {
        assert_snapshot!(
            apply_code_action(
                rewrite_select_into_as_create_table_as,
                "(select 1 a int$0o t union select 2 a)"
            ),
            @"create table t as (select 1 a union select 2 a)"
        );
    }

    #[test]
    fn compound_no_paren() {
        assert_snapshot!(
            apply_code_action(
                rewrite_select_into_as_create_table_as,
                "select 1 a int$0o t union select 2 a"
            ),
            @"create table t as select 1 a union select 2 a"
        );
    }

    #[test]
    fn not_applicable_on_select() {
        assert!(code_action_not_applicable(
            rewrite_select_into_as_create_table_as,
            "sel$0ect a from foo;"
        ));
    }

    #[test]
    fn not_applicable_on_create_table_as() {
        assert!(code_action_not_applicable(
            rewrite_select_into_as_create_table_as,
            "create table$0 mytable as select a from foo;"
        ));
    }
}
