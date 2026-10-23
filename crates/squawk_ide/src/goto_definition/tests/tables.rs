use insta::assert_snapshot;

use super::{goto, goto_files};

#[test]
fn create_table() {
    assert_snapshot!(goto("
create table users (
  id int,
  name text
);
select name from users$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:14
      │
    2 │ create table users (
      │              ───── 2. destination
      ‡
    6 │ select name from users;
      ╰╴                 ───── 1. source
    ");
}

#[test]
fn create_table_column() {
    assert_snapshot!(goto("
create table public.users (
  id int,
  name text
);
select u.name$0 from users u;
"), @"
    info: goto definition
      ╭▸ file0.sql:4:3
      │
    4 │   name text
      │   ──── 2. destination
    5 │ );
    6 │ select u.name from users u;
      ╰╴         ──── 1. source
    ");
}

#[test]
fn external_table() {
    assert_snapshot!(goto("
select a from foo$0;
"), @"no definition");
}

#[test]
fn table_in_another_file() {
    assert_snapshot!(goto_files(&["
create table users (
  id int,
  name text
);
", "
select u.name$0 from users u;
"]), @"
    info: goto definition
      ╭▸ file1.sql:2:10
      │
    2 │ select u.name from users u;
      │          ──── 1. source
      │
      ⸬  file0.sql:4:3
      │
    4 │   name text
      ╰╴  ──── 2. destination
    ");
}

#[test]
fn table_in_another_file_ref() {
    assert_snapshot!(goto_files(&["
create table users (
  id int,
  name text
);
", "
select name from users$0;
"]), @"
    info: goto definition
      ╭▸ file1.sql:2:18
      │
    2 │ select name from users;
      │                  ───── 1. source
      │
      ⸬  file0.sql:2:14
      │
    2 │ create table users (
      ╰╴             ───── 2. destination
    ");
}

#[test]
fn builtin_table_column() {
    assert_snapshot!(goto("
select relname$0 from pg_class;
"), @"
    info: goto definition
        ╭▸ file0.sql:2:8
        │
      2 │ select relname from pg_class;
        │        ─────── 1. source
        │
        ⸬  builtins.sql:666:3
        │
    666 │   relname name not null,
        ╰╴  ─────── 2. destination
    ");
}

#[test]
fn temp_table_per_file() {
    assert_snapshot!(goto_files(&[
        "
create table t (a int); create temp table t (b int);
",
        "
select a from t$0;
",
    ]), @"
    info: goto definition
      ╭▸ file1.sql:2:15
      │
    2 │ select a from t;
      │               ─ 1. source
      │
      ⸬  file0.sql:2:14
      │
    2 │ create table t (a int); create temp table t (b int);
      ╰╴             ─ 2. destination
    ");
}

#[test]
fn temp_table_numbered_schema() {
    assert_snapshot!(goto("
create temp table t (a int);
select a from pg_temp_12.t$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:19
      │
    2 │ create temp table t (a int);
      │                   ─ 2. destination
    3 │ select a from pg_temp_12.t;
      ╰╴                         ─ 1. source
    ");
}

#[test]
fn generated_column_reference() {
    assert_snapshot!(goto("
create table t (
  a int,
  b int generated always as (a$0 * 2) stored
);
"), @"
    info: goto definition
      ╭▸ file0.sql:3:3
      │
    3 │   a int,
      │   ─ 2. destination
    4 │   b int generated always as (a * 2) stored
      ╰╴                             ─ 1. source
    ");
}

#[test]
fn long_table_name_truncated() {
    assert_snapshot!(goto("
create table aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa_one (
  x int
);
select *
from aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa_two$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:14
      │
    2 │ create table aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa_one (
      │              ─────────────────────────────────────────────────────────────────── 2. destination
      ‡
    6 │ from aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa_two;
      ╰╴     ─────────────────────────────────────────────────────────────────── 1. source
    ");
}

#[test]
fn schema_qualified_table() {
    assert_snapshot!(goto("
create schema s;
create table s.t (a int);
select a from s$0.t;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:15
      │
    2 │ create schema s;
      │               ─ 2. destination
    3 │ create table s.t (a int);
    4 │ select a from s.t;
      ╰╴              ─ 1. source
    ");
}

#[test]
fn analyze_column_ref() {
    assert_snapshot!(goto("
create table t (
  a int,
  b int
);
analyze t (b$0);
"), @"
    info: goto definition
      ╭▸ file0.sql:4:3
      │
    4 │   b int
      │   ─ 2. destination
    5 │ );
    6 │ analyze t (b);
      ╰╴           ─ 1. source
    ");
}

#[test]
fn vacuum_table_ref() {
    assert_snapshot!(goto("
create table t (a int);
vacuum (analyze) t$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:14
      │
    2 │ create table t (a int);
      │              ─ 2. destination
    3 │ vacuum (analyze) t;
      ╰╴                 ─ 1. source
    ");
}

#[test]
fn truncate_table_ref() {
    assert_snapshot!(goto("
create table t (a int);
truncate table only t$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:14
      │
    2 │ create table t (a int);
      │              ─ 2. destination
    3 │ truncate table only t;
      ╰╴                    ─ 1. source
    ");
}

#[test]
fn comment_on_column_ref() {
    assert_snapshot!(goto("
create table t (a int);
comment on column t.a$0 is 'x';
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create table t (a int);
      │                 ─ 2. destination
    3 │ comment on column t.a is 'x';
      ╰╴                    ─ 1. source
    ");
}

#[test]
fn grant_table_ref() {
    assert_snapshot!(goto("
create table t (a int);
grant select on t$0 to public;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:14
      │
    2 │ create table t (a int);
      │              ─ 2. destination
    3 │ grant select on t to public;
      ╰╴                ─ 1. source
    ");
}

#[test]
fn grant_column_ref() {
    assert_snapshot!(goto("
create table t (
  a int,
  b int
);
grant select (a, b$0) on t to public;
"), @"
    info: goto definition
      ╭▸ file0.sql:4:3
      │
    4 │   b int
      │   ─ 2. destination
    5 │ );
    6 │ grant select (a, b) on t to public;
      ╰╴                 ─ 1. source
    ");
}

#[test]
fn security_label_table_ref() {
    assert_snapshot!(goto("
create table t (a int);
security label for selinux on table t$0 is 'x';
"), @"
    info: goto definition
      ╭▸ file0.sql:2:14
      │
    2 │ create table t (a int);
      │              ─ 2. destination
    3 │ security label for selinux on table t is 'x';
      ╰╴                                    ─ 1. source
    ");
}

#[test]
fn typed_table_column_ref() {
    assert_snapshot!(goto("
create type ty as (
  x int
);
create table t
  of ty;
select x$0 from t;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:3
      │
    3 │   x int
      │   ─ 2. destination
      ‡
    7 │ select x from t;
      ╰╴       ─ 1. source
    ");
}

#[test]
fn drop_temp_table_shadows_public() {
    assert_snapshot!(goto("
create table t ();
create temp table t ();
drop table t$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:19
      │
    3 │ create temp table t ();
      │                   ─ 2. destination
    4 │ drop table t;
      ╰╴           ─ 1. source
    ");
}

#[test]
fn drop_public_table_when_temp_exists() {
    assert_snapshot!(goto("
create table t ();
create temp table t ();
drop table public.t$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:14
      │
    2 │ create table t ();
      │              ─ 2. destination
    3 │ create temp table t ();
    4 │ drop table public.t;
      ╰╴                  ─ 1. source
    ");
}
