use insta::assert_snapshot;

use super::check;

#[test]
fn function_with_ordinality_columns_incomplete() {
    assert_snapshot!(check("
create function f() returns table (x int)
  language sql
  return null;
select x, ordinality from f() with ordinality;
"), @"no diagnostics");
}

#[test]
fn json_table_columns() {
    assert_snapshot!(check("
with data as (select '{}'::json as j, 1 as lim)
select id, n, e, o, nope, jt.n
  from data,
  json_table(
    data.j, '$ ? (@.n > $x)' passing data.lim as x, missing as y
    columns(
      id for ordinality,
      n int path '$.n',
      e bool exists path '$.e',
      nested path '$.xs[*]' columns(o text path '$')
    )
  ) as jt;
"), @"
    error: column `nope` does not exist
      ╭▸ 
    3 │ select id, n, e, o, nope, jt.n
      ╰╴                    ━━━━
    error: column `missing` does not exist
      ╭▸ 
    6 │     data.j, '$ ? (@.n > $x)' passing data.lim as x, missing as y
      ╰╴                                                    ━━━━━━━
    ");
}

#[test]
fn json_table_plan_unknown_path() {
    assert_snapshot!(check("
select *
from json_table(
  '{}'::jsonb,
  '$' as root
  columns (a int path '$')
  plan (root outer missing)
);
"), @"
    error: JSON path `missing` does not exist
      ╭▸ 
    7 │   plan (root outer missing)
      ╰╴                   ━━━━━━━
    ");
}

#[test]
fn unnest_multiple_args() {
    assert_snapshot!(check("
create table t (a int[], b text[]);
select x, y, nope
from t,
  unnest(a, b) as u (x, y)
where x > 0 and y = 'a';
"), @"
    error: column `nope` does not exist
      ╭▸ 
    3 │ select x, y, nope
      ╰╴             ━━━━
    ");
}

#[test]
fn unnest_multiple_args_rows_from() {
    assert_snapshot!(check("
create table t (a int[], b text[], c int[]);
select x, y, z
from t,
  rows from (unnest(a, b), unnest(c)) as u (x, y, z);
"), @"no diagnostics");
}

#[test]
fn unnest_multiple_args_with_ordinality() {
    assert_snapshot!(check("
select x, y, n
from unnest(array[1], array['a']) with ordinality as u (x, y, n);
"), @"no diagnostics");
}

#[test]
fn unnest_multiple_args_non_array() {
    assert_snapshot!(check("
select * from unnest(1, array[2]);
"), @"
    error: function `unnest` does not exist
      ╭▸ 
    2 │ select * from unnest(1, array[2]);
      ╰╴              ━━━━━━
    ");
}

#[test]
fn unnest_multiple_args_select_list() {
    assert_snapshot!(check("
select unnest(array[1], array[2]);
"), @"
    error: function `unnest(int4[], int4[])` does not exist
      ╭▸ 
    2 │ select unnest(array[1], array[2]);
      │        ━━━━━━
      │
      ╰ hint: consider adding explicit type casts
    ");
}

#[test]
fn unnest_multiple_args_column_def_list() {
    assert_snapshot!(check("
create table t (a int[], b text[]);
select x, y
from t,
  unnest(a, b) as u (x int, y text);
"), @"
    error: function `unnest(int4[], text[])` does not exist
      ╭▸ 
    5 │   unnest(a, b) as u (x int, y text);
      │   ━━━━━━
      │
      ╰ hint: consider adding explicit type casts
    ");
}

#[test]
fn unnest_multiple_args_composite_array() {
    assert_snapshot!(check("
create type pair as (x int, y text);
create table t (ps pair[], b int[]);
select x, y, unnest from t, unnest(ps, b);
select x, y from t, unnest(ps);
"), @"no diagnostics");
}
