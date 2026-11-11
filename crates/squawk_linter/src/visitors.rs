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
    ty.element_type()
        .and_then(|ty| ty.path_ref())
        .and_then(|path| path.segment())
        .is_some_and(|segment| is_invalid_name(&segment.text()))
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
