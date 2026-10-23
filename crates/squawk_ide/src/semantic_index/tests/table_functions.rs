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
    error: column `missing` does not exist
      ╭▸ 
    6 │     data.j, '$ ? (@.n > $x)' passing data.lim as x, missing as y
      ╰╴                                                    ━━━━━━━
    error: column `nope` does not exist
      ╭▸ 
    3 │ select id, n, e, o, nope, jt.n
      ╰╴                    ━━━━
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
