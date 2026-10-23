use insta::assert_snapshot;

use super::hover;

#[test]
fn index() {
    assert_snapshot!(hover("
create table t (a int);
create index i on t (a);
drop index i$0;
"), @"
    hover: index public.i on public.t
      ╭▸ file0.sql:4:12
      │
    4 │ drop index i;
      ╰╴           ─ hover
    ");
}

#[test]
fn constraint() {
    assert_snapshot!(hover("
create table t (
  a int,
  constraint t_pk primary key (a)
);
alter table t drop constraint t_pk$0;
"), @"
    hover: constraint t_pk on public.t
      ╭▸ file0.sql:6:31
      │
    6 │ alter table t drop constraint t_pk;
      ╰╴                              ──── hover
    ");
}

#[test]
fn trigger() {
    assert_snapshot!(hover("
create table t (a int);
create function f() returns trigger
  language plpgsql
  as 'begin return new; end';
create trigger tr
  after insert
  on t
  for each row
  execute function f();
drop trigger tr$0 on t;
"), @"
    hover: trigger tr on public.t
       ╭▸ file0.sql:11:14
       │
    11 │ drop trigger tr on t;
       ╰╴             ── hover
    ");
}

#[test]
fn rule() {
    assert_snapshot!(hover("
create table t (a int);
create rule r as on insert
  to t
  do nothing;
drop rule r$0 on t;
"), @"
    hover: rule r on public.t
      ╭▸ file0.sql:6:11
      │
    6 │ drop rule r on t;
      ╰╴          ─ hover
    ");
}

#[test]
fn policy() {
    assert_snapshot!(hover("
create table t (a int);
create policy p on t;
drop policy p$0 on t;
"), @"
    hover: policy p on public.t
      ╭▸ file0.sql:4:13
      │
    4 │ drop policy p on t;
      ╰╴            ─ hover
    ");
}

#[test]
fn role() {
    assert_snapshot!(hover("
create role r;
drop role r$0;
"), @"
    hover: role r
      ╭▸ file0.sql:3:11
      │
    3 │ drop role r;
      ╰╴          ─ hover
    ");
}

#[test]
fn schema() {
    assert_snapshot!(hover("
create schema s;
create table s$0.t (a int);
"), @"
    hover: schema s
      ╭▸ file0.sql:3:14
      │
    3 │ create table s.t (a int);
      ╰╴             ─ hover
    ");
}

#[test]
fn builtin_schema() {
    assert_snapshot!(hover("
create table pg_catalog$0.t (a int);
"), @"
    hover: schema pg_catalog
      ╭▸ file0.sql:2:14
      │
    2 │ create table pg_catalog.t (a int);
      │              ────────── hover
      │
      ╰ note: system catalog schema
    ");
}

#[test]
fn tablespace() {
    assert_snapshot!(hover("
create tablespace ts
  location '/tmp';
create table t (a int) tablespace ts$0;
"), @"
    hover: tablespace ts
      ╭▸ file0.sql:4:35
      │
    4 │ create table t (a int) tablespace ts;
      ╰╴                                  ── hover
    ");
}

#[test]
fn database() {
    assert_snapshot!(hover("
create database d;
drop database d$0;
"), @"
    hover: database d
      ╭▸ file0.sql:3:15
      │
    3 │ drop database d;
      ╰╴              ─ hover
    ");
}

#[test]
fn extension() {
    assert_snapshot!(hover("
create extension e;
drop extension e$0;
"), @"
    hover: extension e
      ╭▸ file0.sql:3:16
      │
    3 │ drop extension e;
      ╰╴               ─ hover
    ");
}

#[test]
fn access_method() {
    assert_snapshot!(hover("
create access method am
  type table
  handler heap_tableam_handler;
create table t (a int) using am$0;
"), @"
    hover: access method am
      ╭▸ file0.sql:5:30
      │
    5 │ create table t (a int) using am;
      ╰╴                             ── hover
    ");
}

#[test]
fn event_trigger() {
    assert_snapshot!(hover("
create function f() returns event_trigger
  language plpgsql
  as 'begin end';
create event trigger et on ddl_command_start execute function f();
drop event trigger et$0;
"), @"
    hover: event trigger et
      ╭▸ file0.sql:6:20
      │
    6 │ drop event trigger et;
      ╰╴                   ── hover
    ");
}

#[test]
fn language() {
    assert_snapshot!(hover("
create function f() returns int
  language sql$0
  return 1;
"), @"
    hover: language sql
      ╭▸ file0.sql:3:12
      │
    3 │   language sql
      │            ─── hover
      │
      ╰ note: SQL-language functions
    ");
}

#[test]
fn foreign_data_wrapper() {
    assert_snapshot!(hover("
create foreign data wrapper w;
drop foreign data wrapper w$0;
"), @"
    hover: foreign data wrapper w
      ╭▸ file0.sql:3:27
      │
    3 │ drop foreign data wrapper w;
      ╰╴                          ─ hover
    ");
}

#[test]
fn server() {
    assert_snapshot!(hover("
create foreign data wrapper w;
create server s foreign data wrapper w;
drop server s$0;
"), @"
    hover: server s
      ╭▸ file0.sql:4:13
      │
    4 │ drop server s;
      ╰╴            ─ hover
    ");
}

#[test]
fn conversion() {
    assert_snapshot!(hover("
create conversion c for 'LATIN1' to 'UTF8' from iso8859_1_to_utf8;
drop conversion c$0;
"), @"
    hover: conversion public.c
      ╭▸ file0.sql:3:17
      │
    3 │ drop conversion c;
      ╰╴                ─ hover
    ");
}

#[test]
fn operator_family() {
    assert_snapshot!(hover("
create operator family f
  using btree;
drop operator family f$0 using btree;
"), @"
    hover: operator family public.f using btree
      ╭▸ file0.sql:4:22
      │
    4 │ drop operator family f using btree;
      ╰╴                     ─ hover
    ");
}

#[test]
fn operator_class() {
    assert_snapshot!(hover("
create operator class c
  for type int
  using btree as
  operator 1 <;
drop operator class c$0 using btree;
"), @"
    hover: operator class public.c using btree
      ╭▸ file0.sql:6:21
      │
    6 │ drop operator class c using btree;
      ╰╴                    ─ hover
    ");
}

#[test]
fn statistics() {
    assert_snapshot!(hover("
create table t (
  a int,
  b int
);
create statistics s on a, b from t;
drop statistics s$0;
"), @"
    hover: statistics public.s on public.t
      ╭▸ file0.sql:7:17
      │
    7 │ drop statistics s;
      ╰╴                ─ hover
    ");
}

#[test]
fn text_search_configuration() {
    assert_snapshot!(hover("
create text search configuration c (
  copy = english
);
drop text search configuration c$0;
"), @"
    hover: text search configuration public.c
      ╭▸ file0.sql:5:32
      │
    5 │ drop text search configuration c;
      ╰╴                               ─ hover
    ");
}

#[test]
fn text_search_dictionary() {
    assert_snapshot!(hover("
create text search dictionary d (
  template = simple
);
drop text search dictionary d$0;
"), @"
    hover: text search dictionary public.d
      ╭▸ file0.sql:5:29
      │
    5 │ drop text search dictionary d;
      ╰╴                            ─ hover
    ");
}

#[test]
fn publication() {
    assert_snapshot!(hover("
create publication p;
drop publication p$0;
"), @"
    hover: publication p
      ╭▸ file0.sql:3:18
      │
    3 │ drop publication p;
      ╰╴                 ─ hover
    ");
}

#[test]
fn subscription() {
    assert_snapshot!(hover("
create subscription s connection '' publication p;
drop subscription s$0;
"), @"
    hover: subscription s
      ╭▸ file0.sql:3:19
      │
    3 │ drop subscription s;
      ╰╴                  ─ hover
    ");
}

#[test]
fn collation() {
    assert_snapshot!(hover("
create collation c (
  provider = icu,
  locale = 'und'
);
select 'a' collate c$0;
"), @"
    hover: collation public.c
      ╭▸ file0.sql:6:20
      │
    6 │ select 'a' collate c;
      ╰╴                   ─ hover
    ");
}

#[test]
fn window() {
    assert_snapshot!(hover("
select row_number() over w$0 from (select 1 a) t window w as (order by a);
"), @"
    hover: window w
      ╭▸ file0.sql:2:26
      │
    2 │ select row_number() over w from (select 1 a) t window w as (order by a);
      ╰╴                         ─ hover
    ");
}
