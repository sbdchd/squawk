use insta::assert_snapshot;

use super::check;

#[test]
fn merge() {
    assert_snapshot!(check("
create table t(a int, b int);
create table u(c int, d int);
merge into t using u on t.a = u.c and z = 1
when matched and d > 0 and y = 1 then update set b = d, x = 1
when matched then delete
when not matched then insert (a, w) values (c, v)
when not matched by source then do nothing;
merge into missing using u on true when matched then delete;
merge into t using missing on true when matched then delete;
"), @"
    error: column `z` does not exist
      ╭▸ 
    4 │ merge into t using u on t.a = u.c and z = 1
      ╰╴                                      ━
    error: column `y` does not exist
      ╭▸ 
    5 │ when matched and d > 0 and y = 1 then update set b = d, x = 1
      ╰╴                           ━
    error: column `t.x` does not exist
      ╭▸ 
    5 │ when matched and d > 0 and y = 1 then update set b = d, x = 1
      ╰╴                                                        ━
    error: column `t.w` does not exist
      ╭▸ 
    7 │ when not matched then insert (a, w) values (c, v)
      ╰╴                                 ━
    error: column `v` does not exist
      ╭▸ 
    7 │ when not matched then insert (a, w) values (c, v)
      ╰╴                                               ━
    error: relation `missing` does not exist
      ╭▸ 
    9 │ merge into missing using u on true when matched then delete;
      ╰╴           ━━━━━━━
    error: relation `missing` does not exist
       ╭▸ 
    10 │ merge into t using missing on true when matched then delete;
       ╰╴                   ━━━━━━━
    ");
}

#[test]
fn merge_source_cannot_see_target() {
    assert_snapshot!(check("
create table t(a int);
create table u(c int);
merge into t using (select a from u) s on true when matched then delete;
merge into t using lateral (select t.a) s on true when matched then delete;
"), @"
    error: column `a` does not exist
      ╭▸ 
    4 │ merge into t using (select a from u) s on true when matched then delete;
      ╰╴                           ━
    error: missing FROM-clause entry for table `t`
      ╭▸ 
    5 │ merge into t using lateral (select t.a) s on true when matched then delete;
      ╰╴                                   ━
    ");
}

#[test]
fn merge_not_matched_visibility() {
    assert_snapshot!(check("
create table t(a int);
create table u(c int);
merge into t using u on t.a = u.c
when not matched and a = 1 then insert values (a)
when not matched by target then insert (a) values (t.a)
when not matched by source and c = 1 then update set a = u.c;
"), @"
    error: column `a` does not exist
      ╭▸ 
    5 │ when not matched and a = 1 then insert values (a)
      ╰╴                     ━
    error: column `a` does not exist
      ╭▸ 
    5 │ when not matched and a = 1 then insert values (a)
      ╰╴                                               ━
    error: missing FROM-clause entry for table `t`
      ╭▸ 
    6 │ when not matched by target then insert (a) values (t.a)
      ╰╴                                                   ━
    error: column `c` does not exist
      ╭▸ 
    7 │ when not matched by source and c = 1 then update set a = u.c;
      ╰╴                               ━
    error: missing FROM-clause entry for table `u`
      ╭▸ 
    7 │ when not matched by source and c = 1 then update set a = u.c;
      ╰╴                                                         ━
    ");
}

#[test]
fn merge_alias_hides_table_name() {
    assert_snapshot!(check("
create table t(a int);
create table u(c int);
merge into t as x using u as y on x.a = y.c and t.a = u.c when matched then delete;
"), @"
    error: missing FROM-clause entry for table `t`
      ╭▸ 
    4 │ merge into t as x using u as y on x.a = y.c and t.a = u.c when matched then delete;
      ╰╴                                                ━
    error: missing FROM-clause entry for table `u`
      ╭▸ 
    4 │ merge into t as x using u as y on x.a = y.c and t.a = u.c when matched then delete;
      ╰╴                                                      ━
    ");
}

#[test]
fn merge_target_is_not_a_cte() {
    assert_snapshot!(check("
with t as (select 1 a)
merge into t using t s on true when matched then delete;
"), @"
    error: relation `t` does not exist
      ╭▸ 
    3 │ merge into t using t s on true when matched then delete;
      ╰╴           ━
    ");
}

#[test]
fn merge_returning() {
    assert_snapshot!(check("
create table t(a int);
create table u(c int);
merge into t using u on t.a = u.c
when matched then update set a = c
returning merge_action(), a, c, old.a, new.a, new.c, z;
"), @"
    error: column `new.c` does not exist
      ╭▸ 
    6 │ returning merge_action(), a, c, old.a, new.a, new.c, z;
      ╰╴                                                  ━
    error: column `z` does not exist
      ╭▸ 
    6 │ returning merge_action(), a, c, old.a, new.a, new.c, z;
      ╰╴                                                     ━
    ");
}

#[test]
fn merge_in_cte() {
    assert_snapshot!(check("
create table t(a int);
create table u(c int);
with m as (merge into t using u on t.a = u.c when matched then delete returning a)
select a, b from m;
"), @"
    error: column `b` does not exist
      ╭▸ 
    5 │ select a, b from m;
      ╰╴          ━
    ");
}

#[test]
fn duplicate_assignment_and_source() {
    assert_snapshot!(check("
create table t (a int, b int);
merge into t using t x on true when matched then update set a = 1, a = 2;
merge into t using t on true when matched then do nothing;
"), @"
    error: multiple assignments to same column `a`
      ╭▸ 
    3 │ merge into t using t x on true when matched then update set a = 1, a = 2;
      ╰╴                                                                   ━
    error: table name `t` specified more than once
      ╭▸ 
    4 │ merge into t using t on true when matched then do nothing;
      ╰╴                   ━
    ");
}
