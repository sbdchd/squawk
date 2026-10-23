use insta::assert_snapshot;

use super::goto;

#[test]
fn before_semicolon() {
    assert_snapshot!(goto("
create table t (a int);
select a from t;$0
"), @"
    info: goto definition
      ╭▸ file0.sql:2:14
      │
    2 │ create table t (a int);
      │              ─ 2. destination
    3 │ select a from t;
      ╰╴              ─ 1. source
    ");
}

#[test]
fn before_comma() {
    assert_snapshot!(goto("
create table t (a int, b int);
select a,$0 b from t;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create table t (a int, b int);
      │                 ─ 2. destination
    3 │ select a, b from t;
      ╰╴       ─ 1. source
    ");
}

#[test]
fn before_dot() {
    assert_snapshot!(goto("
create table t (a int);
select t.$0a from t;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:14
      │
    2 │ create table t (a int);
      │              ─ 2. destination
    3 │ select t.a from t;
      ╰╴       ─ 1. source
    ");
}

#[test]
fn before_paren_in_call() {
    assert_snapshot!(goto("
create function f() returns int
  language sql
  return 1;
select f($0);
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create function f() returns int
      │                 ─ 2. destination
      ‡
    5 │ select f();
      ╰╴       ─ 1. source
    ");
}

#[test]
fn before_close_paren() {
    assert_snapshot!(goto("
create table t (a int);
select (a)$0 from t;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create table t (a int);
      │                 ─ 2. destination
    3 │ select (a) from t;
      ╰╴        ─ 1. source
    ");
}

#[test]
fn before_bracket() {
    assert_snapshot!(goto("
create table t (a int[]);
select a[$01] from t;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create table t (a int[]);
      │                 ─ 2. destination
    3 │ select a[1] from t;
      ╰╴       ─ 1. source
    ");
}

#[test]
fn before_close_bracket() {
    assert_snapshot!(goto("
create table t (a int[], i int);
select a[i]$0 from t;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:26
      │
    2 │ create table t (a int[], i int);
      │                          ─ 2. destination
    3 │ select a[i] from t;
      ╰╴         ─ 1. source
    ");
}

#[test]
fn on_whitespace() {
    assert_snapshot!(goto("
create table t (a int);
select a $0from t;
"), @"no definition");
}

#[test]
fn inside_dollar_quoted_body() {
    assert_snapshot!(goto("
create table t (a int);
create function f() returns int
  language sql
  as $$ select a$0 from t $$;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create table t (a int);
      │                 ─ 2. destination
      ‡
    5 │   as $$ select a from t $$;
      ╰╴               ─ 1. source
    ");
}
