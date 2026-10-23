use insta::assert_snapshot;

use super::{goto, goto_files};

#[test]
fn index_definition_returns_self() {
    assert_snapshot!(goto("
create table t (x int);
create index idx$0 on t (x);
"), @"
    info: goto definition
      ╭▸ file0.sql:3:14
      │
    3 │ create index idx on t (x);
      │              ┬──
      │              │
      │              1. source
      ╰╴             2. destination
    ");
}

#[test]
fn create_index_table() {
    assert_snapshot!(goto("
create table t (x int);
create index idx on t$0 (x);
"), @"
    info: goto definition
      ╭▸ file0.sql:2:14
      │
    2 │ create table t (x int);
      │              ─ 2. destination
    3 │ create index idx on t (x);
      ╰╴                    ─ 1. source
    ");
}

#[test]
fn create_index_table_with_schema() {
    assert_snapshot!(goto("
create table s.t (x int);
create table t (x int);
create index idx on s.t$0 (x);
"), @"
    info: goto definition
      ╭▸ file0.sql:2:16
      │
    2 │ create table s.t (x int);
      │                ─ 2. destination
    3 │ create table t (x int);
    4 │ create index idx on s.t (x);
      ╰╴                      ─ 1. source
    ");
}

#[test]
fn create_index_temp_table() {
    assert_snapshot!(goto("
create table t (x int);
create temp table t (x int);
create index idx on t$0 (x);
"), @"
    info: goto definition
      ╭▸ file0.sql:3:19
      │
    3 │ create temp table t (x int);
      │                   ─ 2. destination
    4 │ create index idx on t (x);
      ╰╴                    ─ 1. source
    ");
}

#[test]
fn create_index_column() {
    assert_snapshot!(goto("
create table t (
  a int,
  b int
);
create index idx on t (a, b$0);
"), @"
    info: goto definition
      ╭▸ file0.sql:4:3
      │
    4 │   b int
      │   ─ 2. destination
    5 │ );
    6 │ create index idx on t (a, b);
      ╰╴                          ─ 1. source
    ");
}

#[test]
fn create_index_expr_function_call() {
    assert_snapshot!(goto("
create table articles (title text);
create index on articles (lower$0(title));
"), @"
    info: goto definition
          ╭▸ file0.sql:3:27
          │
        3 │ create index on articles (lower(title));
          │                           ───── 1. source
          │
          ⸬  builtins.sql:17607:28
          │
    17607 │ create function pg_catalog.lower(text) returns text
          ╰╴                           ───── 2. destination
    ");
}

#[test]
fn create_index_expr_column() {
    assert_snapshot!(goto("
create table articles (title text);
create index on articles (lower(title$0));
"), @"
    info: goto definition
      ╭▸ file0.sql:2:24
      │
    2 │ create table articles (title text);
      │                        ───── 2. destination
    3 │ create index on articles (lower(title));
      ╰╴                                ───── 1. source
    ");
}

#[test]
fn create_index_include_column() {
    assert_snapshot!(goto("
create table users (
  id int,
  email text
);
create index idx on users (id) include (email$0);
"), @"
    info: goto definition
      ╭▸ file0.sql:4:3
      │
    4 │   email text
      │   ───── 2. destination
    5 │ );
    6 │ create index idx on users (id) include (email);
      ╰╴                                        ───── 1. source
    ");
}

#[test]
fn create_index_where_column() {
    assert_snapshot!(goto("
create table users (
  id int,
  email text
);
create index idx on users (id) where email$0 is not null;
"), @"
    info: goto definition
      ╭▸ file0.sql:4:3
      │
    4 │   email text
      │   ───── 2. destination
    5 │ );
    6 │ create index idx on users (id) where email is not null;
      ╰╴                                     ───── 1. source
    ");
}

#[test]
fn drop_index() {
    assert_snapshot!(goto("
create table t (x int);
create index idx on t (x);
drop index idx$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:14
      │
    3 │ create index idx on t (x);
      │              ─── 2. destination
    4 │ drop index idx;
      ╰╴           ─── 1. source
    ");
}

#[test]
fn drop_index_multiple() {
    assert_snapshot!(goto("
create table t (
  x int,
  y int
);
create index idx_x on t (x);
create index idx_y on t (y);
drop index idx_x, idx_y$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:7:14
      │
    7 │ create index idx_y on t (y);
      │              ───── 2. destination
    8 │ drop index idx_x, idx_y;
      ╰╴                  ───── 1. source
    ");
}

#[test]
fn drop_index_with_schema() {
    assert_snapshot!(goto("
create table s.t (x int);
create table t (x int);
create index idx on s.t (x);
create index idx on t (x);
drop index s.idx$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:4:14
      │
    4 │ create index idx on s.t (x);
      │              ─── 2. destination
    5 │ create index idx on t (x);
    6 │ drop index s.idx;
      ╰╴             ─── 1. source
    ");
}

#[test]
fn drop_index_with_search_path() {
    assert_snapshot!(goto("
create table s.t (x int);
create index idx on s.t (x);
set search_path = s;
drop index idx$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:14
      │
    3 │ create index idx on s.t (x);
      │              ─── 2. destination
    4 │ set search_path = s;
    5 │ drop index idx;
      ╰╴           ─── 1. source
    ");
}

#[test]
fn drop_index_defined_after() {
    assert_snapshot!(goto("
drop index if exists idx$0;
create table t (x int);
create index idx on t (x);
"), @"no definition");
}

#[test]
fn index_dropped_with_table() {
    assert_snapshot!(goto("
create table t (x int);
create index idx on t (x);
drop table t;
drop index if exists idx$0;
"), @"no definition");
}

#[test]
fn create_index_if_not_exists_keeps_first() {
    assert_snapshot!(goto("
create table t (x int);
create index idx on t (x);
create index if not exists idx on t (x);
drop index idx$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:14
      │
    3 │ create index idx on t (x);
      │              ─── 2. destination
    4 │ create index if not exists idx on t (x);
    5 │ drop index idx;
      ╰╴           ─── 1. source
    ");
}

#[test]
fn alter_index() {
    assert_snapshot!(goto("
create table t (id int);
create index idx on t (id);
alter index idx$0 set (fillfactor = 70);
"), @"
    info: goto definition
      ╭▸ file0.sql:3:14
      │
    3 │ create index idx on t (id);
      │              ─── 2. destination
    4 │ alter index idx set (fillfactor = 70);
      ╰╴            ─── 1. source
    ");
}

#[test]
fn alter_index_attach_partition() {
    assert_snapshot!(goto("
create table t (inserted_at timestamptz) partition by range (inserted_at);
create table part
  partition of t
  for values from ('2026-01-02') to ('2026-01-03');
create index parent_idx on t (inserted_at);
create index child_idx on part (inserted_at);
alter index parent_idx attach partition child_$0idx;
"), @"
    info: goto definition
      ╭▸ file0.sql:7:14
      │
    7 │ create index child_idx on part (inserted_at);
      │              ───────── 2. destination
    8 │ alter index parent_idx attach partition child_idx;
      ╰╴                                        ───────── 1. source
    ");
}

#[test]
fn reindex_index() {
    assert_snapshot!(goto("
create table t (c int);
create index idx on t (c);
reindex index idx$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:14
      │
    3 │ create index idx on t (c);
      │              ─── 2. destination
    4 │ reindex index idx;
      ╰╴              ─── 1. source
    ");
}

#[test]
fn reindex_table() {
    assert_snapshot!(goto("
create table t (c int);
reindex table t$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:14
      │
    2 │ create table t (c int);
      │              ─ 2. destination
    3 │ reindex table t;
      ╰╴              ─ 1. source
    ");
}

#[test]
fn cluster_using_index() {
    assert_snapshot!(goto("
create table foo (id int);
create index i on foo (id);
cluster foo using i$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:14
      │
    3 │ create index i on foo (id);
      │              ─ 2. destination
    4 │ cluster foo using i;
      ╰╴                  ─ 1. source
    ");
}

#[test]
fn cluster_legacy_index() {
    assert_snapshot!(goto("
create table foo (id int);
create index i on foo (id);
cluster i$0 on foo;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:14
      │
    3 │ create index i on foo (id);
      │              ─ 2. destination
    4 │ cluster i on foo;
      ╰╴        ─ 1. source
    ");
}

#[test]
fn alter_table_cluster_on() {
    assert_snapshot!(goto("
create table foo (id int);
create index i on foo (id);
alter table foo cluster on i$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:14
      │
    3 │ create index i on foo (id);
      │              ─ 2. destination
    4 │ alter table foo cluster on i;
      ╰╴                           ─ 1. source
    ");
}

#[test]
fn alter_table_replica_identity_using_index() {
    assert_snapshot!(goto("
create table t (a int);
create unique index idx on t (a);
alter table t replica identity using index idx$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:21
      │
    3 │ create unique index idx on t (a);
      │                     ─── 2. destination
    4 │ alter table t replica identity using index idx;
      ╰╴                                           ─── 1. source
    ");
}

#[test]
fn temp_index_not_visible_in_other_file() {
    assert_snapshot!(goto_files(&["
create temp table t (x int);
create index idx on t (x);
", "
drop index if exists idx$0;
"]), @"no definition");
}

#[test]
fn reindex_primary_key_index() {
    assert_snapshot!(goto("
create table t (a int primary key);
reindex index t_pkey$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:23
      │
    2 │ create table t (a int primary key);
      │                       ─────────── 2. destination
    3 │ reindex index t_pkey;
      ╰╴              ────── 1. source
    ");
}

#[test]
fn cluster_on_named_unique_constraint_index() {
    assert_snapshot!(goto("
create table t (a int constraint t_a_uniq unique);
alter table t cluster on t_a_uniq$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:34
      │
    2 │ create table t (a int constraint t_a_uniq unique);
      │                                  ──────── 2. destination
    3 │ alter table t cluster on t_a_uniq;
      ╰╴                         ──────── 1. source
    ");
}

#[test]
fn alter_materialized_view_cluster_on_ref() {
    assert_snapshot!(goto("
create materialized view v as
  select 1 a;
create index i on v (a);
alter materialized view v cluster on i$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:4:14
      │
    4 │ create index i on v (a);
      │              ─ 2. destination
    5 │ alter materialized view v cluster on i;
      ╰╴                                     ─ 1. source
    ");
}

#[test]
fn repack_index_ref() {
    assert_snapshot!(goto("
create table t (a int);
create index i on t (a);
repack t using index i$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:14
      │
    3 │ create index i on t (a);
      │              ─ 2. destination
    4 │ repack t using index i;
      ╰╴                     ─ 1. source
    ");
}

#[test]
fn alter_index_renamed_ref() {
    assert_snapshot!(goto("
create table t (a int);
create index i on t (a);
alter index i rename to i2;
drop index i2$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:4:25
      │
    4 │ alter index i rename to i2;
      │                         ── 2. destination
    5 │ drop index i2;
      ╰╴           ── 1. source
    ");
}

#[test]
fn reference_before_index_rename() {
    assert_snapshot!(goto("
create table t (id int);
create index idx on t (id);
comment on index idx$0 is 'x';
alter index idx rename to idx2;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:14
      │
    3 │ create index idx on t (id);
      │              ─── 2. destination
    4 │ comment on index idx is 'x';
      ╰╴                 ─── 1. source
    ");
}

#[test]
fn reference_after_index_rename() {
    assert_snapshot!(goto("
create table t (id int);
create index idx on t (id);
alter index idx rename to idx2;
comment on index idx2$0 is 'x';
"), @"
    info: goto definition
      ╭▸ file0.sql:4:27
      │
    4 │ alter index idx rename to idx2;
      │                           ──── 2. destination
    5 │ comment on index idx2 is 'x';
      ╰╴                 ──── 1. source
    ");
}
