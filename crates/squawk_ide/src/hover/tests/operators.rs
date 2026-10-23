use insta::assert_snapshot;

use super::hover;

#[test]
fn operator() {
    assert_snapshot!(hover("
create function f(int, int) returns bool
  language sql
  return true;
create operator === (
  leftarg = int,
  rightarg = int,
  function = f
);
select 1 ===$0 2;
"), @"
    hover: operator public.===(int4, int4) returns bool
       ╭▸ file0.sql:10:10
       │
    10 │ select 1 === 2;
       ╰╴         ─── hover
    ");
}

#[test]
fn prefix_operator() {
    assert_snapshot!(hover("
create function f(int) returns int
  language sql
  return 1;
create operator @@@ (
  rightarg = int,
  function = f
);
select @@@$0 1;
"), @"
    hover: operator public.@@@(none, int4) returns int4
      ╭▸ file0.sql:9:8
      │
    9 │ select @@@ 1;
      ╰╴       ─── hover
    ");
}

#[test]
fn builtin_operator() {
    assert_snapshot!(hover("
select 1 +$0 2;
"), @"
    hover: operator pg_catalog.+(int4, int4) returns int4
      ╭▸ file0.sql:2:10
      │
    2 │ select 1 + 2;
      │          ─ hover
      │
      ╰ note: add
    ");
}
