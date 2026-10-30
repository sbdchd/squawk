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
create table d partition of p (b) for values from (10) to (MAXVALUE);
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
