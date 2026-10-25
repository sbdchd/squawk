use std::borrow::Cow;
use std::ops::Deref;

use rustc_hash::FxHashMap;
use smallvec::SmallVec;

use crate::arena::{IdxRange, LayeredArena};
use crate::constraint_index::ConstraintIndex;
use crate::extension_index::ExtensionIndex;
use crate::file::{FileId, FileRange, InFile, InIrFile, IrFileId, IrFileRange};
use crate::index_index::IndexIndex;
use crate::interner::{Interner, Name};
use crate::ir::{
    Body, Case, Channel, Column, ColumnId, ColumnOwner, Cte, Cursor, ElementTy, EnumLabel,
    EnumLabelId, FunctionId, Item, ItemRange, JsonPath, Merge, Param, ParamId, PreparedStatement,
    PreparedTransaction, Query, RefId, Reference, Savepoint, Scope, SearchPathChange, Source,
    SourceId, SourceKind, Star, Subscript, Transaction, Transform, Ty, TypeId, TypeKind, Window,
};
use crate::keyed_index::{
    AccessMethodIndex, CastIndex, DatabaseIndex, EventTriggerIndex, ForeignDataWrapperIndex,
    GraphElementIndex, GraphLabelIndex, GraphPropertyIndex, LanguageIndex,
    OperatorFamilyMemberIndex, PolicyIndex, PublicationIndex, RoleIndex, RuleIndex, SchemaIndex,
    ServerIndex, SubscriptionIndex, TablespaceIndex, TriggerIndex,
};
use crate::qualified_index::{
    CollationIndex, ConversionIndex, FunctionIndex, OperatorClassIndex, OperatorFamilyIndex,
    OperatorIndex, TextSearchIndex,
};
use crate::sema_error::{SemaError, TyName, reference_error};
use crate::statistics_index::StatisticsIndex;
use crate::table_index::TableIndex;
use crate::type_index::TypeIndex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticKind {
    UnresolvedColumn,
    UnresolvedRelation,
    UnresolvedTable,
    AmbiguousColumn,
    AmbiguousFunction,
    AmbiguousOperator,
    UnresolvedParam,
    UnresolvedFunction,
    UnresolvedType,
    UnresolvedWindow,
    UnresolvedJsonPath,
    UnresolvedIndex,
    UnresolvedConstraint,
    UnresolvedTrigger,
    UnresolvedRule,
    UnresolvedRole,
    UnresolvedPolicy,
    UnresolvedAccessMethod,
    UnresolvedEventTrigger,
    UnresolvedLanguage,
    UnresolvedForeignDataWrapper,
    UnresolvedServer,
    UnresolvedConversion,
    UnresolvedOperatorFamily,
    UnresolvedOperatorClass,
    UnresolvedStatistics,
    UnresolvedTextSearch,
    UnresolvedPublication,
    UnresolvedSubscription,
    UnresolvedGraphElement,
    UnresolvedGraphLabel,
    UnresolvedEnumLabel,
    UnresolvedGraphProperty,
    UnresolvedCollation,
    UnresolvedSchema,
    UnresolvedTablespace,
    UnresolvedDatabase,
    UnresolvedExtension,
    UnresolvedOperator,
    UnresolvedCast,
    UnresolvedTransform,
    UnresolvedSavepoint,
    UnresolvedCursor,
    UnresolvedPreparedTransaction,
    UnresolvedPreparedStatement,
    DuplicatePreparedStatement,
    DependentObjectsStillExist,
    RequiredBySystem,
    TypedTableAlter,
    InvalidDomainConstraint,
    DuplicateRelation,
    DuplicateType,
    DuplicateColumn,
    ColumnCountMismatch,
    GroupingError,
    WindowingError,
    DuplicateAssignment,
    DuplicateCte,
    DuplicateSource,
    DuplicateWindow,
    DuplicateParam,
    MultiplePrimaryKeys,
    DuplicateFunction,
    DuplicateConstraint,
    DuplicateTrigger,
    ReplaceConstraintTrigger,
    DuplicateRule,
    DropReturnRule,
    RenameSelectRule,
    NotIdentityColumn,
    WrongTypeKind,
    WrongRelationKind,
    CannotChangeDatabase,
    DuplicateRole,
    DuplicatePolicy,
    DuplicateAccessMethod,
    AccessMethodKindMismatch,
    DuplicateEventTrigger,
    DuplicateLanguage,
    DuplicateForeignDataWrapper,
    DuplicateServer,
    DuplicateConversion,
    DuplicateOperatorFamily,
    DuplicateOperatorClass,
    DuplicateStatistics,
    DuplicateTextSearch,
    DuplicatePublication,
    DuplicateSubscription,
    DuplicateGraphElement,
    DuplicateGraphLabel,
    DuplicateEnumLabel,
    DuplicateGraphProperty,
    DuplicateCollation,
    DuplicateSchema,
    DuplicateTablespace,
    DuplicateDatabase,
    DuplicateExtension,
    DuplicateOperator,
    DuplicateTransform,
    MissingArrayType,
    MissingShellType,
    InvalidOrdinal,
    WrongReturnType,
    ConflictingNullability,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub kind: DiagnosticKind,
    pub range: FileRange,
    pub message: Cow<'static, str>,
    pub hint: Option<&'static str>,
}

#[derive(Debug, Default)]
pub(crate) struct Errors(Vec<(FileRange, SemaError)>);

impl Errors {
    pub(crate) fn push(&mut self, range: FileRange, error: SemaError) {
        self.0.push((range, error));
    }

    pub(crate) fn shrink_to_fit(&mut self) {
        self.0.shrink_to_fit();
    }
}

pub(crate) struct TypeName<'a> {
    index: &'a IndexData,
    ty: Ty,
}

impl std::fmt::Display for TypeName<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let TyName { name, array } = self.index.ty_name(self.ty);
        let suffix = if array { "[]" } else { "" };
        write!(f, "{}{suffix}", self.index.names.resolve(name))
    }
}

#[derive(Debug)]
pub(crate) struct SemanticIndex {
    // Wrap IndexData so we don't expose a mutable version of the SemanticIndex.
    // All mutation of it happens during lowering.
    data: IndexData,
}

impl From<IndexData> for SemanticIndex {
    fn from(data: IndexData) -> SemanticIndex {
        SemanticIndex { data }
    }
}

impl Deref for SemanticIndex {
    type Target = IndexData;

    fn deref(&self) -> &IndexData {
        &self.data
    }
}

#[derive(Debug)]
pub(crate) struct IndexData {
    pub(crate) names: Interner,
    pub(crate) tables: TableIndex,
    pub(crate) types: TypeIndex,
    pub(crate) functions: FunctionIndex,
    pub(crate) params: LayeredArena<Param>,
    pub(crate) ctes: LayeredArena<Cte>,
    pub(crate) queries: LayeredArena<Query>,
    pub(crate) scopes: LayeredArena<Scope>,
    pub(crate) sources: LayeredArena<Source>,
    pub(crate) columns: LayeredArena<Column>,
    pub(crate) column_owners: FxHashMap<ColumnId, ColumnOwner>,
    pub(crate) merges: LayeredArena<Merge>,
    pub(crate) references: LayeredArena<Reference>,
    pub(crate) bodies: LayeredArena<Body>,
    pub(crate) windows: LayeredArena<Window>,
    pub(crate) json_paths: LayeredArena<JsonPath>,
    pub(crate) indexes: IndexIndex,
    pub(crate) constraints: ConstraintIndex,
    pub(crate) triggers: TriggerIndex,
    pub(crate) rules: RuleIndex,
    pub(crate) roles: RoleIndex,
    pub(crate) policies: PolicyIndex,
    pub(crate) access_methods: AccessMethodIndex,
    pub(crate) event_triggers: EventTriggerIndex,
    pub(crate) languages: LanguageIndex,
    pub(crate) foreign_data_wrappers: ForeignDataWrapperIndex,
    pub(crate) servers: ServerIndex,
    pub(crate) conversions: ConversionIndex,
    pub(crate) operator_families: OperatorFamilyIndex,
    pub(crate) operator_family_members: OperatorFamilyMemberIndex,
    pub(crate) operator_classes: OperatorClassIndex,
    pub(crate) statistics: StatisticsIndex,
    pub(crate) text_search: TextSearchIndex,
    pub(crate) publications: PublicationIndex,
    pub(crate) subscriptions: SubscriptionIndex,
    pub(crate) graph_elements: GraphElementIndex,
    pub(crate) graph_labels: GraphLabelIndex,
    pub(crate) enum_labels: LayeredArena<EnumLabel>,
    pub(crate) graph_properties: GraphPropertyIndex,
    pub(crate) collations: CollationIndex,
    pub(crate) schemas: SchemaIndex,
    pub(crate) tablespaces: TablespaceIndex,
    pub(crate) databases: DatabaseIndex,
    pub(crate) extensions: ExtensionIndex,
    pub(crate) operators: OperatorIndex,
    pub(crate) casts: CastIndex,
    pub(crate) transforms: LayeredArena<Transform>,
    pub(crate) savepoints: LayeredArena<Savepoint>,
    pub(crate) cursors: LayeredArena<Cursor>,
    pub(crate) transactions: LayeredArena<Transaction>,
    pub(crate) prepared_transactions: LayeredArena<PreparedTransaction>,
    pub(crate) prepared_statements: LayeredArena<PreparedStatement>,
    pub(crate) channels: LayeredArena<Channel>,
    pub(crate) cases: LayeredArena<Case>,
    pub(crate) stars: LayeredArena<Star>,
    pub(crate) star_columns: LayeredArena<ColumnId>,
    pub(crate) builtins: Option<&'static SemanticIndex>,
    pub(crate) items: Vec<InFile<ItemRange>>,
    pub(crate) search_paths: Vec<InFile<SearchPathChange>>,
    pub(crate) errors: Errors,
    pub(crate) renames: FxHashMap<Item, SmallVec<[NameVersion; 1]>>,
    pub(crate) param_functions: FxHashMap<ParamId, FunctionId>,
    pub(crate) comments: FxHashMap<Item, Box<str>>,
    pub(crate) type_enum_labels: FxHashMap<TypeId, SmallVec<[EnumLabelId; 4]>>,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct NameVersion {
    pub(crate) until: RefId,
    pub(crate) name: IrFileRange,
}

#[derive(Clone, Copy)]
pub(crate) enum AsOf {
    Ref(RefId),
    Def(FileRange),
    Latest,
}

impl IndexData {
    pub(crate) fn base_type(&self, mut ty: Ty) -> Option<Ty> {
        while let Ty::Type(id) = ty
            && self.types[id].kind == TypeKind::Domain
        {
            ty = self.types[id].base?;
        }
        Some(ty)
    }

    pub(crate) fn type_name(&self, ty: Ty) -> TypeName<'_> {
        TypeName { index: self, ty }
    }

    pub(crate) fn ty_name(&self, ty: Ty) -> TyName {
        let (element, array) = match ty {
            Ty::Type(id) => (ElementTy::Type(id), false),
            Ty::Table(id) => (ElementTy::Table(id), false),
            Ty::Cte(id) => (ElementTy::Cte(id), false),
            Ty::Array(element) => (element, true),
        };
        let name = match element {
            ElementTy::Type(id) => self.types[id].name,
            ElementTy::Table(id) => self.tables[id].name,
            ElementTy::Cte(id) => self.ctes[id].name,
        };
        TyName { name, array }
    }

    pub(crate) fn element_type(&self, ty: Ty) -> Option<Ty> {
        match ty {
            Ty::Array(element) => Some(element.into()),
            Ty::Type(id) => {
                let ty = &self.types[id];
                ty.element
                    .filter(|_| ty.subscript == Some(Subscript::Array))
            }
            Ty::Table(_) | Ty::Cte(_) => None,
        }
    }

    pub(crate) fn name_at(&self, item: Item, current: IrFileRange, as_of: AsOf) -> IrFileRange {
        let Some(versions) = self.renames.get(&item) else {
            return current;
        };
        match as_of {
            AsOf::Ref(reference) => versions
                .iter()
                .find(|version| reference < version.until)
                .map_or(current, |version| version.name),
            AsOf::Def(def) => versions
                .iter()
                .find(|version| self.range(version.name) == def)
                .map_or(current, |version| version.name),
            AsOf::Latest => current,
        }
    }

    pub(crate) fn item_at_range(&self, range: FileRange) -> Option<InFile<ItemRange>> {
        if range.file_id == FileId::Builtins
            && let Some(builtins) = self.builtins
        {
            return builtins.item_at_range(range);
        }
        let idx = self.items.partition_point(|item| {
            (item.file_id, item.value.range.start()) <= (range.file_id, range.value.start())
        });
        let item = *self.items.get(idx.checked_sub(1)?)?;
        (item.file_id == range.file_id && item.value.range.contains_range(range.value))
            .then_some(item)
    }

    pub(crate) fn search_path_at(&self, range: FileRange) -> Option<&[Name]> {
        let idx = self.search_paths.partition_point(|change| {
            (change.file_id, change.value.start) <= (range.file_id, range.value.start())
        });
        let change = self.search_paths.get(idx.checked_sub(1)?)?;
        (change.file_id == range.file_id).then_some(&*change.value.path)
    }

    pub(crate) fn items_at_range(&self, range: FileRange) -> impl Iterator<Item = Item> + '_ {
        let start = self.items.partition_point(|item| {
            (item.file_id, item.value.range.start()) < (range.file_id, range.value.start())
        });
        self.items[start..]
            .iter()
            .take_while(move |item| {
                item.file_id == range.file_id && item.value.range.start() == range.value.start()
            })
            .filter(move |item| item.value.range == range.value)
            .map(|item| item.value.item)
    }

    pub(crate) fn file(&self, mut file_id: IrFileId) -> FileId {
        loop {
            match file_id {
                IrFileId::File(file_id) => return file_id,
                IrFileId::Body(body) => file_id = self.bodies[body].parent,
            }
        }
    }

    pub(crate) fn range(&self, mut range: IrFileRange) -> FileRange {
        loop {
            match range.file_id {
                IrFileId::File(file_id) => return InFile::new(file_id, range.value),
                IrFileId::Body(body) => {
                    let body = &self.bodies[body];
                    range = InIrFile::new(body.parent, body.sql.source_range(range.value));
                }
            }
        }
    }

    pub(crate) fn column_owner(&self, column: ColumnId) -> ColumnOwner {
        self.column_owners
            .get(&column)
            .or_else(|| {
                self.builtins
                    .and_then(|builtins| builtins.column_owners.get(&column))
            })
            .copied()
            .unwrap_or(ColumnOwner::Anonymous)
    }

    pub(crate) fn comment(&self, item: Item) -> Option<&str> {
        self.comments
            .get(&item)
            .or_else(|| {
                self.builtins
                    .and_then(|builtins| builtins.comments.get(&item))
            })
            .map(|comment| &**comment)
    }

    pub(crate) fn param_function(&self, param: ParamId) -> Option<FunctionId> {
        self.param_functions
            .get(&param)
            .or_else(|| {
                self.builtins
                    .and_then(|builtins| builtins.param_functions.get(&param))
            })
            .copied()
    }

    pub(crate) fn enum_labels(&self, ty: TypeId) -> impl Iterator<Item = EnumLabelId> + '_ {
        self.builtins
            .and_then(|builtins| builtins.type_enum_labels.get(&ty))
            .into_iter()
            .chain(self.type_enum_labels.get(&ty))
            .flatten()
            .copied()
    }

    pub(crate) fn merged_columns(&self, column: ColumnId) -> IdxRange<Column> {
        match self.columns[column].merge {
            Some(merge) => self.merges[merge].columns,
            None => IdxRange::single(column),
        }
    }

    pub(crate) fn source_type(&self, source: SourceId) -> Option<Ty> {
        match self.sources[source].kind {
            SourceKind::Table(table) => Some(Ty::Table(table)),
            SourceKind::Cte(cte) => Some(Ty::Cte(cte)),
            SourceKind::Returning(target)
            | SourceKind::Excluded(target)
            | SourceKind::RuleRelation(target) => self.source_type(target),
            SourceKind::External
            | SourceKind::Subquery(_)
            | SourceKind::Function(_)
            | SourceKind::Join
            | SourceKind::UsingAlias
            | SourceKind::ElementVariable
            | SourceKind::Other => None,
        }
    }

    pub(crate) fn file_items(&self, file_id: FileId) -> &[InFile<ItemRange>] {
        let start = self.items.partition_point(|item| item.file_id < file_id);
        let end = self.items.partition_point(|item| item.file_id <= file_id);
        &self.items[start..end]
    }

    pub(crate) fn items(&self) -> impl Iterator<Item = InFile<ItemRange>> + '_ {
        self.builtins
            .into_iter()
            .flat_map(|builtins| &builtins.items)
            .chain(&self.items)
            .copied()
    }

    pub(crate) fn diagnostics(&self) -> Vec<Diagnostic> {
        self.references
            .iter_local()
            .filter_map(|(_, reference)| {
                let name = self.names.resolve(reference.name);
                let (kind, message) = reference_error(reference.kind, reference.resolution, name)?;
                Some(Diagnostic {
                    kind,
                    range: self.range(reference.range),
                    message: message.into(),
                    hint: None,
                })
            })
            .chain(self.errors.0.iter().map(|&(range, error)| Diagnostic {
                kind: error.kind(),
                range,
                message: error.message(&self.names),
                hint: error.hint(),
            }))
            .collect()
    }
}

#[cfg(test)]
mod tests;
