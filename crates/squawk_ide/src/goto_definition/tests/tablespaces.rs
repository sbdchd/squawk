use insta::assert_snapshot;

use super::goto;

#[test]
fn tablespace_create_table() {
    assert_snapshot!(goto("
create tablespace ts
  location '/data';
create table t (a int) tablespace ts$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:19
      │
    2 │ create tablespace ts
      │                   ── 2. destination
    3 │   location '/data';
    4 │ create table t (a int) tablespace ts;
      ╰╴                                  ── 1. source
    ");
}

#[test]
fn tablespace_create_index() {
    assert_snapshot!(goto("
create tablespace ts
  location '/data';
create table t (a int);
create index i on t (a) tablespace ts$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:19
      │
    2 │ create tablespace ts
      │                   ── 2. destination
      ‡
    5 │ create index i on t (a) tablespace ts;
      ╰╴                                   ── 1. source
    ");
}

#[test]
fn tablespace_constraint_index() {
    assert_snapshot!(goto("
create tablespace ts location '/data';
create table t (a int primary key using index tablespace ts$0);
"), @"
    info: goto definition
      ╭▸ file0.sql:2:19
      │
    2 │ create tablespace ts location '/data';
      │                   ── 2. destination
    3 │ create table t (a int primary key using index tablespace ts);
      ╰╴                                                         ── 1. source
    ");
}

#[test]
fn tablespace_alter_table_set() {
    assert_snapshot!(goto("
create tablespace ts
  location '/data';
create table t (a int);
alter table t set tablespace ts$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:19
      │
    2 │ create tablespace ts
      │                   ── 2. destination
      ‡
    5 │ alter table t set tablespace ts;
      ╰╴                             ── 1. source
    ");
}

#[test]
fn tablespace_alter_index_all_in() {
    assert_snapshot!(goto("
create tablespace ts
  location '/data';
alter index all in tablespace ts$0 set tablespace pg_default;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:19
      │
    2 │ create tablespace ts
      │                   ── 2. destination
    3 │   location '/data';
    4 │ alter index all in tablespace ts set tablespace pg_default;
      ╰╴                              ── 1. source
    ");
}

#[test]
fn tablespace_drop() {
    assert_snapshot!(goto(r#"
create tablespace "Ts"
  location '/data';
drop tablespace "Ts"$0;
"#), @r#"
    info: goto definition
      ╭▸ file0.sql:2:19
      │
    2 │ create tablespace "Ts"
      │                   ──── 2. destination
    3 │   location '/data';
    4 │ drop tablespace "Ts";
      ╰╴                ──── 1. source
    "#);
}

#[test]
fn tablespace_builtin() {
    assert_snapshot!(goto("
create table t (a int) tablespace pg_default$0;
"), @"
    info: goto definition
       ╭▸ file0.sql:2:35
       │
     2 │ create table t (a int) tablespace pg_default;
       │                                   ────────── 1. source
       │
       ⸬  builtins.sql:40:19
       │
    40 │ create tablespace pg_default
       ╰╴                  ────────── 2. destination
    ");
}

#[test]
fn tablespace_renamed() {
    assert_snapshot!(goto("
create tablespace ts
  location '/data';
alter tablespace ts rename to ts2;
create table t (a int) tablespace ts2$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:4:31
      │
    4 │ alter tablespace ts rename to ts2;
      │                               ─── 2. destination
    5 │ create table t (a int) tablespace ts2;
      ╰╴                                  ─── 1. source
    ");
}
