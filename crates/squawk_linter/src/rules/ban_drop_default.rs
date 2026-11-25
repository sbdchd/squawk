use squawk_syntax::ast::{self, AstNode};

use crate::visitors::{DefaultAction, visit_default_actions};
use crate::{Linter, Rule, Violation};

pub(crate) fn ban_drop_default(ctx: &mut Linter, file: &ast::SourceFile) {
    visit_default_actions(file, |action| {
        let (node, message) = match action {
            DefaultAction::DropDomain(node) => {
                (node, "dropping a domain default may break existing clients")
            }
            DefaultAction::DropColumn(node) => {
                (node, "dropping a column default may break existing clients")
            }
            DefaultAction::SetDomain(_) | DefaultAction::SetColumn(_) => return,
        };
        ctx.report(Violation::for_node(
            Rule::BanDropDefault,
            message.into(),
            node.syntax(),
        ));
    });
}

#[cfg(test)]
mod test {
    use insta::assert_snapshot;

    use crate::{
        Rule,
        test_utils::{lint_errors, lint_ok},
    };

    #[test]
    fn err() {
        let sql = r#"
ALTER TABLE tbl ALTER COLUMN c DROP DEFAULT;
ALTER TABLE IF EXISTS tbl ALTER COLUMN c DROP DEFAULT;
ALTER TABLE tbl ALTER COLUMN c DROP DEFAULT, ALTER COLUMN d DROP DEFAULT;
"#;
        assert_snapshot!(lint_errors(sql, Rule::BanDropDefault));
    }

    #[test]
    fn ok() {
        lint_ok(
            "ALTER TABLE tbl ALTER COLUMN c SET DEFAULT 1; ALTER TABLE tbl ALTER COLUMN c DROP NOT NULL; DROP DOMAIN d; DROP INDEX i;",
            Rule::BanDropDefault,
        );
    }

    #[test]
    fn domain_err() {
        let sql = "alter domain d drop default;";
        assert_snapshot!(lint_errors(sql, Rule::BanDropDefault), @"
        warning[ban-drop-default]: dropping a domain default may break existing clients
          ╭▸ 
        1 │ alter domain d drop default;
          ╰╴               ━━━━━━━━━━━━
        ");
    }
}
