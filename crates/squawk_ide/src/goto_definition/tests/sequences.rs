use insta::assert_snapshot;

use super::goto;

#[test]
fn sequence_alter() {
    assert_snapshot!(goto("
create sequence s;
alter sequence s$0 restart;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create sequence s;
      │                 ─ 2. destination
    3 │ alter sequence s restart;
      ╰╴               ─ 1. source
    ");
}

#[test]
fn sequence_drop() {
    assert_snapshot!(goto("
create schema app;
create sequence app.s;
drop sequence app.s$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:21
      │
    3 │ create sequence app.s;
      │                     ─ 2. destination
    4 │ drop sequence app.s;
      ╰╴                  ─ 1. source
    ");
}

#[test]
fn sequence_select() {
    assert_snapshot!(goto("
create sequence s;
select last_value from s$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create sequence s;
      │                 ─ 2. destination
    3 │ select last_value from s;
      ╰╴                       ─ 1. source
    ");
}

#[test]
fn sequence_column() {
    assert_snapshot!(goto("
create sequence s;
select last_value$0 from s;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create sequence s;
      │                 ─ 2. destination
    3 │ select last_value from s;
      ╰╴       ────────── 1. source
    ");
}

#[test]
fn sequence_owned_by() {
    assert_snapshot!(goto("
create table t (a int);
create sequence s owned by t.a$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create table t (a int);
      │                 ─ 2. destination
    3 │ create sequence s owned by t.a;
      ╰╴                             ─ 1. source
    ");
}

#[test]
fn sequence_owned_by_table() {
    assert_snapshot!(goto("
create table t (a int);
create sequence s owned by t$0.a;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:14
      │
    2 │ create table t (a int);
      │              ─ 2. destination
    3 │ create sequence s owned by t.a;
      ╰╴                           ─ 1. source
    ");
}

#[test]
fn sequence_as_type() {
    assert_snapshot!(goto("
create domain d as bigint;
create sequence s as d$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:15
      │
    2 │ create domain d as bigint;
      │               ─ 2. destination
    3 │ create sequence s as d;
      ╰╴                     ─ 1. source
    ");
}

#[test]
fn identity_sequence() {
    assert_snapshot!(goto("
create table t (id int generated always as identity);
alter sequence t_id_seq$0 restart;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create table t (id int generated always as identity);
      │                 ── 2. destination
    3 │ alter sequence t_id_seq restart;
      ╰╴               ──────── 1. source
    ");
}

#[test]
fn identity_sequence_name() {
    assert_snapshot!(goto("
create table t (
  id int generated always as identity (
    sequence name s
  )
);
alter sequence s$0 restart;
"), @"
    info: goto definition
      ╭▸ file0.sql:4:19
      │
    4 │     sequence name s
      │                   ─ 2. destination
      ‡
    7 │ alter sequence s restart;
      ╰╴               ─ 1. source
    ");
}

#[test]
fn identity_sequence_as_type() {
    assert_snapshot!(goto("
create domain d as bigint;
create table t (
  id d generated always as identity (
    as d$0
  )
);
"), @"
    info: goto definition
      ╭▸ file0.sql:2:15
      │
    2 │ create domain d as bigint;
      │               ─ 2. destination
      ‡
    5 │     as d
      ╰╴       ─ 1. source
    ");
}

#[test]
fn sequence_nextval() {
    assert_snapshot!(goto("
create sequence s;
select nextval('s$0');
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create sequence s;
      │                 ─ 2. destination
    3 │ select nextval('s');
      ╰╴                ─ 1. source
    ");
}

#[test]
fn sequence_nextval_qualified() {
    assert_snapshot!(goto("
create schema app;
create sequence app.s;
select nextval('app.s$0'::regclass);
"), @"
    info: goto definition
      ╭▸ file0.sql:3:21
      │
    3 │ create sequence app.s;
      │                     ─ 2. destination
    4 │ select nextval('app.s'::regclass);
      ╰╴                    ─ 1. source
    ");
}

#[test]
fn sequence_nextval_schema() {
    assert_snapshot!(goto("
create schema app;
create sequence app.s;
select nextval('app$0.s');
"), @"
    info: goto definition
      ╭▸ file0.sql:2:15
      │
    2 │ create schema app;
      │               ─── 2. destination
    3 │ create sequence app.s;
    4 │ select nextval('app.s');
      ╰╴                ─── 1. source
    ");
}

#[test]
fn sequence_nextval_quoted() {
    assert_snapshot!(goto(r#"
create sequence "My Seq";
select nextval('"My Seq"$0');
"#), @r#"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create sequence "My Seq";
      │                 ──────── 2. destination
    3 │ select nextval('"My Seq"');
      ╰╴                ──────── 1. source
    "#);
}

#[test]
fn alter_sequence_renamed_ref() {
    assert_snapshot!(goto("
create sequence s;
alter sequence s rename to s2;
select last_value from s2$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:28
      │
    3 │ alter sequence s rename to s2;
      │                            ── 2. destination
    4 │ select last_value from s2;
      ╰╴                       ── 1. source
    ");
}

#[test]
fn serial_column_type() {
    assert_snapshot!(goto("
create type pg_catalog.int8;
create table t (a bigserial$0);
"), @"
    info: goto definition
      ╭▸ file0.sql:2:24
      │
    2 │ create type pg_catalog.int8;
      │                        ──── 2. destination
    3 │ create table t (a bigserial);
      ╰╴                  ───────── 1. source
    ");
}
