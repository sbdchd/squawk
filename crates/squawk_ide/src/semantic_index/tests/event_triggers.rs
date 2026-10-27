use insta::assert_snapshot;

use super::check;

#[test]
fn create_event_trigger() {
    assert_snapshot!(check("
create function f() returns event_trigger language plpgsql as 'begin end';
create event trigger e on ddl_command_start
  when tag in ('CREATE TABLE', 'DROP TABLE')
  execute function f();
create event trigger e2 on sql_drop execute procedure missing();
create event trigger e on ddl_command_end execute function f();
"), @"
    error: function `missing()` does not exist
      ╭▸ 
    6 │ create event trigger e2 on sql_drop execute procedure missing();
      ╰╴                                                      ━━━━━━━
    error: event trigger `e` already exists
      ╭▸ 
    7 │ create event trigger e on ddl_command_end execute function f();
      ╰╴                     ━
    ");
}

#[test]
fn drop_event_trigger() {
    assert_snapshot!(check("
create function f() returns event_trigger language plpgsql as 'begin end';
create event trigger e on ddl_command_start execute function f();
drop event trigger e;
drop event trigger e;
drop event trigger if exists e;
create event trigger e on ddl_command_start execute function f();
"), @"
    error: event trigger `e` does not exist
      ╭▸ 
    5 │ drop event trigger e;
      ╰╴                   ━
    ");
}

#[test]
fn event_trigger_rollback() {
    assert_snapshot!(check("
create function f() returns event_trigger language plpgsql as 'begin end';
create event trigger e on ddl_command_start execute function f();
begin;
drop event trigger e;
create event trigger e2 on ddl_command_start execute function f();
rollback;
alter event trigger e disable;
alter event trigger e2 disable;
"), @"
    error: event trigger `e2` does not exist
      ╭▸ 
    9 │ alter event trigger e2 disable;
      ╰╴                    ━━
    ");
}

#[test]
fn alter_event_trigger() {
    assert_snapshot!(check("
create role r;
create function f() returns event_trigger language plpgsql as 'begin end';
create event trigger e on ddl_command_start execute function f();
create event trigger e2 on ddl_command_start execute function f();
alter event trigger e disable;
alter event trigger e enable;
alter event trigger e enable replica;
alter event trigger e enable always;
alter event trigger e owner to r;
alter event trigger e owner to missing_role;
alter event trigger missing disable;
alter event trigger e rename to e2;
alter event trigger e rename to e3;
alter event trigger e disable;
drop event trigger e3;
"), @"
    error: role `missing_role` does not exist
       ╭▸ 
    11 │ alter event trigger e owner to missing_role;
       ╰╴                               ━━━━━━━━━━━━
    error: event trigger `missing` does not exist
       ╭▸ 
    12 │ alter event trigger missing disable;
       ╰╴                    ━━━━━━━
    error: event trigger `e` does not exist
       ╭▸ 
    15 │ alter event trigger e disable;
       ╰╴                    ━
    error: event trigger `e2` already exists
       ╭▸ 
    13 │ alter event trigger e rename to e2;
       ╰╴                                ━━
    ");
}
