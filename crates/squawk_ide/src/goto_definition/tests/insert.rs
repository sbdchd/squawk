use insta::assert_snapshot;

use super::goto;

#[test]
fn insert_table() {
    assert_snapshot!(goto("
create table t (a int);
insert into t$0 values (1);
"), @"
    info: goto definition
      ╭▸ file0.sql:2:14
      │
    2 │ create table t (a int);
      │              ─ 2. destination
    3 │ insert into t values (1);
      ╰╴            ─ 1. source
    ");
}

#[test]
fn insert_column_list() {
    assert_snapshot!(goto("
create table t (
  a int,
  b int
);
insert into t (b$0) values (1);
"), @"
    info: goto definition
      ╭▸ file0.sql:4:3
      │
    4 │   b int
      │   ─ 2. destination
    5 │ );
    6 │ insert into t (b) values (1);
      ╰╴               ─ 1. source
    ");
}

#[test]
fn insert_alias() {
    assert_snapshot!(goto("
create table t (a int);
insert into t as x values (1) returning x$0.a;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:18
      │
    3 │ insert into t as x values (1) returning x.a;
      ╰╴                 ─ 2. destination       ─ 1. source
    ");
}

#[test]
fn insert_returning_column() {
    assert_snapshot!(goto("
create table t (a int);
insert into t values (1) returning a$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create table t (a int);
      │                 ─ 2. destination
    3 │ insert into t values (1) returning a;
      ╰╴                                   ─ 1. source
    ");
}

#[test]
fn insert_select_column() {
    assert_snapshot!(goto("
create table t (a int);
create table u (c int);
insert into t select c$0 from u;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:17
      │
    3 │ create table u (c int);
      │                 ─ 2. destination
    4 │ insert into t select c from u;
      ╰╴                     ─ 1. source
    ");
}

#[test]
fn insert_on_conflict_target_column() {
    assert_snapshot!(goto("
create table t (a int);
insert into t values (1) on conflict (a$0) do nothing;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create table t (a int);
      │                 ─ 2. destination
    3 │ insert into t values (1) on conflict (a) do nothing;
      ╰╴                                      ─ 1. source
    ");
}

#[test]
fn insert_on_conflict_set_column() {
    assert_snapshot!(goto("
create table t (
  a int,
  b int
);
insert into t values (1, 2) on conflict (a) do update set b$0 = 1;
"), @"
    info: goto definition
      ╭▸ file0.sql:4:3
      │
    4 │   b int
      │   ─ 2. destination
    5 │ );
    6 │ insert into t values (1, 2) on conflict (a) do update set b = 1;
      ╰╴                                                          ─ 1. source
    ");
}

#[test]
fn insert_excluded_qualifier() {
    assert_snapshot!(goto("
create table t (
  a int,
  b int
);
insert into t values (1, 2) on conflict (a) do update set b = excluded$0.b;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:14
      │
    2 │ create table t (
      │              ─ 2. destination
      ‡
    6 │ insert into t values (1, 2) on conflict (a) do update set b = excluded.b;
      ╰╴                                                              ──────── 1. source
    ");
}

#[test]
fn insert_excluded_column() {
    assert_snapshot!(goto("
create table t (
  a int,
  b int
);
insert into t values (1, 2) on conflict (a) do update set b = excluded.b$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:4:3
      │
    4 │   b int
      │   ─ 2. destination
    5 │ );
    6 │ insert into t values (1, 2) on conflict (a) do update set b = excluded.b;
      ╰╴                                                                       ─ 1. source
    ");
}

#[test]
fn insert_cte_returning_column() {
    assert_snapshot!(goto("
create table t (a int);
with i as (
  insert into t values (1) returning a
)
select a$0 from i;
"), @"
    info: goto definition
      ╭▸ file0.sql:4:38
      │
    4 │   insert into t values (1) returning a
      │                                      ─ 2. destination
    5 │ )
    6 │ select a from i;
      ╰╴       ─ 1. source
    ");
}

#[test]
fn on_conflict_do_select_for_update_of() {
    assert_snapshot!(goto("
create table t (a int primary key);
insert into t as x values (1) on conflict (a) do select for update of x$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:18
      │
    3 │ insert into t as x values (1) on conflict (a) do select for update of x;
      ╰╴                 ─ 2. destination                                     ─ 1. source
    ");
}

#[test]
fn insert_composite_field() {
    assert_snapshot!(goto("
create type pair as (
  x int,
  y int
);
create table t (p pair);
insert into t (p.x$0) values (1);
"), @"
    info: goto definition
      ╭▸ file0.sql:3:3
      │
    3 │   x int,
      │   ─ 2. destination
      ‡
    7 │ insert into t (p.x) values (1);
      ╰╴                 ─ 1. source
    ");
}

#[test]
fn insert_returning_old_column() {
    assert_snapshot!(goto("
create table t (
  a int,
  b int
);
insert into t values (1, 2), (3, 4) returning old.a$0, new.b;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:3
      │
    3 │   a int,
      │   ─ 2. destination
      ‡
    6 │ insert into t values (1, 2), (3, 4) returning old.a, new.b;
      ╰╴                                                  ─ 1. source
    ");
}

#[test]
fn insert_on_conflict_excluded_whole_row() {
    assert_snapshot!(goto("
create table t (
  a int primary key,
  b jsonb
);
insert into t
values (1, '{}')
on conflict (a) do update set b = to_jsonb(excluded$0);
"), @"
    info: goto definition
      ╭▸ file0.sql:2:14
      │
    2 │ create table t (
      │              ─ 2. destination
      ‡
    8 │ on conflict (a) do update set b = to_jsonb(excluded);
      ╰╴                                           ──────── 1. source
    ");
}

#[test]
fn insert_on_conflict_set_subquery_excluded_column() {
    assert_snapshot!(goto("
create table t (
  a int primary key,
  b int
);
insert into t
values (1, 2)
on conflict (a) do update set b = (select excluded.b$0);
"), @"
    info: goto definition
      ╭▸ file0.sql:4:3
      │
    4 │   b int
      │   ─ 2. destination
      ‡
    8 │ on conflict (a) do update set b = (select excluded.b);
      ╰╴                                                   ─ 1. source
    ");
}
