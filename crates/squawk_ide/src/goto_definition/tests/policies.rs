use insta::assert_snapshot;

use super::goto;

#[test]
fn create_policy_table() {
    assert_snapshot!(goto("
create table t (a int);
create policy p on t$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:14
      │
    2 │ create table t (a int);
      │              ─ 2. destination
    3 │ create policy p on t;
      ╰╴                   ─ 1. source
    ");
}

#[test]
fn create_policy_using_column() {
    assert_snapshot!(goto("
create table t (a int);
create policy p on t using (a$0 > 0);
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create table t (a int);
      │                 ─ 2. destination
    3 │ create policy p on t using (a > 0);
      ╰╴                            ─ 1. source
    ");
}

#[test]
fn create_policy_with_check_qualified() {
    assert_snapshot!(goto("
create table t (a int);
create policy p on t with check (t$0.a > 0);
"), @"
    info: goto definition
      ╭▸ file0.sql:2:14
      │
    2 │ create table t (a int);
      │              ─ 2. destination
    3 │ create policy p on t with check (t.a > 0);
      ╰╴                                 ─ 1. source
    ");
}

#[test]
fn create_policy_role() {
    assert_snapshot!(goto("
create role r;
create table t (a int);
create policy p on t to r$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:13
      │
    2 │ create role r;
      │             ─ 2. destination
    3 │ create table t (a int);
    4 │ create policy p on t to r;
      ╰╴                        ─ 1. source
    ");
}

#[test]
fn create_policy_name() {
    assert_snapshot!(goto("
create table t (a int);
create policy p$0 on t;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:15
      │
    3 │ create policy p on t;
      │               ┬
      │               │
      │               1. source
      ╰╴              2. destination
    ");
}

#[test]
fn drop_policy_name() {
    assert_snapshot!(goto("
create table t (a int);
create policy p on t;
drop policy p$0 on t;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:15
      │
    3 │ create policy p on t;
      │               ─ 2. destination
    4 │ drop policy p on t;
      ╰╴            ─ 1. source
    ");
}

#[test]
fn alter_policy_rename() {
    assert_snapshot!(goto("
create table t (a int);
create policy p on t;
alter policy p on t rename to p2;
drop policy p2$0 on t;
"), @"
    info: goto definition
      ╭▸ file0.sql:4:31
      │
    4 │ alter policy p on t rename to p2;
      │                               ── 2. destination
    5 │ drop policy p2 on t;
      ╰╴            ── 1. source
    ");
}

#[test]
fn alter_policy_using_column() {
    assert_snapshot!(goto("
create table t (a int);
create policy p on t;
alter policy p on t using (a$0 > 0);
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create table t (a int);
      │                 ─ 2. destination
    3 │ create policy p on t;
    4 │ alter policy p on t using (a > 0);
      ╰╴                           ─ 1. source
    ");
}

#[test]
fn comment_on_policy_table_ref() {
    assert_snapshot!(goto("
create table t (a int);
create policy p on t;
comment on policy p on t$0 is 'x';
"), @"
    info: goto definition
      ╭▸ file0.sql:2:14
      │
    2 │ create table t (a int);
      │              ─ 2. destination
    3 │ create policy p on t;
    4 │ comment on policy p on t is 'x';
      ╰╴                       ─ 1. source
    ");
}
