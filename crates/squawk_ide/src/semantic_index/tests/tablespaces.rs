use insta::assert_snapshot;

use super::check;

#[test]
fn tablespace_missing() {
    assert_snapshot!(check("
create table t (a int, b int, unique (a) using index tablespace ts2) tablespace ts1;
create index i on t (b) tablespace ts3;
alter table t set tablespace ts4;
alter index i set tablespace ts5;
alter table all in tablespace ts6 set tablespace ts7;
reindex (tablespace ts8) table t;
create table u tablespace ts9 as select 1;
alter tablespace ts10 set (seq_page_cost = 1);
drop tablespace ts11;
drop tablespace if exists ts12;
"), @"
    error: tablespace `ts1` does not exist
      ╭▸ 
    2 │ create table t (a int, b int, unique (a) using index tablespace ts2) tablespace ts1;
      ╰╴                                                                                ━━━
    error: tablespace `ts2` does not exist
      ╭▸ 
    2 │ create table t (a int, b int, unique (a) using index tablespace ts2) tablespace ts1;
      ╰╴                                                                ━━━
    error: tablespace `ts3` does not exist
      ╭▸ 
    3 │ create index i on t (b) tablespace ts3;
      ╰╴                                   ━━━
    error: tablespace `ts4` does not exist
      ╭▸ 
    4 │ alter table t set tablespace ts4;
      ╰╴                             ━━━
    error: tablespace `ts5` does not exist
      ╭▸ 
    5 │ alter index i set tablespace ts5;
      ╰╴                             ━━━
    error: tablespace `ts6` does not exist
      ╭▸ 
    6 │ alter table all in tablespace ts6 set tablespace ts7;
      ╰╴                              ━━━
    error: tablespace `ts7` does not exist
      ╭▸ 
    6 │ alter table all in tablespace ts6 set tablespace ts7;
      ╰╴                                                 ━━━
    error: tablespace `ts8` does not exist
      ╭▸ 
    7 │ reindex (tablespace ts8) table t;
      ╰╴                    ━━━
    error: tablespace `ts9` does not exist
      ╭▸ 
    8 │ create table u tablespace ts9 as select 1;
      ╰╴                          ━━━
    error: tablespace `ts10` does not exist
      ╭▸ 
    9 │ alter tablespace ts10 set (seq_page_cost = 1);
      ╰╴                 ━━━━
    error: tablespace `ts11` does not exist
       ╭▸ 
    10 │ drop tablespace ts11;
       ╰╴                ━━━━
    ");
}

#[test]
fn tablespace_created_and_dropped() {
    assert_snapshot!(check("
create tablespace ts location '/data';
create table t (a int) tablespace ts;
create index i on t (a) tablespace pg_default;
alter table t set tablespace pg_global;
drop tablespace ts;
create table u (a int) tablespace ts;
"), @"
    error: tablespace `ts` does not exist
      ╭▸ 
    7 │ create table u (a int) tablespace ts;
      ╰╴                                  ━━
    ");
}

#[test]
fn duplicate_tablespace_and_database() {
    assert_snapshot!(check("
create tablespace ts location '/tmp';
create tablespace ts location '/tmp';
create database d;
create database d;
"), @"
    error: tablespace `ts` already exists
      ╭▸ 
    3 │ create tablespace ts location '/tmp';
      ╰╴                  ━━
    error: database `d` already exists
      ╭▸ 
    5 │ create database d;
      ╰╴                ━
    ");
}

#[test]
fn alter_tablespace_rename() {
    assert_snapshot!(check("
create tablespace ts location '/data';
create tablespace other location '/other';
alter tablespace ts rename to ts2;
create table t (a int) tablespace ts;
create table u (a int) tablespace ts2;
alter tablespace ts2 rename to other;
alter tablespace nope rename to x;
begin;
alter tablespace ts2 rename to ts3;
rollback;
create table v (a int) tablespace ts2;
create table w (a int) tablespace ts3;
"), @"
    error: tablespace `ts` does not exist
      ╭▸ 
    5 │ create table t (a int) tablespace ts;
      ╰╴                                  ━━
    error: tablespace `nope` does not exist
      ╭▸ 
    8 │ alter tablespace nope rename to x;
      ╰╴                 ━━━━
    error: tablespace `ts3` does not exist
       ╭▸ 
    13 │ create table w (a int) tablespace ts3;
       ╰╴                                  ━━━
    error: tablespace `other` already exists
      ╭▸ 
    7 │ alter tablespace ts2 rename to other;
      ╰╴                               ━━━━━
    ");
}
