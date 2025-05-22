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
    pub fn add_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ADD_KW)
    }
    #[inline]
    pub fn column_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::COLUMN_KW)
    }
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
pub struct AddConstraint {
    pub(crate) syntax: SyntaxNode,
}
impl AddConstraint {
    #[inline]
    pub fn constraint(&self) -> Option<Constraint> {
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
    pub fn alter_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ALTER_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AlterAggregateStmt {
    pub(crate) syntax: SyntaxNode,
}
impl AlterAggregateStmt {
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
pub struct AlterCollationStmt {
    pub(crate) syntax: SyntaxNode,
}
impl AlterCollationStmt {
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
    pub fn alter_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::ALTER_KW)
    }
    #[inline]
    pub fn constraint_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CONSTRAINT_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AlterConversionStmt {
    pub(crate) syntax: SyntaxNode,
}
impl AlterConversionStmt {
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
pub struct AlterDatabaseStmt {
    pub(crate) syntax: SyntaxNode,
}
impl AlterDatabaseStmt {
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
pub struct AlterDefaultPrivilegesStmt {
    pub(crate) syntax: SyntaxNode,
}
impl AlterDefaultPrivilegesStmt {
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
    pub fn alter_domain_action(&self) -> Option<AlterDomainAction> {
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
pub struct AlterDomainStmt {
    pub(crate) syntax: SyntaxNode,
}
impl AlterDomainStmt {
    #[inline]
    pub fn alter_domain_action(&self) -> Option<AlterDomainAction> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
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
pub struct AlterEventTriggerStmt {
    pub(crate) syntax: SyntaxNode,
}
impl AlterEventTriggerStmt {
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
pub struct AlterExtensionStmt {
    pub(crate) syntax: SyntaxNode,
}
impl AlterExtensionStmt {
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
pub struct AlterForeignDataWrapperStmt {
    pub(crate) syntax: SyntaxNode,
}
impl AlterForeignDataWrapperStmt {
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
pub struct AlterForeignTableStmt {
    pub(crate) syntax: SyntaxNode,
}
impl AlterForeignTableStmt {
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
pub struct AlterFunctionStmt {
    pub(crate) syntax: SyntaxNode,
}
impl AlterFunctionStmt {
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
pub struct AlterGroupStmt {
    pub(crate) syntax: SyntaxNode,
}
impl AlterGroupStmt {
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
pub struct AlterIndexStmt {
    pub(crate) syntax: SyntaxNode,
}
impl AlterIndexStmt {
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
pub struct AlterLanguageStmt {
    pub(crate) syntax: SyntaxNode,
}
impl AlterLanguageStmt {
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
pub struct AlterLargeObjectStmt {
    pub(crate) syntax: SyntaxNode,
}
impl AlterLargeObjectStmt {
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
pub struct AlterMaterializedViewStmt {
    pub(crate) syntax: SyntaxNode,
}
impl AlterMaterializedViewStmt {
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
pub struct AlterOperatorClassStmt {
    pub(crate) syntax: SyntaxNode,
}
impl AlterOperatorClassStmt {
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
pub struct AlterOperatorFamilyStmt {
    pub(crate) syntax: SyntaxNode,
}
impl AlterOperatorFamilyStmt {
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
pub struct AlterOperatorStmt {
    pub(crate) syntax: SyntaxNode,
}
impl AlterOperatorStmt {
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
pub struct AlterPolicyStmt {
    pub(crate) syntax: SyntaxNode,
}
impl AlterPolicyStmt {
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
pub struct AlterProcedureStmt {
    pub(crate) syntax: SyntaxNode,
}
impl AlterProcedureStmt {
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
pub struct AlterPublicationStmt {
    pub(crate) syntax: SyntaxNode,
}
impl AlterPublicationStmt {
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
pub struct AlterRoleStmt {
    pub(crate) syntax: SyntaxNode,
}
impl AlterRoleStmt {
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
pub struct AlterRoutineStmt {
    pub(crate) syntax: SyntaxNode,
}
impl AlterRoutineStmt {
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
pub struct AlterRuleStmt {
    pub(crate) syntax: SyntaxNode,
}
impl AlterRuleStmt {
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
pub struct AlterSchemaStmt {
    pub(crate) syntax: SyntaxNode,
}
impl AlterSchemaStmt {
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
pub struct AlterSequenceStmt {
    pub(crate) syntax: SyntaxNode,
}
impl AlterSequenceStmt {
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
pub struct AlterServerStmt {
    pub(crate) syntax: SyntaxNode,
}
impl AlterServerStmt {
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
pub struct AlterStatisticsStmt {
    pub(crate) syntax: SyntaxNode,
}
impl AlterStatisticsStmt {
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
pub struct AlterSubscriptionStmt {
    pub(crate) syntax: SyntaxNode,
}
impl AlterSubscriptionStmt {
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
pub struct AlterSystemStmt {
    pub(crate) syntax: SyntaxNode,
}
impl AlterSystemStmt {
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
    pub fn path(&self) -> Option<Path> {
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
pub struct AlterTablespaceStmt {
    pub(crate) syntax: SyntaxNode,
}
impl AlterTablespaceStmt {
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
pub struct AlterTextSearchConfigurationStmt {
    pub(crate) syntax: SyntaxNode,
}
impl AlterTextSearchConfigurationStmt {
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
pub struct AlterTextSearchDictionaryStmt {
    pub(crate) syntax: SyntaxNode,
}
impl AlterTextSearchDictionaryStmt {
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
pub struct AlterTextSearchParserStmt {
    pub(crate) syntax: SyntaxNode,
}
impl AlterTextSearchParserStmt {
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
pub struct AlterTextSearchTemplateStmt {
    pub(crate) syntax: SyntaxNode,
}
impl AlterTextSearchTemplateStmt {
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
pub struct AlterTriggerStmt {
    pub(crate) syntax: SyntaxNode,
}
impl AlterTriggerStmt {
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
pub struct AlterTypeStmt {
    pub(crate) syntax: SyntaxNode,
}
impl AlterTypeStmt {
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
pub struct AlterUserMappingStmt {
    pub(crate) syntax: SyntaxNode,
}
impl AlterUserMappingStmt {
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
pub struct AlterUserStmt {
    pub(crate) syntax: SyntaxNode,
}
impl AlterUserStmt {
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
pub struct AlterViewStmt {
    pub(crate) syntax: SyntaxNode,
}
impl AlterViewStmt {
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
pub struct AnalyzeStmt {
    pub(crate) syntax: SyntaxNode,
}
impl AnalyzeStmt {
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
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn ty(&self) -> Option<Type> {
        support::child(&self.syntax)
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
    pub fn expr(&self) -> Option<Expr> {
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
    pub fn expr(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn op(&self) -> Option<Op> {
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
pub struct CallExpr {
    pub(crate) syntax: SyntaxNode,
}
impl CallExpr {
    #[inline]
    pub fn expr(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn param_list(&self) -> Option<ParamList> {
        support::child(&self.syntax)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CallStmt {
    pub(crate) syntax: SyntaxNode,
}
impl CallStmt {
    #[inline]
    pub fn call_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CALL_KW)
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
pub struct CheckpointStmt {
    pub(crate) syntax: SyntaxNode,
}
impl CheckpointStmt {
    #[inline]
    pub fn checkpoint_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CHECKPOINT_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CloseStmt {
    pub(crate) syntax: SyntaxNode,
}
impl CloseStmt {
    #[inline]
    pub fn close_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CLOSE_KW)
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
pub struct ClusterStmt {
    pub(crate) syntax: SyntaxNode,
}
impl ClusterStmt {
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
pub struct Collate {
    pub(crate) syntax: SyntaxNode,
}
impl Collate {
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
pub struct CommentStmt {
    pub(crate) syntax: SyntaxNode,
}
impl CommentStmt {
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
pub struct ConstraintOptionList {
    pub(crate) syntax: SyntaxNode,
}
impl ConstraintOptionList {
    #[inline]
    pub fn deferrable_constraint_option(&self) -> Option<DeferrableConstraintOption> {
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
    pub fn not_deferrable_constraint_option(&self) -> Option<NotDeferrableConstraintOption> {
        support::child(&self.syntax)
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
pub struct CopyStmt {
    pub(crate) syntax: SyntaxNode,
}
impl CopyStmt {
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
pub struct CreateAccessMethodStmt {
    pub(crate) syntax: SyntaxNode,
}
impl CreateAccessMethodStmt {
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
    pub fn param_list(&self) -> Option<ParamList> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn path(&self) -> Option<Path> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn create_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CREATE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateAggregateStmt {
    pub(crate) syntax: SyntaxNode,
}
impl CreateAggregateStmt {
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
pub struct CreateCastStmt {
    pub(crate) syntax: SyntaxNode,
}
impl CreateCastStmt {
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
pub struct CreateCollationStmt {
    pub(crate) syntax: SyntaxNode,
}
impl CreateCollationStmt {
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
pub struct CreateConversionStmt {
    pub(crate) syntax: SyntaxNode,
}
impl CreateConversionStmt {
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
pub struct CreateDatabaseStmt {
    pub(crate) syntax: SyntaxNode,
}
impl CreateDatabaseStmt {
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
    pub fn name(&self) -> Option<Name> {
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
pub struct CreateDomainStmt {
    pub(crate) syntax: SyntaxNode,
}
impl CreateDomainStmt {
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
pub struct CreateEventTriggerStmt {
    pub(crate) syntax: SyntaxNode,
}
impl CreateEventTriggerStmt {
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
pub struct CreateExtensionStmt {
    pub(crate) syntax: SyntaxNode,
}
impl CreateExtensionStmt {
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
pub struct CreateForeignDataWrapperStmt {
    pub(crate) syntax: SyntaxNode,
}
impl CreateForeignDataWrapperStmt {
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
pub struct CreateForeignTableStmt {
    pub(crate) syntax: SyntaxNode,
}
impl CreateForeignTableStmt {
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
pub struct CreateFunc {
    pub(crate) syntax: SyntaxNode,
}
impl CreateFunc {
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
pub struct CreateFunctionStmt {
    pub(crate) syntax: SyntaxNode,
}
impl CreateFunctionStmt {
    #[inline]
    pub fn create_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CREATE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateGroupStmt {
    pub(crate) syntax: SyntaxNode,
}
impl CreateGroupStmt {
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
pub struct CreateLanguageStmt {
    pub(crate) syntax: SyntaxNode,
}
impl CreateLanguageStmt {
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
pub struct CreateMaterializedViewStmt {
    pub(crate) syntax: SyntaxNode,
}
impl CreateMaterializedViewStmt {
    #[inline]
    pub fn create_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CREATE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateOperatorClassStmt {
    pub(crate) syntax: SyntaxNode,
}
impl CreateOperatorClassStmt {
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
pub struct CreateOperatorFamilyStmt {
    pub(crate) syntax: SyntaxNode,
}
impl CreateOperatorFamilyStmt {
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
pub struct CreateOperatorStmt {
    pub(crate) syntax: SyntaxNode,
}
impl CreateOperatorStmt {
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
pub struct CreatePolicyStmt {
    pub(crate) syntax: SyntaxNode,
}
impl CreatePolicyStmt {
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
pub struct CreateProcedureStmt {
    pub(crate) syntax: SyntaxNode,
}
impl CreateProcedureStmt {
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
pub struct CreatePublicationStmt {
    pub(crate) syntax: SyntaxNode,
}
impl CreatePublicationStmt {
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
pub struct CreateRoleStmt {
    pub(crate) syntax: SyntaxNode,
}
impl CreateRoleStmt {
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
pub struct CreateRuleStmt {
    pub(crate) syntax: SyntaxNode,
}
impl CreateRuleStmt {
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
pub struct CreateSchemaStmt {
    pub(crate) syntax: SyntaxNode,
}
impl CreateSchemaStmt {
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
pub struct CreateSequenceStmt {
    pub(crate) syntax: SyntaxNode,
}
impl CreateSequenceStmt {
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
pub struct CreateServerStmt {
    pub(crate) syntax: SyntaxNode,
}
impl CreateServerStmt {
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
pub struct CreateStatisticsStmt {
    pub(crate) syntax: SyntaxNode,
}
impl CreateStatisticsStmt {
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
pub struct CreateSubscriptionStmt {
    pub(crate) syntax: SyntaxNode,
}
impl CreateSubscriptionStmt {
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
pub struct CreateTableAsStmt {
    pub(crate) syntax: SyntaxNode,
}
impl CreateTableAsStmt {
    #[inline]
    pub fn create_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CREATE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateTablespaceStmt {
    pub(crate) syntax: SyntaxNode,
}
impl CreateTablespaceStmt {
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
pub struct CreateTextSearchConfigurationStmt {
    pub(crate) syntax: SyntaxNode,
}
impl CreateTextSearchConfigurationStmt {
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
pub struct CreateTextSearchDictionaryStmt {
    pub(crate) syntax: SyntaxNode,
}
impl CreateTextSearchDictionaryStmt {
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
pub struct CreateTextSearchParserStmt {
    pub(crate) syntax: SyntaxNode,
}
impl CreateTextSearchParserStmt {
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
pub struct CreateTextSearchTemplateStmt {
    pub(crate) syntax: SyntaxNode,
}
impl CreateTextSearchTemplateStmt {
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
pub struct CreateTransformStmt {
    pub(crate) syntax: SyntaxNode,
}
impl CreateTransformStmt {
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
pub struct CreateTriggerStmt {
    pub(crate) syntax: SyntaxNode,
}
impl CreateTriggerStmt {
    #[inline]
    pub fn create_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CREATE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateTypeStmt {
    pub(crate) syntax: SyntaxNode,
}
impl CreateTypeStmt {
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
pub struct CreateUserMappingStmt {
    pub(crate) syntax: SyntaxNode,
}
impl CreateUserMappingStmt {
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
pub struct CreateUserStmt {
    pub(crate) syntax: SyntaxNode,
}
impl CreateUserStmt {
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
pub struct CreateViewStmt {
    pub(crate) syntax: SyntaxNode,
}
impl CreateViewStmt {
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
pub struct DeallocateStmt {
    pub(crate) syntax: SyntaxNode,
}
impl DeallocateStmt {
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
pub struct DeclareStmt {
    pub(crate) syntax: SyntaxNode,
}
impl DeclareStmt {
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
pub struct DeleteStmt {
    pub(crate) syntax: SyntaxNode,
}
impl DeleteStmt {
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
pub struct DiscardStmt {
    pub(crate) syntax: SyntaxNode,
}
impl DiscardStmt {
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
pub struct DoStmt {
    pub(crate) syntax: SyntaxNode,
}
impl DoStmt {
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
pub struct DropAccessMethodStmt {
    pub(crate) syntax: SyntaxNode,
}
impl DropAccessMethodStmt {
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
    pub fn aggregates(&self) -> AstChildren<CallExpr> {
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
pub struct DropCastStmt {
    pub(crate) syntax: SyntaxNode,
}
impl DropCastStmt {
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
pub struct DropCollationStmt {
    pub(crate) syntax: SyntaxNode,
}
impl DropCollationStmt {
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
pub struct DropConversionStmt {
    pub(crate) syntax: SyntaxNode,
}
impl DropConversionStmt {
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
pub struct DropDatabaseStmt {
    pub(crate) syntax: SyntaxNode,
}
impl DropDatabaseStmt {
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
pub struct DropDomainStmt {
    pub(crate) syntax: SyntaxNode,
}
impl DropDomainStmt {
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
pub struct DropEventTriggerStmt {
    pub(crate) syntax: SyntaxNode,
}
impl DropEventTriggerStmt {
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
pub struct DropExtensionStmt {
    pub(crate) syntax: SyntaxNode,
}
impl DropExtensionStmt {
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
pub struct DropForeignDataWrapperStmt {
    pub(crate) syntax: SyntaxNode,
}
impl DropForeignDataWrapperStmt {
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
pub struct DropForeignTableStmt {
    pub(crate) syntax: SyntaxNode,
}
impl DropForeignTableStmt {
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
pub struct DropFunctionStmt {
    pub(crate) syntax: SyntaxNode,
}
impl DropFunctionStmt {
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
pub struct DropGroupStmt {
    pub(crate) syntax: SyntaxNode,
}
impl DropGroupStmt {
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
pub struct DropIndexStmt {
    pub(crate) syntax: SyntaxNode,
}
impl DropIndexStmt {
    #[inline]
    pub fn if_exists(&self) -> Option<IfExists> {
        support::child(&self.syntax)
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
pub struct DropLanguageStmt {
    pub(crate) syntax: SyntaxNode,
}
impl DropLanguageStmt {
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
pub struct DropMaterializedViewStmt {
    pub(crate) syntax: SyntaxNode,
}
impl DropMaterializedViewStmt {
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
pub struct DropOperatorClassStmt {
    pub(crate) syntax: SyntaxNode,
}
impl DropOperatorClassStmt {
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
pub struct DropOperatorFamilyStmt {
    pub(crate) syntax: SyntaxNode,
}
impl DropOperatorFamilyStmt {
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
pub struct DropOperatorStmt {
    pub(crate) syntax: SyntaxNode,
}
impl DropOperatorStmt {
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
pub struct DropOwnedStmt {
    pub(crate) syntax: SyntaxNode,
}
impl DropOwnedStmt {
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
pub struct DropPolicyStmt {
    pub(crate) syntax: SyntaxNode,
}
impl DropPolicyStmt {
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
pub struct DropProcedureStmt {
    pub(crate) syntax: SyntaxNode,
}
impl DropProcedureStmt {
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
pub struct DropPublicationStmt {
    pub(crate) syntax: SyntaxNode,
}
impl DropPublicationStmt {
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
pub struct DropRoleStmt {
    pub(crate) syntax: SyntaxNode,
}
impl DropRoleStmt {
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
pub struct DropRoutineStmt {
    pub(crate) syntax: SyntaxNode,
}
impl DropRoutineStmt {
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
pub struct DropRuleStmt {
    pub(crate) syntax: SyntaxNode,
}
impl DropRuleStmt {
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
pub struct DropSchemaStmt {
    pub(crate) syntax: SyntaxNode,
}
impl DropSchemaStmt {
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
pub struct DropSequenceStmt {
    pub(crate) syntax: SyntaxNode,
}
impl DropSequenceStmt {
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
pub struct DropServerStmt {
    pub(crate) syntax: SyntaxNode,
}
impl DropServerStmt {
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
pub struct DropStatisticsStmt {
    pub(crate) syntax: SyntaxNode,
}
impl DropStatisticsStmt {
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
pub struct DropSubscriptionStmt {
    pub(crate) syntax: SyntaxNode,
}
impl DropSubscriptionStmt {
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
pub struct DropTablespaceStmt {
    pub(crate) syntax: SyntaxNode,
}
impl DropTablespaceStmt {
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
pub struct DropTextSearchConfigStmt {
    pub(crate) syntax: SyntaxNode,
}
impl DropTextSearchConfigStmt {
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
pub struct DropTextSearchDictStmt {
    pub(crate) syntax: SyntaxNode,
}
impl DropTextSearchDictStmt {
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
pub struct DropTextSearchParserStmt {
    pub(crate) syntax: SyntaxNode,
}
impl DropTextSearchParserStmt {
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
pub struct DropTextSearchTemplateStmt {
    pub(crate) syntax: SyntaxNode,
}
impl DropTextSearchTemplateStmt {
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
pub struct DropTransformStmt {
    pub(crate) syntax: SyntaxNode,
}
impl DropTransformStmt {
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
pub struct DropTriggerStmt {
    pub(crate) syntax: SyntaxNode,
}
impl DropTriggerStmt {
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
pub struct DropTypeStmt {
    pub(crate) syntax: SyntaxNode,
}
impl DropTypeStmt {
    #[inline]
    pub fn if_exists(&self) -> Option<IfExists> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn drop_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::DROP_KW)
    }
    #[inline]
    pub fn type_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::TYPE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DropUserMappingStmt {
    pub(crate) syntax: SyntaxNode,
}
impl DropUserMappingStmt {
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
pub struct DropUserStmt {
    pub(crate) syntax: SyntaxNode,
}
impl DropUserStmt {
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
pub struct DropViewStmt {
    pub(crate) syntax: SyntaxNode,
}
impl DropViewStmt {
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
pub struct ExcludeConstraint {
    pub(crate) syntax: SyntaxNode,
}
impl ExcludeConstraint {
    #[inline]
    pub fn exclude_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::EXCLUDE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExecuteStmt {
    pub(crate) syntax: SyntaxNode,
}
impl ExecuteStmt {
    #[inline]
    pub fn execute_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::EXECUTE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExplainStmt {
    pub(crate) syntax: SyntaxNode,
}
impl ExplainStmt {
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
pub struct FetchStmt {
    pub(crate) syntax: SyntaxNode,
}
impl FetchStmt {
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
    pub fn column_list(&self) -> Option<ColumnList> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn path(&self) -> Option<Path> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn foreign_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::FOREIGN_KW)
    }
    #[inline]
    pub fn full_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::FULL_KW)
    }
    #[inline]
    pub fn key_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::KEY_KW)
    }
    #[inline]
    pub fn match_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::MATCH_KW)
    }
    #[inline]
    pub fn partial_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::PARTIAL_KW)
    }
    #[inline]
    pub fn references_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::REFERENCES_KW)
    }
    #[inline]
    pub fn simple_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SIMPLE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FromClause {
    pub(crate) syntax: SyntaxNode,
}
impl FromClause {
    #[inline]
    pub fn from_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::FROM_KW)
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
pub struct GrantStmt {
    pub(crate) syntax: SyntaxNode,
}
impl GrantStmt {
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
    pub fn expr(&self) -> Option<Expr> {
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
pub struct InsertStmt {
    pub(crate) syntax: SyntaxNode,
}
impl InsertStmt {
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
pub struct IsNull {
    pub(crate) syntax: SyntaxNode,
}
impl IsNull {
    #[inline]
    pub fn is_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::IS_KW)
    }
    #[inline]
    pub fn null_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::NULL_KW)
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
pub struct ListenStmt {
    pub(crate) syntax: SyntaxNode,
}
impl ListenStmt {
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
pub struct LoadStmt {
    pub(crate) syntax: SyntaxNode,
}
impl LoadStmt {
    #[inline]
    pub fn load_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::LOAD_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LockStmt {
    pub(crate) syntax: SyntaxNode,
}
impl LockStmt {
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
pub struct MergeStmt {
    pub(crate) syntax: SyntaxNode,
}
impl MergeStmt {
    #[inline]
    pub fn merge_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::MERGE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MoveStmt {
    pub(crate) syntax: SyntaxNode,
}
impl MoveStmt {
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
pub struct NotifyStmt {
    pub(crate) syntax: SyntaxNode,
}
impl NotifyStmt {
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
    pub fn parallel_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::PARALLEL_KW)
    }
    #[inline]
    pub fn restricted_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::RESTRICTED_KW)
    }
    #[inline]
    pub fn safe_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SAFE_KW)
    }
    #[inline]
    pub fn unsafe_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::UNSAFE_KW)
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
    pub fn percent_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::PERCENT)
    }
    #[inline]
    pub fn type_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::TYPE_KW)
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
pub struct PrepareStmt {
    pub(crate) syntax: SyntaxNode,
}
impl PrepareStmt {
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
pub struct PrepareTransactionStmt {
    pub(crate) syntax: SyntaxNode,
}
impl PrepareTransactionStmt {
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
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn constraint_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CONSTRAINT_KW)
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
pub struct ReassignStmt {
    pub(crate) syntax: SyntaxNode,
}
impl ReassignStmt {
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
    pub fn name_ref(&self) -> Option<NameRef> {
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
    pub fn full_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::FULL_KW)
    }
    #[inline]
    pub fn match_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::MATCH_KW)
    }
    #[inline]
    pub fn partial_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::PARTIAL_KW)
    }
    #[inline]
    pub fn references_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::REFERENCES_KW)
    }
    #[inline]
    pub fn simple_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SIMPLE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RefreshStmt {
    pub(crate) syntax: SyntaxNode,
}
impl RefreshStmt {
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
pub struct ReindexStmt {
    pub(crate) syntax: SyntaxNode,
}
impl ReindexStmt {
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
pub struct ReleaseSavepointStmt {
    pub(crate) syntax: SyntaxNode,
}
impl ReleaseSavepointStmt {
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
pub struct ResetStmt {
    pub(crate) syntax: SyntaxNode,
}
impl ResetStmt {
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
pub struct RevokeStmt {
    pub(crate) syntax: SyntaxNode,
}
impl RevokeStmt {
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
pub struct RollbackStmt {
    pub(crate) syntax: SyntaxNode,
}
impl RollbackStmt {
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
pub struct SavepointStmt {
    pub(crate) syntax: SyntaxNode,
}
impl SavepointStmt {
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
pub struct SecurityLabelStmt {
    pub(crate) syntax: SyntaxNode,
}
impl SecurityLabelStmt {
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
    pub fn select_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SELECT_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SelectClause {
    pub(crate) syntax: SyntaxNode,
}
impl SelectClause {
    #[inline]
    pub fn select_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SELECT_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SelectIntoStmt {
    pub(crate) syntax: SyntaxNode,
}
impl SelectIntoStmt {
    #[inline]
    pub fn select_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SELECT_KW)
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
pub struct SetConstraintsStmt {
    pub(crate) syntax: SyntaxNode,
}
impl SetConstraintsStmt {
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
pub struct SetRoleStmt {
    pub(crate) syntax: SyntaxNode,
}
impl SetRoleStmt {
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
pub struct SetSessionAuthStmt {
    pub(crate) syntax: SyntaxNode,
}
impl SetSessionAuthStmt {
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
pub struct SetStmt {
    pub(crate) syntax: SyntaxNode,
}
impl SetStmt {
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
pub struct SetTransactionStmt {
    pub(crate) syntax: SyntaxNode,
}
impl SetTransactionStmt {
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
    pub fn set_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::SET_KW)
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
pub struct ShowStmt {
    pub(crate) syntax: SyntaxNode,
}
impl ShowStmt {
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
pub struct TableArgs {
    pub(crate) syntax: SyntaxNode,
}
impl TableArgs {
    #[inline]
    pub fn table_arg(&self) -> Option<TableArg> {
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
    pub fn comma_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::COMMA)
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
pub struct TableStmt {
    pub(crate) syntax: SyntaxNode,
}
impl TableStmt {
    #[inline]
    pub fn table_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::TABLE_KW)
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
pub struct TruncateStmt {
    pub(crate) syntax: SyntaxNode,
}
impl TruncateStmt {
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
pub struct UniqueConstraint {
    pub(crate) syntax: SyntaxNode,
}
impl UniqueConstraint {
    #[inline]
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
    #[inline]
    pub fn constraint_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::CONSTRAINT_KW)
    }
    #[inline]
    pub fn unique_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, SyntaxKind::UNIQUE_KW)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnlistenStmt {
    pub(crate) syntax: SyntaxNode,
}
impl UnlistenStmt {
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
pub struct UpdateStmt {
    pub(crate) syntax: SyntaxNode,
}
impl UpdateStmt {
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
pub struct VacuumStmt {
    pub(crate) syntax: SyntaxNode,
}
impl VacuumStmt {
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
    BinExpr(BinExpr),
    CallExpr(CallExpr),
    CaseExpr(CaseExpr),
    CastExpr(CastExpr),
    FieldExpr(FieldExpr),
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
pub enum ParamMode {
    ParamIn(ParamIn),
    ParamInOut(ParamInOut),
    ParamOut(ParamOut),
    ParamVariadic(ParamVariadic),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Stmt {
    AlterAggregate(AlterAggregate),
    AlterDomain(AlterDomain),
    AlterTable(AlterTable),
    Begin(Begin),
    Commit(Commit),
    CreateAggregate(CreateAggregate),
    CreateDomain(CreateDomain),
    CreateFunc(CreateFunc),
    CreateIndex(CreateIndex),
    CreateMaterializedViewStmt(CreateMaterializedViewStmt),
    CreateTable(CreateTable),
    CreateTableAsStmt(CreateTableAsStmt),
    DeclareStmt(DeclareStmt),
    DeleteStmt(DeleteStmt),
    DropAggregate(DropAggregate),
    DropDatabase(DropDatabase),
    DropIndex(DropIndex),
    DropTable(DropTable),
    DropType(DropType),
    ExecuteStmt(ExecuteStmt),
    InsertStmt(InsertStmt),
    MergeStmt(MergeStmt),
    NotifyStmt(NotifyStmt),
    ReleaseSavepointStmt(ReleaseSavepointStmt),
    RevokeStmt(RevokeStmt),
    Rollback(Rollback),
    RollbackStmt(RollbackStmt),
    SavepointStmt(SavepointStmt),
    Select(Select),
    TableStmt(TableStmt),
    TruncateStmt(TruncateStmt),
    UpdateStmt(UpdateStmt),
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
impl AstNode for AlterAggregateStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_AGGREGATE_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterCollationStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_COLLATION_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
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
impl AstNode for AlterConversionStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_CONVERSION_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterDatabaseStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_DATABASE_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterDefaultPrivilegesStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_DEFAULT_PRIVILEGES_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
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
impl AstNode for AlterDomainStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_DOMAIN_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterEventTriggerStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_EVENT_TRIGGER_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterExtensionStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_EXTENSION_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterForeignDataWrapperStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_FOREIGN_DATA_WRAPPER_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterForeignTableStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_FOREIGN_TABLE_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterFunctionStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_FUNCTION_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterGroupStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_GROUP_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterIndexStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_INDEX_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterLanguageStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_LANGUAGE_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterLargeObjectStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_LARGE_OBJECT_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterMaterializedViewStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_MATERIALIZED_VIEW_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterOperatorClassStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_OPERATOR_CLASS_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterOperatorFamilyStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_OPERATOR_FAMILY_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterOperatorStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_OPERATOR_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterPolicyStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_POLICY_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterProcedureStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_PROCEDURE_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterPublicationStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_PUBLICATION_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterRoleStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_ROLE_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterRoutineStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_ROUTINE_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterRuleStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_RULE_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterSchemaStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_SCHEMA_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterSequenceStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_SEQUENCE_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterServerStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_SERVER_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterStatisticsStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_STATISTICS_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterSubscriptionStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_SUBSCRIPTION_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterSystemStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_SYSTEM_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
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
impl AstNode for AlterTablespaceStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_TABLESPACE_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterTextSearchConfigurationStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_TEXT_SEARCH_CONFIGURATION_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterTextSearchDictionaryStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_TEXT_SEARCH_DICTIONARY_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterTextSearchParserStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_TEXT_SEARCH_PARSER_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterTextSearchTemplateStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_TEXT_SEARCH_TEMPLATE_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterTriggerStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_TRIGGER_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterTypeStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_TYPE_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterUserMappingStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_USER_MAPPING_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterUserStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_USER_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AlterViewStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ALTER_VIEW_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for AnalyzeStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ANALYZE_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
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
impl AstNode for CallStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CALL_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
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
impl AstNode for CheckpointStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CHECKPOINT_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CloseStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CLOSE_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
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
impl AstNode for ClusterStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CLUSTER_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
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
impl AstNode for CommentStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::COMMENT_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
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
impl AstNode for ConstraintOptionList {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CONSTRAINT_OPTION_LIST
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
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
impl AstNode for CopyStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::COPY_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
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
impl AstNode for CreateAccessMethodStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_ACCESS_METHOD_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
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
impl AstNode for CreateAggregateStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_AGGREGATE_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CreateCastStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_CAST_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CreateCollationStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_COLLATION_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CreateConversionStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_CONVERSION_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CreateDatabaseStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_DATABASE_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
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
impl AstNode for CreateDomainStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_DOMAIN_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CreateEventTriggerStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_EVENT_TRIGGER_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CreateExtensionStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_EXTENSION_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CreateForeignDataWrapperStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_FOREIGN_DATA_WRAPPER_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CreateForeignTableStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_FOREIGN_TABLE_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CreateFunc {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_FUNC
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CreateFunctionStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_FUNCTION_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CreateGroupStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_GROUP_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
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
impl AstNode for CreateLanguageStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_LANGUAGE_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CreateMaterializedViewStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_MATERIALIZED_VIEW_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CreateOperatorClassStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_OPERATOR_CLASS_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CreateOperatorFamilyStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_OPERATOR_FAMILY_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CreateOperatorStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_OPERATOR_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CreatePolicyStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_POLICY_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CreateProcedureStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_PROCEDURE_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CreatePublicationStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_PUBLICATION_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CreateRoleStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_ROLE_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CreateRuleStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_RULE_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CreateSchemaStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_SCHEMA_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CreateSequenceStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_SEQUENCE_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CreateServerStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_SERVER_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CreateStatisticsStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_STATISTICS_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CreateSubscriptionStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_SUBSCRIPTION_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
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
impl AstNode for CreateTableAsStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_TABLE_AS_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CreateTablespaceStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_TABLESPACE_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CreateTextSearchConfigurationStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_TEXT_SEARCH_CONFIGURATION_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CreateTextSearchDictionaryStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_TEXT_SEARCH_DICTIONARY_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CreateTextSearchParserStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_TEXT_SEARCH_PARSER_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CreateTextSearchTemplateStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_TEXT_SEARCH_TEMPLATE_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CreateTransformStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_TRANSFORM_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CreateTriggerStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_TRIGGER_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CreateTypeStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_TYPE_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CreateUserMappingStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_USER_MAPPING_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CreateUserStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_USER_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for CreateViewStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CREATE_VIEW_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
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
impl AstNode for DeallocateStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DEALLOCATE_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DeclareStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DECLARE_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
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
impl AstNode for DeleteStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DELETE_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
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
impl AstNode for DiscardStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DISCARD_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
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
impl AstNode for DoStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DO_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
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
impl AstNode for DropAccessMethodStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_ACCESS_METHOD_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
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
impl AstNode for DropCastStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_CAST_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DropCollationStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_COLLATION_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
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
impl AstNode for DropConversionStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_CONVERSION_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
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
impl AstNode for DropDatabaseStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_DATABASE_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
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
impl AstNode for DropDomainStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_DOMAIN_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DropEventTriggerStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_EVENT_TRIGGER_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
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
impl AstNode for DropExtensionStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_EXTENSION_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DropForeignDataWrapperStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_FOREIGN_DATA_WRAPPER_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DropForeignTableStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_FOREIGN_TABLE_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DropFunctionStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_FUNCTION_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DropGroupStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_GROUP_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
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
impl AstNode for DropIndexStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_INDEX_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DropLanguageStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_LANGUAGE_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DropMaterializedViewStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_MATERIALIZED_VIEW_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
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
impl AstNode for DropOperatorClassStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_OPERATOR_CLASS_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DropOperatorFamilyStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_OPERATOR_FAMILY_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DropOperatorStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_OPERATOR_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DropOwnedStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_OWNED_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DropPolicyStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_POLICY_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DropProcedureStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_PROCEDURE_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DropPublicationStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_PUBLICATION_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DropRoleStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_ROLE_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DropRoutineStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_ROUTINE_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DropRuleStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_RULE_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DropSchemaStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_SCHEMA_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DropSequenceStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_SEQUENCE_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DropServerStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_SERVER_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DropStatisticsStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_STATISTICS_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DropSubscriptionStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_SUBSCRIPTION_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
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
impl AstNode for DropTablespaceStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_TABLESPACE_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DropTextSearchConfigStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_TEXT_SEARCH_CONFIG_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DropTextSearchDictStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_TEXT_SEARCH_DICT_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DropTextSearchParserStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_TEXT_SEARCH_PARSER_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DropTextSearchTemplateStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_TEXT_SEARCH_TEMPLATE_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DropTransformStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_TRANSFORM_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DropTriggerStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_TRIGGER_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
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
impl AstNode for DropTypeStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_TYPE_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DropUserMappingStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_USER_MAPPING_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DropUserStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_USER_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for DropViewStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DROP_VIEW_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
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
impl AstNode for ExecuteStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::EXECUTE_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for ExplainStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::EXPLAIN_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
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
impl AstNode for FetchStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::FETCH_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
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
impl AstNode for GrantStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::GRANT_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
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
impl AstNode for InsertStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::INSERT_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
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
impl AstNode for IsNull {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::IS_NULL
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
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
impl AstNode for ListenStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::LISTEN_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
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
impl AstNode for LoadStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::LOAD_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for LockStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::LOCK_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
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
impl AstNode for MergeStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::MERGE_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for MoveStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::MOVE_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
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
impl AstNode for NotifyStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::NOTIFY_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
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
impl AstNode for PrepareStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::PREPARE_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for PrepareTransactionStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::PREPARE_TRANSACTION_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
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
impl AstNode for ReassignStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::REASSIGN_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
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
impl AstNode for RefreshStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::REFRESH_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for ReindexStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::REINDEX_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
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
impl AstNode for ReleaseSavepointStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::RELEASE_SAVEPOINT_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
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
impl AstNode for ResetStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::RESET_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
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
impl AstNode for RevokeStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::REVOKE_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
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
impl AstNode for RollbackStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ROLLBACK_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
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
impl AstNode for SavepointStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SAVEPOINT_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
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
impl AstNode for SecurityLabelStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SECURITY_LABEL_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
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
impl AstNode for SelectIntoStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SELECT_INTO_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
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
impl AstNode for SetConstraintsStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SET_CONSTRAINTS_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
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
impl AstNode for SetRoleStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SET_ROLE_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
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
impl AstNode for SetSessionAuthStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SET_SESSION_AUTH_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
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
impl AstNode for SetStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SET_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
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
impl AstNode for SetTransactionStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SET_TRANSACTION_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
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
impl AstNode for ShowStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SHOW_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
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
impl AstNode for TableStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::TABLE_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
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
impl AstNode for TruncateStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::TRUNCATE_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
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
impl AstNode for UnlistenStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::UNLISTEN_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for UpdateStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::UPDATE_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
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
impl AstNode for VacuumStmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::VACUUM_STMT
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
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
            _ => return None,
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
            _ => return None,
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
            _ => return None,
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
                | SyntaxKind::BIN_EXPR
                | SyntaxKind::CALL_EXPR
                | SyntaxKind::CASE_EXPR
                | SyntaxKind::CAST_EXPR
                | SyntaxKind::FIELD_EXPR
                | SyntaxKind::LITERAL
                | SyntaxKind::NAME_REF
        )
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            SyntaxKind::ARRAY_EXPR => Expr::ArrayExpr(ArrayExpr { syntax }),
            SyntaxKind::BIN_EXPR => Expr::BinExpr(BinExpr { syntax }),
            SyntaxKind::CALL_EXPR => Expr::CallExpr(CallExpr { syntax }),
            SyntaxKind::CASE_EXPR => Expr::CaseExpr(CaseExpr { syntax }),
            SyntaxKind::CAST_EXPR => Expr::CastExpr(CastExpr { syntax }),
            SyntaxKind::FIELD_EXPR => Expr::FieldExpr(FieldExpr { syntax }),
            SyntaxKind::LITERAL => Expr::Literal(Literal { syntax }),
            SyntaxKind::NAME_REF => Expr::NameRef(NameRef { syntax }),
            _ => return None,
        };
        Some(res)
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Expr::ArrayExpr(it) => &it.syntax,
            Expr::BinExpr(it) => &it.syntax,
            Expr::CallExpr(it) => &it.syntax,
            Expr::CaseExpr(it) => &it.syntax,
            Expr::CastExpr(it) => &it.syntax,
            Expr::FieldExpr(it) => &it.syntax,
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
            _ => return None,
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
            _ => return None,
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
impl AstNode for Stmt {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            SyntaxKind::ALTER_AGGREGATE
                | SyntaxKind::ALTER_DOMAIN
                | SyntaxKind::ALTER_TABLE
                | SyntaxKind::BEGIN
                | SyntaxKind::COMMIT
                | SyntaxKind::CREATE_AGGREGATE
                | SyntaxKind::CREATE_DOMAIN
                | SyntaxKind::CREATE_FUNC
                | SyntaxKind::CREATE_INDEX
                | SyntaxKind::CREATE_MATERIALIZED_VIEW_STMT
                | SyntaxKind::CREATE_TABLE
                | SyntaxKind::CREATE_TABLE_AS_STMT
                | SyntaxKind::DECLARE_STMT
                | SyntaxKind::DELETE_STMT
                | SyntaxKind::DROP_AGGREGATE
                | SyntaxKind::DROP_DATABASE
                | SyntaxKind::DROP_INDEX
                | SyntaxKind::DROP_TABLE
                | SyntaxKind::DROP_TYPE
                | SyntaxKind::EXECUTE_STMT
                | SyntaxKind::INSERT_STMT
                | SyntaxKind::MERGE_STMT
                | SyntaxKind::NOTIFY_STMT
                | SyntaxKind::RELEASE_SAVEPOINT_STMT
                | SyntaxKind::REVOKE_STMT
                | SyntaxKind::ROLLBACK
                | SyntaxKind::ROLLBACK_STMT
                | SyntaxKind::SAVEPOINT_STMT
                | SyntaxKind::SELECT
                | SyntaxKind::TABLE_STMT
                | SyntaxKind::TRUNCATE_STMT
                | SyntaxKind::UPDATE_STMT
                | SyntaxKind::VALUES
        )
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            SyntaxKind::ALTER_AGGREGATE => Stmt::AlterAggregate(AlterAggregate { syntax }),
            SyntaxKind::ALTER_DOMAIN => Stmt::AlterDomain(AlterDomain { syntax }),
            SyntaxKind::ALTER_TABLE => Stmt::AlterTable(AlterTable { syntax }),
            SyntaxKind::BEGIN => Stmt::Begin(Begin { syntax }),
            SyntaxKind::COMMIT => Stmt::Commit(Commit { syntax }),
            SyntaxKind::CREATE_AGGREGATE => Stmt::CreateAggregate(CreateAggregate { syntax }),
            SyntaxKind::CREATE_DOMAIN => Stmt::CreateDomain(CreateDomain { syntax }),
            SyntaxKind::CREATE_FUNC => Stmt::CreateFunc(CreateFunc { syntax }),
            SyntaxKind::CREATE_INDEX => Stmt::CreateIndex(CreateIndex { syntax }),
            SyntaxKind::CREATE_MATERIALIZED_VIEW_STMT => {
                Stmt::CreateMaterializedViewStmt(CreateMaterializedViewStmt { syntax })
            }
            SyntaxKind::CREATE_TABLE => Stmt::CreateTable(CreateTable { syntax }),
            SyntaxKind::CREATE_TABLE_AS_STMT => {
                Stmt::CreateTableAsStmt(CreateTableAsStmt { syntax })
            }
            SyntaxKind::DECLARE_STMT => Stmt::DeclareStmt(DeclareStmt { syntax }),
            SyntaxKind::DELETE_STMT => Stmt::DeleteStmt(DeleteStmt { syntax }),
            SyntaxKind::DROP_AGGREGATE => Stmt::DropAggregate(DropAggregate { syntax }),
            SyntaxKind::DROP_DATABASE => Stmt::DropDatabase(DropDatabase { syntax }),
            SyntaxKind::DROP_INDEX => Stmt::DropIndex(DropIndex { syntax }),
            SyntaxKind::DROP_TABLE => Stmt::DropTable(DropTable { syntax }),
            SyntaxKind::DROP_TYPE => Stmt::DropType(DropType { syntax }),
            SyntaxKind::EXECUTE_STMT => Stmt::ExecuteStmt(ExecuteStmt { syntax }),
            SyntaxKind::INSERT_STMT => Stmt::InsertStmt(InsertStmt { syntax }),
            SyntaxKind::MERGE_STMT => Stmt::MergeStmt(MergeStmt { syntax }),
            SyntaxKind::NOTIFY_STMT => Stmt::NotifyStmt(NotifyStmt { syntax }),
            SyntaxKind::RELEASE_SAVEPOINT_STMT => {
                Stmt::ReleaseSavepointStmt(ReleaseSavepointStmt { syntax })
            }
            SyntaxKind::REVOKE_STMT => Stmt::RevokeStmt(RevokeStmt { syntax }),
            SyntaxKind::ROLLBACK => Stmt::Rollback(Rollback { syntax }),
            SyntaxKind::ROLLBACK_STMT => Stmt::RollbackStmt(RollbackStmt { syntax }),
            SyntaxKind::SAVEPOINT_STMT => Stmt::SavepointStmt(SavepointStmt { syntax }),
            SyntaxKind::SELECT => Stmt::Select(Select { syntax }),
            SyntaxKind::TABLE_STMT => Stmt::TableStmt(TableStmt { syntax }),
            SyntaxKind::TRUNCATE_STMT => Stmt::TruncateStmt(TruncateStmt { syntax }),
            SyntaxKind::UPDATE_STMT => Stmt::UpdateStmt(UpdateStmt { syntax }),
            SyntaxKind::VALUES => Stmt::Values(Values { syntax }),
            _ => return None,
        };
        Some(res)
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Stmt::AlterAggregate(it) => &it.syntax,
            Stmt::AlterDomain(it) => &it.syntax,
            Stmt::AlterTable(it) => &it.syntax,
            Stmt::Begin(it) => &it.syntax,
            Stmt::Commit(it) => &it.syntax,
            Stmt::CreateAggregate(it) => &it.syntax,
            Stmt::CreateDomain(it) => &it.syntax,
            Stmt::CreateFunc(it) => &it.syntax,
            Stmt::CreateIndex(it) => &it.syntax,
            Stmt::CreateMaterializedViewStmt(it) => &it.syntax,
            Stmt::CreateTable(it) => &it.syntax,
            Stmt::CreateTableAsStmt(it) => &it.syntax,
            Stmt::DeclareStmt(it) => &it.syntax,
            Stmt::DeleteStmt(it) => &it.syntax,
            Stmt::DropAggregate(it) => &it.syntax,
            Stmt::DropDatabase(it) => &it.syntax,
            Stmt::DropIndex(it) => &it.syntax,
            Stmt::DropTable(it) => &it.syntax,
            Stmt::DropType(it) => &it.syntax,
            Stmt::ExecuteStmt(it) => &it.syntax,
            Stmt::InsertStmt(it) => &it.syntax,
            Stmt::MergeStmt(it) => &it.syntax,
            Stmt::NotifyStmt(it) => &it.syntax,
            Stmt::ReleaseSavepointStmt(it) => &it.syntax,
            Stmt::RevokeStmt(it) => &it.syntax,
            Stmt::Rollback(it) => &it.syntax,
            Stmt::RollbackStmt(it) => &it.syntax,
            Stmt::SavepointStmt(it) => &it.syntax,
            Stmt::Select(it) => &it.syntax,
            Stmt::TableStmt(it) => &it.syntax,
            Stmt::TruncateStmt(it) => &it.syntax,
            Stmt::UpdateStmt(it) => &it.syntax,
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
impl From<AlterDomain> for Stmt {
    #[inline]
    fn from(node: AlterDomain) -> Stmt {
        Stmt::AlterDomain(node)
    }
}
impl From<AlterTable> for Stmt {
    #[inline]
    fn from(node: AlterTable) -> Stmt {
        Stmt::AlterTable(node)
    }
}
impl From<Begin> for Stmt {
    #[inline]
    fn from(node: Begin) -> Stmt {
        Stmt::Begin(node)
    }
}
impl From<Commit> for Stmt {
    #[inline]
    fn from(node: Commit) -> Stmt {
        Stmt::Commit(node)
    }
}
impl From<CreateAggregate> for Stmt {
    #[inline]
    fn from(node: CreateAggregate) -> Stmt {
        Stmt::CreateAggregate(node)
    }
}
impl From<CreateDomain> for Stmt {
    #[inline]
    fn from(node: CreateDomain) -> Stmt {
        Stmt::CreateDomain(node)
    }
}
impl From<CreateFunc> for Stmt {
    #[inline]
    fn from(node: CreateFunc) -> Stmt {
        Stmt::CreateFunc(node)
    }
}
impl From<CreateIndex> for Stmt {
    #[inline]
    fn from(node: CreateIndex) -> Stmt {
        Stmt::CreateIndex(node)
    }
}
impl From<CreateMaterializedViewStmt> for Stmt {
    #[inline]
    fn from(node: CreateMaterializedViewStmt) -> Stmt {
        Stmt::CreateMaterializedViewStmt(node)
    }
}
impl From<CreateTable> for Stmt {
    #[inline]
    fn from(node: CreateTable) -> Stmt {
        Stmt::CreateTable(node)
    }
}
impl From<CreateTableAsStmt> for Stmt {
    #[inline]
    fn from(node: CreateTableAsStmt) -> Stmt {
        Stmt::CreateTableAsStmt(node)
    }
}
impl From<DeclareStmt> for Stmt {
    #[inline]
    fn from(node: DeclareStmt) -> Stmt {
        Stmt::DeclareStmt(node)
    }
}
impl From<DeleteStmt> for Stmt {
    #[inline]
    fn from(node: DeleteStmt) -> Stmt {
        Stmt::DeleteStmt(node)
    }
}
impl From<DropAggregate> for Stmt {
    #[inline]
    fn from(node: DropAggregate) -> Stmt {
        Stmt::DropAggregate(node)
    }
}
impl From<DropDatabase> for Stmt {
    #[inline]
    fn from(node: DropDatabase) -> Stmt {
        Stmt::DropDatabase(node)
    }
}
impl From<DropIndex> for Stmt {
    #[inline]
    fn from(node: DropIndex) -> Stmt {
        Stmt::DropIndex(node)
    }
}
impl From<DropTable> for Stmt {
    #[inline]
    fn from(node: DropTable) -> Stmt {
        Stmt::DropTable(node)
    }
}
impl From<DropType> for Stmt {
    #[inline]
    fn from(node: DropType) -> Stmt {
        Stmt::DropType(node)
    }
}
impl From<ExecuteStmt> for Stmt {
    #[inline]
    fn from(node: ExecuteStmt) -> Stmt {
        Stmt::ExecuteStmt(node)
    }
}
impl From<InsertStmt> for Stmt {
    #[inline]
    fn from(node: InsertStmt) -> Stmt {
        Stmt::InsertStmt(node)
    }
}
impl From<MergeStmt> for Stmt {
    #[inline]
    fn from(node: MergeStmt) -> Stmt {
        Stmt::MergeStmt(node)
    }
}
impl From<NotifyStmt> for Stmt {
    #[inline]
    fn from(node: NotifyStmt) -> Stmt {
        Stmt::NotifyStmt(node)
    }
}
impl From<ReleaseSavepointStmt> for Stmt {
    #[inline]
    fn from(node: ReleaseSavepointStmt) -> Stmt {
        Stmt::ReleaseSavepointStmt(node)
    }
}
impl From<RevokeStmt> for Stmt {
    #[inline]
    fn from(node: RevokeStmt) -> Stmt {
        Stmt::RevokeStmt(node)
    }
}
impl From<Rollback> for Stmt {
    #[inline]
    fn from(node: Rollback) -> Stmt {
        Stmt::Rollback(node)
    }
}
impl From<RollbackStmt> for Stmt {
    #[inline]
    fn from(node: RollbackStmt) -> Stmt {
        Stmt::RollbackStmt(node)
    }
}
impl From<SavepointStmt> for Stmt {
    #[inline]
    fn from(node: SavepointStmt) -> Stmt {
        Stmt::SavepointStmt(node)
    }
}
impl From<Select> for Stmt {
    #[inline]
    fn from(node: Select) -> Stmt {
        Stmt::Select(node)
    }
}
impl From<TableStmt> for Stmt {
    #[inline]
    fn from(node: TableStmt) -> Stmt {
        Stmt::TableStmt(node)
    }
}
impl From<TruncateStmt> for Stmt {
    #[inline]
    fn from(node: TruncateStmt) -> Stmt {
        Stmt::TruncateStmt(node)
    }
}
impl From<UpdateStmt> for Stmt {
    #[inline]
    fn from(node: UpdateStmt) -> Stmt {
        Stmt::UpdateStmt(node)
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
        matches!(
            kind,
            SyntaxKind::COLUMN | SyntaxKind::LIKE_CLAUSE | SyntaxKind::TABLE_CONSTRAINT
        )
    }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            SyntaxKind::COLUMN => TableArg::Column(Column { syntax }),
            SyntaxKind::LIKE_CLAUSE => TableArg::LikeClause(LikeClause { syntax }),
            // SyntaxKind::TABLE_CONSTRAINT => TableArg::TableConstraint(TableConstraint { syntax }),
            _ => return None,
        };
        Some(res)
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        match self {
            TableArg::Column(it) => &it.syntax,
            TableArg::LikeClause(it) => &it.syntax,
            TableArg::TableConstraint(it) => &it.syntax(),
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
impl From<TableConstraint> for TableArg {
    #[inline]
    fn from(node: TableConstraint) -> TableArg {
        TableArg::TableConstraint(node)
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
            _ => return None,
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
            _ => return None,
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
            _ => return None,
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
