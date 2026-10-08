use crate::{Linter, Rule, Violation};
use squawk_syntax::{
    Parse, SourceFile,
    ast::{self, AstNode},
};

pub(crate) fn ban_replica_identity(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    for stmt in parse.tree().stmts() {
        match stmt {
            ast::Stmt::AlterTable(table) => {
                for action in table.actions() {
                    if let ast::AlterTableAction::ReplicaIdentity(node) = action {
                        ctx.report(Violation::for_node(Rule::BanReplicaIdentity, "Changing replica identity may silently change replication for existing clients.".into(), node.syntax()));
                    }
                }
            }
            ast::Stmt::DropPublication(node) => ctx.report(Violation::for_node(
                Rule::BanReplicaIdentity,
                "Dropping a publication stops replication for existing consumers.".into(),
                node.syntax(),
            )),
            ast::Stmt::AlterPublication(publication) => {
                if let Some(action) = publication.action()
                    && matches!(
                        action,
                        ast::AlterPublicationAction::DropPublicationObjects(_)
                            | ast::AlterPublicationAction::SetPublicationObjects(_)
                            | ast::AlterPublicationAction::SetAllPublicationObjectList(_)
                            | ast::AlterPublicationAction::SetOptions(_)
                    )
                {
                    ctx.report(Violation::for_node(
                            Rule::BanReplicaIdentity,
                            "Changing publication tables or options can stop or change replication for existing consumers.".into(),
                            action.syntax(),
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
        let sql = "ALTER TABLE t REPLICA IDENTITY FULL;";
        assert_snapshot!(lint_errors(sql, Rule::BanReplicaIdentity));
    }
    #[test]
    fn ok() {
        lint_ok(
            "ALTER TABLE t ENABLE ROW LEVEL SECURITY; ALTER PUBLICATION p ADD TABLE t; CREATE PUBLICATION p FOR TABLE t;",
            Rule::BanReplicaIdentity,
        );
    }

    #[test]
    fn publication_changes() {
        let sql = "DROP PUBLICATION p; ALTER PUBLICATION p DROP TABLE t; ALTER PUBLICATION p SET TABLE t; ALTER PUBLICATION p SET (publish = 'insert'); ALTER PUBLICATION p SET TABLES IN SCHEMA public;";
        assert_eq!(
            lint_errors(sql, Rule::BanReplicaIdentity)
                .matches("warning[ban-replica-identity]")
                .count(),
            5
        );
    }
}
