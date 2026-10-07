use squawk_syntax::ast::{self, AstNode};

use crate::{Linter, Rule, Violation};

pub(super) fn check(ctx: &mut Linter, action: &ast::AlterTableAction) {
    match action {
        ast::AlterTableAction::DetachPartition(node) => ctx.report(Violation::for_node(
            Rule::BanDetachInheritance,
            "Detaching a partition changes which rows existing clients can read and write through the parent table.".into(), node.syntax())),
        ast::AlterTableAction::NoInheritTable(node) => ctx.report(Violation::for_node(
            Rule::BanDetachInheritance,
            "NO INHERIT changes which rows existing clients can read and write through the parent table.".into(), node.syntax())),
        _ => (),
    }
}
