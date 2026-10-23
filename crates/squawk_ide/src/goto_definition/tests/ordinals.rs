use insta::assert_snapshot;

use super::goto;

#[test]
fn ordinal_in_compound_order_by() {
    assert_snapshot!(goto("
select 1 a, 2 b
union
select 3, 4
order by 2$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:15
      │
    2 │ select 1 a, 2 b
      │               ─ 2. destination
      ‡
    5 │ order by 2;
      ╰╴         ─ 1. source
    ");
}

#[test]
fn ordinal_in_values_order_by() {
    assert_snapshot!(goto("
values (1, 2), (3, 4) order by 2$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:12
      │
    2 │ values (1, 2), (3, 4) order by 2;
      ╰╴           ─ 2. destination    ─ 1. source
    ");
}

#[test]
fn ordinal_out_of_range() {
    assert_snapshot!(goto("
select 1 a order by 2$0;
"), @"no definition");
}

#[test]
fn ordinal_zero() {
    assert_snapshot!(goto("
select 1 a order by 0$0;
"), @"no definition");
}

#[test]
fn ordinal_in_window_order_by_is_not_an_ordinal() {
    assert_snapshot!(goto("
select 1 a, row_number() over (order by 1$0);
"), @"no definition");
}

#[test]
fn ordinal_in_rollup_with_alias() {
    assert_snapshot!(goto("
create table t (
  a int,
  b int
);
select a x, b from t group by rollup (x, 2$0);
"), @"
    info: goto definition
      ╭▸ file0.sql:6:13
      │
    6 │ select a x, b from t group by rollup (x, 2);
      ╰╴            ─ 2. destination             ─ 1. source
    ");
}

#[test]
fn ordinal_after_unknown_star() {
    assert_snapshot!(goto("
select *, 1 a from foo order by 1$0;
"), @"no definition");
}

#[test]
fn function_returns_composite_ordinal() {
    assert_snapshot!(goto("
create type pair as (
  a int,
  b int
);
create function f() returns pair
  language sql
  as 'select 1, 2';
select * from f() order by 2$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:4:3
      │
    4 │   b int
      │   ─ 2. destination
      ‡
    9 │ select * from f() order by 2;
      ╰╴                           ─ 1. source
    ");
}

#[test]
fn select_target_ordinal_in_distinct_on() {
    assert_snapshot!(goto("
select distinct on (1$0) a from t
"), @"
    info: goto definition
      ╭▸ file0.sql:2:21
      │
    2 │ select distinct on (1) a from t
      │                     ┬  ─ 2. destination
      │                     │
      ╰╴                    1. source
    ");
}

#[test]
fn select_target_ordinal_in_star_across_from_items() {
    assert_snapshot!(goto("
with t as (
  select 1 a, 2 b
),
u as (
  select 3 c, 4 d
)
select * from t, u order by 4$0
"), @"
    info: goto definition
      ╭▸ file0.sql:6:17
      │
    6 │   select 3 c, 4 d
      │                 ─ 2. destination
    7 │ )
    8 │ select * from t, u order by 4
      ╰╴                            ─ 1. source
    ");
}

#[test]
fn select_target_ordinal_in_star_with_alias_columns() {
    assert_snapshot!(goto("
with t as (
  select 1 a, 2 b
)
select * from t as x (y) order by 1$0
"), @"
    info: goto definition
      ╭▸ file0.sql:5:23
      │
    5 │ select * from t as x (y) order by 1
      │                       ┬           ─ 1. source
      │                       │
      ╰╴                      2. destination
    ");
}

#[test]
fn select_target_ordinal_in_star_through_cte_star() {
    assert_snapshot!(goto("
with t as (
  select 1 a, 2 b
),
u as (
  select * from t
)
select * from u order by 2$0
"), @"
    info: goto definition
      ╭▸ file0.sql:3:17
      │
    3 │   select 1 a, 2 b
      │                 ─ 2. destination
      ‡
    8 │ select * from u order by 2
      ╰╴                         ─ 1. source
    ");
}
