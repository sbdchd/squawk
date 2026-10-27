use insta::assert_snapshot;

use super::check;

#[test]
fn delete() {
    assert_snapshot!(check("
create table t(a int, b int);
create table u(c int);
delete from t where a = 1 and z = 2;
delete from t using u where t.a = u.c returning a, c, y;
delete from missing;
"), @"
    error: column `z` does not exist
      ╭▸ 
    4 │ delete from t where a = 1 and z = 2;
      ╰╴                              ━
    error: column `y` does not exist
      ╭▸ 
    5 │ delete from t using u where t.a = u.c returning a, c, y;
      ╰╴                                                      ━
    error: relation `missing` does not exist
      ╭▸ 
    6 │ delete from missing;
      ╰╴            ━━━━━━━
    ");
}

#[test]
fn delete_alias_hides_table_name() {
    assert_snapshot!(check("
create table t(a int);
delete from t as x where x.a = 1 and t.a = 1;
"), @"
    error: missing FROM-clause entry for table `t`
      ╭▸ 
    3 │ delete from t as x where x.a = 1 and t.a = 1;
      ╰╴                                     ━
    ");
}

#[test]
fn delete_target_is_not_a_cte() {
    assert_snapshot!(check("
with t as (select 1 a)
delete from t;
"), @"
    error: relation `t` does not exist
      ╭▸ 
    3 │ delete from t;
      ╰╴            ━
    ");
}

#[test]
fn delete_in_cte() {
    assert_snapshot!(check("
create table t(a int);
with d as (delete from t returning a)
select a, b from d;
"), @"
    error: column `b` does not exist
      ╭▸ 
    4 │ select a, b from d;
      ╰╴          ━
    ");
}

#[test]
fn delete_returning_old_new() {
    assert_snapshot!(check("
create table t(a int);
delete from t returning a, old.a, new.a, old.z, old.*;
delete from t where old.a = 1;
"), @"
    error: column `old.z` does not exist
      ╭▸ 
    3 │ delete from t returning a, old.a, new.a, old.z, old.*;
      ╰╴                                             ━
    error: missing FROM-clause entry for table `old`
      ╭▸ 
    4 │ delete from t where old.a = 1;
      ╰╴                    ━━━
    ");
}

#[test]
fn delete_returning_old_new_renamed() {
    assert_snapshot!(check("
create table t(a int);
delete from t returning with (old as o, new as n) o.a, n.a, old.a;
"), @"
    error: missing FROM-clause entry for table `old`
      ╭▸ 
    3 │ delete from t returning with (old as o, new as n) o.a, n.a, old.a;
      ╰╴                                                            ━━━
    ");
}

#[test]
fn delete_for_portion_of() {
    assert_snapshot!(check("
create table t(id int, valid_at daterange);
delete from t for portion of valid_at from '2020-01-01' to '2021-01-01' where id = 1;
delete from t for portion of nope (daterange(null, nope2));
"), @"
    error: column `t.nope` does not exist
      ╭▸ 
    4 │ delete from t for portion of nope (daterange(null, nope2));
      ╰╴                             ━━━━
    error: column `nope2` does not exist
      ╭▸ 
    4 │ delete from t for portion of nope (daterange(null, nope2));
      ╰╴                                                   ━━━━━
    ");
}
