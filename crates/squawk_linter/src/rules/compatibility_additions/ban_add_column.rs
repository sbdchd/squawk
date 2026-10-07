use squawk_syntax::ast::{self, AstNode};

use crate::{Linter, Rule, Violation};

pub(super) fn check(ctx: &mut Linter, action: &ast::AlterTableAction) {
    if let ast::AlterTableAction::AddColumn(column) = action {
        ctx.report(Violation::for_node(
            Rule::BanAddColumn,
            "Adding a column changes the shape of rows existing clients receive and can break positional inserts.".into(),
            column.syntax(),
        ));
    }
}
