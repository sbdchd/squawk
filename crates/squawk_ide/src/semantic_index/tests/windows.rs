use insta::assert_snapshot;

use super::check;

#[test]
fn window_clause() {
    assert_snapshot!(check("
create table t(a int, b int);
select count(*) over w from t window w as (partition by a order by b);
select count(*) over nope from t window w as ();
select count(*) over (nope2 order by a) from t;
select count(*) over w2 from t window w2 as (w2), w3 as (nope3);
select count(*) over w from t window w as (partition by nope4);
select (select count(*) over w from t) from t window w as ();
select count(*) over w from t window w as () order by sum(a) over w;
select * from (select count(*) over w from t window w as ()) s;
select * from t where false union select count(*) over w, 1 from t window w as ();
"), @"
    error: window `nope` does not exist
      ╭▸ 
    4 │ select count(*) over nope from t window w as ();
      ╰╴                     ━━━━
    error: window `nope2` does not exist
      ╭▸ 
    5 │ select count(*) over (nope2 order by a) from t;
      ╰╴                      ━━━━━
    error: window `w2` does not exist
      ╭▸ 
    6 │ select count(*) over w2 from t window w2 as (w2), w3 as (nope3);
      ╰╴                                             ━━
    error: window `nope3` does not exist
      ╭▸ 
    6 │ select count(*) over w2 from t window w2 as (w2), w3 as (nope3);
      ╰╴                                                         ━━━━━
    error: column `nope4` does not exist
      ╭▸ 
    7 │ select count(*) over w from t window w as (partition by nope4);
      ╰╴                                                        ━━━━━
    error: window `w` does not exist
      ╭▸ 
    8 │ select (select count(*) over w from t) from t window w as ();
      ╰╴                             ━
    ");
}
