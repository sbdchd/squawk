use crate::{
    versions::Version,
    violations::{RuleViolation, RuleViolationKind},
};

use squawk_parser::ast::{RawStmt, Stmt};

#[must_use]
pub fn ban_alter_domain_with_add_constraint(
    tree: &[RawStmt],
    _pg_version: Option<Version>,
    _assume_in_transaction: bool,
) -> Vec<RuleViolation> {
    let mut errs = vec![];
    for raw_stmt in tree {
        match &raw_stmt.stmt {
            Stmt::AlterDomainStmt(stmt) if stmt.subtype == "C" => {
                errs.push(RuleViolation::new(
                    RuleViolationKind::BanAlterDomainWithAddConstraint,
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
            &RuleViolationKind::BanAlterDomainWithAddConstraint,
            None,
            false,
        )
        .unwrap()
    }

    #[test]
    fn ban_alter_domain_without_add_constraint_is_ok() {
        let sql = r"
     ALTER DOMAIN domain_name_1 SET DEFAULT 1;
     ALTER DOMAIN domain_name_2 SET NOT NULL;
     ALTER DOMAIN domain_name_3 DROP CONSTRAINT other_domain_name;
     ALTER DOMAIN domain_name_4 RENAME CONSTRAINT constraint_name TO other_constraint_name;
     ALTER DOMAIN domain_name_5 RENAME TO other_domain_name;
     ALTER DOMAIN domain_name_6 VALIDATE CONSTRAINT constraint_name;
     ALTER DOMAIN domain_name_7 OWNER TO you;
     ALTER DOMAIN domain_name_8 SET SCHEMA foo;
       ";
        assert_eq!(lint_sql(sql), vec![]);
    }

    #[test]
    fn ban_alter_domain_with_add_constraint_works() {
        let sql = r"
     ALTER DOMAIN domain_name ADD CONSTRAINT constraint_name CHECK (value > 0);
       ";
        assert_debug_snapshot!(lint_sql(sql));
    }
}
