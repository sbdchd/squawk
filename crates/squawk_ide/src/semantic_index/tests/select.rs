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

#[test]
fn duplicate_from_source() {
    assert_snapshot!(check("
create schema s1;
create schema s2;
create table s1.t (a int);
create table s2.t (a int);
create table t (a int, b int);
select 1 from t, t;
select 1 from t x, t x;
select 1 from t join t using (a);
select 1 from s1.t, s2.t;
select 1 from (select 1) s, (select 2) s;
select 1 from (select 1), (select 2);
select 1 from generate_series(1, 2), generate_series(1, 2);
select 1 from (t x join t y using (a)) j, t j;
select 1 from (t x join t y using (a)) j, t x;
select 1 from t, lateral (select 1 from t) s;
with c as (select 1) select 1 from c, c;
"), @"
    error: table name `t` specified more than once
      ╭▸ 
    7 │ select 1 from t, t;
      ╰╴                 ━
    error: table name `x` specified more than once
      ╭▸ 
    8 │ select 1 from t x, t x;
      ╰╴                     ━
    error: table name `t` specified more than once
      ╭▸ 
    9 │ select 1 from t join t using (a);
      ╰╴                     ━
    error: table name `s` specified more than once
       ╭▸ 
    11 │ select 1 from (select 1) s, (select 2) s;
       ╰╴                                       ━
    error: table name `generate_series` specified more than once
       ╭▸ 
    13 │ select 1 from generate_series(1, 2), generate_series(1, 2);
       ╰╴                                     ━━━━━━━━━━━━━━━━━━━━━
    error: table name `j` specified more than once
       ╭▸ 
    14 │ select 1 from (t x join t y using (a)) j, t j;
       ╰╴                                            ━
    error: table name `c` specified more than once
       ╭▸ 
    17 │ with c as (select 1) select 1 from c, c;
       ╰╴                                      ━
    ");
}

#[test]
fn values_lists_different_lengths() {
    assert_snapshot!(check("
values (1), (1, 2), (3);
values (1, 2), (3, 4);
"), @"
    error: VALUES lists must all be the same length
      ╭▸ 
    2 │ values (1), (1, 2), (3);
      ╰╴            ━━━━━━
    ");
}

#[test]
fn compound_column_count_mismatch() {
    assert_snapshot!(check("
select 1 union select 1, 2;
select 1 intersect select 1, 2;
select 1, 2 except select 1;
select 1 union select 1 union all select 1, 2;
select 1 union (select 1, 2);
select 1 union values (1, 2);
select 1 union select 2;
select * from missing union select 1, 2;
"), @"
    error: relation `missing` does not exist
      ╭▸ 
    9 │ select * from missing union select 1, 2;
      ╰╴              ━━━━━━━
    error: each UNION query must have the same number of columns
      ╭▸ 
    2 │ select 1 union select 1, 2;
      ╰╴               ━━━━━━━━━━━
    error: each INTERSECT query must have the same number of columns
      ╭▸ 
    3 │ select 1 intersect select 1, 2;
      ╰╴                   ━━━━━━━━━━━
    error: each EXCEPT query must have the same number of columns
      ╭▸ 
    4 │ select 1, 2 except select 1;
      ╰╴                   ━━━━━━━━
    error: each UNION query must have the same number of columns
      ╭▸ 
    5 │ select 1 union select 1 union all select 1, 2;
      ╰╴                                  ━━━━━━━━━━━
    error: each UNION query must have the same number of columns
      ╭▸ 
    6 │ select 1 union (select 1, 2);
      ╰╴               ━━━━━━━━━━━━━
    error: each UNION query must have the same number of columns
      ╭▸ 
    7 │ select 1 union values (1, 2);
      ╰╴               ━━━━━━━━━━━━━
    ");
}

#[test]
fn from_alias_column_count_mismatch() {
    assert_snapshot!(check("
create table t (a int, b int);
select * from t x (p, q, r);
select * from (select 1) x (p, q);
select * from (values (1)) x (p, q);
select * from (t join t u using (a)) j (p, q, r, s);
select * from generate_series(1, 2) g (a, b);
select * from json_each('{}') j (k, v, extra);
select * from t x (p, q);
select * from t x (p);
select * from generate_series(1, 2) with ordinality g (a, b);
select * from unnest(array[1], array[2]) u (a, b);
select * from (t join t u using (a)) j (p, q, r);
select * from missing x (p, q, r);
"), @"
    error: relation `missing` does not exist
       ╭▸ 
    14 │ select * from missing x (p, q, r);
       ╰╴              ━━━━━━━
    error: table `x` has 2 columns available but 3 columns specified
      ╭▸ 
    3 │ select * from t x (p, q, r);
      ╰╴                         ━
    error: table `x` has 1 column available but 2 columns specified
      ╭▸ 
    4 │ select * from (select 1) x (p, q);
      ╰╴                               ━
    error: table `x` has 1 column available but 2 columns specified
      ╭▸ 
    5 │ select * from (values (1)) x (p, q);
      ╰╴                                 ━
    error: join expression `j` has 3 columns available but 4 columns specified
      ╭▸ 
    6 │ select * from (t join t u using (a)) j (p, q, r, s);
      ╰╴                                                 ━
    error: table `g` has 1 column available but 2 columns specified
      ╭▸ 
    7 │ select * from generate_series(1, 2) g (a, b);
      ╰╴                                          ━
    error: table `j` has 2 columns available but 3 columns specified
      ╭▸ 
    8 │ select * from json_each('{}') j (k, v, extra);
      ╰╴                                       ━━━━━
    ");
}
