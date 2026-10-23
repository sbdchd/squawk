use insta::assert_snapshot;

use super::hover;

#[test]
fn inherited_column() {
    assert_snapshot!(hover("
create table p (a int);
create table c (b int) inherits (p);
select a$0 from c;
"), @"
    hover: column public.c.a int4
      ╭▸ file0.sql:4:8
      │
    4 │ select a from c;
      ╰╴       ─ hover
    ");
}

#[test]
fn inherited_column_on_parent() {
    assert_snapshot!(hover("
create table p (a int);
create table c (b int) inherits (p);
select a$0 from p;
"), @"
    hover: column public.p.a int4
      ╭▸ file0.sql:4:8
      │
    4 │ select a from p;
      ╰╴       ─ hover
    ");
}

#[test]
fn inherited_column_merged_with_local() {
    assert_snapshot!(hover("
create table p (a int);
create table c (
  a int,
  b int
)
  inherits (p);
select a$0 from c;
"), @"
    hover: column public.c.a int4
      ╭▸ file0.sql:8:8
      │
    8 │ select a from c;
      ╰╴       ─ hover
    ");
}

#[test]
fn inherited_column_from_second_parent() {
    assert_snapshot!(hover("
create table p1 (a int);
create table p2 (b text);
create table c () inherits (p1, p2);
select b$0 from c;
"), @"
    hover: column public.c.b text
      ╭▸ file0.sql:5:8
      │
    5 │ select b from c;
      ╰╴       ─ hover
    ");
}

#[test]
fn alter_table_inherit_column() {
    assert_snapshot!(hover("
create table p (a int);
create table c (a int);
alter table c inherit p;
select a$0 from c;
"), @"
    hover: column public.c.a int4
      ╭▸ file0.sql:5:8
      │
    5 │ select a from c;
      ╰╴       ─ hover
    ");
}

#[test]
fn partition_of_column() {
    assert_snapshot!(hover("
create table p (
  a int,
  b text
)
  partition by list (a);
create table c
  partition of p
  for values in (1);
select b$0 from c;
"), @"
    hover: column public.c.b text
       ╭▸ file0.sql:10:8
       │
    10 │ select b from c;
       ╰╴       ─ hover
    ");
}

#[test]
fn partition_of_column_override() {
    assert_snapshot!(hover("
create table p (
  a int,
  b text
)
  partition by list (a);
create table c
  partition of p (b not null)
  for values in (1);
select b$0 from c;
"), @"
    hover: column public.c.b text
       ╭▸ file0.sql:10:8
       │
    10 │ select b from c;
       ╰╴       ─ hover
    ");
}

#[test]
fn typed_table_column() {
    assert_snapshot!(hover("
create type pair as (
  a int,
  b text
);
create table t
  of pair;
select b$0 from t;
"), @"
    hover: column public.t.b text
      ╭▸ file0.sql:8:8
      │
    8 │ select b from t;
      ╰╴       ─ hover
    ");
}

#[test]
fn column_type_changed_in_parent() {
    assert_snapshot!(hover("
create table t (a int);
create table u () inherits (t);
create table v () inherits (u);
alter table t alter column a type text;
select a$0 from v;
"), @"
    hover: column public.v.a text
      ╭▸ file0.sql:6:8
      │
    6 │ select a from v;
      ╰╴       ─ hover
    ");
}
