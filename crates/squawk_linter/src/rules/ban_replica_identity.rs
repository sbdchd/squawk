use crate::{Linter, Rule, Violation};
use squawk_syntax::{
    Parse, SourceFile,
    ast::{self, AstNode},
};

pub(crate) fn ban_replica_identity(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    for stmt in parse.tree().stmts() {
        if let ast::Stmt::AlterTable(table) = stmt {
            for action in table.actions() {
                if let ast::AlterTableAction::ReplicaIdentity(node) = action {
                    ctx.report(Violation::for_node(Rule::BanReplicaIdentity, "Changing replica identity may silently change replication for existing clients.".into(), node.syntax()));
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
        let sql = "ALTER TABLE t REPLICA IDENTITY FULL;";
        assert_snapshot!(lint_errors(sql, Rule::BanReplicaIdentity));
    }
    #[test]
    fn ok() {
        lint_ok(
            "ALTER TABLE t ENABLE ROW LEVEL SECURITY;",
            Rule::BanReplicaIdentity,
        );
    }
}
