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
    error: column `nope` does not exist
      ╭▸ 
    5 │ select t.nope, nope(t), t from t;
      ╰╴         ━━━━
    error: function `nope` does not exist
      ╭▸ 
    5 │ select t.nope, nope(t), t from t;
      ╰╴               ━━━━
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
    error: column `d` does not exist
      ╭▸ 
    5 │ select t.d, u.d from t, u;
      ╰╴         ━
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
    error: function `f` does not exist
       ╭▸ 
    18 │   (f()).x,
       ╰╴   ━
    error: operator `-` does not exist
       ╭▸ 
    19 │   (-p).x,
       ╰╴   ━
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
