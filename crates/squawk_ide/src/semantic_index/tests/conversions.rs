use insta::assert_snapshot;

use super::check;

#[test]
fn create_conversion_function_without_bool_arg() {
    assert_snapshot!(check("
create function f(int, int, cstring, internal, int) returns void language c as 'f';
create conversion c for 'LATIN1' to 'UTF8' from f;
"), @"
    error: function `f` does not exist
      ╭▸ 
    3 │ create conversion c for 'LATIN1' to 'UTF8' from f;
      ╰╴                                                ━
    ");
}

#[test]
fn create_conversion() {
    assert_snapshot!(check("
create function f(int, int, cstring, internal, int, bool) returns int language c as 'f';
create function g(int) returns int language c as 'g';
create conversion c for 'LATIN1' to 'UTF8' from f;
create default conversion s.c2 for 'LATIN1' to 'UTF8' from f;
create conversion c3 for 'LATIN1' to 'UTF8' from g;
create conversion c for 'LATIN1' to 'UTF8' from f;
create conversion public.c for 'LATIN1' to 'UTF8' from f;
create conversion utf8_to_iso_8859_1 for 'UTF8' to 'LATIN1' from utf8_to_iso8859_1;
"), @"
    error: schema `s` does not exist
      ╭▸ 
    5 │ create default conversion s.c2 for 'LATIN1' to 'UTF8' from f;
      ╰╴                          ━
    error: function `g` does not exist
      ╭▸ 
    6 │ create conversion c3 for 'LATIN1' to 'UTF8' from g;
      ╰╴                                                 ━
    error: conversion `c` already exists
      ╭▸ 
    7 │ create conversion c for 'LATIN1' to 'UTF8' from f;
      ╰╴                  ━
    error: conversion `c` already exists
      ╭▸ 
    8 │ create conversion public.c for 'LATIN1' to 'UTF8' from f;
      ╰╴                         ━
    ");
}

#[test]
fn alter_conversion() {
    assert_snapshot!(check("
create role r;
create schema s;
create function f(int, int, cstring, internal, int, bool) returns int language c as 'f';
create conversion c for 'LATIN1' to 'UTF8' from f;
create conversion c2 for 'LATIN1' to 'UTF8' from f;
create conversion s.c4 for 'LATIN1' to 'UTF8' from f;
alter conversion c owner to r;
alter conversion c owner to missing_role;
alter conversion missing owner to r;
alter conversion c rename to c2;
alter conversion c rename to c3;
alter conversion c3 set schema s;
alter conversion s.c3 owner to r;
alter conversion c3 owner to r;
alter conversion c2 rename to c4;
alter conversion c4 set schema s;
alter conversion c2 set schema missing_schema;
"), @"
    error: role `missing_role` does not exist
      ╭▸ 
    9 │ alter conversion c owner to missing_role;
      ╰╴                            ━━━━━━━━━━━━
    error: conversion `missing` does not exist
       ╭▸ 
    10 │ alter conversion missing owner to r;
       ╰╴                 ━━━━━━━
    error: conversion `c3` does not exist
       ╭▸ 
    15 │ alter conversion c3 owner to r;
       ╰╴                 ━━
    error: conversion `c2` does not exist
       ╭▸ 
    18 │ alter conversion c2 set schema missing_schema;
       ╰╴                 ━━
    error: schema `missing_schema` does not exist
       ╭▸ 
    18 │ alter conversion c2 set schema missing_schema;
       ╰╴                               ━━━━━━━━━━━━━━
    error: conversion `c2` already exists
       ╭▸ 
    11 │ alter conversion c rename to c2;
       ╰╴                             ━━
    error: conversion `c4` already exists in schema `s`
       ╭▸ 
    17 │ alter conversion c4 set schema s;
       ╰╴                               ━
    ");
}

#[test]
fn drop_conversion() {
    assert_snapshot!(check("
create function f(int, int, cstring, internal, int, bool) returns int language c as 'f';
create conversion c for 'LATIN1' to 'UTF8' from f;
drop conversion c;
drop conversion c;
drop conversion if exists c;
drop conversion pg_catalog.utf8_to_iso_8859_1;
drop conversion utf8_to_iso_8859_1;
create conversion c for 'LATIN1' to 'UTF8' from f;
"), @"
    error: conversion `c` does not exist
      ╭▸ 
    5 │ drop conversion c;
      ╰╴                ━
    error: conversion `utf8_to_iso_8859_1` does not exist
      ╭▸ 
    8 │ drop conversion utf8_to_iso_8859_1;
      ╰╴                ━━━━━━━━━━━━━━━━━━
    ");
}

#[test]
fn conversion_rollback() {
    assert_snapshot!(check("
create function f(int, int, cstring, internal, int, bool) returns int language c as 'f';
create conversion c for 'LATIN1' to 'UTF8' from f;
begin;
drop conversion c;
create conversion c2 for 'LATIN1' to 'UTF8' from f;
rollback;
drop conversion c;
drop conversion c2;
"), @"
    error: conversion `c2` does not exist
      ╭▸ 
    9 │ drop conversion c2;
      ╰╴                ━━
    ");
}
