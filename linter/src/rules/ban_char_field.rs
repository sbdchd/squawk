use crate::violations::{RuleViolation, RuleViolationKind};
use squawk_parser::ast::{ColumnDefTypeName, QualifiedName, RootStmt, Stmt, TableElt};

#[must_use]
pub fn ban_char_type(tree: &[RootStmt]) -> Vec<RuleViolation> {
    let mut errs = vec![];
    for RootStmt::RawStmt(raw_stmt) in tree {
        match &raw_stmt.stmt {
            Stmt::CreateStmt(stmt) => {
                for column_def in &stmt.table_elts {
                    if let TableElt::ColumnDef(column_def) = column_def {
                        let ColumnDefTypeName::TypeName(type_name) = &column_def.type_name;
                        for QualifiedName::String(field_type_name) in &type_name.names {
                            if field_type_name.str == "bpchar" {
                                errs.push(RuleViolation::new(
                                    RuleViolationKind::BanCharField,
                                    raw_stmt.into(),
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
    use crate::check_sql;
    use insta::assert_debug_snapshot;
    #[test]
    fn test_creating_table_with_char_errors() {
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
        assert_debug_snapshot!(check_sql(sql, &[]));
    }

    #[test]
    fn test_creating_table_with_var_char_and_text_okay() {
        let sql = r#"
BEGIN;
CREATE TABLE "core_bar" (
    "id" serial NOT NULL PRIMARY KEY,
    "alpha" varchar(100) NOT NULL,
    "beta" text NOT NULL
);
COMMIT;
        "#;
        assert_debug_snapshot!(check_sql(sql, &["prefer-text-field".into()]));
    }
}
