use insta::assert_snapshot;

use super::check;

#[test]
fn column_and_function_notation() {
    assert_snapshot!(check("
create table t (a text);
create function d(t) returns text as 'select 1' language sql;
select a, t.a, a(t), t.d, d(t) from t;
select t.nope, nope(t), t from t;
"), @"
    error: column `t.nope` does not exist
      ╭▸ 
    5 │ select t.nope, nope(t), t from t;
      ╰╴         ━━━━
    error: function `nope(t)` does not exist
      ╭▸ 
    5 │ select t.nope, nope(t), t from t;
      │                ━━━━
      │
      ╰ hint: consider adding explicit type casts
    ");
}

#[test]
fn row_function_requires_matching_type() {
    assert_snapshot!(check("
create table t (a text);
create table u (b text);
create function d(u) returns text as 'select 1' language sql;
select t.d, u.d from t, u;
"), @"
    error: column `t.d` does not exist
      ╭▸ 
    5 │ select t.d, u.d from t, u;
      │          ━
      ╰╴
    help: a column with a similar name exists
      ╭╴
    5 - select t.d, u.d from t, u;
    5 + select t.a, u.d from t, u;
      ╰╴
    ");
}

#[test]
fn field_access_on_expressions() {
    assert_snapshot!(check(r#"
create type pair as (x int, y int);
create table t (p pair, ps pair[]);
select
  (p).x,
  (p).nope,
  (t).p,
  (t.p).y,
  (ps[1]).x,
  (ps[1:2]).x,
  (p::pair).y,
  ((p)).x,
  (row(1, 2)::pair).x,
  (case when true then p end).x,
  (p collate "C").x,
  (array[p]).x,
  (1 + 1).x,
  (f()).x,
  (-p).x,
  ((p).x).y
from t;
"#), @"
    error: column `nope` does not exist
      ╭▸ 
    6 │   (p).nope,
      ╰╴      ━━━━
    error: column `x` does not exist
       ╭▸ 
    10 │   (ps[1:2]).x,
       ╰╴            ━
    error: column `x` does not exist
       ╭▸ 
    16 │   (array[p]).x,
       ╰╴             ━
    error: function `f()` does not exist
       ╭▸ 
    18 │   (f()).x,
       ╰╴   ━
    error: operator does not exist: `- pair`
       ╭▸ 
    19 │   (-p).x,
       │    ━
       │
       ╰ hint: consider adding explicit type casts
    ");
}

#[test]
fn field_access_star_on_expressions() {
    assert_snapshot!(check("
create type pair as (x int, y int);
create table t (p pair);
select x, y, nope from (select (p).* from t) s;
"), @"
    error: column `nope` does not exist
      ╭▸ 
    4 │ select x, y, nope from (select (p).* from t) s;
      ╰╴             ━━━━
    ");
}

#[test]
fn paren_source_uses_column_aliases() {
    assert_snapshot!(check("
create table t (a int, b int);
select (t).x, (t).* from t as t(x, y);
select (t).a from t as t(x, y);
"), @"
    error: column `t.a` does not exist
      ╭▸ 
    4 │ select (t).a from t as t(x, y);
      │            ━
      ╰╴
    help: a column with a similar name exists
      ╭╴
    4 - select (t).a from t as t(x, y);
    4 + select (t).x from t as t(x, y);
      ╰╴
    ");
}

#[test]
fn paren_star_source_uses_column_aliases() {
    assert_snapshot!(check("
create table t (a int, b int);
select (t.*).x, ((t.*)).* from t as t(x, y);
select (t.*).a from t as t(x, y);
"), @"
    error: column `t.a` does not exist
      ╭▸ 
    4 │ select (t.*).a from t as t(x, y);
      │              ━
      ╰╴
    help: a column with a similar name exists
      ╭╴
    4 - select (t.*).a from t as t(x, y);
    4 + select (t.*).x from t as t(x, y);
      ╰╴
    ");
}

#[test]
fn paren_scalar_function_source_is_not_composite() {
    assert_snapshot!(check("
select f.x, (f).x, (f).* from generate_series(1, 2) as f(x);
select (f).f from generate_series(1, 2) as f;
select (f).x from unnest(array[1]) as f(x);
select (f).x from unnest(array[1], array[2]) as f(x, y);
select (f).x from generate_series(1, 2) with ordinality as f(x, n);
select (f).x from rows from (generate_series(1, 2)) as f(x);
"), @"
    error: column notation `.x` applied to type `int4`, which is not a composite type
      ╭▸ 
    2 │ select f.x, (f).x, (f).* from generate_series(1, 2) as f(x);
      ╰╴                ━
    error: type `int4` is not a composite type
      ╭▸ 
    2 │ select f.x, (f).x, (f).* from generate_series(1, 2) as f(x);
      ╰╴                       ━
    error: column notation `.f` applied to type `int4`, which is not a composite type
      ╭▸ 
    3 │ select (f).f from generate_series(1, 2) as f;
      ╰╴           ━
    error: column notation `.x` applied to type `int4`, which is not a composite type
      ╭▸ 
    4 │ select (f).x from unnest(array[1]) as f(x);
      ╰╴           ━
    error: column notation `.x` applied to type `int4`, which is not a composite type
      ╭▸ 
    7 │ select (f).x from rows from (generate_series(1, 2)) as f(x);
      ╰╴           ━
    ");
}

#[test]
fn record_field_selection_is_unchecked() {
    assert_snapshot!(check("
select (row(1, 2)).f1, ((1, 2)).f2;
"), @"no diagnostics");
}

#[test]
fn domain_over_composite_field_selection() {
    assert_snapshot!(check("
create type c as (x int);
create domain d as c;
create table u (dc d);
select (dc).x, (dc).*, (dc).zz from u;
"), @"
    error: column `zz` does not exist
      ╭▸ 
    5 │ select (dc).x, (dc).*, (dc).zz from u;
      ╰╴                            ━━
    ");
}
