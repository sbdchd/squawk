use squawk_parser::ast::RawStmt;
use squawk_parser::ast::{
    AlterTableCmds, AlterTableDef, AlterTableType, ColumnDefConstraint, ColumnDefTypeName,
    ConstrType, ObjectType, QualifiedName, RelationKind, RootStmt, Stmt, TableElt,
    TransactionStmtKind,
};
use squawk_parser::error::PGQueryError;
use squawk_parser::parse::parse_sql_query;

use serde::Serialize;
use std::collections::HashSet;
use std::convert::TryFrom;

#[derive(Debug, PartialEq, Clone, Serialize, Hash, Eq)]
pub enum RuleViolationKind {
    RequireConcurrentIndexCreation,
    ConstraintMissingNotValid,
    AddingFieldWithDefault,
    ChangingColumnType,
    AddingNotNullableField,
    RenamingColumn,
    RenamingTable,
    DisallowedUniqueConstraint,
    BanDropDatabase,
    PreferTextField,
    PreferRobustStmts,
}

impl std::fmt::Display for RuleViolationKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_value = match self {
            Self::RequireConcurrentIndexCreation => "require-concurrent-index-creation",
            Self::ConstraintMissingNotValid => "constraint-missing-not-valid",
            Self::AddingFieldWithDefault => "adding-field-with-default",
            Self::ChangingColumnType => "changing-column-type",
            Self::AddingNotNullableField => "adding-not-nullable-field",
            Self::RenamingColumn => "renaming-column",
            Self::RenamingTable => "renaming-table",
            Self::DisallowedUniqueConstraint => "disallowed-unique-constraint",
            Self::BanDropDatabase => "ban-drop-database",
            Self::PreferTextField => "prefer-text-field",
            Self::PreferRobustStmts => "prefer-robust-stmts",
        };
        write!(f, "{}", str_value)
    }
}

impl std::convert::TryFrom<&str> for RuleViolationKind {
    type Error = ();
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "require-concurrent-index-creation" => Ok(Self::RequireConcurrentIndexCreation),
            "constraint-missing-not-valid" => Ok(Self::ConstraintMissingNotValid),
            "adding-field-with-default" => Ok(Self::AddingFieldWithDefault),
            "changing-column-type" => Ok(Self::ChangingColumnType),
            "adding-not-nullable-field" => Ok(Self::AddingNotNullableField),
            "renaming-column" => Ok(Self::RenamingColumn),
            "renaming-table" => Ok(Self::RenamingTable),
            "disallowed-unique-constraint" => Ok(Self::DisallowedUniqueConstraint),
            "ban-drop-database" => Ok(Self::BanDropDatabase),
            "prefer-text-field" => Ok(Self::PreferTextField),
            "prefer-robust-stmts" => Ok(Self::PreferRobustStmts),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Span {
    pub start: i32,
    pub len: Option<i32>,
}

#[derive(Debug, PartialEq, Serialize, Clone)]
pub enum ViolationMessage {
    Note(String),
    Help(String),
}

#[derive(Debug, PartialEq)]
pub struct RuleViolation {
    pub kind: RuleViolationKind,
    pub span: Span,
    pub messages: Vec<ViolationMessage>,
}

impl RuleViolation {
    fn new(
        kind: RuleViolationKind,
        node: &RawStmt,
        messages: Option<Vec<ViolationMessage>>,
    ) -> Self {
        let messages = messages.unwrap_or_else(|| {
            RULES
                .iter()
                .find(|r| r.name == kind)
                .unwrap()
                .messages
                .clone()
        });
        Self {
            kind,
            span: Span {
                start: node.stmt_location,
                len: node.stmt_len,
            },
            messages,
        }
    }
}

pub fn require_concurrent_index_creation(tree: &[RootStmt]) -> Vec<RuleViolation> {
    let tables_created = tables_created_in_transaction(tree);
    let mut errs = vec![];
    for RootStmt::RawStmt(raw_stmt) in tree {
        match &raw_stmt.stmt {
            Stmt::IndexStmt(stmt) => {
                let RelationKind::RangeVar(range) = &stmt.relation;
                let tbl_name = &range.relname;
                if !stmt.concurrent && !tables_created.contains(tbl_name) {
                    errs.push(RuleViolation::new(
                        RuleViolationKind::RequireConcurrentIndexCreation,
                        raw_stmt,
                        None,
                    ));
                }
            }
            _ => continue,
        }
    }
    errs
}

pub fn renaming_column(tree: &[RootStmt]) -> Vec<RuleViolation> {
    let mut errs = vec![];
    for RootStmt::RawStmt(raw_stmt) in tree {
        match &raw_stmt.stmt {
            Stmt::RenameStmt(stmt) => match stmt.rename_type {
                ObjectType::Column => {
                    errs.push(RuleViolation::new(
                        RuleViolationKind::RenamingColumn,
                        raw_stmt,
                        None,
                    ));
                }
                _ => continue,
            },
            _ => continue,
        }
    }
    errs
}

pub fn renaming_table(tree: &[RootStmt]) -> Vec<RuleViolation> {
    let mut errs = vec![];
    for RootStmt::RawStmt(raw_stmt) in tree {
        match &raw_stmt.stmt {
            Stmt::RenameStmt(stmt) => match stmt.rename_type {
                ObjectType::Table => {
                    errs.push(RuleViolation::new(
                        RuleViolationKind::RenamingTable,
                        raw_stmt,
                        None,
                    ));
                }
                _ => continue,
            },
            _ => continue,
        }
    }
    errs
}

pub fn changing_column_type(tree: &[RootStmt]) -> Vec<RuleViolation> {
    let mut errs = vec![];
    for RootStmt::RawStmt(raw_stmt) in tree {
        match &raw_stmt.stmt {
            Stmt::AlterTableStmt(stmt) => {
                for AlterTableCmds::AlterTableCmd(cmd) in &stmt.cmds {
                    if cmd.subtype == AlterTableType::AlterColumnType {
                        errs.push(RuleViolation::new(
                            RuleViolationKind::ChangingColumnType,
                            raw_stmt,
                            None,
                        ));
                    }
                }
            }
            _ => continue,
        }
    }
    errs
}

pub fn adding_not_nullable_field(tree: &[RootStmt]) -> Vec<RuleViolation> {
    let mut errs = vec![];
    for RootStmt::RawStmt(raw_stmt) in tree {
        match &raw_stmt.stmt {
            Stmt::AlterTableStmt(stmt) => {
                for AlterTableCmds::AlterTableCmd(cmd) in &stmt.cmds {
                    if cmd.subtype == AlterTableType::AddColumn {
                        if let Some(AlterTableDef::ColumnDef(column_def)) = &cmd.def {
                            for ColumnDefConstraint::Constraint(constraint) in
                                &column_def.constraints
                            {
                                if constraint.contype == ConstrType::NotNull {
                                    errs.push(RuleViolation::new(
                                        RuleViolationKind::AddingNotNullableField,
                                        raw_stmt,
                                        None,
                                    ));
                                }
                            }
                        }
                    }
                }
            }
            _ => continue,
        }
    }
    errs
}

pub fn adding_field_with_default(tree: &[RootStmt]) -> Vec<RuleViolation> {
    let mut errs = vec![];
    for RootStmt::RawStmt(raw_stmt) in tree {
        match &raw_stmt.stmt {
            Stmt::AlterTableStmt(stmt) => {
                for AlterTableCmds::AlterTableCmd(cmd) in &stmt.cmds {
                    match &cmd.def {
                        Some(AlterTableDef::ColumnDef(def)) => {
                            for ColumnDefConstraint::Constraint(constraint) in &def.constraints {
                                if constraint.contype == ConstrType::Default {
                                    errs.push(RuleViolation::new(
                                        RuleViolationKind::AddingFieldWithDefault,
                                        raw_stmt,
                                        None,
                                    ));
                                }
                            }
                        }
                        _ => continue,
                    }
                }
            }
            _ => continue,
        }
    }
    errs
}

pub fn disallow_unique_constraint(tree: &[RootStmt]) -> Vec<RuleViolation> {
    let tables_created = tables_created_in_transaction(tree);
    let mut errs = vec![];
    for RootStmt::RawStmt(raw_stmt) in tree {
        match &raw_stmt.stmt {
            Stmt::AlterTableStmt(stmt) => {
                let RelationKind::RangeVar(range) = &stmt.relation;
                let tbl_name = &range.relname;
                for AlterTableCmds::AlterTableCmd(cmd) in &stmt.cmds {
                    match &cmd.def {
                        Some(AlterTableDef::Constraint(constraint)) => {
                            if !tables_created.contains(tbl_name)
                                && constraint.contype == ConstrType::Unique
                                && constraint.indexname.is_none()
                            {
                                errs.push(RuleViolation::new(
                                    RuleViolationKind::DisallowedUniqueConstraint,
                                    raw_stmt,
                                    None,
                                ));
                            }
                        }
                        _ => continue,
                    }
                }
            }
            _ => continue,
        }
    }
    errs
}

fn tables_created_in_transaction(tree: &[RootStmt]) -> HashSet<String> {
    let mut created_table_names = HashSet::new();
    let mut inside_transaction = false;
    for RootStmt::RawStmt(raw_stmt) in tree {
        match &raw_stmt.stmt {
            Stmt::TransactionStmt(stmt) => match stmt.kind {
                TransactionStmtKind::Begin => inside_transaction = true,
                TransactionStmtKind::Commit => inside_transaction = false,
                _ => continue,
            },
            Stmt::CreateStmt(stmt) if inside_transaction => {
                let RelationKind::RangeVar(stmt) = &stmt.relation;
                let table_name = &stmt.relname;
                created_table_names.insert(table_name.to_owned());
            }
            _ => continue,
        }
    }
    created_table_names
}

pub fn constraint_missing_not_valid(tree: &[RootStmt]) -> Vec<RuleViolation> {
    let mut errs = vec![];
    let tables_created = tables_created_in_transaction(tree);
    for RootStmt::RawStmt(raw_stmt) in tree {
        match &raw_stmt.stmt {
            Stmt::AlterTableStmt(stmt) => {
                let RelationKind::RangeVar(range) = &stmt.relation;
                let tbl_name = &range.relname;
                for AlterTableCmds::AlterTableCmd(cmd) in &stmt.cmds {
                    match &cmd.def {
                        Some(AlterTableDef::Constraint(constraint)) => {
                            if !tables_created.contains(tbl_name) && constraint.initially_valid {
                                errs.push(RuleViolation::new(
                                    RuleViolationKind::ConstraintMissingNotValid,
                                    raw_stmt,
                                    None,
                                ));
                            }
                        }
                        _ => continue,
                    }
                }
            }
            _ => continue,
        }
    }
    errs
}

/// Brad's Rule aka ban dropping database statements.
pub fn ban_drop_database(tree: &[RootStmt]) -> Vec<RuleViolation> {
    let mut errs = vec![];
    for RootStmt::RawStmt(raw_stmt) in tree {
        match &raw_stmt.stmt {
            Stmt::DropdbStmt(_) => {
                errs.push(RuleViolation::new(
                    RuleViolationKind::BanDropDatabase,
                    raw_stmt,
                    None,
                ));
            }
            _ => continue,
        }
    }
    errs
}

/// It's easier to update the check constraint on a text field than a varchar()
/// size since the check constraint can use NOT VALID with a separate VALIDATE
/// call.
pub fn prefer_text_field(tree: &[RootStmt]) -> Vec<RuleViolation> {
    let mut errs = vec![];
    for RootStmt::RawStmt(raw_stmt) in tree {
        match &raw_stmt.stmt {
            Stmt::CreateStmt(stmt) => {
                for TableElt::ColumnDef(column_def) in &stmt.table_elts {
                    let ColumnDefTypeName::TypeName(type_name) = &column_def.type_name;
                    for QualifiedName::String(field_type_name) in &type_name.names {
                        if field_type_name.str == "varchar" {
                            errs.push(RuleViolation::new(
                                RuleViolationKind::PreferTextField,
                                raw_stmt,
                                None,
                            ));
                        }
                    }
                }
            }
            _ => continue,
        }
    }
    errs
}

/// If a migration is running in a transaction, then we skip the statements
/// because if it fails part way through, it will revert.
/// For the cases where statements aren't running in a transaction, for instance,
/// when we CREATE INDEX CONCURRENTLY, we should try and make those migrations
/// more robust by using guards like `IF NOT EXISTS`. So if the migration fails
/// halfway through, it can be rerun without human intervention.
pub fn prefer_robust_stmts(tree: &[RootStmt]) -> Vec<RuleViolation> {
    let mut errs = vec![];
    let mut inside_transaction = false;
    for RootStmt::RawStmt(raw_stmt) in tree {
        match &raw_stmt.stmt {
            Stmt::TransactionStmt(stmt) => match stmt.kind {
                TransactionStmtKind::Begin => inside_transaction = true,
                TransactionStmtKind::Commit => inside_transaction = false,
                _ => continue,
            },
            Stmt::AlterTableStmt(stmt) => {
                for AlterTableCmds::AlterTableCmd(cmd) in &stmt.cmds {
                    if cmd.missing_ok || inside_transaction {
                        continue;
                    }
                    errs.push(RuleViolation::new(
                        RuleViolationKind::PreferRobustStmts,
                        raw_stmt,
                        None,
                    ));
                }
            }
            Stmt::IndexStmt(stmt) if !stmt.if_not_exists && !inside_transaction => {
                errs.push(RuleViolation::new(
                    RuleViolationKind::PreferRobustStmts,
                    raw_stmt,
                    None,
                ));
            }
            Stmt::CreateStmt(stmt) if !stmt.if_not_exists && !inside_transaction => {
                errs.push(RuleViolation::new(
                    RuleViolationKind::PreferRobustStmts,
                    raw_stmt,
                    None,
                ));
            }
            _ => continue,
        }
    }
    errs
}

#[derive(Debug, PartialEq)]
pub enum CheckSQLError {
    ParsingSQL(PGQueryError),
}

impl std::convert::From<PGQueryError> for CheckSQLError {
    fn from(err: PGQueryError) -> Self {
        Self::ParsingSQL(err)
    }
}

pub struct SquawkRule {
    pub name: RuleViolationKind,
    func: fn(&[RootStmt]) -> Vec<RuleViolation>,
    pub messages: Vec<ViolationMessage>,
}

lazy_static! {
    pub static ref RULES: Vec<SquawkRule> = vec![
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
        // see ChangingColumnType
        SquawkRule {
            name: RuleViolationKind::AddingFieldWithDefault,
            func: adding_field_with_default,
            messages: vec![
                ViolationMessage::Note(
                    "In Postgres versions <11 adding a field with a DEFAULT requires a table rewrite with an ACCESS EXCLUSIVE lock.".into(),
                ),
                ViolationMessage::Help(
                    "Add the field as nullable, then set a default, backfill, and remove nullabilty.".into(),
                ),

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
                ViolationMessage::Help("Add NOT VALID to the constraint and then VALIDATE the constraint.".into()),
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
            name: RuleViolationKind::PreferRobustStmts,
            func: prefer_robust_stmts,
            messages: vec![
                ViolationMessage::Help(
                    "Consider wrapping in a transaction or adding a IF NOT EXISTS clause.".into()
                ),
            ]
        }
    ];
}

pub fn check_sql(
    sql: &str,
    excluded_rules: &[String],
) -> Result<Vec<RuleViolation>, CheckSQLError> {
    let tree = parse_sql_query(sql)?;

    let excluded_rules: HashSet<RuleViolationKind> = excluded_rules
        .iter()
        .flat_map(|s| RuleViolationKind::try_from(s.as_ref()).ok())
        .collect();

    let mut errs = vec![];
    for rule in RULES.iter().filter(|r| !excluded_rules.contains(&r.name)) {
        errs.extend((rule.func)(&tree));
    }

    errs.sort_by_key(|v| v.span.start);

    Ok(errs)
}

#[cfg(test)]
mod test_rules {
    use super::*;

    use insta::assert_debug_snapshot;

    /// Ensure we handle both serializing and deserializing RuleViolationKind
    #[test]
    fn test_parsing_rule_kind() {
        let rule_names = RULES.iter().map(|r| r.name.clone());
        for rule in rule_names {
            assert_eq!(
                RuleViolationKind::try_from(rule.to_string().as_ref()),
                Ok(rule)
            );
        }
    }

    /// Ensure we stort the resulting violations by where they occur in the file.
    #[test]
    fn test_check_rules_orderin() {
        let sql = r#"
  ALTER TABLE "table_name" RENAME COLUMN "column_name" TO "new_column_name";
  CREATE INDEX "field_name_idx" ON "table_name" ("field_name");
  "#;

        let res = check_sql(sql, &[]).expect("valid parsing of SQL");
        let mut prev_span_start = -1;
        for violation in res.iter() {
            assert!(violation.span.start > prev_span_start);
            prev_span_start = violation.span.start;
        }
    }

    /// ```sql
    /// -- instead of
    /// CREATE INDEX "field_name_idx" ON "table_name" ("field_name");
    /// -- use CONCURRENTLY
    /// CREATE INDEX CONCURRENTLY "field_name_idx" ON "table_name" ("field_name");
    /// ```
    #[test]
    fn test_adding_index_non_concurrently() {
        let bad_sql = r#"
  -- instead of
  CREATE INDEX "field_name_idx" ON "table_name" ("field_name");
  "#;

        assert_debug_snapshot!(check_sql(bad_sql, &[]));

        let ok_sql = r#"
  -- use CONCURRENTLY
  CREATE INDEX CONCURRENTLY "field_name_idx" ON "table_name" ("field_name");
  "#;
        assert_debug_snapshot!(check_sql(ok_sql, &[]));
    }

    /// ```sql
    /// -- instead of
    /// ALTER TABLE distributors ADD CONSTRAINT distfk FOREIGN KEY (address) REFERENCES addresses (address);
    /// -- use `NOT VALID`
    /// ALTER TABLE distributors ADD CONSTRAINT distfk FOREIGN KEY (address) REFERENCES addresses (address) NOT VALID;
    /// ALTER TABLE distributors VALIDATE CONSTRAINT distfk;
    /// ```
    #[test]
    fn test_adding_foreign_key() {
        let bad_sql = r#"
-- instead of
ALTER TABLE distributors ADD CONSTRAINT distfk FOREIGN KEY (address) REFERENCES addresses (address);
   "#;

        assert_debug_snapshot!(check_sql(bad_sql, &[]));

        let ok_sql = r#"
-- use `NOT VALID`
ALTER TABLE distributors ADD CONSTRAINT distfk FOREIGN KEY (address) REFERENCES addresses (address) NOT VALID;
ALTER TABLE distributors VALIDATE CONSTRAINT distfk;
   "#;
        assert_debug_snapshot!(check_sql(ok_sql, &[]));
    }

    ///
    /// ```sql
    /// -- instead of
    /// ALTER TABLE "accounts" ADD CONSTRAINT "positive_balance" CHECK ("balance" >= 0);
    ///
    /// -- use `NOT VALID`
    /// ALTER TABLE "accounts" ADD CONSTRAINT "positive_balance" CHECK ("balance" >= 0) NOT VALID;
    /// ALTER TABLE accounts VALIDATE CONSTRAINT positive_balance;
    /// ```
    #[test]
    fn test_adding_check_constraint() {
        let bad_sql = r#"
-- instead of
ALTER TABLE "accounts" ADD CONSTRAINT "positive_balance" CHECK ("balance" >= 0);
   "#;

        let ok_sql = r#"
-- use `NOT VALID`
ALTER TABLE "accounts" ADD CONSTRAINT "positive_balance" CHECK ("balance" >= 0) NOT VALID;
ALTER TABLE accounts VALIDATE CONSTRAINT positive_balance;
   "#;

        assert_debug_snapshot!(check_sql(bad_sql, &[]));

        assert_debug_snapshot!(check_sql(ok_sql, &[]));
    }

    /// ```sql
    /// -- instead of
    /// ALTER TABLE table_name ADD CONSTRAINT field_name_constraint UNIQUE (field_name);
    /// -- also works with PRIMARY KEY
    /// -- use:
    /// -- To recreate a primary key constraint, without blocking updates while the index is rebuilt:
    /// CREATE UNIQUE INDEX CONCURRENTLY dist_id_temp_idx ON distributors (dist_id);
    /// ALTER TABLE distributors DROP CONSTRAINT distributors_pkey,
    /// ADD CONSTRAINT distributors_pkey PRIMARY KEY USING INDEX dist_id_temp_idx;
    /// ```
    #[test]
    fn test_adding_unique_constraint() {
        let bad_sql = r#"
ALTER TABLE table_name ADD CONSTRAINT field_name_constraint UNIQUE (field_name);
   "#;

        let ok_sql = r#"
CREATE UNIQUE INDEX CONCURRENTLY dist_id_temp_idx ON distributors (dist_id);
ALTER TABLE distributors DROP CONSTRAINT distributors_pkey,
ADD CONSTRAINT distributors_pkey PRIMARY KEY USING INDEX dist_id_temp_idx;
   "#;

        assert_debug_snapshot!(check_sql(bad_sql, &[]));
        assert_debug_snapshot!(check_sql(ok_sql, &[]));
    }

    /// Creating a UNQIUE constraint from an existing index should be considered
    /// safe
    #[test]
    fn test_unique_constraint_ok() {
        let sql = r#"
CREATE UNIQUE INDEX CONCURRENTLY "legacy_questiongrouppg_mongo_id_1f8f47d9_uniq_idx" 
    ON "legacy_questiongrouppg" ("mongo_id");
ALTER TABLE "legacy_questiongrouppg" ADD CONSTRAINT "legacy_questiongrouppg_mongo_id_1f8f47d9_uniq" UNIQUE USING INDEX "legacy_questiongrouppg_mongo_id_1f8f47d9_uniq_idx";
        "#;
        assert_eq!(check_sql(sql, &[]), Ok(vec![]));
    }

    ///
    /// ```sql
    /// -- instead of
    /// ALTER TABLE "core_recipe" ADD COLUMN "foo" integer DEFAULT 10;
    /// -- use
    /// ALTER TABLE "core_recipe" ADD COLUMN "foo" integer;
    /// ALTER TABLE "core_recipe" ALTER COLUMN "foo" SET DEFAULT 10;
    /// -- backfill
    /// -- remove nullability
    /// ```
    #[test]
    fn test_adding_field_with_default() {
        let bad_sql = r#"
-- instead of
ALTER TABLE "core_recipe" ADD COLUMN "foo" integer DEFAULT 10;
"#;

        let ok_sql = r#"
-- use
ALTER TABLE "core_recipe" ADD COLUMN "foo" integer;
ALTER TABLE "core_recipe" ALTER COLUMN "foo" SET DEFAULT 10;
-- backfill
-- remove nullability
        "#;

        assert_debug_snapshot!(check_sql(bad_sql, &[]));
        assert_debug_snapshot!(check_sql(ok_sql, &[]));
    }

    #[test]
    fn test_changing_field_type() {
        let bad_sql = r#"
BEGIN;
--
-- Alter field edits on recipe
--
ALTER TABLE "core_recipe" ALTER COLUMN "edits" TYPE text USING "edits"::text;
COMMIT;
        "#;
        assert_debug_snapshot!(check_sql(bad_sql, &[]));

        let bad_sql = r#"
BEGIN;
--
-- Alter field foo on recipe
--
ALTER TABLE "core_recipe" ALTER COLUMN "foo" TYPE varchar(255) USING "foo"::varchar(255);
ALTER TABLE "core_recipe" ALTER COLUMN "foo" TYPE text USING "foo"::text;
COMMIT;
        "#;

        assert_debug_snapshot!(check_sql(bad_sql, &[]));
    }

    #[test]
    fn test_adding_field_that_is_not_nullable() {
        let bad_sql = r#"
BEGIN;
--
-- Add field foo to recipe
--
ALTER TABLE "core_recipe" ADD COLUMN "foo" integer DEFAULT 10 NOT NULL;
ALTER TABLE "core_recipe" ALTER COLUMN "foo" DROP DEFAULT;
COMMIT;
        "#;

        assert_debug_snapshot!(check_sql(bad_sql, &[]));

        let bad_sql = r#"
-- not sure how this would ever work, but might as well test it
ALTER TABLE "core_recipe" ADD COLUMN "foo" integer NOT NULL;
        "#;

        assert_debug_snapshot!(check_sql(bad_sql, &[]));
    }

    #[test]
    fn test_renaming_column() {
        let sql = r#"
ALTER TABLE "table_name" RENAME COLUMN "column_name" TO "new_column_name";
        "#;

        assert_debug_snapshot!(check_sql(sql, &[]));
    }

    #[test]
    fn test_renaming_table() {
        let sql = r#"
ALTER TABLE "table_name" RENAME TO "new_table_name";
        "#;

        assert_debug_snapshot!(check_sql(sql, &[]));
    }

    #[test]
    fn test_ban_drop_database() {
        let sql = r#"
DROP DATABASE "table_name";
DROP DATABASE IF EXISTS "table_name";
DROP DATABASE IF EXISTS "table_name"
        "#;
        assert_debug_snapshot!(check_sql(sql, &[]));
    }

    #[test]
    fn test_ensure_ignored_when_new_table() {
        let sql = r#"
BEGIN;
CREATE TABLE "core_foo" (
  "id" serial NOT NULL PRIMARY KEY, 
  "created" timestamp with time zone NOT NULL, 
  "modified" timestamp with time zone NOT NULL, 
  "mongo_id" varchar(255) NOT NULL UNIQUE, 
  "description" text NOT NULL, 
  "metadata" jsonb NOT NULL, 
  "kind" varchar(255) NOT NULL, 
  "age" integer NOT NULL, 
  "tenant_id" integer NULL
);
CREATE INDEX "age_index" ON "core_foo" ("age");
ALTER TABLE "core_foo" ADD CONSTRAINT "age_restriction" CHECK ("age" >= 25);
ALTER TABLE "core_foo" ADD CONSTRAINT "core_foo_tenant_id_4d397ef9_fk_core_myuser_id" 
    FOREIGN KEY ("tenant_id") REFERENCES "core_myuser" ("id") 
    DEFERRABLE INITIALLY DEFERRED;
CREATE INDEX "core_foo_mongo_id_1c1a7e39_like" ON "core_foo" ("mongo_id" varchar_pattern_ops);
CREATE INDEX "core_foo_tenant_id_4d397ef9" ON "core_foo" ("tenant_id");
COMMIT;
        "#;

        assert_debug_snapshot!(check_sql(
            sql,
            &[RuleViolationKind::PreferTextField.to_string()]
        ));
    }

    /// Changing a column of varchar(255) to varchar(1000) requires an ACCESS
    /// EXCLUSIVE lock
    #[test]
    fn test_increasing_varchar_size() {
        let sql = r#"
BEGIN;
--
-- Alter field kind on foo
--
ALTER TABLE "core_foo" ALTER COLUMN "kind" TYPE varchar(1000) USING "kind"::varchar(1000);
COMMIT;
"#;
        assert_debug_snapshot!(check_sql(sql, &[]), @r###"
        Ok(
            [
                RuleViolation {
                    kind: ChangingColumnType,
                    span: Span {
                        start: 7,
                        len: Some(
                            123,
                        ),
                    },
                    messages: [
                        Note(
                            "Requires an ACCESS EXCLUSIVE lock on the table which blocks reads.",
                        ),
                        Note(
                            "Changing the type may break existing clients.",
                        ),
                    ],
                },
            ],
        )
        "###);
    }

    #[test]
    fn test_prefer_text_field() {
        let bad_sql = r#"
BEGIN;
--
-- Create model Bar
--
CREATE TABLE "core_bar" (
    "id" serial NOT NULL PRIMARY KEY, 
    "alpha" varchar(100) NOT NULL
);
COMMIT;
"#;
        assert_debug_snapshot!(check_sql(bad_sql, &[]), @r###"
        Ok(
            [
                RuleViolation {
                    kind: PreferTextField,
                    span: Span {
                        start: 7,
                        len: Some(
                            127,
                        ),
                    },
                    messages: [
                        Note(
                            "Changing the size of a varchar field requires an ACCESS EXCLUSIVE lock.",
                        ),
                        Help(
                            "Use a text field with a check constraint.",
                        ),
                    ],
                },
            ],
        )
        "###);

        let ok_sql = r#"
BEGIN;
--
-- Create model Bar
--
CREATE TABLE "core_bar" (
    "id" serial NOT NULL PRIMARY KEY, 
    "bravo" text NOT NULL
);
--
-- Create constraint text_size on model bar
--
ALTER TABLE "core_bar" ADD CONSTRAINT "text_size" CHECK (LENGTH("bravo") <= 100);
COMMIT;"#;
        assert_debug_snapshot!(check_sql(ok_sql, &[]), @r###"
        Ok(
            [],
        )
        "###);
    }

    /// If the statement is in a transaction, or it has a guard like IF NOT
    /// EXISTS, then it is considered valid by the `prefer-robust-stmt` rule.
    #[test]
    fn test_prefer_robust_stmt_okay_cases() {
        let sql = r#"
BEGIN;
ALTER TABLE "core_foo" ADD COLUMN "answer_id" integer NULL;
COMMIT;
"#;
        assert_eq!(check_sql(sql, &[]), Ok(vec![]));

        let sql = r#"
ALTER TABLE "core_foo" ADD COLUMN IF NOT EXISTS "answer_id" integer NULL;
"#;
        assert_eq!(check_sql(sql, &[]), Ok(vec![]));

        let sql = r#"
CREATE INDEX CONCURRENTLY IF NOT EXISTS "core_foo_idx" ON "core_foo" ("answer_id");
"#;
        assert_eq!(check_sql(sql, &[]), Ok(vec![]));

        let sql = r#"
BEGIN;
CREATE TABLE "core_bar" (
    "id" serial NOT NULL PRIMARY KEY,
    "bravo" text NOT NULL
);
COMMIT;
"#;
        assert_eq!(check_sql(sql, &[]), Ok(vec![]));

        let sql = r#"
CREATE TABLE IF NOT EXISTS "core_bar" (
    "id" serial NOT NULL PRIMARY KEY,
    "bravo" text NOT NULL
);
"#;
        assert_eq!(check_sql(sql, &[]), Ok(vec![]));

        // select is fine, we're only interested in modifications to the tables
        let sql = r#"
SELECT 1;
"#;
        assert_eq!(check_sql(sql, &[]), Ok(vec![]));

        // inserts are also okay
        let sql = r#"
INSERT INTO tbl VALUES (a);
"#;
        assert_eq!(check_sql(sql, &[]), Ok(vec![]));

        let sql = r#"
ALTER TABLE "core_foo" DROP CONSTRAINT IF EXISTS "core_foo_idx";
        "#;
        assert_eq!(check_sql(sql, &[]), Ok(vec![]));
    }

    #[test]
    fn test_prefer_robust_stmt_failure_cases() {
        let sql = r#"
ALTER TABLE "core_foo" ADD COLUMN "answer_id" integer NULL;
"#;
        assert_debug_snapshot!(check_sql(sql, &[]), @r###"
        Ok(
            [
                RuleViolation {
                    kind: PreferRobustStmts,
                    span: Span {
                        start: 0,
                        len: Some(
                            59,
                        ),
                    },
                    messages: [
                        Help(
                            "Consider wrapping in a transaction or adding a IF NOT EXISTS clause.",
                        ),
                    ],
                },
            ],
        )
        "###);

        let sql = r#"
CREATE INDEX CONCURRENTLY "core_foo_idx" ON "core_foo" ("answer_id");
"#;
        assert_debug_snapshot!(check_sql(sql, &[]), @r###"
        Ok(
            [
                RuleViolation {
                    kind: PreferRobustStmts,
                    span: Span {
                        start: 0,
                        len: Some(
                            69,
                        ),
                    },
                    messages: [
                        Help(
                            "Consider wrapping in a transaction or adding a IF NOT EXISTS clause.",
                        ),
                    ],
                },
            ],
        )
        "###);

        let sql = r#"
CREATE TABLE "core_bar" ( "id" serial NOT NULL PRIMARY KEY, "bravo" text NOT NULL);
"#;
        assert_debug_snapshot!(check_sql(sql, &[]), @r###"
        Ok(
            [
                RuleViolation {
                    kind: PreferRobustStmts,
                    span: Span {
                        start: 0,
                        len: Some(
                            83,
                        ),
                    },
                    messages: [
                        Help(
                            "Consider wrapping in a transaction or adding a IF NOT EXISTS clause.",
                        ),
                    ],
                },
            ],
        )
        "###);

        let sql = r#"
ALTER TABLE "core_foo" DROP CONSTRAINT "core_foo_idx";
        "#;
        assert_debug_snapshot!(check_sql(sql, &[]), @r###"
        Ok(
            [
                RuleViolation {
                    kind: PreferRobustStmts,
                    span: Span {
                        start: 0,
                        len: Some(
                            54,
                        ),
                    },
                    messages: [
                        Help(
                            "Consider wrapping in a transaction or adding a IF NOT EXISTS clause.",
                        ),
                    ],
                },
            ],
        )
        "###);
    }
}
