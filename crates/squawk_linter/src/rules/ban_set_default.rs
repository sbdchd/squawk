use squawk_syntax::ast::{self, AstNode};

use crate::visitors::{DefaultAction, visit_default_actions};
use crate::{Linter, Rule, Violation};

pub(crate) fn ban_set_default(ctx: &mut Linter, file: &ast::SourceFile) {
    visit_default_actions(file, |action| {
        let (node, message) = match action {
            DefaultAction::SetDomain(node) => (
                node,
                "setting a domain default may silently change values written by existing clients",
            ),
            DefaultAction::SetColumn(node) => (
                node,
                "setting a column default may silently change values written by existing clients",
            ),
            DefaultAction::DropDomain(_) | DefaultAction::DropColumn(_) => return,
        };
        ctx.report(Violation::for_node(
            Rule::BanSetDefault,
            message.into(),
            node.syntax(),
        ));
    });
}

#[cfg(test)]
mod test {
    use crate::{
        Rule,
        test_utils::{lint_errors, lint_ok},
    };
    use insta::assert_snapshot;
    #[test]
    fn err() {
        let sql = "ALTER TABLE t ALTER COLUMN c SET DEFAULT 1;";
        assert_snapshot!(lint_errors(sql, Rule::BanSetDefault));
    }
    #[test]
    fn ok() {
        lint_ok(
            "ALTER TABLE t ALTER COLUMN c DROP DEFAULT;",
            Rule::BanSetDefault,
        );
    }

    #[test]
    fn domain_err() {
        let sql = "alter domain d set default 1;";
        assert_snapshot!(lint_errors(sql, Rule::BanSetDefault), @"
        warning[ban-set-default]: setting a domain default may silently change values written by existing clients
          ╭▸ 
        1 │ alter domain d set default 1;
          ╰╴               ━━━━━━━━━━━━━
        ");
    }
}
