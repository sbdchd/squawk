use insta::assert_snapshot;
use rowan::{TextRange, TextSize};

use super::{build_index, check};
use crate::file::InFile;

#[test]
fn pg_catalog_before_public() {
    assert_snapshot!(check("
create table pg_class (x int);
select x from pg_class;
select x from public.pg_class;
"), @"
    error: column `x` does not exist
      ╭▸ 
    3 │ select x from pg_class;
      ╰╴       ━
    ");
}

#[test]
fn set_search_path() {
    assert_snapshot!(check("
create schema myschema;
set search_path to myschema;
create table t (a int);
select a from t;
select a from myschema.t;
select a from public.t;
"), @"
    error: relation `t` does not exist
      ╭▸ 
    7 │ select a from public.t;
      ╰╴                     ━
    ");
}

#[test]
fn set_search_path_order() {
    assert_snapshot!(check("
create schema b;
create schema a;
create table b.t (y int);
create table a.t (x int);
set search_path = b, a;
select y, x from t;
"), @"
    error: column `x` does not exist
      ╭▸ 
    7 │ select y, x from t;
      ╰╴          ━
    ");
}

#[test]
fn set_search_path_explicit_pg_catalog() {
    assert_snapshot!(check("
set search_path = public, pg_catalog;
create table pg_class (x int);
select x from pg_class;
"), @"no diagnostics");
}

#[test]
fn set_search_path_dollar_user() {
    assert_snapshot!(check(r#"
create schema app;
set search_path = "$user", app;
create table t (a int);
select a from app.t;
"#), @"no diagnostics");
}

#[test]
fn set_search_path_quoting() {
    assert_snapshot!(check(r#"
create schema "MySchema";
create table "MySchema".t (a int);
set search_path = MySchema;
select a from t;
set search_path = "MySchema";
select a from t;
set search_path = 'MySchema';
select a from t;
"#), @"
    error: relation `t` does not exist
      ╭▸ 
    5 │ select a from t;
      ╰╴              ━
    ");
}

#[test]
fn set_schema() {
    assert_snapshot!(check("
create schema app;
set schema 'app';
create table t (a int);
select a from app.t;
"), @"no diagnostics");
}

#[test]
fn reset_search_path() {
    assert_snapshot!(check("
create schema app;
create table app.t (a int);
set search_path = app;
select a from t;
reset search_path;
select a from t;
set search_path = app;
reset all;
select a from t;
set search_path = app;
set search_path to default;
select a from t;
"), @"
    error: relation `t` does not exist
      ╭▸ 
    7 │ select a from t;
      ╰╴              ━
    error: relation `t` does not exist
       ╭▸ 
    10 │ select a from t;
       ╰╴              ━
    error: relation `t` does not exist
       ╭▸ 
    13 │ select a from t;
       ╰╴              ━
    ");
}

#[test]
fn set_local_search_path() {
    assert_snapshot!(check("
create schema app;
create table app.t (a int);
set local search_path = app;
select a from t;
begin;
set local search_path = app;
select a from t;
commit;
select a from t;
"), @"
    error: relation `t` does not exist
      ╭▸ 
    5 │ select a from t;
      ╰╴              ━
    error: relation `t` does not exist
       ╭▸ 
    10 │ select a from t;
       ╰╴              ━
    ");
}

#[test]
fn rollback_search_path() {
    assert_snapshot!(check("
create schema app;
create table app.t (a int);
begin;
set search_path = app;
select a from t;
rollback;
select a from t;
begin;
set search_path = app;
commit;
select a from t;
"), @"
    error: relation `t` does not exist
      ╭▸ 
    8 │ select a from t;
      ╰╴              ━
    ");
}

#[test]
fn set_config_search_path() {
    assert_snapshot!(check(r#"
create schema app;
create schema "App";
create table app.t (a int);
create table "App".u (b int);
select pg_catalog.set_config('search_path', '', false);
select a from t;
select set_config('search_path', 'app, "App"', false);
select a from t;
select b from u;
"#), @"
    error: relation `t` does not exist
      ╭▸ 
    7 │ select a from t;
      ╰╴              ━
    ");
}

#[test]
fn create_function_search_path() {
    assert_snapshot!(check("
create schema app;
create table app.t (a int);
create function f() returns int language sql
  set search_path = app
  return (select a from t);
create function g() returns int language sql
  set search_path = app
  begin atomic
    select a from t;
  end;
select f(), g(), app.f();
select a from t;
"), @"
    error: function `app.f()` does not exist
       ╭▸ 
    12 │ select f(), g(), app.f();
       ╰╴                     ━
    error: relation `t` does not exist
       ╭▸ 
    13 │ select a from t;
       ╰╴              ━
    ");
}

#[test]
fn create_function_search_path_from_current() {
    assert_snapshot!(check("
create schema app;
create table app.t (a int);
set search_path = app;
create function f() returns int language sql
  set search_path from current
  return (select a from t);
select f(), a from t;
"), @"no diagnostics");
}

#[test]
fn temp_table_shadows_public() {
    assert_snapshot!(check("
create table t (a int);
create temp table t (b int);
select b from t;
select a from t;
select a from public.t;
select b from pg_temp.t;
"), @"
    error: column `a` does not exist
      ╭▸ 
    5 │ select a from t;
      ╰╴       ━
    ");
}

#[test]
fn temp_table_and_view_kinds() {
    assert_snapshot!(check("
create temporary table t (a int);
create local temp table u (b int);
create table pg_temp.v (c int);
create temp view w as select 1 d;
select a, b, c, d from t, u, v, w;
select a from public.t;
select d from public.w;
"), @"
    error: relation `t` does not exist
      ╭▸ 
    7 │ select a from public.t;
      ╰╴                     ━
    error: relation `w` does not exist
      ╭▸ 
    8 │ select d from public.w;
      ╰╴                     ━
    ");
}

#[test]
fn temp_schema_numbered_alias() {
    assert_snapshot!(check("
create table pg_temp_3.t (a int);
create table u (b int);
set search_path = public, pg_temp_7;
select a from pg_temp.t;
select a from pg_temp_1.t;
select b from u;
select a from t;
"), @"no diagnostics");
}

#[test]
fn temp_schema_explicit_position() {
    assert_snapshot!(check("
create table t (a int);
create temp table t (b int);
set search_path = public, pg_temp;
select a from t;
select b from t;
"), @"
    error: column `b` does not exist
      ╭▸ 
    6 │ select b from t;
      ╰╴       ━
    ");
}

#[test]
fn temp_functions_not_searched() {
    assert_snapshot!(check("
create function pg_temp.f() returns int language sql return 1;
set search_path = pg_temp, public;
select f(), pg_temp.f(), pg_temp_2.f();
"), @"
    error: function `f()` does not exist
      ╭▸ 
    4 │ select f(), pg_temp.f(), pg_temp_2.f();
      ╰╴       ━
    ");
}

#[test]
fn temp_table_not_created_via_search_path() {
    assert_snapshot!(check("
create schema app;
set search_path = pg_temp, app;
create table t (a int);
select a from app.t;
select a from pg_temp.t;
"), @"
    error: relation `t` does not exist
      ╭▸ 
    6 │ select a from pg_temp.t;
      ╰╴                      ━
    ");
}

#[test]
fn set_catalog() {
    assert_snapshot!(check("
set catalog 'db';
set local catalog 'db';
"), @"
    error: current database cannot be changed
      ╭▸ 
    2 │ set catalog 'db';
      ╰╴    ━━━━━━━━━━━━
    error: current database cannot be changed
      ╭▸ 
    3 │ set local catalog 'db';
      ╰╴          ━━━━━━━━━━━━
    ");
}

#[test]
fn search_path_missing_schema() {
    assert_snapshot!(check(r#"
set search_path to "$user", missing, 'also_missing';
alter role current_user set search_path = missing;
create function f() returns int language sql set search_path = missing as $$ select 1 $$;
"#), @"no diagnostics");
}

#[track_caller]
fn search_path_at(sql: &str) -> String {
    let offset = sql.find("$0").unwrap();
    let sql = sql.replacen("$0", "", 1);
    let (file, index) = build_index(&sql);
    let range = InFile::new(
        file,
        TextRange::empty(TextSize::new(u32::try_from(offset).unwrap())),
    );
    index
        .search_path_at(range)
        .unwrap()
        .iter()
        .map(|&schema| index.names.resolve(schema))
        .collect::<Vec<_>>()
        .join(", ")
}

#[test]
fn search_path_at_function_body() {
    assert_snapshot!(search_path_at("
create function f() returns int
  set search_path = other
  as $$ select $01 $$
  language sql;
"), @"other");
}

#[test]
fn search_path_at_after_function_body() {
    assert_snapshot!(search_path_at("
create function f() returns int
  set search_path = other
  as $$ select 1 $$
  language s$0ql;
"), @"$user, public");
}

#[test]
fn search_path_at_return_body() {
    assert_snapshot!(search_path_at("
create function f() returns int
  set search_path = other
  return $01;
"), @"other");
}

#[test]
fn search_path_at_create_schema_element() {
    assert_snapshot!(search_path_at("
create schema s
  create table t (a int)
  create view v as select $01;
"), @"s, $user, public");
}

#[test]
fn search_path_at_after_create_schema() {
    assert_snapshot!(search_path_at("
create schema s
  create table t (a int);
select $01;
"), @"$user, public");
}

#[test]
fn search_path_at_atomic_body() {
    assert_snapshot!(search_path_at("
create function f() returns int
  set search_path = other
  begin atomic
    select $01;
  end;
"), @"other");
}

#[test]
fn search_path_at_after_atomic_body() {
    assert_snapshot!(search_path_at("
create function f() returns int
  set search_path = other
  begin atomic
    select 1;
  end;
select $01;
"), @"$user, public");
}

#[test]
fn search_path_at_set_in_body() {
    assert_snapshot!(search_path_at("
create function f() returns void
  as $$ set search_path = other; select $01 $$
  language sql;
"), @"other");
}

#[test]
fn search_path_at_after_set_in_body() {
    assert_snapshot!(search_path_at("
create function f() returns void
  as $$ set search_path = other; select 1 $$
  language sql;
select $01;
"), @"$user, public");
}

#[test]
fn search_path_at_after_set_in_body_and_commit() {
    assert_snapshot!(search_path_at("
create function f() returns void
  as $$ set search_path = other; select 1 $$
  language sql;
begin;
commit;
select $01;
"), @"$user, public");
}
