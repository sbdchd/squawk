use crate::{Linter, Rule, Violation};
use squawk_syntax::{
    Parse, SourceFile,
    ast::{self, AstNode},
};

pub(crate) fn ban_disable_trigger(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    for stmt in parse.tree().stmts() {
        if let ast::Stmt::AlterTable(table) = stmt {
            for action in table.actions() {
                if matches!(
                    action,
                    ast::AlterTableAction::DisableTrigger(_)
                        | ast::AlterTableAction::DisableRule(_)
                        | ast::AlterTableAction::DisableRls(_)
                        | ast::AlterTableAction::ForceRls(_)
                        | ast::AlterTableAction::NoForceRls(_)
                        | ast::AlterTableAction::EnableReplicaTrigger(_)
                        | ast::AlterTableAction::EnableReplicaRule(_)
                        | ast::AlterTableAction::EnableAlwaysTrigger(_)
                        | ast::AlterTableAction::EnableAlwaysRule(_)
                ) {
                    ctx.report(Violation::for_node(Rule::BanDisableTrigger, "Disabling a trigger, rule, or row level security may silently change behaviour for existing clients.".into(), action.syntax()));
                }
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
        let sql = "ALTER TABLE t DISABLE TRIGGER trg; ALTER TABLE t DISABLE RULE r; ALTER TABLE t DISABLE ROW LEVEL SECURITY; ALTER TABLE t FORCE ROW LEVEL SECURITY;";
        let errors = lint_errors(sql, Rule::BanDisableTrigger);
        assert_eq!(errors.matches("warning[ban-disable-trigger]").count(), 4);
        assert_snapshot!(errors);
    }
    #[test]
    fn ok() {
        lint_ok("ALTER TABLE t ENABLE TRIGGER trg;", Rule::BanDisableTrigger);
    }
}
