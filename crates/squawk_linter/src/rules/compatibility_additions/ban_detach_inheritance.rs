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

#[cfg(test)]
mod test {
    use crate::{
        Rule,
        test_utils::{lint_errors, lint_ok},
    };

    #[test]
    fn detach_inheritance() {
        assert_eq!(lint_errors("ALTER TABLE parent DETACH PARTITION child CONCURRENTLY; ALTER TABLE child NO INHERIT parent;", Rule::BanDetachInheritance).matches("warning[ban-detach-inheritance]").count(), 2);
        lint_ok(
            "CREATE TABLE child (id int); ALTER TABLE child NO INHERIT parent;",
            Rule::BanDetachInheritance,
        );
    }
}
