use crate::{
    versions::Version,
    violations::{RuleViolation, RuleViolationKind},
};

use squawk_parser::ast::{RawStmt, Stmt, TableElt};

#[must_use]
pub fn ban_char_type(
    tree: &[RawStmt],
    _pg_version: Option<Version>,
    _assume_in_transaction: bool,
) -> Vec<RuleViolation> {
    let mut errs = vec![];
    for raw_stmt in tree {
        match &raw_stmt.stmt {
            Stmt::CreateStmt(stmt) => {
                for column_def in &stmt.table_elts {
                    if let TableElt::ColumnDef(column_def) = column_def {
                        let type_name = &column_def.type_name;
                        for field_type_name in &type_name.names {
                            if field_type_name.string.sval == "bpchar" {
                                errs.push(RuleViolation::new(
                                    RuleViolationKind::BanCharField,
                                    column_def.into(),
                                    None,
                                ));
                            }
                        }
                    }
                }
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
        check_sql_with_rule(sql, &RuleViolationKind::BanCharField, None, false).unwrap()
    }

    #[test]
    fn creating_table_with_char_errors() {
        let sql = r#"
BEGIN;
CREATE TABLE "core_bar" (
    "id" serial NOT NULL PRIMARY KEY,
    "alpha" char(100) NOT NULL,
    "beta" character(100) NOT NULL,
    "charlie" char NOT NULL,
    "delta" character NOT NULL
);
COMMIT;
        "#;
        assert_debug_snapshot!(lint_sql(sql));
    }

    #[test]
    fn creating_table_with_var_char_and_text_okay() {
        let sql = r#"
BEGIN;
CREATE TABLE "core_bar" (
    "id" serial NOT NULL PRIMARY KEY,
    "alpha" varchar(100) NOT NULL,
    "beta" text NOT NULL
);
COMMIT;
        "#;
        assert_debug_snapshot!(lint_sql(sql));
    }
}
