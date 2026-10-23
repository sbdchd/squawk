use insta::assert_snapshot;

use super::goto;

#[test]
fn create_rule_table() {
    assert_snapshot!(goto("
create table t (a int);
create rule r as on insert
  to t$0
  do instead nothing;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:14
      │
    2 │ create table t (a int);
      │              ─ 2. destination
    3 │ create rule r as on insert
    4 │   to t
      ╰╴     ─ 1. source
    ");
}

#[test]
fn create_rule_where_new() {
    assert_snapshot!(goto("
create table t (a int);
create rule r as on update
  to t
    where new$0.a > 1
  do instead nothing;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:14
      │
    2 │ create table t (a int);
      │              ─ 2. destination
      ‡
    5 │     where new.a > 1
      ╰╴          ─── 1. source
    ");
}

#[test]
fn create_rule_where_old_column() {
    assert_snapshot!(goto("
create table t (a int);
create rule r as on update
  to t
    where old.a$0 > 1
  do instead nothing;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create table t (a int);
      │                 ─ 2. destination
      ‡
    5 │     where old.a > 1
      ╰╴              ─ 1. source
    ");
}

#[test]
fn create_rule_action_new() {
    assert_snapshot!(goto("
create table t (a int);
create table log (a int);
create rule r as on insert
  to t
  do also insert into log values (new$0.a);
"), @"
    info: goto definition
      ╭▸ file0.sql:2:14
      │
    2 │ create table t (a int);
      │              ─ 2. destination
      ‡
    6 │   do also insert into log values (new.a);
      ╰╴                                  ─── 1. source
    ");
}

#[test]
fn create_rule_action_new_column() {
    assert_snapshot!(goto("
create table t (a int);
create table log (b int);
create rule r as on insert
  to t
  do also insert into log values (new.a$0);
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create table t (a int);
      │                 ─ 2. destination
      ‡
    6 │   do also insert into log values (new.a);
      ╰╴                                      ─ 1. source
    ");
}

#[test]
fn create_rule_action_table() {
    assert_snapshot!(goto("
create table t (a int);
create table log (a int);
create rule r as on insert
  to t
  do also (
    insert into log$0 values (new.a)
  );
"), @"
    info: goto definition
      ╭▸ file0.sql:3:14
      │
    3 │ create table log (a int);
      │              ─── 2. destination
      ‡
    7 │     insert into log values (new.a)
      ╰╴                ─── 1. source
    ");
}

#[test]
fn create_rule_name() {
    assert_snapshot!(goto("
create table t (a int);
create rule r$0 as on insert
  to t
  do instead nothing;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:13
      │
    3 │ create rule r as on insert
      │             ┬
      │             │
      │             1. source
      ╰╴            2. destination
    ");
}

#[test]
fn drop_rule_name() {
    assert_snapshot!(goto("
create table t (a int);
create rule r as on insert
  to t
  do instead nothing;
drop rule r$0 on t;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:13
      │
    3 │ create rule r as on insert
      │             ─ 2. destination
      ‡
    6 │ drop rule r on t;
      ╰╴          ─ 1. source
    ");
}

#[test]
fn drop_rule_table() {
    assert_snapshot!(goto("
create table t (a int);
create rule r as on insert
  to t
  do instead nothing;
drop rule r on t$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:14
      │
    2 │ create table t (a int);
      │              ─ 2. destination
      ‡
    6 │ drop rule r on t;
      ╰╴               ─ 1. source
    ");
}

#[test]
fn alter_rule_rename() {
    assert_snapshot!(goto("
create table t (a int);
create rule r as on insert
  to t
  do instead nothing;
alter rule r on t rename to r2;
drop rule r2$0 on t;
"), @"
    info: goto definition
      ╭▸ file0.sql:6:29
      │
    6 │ alter rule r on t rename to r2;
      │                             ── 2. destination
    7 │ drop rule r2 on t;
      ╰╴          ── 1. source
    ");
}

#[test]
fn alter_table_enable_rule() {
    assert_snapshot!(goto("
create table t (a int);
create rule r as on insert
  to t
  do instead nothing;
alter table t enable rule r$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:13
      │
    3 │ create rule r as on insert
      │             ─ 2. destination
      ‡
    6 │ alter table t enable rule r;
      ╰╴                          ─ 1. source
    ");
}

#[test]
fn view_return_rule() {
    assert_snapshot!(goto(r#"
create table t (a int);
create view v as
  select a from t;
drop rule "_RETURN"$0 on v;
"#), @r#"
    info: goto definition
      ╭▸ file0.sql:3:13
      │
    3 │ create view v as
      │             ─ 2. destination
    4 │   select a from t;
    5 │ drop rule "_RETURN" on v;
      ╰╴          ───────── 1. source
    "#);
}

#[test]
fn materialized_view_return_rule() {
    assert_snapshot!(goto(r#"
create table t (a int);
create materialized view m as
  select a from t;
drop rule if exists "_RETURN"$0 on m;
"#), @r#"
    info: goto definition
      ╭▸ file0.sql:3:26
      │
    3 │ create materialized view m as
      │                          ─ 2. destination
    4 │   select a from t;
    5 │ drop rule if exists "_RETURN" on m;
      ╰╴                    ───────── 1. source
    "#);
}
