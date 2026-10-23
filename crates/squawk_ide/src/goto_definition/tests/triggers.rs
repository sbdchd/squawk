use insta::assert_snapshot;

use super::goto;

#[test]
fn create_trigger_function() {
    assert_snapshot!(goto("
create function f() returns trigger
  language plpgsql
  as 'begin return new; end';
create table t (a int);
create trigger tr
  after insert
  on t
  for each row
  execute function f$0();
"), @"
    info: goto definition
       ╭▸ file0.sql:2:17
       │
     2 │ create function f() returns trigger
       │                 ─ 2. destination
       ‡
    10 │   execute function f();
       ╰╴                   ─ 1. source
    ");
}

#[test]
fn create_trigger_table() {
    assert_snapshot!(goto("
create table t (a int);
create trigger tr
  after insert
  on t$0
  for each row
  execute function f();
"), @"
    info: goto definition
      ╭▸ file0.sql:2:14
      │
    2 │ create table t (a int);
      │              ─ 2. destination
      ‡
    5 │   on t
      ╰╴     ─ 1. source
    ");
}

#[test]
fn create_trigger_update_of_column() {
    assert_snapshot!(goto("
create table t (a int);
create trigger tr
  after update of a$0
  on t
  for each row
  execute function f();
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create table t (a int);
      │                 ─ 2. destination
    3 │ create trigger tr
    4 │   after update of a
      ╰╴                  ─ 1. source
    ");
}

#[test]
fn create_trigger_when_new_column() {
    assert_snapshot!(goto("
create table t (a int);
create trigger tr
  after update
  on t
  for each row
  when (new.a$0 > 1)
  execute function f();
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create table t (a int);
      │                 ─ 2. destination
      ‡
    7 │   when (new.a > 1)
      ╰╴            ─ 1. source
    ");
}

#[test]
fn create_trigger_when_new() {
    assert_snapshot!(goto("
create table t (a int);
create trigger tr
  after update
  on t
  for each row
  when (new$0.a > 1)
  execute function f();
"), @"
    info: goto definition
      ╭▸ file0.sql:2:14
      │
    2 │ create table t (a int);
      │              ─ 2. destination
      ‡
    7 │   when (new.a > 1)
      ╰╴        ─── 1. source
    ");
}

#[test]
fn create_trigger_when_old() {
    assert_snapshot!(goto("
create table t (a int);
create trigger tr
  after update
  on t
  for each row
  when (old$0.a > 1)
  execute function f();
"), @"
    info: goto definition
      ╭▸ file0.sql:2:14
      │
    2 │ create table t (a int);
      │              ─ 2. destination
      ‡
    7 │   when (old.a > 1)
      ╰╴        ─── 1. source
    ");
}

#[test]
fn create_trigger_name() {
    assert_snapshot!(goto("
create table t (a int);
create trigger tr$0
  after insert
  on t
  for each row
  execute function f();
"), @"
    info: goto definition
      ╭▸ file0.sql:3:16
      │
    3 │ create trigger tr
      │                ┬─
      │                │
      │                1. source
      ╰╴               2. destination
    ");
}

#[test]
fn drop_trigger_name() {
    assert_snapshot!(goto("
create table t (a int);
create trigger tr
  after insert
  on t
  for each row
  execute function f();
drop trigger tr$0 on t;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:16
      │
    3 │ create trigger tr
      │                ── 2. destination
      ‡
    8 │ drop trigger tr on t;
      ╰╴             ── 1. source
    ");
}

#[test]
fn drop_trigger_table() {
    assert_snapshot!(goto("
create table t (a int);
create trigger tr
  after insert
  on t
  for each row
  execute function f();
drop trigger tr on t$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:14
      │
    2 │ create table t (a int);
      │              ─ 2. destination
      ‡
    8 │ drop trigger tr on t;
      ╰╴                   ─ 1. source
    ");
}

#[test]
fn drop_trigger_replaced() {
    assert_snapshot!(goto("
create table t (a int);
create trigger tr
  after insert
  on t
  for each row
  execute function f();
create or replace trigger tr
  after update
  on t
  for each row
  execute function f();
drop trigger tr$0 on t;
"), @"
    info: goto definition
       ╭▸ file0.sql:8:27
       │
     8 │ create or replace trigger tr
       │                           ── 2. destination
       ‡
    13 │ drop trigger tr on t;
       ╰╴             ── 1. source
    ");
}

#[test]
fn alter_trigger_renamed() {
    assert_snapshot!(goto("
create table t (a int);
create trigger tr
  after insert
  on t
  for each row
  execute function f();
alter trigger tr on t rename to tr2;
drop trigger tr2$0 on t;
"), @"
    info: goto definition
      ╭▸ file0.sql:8:33
      │
    8 │ alter trigger tr on t rename to tr2;
      │                                 ─── 2. destination
    9 │ drop trigger tr2 on t;
      ╰╴             ─── 1. source
    ");
}

#[test]
fn comment_on_trigger_ref() {
    assert_snapshot!(goto("
create table t (a int);
create function f() returns trigger
  language plpgsql
  as 'begin return new; end';
create trigger tr
  before insert
  on t
  for each row
  execute function f();
comment on trigger tr$0 on t is 'x';
"), @"
    info: goto definition
       ╭▸ file0.sql:6:16
       │
     6 │ create trigger tr
       │                ── 2. destination
       ‡
    11 │ comment on trigger tr on t is 'x';
       ╰╴                   ── 1. source
    ");
}

#[test]
fn alter_table_disable_trigger_ref() {
    assert_snapshot!(goto("
create table t (a int);
create function f() returns trigger
  language plpgsql
  as 'begin return new; end';
create trigger tr
  before insert
  on t
  for each row
  execute function f();
alter table t disable trigger tr$0;
"), @"
    info: goto definition
       ╭▸ file0.sql:6:16
       │
     6 │ create trigger tr
       │                ── 2. destination
       ‡
    11 │ alter table t disable trigger tr;
       ╰╴                              ── 1. source
    ");
}

#[test]
fn create_constraint_trigger_from_table() {
    assert_snapshot!(goto("
create table t (id int);
create table ref_t (id int);
create constraint trigger trg
  after insert
  on t
  from ref_t$0
  for each row
  execute function f();
"), @"
    info: goto definition
      ╭▸ file0.sql:3:14
      │
    3 │ create table ref_t (id int);
      │              ───── 2. destination
      ‡
    7 │   from ref_t
      ╰╴       ───── 1. source
    ");
}

#[test]
fn create_trigger_procedure() {
    assert_snapshot!(goto("
create function a() returns trigger
  language sql
  as 'select 1';
create trigger tr
  before truncate or delete or insert
  on t
  execute procedure a$0();
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create function a() returns trigger
      │                 ─ 2. destination
      ‡
    8 │   execute procedure a();
      ╰╴                    ─ 1. source
    ");
}
