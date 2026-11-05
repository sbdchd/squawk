use squawk_syntax::ast;

use crate::Linter;

fn is_serial_name(name: &str) -> bool {
    matches!(
        name,
        "serial" | "serial2" | "serial4" | "serial8" | "smallserial" | "bigserial"
    )
}

pub(crate) fn is_serial_type(ty: &ast::Type) -> bool {
    is_not_valid_int_type(ty, is_serial_name)
}

pub(crate) fn is_not_valid_int_type(ty: &ast::Type, is_invalid_name: fn(&str) -> bool) -> bool {
    match ty {
        ast::Type::ArrayType(array_type) => {
            if let Some(ty) = array_type.ty() {
                is_not_valid_int_type(&ty, is_invalid_name)
            } else {
                false
            }
        }
        ast::Type::PathType(path_type) => {
            let Some(segment) = path_type.path_ref().and_then(|x| x.segment()) else {
                return false;
            };
            is_invalid_name(&segment.text())
        }
        ast::Type::VarcharType(_) | ast::Type::CharacterType(_) => false,
        ast::Type::BitType(_) | ast::Type::BitVaryingType(_) => false,
        ast::Type::DoubleType(_) => false,
        ast::Type::TimeType(_) | ast::Type::TimestampType(_) => false,
        ast::Type::IntervalType(_) => false,
    }
}

pub(crate) fn check_not_allowed_types(
    ctx: &mut Linter,
    file: &ast::SourceFile,
    check_ty: impl Fn(&mut Linter, Option<ast::Type>),
) {
    for stmt in file.stmts() {
        match stmt {
            ast::Stmt::CreateTable(create_table) => {
                if let Some(table_arg_list) = create_table.table_arg_list() {
                    for arg in table_arg_list.args() {
                        if let ast::TableArg::Column(column) = arg {
                            check_ty(ctx, column.ty());
                        }
                    }
                }
            }
            ast::Stmt::AlterTable(alter_table) => {
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
