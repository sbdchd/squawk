use crate::{Linter, Rule, Violation};
use squawk_syntax::{
    Parse, SourceFile,
    ast::{self, AstNode},
};

pub(crate) fn ban_drop_constraint(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    for stmt in parse.tree().stmts() {
        let actions: Vec<_> = match &stmt {
            ast::Stmt::AlterTable(table) => table.actions().collect(),
            ast::Stmt::AlterForeignTable(table) => table.actions().collect(),
            _ => Vec::new(),
        };
        for action in actions {
            match action {
                ast::AlterTableAction::DropConstraint(node) => ctx.report(Violation::for_node(Rule::BanDropConstraint, "Dropping a constraint may remove a guarantee that existing clients assume.".into(), node.syntax())),
                ast::AlterTableAction::AlterConstraint(node) => {
                    for option in node.constraint_options() {
                        if let ast::ConstraintOption::NotEnforced(option) = option {
                            ctx.report(Violation::for_node(Rule::BanDropConstraint, "Disabling constraint enforcement may remove a guarantee that existing clients assume.".into(), option.syntax()));
                        }
                    }
                }
                _ => (),
            }
        }
        if let ast::Stmt::AlterDomain(domain) = stmt {
            if let Some(ast::AlterDomainAction::DropConstraint(node)) = domain.action() {
                ctx.report(Violation::for_node(Rule::BanDropConstraint, "Dropping a constraint may remove a guarantee that existing clients assume.".into(), node.syntax()));
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
    use insta::assert_snapshot;
    #[test]
    fn err() {
        let sql = "ALTER TABLE t DROP CONSTRAINT IF EXISTS c;";
        assert_snapshot!(lint_errors(sql, Rule::BanDropConstraint));
    }
    #[test]
    fn ok() {
        lint_ok(
            "ALTER TABLE t ADD CONSTRAINT c CHECK (id > 0);",
            Rule::BanDropConstraint,
        );
    }
}
