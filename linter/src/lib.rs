#![allow(clippy::shadow_unrelated)]
#![allow(clippy::missing_errors_doc)]
pub mod errors;
pub mod rules;
pub mod versions;
pub mod violations;
#[macro_use]
extern crate lazy_static;

use crate::errors::CheckSqlError;
use crate::rules::adding_required_field;
use crate::rules::ban_alter_domain_with_add_constraint;
use crate::rules::ban_concurrent_index_creation_in_transaction;
use crate::rules::ban_create_domain_with_constraint;
use crate::rules::ban_drop_not_null;
use crate::rules::prefer_big_int;
use crate::rules::prefer_identity;
use crate::rules::transaction_nesting;
use crate::rules::{
    adding_field_with_default, adding_foreign_key_constraint, adding_not_nullable_field,
    adding_primary_key_constraint, ban_char_type, ban_drop_column, ban_drop_database,
    ban_drop_table, changing_column_type, constraint_missing_not_valid, disallow_unique_constraint,
    prefer_bigint_over_int, prefer_bigint_over_smallint, prefer_robust_stmts, prefer_text_field,
    prefer_timestamptz, renaming_column, renaming_table, require_concurrent_index_creation,
    require_concurrent_index_deletion,
};
use crate::violations::{RuleViolation, RuleViolationKind, ViolationMessage};
use squawk_parser::ast::RawStmt;
use squawk_parser::parse::parse_sql_query;
use std::collections::HashSet;
use versions::Version;

#[derive(Clone)]
pub struct SquawkRule {
    pub name: RuleViolationKind,
    func: fn(&[RawStmt], Option<Version>, bool) -> Vec<RuleViolation>,
    pub messages: Vec<ViolationMessage>,
}

lazy_static! {
    pub static ref RULES: Vec<SquawkRule> = vec![
    // see ChangingColumnType
    SquawkRule {
        name: RuleViolationKind::AddingFieldWithDefault,
        func: adding_field_with_default,
        messages: vec![
            ViolationMessage::Note(
                "Adding a field with a VOLATILE DEFAULT requires a table rewrite with an ACCESS EXCLUSIVE lock. In Postgres versions 11+, non-VOLATILE DEFAULTs can be added without a rewrite.".into(),
            ),
            ViolationMessage::Help(
                "Add the field as nullable, then set a default, backfill, and remove nullabilty.".into(),
            ),

        ],
    },
    SquawkRule {
        name: RuleViolationKind::AddingForeignKeyConstraint,
        func: adding_foreign_key_constraint,
        messages: vec![
            ViolationMessage::Note(
                "Requires a table scan of the table you're altering and a SHARE ROW EXCLUSIVE lock on both tables, which blocks writes to both tables while your table is scanned.".into()
            ),
            ViolationMessage::Help("Add NOT VALID to the constraint in one transaction and then VALIDATE the constraint in a separate transaction.".into()),
        ]
    },
    // usually paired with a DEFAULT
    SquawkRule {
        name: RuleViolationKind::AddingNotNullableField,
        func: adding_not_nullable_field,
        messages: vec![
            // https://www.postgresql.org/docs/10/sql-altertable.html
            ViolationMessage::Note(
                "Adding a NOT NULL field requires exclusive locks and table rewrites.".into(),
            ),
            ViolationMessage::Help("Make the field nullable.".into())
        ],
    },
    SquawkRule {
        name: RuleViolationKind::AddingRequiredField,
        func: adding_required_field,
        messages: vec![
            ViolationMessage::Note(
                "Adding a NOT NULL field without a DEFAULT will fail for a populated table.".into()
            ),
            ViolationMessage::Help(
                "Make the field nullable or add a non-VOLATILE DEFAULT (Postgres 11+).".into()
            ),
        ],
    },
    SquawkRule {
        name: RuleViolationKind::AddingSerialPrimaryKeyField,
        func: adding_primary_key_constraint,
        messages: vec![
            ViolationMessage::Note(
                "Adding a PRIMARY KEY constraint results in locks and table rewrites".into(),
            ),
            ViolationMessage::Help(
                "Add the PRIMARY KEY constraint USING an index.".into(),
            ),

        ],
    },
    SquawkRule {
        name: RuleViolationKind::BanAlterDomainWithAddConstraint,
        func: ban_alter_domain_with_add_constraint,
        messages: vec![
            ViolationMessage::Note(
                "Domains with constraints have poor support for online migrations".into()
            ),
        ],
    },
    SquawkRule {
        name: RuleViolationKind::BanCharField,
        func: ban_char_type,
        messages: vec![
            ViolationMessage::Help(
                "Use text or varchar instead.".into()
            ),
        ]
    },
    SquawkRule {
        name: RuleViolationKind::BanConcurrentIndexCreationInTransaction,
        func: ban_concurrent_index_creation_in_transaction,
        messages: vec![
            ViolationMessage::Note(
                "Concurrent index creation is not allowed inside a transaction.".into()
            ),
            ViolationMessage::Help(
                "Build the index outside any transactions.".into()
            ),
        ],
    },
    SquawkRule {
        name: RuleViolationKind::BanCreateDomainWithConstraint,
        func: ban_create_domain_with_constraint,
        messages: vec![
            ViolationMessage::Note(
                "Domains with constraints have poor support for online migrations".into()
            ),
        ],
    },
    SquawkRule {
        name: RuleViolationKind::BanDropColumn,
        func: ban_drop_column,
        messages: vec![
            ViolationMessage::Note(
                "Dropping a column may break existing clients.".into()
            ),
        ],
    },
    SquawkRule {
        name: RuleViolationKind::BanDropDatabase,
        func: ban_drop_database,
        messages: vec![
            ViolationMessage::Note(
                "Dropping a database may break existing clients.".into()
            )
        ],
    },
    SquawkRule {
        name: RuleViolationKind::BanDropNotNull,
        func: ban_drop_not_null,
        messages: vec![
            ViolationMessage::Note(
                "Dropping a NOT NULL constraint may break existing clients.".into()
            ),
        ],
    },
    SquawkRule {
        name: RuleViolationKind::BanDropTable,
        func: ban_drop_table,
        messages: vec![
            ViolationMessage::Note(
                "Dropping a table may break existing clients.".into()
            )
        ],
    },
    // > Adding a column with a volatile DEFAULT or changing the type of an
    // > existing column will require the entire table and its indexes to be
    // > rewritten. As an exception, when changing the type of an existing
    // > column, if the USING clause does not change the column contents and
    // > the old type is either binary coercible to the new type or an
    // > unconstrained domain over the new type, a table rewrite is not
    // > needed; but any indexes on the affected columns must still be
    // > rebuilt. Table and/or index rebuilds may take a significant amount of
    // > time for a large table; and will temporarily require as much as
    // > double the disk space.
    // https://www.postgresql.org/docs/current/sql-altertable.html
    SquawkRule {
        name: RuleViolationKind::ChangingColumnType,
        func: changing_column_type,
        messages: vec![
            ViolationMessage::Note("Requires an ACCESS EXCLUSIVE lock on the table which blocks reads.".into()),
            ViolationMessage::Note("Changing the type may break existing clients.".into()),
        ],
    },

    // > Scanning a large table to verify a new foreign key or check
    // > constraint can take a long time, and other updates to the table are
    // > locked out until the ALTER TABLE ADD CONSTRAINT command is committed.
    // > The main purpose of the NOT VALID constraint option is to reduce the
    // > impact of adding a constraint on concurrent updates. With NOT VALID,
    // > the ADD CONSTRAINT command does not scan the table and can be
    // > committed immediately. After that, a VALIDATE CONSTRAINT command can
    // > be issued to verify that existing rows satisfy the constraint. The
    // > validation step does not need to lock out concurrent updates, since
    // > it knows that other transactions will be enforcing the constraint for
    // > rows that they insert or update; only pre-existing rows need to be
    // > checked. Hence, validation acquires only a SHARE UPDATE EXCLUSIVE
    // > lock on the table being altered. (If the constraint is a foreign key
    // > then a ROW SHARE lock is also required on the table referenced by the
    // > constraint.) In addition to improving concurrency, it can be useful
    // > to use NOT VALID and VALIDATE CONSTRAINT in cases where the table is
    // > known to contain pre-existing violations. Once the constraint is in
    // > place, no new violations can be inserted, and the existing problems
    // > can be corrected at leisure until VALIDATE CONSTRAINT finally
    // > succeeds.
    // https://www.postgresql.org/docs/current/sql-altertable.html#SQL-ALTERTABLE-NOTES
    SquawkRule {
        name: RuleViolationKind::ConstraintMissingNotValid,
        func: constraint_missing_not_valid,
        messages: vec![
            ViolationMessage::Note("Requires a table scan to verify constraint and an ACCESS EXCLUSIVE lock which blocks reads.".into()),
            ViolationMessage::Help("Add NOT VALID to the constraint in one transaction and then VALIDATE the constraint in a separate transaction.".into()),
        ],
    },
    // > Although most forms of ADD table_constraint require an ACCESS
    // > EXCLUSIVE lock, ADD FOREIGN KEY requires only a SHARE ROW EXCLUSIVE
    // > lock.
    // https://www.postgresql.org/docs/current/sql-altertable.html
    SquawkRule {
        name: RuleViolationKind::DisallowedUniqueConstraint,
        func: disallow_unique_constraint,
        messages: vec![
            ViolationMessage::Note(
                "Adding a UNIQUE constraint requires an ACCESS EXCLUSIVE lock which blocks reads.".into(),
            ),
            ViolationMessage::Help(
                "Create an index CONCURRENTLY and create the constraint using the index.".into(),
            ),

        ],
    },
    SquawkRule {
        name: RuleViolationKind::PreferBigInt,
        func: prefer_big_int,
        messages: vec![
            ViolationMessage::Note(
                "Hitting the max 32 bit integer is possible and may break your application.".into()
            ),
            ViolationMessage::Help(
                "Use 64bit integer values instead to prevent hitting this limit.".into()
            ),
        ],
    },
    SquawkRule {
        name: RuleViolationKind::PreferBigintOverInt,
        func: prefer_bigint_over_int,
        messages: vec![
            ViolationMessage::Note(
                "Hitting the max 32 bit integer is possible and may break your application.".into()
            ),
            ViolationMessage::Help(
                "Use 64bit integer values instead to prevent hitting this limit.".into()
            ),
        ],
    },
    SquawkRule {
        name: RuleViolationKind::PreferBigintOverSmallint,
        func: prefer_bigint_over_smallint,
        messages: vec![
            ViolationMessage::Note(
                "Hitting the max 16 bit integer is possible and may break your application.".into()
            ),
            ViolationMessage::Help(
                "Use 64bit integer values instead to prevent hitting this limit.".into()
            ),
        ],
    },
    SquawkRule {
        name: RuleViolationKind::PreferIdentity,
        func: prefer_identity,
        messages: vec![
            ViolationMessage::Note(
                "Serial types have confusing behaviors that make schema management difficult.".into()
            ),
            ViolationMessage::Help(
                "Use identity columns instead for more features and better usability.".into()
            ),
        ],
    },
    SquawkRule {
        name: RuleViolationKind::PreferRobustStmts,
        func: prefer_robust_stmts,
        messages: vec![
            ViolationMessage::Help(
                "Consider wrapping in a transaction or adding a IF NOT EXISTS clause if the statement supports it.".into()
            ),
        ]
    },
    // see ConstraintMissingNotValid for more docs
    SquawkRule {
        name: RuleViolationKind::PreferTextField,
        func: prefer_text_field,
        messages: vec![
            ViolationMessage::Note(
                "Changing the size of a varchar field requires an ACCESS EXCLUSIVE lock.".into()
            ),
            ViolationMessage::Help(
                "Use a text field with a check constraint.".into()
            ),
        ]
    },
    SquawkRule {
        name: RuleViolationKind::PreferTimestampTz,
        func: prefer_timestamptz,
        messages: vec![
            ViolationMessage::Note(
                "A timestamp field without a timezone can lead to data loss, depending on your database session timezone.".into()
            ),
            ViolationMessage::Help(
                "Use timestamptz instead of timestamp for your column type.".into()
            ),
        ]
    },
    // > The RENAME forms change the name of a table (or an index, sequence,
    // > view, materialized view, or foreign table), the name of an individual
    // > column in a table, or the name of a constraint of the table. There is
    // > no effect on the stored data.
    // https://www.postgresql.org/docs/10/sql-altertable.html
    SquawkRule {
        name: RuleViolationKind::RenamingColumn,
        func: renaming_column,
        messages: vec![
            ViolationMessage::Note(
                "Renaming a column may break existing clients.".into()
            ),
        ],
    },
    // see RenamingColumn rule
    SquawkRule {
        name: RuleViolationKind::RenamingTable,
        func: renaming_table,
        messages: vec![
            ViolationMessage::Note(
                "Renaming a table may break existing clients.".into()
            ),
        ],
    },
    // https://www.postgresql.org/docs/10/sql-createindex.html#SQL-CREATEINDEX-CONCURRENTLY
    SquawkRule {
        name: RuleViolationKind::RequireConcurrentIndexCreation,
        func: require_concurrent_index_creation,
        messages: vec![
            ViolationMessage::Note(
                "Creating an index blocks writes.".into()
            ),
            ViolationMessage::Help(
                "Create the index CONCURRENTLY.".into()
            ),
        ],
    },
    // https://www.postgresql.org/docs/10/sql-dropindex.html
    SquawkRule {
        name: RuleViolationKind::RequireConcurrentIndexDeletion,
        func: require_concurrent_index_deletion,
        messages: vec![
            ViolationMessage::Note(
                "Deleting an index blocks selects, inserts, updates, and deletes on the index's table.".into()
            ),
            ViolationMessage::Help(
                "Delete the index CONCURRENTLY.".into()
            ),
        ],
    },
    SquawkRule {
        name: RuleViolationKind::TransactionNesting,
        func: transaction_nesting,
        messages: vec![
            ViolationMessage::Note(
                "There is an existing transaction already in progress.".into()
            ),
            ViolationMessage::Help(
                "COMMIT the previous transaction before issuing a BEGIN or START TRANSACTION statement.".into()
            ),
        ],
    },
    // generator::new-rule-above
    ];

}

pub fn check_sql(
    sql: &str,
    excluded_rules: &[RuleViolationKind],
    pg_version: Option<Version>,
    assume_in_transaction: bool,
) -> Result<Vec<RuleViolation>, CheckSqlError> {
    let tree = parse_sql_query(sql)?;

    let excluded_rules: HashSet<RuleViolationKind> = excluded_rules.iter().cloned().collect();

    let mut errs = vec![];
    for rule in RULES.iter().filter(|r| !excluded_rules.contains(&r.name)) {
        errs.extend((rule.func)(&tree, pg_version, assume_in_transaction));
    }

    errs.sort_by_key(|v| v.span.start);

    Ok(errs)
}

pub fn check_sql_with_rule(
    sql: &str,
    rule_kind: &RuleViolationKind,
    pg_version: Option<Version>,
    assume_in_transaction: bool,
) -> Result<Vec<RuleViolation>, CheckSqlError> {
    let tree = parse_sql_query(sql)?;
    let mut errs = vec![];
    for rule in RULES.iter() {
        if rule.name == *rule_kind {
            errs.extend((rule.func)(&tree, pg_version, assume_in_transaction));
        }
    }

    errs.sort_by_key(|v| v.span.start);

    Ok(errs)
}

#[cfg(test)]
mod test_rules {
    use super::*;
    use insta::{assert_debug_snapshot, assert_display_snapshot};
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    fn rules_should_be_sorted() {
        let original_rules: Vec<String> = RULES.iter().map(|x| x.name.to_string()).collect();
        let mut sorted_rule_ids = original_rules.clone();
        sorted_rule_ids.sort();
        assert_eq!(original_rules, sorted_rule_ids);
    }
    /// Ensure we handle both serializing and deserializing `RuleViolationKind`
    #[test]
    fn parsing_rule_kind() {
        let rule_names = RULES.iter().map(|r| r.name.clone());
        for rule in rule_names {
            let rule_str = rule.to_string();
            assert_eq!(RuleViolationKind::from_str(&rule_str), Ok(rule.clone()));
            assert_eq!(RuleViolationKind::try_from(rule_str.as_ref()), Ok(rule));
        }
    }
    /// Ensure rule names don't change
    #[test]
    fn rule_names_debug_snap() {
        let rule_names: Vec<String> = RULES.iter().map(|r| r.name.to_string()).collect();
        assert_debug_snapshot!(rule_names);
    }
    #[test]
    fn rule_names_display_snap() {
        let rule_names: Vec<String> = RULES.iter().map(|r| r.name.to_string()).collect();
        assert_display_snapshot!(rule_names.join("\n"));
    }

    /// Ensure we stort the resulting violations by where they occur in the file.
    #[test]
    fn check_rules_orderin() {
        let sql = r#"
  ALTER TABLE "table_name" RENAME COLUMN "column_name" TO "new_column_name";
  CREATE INDEX "field_name_idx" ON "table_name" ("field_name");
  "#;

        let res = check_sql(sql, &[RuleViolationKind::PreferRobustStmts], None, false)
            .expect("valid parsing of SQL");
        let mut prev_span_start = -1;
        for violation in &res {
            assert!(violation.span.start > prev_span_start);
            prev_span_start = violation.span.start;
        }
    }
}
