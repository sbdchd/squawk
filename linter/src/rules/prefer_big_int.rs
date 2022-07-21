use crate::{
    versions::Version,
    violations::RuleViolation,
};

use squawk_parser::ast::RawStmt;


#[must_use]
pub fn prefer_big_int(tree: &[RawStmt], _pg_version: Option<Version>) -> Vec<RuleViolation> {
    let mut errs = vec![];
    errs
}

#[cfg(test)]
mod test_rules {
    use crate::check_sql;

    #[test]
    fn test_prefer_big_int() {
        let ok_sql = r#"
SELECT 1;
  "#;
        assert_eq!(check_sql(ok_sql, &[], None), Ok(vec![]));
    }
}
