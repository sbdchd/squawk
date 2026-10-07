use squawk_syntax::ast::{self, AstNode};

use crate::{Linter, Rule, Violation};

pub(super) fn check(ctx: &mut Linter, ty: &ast::AlterType) {
    if let Some(ast::AlterTypeAction::AddValue(node)) = ty.action() {
        ctx.report(Violation::for_node(
            Rule::BanAddEnumValue,
            "Adding an enum value changes the set of values existing clients can receive.".into(),
            node.syntax(),
        ));
    }
}
