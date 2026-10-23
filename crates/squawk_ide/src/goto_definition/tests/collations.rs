use insta::assert_snapshot;

use super::goto;

#[test]
fn collation_ref() {
    assert_snapshot!(goto("
create collation c (
  provider = icu,
  locale = 'und'
);
select 'a' collate c$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:18
      │
    2 │ create collation c (
      │                  ─ 2. destination
      ‡
    6 │ select 'a' collate c;
      ╰╴                   ─ 1. source
    ");
}

#[test]
fn collation_column_clause() {
    assert_snapshot!(goto("
create collation c (
  provider = icu,
  locale = 'und'
);
create table t (a text collate c$0);
"), @"
    info: goto definition
      ╭▸ file0.sql:2:18
      │
    2 │ create collation c (
      │                  ─ 2. destination
      ‡
    6 │ create table t (a text collate c);
      ╰╴                               ─ 1. source
    ");
}

#[test]
fn collation_drop() {
    assert_snapshot!(goto("
create collation s.c (
  provider = icu,
  locale = 'und'
);
drop collation s.c$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:20
      │
    2 │ create collation s.c (
      │                    ─ 2. destination
      ‡
    6 │ drop collation s.c;
      ╰╴                 ─ 1. source
    ");
}

#[test]
fn collation_builtin() {
    assert_snapshot!(goto(r#"
select 'a' collate "C"$0;
"#), @r#"
    info: goto definition
          ╭▸ file0.sql:2:20
          │
        2 │ select 'a' collate "C";
          │                    ─── 1. source
          │
          ⸬  builtins.sql:38143:29
          │
    38143 │ create collation pg_catalog."C" (
          ╰╴                            ─── 2. destination
    "#);
}

#[test]
fn collation_builtin_default() {
    assert_snapshot!(goto(r#"
select 'a' collate "default"$0;
"#), @r#"
    info: goto definition
          ╭▸ file0.sql:2:20
          │
        2 │ select 'a' collate "default";
          │                    ───────── 1. source
          │
          ⸬  builtins.sql:38155:29
          │
    38155 │ create collation pg_catalog.default (
          ╰╴                            ─────── 2. destination
    "#);
}

#[test]
fn alter_collation_renamed_ref() {
    assert_snapshot!(goto("
create collation c (
  locale = 'C'
);
alter collation c rename to c2;
create table t (a text collate c2$0);
"), @"
    info: goto definition
      ╭▸ file0.sql:5:29
      │
    5 │ alter collation c rename to c2;
      │                             ── 2. destination
    6 │ create table t (a text collate c2);
      ╰╴                               ── 1. source
    ");
}

#[test]
fn collate_in_order_by() {
    assert_snapshot!(goto(r#"
create collation c from "C";
create table t (a text);
select a from t order by a collate c$0;
"#), @r#"
    info: goto definition
      ╭▸ file0.sql:2:18
      │
    2 │ create collation c from "C";
      │                  ─ 2. destination
    3 │ create table t (a text);
    4 │ select a from t order by a collate c;
      ╰╴                                   ─ 1. source
    "#);
}

#[test]
fn collate_in_index_expr() {
    assert_snapshot!(goto(r#"
create collation c from "C";
create table t (a text);
create index idx on t (a collate c$0);
"#), @r#"
    info: goto definition
      ╭▸ file0.sql:2:18
      │
    2 │ create collation c from "C";
      │                  ─ 2. destination
    3 │ create table t (a text);
    4 │ create index idx on t (a collate c);
      ╰╴                                 ─ 1. source
    "#);
}

#[test]
fn collation_after_schema_rename() {
    assert_snapshot!(goto("
create schema s;
create collation s.c (
  locale = 'C'
);
alter schema s rename to t;
select 'a' collate t.c$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:20
      │
    3 │ create collation s.c (
      │                    ─ 2. destination
      ‡
    7 │ select 'a' collate t.c;
      ╰╴                     ─ 1. source
    ");
}
