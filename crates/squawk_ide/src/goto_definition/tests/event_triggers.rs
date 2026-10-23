use insta::assert_snapshot;

use super::goto;

#[test]
fn create_event_trigger_function() {
    assert_snapshot!(goto("
create function f() returns event_trigger
  language plpgsql
  as 'begin end';
create event trigger e on ddl_command_start execute function f$0();
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create function f() returns event_trigger
      │                 ─ 2. destination
      ‡
    5 │ create event trigger e on ddl_command_start execute function f();
      ╰╴                                                             ─ 1. source
    ");
}

#[test]
fn event_trigger_name() {
    assert_snapshot!(goto("
create function f() returns event_trigger
  language plpgsql
  as 'begin end';
create event trigger e$0 on ddl_command_start execute function f();
"), @"
    info: goto definition
      ╭▸ file0.sql:5:22
      │
    5 │ create event trigger e on ddl_command_start execute function f();
      │                      ┬
      │                      │
      │                      1. source
      ╰╴                     2. destination
    ");
}

#[test]
fn drop_event_trigger_ref() {
    assert_snapshot!(goto("
create function f() returns event_trigger
  language plpgsql
  as 'begin end';
create event trigger e on ddl_command_start execute function f();
drop event trigger e$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:5:22
      │
    5 │ create event trigger e on ddl_command_start execute function f();
      │                      ─ 2. destination
    6 │ drop event trigger e;
      ╰╴                   ─ 1. source
    ");
}

#[test]
fn renamed_event_trigger() {
    assert_snapshot!(goto("
create function f() returns event_trigger
  language plpgsql
  as 'begin end';
create event trigger e on ddl_command_start execute function f();
alter event trigger e rename to e2;
alter event trigger e2$0 disable;
"), @"
    info: goto definition
      ╭▸ file0.sql:6:33
      │
    6 │ alter event trigger e rename to e2;
      │                                 ── 2. destination
    7 │ alter event trigger e2 disable;
      ╰╴                    ── 1. source
    ");
}
