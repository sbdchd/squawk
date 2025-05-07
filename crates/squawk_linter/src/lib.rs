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

pub use version::Version;

mod ignore;
mod ignore_index;
mod version;
mod visitors;

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
use rules::ban_drop_database;
use rules::ban_drop_not_null;
use rules::ban_drop_table;
use rules::changing_column_type;
use rules::constraint_missing_not_valid;
use rules::disallow_unique_constraint;
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
use rules::transaction_nesting;
// xtask:new-lint:rule-import

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Hash, Eq, Deserialize, Sequence)]
pub enum Rule {
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

impl TryFrom<&str> for Rule {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "require-concurrent-index-creation" => Ok(Rule::RequireConcurrentIndexCreation),
            "require-concurrent-index-deletion" => Ok(Rule::RequireConcurrentIndexDeletion),
            "constraint-missing-not-valid" => Ok(Rule::ConstraintMissingNotValid),
            "adding-field-with-default" => Ok(Rule::AddingFieldWithDefault),
            "adding-foreign-key-constraint" => Ok(Rule::AddingForeignKeyConstraint),
            "changing-column-type" => Ok(Rule::ChangingColumnType),
            "adding-not-nullable-field" => Ok(Rule::AddingNotNullableField),
            "adding-serial-primary-key-field" => Ok(Rule::AddingSerialPrimaryKeyField),
            "renaming-column" => Ok(Rule::RenamingColumn),
            "renaming-table" => Ok(Rule::RenamingTable),
            "disallowed-unique-constraint" => Ok(Rule::DisallowedUniqueConstraint),
            "ban-drop-database" => Ok(Rule::BanDropDatabase),
            "prefer-bigint-over-int" => Ok(Rule::PreferBigintOverInt),
            "prefer-bigint-over-smallint" => Ok(Rule::PreferBigintOverSmallint),
            "prefer-identity" => Ok(Rule::PreferIdentity),
            "prefer-robust-stmts" => Ok(Rule::PreferRobustStmts),
            "prefer-text-field" => Ok(Rule::PreferTextField),
            "prefer-timestamptz" => Ok(Rule::PreferTimestampTz),
            "ban-char-field" => Ok(Rule::BanCharField),
            "ban-drop-column" => Ok(Rule::BanDropColumn),
            "ban-drop-table" => Ok(Rule::BanDropTable),
            "ban-drop-not-null" => Ok(Rule::BanDropNotNull),
            "transaction-nesting" => Ok(Rule::TransactionNesting),
            "adding-required-field" => Ok(Rule::AddingRequiredField),
            "ban-concurrent-index-creation-in-transaction" => {
                Ok(Rule::BanConcurrentIndexCreationInTransaction)
            }
            "ban-create-domain-with-constraint" => Ok(Rule::BanCreateDomainWithConstraint),
            "ban-alter-domain-with-add-constraint" => Ok(Rule::BanAlterDomainWithAddConstraint),
            // xtask:new-lint:str-name
            _ => Err(format!("Unknown violation name: {}", s)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnknownRuleName {
    val: String,
}

impl std::fmt::Display for UnknownRuleName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "invalid rule name {}", self.val)
    }
}

impl std::str::FromStr for Rule {
    type Err = UnknownRuleName;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_plain::from_str(s).map_err(|_| UnknownRuleName { val: s.to_string() })
    }
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = match &self {
            Rule::RequireConcurrentIndexCreation => "require-concurrent-index-creation",
            Rule::RequireConcurrentIndexDeletion => "require-concurrent-index-deletion",
            Rule::ConstraintMissingNotValid => "constraint-missing-not-valid",
            Rule::AddingFieldWithDefault => "adding-field-with-default",
            Rule::AddingForeignKeyConstraint => "adding-foreign-key-constraint",
            Rule::ChangingColumnType => "changing-column-type",
            Rule::AddingNotNullableField => "adding-not-nullable-field",
            Rule::AddingSerialPrimaryKeyField => "adding-serial-primary-key-field",
            Rule::RenamingColumn => "renaming-column",
            Rule::RenamingTable => "renaming-table",
            Rule::DisallowedUniqueConstraint => "disallowed-unique-constraint",
            Rule::BanDropDatabase => "ban-drop-database",
            Rule::PreferBigintOverInt => "prefer-bigint-over-int",
            Rule::PreferBigintOverSmallint => "prefer-bigint-over-smallint",
            Rule::PreferIdentity => "prefer-identity",
            Rule::PreferRobustStmts => "prefer-robust-stmts",
            Rule::PreferTextField => "prefer-text-field",
            Rule::PreferTimestampTz => "prefer-timestamp-tz",
            Rule::BanCharField => "ban-char-field",
            Rule::BanDropColumn => "ban-drop-column",
            Rule::BanDropTable => "ban-drop-table",
            Rule::BanDropNotNull => "ban-drop-not-null",
            Rule::TransactionNesting => "transaction-nesting",
            Rule::AddingRequiredField => "adding-required-field",
            Rule::BanConcurrentIndexCreationInTransaction => {
                "ban-concurrent-index-creation-in-transaction"
            }
            Rule::BanCreateDomainWithConstraint => "ban-create-domain-with-constraint",
            Rule::UnusedIgnore => "unused-ignore",
            Rule::BanAlterDomainWithAddConstraint => "ban-alter-domain-with-add-constraint",
        };
        write!(f, "{}", val)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Violation {
    // TODO: should this be String instead?
    pub code: Rule,
    pub message: String,
    pub text_range: TextRange,
    pub help: Option<String>,
}

impl Violation {
    #[must_use]
    pub fn new(
        code: Rule,
        message: String,
        text_range: TextRange,
        help: impl Into<Option<String>>,
    ) -> Self {
        Self {
            code,
            text_range,
            message,
            help: help.into(),
        }
    }
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
    pub fn lint(&mut self, file: Parse<SourceFile>, text: &str) -> Vec<Violation> {
        if self.rules.contains(&Rule::AddingFieldWithDefault) {
            adding_field_with_default(self, &file);
        }
        if self.rules.contains(&Rule::AddingForeignKeyConstraint) {
            adding_foreign_key_constraint(self, &file);
        }
        if self.rules.contains(&Rule::AddingNotNullableField) {
            adding_not_null_field(self, &file);
        }
        if self.rules.contains(&Rule::AddingSerialPrimaryKeyField) {
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
        if self.rules.contains(&Rule::DisallowedUniqueConstraint) {
            disallow_unique_constraint(self, &file);
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
        if self.rules.contains(&Rule::PreferTimestampTz) {
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
        if self.rules.contains(&Rule::TransactionNesting) {
            transaction_nesting(self, &file);
        }
        // xtask:new-lint:rule-call

        // locate any ignores in the file
        find_ignores(self, &file.syntax_node());

        self.errors(text)
    }

    fn errors(&mut self, text: &str) -> Vec<Violation> {
        let ignore_index = IgnoreIndex::new(text, &self.ignores);
        let mut errors: Vec<Violation> = self
            .errors
            .iter()
            // TODO: we should have errors for when there was an ignore but that
            // ignore didn't actually ignore anything
            .filter(|err| !ignore_index.contains(err.text_range, err.code))
            .cloned()
            .collect::<Vec<_>>();
        // ensure we order them by where they appear in the file
        errors.sort_by_key(|x| x.text_range.start());
        errors
    }

    pub fn with_all_rules() -> Self {
        let rules = all::<Rule>().collect::<HashSet<_>>();
        Linter::from(rules)
    }

    pub fn without_rules(exclude: &[Rule]) -> Self {
        let all_rules = all::<Rule>().collect::<HashSet<_>>();
        let mut exclude_set = HashSet::with_capacity(exclude.len());
        for e in exclude {
            exclude_set.insert(e);
        }

        let rules = all_rules
            .into_iter()
            .filter(|x| !exclude_set.contains(x))
            .collect::<HashSet<_>>();

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
