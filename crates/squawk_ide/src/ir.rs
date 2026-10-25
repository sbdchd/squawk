use std::range::Range;

use la_arena::Idx;
use rowan::{TextRange, TextSize};
use squawk_syntax::{AstPtr, SyntaxNodePtr, ast, sql_body::SqlBody};

use crate::arena::{IdxRange, LayeredArena};
use crate::file::{InIrFile, IrFileId, IrFileRange};
use crate::interner::Name;

pub(crate) type QueryId = Idx<Query>;
pub(crate) type ScopeId = Idx<Scope>;
pub(crate) type CteId = Idx<Cte>;
pub(crate) type SourceId = Idx<Source>;
pub(crate) type ColumnId = Idx<Column>;
pub(crate) type TableId = Idx<Table>;
pub(crate) type RefId = Idx<Reference>;
pub(crate) type FunctionId = Idx<Function>;
pub(crate) type ParamId = Idx<Param>;
pub(crate) type TypeId = Idx<Type>;
pub(crate) type MergeId = Idx<Merge>;
pub(crate) type BodyId = Idx<Body>;
pub(crate) type WindowId = Idx<Window>;
pub(crate) type JsonPathId = Idx<JsonPath>;
pub(crate) type IndexId = Idx<Index>;
pub(crate) type ConstraintId = Idx<Constraint>;
pub(crate) type TriggerId = Idx<Trigger>;
pub(crate) type RuleId = Idx<Rule>;
pub(crate) type RoleId = Idx<Role>;
pub(crate) type PolicyId = Idx<Policy>;
pub(crate) type AccessMethodId = Idx<AccessMethod>;
pub(crate) type EventTriggerId = Idx<EventTrigger>;
pub(crate) type LanguageId = Idx<Language>;
pub(crate) type ForeignDataWrapperId = Idx<ForeignDataWrapper>;
pub(crate) type ServerId = Idx<Server>;
pub(crate) type ConversionId = Idx<Conversion>;
pub(crate) type OperatorFamilyId = Idx<OperatorFamily>;
pub(crate) type OperatorFamilyMemberId = Idx<OperatorFamilyMember>;
pub(crate) type OperatorClassId = Idx<OperatorClass>;
pub(crate) type StatisticsId = Idx<Statistics>;
pub(crate) type TextSearchId = Idx<TextSearch>;
pub(crate) type PublicationId = Idx<Publication>;
pub(crate) type SubscriptionId = Idx<Subscription>;
pub(crate) type GraphElementId = Idx<GraphElement>;
pub(crate) type GraphLabelId = Idx<GraphLabel>;
pub(crate) type EnumLabelId = Idx<EnumLabel>;
pub(crate) type GraphPropertyId = Idx<GraphProperty>;
pub(crate) type CollationId = Idx<Collation>;
pub(crate) type SchemaId = Idx<Schema>;
pub(crate) type TablespaceId = Idx<Tablespace>;
pub(crate) type DatabaseId = Idx<Database>;
pub(crate) type ExtensionId = Idx<Extension>;
pub(crate) type OperatorId = Idx<Operator>;
pub(crate) type CastId = Idx<Cast>;
pub(crate) type TransformId = Idx<Transform>;
pub(crate) type SavepointId = Idx<Savepoint>;
pub(crate) type CursorId = Idx<Cursor>;
pub(crate) type TransactionId = Idx<Transaction>;
pub(crate) type PreparedTransactionId = Idx<PreparedTransaction>;
pub(crate) type PreparedStatementId = Idx<PreparedStatement>;
pub(crate) type ChannelId = Idx<Channel>;
pub(crate) type CaseId = Idx<Case>;
pub(crate) type StarId = Idx<Star>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TableKind {
    Table,
    ForeignTable,
    View,
    MaterializedView,
    Sequence,
    PropertyGraph,
}

impl TableKind {
    pub(crate) fn label(self) -> &'static str {
        match self {
            TableKind::Table => "table",
            TableKind::ForeignTable => "foreign table",
            TableKind::View => "view",
            TableKind::MaterializedView => "materialized view",
            TableKind::Sequence => "sequence",
            TableKind::PropertyGraph => "property graph",
        }
    }
}

#[derive(Debug)]
pub(crate) struct Table {
    pub(crate) kind: TableKind,
    pub(crate) schema: Name,
    pub(crate) name: Name,
    pub(crate) ptr: InIrFile<SyntaxNodePtr>,
    pub(crate) name_ptr: InIrFile<SyntaxNodePtr>,
    pub(crate) columns: Columns,
    pub(crate) constraints_complete: bool,
    pub(crate) sequence_owner: Option<SequenceOwner>,
    pub(crate) of_type: Option<TypeId>,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct SequenceOwner {
    pub(crate) table: TableId,
    pub(crate) column: Name,
    pub(crate) identity: bool,
}

#[derive(Debug)]
pub(crate) struct Index {
    pub(crate) schema: Name,
    pub(crate) name: Name,
    pub(crate) table: Option<TableId>,
    pub(crate) ptr: InIrFile<SyntaxNodePtr>,
    pub(crate) name_ptr: InIrFile<SyntaxNodePtr>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ConstraintKind {
    Check,
    NotNull { column: Name },
    PrimaryKey,
    Unique,
    ForeignKey,
    Exclude,
    DomainNotNull,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum ConstraintOwner {
    Table(TableId),
    Domain(TypeId),
}

#[derive(Debug)]
pub(crate) struct Constraint {
    pub(crate) kind: ConstraintKind,
    pub(crate) name: Name,
    pub(crate) owner: ConstraintOwner,
    pub(crate) index: Option<IndexId>,
    pub(crate) ptr: InIrFile<SyntaxNodePtr>,
    pub(crate) name_ptr: InIrFile<SyntaxNodePtr>,
}

#[derive(Debug)]
pub(crate) struct Trigger {
    pub(crate) name: Name,
    pub(crate) table: TableId,
    pub(crate) constraint: bool,
    #[expect(dead_code)]
    pub(crate) ptr: InIrFile<SyntaxNodePtr>,
    pub(crate) name_ptr: InIrFile<SyntaxNodePtr>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RuleEvent {
    Select,
    Insert,
    Update,
    Delete,
}

#[derive(Debug)]
pub(crate) struct Rule {
    pub(crate) name: Name,
    pub(crate) table: TableId,
    pub(crate) event: RuleEvent,
    #[expect(dead_code)]
    pub(crate) ptr: InIrFile<SyntaxNodePtr>,
    pub(crate) name_ptr: InIrFile<SyntaxNodePtr>,
}

#[derive(Debug)]
pub(crate) struct Collation {
    pub(crate) schema: Name,
    pub(crate) name: Name,
    pub(crate) ptr: InIrFile<SyntaxNodePtr>,
    pub(crate) name_ptr: InIrFile<AstPtr<ast::PathSegment>>,
}

#[derive(Debug)]
pub(crate) struct Schema {
    pub(crate) name: Name,
    pub(crate) ptr: InIrFile<AstPtr<ast::CreateSchema>>,
    pub(crate) name_ptr: InIrFile<SyntaxNodePtr>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AccessMethodKind {
    Table,
    Index,
}

#[derive(Debug)]
pub(crate) struct AccessMethod {
    pub(crate) name: Name,
    pub(crate) kind: AccessMethodKind,
    #[expect(dead_code)]
    pub(crate) ptr: InIrFile<SyntaxNodePtr>,
    pub(crate) name_ptr: InIrFile<SyntaxNodePtr>,
}

#[derive(Debug)]
pub(crate) struct EventTrigger {
    pub(crate) name: Name,
    #[expect(dead_code)]
    pub(crate) ptr: InIrFile<SyntaxNodePtr>,
    pub(crate) name_ptr: InIrFile<SyntaxNodePtr>,
}

#[derive(Debug)]
pub(crate) struct Language {
    pub(crate) name: Name,
    #[expect(dead_code)]
    pub(crate) ptr: InIrFile<SyntaxNodePtr>,
    pub(crate) name_ptr: InIrFile<SyntaxNodePtr>,
}

#[derive(Debug)]
pub(crate) struct ForeignDataWrapper {
    pub(crate) name: Name,
    #[expect(dead_code)]
    pub(crate) ptr: InIrFile<SyntaxNodePtr>,
    pub(crate) name_ptr: InIrFile<SyntaxNodePtr>,
}

#[derive(Debug)]
pub(crate) struct Server {
    pub(crate) name: Name,
    #[expect(dead_code)]
    pub(crate) ptr: InIrFile<SyntaxNodePtr>,
    pub(crate) name_ptr: InIrFile<SyntaxNodePtr>,
}

#[derive(Debug)]
pub(crate) struct Conversion {
    pub(crate) schema: Name,
    pub(crate) name: Name,
    pub(crate) ptr: InIrFile<SyntaxNodePtr>,
    pub(crate) name_ptr: InIrFile<SyntaxNodePtr>,
}

#[derive(Debug)]
pub(crate) struct OperatorFamily {
    pub(crate) schema: Name,
    pub(crate) name: Name,
    pub(crate) access_method: Option<AccessMethodId>,
    pub(crate) ptr: InIrFile<SyntaxNodePtr>,
    pub(crate) name_ptr: InIrFile<SyntaxNodePtr>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum MemberOwner {
    Class(OperatorClassId),
    Family(OperatorFamilyId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum MemberKind {
    Operator,
    Function,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Operands {
    pub(crate) left: Operand,
    pub(crate) right: Operand,
}

#[derive(Debug)]
pub(crate) struct OperatorFamilyMember {
    pub(crate) owner: MemberOwner,
    pub(crate) kind: MemberKind,
    pub(crate) number: Option<u16>,
    pub(crate) operands: Operands,
    pub(crate) ptr: InIrFile<SyntaxNodePtr>,
}

#[derive(Debug)]
pub(crate) struct OperatorClass {
    pub(crate) schema: Name,
    pub(crate) name: Name,
    pub(crate) access_method: Option<AccessMethodId>,
    pub(crate) ptr: InIrFile<SyntaxNodePtr>,
    pub(crate) name_ptr: InIrFile<SyntaxNodePtr>,
}

#[derive(Debug)]
pub(crate) struct Statistics {
    pub(crate) schema: Name,
    pub(crate) name: Name,
    pub(crate) table: Option<TableId>,
    pub(crate) ptr: InIrFile<SyntaxNodePtr>,
    pub(crate) name_ptr: InIrFile<SyntaxNodePtr>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TextSearchKind {
    Parser,
    Template,
    Dictionary,
    Configuration,
}

impl TextSearchKind {
    pub(crate) fn label(self) -> &'static str {
        match self {
            TextSearchKind::Parser => "text search parser",
            TextSearchKind::Template => "text search template",
            TextSearchKind::Dictionary => "text search dictionary",
            TextSearchKind::Configuration => "text search configuration",
        }
    }
}

#[derive(Debug)]
pub(crate) struct TextSearch {
    pub(crate) kind: TextSearchKind,
    pub(crate) schema: Name,
    pub(crate) name: Name,
    pub(crate) ptr: InIrFile<SyntaxNodePtr>,
    pub(crate) name_ptr: InIrFile<SyntaxNodePtr>,
}

#[derive(Debug)]
pub(crate) struct Publication {
    pub(crate) name: Name,
    #[expect(dead_code)]
    pub(crate) ptr: InIrFile<SyntaxNodePtr>,
    pub(crate) name_ptr: InIrFile<SyntaxNodePtr>,
}

#[derive(Debug)]
pub(crate) struct Subscription {
    pub(crate) name: Name,
    #[expect(dead_code)]
    pub(crate) ptr: InIrFile<SyntaxNodePtr>,
    pub(crate) name_ptr: InIrFile<SyntaxNodePtr>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GraphElementKind {
    Vertex,
    Edge,
}

#[derive(Debug)]
pub(crate) struct GraphElement {
    pub(crate) graph: TableId,
    pub(crate) kind: GraphElementKind,
    pub(crate) alias: Name,
    pub(crate) table: Option<TableId>,
    pub(crate) source: Option<GraphElementId>,
    pub(crate) destination: Option<GraphElementId>,
    pub(crate) name_ptr: InIrFile<SyntaxNodePtr>,
}

#[derive(Debug)]
pub(crate) struct EnumLabel {
    pub(crate) ty: TypeId,
    pub(crate) name: Name,
    pub(crate) name_ptr: InIrFile<SyntaxNodePtr>,
}

#[derive(Debug)]
pub(crate) struct GraphLabel {
    pub(crate) element: GraphElementId,
    pub(crate) name: Name,
    pub(crate) name_ptr: InIrFile<SyntaxNodePtr>,
}

#[derive(Debug)]
pub(crate) struct GraphProperty {
    pub(crate) label: GraphLabelId,
    pub(crate) name: Name,
    pub(crate) column: ColumnId,
}

#[derive(Debug)]
pub(crate) struct Policy {
    pub(crate) name: Name,
    pub(crate) table: TableId,
    #[expect(dead_code)]
    pub(crate) ptr: InIrFile<SyntaxNodePtr>,
    pub(crate) name_ptr: InIrFile<SyntaxNodePtr>,
}

#[derive(Debug)]
pub(crate) struct Role {
    pub(crate) name: Name,
    #[expect(dead_code)]
    pub(crate) ptr: InIrFile<SyntaxNodePtr>,
    pub(crate) name_ptr: InIrFile<SyntaxNodePtr>,
}

#[derive(Debug)]
pub(crate) struct Tablespace {
    pub(crate) name: Name,
    pub(crate) ptr: InIrFile<AstPtr<ast::CreateTablespace>>,
    pub(crate) name_ptr: InIrFile<AstPtr<ast::Tablespace>>,
}

#[derive(Debug)]
pub(crate) struct Database {
    pub(crate) name: Name,
    pub(crate) ptr: InIrFile<AstPtr<ast::CreateDatabase>>,
    pub(crate) name_ptr: InIrFile<AstPtr<ast::Database>>,
}

#[derive(Debug)]
pub(crate) struct Extension {
    pub(crate) schema: Name,
    pub(crate) name: Name,
    #[expect(dead_code)]
    pub(crate) ptr: InIrFile<AstPtr<ast::CreateExtension>>,
    pub(crate) name_ptr: InIrFile<AstPtr<ast::Extension>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Operand {
    None,
    Unknown,
    Ty(Ty),
}

impl Operand {
    pub(crate) fn matches(self, other: Operand) -> bool {
        match (self, other) {
            (Operand::None, Operand::None)
            | (Operand::Unknown, Operand::Unknown | Operand::Ty(_))
            | (Operand::Ty(_), Operand::Unknown) => true,
            (Operand::Ty(a), Operand::Ty(b)) => a == b,
            (Operand::None, Operand::Unknown | Operand::Ty(_))
            | (Operand::Unknown | Operand::Ty(_), Operand::None) => false,
        }
    }
}

#[derive(Debug)]
pub(crate) struct Operator {
    pub(crate) schema: Name,
    pub(crate) name: Name,
    pub(crate) left: Operand,
    pub(crate) right: Operand,
    pub(crate) function: Option<FunctionId>,
    pub(crate) ptr: InIrFile<AstPtr<ast::CreateOperator>>,
    pub(crate) name_range: IrFileRange,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CastContext {
    Implicit,
    Assignment,
    Explicit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct CastTypes {
    pub(crate) source: Ty,
    pub(crate) target: Ty,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct TransformKey {
    pub(crate) ty: Ty,
    pub(crate) language: LanguageId,
}

#[derive(Debug)]
pub(crate) struct Transform {
    pub(crate) key: TransformKey,
    #[expect(dead_code)]
    pub(crate) ptr: InIrFile<AstPtr<ast::CreateTransform>>,
}

#[derive(Debug)]
pub(crate) struct Cast {
    pub(crate) source: Ty,
    pub(crate) target: Ty,
    pub(crate) context: CastContext,
    #[expect(dead_code)]
    pub(crate) ptr: InIrFile<AstPtr<ast::CreateCast>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TypeKind {
    Shell,
    Type,
    Domain,
    Enum,
    Range,
    Multirange,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Subscript {
    Array,
    RawArray,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TypeCategory {
    Array,
    Bool,
    Composite,
    DateTime,
    Enum,
    Geometric,
    Network,
    Numeric,
    Pseudo,
    Range,
    String,
    Timespan,
    User,
    BitString,
    Unknown,
    Internal,
    Other(u8),
}

impl TypeCategory {
    pub(crate) fn from_code(code: u8) -> TypeCategory {
        match code {
            b'A' => TypeCategory::Array,
            b'B' => TypeCategory::Bool,
            b'C' => TypeCategory::Composite,
            b'D' => TypeCategory::DateTime,
            b'E' => TypeCategory::Enum,
            b'G' => TypeCategory::Geometric,
            b'I' => TypeCategory::Network,
            b'N' => TypeCategory::Numeric,
            b'P' => TypeCategory::Pseudo,
            b'R' => TypeCategory::Range,
            b'S' => TypeCategory::String,
            b'T' => TypeCategory::Timespan,
            b'U' => TypeCategory::User,
            b'V' => TypeCategory::BitString,
            b'X' => TypeCategory::Unknown,
            b'Z' => TypeCategory::Internal,
            code => TypeCategory::Other(code),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum Ty {
    Type(TypeId),
    Table(TableId),
    Cte(CteId),
    Array(ElementTy),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum ElementTy {
    Type(TypeId),
    Table(TableId),
    Cte(CteId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum ColumnTypeRef {
    Table(TableId),
    Type(TypeId),
}

impl ColumnTypeRef {
    pub(crate) fn of(ty: Ty) -> Option<ColumnTypeRef> {
        match ty {
            Ty::Table(table) | Ty::Array(ElementTy::Table(table)) => {
                Some(ColumnTypeRef::Table(table))
            }
            Ty::Type(ty) | Ty::Array(ElementTy::Type(ty)) => Some(ColumnTypeRef::Type(ty)),
            Ty::Cte(_) | Ty::Array(ElementTy::Cte(_)) => None,
        }
    }

    pub(crate) fn in_columns(
        columns: &LayeredArena<Column>,
        range: IdxRange<Column>,
    ) -> impl Iterator<Item = ColumnTypeRef> + '_ {
        range
            .into_iter()
            .filter_map(|id| columns[id].ty.and_then(ColumnTypeRef::of))
    }
}

impl Ty {
    #[must_use]
    pub(crate) fn array(self) -> Ty {
        Ty::Array(match self {
            Ty::Type(ty) => ElementTy::Type(ty),
            Ty::Table(table) => ElementTy::Table(table),
            Ty::Cte(cte) => ElementTy::Cte(cte),
            Ty::Array(element) => element,
        })
    }

    pub(crate) fn element(self) -> Option<Ty> {
        match self {
            Ty::Array(element) => Some(element.into()),
            Ty::Type(_) | Ty::Table(_) | Ty::Cte(_) => None,
        }
    }
}

impl From<ElementTy> for Ty {
    fn from(element: ElementTy) -> Ty {
        match element {
            ElementTy::Type(ty) => Ty::Type(ty),
            ElementTy::Table(table) => Ty::Table(table),
            ElementTy::Cte(cte) => Ty::Cte(cte),
        }
    }
}

#[derive(Debug)]
pub(crate) struct Type {
    pub(crate) kind: TypeKind,
    pub(crate) schema: Name,
    pub(crate) name: Name,
    pub(crate) ptr: InIrFile<SyntaxNodePtr>,
    pub(crate) name_ptr: InIrFile<AstPtr<ast::PathSegment>>,
    pub(crate) shell_name_ptr: Option<InIrFile<AstPtr<ast::PathSegment>>>,
    pub(crate) columns: Option<Columns>,
    pub(crate) base: Option<Ty>,
    pub(crate) element: Option<Ty>,
    pub(crate) subtype: Option<Ty>,
    pub(crate) range: Option<TypeId>,
    pub(crate) subscript: Option<Subscript>,
    pub(crate) subscript_result: Option<Ty>,
    pub(crate) slice_result: Option<Ty>,
    pub(crate) category: Option<TypeCategory>,
    pub(crate) preferred: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FunctionKind {
    Function,
    Aggregate,
    Procedure,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RoutineKind {
    Function,
    Aggregate,
    Procedure,
    Routine,
}

impl RoutineKind {
    pub(crate) fn matches(self, kind: FunctionKind) -> bool {
        match self {
            RoutineKind::Function => kind == FunctionKind::Function,
            RoutineKind::Aggregate => kind == FunctionKind::Aggregate,
            RoutineKind::Procedure => kind == FunctionKind::Procedure,
            RoutineKind::Routine => true,
        }
    }
}

#[derive(Debug)]
pub(crate) struct Function {
    pub(crate) kind: FunctionKind,
    pub(crate) window: bool,
    pub(crate) schema: Name,
    pub(crate) name: Name,
    pub(crate) ptr: InIrFile<SyntaxNodePtr>,
    pub(crate) name_ptr: InIrFile<AstPtr<ast::PathSegment>>,
    pub(crate) params: IdxRange<Param>,
    pub(crate) direct_args: Option<usize>,
    pub(crate) ret: Option<Ty>,
    pub(crate) columns: Option<Columns>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ParamMode {
    In,
    Out,
    InOut,
    Variadic,
}

#[derive(Debug)]
pub(crate) struct Param {
    pub(crate) name: Option<Name>,
    pub(crate) mode: ParamMode,
    pub(crate) ty: Option<Ty>,
    pub(crate) has_default: bool,
    pub(crate) ptr: InIrFile<SyntaxNodePtr>,
    pub(crate) name_ptr: Option<InIrFile<AstPtr<ast::ParamName>>>,
}

#[derive(Debug)]
pub(crate) struct Body {
    pub(crate) parent: IrFileId,
    pub(crate) sql: SqlBody,
}

#[derive(Debug)]
pub(crate) struct Cte {
    pub(crate) name: Name,
    #[expect(dead_code)]
    pub(crate) ptr: InIrFile<AstPtr<ast::WithTable>>,
    pub(crate) name_ptr: InIrFile<AstPtr<ast::CteName>>,
    pub(crate) query: Option<QueryId>,
    pub(crate) columns: Columns,
}

#[derive(Debug)]
pub(crate) struct Window {
    pub(crate) name: Name,
    #[expect(dead_code)]
    pub(crate) ptr: InIrFile<AstPtr<ast::WindowDef>>,
    pub(crate) name_ptr: InIrFile<AstPtr<ast::Window>>,
    pub(crate) clauses: WindowClauses,
}

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct WindowClauses {
    pub(crate) partition_by: bool,
    pub(crate) order_by: bool,
    pub(crate) frame: bool,
}

#[derive(Debug)]
pub(crate) struct JsonPath {
    pub(crate) name: Name,
    pub(crate) name_ptr: InIrFile<AstPtr<ast::JsonPathName>>,
}

#[derive(Debug)]
pub(crate) struct Savepoint {
    pub(crate) name: Name,
    #[expect(dead_code)]
    pub(crate) ptr: InIrFile<AstPtr<ast::SavepointCreate>>,
    pub(crate) name_ptr: InIrFile<AstPtr<ast::Savepoint>>,
}

#[derive(Debug)]
pub(crate) struct Cursor {
    pub(crate) name: Name,
    pub(crate) hold: bool,
    #[expect(dead_code)]
    pub(crate) ptr: InIrFile<AstPtr<ast::Declare>>,
    pub(crate) name_ptr: InIrFile<AstPtr<ast::Cursor>>,
}

#[derive(Debug)]
pub(crate) struct Transaction {
    pub(crate) begin: InIrFile<AstPtr<ast::Begin>>,
    pub(crate) end: Option<InIrFile<SyntaxNodePtr>>,
}

#[derive(Debug)]
pub(crate) struct PreparedTransaction {
    pub(crate) name: Name,
    #[expect(dead_code)]
    pub(crate) ptr: InIrFile<AstPtr<ast::PrepareTransaction>>,
    pub(crate) name_ptr: InIrFile<AstPtr<ast::Literal>>,
}

#[derive(Debug)]
pub(crate) struct PreparedStatement {
    pub(crate) name: Name,
    pub(crate) columns: Columns,
    #[expect(dead_code)]
    pub(crate) ptr: InIrFile<AstPtr<ast::Prepare>>,
    pub(crate) name_ptr: InIrFile<AstPtr<ast::PreparedStatement>>,
}

#[derive(Debug)]
pub(crate) struct Case {
    pub(crate) case_range: IrFileRange,
}

#[derive(Debug)]
pub(crate) struct Star {
    pub(crate) columns: IdxRange<ColumnId>,
}

#[derive(Debug)]
pub(crate) struct Channel {
    pub(crate) name: Name,
    pub(crate) name_ptr: InIrFile<AstPtr<ast::Channel>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Columns {
    pub(crate) range: IdxRange<Column>,
    pub(crate) complete: bool,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Column {
    pub(crate) name: Option<Name>,
    pub(crate) ptr: InIrFile<SyntaxNodePtr>,
    pub(crate) ty: Option<Ty>,
    pub(crate) merge: Option<MergeId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ColumnOwner {
    Table(TableId),
    Type(TypeId),
    Cte(CteId),
    Function(FunctionId),
    PreparedStatement(PreparedStatementId),
    Anonymous,
}

#[derive(Debug)]
pub(crate) struct Merge {
    pub(crate) columns: IdxRange<Column>,
    pub(crate) plain: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum QueryKind {
    Select,
    Compound,
    Values,
    Table,
    Delete,
    Insert,
    Update,
    Merge,
    Index,
    Constraint,
    Trigger,
    Rule,
    Policy,
    Statistics,
    Publication,
    PropertyGraph,
    GraphTable,
    Copy,
}

#[derive(Debug)]
pub(crate) struct Query {
    #[expect(dead_code)]
    pub(crate) kind: QueryKind,
    #[expect(dead_code)]
    pub(crate) ptr: InIrFile<SyntaxNodePtr>,
    #[expect(dead_code)]
    pub(crate) parent: Option<QueryId>,
    pub(crate) sources: Vec<SourceId>,
    pub(crate) columns: Columns,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SourceKind {
    Cte(CteId),
    Table(TableId),
    External,
    Subquery(QueryId),
    Function(FunctionId),
    Join,
    UsingAlias,
    Returning(SourceId),
    Excluded(SourceId),
    RuleRelation(SourceId),
    ElementVariable,
    Other,
}

impl SourceKind {
    pub(crate) fn is_qualified_only(self) -> bool {
        matches!(
            self,
            SourceKind::UsingAlias
                | SourceKind::Returning(_)
                | SourceKind::RuleRelation(_)
                | SourceKind::ElementVariable
        )
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Scope {
    pub(crate) query: QueryId,
    pub(crate) parent: Option<ScopeId>,
    pub(crate) visible: Range<usize>,
    pub(crate) hidden: Vec<Range<usize>>,
}

impl Scope {
    pub(crate) fn visible_indices(&self) -> impl Iterator<Item = usize> + '_ {
        self.visible
            .into_iter()
            .filter(|index| !self.hidden.iter().any(|range| range.contains(index)))
    }

    pub(crate) fn hidden_indices(&self) -> impl Iterator<Item = usize> + '_ {
        self.hidden
            .iter()
            .flat_map(|range| range.start.max(self.visible.start)..range.end.min(self.visible.end))
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Source {
    pub(crate) name: Option<Name>,
    pub(crate) kind: SourceKind,
    pub(crate) ptr: InIrFile<SyntaxNodePtr>,
    pub(crate) alias_ptr: Option<InIrFile<AstPtr<ast::TableAlias>>>,
    pub(crate) columns: Columns,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Resolution {
    Column {
        source: Option<SourceId>,
        column: ColumnId,
    },
    Source(SourceId),
    Cte(CteId),
    Table(TableId),
    Function(FunctionId),
    Param(ParamId),
    Type(TypeId),
    Window(WindowId),
    JsonPath(JsonPathId),
    Index(IndexId),
    Constraint(ConstraintId),
    Trigger(TriggerId),
    Rule(RuleId),
    Role(RoleId),
    Policy(PolicyId),
    AccessMethod(AccessMethodId),
    EventTrigger(EventTriggerId),
    Language(LanguageId),
    ForeignDataWrapper(ForeignDataWrapperId),
    Server(ServerId),
    Conversion(ConversionId),
    OperatorFamily(OperatorFamilyId),
    OperatorClass(OperatorClassId),
    Statistics(StatisticsId),
    TextSearch(TextSearchId),
    Publication(PublicationId),
    Subscription(SubscriptionId),
    GraphElement(GraphElementId),
    GraphLabel(GraphLabelId),
    EnumLabel(EnumLabelId),
    GraphProperty(GraphPropertyId),
    Collation(CollationId),
    Schema(SchemaId),
    Tablespace(TablespaceId),
    Database(DatabaseId),
    Extension(ExtensionId),
    Operator(OperatorId),
    Savepoint(SavepointId),
    Cursor(CursorId),
    PreparedTransaction(PreparedTransactionId),
    PreparedStatement(PreparedStatementId),
    Channel(ChannelId),
    Unknown,
    Unresolved,
    Ambiguous,
}

impl Resolution {
    pub(crate) fn item(self) -> Option<Item> {
        Some(match self {
            Resolution::Column { column, .. } => Item::Column(column),
            Resolution::Source(source) => Item::Source(source),
            Resolution::Cte(cte) => Item::Cte(cte),
            Resolution::Table(table) => Item::Table(table),
            Resolution::Function(function) => Item::Function(function),
            Resolution::Param(param) => Item::Param(param),
            Resolution::Type(ty) => Item::Type(ty),
            Resolution::Window(window) => Item::Window(window),
            Resolution::JsonPath(path) => Item::JsonPath(path),
            Resolution::Index(index) => Item::Index(index),
            Resolution::Constraint(constraint) => Item::Constraint(constraint),
            Resolution::Trigger(trigger) => Item::Trigger(trigger),
            Resolution::Rule(rule) => Item::Rule(rule),
            Resolution::Role(role) => Item::Role(role),
            Resolution::Policy(policy) => Item::Policy(policy),
            Resolution::AccessMethod(access_method) => Item::AccessMethod(access_method),
            Resolution::EventTrigger(event_trigger) => Item::EventTrigger(event_trigger),
            Resolution::Language(language) => Item::Language(language),
            Resolution::ForeignDataWrapper(wrapper) => Item::ForeignDataWrapper(wrapper),
            Resolution::Server(server) => Item::Server(server),
            Resolution::Conversion(conversion) => Item::Conversion(conversion),
            Resolution::OperatorFamily(family) => Item::OperatorFamily(family),
            Resolution::OperatorClass(class) => Item::OperatorClass(class),
            Resolution::Statistics(statistics) => Item::Statistics(statistics),
            Resolution::TextSearch(text_search) => Item::TextSearch(text_search),
            Resolution::Publication(publication) => Item::Publication(publication),
            Resolution::Subscription(subscription) => Item::Subscription(subscription),
            Resolution::GraphElement(graph_element) => Item::GraphElement(graph_element),
            Resolution::GraphLabel(graph_label) => Item::GraphLabel(graph_label),
            Resolution::GraphProperty(graph_property) => Item::GraphProperty(graph_property),
            Resolution::EnumLabel(label) => Item::EnumLabel(label),
            Resolution::Collation(collation) => Item::Collation(collation),
            Resolution::Schema(schema) => Item::Schema(schema),
            Resolution::Tablespace(tablespace) => Item::Tablespace(tablespace),
            Resolution::Database(database) => Item::Database(database),
            Resolution::Extension(extension) => Item::Extension(extension),
            Resolution::Operator(operator) => Item::Operator(operator),
            Resolution::Savepoint(savepoint) => Item::Savepoint(savepoint),
            Resolution::Cursor(cursor) => Item::Cursor(cursor),
            Resolution::PreparedTransaction(prepared) => Item::PreparedTransaction(prepared),
            Resolution::PreparedStatement(prepared) => Item::PreparedStatement(prepared),
            Resolution::Channel(channel) => Item::Channel(channel),
            Resolution::Unknown | Resolution::Unresolved | Resolution::Ambiguous => return None,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ReferenceKind {
    Column,
    Relation,
    Table,
    Function(RoutineKind),
    Param,
    Type,
    Window,
    JsonPath,
    Index,
    Constraint,
    Trigger,
    Rule,
    Role,
    Policy,
    AccessMethod,
    EventTrigger,
    Language,
    ForeignDataWrapper,
    Server,
    Conversion,
    OperatorFamily,
    OperatorClass,
    Statistics,
    TextSearch(TextSearchKind),
    Publication,
    Subscription,
    GraphElement,
    GraphLabel,
    EnumLabel,
    GraphProperty,
    Collation,
    Schema,
    Tablespace,
    Database,
    Extension,
    Operator,
    Savepoint,
    Cursor,
    PreparedTransaction,
    PreparedStatement,
    Channel,
}

#[derive(Debug)]
pub(crate) struct Reference {
    pub(crate) name: Name,
    pub(crate) kind: ReferenceKind,
    pub(crate) range: IrFileRange,
    pub(crate) resolution: Resolution,
    pub(crate) scope: Option<ScopeId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum Item {
    Reference(RefId),
    Cte(CteId),
    Column(ColumnId),
    Source(SourceId),
    Table(TableId),
    Function(FunctionId),
    Param(ParamId),
    Type(TypeId),
    Window(WindowId),
    JsonPath(JsonPathId),
    Index(IndexId),
    Constraint(ConstraintId),
    Trigger(TriggerId),
    Rule(RuleId),
    Role(RoleId),
    Policy(PolicyId),
    AccessMethod(AccessMethodId),
    EventTrigger(EventTriggerId),
    Language(LanguageId),
    ForeignDataWrapper(ForeignDataWrapperId),
    Server(ServerId),
    Conversion(ConversionId),
    OperatorFamily(OperatorFamilyId),
    OperatorClass(OperatorClassId),
    Statistics(StatisticsId),
    TextSearch(TextSearchId),
    Publication(PublicationId),
    Subscription(SubscriptionId),
    GraphElement(GraphElementId),
    GraphLabel(GraphLabelId),
    EnumLabel(EnumLabelId),
    GraphProperty(GraphPropertyId),
    Collation(CollationId),
    Schema(SchemaId),
    Tablespace(TablespaceId),
    Database(DatabaseId),
    Extension(ExtensionId),
    Operator(OperatorId),
    Savepoint(SavepointId),
    Cursor(CursorId),
    PreparedTransaction(PreparedTransactionId),
    PreparedStatement(PreparedStatementId),
    Channel(ChannelId),
    TransactionBegin(TransactionId),
    TransactionEnd(TransactionId),
    CaseKeyword(CaseId),
    Star(StarId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ItemRange {
    pub(crate) range: TextRange,
    pub(crate) item: Item,
}

#[derive(Debug, Clone)]
pub(crate) struct SearchPathChange {
    pub(crate) start: TextSize,
    pub(crate) path: Box<[Name]>,
}
