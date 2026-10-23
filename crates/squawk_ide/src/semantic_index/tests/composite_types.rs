use insta::assert_snapshot;

use super::check;

#[test]
fn comment_on_column_composite_type_attribute_missing() {
    assert_snapshot!(check("
create type address as (city text, zip text);
create type mood as enum ('happy');
comment on column address.city is 'x';
comment on column address.street is 'x';
comment on column mood.happy is 'x';
"), @"
    error: column `street` does not exist
      ╭▸ 
    5 │ comment on column address.street is 'x';
      ╰╴                          ━━━━━━
    error: relation `mood` does not exist
      ╭▸ 
    6 │ comment on column mood.happy is 'x';
      ╰╴                  ━━━━
    ");
}

#[test]
fn alter_type_attribute_typed_table() {
    assert_snapshot!(check("
create type t as (a int, b text);
create table x of t;
alter type t add attribute c int;
alter type t drop attribute b;
alter type t rename attribute a to a2;
alter type t alter attribute a type bigint;
select a, b from x;
select c from x;
alter type t add attribute c int cascade;
alter type t drop attribute b cascade;
alter type t rename attribute a to a2 cascade;
select a2, c from x;
select a, b from x;
"), @"
    error: column `c` does not exist
      ╭▸ 
    9 │ select c from x;
      ╰╴       ━
    error: column `a` does not exist
       ╭▸ 
    14 │ select a, b from x;
       ╰╴       ━
    error: column `b` does not exist
       ╭▸ 
    14 │ select a, b from x;
       ╰╴          ━
    error: cannot alter type `t` because it is the type of a typed table
      ╭▸ 
    4 │ alter type t add attribute c int;
      │            ━
      │
      ╰ hint: Use ALTER ... CASCADE to alter the typed tables too.
    error: cannot alter type `t` because it is the type of a typed table
      ╭▸ 
    5 │ alter type t drop attribute b;
      │            ━
      │
      ╰ hint: Use ALTER ... CASCADE to alter the typed tables too.
    error: cannot alter type `t` because it is the type of a typed table
      ╭▸ 
    6 │ alter type t rename attribute a to a2;
      │            ━
      │
      ╰ hint: Use ALTER ... CASCADE to alter the typed tables too.
    error: cannot alter type `t` because it is the type of a typed table
      ╭▸ 
    7 │ alter type t alter attribute a type bigint;
      │            ━
      │
      ╰ hint: Use ALTER ... CASCADE to alter the typed tables too.
    ");
}

#[test]
fn alter_type_attribute_typed_table_of() {
    assert_snapshot!(check("
create type t as (a int);
create table y (a int);
alter table y of t;
alter type t add attribute b int;
alter table y not of;
alter type t add attribute b int;
select b from y;
begin;
alter table y of t;
rollback;
alter type t add attribute c int;
"), @"
    error: column `b` does not exist
      ╭▸ 
    8 │ select b from y;
      ╰╴       ━
    error: cannot alter type `t` because it is the type of a typed table
      ╭▸ 
    5 │ alter type t add attribute b int;
      │            ━
      │
      ╰ hint: Use ALTER ... CASCADE to alter the typed tables too.
    ");
}

#[test]
fn alter_type_attribute_type_typed_table() {
    assert_snapshot!(check("
create type t as (a int);
create table x of t;
create function g(int) returns int language sql return 1;
select g(a) from x;
alter type t alter attribute a type text cascade;
select g(a) from x;
"), @"
    error: function `g` does not exist
      ╭▸ 
    7 │ select g(a) from x;
      ╰╴       ━
    ");
}

#[test]
fn alter_type_attributes() {
    assert_snapshot!(check("
create type t as (a int, b text);
alter type t add attribute c int, drop attribute b;
select (null::t).a, (null::t).c;
select (null::t).b;
alter type t drop attribute b;
alter type t drop attribute if exists b;
alter type t add attribute a int;
alter type t alter attribute a type text, alter attribute nope type int;
alter type t add attribute d nope_type;
"), @"
    error: column `b` does not exist
      ╭▸ 
    5 │ select (null::t).b;
      ╰╴                 ━
    error: column `b` does not exist
      ╭▸ 
    6 │ alter type t drop attribute b;
      ╰╴                            ━
    error: column `nope` does not exist
      ╭▸ 
    9 │ alter type t alter attribute a type text, alter attribute nope type int;
      ╰╴                                                          ━━━━
    error: type `nope_type` does not exist
       ╭▸ 
    10 │ alter type t add attribute d nope_type;
       ╰╴                             ━━━━━━━━━
    error: column `a` of relation `t` already exists
      ╭▸ 
    8 │ alter type t add attribute a int;
      ╰╴                           ━
    ");
}

#[test]
fn alter_type_alter_attribute_type() {
    assert_snapshot!(check("
create type t as (a int);
create function g(text) returns int language sql return 1;
select g((null::t).a);
alter type t alter attribute a set data type text;
select g((null::t).a);
"), @"
    error: function `g` does not exist
      ╭▸ 
    4 │ select g((null::t).a);
      ╰╴       ━
    ");
}

#[test]
fn alter_type_rename_attribute() {
    assert_snapshot!(check("
create type t as (a int, b int);
alter type t rename attribute a to x;
select (null::t).x;
select (null::t).a;
alter type t rename attribute x to b;
alter type t rename attribute nope to y;
"), @"
    error: column `a` does not exist
      ╭▸ 
    5 │ select (null::t).a;
      ╰╴                 ━
    error: column `nope` does not exist
      ╭▸ 
    7 │ alter type t rename attribute nope to y;
      ╰╴                              ━━━━
    error: column `b` of relation `t` already exists
      ╭▸ 
    6 │ alter type t rename attribute x to b;
      ╰╴                                   ━
    ");
}

#[test]
fn alter_type_attributes_rollback() {
    assert_snapshot!(check("
create type t as (a int);
begin;
alter type t drop attribute a, add attribute b int;
rollback;
select (null::t).a;
select (null::t).b;
"), @"
    error: column `b` does not exist
      ╭▸ 
    7 │ select (null::t).b;
      ╰╴                 ━
    ");
}

#[test]
fn create_typed_table() {
    assert_snapshot!(check("
create type ty as (x int, y text);
create table t of ty;
select x, y, nope from t;
create table u of ty (x with options default 1, primary key (x));
select x, y from u;
create table v of nope_type;
select z from v;
"), @"
    error: column `nope` does not exist
      ╭▸ 
    4 │ select x, y, nope from t;
      ╰╴             ━━━━
    error: type `nope_type` does not exist
      ╭▸ 
    7 │ create table v of nope_type;
      ╰╴                  ━━━━━━━━━
    ");
}

#[test]
fn alter_type_attribute_on_non_composite() {
    assert_snapshot!(check("
create type c as (a int);
create type e as enum ('x');
create domain d as int;
create type r as range (subtype = int);
create type sh;
alter type c add attribute b text;
alter type c rename attribute a to a2;
alter type e add attribute b text;
alter type d drop attribute a;
alter type r alter attribute a type text;
alter type sh rename attribute a to b;
alter type e rename to e2;
"), @"
    error: `e` is not a composite type
      ╭▸ 
    9 │ alter type e add attribute b text;
      ╰╴           ━
    error: `d` is not a composite type
       ╭▸ 
    10 │ alter type d drop attribute a;
       ╰╴           ━
    error: `r` is not a composite type
       ╭▸ 
    11 │ alter type r alter attribute a type text;
       ╰╴           ━
    error: `sh` is not a composite type
       ╭▸ 
    12 │ alter type sh rename attribute a to b;
       ╰╴           ━━
    ");
}
