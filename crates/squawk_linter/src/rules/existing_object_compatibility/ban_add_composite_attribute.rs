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

#[cfg(test)]
mod test {
    use crate::{
        Rule,
        test_utils::{lint_errors, lint_ok},
    };

    #[test]
    fn composite_attribute() {
        assert_eq!(
            lint_errors(
                "ALTER TYPE address ADD ATTRIBUTE street text;",
                Rule::BanAddCompositeAttribute
            )
            .matches("warning[ban-add-composite-attribute]")
            .count(),
            1
        );
        lint_ok(
            "CREATE TYPE address AS (city text); ALTER TYPE address ADD ATTRIBUTE street text;",
            Rule::BanAddCompositeAttribute,
        );
        lint_ok(
            "ALTER TYPE address DROP ATTRIBUTE street;",
            Rule::BanAddCompositeAttribute,
        );
    }
}
