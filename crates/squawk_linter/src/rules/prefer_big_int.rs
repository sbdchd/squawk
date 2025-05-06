use std::collections::HashSet;

use squawk_syntax::{
    ast::{self, AstNode, HasModuleItem},
    Parse, SourceFile,
};

use crate::{text::trim_quotes, Linter, Rule, Violation};
use lazy_static::lazy_static;

lazy_static! {
    static ref SMALL_INT_TYPES: HashSet<&'static str> = HashSet::from([
        "smallint",
        "integer",
        "int2",
        "int4",
        "serial",
        "serial2",
        "serial4",
        "smallserial",
    ]);
}

pub(crate) fn is_not_valid_int_type(ty: &ast::Type, invalid_type_names: &HashSet<&str>) -> bool {
    match ty {
        ast::Type::ArrayType(array_type) => {
            if let Some(ty) = array_type.ty() {
                is_not_valid_int_type(&ty, invalid_type_names)
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
            let name = trim_quotes(ty_name.as_str());
            invalid_type_names.contains(name)
        }
        ast::Type::CharType(_) => false,
        ast::Type::BitType(_) => false,
        ast::Type::DoubleType(_) => false,
        ast::Type::TimeType(_) => false,
        ast::Type::IntervalType(_) => false,
    }
}

pub(crate) fn check_not_allowed_types(
    ctx: &mut Linter,
    file: &ast::SourceFile,
    check_ty: impl Fn(&mut Linter, Option<ast::Type>),
) {
    for item in file.items() {
        match item {
            ast::Item::CreateTable(create_table) => {
                if let Some(table_args) = create_table.table_args() {
                    for arg in table_args.args() {
                        if let ast::TableArg::Column(column) = arg {
                            check_ty(ctx, column.ty());
                        }
                    }
                }
            }
            ast::Item::AlterTable(alter_table) => {
                for action in alter_table.actions() {
                    match action {
                        ast::AlterTableAction::AddColumn(add_column) => {
                            check_ty(ctx, add_column.ty());
                        }
                        ast::AlterTableAction::AlterColumn(alter_column) => {
                            if let Some(ast::AlterColumnOption::SetType(set_type)) =
                                alter_column.option()
                            {
                                check_ty(ctx, set_type.ty());
                            }
                        }
                        _ => (),
                    }
                }
            }
            _ => (),
        }
    }
}

fn check_ty_for_big_int(ctx: &mut Linter, ty: Option<ast::Type>) {
    if let Some(ty) = ty {
        if is_not_valid_int_type(&ty, &SMALL_INT_TYPES) {
            ctx.report(Violation::new(
                Rule::PreferBigInt,
                "Using 32-bit integer fields can result in hitting the max `int` limit.".into(),
                ty.syntax().text_range(),
                "Use 64-bit integer values instead to prevent hitting this limit.".to_string(),
            ));
        };
    }
}

pub(crate) fn prefer_big_int(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    let file = parse.tree();
    check_not_allowed_types(ctx, &file, check_ty_for_big_int);
}

#[cfg(test)]
mod test {
    use insta::assert_debug_snapshot;

    use crate::{Linter, Rule};

    #[test]
    fn err() {
        let sql = r#"
create table users (
    id smallint
);
create table users (
    id int2
);
create table users (
    id integer
);
create table users (
    id int4
);
create table users (
    id serial
);
create table users (
    id serial2
);
create table users (
    id serial4
);
create table users (
    id smallserial
);
        "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::PreferBigInt]);
        let errors = linter.lint(file, sql);
        assert_ne!(errors.len(), 0);
        assert_eq!(errors.len(), 8);
        assert_eq!(
            errors
                .iter()
                .filter(|x| x.code == Rule::PreferBigInt)
                .count(),
            8
        );
        assert_debug_snapshot!(errors);
    }

    #[test]
    fn ok() {
        let sql = r#"
create table users (
    id bigint
);
create table users (
    id int8
);
create table users (
    id bigserial
);
create table users (
    id serial8
);
        "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::PreferBigInt]);
        let errors = linter.lint(file, sql);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn create_table_many_err() {
        let sql = r#"
create table users (
    foo integer,
    bar serial
);
        "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::PreferBigInt]);
        let errors = linter.lint(file, sql);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors);
    }

    #[test]
    fn alter_table_add_column_err() {
        let sql = r#"
alter table t add column c integer;
        "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::PreferBigInt]);
        let errors = linter.lint(file, sql);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors);
    }

    #[test]
    fn alter_table_alter_column_type_err() {
        let sql = r#"
alter table t alter column c type integer;
        "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::PreferBigInt]);
        let errors = linter.lint(file, sql);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors);
    }

    #[test]
    fn alter_table_alter_column_type_with_quotes_err() {
        let sql = r#"
alter table t alter column c type "integer";
        "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::PreferBigInt]);
        let errors = linter.lint(file, sql);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors);
    }
}
