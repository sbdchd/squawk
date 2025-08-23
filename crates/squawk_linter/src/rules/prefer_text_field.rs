use std::collections::HashSet;

use squawk_syntax::{
    Parse, SourceFile,
    ast::{self, AstNode},
};

use crate::{Edit, Fix, Linter, Rule, Violation, identifier::Identifier};

use crate::visitors::check_not_allowed_types;

use lazy_static::lazy_static;

lazy_static! {
    static ref VARCHAR_TYPE_NAMES: HashSet<&'static str> = HashSet::from(["varchar"]);
}

fn is_not_allowed_varchar(ty: &ast::Type) -> bool {
    match ty {
        ast::Type::ArrayType(array_type) => {
            if let Some(ty) = array_type.ty() {
                is_not_allowed_varchar(&ty)
            } else {
                false
            }
        }
        ast::Type::PercentType(_) => false,
        ast::Type::PathType(path_type) => {
            let Some(ty_name) = path_type
                .path()
                .and_then(|x| x.segment())
                .and_then(|x| x.name_ref())
                .map(|x| x.text().to_string())
            else {
                return false;
            };
            // if we don't have any args, then it's the same as `text`
            Identifier::new(ty_name.as_str()) == Identifier::new("varchar")
                && path_type.arg_list().is_some()
        }
        ast::Type::CharType(char_type) => {
            Identifier::new(&char_type.text()) == Identifier::new("varchar")
                && char_type.arg_list().is_some()
        }
        ast::Type::BitType(_) => false,
        ast::Type::DoubleType(_) => false,
        ast::Type::TimeType(_) => false,
        ast::Type::IntervalType(_) => false,
    }
}

fn create_varchar_to_text_fix(ty: &ast::Type) -> Option<Fix> {
    let range = match ty {
        ast::Type::PathType(path_type) => {
            // we'll replace the entire path type, including args
            // so: `"varchar"(100)` becomes `text`
            path_type.syntax().text_range()
        }
        ast::Type::CharType(char_type) => {
            // we'll replace the entire char type, including args
            // so: `varchar(100)` becomes `text`
            char_type.syntax().text_range()
        }
        ast::Type::ArrayType(array_type) => {
            let ty = array_type.ty()?;
            ty.syntax().text_range()
        }
        _ => return None,
    };
    let edit = Edit::replace(range, "text");
    Some(Fix::new("Replace with `text`", vec![edit]))
}

fn check_ty_for_varchar(ctx: &mut Linter, ty: Option<ast::Type>) {
    if let Some(ty) = ty {
        if is_not_allowed_varchar(&ty) {
            let fix = create_varchar_to_text_fix(&ty);
            ctx.report(Violation::for_node(
                Rule::PreferTextField,
               "Changing the size of a `varchar` field requires an `ACCESS EXCLUSIVE` lock, that will prevent all reads and writes to the table.".to_string(),
                ty.syntax(),
            ).help("Use a `TEXT` field with a `CHECK` constraint.").fix(fix));
        };
    }
}

pub(crate) fn prefer_text_field(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    let file = parse.tree();
    check_not_allowed_types(ctx, &file, check_ty_for_varchar);
}

#[cfg(test)]
mod test {
    use insta::{assert_debug_snapshot, assert_snapshot};

    use crate::{
        Rule,
        test_utils::{fix_sql, lint},
    };

    fn fix(sql: &str) -> String {
        fix_sql(sql, Rule::PreferTextField)
    }

    /// Changing a column of varchar(255) to varchar(1000) requires an ACCESS
    /// EXCLUSIVE lock
    #[test]
    fn increase_varchar_size_err() {
        let sql = r#"
BEGIN;
--
-- Alter field kind on foo
--
ALTER TABLE "core_foo" ALTER COLUMN "kind" TYPE varchar(1000) USING "kind"::varchar(1000);
COMMIT;
        "#;
        let errors = lint(sql, Rule::PreferTextField);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors);
    }

    #[test]
    fn create_table_with_varchar_err() {
        let sql = r#"
BEGIN;
--
-- Create model Bar
--
CREATE TABLE "core_bar" (
    "id" serial NOT NULL PRIMARY KEY, 
    "alpha" varchar(100) NOT NULL
);
COMMIT;
        "#;
        let errors = lint(sql, Rule::PreferTextField);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors);
    }

    #[test]
    fn create_table_with_pgcatalog_varchar_err() {
        let sql = r#"
create table t (
    "id" serial NOT NULL PRIMARY KEY, 
    "alpha" pg_catalog.varchar(100) NOT NULL
);
        "#;
        let errors = lint(sql, Rule::PreferTextField);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors);
    }

    #[test]
    fn adding_column_non_text_err() {
        let sql = r#"
BEGIN;
ALTER TABLE "foo_table" ADD COLUMN "foo_column" varchar(256) NULL;
COMMIT;
        "#;
        let errors = lint(sql, Rule::PreferTextField);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors);
    }

    #[test]
    fn varchar_without_specified_limit_ok() {
        let sql = r#"
CREATE TABLE IF NOT EXISTS foo_table(bar_col varchar);
        "#;
        let errors = lint(sql, Rule::PreferTextField);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn create_table_with_text_ok() {
        let sql = r#"
BEGIN;
--
-- Create model Bar
--
CREATE TABLE "core_bar" (
    "id" serial NOT NULL PRIMARY KEY, 
    "bravo" text NOT NULL
);
--
-- Create constraint text_size on model bar
--
ALTER TABLE "core_bar" ADD CONSTRAINT "text_size" CHECK (LENGTH("bravo") <= 100);
COMMIT;
        "#;
        let errors = lint(sql, Rule::PreferTextField);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn fix_varchar_with_length() {
        assert_snapshot!(fix("CREATE TABLE t (c varchar(100));"), @"CREATE TABLE t (c text);");
        assert_snapshot!(fix("CREATE TABLE t (c varchar(255));"), @"CREATE TABLE t (c text);");
        assert_snapshot!(fix("CREATE TABLE t (c varchar(50));"), @"CREATE TABLE t (c text);");
    }

    #[test]
    fn fix_mixed_case_varchar() {
        assert_snapshot!(fix("CREATE TABLE t (c VARCHAR(100));"), @"CREATE TABLE t (c text);");
        assert_snapshot!(fix("CREATE TABLE t (c Varchar(50));"), @"CREATE TABLE t (c text);");
        assert_snapshot!(fix("CREATE TABLE t (c VarChar(255));"), @"CREATE TABLE t (c text);");
    }

    #[test]
    fn fix_varchar_arrays() {
        assert_snapshot!(fix("CREATE TABLE t (c varchar(100)[]);"), @"CREATE TABLE t (c text[]);");
        assert_snapshot!(fix("CREATE TABLE t (c varchar(255)[5]);"), @"CREATE TABLE t (c text[5]);");
        assert_snapshot!(fix("CREATE TABLE t (c varchar(50)[3][4]);"), @"CREATE TABLE t (c text[3][4]);");
    }

    #[test]
    fn fix_alter_table() {
        assert_snapshot!(fix("ALTER TABLE t ADD COLUMN c varchar(100);"), @"ALTER TABLE t ADD COLUMN c text;");
        assert_snapshot!(fix("ALTER TABLE t ALTER COLUMN c TYPE varchar(256);"), @"ALTER TABLE t ALTER COLUMN c TYPE text;");
    }

    #[test]
    fn fix_multiple_varchar_columns() {
        assert_snapshot!(fix("CREATE TABLE t (a varchar(100), b varchar(255), c varchar(50));"), @"CREATE TABLE t (a text, b text, c text);");
    }

    #[test]
    fn fix_pgcatalog_varchar() {
        assert_snapshot!(fix("CREATE TABLE t (c pg_catalog.varchar(100));"), @"CREATE TABLE t (c text);");
    }
}
