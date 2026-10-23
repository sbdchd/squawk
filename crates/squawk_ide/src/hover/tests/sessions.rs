use insta::assert_snapshot;

use super::hover;

#[test]
fn cursor() {
    assert_snapshot!(hover("
begin;
declare c cursor
for select 1;
fetch next from c$0;
"), @"
    hover: cursor c
      ╭▸ file0.sql:5:17
      │
    5 │ fetch next from c;
      ╰╴                ─ hover
    ");
}

#[test]
fn cursor_with_hold() {
    assert_snapshot!(hover("
declare c cursor with hold
for select 1;
close c$0;
"), @"
    hover: cursor c with hold
      ╭▸ file0.sql:4:7
      │
    4 │ close c;
      ╰╴      ─ hover
    ");
}

#[test]
fn savepoint() {
    assert_snapshot!(hover("
begin;
savepoint s;
rollback to savepoint s$0;
"), @"
    hover: savepoint s
      ╭▸ file0.sql:4:23
      │
    4 │ rollback to savepoint s;
      ╰╴                      ─ hover
    ");
}

#[test]
fn prepared_statement() {
    assert_snapshot!(hover("
prepare p as
  select 1 a, 'b' b;
execute p$0;
"), @"
    hover: prepare p(a int4, b text)
      ╭▸ file0.sql:4:9
      │
    4 │ execute p;
      ╰╴        ─ hover
    ");
}

#[test]
fn prepared_transaction() {
    assert_snapshot!(hover("
prepare transaction 'tx';
commit prepared 'tx'$0;
"), @"
    hover: prepared transaction 'tx'
      ╭▸ file0.sql:3:17
      │
    3 │ commit prepared 'tx';
      ╰╴                ──── hover
    ");
}

#[test]
fn channel() {
    assert_snapshot!(hover("
listen c;
notify c$0;
"), @"
    hover: channel c
      ╭▸ file0.sql:3:8
      │
    3 │ notify c;
      ╰╴       ─ hover
    ");
}

#[test]
fn transaction_begin() {
    assert_snapshot!(hover("
begin$0;
commit;
"), @"no hover");
}
