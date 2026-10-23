use insta::assert_snapshot;

use super::goto;

#[test]
fn inherited_column() {
    assert_snapshot!(goto("
create table p (a int);
create table c (b int) inherits (p);
select a$0 from c;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create table p (a int);
      │                 ─ 2. destination
    3 │ create table c (b int) inherits (p);
    4 │ select a from c;
      ╰╴       ─ 1. source
    ");
}

#[test]
fn inherits_table_name() {
    assert_snapshot!(goto("
create table p (a int);
create table c (b int) inherits (p$0);
"), @"
    info: goto definition
      ╭▸ file0.sql:2:14
      │
    2 │ create table p (a int);
      │              ─ 2. destination
    3 │ create table c (b int) inherits (p);
      ╰╴                                 ─ 1. source
    ");
}

#[test]
fn inherited_column_merged_with_local() {
    assert_snapshot!(goto("
create table p (a int);
create table c (
  a int,
  b int
)
  inherits (p);
select a$0 from c;
"), @"
    info: goto definition
      ╭▸ file0.sql:4:3
      │
    4 │   a int,
      │   ─ 2. destination
      ‡
    8 │ select a from c;
      ╰╴       ─ 1. source
    ");
}

#[test]
fn inherited_column_from_first_parent() {
    assert_snapshot!(goto("
create table p1 (
  a int,
  b int
);
create table p2 (
  b int,
  c int
);
create table c (d int) inherits (p1, p2);
select b$0 from (select * from c);
"), @"
    info: goto definition
       ╭▸ file0.sql:4:3
       │
     4 │   b int
       │   ─ 2. destination
       ‡
    11 │ select b from (select * from c);
       ╰╴       ─ 1. source
    ");
}

#[test]
fn inherited_column_from_second_parent() {
    assert_snapshot!(goto("
create table p1 (
  a int,
  b int
);
create table p2 (
  b int,
  c int
);
create table c (d int) inherits (p1, p2);
select c$0 from (select * from c);
"), @"
    info: goto definition
       ╭▸ file0.sql:8:3
       │
     8 │   c int
       │   ─ 2. destination
       ‡
    11 │ select c from (select * from c);
       ╰╴       ─ 1. source
    ");
}

#[test]
fn like_column() {
    assert_snapshot!(goto("
create table s (a int);
create table t (
  b int,
  like s
);
select a$0 from t;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create table s (a int);
      │                 ─ 2. destination
      ‡
    7 │ select a from t;
      ╰╴       ─ 1. source
    ");
}

#[test]
fn like_table_name() {
    assert_snapshot!(goto("
create table s (a int);
create table t (like s$0);
"), @"
    info: goto definition
      ╭▸ file0.sql:2:14
      │
    2 │ create table s (a int);
      │              ─ 2. destination
    3 │ create table t (like s);
      ╰╴                     ─ 1. source
    ");
}

#[test]
fn like_column_merged_with_inherited() {
    assert_snapshot!(goto("
create table p (a int);
create table s (a int);
create table t (like s) inherits (p);
select a$0 from t;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:17
      │
    3 │ create table s (a int);
      │                 ─ 2. destination
    4 │ create table t (like s) inherits (p);
    5 │ select a from t;
      ╰╴       ─ 1. source
    ");
}

#[test]
fn alter_table_inherit_ref() {
    assert_snapshot!(goto("
create table parent (a int);
create table child (a int);
alter table child inherit parent$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:14
      │
    2 │ create table parent (a int);
      │              ────── 2. destination
    3 │ create table child (a int);
    4 │ alter table child inherit parent;
      ╰╴                          ────── 1. source
    ");
}

#[test]
fn column_added_to_parent() {
    assert_snapshot!(goto("
create table t (a int);
create table u (b int) inherits (t);
alter table t add column c int;
select c$0 from u;
"), @"
    info: goto definition
      ╭▸ file0.sql:4:26
      │
    4 │ alter table t add column c int;
      │                          ─ 2. destination
    5 │ select c from u;
      ╰╴       ─ 1. source
    ");
}

#[test]
fn column_added_to_grandparent() {
    assert_snapshot!(goto("
create table t (a int);
create table u () inherits (t);
create table v () inherits (u);
alter table t add column c int;
select c$0 from v;
"), @"
    info: goto definition
      ╭▸ file0.sql:5:26
      │
    5 │ alter table t add column c int;
      │                          ─ 2. destination
    6 │ select c from v;
      ╰╴       ─ 1. source
    ");
}

#[test]
fn column_added_to_parent_merges_with_child_column() {
    assert_snapshot!(goto("
create table t (a int);
create table u (c int) inherits (t);
alter table t add column c int;
select c$0 from u;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:17
      │
    3 │ create table u (c int) inherits (t);
      │                 ─ 2. destination
    4 │ alter table t add column c int;
    5 │ select c from u;
      ╰╴       ─ 1. source
    ");
}

#[test]
fn column_added_after_alter_inherit() {
    assert_snapshot!(goto("
create table t (a int);
create table u (a int);
alter table u inherit t;
alter table t add column c int;
select c$0 from u;
"), @"
    info: goto definition
      ╭▸ file0.sql:5:26
      │
    5 │ alter table t add column c int;
      │                          ─ 2. destination
    6 │ select c from u;
      ╰╴       ─ 1. source
    ");
}

#[test]
fn column_added_after_no_inherit() {
    assert_snapshot!(goto("
create table t (a int);
create table u () inherits (t);
alter table u no inherit t;
alter table t add column c int;
select c$0 from u;
"), @"no definition");
}

#[test]
fn column_added_to_partitioned_table() {
    assert_snapshot!(goto("
create table t (a int) partition by range (a);
create table u
  partition of t
  for values from (1) to (10);
alter table t add column c int;
select c$0 from u;
"), @"
    info: goto definition
      ╭▸ file0.sql:6:26
      │
    6 │ alter table t add column c int;
      │                          ─ 2. destination
    7 │ select c from u;
      ╰╴       ─ 1. source
    ");
}

#[test]
fn column_added_after_rolled_back_inherit() {
    assert_snapshot!(goto("
create table t (a int);
create table u (a int);
begin;
alter table u inherit t;
rollback;
alter table t add column c int;
select c$0 from u;
"), @"no definition");
}

#[test]
fn column_added_to_merged_partition() {
    assert_snapshot!(goto("
create table t (a int) partition by range (a);
create table u1
  partition of t
  for values from (1) to (2);
create table u2
  partition of t
  for values from (2) to (3);
alter table t merge partitions (u1, u2) into u3;
alter table t add column c int;
select c$0 from u3;
"), @"
    info: goto definition
       ╭▸ file0.sql:10:26
       │
    10 │ alter table t add column c int;
       │                          ─ 2. destination
    11 │ select c from u3;
       ╰╴       ─ 1. source
    ");
}

#[test]
fn column_added_to_split_partition() {
    assert_snapshot!(goto("
create table t (a int) partition by range (a);
create table u
  partition of t
  for values from (1) to (3);
alter table t
  split partition u into (
    partition u1 for values from (1) to (2),
    partition u2 for values from (2) to (3)
  );
alter table t add column c int;
select c$0 from u1;
"), @"
    info: goto definition
       ╭▸ file0.sql:11:26
       │
    11 │ alter table t add column c int;
       │                          ─ 2. destination
    12 │ select c from u1;
       ╰╴       ─ 1. source
    ");
}

#[test]
fn column_renamed_in_parent() {
    assert_snapshot!(goto("
create table t (a int);
create table u () inherits (t);
alter table t rename column a to b;
select b$0 from u;
"), @"
    info: goto definition
      ╭▸ file0.sql:4:34
      │
    4 │ alter table t rename column a to b;
      │                                  ─ 2. destination
    5 │ select b from u;
      ╰╴       ─ 1. source
    ");
}

#[test]
fn column_renamed_in_grandparent() {
    assert_snapshot!(goto("
create table t (a int);
create table u () inherits (t);
create table v () inherits (u);
alter table t rename column a to b;
select b$0 from v;
"), @"
    info: goto definition
      ╭▸ file0.sql:5:34
      │
    5 │ alter table t rename column a to b;
      │                                  ─ 2. destination
    6 │ select b from v;
      ╰╴       ─ 1. source
    ");
}

#[test]
fn column_dropped_from_parent() {
    assert_snapshot!(goto("
create table t (a int);
create table u () inherits (t);
create table v () inherits (u);
alter table t drop column a;
select a$0 from v;
"), @"no definition");
}

#[test]
fn column_dropped_from_only_parent() {
    assert_snapshot!(goto("
create table t (a int);
create table u () inherits (t);
alter table only t drop column a;
select a$0 from u;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create table t (a int);
      │                 ─ 2. destination
      ‡
    5 │ select a from u;
      ╰╴       ─ 1. source
    ");
}

#[test]
fn column_dropped_from_parent_keeps_local_column() {
    assert_snapshot!(goto("
create table t (a int);
create table u (a int) inherits (t);
alter table t drop column a;
select a$0 from u;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:17
      │
    3 │ create table u (a int) inherits (t);
      │                 ─ 2. destination
    4 │ alter table t drop column a;
    5 │ select a from u;
      ╰╴       ─ 1. source
    ");
}

#[test]
fn column_dropped_from_one_of_two_parents() {
    assert_snapshot!(goto("
create table p1 (a int);
create table p2 (a int);
create table c () inherits (p1, p2);
alter table p1 drop column a;
select a$0 from c;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:18
      │
    3 │ create table p2 (a int);
      │                  ─ 2. destination
      ‡
    6 │ select a from c;
      ╰╴       ─ 1. source
    ");
}

#[test]
fn column_dropped_from_one_of_two_parents_grandchild() {
    assert_snapshot!(goto("
create table p1 (a int);
create table p2 (a int);
create table c () inherits (p1, p2);
create table d () inherits (c);
alter table p1 drop column a;
select a$0 from d;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:18
      │
    3 │ create table p2 (a int);
      │                  ─ 2. destination
      ‡
    7 │ select a from d;
      ╰╴       ─ 1. source
    ");
}
