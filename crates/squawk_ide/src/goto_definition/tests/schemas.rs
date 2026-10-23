use insta::assert_snapshot;

use super::{goto, goto_files};

#[test]
fn search_path_table() {
    assert_snapshot!(goto("
create table public.t (a int);
create table app.t (a int);
set search_path = app, public;
select a from t$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:18
      │
    3 │ create table app.t (a int);
      │                  ─ 2. destination
    4 │ set search_path = app, public;
    5 │ select a from t;
      ╰╴              ─ 1. source
    ");
}

#[test]
fn search_path_function_body() {
    assert_snapshot!(goto("
create table public.t (a int);
create table app.t (a int);
create function f() returns int
  language sql
  set search_path = app
  return (select a from t$0);
"), @"
    info: goto definition
      ╭▸ file0.sql:3:18
      │
    3 │ create table app.t (a int);
      │                  ─ 2. destination
      ‡
    7 │   return (select a from t);
      ╰╴                        ─ 1. source
    ");
}

#[test]
fn search_path_resets_per_file() {
    assert_snapshot!(goto_files(&[
        "
set search_path = app; create table t (a int);
",
        "
create table t (a int); select a from t$0;
",
    ]), @"
    info: goto definition
      ╭▸ file1.sql:2:14
      │
    2 │ create table t (a int); select a from t;
      ╰╴             ─ 2. destination         ─ 1. source
    ");
}

#[test]
fn long_schema_in_search_path_truncated() {
    assert_snapshot!(goto("
create table ssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssss_one.t (
  x int
);
set
  search_path
  = ssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssss_two;
select * from t$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:83
      │
    2 │ create table ssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssss_one.t (
      │                                                                                   ─ 2. destination
      ‡
    8 │ select * from t;
      ╰╴              ─ 1. source
    ");
}

#[test]
fn schema_in_create_table() {
    assert_snapshot!(goto("
create schema s;
create table s$0.t (a int);
"), @"
    info: goto definition
      ╭▸ file0.sql:2:15
      │
    2 │ create schema s;
      │               ─ 2. destination
    3 │ create table s.t (a int);
      ╰╴             ─ 1. source
    ");
}

#[test]
fn schema_quoted() {
    assert_snapshot!(goto(r#"
create schema "S";
create table "S".t (a int);
select a from "S"$0.t;
"#), @r#"
    info: goto definition
      ╭▸ file0.sql:2:15
      │
    2 │ create schema "S";
      │               ─── 2. destination
    3 │ create table "S".t (a int);
    4 │ select a from "S".t;
      ╰╴              ─── 1. source
    "#);
}

#[test]
fn schema_builtin() {
    assert_snapshot!(goto("
select pg_catalog$0.now();
"), @"
    info: goto definition
       ╭▸ file0.sql:2:8
       │
     2 │ select pg_catalog.now();
       │        ────────── 1. source
       │
       ⸬  builtins.sql:58:15
       │
    58 │ create schema pg_catalog;
       ╰╴              ────────── 2. destination
    ");
}

#[test]
fn schema_if_not_exists() {
    assert_snapshot!(goto("
create schema s;
create schema if not exists s;
create table s$0.t (a int);
"), @"
    info: goto definition
      ╭▸ file0.sql:2:15
      │
    2 │ create schema s;
      │               ─ 2. destination
    3 │ create schema if not exists s;
    4 │ create table s.t (a int);
      ╰╴             ─ 1. source
    ");
}

#[test]
fn schema_element_table() {
    assert_snapshot!(goto("
create schema
  s
  create table t (a int);
select a from s.t$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:4:16
      │
    4 │   create table t (a int);
      │                ─ 2. destination
    5 │ select a from s.t;
      ╰╴                ─ 1. source
    ");
}

#[test]
fn schema_element_view_references_element_table() {
    assert_snapshot!(goto("
create schema
  s
  create table t (a int)
  create view v as
    select a from t$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:4:16
      │
    4 │   create table t (a int)
      │                ─ 2. destination
    5 │   create view v as
    6 │     select a from t;
      ╰╴                  ─ 1. source
    ");
}

#[test]
fn schema_from_authorization() {
    assert_snapshot!(goto("
create role a;
create schema authorization a;
create table a$0.t (x int);
"), @"
    info: goto definition
      ╭▸ file0.sql:3:29
      │
    3 │ create schema authorization a;
      │                             ─ 2. destination
    4 │ create table a.t (x int);
      ╰╴             ─ 1. source
    ");
}

#[test]
fn drop_schema_ref() {
    assert_snapshot!(goto("
create schema s;
drop schema s$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:15
      │
    2 │ create schema s;
      │               ─ 2. destination
    3 │ drop schema s;
      ╰╴            ─ 1. source
    ");
}

#[test]
fn alter_schema_rename_ref() {
    assert_snapshot!(goto("
create schema s;
create table s.t (a int);
alter schema s rename to r;
select a from r$0.t;
"), @"
    info: goto definition
      ╭▸ file0.sql:4:26
      │
    4 │ alter schema s rename to r;
      │                          ─ 2. destination
    5 │ select a from r.t;
      ╰╴              ─ 1. source
    ");
}

#[test]
fn alter_schema_renamed_table_ref() {
    assert_snapshot!(goto("
create schema s;
create table s.t (a int);
alter schema s rename to r;
select a from r.t$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:16
      │
    3 │ create table s.t (a int);
      │                ─ 2. destination
    4 │ alter schema s rename to r;
    5 │ select a from r.t;
      ╰╴                ─ 1. source
    ");
}

#[test]
fn alter_materialized_view_set_schema_ref() {
    assert_snapshot!(goto("
create schema s;
create materialized view v as
  select 1 a;
alter materialized view v set schema s;
select a from s.v$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:26
      │
    3 │ create materialized view v as
      │                          ─ 2. destination
      ‡
    6 │ select a from s.v;
      ╰╴                ─ 1. source
    ");
}

#[test]
fn revoke_schema_ref() {
    assert_snapshot!(goto("
create schema sc;
revoke all on all tables in schema sc$0 from public;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:15
      │
    2 │ create schema sc;
      │               ── 2. destination
    3 │ revoke all on all tables in schema sc from public;
      ╰╴                                   ── 1. source
    ");
}

#[test]
fn alter_default_privileges_schema_ref() {
    assert_snapshot!(goto("
create schema sc;
alter default privileges in schema sc$0 grant select on tables to public;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:15
      │
    2 │ create schema sc;
      │               ── 2. destination
    3 │ alter default privileges in schema sc grant select on tables to public;
      ╰╴                                   ── 1. source
    ");
}

#[test]
fn reindex_schema_ref() {
    assert_snapshot!(goto("
create schema s;
reindex schema s$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:15
      │
    2 │ create schema s;
      │               ─ 2. destination
    3 │ reindex schema s;
      ╰╴               ─ 1. source
    ");
}

#[test]
fn with_search_path_via_set_config() {
    assert_snapshot!(goto("
select set_config('search_path', 'foo, public', false);
create table foo.t ();
drop table t$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:18
      │
    3 │ create table foo.t ();
      │                  ─ 2. destination
    4 │ drop table t;
      ╰╴           ─ 1. source
    ");
}

#[test]
fn with_search_path_via_set_config_pg_catalog_qualified() {
    assert_snapshot!(goto("
select pg_catalog.set_config('search_path', 'foo', false);
create table foo.t ();
drop table t$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:18
      │
    3 │ create table foo.t ();
      │                  ─ 2. destination
    4 │ drop table t;
      ╰╴           ─ 1. source
    ");
}

#[test]
fn with_set_schema() {
    assert_snapshot!(goto("
set schema 'myschema';
create table myschema.t ();
drop table t$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:23
      │
    3 │ create table myschema.t ();
      │                       ─ 2. destination
    4 │ drop table t;
      ╰╴           ─ 1. source
    ");
}

#[test]
fn with_search_path_empty() {
    assert_snapshot!(goto("
set search_path = '';
create table t ();
drop table t$0;
"), @"no definition");
}

#[test]
fn search_path_schema_name() {
    assert_snapshot!(goto("
create schema app;
set search_path to app$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:15
      │
    2 │ create schema app;
      │               ─── 2. destination
    3 │ set search_path to app;
      ╰╴                   ─── 1. source
    ");
}

#[test]
fn search_path_schema_name_string_literal() {
    assert_snapshot!(goto("
create schema app;
set search_path to 'app$0';
"), @"
    info: goto definition
      ╭▸ file0.sql:2:15
      │
    2 │ create schema app;
      │               ─── 2. destination
    3 │ set search_path to 'app';
      ╰╴                   ───── 1. source
    ");
}

#[test]
fn alter_role_set_search_path() {
    assert_snapshot!(goto("
create schema app;
create role app;
create role r;
alter role r set search_path = app$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:15
      │
    2 │ create schema app;
      │               ─── 2. destination
      ‡
    5 │ alter role r set search_path = app;
      ╰╴                               ─── 1. source
    ");
}

#[test]
fn alter_database_set_search_path() {
    assert_snapshot!(goto("
create schema app;
alter database d set search_path = app$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:15
      │
    2 │ create schema app;
      │               ─── 2. destination
    3 │ alter database d set search_path = app;
      ╰╴                                   ─── 1. source
    ");
}

#[test]
fn alter_function_set_search_path() {
    assert_snapshot!(goto("
create schema app;
create function f() returns int
  language sql
  as $$ select 1 $$;
alter function f() set search_path = app$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:15
      │
    2 │ create schema app;
      │               ─── 2. destination
      ‡
    6 │ alter function f() set search_path = app;
      ╰╴                                     ─── 1. source
    ");
}

#[test]
fn schema_qualified_column() {
    assert_snapshot!(goto("
create schema s;
create table s.t (a int);
select s$0.t.a from s.t;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:15
      │
    2 │ create schema s;
      │               ─ 2. destination
    3 │ create table s.t (a int);
    4 │ select s.t.a from s.t;
      ╰╴       ─ 1. source
    ");
}

#[test]
fn database_schema_qualified_column() {
    assert_snapshot!(goto("
create schema s;
create database d;
create table s.t (a int);
select d.s$0.t.a from s.t;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:15
      │
    2 │ create schema s;
      │               ─ 2. destination
      ‡
    5 │ select d.s.t.a from s.t;
      ╰╴         ─ 1. source
    ");
}

#[test]
fn schema_authorization_current_role_elements() {
    assert_snapshot!(goto("
create schema
  authorization current_role
  create table tab (id int)
  create view v as
    select id from tab$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:4:16
      │
    4 │   create table tab (id int)
      │                ─── 2. destination
    5 │   create view v as
    6 │     select id from tab;
      ╰╴                   ─── 1. source
    ");
}
