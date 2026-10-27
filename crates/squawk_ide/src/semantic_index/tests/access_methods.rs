use insta::assert_snapshot;

use super::check;

#[test]
fn create_access_method() {
    assert_snapshot!(check("
create function h(internal) returns index_am_handler language c as 'h';
create function wrong(int) returns index_am_handler language c as 'h';
create access method a type index handler h;
create access method b type table handler missing;
create access method c type index handler wrong;
create access method a type index handler h;
create access method btree type index handler h;
"), @"
    error: function `missing` does not exist
      ╭▸ 
    5 │ create access method b type table handler missing;
      ╰╴                                          ━━━━━━━
    error: function `wrong` does not exist
      ╭▸ 
    6 │ create access method c type index handler wrong;
      ╰╴                                          ━━━━━
    error: access method `a` already exists
      ╭▸ 
    7 │ create access method a type index handler h;
      ╰╴                     ━
    error: access method `btree` already exists
      ╭▸ 
    8 │ create access method btree type index handler h;
      ╰╴                     ━━━━━
    ");
}

#[test]
fn drop_access_method() {
    assert_snapshot!(check("
create function h(internal) returns index_am_handler language c as 'h';
create access method a type index handler h;
drop access method a;
drop access method a;
drop access method if exists a;
create access method a type index handler h;
"), @"
    error: access method `a` does not exist
      ╭▸ 
    5 │ drop access method a;
      ╰╴                   ━
    ");
}

#[test]
fn access_method_rollback() {
    assert_snapshot!(check("
create function h(internal) returns index_am_handler language c as 'h';
create table t (a int);
begin;
create access method a type index handler h;
rollback;
create index on t using a (a);
"), @"
    error: access method `a` does not exist
      ╭▸ 
    7 │ create index on t using a (a);
      ╰╴                        ━
    ");
}

#[test]
fn access_method_refs() {
    assert_snapshot!(check("
create table t (a int) using heap;
create table t2 (a int) using btree;
create table t3 (a int) using missing;
create table t4 using heap as select 1 as a;
create materialized view m using heap as select 1 as a;
create materialized view m2 using gist as select 1 as a;
create index on t using btree (a);
create index on t using hash (a);
create index on t using heap (a);
create index on t using missing (a);
alter table t set access method heap;
alter table t set access method gin;
create table t5 (a int, exclude using gist (a with =));
create table t6 (a int, exclude using missing (a with =));
"), @"
    error: access method `btree` is not of type TABLE
      ╭▸ 
    3 │ create table t2 (a int) using btree;
      ╰╴                              ━━━━━
    error: access method `missing` does not exist
      ╭▸ 
    4 │ create table t3 (a int) using missing;
      ╰╴                              ━━━━━━━
    error: access method `gist` is not of type TABLE
      ╭▸ 
    7 │ create materialized view m2 using gist as select 1 as a;
      ╰╴                                  ━━━━
    error: access method `heap` is not of type INDEX
       ╭▸ 
    10 │ create index on t using heap (a);
       ╰╴                        ━━━━
    error: access method `missing` does not exist
       ╭▸ 
    11 │ create index on t using missing (a);
       ╰╴                        ━━━━━━━
    error: access method `gin` is not of type TABLE
       ╭▸ 
    13 │ alter table t set access method gin;
       ╰╴                                ━━━
    error: access method `missing` does not exist
       ╭▸ 
    15 │ create table t6 (a int, exclude using missing (a with =));
       ╰╴                                      ━━━━━━━
    ");
}

#[test]
fn tablesample() {
    assert_snapshot!(check("
create table t (a int);
select a from t tablesample bernoulli (10);
select a from t tablesample system (10) repeatable (42);
select a from t tablesample pg_catalog.system (10);
select a from t tablesample nope (10);
select a from t tablesample bernoulli (nope);
select x.a from t as x tablesample system (1 + 2) repeatable (nope2);
"), @"
    error: function `nope` does not exist
      ╭▸ 
    6 │ select a from t tablesample nope (10);
      ╰╴                            ━━━━
    error: column `nope` does not exist
      ╭▸ 
    7 │ select a from t tablesample bernoulli (nope);
      ╰╴                                       ━━━━
    error: column `nope2` does not exist
      ╭▸ 
    8 │ select x.a from t as x tablesample system (1 + 2) repeatable (nope2);
      ╰╴                                                              ━━━━━
    ");
}
