use insta::assert_snapshot;

use super::check;

#[test]
fn ordinal_out_of_range() {
    assert_snapshot!(check("
create table t(a int, b int);
select distinct on (3) a from t group by 0, cube (1, 4) order by 2, 5;
"), @"
    error: `distinct on` position 3 is not in select list
      ╭▸ 
    3 │ select distinct on (3) a from t group by 0, cube (1, 4) order by 2, 5;
      ╰╴                    ━
    error: `group by` position 0 is not in select list
      ╭▸ 
    3 │ select distinct on (3) a from t group by 0, cube (1, 4) order by 2, 5;
      ╰╴                                         ━
    error: `group by` position 4 is not in select list
      ╭▸ 
    3 │ select distinct on (3) a from t group by 0, cube (1, 4) order by 2, 5;
      ╰╴                                                     ━
    error: `order by` position 2 is not in select list
      ╭▸ 
    3 │ select distinct on (3) a from t group by 0, cube (1, 4) order by 2, 5;
      ╰╴                                                                 ━
    error: `order by` position 5 is not in select list
      ╭▸ 
    3 │ select distinct on (3) a from t group by 0, cube (1, 4) order by 2, 5;
      ╰╴                                                                    ━
    ");
}

#[test]
fn ordinal_after_unknown_star_is_not_reported() {
    assert_snapshot!(check("
select * from foo order by 5;
"), @"
    error: relation `foo` does not exist
      ╭▸ 
    2 │ select * from foo order by 5;
      ╰╴              ━━━
    ");
}
