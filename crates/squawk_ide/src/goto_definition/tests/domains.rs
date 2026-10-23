use insta::assert_snapshot;

use super::goto;

#[test]
fn drop_domain() {
    assert_snapshot!(goto("
create domain posint as int;
drop domain posint$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:15
      │
    2 │ create domain posint as int;
      │               ────── 2. destination
    3 │ drop domain posint;
      ╰╴            ────── 1. source
    ");
}

#[test]
fn domain_check_value() {
    assert_snapshot!(goto("
create domain d as int check (value$0 > 0);
"), @"
    info: goto definition
      ╭▸ file0.sql:2:15
      │
    2 │ create domain d as int check (value > 0);
      │               ┬               ───── 1. source
      │               │
      ╰╴              2. destination
    ");
}

#[test]
fn alter_domain_check_value() {
    assert_snapshot!(goto("
create domain d as int;
alter domain d add check (value$0 > 0);
"), @"
    info: goto definition
      ╭▸ file0.sql:2:15
      │
    2 │ create domain d as int;
      │               ─ 2. destination
    3 │ alter domain d add check (value > 0);
      ╰╴                          ───── 1. source
    ");
}

#[test]
fn alter_domain_ref() {
    assert_snapshot!(goto("
create domain d as int;
alter domain d$0 set not null;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:15
      │
    2 │ create domain d as int;
      │               ─ 2. destination
    3 │ alter domain d set not null;
      ╰╴             ─ 1. source
    ");
}

#[test]
fn alter_domain_drop_constraint_ref() {
    assert_snapshot!(goto("
create domain d as int constraint c check (value > 0);
alter domain d drop constraint c$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:35
      │
    2 │ create domain d as int constraint c check (value > 0);
      │                                   ─ 2. destination
    3 │ alter domain d drop constraint c;
      ╰╴                               ─ 1. source
    ");
}

#[test]
fn alter_domain_renamed_constraint_ref() {
    assert_snapshot!(goto("
create domain d as int constraint c check (value > 0);
alter domain d rename constraint c to c2;
alter domain d validate constraint c2$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:39
      │
    3 │ alter domain d rename constraint c to c2;
      │                                       ── 2. destination
    4 │ alter domain d validate constraint c2;
      ╰╴                                   ── 1. source
    ");
}

#[test]
fn alter_domain_rename_ref() {
    assert_snapshot!(goto("
create domain d as int;
alter domain d rename to e;
select 1::e$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:26
      │
    3 │ alter domain d rename to e;
      │                          ─ 2. destination
    4 │ select 1::e;
      ╰╴          ─ 1. source
    ");
}

#[test]
fn drop_domain_on_non_domain_ref() {
    assert_snapshot!(goto("
create type t as (
  a int
);
drop domain t$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:13
      │
    2 │ create type t as (
      │             ─ 2. destination
      ‡
    5 │ drop domain t;
      ╰╴            ─ 1. source
    ");
}
