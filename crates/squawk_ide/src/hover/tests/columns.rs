use insta::assert_snapshot;

use super::hover;

#[test]
fn column_definition() {
    assert_snapshot!(hover("
create table t (
  a$0 int,
  b text
);
"), @"
    hover: column public.t.a int4
      ╭▸ file0.sql:3:3
      │
    3 │   a int,
      ╰╴  ─ hover
    ");
}

#[test]
fn column_reference() {
    assert_snapshot!(hover("
create table t (
  a int,
  b text
);
select b$0 from t;
"), @"
    hover: column public.t.b text
      ╭▸ file0.sql:6:8
      │
    6 │ select b from t;
      ╰╴       ─ hover
    ");
}

#[test]
fn qualified_column() {
    assert_snapshot!(hover("
create table t (
  a int,
  b text
);
select t.b$0 from t;
"), @"
    hover: column public.t.b text
      ╭▸ file0.sql:6:10
      │
    6 │ select t.b from t;
      ╰╴         ─ hover
    ");
}

#[test]
fn column_in_create_index() {
    assert_snapshot!(hover("
create table users (
  id int,
  email text
);
create index idx_email on users (email$0);
"), @"
    hover: column public.users.email text
      ╭▸ file0.sql:6:34
      │
    6 │ create index idx_email on users (email);
      ╰╴                                 ───── hover
    ");
}

#[test]
fn column_varchar() {
    assert_snapshot!(hover("
create table t (name varchar(100));
select name$0 from t;
"), @"
    hover: column public.t.name varchar
      ╭▸ file0.sql:3:8
      │
    3 │ select name from t;
      ╰╴       ──── hover
    ");
}

#[test]
fn column_domain() {
    assert_snapshot!(hover("
create domain d as int;
create table t (a d);
select a$0 from t;
"), @"
    hover: column public.t.a d
      ╭▸ file0.sql:4:8
      │
    4 │ select a from t;
      ╰╴       ─ hover
    ");
}

#[test]
fn column_array() {
    assert_snapshot!(hover("
create table t (a int[]);
select a$0 from t;
"), @"
    hover: column public.t.a int4[]
      ╭▸ file0.sql:3:8
      │
    3 │ select a from t;
      ╰╴       ─ hover
    ");
}

#[test]
fn serial_column() {
    assert_snapshot!(hover("
create table t (id serial);
select id$0 from t;
"), @"
    hover: column public.t.id int4
      ╭▸ file0.sql:3:8
      │
    3 │ select id from t;
      ╰╴       ── hover
    ");
}

#[test]
fn column_in_insert() {
    assert_snapshot!(hover("
create table t (
  a int,
  b text
);
insert into t (b$0) values ('x');
"), @"
    hover: column public.t.b text
      ╭▸ file0.sql:6:16
      │
    6 │ insert into t (b) values ('x');
      ╰╴               ─ hover
    ");
}

#[test]
fn column_in_update() {
    assert_snapshot!(hover("
create table t (
  a int,
  b text
);
update t set b$0 = 'x';
"), @"
    hover: column public.t.b text
      ╭▸ file0.sql:6:14
      │
    6 │ update t set b = 'x';
      ╰╴             ─ hover
    ");
}

#[test]
fn column_in_where() {
    assert_snapshot!(hover("
create table t (
  a int,
  b text
);
select * from t where a$0 = 1;
"), @"
    hover: column public.t.a int4
      ╭▸ file0.sql:6:23
      │
    6 │ select * from t where a = 1;
      ╰╴                      ─ hover
    ");
}

#[test]
fn view_column() {
    assert_snapshot!(hover("
create table t (a int);
create view v as
  select a from t;
select a$0 from v;
"), @"
    hover: column public.v.a int4
      ╭▸ file0.sql:5:8
      │
    5 │ select a from v;
      ╰╴       ─ hover
    ");
}

#[test]
fn using_join_column() {
    assert_snapshot!(hover("
create table t (id int);
create table u (id int);
select id$0 from t join u using (id);
"), @"
    hover: column public.t.id int4
      ╭▸ file0.sql:4:8
      │
    4 │ select id from t join u using (id);
      ╰╴       ── hover
    ");
}

#[test]
fn composite_type_field() {
    assert_snapshot!(hover("
create type pair as (
  a int,
  b text
);
create table t (p pair);
select (p).b$0 from t;
"), @"
    hover: field public.pair.b text
      ╭▸ file0.sql:7:12
      │
    7 │ select (p).b from t;
      ╰╴           ─ hover
    ");
}

#[test]
fn function_returns_table_column() {
    assert_snapshot!(hover("
create function f() returns table (a int, b text)
  language sql
  as 'select 1, ''a''';
select b$0 from f();
"), @"
    hover: column public.f.b text
      ╭▸ file0.sql:5:8
      │
    5 │ select b from f();
      ╰╴       ─ hover
    ");
}

#[test]
fn renamed_column() {
    assert_snapshot!(hover("
create table t (a int);
alter table t rename column a to b;
select b$0 from t;
"), @"
    hover: column public.t.b int4
      ╭▸ file0.sql:4:8
      │
    4 │ select b from t;
      ╰╴       ─ hover
    ");
}

#[test]
fn builtin_column() {
    assert_snapshot!(hover("
select relname$0 from pg_class;
"), @"
    hover: column pg_catalog.pg_class.relname name
      ╭▸ file0.sql:2:8
      │
    2 │ select relname from pg_class;
      ╰╴       ─────── hover
    ");
}

#[test]
fn added_column() {
    assert_snapshot!(hover("
create table t (a int);
alter table t add column b text;
select b$0 from t;
"), @"
    hover: column public.t.b text
      ╭▸ file0.sql:4:8
      │
    4 │ select b from t;
      ╰╴       ─ hover
    ");
}

#[test]
fn copied_column_from_like() {
    assert_snapshot!(hover("
create table t (a int);
create table u (like t);
select a$0 from u;
"), @"
    hover: column public.u.a int4
      ╭▸ file0.sql:4:8
      │
    4 │ select a from u;
      ╰╴       ─ hover
    ");
}

#[test]
fn replaced_view_column() {
    assert_snapshot!(hover("
create view v as
  select 1 a;
create or replace view v as
  select 1 a, 'b' b;
select b$0 from v;
"), @"
    hover: column public.v.b text
      ╭▸ file0.sql:6:8
      │
    6 │ select b from v;
      ╰╴       ─ hover
    ");
}

#[test]
fn moved_function_column() {
    assert_snapshot!(hover("
create schema s;
create function f() returns table (a int)
  language sql
  as 'select 1';
alter function f() set schema s;
select a$0 from s.f();
"), @"
    hover: column s.f.a int4
      ╭▸ file0.sql:7:8
      │
    7 │ select a from s.f();
      ╰╴       ─ hover
    ");
}

#[test]
fn table_column_after_view() {
    assert_snapshot!(hover("
create table t (a int);
create view v as select * from t;
select a$0 from t;
"), @"
    hover: column public.t.a int4
      ╭▸ file0.sql:4:8
      │
    4 │ select a from t;
      ╰╴       ─ hover
    ");
}

#[test]
fn table_column_after_cte() {
    assert_snapshot!(hover("
create table t (a int);
with c as (select * from t)
select a$0 from t;
"), @"
    hover: column public.t.a int4
      ╭▸ file0.sql:4:8
      │
    4 │ select a from t;
      ╰╴       ─ hover
    ");
}

#[test]
fn cte_from_table_column() {
    assert_snapshot!(hover("
create table t (a int);
with c as (table t)
select a$0 from c;
"), @"
    hover: column c.a int4
      ╭▸ file0.sql:4:8
      │
    4 │ select a from c;
      ╰╴       ─ hover
    ");
}

#[test]
fn full_join_using_column() {
    assert_snapshot!(hover("
create table t (id int);
create table u (id int);
select id$0 from t full join u using (id);
"), @"
    hover: column public.t.id int4
          column public.u.id int4
      ╭▸ file0.sql:4:8
      │
    4 │ select id from t full join u using (id);
      ╰╴       ── hover
    ");
}
