use insta::assert_snapshot;

use super::goto;

#[test]
fn cast_source_type() {
    assert_snapshot!(goto("
create type t as (
  a int
);
create cast (t$0 as text) with inout;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:13
      │
    2 │ create type t as (
      │             ─ 2. destination
      ‡
    5 │ create cast (t as text) with inout;
      ╰╴             ─ 1. source
    ");
}

#[test]
fn cast_function() {
    assert_snapshot!(goto("
create type t as (
  a int
);
create function f(x t) returns text
  language sql
  return 'a';
create cast (t as text) with function f$0(t);
"), @"
    info: goto definition
      ╭▸ file0.sql:5:17
      │
    5 │ create function f(x t) returns text
      │                 ─ 2. destination
      ‡
    8 │ create cast (t as text) with function f(t);
      ╰╴                                      ─ 1. source
    ");
}

#[test]
fn cast_function_overload_by_type() {
    assert_snapshot!(goto("
create function f(x int) returns text
  language sql
  return 'a';
create function f(x bigint) returns text
  language sql
  return 'b';
create cast (bigint as text) with function f$0(int8) as assignment;
"), @"
    info: goto definition
      ╭▸ file0.sql:5:17
      │
    5 │ create function f(x bigint) returns text
      │                 ─ 2. destination
      ‡
    8 │ create cast (bigint as text) with function f(int8) as assignment;
      ╰╴                                           ─ 1. source
    ");
}

#[test]
fn cast_function_schema_qualified() {
    assert_snapshot!(goto("
create schema s;
create function s.f(x int) returns text
  language sql
  return 'a';
create cast (int as text) with function s.f$0(integer) as implicit;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:19
      │
    3 │ create function s.f(x int) returns text
      │                   ─ 2. destination
      ‡
    6 │ create cast (int as text) with function s.f(integer) as implicit;
      ╰╴                                          ─ 1. source
    ");
}

#[test]
fn cast_function_without_args() {
    assert_snapshot!(goto("
create function f(x int) returns text
  language sql
  return 'a';
create cast (int as text) with function f$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create function f(x int) returns text
      │                 ─ 2. destination
      ‡
    5 │ create cast (int as text) with function f;
      ╰╴                                        ─ 1. source
    ");
}

#[test]
fn drop_cast_target_type() {
    assert_snapshot!(goto("
create domain d as int;
drop cast (int as d$0);
"), @"
    info: goto definition
      ╭▸ file0.sql:2:15
      │
    2 │ create domain d as int;
      │               ─ 2. destination
    3 │ drop cast (int as d);
      ╰╴                  ─ 1. source
    ");
}
