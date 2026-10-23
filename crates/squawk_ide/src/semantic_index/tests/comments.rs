use insta::assert_snapshot;

use super::check;

#[test]
fn comment_on() {
    assert_snapshot!(check("
create table t (a int constraint c check (a > 0));
create view v as select 1 a;
create materialized view m as select 1 a;
create sequence s;
create index i on t (a);
create type ty as (x int);
create domain d as int constraint dc check (value > 0);
create function f(int) returns int language sql as 'select 1';
create procedure p() language sql as 'select 1';
create schema sc;
create role r;
create policy pol on t;
create function tf() returns trigger language plpgsql as 'begin return new; end';
create trigger tr before insert on t for each row execute function tf();
create rule ru as on insert to t do nothing;
create collation co (locale = 'C');
comment on table t is 'x';
comment on table nope is 'x';
comment on table v is 'x';
comment on view v is 'x';
comment on view nope is 'x';
comment on materialized view m is 'x';
comment on materialized view nope is 'x';
comment on sequence s is 'x';
comment on sequence nope is 'x';
comment on index i is 'x';
comment on index nope is 'x';
comment on column t.a is 'x';
comment on column t.nope is 'x';
comment on column v.a is null;
comment on column nope.a is 'x';
comment on constraint c on t is 'x';
comment on constraint nope on t is 'x';
comment on constraint dc on domain d is 'x';
comment on constraint nope on domain d is 'x';
comment on type ty is 'x';
comment on type nope is 'x';
comment on domain d is 'x';
comment on domain nope is 'x';
comment on function f(int) is 'x';
comment on function f(text) is 'x';
comment on procedure p() is 'x';
comment on procedure nope() is 'x';
comment on routine f(int) is 'x';
comment on schema sc is 'x';
comment on schema nope is 'x';
comment on role r is 'x';
comment on role nope is 'x';
comment on policy pol on t is 'x';
comment on policy nope on t is 'x';
comment on trigger tr on t is 'x';
comment on trigger nope on t is 'x';
comment on rule ru on t is 'x';
comment on rule nope on t is 'x';
comment on collation co is 'x';
comment on collation nope is 'x';
comment on operator + (int, int) is 'x';
comment on operator + (int, nope) is 'x';
comment on cast (int as text) is 'x';
comment on language sql is 'x';
comment on language nope is 'x';
comment on extension nope is 'x';
comment on access method btree is 'x';
comment on access method nope is 'x';
comment on text search configuration english is 'x';
comment on text search configuration nope is 'x';
"), @"
    error: relation `nope` does not exist
       ╭▸ 
    19 │ comment on table nope is 'x';
       ╰╴                 ━━━━
    error: relation `nope` does not exist
       ╭▸ 
    22 │ comment on view nope is 'x';
       ╰╴                ━━━━
    error: relation `nope` does not exist
       ╭▸ 
    24 │ comment on materialized view nope is 'x';
       ╰╴                             ━━━━
    error: relation `nope` does not exist
       ╭▸ 
    26 │ comment on sequence nope is 'x';
       ╰╴                    ━━━━
    error: index `nope` does not exist
       ╭▸ 
    28 │ comment on index nope is 'x';
       ╰╴                 ━━━━
    error: column `nope` does not exist
       ╭▸ 
    30 │ comment on column t.nope is 'x';
       ╰╴                    ━━━━
    error: relation `nope` does not exist
       ╭▸ 
    32 │ comment on column nope.a is 'x';
       ╰╴                  ━━━━
    error: constraint `nope` does not exist
       ╭▸ 
    34 │ comment on constraint nope on t is 'x';
       ╰╴                      ━━━━
    error: constraint `nope` does not exist
       ╭▸ 
    36 │ comment on constraint nope on domain d is 'x';
       ╰╴                      ━━━━
    error: type `nope` does not exist
       ╭▸ 
    38 │ comment on type nope is 'x';
       ╰╴                ━━━━
    error: type `nope` does not exist
       ╭▸ 
    40 │ comment on domain nope is 'x';
       ╰╴                  ━━━━
    error: function `f` does not exist
       ╭▸ 
    42 │ comment on function f(text) is 'x';
       ╰╴                    ━
    error: procedure `nope` does not exist
       ╭▸ 
    44 │ comment on procedure nope() is 'x';
       ╰╴                     ━━━━
    error: schema `nope` does not exist
       ╭▸ 
    47 │ comment on schema nope is 'x';
       ╰╴                  ━━━━
    error: role `nope` does not exist
       ╭▸ 
    49 │ comment on role nope is 'x';
       ╰╴                ━━━━
    error: policy `nope` does not exist
       ╭▸ 
    51 │ comment on policy nope on t is 'x';
       ╰╴                  ━━━━
    error: trigger `nope` does not exist
       ╭▸ 
    53 │ comment on trigger nope on t is 'x';
       ╰╴                   ━━━━
    error: rule `nope` does not exist
       ╭▸ 
    55 │ comment on rule nope on t is 'x';
       ╰╴                ━━━━
    error: collation `nope` does not exist
       ╭▸ 
    57 │ comment on collation nope is 'x';
       ╰╴                     ━━━━
    error: type `nope` does not exist
       ╭▸ 
    59 │ comment on operator + (int, nope) is 'x';
       ╰╴                            ━━━━
    error: language `nope` does not exist
       ╭▸ 
    62 │ comment on language nope is 'x';
       ╰╴                    ━━━━
    error: extension `nope` does not exist
       ╭▸ 
    63 │ comment on extension nope is 'x';
       ╰╴                     ━━━━
    error: access method `nope` does not exist
       ╭▸ 
    65 │ comment on access method nope is 'x';
       ╰╴                         ━━━━
    error: text search configuration `nope` does not exist
       ╭▸ 
    67 │ comment on text search configuration nope is 'x';
       ╰╴                                     ━━━━
    error: `v` is not a table
       ╭▸ 
    20 │ comment on table v is 'x';
       ╰╴                 ━
    ");
}

#[test]
fn security_label() {
    assert_snapshot!(check("
create table t (a int);
create role r;
security label on table t is 'x';
security label for selinux on table nope is 'x';
security label on column t.a is null;
security label on column t.nope is 'x';
security label on role r is 'x';
security label on role nope is 'x';
security label on schema nope is 'x';
"), @"
    error: relation `nope` does not exist
      ╭▸ 
    5 │ security label for selinux on table nope is 'x';
      ╰╴                                    ━━━━
    error: column `nope` does not exist
      ╭▸ 
    7 │ security label on column t.nope is 'x';
      ╰╴                           ━━━━
    error: role `nope` does not exist
      ╭▸ 
    9 │ security label on role nope is 'x';
      ╰╴                       ━━━━
    error: schema `nope` does not exist
       ╭▸ 
    10 │ security label on schema nope is 'x';
       ╰╴                         ━━━━
    ");
}
