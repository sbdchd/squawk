use insta::assert_snapshot;

use super::check;

#[test]
fn domain_polymorphic_args() {
    assert_snapshot!(check("
create domain ints as int[];
create domain posint as int;
create function f(x anyarray) returns int language sql return 1;
create function g(x anynonarray) returns int language sql return 1;
create table t (a ints, b posint);
select f(a), f(b), g(a), g(b) from t;
"), @"
    error: function `f(posint)` does not exist
      ╭▸ 
    7 │ select f(a), f(b), g(a), g(b) from t;
      │              ━
      │
      ╰ hint: You might need to add explicit type casts.
    error: function `g(ints)` does not exist
      ╭▸ 
    7 │ select f(a), f(b), g(a), g(b) from t;
      │                    ━
      │
      ╰ hint: You might need to add explicit type casts.
    ");
}

#[test]
fn variadic_polymorphic_args() {
    assert_snapshot!(check("
create function f(variadic x anyarray) returns int language sql return 1;
create function g(variadic x anycompatiblearray) returns int language sql return 1;
create table t (a int, b bigint, c int[]);
select f(a, a), f(variadic c), g(a, b), g(variadic c) from t;
"), @"no diagnostics");
}

#[test]
fn polymorphic_args_must_agree() {
    assert_snapshot!(check("
create domain posint as int;
create function f(x anyelement, y anyelement) returns int language sql return 1;
create table t (a int, b text, c posint);
select f(a, a), f(a, b), f(a, 'x'), f(a, c) from t;
"), @"
    error: function `f(int4, text)` does not exist
      ╭▸ 
    5 │ select f(a, a), f(a, b), f(a, 'x'), f(a, c) from t;
      │                 ━
      │
      ╰ hint: You might need to add explicit type casts.
    error: function `f(int4, posint)` does not exist
      ╭▸ 
    5 │ select f(a, a), f(a, b), f(a, 'x'), f(a, c) from t;
      │                                     ━
      │
      ╰ hint: You might need to add explicit type casts.
    ");
}

#[test]
fn polymorphic_array_element_must_match() {
    assert_snapshot!(check("
create function f(x anyarray, y anyelement) returns int language sql return 1;
create table t (a int[], b int, c text, d int2vector, e smallint);
select f(a, b), f(a, c), f(d, e), f(d, b) from t;
"), @"
    error: function `f(int4[], text)` does not exist
      ╭▸ 
    4 │ select f(a, b), f(a, c), f(d, e), f(d, b) from t;
      │                 ━
      │
      ╰ hint: You might need to add explicit type casts.
    error: function `f(int2vector, int4)` does not exist
      ╭▸ 
    4 │ select f(a, b), f(a, c), f(d, e), f(d, b) from t;
      │                                   ━
      │
      ╰ hint: You might need to add explicit type casts.
    ");
}

#[test]
fn polymorphic_nonarray_and_enum() {
    assert_snapshot!(check("
create type mood as enum ('sad', 'happy');
create function f(x anynonarray, y anyarray) returns int language sql return 1;
create function g(x anyenum, y anyelement) returns int language sql return 1;
create table t (a int, b int[], m mood);
select f(a, b), g(m, m), g(m, a) from t;
"), @"
    error: function `g(mood, int4)` does not exist
      ╭▸ 
    6 │ select f(a, b), g(m, m), g(m, a) from t;
      │                          ━
      │
      ╰ hint: You might need to add explicit type casts.
    ");
}

#[test]
fn polymorphic_range_subtype() {
    assert_snapshot!(check("
create function f(x anyrange, y anyelement) returns int language sql return 1;
create function g(x anymultirange, y anyrange) returns int language sql return 1;
create table t (r int4range, m int4multirange, n nummultirange, a int, b text);
select f(r, a), f(r, b), g(m, r), g(n, r) from t;
"), @"
    error: function `f(int4range, text)` does not exist
      ╭▸ 
    5 │ select f(r, a), f(r, b), g(m, r), g(n, r) from t;
      │                 ━
      │
      ╰ hint: You might need to add explicit type casts.
    error: function `g(nummultirange, int4range)` does not exist
      ╭▸ 
    5 │ select f(r, a), f(r, b), g(m, r), g(n, r) from t;
      │                                   ━
      │
      ╰ hint: You might need to add explicit type casts.
    ");
}

#[test]
fn variadic_polymorphic_conflict() {
    assert_snapshot!(check("
create function f(variadic x anyarray) returns int language sql return 1;
create table t (a int, b text);
select f(a, a), f(a, b), f(a, 'x') from t;
"), @"
    error: function `f(int4, text)` does not exist
      ╭▸ 
    4 │ select f(a, a), f(a, b), f(a, 'x') from t;
      │                 ━
      │
      ╰ hint: You might need to add explicit type casts.
    ");
}

#[test]
fn anycompatible_common_type() {
    assert_snapshot!(check("
create function f(x anycompatible, y anycompatible) returns int language sql return 1;
create function g(x anycompatiblearray, y anycompatible) returns int language sql return 1;
create function h(x anycompatiblenonarray, y anycompatible) returns int language sql return 1;
create table t (a int, b bigint, c text, d int[], e int[]);
select f(a, b), f(a, c), f(a, 'x'), g(d, b), g(d, c), h(a, b), h(d, e) from t;
"), @"
    error: function `f(int4, text)` does not exist
      ╭▸ 
    6 │ select f(a, b), f(a, c), f(a, 'x'), g(d, b), g(d, c), h(a, b), h(d, e) from t;
      │                 ━
      │
      ╰ hint: You might need to add explicit type casts.
    error: function `g(int4[], text)` does not exist
      ╭▸ 
    6 │ select f(a, b), f(a, c), f(a, 'x'), g(d, b), g(d, c), h(a, b), h(d, e) from t;
      │                                              ━
      │
      ╰ hint: You might need to add explicit type casts.
    error: function `h(int4[], int4[])` does not exist
      ╭▸ 
    6 │ select f(a, b), f(a, c), f(a, 'x'), g(d, b), g(d, c), h(a, b), h(d, e) from t;
      │                                                                ━
      │
      ╰ hint: You might need to add explicit type casts.
    ");
}

#[test]
fn anycompatible_range() {
    assert_snapshot!(check("
create function f(x anycompatiblerange, y anycompatible) returns int language sql return 1;
create function g(x anycompatiblemultirange, y anycompatiblerange) returns int language sql return 1;
create table t (r int4range, m int4multirange, n nummultirange, a smallint, b bigint);
select f(r, a), f(r, b), g(m, r), g(n, r) from t;
"), @"
    error: function `f(int4range, int8)` does not exist
      ╭▸ 
    5 │ select f(r, a), f(r, b), g(m, r), g(n, r) from t;
      │                 ━
      │
      ╰ hint: You might need to add explicit type casts.
    error: function `g(nummultirange, int4range)` does not exist
      ╭▸ 
    5 │ select f(r, a), f(r, b), g(m, r), g(n, r) from t;
      │                                   ━
      │
      ╰ hint: You might need to add explicit type casts.
    ");
}

#[test]
fn variadic_anycompatible() {
    assert_snapshot!(check("
create function f(variadic x anycompatiblearray) returns int language sql return 1;
create table t (a int, b bigint, c text);
select f(a, b), f(a, c) from t;
"), @"
    error: function `f(int4, text)` does not exist
      ╭▸ 
    4 │ select f(a, b), f(a, c) from t;
      │                 ━
      │
      ╰ hint: You might need to add explicit type casts.
    ");
}

#[test]
fn polymorphic_return_types() {
    assert_snapshot!(check("
create function id(x anyelement) returns anyelement language sql return x;
create function wrap(x anyelement) returns anyarray language sql return array[x];
create function first(x anyarray) returns anyelement language sql return x[1];
create function pick(x anycompatible, y anycompatible) returns anycompatible language sql return x;
create function is_int(x int) returns int language sql return 1;
create function is_ints(x int[]) returns int language sql return 1;
create function is_bigint(x bigint) returns int language sql return 1;
create function is_text(x text) returns int language sql return 1;
create table t (a int, b bigint, d int[]);
select is_int(id(a)), is_ints(wrap(a)), is_int(first(d)), is_bigint(pick(a, b)),
  is_text(id(a)), is_text(first(d)), is_int(pick(a, b)) from t;
"), @"
    error: function `is_text(int4)` does not exist
       ╭▸ 
    12 │   is_text(id(a)), is_text(first(d)), is_int(pick(a, b)) from t;
       │   ━━━━━━━
       │
       ╰ hint: You might need to add explicit type casts.
    error: function `is_text(int4)` does not exist
       ╭▸ 
    12 │   is_text(id(a)), is_text(first(d)), is_int(pick(a, b)) from t;
       │                   ━━━━━━━
       │
       ╰ hint: You might need to add explicit type casts.
    error: function `is_int(int8)` does not exist
       ╭▸ 
    12 │   is_text(id(a)), is_text(first(d)), is_int(pick(a, b)) from t;
       │                                      ━━━━━━
       │
       ╰ hint: You might need to add explicit type casts.
    ");
}

#[test]
fn polymorphic_range_return_types() {
    assert_snapshot!(check("
create function is_int(x int) returns int language sql return 1;
create function is_mr(x int4multirange) returns int language sql return 1;
create function is_text(x text) returns int language sql return 1;
create table t (r int4range);
select is_int(lower(r)), is_mr(multirange(r)), is_text(lower(r)) from t;
"), @"
    error: function `is_text(int4)` does not exist
      ╭▸ 
    6 │ select is_int(lower(r)), is_mr(multirange(r)), is_text(lower(r)) from t;
      │                                                ━━━━━━━
      │
      ╰ hint: You might need to add explicit type casts.
    ");
}

#[test]
fn anycompatible_unknown_literals_return_text() {
    assert_snapshot!(check("
create function pick(x anycompatible, y anycompatible) returns anycompatible language sql return x;
create function is_text(x text) returns int language sql return 1;
create function is_int(x int) returns int language sql return 1;
select is_text(pick('a', 'b')), is_int(pick('a', 'b'));
"), @"
    error: function `is_int(text)` does not exist
      ╭▸ 
    5 │ select is_text(pick('a', 'b')), is_int(pick('a', 'b'));
      │                                 ━━━━━━
      │
      ╰ hint: You might need to add explicit type casts.
    ");
}

#[test]
fn variadic_polymorphic_return_type() {
    assert_snapshot!(check("
create function f(variadic x anyarray) returns anyarray language sql return x;
create function is_ints(x int[]) returns int language sql return 1;
create function is_int(x int) returns int language sql return 1;
create table t (a int);
select is_ints(f(a, a)), is_int(f(a, a)) from t;
"), @"
    error: function `is_int(int4[])` does not exist
      ╭▸ 
    6 │ select is_ints(f(a, a)), is_int(f(a, a)) from t;
      │                          ━━━━━━
      │
      ╰ hint: You might need to add explicit type casts.
    ");
}

#[test]
fn polymorphic_array_of_array() {
    assert_snapshot!(check("
create function f(variadic x anyarray) returns int language sql return 1;
create function g(variadic x anycompatiblearray) returns int language sql return 1;
create function h(x anyelement) returns anyarray language sql return array[x];
create function k(x anyelement, y anyarray) returns anyarray language sql return y;
create table t (a int, c int[]);
select f(c), f(variadic c), g(c), h(c), h(a), k(a, c) from t;
"), @"
    error: could not find array type for data type `int4[]`
      ╭▸ 
    7 │ select f(c), f(variadic c), g(c), h(c), h(a), k(a, c) from t;
      ╰╴       ━
    error: could not find array type for data type `int4[]`
      ╭▸ 
    7 │ select f(c), f(variadic c), g(c), h(c), h(a), k(a, c) from t;
      ╰╴                            ━
    error: could not find array type for data type `int4[]`
      ╭▸ 
    7 │ select f(c), f(variadic c), g(c), h(c), h(a), k(a, c) from t;
      ╰╴                                  ━
    ");
}

#[test]
fn create_aggregate_polymorphic_sfunc() {
    assert_snapshot!(check("
create aggregate agg(int) (sfunc = array_append, stype = int[], initcond = '{}');
create aggregate agg2(anycompatible) (sfunc = array_append, stype = anycompatiblearray);
"), @"no diagnostics");
}
