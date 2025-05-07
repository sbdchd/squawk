use std::collections::HashSet;

use squawk_syntax::ast::{self, HasModuleItem};

use crate::{text::trim_quotes, Linter};

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
