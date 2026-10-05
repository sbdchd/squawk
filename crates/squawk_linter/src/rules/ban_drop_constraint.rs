use crate::{Linter, Rule, Violation};
use squawk_syntax::{
    Parse, SourceFile,
    ast::{self, AstNode},
};

pub(crate) fn ban_drop_constraint(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    for stmt in parse.tree().stmts() {
        let actions = match &stmt {
            ast::Stmt::AlterTable(table) => Some(table.actions()),
            ast::Stmt::AlterForeignTable(table) => Some(table.actions()),
            _ => None,
        };
        for action in actions.into_iter().flatten() {
            match action {
                ast::AlterTableAction::DropConstraint(node) => ctx.report(Violation::for_node(
                    Rule::BanDropConstraint,
                    "Dropping a constraint may remove a guarantee that existing clients assume."
                        .into(),
                    node.syntax(),
                )),
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
                ctx.report(Violation::for_node(
                    Rule::BanDropConstraint,
                    "Dropping a constraint may remove a guarantee that existing clients assume."
                        .into(),
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
    use insta::assert_snapshot;
    #[test]
    fn err() {
        let sql = "ALTER TABLE t DROP CONSTRAINT IF EXISTS c;";
        assert_snapshot!(lint_errors(sql, Rule::BanDropConstraint));
    }
    #[test]
    fn other_constraints() {
        let sql = "ALTER DOMAIN d DROP CONSTRAINT c; ALTER TABLE t ALTER CONSTRAINT c NOT ENFORCED; ALTER FOREIGN TABLE ft DROP CONSTRAINT c;";
        let errors = lint_errors(sql, Rule::BanDropConstraint);
        assert_eq!(errors.matches("warning[ban-drop-constraint]").count(), 3);
    }
    #[test]
    fn ok() {
        lint_ok(
            "ALTER TABLE t ADD CONSTRAINT c CHECK (id > 0);",
            Rule::BanDropConstraint,
        );
        lint_ok(
            "ALTER TABLE t ALTER CONSTRAINT c ENFORCED;",
            Rule::BanDropConstraint,
        );
    }
}
