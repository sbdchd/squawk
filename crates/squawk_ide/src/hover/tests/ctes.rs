use insta::assert_snapshot;

use super::hover;

#[test]
fn cte_definition() {
    assert_snapshot!(hover("
with t$0 as (
  select 1 a, 'b' b
)
select * from t;
"), @"
    hover: with t(a int4, b text)
      ╭▸ file0.sql:2:6
      │
    2 │ with t as (
      ╰╴     ─ hover
    ");
}

#[test]
fn cte_reference() {
    assert_snapshot!(hover("
with t as (
  select 1 a, 'b' b
)
select * from t$0;
"), @"
    hover: with t(a int4, b text)
      ╭▸ file0.sql:5:15
      │
    5 │ select * from t;
      ╰╴              ─ hover
    ");
}

#[test]
fn cte_column() {
    assert_snapshot!(hover("
with t as (
  select 1 a
)
select a$0 from t;
"), @"
    hover: column t.a int4
      ╭▸ file0.sql:5:8
      │
    5 │ select a from t;
      ╰╴       ─ hover
    ");
}

#[test]
fn cte_qualified_column() {
    assert_snapshot!(hover("
with t as (
  select 1 a
)
select t.a$0 from t;
"), @"
    hover: column t.a int4
      ╭▸ file0.sql:5:10
      │
    5 │ select t.a from t;
      ╰╴         ─ hover
    ");
}

#[test]
fn cte_column_list() {
    assert_snapshot!(hover("
with t(x, y) as (
  select 1, 'b'
)
select y$0 from t;
"), @"
    hover: column t.y text
      ╭▸ file0.sql:5:8
      │
    5 │ select y from t;
      ╰╴       ─ hover
    ");
}

#[test]
fn recursive_cte() {
    assert_snapshot!(hover("
with recursive t(n) as (
  select 1 union all select n + 1 from t
)
select n$0 from t;
"), @"
    hover: column t.n int4
      ╭▸ file0.sql:5:8
      │
    5 │ select n from t;
      ╰╴       ─ hover
    ");
}
