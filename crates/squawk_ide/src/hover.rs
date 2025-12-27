use crate::binder;
use crate::offsets::token_from_offset;
use crate::resolve;
use rowan::TextSize;
use squawk_syntax::ast::{self, AstNode};

pub fn hover(file: &ast::SourceFile, offset: TextSize) -> Option<String> {
    let token = token_from_offset(file, offset)?;
    let parent = token.parent()?;

    let binder = binder::bind(file);

    if let Some(name_ref) = ast::NameRef::cast(parent.clone()) {
        if is_column_ref(&name_ref) {
            return hover_column(file, &name_ref, &binder);
        }

        if is_table_ref(&name_ref) {
            return hover_table(file, &name_ref, &binder);
        }

        if is_index_ref(&name_ref) {
            return hover_index(file, &name_ref, &binder);
        }

        if is_function_ref(&name_ref) {
            return hover_function(file, &name_ref, &binder);
        }

        if is_select_function_call(&name_ref) {
            return hover_function(file, &name_ref, &binder);
        }
    }

    if let Some(name) = ast::Name::cast(parent) {
        if let Some(column) = name.syntax().parent().and_then(ast::Column::cast)
            && let Some(create_table) = column.syntax().ancestors().find_map(ast::CreateTable::cast)
        {
            return hover_column_definition(&create_table, &column, &binder);
        }

        if let Some(create_table) = name.syntax().ancestors().find_map(ast::CreateTable::cast) {
            return format_create_table(&create_table, &binder);
        }

        if let Some(create_index) = name.syntax().ancestors().find_map(ast::CreateIndex::cast) {
            return format_create_index(&create_index, &binder);
        }

        if let Some(create_function) = name
            .syntax()
            .ancestors()
            .find_map(ast::CreateFunction::cast)
        {
            return format_create_function(&create_function, &binder);
        }
    }

    None
}

fn hover_column(
    file: &ast::SourceFile,
    name_ref: &ast::NameRef,
    binder: &binder::Binder,
) -> Option<String> {
    let column_name = name_ref.syntax().text().to_string();

    let create_index = name_ref
        .syntax()
        .ancestors()
        .find_map(ast::CreateIndex::cast)?;

    let relation_name = create_index.relation_name()?;
    let path = relation_name.path()?;

    let (schema, table_name) = resolve::resolve_table_info(binder, &path)?;

    let column_ptr = resolve::resolve_name_ref(binder, name_ref)?;

    let root = file.syntax();
    let column_name_node = column_ptr.to_node(root);

    let column = column_name_node.ancestors().find_map(ast::Column::cast)?;

    let ty = column.ty()?;

    Some(format!(
        "{schema}.{table_name}.{column_name} {}",
        ty.syntax().text()
    ))
}

fn hover_column_definition(
    create_table: &ast::CreateTable,
    column: &ast::Column,
    binder: &binder::Binder,
) -> Option<String> {
    let column_name = column.name()?.syntax().text().to_string();
    let ty = column.ty()?;
    let path = create_table.path()?;
    let table_name = path.segment()?.name()?.syntax().text().to_string();

    let schema = if let Some(qualifier) = path.qualifier() {
        qualifier.syntax().text().to_string()
    } else if let Some(schema) = table_schema(create_table, binder) {
        schema
    } else {
        return Some(format!(
            "{}.{} {}",
            table_name,
            column_name,
            ty.syntax().text()
        ));
    };

    Some(format!(
        "{}.{}.{} {}",
        schema,
        table_name,
        column_name,
        ty.syntax().text()
    ))
}

fn hover_table(
    file: &ast::SourceFile,
    name_ref: &ast::NameRef,
    binder: &binder::Binder,
) -> Option<String> {
    let table_ptr = resolve::resolve_name_ref(binder, name_ref)?;

    let root = file.syntax();
    let table_name_node = table_ptr.to_node(root);

    let create_table = table_name_node
        .ancestors()
        .find_map(ast::CreateTable::cast)?;

    format_create_table(&create_table, binder)
}

fn hover_index(
    file: &ast::SourceFile,
    name_ref: &ast::NameRef,
    binder: &binder::Binder,
) -> Option<String> {
    let index_ptr = resolve::resolve_name_ref(binder, name_ref)?;

    let root = file.syntax();
    let index_name_node = index_ptr.to_node(root);

    let create_index = index_name_node
        .ancestors()
        .find_map(ast::CreateIndex::cast)?;

    format_create_index(&create_index, binder)
}

// Insert inferred schema into the create table hover info
fn format_create_table(create_table: &ast::CreateTable, binder: &binder::Binder) -> Option<String> {
    let path = create_table.path()?;
    let mut text = create_table.syntax().text().to_string();

    if path.qualifier().is_some() {
        return Some(text);
    }

    let Some(schema) = table_schema(create_table, binder) else {
        return Some(text);
    };

    let Some(offset) = table_name_offset(create_table, &path) else {
        return Some(text);
    };

    text.insert_str(offset, &format!("{}.", schema));
    Some(text)
}

fn table_schema(create_table: &ast::CreateTable, binder: &binder::Binder) -> Option<String> {
    let is_temp = create_table.temp_token().is_some() || create_table.temporary_token().is_some();
    if is_temp {
        return Some("pg_temp".to_string());
    }

    let position = create_table.syntax().text_range().start();
    let search_path = binder.search_path_at(position);
    search_path.first().map(|s| s.to_string())
}

fn table_name_offset(create_table: &ast::CreateTable, path: &ast::Path) -> Option<usize> {
    let segment = path.segment()?;
    let name = segment.name()?;
    let name_start = name.syntax().text_range().start();
    let create_table_start = create_table.syntax().text_range().start();
    Some((name_start - create_table_start).into())
}

// Insert inferred schema for index name and table name
fn format_create_index(create_index: &ast::CreateIndex, binder: &binder::Binder) -> Option<String> {
    let mut text = create_index.syntax().text().to_string();
    let create_index_start = create_index.syntax().text_range().start();

    let Some(index_schema_str) = index_schema(create_index, binder) else {
        return Some(text);
    };

    let mut insertions = vec![];

    if let Some(name) = create_index.name() {
        let has_schema = name
            .syntax()
            .parent()
            .map(|p| ast::Path::can_cast(p.kind()))
            .unwrap_or(false);

        if !has_schema {
            let name_start = name.syntax().text_range().start();
            let offset: usize = (name_start - create_index_start).into();
            insertions.push((offset, format!("{}.", index_schema_str)));
        }
    }

    if let Some(relation_name) = create_index.relation_name()
        && let Some(path) = relation_name.path()
        && path.qualifier().is_none()
    {
        let (table_schema, _) = resolve::resolve_table_info(binder, &path)?;
        let segment = path.segment()?;
        let name_ref = segment.name_ref()?;
        let table_name_start = name_ref.syntax().text_range().start();
        let offset: usize = (table_name_start - create_index_start).into();
        insertions.push((offset, format!("{}.", table_schema)));
    }

    insertions.sort_by(|a, b| b.0.cmp(&a.0));
    for (offset, schema_str) in insertions {
        text.insert_str(offset, &schema_str);
    }

    Some(text)
}

fn index_schema(create_index: &ast::CreateIndex, binder: &binder::Binder) -> Option<String> {
    let position = create_index.syntax().text_range().start();
    let search_path = binder.search_path_at(position);
    search_path.first().map(|s| s.to_string())
}

fn is_column_ref(name_ref: &ast::NameRef) -> bool {
    let mut in_partition_item = false;

    for ancestor in name_ref.syntax().ancestors() {
        if ast::PartitionItem::can_cast(ancestor.kind()) {
            in_partition_item = true;
        }
        if ast::CreateIndex::can_cast(ancestor.kind()) {
            return in_partition_item;
        }
    }
    false
}

fn is_table_ref(name_ref: &ast::NameRef) -> bool {
    let mut in_partition_item = false;

    for ancestor in name_ref.syntax().ancestors() {
        if ast::DropTable::can_cast(ancestor.kind()) {
            return true;
        }
        if ast::Table::can_cast(ancestor.kind()) {
            return true;
        }
        if ast::DropIndex::can_cast(ancestor.kind()) {
            return false;
        }
        if ast::PartitionItem::can_cast(ancestor.kind()) {
            in_partition_item = true;
        }
        if ast::CreateIndex::can_cast(ancestor.kind()) {
            return !in_partition_item;
        }
    }
    false
}

fn is_index_ref(name_ref: &ast::NameRef) -> bool {
    for ancestor in name_ref.syntax().ancestors() {
        if ast::DropIndex::can_cast(ancestor.kind()) {
            return true;
        }
    }
    false
}

fn is_function_ref(name_ref: &ast::NameRef) -> bool {
    for ancestor in name_ref.syntax().ancestors() {
        if ast::DropFunction::can_cast(ancestor.kind()) {
            return true;
        }
    }
    false
}

fn is_select_function_call(name_ref: &ast::NameRef) -> bool {
    let mut in_call_expr = false;

    for ancestor in name_ref.syntax().ancestors() {
        if ast::CallExpr::can_cast(ancestor.kind()) {
            in_call_expr = true;
        }
        if ast::Select::can_cast(ancestor.kind()) && in_call_expr {
            return true;
        }
    }
    false
}

fn hover_function(
    file: &ast::SourceFile,
    name_ref: &ast::NameRef,
    binder: &binder::Binder,
) -> Option<String> {
    let function_ptr = resolve::resolve_name_ref(binder, name_ref)?;

    let root = file.syntax();
    let function_name_node = function_ptr.to_node(root);

    let create_function = function_name_node
        .ancestors()
        .find_map(ast::CreateFunction::cast)?;

    format_create_function(&create_function, binder)
}

fn format_create_function(
    create_function: &ast::CreateFunction,
    binder: &binder::Binder,
) -> Option<String> {
    let path = create_function.path()?;
    let segment = path.segment()?;
    let name = segment.name()?;
    let function_name = name.syntax().text().to_string();

    let schema = if let Some(qualifier) = path.qualifier() {
        qualifier.syntax().text().to_string()
    } else {
        function_schema(create_function, binder)?
    };

    let param_list = create_function.param_list()?;
    let params = param_list.syntax().text().to_string();

    let ret_type = create_function.ret_type()?;
    let return_type = ret_type.syntax().text().to_string();

    Some(format!(
        "function {}.{}{} {}",
        schema, function_name, params, return_type
    ))
}

fn function_schema(
    create_function: &ast::CreateFunction,
    binder: &binder::Binder,
) -> Option<String> {
    let position = create_function.syntax().text_range().start();
    let search_path = binder.search_path_at(position);
    search_path.first().map(|s| s.to_string())
}

#[cfg(test)]
mod test {
    use crate::hover::hover;
    use crate::test_utils::fixture;
    use annotate_snippets::{AnnotationKind, Level, Renderer, Snippet, renderer::DecorStyle};
    use insta::assert_snapshot;
    use squawk_syntax::ast;

    #[track_caller]
    fn check_hover(sql: &str) -> String {
        check_hover_(sql).expect("should find hover information")
    }

    #[track_caller]
    fn check_hover_(sql: &str) -> Option<String> {
        let (mut offset, sql) = fixture(sql);
        offset = offset.checked_sub(1.into()).unwrap_or_default();
        let parse = ast::SourceFile::parse(&sql);
        assert_eq!(parse.errors(), vec![]);
        let file: ast::SourceFile = parse.tree();

        if let Some(type_info) = hover(&file, offset) {
            let offset_usize: usize = offset.into();
            let title = format!("hover: {}", type_info);
            let group = Level::INFO.primary_title(&title).element(
                Snippet::source(&sql).fold(true).annotation(
                    AnnotationKind::Context
                        .span(offset_usize..offset_usize + 1)
                        .label("hover"),
                ),
            );
            let renderer = Renderer::plain().decor_style(DecorStyle::Unicode);
            return Some(
                renderer
                    .render(&[group])
                    .to_string()
                    // neater
                    .replace("info: hover:", "hover:"),
            );
        }
        None
    }

    #[test]
    fn hover_column_in_create_index() {
        assert_snapshot!(check_hover("
create table users(id int, email text);
create index idx_email on users(email$0);
"), @r"
        hover: public.users.email text
          ╭▸ 
        3 │ create index idx_email on users(email);
          ╰╴                                    ─ hover
        ");
    }

    #[test]
    fn hover_column_int_type() {
        assert_snapshot!(check_hover("
create table users(id int, email text);
create index idx_id on users(id$0);
"), @r"
        hover: public.users.id int
          ╭▸ 
        3 │ create index idx_id on users(id);
          ╰╴                              ─ hover
        ");
    }

    #[test]
    fn hover_column_with_schema() {
        assert_snapshot!(check_hover("
create table public.users(id int, email text);
create index idx_email on public.users(email$0);
"), @r"
        hover: public.users.email text
          ╭▸ 
        3 │ create index idx_email on public.users(email);
          ╰╴                                           ─ hover
        ");
    }

    #[test]
    fn hover_column_temp_table() {
        assert_snapshot!(check_hover("
create temp table users(id int, email text);
create index idx_email on users(email$0);
"), @r"
        hover: pg_temp.users.email text
          ╭▸ 
        3 │ create index idx_email on users(email);
          ╰╴                                    ─ hover
        ");
    }

    #[test]
    fn hover_column_multiple_columns() {
        assert_snapshot!(check_hover("
create table users(id int, email text, name varchar(100));
create index idx_users on users(id, email$0, name);
"), @r"
        hover: public.users.email text
          ╭▸ 
        3 │ create index idx_users on users(id, email, name);
          ╰╴                                        ─ hover
        ");
    }

    #[test]
    fn hover_column_varchar() {
        assert_snapshot!(check_hover("
create table users(id int, name varchar(100));
create index idx_name on users(name$0);
"), @r"
        hover: public.users.name varchar(100)
          ╭▸ 
        3 │ create index idx_name on users(name);
          ╰╴                                  ─ hover
        ");
    }

    #[test]
    fn hover_column_bigint() {
        assert_snapshot!(check_hover("
create table metrics(value bigint);
create index idx_value on metrics(value$0);
"), @r"
        hover: public.metrics.value bigint
          ╭▸ 
        3 │ create index idx_value on metrics(value);
          ╰╴                                      ─ hover
        ");
    }

    #[test]
    fn hover_column_timestamp() {
        assert_snapshot!(check_hover("
create table events(created_at timestamp with time zone);
create index idx_created on events(created_at$0);
"), @r"
        hover: public.events.created_at timestamp with time zone
          ╭▸ 
        3 │ create index idx_created on events(created_at);
          ╰╴                                            ─ hover
        ");
    }

    #[test]
    fn hover_column_with_search_path() {
        assert_snapshot!(check_hover(r#"
set search_path to myschema;
create table myschema.users(id int, email text);
create index idx_email on users(email$0);
"#), @r"
        hover: myschema.users.email text
          ╭▸ 
        4 │ create index idx_email on users(email);
          ╰╴                                    ─ hover
        ");
    }

    #[test]
    fn hover_column_explicit_schema_overrides_search_path() {
        assert_snapshot!(check_hover(r#"
set search_path to myschema;
create table public.users(id int, email text);
create table myschema.users(value bigint);
create index idx_email on public.users(email$0);
"#), @r"
        hover: public.users.email text
          ╭▸ 
        5 │ create index idx_email on public.users(email);
          ╰╴                                           ─ hover
        ");
    }

    #[test]
    fn hover_on_table_name() {
        assert_snapshot!(check_hover("
create table t(id int);
create index idx on t$0(id);
"), @r"
        hover: create table public.t(id int)
          ╭▸ 
        3 │ create index idx on t(id);
          ╰╴                    ─ hover
        ");
    }

    #[test]
    fn hover_on_index_name_in_create() {
        assert_snapshot!(check_hover("
create table users(id int);
create index idx$0 on users(id);
"), @r"
        hover: create index public.idx on public.users(id)
          ╭▸ 
        3 │ create index idx on users(id);
          ╰╴               ─ hover
        ");
    }

    #[test]
    fn hover_table_in_create_index() {
        assert_snapshot!(check_hover("
create table users(id int, email text);
create index idx_email on users$0(email);
"), @r"
        hover: create table public.users(id int, email text)
          ╭▸ 
        3 │ create index idx_email on users(email);
          ╰╴                              ─ hover
        ");
    }

    #[test]
    fn hover_table_with_schema() {
        assert_snapshot!(check_hover("
create table public.users(id int, email text);
create index idx on public.users$0(id);
"), @r"
        hover: create table public.users(id int, email text)
          ╭▸ 
        3 │ create index idx on public.users(id);
          ╰╴                               ─ hover
        ");
    }

    #[test]
    fn hover_table_temp() {
        assert_snapshot!(check_hover("
create temp table users(id int, email text);
create index idx on users$0(id);
"), @r"
        hover: create temp table pg_temp.users(id int, email text)
          ╭▸ 
        3 │ create index idx on users(id);
          ╰╴                        ─ hover
        ");
    }

    #[test]
    fn hover_table_multiline() {
        assert_snapshot!(check_hover("
create table users(
    id int,
    email text,
    name varchar(100)
);
create index idx on users$0(id);
"), @r"
        hover: create table public.users(
                  id int,
                  email text,
                  name varchar(100)
              )
          ╭▸ 
        7 │ create index idx on users(id);
          ╰╴                        ─ hover
        ");
    }

    #[test]
    fn hover_table_with_search_path() {
        assert_snapshot!(check_hover(r#"
set search_path to myschema;
create table users(id int, email text);
create index idx on users$0(id);
"#), @r"
        hover: create table myschema.users(id int, email text)
          ╭▸ 
        4 │ create index idx on users(id);
          ╰╴                        ─ hover
        ");
    }

    #[test]
    fn hover_table_search_path_at_definition() {
        assert_snapshot!(check_hover(r#"
set search_path to myschema;
create table users(id int, email text);
set search_path to myschema, otherschema;
create index idx on users$0(id);
"#), @r"
        hover: create table myschema.users(id int, email text)
          ╭▸ 
        5 │ create index idx on users(id);
          ╰╴                        ─ hover
        ");
    }

    #[test]
    fn hover_on_create_table_definition() {
        assert_snapshot!(check_hover("
create table t$0(x bigint);
"), @r"
        hover: create table public.t(x bigint)
          ╭▸ 
        2 │ create table t(x bigint);
          ╰╴             ─ hover
        ");
    }

    #[test]
    fn hover_on_create_table_definition_with_schema() {
        assert_snapshot!(check_hover("
create table myschema.users$0(id int);
"), @r"
        hover: create table myschema.users(id int)
          ╭▸ 
        2 │ create table myschema.users(id int);
          ╰╴                          ─ hover
        ");
    }

    #[test]
    fn hover_on_create_temp_table_definition() {
        assert_snapshot!(check_hover("
create temp table t$0(x bigint);
"), @r"
        hover: create temp table pg_temp.t(x bigint)
          ╭▸ 
        2 │ create temp table t(x bigint);
          ╰╴                  ─ hover
        ");
    }

    #[test]
    fn hover_on_column_in_create_table() {
        assert_snapshot!(check_hover("
create table t(id$0 int);
"), @r"
        hover: public.t.id int
          ╭▸ 
        2 │ create table t(id int);
          ╰╴                ─ hover
        ");
    }

    #[test]
    fn hover_on_column_in_create_table_with_schema() {
        assert_snapshot!(check_hover("
create table myschema.users(id$0 int, name text);
"), @r"
        hover: myschema.users.id int
          ╭▸ 
        2 │ create table myschema.users(id int, name text);
          ╰╴                             ─ hover
        ");
    }

    #[test]
    fn hover_on_column_in_temp_table() {
        assert_snapshot!(check_hover("
create temp table t(x$0 bigint);
"), @r"
        hover: pg_temp.t.x bigint
          ╭▸ 
        2 │ create temp table t(x bigint);
          ╰╴                    ─ hover
        ");
    }

    #[test]
    fn hover_on_multiple_columns() {
        assert_snapshot!(check_hover("
create table t(id int, email$0 text, name varchar(100));
"), @r"
        hover: public.t.email text
          ╭▸ 
        2 │ create table t(id int, email text, name varchar(100));
          ╰╴                           ─ hover
        ");
    }

    #[test]
    fn hover_on_drop_table() {
        assert_snapshot!(check_hover("
create table users(id int, email text);
drop table users$0;
"), @r"
        hover: create table public.users(id int, email text)
          ╭▸ 
        3 │ drop table users;
          ╰╴               ─ hover
        ");
    }

    #[test]
    fn hover_on_drop_table_with_schema() {
        assert_snapshot!(check_hover("
create table myschema.users(id int);
drop table myschema.users$0;
"), @r"
        hover: create table myschema.users(id int)
          ╭▸ 
        3 │ drop table myschema.users;
          ╰╴                        ─ hover
        ");
    }

    #[test]
    fn hover_on_drop_temp_table() {
        assert_snapshot!(check_hover("
create temp table t(x bigint);
drop table t$0;
"), @r"
        hover: create temp table pg_temp.t(x bigint)
          ╭▸ 
        3 │ drop table t;
          ╰╴           ─ hover
        ");
    }

    #[test]
    fn hover_on_create_index_definition() {
        assert_snapshot!(check_hover("
create table t(x bigint);
create index idx$0 on t(x);
"), @r"
        hover: create index public.idx on public.t(x)
          ╭▸ 
        3 │ create index idx on t(x);
          ╰╴               ─ hover
        ");
    }

    #[test]
    fn hover_on_drop_index() {
        assert_snapshot!(check_hover("
create table t(x bigint);
create index idx_x on t(x);
drop index idx_x$0;
"), @r"
        hover: create index public.idx_x on public.t(x)
          ╭▸ 
        4 │ drop index idx_x;
          ╰╴               ─ hover
        ");
    }

    #[test]
    fn hover_on_drop_function() {
        assert_snapshot!(check_hover("
create function foo() returns int as $$ select 1 $$ language sql;
drop function foo$0();
"), @r"
        hover: function public.foo() returns int
          ╭▸ 
        3 │ drop function foo();
          ╰╴                ─ hover
        ");
    }

    #[test]
    fn hover_on_drop_function_with_schema() {
        assert_snapshot!(check_hover("
create function myschema.foo() returns int as $$ select 1 $$ language sql;
drop function myschema.foo$0();
"), @r"
        hover: function myschema.foo() returns int
          ╭▸ 
        3 │ drop function myschema.foo();
          ╰╴                         ─ hover
        ");
    }

    #[test]
    fn hover_on_create_function_definition() {
        assert_snapshot!(check_hover("
create function foo$0() returns int as $$ select 1 $$ language sql;
"), @r"
        hover: function public.foo() returns int
          ╭▸ 
        2 │ create function foo() returns int as $$ select 1 $$ language sql;
          ╰╴                  ─ hover
        ");
    }

    #[test]
    fn hover_on_create_function_with_explicit_schema() {
        assert_snapshot!(check_hover("
create function myschema.foo$0() returns int as $$ select 1 $$ language sql;
"), @r"
        hover: function myschema.foo() returns int
          ╭▸ 
        2 │ create function myschema.foo() returns int as $$ select 1 $$ language sql;
          ╰╴                           ─ hover
        ");
    }

    #[test]
    fn hover_on_drop_function_with_search_path() {
        assert_snapshot!(check_hover(r#"
set search_path to myschema;
create function foo() returns int as $$ select 1 $$ language sql;
drop function foo$0();
"#), @r"
        hover: function myschema.foo() returns int
          ╭▸ 
        4 │ drop function foo();
          ╰╴                ─ hover
        ");
    }

    #[test]
    fn hover_on_select_function_call() {
        assert_snapshot!(check_hover("
create function foo() returns int as $$ select 1 $$ language sql;
select foo$0();
"), @r"
        hover: function public.foo() returns int
          ╭▸ 
        3 │ select foo();
          ╰╴         ─ hover
        ");
    }

    #[test]
    fn hover_on_select_function_call_with_schema() {
        assert_snapshot!(check_hover("
create function public.foo() returns int as $$ select 1 $$ language sql;
select public.foo$0();
"), @r"
        hover: function public.foo() returns int
          ╭▸ 
        3 │ select public.foo();
          ╰╴                ─ hover
        ");
    }

    #[test]
    fn hover_on_select_function_call_with_search_path() {
        assert_snapshot!(check_hover(r#"
set search_path to myschema;
create function foo() returns int as $$ select 1 $$ language sql;
select foo$0();
"#), @r"
        hover: function myschema.foo() returns int
          ╭▸ 
        4 │ select foo();
          ╰╴         ─ hover
        ");
    }

    #[test]
    fn hover_on_select_function_call_with_params() {
        assert_snapshot!(check_hover("
create function add(a int, b int) returns int as $$ select a + b $$ language sql;
select add$0(1, 2);
"), @r"
        hover: function public.add(a int, b int) returns int
          ╭▸ 
        3 │ select add(1, 2);
          ╰╴         ─ hover
        ");
    }
}
