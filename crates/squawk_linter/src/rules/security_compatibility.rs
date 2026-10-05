use crate::{Linter, Rule, Violation};
use squawk_syntax::{
    Parse, SourceFile,
    ast::{self, AstNode},
};

pub(crate) fn security_compatibility(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    for stmt in parse.tree().stmts() {
        match stmt {
            ast::Stmt::DropExtension(node) => report(
                ctx,
                Rule::BanDropExtension,
                "Dropping an extension removes objects used by existing clients.",
                node.syntax(),
            ),
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

    #[test]
    fn security_rules_are_targeted_and_configurable() {
        let cases = [
            (
                Rule::BanDropExtension,
                "DROP EXTENSION IF EXISTS hstore, citext;",
                "CREATE EXTENSION hstore;",
            ),
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
            if rule != Rule::BanDropExtension {
                assert!(
                    !Linter::with_default_rules()
                        .lint(&parse, bad)
                        .iter()
                        .any(|v| v.code == rule),
                    "{bad}"
                );
            }
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
