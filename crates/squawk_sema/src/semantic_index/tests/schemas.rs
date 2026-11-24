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
    error: cannot drop schema `information_schema` because other objects depend on it
      ╭▸ 
    2 │ drop schema information_schema;
      ╰╴            ━━━━━━━━━━━━━━━━━━
    error: schema `information_schema` does not exist
      ╭▸ 
    9 │ select table_name from information_schema.tables;
      ╰╴                       ━━━━━━━━━━━━━━━━━━
    error: relation `tables` does not exist
      ╭▸ 
    9 │ select table_name from information_schema.tables;
      ╰╴                                          ━━━━━━
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
    error: schema `s` already exists
       ╭▸ 
    11 │ alter schema r rename to s;
       ╰╴                         ━
    error: schema `nope` does not exist
       ╭▸ 
    12 │ alter schema nope rename to x;
       ╰╴             ━━━━
    error: role `nope` does not exist
       ╭▸ 
    13 │ alter schema r owner to nope;
       ╰╴                        ━━━━
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

#[test]
fn set_schema_to_same_schema() {
    assert_snapshot!(check("
create schema s;
create table s.w (
  a int,
  b int
);
alter table s.w set schema s;
create view s.v as
  select a from s.w;
alter view s.v set schema s;
create sequence s.q;
alter sequence s.q set schema s;
create function s.f() returns int
  as $$ select 1 $$
  language sql;
alter function s.f() set schema s;
create aggregate s.agg(int) (
  sfunc = int4pl,
  stype = int
);
alter aggregate s.agg(int) set schema s;
create type s.e as enum (
  'a'
);
alter type s.e set schema s;
create domain s.d as int;
alter domain s.d set schema s;
create function s.eq(int, s.e) returns bool
  as $$ select true $$
  language sql;
create operator s.=== (
  leftarg = int,
  rightarg = s.e,
  function = s.eq
);
alter operator s.=== (int, s.e) set schema s;
create collation s.c (
  locale = 'C'
);
alter collation s.c set schema s;
create statistics s.st on a, b from s.w;
alter statistics s.st set schema s;
create text search configuration s.ts (
  copy = english
);
alter text search configuration s.ts set schema s;
create operator family s.fam
  using btree;
alter operator family s.fam using btree set schema s;
create operator class s.cls
  for type int
  using hash as
    operator 1 =;
alter operator class s.cls using hash set schema s;
create conversion s.conv for 'LATIN1' to 'UTF8' from iso8859_1_to_utf8;
alter conversion s.conv set schema s;
"), @"no diagnostics");
}

#[test]
fn if_exists_missing_schema() {
    assert_snapshot!(check("
drop table if exists nos.t;
drop view if exists nos.t;
drop materialized view if exists nos.t;
drop sequence if exists nos.t;
drop index if exists nos.t;
drop type if exists nos.t;
drop domain if exists nos.t;
drop function if exists nos.t;
drop aggregate if exists nos.t(int);
drop routine if exists nos.t(nos.x);
drop cast if exists (nos.x as int);
drop collation if exists nos.t;
drop conversion if exists nos.t;
drop statistics if exists nos.t;
drop text search parser if exists nos.t;
drop operator class if exists nos.t using btree;
drop operator family if exists nos.t using btree;
drop trigger if exists x on nos.t;
drop rule if exists x on nos.t;
drop policy if exists x on nos.t;
alter table if exists nos.t add column b int;
alter index if exists nos.t rename to u;
create function int_add(int, int) returns int
  language sql
  as 'select 1';
create operator @#@ (
  leftarg = int,
  rightarg = int,
  function = int_add
);
drop operator if exists @#@ (nos.x, int);
drop operator if exists nos.@#@ (int, int);
select 1 @#@ 1;
create function foo(int) returns int
  language sql
  as 'select 1';
drop function if exists foo(nos.x);
drop function if exists nos.foo(int);
drop function if exists foo(nos.t.a%type);
select foo(1);
drop table nos.t;
"), @"
    error: schema `nos` does not exist
       ╭▸ 
    42 │ drop table nos.t;
       ╰╴           ━━━
    error: relation `t` does not exist
       ╭▸ 
    42 │ drop table nos.t;
       ╰╴               ━
    ");
}
