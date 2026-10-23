use insta::assert_snapshot;

use super::goto;

#[test]
fn merge_target_table() {
    assert_snapshot!(goto("
create table t (a int);
create table u (c int);
merge into t$0
using u on true
when matched then
  delete;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:14
      │
    2 │ create table t (a int);
      │              ─ 2. destination
    3 │ create table u (c int);
    4 │ merge into t
      ╰╴           ─ 1. source
    ");
}

#[test]
fn merge_source_table() {
    assert_snapshot!(goto("
create table t (a int);
create table u (c int);
merge into t
using u$0 on true
when matched then
  delete;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:14
      │
    3 │ create table u (c int);
      │              ─ 2. destination
    4 │ merge into t
    5 │ using u on true
      ╰╴      ─ 1. source
    ");
}

#[test]
fn merge_on_column() {
    assert_snapshot!(goto("
create table t (a int);
create table u (c int);
merge into t
using u on a = c$0
when matched then
  delete;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:17
      │
    3 │ create table u (c int);
      │                 ─ 2. destination
    4 │ merge into t
    5 │ using u on a = c
      ╰╴               ─ 1. source
    ");
}

#[test]
fn merge_update_set_column() {
    assert_snapshot!(goto("
create table t (a int);
create table u (c int);
merge into t
using u on true
when matched then
  update set a$0 = c;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create table t (a int);
      │                 ─ 2. destination
      ‡
    7 │   update set a = c;
      ╰╴             ─ 1. source
    ");
}

#[test]
fn merge_insert_column() {
    assert_snapshot!(goto("
create table t (a int);
create table u (c int);
merge into t
using u on true
when not matched then
  insert (a$0) values (c);
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create table t (a int);
      │                 ─ 2. destination
      ‡
    7 │   insert (a) values (c);
      ╰╴          ─ 1. source
    ");
}

#[test]
fn merge_insert_value() {
    assert_snapshot!(goto("
create table t (a int);
create table u (c int);
merge into t
using u on true
when not matched then
  insert (a) values (c$0);
"), @"
    info: goto definition
      ╭▸ file0.sql:3:17
      │
    3 │ create table u (c int);
      │                 ─ 2. destination
      ‡
    7 │   insert (a) values (c);
      ╰╴                     ─ 1. source
    ");
}

#[test]
fn merge_alias_qualifier() {
    assert_snapshot!(goto("
create table t (a int);
create table u (c int);
merge into t as x
using u on x$0.a = c
when matched then
  delete;
"), @"
    info: goto definition
      ╭▸ file0.sql:4:17
      │
    4 │ merge into t as x
      │                 ─ 2. destination
    5 │ using u on x.a = c
      ╰╴           ─ 1. source
    ");
}

#[test]
fn merge_returning_column() {
    assert_snapshot!(goto("
create table t (a int);
create table u (c int);
merge into t
using u on true
when matched then
  delete
returning c$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:17
      │
    3 │ create table u (c int);
      │                 ─ 2. destination
      ‡
    8 │ returning c;
      ╰╴          ─ 1. source
    ");
}

#[test]
fn merge_cte_returning_column() {
    assert_snapshot!(goto("
create table t (a int);
create table u (c int);
with m as (
  merge into t
  using u on true
  when matched then
    delete
  returning a
)
select a$0 from m;
"), @"
    info: goto definition
       ╭▸ file0.sql:9:13
       │
     9 │   returning a
       │             ─ 2. destination
    10 │ )
    11 │ select a from m;
       ╰╴       ─ 1. source
    ");
}

#[test]
fn merge_on_ambiguous_column_with_target() {
    assert_snapshot!(goto("
create table t (a int);
create table u (a int);
merge into t
using u on a$0 = 1
when matched then
  do nothing;
"), @"no definition");
}

#[test]
fn merge_using_subquery_hides_target_column() {
    assert_snapshot!(goto("
create table t (
  a int,
  z int
);
create table u (b int);
create table w (y int);
merge into t
using (select a$0) s on true
when matched then
  delete;
"), @"no definition");
}

#[test]
fn merge_not_matched_target_condition_hides_target_column() {
    assert_snapshot!(goto("
create table t (a int);
create table u (b int);
merge into t
using u on true
when not matched and t.a$0 = 1 then
  do nothing;
"), @"no definition");
}

#[test]
fn merge_not_matched_source_condition_hides_source_column() {
    assert_snapshot!(goto("
create table t (a int);
create table u (b int);
merge into t
using u on true
when not matched by source and u.b$0 = 1 then
  do nothing;
"), @"no definition");
}

#[test]
fn merge_not_matched_source_action_sees_target_column() {
    assert_snapshot!(goto("
create table t (a int);
create table u (b int);
merge into t
using u on true
when not matched by source then
  update set a = t.a$0 + 1;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create table t (a int);
      │                 ─ 2. destination
      ‡
    7 │   update set a = t.a + 1;
      ╰╴                   ─ 1. source
    ");
}

#[test]
fn merge_not_matched_by_source_column_not_ambiguous() {
    assert_snapshot!(goto("
create table t (a int);
create table u (a int);
merge into t
using u on t.a = u.a
when not matched by source then
  update set a = a$0 + 1;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create table t (a int);
      │                 ─ 2. destination
      ‡
    7 │   update set a = a + 1;
      ╰╴                 ─ 1. source
    ");
}

#[test]
fn merge_returning_old_column() {
    assert_snapshot!(goto("
create table x (
  a int,
  b int
);
create table y (
  c int,
  d int
);
merge into x as g
using y as k on g.a = c and a = k.c
when matched and g.a = k.c then
  do nothing
returning old.a$0, new.a;
"), @"
    info: goto definition
       ╭▸ file0.sql:3:3
       │
     3 │   a int,
       │   ─ 2. destination
       ‡
    14 │ returning old.a, new.a;
       ╰╴              ─ 1. source
    ");
}
