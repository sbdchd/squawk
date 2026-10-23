use insta::assert_snapshot;

use super::check;

#[test]
fn create_index_unknown_table() {
    assert_snapshot!(check("
create index idx on t(x);
"), @"
    error: relation `t` does not exist
      ╭▸ 
    2 │ create index idx on t(x);
      ╰╴                    ━
    ");
}

#[test]
fn create_index_unknown_column() {
    assert_snapshot!(check("
create table t(a int);
create index idx on t(a, b) include (c) where d > 0;
"), @"
    error: column `b` does not exist
      ╭▸ 
    3 │ create index idx on t(a, b) include (c) where d > 0;
      ╰╴                         ━
    error: column `c` does not exist
      ╭▸ 
    3 │ create index idx on t(a, b) include (c) where d > 0;
      ╰╴                                     ━
    error: column `d` does not exist
      ╭▸ 
    3 │ create index idx on t(a, b) include (c) where d > 0;
      ╰╴                                              ━
    ");
}

#[test]
fn drop_index_unknown() {
    assert_snapshot!(check("
drop index idx;
drop index if exists idx2;
"), @"
    error: index `idx` does not exist
      ╭▸ 
    2 │ drop index idx;
      ╰╴           ━━━
    ");
}

#[test]
fn drop_index_twice() {
    assert_snapshot!(check("
create table t(a int);
create index idx on t(a);
drop index idx;
drop index idx;
"), @"
    error: index `idx` does not exist
      ╭▸ 
    5 │ drop index idx;
      ╰╴           ━━━
    ");
}

#[test]
fn cluster_using_temp_index() {
    assert_snapshot!(check("
create temp table t(a int);
create index idx on t(a);
create table u(a int);
cluster u using idx;
"), @"no diagnostics");
}

#[test]
fn rollback_create_index() {
    assert_snapshot!(check("
create table t (a int);
begin;
create index i on t (a);
rollback;
drop index i;
"), @"
    error: index `i` does not exist
      ╭▸ 
    6 │ drop index i;
      ╰╴           ━
    ");
}

#[test]
fn duplicate_index() {
    assert_snapshot!(check("
create table t (a int);
create index i on t (a);
create index i on t (a);
create index if not exists i on t (a);
create index t on t (a);
create index on t (a);
create index on t (a);
"), @"
    error: relation `i` already exists
      ╭▸ 
    4 │ create index i on t (a);
      ╰╴             ━
    error: relation `t` already exists
      ╭▸ 
    6 │ create index t on t (a);
      ╰╴             ━
    ");
}

#[test]
fn index_item_collate_reported_once() {
    assert_snapshot!(check("
create table t (b text);
create index on t (b collate missing1);
insert into t values ('x') on conflict (b collate missing2) do nothing;
"), @"
    error: collation `missing1` does not exist
      ╭▸ 
    3 │ create index on t (b collate missing1);
      ╰╴                             ━━━━━━━━
    error: collation `missing2` does not exist
      ╭▸ 
    4 │ insert into t values ('x') on conflict (b collate missing2) do nothing;
      ╰╴                                                  ━━━━━━━━
    ");
}

#[test]
fn repack() {
    assert_snapshot!(check("
create table t (a int);
create index i on t (a);
create view v as select 1 a;
repack t;
repack t using index i;
repack t using index nope;
repack v;
"), @"
    error: index `nope` does not exist
      ╭▸ 
    7 │ repack t using index nope;
      ╰╴                     ━━━━
    error: `v` is not a table or materialized view
      ╭▸ 
    8 │ repack v;
      ╰╴       ━
    ");
}

#[test]
fn alter_index_rename() {
    assert_snapshot!(check("
create table t (a int, b int);
create index i on t (a);
create index j on t (b);
alter index i rename to i2;
drop index i;
alter index i2 rename to j;
alter index i2 rename to t;
alter index nope rename to x;
alter index if exists nope rename to x;
begin;
alter index i2 rename to i3;
rollback;
drop index i3;
drop index i2;
"), @"
    error: index `i` does not exist
      ╭▸ 
    6 │ drop index i;
      ╰╴           ━
    error: index `nope` does not exist
      ╭▸ 
    9 │ alter index nope rename to x;
      ╰╴            ━━━━
    error: index `i3` does not exist
       ╭▸ 
    14 │ drop index i3;
       ╰╴           ━━
    error: relation `j` already exists
      ╭▸ 
    7 │ alter index i2 rename to j;
      ╰╴                         ━
    error: relation `t` already exists
      ╭▸ 
    8 │ alter index i2 rename to t;
      ╰╴                         ━
    ");
}

#[test]
fn reindex_schema() {
    assert_snapshot!(check("
create schema s;
reindex schema s;
reindex schema concurrently s;
reindex (verbose) schema nope;
"), @"
    error: schema `nope` does not exist
      ╭▸ 
    5 │ reindex (verbose) schema nope;
      ╰╴                         ━━━━
    ");
}

#[test]
fn create_index_on_non_indexable_relation() {
    assert_snapshot!(check("
create view v as select 1 a;
create sequence s;
create materialized view m as select 1 a;
create table p (a int) partition by range (a);
create index iv on v (a);
create index is_ on s (last_value);
create index im on m (a);
create index ip on p (a);
drop index iv;
drop index im;
"), @"
    error: index `iv` does not exist
       ╭▸ 
    10 │ drop index iv;
       ╰╴           ━━
    error: cannot create index on view `v`
      ╭▸ 
    6 │ create index iv on v (a);
      ╰╴                   ━
    error: cannot create index on sequence `s`
      ╭▸ 
    7 │ create index is_ on s (last_value);
      ╰╴                    ━
    ");
}
