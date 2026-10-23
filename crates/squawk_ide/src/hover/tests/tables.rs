use insta::assert_snapshot;

use super::{hover, hover_files};

#[test]
fn table_definition() {
    assert_snapshot!(hover("
create table t$0 (
  a int,
  b text
);
"), @"
    hover: table public.t(a int4, b text)
      ╭▸ file0.sql:2:14
      │
    2 │ create table t (
      ╰╴             ─ hover
    ");
}

#[test]
fn table_reference() {
    assert_snapshot!(hover("
create table t (
  a int,
  b text
);
select * from t$0;
"), @"
    hover: table public.t(a int4, b text)
      ╭▸ file0.sql:6:15
      │
    6 │ select * from t;
      ╰╴              ─ hover
    ");
}

#[test]
fn table_schema_qualified() {
    assert_snapshot!(hover("
create schema s;
create table s.t (a int);
select * from s.t$0;
"), @"
    hover: table s.t(a int4)
      ╭▸ file0.sql:4:17
      │
    4 │ select * from s.t;
      ╰╴                ─ hover
    ");
}

#[test]
fn temp_table() {
    assert_snapshot!(hover("
create temp table t (a int);
select * from t$0;
"), @"
    hover: table pg_temp.t(a int4)
      ╭▸ file0.sql:3:15
      │
    3 │ select * from t;
      ╰╴              ─ hover
    ");
}

#[test]
fn foreign_table() {
    assert_snapshot!(hover("
create foreign data wrapper w;
create server s foreign data wrapper w;
create foreign table t (a int) server s;
select * from t$0;
"), @"
    hover: foreign table public.t(a int4)
      ╭▸ file0.sql:5:15
      │
    5 │ select * from t;
      ╰╴              ─ hover
    ");
}

#[test]
fn view() {
    assert_snapshot!(hover("
create table t (
  a int,
  b text
);
create view v as
  select a, b from t;
select * from v$0;
"), @"
    hover: view public.v(a int4, b text)
      ╭▸ file0.sql:8:15
      │
    8 │ select * from v;
      ╰╴              ─ hover
    ");
}

#[test]
fn view_column_list() {
    assert_snapshot!(hover("
create view v (x, y) as
  select 1, 'a';
select * from v$0;
"), @"
    hover: view public.v(x int4, y text)
      ╭▸ file0.sql:4:15
      │
    4 │ select * from v;
      ╰╴              ─ hover
    ");
}

#[test]
fn materialized_view() {
    assert_snapshot!(hover("
create materialized view v as
  select 1 a;
select * from v$0;
"), @"
    hover: materialized view public.v(a int4)
      ╭▸ file0.sql:4:15
      │
    4 │ select * from v;
      ╰╴              ─ hover
    ");
}

#[test]
fn sequence() {
    assert_snapshot!(hover("
create sequence s;
select nextval('s$0');
"), @"
    hover: sequence public.s
      ╭▸ file0.sql:3:17
      │
    3 │ select nextval('s');
      ╰╴                ─ hover
    ");
}

#[test]
fn drop_sequence() {
    assert_snapshot!(hover("
create sequence s;
drop sequence s$0;
"), @"
    hover: sequence public.s
      ╭▸ file0.sql:3:15
      │
    3 │ drop sequence s;
      ╰╴              ─ hover
    ");
}

#[test]
fn table_alias() {
    assert_snapshot!(hover("
create table t (a int);
select x$0.a from t as x;
"), @"
    hover: table public.t(a int4)
      ╭▸ file0.sql:3:8
      │
    3 │ select x.a from t as x;
      ╰╴       ─ hover
    ");
}

#[test]
fn table_alias_definition() {
    assert_snapshot!(hover("
create table t (a int);
select x.a from t as x$0;
"), @"
    hover: table public.t(a int4)
      ╭▸ file0.sql:3:22
      │
    3 │ select x.a from t as x;
      ╰╴                     ─ hover
    ");
}

#[test]
fn create_table_as() {
    assert_snapshot!(hover("
create table t as
  select 1 a, 'b' b;
select * from t$0;
"), @"
    hover: table public.t(a int4, b text)
      ╭▸ file0.sql:4:15
      │
    4 │ select * from t;
      ╰╴              ─ hover
    ");
}

#[test]
fn table_like() {
    assert_snapshot!(hover("
create table t (a int);
create table u (like t);
select * from u$0;
"), @"
    hover: table public.u(a int4)
      ╭▸ file0.sql:4:15
      │
    4 │ select * from u;
      ╰╴              ─ hover
    ");
}

#[test]
fn renamed_table() {
    assert_snapshot!(hover("
create table t (a int);
alter table t rename to u;
select * from u$0;
"), @"
    hover: table public.u(a int4)
      ╭▸ file0.sql:4:15
      │
    4 │ select * from u;
      ╰╴              ─ hover
    ");
}

#[test]
fn builtin_catalog() {
    assert_snapshot!(hover("
select * from pg_class$0;
"), @"
    hover: table pg_catalog.pg_class(oid oid, relname name, relnamespace oid, reltype oid, reloftype oid, relowner oid, relam oid, relfilenode oid, reltablespace oid, relpages int4, reltuples float4, relallvisible int4, relallfrozen int4, reltoastrelid oid, relhasindex bool, relisshared bool, relpersistence char, relkind char, relnatts int2, relchecks int2, relhasrules bool, relhastriggers bool, relhassubclass bool, relrowsecurity bool, relforcerowsecurity bool, relispopulated bool, relreplident char, relispartition bool, relrewrite oid, relfrozenxid xid, relminmxid xid, relacl aclitem[], reloptions text[], relpartbound pg_node_tree)
      ╭▸ file0.sql:2:15
      │
    2 │ select * from pg_class;
      ╰╴              ──────── hover
    ");
}

#[test]
fn multiple_files() {
    assert_snapshot!(hover_files(&["
create table t (a int);
", "
select * from t$0;
"]), @"
    hover: table public.t(a int4)
      ╭▸ file1.sql:2:15
      │
    2 │ select * from t;
      ╰╴              ─ hover
    ");
}
