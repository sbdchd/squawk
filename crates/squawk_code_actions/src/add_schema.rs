use squawk_diagnostics::Edit;
use squawk_rowan::{TextRange, TextSize};
use squawk_sema::{file::InFile, ir::Item};
use squawk_syntax::ast::{self, AstNode};

use super::{ActionKind, CodeAction};
use squawk_ide_db::offsets::token_from_offset;
use squawk_ide_db::{Db, semantic_index};

pub(super) fn add_schema(
    db: &dyn Db,
    position: InFile<TextSize>,
    actions: &mut Vec<CodeAction>,
) -> Option<()> {
    let offset = position.value;
    let token = token_from_offset(db, position)?;

    if let Some(ty) = token.parent_ancestors().find_map(ast::Type::cast)
        && let Some(edit) = builtin_type_edit(&ty)
    {
        if !edit.range.contains(offset) {
            return None;
        }
        actions.push(CodeAction {
            title: "Add schema".to_owned(),
            edits: vec![Edit::replace(edit.range, edit.text)],
            kind: ActionKind::RefactorRewrite,
        });
        return Some(());
    }

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

struct BuiltinTypeEdit {
    range: TextRange,
    text: String,
}

fn builtin_type_edit(ty: &ast::Type) -> Option<BuiltinTypeEdit> {
    let name = ty.builtin_type_name()?;
    let modifier = match ty {
        ast::Type::ArrayType(_) | ast::Type::SetofType(_) => return None,
        ast::Type::PathType(path_type) => {
            let segment = path_type.path_ref()?.unqualified_segment()?;
            let segment_range = segment.syntax().text_range();
            let range = match (segment.text().as_ref(), path_type.arg_list()) {
                ("float", Some(arg_list)) => segment_range.cover(arg_list.syntax().text_range()),
                _ => segment_range,
            };
            return Some(BuiltinTypeEdit {
                range,
                text: format!("pg_catalog.{name}"),
            });
        }
        ast::Type::BitType(bit) => bit
            .arg_list()
            .map_or_else(|| "(1)".to_owned(), |args| args.syntax().to_string()),
        ast::Type::CharacterType(character) => character
            .arg_list()
            .map_or_else(|| "(1)".to_owned(), |args| args.syntax().to_string()),
        ast::Type::BitVaryingType(bit) => bit
            .arg_list()
            .map_or_else(String::new, |args| args.syntax().to_string()),
        ast::Type::VarcharType(varchar) => varchar
            .arg_list()
            .map_or_else(String::new, |args| args.syntax().to_string()),
        ast::Type::DoubleType(_) => String::new(),
        ast::Type::IntervalType(interval) => {
            if interval.interval_qualifier().is_some() || interval.literal().is_some() {
                return None;
            }
            String::new()
        }
        ast::Type::TimeType(time) => time
            .literal()
            .map_or_else(String::new, |literal| format!("({})", literal.syntax())),
        ast::Type::TimestampType(timestamp) => timestamp
            .literal()
            .map_or_else(String::new, |literal| format!("({})", literal.syntax())),
    };
    Some(BuiltinTypeEdit {
        range: ty.syntax().text_range(),
        text: format!("pg_catalog.{name}{modifier}"),
    })
}

#[cfg(test)]
mod test {
    use insta::assert_snapshot;

    use crate::test_utils::{apply_code_action, code_action_not_applicable};

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
    fn add_schema_alter_table() {
        assert_snapshot!(
            apply_code_action(
                add_schema,
                "
create table t ();
alter table t$0 add column c text;"
            ),
            @"
        create table t ();
        alter table public.t add column c text;
        "
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
            @"
        create function public.f() returns int8
          as 'select 1'
          language sql;
        "
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
        assert_snapshot!(
            apply_code_action(
                add_schema,
                "
create table t ();
table t$0;"
            ),
            @"
        create table t ();
        table public.t;
        "
        );
    }

    #[test]
    fn add_schema_select_from() {
        assert_snapshot!(apply_code_action(
            add_schema,
            "create table t(a text, b int);
        select t from t$0;"),
            @"
        create table t(a text, b int);
                select t from public.t;
        "
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
        create table myschema.t(a text, b int);
        "
        );
    }

    #[test]
    fn add_schema_before_search_path_change() {
        assert_snapshot!(
            apply_code_action(
                add_schema,
                "
create table app.t ();
create table other.t ();
set search_path to app;
select * from t$0;
set search_path to other;"
            ),
            @"
        create table app.t ();
        create table other.t ();
        set search_path to app;
        select * from app.t;
        set search_path to other;
        "
        );
    }

    #[test]
    fn add_schema_after_reset() {
        assert_snapshot!(
            apply_code_action(
                add_schema,
                "
create table app.t ();
create table public.t ();
set search_path to app;
reset search_path;
select * from t$0;"
            ),
            @"
        create table app.t ();
        create table public.t ();
        set search_path to app;
        reset search_path;
        select * from public.t;
        "
        );
    }

    #[test]
    fn add_schema_after_local_search_path() {
        assert_snapshot!(
            apply_code_action(
                add_schema,
                "
create table app.t ();
create table other.t ();
set search_path to app;
begin;
set local search_path to other;
commit;
select * from t$0;"
            ),
            @"
        create table app.t ();
        create table other.t ();
        set search_path to app;
        begin;
        set local search_path to other;
        commit;
        select * from app.t;
        "
        );
    }

    #[test]
    fn add_schema_in_routine() {
        assert_snapshot!(
            apply_code_action(
                add_schema,
                "
create function app.t() returns int
  language sql
  return 1;
create function public.t() returns int
  language sql
  return 1;
create function f() returns int
  language sql
  set search_path to app
  begin atomic
    select t$0();
  end;"
            ),
            @"
        create function app.t() returns int
          language sql
          return 1;
        create function public.t() returns int
          language sql
          return 1;
        create function f() returns int
          language sql
          set search_path to app
          begin atomic
            select app.t();
          end;
        "
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

        select public.f();
        "
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

    #[test]
    fn add_schema_bigint() {
        assert_snapshot!(apply_code_action(
            add_schema,
            "select 1::bigint$0;"),
            @"select 1::pg_catalog.int8;"
        );
    }

    #[test]
    fn add_schema_bigint_array() {
        assert_snapshot!(apply_code_action(
            add_schema,
            "select 1::bigint$0[];"),
            @"select 1::pg_catalog.int8[];"
        );
    }

    #[test]
    fn add_schema_bigint_literal() {
        assert_snapshot!(apply_code_action(
            add_schema,
            "select bigint$0 '1';"),
            @"select pg_catalog.int8 '1';"
        );
    }

    #[test]
    fn add_schema_int_column() {
        assert_snapshot!(apply_code_action(
            add_schema,
            "create table t (a int$0);"),
            @"create table t (a pg_catalog.int4);"
        );
    }

    #[test]
    fn add_schema_numeric_with_args() {
        assert_snapshot!(apply_code_action(
            add_schema,
            "select 1::numeric$0(10, 2);"),
            @"select 1::pg_catalog.numeric(10, 2);"
        );
    }

    #[test]
    fn add_schema_float_with_precision() {
        assert_snapshot!(apply_code_action(
            add_schema,
            "select 1::float$0(10);"),
            @"select 1::pg_catalog.float4;"
        );
    }

    #[test]
    fn add_schema_double_precision() {
        assert_snapshot!(apply_code_action(
            add_schema,
            "select 1::double$0 precision;"),
            @"select 1::pg_catalog.float8;"
        );
    }

    #[test]
    fn add_schema_varchar_with_args() {
        assert_snapshot!(apply_code_action(
            add_schema,
            "select 'a'::character$0 varying(10);"),
            @"select 'a'::pg_catalog.varchar(10);"
        );
    }

    #[test]
    fn add_schema_char() {
        assert_snapshot!(apply_code_action(
            add_schema,
            "select 'a'::char$0;"),
            @"select 'a'::pg_catalog.bpchar(1);"
        );
    }

    #[test]
    fn add_schema_bit() {
        assert_snapshot!(apply_code_action(
            add_schema,
            "select '1'::bit$0;"),
            @"select '1'::pg_catalog.bit(1);"
        );
    }

    #[test]
    fn add_schema_timestamptz() {
        assert_snapshot!(apply_code_action(
            add_schema,
            "select now()::timestamp$0(3) with time zone;"),
            @"select now()::pg_catalog.timestamptz(3);"
        );
    }

    #[test]
    fn add_schema_interval() {
        assert_snapshot!(apply_code_action(
            add_schema,
            "select '1 day'::interval$0;"),
            @"select '1 day'::pg_catalog.interval;"
        );
    }

    #[test]
    fn add_schema_setof_double_precision() {
        assert_snapshot!(apply_code_action(
            add_schema,
            "create function f() returns setof double$0 precision
  language sql
  return 1;"),
            @"
        create function f() returns setof pg_catalog.float8
          language sql
          return 1;
        "
        );
    }

    #[test]
    fn add_schema_interval_with_fields_not_applicable() {
        assert!(code_action_not_applicable(
            add_schema,
            "select '1'::interval$0 year to month;"
        ));
    }
}
