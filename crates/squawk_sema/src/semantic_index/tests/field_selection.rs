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
fn row_field_selection() {
    assert_snapshot!(check("
create function f() returns record
  language sql
  as 'select 1, 2';
create function g(out a int, out b int)
  language sql
  as 'select 1, 2';
select
  (row(1, 2)).f2,
  ((1, 2)).f1,
  (row(1)).x,
  ((1, 2)).*,
  (f()).x,
  (g()).a,
  (g()).zz;
"), @"
    error: could not identify column `x` in type `record`
       ╭▸ 
    11 │   (row(1)).x,
       ╰╴           ━
    error: could not identify column `x` in type `record`
       ╭▸ 
    13 │   (f()).x,
       ╰╴        ━
    error: could not identify column `zz` in type `record`
       ╭▸ 
    15 │   (g()).zz;
       ╰╴        ━━
    ");
}

#[test]
fn boolean_expr_field_selection() {
    assert_snapshot!(check("
select (exists(select 1)).x, (1 between 0 and 2).x, (1 not between 0 and 2).x;
select
  (1 is null).x,
  (1 isnull).x,
  (1 notnull).x,
  (true is true).x,
  (true is not false).x;
select (not true).x, (true and false).x, (true or false).x, ('1' is json).x;
"), @"
    error: column notation `.x` applied to type `bool`, which is not a composite type
      ╭▸ 
    2 │ select (exists(select 1)).x, (1 between 0 and 2).x, (1 not between 0 and 2).x;
      ╰╴                          ━
    error: column notation `.x` applied to type `bool`, which is not a composite type
      ╭▸ 
    2 │ select (exists(select 1)).x, (1 between 0 and 2).x, (1 not between 0 and 2).x;
      ╰╴                                                 ━
    error: column notation `.x` applied to type `bool`, which is not a composite type
      ╭▸ 
    2 │ select (exists(select 1)).x, (1 between 0 and 2).x, (1 not between 0 and 2).x;
      ╰╴                                                                            ━
    error: column notation `.x` applied to type `bool`, which is not a composite type
      ╭▸ 
    4 │   (1 is null).x,
      ╰╴              ━
    error: column notation `.x` applied to type `bool`, which is not a composite type
      ╭▸ 
    5 │   (1 isnull).x,
      ╰╴             ━
    error: column notation `.x` applied to type `bool`, which is not a composite type
      ╭▸ 
    6 │   (1 notnull).x,
      ╰╴              ━
    error: column notation `.x` applied to type `bool`, which is not a composite type
      ╭▸ 
    7 │   (true is true).x,
      ╰╴                 ━
    error: column notation `.x` applied to type `bool`, which is not a composite type
      ╭▸ 
    8 │   (true is not false).x;
      ╰╴                      ━
    error: column notation `.x` applied to type `bool`, which is not a composite type
      ╭▸ 
    9 │ select (not true).x, (true and false).x, (true or false).x, ('1' is json).x;
      ╰╴                  ━
    error: column notation `.x` applied to type `bool`, which is not a composite type
      ╭▸ 
    9 │ select (not true).x, (true and false).x, (true or false).x, ('1' is json).x;
      ╰╴                                      ━
    error: column notation `.x` applied to type `bool`, which is not a composite type
      ╭▸ 
    9 │ select (not true).x, (true and false).x, (true or false).x, ('1' is json).x;
      ╰╴                                                         ━
    error: column notation `.x` applied to type `bool`, which is not a composite type
      ╭▸ 
    9 │ select (not true).x, (true and false).x, (true or false).x, ('1' is json).x;
      ╰╴                                                                          ━
    ");
}

#[test]
fn date_time_keyword_field_selection() {
    assert_snapshot!(check("
select
  (current_date).x,
  (current_time).x,
  (current_time(1)).x,
  (current_timestamp(3)).x;
select
  (localtime).x,
  (localtime(1)).x,
  (localtimestamp).x,
  (localtimestamp(1)).x;
"), @"
    error: column notation `.x` applied to type `date`, which is not a composite type
      ╭▸ 
    3 │   (current_date).x,
      ╰╴                 ━
    error: column notation `.x` applied to type `timetz`, which is not a composite type
      ╭▸ 
    4 │   (current_time).x,
      ╰╴                 ━
    error: column notation `.x` applied to type `timetz`, which is not a composite type
      ╭▸ 
    5 │   (current_time(1)).x,
      ╰╴                    ━
    error: column notation `.x` applied to type `timestamptz`, which is not a composite type
      ╭▸ 
    6 │   (current_timestamp(3)).x;
      ╰╴                         ━
    error: column notation `.x` applied to type `time`, which is not a composite type
      ╭▸ 
    8 │   (localtime).x,
      ╰╴              ━
    error: column notation `.x` applied to type `time`, which is not a composite type
      ╭▸ 
    9 │   (localtime(1)).x,
      ╰╴                 ━
    error: column notation `.x` applied to type `timestamp`, which is not a composite type
       ╭▸ 
    10 │   (localtimestamp).x,
       ╰╴                   ━
    error: column notation `.x` applied to type `timestamp`, which is not a composite type
       ╭▸ 
    11 │   (localtimestamp(1)).x;
       ╰╴                      ━
    ");
}

#[test]
fn json_field_selection() {
    assert_snapshot!(check("
select
  (json_object()).x,
  (json_object(returning jsonb)).x,
  (json_array()).x,
  (json_array(returning jsonb)).x,
  (json('1')).x,
  (json_scalar(1)).x,
  (json_serialize('1'::json)).x,
  (json_serialize('1'::json returning bytea)).x;
select
  (json_query('1'::jsonb, '$')).x,
  (json_query('1'::jsonb, '$' returning text)).x,
  (json_value('1'::jsonb, '$')).x,
  (json_value('1'::jsonb, '$' returning int)).x,
  (json_exists('1'::jsonb, '$')).x;
select (json_objectagg(k: v)).x, (json_arrayagg(v returning jsonb)).x
from (values ('a', 1)) as t (k, v);
"), @"
    error: column notation `.x` applied to type `json`, which is not a composite type
      ╭▸ 
    3 │   (json_object()).x,
      ╰╴                  ━
    error: column notation `.x` applied to type `jsonb`, which is not a composite type
      ╭▸ 
    4 │   (json_object(returning jsonb)).x,
      ╰╴                                 ━
    error: column notation `.x` applied to type `json`, which is not a composite type
      ╭▸ 
    5 │   (json_array()).x,
      ╰╴                 ━
    error: column notation `.x` applied to type `jsonb`, which is not a composite type
      ╭▸ 
    6 │   (json_array(returning jsonb)).x,
      ╰╴                                ━
    error: column notation `.x` applied to type `json`, which is not a composite type
      ╭▸ 
    7 │   (json('1')).x,
      ╰╴              ━
    error: column notation `.x` applied to type `json`, which is not a composite type
      ╭▸ 
    8 │   (json_scalar(1)).x,
      ╰╴                   ━
    error: column notation `.x` applied to type `text`, which is not a composite type
      ╭▸ 
    9 │   (json_serialize('1'::json)).x,
      ╰╴                              ━
    error: column notation `.x` applied to type `bytea`, which is not a composite type
       ╭▸ 
    10 │   (json_serialize('1'::json returning bytea)).x;
       ╰╴                                              ━
    error: column notation `.x` applied to type `jsonb`, which is not a composite type
       ╭▸ 
    12 │   (json_query('1'::jsonb, '$')).x,
       ╰╴                                ━
    error: column notation `.x` applied to type `text`, which is not a composite type
       ╭▸ 
    13 │   (json_query('1'::jsonb, '$' returning text)).x,
       ╰╴                                               ━
    error: column notation `.x` applied to type `text`, which is not a composite type
       ╭▸ 
    14 │   (json_value('1'::jsonb, '$')).x,
       ╰╴                                ━
    error: column notation `.x` applied to type `int4`, which is not a composite type
       ╭▸ 
    15 │   (json_value('1'::jsonb, '$' returning int)).x,
       ╰╴                                              ━
    error: column notation `.x` applied to type `bool`, which is not a composite type
       ╭▸ 
    16 │   (json_exists('1'::jsonb, '$')).x;
       ╰╴                                 ━
    error: column notation `.x` applied to type `json`, which is not a composite type
       ╭▸ 
    17 │ select (json_objectagg(k: v)).x, (json_arrayagg(v returning jsonb)).x
       ╰╴                              ━
    error: column notation `.x` applied to type `jsonb`, which is not a composite type
       ╭▸ 
    17 │ select (json_objectagg(k: v)).x, (json_arrayagg(v returning jsonb)).x
       ╰╴                                                                    ━
    ");
}

#[test]
fn xml_field_selection() {
    assert_snapshot!(check("
select
  (xmlconcat('<a/>'::xml, '<b/>')).x,
  (xmlelement(name a)).x,
  (xmlforest(1 as a)).x,
  (xmlparse(document '<a/>')).x,
  (xmlpi(name a)).x,
  (xmlroot('<a/>'::xml, version no value)).x,
  (xmlserialize(document '<a/>'::xml as text)).x,
  (xmlserialize(document '<a/>'::xml as varchar(10))).x;
"), @"
    error: column notation `.x` applied to type `xml`, which is not a composite type
      ╭▸ 
    3 │   (xmlconcat('<a/>'::xml, '<b/>')).x,
      ╰╴                                   ━
    error: column notation `.x` applied to type `xml`, which is not a composite type
      ╭▸ 
    4 │   (xmlelement(name a)).x,
      ╰╴                       ━
    error: column notation `.x` applied to type `xml`, which is not a composite type
      ╭▸ 
    5 │   (xmlforest(1 as a)).x,
      ╰╴                      ━
    error: column notation `.x` applied to type `xml`, which is not a composite type
      ╭▸ 
    6 │   (xmlparse(document '<a/>')).x,
      ╰╴                              ━
    error: column notation `.x` applied to type `xml`, which is not a composite type
      ╭▸ 
    7 │   (xmlpi(name a)).x,
      ╰╴                  ━
    error: column notation `.x` applied to type `xml`, which is not a composite type
      ╭▸ 
    8 │   (xmlroot('<a/>'::xml, version no value)).x,
      ╰╴                                           ━
    error: column notation `.x` applied to type `text`, which is not a composite type
      ╭▸ 
    9 │   (xmlserialize(document '<a/>'::xml as text)).x,
      ╰╴                                               ━
    error: column notation `.x` applied to type `varchar`, which is not a composite type
       ╭▸ 
    10 │   (xmlserialize(document '<a/>'::xml as varchar(10))).x;
       ╰╴                                                      ━
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

#[test]
fn functional_notation() {
    assert_snapshot!(check("
create table t (a int);
select x.count, x.a from t x;
select ('{}'::jsonb).jsonb_typeof;
select x.nope from t x;
select ('{}'::jsonb).nope;
"), @"
    error: column `x.nope` does not exist
      ╭▸ 
    5 │ select x.nope from t x;
      ╰╴         ━━━━
    error: column notation `.nope` applied to type `jsonb`, which is not a composite type
      ╭▸ 
    6 │ select ('{}'::jsonb).nope;
      ╰╴                     ━━━━
    ");
}

#[test]
fn record_field_through_cte() {
    assert_snapshot!(check("
with cte as (select pg_get_keywords() k) select (k).word from cte;
"), @"no diagnostics");
}

#[test]
fn anonymous_record_functions() {
    assert_snapshot!(check("
create type ct as (
  a int
);
create function g(ct) returns int
  language sql
  as 'select 1';
create function h(text) returns int
  language sql
  as 'select 1';
create function h(anyelement) returns int
  language sql
  as 'select 1';
create function rr() returns record
  language sql
  as 'select 1, 2';
select (row(1)).pg_typeof, (row(1, 'a')).to_json, (row(1)).count, (row(1)).g;
select (row(1)).h + 1, (rr()).pg_typeof, (row(1)).f1;
select (row(1)).nope;
select (row(1)).ascii;
select (row(1)).array_ndims;
"), @"
    error: could not identify column `nope` in type `record`
       ╭▸ 
    19 │ select (row(1)).nope;
       ╰╴                ━━━━
    error: could not identify column `ascii` in type `record`
       ╭▸ 
    20 │ select (row(1)).ascii;
       ╰╴                ━━━━━
    error: could not identify column `array_ndims` in type `record`
       ╭▸ 
    21 │ select (row(1)).array_ndims;
       ╰╴                ━━━━━━━━━━━
    ");
}

#[test]
fn missing_record_fields() {
    assert_snapshot!(check("
create function o(out a int, out b int)
  language sql
  as 'select 1, 2';
select (s.x).f1 + 1, (s.x).f2 || 'b' from (select row(1, 'a') as x) s;
with c as (
  select row(1) as x
)
select (x).f1 from c;
select (s.x).a from (select o() as x) s;
select (o.y).f1 from (select s.x as y from (select row(1) as x) s) o;
select (row(1)::record).f1;
select (row(1, s.*)).f3 from (select 1 as a, 2 as b) s;
select (null::record).nope;
select (case when true then row(1) end).f1;
select ((select row(1))).f1;
select (array[row(1)])[1].f1;
select (coalesce(row(1), row(2))).f1;
select (s.x).nope from (select row(1) as x) s;
select (s.x).nope from (select o() as x) s;
select (o.y).nope from (select s.x as y from (select row(1) as x) s) o;
select (row(s.*)).f3 from (select 1 as a, 2 as b) s;
"), @"
    error: could not identify column `nope` in type `record`
       ╭▸ 
    14 │ select (null::record).nope;
       ╰╴                      ━━━━
    error: could not identify column `f1` in type `record`
       ╭▸ 
    15 │ select (case when true then row(1) end).f1;
       ╰╴                                        ━━
    error: could not identify column `f1` in type `record`
       ╭▸ 
    16 │ select ((select row(1))).f1;
       ╰╴                         ━━
    error: could not identify column `f1` in type `record`
       ╭▸ 
    17 │ select (array[row(1)])[1].f1;
       ╰╴                          ━━
    error: could not identify column `f1` in type `record`
       ╭▸ 
    18 │ select (coalesce(row(1), row(2))).f1;
       ╰╴                                  ━━
    error: could not identify column `nope` in type `record`
       ╭▸ 
    19 │ select (s.x).nope from (select row(1) as x) s;
       ╰╴             ━━━━
    error: could not identify column `nope` in type `record`
       ╭▸ 
    20 │ select (s.x).nope from (select o() as x) s;
       ╰╴             ━━━━
    error: could not identify column `nope` in type `record`
       ╭▸ 
    21 │ select (o.y).nope from (select s.x as y from (select row(1) as x) s) o;
       ╰╴             ━━━━
    error: could not identify column `f3` in type `record`
       ╭▸ 
    22 │ select (row(s.*)).f3 from (select 1 as a, 2 as b) s;
       ╰╴                  ━━
    ");
}

#[test]
fn opaque_record_fields() {
    assert_snapshot!(check("
create function rr() returns record
  language sql
  as 'select 1, 2';
select (x).f1 from (select row(1)) s (x);
with c(x) as (
  select row(1)
)
select (y).f1 from c as d (y);
select (o.y).a from (select s as y from (select 1 as a) s) o;
select (x).f1
from (select row(1) as x) s right join (select row(1) as x) t using (x);
select (s.x).* from (select row(1) as x) s;
select (s.x).f1 from (values (row(1))) s (x);
select (x).nope from (select row(1)) s (x);
with c(x) as (
  select row(1)
)
select (y).nope from c as d (y);
select (o.y).nope from (select s as y from (select 1 as a) s) o;
select (o.y).nope from (select j as y from json_to_record('{}') as j (a int)) o;
select (s.x).column1 from (select rr() as x) s;
select (x).f1
from (select row(1) as x) s full join (select row(1) as x) t using (x);
select (lag(x) over ()).f1 from (select row(1) as x) s;
select (s.y).f1 from (select lag(x) over () as y from (select row(1) as x) t) s;
select (null::record).*;
select (rr()).*;
select row((s.x).*) from (values (row(1))) s (x);
"), @"
    error: could not identify column `f1` in type `record`
       ╭▸ 
    14 │ select (s.x).f1 from (values (row(1))) s (x);
       ╰╴             ━━
    error: could not identify column `nope` in type `record`
       ╭▸ 
    15 │ select (x).nope from (select row(1)) s (x);
       ╰╴           ━━━━
    error: could not identify column `nope` in type `record`
       ╭▸ 
    19 │ select (y).nope from c as d (y);
       ╰╴           ━━━━
    error: could not identify column `nope` in type `record`
       ╭▸ 
    20 │ select (o.y).nope from (select s as y from (select 1 as a) s) o;
       ╰╴             ━━━━
    error: could not identify column `nope` in type `record`
       ╭▸ 
    21 │ select (o.y).nope from (select j as y from json_to_record('{}') as j (a int)) o;
       ╰╴             ━━━━
    error: could not identify column `column1` in type `record`
       ╭▸ 
    22 │ select (s.x).column1 from (select rr() as x) s;
       ╰╴             ━━━━━━━
    error: could not identify column `f1` in type `record`
       ╭▸ 
    23 │ select (x).f1
       ╰╴           ━━
    error: could not identify column `f1` in type `record`
       ╭▸ 
    25 │ select (lag(x) over ()).f1 from (select row(1) as x) s;
       ╰╴                        ━━
    error: could not identify column `f1` in type `record`
       ╭▸ 
    26 │ select (s.y).f1 from (select lag(x) over () as y from (select row(1) as x) t) s;
       ╰╴             ━━
    error: cannot expand `.*` on a `record` with unknown columns
       ╭▸ 
    27 │ select (null::record).*;
       ╰╴                      ━
    error: cannot expand `.*` on a `record` with unknown columns
       ╭▸ 
    28 │ select (rr()).*;
       ╰╴              ━
    error: cannot expand `.*` on a `record` with unknown columns
       ╭▸ 
    29 │ select row((s.x).*) from (values (row(1))) s (x);
       ╰╴                 ━
    ");
}

#[test]
fn function_source_rows() {
    assert_snapshot!(check("
select (o.y) + 1, (o.y).pg_typeof
from (select j as y from generate_series(1, 2) as j (b)) o;
select (o.y).a, (o.y).b
from (select u as y from unnest(array[1], array[2]) as u (a, b)) o;
select * from unnest(array[(1, 2), (3, 4)]) as t (a int, b int);
select * from pg_class c, unnest(array[c]) as u where u.relname = 'a';
select (o.y).b from (select j as y from generate_series(1, 2) as j (b)) o;
select (o.y).nope
from (select j as y from generate_series(1, 2) with ordinality as j (b, c)) o;
select (o.y).nope
from (
  select j as y
  from rows from (generate_series(1, 2), generate_series(1, 3)) as j (b, c)
) o;
"), @"
    error: column notation `.b` applied to type `int4`, which is not a composite type
      ╭▸ 
    8 │ select (o.y).b from (select j as y from generate_series(1, 2) as j (b)) o;
      ╰╴             ━
    error: could not identify column `nope` in type `record`
      ╭▸ 
    9 │ select (o.y).nope
      ╰╴             ━━━━
    error: could not identify column `nope` in type `record`
       ╭▸ 
    11 │ select (o.y).nope
       ╰╴             ━━━━
    ");
}
