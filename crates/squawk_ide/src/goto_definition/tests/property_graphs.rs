use insta::assert_snapshot;

use super::goto;

#[test]
fn property_graph_vertex_table() {
    assert_snapshot!(goto("
create table person (id int);
create property graph g vertex tables (person$0);
"), @"
    info: goto definition
      ╭▸ file0.sql:2:14
      │
    2 │ create table person (id int);
      │              ────── 2. destination
    3 │ create property graph g vertex tables (person);
      ╰╴                                       ────── 1. source
    ");
}

#[test]
fn property_graph_key_column() {
    assert_snapshot!(goto("
create table person (id int);
create property graph g vertex tables (person key (id$0));
"), @"
    info: goto definition
      ╭▸ file0.sql:2:22
      │
    2 │ create table person (id int);
      │                      ── 2. destination
    3 │ create property graph g vertex tables (person key (id));
      ╰╴                                                   ── 1. source
    ");
}

#[test]
fn property_graph_property_column() {
    assert_snapshot!(goto("
create table person (
  id int,
  name text
);
create property graph g vertex tables (person properties (name$0 as n));
"), @"
    info: goto definition
      ╭▸ file0.sql:4:3
      │
    4 │   name text
      │   ──── 2. destination
    5 │ );
    6 │ create property graph g vertex tables (person properties (name as n));
      ╰╴                                                          ──── 1. source
    ");
}

#[test]
fn property_graph_edge_source_vertex() {
    assert_snapshot!(goto("
create table person (id int);
create table knows (
  src int,
  dst int
);
create property graph g
  vertex tables (person as p)
  edge tables (knows source key (src) references p$0 (id) destination p);
"), @"
    info: goto definition
      ╭▸ file0.sql:8:28
      │
    8 │   vertex tables (person as p)
      │                            ─ 2. destination
    9 │   edge tables (knows source key (src) references p (id) destination p);
      ╰╴                                                 ─ 1. source
    ");
}

#[test]
fn property_graph_edge_references_column() {
    assert_snapshot!(goto("
create table person (id int);
create table knows (
  src int,
  dst int
);
create property graph g
  vertex tables (person)
  edge tables (
    knows source key (src) references person (id$0) destination person
  );
"), @"
    info: goto definition
       ╭▸ file0.sql:2:22
       │
     2 │ create table person (id int);
       │                      ── 2. destination
       ‡
    10 │     knows source key (src) references person (id) destination person
       ╰╴                                              ── 1. source
    ");
}

#[test]
fn alter_property_graph_ref() {
    assert_snapshot!(goto("
create table person (id int);
create property graph g vertex tables (person);
alter property graph g$0 owner to current_user;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:23
      │
    3 │ create property graph g vertex tables (person);
      │                       ─ 2. destination
    4 │ alter property graph g owner to current_user;
      ╰╴                     ─ 1. source
    ");
}

#[test]
fn alter_property_graph_element_ref() {
    assert_snapshot!(goto("
create table person (id int);
create property graph g vertex tables (person as p);
alter property graph g alter vertex table p$0 add label x;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:50
      │
    3 │ create property graph g vertex tables (person as p);
      │                                                  ─ 2. destination
    4 │ alter property graph g alter vertex table p add label x;
      ╰╴                                          ─ 1. source
    ");
}

#[test]
fn alter_property_graph_label_ref() {
    assert_snapshot!(goto("
create table person (id int);
create property graph g vertex tables (person label human label other);
alter property graph g alter vertex table person drop label human$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:53
      │
    3 │ create property graph g vertex tables (person label human label other);
      │                                                     ───── 2. destination
    4 │ alter property graph g alter vertex table person drop label human;
      ╰╴                                                            ───── 1. source
    ");
}

#[test]
fn alter_property_graph_default_label_ref() {
    assert_snapshot!(goto("
create table person (id int);
create property graph g vertex tables (person);
alter property graph g alter vertex table person drop label person$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:40
      │
    3 │ create property graph g vertex tables (person);
      │                                        ────── 2. destination
    4 │ alter property graph g alter vertex table person drop label person;
      ╰╴                                                            ────── 1. source
    ");
}

#[test]
fn drop_property_graph_ref() {
    assert_snapshot!(goto("
create table person (id int);
create property graph g vertex tables (person);
drop property graph g$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:23
      │
    3 │ create property graph g vertex tables (person);
      │                       ─ 2. destination
    4 │ drop property graph g;
      ╰╴                    ─ 1. source
    ");
}

#[test]
fn alter_property_graph_unaliased_element_ref() {
    assert_snapshot!(goto("
create table person (id int);
create property graph g vertex tables (person);
alter property graph g alter vertex table person$0 add label x;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:40
      │
    3 │ create property graph g vertex tables (person);
      │                                        ────── 2. destination
    4 │ alter property graph g alter vertex table person add label x;
      ╰╴                                          ────── 1. source
    ");
}

#[test]
fn graph_table_graph() {
    assert_snapshot!(goto("
create table person (id int);
create property graph g vertex tables (person);
select * from graph_table(g$0 match (a) columns (a.id));
"), @"
    info: goto definition
      ╭▸ file0.sql:3:23
      │
    3 │ create property graph g vertex tables (person);
      │                       ─ 2. destination
    4 │ select * from graph_table(g match (a) columns (a.id));
      ╰╴                          ─ 1. source
    ");
}

#[test]
fn graph_table_label() {
    assert_snapshot!(goto("
create table person (id int);
create property graph g vertex tables (person label human);
select * from graph_table(g match (a is human$0) columns (a.id));
"), @"
    info: goto definition
      ╭▸ file0.sql:3:53
      │
    3 │ create property graph g vertex tables (person label human);
      │                                                     ───── 2. destination
    4 │ select * from graph_table(g match (a is human) columns (a.id));
      ╰╴                                        ───── 1. source
    ");
}

#[test]
fn graph_table_element_variable() {
    assert_snapshot!(goto("
create table person (id int);
create property graph g vertex tables (person);
select * from graph_table(g match (a) columns (a$0.id));
"), @"
    info: goto definition
      ╭▸ file0.sql:4:36
      │
    4 │ select * from graph_table(g match (a) columns (a.id));
      │                                    ┬           ─ 1. source
      │                                    │
      ╰╴                                   2. destination
    ");
}

#[test]
fn graph_table_column_property() {
    assert_snapshot!(goto("
create table person (id int);
create property graph g vertex tables (person);
select * from graph_table(g match (a) where a.id$0 > 0 columns (a.id));
"), @"
    info: goto definition
      ╭▸ file0.sql:2:22
      │
    2 │ create table person (id int);
      │                      ── 2. destination
    3 │ create property graph g vertex tables (person);
    4 │ select * from graph_table(g match (a) where a.id > 0 columns (a.id));
      ╰╴                                              ── 1. source
    ");
}

#[test]
fn graph_table_named_property() {
    assert_snapshot!(goto("
create table person (id int);
create property graph g vertex tables (person properties (id + 1 as next));
select * from graph_table(g match (a) columns (a.next$0));
"), @"
    info: goto definition
      ╭▸ file0.sql:3:69
      │
    3 │ create property graph g vertex tables (person properties (id + 1 as next));
      │                                                                     ──── 2. destination
    4 │ select * from graph_table(g match (a) columns (a.next));
      ╰╴                                                 ──── 1. source
    ");
}

#[test]
fn graph_table_output_column() {
    assert_snapshot!(goto("
create table person (id int);
create property graph g vertex tables (person);
select gt.ident$0 from graph_table(g match (a) columns (a.id as ident)) gt;
"), @"
    info: goto definition
      ╭▸ file0.sql:4:11
      │
    4 │ select gt.ident from graph_table(g match (a) columns (a.id as ident)) gt;
      ╰╴          ───── 1. source                                     ───── 2. destination
    ");
}

#[test]
fn drop_graph_property_ref() {
    assert_snapshot!(goto("
create table person (id int);
create property graph g vertex tables (person properties (id + 1 as next));
alter property graph g
  alter vertex table person alter label person drop properties (next$0);
"), @"
    info: goto definition
      ╭▸ file0.sql:3:69
      │
    3 │ create property graph g vertex tables (person properties (id + 1 as next));
      │                                                                     ──── 2. destination
    4 │ alter property graph g
    5 │   alter vertex table person alter label person drop properties (next);
      ╰╴                                                                ──── 1. source
    ");
}

#[test]
fn alter_property_graph_renamed_ref() {
    assert_snapshot!(goto("
create table t (a int);
create property graph g vertex tables (t);
alter property graph g rename to g2;
drop property graph g2$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:4:34
      │
    4 │ alter property graph g rename to g2;
      │                                  ── 2. destination
    5 │ drop property graph g2;
      ╰╴                    ── 1. source
    ");
}
