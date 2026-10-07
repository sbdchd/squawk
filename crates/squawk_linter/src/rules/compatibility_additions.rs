mod ban_add_column;
mod ban_add_composite_attribute;
mod ban_add_enum_value;
mod ban_alter_sequence_values;
mod ban_detach_inheritance;
mod ban_new_write_restriction;

use rustc_hash::FxHashSet;
use squawk_syntax::{
    Parse, SourceFile,
    ast::{self, AstNode},
};

use crate::{Linter, Rule};

// Only unconditional CREATE statements establish that an object is new to this file.
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
                    if !new_table {
                        if ctx.rules.contains(&Rule::BanAddColumn) {
                            ban_add_column::check(ctx, &action);
                        }
                        if ctx.rules.contains(&Rule::BanNewWriteRestriction) {
                            ban_new_write_restriction::check_table_action(ctx, &action);
                        }
                        if ctx.rules.contains(&Rule::BanAlterSequenceValues) {
                            ban_alter_sequence_values::check_table_action(ctx, &action);
                        }
                        if ctx.rules.contains(&Rule::BanDetachInheritance) {
                            ban_detach_inheritance::check(ctx, &action);
                        }
                    }
                }
            }
            ast::Stmt::AlterForeignTable(table) => {
                for action in table.actions() {
                    if ctx.rules.contains(&Rule::BanAddColumn) {
                        ban_add_column::check(ctx, &action);
                    }
                    if ctx.rules.contains(&Rule::BanNewWriteRestriction) {
                        ban_new_write_restriction::check_foreign_table_action(ctx, &action);
                    }
                }
            }
            ast::Stmt::AlterDomain(domain) if ctx.rules.contains(&Rule::BanNewWriteRestriction) => {
                ban_new_write_restriction::check_domain(ctx, &domain);
            }
            ast::Stmt::CreateIndex(index) if ctx.rules.contains(&Rule::BanNewWriteRestriction) => {
                if index
                    .table_relation_name()
                    .and_then(|n| n.table_name_ref())
                    .and_then(|n| n.path_ref())
                    .is_none_or(|n| !tables.contains(&n.syntax().to_string()))
                {
                    ban_new_write_restriction::check_index(ctx, &index);
                }
            }
            ast::Stmt::AlterType(ty) => {
                let new_type = ty
                    .type_name_ref()
                    .and_then(|n| n.path_ref())
                    .is_some_and(|n| types.contains(&n.syntax().to_string()));
                if !new_type {
                    if ctx.rules.contains(&Rule::BanAddEnumValue) {
                        ban_add_enum_value::check(ctx, &ty);
                    }
                    if ctx.rules.contains(&Rule::BanAddCompositeAttribute) {
                        ban_add_composite_attribute::check(ctx, &ty);
                    }
                }
            }
            ast::Stmt::AlterSequence(seq) if ctx.rules.contains(&Rule::BanAlterSequenceValues) => {
                let new_sequence = seq
                    .sequence_ref()
                    .and_then(|n| n.path_ref())
                    .is_some_and(|n| sequences.contains(&n.syntax().to_string()));
                if !new_sequence {
                    ban_alter_sequence_values::check_sequence(ctx, &seq);
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
    fn add_column() {
        assert_eq!(
            lint_errors("ALTER TABLE t ADD COLUMN c int;", Rule::BanAddColumn)
                .matches("warning[ban-add-column]")
                .count(),
            1
        );
        assert_eq!(
            lint_errors(
                "ALTER FOREIGN TABLE ft ADD COLUMN c int;",
                Rule::BanAddColumn
            )
            .matches("warning[ban-add-column]")
            .count(),
            1
        );
        lint_ok(
            "CREATE TABLE t (id int); ALTER TABLE t ADD COLUMN c int;",
            Rule::BanAddColumn,
        );
        assert_eq!(
            lint_errors(
                "CREATE TABLE IF NOT EXISTS t (id int); ALTER TABLE t ADD COLUMN c int;",
                Rule::BanAddColumn
            )
            .matches("warning[ban-add-column]")
            .count(),
            1
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
            "ALTER SEQUENCE ids OWNER TO app; ALTER SEQUENCE ids OWNED BY t.id; ALTER SEQUENCE ids SET LOGGED; ALTER SEQUENCE ids SET UNLOGGED;",
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
