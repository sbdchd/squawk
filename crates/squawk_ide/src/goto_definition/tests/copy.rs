use insta::assert_snapshot;

use super::goto;

#[test]
fn copy_table_ref() {
    assert_snapshot!(goto("
create table t (a int);
copy t$0 from stdin;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:14
      │
    2 │ create table t (a int);
      │              ─ 2. destination
    3 │ copy t from stdin;
      ╰╴     ─ 1. source
    ");
}

#[test]
fn copy_column_ref() {
    assert_snapshot!(goto("
create table t (
  a int,
  b int
);
copy t (a, b$0) to stdout;
"), @"
    info: goto definition
      ╭▸ file0.sql:4:3
      │
    4 │   b int
      │   ─ 2. destination
    5 │ );
    6 │ copy t (a, b) to stdout;
      ╰╴           ─ 1. source
    ");
}

#[test]
fn copy_where_column_ref() {
    assert_snapshot!(goto("
create table t (a int);
copy t from stdin where a$0 > 1;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create table t (a int);
      │                 ─ 2. destination
    3 │ copy t from stdin where a > 1;
      ╰╴                        ─ 1. source
    ");
}

#[test]
fn copy_force_quote_column_ref() {
    assert_snapshot!(goto("
create table t (a int);
copy t to stdout with (format csv, force_quote (a$0));
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create table t (a int);
      │                 ─ 2. destination
    3 │ copy t to stdout with (format csv, force_quote (a));
      ╰╴                                                ─ 1. source
    ");
}

#[test]
fn copy_legacy_force_quote_column_ref() {
    assert_snapshot!(goto("
create table t (a int);
copy t to stdout csv force quote a$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create table t (a int);
      │                 ─ 2. destination
    3 │ copy t to stdout csv force quote a;
      ╰╴                                 ─ 1. source
    ");
}

#[test]
fn copy_query_table_ref() {
    assert_snapshot!(goto("
create table t (a int);
copy (select a from t$0) to stdout;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:14
      │
    2 │ create table t (a int);
      │              ─ 2. destination
    3 │ copy (select a from t) to stdout;
      ╰╴                    ─ 1. source
    ");
}
