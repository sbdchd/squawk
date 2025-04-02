use crate::{
    versions::Version,
    violations::{RuleViolation, RuleViolationKind},
};

use squawk_parser::ast::{RawStmt, Stmt};

#[must_use]
pub fn ban_create_domain_with_constraint(
    tree: &[RawStmt],
    _pg_version: Option<Version>,
    _assume_in_transaction: bool,
) -> Vec<RuleViolation> {
    let mut errs = vec![];
    for raw_stmt in tree {
        match &raw_stmt.stmt {
            Stmt::CreateDomainStmt(stmt) if !stmt.constraints.is_empty() => {
                errs.push(RuleViolation::new(
                    RuleViolationKind::BanCreateDomainWithConstraint,
                    raw_stmt.into(),
                    None,
                ));
            }
            _ => continue,
        }
    }
    errs
}

#[cfg(test)]
mod test_rules {
    use crate::{
        check_sql_with_rule,
        violations::{RuleViolation, RuleViolationKind},
    };
    use insta::assert_debug_snapshot;

    fn lint_sql(sql: &str) -> Vec<RuleViolation> {
        check_sql_with_rule(
            sql,
            &RuleViolationKind::BanCreateDomainWithConstraint,
            None,
            false,
        )
        .unwrap()
    }

    #[test]
    fn ban_create_domain_without_constraint_is_ok() {
        let sql = r"
    CREATE DOMAIN domain_name_1 AS TEXT;
    CREATE DOMAIN domain_name_2 AS CHARACTER VARYING;
      ";
        assert_eq!(lint_sql(sql), vec![]);
    }

    #[test]
    fn ban_create_domain_with_constraint_works() {
        let sql = r"
    CREATE DOMAIN domain_name_3 AS NUMERIC(15,5) CHECK (value > 0);
      ";
        assert_debug_snapshot!(lint_sql(sql));
    }
}
