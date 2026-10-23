use insta::assert_snapshot;

use super::goto;

#[test]
fn create_operator_class_family() {
    assert_snapshot!(goto("
create operator family f
  using btree;
create operator class c
  for type int
  using btree
  family f$0 as
  storage int;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:24
      │
    2 │ create operator family f
      │                        ─ 2. destination
      ‡
    7 │   family f as
      ╰╴         ─ 1. source
    ");
}

#[test]
fn create_operator_class_operator() {
    assert_snapshot!(goto("
create type t;
create function t_lt(t, t) returns bool
  language c
  as 'f';
create operator < (
  leftarg = t,
  rightarg = t,
  function = t_lt
);
create operator class c
  for type t
  using btree as
  operator 1 <$0;
"), @"
    info: goto definition
       ╭▸ file0.sql:6:17
       │
     6 │ create operator < (
       │                 ─ 2. destination
       ‡
    14 │   operator 1 <;
       ╰╴             ─ 1. source
    ");
}

#[test]
fn create_operator_class_function() {
    assert_snapshot!(goto("
create function cmp(int, int) returns int
  language c
  as 'f';
create operator class c
  for type int
  using btree as
  function 1 cmp$0(int, int);
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create function cmp(int, int) returns int
      │                 ─── 2. destination
      ‡
    8 │   function 1 cmp(int, int);
      ╰╴             ─── 1. source
    ");
}

#[test]
fn index_user_operator_class() {
    assert_snapshot!(goto("
create operator class c
  for type int
  using btree as
  storage int;
create table t (a int);
create index on t (a c$0);
"), @"
    info: goto definition
      ╭▸ file0.sql:2:23
      │
    2 │ create operator class c
      │                       ─ 2. destination
      ‡
    7 │ create index on t (a c);
      ╰╴                     ─ 1. source
    ");
}

#[test]
fn index_builtin_operator_class() {
    assert_snapshot!(goto("
create table t (a text);
create index on t (a text_pattern_ops$0);
"), @"
    info: goto definition
          ╭▸ file0.sql:3:22
          │
        3 │ create index on t (a text_pattern_ops);
          │                      ──────────────── 1. source
          │
          ⸬  builtins.sql:40210:34
          │
    40210 │ create operator class pg_catalog.text_pattern_ops
          ╰╴                                 ──────────────── 2. destination
    ");
}

#[test]
fn drop_operator_family_ref() {
    assert_snapshot!(goto("
create operator family f
  using btree;
drop operator family f$0 using btree;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:24
      │
    2 │ create operator family f
      │                        ─ 2. destination
    3 │   using btree;
    4 │ drop operator family f using btree;
      ╰╴                     ─ 1. source
    ");
}

#[test]
fn operator_class_access_method() {
    assert_snapshot!(goto("
create operator class c
  for type int
  using hash$0 as
  storage int;
"), @"
    info: goto definition
          ╭▸ file0.sql:4:9
          │
        4 │   using hash as
          │         ──── 1. source
          │
          ⸬  builtins.sql:38204:22
          │
    38204 │ create access method hash
          ╰╴                     ──── 2. destination
    ");
}

#[test]
fn partition_by_operator_class() {
    assert_snapshot!(goto("
create operator class c
  for type int
  using hash as
  storage int;
create table t (a int) partition by hash (a c$0);
"), @"
    info: goto definition
      ╭▸ file0.sql:2:23
      │
    2 │ create operator class c
      │                       ─ 2. destination
      ‡
    6 │ create table t (a int) partition by hash (a c);
      ╰╴                                            ─ 1. source
    ");
}

#[test]
fn operator_family_after_schema_rename() {
    assert_snapshot!(goto("
create schema s;
create operator family s.f
  using btree;
alter schema s rename to t;
create operator class c
  for type int
  using btree
  family t.f$0 as
  storage int;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:26
      │
    3 │ create operator family s.f
      │                          ─ 2. destination
      ‡
    9 │   family t.f as
      ╰╴           ─ 1. source
    ");
}
