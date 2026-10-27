use insta::assert_snapshot;

use super::check;

#[test]
fn insert_column_target_subscripts() {
    assert_snapshot!(check("
create table t(a int unique, d int[]);
insert into t(d[a], d[x:1]) values (1, 2);
insert into t values (1, '{}') on conflict (a) do update set d[excluded.a] = 1, d[1:y] = 2;
"), @"
    error: column `a` does not exist
      ╭▸ 
    3 │ insert into t(d[a], d[x:1]) values (1, 2);
      ╰╴                ━
    error: column `x` does not exist
      ╭▸ 
    3 │ insert into t(d[a], d[x:1]) values (1, 2);
      ╰╴                      ━
    error: column `y` does not exist
      ╭▸ 
    4 │ insert into t values (1, '{}') on conflict (a) do update set d[excluded.a] = 1, d[1:y] = 2;
      ╰╴                                                                                    ━
    ");
}

#[test]
fn update_column_target_subscripts() {
    assert_snapshot!(check("
create table t(a int, d int[]);
update t set d[a] = 1, d[x:1] = 2;
"), @"
    error: column `x` does not exist
      ╭▸ 
    3 │ update t set d[a] = 1, d[x:1] = 2;
      ╰╴                         ━
    ");
}

#[test]
fn vector_types_are_arrays() {
    assert_snapshot!(check("
create function f(x anyarray) returns int language sql return 1;
create function g(x anynonarray) returns int language sql return 1;
create table t (a int2vector, b oidvector);
select f(a), f(b), g(a), g(b) from t;
"), @"
    error: function `g` does not exist
      ╭▸ 
    5 │ select f(a), f(b), g(a), g(b) from t;
      ╰╴                   ━
    error: function `g` does not exist
      ╭▸ 
    5 │ select f(a), f(b), g(a), g(b) from t;
      ╰╴                         ━
    ");
}

#[test]
fn vector_types_coerce_to_arrays() {
    assert_snapshot!(check("
create function f(x int2[]) returns int language sql return 1;
create function g(x int4[]) returns int language sql return 1;
create function h(x oid[]) returns int language sql return 1;
create function i(x int2vector) returns int language sql return 1;
create function j(x oidvector) returns int language sql return 1;
create table t (a int2vector, b oidvector, c int2[], d oid[]);
select f(a), g(a), h(b), i(c), j(d) from t;
"), @"
    error: function `i` does not exist
      ╭▸ 
    8 │ select f(a), g(a), h(b), i(c), j(d) from t;
      ╰╴                         ━
    error: function `j` does not exist
      ╭▸ 
    8 │ select f(a), g(a), h(b), i(c), j(d) from t;
      ╰╴                               ━
    ");
}

#[test]
fn raw_array_subscripts() {
    assert_snapshot!(check(r#"
create function f(x float8) returns int language sql return 1;
create function g(x point) returns int language sql return 1;
create function h(x "char") returns int language sql return 1;
create table t (p point, l line, b box, s lseg, n name);
select f(p[0]), f(l[0]), g(b[0]), g(s[0]), h(n[0]), f(n[0]) from t;
"#), @"
    error: function `f` does not exist
      ╭▸ 
    6 │ select f(p[0]), f(l[0]), g(b[0]), g(s[0]), h(n[0]), f(n[0]) from t;
      ╰╴                                                    ━
    ");
}

#[test]
fn raw_array_types_are_not_arrays() {
    assert_snapshot!(check("
create function f(x anyarray) returns int language sql return 1;
create table t (p point);
select f(p) from t;
"), @"
    error: function `f` does not exist
      ╭▸ 
    4 │ select f(p) from t;
      ╰╴       ━
    ");
}

#[test]
fn raw_array_slices() {
    assert_snapshot!(check("
create function f(x point) returns int language sql return 1;
create function g(x float8) returns int language sql return 1;
create table t (p point);
select f(p[0:1]), g(p[0:1]) from t;
"), @"
    error: function `g` does not exist
      ╭▸ 
    5 │ select f(p[0:1]), g(p[0:1]) from t;
      ╰╴                  ━
    ");
}

#[test]
fn vector_type_slices() {
    assert_snapshot!(check("
create function f(x smallint[]) returns int language sql return 1;
create function g(x int2vector) returns int language sql return 1;
create table t (a int2vector);
select f(a[0:1]), g(a[0:1]) from t;
"), @"
    error: function `g` does not exist
      ╭▸ 
    5 │ select f(a[0:1]), g(a[0:1]) from t;
      ╰╴                  ━
    ");
}

#[test]
fn domain_over_array_subscripts() {
    assert_snapshot!(check("
create domain ints as int[];
create domain more_ints as ints;
create function f(x int) returns int language sql return 1;
create function g(x int[]) returns int language sql return 1;
create table t (a ints, b more_ints);
select f(a[1]), f(b[1]), g(a[1:2]), g(b[1:2]), g(a[1]) from t;
"), @"
    error: function `g` does not exist
      ╭▸ 
    7 │ select f(a[1]), f(b[1]), g(a[1:2]), g(b[1:2]), g(a[1]) from t;
      ╰╴                                               ━
    ");
}

#[test]
fn jsonb_subscripts() {
    assert_snapshot!(check("
create function f(x jsonb) returns int language sql return 1;
create function g(x int) returns int language sql return 1;
create table t (j jsonb);
select f(j['a']), f(j['a']['b']), g(j['a']), f(j['a':'b']) from t;
"), @"
    error: function `g` does not exist
      ╭▸ 
    5 │ select f(j['a']), f(j['a']['b']), g(j['a']), f(j['a':'b']) from t;
      ╰╴                                  ━
    ");
}

#[test]
fn generated_subscript_comment() {
    assert_snapshot!(check("
create type hstore;
create function hstore_in(cstring) returns hstore language c as 'hstore';
create function hstore_out(hstore) returns cstring language c as 'hstore';
create function hstore_subscript_handler(internal) returns internal language c as 'hstore';
-- subscript: pg_catalog.text
create type hstore (input = hstore_in, output = hstore_out, subscript = hstore_subscript_handler);
create function f(x text) returns int language sql return 1;
create function g(x hstore) returns int language sql return 1;
create table t (h hstore);
select f(h['k']), g(h['k']) from t;
"), @"
    error: function `g` does not exist
       ╭▸ 
    11 │ select f(h['k']), g(h['k']) from t;
       ╰╴                  ━
    ");
}

#[test]
fn generated_subscript_comment_quoted() {
    assert_snapshot!(check(r#"
create type t;
create function t_in(cstring) returns t language c as 't';
create function t_out(t) returns cstring language c as 't';
create function t_subscript(internal) returns internal language c as 't';
-- subscript: pg_catalog."char"
-- subscript slice: pg_catalog.int4[]
create type t (input = t_in, output = t_out, subscript = t_subscript);
create function f(x "char") returns int language sql return 1;
create function g(x int[]) returns int language sql return 1;
create table u (a t);
select f(a[1]), g(a[1:2]), f(a[1:2]) from u;
"#), @"
    error: function `f` does not exist
       ╭▸ 
    12 │ select f(a[1]), g(a[1:2]), f(a[1:2]) from u;
       ╰╴                           ━
    ");
}

#[test]
fn subscript_handler_outside_pg_catalog() {
    assert_snapshot!(check("
create function array_subscript_handler(internal) returns internal language c as 'x';
create type t (input = int4in, output = int4out, element = int, subscript = public.array_subscript_handler);
create function f(x anyarray) returns int language sql return 1;
create function g(x int) returns int language sql return 1;
create table u (a t);
select f(a), g(a[0]) from u;
"), @"
    error: function `f` does not exist
      ╭▸ 
    7 │ select f(a), g(a[0]) from u;
      ╰╴       ━
    ");
}

#[test]
fn vector_type_subscripts() {
    assert_snapshot!(check("
create function f(x smallint) returns int language sql return 1;
create table t (a int2vector, b oidvector);
select f(a[0]), f(b[0]) from t;
"), @"
    error: function `f` does not exist
      ╭▸ 
    4 │ select f(a[0]), f(b[0]) from t;
      ╰╴                ━
    ");
}

#[test]
fn alter_type_set_subscript() {
    assert_snapshot!(check("
create type t;
create function t_in(cstring) returns t language c as 'x';
create function t_out(t) returns cstring language c as 'x';
create type t (input = t_in, output = t_out, element = int4);
create function g(text) returns int language sql return 1;
create table x (v t);
select g(v[1]) from x;
alter type t set (subscript = array_subscript_handler);
select g(v[1]), g(v[1:2]) from x;
begin;
alter type t set (subscript = none);
rollback;
alter type t set (receive = none);
select g(v[1]) from x;
alter type t set (subscript = none);
select g(v[1]) from x;
"), @"
    error: function `g` does not exist
       ╭▸ 
    10 │ select g(v[1]), g(v[1:2]) from x;
       ╰╴       ━
    error: function `g` does not exist
       ╭▸ 
    10 │ select g(v[1]), g(v[1:2]) from x;
       ╰╴                ━
    error: function `g` does not exist
       ╭▸ 
    15 │ select g(v[1]) from x;
       ╰╴       ━
    ");
}
