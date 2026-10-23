use insta::assert_snapshot;

use super::check;

#[test]
fn schema_missing() {
    assert_snapshot!(check("
create table nope.t (a int);
create schema s;
create table s.t (a int);
"), @"
    error: schema `nope` does not exist
      ╭▸ 
    2 │ create table nope.t (a int);
      ╰╴             ━━━━
    ");
}

#[test]
fn schema_rollback() {
    assert_snapshot!(check("
begin;
create schema s;
rollback;
create table s.t (a int);
"), @"
    error: schema `s` does not exist
      ╭▸ 
    5 │ create table s.t (a int);
      ╰╴             ━
    ");
}

#[test]
fn schema_elements() {
    assert_snapshot!(check("
create schema s
  create table t (a int)
  create view v as select a from t
  create index i on t (a);
select a from s.t;
select a from s.v;
select a from t;
"), @"
    error: relation `t` does not exist
      ╭▸ 
    8 │ select a from t;
      ╰╴              ━
    ");
}

#[test]
fn duplicate_schema() {
    assert_snapshot!(check("
create schema s;
create schema s;
create schema if not exists s;
create schema public;
"), @"
    error: schema `s` already exists
      ╭▸ 
    3 │ create schema s;
      ╰╴              ━
    error: schema `public` already exists
      ╭▸ 
    5 │ create schema public;
      ╰╴              ━━━━━━
    ");
}

#[test]
fn create_schema_authorization() {
    assert_snapshot!(check("
create role a;
create schema s authorization a;
create schema s2 authorization missing;
create schema authorization a
  create table t (x int);
select * from a.t;
create schema authorization missing;
create schema authorization current_user;
"), @"
    error: role `missing` does not exist
      ╭▸ 
    4 │ create schema s2 authorization missing;
      ╰╴                               ━━━━━━━
    error: role `missing` does not exist
      ╭▸ 
    8 │ create schema authorization missing;
      ╰╴                            ━━━━━━━
    ");
}

#[test]
fn drop_pg_catalog_schema() {
    assert_snapshot!(check("
drop schema pg_catalog;
drop schema pg_catalog cascade;
drop schema if exists pg_catalog;
select lower('a'), relname from pg_catalog.pg_class;
"), @"
    error: cannot drop schema `pg_catalog` because it is required by the database system
      ╭▸ 
    2 │ drop schema pg_catalog;
      ╰╴            ━━━━━━━━━━
    error: cannot drop schema `pg_catalog` because it is required by the database system
      ╭▸ 
    3 │ drop schema pg_catalog cascade;
      ╰╴            ━━━━━━━━━━
    error: cannot drop schema `pg_catalog` because it is required by the database system
      ╭▸ 
    4 │ drop schema if exists pg_catalog;
      ╰╴                      ━━━━━━━━━━
    ");
}

#[test]
fn drop_information_schema() {
    assert_snapshot!(check("
drop schema information_schema;
select table_name from information_schema.tables;
begin;
drop schema information_schema cascade;
rollback;
select table_name from information_schema.tables;
drop schema information_schema cascade;
select table_name from information_schema.tables;
"), @"
    error: schema `information_schema` does not exist
      ╭▸ 
    9 │ select table_name from information_schema.tables;
      ╰╴                       ━━━━━━━━━━━━━━━━━━
    error: relation `tables` does not exist
      ╭▸ 
    9 │ select table_name from information_schema.tables;
      ╰╴                                          ━━━━━━
    error: cannot drop schema `information_schema` because other objects depend on it
      ╭▸ 
    2 │ drop schema information_schema;
      ╰╴            ━━━━━━━━━━━━━━━━━━
    ");
}

#[test]
fn alter_schema_rename() {
    assert_snapshot!(check("
create schema s;
create table s.t (a int);
create function s.f() returns int language sql return 1;
create domain s.d as int;
alter schema s rename to r;
select a from s.t;
select a from r.t;
select r.f(), 1::r.d;
create schema s;
alter schema r rename to s;
alter schema nope rename to x;
alter schema r owner to nope;
"), @"
    error: schema `s` does not exist
      ╭▸ 
    7 │ select a from s.t;
      ╰╴              ━
    error: relation `t` does not exist
      ╭▸ 
    7 │ select a from s.t;
      ╰╴                ━
    error: schema `nope` does not exist
       ╭▸ 
    12 │ alter schema nope rename to x;
       ╰╴             ━━━━
    error: role `nope` does not exist
       ╭▸ 
    13 │ alter schema r owner to nope;
       ╰╴                        ━━━━
    error: schema `s` already exists
       ╭▸ 
    11 │ alter schema r rename to s;
       ╰╴                         ━
    ");
}

#[test]
fn alter_schema_rename_rollback() {
    assert_snapshot!(check("
create schema s;
create table s.t (a int);
begin;
alter schema s rename to r;
select a from r.t;
rollback;
select a from s.t;
select a from r.t;
"), @"
    error: schema `r` does not exist
      ╭▸ 
    9 │ select a from r.t;
      ╰╴              ━
    error: relation `t` does not exist
      ╭▸ 
    9 │ select a from r.t;
      ╰╴                ━
    ");
}
