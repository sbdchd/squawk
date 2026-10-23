use rowan::TextSize;
use salsa::Database as Db;
use squawk_linter::Edit;
use squawk_syntax::ast::{self, AstNode};

use super::{ActionKind, CodeAction};
use crate::{db::semantic_index, file::InFile, ir::Item, offsets::token_from_offset};

pub(super) fn add_schema(
    db: &dyn Db,
    position: InFile<TextSize>,
    actions: &mut Vec<CodeAction>,
) -> Option<()> {
    let offset = position.value;
    let token = token_from_offset(db, position)?;
    let range = token.parent_ancestors().find_map(|node| {
        if let Some(path) = ast::Path::cast(node.clone())
            && path.qualifier().is_none()
        {
            return Some(path.syntax().text_range());
        }
        if let Some(path) = ast::PathRef::cast(node.clone())
            && path.qualifier().is_none()
        {
            return Some(path.syntax().text_range());
        }
        if let Some(call_expr) = ast::CallExpr::cast(node) {
            let name_ref = call_expr.expr()?.as_name_ref()?;
            return Some(name_ref.syntax().text_range());
        }
        None
    })?;

    if !range.contains(offset) {
        return None;
    }

    let token_start = token.text_range().start();
    let index = semantic_index(db, position.file_id);
    let range = token.text_range();
    let item = index.item_at_range(position.with_value(range))?.value.item;
    let item = match item {
        Item::Reference(reference) => index.references[reference].resolution.item()?,
        item => item,
    };
    let schema = match item {
        Item::Table(table) => index.tables[table].schema,
        Item::Function(function) => index.functions[function].schema,
        Item::Type(ty) => index.types[ty].schema,
        _ => return None,
    };
    let replacement = format!("{}.", index.names.resolve(schema));

    actions.push(CodeAction {
        title: "Add schema".to_owned(),
        edits: vec![Edit::insert(replacement, token_start)],
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
    #[should_panic(expected = "We should always have actions")]
    fn add_schema_alter_table() {
        assert_snapshot!(apply_code_action(
            add_schema,
            "alter table t$0 add column c text;"),
            @"alter table public.t add column c text;"
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
    #[should_panic(expected = "We should always have actions")]
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
    #[should_panic(expected = "We should always have actions")]
    fn add_schema_before_search_path_change() {
        assert_snapshot!(apply_code_action(
            add_schema,
            "set search_path to app; select * from t$0; set search_path to other;"),
            @"set search_path to app; select * from app.t; set search_path to other;"
        );
    }

    #[test]
    #[should_panic(expected = "We should always have actions")]
    fn add_schema_after_reset() {
        assert_snapshot!(apply_code_action(
            add_schema,
            "set search_path to app; reset search_path; select * from t$0;"),
            @"set search_path to app; reset search_path; select * from public.t;"
        );
    }

    #[test]
    #[should_panic(expected = "We should always have actions")]
    fn add_schema_after_local_search_path() {
        assert_snapshot!(apply_code_action(
            add_schema,
            "set search_path to app; begin; set local search_path to other; commit; select * from t$0;"),
            @"set search_path to app; begin; set local search_path to other; commit; select * from app.t;"
        );
    }

    #[test]
    #[should_panic(expected = "We should always have actions")]
    fn add_schema_in_routine() {
        assert_snapshot!(apply_code_action(
            add_schema,
            "create function f() returns int language sql set search_path to app begin atomic select t$0(); end;"),
            @"create function f() returns int language sql set search_path to app begin atomic select app.t(); end;"
        );
    }

    #[test]
    fn add_schema_resolved_non_public_table() {
        assert_snapshot!(apply_code_action(
            add_schema,
            "create table app.t(a int); set search_path to app; select * from t$0;"),
            @"create table app.t(a int); set search_path to app; select * from app.t;"
        );
    }

    #[test]
    fn add_schema_unresolved() {
        assert!(code_action_not_applicable(
            add_schema,
            "set search_path to app; select * from t$0;"
        ));
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
