use insta::assert_snapshot;

use super::goto;

#[test]
fn like_composite_type() {
    assert_snapshot!(goto("
create type ty as (
  a int
);
create table t (like ty$0);
"), @"
    info: goto definition
      ╭▸ file0.sql:2:13
      │
    2 │ create type ty as (
      │             ── 2. destination
      ‡
    5 │ create table t (like ty);
      ╰╴                     ── 1. source
    ");
}

#[test]
fn alter_type_drop_attribute_ref() {
    assert_snapshot!(goto("
create type t as (
  a int
);
alter type t drop attribute a$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:3
      │
    3 │   a int
      │   ─ 2. destination
    4 │ );
    5 │ alter type t drop attribute a;
      ╰╴                            ─ 1. source
    ");
}

#[test]
fn alter_type_added_attribute_ref() {
    assert_snapshot!(goto("
create type t as (
  a int
);
alter type t add attribute b text;
select (null::t).b$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:5:28
      │
    5 │ alter type t add attribute b text;
      │                            ─ 2. destination
    6 │ select (null::t).b;
      ╰╴                 ─ 1. source
    ");
}

#[test]
fn alter_type_renamed_attribute_ref() {
    assert_snapshot!(goto("
create type t as (
  a int
);
alter type t rename attribute a to x;
select (null::t).x$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:5:36
      │
    5 │ alter type t rename attribute a to x;
      │                                    ─ 2. destination
    6 │ select (null::t).x;
      ╰╴                 ─ 1. source
    ");
}

#[test]
fn comment_on_column_composite_type_attribute() {
    assert_snapshot!(goto("
create type address as (
  city text,
  zip text
);
comment on column address.zip$0 is 'x';
"), @"
    info: goto definition
      ╭▸ file0.sql:4:3
      │
    4 │   zip text
      │   ─── 2. destination
    5 │ );
    6 │ comment on column address.zip is 'x';
      ╰╴                          ─── 1. source
    ");
}

#[test]
fn comment_on_column_composite_type_name() {
    assert_snapshot!(goto("
create type address as (
  city text,
  zip text
);
comment on column address$0.zip is 'x';
"), @"
    info: goto definition
      ╭▸ file0.sql:2:13
      │
    2 │ create type address as (
      │             ─────── 2. destination
      ‡
    6 │ comment on column address.zip is 'x';
      ╰╴                  ─────── 1. source
    ");
}

#[test]
fn comment_on_column_schema_qualified_composite_type_attribute() {
    assert_snapshot!(goto("
create schema s;
create type s.address as (
  city text,
  zip text
);
comment on column s.address.city$0 is 'x';
"), @"
    info: goto definition
      ╭▸ file0.sql:4:3
      │
    4 │   city text,
      │   ──── 2. destination
      ‡
    7 │ comment on column s.address.city is 'x';
      ╰╴                            ──── 1. source
    ");
}

#[test]
fn security_label_on_column_composite_type_attribute() {
    assert_snapshot!(goto("
create type address as (
  city text,
  zip text
);
security label on column address.city$0 is 'x';
"), @"
    info: goto definition
      ╭▸ file0.sql:3:3
      │
    3 │   city text,
      │   ──── 2. destination
      ‡
    6 │ security label on column address.city is 'x';
      ╰╴                                 ──── 1. source
    ");
}

#[test]
fn percent_type_composite_type_attribute() {
    assert_snapshot!(goto("
create type address as (
  city text,
  zip text
);
create function f(x address.city$0%type) returns int
  language sql
  as 'select 1';
"), @"
    info: goto definition
      ╭▸ file0.sql:3:3
      │
    3 │   city text,
      │   ──── 2. destination
      ‡
    6 │ create function f(x address.city%type) returns int
      ╰╴                            ──── 1. source
    ");
}
