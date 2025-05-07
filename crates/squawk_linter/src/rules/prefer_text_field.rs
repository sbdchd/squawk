use std::collections::HashSet;

use squawk_syntax::{
    ast::{self, AstNode, HasArgList},
    Parse, SourceFile,
};

use crate::{text::trim_quotes, Linter, Rule, Violation};

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
            trim_quotes(ty_name.as_str()) == "varchar" && path_type.arg_list().is_some()
        }
        ast::Type::CharType(char_type) => {
            trim_quotes(&char_type.text()) == "varchar" && char_type.arg_list().is_some()
        }
        ast::Type::BitType(_) => false,
        ast::Type::DoubleType(_) => false,
        ast::Type::TimeType(_) => false,
        ast::Type::IntervalType(_) => false,
    }
}

fn check_ty_for_varchar(ctx: &mut Linter, ty: Option<ast::Type>) {
    if let Some(ty) = ty {
        if is_not_allowed_varchar(&ty) {
            ctx.report(Violation::new(
                Rule::PreferTextField,
               "Changing the size of a `varchar` field requires an `ACCESS EXCLUSIVE` lock, that will prevent all reads and writes to the table.".to_string(),
                ty.syntax().text_range(),
                "Use a `TEXT` field with a `CHECK` constraint.".to_string(),
            ));
        };
    }
}

pub(crate) fn prefer_text_field(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    let file = parse.tree();
    check_not_allowed_types(ctx, &file, check_ty_for_varchar);
}

#[cfg(test)]
mod test {
    use insta::assert_debug_snapshot;

    use crate::{Linter, Rule};

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
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::PreferTextField]);
        let errors = linter.lint(file, sql);
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
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::PreferTextField]);
        let errors = linter.lint(file, sql);
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
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::PreferTextField]);
        let errors = linter.lint(file, sql);
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
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::PreferTextField]);
        let errors = linter.lint(file, sql);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors);
    }

    #[test]
    fn varchar_without_specified_limit_ok() {
        let sql = r#"
CREATE TABLE IF NOT EXISTS foo_table(bar_col varchar);
        "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::PreferTextField]);
        let errors = linter.lint(file, sql);
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
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::PreferTextField]);
        let errors = linter.lint(file, sql);
        assert_eq!(errors.len(), 0);
    }
}
