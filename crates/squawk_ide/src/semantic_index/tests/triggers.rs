use insta::assert_snapshot;

use super::check;

#[test]
fn create_trigger() {
    assert_snapshot!(check("
create table t (a int, b int);
create table u (c int);
create function f() returns trigger language plpgsql as 'begin return new; end';
create trigger tr after update of a, z on t
  for each row
  when (new.a > old.b and new.zz = 1)
  execute function f('arg');
create constraint trigger tr2 after insert on t from u
  for each row execute procedure f();
"), @"
    error: column `z` does not exist
      ╭▸ 
    5 │ create trigger tr after update of a, z on t
      ╰╴                                     ━
    error: column `zz` does not exist
      ╭▸ 
    7 │   when (new.a > old.b and new.zz = 1)
      ╰╴                              ━━
    ");
}

#[test]
fn create_trigger_unresolved() {
    assert_snapshot!(check("
create trigger tr after insert on missing_table
  for each row execute function missing_fn();
create function f(int) returns trigger language plpgsql as 'begin return new; end';
create table t (a int);
create trigger tr after insert on t
  from missing_from
  for each row execute function f();
"), @"
    error: relation `missing_table` does not exist
      ╭▸ 
    2 │ create trigger tr after insert on missing_table
      ╰╴                                  ━━━━━━━━━━━━━
    error: function `missing_fn` does not exist
      ╭▸ 
    3 │   for each row execute function missing_fn();
      ╰╴                                ━━━━━━━━━━
    error: relation `missing_from` does not exist
      ╭▸ 
    7 │   from missing_from
      ╰╴       ━━━━━━━━━━━━
    error: function `f` does not exist
      ╭▸ 
    8 │   for each row execute function f();
      ╰╴                                ━
    ");
}

#[test]
fn create_trigger_when_unqualified_column() {
    assert_snapshot!(check("
create table t (a int);
create function f() returns trigger language plpgsql as 'begin return new; end';
create trigger tr after update on t
  for each row when (a > 1) execute function f();
"), @"
    error: column reference `a` is ambiguous
      ╭▸ 
    5 │   for each row when (a > 1) execute function f();
      ╰╴                     ━
    ");
}

#[test]
fn duplicate_trigger() {
    assert_snapshot!(check(r#"
create table t (a int);
create table u (a int);
create function f() returns trigger language plpgsql as 'begin return new; end';
create trigger tr after insert on t for each row execute function f();
create trigger tr after insert on t for each row execute function f();
create trigger "TR" after insert on t for each row execute function f();
create trigger tr after insert on u for each row execute function f();
create or replace trigger tr after insert on t for each row execute function f();
"#), @"
    error: trigger `tr` for relation `t` already exists
      ╭▸ 
    6 │ create trigger tr after insert on t for each row execute function f();
      ╰╴               ━━
    ");
}

#[test]
fn duplicate_trigger_after_drop_table() {
    assert_snapshot!(check("
create table t (a int);
create function f() returns trigger language plpgsql as 'begin return new; end';
create trigger tr after insert on t for each row execute function f();
drop table t;
create table t (a int);
create trigger tr after insert on t for each row execute function f();
"), @"no diagnostics");
}

#[test]
fn duplicate_trigger_after_rollback() {
    assert_snapshot!(check("
create table t (a int);
create function f() returns trigger language plpgsql as 'begin return new; end';
begin;
create trigger tr after insert on t for each row execute function f();
rollback;
create trigger tr after insert on t for each row execute function f();
"), @"no diagnostics");
}

#[test]
fn drop_trigger() {
    assert_snapshot!(check("
create table t (a int);
create function f() returns trigger language plpgsql as 'begin return new; end';
create trigger tr after insert on t for each row execute function f();
drop trigger tr on t;
drop trigger tr on t;
drop trigger if exists tr on t;
drop trigger if exists tr on missing_table;
drop trigger tr on missing_table;
create trigger tr after insert on t for each row execute function f();
"), @"
    error: trigger `tr` does not exist
      ╭▸ 
    6 │ drop trigger tr on t;
      ╰╴             ━━
    error: relation `missing_table` does not exist
      ╭▸ 
    9 │ drop trigger tr on missing_table;
      ╰╴                   ━━━━━━━━━━━━━
    ");
}

#[test]
fn drop_trigger_rollback() {
    assert_snapshot!(check("
create table t (a int);
create function f() returns trigger language plpgsql as 'begin return new; end';
create trigger tr after insert on t for each row execute function f();
begin;
drop trigger tr on t;
rollback;
drop trigger tr on t;
drop trigger tr on t;
"), @"
    error: trigger `tr` does not exist
      ╭▸ 
    9 │ drop trigger tr on t;
      ╰╴             ━━
    ");
}

#[test]
fn alter_trigger_rename() {
    assert_snapshot!(check("
create table t (a int);
create function f() returns trigger language plpgsql as 'begin return new; end';
create trigger tr after insert on t for each row execute function f();
create trigger other after insert on t for each row execute function f();
alter trigger tr on t rename to tr2;
alter trigger tr on t rename to tr3;
alter trigger tr2 on t rename to other;
drop trigger tr2 on t;
alter trigger missing on t depends on extension missing_ext;
"), @"
    error: trigger `tr` does not exist
      ╭▸ 
    7 │ alter trigger tr on t rename to tr3;
      ╰╴              ━━
    error: trigger `missing` does not exist
       ╭▸ 
    10 │ alter trigger missing on t depends on extension missing_ext;
       ╰╴              ━━━━━━━
    error: extension `missing_ext` does not exist
       ╭▸ 
    10 │ alter trigger missing on t depends on extension missing_ext;
       ╰╴                                                ━━━━━━━━━━━
    error: trigger `other` for relation `t` already exists
      ╭▸ 
    8 │ alter trigger tr2 on t rename to other;
      ╰╴                                 ━━━━━
    ");
}

#[test]
fn replace_constraint_trigger() {
    assert_snapshot!(check("
create table t (a int);
create function f() returns trigger language plpgsql as 'begin return new; end';
create constraint trigger tr after insert on t for each row execute function f();
create or replace trigger tr after insert on t for each row execute function f();
create or replace constraint trigger tr2 after insert on t for each row execute function f();
"), @"
    error: trigger `tr` for relation `t` is a constraint trigger
      ╭▸ 
    5 │ create or replace trigger tr after insert on t for each row execute function f();
      ╰╴                          ━━
    error: CREATE OR REPLACE CONSTRAINT TRIGGER is not supported
      ╭▸ 
    6 │ create or replace constraint trigger tr2 after insert on t for each row execute function f();
      ╰╴       ━━━━━━━━━━
    ");
}

#[test]
fn alter_table_triggers() {
    assert_snapshot!(check("
create table t (a int);
create function tf() returns trigger language plpgsql as 'begin return new; end';
create trigger tr before insert on t for each row execute function tf();
alter table t disable trigger tr;
alter table t enable trigger tr;
alter table t enable always trigger tr;
alter table t enable replica trigger tr;
alter table t disable trigger all, enable trigger user;
alter table t disable trigger nope;
alter table t enable always trigger nope2;
"), @"
    error: trigger `nope` does not exist
       ╭▸ 
    10 │ alter table t disable trigger nope;
       ╰╴                              ━━━━
    error: trigger `nope2` does not exist
       ╭▸ 
    11 │ alter table t enable always trigger nope2;
       ╰╴                                    ━━━━━
    ");
}
