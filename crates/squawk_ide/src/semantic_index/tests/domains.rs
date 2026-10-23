use insta::assert_snapshot;

use super::check;

#[test]
fn domain_args_use_base_type() {
    assert_snapshot!(check("
create domain posint as int;
create domain small as posint;
create function f(x int) returns int language sql return 1;
create function f(x bigint) returns int language sql return 2;
create function g(x text) returns int language sql return 1;
create function h(x posint) returns int language sql return 1;
create function h(x int) returns int language sql return 2;
create table t (a posint, b small, c int, d text);
select f(a), f(b), g(a), g(b), h(a), h(c), h(d) from t;
"), @"
    error: function `g` does not exist
       ╭▸ 
    10 │ select f(a), f(b), g(a), g(b), h(a), h(c), h(d) from t;
       ╰╴                   ━
    error: function `g` does not exist
       ╭▸ 
    10 │ select f(a), f(b), g(a), g(b), h(a), h(c), h(d) from t;
       ╰╴                         ━
    error: function `h` does not exist
       ╭▸ 
    10 │ select f(a), f(b), g(a), g(b), h(a), h(c), h(d) from t;
       ╰╴                                           ━
    ");
}

#[test]
fn domain_params() {
    assert_snapshot!(check("
create domain dtext as text;
create function f(x dtext) returns int language sql return 1;
create function f(x int) returns int language sql return 2;
create function g(x dtext) returns int language sql return 1;
create table t (a int, b varchar, c text);
select f('x'), g(a), g(b), g(c) from t;
"), @"
    error: function `g` does not exist
      ╭▸ 
    7 │ select f('x'), g(a), g(b), g(c) from t;
      ╰╴               ━
    ");
}

#[test]
fn create_domain_qualifiers() {
    assert_snapshot!(check(r#"
create domain d as int constraint positive check (value > 0) not null default 1 collate "C";
create domain e as int check (nope > 0) default nope() collate nope;
create domain f as int primary key;
create domain g as int references t(a);
"#), @"
    error: column `nope` does not exist
      ╭▸ 
    3 │ create domain e as int check (nope > 0) default nope() collate nope;
      ╰╴                              ━━━━
    error: function `nope` does not exist
      ╭▸ 
    3 │ create domain e as int check (nope > 0) default nope() collate nope;
      ╰╴                                                ━━━━
    error: collation `nope` does not exist
      ╭▸ 
    3 │ create domain e as int check (nope > 0) default nope() collate nope;
      ╰╴                                                               ━━━━
    error: primary key constraints not possible for domains
      ╭▸ 
    4 │ create domain f as int primary key;
      ╰╴                       ━━━━━━━━━━━
    error: foreign key constraints not possible for domains
      ╭▸ 
    5 │ create domain g as int references t(a);
      ╰╴                       ━━━━━━━━━━━━━━━
    ");
}

#[test]
fn domain_duplicate_constraint() {
    assert_snapshot!(check("
create domain d as int constraint c check (value > 0) constraint c check (value < 10);
alter domain d add constraint c check (value <> 5);
create table t (a int constraint c check (a > 0));
"), @"
    error: constraint `c` for domain `d` already exists
      ╭▸ 
    2 │ create domain d as int constraint c check (value > 0) constraint c check (value < 10);
      ╰╴                                                                 ━
    error: constraint `c` for domain `d` already exists
      ╭▸ 
    3 │ alter domain d add constraint c check (value <> 5);
      ╰╴                              ━
    ");
}

#[test]
fn alter_domain_constraints() {
    assert_snapshot!(check("
create domain d as int check (value > 0) not null;
alter domain d drop constraint d_check;
alter domain d drop constraint d_check;
alter domain d drop constraint if exists d_check;
alter domain d drop constraint d_not_null;
alter domain d add constraint c check (value > 1) not valid;
alter domain d validate constraint c;
alter domain d validate constraint nope;
alter domain d rename constraint c to c2;
alter domain d drop constraint c;
alter domain d drop constraint c2;
alter domain nope drop constraint c;
"), @"
    error: constraint `d_check` does not exist
      ╭▸ 
    4 │ alter domain d drop constraint d_check;
      ╰╴                               ━━━━━━━
    error: constraint `nope` does not exist
      ╭▸ 
    9 │ alter domain d validate constraint nope;
      ╰╴                                   ━━━━
    error: constraint `c` does not exist
       ╭▸ 
    11 │ alter domain d drop constraint c;
       ╰╴                               ━
    error: type `nope` does not exist
       ╭▸ 
    13 │ alter domain nope drop constraint c;
       ╰╴             ━━━━
    ");
}

#[test]
fn alter_domain_not_null() {
    assert_snapshot!(check("
create domain d as int;
alter domain d set not null;
alter domain d drop constraint d_not_null;
alter domain d set not null;
alter domain d drop not null;
alter domain d drop constraint d_not_null;
"), @"
    error: constraint `d_not_null` does not exist
      ╭▸ 
    7 │ alter domain d drop constraint d_not_null;
      ╰╴                               ━━━━━━━━━━
    ");
}

#[test]
fn alter_domain_rename() {
    assert_snapshot!(check("
create domain d as int;
create table t (a d);
alter domain d rename to e;
select 1::d;
select 1::e, a from t;
create domain f as int;
alter domain e rename to f;
alter domain e set schema nope;
alter domain e owner to nope;
alter domain e set default nope();
alter domain e drop default;
"), @"
    error: type `d` does not exist
      ╭▸ 
    5 │ select 1::d;
      ╰╴          ━
    error: schema `nope` does not exist
      ╭▸ 
    9 │ alter domain e set schema nope;
      ╰╴                          ━━━━
    error: role `nope` does not exist
       ╭▸ 
    10 │ alter domain e owner to nope;
       ╰╴                        ━━━━
    error: function `nope` does not exist
       ╭▸ 
    11 │ alter domain e set default nope();
       ╰╴                           ━━━━
    error: type `f` already exists
      ╭▸ 
    8 │ alter domain e rename to f;
      ╰╴                         ━
    ");
}

#[test]
fn alter_domain_set_schema() {
    assert_snapshot!(check("
create schema s;
create domain d as int;
alter domain d set schema s;
select 1::d;
select 1::s.d;
"), @"
    error: type `d` does not exist
      ╭▸ 
    5 │ select 1::d;
      ╰╴          ━
    ");
}

#[test]
fn alter_domain_rename_rollback() {
    assert_snapshot!(check("
create domain d as int;
begin;
alter domain d rename to e;
rollback;
select 1::d;
select 1::e;
"), @"
    error: type `e` does not exist
      ╭▸ 
    7 │ select 1::e;
      ╰╴          ━
    ");
}

#[test]
fn alter_domain_on_non_domain() {
    assert_snapshot!(check("
create type t as (a int);
alter domain t add check (value > 0);
"), @"
    error: `t` is not a domain
      ╭▸ 
    3 │ alter domain t add check (value > 0);
      ╰╴             ━
    ");
}

#[test]
fn domain_statements_on_non_domain() {
    assert_snapshot!(check("
create type t as (a int);
drop domain t;
drop domain if exists t;
comment on domain t is 'x';
drop type t;
"), @"
    error: `t` is not a domain
      ╭▸ 
    3 │ drop domain t;
      ╰╴            ━
    error: `t` is not a domain
      ╭▸ 
    4 │ drop domain if exists t;
      ╰╴                      ━
    error: `t` is not a domain
      ╭▸ 
    5 │ comment on domain t is 'x';
      ╰╴                  ━
    ");
}
