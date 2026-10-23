use insta::assert_snapshot;

use super::goto;

#[test]
fn window_ref_in_over() {
    assert_snapshot!(goto("
create table t (
  a int,
  b int
);
select count(*) over w$0 from t window w as (partition by a);
"), @"
    info: goto definition
      ╭▸ file0.sql:6:22
      │
    6 │ select count(*) over w from t window w as (partition by a);
      ╰╴                     ─ 1. source     ─ 2. destination
    ");
}

#[test]
fn window_ref_in_window_spec() {
    assert_snapshot!(goto("
create table t (
  a int,
  b int
);
select count(*) over (w$0 order by b) from t window w as (partition by a);
"), @"
    info: goto definition
      ╭▸ file0.sql:6:23
      │
    6 │ select count(*) over (w order by b) from t window w as (partition by a);
      ╰╴                      ─ 1. source                 ─ 2. destination
    ");
}

#[test]
fn window_ref_in_window_clause() {
    assert_snapshot!(goto("
create table t (
  a int,
  b int
);
select count(*) over w2
from t
window w1 as (partition by a), w2 as (w1$0 order by b);
"), @"
    info: goto definition
      ╭▸ file0.sql:8:8
      │
    8 │ window w1 as (partition by a), w2 as (w1 order by b);
      ╰╴       ── 2. destination              ── 1. source
    ");
}

#[test]
fn window_partition_by_column() {
    assert_snapshot!(goto("
create table t (
  a int,
  b int
);
select count(*) over w from t window w as (partition by a$0);
"), @"
    info: goto definition
      ╭▸ file0.sql:3:3
      │
    3 │   a int,
      │   ─ 2. destination
      ‡
    6 │ select count(*) over w from t window w as (partition by a);
      ╰╴                                                        ─ 1. source
    ");
}

#[test]
fn window_ref_quoted() {
    assert_snapshot!(goto(r#"
create table t (a int);
select count(*) over "W"$0 from t window "W" as ();
"#), @r#"
    info: goto definition
      ╭▸ file0.sql:3:22
      │
    3 │ select count(*) over "W" from t window "W" as ();
      ╰╴                     ─── 1. source     ─── 2. destination
    "#);
}
