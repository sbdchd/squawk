use rowan::TextSize;
use salsa::Database as Db;
use squawk_syntax::ast::{self, AstNode};

use super::{ActionKind, CodeAction};
use crate::{
    db::{File, bind},
    offsets::token_from_offset,
};

pub(super) fn add_schema(
    db: &dyn Db,
    file: File,
    actions: &mut Vec<CodeAction>,
    offset: TextSize,
) -> Option<()> {
    let token = token_from_offset(db, file, offset)?;
    let range = token.parent_ancestors().find_map(|node| {
        if let Some(path) = ast::Path::cast(node.clone()) {
            if path.qualifier().is_some() {
                return None;
            }
            return Some(path.syntax().text_range());
        }
        if let Some(from_item) = ast::FromItem::cast(node.clone()) {
            let name_ref = from_item.name_ref()?;
            return Some(name_ref.syntax().text_range());
        }
        if let Some(call_expr) = ast::CallExpr::cast(node) {
            let ast::Expr::NameRef(name_ref) = call_expr.expr()? else {
                return None;
            };
            return Some(name_ref.syntax().text_range());
        }
        None
    })?;

    if !range.contains(offset) {
        return None;
    }

    let position = token.text_range().start();
    let schema = bind(db, file).search_path_at(position).first()?.to_string();
    let replacement = format!("{}.", schema);

    actions.push(CodeAction {
        title: "Add schema".to_owned(),
        edits: vec![squawk_linter::Edit::insert(replacement, position)],
        kind: ActionKind::RefactorRewrite,
    });

    Some(())
}

#[cfg(test)]
mod test {
    use insta::assert_snapshot;

    use crate::code_actions::test_utils::{apply_code_action, code_action_not_applicable};

    use super::add_schema;

    #[test]
    fn add_schema_simple() {
        assert_snapshot!(apply_code_action(
            add_schema,
            "create table t$0(a text, b int);"),
            @"create table public.t(a text, b int);"
        );
    }

    #[test]
    fn add_schema_create_foreign_table() {
        assert_snapshot!(apply_code_action(
            add_schema,
            "create foreign table t$0(a text, b int) server foo;"),
            @"create foreign table public.t(a text, b int) server foo;"
        );
    }

    #[test]
    fn add_schema_create_function() {
        assert_snapshot!(apply_code_action(
            add_schema,
            "create function f$0() returns int8\n  as 'select 1'\n  language sql;"),
            @"create function public.f() returns int8
  as 'select 1'
  language sql;"
        );
    }

    #[test]
    fn add_schema_create_type() {
        assert_snapshot!(apply_code_action(
            add_schema,
            "create type t$0 as enum ();"),
            @"create type public.t as enum ();"
        );
    }

    #[test]
    fn add_schema_table_stmt() {
        assert_snapshot!(apply_code_action(
            add_schema,
            "table t$0;"),
            @"table public.t;"
        );
    }

    #[test]
    fn add_schema_select_from() {
        assert_snapshot!(apply_code_action(
            add_schema,
            "create table t(a text, b int);
        select t from t$0;"),
            @"create table t(a text, b int);
        select t from public.t;"
        );
    }

    #[test]
    fn add_schema_select_table_value() {
        // we can't insert the schema here because:
        // `select public.t from t` isn't valid
        assert!(code_action_not_applicable(
            add_schema,
            "create table t(a text, b int);
        select t$0 from t;"
        ));
    }

    #[test]
    fn add_schema_select_unqualified_column() {
        // not applicable since we don't have the table name set
        // we'll have another quick action to insert table names
        assert!(code_action_not_applicable(
            add_schema,
            "create table t(a text, b int);
        select a$0 from t;"
        ));
    }

    #[test]
    fn add_schema_select_qualified_column() {
        // not valid because we haven't specified the schema on the table name
        // `select public.t.c from t` isn't valid sql
        assert!(code_action_not_applicable(
            add_schema,
            "create table t(c text);
        select t$0.c from t;"
        ));
    }

    #[test]
    fn add_schema_with_search_path() {
        assert_snapshot!(
            apply_code_action(
                add_schema,
                "
set search_path to myschema;
create table t$0(a text, b int);"
            ),
            @"
set search_path to myschema;
create table myschema.t(a text, b int);"
        );
    }

    #[test]
    fn add_schema_not_applicable_with_schema() {
        assert!(code_action_not_applicable(
            add_schema,
            "create table myschema.t$0(a text, b int);"
        ));
    }

    #[test]
    fn add_schema_function_call() {
        assert_snapshot!(apply_code_action(
            add_schema,
            "
create function f() returns int8
  as 'select 1'
  language sql;

select f$0();"),
            @"
create function f() returns int8
  as 'select 1'
  language sql;

select public.f();"
        );
    }

    #[test]
    fn add_schema_function_call_not_applicable_with_schema() {
        assert!(code_action_not_applicable(
            add_schema,
            "
create function f() returns int8 as 'select 1' language sql;
select myschema.f$0();"
        ));
    }
}
