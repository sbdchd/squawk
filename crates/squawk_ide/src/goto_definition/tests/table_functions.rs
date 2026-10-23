use insta::assert_snapshot;

use super::goto;

#[test]
fn function_in_from() {
    assert_snapshot!(goto("
create function f() returns table (x int)
  language sql
  return null;
select x from f$0();
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create function f() returns table (x int)
      │                 ─ 2. destination
      ‡
    5 │ select x from f();
      ╰╴              ─ 1. source
    ");
}

#[test]
fn function_table_column() {
    assert_snapshot!(goto("
create function f() returns table (x int)
  language sql
  return null;
select t.x$0 from f() t;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:36
      │
    2 │ create function f() returns table (x int)
      │                                    ─ 2. destination
      ‡
    5 │ select t.x from f() t;
      ╰╴         ─ 1. source
    ");
}

#[test]
fn function_source_qualifier() {
    assert_snapshot!(goto("
create function f() returns table (x int)
  language sql
  return null;
select f$0.x from f();
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create function f() returns table (x int)
      │                 ─ 2. destination
      ‡
    5 │ select f.x from f();
      ╰╴       ─ 1. source
    ");
}

#[test]
fn json_table_nested_column() {
    assert_snapshot!(goto(r#"
with data as (
  select '{"company": "acme"}'::json as company_json
)
select company, department, employee$0, salary
from data,
  json_table(
    data.company_json,
    '$'
    columns (
      company text path '$.company',
      nested path '$.departments[*]'
      columns (
        department text path '$.name',
        nested path '$.employees[*]'
        columns (employee text path '$.name', salary numeric path '$.salary')
      )
    )
  ) as jt;
"#), @"
    info: goto definition
       ╭▸ file0.sql:5:29
       │
     5 │ select company, department, employee, salary
       │                             ──────── 1. source
       ‡
    16 │         columns (employee text path '$.name', salary numeric path '$.salary')
       ╰╴                 ──────── 2. destination
    ");
}

#[test]
fn json_table_lateral_column() {
    assert_snapshot!(goto("
with data as (
  select '{}'::json as j
)
select * from data, json_table(data.j$0, '$' columns (n int path '$.n')) as jt;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:24
      │
    3 │   select '{}'::json as j
      │                        ─ 2. destination
    4 │ )
    5 │ select * from data, json_table(data.j, '$' columns (n int path '$.n')) as jt;
      ╰╴                                    ─ 1. source
    ");
}

#[test]
fn with_ordinality_column() {
    assert_snapshot!(goto("
select ordinality$0 from unnest(array[1, 2]) with ordinality;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:8
      │
    2 │ select ordinality from unnest(array[1, 2]) with ordinality;
      ╰╴       ────────── 1. source                ─────────────── 2. destination
    ");
}

#[test]
fn with_ordinality_after_out_params() {
    assert_snapshot!(goto("
create function f(out a int, out b int) returns setof record
  language sql
  as 'select 1, 2';
select x.ordinality$0 from f() with ordinality as x;
"), @"
    info: goto definition
      ╭▸ file0.sql:5:10
      │
    5 │ select x.ordinality from f() with ordinality as x;
      │          ┬─────────          ─────────────── 2. destination
      │          │
      ╰╴         1. source
    ");
}

#[test]
fn with_ordinality_out_param_column() {
    assert_snapshot!(goto("
create function f(out a int, out b int) returns setof record
  language sql
  as 'select 1, 2';
select b$0 from f() with ordinality;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:34
      │
    2 │ create function f(out a int, out b int) returns setof record
      │                                  ─ 2. destination
      ‡
    5 │ select b from f() with ordinality;
      ╰╴       ─ 1. source
    ");
}

#[test]
fn with_ordinality_star_ordinal() {
    assert_snapshot!(goto("
create function f(out a int, out b int) returns setof record
  language sql
  as 'select 1, 2';
select * from f() with ordinality order by 3$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:5:19
      │
    5 │ select * from f() with ordinality order by 3;
      │                   ┬──────────────          ─ 1. source
      │                   │
      ╰╴                  2. destination
    ");
}

#[test]
fn with_ordinality_renamed_by_alias() {
    assert_snapshot!(goto("
create function f(out a int, out b int) returns setof record
  language sql
  as 'select 1, 2';
select n$0 from f() with ordinality as x (c, d, n);
"), @"
    info: goto definition
      ╭▸ file0.sql:5:8
      │
    5 │ select n from f() with ordinality as x (c, d, n);
      ╰╴       ─ 1. source                            ─ 2. destination
    ");
}

#[test]
fn rows_from_with_ordinality_column() {
    assert_snapshot!(goto("
select ordinality$0 from rows from (unnest(array[1, 2])) with ordinality;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:8
      │
    2 │ select ordinality from rows from (unnest(array[1, 2])) with ordinality;
      ╰╴       ────────── 1. source                            ─────────────── 2. destination
    ");
}

#[test]
fn xmltable_ordinality_column() {
    assert_snapshot!(goto("
select n$0
from xmltable(
  '/r' passing '<r/>'::xml
  columns
    n for ordinality,
    v text path 'v'
);
"), @"
    info: goto definition
      ╭▸ file0.sql:2:8
      │
    2 │ select n
      │        ─ 1. source
      ‡
    6 │     n for ordinality,
      ╰╴    ─ 2. destination
    ");
}

#[test]
fn record_function_column_def_list() {
    assert_snapshot!(goto("
create function f() returns record
  language sql
  as 'select 1';
select x$0 from f() as t (x int);
"), @"
    info: goto definition
      ╭▸ file0.sql:5:8
      │
    5 │ select x from f() as t (x int);
      ╰╴       ─ 1. source      ─ 2. destination
    ");
}

#[test]
fn function_returns_setof_table_column() {
    assert_snapshot!(goto("
create table t (
  a int,
  b int
);
create function f() returns setof t
  language sql
  as 'select * from t';
select b$0 from f();
"), @"
    info: goto definition
      ╭▸ file0.sql:4:3
      │
    4 │   b int
      │   ─ 2. destination
      ‡
    9 │ select b from f();
      ╰╴       ─ 1. source
    ");
}

#[test]
fn scalar_function_column() {
    assert_snapshot!(goto("
select unnest$0 from unnest(array[1, 2]);
"), @"
    info: goto definition
      ╭▸ file0.sql:2:8
      │
    2 │ select unnest from unnest(array[1, 2]);
      │        ┬─────      ────── 2. destination
      │        │
      ╰╴       1. source
    ");
}

#[test]
fn scalar_function_column_named_by_alias() {
    assert_snapshot!(goto("
select u$0 from unnest(array[1, 2]) as u;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:8
      │
    2 │ select u from unnest(array[1, 2]) as u;
      ╰╴       ─ 1. source                   ─ 2. destination
    ");
}

#[test]
fn scalar_function_with_ordinality_alias_columns() {
    assert_snapshot!(goto("
select * from unnest(array[1, 2]) with ordinality as u (x, o) order by 2$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:60
      │
    2 │ select * from unnest(array[1, 2]) with ordinality as u (x, o) order by 2;
      │                                                            ┬           ─ 1. source
      │                                                            │
      ╰╴                                                           2. destination
    ");
}

#[test]
fn rows_from_columns_across_items() {
    assert_snapshot!(goto("
create function f() returns table (a int, b int)
  language sql
  as 'select 1, 2';
select z.c$0 from rows from (unnest(array[1, 2]), f()) as z (u, a, c);
"), @"
    info: goto definition
      ╭▸ file0.sql:5:10
      │
    5 │ select z.c from rows from (unnest(array[1, 2]), f()) as z (u, a, c);
      ╰╴         ─ 1. source                                             ─ 2. destination
    ");
}

#[test]
fn json_table_plan_root_path() {
    assert_snapshot!(goto("
select *
from json_table(
  '{}'::jsonb,
  '$' as root
  columns (nested path '$.items[*]' as items columns (value text path '$'))
  plan (ro$0ot outer items)
);
"), @"
    info: goto definition
      ╭▸ file0.sql:5:10
      │
    5 │   '$' as root
      │          ──── 2. destination
    6 │   columns (nested path '$.items[*]' as items columns (value text path '$'))
    7 │   plan (root outer items)
      ╰╴        ──── 1. source
    ");
}

#[test]
fn json_table_plan_nested_path() {
    assert_snapshot!(goto("
select *
from json_table(
  '{}'::jsonb,
  '$' as root
  columns (nested path '$.items[*]' as items columns (value text path '$'))
  plan (root outer ite$0ms)
);
"), @"
    info: goto definition
      ╭▸ file0.sql:6:40
      │
    6 │   columns (nested path '$.items[*]' as items columns (value text path '$'))
      │                                        ───── 2. destination
    7 │   plan (root outer items)
      ╰╴                   ───── 1. source
    ");
}
