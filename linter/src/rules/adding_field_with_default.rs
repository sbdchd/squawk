use std::collections::HashSet;

use crate::{
    versions::Version,
    violations::{RuleViolation, RuleViolationKind, ViolationMessage},
};

use serde_json::{json, Value};
use squawk_parser::ast::{
    AlterTableCmds, AlterTableDef, ColumnDefConstraint, ConstrType, RawStmt, Stmt,
};

fn constraint_has_constant_expr(raw_expr: &Value) -> bool {
    raw_expr["A_Const"] != Value::Null || raw_expr["TypeCast"]["arg"]["A_Const"] != Value::Null
}

fn is_non_volatile_func_call(raw_expr: &Value, non_volatile_funcs: &HashSet<String>) -> bool {
    let func_name = raw_expr["FuncCall"]["funcname"][0]["String"]["sval"].as_str();

    let Some(func_name) = func_name else {
        return false;
    };

    // NOTE(chdsbd): we don't check functions with args, because I'm not certain
    // if there's a problem with volatile functions there. If we need this
    // functionality, we can add it later.
    raw_expr["FuncCall"]["args"] == Value::Null && non_volatile_funcs.contains(func_name)
}

// Generated via the following Postgres query:
//      select proname from pg_proc where provolatile <> 'v';
const NON_VOLATILE_BUILT_IN_FUNCTIONS: &str = include_str!("non_volatile_built_in_functions.txt");

#[must_use]
pub fn adding_field_with_default(
    tree: &[RawStmt],
    pg_version: Option<Version>,
    _assume_in_transaction: bool,
) -> Vec<RuleViolation> {
    let mut errs = vec![];

    let non_volatile_funcs: HashSet<_> = NON_VOLATILE_BUILT_IN_FUNCTIONS
        .split('\n')
        .map(|x| x.trim().to_lowercase())
        .filter(|x| !x.is_empty())
        .collect();

    for raw_stmt in tree {
        match &raw_stmt.stmt {
            Stmt::AlterTableStmt(stmt) => {
                for AlterTableCmds::AlterTableCmd(cmd) in &stmt.cmds {
                    match &cmd.def {
                        Some(AlterTableDef::ColumnDef(def)) => {
                            for ColumnDefConstraint::Constraint(constraint) in &def.constraints {
                                if constraint.contype == ConstrType::Default {
                                    if let Some(pg_version) = pg_version {
                                        let def = json!({});
                                        let raw_expr = constraint.raw_expr.as_ref().unwrap_or(&def);
                                        if pg_version > Version::new(11, None, None)
                                            && (constraint_has_constant_expr(raw_expr)
                                                || is_non_volatile_func_call(
                                                    raw_expr,
                                                    &non_volatile_funcs,
                                                ))
                                        {
                                            continue;
                                        }
                                    }
                                    errs.push(RuleViolation::new(
                                        RuleViolationKind::AddingFieldWithDefault,
                                        raw_stmt.into(),
                                        None,
                                    ));
                                }
                                if constraint.contype == ConstrType::Generated {
                                    errs.push(RuleViolation::new(
                                        RuleViolationKind::AddingFieldWithDefault,
                                        raw_stmt.into(),
                                        Some(vec![
                                            ViolationMessage::Note(
                                            "Adding a generated column requires a table rewrite with an ACCESS EXCLUSIVE lock.".into(),
                                        ),
                                        ViolationMessage::Help(
                                            "Add the column as nullable, backfill existing rows, and add a trigger to update the column on write instead.".into(),
                                        ),
                                        ]),
                                    ));
                                }
                            }
                        }
                        _ => continue,
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
    use std::str::FromStr;

    use crate::{
        check_sql_with_rule,
        versions::Version,
        violations::{RuleViolation, RuleViolationKind},
    };

    use insta::assert_debug_snapshot;

    fn lint_sql(sql: &str, pg_version: Option<Version>) -> Vec<RuleViolation> {
        check_sql_with_rule(
            sql,
            &RuleViolationKind::AddingFieldWithDefault,
            pg_version,
            false,
        )
        .unwrap()
    }

    ///
    /// ```sql
    /// -- instead of
    /// ALTER TABLE "core_recipe" ADD COLUMN "foo" integer DEFAULT 10;
    /// -- use
    /// ALTER TABLE "core_recipe" ADD COLUMN "foo" integer;
    /// ALTER TABLE "core_recipe" ALTER COLUMN "foo" SET DEFAULT 10;
    /// -- backfill
    /// -- remove nullability
    /// ```
    #[test]
    fn docs_example_bad() {
        let bad_sql = r#"
-- instead of
ALTER TABLE "core_recipe" ADD COLUMN "foo" integer DEFAULT 10;
"#;
        assert_debug_snapshot!(lint_sql(bad_sql, None));
    }
    #[test]
    fn docs_example_ok() {
        let ok_sql = r#"
-- use
ALTER TABLE "core_recipe" ADD COLUMN "foo" integer;
ALTER TABLE "core_recipe" ALTER COLUMN "foo" SET DEFAULT 10;
-- backfill
-- remove nullability
        "#;
        assert_debug_snapshot!(lint_sql(ok_sql, None));
    }

    #[test]
    fn default_integer_ok() {
        let ok_sql = r#"
-- NON-VOLATILE
ALTER TABLE "core_recipe" ADD COLUMN "foo" integer DEFAULT 10;
"#;

        let pg_version_11 = Some(Version::from_str("11.0.0").unwrap());
        assert_debug_snapshot!(lint_sql(ok_sql, pg_version_11));
    }

    #[test]
    fn default_uuid_err() {
        let bad_sql = r#"
-- VOLATILE
ALTER TABLE "core_recipe" ADD COLUMN "foo" integer DEFAULT uuid();
"#;

        let pg_version_11 = Some(Version::from_str("11.0.0").unwrap());
        assert_debug_snapshot!(lint_sql(bad_sql, pg_version_11));
    }

    #[test]
    fn default_volatile_func_err() {
        let bad_sql = r#"
-- VOLATILE
ALTER TABLE "core_recipe" ADD COLUMN "foo" boolean DEFAULT random();
"#;
        let pg_version_11 = Some(Version::from_str("11.0.0").unwrap());
        assert_debug_snapshot!(lint_sql(bad_sql, pg_version_11));
    }
    #[test]
    fn default_bool_ok() {
        let ok_sql = r#"
-- NON-VOLATILE
ALTER TABLE "core_recipe" ADD COLUMN "foo" boolean DEFAULT true;
"#;
        let pg_version_11 = Some(Version::from_str("11.0.0").unwrap());
        assert_debug_snapshot!(lint_sql(ok_sql, pg_version_11));
    }
    #[test]
    fn default_str_ok() {
        let ok_sql = r#"
-- NON-VOLATILE
ALTER TABLE "core_recipe" ADD COLUMN "foo" text DEFAULT 'some-str';
"#;
        let pg_version_11 = Some(Version::from_str("11.0.0").unwrap());
        assert_debug_snapshot!(lint_sql(ok_sql, pg_version_11));
    }
    #[test]
    fn default_enum_ok() {
        let ok_sql = r#"
-- NON-VOLATILE
ALTER TABLE "core_recipe" ADD COLUMN "foo" some_enum_type DEFAULT 'my-enum-variant';
"#;
        let pg_version_11 = Some(Version::from_str("11.0.0").unwrap());
        assert_debug_snapshot!(lint_sql(ok_sql, pg_version_11));
    }
    #[test]
    fn default_jsonb_ok() {
        let ok_sql = r#"
-- NON-VOLATILE
ALTER TABLE "core_recipe" ADD COLUMN "foo" jsonb DEFAULT '{}'::jsonb;
"#;
        let pg_version_11 = Some(Version::from_str("11.0.0").unwrap());
        assert_debug_snapshot!(lint_sql(ok_sql, pg_version_11));
    }
    #[test]
    fn default_arbitrary_func_err() {
        let ok_sql = r#"
-- NON-VOLATILE
ALTER TABLE "core_recipe" ADD COLUMN "foo" jsonb DEFAULT myjsonb();
"#;
        let pg_version_11 = Some(Version::from_str("11.0.0").unwrap());
        assert_debug_snapshot!(lint_sql(ok_sql, pg_version_11));
    }
    #[test]
    fn default_random_with_args_err() {
        let ok_sql = r#"
-- NON-VOLATILE
ALTER TABLE "core_recipe" ADD COLUMN "foo" timestamptz DEFAULT now(123);
"#;
        let pg_version_11 = Some(Version::from_str("11.0.0").unwrap());
        assert_debug_snapshot!(lint_sql(ok_sql, pg_version_11));
    }
    #[test]
    fn default_now_func_ok() {
        let ok_sql = r#"
-- NON-VOLATILE
ALTER TABLE "core_recipe" ADD COLUMN "foo" timestamptz DEFAULT now();
"#;
        let pg_version_11 = Some(Version::from_str("11.0.0").unwrap());
        assert_debug_snapshot!(lint_sql(ok_sql, pg_version_11));
    }
    #[test]
    fn add_numbers_ok() {
        // This should be okay, but we don't handle expressions like this at the moment.
        let ok_sql = r"
alter table account_metadata add column blah integer default 2 + 2;
";
        let pg_version_11 = Some(Version::from_str("11.0.0").unwrap());
        assert_debug_snapshot!(lint_sql(ok_sql, pg_version_11));
    }

    #[test]
    fn generated_stored() {
        let bad_sql = r"
        ALTER TABLE foo
    ADD COLUMN bar numeric GENERATED ALWAYS AS (bar + baz) STORED;
        ";
        let pg_version_11 = Some(Version::from_str("11.0.0").unwrap());
        assert_debug_snapshot!(lint_sql(bad_sql, pg_version_11));
    }
}
