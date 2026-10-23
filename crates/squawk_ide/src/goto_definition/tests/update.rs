use insta::assert_snapshot;

use super::goto;

#[test]
fn update_table() {
    assert_snapshot!(goto("
create table t (a int);
update t$0 set a = 1;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:14
      │
    2 │ create table t (a int);
      │              ─ 2. destination
    3 │ update t set a = 1;
      ╰╴       ─ 1. source
    ");
}

#[test]
fn update_set_column() {
    assert_snapshot!(goto("
create table t (
  a int,
  b int
);
update t set b$0 = 1;
"), @"
    info: goto definition
      ╭▸ file0.sql:4:3
      │
    4 │   b int
      │   ─ 2. destination
    5 │ );
    6 │ update t set b = 1;
      ╰╴             ─ 1. source
    ");
}

#[test]
fn update_set_expr_column() {
    assert_snapshot!(goto("
create table t (
  a int,
  b int
);
update t set a = b$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:4:3
      │
    4 │   b int
      │   ─ 2. destination
    5 │ );
    6 │ update t set a = b;
      ╰╴                 ─ 1. source
    ");
}

#[test]
fn update_from_column() {
    assert_snapshot!(goto("
create table t (a int);
create table u (c int);
update t set a = c$0 from u;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:17
      │
    3 │ create table u (c int);
      │                 ─ 2. destination
    4 │ update t set a = c from u;
      ╰╴                 ─ 1. source
    ");
}

#[test]
fn update_alias_qualifier() {
    assert_snapshot!(goto("
create table t (a int);
update t as x set a = 1 where x$0.a = 1;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:13
      │
    3 │ update t as x set a = 1 where x.a = 1;
      ╰╴            ─ 2. destination  ─ 1. source
    ");
}

#[test]
fn update_returning_column() {
    assert_snapshot!(goto("
create table t (a int);
update t set a = 1 returning a$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create table t (a int);
      │                 ─ 2. destination
    3 │ update t set a = 1 returning a;
      ╰╴                             ─ 1. source
    ");
}

#[test]
fn update_cte_returning_column() {
    assert_snapshot!(goto("
create table t (a int);
with i as (
  update t set a = 1 returning a
)
select a$0 from i;
"), @"
    info: goto definition
      ╭▸ file0.sql:4:32
      │
    4 │   update t set a = 1 returning a
      │                                ─ 2. destination
    5 │ )
    6 │ select a from i;
      ╰╴       ─ 1. source
    ");
}

#[test]
fn update_for_portion_of_column() {
    assert_snapshot!(goto("
create table t (
  id int,
  valid_at daterange
);
update t for portion of valid_at$0 from '2020-01-01' to '2021-01-01' set id = 1;
"), @"
    info: goto definition
      ╭▸ file0.sql:4:3
      │
    4 │   valid_at daterange
      │   ──────── 2. destination
    5 │ );
    6 │ update t for portion of valid_at from '2020-01-01' to '2021-01-01' set id = 1;
      ╰╴                        ──────── 1. source
    ");
}

#[test]
fn update_set_composite_field() {
    assert_snapshot!(goto("
create type pair as (
  x int,
  y int
);
create table t (p pair);
update t set p.y$0 = 1;
"), @"
    info: goto definition
      ╭▸ file0.sql:4:3
      │
    4 │   y int
      │   ─ 2. destination
      ‡
    7 │ update t set p.y = 1;
      ╰╴               ─ 1. source
    ");
}

#[test]
fn update_set_nested_composite_field() {
    assert_snapshot!(goto("
create type inner_t as (
  a int
);
create type outer_t as (
  i inner_t
);
create table t (o outer_t);
update t set o.i.a$0 = 1;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:3
      │
    3 │   a int
      │   ─ 2. destination
      ‡
    9 │ update t set o.i.a = 1;
      ╰╴                 ─ 1. source
    ");
}

#[test]
fn update_set_table_type_field() {
    assert_snapshot!(goto("
create table u (
  a int,
  b int
);
create table t (r u);
update t set r.b$0 = 1;
"), @"
    info: goto definition
      ╭▸ file0.sql:4:3
      │
    4 │   b int
      │   ─ 2. destination
      ‡
    7 │ update t set r.b = 1;
      ╰╴               ─ 1. source
    ");
}

#[test]
fn update_set_array_element_field() {
    assert_snapshot!(goto("
create type pair as (
  x int,
  y int
);
create table t (a pair[]);
update t set a[1].x$0 = 1;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:3
      │
    3 │   x int,
      │   ─ 2. destination
      ‡
    7 │ update t set a[1].x = 1;
      ╰╴                  ─ 1. source
    ");
}

#[test]
fn update_set_multidimensional_array_element_field() {
    assert_snapshot!(goto("
create type pair as (
  x int,
  y int
);
create table t (a pair[][]);
update t set a[1][2].y$0 = 1;
"), @"
    info: goto definition
      ╭▸ file0.sql:4:3
      │
    4 │   y int
      │   ─ 2. destination
      ‡
    7 │ update t set a[1][2].y = 1;
      ╰╴                     ─ 1. source
    ");
}

#[test]
fn update_set_composite_array_field_element_field() {
    assert_snapshot!(goto("
create type pair as (
  x int,
  y int
);
create type holder as (
  ps pair[]
);
create table t (h holder);
update t set h.ps[1].x$0 = 1;
"), @"
    info: goto definition
       ╭▸ file0.sql:3:3
       │
     3 │   x int,
       │   ─ 2. destination
       ‡
    10 │ update t set h.ps[1].x = 1;
       ╰╴                     ─ 1. source
    ");
}

#[test]
fn update_from_ambiguous_column_with_target() {
    assert_snapshot!(goto("
create table t (a int);
create table u (a int);
update t set a = 1 from u where a$0 = 1;
"), @"no definition");
}

#[test]
fn update_from_ambiguous_column_between_sources() {
    assert_snapshot!(goto("
create table t (a int);
create table u (x int);
create table w (x int);
update t set a = x$0 from u, w;
"), @"no definition");
}

#[test]
fn update_from_lateral_column_not_ambiguous_with_target() {
    assert_snapshot!(goto("
create table t (a int);
create table u (a int);
update t set a = 1 from u, lateral (select a$0) v;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:17
      │
    3 │ create table u (a int);
      │                 ─ 2. destination
    4 │ update t set a = 1 from u, lateral (select a) v;
      ╰╴                                           ─ 1. source
    ");
}

#[test]
fn update_from_function_arg_hides_target_column() {
    assert_snapshot!(goto("
create table t (
  a int,
  z int
);
create table u (b int);
create table w (y int);
update t set a = 1 from generate_series(1, a$0) g;
"), @"no definition");
}

#[test]
fn update_from_on_clause_hides_target_column() {
    assert_snapshot!(goto("
create table t (
  a int,
  z int
);
create table u (b int);
create table w (y int);
update t set a = 1 from u join w on a$0 = 1;
"), @"no definition");
}

#[test]
fn update_from_lateral_subquery_hides_target_column() {
    assert_snapshot!(goto("
create table t (
  a int,
  z int
);
create table u (b int);
create table w (y int);
update t set a = 1 from u, lateral (select z$0) v;
"), @"no definition");
}

#[test]
fn update_where_subquery_target_column() {
    assert_snapshot!(goto("
create table t (
  a int,
  b int
);
create table u (b int);
update t set a = 1 where exists(select 1 from u where u.b = t.b$0);
"), @"
    info: goto definition
      ╭▸ file0.sql:4:3
      │
    4 │   b int
      │   ─ 2. destination
      ‡
    7 │ update t set a = 1 where exists(select 1 from u where u.b = t.b);
      ╰╴                                                              ─ 1. source
    ");
}

#[test]
fn update_where_subquery_target_hidden_by_alias() {
    assert_snapshot!(goto("
create table t (
  a int,
  b int
);
create table u (b int);
update t as x set a = 1 where exists(select 1 from u where u.b = t.b$0);
"), @"no definition");
}

#[test]
fn update_alias_hides_table_name_qualified_column() {
    assert_snapshot!(goto("
create table t (a int);
update t as u set a = t.a$0;
"), @"no definition");
}

#[test]
fn update_returning_old_alias_hides_old_qualifier() {
    assert_snapshot!(goto("
create table t (a int);
update t set a = 1 returning with (old as before) old$0.a;
"), @"no definition");
}

#[test]
fn update_returning_old_alias_keeps_new_visible() {
    assert_snapshot!(goto("
create table t (a int);
update t set a = 1 returning with (old as before) new.a$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create table t (a int);
      │                 ─ 2. destination
    3 │ update t set a = 1 returning with (old as before) new.a;
      ╰╴                                                      ─ 1. source
    ");
}

#[test]
fn update_returning_new_column() {
    assert_snapshot!(goto("
create table t (
  a int,
  b int
);
update t set a = 42 returning old.a, new.b$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:4:3
      │
    4 │   b int
      │   ─ 2. destination
    5 │ );
    6 │ update t set a = 42 returning old.a, new.b;
      ╰╴                                         ─ 1. source
    ");
}
