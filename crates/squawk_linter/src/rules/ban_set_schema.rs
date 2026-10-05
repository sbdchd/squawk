use crate::{Linter, Rule, Violation};
use squawk_syntax::{
    Parse, SourceFile,
    ast::{self, AstNode},
};

pub(crate) fn ban_set_schema(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    for stmt in parse.tree().stmts() {
        match stmt {
            ast::Stmt::AlterForeignTable(node) => {
                for action in node.actions() {
                    if let ast::AlterTableAction::SetSchema(node) = action {
                        ctx.report(Violation::for_node(
                            Rule::BanSetSchema,
                            "Moving an object to another schema may break existing clients.".into(),
                            node.syntax(),
                        ));
                    }
                }
            }
            ast::Stmt::AlterTable(node) => {
                for action in node.actions() {
                    if let ast::AlterTableAction::SetSchema(node) = action {
                        ctx.report(Violation::for_node(
                            Rule::BanSetSchema,
                            "Moving an object to another schema may break existing clients.".into(),
                            node.syntax(),
                        ));
                    }
                }
            }
            ast::Stmt::AlterView(node) => {
                for action in node.action().into_iter() {
                    if let ast::AlterViewAction::SetSchema(node) = action {
                        ctx.report(Violation::for_node(
                            Rule::BanSetSchema,
                            "Moving an object to another schema may break existing clients.".into(),
                            node.syntax(),
                        ));
                    }
                }
            }
            ast::Stmt::AlterMaterializedView(node) => {
                for action in node.action() {
                    if let ast::AlterMaterializedViewAction::SetSchema(node) = action {
                        ctx.report(Violation::for_node(
                            Rule::BanSetSchema,
                            "Moving an object to another schema may break existing clients.".into(),
                            node.syntax(),
                        ));
                    }
                }
            }
            ast::Stmt::AlterProcedure(node) => {
                if let Some(ast::AlterProcedureAction::SetSchema(action)) = node.action() {
                    ctx.report(Violation::for_node(
                        Rule::BanSetSchema,
                        "Moving an object to another schema may break existing clients.".into(),
                        action.syntax(),
                    ));
                }
            }
            ast::Stmt::AlterRoutine(node) => {
                if let Some(ast::AlterRoutineAction::SetSchema(action)) = node.action() {
                    ctx.report(Violation::for_node(
                        Rule::BanSetSchema,
                        "Moving an object to another schema may break existing clients.".into(),
                        action.syntax(),
                    ));
                }
            }
            ast::Stmt::AlterFunction(node) => {
                for action in node.action().into_iter() {
                    if let ast::AlterFunctionAction::SetSchema(node) = action {
                        ctx.report(Violation::for_node(
                            Rule::BanSetSchema,
                            "Moving an object to another schema may break existing clients.".into(),
                            node.syntax(),
                        ));
                    }
                }
            }
            ast::Stmt::AlterType(node) => {
                for action in node.action().into_iter() {
                    if let ast::AlterTypeAction::SetSchema(node) = action {
                        ctx.report(Violation::for_node(
                            Rule::BanSetSchema,
                            "Moving an object to another schema may break existing clients.".into(),
                            node.syntax(),
                        ));
                    }
                }
            }
            ast::Stmt::AlterSequence(node) => {
                for action in node.actions() {
                    if let ast::AlterSequenceAction::SetSchema(node) = action {
                        ctx.report(Violation::for_node(
                            Rule::BanSetSchema,
                            "Moving an object to another schema may break existing clients.".into(),
                            node.syntax(),
                        ));
                    }
                }
            }
            ast::Stmt::AlterDomain(node) => {
                for action in node.action().into_iter() {
                    if let ast::AlterDomainAction::SetSchema(node) = action {
                        ctx.report(Violation::for_node(
                            Rule::BanSetSchema,
                            "Moving an object to another schema may break existing clients.".into(),
                            node.syntax(),
                        ));
                    }
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
        let sql = "ALTER TABLE t SET SCHEMA s; ALTER VIEW v SET SCHEMA s; ALTER MATERIALIZED VIEW mv SET SCHEMA s; ALTER FUNCTION f() SET SCHEMA s; ALTER TYPE typ SET SCHEMA s; ALTER SEQUENCE seq SET SCHEMA s; ALTER DOMAIN d SET SCHEMA s;";
        let errors = lint_errors(sql, Rule::BanSetSchema);
        assert_eq!(errors.matches("warning[ban-set-schema]").count(), 7);
        assert_snapshot!(errors);
    }
    #[test]
    fn ok() {
        lint_ok("ALTER TABLE t OWNER TO app;", Rule::BanSetSchema);
    }
}
