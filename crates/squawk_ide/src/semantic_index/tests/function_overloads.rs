use insta::assert_snapshot;

use super::check;

#[test]
fn user_function_overloads_builtin() {
    assert_snapshot!(check("
create function lower(a int, b int) returns int language sql return a;
select lower('A'), lower(1, 2), lower(1, 2, 3);
"), @"
    error: function `lower` does not exist
      ╭▸ 
    3 │ select lower('A'), lower(1, 2), lower(1, 2, 3);
      ╰╴                                ━━━━━
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
