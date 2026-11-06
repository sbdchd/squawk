use insta::assert_snapshot;

use super::check;

#[test]
fn select() {
    assert_snapshot!(check("
create table t (
  a int,
  b bool
);
select a from t where a + 1;
select a from t group by a having sum(a);
select count(*) filter (where a) from t;
select a from t where b and 'true';
select a from t where null;
"), @"
    error: argument of `where` must be type `bool`, not type `int4`
      ╭▸ 
    6 │ select a from t where a + 1;
      ╰╴                      ━━━━━
    error: argument of `having` must be type `bool`, not type `int8`
      ╭▸ 
    7 │ select a from t group by a having sum(a);
      ╰╴                                  ━━━━━━
    error: argument of `filter` must be type `bool`, not type `int4`
      ╭▸ 
    8 │ select count(*) filter (where a) from t;
      ╰╴                              ━
    ");
}

#[test]
fn join() {
    assert_snapshot!(check("
create table t (a int);
create table p (a int);
select * from t join p on t.a;
"), @"
    error: argument of `on` must be type `bool`, not type `int4`
      ╭▸ 
    4 │ select * from t join p on t.a;
      ╰╴                          ━━━
    ");
}

#[test]
fn expressions() {
    assert_snapshot!(check("
create table t (
  a int,
  b bool,
  c text
);
select case when a then 1 when b then 2 end, case a when 1 then 1 end from t;
select a and b, b or c, not a from t;
"), @"
    error: argument of `when` must be type `bool`, not type `int4`
      ╭▸ 
    7 │ select case when a then 1 when b then 2 end, case a when 1 then 1 end from t;
      ╰╴                 ━
    error: argument of `and` must be type `bool`, not type `int4`
      ╭▸ 
    8 │ select a and b, b or c, not a from t;
      ╰╴       ━
    error: argument of `or` must be type `bool`, not type `text`
      ╭▸ 
    8 │ select a and b, b or c, not a from t;
      ╰╴                     ━
    error: argument of `not` must be type `bool`, not type `int4`
      ╭▸ 
    8 │ select a and b, b or c, not a from t;
      ╰╴                            ━
    ");
}

#[test]
fn delete() {
    assert_snapshot!(check("
create table t (c text);
delete from t where c;
"), @"
    error: argument of `where` must be type `bool`, not type `text`
      ╭▸ 
    3 │ delete from t where c;
      ╰╴                    ━
    ");
}

#[test]
fn update() {
    assert_snapshot!(check("
create table t (a int);
update t set a = 1 where a;
"), @"
    error: argument of `where` must be type `bool`, not type `int4`
      ╭▸ 
    3 │ update t set a = 1 where a;
      ╰╴                         ━
    ");
}

#[test]
fn merge() {
    assert_snapshot!(check("
create table t (
  a int,
  b bool
);
create table u (
  a int,
  b bool
);
merge into t
using u on u.a
when matched and t.a then
  delete;
"), @"
    error: argument of `on` must be type `bool`, not type `int4`
       ╭▸ 
    11 │ using u on u.a
       ╰╴           ━━━
    error: argument of `when` must be type `bool`, not type `int4`
       ╭▸ 
    12 │ when matched and t.a then
       ╰╴                 ━━━
    ");
}

#[test]
fn create_table() {
    assert_snapshot!(check("
create table u (
  a int,
  check (a)
);
"), @"
    error: argument of `check` must be type `bool`, not type `int4`
      ╭▸ 
    4 │   check (a)
      ╰╴         ━
    ");
}

#[test]
fn create_domain() {
    assert_snapshot!(check("
create domain d as int check (value);
"), @"
    error: argument of `check` must be type `bool`, not type `int4`
      ╭▸ 
    2 │ create domain d as int check (value);
      ╰╴                              ━━━━━
    ");
}

#[test]
fn create_policy() {
    assert_snapshot!(check("
create table t (
  a int,
  c text
);
create policy p on t using (a) with check (c);
"), @"
    error: argument of `policy` must be type `bool`, not type `int4`
      ╭▸ 
    6 │ create policy p on t using (a) with check (c);
      ╰╴                            ━
    error: argument of `policy` must be type `bool`, not type `text`
      ╭▸ 
    6 │ create policy p on t using (a) with check (c);
      ╰╴                                           ━
    ");
}

#[test]
fn create_trigger() {
    assert_snapshot!(check("
create table t (a int);
create function f() returns trigger
  language plpgsql
  as 'begin return new; end';
create trigger tr
  before insert
  on t
  for each row
  when (new.a)
  execute function f();
"), @"
    error: argument of `when` must be type `bool`, not type `int4`
       ╭▸ 
    10 │   when (new.a)
       ╰╴        ━━━━━
    ");
}

#[test]
fn create_index() {
    assert_snapshot!(check("
create table t (a int);
create index i on t (a) where a;
"), @"
    error: argument of `where` must be type `bool`, not type `int4`
      ╭▸ 
    3 │ create index i on t (a) where a;
      ╰╴                              ━
    ");
}

#[test]
fn create_publication() {
    assert_snapshot!(check("
create table t (a int);
create publication pub for table t where (a);
"), @"
    error: argument of `where` must be type `bool`, not type `int4`
      ╭▸ 
    3 │ create publication pub for table t where (a);
      ╰╴                                          ━
    ");
}

#[test]
fn copy() {
    assert_snapshot!(check("
create table t (a int);
copy t from '/tmp/t' where a;
"), @"
    error: argument of `where` must be type `bool`, not type `int4`
      ╭▸ 
    3 │ copy t from '/tmp/t' where a;
      ╰╴                           ━
    ");
}

#[test]
fn in_list() {
    assert_snapshot!(check("
create function eqi(int, text) returns int
  language sql
  return 1;
create operator = (
  leftarg = int,
  rightarg = text,
  function = eqi
);
create operator <> (
  leftarg = int,
  rightarg = text,
  function = eqi
);
create table t (
  a int,
  b text
);
select 1 from t where a in (1, 2) and a in ('1', '2') and a in (1, 2.5);
select 1 from t where a in ('x'::text);
select 1 from t where a not in ('x'::text, 'y'::text);
select 1 from t where a in (b);
select 1 from t where a in (1, now());
create function eqp(point, text) returns int
  language sql
  return 1;
create operator = (
  leftarg = point,
  rightarg = text,
  function = eqp
);
create table p (p point);
select 1 from p where p in ('x');
"), @"
    error: operator `int4 = text` must return type `bool`, not type `int4`
       ╭▸ 
    20 │ select 1 from t where a in ('x'::text);
       │                             ━━━━━━━━━
       │
       ╰ note: `in` compares each element using `=`
    error: operator `int4 <> text` must return type `bool`, not type `int4`
       ╭▸ 
    21 │ select 1 from t where a not in ('x'::text, 'y'::text);
       │                                 ━━━━━━━━━
       │
       ╰ note: `not in` compares each element using `<>`
    error: operator `int4 <> text` must return type `bool`, not type `int4`
       ╭▸ 
    21 │ select 1 from t where a not in ('x'::text, 'y'::text);
       │                                            ━━━━━━━━━
       │
       ╰ note: `not in` compares each element using `<>`
    error: operator `int4 = text` must return type `bool`, not type `int4`
       ╭▸ 
    22 │ select 1 from t where a in (b);
       │                             ━
       │
       ╰ note: `in` compares each element using `=`
    error: operator does not exist: `int4 = timestamptz`
       ╭▸ 
    23 │ select 1 from t where a in (1, now());
       │                                ━━━━━
       │
       ╰ hint: consider adding explicit type casts
    error: operator `point = text` must return type `bool`, not type `int4`
       ╭▸ 
    33 │ select 1 from p where p in ('x');
       │                             ━━━
       │
       ╰ note: `in` compares each element using `=`
    ");
}

#[test]
fn quantified_non_array() {
    assert_snapshot!(check("
create domain ints as int[];
create table t (
  a int,
  b int[],
  c ints,
  p point
);
select 1 = any(1);
select 1 from t where a = any(a);
select 1 from t where a = some(a + 1);
select 1 from t where a <> all(a);
select 1 from t where p = any(p);
select 1 from t where a = any(b) and a = any(c) and a = all('{1}') and a = any(null);
select 1 from t where a = any(array[1, 2]) and a = any(select a from t);
"), @"
    error: argument of `any` must be an array, not type `int4`
      ╭▸ 
    9 │ select 1 = any(1);
      ╰╴               ━
    error: argument of `any` must be an array, not type `int4`
       ╭▸ 
    10 │ select 1 from t where a = any(a);
       ╰╴                              ━
    error: argument of `some` must be an array, not type `int4`
       ╭▸ 
    11 │ select 1 from t where a = some(a + 1);
       ╰╴                               ━━━━━
    error: argument of `all` must be an array, not type `int4`
       ╭▸ 
    12 │ select 1 from t where a <> all(a);
       ╰╴                               ━
    error: operator `=` does not exist
       ╭▸ 
    13 │ select 1 from t where p = any(p);
       ╰╴                        ━
    error: argument of `any` must be an array, not type `point`
       ╭▸ 
    13 │ select 1 from t where p = any(p);
       ╰╴                              ━
    ");
}

#[test]
fn join_using() {
    assert_snapshot!(check("
create function eqi(int, text) returns int
  language sql
  return 1;
create operator = (
  leftarg = int,
  rightarg = text,
  function = eqi
);
create table t (a int);
create table p (a text);
select 1 from t join p using (a);
"), @"
    error: operator `int4 = text` must return type `bool`, not type `int4`
       ╭▸ 
    12 │ select 1 from t join p using (a);
       │                               ━
       │
       ╰ note: `using` compares the joined columns using `=`
    ");
}

#[test]
fn graph_table() {
    assert_snapshot!(check("
create table person (
  id int,
  name text
);
create table knows (
  src int,
  dst int
);
create property graph g
  vertex tables (person)
  edge tables (knows source person destination person);
select *
from graph_table(
  g
  match
    (a is person where a.id)-[e is knows]->(b)
  where b.name
  columns (a.name as n)
);
"), @"
    error: argument of `where` must be type `bool`, not type `int4`
       ╭▸ 
    17 │     (a is person where a.id)-[e is knows]->(b)
       ╰╴                       ━━━━
    error: argument of `where` must be type `bool`, not type `text`
       ╭▸ 
    18 │   where b.name
       ╰╴        ━━━━━━
    ");
}
