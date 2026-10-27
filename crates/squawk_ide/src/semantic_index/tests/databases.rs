use insta::assert_snapshot;

use super::check;

#[test]
fn database_missing() {
    assert_snapshot!(check("
create database d1 template nope;
alter database d2 set tablespace pg_default;
reindex database d3;
reindex system d4;
drop database d5;
drop database if exists d6;
create table t (a int);
select nope.public.t.a from t;
"), @"
    error: database `nope` does not exist
      ╭▸ 
    2 │ create database d1 template nope;
      ╰╴                            ━━━━
    error: database `d2` does not exist
      ╭▸ 
    3 │ alter database d2 set tablespace pg_default;
      ╰╴               ━━
    error: database `d3` does not exist
      ╭▸ 
    4 │ reindex database d3;
      ╰╴                 ━━
    error: database `d4` does not exist
      ╭▸ 
    5 │ reindex system d4;
      ╰╴               ━━
    error: database `d5` does not exist
      ╭▸ 
    6 │ drop database d5;
      ╰╴              ━━
    error: database `nope` does not exist
      ╭▸ 
    9 │ select nope.public.t.a from t;
      ╰╴       ━━━━
    ");
}

#[test]
fn database_created_and_dropped() {
    assert_snapshot!(check("
create database d template template0 tablespace pg_default;
alter database d with connection limit 5;
alter database d set search_path = public;
reindex database d;
drop database d;
alter database d reset all;
"), @"
    error: database `d` does not exist
      ╭▸ 
    7 │ alter database d reset all;
      ╰╴               ━
    ");
}

#[test]
fn database_qualified_column() {
    assert_snapshot!(check("
create table t (a int);
select postgres.public.t.a, postgres.public.t.nope, a.b.c.d.e from t;
"), @"
    error: column `t.nope` does not exist
      ╭▸ 
    3 │ select postgres.public.t.a, postgres.public.t.nope, a.b.c.d.e from t;
      ╰╴                                              ━━━━
    ");
}

#[test]
fn alter_database_rename() {
    assert_snapshot!(check("
create database db;
create database other;
alter database db rename to db2;
alter database db owner to current_user;
alter database db2 owner to current_user;
alter database db2 rename to other;
alter database nope rename to x;
begin;
alter database db2 rename to db3;
rollback;
drop database db2;
drop database db3;
"), @"
    error: database `db` does not exist
      ╭▸ 
    5 │ alter database db owner to current_user;
      ╰╴               ━━
    error: database `other` already exists
      ╭▸ 
    7 │ alter database db2 rename to other;
      ╰╴                             ━━━━━
    error: database `nope` does not exist
      ╭▸ 
    8 │ alter database nope rename to x;
      ╰╴               ━━━━
    error: database `db3` does not exist
       ╭▸ 
    13 │ drop database db3;
       ╰╴              ━━━
    ");
}
