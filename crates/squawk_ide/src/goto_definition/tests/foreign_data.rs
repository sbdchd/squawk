use insta::assert_snapshot;

use super::goto;

#[test]
fn foreign_table_column() {
    assert_snapshot!(goto("
create foreign table t (a int) server s;
select a$0 from t;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:25
      │
    2 │ create foreign table t (a int) server s;
      │                         ─ 2. destination
    3 │ select a from t;
      ╰╴       ─ 1. source
    ");
}

#[test]
fn foreign_table_name() {
    assert_snapshot!(goto("
create foreign table t (a int) server s;
select a from t$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:22
      │
    2 │ create foreign table t (a int) server s;
      │                      ─ 2. destination
    3 │ select a from t;
      ╰╴              ─ 1. source
    ");
}

#[test]
fn create_server_foreign_data_wrapper() {
    assert_snapshot!(goto("
create foreign data wrapper w;
create server s foreign data wrapper w$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:29
      │
    2 │ create foreign data wrapper w;
      │                             ─ 2. destination
    3 │ create server s foreign data wrapper w;
      ╰╴                                     ─ 1. source
    ");
}

#[test]
fn create_foreign_data_wrapper_handler() {
    assert_snapshot!(goto("
create function h() returns fdw_handler
  language c
  as 'h';
create foreign data wrapper w handler h$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create function h() returns fdw_handler
      │                 ─ 2. destination
      ‡
    5 │ create foreign data wrapper w handler h;
      ╰╴                                      ─ 1. source
    ");
}

#[test]
fn create_foreign_data_wrapper_validator() {
    assert_snapshot!(goto("
create function v(text[], oid) returns void
  language c
  as 'v';
create foreign data wrapper w validator v$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create function v(text[], oid) returns void
      │                 ─ 2. destination
      ‡
    5 │ create foreign data wrapper w validator v;
      ╰╴                                        ─ 1. source
    ");
}

#[test]
fn foreign_table_server() {
    assert_snapshot!(goto("
create foreign data wrapper w;
create server s foreign data wrapper w;
create foreign table t (a int) server s$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:15
      │
    3 │ create server s foreign data wrapper w;
      │               ─ 2. destination
    4 │ create foreign table t (a int) server s;
      ╰╴                                      ─ 1. source
    ");
}

#[test]
fn user_mapping_server() {
    assert_snapshot!(goto("
create foreign data wrapper w;
create server s foreign data wrapper w;
create user mapping for public server s$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:15
      │
    3 │ create server s foreign data wrapper w;
      │               ─ 2. destination
    4 │ create user mapping for public server s;
      ╰╴                                      ─ 1. source
    ");
}

#[test]
fn user_mapping_role() {
    assert_snapshot!(goto("
create role r;
create foreign data wrapper w;
create server s foreign data wrapper w;
create user mapping for r$0 server s;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:13
      │
    2 │ create role r;
      │             ─ 2. destination
      ‡
    5 │ create user mapping for r server s;
      ╰╴                        ─ 1. source
    ");
}

#[test]
fn renamed_server() {
    assert_snapshot!(goto("
create foreign data wrapper w;
create server s foreign data wrapper w;
alter server s rename to s2;
drop server s2$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:4:26
      │
    4 │ alter server s rename to s2;
      │                          ── 2. destination
    5 │ drop server s2;
      ╰╴            ── 1. source
    ");
}

#[test]
fn import_foreign_schema_into() {
    assert_snapshot!(goto("
create foreign data wrapper w;
create server s foreign data wrapper w;
create schema local;
import foreign schema remote from server s into local$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:4:15
      │
    4 │ create schema local;
      │               ───── 2. destination
    5 │ import foreign schema remote from server s into local;
      ╰╴                                                ───── 1. source
    ");
}

#[test]
fn drop_foreign_table_ref() {
    assert_snapshot!(goto("
create foreign data wrapper w;
create server s foreign data wrapper w;
create foreign table ft (a int) server s;
drop foreign table ft$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:4:22
      │
    4 │ create foreign table ft (a int) server s;
      │                      ── 2. destination
    5 │ drop foreign table ft;
      ╰╴                   ── 1. source
    ");
}

#[test]
fn alter_foreign_table_drop_column_ref() {
    assert_snapshot!(goto("
create foreign data wrapper w;
create server s foreign data wrapper w;
create foreign table ft (a int) server s;
alter foreign table ft drop column a$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:4:26
      │
    4 │ create foreign table ft (a int) server s;
      │                          ─ 2. destination
    5 │ alter foreign table ft drop column a;
      ╰╴                                   ─ 1. source
    ");
}
