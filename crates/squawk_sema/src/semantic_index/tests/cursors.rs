use insta::assert_snapshot;

use super::check;

#[test]
fn unresolved_cursor() {
    assert_snapshot!(check("
create table t (a int);
update t set a = 1 where current of c;
begin;
declare c cursor for select a from t;
declare d cursor with hold for select a from t;
savepoint sp;
declare e cursor for select a from t;
rollback to sp;
fetch c;
fetch e;
close c;
fetch c;
commit;
fetch d;
delete from t where current of c;
begin;
declare f cursor for select a from t;
close all;
fetch d;
fetch f;
"), @"
    error: cursor `c` does not exist
      ╭▸ 
    3 │ update t set a = 1 where current of c;
      ╰╴                                    ━
    error: cursor `e` does not exist
       ╭▸ 
    11 │ fetch e;
       ╰╴      ━
    error: cursor `c` does not exist
       ╭▸ 
    13 │ fetch c;
       ╰╴      ━
    error: cursor `c` does not exist
       ╭▸ 
    16 │ delete from t where current of c;
       ╰╴                               ━
    error: cursor `d` does not exist
       ╭▸ 
    20 │ fetch d;
       ╰╴      ━
    error: cursor `f` does not exist
       ╭▸ 
    21 │ fetch f;
       ╰╴      ━
    ");
}
