use insta::assert_snapshot;

use super::goto;

#[test]
fn publication_table() {
    assert_snapshot!(goto("
create table t (a int);
create publication p for table t$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:14
      │
    2 │ create table t (a int);
      │              ─ 2. destination
    3 │ create publication p for table t;
      ╰╴                               ─ 1. source
    ");
}

#[test]
fn publication_column() {
    assert_snapshot!(goto("
create table t (a int);
create publication p for table t (a$0);
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create table t (a int);
      │                 ─ 2. destination
    3 │ create publication p for table t (a);
      ╰╴                                  ─ 1. source
    ");
}

#[test]
fn publication_where_column() {
    assert_snapshot!(goto("
create table t (a int);
create publication p for table t where (a$0 > 0);
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create table t (a int);
      │                 ─ 2. destination
    3 │ create publication p for table t where (a > 0);
      ╰╴                                        ─ 1. source
    ");
}

#[test]
fn alter_publication_ref() {
    assert_snapshot!(goto("
create publication p;
alter publication p$0 owner to current_user;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:20
      │
    2 │ create publication p;
      │                    ─ 2. destination
    3 │ alter publication p owner to current_user;
      ╰╴                  ─ 1. source
    ");
}

#[test]
fn drop_subscription_ref() {
    assert_snapshot!(goto("
create subscription s connection 'host=x' publication p;
drop subscription s$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:21
      │
    2 │ create subscription s connection 'host=x' publication p;
      │                     ─ 2. destination
    3 │ drop subscription s;
      ╰╴                  ─ 1. source
    ");
}

#[test]
fn subscription_server() {
    assert_snapshot!(goto("
create foreign data wrapper w;
create server srv foreign data wrapper w;
create subscription s server srv$0 publication p;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:15
      │
    3 │ create server srv foreign data wrapper w;
      │               ─── 2. destination
    4 │ create subscription s server srv publication p;
      ╰╴                             ─── 1. source
    ");
}

#[test]
fn alter_subscription() {
    assert_snapshot!(goto("
create subscription sub connection $$host=localhost$$ publication pub;
alter subscription sub$0 refresh publication;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:21
      │
    2 │ create subscription sub connection $$host=localhost$$ publication pub;
      │                     ─── 2. destination
    3 │ alter subscription sub refresh publication;
      ╰╴                   ─── 1. source
    ");
}
