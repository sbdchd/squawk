use insta::assert_snapshot;

use super::goto;

#[test]
fn create_aggregate_sfunc() {
    assert_snapshot!(goto("
create function add(int, int) returns int
  language sql
  return $1 + $2;
create function add(text, text) returns text
  language sql
  return $1 || $2;
create aggregate s(text) (
  sfunc = add$0,
  stype = text
);
"), @"
    info: goto definition
      ╭▸ file0.sql:5:17
      │
    5 │ create function add(text, text) returns text
      │                 ─── 2. destination
      ‡
    9 │   sfunc = add,
      ╰╴          ─── 1. source
    ");
}

#[test]
fn create_aggregate_stype() {
    assert_snapshot!(goto("
create type st as (
  a int
);
create function f(st, int) returns st
  language sql
  return $1;
create aggregate s(int) (
  sfunc = f,
  stype = st$0
);
"), @"
    info: goto definition
       ╭▸ file0.sql:2:13
       │
     2 │ create type st as (
       │             ── 2. destination
       ‡
    10 │   stype = st
       ╰╴          ── 1. source
    ");
}

#[test]
fn create_aggregate_sortop() {
    assert_snapshot!(goto("
create function g(int, int) returns int
  language sql
  return $1;
create operator >>> (
  leftarg = int,
  rightarg = int,
  function = int4gt
);
create aggregate m(int) (
  sfunc = g,
  stype = int,
  sortop = >>>$0
);
"), @"
    info: goto definition
       ╭▸ file0.sql:5:17
       │
     5 │ create operator >>> (
       │                 ─── 2. destination
       ‡
    13 │   sortop = >>>
       ╰╴           ─── 1. source
    ");
}

#[test]
fn ordered_set_aggregate_call_ref() {
    assert_snapshot!(goto("
create function sfn(internal, text) returns internal
  language c
  as 'x';
create function sfn1(internal, float8) returns internal
  language c
  as 'x';
create function ffn(internal, float8) returns text
  language c
  as 'x';
create aggregate pct(float8) (
  sfunc = sfn1,
  stype = internal
);
create aggregate pct(float8 order by text) (
  sfunc = sfn,
  stype = internal,
  finalfunc = ffn
);
select pct$0(0.5) within group (order by 'x'::text);
"), @"
    info: goto definition
       ╭▸ file0.sql:15:18
       │
    15 │ create aggregate pct(float8 order by text) (
       │                  ─── 2. destination
       ‡
    20 │ select pct(0.5) within group (order by 'x'::text);
       ╰╴       ─── 1. source
    ");
}

#[test]
fn ordered_set_aggregate_plain_call_ref() {
    assert_snapshot!(goto("
create function sfn(internal, text) returns internal
  language c
  as 'x';
create function sfn1(internal, float8) returns internal
  language c
  as 'x';
create function ffn(internal, float8) returns text
  language c
  as 'x';
create aggregate pct(float8) (
  sfunc = sfn1,
  stype = internal
);
create aggregate pct(float8 order by text) (
  sfunc = sfn,
  stype = internal,
  finalfunc = ffn
);
select pct$0(0.5);
"), @"
    info: goto definition
       ╭▸ file0.sql:11:18
       │
    11 │ create aggregate pct(float8) (
       │                  ─── 2. destination
       ‡
    20 │ select pct(0.5);
       ╰╴       ─── 1. source
    ");
}

#[test]
fn drop_ordered_set_aggregate_flat_ref() {
    assert_snapshot!(goto("
create function sfn(internal, text) returns internal
  language c
  as 'x';
create function ffn(internal, float8) returns text
  language c
  as 'x';
create aggregate pct(float8 order by text) (
  sfunc = sfn,
  stype = internal,
  finalfunc = ffn
);
drop aggregate pct$0(float8, text);
"), @"
    info: goto definition
       ╭▸ file0.sql:8:18
       │
     8 │ create aggregate pct(float8 order by text) (
       │                  ─── 2. destination
       ‡
    13 │ drop aggregate pct(float8, text);
       ╰╴               ─── 1. source
    ");
}

#[test]
fn old_style_aggregate_call_ref() {
    assert_snapshot!(goto("
create function add(int, int) returns int
  language sql
  return $1 + $2;
create aggregate old (
  basetype = int,
  sfunc = add,
  stype = int
);
create aggregate old (
  basetype = text,
  sfunc = textcat,
  stype = text
);
select old$0(1);
"), @"
    info: goto definition
       ╭▸ file0.sql:5:18
       │
     5 │ create aggregate old (
       │                  ─── 2. destination
       ‡
    15 │ select old(1);
       ╰╴       ─── 1. source
    ");
}

#[test]
fn drop_aggregate_ref() {
    assert_snapshot!(goto("
create function add(int, int) returns int
  language sql
  return $1 + $2;
create aggregate s(int) (
  sfunc = add,
  stype = int
);
drop aggregate s$0(int);
"), @"
    info: goto definition
      ╭▸ file0.sql:5:18
      │
    5 │ create aggregate s(int) (
      │                  ─ 2. destination
      ‡
    9 │ drop aggregate s(int);
      ╰╴               ─ 1. source
    ");
}

#[test]
fn alter_aggregate_rename_ref() {
    assert_snapshot!(goto("
create function add(int, int) returns int
  language sql
  return $1 + $2;
create aggregate s(int) (
  sfunc = add,
  stype = int
);
alter aggregate s(int) rename to t;
select t$0(1);
"), @"
    info: goto definition
       ╭▸ file0.sql:9:34
       │
     9 │ alter aggregate s(int) rename to t;
       │                                  ─ 2. destination
    10 │ select t(1);
       ╰╴       ─ 1. source
    ");
}

#[test]
fn builtin_ordered_set_aggregate() {
    assert_snapshot!(goto("
select percentile_cont$0(0.5) within group (order by 1.5::float8);
"), @"
    info: goto definition
          ╭▸ file0.sql:2:8
          │
        2 │ select percentile_cont(0.5) within group (order by 1.5::float8);
          │        ─────────────── 1. source
          │
          ⸬  builtins.sql:41553:29
          │
    41553 │ create aggregate pg_catalog.percentile_cont(
          ╰╴                            ─────────────── 2. destination
    ");
}

#[test]
fn builtin_hypothetical_set_aggregate() {
    assert_snapshot!(goto("
select rank$0(1) within group (order by 2);
"), @r#"
    info: goto definition
          ╭▸ file0.sql:2:8
          │
        2 │ select rank(1) within group (order by 2);
          │        ──── 1. source
          │
          ⸬  builtins.sql:41667:29
          │
    41667 │ create aggregate pg_catalog.rank(variadic "any" order by variadic "any") (
          ╰╴                            ──── 2. destination
    "#);
}

#[test]
fn builtin_aggregate_sfunc() {
    assert_snapshot!(goto("
select pg_catalog.count$0(1);
"), @r#"
    info: goto definition
          ╭▸ file0.sql:2:19
          │
        2 │ select pg_catalog.count(1);
          │                   ───── 1. source
          │
          ⸬  builtins.sql:42442:29
          │
    42442 │ create aggregate pg_catalog.count("any") (
          ╰╴                            ───── 2. destination
    "#);
}

#[test]
fn drop_aggregate_missing_arg_type_matches_nothing() {
    assert_snapshot!(goto("
drop aggregate sum$0(complex);
"), @"no definition");
}
