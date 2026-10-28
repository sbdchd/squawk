use insta::assert_snapshot;

use super::check;

#[test]
fn unresolved_savepoint() {
    assert_snapshot!(check("
begin;
savepoint a;
savepoint b;
release a;
rollback to b;
commit;
rollback to a;
"), @"
    error: savepoint `b` does not exist
      ╭▸ 
    6 │ rollback to b;
      ╰╴            ━
    error: savepoint `a` does not exist
      ╭▸ 
    8 │ rollback to a;
      ╰╴            ━
    ");
}

#[test]
fn unknown_prepared_transaction() {
    assert_snapshot!(check("
commit prepared 'foo';
rollback prepared 'bar';
"), @"no diagnostics");
}

#[test]
fn rollback_to_savepoint() {
    assert_snapshot!(check("
begin;
create table t (a int);
savepoint sp;
create table u (a int);
drop table t;
rollback to savepoint sp;
select a from t;
select a from u;
create table v (a int);
rollback to sp;
select a from v;
commit;
select a from t;
"), @"
    error: relation `u` does not exist
      ╭▸ 
    9 │ select a from u;
      │               ━
      ╰╴
    help: a relation with a similar name exists
      ╭╴
    9 - select a from u;
    9 + select a from t;
      ╰╴
    error: relation `v` does not exist
       ╭▸ 
    12 │ select a from v;
       │               ━
       ╰╴
    help: a relation with a similar name exists
       ╭╴
    12 - select a from v;
    12 + select a from t;
       ╰╴
    ");
}

#[test]
fn rollback_to_savepoint_search_path() {
    assert_snapshot!(check("
create schema app;
create table app.t (a int);
begin;
savepoint sp;
set local search_path = app;
select a from t;
rollback to sp;
select a from t;
set search_path = app;
savepoint sp2;
set search_path = public;
rollback to sp2;
select a from t;
commit;
select a from t;
"), @"
    error: relation `t` does not exist
      ╭▸ 
    9 │ select a from t;
      ╰╴              ━
    ");
}

#[test]
fn release_savepoint_keeps_changes() {
    assert_snapshot!(check("
begin;
savepoint a;
create table t (a int);
release a;
select a from t;
rollback;
select a from t;
"), @"
    error: relation `t` does not exist
      ╭▸ 
    8 │ select a from t;
      ╰╴              ━
    ");
}

#[test]
fn rollback_and_chain() {
    assert_snapshot!(check("
begin;
rollback and chain;
create table t (a int);
rollback;
select a from t;
begin;
commit and chain;
create table u (a int);
rollback;
select a from u;
"), @"
    error: relation `t` does not exist
      ╭▸ 
    6 │ select a from t;
      ╰╴              ━
    error: relation `u` does not exist
       ╭▸ 
    11 │ select a from u;
       ╰╴              ━
    ");
}

#[test]
fn prepare_transaction_keeps_changes() {
    assert_snapshot!(check("
begin;
create table t (a int);
prepare transaction 'foo';
rollback;
select a from t;
"), @"no diagnostics");
}

#[test]
fn rollback_in_function_body_keeps_outer_transaction() {
    assert_snapshot!(check("
begin;
drop table if exists t;
create function f() returns int language sql as $$
begin;
commit;
$$;
create table t (a int);
rollback;
select a from t;
"), @"
    error: relation `t` does not exist
       ╭▸ 
    10 │ select a from t;
       ╰╴              ━
    ");
}
