use insta::assert_snapshot;

use super::goto;

#[test]
fn database_alter() {
    assert_snapshot!(goto("
create database d;
alter database d$0 set tablespace pg_default;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create database d;
      │                 ─ 2. destination
    3 │ alter database d set tablespace pg_default;
      ╰╴               ─ 1. source
    ");
}

#[test]
fn database_template() {
    assert_snapshot!(goto("
create database d;
create database e template d$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create database d;
      │                 ─ 2. destination
    3 │ create database e template d;
      ╰╴                           ─ 1. source
    ");
}

#[test]
fn database_quoted_drop() {
    assert_snapshot!(goto(r#"
create database "D";
drop database "D"$0;
"#), @r#"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create database "D";
      │                 ─── 2. destination
    3 │ drop database "D";
      ╰╴              ─── 1. source
    "#);
}

#[test]
fn database_qualified_column() {
    assert_snapshot!(goto("
create database d;
create table t (a int);
select d$0.public.t.a from t;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create database d;
      │                 ─ 2. destination
    3 │ create table t (a int);
    4 │ select d.public.t.a from t;
      ╰╴       ─ 1. source
    ");
}

#[test]
fn database_builtin() {
    assert_snapshot!(goto("
reindex database postgres$0;
"), @"
    info: goto definition
       ╭▸ file0.sql:2:18
       │
     2 │ reindex database postgres;
       │                  ──────── 1. source
       │
       ⸬  builtins.sql:47:17
       │
    47 │ create database postgres;
       ╰╴                ──────── 2. destination
    ");
}

#[test]
fn database_renamed() {
    assert_snapshot!(goto("
create database db;
alter database db rename to db2;
drop database db2$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:29
      │
    3 │ alter database db rename to db2;
      │                             ─── 2. destination
    4 │ drop database db2;
      ╰╴              ─── 1. source
    ");
}
