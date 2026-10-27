use insta::assert_snapshot;

use super::check;

#[test]
fn cte_unknown_column() {
    assert_snapshot!(check("
with t as (select 1 a)
select a, b from t;
"), @"
    error: column `b` does not exist
      ╭▸ 
    3 │ select a, b from t;
      │           ━
      ╰╴
    help: a column with a similar name exists
      ╭╴
    3 - select a, b from t;
    3 + select a, a from t;
      ╰╴
    ");
}

#[test]
fn cte_all_columns_found() {
    assert_snapshot!(check("
with t as (select 1 a, 2 as b)
select a, b, t.a from t;
"), @"no diagnostics");
}

#[test]
fn cte_column_list_renames() {
    assert_snapshot!(check("
with t(x, y) as (select 1 a, 2 b)
select x, y, a from t;
"), @"
    error: column `a` does not exist
      ╭▸ 
    3 │ select x, y, a from t;
      │              ━
      ╰╴
    help: a column with a similar name exists
      ╭╴
    3 - select x, y, a from t;
    3 + select x, y, x from t;
      ╰╴
    ");
}

#[test]
fn recursive_cte() {
    assert_snapshot!(check("
with recursive t(n) as (select 1 union all select n + 1 from t where n < 10)
select n, m from t;
"), @"
    error: column `m` does not exist
      ╭▸ 
    3 │ select n, m from t;
      │           ━
      ╰╴
    help: a column with a similar name exists
      ╭╴
    3 - select n, m from t;
    3 + select n, n from t;
      ╰╴
    ");
}

#[test]
fn recursive_cte_columns_from_non_recursive_term() {
    assert_snapshot!(check("
with recursive t as (select 1 as n union all select n + 1 from t where nope < 10)
select n, m from t;
"), @"
    error: column `nope` does not exist
      ╭▸ 
    2 │ with recursive t as (select 1 as n union all select n + 1 from t where nope < 10)
      ╰╴                                                                       ━━━━
    error: column `m` does not exist
      ╭▸ 
    3 │ select n, m from t;
      │           ━
      ╰╴
    help: a column with a similar name exists
      ╭╴
    3 - select n, m from t;
    3 + select n, n from t;
      ╰╴
    ");
}

#[test]
fn recursive_cte_columns_in_nested_subquery() {
    assert_snapshot!(check("
with recursive t as (
  select x as n from (select 1 x union select 2) s
  union all
  select n + 1 from t where n < 10
)
select n, x from t;
"), @"
    error: column `x` does not exist
      ╭▸ 
    7 │ select n, x from t;
      │           ━
      ╰╴
    help: a column with a similar name exists
      ╭╴
    7 - select n, x from t;
    7 + select n, n from t;
      ╰╴
    ");
}

#[test]
fn recursive_cte_parenthesized_body() {
    assert_snapshot!(check("
with recursive t as ((select 1 as n union all select n + 1 from t where nope < 10))
select n from t;
"), @"
    error: column `nope` does not exist
      ╭▸ 
    2 │ with recursive t as ((select 1 as n union all select n + 1 from t where nope < 10))
      ╰╴                                                                        ━━━━
    ");
}

#[test]
fn cte_parenthesized_body() {
    assert_snapshot!(check("
with t as (((select 1 a)))
select a, b from t;
"), @"
    error: column `b` does not exist
      ╭▸ 
    3 │ select a, b from t;
      │           ━
      ╰╴
    help: a column with a similar name exists
      ╭╴
    3 - select a, b from t;
    3 + select a, a from t;
      ╰╴
    ");
}

#[test]
fn cte_search_cycle_columns() {
    assert_snapshot!(check("
with recursive t(n) as (select 1 union all select n + 1 from t)
search depth first by n, m set ord
cycle n, x set is_cycle using path
select n, ord, is_cycle, path, z from t;
"), @"
    error: column `m` does not exist
      ╭▸ 
    3 │ search depth first by n, m set ord
      ╰╴                         ━
    error: column `x` does not exist
      ╭▸ 
    4 │ cycle n, x set is_cycle using path
      ╰╴         ━
    error: column `z` does not exist
      ╭▸ 
    5 │ select n, ord, is_cycle, path, z from t;
      │                                ━
      ╰╴
    help: a column with a similar name exists
      ╭╴
    5 - select n, ord, is_cycle, path, z from t;
    5 + select n, ord, is_cycle, path, n from t;
      ╰╴
    ");
}

#[test]
fn duplicate_column_names_are_ambiguous() {
    assert_snapshot!(check("
with t as (select 1 a, 2 a)
select a from t;
with t as (select 1 a, 2 a)
select t.a from t;
with t as (select 1 a, 2 a)
select * from t;
with a as (select 1 id), b as (select 2 id)
select id from (select * from a join b using (id)) s;
"), @"
    error: column reference `a` is ambiguous
      ╭▸ 
    3 │ select a from t;
      ╰╴       ━
    error: column reference `a` is ambiguous
      ╭▸ 
    5 │ select t.a from t;
      ╰╴         ━
    ");
}

#[test]
fn duplicate_cte_name() {
    assert_snapshot!(check("
with x as (select 1), x as (select 2) select 1;
with recursive x as (select 1), x as (select 2) select 1;
with x as (select 1) select 1 from (with x as (select 2) select 1) s;
"), @"
    error: `with` query name `x` specified more than once
      ╭▸ 
    2 │ with x as (select 1), x as (select 2) select 1;
      ╰╴                      ━
    error: `with` query name `x` specified more than once
      ╭▸ 
    3 │ with recursive x as (select 1), x as (select 2) select 1;
      ╰╴                                ━
    ");
}

#[test]
fn cte_column_count_mismatch() {
    assert_snapshot!(check("
with x (a, b) as (select 1) select * from x;
with recursive r (n, m) as (select 1 union all select n + 1 from r) select * from r;
with x (a) as (select 1, 2) select * from x;
with x (a, b) as (select * from missing) select * from x;
"), @"
    error: `with` query `x` has 1 column available but 2 columns specified
      ╭▸ 
    2 │ with x (a, b) as (select 1) select * from x;
      ╰╴           ━
    error: `with` query `r` has 1 column available but 2 columns specified
      ╭▸ 
    3 │ with recursive r (n, m) as (select 1 union all select n + 1 from r) select * from r;
      ╰╴                     ━
    error: relation `missing` does not exist
      ╭▸ 
    5 │ with x (a, b) as (select * from missing) select * from x;
      ╰╴                                ━━━━━━━
    ");
}
