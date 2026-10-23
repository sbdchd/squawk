use insta::assert_snapshot;

use super::goto;

#[test]
fn transform_type() {
    assert_snapshot!(goto("
create type t as (
  a int
);
create transform for t$0 language plpython3u (from sql with function f(internal));
"), @"
    info: goto definition
      ╭▸ file0.sql:2:13
      │
    2 │ create type t as (
      │             ─ 2. destination
      ‡
    5 │ create transform for t language plpython3u (from sql with function f(internal));
      ╰╴                     ─ 1. source
    ");
}

#[test]
fn transform_function() {
    assert_snapshot!(goto("
create function f(int) returns internal
  language sql
  return null;
create function f(internal) returns internal
  language sql
  return null;
create transform for int language plpython3u (
  from sql with function f$0(internal)
);
"), @"
    info: goto definition
      ╭▸ file0.sql:5:17
      │
    5 │ create function f(internal) returns internal
      │                 ─ 2. destination
      ‡
    9 │   from sql with function f(internal)
      ╰╴                         ─ 1. source
    ");
}

#[test]
fn drop_transform_type() {
    assert_snapshot!(goto("
create type t as (
  a int
);
drop transform for t$0 language plpython3u;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:13
      │
    2 │ create type t as (
      │             ─ 2. destination
      ‡
    5 │ drop transform for t language plpython3u;
      ╰╴                   ─ 1. source
    ");
}

#[test]
fn function_transform_option() {
    assert_snapshot!(goto("
create type t as (
  a int
);
create function f() returns int
  language sql
  transform for type t$0
  return 1;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:13
      │
    2 │ create type t as (
      │             ─ 2. destination
      ‡
    7 │   transform for type t
      ╰╴                     ─ 1. source
    ");
}
