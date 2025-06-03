use crate::ast::AstNode;
use crate::ast::{support, AstChildren};
use crate::syntax_node::SyntaxNode;
use crate::syntax_node::SyntaxToken;
use crate::SyntaxKind;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AddColumn {
    pub(crate) syntax: SyntaxNode,
}
impl AddColumn {
    #[inline]
    pub fn collate(&self) -> Option<Collate> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn constraints(&self) -> AstChildren<Constraint> {
        support::children(&self.syntax)
    }
    #[inline]
    pub fn if_not_exists(&self) -> Option<IfNotExists> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn ty(&self) -> Option<Type> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn add_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ADD_KW)
    }
    #[inline]
    pub fn column_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::COLUMN_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AddConstraint {
    pub(crate) syntax: SyntaxNode,
}
impl AddConstraint {
    #[inline]
    pub fn constraint(&self) -> Option<Constraint> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn deferrable_constraint_option(&self) -> Option<DeferrableConstraintOption> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn enforced(&self) -> Option<Enforced> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn initially_deferred_constraint_option(
        &self,
    ) -> Option<InitiallyDeferredConstraintOption> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn initially_immediate_constraint_option(
        &self,
    ) -> Option<InitiallyImmediateConstraintOption> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn no_inherit(&self) -> Option<NoInherit> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn not_deferrable_constraint_option(&self) -> Option<NotDeferrableConstraintOption> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn not_enforced(&self) -> Option<NotEnforced> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn not_valid(&self) -> Option<NotValid> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn add_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ADD_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AddGenerated {
    pub(crate) syntax: SyntaxNode,
}
impl AddGenerated {
    #[inline]
    pub fn add_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ADD_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Aggregate {
    pub(crate) syntax: SyntaxNode,
}
impl Aggregate {
    #[inline]
    pub fn param_list(&self) -> Option<ParamList> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn path(&self) -> Option<Path> {
        support::child(&self.syntax)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Alias {
    pub(crate) syntax: SyntaxNode,
}
impl Alias {
    #[inline]
    pub fn as_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::AS_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AlterAggregate {
    pub(crate) syntax: SyntaxNode,
}
impl AlterAggregate {
    #[inline]
    pub fn aggregate(&self) -> Option<Aggregate> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn aggregate_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::AGGREGATE_KW)
    }
    #[inline]
    pub fn alter_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ALTER_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AlterCollation {
    pub(crate) syntax: SyntaxNode,
}
impl AlterCollation {
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn alter_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ALTER_KW)
    }
    #[inline]
    pub fn collation_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::COLLATION_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AlterColumn {
    pub(crate) syntax: SyntaxNode,
}
impl AlterColumn {
    #[inline]
    pub fn option(&self) -> Option<AlterColumnOption> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn alter_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ALTER_KW)
    }
    #[inline]
    pub fn column_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::COLUMN_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AlterConstraint {
    pub(crate) syntax: SyntaxNode,
}
impl AlterConstraint {
    #[inline]
    pub fn option(&self) -> Option<AlterColumnOption> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn alter_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ALTER_KW)
    }
    #[inline]
    pub fn constraint_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CONSTRAINT_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AlterConversion {
    pub(crate) syntax: SyntaxNode,
}
impl AlterConversion {
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn alter_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ALTER_KW)
    }
    #[inline]
    pub fn conversion_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CONVERSION_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AlterDatabase {
    pub(crate) syntax: SyntaxNode,
}
impl AlterDatabase {
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn alter_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ALTER_KW)
    }
    #[inline]
    pub fn database_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DATABASE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AlterDefaultPrivileges {
    pub(crate) syntax: SyntaxNode,
}
impl AlterDefaultPrivileges {
    #[inline]
    pub fn alter_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ALTER_KW)
    }
    #[inline]
    pub fn default_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DEFAULT_KW)
    }
    #[inline]
    pub fn privileges_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::PRIVILEGES_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AlterDomain {
    pub(crate) syntax: SyntaxNode,
}
impl AlterDomain {
    #[inline]
    pub fn action(&self) -> Option<AlterDomainAction> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn path(&self) -> Option<Path> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn alter_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ALTER_KW)
    }
    #[inline]
    pub fn domain_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DOMAIN_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AlterEventTrigger {
    pub(crate) syntax: SyntaxNode,
}
impl AlterEventTrigger {
    #[inline]
    pub fn alter_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ALTER_KW)
    }
    #[inline]
    pub fn event_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::EVENT_KW)
    }
    #[inline]
    pub fn trigger_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::TRIGGER_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AlterExtension {
    pub(crate) syntax: SyntaxNode,
}
impl AlterExtension {
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn alter_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ALTER_KW)
    }
    #[inline]
    pub fn extension_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::EXTENSION_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AlterForeignDataWrapper {
    pub(crate) syntax: SyntaxNode,
}
impl AlterForeignDataWrapper {
    #[inline]
    pub fn alter_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ALTER_KW)
    }
    #[inline]
    pub fn data_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DATA_KW)
    }
    #[inline]
    pub fn foreign_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::FOREIGN_KW)
    }
    #[inline]
    pub fn wrapper_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::WRAPPER_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AlterForeignTable {
    pub(crate) syntax: SyntaxNode,
}
impl AlterForeignTable {
    #[inline]
    pub fn alter_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ALTER_KW)
    }
    #[inline]
    pub fn foreign_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::FOREIGN_KW)
    }
    #[inline]
    pub fn table_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::TABLE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AlterFunction {
    pub(crate) syntax: SyntaxNode,
}
impl AlterFunction {
    #[inline]
    pub fn alter_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ALTER_KW)
    }
    #[inline]
    pub fn function_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::FUNCTION_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AlterGroup {
    pub(crate) syntax: SyntaxNode,
}
impl AlterGroup {
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn alter_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ALTER_KW)
    }
    #[inline]
    pub fn group_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::GROUP_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AlterIndex {
    pub(crate) syntax: SyntaxNode,
}
impl AlterIndex {
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn alter_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ALTER_KW)
    }
    #[inline]
    pub fn index_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::INDEX_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AlterLanguage {
    pub(crate) syntax: SyntaxNode,
}
impl AlterLanguage {
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn alter_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ALTER_KW)
    }
    #[inline]
    pub fn language_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::LANGUAGE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AlterLargeObject {
    pub(crate) syntax: SyntaxNode,
}
impl AlterLargeObject {
    #[inline]
    pub fn alter_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ALTER_KW)
    }
    #[inline]
    pub fn large_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::LARGE_KW)
    }
    #[inline]
    pub fn object_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::OBJECT_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AlterMaterializedView {
    pub(crate) syntax: SyntaxNode,
}
impl AlterMaterializedView {
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn alter_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ALTER_KW)
    }
    #[inline]
    pub fn materialized_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::MATERIALIZED_KW)
    }
    #[inline]
    pub fn view_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::VIEW_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AlterOperator {
    pub(crate) syntax: SyntaxNode,
}
impl AlterOperator {
    #[inline]
    pub fn alter_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ALTER_KW)
    }
    #[inline]
    pub fn operator_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::OPERATOR_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AlterOperatorClass {
    pub(crate) syntax: SyntaxNode,
}
impl AlterOperatorClass {
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn alter_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ALTER_KW)
    }
    #[inline]
    pub fn class_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CLASS_KW)
    }
    #[inline]
    pub fn operator_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::OPERATOR_KW)
    }
    #[inline]
    pub fn using_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::USING_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AlterOperatorFamily {
    pub(crate) syntax: SyntaxNode,
}
impl AlterOperatorFamily {
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn alter_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ALTER_KW)
    }
    #[inline]
    pub fn family_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::FAMILY_KW)
    }
    #[inline]
    pub fn operator_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::OPERATOR_KW)
    }
    #[inline]
    pub fn using_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::USING_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AlterPolicy {
    pub(crate) syntax: SyntaxNode,
}
impl AlterPolicy {
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn alter_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ALTER_KW)
    }
    #[inline]
    pub fn on_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ON_KW)
    }
    #[inline]
    pub fn policy_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::POLICY_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AlterProcedure {
    pub(crate) syntax: SyntaxNode,
}
impl AlterProcedure {
    #[inline]
    pub fn alter_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ALTER_KW)
    }
    #[inline]
    pub fn procedure_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::PROCEDURE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AlterPublication {
    pub(crate) syntax: SyntaxNode,
}
impl AlterPublication {
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn alter_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ALTER_KW)
    }
    #[inline]
    pub fn publication_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::PUBLICATION_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AlterRole {
    pub(crate) syntax: SyntaxNode,
}
impl AlterRole {
    #[inline]
    pub fn alter_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ALTER_KW)
    }
    #[inline]
    pub fn role_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ROLE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AlterRoutine {
    pub(crate) syntax: SyntaxNode,
}
impl AlterRoutine {
    #[inline]
    pub fn alter_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ALTER_KW)
    }
    #[inline]
    pub fn routine_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ROUTINE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AlterRule {
    pub(crate) syntax: SyntaxNode,
}
impl AlterRule {
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn alter_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ALTER_KW)
    }
    #[inline]
    pub fn on_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ON_KW)
    }
    #[inline]
    pub fn rule_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::RULE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AlterSchema {
    pub(crate) syntax: SyntaxNode,
}
impl AlterSchema {
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn alter_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ALTER_KW)
    }
    #[inline]
    pub fn rename_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::RENAME_KW)
    }
    #[inline]
    pub fn schema_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SCHEMA_KW)
    }
    #[inline]
    pub fn to_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::TO_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AlterSequence {
    pub(crate) syntax: SyntaxNode,
}
impl AlterSequence {
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn alter_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ALTER_KW)
    }
    #[inline]
    pub fn sequence_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SEQUENCE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AlterServer {
    pub(crate) syntax: SyntaxNode,
}
impl AlterServer {
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn alter_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ALTER_KW)
    }
    #[inline]
    pub fn server_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SERVER_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AlterStatistics {
    pub(crate) syntax: SyntaxNode,
}
impl AlterStatistics {
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn alter_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ALTER_KW)
    }
    #[inline]
    pub fn statistics_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::STATISTICS_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AlterSubscription {
    pub(crate) syntax: SyntaxNode,
}
impl AlterSubscription {
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn alter_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ALTER_KW)
    }
    #[inline]
    pub fn subscription_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SUBSCRIPTION_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AlterSystem {
    pub(crate) syntax: SyntaxNode,
}
impl AlterSystem {
    #[inline]
    pub fn alter_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ALTER_KW)
    }
    #[inline]
    pub fn set_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SET_KW)
    }
    #[inline]
    pub fn system_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SYSTEM_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AlterTable {
    pub(crate) syntax: SyntaxNode,
}
impl AlterTable {
    #[inline]
    pub fn actions(&self) -> AstChildren<AlterTableAction> {
        support::children(&self.syntax)
    }
    #[inline]
    pub fn relation_name(&self) -> Option<RelationName> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn alter_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ALTER_KW)
    }
    #[inline]
    pub fn table_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::TABLE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AlterTablespace {
    pub(crate) syntax: SyntaxNode,
}
impl AlterTablespace {
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn alter_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ALTER_KW)
    }
    #[inline]
    pub fn tablespace_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::TABLESPACE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AlterTextSearchConfiguration {
    pub(crate) syntax: SyntaxNode,
}
impl AlterTextSearchConfiguration {
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn alter_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ALTER_KW)
    }
    #[inline]
    pub fn configuration_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CONFIGURATION_KW)
    }
    #[inline]
    pub fn search_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SEARCH_KW)
    }
    #[inline]
    pub fn text_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::TEXT_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AlterTextSearchDictionary {
    pub(crate) syntax: SyntaxNode,
}
impl AlterTextSearchDictionary {
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn alter_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ALTER_KW)
    }
    #[inline]
    pub fn dictionary_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DICTIONARY_KW)
    }
    #[inline]
    pub fn search_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SEARCH_KW)
    }
    #[inline]
    pub fn text_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::TEXT_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AlterTextSearchParser {
    pub(crate) syntax: SyntaxNode,
}
impl AlterTextSearchParser {
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn alter_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ALTER_KW)
    }
    #[inline]
    pub fn parser_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::PARSER_KW)
    }
    #[inline]
    pub fn search_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SEARCH_KW)
    }
    #[inline]
    pub fn text_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::TEXT_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AlterTextSearchTemplate {
    pub(crate) syntax: SyntaxNode,
}
impl AlterTextSearchTemplate {
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn alter_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ALTER_KW)
    }
    #[inline]
    pub fn search_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SEARCH_KW)
    }
    #[inline]
    pub fn template_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::TEMPLATE_KW)
    }
    #[inline]
    pub fn text_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::TEXT_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AlterTrigger {
    pub(crate) syntax: SyntaxNode,
}
impl AlterTrigger {
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn alter_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ALTER_KW)
    }
    #[inline]
    pub fn on_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ON_KW)
    }
    #[inline]
    pub fn trigger_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::TRIGGER_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AlterType {
    pub(crate) syntax: SyntaxNode,
}
impl AlterType {
    #[inline]
    pub fn ty(&self) -> Option<Type> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn alter_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ALTER_KW)
    }
    #[inline]
    pub fn type_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::TYPE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AlterUser {
    pub(crate) syntax: SyntaxNode,
}
impl AlterUser {
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn alter_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ALTER_KW)
    }
    #[inline]
    pub fn user_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::USER_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AlterUserMapping {
    pub(crate) syntax: SyntaxNode,
}
impl AlterUserMapping {
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn alter_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ALTER_KW)
    }
    #[inline]
    pub fn for_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::FOR_KW)
    }
    #[inline]
    pub fn mapping_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::MAPPING_KW)
    }
    #[inline]
    pub fn server_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SERVER_KW)
    }
    #[inline]
    pub fn user_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::USER_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AlterView {
    pub(crate) syntax: SyntaxNode,
}
impl AlterView {
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn alter_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ALTER_KW)
    }
    #[inline]
    pub fn view_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::VIEW_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Analyze {
    pub(crate) syntax: SyntaxNode,
}
impl Analyze {
    #[inline]
    pub fn analyze_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ANALYZE_KW)
    }
    #[inline]
    pub fn verbose_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::VERBOSE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Arg {
    pub(crate) syntax: SyntaxNode,
}
impl Arg {
    #[inline]
    pub fn expr(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArgList {
    pub(crate) syntax: SyntaxNode,
}
impl ArgList {
    #[inline]
    pub fn args(&self) -> AstChildren<Expr> {
        support::children(&self.syntax)
    }
    #[inline]
    pub fn expr(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::R_PAREN)
    }
    #[inline]
    pub fn star_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::STAR)
    }
    #[inline]
    pub fn all_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ALL_KW)
    }
    #[inline]
    pub fn distinct_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DISTINCT_KW)
    }
    #[inline]
    pub fn variadic_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::VARIADIC_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArrayExpr {
    pub(crate) syntax: SyntaxNode,
}
impl ArrayExpr {
    #[inline]
    pub fn exprs(&self) -> AstChildren<Expr> {
        support::children(&self.syntax)
    }
    #[inline]
    pub fn select(&self) -> Option<Select> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::R_PAREN)
    }
    #[inline]
    pub fn l_brack_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::L_BRACK)
    }
    #[inline]
    pub fn r_brack_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::R_BRACK)
    }
    #[inline]
    pub fn array_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ARRAY_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArrayType {
    pub(crate) syntax: SyntaxNode,
}
impl ArrayType {
    #[inline]
    pub fn expr(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn ty(&self) -> Option<Type> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn l_brack_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::L_BRACK)
    }
    #[inline]
    pub fn r_brack_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::R_BRACK)
    }
    #[inline]
    pub fn array_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ARRAY_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AsFuncOption {
    pub(crate) syntax: SyntaxNode,
}
impl AsFuncOption {
    #[inline]
    pub fn definition(&self) -> Option<Literal> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn link_symbol(&self) -> Option<Literal> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn obj_file(&self) -> Option<Literal> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn comma_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::COMMA)
    }
    #[inline]
    pub fn as_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::AS_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AtTimeZone {
    pub(crate) syntax: SyntaxNode,
}
impl AtTimeZone {
    #[inline]
    pub fn at_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::AT_KW)
    }
    #[inline]
    pub fn time_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::TIME_KW)
    }
    #[inline]
    pub fn zone_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ZONE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AttachPartition {
    pub(crate) syntax: SyntaxNode,
}
impl AttachPartition {
    #[inline]
    pub fn attach_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ATTACH_KW)
    }
    #[inline]
    pub fn partition_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::PARTITION_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Begin {
    pub(crate) syntax: SyntaxNode,
}
impl Begin {
    #[inline]
    pub fn transaction_mode_list(&self) -> Option<TransactionModeList> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn begin_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::BEGIN_KW)
    }
    #[inline]
    pub fn start_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::START_KW)
    }
    #[inline]
    pub fn transaction_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::TRANSACTION_KW)
    }
    #[inline]
    pub fn work_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::WORK_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BeginFuncOption {
    pub(crate) syntax: SyntaxNode,
}
impl BeginFuncOption {
    #[inline]
    pub fn atomic_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ATOMIC_KW)
    }
    #[inline]
    pub fn begin_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::BEGIN_KW)
    }
    #[inline]
    pub fn end_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::END_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BetweenExpr {
    pub(crate) syntax: SyntaxNode,
}
impl BetweenExpr {
    #[inline]
    pub fn end(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn start(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn target(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn and_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::AND_KW)
    }
    #[inline]
    pub fn between_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::BETWEEN_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BinExpr {
    pub(crate) syntax: SyntaxNode,
}
impl BinExpr {
    #[inline]
    pub fn lhs(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn op(&self) -> Option<Op> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn rhs(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BitType {
    pub(crate) syntax: SyntaxNode,
}
impl BitType {
    #[inline]
    pub fn arg_list(&self) -> Option<ArgList> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn bit_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::BIT_KW)
    }
    #[inline]
    pub fn varying_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::VARYING_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Call {
    pub(crate) syntax: SyntaxNode,
}
impl Call {
    #[inline]
    pub fn call_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CALL_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CallExpr {
    pub(crate) syntax: SyntaxNode,
}
impl CallExpr {
    #[inline]
    pub fn arg_list(&self) -> Option<ArgList> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn expr(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Cascade {
    pub(crate) syntax: SyntaxNode,
}
impl Cascade {
    #[inline]
    pub fn cascade_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CASCADE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CaseExpr {
    pub(crate) syntax: SyntaxNode,
}
impl CaseExpr {
    #[inline]
    pub fn case_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CASE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CastExpr {
    pub(crate) syntax: SyntaxNode,
}
impl CastExpr {
    #[inline]
    pub fn colon_colon(&self) -> Option<ColonColon> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn expr(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn ty(&self) -> Option<Type> {
        support::child(&self.syntax)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CharType {
    pub(crate) syntax: SyntaxNode,
}
impl CharType {
    #[inline]
    pub fn arg_list(&self) -> Option<ArgList> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn char_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CHAR_KW)
    }
    #[inline]
    pub fn character_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CHARACTER_KW)
    }
    #[inline]
    pub fn nchar_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::NCHAR_KW)
    }
    #[inline]
    pub fn varchar_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::VARCHAR_KW)
    }
    #[inline]
    pub fn varying_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::VARYING_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CheckConstraint {
    pub(crate) syntax: SyntaxNode,
}
impl CheckConstraint {
    #[inline]
    pub fn expr(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::R_PAREN)
    }
    #[inline]
    pub fn check_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CHECK_KW)
    }
    #[inline]
    pub fn constraint_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CONSTRAINT_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Checkpoint {
    pub(crate) syntax: SyntaxNode,
}
impl Checkpoint {
    #[inline]
    pub fn checkpoint_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CHECKPOINT_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Close {
    pub(crate) syntax: SyntaxNode,
}
impl Close {
    #[inline]
    pub fn close_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CLOSE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Cluster {
    pub(crate) syntax: SyntaxNode,
}
impl Cluster {
    #[inline]
    pub fn cluster_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CLUSTER_KW)
    }
    #[inline]
    pub fn verbose_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::VERBOSE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ClusterOn {
    pub(crate) syntax: SyntaxNode,
}
impl ClusterOn {
    #[inline]
    pub fn cluster_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CLUSTER_KW)
    }
    #[inline]
    pub fn on_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ON_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Collate {
    pub(crate) syntax: SyntaxNode,
}
impl Collate {
    #[inline]
    pub fn path(&self) -> Option<Path> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn collate_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::COLLATE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ColonColon {
    pub(crate) syntax: SyntaxNode,
}
impl ColonColon {
    #[inline]
    pub fn colon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::COLON)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ColonEq {
    pub(crate) syntax: SyntaxNode,
}
impl ColonEq {
    #[inline]
    pub fn colon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::COLON)
    }
    #[inline]
    pub fn eq_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::EQ)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Column {
    pub(crate) syntax: SyntaxNode,
}
impl Column {
    #[inline]
    pub fn collate(&self) -> Option<Collate> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn index_expr(&self) -> Option<IndexExpr> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn name(&self) -> Option<Name> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn ty(&self) -> Option<Type> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn period_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::PERIOD_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ColumnList {
    pub(crate) syntax: SyntaxNode,
}
impl ColumnList {
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::R_PAREN)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CommentOn {
    pub(crate) syntax: SyntaxNode,
}
impl CommentOn {
    #[inline]
    pub fn comment_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::COMMENT_KW)
    }
    #[inline]
    pub fn on_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ON_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Commit {
    pub(crate) syntax: SyntaxNode,
}
impl Commit {
    #[inline]
    pub fn literal(&self) -> Option<Literal> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn and_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::AND_KW)
    }
    #[inline]
    pub fn chain_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CHAIN_KW)
    }
    #[inline]
    pub fn commit_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::COMMIT_KW)
    }
    #[inline]
    pub fn no_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::NO_KW)
    }
    #[inline]
    pub fn prepared_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::PREPARED_KW)
    }
    #[inline]
    pub fn transaction_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::TRANSACTION_KW)
    }
    #[inline]
    pub fn work_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::WORK_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CompoundSelect {
    pub(crate) syntax: SyntaxNode,
}
impl CompoundSelect {
    #[inline]
    pub fn select(&self) -> Option<Select> {
        support::child(&self.syntax)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ConstraintExclusions {
    pub(crate) syntax: SyntaxNode,
}
impl ConstraintExclusions {
    #[inline]
    pub fn exclude_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::EXCLUDE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ConstraintIncludeClause {
    pub(crate) syntax: SyntaxNode,
}
impl ConstraintIncludeClause {
    #[inline]
    pub fn include_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::INCLUDE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ConstraintIndexMethod {
    pub(crate) syntax: SyntaxNode,
}
impl ConstraintIndexMethod {
    #[inline]
    pub fn using_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::USING_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ConstraintIndexTablespace {
    pub(crate) syntax: SyntaxNode,
}
impl ConstraintIndexTablespace {
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn index_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::INDEX_KW)
    }
    #[inline]
    pub fn tablespace_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::TABLESPACE_KW)
    }
    #[inline]
    pub fn using_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::USING_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ConstraintStorageParams {
    pub(crate) syntax: SyntaxNode,
}
impl ConstraintStorageParams {
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::R_PAREN)
    }
    #[inline]
    pub fn with_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::WITH_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ConstraintWhereClause {
    pub(crate) syntax: SyntaxNode,
}
impl ConstraintWhereClause {
    #[inline]
    pub fn where_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::WHERE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Copy {
    pub(crate) syntax: SyntaxNode,
}
impl Copy {
    #[inline]
    pub fn copy_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::COPY_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CostFuncOption {
    pub(crate) syntax: SyntaxNode,
}
impl CostFuncOption {
    #[inline]
    pub fn cost_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::COST_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateAccessMethod {
    pub(crate) syntax: SyntaxNode,
}
impl CreateAccessMethod {
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn access_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ACCESS_KW)
    }
    #[inline]
    pub fn create_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CREATE_KW)
    }
    #[inline]
    pub fn method_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::METHOD_KW)
    }
    #[inline]
    pub fn type_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::TYPE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateAggregate {
    pub(crate) syntax: SyntaxNode,
}
impl CreateAggregate {
    #[inline]
    pub fn or_replace(&self) -> Option<OrReplace> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn param_list(&self) -> Option<ParamList> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn path(&self) -> Option<Path> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn aggregate_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::AGGREGATE_KW)
    }
    #[inline]
    pub fn create_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CREATE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateCast {
    pub(crate) syntax: SyntaxNode,
}
impl CreateCast {
    #[inline]
    pub fn ty(&self) -> Option<Type> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::R_PAREN)
    }
    #[inline]
    pub fn as_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::AS_KW)
    }
    #[inline]
    pub fn cast_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CAST_KW)
    }
    #[inline]
    pub fn create_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CREATE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateCollation {
    pub(crate) syntax: SyntaxNode,
}
impl CreateCollation {
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn collation_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::COLLATION_KW)
    }
    #[inline]
    pub fn create_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CREATE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateConversion {
    pub(crate) syntax: SyntaxNode,
}
impl CreateConversion {
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn conversion_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CONVERSION_KW)
    }
    #[inline]
    pub fn create_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CREATE_KW)
    }
    #[inline]
    pub fn for_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::FOR_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateDatabase {
    pub(crate) syntax: SyntaxNode,
}
impl CreateDatabase {
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn create_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CREATE_KW)
    }
    #[inline]
    pub fn database_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DATABASE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateDomain {
    pub(crate) syntax: SyntaxNode,
}
impl CreateDomain {
    #[inline]
    pub fn collate(&self) -> Option<Collate> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn constraints(&self) -> AstChildren<Constraint> {
        support::children(&self.syntax)
    }
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn ty(&self) -> Option<Type> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn as_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::AS_KW)
    }
    #[inline]
    pub fn create_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CREATE_KW)
    }
    #[inline]
    pub fn domain_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DOMAIN_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateEventTrigger {
    pub(crate) syntax: SyntaxNode,
}
impl CreateEventTrigger {
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn create_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CREATE_KW)
    }
    #[inline]
    pub fn event_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::EVENT_KW)
    }
    #[inline]
    pub fn on_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ON_KW)
    }
    #[inline]
    pub fn trigger_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::TRIGGER_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateExtension {
    pub(crate) syntax: SyntaxNode,
}
impl CreateExtension {
    #[inline]
    pub fn create_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CREATE_KW)
    }
    #[inline]
    pub fn extension_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::EXTENSION_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateForeignDataWrapper {
    pub(crate) syntax: SyntaxNode,
}
impl CreateForeignDataWrapper {
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn create_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CREATE_KW)
    }
    #[inline]
    pub fn data_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DATA_KW)
    }
    #[inline]
    pub fn foreign_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::FOREIGN_KW)
    }
    #[inline]
    pub fn wrapper_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::WRAPPER_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateForeignTable {
    pub(crate) syntax: SyntaxNode,
}
impl CreateForeignTable {
    #[inline]
    pub fn if_not_exists(&self) -> Option<IfNotExists> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn create_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CREATE_KW)
    }
    #[inline]
    pub fn foreign_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::FOREIGN_KW)
    }
    #[inline]
    pub fn table_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::TABLE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateFunction {
    pub(crate) syntax: SyntaxNode,
}
impl CreateFunction {
    #[inline]
    pub fn option_list(&self) -> Option<FuncOptionList> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn or_replace(&self) -> Option<OrReplace> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn param_list(&self) -> Option<ParamList> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn path(&self) -> Option<Path> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn ret_type(&self) -> Option<RetType> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn create_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CREATE_KW)
    }
    #[inline]
    pub fn function_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::FUNCTION_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateGroup {
    pub(crate) syntax: SyntaxNode,
}
impl CreateGroup {
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn create_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CREATE_KW)
    }
    #[inline]
    pub fn group_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::GROUP_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateIndex {
    pub(crate) syntax: SyntaxNode,
}
impl CreateIndex {
    #[inline]
    pub fn if_not_exists(&self) -> Option<IfNotExists> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn name(&self) -> Option<Name> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn relation_name(&self) -> Option<RelationName> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn concurrently_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CONCURRENTLY_KW)
    }
    #[inline]
    pub fn create_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CREATE_KW)
    }
    #[inline]
    pub fn index_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::INDEX_KW)
    }
    #[inline]
    pub fn on_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ON_KW)
    }
    #[inline]
    pub fn unique_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::UNIQUE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateLanguage {
    pub(crate) syntax: SyntaxNode,
}
impl CreateLanguage {
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn create_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CREATE_KW)
    }
    #[inline]
    pub fn language_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::LANGUAGE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateMaterializedView {
    pub(crate) syntax: SyntaxNode,
}
impl CreateMaterializedView {
    #[inline]
    pub fn create_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CREATE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateOperator {
    pub(crate) syntax: SyntaxNode,
}
impl CreateOperator {
    #[inline]
    pub fn create_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CREATE_KW)
    }
    #[inline]
    pub fn operator_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::OPERATOR_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateOperatorClass {
    pub(crate) syntax: SyntaxNode,
}
impl CreateOperatorClass {
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn ty(&self) -> Option<Type> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn class_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CLASS_KW)
    }
    #[inline]
    pub fn create_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CREATE_KW)
    }
    #[inline]
    pub fn default_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DEFAULT_KW)
    }
    #[inline]
    pub fn for_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::FOR_KW)
    }
    #[inline]
    pub fn operator_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::OPERATOR_KW)
    }
    #[inline]
    pub fn type_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::TYPE_KW)
    }
    #[inline]
    pub fn using_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::USING_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateOperatorFamily {
    pub(crate) syntax: SyntaxNode,
}
impl CreateOperatorFamily {
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn create_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CREATE_KW)
    }
    #[inline]
    pub fn family_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::FAMILY_KW)
    }
    #[inline]
    pub fn operator_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::OPERATOR_KW)
    }
    #[inline]
    pub fn using_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::USING_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreatePolicy {
    pub(crate) syntax: SyntaxNode,
}
impl CreatePolicy {
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn create_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CREATE_KW)
    }
    #[inline]
    pub fn on_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ON_KW)
    }
    #[inline]
    pub fn policy_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::POLICY_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateProcedure {
    pub(crate) syntax: SyntaxNode,
}
impl CreateProcedure {
    #[inline]
    pub fn create_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CREATE_KW)
    }
    #[inline]
    pub fn procedure_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::PROCEDURE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreatePublication {
    pub(crate) syntax: SyntaxNode,
}
impl CreatePublication {
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn create_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CREATE_KW)
    }
    #[inline]
    pub fn publication_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::PUBLICATION_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateRole {
    pub(crate) syntax: SyntaxNode,
}
impl CreateRole {
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn create_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CREATE_KW)
    }
    #[inline]
    pub fn role_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ROLE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateRule {
    pub(crate) syntax: SyntaxNode,
}
impl CreateRule {
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn as_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::AS_KW)
    }
    #[inline]
    pub fn create_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CREATE_KW)
    }
    #[inline]
    pub fn on_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ON_KW)
    }
    #[inline]
    pub fn rule_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::RULE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateSchema {
    pub(crate) syntax: SyntaxNode,
}
impl CreateSchema {
    #[inline]
    pub fn create_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CREATE_KW)
    }
    #[inline]
    pub fn schema_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SCHEMA_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateSequence {
    pub(crate) syntax: SyntaxNode,
}
impl CreateSequence {
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn create_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CREATE_KW)
    }
    #[inline]
    pub fn sequence_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SEQUENCE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateServer {
    pub(crate) syntax: SyntaxNode,
}
impl CreateServer {
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn create_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CREATE_KW)
    }
    #[inline]
    pub fn server_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SERVER_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateStatistics {
    pub(crate) syntax: SyntaxNode,
}
impl CreateStatistics {
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn create_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CREATE_KW)
    }
    #[inline]
    pub fn statistics_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::STATISTICS_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateSubscription {
    pub(crate) syntax: SyntaxNode,
}
impl CreateSubscription {
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn create_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CREATE_KW)
    }
    #[inline]
    pub fn subscription_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SUBSCRIPTION_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateTable {
    pub(crate) syntax: SyntaxNode,
}
impl CreateTable {
    #[inline]
    pub fn if_not_exists(&self) -> Option<IfNotExists> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn path(&self) -> Option<Path> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn table_args(&self) -> Option<TableArgs> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn create_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CREATE_KW)
    }
    #[inline]
    pub fn table_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::TABLE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateTableAs {
    pub(crate) syntax: SyntaxNode,
}
impl CreateTableAs {
    #[inline]
    pub fn create_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CREATE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateTablespace {
    pub(crate) syntax: SyntaxNode,
}
impl CreateTablespace {
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn create_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CREATE_KW)
    }
    #[inline]
    pub fn tablespace_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::TABLESPACE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateTextSearchConfiguration {
    pub(crate) syntax: SyntaxNode,
}
impl CreateTextSearchConfiguration {
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn configuration_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CONFIGURATION_KW)
    }
    #[inline]
    pub fn create_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CREATE_KW)
    }
    #[inline]
    pub fn search_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SEARCH_KW)
    }
    #[inline]
    pub fn text_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::TEXT_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateTextSearchDictionary {
    pub(crate) syntax: SyntaxNode,
}
impl CreateTextSearchDictionary {
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn create_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CREATE_KW)
    }
    #[inline]
    pub fn dictionary_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DICTIONARY_KW)
    }
    #[inline]
    pub fn search_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SEARCH_KW)
    }
    #[inline]
    pub fn text_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::TEXT_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateTextSearchParser {
    pub(crate) syntax: SyntaxNode,
}
impl CreateTextSearchParser {
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn create_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CREATE_KW)
    }
    #[inline]
    pub fn parser_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::PARSER_KW)
    }
    #[inline]
    pub fn search_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SEARCH_KW)
    }
    #[inline]
    pub fn text_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::TEXT_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateTextSearchTemplate {
    pub(crate) syntax: SyntaxNode,
}
impl CreateTextSearchTemplate {
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn create_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CREATE_KW)
    }
    #[inline]
    pub fn search_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SEARCH_KW)
    }
    #[inline]
    pub fn template_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::TEMPLATE_KW)
    }
    #[inline]
    pub fn text_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::TEXT_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateTransform {
    pub(crate) syntax: SyntaxNode,
}
impl CreateTransform {
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn ty(&self) -> Option<Type> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn create_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CREATE_KW)
    }
    #[inline]
    pub fn for_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::FOR_KW)
    }
    #[inline]
    pub fn language_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::LANGUAGE_KW)
    }
    #[inline]
    pub fn transform_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::TRANSFORM_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateTrigger {
    pub(crate) syntax: SyntaxNode,
}
impl CreateTrigger {
    #[inline]
    pub fn create_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CREATE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateType {
    pub(crate) syntax: SyntaxNode,
}
impl CreateType {
    #[inline]
    pub fn ty(&self) -> Option<Type> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn create_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CREATE_KW)
    }
    #[inline]
    pub fn type_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::TYPE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateUser {
    pub(crate) syntax: SyntaxNode,
}
impl CreateUser {
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn create_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CREATE_KW)
    }
    #[inline]
    pub fn user_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::USER_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateUserMapping {
    pub(crate) syntax: SyntaxNode,
}
impl CreateUserMapping {
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn create_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CREATE_KW)
    }
    #[inline]
    pub fn for_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::FOR_KW)
    }
    #[inline]
    pub fn mapping_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::MAPPING_KW)
    }
    #[inline]
    pub fn server_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SERVER_KW)
    }
    #[inline]
    pub fn user_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::USER_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateView {
    pub(crate) syntax: SyntaxNode,
}
impl CreateView {
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn create_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CREATE_KW)
    }
    #[inline]
    pub fn view_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::VIEW_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CustomOp {
    pub(crate) syntax: SyntaxNode,
}
impl CustomOp {
    #[inline]
    pub fn bang_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::BANG)
    }
    #[inline]
    pub fn pound_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::POUND)
    }
    #[inline]
    pub fn percent_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::PERCENT)
    }
    #[inline]
    pub fn amp_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::AMP)
    }
    #[inline]
    pub fn star_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::STAR)
    }
    #[inline]
    pub fn plus_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::PLUS)
    }
    #[inline]
    pub fn minus_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::MINUS)
    }
    #[inline]
    pub fn slash_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SLASH)
    }
    #[inline]
    pub fn l_angle_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::L_ANGLE)
    }
    #[inline]
    pub fn eq_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::EQ)
    }
    #[inline]
    pub fn r_angle_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::R_ANGLE)
    }
    #[inline]
    pub fn question_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::QUESTION)
    }
    #[inline]
    pub fn at_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::AT)
    }
    #[inline]
    pub fn caret_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CARET)
    }
    #[inline]
    pub fn backtick_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::BACKTICK)
    }
    #[inline]
    pub fn pipe_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::PIPE)
    }
    #[inline]
    pub fn tilde_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::TILDE)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Deallocate {
    pub(crate) syntax: SyntaxNode,
}
impl Deallocate {
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn all_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ALL_KW)
    }
    #[inline]
    pub fn deallocate_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DEALLOCATE_KW)
    }
    #[inline]
    pub fn prepare_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::PREPARE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Declare {
    pub(crate) syntax: SyntaxNode,
}
impl Declare {
    #[inline]
    pub fn declare_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DECLARE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DefaultConstraint {
    pub(crate) syntax: SyntaxNode,
}
impl DefaultConstraint {
    #[inline]
    pub fn expr(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn constraint_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CONSTRAINT_KW)
    }
    #[inline]
    pub fn default_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DEFAULT_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Deferrable {
    pub(crate) syntax: SyntaxNode,
}
impl Deferrable {
    #[inline]
    pub fn deferrable_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DEFERRABLE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DeferrableConstraintOption {
    pub(crate) syntax: SyntaxNode,
}
impl DeferrableConstraintOption {
    #[inline]
    pub fn deferrable_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DEFERRABLE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Delete {
    pub(crate) syntax: SyntaxNode,
}
impl Delete {
    #[inline]
    pub fn delete_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DELETE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DetachPartition {
    pub(crate) syntax: SyntaxNode,
}
impl DetachPartition {
    #[inline]
    pub fn detach_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DETACH_KW)
    }
    #[inline]
    pub fn partition_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::PARTITION_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DisableRls {
    pub(crate) syntax: SyntaxNode,
}
impl DisableRls {
    #[inline]
    pub fn disable_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DISABLE_KW)
    }
    #[inline]
    pub fn level_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::LEVEL_KW)
    }
    #[inline]
    pub fn row_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ROW_KW)
    }
    #[inline]
    pub fn security_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SECURITY_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DisableRule {
    pub(crate) syntax: SyntaxNode,
}
impl DisableRule {
    #[inline]
    pub fn disable_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DISABLE_KW)
    }
    #[inline]
    pub fn rule_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::RULE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DisableTrigger {
    pub(crate) syntax: SyntaxNode,
}
impl DisableTrigger {
    #[inline]
    pub fn disable_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DISABLE_KW)
    }
    #[inline]
    pub fn trigger_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::TRIGGER_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Discard {
    pub(crate) syntax: SyntaxNode,
}
impl Discard {
    #[inline]
    pub fn all_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ALL_KW)
    }
    #[inline]
    pub fn discard_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DISCARD_KW)
    }
    #[inline]
    pub fn plans_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::PLANS_KW)
    }
    #[inline]
    pub fn sequences_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SEQUENCES_KW)
    }
    #[inline]
    pub fn temp_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::TEMP_KW)
    }
    #[inline]
    pub fn temporary_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::TEMPORARY_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DistinctClause {
    pub(crate) syntax: SyntaxNode,
}
impl DistinctClause {
    #[inline]
    pub fn distinct_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DISTINCT_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Do {
    pub(crate) syntax: SyntaxNode,
}
impl Do {
    #[inline]
    pub fn do_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DO_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DoubleType {
    pub(crate) syntax: SyntaxNode,
}
impl DoubleType {
    #[inline]
    pub fn double_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DOUBLE_KW)
    }
    #[inline]
    pub fn precision_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::PRECISION_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DropAccessMethod {
    pub(crate) syntax: SyntaxNode,
}
impl DropAccessMethod {
    #[inline]
    pub fn if_exists(&self) -> Option<IfExists> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn access_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ACCESS_KW)
    }
    #[inline]
    pub fn drop_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DROP_KW)
    }
    #[inline]
    pub fn method_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::METHOD_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DropAggregate {
    pub(crate) syntax: SyntaxNode,
}
impl DropAggregate {
    #[inline]
    pub fn aggregates(&self) -> AstChildren<Aggregate> {
        support::children(&self.syntax)
    }
    #[inline]
    pub fn if_exists(&self) -> Option<IfExists> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn aggregate_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::AGGREGATE_KW)
    }
    #[inline]
    pub fn drop_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DROP_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DropCast {
    pub(crate) syntax: SyntaxNode,
}
impl DropCast {
    #[inline]
    pub fn if_exists(&self) -> Option<IfExists> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn ty(&self) -> Option<Type> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::R_PAREN)
    }
    #[inline]
    pub fn as_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::AS_KW)
    }
    #[inline]
    pub fn cast_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CAST_KW)
    }
    #[inline]
    pub fn drop_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DROP_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DropCollation {
    pub(crate) syntax: SyntaxNode,
}
impl DropCollation {
    #[inline]
    pub fn if_exists(&self) -> Option<IfExists> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn collation_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::COLLATION_KW)
    }
    #[inline]
    pub fn drop_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DROP_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DropColumn {
    pub(crate) syntax: SyntaxNode,
}
impl DropColumn {
    #[inline]
    pub fn if_exists(&self) -> Option<IfExists> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn column_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::COLUMN_KW)
    }
    #[inline]
    pub fn drop_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DROP_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DropConstraint {
    pub(crate) syntax: SyntaxNode,
}
impl DropConstraint {
    #[inline]
    pub fn constraint(&self) -> Option<Constraint> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn if_exists(&self) -> Option<IfExists> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn cascade_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CASCADE_KW)
    }
    #[inline]
    pub fn drop_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DROP_KW)
    }
    #[inline]
    pub fn restrict_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::RESTRICT_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DropConversion {
    pub(crate) syntax: SyntaxNode,
}
impl DropConversion {
    #[inline]
    pub fn if_exists(&self) -> Option<IfExists> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn conversion_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CONVERSION_KW)
    }
    #[inline]
    pub fn drop_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DROP_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DropDatabase {
    pub(crate) syntax: SyntaxNode,
}
impl DropDatabase {
    #[inline]
    pub fn if_exists(&self) -> Option<IfExists> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn database_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DATABASE_KW)
    }
    #[inline]
    pub fn drop_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DROP_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DropDefault {
    pub(crate) syntax: SyntaxNode,
}
impl DropDefault {
    #[inline]
    pub fn default_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DEFAULT_KW)
    }
    #[inline]
    pub fn drop_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DROP_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DropDomain {
    pub(crate) syntax: SyntaxNode,
}
impl DropDomain {
    #[inline]
    pub fn if_exists(&self) -> Option<IfExists> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn types(&self) -> AstChildren<Type> {
        support::children(&self.syntax)
    }
    #[inline]
    pub fn domain_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DOMAIN_KW)
    }
    #[inline]
    pub fn drop_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DROP_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DropEventTrigger {
    pub(crate) syntax: SyntaxNode,
}
impl DropEventTrigger {
    #[inline]
    pub fn if_exists(&self) -> Option<IfExists> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn drop_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DROP_KW)
    }
    #[inline]
    pub fn event_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::EVENT_KW)
    }
    #[inline]
    pub fn trigger_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::TRIGGER_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DropExpression {
    pub(crate) syntax: SyntaxNode,
}
impl DropExpression {
    #[inline]
    pub fn if_exists(&self) -> Option<IfExists> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn drop_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DROP_KW)
    }
    #[inline]
    pub fn expression_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::EXPRESSION_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DropExtension {
    pub(crate) syntax: SyntaxNode,
}
impl DropExtension {
    #[inline]
    pub fn if_exists(&self) -> Option<IfExists> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn name_refs(&self) -> AstChildren<NameRef> {
        support::children(&self.syntax)
    }
    #[inline]
    pub fn drop_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DROP_KW)
    }
    #[inline]
    pub fn extension_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::EXTENSION_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DropForeignDataWrapper {
    pub(crate) syntax: SyntaxNode,
}
impl DropForeignDataWrapper {
    #[inline]
    pub fn if_exists(&self) -> Option<IfExists> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn data_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DATA_KW)
    }
    #[inline]
    pub fn drop_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DROP_KW)
    }
    #[inline]
    pub fn foreign_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::FOREIGN_KW)
    }
    #[inline]
    pub fn wrapper_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::WRAPPER_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DropForeignTable {
    pub(crate) syntax: SyntaxNode,
}
impl DropForeignTable {
    #[inline]
    pub fn if_exists(&self) -> Option<IfExists> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn drop_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DROP_KW)
    }
    #[inline]
    pub fn foreign_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::FOREIGN_KW)
    }
    #[inline]
    pub fn table_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::TABLE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DropFunction {
    pub(crate) syntax: SyntaxNode,
}
impl DropFunction {
    #[inline]
    pub fn if_exists(&self) -> Option<IfExists> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn drop_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DROP_KW)
    }
    #[inline]
    pub fn function_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::FUNCTION_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DropGroup {
    pub(crate) syntax: SyntaxNode,
}
impl DropGroup {
    #[inline]
    pub fn if_exists(&self) -> Option<IfExists> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn drop_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DROP_KW)
    }
    #[inline]
    pub fn group_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::GROUP_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DropIdentity {
    pub(crate) syntax: SyntaxNode,
}
impl DropIdentity {
    #[inline]
    pub fn if_exists(&self) -> Option<IfExists> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn drop_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DROP_KW)
    }
    #[inline]
    pub fn identity_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::IDENTITY_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DropIndex {
    pub(crate) syntax: SyntaxNode,
}
impl DropIndex {
    #[inline]
    pub fn if_exists(&self) -> Option<IfExists> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn paths(&self) -> AstChildren<Path> {
        support::children(&self.syntax)
    }
    #[inline]
    pub fn concurrently_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CONCURRENTLY_KW)
    }
    #[inline]
    pub fn drop_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DROP_KW)
    }
    #[inline]
    pub fn index_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::INDEX_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DropLanguage {
    pub(crate) syntax: SyntaxNode,
}
impl DropLanguage {
    #[inline]
    pub fn if_exists(&self) -> Option<IfExists> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn drop_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DROP_KW)
    }
    #[inline]
    pub fn language_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::LANGUAGE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DropMaterializedView {
    pub(crate) syntax: SyntaxNode,
}
impl DropMaterializedView {
    #[inline]
    pub fn if_exists(&self) -> Option<IfExists> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn drop_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DROP_KW)
    }
    #[inline]
    pub fn materialized_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::MATERIALIZED_KW)
    }
    #[inline]
    pub fn view_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::VIEW_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DropNotNull {
    pub(crate) syntax: SyntaxNode,
}
impl DropNotNull {
    #[inline]
    pub fn drop_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DROP_KW)
    }
    #[inline]
    pub fn not_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::NOT_KW)
    }
    #[inline]
    pub fn null_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::NULL_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DropOperator {
    pub(crate) syntax: SyntaxNode,
}
impl DropOperator {
    #[inline]
    pub fn if_exists(&self) -> Option<IfExists> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn drop_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DROP_KW)
    }
    #[inline]
    pub fn operator_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::OPERATOR_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DropOperatorClass {
    pub(crate) syntax: SyntaxNode,
}
impl DropOperatorClass {
    #[inline]
    pub fn if_exists(&self) -> Option<IfExists> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn class_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CLASS_KW)
    }
    #[inline]
    pub fn drop_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DROP_KW)
    }
    #[inline]
    pub fn operator_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::OPERATOR_KW)
    }
    #[inline]
    pub fn using_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::USING_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DropOperatorFamily {
    pub(crate) syntax: SyntaxNode,
}
impl DropOperatorFamily {
    #[inline]
    pub fn if_exists(&self) -> Option<IfExists> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn drop_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DROP_KW)
    }
    #[inline]
    pub fn family_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::FAMILY_KW)
    }
    #[inline]
    pub fn operator_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::OPERATOR_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DropOwned {
    pub(crate) syntax: SyntaxNode,
}
impl DropOwned {
    #[inline]
    pub fn by_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::BY_KW)
    }
    #[inline]
    pub fn drop_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DROP_KW)
    }
    #[inline]
    pub fn owned_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::OWNED_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DropPolicy {
    pub(crate) syntax: SyntaxNode,
}
impl DropPolicy {
    #[inline]
    pub fn if_exists(&self) -> Option<IfExists> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn drop_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DROP_KW)
    }
    #[inline]
    pub fn on_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ON_KW)
    }
    #[inline]
    pub fn policy_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::POLICY_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DropProcedure {
    pub(crate) syntax: SyntaxNode,
}
impl DropProcedure {
    #[inline]
    pub fn if_exists(&self) -> Option<IfExists> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn drop_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DROP_KW)
    }
    #[inline]
    pub fn procedure_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::PROCEDURE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DropPublication {
    pub(crate) syntax: SyntaxNode,
}
impl DropPublication {
    #[inline]
    pub fn if_exists(&self) -> Option<IfExists> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn name_refs(&self) -> AstChildren<NameRef> {
        support::children(&self.syntax)
    }
    #[inline]
    pub fn drop_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DROP_KW)
    }
    #[inline]
    pub fn publication_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::PUBLICATION_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DropRole {
    pub(crate) syntax: SyntaxNode,
}
impl DropRole {
    #[inline]
    pub fn if_exists(&self) -> Option<IfExists> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn name_refs(&self) -> AstChildren<NameRef> {
        support::children(&self.syntax)
    }
    #[inline]
    pub fn drop_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DROP_KW)
    }
    #[inline]
    pub fn role_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ROLE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DropRoutine {
    pub(crate) syntax: SyntaxNode,
}
impl DropRoutine {
    #[inline]
    pub fn if_exists(&self) -> Option<IfExists> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn drop_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DROP_KW)
    }
    #[inline]
    pub fn routine_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ROUTINE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DropRule {
    pub(crate) syntax: SyntaxNode,
}
impl DropRule {
    #[inline]
    pub fn if_exists(&self) -> Option<IfExists> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn drop_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DROP_KW)
    }
    #[inline]
    pub fn on_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ON_KW)
    }
    #[inline]
    pub fn rule_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::RULE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DropSchema {
    pub(crate) syntax: SyntaxNode,
}
impl DropSchema {
    #[inline]
    pub fn if_exists(&self) -> Option<IfExists> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn drop_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DROP_KW)
    }
    #[inline]
    pub fn schema_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SCHEMA_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DropSequence {
    pub(crate) syntax: SyntaxNode,
}
impl DropSequence {
    #[inline]
    pub fn if_exists(&self) -> Option<IfExists> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn name_refs(&self) -> AstChildren<NameRef> {
        support::children(&self.syntax)
    }
    #[inline]
    pub fn drop_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DROP_KW)
    }
    #[inline]
    pub fn sequence_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SEQUENCE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DropServer {
    pub(crate) syntax: SyntaxNode,
}
impl DropServer {
    #[inline]
    pub fn if_exists(&self) -> Option<IfExists> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn drop_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DROP_KW)
    }
    #[inline]
    pub fn server_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SERVER_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DropStatistics {
    pub(crate) syntax: SyntaxNode,
}
impl DropStatistics {
    #[inline]
    pub fn if_exists(&self) -> Option<IfExists> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn drop_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DROP_KW)
    }
    #[inline]
    pub fn statistics_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::STATISTICS_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DropSubscription {
    pub(crate) syntax: SyntaxNode,
}
impl DropSubscription {
    #[inline]
    pub fn if_exists(&self) -> Option<IfExists> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn drop_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DROP_KW)
    }
    #[inline]
    pub fn subscription_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SUBSCRIPTION_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DropTable {
    pub(crate) syntax: SyntaxNode,
}
impl DropTable {
    #[inline]
    pub fn if_exists(&self) -> Option<IfExists> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn path(&self) -> Option<Path> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn comma_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::COMMA)
    }
    #[inline]
    pub fn cascade_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CASCADE_KW)
    }
    #[inline]
    pub fn drop_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DROP_KW)
    }
    #[inline]
    pub fn restrict_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::RESTRICT_KW)
    }
    #[inline]
    pub fn table_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::TABLE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DropTablespace {
    pub(crate) syntax: SyntaxNode,
}
impl DropTablespace {
    #[inline]
    pub fn if_exists(&self) -> Option<IfExists> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn drop_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DROP_KW)
    }
    #[inline]
    pub fn tablespace_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::TABLESPACE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DropTextSearchConfig {
    pub(crate) syntax: SyntaxNode,
}
impl DropTextSearchConfig {
    #[inline]
    pub fn if_exists(&self) -> Option<IfExists> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn configuration_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CONFIGURATION_KW)
    }
    #[inline]
    pub fn drop_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DROP_KW)
    }
    #[inline]
    pub fn search_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SEARCH_KW)
    }
    #[inline]
    pub fn text_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::TEXT_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DropTextSearchDict {
    pub(crate) syntax: SyntaxNode,
}
impl DropTextSearchDict {
    #[inline]
    pub fn if_exists(&self) -> Option<IfExists> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn dictionary_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DICTIONARY_KW)
    }
    #[inline]
    pub fn drop_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DROP_KW)
    }
    #[inline]
    pub fn search_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SEARCH_KW)
    }
    #[inline]
    pub fn text_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::TEXT_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DropTextSearchParser {
    pub(crate) syntax: SyntaxNode,
}
impl DropTextSearchParser {
    #[inline]
    pub fn if_exists(&self) -> Option<IfExists> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn drop_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DROP_KW)
    }
    #[inline]
    pub fn parser_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::PARSER_KW)
    }
    #[inline]
    pub fn search_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SEARCH_KW)
    }
    #[inline]
    pub fn text_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::TEXT_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DropTextSearchTemplate {
    pub(crate) syntax: SyntaxNode,
}
impl DropTextSearchTemplate {
    #[inline]
    pub fn if_exists(&self) -> Option<IfExists> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn drop_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DROP_KW)
    }
    #[inline]
    pub fn search_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SEARCH_KW)
    }
    #[inline]
    pub fn template_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::TEMPLATE_KW)
    }
    #[inline]
    pub fn text_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::TEXT_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DropTransform {
    pub(crate) syntax: SyntaxNode,
}
impl DropTransform {
    #[inline]
    pub fn if_exists(&self) -> Option<IfExists> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn drop_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DROP_KW)
    }
    #[inline]
    pub fn transform_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::TRANSFORM_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DropTrigger {
    pub(crate) syntax: SyntaxNode,
}
impl DropTrigger {
    #[inline]
    pub fn if_exists(&self) -> Option<IfExists> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn drop_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DROP_KW)
    }
    #[inline]
    pub fn on_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ON_KW)
    }
    #[inline]
    pub fn trigger_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::TRIGGER_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DropType {
    pub(crate) syntax: SyntaxNode,
}
impl DropType {
    #[inline]
    pub fn if_exists(&self) -> Option<IfExists> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn paths(&self) -> AstChildren<Path> {
        support::children(&self.syntax)
    }
    #[inline]
    pub fn cascade_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CASCADE_KW)
    }
    #[inline]
    pub fn drop_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DROP_KW)
    }
    #[inline]
    pub fn restrict_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::RESTRICT_KW)
    }
    #[inline]
    pub fn type_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::TYPE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DropUser {
    pub(crate) syntax: SyntaxNode,
}
impl DropUser {
    #[inline]
    pub fn if_exists(&self) -> Option<IfExists> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn name_refs(&self) -> AstChildren<NameRef> {
        support::children(&self.syntax)
    }
    #[inline]
    pub fn drop_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DROP_KW)
    }
    #[inline]
    pub fn user_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::USER_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DropUserMapping {
    pub(crate) syntax: SyntaxNode,
}
impl DropUserMapping {
    #[inline]
    pub fn if_exists(&self) -> Option<IfExists> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn drop_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DROP_KW)
    }
    #[inline]
    pub fn for_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::FOR_KW)
    }
    #[inline]
    pub fn mapping_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::MAPPING_KW)
    }
    #[inline]
    pub fn server_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SERVER_KW)
    }
    #[inline]
    pub fn user_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::USER_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DropView {
    pub(crate) syntax: SyntaxNode,
}
impl DropView {
    #[inline]
    pub fn if_exists(&self) -> Option<IfExists> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn drop_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DROP_KW)
    }
    #[inline]
    pub fn view_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::VIEW_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnableAlwaysRule {
    pub(crate) syntax: SyntaxNode,
}
impl EnableAlwaysRule {
    #[inline]
    pub fn always_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ALWAYS_KW)
    }
    #[inline]
    pub fn enable_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ENABLE_KW)
    }
    #[inline]
    pub fn rule_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::RULE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnableAlwaysTrigger {
    pub(crate) syntax: SyntaxNode,
}
impl EnableAlwaysTrigger {
    #[inline]
    pub fn always_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ALWAYS_KW)
    }
    #[inline]
    pub fn enable_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ENABLE_KW)
    }
    #[inline]
    pub fn trigger_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::TRIGGER_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnableReplicaRule {
    pub(crate) syntax: SyntaxNode,
}
impl EnableReplicaRule {
    #[inline]
    pub fn enable_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ENABLE_KW)
    }
    #[inline]
    pub fn replica_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::REPLICA_KW)
    }
    #[inline]
    pub fn rule_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::RULE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnableReplicaTrigger {
    pub(crate) syntax: SyntaxNode,
}
impl EnableReplicaTrigger {
    #[inline]
    pub fn enable_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ENABLE_KW)
    }
    #[inline]
    pub fn replica_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::REPLICA_KW)
    }
    #[inline]
    pub fn trigger_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::TRIGGER_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnableRls {
    pub(crate) syntax: SyntaxNode,
}
impl EnableRls {
    #[inline]
    pub fn enable_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ENABLE_KW)
    }
    #[inline]
    pub fn level_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::LEVEL_KW)
    }
    #[inline]
    pub fn row_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ROW_KW)
    }
    #[inline]
    pub fn security_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SECURITY_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnableRule {
    pub(crate) syntax: SyntaxNode,
}
impl EnableRule {
    #[inline]
    pub fn enable_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ENABLE_KW)
    }
    #[inline]
    pub fn rule_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::RULE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnableTrigger {
    pub(crate) syntax: SyntaxNode,
}
impl EnableTrigger {
    #[inline]
    pub fn enable_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ENABLE_KW)
    }
    #[inline]
    pub fn trigger_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::TRIGGER_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Enforced {
    pub(crate) syntax: SyntaxNode,
}
impl Enforced {
    #[inline]
    pub fn enforced_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ENFORCED_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExcludeConstraint {
    pub(crate) syntax: SyntaxNode,
}
impl ExcludeConstraint {
    #[inline]
    pub fn constraint_exclusions(&self) -> Option<ConstraintExclusions> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn constraint_index_method(&self) -> Option<ConstraintIndexMethod> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn exclude_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::EXCLUDE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Execute {
    pub(crate) syntax: SyntaxNode,
}
impl Execute {
    #[inline]
    pub fn execute_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::EXECUTE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Explain {
    pub(crate) syntax: SyntaxNode,
}
impl Explain {
    #[inline]
    pub fn explain_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::EXPLAIN_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FatArrow {
    pub(crate) syntax: SyntaxNode,
}
impl FatArrow {
    #[inline]
    pub fn eq_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::EQ)
    }
    #[inline]
    pub fn r_angle_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::R_ANGLE)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Fetch {
    pub(crate) syntax: SyntaxNode,
}
impl Fetch {
    #[inline]
    pub fn fetch_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::FETCH_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FetchClause {
    pub(crate) syntax: SyntaxNode,
}
impl FetchClause {
    #[inline]
    pub fn fetch_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::FETCH_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FieldExpr {
    pub(crate) syntax: SyntaxNode,
}
impl FieldExpr {
    #[inline]
    pub fn expr(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn star_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::STAR)
    }
    #[inline]
    pub fn dot_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DOT)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FilterClause {
    pub(crate) syntax: SyntaxNode,
}
impl FilterClause {
    #[inline]
    pub fn expr(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::R_PAREN)
    }
    #[inline]
    pub fn filter_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::FILTER_KW)
    }
    #[inline]
    pub fn where_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::WHERE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ForceRls {
    pub(crate) syntax: SyntaxNode,
}
impl ForceRls {
    #[inline]
    pub fn force_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::FORCE_KW)
    }
    #[inline]
    pub fn level_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::LEVEL_KW)
    }
    #[inline]
    pub fn row_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ROW_KW)
    }
    #[inline]
    pub fn security_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SECURITY_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ForeignKeyConstraint {
    pub(crate) syntax: SyntaxNode,
}
impl ForeignKeyConstraint {
    #[inline]
    pub fn from_columns(&self) -> Option<ColumnList> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn match_type(&self) -> Option<MatchType> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn on_delete_action(&self) -> Option<OnDeleteAction> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn on_update_action(&self) -> Option<OnUpdateAction> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn path(&self) -> Option<Path> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn to_columns(&self) -> Option<ColumnList> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn foreign_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::FOREIGN_KW)
    }
    #[inline]
    pub fn key_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::KEY_KW)
    }
    #[inline]
    pub fn references_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::REFERENCES_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FromClause {
    pub(crate) syntax: SyntaxNode,
}
impl FromClause {
    #[inline]
    pub fn from_items(&self) -> AstChildren<FromItem> {
        support::children(&self.syntax)
    }
    #[inline]
    pub fn from_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::FROM_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FromItem {
    pub(crate) syntax: SyntaxNode,
}
impl FromItem {
    #[inline]
    pub fn only_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ONLY_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FuncOptionList {
    pub(crate) syntax: SyntaxNode,
}
impl FuncOptionList {
    #[inline]
    pub fn options(&self) -> AstChildren<FuncOption> {
        support::children(&self.syntax)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GeneratedConstraint {
    pub(crate) syntax: SyntaxNode,
}
impl GeneratedConstraint {
    #[inline]
    pub fn expr(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn sequence_option_list(&self) -> Option<SequenceOptionList> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::R_PAREN)
    }
    #[inline]
    pub fn always_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ALWAYS_KW)
    }
    #[inline]
    pub fn as_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::AS_KW)
    }
    #[inline]
    pub fn by_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::BY_KW)
    }
    #[inline]
    pub fn constraint_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CONSTRAINT_KW)
    }
    #[inline]
    pub fn default_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DEFAULT_KW)
    }
    #[inline]
    pub fn generated_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::GENERATED_KW)
    }
    #[inline]
    pub fn identity_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::IDENTITY_KW)
    }
    #[inline]
    pub fn stored_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::STORED_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Grant {
    pub(crate) syntax: SyntaxNode,
}
impl Grant {
    #[inline]
    pub fn grant_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::GRANT_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GroupByClause {
    pub(crate) syntax: SyntaxNode,
}
impl GroupByClause {
    #[inline]
    pub fn group_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::GROUP_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Gteq {
    pub(crate) syntax: SyntaxNode,
}
impl Gteq {
    #[inline]
    pub fn eq_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::EQ)
    }
    #[inline]
    pub fn r_angle_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::R_ANGLE)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HavingClause {
    pub(crate) syntax: SyntaxNode,
}
impl HavingClause {
    #[inline]
    pub fn having_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::HAVING_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IfExists {
    pub(crate) syntax: SyntaxNode,
}
impl IfExists {
    #[inline]
    pub fn exists_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::EXISTS_KW)
    }
    #[inline]
    pub fn if_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::IF_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IfNotExists {
    pub(crate) syntax: SyntaxNode,
}
impl IfNotExists {
    #[inline]
    pub fn exists_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::EXISTS_KW)
    }
    #[inline]
    pub fn if_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::IF_KW)
    }
    #[inline]
    pub fn not_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::NOT_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImportForeignSchema {
    pub(crate) syntax: SyntaxNode,
}
impl ImportForeignSchema {
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn foreign_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::FOREIGN_KW)
    }
    #[inline]
    pub fn import_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::IMPORT_KW)
    }
    #[inline]
    pub fn schema_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SCHEMA_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IndexExpr {
    pub(crate) syntax: SyntaxNode,
}
impl IndexExpr {
    #[inline]
    pub fn base(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn index(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn l_brack_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::L_BRACK)
    }
    #[inline]
    pub fn r_brack_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::R_BRACK)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IndexParams {
    pub(crate) syntax: SyntaxNode,
}
impl IndexParams {
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::R_PAREN)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Inherit {
    pub(crate) syntax: SyntaxNode,
}
impl Inherit {
    #[inline]
    pub fn inherit_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::INHERIT_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InitiallyDeferredConstraintOption {
    pub(crate) syntax: SyntaxNode,
}
impl InitiallyDeferredConstraintOption {
    #[inline]
    pub fn deferred_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DEFERRED_KW)
    }
    #[inline]
    pub fn initially_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::INITIALLY_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InitiallyImmediateConstraintOption {
    pub(crate) syntax: SyntaxNode,
}
impl InitiallyImmediateConstraintOption {
    #[inline]
    pub fn immediate_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::IMMEDIATE_KW)
    }
    #[inline]
    pub fn initially_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::INITIALLY_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Insert {
    pub(crate) syntax: SyntaxNode,
}
impl Insert {
    #[inline]
    pub fn insert_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::INSERT_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IntervalType {
    pub(crate) syntax: SyntaxNode,
}
impl IntervalType {
    #[inline]
    pub fn literal(&self) -> Option<Literal> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::R_PAREN)
    }
    #[inline]
    pub fn day_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DAY_KW)
    }
    #[inline]
    pub fn hour_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::HOUR_KW)
    }
    #[inline]
    pub fn interval_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::INTERVAL_KW)
    }
    #[inline]
    pub fn minute_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::MINUTE_KW)
    }
    #[inline]
    pub fn month_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::MONTH_KW)
    }
    #[inline]
    pub fn second_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SECOND_KW)
    }
    #[inline]
    pub fn to_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::TO_KW)
    }
    #[inline]
    pub fn year_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::YEAR_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IntoClause {
    pub(crate) syntax: SyntaxNode,
}
impl IntoClause {
    #[inline]
    pub fn into_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::INTO_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IsDistinctFrom {
    pub(crate) syntax: SyntaxNode,
}
impl IsDistinctFrom {
    #[inline]
    pub fn distinct_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DISTINCT_KW)
    }
    #[inline]
    pub fn from_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::FROM_KW)
    }
    #[inline]
    pub fn is_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::IS_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IsNormalized {
    pub(crate) syntax: SyntaxNode,
}
impl IsNormalized {
    #[inline]
    pub fn unicode_normal_form(&self) -> Option<UnicodeNormalForm> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn is_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::IS_KW)
    }
    #[inline]
    pub fn normalized_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::NORMALIZED_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IsNot {
    pub(crate) syntax: SyntaxNode,
}
impl IsNot {
    #[inline]
    pub fn is_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::IS_KW)
    }
    #[inline]
    pub fn not_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::NOT_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IsNotDistinctFrom {
    pub(crate) syntax: SyntaxNode,
}
impl IsNotDistinctFrom {
    #[inline]
    pub fn distinct_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DISTINCT_KW)
    }
    #[inline]
    pub fn from_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::FROM_KW)
    }
    #[inline]
    pub fn is_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::IS_KW)
    }
    #[inline]
    pub fn not_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::NOT_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IsNotNormalized {
    pub(crate) syntax: SyntaxNode,
}
impl IsNotNormalized {
    #[inline]
    pub fn unicode_normal_form(&self) -> Option<UnicodeNormalForm> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn is_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::IS_KW)
    }
    #[inline]
    pub fn normalized_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::NORMALIZED_KW)
    }
    #[inline]
    pub fn not_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::NOT_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Join {
    pub(crate) syntax: SyntaxNode,
}
impl Join {
    #[inline]
    pub fn cross_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CROSS_KW)
    }
    #[inline]
    pub fn full_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::FULL_KW)
    }
    #[inline]
    pub fn inner_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::INNER_KW)
    }
    #[inline]
    pub fn join_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::JOIN_KW)
    }
    #[inline]
    pub fn left_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::LEFT_KW)
    }
    #[inline]
    pub fn natural_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::NATURAL_KW)
    }
    #[inline]
    pub fn outer_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::OUTER_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsonBehaviorClause {
    pub(crate) syntax: SyntaxNode,
}
impl JsonBehaviorClause {
    #[inline]
    pub fn expr(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn default_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DEFAULT_KW)
    }
    #[inline]
    pub fn empty_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::EMPTY_KW)
    }
    #[inline]
    pub fn error_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ERROR_KW)
    }
    #[inline]
    pub fn null_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::NULL_KW)
    }
    #[inline]
    pub fn on_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ON_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsonFormatClause {
    pub(crate) syntax: SyntaxNode,
}
impl JsonFormatClause {
    #[inline]
    pub fn name(&self) -> Option<Name> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn encoding_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ENCODING_KW)
    }
    #[inline]
    pub fn format_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::FORMAT_KW)
    }
    #[inline]
    pub fn json_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::JSON_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsonKeyValue {
    pub(crate) syntax: SyntaxNode,
}
impl JsonKeyValue {
    #[inline]
    pub fn expr(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn json_value_expr(&self) -> Option<JsonValueExpr> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn colon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::COLON)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsonKeysUniqueClause {
    pub(crate) syntax: SyntaxNode,
}
impl JsonKeysUniqueClause {
    #[inline]
    pub fn keys_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::KEYS_KW)
    }
    #[inline]
    pub fn unique_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::UNIQUE_KW)
    }
    #[inline]
    pub fn with_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::WITH_KW)
    }
    #[inline]
    pub fn without_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::WITHOUT_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsonNullClause {
    pub(crate) syntax: SyntaxNode,
}
impl JsonNullClause {
    #[inline]
    pub fn absent_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ABSENT_KW)
    }
    #[inline]
    pub fn null_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::NULL_KW)
    }
    #[inline]
    pub fn on_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ON_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsonOnErrorClause {
    pub(crate) syntax: SyntaxNode,
}
impl JsonOnErrorClause {
    #[inline]
    pub fn empty_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::EMPTY_KW)
    }
    #[inline]
    pub fn error_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ERROR_KW)
    }
    #[inline]
    pub fn null_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::NULL_KW)
    }
    #[inline]
    pub fn on_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ON_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsonPassingClause {
    pub(crate) syntax: SyntaxNode,
}
impl JsonPassingClause {
    #[inline]
    pub fn named_args(&self) -> AstChildren<NamedArg> {
        support::children(&self.syntax)
    }
    #[inline]
    pub fn passing_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::PASSING_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsonQuotesClause {
    pub(crate) syntax: SyntaxNode,
}
impl JsonQuotesClause {
    #[inline]
    pub fn keep_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::KEEP_KW)
    }
    #[inline]
    pub fn omit_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::OMIT_KW)
    }
    #[inline]
    pub fn quotes_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::QUOTES_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsonReturningClause {
    pub(crate) syntax: SyntaxNode,
}
impl JsonReturningClause {
    #[inline]
    pub fn ty(&self) -> Option<Type> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn returning_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::RETURNING_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsonValueExpr {
    pub(crate) syntax: SyntaxNode,
}
impl JsonValueExpr {
    #[inline]
    pub fn expr(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn json_format_clause(&self) -> Option<JsonFormatClause> {
        support::child(&self.syntax)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsonWrapperBehaviorClause {
    pub(crate) syntax: SyntaxNode,
}
impl JsonWrapperBehaviorClause {
    #[inline]
    pub fn conditional_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CONDITIONAL_KW)
    }
    #[inline]
    pub fn with_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::WITH_KW)
    }
    #[inline]
    pub fn without_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::WITHOUT_KW)
    }
    #[inline]
    pub fn wrapper_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::WRAPPER_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LanguageFuncOption {
    pub(crate) syntax: SyntaxNode,
}
impl LanguageFuncOption {
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn language_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::LANGUAGE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LeakproofFuncOption {
    pub(crate) syntax: SyntaxNode,
}
impl LeakproofFuncOption {
    #[inline]
    pub fn leakproof_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::LEAKPROOF_KW)
    }
    #[inline]
    pub fn not_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::NOT_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LikeClause {
    pub(crate) syntax: SyntaxNode,
}
impl LikeClause {
    #[inline]
    pub fn like_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::LIKE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LimitClause {
    pub(crate) syntax: SyntaxNode,
}
impl LimitClause {
    #[inline]
    pub fn limit_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::LIMIT_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Listen {
    pub(crate) syntax: SyntaxNode,
}
impl Listen {
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn listen_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::LISTEN_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Literal {
    pub(crate) syntax: SyntaxNode,
}
impl Literal {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Load {
    pub(crate) syntax: SyntaxNode,
}
impl Load {
    #[inline]
    pub fn load_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::LOAD_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Lock {
    pub(crate) syntax: SyntaxNode,
}
impl Lock {
    #[inline]
    pub fn lock_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::LOCK_KW)
    }
    #[inline]
    pub fn table_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::TABLE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LockingClause {
    pub(crate) syntax: SyntaxNode,
}
impl LockingClause {
    #[inline]
    pub fn for_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::FOR_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Lteq {
    pub(crate) syntax: SyntaxNode,
}
impl Lteq {
    #[inline]
    pub fn l_angle_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::L_ANGLE)
    }
    #[inline]
    pub fn eq_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::EQ)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MatchFull {
    pub(crate) syntax: SyntaxNode,
}
impl MatchFull {
    #[inline]
    pub fn full_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::FULL_KW)
    }
    #[inline]
    pub fn match_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::MATCH_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MatchPartial {
    pub(crate) syntax: SyntaxNode,
}
impl MatchPartial {
    #[inline]
    pub fn match_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::MATCH_KW)
    }
    #[inline]
    pub fn partial_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::PARTIAL_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MatchSimple {
    pub(crate) syntax: SyntaxNode,
}
impl MatchSimple {
    #[inline]
    pub fn match_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::MATCH_KW)
    }
    #[inline]
    pub fn simple_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SIMPLE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Merge {
    pub(crate) syntax: SyntaxNode,
}
impl Merge {
    #[inline]
    pub fn merge_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::MERGE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Move {
    pub(crate) syntax: SyntaxNode,
}
impl Move {
    #[inline]
    pub fn move_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::MOVE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Name {
    pub(crate) syntax: SyntaxNode,
}
impl Name {
    #[inline]
    pub fn ident_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::IDENT)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NameRef {
    pub(crate) syntax: SyntaxNode,
}
impl NameRef {
    #[inline]
    pub fn ident_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::IDENT)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NamedArg {
    pub(crate) syntax: SyntaxNode,
}
impl NamedArg {
    #[inline]
    pub fn expr(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn fat_arrow(&self) -> Option<FatArrow> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Neq {
    pub(crate) syntax: SyntaxNode,
}
impl Neq {
    #[inline]
    pub fn bang_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::BANG)
    }
    #[inline]
    pub fn eq_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::EQ)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Neqb {
    pub(crate) syntax: SyntaxNode,
}
impl Neqb {
    #[inline]
    pub fn l_angle_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::L_ANGLE)
    }
    #[inline]
    pub fn r_angle_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::R_ANGLE)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NoAction {
    pub(crate) syntax: SyntaxNode,
}
impl NoAction {
    #[inline]
    pub fn action_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ACTION_KW)
    }
    #[inline]
    pub fn no_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::NO_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NoForceRls {
    pub(crate) syntax: SyntaxNode,
}
impl NoForceRls {
    #[inline]
    pub fn force_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::FORCE_KW)
    }
    #[inline]
    pub fn level_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::LEVEL_KW)
    }
    #[inline]
    pub fn no_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::NO_KW)
    }
    #[inline]
    pub fn row_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ROW_KW)
    }
    #[inline]
    pub fn security_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SECURITY_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NoInherit {
    pub(crate) syntax: SyntaxNode,
}
impl NoInherit {
    #[inline]
    pub fn inherit_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::INHERIT_KW)
    }
    #[inline]
    pub fn no_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::NO_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NotDeferrable {
    pub(crate) syntax: SyntaxNode,
}
impl NotDeferrable {
    #[inline]
    pub fn deferrable_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DEFERRABLE_KW)
    }
    #[inline]
    pub fn not_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::NOT_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NotDeferrableConstraintOption {
    pub(crate) syntax: SyntaxNode,
}
impl NotDeferrableConstraintOption {
    #[inline]
    pub fn deferrable_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DEFERRABLE_KW)
    }
    #[inline]
    pub fn not_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::NOT_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NotEnforced {
    pub(crate) syntax: SyntaxNode,
}
impl NotEnforced {
    #[inline]
    pub fn enforced_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ENFORCED_KW)
    }
    #[inline]
    pub fn not_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::NOT_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NotIlike {
    pub(crate) syntax: SyntaxNode,
}
impl NotIlike {
    #[inline]
    pub fn ilike_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ILIKE_KW)
    }
    #[inline]
    pub fn not_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::NOT_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NotIn {
    pub(crate) syntax: SyntaxNode,
}
impl NotIn {
    #[inline]
    pub fn in_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::IN_KW)
    }
    #[inline]
    pub fn not_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::NOT_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NotLike {
    pub(crate) syntax: SyntaxNode,
}
impl NotLike {
    #[inline]
    pub fn like_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::LIKE_KW)
    }
    #[inline]
    pub fn not_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::NOT_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NotNullConstraint {
    pub(crate) syntax: SyntaxNode,
}
impl NotNullConstraint {
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn constraint_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CONSTRAINT_KW)
    }
    #[inline]
    pub fn not_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::NOT_KW)
    }
    #[inline]
    pub fn null_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::NULL_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NotOf {
    pub(crate) syntax: SyntaxNode,
}
impl NotOf {
    #[inline]
    pub fn not_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::NOT_KW)
    }
    #[inline]
    pub fn of_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::OF_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NotValid {
    pub(crate) syntax: SyntaxNode,
}
impl NotValid {
    #[inline]
    pub fn not_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::NOT_KW)
    }
    #[inline]
    pub fn valid_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::VALID_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Notify {
    pub(crate) syntax: SyntaxNode,
}
impl Notify {
    #[inline]
    pub fn notify_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::NOTIFY_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NullConstraint {
    pub(crate) syntax: SyntaxNode,
}
impl NullConstraint {
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn constraint_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CONSTRAINT_KW)
    }
    #[inline]
    pub fn null_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::NULL_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OfType {
    pub(crate) syntax: SyntaxNode,
}
impl OfType {
    #[inline]
    pub fn ty(&self) -> Option<Type> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn of_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::OF_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OffsetClause {
    pub(crate) syntax: SyntaxNode,
}
impl OffsetClause {
    #[inline]
    pub fn offset_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::OFFSET_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OnDeleteAction {
    pub(crate) syntax: SyntaxNode,
}
impl OnDeleteAction {
    #[inline]
    pub fn ref_action(&self) -> Option<RefAction> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn delete_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DELETE_KW)
    }
    #[inline]
    pub fn on_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ON_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OnUpdateAction {
    pub(crate) syntax: SyntaxNode,
}
impl OnUpdateAction {
    #[inline]
    pub fn ref_action(&self) -> Option<RefAction> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn on_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ON_KW)
    }
    #[inline]
    pub fn update_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::UPDATE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Op {
    pub(crate) syntax: SyntaxNode,
}
impl Op {
    #[inline]
    pub fn at_time_zone(&self) -> Option<AtTimeZone> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn colon_colon(&self) -> Option<ColonColon> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn colon_eq(&self) -> Option<ColonEq> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn custom_op(&self) -> Option<CustomOp> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn fat_arrow(&self) -> Option<FatArrow> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn gteq(&self) -> Option<Gteq> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn is_distinct_from(&self) -> Option<IsDistinctFrom> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn is_not(&self) -> Option<IsNot> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn is_not_distinct_from(&self) -> Option<IsNotDistinctFrom> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn lteq(&self) -> Option<Lteq> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn neq(&self) -> Option<Neq> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn neqb(&self) -> Option<Neqb> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn not_ilike(&self) -> Option<NotIlike> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn not_in(&self) -> Option<NotIn> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn not_like(&self) -> Option<NotLike> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn operator_call(&self) -> Option<OperatorCall> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn similar_to(&self) -> Option<SimilarTo> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn percent_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::PERCENT)
    }
    #[inline]
    pub fn plus_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::PLUS)
    }
    #[inline]
    pub fn minus_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::MINUS)
    }
    #[inline]
    pub fn slash_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SLASH)
    }
    #[inline]
    pub fn colon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::COLON)
    }
    #[inline]
    pub fn l_angle_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::L_ANGLE)
    }
    #[inline]
    pub fn eq_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::EQ)
    }
    #[inline]
    pub fn r_angle_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::R_ANGLE)
    }
    #[inline]
    pub fn caret_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CARET)
    }
    #[inline]
    pub fn and_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::AND_KW)
    }
    #[inline]
    pub fn collate_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::COLLATE_KW)
    }
    #[inline]
    pub fn ilike_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ILIKE_KW)
    }
    #[inline]
    pub fn in_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::IN_KW)
    }
    #[inline]
    pub fn is_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::IS_KW)
    }
    #[inline]
    pub fn like_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::LIKE_KW)
    }
    #[inline]
    pub fn or_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::OR_KW)
    }
    #[inline]
    pub fn overlaps_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::OVERLAPS_KW)
    }
    #[inline]
    pub fn value_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::VALUE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OperatorCall {
    pub(crate) syntax: SyntaxNode,
}
impl OperatorCall {
    #[inline]
    pub fn op(&self) -> Option<Op> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn path(&self) -> Option<Path> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::R_PAREN)
    }
    #[inline]
    pub fn dot_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DOT)
    }
    #[inline]
    pub fn operator_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::OPERATOR_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OrReplace {
    pub(crate) syntax: SyntaxNode,
}
impl OrReplace {
    #[inline]
    pub fn or_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::OR_KW)
    }
    #[inline]
    pub fn replace_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::REPLACE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OrderByClause {
    pub(crate) syntax: SyntaxNode,
}
impl OrderByClause {
    #[inline]
    pub fn by_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::BY_KW)
    }
    #[inline]
    pub fn order_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ORDER_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OverClause {
    pub(crate) syntax: SyntaxNode,
}
impl OverClause {
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::R_PAREN)
    }
    #[inline]
    pub fn over_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::OVER_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OwnerTo {
    pub(crate) syntax: SyntaxNode,
}
impl OwnerTo {
    #[inline]
    pub fn role(&self) -> Option<Role> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn owner_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::OWNER_KW)
    }
    #[inline]
    pub fn to_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::TO_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParallelFuncOption {
    pub(crate) syntax: SyntaxNode,
}
impl ParallelFuncOption {
    #[inline]
    pub fn ident_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::IDENT)
    }
    #[inline]
    pub fn parallel_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::PARALLEL_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Param {
    pub(crate) syntax: SyntaxNode,
}
impl Param {
    #[inline]
    pub fn mode(&self) -> Option<ParamMode> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn name(&self) -> Option<Name> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn param_default(&self) -> Option<ParamDefault> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn ty(&self) -> Option<Type> {
        support::child(&self.syntax)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParamDefault {
    pub(crate) syntax: SyntaxNode,
}
impl ParamDefault {
    #[inline]
    pub fn expr(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn eq_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::EQ)
    }
    #[inline]
    pub fn default_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DEFAULT_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParamIn {
    pub(crate) syntax: SyntaxNode,
}
impl ParamIn {
    #[inline]
    pub fn in_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::IN_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParamInOut {
    pub(crate) syntax: SyntaxNode,
}
impl ParamInOut {
    #[inline]
    pub fn in_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::IN_KW)
    }
    #[inline]
    pub fn inout_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::INOUT_KW)
    }
    #[inline]
    pub fn out_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::OUT_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParamList {
    pub(crate) syntax: SyntaxNode,
}
impl ParamList {
    #[inline]
    pub fn params(&self) -> AstChildren<Param> {
        support::children(&self.syntax)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParamOut {
    pub(crate) syntax: SyntaxNode,
}
impl ParamOut {
    #[inline]
    pub fn out_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::OUT_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParamVariadic {
    pub(crate) syntax: SyntaxNode,
}
impl ParamVariadic {
    #[inline]
    pub fn variadic_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::VARIADIC_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParenExpr {
    pub(crate) syntax: SyntaxNode,
}
impl ParenExpr {
    #[inline]
    pub fn expr(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::R_PAREN)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Path {
    pub(crate) syntax: SyntaxNode,
}
impl Path {
    #[inline]
    pub fn qualifier(&self) -> Option<Path> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn segment(&self) -> Option<PathSegment> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn dot_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DOT)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PathSegment {
    pub(crate) syntax: SyntaxNode,
}
impl PathSegment {
    #[inline]
    pub fn name(&self) -> Option<Name> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PathType {
    pub(crate) syntax: SyntaxNode,
}
impl PathType {
    #[inline]
    pub fn arg_list(&self) -> Option<ArgList> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn path(&self) -> Option<Path> {
        support::child(&self.syntax)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PercentType {
    pub(crate) syntax: SyntaxNode,
}
impl PercentType {
    #[inline]
    pub fn percent_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::PERCENT)
    }
    #[inline]
    pub fn type_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::TYPE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PercentTypeClause {
    pub(crate) syntax: SyntaxNode,
}
impl PercentTypeClause {
    #[inline]
    pub fn path(&self) -> Option<Path> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn percent_type_clause(&self) -> Option<PercentTypeClause> {
        support::child(&self.syntax)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PostfixExpr {
    pub(crate) syntax: SyntaxNode,
}
impl PostfixExpr {
    #[inline]
    pub fn expr(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PrefixExpr {
    pub(crate) syntax: SyntaxNode,
}
impl PrefixExpr {
    #[inline]
    pub fn expr(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Prepare {
    pub(crate) syntax: SyntaxNode,
}
impl Prepare {
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn prepare_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::PREPARE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PrepareTransaction {
    pub(crate) syntax: SyntaxNode,
}
impl PrepareTransaction {
    #[inline]
    pub fn literal(&self) -> Option<Literal> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn prepare_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::PREPARE_KW)
    }
    #[inline]
    pub fn transaction_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::TRANSACTION_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PrimaryKeyConstraint {
    pub(crate) syntax: SyntaxNode,
}
impl PrimaryKeyConstraint {
    #[inline]
    pub fn column_list(&self) -> Option<ColumnList> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn index_params(&self) -> Option<IndexParams> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn using_index(&self) -> Option<UsingIndex> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn constraint_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CONSTRAINT_KW)
    }
    #[inline]
    pub fn key_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::KEY_KW)
    }
    #[inline]
    pub fn primary_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::PRIMARY_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReadCommitted {
    pub(crate) syntax: SyntaxNode,
}
impl ReadCommitted {
    #[inline]
    pub fn committed_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::COMMITTED_KW)
    }
    #[inline]
    pub fn read_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::READ_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReadOnly {
    pub(crate) syntax: SyntaxNode,
}
impl ReadOnly {
    #[inline]
    pub fn only_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ONLY_KW)
    }
    #[inline]
    pub fn read_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::READ_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReadUncommitted {
    pub(crate) syntax: SyntaxNode,
}
impl ReadUncommitted {
    #[inline]
    pub fn read_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::READ_KW)
    }
    #[inline]
    pub fn uncommitted_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::UNCOMMITTED_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReadWrite {
    pub(crate) syntax: SyntaxNode,
}
impl ReadWrite {
    #[inline]
    pub fn read_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::READ_KW)
    }
    #[inline]
    pub fn write_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::WRITE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Reassign {
    pub(crate) syntax: SyntaxNode,
}
impl Reassign {
    #[inline]
    pub fn reassign_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::REASSIGN_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReferencesConstraint {
    pub(crate) syntax: SyntaxNode,
}
impl ReferencesConstraint {
    #[inline]
    pub fn match_type(&self) -> Option<MatchType> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn on_delete_action(&self) -> Option<OnDeleteAction> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn on_update_action(&self) -> Option<OnUpdateAction> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn path(&self) -> Option<Path> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::R_PAREN)
    }
    #[inline]
    pub fn constraint_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CONSTRAINT_KW)
    }
    #[inline]
    pub fn references_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::REFERENCES_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Refresh {
    pub(crate) syntax: SyntaxNode,
}
impl Refresh {
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn concurrently_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CONCURRENTLY_KW)
    }
    #[inline]
    pub fn data_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DATA_KW)
    }
    #[inline]
    pub fn materialized_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::MATERIALIZED_KW)
    }
    #[inline]
    pub fn refresh_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::REFRESH_KW)
    }
    #[inline]
    pub fn view_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::VIEW_KW)
    }
    #[inline]
    pub fn with_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::WITH_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Reindex {
    pub(crate) syntax: SyntaxNode,
}
impl Reindex {
    #[inline]
    pub fn reindex_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::REINDEX_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RelationName {
    pub(crate) syntax: SyntaxNode,
}
impl RelationName {
    #[inline]
    pub fn path(&self) -> Option<Path> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::R_PAREN)
    }
    #[inline]
    pub fn star_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::STAR)
    }
    #[inline]
    pub fn only_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ONLY_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReleaseSavepoint {
    pub(crate) syntax: SyntaxNode,
}
impl ReleaseSavepoint {
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn release_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::RELEASE_KW)
    }
    #[inline]
    pub fn savepoint_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SAVEPOINT_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RenameColumn {
    pub(crate) syntax: SyntaxNode,
}
impl RenameColumn {
    #[inline]
    pub fn column_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::COLUMN_KW)
    }
    #[inline]
    pub fn rename_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::RENAME_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RenameConstraint {
    pub(crate) syntax: SyntaxNode,
}
impl RenameConstraint {
    #[inline]
    pub fn name(&self) -> Option<Name> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn constraint_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CONSTRAINT_KW)
    }
    #[inline]
    pub fn rename_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::RENAME_KW)
    }
    #[inline]
    pub fn to_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::TO_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RenameTable {
    pub(crate) syntax: SyntaxNode,
}
impl RenameTable {
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn rename_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::RENAME_KW)
    }
    #[inline]
    pub fn to_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::TO_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RenameTo {
    pub(crate) syntax: SyntaxNode,
}
impl RenameTo {
    #[inline]
    pub fn name(&self) -> Option<Name> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn rename_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::RENAME_KW)
    }
    #[inline]
    pub fn to_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::TO_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RepeatableRead {
    pub(crate) syntax: SyntaxNode,
}
impl RepeatableRead {
    #[inline]
    pub fn read_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::READ_KW)
    }
    #[inline]
    pub fn repeatable_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::REPEATABLE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReplicaIdentity {
    pub(crate) syntax: SyntaxNode,
}
impl ReplicaIdentity {
    #[inline]
    pub fn identity_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::IDENTITY_KW)
    }
    #[inline]
    pub fn replica_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::REPLICA_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Reset {
    pub(crate) syntax: SyntaxNode,
}
impl Reset {
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn all_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ALL_KW)
    }
    #[inline]
    pub fn reset_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::RESET_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResetFuncOption {
    pub(crate) syntax: SyntaxNode,
}
impl ResetFuncOption {
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn reset_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::RESET_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResetOptions {
    pub(crate) syntax: SyntaxNode,
}
impl ResetOptions {
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::R_PAREN)
    }
    #[inline]
    pub fn reset_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::RESET_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResetStorageParams {
    pub(crate) syntax: SyntaxNode,
}
impl ResetStorageParams {
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::R_PAREN)
    }
    #[inline]
    pub fn reset_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::RESET_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Restart {
    pub(crate) syntax: SyntaxNode,
}
impl Restart {
    #[inline]
    pub fn restart_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::RESTART_KW)
    }
    #[inline]
    pub fn with_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::WITH_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Restrict {
    pub(crate) syntax: SyntaxNode,
}
impl Restrict {
    #[inline]
    pub fn restrict_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::RESTRICT_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RetType {
    pub(crate) syntax: SyntaxNode,
}
impl RetType {
    #[inline]
    pub fn ty(&self) -> Option<Type> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn returns_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::RETURNS_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReturnFuncOption {
    pub(crate) syntax: SyntaxNode,
}
impl ReturnFuncOption {
    #[inline]
    pub fn expr(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn return_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::RETURN_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Revoke {
    pub(crate) syntax: SyntaxNode,
}
impl Revoke {
    #[inline]
    pub fn revoke_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::REVOKE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Role {
    pub(crate) syntax: SyntaxNode,
}
impl Role {
    #[inline]
    pub fn current_role_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CURRENT_ROLE_KW)
    }
    #[inline]
    pub fn current_user_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CURRENT_USER_KW)
    }
    #[inline]
    pub fn group_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::GROUP_KW)
    }
    #[inline]
    pub fn ident_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::IDENT)
    }
    #[inline]
    pub fn session_user_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SESSION_USER_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Rollback {
    pub(crate) syntax: SyntaxNode,
}
impl Rollback {
    #[inline]
    pub fn abort_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ABORT_KW)
    }
    #[inline]
    pub fn rollback_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ROLLBACK_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RowsFuncOption {
    pub(crate) syntax: SyntaxNode,
}
impl RowsFuncOption {
    #[inline]
    pub fn rows_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ROWS_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Savepoint {
    pub(crate) syntax: SyntaxNode,
}
impl Savepoint {
    #[inline]
    pub fn savepoint_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SAVEPOINT_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SecurityFuncOption {
    pub(crate) syntax: SyntaxNode,
}
impl SecurityFuncOption {
    #[inline]
    pub fn definer_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DEFINER_KW)
    }
    #[inline]
    pub fn invoker_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::INVOKER_KW)
    }
    #[inline]
    pub fn security_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SECURITY_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SecurityLabel {
    pub(crate) syntax: SyntaxNode,
}
impl SecurityLabel {
    #[inline]
    pub fn label_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::LABEL_KW)
    }
    #[inline]
    pub fn security_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SECURITY_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Select {
    pub(crate) syntax: SyntaxNode,
}
impl Select {
    #[inline]
    pub fn fetch_clause(&self) -> Option<FetchClause> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn filter_clause(&self) -> Option<FilterClause> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn from_clause(&self) -> Option<FromClause> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn group_by_clause(&self) -> Option<GroupByClause> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn having_clause(&self) -> Option<HavingClause> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn limit_clause(&self) -> Option<LimitClause> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn locking_clauses(&self) -> AstChildren<LockingClause> {
        support::children(&self.syntax)
    }
    #[inline]
    pub fn offset_clause(&self) -> Option<OffsetClause> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn order_by_clause(&self) -> Option<OrderByClause> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn select_clause(&self) -> Option<SelectClause> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn where_clause(&self) -> Option<WhereClause> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn window_clause(&self) -> Option<WindowClause> {
        support::child(&self.syntax)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SelectClause {
    pub(crate) syntax: SyntaxNode,
}
impl SelectClause {
    #[inline]
    pub fn distinct_clause(&self) -> Option<DistinctClause> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn target_list(&self) -> Option<TargetList> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn all_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ALL_KW)
    }
    #[inline]
    pub fn select_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SELECT_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SelectInto {
    pub(crate) syntax: SyntaxNode,
}
impl SelectInto {
    #[inline]
    pub fn filter_clause(&self) -> Option<FilterClause> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn from_clause(&self) -> Option<FromClause> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn group_by_clause(&self) -> Option<GroupByClause> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn having_clause(&self) -> Option<HavingClause> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn into_clause(&self) -> Option<IntoClause> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn limit_clause(&self) -> Option<LimitClause> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn locking_clauses(&self) -> AstChildren<LockingClause> {
        support::children(&self.syntax)
    }
    #[inline]
    pub fn offset_clause(&self) -> Option<OffsetClause> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn order_by_clause(&self) -> Option<OrderByClause> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn select_clause(&self) -> Option<SelectClause> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn where_clause(&self) -> Option<WhereClause> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn window_clause(&self) -> Option<WindowClause> {
        support::child(&self.syntax)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SequenceOptionList {
    pub(crate) syntax: SyntaxNode,
}
impl SequenceOptionList {
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::R_PAREN)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Serializable {
    pub(crate) syntax: SyntaxNode,
}
impl Serializable {
    #[inline]
    pub fn serializable_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SERIALIZABLE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Set {
    pub(crate) syntax: SyntaxNode,
}
impl Set {
    #[inline]
    pub fn set_options(&self) -> Option<SetOptions> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn set_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SET_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SetAccessMethod {
    pub(crate) syntax: SyntaxNode,
}
impl SetAccessMethod {
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn access_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ACCESS_KW)
    }
    #[inline]
    pub fn method_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::METHOD_KW)
    }
    #[inline]
    pub fn set_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SET_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SetCompression {
    pub(crate) syntax: SyntaxNode,
}
impl SetCompression {
    #[inline]
    pub fn compression_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::COMPRESSION_KW)
    }
    #[inline]
    pub fn set_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SET_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SetConstraints {
    pub(crate) syntax: SyntaxNode,
}
impl SetConstraints {
    #[inline]
    pub fn constraints_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CONSTRAINTS_KW)
    }
    #[inline]
    pub fn set_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SET_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SetDefault {
    pub(crate) syntax: SyntaxNode,
}
impl SetDefault {
    #[inline]
    pub fn expr(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn default_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DEFAULT_KW)
    }
    #[inline]
    pub fn set_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SET_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SetDefaultColumns {
    pub(crate) syntax: SyntaxNode,
}
impl SetDefaultColumns {
    #[inline]
    pub fn column_list(&self) -> Option<ColumnList> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn default_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DEFAULT_KW)
    }
    #[inline]
    pub fn set_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SET_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SetExpression {
    pub(crate) syntax: SyntaxNode,
}
impl SetExpression {
    #[inline]
    pub fn expr(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn expression_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::EXPRESSION_KW)
    }
    #[inline]
    pub fn set_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SET_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SetFuncOption {
    pub(crate) syntax: SyntaxNode,
}
impl SetFuncOption {
    #[inline]
    pub fn set_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SET_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SetGenerated {
    pub(crate) syntax: SyntaxNode,
}
impl SetGenerated {
    #[inline]
    pub fn set_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SET_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SetGeneratedOptions {
    pub(crate) syntax: SyntaxNode,
}
impl SetGeneratedOptions {
    #[inline]
    pub fn set_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SET_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SetLogged {
    pub(crate) syntax: SyntaxNode,
}
impl SetLogged {
    #[inline]
    pub fn logged_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::LOGGED_KW)
    }
    #[inline]
    pub fn set_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SET_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SetNotNull {
    pub(crate) syntax: SyntaxNode,
}
impl SetNotNull {
    #[inline]
    pub fn not_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::NOT_KW)
    }
    #[inline]
    pub fn null_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::NULL_KW)
    }
    #[inline]
    pub fn set_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SET_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SetNullColumns {
    pub(crate) syntax: SyntaxNode,
}
impl SetNullColumns {
    #[inline]
    pub fn column_list(&self) -> Option<ColumnList> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn null_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::NULL_KW)
    }
    #[inline]
    pub fn set_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SET_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SetOptions {
    pub(crate) syntax: SyntaxNode,
}
impl SetOptions {
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::R_PAREN)
    }
    #[inline]
    pub fn set_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SET_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SetOptionsList {
    pub(crate) syntax: SyntaxNode,
}
impl SetOptionsList {
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::R_PAREN)
    }
    #[inline]
    pub fn set_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SET_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SetRole {
    pub(crate) syntax: SyntaxNode,
}
impl SetRole {
    #[inline]
    pub fn role_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ROLE_KW)
    }
    #[inline]
    pub fn set_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SET_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SetSchema {
    pub(crate) syntax: SyntaxNode,
}
impl SetSchema {
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn schema_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SCHEMA_KW)
    }
    #[inline]
    pub fn set_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SET_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SetSequenceOption {
    pub(crate) syntax: SyntaxNode,
}
impl SetSequenceOption {
    #[inline]
    pub fn set_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SET_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SetSessionAuth {
    pub(crate) syntax: SyntaxNode,
}
impl SetSessionAuth {
    #[inline]
    pub fn authorization_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::AUTHORIZATION_KW)
    }
    #[inline]
    pub fn session_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SESSION_KW)
    }
    #[inline]
    pub fn set_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SET_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SetStatistics {
    pub(crate) syntax: SyntaxNode,
}
impl SetStatistics {
    #[inline]
    pub fn set_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SET_KW)
    }
    #[inline]
    pub fn statistics_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::STATISTICS_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SetStorage {
    pub(crate) syntax: SyntaxNode,
}
impl SetStorage {
    #[inline]
    pub fn set_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SET_KW)
    }
    #[inline]
    pub fn storage_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::STORAGE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SetStorageParams {
    pub(crate) syntax: SyntaxNode,
}
impl SetStorageParams {
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::R_PAREN)
    }
    #[inline]
    pub fn set_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SET_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SetTablespace {
    pub(crate) syntax: SyntaxNode,
}
impl SetTablespace {
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn set_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SET_KW)
    }
    #[inline]
    pub fn tablespace_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::TABLESPACE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SetTransaction {
    pub(crate) syntax: SyntaxNode,
}
impl SetTransaction {
    #[inline]
    pub fn set_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SET_KW)
    }
    #[inline]
    pub fn transaction_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::TRANSACTION_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SetType {
    pub(crate) syntax: SyntaxNode,
}
impl SetType {
    #[inline]
    pub fn collate(&self) -> Option<Collate> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn ty(&self) -> Option<Type> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn set_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SET_KW)
    }
    #[inline]
    pub fn type_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::TYPE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SetUnlogged {
    pub(crate) syntax: SyntaxNode,
}
impl SetUnlogged {
    #[inline]
    pub fn set_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SET_KW)
    }
    #[inline]
    pub fn unlogged_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::UNLOGGED_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SetWithoutCluster {
    pub(crate) syntax: SyntaxNode,
}
impl SetWithoutCluster {
    #[inline]
    pub fn cluster_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CLUSTER_KW)
    }
    #[inline]
    pub fn set_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SET_KW)
    }
    #[inline]
    pub fn without_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::WITHOUT_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SetWithoutOids {
    pub(crate) syntax: SyntaxNode,
}
impl SetWithoutOids {
    #[inline]
    pub fn oids_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::OIDS_KW)
    }
    #[inline]
    pub fn set_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SET_KW)
    }
    #[inline]
    pub fn without_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::WITHOUT_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Show {
    pub(crate) syntax: SyntaxNode,
}
impl Show {
    #[inline]
    pub fn show_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SHOW_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SimilarTo {
    pub(crate) syntax: SyntaxNode,
}
impl SimilarTo {
    #[inline]
    pub fn similar_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SIMILAR_KW)
    }
    #[inline]
    pub fn to_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::TO_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SourceFile {
    pub(crate) syntax: SyntaxNode,
}
impl SourceFile {
    #[inline]
    pub fn stmts(&self) -> AstChildren<Stmt> {
        support::children(&self.syntax)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StrictFuncOption {
    pub(crate) syntax: SyntaxNode,
}
impl StrictFuncOption {
    #[inline]
    pub fn called_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CALLED_KW)
    }
    #[inline]
    pub fn input_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::INPUT_KW)
    }
    #[inline]
    pub fn null_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::NULL_KW)
    }
    #[inline]
    pub fn on_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ON_KW)
    }
    #[inline]
    pub fn returns_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::RETURNS_KW)
    }
    #[inline]
    pub fn strict_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::STRICT_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SupportFuncOption {
    pub(crate) syntax: SyntaxNode,
}
impl SupportFuncOption {
    #[inline]
    pub fn support_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SUPPORT_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Table {
    pub(crate) syntax: SyntaxNode,
}
impl Table {
    #[inline]
    pub fn table_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::TABLE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TableArgs {
    pub(crate) syntax: SyntaxNode,
}
impl TableArgs {
    #[inline]
    pub fn args(&self) -> AstChildren<TableArg> {
        support::children(&self.syntax)
    }
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::R_PAREN)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TableList {
    pub(crate) syntax: SyntaxNode,
}
impl TableList {
    #[inline]
    pub fn relation_names(&self) -> AstChildren<RelationName> {
        support::children(&self.syntax)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Target {
    pub(crate) syntax: SyntaxNode,
}
impl Target {
    #[inline]
    pub fn expr(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn star_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::STAR)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TargetList {
    pub(crate) syntax: SyntaxNode,
}
impl TargetList {
    #[inline]
    pub fn targets(&self) -> AstChildren<Target> {
        support::children(&self.syntax)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TimeType {
    pub(crate) syntax: SyntaxNode,
}
impl TimeType {
    #[inline]
    pub fn literal(&self) -> Option<Literal> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn with_timezone(&self) -> Option<WithTimezone> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn without_timezone(&self) -> Option<WithoutTimezone> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::R_PAREN)
    }
    #[inline]
    pub fn time_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::TIME_KW)
    }
    #[inline]
    pub fn timestamp_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::TIMESTAMP_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TransactionModeIsolationLevel {
    pub(crate) syntax: SyntaxNode,
}
impl TransactionModeIsolationLevel {
    #[inline]
    pub fn read_committed(&self) -> Option<ReadCommitted> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn read_uncommitted(&self) -> Option<ReadUncommitted> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn repeatable_read(&self) -> Option<RepeatableRead> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn serializable(&self) -> Option<Serializable> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn isolation_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ISOLATION_KW)
    }
    #[inline]
    pub fn level_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::LEVEL_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TransactionModeList {
    pub(crate) syntax: SyntaxNode,
}
impl TransactionModeList {
    #[inline]
    pub fn transaction_modes(&self) -> AstChildren<TransactionMode> {
        support::children(&self.syntax)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TransformFuncOption {
    pub(crate) syntax: SyntaxNode,
}
impl TransformFuncOption {
    #[inline]
    pub fn transform_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::TRANSFORM_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Truncate {
    pub(crate) syntax: SyntaxNode,
}
impl Truncate {
    #[inline]
    pub fn table_list(&self) -> Option<TableList> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn cascade_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CASCADE_KW)
    }
    #[inline]
    pub fn continue_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CONTINUE_KW)
    }
    #[inline]
    pub fn identity_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::IDENTITY_KW)
    }
    #[inline]
    pub fn restart_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::RESTART_KW)
    }
    #[inline]
    pub fn restrict_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::RESTRICT_KW)
    }
    #[inline]
    pub fn table_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::TABLE_KW)
    }
    #[inline]
    pub fn truncate_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::TRUNCATE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TupleExpr {
    pub(crate) syntax: SyntaxNode,
}
impl TupleExpr {
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::R_PAREN)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnicodeNormalForm {
    pub(crate) syntax: SyntaxNode,
}
impl UnicodeNormalForm {
    #[inline]
    pub fn nfc_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::NFC_KW)
    }
    #[inline]
    pub fn nfd_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::NFD_KW)
    }
    #[inline]
    pub fn nfkc_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::NFKC_KW)
    }
    #[inline]
    pub fn nfkd_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::NFKD_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UniqueConstraint {
    pub(crate) syntax: SyntaxNode,
}
impl UniqueConstraint {
    #[inline]
    pub fn column_list(&self) -> Option<ColumnList> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn using_index(&self) -> Option<UsingIndex> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn constraint_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CONSTRAINT_KW)
    }
    #[inline]
    pub fn distinct_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DISTINCT_KW)
    }
    #[inline]
    pub fn not_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::NOT_KW)
    }
    #[inline]
    pub fn nulls_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::NULLS_KW)
    }
    #[inline]
    pub fn unique_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::UNIQUE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Unlisten {
    pub(crate) syntax: SyntaxNode,
}
impl Unlisten {
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn star_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::STAR)
    }
    #[inline]
    pub fn unlisten_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::UNLISTEN_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Update {
    pub(crate) syntax: SyntaxNode,
}
impl Update {
    #[inline]
    pub fn update_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::UPDATE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UsingClause {
    pub(crate) syntax: SyntaxNode,
}
impl UsingClause {
    #[inline]
    pub fn using_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::USING_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UsingIndex {
    pub(crate) syntax: SyntaxNode,
}
impl UsingIndex {
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn index_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::INDEX_KW)
    }
    #[inline]
    pub fn using_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::USING_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Vacuum {
    pub(crate) syntax: SyntaxNode,
}
impl Vacuum {
    #[inline]
    pub fn vacuum_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::VACUUM_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ValidateConstraint {
    pub(crate) syntax: SyntaxNode,
}
impl ValidateConstraint {
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn constraint_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CONSTRAINT_KW)
    }
    #[inline]
    pub fn validate_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::VALIDATE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Values {
    pub(crate) syntax: SyntaxNode,
}
impl Values {
    #[inline]
    pub fn values_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::VALUES_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VolatilityFuncOption {
    pub(crate) syntax: SyntaxNode,
}
impl VolatilityFuncOption {
    #[inline]
    pub fn immutable_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::IMMUTABLE_KW)
    }
    #[inline]
    pub fn stable_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::STABLE_KW)
    }
    #[inline]
    pub fn volatile_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::VOLATILE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WhenClause {
    pub(crate) syntax: SyntaxNode,
}
impl WhenClause {
    #[inline]
    pub fn when_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::WHEN_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WhereClause {
    pub(crate) syntax: SyntaxNode,
}
impl WhereClause {
    #[inline]
    pub fn where_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::WHERE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WindowClause {
    pub(crate) syntax: SyntaxNode,
}
impl WindowClause {
    #[inline]
    pub fn ident_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::IDENT)
    }
    #[inline]
    pub fn window_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::WINDOW_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WindowDef {
    pub(crate) syntax: SyntaxNode,
}
impl WindowDef {
    #[inline]
    pub fn expr(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn by_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::BY_KW)
    }
    #[inline]
    pub fn ident_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::IDENT)
    }
    #[inline]
    pub fn partition_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::PARTITION_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WindowFuncOption {
    pub(crate) syntax: SyntaxNode,
}
impl WindowFuncOption {
    #[inline]
    pub fn window_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::WINDOW_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WithClause {
    pub(crate) syntax: SyntaxNode,
}
impl WithClause {
    #[inline]
    pub fn with_tables(&self) -> AstChildren<WithTable> {
        support::children(&self.syntax)
    }
    #[inline]
    pub fn recursive_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::RECURSIVE_KW)
    }
    #[inline]
    pub fn with_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::WITH_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WithTable {
    pub(crate) syntax: SyntaxNode,
}
impl WithTable {
    #[inline]
    pub fn with_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::WITH_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WithTimezone {
    pub(crate) syntax: SyntaxNode,
}
impl WithTimezone {
    #[inline]
    pub fn time_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::TIME_KW)
    }
    #[inline]
    pub fn with_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::WITH_KW)
    }
    #[inline]
    pub fn zone_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ZONE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WithinClause {
    pub(crate) syntax: SyntaxNode,
}
impl WithinClause {
    #[inline]
    pub fn order_by_clause(&self) -> Option<OrderByClause> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::L_PAREN)
    }
    #[inline]
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::R_PAREN)
    }
    #[inline]
    pub fn group_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::GROUP_KW)
    }
    #[inline]
    pub fn within_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::WITHIN_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WithoutTimezone {
    pub(crate) syntax: SyntaxNode,
}
impl WithoutTimezone {
    #[inline]
    pub fn time_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::TIME_KW)
    }
    #[inline]
    pub fn without_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::WITHOUT_KW)
    }
    #[inline]
    pub fn zone_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ZONE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AlterColumnOption {
    AddGenerated(AddGenerated),
    DropDefault(DropDefault),
    DropExpression(DropExpression),
    DropIdentity(DropIdentity),
    DropNotNull(DropNotNull),
    ResetOptions(ResetOptions),
    Restart(Restart),
    SetCompression(SetCompression),
    SetDefault(SetDefault),
    SetExpression(SetExpression),
    SetGenerated(SetGenerated),
    SetGeneratedOptions(SetGeneratedOptions),
    SetNotNull(SetNotNull),
    SetOptions(SetOptions),
    SetSequenceOption(SetSequenceOption),
    SetStatistics(SetStatistics),
    SetStorage(SetStorage),
    SetType(SetType),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AlterDomainAction {
    AddConstraint(AddConstraint),
    DropConstraint(DropConstraint),
    DropDefault(DropDefault),
    DropNotNull(DropNotNull),
    OwnerTo(OwnerTo),
    RenameConstraint(RenameConstraint),
    RenameTo(RenameTo),
    SetDefault(SetDefault),
    SetNotNull(SetNotNull),
    SetSchema(SetSchema),
    ValidateConstraint(ValidateConstraint),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AlterTableAction {
    AddColumn(AddColumn),
    AddConstraint(AddConstraint),
    AlterColumn(AlterColumn),
    AlterConstraint(AlterConstraint),
    AttachPartition(AttachPartition),
    ClusterOn(ClusterOn),
    DetachPartition(DetachPartition),
    DisableRls(DisableRls),
    DisableRule(DisableRule),
    DisableTrigger(DisableTrigger),
    DropColumn(DropColumn),
    DropConstraint(DropConstraint),
    EnableAlwaysRule(EnableAlwaysRule),
    EnableAlwaysTrigger(EnableAlwaysTrigger),
    EnableReplicaRule(EnableReplicaRule),
    EnableReplicaTrigger(EnableReplicaTrigger),
    EnableRls(EnableRls),
    EnableRule(EnableRule),
    EnableTrigger(EnableTrigger),
    ForceRls(ForceRls),
    Inherit(Inherit),
    NoForceRls(NoForceRls),
    NoInherit(NoInherit),
    NotOf(NotOf),
    OfType(OfType),
    OwnerTo(OwnerTo),
    RenameColumn(RenameColumn),
    RenameConstraint(RenameConstraint),
    RenameTable(RenameTable),
    ReplicaIdentity(ReplicaIdentity),
    ResetStorageParams(ResetStorageParams),
    SetAccessMethod(SetAccessMethod),
    SetLogged(SetLogged),
    SetSchema(SetSchema),
    SetStorageParams(SetStorageParams),
    SetTablespace(SetTablespace),
    SetUnlogged(SetUnlogged),
    SetWithoutCluster(SetWithoutCluster),
    SetWithoutOids(SetWithoutOids),
    ValidateConstraint(ValidateConstraint),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Constraint {
    CheckConstraint(CheckConstraint),
    DefaultConstraint(DefaultConstraint),
    ForeignKeyConstraint(ForeignKeyConstraint),
    GeneratedConstraint(GeneratedConstraint),
    NotNullConstraint(NotNullConstraint),
    NullConstraint(NullConstraint),
    PrimaryKeyConstraint(PrimaryKeyConstraint),
    ReferencesConstraint(ReferencesConstraint),
    UniqueConstraint(UniqueConstraint),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expr {
    ArrayExpr(ArrayExpr),
    BetweenExpr(BetweenExpr),
    BinExpr(BinExpr),
    CallExpr(CallExpr),
    CaseExpr(CaseExpr),
    CastExpr(CastExpr),
    FieldExpr(FieldExpr),
    IndexExpr(IndexExpr),
    Literal(Literal),
    NameRef(NameRef),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FuncOption {
    AsFuncOption(AsFuncOption),
    BeginFuncOption(BeginFuncOption),
    CostFuncOption(CostFuncOption),
    LanguageFuncOption(LanguageFuncOption),
    LeakproofFuncOption(LeakproofFuncOption),
    ParallelFuncOption(ParallelFuncOption),
    ResetFuncOption(ResetFuncOption),
    ReturnFuncOption(ReturnFuncOption),
    RowsFuncOption(RowsFuncOption),
    SecurityFuncOption(SecurityFuncOption),
    SetFuncOption(SetFuncOption),
    StrictFuncOption(StrictFuncOption),
    SupportFuncOption(SupportFuncOption),
    TransformFuncOption(TransformFuncOption),
    VolatilityFuncOption(VolatilityFuncOption),
    WindowFuncOption(WindowFuncOption),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MatchType {
    MatchFull(MatchFull),
    MatchPartial(MatchPartial),
    MatchSimple(MatchSimple),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ParamMode {
    ParamIn(ParamIn),
    ParamInOut(ParamInOut),
    ParamOut(ParamOut),
    ParamVariadic(ParamVariadic),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RefAction {
    Cascade(Cascade),
    NoAction(NoAction),
    Restrict(Restrict),
    SetDefaultColumns(SetDefaultColumns),
    SetNullColumns(SetNullColumns),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Stmt {
    AlterAggregate(AlterAggregate),
    AlterCollation(AlterCollation),
    AlterConversion(AlterConversion),
    AlterDatabase(AlterDatabase),
    AlterDefaultPrivileges(AlterDefaultPrivileges),
    AlterDomain(AlterDomain),
    AlterEventTrigger(AlterEventTrigger),
    AlterExtension(AlterExtension),
    AlterForeignDataWrapper(AlterForeignDataWrapper),
    AlterForeignTable(AlterForeignTable),
    AlterFunction(AlterFunction),
    AlterGroup(AlterGroup),
    AlterIndex(AlterIndex),
    AlterLanguage(AlterLanguage),
    AlterLargeObject(AlterLargeObject),
    AlterMaterializedView(AlterMaterializedView),
    AlterOperator(AlterOperator),
    AlterOperatorClass(AlterOperatorClass),
    AlterOperatorFamily(AlterOperatorFamily),
    AlterPolicy(AlterPolicy),
    AlterProcedure(AlterProcedure),
    AlterPublication(AlterPublication),
    AlterRole(AlterRole),
    AlterRoutine(AlterRoutine),
    AlterRule(AlterRule),
    AlterSchema(AlterSchema),
    AlterSequence(AlterSequence),
    AlterServer(AlterServer),
    AlterStatistics(AlterStatistics),
    AlterSubscription(AlterSubscription),
    AlterSystem(AlterSystem),
    AlterTable(AlterTable),
    AlterTablespace(AlterTablespace),
    AlterTextSearchConfiguration(AlterTextSearchConfiguration),
    AlterTextSearchDictionary(AlterTextSearchDictionary),
    AlterTextSearchParser(AlterTextSearchParser),
    AlterTextSearchTemplate(AlterTextSearchTemplate),
    AlterTrigger(AlterTrigger),
    AlterType(AlterType),
    AlterUser(AlterUser),
    AlterUserMapping(AlterUserMapping),
    AlterView(AlterView),
    Analyze(Analyze),
    Begin(Begin),
    Call(Call),
    Checkpoint(Checkpoint),
    Close(Close),
    Cluster(Cluster),
    CommentOn(CommentOn),
    Commit(Commit),
    Copy(Copy),
    CreateAccessMethod(CreateAccessMethod),
    CreateAggregate(CreateAggregate),
    CreateCast(CreateCast),
    CreateCollation(CreateCollation),
    CreateConversion(CreateConversion),
    CreateDatabase(CreateDatabase),
    CreateDomain(CreateDomain),
    CreateEventTrigger(CreateEventTrigger),
    CreateExtension(CreateExtension),
    CreateForeignDataWrapper(CreateForeignDataWrapper),
    CreateForeignTable(CreateForeignTable),
    CreateFunction(CreateFunction),
    CreateGroup(CreateGroup),
    CreateIndex(CreateIndex),
    CreateLanguage(CreateLanguage),
    CreateMaterializedView(CreateMaterializedView),
    CreateOperator(CreateOperator),
    CreateOperatorClass(CreateOperatorClass),
    CreateOperatorFamily(CreateOperatorFamily),
    CreatePolicy(CreatePolicy),
    CreateProcedure(CreateProcedure),
    CreatePublication(CreatePublication),
    CreateRole(CreateRole),
    CreateRule(CreateRule),
    CreateSchema(CreateSchema),
    CreateSequence(CreateSequence),
    CreateServer(CreateServer),
    CreateStatistics(CreateStatistics),
    CreateSubscription(CreateSubscription),
    CreateTable(CreateTable),
    CreateTableAs(CreateTableAs),
    CreateTablespace(CreateTablespace),
    CreateTextSearchConfiguration(CreateTextSearchConfiguration),
    CreateTextSearchDictionary(CreateTextSearchDictionary),
    CreateTextSearchParser(CreateTextSearchParser),
    CreateTextSearchTemplate(CreateTextSearchTemplate),
    CreateTransform(CreateTransform),
    CreateTrigger(CreateTrigger),
    CreateType(CreateType),
    CreateUser(CreateUser),
    CreateUserMapping(CreateUserMapping),
    CreateView(CreateView),
    Deallocate(Deallocate),
    Declare(Declare),
    Delete(Delete),
    Discard(Discard),
    Do(Do),
    DropAccessMethod(DropAccessMethod),
    DropAggregate(DropAggregate),
    DropCast(DropCast),
    DropCollation(DropCollation),
    DropConversion(DropConversion),
    DropDatabase(DropDatabase),
    DropDomain(DropDomain),
    DropEventTrigger(DropEventTrigger),
    DropExtension(DropExtension),
    DropForeignDataWrapper(DropForeignDataWrapper),
    DropForeignTable(DropForeignTable),
    DropFunction(DropFunction),
    DropGroup(DropGroup),
    DropIndex(DropIndex),
    DropLanguage(DropLanguage),
    DropMaterializedView(DropMaterializedView),
    DropOperator(DropOperator),
    DropOperatorClass(DropOperatorClass),
    DropOperatorFamily(DropOperatorFamily),
    DropOwned(DropOwned),
    DropPolicy(DropPolicy),
    DropProcedure(DropProcedure),
    DropPublication(DropPublication),
    DropRole(DropRole),
    DropRoutine(DropRoutine),
    DropRule(DropRule),
    DropSchema(DropSchema),
    DropSequence(DropSequence),
    DropServer(DropServer),
    DropStatistics(DropStatistics),
    DropSubscription(DropSubscription),
    DropTable(DropTable),
    DropTablespace(DropTablespace),
    DropTextSearchConfig(DropTextSearchConfig),
    DropTextSearchDict(DropTextSearchDict),
    DropTextSearchParser(DropTextSearchParser),
    DropTextSearchTemplate(DropTextSearchTemplate),
    DropTransform(DropTransform),
    DropTrigger(DropTrigger),
    DropType(DropType),
    DropUser(DropUser),
    DropUserMapping(DropUserMapping),
    DropView(DropView),
    Execute(Execute),
    Explain(Explain),
    Fetch(Fetch),
    Grant(Grant),
    ImportForeignSchema(ImportForeignSchema),
    Insert(Insert),
    Listen(Listen),
    Load(Load),
    Lock(Lock),
    Merge(Merge),
    Move(Move),
    Notify(Notify),
    Prepare(Prepare),
    PrepareTransaction(PrepareTransaction),
    Reassign(Reassign),
    Refresh(Refresh),
    Reindex(Reindex),
    ReleaseSavepoint(ReleaseSavepoint),
    Reset(Reset),
    Revoke(Revoke),
    Rollback(Rollback),
    Savepoint(Savepoint),
    SecurityLabel(SecurityLabel),
    Select(Select),
    SelectInto(SelectInto),
    Set(Set),
    SetConstraints(SetConstraints),
    SetRole(SetRole),
    SetSessionAuth(SetSessionAuth),
    SetTransaction(SetTransaction),
    Show(Show),
    Truncate(Truncate),
    Unlisten(Unlisten),
    Update(Update),
    Vacuum(Vacuum),
    Values(Values),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TableArg {
    Column(Column),
    LikeClause(LikeClause),
    TableConstraint(TableConstraint),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TableConstraint {
    CheckConstraint(CheckConstraint),
    ExcludeConstraint(ExcludeConstraint),
    ForeignKeyConstraint(ForeignKeyConstraint),
    PrimaryKeyConstraint(PrimaryKeyConstraint),
    UniqueConstraint(UniqueConstraint),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TransactionMode {
    Deferrable(Deferrable),
    NotDeferrable(NotDeferrable),
    ReadOnly(ReadOnly),
    ReadWrite(ReadWrite),
    TransactionModeIsolationLevel(TransactionModeIsolationLevel),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    ArrayType(ArrayType),
    BitType(BitType),
    CharType(CharType),
    DoubleType(DoubleType),
    IntervalType(IntervalType),
    PathType(PathType),
    PercentType(PercentType),
    TimeType(TimeType),
}
impl AstNode for AddColumn {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ADD_COLUMN
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AddConstraint {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ADD_CONSTRAINT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AddGenerated {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ADD_GENERATED
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Aggregate {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::AGGREGATE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Alias {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALIAS
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterAggregate {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_AGGREGATE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterCollation {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_COLLATION
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterColumn {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_COLUMN
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterConstraint {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_CONSTRAINT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterConversion {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_CONVERSION
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterDatabase {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_DATABASE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterDefaultPrivileges {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_DEFAULT_PRIVILEGES
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterDomain {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_DOMAIN
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterEventTrigger {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_EVENT_TRIGGER
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterExtension {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_EXTENSION
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterForeignDataWrapper {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_FOREIGN_DATA_WRAPPER
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterForeignTable {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_FOREIGN_TABLE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterFunction {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_FUNCTION
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterGroup {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_GROUP
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterIndex {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_INDEX
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterLanguage {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_LANGUAGE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterLargeObject {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_LARGE_OBJECT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterMaterializedView {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_MATERIALIZED_VIEW
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterOperator {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_OPERATOR
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterOperatorClass {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_OPERATOR_CLASS
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterOperatorFamily {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_OPERATOR_FAMILY
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterPolicy {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_POLICY
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterProcedure {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_PROCEDURE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterPublication {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_PUBLICATION
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterRole {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_ROLE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterRoutine {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_ROUTINE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterRule {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_RULE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterSchema {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_SCHEMA
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterSequence {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_SEQUENCE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterServer {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_SERVER
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterStatistics {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_STATISTICS
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterSubscription {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_SUBSCRIPTION
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterSystem {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_SYSTEM
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterTable {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_TABLE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterTablespace {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_TABLESPACE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterTextSearchConfiguration {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_TEXT_SEARCH_CONFIGURATION
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterTextSearchDictionary {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_TEXT_SEARCH_DICTIONARY
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterTextSearchParser {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_TEXT_SEARCH_PARSER
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterTextSearchTemplate {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_TEXT_SEARCH_TEMPLATE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterTrigger {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_TRIGGER
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterType {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_TYPE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterUser {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_USER
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterUserMapping {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_USER_MAPPING
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterView {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_VIEW
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Analyze {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ANALYZE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Arg {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ARG
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for ArgList {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ARG_LIST
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for ArrayExpr {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ARRAY_EXPR
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for ArrayType {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ARRAY_TYPE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AsFuncOption {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::AS_FUNC_OPTION
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AtTimeZone {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::AT_TIME_ZONE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AttachPartition {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ATTACH_PARTITION
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Begin {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::BEGIN
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for BeginFuncOption {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::BEGIN_FUNC_OPTION
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for BetweenExpr {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::BETWEEN_EXPR
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for BinExpr {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::BIN_EXPR
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for BitType {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::BIT_TYPE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Call {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CALL
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CallExpr {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CALL_EXPR
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Cascade {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CASCADE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CaseExpr {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CASE_EXPR
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CastExpr {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CAST_EXPR
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CharType {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CHAR_TYPE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CheckConstraint {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CHECK_CONSTRAINT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Checkpoint {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CHECKPOINT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Close {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CLOSE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Cluster {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CLUSTER
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for ClusterOn {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CLUSTER_ON
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Collate {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::COLLATE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for ColonColon {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::COLON_COLON
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for ColonEq {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::COLON_EQ
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Column {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::COLUMN
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for ColumnList {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::COLUMN_LIST
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CommentOn {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::COMMENT_ON
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Commit {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::COMMIT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CompoundSelect {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::COMPOUND_SELECT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for ConstraintExclusions {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CONSTRAINT_EXCLUSIONS
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for ConstraintIncludeClause {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CONSTRAINT_INCLUDE_CLAUSE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for ConstraintIndexMethod {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CONSTRAINT_INDEX_METHOD
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for ConstraintIndexTablespace {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CONSTRAINT_INDEX_TABLESPACE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for ConstraintStorageParams {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CONSTRAINT_STORAGE_PARAMS
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for ConstraintWhereClause {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CONSTRAINT_WHERE_CLAUSE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Copy {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::COPY
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CostFuncOption {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::COST_FUNC_OPTION
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CreateAccessMethod {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_ACCESS_METHOD
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CreateAggregate {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_AGGREGATE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CreateCast {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_CAST
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CreateCollation {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_COLLATION
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CreateConversion {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_CONVERSION
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CreateDatabase {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_DATABASE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CreateDomain {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_DOMAIN
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CreateEventTrigger {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_EVENT_TRIGGER
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CreateExtension {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_EXTENSION
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CreateForeignDataWrapper {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_FOREIGN_DATA_WRAPPER
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CreateForeignTable {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_FOREIGN_TABLE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CreateFunction {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_FUNCTION
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CreateGroup {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_GROUP
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CreateIndex {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_INDEX
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CreateLanguage {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_LANGUAGE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CreateMaterializedView {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_MATERIALIZED_VIEW
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CreateOperator {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_OPERATOR
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CreateOperatorClass {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_OPERATOR_CLASS
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CreateOperatorFamily {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_OPERATOR_FAMILY
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CreatePolicy {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_POLICY
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CreateProcedure {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_PROCEDURE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CreatePublication {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_PUBLICATION
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CreateRole {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_ROLE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CreateRule {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_RULE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CreateSchema {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_SCHEMA
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CreateSequence {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_SEQUENCE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CreateServer {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_SERVER
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CreateStatistics {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_STATISTICS
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CreateSubscription {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_SUBSCRIPTION
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CreateTable {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_TABLE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CreateTableAs {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_TABLE_AS
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CreateTablespace {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_TABLESPACE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CreateTextSearchConfiguration {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_TEXT_SEARCH_CONFIGURATION
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CreateTextSearchDictionary {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_TEXT_SEARCH_DICTIONARY
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CreateTextSearchParser {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_TEXT_SEARCH_PARSER
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CreateTextSearchTemplate {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_TEXT_SEARCH_TEMPLATE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CreateTransform {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_TRANSFORM
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CreateTrigger {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_TRIGGER
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CreateType {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_TYPE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CreateUser {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_USER
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CreateUserMapping {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_USER_MAPPING
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CreateView {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_VIEW
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CustomOp {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CUSTOM_OP
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Deallocate {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DEALLOCATE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Declare {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DECLARE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DefaultConstraint {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DEFAULT_CONSTRAINT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Deferrable {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DEFERRABLE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DeferrableConstraintOption {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DEFERRABLE_CONSTRAINT_OPTION
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Delete {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DELETE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DetachPartition {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DETACH_PARTITION
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DisableRls {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DISABLE_RLS
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DisableRule {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DISABLE_RULE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DisableTrigger {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DISABLE_TRIGGER
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Discard {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DISCARD
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DistinctClause {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DISTINCT_CLAUSE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Do {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DO
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DoubleType {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DOUBLE_TYPE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DropAccessMethod {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_ACCESS_METHOD
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DropAggregate {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_AGGREGATE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DropCast {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_CAST
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DropCollation {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_COLLATION
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DropColumn {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_COLUMN
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DropConstraint {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_CONSTRAINT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DropConversion {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_CONVERSION
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DropDatabase {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_DATABASE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DropDefault {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_DEFAULT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DropDomain {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_DOMAIN
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DropEventTrigger {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_EVENT_TRIGGER
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DropExpression {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_EXPRESSION
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DropExtension {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_EXTENSION
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DropForeignDataWrapper {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_FOREIGN_DATA_WRAPPER
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DropForeignTable {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_FOREIGN_TABLE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DropFunction {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_FUNCTION
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DropGroup {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_GROUP
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DropIdentity {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_IDENTITY
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DropIndex {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_INDEX
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DropLanguage {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_LANGUAGE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DropMaterializedView {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_MATERIALIZED_VIEW
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DropNotNull {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_NOT_NULL
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DropOperator {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_OPERATOR
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DropOperatorClass {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_OPERATOR_CLASS
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DropOperatorFamily {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_OPERATOR_FAMILY
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DropOwned {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_OWNED
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DropPolicy {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_POLICY
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DropProcedure {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_PROCEDURE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DropPublication {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_PUBLICATION
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DropRole {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_ROLE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DropRoutine {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_ROUTINE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DropRule {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_RULE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DropSchema {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_SCHEMA
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DropSequence {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_SEQUENCE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DropServer {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_SERVER
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DropStatistics {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_STATISTICS
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DropSubscription {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_SUBSCRIPTION
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DropTable {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_TABLE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DropTablespace {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_TABLESPACE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DropTextSearchConfig {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_TEXT_SEARCH_CONFIG
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DropTextSearchDict {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_TEXT_SEARCH_DICT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DropTextSearchParser {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_TEXT_SEARCH_PARSER
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DropTextSearchTemplate {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_TEXT_SEARCH_TEMPLATE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DropTransform {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_TRANSFORM
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DropTrigger {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_TRIGGER
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DropType {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_TYPE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DropUser {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_USER
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DropUserMapping {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_USER_MAPPING
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DropView {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_VIEW
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for EnableAlwaysRule {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ENABLE_ALWAYS_RULE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for EnableAlwaysTrigger {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ENABLE_ALWAYS_TRIGGER
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for EnableReplicaRule {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ENABLE_REPLICA_RULE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for EnableReplicaTrigger {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ENABLE_REPLICA_TRIGGER
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for EnableRls {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ENABLE_RLS
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for EnableRule {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ENABLE_RULE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for EnableTrigger {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ENABLE_TRIGGER
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Enforced {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ENFORCED
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for ExcludeConstraint {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::EXCLUDE_CONSTRAINT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Execute {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::EXECUTE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Explain {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::EXPLAIN
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for FatArrow {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::FAT_ARROW
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Fetch {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::FETCH
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for FetchClause {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::FETCH_CLAUSE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for FieldExpr {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::FIELD_EXPR
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for FilterClause {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::FILTER_CLAUSE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for ForceRls {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::FORCE_RLS
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for ForeignKeyConstraint {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::FOREIGN_KEY_CONSTRAINT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for FromClause {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::FROM_CLAUSE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for FromItem {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::FROM_ITEM
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for FuncOptionList {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::FUNC_OPTION_LIST
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for GeneratedConstraint {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::GENERATED_CONSTRAINT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Grant {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::GRANT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for GroupByClause {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::GROUP_BY_CLAUSE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Gteq {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::GTEQ
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for HavingClause {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::HAVING_CLAUSE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for IfExists {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::IF_EXISTS
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for IfNotExists {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::IF_NOT_EXISTS
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for ImportForeignSchema {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::IMPORT_FOREIGN_SCHEMA
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for IndexExpr {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::INDEX_EXPR
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for IndexParams {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::INDEX_PARAMS
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Inherit {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::INHERIT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for InitiallyDeferredConstraintOption {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::INITIALLY_DEFERRED_CONSTRAINT_OPTION
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for InitiallyImmediateConstraintOption {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::INITIALLY_IMMEDIATE_CONSTRAINT_OPTION
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Insert {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::INSERT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for IntervalType {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::INTERVAL_TYPE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for IntoClause {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::INTO_CLAUSE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for IsDistinctFrom {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::IS_DISTINCT_FROM
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for IsNormalized {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::IS_NORMALIZED
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for IsNot {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::IS_NOT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for IsNotDistinctFrom {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::IS_NOT_DISTINCT_FROM
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for IsNotNormalized {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::IS_NOT_NORMALIZED
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Join {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::JOIN
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for JsonBehaviorClause {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::JSON_BEHAVIOR_CLAUSE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for JsonFormatClause {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::JSON_FORMAT_CLAUSE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for JsonKeyValue {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::JSON_KEY_VALUE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for JsonKeysUniqueClause {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::JSON_KEYS_UNIQUE_CLAUSE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for JsonNullClause {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::JSON_NULL_CLAUSE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for JsonOnErrorClause {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::JSON_ON_ERROR_CLAUSE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for JsonPassingClause {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::JSON_PASSING_CLAUSE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for JsonQuotesClause {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::JSON_QUOTES_CLAUSE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for JsonReturningClause {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::JSON_RETURNING_CLAUSE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for JsonValueExpr {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::JSON_VALUE_EXPR
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for JsonWrapperBehaviorClause {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::JSON_WRAPPER_BEHAVIOR_CLAUSE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for LanguageFuncOption {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::LANGUAGE_FUNC_OPTION
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for LeakproofFuncOption {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::LEAKPROOF_FUNC_OPTION
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for LikeClause {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::LIKE_CLAUSE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for LimitClause {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::LIMIT_CLAUSE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Listen {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::LISTEN
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Literal {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::LITERAL
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Load {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::LOAD
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Lock {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::LOCK
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for LockingClause {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::LOCKING_CLAUSE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Lteq {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::LTEQ
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for MatchFull {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::MATCH_FULL
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for MatchPartial {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::MATCH_PARTIAL
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for MatchSimple {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::MATCH_SIMPLE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Merge {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::MERGE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Move {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::MOVE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Name {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::NAME
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for NameRef {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::NAME_REF
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for NamedArg {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::NAMED_ARG
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Neq {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::NEQ
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Neqb {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::NEQB
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for NoAction {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::NO_ACTION
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for NoForceRls {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::NO_FORCE_RLS
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for NoInherit {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::NO_INHERIT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for NotDeferrable {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::NOT_DEFERRABLE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for NotDeferrableConstraintOption {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::NOT_DEFERRABLE_CONSTRAINT_OPTION
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for NotEnforced {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::NOT_ENFORCED
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for NotIlike {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::NOT_ILIKE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for NotIn {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::NOT_IN
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for NotLike {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::NOT_LIKE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for NotNullConstraint {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::NOT_NULL_CONSTRAINT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for NotOf {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::NOT_OF
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for NotValid {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::NOT_VALID
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Notify {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::NOTIFY
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for NullConstraint {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::NULL_CONSTRAINT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for OfType {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::OF_TYPE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for OffsetClause {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::OFFSET_CLAUSE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for OnDeleteAction {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ON_DELETE_ACTION
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for OnUpdateAction {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ON_UPDATE_ACTION
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Op {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::OP
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for OperatorCall {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::OPERATOR_CALL
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for OrReplace {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::OR_REPLACE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for OrderByClause {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ORDER_BY_CLAUSE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for OverClause {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::OVER_CLAUSE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for OwnerTo {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::OWNER_TO
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for ParallelFuncOption {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::PARALLEL_FUNC_OPTION
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Param {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::PARAM
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for ParamDefault {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::PARAM_DEFAULT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for ParamIn {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::PARAM_IN
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for ParamInOut {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::PARAM_IN_OUT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for ParamList {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::PARAM_LIST
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for ParamOut {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::PARAM_OUT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for ParamVariadic {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::PARAM_VARIADIC
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for ParenExpr {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::PAREN_EXPR
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Path {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::PATH
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for PathSegment {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::PATH_SEGMENT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for PathType {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::PATH_TYPE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for PercentType {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::PERCENT_TYPE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for PercentTypeClause {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::PERCENT_TYPE_CLAUSE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for PostfixExpr {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::POSTFIX_EXPR
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for PrefixExpr {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::PREFIX_EXPR
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Prepare {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::PREPARE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for PrepareTransaction {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::PREPARE_TRANSACTION
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for PrimaryKeyConstraint {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::PRIMARY_KEY_CONSTRAINT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for ReadCommitted {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::READ_COMMITTED
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for ReadOnly {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::READ_ONLY
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for ReadUncommitted {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::READ_UNCOMMITTED
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for ReadWrite {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::READ_WRITE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Reassign {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::REASSIGN
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for ReferencesConstraint {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::REFERENCES_CONSTRAINT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Refresh {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::REFRESH
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Reindex {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::REINDEX
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for RelationName {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::RELATION_NAME
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for ReleaseSavepoint {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::RELEASE_SAVEPOINT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for RenameColumn {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::RENAME_COLUMN
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for RenameConstraint {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::RENAME_CONSTRAINT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for RenameTable {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::RENAME_TABLE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for RenameTo {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::RENAME_TO
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for RepeatableRead {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::REPEATABLE_READ
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for ReplicaIdentity {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::REPLICA_IDENTITY
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Reset {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::RESET
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for ResetFuncOption {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::RESET_FUNC_OPTION
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for ResetOptions {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::RESET_OPTIONS
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for ResetStorageParams {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::RESET_STORAGE_PARAMS
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Restart {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::RESTART
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Restrict {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::RESTRICT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for RetType {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::RET_TYPE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for ReturnFuncOption {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::RETURN_FUNC_OPTION
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Revoke {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::REVOKE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Role {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ROLE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Rollback {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ROLLBACK
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for RowsFuncOption {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ROWS_FUNC_OPTION
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Savepoint {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SAVEPOINT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for SecurityFuncOption {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SECURITY_FUNC_OPTION
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for SecurityLabel {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SECURITY_LABEL
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Select {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SELECT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for SelectClause {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SELECT_CLAUSE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for SelectInto {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SELECT_INTO
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for SequenceOptionList {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SEQUENCE_OPTION_LIST
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Serializable {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SERIALIZABLE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Set {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SET
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for SetAccessMethod {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SET_ACCESS_METHOD
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for SetCompression {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SET_COMPRESSION
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for SetConstraints {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SET_CONSTRAINTS
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for SetDefault {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SET_DEFAULT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for SetDefaultColumns {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SET_DEFAULT_COLUMNS
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for SetExpression {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SET_EXPRESSION
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for SetFuncOption {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SET_FUNC_OPTION
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for SetGenerated {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SET_GENERATED
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for SetGeneratedOptions {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SET_GENERATED_OPTIONS
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for SetLogged {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SET_LOGGED
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for SetNotNull {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SET_NOT_NULL
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for SetNullColumns {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SET_NULL_COLUMNS
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for SetOptions {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SET_OPTIONS
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for SetOptionsList {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SET_OPTIONS_LIST
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for SetRole {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SET_ROLE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for SetSchema {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SET_SCHEMA
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for SetSequenceOption {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SET_SEQUENCE_OPTION
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for SetSessionAuth {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SET_SESSION_AUTH
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for SetStatistics {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SET_STATISTICS
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for SetStorage {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SET_STORAGE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for SetStorageParams {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SET_STORAGE_PARAMS
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for SetTablespace {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SET_TABLESPACE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for SetTransaction {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SET_TRANSACTION
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for SetType {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SET_TYPE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for SetUnlogged {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SET_UNLOGGED
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for SetWithoutCluster {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SET_WITHOUT_CLUSTER
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for SetWithoutOids {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SET_WITHOUT_OIDS
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Show {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SHOW
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for SimilarTo {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SIMILAR_TO
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for SourceFile {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SOURCE_FILE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for StrictFuncOption {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::STRICT_FUNC_OPTION
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for SupportFuncOption {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SUPPORT_FUNC_OPTION
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Table {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::TABLE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for TableArgs {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::TABLE_ARGS
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for TableList {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::TABLE_LIST
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Target {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::TARGET
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for TargetList {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::TARGET_LIST
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for TimeType {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::TIME_TYPE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for TransactionModeIsolationLevel {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::TRANSACTION_MODE_ISOLATION_LEVEL
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for TransactionModeList {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::TRANSACTION_MODE_LIST
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for TransformFuncOption {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::TRANSFORM_FUNC_OPTION
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Truncate {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::TRUNCATE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for TupleExpr {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::TUPLE_EXPR
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for UnicodeNormalForm {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::UNICODE_NORMAL_FORM
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for UniqueConstraint {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::UNIQUE_CONSTRAINT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Unlisten {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::UNLISTEN
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Update {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::UPDATE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for UsingClause {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::USING_CLAUSE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for UsingIndex {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::USING_INDEX
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Vacuum {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::VACUUM
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for ValidateConstraint {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::VALIDATE_CONSTRAINT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Values {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::VALUES
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for VolatilityFuncOption {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::VOLATILITY_FUNC_OPTION
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for WhenClause {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::WHEN_CLAUSE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for WhereClause {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::WHERE_CLAUSE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for WindowClause {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::WINDOW_CLAUSE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for WindowDef {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::WINDOW_DEF
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for WindowFuncOption {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::WINDOW_FUNC_OPTION
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for WithClause {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::WITH_CLAUSE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for WithTable {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::WITH_TABLE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for WithTimezone {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::WITH_TIMEZONE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for WithinClause {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::WITHIN_CLAUSE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for WithoutTimezone {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::WITHOUT_TIMEZONE
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterColumnOption {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            SyntaxKind::ADD_GENERATED
                | SyntaxKind::DROP_DEFAULT
                | SyntaxKind::DROP_EXPRESSION
                | SyntaxKind::DROP_IDENTITY
                | SyntaxKind::DROP_NOT_NULL
                | SyntaxKind::RESET_OPTIONS
                | SyntaxKind::RESTART
                | SyntaxKind::SET_COMPRESSION
                | SyntaxKind::SET_DEFAULT
                | SyntaxKind::SET_EXPRESSION
                | SyntaxKind::SET_GENERATED
                | SyntaxKind::SET_GENERATED_OPTIONS
                | SyntaxKind::SET_NOT_NULL
                | SyntaxKind::SET_OPTIONS
                | SyntaxKind::SET_SEQUENCE_OPTION
                | SyntaxKind::SET_STATISTICS
                | SyntaxKind::SET_STORAGE
                | SyntaxKind::SET_TYPE
        )
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            SyntaxKind::ADD_GENERATED => AlterColumnOption::AddGenerated(AddGenerated { syntax }),
            SyntaxKind::DROP_DEFAULT => AlterColumnOption::DropDefault(DropDefault { syntax }),
            SyntaxKind::DROP_EXPRESSION => {
                AlterColumnOption::DropExpression(DropExpression { syntax })
            }
            SyntaxKind::DROP_IDENTITY => AlterColumnOption::DropIdentity(DropIdentity { syntax }),
            SyntaxKind::DROP_NOT_NULL => AlterColumnOption::DropNotNull(DropNotNull { syntax }),
            SyntaxKind::RESET_OPTIONS => AlterColumnOption::ResetOptions(ResetOptions { syntax }),
            SyntaxKind::RESTART => AlterColumnOption::Restart(Restart { syntax }),
            SyntaxKind::SET_COMPRESSION => {
                AlterColumnOption::SetCompression(SetCompression { syntax })
            }
            SyntaxKind::SET_DEFAULT => AlterColumnOption::SetDefault(SetDefault { syntax }),
            SyntaxKind::SET_EXPRESSION => {
                AlterColumnOption::SetExpression(SetExpression { syntax })
            }
            SyntaxKind::SET_GENERATED => AlterColumnOption::SetGenerated(SetGenerated { syntax }),
            SyntaxKind::SET_GENERATED_OPTIONS => {
                AlterColumnOption::SetGeneratedOptions(SetGeneratedOptions { syntax })
            }
            SyntaxKind::SET_NOT_NULL => AlterColumnOption::SetNotNull(SetNotNull { syntax }),
            SyntaxKind::SET_OPTIONS => AlterColumnOption::SetOptions(SetOptions { syntax }),
            SyntaxKind::SET_SEQUENCE_OPTION => {
                AlterColumnOption::SetSequenceOption(SetSequenceOption { syntax })
            }
            SyntaxKind::SET_STATISTICS => {
                AlterColumnOption::SetStatistics(SetStatistics { syntax })
            }
            SyntaxKind::SET_STORAGE => AlterColumnOption::SetStorage(SetStorage { syntax }),
            SyntaxKind::SET_TYPE => AlterColumnOption::SetType(SetType { syntax }),
            _ => {
                return None;
            }
        };
        Some(res)
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AlterColumnOption::AddGenerated(it) => &it.syntax,
            AlterColumnOption::DropDefault(it) => &it.syntax,
            AlterColumnOption::DropExpression(it) => &it.syntax,
            AlterColumnOption::DropIdentity(it) => &it.syntax,
            AlterColumnOption::DropNotNull(it) => &it.syntax,
            AlterColumnOption::ResetOptions(it) => &it.syntax,
            AlterColumnOption::Restart(it) => &it.syntax,
            AlterColumnOption::SetCompression(it) => &it.syntax,
            AlterColumnOption::SetDefault(it) => &it.syntax,
            AlterColumnOption::SetExpression(it) => &it.syntax,
            AlterColumnOption::SetGenerated(it) => &it.syntax,
            AlterColumnOption::SetGeneratedOptions(it) => &it.syntax,
            AlterColumnOption::SetNotNull(it) => &it.syntax,
            AlterColumnOption::SetOptions(it) => &it.syntax,
            AlterColumnOption::SetSequenceOption(it) => &it.syntax,
            AlterColumnOption::SetStatistics(it) => &it.syntax,
            AlterColumnOption::SetStorage(it) => &it.syntax,
            AlterColumnOption::SetType(it) => &it.syntax,
        }
    }
}
impl From<AddGenerated> for AlterColumnOption {
    #[inline]
    fn from(node: AddGenerated) -> AlterColumnOption {
        AlterColumnOption::AddGenerated(node)
    }
}
impl From<DropDefault> for AlterColumnOption {
    #[inline]
    fn from(node: DropDefault) -> AlterColumnOption {
        AlterColumnOption::DropDefault(node)
    }
}
impl From<DropExpression> for AlterColumnOption {
    #[inline]
    fn from(node: DropExpression) -> AlterColumnOption {
        AlterColumnOption::DropExpression(node)
    }
}
impl From<DropIdentity> for AlterColumnOption {
    #[inline]
    fn from(node: DropIdentity) -> AlterColumnOption {
        AlterColumnOption::DropIdentity(node)
    }
}
impl From<DropNotNull> for AlterColumnOption {
    #[inline]
    fn from(node: DropNotNull) -> AlterColumnOption {
        AlterColumnOption::DropNotNull(node)
    }
}
impl From<ResetOptions> for AlterColumnOption {
    #[inline]
    fn from(node: ResetOptions) -> AlterColumnOption {
        AlterColumnOption::ResetOptions(node)
    }
}
impl From<Restart> for AlterColumnOption {
    #[inline]
    fn from(node: Restart) -> AlterColumnOption {
        AlterColumnOption::Restart(node)
    }
}
impl From<SetCompression> for AlterColumnOption {
    #[inline]
    fn from(node: SetCompression) -> AlterColumnOption {
        AlterColumnOption::SetCompression(node)
    }
}
impl From<SetDefault> for AlterColumnOption {
    #[inline]
    fn from(node: SetDefault) -> AlterColumnOption {
        AlterColumnOption::SetDefault(node)
    }
}
impl From<SetExpression> for AlterColumnOption {
    #[inline]
    fn from(node: SetExpression) -> AlterColumnOption {
        AlterColumnOption::SetExpression(node)
    }
}
impl From<SetGenerated> for AlterColumnOption {
    #[inline]
    fn from(node: SetGenerated) -> AlterColumnOption {
        AlterColumnOption::SetGenerated(node)
    }
}
impl From<SetGeneratedOptions> for AlterColumnOption {
    #[inline]
    fn from(node: SetGeneratedOptions) -> AlterColumnOption {
        AlterColumnOption::SetGeneratedOptions(node)
    }
}
impl From<SetNotNull> for AlterColumnOption {
    #[inline]
    fn from(node: SetNotNull) -> AlterColumnOption {
        AlterColumnOption::SetNotNull(node)
    }
}
impl From<SetOptions> for AlterColumnOption {
    #[inline]
    fn from(node: SetOptions) -> AlterColumnOption {
        AlterColumnOption::SetOptions(node)
    }
}
impl From<SetSequenceOption> for AlterColumnOption {
    #[inline]
    fn from(node: SetSequenceOption) -> AlterColumnOption {
        AlterColumnOption::SetSequenceOption(node)
    }
}
impl From<SetStatistics> for AlterColumnOption {
    #[inline]
    fn from(node: SetStatistics) -> AlterColumnOption {
        AlterColumnOption::SetStatistics(node)
    }
}
impl From<SetStorage> for AlterColumnOption {
    #[inline]
    fn from(node: SetStorage) -> AlterColumnOption {
        AlterColumnOption::SetStorage(node)
    }
}
impl From<SetType> for AlterColumnOption {
    #[inline]
    fn from(node: SetType) -> AlterColumnOption {
        AlterColumnOption::SetType(node)
    }
}
impl AstNode for AlterDomainAction {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            SyntaxKind::ADD_CONSTRAINT
                | SyntaxKind::DROP_CONSTRAINT
                | SyntaxKind::DROP_DEFAULT
                | SyntaxKind::DROP_NOT_NULL
                | SyntaxKind::OWNER_TO
                | SyntaxKind::RENAME_CONSTRAINT
                | SyntaxKind::RENAME_TO
                | SyntaxKind::SET_DEFAULT
                | SyntaxKind::SET_NOT_NULL
                | SyntaxKind::SET_SCHEMA
                | SyntaxKind::VALIDATE_CONSTRAINT
        )
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            SyntaxKind::ADD_CONSTRAINT => {
                AlterDomainAction::AddConstraint(AddConstraint { syntax })
            }
            SyntaxKind::DROP_CONSTRAINT => {
                AlterDomainAction::DropConstraint(DropConstraint { syntax })
            }
            SyntaxKind::DROP_DEFAULT => AlterDomainAction::DropDefault(DropDefault { syntax }),
            SyntaxKind::DROP_NOT_NULL => AlterDomainAction::DropNotNull(DropNotNull { syntax }),
            SyntaxKind::OWNER_TO => AlterDomainAction::OwnerTo(OwnerTo { syntax }),
            SyntaxKind::RENAME_CONSTRAINT => {
                AlterDomainAction::RenameConstraint(RenameConstraint { syntax })
            }
            SyntaxKind::RENAME_TO => AlterDomainAction::RenameTo(RenameTo { syntax }),
            SyntaxKind::SET_DEFAULT => AlterDomainAction::SetDefault(SetDefault { syntax }),
            SyntaxKind::SET_NOT_NULL => AlterDomainAction::SetNotNull(SetNotNull { syntax }),
            SyntaxKind::SET_SCHEMA => AlterDomainAction::SetSchema(SetSchema { syntax }),
            SyntaxKind::VALIDATE_CONSTRAINT => {
                AlterDomainAction::ValidateConstraint(ValidateConstraint { syntax })
            }
            _ => {
                return None;
            }
        };
        Some(res)
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AlterDomainAction::AddConstraint(it) => &it.syntax,
            AlterDomainAction::DropConstraint(it) => &it.syntax,
            AlterDomainAction::DropDefault(it) => &it.syntax,
            AlterDomainAction::DropNotNull(it) => &it.syntax,
            AlterDomainAction::OwnerTo(it) => &it.syntax,
            AlterDomainAction::RenameConstraint(it) => &it.syntax,
            AlterDomainAction::RenameTo(it) => &it.syntax,
            AlterDomainAction::SetDefault(it) => &it.syntax,
            AlterDomainAction::SetNotNull(it) => &it.syntax,
            AlterDomainAction::SetSchema(it) => &it.syntax,
            AlterDomainAction::ValidateConstraint(it) => &it.syntax,
        }
    }
}
impl From<AddConstraint> for AlterDomainAction {
    #[inline]
    fn from(node: AddConstraint) -> AlterDomainAction {
        AlterDomainAction::AddConstraint(node)
    }
}
impl From<DropConstraint> for AlterDomainAction {
    #[inline]
    fn from(node: DropConstraint) -> AlterDomainAction {
        AlterDomainAction::DropConstraint(node)
    }
}
impl From<DropDefault> for AlterDomainAction {
    #[inline]
    fn from(node: DropDefault) -> AlterDomainAction {
        AlterDomainAction::DropDefault(node)
    }
}
impl From<DropNotNull> for AlterDomainAction {
    #[inline]
    fn from(node: DropNotNull) -> AlterDomainAction {
        AlterDomainAction::DropNotNull(node)
    }
}
impl From<OwnerTo> for AlterDomainAction {
    #[inline]
    fn from(node: OwnerTo) -> AlterDomainAction {
        AlterDomainAction::OwnerTo(node)
    }
}
impl From<RenameConstraint> for AlterDomainAction {
    #[inline]
    fn from(node: RenameConstraint) -> AlterDomainAction {
        AlterDomainAction::RenameConstraint(node)
    }
}
impl From<RenameTo> for AlterDomainAction {
    #[inline]
    fn from(node: RenameTo) -> AlterDomainAction {
        AlterDomainAction::RenameTo(node)
    }
}
impl From<SetDefault> for AlterDomainAction {
    #[inline]
    fn from(node: SetDefault) -> AlterDomainAction {
        AlterDomainAction::SetDefault(node)
    }
}
impl From<SetNotNull> for AlterDomainAction {
    #[inline]
    fn from(node: SetNotNull) -> AlterDomainAction {
        AlterDomainAction::SetNotNull(node)
    }
}
impl From<SetSchema> for AlterDomainAction {
    #[inline]
    fn from(node: SetSchema) -> AlterDomainAction {
        AlterDomainAction::SetSchema(node)
    }
}
impl From<ValidateConstraint> for AlterDomainAction {
    #[inline]
    fn from(node: ValidateConstraint) -> AlterDomainAction {
        AlterDomainAction::ValidateConstraint(node)
    }
}
impl AstNode for AlterTableAction {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            SyntaxKind::ADD_COLUMN
                | SyntaxKind::ADD_CONSTRAINT
                | SyntaxKind::ALTER_COLUMN
                | SyntaxKind::ALTER_CONSTRAINT
                | SyntaxKind::ATTACH_PARTITION
                | SyntaxKind::CLUSTER_ON
                | SyntaxKind::DETACH_PARTITION
                | SyntaxKind::DISABLE_RLS
                | SyntaxKind::DISABLE_RULE
                | SyntaxKind::DISABLE_TRIGGER
                | SyntaxKind::DROP_COLUMN
                | SyntaxKind::DROP_CONSTRAINT
                | SyntaxKind::ENABLE_ALWAYS_RULE
                | SyntaxKind::ENABLE_ALWAYS_TRIGGER
                | SyntaxKind::ENABLE_REPLICA_RULE
                | SyntaxKind::ENABLE_REPLICA_TRIGGER
                | SyntaxKind::ENABLE_RLS
                | SyntaxKind::ENABLE_RULE
                | SyntaxKind::ENABLE_TRIGGER
                | SyntaxKind::FORCE_RLS
                | SyntaxKind::INHERIT
                | SyntaxKind::NO_FORCE_RLS
                | SyntaxKind::NO_INHERIT
                | SyntaxKind::NOT_OF
                | SyntaxKind::OF_TYPE
                | SyntaxKind::OWNER_TO
                | SyntaxKind::RENAME_COLUMN
                | SyntaxKind::RENAME_CONSTRAINT
                | SyntaxKind::RENAME_TABLE
                | SyntaxKind::REPLICA_IDENTITY
                | SyntaxKind::RESET_STORAGE_PARAMS
                | SyntaxKind::SET_ACCESS_METHOD
                | SyntaxKind::SET_LOGGED
                | SyntaxKind::SET_SCHEMA
                | SyntaxKind::SET_STORAGE_PARAMS
                | SyntaxKind::SET_TABLESPACE
                | SyntaxKind::SET_UNLOGGED
                | SyntaxKind::SET_WITHOUT_CLUSTER
                | SyntaxKind::SET_WITHOUT_OIDS
                | SyntaxKind::VALIDATE_CONSTRAINT
        )
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            SyntaxKind::ADD_COLUMN => AlterTableAction::AddColumn(AddColumn { syntax }),
            SyntaxKind::ADD_CONSTRAINT => AlterTableAction::AddConstraint(AddConstraint { syntax }),
            SyntaxKind::ALTER_COLUMN => AlterTableAction::AlterColumn(AlterColumn { syntax }),
            SyntaxKind::ALTER_CONSTRAINT => {
                AlterTableAction::AlterConstraint(AlterConstraint { syntax })
            }
            SyntaxKind::ATTACH_PARTITION => {
                AlterTableAction::AttachPartition(AttachPartition { syntax })
            }
            SyntaxKind::CLUSTER_ON => AlterTableAction::ClusterOn(ClusterOn { syntax }),
            SyntaxKind::DETACH_PARTITION => {
                AlterTableAction::DetachPartition(DetachPartition { syntax })
            }
            SyntaxKind::DISABLE_RLS => AlterTableAction::DisableRls(DisableRls { syntax }),
            SyntaxKind::DISABLE_RULE => AlterTableAction::DisableRule(DisableRule { syntax }),
            SyntaxKind::DISABLE_TRIGGER => {
                AlterTableAction::DisableTrigger(DisableTrigger { syntax })
            }
            SyntaxKind::DROP_COLUMN => AlterTableAction::DropColumn(DropColumn { syntax }),
            SyntaxKind::DROP_CONSTRAINT => {
                AlterTableAction::DropConstraint(DropConstraint { syntax })
            }
            SyntaxKind::ENABLE_ALWAYS_RULE => {
                AlterTableAction::EnableAlwaysRule(EnableAlwaysRule { syntax })
            }
            SyntaxKind::ENABLE_ALWAYS_TRIGGER => {
                AlterTableAction::EnableAlwaysTrigger(EnableAlwaysTrigger { syntax })
            }
            SyntaxKind::ENABLE_REPLICA_RULE => {
                AlterTableAction::EnableReplicaRule(EnableReplicaRule { syntax })
            }
            SyntaxKind::ENABLE_REPLICA_TRIGGER => {
                AlterTableAction::EnableReplicaTrigger(EnableReplicaTrigger { syntax })
            }
            SyntaxKind::ENABLE_RLS => AlterTableAction::EnableRls(EnableRls { syntax }),
            SyntaxKind::ENABLE_RULE => AlterTableAction::EnableRule(EnableRule { syntax }),
            SyntaxKind::ENABLE_TRIGGER => AlterTableAction::EnableTrigger(EnableTrigger { syntax }),
            SyntaxKind::FORCE_RLS => AlterTableAction::ForceRls(ForceRls { syntax }),
            SyntaxKind::INHERIT => AlterTableAction::Inherit(Inherit { syntax }),
            SyntaxKind::NO_FORCE_RLS => AlterTableAction::NoForceRls(NoForceRls { syntax }),
            SyntaxKind::NO_INHERIT => AlterTableAction::NoInherit(NoInherit { syntax }),
            SyntaxKind::NOT_OF => AlterTableAction::NotOf(NotOf { syntax }),
            SyntaxKind::OF_TYPE => AlterTableAction::OfType(OfType { syntax }),
            SyntaxKind::OWNER_TO => AlterTableAction::OwnerTo(OwnerTo { syntax }),
            SyntaxKind::RENAME_COLUMN => AlterTableAction::RenameColumn(RenameColumn { syntax }),
            SyntaxKind::RENAME_CONSTRAINT => {
                AlterTableAction::RenameConstraint(RenameConstraint { syntax })
            }
            SyntaxKind::RENAME_TABLE => AlterTableAction::RenameTable(RenameTable { syntax }),
            SyntaxKind::REPLICA_IDENTITY => {
                AlterTableAction::ReplicaIdentity(ReplicaIdentity { syntax })
            }
            SyntaxKind::RESET_STORAGE_PARAMS => {
                AlterTableAction::ResetStorageParams(ResetStorageParams { syntax })
            }
            SyntaxKind::SET_ACCESS_METHOD => {
                AlterTableAction::SetAccessMethod(SetAccessMethod { syntax })
            }
            SyntaxKind::SET_LOGGED => AlterTableAction::SetLogged(SetLogged { syntax }),
            SyntaxKind::SET_SCHEMA => AlterTableAction::SetSchema(SetSchema { syntax }),
            SyntaxKind::SET_STORAGE_PARAMS => {
                AlterTableAction::SetStorageParams(SetStorageParams { syntax })
            }
            SyntaxKind::SET_TABLESPACE => AlterTableAction::SetTablespace(SetTablespace { syntax }),
            SyntaxKind::SET_UNLOGGED => AlterTableAction::SetUnlogged(SetUnlogged { syntax }),
            SyntaxKind::SET_WITHOUT_CLUSTER => {
                AlterTableAction::SetWithoutCluster(SetWithoutCluster { syntax })
            }
            SyntaxKind::SET_WITHOUT_OIDS => {
                AlterTableAction::SetWithoutOids(SetWithoutOids { syntax })
            }
            SyntaxKind::VALIDATE_CONSTRAINT => {
                AlterTableAction::ValidateConstraint(ValidateConstraint { syntax })
            }
            _ => {
                return None;
            }
        };
        Some(res)
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AlterTableAction::AddColumn(it) => &it.syntax,
            AlterTableAction::AddConstraint(it) => &it.syntax,
            AlterTableAction::AlterColumn(it) => &it.syntax,
            AlterTableAction::AlterConstraint(it) => &it.syntax,
            AlterTableAction::AttachPartition(it) => &it.syntax,
            AlterTableAction::ClusterOn(it) => &it.syntax,
            AlterTableAction::DetachPartition(it) => &it.syntax,
            AlterTableAction::DisableRls(it) => &it.syntax,
            AlterTableAction::DisableRule(it) => &it.syntax,
            AlterTableAction::DisableTrigger(it) => &it.syntax,
            AlterTableAction::DropColumn(it) => &it.syntax,
            AlterTableAction::DropConstraint(it) => &it.syntax,
            AlterTableAction::EnableAlwaysRule(it) => &it.syntax,
            AlterTableAction::EnableAlwaysTrigger(it) => &it.syntax,
            AlterTableAction::EnableReplicaRule(it) => &it.syntax,
            AlterTableAction::EnableReplicaTrigger(it) => &it.syntax,
            AlterTableAction::EnableRls(it) => &it.syntax,
            AlterTableAction::EnableRule(it) => &it.syntax,
            AlterTableAction::EnableTrigger(it) => &it.syntax,
            AlterTableAction::ForceRls(it) => &it.syntax,
            AlterTableAction::Inherit(it) => &it.syntax,
            AlterTableAction::NoForceRls(it) => &it.syntax,
            AlterTableAction::NoInherit(it) => &it.syntax,
            AlterTableAction::NotOf(it) => &it.syntax,
            AlterTableAction::OfType(it) => &it.syntax,
            AlterTableAction::OwnerTo(it) => &it.syntax,
            AlterTableAction::RenameColumn(it) => &it.syntax,
            AlterTableAction::RenameConstraint(it) => &it.syntax,
            AlterTableAction::RenameTable(it) => &it.syntax,
            AlterTableAction::ReplicaIdentity(it) => &it.syntax,
            AlterTableAction::ResetStorageParams(it) => &it.syntax,
            AlterTableAction::SetAccessMethod(it) => &it.syntax,
            AlterTableAction::SetLogged(it) => &it.syntax,
            AlterTableAction::SetSchema(it) => &it.syntax,
            AlterTableAction::SetStorageParams(it) => &it.syntax,
            AlterTableAction::SetTablespace(it) => &it.syntax,
            AlterTableAction::SetUnlogged(it) => &it.syntax,
            AlterTableAction::SetWithoutCluster(it) => &it.syntax,
            AlterTableAction::SetWithoutOids(it) => &it.syntax,
            AlterTableAction::ValidateConstraint(it) => &it.syntax,
        }
    }
}
impl From<AddColumn> for AlterTableAction {
    #[inline]
    fn from(node: AddColumn) -> AlterTableAction {
        AlterTableAction::AddColumn(node)
    }
}
impl From<AddConstraint> for AlterTableAction {
    #[inline]
    fn from(node: AddConstraint) -> AlterTableAction {
        AlterTableAction::AddConstraint(node)
    }
}
impl From<AlterColumn> for AlterTableAction {
    #[inline]
    fn from(node: AlterColumn) -> AlterTableAction {
        AlterTableAction::AlterColumn(node)
    }
}
impl From<AlterConstraint> for AlterTableAction {
    #[inline]
    fn from(node: AlterConstraint) -> AlterTableAction {
        AlterTableAction::AlterConstraint(node)
    }
}
impl From<AttachPartition> for AlterTableAction {
    #[inline]
    fn from(node: AttachPartition) -> AlterTableAction {
        AlterTableAction::AttachPartition(node)
    }
}
impl From<ClusterOn> for AlterTableAction {
    #[inline]
    fn from(node: ClusterOn) -> AlterTableAction {
        AlterTableAction::ClusterOn(node)
    }
}
impl From<DetachPartition> for AlterTableAction {
    #[inline]
    fn from(node: DetachPartition) -> AlterTableAction {
        AlterTableAction::DetachPartition(node)
    }
}
impl From<DisableRls> for AlterTableAction {
    #[inline]
    fn from(node: DisableRls) -> AlterTableAction {
        AlterTableAction::DisableRls(node)
    }
}
impl From<DisableRule> for AlterTableAction {
    #[inline]
    fn from(node: DisableRule) -> AlterTableAction {
        AlterTableAction::DisableRule(node)
    }
}
impl From<DisableTrigger> for AlterTableAction {
    #[inline]
    fn from(node: DisableTrigger) -> AlterTableAction {
        AlterTableAction::DisableTrigger(node)
    }
}
impl From<DropColumn> for AlterTableAction {
    #[inline]
    fn from(node: DropColumn) -> AlterTableAction {
        AlterTableAction::DropColumn(node)
    }
}
impl From<DropConstraint> for AlterTableAction {
    #[inline]
    fn from(node: DropConstraint) -> AlterTableAction {
        AlterTableAction::DropConstraint(node)
    }
}
impl From<EnableAlwaysRule> for AlterTableAction {
    #[inline]
    fn from(node: EnableAlwaysRule) -> AlterTableAction {
        AlterTableAction::EnableAlwaysRule(node)
    }
}
impl From<EnableAlwaysTrigger> for AlterTableAction {
    #[inline]
    fn from(node: EnableAlwaysTrigger) -> AlterTableAction {
        AlterTableAction::EnableAlwaysTrigger(node)
    }
}
impl From<EnableReplicaRule> for AlterTableAction {
    #[inline]
    fn from(node: EnableReplicaRule) -> AlterTableAction {
        AlterTableAction::EnableReplicaRule(node)
    }
}
impl From<EnableReplicaTrigger> for AlterTableAction {
    #[inline]
    fn from(node: EnableReplicaTrigger) -> AlterTableAction {
        AlterTableAction::EnableReplicaTrigger(node)
    }
}
impl From<EnableRls> for AlterTableAction {
    #[inline]
    fn from(node: EnableRls) -> AlterTableAction {
        AlterTableAction::EnableRls(node)
    }
}
impl From<EnableRule> for AlterTableAction {
    #[inline]
    fn from(node: EnableRule) -> AlterTableAction {
        AlterTableAction::EnableRule(node)
    }
}
impl From<EnableTrigger> for AlterTableAction {
    #[inline]
    fn from(node: EnableTrigger) -> AlterTableAction {
        AlterTableAction::EnableTrigger(node)
    }
}
impl From<ForceRls> for AlterTableAction {
    #[inline]
    fn from(node: ForceRls) -> AlterTableAction {
        AlterTableAction::ForceRls(node)
    }
}
impl From<Inherit> for AlterTableAction {
    #[inline]
    fn from(node: Inherit) -> AlterTableAction {
        AlterTableAction::Inherit(node)
    }
}
impl From<NoForceRls> for AlterTableAction {
    #[inline]
    fn from(node: NoForceRls) -> AlterTableAction {
        AlterTableAction::NoForceRls(node)
    }
}
impl From<NoInherit> for AlterTableAction {
    #[inline]
    fn from(node: NoInherit) -> AlterTableAction {
        AlterTableAction::NoInherit(node)
    }
}
impl From<NotOf> for AlterTableAction {
    #[inline]
    fn from(node: NotOf) -> AlterTableAction {
        AlterTableAction::NotOf(node)
    }
}
impl From<OfType> for AlterTableAction {
    #[inline]
    fn from(node: OfType) -> AlterTableAction {
        AlterTableAction::OfType(node)
    }
}
impl From<OwnerTo> for AlterTableAction {
    #[inline]
    fn from(node: OwnerTo) -> AlterTableAction {
        AlterTableAction::OwnerTo(node)
    }
}
impl From<RenameColumn> for AlterTableAction {
    #[inline]
    fn from(node: RenameColumn) -> AlterTableAction {
        AlterTableAction::RenameColumn(node)
    }
}
impl From<RenameConstraint> for AlterTableAction {
    #[inline]
    fn from(node: RenameConstraint) -> AlterTableAction {
        AlterTableAction::RenameConstraint(node)
    }
}
impl From<RenameTable> for AlterTableAction {
    #[inline]
    fn from(node: RenameTable) -> AlterTableAction {
        AlterTableAction::RenameTable(node)
    }
}
impl From<ReplicaIdentity> for AlterTableAction {
    #[inline]
    fn from(node: ReplicaIdentity) -> AlterTableAction {
        AlterTableAction::ReplicaIdentity(node)
    }
}
impl From<ResetStorageParams> for AlterTableAction {
    #[inline]
    fn from(node: ResetStorageParams) -> AlterTableAction {
        AlterTableAction::ResetStorageParams(node)
    }
}
impl From<SetAccessMethod> for AlterTableAction {
    #[inline]
    fn from(node: SetAccessMethod) -> AlterTableAction {
        AlterTableAction::SetAccessMethod(node)
    }
}
impl From<SetLogged> for AlterTableAction {
    #[inline]
    fn from(node: SetLogged) -> AlterTableAction {
        AlterTableAction::SetLogged(node)
    }
}
impl From<SetSchema> for AlterTableAction {
    #[inline]
    fn from(node: SetSchema) -> AlterTableAction {
        AlterTableAction::SetSchema(node)
    }
}
impl From<SetStorageParams> for AlterTableAction {
    #[inline]
    fn from(node: SetStorageParams) -> AlterTableAction {
        AlterTableAction::SetStorageParams(node)
    }
}
impl From<SetTablespace> for AlterTableAction {
    #[inline]
    fn from(node: SetTablespace) -> AlterTableAction {
        AlterTableAction::SetTablespace(node)
    }
}
impl From<SetUnlogged> for AlterTableAction {
    #[inline]
    fn from(node: SetUnlogged) -> AlterTableAction {
        AlterTableAction::SetUnlogged(node)
    }
}
impl From<SetWithoutCluster> for AlterTableAction {
    #[inline]
    fn from(node: SetWithoutCluster) -> AlterTableAction {
        AlterTableAction::SetWithoutCluster(node)
    }
}
impl From<SetWithoutOids> for AlterTableAction {
    #[inline]
    fn from(node: SetWithoutOids) -> AlterTableAction {
        AlterTableAction::SetWithoutOids(node)
    }
}
impl From<ValidateConstraint> for AlterTableAction {
    #[inline]
    fn from(node: ValidateConstraint) -> AlterTableAction {
        AlterTableAction::ValidateConstraint(node)
    }
}
impl AstNode for Constraint {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            SyntaxKind::CHECK_CONSTRAINT
                | SyntaxKind::DEFAULT_CONSTRAINT
                | SyntaxKind::FOREIGN_KEY_CONSTRAINT
                | SyntaxKind::GENERATED_CONSTRAINT
                | SyntaxKind::NOT_NULL_CONSTRAINT
                | SyntaxKind::NULL_CONSTRAINT
                | SyntaxKind::PRIMARY_KEY_CONSTRAINT
                | SyntaxKind::REFERENCES_CONSTRAINT
                | SyntaxKind::UNIQUE_CONSTRAINT
        )
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            SyntaxKind::CHECK_CONSTRAINT => Constraint::CheckConstraint(CheckConstraint { syntax }),
            SyntaxKind::DEFAULT_CONSTRAINT => {
                Constraint::DefaultConstraint(DefaultConstraint { syntax })
            }
            SyntaxKind::FOREIGN_KEY_CONSTRAINT => {
                Constraint::ForeignKeyConstraint(ForeignKeyConstraint { syntax })
            }
            SyntaxKind::GENERATED_CONSTRAINT => {
                Constraint::GeneratedConstraint(GeneratedConstraint { syntax })
            }
            SyntaxKind::NOT_NULL_CONSTRAINT => {
                Constraint::NotNullConstraint(NotNullConstraint { syntax })
            }
            SyntaxKind::NULL_CONSTRAINT => Constraint::NullConstraint(NullConstraint { syntax }),
            SyntaxKind::PRIMARY_KEY_CONSTRAINT => {
                Constraint::PrimaryKeyConstraint(PrimaryKeyConstraint { syntax })
            }
            SyntaxKind::REFERENCES_CONSTRAINT => {
                Constraint::ReferencesConstraint(ReferencesConstraint { syntax })
            }
            SyntaxKind::UNIQUE_CONSTRAINT => {
                Constraint::UniqueConstraint(UniqueConstraint { syntax })
            }
            _ => {
                return None;
            }
        };
        Some(res)
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Constraint::CheckConstraint(it) => &it.syntax,
            Constraint::DefaultConstraint(it) => &it.syntax,
            Constraint::ForeignKeyConstraint(it) => &it.syntax,
            Constraint::GeneratedConstraint(it) => &it.syntax,
            Constraint::NotNullConstraint(it) => &it.syntax,
            Constraint::NullConstraint(it) => &it.syntax,
            Constraint::PrimaryKeyConstraint(it) => &it.syntax,
            Constraint::ReferencesConstraint(it) => &it.syntax,
            Constraint::UniqueConstraint(it) => &it.syntax,
        }
    }
}
impl From<CheckConstraint> for Constraint {
    #[inline]
    fn from(node: CheckConstraint) -> Constraint {
        Constraint::CheckConstraint(node)
    }
}
impl From<DefaultConstraint> for Constraint {
    #[inline]
    fn from(node: DefaultConstraint) -> Constraint {
        Constraint::DefaultConstraint(node)
    }
}
impl From<ForeignKeyConstraint> for Constraint {
    #[inline]
    fn from(node: ForeignKeyConstraint) -> Constraint {
        Constraint::ForeignKeyConstraint(node)
    }
}
impl From<GeneratedConstraint> for Constraint {
    #[inline]
    fn from(node: GeneratedConstraint) -> Constraint {
        Constraint::GeneratedConstraint(node)
    }
}
impl From<NotNullConstraint> for Constraint {
    #[inline]
    fn from(node: NotNullConstraint) -> Constraint {
        Constraint::NotNullConstraint(node)
    }
}
impl From<NullConstraint> for Constraint {
    #[inline]
    fn from(node: NullConstraint) -> Constraint {
        Constraint::NullConstraint(node)
    }
}
impl From<PrimaryKeyConstraint> for Constraint {
    #[inline]
    fn from(node: PrimaryKeyConstraint) -> Constraint {
        Constraint::PrimaryKeyConstraint(node)
    }
}
impl From<ReferencesConstraint> for Constraint {
    #[inline]
    fn from(node: ReferencesConstraint) -> Constraint {
        Constraint::ReferencesConstraint(node)
    }
}
impl From<UniqueConstraint> for Constraint {
    #[inline]
    fn from(node: UniqueConstraint) -> Constraint {
        Constraint::UniqueConstraint(node)
    }
}
impl AstNode for Expr {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            SyntaxKind::ARRAY_EXPR
                | SyntaxKind::BETWEEN_EXPR
                | SyntaxKind::BIN_EXPR
                | SyntaxKind::CALL_EXPR
                | SyntaxKind::CASE_EXPR
                | SyntaxKind::CAST_EXPR
                | SyntaxKind::FIELD_EXPR
                | SyntaxKind::INDEX_EXPR
                | SyntaxKind::LITERAL
                | SyntaxKind::NAME_REF
        )
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            SyntaxKind::ARRAY_EXPR => Expr::ArrayExpr(ArrayExpr { syntax }),
            SyntaxKind::BETWEEN_EXPR => Expr::BetweenExpr(BetweenExpr { syntax }),
            SyntaxKind::BIN_EXPR => Expr::BinExpr(BinExpr { syntax }),
            SyntaxKind::CALL_EXPR => Expr::CallExpr(CallExpr { syntax }),
            SyntaxKind::CASE_EXPR => Expr::CaseExpr(CaseExpr { syntax }),
            SyntaxKind::CAST_EXPR => Expr::CastExpr(CastExpr { syntax }),
            SyntaxKind::FIELD_EXPR => Expr::FieldExpr(FieldExpr { syntax }),
            SyntaxKind::INDEX_EXPR => Expr::IndexExpr(IndexExpr { syntax }),
            SyntaxKind::LITERAL => Expr::Literal(Literal { syntax }),
            SyntaxKind::NAME_REF => Expr::NameRef(NameRef { syntax }),
            _ => {
                return None;
            }
        };
        Some(res)
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Expr::ArrayExpr(it) => &it.syntax,
            Expr::BetweenExpr(it) => &it.syntax,
            Expr::BinExpr(it) => &it.syntax,
            Expr::CallExpr(it) => &it.syntax,
            Expr::CaseExpr(it) => &it.syntax,
            Expr::CastExpr(it) => &it.syntax,
            Expr::FieldExpr(it) => &it.syntax,
            Expr::IndexExpr(it) => &it.syntax,
            Expr::Literal(it) => &it.syntax,
            Expr::NameRef(it) => &it.syntax,
        }
    }
}
impl From<ArrayExpr> for Expr {
    #[inline]
    fn from(node: ArrayExpr) -> Expr {
        Expr::ArrayExpr(node)
    }
}
impl From<BetweenExpr> for Expr {
    #[inline]
    fn from(node: BetweenExpr) -> Expr {
        Expr::BetweenExpr(node)
    }
}
impl From<BinExpr> for Expr {
    #[inline]
    fn from(node: BinExpr) -> Expr {
        Expr::BinExpr(node)
    }
}
impl From<CallExpr> for Expr {
    #[inline]
    fn from(node: CallExpr) -> Expr {
        Expr::CallExpr(node)
    }
}
impl From<CaseExpr> for Expr {
    #[inline]
    fn from(node: CaseExpr) -> Expr {
        Expr::CaseExpr(node)
    }
}
impl From<CastExpr> for Expr {
    #[inline]
    fn from(node: CastExpr) -> Expr {
        Expr::CastExpr(node)
    }
}
impl From<FieldExpr> for Expr {
    #[inline]
    fn from(node: FieldExpr) -> Expr {
        Expr::FieldExpr(node)
    }
}
impl From<IndexExpr> for Expr {
    #[inline]
    fn from(node: IndexExpr) -> Expr {
        Expr::IndexExpr(node)
    }
}
impl From<Literal> for Expr {
    #[inline]
    fn from(node: Literal) -> Expr {
        Expr::Literal(node)
    }
}
impl From<NameRef> for Expr {
    #[inline]
    fn from(node: NameRef) -> Expr {
        Expr::NameRef(node)
    }
}
impl AstNode for FuncOption {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            SyntaxKind::AS_FUNC_OPTION
                | SyntaxKind::BEGIN_FUNC_OPTION
                | SyntaxKind::COST_FUNC_OPTION
                | SyntaxKind::LANGUAGE_FUNC_OPTION
                | SyntaxKind::LEAKPROOF_FUNC_OPTION
                | SyntaxKind::PARALLEL_FUNC_OPTION
                | SyntaxKind::RESET_FUNC_OPTION
                | SyntaxKind::RETURN_FUNC_OPTION
                | SyntaxKind::ROWS_FUNC_OPTION
                | SyntaxKind::SECURITY_FUNC_OPTION
                | SyntaxKind::SET_FUNC_OPTION
                | SyntaxKind::STRICT_FUNC_OPTION
                | SyntaxKind::SUPPORT_FUNC_OPTION
                | SyntaxKind::TRANSFORM_FUNC_OPTION
                | SyntaxKind::VOLATILITY_FUNC_OPTION
                | SyntaxKind::WINDOW_FUNC_OPTION
        )
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            SyntaxKind::AS_FUNC_OPTION => FuncOption::AsFuncOption(AsFuncOption { syntax }),
            SyntaxKind::BEGIN_FUNC_OPTION => {
                FuncOption::BeginFuncOption(BeginFuncOption { syntax })
            }
            SyntaxKind::COST_FUNC_OPTION => FuncOption::CostFuncOption(CostFuncOption { syntax }),
            SyntaxKind::LANGUAGE_FUNC_OPTION => {
                FuncOption::LanguageFuncOption(LanguageFuncOption { syntax })
            }
            SyntaxKind::LEAKPROOF_FUNC_OPTION => {
                FuncOption::LeakproofFuncOption(LeakproofFuncOption { syntax })
            }
            SyntaxKind::PARALLEL_FUNC_OPTION => {
                FuncOption::ParallelFuncOption(ParallelFuncOption { syntax })
            }
            SyntaxKind::RESET_FUNC_OPTION => {
                FuncOption::ResetFuncOption(ResetFuncOption { syntax })
            }
            SyntaxKind::RETURN_FUNC_OPTION => {
                FuncOption::ReturnFuncOption(ReturnFuncOption { syntax })
            }
            SyntaxKind::ROWS_FUNC_OPTION => FuncOption::RowsFuncOption(RowsFuncOption { syntax }),
            SyntaxKind::SECURITY_FUNC_OPTION => {
                FuncOption::SecurityFuncOption(SecurityFuncOption { syntax })
            }
            SyntaxKind::SET_FUNC_OPTION => FuncOption::SetFuncOption(SetFuncOption { syntax }),
            SyntaxKind::STRICT_FUNC_OPTION => {
                FuncOption::StrictFuncOption(StrictFuncOption { syntax })
            }
            SyntaxKind::SUPPORT_FUNC_OPTION => {
                FuncOption::SupportFuncOption(SupportFuncOption { syntax })
            }
            SyntaxKind::TRANSFORM_FUNC_OPTION => {
                FuncOption::TransformFuncOption(TransformFuncOption { syntax })
            }
            SyntaxKind::VOLATILITY_FUNC_OPTION => {
                FuncOption::VolatilityFuncOption(VolatilityFuncOption { syntax })
            }
            SyntaxKind::WINDOW_FUNC_OPTION => {
                FuncOption::WindowFuncOption(WindowFuncOption { syntax })
            }
            _ => {
                return None;
            }
        };
        Some(res)
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        match self {
            FuncOption::AsFuncOption(it) => &it.syntax,
            FuncOption::BeginFuncOption(it) => &it.syntax,
            FuncOption::CostFuncOption(it) => &it.syntax,
            FuncOption::LanguageFuncOption(it) => &it.syntax,
            FuncOption::LeakproofFuncOption(it) => &it.syntax,
            FuncOption::ParallelFuncOption(it) => &it.syntax,
            FuncOption::ResetFuncOption(it) => &it.syntax,
            FuncOption::ReturnFuncOption(it) => &it.syntax,
            FuncOption::RowsFuncOption(it) => &it.syntax,
            FuncOption::SecurityFuncOption(it) => &it.syntax,
            FuncOption::SetFuncOption(it) => &it.syntax,
            FuncOption::StrictFuncOption(it) => &it.syntax,
            FuncOption::SupportFuncOption(it) => &it.syntax,
            FuncOption::TransformFuncOption(it) => &it.syntax,
            FuncOption::VolatilityFuncOption(it) => &it.syntax,
            FuncOption::WindowFuncOption(it) => &it.syntax,
        }
    }
}
impl From<AsFuncOption> for FuncOption {
    #[inline]
    fn from(node: AsFuncOption) -> FuncOption {
        FuncOption::AsFuncOption(node)
    }
}
impl From<BeginFuncOption> for FuncOption {
    #[inline]
    fn from(node: BeginFuncOption) -> FuncOption {
        FuncOption::BeginFuncOption(node)
    }
}
impl From<CostFuncOption> for FuncOption {
    #[inline]
    fn from(node: CostFuncOption) -> FuncOption {
        FuncOption::CostFuncOption(node)
    }
}
impl From<LanguageFuncOption> for FuncOption {
    #[inline]
    fn from(node: LanguageFuncOption) -> FuncOption {
        FuncOption::LanguageFuncOption(node)
    }
}
impl From<LeakproofFuncOption> for FuncOption {
    #[inline]
    fn from(node: LeakproofFuncOption) -> FuncOption {
        FuncOption::LeakproofFuncOption(node)
    }
}
impl From<ParallelFuncOption> for FuncOption {
    #[inline]
    fn from(node: ParallelFuncOption) -> FuncOption {
        FuncOption::ParallelFuncOption(node)
    }
}
impl From<ResetFuncOption> for FuncOption {
    #[inline]
    fn from(node: ResetFuncOption) -> FuncOption {
        FuncOption::ResetFuncOption(node)
    }
}
impl From<ReturnFuncOption> for FuncOption {
    #[inline]
    fn from(node: ReturnFuncOption) -> FuncOption {
        FuncOption::ReturnFuncOption(node)
    }
}
impl From<RowsFuncOption> for FuncOption {
    #[inline]
    fn from(node: RowsFuncOption) -> FuncOption {
        FuncOption::RowsFuncOption(node)
    }
}
impl From<SecurityFuncOption> for FuncOption {
    #[inline]
    fn from(node: SecurityFuncOption) -> FuncOption {
        FuncOption::SecurityFuncOption(node)
    }
}
impl From<SetFuncOption> for FuncOption {
    #[inline]
    fn from(node: SetFuncOption) -> FuncOption {
        FuncOption::SetFuncOption(node)
    }
}
impl From<StrictFuncOption> for FuncOption {
    #[inline]
    fn from(node: StrictFuncOption) -> FuncOption {
        FuncOption::StrictFuncOption(node)
    }
}
impl From<SupportFuncOption> for FuncOption {
    #[inline]
    fn from(node: SupportFuncOption) -> FuncOption {
        FuncOption::SupportFuncOption(node)
    }
}
impl From<TransformFuncOption> for FuncOption {
    #[inline]
    fn from(node: TransformFuncOption) -> FuncOption {
        FuncOption::TransformFuncOption(node)
    }
}
impl From<VolatilityFuncOption> for FuncOption {
    #[inline]
    fn from(node: VolatilityFuncOption) -> FuncOption {
        FuncOption::VolatilityFuncOption(node)
    }
}
impl From<WindowFuncOption> for FuncOption {
    #[inline]
    fn from(node: WindowFuncOption) -> FuncOption {
        FuncOption::WindowFuncOption(node)
    }
}
impl AstNode for MatchType {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            SyntaxKind::MATCH_FULL | SyntaxKind::MATCH_PARTIAL | SyntaxKind::MATCH_SIMPLE
        )
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            SyntaxKind::MATCH_FULL => MatchType::MatchFull(MatchFull { syntax }),
            SyntaxKind::MATCH_PARTIAL => MatchType::MatchPartial(MatchPartial { syntax }),
            SyntaxKind::MATCH_SIMPLE => MatchType::MatchSimple(MatchSimple { syntax }),
            _ => {
                return None;
            }
        };
        Some(res)
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        match self {
            MatchType::MatchFull(it) => &it.syntax,
            MatchType::MatchPartial(it) => &it.syntax,
            MatchType::MatchSimple(it) => &it.syntax,
        }
    }
}
impl From<MatchFull> for MatchType {
    #[inline]
    fn from(node: MatchFull) -> MatchType {
        MatchType::MatchFull(node)
    }
}
impl From<MatchPartial> for MatchType {
    #[inline]
    fn from(node: MatchPartial) -> MatchType {
        MatchType::MatchPartial(node)
    }
}
impl From<MatchSimple> for MatchType {
    #[inline]
    fn from(node: MatchSimple) -> MatchType {
        MatchType::MatchSimple(node)
    }
}
impl AstNode for ParamMode {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            SyntaxKind::PARAM_IN
                | SyntaxKind::PARAM_IN_OUT
                | SyntaxKind::PARAM_OUT
                | SyntaxKind::PARAM_VARIADIC
        )
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            SyntaxKind::PARAM_IN => ParamMode::ParamIn(ParamIn { syntax }),
            SyntaxKind::PARAM_IN_OUT => ParamMode::ParamInOut(ParamInOut { syntax }),
            SyntaxKind::PARAM_OUT => ParamMode::ParamOut(ParamOut { syntax }),
            SyntaxKind::PARAM_VARIADIC => ParamMode::ParamVariadic(ParamVariadic { syntax }),
            _ => {
                return None;
            }
        };
        Some(res)
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        match self {
            ParamMode::ParamIn(it) => &it.syntax,
            ParamMode::ParamInOut(it) => &it.syntax,
            ParamMode::ParamOut(it) => &it.syntax,
            ParamMode::ParamVariadic(it) => &it.syntax,
        }
    }
}
impl From<ParamIn> for ParamMode {
    #[inline]
    fn from(node: ParamIn) -> ParamMode {
        ParamMode::ParamIn(node)
    }
}
impl From<ParamInOut> for ParamMode {
    #[inline]
    fn from(node: ParamInOut) -> ParamMode {
        ParamMode::ParamInOut(node)
    }
}
impl From<ParamOut> for ParamMode {
    #[inline]
    fn from(node: ParamOut) -> ParamMode {
        ParamMode::ParamOut(node)
    }
}
impl From<ParamVariadic> for ParamMode {
    #[inline]
    fn from(node: ParamVariadic) -> ParamMode {
        ParamMode::ParamVariadic(node)
    }
}
impl AstNode for RefAction {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            SyntaxKind::CASCADE
                | SyntaxKind::NO_ACTION
                | SyntaxKind::RESTRICT
                | SyntaxKind::SET_DEFAULT_COLUMNS
                | SyntaxKind::SET_NULL_COLUMNS
        )
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            SyntaxKind::CASCADE => RefAction::Cascade(Cascade { syntax }),
            SyntaxKind::NO_ACTION => RefAction::NoAction(NoAction { syntax }),
            SyntaxKind::RESTRICT => RefAction::Restrict(Restrict { syntax }),
            SyntaxKind::SET_DEFAULT_COLUMNS => {
                RefAction::SetDefaultColumns(SetDefaultColumns { syntax })
            }
            SyntaxKind::SET_NULL_COLUMNS => RefAction::SetNullColumns(SetNullColumns { syntax }),
            _ => {
                return None;
            }
        };
        Some(res)
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        match self {
            RefAction::Cascade(it) => &it.syntax,
            RefAction::NoAction(it) => &it.syntax,
            RefAction::Restrict(it) => &it.syntax,
            RefAction::SetDefaultColumns(it) => &it.syntax,
            RefAction::SetNullColumns(it) => &it.syntax,
        }
    }
}
impl From<Cascade> for RefAction {
    #[inline]
    fn from(node: Cascade) -> RefAction {
        RefAction::Cascade(node)
    }
}
impl From<NoAction> for RefAction {
    #[inline]
    fn from(node: NoAction) -> RefAction {
        RefAction::NoAction(node)
    }
}
impl From<Restrict> for RefAction {
    #[inline]
    fn from(node: Restrict) -> RefAction {
        RefAction::Restrict(node)
    }
}
impl From<SetDefaultColumns> for RefAction {
    #[inline]
    fn from(node: SetDefaultColumns) -> RefAction {
        RefAction::SetDefaultColumns(node)
    }
}
impl From<SetNullColumns> for RefAction {
    #[inline]
    fn from(node: SetNullColumns) -> RefAction {
        RefAction::SetNullColumns(node)
    }
}
impl AstNode for Stmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            SyntaxKind::ALTER_AGGREGATE
                | SyntaxKind::ALTER_COLLATION
                | SyntaxKind::ALTER_CONVERSION
                | SyntaxKind::ALTER_DATABASE
                | SyntaxKind::ALTER_DEFAULT_PRIVILEGES
                | SyntaxKind::ALTER_DOMAIN
                | SyntaxKind::ALTER_EVENT_TRIGGER
                | SyntaxKind::ALTER_EXTENSION
                | SyntaxKind::ALTER_FOREIGN_DATA_WRAPPER
                | SyntaxKind::ALTER_FOREIGN_TABLE
                | SyntaxKind::ALTER_FUNCTION
                | SyntaxKind::ALTER_GROUP
                | SyntaxKind::ALTER_INDEX
                | SyntaxKind::ALTER_LANGUAGE
                | SyntaxKind::ALTER_LARGE_OBJECT
                | SyntaxKind::ALTER_MATERIALIZED_VIEW
                | SyntaxKind::ALTER_OPERATOR
                | SyntaxKind::ALTER_OPERATOR_CLASS
                | SyntaxKind::ALTER_OPERATOR_FAMILY
                | SyntaxKind::ALTER_POLICY
                | SyntaxKind::ALTER_PROCEDURE
                | SyntaxKind::ALTER_PUBLICATION
                | SyntaxKind::ALTER_ROLE
                | SyntaxKind::ALTER_ROUTINE
                | SyntaxKind::ALTER_RULE
                | SyntaxKind::ALTER_SCHEMA
                | SyntaxKind::ALTER_SEQUENCE
                | SyntaxKind::ALTER_SERVER
                | SyntaxKind::ALTER_STATISTICS
                | SyntaxKind::ALTER_SUBSCRIPTION
                | SyntaxKind::ALTER_SYSTEM
                | SyntaxKind::ALTER_TABLE
                | SyntaxKind::ALTER_TABLESPACE
                | SyntaxKind::ALTER_TEXT_SEARCH_CONFIGURATION
                | SyntaxKind::ALTER_TEXT_SEARCH_DICTIONARY
                | SyntaxKind::ALTER_TEXT_SEARCH_PARSER
                | SyntaxKind::ALTER_TEXT_SEARCH_TEMPLATE
                | SyntaxKind::ALTER_TRIGGER
                | SyntaxKind::ALTER_TYPE
                | SyntaxKind::ALTER_USER
                | SyntaxKind::ALTER_USER_MAPPING
                | SyntaxKind::ALTER_VIEW
                | SyntaxKind::ANALYZE
                | SyntaxKind::BEGIN
                | SyntaxKind::CALL
                | SyntaxKind::CHECKPOINT
                | SyntaxKind::CLOSE
                | SyntaxKind::CLUSTER
                | SyntaxKind::COMMENT_ON
                | SyntaxKind::COMMIT
                | SyntaxKind::COPY
                | SyntaxKind::CREATE_ACCESS_METHOD
                | SyntaxKind::CREATE_AGGREGATE
                | SyntaxKind::CREATE_CAST
                | SyntaxKind::CREATE_COLLATION
                | SyntaxKind::CREATE_CONVERSION
                | SyntaxKind::CREATE_DATABASE
                | SyntaxKind::CREATE_DOMAIN
                | SyntaxKind::CREATE_EVENT_TRIGGER
                | SyntaxKind::CREATE_EXTENSION
                | SyntaxKind::CREATE_FOREIGN_DATA_WRAPPER
                | SyntaxKind::CREATE_FOREIGN_TABLE
                | SyntaxKind::CREATE_FUNCTION
                | SyntaxKind::CREATE_GROUP
                | SyntaxKind::CREATE_INDEX
                | SyntaxKind::CREATE_LANGUAGE
                | SyntaxKind::CREATE_MATERIALIZED_VIEW
                | SyntaxKind::CREATE_OPERATOR
                | SyntaxKind::CREATE_OPERATOR_CLASS
                | SyntaxKind::CREATE_OPERATOR_FAMILY
                | SyntaxKind::CREATE_POLICY
                | SyntaxKind::CREATE_PROCEDURE
                | SyntaxKind::CREATE_PUBLICATION
                | SyntaxKind::CREATE_ROLE
                | SyntaxKind::CREATE_RULE
                | SyntaxKind::CREATE_SCHEMA
                | SyntaxKind::CREATE_SEQUENCE
                | SyntaxKind::CREATE_SERVER
                | SyntaxKind::CREATE_STATISTICS
                | SyntaxKind::CREATE_SUBSCRIPTION
                | SyntaxKind::CREATE_TABLE
                | SyntaxKind::CREATE_TABLE_AS
                | SyntaxKind::CREATE_TABLESPACE
                | SyntaxKind::CREATE_TEXT_SEARCH_CONFIGURATION
                | SyntaxKind::CREATE_TEXT_SEARCH_DICTIONARY
                | SyntaxKind::CREATE_TEXT_SEARCH_PARSER
                | SyntaxKind::CREATE_TEXT_SEARCH_TEMPLATE
                | SyntaxKind::CREATE_TRANSFORM
                | SyntaxKind::CREATE_TRIGGER
                | SyntaxKind::CREATE_TYPE
                | SyntaxKind::CREATE_USER
                | SyntaxKind::CREATE_USER_MAPPING
                | SyntaxKind::CREATE_VIEW
                | SyntaxKind::DEALLOCATE
                | SyntaxKind::DECLARE
                | SyntaxKind::DELETE
                | SyntaxKind::DISCARD
                | SyntaxKind::DO
                | SyntaxKind::DROP_ACCESS_METHOD
                | SyntaxKind::DROP_AGGREGATE
                | SyntaxKind::DROP_CAST
                | SyntaxKind::DROP_COLLATION
                | SyntaxKind::DROP_CONVERSION
                | SyntaxKind::DROP_DATABASE
                | SyntaxKind::DROP_DOMAIN
                | SyntaxKind::DROP_EVENT_TRIGGER
                | SyntaxKind::DROP_EXTENSION
                | SyntaxKind::DROP_FOREIGN_DATA_WRAPPER
                | SyntaxKind::DROP_FOREIGN_TABLE
                | SyntaxKind::DROP_FUNCTION
                | SyntaxKind::DROP_GROUP
                | SyntaxKind::DROP_INDEX
                | SyntaxKind::DROP_LANGUAGE
                | SyntaxKind::DROP_MATERIALIZED_VIEW
                | SyntaxKind::DROP_OPERATOR
                | SyntaxKind::DROP_OPERATOR_CLASS
                | SyntaxKind::DROP_OPERATOR_FAMILY
                | SyntaxKind::DROP_OWNED
                | SyntaxKind::DROP_POLICY
                | SyntaxKind::DROP_PROCEDURE
                | SyntaxKind::DROP_PUBLICATION
                | SyntaxKind::DROP_ROLE
                | SyntaxKind::DROP_ROUTINE
                | SyntaxKind::DROP_RULE
                | SyntaxKind::DROP_SCHEMA
                | SyntaxKind::DROP_SEQUENCE
                | SyntaxKind::DROP_SERVER
                | SyntaxKind::DROP_STATISTICS
                | SyntaxKind::DROP_SUBSCRIPTION
                | SyntaxKind::DROP_TABLE
                | SyntaxKind::DROP_TABLESPACE
                | SyntaxKind::DROP_TEXT_SEARCH_CONFIG
                | SyntaxKind::DROP_TEXT_SEARCH_DICT
                | SyntaxKind::DROP_TEXT_SEARCH_PARSER
                | SyntaxKind::DROP_TEXT_SEARCH_TEMPLATE
                | SyntaxKind::DROP_TRANSFORM
                | SyntaxKind::DROP_TRIGGER
                | SyntaxKind::DROP_TYPE
                | SyntaxKind::DROP_USER
                | SyntaxKind::DROP_USER_MAPPING
                | SyntaxKind::DROP_VIEW
                | SyntaxKind::EXECUTE
                | SyntaxKind::EXPLAIN
                | SyntaxKind::FETCH
                | SyntaxKind::GRANT
                | SyntaxKind::IMPORT_FOREIGN_SCHEMA
                | SyntaxKind::INSERT
                | SyntaxKind::LISTEN
                | SyntaxKind::LOAD
                | SyntaxKind::LOCK
                | SyntaxKind::MERGE
                | SyntaxKind::MOVE
                | SyntaxKind::NOTIFY
                | SyntaxKind::PREPARE
                | SyntaxKind::PREPARE_TRANSACTION
                | SyntaxKind::REASSIGN
                | SyntaxKind::REFRESH
                | SyntaxKind::REINDEX
                | SyntaxKind::RELEASE_SAVEPOINT
                | SyntaxKind::RESET
                | SyntaxKind::REVOKE
                | SyntaxKind::ROLLBACK
                | SyntaxKind::SAVEPOINT
                | SyntaxKind::SECURITY_LABEL
                | SyntaxKind::SELECT
                | SyntaxKind::SELECT_INTO
                | SyntaxKind::SET
                | SyntaxKind::SET_CONSTRAINTS
                | SyntaxKind::SET_ROLE
                | SyntaxKind::SET_SESSION_AUTH
                | SyntaxKind::SET_TRANSACTION
                | SyntaxKind::SHOW
                | SyntaxKind::TRUNCATE
                | SyntaxKind::UNLISTEN
                | SyntaxKind::UPDATE
                | SyntaxKind::VACUUM
                | SyntaxKind::VALUES
        )
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            SyntaxKind::ALTER_AGGREGATE => Stmt::AlterAggregate(AlterAggregate { syntax }),
            SyntaxKind::ALTER_COLLATION => Stmt::AlterCollation(AlterCollation { syntax }),
            SyntaxKind::ALTER_CONVERSION => Stmt::AlterConversion(AlterConversion { syntax }),
            SyntaxKind::ALTER_DATABASE => Stmt::AlterDatabase(AlterDatabase { syntax }),
            SyntaxKind::ALTER_DEFAULT_PRIVILEGES => {
                Stmt::AlterDefaultPrivileges(AlterDefaultPrivileges { syntax })
            }
            SyntaxKind::ALTER_DOMAIN => Stmt::AlterDomain(AlterDomain { syntax }),
            SyntaxKind::ALTER_EVENT_TRIGGER => {
                Stmt::AlterEventTrigger(AlterEventTrigger { syntax })
            }
            SyntaxKind::ALTER_EXTENSION => Stmt::AlterExtension(AlterExtension { syntax }),
            SyntaxKind::ALTER_FOREIGN_DATA_WRAPPER => {
                Stmt::AlterForeignDataWrapper(AlterForeignDataWrapper { syntax })
            }
            SyntaxKind::ALTER_FOREIGN_TABLE => {
                Stmt::AlterForeignTable(AlterForeignTable { syntax })
            }
            SyntaxKind::ALTER_FUNCTION => Stmt::AlterFunction(AlterFunction { syntax }),
            SyntaxKind::ALTER_GROUP => Stmt::AlterGroup(AlterGroup { syntax }),
            SyntaxKind::ALTER_INDEX => Stmt::AlterIndex(AlterIndex { syntax }),
            SyntaxKind::ALTER_LANGUAGE => Stmt::AlterLanguage(AlterLanguage { syntax }),
            SyntaxKind::ALTER_LARGE_OBJECT => Stmt::AlterLargeObject(AlterLargeObject { syntax }),
            SyntaxKind::ALTER_MATERIALIZED_VIEW => {
                Stmt::AlterMaterializedView(AlterMaterializedView { syntax })
            }
            SyntaxKind::ALTER_OPERATOR => Stmt::AlterOperator(AlterOperator { syntax }),
            SyntaxKind::ALTER_OPERATOR_CLASS => {
                Stmt::AlterOperatorClass(AlterOperatorClass { syntax })
            }
            SyntaxKind::ALTER_OPERATOR_FAMILY => {
                Stmt::AlterOperatorFamily(AlterOperatorFamily { syntax })
            }
            SyntaxKind::ALTER_POLICY => Stmt::AlterPolicy(AlterPolicy { syntax }),
            SyntaxKind::ALTER_PROCEDURE => Stmt::AlterProcedure(AlterProcedure { syntax }),
            SyntaxKind::ALTER_PUBLICATION => Stmt::AlterPublication(AlterPublication { syntax }),
            SyntaxKind::ALTER_ROLE => Stmt::AlterRole(AlterRole { syntax }),
            SyntaxKind::ALTER_ROUTINE => Stmt::AlterRoutine(AlterRoutine { syntax }),
            SyntaxKind::ALTER_RULE => Stmt::AlterRule(AlterRule { syntax }),
            SyntaxKind::ALTER_SCHEMA => Stmt::AlterSchema(AlterSchema { syntax }),
            SyntaxKind::ALTER_SEQUENCE => Stmt::AlterSequence(AlterSequence { syntax }),
            SyntaxKind::ALTER_SERVER => Stmt::AlterServer(AlterServer { syntax }),
            SyntaxKind::ALTER_STATISTICS => Stmt::AlterStatistics(AlterStatistics { syntax }),
            SyntaxKind::ALTER_SUBSCRIPTION => Stmt::AlterSubscription(AlterSubscription { syntax }),
            SyntaxKind::ALTER_SYSTEM => Stmt::AlterSystem(AlterSystem { syntax }),
            SyntaxKind::ALTER_TABLE => Stmt::AlterTable(AlterTable { syntax }),
            SyntaxKind::ALTER_TABLESPACE => Stmt::AlterTablespace(AlterTablespace { syntax }),
            SyntaxKind::ALTER_TEXT_SEARCH_CONFIGURATION => {
                Stmt::AlterTextSearchConfiguration(AlterTextSearchConfiguration { syntax })
            }
            SyntaxKind::ALTER_TEXT_SEARCH_DICTIONARY => {
                Stmt::AlterTextSearchDictionary(AlterTextSearchDictionary { syntax })
            }
            SyntaxKind::ALTER_TEXT_SEARCH_PARSER => {
                Stmt::AlterTextSearchParser(AlterTextSearchParser { syntax })
            }
            SyntaxKind::ALTER_TEXT_SEARCH_TEMPLATE => {
                Stmt::AlterTextSearchTemplate(AlterTextSearchTemplate { syntax })
            }
            SyntaxKind::ALTER_TRIGGER => Stmt::AlterTrigger(AlterTrigger { syntax }),
            SyntaxKind::ALTER_TYPE => Stmt::AlterType(AlterType { syntax }),
            SyntaxKind::ALTER_USER => Stmt::AlterUser(AlterUser { syntax }),
            SyntaxKind::ALTER_USER_MAPPING => Stmt::AlterUserMapping(AlterUserMapping { syntax }),
            SyntaxKind::ALTER_VIEW => Stmt::AlterView(AlterView { syntax }),
            SyntaxKind::ANALYZE => Stmt::Analyze(Analyze { syntax }),
            SyntaxKind::BEGIN => Stmt::Begin(Begin { syntax }),
            SyntaxKind::CALL => Stmt::Call(Call { syntax }),
            SyntaxKind::CHECKPOINT => Stmt::Checkpoint(Checkpoint { syntax }),
            SyntaxKind::CLOSE => Stmt::Close(Close { syntax }),
            SyntaxKind::CLUSTER => Stmt::Cluster(Cluster { syntax }),
            SyntaxKind::COMMENT_ON => Stmt::CommentOn(CommentOn { syntax }),
            SyntaxKind::COMMIT => Stmt::Commit(Commit { syntax }),
            SyntaxKind::COPY => Stmt::Copy(Copy { syntax }),
            SyntaxKind::CREATE_ACCESS_METHOD => {
                Stmt::CreateAccessMethod(CreateAccessMethod { syntax })
            }
            SyntaxKind::CREATE_AGGREGATE => Stmt::CreateAggregate(CreateAggregate { syntax }),
            SyntaxKind::CREATE_CAST => Stmt::CreateCast(CreateCast { syntax }),
            SyntaxKind::CREATE_COLLATION => Stmt::CreateCollation(CreateCollation { syntax }),
            SyntaxKind::CREATE_CONVERSION => Stmt::CreateConversion(CreateConversion { syntax }),
            SyntaxKind::CREATE_DATABASE => Stmt::CreateDatabase(CreateDatabase { syntax }),
            SyntaxKind::CREATE_DOMAIN => Stmt::CreateDomain(CreateDomain { syntax }),
            SyntaxKind::CREATE_EVENT_TRIGGER => {
                Stmt::CreateEventTrigger(CreateEventTrigger { syntax })
            }
            SyntaxKind::CREATE_EXTENSION => Stmt::CreateExtension(CreateExtension { syntax }),
            SyntaxKind::CREATE_FOREIGN_DATA_WRAPPER => {
                Stmt::CreateForeignDataWrapper(CreateForeignDataWrapper { syntax })
            }
            SyntaxKind::CREATE_FOREIGN_TABLE => {
                Stmt::CreateForeignTable(CreateForeignTable { syntax })
            }
            SyntaxKind::CREATE_FUNCTION => Stmt::CreateFunction(CreateFunction { syntax }),
            SyntaxKind::CREATE_GROUP => Stmt::CreateGroup(CreateGroup { syntax }),
            SyntaxKind::CREATE_INDEX => Stmt::CreateIndex(CreateIndex { syntax }),
            SyntaxKind::CREATE_LANGUAGE => Stmt::CreateLanguage(CreateLanguage { syntax }),
            SyntaxKind::CREATE_MATERIALIZED_VIEW => {
                Stmt::CreateMaterializedView(CreateMaterializedView { syntax })
            }
            SyntaxKind::CREATE_OPERATOR => Stmt::CreateOperator(CreateOperator { syntax }),
            SyntaxKind::CREATE_OPERATOR_CLASS => {
                Stmt::CreateOperatorClass(CreateOperatorClass { syntax })
            }
            SyntaxKind::CREATE_OPERATOR_FAMILY => {
                Stmt::CreateOperatorFamily(CreateOperatorFamily { syntax })
            }
            SyntaxKind::CREATE_POLICY => Stmt::CreatePolicy(CreatePolicy { syntax }),
            SyntaxKind::CREATE_PROCEDURE => Stmt::CreateProcedure(CreateProcedure { syntax }),
            SyntaxKind::CREATE_PUBLICATION => Stmt::CreatePublication(CreatePublication { syntax }),
            SyntaxKind::CREATE_ROLE => Stmt::CreateRole(CreateRole { syntax }),
            SyntaxKind::CREATE_RULE => Stmt::CreateRule(CreateRule { syntax }),
            SyntaxKind::CREATE_SCHEMA => Stmt::CreateSchema(CreateSchema { syntax }),
            SyntaxKind::CREATE_SEQUENCE => Stmt::CreateSequence(CreateSequence { syntax }),
            SyntaxKind::CREATE_SERVER => Stmt::CreateServer(CreateServer { syntax }),
            SyntaxKind::CREATE_STATISTICS => Stmt::CreateStatistics(CreateStatistics { syntax }),
            SyntaxKind::CREATE_SUBSCRIPTION => {
                Stmt::CreateSubscription(CreateSubscription { syntax })
            }
            SyntaxKind::CREATE_TABLE => Stmt::CreateTable(CreateTable { syntax }),
            SyntaxKind::CREATE_TABLE_AS => Stmt::CreateTableAs(CreateTableAs { syntax }),
            SyntaxKind::CREATE_TABLESPACE => Stmt::CreateTablespace(CreateTablespace { syntax }),
            SyntaxKind::CREATE_TEXT_SEARCH_CONFIGURATION => {
                Stmt::CreateTextSearchConfiguration(CreateTextSearchConfiguration { syntax })
            }
            SyntaxKind::CREATE_TEXT_SEARCH_DICTIONARY => {
                Stmt::CreateTextSearchDictionary(CreateTextSearchDictionary { syntax })
            }
            SyntaxKind::CREATE_TEXT_SEARCH_PARSER => {
                Stmt::CreateTextSearchParser(CreateTextSearchParser { syntax })
            }
            SyntaxKind::CREATE_TEXT_SEARCH_TEMPLATE => {
                Stmt::CreateTextSearchTemplate(CreateTextSearchTemplate { syntax })
            }
            SyntaxKind::CREATE_TRANSFORM => Stmt::CreateTransform(CreateTransform { syntax }),
            SyntaxKind::CREATE_TRIGGER => Stmt::CreateTrigger(CreateTrigger { syntax }),
            SyntaxKind::CREATE_TYPE => Stmt::CreateType(CreateType { syntax }),
            SyntaxKind::CREATE_USER => Stmt::CreateUser(CreateUser { syntax }),
            SyntaxKind::CREATE_USER_MAPPING => {
                Stmt::CreateUserMapping(CreateUserMapping { syntax })
            }
            SyntaxKind::CREATE_VIEW => Stmt::CreateView(CreateView { syntax }),
            SyntaxKind::DEALLOCATE => Stmt::Deallocate(Deallocate { syntax }),
            SyntaxKind::DECLARE => Stmt::Declare(Declare { syntax }),
            SyntaxKind::DELETE => Stmt::Delete(Delete { syntax }),
            SyntaxKind::DISCARD => Stmt::Discard(Discard { syntax }),
            SyntaxKind::DO => Stmt::Do(Do { syntax }),
            SyntaxKind::DROP_ACCESS_METHOD => Stmt::DropAccessMethod(DropAccessMethod { syntax }),
            SyntaxKind::DROP_AGGREGATE => Stmt::DropAggregate(DropAggregate { syntax }),
            SyntaxKind::DROP_CAST => Stmt::DropCast(DropCast { syntax }),
            SyntaxKind::DROP_COLLATION => Stmt::DropCollation(DropCollation { syntax }),
            SyntaxKind::DROP_CONVERSION => Stmt::DropConversion(DropConversion { syntax }),
            SyntaxKind::DROP_DATABASE => Stmt::DropDatabase(DropDatabase { syntax }),
            SyntaxKind::DROP_DOMAIN => Stmt::DropDomain(DropDomain { syntax }),
            SyntaxKind::DROP_EVENT_TRIGGER => Stmt::DropEventTrigger(DropEventTrigger { syntax }),
            SyntaxKind::DROP_EXTENSION => Stmt::DropExtension(DropExtension { syntax }),
            SyntaxKind::DROP_FOREIGN_DATA_WRAPPER => {
                Stmt::DropForeignDataWrapper(DropForeignDataWrapper { syntax })
            }
            SyntaxKind::DROP_FOREIGN_TABLE => Stmt::DropForeignTable(DropForeignTable { syntax }),
            SyntaxKind::DROP_FUNCTION => Stmt::DropFunction(DropFunction { syntax }),
            SyntaxKind::DROP_GROUP => Stmt::DropGroup(DropGroup { syntax }),
            SyntaxKind::DROP_INDEX => Stmt::DropIndex(DropIndex { syntax }),
            SyntaxKind::DROP_LANGUAGE => Stmt::DropLanguage(DropLanguage { syntax }),
            SyntaxKind::DROP_MATERIALIZED_VIEW => {
                Stmt::DropMaterializedView(DropMaterializedView { syntax })
            }
            SyntaxKind::DROP_OPERATOR => Stmt::DropOperator(DropOperator { syntax }),
            SyntaxKind::DROP_OPERATOR_CLASS => {
                Stmt::DropOperatorClass(DropOperatorClass { syntax })
            }
            SyntaxKind::DROP_OPERATOR_FAMILY => {
                Stmt::DropOperatorFamily(DropOperatorFamily { syntax })
            }
            SyntaxKind::DROP_OWNED => Stmt::DropOwned(DropOwned { syntax }),
            SyntaxKind::DROP_POLICY => Stmt::DropPolicy(DropPolicy { syntax }),
            SyntaxKind::DROP_PROCEDURE => Stmt::DropProcedure(DropProcedure { syntax }),
            SyntaxKind::DROP_PUBLICATION => Stmt::DropPublication(DropPublication { syntax }),
            SyntaxKind::DROP_ROLE => Stmt::DropRole(DropRole { syntax }),
            SyntaxKind::DROP_ROUTINE => Stmt::DropRoutine(DropRoutine { syntax }),
            SyntaxKind::DROP_RULE => Stmt::DropRule(DropRule { syntax }),
            SyntaxKind::DROP_SCHEMA => Stmt::DropSchema(DropSchema { syntax }),
            SyntaxKind::DROP_SEQUENCE => Stmt::DropSequence(DropSequence { syntax }),
            SyntaxKind::DROP_SERVER => Stmt::DropServer(DropServer { syntax }),
            SyntaxKind::DROP_STATISTICS => Stmt::DropStatistics(DropStatistics { syntax }),
            SyntaxKind::DROP_SUBSCRIPTION => Stmt::DropSubscription(DropSubscription { syntax }),
            SyntaxKind::DROP_TABLE => Stmt::DropTable(DropTable { syntax }),
            SyntaxKind::DROP_TABLESPACE => Stmt::DropTablespace(DropTablespace { syntax }),
            SyntaxKind::DROP_TEXT_SEARCH_CONFIG => {
                Stmt::DropTextSearchConfig(DropTextSearchConfig { syntax })
            }
            SyntaxKind::DROP_TEXT_SEARCH_DICT => {
                Stmt::DropTextSearchDict(DropTextSearchDict { syntax })
            }
            SyntaxKind::DROP_TEXT_SEARCH_PARSER => {
                Stmt::DropTextSearchParser(DropTextSearchParser { syntax })
            }
            SyntaxKind::DROP_TEXT_SEARCH_TEMPLATE => {
                Stmt::DropTextSearchTemplate(DropTextSearchTemplate { syntax })
            }
            SyntaxKind::DROP_TRANSFORM => Stmt::DropTransform(DropTransform { syntax }),
            SyntaxKind::DROP_TRIGGER => Stmt::DropTrigger(DropTrigger { syntax }),
            SyntaxKind::DROP_TYPE => Stmt::DropType(DropType { syntax }),
            SyntaxKind::DROP_USER => Stmt::DropUser(DropUser { syntax }),
            SyntaxKind::DROP_USER_MAPPING => Stmt::DropUserMapping(DropUserMapping { syntax }),
            SyntaxKind::DROP_VIEW => Stmt::DropView(DropView { syntax }),
            SyntaxKind::EXECUTE => Stmt::Execute(Execute { syntax }),
            SyntaxKind::EXPLAIN => Stmt::Explain(Explain { syntax }),
            SyntaxKind::FETCH => Stmt::Fetch(Fetch { syntax }),
            SyntaxKind::GRANT => Stmt::Grant(Grant { syntax }),
            SyntaxKind::IMPORT_FOREIGN_SCHEMA => {
                Stmt::ImportForeignSchema(ImportForeignSchema { syntax })
            }
            SyntaxKind::INSERT => Stmt::Insert(Insert { syntax }),
            SyntaxKind::LISTEN => Stmt::Listen(Listen { syntax }),
            SyntaxKind::LOAD => Stmt::Load(Load { syntax }),
            SyntaxKind::LOCK => Stmt::Lock(Lock { syntax }),
            SyntaxKind::MERGE => Stmt::Merge(Merge { syntax }),
            SyntaxKind::MOVE => Stmt::Move(Move { syntax }),
            SyntaxKind::NOTIFY => Stmt::Notify(Notify { syntax }),
            SyntaxKind::PREPARE => Stmt::Prepare(Prepare { syntax }),
            SyntaxKind::PREPARE_TRANSACTION => {
                Stmt::PrepareTransaction(PrepareTransaction { syntax })
            }
            SyntaxKind::REASSIGN => Stmt::Reassign(Reassign { syntax }),
            SyntaxKind::REFRESH => Stmt::Refresh(Refresh { syntax }),
            SyntaxKind::REINDEX => Stmt::Reindex(Reindex { syntax }),
            SyntaxKind::RELEASE_SAVEPOINT => Stmt::ReleaseSavepoint(ReleaseSavepoint { syntax }),
            SyntaxKind::RESET => Stmt::Reset(Reset { syntax }),
            SyntaxKind::REVOKE => Stmt::Revoke(Revoke { syntax }),
            SyntaxKind::ROLLBACK => Stmt::Rollback(Rollback { syntax }),
            SyntaxKind::SAVEPOINT => Stmt::Savepoint(Savepoint { syntax }),
            SyntaxKind::SECURITY_LABEL => Stmt::SecurityLabel(SecurityLabel { syntax }),
            SyntaxKind::SELECT => Stmt::Select(Select { syntax }),
            SyntaxKind::SELECT_INTO => Stmt::SelectInto(SelectInto { syntax }),
            SyntaxKind::SET => Stmt::Set(Set { syntax }),
            SyntaxKind::SET_CONSTRAINTS => Stmt::SetConstraints(SetConstraints { syntax }),
            SyntaxKind::SET_ROLE => Stmt::SetRole(SetRole { syntax }),
            SyntaxKind::SET_SESSION_AUTH => Stmt::SetSessionAuth(SetSessionAuth { syntax }),
            SyntaxKind::SET_TRANSACTION => Stmt::SetTransaction(SetTransaction { syntax }),
            SyntaxKind::SHOW => Stmt::Show(Show { syntax }),
            SyntaxKind::TRUNCATE => Stmt::Truncate(Truncate { syntax }),
            SyntaxKind::UNLISTEN => Stmt::Unlisten(Unlisten { syntax }),
            SyntaxKind::UPDATE => Stmt::Update(Update { syntax }),
            SyntaxKind::VACUUM => Stmt::Vacuum(Vacuum { syntax }),
            SyntaxKind::VALUES => Stmt::Values(Values { syntax }),
            _ => {
                return None;
            }
        };
        Some(res)
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Stmt::AlterAggregate(it) => &it.syntax,
            Stmt::AlterCollation(it) => &it.syntax,
            Stmt::AlterConversion(it) => &it.syntax,
            Stmt::AlterDatabase(it) => &it.syntax,
            Stmt::AlterDefaultPrivileges(it) => &it.syntax,
            Stmt::AlterDomain(it) => &it.syntax,
            Stmt::AlterEventTrigger(it) => &it.syntax,
            Stmt::AlterExtension(it) => &it.syntax,
            Stmt::AlterForeignDataWrapper(it) => &it.syntax,
            Stmt::AlterForeignTable(it) => &it.syntax,
            Stmt::AlterFunction(it) => &it.syntax,
            Stmt::AlterGroup(it) => &it.syntax,
            Stmt::AlterIndex(it) => &it.syntax,
            Stmt::AlterLanguage(it) => &it.syntax,
            Stmt::AlterLargeObject(it) => &it.syntax,
            Stmt::AlterMaterializedView(it) => &it.syntax,
            Stmt::AlterOperator(it) => &it.syntax,
            Stmt::AlterOperatorClass(it) => &it.syntax,
            Stmt::AlterOperatorFamily(it) => &it.syntax,
            Stmt::AlterPolicy(it) => &it.syntax,
            Stmt::AlterProcedure(it) => &it.syntax,
            Stmt::AlterPublication(it) => &it.syntax,
            Stmt::AlterRole(it) => &it.syntax,
            Stmt::AlterRoutine(it) => &it.syntax,
            Stmt::AlterRule(it) => &it.syntax,
            Stmt::AlterSchema(it) => &it.syntax,
            Stmt::AlterSequence(it) => &it.syntax,
            Stmt::AlterServer(it) => &it.syntax,
            Stmt::AlterStatistics(it) => &it.syntax,
            Stmt::AlterSubscription(it) => &it.syntax,
            Stmt::AlterSystem(it) => &it.syntax,
            Stmt::AlterTable(it) => &it.syntax,
            Stmt::AlterTablespace(it) => &it.syntax,
            Stmt::AlterTextSearchConfiguration(it) => &it.syntax,
            Stmt::AlterTextSearchDictionary(it) => &it.syntax,
            Stmt::AlterTextSearchParser(it) => &it.syntax,
            Stmt::AlterTextSearchTemplate(it) => &it.syntax,
            Stmt::AlterTrigger(it) => &it.syntax,
            Stmt::AlterType(it) => &it.syntax,
            Stmt::AlterUser(it) => &it.syntax,
            Stmt::AlterUserMapping(it) => &it.syntax,
            Stmt::AlterView(it) => &it.syntax,
            Stmt::Analyze(it) => &it.syntax,
            Stmt::Begin(it) => &it.syntax,
            Stmt::Call(it) => &it.syntax,
            Stmt::Checkpoint(it) => &it.syntax,
            Stmt::Close(it) => &it.syntax,
            Stmt::Cluster(it) => &it.syntax,
            Stmt::CommentOn(it) => &it.syntax,
            Stmt::Commit(it) => &it.syntax,
            Stmt::Copy(it) => &it.syntax,
            Stmt::CreateAccessMethod(it) => &it.syntax,
            Stmt::CreateAggregate(it) => &it.syntax,
            Stmt::CreateCast(it) => &it.syntax,
            Stmt::CreateCollation(it) => &it.syntax,
            Stmt::CreateConversion(it) => &it.syntax,
            Stmt::CreateDatabase(it) => &it.syntax,
            Stmt::CreateDomain(it) => &it.syntax,
            Stmt::CreateEventTrigger(it) => &it.syntax,
            Stmt::CreateExtension(it) => &it.syntax,
            Stmt::CreateForeignDataWrapper(it) => &it.syntax,
            Stmt::CreateForeignTable(it) => &it.syntax,
            Stmt::CreateFunction(it) => &it.syntax,
            Stmt::CreateGroup(it) => &it.syntax,
            Stmt::CreateIndex(it) => &it.syntax,
            Stmt::CreateLanguage(it) => &it.syntax,
            Stmt::CreateMaterializedView(it) => &it.syntax,
            Stmt::CreateOperator(it) => &it.syntax,
            Stmt::CreateOperatorClass(it) => &it.syntax,
            Stmt::CreateOperatorFamily(it) => &it.syntax,
            Stmt::CreatePolicy(it) => &it.syntax,
            Stmt::CreateProcedure(it) => &it.syntax,
            Stmt::CreatePublication(it) => &it.syntax,
            Stmt::CreateRole(it) => &it.syntax,
            Stmt::CreateRule(it) => &it.syntax,
            Stmt::CreateSchema(it) => &it.syntax,
            Stmt::CreateSequence(it) => &it.syntax,
            Stmt::CreateServer(it) => &it.syntax,
            Stmt::CreateStatistics(it) => &it.syntax,
            Stmt::CreateSubscription(it) => &it.syntax,
            Stmt::CreateTable(it) => &it.syntax,
            Stmt::CreateTableAs(it) => &it.syntax,
            Stmt::CreateTablespace(it) => &it.syntax,
            Stmt::CreateTextSearchConfiguration(it) => &it.syntax,
            Stmt::CreateTextSearchDictionary(it) => &it.syntax,
            Stmt::CreateTextSearchParser(it) => &it.syntax,
            Stmt::CreateTextSearchTemplate(it) => &it.syntax,
            Stmt::CreateTransform(it) => &it.syntax,
            Stmt::CreateTrigger(it) => &it.syntax,
            Stmt::CreateType(it) => &it.syntax,
            Stmt::CreateUser(it) => &it.syntax,
            Stmt::CreateUserMapping(it) => &it.syntax,
            Stmt::CreateView(it) => &it.syntax,
            Stmt::Deallocate(it) => &it.syntax,
            Stmt::Declare(it) => &it.syntax,
            Stmt::Delete(it) => &it.syntax,
            Stmt::Discard(it) => &it.syntax,
            Stmt::Do(it) => &it.syntax,
            Stmt::DropAccessMethod(it) => &it.syntax,
            Stmt::DropAggregate(it) => &it.syntax,
            Stmt::DropCast(it) => &it.syntax,
            Stmt::DropCollation(it) => &it.syntax,
            Stmt::DropConversion(it) => &it.syntax,
            Stmt::DropDatabase(it) => &it.syntax,
            Stmt::DropDomain(it) => &it.syntax,
            Stmt::DropEventTrigger(it) => &it.syntax,
            Stmt::DropExtension(it) => &it.syntax,
            Stmt::DropForeignDataWrapper(it) => &it.syntax,
            Stmt::DropForeignTable(it) => &it.syntax,
            Stmt::DropFunction(it) => &it.syntax,
            Stmt::DropGroup(it) => &it.syntax,
            Stmt::DropIndex(it) => &it.syntax,
            Stmt::DropLanguage(it) => &it.syntax,
            Stmt::DropMaterializedView(it) => &it.syntax,
            Stmt::DropOperator(it) => &it.syntax,
            Stmt::DropOperatorClass(it) => &it.syntax,
            Stmt::DropOperatorFamily(it) => &it.syntax,
            Stmt::DropOwned(it) => &it.syntax,
            Stmt::DropPolicy(it) => &it.syntax,
            Stmt::DropProcedure(it) => &it.syntax,
            Stmt::DropPublication(it) => &it.syntax,
            Stmt::DropRole(it) => &it.syntax,
            Stmt::DropRoutine(it) => &it.syntax,
            Stmt::DropRule(it) => &it.syntax,
            Stmt::DropSchema(it) => &it.syntax,
            Stmt::DropSequence(it) => &it.syntax,
            Stmt::DropServer(it) => &it.syntax,
            Stmt::DropStatistics(it) => &it.syntax,
            Stmt::DropSubscription(it) => &it.syntax,
            Stmt::DropTable(it) => &it.syntax,
            Stmt::DropTablespace(it) => &it.syntax,
            Stmt::DropTextSearchConfig(it) => &it.syntax,
            Stmt::DropTextSearchDict(it) => &it.syntax,
            Stmt::DropTextSearchParser(it) => &it.syntax,
            Stmt::DropTextSearchTemplate(it) => &it.syntax,
            Stmt::DropTransform(it) => &it.syntax,
            Stmt::DropTrigger(it) => &it.syntax,
            Stmt::DropType(it) => &it.syntax,
            Stmt::DropUser(it) => &it.syntax,
            Stmt::DropUserMapping(it) => &it.syntax,
            Stmt::DropView(it) => &it.syntax,
            Stmt::Execute(it) => &it.syntax,
            Stmt::Explain(it) => &it.syntax,
            Stmt::Fetch(it) => &it.syntax,
            Stmt::Grant(it) => &it.syntax,
            Stmt::ImportForeignSchema(it) => &it.syntax,
            Stmt::Insert(it) => &it.syntax,
            Stmt::Listen(it) => &it.syntax,
            Stmt::Load(it) => &it.syntax,
            Stmt::Lock(it) => &it.syntax,
            Stmt::Merge(it) => &it.syntax,
            Stmt::Move(it) => &it.syntax,
            Stmt::Notify(it) => &it.syntax,
            Stmt::Prepare(it) => &it.syntax,
            Stmt::PrepareTransaction(it) => &it.syntax,
            Stmt::Reassign(it) => &it.syntax,
            Stmt::Refresh(it) => &it.syntax,
            Stmt::Reindex(it) => &it.syntax,
            Stmt::ReleaseSavepoint(it) => &it.syntax,
            Stmt::Reset(it) => &it.syntax,
            Stmt::Revoke(it) => &it.syntax,
            Stmt::Rollback(it) => &it.syntax,
            Stmt::Savepoint(it) => &it.syntax,
            Stmt::SecurityLabel(it) => &it.syntax,
            Stmt::Select(it) => &it.syntax,
            Stmt::SelectInto(it) => &it.syntax,
            Stmt::Set(it) => &it.syntax,
            Stmt::SetConstraints(it) => &it.syntax,
            Stmt::SetRole(it) => &it.syntax,
            Stmt::SetSessionAuth(it) => &it.syntax,
            Stmt::SetTransaction(it) => &it.syntax,
            Stmt::Show(it) => &it.syntax,
            Stmt::Truncate(it) => &it.syntax,
            Stmt::Unlisten(it) => &it.syntax,
            Stmt::Update(it) => &it.syntax,
            Stmt::Vacuum(it) => &it.syntax,
            Stmt::Values(it) => &it.syntax,
        }
    }
}
impl From<AlterAggregate> for Stmt {
    #[inline]
    fn from(node: AlterAggregate) -> Stmt {
        Stmt::AlterAggregate(node)
    }
}
impl From<AlterCollation> for Stmt {
    #[inline]
    fn from(node: AlterCollation) -> Stmt {
        Stmt::AlterCollation(node)
    }
}
impl From<AlterConversion> for Stmt {
    #[inline]
    fn from(node: AlterConversion) -> Stmt {
        Stmt::AlterConversion(node)
    }
}
impl From<AlterDatabase> for Stmt {
    #[inline]
    fn from(node: AlterDatabase) -> Stmt {
        Stmt::AlterDatabase(node)
    }
}
impl From<AlterDefaultPrivileges> for Stmt {
    #[inline]
    fn from(node: AlterDefaultPrivileges) -> Stmt {
        Stmt::AlterDefaultPrivileges(node)
    }
}
impl From<AlterDomain> for Stmt {
    #[inline]
    fn from(node: AlterDomain) -> Stmt {
        Stmt::AlterDomain(node)
    }
}
impl From<AlterEventTrigger> for Stmt {
    #[inline]
    fn from(node: AlterEventTrigger) -> Stmt {
        Stmt::AlterEventTrigger(node)
    }
}
impl From<AlterExtension> for Stmt {
    #[inline]
    fn from(node: AlterExtension) -> Stmt {
        Stmt::AlterExtension(node)
    }
}
impl From<AlterForeignDataWrapper> for Stmt {
    #[inline]
    fn from(node: AlterForeignDataWrapper) -> Stmt {
        Stmt::AlterForeignDataWrapper(node)
    }
}
impl From<AlterForeignTable> for Stmt {
    #[inline]
    fn from(node: AlterForeignTable) -> Stmt {
        Stmt::AlterForeignTable(node)
    }
}
impl From<AlterFunction> for Stmt {
    #[inline]
    fn from(node: AlterFunction) -> Stmt {
        Stmt::AlterFunction(node)
    }
}
impl From<AlterGroup> for Stmt {
    #[inline]
    fn from(node: AlterGroup) -> Stmt {
        Stmt::AlterGroup(node)
    }
}
impl From<AlterIndex> for Stmt {
    #[inline]
    fn from(node: AlterIndex) -> Stmt {
        Stmt::AlterIndex(node)
    }
}
impl From<AlterLanguage> for Stmt {
    #[inline]
    fn from(node: AlterLanguage) -> Stmt {
        Stmt::AlterLanguage(node)
    }
}
impl From<AlterLargeObject> for Stmt {
    #[inline]
    fn from(node: AlterLargeObject) -> Stmt {
        Stmt::AlterLargeObject(node)
    }
}
impl From<AlterMaterializedView> for Stmt {
    #[inline]
    fn from(node: AlterMaterializedView) -> Stmt {
        Stmt::AlterMaterializedView(node)
    }
}
impl From<AlterOperator> for Stmt {
    #[inline]
    fn from(node: AlterOperator) -> Stmt {
        Stmt::AlterOperator(node)
    }
}
impl From<AlterOperatorClass> for Stmt {
    #[inline]
    fn from(node: AlterOperatorClass) -> Stmt {
        Stmt::AlterOperatorClass(node)
    }
}
impl From<AlterOperatorFamily> for Stmt {
    #[inline]
    fn from(node: AlterOperatorFamily) -> Stmt {
        Stmt::AlterOperatorFamily(node)
    }
}
impl From<AlterPolicy> for Stmt {
    #[inline]
    fn from(node: AlterPolicy) -> Stmt {
        Stmt::AlterPolicy(node)
    }
}
impl From<AlterProcedure> for Stmt {
    #[inline]
    fn from(node: AlterProcedure) -> Stmt {
        Stmt::AlterProcedure(node)
    }
}
impl From<AlterPublication> for Stmt {
    #[inline]
    fn from(node: AlterPublication) -> Stmt {
        Stmt::AlterPublication(node)
    }
}
impl From<AlterRole> for Stmt {
    #[inline]
    fn from(node: AlterRole) -> Stmt {
        Stmt::AlterRole(node)
    }
}
impl From<AlterRoutine> for Stmt {
    #[inline]
    fn from(node: AlterRoutine) -> Stmt {
        Stmt::AlterRoutine(node)
    }
}
impl From<AlterRule> for Stmt {
    #[inline]
    fn from(node: AlterRule) -> Stmt {
        Stmt::AlterRule(node)
    }
}
impl From<AlterSchema> for Stmt {
    #[inline]
    fn from(node: AlterSchema) -> Stmt {
        Stmt::AlterSchema(node)
    }
}
impl From<AlterSequence> for Stmt {
    #[inline]
    fn from(node: AlterSequence) -> Stmt {
        Stmt::AlterSequence(node)
    }
}
impl From<AlterServer> for Stmt {
    #[inline]
    fn from(node: AlterServer) -> Stmt {
        Stmt::AlterServer(node)
    }
}
impl From<AlterStatistics> for Stmt {
    #[inline]
    fn from(node: AlterStatistics) -> Stmt {
        Stmt::AlterStatistics(node)
    }
}
impl From<AlterSubscription> for Stmt {
    #[inline]
    fn from(node: AlterSubscription) -> Stmt {
        Stmt::AlterSubscription(node)
    }
}
impl From<AlterSystem> for Stmt {
    #[inline]
    fn from(node: AlterSystem) -> Stmt {
        Stmt::AlterSystem(node)
    }
}
impl From<AlterTable> for Stmt {
    #[inline]
    fn from(node: AlterTable) -> Stmt {
        Stmt::AlterTable(node)
    }
}
impl From<AlterTablespace> for Stmt {
    #[inline]
    fn from(node: AlterTablespace) -> Stmt {
        Stmt::AlterTablespace(node)
    }
}
impl From<AlterTextSearchConfiguration> for Stmt {
    #[inline]
    fn from(node: AlterTextSearchConfiguration) -> Stmt {
        Stmt::AlterTextSearchConfiguration(node)
    }
}
impl From<AlterTextSearchDictionary> for Stmt {
    #[inline]
    fn from(node: AlterTextSearchDictionary) -> Stmt {
        Stmt::AlterTextSearchDictionary(node)
    }
}
impl From<AlterTextSearchParser> for Stmt {
    #[inline]
    fn from(node: AlterTextSearchParser) -> Stmt {
        Stmt::AlterTextSearchParser(node)
    }
}
impl From<AlterTextSearchTemplate> for Stmt {
    #[inline]
    fn from(node: AlterTextSearchTemplate) -> Stmt {
        Stmt::AlterTextSearchTemplate(node)
    }
}
impl From<AlterTrigger> for Stmt {
    #[inline]
    fn from(node: AlterTrigger) -> Stmt {
        Stmt::AlterTrigger(node)
    }
}
impl From<AlterType> for Stmt {
    #[inline]
    fn from(node: AlterType) -> Stmt {
        Stmt::AlterType(node)
    }
}
impl From<AlterUser> for Stmt {
    #[inline]
    fn from(node: AlterUser) -> Stmt {
        Stmt::AlterUser(node)
    }
}
impl From<AlterUserMapping> for Stmt {
    #[inline]
    fn from(node: AlterUserMapping) -> Stmt {
        Stmt::AlterUserMapping(node)
    }
}
impl From<AlterView> for Stmt {
    #[inline]
    fn from(node: AlterView) -> Stmt {
        Stmt::AlterView(node)
    }
}
impl From<Analyze> for Stmt {
    #[inline]
    fn from(node: Analyze) -> Stmt {
        Stmt::Analyze(node)
    }
}
impl From<Begin> for Stmt {
    #[inline]
    fn from(node: Begin) -> Stmt {
        Stmt::Begin(node)
    }
}
impl From<Call> for Stmt {
    #[inline]
    fn from(node: Call) -> Stmt {
        Stmt::Call(node)
    }
}
impl From<Checkpoint> for Stmt {
    #[inline]
    fn from(node: Checkpoint) -> Stmt {
        Stmt::Checkpoint(node)
    }
}
impl From<Close> for Stmt {
    #[inline]
    fn from(node: Close) -> Stmt {
        Stmt::Close(node)
    }
}
impl From<Cluster> for Stmt {
    #[inline]
    fn from(node: Cluster) -> Stmt {
        Stmt::Cluster(node)
    }
}
impl From<CommentOn> for Stmt {
    #[inline]
    fn from(node: CommentOn) -> Stmt {
        Stmt::CommentOn(node)
    }
}
impl From<Commit> for Stmt {
    #[inline]
    fn from(node: Commit) -> Stmt {
        Stmt::Commit(node)
    }
}
impl From<Copy> for Stmt {
    #[inline]
    fn from(node: Copy) -> Stmt {
        Stmt::Copy(node)
    }
}
impl From<CreateAccessMethod> for Stmt {
    #[inline]
    fn from(node: CreateAccessMethod) -> Stmt {
        Stmt::CreateAccessMethod(node)
    }
}
impl From<CreateAggregate> for Stmt {
    #[inline]
    fn from(node: CreateAggregate) -> Stmt {
        Stmt::CreateAggregate(node)
    }
}
impl From<CreateCast> for Stmt {
    #[inline]
    fn from(node: CreateCast) -> Stmt {
        Stmt::CreateCast(node)
    }
}
impl From<CreateCollation> for Stmt {
    #[inline]
    fn from(node: CreateCollation) -> Stmt {
        Stmt::CreateCollation(node)
    }
}
impl From<CreateConversion> for Stmt {
    #[inline]
    fn from(node: CreateConversion) -> Stmt {
        Stmt::CreateConversion(node)
    }
}
impl From<CreateDatabase> for Stmt {
    #[inline]
    fn from(node: CreateDatabase) -> Stmt {
        Stmt::CreateDatabase(node)
    }
}
impl From<CreateDomain> for Stmt {
    #[inline]
    fn from(node: CreateDomain) -> Stmt {
        Stmt::CreateDomain(node)
    }
}
impl From<CreateEventTrigger> for Stmt {
    #[inline]
    fn from(node: CreateEventTrigger) -> Stmt {
        Stmt::CreateEventTrigger(node)
    }
}
impl From<CreateExtension> for Stmt {
    #[inline]
    fn from(node: CreateExtension) -> Stmt {
        Stmt::CreateExtension(node)
    }
}
impl From<CreateForeignDataWrapper> for Stmt {
    #[inline]
    fn from(node: CreateForeignDataWrapper) -> Stmt {
        Stmt::CreateForeignDataWrapper(node)
    }
}
impl From<CreateForeignTable> for Stmt {
    #[inline]
    fn from(node: CreateForeignTable) -> Stmt {
        Stmt::CreateForeignTable(node)
    }
}
impl From<CreateFunction> for Stmt {
    #[inline]
    fn from(node: CreateFunction) -> Stmt {
        Stmt::CreateFunction(node)
    }
}
impl From<CreateGroup> for Stmt {
    #[inline]
    fn from(node: CreateGroup) -> Stmt {
        Stmt::CreateGroup(node)
    }
}
impl From<CreateIndex> for Stmt {
    #[inline]
    fn from(node: CreateIndex) -> Stmt {
        Stmt::CreateIndex(node)
    }
}
impl From<CreateLanguage> for Stmt {
    #[inline]
    fn from(node: CreateLanguage) -> Stmt {
        Stmt::CreateLanguage(node)
    }
}
impl From<CreateMaterializedView> for Stmt {
    #[inline]
    fn from(node: CreateMaterializedView) -> Stmt {
        Stmt::CreateMaterializedView(node)
    }
}
impl From<CreateOperator> for Stmt {
    #[inline]
    fn from(node: CreateOperator) -> Stmt {
        Stmt::CreateOperator(node)
    }
}
impl From<CreateOperatorClass> for Stmt {
    #[inline]
    fn from(node: CreateOperatorClass) -> Stmt {
        Stmt::CreateOperatorClass(node)
    }
}
impl From<CreateOperatorFamily> for Stmt {
    #[inline]
    fn from(node: CreateOperatorFamily) -> Stmt {
        Stmt::CreateOperatorFamily(node)
    }
}
impl From<CreatePolicy> for Stmt {
    #[inline]
    fn from(node: CreatePolicy) -> Stmt {
        Stmt::CreatePolicy(node)
    }
}
impl From<CreateProcedure> for Stmt {
    #[inline]
    fn from(node: CreateProcedure) -> Stmt {
        Stmt::CreateProcedure(node)
    }
}
impl From<CreatePublication> for Stmt {
    #[inline]
    fn from(node: CreatePublication) -> Stmt {
        Stmt::CreatePublication(node)
    }
}
impl From<CreateRole> for Stmt {
    #[inline]
    fn from(node: CreateRole) -> Stmt {
        Stmt::CreateRole(node)
    }
}
impl From<CreateRule> for Stmt {
    #[inline]
    fn from(node: CreateRule) -> Stmt {
        Stmt::CreateRule(node)
    }
}
impl From<CreateSchema> for Stmt {
    #[inline]
    fn from(node: CreateSchema) -> Stmt {
        Stmt::CreateSchema(node)
    }
}
impl From<CreateSequence> for Stmt {
    #[inline]
    fn from(node: CreateSequence) -> Stmt {
        Stmt::CreateSequence(node)
    }
}
impl From<CreateServer> for Stmt {
    #[inline]
    fn from(node: CreateServer) -> Stmt {
        Stmt::CreateServer(node)
    }
}
impl From<CreateStatistics> for Stmt {
    #[inline]
    fn from(node: CreateStatistics) -> Stmt {
        Stmt::CreateStatistics(node)
    }
}
impl From<CreateSubscription> for Stmt {
    #[inline]
    fn from(node: CreateSubscription) -> Stmt {
        Stmt::CreateSubscription(node)
    }
}
impl From<CreateTable> for Stmt {
    #[inline]
    fn from(node: CreateTable) -> Stmt {
        Stmt::CreateTable(node)
    }
}
impl From<CreateTableAs> for Stmt {
    #[inline]
    fn from(node: CreateTableAs) -> Stmt {
        Stmt::CreateTableAs(node)
    }
}
impl From<CreateTablespace> for Stmt {
    #[inline]
    fn from(node: CreateTablespace) -> Stmt {
        Stmt::CreateTablespace(node)
    }
}
impl From<CreateTextSearchConfiguration> for Stmt {
    #[inline]
    fn from(node: CreateTextSearchConfiguration) -> Stmt {
        Stmt::CreateTextSearchConfiguration(node)
    }
}
impl From<CreateTextSearchDictionary> for Stmt {
    #[inline]
    fn from(node: CreateTextSearchDictionary) -> Stmt {
        Stmt::CreateTextSearchDictionary(node)
    }
}
impl From<CreateTextSearchParser> for Stmt {
    #[inline]
    fn from(node: CreateTextSearchParser) -> Stmt {
        Stmt::CreateTextSearchParser(node)
    }
}
impl From<CreateTextSearchTemplate> for Stmt {
    #[inline]
    fn from(node: CreateTextSearchTemplate) -> Stmt {
        Stmt::CreateTextSearchTemplate(node)
    }
}
impl From<CreateTransform> for Stmt {
    #[inline]
    fn from(node: CreateTransform) -> Stmt {
        Stmt::CreateTransform(node)
    }
}
impl From<CreateTrigger> for Stmt {
    #[inline]
    fn from(node: CreateTrigger) -> Stmt {
        Stmt::CreateTrigger(node)
    }
}
impl From<CreateType> for Stmt {
    #[inline]
    fn from(node: CreateType) -> Stmt {
        Stmt::CreateType(node)
    }
}
impl From<CreateUser> for Stmt {
    #[inline]
    fn from(node: CreateUser) -> Stmt {
        Stmt::CreateUser(node)
    }
}
impl From<CreateUserMapping> for Stmt {
    #[inline]
    fn from(node: CreateUserMapping) -> Stmt {
        Stmt::CreateUserMapping(node)
    }
}
impl From<CreateView> for Stmt {
    #[inline]
    fn from(node: CreateView) -> Stmt {
        Stmt::CreateView(node)
    }
}
impl From<Deallocate> for Stmt {
    #[inline]
    fn from(node: Deallocate) -> Stmt {
        Stmt::Deallocate(node)
    }
}
impl From<Declare> for Stmt {
    #[inline]
    fn from(node: Declare) -> Stmt {
        Stmt::Declare(node)
    }
}
impl From<Delete> for Stmt {
    #[inline]
    fn from(node: Delete) -> Stmt {
        Stmt::Delete(node)
    }
}
impl From<Discard> for Stmt {
    #[inline]
    fn from(node: Discard) -> Stmt {
        Stmt::Discard(node)
    }
}
impl From<Do> for Stmt {
    #[inline]
    fn from(node: Do) -> Stmt {
        Stmt::Do(node)
    }
}
impl From<DropAccessMethod> for Stmt {
    #[inline]
    fn from(node: DropAccessMethod) -> Stmt {
        Stmt::DropAccessMethod(node)
    }
}
impl From<DropAggregate> for Stmt {
    #[inline]
    fn from(node: DropAggregate) -> Stmt {
        Stmt::DropAggregate(node)
    }
}
impl From<DropCast> for Stmt {
    #[inline]
    fn from(node: DropCast) -> Stmt {
        Stmt::DropCast(node)
    }
}
impl From<DropCollation> for Stmt {
    #[inline]
    fn from(node: DropCollation) -> Stmt {
        Stmt::DropCollation(node)
    }
}
impl From<DropConversion> for Stmt {
    #[inline]
    fn from(node: DropConversion) -> Stmt {
        Stmt::DropConversion(node)
    }
}
impl From<DropDatabase> for Stmt {
    #[inline]
    fn from(node: DropDatabase) -> Stmt {
        Stmt::DropDatabase(node)
    }
}
impl From<DropDomain> for Stmt {
    #[inline]
    fn from(node: DropDomain) -> Stmt {
        Stmt::DropDomain(node)
    }
}
impl From<DropEventTrigger> for Stmt {
    #[inline]
    fn from(node: DropEventTrigger) -> Stmt {
        Stmt::DropEventTrigger(node)
    }
}
impl From<DropExtension> for Stmt {
    #[inline]
    fn from(node: DropExtension) -> Stmt {
        Stmt::DropExtension(node)
    }
}
impl From<DropForeignDataWrapper> for Stmt {
    #[inline]
    fn from(node: DropForeignDataWrapper) -> Stmt {
        Stmt::DropForeignDataWrapper(node)
    }
}
impl From<DropForeignTable> for Stmt {
    #[inline]
    fn from(node: DropForeignTable) -> Stmt {
        Stmt::DropForeignTable(node)
    }
}
impl From<DropFunction> for Stmt {
    #[inline]
    fn from(node: DropFunction) -> Stmt {
        Stmt::DropFunction(node)
    }
}
impl From<DropGroup> for Stmt {
    #[inline]
    fn from(node: DropGroup) -> Stmt {
        Stmt::DropGroup(node)
    }
}
impl From<DropIndex> for Stmt {
    #[inline]
    fn from(node: DropIndex) -> Stmt {
        Stmt::DropIndex(node)
    }
}
impl From<DropLanguage> for Stmt {
    #[inline]
    fn from(node: DropLanguage) -> Stmt {
        Stmt::DropLanguage(node)
    }
}
impl From<DropMaterializedView> for Stmt {
    #[inline]
    fn from(node: DropMaterializedView) -> Stmt {
        Stmt::DropMaterializedView(node)
    }
}
impl From<DropOperator> for Stmt {
    #[inline]
    fn from(node: DropOperator) -> Stmt {
        Stmt::DropOperator(node)
    }
}
impl From<DropOperatorClass> for Stmt {
    #[inline]
    fn from(node: DropOperatorClass) -> Stmt {
        Stmt::DropOperatorClass(node)
    }
}
impl From<DropOperatorFamily> for Stmt {
    #[inline]
    fn from(node: DropOperatorFamily) -> Stmt {
        Stmt::DropOperatorFamily(node)
    }
}
impl From<DropOwned> for Stmt {
    #[inline]
    fn from(node: DropOwned) -> Stmt {
        Stmt::DropOwned(node)
    }
}
impl From<DropPolicy> for Stmt {
    #[inline]
    fn from(node: DropPolicy) -> Stmt {
        Stmt::DropPolicy(node)
    }
}
impl From<DropProcedure> for Stmt {
    #[inline]
    fn from(node: DropProcedure) -> Stmt {
        Stmt::DropProcedure(node)
    }
}
impl From<DropPublication> for Stmt {
    #[inline]
    fn from(node: DropPublication) -> Stmt {
        Stmt::DropPublication(node)
    }
}
impl From<DropRole> for Stmt {
    #[inline]
    fn from(node: DropRole) -> Stmt {
        Stmt::DropRole(node)
    }
}
impl From<DropRoutine> for Stmt {
    #[inline]
    fn from(node: DropRoutine) -> Stmt {
        Stmt::DropRoutine(node)
    }
}
impl From<DropRule> for Stmt {
    #[inline]
    fn from(node: DropRule) -> Stmt {
        Stmt::DropRule(node)
    }
}
impl From<DropSchema> for Stmt {
    #[inline]
    fn from(node: DropSchema) -> Stmt {
        Stmt::DropSchema(node)
    }
}
impl From<DropSequence> for Stmt {
    #[inline]
    fn from(node: DropSequence) -> Stmt {
        Stmt::DropSequence(node)
    }
}
impl From<DropServer> for Stmt {
    #[inline]
    fn from(node: DropServer) -> Stmt {
        Stmt::DropServer(node)
    }
}
impl From<DropStatistics> for Stmt {
    #[inline]
    fn from(node: DropStatistics) -> Stmt {
        Stmt::DropStatistics(node)
    }
}
impl From<DropSubscription> for Stmt {
    #[inline]
    fn from(node: DropSubscription) -> Stmt {
        Stmt::DropSubscription(node)
    }
}
impl From<DropTable> for Stmt {
    #[inline]
    fn from(node: DropTable) -> Stmt {
        Stmt::DropTable(node)
    }
}
impl From<DropTablespace> for Stmt {
    #[inline]
    fn from(node: DropTablespace) -> Stmt {
        Stmt::DropTablespace(node)
    }
}
impl From<DropTextSearchConfig> for Stmt {
    #[inline]
    fn from(node: DropTextSearchConfig) -> Stmt {
        Stmt::DropTextSearchConfig(node)
    }
}
impl From<DropTextSearchDict> for Stmt {
    #[inline]
    fn from(node: DropTextSearchDict) -> Stmt {
        Stmt::DropTextSearchDict(node)
    }
}
impl From<DropTextSearchParser> for Stmt {
    #[inline]
    fn from(node: DropTextSearchParser) -> Stmt {
        Stmt::DropTextSearchParser(node)
    }
}
impl From<DropTextSearchTemplate> for Stmt {
    #[inline]
    fn from(node: DropTextSearchTemplate) -> Stmt {
        Stmt::DropTextSearchTemplate(node)
    }
}
impl From<DropTransform> for Stmt {
    #[inline]
    fn from(node: DropTransform) -> Stmt {
        Stmt::DropTransform(node)
    }
}
impl From<DropTrigger> for Stmt {
    #[inline]
    fn from(node: DropTrigger) -> Stmt {
        Stmt::DropTrigger(node)
    }
}
impl From<DropType> for Stmt {
    #[inline]
    fn from(node: DropType) -> Stmt {
        Stmt::DropType(node)
    }
}
impl From<DropUser> for Stmt {
    #[inline]
    fn from(node: DropUser) -> Stmt {
        Stmt::DropUser(node)
    }
}
impl From<DropUserMapping> for Stmt {
    #[inline]
    fn from(node: DropUserMapping) -> Stmt {
        Stmt::DropUserMapping(node)
    }
}
impl From<DropView> for Stmt {
    #[inline]
    fn from(node: DropView) -> Stmt {
        Stmt::DropView(node)
    }
}
impl From<Execute> for Stmt {
    #[inline]
    fn from(node: Execute) -> Stmt {
        Stmt::Execute(node)
    }
}
impl From<Explain> for Stmt {
    #[inline]
    fn from(node: Explain) -> Stmt {
        Stmt::Explain(node)
    }
}
impl From<Fetch> for Stmt {
    #[inline]
    fn from(node: Fetch) -> Stmt {
        Stmt::Fetch(node)
    }
}
impl From<Grant> for Stmt {
    #[inline]
    fn from(node: Grant) -> Stmt {
        Stmt::Grant(node)
    }
}
impl From<ImportForeignSchema> for Stmt {
    #[inline]
    fn from(node: ImportForeignSchema) -> Stmt {
        Stmt::ImportForeignSchema(node)
    }
}
impl From<Insert> for Stmt {
    #[inline]
    fn from(node: Insert) -> Stmt {
        Stmt::Insert(node)
    }
}
impl From<Listen> for Stmt {
    #[inline]
    fn from(node: Listen) -> Stmt {
        Stmt::Listen(node)
    }
}
impl From<Load> for Stmt {
    #[inline]
    fn from(node: Load) -> Stmt {
        Stmt::Load(node)
    }
}
impl From<Lock> for Stmt {
    #[inline]
    fn from(node: Lock) -> Stmt {
        Stmt::Lock(node)
    }
}
impl From<Merge> for Stmt {
    #[inline]
    fn from(node: Merge) -> Stmt {
        Stmt::Merge(node)
    }
}
impl From<Move> for Stmt {
    #[inline]
    fn from(node: Move) -> Stmt {
        Stmt::Move(node)
    }
}
impl From<Notify> for Stmt {
    #[inline]
    fn from(node: Notify) -> Stmt {
        Stmt::Notify(node)
    }
}
impl From<Prepare> for Stmt {
    #[inline]
    fn from(node: Prepare) -> Stmt {
        Stmt::Prepare(node)
    }
}
impl From<PrepareTransaction> for Stmt {
    #[inline]
    fn from(node: PrepareTransaction) -> Stmt {
        Stmt::PrepareTransaction(node)
    }
}
impl From<Reassign> for Stmt {
    #[inline]
    fn from(node: Reassign) -> Stmt {
        Stmt::Reassign(node)
    }
}
impl From<Refresh> for Stmt {
    #[inline]
    fn from(node: Refresh) -> Stmt {
        Stmt::Refresh(node)
    }
}
impl From<Reindex> for Stmt {
    #[inline]
    fn from(node: Reindex) -> Stmt {
        Stmt::Reindex(node)
    }
}
impl From<ReleaseSavepoint> for Stmt {
    #[inline]
    fn from(node: ReleaseSavepoint) -> Stmt {
        Stmt::ReleaseSavepoint(node)
    }
}
impl From<Reset> for Stmt {
    #[inline]
    fn from(node: Reset) -> Stmt {
        Stmt::Reset(node)
    }
}
impl From<Revoke> for Stmt {
    #[inline]
    fn from(node: Revoke) -> Stmt {
        Stmt::Revoke(node)
    }
}
impl From<Rollback> for Stmt {
    #[inline]
    fn from(node: Rollback) -> Stmt {
        Stmt::Rollback(node)
    }
}
impl From<Savepoint> for Stmt {
    #[inline]
    fn from(node: Savepoint) -> Stmt {
        Stmt::Savepoint(node)
    }
}
impl From<SecurityLabel> for Stmt {
    #[inline]
    fn from(node: SecurityLabel) -> Stmt {
        Stmt::SecurityLabel(node)
    }
}
impl From<Select> for Stmt {
    #[inline]
    fn from(node: Select) -> Stmt {
        Stmt::Select(node)
    }
}
impl From<SelectInto> for Stmt {
    #[inline]
    fn from(node: SelectInto) -> Stmt {
        Stmt::SelectInto(node)
    }
}
impl From<Set> for Stmt {
    #[inline]
    fn from(node: Set) -> Stmt {
        Stmt::Set(node)
    }
}
impl From<SetConstraints> for Stmt {
    #[inline]
    fn from(node: SetConstraints) -> Stmt {
        Stmt::SetConstraints(node)
    }
}
impl From<SetRole> for Stmt {
    #[inline]
    fn from(node: SetRole) -> Stmt {
        Stmt::SetRole(node)
    }
}
impl From<SetSessionAuth> for Stmt {
    #[inline]
    fn from(node: SetSessionAuth) -> Stmt {
        Stmt::SetSessionAuth(node)
    }
}
impl From<SetTransaction> for Stmt {
    #[inline]
    fn from(node: SetTransaction) -> Stmt {
        Stmt::SetTransaction(node)
    }
}
impl From<Show> for Stmt {
    #[inline]
    fn from(node: Show) -> Stmt {
        Stmt::Show(node)
    }
}
impl From<Truncate> for Stmt {
    #[inline]
    fn from(node: Truncate) -> Stmt {
        Stmt::Truncate(node)
    }
}
impl From<Unlisten> for Stmt {
    #[inline]
    fn from(node: Unlisten) -> Stmt {
        Stmt::Unlisten(node)
    }
}
impl From<Update> for Stmt {
    #[inline]
    fn from(node: Update) -> Stmt {
        Stmt::Update(node)
    }
}
impl From<Vacuum> for Stmt {
    #[inline]
    fn from(node: Vacuum) -> Stmt {
        Stmt::Vacuum(node)
    }
}
impl From<Values> for Stmt {
    #[inline]
    fn from(node: Values) -> Stmt {
        Stmt::Values(node)
    }
}
impl AstNode for TableArg {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, SyntaxKind::COLUMN | SyntaxKind::LIKE_CLAUSE)
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            SyntaxKind::COLUMN => TableArg::Column(Column { syntax }),
            SyntaxKind::LIKE_CLAUSE => TableArg::LikeClause(LikeClause { syntax }),
            _ => {
                if let Some(result) = TableConstraint::cast(syntax) {
                    return Some(TableArg::TableConstraint(result));
                }
                return None;
            }
        };
        Some(res)
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        match self {
            TableArg::Column(it) => &it.syntax,
            TableArg::LikeClause(it) => &it.syntax,
            TableArg::TableConstraint(it) => it.syntax(),
        }
    }
}
impl From<Column> for TableArg {
    #[inline]
    fn from(node: Column) -> TableArg {
        TableArg::Column(node)
    }
}
impl From<LikeClause> for TableArg {
    #[inline]
    fn from(node: LikeClause) -> TableArg {
        TableArg::LikeClause(node)
    }
}
impl AstNode for TableConstraint {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            SyntaxKind::CHECK_CONSTRAINT
                | SyntaxKind::EXCLUDE_CONSTRAINT
                | SyntaxKind::FOREIGN_KEY_CONSTRAINT
                | SyntaxKind::PRIMARY_KEY_CONSTRAINT
                | SyntaxKind::UNIQUE_CONSTRAINT
        )
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            SyntaxKind::CHECK_CONSTRAINT => {
                TableConstraint::CheckConstraint(CheckConstraint { syntax })
            }
            SyntaxKind::EXCLUDE_CONSTRAINT => {
                TableConstraint::ExcludeConstraint(ExcludeConstraint { syntax })
            }
            SyntaxKind::FOREIGN_KEY_CONSTRAINT => {
                TableConstraint::ForeignKeyConstraint(ForeignKeyConstraint { syntax })
            }
            SyntaxKind::PRIMARY_KEY_CONSTRAINT => {
                TableConstraint::PrimaryKeyConstraint(PrimaryKeyConstraint { syntax })
            }
            SyntaxKind::UNIQUE_CONSTRAINT => {
                TableConstraint::UniqueConstraint(UniqueConstraint { syntax })
            }
            _ => {
                return None;
            }
        };
        Some(res)
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        match self {
            TableConstraint::CheckConstraint(it) => &it.syntax,
            TableConstraint::ExcludeConstraint(it) => &it.syntax,
            TableConstraint::ForeignKeyConstraint(it) => &it.syntax,
            TableConstraint::PrimaryKeyConstraint(it) => &it.syntax,
            TableConstraint::UniqueConstraint(it) => &it.syntax,
        }
    }
}
impl From<CheckConstraint> for TableConstraint {
    #[inline]
    fn from(node: CheckConstraint) -> TableConstraint {
        TableConstraint::CheckConstraint(node)
    }
}
impl From<ExcludeConstraint> for TableConstraint {
    #[inline]
    fn from(node: ExcludeConstraint) -> TableConstraint {
        TableConstraint::ExcludeConstraint(node)
    }
}
impl From<ForeignKeyConstraint> for TableConstraint {
    #[inline]
    fn from(node: ForeignKeyConstraint) -> TableConstraint {
        TableConstraint::ForeignKeyConstraint(node)
    }
}
impl From<PrimaryKeyConstraint> for TableConstraint {
    #[inline]
    fn from(node: PrimaryKeyConstraint) -> TableConstraint {
        TableConstraint::PrimaryKeyConstraint(node)
    }
}
impl From<UniqueConstraint> for TableConstraint {
    #[inline]
    fn from(node: UniqueConstraint) -> TableConstraint {
        TableConstraint::UniqueConstraint(node)
    }
}
impl AstNode for TransactionMode {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            SyntaxKind::DEFERRABLE
                | SyntaxKind::NOT_DEFERRABLE
                | SyntaxKind::READ_ONLY
                | SyntaxKind::READ_WRITE
                | SyntaxKind::TRANSACTION_MODE_ISOLATION_LEVEL
        )
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            SyntaxKind::DEFERRABLE => TransactionMode::Deferrable(Deferrable { syntax }),
            SyntaxKind::NOT_DEFERRABLE => TransactionMode::NotDeferrable(NotDeferrable { syntax }),
            SyntaxKind::READ_ONLY => TransactionMode::ReadOnly(ReadOnly { syntax }),
            SyntaxKind::READ_WRITE => TransactionMode::ReadWrite(ReadWrite { syntax }),
            SyntaxKind::TRANSACTION_MODE_ISOLATION_LEVEL => {
                TransactionMode::TransactionModeIsolationLevel(TransactionModeIsolationLevel {
                    syntax,
                })
            }
            _ => {
                return None;
            }
        };
        Some(res)
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        match self {
            TransactionMode::Deferrable(it) => &it.syntax,
            TransactionMode::NotDeferrable(it) => &it.syntax,
            TransactionMode::ReadOnly(it) => &it.syntax,
            TransactionMode::ReadWrite(it) => &it.syntax,
            TransactionMode::TransactionModeIsolationLevel(it) => &it.syntax,
        }
    }
}
impl From<Deferrable> for TransactionMode {
    #[inline]
    fn from(node: Deferrable) -> TransactionMode {
        TransactionMode::Deferrable(node)
    }
}
impl From<NotDeferrable> for TransactionMode {
    #[inline]
    fn from(node: NotDeferrable) -> TransactionMode {
        TransactionMode::NotDeferrable(node)
    }
}
impl From<ReadOnly> for TransactionMode {
    #[inline]
    fn from(node: ReadOnly) -> TransactionMode {
        TransactionMode::ReadOnly(node)
    }
}
impl From<ReadWrite> for TransactionMode {
    #[inline]
    fn from(node: ReadWrite) -> TransactionMode {
        TransactionMode::ReadWrite(node)
    }
}
impl From<TransactionModeIsolationLevel> for TransactionMode {
    #[inline]
    fn from(node: TransactionModeIsolationLevel) -> TransactionMode {
        TransactionMode::TransactionModeIsolationLevel(node)
    }
}
impl AstNode for Type {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            SyntaxKind::ARRAY_TYPE
                | SyntaxKind::BIT_TYPE
                | SyntaxKind::CHAR_TYPE
                | SyntaxKind::DOUBLE_TYPE
                | SyntaxKind::INTERVAL_TYPE
                | SyntaxKind::PATH_TYPE
                | SyntaxKind::PERCENT_TYPE
                | SyntaxKind::TIME_TYPE
        )
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            SyntaxKind::ARRAY_TYPE => Type::ArrayType(ArrayType { syntax }),
            SyntaxKind::BIT_TYPE => Type::BitType(BitType { syntax }),
            SyntaxKind::CHAR_TYPE => Type::CharType(CharType { syntax }),
            SyntaxKind::DOUBLE_TYPE => Type::DoubleType(DoubleType { syntax }),
            SyntaxKind::INTERVAL_TYPE => Type::IntervalType(IntervalType { syntax }),
            SyntaxKind::PATH_TYPE => Type::PathType(PathType { syntax }),
            SyntaxKind::PERCENT_TYPE => Type::PercentType(PercentType { syntax }),
            SyntaxKind::TIME_TYPE => Type::TimeType(TimeType { syntax }),
            _ => {
                return None;
            }
        };
        Some(res)
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Type::ArrayType(it) => &it.syntax,
            Type::BitType(it) => &it.syntax,
            Type::CharType(it) => &it.syntax,
            Type::DoubleType(it) => &it.syntax,
            Type::IntervalType(it) => &it.syntax,
            Type::PathType(it) => &it.syntax,
            Type::PercentType(it) => &it.syntax,
            Type::TimeType(it) => &it.syntax,
        }
    }
}
impl From<ArrayType> for Type {
    #[inline]
    fn from(node: ArrayType) -> Type {
        Type::ArrayType(node)
    }
}
impl From<BitType> for Type {
    #[inline]
    fn from(node: BitType) -> Type {
        Type::BitType(node)
    }
}
impl From<CharType> for Type {
    #[inline]
    fn from(node: CharType) -> Type {
        Type::CharType(node)
    }
}
impl From<DoubleType> for Type {
    #[inline]
    fn from(node: DoubleType) -> Type {
        Type::DoubleType(node)
    }
}
impl From<IntervalType> for Type {
    #[inline]
    fn from(node: IntervalType) -> Type {
        Type::IntervalType(node)
    }
}
impl From<PathType> for Type {
    #[inline]
    fn from(node: PathType) -> Type {
        Type::PathType(node)
    }
}
impl From<PercentType> for Type {
    #[inline]
    fn from(node: PercentType) -> Type {
        Type::PercentType(node)
    }
}
impl From<TimeType> for Type {
    #[inline]
    fn from(node: TimeType) -> Type {
        Type::TimeType(node)
    }
}
