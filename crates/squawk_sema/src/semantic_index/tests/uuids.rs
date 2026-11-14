use insta::assert_snapshot;

use super::check;

#[test]
fn invalid_uuid_cast() {
    assert_snapshot!(check("
select
  'not-a-uuid'::uuid,
  ''::uuid,
  'a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a1'::uuid,
  ' a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11'::uuid;
"), @"
    error: invalid input value for type `uuid`: `not-a-uuid`
      ╭▸ 
    3 │   'not-a-uuid'::uuid,
      ╰╴  ━━━━━━━━━━━━
    error: invalid input value for type `uuid`: ``
      ╭▸ 
    4 │   ''::uuid,
      ╰╴  ━━
    error: invalid input value for type `uuid`: `a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a1`
      ╭▸ 
    5 │   'a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a1'::uuid,
      ╰╴  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    error: invalid input value for type `uuid`: ` a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11`
      ╭▸ 
    6 │   ' a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11'::uuid;
      ╰╴  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    ");
}

#[test]
fn valid_uuids() {
    assert_snapshot!(check("
select
  'a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11'::uuid,
  '{A0EEBC99-9C0B-4EF8-BB6D-6BB9BD380A11}'::uuid,
  'a0eebc999c0b4ef8bb6d6bb9bd380a11'::uuid,
  'a0ee-bc99-9c0b-4ef8-bb6d-6bb9-bd38-0a11'::uuid;
"), @"no diagnostics");
}

#[test]
fn uuid_column_literals() {
    assert_snapshot!(check("
create domain d as uuid;
create table t (
  a uuid,
  b d
);
insert into t values ('x', 'a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11');
select *
from t
where a = 'y' or b in ('a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', 'z');
"), @"
    error: invalid input value for type `uuid`: `x`
      ╭▸ 
    7 │ insert into t values ('x', 'a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11');
      ╰╴                      ━━━
    error: invalid input value for type `uuid`: `y`
       ╭▸ 
    10 │ where a = 'y' or b in ('a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', 'z');
       ╰╴          ━━━
    error: invalid input value for type `uuid`: `z`
       ╭▸ 
    10 │ where a = 'y' or b in ('a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', 'z');
       ╰╴                                                               ━━━
    ");
}
