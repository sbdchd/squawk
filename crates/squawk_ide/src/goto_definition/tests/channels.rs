use insta::assert_snapshot;

use super::goto;

#[test]
fn notify_channel_relisten_goes_to_first_listen() {
    assert_snapshot!(goto("
listen updates;
listen updates;
notify updates$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:8
      │
    2 │ listen updates;
      │        ─────── 2. destination
    3 │ listen updates;
    4 │ notify updates;
      ╰╴       ─────── 1. source
    ");
}

#[test]
fn notify_channel_after_unlisten() {
    assert_snapshot!(goto("
listen updates;
unlisten updates;
notify updates$0;
"), @"no definition");
}

#[test]
fn notify_channel_after_unlisten_all() {
    assert_snapshot!(goto("
listen updates;
unlisten *;
notify updates$0;
"), @"no definition");
}

#[test]
fn rule_notify_channel() {
    assert_snapshot!(goto("
create table t (a int);
listen updates;
create rule r as on insert
  to t
  do also notify updates$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:8
      │
    3 │ listen updates;
      │        ─────── 2. destination
      ‡
    6 │   do also notify updates;
      ╰╴                 ─────── 1. source
    ");
}
