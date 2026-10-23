use insta::assert_snapshot;

use super::goto;

#[test]
fn partition_of_table_name() {
    assert_snapshot!(goto("
create table p (a int) partition by range (a);
create table c
  partition of p$0
  for values from (1) to (10);
"), @"
    info: goto definition
      ╭▸ file0.sql:2:14
      │
    2 │ create table p (a int) partition by range (a);
      │              ─ 2. destination
    3 │ create table c
    4 │   partition of p
      ╰╴               ─ 1. source
    ");
}

#[test]
fn partition_of_column() {
    assert_snapshot!(goto("
create table p (a int) partition by range (a);
create table c
  partition of p
  for values from (1) to (10);
select a$0 from c;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create table p (a int) partition by range (a);
      │                 ─ 2. destination
      ‡
    6 │ select a from c;
      ╰╴       ─ 1. source
    ");
}

#[test]
fn partition_of_column_override() {
    assert_snapshot!(goto("
create table p (a int) partition by range (a);
create table c
  partition of p (a)
  for values from (1) to (10);
select a$0 from c;
"), @"
    info: goto definition
      ╭▸ file0.sql:4:19
      │
    4 │   partition of p (a)
      │                   ─ 2. destination
    5 │   for values from (1) to (10);
    6 │ select a from c;
      ╰╴       ─ 1. source
    ");
}

#[test]
fn partition_by_column() {
    assert_snapshot!(goto("
create table t (a int) partition by range (a$0);
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create table t (a int) partition by range (a);
      ╰╴                ─ 2. destination           ─ 1. source
    ");
}

#[test]
fn alter_table_attach_partition_ref() {
    assert_snapshot!(goto("
create table p (a int) partition by list (a);
create table p1 (a int);
alter table p attach partition p1$0 for values in (1);
"), @"
    info: goto definition
      ╭▸ file0.sql:3:14
      │
    3 │ create table p1 (a int);
      │              ── 2. destination
    4 │ alter table p attach partition p1 for values in (1);
      ╰╴                               ── 1. source
    ");
}

#[test]
fn merged_partition_ref() {
    assert_snapshot!(goto("
create table p (a int) partition by list (a);
create table p1
  partition of p
  for values in (1);
create table p2
  partition of p
  for values in (2);
alter table p merge partitions (p1, p2) into p12;
select a from p12$0;
"), @"
    info: goto definition
       ╭▸ file0.sql:9:46
       │
     9 │ alter table p merge partitions (p1, p2) into p12;
       │                                              ─── 2. destination
    10 │ select a from p12;
       ╰╴              ─── 1. source
    ");
}

#[test]
fn split_partition_ref() {
    assert_snapshot!(goto("
create table p (a int) partition by list (a);
create table p12
  partition of p
  for values in (1, 2);
alter table p
  split partition p12 into (
    partition p1 for values in (1),
    partition p2 for values in (2)
  );
select a$0 from p2;
"), @"
    info: goto definition
       ╭▸ file0.sql:2:17
       │
     2 │ create table p (a int) partition by list (a);
       │                 ─ 2. destination
       ‡
    11 │ select a from p2;
       ╰╴       ─ 1. source
    ");
}

#[test]
fn alter_table_detach_partition() {
    assert_snapshot!(goto("
create table parent (id int) partition by range (id);
create table child
  partition of parent
  for values from (1) to (10);
alter table parent detach partition ch$0ild;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:14
      │
    3 │ create table child
      │              ───── 2. destination
      ‡
    6 │ alter table parent detach partition child;
      ╰╴                                    ───── 1. source
    ");
}
