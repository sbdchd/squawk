use insta::assert_snapshot;

use super::goto;

#[test]
fn select_into_table_name() {
    assert_snapshot!(goto("
select 1 as a into t;
select a from t$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:20
      │
    2 │ select 1 as a into t;
      │                    ─ 2. destination
    3 │ select a from t;
      ╰╴              ─ 1. source
    ");
}

#[test]
fn select_into_column() {
    assert_snapshot!(goto("
create table s (a int);
select a into t from s;
select a$0 from t;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:8
      │
    3 │ select a into t from s;
      │        ─ 2. destination
    4 │ select a from t;
      ╰╴       ─ 1. source
    ");
}

#[test]
fn select_into_temp() {
    assert_snapshot!(goto("
select 1 as a into temp t;
select a from pg_temp.t$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:25
      │
    2 │ select 1 as a into temp t;
      │                         ─ 2. destination
    3 │ select a from pg_temp.t;
      ╰╴                      ─ 1. source
    ");
}

#[test]
fn create_table_as_table_name() {
    assert_snapshot!(goto("
create table t as
  select 1 as a;
select a from t$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:14
      │
    2 │ create table t as
      │              ─ 2. destination
    3 │   select 1 as a;
    4 │ select a from t;
      ╰╴              ─ 1. source
    ");
}

#[test]
fn create_table_as_column() {
    assert_snapshot!(goto("
create table s (a int);
create table t as
  select a from s;
select a$0 from t;
"), @"
    info: goto definition
      ╭▸ file0.sql:4:10
      │
    4 │   select a from s;
      │          ─ 2. destination
    5 │ select a from t;
      ╰╴       ─ 1. source
    ");
}

#[test]
fn create_table_as_column_list() {
    assert_snapshot!(goto("
create table t (
  x,
  y
) as
  select 1, 2;
select y$0 from t;
"), @"
    info: goto definition
      ╭▸ file0.sql:4:3
      │
    4 │   y
      │   ─ 2. destination
      ‡
    7 │ select y from t;
      ╰╴       ─ 1. source
    ");
}

#[test]
fn create_table_as_query_source() {
    assert_snapshot!(goto("
create table s (a int);
create table t as
  select a from s$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:14
      │
    2 │ create table s (a int);
      │              ─ 2. destination
    3 │ create table t as
    4 │   select a from s;
      ╰╴                ─ 1. source
    ");
}
