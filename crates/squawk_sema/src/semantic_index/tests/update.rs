use insta::assert_snapshot;

use super::check;

#[test]
fn update() {
    assert_snapshot!(check("
create table t(a int, b int);
create table u(c int);
update t set a = 1, z = 2 where b = 1 and y = 2;
update t set a = c from u where t.b = u.c returning a, c, x;
update t set (a, b) = (1, w), (v) = (select c from u);
update missing set a = 1;
"), @"
    error: column `t.z` does not exist
      ╭▸ 
    4 │ update t set a = 1, z = 2 where b = 1 and y = 2;
      │                     ━
      ╰╴
    help: a column with a similar name exists
      ╭╴
    4 - update t set a = 1, z = 2 where b = 1 and y = 2;
    4 + update t set a = 1, a = 2 where b = 1 and y = 2;
      ╰╴
    error: column `y` does not exist
      ╭▸ 
    4 │ update t set a = 1, z = 2 where b = 1 and y = 2;
      │                                           ━
      ╰╴
    help: a column with a similar name exists
      ╭╴
    4 - update t set a = 1, z = 2 where b = 1 and y = 2;
    4 + update t set a = 1, z = 2 where b = 1 and a = 2;
      ╰╴
    error: column `x` does not exist
      ╭▸ 
    5 │ update t set a = c from u where t.b = u.c returning a, c, x;
      │                                                           ━
      ╰╴
    help: a column with a similar name exists
      ╭╴
    5 - update t set a = c from u where t.b = u.c returning a, c, x;
    5 + update t set a = c from u where t.b = u.c returning a, c, a;
      ╰╴
    error: column `w` does not exist
      ╭▸ 
    6 │ update t set (a, b) = (1, w), (v) = (select c from u);
      │                           ━
      ╰╴
    help: a column with a similar name exists
      ╭╴
    6 - update t set (a, b) = (1, w), (v) = (select c from u);
    6 + update t set (a, b) = (1, a), (v) = (select c from u);
      ╰╴
    error: column `t.v` does not exist
      ╭▸ 
    6 │ update t set (a, b) = (1, w), (v) = (select c from u);
      │                                ━
      ╰╴
    help: a column with a similar name exists
      ╭╴
    6 - update t set (a, b) = (1, w), (v) = (select c from u);
    6 + update t set (a, b) = (1, w), (a) = (select c from u);
      ╰╴
    error: relation `missing` does not exist
      ╭▸ 
    7 │ update missing set a = 1;
      ╰╴       ━━━━━━━
    ");
}

#[test]
fn update_alias_hides_table_name() {
    assert_snapshot!(check("
create table t(a int);
update t as x set a = x.a + t.a;
"), @"
    error: missing `from` clause entry for table `t`
      ╭▸ 
    3 │ update t as x set a = x.a + t.a;
      ╰╴                            ━
    ");
}

#[test]
fn update_set_target_is_not_qualified_by_from() {
    assert_snapshot!(check("
create table t(a int);
create table u(c int);
update t set c = 1 from u;
"), @"
    error: column `t.c` does not exist
      ╭▸ 
    4 │ update t set c = 1 from u;
      │              ━
      ╰╴
    help: a column with a similar name exists
      ╭╴
    4 - update t set c = 1 from u;
    4 + update t set a = 1 from u;
      ╰╴
    ");
}

#[test]
fn update_target_is_not_a_cte() {
    assert_snapshot!(check("
with t as (select 1 a)
update t set a = 1;
"), @"
    error: relation `t` does not exist
      ╭▸ 
    3 │ update t set a = 1;
      ╰╴       ━
    ");
}

#[test]
fn update_in_cte() {
    assert_snapshot!(check("
create table t(a int);
with i as (update t set a = 1 returning a)
select a, b from i;
"), @"
    error: column `b` does not exist
      ╭▸ 
    4 │ select a, b from i;
      │           ━
      ╰╴
    help: a column with a similar name exists
      ╭╴
    4 - select a, b from i;
    4 + select a, a from i;
      ╰╴
    ");
}

#[test]
fn update_returning_old_new() {
    assert_snapshot!(check("
create table t(a int);
update t set a = 1 returning old.a, new.a, new.z;
"), @"
    error: column `new.z` does not exist
      ╭▸ 
    3 │ update t set a = 1 returning old.a, new.a, new.z;
      │                                                ━
      ╰╴
    help: a column with a similar name exists
      ╭╴
    3 - update t set a = 1 returning old.a, new.a, new.z;
    3 + update t set a = 1 returning old.a, new.a, new.a;
      ╰╴
    ");
}

#[test]
fn update_for_portion_of() {
    assert_snapshot!(check("
create table t(id int, valid_at daterange, b int);
update t for portion of valid_at from '2020-01-01' to '2021-01-01' set b = 1;
update t for portion of nope from '2020-01-01' to nope2 set b = 1;
update t for portion of valid_at (daterange('2020-01-01', nope3)) set b = 1;
"), @"
    error: column `t.nope` does not exist
      ╭▸ 
    4 │ update t for portion of nope from '2020-01-01' to nope2 set b = 1;
      ╰╴                        ━━━━
    error: column `nope2` does not exist
      ╭▸ 
    4 │ update t for portion of nope from '2020-01-01' to nope2 set b = 1;
      ╰╴                                                  ━━━━━
    error: column `nope3` does not exist
      ╭▸ 
    5 │ update t for portion of valid_at (daterange('2020-01-01', nope3)) set b = 1;
      ╰╴                                                          ━━━━━
    ");
}

#[test]
fn update_set_qualified_target() {
    assert_snapshot!(check("
create type pair as (x int, y int);
create table t(a int);
create table u(u pair);
update t set t.a = 1;
update t as x set x.a = 1, t.a = 2;
update u set u.x = 1;
insert into t values (1) on conflict do update set t.a = 2;
"), @"
    error: column `t` of relation `t` does not exist
      ╭▸ 
    5 │ update t set t.a = 1;
      │              ━━
      ╰╴
    help: `set` target columns cannot be qualified with the relation name
      ╭╴
    5 - update t set t.a = 1;
    5 + update t set a = 1;
      ╰╴
    error: column `x` of relation `t` does not exist
      ╭▸ 
    6 │ update t as x set x.a = 1, t.a = 2;
      │                   ━━
      ╰╴
    help: `set` target columns cannot be qualified with the relation name
      ╭╴
    6 - update t as x set x.a = 1, t.a = 2;
    6 + update t as x set a = 1, t.a = 2;
      ╰╴
    error: column `t.t` does not exist
      ╭▸ 
    6 │ update t as x set x.a = 1, t.a = 2;
      │                            ━
      ╰╴
    help: a column with a similar name exists
      ╭╴
    6 - update t as x set x.a = 1, t.a = 2;
    6 + update t as x set x.a = 1, a.a = 2;
      ╰╴
    error: column `t` of relation `t` does not exist
      ╭▸ 
    8 │ insert into t values (1) on conflict do update set t.a = 2;
      │                                                    ━━
      ╰╴
    help: `set` target columns cannot be qualified with the relation name
      ╭╴
    8 - insert into t values (1) on conflict do update set t.a = 2;
    8 + insert into t values (1) on conflict do update set a = 2;
      ╰╴
    ");
}

#[test]
fn update_set_composite_field_missing() {
    assert_snapshot!(check("
create type pair as (x int, y int);
create table t (p pair, q unknown_t, a int[]);
update t set p.x = 1, p.z = 2, q.z = 3, a[1].z = 4;
"), @"
    error: type `unknown_t` does not exist
      ╭▸ 
    3 │ create table t (p pair, q unknown_t, a int[]);
      │                           ━━━━━━━━━
      ╰╴
    help: a type with a similar name exists
      ╭╴
    3 - create table t (p pair, q unknown_t, a int[]);
    3 + create table t (p pair, q unknown, a int[]);
      ╰╴
    error: column `z` does not exist
      ╭▸ 
    4 │ update t set p.x = 1, p.z = 2, q.z = 3, a[1].z = 4;
      ╰╴                        ━
    ");
}

#[test]
fn update_set_array_field_missing() {
    assert_snapshot!(check("
create type pair as (x int, y int);
create table t (a pair[]);
update t set a[1].x = 1, a[1].z = 2, a[1:2].x = 3, a[1][1:2].y = 4, a.x = 5;
"), @"
    error: column `z` does not exist
      ╭▸ 
    4 │ update t set a[1].x = 1, a[1].z = 2, a[1:2].x = 3, a[1][1:2].y = 4, a.x = 5;
      ╰╴                              ━
    error: column `x` does not exist
      ╭▸ 
    4 │ update t set a[1].x = 1, a[1].z = 2, a[1:2].x = 3, a[1][1:2].y = 4, a.x = 5;
      ╰╴                                            ━
    error: column `y` does not exist
      ╭▸ 
    4 │ update t set a[1].x = 1, a[1].z = 2, a[1:2].x = 3, a[1][1:2].y = 4, a.x = 5;
      ╰╴                                                             ━
    error: column `x` does not exist
      ╭▸ 
    4 │ update t set a[1].x = 1, a[1].z = 2, a[1:2].x = 3, a[1][1:2].y = 4, a.x = 5;
      ╰╴                                                                      ━
    ");
}

#[test]
fn duplicate_assignment() {
    assert_snapshot!(check("
create table t (a int[], b int);
update t set b = 1, b = 2;
update t set (a, b) = (null, 1), b = 2;
update t set a[1] = 1, a[2] = 2;
insert into t values ('{1}', 2) on conflict do update set b = 1, b = 2;
"), @"
    error: multiple assignments to same column `b`
      ╭▸ 
    3 │ update t set b = 1, b = 2;
      ╰╴                    ━
    error: multiple assignments to same column `b`
      ╭▸ 
    4 │ update t set (a, b) = (null, 1), b = 2;
      ╰╴                                 ━
    error: multiple assignments to same column `b`
      ╭▸ 
    6 │ insert into t values ('{1}', 2) on conflict do update set b = 1, b = 2;
      ╰╴                                                                 ━
    ");
}

#[test]
fn duplicate_from_source() {
    assert_snapshot!(check("
create table t (a int, b int);
update t set a = 1 from t;
update t x set a = 1 from t;
"), @"
    error: table name `t` specified more than once
      ╭▸ 
    3 │ update t set a = 1 from t;
      ╰╴                        ━
    ");
}
