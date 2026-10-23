use insta::assert_snapshot;

use super::goto;

#[test]
fn lateral_deeply_nested_paren_expr_values_alias_in_subquery() {
    assert_snapshot!(goto("
select u.n, x.val
from (values (1), (2)) u (n)
  cross join lateral ((((select u$0.n * 10 as val)))) x;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:24
      │
    3 │ from (values (1), (2)) u (n)
      │                        ─ 2. destination
    4 │   cross join lateral ((((select u.n * 10 as val)))) x;
      ╰╴                                ─ 1. source
    ");
}

#[test]
fn nested_join_lateral_in_paren_sees_outer_earlier() {
    assert_snapshot!(goto("
create table u (x int);
create table w (y int);
create table z (x int);
select * from u join (w cross join lateral (select x$0) v) on true;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create table u (x int);
      │                 ─ 2. destination
      ‡
    5 │ select * from u join (w cross join lateral (select x) v) on true;
      ╰╴                                                   ─ 1. source
    ");
}

#[test]
fn aliased_join_expr_qualified_right_column() {
    assert_snapshot!(goto("
create table t (
  a int,
  b int
);
create table u (
  a int,
  c int
);
select j.c$0 from (t join u using (a)) as j;
"), @"
    info: goto definition
       ╭▸ file0.sql:8:3
       │
     8 │   c int
       │   ─ 2. destination
     9 │ );
    10 │ select j.c from (t join u using (a)) as j;
       ╰╴         ─ 1. source
    ");
}

#[test]
fn aliased_join_expr_merged_column() {
    assert_snapshot!(goto("
create table t (
  a int,
  b int
);
create table u (
  a int,
  c int
);
select j.a$0 from (t join u using (a)) as j;
"), @"
    info: goto definition
       ╭▸ file0.sql:3:3
       │
     3 │   a int,
       │   ─ 2. destination
       ‡
    10 │ select j.a from (t join u using (a)) as j;
       ╰╴         ─ 1. source
    ");
}

#[test]
fn join_using_alias_column() {
    assert_snapshot!(goto("
create table a (x int);
create table b (x int);
select j.x$0 from a join b using (x) as j;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create table a (x int);
      │                 ─ 2. destination
    3 │ create table b (x int);
    4 │ select j.x from a join b using (x) as j;
      ╰╴         ─ 1. source
    ");
}

#[test]
fn join_using_alias_table() {
    assert_snapshot!(goto("
create table a (x int);
create table b (x int);
select j$0.x from a join b using (x) as j;
"), @"
    info: goto definition
      ╭▸ file0.sql:4:8
      │
    4 │ select j.x from a join b using (x) as j;
      ╰╴       ─ 1. source                    ─ 2. destination
    ");
}

#[test]
fn join_using_merged_column() {
    assert_snapshot!(goto("
create table a (x int);
create table b (x int);
select x$0 from a join b using (x);
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create table a (x int);
      │                 ─ 2. destination
    3 │ create table b (x int);
    4 │ select x from a join b using (x);
      ╰╴       ─ 1. source
    ");
}

#[test]
fn join_using_column_ref() {
    assert_snapshot!(goto("
create table a (x int);
create table b (x int);
select * from a join b using (x$0);
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create table a (x int);
      │                 ─ 2. destination
    3 │ create table b (x int);
      │                 ─ 3. destination
    4 │ select * from a join b using (x);
      ╰╴                              ─ 1. source
    ");
}

#[test]
fn natural_join_merged_column() {
    assert_snapshot!(goto("
create table a (
  x int,
  y int
);
create table b (
  x int,
  z int
);
select x$0 from a natural join b;
"), @"
    info: goto definition
       ╭▸ file0.sql:3:3
       │
     3 │   x int,
       │   ─ 2. destination
       ‡
    10 │ select x from a natural join b;
       ╰╴       ─ 1. source
    ");
}

#[test]
fn nested_join_using_merged_column() {
    assert_snapshot!(goto("
create table a (x int);
create table b (x int);
create table c (x int);
select x$0 from a join b using (x) join c using (x);
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create table a (x int);
      │                 ─ 2. destination
      ‡
    5 │ select x from a join b using (x) join c using (x);
      ╰╴       ─ 1. source
    ");
}

#[test]
fn left_join_using_merged_column() {
    assert_snapshot!(goto("
create table a (x int);
create table b (x int);
select x$0 from a left join b using (x);
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create table a (x int);
      │                 ─ 2. destination
    3 │ create table b (x int);
    4 │ select x from a left join b using (x);
      ╰╴       ─ 1. source
    ");
}

#[test]
fn right_join_using_merged_column() {
    assert_snapshot!(goto("
create table a (x int);
create table b (x int);
select x$0 from a right join b using (x);
"), @"
    info: goto definition
      ╭▸ file0.sql:3:17
      │
    3 │ create table b (x int);
      │                 ─ 2. destination
    4 │ select x from a right join b using (x);
      ╰╴       ─ 1. source
    ");
}

#[test]
fn full_join_using_merged_column() {
    assert_snapshot!(goto("
create table a (x int);
create table b (x int);
select x$0 from a full join b using (x);
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create table a (x int);
      │                 ─ 2. destination
    3 │ create table b (x int);
      │                 ─ 3. destination
    4 │ select x from a full join b using (x);
      ╰╴       ─ 1. source
    ");
}

#[test]
fn inner_join_using_prefers_uncoerced_column() {
    assert_snapshot!(goto("
create table a (x int);
create table b (x bigint);
select x$0 from a join b using (x);
"), @"
    info: goto definition
      ╭▸ file0.sql:3:17
      │
    3 │ create table b (x bigint);
      │                 ─ 2. destination
    4 │ select x from a join b using (x);
      ╰╴       ─ 1. source
    ");
}

#[test]
fn inner_join_using_after_full_join_prefers_right() {
    assert_snapshot!(goto("
create table a (x int);
create table b (x int);
create table c (x int);
select x$0 from a full join b using (x) join c using (x);
"), @"
    info: goto definition
      ╭▸ file0.sql:4:17
      │
    4 │ create table c (x int);
      │                 ─ 2. destination
    5 │ select x from a full join b using (x) join c using (x);
      ╰╴       ─ 1. source
    ");
}

#[test]
fn inner_join_using_after_full_join_on_both_sides() {
    assert_snapshot!(goto("
create table a (x int);
create table b (x int);
create table c (x int);
create table d (x int);
select x$0
from (a full join b using (x)) join (c full join d using (x)) using (x);
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create table a (x int);
      │                 ─ 2. destination
    3 │ create table b (x int);
      │                 ─ 3. destination
      ‡
    6 │ select x
      ╰╴       ─ 1. source
    ");
}

#[test]
fn from_lateral_subquery_sees_earlier_sibling() {
    assert_snapshot!(goto("
create table u (
  x int,
  j jsonb
);
select * from u, lateral (select x$0) v;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:3
      │
    3 │   x int,
      │   ─ 2. destination
      ‡
    6 │ select * from u, lateral (select x) v;
      ╰╴                                 ─ 1. source
    ");
}

#[test]
fn from_lateral_subquery_hides_later_sibling() {
    assert_snapshot!(goto("
create table u (
  x int,
  j jsonb
);
select * from lateral (select x$0) v, u;
"), @"no definition");
}

#[test]
fn from_non_lateral_subquery_hides_sibling() {
    assert_snapshot!(goto("
create table u (
  x int,
  j jsonb
);
select * from u, (select x$0) v;
"), @"no definition");
}

#[test]
fn from_function_arg_sees_earlier_sibling() {
    assert_snapshot!(goto("
create table u (
  x int,
  j jsonb
);
select * from u, generate_series(1, x$0) g;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:3
      │
    3 │   x int,
      │   ─ 2. destination
      ‡
    6 │ select * from u, generate_series(1, x) g;
      ╰╴                                    ─ 1. source
    ");
}

#[test]
fn from_function_arg_hides_later_sibling() {
    assert_snapshot!(goto("
create table u (
  x int,
  j jsonb
);
select * from generate_series(1, x$0) g, u;
"), @"no definition");
}

#[test]
fn from_rows_from_arg_hides_later_sibling() {
    assert_snapshot!(goto("
create table u (
  x int,
  j jsonb
);
select * from rows from (generate_series(1, x$0)) g, u;
"), @"no definition");
}

#[test]
fn from_json_table_arg_hides_later_sibling() {
    assert_snapshot!(goto("
create table u (
  x int,
  j jsonb
);
select * from json_table(j$0, '$' columns (a int)) jt, u;
"), @"no definition");
}

#[test]
fn from_function_arg_sees_earlier_join_using_alias() {
    assert_snapshot!(goto("
create table u (b int);
create table w (b int);
select * from u join w using (b) as j, generate_series(1, j$0.b) g;
"), @"
    info: goto definition
      ╭▸ file0.sql:4:37
      │
    4 │ select * from u join w using (b) as j, generate_series(1, j.b) g;
      ╰╴                                    ─ 2. destination      ─ 1. source
    ");
}

#[test]
fn from_non_lateral_subquery_hides_join_using_alias() {
    assert_snapshot!(goto("
create table u (b int);
create table w (b int);
select * from u join w using (b) as j, (select j$0.b) q;
"), @"no definition");
}

#[test]
fn on_clause_subquery_sees_own_join() {
    assert_snapshot!(goto("
create table u (
  x int,
  b int
);
create table w (
  y int,
  b int
);
create table z (q int);
create table y (f int);
create table a (c int);
create table b (d int);
select * from u join w on exists(select x$0);
"), @"
    info: goto definition
       ╭▸ file0.sql:3:3
       │
     3 │   x int,
       │   ─ 2. destination
       ‡
    14 │ select * from u join w on exists(select x);
       ╰╴                                        ─ 1. source
    ");
}

#[test]
fn on_clause_falls_through_to_outer_level() {
    assert_snapshot!(goto("
create table u (
  x int,
  b int
);
create table w (
  y int,
  b int
);
create table z (q int);
create table y (f int);
create table a (c int);
create table b (d int);
select (select 1 from w join z on x$0 = 1) from u;
"), @"
    info: goto definition
       ╭▸ file0.sql:3:3
       │
     3 │   x int,
       │   ─ 2. destination
       ‡
    14 │ select (select 1 from w join z on x = 1) from u;
       ╰╴                                  ─ 1. source
    ");
}

#[test]
fn on_clause_hides_comma_sibling() {
    assert_snapshot!(goto("
create table u (
  x int,
  b int
);
create table w (
  y int,
  b int
);
create table z (q int);
create table y (f int);
create table a (c int);
create table b (d int);
select * from u, w join z on x$0 = 1;
"), @"no definition");
}

#[test]
fn on_clause_hides_later_comma_sibling() {
    assert_snapshot!(goto("
create table u (
  x int,
  b int
);
create table w (
  y int,
  b int
);
create table z (q int);
create table y (f int);
create table a (c int);
create table b (d int);
select * from w join z on x$0 = 1, u;
"), @"no definition");
}

#[test]
fn on_clause_hides_table_outside_paren_join() {
    assert_snapshot!(goto("
create table u (
  x int,
  b int
);
create table w (
  y int,
  b int
);
create table z (q int);
create table y (f int);
create table a (c int);
create table b (d int);
select * from y join (a join b on a.c = y$0.f) on true;
"), @"no definition");
}

#[test]
fn right_join_lateral_hides_left_column() {
    assert_snapshot!(goto("
create table a (x int);
select * from a right join lateral (select a.x$0) v on true;
"), @"no definition");
}

#[test]
fn left_join_lateral_sees_left_column() {
    assert_snapshot!(goto("
create table a (x int);
select * from a left join lateral (select a.x$0) v on true;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create table a (x int);
      │                 ─ 2. destination
    3 │ select * from a left join lateral (select a.x) v on true;
      ╰╴                                            ─ 1. source
    ");
}

#[test]
fn full_join_function_hides_left_column() {
    assert_snapshot!(goto("
create table a (x int);
select * from a full join generate_series(1, a.x$0) g on true;
"), @"no definition");
}

#[test]
fn nested_join_non_lateral_in_paren_hides_outer() {
    assert_snapshot!(goto("
create table u (x int);
create table w (y int);
create table z (x int);
select * from z join (w cross join (select x$0) v) on true;
"), @"no definition");
}

#[test]
fn nested_join_lateral_in_paren_hides_later() {
    assert_snapshot!(goto("
create table u (x int);
create table w (y int);
create table z (x int);
select * from (w cross join lateral (select x$0) v cross join u);
"), @"no definition");
}

#[test]
fn nested_join_aliased_paren_join_hides_inner_name() {
    assert_snapshot!(goto("
create table u (x int);
create table w (y int);
create table z (x int);
select * from (u cross join w) p, lateral (select u$0.x) q;
"), @"no definition");
}

#[test]
fn ambiguous_unqualified_column_comma_join() {
    assert_snapshot!(goto("
create table t (a int);
create table u (a int);
select a$0 from t, u;
"), @"no definition");
}

#[test]
fn unaliased_paren_join_qualified_column_outer_on_clause() {
    assert_snapshot!(goto("
create table t (a int);
create table u (b int);
create table v (c int);
select 1 from (t join u on t.a = u.b) join v on t.a$0 = v.c;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create table t (a int);
      │                 ─ 2. destination
      ‡
    5 │ select 1 from (t join u on t.a = u.b) join v on t.a = v.c;
      ╰╴                                                  ─ 1. source
    ");
}
