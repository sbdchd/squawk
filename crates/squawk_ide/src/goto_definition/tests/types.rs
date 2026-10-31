use insta::assert_snapshot;

use super::goto;

#[test]
fn cast_to_user_type() {
    assert_snapshot!(goto("
create type mood as enum (
  'happy',
  'sad'
);
select 'happy'::mood$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:13
      │
    2 │ create type mood as enum (
      │             ──── 2. destination
      ‡
    6 │ select 'happy'::mood;
      ╰╴                ──── 1. source
    ");
}

#[test]
fn cast_to_derived_array_type() {
    assert_snapshot!(goto("
create type mood as enum (
  'happy',
  'sad'
);
select '{happy}'::_mood$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:13
      │
    2 │ create type mood as enum (
      │             ──── 2. destination
      ‡
    6 │ select '{happy}'::_mood;
      ╰╴                  ───── 1. source
    ");
}

#[test]
fn cast_function_syntax() {
    assert_snapshot!(goto("
create domain posint as int check (value > 0);
select cast(1 as posint$0);
"), @"
    info: goto definition
      ╭▸ file0.sql:2:15
      │
    2 │ create domain posint as int check (value > 0);
      │               ────── 2. destination
    3 │ select cast(1 as posint);
      ╰╴                 ────── 1. source
    ");
}

#[test]
fn cast_to_qualified_array_type() {
    assert_snapshot!(goto("
create type app.pair as (
  a int,
  b int
);
select null::app.pair$0[];
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create type app.pair as (
      │                 ──── 2. destination
      ‡
    6 │ select null::app.pair[];
      ╰╴                 ──── 1. source
    ");
}

#[test]
fn cast_to_table_row_type() {
    assert_snapshot!(goto("
create table t (a int);
select row(1)::t$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:14
      │
    2 │ create table t (a int);
      │              ─ 2. destination
    3 │ select row(1)::t;
      ╰╴               ─ 1. source
    ");
}

#[test]
fn typed_literal() {
    assert_snapshot!(goto("
create type mood as enum (
  'happy',
  'sad'
);
select mood$0 'happy';
"), @"
    info: goto definition
      ╭▸ file0.sql:2:13
      │
    2 │ create type mood as enum (
      │             ──── 2. destination
      ‡
    6 │ select mood 'happy';
      ╰╴       ──── 1. source
    ");
}

#[test]
fn treat_type() {
    assert_snapshot!(goto("
create type mood as enum (
  'happy',
  'sad'
);
select treat('happy' as mood$0);
"), @"
    info: goto definition
      ╭▸ file0.sql:2:13
      │
    2 │ create type mood as enum (
      │             ──── 2. destination
      ‡
    6 │ select treat('happy' as mood);
      ╰╴                        ──── 1. source
    ");
}

#[test]
fn cast_to_builtin_type() {
    assert_snapshot!(goto("
select '1'::int4$0;
"), @"
    info: goto definition
          ╭▸ file0.sql:2:13
          │
        2 │ select '1'::int4;
          │             ──── 1. source
          │
          ⸬  builtins.sql:192:24
          │
      192 │ create type pg_catalog.int4;
          │                        ──── 2. destination
          ‡
    32162 │ create type pg_catalog.int4 (
          ╰╴                       ──── 3. destination
    ");
}

#[test]
fn cast_to_keyword_type() {
    assert_snapshot!(goto("
select '1'::integer$0;
"), @"
    info: goto definition
          ╭▸ file0.sql:2:13
          │
        2 │ select '1'::integer;
          │             ─────── 1. source
          │
          ⸬  builtins.sql:192:24
          │
      192 │ create type pg_catalog.int4;
          │                        ──── 2. destination
          ‡
    32162 │ create type pg_catalog.int4 (
          ╰╴                       ──── 3. destination
    ");
}

#[test]
fn cast_to_multi_word_type() {
    assert_snapshot!(goto("
select now()::timestamp with time zone$0;
"), @"
    info: goto definition
          ╭▸ file0.sql:2:15
          │
        2 │ select now()::timestamp with time zone;
          │               ──────────────────────── 1. source
          │
          ⸬  builtins.sql:360:24
          │
      360 │ create type pg_catalog.timestamptz;
          │                        ─────────── 2. destination
          ‡
    32425 │ create type pg_catalog.timestamptz (
          ╰╴                       ─────────── 3. destination
    ");
}

#[test]
fn keyword_type_ignores_search_path() {
    assert_snapshot!(goto("
create type int as enum (
  'a'
);
select '1'::int$0;
"), @"
    info: goto definition
          ╭▸ file0.sql:5:13
          │
        5 │ select '1'::int;
          │             ─── 1. source
          │
          ⸬  builtins.sql:192:24
          │
      192 │ create type pg_catalog.int4;
          │                        ──── 2. destination
          ‡
    32162 │ create type pg_catalog.int4 (
          ╰╴                       ──── 3. destination
    ");
}

#[test]
fn quoted_type_name_is_not_keyword() {
    assert_snapshot!(goto(r#"
create type "int" as enum (
  'a'
);
select 'a'::"int"$0;
"#), @r#"
    info: goto definition
      ╭▸ file0.sql:2:13
      │
    2 │ create type "int" as enum (
      │             ───── 2. destination
      ‡
    5 │ select 'a'::"int";
      ╰╴            ───── 1. source
    "#);
}

#[test]
fn type_search_path() {
    assert_snapshot!(goto("
create type public.mood as enum (
  'a'
);
create type app.mood as enum (
  'a'
);
set search_path = app, public;
select 'a'::mood$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:5:17
      │
    5 │ create type app.mood as enum (
      │                 ──── 2. destination
      ‡
    9 │ select 'a'::mood;
      ╰╴            ──── 1. source
    ");
}

#[test]
fn column_type() {
    assert_snapshot!(goto("
create type mood as enum (
  'happy',
  'sad'
);
create table t (m mood$0);
"), @"
    info: goto definition
      ╭▸ file0.sql:2:13
      │
    2 │ create type mood as enum (
      │             ──── 2. destination
      ‡
    6 │ create table t (m mood);
      ╰╴                  ──── 1. source
    ");
}

#[test]
fn function_param_and_return_type() {
    assert_snapshot!(goto("
create type mood as enum (
  'happy',
  'sad'
);
create function f(m mood) returns mood$0
  as 'select m'
  language sql;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:13
      │
    2 │ create type mood as enum (
      │             ──── 2. destination
      ‡
    6 │ create function f(m mood) returns mood
      ╰╴                                  ──── 1. source
    ");
}

#[test]
fn type_definition_itself() {
    assert_snapshot!(goto("
create type mood$0 as enum (
  'happy',
  'sad'
);
"), @"
    info: goto definition
      ╭▸ file0.sql:2:13
      │
    2 │ create type mood as enum (
      │             ┬───
      │             │
      │             1. source
      ╰╴            2. destination
    ");
}

#[test]
fn drop_type() {
    assert_snapshot!(goto("
create type mood as enum (
  'happy',
  'sad'
);
drop type mood$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:13
      │
    2 │ create type mood as enum (
      │             ──── 2. destination
      ‡
    6 │ drop type mood;
      ╰╴          ──── 1. source
    ");
}

#[test]
fn drop_type_qualified_list() {
    assert_snapshot!(goto("
create type app.a as enum (
  'x'
);
create type app.b as enum (
  'x'
);
drop type if exists app.a, app.b$0 cascade;
"), @"
    info: goto definition
      ╭▸ file0.sql:5:17
      │
    5 │ create type app.b as enum (
      │                 ─ 2. destination
      ‡
    8 │ drop type if exists app.a, app.b cascade;
      ╰╴                               ─ 1. source
    ");
}

#[test]
fn recreated_type_after_drop() {
    assert_snapshot!(goto("
create type mood as enum (
  'a'
);
drop type mood;
create type mood as enum (
  'b'
);
select 'b'::mood$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:6:13
      │
    6 │ create type mood as enum (
      │             ──── 2. destination
      ‡
    9 │ select 'b'::mood;
      ╰╴            ──── 1. source
    ");
}

#[test]
fn dropped_type_falls_back_to_next_schema() {
    assert_snapshot!(goto("
create type public.mood as enum (
  'a'
);
create type app.mood as enum (
  'a'
);
set search_path = app, public;
drop type mood;
select 'a'::mood$0;
"), @"
    info: goto definition
       ╭▸ file0.sql:2:20
       │
     2 │ create type public.mood as enum (
       │                    ──── 2. destination
       ‡
    10 │ select 'a'::mood;
       ╰╴            ──── 1. source
    ");
}

#[test]
fn type_query_before_drop_and_recreate() {
    assert_snapshot!(goto("
create type mood as enum (
  'a'
);
select 'a'::mood$0;
drop type mood;
create type mood as enum (
  'b'
);
select 'b'::mood;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:13
      │
    2 │ create type mood as enum (
      │             ──── 2. destination
      ‡
    5 │ select 'a'::mood;
      ╰╴            ──── 1. source
    ");
}

#[test]
fn type_query_after_drop_and_recreate() {
    assert_snapshot!(goto("
create type mood as enum (
  'a'
);
select 'a'::mood;
drop type mood;
create type mood as enum (
  'b'
);
select 'b'::mood$0;
"), @"
    info: goto definition
       ╭▸ file0.sql:7:13
       │
     7 │ create type mood as enum (
       │             ──── 2. destination
       ‡
    10 │ select 'b'::mood;
       ╰╴            ──── 1. source
    ");
}

#[test]
fn schema_qualified_type() {
    assert_snapshot!(goto("
create schema s;
create type s.pair as (
  x int,
  y int
);
select null::s$0.pair;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:15
      │
    2 │ create schema s;
      │               ─ 2. destination
      ‡
    7 │ select null::s.pair;
      ╰╴             ─ 1. source
    ");
}

#[test]
fn create_type_input_builtin_function() {
    assert_snapshot!(goto("
create type t (
  input = int4in$0,
  output = int4out
);
"), @"
    info: goto definition
          ╭▸ file0.sql:3:11
          │
        3 │   input = int4in,
          │           ────── 1. source
          │
          ⸬  builtins.sql:14640:28
          │
    14640 │ create function pg_catalog.int4in(cstring) returns integer
          ╰╴                           ────── 2. destination
    ");
}

#[test]
fn create_type_output_function() {
    assert_snapshot!(goto("
create type t;
create function t_in(cstring) returns t
  language internal
  as 'int4in';
create function t_out(t) returns cstring
  language internal
  as 'int4out';
create type t (
  input = t_in,
  output = t_out$0
);
"), @"
    info: goto definition
       ╭▸ file0.sql:6:17
       │
     6 │ create function t_out(t) returns cstring
       │                 ───── 2. destination
       ‡
    11 │   output = t_out
       ╰╴           ───── 1. source
    ");
}

#[test]
fn create_type_shell_reference_goes_to_shell_and_full_definition() {
    assert_snapshot!(goto("
create type t;
create function t_in(cstring) returns t$0
  language internal
  as 'int4in';
create type t (
  input = t_in,
  output = int4out
);
"), @"
    info: goto definition
      ╭▸ file0.sql:2:13
      │
    2 │ create type t;
      │             ─ 2. destination
    3 │ create function t_in(cstring) returns t
      │                                       ─ 1. source
      ‡
    6 │ create type t (
      ╰╴            ─ 3. destination
    ");
}

#[test]
fn shell_type_reference_goes_to_shell_and_full_definition() {
    assert_snapshot!(goto("
create type t;
create type t (
  input = int4in,
  output = int4out
);
select 1::t$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:13
      │
    2 │ create type t;
      │             ─ 2. destination
    3 │ create type t (
      │             ─ 3. destination
      ‡
    7 │ select 1::t;
      ╰╴          ─ 1. source
    ");
}

#[test]
fn create_type_like() {
    assert_snapshot!(goto("
create type t (
  input = int4in,
  output = int4out,
  like = int4$0
);
"), @"
    info: goto definition
          ╭▸ file0.sql:5:10
          │
        5 │   like = int4
          │          ──── 1. source
          │
          ⸬  builtins.sql:192:24
          │
      192 │ create type pg_catalog.int4;
          │                        ──── 2. destination
          ‡
    32162 │ create type pg_catalog.int4 (
          ╰╴                       ──── 3. destination
    ");
}

#[test]
fn create_type_element_without_array_subscript() {
    assert_snapshot!(goto("
create type u as (
  a int
);
create type t (
  input = int4in,
  output = int4out,
  element = u$0
);
"), @"
    info: goto definition
      ╭▸ file0.sql:2:13
      │
    2 │ create type u as (
      │             ─ 2. destination
      ‡
    8 │   element = u
      ╰╴            ─ 1. source
    ");
}

#[test]
fn alter_type_rename_ref() {
    assert_snapshot!(goto("
create type t as (
  a int
);
alter type t rename to t2;
select null::t2$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:5:24
      │
    5 │ alter type t rename to t2;
      │                        ── 2. destination
    6 │ select null::t2;
      ╰╴             ── 1. source
    ");
}

#[test]
fn alter_type_set_options_ref() {
    assert_snapshot!(goto("
create type t;
create function t_in(cstring) returns t
  language c
  as 'x';
create function t_out(t) returns cstring
  language c
  as 'x';
create type t (
  input = t_in,
  output = t_out
);
create function t_send(t) returns bytea
  language c
  as 'x';
alter type t set (send = t_send$0);
"), @"
    info: goto definition
       ╭▸ file0.sql:13:17
       │
    13 │ create function t_send(t) returns bytea
       │                 ────── 2. destination
       ‡
    16 │ alter type t set (send = t_send);
       ╰╴                         ────── 1. source
    ");
}

#[test]
fn percent_type_column_ref() {
    assert_snapshot!(goto("
create table t (a int);
create function f(x t.a$0%type) returns int
  language sql
  as 'select 1';
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create table t (a int);
      │                 ─ 2. destination
    3 │ create function f(x t.a%type) returns int
      ╰╴                      ─ 1. source
    ");
}

#[test]
fn type_reference_before_rename() {
    assert_snapshot!(goto("
create type ty as (
  a int
);
create table t (c ty$0);
alter type ty rename to ty2;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:13
      │
    2 │ create type ty as (
      │             ── 2. destination
      ‡
    5 │ create table t (c ty);
      ╰╴                  ── 1. source
    ");
}

#[test]
fn cast_float_with_small_arg() {
    assert_snapshot!(goto("
create type pg_catalog.float4;
select '1'::float$0(8);
"), @"
    info: goto definition
      ╭▸ file0.sql:2:24
      │
    2 │ create type pg_catalog.float4;
      │                        ────── 2. destination
    3 │ select '1'::float(8);
      ╰╴            ───── 1. source
    ");
}

#[test]
fn cast_float_with_large_arg() {
    assert_snapshot!(goto("
create type pg_catalog.float8;
select '1'::float$0(25);
"), @"
    info: goto definition
      ╭▸ file0.sql:2:24
      │
    2 │ create type pg_catalog.float8;
      │                        ────── 2. destination
    3 │ select '1'::float(25);
      ╰╴            ───── 1. source
    ");
}

#[test]
fn cast_dec() {
    assert_snapshot!(goto("
create type pg_catalog.numeric;
select '10'::dec$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:24
      │
    2 │ create type pg_catalog.numeric;
      │                        ─────── 2. destination
    3 │ select '10'::dec;
      ╰╴             ─── 1. source
    ");
}

#[test]
fn cast_varchar_with_modifier() {
    assert_snapshot!(goto("
create type pg_catalog.varchar;
select '1'::varchar$0(1);
"), @"
    info: goto definition
      ╭▸ file0.sql:2:24
      │
    2 │ create type pg_catalog.varchar;
      │                        ─────── 2. destination
    3 │ select '1'::varchar(1);
      ╰╴            ────────── 1. source
    ");
}

#[test]
fn json_value_returning_type() {
    assert_snapshot!(goto("
create domain d as int;
select json_value('{}', '$.a' returning d$0);
"), @"
    info: goto definition
      ╭▸ file0.sql:2:15
      │
    2 │ create domain d as int;
      │               ─ 2. destination
    3 │ select json_value('{}', '$.a' returning d);
      ╰╴                                        ─ 1. source
    ");
}

#[test]
fn json_object_returning_type() {
    assert_snapshot!(goto("
create domain d as jsonb;
select json_object('a': 1 returning d$0);
"), @"
    info: goto definition
      ╭▸ file0.sql:2:15
      │
    2 │ create domain d as jsonb;
      │               ─ 2. destination
    3 │ select json_object('a': 1 returning d);
      ╰╴                                    ─ 1. source
    ");
}

#[test]
fn json_table_column_type() {
    assert_snapshot!(goto("
create domain d as int;
select * from json_table('[]'::jsonb, '$' columns (a d$0 path '$.a')) jt;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:15
      │
    2 │ create domain d as int;
      │               ─ 2. destination
    3 │ select * from json_table('[]'::jsonb, '$' columns (a d path '$.a')) jt;
      ╰╴                                                     ─ 1. source
    ");
}

#[test]
fn json_table_exists_column_type() {
    assert_snapshot!(goto("
create domain d as bool;
select * from json_table('[]'::jsonb, '$' columns (e d$0 exists path '$.e')) jt;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:15
      │
    2 │ create domain d as bool;
      │               ─ 2. destination
    3 │ select * from json_table('[]'::jsonb, '$' columns (e d exists path '$.e')) jt;
      ╰╴                                                     ─ 1. source
    ");
}

#[test]
fn xmlserialize_type() {
    assert_snapshot!(goto("
create domain d as text;
select xmlserialize(content '<a/>'::xml as d$0);
"), @"
    info: goto definition
      ╭▸ file0.sql:2:15
      │
    2 │ create domain d as text;
      │               ─ 2. destination
    3 │ select xmlserialize(content '<a/>'::xml as d);
      ╰╴                                           ─ 1. source
    ");
}
