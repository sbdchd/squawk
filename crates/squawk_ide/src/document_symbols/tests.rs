use annotate_snippets::{AnnotationKind, Group, Level, Renderer, Snippet, renderer::DecorStyle};
use insta::assert_snapshot;
use squawk_sema::{file::InFile, semantic_index_builder::SemanticIndexBuilder};
use squawk_syntax::SourceFile;

use crate::document_symbols::{
    DocumentSymbol, DocumentSymbolKind, document_symbols_with_index as document_symbols,
};

fn find_symbols(sql: &str) -> Vec<DocumentSymbol> {
    let parse = SourceFile::parse(sql);
    assert_eq!(parse.errors(), vec![]);
    let file = parse.tree();
    let mut db = crate::db::Database::default();
    let file = InFile::new(db.add_file(sql.into()), &file);
    let mut builder = SemanticIndexBuilder::new();
    builder.add_file(file);
    let index = builder.build();
    document_symbols(&index, file)
}

#[track_caller]
fn symbols_not_found(sql: &str) {
    assert_eq!(find_symbols(sql), vec![]);
}

#[track_caller]
fn symbols(sql: &str) -> String {
    let symbols = find_symbols(sql);
    assert!(!symbols.is_empty(), "no symbols found");
    let groups: Vec<_> = symbols
        .iter()
        .map(|symbol| symbol_group(symbol, sql))
        .collect();
    Renderer::plain()
        .decor_style(DecorStyle::Unicode)
        .render(&groups)
}

fn label(symbol: &DocumentSymbol) -> String {
    let kind = match symbol.kind {
        DocumentSymbolKind::Schema => "schema",
        DocumentSymbolKind::Table => "table",
        DocumentSymbolKind::View => "view",
        DocumentSymbolKind::MaterializedView => "materialized view",
        DocumentSymbolKind::Function => "function",
        DocumentSymbolKind::Aggregate => "aggregate",
        DocumentSymbolKind::Procedure => "procedure",
        DocumentSymbolKind::EventTrigger => "event trigger",
        DocumentSymbolKind::Role => "role",
        DocumentSymbolKind::Rule => "rule",
        DocumentSymbolKind::Policy => "policy",
        DocumentSymbolKind::PropertyGraph => "property graph",
        DocumentSymbolKind::Type => "type",
        DocumentSymbolKind::Enum => "enum",
        DocumentSymbolKind::Index => "index",
        DocumentSymbolKind::Domain => "domain",
        DocumentSymbolKind::Sequence => "sequence",
        DocumentSymbolKind::Statistics => "statistics",
        DocumentSymbolKind::Trigger => "trigger",
        DocumentSymbolKind::Tablespace => "tablespace",
        DocumentSymbolKind::Database => "database",
        DocumentSymbolKind::Server => "server",
        DocumentSymbolKind::Extension => "extension",
        DocumentSymbolKind::OperatorClass => "operator class",
        DocumentSymbolKind::OperatorFamily => "operator family",
        DocumentSymbolKind::Operator => "operator",
        DocumentSymbolKind::Publication => "publication",
        DocumentSymbolKind::Subscription => "subscription",
        DocumentSymbolKind::Column => "column",
        DocumentSymbolKind::Variant => "variant",
        DocumentSymbolKind::Cursor => "cursor",
        DocumentSymbolKind::PreparedStatement => "prepared statement",
        DocumentSymbolKind::Channel => "channel",
        DocumentSymbolKind::Savepoint => "savepoint",
    };
    match &symbol.detail {
        Some(detail) => format!("{kind}: {} {detail}", symbol.name),
        None => format!("{kind}: {}", symbol.name),
    }
}

fn symbol_group<'a>(symbol: &DocumentSymbol, sql: &'a str) -> Group<'a> {
    let snippet = Snippet::source(sql)
        .fold(true)
        .annotation(
            AnnotationKind::Primary
                .span(symbol.focus_range.into())
                .label("focus range"),
        )
        .annotation(
            AnnotationKind::Context
                .span(symbol.full_range.into())
                .label("full range"),
        );
    let mut group = Level::INFO.primary_title(label(symbol)).element(snippet);
    if !symbol.children.is_empty() {
        let mut children = Snippet::source(sql).fold(true);
        for child in &symbol.children {
            assert_eq!(child.children, vec![]);
            children = children
                .annotation(
                    AnnotationKind::Context
                        .span(child.full_range.into())
                        .label(format!("full range for `{}`", label(child))),
                )
                .annotation(
                    AnnotationKind::Primary
                        .span(child.focus_range.into())
                        .label("focus range"),
                );
        }
        group = group.element(children);
    }
    group
}
#[test]
fn create_table() {
    assert_snapshot!(symbols("
create table users (
  id int,
  email citext
);"), @"
    info: table: public.users
      ╭▸ 
    2 │   create table users (
      │   │            ━━━━━ focus range
      │ ┌─┘
      │ │
    3 │ │   id int,
    4 │ │   email citext
    5 │ │ );
      │ └──┘ full range
      │
      ⸬  
    3 │     id int,
      │     ┯━────
      │     │
      │     full range for `column: id int`
      │     focus range
    4 │     email citext
      │     ┯━━━━───────
      │     │
      │     full range for `column: email citext`
      ╰╴    focus range
    ");
}

#[test]
fn create_table_as() {
    assert_snapshot!(symbols("
create table t as select 1 a;
"), @"
    info: table: public.t
      ╭▸ 
    2 │ create table t as select 1 a;
      │ ┬────────────┯───────────────
      │ │            │
      │ │            focus range
      │ full range
      │
      ⸬  
    2 │ create table t as select 1 a;
      │                            ┯
      │                            │
      │                            full range for `column: a int4`
      ╰╴                           focus range
    ");
}

#[test]
fn select_into() {
    assert_snapshot!(symbols("select 1 as id into users;"), @"
    info: table: public.users
      ╭▸ 
    1 │ select 1 as id into users;
      │ ┬───────────────────┯━━━━─
      │ │                   │
      │ │                   focus range
      │ full range
      │
      ⸬  
    1 │ select 1 as id into users;
      │             ┯━
      │             │
      │             full range for `column: id int4`
      ╰╴            focus range
    ");
}

#[test]
fn select_into_with_cte() {
    assert_snapshot!(symbols("
with source as (
  select 1 as id
)
select id
into archive.users
from source;
"), @"
    info: table: source
      ╭▸ 
    2 │   with source as (
      │        │━━━━━
      │        │
      │ ┌──────focus range
      │ │
    3 │ │   select 1 as id
    4 │ │ )
      │ └─┘ full range
      │
      ⸬  
    3 │     select 1 as id
      │                 ┯━
      │                 │
      │                 full range for `column: id int4`
      │                 focus range
      ╰╴
    info: table: archive.users
      ╭▸ 
    2 │ ┌ with source as (
    3 │ │   select 1 as id
    4 │ │ )
    5 │ │ select id
    6 │ │ into archive.users
      │ │              ━━━━━ focus range
    7 │ │ from source;
      │ └────────────┘ full range
      │
      ⸬  
    5 │   select id
      │          ┯━
      │          │
      │          full range for `column: id int4`
      ╰╴         focus range
    ");
}

#[test]
fn schema_contents() {
    let sql = "
create schema
  app
  create table users (id int)
  create view ids as
    select id from users;
";
    let symbols = find_symbols(sql);
    assert_eq!(symbols.len(), 1);
    let schema = &symbols[0];
    assert_eq!(schema.kind, DocumentSymbolKind::Schema);
    assert_eq!(schema.name, "app");
    assert_eq!(schema.children.len(), 2);
    let table = &schema.children[0];
    let view = &schema.children[1];
    assert_eq!(table.kind, DocumentSymbolKind::Table);
    assert_eq!(table.name, "app.users");
    assert_eq!(view.kind, DocumentSymbolKind::View);
    assert_eq!(view.name, "app.ids");
    for child in &schema.children {
        assert!(schema.full_range.contains_range(child.full_range));
        assert_eq!(child.children.len(), 1);
        let column = &child.children[0];
        assert_eq!(column.name, "id");
        assert_eq!(column.kind, DocumentSymbolKind::Column);
        assert!(child.full_range.contains_range(column.focus_range));
    }
}

#[test]
fn create_operator() {
    assert_snapshot!(symbols("
create operator public.=== (
  function = int4eq,
  leftarg = int,
  rightarg = int
);
"), @"
    info: operator: public.===
      ╭▸ 
    2 │   create operator public.=== (
      │   │                      ━━━ focus range
      │ ┌─┘
      │ │
    3 │ │   function = int4eq,
    4 │ │   leftarg = int,
    5 │ │   rightarg = int
    6 │ │ );
      ╰╴└──┘ full range
    ");
}

#[test]
fn publication_and_subscription() {
    assert_snapshot!(symbols("
create publication events for all tables;
create subscription replica
  connection 'host=localhost'
  publication events;
"), @"
    info: publication: events
      ╭▸ 
    2 │ create publication events for all tables;
      │ ┬──────────────────┯━━━━━────────────────
      │ │                  │
      │ │                  focus range
      │ full range
      ╰╴
    info: subscription: replica
      ╭▸ 
    3 │   create subscription replica
      │   │                   ━━━━━━━ focus range
      │ ┌─┘
      │ │
    4 │ │   connection 'host=localhost'
    5 │ │   publication events;
      ╰╴└─────────────────────┘ full range
    ");
}

#[test]
fn create_schema() {
    assert_snapshot!(symbols("
create schema foo;
"), @"
    info: schema: foo
      ╭▸ 
    2 │ create schema foo;
      │ ┬─────────────┯━━─
      │ │             │
      │ │             focus range
      ╰╴full range
    ");
}

#[test]
fn create_schema_authorization() {
    assert_snapshot!(symbols("
create schema authorization foo;
"), @"
    info: schema: foo
      ╭▸ 
    2 │ create schema authorization foo;
      │ ┬───────────────────────────┯━━─
      │ │                           │
      │ │                           focus range
      ╰╴full range
    ");
}

#[test]
fn listen_notify_unlisten() {
    assert_snapshot!(symbols("
listen updates;
notify updates;
unlisten updates;
unlisten *;
"), @"
    info: channel: updates listen
      ╭▸ 
    2 │ listen updates;
      │ ┬──────┯━━━━━━─
      │ │      │
      │ │      focus range
      │ full range
      ╰╴
    info: channel: updates notify
      ╭▸ 
    3 │ notify updates;
      │ ┬──────┯━━━━━━─
      │ │      │
      │ │      focus range
      ╰╴full range
    info: channel: updates unlisten
      ╭▸ 
    4 │ unlisten updates;
      │ ┬────────┯━━━━━━─
      │ │        │
      │ │        focus range
      ╰╴full range
    ");
}

#[test]
fn create_function() {
    assert_snapshot!(
        symbols("create function hello() returns void as $$ select 1; $$ language sql;"),
        @"
    info: function: public.hello
      ╭▸ 
    1 │ create function hello() returns void as $$ select 1; $$ language sql;
      │ ┬───────────────┯━━━━────────────────────────────────────────────────
      │ │               │
      │ │               focus range
      ╰╴full range
    "
    );
}

#[test]
fn create_materialized_view() {
    assert_snapshot!(
        symbols("create materialized view reports as select 1;"),
        @"
    info: materialized view: public.reports
      ╭▸ 
    1 │ create materialized view reports as select 1;
      │ ┬────────────────────────┯━━━━━━─────────────
      │ │                        │
      │ │                        focus range
      │ full range
      │
      ⸬  
    1 │ create materialized view reports as select 1;
      │                                            ┯
      │                                            │
      │                                            full range for `column: ?column? int4`
      ╰╴                                           focus range
    "
    );
}

#[test]
fn create_aggregate() {
    assert_snapshot!(
        symbols("create aggregate myavg(int) (sfunc = int4_avg_accum, stype = _int8);"),
        @"
    info: aggregate: public.myavg
      ╭▸ 
    1 │ create aggregate myavg(int) (sfunc = int4_avg_accum, stype = _int8);
      │ ┬────────────────┯━━━━──────────────────────────────────────────────
      │ │                │
      │ │                focus range
      ╰╴full range
    "
    );
}

#[test]
fn create_procedure() {
    assert_snapshot!(
        symbols("create procedure hello() language sql as $$ select 1; $$;"),
        @"
    info: procedure: public.hello
      ╭▸ 
    1 │ create procedure hello() language sql as $$ select 1; $$;
      │ ┬────────────────┯━━━━───────────────────────────────────
      │ │                │
      │ │                focus range
      ╰╴full range
    "
    );
}

#[test]
fn create_index() {
    assert_snapshot!(symbols("
create table users (email text);
create index idx_users_email on users (email);
"), @"
    info: table: public.users
      ╭▸ 
    2 │ create table users (email text);
      │ ┬────────────┯━━━━──────────────
      │ │            │
      │ │            focus range
      │ full range
      │
      ⸬  
    2 │ create table users (email text);
      │                     ┯━━━━─────
      │                     │
      │                     full range for `column: email text`
      │                     focus range
      ╰╴
    info: index: public.idx_users_email
      ╭▸ 
    3 │ create index idx_users_email on users (email);
      │ ┬────────────┯━━━━━━━━━━━━━━──────────────────
      │ │            │
      │ │            focus range
      ╰╴full range
    ");
}

#[test]
fn create_domain() {
    assert_snapshot!(
        symbols("create domain email_addr as text;"),
        @"
    info: domain: public.email_addr
      ╭▸ 
    1 │ create domain email_addr as text;
      │ ┬─────────────┯━━━━━━━━━─────────
      │ │             │
      │ │             focus range
      ╰╴full range
    "
    );
}

#[test]
fn create_sequence() {
    assert_snapshot!(
        symbols("create sequence user_id_seq;"),
        @"
    info: sequence: public.user_id_seq
      ╭▸ 
    1 │ create sequence user_id_seq;
      │ ┬───────────────┯━━━━━━━━━━─
      │ │               │
      │ │               focus range
      ╰╴full range
    "
    );
}

#[test]
fn create_trigger() {
    assert_snapshot!(symbols("
create table users ();
create trigger update_timestamp
  before update on users
  execute function update_modified_column();
"), @"
    info: table: public.users
      ╭▸ 
    2 │ create table users ();
      │ ┬────────────┯━━━━────
      │ │            │
      │ │            focus range
      │ full range
      ╰╴
    info: trigger: update_timestamp
      ╭▸ 
    3 │   create trigger update_timestamp
      │   │              ━━━━━━━━━━━━━━━━ focus range
      │ ┌─┘
      │ │
    4 │ │   before update on users
    5 │ │   execute function update_modified_column();
      ╰╴└────────────────────────────────────────────┘ full range
    ");
}

#[test]
fn create_event_trigger() {
    assert_snapshot!(
        symbols("create event trigger et on ddl_command_start execute function f();"),
        @"
    info: event trigger: et
      ╭▸ 
    1 │ create event trigger et on ddl_command_start execute function f();
      │ ┬────────────────────┯━───────────────────────────────────────────
      │ │                    │
      │ │                    focus range
      ╰╴full range
    "
    );
}

#[test]
fn create_tablespace() {
    assert_snapshot!(symbols("
create tablespace dbspace location '/data/dbs';
"), @"
    info: tablespace: dbspace
      ╭▸ 
    2 │ create tablespace dbspace location '/data/dbs';
      │ ┬─────────────────┯━━━━━━──────────────────────
      │ │                 │
      │ │                 focus range
      ╰╴full range
    ");
}

#[test]
fn create_database() {
    assert_snapshot!(
        symbols("create database mydb;"),
        @"
    info: database: mydb
      ╭▸ 
    1 │ create database mydb;
      │ ┬───────────────┯━━━─
      │ │               │
      │ │               focus range
      ╰╴full range
    "
    );
}

#[test]
fn create_server() {
    assert_snapshot!(symbols("
create server myserver foreign data wrapper postgres_fdw;
"), @"
    info: server: myserver
      ╭▸ 
    2 │ create server myserver foreign data wrapper postgres_fdw;
      │ ┬─────────────┯━━━━━━━───────────────────────────────────
      │ │             │
      │ │             focus range
      ╰╴full range
    ");
}

#[test]
fn create_extension() {
    assert_snapshot!(
        symbols("create extension pgcrypto;"),
        @"
    info: extension: pgcrypto
      ╭▸ 
    1 │ create extension pgcrypto;
      │ ┬────────────────┯━━━━━━━─
      │ │                │
      │ │                focus range
      ╰╴full range
    "
    );
}

#[test]
fn create_role() {
    assert_snapshot!(symbols("
create role reader;
"), @"
    info: role: reader
      ╭▸ 
    2 │ create role reader;
      │ ┬───────────┯━━━━━─
      │ │           │
      │ │           focus range
      ╰╴full range
    ");
}

#[test]
fn create_rule() {
    assert_snapshot!(symbols("
create table t ();
create rule r as on insert to t do nothing;
"), @"
    info: table: public.t
      ╭▸ 
    2 │ create table t ();
      │ ┬────────────┯────
      │ │            │
      │ │            focus range
      │ full range
      ╰╴
    info: rule: r
      ╭▸ 
    3 │ create rule r as on insert to t do nothing;
      │ ┬───────────┯──────────────────────────────
      │ │           │
      │ │           focus range
      ╰╴full range
    ");
}

#[test]
fn create_rule_or_replace() {
    assert_snapshot!(symbols("
create table mytable ();
create or replace rule notify_me as on update to mytable do also notify mytable;
"), @"
    info: table: public.mytable
      ╭▸ 
    2 │ create table mytable ();
      │ ┬────────────┯━━━━━━────
      │ │            │
      │ │            focus range
      │ full range
      ╰╴
    info: rule: notify_me
      ╭▸ 
    3 │ create or replace rule notify_me as on update to mytable do also notify mytable;
      │ ┬──────────────────────┯━━━━━━━━────────────────────────────────────────────────
      │ │                      │
      │ │                      focus range
      ╰╴full range
    ");
}

#[test]
fn create_policy() {
    assert_snapshot!(symbols("
create table t ();
create policy allow_read on t;
"), @"
    info: table: public.t
      ╭▸ 
    2 │ create table t ();
      │ ┬────────────┯────
      │ │            │
      │ │            focus range
      │ full range
      ╰╴
    info: policy: allow_read
      ╭▸ 
    3 │ create policy allow_read on t;
      │ ┬─────────────┯━━━━━━━━━──────
      │ │             │
      │ │             focus range
      ╰╴full range
    ");
}

#[test]
fn multiple_symbols() {
    assert_snapshot!(symbols("
create table users (id int);
create table posts (id int);
create function get_user(user_id int) returns void as $$ select 1; $$ language sql;
"), @"
    info: table: public.users
      ╭▸ 
    2 │ create table users (id int);
      │ ┬────────────┯━━━━──────────
      │ │            │
      │ │            focus range
      │ full range
      │
      ⸬  
    2 │ create table users (id int);
      │                     ┯━────
      │                     │
      │                     full range for `column: id int`
      │                     focus range
      ╰╴
    info: table: public.posts
      ╭▸ 
    3 │ create table posts (id int);
      │ ┬────────────┯━━━━──────────
      │ │            │
      │ │            focus range
      │ full range
      │
      ⸬  
    3 │ create table posts (id int);
      │                     ┯━────
      │                     │
      │                     full range for `column: id int`
      ╰╴                    focus range
    info: function: public.get_user
      ╭▸ 
    4 │ create function get_user(user_id int) returns void as $$ select 1; $$ language sql;
      │ ┬───────────────┯━━━━━━━───────────────────────────────────────────────────────────
      │ │               │
      │ │               focus range
      ╰╴full range
    ");
}

#[test]
fn qualified_names() {
    assert_snapshot!(symbols("
create table public.users (id int);
create function my_schema.hello() returns void as $$ select 1; $$ language sql;
"), @"
    info: table: public.users
      ╭▸ 
    2 │ create table public.users (id int);
      │ ┬───────────────────┯━━━━──────────
      │ │                   │
      │ │                   focus range
      │ full range
      │
      ⸬  
    2 │ create table public.users (id int);
      │                            ┯━────
      │                            │
      │                            full range for `column: id int`
      │                            focus range
      ╰╴
    info: function: my_schema.hello
      ╭▸ 
    3 │ create function my_schema.hello() returns void as $$ select 1; $$ language sql;
      │ ┬─────────────────────────┯━━━━────────────────────────────────────────────────
      │ │                         │
      │ │                         focus range
      ╰╴full range
    ");
}

#[test]
fn create_property_graph() {
    assert_snapshot!(symbols("
create property graph foo.bar
  vertex tables (t key (a) no properties);
"), @"
    info: property graph: foo.bar
      ╭▸ 
    2 │   create property graph foo.bar
      │   │                         ━━━ focus range
      │ ┌─┘
      │ │
    3 │ │   vertex tables (t key (a) no properties);
      ╰╴└──────────────────────────────────────────┘ full range
    ");
}

#[test]
fn create_type() {
    assert_snapshot!(
        symbols("create type status as enum ('active', 'inactive');"),
        @"
    info: enum: public.status
      ╭▸ 
    1 │ create type status as enum ('active', 'inactive');
      │ ┬───────────┯━━━━━────────────────────────────────
      │ │           │
      │ │           focus range
      │ full range
      │
      ⸬  
    1 │ create type status as enum ('active', 'inactive');
      │                             ┯━━━━━━━  ┯━━━━━━━━━
      │                             │         │
      │                             │         full range for `variant: inactive`
      │                             │         focus range
      │                             full range for `variant: active`
      ╰╴                            focus range
    "
    );
}

#[test]
fn create_type_composite() {
    assert_snapshot!(
        symbols("create type person as (name text, age int);"),
        @"
    info: type: public.person
      ╭▸ 
    1 │ create type person as (name text, age int);
      │ ┬───────────┯━━━━━─────────────────────────
      │ │           │
      │ │           focus range
      │ full range
      │
      ⸬  
    1 │ create type person as (name text, age int);
      │                        ┯━━━─────  ┯━━────
      │                        │          │
      │                        │          full range for `column: age int`
      │                        │          focus range
      │                        full range for `column: name text`
      ╰╴                       focus range
    "
    );
}

#[test]
fn create_type_composite_multiple_columns() {
    assert_snapshot!(
        symbols("create type address as (street text, city text, zip varchar(10));"),
        @"
    info: type: public.address
      ╭▸ 
    1 │ create type address as (street text, city text, zip varchar(10));
      │ ┬───────────┯━━━━━━──────────────────────────────────────────────
      │ │           │
      │ │           focus range
      │ full range
      │
      ⸬  
    1 │ create type address as (street text, city text, zip varchar(10));
      │                         ┯━━━━━─────  ┯━━━─────  ┯━━────────────
      │                         │            │          │
      │                         │            │          full range for `column: zip varchar(10)`
      │                         │            │          focus range
      │                         │            full range for `column: city text`
      │                         │            focus range
      │                         full range for `column: street text`
      ╰╴                        focus range
    "
    );
}

#[test]
fn create_type_with_schema() {
    assert_snapshot!(
        symbols("create type myschema.status as enum ('active', 'inactive');"),
        @"
    info: enum: myschema.status
      ╭▸ 
    1 │ create type myschema.status as enum ('active', 'inactive');
      │ ┬────────────────────┯━━━━━────────────────────────────────
      │ │                    │
      │ │                    focus range
      │ full range
      │
      ⸬  
    1 │ create type myschema.status as enum ('active', 'inactive');
      │                                      ┯━━━━━━━  ┯━━━━━━━━━
      │                                      │         │
      │                                      │         full range for `variant: inactive`
      │                                      │         focus range
      │                                      full range for `variant: active`
      ╰╴                                     focus range
    "
    );
}

#[test]
fn create_type_enum_multiple_variants() {
    assert_snapshot!(
        symbols("create type priority as enum ('low', 'medium', 'high', 'urgent');"),
        @"
    info: enum: public.priority
      ╭▸ 
    1 │ create type priority as enum ('low', 'medium', 'high', 'urgent');
      │ ┬───────────┯━━━━━━━─────────────────────────────────────────────
      │ │           │
      │ │           focus range
      │ full range
      │
      ⸬  
    1 │ create type priority as enum ('low', 'medium', 'high', 'urgent');
      │                               ┯━━━━  ┯━━━━━━━  ┯━━━━━  ┯━━━━━━━
      │                               │      │         │       │
      │                               │      │         │       full range for `variant: urgent`
      │                               │      │         │       focus range
      │                               │      │         full range for `variant: high`
      │                               │      │         focus range
      │                               │      full range for `variant: medium`
      │                               │      focus range
      │                               full range for `variant: low`
      ╰╴                              focus range
    "
    );
}

#[test]
fn declare_cursor() {
    assert_snapshot!(symbols("
declare c scroll cursor for select * from t;
"), @"
    info: cursor: c
      ╭▸ 
    2 │ declare c scroll cursor for select * from t;
      │ ┬───────┯───────────────────────────────────
      │ │       │
      │ │       focus range
      ╰╴full range
    ");
}

#[test]
fn prepare_statement() {
    assert_snapshot!(symbols("
prepare stmt as select 1;
"), @"
    info: prepared statement: stmt
      ╭▸ 
    2 │ prepare stmt as select 1;
      │ ┬───────┯━━━─────────────
      │ │       │
      │ │       focus range
      ╰╴full range
    ");
}

#[test]
fn empty_file() {
    symbols_not_found("");
}

#[test]
fn non_create_statements() {
    symbols_not_found("select * from users;");
}

#[test]
fn cte_table() {
    assert_snapshot!(
        symbols("
with recent_users as (
  select id, email as user_email
  from users
)
select * from recent_users;
"),
        @"
    info: table: recent_users
      ╭▸ 
    2 │   with recent_users as (
      │        │━━━━━━━━━━━
      │        │
      │ ┌──────focus range
      │ │
    3 │ │   select id, email as user_email
    4 │ │   from users
    5 │ │ )
      │ └─┘ full range
      │
      ⸬  
    3 │     select id, email as user_email
      │            ┯━           ┯━━━━━━━━━
      │            │            │
      │            │            full range for `column: user_email`
      │            │            focus range
      │            full range for `column: id`
      ╰╴           focus range
    "
    );
}

#[test]
fn cte_table_with_column_list() {
    assert_snapshot!(
        symbols("
with t(a, b, c) as (
  select 1, 2, 3
)
select * from t;
"),
        @"
    info: table: t
      ╭▸ 
    2 │   with t(a, b, c) as (
      │        ━ focus range
      │ ┌──────┘
      │ │
    3 │ │   select 1, 2, 3
    4 │ │ )
      │ └─┘ full range
      │
      ⸬  
    2 │   with t(a, b, c) as (
      │          ┯  ┯  ┯
      │          │  │  │
      │          │  │  full range for `column: c int4`
      │          │  │  focus range
      │          │  full range for `column: b int4`
      │          │  focus range
      │          full range for `column: a int4`
      ╰╴         focus range
    "
    );
}

#[test]
fn create_foreign_table() {
    assert_snapshot!(symbols("
create foreign table films (
  code char(5),
  title varchar(40)
) server film_server;
"), @"
    info: table: public.films
      ╭▸ 
    2 │   create foreign table films (
      │   │                    ━━━━━ focus range
      │ ┌─┘
      │ │
    3 │ │   code char(5),
    4 │ │   title varchar(40)
    5 │ │ ) server film_server;
      │ └─────────────────────┘ full range
      │
      ⸬  
    3 │     code char(5),
      │     ┯━━━────────
      │     │
      │     full range for `column: code char(5)`
      │     focus range
    4 │     title varchar(40)
      │     ┯━━━━────────────
      │     │
      │     full range for `column: title varchar(40)`
      ╰╴    focus range
    ");
}

#[test]
fn create_schema_authorization_existing_role() {
    assert_snapshot!(symbols("
create role foo;
create schema authorization foo;
"), @"
    info: role: foo
      ╭▸ 
    2 │ create role foo;
      │ ┬───────────┯━━─
      │ │           │
      │ │           focus range
      │ full range
      ╰╴
    info: schema: foo
      ╭▸ 
    3 │ create schema authorization foo;
      │ ┬───────────────────────────┯━━─
      │ │                           │
      │ │                           focus range
      ╰╴full range
    ");
}

#[test]
fn create_view_inferred_columns() {
    assert_snapshot!(symbols("
create table users (id int, email text);
create view v as
  select id, email as user_email from users;
"), @"
    info: table: public.users
      ╭▸ 
    2 │ create table users (id int, email text);
      │ ┬────────────┯━━━━──────────────────────
      │ │            │
      │ │            focus range
      │ full range
      │
      ⸬  
    2 │ create table users (id int, email text);
      │                     ┯━────  ┯━━━━─────
      │                     │       │
      │                     │       full range for `column: email text`
      │                     │       focus range
      │                     full range for `column: id int`
      │                     focus range
      ╰╴
    info: view: public.v
      ╭▸ 
    3 │   create view v as
      │   │           ━ focus range
      │ ┌─┘
      │ │
    4 │ │   select id, email as user_email from users;
      │ └────────────────────────────────────────────┘ full range
      │
      ⸬  
    4 │     select id, email as user_email from users;
      │            ┯━           ┯━━━━━━━━━
      │            │            │
      │            │            full range for `column: user_email text`
      │            │            focus range
      │            full range for `column: id int4`
      ╰╴           focus range
    ");
}

#[test]
fn create_table_inherits() {
    assert_snapshot!(symbols("
create table base (id int);
create table child (name text) inherits (base);
"), @"
    info: table: public.base
      ╭▸ 
    2 │ create table base (id int);
      │ ┬────────────┯━━━──────────
      │ │            │
      │ │            focus range
      │ full range
      │
      ⸬  
    2 │ create table base (id int);
      │                    ┯━────
      │                    │
      │                    full range for `column: id int`
      │                    focus range
      ╰╴
    info: table: public.child
      ╭▸ 
    3 │ create table child (name text) inherits (base);
      │ ┬────────────┯━━━━─────────────────────────────
      │ │            │
      │ │            focus range
      │ full range
      │
      ⸬  
    3 │ create table child (name text) inherits (base);
      │                     ┯━━━─────
      │                     │
      │                     full range for `column: name text`
      ╰╴                    focus range
    ");
}

#[test]
fn create_table_alter_add_column() {
    assert_snapshot!(symbols("
create table t (a int);
alter table t add column b text;
"), @"
    info: table: public.t
      ╭▸ 
    2 │ create table t (a int);
      │ ┬────────────┯─────────
      │ │            │
      │ │            focus range
      │ full range
      │
      ⸬  
    2 │ create table t (a int);
      │                 ┯────
      │                 │
      │                 full range for `column: a int`
      ╰╴                focus range
    ");
}

#[test]
fn create_type_enum_add_value() {
    assert_snapshot!(symbols("
create type mood as enum ('happy');
alter type mood add value 'sad';
"), @"
    info: enum: public.mood
      ╭▸ 
    2 │ create type mood as enum ('happy');
      │ ┬───────────┯━━━───────────────────
      │ │           │
      │ │           focus range
      │ full range
      │
      ⸬  
    2 │ create type mood as enum ('happy');
      │                           ┯━━━━━━
      │                           │
      │                           full range for `variant: happy`
      ╰╴                          focus range
    ");
}

#[test]
fn search_path_schema() {
    assert_snapshot!(symbols("
create schema app;
set search_path to app;
create table t (id bigint);
"), @"
    info: schema: app
      ╭▸ 
    2 │ create schema app;
      │ ┬─────────────┯━━─
      │ │             │
      │ │             focus range
      │ full range
      ╰╴
    info: table: app.t
      ╭▸ 
    4 │ create table t (id bigint);
      │ ┬────────────┯─────────────
      │ │            │
      │ │            focus range
      │ full range
      │
      ⸬  
    4 │ create table t (id bigint);
      │                 ┯━───────
      │                 │
      │                 full range for `column: id bigint`
      ╰╴                focus range
    ");
}

#[test]
fn notify_without_listen() {
    assert_snapshot!(symbols("
notify updates;
"), @"
    info: channel: updates notify
      ╭▸ 
    2 │ notify updates;
      │ ┬──────┯━━━━━━─
      │ │      │
      │ │      focus range
      ╰╴full range
    ");
}

#[test]
fn create_operator_family() {
    assert_snapshot!(symbols("
create operator family int_ops
  using btree;
"), @"
    info: operator family: public.int_ops
      ╭▸ 
    2 │   create operator family int_ops
      │   │                      ━━━━━━━ focus range
      │ ┌─┘
      │ │
    3 │ │   using btree;
      ╰╴└──────────────┘ full range
    ");
}

#[test]
fn create_operator_class() {
    assert_snapshot!(symbols("
create operator class int4_ops
  for type int4
  using btree
  family int_ops as
  operator 1 <,
  operator 3 = (int4, int4),
  function 1 btint4cmp(int4, int4);
"), @"
    info: operator class: public.int4_ops
      ╭▸ 
    2 │   create operator class int4_ops
      │   │                     ━━━━━━━━ focus range
      │ ┌─┘
      │ │
    3 │ │   for type int4
    4 │ │   using btree
    5 │ │   family int_ops as
    6 │ │   operator 1 <,
    7 │ │   operator 3 = (int4, int4),
    8 │ │   function 1 btint4cmp(int4, int4);
      │ └───────────────────────────────────┘ full range
      │
      ⸬  
    6 │     operator 1 <,
      │     ┬──────────┯
      │     │          │
      │     │          focus range
      │     full range for `operator: < strategy 1`
    7 │     operator 3 = (int4, int4),
      │     ┬──────────┯─────────────
      │     │          │
      │     │          focus range
      │     full range for `operator: = strategy 3`
    8 │     function 1 btint4cmp(int4, int4);
      │     ┬──────────┯━━━━━━━━────────────
      │     │          │
      │     │          focus range
      ╰╴    full range for `function: btint4cmp support 1`
    ");
}

#[test]
fn create_view_star_columns() {
    assert_snapshot!(symbols("
create table users (id int, email text);
create view v as
  select * from users;
"), @"
    info: table: public.users
      ╭▸ 
    2 │ create table users (id int, email text);
      │ ┬────────────┯━━━━──────────────────────
      │ │            │
      │ │            focus range
      │ full range
      │
      ⸬  
    2 │ create table users (id int, email text);
      │                     ┯━────  ┯━━━━─────
      │                     │       │
      │                     │       full range for `column: email text`
      │                     │       focus range
      │                     full range for `column: id int`
      │                     focus range
      ╰╴
    info: view: public.v
      ╭▸ 
    3 │   create view v as
      │   │           ━ focus range
      │ ┌─┘
      │ │
    4 │ │   select * from users;
      │ └──────────────────────┘ full range
      │
      ⸬  
    4 │     select * from users;
      │            ┯
      │            │
      │            full range for `column: id int4`
      │            focus range
      │            full range for `column: email text`
      ╰╴           focus range
    ");
}

#[test]
fn create_view_qualified_star_columns() {
    assert_snapshot!(symbols("
create table users (id int, email text);
create view v as
  select u.*, 1 as n from users as u;
"), @"
    info: table: public.users
      ╭▸ 
    2 │ create table users (id int, email text);
      │ ┬────────────┯━━━━──────────────────────
      │ │            │
      │ │            focus range
      │ full range
      │
      ⸬  
    2 │ create table users (id int, email text);
      │                     ┯━────  ┯━━━━─────
      │                     │       │
      │                     │       full range for `column: email text`
      │                     │       focus range
      │                     full range for `column: id int`
      │                     focus range
      ╰╴
    info: view: public.v
      ╭▸ 
    3 │   create view v as
      │   │           ━ focus range
      │ ┌─┘
      │ │
    4 │ │   select u.*, 1 as n from users as u;
      │ └─────────────────────────────────────┘ full range
      │
      ⸬  
    4 │     select u.*, 1 as n from users as u;
      │              ┯       ┯
      │              │       │
      │              │       full range for `column: n int4`
      │              │       focus range
      │              full range for `column: id int4`
      │              focus range
      │              full range for `column: email text`
      ╰╴             focus range
    ");
}

#[test]
fn cte_star_columns() {
    assert_snapshot!(symbols("
create table users (id int);
with c as (
  select * from users
)
select * from c;
"), @"
    info: table: public.users
      ╭▸ 
    2 │ create table users (id int);
      │ ┬────────────┯━━━━──────────
      │ │            │
      │ │            focus range
      │ full range
      │
      ⸬  
    2 │ create table users (id int);
      │                     ┯━────
      │                     │
      │                     full range for `column: id int`
      │                     focus range
      ╰╴
    info: table: c
      ╭▸ 
    3 │   with c as (
      │        ━ focus range
      │ ┌──────┘
      │ │
    4 │ │   select * from users
    5 │ │ )
      │ └─┘ full range
      │
      ⸬  
    4 │     select * from users
      │            ┯
      │            │
      │            full range for `column: id int4`
      ╰╴           focus range
    ");
}
