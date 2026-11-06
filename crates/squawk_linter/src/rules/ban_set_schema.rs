use crate::{Linter, Rule, Violation};
use squawk_syntax::ast::{self, AstNode};

fn report_set_schema(ctx: &mut Linter, set_schema: &ast::SetSchema) {
    ctx.report(Violation::for_node(
        Rule::BanSetSchema,
        "moving an object to another schema may break existing clients".into(),
        set_schema.syntax(),
    ));
}

pub(crate) fn ban_set_schema(ctx: &mut Linter, file: &ast::SourceFile) {
    for stmt in file.stmts() {
        match stmt {
            ast::Stmt::AlterForeignTable(node) => {
                for action in node.actions() {
                    if let ast::AlterTableAction::SetSchema(node) = action {
                        ctx.report(Violation::for_node(
                            Rule::BanSetSchema,
                            "moving an object to another schema may break existing clients".into(),
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
                            "moving an object to another schema may break existing clients".into(),
                            node.syntax(),
                        ));
                    }
                }
            }
            ast::Stmt::AlterView(node) => {
                if let Some(action) = node.action()
                    && let ast::AlterViewAction::SetSchema(node) = action
                {
                    ctx.report(Violation::for_node(
                        Rule::BanSetSchema,
                        "moving an object to another schema may break existing clients".into(),
                        node.syntax(),
                    ));
                }
            }
            ast::Stmt::AlterMaterializedView(node) => {
                for action in node.action() {
                    if let ast::AlterMaterializedViewAction::SetSchema(node) = action {
                        ctx.report(Violation::for_node(
                            Rule::BanSetSchema,
                            "moving an object to another schema may break existing clients".into(),
                            node.syntax(),
                        ));
                    }
                }
            }
            ast::Stmt::AlterAggregate(node) => {
                if let Some(ast::AlterAggregateAction::SetSchema(action)) = node.action() {
                    ctx.report(Violation::for_node(
                        Rule::BanSetSchema,
                        "moving an object to another schema may break existing clients".into(),
                        action.syntax(),
                    ));
                }
            }
            ast::Stmt::AlterExtension(node) => {
                if let Some(ast::AlterExtensionAction::SetSchema(action)) = node.action() {
                    ctx.report(Violation::for_node(
                        Rule::BanSetSchema,
                        "moving an object to another schema may break existing clients".into(),
                        action.syntax(),
                    ));
                }
            }
            ast::Stmt::AlterProcedure(node) => {
                if let Some(ast::AlterProcedureAction::SetSchema(action)) = node.action() {
                    ctx.report(Violation::for_node(
                        Rule::BanSetSchema,
                        "moving an object to another schema may break existing clients".into(),
                        action.syntax(),
                    ));
                }
            }
            ast::Stmt::AlterRoutine(node) => {
                if let Some(ast::AlterRoutineAction::SetSchema(action)) = node.action() {
                    ctx.report(Violation::for_node(
                        Rule::BanSetSchema,
                        "moving an object to another schema may break existing clients".into(),
                        action.syntax(),
                    ));
                }
            }
            ast::Stmt::AlterFunction(node) => {
                if let Some(action) = node.action()
                    && let ast::AlterFunctionAction::SetSchema(node) = action
                {
                    ctx.report(Violation::for_node(
                        Rule::BanSetSchema,
                        "moving an object to another schema may break existing clients".into(),
                        node.syntax(),
                    ));
                }
            }
            ast::Stmt::AlterType(node) => {
                if let Some(action) = node.action()
                    && let ast::AlterTypeAction::SetSchema(node) = action
                {
                    ctx.report(Violation::for_node(
                        Rule::BanSetSchema,
                        "moving an object to another schema may break existing clients".into(),
                        node.syntax(),
                    ));
                }
            }
            ast::Stmt::AlterSequence(node) => {
                for action in node.actions() {
                    if let ast::AlterSequenceAction::SetSchema(node) = action {
                        ctx.report(Violation::for_node(
                            Rule::BanSetSchema,
                            "moving an object to another schema may break existing clients".into(),
                            node.syntax(),
                        ));
                    }
                }
            }
            ast::Stmt::AlterDomain(node) => {
                if let Some(action) = node.action()
                    && let ast::AlterDomainAction::SetSchema(node) = action
                {
                    ctx.report(Violation::for_node(
                        Rule::BanSetSchema,
                        "moving an object to another schema may break existing clients".into(),
                        node.syntax(),
                    ));
                }
            }
            ast::Stmt::AlterCollation(node) => {
                if let Some(ast::AlterCollationAction::SetSchema(action)) = node.action() {
                    report_set_schema(ctx, &action);
                }
            }
            ast::Stmt::AlterConversion(node) => {
                if let Some(ast::AlterConversionAction::SetSchema(action)) = node.action() {
                    report_set_schema(ctx, &action);
                }
            }
            ast::Stmt::AlterOperator(node) => {
                if let Some(ast::AlterOperatorAction::SetSchema(action)) = node.action() {
                    report_set_schema(ctx, &action);
                }
            }
            ast::Stmt::AlterOperatorClass(node) => {
                if let Some(ast::AlterOperatorClassAction::SetSchema(action)) = node.action() {
                    report_set_schema(ctx, &action);
                }
            }
            ast::Stmt::AlterOperatorFamily(node) => {
                if let Some(ast::AlterOperatorFamilyAction::SetSchema(action)) = node.action() {
                    report_set_schema(ctx, &action);
                }
            }
            ast::Stmt::AlterPropertyGraph(node) => {
                if let Some(ast::AlterPropertyGraphAction::SetSchema(action)) = node.action() {
                    report_set_schema(ctx, &action);
                }
            }
            ast::Stmt::AlterStatistics(node) => {
                if let Some(ast::AlterStatisticsAction::SetSchema(action)) = node.action() {
                    report_set_schema(ctx, &action);
                }
            }
            ast::Stmt::AlterTextSearchConfiguration(node) => {
                if let Some(ast::AlterTextSearchConfigurationAction::SetSchema(action)) =
                    node.action()
                {
                    report_set_schema(ctx, &action);
                }
            }
            ast::Stmt::AlterTextSearchDictionary(node) => {
                if let Some(ast::AlterTextSearchDictionaryAction::SetSchema(action)) = node.action()
                {
                    report_set_schema(ctx, &action);
                }
            }
            ast::Stmt::AlterTextSearchParser(node) => {
                if let Some(ast::AlterTextSearchParserAction::SetSchema(action)) = node.action() {
                    report_set_schema(ctx, &action);
                }
            }
            ast::Stmt::AlterTextSearchTemplate(node) => {
                if let Some(ast::AlterTextSearchTemplateAction::SetSchema(action)) = node.action() {
                    report_set_schema(ctx, &action);
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
    fn aggregate_and_extension() {
        let sql = "ALTER AGGREGATE agg(int) SET SCHEMA s; ALTER EXTENSION hstore SET SCHEMA s;";
        assert_eq!(
            lint_errors(sql, Rule::BanSetSchema)
                .matches("warning[ban-set-schema]")
                .count(),
            2
        );
    }
    #[test]
    fn other_schema_moves() {
        let sql = "ALTER PROCEDURE p() SET SCHEMA s; ALTER ROUTINE f() SET SCHEMA s; ALTER FOREIGN TABLE ft SET SCHEMA s;";
        let errors = lint_errors(sql, Rule::BanSetSchema);
        assert_eq!(errors.matches("warning[ban-set-schema]").count(), 3);
    }
    #[test]
    fn remaining_schema_moves() {
        let sql = "
alter collation c set schema s;
alter conversion c set schema s;
alter operator + (int, int) set schema s;
alter operator class oc using btree set schema s;
alter operator family of using btree set schema s;
alter statistics st set schema s;
alter text search configuration tc set schema s;
alter text search dictionary td set schema s;
alter text search parser tp set schema s;
alter text search template tt set schema s;
alter property graph g set schema s;
";
        let errors = lint_errors(sql, Rule::BanSetSchema);
        assert_eq!(errors.matches("warning[ban-set-schema]").count(), 11);
    }
    #[test]
    fn ok() {
        lint_ok("ALTER TABLE t OWNER TO app;", Rule::BanSetSchema);
        lint_ok("ALTER AGGREGATE agg(int) OWNER TO app;", Rule::BanSetSchema);
    }
}
