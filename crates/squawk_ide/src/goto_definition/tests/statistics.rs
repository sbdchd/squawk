use insta::assert_snapshot;

use super::goto;

#[test]
fn create_statistics_column() {
    assert_snapshot!(goto("
create table t (
  a int,
  b int
);
create statistics s on a$0, b from t;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:3
      │
    3 │   a int,
      │   ─ 2. destination
      ‡
    6 │ create statistics s on a, b from t;
      ╰╴                       ─ 1. source
    ");
}

#[test]
fn create_statistics_table() {
    assert_snapshot!(goto("
create table t (
  a int,
  b int
);
create statistics s on a, b from t$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:14
      │
    2 │ create table t (
      │              ─ 2. destination
      ‡
    6 │ create statistics s on a, b from t;
      ╰╴                                 ─ 1. source
    ");
}

#[test]
fn drop_statistics_ref() {
    assert_snapshot!(goto("
create table t (
  a int,
  b int
);
create statistics s on a, b from t;
drop statistics s$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:6:19
      │
    6 │ create statistics s on a, b from t;
      │                   ─ 2. destination
    7 │ drop statistics s;
      ╰╴                ─ 1. source
    ");
}

#[test]
fn drop_unnamed_statistics() {
    assert_snapshot!(goto("
create table t (
  a int,
  b int
);
create statistics on a, b from t;
drop statistics t_a_b_stat$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:6:19
      │
    6 │ create statistics on a, b from t;
      │                   ─────── 2. destination
    7 │ drop statistics t_a_b_stat;
      ╰╴                ────────── 1. source
    ");
}

#[test]
fn alter_statistics() {
    assert_snapshot!(goto("
create table t (a int);
create statistics s on a from t;
alter statistics s$0 set statistics 100;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:19
      │
    3 │ create statistics s on a from t;
      │                   ─ 2. destination
    4 │ alter statistics s set statistics 100;
      ╰╴                 ─ 1. source
    ");
}
