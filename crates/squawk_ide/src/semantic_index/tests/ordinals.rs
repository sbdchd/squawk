use insta::assert_snapshot;

use super::check;

#[test]
fn ordinal_out_of_range() {
    assert_snapshot!(check("
create table t(a int, b int);
select distinct on (3) a from t group by 0, cube (1, 4) order by 2, 5;
"), @"
    error: DISTINCT ON position 3 is not in select list
      ╭▸ 
    3 │ select distinct on (3) a from t group by 0, cube (1, 4) order by 2, 5;
      ╰╴                    ━
    error: GROUP BY position 0 is not in select list
      ╭▸ 
    3 │ select distinct on (3) a from t group by 0, cube (1, 4) order by 2, 5;
      ╰╴                                         ━
    error: GROUP BY position 4 is not in select list
      ╭▸ 
    3 │ select distinct on (3) a from t group by 0, cube (1, 4) order by 2, 5;
      ╰╴                                                     ━
    error: ORDER BY position 2 is not in select list
      ╭▸ 
    3 │ select distinct on (3) a from t group by 0, cube (1, 4) order by 2, 5;
      ╰╴                                                                 ━
    error: ORDER BY position 5 is not in select list
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
