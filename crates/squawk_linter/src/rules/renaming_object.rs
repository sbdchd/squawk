use crate::{Linter, Rule, Violation};
use squawk_syntax::{
    Parse, SourceFile,
    ast::{self, AstNode},
};

pub(crate) fn renaming_object(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    for stmt in parse.tree().stmts() {
        match stmt {
            ast::Stmt::AlterForeignTable(node) => {
                for action in node.actions() {
                    if let ast::AlterTableAction::TableRenameTo(node) = action {
                        ctx.report(Violation::for_node(
                            Rule::RenamingObject,
                            "Renaming a foreign table may break existing clients.".into(),
                            node.syntax(),
                        ));
                    }
                }
            }
            ast::Stmt::AlterTable(node) => {
                for action in node.actions() {
                    if let ast::AlterTableAction::RenameConstraint(node) = action {
                        ctx.report(Violation::for_node(
                            Rule::RenamingObject,
                            "Renaming a constraint may break existing clients.".into(),
                            node.syntax(),
                        ));
                    }
                }
            }
            ast::Stmt::AlterRole(node) => {
                if let Some(ast::AlterRoleAction::RoleRenameTo(action)) = node.action() {
                    ctx.report(Violation::for_node(
                        Rule::RenamingObject,
                        "Renaming a role may break existing clients.".into(),
                        action.syntax(),
                    ));
                }
            }
            ast::Stmt::AlterUser(node) => {
                if let Some(ast::AlterUserAction::RoleRenameTo(action)) = node.action() {
                    ctx.report(Violation::for_node(
                        Rule::RenamingObject,
                        "Renaming a user may break existing clients.".into(),
                        action.syntax(),
                    ));
                }
            }
            ast::Stmt::AlterGroup(node) => {
                if let Some(ast::AlterGroupAction::RoleRenameTo(action)) = node.action() {
                    ctx.report(Violation::for_node(
                        Rule::RenamingObject,
                        "Renaming a group may break existing clients.".into(),
                        action.syntax(),
                    ));
                }
            }
            ast::Stmt::AlterDatabase(node) => {
                if let Some(ast::AlterDatabaseAction::DatabaseRenameTo(action)) = node.action() {
                    ctx.report(Violation::for_node(
                        Rule::RenamingObject,
                        "Renaming a database may break existing clients.".into(),
                        action.syntax(),
                    ));
                }
            }
            ast::Stmt::AlterTrigger(node) => {
                if let Some(ast::AlterTriggerAction::TriggerRenameTo(action)) = node.action() {
                    ctx.report(Violation::for_node(
                        Rule::RenamingObject,
                        "Renaming a trigger may break existing clients.".into(),
                        action.syntax(),
                    ));
                }
            }
            ast::Stmt::AlterPolicy(node) => {
                if let Some(ast::AlterPolicyAction::PolicyRenameTo(action)) = node.action() {
                    ctx.report(Violation::for_node(
                        Rule::RenamingObject,
                        "Renaming a policy may break existing clients.".into(),
                        action.syntax(),
                    ));
                }
            }
            ast::Stmt::AlterRoutine(node) => {
                if let Some(ast::AlterRoutineAction::RoutineRenameTo(action)) = node.action() {
                    ctx.report(Violation::for_node(
                        Rule::RenamingObject,
                        "Renaming a routine may break existing clients.".into(),
                        action.syntax(),
                    ));
                }
            }
            ast::Stmt::AlterView(node) => {
                if let Some(action) = node.action()
                    && let ast::AlterViewAction::ViewRenameTo(node) = action
                {
                    ctx.report(Violation::for_node(
                        Rule::RenamingObject,
                        "Renaming a view may break existing clients.".into(),
                        node.syntax(),
                    ));
                }
            }
            ast::Stmt::AlterMaterializedView(node) => {
                for action in node.action() {
                    if let ast::AlterMaterializedViewAction::ViewRenameTo(node) = action {
                        ctx.report(Violation::for_node(
                            Rule::RenamingObject,
                            "Renaming a materialized view may break existing clients.".into(),
                            node.syntax(),
                        ));
                    }
                }
            }
            ast::Stmt::AlterFunction(node) => {
                if let Some(action) = node.action()
                    && let ast::AlterFunctionAction::FunctionRenameTo(node) = action
                {
                    ctx.report(Violation::for_node(
                        Rule::RenamingObject,
                        "Renaming a function may break existing clients.".into(),
                        node.syntax(),
                    ));
                }
            }
            ast::Stmt::AlterAggregate(node) => {
                if let Some(ast::AlterAggregateAction::AggregateRenameTo(action)) = node.action() {
                    ctx.report(Violation::for_node(
                        Rule::RenamingObject,
                        "Renaming an aggregate may break existing clients.".into(),
                        action.syntax(),
                    ));
                }
            }
            ast::Stmt::AlterProcedure(node) => {
                if let Some(action) = node.action()
                    && let ast::AlterProcedureAction::ProcedureRenameTo(node) = action
                {
                    ctx.report(Violation::for_node(
                        Rule::RenamingObject,
                        "Renaming a procedure may break existing clients.".into(),
                        node.syntax(),
                    ));
                }
            }
            ast::Stmt::AlterType(node) => {
                if let Some(action) = node.action() {
                    match action {
                        ast::AlterTypeAction::TypeRenameTo(node) => {
                            ctx.report(Violation::for_node(
                                Rule::RenamingObject,
                                "Renaming a type may break existing clients.".into(),
                                node.syntax(),
                            ));
                        }
                        ast::AlterTypeAction::RenameValue(node) => ctx.report(Violation::for_node(
                            Rule::RenamingObject,
                            "Renaming a type value may break existing clients.".into(),
                            node.syntax(),
                        )),
                        _ => (),
                    }
                }
            }
            ast::Stmt::AlterSequence(node) => {
                for action in node.actions() {
                    if let ast::AlterSequenceAction::SequenceRenameTo(node) = action {
                        ctx.report(Violation::for_node(
                            Rule::RenamingObject,
                            "Renaming a sequence may break existing clients.".into(),
                            node.syntax(),
                        ));
                    }
                }
            }
            ast::Stmt::AlterSchema(node) => {
                if let Some(action) = node.action()
                    && let ast::AlterSchemaAction::SchemaRenameTo(node) = action
                {
                    ctx.report(Violation::for_node(
                        Rule::RenamingObject,
                        "Renaming a schema may break existing clients.".into(),
                        node.syntax(),
                    ));
                }
            }
            ast::Stmt::AlterDomain(node) => {
                if let Some(action) = node.action() {
                    match action {
                        ast::AlterDomainAction::DomainRenameTo(node) => {
                            ctx.report(Violation::for_node(
                                Rule::RenamingObject,
                                "Renaming a domain may break existing clients.".into(),
                                node.syntax(),
                            ));
                        }
                        ast::AlterDomainAction::RenameConstraint(node) => {
                            ctx.report(Violation::for_node(
                                Rule::RenamingObject,
                                "Renaming a constraint may break existing clients.".into(),
                                node.syntax(),
                            ));
                        }
                        _ => (),
                    }
                }
            }
            ast::Stmt::AlterIndex(node) => {
                if let Some(action) = node.action()
                    && let ast::AlterIndexAction::IndexRenameTo(node) = action
                {
                    ctx.report(Violation::for_node(
                        Rule::RenamingObject,
                        "Renaming an index may break existing clients.".into(),
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
        let sql = "ALTER VIEW v RENAME TO v2; ALTER MATERIALIZED VIEW mv RENAME TO mv2; ALTER FUNCTION f() RENAME TO f2; ALTER PROCEDURE p() RENAME TO p2; ALTER TYPE typ RENAME TO typ2; ALTER SEQUENCE seq RENAME TO seq2; ALTER SCHEMA s RENAME TO s2; ALTER DOMAIN d RENAME TO d2; ALTER INDEX i RENAME TO i2; ALTER TYPE typ RENAME VALUE 'a' TO 'b';";
        let errors = lint_errors(sql, Rule::RenamingObject);
        assert_eq!(errors.matches("warning[renaming-object]").count(), 10);
        assert_snapshot!(errors);
    }
    #[test]
    fn aggregate() {
        assert_eq!(
            lint_errors(
                "ALTER AGGREGATE agg(int) RENAME TO agg2;",
                Rule::RenamingObject
            )
            .matches("warning[renaming-object]")
            .count(),
            1
        );
    }
    #[test]
    fn other_renames() {
        let sql = "ALTER FOREIGN TABLE ft RENAME TO ft2; ALTER ROUTINE f() RENAME TO g; ALTER TABLE t RENAME CONSTRAINT old TO renamed; ALTER DOMAIN d RENAME CONSTRAINT old TO renamed; ALTER ROLE app RENAME TO app2; ALTER USER app RENAME TO app2; ALTER GROUP app RENAME TO app2; ALTER DATABASE db RENAME TO db2; ALTER TRIGGER tr ON t RENAME TO tr2; ALTER POLICY p ON t RENAME TO p2;";
        let errors = lint_errors(sql, Rule::RenamingObject);
        assert_eq!(errors.matches("warning[renaming-object]").count(), 10);
    }
    #[test]
    fn ok() {
        lint_ok(
            "ALTER TABLE t RENAME TO t2; ALTER TABLE t RENAME COLUMN c TO d;",
            Rule::RenamingObject,
        );
        lint_ok(
            "ALTER AGGREGATE agg(int) OWNER TO app;",
            Rule::RenamingObject,
        );
        lint_ok("ALTER POLICY p ON t USING (true);", Rule::RenamingObject);
    }
}
