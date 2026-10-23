use insta::assert_snapshot;

use super::goto;

#[test]
fn operator_drop() {
    assert_snapshot!(goto("
create operator === (
  leftarg = int,
  rightarg = text,
  function = f
);
create operator === (
  leftarg = int,
  rightarg = int,
  function = int4eq
);
drop operator ===$0 (int, int);
"), @"
    info: goto definition
       ╭▸ file0.sql:7:17
       │
     7 │ create operator === (
       │                 ─── 2. destination
       ‡
    12 │ drop operator === (int, int);
       ╰╴              ─── 1. source
    ");
}

#[test]
fn operator_qualified_drop() {
    assert_snapshot!(goto("
create schema s;
create operator s.=== (
  leftarg = int,
  rightarg = int,
  function = int4eq
);
drop operator s.===$0 (int, int);
"), @"
    info: goto definition
      ╭▸ file0.sql:3:19
      │
    3 │ create operator s.=== (
      │                   ─── 2. destination
      ‡
    8 │ drop operator s.=== (int, int);
      ╰╴                ─── 1. source
    ");
}

#[test]
fn operator_prefix() {
    assert_snapshot!(goto("
create operator - (
  rightarg = text,
  function = f
);
alter operator -$0 (none, text) set (restrict = eqsel);
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create operator - (
      │                 ─ 2. destination
      ‡
    6 │ alter operator - (none, text) set (restrict = eqsel);
      ╰╴               ─ 1. source
    ");
}

#[test]
fn operator_function() {
    assert_snapshot!(goto("
create function f(int, int) returns bool
  as 'select true'
  language sql;
create function f(int, text) returns bool
  as 'select true'
  language sql;
create operator === (
  leftarg = int,
  rightarg = text,
  function = f$0
);
"), @"
    info: goto definition
       ╭▸ file0.sql:5:17
       │
     5 │ create function f(int, text) returns bool
       │                 ─ 2. destination
       ‡
    11 │   function = f
       ╰╴             ─ 1. source
    ");
}

#[test]
fn operator_arg_type() {
    assert_snapshot!(goto("
create type t as (
  a int
);
create operator === (
  leftarg = t$0,
  rightarg = int,
  function = f
);
"), @"
    info: goto definition
      ╭▸ file0.sql:2:13
      │
    2 │ create type t as (
      │             ─ 2. destination
      ‡
    6 │   leftarg = t,
      ╰╴            ─ 1. source
    ");
}

#[test]
fn operator_bin_expr_builtin() {
    assert_snapshot!(goto("
select 1 +$0 2;
"), @"
    info: goto definition
          ╭▸ file0.sql:2:10
          │
        2 │ select 1 + 2;
          │          ─ 1. source
          │
          ⸬  builtins.sql:33520:28
          │
    33520 │ create operator pg_catalog.+ (
          ╰╴                           ─ 2. destination
    ");
}

#[test]
fn operator_bin_expr_neqb() {
    assert_snapshot!(goto("
select 1 !$0= 2;
"), @"
    info: goto definition
          ╭▸ file0.sql:2:10
          │
        2 │ select 1 != 2;
          │          ── 1. source
          │
          ⸬  builtins.sql:35550:28
          │
    35550 │ create operator pg_catalog.<> (
          ╰╴                           ── 2. destination
    ");
}

#[test]
fn operator_bin_expr_operator_call_schema() {
    assert_snapshot!(goto("
create schema s;
create operator s.=== (
  leftarg = int,
  rightarg = int,
  function = int4eq
);
select 1 operator(s$0.===) 2;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:15
      │
    2 │ create schema s;
      │               ─ 2. destination
      ‡
    8 │ select 1 operator(s.===) 2;
      ╰╴                  ─ 1. source
    ");
}

#[test]
fn keyword_operator_like() {
    assert_snapshot!(goto("
create table t (s text);
select s like$0 'a%' from t;
"), @"
    info: goto definition
          ╭▸ file0.sql:3:10
          │
        3 │ select s like 'a%' from t;
          │          ──── 1. source
          │
          ⸬  builtins.sql:38115:28
          │
    38115 │ create operator pg_catalog.~~ (
          ╰╴                           ── 2. destination
    ");
}

#[test]
fn keyword_operator_not_ilike() {
    assert_snapshot!(goto("
create table t (s text);
select s not il$0ike 'a%' from t;
"), @"
    info: goto definition
          ╭▸ file0.sql:3:10
          │
        3 │ select s not ilike 'a%' from t;
          │          ───────── 1. source
          │
          ⸬  builtins.sql:32681:28
          │
    32681 │ create operator pg_catalog.!~~* (
          ╰╴                           ──── 2. destination
    ");
}

#[test]
fn keyword_operator_like_escape() {
    assert_snapshot!(goto("
create table t (s text);
select s like 'a!%' esc$0ape '!' from t;
"), @"
    info: goto definition
          ╭▸ file0.sql:3:21
          │
        3 │ select s like 'a!%' escape '!' from t;
          │                     ────── 1. source
          │
          ⸬  builtins.sql:17324:28
          │
    17324 │ create function pg_catalog.like_escape(text, text) returns text
          ╰╴                           ─────────── 2. destination
    ");
}

#[test]
fn keyword_operator_similar_to() {
    assert_snapshot!(goto("
create table t (s text);
select s simi$0lar to 'a%' from t;
"), @"
    info: goto definition
          ╭▸ file0.sql:3:10
          │
        3 │ select s similar to 'a%' from t;
          │          ────────── 1. source
          │
          ⸬  builtins.sql:37982:28
          │
    37982 │ create operator pg_catalog.~ (
          ╰╴                           ─ 2. destination
    ");
}

#[test]
fn keyword_operator_not_similar_to_escape() {
    assert_snapshot!(goto("
create table t (s text);
select s not similar to 'a!%' esc$0ape '!' from t;
"), @"
    info: goto definition
          ╭▸ file0.sql:3:31
          │
        3 │ select s not similar to 'a!%' escape '!' from t;
          │                               ────── 1. source
          │
          ⸬  builtins.sql:26747:28
          │
    26747 │ create function pg_catalog.similar_to_escape(text, text) returns text
          ╰╴                           ───────────────── 2. destination
    ");
}

#[test]
fn keyword_operator_in_list() {
    assert_snapshot!(goto("
create table t (i int);
select i i$0n (1, 2) from t;
"), @"
    info: goto definition
          ╭▸ file0.sql:3:10
          │
        3 │ select i in (1, 2) from t;
          │          ── 1. source
          │
          ⸬  builtins.sql:36138:28
          │
    36138 │ create operator pg_catalog.= (
          ╰╴                           ─ 2. destination
    ");
}

#[test]
fn keyword_operator_not_in_subquery() {
    assert_snapshot!(goto("
create table t (
  i int,
  b bigint
);
select i not i$0n (select b from t) from t;
"), @"
    info: goto definition
          ╭▸ file0.sql:6:10
          │
        6 │ select i not in (select b from t) from t;
          │          ────── 1. source
          │
          ⸬  builtins.sql:35543:28
          │
    35543 │ create operator pg_catalog.<> (
          ╰╴                           ── 2. destination
    ");
}

#[test]
fn keyword_operator_is_distinct_from() {
    assert_snapshot!(goto("
create table t (s text);
select s is dis$0tinct from 'a' from t;
"), @"
    info: goto definition
          ╭▸ file0.sql:3:10
          │
        3 │ select s is distinct from 'a' from t;
          │          ──────────────── 1. source
          │
          ⸬  builtins.sql:36299:28
          │
    36299 │ create operator pg_catalog.= (
          ╰╴                           ─ 2. destination
    ");
}

#[test]
fn keyword_operator_any_array() {
    assert_snapshot!(goto("
create table t (
  i int,
  a bigint[]
);
select i =$0 any(a) from t;
"), @"
    info: goto definition
          ╭▸ file0.sql:6:10
          │
        6 │ select i = any(a) from t;
          │          ─ 1. source
          │
          ⸬  builtins.sql:36131:28
          │
    36131 │ create operator pg_catalog.= (
          ╰╴                           ─ 2. destination
    ");
}

#[test]
fn keyword_operator_all_subquery() {
    assert_snapshot!(goto("
create table t (
  i int,
  n numeric
);
select i >$0 all(select n from t) from t;
"), @"
    info: goto definition
          ╭▸ file0.sql:6:10
          │
        6 │ select i > all(select n from t) from t;
          │          ─ 1. source
          │
          ⸬  builtins.sql:36635:28
          │
    36635 │ create operator pg_catalog.> (
          ╰╴                           ─ 2. destination
    ");
}

#[test]
fn keyword_operator_scalar_subquery() {
    assert_snapshot!(goto("
create table t (
  i int,
  s text
);
select s =$0 (select s from t limit 1) from t;
"), @"
    info: goto definition
          ╭▸ file0.sql:6:10
          │
        6 │ select s = (select s from t limit 1) from t;
          │          ─ 1. source
          │
          ⸬  builtins.sql:36299:28
          │
    36299 │ create operator pg_catalog.= (
          ╰╴                           ─ 2. destination
    ");
}

#[test]
fn operator_prefix_expr_builtin() {
    assert_snapshot!(goto("
select -$01.5;
"), @"
    info: goto definition
          ╭▸ file0.sql:2:8
          │
        2 │ select -1.5;
          │        ─ 1. source
          │
          ⸬  builtins.sql:33726:28
          │
    33726 │ create operator pg_catalog.- (
          ╰╴                           ─ 2. destination
    ");
}

#[test]
fn operator_prefix_expr_operator_call() {
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
select operator(~~~$0) 1;
"), @"
    info: goto definition
       ╭▸ file0.sql:7:17
       │
     7 │ create operator ~~~ (
       │                 ─── 2. destination
       ‡
    11 │ select operator(~~~) 1;
       ╰╴                ─── 1. source
    ");
}

#[test]
fn operator_bin_expr_operator_call() {
    assert_snapshot!(goto("
create operator === (
  leftarg = int,
  rightarg = int,
  function = int4eq
);
select 1 operator(===$0) 2;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create operator === (
      │                 ─── 2. destination
      ‡
    7 │ select 1 operator(===) 2;
      ╰╴                  ─── 1. source
    ");
}

#[test]
fn operator_bin_expr_operator_call_qualified() {
    assert_snapshot!(goto("
create schema s;
create operator === (
  leftarg = int,
  rightarg = int,
  function = int4eq
);
create operator s.=== (
  leftarg = int,
  rightarg = int,
  function = int4eq
);
select 1 operator(s.===$0) 2;
"), @"
    info: goto definition
       ╭▸ file0.sql:8:19
       │
     8 │ create operator s.=== (
       │                   ─── 2. destination
       ‡
    13 │ select 1 operator(s.===) 2;
       ╰╴                    ─── 1. source
    ");
}

#[test]
fn operator_commutator() {
    assert_snapshot!(goto("
create operator <<< (
  leftarg = int,
  rightarg = text,
  function = f
);
create operator >>> (
  leftarg = text,
  rightarg = int,
  function = g,
  commutator = <<<$0
);
"), @"
    info: goto definition
       ╭▸ file0.sql:2:17
       │
     2 │ create operator <<< (
       │                 ─── 2. destination
       ‡
    11 │   commutator = <<<
       ╰╴               ─── 1. source
    ");
}

#[test]
fn operator_self_commutator() {
    assert_snapshot!(goto("
create operator === (
  leftarg = int,
  rightarg = int,
  function = int4eq,
  commutator = operator(===$0)
);
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create operator === (
      │                 ─── 2. destination
      ‡
    6 │   commutator = operator(===)
      ╰╴                        ─── 1. source
    ");
}

#[test]
fn operator_builtin() {
    assert_snapshot!(goto("
drop operator pg_catalog.=$0 (int, int);
"), @"
    info: goto definition
          ╭▸ file0.sql:2:26
          │
        2 │ drop operator pg_catalog.= (int, int);
          │                          ─ 1. source
          │
          ⸬  builtins.sql:36138:28
          │
    36138 │ create operator pg_catalog.= (
          ╰╴                           ─ 2. destination
    ");
}

#[test]
fn comment_on_operator_ref() {
    assert_snapshot!(goto("
create function f(int, int) returns int
  language sql
  as 'select 1';
create operator === (
  leftarg = int,
  rightarg = int,
  function = f
);
comment on operator ===$0 (int, int) is 'x';
"), @"
    info: goto definition
       ╭▸ file0.sql:5:17
       │
     5 │ create operator === (
       │                 ─── 2. destination
       ‡
    10 │ comment on operator === (int, int) is 'x';
       ╰╴                    ─── 1. source
    ");
}

#[test]
fn operator_after_set_schema() {
    assert_snapshot!(goto("
create schema s;
create operator === (
  leftarg = int,
  rightarg = int,
  function = int4eq
);
alter operator === (int, int) set schema s;
drop operator s.===$0 (int, int);
"), @"
    info: goto definition
      ╭▸ file0.sql:3:17
      │
    3 │ create operator === (
      │                 ─── 2. destination
      ‡
    9 │ drop operator s.=== (int, int);
      ╰╴                ─── 1. source
    ");
}

#[test]
fn exclusion_operator_call_ref() {
    assert_snapshot!(goto("
create schema s;
create operator s.=== (
  leftarg = int,
  rightarg = int,
  function = int4eq
);
create table t (
  a int,
  exclude (a with operator(s.===$0))
);
"), @"
    info: goto definition
       ╭▸ file0.sql:3:19
       │
     3 │ create operator s.=== (
       │                   ─── 2. destination
       ‡
    10 │   exclude (a with operator(s.===))
       ╰╴                             ─── 1. source
    ");
}

#[test]
fn create_operator_quoted_attribute_names() {
    assert_snapshot!(goto(r#"
create function area_equal(box, box) returns bool
  as 'select true'
  language sql;
create operator === (
  "Leftarg" = box,
  "Rightarg" = box,
  "Procedure" = area_equal$0
);
"#), @r#"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create function area_equal(box, box) returns bool
      │                 ────────── 2. destination
      ‡
    8 │   "Procedure" = area_equal
      ╰╴                ────────── 1. source
    "#);
}
