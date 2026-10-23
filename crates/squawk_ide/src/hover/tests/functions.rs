use insta::assert_snapshot;

use super::hover;

#[test]
fn function_call() {
    assert_snapshot!(hover("
create function add(a int, b int) returns int
  language sql
  return a + b;
select add$0(1, 2);
"), @"
    hover: function public.add(a int4, b int4) returns int4
      ╭▸ file0.sql:5:8
      │
    5 │ select add(1, 2);
      ╰╴       ─── hover
    ");
}

#[test]
fn function_definition() {
    assert_snapshot!(hover("
create function add$0(a int, b int) returns int
  language sql
  return a + b;
"), @"
    hover: function public.add(a int4, b int4) returns int4
      ╭▸ file0.sql:2:17
      │
    2 │ create function add(a int, b int) returns int
      ╰╴                ─── hover
    ");
}

#[test]
fn function_schema_qualified() {
    assert_snapshot!(hover("
create schema s;
create function s.f() returns text
  language sql
  return 'a';
select s.f$0();
"), @"
    hover: function s.f() returns text
      ╭▸ file0.sql:6:10
      │
    6 │ select s.f();
      ╰╴         ─ hover
    ");
}

#[test]
fn function_unnamed_params() {
    assert_snapshot!(hover("
create function f(int, text) returns bool
  language sql
  return true;
select f$0(1, 'a');
"), @"
    hover: function public.f(int4, text) returns bool
      ╭▸ file0.sql:5:8
      │
    5 │ select f(1, 'a');
      ╰╴       ─ hover
    ");
}

#[test]
fn function_out_params() {
    assert_snapshot!(hover("
create function f(in a int, out b int, inout c text)
  language sql
  as 'select 1, ''a''';
select f$0(1, 'a');
"), @"
    hover: function public.f(a int4, out b int4, inout c text) returns record
      ╭▸ file0.sql:5:8
      │
    5 │ select f(1, 'a');
      ╰╴       ─ hover
    ");
}

#[test]
fn function_variadic() {
    assert_snapshot!(hover("
create function f(variadic a int[]) returns int
  language sql
  return 1;
select f$0(1, 2, 3);
"), @"
    hover: function public.f(variadic a int4[]) returns int4
      ╭▸ file0.sql:5:8
      │
    5 │ select f(1, 2, 3);
      ╰╴       ─ hover
    ");
}

#[test]
fn function_default() {
    assert_snapshot!(hover("
create function f(a int default 1) returns int
  language sql
  return a;
select f$0();
"), @"
    hover: function public.f(a int4 default) returns int4
      ╭▸ file0.sql:5:8
      │
    5 │ select f();
      ╰╴       ─ hover
    ");
}

#[test]
fn function_returns_table() {
    assert_snapshot!(hover("
create function f() returns table (a int, b text)
  language sql
  as 'select 1, ''a''';
select * from f$0();
"), @"
    hover: function public.f() returns table(a int4, b text)
      ╭▸ file0.sql:5:15
      │
    5 │ select * from f();
      ╰╴              ─ hover
    ");
}

#[test]
fn function_returns_setof_table() {
    assert_snapshot!(hover("
create table t (a int);
create function f() returns setof t
  language sql
  as 'select * from t';
select * from f$0();
"), @"
    hover: function public.f() returns t
      ╭▸ file0.sql:6:15
      │
    6 │ select * from f();
      ╰╴              ─ hover
    ");
}

#[test]
fn builtin_function() {
    assert_snapshot!(hover("
select now$0();
"), @"
    hover: function pg_catalog.now() returns timestamptz
      ╭▸ file0.sql:2:8
      │
    2 │ select now();
      │        ─── hover
      │
      ╰ note: current transaction time
    ");
}

#[test]
fn overloaded_function() {
    assert_snapshot!(hover("
create function f(a int) returns int
  language sql
  return a;
create function f(a text) returns text
  language sql
  return a;
select f$0('a'::text);
"), @"
    hover: function public.f(a text) returns text
      ╭▸ file0.sql:8:8
      │
    8 │ select f('a'::text);
      ╰╴       ─ hover
    ");
}

#[test]
fn param_reference() {
    assert_snapshot!(hover("
create function f(a int) returns int
  language sql
  return a$0 + 1;
"), @"
    hover: parameter public.f.a int4
      ╭▸ file0.sql:4:10
      │
    4 │   return a + 1;
      ╰╴         ─ hover
    ");
}

#[test]
fn param_definition() {
    assert_snapshot!(hover("
create function f(a$0 int) returns int
  language sql
  return a + 1;
"), @"
    hover: parameter public.f.a int4
      ╭▸ file0.sql:2:19
      │
    2 │ create function f(a int) returns int
      ╰╴                  ─ hover
    ");
}

#[test]
fn positional_param() {
    assert_snapshot!(hover("
create function f(int) returns int
  language sql
  return $1$0 + 1;
"), @"
    hover: parameter public.f.$1 int4
      ╭▸ file0.sql:4:10
      │
    4 │   return $1 + 1;
      ╰╴         ── hover
    ");
}

#[test]
fn named_arg() {
    assert_snapshot!(hover("
create function f(a int, b int) returns int
  language sql
  return a + b;
select f(b$0 => 1, a => 2);
"), @"
    hover: parameter public.f.b int4
      ╭▸ file0.sql:5:10
      │
    5 │ select f(b => 1, a => 2);
      ╰╴         ─ hover
    ");
}

#[test]
fn aggregate() {
    assert_snapshot!(hover("
create aggregate my_sum(int) (
  sfunc = int4pl,
  stype = int
);
select my_sum$0(1);
"), @"
    hover: aggregate public.my_sum(int4) returns int4
      ╭▸ file0.sql:6:8
      │
    6 │ select my_sum(1);
      ╰╴       ────── hover
    ");
}

#[test]
fn ordered_set_aggregate() {
    assert_snapshot!(hover("
select percentile_disc$0(0.5) within group (order by x) from (select 1 x) t;
"), @"
    hover: aggregate pg_catalog.percentile_disc(float8 order by anyelement) returns anyelement
      ╭▸ file0.sql:2:8
      │
    2 │ select percentile_disc(0.5) within group (order by x) from (select 1 x) t;
      │        ─────────────── hover
      │
      ╰ note: discrete percentile
    ");
}

#[test]
fn procedure() {
    assert_snapshot!(hover("
create procedure p(a int)
  language sql
  as 'select 1';
call p$0(1);
"), @"
    hover: procedure public.p(a int4)
      ╭▸ file0.sql:5:6
      │
    5 │ call p(1);
      ╰╴     ─ hover
    ");
}

#[test]
fn drop_function() {
    assert_snapshot!(hover("
create function f(a int) returns int
  language sql
  return a;
drop function f$0(int);
"), @"
    hover: function public.f(a int4) returns int4
      ╭▸ file0.sql:5:15
      │
    5 │ drop function f(int);
      ╰╴              ─ hover
    ");
}

#[test]
fn function_in_body() {
    assert_snapshot!(hover("
create table t (a int);
create function f() returns int
  language sql
  return (select a$0 from t);
"), @"
    hover: column public.t.a int4
      ╭▸ file0.sql:5:18
      │
    5 │   return (select a from t);
      ╰╴                 ─ hover
    ");
}

#[test]
fn builtin_named_arg() {
    assert_snapshot!(hover("
select make_interval(days$0 => 1);
"), @"
    hover: parameter pg_catalog.make_interval.days int4
      ╭▸ file0.sql:2:22
      │
    2 │ select make_interval(days => 1);
      ╰╴                     ──── hover
    ");
}

#[test]
fn builtin_aggregate() {
    assert_snapshot!(hover("
select count$0(*) from pg_class;
"), @"
    hover: aggregate pg_catalog.count() returns int8
      ╭▸ file0.sql:2:8
      │
    2 │ select count(*) from pg_class;
      │        ───── hover
      │
      ╰ note: number of input rows
    ");
}

#[test]
fn aggregate_finalfunc() {
    assert_snapshot!(hover("
create function fin(int) returns text
  language sql
  return 'a';
create aggregate agg(int) (
  sfunc = int4pl,
  stype = int,
  finalfunc = fin
);
select agg$0(1);
"), @"
    hover: aggregate public.agg(int4) returns text
       ╭▸ file0.sql:10:8
       │
    10 │ select agg(1);
       ╰╴       ─── hover
    ");
}

#[test]
fn builtin_aggregate_finalfunc() {
    assert_snapshot!(hover("
select avg$0(1::int4);
"), @"
    hover: aggregate pg_catalog.avg(int4) returns numeric
      ╭▸ file0.sql:2:8
      │
    2 │ select avg(1::int4);
      │        ─── hover
      │
      ╰ note: the average (arithmetic mean) as numeric of all integer values
    ");
}

#[test]
fn moved_function_param() {
    assert_snapshot!(hover("
create schema s;
create function f(a int) returns int
  language sql
  return a$0;
alter function f(int) set schema s;
"), @"
    hover: parameter s.f.a int4
      ╭▸ file0.sql:5:10
      │
    5 │   return a;
      ╰╴         ─ hover
    ");
}

#[test]
fn second_positional_param() {
    assert_snapshot!(hover("
create function f(int, text) returns text
  language sql
  return $2$0;
"), @"
    hover: parameter public.f.$2 text
      ╭▸ file0.sql:4:10
      │
    4 │   return $2;
      ╰╴         ── hover
    ");
}

#[test]
fn function_returns_table_percent_type() {
    assert_snapshot!(hover("
create table t (a int);
create function f() returns table (x t.a%type)
  language sql
  as 'select 1';
select * from f$0();
"), @"
    hover: function public.f() returns table(x int4)
      ╭▸ file0.sql:6:15
      │
    6 │ select * from f();
      ╰╴              ─ hover
    ");
}

#[test]
fn function_single_out_param_call_type() {
    assert_snapshot!(hover("
create function f(out a int)
  as 'select 1'
  language sql;
select x$0 from (select f() as x) s;
"), @"
    hover: column x int4
      ╭▸ file0.sql:5:8
      │
    5 │ select x from (select f() as x) s;
      ╰╴       ─ hover
    ");
}
