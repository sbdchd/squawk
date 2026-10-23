use insta::assert_snapshot;

use super::goto;

#[test]
fn create_access_method_handler() {
    assert_snapshot!(goto("
create function h(internal) returns index_am_handler
  language c
  as 'h';
create access method a
  type index
  handler h$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create function h(internal) returns index_am_handler
      │                 ─ 2. destination
      ‡
    7 │   handler h;
      ╰╴          ─ 1. source
    ");
}

#[test]
fn create_index_using_access_method() {
    assert_snapshot!(goto("
create function h(internal) returns index_am_handler
  language c
  as 'h';
create access method a
  type index
  handler h;
create table t (x int);
create index
  on t using a$0 (x);
"), @"
    info: goto definition
       ╭▸ file0.sql:5:22
       │
     5 │ create access method a
       │                      ─ 2. destination
       ‡
    10 │   on t using a (x);
       ╰╴             ─ 1. source
    ");
}

#[test]
fn drop_access_method_ref() {
    assert_snapshot!(goto("
create function h(internal) returns table_am_handler
  language c
  as 'h';
create access method a
  type table
  handler h;
drop access method a$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:5:22
      │
    5 │ create access method a
      │                      ─ 2. destination
      ‡
    8 │ drop access method a;
      ╰╴                   ─ 1. source
    ");
}

#[test]
fn builtin_access_method() {
    assert_snapshot!(goto("
create table t (x int);
create index
  on t using btree$0 (x);
"), @"
    info: goto definition
          ╭▸ file0.sql:4:14
          │
        4 │   on t using btree (x);
          │              ───── 1. source
          │
          ⸬  builtins.sql:38189:22
          │
    38189 │ create access method btree
          ╰╴                     ───── 2. destination
    ");
}

#[test]
fn builtin_access_method_handler() {
    assert_snapshot!(goto("
create access method a
  type index
  handler bthandler$0;
"), @"
    info: goto definition
         ╭▸ file0.sql:4:11
         │
       4 │   handler bthandler;
         │           ───────── 1. source
         │
         ⸬  builtins.sql:7656:28
         │
    7656 │ create function pg_catalog.bthandler(internal) returns index_am_handler
         ╰╴                           ───────── 2. destination
    ");
}

#[test]
fn alter_table_set_access_method() {
    assert_snapshot!(goto("
create table t (x int);
alter table t set access method heap$0;
"), @"
    info: goto definition
          ╭▸ file0.sql:3:33
          │
        3 │ alter table t set access method heap;
          │                                 ──── 1. source
          │
          ⸬  builtins.sql:38209:22
          │
    38209 │ create access method heap
          ╰╴                     ──── 2. destination
    ");
}

#[test]
fn tablesample_method_ref() {
    assert_snapshot!(goto("
create function my_sample(internal) returns tsm_handler
  language c
  as 'x';
create table t (a int);
select a from t tablesample my_sample$0 (10);
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create function my_sample(internal) returns tsm_handler
      │                 ───────── 2. destination
      ‡
    6 │ select a from t tablesample my_sample (10);
      ╰╴                            ───────── 1. source
    ");
}
