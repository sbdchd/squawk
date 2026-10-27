use insta::assert_snapshot;

use super::check;

#[test]
fn unresolved_type() {
    assert_snapshot!(check("
create type mood as enum ('a');
select 'a'::mood, 'a'::nope, 1::int, '{}'::text[], now()::timestamptz, 1::double precision;
create table t (a nope2);
"), @"
    error: type `nope` does not exist
      ╭▸ 
    3 │ select 'a'::mood, 'a'::nope, 1::int, '{}'::text[], now()::timestamptz, 1::double precision;
      ╰╴                       ━━━━
    error: type `nope2` does not exist
      ╭▸ 
    4 │ create table t (a nope2);
      ╰╴                  ━━━━━
    ");
}

#[test]
fn dropped_type() {
    assert_snapshot!(check("
create type mood as enum ('a');
create domain posint as int;
create type floatrange as range (subtype = float8);
drop type mood, nope;
drop type if exists nope2;
drop domain mood;
drop domain posint;
drop type floatrange;
select 'a'::mood, 1::posint, '{}'::floatmultirange;
"), @"
    error: type `nope` does not exist
      ╭▸ 
    5 │ drop type mood, nope;
      ╰╴                ━━━━
    error: type `mood` does not exist
      ╭▸ 
    7 │ drop domain mood;
      ╰╴            ━━━━
    error: type `mood` does not exist
       ╭▸ 
    10 │ select 'a'::mood, 1::posint, '{}'::floatmultirange;
       ╰╴            ━━━━
    error: type `posint` does not exist
       ╭▸ 
    10 │ select 'a'::mood, 1::posint, '{}'::floatmultirange;
       ╰╴                     ━━━━━━
    error: type `floatmultirange` does not exist
       ╭▸ 
    10 │ select 'a'::mood, 1::posint, '{}'::floatmultirange;
       ╰╴                                   ━━━━━━━━━━━━━━━
    ");
}

#[test]
fn rollback_drop_type() {
    assert_snapshot!(check("
create type e as enum ('a');
begin;
drop type e;
rollback;
create table t (a e);
"), @"no diagnostics");
}

#[test]
fn duplicate_type() {
    assert_snapshot!(check("
create type e as enum ('a');
create type e as enum ('b');
create domain e as int;
create table e (a int);
create table t (a int);
create type t as (a int);
"), @"
    error: type `e` already exists
      ╭▸ 
    3 │ create type e as enum ('b');
      ╰╴            ━
    error: type `e` already exists
      ╭▸ 
    4 │ create domain e as int;
      ╰╴              ━
    error: type `e` already exists
      ╭▸ 
    5 │ create table e (a int);
      ╰╴             ━
    error: type `t` already exists
      ╭▸ 
    7 │ create type t as (a int);
      ╰╴            ━
    ");
}

#[test]
fn shell_type_completed() {
    assert_snapshot!(check("
create type t;
create function t_in(cstring) returns t language internal as 'int4in';
create function t_out(t) returns cstring language internal as 'int4out';
create type t (input = t_in, output = t_out);
create type t (input = t_in, output = t_out);
"), @"
    error: type `t` already exists
      ╭▸ 
    6 │ create type t (input = t_in, output = t_out);
      ╰╴            ━
    ");
}

#[test]
fn shell_type_completion_rolled_back() {
    assert_snapshot!(check("
create type t;
begin;
create type t (input = int4in, output = int4out);
rollback;
create type t (input = int4in, output = int4out);
"), @"no diagnostics");
}

#[test]
fn base_type_unresolved_functions() {
    assert_snapshot!(check("
create type t (
  input = t_in,
  output = t_out,
  receive = t_recv,
  send = t_send,
  typmod_in = t_typmod_in,
  typmod_out = t_typmod_out,
  analyze = t_analyze,
  subscript = t_subscript,
  like = missing,
  element = missing_element
);
"), @"
    error: function `t_in` does not exist
      ╭▸ 
    3 │   input = t_in,
      ╰╴          ━━━━
    error: function `t_out` does not exist
      ╭▸ 
    4 │   output = t_out,
      ╰╴           ━━━━━
    error: function `t_recv` does not exist
      ╭▸ 
    5 │   receive = t_recv,
      ╰╴            ━━━━━━
    error: function `t_send` does not exist
      ╭▸ 
    6 │   send = t_send,
      ╰╴         ━━━━━━
    error: function `t_typmod_in` does not exist
      ╭▸ 
    7 │   typmod_in = t_typmod_in,
      ╰╴              ━━━━━━━━━━━
    error: function `t_typmod_out` does not exist
      ╭▸ 
    8 │   typmod_out = t_typmod_out,
      ╰╴               ━━━━━━━━━━━━
    error: function `t_analyze` does not exist
      ╭▸ 
    9 │   analyze = t_analyze,
      ╰╴            ━━━━━━━━━
    error: function `t_subscript` does not exist
       ╭▸ 
    10 │   subscript = t_subscript,
       ╰╴              ━━━━━━━━━━━
    error: type `missing` does not exist
       ╭▸ 
    11 │   like = missing,
       ╰╴         ━━━━━━━
    error: type `missing_element` does not exist
       ╭▸ 
    12 │   element = missing_element
       ╰╴            ━━━━━━━━━━━━━━━
    ");
}

#[test]
fn drop_types_together() {
    assert_snapshot!(check("
create type a as enum ('x');
create type b as (x a);
create domain d as a;
drop type a, b;
drop type a, b, d;
"), @"
    error: cannot drop type `a` because other objects depend on it
      ╭▸ 
    5 │ drop type a, b;
      ╰╴          ━
    ");
}

#[test]
fn alter_type_rename_and_schema() {
    assert_snapshot!(check("
create schema s;
create type t as (a int);
create table u (c t);
alter type t rename to t2;
select 1::t;
select (c).a from u;
alter type t2 set schema s;
select null::s.t2;
alter type nope rename to x;
alter type s.t2 owner to nope;
"), @"
    error: type `t` does not exist
      ╭▸ 
    6 │ select 1::t;
      ╰╴          ━
    error: type `nope` does not exist
       ╭▸ 
    10 │ alter type nope rename to x;
       ╰╴           ━━━━
    error: role `nope` does not exist
       ╭▸ 
    11 │ alter type s.t2 owner to nope;
       ╰╴                         ━━━━
    ");
}

#[test]
fn alter_type_set_options() {
    assert_snapshot!(check("
create type t;
create function t_in(cstring) returns t language c as 'x';
create function t_out(t) returns cstring language c as 'x';
create type t (input = t_in, output = t_out);
create function t_send(t) returns bytea language c as 'x';
alter type t set (send = t_send, receive = nope);
drop function t_send(t);
drop function t_send(t) cascade;
select null::t;
"), @"
    error: function `nope` does not exist
      ╭▸ 
    7 │ alter type t set (send = t_send, receive = nope);
      ╰╴                                           ━━━━
    error: cannot drop function `t_send` because other objects depend on it
      ╭▸ 
    8 │ drop function t_send(t);
      ╰╴              ━━━━━━
    error: type `t` does not exist
       ╭▸ 
    10 │ select null::t;
       ╰╴             ━
    ");
}
