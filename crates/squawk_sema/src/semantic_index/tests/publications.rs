use insta::assert_snapshot;

use super::check;

#[test]
fn create_publication() {
    assert_snapshot!(check("
create schema s;
create table t (a int, b int);
create table u (c int);
create publication p for table t (a, b) where (a > 0 and zz = 1), only u;
create publication p2 for table t (a, missing_col), missing_table;
create publication p3 for tables in schema s, current_schema;
create publication p4 for tables in schema missing_schema;
create publication p5 for all tables;
create publication p6 with (publish = 'insert');
create publication p;
"), @"
    error: column `zz` does not exist
      ╭▸ 
    5 │ create publication p for table t (a, b) where (a > 0 and zz = 1), only u;
      ╰╴                                                         ━━
    error: column `missing_col` does not exist
      ╭▸ 
    6 │ create publication p2 for table t (a, missing_col), missing_table;
      ╰╴                                      ━━━━━━━━━━━
    error: relation `missing_table` does not exist
      ╭▸ 
    6 │ create publication p2 for table t (a, missing_col), missing_table;
      ╰╴                                                    ━━━━━━━━━━━━━
    error: schema `missing_schema` does not exist
      ╭▸ 
    8 │ create publication p4 for tables in schema missing_schema;
      ╰╴                                           ━━━━━━━━━━━━━━
    error: publication `p` already exists
       ╭▸ 
    11 │ create publication p;
       ╰╴                   ━
    ");
}

#[test]
fn alter_publication() {
    assert_snapshot!(check("
create role r;
create table t (a int);
create publication p;
create publication p2;
alter publication p add table t where (a > 1);
alter publication p add table missing_table;
alter publication p drop table t;
alter publication p set table t (a, missing_col);
alter publication p set (publish = 'update');
alter publication p owner to r;
alter publication p owner to missing_role;
alter publication missing add table t;
alter publication p rename to p2;
alter publication p rename to p3;
alter publication p3 add table t;
alter publication p add table t;
"), @"
    error: relation `missing_table` does not exist
      ╭▸ 
    7 │ alter publication p add table missing_table;
      ╰╴                              ━━━━━━━━━━━━━
    error: column `missing_col` does not exist
      ╭▸ 
    9 │ alter publication p set table t (a, missing_col);
      ╰╴                                    ━━━━━━━━━━━
    error: role `missing_role` does not exist
       ╭▸ 
    12 │ alter publication p owner to missing_role;
       ╰╴                             ━━━━━━━━━━━━
    error: publication `missing` does not exist
       ╭▸ 
    13 │ alter publication missing add table t;
       ╰╴                  ━━━━━━━
    error: publication `p2` already exists
       ╭▸ 
    14 │ alter publication p rename to p2;
       ╰╴                              ━━
    error: publication `p` does not exist
       ╭▸ 
    17 │ alter publication p add table t;
       ╰╴                  ━
    ");
}

#[test]
fn drop_publication() {
    assert_snapshot!(check("
create publication p;
create publication p2;
drop publication p, p2;
drop publication p;
drop publication if exists p, missing;
begin;
create publication p3;
rollback;
drop publication p3;
"), @"
    error: publication `p` does not exist
      ╭▸ 
    5 │ drop publication p;
      ╰╴                 ━
    error: publication `p3` does not exist
       ╭▸ 
    10 │ drop publication p3;
       ╰╴                 ━━
    ");
}

#[test]
fn subscription() {
    assert_snapshot!(check("
create role r;
create foreign data wrapper w;
create server srv foreign data wrapper w;
create subscription sub connection 'host=x' publication remote_pub, other_pub;
create subscription sub2 server srv publication remote_pub;
create subscription sub3 server missing_server publication remote_pub;
create subscription sub connection 'host=x' publication remote_pub;
alter subscription sub add publication another_pub;
alter subscription sub set publication remote_pub with (refresh = false);
alter subscription sub drop publication other_pub;
alter subscription sub refresh publication;
alter subscription sub enable;
alter subscription sub disable;
alter subscription sub connection 'host=y';
alter subscription sub server srv;
alter subscription sub server missing_server;
alter subscription sub owner to r;
alter subscription sub owner to missing_role;
alter subscription missing enable;
alter subscription sub rename to sub2;
alter subscription sub rename to sub4;
drop subscription sub;
drop subscription sub4;
drop subscription if exists sub4;
"), @"
    error: server `missing_server` does not exist
      ╭▸ 
    7 │ create subscription sub3 server missing_server publication remote_pub;
      ╰╴                                ━━━━━━━━━━━━━━
    error: subscription `sub` already exists
      ╭▸ 
    8 │ create subscription sub connection 'host=x' publication remote_pub;
      ╰╴                    ━━━
    error: server `missing_server` does not exist
       ╭▸ 
    17 │ alter subscription sub server missing_server;
       ╰╴                              ━━━━━━━━━━━━━━
    error: role `missing_role` does not exist
       ╭▸ 
    19 │ alter subscription sub owner to missing_role;
       ╰╴                                ━━━━━━━━━━━━
    error: subscription `missing` does not exist
       ╭▸ 
    20 │ alter subscription missing enable;
       ╰╴                   ━━━━━━━
    error: subscription `sub2` already exists
       ╭▸ 
    21 │ alter subscription sub rename to sub2;
       ╰╴                                 ━━━━
    ");
}
