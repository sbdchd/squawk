use crate::{Linter, Rule, Violation};
use squawk_syntax::{
    Parse, SourceFile,
    ast::{self, AstNode},
};

pub(crate) fn security_compatibility(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    for stmt in parse.tree().stmts() {
        match stmt {
            ast::Stmt::CreatePolicy(node) => report(
                ctx,
                Rule::BanCreatePolicy,
                "Creating a policy may change access for existing clients.",
                node.syntax(),
            ),
            ast::Stmt::AlterPolicy(node) => {
                if let Some(ast::AlterPolicyAction::AlterPolicyTo(action)) = node.action() {
                    if let Some(roles) = action.policy_roles() {
                        report(
                            ctx,
                            Rule::BanAlterPolicyRoles,
                            "Changing policy roles may change access for existing clients.",
                            roles.syntax(),
                        );
                    }
                    if let Some(using) = action.using_expr_clause() {
                        report(
                            ctx,
                            Rule::BanAlterPolicyCondition,
                            "Changing a policy condition may change access for existing clients.",
                            using.syntax(),
                        );
                    }
                    if let Some(check) = action.with_check_expr_clause() {
                        report(
                            ctx,
                            Rule::BanAlterPolicyCondition,
                            "Changing a policy condition may change access for existing clients.",
                            check.syntax(),
                        );
                    }
                }
            }
            ast::Stmt::AlterFunction(node) => {
                if let Some(ast::AlterFunctionAction::FuncOptionList(options)) = node.action() {
                    report(
                        ctx,
                        Rule::BanAlterFunctionOptions,
                        "Changing function options may change behaviour for existing clients.",
                        options.syntax(),
                    );
                }
            }
            ast::Stmt::AlterProcedure(node) => {
                if let Some(ast::AlterProcedureAction::FuncOptionList(options)) = node.action() {
                    report(
                        ctx,
                        Rule::BanAlterFunctionOptions,
                        "Changing procedure options may change behaviour for existing clients.",
                        options.syntax(),
                    );
                }
            }
            ast::Stmt::AlterRoutine(node) => {
                if let Some(ast::AlterRoutineAction::FuncOptionList(options)) = node.action() {
                    report(
                        ctx,
                        Rule::BanAlterFunctionOptions,
                        "Changing routine options may change behaviour for existing clients.",
                        options.syntax(),
                    );
                }
            }
            ast::Stmt::AlterView(node) => {
                if let Some(action) = node.action() {
                    match action {
                        ast::AlterViewAction::SetOptions(options) => report(
                            ctx,
                            Rule::BanAlterViewOptions,
                            "Changing view options may change behaviour for existing clients.",
                            options.syntax(),
                        ),
                        ast::AlterViewAction::ResetOptions(options) => report(
                            ctx,
                            Rule::BanAlterViewOptions,
                            "Changing view options may change behaviour for existing clients.",
                            options.syntax(),
                        ),
                        _ => {}
                    }
                }
            }
            ast::Stmt::AlterRole(node) => {
                if let Some(action) = node.action() {
                    match action {
                        ast::AlterRoleAction::RoleOptionList(options) => report(
                            ctx,
                            Rule::BanAlterRoleOptions,
                            "Changing role options may change access for existing clients.",
                            options.syntax(),
                        ),
                        ast::AlterRoleAction::SetConfigParam(config) => report(
                            ctx,
                            Rule::BanAlterRoleOptions,
                            "Changing role configuration may change behaviour for existing clients.",
                            config.syntax(),
                        ),
                        ast::AlterRoleAction::ResetConfigParam(config) => report(
                            ctx,
                            Rule::BanAlterRoleOptions,
                            "Changing role configuration may change behaviour for existing clients.",
                            config.syntax(),
                        ),
                        _ => {}
                    }
                }
            }
            ast::Stmt::AlterUser(node) => {
                if let Some(action) = node.action() {
                    match action {
                        ast::AlterUserAction::RoleOptionList(options) => report(
                            ctx,
                            Rule::BanAlterRoleOptions,
                            "Changing role options may change access for existing clients.",
                            options.syntax(),
                        ),
                        ast::AlterUserAction::SetConfigParam(config) => report(
                            ctx,
                            Rule::BanAlterRoleOptions,
                            "Changing role configuration may change behaviour for existing clients.",
                            config.syntax(),
                        ),
                        ast::AlterUserAction::ResetConfigParam(config) => report(
                            ctx,
                            Rule::BanAlterRoleOptions,
                            "Changing role configuration may change behaviour for existing clients.",
                            config.syntax(),
                        ),
                        _ => {}
                    }
                }
            }
            ast::Stmt::AlterDatabase(node) => {
                if let Some(action) = node.action() {
                    match action {
                        ast::AlterDatabaseAction::DatabaseOptionList(options) => report(
                            ctx,
                            Rule::BanAlterDatabaseOptions,
                            "Changing database options may change behaviour for existing clients.",
                            options.syntax(),
                        ),
                        ast::AlterDatabaseAction::SetConfigParam(config) => report(
                            ctx,
                            Rule::BanAlterDatabaseOptions,
                            "Changing database configuration may change behaviour for existing clients.",
                            config.syntax(),
                        ),
                        ast::AlterDatabaseAction::ResetConfigParam(config) => report(
                            ctx,
                            Rule::BanAlterDatabaseOptions,
                            "Changing database configuration may change behaviour for existing clients.",
                            config.syntax(),
                        ),
                        _ => {}
                    }
                }
            }
            ast::Stmt::AlterSystem(node) => {
                if let Some(action) = node.action() {
                    match action {
                        ast::AlterSystemAction::SetConfigParam(config) => report(
                            ctx,
                            Rule::BanAlterSystemOptions,
                            "Changing server configuration may change behaviour for existing clients.",
                            config.syntax(),
                        ),
                        ast::AlterSystemAction::ResetConfigParam(config) => report(
                            ctx,
                            Rule::BanAlterSystemOptions,
                            "Changing server configuration may change behaviour for existing clients.",
                            config.syntax(),
                        ),
                    }
                }
            }
            ast::Stmt::AlterExtension(node) => {
                if let Some(action) = node.action() {
                    match action {
                        ast::AlterExtensionAction::AlterExtensionUpdate(update) => report(
                            ctx,
                            Rule::BanAlterExtension,
                            "Updating an extension can change or remove objects used by existing clients.",
                            update.syntax(),
                        ),
                        ast::AlterExtensionAction::AlterExtensionDrop(drop) => report(
                            ctx,
                            Rule::BanAlterExtension,
                            "Removing an object from an extension changes how the object is managed for existing clients.",
                            drop.syntax(),
                        ),
                        _ => {}
                    }
                }
            }
            ast::Stmt::AlterTable(node) => {
                for action in node.actions() {
                    match action {
                        ast::AlterTableAction::EnableRls(value) => report(
                            ctx,
                            Rule::BanAlterRowLevelSecurity,
                            "Changing row level security may change access for existing clients.",
                            value.syntax(),
                        ),
                        ast::AlterTableAction::DisableRls(value) => report(
                            ctx,
                            Rule::BanAlterRowLevelSecurity,
                            "Changing row level security may change access for existing clients.",
                            value.syntax(),
                        ),
                        ast::AlterTableAction::ForceRls(value) => report(
                            ctx,
                            Rule::BanAlterRowLevelSecurity,
                            "Changing row level security may change access for existing clients.",
                            value.syntax(),
                        ),
                        ast::AlterTableAction::NoForceRls(value) => report(
                            ctx,
                            Rule::BanAlterRowLevelSecurity,
                            "Changing row level security may change access for existing clients.",
                            value.syntax(),
                        ),
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
}

fn report(ctx: &mut Linter, rule: Rule, message: &str, node: &squawk_syntax::SyntaxNode) {
    if ctx.rules.contains(&rule) {
        ctx.report(Violation::for_node(rule, message.into(), node));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{lint_errors, lint_ok};

    #[test]
    fn security_rules_are_targeted_and_configurable() {
        let cases = [
            (
                Rule::BanCreatePolicy,
                "CREATE POLICY p ON t USING (true);",
                "DROP POLICY p ON t;",
            ),
            (
                Rule::BanAlterPolicyRoles,
                "ALTER POLICY p ON t TO admin;",
                "ALTER POLICY p ON t USING (true);",
            ),
            (
                Rule::BanAlterPolicyCondition,
                "ALTER POLICY p ON t USING (true) WITH CHECK (false);",
                "ALTER POLICY p ON t TO admin;",
            ),
            (
                Rule::BanAlterFunctionOptions,
                "ALTER FUNCTION f() SECURITY DEFINER;",
                "ALTER FUNCTION f() RENAME TO g;",
            ),
            (
                Rule::BanAlterFunctionOptions,
                "ALTER PROCEDURE p() SECURITY DEFINER;",
                "ALTER PROCEDURE p() RENAME TO q;",
            ),
            (
                Rule::BanAlterFunctionOptions,
                "ALTER ROUTINE f() SET search_path TO private;",
                "ALTER ROUTINE f() SET SCHEMA private;",
            ),
            (
                Rule::BanAlterRoleOptions,
                "ALTER USER app NOLOGIN;",
                "ALTER USER app RENAME TO app2;",
            ),
            (
                Rule::BanAlterViewOptions,
                "ALTER VIEW v SET (security_barrier = true);",
                "ALTER VIEW v RENAME TO w;",
            ),
            (
                Rule::BanAlterRoleOptions,
                "ALTER ROLE r NOLOGIN;",
                "ALTER ROLE r RENAME TO s;",
            ),
            (
                Rule::BanAlterDatabaseOptions,
                "ALTER DATABASE d SET search_path TO public;",
                "ALTER DATABASE d RENAME TO e;",
            ),
            (
                Rule::BanAlterRowLevelSecurity,
                "ALTER TABLE t ENABLE ROW LEVEL SECURITY;",
                "ALTER TABLE t ADD COLUMN c int;",
            ),
            (
                Rule::BanAlterSystemOptions,
                "ALTER SYSTEM SET timezone = 'UTC';",
                "ALTER DATABASE d SET timezone = 'UTC';",
            ),
            (
                Rule::BanAlterSystemOptions,
                "ALTER SYSTEM RESET ALL;",
                "ALTER ROLE r RESET ALL;",
            ),
            (
                Rule::BanAlterExtension,
                "ALTER EXTENSION postgis UPDATE TO '3.4.0';",
                "ALTER EXTENSION postgis ADD FUNCTION f();",
            ),
            (
                Rule::BanAlterExtension,
                "ALTER EXTENSION postgis DROP FUNCTION f();",
                "ALTER EXTENSION postgis SET SCHEMA gis;",
            ),
        ];
        for (rule, bad, good) in cases {
            assert_eq!(Rule::try_from(rule.to_string().as_str()), Ok(rule));
            let parse = SourceFile::parse(bad);
            assert!(parse.errors().is_empty(), "{bad}: {:?}", parse.errors());
            assert!(
                Linter::from([rule])
                    .lint(&parse, bad)
                    .iter()
                    .any(|v| v.code == rule),
                "{bad}"
            );
            assert!(
                !Linter::with_default_rules()
                    .lint(&parse, bad)
                    .iter()
                    .any(|v| v.code == rule),
                "{bad}"
            );
            let parse = SourceFile::parse(good);
            assert!(parse.errors().is_empty(), "{good}: {:?}", parse.errors());
            assert!(
                !Linter::from([rule])
                    .lint(&parse, good)
                    .iter()
                    .any(|v| v.code == rule),
                "{good}"
            );
        }
    }

    #[test]
    fn routine_options() {
        let sql = "ALTER PROCEDURE p() SECURITY DEFINER; ALTER PROCEDURE p() SET search_path TO private; ALTER PROCEDURE p() RESET ALL; ALTER ROUTINE f() SECURITY INVOKER; ALTER ROUTINE f() SET search_path TO private; ALTER ROUTINE f() RESET ALL;";
        assert_eq!(
            lint_errors(sql, Rule::BanAlterFunctionOptions)
                .matches("warning[ban-alter-function-options]")
                .count(),
            6
        );
        lint_ok(
            "ALTER PROCEDURE p() RENAME TO q; ALTER ROUTINE f() SET SCHEMA private; ALTER PROCEDURE p() OWNER TO app;",
            Rule::BanAlterFunctionOptions,
        );
    }

    #[test]
    fn user_options() {
        let sql = "ALTER USER app NOLOGIN; ALTER USER app NOBYPASSRLS; ALTER USER app IN DATABASE db SET search_path TO private; ALTER USER app RESET ALL;";
        assert_eq!(
            lint_errors(sql, Rule::BanAlterRoleOptions)
                .matches("warning[ban-alter-role-options]")
                .count(),
            4
        );
        lint_ok("ALTER USER app RENAME TO app2;", Rule::BanAlterRoleOptions);
    }

    #[test]
    fn policy_and_configuration_variants() {
        for (rule, sql) in [
            (
                Rule::BanAlterPolicyCondition,
                "ALTER POLICY p ON t WITH CHECK (true);",
            ),
            (
                Rule::BanAlterViewOptions,
                "ALTER VIEW v RESET (security_barrier);",
            ),
            (
                Rule::BanAlterRoleOptions,
                "ALTER ROLE r SET search_path TO public;",
            ),
            (Rule::BanAlterRoleOptions, "ALTER ROLE r RESET search_path;"),
            (
                Rule::BanAlterDatabaseOptions,
                "ALTER DATABASE d WITH CONNECTION LIMIT 5;",
            ),
            (
                Rule::BanAlterDatabaseOptions,
                "ALTER DATABASE d RESET search_path;",
            ),
            (
                Rule::BanAlterRowLevelSecurity,
                "ALTER TABLE t NO FORCE ROW LEVEL SECURITY;",
            ),
        ] {
            let parse = SourceFile::parse(sql);
            assert!(parse.errors().is_empty(), "{sql}: {:?}", parse.errors());
            assert!(
                Linter::from([rule])
                    .lint(&parse, sql)
                    .iter()
                    .any(|v| v.code == rule),
                "{sql}"
            );
        }
    }
}
