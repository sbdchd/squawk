use insta::assert_snapshot;

use super::goto;

#[test]
fn create_conversion_function() {
    assert_snapshot!(goto("
create function f(int, int, cstring, internal, int, bool) returns int
  language c
  as 'f';
create conversion c for 'LATIN1' to 'UTF8' from f$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create function f(int, int, cstring, internal, int, bool) returns int
      │                 ─ 2. destination
      ‡
    5 │ create conversion c for 'LATIN1' to 'UTF8' from f;
      ╰╴                                                ─ 1. source
    ");
}

#[test]
fn drop_conversion_ref() {
    assert_snapshot!(goto("
create function f(int, int, cstring, internal, int, bool) returns int
  language c
  as 'f';
create conversion c for 'LATIN1' to 'UTF8' from f;
drop conversion c$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:5:19
      │
    5 │ create conversion c for 'LATIN1' to 'UTF8' from f;
      │                   ─ 2. destination
    6 │ drop conversion c;
      ╰╴                ─ 1. source
    ");
}

#[test]
fn builtin_conversion() {
    assert_snapshot!(goto("
alter conversion utf8_to_iso_8859_1$0 owner to current_user;
"), @"
    info: goto definition
          ╭▸ file0.sql:2:18
          │
        2 │ alter conversion utf8_to_iso_8859_1 owner to current_user;
          │                  ────────────────── 1. source
          │
          ⸬  builtins.sql:38675:38
          │
    38675 │ create default conversion pg_catalog.utf8_to_iso_8859_1
          ╰╴                                     ────────────────── 2. destination
    ");
}

#[test]
fn conversion_set_schema() {
    assert_snapshot!(goto("
create schema s;
create function f(int, int, cstring, internal, int, bool) returns int
  language c
  as 'f';
create conversion c for 'LATIN1' to 'UTF8' from f;
alter conversion c set schema s;
drop conversion s.c$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:6:19
      │
    6 │ create conversion c for 'LATIN1' to 'UTF8' from f;
      │                   ─ 2. destination
    7 │ alter conversion c set schema s;
    8 │ drop conversion s.c;
      ╰╴                  ─ 1. source
    ");
}
