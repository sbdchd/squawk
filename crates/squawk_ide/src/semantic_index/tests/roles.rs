use insta::assert_snapshot;

use super::check;

#[test]
fn create_role() {
    assert_snapshot!(check(r#"
create role a;
create user b in role a, missing1;
create group c with user b admin a role missing2;
create role "A" in group pg_monitor;
create role a;
create user a;
create group a;
"#), @"
    error: role `missing1` does not exist
      ╭▸ 
    3 │ create user b in role a, missing1;
      ╰╴                         ━━━━━━━━
    error: role `missing2` does not exist
      ╭▸ 
    4 │ create group c with user b admin a role missing2;
      ╰╴                                        ━━━━━━━━
    error: role `a` already exists
      ╭▸ 
    6 │ create role a;
      ╰╴            ━
    error: role `a` already exists
      ╭▸ 
    7 │ create user a;
      ╰╴            ━
    error: role `a` already exists
      ╭▸ 
    8 │ create group a;
      ╰╴             ━
    ");
}

#[test]
fn alter_role() {
    assert_snapshot!(check("
create role a;
create role b;
alter role a with nologin;
alter role a in database postgres set work_mem = '1MB';
alter role a in database missing_db reset all;
alter role missing set work_mem = '1MB';
alter role current_user set work_mem = '1MB';
alter role all set work_mem = '1MB';
alter user b valid until 'infinity';
alter user missing reset all;
alter group a add user b, missing;
alter group a drop user b;
"), @"
    error: database `missing_db` does not exist
      ╭▸ 
    6 │ alter role a in database missing_db reset all;
      ╰╴                         ━━━━━━━━━━
    error: role `missing` does not exist
      ╭▸ 
    7 │ alter role missing set work_mem = '1MB';
      ╰╴           ━━━━━━━
    error: role `missing` does not exist
       ╭▸ 
    11 │ alter user missing reset all;
       ╰╴           ━━━━━━━
    error: role `missing` does not exist
       ╭▸ 
    12 │ alter group a add user b, missing;
       ╰╴                          ━━━━━━━
    ");
}

#[test]
fn rename_role() {
    assert_snapshot!(check("
create role a;
create role b;
alter role a rename to b;
alter role a rename to c;
alter user c rename to d;
alter group d rename to e;
alter role a rename to f;
alter role e with login;
"), @"
    error: role `b` already exists
      ╭▸ 
    4 │ alter role a rename to b;
      ╰╴                       ━
    error: role `a` does not exist
      ╭▸ 
    8 │ alter role a rename to f;
      ╰╴           ━
    ");
}

#[test]
fn drop_role() {
    assert_snapshot!(check("
create role a;
create user b;
create group c;
drop role a, b;
drop user c;
drop group a;
drop role if exists a, missing;
create role a;
"), @"
    error: role `a` does not exist
      ╭▸ 
    7 │ drop group a;
      ╰╴           ━
    ");
}

#[test]
fn role_rollback() {
    assert_snapshot!(check("
create role a;
begin;
drop role a;
create role b;
rollback;
alter role a with login;
alter role b with login;
"), @"
    error: role `b` does not exist
      ╭▸ 
    8 │ alter role b with login;
      ╰╴           ━
    ");
}

#[test]
fn builtin_roles() {
    assert_snapshot!(check("
create role a in role pg_read_all_data, pg_write_all_data;
alter database postgres owner to pg_database_owner;
create role pg_monitor;
"), @"
    error: role `pg_monitor` already exists
      ╭▸ 
    4 │ create role pg_monitor;
      ╰╴            ━━━━━━━━━━
    ");
}

#[test]
fn role_refs() {
    assert_snapshot!(check("
create role a;
create table t (a int);
create sequence s;
create collation c (locale = 'C');
set role a;
set role missing;
set role 'a';
set role none;
set session authorization a;
set session authorization 'missing';
set session authorization default;
reset role;
drop owned by a, missing;
reassign owned by a, missing to a;
reassign owned by current_user to missing;
alter table t owner to a;
alter table t owner to missing;
alter sequence s owner to a;
alter collation c owner to missing;
alter database postgres owner to missing;
create database d owner = missing;
alter tablespace pg_default owner to missing;
create tablespace ts owner missing location '/tmp';
alter table all in tablespace pg_default owned by missing set tablespace pg_global;
alter table t owner to current_user;
alter table t owner to session_user;
"), @"
    error: role `missing` does not exist
      ╭▸ 
    7 │ set role missing;
      ╰╴         ━━━━━━━
    error: role `missing` does not exist
       ╭▸ 
    11 │ set session authorization 'missing';
       ╰╴                          ━━━━━━━━━
    error: role `missing` does not exist
       ╭▸ 
    14 │ drop owned by a, missing;
       ╰╴                 ━━━━━━━
    error: role `missing` does not exist
       ╭▸ 
    15 │ reassign owned by a, missing to a;
       ╰╴                     ━━━━━━━
    error: role `missing` does not exist
       ╭▸ 
    16 │ reassign owned by current_user to missing;
       ╰╴                                  ━━━━━━━
    error: role `missing` does not exist
       ╭▸ 
    18 │ alter table t owner to missing;
       ╰╴                       ━━━━━━━
    error: role `missing` does not exist
       ╭▸ 
    20 │ alter collation c owner to missing;
       ╰╴                           ━━━━━━━
    error: role `missing` does not exist
       ╭▸ 
    21 │ alter database postgres owner to missing;
       ╰╴                                 ━━━━━━━
    error: role `missing` does not exist
       ╭▸ 
    22 │ create database d owner = missing;
       ╰╴                          ━━━━━━━
    error: role `missing` does not exist
       ╭▸ 
    23 │ alter tablespace pg_default owner to missing;
       ╰╴                                     ━━━━━━━
    error: role `missing` does not exist
       ╭▸ 
    24 │ create tablespace ts owner missing location '/tmp';
       ╰╴                           ━━━━━━━
    error: role `missing` does not exist
       ╭▸ 
    25 │ alter table all in tablespace pg_default owned by missing set tablespace pg_global;
       ╰╴                                                  ━━━━━━━
    ");
}

#[test]
fn grant_and_revoke() {
    assert_snapshot!(check("
create table t (a int, b int);
create view v as select 1 a;
create sequence s;
create schema sc;
create role r;
create role r2;
create function f(int) returns int language sql as 'select 1';
create procedure p() language sql as 'select 1';
create type ty as (x int);
create domain d as int;
grant select on t to r;
grant select, insert on table t, v, s to r, public, current_user;
grant select on nope to r;
grant select (a, b) on t to r;
grant update (nope) on t to r;
grant all privileges (a) on t to r;
grant all on nope to nope2;
grant usage on sequence s to r;
grant usage on sequence nope to r;
grant usage, create on schema sc to r;
grant usage on schema nope to r;
grant select on all tables in schema sc, nope to r;
grant execute on function f(int) to r;
grant execute on function f(text) to r;
grant execute on procedure p() to r;
grant execute on routine nope to r;
grant execute on all functions in schema sc to r;
grant usage on type ty to r;
grant usage on type nope to r;
grant usage on domain d to r;
grant usage on language sql to r;
grant usage on language nope to r;
grant set on parameter work_mem to r;
grant select on t to r with grant option granted by r2;
grant select on t to r granted by nope;
grant r to r2;
grant nope to r2 with admin option;
grant r to nope;
revoke select on t from r;
revoke grant option for select (a) on t from r cascade;
revoke select on nope from r;
revoke all on schema sc from nope;
revoke r from r2;
revoke admin option for nope from r2;
"), @"
    error: relation `nope` does not exist
       ╭▸ 
    14 │ grant select on nope to r;
       ╰╴                ━━━━
    error: column `nope` does not exist
       ╭▸ 
    16 │ grant update (nope) on t to r;
       ╰╴              ━━━━
    error: relation `nope` does not exist
       ╭▸ 
    18 │ grant all on nope to nope2;
       ╰╴             ━━━━
    error: role `nope2` does not exist
       ╭▸ 
    18 │ grant all on nope to nope2;
       ╰╴                     ━━━━━
    error: relation `nope` does not exist
       ╭▸ 
    20 │ grant usage on sequence nope to r;
       ╰╴                        ━━━━
    error: schema `nope` does not exist
       ╭▸ 
    22 │ grant usage on schema nope to r;
       ╰╴                      ━━━━
    error: schema `nope` does not exist
       ╭▸ 
    23 │ grant select on all tables in schema sc, nope to r;
       ╰╴                                         ━━━━
    error: function `f(text)` does not exist
       ╭▸ 
    25 │ grant execute on function f(text) to r;
       ╰╴                          ━
    error: routine `nope` does not exist
       ╭▸ 
    27 │ grant execute on routine nope to r;
       ╰╴                         ━━━━
    error: type `nope` does not exist
       ╭▸ 
    30 │ grant usage on type nope to r;
       ╰╴                    ━━━━
    error: language `nope` does not exist
       ╭▸ 
    33 │ grant usage on language nope to r;
       ╰╴                        ━━━━
    error: role `nope` does not exist
       ╭▸ 
    36 │ grant select on t to r granted by nope;
       ╰╴                                  ━━━━
    error: role `nope` does not exist
       ╭▸ 
    38 │ grant nope to r2 with admin option;
       ╰╴      ━━━━
    error: role `nope` does not exist
       ╭▸ 
    39 │ grant r to nope;
       ╰╴           ━━━━
    error: relation `nope` does not exist
       ╭▸ 
    42 │ revoke select on nope from r;
       ╰╴                 ━━━━
    error: role `nope` does not exist
       ╭▸ 
    43 │ revoke all on schema sc from nope;
       ╰╴                             ━━━━
    error: role `nope` does not exist
       ╭▸ 
    45 │ revoke admin option for nope from r2;
       ╰╴                        ━━━━
    ");
}

#[test]
fn alter_default_privileges() {
    assert_snapshot!(check("
create schema sc;
create role r;
create role r2;
alter default privileges grant select on tables to r;
alter default privileges for role r in schema sc grant all on functions to r2, public;
alter default privileges for user nope in schema sc, nope2 grant usage on types to r;
alter default privileges in schema sc revoke grant option for select on tables from nope3 cascade;
alter default privileges revoke usage, create on schemas from r;
"), @"
    error: role `nope` does not exist
      ╭▸ 
    7 │ alter default privileges for user nope in schema sc, nope2 grant usage on types to r;
      ╰╴                                  ━━━━
    error: schema `nope2` does not exist
      ╭▸ 
    7 │ alter default privileges for user nope in schema sc, nope2 grant usage on types to r;
      ╰╴                                                     ━━━━━
    error: role `nope3` does not exist
      ╭▸ 
    8 │ alter default privileges in schema sc revoke grant option for select on tables from nope3 cascade;
      ╰╴                                                                                    ━━━━━
    ");
}

#[test]
fn alter_large_object() {
    assert_snapshot!(check("
create role r;
alter large object 123 owner to r;
alter large object 123 owner to current_user;
alter large object 123 owner to nope;
comment on large object 123 is 'x';
security label on large object 123 is 'x';
grant select on large object 123, 456 to r;
revoke all on large object 123 from r;
"), @"
    error: role `nope` does not exist
      ╭▸ 
    5 │ alter large object 123 owner to nope;
      ╰╴                                ━━━━
    ");
}

#[test]
fn grant_to_quoted_public() {
    assert_snapshot!(check(r#"
create table t (a int);
grant select on t to "public";
"#), @"no diagnostics");
}
