use insta::assert_snapshot;

use super::check;

#[test]
fn create_operator_family() {
    assert_snapshot!(check("
create operator family f using btree;
create operator family s.f2 using hash;
create operator family f3 using missing;
create operator family f3 using heap;
create operator family f using btree;
create operator family f using hash;
create operator family integer_ops using btree;
create operator family pg_catalog.integer_ops using btree;
"), @"
    error: schema `s` does not exist
      ╭▸ 
    3 │ create operator family s.f2 using hash;
      ╰╴                       ━
    error: access method `missing` does not exist
      ╭▸ 
    4 │ create operator family f3 using missing;
      ╰╴                                ━━━━━━━
    error: access method `heap` is not of type INDEX
      ╭▸ 
    5 │ create operator family f3 using heap;
      ╰╴                                ━━━━
    error: operator family `f` for access method `btree` already exists
      ╭▸ 
    6 │ create operator family f using btree;
      ╰╴                       ━
    error: operator family `integer_ops` for access method `btree` already exists
      ╭▸ 
    9 │ create operator family pg_catalog.integer_ops using btree;
      ╰╴                                  ━━━━━━━━━━━
    ");
}

#[test]
fn create_operator_class() {
    assert_snapshot!(check("
create type t;
create function t_cmp(t, t) returns int language c as 'f';
create function t_lt(t, t) returns bool language c as 'f';
create operator < (leftarg = t, rightarg = t, function = t_lt);
create operator family f using btree;
create operator class c default for type t using btree family f as
  operator 1 <,
  operator 2 <= (t, t),
  operator 3 = (int, int) for search,
  operator 4 <-> for order by integer_ops,
  operator 5 <-> for order by missing_family,
  function 1 t_cmp(t, t),
  function 2 (t, t) missing_fn(t, t),
  storage missing_type;
create operator class c for type missing_type using btree family missing_family as storage int;
create operator class c2 for type t using btree as storage t;
drop operator family c2 using btree;
create operator class c for type t using hash as storage t;
"), @"
    error: operator `<=` does not exist
      ╭▸ 
    9 │   operator 2 <= (t, t),
      ╰╴             ━━
    error: operator `<->` does not exist
       ╭▸ 
    11 │   operator 4 <-> for order by integer_ops,
       ╰╴             ━━━
    error: operator `<->` does not exist
       ╭▸ 
    12 │   operator 5 <-> for order by missing_family,
       ╰╴             ━━━
    error: operator family `missing_family` does not exist
       ╭▸ 
    12 │   operator 5 <-> for order by missing_family,
       ╰╴                              ━━━━━━━━━━━━━━
    error: function `missing_fn` does not exist
       ╭▸ 
    14 │   function 2 (t, t) missing_fn(t, t),
       ╰╴                    ━━━━━━━━━━
    error: type `missing_type` does not exist
       ╭▸ 
    15 │   storage missing_type;
       ╰╴          ━━━━━━━━━━━━
    error: type `missing_type` does not exist
       ╭▸ 
    16 │ create operator class c for type missing_type using btree family missing_family as storage int;
       ╰╴                                 ━━━━━━━━━━━━
    error: operator family `missing_family` does not exist
       ╭▸ 
    16 │ create operator class c for type missing_type using btree family missing_family as storage int;
       ╰╴                                                                 ━━━━━━━━━━━━━━
    error: operator class `c` for access method `btree` already exists
       ╭▸ 
    16 │ create operator class c for type missing_type using btree family missing_family as storage int;
       ╰╴                      ━
    ");
}

#[test]
fn alter_operator_family() {
    assert_snapshot!(check("
create role r;
create schema s;
create function cmp(int, int) returns int language c as 'f';
create operator family f using btree;
create operator family f2 using btree;
alter operator family f using btree add
  operator 1 < (int, int),
  operator 2 <<< (int, int),
  function 1 cmp(int, int),
  function 2 (int, int) missing_fn(int, int);
alter operator family f using btree drop
  operator 1 (int, int),
  function 1 (int, missing_type);
alter operator family f using btree owner to r;
alter operator family f using btree owner to missing_role;
alter operator family missing using btree owner to r;
alter operator family f using hash owner to r;
alter operator family f using btree rename to f2;
alter operator family f using btree rename to f3;
alter operator family f3 using btree set schema s;
alter operator family s.f3 using btree owner to r;
alter operator family f3 using btree owner to r;
"), @"
    error: operator `<<<` does not exist
      ╭▸ 
    9 │   operator 2 <<< (int, int),
      ╰╴             ━━━
    error: function `missing_fn` does not exist
       ╭▸ 
    11 │   function 2 (int, int) missing_fn(int, int);
       ╰╴                        ━━━━━━━━━━
    error: type `missing_type` does not exist
       ╭▸ 
    14 │   function 1 (int, missing_type);
       ╰╴                   ━━━━━━━━━━━━
    error: role `missing_role` does not exist
       ╭▸ 
    16 │ alter operator family f using btree owner to missing_role;
       ╰╴                                             ━━━━━━━━━━━━
    error: operator family `missing` does not exist
       ╭▸ 
    17 │ alter operator family missing using btree owner to r;
       ╰╴                      ━━━━━━━
    error: operator family `f` does not exist
       ╭▸ 
    18 │ alter operator family f using hash owner to r;
       ╰╴                      ━
    error: operator family `f3` does not exist
       ╭▸ 
    23 │ alter operator family f3 using btree owner to r;
       ╰╴                      ━━
    error: operator family `f2` for access method `btree` already exists
       ╭▸ 
    19 │ alter operator family f using btree rename to f2;
       ╰╴                                              ━━
    ");
}

#[test]
fn alter_operator_class() {
    assert_snapshot!(check("
create role r;
create schema s;
create operator class c for type int using btree as storage int;
create operator class c2 for type int using btree as storage int;
alter operator class c using btree owner to r;
alter operator class c using btree owner to missing_role;
alter operator class missing using btree owner to r;
alter operator class c using btree rename to c2;
alter operator class c using btree rename to c3;
alter operator class c3 using btree set schema s;
alter operator class s.c3 using btree owner to r;
alter operator class c3 using btree owner to r;
"), @"
    error: role `missing_role` does not exist
      ╭▸ 
    7 │ alter operator class c using btree owner to missing_role;
      ╰╴                                            ━━━━━━━━━━━━
    error: operator class `missing` does not exist
      ╭▸ 
    8 │ alter operator class missing using btree owner to r;
      ╰╴                     ━━━━━━━
    error: operator class `c3` does not exist
       ╭▸ 
    13 │ alter operator class c3 using btree owner to r;
       ╰╴                     ━━
    error: operator class `c2` for access method `btree` already exists
      ╭▸ 
    9 │ alter operator class c using btree rename to c2;
      ╰╴                                             ━━
    ");
}

#[test]
fn drop_operator_class_and_family() {
    assert_snapshot!(check("
create operator family f using btree;
create operator class c for type int using btree family f as storage int;
drop operator class c using btree;
drop operator class c using btree;
drop operator class if exists c using btree;
drop operator family f using btree;
drop operator family f using btree;
drop operator family if exists f using btree;
drop operator class int4_ops using hash;
drop operator class int4_ops using brin;
"), @"
    error: operator class `c` does not exist
      ╭▸ 
    5 │ drop operator class c using btree;
      ╰╴                    ━
    error: operator family `f` does not exist
      ╭▸ 
    8 │ drop operator family f using btree;
      ╰╴                     ━
    error: operator class `int4_ops` does not exist
       ╭▸ 
    11 │ drop operator class int4_ops using brin;
       ╰╴                    ━━━━━━━━
    ");
}

#[test]
fn operator_class_rollback() {
    assert_snapshot!(check("
create operator class c for type int using btree as storage int;
begin;
drop operator class c using btree;
create operator family f using btree;
rollback;
drop operator class c using btree;
drop operator family f using btree;
"), @"
    error: operator family `f` does not exist
      ╭▸ 
    8 │ drop operator family f using btree;
      ╰╴                     ━
    ");
}

#[test]
fn index_operator_class() {
    assert_snapshot!(check("
create table t (a int, b text);
create index on t (a int4_ops, b text_pattern_ops);
create index on t using hash (a int4_ops);
create index on t using gist (a missing_ops);
create index on t using brin (a int4_minmax_ops);
create index on t using brin (a text_pattern_ops);
create index on t (a pg_catalog.int4_ops);
insert into t values (1, 'x') on conflict (a int4_ops) do nothing;
insert into t values (1, 'x') on conflict (a missing_ops) do nothing;
"), @"
    error: operator class `missing_ops` does not exist
      ╭▸ 
    5 │ create index on t using gist (a missing_ops);
      ╰╴                                ━━━━━━━━━━━
    error: operator class `text_pattern_ops` does not exist
      ╭▸ 
    7 │ create index on t using brin (a text_pattern_ops);
      ╰╴                                ━━━━━━━━━━━━━━━━
    error: operator class `missing_ops` does not exist
       ╭▸ 
    10 │ insert into t values (1, 'x') on conflict (a missing_ops) do nothing;
       ╰╴                                             ━━━━━━━━━━━
    ");
}

#[test]
fn alter_operator_family_drop_member() {
    assert_snapshot!(check("
create function cmp(int, int) returns int language sql return 0;
create function cmp_big(bigint, bigint) returns int language sql return 0;
create operator family fam using btree;
alter operator family fam using btree add
  function 1 (int, int) cmp(int, int),
  function 1 (bigint, bigint) cmp_big(bigint, bigint);
alter operator family fam using btree drop function 2 (int, int);
alter operator family fam using btree drop function 1 (bigint);
drop function cmp(int, int);
begin;
alter operator family fam using btree drop function 1 (int, int);
rollback;
drop function cmp(int, int);
alter operator family fam using btree drop function 1 (int, int);
drop function cmp(int, int);
drop function cmp_big(bigint, bigint);
"), @"
    error: cannot drop function `cmp` because other objects depend on it
       ╭▸ 
    10 │ drop function cmp(int, int);
       ╰╴              ━━━
    error: cannot drop function `cmp` because other objects depend on it
       ╭▸ 
    14 │ drop function cmp(int, int);
       ╰╴              ━━━
    ");
}
