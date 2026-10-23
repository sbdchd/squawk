use insta::assert_snapshot;

use super::check;

#[test]
fn collation_unknown() {
    assert_snapshot!(check(r#"
create table t(a text collate "C", b text collate missing);
create type c as (x text collate nope);
create type r as range (subtype = text, collation = "POSIX");
create index i on t (a collate pg_catalog."default");
insert into t values ('a') on conflict (a collate ucs_basic) do nothing;
select a collate "unicode", b collate foo.bar from t;
create collation x from typo;
alter collation y refresh version;
drop collation z;
drop collation if exists w;
"#), @r#"
    error: collation `missing` does not exist
      ╭▸ 
    2 │ create table t(a text collate "C", b text collate missing);
      ╰╴                                                  ━━━━━━━
    error: collation `nope` does not exist
      ╭▸ 
    3 │ create type c as (x text collate nope);
      ╰╴                                 ━━━━
    error: schema `foo` does not exist
      ╭▸ 
    7 │ select a collate "unicode", b collate foo.bar from t;
      ╰╴                                      ━━━
    error: collation `bar` does not exist
      ╭▸ 
    7 │ select a collate "unicode", b collate foo.bar from t;
      ╰╴                                          ━━━
    error: collation `typo` does not exist
      ╭▸ 
    8 │ create collation x from typo;
      ╰╴                        ━━━━
    error: collation `y` does not exist
      ╭▸ 
    9 │ alter collation y refresh version;
      ╰╴                ━
    error: collation `z` does not exist
       ╭▸ 
    10 │ drop collation z;
       ╰╴               ━
    "#);
}

#[test]
fn collation_created_and_dropped() {
    assert_snapshot!(check(r#"
create collation c (provider = icu, locale = 'und');
create collation if not exists c (provider = icu, locale = 'und');
create collation d from c;
select 'a' collate c, 'b' collate public.d;
drop collation c, d;
select 'a' collate c;
"#), @"
    error: collation `c` does not exist
      ╭▸ 
    7 │ select 'a' collate c;
      ╰╴                   ━
    ");
}

#[test]
fn collation_schema_search_path() {
    assert_snapshot!(check(r#"
create schema s;
create collation s.c (locale = 'C');
select 'a' collate c;
set search_path = s;
select 'a' collate c;
"#), @"
    error: collation `c` does not exist
      ╭▸ 
    4 │ select 'a' collate c;
      ╰╴                   ━
    ");
}

#[test]
fn collation_ignores_temp_schema() {
    assert_snapshot!(check(r#"
create collation pg_temp.c (locale = 'C');
select 'a' collate c;
select 'a' collate pg_temp.c;
"#), @"
    error: collation `c` does not exist
      ╭▸ 
    3 │ select 'a' collate c;
      ╰╴                   ━
    ");
}

#[test]
fn collation_dropped_in_rolled_back_transaction() {
    assert_snapshot!(check(r#"
create collation c (locale = 'C');
begin;
drop collation c;
rollback;
select 'a' collate c;
begin;
create collation d (locale = 'C');
rollback;
select 'a' collate d;
"#), @"
    error: collation `d` does not exist
       ╭▸ 
    10 │ select 'a' collate d;
       ╰╴                   ━
    ");
}

#[test]
fn duplicate_collation() {
    assert_snapshot!(check(r#"
create collation c from "C";
create collation c from "C";
create collation if not exists c from "C";
"#), @r#"
    error: collation `c` already exists
      ╭▸ 
    3 │ create collation c from "C";
      ╰╴                 ━
    "#);
}

#[test]
fn alter_collation_rename_and_set_schema() {
    assert_snapshot!(check("
create schema s;
create collation c (locale = 'C');
create collation d (locale = 'C');
create collation s.e (locale = 'C');
alter collation c rename to c2;
create table t (a text collate c);
create table u (a text collate c2);
alter collation c2 rename to d;
alter collation c2 set schema s;
create table v (a text collate c2);
create table w (a text collate s.c2);
alter collation d rename to e;
alter collation e set schema s;
alter collation nope rename to x;
alter collation s.c2 set schema nope;
begin;
alter collation s.c2 rename to c3;
rollback;
drop collation s.c3;
drop collation s.c2;
"), @"
    error: collation `c` does not exist
      ╭▸ 
    7 │ create table t (a text collate c);
      ╰╴                               ━
    error: collation `c2` does not exist
       ╭▸ 
    11 │ create table v (a text collate c2);
       ╰╴                               ━━
    error: collation `nope` does not exist
       ╭▸ 
    15 │ alter collation nope rename to x;
       ╰╴                ━━━━
    error: schema `nope` does not exist
       ╭▸ 
    16 │ alter collation s.c2 set schema nope;
       ╰╴                                ━━━━
    error: collation `c3` does not exist
       ╭▸ 
    20 │ drop collation s.c3;
       ╰╴                 ━━
    error: collation `d` already exists
      ╭▸ 
    9 │ alter collation c2 rename to d;
      ╰╴                             ━
    error: collation `e` already exists in schema `s`
       ╭▸ 
    14 │ alter collation e set schema s;
       ╰╴                             ━
    ");
}
