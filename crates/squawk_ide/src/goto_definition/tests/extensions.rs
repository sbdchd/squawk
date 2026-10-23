use insta::assert_snapshot;

use super::goto;

#[test]
fn extension_alter() {
    assert_snapshot!(goto("
create extension e;
alter extension e$0 update;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:18
      │
    2 │ create extension e;
      │                  ─ 2. destination
    3 │ alter extension e update;
      ╰╴                ─ 1. source
    ");
}

#[test]
fn extension_quoted_drop() {
    assert_snapshot!(goto(r#"
create extension "uuid-ossp";
drop extension "uuid-ossp"$0;
"#), @r#"
    info: goto definition
      ╭▸ file0.sql:2:18
      │
    2 │ create extension "uuid-ossp";
      │                  ─────────── 2. destination
    3 │ drop extension "uuid-ossp";
      ╰╴               ─────────── 1. source
    "#);
}

#[test]
fn extension_schema() {
    assert_snapshot!(goto("
create schema s;
create extension e schema s$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:15
      │
    2 │ create schema s;
      │               ─ 2. destination
    3 │ create extension e schema s;
      ╰╴                          ─ 1. source
    ");
}

#[test]
fn alter_materialized_view_depends_on_extension_ref() {
    assert_snapshot!(goto("
create extension e;
create materialized view v as
  select 1 a;
alter materialized view v depends on extension e$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:18
      │
    2 │ create extension e;
      │                  ─ 2. destination
      ‡
    5 │ alter materialized view v depends on extension e;
      ╰╴                                               ─ 1. source
    ");
}

#[test]
fn alter_extension_add_table_ref() {
    assert_snapshot!(goto("
create extension e;
create table t (a int);
alter extension e add table t$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:14
      │
    3 │ create table t (a int);
      │              ─ 2. destination
    4 │ alter extension e add table t;
      ╰╴                            ─ 1. source
    ");
}

#[test]
fn alter_extension_add_foreign_table() {
    assert_snapshot!(goto("
create extension e;
create foreign table t (id int) server s;
alter extension e add foreign table t$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:22
      │
    3 │ create foreign table t (id int) server s;
      │                      ─ 2. destination
    4 │ alter extension e add foreign table t;
      ╰╴                                    ─ 1. source
    ");
}

#[test]
fn extension_function() {
    assert_snapshot!(goto("
create extension hstore;
select akeys$0(null::hstore);
"), @"
    info: goto definition
       ╭▸ file0.sql:3:8
       │
     3 │ select akeys(null::hstore);
       │        ───── 1. source
       │
       ⸬  hstore.sql:12:17
       │
    12 │ create function akeys(hstore) returns text[]
       ╰╴                ───── 2. destination
    ");
}

#[test]
fn extension_type_in_schema() {
    assert_snapshot!(goto("
create schema s;
create extension citext schema s;
create table t (a s.citext$0);
"), @"
    info: goto definition
        ╭▸ file0.sql:4:21
        │
      4 │ create table t (a s.citext);
        │                     ────── 1. source
        │
        ⸬  citext.sql:7:13
        │
      7 │ create type citext;
        │             ────── 2. destination
        ‡
    315 │ create type citext (
        ╰╴            ────── 3. destination
    ");
}
