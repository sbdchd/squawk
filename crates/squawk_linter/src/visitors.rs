use squawk_syntax::ast;

pub(crate) fn is_not_valid_int_type(ty: &ast::Type, is_invalid_name: fn(&str) -> bool) -> bool {
    ty.element_type()
        .and_then(|ty| ty.path_ref())
        .and_then(|path| path.segment())
        .is_some_and(|segment| is_invalid_name(&segment.text()))
}

pub(crate) fn visit_not_allowed_types(
    file: &ast::SourceFile,
    mut visit: impl FnMut(Option<ast::Type>),
) {
    for stmt in file.stmts() {
        match stmt {
            ast::Stmt::CreateTable(create_table) => {
                if let Some(table_arg_list) = create_table.table_arg_list() {
                    for arg in table_arg_list.args() {
                        if let ast::TableArg::Column(column) = arg {
                            visit(column.ty());
                        }
                    }
                }
            }
            ast::Stmt::AlterTable(alter_table) => {
                for action in alter_table.actions() {
                    match action {
                        ast::AlterTableAction::AddColumn(add_column) => {
                            visit(add_column.ty());
                        }
                        ast::AlterTableAction::AlterColumn(alter_column) => {
                            if let Some(ast::AlterColumnOption::SetType(set_type)) =
                                alter_column.option()
                            {
                                visit(set_type.ty());
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

pub(crate) enum DefaultAction {
    SetDomain(ast::SetDefault),
    DropDomain(ast::DropDefault),
    SetColumn(ast::SetDefault),
    DropColumn(ast::DropDefault),
}

pub(crate) fn visit_default_actions(file: &ast::SourceFile, mut visit: impl FnMut(DefaultAction)) {
    for stmt in file.stmts() {
        match stmt {
            ast::Stmt::AlterDomain(domain) => match domain.action() {
                Some(ast::AlterDomainAction::SetDefault(node)) => {
                    visit(DefaultAction::SetDomain(node));
                }
                Some(ast::AlterDomainAction::DropDefault(node)) => {
                    visit(DefaultAction::DropDomain(node));
                }
                _ => (),
            },
            ast::Stmt::AlterView(view) => {
                if let Some(ast::AlterViewAction::AlterViewColumn(column)) = view.action() {
                    match column.alter_view_column_action() {
                        Some(ast::AlterViewColumnAction::SetDefault(node)) => {
                            visit(DefaultAction::SetColumn(node));
                        }
                        Some(ast::AlterViewColumnAction::DropDefault(node)) => {
                            visit(DefaultAction::DropColumn(node));
                        }
                        None => (),
                    }
                }
            }
            ast::Stmt::AlterTable(table) => {
                visit_column_default_actions(table.actions(), &mut visit);
            }
            ast::Stmt::AlterForeignTable(table) => {
                visit_column_default_actions(table.actions(), &mut visit);
            }
            _ => (),
        }
    }
}

fn visit_column_default_actions(
    actions: impl Iterator<Item = ast::AlterTableAction>,
    visit: &mut impl FnMut(DefaultAction),
) {
    for action in actions {
        if let ast::AlterTableAction::AlterColumn(column) = action {
            match column.option() {
                Some(ast::AlterColumnOption::SetDefault(node)) => {
                    visit(DefaultAction::SetColumn(node));
                }
                Some(ast::AlterColumnOption::DropDefault(node)) => {
                    visit(DefaultAction::DropColumn(node));
                }
                _ => (),
            }
        }
    }
}
