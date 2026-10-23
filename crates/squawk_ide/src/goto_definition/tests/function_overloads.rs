use insta::assert_snapshot;

use super::goto;

#[test]
fn function_call_overload_by_arg_count() {
    assert_snapshot!(goto("
create function f(a int) returns int
  language sql
  return a;
create function f(a int, b int) returns int
  language sql
  return a + b;
select f$0(1);
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create function f(a int) returns int
      │                 ─ 2. destination
      ‡
    8 │ select f(1);
      ╰╴       ─ 1. source
    ");
}

#[test]
fn function_call_overload_by_arg_type() {
    assert_snapshot!(goto("
create function f(a int) returns int
  language sql
  return a;
create function f(a text) returns text
  language sql
  return a;
select f$0('a');
"), @"
    info: goto definition
      ╭▸ file0.sql:5:17
      │
    5 │ create function f(a text) returns text
      │                 ─ 2. destination
      ‡
    8 │ select f('a');
      ╰╴       ─ 1. source
    ");
}

#[test]
fn function_call_overload_by_column_type() {
    assert_snapshot!(goto("
create table t (
  a bigint,
  b text
);
create function f(x bigint) returns int
  language sql
  return 1;
create function f(x text) returns int
  language sql
  return 2;
select f$0(a) from t;
"), @"
    info: goto definition
       ╭▸ file0.sql:6:17
       │
     6 │ create function f(x bigint) returns int
       │                 ─ 2. destination
       ‡
    12 │ select f(a) from t;
       ╰╴       ─ 1. source
    ");
}

#[test]
fn function_call_overload_prefers_exact_over_polymorphic() {
    assert_snapshot!(goto("
create function f(x anyelement) returns int
  language sql
  return 1;
create function f(x int) returns int
  language sql
  return 2;
create function f(x bigint) returns int
  language sql
  return 3;
select f$0(1);
"), @"
    info: goto definition
       ╭▸ file0.sql:5:17
       │
     5 │ create function f(x int) returns int
       │                 ─ 2. destination
       ‡
    11 │ select f(1);
       ╰╴       ─ 1. source
    ");
}

#[test]
fn function_call_overload_implicit_cast() {
    assert_snapshot!(goto("
create function f(x bigint) returns int
  language sql
  return 1;
create function f(x text) returns int
  language sql
  return 2;
select f$0(1);
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create function f(x bigint) returns int
      │                 ─ 2. destination
      ‡
    8 │ select f(1);
      ╰╴       ─ 1. source
    ");
}

#[test]
fn function_call_overload_preferred_type() {
    assert_snapshot!(goto("
create function f(x numeric) returns int
  language sql
  return 1;
create function f(x double precision) returns int
  language sql
  return 2;
select f$0(1);
"), @"
    info: goto definition
      ╭▸ file0.sql:5:17
      │
    5 │ create function f(x double precision) returns int
      │                 ─ 2. destination
      ‡
    8 │ select f(1);
      ╰╴       ─ 1. source
    ");
}

#[test]
fn function_call_string_literal_prefers_user_preferred_type() {
    assert_snapshot!(goto("
create type a (
  input = int4in,
  output = int4out,
  category = 'Q'
);
create type b (
  input = int4in,
  output = int4out,
  category = 'Q',
  preferred = true
);
create function f(x a) returns int
  language sql
  return 1;
create function f(x b) returns int
  language sql
  return 2;
select f$0('x');
"), @"
    info: goto definition
       ╭▸ file0.sql:16:17
       │
    16 │ create function f(x b) returns int
       │                 ─ 2. destination
       ‡
    19 │ select f('x');
       ╰╴       ─ 1. source
    ");
}

#[test]
fn function_call_string_literal_user_string_category() {
    assert_snapshot!(goto("
create type a (
  input = int4in,
  output = int4out,
  category = 'S'
);
create function f(x int) returns int
  language sql
  return 1;
create function f(x a) returns int
  language sql
  return 2;
select f$0('x');
"), @"
    info: goto definition
       ╭▸ file0.sql:10:17
       │
    10 │ create function f(x a) returns int
       │                 ─ 2. destination
       ‡
    13 │ select f('x');
       ╰╴       ─ 1. source
    ");
}

#[test]
fn function_call_string_literal_user_category_without_preferred() {
    assert_snapshot!(goto("
create type a (
  input = int4in,
  output = int4out,
  category = 'Q'
);
create type b (
  input = int4in,
  output = int4out,
  category = 'Q'
);
create function f(x a) returns int
  language sql
  return 1;
create function f(x b) returns int
  language sql
  return 2;
select f$0('x');
"), @"no definition");
}

#[test]
fn function_call_overload_unknown_prefers_string() {
    assert_snapshot!(goto("
create function f(x int) returns int
  language sql
  return 1;
create function f(x text) returns int
  language sql
  return 2;
select f$0(null);
"), @"
    info: goto definition
      ╭▸ file0.sql:5:17
      │
    5 │ create function f(x text) returns int
      │                 ─ 2. destination
      ‡
    8 │ select f(null);
      ╰╴       ─ 1. source
    ");
}

#[test]
fn function_call_string_literal_converts_to_int() {
    assert_snapshot!(goto("
create function f(x int) returns int
  language sql
  return x;
select f$0('1');
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create function f(x int) returns int
      │                 ─ 2. destination
      ‡
    5 │ select f('1');
      ╰╴       ─ 1. source
    ");
}

#[test]
fn function_call_string_literal_overload_picked_by_other_arg() {
    assert_snapshot!(goto("
create function f(a int, b int) returns int
  language sql
  return 1;
create function f(a bigint, b bigint) returns int
  language sql
  return 2;
select f$0(1, '2');
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create function f(a int, b int) returns int
      │                 ─ 2. destination
      ‡
    8 │ select f(1, '2');
      ╰╴       ─ 1. source
    ");
}

#[test]
fn function_call_string_literal_ambiguous_int_overloads() {
    assert_snapshot!(goto("
create function f(x int) returns int
  language sql
  return 1;
create function f(x bigint) returns int
  language sql
  return 2;
select f$0('1');
"), @"no definition");
}

#[test]
fn function_call_null_ambiguous_int_overloads() {
    assert_snapshot!(goto("
create function f(x int) returns int
  language sql
  return 1;
create function f(x bigint) returns int
  language sql
  return 2;
select f$0(null);
"), @"no definition");
}

#[test]
fn function_call_string_literal_ambiguous_categories() {
    assert_snapshot!(goto("
create function f(x int) returns int
  language sql
  return 1;
create function f(x bool) returns int
  language sql
  return 2;
select f$0('1');
"), @"no definition");
}

#[test]
fn function_call_implicit_cast_ambiguous() {
    assert_snapshot!(goto("
create function f(x bigint) returns int
  language sql
  return 1;
create function f(x numeric) returns int
  language sql
  return 2;
select f$0(1);
"), @"no definition");
}

#[test]
fn function_call_string_literal_prefers_preferred_type() {
    assert_snapshot!(goto("
create function f(x int) returns int
  language sql
  return 1;
create function f(x double precision) returns int
  language sql
  return 2;
select f$0('1');
"), @"
    info: goto definition
      ╭▸ file0.sql:5:17
      │
    5 │ create function f(x double precision) returns int
      │                 ─ 2. destination
      ‡
    8 │ select f('1');
      ╰╴       ─ 1. source
    ");
}

#[test]
fn function_call_string_literal_prefers_text() {
    assert_snapshot!(goto("
create function f(x varchar) returns int
  language sql
  return 1;
create function f(x text) returns int
  language sql
  return 2;
select f$0('a');
"), @"
    info: goto definition
      ╭▸ file0.sql:5:17
      │
    5 │ create function f(x text) returns int
      │                 ─ 2. destination
      ‡
    8 │ select f('a');
      ╰╴       ─ 1. source
    ");
}

#[test]
fn builtin_function_string_literal_preferred_type() {
    assert_snapshot!(goto("
select round$0('1.5');
"), @"
    info: goto definition
          ╭▸ file0.sql:2:8
          │
        2 │ select round('1.5');
          │        ───── 1. source
          │
          ⸬  builtins.sql:26321:28
          │
    26321 │ create function pg_catalog.round(double precision) returns double precision
          ╰╴                           ───── 2. destination
    ");
}

#[test]
fn function_call_overload_by_named_arg_type() {
    assert_snapshot!(goto("
create function f(a int, b int) returns int
  language sql
  return 1;
create function f(a int, b text) returns int
  language sql
  return 2;
select f$0(b => 'x', a => 1);
"), @"
    info: goto definition
      ╭▸ file0.sql:5:17
      │
    5 │ create function f(a int, b text) returns int
      │                 ─ 2. destination
      ‡
    8 │ select f(b => 'x', a => 1);
      ╰╴       ─ 1. source
    ");
}

#[test]
fn function_call_overload_by_nested_call_type() {
    assert_snapshot!(goto("
create function g() returns text
  language sql
  return 'a';
create function f(x int) returns int
  language sql
  return 1;
create function f(x text) returns int
  language sql
  return 2;
select f$0(g());
"), @"
    info: goto definition
       ╭▸ file0.sql:8:17
       │
     8 │ create function f(x text) returns int
       │                 ─ 2. destination
       ‡
    11 │ select f(g());
       ╰╴       ─ 1. source
    ");
}

#[test]
fn function_call_overload_variadic_element_type() {
    assert_snapshot!(goto("
create function f(variadic xs int[]) returns int
  language sql
  return 1;
create function f(variadic xs text[]) returns int
  language sql
  return 2;
select f$0('a', 'b');
"), @"
    info: goto definition
      ╭▸ file0.sql:5:17
      │
    5 │ create function f(variadic xs text[]) returns int
      │                 ─ 2. destination
      ‡
    8 │ select f('a', 'b');
      ╰╴       ─ 1. source
    ");
}

#[test]
fn function_call_overload_array_arg() {
    assert_snapshot!(goto("
create function f(x int) returns int
  language sql
  return 1;
create function f(x anyarray) returns int
  language sql
  return 2;
select f$0(array[1]::int[]);
"), @"
    info: goto definition
      ╭▸ file0.sql:5:17
      │
    5 │ create function f(x anyarray) returns int
      │                 ─ 2. destination
      ‡
    8 │ select f(array[1]::int[]);
      ╰╴       ─ 1. source
    ");
}

#[test]
fn function_call_overload_across_schemas() {
    assert_snapshot!(goto("
create function public.length(x int) returns int
  language sql
  return x;
select length$0(1);
"), @"
    info: goto definition
      ╭▸ file0.sql:2:24
      │
    2 │ create function public.length(x int) returns int
      │                        ────── 2. destination
      ‡
    5 │ select length(1);
      ╰╴       ────── 1. source
    ");
}

#[test]
fn builtin_function_overload_by_arg_type() {
    assert_snapshot!(goto("
select abs$0(1::bigint);
"), @"
    info: goto definition
         ╭▸ file0.sql:2:8
         │
       2 │ select abs(1::bigint);
         │        ─── 1. source
         │
         ⸬  builtins.sql:5117:28
         │
    5117 │ create function pg_catalog.abs(bigint) returns bigint
         ╰╴                           ─── 2. destination
    ");
}

#[test]
fn builtin_function_overload_preferred_type() {
    assert_snapshot!(goto("
select round$0(1);
"), @"
    info: goto definition
          ╭▸ file0.sql:2:8
          │
        2 │ select round(1);
          │        ───── 1. source
          │
          ⸬  builtins.sql:26321:28
          │
    26321 │ create function pg_catalog.round(double precision) returns double precision
          ╰╴                           ───── 2. destination
    ");
}

#[test]
fn builtin_function_overload_text() {
    assert_snapshot!(goto("
select length$0('abc');
"), @"
    info: goto definition
          ╭▸ file0.sql:2:8
          │
        2 │ select length('abc');
          │        ────── 1. source
          │
          ⸬  builtins.sql:17296:28
          │
    17296 │ create function pg_catalog.length(text) returns integer
          ╰╴                           ────── 2. destination
    ");
}

#[test]
fn function_call_default_and_variadic() {
    assert_snapshot!(goto("
create function f(a int, b int default 1) returns int
  language sql
  return a;
create function g(a int, variadic rest int[]) returns int
  language sql
  return a;
select f(1), f(1, 2), g$0(1, 2, 3, 4);
"), @"
    info: goto definition
      ╭▸ file0.sql:5:17
      │
    5 │ create function g(a int, variadic rest int[]) returns int
      │                 ─ 2. destination
      ‡
    8 │ select f(1), f(1, 2), g(1, 2, 3, 4);
      ╰╴                      ─ 1. source
    ");
}

#[test]
fn function_call_wrong_arg_count() {
    assert_snapshot!(goto("
create function f(a int) returns int
  language sql
  return a;
select f$0(1, 2);
"), @"no definition");
}

#[test]
fn named_arg_picks_overload() {
    assert_snapshot!(goto("
create function f(a int) returns int
  language sql
  return a;
create function f(b int) returns int
  language sql
  return b;
select f$0(a => 1);
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create function f(a int) returns int
      │                 ─ 2. destination
      ‡
    8 │ select f(a => 1);
      ╰╴       ─ 1. source
    ");
}
