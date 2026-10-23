use insta::assert_snapshot;

use super::goto;

#[test]
fn operator_prefix_over_infix() {
    assert_snapshot!(goto("
create operator ~~~ (
  leftarg = int,
  rightarg = int,
  function = int4mi
);
create operator ~~~ (
  rightarg = int,
  function = int4um
);
create operator ~~~ (
  leftarg = int,
  rightarg = int,
  function = int4pl
);
drop operator ~~~$0 (none, int);
"), @"
    info: goto definition
       ╭▸ file0.sql:7:17
       │
     7 │ create operator ~~~ (
       │                 ─── 2. destination
       ‡
    16 │ drop operator ~~~ (none, int);
       ╰╴              ─── 1. source
    ");
}

#[test]
fn operator_infix_over_prefix() {
    assert_snapshot!(goto("
create operator ~~~ (
  leftarg = int,
  rightarg = int,
  function = int4mi
);
create operator ~~~ (
  rightarg = int,
  function = int4um
);
drop operator ~~~$0 (int, int);
"), @"
    info: goto definition
       ╭▸ file0.sql:2:17
       │
     2 │ create operator ~~~ (
       │                 ─── 2. destination
       ‡
    11 │ drop operator ~~~ (int, int);
       ╰╴              ─── 1. source
    ");
}

#[test]
fn operator_bin_expr_overload() {
    assert_snapshot!(goto("
create operator === (
  leftarg = int,
  rightarg = int,
  function = int4eq
);
create operator === (
  leftarg = text,
  rightarg = text,
  function = texteq
);
select 'a'::text ===$0 'b'::text;
"), @"
    info: goto definition
       ╭▸ file0.sql:7:17
       │
     7 │ create operator === (
       │                 ─── 2. destination
       ‡
    12 │ select 'a'::text === 'b'::text;
       ╰╴                 ─── 1. source
    ");
}

#[test]
fn operator_bin_expr_unknown_literal_assumes_other_type() {
    assert_snapshot!(goto("
create type t;
create operator === (
  leftarg = int,
  rightarg = int,
  function = int4eq
);
create operator === (
  leftarg = t,
  rightarg = t,
  function = f
);
select '1' ===$0 2;
"), @"
    info: goto definition
       ╭▸ file0.sql:3:17
       │
     3 │ create operator === (
       │                 ─── 2. destination
       ‡
    13 │ select '1' === 2;
       ╰╴           ─── 1. source
    ");
}

#[test]
fn operator_result_type_picks_overload() {
    assert_snapshot!(goto("
create function f(int) returns int
  as 'select 1'
  language sql;
create function f(text) returns int
  as 'select 1'
  language sql;
select f$0('a' || 'b');
"), @"
    info: goto definition
      ╭▸ file0.sql:5:17
      │
    5 │ create function f(text) returns int
      │                 ─ 2. destination
      ‡
    8 │ select f('a' || 'b');
      ╰╴       ─ 1. source
    ");
}

#[test]
fn operator_result_type_builtin_arithmetic() {
    assert_snapshot!(goto("
create function f(int) returns int
  as 'select 1'
  language sql;
create function f(bigint) returns int
  as 'select 1'
  language sql;
select f$0(1::bigint + 1);
"), @"
    info: goto definition
      ╭▸ file0.sql:5:17
      │
    5 │ create function f(bigint) returns int
      │                 ─ 2. destination
      ‡
    8 │ select f(1::bigint + 1);
      ╰╴       ─ 1. source
    ");
}

#[test]
fn operator_result_type_polymorphic() {
    assert_snapshot!(goto("
create function f(int[]) returns int
  as 'select 1'
  language sql;
create function f(text[]) returns int
  as 'select 1'
  language sql;
select f$0(array[1] || 2);
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create function f(int[]) returns int
      │                 ─ 2. destination
      ‡
    8 │ select f(array[1] || 2);
      ╰╴       ─ 1. source
    ");
}

#[test]
fn operator_result_type_custom() {
    assert_snapshot!(goto("
create type t as (
  a int
);
create function make_t(int, int) returns t
  as 'select row(1)'
  language sql;
create operator === (
  leftarg = int,
  rightarg = int,
  function = make_t
);
select (1 === 2).a$0;
"), @"
    info: goto definition
       ╭▸ file0.sql:3:3
       │
     3 │   a int
       │   ─ 2. destination
       ‡
    13 │ select (1 === 2).a;
       ╰╴                 ─ 1. source
    ");
}

#[test]
fn operator_result_type_prefix() {
    assert_snapshot!(goto("
create function f(numeric) returns int
  as 'select 1'
  language sql;
create function f(text) returns int
  as 'select 1'
  language sql;
select f$0(-1.5);
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create function f(numeric) returns int
      │                 ─ 2. destination
      ‡
    8 │ select f(-1.5);
      ╰╴       ─ 1. source
    ");
}

#[test]
fn keyword_operator_like_bytea() {
    assert_snapshot!(goto("
create table t (b bytea);
select b like$0 'a%' from t;
"), @"
    info: goto definition
          ╭▸ file0.sql:3:10
          │
        3 │ select b like 'a%' from t;
          │          ──── 1. source
          │
          ⸬  builtins.sql:38094:28
          │
    38094 │ create operator pg_catalog.~~ (
          ╰╴                           ── 2. destination
    ");
}

#[test]
fn keyword_operator_in_list_common_type() {
    assert_snapshot!(goto("
create table t (i int);
select i i$0n (1, 2.5) from t;
"), @"
    info: goto definition
          ╭▸ file0.sql:3:10
          │
        3 │ select i in (1, 2.5) from t;
          │          ── 1. source
          │
          ⸬  builtins.sql:36215:28
          │
    36215 │ create operator pg_catalog.= (
          ╰╴                           ─ 2. destination
    ");
}

#[test]
fn keyword_operator_like_any_unknown() {
    assert_snapshot!(goto("
create table t (s text);
select s like$0 any('{a,b}') from t;
"), @"
    info: goto definition
          ╭▸ file0.sql:3:10
          │
        3 │ select s like any('{a,b}') from t;
          │          ──── 1. source
          │
          ⸬  builtins.sql:38115:28
          │
    38115 │ create operator pg_catalog.~~ (
          ╰╴                           ── 2. destination
    ");
}

#[test]
fn operator_prefix_expr_over_infix() {
    assert_snapshot!(goto("
create operator ~~~ (
  leftarg = int,
  rightarg = int,
  function = int4mi
);
create operator ~~~ (
  rightarg = int,
  function = int4um
);
select ~~~$0 1;
"), @"
    info: goto definition
       ╭▸ file0.sql:7:17
       │
     7 │ create operator ~~~ (
       │                 ─── 2. destination
       ‡
    11 │ select ~~~ 1;
       ╰╴       ─── 1. source
    ");
}

#[test]
fn exclusion_operator_polymorphic_overload() {
    assert_snapshot!(goto("
create table t (
  r tstzrange,
  exclude using gist (r with &&$0)
);
"), @"
    info: goto definition
          ╭▸ file0.sql:4:30
          │
        4 │   exclude using gist (r with &&)
          │                              ── 1. source
          │
          ⸬  builtins.sql:32931:28
          │
    32931 │ create operator pg_catalog.&& (
          ╰╴                           ── 2. destination
    ");
}

#[test]
fn exclusion_operator_user_overload() {
    assert_snapshot!(goto("
create operator === (
  leftarg = int,
  rightarg = int,
  function = int4eq
);
create operator === (
  leftarg = text,
  rightarg = text,
  function = texteq
);
create table t (
  a text,
  exclude (a with ===$0)
);
"), @"
    info: goto definition
       ╭▸ file0.sql:7:17
       │
     7 │ create operator === (
       │                 ─── 2. destination
       ‡
    14 │   exclude (a with ===)
       ╰╴                  ─── 1. source
    ");
}
