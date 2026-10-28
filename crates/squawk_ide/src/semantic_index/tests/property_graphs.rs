use insta::assert_snapshot;

use super::check;

#[test]
fn create_property_graph() {
    assert_snapshot!(check("
create table person (id int, name text);
create table knows (src int, dst int, since date);
create property graph g
  vertex tables (person key (id) label human properties (name, id + 1 as next_id, missing_col))
  edge tables (
    knows key (src, dst)
      source key (src) references person (id)
      destination key (missing_src) references person (missing_dst)
      default label
  );
create property graph g2
  vertex tables (person, person as p2 no properties, missing_table)
  edge tables (knows source person destination missing_vertex);
create property graph g3 vertex tables (person, person);
create property graph g;
create table g (a int);
"), @"
    error: column `missing_col` does not exist
      ╭▸ 
    5 │   vertex tables (person key (id) label human properties (name, id + 1 as next_id, missing_col))
      ╰╴                                                                                  ━━━━━━━━━━━
    error: column `missing_src` does not exist
      ╭▸ 
    9 │       destination key (missing_src) references person (missing_dst)
      ╰╴                       ━━━━━━━━━━━
    error: column `missing_dst` does not exist
      ╭▸ 
    9 │       destination key (missing_src) references person (missing_dst)
      ╰╴                                                       ━━━━━━━━━━━
    error: relation `missing_table` does not exist
       ╭▸ 
    13 │   vertex tables (person, person as p2 no properties, missing_table)
       ╰╴                                                     ━━━━━━━━━━━━━
    error: element table `missing_vertex` does not exist
       ╭▸ 
    14 │   edge tables (knows source person destination missing_vertex);
       ╰╴                                               ━━━━━━━━━━━━━━
    error: element table `person` already exists
       ╭▸ 
    15 │ create property graph g3 vertex tables (person, person);
       ╰╴                                                ━━━━━━
    error: relation `g` already exists
       ╭▸ 
    16 │ create property graph g;
       ╰╴                      ━
    error: relation `g` already exists
       ╭▸ 
    17 │ create table g (a int);
       ╰╴             ━
    ");
}

#[test]
fn alter_property_graph() {
    assert_snapshot!(check("
create role r;
create table person (id int, name text);
create table knows (src int, dst int);
create property graph g vertex tables (person);
alter property graph g add vertex tables (person as p2 label a label b);
alter property graph g add edge tables (knows source person destination p2);
alter property graph g alter vertex table p2 add label c properties (name, zz);
alter property graph g alter vertex table p2 drop label a;
alter property graph g alter vertex table p2 drop label a;
alter property graph g alter vertex table p2 alter label b add properties (id as ident, yy);
alter property graph g alter vertex table missing add label d;
alter property graph g alter edge table person add label d;
alter property graph g drop edge tables (knows);
alter property graph g drop edge tables (knows);
alter property graph g drop vertex tables (p2, missing);
alter property graph g owner to r;
alter property graph missing owner to r;
alter property graph if exists missing owner to r;
"), @"
    error: column `zz` does not exist
      ╭▸ 
    8 │ alter property graph g alter vertex table p2 add label c properties (name, zz);
      ╰╴                                                                           ━━
    error: label `a` does not exist
       ╭▸ 
    10 │ alter property graph g alter vertex table p2 drop label a;
       ╰╴                                                        ━
    error: column `yy` does not exist
       ╭▸ 
    11 │ alter property graph g alter vertex table p2 alter label b add properties (id as ident, yy);
       ╰╴                                                                                        ━━
    error: element table `missing` does not exist
       ╭▸ 
    12 │ alter property graph g alter vertex table missing add label d;
       ╰╴                                          ━━━━━━━
    error: element table `person` does not exist
       ╭▸ 
    13 │ alter property graph g alter edge table person add label d;
       ╰╴                                        ━━━━━━
    error: element table `knows` does not exist
       ╭▸ 
    15 │ alter property graph g drop edge tables (knows);
       ╰╴                                         ━━━━━
    error: element table `missing` does not exist
       ╭▸ 
    16 │ alter property graph g drop vertex tables (p2, missing);
       ╰╴                                               ━━━━━━━
    error: relation `missing` does not exist
       ╭▸ 
    18 │ alter property graph missing owner to r;
       ╰╴                     ━━━━━━━
    ");
}

#[test]
fn drop_property_graph() {
    assert_snapshot!(check("
create table t (a int);
create property graph g vertex tables (t);
drop property graph g;
drop property graph g;
drop property graph if exists g;
drop property graph t;
begin;
create property graph g2 vertex tables (t);
rollback;
drop property graph g2;
"), @"
    error: relation `g` does not exist
      ╭▸ 
    5 │ drop property graph g;
      ╰╴                    ━
    error: `t` is not a property graph
      ╭▸ 
    7 │ drop property graph t;
      │                     ━
      │
      ╰ hint: use `drop table` to remove a table
    error: relation `g2` does not exist
       ╭▸ 
    11 │ drop property graph g2;
       ╰╴                    ━━
    ");
}

#[test]
fn property_graph_dropped_table() {
    assert_snapshot!(check("
create table person (id int);
create table city (id int);
create table knows (src int, dst int);
create table lives_in (src int, dst int);
create property graph g
  vertex tables (person, city)
  edge tables (
    knows source person destination person,
    lives_in source person destination city
  );
drop table city cascade;
alter property graph g alter vertex table city add label c;
alter property graph g alter edge table lives_in add label l;
alter property graph g alter edge table knows add label k;
begin;
drop table knows cascade;
rollback;
alter property graph g alter edge table knows add label k2;
drop table knows cascade;
alter property graph g alter edge table knows add label k3;
"), @"
    error: element table `city` does not exist
       ╭▸ 
    13 │ alter property graph g alter vertex table city add label c;
       ╰╴                                          ━━━━
    error: element table `lives_in` does not exist
       ╭▸ 
    14 │ alter property graph g alter edge table lives_in add label l;
       ╰╴                                        ━━━━━━━━
    error: element table `knows` does not exist
       ╭▸ 
    21 │ alter property graph g alter edge table knows add label k3;
       ╰╴                                        ━━━━━
    ");
}

#[test]
fn graph_table() {
    assert_snapshot!(check("
create table person (id int, name text);
create table city (id int, title text);
create table knows (src int, dst int, since date);
create property graph g
  vertex tables (
    person label human properties (name, id * 2 as double_id),
    city no properties
  )
  edge tables (knows source person destination person);
select gt.n, gt.d, gt.since, gt.missing
from graph_table(
  g match (a is human where a.name = 'x')-[e is knows]->(b where b.double_id > 1)
  where a.id = 1 and e.since is not null and z.name = 'y'
  columns (a.name as n, a.double_id as d, e.since)
) as gt;
select * from graph_table(g match (c is city) columns (c.title as t));
select * from graph_table(g match (a is missing_label | human) columns (a.name as n));
select * from graph_table(missing_graph match (a) columns (a.name as n));
"), @"
    error: column `gt.missing` does not exist
       ╭▸ 
    11 │ select gt.n, gt.d, gt.since, gt.missing
       ╰╴                                ━━━━━━━
    error: column `a.id` does not exist
       ╭▸ 
    14 │   where a.id = 1 and e.since is not null and z.name = 'y'
       ╰╴          ━━
    error: missing `from` clause entry for table `z`
       ╭▸ 
    14 │   where a.id = 1 and e.since is not null and z.name = 'y'
       ╰╴                                             ━
    error: column `c.title` does not exist
       ╭▸ 
    17 │ select * from graph_table(g match (c is city) columns (c.title as t));
       ╰╴                                                         ━━━━━
    error: label `missing_label` does not exist
       ╭▸ 
    18 │ select * from graph_table(g match (a is missing_label | human) columns (a.name as n));
       ╰╴                                        ━━━━━━━━━━━━━
    error: relation `missing_graph` does not exist
       ╭▸ 
    19 │ select * from graph_table(missing_graph match (a) columns (a.name as n));
       ╰╴                          ━━━━━━━━━━━━━
    ");
}

#[test]
fn drop_graph_properties() {
    assert_snapshot!(check("
create table person (id int, name text);
create property graph g vertex tables (person properties (id, name as n));
alter property graph g alter vertex table person alter label person drop properties (n);
alter property graph g alter vertex table person alter label person drop properties (n, missing);
alter property graph g alter vertex table person alter label person add properties (id as n, id);
select * from graph_table(g match (p) columns (p.n as x, p.name as y));
"), @"
    error: property `n` does not exist
      ╭▸ 
    5 │ alter property graph g alter vertex table person alter label person drop properties (n, missing);
      ╰╴                                                                                     ━
    error: property `missing` does not exist
      ╭▸ 
    5 │ alter property graph g alter vertex table person alter label person drop properties (n, missing);
      ╰╴                                                                                        ━━━━━━━
    error: property `id` already exists
      ╭▸ 
    6 │ alter property graph g alter vertex table person alter label person add properties (id as n, id);
      ╰╴                                                                                             ━━
    error: column `p.name` does not exist
      ╭▸ 
    7 │ select * from graph_table(g match (p) columns (p.n as x, p.name as y));
      ╰╴                                                           ━━━━
    ");
}

#[test]
fn alter_property_graph_rename() {
    assert_snapshot!(check("
create table t (a int);
create property graph g vertex tables (t);
alter property graph g rename to g2;
select * from graph_table(g match (x) columns (x.a as a));
select * from graph_table(g2 match (x) columns (x.a as a));
alter property graph g2 rename to t;
alter property graph nope rename to g3;
alter property graph if exists nope rename to g3;
"), @"
    error: relation `g` does not exist
      ╭▸ 
    5 │ select * from graph_table(g match (x) columns (x.a as a));
      │                           ━
      ╰╴
    help: a relation with a similar name exists
      ╭╴
    5 │ select * from graph_table(g2 match (x) columns (x.a as a));
      ╰╴                           +
    error: relation `t` already exists
      ╭▸ 
    7 │ alter property graph g2 rename to t;
      ╰╴                                  ━
    error: relation `nope` does not exist
      ╭▸ 
    8 │ alter property graph nope rename to g3;
      ╰╴                     ━━━━
    ");
}

#[test]
fn alter_property_graph_set_schema() {
    assert_snapshot!(check("
create schema sc;
create table t (a int);
create property graph g vertex tables (t);
alter property graph g set schema sc;
drop property graph g;
drop property graph sc.g;
alter property graph t set schema sc;
"), @"
    error: relation `g` does not exist
      ╭▸ 
    6 │ drop property graph g;
      ╰╴                    ━
    error: `t` is not a property graph
      ╭▸ 
    8 │ alter property graph t set schema sc;
      ╰╴                     ━
    ");
}

#[test]
fn graph_table_invalid_label_expr() {
    assert_snapshot!(check("
create table person (id int, name text);
create property graph g vertex tables (person);
select * from graph_table(g match (a is 1) columns (a.name as n));
select * from graph_table(g match (a is person.x) columns (a.name as n));
select * from graph_table(g match (a is f(1)) columns (a.name as n));
"), @"no diagnostics");
}
