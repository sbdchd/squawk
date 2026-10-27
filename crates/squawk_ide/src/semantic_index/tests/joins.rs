use insta::assert_snapshot;

use super::check;

#[test]
fn using_merges_columns() {
    assert_snapshot!(check("
with a as (select 1 id), b as (select 2 id)
select id from a join b using (id);
"), @"no diagnostics");
}

#[test]
fn lateral_sources() {
    assert_snapshot!(check("
create table a (x int);
create table b (y int);
select * from a, generate_series(1, x) g;
select * from generate_series(1, x) g, a;
select * from a, rows from (generate_series(1, x)) g;
select * from rows from (generate_series(1, x)) g, a;
select * from a, lateral (select x, y) w, b;
select * from a, (select x) v;
select * from a left join lateral (select a.x) v on true;
select * from a join (b cross join lateral (select x, y) v) on true;
select * from (b cross join lateral (select x) v cross join a);
"), @"
    error: column `x` does not exist
      ╭▸ 
    5 │ select * from generate_series(1, x) g, a;
      ╰╴                                 ━
    error: column `x` does not exist
      ╭▸ 
    7 │ select * from rows from (generate_series(1, x)) g, a;
      ╰╴                                            ━
    error: column `y` does not exist
      ╭▸ 
    8 │ select * from a, lateral (select x, y) w, b;
      │                                     ━
      ╰╴
    help: a column with a similar name exists
      ╭╴
    8 - select * from a, lateral (select x, y) w, b;
    8 + select * from a, lateral (select x, x) w, b;
      ╰╴
    error: column `x` does not exist
      ╭▸ 
    9 │ select * from a, (select x) v;
      ╰╴                         ━
    error: column `x` does not exist
       ╭▸ 
    12 │ select * from (b cross join lateral (select x) v cross join a);
       │                                             ━
       ╰╴
    help: a column with a similar name exists
       ╭╴
    12 - select * from (b cross join lateral (select x) v cross join a);
    12 + select * from (b cross join lateral (select y) v cross join a);
       ╰╴
    ");
}

#[test]
fn right_join_hides_lhs() {
    assert_snapshot!(check("
create table a (x int);
create table b (y int);
create table c (z int);
select * from a right join lateral (select x) v on true;
select * from a full join generate_series(1, x) g on true;
select * from a left join lateral (select x) v on true;
select * from c, a right join lateral (select z, a.x) v on true;
select * from a right join (b cross join lateral (select x, y) v) on true;
select * from a right join b on true cross join lateral (select x, y) v;
"), @"
    error: column `x` does not exist
      ╭▸ 
    5 │ select * from a right join lateral (select x) v on true;
      ╰╴                                           ━
    error: column `x` does not exist
      ╭▸ 
    6 │ select * from a full join generate_series(1, x) g on true;
      ╰╴                                             ━
    error: missing `from` clause entry for table `a`
      ╭▸ 
    8 │ select * from c, a right join lateral (select z, a.x) v on true;
      ╰╴                                                 ━
    error: column `x` does not exist
      ╭▸ 
    9 │ select * from a right join (b cross join lateral (select x, y) v) on true;
      │                                                          ━
      ╰╴
    help: a column with a similar name exists
      ╭╴
    9 - select * from a right join (b cross join lateral (select x, y) v) on true;
    9 + select * from a right join (b cross join lateral (select y, y) v) on true;
      ╰╴
    ");
}

#[test]
fn join_condition_scope() {
    assert_snapshot!(check("
create table a (x int);
create table b (y int);
create table c (z int);
select * from a join b on x = y;
select * from a, b join c on x = z;
select * from b join c on x = z, a;
select * from a join (b join c on a.x = y) on true;
select * from a join (b join c on y = z) on x = z;
select * from a join b on exists (select 1 from c where z = x and y = 1);
select * from a join b using (nope);
select * from c, a join b using (z);
select (select 1 from b join c on x = y) from a;
"), @"
    error: column `x` does not exist
      ╭▸ 
    6 │ select * from a, b join c on x = z;
      │                              ━
      ╰╴
    help: a column with a similar name exists
      ╭╴
    6 - select * from a, b join c on x = z;
    6 + select * from a, b join c on y = z;
      ╰╴
    error: column `x` does not exist
      ╭▸ 
    7 │ select * from b join c on x = z, a;
      │                           ━
      ╰╴
    help: a column with a similar name exists
      ╭╴
    7 - select * from b join c on x = z, a;
    7 + select * from b join c on y = z, a;
      ╰╴
    error: missing `from` clause entry for table `a`
      ╭▸ 
    8 │ select * from a join (b join c on a.x = y) on true;
      ╰╴                                  ━
    error: column `nope` does not exist
       ╭▸ 
    11 │ select * from a join b using (nope);
       ╰╴                              ━━━━
    error: column `z` does not exist
       ╭▸ 
    12 │ select * from c, a join b using (z);
       ╰╴                                 ━
    ");
}

#[test]
fn join_alias() {
    assert_snapshot!(check("
create table a (x int, y int);
create table b (x int, z int);
select j.x, j.y, j.z, j.w from (a join b using (x)) as j;
select a.x, b.z from (a join b using (x)) as j;
select j.x, j.y, j.z from (a natural join b) as j;
select j.p, j.q, j.r, j.x from (a join b using (x)) as j(p, q);
select x, y, z from (a join b using (x)) as j;
"), @"
    error: column `j.w` does not exist
      ╭▸ 
    4 │ select j.x, j.y, j.z, j.w from (a join b using (x)) as j;
      ╰╴                        ━
    error: missing `from` clause entry for table `a`
      ╭▸ 
    5 │ select a.x, b.z from (a join b using (x)) as j;
      ╰╴       ━
    error: missing `from` clause entry for table `b`
      ╭▸ 
    5 │ select a.x, b.z from (a join b using (x)) as j;
      ╰╴            ━
    error: column `j.r` does not exist
      ╭▸ 
    7 │ select j.p, j.q, j.r, j.x from (a join b using (x)) as j(p, q);
      ╰╴                   ━
    error: column `j.x` does not exist
      ╭▸ 
    7 │ select j.p, j.q, j.r, j.x from (a join b using (x)) as j(p, q);
      ╰╴                        ━
    ");
}

#[test]
fn join_using_alias() {
    assert_snapshot!(check("
create table a (x int, y int);
create table b (x int, z int);
create table c (w int);
select k.x, k.y from a join b using (x) as k;
select a.y, b.z, x from a join b using (x) as k;
select * from a join b using (x) as k, generate_series(1, k.x) g;
select * from a join b using (x) as k, (select k.x) v;
select * from a join b using (x) as k, c join a as d on k.x = w;
create view v (p, q, r, s) as select * from a join b using (x) as k;
select x from v;
create view u as select k.* from a join b using (x) as k;
select y from u;
"), @"
    error: column `k.y` does not exist
      ╭▸ 
    5 │ select k.x, k.y from a join b using (x) as k;
      ╰╴              ━
    error: missing `from` clause entry for table `k`
      ╭▸ 
    8 │ select * from a join b using (x) as k, (select k.x) v;
      ╰╴                                               ━
    error: missing `from` clause entry for table `k`
      ╭▸ 
    9 │ select * from a join b using (x) as k, c join a as d on k.x = w;
      ╰╴                                                        ━
    error: `create view` specifies more column names than columns
       ╭▸ 
    10 │ create view v (p, q, r, s) as select * from a join b using (x) as k;
       ╰╴                        ━
    error: column `x` does not exist
       ╭▸ 
    11 │ select x from v;
       │        ━
       ╰╴
    help: a column with a similar name exists
       ╭╴
    11 - select x from v;
    11 + select p from v;
       ╰╴
    error: column `y` does not exist
       ╭▸ 
    13 │ select y from u;
       │        ━
       ╰╴
    help: a column with a similar name exists
       ╭╴
    13 - select y from u;
    13 + select x from u;
       ╰╴
    ");
}

#[test]
fn merged_columns() {
    assert_snapshot!(check("
create table a (x int, y int);
create table b (x int, z int);
create table c (x int);
select x from a join b using (x);
select x from a join b using (x), c;
select x from a natural join b;
select x from a natural join b, c;
select x, y, z from a natural join b;
select x from a join b using (x) join c using (x);
select x from a join b on true;
"), @"
    error: column reference `x` is ambiguous
      ╭▸ 
    6 │ select x from a join b using (x), c;
      ╰╴       ━
    error: column reference `x` is ambiguous
      ╭▸ 
    8 │ select x from a natural join b, c;
      ╰╴       ━
    error: column reference `x` is ambiguous
       ╭▸ 
    11 │ select x from a join b on true;
       ╰╴       ━
    ");
}

#[test]
fn right_join_lateral_hidden_source_blocks_outer_column() {
    assert_snapshot!(check("
create table t (a int);
create table o (a int);
select * from o where exists (
  select 1 from t right join lateral (select a) s on true
);
"), @"
    error: column `a` does not exist
      ╭▸ 
    5 │   select 1 from t right join lateral (select a) s on true
      ╰╴                                             ━
    ");
}
