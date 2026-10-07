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

#[cfg(test)]
mod test {
    use crate::{
        Rule,
        test_utils::{lint_errors, lint_ok},
    };

    #[test]
    fn enum_value() {
        assert_eq!(
            lint_errors(
                "ALTER TYPE mood ADD VALUE IF NOT EXISTS 'happy' BEFORE 'sad';",
                Rule::BanAddEnumValue
            )
            .matches("warning[ban-add-enum-value]")
            .count(),
            1
        );
        lint_ok(
            "CREATE TYPE mood AS ENUM ('sad'); ALTER TYPE mood ADD VALUE 'happy';",
            Rule::BanAddEnumValue,
        );
    }
}
