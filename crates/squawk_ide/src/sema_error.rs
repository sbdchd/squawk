use std::borrow::Cow;
use std::fmt;

use crate::arena::{IdxRange, LayeredArena};
use crate::interner::{Interner, Name};
use crate::ir::{
    AccessMethodKind, ReferenceKind, Resolution, RoutineKind, TableKind, TextSearchKind, TypeKind,
};
use rowan::{TextRange, TextSize};
use squawk_linter::{Edit, Fix};
use squawk_syntax::quote::{quote_ident, quote_string_literal};

use crate::lower::{ExprContext, OutputClause, TableLookup};
use crate::semantic_index::{DiagnosticKind, Suggestion};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SimilarColumn {
    pub(crate) qualifier: Option<Name>,
    pub(crate) column: Name,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SimilarType {
    Named(Name),
    Keyword(&'static str),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Similar {
    Column(SimilarColumn),
    Table(Name),
    Type(SimilarType),
    Window(Name),
    EnumLabel(Name),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct TyName {
    pub(crate) name: Name,
    pub(crate) array: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OperandName {
    Ty(TyName),
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LookupKind {
    Call,
    Signature,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ErrorArgs {
    List(IdxRange<ErrorArg>),
    Star,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ErrorArg {
    pub(crate) name: Option<Name>,
    pub(crate) ty: OperandName,
    pub(crate) variadic: bool,
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
    WindowWithoutOver {
        name: Name,
    },
    MisplacedAggregate {
        context: ExprContext,
    },
    MisplacedWindow {
        context: ExprContext,
    },
    MisplacedSubquery {
        context: ExprContext,
    },
    DefaultColumnReference,
    GeneratedColumnReference {
        column: Name,
    },
    SubqueryNotSingleColumn,
    SubqueryTooManyColumns,
    SubqueryTooFewColumns,
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
    UnresolvedQualifiedColumn {
        qualifier: Name,
        column: Name,
        similar: Option<Name>,
    },
    UnresolvedReference {
        kind: ReferenceKind,
        name: Name,
        similar: Option<Similar>,
    },
    AmbiguousReference {
        kind: ReferenceKind,
        name: Name,
    },
    NotLateral {
        table: Name,
        column: Option<Name>,
        lateral_at: TextSize,
    },
    UnresolvedRoutine {
        kind: RoutineKind,
        schema: Option<Name>,
        name: Name,
        args: ErrorArgs,
        lookup: LookupKind,
        similar: Option<Name>,
    },
    UnresolvedOperator {
        schema: Option<Name>,
        name: Name,
        left: Option<OperandName>,
        right: OperandName,
        lookup: LookupKind,
    },
    InvalidEnumValue {
        ty: Name,
        value: Name,
        similar: Option<Name>,
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
    NoPrimaryKeyForReferencedTable {
        table: Name,
    },
    NoUniqueConstraintForReferencedTable {
        table: Name,
    },
    DeferrablePrimaryKeyForReferencedTable {
        table: Name,
    },
    DeferrableUniqueConstraintForReferencedTable {
        table: Name,
    },
    NoConflictArbiter,
    DeferrableConflictArbiter,
    RefreshConcurrentlyWithoutUniqueIndex {
        name: Name,
    },
    UniqueIndexNotSupported {
        access_method: Name,
    },
    InvalidOrdinal {
        clause: OutputClause,
        position: usize,
    },
    MissingArrayType {
        ty: TyName,
    },
    WindowOverride {
        name: Name,
        clause: WindowClause,
    },
    WindowFrameCopy {
        name: Name,
        hint: bool,
    },
    WindowAggregateOrderBy,
    WindowDistinct,
    TriggerReturnType {
        name: Name,
        kind: TriggerFunctionKind,
    },
    ConflictingNullability {
        column: Name,
        table: Name,
    },
    RowLengthMismatch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum WindowClause {
    PartitionBy,
    OrderBy,
}

impl WindowClause {
    fn label(self) -> &'static str {
        match self {
            WindowClause::PartitionBy => "`partition by`",
            WindowClause::OrderBy => "`order by`",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TriggerFunctionKind {
    Trigger,
    EventTrigger,
}

impl TriggerFunctionKind {
    pub(crate) fn return_type(self) -> &'static str {
        match self {
            TriggerFunctionKind::Trigger => "trigger",
            TriggerFunctionKind::EventTrigger => "event_trigger",
        }
    }
}

const COPY_SELECT_HINT: &str = "try the `copy (select ...) to` variant";

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
            Some("to enable copying to a view, provide an `instead of insert` trigger"),
        ),
        (false, TableKind::MaterializedView) => ("materialized view", None),
        (false, TableKind::Sequence) => ("sequence", None),
        (false, TableKind::Table | TableKind::ForeignTable | TableKind::PropertyGraph) => {
            ("non-table relation", None)
        }
    }
}

fn unresolved_kind(kind: ReferenceKind) -> DiagnosticKind {
    match kind {
        ReferenceKind::Relation => DiagnosticKind::UnresolvedRelation,
        ReferenceKind::Table => DiagnosticKind::UnresolvedTable,
        ReferenceKind::Column => DiagnosticKind::UnresolvedColumn,
        ReferenceKind::Function(_) => DiagnosticKind::UnresolvedFunction,
        ReferenceKind::Type => DiagnosticKind::UnresolvedType,
        ReferenceKind::Window => DiagnosticKind::UnresolvedWindow,
        ReferenceKind::JsonPath => DiagnosticKind::UnresolvedJsonPath,
        ReferenceKind::Index => DiagnosticKind::UnresolvedIndex,
        ReferenceKind::Constraint => DiagnosticKind::UnresolvedConstraint,
        ReferenceKind::Trigger => DiagnosticKind::UnresolvedTrigger,
        ReferenceKind::Rule => DiagnosticKind::UnresolvedRule,
        ReferenceKind::Role => DiagnosticKind::UnresolvedRole,
        ReferenceKind::Policy => DiagnosticKind::UnresolvedPolicy,
        ReferenceKind::AccessMethod => DiagnosticKind::UnresolvedAccessMethod,
        ReferenceKind::EventTrigger => DiagnosticKind::UnresolvedEventTrigger,
        ReferenceKind::Language => DiagnosticKind::UnresolvedLanguage,
        ReferenceKind::ForeignDataWrapper => DiagnosticKind::UnresolvedForeignDataWrapper,
        ReferenceKind::Server => DiagnosticKind::UnresolvedServer,
        ReferenceKind::Conversion => DiagnosticKind::UnresolvedConversion,
        ReferenceKind::OperatorFamily => DiagnosticKind::UnresolvedOperatorFamily,
        ReferenceKind::OperatorClass => DiagnosticKind::UnresolvedOperatorClass,
        ReferenceKind::Statistics => DiagnosticKind::UnresolvedStatistics,
        ReferenceKind::TextSearch(_) => DiagnosticKind::UnresolvedTextSearch,
        ReferenceKind::Publication => DiagnosticKind::UnresolvedPublication,
        ReferenceKind::Subscription => DiagnosticKind::UnresolvedSubscription,
        ReferenceKind::GraphElement => DiagnosticKind::UnresolvedGraphElement,
        ReferenceKind::GraphLabel => DiagnosticKind::UnresolvedGraphLabel,
        ReferenceKind::EnumLabel => DiagnosticKind::UnresolvedEnumLabel,
        ReferenceKind::GraphProperty => DiagnosticKind::UnresolvedGraphProperty,
        ReferenceKind::Collation => DiagnosticKind::UnresolvedCollation,
        ReferenceKind::Schema => DiagnosticKind::UnresolvedSchema,
        ReferenceKind::Tablespace => DiagnosticKind::UnresolvedTablespace,
        ReferenceKind::Database => DiagnosticKind::UnresolvedDatabase,
        ReferenceKind::Extension => DiagnosticKind::UnresolvedExtension,
        ReferenceKind::Operator => DiagnosticKind::UnresolvedOperator,
        ReferenceKind::Savepoint => DiagnosticKind::UnresolvedSavepoint,
        ReferenceKind::Cursor => DiagnosticKind::UnresolvedCursor,
        ReferenceKind::PreparedTransaction => DiagnosticKind::UnresolvedPreparedTransaction,
        ReferenceKind::PreparedStatement => DiagnosticKind::UnresolvedPreparedStatement,
        ReferenceKind::Param => DiagnosticKind::UnresolvedParam,
        ReferenceKind::Channel => unreachable!(),
    }
}

fn ambiguous_kind(kind: ReferenceKind) -> DiagnosticKind {
    match kind {
        ReferenceKind::Function(_) => DiagnosticKind::AmbiguousFunction,
        ReferenceKind::Operator => DiagnosticKind::AmbiguousOperator,
        ReferenceKind::Column
        | ReferenceKind::Relation
        | ReferenceKind::Table
        | ReferenceKind::Param
        | ReferenceKind::Type
        | ReferenceKind::Window
        | ReferenceKind::JsonPath
        | ReferenceKind::Index
        | ReferenceKind::Constraint
        | ReferenceKind::Trigger
        | ReferenceKind::Rule
        | ReferenceKind::Role
        | ReferenceKind::Policy
        | ReferenceKind::AccessMethod
        | ReferenceKind::EventTrigger
        | ReferenceKind::Language
        | ReferenceKind::ForeignDataWrapper
        | ReferenceKind::Server
        | ReferenceKind::Conversion
        | ReferenceKind::OperatorFamily
        | ReferenceKind::OperatorClass
        | ReferenceKind::Statistics
        | ReferenceKind::TextSearch(_)
        | ReferenceKind::Publication
        | ReferenceKind::Subscription
        | ReferenceKind::GraphElement
        | ReferenceKind::GraphLabel
        | ReferenceKind::EnumLabel
        | ReferenceKind::GraphProperty
        | ReferenceKind::Collation
        | ReferenceKind::Schema
        | ReferenceKind::Tablespace
        | ReferenceKind::Database
        | ReferenceKind::Extension
        | ReferenceKind::Savepoint
        | ReferenceKind::Cursor
        | ReferenceKind::PreparedTransaction
        | ReferenceKind::PreparedStatement
        | ReferenceKind::Channel => DiagnosticKind::AmbiguousColumn,
    }
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
            ExprContext::Where => "`where`",
            ExprContext::JoinCondition => "`join` conditions",
            ExprContext::GroupBy => "`group by`",
            ExprContext::Having => "`having`",
            ExprContext::Filter => "`filter`",
            ExprContext::WindowDefinition => "window definitions",
            ExprContext::WindowRange => "window `range`",
            ExprContext::WindowRows => "window `rows`",
            ExprContext::WindowGroups => "window `groups`",
            ExprContext::Limit => "`limit`",
            ExprContext::Offset => "`offset`",
            ExprContext::Values => "`values`",
            ExprContext::Returning => "`returning`",
            ExprContext::Insert => "`insert`",
            ExprContext::Update => "`update`",
            ExprContext::FromFunction => "functions in `from`",
            ExprContext::MergeWhen => "`merge when` conditions",
            ExprContext::CheckConstraint => "check constraints",
            ExprContext::Default => "`default` expressions",
            ExprContext::GeneratedColumn => "column generation expressions",
            ExprContext::IndexExpression => "index expressions",
            ExprContext::IndexPredicate => "index predicates",
            ExprContext::StatisticsExpression => "statistics expressions",
            ExprContext::PartitionKey => "partition key expressions",
            ExprContext::PartitionBound => "partition bound",
            ExprContext::Policy => "policy expressions",
            ExprContext::TriggerWhen => "trigger `when` conditions",
            ExprContext::CallArgument => "`call` arguments",
            ExprContext::Transform => "transform expressions",
            ExprContext::CopyWhere => "`copy from where` conditions",
            ExprContext::ExecuteParameter => "`execute` parameters",
        }
    }

    pub(crate) fn subquery_label(self) -> Option<&'static str> {
        match self {
            ExprContext::CheckConstraint => Some("check constraint"),
            ExprContext::Default => Some("`default` expression"),
            ExprContext::GeneratedColumn => Some("column generation expression"),
            ExprContext::IndexExpression => Some("index expression"),
            ExprContext::IndexPredicate => Some("index predicate"),
            ExprContext::StatisticsExpression => Some("statistics expression"),
            ExprContext::PartitionKey => Some("partition key expression"),
            ExprContext::PartitionBound => Some("partition bound"),
            ExprContext::TriggerWhen => Some("trigger `when` condition"),
            ExprContext::CallArgument => Some("`call` argument"),
            ExprContext::Transform => Some("transform expression"),
            ExprContext::CopyWhere => Some("`copy from where` condition"),
            ExprContext::ExecuteParameter => Some("`execute` parameter"),
            ExprContext::Unrestricted
            | ExprContext::Where
            | ExprContext::JoinCondition
            | ExprContext::GroupBy
            | ExprContext::Having
            | ExprContext::Filter
            | ExprContext::WindowDefinition
            | ExprContext::WindowRange
            | ExprContext::WindowRows
            | ExprContext::WindowGroups
            | ExprContext::Limit
            | ExprContext::Offset
            | ExprContext::Values
            | ExprContext::Returning
            | ExprContext::Insert
            | ExprContext::Update
            | ExprContext::FromFunction
            | ExprContext::MergeWhen
            | ExprContext::Policy => None,
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
            TableLookup::Any | TableLookup::FromItem => "relation",
            TableLookup::Like => "table, view, materialized view, composite type, or foreign table",
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
            OutputClause::OrderBy => "`order by`",
            OutputClause::GroupBy => "`group by`",
            OutputClause::DistinctOn => "`distinct on`",
        }
    }
}

fn drop_hint(kind: TableKind) -> &'static str {
    match kind {
        TableKind::Table => "use `drop table` to remove a table",
        TableKind::ForeignTable => "use `drop foreign table` to remove a foreign table",
        TableKind::View => "use `drop view` to remove a view",
        TableKind::MaterializedView => "use `drop materialized view` to remove a materialized view",
        TableKind::Sequence => "use `drop sequence` to remove a sequence",
        TableKind::PropertyGraph => "use `drop property graph` to remove a property graph",
    }
}

fn replace_suggestion(message: &'static str, range: TextRange, replacement: String) -> Suggestion {
    Suggestion {
        message: message.into(),
        fix: Fix {
            title: format!("Replace with `{replacement}`").into(),
            edits: vec![Edit::replace(range, replacement)],
        },
    }
}

fn plural_columns(count: usize) -> &'static str {
    if count == 1 { "column" } else { "columns" }
}

impl SemaError {
    pub(crate) fn reference(
        kind: ReferenceKind,
        resolution: Resolution,
        name: Name,
    ) -> Option<SemaError> {
        match resolution {
            Resolution::Unresolved if kind == ReferenceKind::Channel => None,
            Resolution::Unresolved => Some(SemaError::UnresolvedReference {
                kind,
                name,
                similar: None,
            }),
            Resolution::Ambiguous => Some(SemaError::AmbiguousReference { kind, name }),
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
            | Resolution::Unknown => None,
        }
    }

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
            SemaError::MisplacedSubquery { .. } | SemaError::DefaultColumnReference => {
                DiagnosticKind::FeatureNotSupported
            }
            SemaError::GeneratedColumnReference { .. } => DiagnosticKind::InvalidColumnReference,
            SemaError::NestedWindow
            | SemaError::WindowWithoutOver { .. }
            | SemaError::MisplacedWindow { .. }
            | SemaError::WindowOverride { .. }
            | SemaError::WindowFrameCopy { .. }
            | SemaError::WindowAggregateOrderBy
            | SemaError::WindowDistinct => DiagnosticKind::WindowingError,
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
            SemaError::UnresolvedOperator { .. } => DiagnosticKind::UnresolvedOperator,
            SemaError::UnresolvedRoutine { .. } => DiagnosticKind::UnresolvedFunction,
            SemaError::UnresolvedQualifiedColumn { .. } => DiagnosticKind::UnresolvedColumn,
            SemaError::UnresolvedReference { kind, .. } => unresolved_kind(kind),
            SemaError::AmbiguousReference { kind, .. } => ambiguous_kind(kind),
            SemaError::NotLateral {
                column: Some(_), ..
            } => DiagnosticKind::UnresolvedColumn,
            SemaError::NotLateral { column: None, .. } => DiagnosticKind::UnresolvedRelation,
            SemaError::InvalidEnumValue { .. } => DiagnosticKind::UnresolvedEnumLabel,
            SemaError::NotEnum { .. }
            | SemaError::NotComposite { .. }
            | SemaError::WrongTypeKind { .. } => DiagnosticKind::WrongTypeKind,
            SemaError::TypedTableAlter { .. } => DiagnosticKind::TypedTableAlter,
            SemaError::InvalidDomainConstraint { .. } => DiagnosticKind::InvalidDomainConstraint,
            SemaError::CompoundColumnCount { .. }
            | SemaError::ColumnListTooLong { .. }
            | SemaError::InsertMoreTargetsThanExpressions { .. }
            | SemaError::RowLengthMismatch
            | SemaError::SubqueryNotSingleColumn
            | SemaError::SubqueryTooManyColumns
            | SemaError::SubqueryTooFewColumns => DiagnosticKind::ColumnCountMismatch,
            SemaError::MissingShellType => DiagnosticKind::MissingShellType,
            SemaError::WrongRelationKind { .. }
            | SemaError::CannotCopy { .. }
            | SemaError::CannotCreateIndex { .. } => DiagnosticKind::WrongRelationKind,
            SemaError::NotIdentityColumn { .. } => DiagnosticKind::NotIdentityColumn,
            SemaError::ConstraintTrigger { .. } => DiagnosticKind::ReplaceConstraintTrigger,
            SemaError::DropReturnRule { .. } => DiagnosticKind::DropReturnRule,
            SemaError::MultiplePrimaryKeys { .. } => DiagnosticKind::MultiplePrimaryKeys,
            SemaError::NoPrimaryKeyForReferencedTable { .. }
            | SemaError::NoUniqueConstraintForReferencedTable { .. }
            | SemaError::DeferrablePrimaryKeyForReferencedTable { .. }
            | SemaError::DeferrableUniqueConstraintForReferencedTable { .. } => {
                DiagnosticKind::InvalidForeignKey
            }
            SemaError::NoConflictArbiter | SemaError::DeferrableConflictArbiter => {
                DiagnosticKind::InvalidConflictArbiter
            }
            SemaError::RefreshConcurrentlyWithoutUniqueIndex { .. } => {
                DiagnosticKind::MissingUniqueIndex
            }
            SemaError::UniqueIndexNotSupported { .. } => DiagnosticKind::UniqueIndexNotSupported,
            SemaError::InvalidOrdinal { .. } => DiagnosticKind::InvalidOrdinal,
            SemaError::MissingArrayType { .. } => DiagnosticKind::MissingArrayType,
            SemaError::TriggerReturnType { .. } => DiagnosticKind::WrongReturnType,
            SemaError::ConflictingNullability { .. } => DiagnosticKind::ConflictingNullability,
        }
    }

    pub(crate) fn suggestion(&self, range: TextRange, names: &Interner) -> Option<Suggestion> {
        let suggestion = match *self {
            SemaError::UnresolvedReference {
                similar: Some(Similar::Column(similar)),
                ..
            } => {
                let column = quote_ident(names.resolve(similar.column));
                let replacement = match similar.qualifier {
                    Some(qualifier) => {
                        format!("{}.{column}", quote_ident(names.resolve(qualifier)))
                    }
                    None => column,
                };
                replace_suggestion("a column with a similar name exists", range, replacement)
            }
            SemaError::UnresolvedReference {
                similar: Some(Similar::Table(table)),
                ..
            } => replace_suggestion(
                "a relation with a similar name exists",
                range,
                quote_ident(names.resolve(table)),
            ),
            SemaError::UnresolvedReference {
                similar: Some(Similar::Type(similar)),
                ..
            } => {
                let replacement = match similar {
                    SimilarType::Named(name) => quote_ident(names.resolve(name)),
                    SimilarType::Keyword(keyword) => keyword.to_owned(),
                };
                replace_suggestion("a type with a similar name exists", range, replacement)
            }
            SemaError::UnresolvedReference {
                similar: Some(Similar::Window(window)),
                ..
            } => replace_suggestion(
                "a window with a similar name exists",
                range,
                quote_ident(names.resolve(window)),
            ),
            SemaError::UnresolvedReference {
                similar: Some(Similar::EnumLabel(label)),
                ..
            }
            | SemaError::InvalidEnumValue {
                similar: Some(label),
                ..
            } => replace_suggestion(
                "an enum label with a similar name exists",
                range,
                quote_string_literal(names.resolve(label)),
            ),
            SemaError::UnresolvedQualifiedColumn {
                similar: Some(similar),
                ..
            } => replace_suggestion(
                "a column with a similar name exists",
                range,
                quote_ident(names.resolve(similar)),
            ),
            SemaError::UnresolvedRoutine {
                kind,
                similar: Some(similar),
                ..
            } => {
                let message = match kind {
                    RoutineKind::Function | RoutineKind::Routine => {
                        "a function with a similar name exists"
                    }
                    RoutineKind::Aggregate => "an aggregate with a similar name exists",
                    RoutineKind::Procedure => "a procedure with a similar name exists",
                };
                replace_suggestion(message, range, quote_ident(names.resolve(similar)))
            }
            SemaError::NotLateral {
                table,
                column,
                lateral_at,
            } => {
                let table = names.resolve(table);
                let message = match column {
                    Some(column) => format!(
                        "to reference column `{}` of table `{table}`, mark the subquery as `lateral`",
                        names.resolve(column)
                    ),
                    None => {
                        format!("to reference table `{table}`, mark the subquery as `lateral`")
                    }
                };
                Suggestion {
                    message: message.into(),
                    fix: Fix {
                        title: "Add `lateral`".into(),
                        edits: vec![Edit::insert("lateral ", lateral_at)],
                    },
                }
            }
            _ => return None,
        };
        Some(suggestion)
    }

    pub(crate) fn hint(&self) -> Option<&'static str> {
        match *self {
            SemaError::TypedTableAlter { .. } => {
                Some("use `alter ... cascade` to alter the typed tables too")
            }
            SemaError::InsertMoreTargetsThanExpressions { row_hint } => row_hint.then_some(
                "the insertion source is a row expression containing the same number of columns expected by the `insert`; consider removing the extra parentheses",
            ),
            SemaError::MissingShellType => Some(
                "create the type as a shell type, then create its canonicalization function, then do a full `create type`",
            ),
            SemaError::WrongRelationKind { kind, drop, .. } => drop.then(|| drop_hint(kind)),
            SemaError::CannotCopy {
                copy_from, kind, ..
            } => copy_target(copy_from, kind).1,
            SemaError::WindowFrameCopy { hint, .. } => {
                hint.then_some("omit the parentheses in this `over` clause")
            }
            SemaError::RefreshConcurrentlyWithoutUniqueIndex { .. } => Some(
                "create a unique index with no `where` clause on one or more columns of the materialized view",
            ),
            SemaError::UnresolvedRoutine {
                similar: Some(_), ..
            } => None,
            SemaError::UnresolvedRoutine {
                args,
                lookup,
                similar: None,
                ..
            } => match (lookup, args) {
                (LookupKind::Call, ErrorArgs::List(args)) if !args.is_empty() => {
                    Some("consider adding explicit type casts")
                }
                (LookupKind::Call | LookupKind::Signature, ErrorArgs::List(_) | ErrorArgs::Star) => {
                    None
                }
            },
            SemaError::UnresolvedOperator { lookup, .. } => match lookup {
                LookupKind::Call => Some("consider adding explicit type casts"),
                LookupKind::Signature => None,
            },
            _ => None,
        }
    }

    fn static_message(&self) -> Option<&'static str> {
        match *self {
            SemaError::CannotChangeDatabase => Some("current database cannot be changed"),
            SemaError::NoConflictArbiter => Some(
                "there is no unique or exclusion constraint matching the `on conflict` specification",
            ),
            SemaError::DeferrableConflictArbiter => Some(
                "`on conflict` does not support deferrable unique constraints/exclusion constraints as arbiters",
            ),
            SemaError::ValuesLengthMismatch => Some("`values` lists must all be the same length"),
            SemaError::InsertMoreExpressionsThanTargets => {
                Some("`insert` has more expressions than target columns")
            }
            SemaError::ViewTooManyColumnNames => {
                Some("`create view` specifies more column names than columns")
            }
            SemaError::CreateAsTooManyColumnNames => Some("too many column names were specified"),
            SemaError::ReplaceConstraintTrigger => {
                Some("`create or replace constraint trigger` is not supported")
            }
            SemaError::RenameSelectRule => Some("renaming an `on select` rule is not allowed"),
            SemaError::NestedAggregate => Some("aggregate function calls cannot be nested"),
            SemaError::AggregateContainsWindow => {
                Some("aggregate function calls cannot contain window function calls")
            }
            SemaError::NestedWindow => Some("window function calls cannot be nested"),
            SemaError::InsertMoreTargetsThanExpressions { .. } => {
                Some("`insert` has more target columns than expressions")
            }
            SemaError::MissingShellType => {
                Some("cannot specify a canonical function without a pre-created shell type")
            }
            SemaError::WindowAggregateOrderBy => {
                Some("aggregate `order by` is not implemented for window functions")
            }
            SemaError::WindowDistinct => Some("`distinct` is not implemented for window functions"),
            SemaError::RowLengthMismatch => Some("unequal number of entries in row expressions"),
            SemaError::DefaultColumnReference => {
                Some("cannot use column reference in `default` expression")
            }
            SemaError::SubqueryNotSingleColumn => Some("subquery must return only one column"),
            SemaError::SubqueryTooManyColumns => Some("subquery has too many columns"),
            SemaError::SubqueryTooFewColumns => Some("subquery has too few columns"),
            _ => None,
        }
    }

    pub(crate) fn message(
        &self,
        names: &Interner,
        args: &LayeredArena<ErrorArg>,
    ) -> Cow<'static, str> {
        match self.static_message() {
            Some(message) => Cow::Borrowed(message),
            None => Cow::Owned(
                ErrorMessage {
                    error: self,
                    names,
                    args,
                }
                .to_string(),
            ),
        }
    }
}

struct ErrorMessage<'a> {
    error: &'a SemaError,
    names: &'a Interner,
    args: &'a LayeredArena<ErrorArg>,
}

impl ErrorMessage<'_> {
    fn ty(&self, ty: TyName) -> impl fmt::Display + '_ {
        fmt::from_fn(move |f| {
            let suffix = if ty.array { "[]" } else { "" };
            write!(f, "{}{suffix}", self.names.resolve(ty.name))
        })
    }

    fn operand(&self, operand: OperandName) -> impl fmt::Display + '_ {
        fmt::from_fn(move |f| match operand {
            OperandName::Ty(ty) => write!(f, "{}", self.ty(ty)),
            OperandName::Unknown => f.write_str("unknown"),
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
            | SemaError::MissingShellType
            | SemaError::WindowAggregateOrderBy
            | SemaError::WindowDistinct
            | SemaError::RowLengthMismatch
            | SemaError::DefaultColumnReference
            | SemaError::SubqueryNotSingleColumn
            | SemaError::SubqueryTooManyColumns
            | SemaError::SubqueryTooFewColumns
            | SemaError::NoConflictArbiter
            | SemaError::DeferrableConflictArbiter => {
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
            SemaError::MisplacedSubquery { context } => write!(
                f,
                "cannot use subquery in {}",
                context.subquery_label().unwrap_or_default()
            ),
            SemaError::GeneratedColumnReference { column } => write!(
                f,
                "cannot use generated column `{}` in column generation expression",
                n(column)
            ),
            SemaError::WindowWithoutOver { name } => {
                write!(f, "window function `{}` requires an `over` clause", n(name))
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
            SemaError::DuplicateColumn { name, relation } => {
                write!(f, "column `{}.{}` already exists", n(relation), n(name))
            }
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
                write!(
                    f,
                    "`with` query name `{}` specified more than once",
                    n(name)
                )
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
                    AccessMethodKind::Table => "`table`",
                    AccessMethodKind::Index => "`index`",
                };
                write!(f, "access method `{}` is not of type {expected}", n(name))
            }
            SemaError::UnresolvedCast { source, target } => write!(
                f,
                "cast from type `{}` to type `{}` does not exist",
                self.ty(source),
                self.ty(target)
            ),
            SemaError::UnresolvedReference { kind, name, .. } => {
                let name = n(name);
                match kind {
                    ReferenceKind::Relation => {
                        write!(f, "missing `from` clause entry for table `{name}`")
                    }
                    ReferenceKind::Table => write!(f, "relation `{name}` does not exist"),
                    ReferenceKind::Column => write!(f, "column `{name}` does not exist"),
                    ReferenceKind::Function(kind) => {
                        write!(f, "{} `{name}` does not exist", kind.label())
                    }
                    ReferenceKind::Type => write!(f, "type `{name}` does not exist"),
                    ReferenceKind::Window => write!(f, "window `{name}` does not exist"),
                    ReferenceKind::JsonPath => write!(f, "JSON path `{name}` does not exist"),
                    ReferenceKind::Index => write!(f, "index `{name}` does not exist"),
                    ReferenceKind::Constraint => write!(f, "constraint `{name}` does not exist"),
                    ReferenceKind::Trigger => write!(f, "trigger `{name}` does not exist"),
                    ReferenceKind::Rule => write!(f, "rule `{name}` does not exist"),
                    ReferenceKind::Role => write!(f, "role `{name}` does not exist"),
                    ReferenceKind::Policy => write!(f, "policy `{name}` does not exist"),
                    ReferenceKind::AccessMethod => {
                        write!(f, "access method `{name}` does not exist")
                    }
                    ReferenceKind::EventTrigger => {
                        write!(f, "event trigger `{name}` does not exist")
                    }
                    ReferenceKind::Language => write!(f, "language `{name}` does not exist"),
                    ReferenceKind::ForeignDataWrapper => {
                        write!(f, "foreign-data wrapper `{name}` does not exist")
                    }
                    ReferenceKind::Server => write!(f, "server `{name}` does not exist"),
                    ReferenceKind::Conversion => write!(f, "conversion `{name}` does not exist"),
                    ReferenceKind::OperatorFamily => {
                        write!(f, "operator family `{name}` does not exist")
                    }
                    ReferenceKind::OperatorClass => {
                        write!(f, "operator class `{name}` does not exist")
                    }
                    ReferenceKind::Statistics => {
                        write!(f, "statistics object `{name}` does not exist")
                    }
                    ReferenceKind::TextSearch(kind) => {
                        write!(f, "{} `{name}` does not exist", kind.label())
                    }
                    ReferenceKind::Publication => write!(f, "publication `{name}` does not exist"),
                    ReferenceKind::Subscription => {
                        write!(f, "subscription `{name}` does not exist")
                    }
                    ReferenceKind::GraphElement => {
                        write!(f, "element table `{name}` does not exist")
                    }
                    ReferenceKind::GraphLabel => write!(f, "label `{name}` does not exist"),
                    ReferenceKind::EnumLabel => write!(f, "enum label `{name}` does not exist"),
                    ReferenceKind::GraphProperty => write!(f, "property `{name}` does not exist"),
                    ReferenceKind::Collation => write!(f, "collation `{name}` does not exist"),
                    ReferenceKind::Schema => write!(f, "schema `{name}` does not exist"),
                    ReferenceKind::Tablespace => write!(f, "tablespace `{name}` does not exist"),
                    ReferenceKind::Database => write!(f, "database `{name}` does not exist"),
                    ReferenceKind::Extension => write!(f, "extension `{name}` does not exist"),
                    ReferenceKind::Operator => write!(f, "operator `{name}` does not exist"),
                    ReferenceKind::Savepoint => write!(f, "savepoint `{name}` does not exist"),
                    ReferenceKind::Cursor => write!(f, "cursor `{name}` does not exist"),
                    ReferenceKind::PreparedTransaction => {
                        write!(f, "prepared transaction `{name}` does not exist")
                    }
                    ReferenceKind::PreparedStatement => {
                        write!(f, "prepared statement `{name}` does not exist")
                    }
                    ReferenceKind::Param => write!(f, "there is no parameter `{name}`"),
                    ReferenceKind::Channel => unreachable!(),
                }
            }
            SemaError::AmbiguousReference { kind, name } => {
                let name = n(name);
                match kind {
                    ReferenceKind::Function(kind) => {
                        write!(f, "{} `{name}` is not unique", kind.label())
                    }
                    ReferenceKind::Operator => write!(f, "operator `{name}` is not unique"),
                    ReferenceKind::Column
                    | ReferenceKind::Relation
                    | ReferenceKind::Table
                    | ReferenceKind::Param
                    | ReferenceKind::Type
                    | ReferenceKind::Window
                    | ReferenceKind::JsonPath
                    | ReferenceKind::Index
                    | ReferenceKind::Constraint
                    | ReferenceKind::Trigger
                    | ReferenceKind::Rule
                    | ReferenceKind::Role
                    | ReferenceKind::Policy
                    | ReferenceKind::AccessMethod
                    | ReferenceKind::EventTrigger
                    | ReferenceKind::Language
                    | ReferenceKind::ForeignDataWrapper
                    | ReferenceKind::Server
                    | ReferenceKind::Conversion
                    | ReferenceKind::OperatorFamily
                    | ReferenceKind::OperatorClass
                    | ReferenceKind::Statistics
                    | ReferenceKind::TextSearch(_)
                    | ReferenceKind::Publication
                    | ReferenceKind::Subscription
                    | ReferenceKind::GraphElement
                    | ReferenceKind::GraphLabel
                    | ReferenceKind::EnumLabel
                    | ReferenceKind::GraphProperty
                    | ReferenceKind::Collation
                    | ReferenceKind::Schema
                    | ReferenceKind::Tablespace
                    | ReferenceKind::Database
                    | ReferenceKind::Extension
                    | ReferenceKind::Savepoint
                    | ReferenceKind::Cursor
                    | ReferenceKind::PreparedTransaction
                    | ReferenceKind::PreparedStatement
                    | ReferenceKind::Channel => {
                        write!(f, "column reference `{name}` is ambiguous")
                    }
                }
            }
            SemaError::UnresolvedQualifiedColumn {
                qualifier,
                column,
                similar: _,
            } => {
                write!(f, "column `{}.{}` does not exist", n(qualifier), n(column))
            }
            SemaError::NotLateral {
                column: Some(column),
                ..
            } => write!(f, "column `{}` does not exist", n(column)),
            SemaError::NotLateral {
                table,
                column: None,
                ..
            } => write!(
                f,
                "invalid reference to `from` clause entry for table `{}`",
                n(table)
            ),
            SemaError::UnresolvedRoutine {
                kind,
                schema,
                name,
                args,
                lookup: _,
                similar: _,
            } => {
                let label = match kind {
                    RoutineKind::Routine => RoutineKind::Function.label(),
                    RoutineKind::Function | RoutineKind::Aggregate | RoutineKind::Procedure => {
                        kind.label()
                    }
                };
                write!(f, "{label} `")?;
                if let Some(schema) = schema {
                    write!(f, "{}.", n(schema))?;
                }
                write!(f, "{}(", n(name))?;
                let args = match args {
                    ErrorArgs::List(args) => args,
                    ErrorArgs::Star => return f.write_str("*)` does not exist"),
                };
                for (i, arg) in args.iter().enumerate() {
                    let arg = self.args[arg];
                    if i > 0 {
                        f.write_str(", ")?;
                    }
                    if arg.variadic {
                        f.write_str("variadic ")?;
                    }
                    if let Some(name) = arg.name {
                        write!(f, "{} => ", n(name))?;
                    }
                    write!(f, "{}", self.operand(arg.ty))?;
                }
                f.write_str(")` does not exist")
            }
            SemaError::UnresolvedOperator {
                schema,
                name,
                left,
                right,
                lookup: _,
            } => {
                f.write_str("operator does not exist: `")?;
                if let Some(left) = left {
                    write!(f, "{} ", self.operand(left))?;
                }
                if let Some(schema) = schema {
                    write!(f, "{}.", n(schema))?;
                }
                write!(f, "{} {}`", n(name), self.operand(right))
            }
            SemaError::UnresolvedTransform { ty, language } => write!(
                f,
                "transform for type `{}` language `{}` does not exist",
                self.ty(ty),
                n(language)
            ),
            SemaError::InvalidEnumValue {
                ty,
                value,
                similar: _,
            } => write!(
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
                write!(f, "each `{op}` query must have the same number of columns")
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
                "column `{}.{}` is not an identity column",
                n(relation),
                n(column)
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
            SemaError::NoPrimaryKeyForReferencedTable { table } => write!(
                f,
                "there is no primary key for referenced table `{}`",
                n(table)
            ),
            SemaError::NoUniqueConstraintForReferencedTable { table } => write!(
                f,
                "there is no unique constraint matching given keys for referenced table `{}`",
                n(table)
            ),
            SemaError::DeferrablePrimaryKeyForReferencedTable { table } => write!(
                f,
                "cannot use a deferrable primary key for referenced table `{}`",
                n(table)
            ),
            SemaError::DeferrableUniqueConstraintForReferencedTable { table } => write!(
                f,
                "cannot use a deferrable unique constraint for referenced table `{}`",
                n(table)
            ),
            SemaError::RefreshConcurrentlyWithoutUniqueIndex { name } => write!(
                f,
                "cannot refresh materialized view `{}` concurrently",
                n(name)
            ),
            SemaError::UniqueIndexNotSupported { access_method } => write!(
                f,
                "access method `{}` does not support unique indexes",
                n(access_method)
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
            SemaError::WindowOverride { name, clause } => write!(
                f,
                "cannot override {} clause of window `{}`",
                clause.label(),
                n(name)
            ),
            SemaError::WindowFrameCopy { name, .. } => write!(
                f,
                "cannot copy window `{}` because it has a frame clause",
                n(name)
            ),
            SemaError::TriggerReturnType { name, kind } => write!(
                f,
                "function `{}` must return type `{}`",
                n(name),
                kind.return_type()
            ),
            SemaError::ConflictingNullability { column, table } => write!(
                f,
                "conflicting `null`/`not null` declarations for column `{}` of table `{}`",
                n(column),
                n(table)
            ),
        }
    }
}
