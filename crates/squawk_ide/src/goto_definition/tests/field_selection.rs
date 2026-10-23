use insta::assert_snapshot;

use super::goto;

#[test]
fn column_notation_column() {
    assert_snapshot!(goto("
create table t (a text);
select a$0 from t;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create table t (a text);
      │                 ─ 2. destination
    3 │ select a from t;
      ╰╴       ─ 1. source
    ");
}

#[test]
fn column_notation_column_with_table() {
    assert_snapshot!(goto("
create table t (a text);
select t.a$0 from t;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create table t (a text);
      │                 ─ 2. destination
    3 │ select t.a from t;
      ╰╴         ─ 1. source
    ");
}

#[test]
fn function_notation_column() {
    assert_snapshot!(goto("
create table t (a text);
select a$0(t) from t;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create table t (a text);
      │                 ─ 2. destination
    3 │ select a(t) from t;
      ╰╴       ─ 1. source
    ");
}

#[test]
fn column_notation_function() {
    assert_snapshot!(goto("
create table t (a text);
create function d(t) returns text
  as 'select 1'
  language sql;
select t.d$0 from t;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:17
      │
    3 │ create function d(t) returns text
      │                 ─ 2. destination
      ‡
    6 │ select t.d from t;
      ╰╴         ─ 1. source
    ");
}

#[test]
fn function_notation_function() {
    assert_snapshot!(goto("
create table t (a text);
create function d(t) returns text
  as 'select 1'
  language sql;
select d$0(t) from t;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:17
      │
    3 │ create function d(t) returns text
      │                 ─ 2. destination
      ‡
    6 │ select d(t) from t;
      ╰╴       ─ 1. source
    ");
}

#[test]
fn whole_row_reference() {
    assert_snapshot!(goto("
create table t (a text);
create function d(t) returns text
  as 'select 1'
  language sql;
select d(t$0) from t;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:14
      │
    2 │ create table t (a text);
      │              ─ 2. destination
      ‡
    6 │ select d(t) from t;
      ╰╴         ─ 1. source
    ");
}

#[test]
fn function_notation_prefers_row_function() {
    assert_snapshot!(goto("
create table t (a text);
create function a(t) returns text
  as 'select 1'
  language sql;
select a$0(t) from t;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:17
      │
    3 │ create function a(t) returns text
      │                 ─ 2. destination
      ‡
    6 │ select a(t) from t;
      ╰╴       ─ 1. source
    ");
}

#[test]
fn column_notation_prefers_column() {
    assert_snapshot!(goto("
create table t (a text);
create function a(t) returns text
  as 'select 1'
  language sql;
select t.a$0 from t;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create table t (a text);
      │                 ─ 2. destination
      ‡
    6 │ select t.a from t;
      ╰╴         ─ 1. source
    ");
}

#[test]
fn field_on_paren_column() {
    assert_snapshot!(goto("
create type pair as (
  x int,
  y int
);
create table t (p pair);
select (p).x$0 from t;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:3
      │
    3 │   x int,
      │   ─ 2. destination
      ‡
    7 │ select (p).x from t;
      ╰╴           ─ 1. source
    ");
}

#[test]
fn field_on_cast() {
    assert_snapshot!(goto("
create type pair as (
  x int,
  y int
);
select (null::pair).y$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:4:3
      │
    4 │   y int
      │   ─ 2. destination
    5 │ );
    6 │ select (null::pair).y;
      ╰╴                    ─ 1. source
    ");
}

#[test]
fn field_on_index_expr() {
    assert_snapshot!(goto("
create type pair as (
  x int,
  y int
);
create table t (ps pair[]);
select (ps[1]).x$0 from t;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:3
      │
    3 │   x int,
      │   ─ 2. destination
      ‡
    7 │ select (ps[1]).x from t;
      ╰╴               ─ 1. source
    ");
}

#[test]
fn field_on_nested_field() {
    assert_snapshot!(goto("
create type inner_t as (
  a int
);
create type outer_t as (
  i inner_t
);
create table t (o outer_t);
select (o).i.a$0 from t;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:3
      │
    3 │   a int
      │   ─ 2. destination
      ‡
    9 │ select (o).i.a from t;
      ╰╴             ─ 1. source
    ");
}

#[test]
fn field_on_whole_row() {
    assert_snapshot!(goto("
create table t (a int);
select (t).a$0 from t;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create table t (a int);
      │                 ─ 2. destination
    3 │ select (t).a from t;
      ╰╴           ─ 1. source
    ");
}

#[test]
fn field_of_function_call_star() {
    assert_snapshot!(goto("
create function f() returns table (a int, b int)
  language sql
  as 'select 1, 2';
select b$0 from (select (f()).*) s;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:43
      │
    2 │ create function f() returns table (a int, b int)
      │                                           ─ 2. destination
      ‡
    5 │ select b from (select (f()).*) s;
      ╰╴       ─ 1. source
    ");
}
