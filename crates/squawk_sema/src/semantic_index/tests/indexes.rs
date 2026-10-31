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
      │                          ━
      ╰╴
    help: a column with a similar name exists
      ╭╴
    3 - create index idx on t(a, b) include (c) where d > 0;
    3 + create index idx on t(a, a) include (c) where d > 0;
      ╰╴
    error: column `c` does not exist
      ╭▸ 
    3 │ create index idx on t(a, b) include (c) where d > 0;
      │                                      ━
      ╰╴
    help: a column with a similar name exists
      ╭╴
    3 - create index idx on t(a, b) include (c) where d > 0;
    3 + create index idx on t(a, b) include (a) where d > 0;
      ╰╴
    error: column `d` does not exist
      ╭▸ 
    3 │ create index idx on t(a, b) include (c) where d > 0;
      │                                               ━
      ╰╴
    help: a column with a similar name exists
      ╭╴
    3 - create index idx on t(a, b) include (c) where d > 0;
    3 + create index idx on t(a, b) include (c) where a > 0;
      ╰╴
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
    error: relation `j` already exists
      ╭▸ 
    7 │ alter index i2 rename to j;
      ╰╴                         ━
    error: relation `t` already exists
      ╭▸ 
    8 │ alter index i2 rename to t;
      ╰╴                         ━
    error: index `nope` does not exist
      ╭▸ 
    9 │ alter index nope rename to x;
      ╰╴            ━━━━
    error: index `i3` does not exist
       ╭▸ 
    14 │ drop index i3;
       ╰╴           ━━
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
    error: cannot create index on view `v`
      ╭▸ 
    6 │ create index iv on v (a);
      ╰╴                   ━
    error: cannot create index on sequence `s`
      ╭▸ 
    7 │ create index is_ on s (last_value);
      ╰╴                    ━
    error: index `iv` does not exist
       ╭▸ 
    10 │ drop index iv;
       ╰╴           ━━
    ");
}

#[test]
fn unnamed_index_generated_name() {
    assert_snapshot!(check("
create table ti (a int, b int);
create index on ti (a);
create unique index on ti (a);
create index on ti (lower(b::text)) include (b);
drop index ti_a_idx;
drop index ti_a_idx1;
drop index ti_expr_b_idx;
drop index ti_lower_b_idx;
create index on ti (a);
create index ti_a_idx on ti (b);
"), @"
    error: index `ti_expr_b_idx` does not exist
      ╭▸ 
    8 │ drop index ti_expr_b_idx;
      ╰╴           ━━━━━━━━━━━━━
    error: relation `ti_a_idx` already exists
       ╭▸ 
    11 │ create index ti_a_idx on ti (b);
       ╰╴             ━━━━━━━━
    ");
}

#[test]
fn unique_index_unsupported_access_method() {
    assert_snapshot!(check("
create table t (
  a int,
  r int4range,
  v tsvector
);
create unique index on t using btree (a);
create unique index on t using hash (a);
create unique index on t using gist (r);
create unique index on t using gin (v);
create unique index on t using spgist (r);
create unique index on t using brin (a);
create index on t using gin (v);
create extension bloom;
create unique index on t using bloom (a);
"), @"
    error: access method `hash` does not support unique indexes
      ╭▸ 
    8 │ create unique index on t using hash (a);
      ╰╴                               ━━━━
    error: access method `gist` does not support unique indexes
      ╭▸ 
    9 │ create unique index on t using gist (r);
      ╰╴                               ━━━━
    error: access method `gin` does not support unique indexes
       ╭▸ 
    10 │ create unique index on t using gin (v);
       ╰╴                               ━━━
    error: access method `spgist` does not support unique indexes
       ╭▸ 
    11 │ create unique index on t using spgist (r);
       ╰╴                               ━━━━━━
    error: access method `brin` does not support unique indexes
       ╭▸ 
    12 │ create unique index on t using brin (a);
       ╰╴                               ━━━━
    error: access method `bloom` does not support unique indexes
       ╭▸ 
    15 │ create unique index on t using bloom (a);
       ╰╴                               ━━━━━
    ");
}

#[test]
fn drop_column_drops_dependent_indexes() {
    assert_snapshot!(check("
create table t (
  a int,
  b int,
  c int
);
create index t_inc on t (a) include (b);
create index t_expr on t (lower(b::text));
create index t_pred on t (a) where c > 0;
alter table t drop column b;
alter table t drop column c;
drop index t_inc;
drop index t_expr;
drop index t_pred;

create table u (
  a int,
  b int,
  unique (a) include (b)
);
alter table u drop column b;
alter table u drop constraint u_a_key;

create table e (
  a int,
  b int,
  exclude ((a + b) with =)
);
alter table e drop column b;
alter table e drop constraint e_expr_excl;

create table r (
  a int,
  b int
);
create index r_expr on r ((a + b));
alter table r rename column b to x;
alter table r drop column x;
drop index r_expr;
"), @"
    error: index `t_inc` does not exist
       ╭▸ 
    12 │ drop index t_inc;
       ╰╴           ━━━━━
    error: index `t_expr` does not exist
       ╭▸ 
    13 │ drop index t_expr;
       ╰╴           ━━━━━━
    error: index `t_pred` does not exist
       ╭▸ 
    14 │ drop index t_pred;
       ╰╴           ━━━━━━
    error: constraint `u_a_key` does not exist
       ╭▸ 
    22 │ alter table u drop constraint u_a_key;
       ╰╴                              ━━━━━━━
    error: constraint `e_expr_excl` does not exist
       ╭▸ 
    30 │ alter table e drop constraint e_expr_excl;
       ╰╴                              ━━━━━━━━━━━
    error: index `r_expr` does not exist
       ╭▸ 
    39 │ drop index r_expr;
       ╰╴           ━━━━━━
    ");
}
