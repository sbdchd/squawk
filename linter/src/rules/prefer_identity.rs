use crate::{
    versions::Version,
    violations::RuleViolation,
};

use squawk_parser::ast::RawStmt;


#[must_use]
pub fn prefer_identity(tree: &[RawStmt], _pg_version: Option<Version>) -> Vec<RuleViolation> {
    let mut errs = vec![];
    errs
}

#[cfg(test)]
mod test_rules {
    use crate::{
         check_sql_with_rule,
         violations::{RuleViolation, RuleViolationKind},
     };
     fn lint_sql(sql: &str) -> Vec<RuleViolation> {
         check_sql_with_rule(sql, &RuleViolationKind::PreferIdentity, None).unwrap()
     }

    #[test]
    fn test_prefer_identity() {
        let ok_sql = r#"
SELECT 1;
  "#;
        assert_eq!(lint_sql(ok_sql), vec![]);
    }
}
