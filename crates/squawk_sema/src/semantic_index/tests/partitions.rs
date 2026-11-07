use insta::assert_snapshot;

use super::check;

#[test]
fn partition_of() {
    assert_snapshot!(check("
create foreign data wrapper w;
create server s foreign data wrapper w;
create table p (a int, b text) partition by range (a);
create table c partition of p for values from (minvalue) to (10);
select a, b, nope from c;
create table d partition of p (b) for values from (10) to (20);
select a, b, nope2 from d;
create table e partition of nope3 default;
select x from e;
create table l (a int) partition by list (a);
create table l1 partition of l for values in (1, 2, nope4);
create foreign table f partition of p for values from (20) to (30) server s;
select a, b, nope5 from f;
"), @"
    error: column `nope` does not exist
      ╭▸ 
    6 │ select a, b, nope from c;
      ╰╴             ━━━━
    error: column `nope2` does not exist
      ╭▸ 
    8 │ select a, b, nope2 from d;
      ╰╴             ━━━━━
    error: relation `nope3` does not exist
      ╭▸ 
    9 │ create table e partition of nope3 default;
      ╰╴                            ━━━━━
    error: column `nope4` does not exist
       ╭▸ 
    12 │ create table l1 partition of l for values in (1, 2, nope4);
       ╰╴                                                    ━━━━━
    error: column `nope5` does not exist
       ╭▸ 
    14 │ select a, b, nope5 from f;
       ╰╴             ━━━━━
    ");
}

#[test]
fn partition_by() {
    assert_snapshot!(check(r#"
create table r (a int, b text) partition by range (a, (b || 'x'), missing);
create table l (a int, b text) partition by list (b collate "C");
create table l2 (a int, b text) partition by list (b collate missing_collation);
create table h (a int) partition by hash (a int4_ops);
create table h2 (a int) partition by hash (a int4_minmax_ops);
create table r2 (a text) partition by range (a text_pattern_ops);
create table r3 (a int) partition by range (a missing_ops);
create table c partition of r for values from (1, 'a', 1) to (2, 'b', 2) partition by list (b);
"#), @"
    error: column `missing` does not exist
      ╭▸ 
    2 │ create table r (a int, b text) partition by range (a, (b || 'x'), missing);
      ╰╴                                                                  ━━━━━━━
    error: collation `missing_collation` does not exist
      ╭▸ 
    4 │ create table l2 (a int, b text) partition by list (b collate missing_collation);
      ╰╴                                                             ━━━━━━━━━━━━━━━━━
    error: operator class `int4_minmax_ops` does not exist
      ╭▸ 
    6 │ create table h2 (a int) partition by hash (a int4_minmax_ops);
      ╰╴                                             ━━━━━━━━━━━━━━━
    error: operator class `missing_ops` does not exist
      ╭▸ 
    8 │ create table r3 (a int) partition by range (a missing_ops);
      ╰╴                                              ━━━━━━━━━━━
    ");
}

#[test]
fn alter_table_attach_detach_partition() {
    assert_snapshot!(check("
create table p (a int) partition by range (a);
create table p1 (a int);
create table p2 (a int);
create foreign data wrapper w;
create server s foreign data wrapper w;
create foreign table pf (a int) server s;
alter table p attach partition p1 for values from (1) to (10);
alter table p attach partition p2 default;
alter table p attach partition pf for values from (10) to (minvalue);
alter table p attach partition nope for values from (1) to (nope2);
alter table p detach partition p1;
alter table p detach partition p2 concurrently;
alter table p detach partition nope3 finalize;
"), @"
    error: relation `nope` does not exist
       ╭▸ 
    11 │ alter table p attach partition nope for values from (1) to (nope2);
       ╰╴                               ━━━━
    error: column `nope2` does not exist
       ╭▸ 
    11 │ alter table p attach partition nope for values from (1) to (nope2);
       ╰╴                                                            ━━━━━
    error: relation `nope3` does not exist
       ╭▸ 
    14 │ alter table p detach partition nope3 finalize;
       ╰╴                               ━━━━━
    ");
}

#[test]
fn alter_table_merge_partitions() {
    assert_snapshot!(check("
create table p (a int, b text) partition by range (a);
create table p1 partition of p for values from (1) to (10);
create table p2 partition of p for values from (10) to (20);
create table other (x int);
alter table p merge partitions (p1, p2) into p12;
select a, b from p12;
select a from p1;
begin;
alter table p merge partitions (p12) into p_rolled_back;
rollback;
select a from p_rolled_back;
select a from p12;
alter table p merge partitions (p12) into p12;
select a from p12;
alter table p merge partitions (p12, nope) into other;
"), @"
    error: relation `p1` does not exist
      ╭▸ 
    8 │ select a from p1;
      │               ━━
      ╰╴
    help: a relation with a similar name exists
      ╭╴
    8 │ select a from p12;
      ╰╴                +
    error: relation `p_rolled_back` does not exist
       ╭▸ 
    12 │ select a from p_rolled_back;
       ╰╴              ━━━━━━━━━━━━━
    error: relation `nope` does not exist
       ╭▸ 
    16 │ alter table p merge partitions (p12, nope) into other;
       ╰╴                                     ━━━━
    error: relation `other` already exists
       ╭▸ 
    16 │ alter table p merge partitions (p12, nope) into other;
       ╰╴                                                ━━━━━
    ");
}

#[test]
fn alter_table_split_partition() {
    assert_snapshot!(check("
create table p (a int) partition by range (a);
create table p12 partition of p for values from (1) to (20);
alter table p split partition p12 into (
  partition p1 for values from (1) to (10),
  partition p2 for values from (10) to (nope)
);
select a from p1;
select a from p2;
select a from p12;
alter table p split partition nope into (partition p3 for values from (20) to (30));
"), @"
    error: column `nope` does not exist
      ╭▸ 
    6 │   partition p2 for values from (10) to (nope)
      ╰╴                                        ━━━━
    error: relation `p12` does not exist
       ╭▸ 
    10 │ select a from p12;
       │               ━━━
       ╰╴
    help: a relation with a similar name exists
       ╭╴
    10 - select a from p12;
    10 + select a from p2;
       ╰╴
    error: relation `nope` does not exist
       ╭▸ 
    11 │ alter table p split partition nope into (partition p3 for values from (20) to (30));
       ╰╴                              ━━━━
    ");
}

#[test]
fn attach_partition_circular() {
    assert_snapshot!(check("
create table p (a int) partition by list (a);
create table c partition of p for values in (1) partition by list (a);
alter table c attach partition p for values in (2);
alter table p attach partition p for values in (3);
alter table c add primary key (a);
"), @"
    error: circular inheritance
      ╭▸ 
    4 │ alter table c attach partition p for values in (2);
      │                                ━
      │
      ╰ note: `c` is already a child of `p`
    error: circular inheritance
      ╭▸ 
    5 │ alter table p attach partition p for values in (3);
      │                                ━
      │
      ╰ note: `p` is already a child of `p`
    ");
}

#[test]
fn truncate_only_partitioned_table() {
    assert_snapshot!(check("
create table p (a int) partition by range (a);
create table c
  partition of p
  for values from (0) to (10);
create table l (a int);
truncate only p;
truncate table only (p), c;
truncate only c, l;
truncate p;
truncate p, only p;
"), @"
    error: cannot truncate only partitioned table `p`
      ╭▸ 
    7 │ truncate only p;
      │               ━
      ╰╴
    help: remove `only` to also truncate the partitions, or truncate the partitions directly
      ╭╴
    7 - truncate only p;
    7 + truncate p;
      ╰╴
    error: cannot truncate only partitioned table `p`
      ╭▸ 
    8 │ truncate table only (p), c;
      │                      ━
      ╰╴
    help: remove `only` to also truncate the partitions, or truncate the partitions directly
      ╭╴
    8 - truncate table only (p), c;
    8 + truncate table p, c;
      ╰╴
    ");
}

#[test]
fn unique_index_on_partitioned_table_missing_partition_key() {
    assert_snapshot!(check("
create table pt (
  k int,
  v int
)
  partition by range (k);
create unique index on pt (v);
create unique index on only pt (v);
"), @"
    error: unique index on partitioned table `pt` must include partition key column `k`
      ╭▸ 
    7 │ create unique index on pt (v);
      ╰╴                          ━━━
    error: unique index on partitioned table `pt` must include partition key column `k`
      ╭▸ 
    8 │ create unique index on only pt (v);
      ╰╴                               ━━━
    ");
}

#[test]
fn unique_index_on_partitioned_table_include_does_not_cover_partition_key() {
    assert_snapshot!(check("
create table pt (
  k int,
  v int
)
  partition by range (k);
create unique index on pt (v) include (k);
"), @"
    error: unique index on partitioned table `pt` must include partition key column `k`
      ╭▸ 
    7 │ create unique index on pt (v) include (k);
      ╰╴                          ━━━
    ");
}

#[test]
fn unique_index_on_partitioned_table_expression_does_not_cover_partition_key() {
    assert_snapshot!(check("
create table pt (
  k int,
  v int
)
  partition by range (k);
create unique index on pt ((k + 1), v);
"), @"
    error: unique index on partitioned table `pt` must include partition key column `k`
      ╭▸ 
    7 │ create unique index on pt ((k + 1), v);
      ╰╴                          ━━━━━━━━━━━━
    ");
}

#[test]
fn partial_unique_index_on_partitioned_table() {
    assert_snapshot!(check("
create table pt (
  k int,
  v int
)
  partition by hash (k);
create unique index on pt (k) where v > 0;
"), @"no diagnostics");
}

#[test]
fn primary_key_on_partitioned_table_missing_partition_key() {
    assert_snapshot!(check("
create table pt (
  k int,
  v int
)
  partition by range (k);
alter table pt add primary key (v);
create table pt2 (
  k int,
  v int,
  primary key (v)
)
  partition by list (k);
"), @"
    error: primary key on partitioned table `pt` must include partition key column `k`
      ╭▸ 
    7 │ alter table pt add primary key (v);
      ╰╴                   ━━━━━━━━━━━━━━━
    error: primary key on partitioned table `pt2` must include partition key column `k`
       ╭▸ 
    11 │   primary key (v)
       ╰╴  ━━━━━━━━━━━━━━━
    ");
}

#[test]
fn unique_constraint_on_partitioned_table_missing_partition_key() {
    assert_snapshot!(check("
create table pt (
  k int,
  v int unique
)
  partition by range (k);
"), @"
    error: unique constraint on partitioned table `pt` must include partition key column `k`
      ╭▸ 
    4 │   v int unique
      ╰╴        ━━━━━━
    ");
}

#[test]
fn unique_constraint_on_partitioned_table_with_partition_key() {
    assert_snapshot!(check("
create table pt (
  k int,
  v int
)
  partition by range (k);
alter table pt add unique (v, k);
create table plain (k int unique);
"), @"no diagnostics");
}

#[test]
fn unique_constraint_on_partitioned_table_with_multi_column_key() {
    assert_snapshot!(check("
create table pt (
  k int,
  j int,
  v int
)
  partition by range (k, j);
alter table pt add unique (k);
alter table pt add unique (j, v, k);
"), @"
    error: unique constraint on partitioned table `pt` must include partition key column `j`
      ╭▸ 
    8 │ alter table pt add unique (k);
      ╰╴                   ━━━━━━━━━━
    ");
}

#[test]
fn unique_constraint_on_partitioned_table_with_expression_key() {
    assert_snapshot!(check("
create table pt (
  k int,
  v int
)
  partition by range ((k + 1));
alter table pt add unique (k);
"), @"
    error: unique constraint is not supported on partitioned table `pt` because its partition key contains expressions
      ╭▸ 
    7 │ alter table pt add unique (k);
      ╰╴                   ━━━━━━━━━━
    ");
}

#[test]
fn unique_constraint_on_partitioned_table_after_partition_key_rename() {
    assert_snapshot!(check("
create table pt (
  k int,
  v int
)
  partition by hash (k);
alter table pt rename column k to kk;
alter table pt add primary key (kk);
begin;
alter table pt rename column kk to k2;
rollback;
alter table pt add unique (kk, v);
"), @"no diagnostics");
}

#[test]
fn overlapping_partitions() {
    assert_snapshot!(check("
create table pt (k int) partition by range (k);
create table pc partition of pt for values from (1) to (10);
create table pc2 partition of pt for values from (5) to (15);
create table pc3 partition of pt for values from (10) to (20);
create table pc4 partition of pt for values from (minvalue) to (1);
create table pc5 partition of pt for values from (-5) to (0);
create table pd partition of pt default;
create table pd2 partition of pt default;
create table pl (k int) partition by list (k);
create table pl1 partition of pl for values in (1, 2);
create table pl2 partition of pl for values in (2, '3');
create table pl3 partition of pl for values in ('3');
create table ph (k int) partition by hash (k);
create table ph1 partition of ph for values with (modulus 4, remainder 1);
create table ph2 partition of ph for values with (modulus 8, remainder 5);
create table ph3 partition of ph for values with (modulus 8, remainder 3);
create table ph4 partition of ph for values with (modulus 4, remainder 1);
create table pt2 (k text) partition by list (k);
create table pt2a partition of pt2 for values in ('a');
create table pt2b partition of pt2 for values in ('a');
create table pm (a int, b int) partition by range (a, b);
create table pm1 partition of pm for values from (1, 1) to (1, 10);
create table pm2 partition of pm for values from (1, 10) to (2, 0);
create table pm3 partition of pm for values from (1, 5) to (1, 6);
create table loose (k int);
alter table pt attach partition loose for values from (12) to (13);
alter table pt detach partition pc;
create table pc6 partition of pt for values from (1) to (10);
create table pr (k text) partition by range (k);
create table pr1 partition of pr for values from ('a') to ('m');
create table pr2 partition of pr for values from ('b') to ('c');
"), @"
    error: partition `pc2` would overlap partition `pc`
      ╭▸ 
    4 │ create table pc2 partition of pt for values from (5) to (15);
      ╰╴                                 ━━━━━━━━━━━━━━━━━━━━━━━━━━━
    error: partition `pc5` would overlap partition `pc4`
      ╭▸ 
    7 │ create table pc5 partition of pt for values from (-5) to (0);
      ╰╴                                 ━━━━━━━━━━━━━━━━━━━━━━━━━━━
    error: partition `pd2` conflicts with existing default partition `pd`
      ╭▸ 
    9 │ create table pd2 partition of pt default;
      ╰╴                                 ━━━━━━━
    error: partition `pl2` would overlap partition `pl1`
       ╭▸ 
    12 │ create table pl2 partition of pl for values in (2, '3');
       ╰╴                                 ━━━━━━━━━━━━━━━━━━━━━━
    error: partition `ph2` would overlap partition `ph1`
       ╭▸ 
    16 │ create table ph2 partition of ph for values with (modulus 8, remainder 5);
       ╰╴                                 ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    error: partition `ph4` would overlap partition `ph1`
       ╭▸ 
    18 │ create table ph4 partition of ph for values with (modulus 4, remainder 1);
       ╰╴                                 ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    error: partition `pt2b` would overlap partition `pt2a`
       ╭▸ 
    21 │ create table pt2b partition of pt2 for values in ('a');
       ╰╴                                   ━━━━━━━━━━━━━━━━━━━
    error: partition `pm3` would overlap partition `pm1`
       ╭▸ 
    25 │ create table pm3 partition of pm for values from (1, 5) to (1, 6);
       ╰╴                                 ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    error: partition `loose` would overlap partition `pc3`
       ╭▸ 
    27 │ alter table pt attach partition loose for values from (12) to (13);
       ╰╴                                      ━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    ");
}

#[test]
fn overlapping_text_list_partitions() {
    assert_snapshot!(check("
create table tl (k text) partition by list (k);
create table tl1
  partition of tl
  for values in (
    'a'
    'b'
  );
create table tl2
  partition of tl
  for values in ('a');
create table tl3
  partition of tl
  for values in ('it''s');
create table tl4
  partition of tl
  for values in ('it''s');
create table tl5
  partition of tl
  for values in (e'x');
create table tl6
  partition of tl
  for values in ('x');
"), @"
    error: partition `tl4` would overlap partition `tl3`
       ╭▸ 
    17 │   for values in ('it''s');
       ╰╴  ━━━━━━━━━━━━━━━━━━━━━━━
    error: partition `tl6` would overlap partition `tl5`
       ╭▸ 
    23 │   for values in ('x');
       ╰╴  ━━━━━━━━━━━━━━━━━━━
    ");
}
