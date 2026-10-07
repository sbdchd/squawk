use squawk_syntax::ast::{self, AstNode};

use crate::{Linter, Rule, Violation};

pub(super) fn check(ctx: &mut Linter, ty: &ast::AlterType) {
    if let Some(ast::AlterTypeAction::AlterTypeAttributeActionList(list)) = ty.action() {
        for action in list.actions() {
            if let ast::AlterTypeAttributeAction::AddAttribute(node) = action {
                ctx.report(Violation::for_node(
                    Rule::BanAddCompositeAttribute,
                    "Adding a composite attribute changes the shape of values existing clients receive.".into(),
                    node.syntax(),
                ));
            }
        }
    }
}
