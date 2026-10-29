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
      │           ━
      ╰╴
    help: a column with a similar name exists
      ╭╴
    3 - select a, z from u;
    3 + select a, a from u;
      ╰╴
    ");
}

#[test]
fn qualified_column_unknown() {
    assert_snapshot!(check("
with t as (select 1 a)
select t.a, t.b, x.a from t;
"), @"
    error: column `t.b` does not exist
      ╭▸ 
    3 │ select t.a, t.b, x.a from t;
      ╰╴              ━
    error: missing `from` clause entry for table `x`
      ╭▸ 
    3 │ select t.a, t.b, x.a from t;
      │                  ━
      ╰╴
    help: a relation with a similar name exists
      ╭╴
    3 - select t.a, t.b, x.a from t;
    3 + select t.a, t.b, t.a from t;
      ╰╴
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
    error: column `s.b` does not exist
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
    error: column `x.a` does not exist
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
      │                                                                   ━
      ╰╴
    help: a column with a similar name exists
      ╭╴
    3 - select a from t where exists (select 1 from u where u.b = t.a and c = 1);
    3 + select a from t where exists (select 1 from u where u.b = t.a and b = 1);
      ╰╴
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
      │                                     ━
      ╰╴
    help: a column with a similar name exists
      ╭╴
    3 - select a as x from t order by x, a, y;
    3 + select a as x from t order by x, a, a;
      ╰╴
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
      │                           ━
      ╰╴
    help: a column with a similar name exists
      ╭╴
    3 - select distinct on (x, b, y) a as x from t order by x;
    3 + select distinct on (x, b, a) a as x from t order by x;
      ╰╴
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
      │                              ━
      ╰╴
    help: a column with a similar name exists
      ╭╴
    3 - select a from t union select b from t order by a;
    3 + select a from t union select a from t order by a;
      ╰╴
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
      │                 ━━━━━━━
      ╰╴
    help: a column with a similar name exists
      ╭╴
    2 - select column1, column3 from (values (1, 2)) v;
    2 + select column1, column1 from (values (1, 2)) v;
      ╰╴
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
      │        ━
      ╰╴
    help: a column with a similar name exists
      ╭╴
    5 - select a, b, c, x, y from t as u (x, y);
    5 + select x, b, c, x, y from t as u (x, y);
      ╰╴
    error: column `b` does not exist
      ╭▸ 
    5 │ select a, b, c, x, y from t as u (x, y);
      │           ━
      ╰╴
    help: a column with a similar name exists
      ╭╴
    5 - select a, b, c, x, y from t as u (x, y);
    5 + select a, x, c, x, y from t as u (x, y);
      ╰╴
    ");
}

#[test]
fn select_for_update_of_unknown_relation() {
    assert_snapshot!(check("
create table t (a int);
select a from t as x for update of t;
"), @"
    error: missing `from` clause entry for table `t`
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
    error: missing `from` clause entry for table `t`
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
    error: `values` lists must all be the same length
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
    error: each `union` query must have the same number of columns
      ╭▸ 
    2 │ select 1 union select 1, 2;
      ╰╴               ━━━━━━━━━━━
    error: each `intersect` query must have the same number of columns
      ╭▸ 
    3 │ select 1 intersect select 1, 2;
      ╰╴                   ━━━━━━━━━━━
    error: each `except` query must have the same number of columns
      ╭▸ 
    4 │ select 1, 2 except select 1;
      ╰╴                   ━━━━━━━━
    error: each `union` query must have the same number of columns
      ╭▸ 
    5 │ select 1 union select 1 union all select 1, 2;
      ╰╴                                  ━━━━━━━━━━━
    error: each `union` query must have the same number of columns
      ╭▸ 
    6 │ select 1 union (select 1, 2);
      ╰╴               ━━━━━━━━━━━━━
    error: each `union` query must have the same number of columns
      ╭▸ 
    7 │ select 1 union values (1, 2);
      ╰╴               ━━━━━━━━━━━━━
    error: relation `missing` does not exist
      ╭▸ 
    9 │ select * from missing union select 1, 2;
      ╰╴              ━━━━━━━
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
    error: relation `missing` does not exist
       ╭▸ 
    14 │ select * from missing x (p, q, r);
       ╰╴              ━━━━━━━
    ");
}

#[test]
fn unresolved_qualified_column() {
    assert_snapshot!(check("
create table t (a int);
select t.nope from t;
select x.nope from t as x;
select public.t.nope from t;
select s.nope from (select 1 as a) as s;
with c as (select 1 as a) select c.nope from c;
comment on column t.nope is 'x';
update t as u set nope = 1;
insert into t (nope) values (1);
alter table t drop column nope;
alter table t drop column if exists nope;
"), @"
    error: column `t.nope` does not exist
      ╭▸ 
    3 │ select t.nope from t;
      ╰╴         ━━━━
    error: column `x.nope` does not exist
      ╭▸ 
    4 │ select x.nope from t as x;
      ╰╴         ━━━━
    error: column `t.nope` does not exist
      ╭▸ 
    5 │ select public.t.nope from t;
      ╰╴                ━━━━
    error: column `s.nope` does not exist
      ╭▸ 
    6 │ select s.nope from (select 1 as a) as s;
      ╰╴         ━━━━
    error: column `c.nope` does not exist
      ╭▸ 
    7 │ with c as (select 1 as a) select c.nope from c;
      ╰╴                                   ━━━━
    error: column `t.nope` does not exist
      ╭▸ 
    8 │ comment on column t.nope is 'x';
      ╰╴                    ━━━━
    error: column `t.nope` does not exist
      ╭▸ 
    9 │ update t as u set nope = 1;
      ╰╴                  ━━━━
    error: column `t.nope` does not exist
       ╭▸ 
    10 │ insert into t (nope) values (1);
       ╰╴               ━━━━
    error: column `t.nope` does not exist
       ╭▸ 
    11 │ alter table t drop column nope;
       ╰╴                          ━━━━
    ");
}

#[test]
fn unresolved_column_suggestion() {
    assert_snapshot!(check("
create table t (apple int, banana int, created_at int);
create table u (cherry int);
select app from t;
select appel from t;
select bananna from t;
select at_created from t;
select cheery from t, u;
select zzz from t;
select app from t as x;
select appl from t, t as t2;
"), @"
    error: column `app` does not exist
      ╭▸ 
    4 │ select app from t;
      │        ━━━
      ╰╴
    help: a column with a similar name exists
      ╭╴
    4 │ select apple from t;
      ╰╴          ++
    error: column `appel` does not exist
      ╭▸ 
    5 │ select appel from t;
      │        ━━━━━
      ╰╴
    help: a column with a similar name exists
      ╭╴
    5 - select appel from t;
    5 + select apple from t;
      ╰╴
    error: column `bananna` does not exist
      ╭▸ 
    6 │ select bananna from t;
      │        ━━━━━━━
      ╰╴
    help: a column with a similar name exists
      ╭╴
    6 - select bananna from t;
    6 + select banana from t;
      ╰╴
    error: column `at_created` does not exist
      ╭▸ 
    7 │ select at_created from t;
      │        ━━━━━━━━━━
      ╰╴
    help: a column with a similar name exists
      ╭╴
    7 - select at_created from t;
    7 + select created_at from t;
      ╰╴
    error: column `cheery` does not exist
      ╭▸ 
    8 │ select cheery from t, u;
      │        ━━━━━━
      ╰╴
    help: a column with a similar name exists
      ╭╴
    8 - select cheery from t, u;
    8 + select cherry from t, u;
      ╰╴
    error: column `zzz` does not exist
      ╭▸ 
    9 │ select zzz from t;
      ╰╴       ━━━
    error: column `app` does not exist
       ╭▸ 
    10 │ select app from t as x;
       │        ━━━
       ╰╴
    help: a column with a similar name exists
       ╭╴
    10 │ select apple from t as x;
       ╰╴          ++
    error: column `appl` does not exist
       ╭▸ 
    11 │ select appl from t, t as t2;
       │        ━━━━
       ╰╴
    help: a column with a similar name exists
       ╭╴
    11 - select appl from t, t as t2;
    11 + select t.apple from t, t as t2;
       ╰╴
    ");
}
