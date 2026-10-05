use rustc_hash::FxHashSet;
use std::fmt;

use enum_iterator::Sequence;
use enum_iterator::all;
pub use ignore::Ignore;
use ignore::find_ignores;
use ignore::has_disable_assume_in_transaction;
use ignore_index::IgnoreIndex;
use rowan::TextRange;
use rowan::TextSize;
use serde::Deserialize;

use squawk_syntax::SyntaxNode;
use squawk_syntax::{Parse, SourceFile};

pub use version::Version;

pub mod analyze;
pub mod ignore;
mod ignore_index;
mod version;
mod visitors;

mod rules;

#[cfg(test)]
mod test_utils;
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
use rules::ban_drop_default;
use rules::ban_drop_function;
use rules::ban_drop_not_null;
use rules::ban_drop_table;
use rules::ban_drop_trigger;
use rules::ban_drop_type;
use rules::ban_drop_view;
use rules::ban_duplicate_column_assignments;
use rules::ban_truncate_cascade;
use rules::ban_uncommitted_transaction;
use rules::changing_column_type;
use rules::constraint_missing_not_valid;
use rules::disallow_unique_constraint;
use rules::identifier_too_long;
use rules::prefer_bigint_over_int;
use rules::prefer_bigint_over_smallint;
use rules::prefer_identity;
use rules::prefer_repack;
use rules::prefer_robust_stmts;
use rules::prefer_text_field;
use rules::prefer_timestamptz;
use rules::renaming_column;
use rules::renaming_table;
use rules::require_concurrent_index_creation;
use rules::require_concurrent_index_deletion;
use rules::require_concurrent_partition_detach;
use rules::require_concurrent_reindex;
use rules::require_enum_value_ordering;
use rules::require_table_schema;
use rules::require_timeout_settings;
use rules::transaction_nesting;
use rules::{
    ban_alter_identity, ban_drop_constraint, ban_drop_domain, ban_drop_extension,
    ban_drop_generated_expression, ban_drop_schema, ban_drop_sequence, ban_set_schema,
    renaming_object,
    ban_alter_generated_expression, ban_disable_trigger, ban_drop_index, ban_drop_policy,
    ban_replace_view_function, ban_replica_identity, ban_revoke, ban_set_default,
    compatibility_additions, security_compatibility,
};
// xtask:new-rule:rule-import

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq, Sequence)]
pub enum Rule {
    RequireConcurrentIndexCreation,
    RequireConcurrentIndexDeletion,
    ConstraintMissingNotValid,
    AddingFieldWithDefault,
    AddingForeignKeyConstraint,
    ChangingColumnType,
    AddingNotNullableField,
    AddingSerialPrimaryKeyField,
    RenamingColumn,
    RenamingTable,
    DisallowedUniqueConstraint,
    BanDropDatabase,
    PreferBigintOverInt,
    PreferBigintOverSmallint,
    PreferIdentity,
    PreferRepack,
    PreferRobustStmts,
    PreferTextField,
    PreferTimestampTz,
    BanCharField,
    BanDropColumn,
    BanDropTable,
    BanDropNotNull,
    TransactionNesting,
    AddingRequiredField,
    BanConcurrentIndexCreationInTransaction,
    UnusedIgnore,
    BanCreateDomainWithConstraint,
    BanAlterDomainWithAddConstraint,
    BanTruncateCascade,
    RequireTimeoutSettings,
    BanUncommittedTransaction,
    RequireEnumValueOrdering,
    RequireTableSchema,
    IdentifierTooLong,
    RequireConcurrentPartitionDetach,
    RequireConcurrentReindex,
    RequireLockTimeout,
    RequireStatementTimeout,
    BanDuplicateColumnAssignments,
    BanDropView,
    BanDropFunction,
    BanDropType,
    BanDropDefault,
    BanDropTrigger,
    BanDropSchema,
    BanDropSequence,
    BanDropDomain,
    BanDropConstraint,
    BanDropGeneratedExpression,
    RenamingObject,
    BanSetSchema,
    BanAlterIdentity,
    BanDropExtension,
    BanAlterGeneratedExpression,
    BanDropIndex,
    BanSetDefault,
    BanDisableTrigger,
    BanReplicaIdentity,
    BanDropPolicy,
    BanRevoke,
    BanReplaceViewFunction,
    BanAlterPolicyCondition,
    BanAlterPolicyRoles,
    BanCreatePolicy,
    BanAlterFunctionOptions,
    BanAlterViewOptions,
    BanAlterRoleOptions,
    BanAlterDatabaseOptions,
    BanAlterRowLevelSecurity,
    BanNewWriteRestriction,
    BanAddEnumValue,
    BanAddCompositeAttribute,
    BanAddColumn,
    BanDetachInheritance,
    BanAlterSequenceValues,
    BanAlterSystemOptions,
    BanAlterExtension,
    // xtask:new-rule:error-name
}

impl Rule {
    /// Rules that are opt-in are not enabled by default.
    /// They must be explicitly included via configuration.
    pub fn is_opt_in(&self) -> bool {
        // require-timeout-settings is an alias, see `Rule::expands_to`
        matches!(
            self,
            Rule::RequireTableSchema
                | Rule::RequireTimeoutSettings
                | Rule::BanDropTrigger
                | Rule::BanAlterGeneratedExpression
                | Rule::BanDropIndex
                | Rule::BanSetDefault
                | Rule::BanDisableTrigger
                | Rule::BanReplicaIdentity
                | Rule::BanDropPolicy
                | Rule::BanRevoke
                | Rule::BanReplaceViewFunction
                | Rule::BanAlterPolicyCondition
                | Rule::BanAlterPolicyRoles
                | Rule::BanCreatePolicy
                | Rule::BanAlterFunctionOptions
                | Rule::BanAlterViewOptions
                | Rule::BanAlterRoleOptions
                | Rule::BanAlterDatabaseOptions
                | Rule::BanAlterRowLevelSecurity
                | Rule::BanNewWriteRestriction
                | Rule::BanAddEnumValue
                | Rule::BanAddCompositeAttribute
                | Rule::BanAddColumn
                | Rule::BanDetachInheritance
                | Rule::BanAlterSequenceValues
                | Rule::BanAlterSystemOptions
                | Rule::BanAlterExtension
        )
    }

    /// Rules that are deprecated aliases for other rules.
    pub fn expands_to(&self) -> &[Rule] {
        match self {
            Rule::RequireTimeoutSettings => {
                &[Rule::RequireLockTimeout, Rule::RequireStatementTimeout]
            }
            _ => &[],
        }
    }
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
            "prefer-repack" => Ok(Rule::PreferRepack),
            "prefer-robust-stmts" => Ok(Rule::PreferRobustStmts),
            "prefer-text-field" => Ok(Rule::PreferTextField),
            // this is typo'd so we just support both
            "prefer-timestamptz" => Ok(Rule::PreferTimestampTz),
            "prefer-timestamp-tz" => Ok(Rule::PreferTimestampTz),
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
            "ban-truncate-cascade" => Ok(Rule::BanTruncateCascade),
            "require-timeout-settings" => Ok(Rule::RequireTimeoutSettings),
            "ban-uncommitted-transaction" => Ok(Rule::BanUncommittedTransaction),
            "require-enum-value-ordering" => Ok(Rule::RequireEnumValueOrdering),
            "require-table-schema" => Ok(Rule::RequireTableSchema),
            "identifier-too-long" => Ok(Rule::IdentifierTooLong),
            "require-concurrent-partition-detach" => Ok(Rule::RequireConcurrentPartitionDetach),
            "require-concurrent-reindex" => Ok(Rule::RequireConcurrentReindex),
            "require-lock-timeout" => Ok(Rule::RequireLockTimeout),
            "require-statement-timeout" => Ok(Rule::RequireStatementTimeout),
            "ban-duplicate-column-assignments" => Ok(Rule::BanDuplicateColumnAssignments),
            "ban-drop-view" => Ok(Rule::BanDropView),
            "ban-drop-function" => Ok(Rule::BanDropFunction),
            "ban-drop-type" => Ok(Rule::BanDropType),
            "ban-drop-default" => Ok(Rule::BanDropDefault),
            "ban-drop-trigger" => Ok(Rule::BanDropTrigger),
            "ban-drop-schema" => Ok(Rule::BanDropSchema),
            "ban-drop-sequence" => Ok(Rule::BanDropSequence),
            "ban-drop-domain" => Ok(Rule::BanDropDomain),
            "ban-drop-constraint" => Ok(Rule::BanDropConstraint),
            "ban-drop-generated-expression" => Ok(Rule::BanDropGeneratedExpression),
            "renaming-object" => Ok(Rule::RenamingObject),
            "ban-set-schema" => Ok(Rule::BanSetSchema),
            "ban-alter-identity" => Ok(Rule::BanAlterIdentity),
            "ban-drop-extension" => Ok(Rule::BanDropExtension),
            "ban-alter-generated-expression" => Ok(Rule::BanAlterGeneratedExpression),
            "ban-drop-index" => Ok(Rule::BanDropIndex),
            "ban-set-default" => Ok(Rule::BanSetDefault),
            "ban-disable-trigger" => Ok(Rule::BanDisableTrigger),
            "ban-replica-identity" => Ok(Rule::BanReplicaIdentity),
            "ban-drop-policy" => Ok(Rule::BanDropPolicy),
            "ban-revoke" => Ok(Rule::BanRevoke),
            "ban-replace-view-function" => Ok(Rule::BanReplaceViewFunction),
            "ban-alter-policy-condition" => Ok(Rule::BanAlterPolicyCondition),
            "ban-alter-policy-roles" => Ok(Rule::BanAlterPolicyRoles),
            "ban-create-policy" => Ok(Rule::BanCreatePolicy),
            "ban-alter-function-options" => Ok(Rule::BanAlterFunctionOptions),
            "ban-alter-view-options" => Ok(Rule::BanAlterViewOptions),
            "ban-alter-role-options" => Ok(Rule::BanAlterRoleOptions),
            "ban-alter-database-options" => Ok(Rule::BanAlterDatabaseOptions),
            "ban-alter-row-level-security" => Ok(Rule::BanAlterRowLevelSecurity),
            "ban-new-write-restriction" => Ok(Rule::BanNewWriteRestriction),
            "ban-add-enum-value" => Ok(Rule::BanAddEnumValue),
            "ban-add-composite-attribute" => Ok(Rule::BanAddCompositeAttribute),
            "ban-add-column" => Ok(Rule::BanAddColumn),
            "ban-detach-inheritance" => Ok(Rule::BanDetachInheritance),
            "ban-alter-sequence-values" => Ok(Rule::BanAlterSequenceValues),
            "ban-alter-system-options" => Ok(Rule::BanAlterSystemOptions),
            "ban-alter-extension" => Ok(Rule::BanAlterExtension),
            // xtask:new-rule:str-name
            _ => Err(format!("Unknown violation name: {s}")),
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

impl std::error::Error for UnknownRuleName {}

impl std::str::FromStr for Rule {
    type Err = UnknownRuleName;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Rule::try_from(s).map_err(|_| UnknownRuleName { val: s.to_string() })
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
            Rule::PreferRepack => "prefer-repack",
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
            Rule::BanTruncateCascade => "ban-truncate-cascade",
            Rule::RequireTimeoutSettings => "require-timeout-settings",
            Rule::BanUncommittedTransaction => "ban-uncommitted-transaction",
            Rule::RequireEnumValueOrdering => "require-enum-value-ordering",
            Rule::RequireTableSchema => "require-table-schema",
            Rule::IdentifierTooLong => "identifier-too-long",
            Rule::RequireConcurrentPartitionDetach => "require-concurrent-partition-detach",
            Rule::RequireConcurrentReindex => "require-concurrent-reindex",
            Rule::RequireLockTimeout => "require-lock-timeout",
            Rule::RequireStatementTimeout => "require-statement-timeout",
            Rule::BanDuplicateColumnAssignments => "ban-duplicate-column-assignments",
            Rule::BanDropView => "ban-drop-view",
            Rule::BanDropFunction => "ban-drop-function",
            Rule::BanDropType => "ban-drop-type",
            Rule::BanDropDefault => "ban-drop-default",
            Rule::BanDropTrigger => "ban-drop-trigger",
            Rule::BanDropSchema => "ban-drop-schema",
            Rule::BanDropSequence => "ban-drop-sequence",
            Rule::BanDropDomain => "ban-drop-domain",
            Rule::BanDropConstraint => "ban-drop-constraint",
            Rule::BanDropGeneratedExpression => "ban-drop-generated-expression",
            Rule::RenamingObject => "renaming-object",
            Rule::BanSetSchema => "ban-set-schema",
            Rule::BanAlterIdentity => "ban-alter-identity",
            Rule::BanDropExtension => "ban-drop-extension",
            Rule::BanAlterGeneratedExpression => "ban-alter-generated-expression",
            Rule::BanDropIndex => "ban-drop-index",
            Rule::BanSetDefault => "ban-set-default",
            Rule::BanDisableTrigger => "ban-disable-trigger",
            Rule::BanReplicaIdentity => "ban-replica-identity",
            Rule::BanDropPolicy => "ban-drop-policy",
            Rule::BanRevoke => "ban-revoke",
            Rule::BanReplaceViewFunction => "ban-replace-view-function",
            Rule::BanAlterPolicyCondition => "ban-alter-policy-condition",
            Rule::BanAlterPolicyRoles => "ban-alter-policy-roles",
            Rule::BanCreatePolicy => "ban-create-policy",
            Rule::BanAlterFunctionOptions => "ban-alter-function-options",
            Rule::BanAlterViewOptions => "ban-alter-view-options",
            Rule::BanAlterRoleOptions => "ban-alter-role-options",
            Rule::BanAlterDatabaseOptions => "ban-alter-database-options",
            Rule::BanAlterRowLevelSecurity => "ban-alter-row-level-security",
            Rule::BanNewWriteRestriction => "ban-new-write-restriction",
            Rule::BanAddEnumValue => "ban-add-enum-value",
            Rule::BanAddCompositeAttribute => "ban-add-composite-attribute",
            Rule::BanAddColumn => "ban-add-column",
            Rule::BanDetachInheritance => "ban-detach-inheritance",
            Rule::BanAlterSequenceValues => "ban-alter-sequence-values",
            Rule::BanAlterSystemOptions => "ban-alter-system-options",
            Rule::BanAlterExtension => "ban-alter-extension",
            // xtask:new-rule:variant-to-name
        };
        write!(f, "{val}")
    }
}

impl<'de> Deserialize<'de> for Rule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fix {
    pub title: String,
    pub edits: Vec<Edit>,
}

impl Fix {
    fn new<T: Into<String>>(title: T, edits: Vec<Edit>) -> Fix {
        Fix {
            title: title.into(),
            edits,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Edit {
    pub text_range: TextRange,
    // TODO: does this need to be an Option?
    pub text: Option<String>,
}
impl Edit {
    pub fn insert<T: Into<String>>(text: T, at: TextSize) -> Self {
        Self {
            text_range: TextRange::new(at, at),
            text: Some(text.into()),
        }
    }
    pub fn replace<T: Into<String>>(text_range: TextRange, text: T) -> Self {
        Self {
            text_range,
            text: Some(text.into()),
        }
    }
    pub fn delete(text_range: TextRange) -> Self {
        Self {
            text_range,
            text: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Violation {
    // TODO: should this be String instead?
    pub code: Rule,
    pub message: String,
    pub text_range: TextRange,
    pub help: Option<String>,
    pub fix: Option<Fix>,
}

impl Violation {
    #[must_use]
    pub fn for_node(code: Rule, message: String, node: &SyntaxNode) -> Self {
        let range = node.text_range();

        let start = node
            .children_with_tokens()
            .find(|x| !x.kind().is_trivia())
            .map(|x| x.text_range().start())
            // Not sure we actually hit this, but just being safe
            .unwrap_or_else(|| range.start());

        Self {
            code,
            text_range: TextRange::new(start, range.end()),
            message,
            help: None,
            fix: None,
        }
    }

    #[must_use]
    pub fn for_range(code: Rule, message: String, text_range: TextRange) -> Self {
        Self {
            code,
            text_range,
            message,
            help: None,
            fix: None,
        }
    }

    fn fix<F: Into<Option<Fix>>>(mut self, fix: F) -> Violation {
        self.fix = fix.into();
        self
    }
    fn help(mut self, help: impl Into<String>) -> Violation {
        self.help = Some(help.into());
        self
    }
}

#[derive(Clone, Default)]
pub struct LinterSettings {
    pub pg_version: Version,
    pub assume_in_transaction: bool,
}

pub struct Linter {
    errors: Vec<Violation>,
    ignores: Vec<Ignore>,
    pub rules: FxHashSet<Rule>,
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
    pub fn lint(&mut self, file: &Parse<SourceFile>, text: &str) -> Vec<Violation> {
        if has_disable_assume_in_transaction(&file.syntax_node()) {
            self.settings.assume_in_transaction = false;
        }

        if self.rules.contains(&Rule::AddingFieldWithDefault) {
            adding_field_with_default(self, file);
        }
        if self.rules.contains(&Rule::AddingForeignKeyConstraint) {
            adding_foreign_key_constraint(self, file);
        }
        if self.rules.contains(&Rule::AddingNotNullableField) {
            adding_not_null_field(self, file);
        }
        if self.rules.contains(&Rule::AddingSerialPrimaryKeyField) {
            adding_primary_key_constraint(self, file);
        }
        if self.rules.contains(&Rule::AddingRequiredField) {
            adding_required_field(self, file);
        }
        if self.rules.contains(&Rule::BanDropDatabase) {
            ban_drop_database(self, file);
        }
        if self.rules.contains(&Rule::BanCharField) {
            ban_char_field(self, file);
        }
        if self
            .rules
            .contains(&Rule::BanConcurrentIndexCreationInTransaction)
        {
            ban_concurrent_index_creation_in_transaction(self, file);
        }
        if self.rules.contains(&Rule::BanDropColumn) {
            ban_drop_column(self, file);
        }
        if self.rules.contains(&Rule::BanDropNotNull) {
            ban_drop_not_null(self, file);
        }
        if self.rules.contains(&Rule::BanDropTable) {
            ban_drop_table(self, file);
        }
        if self.rules.contains(&Rule::ChangingColumnType) {
            changing_column_type(self, file);
        }
        if self.rules.contains(&Rule::ConstraintMissingNotValid) {
            constraint_missing_not_valid(self, file);
        }
        if self.rules.contains(&Rule::DisallowedUniqueConstraint) {
            disallow_unique_constraint(self, file);
        }
        if self.rules.contains(&Rule::PreferBigintOverInt) {
            prefer_bigint_over_int(self, file);
        }
        if self.rules.contains(&Rule::PreferBigintOverSmallint) {
            prefer_bigint_over_smallint(self, file);
        }
        if self.rules.contains(&Rule::PreferIdentity) {
            prefer_identity(self, file);
        }
        if self.rules.contains(&Rule::PreferRepack) {
            prefer_repack(self, file);
        }
        if self.rules.contains(&Rule::PreferRobustStmts) {
            prefer_robust_stmts(self, file);
        }
        if self.rules.contains(&Rule::PreferTextField) {
            prefer_text_field(self, file);
        }
        if self.rules.contains(&Rule::PreferTimestampTz) {
            prefer_timestamptz(self, file);
        }
        if self.rules.contains(&Rule::RenamingColumn) {
            renaming_column(self, file);
        }
        if self.rules.contains(&Rule::RenamingTable) {
            renaming_table(self, file);
        }
        if self.rules.contains(&Rule::RequireConcurrentIndexCreation) {
            require_concurrent_index_creation(self, file);
        }
        if self.rules.contains(&Rule::RequireConcurrentIndexDeletion) {
            require_concurrent_index_deletion(self, file);
        }
        if self.rules.contains(&Rule::BanCreateDomainWithConstraint) {
            ban_create_domain_with_constraint(self, file);
        }
        if self.rules.contains(&Rule::BanAlterDomainWithAddConstraint) {
            ban_alter_domain_with_add_constraint(self, file);
        }
        if self.rules.contains(&Rule::TransactionNesting) {
            transaction_nesting(self, file);
        }
        if self.rules.contains(&Rule::BanTruncateCascade) {
            ban_truncate_cascade(self, file);
        }
        if self.rules.contains(&Rule::RequireLockTimeout)
            || self.rules.contains(&Rule::RequireStatementTimeout)
        {
            require_timeout_settings(self, file);
        }
        if self.rules.contains(&Rule::BanUncommittedTransaction) {
            ban_uncommitted_transaction(self, file);
        }
        if self.rules.contains(&Rule::RequireEnumValueOrdering) {
            require_enum_value_ordering(self, file);
        }
        if self.rules.contains(&Rule::RequireTableSchema) {
            require_table_schema(self, file);
        }
        if self.rules.contains(&Rule::IdentifierTooLong) {
            identifier_too_long(self, file);
        }
        if self.rules.contains(&Rule::RequireConcurrentPartitionDetach) {
            require_concurrent_partition_detach(self, file);
        }
        if self.rules.contains(&Rule::RequireConcurrentReindex) {
            require_concurrent_reindex(self, file);
        }
        if self.rules.contains(&Rule::BanDuplicateColumnAssignments) {
            ban_duplicate_column_assignments(self, file);
        }
        if self.rules.contains(&Rule::BanDropView) {
            ban_drop_view(self, file);
        }
        if self.rules.contains(&Rule::BanDropFunction) {
            ban_drop_function(self, file);
        }
        if self.rules.contains(&Rule::BanDropType) {
            ban_drop_type(self, file);
        }
        if self.rules.contains(&Rule::BanDropDefault) {
            ban_drop_default(self, file);
        }
        if self.rules.contains(&Rule::BanDropTrigger) {
            ban_drop_trigger(self, file);
        }
        if self.rules.contains(&Rule::BanDropSchema) {
            ban_drop_schema(self, file);
        }
        if self.rules.contains(&Rule::BanDropSequence) {
            ban_drop_sequence(self, file);
        }
        if self.rules.contains(&Rule::BanDropDomain) {
            ban_drop_domain(self, file);
        }
        if self.rules.contains(&Rule::BanDropConstraint) {
            ban_drop_constraint(self, file);
        }
        if self.rules.contains(&Rule::BanDropGeneratedExpression) {
            ban_drop_generated_expression(self, file);
        }
        if self.rules.contains(&Rule::RenamingObject) {
            renaming_object(self, file);
        }
        if self.rules.contains(&Rule::BanSetSchema) {
            ban_set_schema(self, file);
        }
        if self.rules.contains(&Rule::BanAlterIdentity) {
            ban_alter_identity(self, file);
        }
        if self.rules.contains(&Rule::BanDropExtension) {
            ban_drop_extension(self, file);
        }
        if self.rules.contains(&Rule::BanAlterGeneratedExpression) {
            ban_alter_generated_expression(self, file);
        }
        if self.rules.contains(&Rule::BanDropIndex) {
            ban_drop_index(self, file);
        }
        if self.rules.contains(&Rule::BanSetDefault) {
            ban_set_default(self, file);
        }
        if self.rules.contains(&Rule::BanDisableTrigger) {
            ban_disable_trigger(self, file);
        }
        if self.rules.contains(&Rule::BanReplicaIdentity) {
            ban_replica_identity(self, file);
        }
        if self.rules.contains(&Rule::BanDropPolicy) {
            ban_drop_policy(self, file);
        }
        if self.rules.contains(&Rule::BanRevoke) {
            ban_revoke(self, file);
        }
        if self.rules.contains(&Rule::BanReplaceViewFunction) {
            ban_replace_view_function(self, file);
        }
        security_compatibility(self, file);
        if [
            Rule::BanNewWriteRestriction,
            Rule::BanAddEnumValue,
            Rule::BanAddCompositeAttribute,
            Rule::BanAddColumn,
            Rule::BanDetachInheritance,
            Rule::BanAlterSequenceValues,
        ]
        .iter()
        .any(|rule| self.rules.contains(rule))
        {
            compatibility_additions(self, file);
        }
        // xtask:new-rule:rule-call

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

    fn default_rules() -> FxHashSet<Rule> {
        all::<Rule>()
            .filter(|r| !r.is_opt_in())
            .collect::<FxHashSet<_>>()
    }

    pub fn with_default_rules() -> Self {
        let rules = Linter::default_rules();
        Linter::from(rules)
    }

    pub fn with_rules(include: &[Rule], exclude: &[Rule]) -> Self {
        let mut default_rules = Linter::default_rules();

        for rule in include {
            default_rules.insert(*rule);
            default_rules.extend(rule.expands_to());
        }

        for rule in exclude {
            default_rules.remove(rule);
            for expanded in rule.expands_to() {
                default_rules.remove(expanded);
            }
        }

        // drop aliases so `Linter::from` doesn't expand them again and re-add
        // excluded rules
        default_rules.retain(|rule| rule.expands_to().is_empty());

        Linter::from(default_rules)
    }

    pub fn from(rules: impl IntoIterator<Item = Rule>) -> Self {
        let mut rules: FxHashSet<Rule> = rules.into_iter().collect();
        for rule in rules.clone() {
            rules.extend(rule.expands_to());
        }
        Self {
            errors: vec![],
            ignores: vec![],
            rules,
            settings: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;

    use super::*;

    #[test]
    fn prefer_timestamp_aliases() {
        let rule1: Rule = "prefer-timestamp-tz".parse().unwrap();
        let rule2: Rule = "prefer-timestamptz".parse().unwrap();
        assert_eq!(rule1, rule2);
        assert_debug_snapshot!(rule1, @"PreferTimestampTz");
    }

    #[test]
    fn invalid_rule_name() {
        let result: Result<Rule, _> = "invalid-rule-name".parse();
        assert!(result.is_err());
    }

    #[test]
    fn with_rules_opt_in_disabled_by_default() {
        let linter = Linter::with_rules(&[], &[]);
        assert!(!linter.rules.contains(&Rule::RequireTableSchema));
        assert!(!linter.rules.contains(&Rule::BanDropTrigger));
    }

    #[test]
    fn with_rules_opt_in_enabled_via_include() {
        for rule in [Rule::RequireTableSchema, Rule::BanDropTrigger] {
            let linter = Linter::with_rules(&[rule], &[]);
            assert!(linter.rules.contains(&rule));
        }
    }

    #[test]
    fn with_rules_exclude_takes_precedence_over_include() {
        let linter = Linter::with_rules(&[Rule::RequireTableSchema], &[Rule::RequireTableSchema]);
        assert!(!linter.rules.contains(&Rule::RequireTableSchema));
    }

    #[test]
    fn with_rules_exclude_removes_default_rule() {
        let linter = Linter::with_rules(&[], &[Rule::BanDropTable]);
        assert!(!linter.rules.contains(&Rule::BanDropTable));
    }

    #[test]
    fn require_timeout_settings_expands_to_granular_rules() {
        let linter = Linter::from([Rule::RequireTimeoutSettings]);
        assert!(linter.rules.contains(&Rule::RequireLockTimeout));
        assert!(linter.rules.contains(&Rule::RequireStatementTimeout));
    }

    #[test]
    fn with_rules_exclude_timeout_settings_removes_granular_rules() {
        let linter = Linter::with_rules(&[], &[Rule::RequireTimeoutSettings]);
        assert!(!linter.rules.contains(&Rule::RequireLockTimeout));
        assert!(!linter.rules.contains(&Rule::RequireStatementTimeout));
    }

    #[test]
    fn with_rules_exclude_granular_timeout_rule_keeps_other() {
        let linter = Linter::with_rules(&[], &[Rule::RequireStatementTimeout]);
        assert!(linter.rules.contains(&Rule::RequireLockTimeout));
        assert!(!linter.rules.contains(&Rule::RequireStatementTimeout));
    }

    #[test]
    fn with_rules_exclude_granular_rule_wins_over_included_alias() {
        let linter = Linter::with_rules(
            &[Rule::RequireTimeoutSettings],
            &[Rule::RequireStatementTimeout],
        );
        assert!(linter.rules.contains(&Rule::RequireLockTimeout));
        assert!(!linter.rules.contains(&Rule::RequireStatementTimeout));
    }

    #[test]
    fn compatibility_rules_are_enabled_by_default_and_can_be_excluded() {
        for (rule, sql) in [
            (Rule::BanDropConstraint, "ALTER TABLE t DROP CONSTRAINT c;"),
            (
                Rule::BanAlterIdentity,
                "ALTER TABLE t ALTER COLUMN id ADD GENERATED ALWAYS AS IDENTITY;",
            ),
            (
                Rule::BanDropGeneratedExpression,
                "ALTER TABLE t ALTER COLUMN c DROP EXPRESSION;",
            ),
            (Rule::BanDropExtension, "DROP EXTENSION hstore;"),
        ] {
            let parse = SourceFile::parse(sql);
            assert!(parse.errors().is_empty());
            assert!(
                Linter::with_default_rules()
                    .lint(&parse, sql)
                    .iter()
                    .any(|v| v.code == rule)
            );
            assert!(
                !Linter::with_rules(&[], &[rule])
                    .lint(&parse, sql)
                    .iter()
                    .any(|v| v.code == rule)
            );
        }
    }

    #[test]
    fn compatibility_rules_require_explicit_configuration() {
        for (rule, sql) in [
            (
                Rule::BanAlterGeneratedExpression,
                "ALTER TABLE t ALTER COLUMN c SET EXPRESSION AS (id + 1);",
            ),
            (Rule::BanAddColumn, "ALTER TABLE t ADD COLUMN c int;"),
            (Rule::BanReplicaIdentity, "DROP PUBLICATION p;"),
            (Rule::BanAddEnumValue, "ALTER TYPE mood ADD VALUE 'new';"),
        ] {
            let parse = SourceFile::parse(sql);
            assert!(parse.errors().is_empty());
            assert!(
                !Linter::with_default_rules()
                    .lint(&parse, sql)
                    .iter()
                    .any(|v| v.code == rule)
            );
            assert!(
                Linter::with_rules(&[rule], &[])
                    .lint(&parse, sql)
                    .iter()
                    .any(|v| v.code == rule)
            );
            assert!(
                !Linter::with_rules(&[rule], &[rule])
                    .lint(&parse, sql)
                    .iter()
                    .any(|v| v.code == rule)
            );
        }
    }
}
