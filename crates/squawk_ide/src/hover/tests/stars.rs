use insta::assert_snapshot;

use super::hover;

#[test]
fn select_star() {
    assert_snapshot!(hover("
create table u (
  id int,
  b int
);
select *$0 from u;
"), @"
    hover: column public.u.id int4
          column public.u.b int4
      ╭▸ file0.sql:6:8
      │
    6 │ select * from u;
      ╰╴       ─ hover
    ");
}

#[test]
fn select_qualified_star() {
    assert_snapshot!(hover("
create table u (
  id int,
  b int
);
select u.*$0 from u;
"), @"
    hover: column public.u.id int4
          column public.u.b int4
      ╭▸ file0.sql:6:10
      │
    6 │ select u.* from u;
      ╰╴         ─ hover
    ");
}

#[test]
fn select_star_multiple_tables() {
    assert_snapshot!(hover("
create table t (a int);
create table u (b text);
select *$0 from t, u;
"), @"
    hover: column public.t.a int4
          column public.u.b text
      ╭▸ file0.sql:4:8
      │
    4 │ select * from t, u;
      ╰╴       ─ hover
    ");
}

#[test]
fn select_qualified_star_alias() {
    assert_snapshot!(hover("
create table t (a int);
create table u (b text);
select x.*$0 from t as x, u;
"), @"
    hover: column public.t.a int4
      ╭▸ file0.sql:4:10
      │
    4 │ select x.* from t as x, u;
      ╰╴         ─ hover
    ");
}

#[test]
fn select_star_from_cte() {
    assert_snapshot!(hover("
with u as (
  select 1 as id, 2 as b
)
select *$0 from u;
"), @"
    hover: column u.id int4
          column u.b int4
      ╭▸ file0.sql:5:8
      │
    5 │ select * from u;
      ╰╴       ─ hover
    ");
}

#[test]
fn select_star_from_empty_cte() {
    assert_snapshot!(hover("
with t as (
  select
)
select *$0 from t;
"), @"no hover");
}

#[test]
fn select_star_from_subquery() {
    assert_snapshot!(hover("
create table t (
  a int,
  b int
);
select *$0 from (select a from t);
"), @"
    hover: column a int4
      ╭▸ file0.sql:6:8
      │
    6 │ select * from (select a from t);
      ╰╴       ─ hover
    ");
}

#[test]
fn select_star_from_subquery_literal() {
    assert_snapshot!(hover("
select *$0 from (select 1) as sub;
"), @"
    hover: column ?column? int4
      ╭▸ file0.sql:2:8
      │
    2 │ select * from (select 1) as sub;
      ╰╴       ─ hover
    ");
}

#[test]
fn select_star_from_nested_stars() {
    assert_snapshot!(hover("
with u as (
  select 1 as id, 2 as b
)
select *$0 from (select *, * from u);
"), @"
    hover: column id int4
          column b int4
          column id int4
          column b int4
      ╭▸ file0.sql:5:8
      │
    5 │ select * from (select *, * from u);
      ╰╴       ─ hover
    ");
}

#[test]
fn select_star_from_view() {
    assert_snapshot!(hover("
create view v as
  select 1 as a;
select v.*$0 from v;
"), @"
    hover: column public.v.a int4
      ╭▸ file0.sql:4:10
      │
    4 │ select v.* from v;
      ╰╴         ─ hover
    ");
}

#[test]
fn select_star_from_function() {
    assert_snapshot!(hover("
create function f() returns table (a int, b text)
  as 'select 1, 2'
  language sql;
select *$0 from f();
"), @"
    hover: column public.f.a int4
          column public.f.b text
      ╭▸ file0.sql:5:8
      │
    5 │ select * from f();
      ╰╴       ─ hover
    ");
}

#[test]
fn select_star_join_using() {
    assert_snapshot!(hover("
create table t (
  id int,
  a int
);
create table u (
  id int,
  b int
);
select *$0 from t join u using (id);
"), @"
    hover: column public.t.id int4
          column public.t.a int4
          column public.u.b int4
       ╭▸ file0.sql:10:8
       │
    10 │ select * from t join u using (id);
       ╰╴       ─ hover
    ");
}

#[test]
fn composite_field_star() {
    assert_snapshot!(hover("
create type pair as (
  a int,
  b text
);
create table t (p pair);
select (p).*$0 from t;
"), @"
    hover: field public.pair.a int4
          field public.pair.b text
      ╭▸ file0.sql:7:12
      │
    7 │ select (p).* from t;
      ╰╴           ─ hover
    ");
}

#[test]
fn update_returning_star() {
    assert_snapshot!(hover("
create table t (
  a int,
  b int
);
update t set a = 1 returning *$0;
"), @"
    hover: column public.t.a int4
          column public.t.b int4
      ╭▸ file0.sql:6:30
      │
    6 │ update t set a = 1 returning *;
      ╰╴                             ─ hover
    ");
}

#[test]
fn insert_returning_star() {
    assert_snapshot!(hover("
create table t (
  a int,
  b int
);
insert into t values (1, 2) returning *$0;
"), @"
    hover: column public.t.a int4
          column public.t.b int4
      ╭▸ file0.sql:6:39
      │
    6 │ insert into t values (1, 2) returning *;
      ╰╴                                      ─ hover
    ");
}

#[test]
fn delete_returning_star() {
    assert_snapshot!(hover("
create table t (
  a int,
  b int
);
delete from t returning *$0;
"), @"
    hover: column public.t.a int4
          column public.t.b int4
      ╭▸ file0.sql:6:25
      │
    6 │ delete from t returning *;
      ╰╴                        ─ hover
    ");
}

#[test]
fn merge_returning_qualified_star_old() {
    assert_snapshot!(hover("
create table t (
  a int,
  b int
);
merge into t
using (select 1 as x, 2 as y) as u on t.a = u.x
when matched then
  update set b = u.y
returning old.*$0;
"), @"
    hover: column public.t.a int4
          column public.t.b int4
       ╭▸ file0.sql:10:15
       │
    10 │ returning old.*;
       ╰╴              ─ hover
    ");
}

#[test]
fn star_in_function_body() {
    assert_snapshot!(hover("
create table t (a int);
create function f() returns setof t
  as $$
  select *$0 from t;
$$
  language sql;
"), @"
    hover: column public.t.a int4
      ╭▸ file0.sql:5:10
      │
    5 │   select * from t;
      ╰╴         ─ hover
    ");
}

#[test]
fn count_star() {
    assert_snapshot!(hover("
create table u (
  id int,
  b int
);
select count(*$0) from u;
"), @"no hover");
}

#[test]
fn select_star_join_using_column_order() {
    assert_snapshot!(hover("
create table t (
  a int,
  id int
);
create table u (
  b int,
  id int
);
select *$0 from t join u using (id);
"), @"
    hover: column public.t.id int4
          column public.t.a int4
          column public.u.b int4
       ╭▸ file0.sql:10:8
       │
    10 │ select * from t join u using (id);
       ╰╴       ─ hover
    ");
}

#[test]
fn select_star_full_join_using() {
    assert_snapshot!(hover("
create table t (
  id int,
  a int
);
create table u (
  id int,
  b int
);
select *$0 from t full join u using (id);
"), @"
    hover: column public.t.id int4
          column public.u.id int4
          column public.t.a int4
          column public.u.b int4
       ╭▸ file0.sql:10:8
       │
    10 │ select * from t full join u using (id);
       ╰╴       ─ hover
    ");
}

#[test]
fn select_star_natural_join() {
    assert_snapshot!(hover("
create table t (
  a int,
  id int,
  x int
);
create table u (
  x int,
  id int,
  b int
);
select *$0 from t natural join u;
"), @"
    hover: column public.t.id int4
          column public.t.x int4
          column public.t.a int4
          column public.u.b int4
       ╭▸ file0.sql:12:8
       │
    12 │ select * from t natural join u;
       ╰╴       ─ hover
    ");
}

#[test]
fn select_star_nested_join_using() {
    assert_snapshot!(hover("
create table t (
  id int,
  a int
);
create table u (
  id int,
  b int
);
create table v (
  c int,
  id int
);
select *$0 from t join u using (id) join v using (id);
"), @"
    hover: column public.t.id int4
          column public.t.a int4
          column public.u.b int4
          column public.v.c int4
       ╭▸ file0.sql:14:8
       │
    14 │ select * from t join u using (id) join v using (id);
       ╰╴       ─ hover
    ");
}

#[test]
fn select_star_join_using_nested_right() {
    assert_snapshot!(hover("
create table t (
  id int,
  a int
);
create table u (
  x int,
  id int
);
create table v (
  x int,
  c int
);
select *$0 from t join (u join v using (x)) using (id);
"), @"
    hover: column public.t.id int4
          column public.t.a int4
          column public.u.x int4
          column public.v.c int4
       ╭▸ file0.sql:14:8
       │
    14 │ select * from t join (u join v using (x)) using (id);
       ╰╴       ─ hover
    ");
}

#[test]
fn select_star_join_using_with_on_join() {
    assert_snapshot!(hover("
create table t (
  id int,
  a int
);
create table u (b int);
create table v (
  id int,
  c int
);
select *$0 from t join u on true join v using (id);
"), @"
    hover: column public.t.id int4
          column public.t.a int4
          column public.u.b int4
          column public.v.c int4
       ╭▸ file0.sql:11:8
       │
    11 │ select * from t join u on true join v using (id);
       ╰╴       ─ hover
    ");
}

#[test]
fn select_star_join_using_alias() {
    assert_snapshot!(hover("
create table t (
  a int,
  id int
);
create table u (
  id int,
  b int
);
select j.*$0 from (t join u using (id)) as j;
"), @"
    hover: column public.t.id int4
          column a int4
          column b int4
       ╭▸ file0.sql:10:10
       │
    10 │ select j.* from (t join u using (id)) as j;
       ╰╴         ─ hover
    ");
}

#[test]
fn select_star_join_using_qualified_left() {
    assert_snapshot!(hover("
create table t (
  id int,
  a int
);
create table u (
  id int,
  b int
);
select t.*$0 from t join u using (id);
"), @"
    hover: column public.t.id int4
          column public.t.a int4
       ╭▸ file0.sql:10:10
       │
    10 │ select t.* from t join u using (id);
       ╰╴         ─ hover
    ");
}

#[test]
fn row_constructor_qualified_star() {
    assert_snapshot!(hover("
create table t (a int, b text);
select row(t.*$0) from t;
"), @"
    hover: column public.t.a int4
          column public.t.b text
      ╭▸ file0.sql:3:14
      │
    3 │ select row(t.*) from t;
      ╰╴             ─ hover
    ");
}

#[test]
fn cast_qualified_star() {
    assert_snapshot!(hover("
create table t (a int, b text);
select (t.*$0)::text from t;
"), @"
    hover: column public.t.a int4
          column public.t.b text
      ╭▸ file0.sql:3:11
      │
    3 │ select (t.*)::text from t;
      ╰╴          ─ hover
    ");
}

#[test]
fn composite_field_star_in_expr() {
    assert_snapshot!(hover("
create type pair as (a int, b text);
create table t (p pair);
select row((p).*$0) from t;
"), @"
    hover: field public.pair.a int4
          field public.pair.b text
      ╭▸ file0.sql:4:16
      │
    4 │ select row((p).*) from t;
      ╰╴               ─ hover
    ");
}

#[test]
fn qualified_star_whole_row_type() {
    assert_snapshot!(hover("
create table t (a int, b text);
select x$0 from (select (t.*) as x from t) s;
"), @"
    hover: column x t
      ╭▸ file0.sql:3:8
      │
    3 │ select x from (select (t.*) as x from t) s;
      ╰╴       ─ hover
    ");
}

#[test]
fn row_constructor_record_type() {
    assert_snapshot!(hover("
create table t (a int, b text);
select y$0 from (select row(t.*) as y from t) s;
"), @"
    hover: column y record
      ╭▸ file0.sql:3:8
      │
    3 │ select y from (select row(t.*) as y from t) s;
      ╰╴       ─ hover
    ");
}

#[test]
fn tuple_record_type() {
    assert_snapshot!(hover("
select y$0 from (select (1, 'a') as y) s;
"), @"
    hover: column y record
      ╭▸ file0.sql:2:8
      │
    2 │ select y from (select (1, 'a') as y) s;
      ╰╴       ─ hover
    ");
}

#[test]
fn function_table_star_in_expr() {
    assert_snapshot!(hover("
create function f() returns table (a int, b text)
  as 'select 1, 2'
  language sql;
select ((f()).*$0)::text;
"), @"
    hover: column public.f.a int4
          column public.f.b text
      ╭▸ file0.sql:5:15
      │
    5 │ select ((f()).*)::text;
      ╰╴              ─ hover
    ");
}

#[test]
fn function_out_columns_star_type() {
    assert_snapshot!(hover("
create function f(out a int, out b text)
  as 'select 1, 2'
  language sql;
select y$0 from (select ((f()).*) as y) s;
"), @"
    hover: column y record
      ╭▸ file0.sql:5:8
      │
    5 │ select y from (select ((f()).*) as y) s;
      ╰╴       ─ hover
    ");
}

#[test]
fn function_table_star_type() {
    assert_snapshot!(hover("
create type pair as (a int, b text);
create function f() returns setof pair
  as 'select 1, 2'
  language sql;
select y$0 from (select ((f()).*) as y) s;
"), @"
    hover: column y pair
      ╭▸ file0.sql:6:8
      │
    6 │ select y from (select ((f()).*) as y) s;
      ╰╴       ─ hover
    ");
}
