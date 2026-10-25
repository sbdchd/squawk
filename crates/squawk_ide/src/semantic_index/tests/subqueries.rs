use insta::assert_snapshot;

use super::check;

#[test]
fn subquery_not_allowed() {
    assert_snapshot!(check("
create table t (
  a int,
  b int
);
create procedure p(x int)
  language sql
  as 'select 1';
call p((select 1));
create function f(x int default (select 1)) returns int
  language sql
  as 'select 1';
create table d (a int default (select 1));
create table c (a int check (a > (select 1)));
create table g (
  a int,
  b int generated always as ((select 1)) stored
);
create index on t ((a + (select 1)));
create index on t (a) where exists(select 1);
alter table t alter column b type bigint using (select 1);
prepare s(int) as
  select $1;
execute s((select 1));
create domain dm as int check (value > (select 1));
create table pt (a int) partition by range ((a + (select 1)));
create table pt2 (a int) partition by range (a);
create table pt2_1
  partition of pt2
  for values from ((select 1)) to (10);
create table d2 (a int default (1 + (select (select 1))));
create table d3 (
  a int check (a = any(select 1)),
  b int check (b = any(array(select 1)))
);
select (select 1) from t where a in (select 1) and exists(select 1);
create policy pol on t using (a in (select 1));
"), @"
    error: cannot use subquery in CALL argument
      ╭▸ 
    9 │ call p((select 1));
      ╰╴        ━━━━━━━━
    error: cannot use subquery in DEFAULT expression
       ╭▸ 
    10 │ create function f(x int default (select 1)) returns int
       ╰╴                                 ━━━━━━━━
    error: cannot use subquery in DEFAULT expression
       ╭▸ 
    13 │ create table d (a int default (select 1));
       ╰╴                               ━━━━━━━━
    error: cannot use subquery in check constraint
       ╭▸ 
    14 │ create table c (a int check (a > (select 1)));
       ╰╴                                  ━━━━━━━━
    error: cannot use subquery in column generation expression
       ╭▸ 
    17 │   b int generated always as ((select 1)) stored
       ╰╴                              ━━━━━━━━
    error: cannot use subquery in index expression
       ╭▸ 
    19 │ create index on t ((a + (select 1)));
       ╰╴                         ━━━━━━━━
    error: cannot use subquery in index predicate
       ╭▸ 
    20 │ create index on t (a) where exists(select 1);
       ╰╴                                   ━━━━━━━━
    error: cannot use subquery in transform expression
       ╭▸ 
    21 │ alter table t alter column b type bigint using (select 1);
       ╰╴                                                ━━━━━━━━
    error: cannot use subquery in EXECUTE parameter
       ╭▸ 
    24 │ execute s((select 1));
       ╰╴           ━━━━━━━━
    error: cannot use subquery in check constraint
       ╭▸ 
    25 │ create domain dm as int check (value > (select 1));
       ╰╴                                        ━━━━━━━━
    error: cannot use subquery in partition key expression
       ╭▸ 
    26 │ create table pt (a int) partition by range ((a + (select 1)));
       ╰╴                                                  ━━━━━━━━
    error: cannot use subquery in partition bound
       ╭▸ 
    30 │   for values from ((select 1)) to (10);
       ╰╴                    ━━━━━━━━
    error: cannot use subquery in DEFAULT expression
       ╭▸ 
    31 │ create table d2 (a int default (1 + (select (select 1))));
       ╰╴                                     ━━━━━━━━━━━━━━━━━
    error: cannot use subquery in check constraint
       ╭▸ 
    33 │   a int check (a = any(select 1)),
       ╰╴                       ━━━━━━━━
    error: cannot use subquery in check constraint
       ╭▸ 
    34 │   b int check (b = any(array(select 1)))
       ╰╴                             ━━━━━━━━
    ");
}
