use insta::assert_snapshot;

use super::check;

#[test]
fn create_statistics() {
    assert_snapshot!(check("
create table t (a int, b int);
create statistics s (ndistinct, dependencies) on a, b from t;
create statistics s2 on a, (b + 1), missing from t;
create statistics s3 on a, b from missing_table;
create statistics s on a, b from t;
create statistics if not exists s on a, b from t;
create statistics missing_schema.s4 on a, b from t;
"), @"
    error: column `missing` does not exist
      ╭▸ 
    4 │ create statistics s2 on a, (b + 1), missing from t;
      ╰╴                                    ━━━━━━━
    error: relation `missing_table` does not exist
      ╭▸ 
    5 │ create statistics s3 on a, b from missing_table;
      ╰╴                                  ━━━━━━━━━━━━━
    error: schema `missing_schema` does not exist
      ╭▸ 
    8 │ create statistics missing_schema.s4 on a, b from t;
      ╰╴                  ━━━━━━━━━━━━━━
    error: statistics object `s` already exists
      ╭▸ 
    6 │ create statistics s on a, b from t;
      ╰╴                  ━
    ");
}

#[test]
fn create_statistics_unnamed() {
    assert_snapshot!(check("
create table t (a int, b int);
create statistics on a, b from t;
create statistics on a, b from t;
create statistics on a, (b + 1) from t;
drop statistics t_a_b_stat;
drop statistics t_a_b_stat1;
drop statistics t_a_expr_stat;
drop statistics t_a_b_stat2;
"), @"
    error: statistics object `t_a_b_stat2` does not exist
      ╭▸ 
    9 │ drop statistics t_a_b_stat2;
      ╰╴                ━━━━━━━━━━━
    ");
}

#[test]
fn alter_statistics() {
    assert_snapshot!(check("
create role r;
create schema sc;
create table t (a int, b int);
create statistics s on a, b from t;
create statistics s2 on a, b from t;
alter statistics s owner to r;
alter statistics s owner to missing_role;
alter statistics s set statistics 100;
alter statistics missing owner to r;
alter statistics s rename to s2;
alter statistics s rename to s3;
alter statistics s3 set schema sc;
alter statistics sc.s3 owner to r;
alter statistics s3 owner to r;
"), @"
    error: role `missing_role` does not exist
      ╭▸ 
    8 │ alter statistics s owner to missing_role;
      ╰╴                            ━━━━━━━━━━━━
    error: statistics object `missing` does not exist
       ╭▸ 
    10 │ alter statistics missing owner to r;
       ╰╴                 ━━━━━━━
    error: statistics object `s3` does not exist
       ╭▸ 
    15 │ alter statistics s3 owner to r;
       ╰╴                 ━━
    error: statistics object `s2` already exists
       ╭▸ 
    11 │ alter statistics s rename to s2;
       ╰╴                             ━━
    ");
}

#[test]
fn drop_statistics() {
    assert_snapshot!(check("
create table t (a int, b int);
create statistics s on a, b from t;
create statistics s2 on a, b from t;
drop statistics s, s2;
drop statistics s;
drop statistics if exists s, missing;
create statistics s3 on a, b from t;
drop table t;
drop statistics s3;
"), @"
    error: statistics object `s` does not exist
      ╭▸ 
    6 │ drop statistics s;
      ╰╴                ━
    error: statistics object `s3` does not exist
       ╭▸ 
    10 │ drop statistics s3;
       ╰╴                ━━
    ");
}

#[test]
fn statistics_rollback() {
    assert_snapshot!(check("
create table t (a int, b int);
create statistics s on a, b from t;
begin;
drop statistics s;
create statistics s2 on a, b from t;
rollback;
drop statistics s;
drop statistics s2;
"), @"
    error: statistics object `s2` does not exist
      ╭▸ 
    9 │ drop statistics s2;
      ╰╴                ━━
    ");
}
