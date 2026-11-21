use insta::assert_snapshot;

use crate::test_utils::render_goto;

use crate::goto_type_definition::goto_type_definition_with_index;

#[track_caller]
fn goto(sql: &str) -> String {
    render_goto(
        &[sql],
        "goto type definition",
        goto_type_definition_with_index,
    )
    .unwrap_or_else(|| "no type definition".to_string())
}

#[test]
fn integer_literal() {
    assert_snapshot!(goto("select 1000$0;"), @"
    info: goto type definition
          ╭▸ file0.sql:1:8
          │
        1 │ select 1000;
          │        ──── 1. source
          │
          ⸬  builtins.sql:32162:24
          │
    32162 │ create type pg_catalog.int4 (
          ╰╴                       ──── 2. destination
    ");
}

#[test]
fn string_literal() {
    assert_snapshot!(goto("select 'hello'$0;"), @"
    info: goto type definition
          ╭▸ file0.sql:1:8
          │
        1 │ select 'hello';
          │        ─────── 1. source
          │
          ⸬  builtins.sql:32396:24
          │
    32396 │ create type pg_catalog.text (
          ╰╴                       ──── 2. destination
    ");
}

#[test]
fn coerced_string_literal() {
    assert_snapshot!(goto("select 1 + '1'$0;"), @"
    info: goto type definition
          ╭▸ file0.sql:1:12
          │
        1 │ select 1 + '1';
          │            ─── 1. source
          │
          ⸬  builtins.sql:32162:24
          │
    32162 │ create type pg_catalog.int4 (
          ╰╴                       ──── 2. destination
    ");
}

#[test]
fn coerced_left_string_literal() {
    assert_snapshot!(goto("select '1'$0 + 1;"), @"
    info: goto type definition
          ╭▸ file0.sql:1:8
          │
        1 │ select '1' + 1;
          │        ─── 1. source
          │
          ⸬  builtins.sql:32162:24
          │
    32162 │ create type pg_catalog.int4 (
          ╰╴                       ──── 2. destination
    ");
}

#[test]
fn comparison_literal() {
    assert_snapshot!(goto("select 1 = '1'$0;"), @"
    info: goto type definition
          ╭▸ file0.sql:1:12
          │
        1 │ select 1 = '1';
          │            ─── 1. source
          │
          ⸬  builtins.sql:32162:24
          │
    32162 │ create type pg_catalog.int4 (
          ╰╴                       ──── 2. destination
    ");
}

#[test]
fn cast_string_literal() {
    assert_snapshot!(goto("create domain d as text; select 'hello'$0::d;"), @"
    info: goto type definition
      ╭▸ file0.sql:1:15
      │
    1 │ create domain d as text; select 'hello'::d;
      ╰╴              ─ 2. destination  ─────── 1. source
    ");
}

#[test]
fn abs_string_literal() {
    assert_snapshot!(goto("select abs('1'$0);"), @"
    info: goto type definition
          ╭▸ file0.sql:1:12
          │
        1 │ select abs('1');
          │            ─── 1. source
          │
          ⸬  builtins.sql:32131:24
          │
    32131 │ create type pg_catalog.float8 (
          ╰╴                       ────── 2. destination
    ");
}

#[test]
fn function_argument_literal() {
    assert_snapshot!(goto("create function f(integer) returns integer
  language sql
  return 1; select f('-1'$0);"), @"
    info: goto type definition
          ╭▸ file0.sql:3:22
          │
        3 │   return 1; select f('-1');
          │                      ──── 1. source
          │
          ⸬  builtins.sql:32162:24
          │
    32162 │ create type pg_catalog.int4 (
          ╰╴                       ──── 2. destination
    ");
}

#[test]
fn sql_body_literal() {
    assert_snapshot!(goto("create function f() returns integer
  language sql
  as $$ select 1000$0; $$;"), @"
    info: goto type definition
          ╭▸ file0.sql:3:16
          │
        3 │   as $$ select 1000; $$;
          │                ──── 1. source
          │
          ⸬  builtins.sql:32162:24
          │
    32162 │ create type pg_catalog.int4 (
          ╰╴                       ──── 2. destination
    ");
}

#[test]
fn null_literal() {
    assert_snapshot!(goto("select null$0;"), @"no type definition");
}

#[test]
fn coerced_enum_literal() {
    assert_snapshot!(goto("create type mood as enum (
  'happy',
  'sad'
); select 'happy'$0::mood;"), @"
    info: goto type definition
      ╭▸ file0.sql:1:13
      │
    1 │ create type mood as enum (
      │             ──── 2. destination
      ‡
    4 │ ); select 'happy'::mood;
      ╰╴          ─────── 1. source
    ");
}

#[test]
fn case_literal() {
    assert_snapshot!(goto("select case when true then '1'$0 else 1 end;"), @"
    info: goto type definition
          ╭▸ file0.sql:1:28
          │
        1 │ select case when true then '1' else 1 end;
          │                            ─── 1. source
          │
          ⸬  builtins.sql:32162:24
          │
    32162 │ create type pg_catalog.int4 (
          ╰╴                       ──── 2. destination
    ");
}

#[test]
fn coalesce_literal() {
    assert_snapshot!(goto("select coalesce('1'$0, 1);"), @"
    info: goto type definition
          ╭▸ file0.sql:1:17
          │
        1 │ select coalesce('1', 1);
          │                 ─── 1. source
          │
          ⸬  builtins.sql:32162:24
          │
    32162 │ create type pg_catalog.int4 (
          ╰╴                       ──── 2. destination
    ");
}

#[test]
fn greatest_literal() {
    assert_snapshot!(goto("select greatest(1, '2'$0);"), @"
    info: goto type definition
          ╭▸ file0.sql:1:20
          │
        1 │ select greatest(1, '2');
          │                    ─── 1. source
          │
          ⸬  builtins.sql:32162:24
          │
    32162 │ create type pg_catalog.int4 (
          ╰╴                       ──── 2. destination
    ");
}

#[test]
fn nullif_literal() {
    assert_snapshot!(goto("select nullif(1, '1'$0);"), @"
    info: goto type definition
          ╭▸ file0.sql:1:18
          │
        1 │ select nullif(1, '1');
          │                  ─── 1. source
          │
          ⸬  builtins.sql:32162:24
          │
    32162 │ create type pg_catalog.int4 (
          ╰╴                       ──── 2. destination
    ");
}

#[test]
fn in_list_literal() {
    assert_snapshot!(goto("select 1 in ('1'$0, 2);"), @"
    info: goto type definition
          ╭▸ file0.sql:1:14
          │
        1 │ select 1 in ('1', 2);
          │              ─── 1. source
          │
          ⸬  builtins.sql:32162:24
          │
    32162 │ create type pg_catalog.int4 (
          ╰╴                       ──── 2. destination
    ");
}

#[test]
fn between_literal() {
    assert_snapshot!(goto("select 1 between '0'$0 and 2;"), @"
    info: goto type definition
          ╭▸ file0.sql:1:18
          │
        1 │ select 1 between '0' and 2;
          │                  ─── 1. source
          │
          ⸬  builtins.sql:32162:24
          │
    32162 │ create type pg_catalog.int4 (
          ╰╴                       ──── 2. destination
    ");
}

#[test]
fn any_literal() {
    assert_snapshot!(goto("select 1 = any('{1}'$0);"), @"
    info: goto type definition
          ╭▸ file0.sql:1:16
          │
        1 │ select 1 = any('{1}');
          │                ───── 1. source
          │
          ⸬  builtins.sql:32162:24
          │
    32162 │ create type pg_catalog.int4 (
          ╰╴                       ──── 2. destination
    ");
}

#[test]
fn array_literal() {
    assert_snapshot!(goto("select array['1'$0, 2];"), @"
    info: goto type definition
          ╭▸ file0.sql:1:14
          │
        1 │ select array['1', 2];
          │              ─── 1. source
          │
          ⸬  builtins.sql:32162:24
          │
    32162 │ create type pg_catalog.int4 (
          ╰╴                       ──── 2. destination
    ");
}

#[test]
fn union_left_literal() {
    assert_snapshot!(goto("select '1'$0 union select 1;"), @"
    info: goto type definition
          ╭▸ file0.sql:1:8
          │
        1 │ select '1' union select 1;
          │        ─── 1. source
          │
          ⸬  builtins.sql:32162:24
          │
    32162 │ create type pg_catalog.int4 (
          ╰╴                       ──── 2. destination
    ");
}

#[test]
fn union_right_literal() {
    assert_snapshot!(goto("select 1 union select '2'$0;"), @"
    info: goto type definition
          ╭▸ file0.sql:1:23
          │
        1 │ select 1 union select '2';
          │                       ─── 1. source
          │
          ⸬  builtins.sql:32162:24
          │
    32162 │ create type pg_catalog.int4 (
          ╰╴                       ──── 2. destination
    ");
}

#[test]
fn values_literal() {
    assert_snapshot!(goto("select * from (values (1), ('2'$0)) v (a);"), @"
    info: goto type definition
          ╭▸ file0.sql:1:29
          │
        1 │ select * from (values (1), ('2')) v (a);
          │                             ─── 1. source
          │
          ⸬  builtins.sql:32162:24
          │
    32162 │ create type pg_catalog.int4 (
          ╰╴                       ──── 2. destination
    ");
}

#[test]
fn limit_literal() {
    assert_snapshot!(goto("create table t (a int); select * from t limit '1'$0;"), @"
    info: goto type definition
          ╭▸ file0.sql:1:47
          │
        1 │ create table t (a int); select * from t limit '1';
          │                                               ─── 1. source
          │
          ⸬  builtins.sql:32168:24
          │
    32168 │ create type pg_catalog.int8 (
          ╰╴                       ──── 2. destination
    ");
}

#[test]
fn polymorphic_unresolved_call() {
    assert_snapshot!(goto("create function f(anyelement) returns anyelement
  language sql
  return $1; select f$0(null);"), @"no type definition");
}

#[test]
fn multi_arg_unnest() {
    assert_snapshot!(goto("select * from unnest$0(array[1], array['a']);"), @"no type definition");
}

#[test]
fn cte_column_int_literal() {
    assert_snapshot!(goto("
create type pg_catalog.int4;

with t as (select 1 a)
select a$0 from t;
"), @"
    info: goto type definition
      ╭▸ file0.sql:2:24
      │
    2 │ create type pg_catalog.int4;
      │                        ──── 2. destination
      ‡
    5 │ select a from t;
      ╰╴       ─ 1. source
    ");
}

#[test]
fn cte_column_hex_int8_literal() {
    assert_snapshot!(goto("
create type pg_catalog.int4;
create type pg_catalog.int8;

with t as (select 0x1_0000_0000 a)
select a$0 from t;
"), @"
    info: goto type definition
      ╭▸ file0.sql:3:24
      │
    3 │ create type pg_catalog.int8;
      │                        ──── 2. destination
      ‡
    6 │ select a from t;
      ╰╴       ─ 1. source
    ");
}

#[test]
fn cte_column_hex_numeric_literal() {
    assert_snapshot!(goto("
create type pg_catalog.int8;
create type pg_catalog.numeric;

with t as (select 0x1_0000_0000_0000_0000 a)
select a$0 from t;
"), @"
    info: goto type definition
      ╭▸ file0.sql:3:24
      │
    3 │ create type pg_catalog.numeric;
      │                        ─────── 2. destination
      ‡
    6 │ select a from t;
      ╰╴       ─ 1. source
    ");
}

#[test]
fn builtin_type() {
    assert_snapshot!(goto("
with t as (select 1 a)
select a$0 from t;
"), @"
    info: goto type definition
          ╭▸ file0.sql:3:8
          │
        3 │ select a from t;
          │        ─ 1. source
          │
          ⸬  builtins.sql:32162:24
          │
    32162 │ create type pg_catalog.int4 (
          ╰╴                       ──── 2. destination
    ");
}

#[test]
fn qualified_column() {
    assert_snapshot!(goto("
create type pg_catalog.int4;
with t as (select 1 a)
select t.a$0 from t;
"), @"
    info: goto type definition
      ╭▸ file0.sql:2:24
      │
    2 │ create type pg_catalog.int4;
      │                        ──── 2. destination
    3 │ with t as (select 1 a)
    4 │ select t.a from t;
      ╰╴         ─ 1. source
    ");
}

#[test]
fn cast() {
    assert_snapshot!(goto("
create type foo;
with t as (select null::foo a)
select a$0 from t;
"), @"
    info: goto type definition
      ╭▸ file0.sql:2:13
      │
    2 │ create type foo;
      │             ─── 2. destination
    3 │ with t as (select null::foo a)
    4 │ select a from t;
      ╰╴       ─ 1. source
    ");
}

#[test]
fn paren_cast() {
    assert_snapshot!(goto("
create type foo;
with t as (select (('x')::foo) a)
select a$0 from t;
"), @"
    info: goto type definition
      ╭▸ file0.sql:2:13
      │
    2 │ create type foo;
      │             ─── 2. destination
    3 │ with t as (select (('x')::foo) a)
    4 │ select a from t;
      ╰╴       ─ 1. source
    ");
}

#[test]
fn column_alias() {
    assert_snapshot!(goto("
create type pg_catalog.text;
select 'a' b$0;
"), @"
    info: goto type definition
      ╭▸ file0.sql:2:24
      │
    2 │ create type pg_catalog.text;
      │                        ──── 2. destination
    3 │ select 'a' b;
      ╰╴           ─ 1. source
    ");
}

#[test]
fn table_column() {
    assert_snapshot!(goto("
create type foo;
create table t(a foo);
select a$0 from t;
"), @"
    info: goto type definition
      ╭▸ file0.sql:2:13
      │
    2 │ create type foo;
      │             ─── 2. destination
    3 │ create table t(a foo);
    4 │ select a from t;
      ╰╴       ─ 1. source
    ");
}

#[test]
fn table_column_def() {
    assert_snapshot!(goto("
create type foo;
create table t(a$0 foo);
"), @"
    info: goto type definition
      ╭▸ file0.sql:2:13
      │
    2 │ create type foo;
      │             ─── 2. destination
    3 │ create table t(a foo);
      ╰╴               ─ 1. source
    ");
}

#[test]
fn nested_cte() {
    assert_snapshot!(goto("
create type pg_catalog.bool;
with t as (select true a), u as (select a from t)
select a$0 from u;
"), @"
    info: goto type definition
      ╭▸ file0.sql:2:24
      │
    2 │ create type pg_catalog.bool;
      │                        ──── 2. destination
    3 │ with t as (select true a), u as (select a from t)
    4 │ select a from u;
      ╰╴       ─ 1. source
    ");
}

#[test]
fn cte_column_list() {
    assert_snapshot!(goto("
create type pg_catalog.numeric;
with t(b) as (select 1.5 a)
select b$0 from t;
"), @"
    info: goto type definition
      ╭▸ file0.sql:2:24
      │
    2 │ create type pg_catalog.numeric;
      │                        ─────── 2. destination
    3 │ with t(b) as (select 1.5 a)
    4 │ select b from t;
      ╰╴       ─ 1. source
    ");
}

#[test]
fn values() {
    assert_snapshot!(goto("
create type pg_catalog.int8;
select column1$0 from (values (9999999999)) v;
"), @"
    info: goto type definition
      ╭▸ file0.sql:2:24
      │
    2 │ create type pg_catalog.int8;
      │                        ──── 2. destination
    3 │ select column1 from (values (9999999999)) v;
      ╰╴       ─────── 1. source
    ");
}

#[test]
fn type_ref() {
    assert_snapshot!(goto("
create type foo;
select null::foo$0;
"), @"
    info: goto type definition
      ╭▸ file0.sql:2:13
      │
    2 │ create type foo;
      │             ─── 2. destination
    3 │ select null::foo;
      ╰╴             ─── 1. source
    ");
}

#[test]
fn unknown_type() {
    assert_snapshot!(goto("
with t as (select null a)
select a$0 from t;
"), @"no type definition");
}

#[test]
fn table_ref() {
    assert_snapshot!(goto("
create table t(a int);
select a from t$0;
"), @"
    info: goto type definition
      ╭▸ file0.sql:2:14
      │
    2 │ create table t(a int);
      │              ─ 2. destination
    3 │ select a from t;
      ╰╴              ─ 1. source
    ");
}

#[test]
fn view_ref() {
    assert_snapshot!(goto("
create view v as select 1 a;
select a from v$0;
"), @"
    info: goto type definition
      ╭▸ file0.sql:2:13
      │
    2 │ create view v as select 1 a;
      │             ─ 2. destination
    3 │ select a from v;
      ╰╴              ─ 1. source
    ");
}

#[test]
fn table_def() {
    assert_snapshot!(goto("
create table t$0(a int);
"), @"
    info: goto type definition
      ╭▸ file0.sql:2:14
      │
    2 │ create table t(a int);
      │              ┬
      │              │
      │              1. source
      ╰╴             2. destination
    ");
}

#[test]
fn whole_row_ref() {
    assert_snapshot!(goto("
create table t(a int);
select t$0 from t;
"), @"
    info: goto type definition
      ╭▸ file0.sql:2:14
      │
    2 │ create table t(a int);
      │              ─ 2. destination
    3 │ select t from t;
      ╰╴       ─ 1. source
    ");
}

#[test]
fn aliased_whole_row_ref() {
    assert_snapshot!(goto("
create table t(a int);
select x$0 from t x;
"), @"
    info: goto type definition
      ╭▸ file0.sql:2:14
      │
    2 │ create table t(a int);
      │              ─ 2. destination
    3 │ select x from t x;
      ╰╴       ─ 1. source
    ");
}

#[test]
fn alias_def() {
    assert_snapshot!(goto("
create table t(a int);
select x.a from t x$0;
"), @"
    info: goto type definition
      ╭▸ file0.sql:2:14
      │
    2 │ create table t(a int);
      │              ─ 2. destination
    3 │ select x.a from t x;
      ╰╴                  ─ 1. source
    ");
}

#[test]
fn whole_row_through_cte() {
    assert_snapshot!(goto("
create table t(a int);
with c as (select t from t)
select t$0 from c;
"), @"
    info: goto type definition
      ╭▸ file0.sql:2:14
      │
    2 │ create table t(a int);
      │              ─ 2. destination
    3 │ with c as (select t from t)
    4 │ select t from c;
      ╰╴       ─ 1. source
    ");
}

#[test]
fn column_of_table_type() {
    assert_snapshot!(goto("
create table t(a int);
create table u(b t);
select b$0 from u;
"), @"
    info: goto type definition
      ╭▸ file0.sql:2:14
      │
    2 │ create table t(a int);
      │              ─ 2. destination
    3 │ create table u(b t);
    4 │ select b from u;
      ╰╴       ─ 1. source
    ");
}

#[test]
fn cast_to_table_type() {
    assert_snapshot!(goto("
create table t(a int);
select null::t$0;
"), @"
    info: goto type definition
      ╭▸ file0.sql:2:14
      │
    2 │ create table t(a int);
      │              ─ 2. destination
    3 │ select null::t;
      ╰╴             ─ 1. source
    ");
}

#[test]
fn cte_ref() {
    assert_snapshot!(goto("
with c as (select 1 a)
select a from c$0;
"), @"
    info: goto type definition
      ╭▸ file0.sql:2:6
      │
    2 │ with c as (select 1 a)
      │      ─ 2. destination
    3 │ select a from c;
      ╰╴              ─ 1. source
    ");
}

#[test]
fn param_def() {
    assert_snapshot!(goto("
create type foo;
create function f(x$0 foo) returns int language sql return 1;
"), @"
    info: goto type definition
      ╭▸ file0.sql:2:13
      │
    2 │ create type foo;
      │             ─── 2. destination
    3 │ create function f(x foo) returns int language sql return 1;
      ╰╴                  ─ 1. source
    ");
}

#[test]
fn param_ref() {
    assert_snapshot!(goto("
create type foo;
create function f(x foo) returns foo language sql return x$0;
"), @"
    info: goto type definition
      ╭▸ file0.sql:2:13
      │
    2 │ create type foo;
      │             ─── 2. destination
    3 │ create function f(x foo) returns foo language sql return x;
      ╰╴                                                         ─ 1. source
    ");
}

#[test]
fn qualified_param_ref() {
    assert_snapshot!(goto("
create type foo;
create function f(x foo) returns foo language sql return f.x$0;
"), @"
    info: goto type definition
      ╭▸ file0.sql:2:13
      │
    2 │ create type foo;
      │             ─── 2. destination
    3 │ create function f(x foo) returns foo language sql return f.x;
      ╰╴                                                           ─ 1. source
    ");
}

#[test]
fn positional_param_ref() {
    assert_snapshot!(goto("
create type foo;
create function f(foo) returns foo language sql return $1$0;
"), @"
    info: goto type definition
      ╭▸ file0.sql:2:13
      │
    2 │ create type foo;
      │             ─── 2. destination
    3 │ create function f(foo) returns foo language sql return $1;
      ╰╴                                                       ── 1. source
    ");
}

#[test]
fn param_builtin_type() {
    assert_snapshot!(goto("
create function f(x$0 int) returns int language sql return x;
"), @"
    info: goto type definition
          ╭▸ file0.sql:2:19
          │
        2 │ create function f(x int) returns int language sql return x;
          │                   ─ 1. source
          │
          ⸬  builtins.sql:32162:24
          │
    32162 │ create type pg_catalog.int4 (
          ╰╴                       ──── 2. destination
    ");
}

#[test]
fn param_table_type() {
    assert_snapshot!(goto("
create table t(a int);
create function f(r$0 t) returns int language sql return 1;
"), @"
    info: goto type definition
      ╭▸ file0.sql:2:14
      │
    2 │ create table t(a int);
      │              ─ 2. destination
    3 │ create function f(r t) returns int language sql return 1;
      ╰╴                  ─ 1. source
    ");
}

#[test]
fn param_in_select_column() {
    assert_snapshot!(goto("
create type foo;
create function f(x foo) returns foo language sql
begin atomic
  with c as (select x as y)
  select y$0 from c;
end;
"), @"
    info: goto type definition
      ╭▸ file0.sql:2:13
      │
    2 │ create type foo;
      │             ─── 2. destination
      ‡
    6 │   select y from c;
      ╰╴         ─ 1. source
    ");
}

#[test]
fn out_param_column() {
    assert_snapshot!(goto("
create type foo;
create function f(out x foo) language sql as 'select null';
select x$0 from f();
"), @"
    info: goto type definition
      ╭▸ file0.sql:2:13
      │
    2 │ create type foo;
      │             ─── 2. destination
    3 │ create function f(out x foo) language sql as 'select null';
    4 │ select x from f();
      ╰╴       ─ 1. source
    ");
}

#[test]
fn cte_def() {
    assert_snapshot!(goto("
with c$0 as (select 1 a)
select a from c;
"), @"
    info: goto type definition
      ╭▸ file0.sql:2:6
      │
    2 │ with c as (select 1 a)
      │      ┬
      │      │
      │      1. source
      ╰╴     2. destination
    ");
}

#[test]
fn cte_whole_row_ref() {
    assert_snapshot!(goto("
with c as (select 1 a)
select c$0 from c;
"), @"
    info: goto type definition
      ╭▸ file0.sql:2:6
      │
    2 │ with c as (select 1 a)
      │      ─ 2. destination
    3 │ select c from c;
      ╰╴       ─ 1. source
    ");
}

#[test]
fn cte_aliased_whole_row_ref() {
    assert_snapshot!(goto("
with c as (select 1 a)
select x$0 from c x;
"), @"
    info: goto type definition
      ╭▸ file0.sql:2:6
      │
    2 │ with c as (select 1 a)
      │      ─ 2. destination
    3 │ select x from c x;
      ╰╴       ─ 1. source
    ");
}

#[test]
fn cte_whole_row_through_cte() {
    assert_snapshot!(goto("
with c as (select 1 a), d as (select c from c)
select c$0 from d;
"), @"
    info: goto type definition
      ╭▸ file0.sql:2:6
      │
    2 │ with c as (select 1 a), d as (select c from c)
      │      ─ 2. destination
    3 │ select c from d;
      ╰╴       ─ 1. source
    ");
}

#[test]
fn create_function_sql_body_param() {
    assert_snapshot!(goto("
create type mood as enum ('happy', 'sad');
create function f(m mood) returns mood
  language sql
  as $$ select m$0 $$;
"), @"
    info: goto type definition
      ╭▸ file0.sql:2:13
      │
    2 │ create type mood as enum ('happy', 'sad');
      │             ──── 2. destination
      ‡
    5 │   as $$ select m $$;
      ╰╴               ─ 1. source
    ");
}

#[test]
fn partition_of_column_override_keeps_parent_type() {
    assert_snapshot!(goto("
create type ty as (x int);
create table p (a ty) partition by list (a);
create table c partition of p (a) default;
select a$0 from c;
"), @"
    info: goto type definition
      ╭▸ file0.sql:2:13
      │
    2 │ create type ty as (x int);
      │             ── 2. destination
      ‡
    5 │ select a from c;
      ╰╴       ─ 1. source
    ");
}

#[test]
fn array_column() {
    assert_snapshot!(goto("
create type pair as (x int, y int);
create table t (a pair[]);
select a$0 from t;
"), @"
    info: goto type definition
      ╭▸ file0.sql:2:13
      │
    2 │ create type pair as (x int, y int);
      │             ──── 2. destination
    3 │ create table t (a pair[]);
    4 │ select a from t;
      ╰╴       ─ 1. source
    ");
}

#[test]
fn array_of_table_column() {
    assert_snapshot!(goto("
create table u (b int);
create table t (a u[][]);
select a$0 from t;
"), @"
    info: goto type definition
      ╭▸ file0.sql:2:14
      │
    2 │ create table u (b int);
      │              ─ 2. destination
    3 │ create table t (a u[][]);
    4 │ select a from t;
      ╰╴       ─ 1. source
    ");
}

#[test]
fn cte_cycle_mark_default_bool() {
    assert_snapshot!(goto("
with recursive t(n) as (select 1 union all select n + 1 from t)
cycle n set is_cycle using path
select is_cycle$0 from t;
"), @"
    info: goto type definition
          ╭▸ file0.sql:4:8
          │
        4 │ select is_cycle from t;
          │        ──────── 1. source
          │
          ⸬  builtins.sql:32057:24
          │
    32057 │ create type pg_catalog.bool (
          ╰╴                       ──── 2. destination
    ");
}

#[test]
fn cte_search_depth_first_record_array() {
    assert_snapshot!(goto("
with recursive t(n) as (select 1 union all select n + 1 from t)
search depth first by n set ord
select ord$0 from t;
"), @"
    info: goto type definition
          ╭▸ file0.sql:4:8
          │
        4 │ select ord from t;
          │        ─── 1. source
          │
          ⸬  builtins.sql:32313:24
          │
    32313 │ create type pg_catalog.record (
          ╰╴                       ────── 2. destination
    ");
}

#[test]
fn function_call_return_type() {
    assert_snapshot!(goto("
create type t;
create function f() returns t language sql return null;
with c as (select f() a)
select a$0 from c;
"), @"
    info: goto type definition
      ╭▸ file0.sql:2:13
      │
    2 │ create type t;
      │             ─ 2. destination
      ‡
    5 │ select a from c;
      ╰╴       ─ 1. source
    ");
}

#[test]
fn prepared_statement_positional_param() {
    assert_snapshot!(goto("
create type e as enum ('a');
prepare p (e) as select $1$0;
"), @"
    info: goto type definition
      ╭▸ file0.sql:2:13
      │
    2 │ create type e as enum ('a');
      │             ─ 2. destination
    3 │ prepare p (e) as select $1;
      ╰╴                        ── 1. source
    ");
}

#[test]
fn domain_check_value() {
    assert_snapshot!(goto("
create type t as (a int);
create domain d as t check ((value$0).a > 0);
"), @"
    info: goto type definition
      ╭▸ file0.sql:2:13
      │
    2 │ create type t as (a int);
      │             ─ 2. destination
    3 │ create domain d as t check ((value).a > 0);
      ╰╴                             ───── 1. source
    ");
}

#[test]
fn with_ordinality_column_is_bigint() {
    assert_snapshot!(goto("
select ordinality$0 from unnest(array[1, 2]) with ordinality;
"), @"
    info: goto type definition
          ╭▸ file0.sql:2:8
          │
        2 │ select ordinality from unnest(array[1, 2]) with ordinality;
          │        ────────── 1. source
          │
          ⸬  builtins.sql:32168:24
          │
    32168 │ create type pg_catalog.int8 (
          ╰╴                       ──── 2. destination
    ");
}

#[test]
fn column_def_list_type() {
    assert_snapshot!(goto("
create type pair as (a int, b int);
create function f() returns record language sql as 'select 1';
select x$0 from f() as t(x pair);
"), @"
    info: goto type definition
      ╭▸ file0.sql:2:13
      │
    2 │ create type pair as (a int, b int);
      │             ──── 2. destination
    3 │ create function f() returns record language sql as 'select 1';
    4 │ select x from f() as t(x pair);
      ╰╴       ─ 1. source
    ");
}

#[test]
fn scalar_function_with_ordinality_alias_column() {
    assert_snapshot!(goto("
select x$0 from unnest(array[1, 2]) with ordinality as u(x, o);
"), @"
    info: goto type definition
          ╭▸ file0.sql:2:8
          │
        2 │ select x from unnest(array[1, 2]) with ordinality as u(x, o);
          │        ─ 1. source
          │
          ⸬  builtins.sql:32162:24
          │
    32162 │ create type pg_catalog.int4 (
          ╰╴                       ──── 2. destination
    ");
}

#[test]
fn rows_from_column_def_list_type() {
    assert_snapshot!(goto("
create type pair as (a int, b int);
create function f() returns record language sql as 'select 1';
select x$0 from rows from (f() as (x pair));
"), @"
    info: goto type definition
      ╭▸ file0.sql:2:13
      │
    2 │ create type pair as (a int, b int);
      │             ──── 2. destination
    3 │ create function f() returns record language sql as 'select 1';
    4 │ select x from rows from (f() as (x pair));
      ╰╴       ─ 1. source
    ");
}

#[test]
fn current_timestamp_keyword_type() {
    assert_snapshot!(goto("
with t as (select current_timestamp as ts)
select ts$0 from t;
"), @"
    info: goto type definition
          ╭▸ file0.sql:3:8
          │
        3 │ select ts from t;
          │        ── 1. source
          │
          ⸬  builtins.sql:32425:24
          │
    32425 │ create type pg_catalog.timestamptz (
          ╰╴                       ─────────── 2. destination
    ");
}

#[test]
fn at_time_zone_type() {
    assert_snapshot!(goto("
with t as (select timestamp '2026-01-01' at time zone 'UTC' as ts)
select ts$0 from t;
"), @"
    info: goto type definition
          ╭▸ file0.sql:3:8
          │
        3 │ select ts from t;
          │        ── 1. source
          │
          ⸬  builtins.sql:32425:24
          │
    32425 │ create type pg_catalog.timestamptz (
          ╰╴                       ─────────── 2. destination
    ");
}

#[test]
fn collation_for_type() {
    assert_snapshot!(goto("
with t as (select collation for ('x') as c)
select c$0 from t;
"), @"
    info: goto type definition
          ╭▸ file0.sql:3:8
          │
        3 │ select c from t;
          │        ─ 1. source
          │
          ⸬  builtins.sql:32396:24
          │
    32396 │ create type pg_catalog.text (
          ╰╴                       ──── 2. destination
    ");
}

#[test]
fn extract_type() {
    assert_snapshot!(goto("
with t as (select extract(year from date '2026-01-01') as y)
select y$0 from t;
"), @"
    info: goto type definition
          ╭▸ file0.sql:3:8
          │
        3 │ select y from t;
          │        ─ 1. source
          │
          ⸬  builtins.sql:32228:24
          │
    32228 │ create type pg_catalog.numeric (
          ╰╴                       ─────── 2. destination
    ");
}

#[test]
fn is_normalized_type() {
    assert_snapshot!(goto("
with t as (select 'abc' is normalized as n)
select n$0 from t;
"), @"
    info: goto type definition
          ╭▸ file0.sql:3:8
          │
        3 │ select n from t;
          │        ─ 1. source
          │
          ⸬  builtins.sql:32057:24
          │
    32057 │ create type pg_catalog.bool (
          ╰╴                       ──── 2. destination
    ");
}

#[test]
fn overlaps_type() {
    assert_snapshot!(goto("
with t as (select (date '2026-01-01', date '2026-01-02') overlaps (date '2026-01-02', date '2026-01-03') as o)
select o$0 from t;
"), @"
    info: goto type definition
          ╭▸ file0.sql:3:8
          │
        3 │ select o from t;
          │        ─ 1. source
          │
          ⸬  builtins.sql:32057:24
          │
    32057 │ create type pg_catalog.bool (
          ╰╴                       ──── 2. destination
    ");
}

#[test]
fn position_type() {
    assert_snapshot!(goto("
with t as (select position('b' in 'abc') as p)
select p$0 from t;
"), @"
    info: goto type definition
          ╭▸ file0.sql:3:8
          │
        3 │ select p from t;
          │        ─ 1. source
          │
          ⸬  builtins.sql:32162:24
          │
    32162 │ create type pg_catalog.int4 (
          ╰╴                       ──── 2. destination
    ");
}

#[test]
fn xmlexists_type() {
    assert_snapshot!(goto("
with t as (select xmlexists('//town' passing '<town/>'::xml) as e)
select e$0 from t;
"), @"
    info: goto type definition
          ╭▸ file0.sql:3:8
          │
        3 │ select e from t;
          │        ─ 1. source
          │
          ⸬  builtins.sql:32057:24
          │
    32057 │ create type pg_catalog.bool (
          ╰╴                       ──── 2. destination
    ");
}

#[test]
fn function_returns_table_column() {
    assert_snapshot!(goto("
create type foo;
create function f() returns table (a foo)
  language sql
  as 'select null';
select a$0 from f();
"), @"
    info: goto type definition
      ╭▸ file0.sql:2:13
      │
    2 │ create type foo;
      │             ─── 2. destination
      ‡
    6 │ select a from f();
      ╰╴       ─ 1. source
    ");
}

#[test]
fn graph_property() {
    assert_snapshot!(goto("
create table person (id int, name text);
create property graph g vertex tables (person properties (name as n));
select * from graph_table(g match (a) columns (a.n$0));
"), @"
    info: goto type definition
          ╭▸ file0.sql:4:50
          │
        4 │ select * from graph_table(g match (a) columns (a.n));
          │                                                  ─ 1. source
          │
          ⸬  builtins.sql:32396:24
          │
    32396 │ create type pg_catalog.text (
          ╰╴                       ──── 2. destination
    ");
}

#[test]
fn function_call() {
    assert_snapshot!(goto("
create function f() returns integer
  language sql
  return 1;
select f$0();
"), @"
    info: goto type definition
          ╭▸ file0.sql:5:8
          │
        5 │ select f();
          │        ─ 1. source
          │
          ⸬  builtins.sql:32162:24
          │
    32162 │ create type pg_catalog.int4 (
          ╰╴                       ──── 2. destination
    ");
}

#[test]
fn builtin_function_call() {
    assert_snapshot!(goto("select now$0();"), @"
    info: goto type definition
          ╭▸ file0.sql:1:8
          │
        1 │ select now();
          │        ─── 1. source
          │
          ⸬  builtins.sql:32425:24
          │
    32425 │ create type pg_catalog.timestamptz (
          ╰╴                       ─────────── 2. destination
    ");
}

#[test]
fn polymorphic_function_call() {
    assert_snapshot!(goto("select array_append$0(array[1], 2);"), @"
    info: goto type definition
          ╭▸ file0.sql:1:8
          │
        1 │ select array_append(array[1], 2);
          │        ──────────── 1. source
          │
          ⸬  builtins.sql:32162:24
          │
    32162 │ create type pg_catalog.int4 (
          ╰╴                       ──── 2. destination
    ");
}

#[test]
fn operator() {
    assert_snapshot!(goto("select 1 +$0 2;"), @"
    info: goto type definition
          ╭▸ file0.sql:1:10
          │
        1 │ select 1 + 2;
          │          ─ 1. source
          │
          ⸬  builtins.sql:32162:24
          │
    32162 │ create type pg_catalog.int4 (
          ╰╴                       ──── 2. destination
    ");
}

#[test]
fn custom_operator() {
    assert_snapshot!(goto("
create function f(integer, integer) returns text
  language sql
  return '';
create operator === (leftarg = integer, rightarg = integer, function = f);
select 1 ===$0 2;
"), @"
    info: goto type definition
          ╭▸ file0.sql:6:10
          │
        6 │ select 1 === 2;
          │          ─── 1. source
          │
          ⸬  builtins.sql:32396:24
          │
    32396 │ create type pg_catalog.text (
          ╰╴                       ──── 2. destination
    ");
}

#[test]
fn polymorphic_operator() {
    assert_snapshot!(goto("select array[1] ||$0 2;"), @"
    info: goto type definition
          ╭▸ file0.sql:1:17
          │
        1 │ select array[1] || 2;
          │                 ── 1. source
          │
          ⸬  builtins.sql:32162:24
          │
    32162 │ create type pg_catalog.int4 (
          ╰╴                       ──── 2. destination
    ");
}

#[test]
fn polymorphic_function_call_field() {
    assert_snapshot!(goto("
create type pair as (a int, b int);
create function f(anyelement) returns anyelement
  language sql
  return $1;
select (f$0(row(1, 2)::pair)).a;
"), @"
    info: goto type definition
      ╭▸ file0.sql:2:13
      │
    2 │ create type pair as (a int, b int);
      │             ──── 2. destination
      ‡
    6 │ select (f(row(1, 2)::pair)).a;
      ╰╴        ─ 1. source
    ");
}

#[test]
fn polymorphic_from_function() {
    assert_snapshot!(goto("select * from unnest$0(array[1, 2]);"), @"
    info: goto type definition
          ╭▸ file0.sql:1:15
          │
        1 │ select * from unnest(array[1, 2]);
          │               ────── 1. source
          │
          ⸬  builtins.sql:32162:24
          │
    32162 │ create type pg_catalog.int4 (
          ╰╴                       ──── 2. destination
    ");
}
