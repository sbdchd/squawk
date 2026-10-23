use insta::assert_snapshot;

use super::goto;

#[test]
fn role_name() {
    assert_snapshot!(goto("
create role a$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:13
      │
    2 │ create role a;
      │             ┬
      │             │
      │             1. source
      ╰╴            2. destination
    ");
}

#[test]
fn alter_role_ref() {
    assert_snapshot!(goto("
create role a;
alter role a$0 with login;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:13
      │
    2 │ create role a;
      │             ─ 2. destination
    3 │ alter role a with login;
      ╰╴           ─ 1. source
    ");
}

#[test]
fn create_role_in_role() {
    assert_snapshot!(goto("
create role a;
create user b in role a$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:13
      │
    2 │ create role a;
      │             ─ 2. destination
    3 │ create user b in role a;
      ╰╴                      ─ 1. source
    ");
}

#[test]
fn drop_role_ref() {
    assert_snapshot!(goto("
create role a;
drop role a$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:13
      │
    2 │ create role a;
      │             ─ 2. destination
    3 │ drop role a;
      ╰╴          ─ 1. source
    ");
}

#[test]
fn renamed_role() {
    assert_snapshot!(goto("
create role a;
alter role a rename to b;
alter role b$0 with login;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:24
      │
    3 │ alter role a rename to b;
      │                        ─ 2. destination
    4 │ alter role b with login;
      ╰╴           ─ 1. source
    ");
}

#[test]
fn owner_to_role() {
    assert_snapshot!(goto("
create role a;
create table t (x int);
alter table t owner to a$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:13
      │
    2 │ create role a;
      │             ─ 2. destination
    3 │ create table t (x int);
    4 │ alter table t owner to a;
      ╰╴                       ─ 1. source
    ");
}

#[test]
fn set_role_literal() {
    assert_snapshot!(goto("
create role a;
set role 'a$0';
"), @"
    info: goto definition
      ╭▸ file0.sql:2:13
      │
    2 │ create role a;
      │             ─ 2. destination
    3 │ set role 'a';
      ╰╴         ─── 1. source
    ");
}

#[test]
fn builtin_role() {
    assert_snapshot!(goto("
create role a in role pg_monitor$0;
"), @"
    info: goto definition
       ╭▸ file0.sql:2:23
       │
     2 │ create role a in role pg_monitor;
       │                       ────────── 1. source
       │
       ⸬  builtins.sql:18:13
       │
    18 │ create role pg_monitor;
       ╰╴            ────────── 2. destination
    ");
}

#[test]
fn schema_authorization_role() {
    assert_snapshot!(goto("
create role a;
create schema authorization a$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:13
      │
    2 │ create role a;
      │             ─ 2. destination
    3 │ create schema authorization a;
      ╰╴                            ─ 1. source
    ");
}

#[test]
fn grant_grantee_ref() {
    assert_snapshot!(goto("
create table t (a int);
create role r;
grant select on t to r$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:13
      │
    3 │ create role r;
      │             ─ 2. destination
    4 │ grant select on t to r;
      ╰╴                     ─ 1. source
    ");
}

#[test]
fn grant_role_ref() {
    assert_snapshot!(goto("
create role r;
create role r2;
grant r$0 to r2;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:13
      │
    2 │ create role r;
      │             ─ 2. destination
    3 │ create role r2;
    4 │ grant r to r2;
      ╰╴      ─ 1. source
    ");
}

#[test]
fn alter_default_privileges_grantee_ref() {
    assert_snapshot!(goto("
create role r;
alter default privileges for role r grant select on tables to r$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:13
      │
    2 │ create role r;
      │             ─ 2. destination
    3 │ alter default privileges for role r grant select on tables to r;
      ╰╴                                                              ─ 1. source
    ");
}

#[test]
fn alter_large_object_owner_ref() {
    assert_snapshot!(goto("
create role r;
alter large object 123 owner to r$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:13
      │
    2 │ create role r;
      │             ─ 2. destination
    3 │ alter large object 123 owner to r;
      ╰╴                                ─ 1. source
    ");
}
