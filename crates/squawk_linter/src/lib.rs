use std::collections::HashSet;
use std::fmt;

use enum_iterator::all;
use enum_iterator::Sequence;
use ignore::find_ignores;
pub use ignore::Ignore;
use ignore_index::IgnoreIndex;
use lazy_static::lazy_static;
use rowan::TextRange;
use serde::{Deserialize, Serialize};

use squawk_syntax::{Parse, SourceFile};

mod ignore;
mod ignore_index;

mod rules;
mod text;
use rules::adding_field_with_default;
use rules::adding_foreign_key_constraint;
use rules::adding_not_null_field;
use rules::adding_primary_key_constraint;
use rules::adding_required_field;
use rules::ban_alter_domain_with_add_constraint;
use rules::ban_char_field;
use rules::ban_concurrent_index_creation_in_transaction;
use rules::ban_create_domain_with_constraint;
use rules::ban_drop_column;
use rules::ban_drop_not_null;
use rules::ban_drop_table;
use rules::changing_column_type;
use rules::constraint_missing_not_valid;
use rules::disallow_unique_constraint;
use rules::prefer_big_int;
use rules::prefer_bigint_over_int;
use rules::prefer_bigint_over_smallint;
use rules::prefer_identity;
use rules::prefer_robust_stmts;
use rules::prefer_text_field;
use rules::prefer_timestamptz;
use rules::renaming_column;
use rules::renaming_table;
use rules::require_concurrent_index_creation;
use rules::require_concurrent_index_deletion;
// xtask:new-lint:rule-import

use rules::ban_drop_database;

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Hash, Eq, Deserialize)]
pub enum ErrorCode {
    #[serde(rename = "require-concurrent-index-creation")]
    RequireConcurrentIndexCreation,
    #[serde(rename = "require-concurrent-index-deletion")]
    RequireConcurrentIndexDeletion,
    #[serde(rename = "constraint-missing-not-valid")]
    ConstraintMissingNotValid,
    #[serde(rename = "adding-field-with-default")]
    AddingFieldWithDefault,
    #[serde(rename = "adding-foreign-key-constraint")]
    AddingForeignKeyConstraint,
    #[serde(rename = "changing-column-type")]
    ChangingColumnType,
    #[serde(rename = "adding-not-nullable-field")]
    AddingNotNullableField,
    #[serde(rename = "adding-serial-primary-key-field")]
    AddingSerialPrimaryKeyField,
    #[serde(rename = "renaming-column")]
    RenamingColumn,
    #[serde(rename = "renaming-table")]
    RenamingTable,
    #[serde(rename = "disallowed-unique-constraint")]
    DisallowedUniqueConstraint,
    #[serde(rename = "ban-drop-database")]
    BanDropDatabase,
    #[serde(rename = "prefer-big-int")]
    PreferBigInt,
    #[serde(rename = "prefer-bigint-over-int")]
    PreferBigintOverInt,
    #[serde(rename = "prefer-bigint-over-smallint")]
    PreferBigintOverSmallint,
    #[serde(rename = "prefer-identity")]
    PreferIdentity,
    #[serde(rename = "prefer-robust-stmts")]
    PreferRobustStmts,
    #[serde(rename = "prefer-text-field")]
    PreferTextField,
    #[serde(rename = "prefer-timestamptz")]
    PreferTimestampTz,
    #[serde(rename = "ban-char-field")]
    BanCharField,
    #[serde(rename = "ban-drop-column")]
    BanDropColumn,
    #[serde(rename = "ban-drop-table")]
    BanDropTable,
    #[serde(rename = "ban-drop-not-null")]
    BanDropNotNull,
    #[serde(rename = "transaction-nesting")]
    TransactionNesting,
    #[serde(rename = "adding-required-field")]
    AddingRequiredField,
    #[serde(rename = "ban-concurrent-index-creation-in-transaction")]
    BanConcurrentIndexCreationInTransaction,
    #[serde(rename = "unused-ignore")]
    UnusedIgnore,
    #[serde(rename = "ban-create-domain-with-constraint")]
    BanCreateDomainWithConstraint,
    #[serde(rename = "ban-alter-domain-with-add-constraint")]
    BanAlterDomainWithAddConstraint,
    // xtask:new-lint:error-name
}

impl TryFrom<&str> for ErrorCode {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "require-concurrent-index-creation" => Ok(ErrorCode::RequireConcurrentIndexCreation),
            "require-concurrent-index-deletion" => Ok(ErrorCode::RequireConcurrentIndexDeletion),
            "constraint-missing-not-valid" => Ok(ErrorCode::ConstraintMissingNotValid),
            "adding-field-with-default" => Ok(ErrorCode::AddingFieldWithDefault),
            "adding-foreign-key-constraint" => Ok(ErrorCode::AddingForeignKeyConstraint),
            "changing-column-type" => Ok(ErrorCode::ChangingColumnType),
            "adding-not-nullable-field" => Ok(ErrorCode::AddingNotNullableField),
            "adding-serial-primary-key-field" => Ok(ErrorCode::AddingSerialPrimaryKeyField),
            "renaming-column" => Ok(ErrorCode::RenamingColumn),
            "renaming-table" => Ok(ErrorCode::RenamingTable),
            "disallowed-unique-constraint" => Ok(ErrorCode::DisallowedUniqueConstraint),
            "ban-drop-database" => Ok(ErrorCode::BanDropDatabase),
            "prefer-big-int" => Ok(ErrorCode::PreferBigInt),
            "prefer-bigint-over-int" => Ok(ErrorCode::PreferBigintOverInt),
            "prefer-bigint-over-smallint" => Ok(ErrorCode::PreferBigintOverSmallint),
            "prefer-identity" => Ok(ErrorCode::PreferIdentity),
            "prefer-robust-stmts" => Ok(ErrorCode::PreferRobustStmts),
            "prefer-text-field" => Ok(ErrorCode::PreferTextField),
            "prefer-timestamptz" => Ok(ErrorCode::PreferTimestampTz),
            "ban-char-field" => Ok(ErrorCode::BanCharField),
            "ban-drop-column" => Ok(ErrorCode::BanDropColumn),
            "ban-drop-table" => Ok(ErrorCode::BanDropTable),
            "ban-drop-not-null" => Ok(ErrorCode::BanDropNotNull),
            "transaction-nesting" => Ok(ErrorCode::TransactionNesting),
            "adding-required-field" => Ok(ErrorCode::AddingRequiredField),
            "ban-concurrent-index-creation-in-transaction" => {
                Ok(ErrorCode::BanConcurrentIndexCreationInTransaction)
            }
            "ban-create-domain-with-constraint" => Ok(ErrorCode::BanCreateDomainWithConstraint),
            "ban-alter-domain-with-add-constraint" => {
                Ok(ErrorCode::BanAlterDomainWithAddConstraint)
            }
            // xtask:new-lint:str-name
            _ => Err(format!("Unknown violation name: {}", s)),
        }
    }
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = match &self {
            ErrorCode::RequireConcurrentIndexCreation => "require-concurrent-index-creation",
            ErrorCode::RequireConcurrentIndexDeletion => "require-concurrent-index-deletion",
            ErrorCode::ConstraintMissingNotValid => "constraint-missing-not-valid",
            ErrorCode::AddingFieldWithDefault => "adding-field-with-default",
            ErrorCode::AddingForeignKeyConstraint => "adding-foreign-key-constraint",
            ErrorCode::ChangingColumnType => "changing-column-type",
            ErrorCode::AddingNotNullableField => "adding-not-nullable-field",
            ErrorCode::AddingSerialPrimaryKeyField => "adding-serial-primary-key-field",
            ErrorCode::RenamingColumn => "renaming-column",
            ErrorCode::RenamingTable => "renaming-table",
            ErrorCode::DisallowedUniqueConstraint => "disallowed-unique-constraint",
            ErrorCode::BanDropDatabase => "ban-drop-database",
            ErrorCode::PreferBigInt => "prefer-big-int",
            ErrorCode::PreferBigintOverInt => "prefer-bigint-over-int",
            ErrorCode::PreferBigintOverSmallint => "prefer-bigint-over-smallint",
            ErrorCode::PreferIdentity => "prefer-identity",
            ErrorCode::PreferRobustStmts => "prefer-robust-stmts",
            ErrorCode::PreferTextField => "prefer-text-field",
            ErrorCode::PreferTimestampTz => "prefer-timestamp-tz",
            ErrorCode::BanCharField => "ban-char-field",
            ErrorCode::BanDropColumn => "ban-drop-column",
            ErrorCode::BanDropTable => "ban-drop-table",
            ErrorCode::BanDropNotNull => "ban-drop-not-null",
            ErrorCode::TransactionNesting => "transaction-nesting",
            ErrorCode::AddingRequiredField => "adding-required-field",
            ErrorCode::BanConcurrentIndexCreationInTransaction => {
                "ban-concurrent-index-creation-in-transaction"
            }
            ErrorCode::BanCreateDomainWithConstraint => "ban-create-domain-with-constraint",
            ErrorCode::UnusedIgnore => "unused-ignore",
            ErrorCode::BanAlterDomainWithAddConstraint => "ban-alter-domain-with-add-constraint",
        };
        write!(f, "{}", val)
    }
}

impl ErrorCode {
    pub fn meta(&self) -> ViolationMeta {
        match self {
        ErrorCode::RequireConcurrentIndexCreation => ViolationMeta::new(
            "Require Concurrent Index Creation",
            [
                ViolationMessage::Note("Creating an index blocks writes."),
                ViolationMessage::Help("Create the index CONCURRENTLY."),
            ]
        ),
        ErrorCode::RequireConcurrentIndexDeletion => ViolationMeta::new(
            "Require Concurrent Index Deletion", 
            [
                ViolationMessage::Note("Deleting an index blocks selects, inserts, updates, and deletes on the index's table."),
                ViolationMessage::Help("Delete the index CONCURRENTLY."),
            ]
        ),
        ErrorCode::ConstraintMissingNotValid => ViolationMeta::new(
            "Constraint Missing Not Valid", 
            [
                ViolationMessage::Note("Requires a table scan to verify constraint and an ACCESS EXCLUSIVE lock which blocks reads."),
                ViolationMessage::Help("Add NOT VALID to the constraint in one transaction and then VALIDATE the constraint in a separate transaction."),
            ]
        ),
        ErrorCode::AddingFieldWithDefault => ViolationMeta::new(
            "Adding Field With Default", 
            [
                ViolationMessage::Note("Adding a field with a VOLATILE DEFAULT requires a table rewrite with an ACCESS EXCLUSIVE lock. In Postgres versions 11+, non-VOLATILE DEFAULTs can be added without a rewrite."),
                ViolationMessage::Help("Add the field as nullable, then set a default, backfill, and remove nullabilty."),
            ]
        ),
        ErrorCode::AddingForeignKeyConstraint => ViolationMeta::new(
            "Adding Foreign Key Constraint", 
            [
                ViolationMessage::Note("Requires a table scan of the table you're altering and a SHARE ROW EXCLUSIVE lock on both tables, which blocks writes to both tables while your table is scanned."),
                ViolationMessage::Help("Add NOT VALID to the constraint in one transaction and then VALIDATE the constraint in a separate transaction."),
            ]
        ),
        ErrorCode::ChangingColumnType => ViolationMeta::new(
            "Changing Column Type", 
            [
                ViolationMessage::Note("Requires an ACCESS EXCLUSIVE lock on the table which blocks reads."),
                ViolationMessage::Note("Changing the type may break existing clients."),
            ]
        ),
        ErrorCode::AddingNotNullableField => ViolationMeta::new(
            "Adding Not Nullable Field", 
            [
                ViolationMessage::Note("Adding a NOT NULL field requires exclusive locks and table rewrites."),
                ViolationMessage::Help("Make the field nullable."),
            ]
        ),
        ErrorCode::AddingSerialPrimaryKeyField => ViolationMeta::new(
            "Adding Serial Primary Key Field", 
            [
                ViolationMessage::Note("Adding a PRIMARY KEY constraint results in locks and table rewrites"),
                ViolationMessage::Help("Add the PRIMARY KEY constraint USING an index."),
            ]
        ),
        ErrorCode::RenamingColumn => ViolationMeta::new(
            "Renaming Column", 
            [ViolationMessage::Note("Renaming a column may break existing clients.")]
        ),
        ErrorCode::RenamingTable => ViolationMeta::new(
            "Renaming Table", 
            [ViolationMessage::Note("Renaming a table may break existing clients.")]
        ),
        ErrorCode::DisallowedUniqueConstraint => ViolationMeta::new(
            "Disallowed Unique Constraint", 
            [
                ViolationMessage::Note("Adding a UNIQUE constraint requires an ACCESS EXCLUSIVE lock which blocks reads."),
                ViolationMessage::Help("Create an index CONCURRENTLY and create the constraint using the index."),
            ]
        ),
        ErrorCode::BanDropDatabase => ViolationMeta::new(
            "Ban Drop Database", 
            [ViolationMessage::Note("Dropping a database may break existing clients.")]
        ),
        ErrorCode::PreferBigInt => ViolationMeta::new(
            "Prefer Big Int", 
            [
                ViolationMessage::Note("Hitting the max 32 bit integer is possible and may break your application."),
                ViolationMessage::Help("Use 64bit integer values instead to prevent hitting this limit."),
            ]
        ),
        ErrorCode::PreferBigintOverSmallint => ViolationMeta::new(
            "Prefer Bigint Over Smallint", 
            [
                ViolationMessage::Note("Hitting the max 16 bit integer is possible and may break your application."),
                ViolationMessage::Help("Use 64bit integer values instead to prevent hitting this limit."),
            ]
        ),
        ErrorCode::PreferIdentity => ViolationMeta::new(
            "Prefer Identity", 
            [
                ViolationMessage::Note("Serial types have confusing behaviors that make schema management difficult."),
                ViolationMessage::Help("Use identity columns instead for more features and better usability."),
            ]
        ),
        ErrorCode::PreferRobustStmts => ViolationMeta::new(
            "Prefer Robust Statements", 
            [ViolationMessage::Help("Consider wrapping in a transaction or adding a IF NOT EXISTS clause if the statement supports it.")]
        ),
        ErrorCode::PreferTextField => ViolationMeta::new(
            "Prefer Text Field", 
            [
                ViolationMessage::Note("Changing the size of a varchar field requires an ACCESS EXCLUSIVE lock."),
                ViolationMessage::Help("Use a text field with a check constraint."),
            ]
        ),
        ErrorCode::PreferTimestampTz => ViolationMeta::new(
            "Prefer Timestamp with Timezone", 
            [
                ViolationMessage::Note("A timestamp field without a timezone can lead to data loss, depending on your database session timezone."),
                ViolationMessage::Help("Use timestamptz instead of timestamp for your column type."),
            ]
        ),
        ErrorCode::BanCharField => ViolationMeta::new(
            "Ban Char Field", 
            [ViolationMessage::Help("Use text or varchar instead.")]
        ),
        ErrorCode::BanDropColumn => ViolationMeta::new(
            "Dropping columns not allowed", 
            [ViolationMessage::Note("Dropping a column may break existing clients.")]
        ),
        ErrorCode::BanDropTable => ViolationMeta::new(
            "Ban Drop Table", 
            [ViolationMessage::Note("Dropping a table may break existing clients.")]
        ),
        ErrorCode::BanDropNotNull => ViolationMeta::new(
            "Ban Drop Not Null", 
            [ViolationMessage::Note("Dropping a NOT NULL constraint may break existing clients.")]
        ),
        ErrorCode::TransactionNesting => ViolationMeta::new(
            "Transaction Nesting", 
            [
                ViolationMessage::Note("There is an existing transaction already in progress."),
                ViolationMessage::Help("COMMIT the previous transaction before issuing a BEGIN or START TRANSACTION statement."),
            ]
        ),
        ErrorCode::AddingRequiredField => ViolationMeta::new(
            "Adding Required Field", 
            [
                ViolationMessage::Note("Adding a NOT NULL field without a DEFAULT will fail for a populated table."),
                ViolationMessage::Help("Make the field nullable or add a non-VOLATILE DEFAULT (Postgres 11+)."),
            ]
        ),
        ErrorCode::BanConcurrentIndexCreationInTransaction => ViolationMeta::new(
            "Ban Concurrent Index Creation in Transaction", 
            [
                ViolationMessage::Note("Concurrent index creation is not allowed inside a transaction."),
                ViolationMessage::Help("Build the index outside any transactions."),
            ]
        ),
        ErrorCode::PreferBigintOverInt => ViolationMeta::new(
            "Prefer Big Int Over Int",
            [
                ViolationMessage::Note(
                    "Hitting the max 32 bit integer is possible and may break your application."
                ),
                ViolationMessage::Help(
                    "Use 64bit integer values instead to prevent hitting this limit."
                ),
            ]
        ),
        ErrorCode::BanCreateDomainWithConstraint => ViolationMeta::new(
            "Ban Create Domains with Constraints",
            [
                ViolationMessage::Note(
                    "Domains with constraints have poor support for online migrations",
                ),
            ]
        ),
        ErrorCode::BanAlterDomainWithAddConstraint => ViolationMeta::new(
            "Ban Alter Domain With Add Constraints",
            [
                ViolationMessage::Note(
                    "Domains with constraints have poor support for online migrations",
                )
            ]
        ),
        ErrorCode::UnusedIgnore => ViolationMeta::new("Unused linter ignore", [])
    }
    }
}

#[derive(Debug)]
pub enum ViolationMessage<'a> {
    Note(&'a str),
    Help(&'a str),
}

#[derive(Debug)]
pub struct ViolationMeta<'a> {
    /// A description of the rule that's used when rendering the error message
    /// in on the CLI. It should be a slightly expanded version of the [`ViolationName`]
    pub title: String,
    /// Messages rendered for each error to provide context and offer advice on how to fix.
    pub messages: Vec<ViolationMessage<'a>>,
}

impl<'a> ViolationMeta<'a> {
    pub fn new(
        title: impl Into<String>,
        messages: impl Into<Vec<ViolationMessage<'a>>>,
    ) -> ViolationMeta<'a> {
        ViolationMeta {
            title: title.into(),
            messages: messages.into(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Violation {
    pub code: ErrorCode,
    pub message: String,
    pub text_range: TextRange,
    pub messages: Vec<String>,
}

impl Violation {
    #[must_use]
    pub(crate) fn new(
        code: ErrorCode,
        message: String,
        text_range: TextRange,
        messages: impl Into<Option<Vec<String>>>,
    ) -> Self {
        Self {
            code,
            text_range,
            message,
            messages: messages.into().unwrap_or_default(),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Version {
    major: i32,
    minor: Option<i32>,
    patch: Option<i32>,
}

impl Version {
    #[must_use]
    pub(crate) fn new(
        major: i32,
        minor: impl Into<Option<i32>>,
        patch: impl Into<Option<i32>>,
    ) -> Self {
        Self {
            major,
            minor: minor.into(),
            patch: patch.into(),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Sequence)]
pub enum Rule {
    AddingFieldWithDefault,
    AddingForeignKeyConstraint,
    AddingNotNullField,
    AddingPrimaryKeyConstraint,
    AddingRequiredField,
    BanDropDatabase,
    BanCharField,
    BanConcurrentIndexCreationInTransaction,
    BanDropColumn,
    BanDropNotNull,
    BanDropTable,
    ChangingColumnType,
    ConstraintMissingNotValid,
    DisallowUniqueConstraint,
    PreferBigInt,
    PreferBigintOverInt,
    PreferBigintOverSmallint,
    PreferIdentity,
    PreferRobustStmts,
    PreferTextField,
    PreferTimestamptz,
    RenamingColumn,
    RenamingTable,
    RequireConcurrentIndexCreation,
    RequireConcurrentIndexDeletion,
    BanCreateDomainWithConstraint,
    BanAlterDomainWithAddConstraint,
    // xtask:new-lint:name
}

pub struct LinterSettings {
    pub pg_version: Version,
    pub assume_in_transaction: bool,
}

lazy_static! {
    static ref DEFAULT_PG_VERSION: Version = Version::new(15, 0, 0);
}

pub struct Linter {
    errors: Vec<Violation>,
    ignores: Vec<Ignore>,
    pub rules: HashSet<Rule>,
    pub settings: LinterSettings,
}

impl Linter {
    fn report(&mut self, error: Violation) {
        self.errors.push(error);
    }

    fn ignore(&mut self, ignore: Ignore) {
        self.ignores.push(ignore);
    }

    #[must_use]
    pub fn lint(&mut self, file: Parse<SourceFile>, text: &str) -> Vec<&Violation> {
        if self.rules.contains(&Rule::AddingFieldWithDefault) {
            adding_field_with_default(self, &file);
        }
        if self.rules.contains(&Rule::AddingForeignKeyConstraint) {
            adding_foreign_key_constraint(self, &file);
        }
        if self.rules.contains(&Rule::AddingNotNullField) {
            adding_not_null_field(self, &file);
        }
        if self.rules.contains(&Rule::AddingPrimaryKeyConstraint) {
            adding_primary_key_constraint(self, &file);
        }
        if self.rules.contains(&Rule::AddingRequiredField) {
            adding_required_field(self, &file);
        }
        if self.rules.contains(&Rule::BanDropDatabase) {
            ban_drop_database(self, &file);
        }
        if self.rules.contains(&Rule::BanCharField) {
            ban_char_field(self, &file);
        }
        if self
            .rules
            .contains(&Rule::BanConcurrentIndexCreationInTransaction)
        {
            ban_concurrent_index_creation_in_transaction(self, &file);
        }
        if self.rules.contains(&Rule::BanDropColumn) {
            ban_drop_column(self, &file);
        }
        if self.rules.contains(&Rule::BanDropNotNull) {
            ban_drop_not_null(self, &file);
        }
        if self.rules.contains(&Rule::BanDropTable) {
            ban_drop_table(self, &file);
        }
        if self.rules.contains(&Rule::ChangingColumnType) {
            changing_column_type(self, &file);
        }
        if self.rules.contains(&Rule::ConstraintMissingNotValid) {
            constraint_missing_not_valid(self, &file);
        }
        if self.rules.contains(&Rule::DisallowUniqueConstraint) {
            disallow_unique_constraint(self, &file);
        }
        if self.rules.contains(&Rule::PreferBigInt) {
            prefer_big_int(self, &file);
        }
        if self.rules.contains(&Rule::PreferBigintOverInt) {
            prefer_bigint_over_int(self, &file);
        }
        if self.rules.contains(&Rule::PreferBigintOverSmallint) {
            prefer_bigint_over_smallint(self, &file);
        }
        if self.rules.contains(&Rule::PreferIdentity) {
            prefer_identity(self, &file);
        }
        if self.rules.contains(&Rule::PreferRobustStmts) {
            prefer_robust_stmts(self, &file);
        }
        if self.rules.contains(&Rule::PreferTextField) {
            prefer_text_field(self, &file);
        }
        if self.rules.contains(&Rule::PreferTimestamptz) {
            prefer_timestamptz(self, &file);
        }
        if self.rules.contains(&Rule::RenamingColumn) {
            renaming_column(self, &file);
        }
        if self.rules.contains(&Rule::RenamingTable) {
            renaming_table(self, &file);
        }
        if self.rules.contains(&Rule::RequireConcurrentIndexCreation) {
            require_concurrent_index_creation(self, &file);
        }
        if self.rules.contains(&Rule::RequireConcurrentIndexDeletion) {
            require_concurrent_index_deletion(self, &file);
        }
        if self.rules.contains(&Rule::BanCreateDomainWithConstraint) {
            ban_create_domain_with_constraint(self, &file);
        }
        if self.rules.contains(&Rule::BanAlterDomainWithAddConstraint) {
            ban_alter_domain_with_add_constraint(self, &file);
        }
        // xtask:new-lint:rule-call

        // locate any ignores in the file
        find_ignores(self, &file.syntax_node());

        self.errors(text)
    }

    fn errors(&mut self, text: &str) -> Vec<&Violation> {
        // ensure we order them by where they appear in the file
        self.errors.sort_by_key(|x| x.text_range.start());

        let ignore_index = IgnoreIndex::new(text, &self.ignores);
        // TODO: we should have errors for when there was an ignore but that
        // ignore didn't actually ignore anything

        self.errors
            .iter()
            .filter(|err| !ignore_index.contains(err.text_range, err.code))
            .collect::<Vec<_>>()
    }

    pub fn with_all_rules() -> Self {
        let rules = all::<Rule>().collect::<HashSet<_>>();
        Linter::from(rules)
    }

    pub fn from(rules: impl Into<HashSet<Rule>>) -> Self {
        Self {
            errors: vec![],
            ignores: vec![],
            rules: rules.into(),
            settings: LinterSettings {
                pg_version: *DEFAULT_PG_VERSION,
                assume_in_transaction: false,
            },
        }
    }
}
