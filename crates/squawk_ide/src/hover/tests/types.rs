use insta::assert_snapshot;

use super::hover;

#[test]
fn base_type_reference() {
    assert_snapshot!(hover("
create type foo;
create table t (a foo$0);
"), @"
    hover: shell type public.foo
      ╭▸ file0.sql:3:19
      │
    3 │ create table t (a foo);
      ╰╴                  ─── hover
    ");
}

#[test]
fn builtin_type() {
    assert_snapshot!(hover("
create table t (a int4$0);
"), @"
    hover: type pg_catalog.int4
      ╭▸ file0.sql:2:19
      │
    2 │ create table t (a int4);
      │                   ──── hover
      │
      ╰ note: -2 billion to 2 billion integer, 4-byte storage
              size: 4, align: 4
    ");
}

#[test]
fn composite_type() {
    assert_snapshot!(hover("
create type pair as (
  a int,
  b text
);
create table t (p pair$0);
"), @"
    hover: type public.pair as (a int4, b text)
      ╭▸ file0.sql:6:19
      │
    6 │ create table t (p pair);
      ╰╴                  ──── hover
    ");
}

#[test]
fn enum_type() {
    assert_snapshot!(hover("
create type mood as enum (
  'happy',
  'sad'
);
create table t (m mood$0);
"), @"
    hover: type public.mood as enum ('happy', 'sad')
      ╭▸ file0.sql:6:19
      │
    6 │ create table t (m mood);
      ╰╴                  ──── hover
    ");
}

#[test]
fn enum_label() {
    assert_snapshot!(hover("
create type mood as enum (
  'happy',
  'sad'
);
create table t (m mood);
select * from t where m = 'sad$0';
"), @"
    hover: enum label 'sad' of public.mood
      ╭▸ file0.sql:7:27
      │
    7 │ select * from t where m = 'sad';
      ╰╴                          ───── hover
    ");
}

#[test]
fn domain() {
    assert_snapshot!(hover("
create domain posint as int check (value > 0);
create table t (a posint$0);
"), @"
    hover: domain public.posint as int4
      ╭▸ file0.sql:3:19
      │
    3 │ create table t (a posint);
      ╰╴                  ────── hover
    ");
}

#[test]
fn range_type() {
    assert_snapshot!(hover("
create type floatrange as range (subtype = float8);
select '[1,2]'::floatrange$0;
"), @"
    hover: type public.floatrange as range (subtype = float8)
      ╭▸ file0.sql:3:17
      │
    3 │ select '[1,2]'::floatrange;
      ╰╴                ────────── hover
    ");
}

#[test]
fn multirange_type() {
    assert_snapshot!(hover("
create type floatrange as range (subtype = float8);
select '{}'::floatmultirange$0;
"), @"
    hover: multirange type public.floatmultirange
      ╭▸ file0.sql:3:14
      │
    3 │ select '{}'::floatmultirange;
      ╰╴             ─────────────── hover
    ");
}

#[test]
fn cast_type() {
    assert_snapshot!(hover("
create type foo as (
  a int
);
select null::foo$0;
"), @"
    hover: type public.foo as (a int4)
      ╭▸ file0.sql:5:14
      │
    5 │ select null::foo;
      ╰╴             ─── hover
    ");
}

#[test]
fn array_type() {
    assert_snapshot!(hover("
create type foo as (
  a int
);
select null::foo$0[];
"), @"
    hover: type public.foo as (a int4)
      ╭▸ file0.sql:5:14
      │
    5 │ select null::foo[];
      ╰╴             ─── hover
    ");
}

#[test]
fn domain_constraint() {
    assert_snapshot!(hover("
create domain posint as int constraint positive$0 check (value > 0);
"), @"
    hover: constraint positive on domain public.posint
      ╭▸ file0.sql:2:40
      │
    2 │ create domain posint as int constraint positive check (value > 0);
      ╰╴                                       ──────── hover
    ");
}

#[test]
fn enum_type_with_added_value() {
    assert_snapshot!(hover("
create type mood as enum (
  'happy',
  'sad'
);
create type other as enum (
  'x'
);
alter type mood add value 'ok';
create table t (m mood$0);
"), @"
    hover: type public.mood as enum ('happy', 'sad', 'ok')
       ╭▸ file0.sql:10:19
       │
    10 │ create table t (m mood);
       ╰╴                  ──── hover
    ");
}

#[test]
fn added_enum_label() {
    assert_snapshot!(hover("
create type mood as enum (
  'happy',
  'sad'
);
alter type mood add value 'ok';
create table t (m mood);
select * from t where m = 'ok$0';
"), @"
    hover: enum label 'ok' of public.mood
      ╭▸ file0.sql:8:27
      │
    8 │ select * from t where m = 'ok';
      ╰╴                          ──── hover
    ");
}

#[test]
fn enum_value_added_before() {
    assert_snapshot!(hover("
create type mood as enum (
  'happy',
  'sad'
);
alter type mood add value 'ok' before 'sad';
create table t (m mood$0);
"), @"
    hover: type public.mood as enum ('happy', 'ok', 'sad')
      ╭▸ file0.sql:7:19
      │
    7 │ create table t (m mood);
      ╰╴                  ──── hover
    ");
}

#[test]
fn enum_value_added_after() {
    assert_snapshot!(hover("
create type mood as enum (
  'happy',
  'sad'
);
alter type mood add value 'ok' after 'happy';
create table t (m mood$0);
"), @"
    hover: type public.mood as enum ('happy', 'ok', 'sad')
      ╭▸ file0.sql:7:19
      │
    7 │ create table t (m mood);
      ╰╴                  ──── hover
    ");
}

#[test]
fn enum_value_added_after_last() {
    assert_snapshot!(hover("
create type mood as enum (
  'happy',
  'sad'
);
alter type mood add value 'ok' after 'sad';
alter type mood add value 'meh' before 'ok';
create table t (m mood$0);
"), @"
    hover: type public.mood as enum ('happy', 'sad', 'meh', 'ok')
      ╭▸ file0.sql:8:19
      │
    8 │ create table t (m mood);
      ╰╴                  ──── hover
    ");
}

#[test]
fn enum_value_added_in_rolled_back_transaction() {
    assert_snapshot!(hover("
create type mood as enum (
  'happy',
  'sad'
);
begin;
alter type mood add value 'ok';
rollback;
create table t (m mood$0);
"), @"
    hover: type public.mood as enum ('happy', 'sad')
      ╭▸ file0.sql:9:19
      │
    9 │ create table t (m mood);
      ╰╴                  ──── hover
    ");
}
