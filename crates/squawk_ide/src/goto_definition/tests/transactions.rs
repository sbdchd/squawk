use insta::assert_snapshot;

use super::{goto, goto_files};

#[test]
fn begin_to_commit() {
    assert_snapshot!(goto("
begin$0;
select 1;
commit;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:1
      │
    2 │ begin;
      │ ───── 1. source
    3 │ select 1;
    4 │ commit;
      ╰╴─────── 2. destination
    ");
}

#[test]
fn begin_to_rollback() {
    assert_snapshot!(goto("
begin$0;
select 1;
rollback;
commit;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:1
      │
    2 │ begin;
      │ ───── 1. source
    3 │ select 1;
    4 │ rollback;
      ╰╴───────── 2. destination
    ");
}

#[test]
fn commit_to_begin() {
    assert_snapshot!(goto("
begin;
select 1;
commit$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:1
      │
    2 │ begin;
      │ ────── 2. destination
    3 │ select 1;
    4 │ commit;
      ╰╴────── 1. source
    ");
}

#[test]
fn rollback_to_begin() {
    assert_snapshot!(goto("
begin;
select 1;
rollback$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:1
      │
    2 │ begin;
      │ ────── 2. destination
    3 │ select 1;
    4 │ rollback;
      ╰╴──────── 1. source
    ");
}

#[test]
fn start_transaction_to_commit() {
    assert_snapshot!(goto("
start$0 transaction;
select 1;
commit;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:1
      │
    2 │ start transaction;
      │ ───── 1. source
    3 │ select 1;
    4 │ commit;
      ╰╴─────── 2. destination
    ");
}

#[test]
fn commit_to_start_transaction() {
    assert_snapshot!(goto("
start transaction;
select 1;
commit$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:1
      │
    2 │ start transaction;
      │ ────────────────── 2. destination
    3 │ select 1;
    4 │ commit;
      ╰╴────── 1. source
    ");
}

#[test]
fn end_to_begin() {
    assert_snapshot!(goto("
begin;
end$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:1
      │
    2 │ begin;
      │ ────── 2. destination
    3 │ end;
      ╰╴─── 1. source
    ");
}

#[test]
fn abort_to_begin() {
    assert_snapshot!(goto("
begin;
abort$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:1
      │
    2 │ begin;
      │ ────── 2. destination
    3 │ abort;
      ╰╴───── 1. source
    ");
}

#[test]
fn prepare_transaction_to_begin() {
    assert_snapshot!(goto("
begin;
prepare$0 transaction 'foo';
"), @"
    info: goto definition
      ╭▸ file0.sql:2:1
      │
    2 │ begin;
      │ ────── 2. destination
    3 │ prepare transaction 'foo';
      ╰╴─────── 1. source
    ");
}

#[test]
fn begin_without_end() {
    assert_snapshot!(goto("
begin$0;
select 1;
"), @"no definition");
}

#[test]
fn commit_without_begin() {
    assert_snapshot!(goto("
begin;
commit;
commit$0;
"), @"no definition");
}

#[test]
fn nested_begin_is_ignored() {
    assert_snapshot!(goto("
begin;
begin;
commit$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:1
      │
    2 │ begin;
      │ ────── 2. destination
    3 │ begin;
    4 │ commit;
      ╰╴────── 1. source
    ");
}

#[test]
fn commit_to_begin_in_sql_string_function_body() {
    assert_snapshot!(goto("
create function f() returns int
  language sql
  as $$
begin;
commit$0;
$$;
"), @"
    info: goto definition
      ╭▸ file0.sql:5:1
      │
    5 │ begin;
      │ ────── 2. destination
    6 │ commit;
      ╰╴────── 1. source
    ");
}

#[test]
fn rollback_to_savepoint() {
    assert_snapshot!(goto("
begin;
savepoint sp;
rollback to savepoint sp$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:11
      │
    3 │ savepoint sp;
      │           ── 2. destination
    4 │ rollback to savepoint sp;
      ╰╴                      ── 1. source
    ");
}

#[test]
fn rollback_to_savepoint_without_savepoint_keyword() {
    assert_snapshot!(goto("
begin;
savepoint sp;
rollback to sp$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:11
      │
    3 │ savepoint sp;
      │           ── 2. destination
    4 │ rollback to sp;
      ╰╴            ── 1. source
    ");
}

#[test]
fn rollback_to_most_recent_savepoint_with_same_name() {
    assert_snapshot!(goto("
begin;
savepoint sp;
savepoint sp;
rollback to sp$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:4:11
      │
    4 │ savepoint sp;
      │           ── 2. destination
    5 │ rollback to sp;
      ╰╴            ── 1. source
    ");
}

#[test]
fn rollback_keeps_target_savepoint() {
    assert_snapshot!(goto("
begin;
savepoint sp;
rollback to sp;
rollback to sp$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:11
      │
    3 │ savepoint sp;
      │           ── 2. destination
    4 │ rollback to sp;
    5 │ rollback to sp;
      ╰╴            ── 1. source
    ");
}

#[test]
fn rollback_discards_later_savepoints() {
    assert_snapshot!(goto("
begin;
savepoint keep;
savepoint discarded;
rollback to keep;
release savepoint discarded$0;
"), @"no definition");
}

#[test]
fn release_savepoint() {
    assert_snapshot!(goto("
begin;
savepoint sp;
release savepoint sp$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:11
      │
    3 │ savepoint sp;
      │           ── 2. destination
    4 │ release savepoint sp;
      ╰╴                  ── 1. source
    ");
}

#[test]
fn release_discards_savepoint() {
    assert_snapshot!(goto("
begin;
savepoint sp;
release sp;
rollback to sp$0;
"), @"no definition");
}

#[test]
fn release_falls_back_to_shadowed_savepoint() {
    assert_snapshot!(goto("
begin;
savepoint sp;
savepoint sp;
release sp;
rollback to sp$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:11
      │
    3 │ savepoint sp;
      │           ── 2. destination
      ‡
    6 │ rollback to sp;
      ╰╴            ── 1. source
    ");
}

#[test]
fn commit_discards_savepoints() {
    assert_snapshot!(goto("
begin;
savepoint sp;
commit;
release savepoint sp$0;
"), @"no definition");
}

#[test]
fn bare_rollback_discards_savepoints() {
    assert_snapshot!(goto("
begin;
savepoint sp;
rollback;
release savepoint sp$0;
"), @"no definition");
}

#[test]
fn prepare_transaction_discards_savepoints() {
    assert_snapshot!(goto("
begin;
savepoint sp;
prepare transaction 'foo';
release savepoint sp$0;
"), @"no definition");
}

#[test]
fn savepoint_quoted() {
    assert_snapshot!(goto(r#"
begin;
savepoint "Sp";
rollback to "Sp"$0;
"#), @r#"
    info: goto definition
      ╭▸ file0.sql:3:11
      │
    3 │ savepoint "Sp";
      │           ──── 2. destination
    4 │ rollback to "Sp";
      ╰╴            ──── 1. source
    "#);
}

#[test]
fn commit_prepared_to_prepare_transaction() {
    assert_snapshot!(goto("
prepare transaction 'foo';
select 1;
commit prepared 'foo'$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:21
      │
    2 │ prepare transaction 'foo';
      │                     ───── 2. destination
    3 │ select 1;
    4 │ commit prepared 'foo';
      ╰╴                ───── 1. source
    ");
}

#[test]
fn rollback_prepared_to_prepare_transaction() {
    assert_snapshot!(goto("
prepare transaction 'foo';
rollback prepared 'foo'$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:21
      │
    2 │ prepare transaction 'foo';
      │                     ───── 2. destination
    3 │ rollback prepared 'foo';
      ╰╴                  ───── 1. source
    ");
}

#[test]
fn commit_prepared_to_most_recent_prepare_transaction() {
    assert_snapshot!(goto("
prepare transaction 'foo';
commit prepared 'foo';
prepare transaction 'foo';
commit prepared 'foo'$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:4:21
      │
    4 │ prepare transaction 'foo';
      │                     ───── 2. destination
    5 │ commit prepared 'foo';
      ╰╴                ───── 1. source
    ");
}

#[test]
fn commit_prepared_before_prepare_transaction() {
    assert_snapshot!(goto("
commit prepared 'foo'$0;
prepare transaction 'foo';
"), @"no definition");
}

#[test]
fn commit_prepared_frees_transaction_id() {
    assert_snapshot!(goto("
prepare transaction 'foo';
commit prepared 'foo';
commit prepared 'foo'$0;
"), @"no definition");
}

#[test]
fn commit_prepared_matches_escaped_transaction_id() {
    assert_snapshot!(goto(r#"
prepare transaction e'fo\u006f';
commit prepared 'foo'$0;
"#), @r"
    info: goto definition
      ╭▸ file0.sql:2:21
      │
    2 │ prepare transaction e'fo\u006f';
      │                     ─────────── 2. destination
    3 │ commit prepared 'foo';
      ╰╴                ───── 1. source
    ");
}

#[test]
fn commit_prepared_matches_dollar_quoted_transaction_id() {
    assert_snapshot!(goto("
prepare transaction $$foo$$;
commit prepared 'foo'$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:21
      │
    2 │ prepare transaction $$foo$$;
      │                     ─────── 2. destination
    3 │ commit prepared 'foo';
      ╰╴                ───── 1. source
    ");
}

#[test]
fn commit_prepared_with_unknown_transaction_id() {
    assert_snapshot!(goto("
begin;
prepare transaction 'foo';
commit prepared 'bar'$0;
"), @"no definition");
}

#[test]
fn commit_prepared_across_files() {
    assert_snapshot!(goto_files(&["
prepare transaction 'foo';
", "
commit prepared 'foo'$0;
"]), @"
    info: goto definition
      ╭▸ file1.sql:2:17
      │
    2 │ commit prepared 'foo';
      │                 ───── 1. source
      │
      ⸬  file0.sql:2:21
      │
    2 │ prepare transaction 'foo';
      ╰╴                    ───── 2. destination
    ");
}
