use insta::assert_snapshot;

use super::{goto, goto_files};

#[test]
fn function_call() {
    assert_snapshot!(goto("
create function add(a int, b int) returns int
  language sql
  return a + b;
select add$0(1, 2);
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create function add(a int, b int) returns int
      │                 ─── 2. destination
      ‡
    5 │ select add(1, 2);
      ╰╴       ─── 1. source
    ");
}

#[test]
fn function_call_schema_qualified() {
    assert_snapshot!(goto("
create function public.add(a int, b int) returns int
  language sql
  return a + b;
select public.add$0(1, 2);
"), @"
    info: goto definition
      ╭▸ file0.sql:2:24
      │
    2 │ create function public.add(a int, b int) returns int
      │                        ─── 2. destination
      ‡
    5 │ select public.add(1, 2);
      ╰╴              ─── 1. source
    ");
}

#[test]
fn function_call_database_qualified() {
    assert_snapshot!(goto("
create function public.add(a int, b int) returns int
  language sql
  return a + b;
select db.public.add$0(1, 2);
"), @"
    info: goto definition
      ╭▸ file0.sql:2:24
      │
    2 │ create function public.add(a int, b int) returns int
      │                        ─── 2. destination
      ‡
    5 │ select db.public.add(1, 2);
      ╰╴                 ─── 1. source
    ");
}

#[test]
fn function_param() {
    assert_snapshot!(goto("
create function add(a int, b int) returns int
  language sql
  return a + b$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:28
      │
    2 │ create function add(a int, b int) returns int
      │                            ─ 2. destination
    3 │   language sql
    4 │   return a + b;
      ╰╴             ─ 1. source
    ");
}

#[test]
fn function_positional_param() {
    assert_snapshot!(goto("
create function add(a int, out c int, b int) returns int
  language sql
  return a + $2$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:39
      │
    2 │ create function add(a int, out c int, b int) returns int
      │                                       ─ 2. destination
    3 │   language sql
    4 │   return a + $2;
      ╰╴             ── 1. source
    ");
}

#[test]
fn function_qualified_param() {
    assert_snapshot!(goto("
create function add(a int, b int) returns int
  language sql
  return add.a$0 + b;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:21
      │
    2 │ create function add(a int, b int) returns int
      │                     ─ 2. destination
    3 │   language sql
    4 │   return add.a + b;
      ╰╴             ─ 1. source
    ");
}

#[test]
fn function_qualifier() {
    assert_snapshot!(goto("
create function add(a int, b int) returns int
  language sql
  return add$0.a + b;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create function add(a int, b int) returns int
      │                 ─── 2. destination
    3 │   language sql
    4 │   return add.a + b;
      ╰╴         ─── 1. source
    ");
}

#[test]
fn recursive_function_call() {
    assert_snapshot!(goto("
create function fact(n int) returns int
  language sql
  return case when n <= 1 then 1 else n * fact$0(n - 1) end;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create function fact(n int) returns int
      │                 ──── 2. destination
    3 │   language sql
    4 │   return case when n <= 1 then 1 else n * fact(n - 1) end;
      ╰╴                                          ──── 1. source
    ");
}

#[test]
fn function_in_another_file() {
    assert_snapshot!(goto_files(&["
create function add(a int, b int) returns int
  language sql
  return a + b;
", "
select add$0(1, 2);
"]), @"
    info: goto definition
      ╭▸ file1.sql:2:8
      │
    2 │ select add(1, 2);
      │        ─── 1. source
      │
      ⸬  file0.sql:2:17
      │
    2 │ create function add(a int, b int) returns int
      ╰╴                ─── 2. destination
    ");
}

#[test]
fn function_column_shadows_param() {
    assert_snapshot!(goto("
create table t (a int);
create function f(a int) returns setof int
  language sql
  begin atomic
    select a$0 from t;
  end;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create table t (a int);
      │                 ─ 2. destination
      ‡
    6 │     select a from t;
      ╰╴           ─ 1. source
    ");
}

#[test]
fn named_arg() {
    assert_snapshot!(goto("
create function f(a int, b int) returns int
  language sql
  return a + b;
select f(1, b$0 => 2);
"), @"
    info: goto definition
      ╭▸ file0.sql:2:26
      │
    2 │ create function f(a int, b int) returns int
      │                          ─ 2. destination
      ‡
    5 │ select f(1, b => 2);
      ╰╴            ─ 1. source
    ");
}

#[test]
fn named_arg_colon_eq() {
    assert_snapshot!(goto("
create function f(a int, b int) returns int
  language sql
  return a + b;
select f(b := 1, a$0 := 2);
"), @"
    info: goto definition
      ╭▸ file0.sql:2:19
      │
    2 │ create function f(a int, b int) returns int
      │                   ─ 2. destination
      ‡
    5 │ select f(b := 1, a := 2);
      ╰╴                 ─ 1. source
    ");
}

#[test]
fn named_arg_skips_default() {
    assert_snapshot!(goto("
create function f(a int, b int default 1, c int default 2) returns int
  language sql
  return a;
select f$0(1, c => 3);
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create function f(a int, b int default 1, c int default 2) returns int
      │                 ─ 2. destination
      ‡
    5 │ select f(1, c => 3);
      ╰╴       ─ 1. source
    ");
}

#[test]
fn named_arg_unknown_name() {
    assert_snapshot!(goto("
create function f(a int) returns int
  language sql
  return a;
select f$0(z => 1);
"), @"no definition");
}

#[test]
fn named_arg_already_positional() {
    assert_snapshot!(goto("
create function f(a int, b int default 1) returns int
  language sql
  return a;
select f$0(1, a => 2);
"), @"no definition");
}

#[test]
fn named_arg_missing_required() {
    assert_snapshot!(goto("
create function f(a int, b int) returns int
  language sql
  return a;
select f$0(b => 2);
"), @"no definition");
}

#[test]
fn variadic_requires_an_argument() {
    assert_snapshot!(goto("
create function g(a int, variadic rest int[]) returns int
  language sql
  return a;
select g$0(1);
"), @"no definition");
}

#[test]
fn variadic_call_uses_array_type() {
    assert_snapshot!(goto("
create function f(variadic xs int[]) returns int
  language sql
  return 1;
create function f(variadic xs text[]) returns int
  language sql
  return 2;
select f$0(variadic array['a', 'b']::text[]);
"), @"
    info: goto definition
      ╭▸ file0.sql:5:17
      │
    5 │ create function f(variadic xs text[]) returns int
      │                 ─ 2. destination
      ‡
    8 │ select f(variadic array['a', 'b']::text[]);
      ╰╴       ─ 1. source
    ");
}

#[test]
fn variadic_call_after_fixed_args() {
    assert_snapshot!(goto("
create function g(a int, variadic rest int[]) returns int
  language sql
  return a;
select g$0(1, variadic array[2, 3]);
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create function g(a int, variadic rest int[]) returns int
      │                 ─ 2. destination
      ‡
    5 │ select g(1, variadic array[2, 3]);
      ╰╴       ─ 1. source
    ");
}

#[test]
fn variadic_call_requires_variadic_param() {
    assert_snapshot!(goto("
create function f(xs int[]) returns int
  language sql
  return 1;
select f$0(variadic array[1, 2]);
"), @"no definition");
}

#[test]
fn variadic_call_cannot_follow_variadic_args() {
    assert_snapshot!(goto("
create function g(variadic rest int[]) returns int
  language sql
  return 1;
select g$0(1, variadic array[2, 3]);
"), @"no definition");
}

#[test]
fn variadic_call_element_type_does_not_match() {
    assert_snapshot!(goto("
create function g(variadic rest int[]) returns int
  language sql
  return 1;
select g$0(variadic 1);
"), @"no definition");
}

#[test]
fn named_arg_name_does_not_exist() {
    assert_snapshot!(goto("
create function f(a int) returns int
  language sql
  return a;
select f(z$0 => 1);
"), @"no definition");
}

#[test]
fn named_arg_unnamed_params() {
    assert_snapshot!(goto("
create function f(int) returns int
  language sql
  return $1;
select f(a$0 => 1);
"), @"no definition");
}

#[test]
fn builtin_function() {
    assert_snapshot!(goto("
select now$0();
"), @"
    info: goto definition
          ╭▸ file0.sql:2:8
          │
        2 │ select now();
          │        ─── 1. source
          │
          ⸬  builtins.sql:19278:28
          │
    19278 │ create function pg_catalog.now() returns timestamp with time zone
          ╰╴                           ─── 2. destination
    ");
}

#[test]
fn builtin_named_arg() {
    assert_snapshot!(goto("
select make_interval(days$0 => 1);
"), @"
    info: goto definition
          ╭▸ file0.sql:2:22
          │
        2 │ select make_interval(days => 1);
          │                      ──── 1. source
          │
          ⸬  builtins.sql:18090:3
          │
    18090 │   days integer default 0,
          ╰╴  ──── 2. destination
    ");
}

#[test]
fn create_function_sql_body_table() {
    assert_snapshot!(goto("
create table users (
  id int,
  name text
);
create function f() returns text
  language sql
  as $$ select name from users$0 $$;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:14
      │
    2 │ create table users (
      │              ───── 2. destination
      ‡
    8 │   as $$ select name from users $$;
      ╰╴                         ───── 1. source
    ");
}

#[test]
fn create_function_sql_body_column() {
    assert_snapshot!(goto("
create table users (
  id int,
  name text
);
create function f() returns text
  as 'select ''x'' || name$0 from users'
  language sql;
"), @"
    info: goto definition
      ╭▸ file0.sql:4:3
      │
    4 │   name text
      │   ──── 2. destination
      ‡
    7 │   as 'select ''x'' || name from users'
      ╰╴                      ──── 1. source
    ");
}

#[test]
fn create_function_sql_body_param() {
    assert_snapshot!(goto("
create function f(user_id int) returns int
  language sql
  as $$ select user_id$0 $$;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:19
      │
    2 │ create function f(user_id int) returns int
      │                   ─────── 2. destination
    3 │   language sql
    4 │   as $$ select user_id $$;
      ╰╴               ─────── 1. source
    ");
}

#[test]
fn create_function_sql_body_positional_param() {
    assert_snapshot!(goto("
create function f(a int, b int) returns int
  language sql
  as $$ select $2$0 $$;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:26
      │
    2 │ create function f(a int, b int) returns int
      │                          ─ 2. destination
    3 │   language sql
    4 │   as $$ select $2 $$;
      ╰╴               ── 1. source
    ");
}

#[test]
fn create_function_sql_body_cte() {
    assert_snapshot!(goto("
create function f() returns int
  language sql
  as $$ with t as (select 1 a) select a from t$0 $$;
"), @"
    info: goto definition
      ╭▸ file0.sql:4:14
      │
    4 │   as $$ with t as (select 1 a) select a from t $$;
      ╰╴             ─ 2. destination                ─ 1. source
    ");
}

#[test]
fn create_function_sql_body_from_other_file() {
    assert_snapshot!(goto_files(&["
create table users (
  id int,
  name text
);
", "
create function f() returns text
  language sql
  as $$ select name from users$0 $$;
"]), @"
    info: goto definition
      ╭▸ file1.sql:4:26
      │
    4 │   as $$ select name from users $$;
      │                          ───── 1. source
      │
      ⸬  file0.sql:2:14
      │
    2 │ create table users (
      ╰╴             ───── 2. destination
    ");
}

#[test]
fn param_default_function() {
    assert_snapshot!(goto("
create function g() returns int
  language sql
  return 1;
create function f(a int default g$0()) returns int
  language sql
  return a;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create function g() returns int
      │                 ─ 2. destination
      ‡
    5 │ create function f(a int default g()) returns int
      ╰╴                                ─ 1. source
    ");
}

#[test]
fn param_default_eq_type() {
    assert_snapshot!(goto("
create type pair as (
  x int,
  y int
);
create function f(a int = null::pair$0) returns int
  language sql
  return 1;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:13
      │
    2 │ create type pair as (
      │             ──── 2. destination
      ‡
    6 │ create function f(a int = null::pair) returns int
      ╰╴                                ──── 1. source
    ");
}

#[test]
fn schema_qualified_function_call() {
    assert_snapshot!(goto("
create schema s;
create function s.f() returns int
  language sql
  return 1;
select s$0.f();
"), @"
    info: goto definition
      ╭▸ file0.sql:2:15
      │
    2 │ create schema s;
      │               ─ 2. destination
      ‡
    6 │ select s.f();
      ╰╴       ─ 1. source
    ");
}

#[test]
fn create_procedure_param() {
    assert_snapshot!(goto("
create table t (a int);
create procedure p(x int)
  language sql
  begin atomic
    insert into t (a) values (x$0);
  end;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:20
      │
    3 │ create procedure p(x int)
      │                    ─ 2. destination
      ‡
    6 │     insert into t (a) values (x);
      ╰╴                              ─ 1. source
    ");
}

#[test]
fn drop_function_ref() {
    assert_snapshot!(goto("
create function f(int) returns int
  language sql
  return 1;
create function f(text) returns int
  language sql
  return 1;
drop function f$0(text);
"), @"
    info: goto definition
      ╭▸ file0.sql:5:17
      │
    5 │ create function f(text) returns int
      │                 ─ 2. destination
      ‡
    8 │ drop function f(text);
      ╰╴              ─ 1. source
    ");
}

#[test]
fn drop_procedure_ref() {
    assert_snapshot!(goto("
create procedure p()
  language sql
  begin atomic
    select 1;
  end;
drop procedure p$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:18
      │
    2 │ create procedure p()
      │                  ─ 2. destination
      ‡
    7 │ drop procedure p;
      ╰╴               ─ 1. source
    ");
}

#[test]
fn drop_routine_ref() {
    assert_snapshot!(goto("
create function f() returns int
  language sql
  return 1;
drop routine f$0();
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create function f() returns int
      │                 ─ 2. destination
      ‡
    5 │ drop routine f();
      ╰╴             ─ 1. source
    ");
}

#[test]
fn alter_function_rename_ref() {
    assert_snapshot!(goto("
create function f() returns int
  language sql
  return 1;
alter function f() rename to g;
select g$0();
"), @"
    info: goto definition
      ╭▸ file0.sql:5:30
      │
    5 │ alter function f() rename to g;
      │                              ─ 2. destination
    6 │ select g();
      ╰╴       ─ 1. source
    ");
}

#[test]
fn alter_function_set_schema_ref() {
    assert_snapshot!(goto("
create schema s;
create function f() returns int
  language sql
  return 1;
alter function f() set schema s;
select s.f$0();
"), @"
    info: goto definition
      ╭▸ file0.sql:3:17
      │
    3 │ create function f() returns int
      │                 ─ 2. destination
      ‡
    7 │ select s.f();
      ╰╴         ─ 1. source
    ");
}

#[test]
fn call_procedure_ref() {
    assert_snapshot!(goto("
create procedure p(int)
  language sql
  begin atomic
    select 1;
  end;
create procedure p(text)
  language sql
  begin atomic
    select 1;
  end;
call p$0('a'::text);
"), @"
    info: goto definition
       ╭▸ file0.sql:7:18
       │
     7 │ create procedure p(text)
       │                  ─ 2. destination
       ‡
    12 │ call p('a'::text);
       ╰╴     ─ 1. source
    ");
}

#[test]
fn call_procedure_named_arg() {
    assert_snapshot!(goto("
create procedure p(a int)
  language sql
  begin atomic
    select 1;
  end;
call p(a$0 => 1);
"), @"
    info: goto definition
      ╭▸ file0.sql:2:20
      │
    2 │ create procedure p(a int)
      │                    ─ 2. destination
      ‡
    7 │ call p(a => 1);
      ╰╴       ─ 1. source
    ");
}

#[test]
fn function_support_option() {
    assert_snapshot!(goto("
create function s(internal) returns internal
  language c
  as 'x';
create function f() returns int
  language sql
  support s$0
  return 1;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create function s(internal) returns internal
      │                 ─ 2. destination
      ‡
    7 │   support s
      ╰╴          ─ 1. source
    ");
}

#[test]
fn drop_function_cascade_ref() {
    assert_snapshot!(goto("
create function f() returns int
  language sql
  return 1;
create view v as
  select f() a;
drop function f$0 cascade;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create function f() returns int
      │                 ─ 2. destination
      ‡
    7 │ drop function f cascade;
      ╰╴              ─ 1. source
    ");
}

#[test]
fn comment_on_function_ref() {
    assert_snapshot!(goto("
create function f(int) returns int
  language sql
  as 'select 1';
create function f(text) returns int
  language sql
  as 'select 1';
comment on function f$0(text) is 'x';
"), @"
    info: goto definition
      ╭▸ file0.sql:5:17
      │
    5 │ create function f(text) returns int
      │                 ─ 2. destination
      ‡
    8 │ comment on function f(text) is 'x';
      ╰╴                    ─ 1. source
    ");
}

#[test]
fn drop_function_missing_arg_type_matches_nothing() {
    assert_snapshot!(goto("
create function add(bigint) returns bigint
  as $$ select 1 $$
  language sql;
drop function add$0(complex);
"), @"no definition");
}

#[test]
fn drop_function_percent_type_arg() {
    assert_snapshot!(goto("
create table t (a bigint);
create function add(bigint) returns bigint
  as $$ select 1 $$
  language sql;
drop function add$0(t.a%type);
"), @"
    info: goto definition
      ╭▸ file0.sql:3:17
      │
    3 │ create function add(bigint) returns bigint
      │                 ─── 2. destination
      ‡
    6 │ drop function add(t.a%type);
      ╰╴              ─── 1. source
    ");
}

#[test]
fn function_param_in_begin_atomic_update() {
    assert_snapshot!(goto("
create table t (a int);
create function f(x int) returns void
  begin atomic
    update t set a = x$0;
  end;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:19
      │
    3 │ create function f(x int) returns void
      │                   ─ 2. destination
    4 │   begin atomic
    5 │     update t set a = x;
      ╰╴                     ─ 1. source
    ");
}

#[test]
fn function_param_in_begin_atomic_merge() {
    assert_snapshot!(goto("
create table t (a int);
create function f(x int) returns void
  begin atomic
    merge into t
    using t u on t.a = x$0
    when matched then
      do nothing;
  end;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:19
      │
    3 │ create function f(x int) returns void
      │                   ─ 2. destination
      ‡
    6 │     using t u on t.a = x
      ╰╴                       ─ 1. source
    ");
}

#[test]
fn function_param_in_begin_atomic_delete_where() {
    assert_snapshot!(goto("
create table t (a int);
create function f(x int) returns void
  begin atomic
    delete from t where a = x$0;
  end;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:19
      │
    3 │ create function f(x int) returns void
      │                   ─ 2. destination
    4 │   begin atomic
    5 │     delete from t where a = x;
      ╰╴                            ─ 1. source
    ");
}

#[test]
fn begin_atomic_param_function_qualified() {
    assert_snapshot!(goto("
create table u (x int);
create table w (y int);
create table t (a int);
create function f(x int) returns setof int
  begin atomic
    select 1 from u, (select f.x$0) v;
  end;
"), @"
    info: goto definition
      ╭▸ file0.sql:5:19
      │
    5 │ create function f(x int) returns setof int
      │                   ─ 2. destination
    6 │   begin atomic
    7 │     select 1 from u, (select f.x) v;
      ╰╴                               ─ 1. source
    ");
}

#[test]
fn begin_atomic_param_wrong_qualifier() {
    assert_snapshot!(goto("
create table w (y int);
create function f(x int) returns setof int
  begin atomic
    select g.x$0 from w;
  end;
"), @"no definition");
}

#[test]
fn begin_atomic_param_not_hidden_by_non_lateral_sibling() {
    assert_snapshot!(goto("
create table u (x int);
create table w (y int);
create table t (a int);
create function f(x int) returns setof int
  begin atomic
    select 1 from u, (values (x$0)) v;
  end;
"), @"
    info: goto definition
      ╭▸ file0.sql:5:19
      │
    5 │ create function f(x int) returns setof int
      │                   ─ 2. destination
    6 │   begin atomic
    7 │     select 1 from u, (values (x)) v;
      ╰╴                              ─ 1. source
    ");
}

#[test]
fn function_own_set_search_path_does_not_leak_after_body() {
    assert_snapshot!(goto("
create schema bar;
create table bar.t (id int);
create table public.t (id int);
create function caller() returns int
  language sql
  set search_path = bar
  begin atomic
    select 1;
  end;
select * from t$0;
"), @"
    info: goto definition
       ╭▸ file0.sql:4:21
       │
     4 │ create table public.t (id int);
       │                     ─ 2. destination
       ‡
    11 │ select * from t;
       ╰╴              ─ 1. source
    ");
}

#[test]
fn function_param_self_qualified_in_sql_string_body() {
    assert_snapshot!(goto("
create function f(value int) returns int
  language sql
  as $$
  select f.value$0 + 1;
$$;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:19
      │
    2 │ create function f(value int) returns int
      │                   ───── 2. destination
      ‡
    5 │   select f.value + 1;
      ╰╴           ───── 1. source
    ");
}
