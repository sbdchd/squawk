use insta::assert_snapshot;

use super::check;

#[test]
fn inherits() {
    assert_snapshot!(check("
create table p (a int);
create table c (b int) inherits (p);
select a, b, nope from c;
create table d () inherits (nope2);
select x from d;
create table e () inherits (pg_class);
select relname, nope3 from e;
"), @"
    error: column `nope` does not exist
      ╭▸ 
    4 │ select a, b, nope from c;
      ╰╴             ━━━━
    error: relation `nope2` does not exist
      ╭▸ 
    5 │ create table d () inherits (nope2);
      ╰╴                            ━━━━━
    error: column `nope3` does not exist
      ╭▸ 
    8 │ select relname, nope3 from e;
      ╰╴                ━━━━━
    ");
}

#[test]
fn like() {
    assert_snapshot!(check("
create table s (a int);
create view v as select 1 as b;
create table t (like s, like v, c int);
select a, b, c, nope from t;
create table u (like nope2);
select x from u;
create type ty as (a int);
create table w (like ty);
select x from w;
create table p (a int);
create table q (like s) inherits (p);
select a, nope3 from q;
"), @"
    error: column `nope` does not exist
      ╭▸ 
    5 │ select a, b, c, nope from t;
      ╰╴                ━━━━
    error: relation `nope2` does not exist
      ╭▸ 
    6 │ create table u (like nope2);
      ╰╴                     ━━━━━
    error: column `nope3` does not exist
       ╭▸ 
    13 │ select a, nope3 from q;
       ╰╴          ━━━━━
    ");
}

#[test]
fn alter_table_inherit_and_of() {
    assert_snapshot!(check("
create type ty as (a int);
create table parent (a int);
create table child (a int);
alter table child inherit parent;
alter table child no inherit parent;
alter table child inherit nope;
alter table child no inherit nope2;
alter table child of ty;
alter table child not of;
alter table child of nope_type;
"), @"
    error: relation `nope` does not exist
      ╭▸ 
    7 │ alter table child inherit nope;
      ╰╴                          ━━━━
    error: relation `nope2` does not exist
      ╭▸ 
    8 │ alter table child no inherit nope2;
      ╰╴                             ━━━━━
    error: type `nope_type` does not exist
       ╭▸ 
    11 │ alter table child of nope_type;
       ╰╴                     ━━━━━━━━━
    ");
}

#[test]
fn drop_parent_requires_cascade() {
    assert_snapshot!(check("
create table t (a int);
create table u () inherits (t);
drop table t;
select a from u;
"), @"
    error: cannot drop table `t` because other objects depend on it
      ╭▸ 
    4 │ drop table t;
      ╰╴           ━
    ");
}

#[test]
fn drop_parent_cascade_drops_children() {
    assert_snapshot!(check("
create table t (a int);
create table u () inherits (t);
create table v () inherits (u);
drop table t cascade;
select a from u;
select a from v;
"), @"
    error: relation `u` does not exist
      ╭▸ 
    6 │ select a from u;
      ╰╴              ━
    error: relation `v` does not exist
      ╭▸ 
    7 │ select a from v;
      ╰╴              ━
    ");
}

#[test]
fn drop_parent_with_children_listed() {
    assert_snapshot!(check("
create table t (a int);
create table u () inherits (t);
drop table t, u;
select a from u;
"), @"
    error: relation `u` does not exist
      ╭▸ 
    5 │ select a from u;
      ╰╴              ━
    ");
}

#[test]
fn drop_parent_after_no_inherit() {
    assert_snapshot!(check("
create table t (a int);
create table u () inherits (t);
alter table u no inherit t;
drop table t;
select a from u;
"), @"no diagnostics");
}

#[test]
fn drop_parent_cascade_rolled_back() {
    assert_snapshot!(check("
create table t (a int);
create table u () inherits (t);
begin;
drop table t cascade;
rollback;
select a from u;
"), @"no diagnostics");
}

#[test]
fn drop_partitioned_table_drops_partitions() {
    assert_snapshot!(check("
create table t (a int) partition by range (a);
create table u
  partition of t
  for values from (1) to (10)
  partition by range (a);
create table v
  partition of u
  for values from (1) to (5);
create index on u (a);
drop table t;
select a from u;
select a from v;
"), @"
    error: relation `u` does not exist
       ╭▸ 
    12 │ select a from u;
       ╰╴              ━
    error: relation `v` does not exist
       ╭▸ 
    13 │ select a from v;
       ╰╴              ━
    ");
}

#[test]
fn drop_partitioned_table_blocked_by_partition_dependent() {
    assert_snapshot!(check("
create table t (a int) partition by range (a);
create table u
  partition of t
  for values from (1) to (10);
create view w as
  select a from u;
drop table t;
select a from u;
"), @"
    error: cannot drop table `t` because other objects depend on it
      ╭▸ 
    8 │ drop table t;
      ╰╴           ━
    ");
}
