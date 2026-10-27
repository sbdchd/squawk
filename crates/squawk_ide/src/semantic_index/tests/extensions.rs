use insta::assert_snapshot;

use super::check;

#[test]
fn extension_missing() {
    assert_snapshot!(check("
create extension e1 schema nope;
alter extension e2 update;
alter extension if_missing set schema public;
drop extension e3;
drop extension if exists e4;
"), @"
    error: schema `nope` does not exist
      ╭▸ 
    2 │ create extension e1 schema nope;
      ╰╴                           ━━━━
    error: extension `e2` does not exist
      ╭▸ 
    3 │ alter extension e2 update;
      ╰╴                ━━
    error: extension `if_missing` does not exist
      ╭▸ 
    4 │ alter extension if_missing set schema public;
      ╰╴                ━━━━━━━━━━
    error: extension `e3` does not exist
      ╭▸ 
    5 │ drop extension e3;
      ╰╴               ━━
    ");
}

#[test]
fn extension_created_and_dropped() {
    assert_snapshot!(check(r#"
create extension if not exists hstore with schema public version '1.8' cascade;
create extension if not exists hstore;
create extension "uuid-ossp";
alter extension hstore update to '1.8';
alter extension hstore set schema public;
drop extension hstore, "uuid-ossp" cascade;
drop extension hstore;
"#), @"
    error: extension `hstore` does not exist
      ╭▸ 
    8 │ drop extension hstore;
      ╰╴               ━━━━━━
    ");
}

#[test]
fn extension_rollback() {
    assert_snapshot!(check("
create extension a;
begin;
create extension b;
drop extension a;
rollback;
alter extension a update;
alter extension b update;
"), @"
    error: extension `b` does not exist
      ╭▸ 
    8 │ alter extension b update;
      ╰╴                ━
    ");
}

#[test]
fn alter_extension_drop_member_rollback() {
    assert_snapshot!(check("
create extension e;
create table t (a int);
alter extension e add table t;
begin;
alter extension e drop table t;
rollback;
drop extension e;
select * from t;
"), @"
    error: relation `t` does not exist
      ╭▸ 
    9 │ select * from t;
      ╰╴              ━
    ");
}

#[test]
fn duplicate_extension() {
    assert_snapshot!(check("
create extension e;
create extension if not exists e;
create extension e;
"), @"
    error: extension `e` already exists
      ╭▸ 
    4 │ create extension e;
      ╰╴                 ━
    ");
}

#[test]
fn index_depends_on_extension() {
    assert_snapshot!(check("
create extension e;
create table t (a int);
create index i on t (a);
alter index i depends on extension e;
alter index i no depends on extension e;
alter index i depends on extension nope;
"), @"
    error: extension `nope` does not exist
      ╭▸ 
    7 │ alter index i depends on extension nope;
      ╰╴                                   ━━━━
    ");
}

#[test]
fn alter_extension_add_drop() {
    assert_snapshot!(check("
create extension e;
create table t (a int);
create function f(int) returns int language sql as 'select 1';
alter extension e add table t;
alter extension e add table nope;
alter extension e drop function f(int);
alter extension e drop function f(text);
alter extension e add schema nope;
alter extension e add operator + (int, int);
alter extension e add cast (int as text);
alter extension nope add table t;
"), @"
    error: relation `nope` does not exist
      ╭▸ 
    6 │ alter extension e add table nope;
      ╰╴                            ━━━━
    error: schema `nope` does not exist
      ╭▸ 
    9 │ alter extension e add schema nope;
      ╰╴                             ━━━━
    error: extension `nope` does not exist
       ╭▸ 
    12 │ alter extension nope add table t;
       ╰╴                ━━━━
    error: function `f(text)` does not exist
      ╭▸ 
    8 │ alter extension e drop function f(text);
      ╰╴                                ━
    ");
}

#[test]
fn extension_objects_resolve() {
    assert_snapshot!(check("
create extension hstore;
create extension citext;
create table t (a hstore, b citext);
select akeys(a), a -> 'k', b = 'x' from t;
"), @"no diagnostics");
}

#[test]
fn extension_objects_in_schema() {
    assert_snapshot!(check("
create schema s;
create extension hstore schema s;
select s.akeys(null::s.hstore);
select akeys(null::hstore);
set search_path = s;
select akeys(null::hstore);
"), @"
    error: type `hstore` does not exist
      ╭▸ 
    5 │ select akeys(null::hstore);
      ╰╴                   ━━━━━━
    error: function `akeys` does not exist
      ╭▸ 
    5 │ select akeys(null::hstore);
      ╰╴       ━━━━━
    ");
}

#[test]
fn extension_objects_unknown_before_create() {
    assert_snapshot!(check("
select akeys(null::hstore);
create extension hstore;
"), @"
    error: type `hstore` does not exist
      ╭▸ 
    2 │ select akeys(null::hstore);
      ╰╴                   ━━━━━━
    error: function `akeys` does not exist
      ╭▸ 
    2 │ select akeys(null::hstore);
      ╰╴       ━━━━━
    ");
}

#[test]
fn drop_extension_drops_members() {
    assert_snapshot!(check("
create extension hstore;
drop extension hstore;
select akeys(null::hstore);
"), @"
    error: type `hstore` does not exist
      ╭▸ 
    4 │ select akeys(null::hstore);
      ╰╴                   ━━━━━━
    error: function `akeys` does not exist
      ╭▸ 
    4 │ select akeys(null::hstore);
      ╰╴       ━━━━━
    ");
}

#[test]
fn alter_extension_set_schema_moves_members() {
    assert_snapshot!(check("
create schema s;
create extension hstore;
alter extension hstore set schema s;
select s.akeys(null::s.hstore);
select akeys(null::hstore);
"), @"
    error: type `hstore` does not exist
      ╭▸ 
    6 │ select akeys(null::hstore);
      ╰╴                   ━━━━━━
    error: function `akeys` does not exist
      ╭▸ 
    6 │ select akeys(null::hstore);
      ╰╴       ━━━━━
    ");
}

#[test]
fn alter_extension_set_schema_rollback() {
    assert_snapshot!(check("
create schema s;
create extension hstore;
begin;
alter extension hstore set schema s;
rollback;
select akeys(null::hstore);
select s.akeys(null::s.hstore);
"), @"
    error: type `hstore` does not exist
      ╭▸ 
    8 │ select s.akeys(null::s.hstore);
      ╰╴                       ━━━━━━
    error: function `akeys` does not exist
      ╭▸ 
    8 │ select s.akeys(null::s.hstore);
      ╰╴         ━━━━━
    ");
}

#[test]
fn create_extension_rollback_drops_members() {
    assert_snapshot!(check("
begin;
create extension hstore;
rollback;
select akeys(null::hstore);
"), @"
    error: type `hstore` does not exist
      ╭▸ 
    5 │ select akeys(null::hstore);
      ╰╴                   ━━━━━━
    error: function `akeys` does not exist
      ╭▸ 
    5 │ select akeys(null::hstore);
      ╰╴       ━━━━━
    ");
}

#[test]
fn create_extension_if_not_exists_lowers_once() {
    assert_snapshot!(check("
create extension hstore;
create extension if not exists hstore;
create extension hstore;
select akeys(null::hstore);
"), @"
    error: extension `hstore` already exists
      ╭▸ 
    4 │ create extension hstore;
      ╰╴                 ━━━━━━
    ");
}
