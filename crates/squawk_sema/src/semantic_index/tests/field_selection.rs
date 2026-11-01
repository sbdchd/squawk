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
    error: column notation `.x` applied to type `pair[]`, which is not a composite type
       ╭▸ 
    10 │   (ps[1:2]).x,
       ╰╴            ━
    error: column notation `.x` applied to type `pair[]`, which is not a composite type
       ╭▸ 
    16 │   (array[p]).x,
       ╰╴             ━
    error: column notation `.x` applied to type `int4`, which is not a composite type
       ╭▸ 
    17 │   (1 + 1).x,
       ╰╴          ━
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
    error: column notation `.y` applied to type `int4`, which is not a composite type
       ╭▸ 
    20 │   ((p).x).y
       ╰╴          ━
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

#[test]
fn array_field_selection_is_not_composite() {
    assert_snapshot!(check("
create table u (arr int[]);
select (arr).zz, (arr).* from u;
"), @"
    error: column notation `.zz` applied to type `int4[]`, which is not a composite type
      ╭▸ 
    3 │ select (arr).zz, (arr).* from u;
      ╰╴             ━━
    error: type `int4[]` is not a composite type
      ╭▸ 
    3 │ select (arr).zz, (arr).* from u;
      ╰╴                       ━
    ");
}

#[test]
fn cte_scalar_column_is_not_composite() {
    assert_snapshot!(check("
with t as (select 1 a)
select (a).f, (a).* from t;
"), @"
    error: column notation `.f` applied to type `int4`, which is not a composite type
      ╭▸ 
    3 │ select (a).f, (a).* from t;
      ╰╴           ━
    error: type `int4` is not a composite type
      ╭▸ 
    3 │ select (a).f, (a).* from t;
      ╰╴                  ━
    ");
}

#[test]
fn unknown_field_selection_is_not_composite() {
    assert_snapshot!(check("
select (null).zz;
"), @"
    error: column notation `.zz` applied to type `unknown`, which is not a composite type
      ╭▸ 
    2 │ select (null).zz;
      ╰╴              ━━
    ");
}

#[test]
fn untyped_composite_field_selection() {
    assert_snapshot!(check("
create table t (a int, b int);
prepare p(t) as select ($1).zz;
select (coalesce(t, t)).zz, (case when true then t end).zz from t;
"), @"
    error: column `zz` does not exist
      ╭▸ 
    3 │ prepare p(t) as select ($1).zz;
      ╰╴                            ━━
    error: column `zz` does not exist
      ╭▸ 
    4 │ select (coalesce(t, t)).zz, (case when true then t end).zz from t;
      ╰╴                        ━━
    error: column `zz` does not exist
      ╭▸ 
    4 │ select (coalesce(t, t)).zz, (case when true then t end).zz from t;
      ╰╴                                                        ━━
    ");
}

#[test]
fn greatest_least_field_selection() {
    assert_snapshot!(check("
create table t (a int, b int);
select (greatest(1, 2)).x, (least(1, 2)).x;
select (greatest(t, t)).zz, (least(t, t)).zz from t;
"), @"
    error: column notation `.x` applied to type `int4`, which is not a composite type
      ╭▸ 
    3 │ select (greatest(1, 2)).x, (least(1, 2)).x;
      ╰╴                        ━
    error: column notation `.x` applied to type `int4`, which is not a composite type
      ╭▸ 
    3 │ select (greatest(1, 2)).x, (least(1, 2)).x;
      ╰╴                                         ━
    error: column `zz` does not exist
      ╭▸ 
    4 │ select (greatest(t, t)).zz, (least(t, t)).zz from t;
      ╰╴                        ━━
    error: column `zz` does not exist
      ╭▸ 
    4 │ select (greatest(t, t)).zz, (least(t, t)).zz from t;
      ╰╴                                          ━━
    ");
}

#[test]
fn nullif_field_selection() {
    assert_snapshot!(check("
create table t (a int, b int);
select (nullif(1, 2)).x, (nullif(t, t)).zz from t;
"), @"
    error: column notation `.x` applied to type `int4`, which is not a composite type
      ╭▸ 
    3 │ select (nullif(1, 2)).x, (nullif(t, t)).zz from t;
      ╰╴                      ━
    error: column `zz` does not exist
      ╭▸ 
    3 │ select (nullif(1, 2)).x, (nullif(t, t)).zz from t;
      ╰╴                                        ━━
    ");
}

#[test]
fn unknown_literal_branches_resolve_to_text() {
    assert_snapshot!(check("
select (coalesce('a', 'b')).x, (coalesce(null, null)).x;
select (case when true then 'a' end).x, (case when true then null end).*;
"), @"
    error: column notation `.x` applied to type `text`, which is not a composite type
      ╭▸ 
    2 │ select (coalesce('a', 'b')).x, (coalesce(null, null)).x;
      ╰╴                            ━
    error: column notation `.x` applied to type `text`, which is not a composite type
      ╭▸ 
    2 │ select (coalesce('a', 'b')).x, (coalesce(null, null)).x;
      ╰╴                                                      ━
    error: column notation `.x` applied to type `text`, which is not a composite type
      ╭▸ 
    3 │ select (case when true then 'a' end).x, (case when true then null end).*;
      ╰╴                                     ━
    error: type `text` is not a composite type
      ╭▸ 
    3 │ select (case when true then 'a' end).x, (case when true then null end).*;
      ╰╴                                                                       ━
    ");
}

#[test]
fn nullif_unknown_first_arg_uses_other_type() {
    assert_snapshot!(check("
select (nullif(null, 1)).x, (nullif('1', 1)).x;
"), @"
    error: column notation `.x` applied to type `int4`, which is not a composite type
      ╭▸ 
    2 │ select (nullif(null, 1)).x, (nullif('1', 1)).x;
      ╰╴                         ━
    error: column notation `.x` applied to type `int4`, which is not a composite type
      ╭▸ 
    2 │ select (nullif(null, 1)).x, (nullif('1', 1)).x;
      ╰╴                                             ━
    ");
}

#[test]
fn grouping_field_selection() {
    assert_snapshot!(check("
create table t (a int, b int);
select (grouping(a)).x from t group by a;
"), @"
    error: column notation `.x` applied to type `int4`, which is not a composite type
      ╭▸ 
    3 │ select (grouping(a)).x from t group by a;
      ╰╴                     ━
    ");
}

#[test]
fn merge_action_field_selection() {
    assert_snapshot!(check("
create table t (a int, b int);
merge into t
using t as s on t.a = s.a
when matched then
  delete
returning (merge_action()).x;
"), @"
    error: column notation `.x` applied to type `text`, which is not a composite type
      ╭▸ 
    7 │ returning (merge_action()).x;
      ╰╴                           ━
    ");
}
