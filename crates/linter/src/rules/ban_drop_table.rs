use crate::{
    versions::Version,
    violations::{RuleViolation, RuleViolationKind},
};

use squawk_parser::ast::{ObjectType, RawStmt, Stmt};

#[must_use]
pub fn ban_drop_table(
    tree: &[RawStmt],
    _pg_version: Option<Version>,
    _assume_in_transaction: bool,
) -> Vec<RuleViolation> {
    let mut errs = vec![];
    for raw_stmt in tree {
        match &raw_stmt.stmt {
            Stmt::DropStmt(stmt) if stmt.remove_type == ObjectType::Table => {
                errs.push(RuleViolation::new(
                    RuleViolationKind::BanDropTable,
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
        check_sql_with_rule(sql, &RuleViolationKind::BanDropTable, None, false).unwrap()
    }

    #[test]
    fn ban_drop_table() {
        let sql = r#"
DROP TABLE "table_name";
DROP TABLE IF EXISTS "table_name";
DROP TABLE IF EXISTS "table_name"
        "#;
        assert_debug_snapshot!(lint_sql(sql));
    }
}
