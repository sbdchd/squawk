use insta::assert_snapshot;

use super::check;

#[test]
fn index_dropped_with_table() {
    assert_snapshot!(check("
create table t(a int);
create index idx on t(a);
drop table t;
reindex index idx;
"), @"
    error: index `idx` does not exist
      ╭▸ 
    5 │ reindex index idx;
      ╰╴              ━━━
    ");
}

#[test]
fn constraint_dropped_with_index_backed_index() {
    assert_snapshot!(check("
create table t(a int primary key);
alter table t drop constraint t_pkey;
reindex index t_pkey;
"), @"
    error: index `t_pkey` does not exist
      ╭▸ 
    4 │ reindex index t_pkey;
      ╰╴              ━━━━━━
    ");
}

#[test]
fn drop_extension_drops_members() {
    assert_snapshot!(check("
create extension e;
create table t (a int);
create function f() returns int language sql as 'select 1';
create type ty as enum ('a');
create table u (a int);
alter extension e add table t;
alter extension e add function f();
alter extension e add type ty;
alter extension e add table u;
alter extension e drop table u;
drop extension e;
select * from t;
select f();
select null::ty;
select * from u;
"), @"
    error: relation `t` does not exist
       ╭▸ 
    13 │ select * from t;
       │               ━
       ╰╴
    help: a relation with a similar name exists
       ╭╴
    13 - select * from t;
    13 + select * from u;
       ╰╴
    error: function `f()` does not exist
       ╭▸ 
    14 │ select f();
       ╰╴       ━
    error: type `ty` does not exist
       ╭▸ 
    15 │ select null::ty;
       ╰╴             ━━
    ");
}

#[test]
fn drop_extension_drops_schema_server_publication_members() {
    assert_snapshot!(check("
create extension e;
create schema s;
create table s.t (a int);
create foreign data wrapper w;
create server srv foreign data wrapper w;
create publication p;
alter extension e add schema s;
alter extension e add server srv;
alter extension e add publication p;
drop extension e;
select * from s.t;
drop schema s;
drop server srv;
drop publication p;
"), @"
    error: schema `s` does not exist
       ╭▸ 
    12 │ select * from s.t;
       ╰╴              ━
    error: relation `t` does not exist
       ╭▸ 
    12 │ select * from s.t;
       ╰╴                ━
    error: schema `s` does not exist
       ╭▸ 
    13 │ drop schema s;
       ╰╴            ━
    error: server `srv` does not exist
       ╭▸ 
    14 │ drop server srv;
       ╰╴            ━━━
    error: publication `p` does not exist
       ╭▸ 
    15 │ drop publication p;
       ╰╴                 ━
    ");
}

#[test]
fn drop_extension_drops_transform_member() {
    assert_snapshot!(check("
create language plpython3u;
create function from_int(internal) returns internal language sql return null;
create transform for int language plpython3u (from sql with function from_int(internal));
create extension e;
alter extension e add transform for int language plpython3u;
drop extension e;
drop transform for int language plpython3u;
alter extension nope add transform for int language plpython3u;
"), @"
    error: transform for type `int4` language `plpython3u` does not exist
      ╭▸ 
    8 │ drop transform for int language plpython3u;
      ╰╴                   ━━━
    error: extension `nope` does not exist
      ╭▸ 
    9 │ alter extension nope add transform for int language plpython3u;
      ╰╴                ━━━━
    error: transform for type `int4` language `plpython3u` does not exist
      ╭▸ 
    9 │ alter extension nope add transform for int language plpython3u;
      ╰╴                         ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    ");
}

#[test]
fn set_schema_moves_owned_sequences() {
    assert_snapshot!(check("
create schema s;
create table t (id serial, b int generated always as identity, c int);
create sequence owned owned by t.c;
alter table t set schema s;
alter sequence s.t_id_seq restart;
alter sequence s.t_b_seq restart;
alter sequence s.owned restart;
alter sequence public.t_id_seq restart;
create schema s2;
create sequence s2.t_id_seq;
alter table s.t set schema s2;
select * from s.t;
"), @"
    error: relation `t_id_seq` does not exist
      ╭▸ 
    9 │ alter sequence public.t_id_seq restart;
      ╰╴                      ━━━━━━━━
    error: relation `t_id_seq` already exists
       ╭▸ 
    12 │ alter table s.t set schema s2;
       ╰╴                           ━━
    ");
}

#[test]
fn identity_sequence_dropped_with_table() {
    assert_snapshot!(check("
create table t (id int generated always as identity (sequence name s));
create sequence owned owned by t.id;
drop table t;
alter sequence t_id_seq restart;
alter sequence s restart;
alter sequence owned restart;
"), @"
    error: relation `t_id_seq` does not exist
      ╭▸ 
    5 │ alter sequence t_id_seq restart;
      ╰╴               ━━━━━━━━
    error: relation `s` does not exist
      ╭▸ 
    6 │ alter sequence s restart;
      ╰╴               ━
    error: relation `owned` does not exist
      ╭▸ 
    7 │ alter sequence owned restart;
      ╰╴               ━━━━━
    ");
}

#[test]
fn drop_function_restrict_with_dependents() {
    assert_snapshot!(check("
create table t (a int);
create function f() returns trigger language plpgsql as 'begin return new; end';
create trigger tr before insert on t for each row execute function f();
drop function f();
drop function f() restrict;
drop trigger tr on t;
"), @"
    error: cannot drop function `f` because other objects depend on it
      ╭▸ 
    5 │ drop function f();
      ╰╴              ━
    error: cannot drop function `f` because other objects depend on it
      ╭▸ 
    6 │ drop function f() restrict;
      ╰╴              ━
    ");
}

#[test]
fn drop_function_cascade_trigger() {
    assert_snapshot!(check("
create table t (a int);
create function f() returns trigger language plpgsql as 'begin return new; end';
create trigger tr before insert on t for each row execute function f();
drop function f() cascade;
drop trigger tr on t;
"), @"
    error: trigger `tr` does not exist
      ╭▸ 
    6 │ drop trigger tr on t;
      ╰╴             ━━
    ");
}

#[test]
fn drop_function_cascade_view_and_function() {
    assert_snapshot!(check("
create function f() returns int language sql return 1;
create function g() returns int language sql begin atomic select f(); end;
create function h() returns int language sql return g();
create function k() returns int language sql as 'select f()';
create view v as select f() a;
create view w as select 1 a;
drop function f cascade;
select g();
select h();
select k();
select * from v;
select * from w;
"), @"
    error: function `g()` does not exist
      ╭▸ 
    9 │ select g();
      │        ━
      ╰╴
    help: a function with a similar name exists
      ╭╴
    9 - select g();
    9 + select k();
      ╰╴
    error: function `h()` does not exist
       ╭▸ 
    10 │ select h();
       │        ━
       ╰╴
    help: a function with a similar name exists
       ╭╴
    10 - select h();
    10 + select k();
       ╰╴
    error: relation `v` does not exist
       ╭▸ 
    12 │ select * from v;
       │               ━
       ╰╴
    help: a relation with a similar name exists
       ╭╴
    12 - select * from v;
    12 + select * from w;
       ╰╴
    ");
}

#[test]
fn drop_function_cascade_rollback() {
    assert_snapshot!(check("
create function f() returns int language sql return 1;
create view v as select f() a;
begin;
drop function f cascade;
rollback;
select f(), a from v;
"), @"no diagnostics");
}

#[test]
fn drop_function_cascade_follows_renames() {
    assert_snapshot!(check("
create table t (a int);
create function f() returns trigger language plpgsql as 'begin return new; end';
create trigger tr before insert on t for each row execute function f();
alter trigger tr on t rename to tr2;
alter function f() rename to g;
drop function g;
drop function g cascade;
drop trigger tr2 on t;
"), @"
    error: cannot drop function `g` because other objects depend on it
      ╭▸ 
    7 │ drop function g;
      ╰╴              ━
    error: trigger `tr2` does not exist
      ╭▸ 
    9 │ drop trigger tr2 on t;
      ╰╴             ━━━
    ");
}

#[test]
fn drop_function_cascade_after_replace() {
    assert_snapshot!(check("
create function f() returns int language sql return 1;
create function g() returns int language sql return f();
create or replace function f() returns int language sql return 2;
drop function f;
create or replace function g() returns int language sql return 3;
drop function f;
select g();
"), @"
    error: cannot drop function `f` because other objects depend on it
      ╭▸ 
    5 │ drop function f;
      ╰╴              ━
    ");
}

#[test]
fn drop_function_cascade_other_objects() {
    assert_snapshot!(check("
create table t (a int check (f(a) > 0));
create function f(int) returns int language sql return 1;
create table u (a int, constraint c check (f(a) > 0));
create index i on u (f(a));
create policy p on u using (f(a) > 0);
create function eq(int, int) returns bool language sql return true;
create operator === (leftarg = int, rightarg = int, function = eq);
create function to_text(int) returns text language sql return 'x';
create domain d as int;
create cast (d as text) with function to_text(int);
drop function to_text;
drop function f(int) cascade;
drop function eq cascade;
drop function to_text cascade;
alter table u drop constraint c;
drop index i;
drop policy p on u;
drop operator === (int, int);
"), @"
    error: function `f(int4)` does not exist
      ╭▸ 
    2 │ create table t (a int check (f(a) > 0));
      │                              ━
      │
      ╰ hint: consider adding explicit type casts
    error: cannot drop function `to_text` because other objects depend on it
       ╭▸ 
    12 │ drop function to_text;
       ╰╴              ━━━━━━━
    error: constraint `c` does not exist
       ╭▸ 
    16 │ alter table u drop constraint c;
       ╰╴                              ━
    error: index `i` does not exist
       ╭▸ 
    17 │ drop index i;
       ╰╴           ━
    error: policy `p` does not exist
       ╭▸ 
    18 │ drop policy p on u;
       ╰╴            ━
    error: operator does not exist: `int4 === int4`
       ╭▸ 
    19 │ drop operator === (int, int);
       ╰╴              ━━━
    ");
}

#[test]
fn drop_function_cascade_column_default() {
    assert_snapshot!(check("
create function f() returns int language sql return 1;
create table t (a int default f(), b int);
alter table t alter column b set default f();
drop function f;
drop function f cascade;
select a, b from t;
"), @"
    error: cannot drop function `f` because other objects depend on it
      ╭▸ 
    5 │ drop function f;
      ╰╴              ━
    ");
}

#[test]
fn drop_function_cascade_generated_column() {
    assert_snapshot!(check("
create function f(int) returns int language sql immutable return $1;
create table t (a int, b int generated always as (f(a)) stored, c int);
alter table t add column d int generated always as (f(c)) stored;
drop function f;
drop function f cascade;
select a, c from t;
select b from t;
select d from t;
"), @"
    error: cannot drop function `f` because other objects depend on it
      ╭▸ 
    5 │ drop function f;
      ╰╴              ━
    error: column `b` does not exist
      ╭▸ 
    8 │ select b from t;
      │        ━
      ╰╴
    help: a column with a similar name exists
      ╭╴
    8 - select b from t;
    8 + select a from t;
      ╰╴
    error: column `d` does not exist
      ╭▸ 
    9 │ select d from t;
      │        ━
      ╰╴
    help: a column with a similar name exists
      ╭╴
    9 - select d from t;
    9 + select a from t;
      ╰╴
    ");
}

#[test]
fn drop_function_column_default_removed() {
    assert_snapshot!(check("
create function f() returns int language sql return 1;
create function g(int) returns int language sql immutable return $1;
create table t (a int default f(), b int generated always as (g(a)) stored);
create table u (a int default f());
create table w (a int default f());
create view v as select 1 a;
alter view v alter column a set default f();
alter table t alter column a drop default;
alter table t alter column b drop expression;
alter table u alter column a set default 1;
alter table w drop column a;
alter view v alter column a drop default;
drop function f;
drop function g;
select a, b from t;
"), @"no diagnostics");
}

#[test]
fn drop_function_cascade_column_default_follows_renames() {
    assert_snapshot!(check("
create function f(int) returns int language sql immutable return $1;
create table t (a int, b int generated always as (f(a)) stored);
alter table t rename column b to c;
alter table t rename to u;
drop function f;
drop function f cascade;
select a from u;
select c from u;
"), @"
    error: cannot drop function `f` because other objects depend on it
      ╭▸ 
    6 │ drop function f;
      ╰╴              ━
    error: column `c` does not exist
      ╭▸ 
    9 │ select c from u;
      │        ━
      ╰╴
    help: a column with a similar name exists
      ╭╴
    9 - select c from u;
    9 + select a from u;
      ╰╴
    ");
}

#[test]
fn drop_function_cascade_generated_column_rollback() {
    assert_snapshot!(check("
create function f(int) returns int language sql immutable return $1;
create table t (a int, b int generated always as (f(a)) stored);
begin;
drop function f cascade;
rollback;
select b from t;
drop function f;
"), @"
    error: cannot drop function `f` because other objects depend on it
      ╭▸ 
    8 │ drop function f;
      ╰╴              ━
    ");
}

#[test]
fn drop_function_after_dropping_table_with_default() {
    assert_snapshot!(check("
create function f() returns int language sql return 1;
create table t (a int default f());
drop table t;
drop function f;
"), @"no diagnostics");
}

#[test]
fn drop_table_restrict_with_dependent_views() {
    assert_snapshot!(check("
create table t (a int);
create view v as select a from t;
create view w as select a from v;
create materialized view m as select a from t;
drop table t;
drop table t restrict;
drop view v;
drop view v, w;
drop materialized view m;
drop table t;
"), @"
    error: cannot drop table `t` because other objects depend on it
      ╭▸ 
    6 │ drop table t;
      ╰╴           ━
    error: cannot drop table `t` because other objects depend on it
      ╭▸ 
    7 │ drop table t restrict;
      ╰╴           ━
    error: cannot drop view `v` because other objects depend on it
      ╭▸ 
    8 │ drop view v;
      ╰╴          ━
    ");
}

#[test]
fn drop_tables_referencing_each_other() {
    assert_snapshot!(check("
create table t (a int primary key);
create table u (a int references t);
drop table t, u;
create table n (a int primary key, parent int references n);
drop table n;
"), @"no diagnostics");
}

#[test]
fn drop_table_cascade_views() {
    assert_snapshot!(check("
create table t (a int);
create table u (a int);
create view v as select a from t;
create view w as select a from v;
create view x as select a from u;
drop table t cascade;
select * from v;
select * from w;
select * from x;
"), @"
    error: relation `v` does not exist
      ╭▸ 
    8 │ select * from v;
      │               ━
      ╰╴
    help: a relation with a similar name exists
      ╭╴
    8 - select * from v;
    8 + select * from x;
      ╰╴
    error: relation `w` does not exist
      ╭▸ 
    9 │ select * from w;
      │               ━
      ╰╴
    help: a relation with a similar name exists
      ╭╴
    9 - select * from w;
    9 + select * from x;
      ╰╴
    ");
}

#[test]
fn drop_table_drops_owned_objects() {
    assert_snapshot!(check("
create table t (a int, b int);
create function f() returns trigger language plpgsql as 'begin return new; end';
create function g(int) returns int language sql immutable return $1;
create trigger tr before insert on t for each row execute function f();
create index i on t (g(a));
create policy p on t using (g(a) > 0);
create sequence seq owned by t.a;
create statistics st on a, b from t;
drop table t;
drop function f;
drop function g;
drop index i;
drop sequence seq;
drop statistics st;
"), @"
    error: index `i` does not exist
       ╭▸ 
    13 │ drop index i;
       ╰╴           ━
    error: relation `seq` does not exist
       ╭▸ 
    14 │ drop sequence seq;
       ╰╴              ━━━
    error: statistics object `st` does not exist
       ╭▸ 
    15 │ drop statistics st;
       ╰╴                ━━
    ");
}

#[test]
fn drop_table_with_foreign_key() {
    assert_snapshot!(check("
create table t (a int primary key);
create table u (a int, constraint fk foreign key (a) references t);
drop table t;
drop table t cascade;
select a from u;
alter table u drop constraint fk;
"), @"
    error: cannot drop table `t` because other objects depend on it
      ╭▸ 
    4 │ drop table t;
      ╰╴           ━
    error: constraint `fk` does not exist
      ╭▸ 
    7 │ alter table u drop constraint fk;
      ╰╴                              ━━
    ");
}

#[test]
fn drop_table_with_dependent_function() {
    assert_snapshot!(check("
create table t (a int);
create function f() returns setof t language sql begin atomic select * from t; end;
drop table t;
drop table t cascade;
select f();
"), @"
    error: cannot drop table `t` because other objects depend on it
      ╭▸ 
    4 │ drop table t;
      ╰╴           ━
    error: function `f()` does not exist
      ╭▸ 
    6 │ select f();
      ╰╴       ━
    ");
}

#[test]
fn drop_view_after_replace() {
    assert_snapshot!(check("
create function f() returns int language sql return 1;
create view v as select f() a;
create view w as select a from v;
create or replace view v as select 1 a, 2 b;
drop function f;
drop view v;
drop view v cascade;
select * from w;
"), @"
    error: cannot drop function `f` because other objects depend on it
      ╭▸ 
    6 │ drop function f;
      ╰╴              ━
    error: cannot drop view `v` because other objects depend on it
      ╭▸ 
    7 │ drop view v;
      ╰╴          ━
    error: relation `w` does not exist
      ╭▸ 
    9 │ select * from w;
      ╰╴              ━
    ");
}

#[test]
fn drop_table_cascade_rollback() {
    assert_snapshot!(check("
create table t (a int);
create view v as select a from t;
begin;
drop table t cascade;
rollback;
select a from v;
drop table t;
"), @"
    error: cannot drop table `t` because other objects depend on it
      ╭▸ 
    8 │ drop table t;
      ╰╴           ━
    ");
}

#[test]
fn drop_function_cascade_through_view() {
    assert_snapshot!(check("
create function f() returns int language sql return 1;
create view v as select f() a;
create view w as select a from v;
drop function f cascade;
select * from w;
"), @"
    error: relation `w` does not exist
      ╭▸ 
    6 │ select * from w;
      ╰╴              ━
    ");
}

#[test]
fn drop_schema_with_extension() {
    assert_snapshot!(check("
create schema s;
create extension hstore with schema s;
drop schema s;
drop schema s cascade;
drop extension hstore;
create extension pg_trgm;
begin;
drop schema public cascade;
rollback;
drop extension pg_trgm;
"), @"
    error: cannot drop schema `s` because other objects depend on it
      ╭▸ 
    4 │ drop schema s;
      ╰╴            ━
    error: extension `hstore` does not exist
      ╭▸ 
    6 │ drop extension hstore;
      ╰╴               ━━━━━━
    ");
}

#[test]
fn drop_schema_after_moving_extension() {
    assert_snapshot!(check("
create schema s2;
create schema s3;
create extension citext with schema s2;
alter extension citext set schema s3;
drop schema s2;
drop schema s3;
alter schema s3 rename to s4;
drop schema s4 cascade;
drop extension citext;
"), @"
    error: cannot drop schema `s3` because other objects depend on it
      ╭▸ 
    7 │ drop schema s3;
      ╰╴            ━━
    error: extension `citext` does not exist
       ╭▸ 
    10 │ drop extension citext;
       ╰╴               ━━━━━━
    ");
}

#[test]
fn drop_schema_cascade_drops_dependent_views() {
    assert_snapshot!(check("
create schema s;
create table s.t (a int);
create view v as select a from s.t;
drop schema s cascade;
select * from v;
"), @"
    error: relation `v` does not exist
      ╭▸ 
    6 │ select * from v;
      ╰╴              ━
    ");
}

#[test]
fn drop_type_cascade_drops_columns() {
    assert_snapshot!(check("
create type mood as enum ('sad');
create type c as (x int, m mood);
create table t (a int, m mood, ms mood[]);
create table u (v c);
drop type mood;
drop type mood cascade;
select a from t;
select m from t;
select ms from t;
select (v).x from u;
select (v).m from u;
"), @"
    error: cannot drop type `mood` because other objects depend on it
      ╭▸ 
    6 │ drop type mood;
      ╰╴          ━━━━
    error: column `m` does not exist
      ╭▸ 
    9 │ select m from t;
      │        ━
      ╰╴
    help: a column with a similar name exists
      ╭╴
    9 - select m from t;
    9 + select a from t;
      ╰╴
    error: column `ms` does not exist
       ╭▸ 
    10 │ select ms from t;
       ╰╴       ━━
    error: column `m` does not exist
       ╭▸ 
    12 │ select (v).m from u;
       ╰╴           ━
    ");
}

#[test]
fn drop_type_cascade_drops_dependent_objects() {
    assert_snapshot!(check("
create type mood as enum ('sad');
create function f(mood) returns int language sql return 1;
create domain d as mood;
create table t (a d, b int);
create function to_text(mood) returns text language sql return 'x';
create cast (mood as text) with function to_text(mood);
create type r as range (subtype = mood);
drop type mood;
drop type mood cascade;
select f(null);
select a from t;
select b from t;
select null::d;
select null::r;
select null::rmultirange;
select to_text(null);
"), @"
    error: cannot drop type `mood` because other objects depend on it
      ╭▸ 
    9 │ drop type mood;
      ╰╴          ━━━━
    error: function `f(unknown)` does not exist
       ╭▸ 
    11 │ select f(null);
       │        ━
       │
       ╰ hint: consider adding explicit type casts
    error: column `a` does not exist
       ╭▸ 
    12 │ select a from t;
       │        ━
       ╰╴
    help: a column with a similar name exists
       ╭╴
    12 - select a from t;
    12 + select b from t;
       ╰╴
    error: type `d` does not exist
       ╭▸ 
    14 │ select null::d;
       ╰╴             ━
    error: type `r` does not exist
       ╭▸ 
    15 │ select null::r;
       ╰╴             ━
    error: type `rmultirange` does not exist
       ╭▸ 
    16 │ select null::rmultirange;
       ╰╴             ━━━━━━━━━━━
    error: function `to_text(unknown)` does not exist
       ╭▸ 
    17 │ select to_text(null);
       │        ━━━━━━━
       ╰╴
    help: a function with a similar name exists
       ╭╴
    17 - select to_text(null);
    17 + select to_hex(null);
       ╰╴
    ");
}

#[test]
fn drop_domain_restrict() {
    assert_snapshot!(check("
create domain pos as int check (value > 0);
drop domain pos;
create domain pos2 as int;
create domain pos3 as pos2;
create table t (a pos2, b int);
drop domain pos2;
drop domain pos3;
drop domain pos2 cascade;
select a from t;
select b from t;
select null::pos3;
"), @"
    error: cannot drop type `pos2` because other objects depend on it
      ╭▸ 
    7 │ drop domain pos2;
      ╰╴            ━━━━
    error: column `a` does not exist
       ╭▸ 
    10 │ select a from t;
       │        ━
       ╰╴
    help: a column with a similar name exists
       ╭╴
    10 - select a from t;
    10 + select b from t;
       ╰╴
    error: type `pos3` does not exist
       ╭▸ 
    12 │ select null::pos3;
       ╰╴             ━━━━
    ");
}

#[test]
fn drop_table_cascade_drops_rowtype_columns() {
    assert_snapshot!(check("
create table t (a int);
create table u (x t, xs t[], y int);
create type c as (z t, w int);
create table v (c c);
drop table t;
drop table t cascade;
select y from u;
select x from u;
select xs from u;
select (c).w from v;
select (c).z from v;
"), @"
    error: cannot drop table `t` because other objects depend on it
      ╭▸ 
    6 │ drop table t;
      ╰╴           ━
    error: column `x` does not exist
      ╭▸ 
    9 │ select x from u;
      │        ━
      ╰╴
    help: a column with a similar name exists
      ╭╴
    9 - select x from u;
    9 + select y from u;
      ╰╴
    error: column `xs` does not exist
       ╭▸ 
    10 │ select xs from u;
       ╰╴       ━━
    error: column `z` does not exist
       ╭▸ 
    12 │ select (c).z from v;
       ╰╴           ━
    ");
}

#[test]
fn drop_type_cascade_rollback() {
    assert_snapshot!(check("
create type mood as enum ('sad');
create table t (m mood);
begin;
drop type mood cascade;
rollback;
select m from t;
drop type mood;
"), @"
    error: cannot drop type `mood` because other objects depend on it
      ╭▸ 
    8 │ drop type mood;
      ╰╴          ━━━━
    ");
}

#[test]
fn drop_type_with_io_functions() {
    assert_snapshot!(check("
create type t;
create function t_in(cstring) returns t language c as 'x';
create function t_out(t) returns cstring language c as 'x';
create type t (input = t_in, output = t_out);
create table u (a t, b int);
drop type t;
drop function t_in;
drop function t_in cascade;
select null::t;
select b from u;
select a from u;
select t_out(null);
"), @"
    error: cannot drop type `t` because other objects depend on it
      ╭▸ 
    7 │ drop type t;
      ╰╴          ━
    error: cannot drop function `t_in` because other objects depend on it
      ╭▸ 
    8 │ drop function t_in;
      ╰╴              ━━━━
    error: type `t` does not exist
       ╭▸ 
    10 │ select null::t;
       ╰╴             ━
    error: column `a` does not exist
       ╭▸ 
    12 │ select a from u;
       │        ━
       ╰╴
    help: a column with a similar name exists
       ╭╴
    12 - select a from u;
    12 + select b from u;
       ╰╴
    error: function `t_out(unknown)` does not exist
       ╭▸ 
    13 │ select t_out(null);
       │        ━━━━━
       │
       ╰ hint: consider adding explicit type casts
    ");
}

#[test]
fn drop_type_cascade_views_and_altered_columns() {
    assert_snapshot!(check("
create type mood as enum ('sad');
create view v as select null::mood m;
create table t (a int);
alter table t add column m mood;
create type c as (x int);
alter type c add attribute m mood;
create table u (v c);
drop type mood cascade;
select * from v;
select m from t;
select (v).m from u;
"), @"
    error: relation `v` does not exist
       ╭▸ 
    10 │ select * from v;
       │               ━
       ╰╴
    help: a relation with a similar name exists
       ╭╴
    10 - select * from v;
    10 + select * from u;
       ╰╴
    error: column `m` does not exist
       ╭▸ 
    11 │ select m from t;
       │        ━
       ╰╴
    help: a column with a similar name exists
       ╭╴
    11 - select m from t;
    11 + select a from t;
       ╰╴
    error: column `m` does not exist
       ╭▸ 
    12 │ select (v).m from u;
       ╰╴           ━
    ");
}

#[test]
fn drop_type_with_typed_table() {
    assert_snapshot!(check("
create type t as (a int);
create table x of t;
drop type t;
drop type t cascade;
select * from x;
"), @"
    error: cannot drop type `t` because other objects depend on it
      ╭▸ 
    4 │ drop type t;
      ╰╴          ━
    error: relation `x` does not exist
      ╭▸ 
    6 │ select * from x;
      ╰╴              ━
    ");
}

#[test]
fn drop_function_cascade_aggregate() {
    assert_snapshot!(check("
create function add(int, int) returns int language sql return $1 + $2;
create aggregate s(int) (sfunc = add, stype = int);
drop function add(int, int);
drop function add(int, int) cascade;
select s(1);
"), @"
    error: cannot drop function `add` because other objects depend on it
      ╭▸ 
    4 │ drop function add(int, int);
      ╰╴              ━━━
    error: function `s(int4)` does not exist
      ╭▸ 
    6 │ select s(1);
      │        ━
      │
      ╰ hint: consider adding explicit type casts
    ");
}

#[test]
fn drop_function_cascade_domain_constraint() {
    assert_snapshot!(check("
create function f(int) returns bool language sql return true;
create domain d as int constraint c check (f(value));
drop function f(int);
drop function f(int) cascade;
alter domain d drop constraint c;
"), @"
    error: cannot drop function `f` because other objects depend on it
      ╭▸ 
    4 │ drop function f(int);
      ╰╴              ━
    error: constraint `c` does not exist
      ╭▸ 
    6 │ alter domain d drop constraint c;
      ╰╴                               ━
    ");
}

#[test]
fn drop_schema_restrict() {
    assert_snapshot!(check("
create schema s;
create table s.t (a int);
drop schema s;
drop schema s restrict;
select a from s.t;
create schema e;
drop schema e;
drop schema e;
drop schema if exists e;
"), @"
    error: cannot drop schema `s` because other objects depend on it
      ╭▸ 
    4 │ drop schema s;
      ╰╴            ━
    error: cannot drop schema `s` because other objects depend on it
      ╭▸ 
    5 │ drop schema s restrict;
      ╰╴            ━
    error: schema `e` does not exist
      ╭▸ 
    9 │ drop schema e;
      ╰╴            ━
    ");
}

#[test]
fn drop_schema_cascade() {
    assert_snapshot!(check("
create schema s;
create table s.t (a int);
create type s.ty as (a int);
create domain s.d as int;
create function s.f() returns int language sql return 1;
create collation s.c (provider = icu, locale = 'und');
create view v as select s.f() a;
drop schema s cascade;
select * from s.t;
select 1::s.ty, 1::s.d;
select s.f();
select * from v;
select 'a' collate s.c;
create schema s;
select * from s.t;
"), @"
    error: schema `s` does not exist
       ╭▸ 
    10 │ select * from s.t;
       ╰╴              ━
    error: relation `t` does not exist
       ╭▸ 
    10 │ select * from s.t;
       ╰╴                ━
    error: schema `s` does not exist
       ╭▸ 
    11 │ select 1::s.ty, 1::s.d;
       ╰╴          ━
    error: type `ty` does not exist
       ╭▸ 
    11 │ select 1::s.ty, 1::s.d;
       ╰╴            ━━
    error: schema `s` does not exist
       ╭▸ 
    11 │ select 1::s.ty, 1::s.d;
       ╰╴                   ━
    error: type `d` does not exist
       ╭▸ 
    11 │ select 1::s.ty, 1::s.d;
       ╰╴                     ━
    error: schema `s` does not exist
       ╭▸ 
    12 │ select s.f();
       ╰╴       ━
    error: function `s.f()` does not exist
       ╭▸ 
    12 │ select s.f();
       ╰╴         ━
    error: relation `v` does not exist
       ╭▸ 
    13 │ select * from v;
       ╰╴              ━
    error: schema `s` does not exist
       ╭▸ 
    14 │ select 'a' collate s.c;
       ╰╴                   ━
    error: collation `c` does not exist
       ╭▸ 
    14 │ select 'a' collate s.c;
       ╰╴                     ━
    error: relation `t` does not exist
       ╭▸ 
    16 │ select * from s.t;
       ╰╴                ━
    ");
}

#[test]
fn drop_schema_cascade_rollback() {
    assert_snapshot!(check("
create schema s;
create table s.t (a int);
begin;
drop schema s cascade;
rollback;
select a from s.t;
"), @"no diagnostics");
}

#[test]
fn alter_table_set_schema_moves_indexes() {
    assert_snapshot!(check("
create schema s;
create table t (a int primary key, b int);
create index i on t (b);
alter table t set schema s;
drop index i;
drop index s.i;
alter table s.t drop constraint t_pkey;
drop index s.t_pkey;
create index j on s.t (b);
begin;
alter table s.t set schema public;
rollback;
drop index public.j;
drop index s.j;
"), @"
    error: index `i` does not exist
      ╭▸ 
    6 │ drop index i;
      ╰╴           ━
    error: index `t_pkey` does not exist
      ╭▸ 
    9 │ drop index s.t_pkey;
      ╰╴             ━━━━━━
    error: index `j` does not exist
       ╭▸ 
    14 │ drop index public.j;
       ╰╴                  ━
    ");
}

#[test]
fn rename_column_updates_dependents() {
    assert_snapshot!(check("
create table t (a int generated always as identity, b int not null);
alter table t rename column a to x;
alter table t alter column x drop identity;
select last_value from t_a_seq;
alter table t rename column b to y;
alter table t add column b int not null;
alter table t drop constraint t_b_not_null;
alter table t drop constraint t_b_not_null1;
create table u (a int generated always as identity);
begin;
alter table u rename column a to x;
rollback;
alter table u alter column a drop identity;
"), @"
    error: relation `t_a_seq` does not exist
      ╭▸ 
    5 │ select last_value from t_a_seq;
      ╰╴                       ━━━━━━━
    ");
}

#[test]
fn drop_function_cascade_exclusion_constraint() {
    assert_snapshot!(check("
create function f(int) returns int language sql immutable return 1;
create table t (a int, constraint ex exclude ((f(a)) with =));
create table u (a int, constraint ex2 exclude (a with =) where (f(a) > 0));
drop function f(int);
drop function f(int) cascade;
alter table t drop constraint ex;
drop index ex;
alter table u drop constraint ex2;
"), @"
    error: cannot drop function `f` because other objects depend on it
      ╭▸ 
    5 │ drop function f(int);
      ╰╴              ━
    error: constraint `ex` does not exist
      ╭▸ 
    7 │ alter table t drop constraint ex;
      ╰╴                              ━━
    error: index `ex` does not exist
      ╭▸ 
    8 │ drop index ex;
      ╰╴           ━━
    error: constraint `ex2` does not exist
      ╭▸ 
    9 │ alter table u drop constraint ex2;
      ╰╴                              ━━━
    ");
}

#[test]
fn drop_function_cascade_operator_class_support() {
    assert_snapshot!(check("
create function cmp(int, int) returns int language sql return 0;
create operator class c for type int using btree as operator 1 <, function 1 cmp(int, int);
drop function cmp(int, int);
drop function cmp(int, int) cascade;
drop operator class c using btree;
create function cmp2(int, int) returns int language sql return 0;
create operator family fam using btree;
alter operator family fam using btree add function 1 (int, int) cmp2(int, int);
drop function cmp2(int, int);
drop operator family fam using btree;
drop function cmp2(int, int);
"), @"
    error: cannot drop function `cmp` because other objects depend on it
      ╭▸ 
    4 │ drop function cmp(int, int);
      ╰╴              ━━━
    error: cannot drop function `cmp2` because other objects depend on it
       ╭▸ 
    10 │ drop function cmp2(int, int);
       ╰╴              ━━━━
    ");
}

#[test]
fn drop_function_operator_class_support_follows_renames() {
    assert_snapshot!(check("
create schema s;
create function cmp(int, int) returns int language sql return 0;
create operator class c for type int using btree as function 1 cmp(int, int);
alter operator class c using btree rename to c2;
drop function cmp(int, int);
alter operator class c2 using btree set schema s;
drop function cmp(int, int);
create function cmp2(int, int) returns int language sql return 0;
create operator family fam using btree;
alter operator family fam using btree add function 1 (int, int) cmp2(int, int);
alter operator family fam using btree rename to fam2;
drop function cmp2(int, int);
alter operator family fam2 using btree set schema s;
drop function cmp2(int, int);
drop operator family s.fam2 using btree;
drop function cmp2(int, int);
"), @"
    error: cannot drop function `cmp` because other objects depend on it
      ╭▸ 
    6 │ drop function cmp(int, int);
      ╰╴              ━━━
    error: cannot drop function `cmp` because other objects depend on it
      ╭▸ 
    8 │ drop function cmp(int, int);
      ╰╴              ━━━
    error: cannot drop function `cmp2` because other objects depend on it
       ╭▸ 
    13 │ drop function cmp2(int, int);
       ╰╴              ━━━━
    error: cannot drop function `cmp2` because other objects depend on it
       ╭▸ 
    15 │ drop function cmp2(int, int);
       ╰╴              ━━━━
    ");
}

#[test]
fn drop_operator_with_operator_family_members() {
    assert_snapshot!(check("
create function eq(int, int) returns bool language sql return true;
create operator === (leftarg = int, rightarg = int, function = eq);
create operator family fam using btree;
alter operator family fam using btree add operator 1 === (int, int);
create operator class c for type int using btree as operator 1 ===;
drop operator === (int, int);
drop operator === (int, int) restrict;
alter operator family fam using btree drop operator 1 (int, int);
drop operator class c using btree;
drop operator === (int, int);
create operator ==== (leftarg = int, rightarg = int, function = eq);
create operator class c2 for type int using btree as operator 1 ====;
drop operator ==== (int, int) cascade;
drop function eq;
"), @"
    error: cannot drop operator `===` because other objects depend on it
      ╭▸ 
    7 │ drop operator === (int, int);
      ╰╴              ━━━
    error: cannot drop operator `===` because other objects depend on it
      ╭▸ 
    8 │ drop operator === (int, int) restrict;
      ╰╴              ━━━
    ");
}

#[test]
fn drop_function_cascade_through_operator_to_member() {
    assert_snapshot!(check("
create function eq(int, int) returns bool language sql return true;
create operator === (leftarg = int, rightarg = int, function = eq);
create operator class c for type int using btree as operator 1 ===;
drop function eq;
drop function eq cascade;
drop operator === (int, int);
drop operator class c using btree;
"), @"
    error: cannot drop function `eq` because other objects depend on it
      ╭▸ 
    5 │ drop function eq;
      ╰╴              ━━
    error: operator does not exist: `int4 === int4`
      ╭▸ 
    7 │ drop operator === (int, int);
      ╰╴              ━━━
    ");
}

#[test]
fn drop_type_typed_columns_after_rollback_and_rename() {
    assert_snapshot!(check("
create type mood as enum (
  'sad'
);
create table t (a int);
begin;
alter table t add column m mood;
rollback;
drop type mood;
create type r as range (subtype = int);
alter type r rename to s;
create table u (m r_multirange);
drop type s;
"), @"
    error: cannot drop type `s` because other objects depend on it
       ╭▸ 
    13 │ drop type s;
       ╰╴          ━
    ");
}
