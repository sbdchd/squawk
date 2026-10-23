use insta::assert_snapshot;

use super::hover;

#[test]
fn subquery_alias() {
    assert_snapshot!(hover("
select s$0.a from (select 1 a) s;
"), @"
    hover: subquery s(a int4)
      ╭▸ file0.sql:2:8
      │
    2 │ select s.a from (select 1 a) s;
      ╰╴       ─ hover
    ");
}

#[test]
fn subquery_column() {
    assert_snapshot!(hover("
select a$0 from (select 1 a) s;
"), @"
    hover: column a int4
      ╭▸ file0.sql:2:8
      │
    2 │ select a from (select 1 a) s;
      ╰╴       ─ hover
    ");
}

#[test]
fn select_target_alias() {
    assert_snapshot!(hover("
create table t (a int);
select a::text as label$0 from t;
"), @"
    hover: column label text
      ╭▸ file0.sql:3:19
      │
    3 │ select a::text as label from t;
      ╰╴                  ───── hover
    ");
}

#[test]
fn ordinal_order_by() {
    assert_snapshot!(hover("
create table t (
  a int,
  b text
);
select a, b from t order by 2$0;
"), @"
    hover: column b text
      ╭▸ file0.sql:6:29
      │
    6 │ select a, b from t order by 2;
      ╰╴                            ─ hover
    ");
}

#[test]
fn ordinal_group_by() {
    assert_snapshot!(hover("
with t as (
  select 1 a, 2 b
)
select a, max(b) from t group by 1$0;
"), @"
    hover: column a int4
      ╭▸ file0.sql:5:34
      │
    5 │ select a, max(b) from t group by 1;
      ╰╴                                 ─ hover
    ");
}

#[test]
fn values_column() {
    assert_snapshot!(hover("
select column1$0 from (values (1, 'a')) v;
"), @"
    hover: column column1 int4
      ╭▸ file0.sql:2:8
      │
    2 │ select column1 from (values (1, 'a')) v;
      ╰╴       ─────── hover
    ");
}

#[test]
fn case_keyword() {
    assert_snapshot!(hover("
select case$0 when true then 1 end;
"), @"no hover");
}

#[test]
fn no_hover_on_keyword() {
    assert_snapshot!(hover("
select$0 1;
"), @"no hover");
}
