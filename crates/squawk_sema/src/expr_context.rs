#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ExprContext {
    Unrestricted,
    Where,
    JoinCondition,
    GroupBy,
    Having,
    Filter,
    WindowDefinition,
    WindowRange,
    WindowRows,
    WindowGroups,
    Limit,
    Offset,
    Values,
    ValuesSingle,
    Returning,
    Insert,
    Update,
    FromFunction,
    MergeWhen,
    CheckConstraint,
    Default,
    GeneratedColumn,
    IndexExpression,
    IndexPredicate,
    StatisticsExpression,
    PartitionKey,
    PartitionBound,
    Policy,
    TriggerWhen,
    CallArgument,
    Transform,
    CopyWhere,
    ExecuteParameter,
}

impl ExprContext {
    pub(crate) fn allows_aggregates(self) -> bool {
        matches!(
            self,
            ExprContext::Unrestricted | ExprContext::Having | ExprContext::WindowDefinition
        )
    }

    pub(crate) fn allows_windows(self) -> bool {
        self == ExprContext::Unrestricted
    }

    pub(crate) fn allows_set_returning(self) -> bool {
        match self {
            ExprContext::Unrestricted
            | ExprContext::GroupBy
            | ExprContext::WindowDefinition
            | ExprContext::ValuesSingle
            | ExprContext::Insert
            | ExprContext::FromFunction => true,
            ExprContext::Where
            | ExprContext::JoinCondition
            | ExprContext::Having
            | ExprContext::Filter
            | ExprContext::WindowRange
            | ExprContext::WindowRows
            | ExprContext::WindowGroups
            | ExprContext::Limit
            | ExprContext::Offset
            | ExprContext::Values
            | ExprContext::Returning
            | ExprContext::Update
            | ExprContext::MergeWhen
            | ExprContext::CheckConstraint
            | ExprContext::Default
            | ExprContext::GeneratedColumn
            | ExprContext::IndexExpression
            | ExprContext::IndexPredicate
            | ExprContext::StatisticsExpression
            | ExprContext::PartitionKey
            | ExprContext::PartitionBound
            | ExprContext::Policy
            | ExprContext::TriggerWhen
            | ExprContext::CallArgument
            | ExprContext::Transform
            | ExprContext::CopyWhere
            | ExprContext::ExecuteParameter => false,
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) enum ConditionKind {
    Where,
    Having,
    On,
    Check,
    Filter,
    Policy,
    TriggerWhen,
    MergeWhen,
    IndexPredicate,
    CopyWhere,
    PublicationWhere,
    CaseWhen,
    And,
    Or,
    Not,
    GraphWhere,
}

impl ConditionKind {
    pub(crate) fn context(self) -> Option<ExprContext> {
        match self {
            ConditionKind::Where => Some(ExprContext::Where),
            ConditionKind::Having => Some(ExprContext::Having),
            ConditionKind::On => Some(ExprContext::JoinCondition),
            ConditionKind::Check => Some(ExprContext::CheckConstraint),
            ConditionKind::Filter => Some(ExprContext::Filter),
            ConditionKind::Policy => Some(ExprContext::Policy),
            ConditionKind::TriggerWhen => Some(ExprContext::TriggerWhen),
            ConditionKind::MergeWhen => Some(ExprContext::MergeWhen),
            ConditionKind::IndexPredicate => Some(ExprContext::IndexPredicate),
            ConditionKind::CopyWhere => Some(ExprContext::CopyWhere),
            ConditionKind::PublicationWhere
            | ConditionKind::CaseWhen
            | ConditionKind::And
            | ConditionKind::Or
            | ConditionKind::Not
            | ConditionKind::GraphWhere => None,
        }
    }

    pub(crate) fn keyword(self) -> &'static str {
        match self {
            ConditionKind::Where
            | ConditionKind::IndexPredicate
            | ConditionKind::CopyWhere
            | ConditionKind::PublicationWhere
            | ConditionKind::GraphWhere => "where",
            ConditionKind::Having => "having",
            ConditionKind::On => "on",
            ConditionKind::Check => "check",
            ConditionKind::Filter => "filter",
            ConditionKind::Policy => "policy",
            ConditionKind::TriggerWhen | ConditionKind::MergeWhen | ConditionKind::CaseWhen => {
                "when"
            }
            ConditionKind::And => "and",
            ConditionKind::Or => "or",
            ConditionKind::Not => "not",
        }
    }
}
