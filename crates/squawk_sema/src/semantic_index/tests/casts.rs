use insta::assert_snapshot;

use super::check;

#[test]
fn builtin_implicit_casts() {
    assert_snapshot!(check("
create function f(x bigint) returns int language sql return 1;
select f(1), f(1.5);
"), @"
    error: argument 1 of `f` has type `numeric`, expected `int8`
      ╭▸ 
    3 │ select f(1), f(1.5);
      │                ━━━
      │
      ╰ hint: consider adding explicit type casts
    ");
}

#[test]
fn user_defined_cast() {
    assert_snapshot!(check("
create type a;
create type b;
create function to_b(a) returns b language sql return null;
create function f(b) returns int language sql return 1;
create table t (x a);
select f(x) from t;
create cast (a as b) with function to_b(a) as assignment;
select f(x) from t;
drop cast (a as b);
create cast (a as b) with function to_b(a) as implicit;
select f(x) from t;
drop cast (a as b);
select f(x) from t;
"), @"
    error: argument 1 of `f` has type `a`, expected `b`
      ╭▸ 
    7 │ select f(x) from t;
      │          ━
      │
      ╰ hint: consider adding explicit type casts
    error: argument 1 of `f` has type `a`, expected `b`
      ╭▸ 
    9 │ select f(x) from t;
      │          ━
      │
      ╰ hint: consider adding explicit type casts
    error: argument 1 of `f` has type `a`, expected `b`
       ╭▸ 
    14 │ select f(x) from t;
       │          ━
       │
       ╰ hint: consider adding explicit type casts
    ");
}

#[test]
fn rollback_cast() {
    assert_snapshot!(check("
create type a;
create type b;
create function to_b(a) returns b language sql return null;
create function f(b) returns int language sql return 1;
create table t (x a);
begin;
create cast (a as b) with function to_b(a) as implicit;
select f(x) from t;
rollback;
select f(x) from t;
create cast (a as b) with function to_b(a) as implicit;
begin;
drop cast (a as b);
select f(x) from t;
rollback;
select f(x) from t;
"), @"
    error: argument 1 of `f` has type `a`, expected `b`
       ╭▸ 
    11 │ select f(x) from t;
       │          ━
       │
       ╰ hint: consider adding explicit type casts
    error: argument 1 of `f` has type `a`, expected `b`
       ╭▸ 
    15 │ select f(x) from t;
       │          ━
       │
       ╰ hint: consider adding explicit type casts
    ");
}

#[test]
fn cast_missing() {
    assert_snapshot!(check("
create function f(x int) returns text language sql return 'a';
create cast (nope as text) with inout;
create cast (int as text) with function f(text);
create cast (int as text) with function g(int);
create cast (int as text) with function f(int, int);
create cast (int as text) with function f(int);
create cast (int as text) without function;
drop cast (int as nope);
drop cast if exists (int as nope);
"), @"
    error: type `nope` does not exist
      ╭▸ 
    3 │ create cast (nope as text) with inout;
      ╰╴             ━━━━
    error: function `f(text)` does not exist
      ╭▸ 
    4 │ create cast (int as text) with function f(text);
      ╰╴                                        ━
    error: function `g(int4)` does not exist
      ╭▸ 
    5 │ create cast (int as text) with function g(int);
      │                                         ━
      ╰╴
    help: a function with a similar name exists
      ╭╴
    5 - create cast (int as text) with function g(int);
    5 + create cast (int as text) with function f(int);
      ╰╴
    error: function `f(int4, int4)` does not exist
      ╭▸ 
    6 │ create cast (int as text) with function f(int, int);
      ╰╴                                        ━
    error: type `nope` does not exist
      ╭▸ 
    9 │ drop cast (int as nope);
      ╰╴                  ━━━━
    ");
}

#[test]
fn drop_cast() {
    assert_snapshot!(check("
create domain d as int;
create function to_text(d) returns text language sql return 'x';
create cast (d as text) with function to_text(d);
drop cast (d as text);
drop cast (d as text);
drop cast if exists (d as text);
drop cast (int[] as text);
drop cast (int as bigint);
create cast (d as text) with function to_text(d);
begin;
drop cast (d as text);
rollback;
drop function to_text(d) cascade;
drop cast (d as text);
"), @"
    error: cast from type `d` to type `text` does not exist
      ╭▸ 
    6 │ drop cast (d as text);
      ╰╴          ━━━━━━━━━━━
    error: cast from type `int4[]` to type `text` does not exist
      ╭▸ 
    8 │ drop cast (int[] as text);
      ╰╴          ━━━━━━━━━━━━━━━
    error: cast from type `d` to type `text` does not exist
       ╭▸ 
    15 │ drop cast (d as text);
       ╰╴          ━━━━━━━━━━━
    ");
}

#[test]
fn empty_array_without_cast() {
    assert_snapshot!(check("
create table t (a int[]);
create domain ia as int[];
create function f(int[]) returns int[]
  language sql
  as 'select $1';

select array[];
insert into t values (array[]);
update t set a = array[];
select a = array[] from t;
select coalesce(a, array[]) from t;
select array_length(array[], 1);
select array[array[]];
select array[array[1], array[]];
select array[]::text;
select array[f(array[])]::int[][];
create table d (a int[] default array[]);

select array[]::int[];
select cast(array[] as int[]);
select (array[])::int[];
select array[array[]]::int[];
select array[(array[])]::int[];
select array[]::ia;
select array[]::unknown_type[];
select array(select 1);
select '{}'::int[];
"), @"
    error: cannot determine type of empty array
      ╭▸ 
    8 │ select array[];
      │        ━━━━━━━
      │
      ╰ hint: explicitly cast to the desired type, for example `array[]::integer[]`
    error: cannot determine type of empty array
      ╭▸ 
    9 │ insert into t values (array[]);
      │                       ━━━━━━━
      │
      ╰ hint: explicitly cast to the desired type, for example `array[]::integer[]`
    error: cannot determine type of empty array
       ╭▸ 
    10 │ update t set a = array[];
       │                  ━━━━━━━
       │
       ╰ hint: explicitly cast to the desired type, for example `array[]::integer[]`
    error: cannot determine type of empty array
       ╭▸ 
    11 │ select a = array[] from t;
       │            ━━━━━━━
       │
       ╰ hint: explicitly cast to the desired type, for example `array[]::integer[]`
    error: cannot determine type of empty array
       ╭▸ 
    12 │ select coalesce(a, array[]) from t;
       │                    ━━━━━━━
       │
       ╰ hint: explicitly cast to the desired type, for example `array[]::integer[]`
    error: cannot determine type of empty array
       ╭▸ 
    13 │ select array_length(array[], 1);
       │                     ━━━━━━━
       │
       ╰ hint: explicitly cast to the desired type, for example `array[]::integer[]`
    error: cannot determine type of empty array
       ╭▸ 
    14 │ select array[array[]];
       │              ━━━━━━━
       │
       ╰ hint: explicitly cast to the desired type, for example `array[]::integer[]`
    error: cannot determine type of empty array
       ╭▸ 
    15 │ select array[array[1], array[]];
       │                        ━━━━━━━
       │
       ╰ hint: explicitly cast to the desired type, for example `array[]::integer[]`
    error: cannot determine type of empty array
       ╭▸ 
    16 │ select array[]::text;
       │        ━━━━━━━
       │
       ╰ hint: explicitly cast to the desired type, for example `array[]::integer[]`
    error: cannot determine type of empty array
       ╭▸ 
    17 │ select array[f(array[])]::int[][];
       │                ━━━━━━━
       │
       ╰ hint: explicitly cast to the desired type, for example `array[]::integer[]`
    error: cannot determine type of empty array
       ╭▸ 
    18 │ create table d (a int[] default array[]);
       │                                 ━━━━━━━
       │
       ╰ hint: explicitly cast to the desired type, for example `array[]::integer[]`
    error: type `unknown_type` does not exist
       ╭▸ 
    26 │ select array[]::unknown_type[];
       ╰╴                ━━━━━━━━━━━━
    ");
}

#[test]
fn invalid_cast() {
    assert_snapshot!(check("
create type mood as enum ('a');
create domain ints as int[];
create table t (a int, b text, c int[], d bool, m mood, j json);
select a::int[] from t;
select cast(a as int[]) from t;
select a::text, b::int, c::text[], c::bigint[], d::int, m::text, b::mood, j::jsonb from t;
select c::ints, a::ints from t;
select d::date from t;
select a::mood from t;
select '{1}'::int[], null::int[], array[1]::text[];
select row(1)::t from t;
select t::text from t;
"), @"
    error: cannot cast type `int4` to `int4[]`
      ╭▸ 
    5 │ select a::int[] from t;
      ╰╴       ━━━━━━━━
    error: cannot cast type `int4` to `int4[]`
      ╭▸ 
    6 │ select cast(a as int[]) from t;
      ╰╴       ━━━━━━━━━━━━━━━━
    error: cannot cast type `int4` to `ints`
      ╭▸ 
    8 │ select c::ints, a::ints from t;
      ╰╴                ━━━━━━━
    error: cannot cast type `bool` to `date`
      ╭▸ 
    9 │ select d::date from t;
      ╰╴       ━━━━━━━
    error: cannot cast type `int4` to `mood`
       ╭▸ 
    10 │ select a::mood from t;
       ╰╴       ━━━━━━━
    ");
}

#[test]
fn function_style_cast() {
    assert_snapshot!(check("
create table c (a int);
select text(1);
select int4('1'::text);
select regclass(1::oid);
select int4('1');
select jsonb('{}'::json);
select int4(point(1, 1));
select text(c) from c;
"), @"
    error: function `int4(point)` does not exist
      ╭▸ 
    8 │ select int4(point(1, 1));
      │        ━━━━
      │
      ├ note: `int4(float8)` expects `float8` for argument 1, found `point`
      ├ note: `int4(jsonb)` expects `jsonb` for argument 1, found `point`
      ├ note: `int4(numeric)` expects `numeric` for argument 1, found `point`
      ├ note: `int4(float4)` expects `float4` for argument 1, found `point`
      ├ note: `int4(int2)` expects `int2` for argument 1, found `point`
      ├ note: and 5 other candidates
      ╰ hint: consider adding explicit type casts
    error: function `text(c)` does not exist
      ╭▸ 
    9 │ select text(c) from c;
      │        ━━━━
      │
      ├ note: `text(bool)` expects `bool` for argument 1, found `c`
      ├ note: `text(bpchar)` expects `bpchar` for argument 1, found `c`
      ├ note: `text(inet)` expects `inet` for argument 1, found `c`
      ├ note: `text(name)` expects `name` for argument 1, found `c`
      ├ note: `text(xml)` expects `xml` for argument 1, found `c`
      ├ note: and 1 other candidate
      ╰ hint: consider adding explicit type casts
    ");
}

#[test]
fn treat() {
    assert_snapshot!(check("
select treat(-5 as abs);
select treat(1 as int4[]);
select treat(1 as text);
select treat(point(1, 1) as int4);
select array_length(treat(1 as int4[]), 1);
"), @"
    error: function `pg_catalog.int4(point)` does not exist
      ╭▸ 
    5 │ select treat(point(1, 1) as int4);
      │                             ━━━━
      │
      ├ note: `pg_catalog.int4(float8)` expects `float8` for argument 1, found `point`
      ├ note: `pg_catalog.int4(jsonb)` expects `jsonb` for argument 1, found `point`
      ├ note: `pg_catalog.int4(numeric)` expects `numeric` for argument 1, found `point`
      ├ note: `pg_catalog.int4(float4)` expects `float4` for argument 1, found `point`
      ├ note: `pg_catalog.int4(int2)` expects `int2` for argument 1, found `point`
      ├ note: and 5 other candidates
      ╰ hint: consider adding explicit type casts
    error: argument 1 of `array_length` has type `int4`, expected `anyarray`
      ╭▸ 
    6 │ select array_length(treat(1 as int4[]), 1);
      │                     ━━━━━━━━━━━━━━━━━━
      │
      ╰ hint: consider adding explicit type casts
    ");
}
