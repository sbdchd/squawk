use crate::{Linter, Rule, Violation};
use squawk_syntax::{
    Parse, SourceFile,
    ast::{self, AstNode},
};

pub(crate) fn ban_revoke(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    for stmt in parse.tree().stmts() {
        for owner in stmt.syntax().descendants().filter_map(ast::OwnerTo::cast) {
            ctx.report(Violation::for_node(Rule::BanRevoke, "Changing object ownership may break existing clients.".into(), owner.syntax()));
        }
        match stmt {
            ast::Stmt::Revoke(node) => {
                ctx.report(Violation::for_node(
                    Rule::BanRevoke,
                    "Revoking privileges may break existing clients.".into(),
                    node.syntax(),
                ));
            }
            ast::Stmt::DropOwned(node) => ctx.report(Violation::for_node(
                Rule::BanRevoke,
                "Dropping owned objects or privileges may break existing clients.".into(),
                node.syntax(),
            )),
            ast::Stmt::Reassign(node) => ctx.report(Violation::for_node(
                Rule::BanRevoke,
                "Reassigning owned objects may break existing clients.".into(),
                node.syntax(),
            )),
            ast::Stmt::DropUser(node) => ctx.report(Violation::for_node(
                Rule::BanRevoke,
                "Dropping a user may break existing clients.".into(),
                node.syntax(),
            )),
            ast::Stmt::DropRole(node) => ctx.report(Violation::for_node(
                Rule::BanRevoke,
                "Dropping a role may break existing clients.".into(),
                node.syntax(),
            )),
            ast::Stmt::AlterDefaultPrivileges(node) => {
                if matches!(
                    node.action(),
                    Some(ast::AlterDefaultPrivilegesAction::RevokeDefaultPrivileges(
                        _
                    ))
                ) {
                    ctx.report(Violation::for_node(
                        Rule::BanRevoke,
                        "Revoking privileges may break existing clients.".into(),
                        node.syntax(),
                    ));
                }
            }
            _ => (),
        }
    }
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
        let sql = "REVOKE SELECT ON t FROM app; ALTER DEFAULT PRIVILEGES REVOKE SELECT ON TABLES FROM app;";
        assert_snapshot!(lint_errors(sql, Rule::BanRevoke));
    }
    #[test]
    fn ok() {
        lint_ok("GRANT SELECT ON t TO app;", Rule::BanRevoke);
    }
}
