use rustc_hash::FxHashSet;
use squawk_syntax::{
    Parse, SourceFile,
    ast::{self, AstNode},
};

use crate::{Linter, Rule, Violation};

// Track only unconditional CREATE statements earlier in the file. An IF NOT EXISTS
// statement does not establish that the object was newly created.
pub(crate) fn compatibility_additions(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    let mut tables = FxHashSet::default();
    let mut types = FxHashSet::default();
    let mut sequences = FxHashSet::default();

    for stmt in parse.tree().stmts() {
        match stmt {
            ast::Stmt::CreateTable(table) => {
                if table.if_not_exists().is_none() {
                    if let Some(name) = table.table_name().and_then(|n| n.path()) {
                        tables.insert(name.syntax().to_string());
                    }
                }
            }
            ast::Stmt::CreateType(ty) => {
                if let Some(name) = ty.type_name().and_then(|n| n.path()) {
                    types.insert(name.syntax().to_string());
                }
            }
            ast::Stmt::CreateSequence(seq) => {
                if seq.if_not_exists().is_none() {
                    if let Some(name) = seq.sequence().and_then(|n| n.path()) {
                        sequences.insert(name.syntax().to_string());
                    }
                }
            }
            ast::Stmt::AlterTable(table) => {
                let new_table = table
                    .table_relation_name()
                    .and_then(|n| n.table_name_ref())
                    .and_then(|n| n.path_ref())
                    .is_some_and(|n| tables.contains(&n.syntax().to_string()));
                for action in table.actions() {
                    if ctx.rules.contains(&Rule::BanNewWriteRestriction) && !new_table {
                        match &action {
                            ast::AlterTableAction::AddConstraint(add) => {
                                if let Some(constraint) = add.constraint() {
                                    let restriction = match &constraint {
                                        ast::Constraint::CheckConstraint(_)
                                        | ast::Constraint::ForeignKeyConstraint(_)
                                        | ast::Constraint::UniqueConstraint(_)
                                        | ast::Constraint::PrimaryKeyConstraint(_)
                                        | ast::Constraint::ExcludeConstraint(_) => true,
                                        _ => false,
                                    };
                                    if restriction {
                                        ctx.report(Violation::for_node(Rule::BanNewWriteRestriction,
                                            "A new constraint can reject writes from existing clients, even when it is NOT VALID.".into(), add.syntax()));
                                    }
                                }
                            }
                            ast::AlterTableAction::AlterColumn(column) => {
                                if let Some(ast::AlterColumnOption::SetNotNull(node)) =
                                    column.option()
                                {
                                    ctx.report(Violation::for_node(
                                        Rule::BanNewWriteRestriction,
                                        "SET NOT NULL can reject writes from existing clients."
                                            .into(),
                                        node.syntax(),
                                    ));
                                }
                            }
                            _ => (),
                        }
                    }
                    if ctx.rules.contains(&Rule::BanAlterSequenceValues) && !new_table {
                        if let ast::AlterTableAction::AlterColumn(column) = &action {
                            if let Some(option) = column.option() {
                                match option {
                                    ast::AlterColumnOption::Restart(node) => ctx.report(Violation::for_node(
                                        Rule::BanAlterSequenceValues, "Restarting an identity sequence can change values generated for existing clients.".into(), node.syntax())),
                                    ast::AlterColumnOption::SetSequenceOption(node) => ctx.report(Violation::for_node(
                                        Rule::BanAlterSequenceValues, "Changing an identity sequence option can change values generated for existing clients.".into(), node.syntax())),
                                    ast::AlterColumnOption::SetGeneratedOptions(options) => {
                                        for option in options.set_generated_options() {
                                            if matches!(option, ast::SetGeneratedOption::Restart(_) | ast::SetGeneratedOption::SetSequenceOption(_)) {
                                                ctx.report(Violation::for_node(Rule::BanAlterSequenceValues,
                                                    "Changing an identity sequence can change values generated for existing clients.".into(), option.syntax()));
                                            }
                                        }
                                    }
                                    _ => (),
                                }
                            }
                        }
                    }
                    if ctx.rules.contains(&Rule::BanDetachInheritance) && !new_table {
                        match action {
                            ast::AlterTableAction::DetachPartition(node) => ctx.report(Violation::for_node(
                                Rule::BanDetachInheritance,
                                "Detaching a partition changes which rows existing clients can read and write through the parent table.".into(), node.syntax())),
                            ast::AlterTableAction::NoInheritTable(node) => ctx.report(Violation::for_node(
                                Rule::BanDetachInheritance,
                                "NO INHERIT changes which rows existing clients can read and write through the parent table.".into(), node.syntax())),
                            _ => (),
                        }
                    }
                }
            }
            ast::Stmt::CreateIndex(index) if ctx.rules.contains(&Rule::BanNewWriteRestriction) => {
                if index.unique_token().is_some()
                    && index
                        .table_relation_name()
                        .and_then(|n| n.table_name_ref())
                        .and_then(|n| n.path_ref())
                        .is_none_or(|n| !tables.contains(&n.syntax().to_string()))
                {
                    ctx.report(Violation::for_node(Rule::BanNewWriteRestriction,
                        "A unique index can reject writes from existing clients, even when created CONCURRENTLY.".into(), index.syntax()));
                }
            }
            ast::Stmt::AlterType(ty) => {
                let new_type = ty
                    .type_name_ref()
                    .and_then(|n| n.path_ref())
                    .is_some_and(|n| types.contains(&n.syntax().to_string()));
                if !new_type {
                    match ty.action() {
                        Some(ast::AlterTypeAction::AddValue(node))
                            if ctx.rules.contains(&Rule::BanAddEnumValue) =>
                        {
                            ctx.report(Violation::for_node(Rule::BanAddEnumValue,
                                "Adding an enum value changes the set of values existing clients can receive.".into(), node.syntax()));
                        }
                        Some(ast::AlterTypeAction::AlterTypeAttributeActionList(list))
                            if ctx.rules.contains(&Rule::BanAddCompositeAttribute) =>
                        {
                            for action in list.actions() {
                                if let ast::AlterTypeAttributeAction::AddAttribute(node) = action {
                                    ctx.report(Violation::for_node(Rule::BanAddCompositeAttribute,
                                        "Adding a composite attribute changes the shape of values existing clients receive.".into(), node.syntax()));
                                }
                            }
                        }
                        _ => (),
                    }
                }
            }
            ast::Stmt::AlterSequence(seq) if ctx.rules.contains(&Rule::BanAlterSequenceValues) => {
                let new_sequence = seq
                    .sequence_ref()
                    .and_then(|n| n.path_ref())
                    .is_some_and(|n| sequences.contains(&n.syntax().to_string()));
                if !new_sequence {
                    for action in seq.actions() {
                        if let ast::AlterSequenceAction::SequenceOption(option) = action {
                            ctx.report(Violation::for_node(Rule::BanAlterSequenceValues,
                                "Changing a sequence option can change values generated for existing clients.".into(), option.syntax()));
                        }
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
    fn enum_value() {
        assert_eq!(
            lint_errors(
                "ALTER TYPE mood ADD VALUE IF NOT EXISTS 'happy' BEFORE 'sad';",
                Rule::BanAddEnumValue
            )
            .matches("warning[ban-add-enum-value]")
            .count(),
            1
        );
        lint_ok(
            "CREATE TYPE mood AS ENUM ('sad'); ALTER TYPE mood ADD VALUE 'happy';",
            Rule::BanAddEnumValue,
        );
    }

    #[test]
    fn composite_attribute() {
        assert_eq!(
            lint_errors(
                "ALTER TYPE address ADD ATTRIBUTE street text;",
                Rule::BanAddCompositeAttribute
            )
            .matches("warning[ban-add-composite-attribute]")
            .count(),
            1
        );
        lint_ok(
            "CREATE TYPE address AS (city text); ALTER TYPE address ADD ATTRIBUTE street text;",
            Rule::BanAddCompositeAttribute,
        );
        lint_ok(
            "ALTER TYPE address DROP ATTRIBUTE street;",
            Rule::BanAddCompositeAttribute,
        );
    }

    #[test]
    fn detach_inheritance() {
        assert_eq!(lint_errors("ALTER TABLE parent DETACH PARTITION child CONCURRENTLY; ALTER TABLE child NO INHERIT parent;", Rule::BanDetachInheritance).matches("warning[ban-detach-inheritance]").count(), 2);
        lint_ok(
            "CREATE TABLE child (id int); ALTER TABLE child NO INHERIT parent;",
            Rule::BanDetachInheritance,
        );
    }

    #[test]
    fn sequence_values() {
        assert_eq!(
            lint_errors(
                "ALTER SEQUENCE ids RESTART WITH 1 INCREMENT BY 2;",
                Rule::BanAlterSequenceValues
            )
            .matches("warning[ban-alter-sequence-values]")
            .count(),
            2
        );
        lint_ok(
            "CREATE SEQUENCE ids; ALTER SEQUENCE ids RESTART WITH 1;",
            Rule::BanAlterSequenceValues,
        );
        lint_ok(
            "ALTER SEQUENCE ids OWNER TO app;",
            Rule::BanAlterSequenceValues,
        );
        assert_eq!(
            lint_errors(
                "ALTER TABLE t ALTER COLUMN id RESTART WITH 5;",
                Rule::BanAlterSequenceValues
            )
            .matches("warning[ban-alter-sequence-values]")
            .count(),
            1
        );
    }
}
