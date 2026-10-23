use insta::assert_snapshot;

use super::goto;

#[test]
fn delete_table() {
    assert_snapshot!(goto("
create table t (a int);
delete from t$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:14
      │
    2 │ create table t (a int);
      │              ─ 2. destination
    3 │ delete from t;
      ╰╴            ─ 1. source
    ");
}

#[test]
fn delete_where_column() {
    assert_snapshot!(goto("
create table t (a int);
delete from t where a$0 = 1;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create table t (a int);
      │                 ─ 2. destination
    3 │ delete from t where a = 1;
      ╰╴                    ─ 1. source
    ");
}

#[test]
fn delete_alias_qualifier() {
    assert_snapshot!(goto("
create table t (a int);
delete from t as x where x$0.a = 1;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:18
      │
    3 │ delete from t as x where x.a = 1;
      │                  ┬       ─ 1. source
      │                  │
      ╰╴                 2. destination
    ");
}

#[test]
fn delete_using_column() {
    assert_snapshot!(goto("
create table t (a int);
create table u (c int);
delete from t using u where t.a = c$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:17
      │
    3 │ create table u (c int);
      │                 ─ 2. destination
    4 │ delete from t using u where t.a = c;
      ╰╴                                  ─ 1. source
    ");
}

#[test]
fn delete_returning_column() {
    assert_snapshot!(goto("
create table t (a int);
delete from t returning a$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create table t (a int);
      │                 ─ 2. destination
    3 │ delete from t returning a;
      ╰╴                        ─ 1. source
    ");
}

#[test]
fn delete_cte_returning_column() {
    assert_snapshot!(goto("
create table t (a int);
with d as (
  delete from t returning a
)
select a$0 from d;
"), @"
    info: goto definition
      ╭▸ file0.sql:4:27
      │
    4 │   delete from t returning a
      │                           ─ 2. destination
    5 │ )
    6 │ select a from d;
      ╰╴       ─ 1. source
    ");
}

#[test]
fn delete_returning_old_column() {
    assert_snapshot!(goto("
create table t (a int);
delete from t returning old.a$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create table t (a int);
      │                 ─ 2. destination
    3 │ delete from t returning old.a;
      ╰╴                            ─ 1. source
    ");
}

#[test]
fn delete_returning_old_qualifier() {
    assert_snapshot!(goto("
create table t (a int);
delete from t returning old$0.a;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:14
      │
    2 │ create table t (a int);
      │              ─ 2. destination
    3 │ delete from t returning old.a;
      ╰╴                        ─── 1. source
    ");
}

#[test]
fn delete_returning_renamed_qualifier() {
    assert_snapshot!(goto("
create table t (a int);
delete from t returning with (new as n) n$0.a;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:38
      │
    3 │ delete from t returning with (new as n) n.a;
      │                                      ┬  ─ 1. source
      │                                      │
      ╰╴                                     2. destination
    ");
}

#[test]
fn delete_returning_target_alias_named_old() {
    assert_snapshot!(goto("
create table t (a int);
delete from t as old returning old$0.a;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:18
      │
    3 │ delete from t as old returning old.a;
      │                  ┬──           ─── 1. source
      │                  │
      ╰╴                 2. destination
    ");
}

#[test]
fn delete_for_portion_of_column() {
    assert_snapshot!(goto("
create table t (
  id int,
  valid_at daterange
);
delete from t for portion of valid_at$0 from '2020-01-01' to '2021-01-01';
"), @"
    info: goto definition
      ╭▸ file0.sql:4:3
      │
    4 │   valid_at daterange
      │   ──────── 2. destination
    5 │ );
    6 │ delete from t for portion of valid_at from '2020-01-01' to '2021-01-01';
      ╰╴                             ──────── 1. source
    ");
}

#[test]
fn delete_using_ambiguous_column_with_target() {
    assert_snapshot!(goto("
create table t (a int);
create table u (a int);
delete from t using u where a$0 = 1;
"), @"no definition");
}

#[test]
fn delete_using_function_arg_hides_target_column() {
    assert_snapshot!(goto("
create table t (
  a int,
  z int
);
create table u (b int);
create table w (y int);
delete from t using generate_series(1, a$0) g;
"), @"no definition");
}
