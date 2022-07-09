use crate::{
    versions::Version,
    violations::{RuleViolation, RuleViolationKind},
};

use squawk_parser::ast::{ObjectType, RawStmt, Stmt};

#[must_use]
pub fn renaming_table(tree: &[RawStmt], _pg_version: Option<Version>) -> Vec<RuleViolation> {
    let mut errs = vec![];
    for raw_stmt in tree {
        match &raw_stmt.stmt {
            Stmt::RenameStmt(stmt) => match stmt.rename_type {
                ObjectType::Table => {
                    errs.push(RuleViolation::new(
                        RuleViolationKind::RenamingTable,
                        raw_stmt.into(),
                        None,
                    ));
                }
                _ => continue,
            },
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
    fn test_renaming_table() {
        let sql = r#"
ALTER TABLE "table_name" RENAME TO "new_table_name";
        "#;

        assert_debug_snapshot!(check_sql(sql, &[], None));
    }
}
