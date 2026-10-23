use insta::assert_snapshot;

use super::goto;

#[test]
fn drop_table() {
    assert_snapshot!(goto("
create table t (a int);
drop table t$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:14
      │
    2 │ create table t (a int);
      │              ─ 2. destination
    3 │ drop table t;
      ╰╴           ─ 1. source
    ");
}

#[test]
fn drop_table_qualified_list() {
    assert_snapshot!(goto("
create table app.a (x int);
create table app.b (x int);
drop table if exists app.a, app.b$0 cascade;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:18
      │
    3 │ create table app.b (x int);
      │                  ─ 2. destination
    4 │ drop table if exists app.a, app.b cascade;
      ╰╴                                ─ 1. source
    ");
}

#[test]
fn recreated_table_after_drop() {
    assert_snapshot!(goto("
create table t (a int);
drop table t;
create table t (b int);
select b$0 from t;
"), @"
    info: goto definition
      ╭▸ file0.sql:4:17
      │
    4 │ create table t (b int);
      │                 ─ 2. destination
    5 │ select b from t;
      ╰╴       ─ 1. source
    ");
}

#[test]
fn dropped_table_falls_back_to_next_schema() {
    assert_snapshot!(goto("
create table public.t (a int);
create table app.t (a int);
set search_path = app, public;
drop table t;
select a$0 from t;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:24
      │
    2 │ create table public.t (a int);
      │                        ─ 2. destination
      ‡
    6 │ select a from t;
      ╰╴       ─ 1. source
    ");
}

#[test]
fn table_query_before_drop_and_recreate() {
    assert_snapshot!(goto("
create table t (a int);
select a$0 from t;
drop table t;
create table t (a int);
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create table t (a int);
      │                 ─ 2. destination
    3 │ select a from t;
      ╰╴       ─ 1. source
    ");
}

#[test]
fn drop_column() {
    assert_snapshot!(goto("
create table t (
  a int,
  b int
);
alter table t drop column b$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:4:3
      │
    4 │   b int
      │   ─ 2. destination
    5 │ );
    6 │ alter table t drop column b;
      ╰╴                          ─ 1. source
    ");
}

#[test]
fn alter_table_drop_column_table_name() {
    assert_snapshot!(goto("
create table t (
  a int,
  b int
);
alter table t$0 drop column b;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:14
      │
    2 │ create table t (
      │              ─ 2. destination
      ‡
    6 │ alter table t drop column b;
      ╰╴            ─ 1. source
    ");
}

#[test]
fn column_query_before_drop_column() {
    assert_snapshot!(goto("
create table t (
  a int,
  b int
);
select b$0 from t;
alter table t drop column b;
"), @"
    info: goto definition
      ╭▸ file0.sql:4:3
      │
    4 │   b int
      │   ─ 2. destination
    5 │ );
    6 │ select b from t;
      ╰╴       ─ 1. source
    ");
}

#[test]
fn star_after_drop_column() {
    assert_snapshot!(goto("
create table t (
  a int,
  b int,
  c int
);
alter table t drop column b;
select c$0 from (select * from t);
"), @"
    info: goto definition
      ╭▸ file0.sql:5:3
      │
    5 │   c int
      │   ─ 2. destination
      ‡
    8 │ select c from (select * from t);
      ╰╴       ─ 1. source
    ");
}

#[test]
fn dropped_column_hidden_from_star() {
    assert_snapshot!(goto("
create table t (
  a int,
  b int
);
alter table t drop column b;
select b$0 from (select * from t);
"), @"no definition");
}

#[test]
fn alter_table_renamed_ref() {
    assert_snapshot!(goto("
create table t (a int);
alter table t rename to t2;
select a from t2$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:25
      │
    3 │ alter table t rename to t2;
      │                         ── 2. destination
    4 │ select a from t2;
      ╰╴              ── 1. source
    ");
}

#[test]
fn alter_table_renamed_column_ref() {
    assert_snapshot!(goto("
create table t (a int);
alter table t rename to t2;
select a$0 from t2;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create table t (a int);
      │                 ─ 2. destination
    3 │ alter table t rename to t2;
    4 │ select a from t2;
      ╰╴       ─ 1. source
    ");
}

#[test]
fn alter_table_renamed_column() {
    assert_snapshot!(goto("
create table t (a int);
alter table t rename column a to b;
select b$0 from t;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:34
      │
    3 │ alter table t rename column a to b;
      │                                  ─ 2. destination
    4 │ select b from t;
      ╰╴       ─ 1. source
    ");
}

#[test]
fn alter_table_added_column() {
    assert_snapshot!(goto("
create table t (a int);
alter table t add column b int;
select b$0 from t;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:26
      │
    3 │ alter table t add column b int;
      │                          ─ 2. destination
    4 │ select b from t;
      ╰╴       ─ 1. source
    ");
}

#[test]
fn alter_table_alter_column_ref() {
    assert_snapshot!(goto("
create table t (a int);
alter table t alter column a$0 set default 1;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create table t (a int);
      │                 ─ 2. destination
    3 │ alter table t alter column a set default 1;
      ╰╴                           ─ 1. source
    ");
}

#[test]
fn table_source_before_rename() {
    assert_snapshot!(goto("
create table t (id int);
select t$0.id from t;
alter table t rename to u;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:14
      │
    2 │ create table t (id int);
      │              ─ 2. destination
    3 │ select t.id from t;
      ╰╴       ─ 1. source
    ");
}

#[test]
fn definition_site_after_rename() {
    assert_snapshot!(goto("
create table t$0 (id int);
alter table t rename to u;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:14
      │
    2 │ create table t (id int);
      │              ┬
      │              │
      │              1. source
      ╰╴             2. destination
    ");
}

#[test]
fn definition_site_after_rename_in_sql_body() {
    assert_snapshot!(goto("
create function f() returns void language sql as $$
create table t$0 (id int);
alter table t rename to u;
$$;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:14
      │
    3 │ create table t (id int);
      │              ┬
      │              │
      │              1. source
      ╰╴             2. destination
    ");
}

#[test]
fn table_renamed_twice() {
    assert_snapshot!(goto("
create table t (id int);
alter table t rename to u;
select * from u$0;
alter table u rename to v;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:25
      │
    3 │ alter table t rename to u;
      │                         ─ 2. destination
    4 │ select * from u;
      ╰╴              ─ 1. source
    ");
}

#[test]
fn reference_after_rolled_back_rename() {
    assert_snapshot!(goto("
create table t (id int);
begin;
alter table t rename to u;
rollback;
select * from t$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:14
      │
    2 │ create table t (id int);
      │              ─ 2. destination
      ‡
    6 │ select * from t;
      ╰╴              ─ 1. source
    ");
}
