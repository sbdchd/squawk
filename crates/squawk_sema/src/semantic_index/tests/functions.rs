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
    error: function `f(zz => int4)` does not exist
      ╭▸ 
    3 │ select f(a => 1, b => x), g(y => 1), f(zz => 1);
      │                                      ━
      │
      ╰ hint: consider adding explicit type casts
    ");
}

#[test]
fn named_arg_name_does_not_exist() {
    assert_snapshot!(check("
create function f(a int) returns int language sql return a;
create function g(int) returns int language sql return $1;
select f(z => 1), g(a => 1);
"), @"
    error: function `f(z => int4)` does not exist
      ╭▸ 
    4 │ select f(z => 1), g(a => 1);
      │        ━
      │
      ╰ hint: consider adding explicit type casts
    error: function `g(a => int4)` does not exist
      ╭▸ 
    4 │ select f(z => 1), g(a => 1);
      │                   ━
      │
      ╰ hint: consider adding explicit type casts
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
      ╰╴       ━
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
    error: procedure `p(c => int4)` does not exist
      ╭▸ 
    7 │ call p(c => 1);
      │      ━
      │
      ╰ hint: consider adding explicit type casts
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
    error: procedure `p(int4)` does not exist
      ╭▸ 
    4 │ call p(1);
      │      ━
      │
      ╰ hint: consider adding explicit type casts
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
create procedure p(int) as 'select 1' language sql;
select foo(1.5::float8);
select foo(1, 'a');
select foo(y => 1);
select public.foo(variadic array[1]);
select nope();
select lower(1);
call p('a'::text);
"), @"
    error: function `foo(float8)` does not exist
      ╭▸ 
    4 │ select foo(1.5::float8);
      │        ━━━
      │
      ╰ hint: consider adding explicit type casts
    error: function `foo(int4, unknown)` does not exist
      ╭▸ 
    5 │ select foo(1, 'a');
      │        ━━━
      │
      ╰ hint: consider adding explicit type casts
    error: function `foo(y => int4)` does not exist
      ╭▸ 
    6 │ select foo(y => 1);
      │        ━━━
      │
      ╰ hint: consider adding explicit type casts
    error: function `public.foo(variadic int4[])` does not exist
      ╭▸ 
    7 │ select public.foo(variadic array[1]);
      │               ━━━
      │
      ╰ hint: consider adding explicit type casts
    error: function `nope()` does not exist
      ╭▸ 
    8 │ select nope();
      ╰╴       ━━━━
    error: function `lower(int4)` does not exist
      ╭▸ 
    9 │ select lower(1);
      │        ━━━━━
      │
      ╰ hint: consider adding explicit type casts
    error: procedure `p(text)` does not exist
       ╭▸ 
    10 │ call p('a'::text);
       │      ━
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
    error: function `app.do_thing(int4, int4)` does not exist
      ╭▸ 
    5 │ select app.do_thing(1, 2);
      │            ━━━━━━━━
      │
      ╰ hint: consider adding explicit type casts
    error: procedure `app.do_thig()` does not exist
      ╭▸ 
    6 │ call app.do_thig();
      ╰╴         ━━━━━━━
    ");
}
