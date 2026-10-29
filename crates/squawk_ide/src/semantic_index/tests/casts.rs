use insta::assert_snapshot;

use super::check;

#[test]
fn builtin_implicit_casts() {
    assert_snapshot!(check("
create function f(x bigint) returns int language sql return 1;
select f(1), f(1.5);
"), @"
    error: function `f(numeric)` does not exist
      ╭▸ 
    3 │ select f(1), f(1.5);
      │              ━
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
    error: function `f(a)` does not exist
      ╭▸ 
    7 │ select f(x) from t;
      │        ━
      │
      ╰ hint: consider adding explicit type casts
    error: function `f(a)` does not exist
      ╭▸ 
    9 │ select f(x) from t;
      │        ━
      │
      ╰ hint: consider adding explicit type casts
    error: function `f(a)` does not exist
       ╭▸ 
    14 │ select f(x) from t;
       │        ━
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
    error: function `f(a)` does not exist
       ╭▸ 
    11 │ select f(x) from t;
       │        ━
       │
       ╰ hint: consider adding explicit type casts
    error: function `f(a)` does not exist
       ╭▸ 
    15 │ select f(x) from t;
       │        ━
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
