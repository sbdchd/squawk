-- create_func_with_in_out
create function dup(in int, out f1 int, out f2 text)
    as $$ select $1, cast($1 as text) || ' is text' $$
    language sql;

-- security_definer
CREATE FUNCTION check_password(uname TEXT, pass TEXT)
RETURNS BOOLEAN AS $$
DECLARE passed BOOLEAN;
BEGIN
        SELECT  (pwd = $2) INTO passed
        FROM    pwds
        WHERE   username = $1;

        RETURN passed;
END;
$$  LANGUAGE plpgsql
    SECURITY DEFINER
    -- Set a secure search_path: trusted schema(s), then 'pg_temp'.
    SET search_path = admin, pg_temp;

-- create_function_with_percent_type
-- column type return
create function f(a t.c%type) 
returns t.b%type 
as '' language plpgsql;

-- setof return
create function f(int)
returns setof b.foo
as '' language sql;

-- void return
create function f(int)
returns void
as '' language sql;

-- omitted return
create function f()
as '' language 'sql';

-- schema
create function public.f()
returns int
as 'select 1' language 'sql';

create function a.b.c.d()
returns int
as 'select 1' language 'sql';

create function public.a()
returns numeric[]
as '' language 'sql';

-- returns table
create function f()
returns table (a text, b int)
as '' language sql;

-- transform
create function f()
returns void
transform for type t, for type b
as ''
language sql;

-- transform with type with args
create function f()
returns varchar(255)
transform for type varchar(100)
as ''
language sql;

-- window
create function f()
returns void
window
as ''
language sql;

-- immutable, stable, volatile
create function f()
returns void
immutable
as ''
language sql;

create function f()
returns void
stable
as ''
language sql;

create function f()
returns void
volatile
as ''
language sql;

-- [ not ] leakproof
create function f()
returns void
not leakproof
as ''
language sql;

create function f()
returns void
leakproof
as ''
language sql;

-- CALLED ON NULL INPUT | RETURNS NULL ON NULL INPUT | STRICT
create function f()
returns void
called on null input
as ''
language sql;

create function f()
returns void
returns null on null input
as ''
language sql;

create function f()
returns void
strict
as ''
language sql;

-- [ EXTERNAL ] SECURITY INVOKER | [ EXTERNAL ] SECURITY DEFINER
create function f()
returns void
external security invoker
as ''
language sql;

create function f()
returns void
security invoker
as ''
language sql;

create function f()
returns void
external security definer
as ''
language sql;

create function f()
returns void
security definer
as ''
language sql;

-- parallel
create function f()
returns void
parallel unsafe
as ''
language sql;

create function f()
returns void
parallel restricted
as ''
language sql;

create function f()
returns void
parallel safe
as ''
language sql;


-- cost
create function f()
returns void
cost 100
as ''
language sql;

-- rows
create function f()
returns void
rows 10
as ''
language sql;


-- suport
create function f()
returns void
support foo.bar
as ''
language sql;

create function f()
returns void
support bar
as ''
language sql;

-- set configuration_parameter
create function f()
returns void
set foo.bar from current
as ''
language sql;

create function f()
returns void
set bar to 'foo'
as ''
language sql;

create function f()
returns void
set bar = 1
as ''
language sql;

-- as def
create function f()
returns void
as 'select 1'
language sql;

-- as 'object_file', 'link_symbol'
create function f()
returns void
as 'foo', 'bar'
language foo;

-- return
create function f()
returns void
language sql
return (select 1);

-- begin atomic
create function f()
returns void
language sql
begin atomic
  select 1;
  select 2;
end;

create function f()
returns void
language sql
begin atomic
  select 1;
  end;
end;

-- lots of ;
create function f()
returns void
language sql
begin atomic
;
;
;
;
  select 1;
;
;
;;
end;

-- all options
create function f()
  returns void
  language sql
  transform for type foo
  window
  immutable
  not leakproof
  strict
  external security invoker
  parallel safe
  cost 1000
  rows 1000
  support foo.bar
  set a.b = 10
  as ''
  as 'foo', 'bar'
  return (select 1);

-- regression
create function foo(int8)
returns int
as 'select 1 + 1'
language sql;

create function f(bitmask bit(8))
returns boolean
as '0'
language sql;

-- argmode
create function foo(in int8)
returns void
as ''
language sql;

create function foo(out int8)
returns void
as ''
language sql;

create function foo(in out int8)
returns void
as ''
language sql;

create function foo(inout int8)
returns void
as ''
language sql;

create function foo(variadic int8)
returns void
as ''
language sql;

-- with_arg_names


-- single arg
create function foo(arg int8)
returns void
as ''
language sql;

-- multi args
create function foo(arg float, arg2 int4)
returns void
as ''
language sql;

-- named and unnamed
create function foo(arg float, int4)
returns void
as ''
language sql;

-- default
-- =
create function foo(arg float = 1, int4 = 4)
returns void
as ''
language sql;

-- not a parser error but something to warn about
create function foo(arg float = 1, int4)
returns void
as ''
language sql;

-- default
create function foo(arg int default 1)
returns void
as ''
language sql;

-- expr w/ default
create function foo(arg int default 1 * 2)
returns void
as ''
language sql;

-- expr w/ =
create function foo(arg int = 1 * 2)
returns void
as ''
language sql;

-- param_type_order
-- mode name type
create function f(in arg int8)
returns void
as '' language sql;

-- mode type
create function f(in int8)
returns void
as '' language sql;

-- name mode type
create function f(int8 in int8)
returns void
as '' language sql;

-- name type
create function f(int8 int8)
returns void
as '' language sql;

-- type
create function f(int8)
returns void
as '' language sql;



