use std::borrow::Cow;
use std::fmt;

use crate::interner::{Interner, Name};
use crate::ir::{
    AccessMethodKind, ReferenceKind, Resolution, RoutineKind, TableKind, TextSearchKind, TypeKind,
};
use crate::lower::{ExprContext, OutputClause, TableLookup};
use crate::semantic_index::DiagnosticKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct TyName {
    pub(crate) name: Name,
    pub(crate) array: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ObjectKind {
    Relation,
    Type,
    Role,
    AccessMethod,
    ForeignDataWrapper,
    Server,
    Statistics,
    Publication,
    Subscription,
    GraphElement,
    GraphProperty,
    GraphLabel,
    Conversion,
    Language,
    EventTrigger,
    Schema,
    Tablespace,
    Database,
    Operator,
    OperatorFamily,
    OperatorClass,
    Collation,
    Extension,
    PreparedStatement,
    EnumLabel,
    Function,
    TextSearch(TextSearchKind),
    Table(TableKind),
}

impl ObjectKind {
    fn label(self) -> &'static str {
        match self {
            ObjectKind::Relation => "relation",
            ObjectKind::Type => "type",
            ObjectKind::Role => "role",
            ObjectKind::AccessMethod => "access method",
            ObjectKind::ForeignDataWrapper => "foreign-data wrapper",
            ObjectKind::Server => "server",
            ObjectKind::Statistics => "statistics object",
            ObjectKind::Publication => "publication",
            ObjectKind::Subscription => "subscription",
            ObjectKind::GraphElement => "element table",
            ObjectKind::GraphProperty => "property",
            ObjectKind::GraphLabel => "label",
            ObjectKind::Conversion => "conversion",
            ObjectKind::Language => "language",
            ObjectKind::EventTrigger => "event trigger",
            ObjectKind::Schema => "schema",
            ObjectKind::Tablespace => "tablespace",
            ObjectKind::Database => "database",
            ObjectKind::Operator => "operator",
            ObjectKind::OperatorFamily => "operator family",
            ObjectKind::OperatorClass => "operator class",
            ObjectKind::Collation => "collation",
            ObjectKind::Extension => "extension",
            ObjectKind::PreparedStatement => "prepared statement",
            ObjectKind::EnumLabel => "enum label",
            ObjectKind::Function => "function",
            ObjectKind::TextSearch(kind) => kind.label(),
            ObjectKind::Table(kind) => kind.label(),
        }
    }

    fn duplicate_kind(self) -> DiagnosticKind {
        match self {
            ObjectKind::Relation => DiagnosticKind::DuplicateRelation,
            ObjectKind::Type => DiagnosticKind::DuplicateType,
            ObjectKind::Role => DiagnosticKind::DuplicateRole,
            ObjectKind::AccessMethod => DiagnosticKind::DuplicateAccessMethod,
            ObjectKind::ForeignDataWrapper => DiagnosticKind::DuplicateForeignDataWrapper,
            ObjectKind::Server => DiagnosticKind::DuplicateServer,
            ObjectKind::Statistics => DiagnosticKind::DuplicateStatistics,
            ObjectKind::Publication => DiagnosticKind::DuplicatePublication,
            ObjectKind::Subscription => DiagnosticKind::DuplicateSubscription,
            ObjectKind::GraphElement => DiagnosticKind::DuplicateGraphElement,
            ObjectKind::GraphProperty => DiagnosticKind::DuplicateGraphProperty,
            ObjectKind::GraphLabel => DiagnosticKind::DuplicateGraphLabel,
            ObjectKind::Conversion => DiagnosticKind::DuplicateConversion,
            ObjectKind::Language => DiagnosticKind::DuplicateLanguage,
            ObjectKind::EventTrigger => DiagnosticKind::DuplicateEventTrigger,
            ObjectKind::Schema => DiagnosticKind::DuplicateSchema,
            ObjectKind::Tablespace => DiagnosticKind::DuplicateTablespace,
            ObjectKind::Database => DiagnosticKind::DuplicateDatabase,
            ObjectKind::Operator => DiagnosticKind::DuplicateOperator,
            ObjectKind::OperatorFamily => DiagnosticKind::DuplicateOperatorFamily,
            ObjectKind::OperatorClass => DiagnosticKind::DuplicateOperatorClass,
            ObjectKind::Collation => DiagnosticKind::DuplicateCollation,
            ObjectKind::Extension => DiagnosticKind::DuplicateExtension,
            ObjectKind::PreparedStatement => DiagnosticKind::DuplicatePreparedStatement,
            ObjectKind::EnumLabel => DiagnosticKind::DuplicateEnumLabel,
            ObjectKind::Function => DiagnosticKind::DuplicateFunction,
            ObjectKind::TextSearch(_) => DiagnosticKind::DuplicateTextSearch,
            ObjectKind::Table(_) => DiagnosticKind::DuplicateRelation,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SemaError {
    CannotChangeDatabase,
    ValuesLengthMismatch,
    InsertMoreExpressionsThanTargets,
    ViewTooManyColumnNames,
    CreateAsTooManyColumnNames,
    ReplaceConstraintTrigger,
    RenameSelectRule,
    NestedAggregate,
    AggregateContainsWindow,
    NestedWindow,
    MisplacedAggregate {
        context: ExprContext,
    },
    MisplacedWindow {
        context: ExprContext,
    },
    Duplicate {
        object: ObjectKind,
        name: Name,
    },
    DuplicateInSchema {
        object: ObjectKind,
        name: Name,
        schema: Name,
    },
    DuplicateForAccessMethod {
        object: ObjectKind,
        name: Name,
        access_method: Option<Name>,
    },
    DuplicatePolicy {
        name: Name,
        table: Name,
    },
    DuplicateTrigger {
        name: Name,
        table: Name,
    },
    DuplicateRule {
        name: Name,
        table: Name,
    },
    DuplicateConstraint {
        name: Name,
        owner: Name,
        domain: bool,
    },
    DuplicateColumn {
        name: Name,
        relation: Name,
    },
    DuplicateFunction {
        name: Name,
        schema: Option<Name>,
    },
    DuplicateTransform {
        ty: TyName,
        language: Name,
    },
    ColumnSpecifiedMoreThanOnce {
        name: Name,
    },
    DuplicateCte {
        name: Name,
    },
    DuplicateSource {
        name: Name,
    },
    DuplicateAssignment {
        name: Name,
    },
    DuplicateWindow {
        name: Name,
    },
    DuplicateParam {
        name: Name,
    },
    DependentObjects {
        object: ObjectKind,
        name: Name,
    },
    RequiredBySystem {
        schema: Name,
    },
    AccessMethodKindMismatch {
        name: Name,
        expected: AccessMethodKind,
    },
    UnresolvedCast {
        source: TyName,
        target: TyName,
    },
    UnresolvedTransform {
        ty: TyName,
        language: Name,
    },
    InvalidEnumValue {
        ty: Name,
        value: Name,
    },
    NotEnum {
        name: Name,
    },
    NotComposite {
        name: Name,
    },
    WrongTypeKind {
        name: Name,
        expected: TypeKind,
    },
    TypedTableAlter {
        name: Name,
    },
    InvalidDomainConstraint {
        what: &'static str,
    },
    CompoundColumnCount {
        op: &'static str,
    },
    ColumnListTooLong {
        what: &'static str,
        name: Name,
        available: usize,
        specified: usize,
    },
    InsertMoreTargetsThanExpressions {
        row_hint: bool,
    },
    MissingShellType,
    WrongRelationKind {
        name: Name,
        expected: TableLookup,
        kind: TableKind,
        drop: bool,
    },
    CannotCopy {
        name: Name,
        copy_from: bool,
        kind: TableKind,
    },
    CannotCreateIndex {
        name: Name,
        kind: TableKind,
    },
    NotIdentityColumn {
        column: Name,
        relation: Name,
    },
    ConstraintTrigger {
        name: Name,
        relation: Name,
    },
    DropReturnRule {
        name: Name,
        kind: TableKind,
        table: Name,
    },
    MultiplePrimaryKeys {
        table: Name,
    },
    InvalidOrdinal {
        clause: OutputClause,
        position: usize,
    },
    MissingArrayType {
        ty: TyName,
    },
}

const COPY_SELECT_HINT: &str = "Try the COPY (SELECT ...) TO variant.";

fn copy_target(copy_from: bool, kind: TableKind) -> (&'static str, Option<&'static str>) {
    match (copy_from, kind) {
        (true, TableKind::View) => ("view", Some(COPY_SELECT_HINT)),
        (true, TableKind::ForeignTable) => ("foreign table", Some(COPY_SELECT_HINT)),
        (true, TableKind::Sequence) => ("sequence", None),
        (true, TableKind::Table | TableKind::MaterializedView | TableKind::PropertyGraph) => {
            ("non-table relation", None)
        }
        (false, TableKind::View) => (
            "view",
            Some("To enable copying to a view, provide an INSTEAD OF INSERT trigger."),
        ),
        (false, TableKind::MaterializedView) => ("materialized view", None),
        (false, TableKind::Sequence) => ("sequence", None),
        (false, TableKind::Table | TableKind::ForeignTable | TableKind::PropertyGraph) => {
            ("non-table relation", None)
        }
    }
}

pub(crate) fn reference_error(
    kind: ReferenceKind,
    resolution: Resolution,
    name: &str,
) -> Option<(DiagnosticKind, String)> {
    Some(match (kind, resolution) {
        (ReferenceKind::Relation, Resolution::Unresolved) => (
            DiagnosticKind::UnresolvedRelation,
            format!("missing FROM-clause entry for table `{name}`"),
        ),
        (ReferenceKind::Table, Resolution::Unresolved) => (
            DiagnosticKind::UnresolvedTable,
            format!("relation `{name}` does not exist"),
        ),
        (ReferenceKind::Column, Resolution::Unresolved) => (
            DiagnosticKind::UnresolvedColumn,
            format!("column `{name}` does not exist"),
        ),
        (ReferenceKind::Function(kind), Resolution::Unresolved) => (
            DiagnosticKind::UnresolvedFunction,
            format!("{} `{name}` does not exist", kind.label()),
        ),
        (ReferenceKind::Type, Resolution::Unresolved) => (
            DiagnosticKind::UnresolvedType,
            format!("type `{name}` does not exist"),
        ),
        (ReferenceKind::Window, Resolution::Unresolved) => (
            DiagnosticKind::UnresolvedWindow,
            format!("window `{name}` does not exist"),
        ),
        (ReferenceKind::JsonPath, Resolution::Unresolved) => (
            DiagnosticKind::UnresolvedJsonPath,
            format!("JSON path `{name}` does not exist"),
        ),
        (ReferenceKind::Index, Resolution::Unresolved) => (
            DiagnosticKind::UnresolvedIndex,
            format!("index `{name}` does not exist"),
        ),
        (ReferenceKind::Constraint, Resolution::Unresolved) => (
            DiagnosticKind::UnresolvedConstraint,
            format!("constraint `{name}` does not exist"),
        ),
        (ReferenceKind::Trigger, Resolution::Unresolved) => (
            DiagnosticKind::UnresolvedTrigger,
            format!("trigger `{name}` does not exist"),
        ),
        (ReferenceKind::Rule, Resolution::Unresolved) => (
            DiagnosticKind::UnresolvedRule,
            format!("rule `{name}` does not exist"),
        ),
        (ReferenceKind::Role, Resolution::Unresolved) => (
            DiagnosticKind::UnresolvedRole,
            format!("role `{name}` does not exist"),
        ),
        (ReferenceKind::Policy, Resolution::Unresolved) => (
            DiagnosticKind::UnresolvedPolicy,
            format!("policy `{name}` does not exist"),
        ),
        (ReferenceKind::AccessMethod, Resolution::Unresolved) => (
            DiagnosticKind::UnresolvedAccessMethod,
            format!("access method `{name}` does not exist"),
        ),
        (ReferenceKind::EventTrigger, Resolution::Unresolved) => (
            DiagnosticKind::UnresolvedEventTrigger,
            format!("event trigger `{name}` does not exist"),
        ),
        (ReferenceKind::Language, Resolution::Unresolved) => (
            DiagnosticKind::UnresolvedLanguage,
            format!("language `{name}` does not exist"),
        ),
        (ReferenceKind::ForeignDataWrapper, Resolution::Unresolved) => (
            DiagnosticKind::UnresolvedForeignDataWrapper,
            format!("foreign-data wrapper `{name}` does not exist"),
        ),
        (ReferenceKind::Server, Resolution::Unresolved) => (
            DiagnosticKind::UnresolvedServer,
            format!("server `{name}` does not exist"),
        ),
        (ReferenceKind::Conversion, Resolution::Unresolved) => (
            DiagnosticKind::UnresolvedConversion,
            format!("conversion `{name}` does not exist"),
        ),
        (ReferenceKind::OperatorFamily, Resolution::Unresolved) => (
            DiagnosticKind::UnresolvedOperatorFamily,
            format!("operator family `{name}` does not exist"),
        ),
        (ReferenceKind::OperatorClass, Resolution::Unresolved) => (
            DiagnosticKind::UnresolvedOperatorClass,
            format!("operator class `{name}` does not exist"),
        ),
        (ReferenceKind::Statistics, Resolution::Unresolved) => (
            DiagnosticKind::UnresolvedStatistics,
            format!("statistics object `{name}` does not exist"),
        ),
        (ReferenceKind::TextSearch(kind), Resolution::Unresolved) => (
            DiagnosticKind::UnresolvedTextSearch,
            format!("{} `{name}` does not exist", kind.label()),
        ),
        (ReferenceKind::Publication, Resolution::Unresolved) => (
            DiagnosticKind::UnresolvedPublication,
            format!("publication `{name}` does not exist"),
        ),
        (ReferenceKind::Subscription, Resolution::Unresolved) => (
            DiagnosticKind::UnresolvedSubscription,
            format!("subscription `{name}` does not exist"),
        ),
        (ReferenceKind::GraphElement, Resolution::Unresolved) => (
            DiagnosticKind::UnresolvedGraphElement,
            format!("element table `{name}` does not exist"),
        ),
        (ReferenceKind::GraphLabel, Resolution::Unresolved) => (
            DiagnosticKind::UnresolvedGraphLabel,
            format!("label `{name}` does not exist"),
        ),
        (ReferenceKind::EnumLabel, Resolution::Unresolved) => (
            DiagnosticKind::UnresolvedEnumLabel,
            format!("enum label `{name}` does not exist"),
        ),
        (ReferenceKind::GraphProperty, Resolution::Unresolved) => (
            DiagnosticKind::UnresolvedGraphProperty,
            format!("property `{name}` does not exist"),
        ),
        (ReferenceKind::Collation, Resolution::Unresolved) => (
            DiagnosticKind::UnresolvedCollation,
            format!("collation `{name}` does not exist"),
        ),
        (ReferenceKind::Schema, Resolution::Unresolved) => (
            DiagnosticKind::UnresolvedSchema,
            format!("schema `{name}` does not exist"),
        ),
        (ReferenceKind::Tablespace, Resolution::Unresolved) => (
            DiagnosticKind::UnresolvedTablespace,
            format!("tablespace `{name}` does not exist"),
        ),
        (ReferenceKind::Database, Resolution::Unresolved) => (
            DiagnosticKind::UnresolvedDatabase,
            format!("database `{name}` does not exist"),
        ),
        (ReferenceKind::Extension, Resolution::Unresolved) => (
            DiagnosticKind::UnresolvedExtension,
            format!("extension `{name}` does not exist"),
        ),
        (ReferenceKind::Operator, Resolution::Unresolved) => (
            DiagnosticKind::UnresolvedOperator,
            format!("operator `{name}` does not exist"),
        ),
        (ReferenceKind::Savepoint, Resolution::Unresolved) => (
            DiagnosticKind::UnresolvedSavepoint,
            format!("savepoint `{name}` does not exist"),
        ),
        (ReferenceKind::Cursor, Resolution::Unresolved) => (
            DiagnosticKind::UnresolvedCursor,
            format!("cursor `{name}` does not exist"),
        ),
        (ReferenceKind::PreparedTransaction, Resolution::Unresolved) => (
            DiagnosticKind::UnresolvedPreparedTransaction,
            format!("prepared transaction `{name}` does not exist"),
        ),
        (ReferenceKind::PreparedStatement, Resolution::Unresolved) => (
            DiagnosticKind::UnresolvedPreparedStatement,
            format!("prepared statement `{name}` does not exist"),
        ),
        (ReferenceKind::Channel, Resolution::Unresolved) => return None,
        (ReferenceKind::Param, Resolution::Unresolved) => (
            DiagnosticKind::UnresolvedParam,
            format!("there is no parameter {name}"),
        ),
        (ReferenceKind::Function(kind), Resolution::Ambiguous) => (
            DiagnosticKind::AmbiguousFunction,
            format!("{} `{name}` is not unique", kind.label()),
        ),
        (ReferenceKind::Operator, Resolution::Ambiguous) => (
            DiagnosticKind::AmbiguousOperator,
            format!("operator `{name}` is not unique"),
        ),
        (_, Resolution::Ambiguous) => (
            DiagnosticKind::AmbiguousColumn,
            format!("column reference `{name}` is ambiguous"),
        ),
        (
            _,
            Resolution::Column { .. }
            | Resolution::Source(_)
            | Resolution::Cte(_)
            | Resolution::Table(_)
            | Resolution::Function(_)
            | Resolution::Param(_)
            | Resolution::Type(_)
            | Resolution::Window(_)
            | Resolution::JsonPath(_)
            | Resolution::Index(_)
            | Resolution::Constraint(_)
            | Resolution::Trigger(_)
            | Resolution::Rule(_)
            | Resolution::Role(_)
            | Resolution::Policy(_)
            | Resolution::AccessMethod(_)
            | Resolution::EventTrigger(_)
            | Resolution::Language(_)
            | Resolution::ForeignDataWrapper(_)
            | Resolution::Server(_)
            | Resolution::Conversion(_)
            | Resolution::OperatorFamily(_)
            | Resolution::OperatorClass(_)
            | Resolution::Statistics(_)
            | Resolution::TextSearch(_)
            | Resolution::Publication(_)
            | Resolution::Subscription(_)
            | Resolution::GraphElement(_)
            | Resolution::GraphLabel(_)
            | Resolution::EnumLabel(_)
            | Resolution::GraphProperty(_)
            | Resolution::Collation(_)
            | Resolution::Schema(_)
            | Resolution::Tablespace(_)
            | Resolution::Database(_)
            | Resolution::Extension(_)
            | Resolution::Operator(_)
            | Resolution::Savepoint(_)
            | Resolution::Cursor(_)
            | Resolution::PreparedTransaction(_)
            | Resolution::PreparedStatement(_)
            | Resolution::Channel(_)
            | Resolution::Unknown,
        ) => return None,
    })
}

impl RoutineKind {
    fn label(self) -> &'static str {
        match self {
            RoutineKind::Function => "function",
            RoutineKind::Aggregate => "aggregate",
            RoutineKind::Procedure => "procedure",
            RoutineKind::Routine => "routine",
        }
    }
}

impl ExprContext {
    fn label(self) -> &'static str {
        match self {
            ExprContext::Unrestricted => "",
            ExprContext::Where => "WHERE",
            ExprContext::JoinCondition => "JOIN conditions",
            ExprContext::GroupBy => "GROUP BY",
            ExprContext::Having => "HAVING",
            ExprContext::Filter => "FILTER",
            ExprContext::WindowDefinition => "window definitions",
            ExprContext::WindowRange => "window RANGE",
            ExprContext::WindowRows => "window ROWS",
            ExprContext::WindowGroups => "window GROUPS",
            ExprContext::Limit => "LIMIT",
            ExprContext::Offset => "OFFSET",
            ExprContext::Values => "VALUES",
            ExprContext::Returning => "RETURNING",
            ExprContext::Insert => "INSERT",
            ExprContext::Update => "UPDATE",
            ExprContext::FromFunction => "functions in FROM",
            ExprContext::MergeWhen => "MERGE WHEN conditions",
            ExprContext::CheckConstraint => "check constraints",
            ExprContext::Default => "DEFAULT expressions",
            ExprContext::GeneratedColumn => "column generation expressions",
            ExprContext::IndexExpression => "index expressions",
            ExprContext::IndexPredicate => "index predicates",
            ExprContext::StatisticsExpression => "statistics expressions",
            ExprContext::PartitionKey => "partition key expressions",
            ExprContext::PartitionBound => "partition bound",
            ExprContext::Policy => "policy expressions",
            ExprContext::TriggerWhen => "trigger WHEN conditions",
            ExprContext::CallArgument => "CALL arguments",
            ExprContext::Transform => "transform expressions",
            ExprContext::CopyWhere => "COPY FROM WHERE conditions",
            ExprContext::ExecuteParameter => "EXECUTE parameters",
        }
    }
}

impl TypeKind {
    fn label(self) -> &'static str {
        match self {
            TypeKind::Shell => "shell type",
            TypeKind::Type => "type",
            TypeKind::Domain => "domain",
            TypeKind::Enum => "enum",
            TypeKind::Range => "range type",
            TypeKind::Multirange => "multirange type",
        }
    }
}

impl TableLookup {
    fn label(self) -> &'static str {
        match self {
            TableLookup::Table => "table",
            TableLookup::ForeignTable => "foreign table",
            TableLookup::TableOrForeignTable => "table or foreign table",
            TableLookup::TableOrView => "table or view",
            TableLookup::TableOrMaterializedView => "table or materialized view",
            TableLookup::Any => "relation",
            TableLookup::View => "view",
            TableLookup::MaterializedView => "materialized view",
            TableLookup::Sequence => "sequence",
            TableLookup::PropertyGraph => "property graph",
            TableLookup::CopyTo => "table or materialized view",
            TableLookup::CopyFrom => "table or foreign table",
            TableLookup::Truncate => "table",
        }
    }
}

impl OutputClause {
    fn label(self) -> &'static str {
        match self {
            OutputClause::OrderBy => "ORDER BY",
            OutputClause::GroupBy => "GROUP BY",
            OutputClause::DistinctOn => "DISTINCT ON",
        }
    }
}

fn drop_hint(kind: TableKind) -> &'static str {
    match kind {
        TableKind::Table => "Use DROP TABLE to remove a table.",
        TableKind::ForeignTable => "Use DROP FOREIGN TABLE to remove a foreign table.",
        TableKind::View => "Use DROP VIEW to remove a view.",
        TableKind::MaterializedView => "Use DROP MATERIALIZED VIEW to remove a materialized view.",
        TableKind::Sequence => "Use DROP SEQUENCE to remove a sequence.",
        TableKind::PropertyGraph => "Use DROP PROPERTY GRAPH to remove a property graph.",
    }
}

fn plural_columns(count: usize) -> &'static str {
    if count == 1 { "column" } else { "columns" }
}

impl SemaError {
    pub(crate) fn kind(&self) -> DiagnosticKind {
        match *self {
            SemaError::CannotChangeDatabase => DiagnosticKind::CannotChangeDatabase,
            SemaError::ValuesLengthMismatch
            | SemaError::InsertMoreExpressionsThanTargets
            | SemaError::ViewTooManyColumnNames
            | SemaError::CreateAsTooManyColumnNames => DiagnosticKind::ColumnCountMismatch,
            SemaError::ReplaceConstraintTrigger => DiagnosticKind::ReplaceConstraintTrigger,
            SemaError::RenameSelectRule => DiagnosticKind::RenameSelectRule,
            SemaError::NestedAggregate
            | SemaError::AggregateContainsWindow
            | SemaError::MisplacedAggregate { .. } => DiagnosticKind::GroupingError,
            SemaError::NestedWindow | SemaError::MisplacedWindow { .. } => {
                DiagnosticKind::WindowingError
            }
            SemaError::Duplicate { object, .. }
            | SemaError::DuplicateInSchema { object, .. }
            | SemaError::DuplicateForAccessMethod { object, .. } => object.duplicate_kind(),
            SemaError::ColumnSpecifiedMoreThanOnce { .. } => DiagnosticKind::DuplicateColumn,
            SemaError::DuplicateCte { .. } => DiagnosticKind::DuplicateCte,
            SemaError::DuplicateSource { .. } => DiagnosticKind::DuplicateSource,
            SemaError::DuplicatePolicy { .. } => DiagnosticKind::DuplicatePolicy,
            SemaError::DuplicateTrigger { .. } => DiagnosticKind::DuplicateTrigger,
            SemaError::DuplicateRule { .. } => DiagnosticKind::DuplicateRule,
            SemaError::DuplicateConstraint { .. } => DiagnosticKind::DuplicateConstraint,
            SemaError::DuplicateColumn { .. } => DiagnosticKind::DuplicateColumn,
            SemaError::DuplicateFunction { .. } => DiagnosticKind::DuplicateFunction,
            SemaError::DuplicateTransform { .. } => DiagnosticKind::DuplicateTransform,
            SemaError::DuplicateAssignment { .. } => DiagnosticKind::DuplicateAssignment,
            SemaError::DuplicateWindow { .. } => DiagnosticKind::DuplicateWindow,
            SemaError::DuplicateParam { .. } => DiagnosticKind::DuplicateParam,
            SemaError::DependentObjects { .. } => DiagnosticKind::DependentObjectsStillExist,
            SemaError::RequiredBySystem { .. } => DiagnosticKind::RequiredBySystem,
            SemaError::AccessMethodKindMismatch { .. } => DiagnosticKind::AccessMethodKindMismatch,
            SemaError::UnresolvedCast { .. } => DiagnosticKind::UnresolvedCast,
            SemaError::UnresolvedTransform { .. } => DiagnosticKind::UnresolvedTransform,
            SemaError::InvalidEnumValue { .. } => DiagnosticKind::UnresolvedEnumLabel,
            SemaError::NotEnum { .. }
            | SemaError::NotComposite { .. }
            | SemaError::WrongTypeKind { .. } => DiagnosticKind::WrongTypeKind,
            SemaError::TypedTableAlter { .. } => DiagnosticKind::TypedTableAlter,
            SemaError::InvalidDomainConstraint { .. } => DiagnosticKind::InvalidDomainConstraint,
            SemaError::CompoundColumnCount { .. }
            | SemaError::ColumnListTooLong { .. }
            | SemaError::InsertMoreTargetsThanExpressions { .. } => {
                DiagnosticKind::ColumnCountMismatch
            }
            SemaError::MissingShellType => DiagnosticKind::MissingShellType,
            SemaError::WrongRelationKind { .. }
            | SemaError::CannotCopy { .. }
            | SemaError::CannotCreateIndex { .. } => DiagnosticKind::WrongRelationKind,
            SemaError::NotIdentityColumn { .. } => DiagnosticKind::NotIdentityColumn,
            SemaError::ConstraintTrigger { .. } => DiagnosticKind::ReplaceConstraintTrigger,
            SemaError::DropReturnRule { .. } => DiagnosticKind::DropReturnRule,
            SemaError::MultiplePrimaryKeys { .. } => DiagnosticKind::MultiplePrimaryKeys,
            SemaError::InvalidOrdinal { .. } => DiagnosticKind::InvalidOrdinal,
            SemaError::MissingArrayType { .. } => DiagnosticKind::MissingArrayType,
        }
    }

    pub(crate) fn hint(&self) -> Option<&'static str> {
        match *self {
            SemaError::TypedTableAlter { .. } => {
                Some("Use ALTER ... CASCADE to alter the typed tables too.")
            }
            SemaError::InsertMoreTargetsThanExpressions { row_hint } => row_hint.then_some(
                "The insertion source is a row expression containing the same number of columns expected by the INSERT. Did you accidentally use extra parentheses?",
            ),
            SemaError::MissingShellType => Some(
                "Create the type as a shell type, then create its canonicalization function, then do a full CREATE TYPE.",
            ),
            SemaError::WrongRelationKind { kind, drop, .. } => drop.then(|| drop_hint(kind)),
            SemaError::CannotCopy {
                copy_from, kind, ..
            } => copy_target(copy_from, kind).1,
            _ => None,
        }
    }

    fn static_message(&self) -> Option<&'static str> {
        match *self {
            SemaError::CannotChangeDatabase => Some("current database cannot be changed"),
            SemaError::ValuesLengthMismatch => Some("VALUES lists must all be the same length"),
            SemaError::InsertMoreExpressionsThanTargets => {
                Some("INSERT has more expressions than target columns")
            }
            SemaError::ViewTooManyColumnNames => {
                Some("CREATE VIEW specifies more column names than columns")
            }
            SemaError::CreateAsTooManyColumnNames => Some("too many column names were specified"),
            SemaError::ReplaceConstraintTrigger => {
                Some("CREATE OR REPLACE CONSTRAINT TRIGGER is not supported")
            }
            SemaError::RenameSelectRule => Some("renaming an ON SELECT rule is not allowed"),
            SemaError::NestedAggregate => Some("aggregate function calls cannot be nested"),
            SemaError::AggregateContainsWindow => {
                Some("aggregate function calls cannot contain window function calls")
            }
            SemaError::NestedWindow => Some("window function calls cannot be nested"),
            SemaError::InsertMoreTargetsThanExpressions { .. } => {
                Some("INSERT has more target columns than expressions")
            }
            SemaError::MissingShellType => {
                Some("cannot specify a canonical function without a pre-created shell type")
            }
            _ => None,
        }
    }

    pub(crate) fn message(&self, names: &Interner) -> Cow<'static, str> {
        match self.static_message() {
            Some(message) => Cow::Borrowed(message),
            None => Cow::Owned(ErrorMessage { error: self, names }.to_string()),
        }
    }
}

struct ErrorMessage<'a> {
    error: &'a SemaError,
    names: &'a Interner,
}

impl ErrorMessage<'_> {
    fn ty(&self, ty: TyName) -> impl fmt::Display + '_ {
        fmt::from_fn(move |f| {
            let suffix = if ty.array { "[]" } else { "" };
            write!(f, "{}{suffix}", self.names.resolve(ty.name))
        })
    }
}

impl fmt::Display for ErrorMessage<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let n = |name| self.names.resolve(name);
        match *self.error {
            SemaError::CannotChangeDatabase
            | SemaError::ValuesLengthMismatch
            | SemaError::InsertMoreExpressionsThanTargets
            | SemaError::ViewTooManyColumnNames
            | SemaError::CreateAsTooManyColumnNames
            | SemaError::ReplaceConstraintTrigger
            | SemaError::RenameSelectRule
            | SemaError::NestedAggregate
            | SemaError::AggregateContainsWindow
            | SemaError::NestedWindow
            | SemaError::InsertMoreTargetsThanExpressions { .. }
            | SemaError::MissingShellType => {
                f.write_str(self.error.static_message().unwrap_or_default())
            }
            SemaError::MisplacedAggregate { context } => write!(
                f,
                "aggregate functions are not allowed in {}",
                context.label()
            ),
            SemaError::MisplacedWindow { context } => {
                let label = match context {
                    ExprContext::WindowRange
                    | ExprContext::WindowRows
                    | ExprContext::WindowGroups => ExprContext::WindowDefinition.label(),
                    context => context.label(),
                };
                write!(f, "window functions are not allowed in {label}")
            }
            SemaError::Duplicate { object, name } => {
                write!(f, "{} `{}` already exists", object.label(), n(name))
            }
            SemaError::DuplicateInSchema {
                object,
                name,
                schema,
            } => write!(
                f,
                "{} `{}` already exists in schema `{}`",
                object.label(),
                n(name),
                n(schema)
            ),
            SemaError::DuplicateForAccessMethod {
                object,
                name,
                access_method,
            } => {
                write!(f, "{} `{}`", object.label(), n(name))?;
                if let Some(access_method) = access_method {
                    write!(f, " for access method `{}`", n(access_method))?;
                }
                f.write_str(" already exists")
            }
            SemaError::DuplicatePolicy { name, table } => write!(
                f,
                "policy `{}` for table `{}` already exists",
                n(name),
                n(table)
            ),
            SemaError::DuplicateTrigger { name, table } => write!(
                f,
                "trigger `{}` for relation `{}` already exists",
                n(name),
                n(table)
            ),
            SemaError::DuplicateRule { name, table } => write!(
                f,
                "rule `{}` for relation `{}` already exists",
                n(name),
                n(table)
            ),
            SemaError::DuplicateConstraint {
                name,
                owner,
                domain,
            } => {
                let what = if domain { "domain" } else { "relation" };
                write!(
                    f,
                    "constraint `{}` for {what} `{}` already exists",
                    n(name),
                    n(owner)
                )
            }
            SemaError::DuplicateColumn { name, relation } => write!(
                f,
                "column `{}` of relation `{}` already exists",
                n(name),
                n(relation)
            ),
            SemaError::DuplicateFunction { name, schema } => {
                write!(
                    f,
                    "function `{}` already exists with same argument types",
                    n(name)
                )?;
                if let Some(schema) = schema {
                    write!(f, " in schema `{}`", n(schema))?;
                }
                Ok(())
            }
            SemaError::DuplicateTransform { ty, language } => write!(
                f,
                "transform for type `{}` language `{}` already exists",
                self.ty(ty),
                n(language)
            ),
            SemaError::ColumnSpecifiedMoreThanOnce { name } => {
                write!(f, "column `{}` specified more than once", n(name))
            }
            SemaError::DuplicateCte { name } => {
                write!(f, "WITH query name `{}` specified more than once", n(name))
            }
            SemaError::DuplicateSource { name } => {
                write!(f, "table name `{}` specified more than once", n(name))
            }
            SemaError::DuplicateAssignment { name } => {
                write!(f, "multiple assignments to same column `{}`", n(name))
            }
            SemaError::DuplicateWindow { name } => {
                write!(f, "window `{}` is already defined", n(name))
            }
            SemaError::DuplicateParam { name } => {
                write!(f, "parameter name `{}` used more than once", n(name))
            }
            SemaError::DependentObjects { object, name } => write!(
                f,
                "cannot drop {} `{}` because other objects depend on it",
                object.label(),
                n(name)
            ),
            SemaError::RequiredBySystem { schema } => write!(
                f,
                "cannot drop schema `{}` because it is required by the database system",
                n(schema)
            ),
            SemaError::AccessMethodKindMismatch { name, expected } => {
                let expected = match expected {
                    AccessMethodKind::Table => "TABLE",
                    AccessMethodKind::Index => "INDEX",
                };
                write!(f, "access method `{}` is not of type {expected}", n(name))
            }
            SemaError::UnresolvedCast { source, target } => write!(
                f,
                "cast from type `{}` to type `{}` does not exist",
                self.ty(source),
                self.ty(target)
            ),
            SemaError::UnresolvedTransform { ty, language } => write!(
                f,
                "transform for type `{}` language `{}` does not exist",
                self.ty(ty),
                n(language)
            ),
            SemaError::InvalidEnumValue { ty, value } => write!(
                f,
                "invalid input value for enum `{}`: `{}`",
                n(ty),
                n(value)
            ),
            SemaError::NotEnum { name } => write!(f, "`{}` is not an enum", n(name)),
            SemaError::NotComposite { name } => write!(f, "`{}` is not a composite type", n(name)),
            SemaError::WrongTypeKind { name, expected } => {
                write!(f, "`{}` is not a {}", n(name), expected.label())
            }
            SemaError::TypedTableAlter { name } => write!(
                f,
                "cannot alter type `{}` because it is the type of a typed table",
                n(name)
            ),
            SemaError::InvalidDomainConstraint { what } => {
                write!(f, "{what} not possible for domains")
            }
            SemaError::CompoundColumnCount { op } => {
                write!(f, "each {op} query must have the same number of columns")
            }
            SemaError::ColumnListTooLong {
                what,
                name,
                available,
                specified,
            } => write!(
                f,
                "{what} `{}` has {available} {} available but {specified} {} specified",
                n(name),
                plural_columns(available),
                plural_columns(specified),
            ),
            SemaError::WrongRelationKind { name, expected, .. } => {
                write!(f, "`{}` is not a {}", n(name), expected.label())
            }
            SemaError::CannotCopy {
                name,
                copy_from,
                kind,
            } => {
                let direction = if copy_from { "from" } else { "to" };
                let what = copy_target(copy_from, kind).0;
                write!(f, "cannot copy {direction} {what} `{}`", n(name))
            }
            SemaError::CannotCreateIndex { name, kind } => {
                write!(f, "cannot create index on {} `{}`", kind.label(), n(name))
            }
            SemaError::NotIdentityColumn { column, relation } => write!(
                f,
                "column `{}` of relation `{}` is not an identity column",
                n(column),
                n(relation)
            ),
            SemaError::ConstraintTrigger { name, relation } => write!(
                f,
                "trigger `{}` for relation `{}` is a constraint trigger",
                n(name),
                n(relation)
            ),
            SemaError::DropReturnRule { name, kind, table } => {
                let what = kind.label();
                let table = n(table);
                write!(
                    f,
                    "cannot drop rule `{}` on {what} `{table}` because {what} `{table}` requires it",
                    n(name)
                )
            }
            SemaError::MultiplePrimaryKeys { table } => write!(
                f,
                "multiple primary keys for table `{}` are not allowed",
                n(table)
            ),
            SemaError::InvalidOrdinal { clause, position } => {
                write!(
                    f,
                    "{} position {position} is not in select list",
                    clause.label()
                )
            }
            SemaError::MissingArrayType { ty } => write!(
                f,
                "could not find array type for data type `{}`",
                self.ty(ty)
            ),
        }
    }
}
