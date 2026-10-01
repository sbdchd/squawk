use crate::{Linter, Rule, Violation};
use squawk_syntax::{
    Parse, SourceFile,
    ast::{self, AstNode},
};

pub(crate) fn renaming_object(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    for stmt in parse.tree().stmts() {
        match stmt {
            ast::Stmt::AlterView(node) => {
                for action in node.action().into_iter() {
                    if let ast::AlterViewAction::ViewRenameTo(node) = action {
                        ctx.report(Violation::for_node(
                            Rule::RenamingObject,
                            "Renaming a view may break existing clients.".into(),
                            node.syntax(),
                        ));
                    }
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
                for action in node.action().into_iter() {
                    if let ast::AlterFunctionAction::FunctionRenameTo(node) = action {
                        ctx.report(Violation::for_node(
                            Rule::RenamingObject,
                            "Renaming a function may break existing clients.".into(),
                            node.syntax(),
                        ));
                    }
                }
            }
            ast::Stmt::AlterProcedure(node) => {
                for action in node.action().into_iter() {
                    if let ast::AlterProcedureAction::ProcedureRenameTo(node) = action {
                        ctx.report(Violation::for_node(
                            Rule::RenamingObject,
                            "Renaming a procedure may break existing clients.".into(),
                            node.syntax(),
                        ));
                    }
                }
            }
            ast::Stmt::AlterType(node) => {
                for action in node.action().into_iter() {
                    match action {
                        ast::AlterTypeAction::TypeRenameTo(node) => {
                            ctx.report(Violation::for_node(
                                Rule::RenamingObject,
                                "Renaming a type may break existing clients.".into(),
                                node.syntax(),
                            ))
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
                for action in node.action().into_iter() {
                    if let ast::AlterSchemaAction::SchemaRenameTo(node) = action {
                        ctx.report(Violation::for_node(
                            Rule::RenamingObject,
                            "Renaming a schema may break existing clients.".into(),
                            node.syntax(),
                        ));
                    }
                }
            }
            ast::Stmt::AlterDomain(node) => {
                for action in node.action().into_iter() {
                    if let ast::AlterDomainAction::DomainRenameTo(node) = action {
                        ctx.report(Violation::for_node(
                            Rule::RenamingObject,
                            "Renaming a domain may break existing clients.".into(),
                            node.syntax(),
                        ));
                    }
                }
            }
            ast::Stmt::AlterIndex(node) => {
                for action in node.action().into_iter() {
                    if let ast::AlterIndexAction::IndexRenameTo(node) = action {
                        ctx.report(Violation::for_node(
                            Rule::RenamingObject,
                            "Renaming an index may break existing clients.".into(),
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
        let sql = "ALTER VIEW v RENAME TO v2; ALTER MATERIALIZED VIEW mv RENAME TO mv2; ALTER FUNCTION f() RENAME TO f2; ALTER PROCEDURE p() RENAME TO p2; ALTER TYPE typ RENAME TO typ2; ALTER SEQUENCE seq RENAME TO seq2; ALTER SCHEMA s RENAME TO s2; ALTER DOMAIN d RENAME TO d2; ALTER INDEX i RENAME TO i2; ALTER TYPE typ RENAME VALUE 'a' TO 'b';";
        let errors = lint_errors(sql, Rule::RenamingObject);
        assert_eq!(errors.matches("warning[renaming-object]").count(), 10);
        assert_snapshot!(errors);
    }
    #[test]
    fn ok() {
        lint_ok(
            "ALTER TABLE t RENAME TO t2; ALTER TABLE t RENAME COLUMN c TO d;",
            Rule::RenamingObject,
        );
    }
}
