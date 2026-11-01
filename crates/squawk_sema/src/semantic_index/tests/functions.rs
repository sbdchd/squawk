use insta::assert_snapshot;

use super::check;

#[test]
fn create_function_params() {
    assert_snapshot!(check("
create function add(a int, b int) returns int
  language sql
  return a + b + c;
"), @"
    error: column `c` does not exist
      ╭▸ 
    4 │   return a + b + c;
      ╰╴                 ━
    ");
}

#[test]
fn create_function_positional_params() {
    assert_snapshot!(check("
create function f(int, int) returns int
  language sql
  return $1 + $2 + $3;
"), @"
    error: there is no parameter `$3`
      ╭▸ 
    4 │   return $1 + $2 + $3;
      ╰╴                   ━━
    ");
}

#[test]
fn create_function_atomic_body() {
    assert_snapshot!(check("
create table users (id int, name text);
create function user_name(user_id int) returns text
  language sql
begin atomic
  select name from users where id = user_id and email = user_name.user_id;
end;
"), @"
    error: column `email` does not exist
      ╭▸ 
    6 │   select name from users where id = user_id and email = user_name.user_id;
      ╰╴                                                ━━━━━
    ");
}

#[test]
fn create_function_qualified_param() {
    assert_snapshot!(check("
create function f(a int) returns int
  language sql
  return f.a + f.b;
"), @"
    error: there is no parameter `b`
      ╭▸ 
    4 │   return f.a + f.b;
      ╰╴                 ━
    ");
}

#[test]
fn create_function_column_shadows_param() {
    assert_snapshot!(check("
create table t (a int);
create function f(a int, b int) returns setof int
  language sql
begin atomic
  select a + b from t;
end;
"), @"no diagnostics");
}

#[test]
fn create_function_returns_table() {
    assert_snapshot!(check("
create function f() returns table (x int, y text)
  language sql
  return null;
select x, y, z from f();
select t.x, t.z from f() as t;
"), @"
    error: column `z` does not exist
      ╭▸ 
    5 │ select x, y, z from f();
      │              ━
      ╰╴
    help: a column with a similar name exists
      ╭╴
    5 - select x, y, z from f();
    5 + select x, y, x from f();
      ╰╴
    error: column `t.z` does not exist
      ╭▸ 
    6 │ select t.x, t.z from f() as t;
      │               ━
      ╰╴
    help: a column with a similar name exists
      ╭╴
    6 - select t.x, t.z from f() as t;
    6 + select t.x, t.x from f() as t;
      ╰╴
    ");
}

#[test]
fn create_function_out_params() {
    assert_snapshot!(check("
create function f(in a int, out x int, inout y int) returns record
  language sql
  return null;
select x, y, a from f(1, 2);
"), @"
    error: column `a` does not exist
      ╭▸ 
    5 │ select x, y, a from f(1, 2);
      │              ━
      ╰╴
    help: a column with a similar name exists
      ╭╴
    5 - select x, y, a from f(1, 2);
    5 + select x, y, x from f(1, 2);
      ╰╴
    ");
}

#[test]
fn unknown_function() {
    assert_snapshot!(check("
select foo(1), x from bar(2);
"), @"
    error: function `foo(int4)` does not exist
      ╭▸ 
    2 │ select foo(1), x from bar(2);
      │        ━━━
      │
      ╰ hint: consider adding explicit type casts
    error: function `bar(int4)` does not exist
      ╭▸ 
    2 │ select foo(1), x from bar(2);
      │                       ━━━
      │
      ╰ hint: consider adding explicit type casts
    ");
}

#[test]
fn named_args_are_not_columns() {
    assert_snapshot!(check("
create function f(a int, b int) returns int language sql return a + b;
select f(a => 1, b => x), g(y => 1), f(zz => 1);
"), @"
    error: column `x` does not exist
      ╭▸ 
    3 │ select f(a => 1, b => x), g(y => 1), f(zz => 1);
      ╰╴                      ━
    error: function `g(y => int4)` does not exist
      ╭▸ 
    3 │ select f(a => 1, b => x), g(y => 1), f(zz => 1);
      │                           ━
      ╰╴
    help: a function with a similar name exists
      ╭╴
    3 - select f(a => 1, b => x), g(y => 1), f(zz => 1);
    3 + select f(a => 1, b => x), f(y => 1), f(zz => 1);
      ╰╴
    error: function `f` has no parameter named `zz`
      ╭▸ 
    3 │ select f(a => 1, b => x), g(y => 1), f(zz => 1);
      ╰╴                                       ━━
    ");
}

#[test]
fn named_arg_name_does_not_exist() {
    assert_snapshot!(check("
create function f(a int) returns int language sql return a;
create function g(int) returns int language sql return $1;
select f(z => 1), g(a => 1);
"), @"
    error: function `f` has no parameter named `z`
      ╭▸ 
    4 │ select f(z => 1), g(a => 1);
      ╰╴         ━
    error: function `g` has no parameter named `a`
      ╭▸ 
    4 │ select f(z => 1), g(a => 1);
      ╰╴                    ━
    ");
}

#[test]
fn unknown_schema_function() {
    assert_snapshot!(check("
select myschema.f(1), pg_catalog.nope();
"), @"
    error: schema `myschema` does not exist
      ╭▸ 
    2 │ select myschema.f(1), pg_catalog.nope();
      ╰╴       ━━━━━━━━
    error: function `myschema.f(int4)` does not exist
      ╭▸ 
    2 │ select myschema.f(1), pg_catalog.nope();
      │                 ━
      │
      ╰ hint: consider adding explicit type casts
    error: function `pg_catalog.nope()` does not exist
      ╭▸ 
    2 │ select myschema.f(1), pg_catalog.nope();
      ╰╴                                 ━━━━
    ");
}

#[test]
fn create_function_sql_body() {
    assert_snapshot!(check("
create table users (id int, name text);
create function user_name(user_id int) returns text
  language sql
  as $$
    select name from users where id = user_id and email = $1 and id = $2;
  $$;
create function f(a int) returns int
  as 'select a + b + f.c'
  language sql;
"), @"
    error: column `email` does not exist
      ╭▸ 
    6 │     select name from users where id = user_id and email = $1 and id = $2;
      ╰╴                                                  ━━━━━
    error: there is no parameter `$2`
      ╭▸ 
    6 │     select name from users where id = user_id and email = $1 and id = $2;
      ╰╴                                                                      ━━
    error: column `b` does not exist
      ╭▸ 
    9 │   as 'select a + b + f.c'
      ╰╴                 ━
    error: there is no parameter `c`
      ╭▸ 
    9 │   as 'select a + b + f.c'
      ╰╴                       ━
    ");
}

#[test]
fn create_function_sql_body_escaped_string() {
    assert_snapshot!(check("
create table t (a text);
create function f() returns text
  as 'select ''x'' || a || b from t'
  language sql;
create function g() returns text
  as E'select\\n''x'' || nope'
  language sql;
"), @r"
    error: column `b` does not exist
      ╭▸ 
    4 │   as 'select ''x'' || a || b from t'
      │                            ━
      ╰╴
    help: a column with a similar name exists
      ╭╴
    4 -   as 'select ''x'' || a || b from t'
    4 +   as 'select ''x'' || a || a from t'
      ╰╴
    error: column `nope` does not exist
      ╭▸ 
    7 │   as E'select\n''x'' || nope'
      ╰╴                        ━━━━
    ");
}

#[test]
fn create_function_sql_body_multiple_stmts() {
    assert_snapshot!(check("
create function f() returns int
  language sql
  as $body$
    select x from nope;
    with t as (select 1 a) select a, b from t;
  $body$;
"), @"
    error: relation `nope` does not exist
      ╭▸ 
    5 │     select x from nope;
      ╰╴                  ━━━━
    error: column `b` does not exist
      ╭▸ 
    6 │     with t as (select 1 a) select a, b from t;
      │                                      ━
      ╰╴
    help: a column with a similar name exists
      ╭╴
    6 -     with t as (select 1 a) select a, b from t;
    6 +     with t as (select 1 a) select a, a from t;
      ╰╴
    ");
}

#[test]
fn create_function_sql_body_nested() {
    assert_snapshot!(check("
create function f() returns int
  language sql
  as $outer$
    create function g(a int) returns int
      language sql
      as $$ select a + b $$;
  $outer$;
"), @"
    error: column `b` does not exist
      ╭▸ 
    7 │       as $$ select a + b $$;
      ╰╴                       ━
    ");
}

#[test]
fn create_function_non_sql_body() {
    assert_snapshot!(check("
create function f() returns int
  language plpgsql
  as $$ select x from nope; $$;
"), @"no diagnostics");
}

#[test]
fn rollback_create_function() {
    assert_snapshot!(check("
begin;
create function f() returns int as 'select 1' language sql;
rollback;
select f();
"), @"
    error: function `f()` does not exist
      ╭▸ 
    5 │ select f();
      ╰╴       ━
    ");
}

#[test]
fn param_default_missing() {
    assert_snapshot!(check("
create function f(a int default nope(), b int = 1) returns int language sql return a;
"), @"
    error: function `nope()` does not exist
      ╭▸ 
    2 │ create function f(a int default nope(), b int = 1) returns int language sql return a;
      ╰╴                                ━━━━
    ");
}

#[test]
fn duplicate_function() {
    assert_snapshot!(check("
create function f(a int) returns int as 'select 1' language sql;
create function f(b int) returns text as 'select 1' language sql;
create function f(a text) returns int as 'select 1' language sql;
create function f(a int, out b int) as 'select 1' language sql;
create or replace function f(a int) returns int as 'select 1' language sql;
create function lower(text) returns text as 'select 1' language sql;
"), @"
    error: function `f` already exists with same argument types
      ╭▸ 
    3 │ create function f(b int) returns text as 'select 1' language sql;
      ╰╴                ━
    error: function `f` already exists with same argument types
      ╭▸ 
    5 │ create function f(a int, out b int) as 'select 1' language sql;
      ╰╴                ━
    ");
}

#[test]
fn create_procedure_params() {
    assert_snapshot!(check("
create table t (a int);
create procedure p(x int)
  language sql
begin atomic
  insert into t (a) values (x + y);
end;
"), @"
    error: column `y` does not exist
      ╭▸ 
    6 │   insert into t (a) values (x + y);
      ╰╴                                ━
    ");
}

#[test]
fn create_procedure_string_body() {
    assert_snapshot!(check("
create table t (a int);
create procedure p(x int) language sql as 'insert into t (b) values (x)';
"), @"
    error: column `t.b` does not exist
      ╭▸ 
    3 │ create procedure p(x int) language sql as 'insert into t (b) values (x)';
      │                                                           ━
      ╰╴
    help: a column with a similar name exists
      ╭╴
    3 - create procedure p(x int) language sql as 'insert into t (b) values (x)';
    3 + create procedure p(x int) language sql as 'insert into t (a) values (x)';
      ╰╴
    ");
}

#[test]
fn duplicate_procedure() {
    assert_snapshot!(check("
create procedure p(a int) language sql as 'select 1';
create procedure p(b int) language sql as 'select 1';
create function p(a int) returns int language sql as 'select 1';
create procedure p(a text) language sql as 'select 1';
create or replace procedure p(a int) language sql as 'select 1';
"), @"
    error: function `p` already exists with same argument types
      ╭▸ 
    3 │ create procedure p(b int) language sql as 'select 1';
      ╰╴                 ━
    error: function `p` already exists with same argument types
      ╭▸ 
    4 │ create function p(a int) returns int language sql as 'select 1';
      ╰╴                ━
    ");
}

#[test]
fn procedure_not_callable_in_expr() {
    assert_snapshot!(check("
create procedure p(a int) language sql as 'select 1';
select p(1);
select public.p(a => 1);
select p(1) + 1;
select p(1), 2;
select p(1) as x;
select * from p(1);
with c as (select 1) select p(1);
select (select p(1));
select p('a', 'b');
create function f() returns void language sql as $$ select p(1) $$;
"), @"
    error: `p(int4)` is a procedure
      ╭▸ 
    3 │ select p(1);
      │        ━
      ╰╴
    help: to call a procedure, use `call`
      ╭╴
    3 - select p(1);
    3 + call p(1);
      ╰╴
    error: `public.p(a => int4)` is a procedure
      ╭▸ 
    4 │ select public.p(a => 1);
      │               ━
      ╰╴
    help: to call a procedure, use `call`
      ╭╴
    4 - select public.p(a => 1);
    4 + call public.p(a => 1);
      ╰╴
    error: `p(int4)` is a procedure
      ╭▸ 
    5 │ select p(1) + 1;
      │        ━
      │
      ╰ hint: to call a procedure, use `call`
    error: `p(int4)` is a procedure
      ╭▸ 
    6 │ select p(1), 2;
      │        ━
      │
      ╰ hint: to call a procedure, use `call`
    error: `p(int4)` is a procedure
      ╭▸ 
    7 │ select p(1) as x;
      │        ━
      │
      ╰ hint: to call a procedure, use `call`
    error: `p(int4)` is a procedure
      ╭▸ 
    8 │ select * from p(1);
      │               ━
      │
      ╰ hint: to call a procedure, use `call`
    error: `p(int4)` is a procedure
      ╭▸ 
    9 │ with c as (select 1) select p(1);
      │                             ━
      │
      ╰ hint: to call a procedure, use `call`
    error: `p(int4)` is a procedure
       ╭▸ 
    10 │ select (select p(1));
       │                ━
       │
       ╰ hint: to call a procedure, use `call`
    error: function `p(unknown, unknown)` does not exist
       ╭▸ 
    11 │ select p('a', 'b');
       │        ━
       ╰╴
    help: a function with a similar name exists
       ╭╴
    11 │ select pi('a', 'b');
       ╰╴        +
    error: `p(int4)` is a procedure
       ╭▸ 
    12 │ create function f() returns void language sql as $$ select p(1) $$;
       │                                                            ━
       ╰╴
    help: to call a procedure, use `call`
       ╭╴
    12 - create function f() returns void language sql as $$ select p(1) $$;
    12 + create function f() returns void language sql as $$ call p(1) $$;
       ╰╴
    ");
}

#[test]
fn drop_function() {
    assert_snapshot!(check("
create function f(int) returns int language sql return 1;
create function g() returns int language sql return 1;
create procedure p() language sql begin atomic select 1; end;
drop function f(int), g;
select f(1), g();
drop function f(int);
drop function if exists f(int);
drop function p();
drop function nope;
"), @"
    error: function `f(int4)` does not exist
      ╭▸ 
    6 │ select f(1), g();
      │        ━
      │
      ╰ hint: consider adding explicit type casts
    error: function `g()` does not exist
      ╭▸ 
    6 │ select f(1), g();
      ╰╴             ━
    error: function `f(int4)` does not exist
      ╭▸ 
    7 │ drop function f(int);
      ╰╴              ━
    error: function `p()` does not exist
      ╭▸ 
    9 │ drop function p();
      │               ━
      ╰╴
    help: a function with a similar name exists
      ╭╴
    9 │ drop function pi();
      ╰╴               +
    error: function `nope` does not exist
       ╭▸ 
    10 │ drop function nope;
       ╰╴              ━━━━
    ");
}

#[test]
fn drop_function_rollback() {
    assert_snapshot!(check("
create function f() returns int language sql return 1;
begin;
drop function f;
rollback;
select f();
"), @"no diagnostics");
}

#[test]
fn drop_function_not_unique() {
    assert_snapshot!(check("
create function f(int) returns int language sql return 1;
create function f(text) returns int language sql return 1;
drop function f;
drop function f(text);
drop function f;
select f(1);
"), @"
    error: function `f` is not unique
      ╭▸ 
    4 │ drop function f;
      ╰╴              ━
    error: function `f(int4)` does not exist
      ╭▸ 
    7 │ select f(1);
      │        ━
      │
      ╰ hint: consider adding explicit type casts
    ");
}

#[test]
fn drop_procedure_and_routine() {
    assert_snapshot!(check("
create procedure p(int) language sql begin atomic select 1; end;
create function f() returns int language sql return 1;
drop procedure f();
drop procedure p(int);
call p(1);
drop routine f;
select f();
drop routine if exists f;
drop procedure if exists p;
"), @"
    error: procedure `f()` does not exist
      ╭▸ 
    4 │ drop procedure f();
      │                ━
      ╰╴
    help: a procedure with a similar name exists
      ╭╴
    4 - drop procedure f();
    4 + drop procedure p();
      ╰╴
    error: procedure `p(int4)` does not exist
      ╭▸ 
    6 │ call p(1);
      │      ━
      │
      ╰ hint: consider adding explicit type casts
    error: function `f()` does not exist
      ╭▸ 
    8 │ select f();
      ╰╴       ━
    ");
}

#[test]
fn create_or_replace_function_then_drop() {
    assert_snapshot!(check("
create function f() returns int language sql return 1;
create or replace function f() returns int language sql return 2;
drop function f();
select f();
"), @"
    error: function `f()` does not exist
      ╭▸ 
    5 │ select f();
      ╰╴       ━
    ");
}

#[test]
fn alter_function_rename() {
    assert_snapshot!(check("
create function f(int) returns int language sql return 1;
create function g(int) returns int language sql return 1;
alter function f(int) rename to h;
select f(1);
select h(1);
alter function h rename to g;
alter function nope(int) rename to x;
"), @"
    error: function `f(int4)` does not exist
      ╭▸ 
    5 │ select f(1);
      │        ━
      ╰╴
    help: a function with a similar name exists
      ╭╴
    5 - select f(1);
    5 + select h(1);
      ╰╴
    error: function `g` already exists with same argument types in schema `public`
      ╭▸ 
    7 │ alter function h rename to g;
      ╰╴                           ━
    error: function `nope(int4)` does not exist
      ╭▸ 
    8 │ alter function nope(int) rename to x;
      ╰╴               ━━━━
    ");
}

#[test]
fn alter_function_set_schema() {
    assert_snapshot!(check("
create schema s;
create function f() returns int language sql return 1;
alter function f() set schema s;
select f();
select s.f();
alter function s.f() set schema nope;
"), @"
    error: function `f()` does not exist
      ╭▸ 
    5 │ select f();
      │        ━
      │
      ╰ note: a function named `f` exists in schema `s`, which is not in the `search_path`
    help: qualify the call with schema `s`
      ╭╴
    5 │ select s.f();
      ╰╴       ++
    error: schema `nope` does not exist
      ╭▸ 
    7 │ alter function s.f() set schema nope;
      ╰╴                                ━━━━
    ");
}

#[test]
fn alter_function_options() {
    assert_snapshot!(check("
create function f() returns int language sql return 1;
alter function f() owner to nope;
alter function f() depends on extension nope;
alter function f() no depends on extension nope;
alter function f() support nope;
alter function f() immutable strict;
"), @"
    error: role `nope` does not exist
      ╭▸ 
    3 │ alter function f() owner to nope;
      ╰╴                            ━━━━
    error: extension `nope` does not exist
      ╭▸ 
    4 │ alter function f() depends on extension nope;
      ╰╴                                        ━━━━
    error: extension `nope` does not exist
      ╭▸ 
    5 │ alter function f() no depends on extension nope;
      ╰╴                                           ━━━━
    error: function `nope` does not exist
      ╭▸ 
    6 │ alter function f() support nope;
      ╰╴                           ━━━━
    ");
}

#[test]
fn alter_procedure_and_routine() {
    assert_snapshot!(check("
create procedure p() language sql begin atomic select 1; end;
alter procedure p() rename to q;
call p();
call q();
alter routine q rename to r;
call r();
alter procedure nope() security definer;
alter routine nope() security definer;
"), @"
    error: procedure `p()` does not exist
      ╭▸ 
    4 │ call p();
      │      ━
      ╰╴
    help: a procedure with a similar name exists
      ╭╴
    4 - call p();
    4 + call q();
      ╰╴
    error: procedure `nope()` does not exist
      ╭▸ 
    8 │ alter procedure nope() security definer;
      ╰╴                ━━━━
    error: function `nope()` does not exist
      ╭▸ 
    9 │ alter routine nope() security definer;
      ╰╴              ━━━━
    ");
}

#[test]
fn call_procedure() {
    assert_snapshot!(check("
create procedure p(a int, b text default 'x') language sql begin atomic select 1; end;
create function f() returns int language sql return 1;
call p(1);
call p(1, 'y');
call p(a => 1, b => 'y');
call p(c => 1);
call f();
call public.f();
call f(1);
call count(1);
call nope();
call p((select x from nope));
"), @"
    error: procedure `p` has no parameter named `c`
      ╭▸ 
    7 │ call p(c => 1);
      ╰╴       ━
    error: `f()` is not a procedure
      ╭▸ 
    8 │ call f();
      │      ━
      ╰╴
    help: to call a function, use `select`
      ╭╴
    8 - call f();
    8 + select f();
      ╰╴
    error: `public.f()` is not a procedure
      ╭▸ 
    9 │ call public.f();
      │             ━
      ╰╴
    help: to call a function, use `select`
      ╭╴
    9 - call public.f();
    9 + select public.f();
      ╰╴
    error: procedure `f(int4)` does not exist
       ╭▸ 
    10 │ call f(1);
       │      ━
       ╰╴
    help: a procedure with a similar name exists
       ╭╴
    10 - call f(1);
    10 + call p(1);
       ╰╴
    error: `count(int4)` is not a procedure
       ╭▸ 
    11 │ call count(1);
       │      ━━━━━
       ╰╴
    help: to call a function, use `select`
       ╭╴
    11 - call count(1);
    11 + select count(1);
       ╰╴
    error: procedure `nope()` does not exist
       ╭▸ 
    12 │ call nope();
       ╰╴     ━━━━
    error: cannot use subquery in `call` argument
       ╭▸ 
    13 │ call p((select x from nope));
       ╰╴        ━━━━━━━━━━━━━━━━━━
    error: relation `nope` does not exist
       ╭▸ 
    13 │ call p((select x from nope));
       ╰╴                      ━━━━
    ");
}

#[test]
fn call_procedure_out_param() {
    assert_snapshot!(check("
create procedure p(a int, out b int) language sql begin atomic select 1; end;
call p(1, null);
call p(1);
"), @"
    error: procedure `p` takes 2 arguments but 1 was supplied
      ╭▸ 
    4 │ call p(1);
      │      ━
      │
      ╰ note: missing `b`
    ");
}

#[test]
fn percent_type_param() {
    assert_snapshot!(check("
create table t (a int, b text);
create function f(x t.a%type) returns t.b%type language sql as 'select null';
drop function f(int);
create function h(x public.t.a%type) returns int language sql as 'select 1';
drop function h(int);
create function k(x t.nope%type) returns int language sql as 'select 1';
create function m(x nope.a%type) returns int language sql as 'select 1';
"), @"
    error: column `t.nope` does not exist
      ╭▸ 
    7 │ create function k(x t.nope%type) returns int language sql as 'select 1';
      ╰╴                      ━━━━
    error: relation `nope` does not exist
      ╭▸ 
    8 │ create function m(x nope.a%type) returns int language sql as 'select 1';
      ╰╴                    ━━━━
    ");
}

#[test]
fn procedure_out_params_in_signature() {
    assert_snapshot!(check("
create procedure p(a int, out b int) language sql as 'select 1';
create procedure p(a int) language sql as 'select 1';
create procedure p(a int, out b int) language sql as 'select 1';
create procedure p(a int, inout b int) language sql as 'select 1';
create function f(a int, out b int) language sql as 'select 1';
create function f(a int) returns int language sql as 'select 1';
"), @"
    error: function `p` already exists with same argument types
      ╭▸ 
    4 │ create procedure p(a int, out b int) language sql as 'select 1';
      ╰╴                 ━
    error: function `p` already exists with same argument types
      ╭▸ 
    5 │ create procedure p(a int, inout b int) language sql as 'select 1';
      ╰╴                 ━
    error: function `f` already exists with same argument types
      ╭▸ 
    7 │ create function f(a int) returns int language sql as 'select 1';
      ╰╴                ━
    ");
}

#[test]
fn drop_function_with_missing_arg_type() {
    assert_snapshot!(check("
create function add(bigint) returns bigint as $$ select 1 $$ language sql;
drop function add(complex);
"), @"
    error: type `complex` does not exist
      ╭▸ 
    3 │ drop function add(complex);
      ╰╴                  ━━━━━━━
    ");
}

#[test]
fn duplicate_param_name() {
    assert_snapshot!(check("
create function f(a int, a int) returns int language sql as 'select 1';
create function g(inout a int, out a int) language sql as 'select 1';
create function h(a int, out a int) language sql as 'select 1';
create function i(variadic a int[], out a int) language sql as 'select 1';
create function j(a int) returns table (a int) language sql as 'select 1';
create function sfn(int, int, int) returns int language sql as 'select 1';
create aggregate k(a int, a int) (sfunc = sfn, stype = int);
"), @"
    error: parameter name `a` used more than once
      ╭▸ 
    2 │ create function f(a int, a int) returns int language sql as 'select 1';
      ╰╴                         ━
    error: parameter name `a` used more than once
      ╭▸ 
    3 │ create function g(inout a int, out a int) language sql as 'select 1';
      ╰╴                                   ━
    error: parameter name `a` used more than once
      ╭▸ 
    8 │ create aggregate k(a int, a int) (sfunc = sfn, stype = int);
      ╰╴                          ━
    ");
}

#[test]
fn unresolved_function_arg_types() {
    assert_snapshot!(check("
create function foo(x int) returns int as 'select 1' language sql;
create function bar(a int, b int default 1) returns int as 'select 1' language sql;
create procedure p(int) as 'select 1' language sql;
select foo(1.5::float8);
select foo(1, 'a');
select foo();
select bar();
select bar(1, 2, 3);
select bar(1, 'a'::text);
select foo(y => 1);
select public.foo(variadic array[1]);
select nope();
select lower(1);
call p('a'::text);
"), @"
    error: argument 1 of `foo` has type `float8`, expected `int4`
      ╭▸ 
    5 │ select foo(1.5::float8);
      │            ━━━━━━━━━━━
      │
      ╰ hint: consider adding explicit type casts
    error: function `foo` takes 1 argument but 2 were supplied
      ╭▸ 
    6 │ select foo(1, 'a');
      ╰╴       ━━━
    error: function `foo` takes 1 argument but 0 were supplied
      ╭▸ 
    7 │ select foo();
      │        ━━━
      │
      ╰ note: missing `x`
    error: function `bar` takes at least 1 argument but 0 were supplied
      ╭▸ 
    8 │ select bar();
      │        ━━━
      │
      ╰ note: missing `a`
    error: function `bar` takes at most 2 arguments but 3 were supplied
      ╭▸ 
    9 │ select bar(1, 2, 3);
      ╰╴       ━━━
    error: argument 2 of `bar` has type `text`, expected `int4`
       ╭▸ 
    10 │ select bar(1, 'a'::text);
       │               ━━━━━━━━━
       │
       ╰ hint: consider adding explicit type casts
    error: function `foo` has no parameter named `y`
       ╭▸ 
    11 │ select foo(y => 1);
       ╰╴           ━
    error: argument 1 of `public.foo` is labeled `variadic` but its parameter is not variadic
       ╭▸ 
    12 │ select public.foo(variadic array[1]);
       ╰╴                  ━━━━━━━━━━━━━━━━━
    error: function `nope()` does not exist
       ╭▸ 
    13 │ select nope();
       ╰╴       ━━━━
    error: function `lower(int4)` does not exist
       ╭▸ 
    14 │ select lower(1);
       │        ━━━━━
       │
       ├ note: candidates are `lower(anymultirange)`, `lower(anyrange)`, `lower(text)`
       ╰ hint: consider adding explicit type casts
    error: argument 1 of `p` has type `text`, expected `int4`
       ╭▸ 
    15 │ call p('a'::text);
       │        ━━━━━━━━━
       │
       ╰ hint: consider adding explicit type casts
    ");
}

#[test]
fn unresolved_function_signature() {
    assert_snapshot!(check("
create function g(int) returns int as 'select 1' language sql;
drop function g(text);
drop function g(x int, out y int);
drop function nope(variadic int[]);
drop procedure nope(int);
drop aggregate nope(int order by text);
drop aggregate nope(*);
drop routine nope(int);
alter function g(text) rename to h;
comment on function public.g(text) is 'x';
drop operator === (int, int);
drop operator ~~~ (none, int);
"), @"
    error: function `g(text)` does not exist
      ╭▸ 
    3 │ drop function g(text);
      ╰╴              ━
    error: function `nope(int4[])` does not exist
      ╭▸ 
    5 │ drop function nope(variadic int[]);
      ╰╴              ━━━━
    error: procedure `nope(int4)` does not exist
      ╭▸ 
    6 │ drop procedure nope(int);
      ╰╴               ━━━━
    error: aggregate `nope(int4, text)` does not exist
      ╭▸ 
    7 │ drop aggregate nope(int order by text);
      ╰╴               ━━━━
    error: aggregate `nope(*)` does not exist
      ╭▸ 
    8 │ drop aggregate nope(*);
      ╰╴               ━━━━
    error: function `nope(int4)` does not exist
      ╭▸ 
    9 │ drop routine nope(int);
      ╰╴             ━━━━
    error: function `g(text)` does not exist
       ╭▸ 
    10 │ alter function g(text) rename to h;
       ╰╴               ━
    error: function `public.g(text)` does not exist
       ╭▸ 
    11 │ comment on function public.g(text) is 'x';
       ╰╴                           ━
    error: operator does not exist: `int4 === int4`
       ╭▸ 
    12 │ drop operator === (int, int);
       ╰╴              ━━━
    error: operator does not exist: `~~~ int4`
       ╭▸ 
    13 │ drop operator ~~~ (none, int);
       ╰╴              ━━━
    ");
}

#[test]
fn similar_qualified_function() {
    assert_snapshot!(check("
create schema app;
create function app.do_thing(int) returns int as 'select 1' language sql;
create procedure app.do_proc() as 'select 1' language sql;
select app.do_thnig(1);
select pg_catalog.json_bild_object('a', 1);
call app.do_proc_();
drop function app.do_thnig(int);
"), @"
    error: function `app.do_thnig(int4)` does not exist
      ╭▸ 
    5 │ select app.do_thnig(1);
      │            ━━━━━━━━
      ╰╴
    help: a function with a similar name exists
      ╭╴
    5 - select app.do_thnig(1);
    5 + select app.do_thing(1);
      ╰╴
    error: function `pg_catalog.json_bild_object(unknown, int4)` does not exist
      ╭▸ 
    6 │ select pg_catalog.json_bild_object('a', 1);
      │                   ━━━━━━━━━━━━━━━━
      ╰╴
    help: a function with a similar name exists
      ╭╴
    6 │ select pg_catalog.json_build_object('a', 1);
      ╰╴                        +
    error: procedure `app.do_proc_()` does not exist
      ╭▸ 
    7 │ call app.do_proc_();
      │          ━━━━━━━━
      ╰╴
    help: a procedure with a similar name exists
      ╭╴
    7 - call app.do_proc_();
    7 + call app.do_proc();
      ╰╴
    error: function `app.do_thnig(int4)` does not exist
      ╭▸ 
    8 │ drop function app.do_thnig(int);
      │                   ━━━━━━━━
      ╰╴
    help: a function with a similar name exists
      ╭╴
    8 - drop function app.do_thnig(int);
    8 + drop function app.do_thing(int);
      ╰╴
    ");
}

#[test]
fn similar_unqualified_function() {
    assert_snapshot!(check("
create function do_thing(int) returns int as 'select 1' language sql;
select do_thnig(1);
"), @"
    error: function `do_thnig(int4)` does not exist
      ╭▸ 
    3 │ select do_thnig(1);
      │        ━━━━━━━━
      ╰╴
    help: a function with a similar name exists
      ╭╴
    3 - select do_thnig(1);
    3 + select do_thing(1);
      ╰╴
    ");
}

#[test]
fn similar_function_skips_existing_name() {
    assert_snapshot!(check("
create schema app;
create function app.do_thing(int) returns int as 'select 1' language sql;
create function app.do_thing2(int, int) returns int as 'select 1' language sql;
select app.do_thing(1, 2);
call app.do_thig();
"), @"
    error: function `app.do_thing` takes 1 argument but 2 were supplied
      ╭▸ 
    5 │ select app.do_thing(1, 2);
      ╰╴           ━━━━━━━━
    error: procedure `app.do_thig()` does not exist
      ╭▸ 
    6 │ call app.do_thig();
      ╰╴         ━━━━━━━
    ");
}

#[test]
fn invalid_named_args() {
    assert_snapshot!(check("
create function foo(x int, y int default 1) returns int language sql return x;
select foo(x => 1, 2);
select foo(x => 1, x => 2);
select nope(a => 1, 2);
"), @"
    error: positional argument cannot follow named argument
      ╭▸ 
    3 │ select foo(x => 1, 2);
      ╰╴                   ━
    error: argument name `x` used more than once
      ╭▸ 
    4 │ select foo(x => 1, x => 2);
      ╰╴                   ━
    error: positional argument cannot follow named argument
      ╭▸ 
    5 │ select nope(a => 1, 2);
      ╰╴                    ━
    ");
}

#[test]
fn named_variadic_requires_label() {
    assert_snapshot!(check("
create function v(a int, variadic b int[]) returns int language sql return a;
select v(a => 1, b => array[1]);
select v(a => 1, variadic b => array[1]);
select v(b => array[1], a => 1);
select v(a => 1, b => 2);
select v(a => 1, b => array['a']);
"), @"
    error: variadic parameter `b` of `v` must be labeled `variadic` when passed by name
      ╭▸ 
    3 │ select v(a => 1, b => array[1]);
      │                  ━━━━━━━━━━━━━
      ╰╴
    help: label the variadic array with `variadic`
      ╭╴
    3 │ select v(a => 1, variadic b => array[1]);
      ╰╴                 ++++++++
    error: variadic parameter `b` of `v` must be labeled `variadic` when passed by name
      ╭▸ 
    5 │ select v(b => array[1], a => 1);
      │          ━━━━━━━━━━━━━
      │
      ╰ hint: pass an array labeled `variadic` as the last argument
    error: variadic parameter `b` of `v` must be labeled `variadic` when passed by name
      ╭▸ 
    6 │ select v(a => 1, b => 2);
      │                  ━━━━━━
      │
      ╰ hint: pass an array labeled `variadic` as the last argument
    error: variadic parameter `b` of `v` must be labeled `variadic` when passed by name
      ╭▸ 
    7 │ select v(a => 1, b => array['a']);
      │                  ━━━━━━━━━━━━━━━
      │
      ╰ hint: pass an array labeled `variadic` as the last argument
    ");
}

#[test]
fn unknown_arg_name() {
    assert_snapshot!(check("
create function foo(x int, y int default 1) returns int language sql return x;
select foo(z => 1);
select foo(1, z => 2);
"), @"
    error: function `foo` has no parameter named `z`
      ╭▸ 
    3 │ select foo(z => 1);
      ╰╴           ━
    error: function `foo` has no parameter named `z`
      ╭▸ 
    4 │ select foo(1, z => 2);
      ╰╴              ━
    ");
}

#[test]
fn arg_positional_and_named() {
    assert_snapshot!(check("
create function foo(x int, y int default 1) returns int language sql return x;
select foo(1, x => 2);
select foo(1, 2, y => 3);
"), @"
    error: argument `x` of `foo` was specified both positionally and by name
      ╭▸ 
    3 │ select foo(1, x => 2);
      ╰╴              ━
    error: argument `y` of `foo` was specified both positionally and by name
      ╭▸ 
    4 │ select foo(1, 2, y => 3);
      ╰╴                 ━
    ");
}

#[test]
fn missing_required_args() {
    assert_snapshot!(check("
create function foo(x int, y int default 1) returns int language sql return x;
create function w(int, b int) returns int language sql return 1;
create function z(a int, b int, c int) returns int language sql return 1;
select foo(y => 2);
select w(b => 1);
select z(b => 1);
select z(1, c => 3);
select z(1);
select foo();
"), @"
    error: function `foo` is missing required argument `x`
      ╭▸ 
    5 │ select foo(y => 2);
      ╰╴       ━━━
    error: function `w` is missing required argument `$1`
      ╭▸ 
    6 │ select w(b => 1);
      ╰╴       ━
    error: function `z` is missing required arguments `a`, `c`
      ╭▸ 
    7 │ select z(b => 1);
      ╰╴       ━
    error: function `z` is missing required argument `b`
      ╭▸ 
    8 │ select z(1, c => 3);
      ╰╴       ━
    error: function `z` takes 3 arguments but 1 was supplied
      ╭▸ 
    9 │ select z(1);
      │        ━
      │
      ╰ note: missing `b`, `c`
    error: function `foo` takes at least 1 argument but 0 were supplied
       ╭▸ 
    10 │ select foo();
       │        ━━━
       │
       ╰ note: missing `x`
    ");
}

#[test]
fn named_arg_type_mismatch() {
    assert_snapshot!(check("
create function foo(x int, y int default 1) returns int language sql return x;
create table t (a text, b bool);
select foo(y => 'a'::text, x => 1);
select foo(1, y => 'a'::text);
select foo(x => true);
select foo(x => now());
select foo(x => array[1]);
select foo(x => a) from t;
select foo(1, y => b) from t;
"), @"
    error: argument `y` of `foo` has type `text`, expected `int4`
      ╭▸ 
    4 │ select foo(y => 'a'::text, x => 1);
      │            ━━━━━━━━━━━━━━
      │
      ╰ hint: consider adding explicit type casts
    error: argument `y` of `foo` has type `text`, expected `int4`
      ╭▸ 
    5 │ select foo(1, y => 'a'::text);
      │               ━━━━━━━━━━━━━━
      │
      ╰ hint: consider adding explicit type casts
    error: argument `x` of `foo` has type `bool`, expected `int4`
      ╭▸ 
    6 │ select foo(x => true);
      │            ━━━━━━━━━
      │
      ╰ hint: consider adding explicit type casts
    error: argument `x` of `foo` has type `timestamptz`, expected `int4`
      ╭▸ 
    7 │ select foo(x => now());
      │            ━━━━━━━━━━
      │
      ╰ hint: consider adding explicit type casts
    error: argument `x` of `foo` has type `int4[]`, expected `int4`
      ╭▸ 
    8 │ select foo(x => array[1]);
      │            ━━━━━━━━━━━━━
      │
      ╰ hint: consider adding explicit type casts
    error: argument `x` of `foo` has type `text`, expected `int4`
      ╭▸ 
    9 │ select foo(x => a) from t;
      │            ━━━━━━
      │
      ╰ hint: consider adding explicit type casts
    error: argument `y` of `foo` has type `bool`, expected `int4`
       ╭▸ 
    10 │ select foo(1, y => b) from t;
       │               ━━━━━━
       │
       ╰ hint: consider adding explicit type casts
    ");
}

#[test]
fn variadic_call_mismatches() {
    assert_snapshot!(check("
create function v(a int, variadic b int[]) returns int language sql return a;
create table t (x text);
select v(1);
select v(variadic array[1]);
select v(1, 2, 'a'::text);
select v(1, variadic array['a']);
select v(a => 1, variadic b => array['a']);
select v(1, 2, x) from t;
"), @"
    error: function `v` takes at least 2 arguments but 1 was supplied
      ╭▸ 
    4 │ select v(1);
      │        ━
      │
      ╰ note: missing `b`
    error: function `v` takes 2 arguments but 1 was supplied
      ╭▸ 
    5 │ select v(variadic array[1]);
      │        ━
      │
      ╰ note: missing `b`
    error: argument 3 of `v` has type `text`, expected `int4`
      ╭▸ 
    6 │ select v(1, 2, 'a'::text);
      │                ━━━━━━━━━
      │
      ╰ hint: consider adding explicit type casts
    error: argument 2 of `v` has type `text[]`, expected `int4[]`
      ╭▸ 
    7 │ select v(1, variadic array['a']);
      │             ━━━━━━━━━━━━━━━━━━━
      │
      ╰ hint: consider adding explicit type casts
    error: argument `b` of `v` has type `text[]`, expected `int4[]`
      ╭▸ 
    8 │ select v(a => 1, variadic b => array['a']);
      │                  ━━━━━━━━━━━━━━━━━━━━━━━━
      │
      ╰ hint: consider adding explicit type casts
    error: argument 3 of `v` has type `text`, expected `int4`
      ╭▸ 
    9 │ select v(1, 2, x) from t;
      │                ━
      │
      ╰ hint: consider adding explicit type casts
    ");
}

#[test]
fn variadic_on_non_variadic_param() {
    assert_snapshot!(check("
create function foo(x int) returns int language sql return x;
create function v(a int, variadic b int[]) returns int language sql return a;
select foo(variadic array[1]);
select foo(variadic x => array[1]);
select v(variadic a => 1);
"), @"
    error: argument 1 of `foo` is labeled `variadic` but its parameter is not variadic
      ╭▸ 
    4 │ select foo(variadic array[1]);
      ╰╴           ━━━━━━━━━━━━━━━━━
    error: argument `x` of `foo` is labeled `variadic` but its parameter is not variadic
      ╭▸ 
    5 │ select foo(variadic x => array[1]);
      ╰╴           ━━━━━━━━━━━━━━━━━━━━━━
    error: argument `a` of `v` is labeled `variadic` but its parameter is not variadic
      ╭▸ 
    6 │ select v(variadic a => 1);
      ╰╴         ━━━━━━━━━━━━━━━
    ");
}

#[test]
fn positional_count_names_missing() {
    assert_snapshot!(check("
create function d(a int, b int, c int default 1) returns int language sql return 1;
create function u(int, int) returns int language sql return 1;
select d();
select u(1);
"), @"
    error: function `d` takes at least 2 arguments but 0 were supplied
      ╭▸ 
    4 │ select d();
      │        ━
      │
      ╰ note: missing `a`, `b`
    error: function `u` takes 2 arguments but 1 was supplied
      ╭▸ 
    5 │ select u(1);
      │        ━
      │
      ╰ note: missing `$2`
    ");
}
