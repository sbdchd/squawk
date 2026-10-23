use insta::assert_snapshot;

use super::goto;

#[test]
fn qualified_column() {
    assert_snapshot!(goto("
with t as (
  select 1 a
)
select t.a$0 from t;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:12
      │
    3 │   select 1 a
      │            ─ 2. destination
    4 │ )
    5 │ select t.a from t;
      ╰╴         ─ 1. source
    ");
}

#[test]
fn qualifier_to_alias() {
    assert_snapshot!(goto("
with t as (
  select 1 a
)
select x$0.a from t as x;
"), @"
    info: goto definition
      ╭▸ file0.sql:5:8
      │
    5 │ select x.a from t as x;
      ╰╴       ─ 1. source   ─ 2. destination
    ");
}

#[test]
fn column_through_star() {
    assert_snapshot!(goto("
with t as (
  select 1 a
),
u as (
  select * from t
)
select a$0 from u;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:12
      │
    3 │   select 1 a
      │            ─ 2. destination
      ‡
    8 │ select a from u;
      ╰╴       ─ 1. source
    ");
}

#[test]
fn subquery_column() {
    assert_snapshot!(goto("
select s.b$0 from (select 1 a, 2 b) s;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:10
      │
    2 │ select s.b from (select 1 a, 2 b) s;
      ╰╴         ─ 1. source           ─ 2. destination
    ");
}

#[test]
fn correlated_column() {
    assert_snapshot!(goto("
with t as (
  select 1 a
),
u as (
  select 2 b
)
select 1 from t where exists(select 1 from u where b = a$0);
"), @"
    info: goto definition
      ╭▸ file0.sql:3:12
      │
    3 │   select 1 a
      │            ─ 2. destination
      ‡
    8 │ select 1 from t where exists(select 1 from u where b = a);
      ╰╴                                                       ─ 1. source
    ");
}

#[test]
fn order_by_output_alias() {
    assert_snapshot!(goto("
with t as (
  select 1 a
)
select a as x from t order by x$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:5:13
      │
    5 │ select a as x from t order by x;
      ╰╴            ─ 2. destination  ─ 1. source
    ");
}

#[test]
fn unresolved_column() {
    assert_snapshot!(goto("
with t as (
  select 1 a
)
select b$0 from t;
"), @"no definition");
}

#[test]
fn from_alias_partial_column_list() {
    assert_snapshot!(goto("
with t as (
  select 1 a, 2 b, 3 c
)
select a, b, c$0 from t as u (x, y);
"), @"
    info: goto definition
      ╭▸ file0.sql:3:22
      │
    3 │   select 1 a, 2 b, 3 c
      │                      ─ 2. destination
    4 │ )
    5 │ select a, b, c from t as u (x, y);
      ╰╴             ─ 1. source
    ");
}

#[test]
fn select_for_update_of_table() {
    assert_snapshot!(goto("
create table t (a int);
select a from t for update of t$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:14
      │
    2 │ create table t (a int);
      │              ─ 2. destination
    3 │ select a from t for update of t;
      ╰╴                              ─ 1. source
    ");
}

#[test]
fn select_for_update_of_alias() {
    assert_snapshot!(goto("
create table t (a int);
create table u (b int);
select a from t as x join u on true for share of u, x$0 nowait;
"), @"
    info: goto definition
      ╭▸ file0.sql:4:20
      │
    4 │ select a from t as x join u on true for share of u, x nowait;
      ╰╴                   ─ 2. destination                 ─ 1. source
    ");
}

#[test]
fn explain_select_column() {
    assert_snapshot!(goto("
create table t (a int);
explain (
  analyze, buffers
)
  select a$0 from t;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create table t (a int);
      │                 ─ 2. destination
      ‡
    6 │   select a from t;
      ╰╴         ─ 1. source
    ");
}

#[test]
fn outer_level_from_subquery_in_from() {
    assert_snapshot!(goto("
create table u (x int);
create table w (y int);
select (select 1 from (select x$0) v) from u;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create table u (x int);
      │                 ─ 2. destination
    3 │ create table w (y int);
    4 │ select (select 1 from (select x) v) from u;
      ╰╴                              ─ 1. source
    ");
}

#[test]
fn outer_level_skips_middle_level() {
    assert_snapshot!(goto("
create table u (x int);
create table w (y int);
select (select (select 1 from (select x$0) v) from w) from u;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create table u (x int);
      │                 ─ 2. destination
    3 │ create table w (y int);
    4 │ select (select (select 1 from (select x) v) from w) from u;
      ╰╴                                      ─ 1. source
    ");
}

#[test]
fn outer_level_nearest_level_wins() {
    assert_snapshot!(goto("
create table u (x int);
create table w (y int);
create table z (x int);
select (select 1 from (select x$0 from z) v) from u;
"), @"
    info: goto definition
      ╭▸ file0.sql:4:17
      │
    4 │ create table z (x int);
      │                 ─ 2. destination
    5 │ select (select 1 from (select x from z) v) from u;
      ╰╴                              ─ 1. source
    ");
}

#[test]
fn outer_level_from_function_arg() {
    assert_snapshot!(goto("
create table u (x int);
create table w (y int);
select (select 1 from generate_series(1, x$0) g) from u;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create table u (x int);
      │                 ─ 2. destination
    3 │ create table w (y int);
    4 │ select (select 1 from generate_series(1, x) g) from u;
      ╰╴                                         ─ 1. source
    ");
}

#[test]
fn outer_level_not_subquery_output_column() {
    assert_snapshot!(goto("
create table u (x int);
create table w (y int);
select (select 1 from (select x$0) v) from w;
"), @"no definition");
}

#[test]
fn outer_level_from_subquery_hides_outer_sibling() {
    assert_snapshot!(goto("
create table u (x int);
create table w (y int);
select * from u, (select (select 1 from (select x$0) v)) q;
"), @"no definition");
}

#[test]
fn select_alias_in_group_by_cube() {
    assert_snapshot!(goto("
create table t (a int);
select a as x from t group by cube (x$0);
"), @"
    info: goto definition
      ╭▸ file0.sql:3:13
      │
    3 │ select a as x from t group by cube (x);
      ╰╴            ─ 2. destination        ─ 1. source
    ");
}

#[test]
fn select_alias_in_group_by_grouping_sets() {
    assert_snapshot!(goto("
create table t (a int);
select a as x from t group by grouping sets ((x$0));
"), @"
    info: goto definition
      ╭▸ file0.sql:3:13
      │
    3 │ select a as x from t group by grouping sets ((x));
      ╰╴            ─ 2. destination                  ─ 1. source
    ");
}

#[test]
fn select_alias_in_distinct_on() {
    assert_snapshot!(goto("
create table t (a int);
select distinct on (x$0) a as x from t;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:21
      │
    3 │ select distinct on (x) a as x from t;
      │                     ┬       ─ 2. destination
      │                     │
      ╰╴                    1. source
    ");
}

#[test]
fn select_alias_group_by_column_name_conflict() {
    assert_snapshot!(goto("
with t as (
  select 2 a
)
select 1 a from t group by a$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:12
      │
    3 │   select 2 a
      │            ─ 2. destination
    4 │ )
    5 │ select 1 a from t group by a;
      ╰╴                           ─ 1. source
    ");
}

#[test]
fn select_alias_order_by_column_name_conflict() {
    assert_snapshot!(goto("
with t as (
  select 2 a
)
select 1 a from t order by a$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:5:10
      │
    5 │ select 1 a from t order by a;
      ╰╴         ─ 2. destination  ─ 1. source
    ");
}

#[test]
fn select_alias_expr_in_order_by_not_found() {
    assert_snapshot!(goto("
with t as (
  select 2 b
)
select 1 a from t order by a$0 + 1
"), @"no definition");
}

#[test]
fn count_star_filter_column() {
    assert_snapshot!(goto("
create table t (a int);
select count(*) filter (where a$0 > 0) from t;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create table t (a int);
      │                 ─ 2. destination
    3 │ select count(*) filter (where a > 0) from t;
      ╰╴                              ─ 1. source
    ");
}

#[test]
fn row_constructor_qualified_star() {
    assert_snapshot!(goto("
create table t (a int, b text);
select row(t$0.*) from t;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:14
      │
    2 │ create table t (a int, b text);
      │              ─ 2. destination
    3 │ select row(t.*) from t;
      ╰╴           ─ 1. source
    ");
}

#[test]
fn cast_qualified_star() {
    assert_snapshot!(goto("
create table t (a int, b text);
select (t$0.*)::text from t;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:14
      │
    2 │ create table t (a int, b text);
      │              ─ 2. destination
    3 │ select (t.*)::text from t;
      ╰╴        ─ 1. source
    ");
}
