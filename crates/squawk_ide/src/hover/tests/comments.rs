use insta::assert_snapshot;

use super::hover;

#[test]
fn function_comment() {
    assert_snapshot!(hover("
-- this is a doc comment
-- for foo
create function foo() returns int
  language sql
  return 1;
select foo$0();
"), @"
    hover: function public.foo() returns int4
      ╭▸ file0.sql:7:8
      │
    7 │ select foo();
      │        ─── hover
      │
      ╰ note: this is a doc comment
              for foo
    ");
}

#[test]
fn type_comment() {
    assert_snapshot!(hover("
-- this is a doc comment
-- for foo
create type foo as enum (
  'a',
  'b'
);
select 1::foo$0;
"), @"
    hover: type public.foo as enum ('a', 'b')
      ╭▸ file0.sql:8:11
      │
    8 │ select 1::foo;
      │           ─── hover
      │
      ╰ note: this is a doc comment
              for foo
    ");
}

#[test]
fn type_alias_comment() {
    assert_snapshot!(hover("
-- 64-bit integer
create type pg_catalog.int8;
select 1::bigint$0;
"), @"
    hover: shell type pg_catalog.int8
      ╭▸ file0.sql:4:11
      │
    4 │ select 1::bigint;
      │           ────── hover
      │
      ╰ note: 64-bit integer
    ");
}

#[test]
fn empty_comment_lines_are_skipped() {
    assert_snapshot!(hover("
-- variable-length string, no limit specified
--
-- size: -1, align: 4
create type pg_catalog.text;
select '1'::text$0;
"), @"
    hover: shell type pg_catalog.text
      ╭▸ file0.sql:6:13
      │
    6 │ select '1'::text;
      │             ──── hover
      │
      ╰ note: variable-length string, no limit specified
              size: -1, align: 4
    ");
}

#[test]
fn block_comment() {
    assert_snapshot!(hover("
/*
 * the users table
 * one row per user
 */
create table users (id int);
select * from users$0;
"), @"
    hover: table public.users(id int4)
      ╭▸ file0.sql:7:15
      │
    7 │ select * from users;
      │               ───── hover
      │
      ╰ note: the users table
              one row per user
    ");
}

#[test]
fn blank_line_detaches_comment() {
    assert_snapshot!(hover("
-- not attached

create table users (id int);
select * from users$0;
"), @"
    hover: table public.users(id int4)
      ╭▸ file0.sql:5:15
      │
    5 │ select * from users;
      ╰╴              ───── hover
    ");
}

#[test]
fn column_comment() {
    assert_snapshot!(hover("
create table users (
  -- email address
  email text
);
select email$0 from users;
"), @"
    hover: column public.users.email text
      ╭▸ file0.sql:6:8
      │
    6 │ select email from users;
      │        ───── hover
      │
      ╰ note: email address
    ");
}

#[test]
fn column_definition_comment() {
    assert_snapshot!(hover("
create table users (
  -- email address
  email$0 text
);
"), @"
    hover: column public.users.email text
      ╭▸ file0.sql:4:3
      │
    4 │   email text
      │   ───── hover
      │
      ╰ note: email address
    ");
}

#[test]
fn composite_field_comment() {
    assert_snapshot!(hover("
create type pair as (
  -- the left side
  a int,
  b text
);
create table t (p pair);
select (p).a$0 from t;
"), @"
    hover: field public.pair.a int4
      ╭▸ file0.sql:8:12
      │
    8 │ select (p).a from t;
      │            ─ hover
      │
      ╰ note: the left side
    ");
}

#[test]
fn table_alias_comment() {
    assert_snapshot!(hover("
-- the users table
create table users (id int);
select u$0.id from users u;
"), @"
    hover: table public.users(id int4)
      ╭▸ file0.sql:4:8
      │
    4 │ select u.id from users u;
      │        ─ hover
      │
      ╰ note: the users table
    ");
}

#[test]
fn view_comment() {
    assert_snapshot!(hover("
-- active users
create view active as
  select 1 a;
select * from active$0;
"), @"
    hover: view public.active(a int4)
      ╭▸ file0.sql:5:15
      │
    5 │ select * from active;
      │               ────── hover
      │
      ╰ note: active users
    ");
}

#[test]
fn statistics_comment() {
    assert_snapshot!(hover("
create table t (
  a int,
  b int
);
-- correlation between a and b
create statistics s on a, b from t;
drop statistics s$0;
"), @"
    hover: statistics public.s on public.t
      ╭▸ file0.sql:8:17
      │
    8 │ drop statistics s;
      │                 ─ hover
      │
      ╰ note: correlation between a and b
    ");
}

#[test]
fn renamed_statistics_keeps_comment() {
    assert_snapshot!(hover("
create table t (
  a int,
  b int
);
-- correlation between a and b
create statistics s on a, b from t;
-- unrelated
alter statistics s rename to s2;
drop statistics s2$0;
"), @"
    hover: statistics public.s2 on public.t
       ╭▸ file0.sql:10:17
       │
    10 │ drop statistics s2;
       │                 ── hover
       │
       ╰ note: correlation between a and b
    ");
}

#[test]
fn moved_function_keeps_comment() {
    assert_snapshot!(hover("
create schema s;
-- adds one
create function f(a int) returns int
  language sql
  return a + 1;
-- unrelated
alter function f(int) set schema s;
select s.f$0(1);
"), @"
    hover: function s.f(a int4) returns int4
      ╭▸ file0.sql:9:10
      │
    9 │ select s.f(1);
      │          ─ hover
      │
      ╰ note: adds one
    ");
}

#[test]
fn replaced_function_uses_new_comment() {
    assert_snapshot!(hover("
-- old
create function f() returns int
  language sql
  return 1;
-- new
create or replace function f() returns int
  language sql
  return 2;
select f$0();
"), @"
    hover: function public.f() returns int4
       ╭▸ file0.sql:10:8
       │
    10 │ select f();
       │        ─ hover
       │
       ╰ note: new
    ");
}

#[test]
fn builtin_function_comment() {
    assert_snapshot!(hover("
select now$0();
"), @"
    hover: function pg_catalog.now() returns timestamptz
      ╭▸ file0.sql:2:8
      │
    2 │ select now();
      │        ─── hover
      │
      ╰ note: current transaction time
    ");
}

#[test]
fn index_comment() {
    assert_snapshot!(hover("
create table t (a int);
-- lookup by a
create index i on t (a);
drop index i$0;
"), @"
    hover: index public.i on public.t
      ╭▸ file0.sql:5:12
      │
    5 │ drop index i;
      │            ─ hover
      │
      ╰ note: lookup by a
    ");
}

#[test]
fn table_constraint_comment() {
    assert_snapshot!(hover("
create table t (
  a int,
  -- a must be positive
  constraint positive check (a > 0)
);
alter table t drop constraint positive$0;
"), @"
    hover: constraint positive on public.t
      ╭▸ file0.sql:7:31
      │
    7 │ alter table t drop constraint positive;
      │                               ──────── hover
      │
      ╰ note: a must be positive
    ");
}

#[test]
fn trigger_comment() {
    assert_snapshot!(hover("
create table t (a int);
create function f() returns trigger
  language plpgsql
  as 'begin return new; end';
-- audit inserts
create trigger tr
  after insert
  on t
  for each row
  execute function f();
drop trigger tr$0 on t;
"), @"
    hover: trigger tr on public.t
       ╭▸ file0.sql:12:14
       │
    12 │ drop trigger tr on t;
       │              ── hover
       │
       ╰ note: audit inserts
    ");
}

#[test]
fn role_comment() {
    assert_snapshot!(hover("
-- read only access
create role reader;
drop role reader$0;
"), @"
    hover: role reader
      ╭▸ file0.sql:4:11
      │
    4 │ drop role reader;
      │           ────── hover
      │
      ╰ note: read only access
    ");
}

#[test]
fn schema_comment() {
    assert_snapshot!(hover("
-- application tables
create schema app;
create table app$0.t (a int);
"), @"
    hover: schema app
      ╭▸ file0.sql:4:14
      │
    4 │ create table app.t (a int);
      │              ─── hover
      │
      ╰ note: application tables
    ");
}

#[test]
fn cte_comment() {
    assert_snapshot!(hover("
with
-- the numbers
t as (
  select 1 a
)
select * from t$0;
"), @"
    hover: with t(a int4)
      ╭▸ file0.sql:7:15
      │
    7 │ select * from t;
      │               ─ hover
      │
      ╰ note: the numbers
    ");
}

#[test]
fn param_comment() {
    assert_snapshot!(hover("
create function f(
  -- how many
  n int
) returns int
  language sql
  return n$0;
"), @"
    hover: parameter public.f.n int4
      ╭▸ file0.sql:7:10
      │
    7 │   return n;
      │          ─ hover
      │
      ╰ note: how many
    ");
}

#[test]
fn window_comment() {
    assert_snapshot!(hover("
select row_number() over w$0
from (select 1 a) t
window -- ordered by a
w as (order by a);
"), @"
    hover: window w
      ╭▸ file0.sql:2:26
      │
    2 │ select row_number() over w
      │                          ─ hover
      │
      ╰ note: ordered by a
    ");
}

#[test]
fn enum_label_comment() {
    assert_snapshot!(hover("
create type mood as enum (
  'happy',
  -- not happy
  'sad'
);
create table t (m mood);
select * from t where m = 'sad$0';
"), @"
    hover: enum label 'sad' of public.mood
      ╭▸ file0.sql:8:27
      │
    8 │ select * from t where m = 'sad';
      │                           ───── hover
      │
      ╰ note: not happy
    ");
}

#[test]
fn cursor_comment() {
    assert_snapshot!(hover("
begin;
-- all the things
declare c cursor
for select 1;
fetch next from c$0;
"), @"
    hover: cursor c
      ╭▸ file0.sql:6:17
      │
    6 │ fetch next from c;
      │                 ─ hover
      │
      ╰ note: all the things
    ");
}

#[test]
fn prepared_statement_comment() {
    assert_snapshot!(hover("
-- one row
prepare p as
  select 1;
execute p$0;
"), @"
    hover: prepare p(?column? int4)
      ╭▸ file0.sql:5:9
      │
    5 │ execute p;
      │         ─ hover
      │
      ╰ note: one row
    ");
}

#[test]
fn operator_comment() {
    assert_snapshot!(hover("
create function f(int, int) returns bool
  language sql
  return true;
-- fuzzy equals
create operator === (
  leftarg = int,
  rightarg = int,
  function = f
);
select 1 ===$0 2;
"), @"
    hover: operator public.===(int4, int4) returns bool
       ╭▸ file0.sql:11:10
       │
    11 │ select 1 === 2;
       │          ─── hover
       │
       ╰ note: fuzzy equals
    ");
}

#[test]
fn renamed_role_keeps_comment() {
    assert_snapshot!(hover("
-- read only access
create role reader;
alter role reader rename to viewer;
drop role viewer$0;
"), @"
    hover: role viewer
      ╭▸ file0.sql:5:11
      │
    5 │ drop role viewer;
      │           ────── hover
      │
      ╰ note: read only access
    ");
}

#[test]
fn renamed_trigger_ignores_alter_comment() {
    assert_snapshot!(hover("
create table t (a int);
create function f() returns trigger
  language plpgsql
  as 'begin return new; end';
-- audit inserts
create trigger tr
  after insert
  on t
  for each row
  execute function f();
-- unrelated
alter trigger tr on t rename to tr2;
drop trigger tr2$0 on t;
"), @"
    hover: trigger tr2 on public.t
       ╭▸ file0.sql:14:14
       │
    14 │ drop trigger tr2 on t;
       │              ─── hover
       │
       ╰ note: audit inserts
    ");
}

#[test]
fn rename_drops_alter_comment_when_none() {
    assert_snapshot!(hover("
create table t (a int);
create policy p on t;
-- unrelated
alter policy p on t rename to p2;
drop policy p2$0 on t;
"), @"
    hover: policy p2 on public.t
      ╭▸ file0.sql:6:13
      │
    6 │ drop policy p2 on t;
      ╰╴            ── hover
    ");
}

#[test]
fn renamed_schema_keeps_comment() {
    assert_snapshot!(hover("
-- application tables
create schema app;
alter schema app rename to application;
create table application$0.t (a int);
"), @"
    hover: schema application
      ╭▸ file0.sql:5:14
      │
    5 │ create table application.t (a int);
      │              ─────────── hover
      │
      ╰ note: application tables
    ");
}

#[test]
fn renamed_operator_family_keeps_comment() {
    assert_snapshot!(hover("
-- custom family
create operator family f
  using btree;
-- unrelated
alter operator family f using btree rename to g;
drop operator family g$0 using btree;
"), @"
    hover: operator family public.g using btree
      ╭▸ file0.sql:7:22
      │
    7 │ drop operator family g using btree;
      │                      ─ hover
      │
      ╰ note: custom family
    ");
}
