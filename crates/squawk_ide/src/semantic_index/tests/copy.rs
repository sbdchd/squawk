use insta::assert_snapshot;

use super::check;

#[test]
fn copy_wrong_relation_kind() {
    assert_snapshot!(check("
create table t (a int);
create view v as select 1 a;
create materialized view m as select 1 a;
create sequence s;
create foreign data wrapper w;
create server srv foreign data wrapper w;
create foreign table ft (a int) server srv;
copy t to stdout;
copy m to stdout;
copy ft to stdout;
copy s to stdout;
copy t from stdin;
copy ft from stdin;
copy v from stdin;
copy m from stdin;
copy s from stdin;
"), @"
    error: cannot copy from foreign table `ft`
       ╭▸ 
    11 │ copy ft to stdout;
       │      ━━
       │
       ╰ hint: try the `copy (select ...) to` variant
    error: cannot copy from sequence `s`
       ╭▸ 
    12 │ copy s to stdout;
       ╰╴     ━
    error: cannot copy to view `v`
       ╭▸ 
    15 │ copy v from stdin;
       │      ━
       │
       ╰ hint: to enable copying to a view, provide an `instead of insert` trigger
    error: cannot copy to materialized view `m`
       ╭▸ 
    16 │ copy m from stdin;
       ╰╴     ━
    error: cannot copy to sequence `s`
       ╭▸ 
    17 │ copy s from stdin;
       ╰╴     ━
    ");
}

#[test]
fn copy() {
    assert_snapshot!(check("
create table t (a int, b int);
create view v as select 1 a;
copy t from stdin;
copy t (a, b) to stdout;
copy t (nope) from stdin;
copy nope from stdin;
copy v to stdout;
copy (select a from t) to stdout;
copy (select nope from t) to stdout;
copy t from stdin where a > 1;
copy t from stdin where nope > 1;
copy t to stdout with (format csv, force_quote (a, nope));
copy t from stdin with (format csv, force_not_null (b), force_null (nope));
copy t to stdout with (format csv, force_quote *);
copy t to stdout csv force quote a, nope;
copy t from stdin csv force not null nope;
"), @"
    error: column `nope` does not exist
      ╭▸ 
    6 │ copy t (nope) from stdin;
      ╰╴        ━━━━
    error: relation `nope` does not exist
      ╭▸ 
    7 │ copy nope from stdin;
      ╰╴     ━━━━
    error: cannot copy from view `v`
      ╭▸ 
    8 │ copy v to stdout;
      │      ━
      │
      ╰ hint: try the `copy (select ...) to` variant
    error: column `nope` does not exist
       ╭▸ 
    10 │ copy (select nope from t) to stdout;
       ╰╴             ━━━━
    error: column `nope` does not exist
       ╭▸ 
    12 │ copy t from stdin where nope > 1;
       ╰╴                        ━━━━
    error: column `nope` does not exist
       ╭▸ 
    13 │ copy t to stdout with (format csv, force_quote (a, nope));
       ╰╴                                                   ━━━━
    error: column `nope` does not exist
       ╭▸ 
    14 │ copy t from stdin with (format csv, force_not_null (b), force_null (nope));
       ╰╴                                                                    ━━━━
    error: column `nope` does not exist
       ╭▸ 
    16 │ copy t to stdout csv force quote a, nope;
       ╰╴                                    ━━━━
    error: column `nope` does not exist
       ╭▸ 
    17 │ copy t from stdin csv force not null nope;
       ╰╴                                     ━━━━
    ");
}
