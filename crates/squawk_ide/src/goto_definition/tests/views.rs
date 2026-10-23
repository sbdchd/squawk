use insta::assert_snapshot;

use super::{goto, goto_files};

#[test]
fn create_view() {
    assert_snapshot!(goto("
create view v as
  select 1 a;
select a from v$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:13
      │
    2 │ create view v as
      │             ─ 2. destination
    3 │   select 1 a;
    4 │ select a from v;
      ╰╴              ─ 1. source
    ");
}

#[test]
fn create_view_column() {
    assert_snapshot!(goto("
create view v as
  select 1 a;
select v.a$0 from v;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:12
      │
    3 │   select 1 a;
      │            ─ 2. destination
    4 │ select v.a from v;
      ╰╴         ─ 1. source
    ");
}

#[test]
fn create_view_column_list() {
    assert_snapshot!(goto("
create view public.v (x) as
  select 1 a;
select x$0 from v;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:23
      │
    2 │ create view public.v (x) as
      │                       ─ 2. destination
    3 │   select 1 a;
    4 │ select x from v;
      ╰╴       ─ 1. source
    ");
}

#[test]
fn view_in_another_file() {
    assert_snapshot!(goto_files(&["
create table users (
  id int,
  name text
);
", "
create view v as
  select * from users;
", "
select name$0 from v;
"]), @"
    info: goto definition
      ╭▸ file2.sql:2:8
      │
    2 │ select name from v;
      │        ──── 1. source
      │
      ⸬  file0.sql:4:3
      │
    4 │   name text
      ╰╴  ──── 2. destination
    ");
}

#[test]
fn drop_view() {
    assert_snapshot!(goto("
create view v as
  select 1 a;
drop view v$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:13
      │
    2 │ create view v as
      │             ─ 2. destination
    3 │   select 1 a;
    4 │ drop view v;
      ╰╴          ─ 1. source
    ");
}

#[test]
fn alter_view_ref() {
    assert_snapshot!(goto("
create view v as
  select 1 a;
alter view v$0 rename to w;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:13
      │
    2 │ create view v as
      │             ─ 2. destination
    3 │   select 1 a;
    4 │ alter view v rename to w;
      ╰╴           ─ 1. source
    ");
}

#[test]
fn alter_view_renamed_ref() {
    assert_snapshot!(goto("
create view v as
  select 1 a;
alter view v rename to w;
select a from w$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:4:24
      │
    4 │ alter view v rename to w;
      │                        ─ 2. destination
    5 │ select a from w;
      ╰╴              ─ 1. source
    ");
}

#[test]
fn alter_view_renamed_column_ref() {
    assert_snapshot!(goto("
create view v as
  select 1 a;
alter view v rename column a to b;
select b$0 from v;
"), @"
    info: goto definition
      ╭▸ file0.sql:4:33
      │
    4 │ alter view v rename column a to b;
      │                                 ─ 2. destination
    5 │ select b from v;
      ╰╴       ─ 1. source
    ");
}

#[test]
fn alter_view_column_ref() {
    assert_snapshot!(goto("
create view v as
  select 1 a;
alter view v alter column a$0 set default 2;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:12
      │
    3 │   select 1 a;
      │            ─ 2. destination
    4 │ alter view v alter column a set default 2;
      ╰╴                          ─ 1. source
    ");
}

#[test]
fn alter_materialized_view_renamed_ref() {
    assert_snapshot!(goto("
create materialized view v as
  select 1 a;
alter materialized view v rename to w;
select a from w$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:4:37
      │
    4 │ alter materialized view v rename to w;
      │                                     ─ 2. destination
    5 │ select a from w;
      ╰╴              ─ 1. source
    ");
}

#[test]
fn alter_materialized_view_renamed_column_ref() {
    assert_snapshot!(goto("
create materialized view v as
  select 1 a;
alter materialized view v rename column a to b;
select b$0 from v;
"), @"
    info: goto definition
      ╭▸ file0.sql:4:46
      │
    4 │ alter materialized view v rename column a to b;
      │                                              ─ 2. destination
    5 │ select b from v;
      ╰╴       ─ 1. source
    ");
}

#[test]
fn alter_materialized_view_column_ref() {
    assert_snapshot!(goto("
create materialized view v as
  select 1 a;
alter materialized view v alter column a$0 set statistics 100;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:12
      │
    3 │   select 1 a;
      │            ─ 2. destination
    4 │ alter materialized view v alter column a set statistics 100;
      ╰╴                                       ─ 1. source
    ");
}

#[test]
fn lock_view_ref() {
    assert_snapshot!(goto("
create view v as
  select 1 a;
lock v$0 in share mode;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:13
      │
    2 │ create view v as
      │             ─ 2. destination
    3 │   select 1 a;
    4 │ lock v in share mode;
      ╰╴     ─ 1. source
    ");
}

#[test]
fn drop_table_on_view_ref() {
    assert_snapshot!(goto("
create view v as
  select 1 a;
drop table v$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:13
      │
    2 │ create view v as
      │             ─ 2. destination
    3 │   select 1 a;
    4 │ drop table v;
      ╰╴           ─ 1. source
    ");
}

#[test]
fn refresh_materialized_view() {
    assert_snapshot!(goto("
create materialized view mv as
  select 1;
refresh materialized view mv$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:26
      │
    2 │ create materialized view mv as
      │                          ── 2. destination
    3 │   select 1;
    4 │ refresh materialized view mv;
      ╰╴                          ── 1. source
    ");
}
