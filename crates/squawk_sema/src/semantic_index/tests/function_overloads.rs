use insta::assert_snapshot;

use super::check;

#[test]
fn user_function_overloads_builtin() {
    assert_snapshot!(check("
create function lower(a int, b int) returns int language sql return a;
select lower('A'), lower(1, 2), lower(1, 2, 3);
"), @"
    error: function `lower(int4, int4, int4)` does not exist
      ╭▸ 
    3 │ select lower('A'), lower(1, 2), lower(1, 2, 3);
      │                                 ━━━━━
      │
      ├ note: candidates are `lower(anymultirange)`, `lower(anyrange)`, `lower(text)`, `lower(int4, int4)`
      ╰ hint: consider adding explicit type casts
    ");
}

#[test]
fn ambiguous_function_call() {
    assert_snapshot!(check("
create function f(x int) returns int language sql return 1;
create function f(x bigint) returns int language sql return 2;
select f('1'), f(1);
"), @"
    error: function `f` is not unique
      ╭▸ 
    4 │ select f('1'), f(1);
      ╰╴       ━
    ");
}

#[test]
fn unknown_literals_narrow_past_mixed_categories() {
    assert_snapshot!(check("
create function f(a int4, b float8) returns int language sql return 1;
create function f(a bool, b int4) returns text language sql return '';
create function g(a int4, b text) returns int language sql return 1;
create function g(a bool, b varchar) returns text language sql return '';
create function h(a int4, b int4) returns int language sql return 1;
create function h(a bool, b int8) returns text language sql return '';
create function k(x int) returns int language sql return 1;
select k(f('1', '1')), k(g('1', '1')), h('1', '1');
"), @"
    error: function `h` is not unique
      ╭▸ 
    9 │ select k(f('1', '1')), k(g('1', '1')), h('1', '1');
      ╰╴                                       ━
    ");
}

#[test]
fn unknown_literal_assumes_known_type() {
    assert_snapshot!(check("
create function f(a int8, b int8) returns int language sql return 1;
create function f(a int8, b int2) returns text language sql return '';
create function k(x int) returns int language sql return 1;
create table t (x int4);
select k(f(x, '1')), f(x::int2, '1') from t;
"), @"
    error: function `f` is not unique
      ╭▸ 
    6 │ select k(f(x, '1')), f(x::int2, '1') from t;
      ╰╴                     ━
    ");
}

#[test]
fn xmlconcat_args_must_be_xml() {
    assert_snapshot!(check("
create domain d as xml;
create table t (
  a int,
  b text,
  c xml,
  e d
);
select xmlconcat(c, e, '<a/>', null), xmlconcat(a), xmlconcat(c, b) from t;
"), @"
    error: argument of `xmlconcat` must be type `xml`, not type `int4`
      ╭▸ 
    9 │ select xmlconcat(c, e, '<a/>', null), xmlconcat(a), xmlconcat(c, b) from t;
      ╰╴                                                ━
    error: argument of `xmlconcat` must be type `xml`, not type `text`
      ╭▸ 
    9 │ select xmlconcat(c, e, '<a/>', null), xmlconcat(a), xmlconcat(c, b) from t;
      ╰╴                                                                 ━
    ");
}

#[test]
fn unresolved_overload_candidates() {
    assert_snapshot!(check("
create function f(a int) returns int language sql return 1;
create function f(a text, b int default 1) returns int language sql return 1;
create function f2(a bool) returns int language sql return 1;
create procedure p(a int) language sql as '';
create procedure p(a text, out b int) language sql as '';
select f(true);
call p(true);
select to_char(true, 'x');
"), @"
    error: function `f(bool)` does not exist
      ╭▸ 
    7 │ select f(true);
      │        ━
      │
      ├ note: candidates are `f(int4)`, `f(text, int4)`
      ╰ hint: consider adding explicit type casts
    error: procedure `p(bool)` does not exist
      ╭▸ 
    8 │ call p(true);
      │      ━
      │
      ├ note: candidates are `p(int4)`, `p(text, int4)`
      ╰ hint: consider adding explicit type casts
    error: function `to_char(bool, unknown)` does not exist
      ╭▸ 
    9 │ select to_char(true, 'x');
      │        ━━━━━━━
      │
      ├ note: candidates are `to_char(interval, text)`, `to_char(numeric, text)`, `to_char(float4, text)`, `to_char(timestamptz, text)`, `to_char(timestamp, text)`, and 3 others
      ╰ hint: consider adding explicit type casts
    ");
}
