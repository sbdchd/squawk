use squawk_syntax::ast::{self, AstNode};

use crate::{Linter, Rule, Violation};

pub(super) fn check_table_action(ctx: &mut Linter, action: &ast::AlterTableAction) {
    match action {
        ast::AlterTableAction::AddConstraint(add) => {
            if let Some(constraint) = add.constraint() {
                let restriction = matches!(
                    constraint,
                    ast::Constraint::CheckConstraint(_)
                        | ast::Constraint::ForeignKeyConstraint(_)
                        | ast::Constraint::UniqueConstraint(_)
                        | ast::Constraint::PrimaryKeyConstraint(_)
                        | ast::Constraint::ExcludeConstraint(_)
                        | ast::Constraint::NotNullConstraint(_)
                );
                if restriction {
                    ctx.report(Violation::for_node(Rule::BanNewWriteRestriction,
                        "A new constraint can reject writes from existing clients, even when it is NOT VALID.".into(), add.syntax()));
                }
            }
        }
        ast::AlterTableAction::AlterColumn(column) => {
            if let Some(ast::AlterColumnOption::SetNotNull(node)) = column.option() {
                ctx.report(Violation::for_node(
                    Rule::BanNewWriteRestriction,
                    "SET NOT NULL can reject writes from existing clients.".into(),
                    node.syntax(),
                ));
            }
        }
        ast::AlterTableAction::AddColumn(column) => check_column_constraints(ctx, column),
        ast::AlterTableAction::AlterConstraint(node) => {
            if node.constraint_options().any(|option| {
                matches!(
                    option,
                    ast::ConstraintOption::Enforced(_)
                        | ast::ConstraintOption::DeferrableConstraintOption(_)
                        | ast::ConstraintOption::NotDeferrableConstraintOption(_)
                        | ast::ConstraintOption::InitiallyImmediateConstraintOption(_)
                        | ast::ConstraintOption::InitiallyDeferredConstraintOption(_)
                )
            }) {
                ctx.report(Violation::for_node(
                    Rule::BanNewWriteRestriction,
                    "Changing constraint enforcement or timing can reject existing client writes."
                        .into(),
                    node.syntax(),
                ));
            }
        }
        _ => (),
    }
}

pub(super) fn check_foreign_table_action(ctx: &mut Linter, action: &ast::AlterTableAction) {
    match action {
        ast::AlterTableAction::AddColumn(column) => check_column_constraints(ctx, column),
        ast::AlterTableAction::AddConstraint(node) => {
            ctx.report(Violation::for_node(
                Rule::BanNewWriteRestriction,
                "A foreign table constraint can reject existing client writes.".into(),
                node.syntax(),
            ));
        }
        ast::AlterTableAction::AlterColumn(column) => {
            if let Some(ast::AlterColumnOption::SetNotNull(node)) = column.option() {
                ctx.report(Violation::for_node(
                    Rule::BanNewWriteRestriction,
                    "SET NOT NULL can reject existing client writes.".into(),
                    node.syntax(),
                ));
            }
        }
        _ => (),
    }
}

pub(super) fn check_domain(ctx: &mut Linter, domain: &ast::AlterDomain) {
    if let Some(action) = domain.action() {
        match action {
            ast::AlterDomainAction::SetNotNull(node) => ctx.report(Violation::for_node(
                Rule::BanNewWriteRestriction,
                "A domain NOT NULL requirement can reject writes from existing clients.".into(), node.syntax())),
            ast::AlterDomainAction::AddConstraint(node) => ctx.report(Violation::for_node(
                Rule::BanNewWriteRestriction,
                "A domain constraint can reject writes from existing clients, even when it is NOT VALID.".into(), node.syntax())),
            _ => (),
        }
    }
}

pub(super) fn check_index(ctx: &mut Linter, index: &ast::CreateIndex) {
    if index.unique_token().is_some() {
        ctx.report(Violation::for_node(Rule::BanNewWriteRestriction,
            "A unique index can reject writes from existing clients, even when created CONCURRENTLY.".into(), index.syntax()));
    }
}

fn check_column_constraints(ctx: &mut Linter, column: &ast::AddColumn) {
    for constraint in column.constraints() {
        if matches!(
            constraint,
            ast::Constraint::NotNullConstraint(_)
                | ast::Constraint::ReferencesConstraint(_)
                | ast::Constraint::CheckConstraint(_)
                | ast::Constraint::UniqueConstraint(_)
                | ast::Constraint::PrimaryKeyConstraint(_)
        ) {
            ctx.report(Violation::for_node(
                Rule::BanNewWriteRestriction,
                "A new column constraint can reject writes from existing clients.".into(),
                constraint.syntax(),
            ));
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        Rule,
        test_utils::{lint_errors, lint_ok},
    };

    #[test]
    fn write_restrictions() {
        let sql = "ALTER TABLE t ADD CONSTRAINT ck CHECK (id > 0) NOT VALID; ALTER TABLE t ADD CONSTRAINT fk FOREIGN KEY (id) REFERENCES p(id) NOT VALID; ALTER TABLE t ALTER COLUMN id SET NOT NULL; CREATE UNIQUE INDEX CONCURRENTLY idx ON t(id);";
        assert_eq!(
            lint_errors(sql, Rule::BanNewWriteRestriction)
                .matches("warning[ban-new-write-restriction]")
                .count(),
            4
        );
        lint_ok(
            "CREATE TABLE t (id int); ALTER TABLE t ADD CONSTRAINT ck CHECK (id > 0) NOT VALID; CREATE UNIQUE INDEX idx ON t(id);",
            Rule::BanNewWriteRestriction,
        );
        lint_ok(
            "CREATE INDEX CONCURRENTLY idx ON t(id);",
            Rule::BanNewWriteRestriction,
        );
    }

    #[test]
    fn more_write_restrictions() {
        assert_eq!(lint_errors("ALTER DOMAIN d SET NOT NULL; ALTER DOMAIN d ADD CONSTRAINT c CHECK (VALUE > 0) NOT VALID;", Rule::BanNewWriteRestriction).matches("warning[ban-new-write-restriction]").count(), 2);
        assert_eq!(
            lint_errors(
                "ALTER TABLE t ADD COLUMN c int NOT NULL;",
                Rule::BanNewWriteRestriction
            )
            .matches("warning[ban-new-write-restriction]")
            .count(),
            1
        );
        assert_eq!(
            lint_errors(
                "ALTER TABLE t ADD CONSTRAINT nn NOT NULL c; ALTER TABLE t ADD NOT NULL c NOT VALID;",
                Rule::BanNewWriteRestriction
            )
            .matches("warning[ban-new-write-restriction]")
            .count(),
            2
        );
        assert_eq!(
            lint_errors(
                "ALTER TABLE t ALTER CONSTRAINT fk NOT DEFERRABLE;",
                Rule::BanNewWriteRestriction
            )
            .matches("warning[ban-new-write-restriction]")
            .count(),
            1
        );
        lint_ok(
            "ALTER TABLE t ALTER CONSTRAINT fk NOT ENFORCED;",
            Rule::BanNewWriteRestriction,
        );
        assert_eq!(
            lint_errors(
                "ALTER FOREIGN TABLE ft ALTER COLUMN c SET NOT NULL;",
                Rule::BanNewWriteRestriction
            )
            .matches("warning[ban-new-write-restriction]")
            .count(),
            1
        );
    }

    #[test]
    fn additional_write_constraint_forms() {
        let sql = "ALTER TABLE t ADD COLUMN c bigint PRIMARY KEY; ALTER FOREIGN TABLE ft ADD COLUMN c int NOT NULL;";
        assert_eq!(
            lint_errors(sql, Rule::BanNewWriteRestriction)
                .matches("warning[ban-new-write-restriction]")
                .count(),
            2
        );
        lint_ok(
            "CREATE TABLE t (id bigint); ALTER TABLE t ADD COLUMN c bigint PRIMARY KEY; ALTER FOREIGN TABLE ft ADD COLUMN c int;",
            Rule::BanNewWriteRestriction,
        );
    }
}
