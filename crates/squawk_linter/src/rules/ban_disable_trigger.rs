use crate::{Linter, Rule, Violation};
use squawk_syntax::{
    Parse, SourceFile,
    ast::{self, AstNode},
};

pub(crate) fn ban_disable_trigger(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    for stmt in parse.tree().stmts() {
        if let ast::Stmt::AlterTable(table) = stmt {
            for action in table.actions() {
                let message = match action {
                    ast::AlterTableAction::EnableReplicaTrigger(_)
                    | ast::AlterTableAction::EnableReplicaRule(_) => {
                        "Replica-only triggers and rules do not fire for normal application writes."
                    }
                    ast::AlterTableAction::DisableTrigger(_)
                    | ast::AlterTableAction::EnableTrigger(_)
                    | ast::AlterTableAction::EnableRule(_)
                    | ast::AlterTableAction::DisableRule(_)
                    | ast::AlterTableAction::EnableAlwaysTrigger(_)
                    | ast::AlterTableAction::EnableAlwaysRule(_) => {
                        "Changing trigger or rule firing may change database side effects for existing clients."
                    }
                    ast::AlterTableAction::DisableRls(_)
                    | ast::AlterTableAction::ForceRls(_)
                    | ast::AlterTableAction::NoForceRls(_) => {
                        "Changing row level security can change visible rows or reject access for existing clients."
                    }
                    _ => continue,
                };
                ctx.report(Violation::for_node(
                    Rule::BanDisableTrigger,
                    message.into(),
                    action.syntax(),
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
        let sql = "ALTER TABLE t DISABLE TRIGGER trg; ALTER TABLE t DISABLE RULE r; ALTER TABLE t DISABLE ROW LEVEL SECURITY; ALTER TABLE t FORCE ROW LEVEL SECURITY;";
        let errors = lint_errors(sql, Rule::BanDisableTrigger);
        assert_eq!(errors.matches("warning[ban-disable-trigger]").count(), 4);
        assert_snapshot!(errors);
    }
    #[test]
    fn enable() {
        let sql = "ALTER TABLE t ENABLE TRIGGER trg; ALTER TABLE t ENABLE RULE r;";
        assert_eq!(
            lint_errors(sql, Rule::BanDisableTrigger)
                .matches("warning[ban-disable-trigger]")
                .count(),
            2
        );
    }

    #[test]
    fn ok() {
        lint_ok(
            "ALTER TABLE t ENABLE ROW LEVEL SECURITY;",
            Rule::BanDisableTrigger,
        );
    }

    #[test]
    fn enforcement_variants() {
        let sql = "ALTER TABLE t ENABLE REPLICA TRIGGER tr; ALTER TABLE t ENABLE REPLICA RULE r; ALTER TABLE t ENABLE ALWAYS TRIGGER tr; ALTER TABLE t ENABLE ALWAYS RULE r; ALTER TABLE t NO FORCE ROW LEVEL SECURITY;";
        assert_eq!(
            lint_errors(sql, Rule::BanDisableTrigger)
                .matches("warning[ban-disable-trigger]")
                .count(),
            5
        );
    }
}
