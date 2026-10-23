use insta::assert_snapshot;

use super::goto;

#[test]
fn cte_column() {
    assert_snapshot!(goto("
with t as (
  select 1 a
)
select a$0 from t;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:12
      │
    3 │   select 1 a
      │            ─ 2. destination
    4 │ )
    5 │ select a from t;
      ╰╴       ─ 1. source
    ");
}

#[test]
fn cte_table_ref() {
    assert_snapshot!(goto("
with t as (
  select 1 a
)
select a from t$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:6
      │
    2 │ with t as (
      │      ─ 2. destination
      ‡
    5 │ select a from t;
      ╰╴              ─ 1. source
    ");
}

#[test]
fn qualifier_to_cte() {
    assert_snapshot!(goto("
with t as (
  select 1 a
)
select t$0.a from t;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:6
      │
    2 │ with t as (
      │      ─ 2. destination
      ‡
    5 │ select t.a from t;
      ╰╴       ─ 1. source
    ");
}

#[test]
fn cte_column_list() {
    assert_snapshot!(goto("
with t(x) as (
  select 1 a
)
select x$0 from t;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:8
      │
    2 │ with t(x) as (
      │        ─ 2. destination
      ‡
    5 │ select x from t;
      ╰╴       ─ 1. source
    ");
}

#[test]
fn recursive_cte_column_in_recursive_term() {
    assert_snapshot!(goto("
with recursive t as (
  select 1 as n union all select n$0 + 1 from t where n < 10
)
select n from t;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:15
      │
    3 │   select 1 as n union all select n + 1 from t where n < 10
      ╰╴              ─ 2. destination   ─ 1. source
    ");
}

#[test]
fn recursive_cte_forward_ref_to_sibling() {
    assert_snapshot!(goto("
with t as (
  select 1 c
)
select *
from (
  with recursive x as (
    select c from t$0
  ),
  t as (
    select 2 c
  )
  select c from x
) s;
"), @"
    info: goto definition
       ╭▸ file0.sql:8:19
       │
     8 │     select c from t
       │                   ─ 1. source
     9 │   ),
    10 │   t as (
       ╰╴  ─ 2. destination
    ");
}

#[test]
fn recursive_cte_forward_ref_column_through_star() {
    assert_snapshot!(goto("
with recursive x as (
  select * from t
),
t as (
  select 2 c
)
select c$0 from x;
"), @"
    info: goto definition
      ╭▸ file0.sql:6:12
      │
    6 │   select 2 c
      │            ─ 2. destination
    7 │ )
    8 │ select c from x;
      ╰╴       ─ 1. source
    ");
}

#[test]
fn non_recursive_cte_forward_ref_uses_outer() {
    assert_snapshot!(goto("
with t as (
  select 1 c
)
select *
from (
  with x as (
    select c from t$0
  ),
  t as (
    select 2 c
  )
  select c from x
) s;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:6
      │
    2 │ with t as (
      │      ─ 2. destination
      ‡
    8 │     select c from t
      ╰╴                  ─ 1. source
    ");
}

#[test]
fn recursive_cte_mutual_recursion() {
    assert_snapshot!(goto("
with recursive x as (
  select * from y
),
y as (
  select 1 c from x
)
select * from x where c$0 = 1;
"), @"
    info: goto definition
      ╭▸ file0.sql:6:12
      │
    6 │   select 1 c from x
      │            ─ 2. destination
    7 │ )
    8 │ select * from x where c = 1;
      ╰╴                      ─ 1. source
    ");
}

#[test]
fn cte_parenthesized_body_column() {
    assert_snapshot!(goto("
with t as (
  ((select 1 a))
)
select a$0 from t;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:14
      │
    3 │   ((select 1 a))
      │              ─ 2. destination
    4 │ )
    5 │ select a from t;
      ╰╴       ─ 1. source
    ");
}

#[test]
fn cte_partial_column_list_hides_renamed() {
    assert_snapshot!(goto("
with k(x) as (
  select 1 id, 2 b
)
select id$0 from k;
"), @"no definition");
}

#[test]
fn cte_qualified_star_join_column_with_partial_column_list() {
    assert_snapshot!(goto("
with u as (
  select 1 id, 2 b
),
t as (
  select 1 id, 2 a
),
k(x) as (
  select u.* from t join u on a = b
)
select b$0 from k;
"), @"
    info: goto definition
       ╭▸ file0.sql:3:18
       │
     3 │   select 1 id, 2 b
       │                  ─ 2. destination
       ‡
    11 │ select b from k;
       ╰╴       ─ 1. source
    ");
}

#[test]
fn recursive_cte_partial_column_list() {
    assert_snapshot!(goto("
with recursive k(n) as (
  select 1, 2 b union all select n + 1, b from k
)
select b$0 from k;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:15
      │
    3 │   select 1, 2 b union all select n + 1, b from k
      │               ─ 2. destination
    4 │ )
    5 │ select b from k;
      ╰╴       ─ 1. source
    ");
}

#[test]
fn cte_search_by_column() {
    assert_snapshot!(goto("
with recursive t(n) as (
  select 1 union all select n + 1 from t
)
search depth first by n$0 set ord
select * from t;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:18
      │
    2 │ with recursive t(n) as (
      │                  ─ 2. destination
      ‡
    5 │ search depth first by n set ord
      ╰╴                      ─ 1. source
    ");
}

#[test]
fn cte_search_set_column() {
    assert_snapshot!(goto("
with recursive t(n) as (
  select 1 union all select n + 1 from t
)
search breadth first by n set ord
select ord$0 from t;
"), @"
    info: goto definition
      ╭▸ file0.sql:5:31
      │
    5 │ search breadth first by n set ord
      │                               ─── 2. destination
    6 │ select ord from t;
      ╰╴       ─── 1. source
    ");
}

#[test]
fn cte_cycle_columns() {
    assert_snapshot!(goto("
with recursive t(n) as (
  select 1 union all select n + 1 from t
)
cycle n$0 set is_cycle using path
select * from t;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:18
      │
    2 │ with recursive t(n) as (
      │                  ─ 2. destination
      ‡
    5 │ cycle n set is_cycle using path
      ╰╴      ─ 1. source
    ");
}

#[test]
fn cte_cycle_mark_column() {
    assert_snapshot!(goto("
with recursive t(n) as (
  select 1 union all select n + 1 from t
)
cycle n set is_cycle to 'Y' default 'N' using path
select is_cycle$0 from t;
"), @"
    info: goto definition
      ╭▸ file0.sql:5:13
      │
    5 │ cycle n set is_cycle to 'Y' default 'N' using path
      │             ──────── 2. destination
    6 │ select is_cycle from t;
      ╰╴       ──────── 1. source
    ");
}

#[test]
fn cte_cycle_path_column() {
    assert_snapshot!(goto("
with recursive t(n) as (
  select 1 union all select n + 1 from t
)
search depth first by n set ord
cycle n set is_cycle using path
select t.path$0 from t;
"), @"
    info: goto definition
      ╭▸ file0.sql:6:28
      │
    6 │ cycle n set is_cycle using path
      │                            ──── 2. destination
    7 │ select t.path from t;
      ╰╴         ──── 1. source
    ");
}

#[test]
fn cte_body_hides_main_from_column() {
    assert_snapshot!(goto("
create table u (
  x int,
  b int
);
create table w (b int);
with c as (
  select x$0
)
select * from u, c;
"), @"no definition");
}

#[test]
fn cte_body_hides_main_from_qualifier() {
    assert_snapshot!(goto("
create table u (
  x int,
  b int
);
create table w (b int);
with c as (
  select u$0.x
)
select * from u, c;
"), @"no definition");
}

#[test]
fn cte_body_hides_update_target_qualifier() {
    assert_snapshot!(goto("
create table u (
  x int,
  b int
);
create table w (b int);
create table t (a int);
with c as (
  select t$0.a
)
update t
set a = 1
from c;
"), @"no definition");
}

#[test]
fn inner_cte_forward_ref_falls_back_to_outer_cte() {
    assert_snapshot!(goto("
with t as (
  select 1 c
)
select *
from (
  with x as (
    select c$0 from t
  ),
  t as (
    select 2 c
  )
  select c from x
) s;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:12
      │
    3 │   select 1 c
      │            ─ 2. destination
      ‡
    8 │     select c from t
      ╰╴           ─ 1. source
    ");
}

#[test]
fn outer_cte_visible_inside_inner_with() {
    assert_snapshot!(goto("
with outer_cte as (
  select 1 c
)
select *
from (
  with inner_cte as (
    select 2 d
  )
  select c$0 from outer_cte
) s;
"), @"
    info: goto definition
       ╭▸ file0.sql:3:12
       │
     3 │   select 1 c
       │            ─ 2. destination
       ‡
    10 │   select c from outer_cte
       ╰╴         ─ 1. source
    ");
}

#[test]
fn qualified_table_ref_prefers_schema_qualified_from_item_over_cte() {
    assert_snapshot!(goto("
create schema s;
create table s.t (a int);
with t as (
  select 1 a
)
select t$0.a from s.t;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:16
      │
    3 │ create table s.t (a int);
      │                ─ 2. destination
      ‡
    7 │ select t.a from s.t;
      ╰╴       ─ 1. source
    ");
}

#[test]
fn cte_in_subquery_sees_outer_query() {
    assert_snapshot!(goto("
create table u (
  x int,
  b int
);
create table w (b int);
select (
  with c as (
    select u.x$0
  )
  select * from c
)
from u;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:3
      │
    3 │   x int,
      │   ─ 2. destination
      ‡
    9 │     select u.x
      ╰╴             ─ 1. source
    ");
}

#[test]
fn cte_values_column1() {
    assert_snapshot!(goto("
with t as (
  values (1, 2), (3, 4)
)
select column1$0, column2 from t;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:11
      │
    3 │   values (1, 2), (3, 4)
      │           ─ 2. destination
    4 │ )
    5 │ select column1, column2 from t;
      ╰╴       ─────── 1. source
    ");
}
