use insta::assert_snapshot;

use super::hover;

#[test]
fn property_graph() {
    assert_snapshot!(hover("
create table v (id int primary key);
create property graph g vertex tables (v);
select * from graph_table(g$0 match (x) columns (x.id));
"), @"
    hover: property graph public.g
      ╭▸ file0.sql:4:27
      │
    4 │ select * from graph_table(g match (x) columns (x.id));
      ╰╴                          ─ hover
    ");
}

#[test]
fn vertex_table() {
    assert_snapshot!(hover("
create table v (id int primary key);
create property graph g vertex tables (v);
alter property graph g drop vertex tables (v$0);
"), @"
    hover: vertex table public.g.v
      ╭▸ file0.sql:4:44
      │
    4 │ alter property graph g drop vertex tables (v);
      ╰╴                                           ─ hover
    ");
}

#[test]
fn label() {
    assert_snapshot!(hover("
create table v (id int primary key);
create property graph g vertex tables (v label person);
select * from graph_table(g match (x is person$0) columns (x.id));
"), @"
    hover: label person
      ╭▸ file0.sql:4:41
      │
    4 │ select * from graph_table(g match (x is person) columns (x.id));
      ╰╴                                        ────── hover
    ");
}

#[test]
fn property() {
    assert_snapshot!(hover("
create table v (id int primary key);
create property graph g vertex tables (v);
select * from graph_table(g match (x) columns (x.id$0));
"), @"
    hover: column id int4
      ╭▸ file0.sql:4:50
      │
    4 │ select * from graph_table(g match (x) columns (x.id));
      ╰╴                                                 ── hover
    ");
}
