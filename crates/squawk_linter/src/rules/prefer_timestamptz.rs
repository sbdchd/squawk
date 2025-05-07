use squawk_syntax::{
    ast::{self, AstNode, HasArgList},
    Parse, SourceFile,
};

use crate::{text::trim_quotes, visitors::check_not_allowed_types};
use crate::{Linter, Rule, Violation};

pub fn is_not_allowed_timestamp(ty: &ast::Type) -> bool {
    match ty {
        ast::Type::ArrayType(array_type) => {
            if let Some(ty) = array_type.ty() {
                is_not_allowed_timestamp(&ty)
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
        ast::Type::CharType(_) => false,
        ast::Type::BitType(_) => false,
        ast::Type::DoubleType(_) => false,
        ast::Type::TimeType(time_type) => {
            if let Some(ty_name) = time_type.name_ref() {
                if ty_name.text() == "timestamp" && time_type.with_timezone().is_none() {
                    return true;
                }
            }
            false
        }
        ast::Type::IntervalType(_) => false,
    }
}

fn check_ty_for_timestamp(ctx: &mut Linter, ty: Option<ast::Type>) {
    if let Some(ty) = ty {
        if is_not_allowed_timestamp(&ty) {
            ctx.report(Violation::new(
                Rule::PreferTimestampTz,
            "When Postgres stores a datetime in a `timestamp` field, Postgres drops the UTC offset. This means 2019-10-11 21:11:24+02 and 2019-10-11 21:11:24-06 will both be stored as 2019-10-11 21:11:24 in the database, even though they are eight hours apart in time.".into(),
                ty.syntax().text_range(),
                "Use timestamptz instead of timestamp for your column type.".to_string(),
            ));
        };
    }
}

pub(crate) fn prefer_timestamptz(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    let file = parse.tree();
    check_not_allowed_types(ctx, &file, check_ty_for_timestamp);
}

#[cfg(test)]
mod test {
    use insta::assert_debug_snapshot;

    use crate::{Linter, Rule};

    #[test]
    fn create_table_with_timestamp_err() {
        let sql = r#"
create table app.users
(
    created_ts   timestamp
);
create table app.accounts
(
    created_ts timestamp without time zone
);
        "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::PreferTimestampTz]);
        let errors = linter.lint(file, sql);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors);
    }

    #[test]
    fn alter_table_with_timestamp_err() {
        let sql = r#"
alter table app.users
    alter column created_ts type timestamp;
alter table app.accounts
    alter column created_ts type timestamp without time zone;
        "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::PreferTimestampTz]);
        let errors = linter.lint(file, sql);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors);
    }

    #[test]
    fn alter_table_with_time_zone_ok() {
        let sql = r#"
create table app.users
(
    created_ts   timestamptz
);
create table app.accounts
(
    created_ts timestamp with time zone
);
        "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::PreferTimestampTz]);
        let errors = linter.lint(file, sql);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn create_table_with_time_zone_ok() {
        let sql = r#"
create table app.users
(
    created_ts   timestamptz
);
create table app.accounts
(
    created_ts timestamp with time zone
);
        "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::PreferTimestampTz]);
        let errors = linter.lint(file, sql);
        assert_eq!(errors.len(), 0);
    }
}
