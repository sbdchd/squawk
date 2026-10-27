use insta::assert_snapshot;

use super::check;

#[test]
fn create_view_columns() {
    assert_snapshot!(check("
create table users (id int, name text);
create view v as select id, name as n from users;
select id, n, name from v;
"), @"
    error: column `name` does not exist
      ╭▸ 
    4 │ select id, n, name from v;
      ╰╴              ━━━━
    ");
}

#[test]
fn create_view_column_list() {
    assert_snapshot!(check("
create view v (x) as select 1 a, 2 b;
select x, b, a from v;
"), @"
    error: column `a` does not exist
      ╭▸ 
    3 │ select x, b, a from v;
      ╰╴             ━
    ");
}

#[test]
fn create_view_unknown_columns() {
    assert_snapshot!(check("
create view v as select * from foo;
select a from v;
"), @"
    error: relation `foo` does not exist
      ╭▸ 
    2 │ create view v as select * from foo;
      ╰╴                               ━━━
    ");
}

#[test]
fn create_view_query_errors() {
    assert_snapshot!(check("
create table users (id int);
create view v as select email from users;
"), @"
    error: column `email` does not exist
      ╭▸ 
    3 │ create view v as select email from users;
      ╰╴                        ━━━━━
    ");
}

#[test]
fn create_materialized_view() {
    assert_snapshot!(check("
create table users (id int, name text);
create materialized view m (x) as select id, name as n from users;
select x, n, id from m;
"), @"
    error: column `id` does not exist
      ╭▸ 
    4 │ select x, n, id from m;
      ╰╴             ━━
    ");
}

#[test]
fn duplicate_materialized_view() {
    assert_snapshot!(check("
create materialized view m as select 1;
create materialized view if not exists m as select 1;
create materialized view m as select 1;
"), @"
    error: relation `m` already exists
      ╭▸ 
    4 │ create materialized view m as select 1;
      ╰╴                         ━
    ");
}

#[test]
fn duplicate_materialized_view_after_view() {
    assert_snapshot!(check("
create view v as select 1;
create materialized view v as select 1;
"), @"
    error: relation `v` already exists
      ╭▸ 
    3 │ create materialized view v as select 1;
      ╰╴                         ━
    ");
}

#[test]
fn materialized_view_refs() {
    assert_snapshot!(check("
create table t (a int);
create view v as select 1 a;
create materialized view m as select 1 a;
refresh materialized view m;
refresh materialized view concurrently t;
alter materialized view m set (fillfactor = 70);
alter materialized view v set (fillfactor = 70);
alter materialized view if exists nope set (fillfactor = 70);
drop materialized view v;
drop view m;
drop materialized view m, nope;
drop materialized view if exists nope2;
select * from m;
"), @"
    error: relation `nope` does not exist
       ╭▸ 
    12 │ drop materialized view m, nope;
       ╰╴                          ━━━━
    error: relation `m` does not exist
       ╭▸ 
    14 │ select * from m;
       ╰╴              ━
    error: `t` is not a materialized view
      ╭▸ 
    6 │ refresh materialized view concurrently t;
      ╰╴                                       ━
    error: `v` is not a materialized view
      ╭▸ 
    8 │ alter materialized view v set (fillfactor = 70);
      ╰╴                        ━
    error: `v` is not a materialized view
       ╭▸ 
    10 │ drop materialized view v;
       │                        ━
       │
       ╰ hint: Use DROP VIEW to remove a view.
    error: `m` is not a view
       ╭▸ 
    11 │ drop view m;
       │           ━
       │
       ╰ hint: Use DROP MATERIALIZED VIEW to remove a materialized view.
    ");
}

#[test]
fn materialized_view_alter_actions() {
    assert_snapshot!(check("
create extension e;
create materialized view v as select 1 a;
alter materialized view v depends on extension e;
alter materialized view v no depends on extension e;
alter materialized view v depends on extension nope;
alter materialized view v rename column a to b;
select a from v;
alter materialized view v rename to w;
select b from v;
"), @"
    error: extension `nope` does not exist
      ╭▸ 
    6 │ alter materialized view v depends on extension nope;
      ╰╴                                               ━━━━
    error: column `a` does not exist
      ╭▸ 
    8 │ select a from v;
      ╰╴       ━
    error: relation `v` does not exist
       ╭▸ 
    10 │ select b from v;
       ╰╴              ━
    ");
}

#[test]
fn create_or_replace_view() {
    assert_snapshot!(check("
create view v as select 1;
create or replace view v as select 2;
create table t (a int);
create or replace view t as select 1;
"), @"
    error: relation `t` already exists
      ╭▸ 
    5 │ create or replace view t as select 1;
      ╰╴                       ━
    ");
}

#[test]
fn alter_view_rename() {
    assert_snapshot!(check("
create table t (a int);
create view v as select a from t;
alter view v rename to w;
select a from v;
select a from w;
create view x as select 1 b;
alter view w rename to x;
alter view w rename to t;
alter view nope rename to y;
alter view if exists nope rename to y;
alter view t rename to z;
"), @"
    error: relation `v` does not exist
      ╭▸ 
    5 │ select a from v;
      ╰╴              ━
    error: relation `nope` does not exist
       ╭▸ 
    10 │ alter view nope rename to y;
       ╰╴           ━━━━
    error: relation `x` already exists
      ╭▸ 
    8 │ alter view w rename to x;
      ╰╴                       ━
    error: relation `t` already exists
      ╭▸ 
    9 │ alter view w rename to t;
      ╰╴                       ━
    error: `t` is not a view
       ╭▸ 
    12 │ alter view t rename to z;
       ╰╴           ━
    ");
}

#[test]
fn alter_view_set_schema() {
    assert_snapshot!(check("
create schema s;
create view v as select 1 a;
alter view v set schema s;
select a from v;
select a from s.v;
alter view s.v set schema nope;
alter view s.v owner to nope;
"), @"
    error: relation `v` does not exist
      ╭▸ 
    5 │ select a from v;
      ╰╴              ━
    error: schema `nope` does not exist
      ╭▸ 
    7 │ alter view s.v set schema nope;
      ╰╴                          ━━━━
    error: role `nope` does not exist
      ╭▸ 
    8 │ alter view s.v owner to nope;
      ╰╴                        ━━━━
    ");
}

#[test]
fn alter_view_rename_column() {
    assert_snapshot!(check("
create view v as select 1 a, 2 b;
alter view v rename column a to c;
select c, b from v;
select a from v;
alter view v rename column c to b;
alter view v rename column nope to d;
"), @"
    error: column `a` does not exist
      ╭▸ 
    5 │ select a from v;
      ╰╴       ━
    error: column `nope` does not exist
      ╭▸ 
    7 │ alter view v rename column nope to d;
      ╰╴                           ━━━━
    error: column `b` of relation `v` already exists
      ╭▸ 
    6 │ alter view v rename column c to b;
      ╰╴                                ━
    ");
}

#[test]
fn alter_view_column_default() {
    assert_snapshot!(check("
create view v as select 1 a;
alter view v alter column a set default nope();
alter view v alter column nope drop default;
alter view v set (security_barrier = true);
alter view v reset (security_barrier);
"), @"
    error: column `nope` does not exist
      ╭▸ 
    4 │ alter view v alter column nope drop default;
      ╰╴                          ━━━━
    error: function `nope()` does not exist
      ╭▸ 
    3 │ alter view v alter column a set default nope();
      ╰╴                                        ━━━━
    ");
}

#[test]
fn alter_view_rename_rollback() {
    assert_snapshot!(check("
create view v as select 1 a;
begin;
alter view v rename to w;
alter view w rename column a to b;
rollback;
select a from v;
select b from w;
"), @"
    error: relation `w` does not exist
      ╭▸ 
    8 │ select b from w;
      ╰╴              ━
    ");
}

#[test]
fn view_column_count_mismatch() {
    assert_snapshot!(check("
create view v (a, b) as select 1;
create materialized view m (a, b) as select 1;
create view w (a) as select 1, 2;
create view z (a, b) as select * from missing;
"), @"
    error: relation `missing` does not exist
      ╭▸ 
    5 │ create view z (a, b) as select * from missing;
      ╰╴                                      ━━━━━━━
    error: CREATE VIEW specifies more column names than columns
      ╭▸ 
    2 │ create view v (a, b) as select 1;
      ╰╴                  ━
    error: too many column names were specified
      ╭▸ 
    3 │ create materialized view m (a, b) as select 1;
      ╰╴                               ━
    ");
}

#[test]
fn refresh_concurrently_without_unique_index() {
    assert_snapshot!(check("
create table t (
  a int,
  b int
);
create materialized view mv as
  select a, b from t;
refresh materialized view concurrently mv;
create unique index mv_lower on mv (lower(b::text));
create unique index mv_partial on mv (a) where a > 0;
refresh materialized view concurrently mv;
refresh materialized view mv;
create unique index mv_a on mv ((a));
refresh materialized view concurrently mv;
drop index mv_a;
refresh materialized view concurrently mv;
create unique index on mv (a, b);
refresh materialized view concurrently mv;
"), @"
    error: cannot refresh materialized view `mv` concurrently
      ╭▸ 
    8 │ refresh materialized view concurrently mv;
      │                                        ━━
      │
      ╰ hint: Create a unique index with no WHERE clause on one or more columns of the materialized view.
    error: cannot refresh materialized view `mv` concurrently
       ╭▸ 
    11 │ refresh materialized view concurrently mv;
       │                                        ━━
       │
       ╰ hint: Create a unique index with no WHERE clause on one or more columns of the materialized view.
    error: cannot refresh materialized view `mv` concurrently
       ╭▸ 
    16 │ refresh materialized view concurrently mv;
       │                                        ━━
       │
       ╰ hint: Create a unique index with no WHERE clause on one or more columns of the materialized view.
    ");
}
