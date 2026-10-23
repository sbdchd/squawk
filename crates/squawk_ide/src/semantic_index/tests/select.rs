use insta::assert_snapshot;

use super::check;

#[test]
fn star_expansion() {
    assert_snapshot!(check("
with t as (select 1 a), u as (select * from t)
select a, z from u;
"), @"
    error: column `z` does not exist
      ╭▸ 
    3 │ select a, z from u;
      ╰╴          ━
    ");
}

#[test]
fn qualified_column_unknown() {
    assert_snapshot!(check("
with t as (select 1 a)
select t.a, t.b, x.a from t;
"), @"
    error: column `b` does not exist
      ╭▸ 
    3 │ select t.a, t.b, x.a from t;
      ╰╴              ━
    error: missing FROM-clause entry for table `x`
      ╭▸ 
    3 │ select t.a, t.b, x.a from t;
      ╰╴                 ━
    ");
}

#[test]
fn undefined_table_columns_are_not_reported() {
    assert_snapshot!(check("
select a, b, foo.c from foo join bar on foo.id = bar.id;
"), @"
    error: relation `foo` does not exist
      ╭▸ 
    2 │ select a, b, foo.c from foo join bar on foo.id = bar.id;
      ╰╴                        ━━━
    error: relation `bar` does not exist
      ╭▸ 
    2 │ select a, b, foo.c from foo join bar on foo.id = bar.id;
      ╰╴                                 ━━━
    ");
}

#[test]
fn undefined_table() {
    assert_snapshot!(check("
select * from t;
"), @"
    error: relation `t` does not exist
      ╭▸ 
    2 │ select * from t;
      ╰╴              ━
    ");
}

#[test]
fn ambiguous_column() {
    assert_snapshot!(check("
with a as (select 1 id), b as (select 2 id)
select id from a, b;
"), @"
    error: column reference `id` is ambiguous
      ╭▸ 
    3 │ select id from a, b;
      ╰╴       ━━
    ");
}

#[test]
fn subquery_in_from() {
    assert_snapshot!(check("
select s.a, s.b from (select 1 a) s;
"), @"
    error: column `b` does not exist
      ╭▸ 
    2 │ select s.a, s.b from (select 1 a) s;
      ╰╴              ━
    ");
}

#[test]
fn from_alias_column_list() {
    assert_snapshot!(check("
with t as (select 1 a, 2 b)
select x.c, x.b, x.a from t as x(c);
"), @"
    error: column `a` does not exist
      ╭▸ 
    3 │ select x.c, x.b, x.a from t as x(c);
      ╰╴                   ━
    ");
}

#[test]
fn correlated_subquery() {
    assert_snapshot!(check("
with t as (select 1 a), u as (select 2 b)
select a from t where exists (select 1 from u where u.b = t.a and c = 1);
"), @"
    error: column `c` does not exist
      ╭▸ 
    3 │ select a from t where exists (select 1 from u where u.b = t.a and c = 1);
      ╰╴                                                                  ━
    ");
}

#[test]
fn order_by_output_name() {
    assert_snapshot!(check("
with t as (select 1 a)
select a as x from t order by x, a, y;
"), @"
    error: column `y` does not exist
      ╭▸ 
    3 │ select a as x from t order by x, a, y;
      ╰╴                                    ━
    ");
}

#[test]
fn distinct_on() {
    assert_snapshot!(check("
with t as (select 1 a, 2 b)
select distinct on (x, b, y) a as x from t order by x;
"), @"
    error: column `y` does not exist
      ╭▸ 
    3 │ select distinct on (x, b, y) a as x from t order by x;
      ╰╴                          ━
    ");
}

#[test]
fn compound_select_shares_with() {
    assert_snapshot!(check("
with t as (select 1 a)
select a from t union select b from t order by a;
"), @"
    error: column `b` does not exist
      ╭▸ 
    3 │ select a from t union select b from t order by a;
      ╰╴                             ━
    ");
}

#[test]
fn values_columns() {
    assert_snapshot!(check("
select column1, column3 from (values (1, 2)) v;
"), @"
    error: column `column3` does not exist
      ╭▸ 
    2 │ select column1, column3 from (values (1, 2)) v;
      ╰╴                ━━━━━━━
    ");
}

#[test]
fn function_names_are_not_columns() {
    assert_snapshot!(check("
with t as (select 1 a)
select count(a), now(), current_user from t;
"), @"no diagnostics");
}

#[test]
fn keyword_column_names() {
    assert_snapshot!(check("
create table users (name text, type text);
select name, type, value from users;
"), @"
    error: column `value` does not exist
      ╭▸ 
    3 │ select name, type, value from users;
      ╰╴                   ━━━━━
    ");
}

#[test]
fn from_alias_partial_column_list_unqualified() {
    assert_snapshot!(check("
with t as (
  select 1 a, 2 b, 3 c
)
select a, b, c, x, y from t as u (x, y);
"), @"
    error: column `a` does not exist
      ╭▸ 
    5 │ select a, b, c, x, y from t as u (x, y);
      ╰╴       ━
    error: column `b` does not exist
      ╭▸ 
    5 │ select a, b, c, x, y from t as u (x, y);
      ╰╴          ━
    ");
}

#[test]
fn select_for_update_of_unknown_relation() {
    assert_snapshot!(check("
create table t (a int);
select a from t as x for update of t;
"), @"
    error: missing FROM-clause entry for table `t`
      ╭▸ 
    3 │ select a from t as x for update of t;
      ╰╴                                   ━
    ");
}

#[test]
fn select_for_update_of_outer_relation() {
    assert_snapshot!(check("
create table t (a int);
create table u (b int);
select (select b from u for update of t) from t;
"), @"
    error: missing FROM-clause entry for table `t`
      ╭▸ 
    4 │ select (select b from u for update of t) from t;
      ╰╴                                      ━
    ");
}
