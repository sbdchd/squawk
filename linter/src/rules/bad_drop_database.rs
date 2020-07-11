use crate::violations::{RuleViolation, RuleViolationKind};
use squawk_parser::ast::{RootStmt, Stmt};

/// Brad's Rule aka ban dropping database statements.
pub fn ban_drop_database(tree: &[RootStmt]) -> Vec<RuleViolation> {
    let mut errs = vec![];
    for RootStmt::RawStmt(raw_stmt) in tree {
        match &raw_stmt.stmt {
            Stmt::DropdbStmt(_) => {
                errs.push(RuleViolation::new(
                    RuleViolationKind::BanDropDatabase,
                    raw_stmt,
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
    use crate::check_sql;
    use insta::assert_debug_snapshot;
    #[test]
    fn test_ban_drop_database() {
        let sql = r#"
DROP DATABASE "table_name";
DROP DATABASE IF EXISTS "table_name";
DROP DATABASE IF EXISTS "table_name"
        "#;
        assert_debug_snapshot!(check_sql(sql, &[]));
    }
}
