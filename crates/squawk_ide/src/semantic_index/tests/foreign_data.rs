use insta::assert_snapshot;

use super::check;

#[test]
fn foreign_table() {
    assert_snapshot!(check("
create foreign data wrapper w;
create server s foreign data wrapper w;
create table p (a int);
create foreign table t (b int, like p) inherits (p) server s;
select a, b, nope from t;
"), @"
    error: column `nope` does not exist
      ╭▸ 
    6 │ select a, b, nope from t;
      ╰╴             ━━━━
    ");
}

#[test]
fn create_foreign_data_wrapper() {
    assert_snapshot!(check("
create function h() returns fdw_handler language c as 'h';
create function v(text[], oid) returns void language c as 'v';
create foreign data wrapper w handler h validator v options (debug 'true');
create foreign data wrapper w2 no handler no validator;
create foreign data wrapper w3 handler v validator h;
create foreign data wrapper w;
"), @"
    error: function `v` does not exist
      ╭▸ 
    6 │ create foreign data wrapper w3 handler v validator h;
      ╰╴                                       ━
    error: function `h` does not exist
      ╭▸ 
    6 │ create foreign data wrapper w3 handler v validator h;
      ╰╴                                                   ━
    error: foreign-data wrapper `w` already exists
      ╭▸ 
    7 │ create foreign data wrapper w;
      ╰╴                            ━
    ");
}

#[test]
fn alter_foreign_data_wrapper() {
    assert_snapshot!(check("
create role r;
create function h() returns fdw_handler language c as 'h';
create foreign data wrapper w;
create foreign data wrapper w2;
alter foreign data wrapper w handler h no validator;
alter foreign data wrapper w handler missing;
alter foreign data wrapper w options (add debug 'true');
alter foreign data wrapper w owner to r;
alter foreign data wrapper w owner to missing_role;
alter foreign data wrapper missing owner to r;
alter foreign data wrapper w rename to w2;
alter foreign data wrapper w rename to w3;
create server s foreign data wrapper w3;
create server s2 foreign data wrapper w;
"), @"
    error: function `missing` does not exist
      ╭▸ 
    7 │ alter foreign data wrapper w handler missing;
      ╰╴                                     ━━━━━━━
    error: role `missing_role` does not exist
       ╭▸ 
    10 │ alter foreign data wrapper w owner to missing_role;
       ╰╴                                      ━━━━━━━━━━━━
    error: foreign-data wrapper `missing` does not exist
       ╭▸ 
    11 │ alter foreign data wrapper missing owner to r;
       ╰╴                           ━━━━━━━
    error: foreign-data wrapper `w2` already exists
       ╭▸ 
    12 │ alter foreign data wrapper w rename to w2;
       ╰╴                                       ━━
    error: foreign-data wrapper `w` does not exist
       ╭▸ 
    15 │ create server s2 foreign data wrapper w;
       ╰╴                                      ━
    ");
}

#[test]
fn drop_foreign_data_wrapper() {
    assert_snapshot!(check("
create foreign data wrapper w;
create foreign data wrapper w2;
drop foreign data wrapper w, w2;
drop foreign data wrapper w;
drop foreign data wrapper if exists w, missing;
create server s foreign data wrapper w;
"), @"
    error: foreign-data wrapper `w` does not exist
      ╭▸ 
    5 │ drop foreign data wrapper w;
      ╰╴                          ━
    error: foreign-data wrapper `w` does not exist
      ╭▸ 
    7 │ create server s foreign data wrapper w;
      ╰╴                                     ━
    ");
}

#[test]
fn builtin_foreign_data_wrapper_missing() {
    assert_snapshot!(check("
create server s foreign data wrapper postgres_fdw;
"), @"
    error: foreign-data wrapper `postgres_fdw` does not exist
      ╭▸ 
    2 │ create server s foreign data wrapper postgres_fdw;
      ╰╴                                     ━━━━━━━━━━━━
    ");
}

#[test]
fn create_server() {
    assert_snapshot!(check("
create foreign data wrapper w;
create server s type 'pg' version '16' foreign data wrapper w options (host 'localhost');
create server s2 foreign data wrapper missing;
create server s foreign data wrapper w;
create server if not exists s foreign data wrapper w;
"), @"
    error: foreign-data wrapper `missing` does not exist
      ╭▸ 
    4 │ create server s2 foreign data wrapper missing;
      ╰╴                                      ━━━━━━━
    error: server `s` already exists
      ╭▸ 
    5 │ create server s foreign data wrapper w;
      ╰╴              ━
    ");
}

#[test]
fn alter_server() {
    assert_snapshot!(check("
create role r;
create foreign data wrapper w;
create server s foreign data wrapper w;
create server s2 foreign data wrapper w;
alter server s version '17' options (set host 'x');
alter server s owner to r;
alter server s owner to missing_role;
alter server missing owner to r;
alter server s rename to s2;
alter server s rename to s3;
create foreign table t (a int) server s3;
create foreign table t2 (a int) server s;
"), @"
    error: role `missing_role` does not exist
      ╭▸ 
    8 │ alter server s owner to missing_role;
      ╰╴                        ━━━━━━━━━━━━
    error: server `missing` does not exist
      ╭▸ 
    9 │ alter server missing owner to r;
      ╰╴             ━━━━━━━
    error: server `s2` already exists
       ╭▸ 
    10 │ alter server s rename to s2;
       ╰╴                         ━━
    error: server `s` does not exist
       ╭▸ 
    13 │ create foreign table t2 (a int) server s;
       ╰╴                                       ━
    ");
}

#[test]
fn drop_server() {
    assert_snapshot!(check("
create foreign data wrapper w;
create server s foreign data wrapper w;
create server s2 foreign data wrapper w;
drop server s, s2;
drop server s;
drop server if exists s, missing;
"), @"
    error: server `s` does not exist
      ╭▸ 
    6 │ drop server s;
      ╰╴            ━
    ");
}

#[test]
fn server_rollback() {
    assert_snapshot!(check("
create foreign data wrapper w;
create server s foreign data wrapper w;
begin;
drop server s;
drop foreign data wrapper w;
create server s2 foreign data wrapper w;
rollback;
alter server s owner to current_user;
alter foreign data wrapper w owner to current_user;
alter server s2 owner to current_user;
"), @"
    error: foreign-data wrapper `w` does not exist
      ╭▸ 
    7 │ create server s2 foreign data wrapper w;
      ╰╴                                      ━
    error: server `s2` does not exist
       ╭▸ 
    11 │ alter server s2 owner to current_user;
       ╰╴             ━━
    ");
}

#[test]
fn user_mapping() {
    assert_snapshot!(check("
create role r;
create foreign data wrapper w;
create server s foreign data wrapper w;
create user mapping for r server s options (user 'x');
create user mapping if not exists for public server s;
create user mapping for current_user server s;
create user mapping for user server s;
create user mapping for missing_role server s;
create user mapping for r server missing;
alter user mapping for r server s options (set user 'y');
alter user mapping for missing_role server missing options (drop user);
drop user mapping for r server s;
drop user mapping if exists for public server missing;
drop user mapping for r server missing;
"), @"
    error: role `missing_role` does not exist
      ╭▸ 
    9 │ create user mapping for missing_role server s;
      ╰╴                        ━━━━━━━━━━━━
    error: server `missing` does not exist
       ╭▸ 
    10 │ create user mapping for r server missing;
       ╰╴                                 ━━━━━━━
    error: role `missing_role` does not exist
       ╭▸ 
    12 │ alter user mapping for missing_role server missing options (drop user);
       ╰╴                       ━━━━━━━━━━━━
    error: server `missing` does not exist
       ╭▸ 
    12 │ alter user mapping for missing_role server missing options (drop user);
       ╰╴                                           ━━━━━━━
    error: server `missing` does not exist
       ╭▸ 
    15 │ drop user mapping for r server missing;
       ╰╴                               ━━━━━━━
    ");
}

#[test]
fn import_foreign_schema() {
    assert_snapshot!(check("
create foreign data wrapper w;
create server s foreign data wrapper w;
create schema local;
import foreign schema remote limit to (a, b) from server s into local;
import foreign schema remote from server missing into missing_schema;
"), @"
    error: server `missing` does not exist
      ╭▸ 
    6 │ import foreign schema remote from server missing into missing_schema;
      ╰╴                                         ━━━━━━━
    error: schema `missing_schema` does not exist
      ╭▸ 
    6 │ import foreign schema remote from server missing into missing_schema;
      ╰╴                                                      ━━━━━━━━━━━━━━
    ");
}

#[test]
fn drop_foreign_table() {
    assert_snapshot!(check("
create foreign data wrapper w;
create server s foreign data wrapper w;
create foreign table ft (a int) server s;
create table t (a int);
drop table ft;
drop foreign table t;
drop foreign table ft;
select a from ft;
drop foreign table ft;
drop foreign table if exists ft;
select a from t;
"), @"
    error: `ft` is not a table
      ╭▸ 
    6 │ drop table ft;
      │            ━━
      │
      ╰ hint: Use DROP FOREIGN TABLE to remove a foreign table.
    error: `t` is not a foreign table
      ╭▸ 
    7 │ drop foreign table t;
      │                    ━
      │
      ╰ hint: Use DROP TABLE to remove a table.
    error: relation `ft` does not exist
      ╭▸ 
    9 │ select a from ft;
      ╰╴              ━━
    error: relation `ft` does not exist
       ╭▸ 
    10 │ drop foreign table ft;
       ╰╴                   ━━
    ");
}

#[test]
fn alter_foreign_table() {
    assert_snapshot!(check("
create foreign data wrapper w;
create server s foreign data wrapper w;
create foreign table ft (a int) server s;
create table t (a int);
alter table ft add constraint c check (a > 0);
alter foreign table ft drop constraint c;
alter foreign table ft drop constraint c;
alter foreign table ft drop column a;
select a from ft;
alter foreign table ft owner to nope;
alter foreign table t owner to nope2;
alter foreign table if exists nope options (add x 'y');
"), @"
    error: constraint `c` does not exist
      ╭▸ 
    8 │ alter foreign table ft drop constraint c;
      ╰╴                                       ━
    error: column `a` does not exist
       ╭▸ 
    10 │ select a from ft;
       ╰╴       ━
    error: role `nope` does not exist
       ╭▸ 
    11 │ alter foreign table ft owner to nope;
       ╰╴                                ━━━━
    error: `t` is not a foreign table
       ╭▸ 
    12 │ alter foreign table t owner to nope2;
       ╰╴                    ━
    error: role `nope2` does not exist
       ╭▸ 
    12 │ alter foreign table t owner to nope2;
       ╰╴                               ━━━━━
    ");
}

#[test]
fn foreign_table_statistics_and_sequence() {
    assert_snapshot!(check("
create foreign data wrapper w;
create server s foreign data wrapper w;
create foreign table ft (a int, b int) server s;
create statistics st on a, b from ft;
create sequence seq owned by ft.a;
create index i on ft (a);
"), @"
    error: cannot create index on foreign table `ft`
      ╭▸ 
    7 │ create index i on ft (a);
      ╰╴                  ━━
    ");
}

#[test]
fn alter_foreign_table_columns() {
    assert_snapshot!(check("
create foreign data wrapper w;
create server s foreign data wrapper w;
create foreign table ft (a int) server s;
alter foreign table ft add column b text, alter column a type bigint;
alter foreign table ft rename column a to c;
select b, c from ft;
alter foreign table ft rename to ft2;
select b from ft2;
"), @"no diagnostics");
}
