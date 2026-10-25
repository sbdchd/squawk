use std::{borrow::Cow, cmp::Ordering, collections::hash_map::Entry, range::Range};

use la_arena::Idx;
use rowan::{TextRange, TextSize};
use rustc_hash::{FxBuildHasher, FxHashMap, FxHashSet};
use smallvec::{SmallVec, smallvec};
use squawk_syntax::{
    AstPtr, SyntaxKind, SyntaxNode, SyntaxNodePtr, SyntaxToken,
    ast::{self, AstNode, NameLike},
    column_name::ColumnName,
    sql_body::SqlBody,
};

use crate::arena::{IdxRange, LayeredArena};
use crate::comments::preceding_comment;
use crate::extensions::BundledExtension;
use crate::file::{FileId, InFile, InIrFile, IrFileId, IrFileRange};
use crate::interner::Name;
use crate::keyed_index::GraphElementIndex;

use crate::{
    ir::{
        AccessMethod, AccessMethodId, AccessMethodKind, Body, Case, Cast, CastContext, CastId,
        CastTypes, Channel, ChannelId, Collation, CollationId, Column, ColumnId, ColumnOwner,
        ColumnTypeRef, Columns, Constraint, ConstraintId, ConstraintKind, ConstraintOwner,
        Conversion, ConversionId, Cte, CteId, Cursor, CursorId, Database, DatabaseId, EnumLabel,
        EnumLabelId, EventTrigger, EventTriggerId, Extension, ExtensionId, ForeignDataWrapper,
        ForeignDataWrapperId, Function, FunctionId, FunctionKind, GraphElement, GraphElementId,
        GraphElementKind, GraphLabel, GraphLabelId, GraphProperty, GraphPropertyId, Index, IndexId,
        Item, ItemRange, JsonPath, JsonPathId, Language, LanguageId, MemberKind, MemberOwner,
        Merge, Operand, Operands, Operator, OperatorClass, OperatorClassId, OperatorFamily,
        OperatorFamilyId, OperatorFamilyMember, OperatorFamilyMemberId, OperatorId, Param, ParamId,
        ParamMode, Policy, PolicyId, PreparedStatement, PreparedStatementId, PreparedTransaction,
        PreparedTransactionId, Publication, PublicationId, Query, QueryId, QueryKind, RefId,
        Reference, ReferenceKind, Resolution, Role, RoleId, RoutineKind, Rule, RuleEvent, RuleId,
        Savepoint, SavepointId, Schema, SchemaId, Scope, ScopeId, SearchPathChange, SequenceOwner,
        Server, ServerId, Source, SourceId, SourceKind, Star, Statistics, StatisticsId, Subscript,
        Subscription, SubscriptionId, Table, TableId, TableKind, Tablespace, TablespaceId,
        TextSearch, TextSearchId, TextSearchKind, Transaction, TransactionId, Transform,
        TransformId, TransformKey, Trigger, TriggerId, Ty, Type, TypeCategory, TypeId, TypeKind,
        Window, WindowClauses, WindowId,
    },
    sema_error::{ObjectKind, SemaError, TriggerFunctionKind, WindowClause},
    semantic_index::{IndexData, NameVersion},
};
use crate::{
    overload::{ArgType, BuiltinTypes, CallArg, Overloads},
    postgres,
};

impl Lowerer {
    pub(crate) fn new(mut index: IndexData) -> Lowerer {
        let public = index.names.intern("public");
        let pg_catalog = index.names.intern("pg_catalog");
        let pg_temp = index.names.intern("pg_temp");
        let dollar_user = index.names.intern("$user");
        let default_search_path = smallvec![dollar_user, public];
        let builtin_types = BuiltinTypes::new(&mut index, pg_catalog);
        Lowerer {
            index,
            file_id: FileId::Builtins.into(),
            ctes: vec![],
            pending_ctes: vec![],
            scopes: vec![],
            windows: vec![],
            json_paths: vec![],
            expr_context: ExprContext::Unrestricted,
            aggregate_calls: vec![],
            generated_refs: vec![],
            function: None,
            public,
            pg_catalog,
            pg_temp,
            dollar_user,
            search_path: default_search_path.clone(),
            session_search_path: default_search_path,
            transaction: None,
            savepoints: vec![],
            cursors: FxHashMap::default(),
            prepared_transactions: FxHashMap::default(),
            prepared_statements: FxHashMap::default(),
            channels: FxHashMap::default(),
            prepare_params: None,
            dependencies: DependencyGraph::default(),
            inheritances: InheritanceGraph::default(),
            column_exprs: ColumnExprs::default(),
            next_column_expr: 0,
            dropped_operator_family_members: FxHashSet::default(),
            undo: vec![],
            dropped_types: FxHashSet::default(),
            dropped_tables: FxHashSet::default(),
            dropped_indexes: FxHashSet::default(),
            dropped_constraints: FxHashSet::default(),
            dropped_triggers: FxHashSet::default(),
            dropped_rules: FxHashSet::default(),
            dropped_roles: FxHashSet::default(),
            dropped_policies: FxHashSet::default(),
            dropped_access_methods: FxHashSet::default(),
            dropped_event_triggers: FxHashSet::default(),
            dropped_languages: FxHashSet::default(),
            dropped_foreign_data_wrappers: FxHashSet::default(),
            dropped_servers: FxHashSet::default(),
            dropped_conversions: FxHashSet::default(),
            dropped_operator_families: FxHashSet::default(),
            dropped_operator_classes: FxHashSet::default(),
            dropped_statistics: FxHashSet::default(),
            dropped_text_search: FxHashSet::default(),
            dropped_publications: FxHashSet::default(),
            dropped_subscriptions: FxHashSet::default(),
            dropped_graph_elements: FxHashSet::default(),
            dropped_graph_labels: FxHashSet::default(),
            dropped_graph_properties: FxHashSet::default(),
            dropped_collations: FxHashSet::default(),
            dropped_schemas: FxHashSet::default(),
            dropped_tablespaces: FxHashSet::default(),
            dropped_databases: FxHashSet::default(),
            dropped_extensions: FxHashSet::default(),
            dropped_operators: FxHashSet::default(),
            dropped_casts: FxHashSet::default(),
            dropped_functions: FxHashSet::default(),
            builtin_types,
            transforms: FxHashMap::default(),
        }
    }

    pub(crate) fn lower_file(&mut self, file: InFile<&ast::SourceFile>) {
        self.file_id = file.file_id.into();
        self.reset_search_path();
        self.transaction = None;
        self.savepoints.clear();
        self.cursors.clear();
        self.prepared_statements.clear();
        self.channels.clear();
        self.undo.clear();
        for stmt in file.value.stmts() {
            self.lower_stmt(&stmt);
        }
    }

    pub(crate) fn finish(self) -> IndexData {
        self.index
    }
}

impl IndexData {
    fn record_rename(&mut self, item: Item, previous: IrFileRange, next: IrFileRange) {
        if previous == next {
            return;
        }
        let until = self.references.next_idx();
        self.renames.entry(item).or_default().push(NameVersion {
            until,
            name: previous,
        });
    }

    fn set_column_owner(&mut self, columns: IdxRange<Column>, owner: ColumnOwner) {
        for column in columns {
            self.column_owners.insert(column, owner);
        }
    }

    fn set_param_function(&mut self, params: IdxRange<Param>, function: FunctionId) {
        for param in params {
            self.param_functions.insert(param, function);
        }
    }
}

struct QueryScope {
    query: QueryId,
    scope: ScopeId,
    merges: Vec<ScopeMerge>,
    windows: Range<usize>,
}

#[derive(Clone, Copy)]
enum LabelPosition {
    End,
    Before(EnumLabelId),
    After(EnumLabelId),
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum CalleeKind {
    Plain,
    OrderedSet,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum WindowSpecKind {
    WindowClause,
    OverClause,
}

#[derive(Clone, Copy)]
enum JoinKind {
    Inner,
    Left,
    Right,
    Full,
}

struct MergedColumn {
    resolution: Resolution,
    inputs: Option<MergeInputs>,
}

#[derive(Clone, Copy)]
struct MergeInputs {
    left: ColumnId,
    right: ColumnId,
}

enum MergeSide {
    Left,
    Right,
    Both,
}

struct ScopeMerge {
    name: Name,
    column: ColumnId,
    sources: Range<usize>,
}

struct JoinColumns<'a> {
    join_merges: &'a [ScopeMerge],
    from_sources: &'a [SourceId],
    source_arena: &'a LayeredArena<Source>,
    column_arena: &'a mut LayeredArena<Column>,
}

impl JoinColumns<'_> {
    fn expand_from(
        &mut self,
        start: usize,
        mut f: impl FnMut(&mut LayeredArena<Column>, ColumnId),
    ) {
        self.expand((start..self.from_sources.len()).into(), &mut f);
    }

    fn expand(
        &mut self,
        range: Range<usize>,
        f: &mut impl FnMut(&mut LayeredArena<Column>, ColumnId),
    ) {
        for merge in self.join_merges {
            if merge.sources == range && !self.is_shadowed(merge) {
                f(self.column_arena, merge.column);
            }
        }
        let mut index = range.start;
        while index < range.end {
            let nested_end = self
                .join_merges
                .iter()
                .filter(|merge| {
                    merge.sources.start == index
                        && merge.sources.end <= range.end
                        && merge.sources != range
                })
                .map(|merge| merge.sources.end)
                .max();
            if let Some(end) = nested_end {
                self.expand((index..end).into(), f);
                index = end;
                continue;
            }
            let source = &self.source_arena[self.from_sources[index]];
            if !source.kind.is_qualified_only() {
                for column in source.columns.range {
                    if !self.is_merged(index, self.column_arena[column].name) {
                        f(self.column_arena, column);
                    }
                }
            }
            index += 1;
        }
    }

    fn is_shadowed(&self, merge: &ScopeMerge) -> bool {
        self.join_merges.iter().any(|outer| {
            outer.name == merge.name
                && outer.sources != merge.sources
                && outer.sources.start <= merge.sources.start
                && merge.sources.end <= outer.sources.end
        })
    }

    fn is_merged(&self, source: usize, name: Option<Name>) -> bool {
        name.is_some_and(|name| {
            self.join_merges
                .iter()
                .any(|merge| merge.name == name && merge.sources.contains(&source))
        })
    }
}

enum ColumnLookup {
    Found(Resolution),
    Ambiguous,
    Missing { complete: bool },
}

enum ScopedColumn {
    Resolved(Resolution),
    Hidden,
}

pub(crate) struct Lowerer {
    index: IndexData,
    file_id: IrFileId,
    ctes: Vec<CteId>,
    pending_ctes: Vec<PendingCte>,
    scopes: Vec<QueryScope>,
    windows: Vec<WindowId>,
    json_paths: Vec<JsonPathId>,
    expr_context: ExprContext,
    aggregate_calls: Vec<AggregateCall>,
    generated_refs: Vec<GeneratedRefs>,
    function: Option<FunctionId>,
    public: Name,
    pg_catalog: Name,
    pg_temp: Name,
    dollar_user: Name,
    search_path: SmallVec<[Name; 4]>,
    session_search_path: SmallVec<[Name; 4]>,
    transaction: Option<OpenTransaction>,
    savepoints: Vec<OpenSavepoint>,
    cursors: FxHashMap<Name, CursorId>,
    prepared_transactions: FxHashMap<Name, PreparedTransactionId>,
    prepared_statements: FxHashMap<Name, PreparedStatementId>,
    channels: FxHashMap<Name, ChannelId>,
    prepare_params: Option<IdxRange<Param>>,
    dependencies: DependencyGraph,
    inheritances: InheritanceGraph,
    column_exprs: ColumnExprs,
    next_column_expr: u32,
    dropped_operator_family_members: FxHashSet<OperatorFamilyMemberId>,
    undo: Vec<Undo>,
    dropped_types: FxHashSet<TypeId>,
    dropped_tables: FxHashSet<TableId>,
    dropped_indexes: FxHashSet<IndexId>,
    dropped_constraints: FxHashSet<ConstraintId>,
    dropped_triggers: FxHashSet<TriggerId>,
    dropped_rules: FxHashSet<RuleId>,
    dropped_roles: FxHashSet<RoleId>,
    dropped_policies: FxHashSet<PolicyId>,
    dropped_access_methods: FxHashSet<AccessMethodId>,
    dropped_event_triggers: FxHashSet<EventTriggerId>,
    dropped_languages: FxHashSet<LanguageId>,
    dropped_foreign_data_wrappers: FxHashSet<ForeignDataWrapperId>,
    dropped_servers: FxHashSet<ServerId>,
    dropped_conversions: FxHashSet<ConversionId>,
    dropped_operator_families: FxHashSet<OperatorFamilyId>,
    dropped_operator_classes: FxHashSet<OperatorClassId>,
    dropped_statistics: FxHashSet<StatisticsId>,
    dropped_text_search: FxHashSet<TextSearchId>,
    dropped_publications: FxHashSet<PublicationId>,
    dropped_subscriptions: FxHashSet<SubscriptionId>,
    dropped_graph_elements: FxHashSet<GraphElementId>,
    dropped_graph_labels: FxHashSet<GraphLabelId>,
    dropped_graph_properties: FxHashSet<GraphPropertyId>,
    dropped_collations: FxHashSet<CollationId>,
    dropped_schemas: FxHashSet<SchemaId>,
    dropped_tablespaces: FxHashSet<TablespaceId>,
    dropped_databases: FxHashSet<DatabaseId>,
    dropped_extensions: FxHashSet<ExtensionId>,
    dropped_operators: FxHashSet<OperatorId>,
    dropped_casts: FxHashSet<CastId>,
    dropped_functions: FxHashSet<FunctionId>,
    builtin_types: BuiltinTypes,
    transforms: FxHashMap<TransformKey, TransformId>,
}

struct OpenTransaction {
    id: Option<TransactionId>,
    snapshot: Snapshot,
}

struct OpenSavepoint {
    id: SavepointId,
    snapshot: Snapshot,
}

struct Snapshot {
    search_path: SmallVec<[Name; 4]>,
    session_search_path: SmallVec<[Name; 4]>,
    undo_len: usize,
    tables: TableId,
    types: TypeId,
    indexes: IndexId,
    constraints: ConstraintId,
    triggers: TriggerId,
    rules: RuleId,
    roles: RoleId,
    policies: PolicyId,
    access_methods: AccessMethodId,
    event_triggers: EventTriggerId,
    languages: LanguageId,
    foreign_data_wrappers: ForeignDataWrapperId,
    servers: ServerId,
    conversions: ConversionId,
    operator_families: OperatorFamilyId,
    operator_classes: OperatorClassId,
    statistics: StatisticsId,
    text_search: TextSearchId,
    publications: PublicationId,
    subscriptions: SubscriptionId,
    graph_elements: GraphElementId,
    graph_labels: GraphLabelId,
    enum_labels: EnumLabelId,
    graph_properties: GraphPropertyId,
    collations: CollationId,
    tablespaces: TablespaceId,
    databases: DatabaseId,
    schemas: SchemaId,
    extensions: ExtensionId,
    operators: OperatorId,
    functions: FunctionId,
    casts: CastId,
    transforms: TransformId,
    cursors: CursorId,
    operator_family_members: OperatorFamilyMemberId,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct ColumnExprId(u32);

#[derive(Clone, Copy, PartialEq, Eq)]
enum ColumnExprKind {
    Default,
    Generated,
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct ColumnExpr {
    id: ColumnExprId,
    kind: ColumnExprKind,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct ColumnKey {
    table: TableId,
    column: Name,
}

#[derive(Default)]
struct ColumnExprs {
    by_column: FxHashMap<ColumnKey, ColumnExpr>,
    by_id: FxHashMap<ColumnExprId, ColumnKey>,
}

impl ColumnExprs {
    fn get(&self, key: ColumnKey) -> Option<ColumnExpr> {
        self.by_column.get(&key).copied()
    }

    fn column_of(&self, id: ColumnExprId) -> Option<ColumnKey> {
        self.by_id.get(&id).copied()
    }

    fn set(&mut self, key: ColumnKey, expression: Option<ColumnExpr>) -> Option<ColumnExpr> {
        let previous = match expression {
            Some(expression) => self.by_column.insert(key, expression),
            None => self.by_column.remove(&key),
        };
        if let Some(previous) = previous {
            self.by_id.remove(&previous.id);
        }
        if let Some(expression) = expression {
            self.by_id.insert(expression.id, key);
        }
        previous
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum TypedColumnOwner {
    Table(TableId),
    Type(TypeId),
}

#[derive(Clone, Copy)]
struct TypedColumn {
    owner: TypedColumnOwner,
    name: Name,
}

#[derive(Clone, Copy)]
struct FunctionParams {
    params: IdxRange<Param>,
    direct_args: Option<usize>,
}

#[derive(Clone, Copy)]
struct QualifiedColumn {
    table: Option<TableId>,
    column: Option<ColumnId>,
}

struct GeneratedRefs {
    table: TableId,
    refs: IdxRange<Reference>,
}

struct PendingCte {
    id: CteId,
    with_table: ast::WithTable,
    scopes: usize,
    ctes: usize,
}

struct RelationSource {
    name: Option<Name>,
    kind: SourceKind,
    columns: Columns,
}

struct IndexColumns {
    columns: SmallVec<[Name; 4]>,
    key_len: usize,
}

#[derive(Clone, Copy)]
enum TypeLookup {
    Found(Ty),
    Missing,
    Unknown,
}

impl TypeLookup {
    fn ty(self) -> Option<Ty> {
        match self {
            TypeLookup::Found(ty) => Some(ty),
            TypeLookup::Missing | TypeLookup::Unknown => None,
        }
    }
}

struct FunctionSource {
    callee: Option<Callee>,
    function: Option<FunctionId>,
    columns: Columns,
}

struct LoweredCall {
    args: SmallVec<[CallArg; 4]>,
    callee: Option<Callee>,
}

#[derive(Clone, Copy)]
struct Callee {
    name: Name,
    resolution: Resolution,
}

struct CallArgs {
    args: SmallVec<[CallArg; 4]>,
    named: Vec<NamedArg>,
}

struct NamedArg {
    name: Name,
    name_ref: ast::ParamNameRef,
}

#[derive(Clone, Copy)]
struct NameResolution {
    kind: ReferenceKind,
    resolution: Resolution,
}

#[derive(Clone, Copy)]
struct SubscriptResults {
    result: Option<Ty>,
    slice: Option<Ty>,
}

struct IdentPart<'a> {
    ident: Cow<'a, str>,
    rest: &'a str,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum InheritanceKind {
    Inherits,
    Partition,
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Inheritance {
    parent: TableId,
    child: TableId,
    kind: InheritanceKind,
}

#[derive(Default)]
struct InheritanceGraph {
    by_parent: FxHashMap<TableId, SmallVec<[Inheritance; 2]>>,
    by_child: FxHashMap<TableId, SmallVec<[Inheritance; 2]>>,
}

impl InheritanceGraph {
    fn add(&mut self, inheritance: Inheritance) -> bool {
        let parents = self.by_child.entry(inheritance.child).or_default();
        if parents.contains(&inheritance) {
            return false;
        }
        parents.push(inheritance);
        self.by_parent
            .entry(inheritance.parent)
            .or_default()
            .push(inheritance);
        true
    }

    fn remove(&mut self, inheritance: Inheritance) -> bool {
        Self::remove_from(&mut self.by_child, inheritance.child, inheritance)
            && Self::remove_from(&mut self.by_parent, inheritance.parent, inheritance)
    }

    fn remove_from(
        map: &mut FxHashMap<TableId, SmallVec<[Inheritance; 2]>>,
        table: TableId,
        inheritance: Inheritance,
    ) -> bool {
        let Entry::Occupied(mut entry) = map.entry(table) else {
            return false;
        };
        let Some(position) = entry
            .get()
            .iter()
            .position(|&existing| existing == inheritance)
        else {
            return false;
        };
        entry.get_mut().remove(position);
        if entry.get().is_empty() {
            entry.remove();
        }
        true
    }

    fn children(&self, parent: TableId) -> &[Inheritance] {
        self.by_parent
            .get(&parent)
            .map_or(&[], |children| children.as_slice())
    }

    fn parents(&self, child: TableId) -> &[Inheritance] {
        self.by_child
            .get(&child)
            .map_or(&[], |parents| parents.as_slice())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Object {
    Function(FunctionId),
    ColumnExpr(ColumnExprId),
    OperatorFamilyMember(OperatorFamilyMemberId),
    Table(TableId),
    Type(TypeId),
    Trigger(TriggerId),
    EventTrigger(EventTriggerId),
    Index(IndexId),
    Constraint(ConstraintId),
    Policy(PolicyId),
    Rule(RuleId),
    Cast(CastId),
    Transform(TransformId),
    Operator(OperatorId),
    AccessMethod(AccessMethodId),
    Language(LanguageId),
    ForeignDataWrapper(ForeignDataWrapperId),
    Conversion(ConversionId),
    TextSearch(TextSearchId),
    Collation(CollationId),
    OperatorFamily(OperatorFamilyId),
    OperatorClass(OperatorClassId),
    Statistics(StatisticsId),
    Extension(ExtensionId),
    Schema(SchemaId),
    Server(ServerId),
    Publication(PublicationId),
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Dependency {
    dependent: Object,
    referenced: Object,
}

#[derive(Default)]
struct DependencyGraph {
    edges: FxHashSet<Dependency>,
    by_referenced: FxHashMap<Object, SmallVec<[Object; 2]>>,
    moved_from: FxHashMap<Object, SmallVec<[Object; 1]>>,
    moved_to: FxHashMap<Object, SmallVec<[Object; 1]>>,
    replaced: FxHashMap<FunctionId, SmallVec<[FunctionId; 1]>>,
}

impl DependencyGraph {
    fn add_dependency(&mut self, dependent: Object, referenced: Object) {
        if !self.edges.insert(Dependency {
            dependent,
            referenced,
        }) {
            return;
        }
        self.by_referenced
            .entry(referenced)
            .or_default()
            .push(dependent);
    }

    fn remove_dependency(&mut self, dependent: Object, referenced: Object) -> bool {
        if !self.edges.remove(&Dependency {
            dependent,
            referenced,
        }) {
            return false;
        }
        let Entry::Occupied(mut entry) = self.by_referenced.entry(referenced) else {
            return false;
        };
        let Some(position) = entry
            .get()
            .iter()
            .position(|&existing| existing == dependent)
        else {
            return false;
        };
        entry.get_mut().remove(position);
        if entry.get().is_empty() {
            entry.remove();
        }
        true
    }

    fn add_move(&mut self, from: Object, to: Object) {
        debug_assert!(from < to);
        self.moved_from.entry(to).or_default().push(from);
        self.moved_to.entry(from).or_default().push(to);
    }

    fn add_replacement(&mut self, old: FunctionId, new: FunctionId) {
        self.replaced.entry(new).or_default().push(old);
    }

    fn dependents_of(&self, referenced: Object) -> &[Object] {
        self.by_referenced
            .get(&referenced)
            .map_or(&[], |dependents| dependents.as_slice())
    }

    fn moved_from(&self, to: Object) -> &[Object] {
        self.moved_from.get(&to).map_or(&[], |from| from.as_slice())
    }

    fn moved_to(&self, from: Object) -> &[Object] {
        self.moved_to.get(&from).map_or(&[], |to| to.as_slice())
    }

    fn replaced_by(&self, new: FunctionId) -> &[FunctionId] {
        self.replaced.get(&new).map_or(&[], |old| old.as_slice())
    }
}

impl Object {
    fn item(self) -> Option<Item> {
        match self {
            Object::Function(id) => Some(Item::Function(id)),
            Object::Table(id) => Some(Item::Table(id)),
            Object::Type(id) => Some(Item::Type(id)),
            Object::Trigger(id) => Some(Item::Trigger(id)),
            Object::EventTrigger(id) => Some(Item::EventTrigger(id)),
            Object::Index(id) => Some(Item::Index(id)),
            Object::Constraint(id) => Some(Item::Constraint(id)),
            Object::Policy(id) => Some(Item::Policy(id)),
            Object::Rule(id) => Some(Item::Rule(id)),
            Object::Operator(id) => Some(Item::Operator(id)),
            Object::AccessMethod(id) => Some(Item::AccessMethod(id)),
            Object::Language(id) => Some(Item::Language(id)),
            Object::ForeignDataWrapper(id) => Some(Item::ForeignDataWrapper(id)),
            Object::Conversion(id) => Some(Item::Conversion(id)),
            Object::TextSearch(id) => Some(Item::TextSearch(id)),
            Object::Collation(id) => Some(Item::Collation(id)),
            Object::OperatorFamily(id) => Some(Item::OperatorFamily(id)),
            Object::OperatorClass(id) => Some(Item::OperatorClass(id)),
            Object::Statistics(id) => Some(Item::Statistics(id)),
            Object::Extension(id) => Some(Item::Extension(id)),
            Object::Schema(id) => Some(Item::Schema(id)),
            Object::Server(id) => Some(Item::Server(id)),
            Object::Publication(id) => Some(Item::Publication(id)),
            Object::ColumnExpr(_)
            | Object::OperatorFamilyMember(_)
            | Object::Cast(_)
            | Object::Transform(_) => None,
        }
    }
}

enum Undo {
    DropTable(TableId),
    DropType(TypeId),
    DropIndex(IndexId),
    DropConstraint(ConstraintId),
    DropTrigger(TriggerId),
    DropRule(RuleId),
    DropRole(RoleId),
    DropPolicy(PolicyId),
    DropAccessMethod(AccessMethodId),
    DropEventTrigger(EventTriggerId),
    DropLanguage(LanguageId),
    DropForeignDataWrapper(ForeignDataWrapperId),
    DropServer(ServerId),
    DropConversion(ConversionId),
    DropFunction(FunctionId),
    DropSchema(SchemaId),
    RenameSchema(Name, Name),
    DropOperatorFamily(OperatorFamilyId),
    DropOperatorClass(OperatorClassId),
    DropStatistics(StatisticsId),
    DropTextSearch(TextSearchId),
    DropPublication(PublicationId),
    DropSubscription(SubscriptionId),
    DropGraphElement(GraphElementId),
    DropGraphLabel(GraphLabelId),
    DropGraphProperty(GraphPropertyId),
    DropCollation(CollationId),
    DropTablespace(TablespaceId),
    DropDatabase(DatabaseId),
    DropExtension(ExtensionId),
    DropOperator(OperatorId),
    DropCast(CastId),
    DropTransform(TransformKey, TransformId),
    TableColumns(TableId, Columns),
    ColumnExpr(ColumnKey, Option<ColumnExpr>),
    DropOperatorFamilyMember(OperatorFamilyMemberId),
    MemberOwner(OperatorFamilyMemberId, MemberOwner),
    MoveTable(TableId, Name, Name, InIrFile<SyntaxNodePtr>),
    MoveIndex(IndexId, Name, Name, InIrFile<SyntaxNodePtr>),
    MoveEnumLabel(EnumLabelId, Name, InIrFile<SyntaxNodePtr>),
    MoveExtension(ExtensionId, Name),
    MoveObjectSchema(Object, Name),
    RemoveDependency(Object, Object),
    AddInheritance(Inheritance),
    RemoveInheritance(Inheritance),
    SequenceOwner(TableId, SequenceOwner),
    ConstraintKind(ConstraintId, ConstraintKind),
    TypeColumns(TypeId, Columns),
    TableOfType(TableId, Option<TypeId>),
    TypeSubscript(TypeId, Option<Subscript>, SubscriptResults),
    FinishShellType(TypeId, Box<Type>),
    MoveType(TypeId, Name, Name, InIrFile<AstPtr<ast::PathSegment>>),
}

enum FieldBase {
    Source(Option<SourceId>),
    Expr(Option<Ty>),
    Columns { columns: Columns, ty: Option<Ty> },
}

struct FieldStar {
    ty: Option<Ty>,
    columns: Option<Columns>,
}

#[derive(Clone, Copy)]
enum Lookup {
    Relation,
    Type,
    Function,
    Operator,
    Collation,
    Conversion,
    OperatorClass,
    Statistics,
    TextSearch,
}

impl Lookup {
    fn searches_temp(self) -> bool {
        match self {
            Lookup::Relation | Lookup::Type | Lookup::Statistics => true,
            Lookup::Function
            | Lookup::Operator
            | Lookup::Collation
            | Lookup::Conversion
            | Lookup::OperatorClass
            | Lookup::TextSearch => false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TableLookup {
    Table,
    ForeignTable,
    TableOrForeignTable,
    TableOrView,
    TableOrMaterializedView,
    Any,
    View,
    MaterializedView,
    Sequence,
    PropertyGraph,
    CopyTo,
    CopyFrom,
    Truncate,
}

impl TableLookup {
    fn accepts(self, kind: TableKind) -> bool {
        match self {
            TableLookup::Table => kind == TableKind::Table,
            TableLookup::ForeignTable => kind == TableKind::ForeignTable,
            TableLookup::TableOrForeignTable => {
                matches!(kind, TableKind::Table | TableKind::ForeignTable)
            }
            TableLookup::TableOrView => matches!(kind, TableKind::Table | TableKind::View),
            TableLookup::TableOrMaterializedView => {
                matches!(kind, TableKind::Table | TableKind::MaterializedView)
            }
            TableLookup::Any => true,
            TableLookup::View => kind == TableKind::View,
            TableLookup::MaterializedView => kind == TableKind::MaterializedView,
            TableLookup::Sequence => kind == TableKind::Sequence,
            TableLookup::PropertyGraph => kind == TableKind::PropertyGraph,
            TableLookup::CopyTo => matches!(kind, TableKind::Table | TableKind::MaterializedView),
            TableLookup::CopyFrom | TableLookup::Truncate => {
                matches!(kind, TableKind::Table | TableKind::ForeignTable)
            }
        }
    }
}

#[derive(Clone, Copy)]
enum RelationLookup {
    Any,
    OnlyTable,
}

#[derive(Clone, Copy)]
enum CteKind {
    NonRecursive,
    Recursive,
}

enum MultirangeName {
    Default,
    Path(ast::PathRef),
    String(String),
}

#[derive(Clone, Copy)]
enum IfMissing {
    Error,
    Ignore,
}

impl IfMissing {
    fn from_if_exists(if_exists: Option<&ast::IfExists>) -> IfMissing {
        match if_exists {
            Some(_) => IfMissing::Ignore,
            None => IfMissing::Error,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Persistence {
    Permanent,
    Temp,
}

impl Persistence {
    fn from_ast(persistence: Option<&ast::Persistence>) -> Persistence {
        match persistence {
            Some(ast::Persistence::Temp(_)) => Persistence::Temp,
            _ => Persistence::Permanent,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum IfExists {
    Error,
    Ignore,
    Replace,
}

impl IfExists {
    fn from_if_not_exists(if_not_exists: Option<&ast::IfNotExists>) -> IfExists {
        match if_not_exists {
            Some(_) => IfExists::Ignore,
            None => IfExists::Error,
        }
    }

    fn from_or_replace(or_replace: Option<&ast::OrReplace>) -> IfExists {
        match or_replace {
            Some(_) => IfExists::Replace,
            None => IfExists::Error,
        }
    }
}

#[derive(Clone, Copy)]
enum SearchPathScope {
    Session,
    Transaction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OutputClause {
    OrderBy,
    GroupBy,
    DistinctOn,
}

struct SelectParts {
    with_clause: Option<ast::WithClause>,
    select_clause: Option<ast::SelectClause>,
    from_clause: Option<ast::FromClause>,
    where_clause: Option<ast::WhereClause>,
    group_by_clause: Option<ast::GroupByClause>,
    having_clause: Option<ast::HavingClause>,
    window_clause: Option<ast::WindowClause>,
    tail_clauses: ast::AstChildren<ast::SelectTailClause>,
}

impl Lowerer {
    fn lower_stmt(&mut self, stmt: &ast::Stmt) {
        self.record_search_path(stmt.syntax().text_range().start());
        self.aggregate_calls.clear();
        match stmt {
            ast::Stmt::CreateTable(create_table) => self.lower_create_table(
                create_table,
                TableKind::Table,
                IfExists::from_if_not_exists(create_table.if_not_exists().as_ref()),
                create_table.partition_of(),
                create_table.partition_type(),
                create_table.tablespace_clause(),
                create_table.using_method(),
                create_table.partition_by(),
                create_table.of_type(),
            ),
            ast::Stmt::CreateForeignTable(create_table) => {
                self.lower_create_foreign_table(create_table);
            }
            ast::Stmt::CreateForeignDataWrapper(create_wrapper) => {
                self.lower_create_foreign_data_wrapper(create_wrapper);
            }
            ast::Stmt::AlterForeignDataWrapper(alter_wrapper) => {
                self.lower_alter_foreign_data_wrapper(alter_wrapper);
            }
            ast::Stmt::DropForeignDataWrapper(drop_wrapper) => {
                self.lower_drop_foreign_data_wrapper(drop_wrapper);
            }
            ast::Stmt::CreateServer(create_server) => self.lower_create_server(create_server),
            ast::Stmt::CreateOperatorFamily(create_family) => {
                self.lower_create_operator_family(create_family);
            }
            ast::Stmt::AlterOperatorFamily(alter_family) => {
                self.lower_alter_operator_family(alter_family);
            }
            ast::Stmt::DropOperatorFamily(drop_family) => {
                self.lower_drop_operator_family(drop_family);
            }
            ast::Stmt::CreateOperatorClass(create_class) => {
                self.lower_create_operator_class(create_class);
            }
            ast::Stmt::AlterOperatorClass(alter_class) => {
                self.lower_alter_operator_class(alter_class);
            }
            ast::Stmt::DropOperatorClass(drop_class) => {
                self.lower_drop_operator_class(drop_class);
            }
            ast::Stmt::CreateTextSearchParser(create_parser) => {
                self.lower_create_text_search_parser(create_parser);
            }
            ast::Stmt::CreateTextSearchTemplate(create_template) => {
                self.lower_create_text_search_template(create_template);
            }
            ast::Stmt::CreateTextSearchDictionary(create_dictionary) => {
                self.lower_create_text_search_dictionary(create_dictionary);
            }
            ast::Stmt::CreateTextSearchConfiguration(create_configuration) => {
                self.lower_create_text_search_configuration(create_configuration);
            }
            ast::Stmt::AlterTextSearchParser(alter_parser) => {
                self.lower_alter_text_search_parser(alter_parser);
            }
            ast::Stmt::AlterTextSearchTemplate(alter_template) => {
                self.lower_alter_text_search_template(alter_template);
            }
            ast::Stmt::AlterTextSearchDictionary(alter_dictionary) => {
                self.lower_alter_text_search_dictionary(alter_dictionary);
            }
            ast::Stmt::AlterTextSearchConfiguration(alter_configuration) => {
                self.lower_alter_text_search_configuration(alter_configuration);
            }
            ast::Stmt::DropTextSearchParser(drop_parser) => {
                self.lower_drop_text_search_parser(drop_parser);
            }
            ast::Stmt::DropTextSearchTemplate(drop_template) => {
                self.lower_drop_text_search_template(drop_template);
            }
            ast::Stmt::DropTextSearchDict(drop_dictionary) => {
                self.lower_drop_text_search_dictionary(drop_dictionary);
            }
            ast::Stmt::DropTextSearchConfig(drop_configuration) => {
                self.lower_drop_text_search_configuration(drop_configuration);
            }
            ast::Stmt::CreatePropertyGraph(create_property_graph) => {
                self.lower_create_property_graph(create_property_graph);
            }
            ast::Stmt::AlterPropertyGraph(alter_property_graph) => {
                self.lower_alter_property_graph(alter_property_graph);
            }
            ast::Stmt::DropPropertyGraph(drop_property_graph) => {
                self.lower_drop_property_graph(drop_property_graph);
            }
            ast::Stmt::CreatePublication(create_publication) => {
                self.lower_create_publication(create_publication);
            }
            ast::Stmt::AlterPublication(alter_publication) => {
                self.lower_alter_publication(alter_publication);
            }
            ast::Stmt::DropPublication(drop_publication) => {
                self.lower_drop_publication(drop_publication);
            }
            ast::Stmt::CreateSubscription(create_subscription) => {
                self.lower_create_subscription(create_subscription);
            }
            ast::Stmt::AlterSubscription(alter_subscription) => {
                self.lower_alter_subscription(alter_subscription);
            }
            ast::Stmt::DropSubscription(drop_subscription) => {
                self.lower_drop_subscription(drop_subscription);
            }
            ast::Stmt::CreateStatistics(create_statistics) => {
                self.lower_create_statistics(create_statistics);
            }
            ast::Stmt::AlterStatistics(alter_statistics) => {
                self.lower_alter_statistics(alter_statistics);
            }
            ast::Stmt::DropStatistics(drop_statistics) => {
                self.lower_drop_statistics(drop_statistics);
            }
            ast::Stmt::CreateConversion(create_conversion) => {
                self.lower_create_conversion(create_conversion);
            }
            ast::Stmt::AlterConversion(alter_conversion) => {
                self.lower_alter_conversion(alter_conversion);
            }
            ast::Stmt::DropConversion(drop_conversion) => {
                self.lower_drop_conversion(drop_conversion);
            }
            ast::Stmt::AlterServer(alter_server) => self.lower_alter_server(alter_server),
            ast::Stmt::DropServer(drop_server) => self.lower_drop_server(drop_server),
            ast::Stmt::CreateUserMapping(create_user_mapping) => {
                self.lower_create_user_mapping(create_user_mapping);
            }
            ast::Stmt::AlterUserMapping(alter_user_mapping) => {
                self.lower_alter_user_mapping(alter_user_mapping);
            }
            ast::Stmt::DropUserMapping(drop_user_mapping) => {
                self.lower_drop_user_mapping(drop_user_mapping);
            }
            ast::Stmt::ImportForeignSchema(import) => self.lower_import_foreign_schema(import),
            ast::Stmt::CreateView(create_view) => self.lower_create_view(create_view),
            ast::Stmt::CreateMaterializedView(create_view) => {
                self.lower_create_materialized_view(create_view);
            }
            ast::Stmt::CreateTableAs(create_table_as) => {
                self.lower_create_table_as(create_table_as);
            }
            ast::Stmt::CreateType(create_type) => self.lower_create_type(create_type),
            ast::Stmt::CreateDomain(create_domain) => self.lower_create_domain(create_domain),
            ast::Stmt::DropType(drop_type) => self.lower_drop_type(drop_type),
            ast::Stmt::DropDomain(drop_domain) => self.lower_drop_domain(drop_domain),
            ast::Stmt::DropTable(drop_table) => self.lower_drop_table(drop_table),
            ast::Stmt::DropView(drop_view) => self.lower_drop_view(drop_view),
            ast::Stmt::DropMaterializedView(drop_view) => {
                self.lower_drop_materialized_view(drop_view);
            }
            ast::Stmt::AlterMaterializedView(alter_view) => {
                self.lower_alter_materialized_view(alter_view);
            }
            ast::Stmt::Refresh(refresh) => self.lower_refresh(refresh),
            ast::Stmt::CreateSequence(create_sequence) => {
                self.lower_create_sequence(create_sequence);
            }
            ast::Stmt::AlterSequence(alter_sequence) => self.lower_alter_sequence(alter_sequence),
            ast::Stmt::DropSequence(drop_sequence) => self.lower_drop_sequence(drop_sequence),
            ast::Stmt::AlterTable(alter_table) => self.lower_alter_table(alter_table),
            ast::Stmt::CreateFunction(create_function) => {
                self.lower_create_function(create_function);
            }
            ast::Stmt::CreateProcedure(create_procedure) => {
                self.lower_create_procedure(create_procedure);
            }
            ast::Stmt::CreateTrigger(create_trigger) => self.lower_create_trigger(create_trigger),
            ast::Stmt::DropTrigger(drop_trigger) => self.lower_drop_trigger(drop_trigger),
            ast::Stmt::AlterTrigger(alter_trigger) => self.lower_alter_trigger(alter_trigger),
            ast::Stmt::CreateRule(create_rule) => self.lower_create_rule(create_rule),
            ast::Stmt::DropRule(drop_rule) => self.lower_drop_rule(drop_rule),
            ast::Stmt::AlterRule(alter_rule) => self.lower_alter_rule(alter_rule),
            ast::Stmt::CreatePolicy(create_policy) => self.lower_create_policy(create_policy),
            ast::Stmt::CreateAccessMethod(create_access_method) => {
                self.lower_create_access_method(create_access_method);
            }
            ast::Stmt::DropAccessMethod(drop_access_method) => {
                self.lower_drop_access_method(drop_access_method);
            }
            ast::Stmt::CreateEventTrigger(create_event_trigger) => {
                self.lower_create_event_trigger(create_event_trigger);
            }
            ast::Stmt::AlterEventTrigger(alter_event_trigger) => {
                self.lower_alter_event_trigger(alter_event_trigger);
            }
            ast::Stmt::DropEventTrigger(drop_event_trigger) => {
                self.lower_drop_event_trigger(drop_event_trigger);
            }
            ast::Stmt::CreateLanguage(create_language) => {
                self.lower_create_language(create_language);
            }
            ast::Stmt::AlterLanguage(alter_language) => self.lower_alter_language(alter_language),
            ast::Stmt::DropLanguage(drop_language) => self.lower_drop_language(drop_language),
            ast::Stmt::Do(do_) => self.lower_do(do_),
            ast::Stmt::AlterPolicy(alter_policy) => self.lower_alter_policy(alter_policy),
            ast::Stmt::DropPolicy(drop_policy) => self.lower_drop_policy(drop_policy),
            ast::Stmt::CreateRole(create_role) => self.lower_create_role(create_role),
            ast::Stmt::CreateUser(create_user) => self.lower_create_user(create_user),
            ast::Stmt::CreateGroup(create_group) => self.lower_create_group(create_group),
            ast::Stmt::AlterRole(alter_role) => self.lower_alter_role(alter_role),
            ast::Stmt::AlterUser(alter_user) => self.lower_alter_user(alter_user),
            ast::Stmt::AlterGroup(alter_group) => self.lower_alter_group(alter_group),
            ast::Stmt::DropRole(drop_role) => self.lower_drop_role(drop_role),
            ast::Stmt::DropUser(drop_user) => self.lower_drop_user(drop_user),
            ast::Stmt::DropGroup(drop_group) => self.lower_drop_group(drop_group),
            ast::Stmt::SetRole(set_role) => self.lower_set_role(set_role),
            ast::Stmt::SetSessionAuth(set_session_auth) => {
                self.lower_set_session_auth(set_session_auth);
            }
            ast::Stmt::DropOwned(drop_owned) => self.lower_drop_owned(drop_owned),
            ast::Stmt::Reassign(reassign) => self.lower_reassign(reassign),
            ast::Stmt::CreateAggregate(create_aggregate) => {
                self.lower_create_aggregate(create_aggregate);
            }
            ast::Stmt::Select(select) => {
                self.lower_simple_select(select);
            }
            ast::Stmt::SelectInto(select) => self.lower_select_into_stmt(select),
            ast::Stmt::CompoundSelect(compound) => {
                self.lower_compound(compound, None);
            }
            ast::Stmt::ParenSelect(paren) => {
                self.lower_paren_select(paren, None);
            }
            ast::Stmt::Table(table) => {
                self.lower_table_query(table);
            }
            ast::Stmt::Values(values) => {
                self.lower_values(values);
            }
            ast::Stmt::Set(set) => self.lower_set(set),
            ast::Stmt::Reset(reset) => self.lower_reset(reset),
            ast::Stmt::Begin(begin) => self.lower_begin(begin),
            ast::Stmt::Commit(commit) => self.lower_commit(commit),
            ast::Stmt::Rollback(rollback) => self.lower_rollback(rollback),
            ast::Stmt::PrepareTransaction(prepare) => self.lower_prepare_transaction(prepare),
            ast::Stmt::SavepointCreate(savepoint) => self.lower_savepoint(savepoint),
            ast::Stmt::ReleaseSavepoint(release) => self.lower_release_savepoint(release),
            ast::Stmt::Delete(delete) => {
                self.lower_delete(delete);
            }
            ast::Stmt::Insert(insert) => {
                self.lower_insert(insert);
            }
            ast::Stmt::Update(update) => {
                self.lower_update(update);
            }
            ast::Stmt::Merge(merge) => {
                self.lower_merge(merge);
            }
            ast::Stmt::CreateIndex(create_index) => self.lower_create_index(create_index),
            ast::Stmt::DropIndex(drop_index) => self.lower_drop_index(drop_index),
            ast::Stmt::AlterIndex(alter_index) => self.lower_alter_index(alter_index),
            ast::Stmt::Reindex(reindex) => self.lower_reindex(reindex),
            ast::Stmt::Cluster(cluster) => self.lower_cluster(cluster),
            ast::Stmt::SetConstraints(set_constraints) => {
                self.lower_set_constraints(set_constraints);
            }
            ast::Stmt::CreateCollation(create_collation) => {
                self.lower_create_collation(create_collation);
            }
            ast::Stmt::AlterCollation(alter_collation) => {
                self.lower_alter_collation(alter_collation);
            }
            ast::Stmt::DropCollation(drop_collation) => self.lower_drop_collation(drop_collation),
            ast::Stmt::Declare(declare) => self.lower_declare(declare),
            ast::Stmt::Close(close) => self.lower_close(close),
            ast::Stmt::Fetch(fetch) => self.lower_fetch(fetch),
            ast::Stmt::Move(move_) => self.lower_move(move_),
            ast::Stmt::Prepare(prepare) => self.lower_prepare(prepare),
            ast::Stmt::Execute(execute) => {
                self.lower_execute(execute);
            }
            ast::Stmt::Deallocate(deallocate) => self.lower_deallocate(deallocate),
            ast::Stmt::Listen(listen) => self.lower_listen(listen),
            ast::Stmt::Notify(notify) => self.lower_notify(notify),
            ast::Stmt::Unlisten(unlisten) => self.lower_unlisten(unlisten),
            ast::Stmt::Explain(explain) => self.lower_explain(explain),
            ast::Stmt::DropFunction(drop_function) => self.lower_drop_function(drop_function),
            ast::Stmt::DropProcedure(drop_procedure) => {
                self.lower_drop_procedure(drop_procedure);
            }
            ast::Stmt::DropRoutine(drop_routine) => self.lower_drop_routine(drop_routine),
            ast::Stmt::AlterFunction(alter_function) => {
                self.lower_alter_function(alter_function);
            }
            ast::Stmt::AlterProcedure(alter_procedure) => {
                self.lower_alter_procedure(alter_procedure);
            }
            ast::Stmt::AlterRoutine(alter_routine) => self.lower_alter_routine(alter_routine),
            ast::Stmt::Call(call) => self.lower_call_stmt(call),
            ast::Stmt::AlterAggregate(alter_aggregate) => {
                self.lower_alter_aggregate(alter_aggregate);
            }
            ast::Stmt::DropAggregate(drop_aggregate) => self.lower_drop_aggregate(drop_aggregate),
            ast::Stmt::AlterDomain(alter_domain) => self.lower_alter_domain(alter_domain),
            ast::Stmt::AlterSchema(alter_schema) => self.lower_alter_schema(alter_schema),
            ast::Stmt::AlterType(alter_type) => self.lower_alter_type(alter_type),
            ast::Stmt::AlterView(alter_view) => self.lower_alter_view(alter_view),
            ast::Stmt::Truncate(truncate) => self.lower_truncate(truncate),
            ast::Stmt::Lock(lock) => self.lower_lock(lock),
            ast::Stmt::Analyze(analyze) => self.lower_analyze(analyze),
            ast::Stmt::Vacuum(vacuum) => self.lower_vacuum(vacuum),
            ast::Stmt::Repack(repack) => self.lower_repack(repack),
            ast::Stmt::AlterForeignTable(alter_table) => {
                self.lower_alter_foreign_table(alter_table);
            }
            ast::Stmt::DropForeignTable(drop_table) => self.lower_drop_foreign_table(drop_table),
            ast::Stmt::DropSchema(drop_schema) => self.lower_drop_schema(drop_schema),
            ast::Stmt::Copy(copy) => self.lower_copy(copy),
            ast::Stmt::CommentOn(comment_on) => self.lower_comment_on(comment_on),
            ast::Stmt::Grant(grant) => self.lower_grant(grant),
            ast::Stmt::Revoke(revoke) => self.lower_revoke(revoke),
            ast::Stmt::SecurityLabel(security_label) => self.lower_security_label(security_label),
            ast::Stmt::AlterDefaultPrivileges(alter_default_privileges) => {
                self.lower_alter_default_privileges(alter_default_privileges);
            }
            ast::Stmt::AlterLargeObject(alter_large_object) => {
                self.lower_alter_large_object(alter_large_object);
            }
            ast::Stmt::CreateSchema(create_schema) => self.lower_create_schema(create_schema),
            ast::Stmt::CreateTablespace(create_tablespace) => {
                self.lower_create_tablespace(create_tablespace);
            }
            ast::Stmt::AlterTablespace(alter_tablespace) => {
                self.lower_alter_tablespace(alter_tablespace);
            }
            ast::Stmt::DropTablespace(drop_tablespace) => {
                self.lower_drop_tablespace(drop_tablespace);
            }
            ast::Stmt::CreateDatabase(create_database) => {
                self.lower_create_database(create_database);
            }
            ast::Stmt::AlterDatabase(alter_database) => self.lower_alter_database(alter_database),
            ast::Stmt::DropDatabase(drop_database) => self.lower_drop_database(drop_database),
            ast::Stmt::CreateExtension(create_extension) => {
                self.lower_create_extension(create_extension);
            }
            ast::Stmt::AlterExtension(alter_extension) => {
                self.lower_alter_extension(alter_extension);
            }
            ast::Stmt::DropExtension(drop_extension) => self.lower_drop_extension(drop_extension),
            ast::Stmt::CreateOperator(create_operator) => {
                self.lower_create_operator(create_operator);
            }
            ast::Stmt::AlterOperator(alter_operator) => self.lower_alter_operator(alter_operator),
            ast::Stmt::DropOperator(drop_operator) => self.lower_drop_operator(drop_operator),
            ast::Stmt::CreateCast(create_cast) => self.lower_create_cast(create_cast),
            ast::Stmt::DropCast(drop_cast) => self.lower_drop_cast(drop_cast),
            ast::Stmt::CreateTransform(create_transform) => {
                self.lower_create_transform(create_transform);
            }
            ast::Stmt::DropTransform(drop_transform) => self.lower_drop_transform(drop_transform),
            ast::Stmt::AlterSystem(_)
            | ast::Stmt::Checkpoint(_)
            | ast::Stmt::Discard(_)
            | ast::Stmt::EmptyStmt(_)
            | ast::Stmt::Load(_)
            | ast::Stmt::ResetRole(_)
            | ast::Stmt::ResetSessionAuth(_)
            | ast::Stmt::SetTransaction(_)
            | ast::Stmt::Show(_) => {}
        }
        self.check_generated_refs();
    }

    fn check_generated_refs(&mut self) {
        let mut generated_refs = std::mem::take(&mut self.generated_refs);
        for GeneratedRefs { table, refs } in generated_refs.drain(..) {
            for reference in refs {
                let Reference {
                    range,
                    resolution: Resolution::Column { column, .. },
                    ..
                } = self.index.references[reference]
                else {
                    continue;
                };
                if let Some(column) = self.index.columns[column].name
                    && self
                        .column_exprs
                        .get(ColumnKey { table, column })
                        .is_some_and(|expression| expression.kind == ColumnExprKind::Generated)
                {
                    let range = self.index.range(range);
                    self.index
                        .errors
                        .push(range, SemaError::GeneratedColumnReference { column });
                }
            }
        }
        self.generated_refs = generated_refs;
    }

    fn lower_drop_type(&mut self, drop_type: &ast::DropType) {
        self.drop_type_refs(
            drop_type
                .type_name_refs()
                .filter_map(|name| name.path_ref()),
            IfMissing::from_if_exists(drop_type.if_exists().as_ref()),
            None,
            drop_type.drop_behavior().as_ref(),
        );
    }

    fn lower_drop_domain(&mut self, drop_domain: &ast::DropDomain) {
        self.drop_type_refs(
            drop_domain
                .domain_refs()
                .filter_map(|domain| domain.path_ref()),
            IfMissing::from_if_exists(drop_domain.if_exists().as_ref()),
            Some(TypeKind::Domain),
            drop_domain.drop_behavior().as_ref(),
        );
    }

    fn drop_type_refs(
        &mut self,
        paths: impl Iterator<Item = ast::PathRef>,
        if_missing: IfMissing,
        kind: Option<TypeKind>,
        drop_behavior: Option<&ast::DropBehavior>,
    ) {
        let mut targets: SmallVec<[(ast::PathRef, TypeId); 4]> = SmallVec::new();
        for path in paths {
            if let Some(ty) = self.lower_type_path_ref(&path, if_missing, kind) {
                targets.push((path, ty));
            }
        }
        if !matches!(drop_behavior, Some(ast::DropBehavior::Cascade(_))) {
            for (path, ty) in &targets {
                let is_target = |ty: TypeId| {
                    targets.iter().any(|(_, target)| {
                        self.index.types[*target].name_ptr == self.index.types[ty].name_ptr
                    })
                };
                let blocked =
                    self.type_dependents(*ty)
                        .into_iter()
                        .any(|dependent| match dependent {
                            Object::Type(dependent) => !is_target(dependent),
                            Object::Constraint(constraint) => !matches!(
                                self.index.constraints[constraint].owner,
                                ConstraintOwner::Domain(domain) if is_target(domain)
                            ),
                            Object::Function(_)
                            | Object::ColumnExpr(_)
                            | Object::OperatorFamilyMember(_)
                            | Object::Table(_)
                            | Object::Trigger(_)
                            | Object::EventTrigger(_)
                            | Object::Index(_)
                            | Object::Policy(_)
                            | Object::Rule(_)
                            | Object::Cast(_)
                            | Object::Transform(_)
                            | Object::Operator(_)
                            | Object::AccessMethod(_)
                            | Object::Language(_)
                            | Object::ForeignDataWrapper(_)
                            | Object::Conversion(_)
                            | Object::TextSearch(_)
                            | Object::Collation(_)
                            | Object::OperatorFamily(_)
                            | Object::OperatorClass(_)
                            | Object::Statistics(_)
                            | Object::Extension(_)
                            | Object::Schema(_)
                            | Object::Server(_)
                            | Object::Publication(_) => true,
                        })
                        || self
                            .type_typed_columns(*ty)
                            .iter()
                            .any(|column| match column.owner {
                                TypedColumnOwner::Type(owner) => !is_target(owner),
                                TypedColumnOwner::Table(_) => true,
                            });
                if blocked {
                    self.push_error(
                        path.syntax(),
                        SemaError::DependentObjects {
                            object: ObjectKind::Type,
                            name: self.index.types[*ty].name,
                        },
                    );
                    return;
                }
            }
        }
        for (_, ty) in targets {
            self.drop_object(Object::Type(ty));
        }
    }

    fn lower_drop_table(&mut self, drop_table: &ast::DropTable) {
        self.drop_relation_refs(
            drop_table
                .table_name_refs()
                .filter_map(|table| table.path_ref()),
            IfMissing::from_if_exists(drop_table.if_exists().as_ref()),
            TableLookup::Table,
            drop_table.drop_behavior().as_ref(),
        );
    }

    fn lower_drop_view(&mut self, drop_view: &ast::DropView) {
        self.drop_relation_refs(
            drop_view.view_refs().filter_map(|view| view.path_ref()),
            IfMissing::from_if_exists(drop_view.if_exists().as_ref()),
            TableLookup::View,
            drop_view.drop_behavior().as_ref(),
        );
    }

    fn lower_drop_materialized_view(&mut self, drop_view: &ast::DropMaterializedView) {
        self.drop_relation_refs(
            drop_view.view_refs().filter_map(|view| view.path_ref()),
            IfMissing::from_if_exists(drop_view.if_exists().as_ref()),
            TableLookup::MaterializedView,
            drop_view.drop_behavior().as_ref(),
        );
    }

    fn lower_alter_materialized_view(&mut self, alter_view: &ast::AlterMaterializedView) {
        if let Some(all_in_tablespace) = alter_view.all_in_tablespace() {
            self.lower_all_in_tablespace(&all_in_tablespace);
            return;
        }
        let view = alter_view
            .view_ref()
            .and_then(|view| view.path_ref())
            .and_then(|path| {
                self.lower_table_ref(
                    &path,
                    IfMissing::from_if_exists(alter_view.if_exists().as_ref()),
                    TableLookup::MaterializedView,
                )
            });
        for action in alter_view.action() {
            match action {
                ast::AlterMaterializedViewAction::RenameColumn(rename) => {
                    if let Some(column_ref) = rename.column_name_ref() {
                        let name = self.lower_constraint_column_ref(view, &column_ref);
                        if let Some(view) = view
                            && let Some(new_name) = rename.column_name()
                        {
                            self.rename_table_column(view, name, &new_name);
                        }
                    }
                }
                ast::AlterMaterializedViewAction::ViewRenameTo(rename) => {
                    if let Some(view) = view
                        && let Some(path) = rename.view().and_then(|view| view.path())
                    {
                        self.rename_table(view, &path);
                    }
                }
                ast::AlterMaterializedViewAction::SetSchema(set_schema) => {
                    self.set_table_schema(view, &set_schema);
                }
                ast::AlterMaterializedViewAction::DependsOnExtension(depends) => {
                    if let Some(extension) = depends.extension_ref() {
                        self.lower_extension_ref(&extension, IfMissing::Error);
                    }
                }
                ast::AlterMaterializedViewAction::NoDependsOnExtension(depends) => {
                    if let Some(extension) = depends.extension_ref() {
                        self.lower_extension_ref(&extension, IfMissing::Error);
                    }
                }
                ast::AlterMaterializedViewAction::AlterTableAction(_) => {}
            }
        }
        let table_actions = alter_view.action().filter_map(|action| match action {
            ast::AlterMaterializedViewAction::AlterTableAction(action) => Some(action),
            ast::AlterMaterializedViewAction::DependsOnExtension(_)
            | ast::AlterMaterializedViewAction::NoDependsOnExtension(_)
            | ast::AlterMaterializedViewAction::RenameColumn(_)
            | ast::AlterMaterializedViewAction::SetSchema(_)
            | ast::AlterMaterializedViewAction::ViewRenameTo(_) => None,
        });
        self.lower_alter_table_actions(view, false, table_actions, alter_view.syntax());
    }

    fn lower_alter_view(&mut self, alter_view: &ast::AlterView) {
        let view = alter_view
            .view_ref()
            .and_then(|view| view.path_ref())
            .and_then(|path| {
                self.lower_table_ref(
                    &path,
                    IfMissing::from_if_exists(alter_view.if_exists().as_ref()),
                    TableLookup::View,
                )
            });
        match alter_view.action() {
            Some(ast::AlterViewAction::AlterViewColumn(alter_column)) => {
                let column = alter_column
                    .name()
                    .map(|column_ref| self.lower_constraint_column_ref(view, &column_ref));
                match alter_column.alter_view_column_action() {
                    Some(ast::AlterViewColumnAction::SetDefault(set_default)) => {
                        if let Some(expr) = set_default.expr() {
                            self.lower_column_expr(view, column, &expr, ColumnExprKind::Default);
                        }
                    }
                    Some(ast::AlterViewColumnAction::DropDefault(_)) => {
                        if let (Some(view), Some(column)) = (view, column) {
                            self.set_column_expr(view, column, None);
                        }
                    }
                    None => {}
                }
            }
            Some(ast::AlterViewAction::RenameColumn(rename)) => {
                if let Some(column_ref) = rename.column_name_ref() {
                    let name = self.lower_constraint_column_ref(view, &column_ref);
                    if let Some(view) = view
                        && let Some(new_name) = rename.column_name()
                    {
                        self.rename_table_column(view, name, &new_name);
                    }
                }
            }
            Some(ast::AlterViewAction::ViewRenameTo(rename)) => {
                if let Some(view) = view
                    && let Some(path) = rename.view().and_then(|view| view.path())
                {
                    self.rename_table(view, &path);
                }
            }
            Some(ast::AlterViewAction::SetSchema(set_schema)) => {
                self.set_table_schema(view, &set_schema);
            }
            Some(ast::AlterViewAction::OwnerTo(owner_to)) => self.lower_owner_to(&owner_to),
            Some(ast::AlterViewAction::SetOptions(_) | ast::AlterViewAction::ResetOptions(_))
            | None => {}
        }
    }

    fn rename_table_column(&mut self, table: TableId, from: Name, name_node: &impl NameLike) {
        let columns = self.index.tables[table].columns;
        let Some(column) = self.find_column_in(columns.range, from) else {
            return;
        };
        let name = self.intern(&name_node.text());
        if self.columns_contain(columns.range, name) {
            self.report_duplicate_column(self.index.tables[table].name, name_node.syntax(), name);
            return;
        }
        let range = self.rename_column_copy(columns.range, column, name, name_node.syntax());
        self.set_table_columns(table, range);
        self.rename_column_dependents(table, from, name);
        let mut i = 0;
        while let Some(&inheritance) = self.inheritances.children(table).get(i) {
            i += 1;
            if !self.is_live_child(inheritance.child) {
                continue;
            }
            let child = inheritance.child;
            if self.columns_contain(self.index.tables[child].columns.range, from) {
                self.rename_table_column(child, from, name_node);
            }
        }
    }

    fn rename_column_dependents(&mut self, table: TableId, from: Name, to: Name) {
        if let Some(expression) = self.column_exprs.get(ColumnKey {
            table,
            column: from,
        }) {
            self.set_column_expr(table, from, None);
            self.set_column_expr(table, to, Some(expression));
        }
        let in_transaction = self.in_transaction();
        for id in self.index.tables.owned_sequences(table) {
            if self.dropped_tables.contains(&id) {
                continue;
            }
            let Some(owner) = self.index.tables[id].sequence_owner else {
                continue;
            };
            if owner.column != from {
                continue;
            }
            self.index.tables.set_sequence_owner(
                id,
                Some(SequenceOwner {
                    column: to,
                    ..owner
                }),
            );
            if in_transaction {
                self.undo.push(Undo::SequenceOwner(id, owner));
            }
        }
        let constraints: SmallVec<[ConstraintId; 8]> = self
            .index
            .constraints
            .owned_by(ConstraintOwner::Table(table))
            .rev()
            .collect();
        for id in constraints {
            if self.dropped_constraints.contains(&id)
                || self.index.constraints[id].kind != (ConstraintKind::NotNull { column: from })
            {
                continue;
            }
            let previous = self
                .index
                .constraints
                .set_kind(id, ConstraintKind::NotNull { column: to });
            if in_transaction {
                self.undo.push(Undo::ConstraintKind(id, previous));
            }
        }
    }

    fn set_table_columns(&mut self, table: TableId, range: IdxRange<Column>) {
        let in_transaction = self.in_transaction();
        if !self.index.tables.is_local(table) {
            return;
        }
        let columns = self.index.tables[table].columns;
        let previous =
            self.index
                .tables
                .set_columns(table, Columns { range, ..columns }, &self.index.columns);
        if in_transaction {
            self.undo.push(Undo::TableColumns(table, previous));
        }
        self.index
            .set_column_owner(range, ColumnOwner::Table(table));
    }

    fn rename_table(&mut self, table: TableId, path: &ast::Path) {
        let Some(segment) = path.segment() else {
            return;
        };
        let name = self.intern(&segment.text());
        let schema = self.index.tables[table].schema;
        if self.report_relation_name_taken(schema, name, segment.syntax()) {
            return;
        }
        self.move_table(table, schema, name, self.ptr(segment.syntax()));
        self.add_item(segment.syntax(), Item::Table(table));
    }

    fn set_table_schema(&mut self, table: Option<TableId>, set_schema: &ast::SetSchema) {
        let Some(schema_ref) = set_schema.schema_ref() else {
            return;
        };
        let Some(schema) = self.lower_set_schema_ref(&schema_ref) else {
            return;
        };
        let Some(table) = table else {
            return;
        };
        let Table { name, name_ptr, .. } = self.index.tables[table];
        if self.report_relation_name_taken(schema, name, schema_ref.syntax()) {
            return;
        }
        for sequence in self.index.tables.owned_sequences(table) {
            if !self.dropped_tables.contains(&sequence)
                && self.report_relation_name_taken(
                    schema,
                    self.index.tables[sequence].name,
                    schema_ref.syntax(),
                )
            {
                return;
            }
        }
        self.move_table(table, schema, name, name_ptr);
        for sequence in self.index.tables.owned_sequences(table) {
            if !self.dropped_tables.contains(&sequence) {
                let Table { name, name_ptr, .. } = self.index.tables[sequence];
                self.move_table(sequence, schema, name, name_ptr);
            }
        }
        let in_transaction = self.in_transaction();
        for id in self.index.indexes.on_table(table) {
            if self.dropped_indexes.contains(&id) {
                continue;
            }
            let index = &self.index.indexes[id];
            let previous = Undo::MoveIndex(id, index.schema, index.name, index.name_ptr);
            if !self.index.indexes.set_schema(id, schema) {
                continue;
            }
            if in_transaction {
                self.undo.push(previous);
            }
        }
    }

    fn report_relation_name_taken(&mut self, schema: Name, name: Name, node: &SyntaxNode) -> bool {
        if self.relation_exists(schema, name) {
            self.report_duplicate(ObjectKind::Relation, node, name);
            true
        } else if self.find_type_in(schema, name).is_some() {
            self.report_duplicate(ObjectKind::Type, node, name);
            true
        } else {
            false
        }
    }

    fn move_table(
        &mut self,
        table: TableId,
        schema: Name,
        name: Name,
        name_ptr: InIrFile<SyntaxNodePtr>,
    ) {
        let in_transaction = self.in_transaction();
        if !self.index.tables.is_local(table) {
            return;
        }
        let current = &self.index.tables[table];
        let previous = Undo::MoveTable(table, current.schema, current.name, current.name_ptr);
        let previous_name = current.name_ptr.file_range();
        self.index.tables.move_to(table, schema, name, name_ptr);
        if in_transaction {
            self.undo.push(previous);
        }
        self.index
            .record_rename(Item::Table(table), previous_name, name_ptr.file_range());
    }

    fn lower_refresh(&mut self, refresh: &ast::Refresh) {
        if let Some(path) = refresh.view_ref().and_then(|view| view.path_ref()) {
            self.lower_table_ref(&path, IfMissing::Error, TableLookup::MaterializedView);
        }
    }

    fn lower_create_sequence(&mut self, create_sequence: &ast::CreateSequence) {
        let mut sequence_owner = None;
        for option in create_sequence.sequence_options() {
            sequence_owner = self.lower_sequence_option(&option).or(sequence_owner);
        }
        let Some(path) = create_sequence
            .sequence()
            .and_then(|sequence| sequence.path())
        else {
            return;
        };
        let Some(segment) = path.segment() else {
            return;
        };
        let columns = self.sequence_columns(self.ptr(segment.syntax()));
        if let Some(sequence) = self.alloc_table(
            TableKind::Sequence,
            Persistence::from_ast(create_sequence.persistence().as_ref()),
            IfExists::from_if_not_exists(create_sequence.if_not_exists().as_ref()),
            create_sequence.syntax(),
            &path,
            columns,
        ) {
            self.index
                .tables
                .set_sequence_owner(sequence, sequence_owner);
        }
    }

    fn lower_identity(
        &mut self,
        table: Option<TableId>,
        column: Option<Name>,
        options: Option<ast::SequenceOptionList>,
        node: &SyntaxNode,
    ) {
        let mut sequence_path = None;
        for option in options.into_iter().flat_map(|list| list.sequence_options()) {
            if let ast::SequenceOption::OptionSequenceName(sequence_name) = &option {
                sequence_path = sequence_name
                    .sequence()
                    .and_then(|sequence| sequence.path());
            }
            self.lower_sequence_option(&option);
        }
        let (Some(table), Some(column)) = (table, column) else {
            return;
        };
        let Some(column_ptr) = self
            .find_table_column(table, column)
            .map(|id| self.index.columns[id].ptr)
        else {
            return;
        };
        let schema = self.index.tables[table].schema;
        let sequence = match sequence_path {
            Some(path) => {
                let Some(segment) = path.segment() else {
                    return;
                };
                let persistence = if schema == self.pg_temp {
                    Persistence::Temp
                } else {
                    Persistence::Permanent
                };
                let columns = self.sequence_columns(self.ptr(segment.syntax()));
                let Some(sequence) = self.alloc_table(
                    TableKind::Sequence,
                    persistence,
                    IfExists::Error,
                    node,
                    &path,
                    columns,
                ) else {
                    return;
                };
                sequence
            }
            None => self.alloc_column_sequence(table, column, column_ptr, node),
        };
        self.index.tables.set_sequence_owner(
            sequence,
            Some(SequenceOwner {
                table,
                column,
                identity: true,
            }),
        );
    }

    fn lower_serial_column(
        &mut self,
        table: TableId,
        column: Name,
        ty: &ast::Type,
        node: &SyntaxNode,
    ) {
        if ty.serial_type_name().is_none() {
            return;
        }
        let Some(column_ptr) = self
            .find_table_column(table, column)
            .map(|id| self.index.columns[id].ptr)
        else {
            return;
        };
        let sequence = self.alloc_column_sequence(table, column, column_ptr, node);
        self.index.tables.set_sequence_owner(
            sequence,
            Some(SequenceOwner {
                table,
                column,
                identity: false,
            }),
        );
        self.add_not_null_constraint(table, column, node, None);
    }

    fn alloc_column_sequence(
        &mut self,
        table: TableId,
        column: Name,
        column_ptr: InIrFile<SyntaxNodePtr>,
        node: &SyntaxNode,
    ) -> TableId {
        let schema = self.index.tables[table].schema;
        let name = postgres::choose_relation_name(
            self.index.names.resolve(self.index.tables[table].name),
            Some(self.index.names.resolve(column)),
            "seq",
            |name| self.relation_name_exists(schema, name),
            |_| false,
        );
        let name = self.intern(&name);
        let columns = self.sequence_columns(column_ptr);
        let id = self.index.tables.alloc(
            Table {
                kind: TableKind::Sequence,
                schema,
                name,
                ptr: self.ptr(node),
                name_ptr: column_ptr,
                columns,
                constraints_complete: true,
                sequence_owner: None,
                of_type: None,
            },
            &self.index.columns,
        );
        self.index
            .set_column_owner(columns.range, ColumnOwner::Table(id));
        id
    }

    fn sequence_columns(&mut self, ptr: InIrFile<SyntaxNodePtr>) -> Columns {
        let int8 = self.builtin_type("int8");
        let bool = self.builtin_type("bool");
        let columns = [
            Column {
                name: Some(self.intern("last_value")),
                ptr,
                ty: int8,
                merge: None,
            },
            Column {
                name: Some(self.intern("log_cnt")),
                ptr,
                ty: int8,
                merge: None,
            },
            Column {
                name: Some(self.intern("is_called")),
                ptr,
                ty: bool,
                merge: None,
            },
        ];
        Columns {
            range: self.index.columns.alloc_many(columns),
            complete: true,
        }
    }

    fn lower_alter_sequence(&mut self, alter_sequence: &ast::AlterSequence) {
        let sequence = alter_sequence
            .sequence_ref()
            .and_then(|sequence| sequence.path_ref())
            .and_then(|path| {
                self.lower_table_ref(
                    &path,
                    IfMissing::from_if_exists(alter_sequence.if_exists().as_ref()),
                    TableLookup::Sequence,
                )
            });
        for action in alter_sequence.actions() {
            match action {
                ast::AlterSequenceAction::SequenceOption(option) => {
                    self.lower_sequence_option(&option);
                }
                ast::AlterSequenceAction::OwnerTo(owner_to) => self.lower_owner_to(&owner_to),
                ast::AlterSequenceAction::SequenceRenameTo(rename) => {
                    if let Some(sequence) = sequence
                        && let Some(path) = rename.sequence().and_then(|sequence| sequence.path())
                    {
                        self.rename_table(sequence, &path);
                    }
                }
                ast::AlterSequenceAction::SetSchema(set_schema) => {
                    self.set_table_schema(sequence, &set_schema);
                }
                ast::AlterSequenceAction::SetLogged(_)
                | ast::AlterSequenceAction::SetUnlogged(_) => {}
            }
        }
    }

    fn lower_drop_sequence(&mut self, drop_sequence: &ast::DropSequence) {
        let if_missing = IfMissing::from_if_exists(drop_sequence.if_exists().as_ref());
        for path in drop_sequence
            .sequence_refs()
            .filter_map(|sequence| sequence.path_ref())
        {
            if let Some(sequence) =
                self.lower_drop_table_ref(&path, if_missing, TableLookup::Sequence)
            {
                self.drop_table(sequence);
            }
        }
    }

    fn lower_sequence_option(&mut self, option: &ast::SequenceOption) -> Option<SequenceOwner> {
        match option {
            ast::SequenceOption::OptionAsType(as_type) => {
                if let Some(ty) = as_type.ty() {
                    self.lower_type_ref(&ty, IfMissing::Error);
                }
                None
            }
            ast::SequenceOption::OptionOwnedBy(owned_by) => match owned_by.owned_by_target() {
                Some(ast::OwnedByTarget::QualifiedColumnNameRef(column)) => {
                    let path = column.path_ref()?;
                    let segment = path.segment()?;
                    let table = self
                        .lower_qualified_column_ref(&path, TableLookup::TableOrForeignTable)
                        .table?;
                    Some(SequenceOwner {
                        table,
                        column: self.intern(&segment.text()),
                        identity: false,
                    })
                }
                Some(ast::OwnedByTarget::OwnedByNone(_)) | None => None,
            },
            ast::SequenceOption::OptionSequenceName(_)
            | ast::SequenceOption::OptionCache(_)
            | ast::SequenceOption::OptionCycle(_)
            | ast::SequenceOption::OptionIncrement(_)
            | ast::SequenceOption::OptionLogged(_)
            | ast::SequenceOption::OptionMaxValue(_)
            | ast::SequenceOption::OptionMinValue(_)
            | ast::SequenceOption::OptionNoCycle(_)
            | ast::SequenceOption::OptionNoMaxValue(_)
            | ast::SequenceOption::OptionNoMinValue(_)
            | ast::SequenceOption::OptionRestart(_)
            | ast::SequenceOption::OptionStart(_)
            | ast::SequenceOption::OptionUnlogged(_) => None,
        }
    }

    fn lower_qualified_column_ref(
        &mut self,
        path: &ast::PathRef,
        lookup: TableLookup,
    ) -> QualifiedColumn {
        let Some(segment) = path.segment() else {
            return QualifiedColumn {
                table: None,
                column: None,
            };
        };
        if matches!(lookup, TableLookup::Any)
            && let Some(qualifier) = path.qualifier()
            && self.find_composite_type(&qualifier).is_some()
        {
            let ty = self.lower_type_path_ref(&qualifier, IfMissing::Error, None);
            let column = self.lower_type_attribute_ref(ty, &segment, IfMissing::Error);
            return QualifiedColumn {
                table: None,
                column,
            };
        }
        let table = path
            .qualifier()
            .and_then(|table| self.lower_table_ref(&table, IfMissing::Error, lookup));
        let name = self.intern(&segment.text());
        let column = table.and_then(|table| self.find_table_column(table, name));
        let resolution = match (table, column) {
            (_, Some(column)) => Resolution::Column {
                source: None,
                column,
            },
            (Some(table), None) if self.index.tables[table].columns.complete => {
                Resolution::Unresolved
            }
            (Some(_) | None, None) => Resolution::Unknown,
        };
        self.add_ref(name, ReferenceKind::Column, segment.syntax(), resolution);
        QualifiedColumn { table, column }
    }

    fn find_composite_type(&mut self, path: &ast::PathRef) -> Option<TypeId> {
        let name = self.intern(&path.segment()?.text());
        let schema = match path.qualifier() {
            Some(qualifier) => Some(self.intern_schema(&qualifier.segment()?.text())),
            None => None,
        };
        if self.find_table(schema, name).is_some() {
            return None;
        }
        self.search_path(schema, Lookup::Type)
            .find_map(|schema| self.find_type_in(schema, name))
            .filter(|ty| self.index.types[*ty].columns.is_some())
    }

    fn lower_create_aggregate(&mut self, create_aggregate: &ast::CreateAggregate) {
        let refs = self.index.references.next_idx();
        let start = self.index.functions.next_idx();
        let param_list = create_aggregate.param_list();
        let attributes = create_aggregate.attribute_list();
        let old_style = param_list.is_none();
        let params = match param_list {
            Some(param_list) => self.alloc_params(Some(param_list)),
            None => FunctionParams {
                params: self.alloc_basetype_param(attributes.as_ref()),
                direct_args: None,
            },
        };
        let function = create_aggregate
            .aggregate_name()
            .and_then(|name| name.path())
            .and_then(|path| {
                self.alloc_function(
                    FunctionKind::Aggregate,
                    IfExists::from_or_replace(create_aggregate.or_replace().as_ref()),
                    create_aggregate.syntax(),
                    &path,
                    params,
                    None,
                )
            });
        let FunctionParams {
            params,
            direct_args,
        } = params;
        let inputs: SmallVec<[Option<Ty>; 4]> = params
            .iter()
            .map(|param| &self.index.params[param])
            .filter(|param| param.mode != ParamMode::Out)
            .map(|param| param.ty)
            .collect();
        if let Some(attributes) = attributes {
            let ret =
                self.lower_aggregate_attributes(&attributes, &inputs, direct_args, !old_style);
            if let Some(function) = function {
                self.index.functions.set_ret(function, ret);
            }
        }
        self.depend_on_refs(
            refs,
            self.index.functions.range_from(start),
            Object::Function,
        );
    }

    fn lower_aggregate_attributes(
        &mut self,
        attributes: &ast::AttributeList,
        all_inputs: &[Option<Ty>],
        direct_args: Option<usize>,
        lower_basetype: bool,
    ) -> Option<Ty> {
        let (direct, aggregated) = match direct_args {
            Some(direct_args) => {
                let (direct, aggregated) = all_inputs.split_at(direct_args.min(all_inputs.len()));
                (direct, Some(aggregated))
            }
            None => (all_inputs, None),
        };
        let mut stype = None;
        let mut mstype = None;
        let mut finalfunc_extra = false;
        let mut mfinalfunc_extra = false;
        let mut finalfunc_ret = None;
        for option in attributes.attribute_options() {
            let Some(name) = attribute_name(&option) else {
                continue;
            };
            let ty = option.attribute_value().and_then(|value| value.ty());
            match name.as_str() {
                "stype" => stype = ty.and_then(|ty| self.lower_type(&ty)),
                "mstype" => mstype = ty.and_then(|ty| self.lower_type(&ty)),
                "basetype" if lower_basetype => {
                    if let Some(ty) = ty {
                        self.lower_type(&ty);
                    }
                }
                "finalfunc_extra" => finalfunc_extra = true,
                "mfinalfunc_extra" => mfinalfunc_extra = true,
                _ => {}
            }
        }
        let inputs = aggregated.unwrap_or(direct);
        let internal = self.builtin_type("internal");
        let bytea = self.builtin_type("bytea");
        let (final_inputs, final_extra_inputs) = match aggregated {
            Some(_) => (direct, all_inputs),
            None => (&[][..], inputs),
        };
        for option in attributes.attribute_options() {
            let (Some(name), Some(value)) = (attribute_name(&option), option.attribute_value())
            else {
                continue;
            };
            let (state, rest): (Option<Ty>, &[Option<Ty>]) = match name.as_str() {
                "sfunc" => (stype, inputs),
                "msfunc" | "minvfunc" => (mstype, inputs),
                "finalfunc" if finalfunc_extra => (stype, final_extra_inputs),
                "finalfunc" => (stype, final_inputs),
                "mfinalfunc" if mfinalfunc_extra => (mstype, inputs),
                "mfinalfunc" => (mstype, &[]),
                "combinefunc" => (stype, std::slice::from_ref(&stype)),
                "serialfunc" => (internal, &[]),
                "deserialfunc" => (bytea, std::slice::from_ref(&internal)),
                "sortop" => {
                    let operand = match inputs {
                        [Some(ty)] => Operand::Ty(*ty),
                        _ => Operand::Unknown,
                    };
                    self.lower_operator_attribute_ref(&value, operand, operand);
                    continue;
                }
                _ => continue,
            };
            if let Some(path) = attribute_path(&option) {
                let resolution = self.lower_function_arg_types_ref(
                    &path,
                    std::iter::once(state).chain(rest.iter().copied()),
                );
                if name == "finalfunc"
                    && let Resolution::Function(function) = resolution
                {
                    finalfunc_ret = self.index.functions[function].ret;
                }
            }
        }
        finalfunc_ret.or(stype)
    }

    fn lower_function_arg_types_ref(
        &mut self,
        path: &ast::PathRef,
        arg_types: impl IntoIterator<Item = Option<Ty>>,
    ) -> Resolution {
        let Some(segment) = path.segment() else {
            return Resolution::Unknown;
        };
        let schema = self.lower_schema_qualifier(path.qualifier());
        let name = self.intern(&segment.text());
        let args: SmallVec<[CallArg; 4]> = arg_types
            .into_iter()
            .map(|ty| CallArg {
                name: None,
                ty: ty.map_or(ArgType::Untyped, ArgType::Known),
                variadic: false,
            })
            .collect();
        let resolution = self.find_routine(schema, name, &args, FunctionKind::Function);
        self.add_ref(
            name,
            ReferenceKind::Function(RoutineKind::Function),
            segment.syntax(),
            resolution,
        );
        resolution
    }

    fn lower_alter_aggregate(&mut self, alter_aggregate: &ast::AlterAggregate) {
        let function = alter_aggregate.aggregate().and_then(|aggregate| {
            let path = aggregate.path_ref()?;
            self.lower_routine_sig(
                &path,
                aggregate.param_list(),
                RoutineKind::Aggregate,
                IfMissing::Error,
            )
        });
        let node = alter_aggregate.syntax();
        match alter_aggregate.action() {
            Some(ast::AlterAggregateAction::AggregateRenameTo(rename)) => {
                if let Some(function) = function
                    && let Some(path) = rename.aggregate_name().and_then(|name| name.path())
                {
                    self.rename_function(function, &path, node);
                }
            }
            Some(ast::AlterAggregateAction::SetSchema(set_schema)) => {
                self.set_function_schema(function, &set_schema, node);
            }
            Some(ast::AlterAggregateAction::OwnerTo(owner_to)) => self.lower_owner_to(&owner_to),
            None => {}
        }
    }

    fn lower_drop_aggregate(&mut self, drop_aggregate: &ast::DropAggregate) {
        let if_missing = IfMissing::from_if_exists(drop_aggregate.if_exists().as_ref());
        let drop_behavior = drop_aggregate.drop_behavior();
        for aggregate in drop_aggregate.aggregates() {
            if let Some(path) = aggregate.path_ref() {
                self.drop_routine_sig(
                    &path,
                    aggregate.param_list(),
                    RoutineKind::Aggregate,
                    if_missing,
                    drop_behavior.as_ref(),
                );
            }
        }
    }

    fn lower_select_into_stmt(&mut self, select: &ast::SelectInto) {
        let query = self.lower_select_into(select);
        if let Some(into_clause) = select.into_clause()
            && let Some(path) = into_clause.table_name().and_then(|name| name.path())
        {
            let columns = self.index.queries[query].columns;
            let persistence = Persistence::from_ast(into_clause.persistence().as_ref());
            self.alloc_table(
                TableKind::Table,
                persistence,
                IfExists::Error,
                select.syntax(),
                &path,
                columns,
            );
        }
    }

    fn lower_begin(&mut self, begin: &ast::Begin) {
        self.savepoints.clear();
        if self.transaction.is_some() {
            return;
        }
        let id = self.index.transactions.alloc(Transaction {
            begin: self.ast_ptr(begin),
            end: None,
        });
        self.start_transaction(Some(id));
        if let Some(token) = begin.begin_token().or_else(|| begin.start_token()) {
            self.add_token_item(&token, Item::TransactionBegin(id));
        }
    }

    fn start_transaction(&mut self, id: Option<TransactionId>) {
        self.transaction = Some(OpenTransaction {
            id,
            snapshot: self.snapshot(),
        });
    }

    fn chain_transaction(&mut self, chain: Option<&ast::ChainClause>) {
        match chain {
            Some(ast::ChainClause::AndChain(_)) => self.start_transaction(None),
            Some(ast::ChainClause::AndNoChain(_)) | None => {}
        }
    }

    fn lower_commit(&mut self, commit: &ast::Commit) {
        match commit {
            ast::Commit::CommitTransaction(commit) => {
                self.end_transaction(
                    commit.commit_token().or_else(|| commit.end_token()),
                    commit.syntax(),
                );
                self.chain_transaction(commit.chain_clause().as_ref());
            }
            ast::Commit::CommitPrepared(commit) => {
                self.savepoints.clear();
                if let Some(literal) = commit.literal() {
                    self.lower_prepared_transaction_ref(&literal);
                }
            }
        }
    }

    fn lower_rollback(&mut self, rollback: &ast::Rollback) {
        match rollback {
            ast::Rollback::RollbackTransaction(rollback) => {
                if let Some(transaction) = self.transaction.take() {
                    self.restore(&transaction.snapshot);
                    self.transaction = Some(transaction);
                }
                self.end_transaction(
                    rollback.rollback_token().or_else(|| rollback.abort_token()),
                    rollback.syntax(),
                );
                self.chain_transaction(rollback.chain_clause().as_ref());
            }
            ast::Rollback::RollbackPrepared(rollback) => {
                self.savepoints.clear();
                if let Some(literal) = rollback.literal() {
                    self.lower_prepared_transaction_ref(&literal);
                }
            }
            ast::Rollback::RollbackToSavepoint(rollback) => {
                if let Some(idx) = rollback
                    .savepoint_ref()
                    .and_then(|savepoint_ref| self.lower_savepoint_ref(&savepoint_ref))
                {
                    self.savepoints.truncate(idx + 1);
                    if let Some(savepoint) = self.savepoints.pop() {
                        self.restore(&savepoint.snapshot);
                        self.savepoints.push(savepoint);
                    }
                }
            }
        }
    }

    fn lower_prepare_transaction(&mut self, prepare: &ast::PrepareTransaction) {
        self.end_transaction(prepare.prepare_token(), prepare.syntax());
        let Some(literal) = prepare.literal() else {
            return;
        };
        let Some(name) = literal.string_value() else {
            return;
        };
        let name = self.index.names.intern(&name);
        let id = self.index.prepared_transactions.alloc(PreparedTransaction {
            name,
            ptr: self.ast_ptr(prepare),
            name_ptr: self.ast_ptr(&literal),
        });
        self.prepared_transactions.insert(name, id);
        self.record_comment(prepare.syntax(), Item::PreparedTransaction(id));
        self.add_item(literal.syntax(), Item::PreparedTransaction(id));
    }

    fn lower_prepared_transaction_ref(&mut self, literal: &ast::Literal) {
        let Some(name) = literal.string_value() else {
            return;
        };
        let name = self.index.names.intern(&name);
        let resolution = self
            .prepared_transactions
            .remove(&name)
            .map_or(Resolution::Unknown, Resolution::PreparedTransaction);
        self.add_ref(
            name,
            ReferenceKind::PreparedTransaction,
            literal.syntax(),
            resolution,
        );
    }

    fn end_transaction(&mut self, token: Option<SyntaxToken>, stmt: &SyntaxNode) {
        self.savepoints.clear();
        let cursors = &self.index.cursors;
        self.cursors.retain(|_, cursor| cursors[*cursor].hold);
        self.undo.clear();
        self.search_path.clone_from(&self.session_search_path);
        let Some(id) = self
            .transaction
            .take()
            .and_then(|transaction| transaction.id)
        else {
            return;
        };
        self.index.transactions[id].end = Some(self.ptr(stmt));
        if let Some(token) = token {
            self.add_token_item(&token, Item::TransactionEnd(id));
        }
    }

    fn in_transaction(&self) -> bool {
        self.transaction.is_some() || !self.savepoints.is_empty()
    }

    fn snapshot(&self) -> Snapshot {
        Snapshot {
            search_path: self.search_path.clone(),
            session_search_path: self.session_search_path.clone(),
            undo_len: self.undo.len(),
            tables: self.index.tables.next_idx(),
            types: self.index.types.next_idx(),
            indexes: self.index.indexes.next_idx(),
            constraints: self.index.constraints.next_idx(),
            triggers: self.index.triggers.next_idx(),
            rules: self.index.rules.next_idx(),
            roles: self.index.roles.next_idx(),
            policies: self.index.policies.next_idx(),
            access_methods: self.index.access_methods.next_idx(),
            event_triggers: self.index.event_triggers.next_idx(),
            languages: self.index.languages.next_idx(),
            foreign_data_wrappers: self.index.foreign_data_wrappers.next_idx(),
            servers: self.index.servers.next_idx(),
            conversions: self.index.conversions.next_idx(),
            operator_families: self.index.operator_families.next_idx(),
            operator_classes: self.index.operator_classes.next_idx(),
            statistics: self.index.statistics.next_idx(),
            text_search: self.index.text_search.next_idx(),
            publications: self.index.publications.next_idx(),
            subscriptions: self.index.subscriptions.next_idx(),
            graph_elements: self.index.graph_elements.next_idx(),
            graph_labels: self.index.graph_labels.next_idx(),
            enum_labels: self.index.enum_labels.next_idx(),
            graph_properties: self.index.graph_properties.next_idx(),
            collations: self.index.collations.next_idx(),
            tablespaces: self.index.tablespaces.next_idx(),
            databases: self.index.databases.next_idx(),
            schemas: self.index.schemas.next_idx(),
            extensions: self.index.extensions.next_idx(),
            operators: self.index.operators.next_idx(),
            functions: self.index.functions.next_idx(),
            casts: self.index.casts.next_idx(),
            transforms: self.index.transforms.next_idx(),
            cursors: self.index.cursors.next_idx(),
            operator_family_members: self.index.operator_family_members.next_idx(),
        }
    }

    fn restore(&mut self, snapshot: &Snapshot) {
        for undo in self.undo.drain(snapshot.undo_len..).rev() {
            match undo {
                Undo::DropTable(table) => {
                    self.dropped_tables.remove(&table);
                }
                Undo::DropType(ty) => {
                    self.dropped_types.remove(&ty);
                }
                Undo::DropIndex(index) => {
                    self.dropped_indexes.remove(&index);
                }
                Undo::DropConstraint(constraint) => {
                    self.dropped_constraints.remove(&constraint);
                }
                Undo::DropTrigger(trigger) => {
                    self.dropped_triggers.remove(&trigger);
                }
                Undo::DropRule(rule) => {
                    self.dropped_rules.remove(&rule);
                }
                Undo::DropRole(role) => {
                    self.dropped_roles.remove(&role);
                }
                Undo::DropPolicy(policy) => {
                    self.dropped_policies.remove(&policy);
                }
                Undo::DropAccessMethod(access_method) => {
                    self.dropped_access_methods.remove(&access_method);
                }
                Undo::DropEventTrigger(event_trigger) => {
                    self.dropped_event_triggers.remove(&event_trigger);
                }
                Undo::DropLanguage(language) => {
                    self.dropped_languages.remove(&language);
                }
                Undo::DropForeignDataWrapper(wrapper) => {
                    self.dropped_foreign_data_wrappers.remove(&wrapper);
                }
                Undo::DropServer(server) => {
                    self.dropped_servers.remove(&server);
                }
                Undo::DropConversion(conversion) => {
                    self.dropped_conversions.remove(&conversion);
                }
                Undo::DropFunction(function) => {
                    self.dropped_functions.remove(&function);
                }
                Undo::DropSchema(schema) => {
                    self.dropped_schemas.remove(&schema);
                }
                Undo::RenameSchema(from, to) => rename_schema_objects(&mut self.index, to, from),
                Undo::DropOperatorFamily(family) => {
                    self.dropped_operator_families.remove(&family);
                }
                Undo::DropOperatorClass(class) => {
                    self.dropped_operator_classes.remove(&class);
                }
                Undo::DropStatistics(statistics) => {
                    self.dropped_statistics.remove(&statistics);
                }
                Undo::DropTextSearch(text_search) => {
                    self.dropped_text_search.remove(&text_search);
                }
                Undo::DropPublication(publication) => {
                    self.dropped_publications.remove(&publication);
                }
                Undo::DropSubscription(subscription) => {
                    self.dropped_subscriptions.remove(&subscription);
                }
                Undo::DropGraphElement(element) => {
                    self.dropped_graph_elements.remove(&element);
                }
                Undo::DropGraphLabel(label) => {
                    self.dropped_graph_labels.remove(&label);
                }
                Undo::DropGraphProperty(property) => {
                    self.dropped_graph_properties.remove(&property);
                }
                Undo::DropCollation(collation) => {
                    self.dropped_collations.remove(&collation);
                }
                Undo::DropTablespace(tablespace) => {
                    self.dropped_tablespaces.remove(&tablespace);
                }
                Undo::DropDatabase(database) => {
                    self.dropped_databases.remove(&database);
                }
                Undo::DropExtension(extension) => {
                    self.dropped_extensions.remove(&extension);
                }
                Undo::DropOperator(operator) => {
                    self.dropped_operators.remove(&operator);
                }
                Undo::DropCast(cast) => {
                    self.dropped_casts.remove(&cast);
                }
                Undo::DropTransform(key, transform) => {
                    self.transforms.insert(key, transform);
                }
                Undo::TableColumns(table, columns) => {
                    self.index
                        .tables
                        .set_columns(table, columns, &self.index.columns);
                }
                Undo::DropOperatorFamilyMember(member) => {
                    self.dropped_operator_family_members.remove(&member);
                }
                Undo::MemberOwner(member, owner) => {
                    self.index.operator_family_members.set_owner(member, owner);
                }
                Undo::ColumnExpr(key, expression) => {
                    self.column_exprs.set(key, expression);
                }
                Undo::MoveTable(id, schema, name, name_ptr) => {
                    let current = self.index.tables[id].name_ptr.file_range();
                    self.index
                        .record_rename(Item::Table(id), current, name_ptr.file_range());
                    self.index.tables.move_to(id, schema, name, name_ptr);
                }
                Undo::MoveIndex(id, schema, name, name_ptr) => {
                    let current = self.index.indexes[id].name_ptr.file_range();
                    self.index
                        .record_rename(Item::Index(id), current, name_ptr.file_range());
                    self.index.indexes.move_to(id, schema, name, name_ptr);
                }
                Undo::MoveExtension(id, schema) => {
                    self.index.extensions.set_schema(id, schema);
                }
                Undo::MoveObjectSchema(object, schema) => {
                    set_object_schema(&mut self.index, object, schema);
                }
                Undo::RemoveDependency(dependent, referenced) => {
                    self.dependencies.add_dependency(dependent, referenced);
                }
                Undo::AddInheritance(inheritance) => {
                    self.inheritances.remove(inheritance);
                }
                Undo::RemoveInheritance(inheritance) => {
                    self.inheritances.add(inheritance);
                }
                Undo::MoveEnumLabel(id, name, name_ptr) => {
                    let current = self.index.enum_labels[id].name_ptr.file_range();
                    self.index
                        .record_rename(Item::EnumLabel(id), current, name_ptr.file_range());
                    let label = &mut self.index.enum_labels[id];
                    label.name = name;
                    label.name_ptr = name_ptr;
                }
                Undo::SequenceOwner(id, owner) => {
                    self.index.tables.set_sequence_owner(id, Some(owner));
                }
                Undo::ConstraintKind(id, kind) => {
                    self.index.constraints.set_kind(id, kind);
                }
                Undo::TypeColumns(ty, columns) => {
                    self.index
                        .types
                        .set_columns(ty, Some(columns), &self.index.columns);
                }
                Undo::TableOfType(table, ty) => {
                    self.index.tables.set_of_type(table, ty);
                }
                Undo::TypeSubscript(id, subscript, results) => {
                    self.index
                        .types
                        .set_subscript(id, subscript, results.result, results.slice);
                }
                Undo::MoveType(id, schema, name, name_ptr) => {
                    let current = self.index.types[id].name_ptr.file_range();
                    self.index
                        .record_rename(Item::Type(id), current, name_ptr.file_range());
                    self.index.types.move_to(id, schema, name, name_ptr);
                }
                Undo::FinishShellType(id, shell) => {
                    self.index.types.replace(id, *shell, &self.index.columns);
                }
            }
        }
        self.dropped_tables
            .extend(self.index.tables.range_from(snapshot.tables));
        self.dropped_types
            .extend(self.index.types.range_from(snapshot.types));
        self.dropped_indexes
            .extend(self.index.indexes.range_from(snapshot.indexes));
        self.dropped_constraints
            .extend(self.index.constraints.range_from(snapshot.constraints));
        self.dropped_triggers
            .extend(self.index.triggers.range_from(snapshot.triggers));
        self.dropped_rules
            .extend(self.index.rules.range_from(snapshot.rules));
        self.dropped_roles
            .extend(self.index.roles.range_from(snapshot.roles));
        self.dropped_policies
            .extend(self.index.policies.range_from(snapshot.policies));
        self.dropped_access_methods.extend(
            self.index
                .access_methods
                .range_from(snapshot.access_methods),
        );
        self.dropped_event_triggers.extend(
            self.index
                .event_triggers
                .range_from(snapshot.event_triggers),
        );
        self.dropped_languages
            .extend(self.index.languages.range_from(snapshot.languages));
        self.dropped_foreign_data_wrappers.extend(
            self.index
                .foreign_data_wrappers
                .range_from(snapshot.foreign_data_wrappers),
        );
        self.dropped_servers
            .extend(self.index.servers.range_from(snapshot.servers));
        self.dropped_conversions
            .extend(self.index.conversions.range_from(snapshot.conversions));
        self.dropped_operator_families.extend(
            self.index
                .operator_families
                .range_from(snapshot.operator_families),
        );
        self.dropped_operator_classes.extend(
            self.index
                .operator_classes
                .range_from(snapshot.operator_classes),
        );
        self.dropped_statistics
            .extend(self.index.statistics.range_from(snapshot.statistics));
        self.dropped_text_search
            .extend(self.index.text_search.range_from(snapshot.text_search));
        self.dropped_publications
            .extend(self.index.publications.range_from(snapshot.publications));
        self.dropped_subscriptions
            .extend(self.index.subscriptions.range_from(snapshot.subscriptions));
        self.dropped_graph_elements.extend(
            self.index
                .graph_elements
                .range_from(snapshot.graph_elements),
        );
        self.dropped_graph_labels
            .extend(self.index.graph_labels.range_from(snapshot.graph_labels));
        for label in self.index.enum_labels.range_from(snapshot.enum_labels) {
            let ty = self.index.enum_labels[label].ty;
            if let Some(labels) = self.index.type_enum_labels.get_mut(&ty) {
                labels.retain(|label| *label < snapshot.enum_labels);
            }
        }
        self.dropped_graph_properties.extend(
            self.index
                .graph_properties
                .range_from(snapshot.graph_properties),
        );
        self.dropped_collations
            .extend(self.index.collations.range_from(snapshot.collations));
        self.dropped_tablespaces
            .extend(self.index.tablespaces.range_from(snapshot.tablespaces));
        self.dropped_databases
            .extend(self.index.databases.range_from(snapshot.databases));
        self.dropped_schemas
            .extend(self.index.schemas.range_from(snapshot.schemas));
        self.dropped_extensions
            .extend(self.index.extensions.range_from(snapshot.extensions));
        self.dropped_operators
            .extend(self.index.operators.range_from(snapshot.operators));
        self.dropped_functions
            .extend(self.index.functions.range_from(snapshot.functions));
        self.dropped_operator_family_members.extend(
            self.index
                .operator_family_members
                .range_from(snapshot.operator_family_members),
        );
        self.dropped_casts
            .extend(self.index.casts.range_from(snapshot.casts));
        self.transforms
            .retain(|_, transform| *transform < snapshot.transforms);
        self.cursors.retain(|_, cursor| *cursor < snapshot.cursors);
        self.search_path.clone_from(&snapshot.search_path);
        self.session_search_path
            .clone_from(&snapshot.session_search_path);
    }

    fn drop_table(&mut self, table: TableId) {
        if self.dropped_tables.insert(table) && self.in_transaction() {
            self.undo.push(Undo::DropTable(table));
        }
    }

    fn drop_trigger(&mut self, trigger: TriggerId) {
        if self.dropped_triggers.insert(trigger) && self.in_transaction() {
            self.undo.push(Undo::DropTrigger(trigger));
        }
    }

    fn drop_rule(&mut self, rule: RuleId) {
        if self.dropped_rules.insert(rule) && self.in_transaction() {
            self.undo.push(Undo::DropRule(rule));
        }
    }

    fn drop_role(&mut self, role: RoleId) {
        if self.dropped_roles.insert(role) && self.in_transaction() {
            self.undo.push(Undo::DropRole(role));
        }
    }

    fn drop_policy(&mut self, policy: PolicyId) {
        if self.dropped_policies.insert(policy) && self.in_transaction() {
            self.undo.push(Undo::DropPolicy(policy));
        }
    }

    fn drop_access_method(&mut self, access_method: AccessMethodId) {
        if self.dropped_access_methods.insert(access_method) && self.in_transaction() {
            self.undo.push(Undo::DropAccessMethod(access_method));
        }
    }

    fn drop_event_trigger(&mut self, event_trigger: EventTriggerId) {
        if self.dropped_event_triggers.insert(event_trigger) && self.in_transaction() {
            self.undo.push(Undo::DropEventTrigger(event_trigger));
        }
    }

    fn drop_language(&mut self, language: LanguageId) {
        if self.dropped_languages.insert(language) && self.in_transaction() {
            self.undo.push(Undo::DropLanguage(language));
        }
    }

    fn drop_foreign_data_wrapper(&mut self, wrapper: ForeignDataWrapperId) {
        if self.dropped_foreign_data_wrappers.insert(wrapper) && self.in_transaction() {
            self.undo.push(Undo::DropForeignDataWrapper(wrapper));
        }
    }

    fn drop_server(&mut self, server: ServerId) {
        if self.dropped_servers.insert(server) && self.in_transaction() {
            self.undo.push(Undo::DropServer(server));
        }
    }

    fn drop_function(&mut self, function: FunctionId) {
        if self.dropped_functions.insert(function) && self.in_transaction() {
            self.undo.push(Undo::DropFunction(function));
        }
    }

    fn drop_schema(&mut self, schema: SchemaId) {
        if self.dropped_schemas.insert(schema) && self.in_transaction() {
            self.undo.push(Undo::DropSchema(schema));
        }
    }

    fn drop_function_with_dependents(&mut self, function: FunctionId) {
        let dependents = self.dependents(Object::Function(function));
        self.drop_function(function);
        for dependent in dependents {
            self.drop_object(dependent);
        }
    }

    fn drop_operator_with_dependents(&mut self, operator: OperatorId) {
        let dependents = self.dependents(Object::Operator(operator));
        self.drop_operator(operator);
        for dependent in dependents {
            self.drop_object(dependent);
        }
    }

    fn drop_object(&mut self, object: Object) {
        match object {
            Object::Function(function) => {
                if !self.dropped_functions.contains(&function) {
                    self.drop_function_with_dependents(function);
                }
            }
            Object::Table(table) => {
                if !self.dropped_tables.contains(&table) {
                    self.drop_table_with_dependents(table);
                }
            }
            Object::Type(ty) => {
                if !self.dropped_types.contains(&ty) {
                    self.drop_type_with_dependents(ty);
                }
            }
            Object::Trigger(trigger) => self.drop_trigger(trigger),
            Object::EventTrigger(event_trigger) => self.drop_event_trigger(event_trigger),
            Object::Index(index) => self.drop_index(index),
            Object::Constraint(constraint) => self.drop_constraint(constraint),
            Object::Policy(policy) => self.drop_policy(policy),
            Object::Rule(rule) => self.drop_rule(rule),
            Object::Cast(cast) => self.drop_cast(cast),
            Object::Transform(transform) => self.drop_transform(transform),
            Object::Operator(operator) => {
                if !self.dropped_operators.contains(&operator) {
                    self.drop_operator_with_dependents(operator);
                }
            }
            Object::AccessMethod(access_method) => self.drop_access_method(access_method),
            Object::Language(language) => self.drop_language(language),
            Object::ForeignDataWrapper(wrapper) => self.drop_foreign_data_wrapper(wrapper),
            Object::Conversion(conversion) => self.drop_conversion(conversion),
            Object::TextSearch(text_search) => self.drop_text_search_id(text_search),
            Object::Collation(collation) => self.drop_collation(collation),
            Object::OperatorFamily(family) => self.drop_operator_family(family),
            Object::OperatorClass(class) => self.drop_operator_class(class),
            Object::Statistics(statistics) => self.drop_statistics(statistics),
            Object::Extension(extension) => {
                if !self.dropped_extensions.contains(&extension) {
                    self.drop_extension_with_dependents(extension);
                }
            }
            Object::Schema(schema) => {
                if !self.dropped_schemas.contains(&schema) {
                    self.drop_schema_with_contents(schema);
                }
            }
            Object::Server(server) => self.drop_server(server),
            Object::Publication(publication) => self.drop_publication(publication),
            Object::ColumnExpr(id) => self.drop_column_expr(id),
            Object::OperatorFamilyMember(member) => self.drop_operator_family_member(member),
        }
    }

    fn drop_operator_family_member(&mut self, member: OperatorFamilyMemberId) {
        if self.dropped_operator_family_members.insert(member) && self.in_transaction() {
            self.undo.push(Undo::DropOperatorFamilyMember(member));
        }
    }

    fn move_operator_family_members(&mut self, from: MemberOwner, to: MemberOwner) {
        if from == to {
            return;
        }
        let ids = self.index.operator_family_members.move_owned(from, to);
        if self.in_transaction() {
            self.undo
                .extend(ids.into_iter().map(|id| Undo::MemberOwner(id, from)));
        }
    }

    fn drop_column_expr(&mut self, id: ColumnExprId) {
        let Some(key) = self.column_exprs.column_of(id) else {
            return;
        };
        let generated = self
            .column_exprs
            .get(key)
            .is_some_and(|expression| expression.kind == ColumnExprKind::Generated);
        self.set_column_expr(key.table, key.column, None);
        if generated {
            self.drop_table_column(key.table, key.column);
        }
    }

    fn drop_table_column(&mut self, table: TableId, column: Name) {
        self.set_column_expr(table, column, None);
        self.drop_column_sequences(table, column);
        if !self.index.tables.is_local(table) {
            return;
        }
        let columns = self.index.tables[table].columns.range;
        let range = self.copy_columns(columns, |columns, id| {
            (columns[id].name != Some(column)).then_some(columns[id])
        });
        self.set_table_columns(table, range);
    }

    fn drop_column_sequences(&mut self, table: TableId, column: Name) {
        for id in self.index.tables.owned_sequences(table) {
            if self.index.tables[id]
                .sequence_owner
                .is_some_and(|owner| owner.column == column)
            {
                self.drop_object(Object::Table(id));
            }
        }
    }

    fn set_column_expr(&mut self, table: TableId, column: Name, expression: Option<ColumnExpr>) {
        let key = ColumnKey { table, column };
        let previous = self.column_exprs.set(key, expression);
        if previous != expression && self.in_transaction() {
            self.undo.push(Undo::ColumnExpr(key, previous));
        }
    }

    fn lower_column_expr(
        &mut self,
        table: Option<TableId>,
        column: Option<Name>,
        expr: &ast::Expr,
        kind: ColumnExprKind,
    ) {
        let refs = self.index.references.next_idx();
        let context = match kind {
            ColumnExprKind::Default => ExprContext::Default,
            ColumnExprKind::Generated => ExprContext::GeneratedColumn,
        };
        self.lower_expr_in(context, expr);
        let (Some(table), Some(column)) = (table, column) else {
            return;
        };
        let id = ColumnExprId(self.next_column_expr);
        self.next_column_expr += 1;
        self.set_column_expr(table, column, Some(ColumnExpr { id, kind }));
        if kind == ColumnExprKind::Generated {
            self.generated_refs.push(GeneratedRefs {
                table,
                refs: self.index.references.range_from(refs),
            });
        }
        for reference in self.index.references.range_from(refs) {
            if let Resolution::Function(function) = self.index.references[reference].resolution {
                self.dependencies
                    .add_dependency(Object::ColumnExpr(id), Object::Function(function));
            }
        }
    }

    fn is_dropped(&self, object: Object) -> bool {
        match object {
            Object::Function(function) => self.dropped_functions.contains(&function),
            Object::Table(table) => self.dropped_tables.contains(&table),
            Object::Type(ty) => self.dropped_types.contains(&ty),
            Object::Trigger(trigger) => self.dropped_triggers.contains(&trigger),
            Object::EventTrigger(event_trigger) => {
                self.dropped_event_triggers.contains(&event_trigger)
            }
            Object::Index(index) => self.dropped_indexes.contains(&index),
            Object::Constraint(constraint) => self.dropped_constraints.contains(&constraint),
            Object::Policy(policy) => self.dropped_policies.contains(&policy),
            Object::Rule(rule) => self.dropped_rules.contains(&rule),
            Object::Cast(cast) => self.dropped_casts.contains(&cast),
            Object::Transform(transform) => {
                let key = self.index.transforms[transform].key;
                self.transforms.get(&key) != Some(&transform)
            }
            Object::Operator(operator) => self.dropped_operators.contains(&operator),
            Object::AccessMethod(access_method) => {
                self.dropped_access_methods.contains(&access_method)
            }
            Object::Language(language) => self.dropped_languages.contains(&language),
            Object::ForeignDataWrapper(wrapper) => {
                self.dropped_foreign_data_wrappers.contains(&wrapper)
            }
            Object::Conversion(conversion) => self.dropped_conversions.contains(&conversion),
            Object::TextSearch(text_search) => self.dropped_text_search.contains(&text_search),
            Object::Collation(collation) => self.dropped_collations.contains(&collation),
            Object::OperatorFamily(family) => self.dropped_operator_families.contains(&family),
            Object::OperatorClass(class) => self.dropped_operator_classes.contains(&class),
            Object::Statistics(statistics) => self.dropped_statistics.contains(&statistics),
            Object::Extension(extension) => self.dropped_extensions.contains(&extension),
            Object::Schema(schema) => self.dropped_schemas.contains(&schema),
            Object::Server(server) => self.dropped_servers.contains(&server),
            Object::Publication(publication) => self.dropped_publications.contains(&publication),
            Object::ColumnExpr(id) => self
                .column_exprs
                .column_of(id)
                .is_none_or(|key| self.dropped_tables.contains(&key.table)),
            Object::OperatorFamilyMember(member) => {
                self.dropped_operator_family_members.contains(&member)
                    || match self.index.operator_family_members[member].owner {
                        MemberOwner::Class(class) => self.dropped_operator_classes.contains(&class),
                        MemberOwner::Family(family) => {
                            self.dropped_operator_families.contains(&family)
                        }
                    }
            }
        }
    }

    fn dependents(&self, object: Object) -> SmallVec<[Object; 4]> {
        let mut referenced: SmallVec<[Object; 4]> = smallvec![object];
        let mut i = 0;
        while let Some(&current) = referenced.get(i) {
            let replaced = if let Object::Function(function) = current {
                self.dependencies.replaced_by(function)
            } else {
                &[]
            };
            let previous = self
                .dependencies
                .moved_from(current)
                .iter()
                .copied()
                .chain(replaced.iter().map(|&old| Object::Function(old)));
            for previous in previous {
                if !referenced.contains(&previous) {
                    referenced.push(previous);
                }
            }
            i += 1;
        }
        let mut dependents: SmallVec<[Object; 4]> = referenced
            .iter()
            .flat_map(|&current| self.dependencies.dependents_of(current))
            .copied()
            .collect();
        let mut i = 0;
        while let Some(&current) = dependents.get(i) {
            dependents.extend_from_slice(self.dependencies.moved_to(current));
            i += 1;
        }
        dependents.sort_unstable();
        dependents.dedup();
        dependents.retain(|dependent| !self.is_dropped(*dependent));
        dependents
    }

    fn depend_on_refs<T>(
        &mut self,
        refs: RefId,
        dependents: IdxRange<T>,
        object: fn(Idx<T>) -> Object,
    ) {
        for reference in self.index.references.range_from(refs) {
            let referenced = match self.index.references[reference].resolution {
                Resolution::Function(function) => Object::Function(function),
                Resolution::Table(table) if self.index.tables.is_local(table) => {
                    Object::Table(table)
                }
                Resolution::Type(ty) if self.index.types.is_local(ty) => Object::Type(ty),
                Resolution::Table(_)
                | Resolution::Column { .. }
                | Resolution::Source(_)
                | Resolution::Cte(_)
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
                | Resolution::Unknown
                | Resolution::Unresolved
                | Resolution::Ambiguous => continue,
            };
            for dependent in dependents {
                let dependent = object(dependent);
                if dependent == referenced
                    || matches!(referenced, Object::Table(table) if self.owner_table(dependent) == Some(table))
                {
                    continue;
                }
                self.dependencies.add_dependency(dependent, referenced);
            }
        }
    }

    fn owner_table(&self, object: Object) -> Option<TableId> {
        match object {
            Object::Table(table) => Some(table),
            Object::Index(index) => self.index.indexes[index].table,
            Object::Trigger(trigger) => Some(self.index.triggers[trigger].table),
            Object::Policy(policy) => Some(self.index.policies[policy].table),
            Object::Rule(rule) => Some(self.index.rules[rule].table),
            Object::Statistics(statistics) => self.index.statistics[statistics].table,
            Object::Constraint(constraint) => match self.index.constraints[constraint].owner {
                ConstraintOwner::Table(table) => Some(table),
                ConstraintOwner::Domain(_) => None,
            },
            Object::ColumnExpr(id) => self.column_exprs.column_of(id).map(|key| key.table),
            Object::Function(_)
            | Object::OperatorFamilyMember(_)
            | Object::Type(_)
            | Object::EventTrigger(_)
            | Object::Cast(_)
            | Object::Transform(_)
            | Object::Operator(_)
            | Object::AccessMethod(_)
            | Object::Language(_)
            | Object::ForeignDataWrapper(_)
            | Object::Conversion(_)
            | Object::TextSearch(_)
            | Object::Collation(_)
            | Object::OperatorFamily(_)
            | Object::OperatorClass(_)
            | Object::Extension(_)
            | Object::Schema(_)
            | Object::Server(_)
            | Object::Publication(_) => None,
        }
    }

    fn drop_table_with_dependents(&mut self, table: TableId) {
        let dependents = self.dependents(Object::Table(table));
        let columns = self.table_typed_columns(table);
        self.drop_table(table);
        self.drop_owned_by_table(table);
        for dependent in dependents {
            self.drop_object(dependent);
        }
        self.drop_columns(&columns);
        let mut i = 0;
        while let Some(&inheritance) = self.inheritances.children(table).get(i) {
            i += 1;
            if self.is_live_child(inheritance.child) {
                self.drop_object(Object::Table(inheritance.child));
            }
        }
    }

    fn drop_type_with_dependents(&mut self, ty: TypeId) {
        let dependents = self.type_dependents(ty);
        let columns = self.type_typed_columns(ty);
        self.drop_type(ty);
        let constraints: SmallVec<[ConstraintId; 4]> = self
            .index
            .constraints
            .owned_by(ConstraintOwner::Domain(ty))
            .rev()
            .collect();
        for id in constraints {
            self.drop_constraint(id);
        }
        for dependent in dependents {
            self.drop_object(dependent);
        }
        self.drop_columns(&columns);
    }

    fn companion_types(&self, ty: TypeId) -> SmallVec<[TypeId; 2]> {
        let range = self.index.types[ty].range.unwrap_or(ty);
        std::iter::once(range)
            .chain(self.index.types.with_range(range))
            .collect()
    }

    fn type_dependents(&self, ty: TypeId) -> SmallVec<[Object; 4]> {
        let mut dependents: SmallVec<[Object; 4]> = SmallVec::new();
        for id in self.companion_types(ty) {
            for dependent in self.dependents(Object::Type(id)) {
                if !dependents.contains(&dependent) {
                    dependents.push(dependent);
                }
            }
            for table in self.typed_tables(id) {
                if !dependents.contains(&Object::Table(table)) {
                    dependents.push(Object::Table(table));
                }
            }
        }
        dependents
    }

    fn table_typed_columns(&self, table: TableId) -> SmallVec<[TypedColumn; 4]> {
        self.typed_columns(&[ColumnTypeRef::Table(table)])
    }

    fn type_typed_columns(&self, ty: TypeId) -> SmallVec<[TypedColumn; 4]> {
        let referenced: SmallVec<[ColumnTypeRef; 2]> = self
            .companion_types(ty)
            .into_iter()
            .map(ColumnTypeRef::Type)
            .collect();
        self.typed_columns(&referenced)
    }

    fn typed_columns(&self, referenced: &[ColumnTypeRef]) -> SmallVec<[TypedColumn; 4]> {
        let mut columns: SmallVec<[TypedColumn; 4]> = SmallVec::new();
        let mut push = |owner: TypedColumnOwner, range: IdxRange<Column>| {
            for column in range {
                let Column { name, ty, .. } = self.index.columns[column];
                if let (Some(name), Some(ty)) = (name, ty)
                    && ColumnTypeRef::of(ty).is_some_and(|ty| referenced.contains(&ty))
                {
                    columns.push(TypedColumn { owner, name });
                }
            }
        };
        let mut tables: SmallVec<[TableId; 4]> = referenced
            .iter()
            .flat_map(|&column_type| self.index.tables.with_column_type(column_type))
            .collect();
        tables.sort_unstable();
        tables.dedup();
        for id in tables {
            let table = &self.index.tables[id];
            if self.index.tables.is_local(id)
                && !self.dropped_tables.contains(&id)
                && matches!(table.kind, TableKind::Table | TableKind::ForeignTable)
            {
                push(TypedColumnOwner::Table(id), table.columns.range);
            }
        }
        let mut types: SmallVec<[TypeId; 4]> = referenced
            .iter()
            .flat_map(|&column_type| self.index.types.with_column_type(column_type))
            .collect();
        types.sort_unstable();
        types.dedup();
        for id in types {
            if self.index.types.is_local(id)
                && !self.dropped_types.contains(&id)
                && let Some(type_columns) = self.index.types[id].columns
            {
                push(TypedColumnOwner::Type(id), type_columns.range);
            }
        }
        columns
    }

    fn drop_columns(&mut self, columns: &[TypedColumn]) {
        for &TypedColumn {
            owner,
            name: column,
        } in columns
        {
            match owner {
                TypedColumnOwner::Table(table) => {
                    if !self.dropped_tables.contains(&table) {
                        self.drop_table_column(table, column);
                    }
                }
                TypedColumnOwner::Type(ty) => {
                    if !self.dropped_types.contains(&ty)
                        && let Some(type_columns) = self.index.types[ty].columns
                    {
                        let range = self.copy_columns(type_columns.range, |columns, id| {
                            (columns[id].name != Some(column)).then_some(columns[id])
                        });
                        self.set_type_columns(ty, range);
                    }
                }
            }
        }
    }

    fn drop_owned_by_table(&mut self, table: TableId) {
        for id in self.index.indexes.on_table(table) {
            self.drop_index(id);
        }
        let triggers: SmallVec<[TriggerId; 4]> = self.index.triggers.get(table).rev().collect();
        for id in triggers {
            self.drop_trigger(id);
        }
        let policies: SmallVec<[PolicyId; 4]> = self.index.policies.get(table).rev().collect();
        for id in policies {
            self.drop_policy(id);
        }
        let rules: SmallVec<[RuleId; 4]> = self.index.rules.get(table).rev().collect();
        for id in rules {
            self.drop_rule(id);
        }
        let statistics: SmallVec<[StatisticsId; 4]> =
            self.index.statistics.on_table(table).rev().collect();
        for id in statistics {
            self.drop_statistics(id);
        }
        let constraints: SmallVec<[ConstraintId; 8]> = self
            .index
            .constraints
            .owned_by(ConstraintOwner::Table(table))
            .rev()
            .collect();
        for id in constraints {
            self.drop_constraint(id);
        }
        for id in self.index.tables.owned_sequences(table) {
            self.drop_object(Object::Table(id));
        }
    }

    fn drop_relation_refs(
        &mut self,
        paths: impl Iterator<Item = ast::PathRef>,
        if_missing: IfMissing,
        lookup: TableLookup,
        drop_behavior: Option<&ast::DropBehavior>,
    ) {
        let mut targets: SmallVec<[(ast::PathRef, TableId); 4]> = SmallVec::new();
        for path in paths {
            if let Some(table) = self.lower_drop_table_ref(&path, if_missing, lookup) {
                targets.push((path, table));
            }
        }
        let cascade = matches!(drop_behavior, Some(ast::DropBehavior::Cascade(_)));
        for (path, table) in &targets {
            if cascade {
                continue;
            }
            let is_target = |owner: TableId| targets.iter().any(|(_, target)| *target == owner);
            if self.drop_blocked(*table, &is_target) {
                let table = &self.index.tables[*table];
                let error = SemaError::DependentObjects {
                    object: ObjectKind::Table(table.kind),
                    name: table.name,
                };
                self.push_error(path.syntax(), error);
                return;
            }
        }
        for (_, table) in targets {
            self.drop_object(Object::Table(table));
        }
    }

    fn drop_blocked(&self, table: TableId, is_target: &dyn Fn(TableId) -> bool) -> bool {
        self.dependents(Object::Table(table))
            .into_iter()
            .any(|dependent| {
                self.owner_table(dependent)
                    .is_none_or(|owner| !is_target(owner))
            })
            || self
                .table_typed_columns(table)
                .iter()
                .any(|column| match column.owner {
                    TypedColumnOwner::Table(owner) => !is_target(owner),
                    TypedColumnOwner::Type(_) => true,
                })
            || self.inheritances.children(table).iter().any(|inheritance| {
                let child = inheritance.child;
                if !self.is_live_child(child) || is_target(child) {
                    return false;
                }
                match inheritance.kind {
                    InheritanceKind::Inherits => true,
                    InheritanceKind::Partition => {
                        self.drop_blocked(child, &|owner| owner == child || is_target(owner))
                    }
                }
            })
    }

    fn record_moves<T>(&mut self, from: Object, moved: IdxRange<T>, object: fn(Idx<T>) -> Object) {
        for to in moved {
            let to = object(to);
            if let (Some(from), Some(to)) = (from.item(), to.item()) {
                self.copy_comment(from, to);
            }
            self.dependencies.add_move(from, to);
        }
    }

    fn drop_transform(&mut self, transform: TransformId) {
        let key = self.index.transforms[transform].key;
        if self.transforms.get(&key) != Some(&transform) {
            return;
        }
        self.transforms.remove(&key);
        if self.in_transaction() {
            self.undo.push(Undo::DropTransform(key, transform));
        }
    }

    fn find_cast(&self, types: CastTypes) -> Option<CastId> {
        self.index
            .casts
            .get(types)
            .find(|cast| !self.dropped_casts.contains(cast))
    }

    fn drop_cast(&mut self, cast: CastId) {
        if self.dropped_casts.insert(cast) && self.in_transaction() {
            self.undo.push(Undo::DropCast(cast));
        }
    }

    fn drop_conversion(&mut self, conversion: ConversionId) {
        if self.dropped_conversions.insert(conversion) && self.in_transaction() {
            self.undo.push(Undo::DropConversion(conversion));
        }
    }

    fn drop_operator_family(&mut self, family: OperatorFamilyId) {
        if self.dropped_operator_families.insert(family) && self.in_transaction() {
            self.undo.push(Undo::DropOperatorFamily(family));
        }
    }

    fn drop_operator_class(&mut self, class: OperatorClassId) {
        if self.dropped_operator_classes.insert(class) && self.in_transaction() {
            self.undo.push(Undo::DropOperatorClass(class));
        }
    }

    fn drop_statistics(&mut self, statistics: StatisticsId) {
        if self.dropped_statistics.insert(statistics) && self.in_transaction() {
            self.undo.push(Undo::DropStatistics(statistics));
        }
    }

    fn drop_index(&mut self, index: IndexId) {
        if self.dropped_indexes.insert(index) && self.in_transaction() {
            self.undo.push(Undo::DropIndex(index));
        }
    }

    fn move_extension(&mut self, extension: ExtensionId, schema: Name) {
        let in_transaction = self.in_transaction();
        let previous = self.index.extensions[extension].schema;
        if !self.index.extensions.set_schema(extension, schema) {
            return;
        }
        self.move_extension_members(extension, previous, schema);
        if in_transaction {
            self.undo.push(Undo::MoveExtension(extension, previous));
        }
    }

    fn drop_extension(&mut self, extension: ExtensionId) {
        if self.dropped_extensions.insert(extension) && self.in_transaction() {
            self.undo.push(Undo::DropExtension(extension));
        }
    }

    fn move_extension_members(&mut self, extension: ExtensionId, from: Name, to: Name) {
        let in_transaction = self.in_transaction();
        for member in self.dependents(Object::Extension(extension)) {
            if self.in_schema(member, from)
                && set_object_schema(&mut self.index, member, to)
                && in_transaction
            {
                self.undo.push(Undo::MoveObjectSchema(member, from));
            }
        }
    }

    fn drop_extension_with_dependents(&mut self, extension: ExtensionId) {
        let dependents = self.dependents(Object::Extension(extension));
        self.drop_extension(extension);
        for dependent in dependents {
            self.drop_object(dependent);
        }
    }

    fn add_inheritance(&mut self, parent: TableId, child: TableId, kind: InheritanceKind) {
        let inheritance = Inheritance {
            parent,
            child,
            kind,
        };
        if self.inheritances.add(inheritance) && self.in_transaction() {
            self.undo.push(Undo::AddInheritance(inheritance));
        }
    }

    fn remove_inheritance(&mut self, parent: TableId, child: TableId, kind: InheritanceKind) {
        let inheritance = Inheritance {
            parent,
            child,
            kind,
        };
        if self.inheritances.remove(inheritance) && self.in_transaction() {
            self.undo.push(Undo::RemoveInheritance(inheritance));
        }
    }

    fn remove_dependency(&mut self, dependent: Object, referenced: Object) {
        if self.dependencies.remove_dependency(dependent, referenced) && self.in_transaction() {
            self.undo
                .push(Undo::RemoveDependency(dependent, referenced));
        }
    }

    fn drop_operator(&mut self, operator: OperatorId) {
        if self.dropped_operators.insert(operator) && self.in_transaction() {
            self.undo.push(Undo::DropOperator(operator));
        }
    }

    fn drop_collation(&mut self, collation: CollationId) {
        if self.dropped_collations.insert(collation) && self.in_transaction() {
            self.undo.push(Undo::DropCollation(collation));
        }
    }

    fn drop_constraint(&mut self, constraint: ConstraintId) {
        if self.dropped_constraints.insert(constraint) && self.in_transaction() {
            self.undo.push(Undo::DropConstraint(constraint));
        }
        if let Some(index) = self.index.constraints[constraint].index {
            self.drop_index(index);
        }
    }

    fn lower_savepoint(&mut self, savepoint_create: &ast::SavepointCreate) {
        let Some(savepoint) = savepoint_create.savepoint() else {
            return;
        };
        let name = self.intern(&savepoint.text());
        let id = self.index.savepoints.alloc(Savepoint {
            name,
            ptr: self.ast_ptr(savepoint_create),
            name_ptr: self.ast_ptr(&savepoint),
        });
        self.record_comment(savepoint_create.syntax(), Item::Savepoint(id));
        self.add_item(savepoint.syntax(), Item::Savepoint(id));
        self.savepoints.push(OpenSavepoint {
            id,
            snapshot: self.snapshot(),
        });
    }

    fn lower_release_savepoint(&mut self, release: &ast::ReleaseSavepoint) {
        if let Some(idx) = release
            .savepoint_ref()
            .and_then(|savepoint_ref| self.lower_savepoint_ref(&savepoint_ref))
        {
            self.savepoints.truncate(idx);
        }
    }

    fn lower_savepoint_ref(&mut self, savepoint_ref: &ast::SavepointRef) -> Option<usize> {
        let name = self.intern(&savepoint_ref.text());
        let idx = self
            .savepoints
            .iter()
            .rposition(|savepoint| self.index.savepoints[savepoint.id].name == name);
        let resolution = idx
            .map(|idx| self.savepoints[idx].id)
            .map_or(Resolution::Unresolved, Resolution::Savepoint);
        self.add_ref(
            name,
            ReferenceKind::Savepoint,
            savepoint_ref.syntax(),
            resolution,
        );
        idx
    }

    fn lower_declare(&mut self, declare: &ast::Declare) {
        if let Some(query) = declare.query() {
            self.lower_query(query);
        }
        let Some(cursor) = declare.cursor() else {
            return;
        };
        let name = self.intern(&cursor.text());
        let hold = matches!(declare.cursor_hold(), Some(ast::CursorHold::WithHold(_)));
        let id = self.index.cursors.alloc(Cursor {
            name,
            hold,
            ptr: self.ast_ptr(declare),
            name_ptr: self.ast_ptr(&cursor),
        });
        self.record_comment(declare.syntax(), Item::Cursor(id));
        self.add_item(cursor.syntax(), Item::Cursor(id));
        self.cursors.insert(name, id);
    }

    fn lower_close(&mut self, close: &ast::Close) {
        if close.all_token().is_some() {
            self.cursors.clear();
        } else if let Some(cursor) = close
            .cursor_ref()
            .and_then(|cursor_ref| self.lower_cursor_ref(&cursor_ref))
        {
            let name = self.index.cursors[cursor].name;
            self.cursors.remove(&name);
        }
    }

    fn lower_fetch(&mut self, fetch: &ast::Fetch) {
        if let Some(cursor_ref) = fetch.cursor_ref() {
            self.lower_cursor_ref(&cursor_ref);
        }
    }

    fn lower_move(&mut self, move_: &ast::Move) {
        if let Some(cursor_ref) = move_.cursor_ref() {
            self.lower_cursor_ref(&cursor_ref);
        }
    }

    fn lower_prepare(&mut self, prepare: &ast::Prepare) {
        let start = self.index.params.next_idx();
        for param in prepare
            .param_list()
            .into_iter()
            .flat_map(|list| list.params())
        {
            let param = self.param_def(&param);
            self.index.params.alloc(param);
        }
        let params = self.index.params.range_from(start);
        let outer = self.prepare_params.replace(params);
        let columns = match prepare.stmt() {
            Some(stmt) => {
                let query = self.lower_preparable_stmt(&stmt);
                self.index.queries[query].columns
            }
            None => self.unknown_columns(),
        };
        self.prepare_params = outer;
        let Some(prepared) = prepare.name() else {
            return;
        };
        let name = self.intern(&prepared.text());
        let id = self.index.prepared_statements.alloc(PreparedStatement {
            name,
            columns,
            ptr: self.ast_ptr(prepare),
            name_ptr: self.ast_ptr(&prepared),
        });
        self.index
            .set_column_owner(columns.range, ColumnOwner::PreparedStatement(id));
        self.record_comment(prepare.syntax(), Item::PreparedStatement(id));
        self.add_item(prepared.syntax(), Item::PreparedStatement(id));
        if let Entry::Vacant(entry) = self.prepared_statements.entry(name) {
            entry.insert(id);
        } else {
            self.report_duplicate(ObjectKind::PreparedStatement, prepared.syntax(), name);
        }
    }

    fn lower_preparable_stmt(&mut self, stmt: &ast::PreparableStmt) -> QueryId {
        match stmt {
            ast::PreparableStmt::CompoundSelect(compound) => self.lower_compound(compound, None),
            ast::PreparableStmt::ParenSelect(paren) => self.lower_paren_select(paren, None),
            ast::PreparableStmt::Select(select) => self.lower_simple_select(select),
            ast::PreparableStmt::SelectInto(select) => self.lower_select_into(select),
            ast::PreparableStmt::Table(table) => self.lower_table_query(table),
            ast::PreparableStmt::Values(values) => self.lower_values(values),
            ast::PreparableStmt::Delete(delete) => self.lower_delete(delete),
            ast::PreparableStmt::Insert(insert) => self.lower_insert(insert),
            ast::PreparableStmt::Update(update) => self.lower_update(update),
            ast::PreparableStmt::Merge(merge) => self.lower_merge(merge),
        }
    }

    fn lower_execute(&mut self, execute: &ast::Execute) -> Option<PreparedStatementId> {
        for arg in execute.arg_list().into_iter().flat_map(|list| list.args()) {
            if let Some(expr) = arg.func_arg_expr() {
                self.with_expr_context(ExprContext::ExecuteParameter, |this| {
                    this.lower_func_arg_expr(&expr);
                });
            }
        }
        self.lower_prepared_statement_ref(&execute.prepared_statement_ref()?)
    }

    fn lower_deallocate(&mut self, deallocate: &ast::Deallocate) {
        if deallocate.all_token().is_some() {
            self.prepared_statements.clear();
        } else if let Some(prepared) = deallocate
            .prepared_statement_ref()
            .and_then(|prepared_ref| self.lower_prepared_statement_ref(&prepared_ref))
        {
            let name = self.index.prepared_statements[prepared].name;
            self.prepared_statements.remove(&name);
        }
    }

    fn lower_prepared_statement_ref(
        &mut self,
        prepared_ref: &ast::PreparedStatementRef,
    ) -> Option<PreparedStatementId> {
        let name = self.intern(&prepared_ref.text());
        let prepared = self.prepared_statements.get(&name).copied();
        self.add_ref(
            name,
            ReferenceKind::PreparedStatement,
            prepared_ref.syntax(),
            prepared.map_or(Resolution::Unresolved, Resolution::PreparedStatement),
        );
        prepared
    }

    fn lower_listen(&mut self, listen: &ast::Listen) {
        let Some(channel) = listen.channel() else {
            return;
        };
        let name = self.intern(&channel.text());
        if let Some(&existing) = self.channels.get(&name) {
            self.add_ref(
                name,
                ReferenceKind::Channel,
                channel.syntax(),
                Resolution::Channel(existing),
            );
            return;
        }
        let id = self.index.channels.alloc(Channel {
            name,
            name_ptr: self.ast_ptr(&channel),
        });
        self.record_comment(listen.syntax(), Item::Channel(id));
        self.add_item(channel.syntax(), Item::Channel(id));
        self.channels.insert(name, id);
    }

    fn lower_notify(&mut self, notify: &ast::Notify) {
        if let Some(channel_ref) = notify.channel_ref() {
            self.lower_channel_ref(&channel_ref);
        }
    }

    fn lower_unlisten(&mut self, unlisten: &ast::Unlisten) {
        if unlisten.star_token().is_some() {
            self.channels.clear();
        } else if let Some(channel_ref) = unlisten.channel_ref() {
            let name = self.lower_channel_ref(&channel_ref);
            self.channels.remove(&name);
        }
    }

    fn lower_channel_ref(&mut self, channel_ref: &ast::ChannelRef) -> Name {
        let name = self.intern(&channel_ref.text());
        let channel = self.channels.get(&name).copied();
        self.add_ref(
            name,
            ReferenceKind::Channel,
            channel_ref.syntax(),
            channel.map_or(Resolution::Unresolved, Resolution::Channel),
        );
        name
    }

    fn lower_explain(&mut self, explain: &ast::Explain) {
        let Some(stmt) = explain.explain_stmt() else {
            return;
        };
        let analyze = explain.explain_mode().is_some_and(|mode| mode.is_analyze());
        match stmt {
            ast::ExplainStmt::CompoundSelect(compound) => {
                self.lower_compound(&compound, None);
            }
            ast::ExplainStmt::ParenSelect(paren) => {
                self.lower_paren_select(&paren, None);
            }
            ast::ExplainStmt::Select(select) => {
                self.lower_simple_select(&select);
            }
            ast::ExplainStmt::Table(table) => {
                self.lower_table_query(&table);
            }
            ast::ExplainStmt::Values(values) => {
                self.lower_values(&values);
            }
            ast::ExplainStmt::Delete(delete) => {
                self.lower_delete(&delete);
            }
            ast::ExplainStmt::Insert(insert) => {
                self.lower_insert(&insert);
            }
            ast::ExplainStmt::Update(update) => {
                self.lower_update(&update);
            }
            ast::ExplainStmt::Merge(merge) => {
                self.lower_merge(&merge);
            }
            ast::ExplainStmt::Execute(execute) => {
                self.lower_execute(&execute);
            }
            ast::ExplainStmt::Declare(declare) => {
                if let Some(query) = declare.query() {
                    self.lower_query(query);
                }
            }
            ast::ExplainStmt::SelectInto(select) => {
                if analyze {
                    self.lower_select_into_stmt(&select);
                } else {
                    self.lower_select_into(&select);
                }
            }
            ast::ExplainStmt::CreateTableAs(create_table_as) => {
                if analyze {
                    self.lower_create_table_as(&create_table_as);
                } else if let Some(query) = create_table_as.query() {
                    self.lower_create_table_as_query(query);
                }
            }
            ast::ExplainStmt::CreateMaterializedView(create_view) => {
                if analyze {
                    self.lower_create_materialized_view(&create_view);
                } else if let Some(query) = create_view.query() {
                    self.lower_query(query);
                }
            }
        }
    }

    fn lower_cursor_ref(&mut self, cursor_ref: &ast::CursorRef) -> Option<CursorId> {
        let name = self.intern(&cursor_ref.text());
        let cursor = self.cursors.get(&name).copied();
        self.add_ref(
            name,
            ReferenceKind::Cursor,
            cursor_ref.syntax(),
            cursor.map_or(Resolution::Unresolved, Resolution::Cursor),
        );
        cursor
    }

    fn lower_create_role(&mut self, create_role: &ast::CreateRole) {
        self.lower_create_role_like(
            create_role.syntax(),
            create_role.role(),
            create_role.role_option_list(),
        );
    }

    fn lower_create_user(&mut self, create_user: &ast::CreateUser) {
        self.lower_create_role_like(
            create_user.syntax(),
            create_user.role(),
            create_user.role_option_list(),
        );
    }

    fn lower_create_group(&mut self, create_group: &ast::CreateGroup) {
        self.lower_create_role_like(
            create_group.syntax(),
            create_group.role(),
            create_group.role_option_list(),
        );
    }

    fn lower_create_role_like(
        &mut self,
        node: &SyntaxNode,
        role: Option<ast::Role>,
        options: Option<ast::RoleOptionList>,
    ) {
        if let Some(options) = options {
            self.lower_role_options(&options);
        }
        if let Some(role) = role
            && role.kind() == Some(ast::RoleKind::Name)
        {
            self.alloc_role(node, &role);
        }
    }

    fn lower_role_options(&mut self, options: &ast::RoleOptionList) {
        for option in options.role_options() {
            let list = match option {
                ast::RoleOption::RoleOptionAdmin(admin) => admin.role_ref_list(),
                ast::RoleOption::RoleOptionInGroup(in_group) => in_group.role_ref_list(),
                ast::RoleOption::RoleOptionInRole(in_role) => in_role.role_ref_list(),
                ast::RoleOption::RoleOptionRole(role) => role.role_ref_list(),
                ast::RoleOption::RoleOptionUser(user) => user.role_ref_list(),
                ast::RoleOption::RoleOptionConnectionLimit(_)
                | ast::RoleOption::RoleOptionEncryptedPassword(_)
                | ast::RoleOption::RoleOptionGeneric(_)
                | ast::RoleOption::RoleOptionInherit(_)
                | ast::RoleOption::RoleOptionPassword(_)
                | ast::RoleOption::RoleOptionSysid(_)
                | ast::RoleOption::RoleOptionValidUntil(_) => None,
            };
            if let Some(list) = list {
                self.lower_role_ref_list(&list, IfMissing::Error);
            }
        }
    }

    fn lower_alter_role(&mut self, alter_role: &ast::AlterRole) {
        let role = alter_role
            .role_ref()
            .and_then(|role_ref| self.lower_role_ref(&role_ref, IfMissing::Error));
        if let Some(database) = alter_role
            .in_database()
            .and_then(|in_database| in_database.database_ref())
        {
            self.lower_database_ref(&database, IfMissing::Error);
        }
        if let Some(action) = alter_role.action() {
            self.lower_alter_role_action(role, &action, alter_role.syntax());
        }
    }

    fn lower_alter_role_action(
        &mut self,
        role: Option<RoleId>,
        action: &ast::AlterRoleAction,
        node: &SyntaxNode,
    ) {
        match action {
            ast::AlterRoleAction::RoleOptionList(options) => {
                self.lower_role_options(options);
            }
            ast::AlterRoleAction::RoleRenameTo(rename) => {
                self.rename_role(role, rename, node);
            }
            ast::AlterRoleAction::SetConfigParam(param) => {
                self.lower_set_config_param(param);
            }
            ast::AlterRoleAction::ResetConfigParam(_) => {}
        }
    }

    fn lower_alter_user(&mut self, alter_user: &ast::AlterUser) {
        let role = alter_user
            .role_ref()
            .and_then(|role_ref| self.lower_role_ref(&role_ref, IfMissing::Error));
        if let Some(database) = alter_user
            .in_database()
            .and_then(|in_database| in_database.database_ref())
        {
            self.lower_database_ref(&database, IfMissing::Error);
        }
        if let Some(action) = alter_user.action() {
            self.lower_alter_role_action(role, &action.into(), alter_user.syntax());
        }
    }

    fn lower_alter_group(&mut self, alter_group: &ast::AlterGroup) {
        let role = alter_group
            .role_ref()
            .and_then(|role_ref| self.lower_role_ref(&role_ref, IfMissing::Error));
        match alter_group.action() {
            Some(ast::AlterGroupAction::AddUsers(add_users)) => {
                for role_ref in add_users.role_refs() {
                    self.lower_role_ref(&role_ref, IfMissing::Error);
                }
            }
            Some(ast::AlterGroupAction::DropUsers(drop_users)) => {
                for role_ref in drop_users.role_refs() {
                    self.lower_role_ref(&role_ref, IfMissing::Error);
                }
            }
            Some(ast::AlterGroupAction::RoleRenameTo(rename)) => {
                self.rename_role(role, &rename, alter_group.syntax());
            }
            None => {}
        }
    }

    fn rename_role(&mut self, role: Option<RoleId>, rename: &ast::RoleRenameTo, node: &SyntaxNode) {
        let (Some(role), Some(new_role)) = (role, rename.role()) else {
            return;
        };
        if new_role.ident_token().is_none() {
            return;
        }
        let new_name = self.intern(&new_role.text());
        if self.find_role(new_name).is_some() {
            self.report_duplicate(ObjectKind::Role, new_role.syntax(), new_name);
            return;
        }
        self.drop_role(role);
        let id = self.alloc_role(node, &new_role);
        self.copy_comment(Item::Role(role), Item::Role(id));
    }

    fn lower_drop_role(&mut self, drop_role: &ast::DropRole) {
        self.lower_drop_role_like(
            IfMissing::from_if_exists(drop_role.if_exists().as_ref()),
            drop_role.role_refs(),
        );
    }

    fn lower_drop_user(&mut self, drop_user: &ast::DropUser) {
        self.lower_drop_role_like(
            IfMissing::from_if_exists(drop_user.if_exists().as_ref()),
            drop_user.role_refs(),
        );
    }

    fn lower_drop_group(&mut self, drop_group: &ast::DropGroup) {
        self.lower_drop_role_like(
            IfMissing::from_if_exists(drop_group.if_exists().as_ref()),
            drop_group.role_refs(),
        );
    }

    fn lower_drop_role_like(
        &mut self,
        if_missing: IfMissing,
        role_refs: ast::AstChildren<ast::RoleRef>,
    ) {
        for role_ref in role_refs {
            if let Some(role) = self.lower_role_ref(&role_ref, if_missing) {
                self.drop_role(role);
            }
        }
    }

    fn lower_set_role(&mut self, set_role: &ast::SetRole) {
        match set_role.set_role_target() {
            Some(ast::SetRoleTarget::RoleRef(role_ref)) => {
                self.lower_role_ref(&role_ref, IfMissing::Error);
            }
            Some(ast::SetRoleTarget::Literal(literal)) => self.lower_role_literal(&literal),
            Some(ast::SetRoleTarget::SetRoleNone(_)) | None => {}
        }
    }

    fn lower_set_session_auth(&mut self, set_session_auth: &ast::SetSessionAuth) {
        match set_session_auth.set_session_auth_target() {
            Some(ast::SetSessionAuthTarget::RoleRef(role_ref)) => {
                self.lower_role_ref(&role_ref, IfMissing::Error);
            }
            Some(ast::SetSessionAuthTarget::Literal(literal)) => self.lower_role_literal(&literal),
            Some(ast::SetSessionAuthTarget::SetSessionAuthDefault(_)) | None => {}
        }
    }

    fn lower_drop_owned(&mut self, drop_owned: &ast::DropOwned) {
        if let Some(list) = drop_owned.role_ref_list() {
            self.lower_role_ref_list(&list, IfMissing::Error);
        }
    }

    fn lower_reassign(&mut self, reassign: &ast::Reassign) {
        for list in [reassign.before(), reassign.after()].into_iter().flatten() {
            self.lower_role_ref_list(&list, IfMissing::Error);
        }
    }

    fn lower_owner_to(&mut self, owner_to: &ast::OwnerTo) {
        if let Some(role_ref) = owner_to.role_ref() {
            self.lower_role_ref(&role_ref, IfMissing::Error);
        }
    }

    fn lower_role_ref_list(&mut self, list: &ast::RoleRefList, if_missing: IfMissing) {
        for role_ref in list.role_refs() {
            self.lower_role_ref(&role_ref, if_missing);
        }
    }

    fn lower_role_ref(&mut self, role_ref: &ast::RoleRef, if_missing: IfMissing) -> Option<RoleId> {
        match role_ref.role_ref_value() {
            Some(ast::RoleRefValue::RoleNameRef(name_ref)) => {
                let name = self.intern(&name_ref.text());
                self.add_role_ref(name, name_ref.syntax(), if_missing)
            }
            Some(
                ast::RoleRefValue::RoleRefCurrentRole(_)
                | ast::RoleRefValue::RoleRefCurrentUser(_)
                | ast::RoleRefValue::RoleRefSessionUser(_),
            )
            | None => None,
        }
    }

    fn lower_role_literal(&mut self, literal: &ast::Literal) {
        if let Some(name) = literal.string_value() {
            let name = self.intern(&name);
            self.add_role_ref(name, literal.syntax(), IfMissing::Error);
        }
    }

    fn add_role_ref(
        &mut self,
        name: Name,
        node: &SyntaxNode,
        if_missing: IfMissing,
    ) -> Option<RoleId> {
        let role = self.find_role(name);
        let resolution = match (role, if_missing) {
            (Some(role), _) => Resolution::Role(role),
            (None, IfMissing::Ignore) => Resolution::Unknown,
            (None, IfMissing::Error) => Resolution::Unresolved,
        };
        self.add_ref(name, ReferenceKind::Role, node, resolution);
        role
    }

    fn alloc_role(&mut self, node: &SyntaxNode, name_node: &impl NameLike) -> RoleId {
        let name = self.intern(&name_node.text());
        if self.find_role(name).is_some() {
            self.report_duplicate(ObjectKind::Role, name_node.syntax(), name);
        }
        let id = self.index.roles.alloc(Role {
            name,
            ptr: self.ptr(node),
            name_ptr: self.ptr(name_node.syntax()),
        });
        self.record_comment(node, Item::Role(id));
        self.add_item(name_node.syntax(), Item::Role(id));
        id
    }

    fn find_role(&self, name: Name) -> Option<RoleId> {
        self.index
            .roles
            .get(name)
            .find(|id| !self.dropped_roles.contains(id))
    }

    fn lower_create_policy(&mut self, create_policy: &ast::CreatePolicy) {
        let refs = self.index.references.next_idx();
        let start = self.index.policies.next_idx();
        let table = create_policy
            .on_table()
            .and_then(|on_table| self.lower_policy_table(&on_table, IfMissing::Error));
        if let Some(roles) = create_policy.policy_roles() {
            self.lower_policy_roles(&roles);
        }
        self.lower_policy_exprs(
            table,
            create_policy.syntax(),
            create_policy.using_expr_clause(),
            create_policy.with_check_expr_clause(),
        );
        if let (Some(table), Some(name)) = (table, create_policy.policy()) {
            let name_node = name.syntax();
            let policy_name = self.intern(&name.text());
            if self.find_policy(table, policy_name).is_some() {
                self.report_duplicate_policy(name_node, policy_name, table);
            }
            self.alloc_policy(create_policy.syntax(), name_node, policy_name, table);
        }
        self.depend_on_refs(refs, self.index.policies.range_from(start), Object::Policy);
    }

    fn lower_alter_policy(&mut self, alter_policy: &ast::AlterPolicy) {
        let table = alter_policy
            .on_table()
            .and_then(|on_table| self.lower_policy_table(&on_table, IfMissing::Error));
        let policy = alter_policy
            .policy_ref()
            .and_then(|policy_ref| self.lower_policy_ref(table, &policy_ref, IfMissing::Error));
        match alter_policy.action() {
            Some(ast::AlterPolicyAction::AlterPolicyTo(alter_to)) => {
                if let Some(roles) = alter_to.policy_roles() {
                    self.lower_policy_roles(&roles);
                }
                self.lower_policy_exprs(
                    table,
                    alter_policy.syntax(),
                    alter_to.using_expr_clause(),
                    alter_to.with_check_expr_clause(),
                );
            }
            Some(ast::AlterPolicyAction::PolicyRenameTo(rename)) => {
                if let Some(policy) = policy
                    && let Some(name) = rename.policy()
                {
                    let table = self.index.policies[policy].table;
                    let new_name = self.intern(&name.text());
                    if self.find_policy(table, new_name).is_some() {
                        self.report_duplicate_policy(name.syntax(), new_name, table);
                        return;
                    }
                    let moved = self.index.policies.next_idx();
                    self.drop_policy(policy);
                    self.alloc_policy(alter_policy.syntax(), name.syntax(), new_name, table);
                    self.record_moves(
                        Object::Policy(policy),
                        self.index.policies.range_from(moved),
                        Object::Policy,
                    );
                }
            }
            None => {}
        }
    }

    fn lower_drop_policy(&mut self, drop_policy: &ast::DropPolicy) {
        let if_missing = IfMissing::from_if_exists(drop_policy.if_exists().as_ref());
        let table = drop_policy
            .on_table()
            .and_then(|on_table| self.lower_policy_table(&on_table, if_missing));
        if let Some(policy_ref) = drop_policy.policy_ref()
            && let Some(policy) = self.lower_policy_ref(table, &policy_ref, if_missing)
        {
            self.drop_policy(policy);
        }
    }

    fn lower_policy_table(
        &mut self,
        on_table: &ast::OnTable,
        if_missing: IfMissing,
    ) -> Option<TableId> {
        let path = on_table.table_name_ref()?.path_ref()?;
        self.lower_table_ref(&path, if_missing, TableLookup::Table)
    }

    fn lower_policy_roles(&mut self, roles: &ast::PolicyRoles) {
        for role_ref in roles
            .role_ref_list()
            .into_iter()
            .flat_map(|list| list.role_refs())
        {
            self.lower_grantee_role_ref(&role_ref);
        }
    }

    fn lower_grantee_role_ref(&mut self, role_ref: &ast::RoleRef) {
        if let Some(ast::RoleRefValue::RoleNameRef(name_ref)) = role_ref.role_ref_value()
            && is_public_role(&name_ref)
        {
            return;
        }
        self.lower_role_ref(role_ref, IfMissing::Error);
    }

    fn lower_policy_exprs(
        &mut self,
        table: Option<TableId>,
        node: &SyntaxNode,
        using: Option<ast::UsingExprClause>,
        with_check: Option<ast::WithCheckExprClause>,
    ) {
        if using.is_none() && with_check.is_none() {
            return;
        }
        self.with_table_scope(QueryKind::Policy, table, node, |this| {
            if let Some(expr) = using.and_then(|clause| clause.expr()) {
                this.lower_expr_in(ExprContext::Policy, &expr);
            }
            if let Some(expr) = with_check.and_then(|clause| clause.expr()) {
                this.lower_expr_in(ExprContext::Policy, &expr);
            }
        });
    }

    fn alloc_policy(
        &mut self,
        node: &SyntaxNode,
        name_node: &SyntaxNode,
        name: Name,
        table: TableId,
    ) {
        let id = self.index.policies.alloc(Policy {
            name,
            table,
            ptr: self.ptr(node),
            name_ptr: self.ptr(name_node),
        });
        self.record_comment(node, Item::Policy(id));
        self.add_item(name_node, Item::Policy(id));
    }

    fn report_duplicate_policy(&mut self, name_node: &SyntaxNode, name: Name, table: TableId) {
        let table = self.index.tables[table].name;
        self.push_error(name_node, SemaError::DuplicatePolicy { name, table });
    }

    fn lower_policy_ref(
        &mut self,
        table: Option<TableId>,
        policy_ref: &ast::PolicyRef,
        if_missing: IfMissing,
    ) -> Option<PolicyId> {
        let name = self.intern(&policy_ref.text());
        let policy = table.and_then(|table| self.find_policy(table, name));
        let resolution = match (policy, table, if_missing) {
            (Some(policy), _, _) => Resolution::Policy(policy),
            (None, Some(_), IfMissing::Error) => Resolution::Unresolved,
            (None, Some(_), IfMissing::Ignore) | (None, None, _) => Resolution::Unknown,
        };
        self.add_ref(name, ReferenceKind::Policy, policy_ref.syntax(), resolution);
        policy
    }

    fn find_policy(&self, table: TableId, name: Name) -> Option<PolicyId> {
        self.index
            .policies
            .get(table)
            .find(|id| self.index.policies[*id].name == name && !self.dropped_policies.contains(id))
    }

    fn lower_create_access_method(&mut self, create_access_method: &ast::CreateAccessMethod) {
        let refs = self.index.references.next_idx();
        let start = self.index.access_methods.next_idx();
        if let Some(path) = create_access_method
            .handler_clause()
            .and_then(|handler| handler.function_name_ref())
            .and_then(|function| function.path_ref())
        {
            let internal = self.builtin_type("internal");
            self.lower_function_path_ref(&path, Some(&[internal]));
        }
        let kind = match create_access_method.access_method_type() {
            Some(ast::AccessMethodType::AccessMethodTypeTable(_)) => AccessMethodKind::Table,
            Some(ast::AccessMethodType::AccessMethodTypeIndex(_)) => AccessMethodKind::Index,
            None => return,
        };
        let Some(access_method) = create_access_method.access_method() else {
            return;
        };
        let name_node = access_method.syntax();
        let name = self.intern(&access_method.text());
        if self.find_access_method(name).is_some() {
            self.report_duplicate(ObjectKind::AccessMethod, name_node, name);
        }
        let id = self.index.access_methods.alloc(AccessMethod {
            name,
            kind,
            ptr: self.ptr(create_access_method.syntax()),
            name_ptr: self.ptr(name_node),
        });
        self.record_comment(create_access_method.syntax(), Item::AccessMethod(id));
        self.add_item(name_node, Item::AccessMethod(id));
        self.depend_on_refs(
            refs,
            self.index.access_methods.range_from(start),
            Object::AccessMethod,
        );
    }

    fn lower_drop_access_method(&mut self, drop_access_method: &ast::DropAccessMethod) {
        let Some(access_method_ref) = drop_access_method.access_method_ref() else {
            return;
        };
        let if_missing = IfMissing::from_if_exists(drop_access_method.if_exists().as_ref());
        if let Some(access_method) =
            self.lower_access_method_ref(&access_method_ref, None, if_missing)
        {
            self.drop_access_method(access_method);
        }
    }

    fn lower_using_method(
        &mut self,
        using_method: &ast::UsingMethod,
        kind: AccessMethodKind,
    ) -> Option<AccessMethodId> {
        let access_method_ref = using_method.access_method_ref()?;
        self.lower_access_method_ref(&access_method_ref, Some(kind), IfMissing::Error)
    }

    fn lower_access_method_ref(
        &mut self,
        access_method_ref: &ast::AccessMethodRef,
        kind: Option<AccessMethodKind>,
        if_missing: IfMissing,
    ) -> Option<AccessMethodId> {
        let name = self.intern(&access_method_ref.text());
        let access_method = self.find_access_method(name);
        let resolution = match (access_method, if_missing) {
            (Some(access_method), _) => Resolution::AccessMethod(access_method),
            (None, IfMissing::Ignore) => Resolution::Unknown,
            (None, IfMissing::Error) => Resolution::Unresolved,
        };
        self.add_ref(
            name,
            ReferenceKind::AccessMethod,
            access_method_ref.syntax(),
            resolution,
        );
        if let (Some(access_method), Some(kind)) = (access_method, kind)
            && self.index.access_methods[access_method].kind != kind
        {
            self.push_error(
                access_method_ref.syntax(),
                SemaError::AccessMethodKindMismatch {
                    name,
                    expected: kind,
                },
            );
        }
        access_method
    }

    fn find_access_method(&self, name: Name) -> Option<AccessMethodId> {
        self.index
            .access_methods
            .get(name)
            .find(|id| !self.dropped_access_methods.contains(id))
    }

    fn lower_create_foreign_table(&mut self, create_table: &ast::CreateForeignTable) {
        if let Some(server_ref) = create_table
            .server_clause()
            .and_then(|clause| clause.server_ref())
        {
            self.lower_server_ref(&server_ref, IfMissing::Error);
        }
        self.lower_create_table(
            create_table,
            TableKind::ForeignTable,
            IfExists::from_if_not_exists(create_table.if_not_exists().as_ref()),
            create_table.partition_of(),
            create_table.partition_type(),
            None,
            None,
            None,
            None,
        );
    }

    fn lower_import_foreign_schema(&mut self, import: &ast::ImportForeignSchema) {
        if let Some(server_ref) = import
            .server_clause()
            .and_then(|clause| clause.server_ref())
        {
            self.lower_server_ref(&server_ref, IfMissing::Error);
        }
        if let Some(schema) = import
            .into_schema()
            .and_then(|into_schema| into_schema.schema_ref())
        {
            self.lower_schema_ref(schema.syntax(), &schema.text());
        }
    }

    fn lower_create_foreign_data_wrapper(
        &mut self,
        create_wrapper: &ast::CreateForeignDataWrapper,
    ) {
        let refs = self.index.references.next_idx();
        let start = self.index.foreign_data_wrappers.next_idx();
        if let Some(options) = create_wrapper.fdw_option_list() {
            self.lower_fdw_options(&options);
        }
        if let Some(wrapper) = create_wrapper.foreign_data_wrapper() {
            self.alloc_foreign_data_wrapper(create_wrapper.syntax(), &wrapper);
        }
        self.depend_on_refs(
            refs,
            self.index.foreign_data_wrappers.range_from(start),
            Object::ForeignDataWrapper,
        );
    }

    fn lower_alter_foreign_data_wrapper(&mut self, alter_wrapper: &ast::AlterForeignDataWrapper) {
        let wrapper = alter_wrapper
            .foreign_data_wrapper_ref()
            .and_then(|wrapper_ref| {
                self.lower_foreign_data_wrapper_ref(&wrapper_ref, IfMissing::Error)
            });
        match alter_wrapper.action() {
            Some(ast::AlterForeignDataWrapperAction::FdwOptionList(options)) => {
                self.lower_fdw_options(&options);
            }
            Some(ast::AlterForeignDataWrapperAction::ForeignDataWrapperRenameTo(rename)) => {
                if let Some(wrapper) = wrapper
                    && let Some(new_wrapper) = rename.foreign_data_wrapper()
                {
                    let new_name = self.intern(&new_wrapper.text());
                    if self.find_foreign_data_wrapper(new_name).is_some() {
                        self.report_duplicate(
                            ObjectKind::ForeignDataWrapper,
                            new_wrapper.syntax(),
                            new_name,
                        );
                        return;
                    }
                    let moved = self.index.foreign_data_wrappers.next_idx();
                    self.drop_foreign_data_wrapper(wrapper);
                    self.alloc_foreign_data_wrapper(alter_wrapper.syntax(), &new_wrapper);
                    self.record_moves(
                        Object::ForeignDataWrapper(wrapper),
                        self.index.foreign_data_wrappers.range_from(moved),
                        Object::ForeignDataWrapper,
                    );
                }
            }
            Some(ast::AlterForeignDataWrapperAction::OwnerTo(owner_to)) => {
                self.lower_owner_to(&owner_to);
            }
            None => {}
        }
    }

    fn lower_drop_foreign_data_wrapper(&mut self, drop_wrapper: &ast::DropForeignDataWrapper) {
        let if_missing = IfMissing::from_if_exists(drop_wrapper.if_exists().as_ref());
        for wrapper_ref in drop_wrapper.foreign_data_wrapper_refs() {
            if let Some(wrapper) = self.lower_foreign_data_wrapper_ref(&wrapper_ref, if_missing) {
                self.drop_foreign_data_wrapper(wrapper);
            }
        }
    }

    fn lower_fdw_options(&mut self, options: &ast::FdwOptionList) {
        for option in options.fdw_options() {
            match option {
                ast::FdwOption::OptionHandler(handler) => {
                    if let Some(path) = handler.function_name_ref().and_then(|f| f.path_ref()) {
                        self.lower_function_path_ref(&path, Some(&[]));
                    }
                }
                ast::FdwOption::OptionValidator(validator) => {
                    if let Some(path) = validator.function_name_ref().and_then(|f| f.path_ref()) {
                        let text_array = self.builtin_type("text").map(Ty::array);
                        let oid = self.builtin_type("oid");
                        self.lower_function_path_ref(&path, Some(&[text_array, oid]));
                    }
                }
                ast::FdwOption::OptionConnection(connection) => {
                    if let Some(path) = connection.function_name_ref().and_then(|f| f.path_ref()) {
                        self.lower_function_path_ref(&path, None);
                    }
                }
                ast::FdwOption::OptionAlterOptionList(_)
                | ast::FdwOption::OptionNoConnection(_)
                | ast::FdwOption::OptionNoHandler(_)
                | ast::FdwOption::OptionNoValidator(_) => {}
            }
        }
    }

    fn alloc_foreign_data_wrapper(&mut self, node: &SyntaxNode, name_node: &impl NameLike) {
        let name = self.intern(&name_node.text());
        if self.find_foreign_data_wrapper(name).is_some() {
            self.report_duplicate(ObjectKind::ForeignDataWrapper, name_node.syntax(), name);
        }
        let id = self.index.foreign_data_wrappers.alloc(ForeignDataWrapper {
            name,
            ptr: self.ptr(node),
            name_ptr: self.ptr(name_node.syntax()),
        });
        self.record_comment(node, Item::ForeignDataWrapper(id));
        self.add_item(name_node.syntax(), Item::ForeignDataWrapper(id));
    }

    fn lower_foreign_data_wrapper_ref(
        &mut self,
        wrapper_ref: &ast::ForeignDataWrapperRef,
        if_missing: IfMissing,
    ) -> Option<ForeignDataWrapperId> {
        let name = self.intern(&wrapper_ref.text());
        let wrapper = self.find_foreign_data_wrapper(name);
        let resolution = match (wrapper, if_missing) {
            (Some(wrapper), _) => Resolution::ForeignDataWrapper(wrapper),
            (None, IfMissing::Ignore) => Resolution::Unknown,
            (None, IfMissing::Error) => Resolution::Unresolved,
        };
        self.add_ref(
            name,
            ReferenceKind::ForeignDataWrapper,
            wrapper_ref.syntax(),
            resolution,
        );
        wrapper
    }

    fn find_foreign_data_wrapper(&self, name: Name) -> Option<ForeignDataWrapperId> {
        self.index
            .foreign_data_wrappers
            .get(name)
            .find(|id| !self.dropped_foreign_data_wrappers.contains(id))
    }

    fn lower_create_server(&mut self, create_server: &ast::CreateServer) {
        if let Some(wrapper_ref) = create_server.foreign_data_wrapper_ref() {
            self.lower_foreign_data_wrapper_ref(&wrapper_ref, IfMissing::Error);
        }
        let Some(server) = create_server.server() else {
            return;
        };
        let name_node = server.syntax();
        let name = self.intern(&server.text());
        if self.find_server(name).is_some() {
            if create_server.if_not_exists().is_some() {
                return;
            }
            self.report_duplicate(ObjectKind::Server, name_node, name);
        }
        self.alloc_server(create_server.syntax(), name_node, name);
    }

    fn lower_alter_server(&mut self, alter_server: &ast::AlterServer) {
        let server = alter_server
            .server_ref()
            .and_then(|server_ref| self.lower_server_ref(&server_ref, IfMissing::Error));
        for action in alter_server.actions() {
            match action {
                ast::AlterServerAction::ServerRenameTo(rename) => {
                    if let Some(server) = server
                        && let Some(new_server) = rename.server()
                    {
                        let name_node = new_server.syntax();
                        let new_name = self.intern(&new_server.text());
                        if self.find_server(new_name).is_some() {
                            self.report_duplicate(ObjectKind::Server, name_node, new_name);
                            continue;
                        }
                        self.drop_server(server);
                        let id = self.alloc_server(alter_server.syntax(), name_node, new_name);
                        self.copy_comment(Item::Server(server), Item::Server(id));
                    }
                }
                ast::AlterServerAction::OwnerTo(owner_to) => self.lower_owner_to(&owner_to),
                ast::AlterServerAction::AlterOptionList(_)
                | ast::AlterServerAction::ServerVersion(_) => {}
            }
        }
    }

    fn lower_drop_server(&mut self, drop_server: &ast::DropServer) {
        let if_missing = IfMissing::from_if_exists(drop_server.if_exists().as_ref());
        for server_ref in drop_server.server_refs() {
            if let Some(server) = self.lower_server_ref(&server_ref, if_missing) {
                self.drop_server(server);
            }
        }
    }

    fn alloc_server(&mut self, node: &SyntaxNode, name_node: &SyntaxNode, name: Name) -> ServerId {
        let id = self.index.servers.alloc(Server {
            name,
            ptr: self.ptr(node),
            name_ptr: self.ptr(name_node),
        });
        self.record_comment(node, Item::Server(id));
        self.add_item(name_node, Item::Server(id));
        id
    }

    fn lower_server_ref(
        &mut self,
        server_ref: &ast::ServerRef,
        if_missing: IfMissing,
    ) -> Option<ServerId> {
        let name = self.intern(&server_ref.text());
        let server = self.find_server(name);
        let resolution = match (server, if_missing) {
            (Some(server), _) => Resolution::Server(server),
            (None, IfMissing::Ignore) => Resolution::Unknown,
            (None, IfMissing::Error) => Resolution::Unresolved,
        };
        self.add_ref(name, ReferenceKind::Server, server_ref.syntax(), resolution);
        server
    }

    fn find_server(&self, name: Name) -> Option<ServerId> {
        self.index
            .servers
            .get(name)
            .find(|id| !self.dropped_servers.contains(id))
    }

    fn lower_create_user_mapping(&mut self, create_user_mapping: &ast::CreateUserMapping) {
        if let Some(role_ref) = create_user_mapping
            .user_mapping_role()
            .and_then(|role| role.role_ref())
        {
            self.lower_grantee_role_ref(&role_ref);
        }
        if let Some(server_ref) = create_user_mapping
            .server_clause()
            .and_then(|clause| clause.server_ref())
        {
            self.lower_server_ref(&server_ref, IfMissing::Error);
        }
    }

    fn lower_alter_user_mapping(&mut self, alter_user_mapping: &ast::AlterUserMapping) {
        if let Some(role_ref) = alter_user_mapping.role_ref() {
            self.lower_grantee_role_ref(&role_ref);
        }
        if let Some(server_ref) = alter_user_mapping
            .server_clause()
            .and_then(|clause| clause.server_ref())
        {
            self.lower_server_ref(&server_ref, IfMissing::Error);
        }
    }

    fn lower_drop_user_mapping(&mut self, drop_user_mapping: &ast::DropUserMapping) {
        let if_missing = IfMissing::from_if_exists(drop_user_mapping.if_exists().as_ref());
        if let Some(role_ref) = drop_user_mapping
            .user_mapping_role()
            .and_then(|role| role.role_ref())
        {
            self.lower_grantee_role_ref(&role_ref);
        }
        if let Some(server_ref) = drop_user_mapping
            .server_clause()
            .and_then(|clause| clause.server_ref())
        {
            self.lower_server_ref(&server_ref, if_missing);
        }
    }

    fn lower_create_operator_family(&mut self, create_family: &ast::CreateOperatorFamily) {
        let access_method = create_family.using_method().and_then(|using_method| {
            self.lower_using_method(&using_method, AccessMethodKind::Index)
        });
        let Some(path) = create_family.op_family_name().and_then(|name| name.path()) else {
            return;
        };
        let Some(segment) = path.segment() else {
            return;
        };
        let schema = self.creation_schema(&path);
        let name = self.intern(&segment.text());
        if self.has_operator_family(schema, name, access_method) {
            self.report_duplicate_for_access_method(
                ObjectKind::OperatorFamily,
                segment.syntax(),
                name,
                access_method,
            );
        }
        let id = self.alloc_operator_family(
            create_family.syntax(),
            segment.syntax(),
            schema,
            name,
            access_method,
        );
        self.add_item(segment.syntax(), Item::OperatorFamily(id));
    }

    fn lower_alter_operator_family(&mut self, alter_family: &ast::AlterOperatorFamily) {
        let access_method = alter_family.using_method().and_then(|using_method| {
            self.lower_using_method(&using_method, AccessMethodKind::Index)
        });
        let family = alter_family
            .op_family_ref()
            .and_then(|family| family.path_ref())
            .and_then(|path| {
                self.lower_operator_family_ref(&path, access_method, IfMissing::Error)
            });
        match alter_family.action() {
            Some(ast::AlterOperatorFamilyAction::AddOpClassOptions(add)) => {
                if let Some(options) = add.operator_class_option_list() {
                    self.lower_operator_class_options(
                        &options,
                        None,
                        family.map(MemberOwner::Family),
                    );
                }
            }
            Some(ast::AlterOperatorFamilyAction::DropOpClassOptions(drop)) => {
                for option in drop
                    .drop_op_class_option_list()
                    .into_iter()
                    .flat_map(|list| list.drop_op_class_options())
                {
                    let (kind, literal, param_list) = match option {
                        ast::DropOpClassOption::DropOpClassOptionFunction(function) => (
                            MemberKind::Function,
                            function.literal(),
                            function.param_list(),
                        ),
                        ast::DropOpClassOption::DropOpClassOptionOperator(operator) => (
                            MemberKind::Operator,
                            operator.literal(),
                            operator.param_list(),
                        ),
                    };
                    let operands = self.lower_member_operands(param_list.as_ref());
                    if let Some(family) = family {
                        self.drop_operator_family_members(
                            MemberOwner::Family(family),
                            kind,
                            member_number(literal.as_ref()),
                            operands,
                        );
                    }
                }
            }
            Some(ast::AlterOperatorFamilyAction::OpFamilyRenameTo(rename)) => {
                if let Some(family) = family
                    && let Some(segment) = rename
                        .op_family_name()
                        .and_then(|name| name.path())
                        .and_then(|path| path.segment())
                {
                    let OperatorFamily {
                        schema,
                        access_method,
                        ..
                    } = self.index.operator_families[family];
                    let name = self.intern(&segment.text());
                    if self.has_operator_family(schema, name, access_method) {
                        self.report_duplicate_for_access_method(
                            ObjectKind::OperatorFamily,
                            segment.syntax(),
                            name,
                            access_method,
                        );
                        return;
                    }
                    self.drop_operator_family(family);
                    let id = self.alloc_operator_family(
                        alter_family.syntax(),
                        segment.syntax(),
                        schema,
                        name,
                        access_method,
                    );
                    self.copy_comment(Item::OperatorFamily(family), Item::OperatorFamily(id));
                    self.move_operator_family_members(
                        MemberOwner::Family(family),
                        MemberOwner::Family(id),
                    );
                    self.add_item(segment.syntax(), Item::OperatorFamily(id));
                }
            }
            Some(ast::AlterOperatorFamilyAction::SetSchema(set_schema)) => {
                let Some(schema_ref) = set_schema.schema_ref() else {
                    return;
                };
                let Some(schema) = self.lower_set_schema_ref(&schema_ref) else {
                    return;
                };
                if let Some(family) = family {
                    let OperatorFamily {
                        name,
                        access_method,
                        name_ptr,
                        ..
                    } = self.index.operator_families[family];
                    if self.has_operator_family(schema, name, access_method) {
                        self.report_duplicate_for_access_method(
                            ObjectKind::OperatorFamily,
                            schema_ref.syntax(),
                            name,
                            access_method,
                        );
                        return;
                    }
                    self.drop_operator_family(family);
                    let id = self.index.operator_families.alloc(OperatorFamily {
                        schema,
                        name,
                        access_method,
                        ptr: self.ptr(alter_family.syntax()),
                        name_ptr,
                    });
                    self.copy_comment(Item::OperatorFamily(family), Item::OperatorFamily(id));
                    self.move_operator_family_members(
                        MemberOwner::Family(family),
                        MemberOwner::Family(id),
                    );
                }
            }
            Some(ast::AlterOperatorFamilyAction::OwnerTo(owner_to)) => {
                self.lower_owner_to(&owner_to);
            }
            None => {}
        }
    }

    fn lower_drop_operator_family(&mut self, drop_family: &ast::DropOperatorFamily) {
        let if_missing = IfMissing::from_if_exists(drop_family.if_exists().as_ref());
        let access_method = drop_family.using_method().and_then(|using_method| {
            self.lower_using_method(&using_method, AccessMethodKind::Index)
        });
        if let Some(family) = drop_family
            .op_family_ref()
            .and_then(|family| family.path_ref())
            .and_then(|path| self.lower_operator_family_ref(&path, access_method, if_missing))
        {
            self.drop_operator_family(family);
        }
    }

    fn lower_create_operator_class(&mut self, create_class: &ast::CreateOperatorClass) {
        let ty = create_class
            .ty()
            .and_then(|ty| self.lower_type_ref(&ty, IfMissing::Error));
        let access_method = create_class.using_method().and_then(|using_method| {
            self.lower_using_method(&using_method, AccessMethodKind::Index)
        });
        let family_ref = create_class
            .operator_family_clause()
            .and_then(|clause| clause.op_family_ref())
            .and_then(|family| family.path_ref());
        if let Some(path) = &family_ref {
            self.lower_operator_family_ref(path, access_method, IfMissing::Error);
        }
        let class =
            self.alloc_create_operator_class(create_class, access_method, family_ref.is_some());
        if let Some(options) = create_class.operator_class_option_list() {
            self.lower_operator_class_options(&options, ty, class.map(MemberOwner::Class));
        }
    }

    fn alloc_create_operator_class(
        &mut self,
        create_class: &ast::CreateOperatorClass,
        access_method: Option<AccessMethodId>,
        has_family: bool,
    ) -> Option<OperatorClassId> {
        let path = create_class.op_class_name()?.path()?;
        let segment = path.segment()?;
        let schema = self.creation_schema(&path);
        let name = self.intern(&segment.text());
        if !has_family
            && self
                .find_operator_family_in(schema, name, access_method)
                .is_none()
        {
            self.alloc_operator_family(
                create_class.syntax(),
                segment.syntax(),
                schema,
                name,
                access_method,
            );
        }
        if self.has_operator_class(schema, name, access_method) {
            self.report_duplicate_for_access_method(
                ObjectKind::OperatorClass,
                segment.syntax(),
                name,
                access_method,
            );
        }
        Some(self.alloc_operator_class(
            create_class.syntax(),
            segment.syntax(),
            schema,
            name,
            access_method,
        ))
    }

    fn lower_operator_class_options(
        &mut self,
        options: &ast::OperatorClassOptionList,
        ty: Option<Ty>,
        owner: Option<MemberOwner>,
    ) {
        let default_operand = ty.map_or(Operand::Unknown, Operand::Ty);
        for option in options.op_class_options() {
            let ptr = self.ptr(option.syntax());
            let refs = self.index.references.next_idx();
            let (kind, literal, operands) = match option {
                ast::OpClassOption::OpClassOptionOperator(operator) => {
                    let (left, right) = match operator.lhs() {
                        Some(lhs) => (
                            self.lower_operand(Some(lhs), IfMissing::Error),
                            self.lower_operand(operator.rhs(), IfMissing::Error),
                        ),
                        None => (default_operand, default_operand),
                    };
                    if let Some(op) = operator.op() {
                        self.lower_operator_ref(&op, left, right, IfMissing::Error);
                    }
                    match operator.op_class_purpose() {
                        Some(ast::OpClassPurpose::OpClassForOrderBy(order_by)) => {
                            if let Some(path) = order_by
                                .op_family_ref()
                                .and_then(|family| family.path_ref())
                            {
                                self.lower_operator_family_ref(&path, None, IfMissing::Error);
                            }
                        }
                        Some(ast::OpClassPurpose::OpClassForSearch(_)) | None => {}
                    }
                    (
                        MemberKind::Operator,
                        operator.literal(),
                        Operands { left, right },
                    )
                }
                ast::OpClassOption::OpClassOptionFunction(function) => {
                    let operands = self.lower_member_operands(function.param_list().as_ref());
                    if let Some(function_sig) = function.function_sig() {
                        self.lower_function_sig(&function_sig);
                    }
                    (MemberKind::Function, function.literal(), operands)
                }
                ast::OpClassOption::OpClassOptionStorage(storage) => {
                    if let Some(ty) = storage.ty() {
                        self.lower_type_ref(&ty, IfMissing::Error);
                    }
                    continue;
                }
            };
            let Some(owner) = owner else {
                continue;
            };
            let member = self
                .index
                .operator_family_members
                .alloc(OperatorFamilyMember {
                    owner,
                    kind,
                    number: member_number(literal.as_ref()),
                    operands,
                    ptr,
                });
            for reference in self.index.references.range_from(refs) {
                let referenced = match self.index.references[reference].resolution {
                    Resolution::Function(function) => Object::Function(function),
                    Resolution::Operator(operator) => Object::Operator(operator),
                    Resolution::Column { .. }
                    | Resolution::Source(_)
                    | Resolution::Cte(_)
                    | Resolution::Table(_)
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
                    | Resolution::Savepoint(_)
                    | Resolution::Cursor(_)
                    | Resolution::PreparedTransaction(_)
                    | Resolution::PreparedStatement(_)
                    | Resolution::Channel(_)
                    | Resolution::Unknown
                    | Resolution::Unresolved
                    | Resolution::Ambiguous => continue,
                };
                self.dependencies
                    .add_dependency(Object::OperatorFamilyMember(member), referenced);
            }
        }
    }

    fn lower_member_operands(&mut self, param_list: Option<&ast::ParamList>) -> Operands {
        let mut operands = Operands {
            left: Operand::Unknown,
            right: Operand::Unknown,
        };
        for (i, param) in param_list
            .into_iter()
            .flat_map(|list| list.params())
            .enumerate()
        {
            let operand = self
                .lower_func_type(param.func_type())
                .map_or(Operand::Unknown, Operand::Ty);
            match i {
                0 => {
                    operands = Operands {
                        left: operand,
                        right: operand,
                    }
                }
                1 => operands.right = operand,
                _ => {}
            }
        }
        operands
    }

    fn drop_operator_family_members(
        &mut self,
        owner: MemberOwner,
        kind: MemberKind,
        number: Option<u16>,
        operands: Operands,
    ) {
        let matches =
            |a: Operand, b: Operand| a == Operand::Unknown || b == Operand::Unknown || a == b;
        let in_transaction = self.in_transaction();
        for id in self.index.operator_family_members.get(owner) {
            let member = &self.index.operator_family_members[id];
            if member.kind == kind
                && member.number == number
                && matches(member.operands.left, operands.left)
                && matches(member.operands.right, operands.right)
                && self.dropped_operator_family_members.insert(id)
                && in_transaction
            {
                self.undo.push(Undo::DropOperatorFamilyMember(id));
            }
        }
    }

    fn lower_alter_operator_class(&mut self, alter_class: &ast::AlterOperatorClass) {
        let access_method = alter_class.using_method().and_then(|using_method| {
            self.lower_using_method(&using_method, AccessMethodKind::Index)
        });
        let class = alter_class
            .op_class_ref()
            .and_then(|class| class.path_ref())
            .and_then(|path| self.lower_operator_class_ref(&path, access_method, IfMissing::Error));
        match alter_class.action() {
            Some(ast::AlterOperatorClassAction::OpClassRenameTo(rename)) => {
                if let Some(class) = class
                    && let Some(segment) = rename
                        .op_class_name()
                        .and_then(|name| name.path())
                        .and_then(|path| path.segment())
                {
                    let OperatorClass {
                        schema,
                        access_method,
                        ..
                    } = self.index.operator_classes[class];
                    let name = self.intern(&segment.text());
                    if self.has_operator_class(schema, name, access_method) {
                        self.report_duplicate_for_access_method(
                            ObjectKind::OperatorClass,
                            segment.syntax(),
                            name,
                            access_method,
                        );
                        return;
                    }
                    self.drop_operator_class(class);
                    let id = self.alloc_operator_class(
                        alter_class.syntax(),
                        segment.syntax(),
                        schema,
                        name,
                        access_method,
                    );
                    self.copy_comment(Item::OperatorClass(class), Item::OperatorClass(id));
                    self.move_operator_family_members(
                        MemberOwner::Class(class),
                        MemberOwner::Class(id),
                    );
                }
            }
            Some(ast::AlterOperatorClassAction::SetSchema(set_schema)) => {
                let Some(schema_ref) = set_schema.schema_ref() else {
                    return;
                };
                let Some(schema) = self.lower_set_schema_ref(&schema_ref) else {
                    return;
                };
                if let Some(class) = class {
                    let OperatorClass {
                        name,
                        access_method,
                        name_ptr,
                        ..
                    } = self.index.operator_classes[class];
                    if self.has_operator_class(schema, name, access_method) {
                        self.report_duplicate_for_access_method(
                            ObjectKind::OperatorClass,
                            schema_ref.syntax(),
                            name,
                            access_method,
                        );
                        return;
                    }
                    self.drop_operator_class(class);
                    let id = self.index.operator_classes.alloc(OperatorClass {
                        schema,
                        name,
                        access_method,
                        ptr: self.ptr(alter_class.syntax()),
                        name_ptr,
                    });
                    self.copy_comment(Item::OperatorClass(class), Item::OperatorClass(id));
                    self.move_operator_family_members(
                        MemberOwner::Class(class),
                        MemberOwner::Class(id),
                    );
                }
            }
            Some(ast::AlterOperatorClassAction::OwnerTo(owner_to)) => {
                self.lower_owner_to(&owner_to);
            }
            None => {}
        }
    }

    fn lower_drop_operator_class(&mut self, drop_class: &ast::DropOperatorClass) {
        let if_missing = IfMissing::from_if_exists(drop_class.if_exists().as_ref());
        let access_method = drop_class.using_method().and_then(|using_method| {
            self.lower_using_method(&using_method, AccessMethodKind::Index)
        });
        if let Some(class) = drop_class
            .op_class_ref()
            .and_then(|class| class.path_ref())
            .and_then(|path| self.lower_operator_class_ref(&path, access_method, if_missing))
        {
            self.drop_operator_class(class);
        }
    }

    fn alloc_operator_family(
        &mut self,
        node: &SyntaxNode,
        name_node: &SyntaxNode,
        schema: Name,
        name: Name,
        access_method: Option<AccessMethodId>,
    ) -> OperatorFamilyId {
        let id = self.index.operator_families.alloc(OperatorFamily {
            schema,
            name,
            access_method,
            ptr: self.ptr(node),
            name_ptr: self.ptr(name_node),
        });
        self.record_comment(node, Item::OperatorFamily(id));
        id
    }

    fn alloc_operator_class(
        &mut self,
        node: &SyntaxNode,
        name_node: &SyntaxNode,
        schema: Name,
        name: Name,
        access_method: Option<AccessMethodId>,
    ) -> OperatorClassId {
        let id = self.index.operator_classes.alloc(OperatorClass {
            schema,
            name,
            access_method,
            ptr: self.ptr(node),
            name_ptr: self.ptr(name_node),
        });
        self.record_comment(node, Item::OperatorClass(id));
        self.add_item(name_node, Item::OperatorClass(id));
        id
    }

    fn report_duplicate_for_access_method(
        &mut self,
        object: ObjectKind,
        node: &SyntaxNode,
        name: Name,
        access_method: Option<AccessMethodId>,
    ) {
        let access_method = access_method.map(|id| self.index.access_methods[id].name);
        let error = SemaError::DuplicateForAccessMethod {
            object,
            name,
            access_method,
        };
        self.push_error(node, error);
    }

    fn lower_operator_family_ref(
        &mut self,
        path: &ast::PathRef,
        access_method: Option<AccessMethodId>,
        if_missing: IfMissing,
    ) -> Option<OperatorFamilyId> {
        let segment = path.segment()?;
        let name = self.intern(&segment.text());
        let schema = self.lower_schema_qualifier(path.qualifier());
        let family = self
            .search_path(schema, Lookup::OperatorClass)
            .find_map(|schema| self.find_operator_family_in(schema, name, access_method));
        let resolution = match (family, if_missing) {
            (Some(family), _) => Resolution::OperatorFamily(family),
            (None, IfMissing::Ignore) => Resolution::Unknown,
            (None, IfMissing::Error) => Resolution::Unresolved,
        };
        self.add_ref(
            name,
            ReferenceKind::OperatorFamily,
            segment.syntax(),
            resolution,
        );
        family
    }

    fn lower_operator_class_ref(
        &mut self,
        path: &ast::PathRef,
        access_method: Option<AccessMethodId>,
        if_missing: IfMissing,
    ) -> Option<OperatorClassId> {
        let segment = path.segment()?;
        let name = self.intern(&segment.text());
        let schema = self.lower_schema_qualifier(path.qualifier());
        let class = self
            .search_path(schema, Lookup::OperatorClass)
            .find_map(|schema| self.find_operator_class_in(schema, name, access_method));
        let resolution = match (class, if_missing) {
            (Some(class), _) => Resolution::OperatorClass(class),
            (None, IfMissing::Ignore) => Resolution::Unknown,
            (None, IfMissing::Error) => Resolution::Unresolved,
        };
        self.add_ref(
            name,
            ReferenceKind::OperatorClass,
            segment.syntax(),
            resolution,
        );
        class
    }

    fn has_operator_family(
        &self,
        schema: Name,
        name: Name,
        access_method: Option<AccessMethodId>,
    ) -> bool {
        access_method.is_some()
            && self.index.operator_families.named(schema, name).any(|id| {
                let family = &self.index.operator_families[id];
                family.access_method == access_method
                    && self.visible(family.schema, family.ptr.file_id)
                    && !self.dropped_operator_families.contains(&id)
            })
    }

    fn has_operator_class(
        &self,
        schema: Name,
        name: Name,
        access_method: Option<AccessMethodId>,
    ) -> bool {
        access_method.is_some()
            && self.index.operator_classes.named(schema, name).any(|id| {
                let class = &self.index.operator_classes[id];
                class.access_method == access_method
                    && self.visible(class.schema, class.ptr.file_id)
                    && !self.dropped_operator_classes.contains(&id)
            })
    }

    fn find_operator_family_in(
        &self,
        schema: Name,
        name: Name,
        access_method: Option<AccessMethodId>,
    ) -> Option<OperatorFamilyId> {
        self.index.operator_families.named(schema, name).find(|id| {
            let family = &self.index.operator_families[*id];
            same_access_method(family.access_method, access_method)
                && self.visible(family.schema, family.ptr.file_id)
                && !self.dropped_operator_families.contains(id)
        })
    }

    fn find_operator_class_in(
        &self,
        schema: Name,
        name: Name,
        access_method: Option<AccessMethodId>,
    ) -> Option<OperatorClassId> {
        self.index.operator_classes.named(schema, name).find(|id| {
            let class = &self.index.operator_classes[*id];
            same_access_method(class.access_method, access_method)
                && self.visible(class.schema, class.ptr.file_id)
                && !self.dropped_operator_classes.contains(id)
        })
    }

    fn lower_create_statistics(&mut self, create_statistics: &ast::CreateStatistics) {
        let table = create_statistics
            .from_table()
            .and_then(|from| from.table_name_ref())
            .and_then(|name| name.path_ref())
            .and_then(|path| {
                self.lower_table_ref(&path, IfMissing::Error, TableLookup::TableOrForeignTable)
            });
        let on_clause = create_statistics.statistics_on_clause();
        if let Some(on_clause) = &on_clause {
            self.with_table_scope(
                QueryKind::Statistics,
                table,
                create_statistics.syntax(),
                |this| {
                    for expr in on_clause.exprs() {
                        this.lower_expr_in(ExprContext::StatisticsExpression, &expr);
                    }
                },
            );
        }
        let if_not_exists = create_statistics.if_not_exists().is_some();
        match create_statistics
            .statistics()
            .and_then(|statistics| statistics.path())
        {
            Some(path) => {
                let Some(segment) = path.segment() else {
                    return;
                };
                let schema = self.creation_schema(&path);
                let name = self.intern(&segment.text());
                if self.find_statistics_in(schema, name).is_some() {
                    if if_not_exists {
                        return;
                    }
                    self.report_duplicate(ObjectKind::Statistics, segment.syntax(), name);
                }
                let id = self.alloc_statistics(
                    create_statistics.syntax(),
                    segment.syntax(),
                    schema,
                    name,
                    table,
                );
                self.record_comment(create_statistics.syntax(), Item::Statistics(id));
                self.add_item(segment.syntax(), Item::Statistics(id));
            }
            None => {
                let Some(table) = table else {
                    return;
                };
                let Table {
                    schema,
                    name: table_name,
                    ..
                } = self.index.tables[table];
                let mut addition = String::new();
                for expr in on_clause.iter().flat_map(|on_clause| on_clause.exprs()) {
                    if !addition.is_empty() {
                        addition.push('_');
                    }
                    if let ast::Expr::NameRef(name_ref) = expr {
                        addition.push_str(&name_ref.text());
                    } else {
                        addition.push_str("expr");
                    }
                }
                let name = postgres::choose_extended_statistic_name(
                    self.index.names.resolve(table_name),
                    &addition,
                    |name| {
                        self.index
                            .names
                            .lookup(name)
                            .is_some_and(|name| self.find_statistics_in(schema, name).is_some())
                    },
                );
                let name = self.intern(&name);
                let name_node = on_clause.as_ref().map_or_else(
                    || create_statistics.syntax().clone(),
                    |on_clause| on_clause.syntax().clone(),
                );
                let id = self.alloc_statistics(
                    create_statistics.syntax(),
                    &name_node,
                    schema,
                    name,
                    Some(table),
                );
                self.record_comment(create_statistics.syntax(), Item::Statistics(id));
            }
        }
    }

    fn lower_alter_statistics(&mut self, alter_statistics: &ast::AlterStatistics) {
        let statistics = alter_statistics
            .statistics_ref()
            .and_then(|statistics| statistics.path_ref())
            .and_then(|path| self.lower_statistics_ref(&path, IfMissing::Error));
        match alter_statistics.action() {
            Some(ast::AlterStatisticsAction::StatisticsRenameTo(rename)) => {
                if let Some(statistics) = statistics
                    && let Some(segment) = rename
                        .statistics()
                        .and_then(|statistics| statistics.path())
                        .and_then(|path| path.segment())
                {
                    let Statistics { schema, table, .. } = self.index.statistics[statistics];
                    let name = self.intern(&segment.text());
                    if self.find_statistics_in(schema, name).is_some() {
                        self.report_duplicate(ObjectKind::Statistics, segment.syntax(), name);
                        return;
                    }
                    self.drop_statistics(statistics);
                    let id = self.alloc_statistics(
                        alter_statistics.syntax(),
                        segment.syntax(),
                        schema,
                        name,
                        table,
                    );
                    self.copy_comment(Item::Statistics(statistics), Item::Statistics(id));
                    self.add_item(segment.syntax(), Item::Statistics(id));
                }
            }
            Some(ast::AlterStatisticsAction::SetSchema(set_schema)) => {
                let Some(schema_ref) = set_schema.schema_ref() else {
                    return;
                };
                let Some(schema) = self.lower_set_schema_ref(&schema_ref) else {
                    return;
                };
                if let Some(statistics) = statistics {
                    let Statistics {
                        name,
                        table,
                        name_ptr,
                        ..
                    } = self.index.statistics[statistics];
                    if self.find_statistics_in(schema, name).is_some() {
                        let error = SemaError::DuplicateInSchema {
                            object: ObjectKind::Statistics,
                            name,
                            schema,
                        };
                        self.push_error(schema_ref.syntax(), error);
                        return;
                    }
                    self.drop_statistics(statistics);
                    let id = self.index.statistics.alloc(Statistics {
                        schema,
                        name,
                        table,
                        ptr: self.ptr(alter_statistics.syntax()),
                        name_ptr,
                    });
                    self.copy_comment(Item::Statistics(statistics), Item::Statistics(id));
                }
            }
            Some(ast::AlterStatisticsAction::SetStatistics(set_statistics)) => {
                if let Some(expr) = set_statistics.expr() {
                    self.lower_expr(&expr);
                }
            }
            Some(ast::AlterStatisticsAction::OwnerTo(owner_to)) => self.lower_owner_to(&owner_to),
            None => {}
        }
    }

    fn lower_drop_statistics(&mut self, drop_statistics: &ast::DropStatistics) {
        let if_missing = IfMissing::from_if_exists(drop_statistics.if_exists().as_ref());
        for path in drop_statistics
            .statistics_refs()
            .filter_map(|statistics| statistics.path_ref())
        {
            if let Some(statistics) = self.lower_statistics_ref(&path, if_missing) {
                self.drop_statistics(statistics);
            }
        }
    }

    fn alloc_statistics(
        &mut self,
        node: &SyntaxNode,
        name_node: &SyntaxNode,
        schema: Name,
        name: Name,
        table: Option<TableId>,
    ) -> StatisticsId {
        self.index.statistics.alloc(Statistics {
            schema,
            name,
            table,
            ptr: self.ptr(node),
            name_ptr: self.ptr(name_node),
        })
    }

    fn lower_statistics_ref(
        &mut self,
        path: &ast::PathRef,
        if_missing: IfMissing,
    ) -> Option<StatisticsId> {
        let segment = path.segment()?;
        let name = self.intern(&segment.text());
        let schema = self.lower_schema_qualifier(path.qualifier());
        let statistics = self
            .search_path(schema, Lookup::Statistics)
            .find_map(|schema| self.find_statistics_in(schema, name));
        let resolution = match (statistics, if_missing) {
            (Some(statistics), _) => Resolution::Statistics(statistics),
            (None, IfMissing::Ignore) => Resolution::Unknown,
            (None, IfMissing::Error) => Resolution::Unresolved,
        };
        self.add_ref(
            name,
            ReferenceKind::Statistics,
            segment.syntax(),
            resolution,
        );
        statistics
    }

    fn find_statistics_in(&self, schema: Name, name: Name) -> Option<StatisticsId> {
        self.index.statistics.named(schema, name).find(|id| {
            let statistics = &self.index.statistics[*id];
            self.visible(statistics.schema, statistics.ptr.file_id)
                && !self.dropped_statistics.contains(id)
                && statistics
                    .table
                    .is_none_or(|table| !self.dropped_tables.contains(&table))
        })
    }

    fn lower_create_text_search_parser(&mut self, create_parser: &ast::CreateTextSearchParser) {
        let refs = self.index.references.next_idx();
        let start = self.index.text_search.next_idx();
        for option in create_parser
            .attribute_list()
            .iter()
            .flat_map(|list| list.attribute_options())
        {
            if let Some(path) = attribute_path(&option) {
                self.lower_function_path_ref(&path, None);
            }
        }
        if let Some(path) = create_parser
            .text_search_parser()
            .and_then(|parser| parser.path())
        {
            self.create_text_search(TextSearchKind::Parser, create_parser.syntax(), &path);
        }
        self.depend_on_refs(
            refs,
            self.index.text_search.range_from(start),
            Object::TextSearch,
        );
    }

    fn lower_create_text_search_template(
        &mut self,
        create_template: &ast::CreateTextSearchTemplate,
    ) {
        let refs = self.index.references.next_idx();
        let start = self.index.text_search.next_idx();
        for option in create_template
            .attribute_list()
            .iter()
            .flat_map(|list| list.attribute_options())
        {
            if let Some(path) = attribute_path(&option) {
                self.lower_function_path_ref(&path, None);
            }
        }
        if let Some(path) = create_template
            .text_search_template()
            .and_then(|template| template.path())
        {
            self.create_text_search(TextSearchKind::Template, create_template.syntax(), &path);
        }
        self.depend_on_refs(
            refs,
            self.index.text_search.range_from(start),
            Object::TextSearch,
        );
    }

    fn lower_create_text_search_dictionary(
        &mut self,
        create_dictionary: &ast::CreateTextSearchDictionary,
    ) {
        for option in create_dictionary
            .attribute_list()
            .iter()
            .flat_map(|list| list.attribute_options())
        {
            if attribute_name(&option).as_deref() == Some("template")
                && let Some(path) = attribute_path(&option)
            {
                self.lower_text_search_ref(TextSearchKind::Template, &path, IfMissing::Error);
            }
        }
        if let Some(path) = create_dictionary
            .text_search_dictionary()
            .and_then(|dictionary| dictionary.path())
        {
            self.create_text_search(
                TextSearchKind::Dictionary,
                create_dictionary.syntax(),
                &path,
            );
        }
    }

    fn lower_create_text_search_configuration(
        &mut self,
        create_configuration: &ast::CreateTextSearchConfiguration,
    ) {
        for option in create_configuration
            .attribute_list()
            .iter()
            .flat_map(|list| list.attribute_options())
        {
            let kind = match attribute_name(&option).as_deref() {
                Some("parser") => TextSearchKind::Parser,
                Some("copy") => TextSearchKind::Configuration,
                _ => continue,
            };
            if let Some(path) = attribute_path(&option) {
                self.lower_text_search_ref(kind, &path, IfMissing::Error);
            }
        }
        if let Some(path) = create_configuration
            .text_search_configuration()
            .and_then(|configuration| configuration.path())
        {
            self.create_text_search(
                TextSearchKind::Configuration,
                create_configuration.syntax(),
                &path,
            );
        }
    }

    fn lower_alter_text_search_parser(&mut self, alter_parser: &ast::AlterTextSearchParser) {
        let parser = alter_parser
            .text_search_parser_ref()
            .and_then(|parser| parser.path_ref())
            .and_then(|path| {
                self.lower_text_search_ref(TextSearchKind::Parser, &path, IfMissing::Error)
            });
        match alter_parser.action() {
            Some(ast::AlterTextSearchParserAction::TextSearchParserRenameTo(rename)) => {
                if let Some(parser) = parser
                    && let Some(segment) = rename
                        .text_search_parser()
                        .and_then(|parser| parser.path())
                        .and_then(|path| path.segment())
                {
                    self.rename_text_search(parser, alter_parser.syntax(), &segment);
                }
            }
            Some(ast::AlterTextSearchParserAction::SetSchema(set_schema)) => {
                self.set_text_search_schema(parser, alter_parser.syntax(), &set_schema);
            }
            None => {}
        }
    }

    fn lower_alter_text_search_template(&mut self, alter_template: &ast::AlterTextSearchTemplate) {
        let template = alter_template
            .text_search_template_ref()
            .and_then(|template| template.path_ref())
            .and_then(|path| {
                self.lower_text_search_ref(TextSearchKind::Template, &path, IfMissing::Error)
            });
        match alter_template.action() {
            Some(ast::AlterTextSearchTemplateAction::TextSearchTemplateRenameTo(rename)) => {
                if let Some(template) = template
                    && let Some(segment) = rename
                        .text_search_template()
                        .and_then(|template| template.path())
                        .and_then(|path| path.segment())
                {
                    self.rename_text_search(template, alter_template.syntax(), &segment);
                }
            }
            Some(ast::AlterTextSearchTemplateAction::SetSchema(set_schema)) => {
                self.set_text_search_schema(template, alter_template.syntax(), &set_schema);
            }
            None => {}
        }
    }

    fn lower_alter_text_search_dictionary(
        &mut self,
        alter_dictionary: &ast::AlterTextSearchDictionary,
    ) {
        let dictionary = alter_dictionary
            .text_search_dictionary_ref()
            .and_then(|dictionary| dictionary.path_ref())
            .and_then(|path| {
                self.lower_text_search_ref(TextSearchKind::Dictionary, &path, IfMissing::Error)
            });
        match alter_dictionary.action() {
            Some(ast::AlterTextSearchDictionaryAction::TextSearchDictionaryRenameTo(rename)) => {
                if let Some(dictionary) = dictionary
                    && let Some(segment) = rename
                        .text_search_dictionary()
                        .and_then(|dictionary| dictionary.path())
                        .and_then(|path| path.segment())
                {
                    self.rename_text_search(dictionary, alter_dictionary.syntax(), &segment);
                }
            }
            Some(ast::AlterTextSearchDictionaryAction::SetSchema(set_schema)) => {
                self.set_text_search_schema(dictionary, alter_dictionary.syntax(), &set_schema);
            }
            Some(ast::AlterTextSearchDictionaryAction::OwnerTo(owner_to)) => {
                self.lower_owner_to(&owner_to);
            }
            Some(ast::AlterTextSearchDictionaryAction::AttributeList(_)) | None => {}
        }
    }

    fn lower_alter_text_search_configuration(
        &mut self,
        alter_configuration: &ast::AlterTextSearchConfiguration,
    ) {
        let configuration = alter_configuration
            .text_search_configuration_ref()
            .and_then(|configuration| configuration.path_ref())
            .and_then(|path| {
                self.lower_text_search_ref(TextSearchKind::Configuration, &path, IfMissing::Error)
            });
        match alter_configuration.action() {
            Some(ast::AlterTextSearchConfigurationAction::AddMapping(add_mapping)) => {
                for dictionary in add_mapping.text_search_dictionary_refs() {
                    self.lower_text_search_dictionary_ref(&dictionary);
                }
            }
            Some(ast::AlterTextSearchConfigurationAction::AlterMapping(alter_mapping)) => {
                for dictionary in alter_mapping.text_search_dictionary_refs() {
                    self.lower_text_search_dictionary_ref(&dictionary);
                }
                if let Some(replace) = alter_mapping.replace_dictionary() {
                    for dictionary in [replace.before(), replace.after()].into_iter().flatten() {
                        self.lower_text_search_dictionary_ref(&dictionary);
                    }
                }
            }
            Some(ast::AlterTextSearchConfigurationAction::TextSearchConfigurationRenameTo(
                rename,
            )) => {
                if let Some(configuration) = configuration
                    && let Some(segment) = rename
                        .text_search_configuration()
                        .and_then(|configuration| configuration.path())
                        .and_then(|path| path.segment())
                {
                    self.rename_text_search(configuration, alter_configuration.syntax(), &segment);
                }
            }
            Some(ast::AlterTextSearchConfigurationAction::SetSchema(set_schema)) => {
                self.set_text_search_schema(
                    configuration,
                    alter_configuration.syntax(),
                    &set_schema,
                );
            }
            Some(ast::AlterTextSearchConfigurationAction::OwnerTo(owner_to)) => {
                self.lower_owner_to(&owner_to);
            }
            Some(ast::AlterTextSearchConfigurationAction::DropMapping(_)) | None => {}
        }
    }

    fn lower_text_search_dictionary_ref(&mut self, dictionary: &ast::TextSearchDictionaryRef) {
        if let Some(path) = dictionary.path_ref() {
            self.lower_text_search_ref(TextSearchKind::Dictionary, &path, IfMissing::Error);
        }
    }

    fn lower_drop_text_search_parser(&mut self, drop_parser: &ast::DropTextSearchParser) {
        self.drop_text_search(
            TextSearchKind::Parser,
            IfMissing::from_if_exists(drop_parser.if_exists().as_ref()),
            drop_parser
                .text_search_parser_ref()
                .and_then(|parser| parser.path_ref()),
        );
    }

    fn lower_drop_text_search_template(&mut self, drop_template: &ast::DropTextSearchTemplate) {
        self.drop_text_search(
            TextSearchKind::Template,
            IfMissing::from_if_exists(drop_template.if_exists().as_ref()),
            drop_template
                .text_search_template_ref()
                .and_then(|template| template.path_ref()),
        );
    }

    fn lower_drop_text_search_dictionary(&mut self, drop_dictionary: &ast::DropTextSearchDict) {
        self.drop_text_search(
            TextSearchKind::Dictionary,
            IfMissing::from_if_exists(drop_dictionary.if_exists().as_ref()),
            drop_dictionary
                .text_search_dictionary_ref()
                .and_then(|dictionary| dictionary.path_ref()),
        );
    }

    fn lower_drop_text_search_configuration(
        &mut self,
        drop_configuration: &ast::DropTextSearchConfig,
    ) {
        self.drop_text_search(
            TextSearchKind::Configuration,
            IfMissing::from_if_exists(drop_configuration.if_exists().as_ref()),
            drop_configuration
                .text_search_configuration_ref()
                .and_then(|configuration| configuration.path_ref()),
        );
    }

    fn drop_text_search(
        &mut self,
        kind: TextSearchKind,
        if_missing: IfMissing,
        path: Option<ast::PathRef>,
    ) {
        if let Some(text_search) =
            path.and_then(|path| self.lower_text_search_ref(kind, &path, if_missing))
        {
            self.drop_text_search_id(text_search);
        }
    }

    fn create_text_search(&mut self, kind: TextSearchKind, node: &SyntaxNode, path: &ast::Path) {
        let Some(segment) = path.segment() else {
            return;
        };
        let schema = self.creation_schema(path);
        let name = self.intern(&segment.text());
        self.alloc_text_search(kind, node, segment.syntax(), schema, name);
    }

    fn rename_text_search(
        &mut self,
        text_search: TextSearchId,
        node: &SyntaxNode,
        segment: &ast::PathSegment,
    ) {
        let TextSearch { kind, schema, .. } = self.index.text_search[text_search];
        let name = self.intern(&segment.text());
        if self.find_text_search_in(kind, schema, name).is_some() {
            self.report_duplicate(ObjectKind::TextSearch(kind), segment.syntax(), name);
            return;
        }
        let moved = self.index.text_search.next_idx();
        self.drop_text_search_id(text_search);
        self.alloc_text_search(kind, node, segment.syntax(), schema, name);
        self.record_moves(
            Object::TextSearch(text_search),
            self.index.text_search.range_from(moved),
            Object::TextSearch,
        );
    }

    fn set_text_search_schema(
        &mut self,
        text_search: Option<TextSearchId>,
        node: &SyntaxNode,
        set_schema: &ast::SetSchema,
    ) {
        let Some(schema_ref) = set_schema.schema_ref() else {
            return;
        };
        let Some(schema) = self.lower_set_schema_ref(&schema_ref) else {
            return;
        };
        let Some(text_search) = text_search else {
            return;
        };
        let TextSearch {
            kind,
            name,
            name_ptr,
            ..
        } = self.index.text_search[text_search];
        if self.find_text_search_in(kind, schema, name).is_some() {
            let error = SemaError::DuplicateInSchema {
                object: ObjectKind::TextSearch(kind),
                name,
                schema,
            };
            self.push_error(schema_ref.syntax(), error);
            return;
        }
        let moved = self.index.text_search.next_idx();
        self.drop_text_search_id(text_search);
        self.index.text_search.alloc(TextSearch {
            kind,
            schema,
            name,
            ptr: self.ptr(node),
            name_ptr,
        });
        self.record_moves(
            Object::TextSearch(text_search),
            self.index.text_search.range_from(moved),
            Object::TextSearch,
        );
    }

    fn drop_publication(&mut self, publication: PublicationId) {
        if self.dropped_publications.insert(publication) && self.in_transaction() {
            self.undo.push(Undo::DropPublication(publication));
        }
    }

    fn drop_subscription(&mut self, subscription: SubscriptionId) {
        if self.dropped_subscriptions.insert(subscription) && self.in_transaction() {
            self.undo.push(Undo::DropSubscription(subscription));
        }
    }

    fn drop_text_search_id(&mut self, text_search: TextSearchId) {
        if self.dropped_text_search.insert(text_search) && self.in_transaction() {
            self.undo.push(Undo::DropTextSearch(text_search));
        }
    }

    fn alloc_text_search(
        &mut self,
        kind: TextSearchKind,
        node: &SyntaxNode,
        name_node: &SyntaxNode,
        schema: Name,
        name: Name,
    ) {
        if self.find_text_search_in(kind, schema, name).is_some() {
            self.report_duplicate(ObjectKind::TextSearch(kind), name_node, name);
        }
        let id = self.index.text_search.alloc(TextSearch {
            kind,
            schema,
            name,
            ptr: self.ptr(node),
            name_ptr: self.ptr(name_node),
        });
        self.record_comment(node, Item::TextSearch(id));
        self.add_item(name_node, Item::TextSearch(id));
    }

    fn lower_text_search_ref(
        &mut self,
        kind: TextSearchKind,
        path: &ast::PathRef,
        if_missing: IfMissing,
    ) -> Option<TextSearchId> {
        let segment = path.segment()?;
        let name = self.intern(&segment.text());
        let schema = self.lower_schema_qualifier(path.qualifier());
        let text_search = self
            .search_path(schema, Lookup::TextSearch)
            .find_map(|schema| self.find_text_search_in(kind, schema, name));
        let resolution = match (text_search, if_missing) {
            (Some(text_search), _) => Resolution::TextSearch(text_search),
            (None, IfMissing::Ignore) => Resolution::Unknown,
            (None, IfMissing::Error) => Resolution::Unresolved,
        };
        self.add_ref(
            name,
            ReferenceKind::TextSearch(kind),
            segment.syntax(),
            resolution,
        );
        text_search
    }

    fn find_text_search_in(
        &self,
        kind: TextSearchKind,
        schema: Name,
        name: Name,
    ) -> Option<TextSearchId> {
        self.index.text_search.named(schema, name).find(|id| {
            let text_search = &self.index.text_search[*id];
            text_search.kind == kind
                && self.visible(text_search.schema, text_search.ptr.file_id)
                && !self.dropped_text_search.contains(id)
        })
    }

    fn lower_create_publication(&mut self, create_publication: &ast::CreatePublication) {
        match create_publication.publication_for_clause() {
            Some(ast::PublicationForClause::ForPublicationObjects(objects)) => {
                for object in objects.publication_objects() {
                    self.lower_publication_object(&object);
                }
            }
            Some(ast::PublicationForClause::ForAllPublicationObjects(all)) => {
                if let Some(except) = all.except_table_clause() {
                    self.lower_publication_except_tables(&except);
                }
            }
            None => {}
        }
        if let Some(publication) = create_publication.publication() {
            self.alloc_publication(create_publication.syntax(), &publication);
        }
    }

    fn lower_alter_publication(&mut self, alter_publication: &ast::AlterPublication) {
        let publication = alter_publication
            .publication_ref()
            .and_then(|publication| self.lower_publication_ref(&publication, IfMissing::Error));
        match alter_publication.action() {
            Some(ast::AlterPublicationAction::AddPublicationObjects(add)) => {
                for object in add.publication_objects() {
                    self.lower_publication_object(&object);
                }
            }
            Some(ast::AlterPublicationAction::DropPublicationObjects(drop)) => {
                for object in drop.publication_objects() {
                    self.lower_publication_object(&object);
                }
            }
            Some(ast::AlterPublicationAction::SetPublicationObjects(set)) => {
                for object in set.publication_objects() {
                    self.lower_publication_object(&object);
                }
            }
            Some(ast::AlterPublicationAction::SetAllPublicationObjectList(set_all)) => {
                if let Some(except) = set_all.except_table_clause() {
                    self.lower_publication_except_tables(&except);
                }
            }
            Some(ast::AlterPublicationAction::PublicationRenameTo(rename)) => {
                if let Some(publication) = publication
                    && let Some(new_publication) = rename.publication()
                {
                    let name = self.intern(&new_publication.text());
                    if self.find_publication(name).is_some() {
                        self.report_duplicate(
                            ObjectKind::Publication,
                            new_publication.syntax(),
                            name,
                        );
                        return;
                    }
                    self.drop_publication(publication);
                    let id = self.alloc_publication(alter_publication.syntax(), &new_publication);
                    self.copy_comment(Item::Publication(publication), Item::Publication(id));
                }
            }
            Some(ast::AlterPublicationAction::OwnerTo(owner_to)) => self.lower_owner_to(&owner_to),
            Some(ast::AlterPublicationAction::SetOptions(_)) | None => {}
        }
    }

    fn lower_drop_publication(&mut self, drop_publication: &ast::DropPublication) {
        let if_missing = IfMissing::from_if_exists(drop_publication.if_exists().as_ref());
        for publication_ref in drop_publication.publication_refs() {
            if let Some(publication) = self.lower_publication_ref(&publication_ref, if_missing) {
                self.drop_publication(publication);
            }
        }
    }

    fn lower_publication_object(&mut self, object: &ast::PublicationObject) {
        match object {
            ast::PublicationObject::PublicationObjectTable(table_object) => {
                let table = table_object
                    .table_name_ref()
                    .and_then(|name| name.path_ref())
                    .and_then(|path| {
                        self.lower_table_ref(&path, IfMissing::Error, TableLookup::Table)
                    });
                if let Some(list) = table_object.column_ref_list() {
                    let columns = match table {
                        Some(table) => self.index.tables[table].columns,
                        None => self.unknown_columns(),
                    };
                    for column_ref in list.column_name_refs() {
                        self.lower_column_ref_in(&columns, &column_ref);
                    }
                }
                if let Some(expr) = table_object
                    .where_condition_clause()
                    .and_then(|clause| clause.expr())
                {
                    self.with_table_scope(
                        QueryKind::Publication,
                        table,
                        table_object.syntax(),
                        |this| {
                            this.lower_expr(&expr);
                        },
                    );
                }
            }
            ast::PublicationObject::PublicationObjectTablesInSchema(schema_object) => {
                if let Some(schema) = schema_object.schema_ref() {
                    self.lower_schema_ref(schema.syntax(), &schema.text());
                }
                if let Some(expr) = schema_object
                    .where_condition_clause()
                    .and_then(|clause| clause.expr())
                {
                    self.with_table_scope(
                        QueryKind::Publication,
                        None,
                        schema_object.syntax(),
                        |this| {
                            this.lower_expr(&expr);
                        },
                    );
                }
            }
            ast::PublicationObject::PublicationObjectCurrentSchema(_) => {}
        }
    }

    fn lower_publication_except_tables(&mut self, except: &ast::ExceptTableClause) {
        for path in except
            .except_table_names()
            .filter_map(|name| name.table_relation_name())
            .filter_map(|name| name.table_name_ref())
            .filter_map(|name| name.path_ref())
        {
            self.lower_table_ref(&path, IfMissing::Error, TableLookup::Table);
        }
    }

    fn alloc_publication(&mut self, node: &SyntaxNode, name_node: &impl NameLike) -> PublicationId {
        let name = self.intern(&name_node.text());
        if self.find_publication(name).is_some() {
            self.report_duplicate(ObjectKind::Publication, name_node.syntax(), name);
        }
        let id = self.index.publications.alloc(Publication {
            name,
            ptr: self.ptr(node),
            name_ptr: self.ptr(name_node.syntax()),
        });
        self.record_comment(node, Item::Publication(id));
        self.add_item(name_node.syntax(), Item::Publication(id));
        id
    }

    fn lower_publication_ref(
        &mut self,
        publication_ref: &ast::PublicationRef,
        if_missing: IfMissing,
    ) -> Option<PublicationId> {
        let name = self.intern(&publication_ref.text());
        let publication = self.find_publication(name);
        let resolution = match (publication, if_missing) {
            (Some(publication), _) => Resolution::Publication(publication),
            (None, IfMissing::Ignore) => Resolution::Unknown,
            (None, IfMissing::Error) => Resolution::Unresolved,
        };
        self.add_ref(
            name,
            ReferenceKind::Publication,
            publication_ref.syntax(),
            resolution,
        );
        publication
    }

    fn find_publication(&self, name: Name) -> Option<PublicationId> {
        self.index
            .publications
            .get(name)
            .find(|id| !self.dropped_publications.contains(id))
    }

    fn lower_create_subscription(&mut self, create_subscription: &ast::CreateSubscription) {
        match create_subscription.source() {
            Some(ast::SubscriptionSource::ServerClause(server)) => {
                if let Some(server_ref) = server.server_ref() {
                    self.lower_server_ref(&server_ref, IfMissing::Error);
                }
            }
            Some(ast::SubscriptionSource::ConnectionClause(_)) | None => {}
        }
        if let Some(subscription) = create_subscription.subscription() {
            self.alloc_subscription(create_subscription.syntax(), &subscription);
        }
    }

    fn lower_alter_subscription(&mut self, alter_subscription: &ast::AlterSubscription) {
        let subscription = alter_subscription
            .subscription_ref()
            .and_then(|subscription| self.lower_subscription_ref(&subscription, IfMissing::Error));
        match alter_subscription.action() {
            Some(ast::AlterSubscriptionAction::ServerClause(server)) => {
                if let Some(server_ref) = server.server_ref() {
                    self.lower_server_ref(&server_ref, IfMissing::Error);
                }
            }
            Some(ast::AlterSubscriptionAction::SubscriptionRenameTo(rename)) => {
                if let Some(subscription) = subscription
                    && let Some(new_subscription) = rename.subscription()
                {
                    let name = self.intern(&new_subscription.text());
                    if self.find_subscription(name).is_some() {
                        self.report_duplicate(
                            ObjectKind::Subscription,
                            new_subscription.syntax(),
                            name,
                        );
                        return;
                    }
                    self.drop_subscription(subscription);
                    let id =
                        self.alloc_subscription(alter_subscription.syntax(), &new_subscription);
                    self.copy_comment(Item::Subscription(subscription), Item::Subscription(id));
                }
            }
            Some(ast::AlterSubscriptionAction::OwnerTo(owner_to)) => self.lower_owner_to(&owner_to),
            Some(
                ast::AlterSubscriptionAction::AddPublication(_)
                | ast::AlterSubscriptionAction::ConnectionClause(_)
                | ast::AlterSubscriptionAction::DisableSubscription(_)
                | ast::AlterSubscriptionAction::DropSubscriptionPublication(_)
                | ast::AlterSubscriptionAction::EnableSubscription(_)
                | ast::AlterSubscriptionAction::RefreshPublication(_)
                | ast::AlterSubscriptionAction::SetOptions(_)
                | ast::AlterSubscriptionAction::SetPublication(_)
                | ast::AlterSubscriptionAction::SkipSubscription(_),
            )
            | None => {}
        }
    }

    fn lower_drop_subscription(&mut self, drop_subscription: &ast::DropSubscription) {
        let if_missing = IfMissing::from_if_exists(drop_subscription.if_exists().as_ref());
        if let Some(subscription) = drop_subscription
            .subscription_ref()
            .and_then(|subscription| self.lower_subscription_ref(&subscription, if_missing))
        {
            self.drop_subscription(subscription);
        }
    }

    fn alloc_subscription(
        &mut self,
        node: &SyntaxNode,
        name_node: &impl NameLike,
    ) -> SubscriptionId {
        let name = self.intern(&name_node.text());
        if self.find_subscription(name).is_some() {
            self.report_duplicate(ObjectKind::Subscription, name_node.syntax(), name);
        }
        let id = self.index.subscriptions.alloc(Subscription {
            name,
            ptr: self.ptr(node),
            name_ptr: self.ptr(name_node.syntax()),
        });
        self.record_comment(node, Item::Subscription(id));
        self.add_item(name_node.syntax(), Item::Subscription(id));
        id
    }

    fn lower_subscription_ref(
        &mut self,
        subscription_ref: &ast::SubscriptionRef,
        if_missing: IfMissing,
    ) -> Option<SubscriptionId> {
        let name = self.intern(&subscription_ref.text());
        let subscription = self.find_subscription(name);
        let resolution = match (subscription, if_missing) {
            (Some(subscription), _) => Resolution::Subscription(subscription),
            (None, IfMissing::Ignore) => Resolution::Unknown,
            (None, IfMissing::Error) => Resolution::Unresolved,
        };
        self.add_ref(
            name,
            ReferenceKind::Subscription,
            subscription_ref.syntax(),
            resolution,
        );
        subscription
    }

    fn find_subscription(&self, name: Name) -> Option<SubscriptionId> {
        self.index
            .subscriptions
            .get(name)
            .find(|id| !self.dropped_subscriptions.contains(id))
    }

    fn lower_create_property_graph(&mut self, create_property_graph: &ast::CreatePropertyGraph) {
        let Some(path) = create_property_graph
            .property_graph()
            .and_then(|graph| graph.path())
        else {
            return;
        };
        let columns = Columns {
            range: self.index.columns.alloc_many([]),
            complete: true,
        };
        let graph = self.alloc_table(
            TableKind::PropertyGraph,
            Persistence::from_ast(create_property_graph.persistence().as_ref()),
            IfExists::Error,
            create_property_graph.syntax(),
            &path,
            columns,
        );
        if let Some(vertex_tables) = create_property_graph.vertex_tables() {
            self.lower_vertex_tables(graph, &vertex_tables);
        }
        if let Some(edge_tables) = create_property_graph.edge_tables() {
            self.lower_edge_tables(graph, &edge_tables);
        }
    }

    fn lower_alter_property_graph(&mut self, alter_property_graph: &ast::AlterPropertyGraph) {
        let graph = alter_property_graph
            .property_graph_ref()
            .and_then(|graph| graph.path_ref())
            .and_then(|path| {
                self.lower_table_ref(
                    &path,
                    IfMissing::from_if_exists(alter_property_graph.if_exists().as_ref()),
                    TableLookup::PropertyGraph,
                )
            });
        match alter_property_graph.action() {
            Some(ast::AlterPropertyGraphAction::AddVertexEdgeTables(add)) => {
                if let Some(vertex_tables) =
                    add.add_vertex_tables().and_then(|add| add.vertex_tables())
                {
                    self.lower_vertex_tables(graph, &vertex_tables);
                }
                if let Some(edge_tables) = add.add_edge_tables().and_then(|add| add.edge_tables()) {
                    self.lower_edge_tables(graph, &edge_tables);
                }
            }
            Some(ast::AlterPropertyGraphAction::DropVertexTables(drop)) => {
                for element_ref in drop.element_table_refs() {
                    if let Some(element) =
                        self.lower_graph_element_ref(graph, GraphElementKind::Vertex, &element_ref)
                    {
                        self.drop_graph_element(element);
                    }
                }
            }
            Some(ast::AlterPropertyGraphAction::DropEdgeTables(drop)) => {
                for element_ref in drop.element_table_refs() {
                    if let Some(element) =
                        self.lower_graph_element_ref(graph, GraphElementKind::Edge, &element_ref)
                    {
                        self.drop_graph_element(element);
                    }
                }
            }
            Some(ast::AlterPropertyGraphAction::AlterVertexEdgeLabels(alter)) => {
                let kind = graph_element_kind(alter.vertex_or_edge().as_ref());
                let element = alter.element_table_ref().and_then(|element_ref| {
                    self.lower_graph_element_ref(graph, kind, &element_ref)
                });
                for add_label in alter.add_labels() {
                    let table =
                        element.and_then(|element| self.index.graph_elements[element].table);
                    let label = match (element, add_label.label()) {
                        (Some(element), Some(label)) => {
                            Some(self.alloc_named_graph_label(element, &label))
                        }
                        (Some(_) | None, None) | (None, Some(_)) => None,
                    };
                    self.lower_label_properties(
                        table,
                        label,
                        add_label.element_table_properties().as_ref(),
                    );
                }
            }
            Some(ast::AlterPropertyGraphAction::DropVertexEdgeLabel(drop)) => {
                let kind = graph_element_kind(drop.vertex_or_edge().as_ref());
                let element = drop.element_table_ref().and_then(|element_ref| {
                    self.lower_graph_element_ref(graph, kind, &element_ref)
                });
                if let Some(label_ref) = drop.label_ref()
                    && let Some(label) = self.lower_graph_label_ref(element, &label_ref)
                {
                    self.drop_graph_label(label);
                }
            }
            Some(ast::AlterPropertyGraphAction::AddVertexEdgeLabelProperties(add)) => {
                let kind = graph_element_kind(add.vertex_or_edge().as_ref());
                let element = add.element_table_ref().and_then(|element_ref| {
                    self.lower_graph_element_ref(graph, kind, &element_ref)
                });
                let label = add
                    .alter_label()
                    .and_then(|label| label.label_ref())
                    .and_then(|label_ref| self.lower_graph_label_ref(element, &label_ref));
                if let Some(list) = add.expr_as_property_name_list() {
                    let table =
                        element.and_then(|element| self.index.graph_elements[element].table);
                    self.lower_graph_property_exprs(table, label, &list);
                }
            }
            Some(ast::AlterPropertyGraphAction::DropVertexEdgeLabelProperties(drop)) => {
                let kind = graph_element_kind(drop.vertex_or_edge().as_ref());
                let element = drop.element_table_ref().and_then(|element_ref| {
                    self.lower_graph_element_ref(graph, kind, &element_ref)
                });
                let label = drop
                    .alter_label()
                    .and_then(|label| label.label_ref())
                    .and_then(|label_ref| self.lower_graph_label_ref(element, &label_ref));
                for property_ref in drop.property_name_refs() {
                    if let Some(property) = self.lower_graph_property_ref(label, &property_ref) {
                        self.drop_graph_property(property);
                    }
                }
            }
            Some(ast::AlterPropertyGraphAction::OwnerTo(owner_to)) => {
                self.lower_owner_to(&owner_to);
            }
            Some(ast::AlterPropertyGraphAction::PropertyGraphRenameTo(rename)) => {
                if let Some(graph) = graph
                    && let Some(path) = rename.property_graph().and_then(|graph| graph.path())
                {
                    self.rename_table(graph, &path);
                }
            }
            Some(ast::AlterPropertyGraphAction::SetSchema(set_schema)) => {
                self.set_table_schema(graph, &set_schema);
            }
            None => {}
        }
    }

    fn lower_drop_property_graph(&mut self, drop_property_graph: &ast::DropPropertyGraph) {
        if let Some(path) = drop_property_graph
            .property_graph_ref()
            .and_then(|graph| graph.path_ref())
        {
            let if_missing = IfMissing::from_if_exists(drop_property_graph.if_exists().as_ref());
            if let Some(graph) =
                self.lower_drop_table_ref(&path, if_missing, TableLookup::PropertyGraph)
            {
                self.drop_table(graph);
            }
        }
    }

    fn lower_vertex_tables(&mut self, graph: Option<TableId>, vertex_tables: &ast::VertexTables) {
        for def in vertex_tables.vertex_table_defs() {
            self.lower_graph_element_def(
                graph,
                GraphElementKind::Vertex,
                def.table_name_ref(),
                def.element_table_alias_clause(),
                def.element_table_key_clause(),
                def.element_table_label_and_properties(),
            );
        }
    }

    fn lower_edge_tables(&mut self, graph: Option<TableId>, edge_tables: &ast::EdgeTables) {
        for def in edge_tables.edge_table_defs() {
            let element = self.lower_graph_element_def(
                graph,
                GraphElementKind::Edge,
                def.table_name_ref(),
                def.element_table_alias_clause(),
                def.element_table_key_clause(),
                def.element_table_label_and_properties(),
            );
            let table = element.and_then(|element| self.index.graph_elements[element].table);
            let source = def.source_vertex_table().and_then(|source| {
                self.lower_edge_endpoint(
                    graph,
                    table,
                    source.column_ref_list(),
                    source.references_table(),
                    source.vertex_table_ref(),
                )
            });
            let destination = def.dest_vertex_table().and_then(|dest| {
                self.lower_edge_endpoint(
                    graph,
                    table,
                    dest.column_ref_list(),
                    dest.references_table(),
                    dest.vertex_table_ref(),
                )
            });
            if let Some(element) = element {
                self.index
                    .graph_elements
                    .set_endpoints(element, source, destination);
            }
        }
    }

    fn lower_graph_element_def(
        &mut self,
        graph: Option<TableId>,
        kind: GraphElementKind,
        table_name_ref: Option<ast::TableNameRef>,
        alias_clause: Option<ast::ElementTableAliasClause>,
        key_clause: Option<ast::ElementTableKeyClause>,
        label_and_properties: Option<ast::ElementTableLabelAndProperties>,
    ) -> Option<GraphElementId> {
        let path = table_name_ref.and_then(|name| name.path_ref())?;
        let segment = path.segment()?;
        let table = self.lower_graph_element_table_ref(&path);
        if let Some(list) = key_clause.and_then(|key| key.column_ref_list()) {
            self.lower_column_ref_list(table, &list);
        }
        let element = graph.map(|graph| {
            self.alloc_graph_element(
                graph,
                kind,
                table,
                &segment,
                alias_clause.and_then(|clause| clause.alias()).as_ref(),
            )
        });
        match label_and_properties {
            Some(ast::ElementTableLabelAndProperties::LabelAndPropertiesList(list)) => {
                for label_and_properties in list.label_and_propertiess() {
                    let label = match (element, label_and_properties.label_spec()) {
                        (Some(element), Some(ast::LabelSpec::NamedLabel(named))) => named
                            .label()
                            .map(|label| self.alloc_named_graph_label(element, &label)),
                        (Some(element), Some(ast::LabelSpec::DefaultLabel(default))) => {
                            let alias = self.index.graph_elements[element].alias;
                            let label = self.alloc_graph_label(element, alias, default.syntax());
                            self.add_item(default.syntax(), Item::GraphLabel(label));
                            Some(label)
                        }
                        (Some(_), None) | (None, _) => None,
                    };
                    self.lower_label_properties(
                        table,
                        label,
                        label_and_properties.element_table_properties().as_ref(),
                    );
                }
            }
            Some(ast::ElementTableLabelAndProperties::ElementTableProperties(properties)) => {
                let label = element.map(|element| self.alloc_implicit_graph_label(element));
                self.lower_label_properties(table, label, Some(&properties));
            }
            None => {
                let label = element.map(|element| self.alloc_implicit_graph_label(element));
                self.lower_label_properties(table, label, None);
            }
        }
        element
    }

    fn alloc_graph_element(
        &mut self,
        graph: TableId,
        kind: GraphElementKind,
        table: Option<TableId>,
        segment: &ast::PathSegmentRef,
        alias_node: Option<&ast::ElementTableAlias>,
    ) -> GraphElementId {
        let name_node = match alias_node {
            Some(alias) => alias.syntax().clone(),
            None => segment.syntax().clone(),
        };
        let alias = match alias_node {
            Some(alias) => alias.text(),
            None => segment.text(),
        };
        let alias = self.intern(&alias);
        if self.find_graph_element(graph, None, alias).is_some() {
            self.report_duplicate(ObjectKind::GraphElement, &name_node, alias);
        }
        let element = self.index.graph_elements.alloc(GraphElement {
            graph,
            kind,
            alias,
            table,
            source: None,
            destination: None,
            name_ptr: self.ptr(&name_node),
        });
        if let Some(def) = segment.syntax().ancestors().find(|node| {
            ast::VertexTableDef::can_cast(node.kind()) || ast::EdgeTableDef::can_cast(node.kind())
        }) {
            self.record_comment(&def, Item::GraphElement(element));
        }
        if alias_node.is_some() {
            self.add_item(&name_node, Item::GraphElement(element));
        }
        element
    }

    fn lower_edge_endpoint(
        &mut self,
        graph: Option<TableId>,
        table: Option<TableId>,
        key: Option<ast::ColumnRefList>,
        references: Option<ast::ReferencesTable>,
        vertex_ref: Option<ast::VertexTableRef>,
    ) -> Option<GraphElementId> {
        if let Some(list) = key {
            self.lower_column_ref_list(table, &list);
        }
        let vertex_ref = references
            .as_ref()
            .and_then(|references| references.vertex_table_ref())
            .or(vertex_ref);
        let vertex = vertex_ref.and_then(|vertex_ref| {
            self.lower_graph_element_ref(graph, GraphElementKind::Vertex, &vertex_ref)
        });
        if let Some(list) = references.and_then(|references| references.column_ref_list()) {
            let table = vertex.and_then(|vertex| self.index.graph_elements[vertex].table);
            self.lower_column_ref_list(table, &list);
        }
        vertex
    }

    fn lower_column_ref_list(&mut self, table: Option<TableId>, list: &ast::ColumnRefList) {
        let columns = match table {
            Some(table) => self.index.tables[table].columns,
            None => self.unknown_columns(),
        };
        for column_ref in list.column_name_refs() {
            self.lower_column_ref_in(&columns, &column_ref);
        }
    }

    fn lower_label_properties(
        &mut self,
        table: Option<TableId>,
        label: Option<GraphLabelId>,
        properties: Option<&ast::ElementTableProperties>,
    ) {
        match properties {
            Some(ast::ElementTableProperties::Properties(properties)) => {
                if let Some(list) = properties.expr_as_property_name_list() {
                    self.lower_graph_property_exprs(table, label, &list);
                }
            }
            Some(ast::ElementTableProperties::NoProperties(_)) => {}
            Some(ast::ElementTableProperties::AllProperties(_)) | None => {
                if let (Some(label), Some(table)) = (label, table) {
                    let columns = self.index.tables[table].columns;
                    for column in columns.range {
                        if let Some(name) = self.index.columns[column].name {
                            self.index.graph_properties.alloc(GraphProperty {
                                label,
                                name,
                                column,
                            });
                        }
                    }
                }
            }
        }
    }

    fn lower_graph_property_exprs(
        &mut self,
        table: Option<TableId>,
        label: Option<GraphLabelId>,
        list: &ast::ExprAsPropertyNameList,
    ) {
        self.with_table_scope(QueryKind::PropertyGraph, table, list.syntax(), |this| {
            for property in list.expr_as_property_names() {
                let expr = property.expr();
                let ty = expr.as_ref().and_then(|expr| this.lower_expr(expr));
                let Some(label) = label else {
                    continue;
                };
                if let Some(property_name) = property.property_name() {
                    let name_node = property_name.syntax();
                    let name = this.intern(&property_name.text());
                    let column = this.index.columns.alloc(Column {
                        name: Some(name),
                        ptr: this.ptr(name_node),
                        ty,
                        merge: None,
                    });
                    if let Some(property) =
                        this.alloc_graph_property(label, name, column, name_node)
                    {
                        this.add_item(name_node, Item::GraphProperty(property));
                    }
                } else if let Some(ast::Expr::NameRef(name_ref)) = &expr
                    && let Some(table) = table
                {
                    let name = this.intern(&name_ref.text());
                    if let Some(column) = this.find_table_column(table, name) {
                        this.alloc_graph_property(label, name, column, name_ref.syntax());
                    }
                }
            }
        });
    }

    fn alloc_graph_property(
        &mut self,
        label: GraphLabelId,
        name: Name,
        column: ColumnId,
        name_node: &SyntaxNode,
    ) -> Option<GraphPropertyId> {
        if self.find_graph_property(label, name).is_some() {
            self.report_duplicate(ObjectKind::GraphProperty, name_node, name);
            return None;
        }
        Some(self.index.graph_properties.alloc(GraphProperty {
            label,
            name,
            column,
        }))
    }

    fn lower_graph_property_ref(
        &mut self,
        label: Option<GraphLabelId>,
        property_ref: &ast::PropertyNameRef,
    ) -> Option<GraphPropertyId> {
        let name = self.intern(&property_ref.text());
        let property = label.and_then(|label| self.find_graph_property(label, name));
        let resolution = match (property, label) {
            (Some(property), _) => Resolution::GraphProperty(property),
            (None, Some(_)) => Resolution::Unresolved,
            (None, None) => Resolution::Unknown,
        };
        self.add_ref(
            name,
            ReferenceKind::GraphProperty,
            property_ref.syntax(),
            resolution,
        );
        property
    }

    fn find_graph_property(&self, label: GraphLabelId, name: Name) -> Option<GraphPropertyId> {
        self.index.graph_properties.get(label).find(|id| {
            self.index.graph_properties[*id].name == name
                && !self.dropped_graph_properties.contains(id)
        })
    }

    fn drop_graph_property(&mut self, property: GraphPropertyId) {
        if self.dropped_graph_properties.insert(property) && self.in_transaction() {
            self.undo.push(Undo::DropGraphProperty(property));
        }
    }

    fn lower_graph_element_table_ref(&mut self, path: &ast::PathRef) -> Option<TableId> {
        let segment = path.segment()?;
        let name = self.intern(&segment.text());
        let schema = self.lower_schema_qualifier(path.qualifier());
        let table =
            self.find_table(schema, name)
                .filter(|table| match self.index.tables[*table].kind {
                    TableKind::Table
                    | TableKind::ForeignTable
                    | TableKind::View
                    | TableKind::MaterializedView => true,
                    TableKind::Sequence | TableKind::PropertyGraph => false,
                });
        let resolution = match table {
            Some(table) => Resolution::Table(table),
            None => Resolution::Unresolved,
        };
        self.add_ref(name, ReferenceKind::Table, segment.syntax(), resolution);
        table
    }

    fn alloc_named_graph_label(
        &mut self,
        element: GraphElementId,
        label: &ast::Label,
    ) -> GraphLabelId {
        let name = self.intern(&label.text());
        let label_id = self.alloc_graph_label(element, name, label.syntax());
        self.add_item(label.syntax(), Item::GraphLabel(label_id));
        label_id
    }

    fn alloc_implicit_graph_label(&mut self, element: GraphElementId) -> GraphLabelId {
        let GraphElement {
            alias, name_ptr, ..
        } = self.index.graph_elements[element];
        self.index.graph_labels.alloc(GraphLabel {
            element,
            name: alias,
            name_ptr,
        })
    }

    fn alloc_graph_label(
        &mut self,
        element: GraphElementId,
        name: Name,
        name_node: &SyntaxNode,
    ) -> GraphLabelId {
        if self.find_graph_label(element, name).is_some() {
            self.report_duplicate(ObjectKind::GraphLabel, name_node, name);
        }
        self.index.graph_labels.alloc(GraphLabel {
            element,
            name,
            name_ptr: self.ptr(name_node),
        })
    }

    fn lower_graph_element_ref(
        &mut self,
        graph: Option<TableId>,
        kind: GraphElementKind,
        name_node: &impl NameLike,
    ) -> Option<GraphElementId> {
        let name = self.intern(&name_node.text());
        let element = graph.and_then(|graph| self.find_graph_element(graph, Some(kind), name));
        let resolution = match (element, graph) {
            (Some(element), _) => Resolution::GraphElement(element),
            (None, Some(_)) => Resolution::Unresolved,
            (None, None) => Resolution::Unknown,
        };
        self.add_ref(
            name,
            ReferenceKind::GraphElement,
            name_node.syntax(),
            resolution,
        );
        element
    }

    fn lower_graph_label_ref(
        &mut self,
        element: Option<GraphElementId>,
        label_ref: &ast::LabelRef,
    ) -> Option<GraphLabelId> {
        let name = self.intern(&label_ref.text());
        let label = element.and_then(|element| self.find_graph_label(element, name));
        let resolution = match (label, element) {
            (Some(label), _) => Resolution::GraphLabel(label),
            (None, Some(_)) => Resolution::Unresolved,
            (None, None) => Resolution::Unknown,
        };
        self.add_ref(
            name,
            ReferenceKind::GraphLabel,
            label_ref.syntax(),
            resolution,
        );
        label
    }

    fn find_graph_element(
        &self,
        graph: TableId,
        kind: Option<GraphElementKind>,
        alias: Name,
    ) -> Option<GraphElementId> {
        self.index.graph_elements.get(graph).find(|id| {
            let element = &self.index.graph_elements[*id];
            element.alias == alias
                && kind.is_none_or(|kind| element.kind == kind)
                && self.graph_element_live(*id)
        })
    }

    fn graph_element_live(&self, id: GraphElementId) -> bool {
        graph_element_live(
            &self.index.graph_elements,
            &self.dropped_graph_elements,
            &self.dropped_tables,
            id,
        )
    }

    fn find_graph_label(&self, element: GraphElementId, name: Name) -> Option<GraphLabelId> {
        self.index.graph_labels.get(element).find(|id| {
            self.index.graph_labels[*id].name == name && !self.dropped_graph_labels.contains(id)
        })
    }

    fn lower_graph_table(&mut self, graph_table: &ast::GraphTableFn) -> Columns {
        let graph = graph_table
            .property_graph_ref()
            .and_then(|graph| graph.path_ref())
            .and_then(|path| {
                self.lower_table_ref(&path, IfMissing::Error, TableLookup::PropertyGraph)
            });
        let query = self.alloc_query(QueryKind::GraphTable, graph_table.syntax());
        self.push_scope(query);
        let patterns = graph_table.path_pattern_list();
        for pattern in patterns.iter().flat_map(|list| list.path_patterns()) {
            visit_path_primaries(&pattern, &mut |primary| {
                self.declare_element_variable(graph, query, primary);
            });
        }
        self.set_visible((0..self.scope_sources_len()).into());
        for pattern in patterns.iter().flat_map(|list| list.path_patterns()) {
            visit_path_primaries(&pattern, &mut |primary| {
                if let Some(expr) = primary.where_clause().and_then(|clause| clause.expr()) {
                    self.lower_expr(&expr);
                }
            });
        }
        if let Some(expr) = graph_table.where_clause().and_then(|clause| clause.expr()) {
            self.lower_expr(&expr);
        }
        let columns = match graph_table.expr_as_column_name_list() {
            Some(list) => self.lower_graph_table_columns(&list),
            None => Columns {
                range: self.index.columns.alloc_many([]),
                complete: true,
            },
        };
        self.scopes.pop();
        columns
    }

    fn declare_element_variable(
        &mut self,
        graph: Option<TableId>,
        query: QueryId,
        primary: &ast::PathPrimary,
    ) {
        let (kind, variable, is_label) = match primary {
            ast::PathPrimary::VertexPattern(vertex) => (
                GraphElementKind::Vertex,
                vertex.element_variable(),
                vertex.is_label(),
            ),
            ast::PathPrimary::EdgeAny(edge) => (
                GraphElementKind::Edge,
                edge.element_variable(),
                edge.is_label(),
            ),
            ast::PathPrimary::EdgeLeft(edge) => (
                GraphElementKind::Edge,
                edge.element_variable(),
                edge.is_label(),
            ),
            ast::PathPrimary::EdgeRight(edge) => (
                GraphElementKind::Edge,
                edge.element_variable(),
                edge.is_label(),
            ),
            ast::PathPrimary::ParenGraphPattern(_) => return,
        };
        let mut names = SmallVec::<[Name; 2]>::new();
        let any_label = match is_label.and_then(|is_label| is_label.expr()) {
            Some(expr) => self.lower_label_expr(graph, &expr, &mut names),
            None => true,
        };
        let Some(variable) = variable else {
            return;
        };
        let name = self.intern(&ast::normalize_name_node(variable.syntax()));
        if let Some(existing) = self.find_query_source(query, name) {
            self.add_ref(
                name,
                ReferenceKind::Relation,
                variable.syntax(),
                Resolution::Source(existing),
            );
            return;
        }
        let columns = match graph {
            Some(graph) => self.element_variable_columns(graph, kind, any_label, &names),
            None => self.unknown_columns(),
        };
        let source = self.index.sources.alloc(Source {
            name: Some(name),
            kind: SourceKind::ElementVariable,
            ptr: self.ptr(variable.syntax()),
            alias_ptr: None,
            columns,
        });
        self.index.queries[query].sources.push(source);
        self.add_item(variable.syntax(), Item::Source(source));
    }

    fn lower_label_expr(
        &mut self,
        graph: Option<TableId>,
        expr: &ast::Expr,
        names: &mut SmallVec<[Name; 2]>,
    ) -> bool {
        match expr {
            ast::Expr::NameRef(name_ref) => {
                if let Some(name) = self.name_ref_name(name_ref) {
                    let label = graph.and_then(|graph| self.find_graph_label_in(graph, name));
                    let resolution = match (label, graph) {
                        (Some(label), _) => Resolution::GraphLabel(label),
                        (None, Some(_)) => Resolution::Unresolved,
                        (None, None) => Resolution::Unknown,
                    };
                    self.add_ref(
                        name,
                        ReferenceKind::GraphLabel,
                        name_ref.syntax(),
                        resolution,
                    );
                    names.push(name);
                }
                false
            }
            ast::Expr::BinExpr(bin_expr) => {
                let lhs = bin_expr
                    .lhs()
                    .is_some_and(|lhs| self.lower_label_expr(graph, &lhs, names));
                let rhs = bin_expr
                    .rhs()
                    .is_some_and(|rhs| self.lower_label_expr(graph, &rhs, names));
                lhs || rhs
            }
            ast::Expr::PrefixExpr(prefix_expr) => {
                if let Some(operand) = prefix_expr.expr() {
                    self.lower_label_expr(graph, &operand, names);
                }
                true
            }
            ast::Expr::ParenExpr(paren_expr) => paren_expr
                .expr()
                .is_some_and(|inner| self.lower_label_expr(graph, &inner, names)),
            ast::Expr::ArrayExpr(_)
            | ast::Expr::BetweenExpr(_)
            | ast::Expr::CallExpr(_)
            | ast::Expr::CaseExpr(_)
            | ast::Expr::CastExpr(_)
            | ast::Expr::Collate(_)
            | ast::Expr::FieldExpr(_)
            | ast::Expr::IndexExpr(_)
            | ast::Expr::Literal(_)
            | ast::Expr::PostfixExpr(_)
            | ast::Expr::SliceExpr(_)
            | ast::Expr::TupleExpr(_) => true,
        }
    }

    fn element_variable_columns(
        &mut self,
        graph: TableId,
        kind: GraphElementKind,
        any_label: bool,
        names: &[Name],
    ) -> Columns {
        let elements = &self.index.graph_elements;
        let labels = &self.index.graph_labels;
        let properties = &self.index.graph_properties;
        let dropped_elements = &self.dropped_graph_elements;
        let dropped_labels = &self.dropped_graph_labels;
        let dropped_properties = &self.dropped_graph_properties;
        let dropped_tables = &self.dropped_tables;
        let label_matches = |id: GraphLabelId| {
            let label = &labels[id];
            let element = &elements[label.element];
            !dropped_labels.contains(&id)
                && element.graph == graph
                && element.kind == kind
                && (any_label || names.contains(&label.name))
                && graph_element_live(elements, dropped_elements, dropped_tables, label.element)
        };
        let complete = labels
            .iter()
            .all(|(id, label)| !label_matches(id) || elements[label.element].table.is_some());
        let property_matches = |id: GraphPropertyId, property: &GraphProperty| {
            !dropped_properties.contains(&id) && label_matches(property.label)
        };
        let range = self.index.columns.alloc_copies(
            properties
                .iter()
                .filter(|&(id, property)| {
                    property_matches(id, property)
                        && !properties
                            .iter()
                            .take_while(|(earlier, _)| *earlier != id)
                            .any(|(earlier, other)| {
                                other.name == property.name && property_matches(earlier, other)
                            })
                })
                .map(|(_, property)| property.column),
        );
        Columns { range, complete }
    }

    fn find_graph_label_in(&self, graph: TableId, name: Name) -> Option<GraphLabelId> {
        self.index
            .graph_elements
            .get(graph)
            .filter(|element| self.graph_element_live(*element))
            .flat_map(|element| self.index.graph_labels.get(element))
            .filter(|id| {
                self.index.graph_labels[*id].name == name && !self.dropped_graph_labels.contains(id)
            })
            .max()
    }

    fn lower_graph_table_columns(&mut self, list: &ast::ExprAsColumnNameList) -> Columns {
        let mut columns = Vec::with_capacity(list.expr_as_column_names().count());
        let mut explicit = vec![];
        for item in list.expr_as_column_names() {
            let expr = item.expr();
            let ty = expr.as_ref().and_then(|expr| self.lower_expr(expr));
            let (name, node) = match (item.column_name(), expr) {
                (Some(column_name), _) => {
                    explicit.push(columns.len());
                    (column_name.text(), column_name.syntax().clone())
                }
                (None, Some(ast::Expr::FieldExpr(field_expr))) => match field_expr.field() {
                    Some(field) => (field.text(), field.syntax().clone()),
                    None => continue,
                },
                (None, Some(ast::Expr::NameRef(name_ref))) => {
                    (name_ref.text(), name_ref.syntax().clone())
                }
                (
                    None,
                    Some(
                        ast::Expr::ArrayExpr(_)
                        | ast::Expr::BetweenExpr(_)
                        | ast::Expr::BinExpr(_)
                        | ast::Expr::CallExpr(_)
                        | ast::Expr::CaseExpr(_)
                        | ast::Expr::CastExpr(_)
                        | ast::Expr::Collate(_)
                        | ast::Expr::IndexExpr(_)
                        | ast::Expr::Literal(_)
                        | ast::Expr::ParenExpr(_)
                        | ast::Expr::PostfixExpr(_)
                        | ast::Expr::PrefixExpr(_)
                        | ast::Expr::SliceExpr(_)
                        | ast::Expr::TupleExpr(_),
                    )
                    | None,
                ) => continue,
            };
            columns.push(Column {
                name: Some(self.intern(&name)),
                ptr: self.ptr(&node),
                ty,
                merge: None,
            });
        }
        let range = self.index.columns.alloc_many(columns);
        let start = range.start().into_raw().into_u32() as usize;
        for offset in explicit {
            let id = nth_idx(start + offset);
            let ptr = self.index.columns[id].ptr;
            self.add_item_range(ptr.file_range(), Item::Column(id));
        }
        Columns {
            range,
            complete: true,
        }
    }

    fn drop_graph_element(&mut self, element: GraphElementId) {
        if self.dropped_graph_elements.insert(element) && self.in_transaction() {
            self.undo.push(Undo::DropGraphElement(element));
        }
    }

    fn drop_graph_label(&mut self, label: GraphLabelId) {
        if self.dropped_graph_labels.insert(label) && self.in_transaction() {
            self.undo.push(Undo::DropGraphLabel(label));
        }
    }

    fn lower_create_conversion(&mut self, create_conversion: &ast::CreateConversion) {
        let refs = self.index.references.next_idx();
        let start = self.index.conversions.next_idx();
        // TODO: report a second DEFAULT conversion for the same encoding pair
        if let Some(path) = create_conversion
            .function_name_ref()
            .and_then(|function| function.path_ref())
        {
            let int4 = self.builtin_type("int4");
            let cstring = self.builtin_type("cstring");
            let internal = self.builtin_type("internal");
            let bool = self.builtin_type("bool");
            self.lower_function_path_ref(&path, Some(&[int4, int4, cstring, internal, int4, bool]));
        }
        let Some(path) = create_conversion
            .conversion()
            .and_then(|conversion| conversion.path())
        else {
            return;
        };
        let Some(segment) = path.segment() else {
            return;
        };
        let schema = self.creation_schema(&path);
        let name = self.intern(&segment.text());
        self.alloc_conversion(create_conversion.syntax(), segment.syntax(), schema, name);
        self.depend_on_refs(
            refs,
            self.index.conversions.range_from(start),
            Object::Conversion,
        );
    }

    fn lower_alter_conversion(&mut self, alter_conversion: &ast::AlterConversion) {
        let conversion = alter_conversion
            .conversion_ref()
            .and_then(|conversion| conversion.path_ref())
            .and_then(|path| self.lower_conversion_ref(&path, IfMissing::Error));
        match alter_conversion.action() {
            Some(ast::AlterConversionAction::ConversionRenameTo(rename)) => {
                if let Some(conversion) = conversion
                    && let Some(segment) = rename
                        .conversion()
                        .and_then(|conversion| conversion.path())
                        .and_then(|path| path.segment())
                {
                    let schema = self.index.conversions[conversion].schema;
                    let name = self.intern(&segment.text());
                    self.move_conversion(
                        conversion,
                        alter_conversion.syntax(),
                        segment.syntax(),
                        schema,
                        name,
                    );
                }
            }
            Some(ast::AlterConversionAction::SetSchema(set_schema)) => {
                let Some(schema_ref) = set_schema.schema_ref() else {
                    return;
                };
                let Some(schema) = self.lower_set_schema_ref(&schema_ref) else {
                    return;
                };
                if let Some(conversion) = conversion {
                    let Conversion { name, name_ptr, .. } = self.index.conversions[conversion];
                    if self.find_conversion_in(schema, name).is_some() {
                        let error = SemaError::DuplicateInSchema {
                            object: ObjectKind::Conversion,
                            name,
                            schema,
                        };
                        self.push_error(schema_ref.syntax(), error);
                        return;
                    }
                    let moved = self.index.conversions.next_idx();
                    self.drop_conversion(conversion);
                    self.index.conversions.alloc(Conversion {
                        schema,
                        name,
                        ptr: self.ptr(alter_conversion.syntax()),
                        name_ptr,
                    });
                    self.record_moves(
                        Object::Conversion(conversion),
                        self.index.conversions.range_from(moved),
                        Object::Conversion,
                    );
                }
            }
            Some(ast::AlterConversionAction::OwnerTo(owner_to)) => self.lower_owner_to(&owner_to),
            None => {}
        }
    }

    fn lower_drop_conversion(&mut self, drop_conversion: &ast::DropConversion) {
        let if_missing = IfMissing::from_if_exists(drop_conversion.if_exists().as_ref());
        if let Some(conversion) = drop_conversion
            .conversion_ref()
            .and_then(|conversion| conversion.path_ref())
            .and_then(|path| self.lower_conversion_ref(&path, if_missing))
        {
            self.drop_conversion(conversion);
        }
    }

    fn move_conversion(
        &mut self,
        conversion: ConversionId,
        node: &SyntaxNode,
        name_node: &SyntaxNode,
        schema: Name,
        name: Name,
    ) {
        if self.find_conversion_in(schema, name).is_some() {
            self.report_duplicate(ObjectKind::Conversion, name_node, name);
            return;
        }
        let moved = self.index.conversions.next_idx();
        self.drop_conversion(conversion);
        self.alloc_conversion(node, name_node, schema, name);
        self.record_moves(
            Object::Conversion(conversion),
            self.index.conversions.range_from(moved),
            Object::Conversion,
        );
    }

    fn alloc_conversion(
        &mut self,
        node: &SyntaxNode,
        name_node: &SyntaxNode,
        schema: Name,
        name: Name,
    ) {
        if self.find_conversion_in(schema, name).is_some() {
            self.report_duplicate(ObjectKind::Conversion, name_node, name);
        }
        let id = self.index.conversions.alloc(Conversion {
            schema,
            name,
            ptr: self.ptr(node),
            name_ptr: self.ptr(name_node),
        });
        self.record_comment(node, Item::Conversion(id));
        self.add_item(name_node, Item::Conversion(id));
    }

    fn lower_conversion_ref(
        &mut self,
        path: &ast::PathRef,
        if_missing: IfMissing,
    ) -> Option<ConversionId> {
        let segment = path.segment()?;
        let name = self.intern(&segment.text());
        let schema = self.lower_schema_qualifier(path.qualifier());
        let conversion = self
            .search_path(schema, Lookup::Conversion)
            .find_map(|schema| self.find_conversion_in(schema, name));
        let resolution = match (conversion, if_missing) {
            (Some(conversion), _) => Resolution::Conversion(conversion),
            (None, IfMissing::Ignore) => Resolution::Unknown,
            (None, IfMissing::Error) => Resolution::Unresolved,
        };
        self.add_ref(
            name,
            ReferenceKind::Conversion,
            segment.syntax(),
            resolution,
        );
        conversion
    }

    fn find_conversion_in(&self, schema: Name, name: Name) -> Option<ConversionId> {
        self.index.conversions.named(schema, name).find(|id| {
            let conversion = &self.index.conversions[*id];
            self.visible(conversion.schema, conversion.ptr.file_id)
                && !self.dropped_conversions.contains(id)
        })
    }

    fn lower_create_language(&mut self, create_language: &ast::CreateLanguage) {
        let refs = self.index.references.next_idx();
        let start = self.index.languages.next_idx();
        if let Some(path) = create_language
            .handler_clause()
            .and_then(|handler| handler.function_name_ref())
            .and_then(|function| function.path_ref())
        {
            self.lower_function_path_ref(&path, Some(&[]));
        }
        if let Some(path) = create_language
            .inline_clause()
            .and_then(|inline| inline.function_name_ref())
            .and_then(|function| function.path_ref())
        {
            let internal = self.builtin_type("internal");
            self.lower_function_path_ref(&path, Some(&[internal]));
        }
        if let Some(path) = create_language
            .validator_clause()
            .and_then(|validator| validator.function_name_ref())
            .and_then(|function| function.path_ref())
        {
            let oid = self.builtin_type("oid");
            self.lower_function_path_ref(&path, Some(&[oid]));
        }
        if let Some(language) = create_language.language() {
            self.alloc_language(
                IfExists::from_or_replace(create_language.or_replace().as_ref()),
                create_language.syntax(),
                &language,
            );
        }
        self.depend_on_refs(
            refs,
            self.index.languages.range_from(start),
            Object::Language,
        );
    }

    fn lower_alter_language(&mut self, alter_language: &ast::AlterLanguage) {
        let language = alter_language
            .language_ref()
            .and_then(|language_ref| self.lower_language_ref(&language_ref, IfMissing::Error));
        match alter_language.action() {
            Some(ast::AlterLanguageAction::LanguageRenameTo(rename)) => {
                if let Some(language) = language
                    && let Some(new_language) = rename.language()
                {
                    let new_name = self.intern(&new_language.text());
                    if self.find_language(new_name).is_some() {
                        self.report_duplicate(
                            ObjectKind::Language,
                            new_language.syntax(),
                            new_name,
                        );
                        return;
                    }
                    let moved = self.index.languages.next_idx();
                    self.drop_language(language);
                    self.alloc_language(IfExists::Error, alter_language.syntax(), &new_language);
                    self.record_moves(
                        Object::Language(language),
                        self.index.languages.range_from(moved),
                        Object::Language,
                    );
                }
            }
            Some(ast::AlterLanguageAction::OwnerTo(owner_to)) => self.lower_owner_to(&owner_to),
            None => {}
        }
    }

    fn lower_drop_language(&mut self, drop_language: &ast::DropLanguage) {
        let Some(language_ref) = drop_language.language_ref() else {
            return;
        };
        let if_missing = IfMissing::from_if_exists(drop_language.if_exists().as_ref());
        if let Some(language) = self.lower_language_ref(&language_ref, if_missing) {
            self.drop_language(language);
        }
    }

    fn lower_do(&mut self, do_: &ast::Do) {
        if let Some(do_language) = do_.do_language() {
            if let Some(language_ref) = do_language.language_ref() {
                self.lower_language_ref(&language_ref, IfMissing::Error);
            }
            if let Some(literal) = do_language.literal() {
                self.lower_language_literal(&literal);
            }
        }
    }

    fn lower_func_options(&mut self, options: &ast::FuncOptionList) -> Option<SmallVec<[Name; 4]>> {
        let mut search_path = None;
        for option in options.options() {
            match option {
                ast::FuncOption::LanguageFuncOption(language) => {
                    if let Some(language_ref) = language.language_ref() {
                        self.lower_language_ref(&language_ref, IfMissing::Error);
                    }
                    if let Some(literal) = language.literal() {
                        self.lower_language_literal(&literal);
                    }
                }
                ast::FuncOption::SupportFuncOption(support) => {
                    if let Some(path) = support
                        .function_name_ref()
                        .and_then(|function| function.path_ref())
                    {
                        let internal = self.builtin_type("internal");
                        self.lower_function_path_ref(&path, Some(&[internal]));
                    }
                }
                ast::FuncOption::TransformFuncOption(transform) => {
                    for ty in transform
                        .transform_for_types()
                        .filter_map(|transform| transform.ty())
                    {
                        self.lower_type(&ty);
                    }
                }
                ast::FuncOption::AsFuncOption(_)
                | ast::FuncOption::CalledOnNullInputFuncOption(_)
                | ast::FuncOption::CostFuncOption(_)
                | ast::FuncOption::LeakproofFuncOption(_)
                | ast::FuncOption::NotLeakproofFuncOption(_)
                | ast::FuncOption::ParallelFuncOption(_)
                | ast::FuncOption::ResetFuncOption(_)
                | ast::FuncOption::ReturnsNullOnNullInputFuncOption(_)
                | ast::FuncOption::RowsFuncOption(_)
                | ast::FuncOption::SecurityDefinerFuncOption(_)
                | ast::FuncOption::SecurityInvokerFuncOption(_)
                | ast::FuncOption::StrictFuncOption(_)
                | ast::FuncOption::WindowFuncOption(_)
                | ast::FuncOption::VolatilityFuncOption(_) => {}
                ast::FuncOption::SetFuncOption(set) => {
                    if let Some(param) = set.set_config_param()
                        && let Some(path) = self.lower_set_config_param(&param)
                    {
                        search_path = Some(path);
                    }
                }
            }
        }
        search_path
    }

    fn alloc_language(
        &mut self,
        if_exists: IfExists,
        node: &SyntaxNode,
        name_node: &impl NameLike,
    ) {
        let name = self.intern(&name_node.text());
        if let Some(existing) = self.find_language(name) {
            match if_exists {
                IfExists::Error | IfExists::Ignore => {
                    self.report_duplicate(ObjectKind::Language, name_node.syntax(), name);
                }
                IfExists::Replace => self.drop_language(existing),
            }
        }
        let id = self.index.languages.alloc(Language {
            name,
            ptr: self.ptr(node),
            name_ptr: self.ptr(name_node.syntax()),
        });
        self.record_comment(node, Item::Language(id));
        self.add_item(name_node.syntax(), Item::Language(id));
    }

    fn lower_language_ref(
        &mut self,
        language_ref: &ast::LanguageRef,
        if_missing: IfMissing,
    ) -> Option<LanguageId> {
        let name = self.intern(&language_ref.text());
        self.add_language_ref(name, language_ref.syntax(), if_missing)
    }

    fn lower_language_literal(&mut self, literal: &ast::Literal) {
        if let Some(name) = literal.string_value() {
            let name = self.intern(&name);
            self.add_language_ref(name, literal.syntax(), IfMissing::Error);
        }
    }

    fn add_language_ref(
        &mut self,
        name: Name,
        node: &SyntaxNode,
        if_missing: IfMissing,
    ) -> Option<LanguageId> {
        let language = self.find_language(name);
        let resolution = match (language, if_missing) {
            (Some(language), _) => Resolution::Language(language),
            (None, IfMissing::Ignore) => Resolution::Unknown,
            (None, IfMissing::Error) => Resolution::Unresolved,
        };
        self.add_ref(name, ReferenceKind::Language, node, resolution);
        language
    }

    fn find_language(&self, name: Name) -> Option<LanguageId> {
        self.index
            .languages
            .get(name)
            .find(|id| !self.dropped_languages.contains(id))
    }

    fn lower_create_event_trigger(&mut self, create_event_trigger: &ast::CreateEventTrigger) {
        let refs = self.index.references.next_idx();
        let start = self.index.event_triggers.next_idx();
        if let Some(callee) = create_event_trigger
            .call_expr()
            .and_then(|call| call.expr())
        {
            self.lower_trigger_function(&callee, TriggerFunctionKind::EventTrigger);
        }
        if let Some(event_trigger) = create_event_trigger.event_trigger() {
            self.alloc_event_trigger(create_event_trigger.syntax(), &event_trigger);
        }
        self.depend_on_refs(
            refs,
            self.index.event_triggers.range_from(start),
            Object::EventTrigger,
        );
    }

    fn lower_alter_event_trigger(&mut self, alter_event_trigger: &ast::AlterEventTrigger) {
        let event_trigger = alter_event_trigger
            .event_trigger_ref()
            .and_then(|event_trigger_ref| {
                self.lower_event_trigger_ref(&event_trigger_ref, IfMissing::Error)
            });
        match alter_event_trigger.action() {
            Some(ast::AlterEventTriggerAction::EventTriggerRenameTo(rename)) => {
                if let Some(event_trigger) = event_trigger
                    && let Some(new_event_trigger) = rename.event_trigger()
                {
                    let new_name = self.intern(&new_event_trigger.text());
                    if self.find_event_trigger(new_name).is_some() {
                        self.report_duplicate(
                            ObjectKind::EventTrigger,
                            new_event_trigger.syntax(),
                            new_name,
                        );
                        return;
                    }
                    let moved = self.index.event_triggers.next_idx();
                    self.drop_event_trigger(event_trigger);
                    self.alloc_event_trigger(alter_event_trigger.syntax(), &new_event_trigger);
                    self.record_moves(
                        Object::EventTrigger(event_trigger),
                        self.index.event_triggers.range_from(moved),
                        Object::EventTrigger,
                    );
                }
            }
            Some(ast::AlterEventTriggerAction::OwnerTo(owner_to)) => self.lower_owner_to(&owner_to),
            Some(
                ast::AlterEventTriggerAction::Disable(_)
                | ast::AlterEventTriggerAction::Enable(_)
                | ast::AlterEventTriggerAction::EnableAlways(_)
                | ast::AlterEventTriggerAction::EnableReplica(_),
            )
            | None => {}
        }
    }

    fn lower_drop_event_trigger(&mut self, drop_event_trigger: &ast::DropEventTrigger) {
        let Some(event_trigger_ref) = drop_event_trigger.event_trigger_ref() else {
            return;
        };
        let if_missing = IfMissing::from_if_exists(drop_event_trigger.if_exists().as_ref());
        if let Some(event_trigger) = self.lower_event_trigger_ref(&event_trigger_ref, if_missing) {
            self.drop_event_trigger(event_trigger);
        }
    }

    fn alloc_event_trigger(&mut self, node: &SyntaxNode, name_node: &impl NameLike) {
        let name = self.intern(&name_node.text());
        if self.find_event_trigger(name).is_some() {
            self.report_duplicate(ObjectKind::EventTrigger, name_node.syntax(), name);
        }
        let id = self.index.event_triggers.alloc(EventTrigger {
            name,
            ptr: self.ptr(node),
            name_ptr: self.ptr(name_node.syntax()),
        });
        self.record_comment(node, Item::EventTrigger(id));
        self.add_item(name_node.syntax(), Item::EventTrigger(id));
    }

    fn lower_event_trigger_ref(
        &mut self,
        event_trigger_ref: &ast::EventTriggerRef,
        if_missing: IfMissing,
    ) -> Option<EventTriggerId> {
        let name = self.intern(&event_trigger_ref.text());
        let event_trigger = self.find_event_trigger(name);
        let resolution = match (event_trigger, if_missing) {
            (Some(event_trigger), _) => Resolution::EventTrigger(event_trigger),
            (None, IfMissing::Ignore) => Resolution::Unknown,
            (None, IfMissing::Error) => Resolution::Unresolved,
        };
        self.add_ref(
            name,
            ReferenceKind::EventTrigger,
            event_trigger_ref.syntax(),
            resolution,
        );
        event_trigger
    }

    fn find_event_trigger(&self, name: Name) -> Option<EventTriggerId> {
        self.index
            .event_triggers
            .get(name)
            .find(|id| !self.dropped_event_triggers.contains(id))
    }

    fn lower_create_schema(&mut self, create_schema: &ast::CreateSchema) {
        let if_exists = IfExists::from_if_not_exists(create_schema.if_not_exists().as_ref());
        let name = match create_schema.create_schema_target() {
            Some(ast::CreateSchemaTarget::NamedSchema(named)) => {
                let name = named.schema().map(|schema| {
                    let name = self.intern(&schema.text());
                    self.alloc_schema(if_exists, create_schema, schema.syntax(), name);
                    name
                });
                if let Some(role_ref) = named.role_ref() {
                    self.lower_role_ref(&role_ref, IfMissing::Error);
                }
                name
            }
            Some(ast::CreateSchemaTarget::AuthorizationSchema(authorization)) => {
                authorization.role().map(|role| {
                    if role.ident_token().is_some() {
                        let name = self.intern(&role.text());
                        self.alloc_schema(if_exists, create_schema, role.syntax(), name);
                        self.add_role_ref(name, role.syntax(), IfMissing::Error);
                        return name;
                    }
                    let name = if role.session_user_token().is_some() {
                        self.intern("session_user")
                    } else {
                        self.intern("current_user")
                    };
                    self.alloc_schema(if_exists, create_schema, role.syntax(), name);
                    name
                })
            }
            None => None,
        };
        let Some(name) = name else {
            return;
        };
        let Some(first) = create_schema.schema_elements().next() else {
            return;
        };
        let range = TextRange::new(
            first.syntax().text_range().start(),
            create_schema.syntax().text_range().end(),
        );
        let mut search_path = self.search_path.clone();
        search_path.insert(0, name);
        self.with_search_path(&search_path, range, |this| {
            for element in create_schema.schema_elements() {
                this.lower_stmt(&element.into());
            }
        });
    }

    fn lower_alter_schema(&mut self, alter_schema: &ast::AlterSchema) {
        let schema = alter_schema
            .schema_ref()
            .and_then(|schema_ref| self.lower_schema_ref_id(&schema_ref, IfMissing::Error));
        match alter_schema.action() {
            Some(ast::AlterSchemaAction::SchemaRenameTo(rename)) => {
                if let Some(schema) = schema
                    && let Some(name_node) = rename.schema()
                {
                    self.rename_schema(schema, &name_node);
                }
            }
            Some(ast::AlterSchemaAction::OwnerTo(owner_to)) => self.lower_owner_to(&owner_to),
            None => {}
        }
    }

    fn rename_schema(&mut self, schema: SchemaId, name_node: &impl NameLike) {
        let name = self.intern_schema(&name_node.text());
        if self.find_schema(name).is_some() {
            self.report_duplicate(ObjectKind::Schema, name_node.syntax(), name);
            return;
        }
        let Schema {
            name: from, ptr, ..
        } = self.index.schemas[schema];
        self.drop_schema(schema);
        let id = self.index.schemas.alloc(Schema {
            name,
            ptr,
            name_ptr: self.ptr(name_node.syntax()),
        });
        self.copy_comment(Item::Schema(schema), Item::Schema(id));
        self.add_item(name_node.syntax(), Item::Schema(id));
        rename_schema_objects(&mut self.index, from, name);
        if self.in_transaction() {
            self.undo.push(Undo::RenameSchema(from, name));
        }
    }

    fn lower_drop_schema(&mut self, drop_schema: &ast::DropSchema) {
        let if_missing = IfMissing::from_if_exists(drop_schema.if_exists().as_ref());
        let cascade = matches!(
            drop_schema.drop_behavior(),
            Some(ast::DropBehavior::Cascade(_))
        );
        for schema_ref in drop_schema.schema_refs() {
            let Some(schema) = self.lower_schema_ref_id(&schema_ref, if_missing) else {
                continue;
            };
            let name = self.index.schemas[schema].name;
            if name == self.pg_catalog {
                self.push_error(
                    schema_ref.syntax(),
                    SemaError::RequiredBySystem { schema: name },
                );
                continue;
            }
            if cascade {
                self.drop_schema_with_contents(schema);
            } else if self
                .indexed_schema_objects(name)
                .any(|object| self.in_schema(object, name))
            {
                self.push_error(
                    schema_ref.syntax(),
                    SemaError::DependentObjects {
                        object: ObjectKind::Schema,
                        name,
                    },
                );
            } else {
                self.drop_schema(schema);
            }
        }
    }

    fn drop_schema_with_contents(&mut self, schema: SchemaId) {
        let name = self.index.schemas[schema].name;
        let indexed: SmallVec<[Object; 8]> = self.indexed_schema_objects(name).collect();
        for object in indexed {
            self.drop_object(object);
        }
        self.drop_schema(schema);
    }

    fn indexed_schema_objects(&self, schema: Name) -> impl Iterator<Item = Object> + '_ {
        let index = &self.index;
        index
            .tables
            .in_schema(schema)
            .rev()
            .map(Object::Table)
            .chain(index.types.in_schema(schema).rev().map(Object::Type))
            .chain(
                index
                    .functions
                    .in_schema(schema)
                    .rev()
                    .map(Object::Function),
            )
            .chain(index.indexes.in_schema(schema).rev().map(Object::Index))
            .chain(
                index
                    .operators
                    .in_schema(schema)
                    .rev()
                    .map(Object::Operator),
            )
            .chain(
                index
                    .statistics
                    .in_schema(schema)
                    .rev()
                    .map(Object::Statistics),
            )
            .chain(
                index
                    .collations
                    .in_schema(schema)
                    .rev()
                    .map(Object::Collation),
            )
            .chain(
                index
                    .conversions
                    .in_schema(schema)
                    .rev()
                    .map(Object::Conversion),
            )
            .chain(
                index
                    .operator_families
                    .in_schema(schema)
                    .rev()
                    .map(Object::OperatorFamily),
            )
            .chain(
                index
                    .operator_classes
                    .in_schema(schema)
                    .rev()
                    .map(Object::OperatorClass),
            )
            .chain(
                index
                    .text_search
                    .in_schema(schema)
                    .rev()
                    .map(Object::TextSearch),
            )
            .chain(
                index
                    .extensions
                    .in_schema(schema)
                    .rev()
                    .map(Object::Extension),
            )
    }

    fn in_schema(&self, object: Object, schema: Name) -> bool {
        let index = &self.index;
        let object_schema = match object {
            Object::Table(table) => index.tables[table].schema,
            Object::Index(idx) => index.indexes[idx].schema,
            Object::Type(ty) => index.types[ty].schema,
            Object::Function(function) => index.functions[function].schema,
            Object::Operator(operator) => index.operators[operator].schema,
            Object::Collation(collation) => index.collations[collation].schema,
            Object::Conversion(conversion) => index.conversions[conversion].schema,
            Object::OperatorFamily(family) => index.operator_families[family].schema,
            Object::OperatorClass(class) => index.operator_classes[class].schema,
            Object::Statistics(statistics) => index.statistics[statistics].schema,
            Object::Extension(extension) => index.extensions[extension].schema,
            Object::TextSearch(text_search) => index.text_search[text_search].schema,
            Object::Trigger(_)
            | Object::EventTrigger(_)
            | Object::Constraint(_)
            | Object::Policy(_)
            | Object::Rule(_)
            | Object::Cast(_)
            | Object::Transform(_)
            | Object::AccessMethod(_)
            | Object::Language(_)
            | Object::ForeignDataWrapper(_)
            | Object::ColumnExpr(_)
            | Object::OperatorFamilyMember(_)
            | Object::Schema(_)
            | Object::Server(_)
            | Object::Publication(_) => return false,
        };
        object_schema == schema && !self.is_dropped(object)
    }

    fn lower_schema_ref_id(
        &mut self,
        schema_ref: &ast::SchemaRef,
        if_missing: IfMissing,
    ) -> Option<SchemaId> {
        let name = self.intern_schema(&schema_ref.text());
        let schema = self.find_schema(name);
        let resolution = match schema {
            Some(schema) => Resolution::Schema(schema),
            None => match if_missing {
                IfMissing::Ignore => Resolution::Unknown,
                IfMissing::Error => Resolution::Unresolved,
            },
        };
        self.add_ref(name, ReferenceKind::Schema, schema_ref.syntax(), resolution);
        schema
    }

    fn lower_create_tablespace(&mut self, create_tablespace: &ast::CreateTablespace) {
        if let Some(role_ref) = create_tablespace
            .tablespace_owner()
            .and_then(|owner| owner.role_ref())
        {
            self.lower_role_ref(&role_ref, IfMissing::Error);
        }
        let Some(tablespace) = create_tablespace.tablespace() else {
            return;
        };
        self.alloc_tablespace(create_tablespace, &tablespace);
    }

    fn lower_alter_tablespace(&mut self, alter_tablespace: &ast::AlterTablespace) {
        let tablespace = alter_tablespace
            .tablespace_ref()
            .and_then(|tablespace| self.lower_tablespace_ref(&tablespace, IfMissing::Error));
        match alter_tablespace.action() {
            Some(ast::AlterTablespaceAction::OwnerTo(owner_to)) => self.lower_owner_to(&owner_to),
            Some(ast::AlterTablespaceAction::TablespaceRenameTo(rename)) => {
                if let Some(tablespace) = tablespace
                    && let Some(new_name) = rename.tablespace()
                {
                    self.rename_tablespace(tablespace, &new_name);
                }
            }
            Some(
                ast::AlterTablespaceAction::ResetOptions(_)
                | ast::AlterTablespaceAction::SetOptions(_),
            )
            | None => {}
        }
    }

    fn lower_drop_tablespace(&mut self, drop_tablespace: &ast::DropTablespace) {
        let Some(tablespace) = drop_tablespace.tablespace_ref() else {
            return;
        };
        let if_missing = IfMissing::from_if_exists(drop_tablespace.if_exists().as_ref());
        if let Some(id) = self.lower_tablespace_ref(&tablespace, if_missing) {
            self.drop_tablespace(id);
        }
    }

    fn drop_tablespace(&mut self, tablespace: TablespaceId) {
        if self.dropped_tablespaces.insert(tablespace) && self.in_transaction() {
            self.undo.push(Undo::DropTablespace(tablespace));
        }
    }

    fn rename_tablespace(&mut self, tablespace: TablespaceId, new_name: &ast::Tablespace) {
        let name = self.intern(&new_name.text());
        if self.find_tablespace(name).is_some() {
            self.report_duplicate(ObjectKind::Tablespace, new_name.syntax(), name);
            return;
        }
        let ptr = self.index.tablespaces[tablespace].ptr;
        self.drop_tablespace(tablespace);
        let id = self.index.tablespaces.alloc(Tablespace {
            name,
            ptr,
            name_ptr: self.ast_ptr(new_name),
        });
        self.copy_comment(Item::Tablespace(tablespace), Item::Tablespace(id));
        self.add_item(new_name.syntax(), Item::Tablespace(id));
    }

    fn lower_all_in_tablespace(&mut self, all_in_tablespace: &ast::AllInTablespace) {
        if let Some(list) = all_in_tablespace
            .owned_by_roles()
            .and_then(|owned_by| owned_by.role_ref_list())
        {
            self.lower_role_ref_list(&list, IfMissing::Error);
        }
        if let Some(tablespace) = all_in_tablespace.tablespace_ref() {
            self.lower_tablespace_ref(&tablespace, IfMissing::Error);
        }
        if let Some(set_tablespace) = all_in_tablespace.set_tablespace() {
            self.lower_set_tablespace(&set_tablespace);
        }
    }

    fn lower_set_tablespace(&mut self, set_tablespace: &ast::SetTablespace) {
        if let Some(tablespace) = set_tablespace.tablespace_ref() {
            self.lower_tablespace_ref(&tablespace, IfMissing::Error);
        }
    }

    fn lower_tablespace_clause(&mut self, clause: &ast::TablespaceClause) {
        if let Some(tablespace) = clause.tablespace_ref() {
            self.lower_tablespace_ref(&tablespace, IfMissing::Error);
        }
    }

    fn lower_constraint_index_tablespace(&mut self, clause: &ast::ConstraintIndexTablespace) {
        if let Some(tablespace) = clause.tablespace_ref() {
            self.lower_tablespace_ref(&tablespace, IfMissing::Error);
        }
    }

    fn lower_tablespace_ref(
        &mut self,
        tablespace: &ast::TablespaceRef,
        if_missing: IfMissing,
    ) -> Option<TablespaceId> {
        let name = self.intern(&tablespace.text());
        let id = self.find_tablespace(name);
        let resolution = match id {
            Some(id) => Resolution::Tablespace(id),
            None => match if_missing {
                IfMissing::Ignore => Resolution::Unknown,
                IfMissing::Error => Resolution::Unresolved,
            },
        };
        self.add_ref(
            name,
            ReferenceKind::Tablespace,
            tablespace.syntax(),
            resolution,
        );
        id
    }

    fn lower_create_database(&mut self, create_database: &ast::CreateDatabase) {
        if let Some(options) = create_database.database_option_list() {
            self.lower_database_options(&options);
        }
        let Some(database) = create_database.database() else {
            return;
        };
        self.alloc_database(create_database, &database);
    }

    fn lower_alter_database(&mut self, alter_database: &ast::AlterDatabase) {
        let database = alter_database
            .database_ref()
            .and_then(|database| self.lower_database_ref(&database, IfMissing::Error));
        match alter_database.action() {
            Some(ast::AlterDatabaseAction::DatabaseOptionList(options)) => {
                self.lower_database_options(&options);
            }
            Some(ast::AlterDatabaseAction::SetTablespace(set_tablespace)) => {
                self.lower_set_tablespace(&set_tablespace);
            }
            Some(ast::AlterDatabaseAction::OwnerTo(owner_to)) => self.lower_owner_to(&owner_to),
            Some(ast::AlterDatabaseAction::DatabaseRenameTo(rename)) => {
                if let Some(database) = database
                    && let Some(new_name) = rename.database()
                {
                    self.rename_database(database, &new_name);
                }
            }
            Some(ast::AlterDatabaseAction::SetConfigParam(param)) => {
                self.lower_set_config_param(&param);
            }
            Some(
                ast::AlterDatabaseAction::RefreshCollationVersion(_)
                | ast::AlterDatabaseAction::ResetConfigParam(_),
            )
            | None => {}
        }
    }

    fn lower_drop_database(&mut self, drop_database: &ast::DropDatabase) {
        let Some(database) = drop_database.database_ref() else {
            return;
        };
        let if_missing = IfMissing::from_if_exists(drop_database.if_exists().as_ref());
        if let Some(id) = self.lower_database_ref(&database, if_missing) {
            self.drop_database(id);
        }
    }

    fn drop_database(&mut self, database: DatabaseId) {
        if self.dropped_databases.insert(database) && self.in_transaction() {
            self.undo.push(Undo::DropDatabase(database));
        }
    }

    fn rename_database(&mut self, database: DatabaseId, new_name: &ast::Database) {
        let name = self.intern(&new_name.text());
        if self.find_database(name).is_some() {
            self.report_duplicate(ObjectKind::Database, new_name.syntax(), name);
            return;
        }
        let ptr = self.index.databases[database].ptr;
        self.drop_database(database);
        let id = self.index.databases.alloc(Database {
            name,
            ptr,
            name_ptr: self.ast_ptr(new_name),
        });
        self.copy_comment(Item::Database(database), Item::Database(id));
        self.add_item(new_name.syntax(), Item::Database(id));
    }

    fn lower_create_extension(&mut self, create_extension: &ast::CreateExtension) {
        let schema = match create_extension
            .extension_schema()
            .and_then(|schema| schema.schema_ref())
        {
            Some(schema) => self.lower_schema_ref(schema.syntax(), &schema.text()),
            None => self.default_creation_schema(),
        };
        let Some(extension) = create_extension.extension() else {
            return;
        };
        let name = self.intern(&extension.text());
        let duplicate = self.find_extension(name).is_some();
        let Some(id) = self.alloc_extension(
            IfExists::from_if_not_exists(create_extension.if_not_exists().as_ref()),
            create_extension,
            &extension,
            schema,
        ) else {
            return;
        };
        if duplicate {
            return;
        }
        if let Some(bundled) = BundledExtension::from_name(self.index.names.resolve(name)) {
            self.lower_extension_script(id, bundled, schema);
        }
    }

    fn lower_extension_script(
        &mut self,
        extension: ExtensionId,
        bundled: BundledExtension,
        schema: Name,
    ) {
        let file_id = bundled.file_id().into();
        let snapshot = self.snapshot();
        let tree = bundled.parse().tree();
        let outer_file = std::mem::replace(&mut self.file_id, file_id);
        self.with_search_path(&[schema], tree.syntax().text_range(), |this| {
            for stmt in tree.stmts() {
                this.lower_stmt(&stmt);
            }
        });
        self.file_id = outer_file;
        let members = self.objects_since(&snapshot);
        for member in members {
            self.dependencies
                .add_dependency(member, Object::Extension(extension));
        }
    }

    fn objects_since(&self, snapshot: &Snapshot) -> impl Iterator<Item = Object> + use<> {
        let index = &self.index;
        index
            .tables
            .range_from(snapshot.tables)
            .iter()
            .map(Object::Table)
            .chain(
                index
                    .types
                    .range_from(snapshot.types)
                    .iter()
                    .map(Object::Type),
            )
            .chain(
                index
                    .functions
                    .range_from(snapshot.functions)
                    .iter()
                    .map(Object::Function),
            )
            .chain(
                index
                    .indexes
                    .range_from(snapshot.indexes)
                    .iter()
                    .map(Object::Index),
            )
            .chain(
                index
                    .constraints
                    .range_from(snapshot.constraints)
                    .iter()
                    .map(Object::Constraint),
            )
            .chain(
                index
                    .triggers
                    .range_from(snapshot.triggers)
                    .iter()
                    .map(Object::Trigger),
            )
            .chain(
                index
                    .event_triggers
                    .range_from(snapshot.event_triggers)
                    .iter()
                    .map(Object::EventTrigger),
            )
            .chain(
                index
                    .policies
                    .range_from(snapshot.policies)
                    .iter()
                    .map(Object::Policy),
            )
            .chain(
                index
                    .rules
                    .range_from(snapshot.rules)
                    .iter()
                    .map(Object::Rule),
            )
            .chain(
                index
                    .casts
                    .range_from(snapshot.casts)
                    .iter()
                    .map(Object::Cast),
            )
            .chain(
                index
                    .transforms
                    .range_from(snapshot.transforms)
                    .iter()
                    .map(Object::Transform),
            )
            .chain(
                index
                    .operators
                    .range_from(snapshot.operators)
                    .iter()
                    .map(Object::Operator),
            )
            .chain(
                index
                    .access_methods
                    .range_from(snapshot.access_methods)
                    .iter()
                    .map(Object::AccessMethod),
            )
            .chain(
                index
                    .languages
                    .range_from(snapshot.languages)
                    .iter()
                    .map(Object::Language),
            )
            .chain(
                index
                    .foreign_data_wrappers
                    .range_from(snapshot.foreign_data_wrappers)
                    .iter()
                    .map(Object::ForeignDataWrapper),
            )
            .chain(
                index
                    .conversions
                    .range_from(snapshot.conversions)
                    .iter()
                    .map(Object::Conversion),
            )
            .chain(
                index
                    .text_search
                    .range_from(snapshot.text_search)
                    .iter()
                    .map(Object::TextSearch),
            )
            .chain(
                index
                    .collations
                    .range_from(snapshot.collations)
                    .iter()
                    .map(Object::Collation),
            )
            .chain(
                index
                    .operator_families
                    .range_from(snapshot.operator_families)
                    .iter()
                    .map(Object::OperatorFamily),
            )
            .chain(
                index
                    .operator_classes
                    .range_from(snapshot.operator_classes)
                    .iter()
                    .map(Object::OperatorClass),
            )
            .chain(
                index
                    .statistics
                    .range_from(snapshot.statistics)
                    .iter()
                    .map(Object::Statistics),
            )
            .chain(
                index
                    .extensions
                    .range_from(snapshot.extensions)
                    .iter()
                    .map(Object::Extension),
            )
            .chain(
                index
                    .schemas
                    .range_from(snapshot.schemas)
                    .iter()
                    .map(Object::Schema),
            )
            .chain(
                index
                    .servers
                    .range_from(snapshot.servers)
                    .iter()
                    .map(Object::Server),
            )
            .chain(
                index
                    .publications
                    .range_from(snapshot.publications)
                    .iter()
                    .map(Object::Publication),
            )
    }

    fn lower_alter_extension(&mut self, alter_extension: &ast::AlterExtension) {
        let extension = alter_extension
            .extension_ref()
            .and_then(|extension| self.lower_extension_ref(&extension, IfMissing::Error));
        match alter_extension.action() {
            Some(ast::AlterExtensionAction::SetSchema(set_schema)) => {
                if let Some(schema_ref) = set_schema.schema_ref()
                    && let Some(schema) = self.lower_set_schema_ref(&schema_ref)
                    && let Some(extension) = extension
                {
                    self.move_extension(extension, schema);
                }
            }
            Some(ast::AlterExtensionAction::AlterExtensionAdd(add)) => {
                if let Some(member) = add.extension_member_object()
                    && let Some(member) = self.lower_extension_member(member)
                    && let Some(extension) = extension
                {
                    self.dependencies
                        .add_dependency(member, Object::Extension(extension));
                }
            }
            Some(ast::AlterExtensionAction::AlterExtensionDrop(drop)) => {
                if let Some(member) = drop.extension_member_object()
                    && let Some(member) = self.lower_extension_member(member)
                    && let Some(extension) = extension
                {
                    self.remove_dependency(member, Object::Extension(extension));
                }
            }
            Some(ast::AlterExtensionAction::AlterExtensionUpdate(_)) | None => {}
        }
    }

    fn lower_extension_member(&mut self, member: ast::ExtensionMemberObject) -> Option<Object> {
        let object = match member {
            ast::ExtensionMemberObject::ObjectAccessMethod(object) => {
                ast::CommentObject::ObjectAccessMethod(object)
            }
            ast::ExtensionMemberObject::ObjectAggregate(object) => {
                ast::CommentObject::ObjectAggregate(object)
            }
            ast::ExtensionMemberObject::ObjectCast(object) => {
                ast::CommentObject::ObjectCast(object)
            }
            ast::ExtensionMemberObject::ObjectCollation(object) => {
                ast::CommentObject::ObjectCollation(object)
            }
            ast::ExtensionMemberObject::ObjectConversion(object) => {
                ast::CommentObject::ObjectConversion(object)
            }
            ast::ExtensionMemberObject::ObjectDatabase(object) => {
                ast::CommentObject::ObjectDatabase(object)
            }
            ast::ExtensionMemberObject::ObjectDomain(object) => {
                ast::CommentObject::ObjectDomain(object)
            }
            ast::ExtensionMemberObject::ObjectEventTrigger(object) => {
                ast::CommentObject::ObjectEventTrigger(object)
            }
            ast::ExtensionMemberObject::ObjectExtension(object) => {
                ast::CommentObject::ObjectExtension(object)
            }
            ast::ExtensionMemberObject::ObjectForeignDataWrapper(object) => {
                ast::CommentObject::ObjectForeignDataWrapper(object)
            }
            ast::ExtensionMemberObject::ObjectForeignTable(object) => {
                ast::CommentObject::ObjectForeignTable(object)
            }
            ast::ExtensionMemberObject::ObjectFunction(object) => {
                ast::CommentObject::ObjectFunction(object)
            }
            ast::ExtensionMemberObject::ObjectIndex(object) => {
                ast::CommentObject::ObjectIndex(object)
            }
            ast::ExtensionMemberObject::ObjectLanguage(object) => {
                ast::CommentObject::ObjectLanguage(object)
            }
            ast::ExtensionMemberObject::ObjectMaterializedView(object) => {
                ast::CommentObject::ObjectMaterializedView(object)
            }
            ast::ExtensionMemberObject::ObjectOperator(object) => {
                ast::CommentObject::ObjectOperator(object)
            }
            ast::ExtensionMemberObject::ObjectOperatorClass(object) => {
                ast::CommentObject::ObjectOperatorClass(object)
            }
            ast::ExtensionMemberObject::ObjectOperatorFamily(object) => {
                ast::CommentObject::ObjectOperatorFamily(object)
            }
            ast::ExtensionMemberObject::ObjectProcedure(object) => {
                ast::CommentObject::ObjectProcedure(object)
            }
            ast::ExtensionMemberObject::ObjectPublication(object) => {
                ast::CommentObject::ObjectPublication(object)
            }
            ast::ExtensionMemberObject::ObjectRole(object) => {
                ast::CommentObject::ObjectRole(object)
            }
            ast::ExtensionMemberObject::ObjectRoutine(object) => {
                ast::CommentObject::ObjectRoutine(object)
            }
            ast::ExtensionMemberObject::ObjectSchema(object) => {
                ast::CommentObject::ObjectSchema(object)
            }
            ast::ExtensionMemberObject::ObjectSequence(object) => {
                ast::CommentObject::ObjectSequence(object)
            }
            ast::ExtensionMemberObject::ObjectServer(object) => {
                ast::CommentObject::ObjectServer(object)
            }
            ast::ExtensionMemberObject::ObjectStatistics(object) => {
                ast::CommentObject::ObjectStatistics(object)
            }
            ast::ExtensionMemberObject::ObjectSubscription(object) => {
                ast::CommentObject::ObjectSubscription(object)
            }
            ast::ExtensionMemberObject::ObjectTable(object) => {
                ast::CommentObject::ObjectTable(object)
            }
            ast::ExtensionMemberObject::ObjectTablespace(object) => {
                ast::CommentObject::ObjectTablespace(object)
            }
            ast::ExtensionMemberObject::ObjectTextSearchConfig(object) => {
                ast::CommentObject::ObjectTextSearchConfig(object)
            }
            ast::ExtensionMemberObject::ObjectTextSearchDict(object) => {
                ast::CommentObject::ObjectTextSearchDict(object)
            }
            ast::ExtensionMemberObject::ObjectTextSearchParser(object) => {
                ast::CommentObject::ObjectTextSearchParser(object)
            }
            ast::ExtensionMemberObject::ObjectTextSearchTemplate(object) => {
                ast::CommentObject::ObjectTextSearchTemplate(object)
            }
            ast::ExtensionMemberObject::ObjectTransform(object) => {
                ast::CommentObject::ObjectTransform(object)
            }
            ast::ExtensionMemberObject::ObjectType(object) => {
                ast::CommentObject::ObjectType(object)
            }
            ast::ExtensionMemberObject::ObjectView(object) => {
                ast::CommentObject::ObjectView(object)
            }
        };
        self.lower_object_ref(&object)
    }

    fn lower_drop_extension(&mut self, drop_extension: &ast::DropExtension) {
        let if_missing = IfMissing::from_if_exists(drop_extension.if_exists().as_ref());
        for extension in drop_extension.extension_refs() {
            if let Some(id) = self.lower_extension_ref(&extension, if_missing) {
                self.drop_extension_with_dependents(id);
            }
        }
    }

    fn lower_extension_ref(
        &mut self,
        extension: &ast::ExtensionRef,
        if_missing: IfMissing,
    ) -> Option<ExtensionId> {
        let name = self.intern(&extension.text());
        let id = self.find_extension(name);
        let resolution = match id {
            Some(id) => Resolution::Extension(id),
            None => match if_missing {
                IfMissing::Ignore => Resolution::Unknown,
                IfMissing::Error => Resolution::Unresolved,
            },
        };
        self.add_ref(
            name,
            ReferenceKind::Extension,
            extension.syntax(),
            resolution,
        );
        id
    }

    fn find_extension(&self, name: Name) -> Option<ExtensionId> {
        self.index
            .extensions
            .named(name)
            .find(|id| !self.dropped_extensions.contains(id))
    }

    fn lower_create_operator(&mut self, create_operator: &ast::CreateOperator) {
        let refs = self.index.references.next_idx();
        let start = self.index.operators.next_idx();
        let Some(op) = create_operator.op() else {
            return;
        };
        let path = op.path_ref();
        let schema = self
            .lower_schema_qualifier(path.as_ref().and_then(|path| path.qualifier()))
            .unwrap_or_else(|| self.default_creation_schema());
        let mut left = Operand::None;
        let mut right = Operand::None;
        for option in create_operator
            .attribute_list()
            .iter()
            .flat_map(|list| list.attribute_options())
        {
            let Some(name) = attribute_name(&option) else {
                continue;
            };
            let operand = match name.as_str() {
                "leftarg" => &mut left,
                "rightarg" => &mut right,
                _ => continue,
            };
            *operand = match option.attribute_value().and_then(|value| value.ty()) {
                Some(ty) => self.lower_type(&ty).map_or(Operand::Unknown, Operand::Ty),
                None => Operand::Unknown,
            };
        }
        let range = op.operator_range();
        let text = op.operator_text();
        let name = self.intern(&text);
        if self
            .find_operator(Some(schema), name, left, right)
            .is_some()
        {
            self.report_duplicate(ObjectKind::Operator, op.syntax(), name);
        }
        let id = self.index.operators.alloc(Operator {
            schema,
            name,
            left,
            right,
            function: None,
            ptr: self.ast_ptr(create_operator),
            name_range: InIrFile::new(self.file_id, range),
        });
        self.record_comment(create_operator.syntax(), Item::Operator(id));
        self.add_item_range(InIrFile::new(self.file_id, range), Item::Operator(id));
        if let Some(attributes) = create_operator.attribute_list() {
            let function = self.lower_operator_attributes(&attributes, left, right);
            self.index.operators.set_function(id, function);
        }
        self.depend_on_refs(
            refs,
            self.index.operators.range_from(start),
            Object::Operator,
        );
    }

    fn lower_alter_operator(&mut self, alter_operator: &ast::AlterOperator) {
        let Some(op_sig) = alter_operator.op_sig() else {
            return;
        };
        let Operands { left, right } = self.lower_op_sig_operands(&op_sig, IfMissing::Error);
        let operator = op_sig
            .op()
            .and_then(|op| self.lower_operator_ref(&op, left, right, IfMissing::Error));
        match alter_operator.action() {
            Some(ast::AlterOperatorAction::SetOptions(set_options)) => {
                if let Some(attributes) = set_options.attribute_list() {
                    self.lower_operator_attributes(&attributes, left, right);
                }
            }
            Some(ast::AlterOperatorAction::OwnerTo(owner_to)) => self.lower_owner_to(&owner_to),
            Some(ast::AlterOperatorAction::SetSchema(set_schema)) => {
                self.set_operator_schema(operator, &set_schema);
            }
            None => {}
        }
    }

    fn set_operator_schema(&mut self, operator: Option<OperatorId>, set_schema: &ast::SetSchema) {
        let Some(schema_ref) = set_schema.schema_ref() else {
            return;
        };
        let Some(schema) = self.lower_set_schema_ref(&schema_ref) else {
            return;
        };
        let Some(operator) = operator else {
            return;
        };
        let Operator {
            name,
            left,
            right,
            function,
            ptr,
            name_range,
            ..
        } = self.index.operators[operator];
        if self
            .find_operator(Some(schema), name, left, right)
            .is_some()
        {
            let error = SemaError::DuplicateInSchema {
                object: ObjectKind::Operator,
                name,
                schema,
            };
            self.push_error(schema_ref.syntax(), error);
            return;
        }
        let moved = self.index.operators.next_idx();
        self.drop_operator(operator);
        self.index.operators.alloc(Operator {
            schema,
            name,
            left,
            right,
            function,
            ptr,
            name_range,
        });
        self.record_moves(
            Object::Operator(operator),
            self.index.operators.range_from(moved),
            Object::Operator,
        );
    }

    fn lower_drop_operator(&mut self, drop_operator: &ast::DropOperator) {
        let if_missing = IfMissing::from_if_exists(drop_operator.if_exists().as_ref());
        let cascade = matches!(
            drop_operator.drop_behavior(),
            Some(ast::DropBehavior::Cascade(_))
        );
        for op_sig in drop_operator
            .op_sig_list()
            .iter()
            .flat_map(|list| list.op_sigs())
        {
            let Operands { left, right } = self.lower_op_sig_operands(&op_sig, if_missing);
            let Some(op) = op_sig.op() else {
                continue;
            };
            let Some(id) = self.lower_operator_ref(&op, left, right, if_missing) else {
                continue;
            };
            if cascade {
                self.drop_operator_with_dependents(id);
            } else if self.dependents(Object::Operator(id)).is_empty() {
                self.drop_operator(id);
            } else {
                self.push_error(
                    op.syntax(),
                    SemaError::DependentObjects {
                        object: ObjectKind::Operator,
                        name: self.index.operators[id].name,
                    },
                );
            }
        }
    }

    fn lower_op_sig_operands(&mut self, op_sig: &ast::OpSig, if_missing: IfMissing) -> Operands {
        let left = self.lower_op_sig_operand(op_sig.lhs(), if_missing);
        let right = self.lower_op_sig_operand(op_sig.rhs(), if_missing);
        Operands { left, right }
    }

    fn lower_op_sig_operand(
        &mut self,
        operand: Option<ast::OpSigOperand>,
        if_missing: IfMissing,
    ) -> Operand {
        match operand {
            Some(ast::OpSigOperand::None(_)) => Operand::None,
            Some(ast::OpSigOperand::Type(ty)) => self
                .lower_type_ref(&ty, if_missing)
                .map_or(Operand::Unknown, Operand::Ty),
            None => Operand::Unknown,
        }
    }

    fn lower_operand(&mut self, ty: Option<ast::Type>, if_missing: IfMissing) -> Operand {
        ty.and_then(|ty| self.lower_type_ref(&ty, if_missing))
            .map_or(Operand::Unknown, Operand::Ty)
    }

    fn lower_operator_attributes(
        &mut self,
        attributes: &ast::AttributeList,
        left: Operand,
        right: Operand,
    ) -> Option<FunctionId> {
        let mut function = None;
        for option in attributes.attribute_options() {
            let (Some(name), Some(value)) = (attribute_name(&option), option.attribute_value())
            else {
                continue;
            };
            match name.as_str() {
                "function" | "procedure" => {
                    if let Some(ast::Type::PathType(path_type)) = value.ty()
                        && let Some(path) = path_type.path_ref()
                    {
                        let arg_types: SmallVec<[Option<Ty>; 2]> = [left, right]
                            .into_iter()
                            .filter_map(|operand| match operand {
                                Operand::None => None,
                                Operand::Unknown => Some(None),
                                Operand::Ty(ty) => Some(Some(ty)),
                            })
                            .collect();
                        function = self.lower_function_path_ref(&path, Some(&arg_types));
                    }
                }
                "restrict" | "join" => {
                    if let Some(ast::Type::PathType(path_type)) = value.ty()
                        && let Some(path) = path_type.path_ref()
                    {
                        self.lower_function_path_ref(&path, None);
                    }
                }
                "commutator" => self.lower_operator_attribute_ref(&value, right, left),
                "negator" => self.lower_operator_attribute_ref(&value, left, right),
                _ => {}
            }
        }
        function
    }

    fn lower_operator_attribute_ref(
        &mut self,
        value: &ast::AttributeValue,
        left: Operand,
        right: Operand,
    ) {
        match value.operator_call().and_then(|call| call.op()) {
            Some(op) => {
                self.lower_operator_ref(&op, left, right, IfMissing::Ignore);
            }
            None => {
                let range = value.syntax().text_range();
                let text = value.operator_text();
                let schema = self.default_creation_schema();
                self.add_operator_ref(range, Some(schema), &text, left, right, IfMissing::Ignore);
            }
        }
    }

    fn lower_operator_ref(
        &mut self,
        op: &ast::Op,
        left: Operand,
        right: Operand,
        if_missing: IfMissing,
    ) -> Option<OperatorId> {
        let path = op.path_ref();
        let schema = self.lower_schema_qualifier(path.as_ref().and_then(|path| path.qualifier()));
        let range = op.operator_range();
        let text = op.operator_text();
        self.add_operator_ref(range, schema, &text, left, right, if_missing)
    }

    fn add_operator_ref(
        &mut self,
        range: TextRange,
        schema: Option<Name>,
        text: &str,
        left: Operand,
        right: Operand,
        if_missing: IfMissing,
    ) -> Option<OperatorId> {
        let name = self.intern(text);
        let id = self.find_operator(schema, name, left, right);
        let resolution = match id {
            Some(id) => Resolution::Operator(id),
            None => match if_missing {
                IfMissing::Ignore => Resolution::Unknown,
                IfMissing::Error => Resolution::Unresolved,
            },
        };
        self.add_ref_at(name, ReferenceKind::Operator, range, resolution);
        id
    }

    fn find_operator(
        &self,
        schema: Option<Name>,
        name: Name,
        left: Operand,
        right: Operand,
    ) -> Option<OperatorId> {
        self.search_path(schema, Lookup::Operator)
            .find_map(|schema| {
                self.index.operators.named(schema, name).find(|id| {
                    let operator = &self.index.operators[*id];
                    self.visible(operator.schema, operator.ptr.file_id)
                        && !self.dropped_operators.contains(id)
                        && operator.left.matches(left)
                        && operator.right.matches(right)
                })
            })
    }

    fn select_operator(
        &self,
        schema: Option<Name>,
        name: Name,
        left: Option<ArgType>,
        right: Option<ArgType>,
    ) -> Resolution {
        let candidates = self
            .search_path(schema, Lookup::Operator)
            .flat_map(|schema| {
                self.index
                    .operators
                    .named(schema, name)
                    .map(|id| (id, &self.index.operators[id]))
                    .filter(move |(id, operator)| {
                        self.visible(operator.schema, operator.ptr.file_id)
                            && !self.dropped_operators.contains(id)
                    })
            });
        self.overloads().select_operator(candidates, left, right)
    }

    fn lower_create_cast(&mut self, create_cast: &ast::CreateCast) {
        let refs = self.index.references.next_idx();
        let start = self.index.casts.next_idx();
        let types = create_cast
            .cast_sig()
            .and_then(|cast_sig| self.lower_cast_sig(&cast_sig, IfMissing::Error));
        match create_cast.cast_method() {
            Some(ast::CastMethod::WithFunction(with_function)) => {
                if let Some(function_sig) = with_function.function_sig() {
                    self.lower_function_sig(&function_sig);
                }
            }
            Some(ast::CastMethod::WithInout(_) | ast::CastMethod::WithoutFunction(_)) | None => {}
        }
        let Some(types) = types else {
            return;
        };
        let context = match create_cast.cast_context() {
            Some(ast::CastContext::AsImplicit(_)) => CastContext::Implicit,
            Some(ast::CastContext::AsAssignment(_)) => CastContext::Assignment,
            None => CastContext::Explicit,
        };
        self.index.casts.alloc(Cast {
            source: types.source,
            target: types.target,
            context,
            ptr: InIrFile::new(self.file_id, AstPtr::new(create_cast)),
        });
        self.depend_on_refs(refs, self.index.casts.range_from(start), Object::Cast);
    }

    fn lower_drop_cast(&mut self, drop_cast: &ast::DropCast) {
        let Some(cast_sig) = drop_cast.cast_sig() else {
            return;
        };
        let if_missing = IfMissing::from_if_exists(drop_cast.if_exists().as_ref());
        let Some(types) = self.lower_cast_sig(&cast_sig, if_missing) else {
            return;
        };
        match self.find_cast(types) {
            Some(cast) => self.drop_cast(cast),
            None if matches!(if_missing, IfMissing::Error) => {
                let error = SemaError::UnresolvedCast {
                    source: self.index.ty_name(types.source),
                    target: self.index.ty_name(types.target),
                };
                self.push_error(cast_sig.syntax(), error);
            }
            None => {}
        }
    }

    fn lower_create_transform(&mut self, create_transform: &ast::CreateTransform) {
        let refs = self.index.references.next_idx();
        let key = self.lower_transform_key(
            create_transform.ty().as_ref(),
            create_transform.language_ref().as_ref(),
            IfMissing::Error,
        );
        for transform_func in create_transform.transform_funcs() {
            let function_sig = match transform_func {
                ast::TransformFunc::TransformFromFunc(from_func) => from_func.function_sig(),
                ast::TransformFunc::TransformToFunc(to_func) => to_func.function_sig(),
            };
            if let Some(function_sig) = function_sig {
                self.lower_function_sig(&function_sig);
            }
        }
        let (Some(key), Some(ty)) = (key, create_transform.ty()) else {
            return;
        };
        let transform = match self.transforms.get(&key) {
            Some(&transform) if create_transform.or_replace().is_some() => transform,
            Some(_) => {
                let error = SemaError::DuplicateTransform {
                    ty: self.index.ty_name(key.ty),
                    language: self.index.languages[key.language].name,
                };
                self.push_error(ty.syntax(), error);
                return;
            }
            None => {
                let transform = self.index.transforms.alloc(Transform {
                    key,
                    ptr: InIrFile::new(self.file_id, AstPtr::new(create_transform)),
                });
                self.transforms.insert(key, transform);
                transform
            }
        };
        self.depend_on_refs(refs, IdxRange::single(transform), Object::Transform);
    }

    fn lower_drop_transform(&mut self, drop_transform: &ast::DropTransform) {
        let if_missing = IfMissing::from_if_exists(drop_transform.if_exists().as_ref());
        let Some(ty) = drop_transform.ty() else {
            return;
        };
        if let Some(transform) = self.lower_transform_ref(
            ty.syntax(),
            Some(&ty),
            drop_transform.language_ref().as_ref(),
            if_missing,
        ) {
            self.drop_object(Object::Transform(transform));
        }
    }

    fn lower_transform_key(
        &mut self,
        ty: Option<&ast::Type>,
        language_ref: Option<&ast::LanguageRef>,
        if_missing: IfMissing,
    ) -> Option<TransformKey> {
        let ty = ty.and_then(|ty| self.lower_type_ref(ty, if_missing));
        let language =
            language_ref.and_then(|language_ref| self.lower_language_ref(language_ref, if_missing));
        Some(TransformKey {
            ty: ty?,
            language: language?,
        })
    }

    fn lower_transform_ref(
        &mut self,
        node: &SyntaxNode,
        ty: Option<&ast::Type>,
        language_ref: Option<&ast::LanguageRef>,
        if_missing: IfMissing,
    ) -> Option<TransformId> {
        let key = self.lower_transform_key(ty, language_ref, if_missing)?;
        let transform = self.transforms.get(&key).copied();
        if transform.is_none() && matches!(if_missing, IfMissing::Error) {
            let error = SemaError::UnresolvedTransform {
                ty: self.index.ty_name(key.ty),
                language: self.index.languages[key.language].name,
            };
            self.push_error(node, error);
        }
        transform
    }

    fn lower_cast_sig(
        &mut self,
        cast_sig: &ast::CastSig,
        if_missing: IfMissing,
    ) -> Option<CastTypes> {
        let source = cast_sig
            .lhs()
            .and_then(|ty| self.lower_type_ref(&ty, if_missing));
        let target = cast_sig
            .rhs()
            .and_then(|ty| self.lower_type_ref(&ty, if_missing));
        Some(CastTypes {
            source: source?,
            target: target?,
        })
    }

    fn lower_function_sig(&mut self, function_sig: &ast::FunctionSig) {
        if let Some(path) = function_sig
            .function_name_ref()
            .and_then(|function_name| function_name.path_ref())
        {
            self.lower_routine_sig(
                &path,
                function_sig.param_list(),
                RoutineKind::Function,
                IfMissing::Error,
            );
        }
    }

    fn lower_routine_sig(
        &mut self,
        path: &ast::PathRef,
        param_list: Option<ast::ParamList>,
        kind: RoutineKind,
        if_missing: IfMissing,
    ) -> Option<FunctionId> {
        let segment = path.segment()?;
        let schema = self.lower_schema_qualifier(path.qualifier());
        let name = self.intern(&segment.text());
        let resolution = match param_list {
            Some(param_list) => {
                let mut arg_types: SmallVec<[Option<Ty>; 4]> = SmallVec::new();
                let mut missing = false;
                for param in param_list.params() {
                    let ty = self.lower_func_type_lookup(param.func_type());
                    missing |= matches!(ty, TypeLookup::Missing);
                    if !matches!(param.mode(), Some(ast::ParamMode::ParamOut(_))) {
                        arg_types.push(ty.ty());
                    }
                }
                for param in param_list
                    .aggregate_order_by()
                    .iter()
                    .flat_map(|order_by| order_by.params())
                {
                    let ty = self.lower_func_type_lookup(param.func_type());
                    missing |= matches!(ty, TypeLookup::Missing);
                    arg_types.push(ty.ty());
                }
                if missing {
                    Some(Resolution::Unknown)
                } else {
                    self.find_function_by(schema, name, |function| {
                        kind.matches(function.kind) && self.has_arg_types(function, &arg_types)
                    })
                    .map(Resolution::Function)
                }
            }
            None => self.find_unique_function(schema, name, kind),
        };
        let resolution = resolution.unwrap_or(match if_missing {
            IfMissing::Error => Resolution::Unresolved,
            IfMissing::Ignore => Resolution::Unknown,
        });
        self.add_ref(
            name,
            ReferenceKind::Function(kind),
            segment.syntax(),
            resolution,
        );
        match resolution {
            Resolution::Function(function) => Some(function),
            _ => None,
        }
    }

    fn find_unique_function(
        &self,
        schema: Option<Name>,
        name: Name,
        kind: RoutineKind,
    ) -> Option<Resolution> {
        let mut functions = self
            .search_path(schema, Lookup::Function)
            .flat_map(|schema| {
                self.index.functions.named(schema, name).filter(move |id| {
                    let function = &self.index.functions[*id];
                    kind.matches(function.kind)
                        && self.visible(function.schema, function.ptr.file_id)
                        && !self.dropped_functions.contains(id)
                })
            });
        match (functions.next(), functions.next()) {
            (Some(function), None) => Some(Resolution::Function(function)),
            (Some(_), Some(_)) => Some(Resolution::Ambiguous),
            (None, _) => None,
        }
    }

    fn lower_drop_function(&mut self, drop_function: &ast::DropFunction) {
        let if_missing = IfMissing::from_if_exists(drop_function.if_exists().as_ref());
        for sig in drop_function
            .function_sig_list()
            .iter()
            .flat_map(|list| list.function_sigs())
        {
            if let Some(path) = sig.function_name_ref().and_then(|name| name.path_ref()) {
                self.drop_routine_sig(
                    &path,
                    sig.param_list(),
                    RoutineKind::Function,
                    if_missing,
                    drop_function.drop_behavior().as_ref(),
                );
            }
        }
    }

    fn lower_drop_procedure(&mut self, drop_procedure: &ast::DropProcedure) {
        let if_missing = IfMissing::from_if_exists(drop_procedure.if_exists().as_ref());
        for sig in drop_procedure
            .procedure_sig_list()
            .iter()
            .flat_map(|list| list.procedure_sigs())
        {
            if let Some(path) = sig.procedure_name_ref().and_then(|name| name.path_ref()) {
                self.drop_routine_sig(
                    &path,
                    sig.param_list(),
                    RoutineKind::Procedure,
                    if_missing,
                    drop_procedure.drop_behavior().as_ref(),
                );
            }
        }
    }

    fn lower_drop_routine(&mut self, drop_routine: &ast::DropRoutine) {
        let if_missing = IfMissing::from_if_exists(drop_routine.if_exists().as_ref());
        for sig in drop_routine
            .routine_sig_list()
            .iter()
            .flat_map(|list| list.routine_sigs())
        {
            if let Some(path) = sig.routine_name_ref().and_then(|name| name.path_ref()) {
                self.drop_routine_sig(
                    &path,
                    sig.param_list(),
                    RoutineKind::Routine,
                    if_missing,
                    drop_routine.drop_behavior().as_ref(),
                );
            }
        }
    }

    fn drop_routine_sig(
        &mut self,
        path: &ast::PathRef,
        param_list: Option<ast::ParamList>,
        kind: RoutineKind,
        if_missing: IfMissing,
        drop_behavior: Option<&ast::DropBehavior>,
    ) {
        let Some(function) = self.lower_routine_sig(path, param_list, kind, if_missing) else {
            return;
        };
        match drop_behavior {
            Some(ast::DropBehavior::Cascade(_)) => self.drop_function_with_dependents(function),
            Some(ast::DropBehavior::Restrict(_)) | None => {
                if self.dependents(Object::Function(function)).is_empty() {
                    self.drop_function(function);
                } else {
                    self.push_error(
                        path.syntax(),
                        SemaError::DependentObjects {
                            object: ObjectKind::Function,
                            name: self.index.functions[function].name,
                        },
                    );
                }
            }
        }
    }

    fn lower_alter_function(&mut self, alter_function: &ast::AlterFunction) {
        let function = alter_function.function_sig().and_then(|sig| {
            let path = sig.function_name_ref()?.path_ref()?;
            self.lower_routine_sig(
                &path,
                sig.param_list(),
                RoutineKind::Function,
                IfMissing::Error,
            )
        });
        let node = alter_function.syntax();
        match alter_function.action() {
            Some(ast::AlterFunctionAction::FunctionRenameTo(rename)) => {
                if let Some(function) = function
                    && let Some(path) = rename.function_name().and_then(|name| name.path())
                {
                    self.rename_function(function, &path, node);
                }
            }
            Some(ast::AlterFunctionAction::SetSchema(set_schema)) => {
                self.set_function_schema(function, &set_schema, node);
            }
            Some(ast::AlterFunctionAction::FuncOptionList(options)) => {
                self.lower_func_options(&options);
            }
            Some(ast::AlterFunctionAction::OwnerTo(owner_to)) => self.lower_owner_to(&owner_to),
            Some(ast::AlterFunctionAction::DependsOnExtension(depends)) => {
                if let Some(extension) = depends.extension_ref() {
                    self.lower_extension_ref(&extension, IfMissing::Error);
                }
            }
            Some(ast::AlterFunctionAction::NoDependsOnExtension(depends)) => {
                if let Some(extension) = depends.extension_ref() {
                    self.lower_extension_ref(&extension, IfMissing::Error);
                }
            }
            None => {}
        }
    }

    fn lower_alter_procedure(&mut self, alter_procedure: &ast::AlterProcedure) {
        let function = alter_procedure.procedure_sig().and_then(|sig| {
            let path = sig.procedure_name_ref()?.path_ref()?;
            self.lower_routine_sig(
                &path,
                sig.param_list(),
                RoutineKind::Procedure,
                IfMissing::Error,
            )
        });
        let node = alter_procedure.syntax();
        match alter_procedure.action() {
            Some(ast::AlterProcedureAction::ProcedureRenameTo(rename)) => {
                if let Some(function) = function
                    && let Some(path) = rename.procedure_name().and_then(|name| name.path())
                {
                    self.rename_function(function, &path, node);
                }
            }
            Some(ast::AlterProcedureAction::SetSchema(set_schema)) => {
                self.set_function_schema(function, &set_schema, node);
            }
            Some(ast::AlterProcedureAction::FuncOptionList(options)) => {
                self.lower_func_options(&options);
            }
            Some(ast::AlterProcedureAction::OwnerTo(owner_to)) => self.lower_owner_to(&owner_to),
            Some(ast::AlterProcedureAction::DependsOnExtension(depends)) => {
                if let Some(extension) = depends.extension_ref() {
                    self.lower_extension_ref(&extension, IfMissing::Error);
                }
            }
            Some(ast::AlterProcedureAction::NoDependsOnExtension(depends)) => {
                if let Some(extension) = depends.extension_ref() {
                    self.lower_extension_ref(&extension, IfMissing::Error);
                }
            }
            None => {}
        }
    }

    fn lower_alter_routine(&mut self, alter_routine: &ast::AlterRoutine) {
        let function = alter_routine.routine_sig().and_then(|sig| {
            let path = sig.routine_name_ref()?.path_ref()?;
            self.lower_routine_sig(
                &path,
                sig.param_list(),
                RoutineKind::Routine,
                IfMissing::Error,
            )
        });
        let node = alter_routine.syntax();
        match alter_routine.action() {
            Some(ast::AlterRoutineAction::RoutineRenameTo(rename)) => {
                if let Some(function) = function
                    && let Some(path) = rename.routine_name().and_then(|name| name.path())
                {
                    self.rename_function(function, &path, node);
                }
            }
            Some(ast::AlterRoutineAction::SetSchema(set_schema)) => {
                self.set_function_schema(function, &set_schema, node);
            }
            Some(ast::AlterRoutineAction::FuncOptionList(options)) => {
                self.lower_func_options(&options);
            }
            Some(ast::AlterRoutineAction::OwnerTo(owner_to)) => self.lower_owner_to(&owner_to),
            Some(ast::AlterRoutineAction::DependsOnExtension(depends)) => {
                if let Some(extension) = depends.extension_ref() {
                    self.lower_extension_ref(&extension, IfMissing::Error);
                }
            }
            Some(ast::AlterRoutineAction::NoDependsOnExtension(depends)) => {
                if let Some(extension) = depends.extension_ref() {
                    self.lower_extension_ref(&extension, IfMissing::Error);
                }
            }
            None => {}
        }
    }

    fn rename_function(&mut self, function: FunctionId, path: &ast::Path, node: &SyntaxNode) {
        let Some(segment) = path.segment() else {
            return;
        };
        let name = self.intern(&segment.text());
        let schema = self.index.functions[function].schema;
        let name_ptr = self.ast_ptr(&segment);
        if let Some(id) =
            self.move_function(function, node, segment.syntax(), schema, name, name_ptr)
        {
            self.add_item(segment.syntax(), Item::Function(id));
        }
    }

    fn set_function_schema(
        &mut self,
        function: Option<FunctionId>,
        set_schema: &ast::SetSchema,
        node: &SyntaxNode,
    ) {
        let Some(schema_ref) = set_schema.schema_ref() else {
            return;
        };
        let Some(schema) = self.lower_set_schema_ref(&schema_ref) else {
            return;
        };
        if let Some(function) = function {
            let Function { name, name_ptr, .. } = self.index.functions[function];
            self.move_function(function, node, schema_ref.syntax(), schema, name, name_ptr);
        }
    }

    fn move_function(
        &mut self,
        function: FunctionId,
        node: &SyntaxNode,
        name_node: &SyntaxNode,
        schema: Name,
        name: Name,
        name_ptr: InIrFile<AstPtr<ast::PathSegment>>,
    ) -> Option<FunctionId> {
        let Function {
            kind,
            window,
            params,
            direct_args,
            ret,
            columns,
            ..
        } = self.index.functions[function];
        if self
            .find_function_by(Some(schema), name, |function| {
                self.same_signature((function.kind, function.params), (kind, params))
            })
            .is_some()
        {
            let error = SemaError::DuplicateFunction {
                name,
                schema: Some(schema),
            };
            self.push_error(name_node, error);
            return None;
        }
        self.drop_function(function);
        let id = self.index.functions.alloc(Function {
            kind,
            window,
            schema,
            name,
            ptr: self.ptr(node),
            name_ptr,
            params,
            direct_args,
            ret,
            columns,
        });
        self.index.set_param_function(params, id);
        if let Some(columns) = columns {
            self.index
                .set_column_owner(columns.range, ColumnOwner::Function(id));
        }
        self.copy_comment(Item::Function(function), Item::Function(id));
        self.dependencies
            .add_move(Object::Function(function), Object::Function(id));
        Some(id)
    }

    fn lower_call_stmt(&mut self, call: &ast::Call) {
        let CallArgs {
            args,
            named: named_args,
        } = self.with_expr_context(ExprContext::CallArgument, |this| {
            this.lower_call_args(call.arg_list())
        });
        let procedure = call
            .procedure_name_ref()
            .and_then(|name| name.path_ref())
            .and_then(|path| {
                let segment = path.segment()?;
                let schema = self.lower_schema_qualifier(path.qualifier());
                let name = self.intern(&segment.text());
                let resolution = self.find_routine(schema, name, &args, FunctionKind::Procedure);
                self.add_ref(
                    name,
                    ReferenceKind::Function(RoutineKind::Procedure),
                    segment.syntax(),
                    resolution,
                );
                match resolution {
                    Resolution::Function(function) => Some(function),
                    _ => None,
                }
            });
        self.lower_named_args(procedure, named_args);
    }

    fn lower_function_path_ref(
        &mut self,
        path: &ast::PathRef,
        arg_types: Option<&[Option<Ty>]>,
    ) -> Option<FunctionId> {
        let segment = path.segment()?;
        let schema = self.lower_schema_qualifier(path.qualifier());
        let name = self.intern(&segment.text());
        let function = self.find_function_by(schema, name, |function| {
            arg_types.is_none_or(|arg_types| self.has_arg_types(function, arg_types))
        });
        let resolution = function.map_or(Resolution::Unresolved, Resolution::Function);
        self.add_ref(
            name,
            ReferenceKind::Function(RoutineKind::Function),
            segment.syntax(),
            resolution,
        );
        function
    }

    fn has_arg_types(&self, function: &Function, arg_types: &[Option<Ty>]) -> bool {
        let mut inputs = function
            .params
            .iter()
            .map(|id| &self.index.params[id])
            .filter(|param| param.mode != ParamMode::Out);
        arg_types.iter().all(|arg_type| {
            inputs
                .next()
                .is_some_and(|param| arg_type.is_none() || *arg_type == param.ty)
        }) && inputs.next().is_none()
    }

    fn lower_database_options(&mut self, options: &ast::DatabaseOptionList) {
        for option in options.database_options() {
            match option {
                ast::DatabaseOption::DatabaseOptionTemplate(template) => {
                    if let Some(database) = template.database_ref() {
                        self.lower_database_ref(&database, IfMissing::Error);
                    }
                }
                ast::DatabaseOption::DatabaseOptionTablespace(tablespace) => {
                    if let Some(tablespace) = tablespace.tablespace_ref() {
                        self.lower_tablespace_ref(&tablespace, IfMissing::Error);
                    }
                }
                ast::DatabaseOption::DatabaseOptionOwner(owner) => {
                    if let Some(role_ref) = owner.role_ref() {
                        self.lower_role_ref(&role_ref, IfMissing::Error);
                    }
                }
                ast::DatabaseOption::DatabaseOptionConnectionLimit(_)
                | ast::DatabaseOption::DatabaseOptionEncoding(_)
                | ast::DatabaseOption::DatabaseOptionGeneric(_) => {}
            }
        }
    }

    fn lower_database_ref(
        &mut self,
        database: &ast::DatabaseRef,
        if_missing: IfMissing,
    ) -> Option<DatabaseId> {
        let name = self.intern(&database.text());
        self.add_database_ref(name, database.syntax(), if_missing)
    }

    fn add_database_ref(
        &mut self,
        name: Name,
        node: &SyntaxNode,
        if_missing: IfMissing,
    ) -> Option<DatabaseId> {
        let id = self.find_database(name);
        let resolution = match id {
            Some(id) => Resolution::Database(id),
            None => match if_missing {
                IfMissing::Ignore => Resolution::Unknown,
                IfMissing::Error => Resolution::Unresolved,
            },
        };
        self.add_ref(name, ReferenceKind::Database, node, resolution);
        id
    }

    fn find_database(&self, name: Name) -> Option<DatabaseId> {
        self.index
            .databases
            .get(name)
            .find(|id| !self.dropped_databases.contains(id))
    }

    fn find_tablespace(&self, name: Name) -> Option<TablespaceId> {
        self.index
            .tablespaces
            .get(name)
            .find(|id| !self.dropped_tablespaces.contains(id))
    }

    fn lower_create_table(
        &mut self,
        create_table: &impl ast::HasCreateTable,
        kind: TableKind,
        if_exists: IfExists,
        partition_of: Option<ast::PartitionOf>,
        partition_type: Option<ast::PartitionType>,
        tablespace: Option<ast::TablespaceClause>,
        using_method: Option<ast::UsingMethod>,
        partition_by: Option<ast::PartitionBy>,
        of_type: Option<ast::OfType>,
    ) {
        if let Some(partition_type) = partition_type {
            self.lower_partition_type(&partition_type);
        }
        if let Some(using_method) = using_method {
            self.lower_using_method(&using_method, AccessMethodKind::Table);
        }
        if let Some(tablespace) = tablespace {
            self.lower_tablespace_clause(&tablespace);
        }
        let Some(path) = create_table.table_name().and_then(|name| name.path()) else {
            return;
        };
        let start = self.index.columns.next_idx();
        let mut complete = true;
        let mut constraints_complete = true;
        let mut parents: SmallVec<[TableId; 2]> = SmallVec::new();
        let inheritance_kind = if partition_of.is_some() {
            InheritanceKind::Partition
        } else {
            InheritanceKind::Inherits
        };
        for parent in partition_of
            .and_then(|partition_of| partition_of.table_name_ref())
            .into_iter()
            .chain(
                create_table
                    .inherits()
                    .into_iter()
                    .flat_map(|inherits| inherits.table_name_refs()),
            )
        {
            constraints_complete = false;
            let Some(path) = parent.path_ref() else {
                continue;
            };
            let Some(parent) = self.lower_table_ref(&path, IfMissing::Error, TableLookup::Table)
            else {
                complete = false;
                continue;
            };
            parents.push(parent);
            let parent_columns = &self.index.tables[parent].columns;
            complete &= parent_columns.complete;
            for id in parent_columns.range {
                let column = self.index.columns[id];
                if column
                    .name
                    .is_none_or(|name| self.find_column_from(start, name).is_none())
                {
                    self.index.columns.alloc(column);
                }
            }
        }
        let mut typed_table_of = None;
        if let Some(ty) = of_type.and_then(|of_type| of_type.ty()) {
            match self.lower_type(&ty) {
                Some(Ty::Type(ty)) => match self.index.types[ty].columns {
                    Some(columns) => {
                        complete &= columns.complete;
                        self.index.columns.alloc_copies(columns.range);
                        typed_table_of = Some(ty);
                    }
                    None => complete = false,
                },
                Some(Ty::Table(_) | Ty::Cte(_) | Ty::Array(_)) | None => complete = false,
            }
        }
        let inherited_end = self.index.columns.next_idx();
        let mut like_primary_keys: SmallVec<[ast::LikeClause; 1]> = SmallVec::new();
        for arg in create_table
            .table_arg_list()
            .into_iter()
            .flat_map(|list| list.args())
        {
            match arg {
                ast::TableArg::Column(column) => {
                    let ty = column.ty().and_then(|ty| self.lower_column_type(&ty));
                    if let Some(column_name) = column.name() {
                        let def = Column {
                            ty,
                            ..self.column_def(&column_name)
                        };
                        if let Some(name) = def.name
                            && self.find_column_from(inherited_end, name).is_some()
                        {
                            let error = SemaError::ColumnSpecifiedMoreThanOnce { name };
                            self.push_error(column_name.syntax(), error);
                            continue;
                        }
                        let id = self.add_table_column(start, inherited_end, def);
                        self.record_comment(column.syntax(), Item::Column(id));
                        self.add_item_range(def.ptr.file_range(), Item::Column(id));
                    }
                }
                ast::TableArg::LikeClause(like) => {
                    constraints_complete = false;
                    let Some(path) = like.relation_name_ref().and_then(|name| name.path_ref())
                    else {
                        complete = false;
                        continue;
                    };
                    let Some(source) = self.lower_like_ref(&path) else {
                        complete = false;
                        continue;
                    };
                    if like_includes_indexes(&like) && self.has_primary_key(source) {
                        like_primary_keys.push(like.clone());
                    }
                    let source_columns = &self.index.tables[source].columns;
                    complete &= source_columns.complete;
                    for id in source_columns.range {
                        let column = self.index.columns[id];
                        if let Some(name) = column.name
                            && self.find_column_from(inherited_end, name).is_some()
                        {
                            let error = SemaError::ColumnSpecifiedMoreThanOnce { name };
                            self.push_error(like.syntax(), error);
                            continue;
                        }
                        self.add_table_column(start, inherited_end, column);
                    }
                }
                ast::TableArg::TableConstraint(_) => {}
            }
        }
        let columns = Columns {
            range: self.index.columns.range_from(start),
            complete,
        };
        let persistence = Persistence::from_ast(create_table.persistence().as_ref());
        let Some(table) = self.alloc_table(
            kind,
            persistence,
            if_exists,
            create_table.syntax(),
            &path,
            columns,
        ) else {
            return;
        };
        for parent in parents {
            self.add_inheritance(parent, table, inheritance_kind);
        }
        for like in like_primary_keys {
            if self.has_primary_key(table) {
                let name = self.index.tables[table].name;
                self.push_error(
                    like.syntax(),
                    SemaError::MultiplePrimaryKeys { table: name },
                );
            }
            self.alloc_index_constraint(
                ConstraintKind::PrimaryKey,
                table,
                like.syntax(),
                None,
                None,
                None,
                "pkey",
            );
        }
        self.index
            .tables
            .set_constraints_complete(table, constraints_complete);
        self.index.tables.set_of_type(table, typed_table_of);
        if let Some(partition_by) = partition_by {
            self.lower_partition_by(table, create_table.syntax(), &partition_by);
        }
        let Some(args) = create_table.table_arg_list() else {
            return;
        };
        self.with_table_scope(
            QueryKind::Constraint,
            Some(table),
            create_table.syntax(),
            |this| {
                for arg in args.args() {
                    match arg {
                        ast::TableArg::Column(column) => {
                            let column_name = column.name().map(|name| this.intern(&name.text()));
                            if let Some(column_name) = column_name
                                && let Some(ty) = column.ty()
                            {
                                this.lower_serial_column(table, column_name, &ty, column.syntax());
                            }
                            if let Some(column_name) = column_name {
                                let constraints =
                                    column.clauses().filter_map(|clause| match clause {
                                        ast::ColumnClause::ColumnConstraint(constraint) => {
                                            Some(constraint.into())
                                        }
                                        ast::ColumnClause::Collate(_)
                                        | ast::ColumnClause::AlterOptionList(_)
                                        | ast::ColumnClause::CompressionMethod(_)
                                        | ast::ColumnClause::Storage(_)
                                        | ast::ColumnClause::WithOptions(_) => None,
                                    });
                                this.check_column_nullability(table, column_name, constraints);
                            }
                            for clause in column.clauses() {
                                match clause {
                                    ast::ColumnClause::ColumnConstraint(constraint) => {
                                        this.lower_constraint(
                                            Some(table),
                                            column_name,
                                            &constraint.into(),
                                        );
                                    }
                                    ast::ColumnClause::Collate(collate) => {
                                        this.lower_collate(&collate);
                                    }
                                    ast::ColumnClause::AlterOptionList(_)
                                    | ast::ColumnClause::CompressionMethod(_)
                                    | ast::ColumnClause::Storage(_)
                                    | ast::ColumnClause::WithOptions(_) => {}
                                }
                            }
                        }
                        ast::TableArg::TableConstraint(constraint) => {
                            this.lower_constraint(Some(table), None, &constraint.into());
                        }
                        ast::TableArg::LikeClause(_) => {}
                    }
                }
            },
        );
    }

    fn lower_partition_by(
        &mut self,
        table: TableId,
        node: &SyntaxNode,
        partition_by: &ast::PartitionBy,
    ) {
        let hash = partition_by
            .partition_strategy()
            .and_then(|strategy| strategy.kind())
            == Some(ast::PartitionStrategyKind::Hash);
        let access_method = self.intern(if hash { "hash" } else { "btree" });
        let access_method = self.find_access_method(access_method);
        self.with_table_scope(QueryKind::Constraint, Some(table), node, |this| {
            for item in partition_by
                .partition_item_list()
                .into_iter()
                .flat_map(|list| list.partition_items())
            {
                if let Some(expr) = item.expr() {
                    this.lower_expr_in(ExprContext::PartitionKey, &expr);
                }
                if let Some(path) = item.op_class_ref().and_then(|op_class| op_class.path_ref()) {
                    this.lower_operator_class_ref(&path, access_method, IfMissing::Error);
                }
            }
        });
    }

    fn lower_partition_type(&mut self, partition_type: &ast::PartitionType) {
        self.with_expr_context(ExprContext::PartitionBound, |this| {
            this.lower_partition_bound(partition_type);
        });
    }

    fn lower_partition_bound(&mut self, partition_type: &ast::PartitionType) {
        match partition_type {
            ast::PartitionType::PartitionForValuesFrom(from_to) => {
                for expr in from_to
                    .from()
                    .into_iter()
                    .flat_map(|from| from.exprs())
                    .chain(from_to.to().into_iter().flat_map(|to| to.exprs()))
                {
                    if let ast::Expr::NameRef(name_ref) = &expr
                        && name_ref.range_bound_keyword().is_some()
                    {
                        continue;
                    }
                    self.lower_expr(&expr);
                }
            }
            ast::PartitionType::PartitionForValuesIn(values_in) => {
                self.lower_exprs(values_in.exprs());
            }
            ast::PartitionType::PartitionForValuesWith(_)
            | ast::PartitionType::PartitionDefault(_) => {}
        }
    }

    fn add_table_column(
        &mut self,
        start: ColumnId,
        inherited_end: ColumnId,
        column: Column,
    ) -> ColumnId {
        let inherited = column
            .name
            .and_then(|name| self.find_column_from(start, name))
            .filter(|id| *id < inherited_end);
        match inherited {
            Some(id) => {
                let ty = column.ty.or(self.index.columns[id].ty);
                self.index.columns[id] = Column { ty, ..column };
                id
            }
            None => self.index.columns.alloc(column),
        }
    }

    fn lower_like_ref(&mut self, path: &ast::PathRef) -> Option<TableId> {
        let segment = path.segment()?;
        let name = self.intern(&segment.text());
        let schema = self.lower_schema_qualifier(path.qualifier());
        let (kind, resolution) = match self.find_table(schema, name) {
            Some(table) => (ReferenceKind::Table, Resolution::Table(table)),
            None => match self.find_type(schema, name) {
                Resolution::Unresolved => (ReferenceKind::Table, Resolution::Unresolved),
                resolution => (ReferenceKind::Type, resolution),
            },
        };
        self.add_ref(name, kind, segment.syntax(), resolution);
        match resolution {
            Resolution::Table(table) => Some(table),
            Resolution::Column { .. }
            | Resolution::Source(_)
            | Resolution::Cte(_)
            | Resolution::Function(_)
            | Resolution::Param(_)
            | Resolution::Type(_)
            | Resolution::Window(_)
            | Resolution::JsonPath(_)
            | Resolution::Savepoint(_)
            | Resolution::Cursor(_)
            | Resolution::PreparedTransaction(_)
            | Resolution::PreparedStatement(_)
            | Resolution::Channel(_)
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
            | Resolution::Unknown
            | Resolution::Unresolved
            | Resolution::Ambiguous => None,
        }
    }

    fn find_column_from(&self, start: ColumnId, name: Name) -> Option<ColumnId> {
        self.find_column_in(self.index.columns.range_from(start), name)
    }

    fn lower_create_view(&mut self, create_view: &ast::CreateView) {
        let refs = self.index.references.next_idx();
        let start = self.index.tables.next_idx();
        let query = create_view.query().map(|query| self.lower_query(query));
        let Some(path) = create_view.view().and_then(|view| view.path()) else {
            return;
        };
        let columns = match query {
            Some(query) => self.index.queries[query].columns,
            None => self.unknown_columns(),
        };
        let columns = match create_view.column_list() {
            Some(list) => self.rename_columns(list.column_names(), &columns, ColumnListOwner::View),
            None => columns,
        };
        let persistence = Persistence::from_ast(create_view.persistence().as_ref());
        if let Some(table) = self.alloc_table(
            TableKind::View,
            persistence,
            IfExists::from_or_replace(create_view.or_replace().as_ref()),
            create_view.syntax(),
            &path,
            columns,
        ) {
            self.alloc_return_rule(table);
        }
        self.depend_on_refs(refs, self.index.tables.range_from(start), Object::Table);
    }

    fn lower_create_materialized_view(&mut self, create_view: &ast::CreateMaterializedView) {
        let refs = self.index.references.next_idx();
        let start = self.index.tables.next_idx();
        let query = create_view.query().map(|query| self.lower_query(query));
        if let Some(using_method) = create_view.using_method() {
            self.lower_using_method(&using_method, AccessMethodKind::Table);
        }
        if let Some(tablespace) = create_view.tablespace_clause() {
            self.lower_tablespace_clause(&tablespace);
        }
        let Some(path) = create_view.view().and_then(|view| view.path()) else {
            return;
        };
        let columns = match query {
            Some(query) => self.index.queries[query].columns,
            None => self.unknown_columns(),
        };
        let columns = match create_view.column_list() {
            Some(list) => {
                self.rename_columns(list.column_names(), &columns, ColumnListOwner::CreateAs)
            }
            None => columns,
        };
        if let Some(table) = self.alloc_table(
            TableKind::MaterializedView,
            Persistence::Permanent,
            IfExists::from_if_not_exists(create_view.if_not_exists().as_ref()),
            create_view.syntax(),
            &path,
            columns,
        ) {
            self.alloc_return_rule(table);
        }
        self.depend_on_refs(refs, self.index.tables.range_from(start), Object::Table);
    }

    fn lower_create_table_as(&mut self, create_table_as: &ast::CreateTableAs) {
        let columns = create_table_as
            .query()
            .map(|query| self.lower_create_table_as_query(query));
        if let Some(using_method) = create_table_as.using_method() {
            self.lower_using_method(&using_method, AccessMethodKind::Table);
        }
        if let Some(tablespace) = create_table_as.tablespace_clause() {
            self.lower_tablespace_clause(&tablespace);
        }
        let Some(path) = create_table_as.table_name().and_then(|name| name.path()) else {
            return;
        };
        let columns = match columns {
            Some(columns) => columns,
            None => self.unknown_columns(),
        };
        let columns = match create_table_as.table_arg_list() {
            Some(list) => self.rename_columns(
                list.args().filter_map(|arg| match arg {
                    ast::TableArg::Column(column) => column.name(),
                    ast::TableArg::LikeClause(_) | ast::TableArg::TableConstraint(_) => None,
                }),
                &columns,
                ColumnListOwner::CreateAs,
            ),
            None => columns,
        };
        let persistence = Persistence::from_ast(create_table_as.persistence().as_ref());
        self.alloc_table(
            TableKind::Table,
            persistence,
            IfExists::from_if_not_exists(create_table_as.if_not_exists().as_ref()),
            create_table_as.syntax(),
            &path,
            columns,
        );
    }

    fn lower_create_table_as_query(&mut self, query: ast::CreateTableAsQuery) -> Columns {
        match query {
            ast::CreateTableAsQuery::SelectVariant(query) => {
                let query = self.lower_query(query);
                self.index.queries[query].columns
            }
            ast::CreateTableAsQuery::Execute(execute) => match self.lower_execute(&execute) {
                Some(prepared) => self.index.prepared_statements[prepared].columns,
                None => self.unknown_columns(),
            },
        }
    }

    fn lower_create_type(&mut self, create_type: &ast::CreateType) {
        let refs = self.index.references.next_idx();
        let mut multirange_name = None;
        let mut columns = None;
        let mut element = None;
        let mut subtype = None;
        let mut subscript = None;
        let mut generated_subscript = None;
        let mut category = None;
        let mut preferred = false;
        let mut kind = TypeKind::Type;
        match create_type.kind() {
            Some(ast::CreateTypeKind::CompositeType(composite)) => {
                let start = self.index.columns.next_idx();
                for field in composite
                    .composite_field_list()
                    .into_iter()
                    .flat_map(|list| list.composite_field_defs())
                {
                    let ty = field.ty().and_then(|ty| self.lower_type(&ty));
                    if let Some(collate) = field.collate() {
                        self.lower_collate(&collate);
                    }
                    if let Some(name) = field.name() {
                        let column = Column {
                            name: Some(self.intern(&name.text())),
                            ptr: self.ptr(name.syntax()),
                            ty,
                            merge: None,
                        };
                        let id = self.alloc_column_def(column);
                        self.record_comment(field.syntax(), Item::Column(id));
                    }
                }
                columns = Some(Columns {
                    range: self.index.columns.range_from(start),
                    complete: true,
                });
            }
            Some(ast::CreateTypeKind::RangeType(range)) => {
                kind = TypeKind::Range;
                multirange_name = Some(MultirangeName::Default);
                let mut canonical = None;
                let mut subtype_diff = None;
                for option in range
                    .attribute_list()
                    .into_iter()
                    .flat_map(|list| list.attribute_options())
                {
                    let Some(name) = attribute_name(&option) else {
                        continue;
                    };
                    let ty = option.attribute_value().and_then(|value| value.ty());
                    match name.as_str() {
                        "subtype" => {
                            subtype = ty.and_then(|ty| self.lower_type(&ty));
                        }
                        "multirange_type_name" => {
                            let value = option.attribute_value();
                            if let Some(name) = value
                                .as_ref()
                                .and_then(|value| value.literal())
                                .and_then(|literal| literal.string_value())
                            {
                                multirange_name = Some(MultirangeName::String(name));
                            } else if let Some(ast::Type::PathType(path_type)) = ty
                                && let Some(path) = path_type.path_ref()
                            {
                                multirange_name = Some(MultirangeName::Path(path));
                            }
                        }
                        "collation" => {
                            if let Some(ast::Type::PathType(path_type)) = ty
                                && let Some(path) = path_type.path_ref()
                            {
                                self.lower_collation_ref(&path, IfMissing::Error);
                            }
                        }
                        option @ ("canonical" | "subtype_diff") => {
                            let Some(ast::Type::PathType(path_type)) = ty else {
                                continue;
                            };
                            if option == "canonical" {
                                canonical = path_type.path_ref();
                            } else {
                                subtype_diff = path_type.path_ref();
                            }
                        }
                        "subtype_opclass" => {
                            if let Some(ast::Type::PathType(path_type)) = ty
                                && let Some(path) = path_type.path_ref()
                            {
                                let btree = self.intern("btree");
                                let access_method = self.find_access_method(btree);
                                self.lower_operator_class_ref(
                                    &path,
                                    access_method,
                                    IfMissing::Error,
                                );
                            }
                        }
                        _ => {}
                    }
                }
                if let Some(path) = canonical {
                    let shell = create_type
                        .type_name()
                        .and_then(|name| name.path())
                        .and_then(|path| self.find_shell_type(&path));
                    match shell {
                        Some(shell) => {
                            self.lower_function_path_ref(&path, Some(&[Some(Ty::Type(shell))]));
                        }
                        None => self.push_error(path.syntax(), SemaError::MissingShellType),
                    }
                }
                if let Some(path) = subtype_diff {
                    self.lower_function_path_ref(&path, Some(&[subtype, subtype]));
                }
            }
            Some(ast::CreateTypeKind::BaseType(base_type)) => {
                let options = self.lower_base_type_options(
                    base_type
                        .attribute_list()
                        .into_iter()
                        .flat_map(|list| list.attribute_options()),
                );
                element = options.element;
                subscript = options.subscript.subscript();
                generated_subscript = self.generated_subscript_results(create_type.syntax());
                category = Some(options.category);
                preferred = options.preferred;
            }
            Some(ast::CreateTypeKind::EnumType(_)) => kind = TypeKind::Enum,
            None => kind = TypeKind::Shell,
        }
        let Some(path) = create_type.type_name().and_then(|name| name.path()) else {
            return;
        };
        let range = match self
            .find_shell_type(&path)
            .filter(|_| kind != TypeKind::Shell)
        {
            Some(shell) => {
                self.finish_shell_type(shell, kind, create_type.syntax(), &path, columns)
            }
            None => self.alloc_type(kind, create_type.syntax(), &path, columns),
        };
        let Some(range) = range else {
            return;
        };
        if let Some(ast::CreateTypeKind::EnumType(enum_type)) = create_type.kind() {
            for variant in enum_type
                .variant_list()
                .iter()
                .flat_map(|list| list.variants())
            {
                if let Some(literal) = variant.literal()
                    && let Some(label) = self.add_enum_label(range, &literal, LabelPosition::End)
                {
                    self.record_comment(variant.syntax(), Item::EnumLabel(label));
                }
            }
        }
        if !matches!(
            create_type.kind(),
            Some(ast::CreateTypeKind::CompositeType(_))
        ) {
            self.depend_on_refs(refs, IdxRange::single(range), Object::Type);
        }
        let subscript_results =
            generated_subscript.unwrap_or(subscript_results(range, element, subscript));
        let types = &mut self.index.types;
        types.set_element(range, element, subtype);
        types.set_subscript(
            range,
            subscript,
            subscript_results.result,
            subscript_results.slice,
        );
        types.set_category(range, category, preferred);
        if let Some(multirange_name) = multirange_name {
            self.alloc_multirange_type(range, multirange_name);
        }
    }

    fn generated_subscript_results(&mut self, node: &SyntaxNode) -> Option<SubscriptResults> {
        let mut result = None;
        let mut slice = None;
        for element in std::iter::successors(node.prev_sibling_or_token(), |element| {
            element.prev_sibling_or_token()
        }) {
            let rowan::NodeOrToken::Token(token) = element else {
                break;
            };
            match token.kind() {
                SyntaxKind::WHITESPACE => {}
                SyntaxKind::COMMENT => {
                    let text = token.text();
                    if let Some(ty) = text.strip_prefix("-- subscript slice: ") {
                        slice = self.generated_type(ty);
                    } else if let Some(ty) = text.strip_prefix("-- subscript: ") {
                        result = Some(self.generated_type(ty));
                    }
                }
                _ => break,
            }
        }
        result.map(|result| SubscriptResults { result, slice })
    }

    fn generated_type(&mut self, text: &str) -> Option<Ty> {
        let (text, array) = match text.strip_suffix("[]") {
            Some(text) => (text, true),
            None => (text, false),
        };
        let IdentPart {
            ident: schema,
            rest,
        } = split_ident(text)?;
        let IdentPart { ident: name, rest } = split_ident(rest.strip_prefix('.')?)?;
        if !rest.is_empty() {
            return None;
        }
        let schema = self.intern(&schema);
        let name = self.intern(&name);
        let ty = Ty::Type(self.find_type_in(schema, name)?);
        Some(if array { ty.array() } else { ty })
    }

    fn find_shell_type(&mut self, path: &ast::Path) -> Option<TypeId> {
        let segment = path.segment()?;
        let name = self.intern(&segment.text());
        let schema = self.creation_schema(path);
        let id = self.find_type_in(schema, name)?;
        (self.index.types.is_local(id) && self.index.types[id].kind == TypeKind::Shell)
            .then_some(id)
    }

    fn finish_shell_type(
        &mut self,
        id: TypeId,
        kind: TypeKind,
        node: &SyntaxNode,
        path: &ast::Path,
        columns: Option<Columns>,
    ) -> Option<TypeId> {
        let segment = path.segment()?;
        let ty = &self.index.types[id];
        let shell = self.index.types.replace(
            id,
            Type {
                kind,
                schema: ty.schema,
                name: ty.name,
                ptr: InIrFile::new(self.file_id, SyntaxNodePtr::new(node)),
                name_ptr: InIrFile::new(self.file_id, AstPtr::new(&segment)),
                shell_name_ptr: Some(ty.name_ptr),
                columns,
                base: None,
                element: None,
                subtype: None,
                range: None,
                subscript: None,
                subscript_result: None,
                slice_result: None,
                category: None,
                preferred: false,
            },
            &self.index.columns,
        );
        if self.in_transaction() {
            self.undo.push(Undo::FinishShellType(id, Box::new(shell)));
        }
        if let Some(columns) = columns {
            self.index
                .set_column_owner(columns.range, ColumnOwner::Type(id));
        }
        self.record_comment(node, Item::Type(id));
        self.add_item(segment.syntax(), Item::Type(id));
        Some(id)
    }

    fn require_enum_type(&mut self, ty: TypeId, path: &ast::PathRef) -> bool {
        if self.index.types[ty].kind == TypeKind::Enum {
            return true;
        }
        if let Some(segment) = path.segment() {
            let name = self.index.types[ty].name;
            self.push_error(segment.syntax(), SemaError::NotEnum { name });
        }
        false
    }

    fn add_enum_label(
        &mut self,
        ty: TypeId,
        literal: &ast::Literal,
        position: LabelPosition,
    ) -> Option<EnumLabelId> {
        let value = literal.string_value()?;
        let name = self.intern(&value);
        if self.find_enum_label(ty, name).is_some() {
            self.report_duplicate_enum_label(literal.syntax(), name);
            return None;
        }
        let id = self.index.enum_labels.alloc(EnumLabel {
            ty,
            name,
            name_ptr: self.ptr(literal.syntax()),
        });
        let labels = self.index.type_enum_labels.entry(ty).or_default();
        let at = match position {
            LabelPosition::End => None,
            LabelPosition::Before(neighbor) => labels.iter().position(|label| *label == neighbor),
            LabelPosition::After(neighbor) => labels
                .iter()
                .position(|label| *label == neighbor)
                .map(|at| at + 1),
        };
        labels.insert(at.unwrap_or(labels.len()), id);
        self.add_item(literal.syntax(), Item::EnumLabel(id));
        Some(id)
    }

    fn find_enum_label(&self, ty: TypeId, name: Name) -> Option<EnumLabelId> {
        self.index
            .enum_labels(ty)
            .find(|id| self.index.enum_labels[*id].name == name)
    }

    fn lower_enum_label_ref(
        &mut self,
        ty: Option<TypeId>,
        literal: &ast::Literal,
    ) -> Option<EnumLabelId> {
        let name = self.intern(&literal.string_value()?);
        let label = ty.and_then(|ty| self.find_enum_label(ty, name));
        let resolution = match (label, ty) {
            (Some(label), _) => Resolution::EnumLabel(label),
            (None, Some(_)) => Resolution::Unresolved,
            (None, None) => Resolution::Unknown,
        };
        self.add_ref(name, ReferenceKind::EnumLabel, literal.syntax(), resolution);
        label
    }

    fn enum_type(&self, ty: Option<Ty>) -> Option<TypeId> {
        match self.index.base_type(ty?)? {
            Ty::Type(id) if self.index.types[id].kind == TypeKind::Enum => Some(id),
            Ty::Type(_) | Ty::Table(_) | Ty::Cte(_) | Ty::Array(_) => None,
        }
    }

    fn lower_enum_comparison(&mut self, other: Option<Ty>, expr: &ast::Expr) {
        let Some(ty) = self.enum_type(other) else {
            return;
        };
        match expr {
            ast::Expr::Literal(literal) => self.lower_enum_value(ty, literal),
            ast::Expr::TupleExpr(tuple) => {
                for expr in tuple.exprs() {
                    if let ast::Expr::Literal(literal) = expr {
                        self.lower_enum_value(ty, &literal);
                    }
                }
            }
            ast::Expr::ParenExpr(paren) => {
                if let Some(literal) = paren.expr().and_then(|expr| expr.as_literal()) {
                    self.lower_enum_value(ty, &literal);
                }
            }
            ast::Expr::ArrayExpr(_)
            | ast::Expr::BetweenExpr(_)
            | ast::Expr::BinExpr(_)
            | ast::Expr::CallExpr(_)
            | ast::Expr::CaseExpr(_)
            | ast::Expr::CastExpr(_)
            | ast::Expr::Collate(_)
            | ast::Expr::FieldExpr(_)
            | ast::Expr::IndexExpr(_)
            | ast::Expr::NameRef(_)
            | ast::Expr::PostfixExpr(_)
            | ast::Expr::PrefixExpr(_)
            | ast::Expr::SliceExpr(_) => {}
        }
    }

    fn lower_enum_value(&mut self, ty: TypeId, literal: &ast::Literal) {
        if !literal.is_string() {
            return;
        }
        let Some(value) = literal.string_value() else {
            return;
        };
        let name = self.intern(&value);
        let label = self.find_enum_label(ty, name);
        let resolution = label.map_or(Resolution::Unknown, Resolution::EnumLabel);
        self.add_ref(name, ReferenceKind::EnumLabel, literal.syntax(), resolution);
        if label.is_none() {
            let error = SemaError::InvalidEnumValue {
                ty: self.index.types[ty].name,
                value: name,
            };
            self.push_error(literal.syntax(), error);
        }
    }

    fn move_enum_label(&mut self, label: EnumLabelId, name: Name, name_node: &SyntaxNode) {
        let in_transaction = self.in_transaction();
        let name_ptr = self.ptr(name_node);
        let Some(current) = self.index.enum_labels.get_local_mut(label) else {
            return;
        };
        let previous = Undo::MoveEnumLabel(label, current.name, current.name_ptr);
        let previous_name = current.name_ptr.file_range();
        current.name = name;
        current.name_ptr = name_ptr;
        if in_transaction {
            self.undo.push(previous);
        }
        self.index
            .record_rename(Item::EnumLabel(label), previous_name, name_ptr.file_range());
        self.add_item(name_node, Item::EnumLabel(label));
    }

    fn report_duplicate_enum_label(&mut self, node: &SyntaxNode, name: Name) {
        self.report_duplicate(ObjectKind::EnumLabel, node, name);
    }

    fn require_composite_type(&mut self, ty: TypeId, path: &ast::PathRef) -> bool {
        if self.index.types[ty].columns.is_some() {
            return true;
        }
        if let Some(segment) = path.segment() {
            let name = self.index.types[ty].name;
            self.push_error(segment.syntax(), SemaError::NotComposite { name });
        }
        false
    }

    fn lower_alter_type(&mut self, alter_type: &ast::AlterType) {
        let refs = self.index.references.next_idx();
        let Some(path) = alter_type.type_name_ref().and_then(|name| name.path_ref()) else {
            return;
        };
        let ty = self.lower_type_path_ref(&path, IfMissing::Error, None);
        match alter_type.action() {
            Some(ast::AlterTypeAction::AlterTypeAttributeActionList(list)) => {
                let ty = ty.filter(|ty| self.require_composite_type(*ty, &path));
                for action in list.actions() {
                    self.lower_alter_type_attribute_action(ty, &action, path.syntax());
                }
            }
            Some(ast::AlterTypeAction::RenameAttribute(rename)) => {
                let ty = ty.filter(|ty| self.require_composite_type(*ty, &path));
                let column = rename
                    .composite_field_ref()
                    .and_then(|field| self.lower_type_attribute_ref(ty, &field, IfMissing::Error));
                if let (Some(ty), Some(column), Some(field)) =
                    (ty, column, rename.composite_field())
                    && let Some(from) = self.index.columns[column].name
                    && let Some(tables) = self.typed_tables_to_alter(
                        ty,
                        rename.drop_behavior().as_ref(),
                        path.syntax(),
                    )
                {
                    for table in tables {
                        self.rename_table_column(table, from, &field);
                    }
                    self.rename_type_attribute(ty, column, &field);
                }
            }
            Some(ast::AlterTypeAction::SetOptions(set_options)) => {
                let options = set_options
                    .attribute_list()
                    .map(|attributes| self.lower_base_type_options(attributes.attribute_options()));
                if let Some(ty) = ty
                    && let Some(options) = options
                {
                    match options.subscript {
                        SubscriptOption::Untyped | SubscriptOption::Handler(_) => {
                            self.set_type_subscript(ty, options.subscript.subscript());
                        }
                        SubscriptOption::Unset => {}
                    }
                }
                if let Some(ty) = ty {
                    self.depend_on_refs(refs, IdxRange::single(ty), Object::Type);
                }
            }
            Some(ast::AlterTypeAction::TypeRenameTo(rename)) => {
                if let Some(ty) = ty
                    && let Some(path) = rename.type_name().and_then(|name| name.path())
                {
                    self.rename_type(ty, &path);
                }
            }
            Some(ast::AlterTypeAction::SetSchema(set_schema)) => {
                self.set_type_schema(ty, &set_schema);
            }
            Some(ast::AlterTypeAction::OwnerTo(owner_to)) => self.lower_owner_to(&owner_to),
            Some(ast::AlterTypeAction::AddValue(add_value)) => {
                let ty = ty.filter(|ty| self.require_enum_type(*ty, &path));
                let position = match add_value.value_position() {
                    Some(ast::ValuePosition::BeforeValue(before)) => before
                        .literal()
                        .and_then(|literal| self.lower_enum_label_ref(ty, &literal))
                        .map_or(LabelPosition::End, LabelPosition::Before),
                    Some(ast::ValuePosition::AfterValue(after)) => after
                        .literal()
                        .and_then(|literal| self.lower_enum_label_ref(ty, &literal))
                        .map_or(LabelPosition::End, LabelPosition::After),
                    None => LabelPosition::End,
                };
                if let Some(ty) = ty
                    && let Some(literal) = add_value.literal()
                {
                    let exists = literal.string_value().is_some_and(|value| {
                        let name = self.intern(&value);
                        self.find_enum_label(ty, name).is_some()
                    });
                    if !(exists && add_value.if_not_exists().is_some()) {
                        self.add_enum_label(ty, &literal, position);
                    }
                }
            }
            Some(ast::AlterTypeAction::RenameValue(rename)) => {
                let ty = ty.filter(|ty| self.require_enum_type(*ty, &path));
                let label = rename
                    .from()
                    .and_then(|literal| self.lower_enum_label_ref(ty, &literal));
                if let Some(ty) = ty
                    && let Some(label) = label
                    && let Some(literal) = rename.to()
                    && let Some(value) = literal.string_value()
                {
                    let name = self.intern(&value);
                    if self.find_enum_label(ty, name).is_some() {
                        self.report_duplicate_enum_label(literal.syntax(), name);
                    } else {
                        self.move_enum_label(label, name, literal.syntax());
                    }
                }
            }
            None => {}
        }
    }

    fn lower_alter_type_attribute_action(
        &mut self,
        ty: Option<TypeId>,
        action: &ast::AlterTypeAttributeAction,
        node: &SyntaxNode,
    ) {
        match action {
            ast::AlterTypeAttributeAction::AddAttribute(add) => {
                let column_ty = add.ty().and_then(|column_ty| self.lower_type(&column_ty));
                if let Some(collate) = add.collate() {
                    self.lower_collate(&collate);
                }
                if let (Some(ty), Some(field)) = (ty, add.composite_field())
                    && let Some(tables) =
                        self.typed_tables_to_alter(ty, add.drop_behavior().as_ref(), node)
                {
                    for table in tables {
                        self.add_typed_table_column(table, &field, column_ty);
                    }
                    self.add_type_attribute(ty, &field, column_ty);
                }
            }
            ast::AlterTypeAttributeAction::DropAttribute(drop) => {
                let if_missing = IfMissing::from_if_exists(drop.if_exists().as_ref());
                if let Some(column) = drop
                    .composite_field_ref()
                    .and_then(|field| self.lower_type_attribute_ref(ty, &field, if_missing))
                    && let Some(ty) = ty
                    && let Some(name) = self.index.columns[column].name
                    && let Some(tables) =
                        self.typed_tables_to_alter(ty, drop.drop_behavior().as_ref(), node)
                {
                    for table in tables {
                        self.drop_table_column(table, name);
                    }
                    self.replace_type_attributes(ty, |columns, id| {
                        (id != column).then_some(columns[id])
                    });
                }
            }
            ast::AlterTypeAttributeAction::AlterAttribute(alter) => {
                let column = alter
                    .composite_field_ref()
                    .and_then(|field| self.lower_type_attribute_ref(ty, &field, IfMissing::Error));
                let column_ty = alter.ty().and_then(|column_ty| self.lower_type(&column_ty));
                if let Some(collate) = alter.collate() {
                    self.lower_collate(&collate);
                }
                if let (Some(ty), Some(column)) = (ty, column)
                    && let Some(name) = self.index.columns[column].name
                    && let Some(tables) =
                        self.typed_tables_to_alter(ty, alter.drop_behavior().as_ref(), node)
                {
                    for table in tables {
                        self.set_table_column_type(table, name, column_ty);
                    }
                    self.replace_type_attributes(ty, |columns, id| {
                        let mut copy = columns[id];
                        if id == column {
                            copy.ty = column_ty;
                        }
                        Some(copy)
                    });
                }
            }
        }
    }

    fn typed_tables(&self, ty: TypeId) -> impl Iterator<Item = TableId> {
        self.index
            .tables
            .of_type(ty)
            .filter(move |id| !self.dropped_tables.contains(id))
    }

    fn typed_tables_to_alter(
        &mut self,
        ty: TypeId,
        drop_behavior: Option<&ast::DropBehavior>,
        node: &SyntaxNode,
    ) -> Option<SmallVec<[TableId; 4]>> {
        let tables: SmallVec<[TableId; 4]> = self.typed_tables(ty).collect();
        if !tables.is_empty() && !matches!(drop_behavior, Some(ast::DropBehavior::Cascade(_))) {
            let name = self.index.types[ty].name;
            self.push_error(node, SemaError::TypedTableAlter { name });
            return None;
        }
        Some(tables)
    }

    fn set_table_of_type(&mut self, table: TableId, ty: Option<TypeId>) {
        let in_transaction = self.in_transaction();
        if !self.index.tables.is_local(table) {
            return;
        }
        let previous = self.index.tables.set_of_type(table, ty);
        if in_transaction {
            self.undo.push(Undo::TableOfType(table, previous));
        }
    }

    fn add_typed_table_column(
        &mut self,
        table: TableId,
        name_node: &impl NameLike,
        ty: Option<Ty>,
    ) {
        let name = self.intern(&name_node.text());
        let columns = self.index.tables[table].columns;
        if self.columns_contain(columns.range, name) {
            return;
        }
        let start = self.index.columns.next_idx();
        self.index.columns.alloc_copies(columns.range);
        self.index.columns.alloc(Column {
            name: Some(name),
            ptr: self.ptr(name_node.syntax()),
            ty,
            merge: None,
        });
        self.set_table_columns(table, self.index.columns.range_from(start));
    }

    fn set_table_column_type(&mut self, table: TableId, column: Name, ty: Option<Ty>) {
        let columns = self.index.tables[table].columns;
        let range = self.copy_columns(columns.range, |columns, id| {
            let mut copy = columns[id];
            if copy.name == Some(column) {
                copy.ty = ty;
            }
            Some(copy)
        });
        self.set_table_columns(table, range);
    }

    fn lower_type_attribute_ref(
        &mut self,
        ty: Option<TypeId>,
        field: &impl NameLike,
        if_missing: IfMissing,
    ) -> Option<ColumnId> {
        let name = self.intern(&field.text());
        let columns = ty.and_then(|ty| self.index.types[ty].columns);
        let column = columns.and_then(|columns| self.find_column_in(columns.range, name));
        let resolution = match (column, columns) {
            (Some(column), _) => Resolution::Column {
                source: None,
                column,
            },
            (None, Some(columns)) if columns.complete => match if_missing {
                IfMissing::Ignore => Resolution::Unknown,
                IfMissing::Error => Resolution::Unresolved,
            },
            (None, _) => Resolution::Unknown,
        };
        self.add_ref(name, ReferenceKind::Column, field.syntax(), resolution);
        column
    }

    fn type_attribute_exists(&self, ty: TypeId, name: Name) -> bool {
        self.index.types[ty]
            .columns
            .is_some_and(|columns| self.columns_contain(columns.range, name))
    }

    fn columns_contain(&self, range: IdxRange<Column>, name: Name) -> bool {
        self.find_column_in(range, name).is_some()
    }

    fn find_column_in(&self, range: IdxRange<Column>, name: Name) -> Option<ColumnId> {
        range
            .iter()
            .find(|id| self.index.columns[*id].name == Some(name))
    }

    fn has_duplicate_column(&self, range: IdxRange<Column>, name: Name) -> bool {
        range
            .iter()
            .filter(|id| self.index.columns[*id].name == Some(name))
            .nth(1)
            .is_some()
    }

    fn find_table_column(&self, table: TableId, name: Name) -> Option<ColumnId> {
        self.find_column_in(self.index.tables[table].columns.range, name)
    }

    fn report_duplicate_column(&mut self, relation: Name, name_node: &SyntaxNode, name: Name) {
        self.push_error(name_node, SemaError::DuplicateColumn { name, relation });
    }

    fn add_type_attribute(&mut self, ty: TypeId, name_node: &impl NameLike, column_ty: Option<Ty>) {
        let name = self.intern(&name_node.text());
        if self.type_attribute_exists(ty, name) {
            self.report_duplicate_column(self.index.types[ty].name, name_node.syntax(), name);
            return;
        }
        let Some(columns) = self.index.types[ty].columns else {
            return;
        };
        let start = self.index.columns.next_idx();
        self.index.columns.alloc_copies(columns.range);
        self.alloc_column_def(Column {
            name: Some(name),
            ptr: self.ptr(name_node.syntax()),
            ty: column_ty,
            merge: None,
        });
        let range = self.index.columns.range_from(start);
        self.set_type_columns(ty, range);
    }

    fn rename_type_attribute(&mut self, ty: TypeId, column: ColumnId, name_node: &impl NameLike) {
        let name = self.intern(&name_node.text());
        if self.type_attribute_exists(ty, name) {
            self.report_duplicate_column(self.index.types[ty].name, name_node.syntax(), name);
            return;
        }
        let Some(columns) = self.index.types[ty].columns else {
            return;
        };
        let range = self.rename_column_copy(columns.range, column, name, name_node.syntax());
        self.set_type_columns(ty, range);
    }

    fn replace_type_attributes(
        &mut self,
        ty: TypeId,
        f: impl FnMut(&LayeredArena<Column>, ColumnId) -> Option<Column>,
    ) {
        let Some(columns) = self.index.types[ty].columns else {
            return;
        };
        let range = self.copy_columns(columns.range, f);
        self.set_type_columns(ty, range);
    }

    fn copy_columns(
        &mut self,
        range: IdxRange<Column>,
        mut f: impl FnMut(&LayeredArena<Column>, ColumnId) -> Option<Column>,
    ) -> IdxRange<Column> {
        let start = self.index.columns.next_idx();
        for id in range {
            if let Some(column) = f(&self.index.columns, id) {
                self.index.columns.alloc(column);
            }
        }
        self.index.columns.range_from(start)
    }

    fn rename_column_copy(
        &mut self,
        range: IdxRange<Column>,
        column: ColumnId,
        name: Name,
        name_node: &SyntaxNode,
    ) -> IdxRange<Column> {
        let ptr = self.ptr(name_node);
        let range = self.copy_columns(range, |columns, id| {
            let mut copy = columns[id];
            if id == column {
                copy.name = Some(name);
                copy.ptr = ptr;
            }
            Some(copy)
        });
        if let Some(id) = self.find_column_in(range, name) {
            self.add_item(name_node, Item::Column(id));
        }
        range
    }

    fn set_type_columns(&mut self, ty: TypeId, range: IdxRange<Column>) {
        let in_transaction = self.in_transaction();
        if !self.index.types.is_local(ty) {
            return;
        }
        let Some(previous) = self.index.types[ty].columns else {
            return;
        };
        self.index
            .types
            .set_columns(ty, Some(Columns { range, ..previous }), &self.index.columns);
        if in_transaction {
            self.undo.push(Undo::TypeColumns(ty, previous));
        }
        self.index.set_column_owner(range, ColumnOwner::Type(ty));
    }

    fn set_type_subscript(&mut self, ty: TypeId, subscript: Option<Subscript>) {
        let in_transaction = self.in_transaction();
        if !self.index.types.is_local(ty) {
            return;
        }
        let current = &self.index.types[ty];
        let results = subscript_results(ty, current.element, subscript);
        let previous = Undo::TypeSubscript(
            ty,
            current.subscript,
            SubscriptResults {
                result: current.subscript_result,
                slice: current.slice_result,
            },
        );
        self.index
            .types
            .set_subscript(ty, subscript, results.result, results.slice);
        if in_transaction {
            self.undo.push(previous);
        }
    }

    fn lower_base_type_options(
        &mut self,
        options: impl Iterator<Item = ast::AttributeOption>,
    ) -> BaseTypeOptions {
        let mut element = None;
        let mut subscript = SubscriptOption::Unset;
        let mut category = TypeCategory::User;
        let mut preferred = false;
        for option in options {
            let Some(name) = attribute_name(&option) else {
                continue;
            };
            let value = option.attribute_value();
            match name.as_str() {
                "element" => {
                    element = value
                        .and_then(|value| value.ty())
                        .and_then(|ty| self.lower_type(&ty));
                }
                "like" => {
                    if let Some(ty) = value.and_then(|value| value.ty()) {
                        self.lower_type(&ty);
                    }
                }
                option @ ("input" | "output" | "receive" | "send" | "typmod_in" | "typmod_out"
                | "analyze" | "subscript") => {
                    if value.as_ref().is_some_and(|v| {
                        v.syntax()
                            .first_token()
                            .is_some_and(|token| token.kind() == SyntaxKind::NONE_KW)
                    }) {
                        if option == "subscript" {
                            subscript = SubscriptOption::Untyped;
                        }
                        continue;
                    }
                    let Some(ast::Type::PathType(path_type)) = value.and_then(|value| value.ty())
                    else {
                        continue;
                    };
                    let Some(path) = path_type.path_ref() else {
                        continue;
                    };
                    let function = self.lower_function_path_ref(&path, None);
                    if option == "subscript" {
                        subscript = function
                            .and_then(|id| {
                                let function = &self.index.functions[id];
                                if function.schema != self.pg_catalog {
                                    return None;
                                }
                                match self.index.names.resolve(function.name) {
                                    "array_subscript_handler" => Some(Subscript::Array),
                                    "raw_array_subscript_handler" => Some(Subscript::RawArray),
                                    _ => None,
                                }
                            })
                            .map_or(SubscriptOption::Untyped, SubscriptOption::Handler);
                    }
                }
                "category" => {
                    if let Some(code) = value
                        .and_then(|value| value.literal())
                        .and_then(|literal| literal.string_value())
                        .and_then(|code| match code.as_bytes() {
                            [code] => Some(*code),
                            _ => None,
                        })
                    {
                        category = TypeCategory::from_code(code);
                    }
                }
                "preferred" => {
                    preferred = value.is_none_or(|value| {
                        value.literal().and_then(|literal| literal.bool_value()) == Some(true)
                    });
                }
                _ => {}
            }
        }
        BaseTypeOptions {
            element,
            subscript,
            category,
            preferred,
        }
    }

    fn lower_create_domain(&mut self, create_domain: &ast::CreateDomain) {
        let refs = self.index.references.next_idx();
        let base = create_domain.ty().and_then(|ty| self.lower_type(&ty));
        let domain = create_domain
            .domain()
            .and_then(|domain| domain.path())
            .and_then(|path| {
                self.alloc_type(TypeKind::Domain, create_domain.syntax(), &path, None)
            });
        if let Some(domain) = domain {
            self.index.types.set_base(domain, base);
            self.depend_on_refs(refs, IdxRange::single(domain), Object::Type);
        }
        for qualifier in create_domain.domain_qualifiers() {
            match qualifier {
                ast::DomainQualifier::Collate(collate) => self.lower_collate(&collate),
                ast::DomainQualifier::Constraint(constraint) => {
                    self.lower_domain_constraint(domain, &constraint);
                }
            }
        }
    }

    fn lower_alter_domain(&mut self, alter_domain: &ast::AlterDomain) {
        let domain = alter_domain
            .domain_ref()
            .and_then(|domain| domain.path_ref())
            .and_then(|path| {
                self.lower_type_path_ref(&path, IfMissing::Error, Some(TypeKind::Domain))
            });
        let owner = domain.map(ConstraintOwner::Domain);
        let node = alter_domain.syntax();
        match alter_domain.action() {
            Some(ast::AlterDomainAction::AddConstraint(add_constraint)) => {
                if let Some(constraint) = add_constraint.constraint() {
                    self.lower_domain_constraint(domain, &constraint);
                }
            }
            Some(ast::AlterDomainAction::DropConstraint(drop_constraint)) => {
                if let Some(name_ref) = drop_constraint.constraint_name_ref()
                    && let Some(constraint) = self.lower_constraint_ref(
                        owner,
                        &name_ref,
                        IfMissing::from_if_exists(drop_constraint.if_exists().as_ref()),
                    )
                {
                    self.drop_constraint(constraint);
                }
            }
            Some(ast::AlterDomainAction::RenameConstraint(rename)) => {
                if let Some(name_ref) = rename.constraint_name_ref()
                    && let Some(constraint) =
                        self.lower_constraint_ref(owner, &name_ref, IfMissing::Error)
                    && let Some(name) = rename.constraint_name()
                {
                    self.rename_constraint(constraint, &name, node);
                }
            }
            Some(ast::AlterDomainAction::ValidateConstraint(validate)) => {
                if let Some(name_ref) = validate.constraint_name_ref() {
                    self.lower_constraint_ref(owner, &name_ref, IfMissing::Error);
                }
            }
            Some(ast::AlterDomainAction::SetDefault(set_default)) => {
                if let Some(expr) = set_default.expr() {
                    self.lower_expr_in(ExprContext::Default, &expr);
                }
            }
            Some(ast::AlterDomainAction::SetNotNull(_)) => {
                if let Some(domain) = domain {
                    self.add_domain_not_null_constraint(domain, node, None);
                }
            }
            Some(ast::AlterDomainAction::DropNotNull(_)) => {
                if let Some(owner) = owner
                    && let Some(constraint) = self.find_domain_not_null_constraint(owner)
                {
                    self.drop_constraint(constraint);
                }
            }
            Some(ast::AlterDomainAction::DomainRenameTo(rename)) => {
                if let Some(domain) = domain
                    && let Some(path) = rename.domain().and_then(|domain| domain.path())
                {
                    self.rename_type(domain, &path);
                }
            }
            Some(ast::AlterDomainAction::SetSchema(set_schema)) => {
                self.set_type_schema(domain, &set_schema);
            }
            Some(ast::AlterDomainAction::OwnerTo(owner_to)) => self.lower_owner_to(&owner_to),
            Some(ast::AlterDomainAction::DropDefault(_)) | None => {}
        }
    }

    fn lower_domain_constraint(&mut self, domain: Option<TypeId>, constraint: &ast::Constraint) {
        let what = match constraint {
            ast::Constraint::CheckConstraint(check) => {
                self.lower_domain_check_constraint(domain, check);
                return;
            }
            ast::Constraint::NotNullConstraint(not_null) => {
                if let Some(domain) = domain {
                    let name = not_null
                        .constraint_name_clause()
                        .and_then(|clause| clause.constraint_name());
                    self.add_domain_not_null_constraint(domain, not_null.syntax(), name);
                }
                return;
            }
            ast::Constraint::DefaultConstraint(default) => {
                self.lower_default_constraint(None, None, default);
                return;
            }
            ast::Constraint::NullConstraint(_) => return,
            ast::Constraint::PrimaryKeyConstraint(_) => "primary key constraints",
            ast::Constraint::UniqueConstraint(_) => "unique constraints",
            ast::Constraint::ForeignKeyConstraint(_) | ast::Constraint::ReferencesConstraint(_) => {
                "foreign key constraints"
            }
            ast::Constraint::ExcludeConstraint(_) => "exclusion constraints",
            ast::Constraint::GeneratedConstraint(_) => "generated columns",
        };
        self.push_error(
            constraint.syntax(),
            SemaError::InvalidDomainConstraint { what },
        );
    }

    fn lower_domain_check_constraint(
        &mut self,
        domain: Option<TypeId>,
        check: &ast::CheckConstraint,
    ) {
        let refs = self.index.references.next_idx();
        if let Some(expr) = check.expr() {
            self.with_domain_scope(domain, check.syntax(), |this| {
                this.lower_expr_in(ExprContext::CheckConstraint, &expr);
            });
        }
        let Some(domain) = domain else {
            return;
        };
        let owner = ConstraintOwner::Domain(domain);
        let name = match check
            .constraint_name_clause()
            .and_then(|clause| clause.constraint_name())
        {
            Some(name) => ConstraintName::Explicit(name),
            None => ConstraintName::Generated(self.choose_constraint_name(owner, None, "check")),
        };
        let constraint =
            self.alloc_constraint(ConstraintKind::Check, owner, check.syntax(), name, None);
        self.depend_on_refs(refs, IdxRange::single(constraint), Object::Constraint);
    }

    fn with_domain_scope(
        &mut self,
        domain: Option<TypeId>,
        node: &SyntaxNode,
        f: impl FnOnce(&mut Self),
    ) {
        let query = self.alloc_query(QueryKind::Constraint, node);
        self.push_scope(query);
        let columns = match domain {
            Some(domain) => {
                let name = self.intern("value");
                let ty = &self.index.types[domain];
                let column = Column {
                    name: Some(name),
                    ptr: ty.name_ptr.map(|ptr| ptr.syntax_node_ptr()),
                    ty: ty.base,
                    merge: None,
                };
                Columns {
                    range: self.index.columns.alloc_many([column]),
                    complete: true,
                }
            }
            None => self.unknown_columns(),
        };
        self.push_source(Source {
            name: None,
            kind: SourceKind::Other,
            ptr: self.ptr(node),
            alias_ptr: None,
            columns,
        });
        self.set_visible((0..1).into());
        f(self);
        self.scopes.pop();
    }

    fn add_domain_not_null_constraint(
        &mut self,
        domain: TypeId,
        node: &SyntaxNode,
        name: Option<ast::ConstraintName>,
    ) {
        let owner = ConstraintOwner::Domain(domain);
        if self.find_domain_not_null_constraint(owner).is_some() {
            return;
        }
        let name = match name {
            Some(name) => ConstraintName::Explicit(name),
            None => ConstraintName::Generated(self.choose_constraint_name(owner, None, "not_null")),
        };
        self.alloc_constraint(ConstraintKind::DomainNotNull, owner, node, name, None);
    }

    fn find_domain_not_null_constraint(&self, owner: ConstraintOwner) -> Option<ConstraintId> {
        self.index.constraints.owned_by(owner).rev().find(|id| {
            self.index.constraints[*id].kind == ConstraintKind::DomainNotNull
                && !self.dropped_constraints.contains(id)
        })
    }

    fn rename_constraint(
        &mut self,
        constraint: ConstraintId,
        name: &ast::ConstraintName,
        node: &SyntaxNode,
    ) {
        let Constraint { owner, index, .. } = self.index.constraints[constraint];
        let new_name = self.intern(&name.text());
        if self.find_constraint(owner, new_name).is_some() {
            self.report_duplicate_constraint(owner, name.syntax(), new_name);
            return;
        }
        if let Some(index) = index {
            let schema = self.index.indexes[index].schema;
            if self.report_relation_name_taken(schema, new_name, name.syntax()) {
                return;
            }
            self.move_index(index, new_name, name.syntax());
        }
        self.move_constraint(constraint, new_name, name.syntax(), node);
    }

    fn move_constraint(
        &mut self,
        constraint: ConstraintId,
        name: Name,
        name_node: &SyntaxNode,
        node: &SyntaxNode,
    ) {
        let Constraint {
            kind, owner, index, ..
        } = self.index.constraints[constraint];
        let moved = self.index.constraints.next_idx();
        if self.dropped_constraints.insert(constraint) && self.in_transaction() {
            self.undo.push(Undo::DropConstraint(constraint));
        }
        let id = self.index.constraints.alloc(Constraint {
            kind,
            name,
            owner,
            index,
            ptr: self.ptr(node),
            name_ptr: self.ptr(name_node),
        });
        self.add_item(name_node, Item::Constraint(id));
        self.record_moves(
            Object::Constraint(constraint),
            self.index.constraints.range_from(moved),
            Object::Constraint,
        );
    }

    fn move_index(&mut self, index: IndexId, name: Name, name_node: &SyntaxNode) {
        let in_transaction = self.in_transaction();
        let name_ptr = self.ptr(name_node);
        if !self.index.indexes.is_local(index) {
            return;
        }
        let current = &self.index.indexes[index];
        let schema = current.schema;
        let previous = Undo::MoveIndex(index, schema, current.name, current.name_ptr);
        let previous_name = current.name_ptr.file_range();
        self.index.indexes.move_to(index, schema, name, name_ptr);
        if in_transaction {
            self.undo.push(previous);
        }
        self.index
            .record_rename(Item::Index(index), previous_name, name_ptr.file_range());
        self.add_item(name_node, Item::Index(index));
    }

    fn rename_index(&mut self, index: IndexId, path: &ast::Path, node: &SyntaxNode) {
        let Some(segment) = path.segment() else {
            return;
        };
        let name = self.intern(&segment.text());
        let schema = self.index.indexes[index].schema;
        if self.report_relation_name_taken(schema, name, segment.syntax()) {
            return;
        }
        self.move_index(index, name, segment.syntax());
        let constraint = self
            .index
            .constraints
            .backed_by(index)
            .rev()
            .find(|id| !self.dropped_constraints.contains(id));
        if let Some(constraint) = constraint {
            self.move_constraint(constraint, name, segment.syntax(), node);
        }
    }

    fn rename_type(&mut self, ty: TypeId, path: &ast::Path) {
        let Some(segment) = path.segment() else {
            return;
        };
        let name = self.intern(&segment.text());
        let schema = self.index.types[ty].schema;
        if self.type_name_taken(schema, name) {
            self.report_duplicate(ObjectKind::Type, segment.syntax(), name);
            return;
        }
        let name_ptr = self.ast_ptr(&segment);
        self.move_type(ty, schema, name, name_ptr);
        self.add_item(segment.syntax(), Item::Type(ty));
    }

    fn set_type_schema(&mut self, ty: Option<TypeId>, set_schema: &ast::SetSchema) {
        let Some(schema_ref) = set_schema.schema_ref() else {
            return;
        };
        let Some(schema) = self.lower_set_schema_ref(&schema_ref) else {
            return;
        };
        let Some(ty) = ty else {
            return;
        };
        let name = self.index.types[ty].name;
        if self.type_name_taken(schema, name) {
            let error = SemaError::DuplicateInSchema {
                object: ObjectKind::Type,
                name,
                schema,
            };
            self.push_error(schema_ref.syntax(), error);
            return;
        }
        let Type { name, name_ptr, .. } = self.index.types[ty];
        self.move_type(ty, schema, name, name_ptr);
    }

    fn type_name_taken(&self, schema: Name, name: Name) -> bool {
        self.find_type_in(schema, name).is_some() || self.find_table(Some(schema), name).is_some()
    }

    fn move_type(
        &mut self,
        ty: TypeId,
        schema: Name,
        name: Name,
        name_ptr: InIrFile<AstPtr<ast::PathSegment>>,
    ) {
        let in_transaction = self.in_transaction();
        if !self.index.types.is_local(ty) {
            return;
        }
        let current = &self.index.types[ty];
        let previous = Undo::MoveType(ty, current.schema, current.name, current.name_ptr);
        let previous_name = current.name_ptr.file_range();
        self.index.types.move_to(ty, schema, name, name_ptr);
        if in_transaction {
            self.undo.push(previous);
        }
        self.index
            .record_rename(Item::Type(ty), previous_name, name_ptr.file_range());
    }

    fn lower_type(&mut self, ty: &ast::Type) -> Option<Ty> {
        self.lower_type_ref(ty, IfMissing::Error)
    }

    fn lower_type_ref(&mut self, ty: &ast::Type, if_missing: IfMissing) -> Option<Ty> {
        self.lower_type_lookup(ty, if_missing).ty()
    }

    fn lower_type_lookup(&mut self, ty: &ast::Type, if_missing: IfMissing) -> TypeLookup {
        let (node, schema, name): (_, _, Cow<str>) = match ty {
            ast::Type::ArrayType(array) => {
                let Some(element) = array.ty() else {
                    return TypeLookup::Unknown;
                };
                return match self.lower_type_lookup(&element, if_missing) {
                    TypeLookup::Found(element) => TypeLookup::Found(element.array()),
                    lookup @ (TypeLookup::Missing | TypeLookup::Unknown) => lookup,
                };
            }
            ast::Type::PathType(path_type) => {
                let Some(path) = path_type.path_ref() else {
                    return TypeLookup::Unknown;
                };
                let Some(segment) = path.segment() else {
                    return TypeLookup::Unknown;
                };
                let qualifier = path.qualifier();
                let (schema, name) = match path_type.keyword_type_name() {
                    Some(name) => (Some(self.pg_catalog), name.into()),
                    None => (
                        self.lower_schema_qualifier(qualifier),
                        segment.text().into(),
                    ),
                };
                (segment.syntax().clone(), schema, name)
            }
            ast::Type::BitType(_) => (ty.syntax().clone(), Some(self.pg_catalog), "bit".into()),
            ast::Type::BitVaryingType(_) => {
                (ty.syntax().clone(), Some(self.pg_catalog), "varbit".into())
            }
            ast::Type::CharacterType(_) => {
                (ty.syntax().clone(), Some(self.pg_catalog), "bpchar".into())
            }
            ast::Type::DoubleType(_) => {
                (ty.syntax().clone(), Some(self.pg_catalog), "float8".into())
            }
            ast::Type::IntervalType(_) => (
                ty.syntax().clone(),
                Some(self.pg_catalog),
                "interval".into(),
            ),
            ast::Type::TimeType(time) => {
                let name = match time.timezone() {
                    Some(ast::Timezone::WithTimezone(_)) => "timetz",
                    Some(ast::Timezone::WithoutTimezone(_)) | None => "time",
                };
                (ty.syntax().clone(), Some(self.pg_catalog), name.into())
            }
            ast::Type::TimestampType(timestamp) => {
                let name = match timestamp.timezone() {
                    Some(ast::Timezone::WithTimezone(_)) => "timestamptz",
                    Some(ast::Timezone::WithoutTimezone(_)) | None => "timestamp",
                };
                (ty.syntax().clone(), Some(self.pg_catalog), name.into())
            }
            ast::Type::VarcharType(_) => {
                (ty.syntax().clone(), Some(self.pg_catalog), "varchar".into())
            }
        };
        self.lookup_type_name(&node, schema, &name, if_missing)
    }

    fn lower_column_type(&mut self, ty: &ast::Type) -> Option<Ty> {
        if let Some(name) = ty.serial_type_name()
            && let Some(segment) = ty.path_ref().and_then(|path| path.segment())
        {
            return self
                .lookup_type_name(
                    segment.syntax(),
                    Some(self.pg_catalog),
                    name,
                    IfMissing::Error,
                )
                .ty();
        }
        self.lower_type(ty)
    }

    fn lookup_type_name(
        &mut self,
        node: &SyntaxNode,
        schema: Option<Name>,
        name: &str,
        if_missing: IfMissing,
    ) -> TypeLookup {
        let name = self.intern(name);
        let resolution = match (self.find_type(schema, name), if_missing) {
            (Resolution::Unresolved, IfMissing::Ignore) => Resolution::Unknown,
            (resolution, IfMissing::Error | IfMissing::Ignore) => resolution,
        };
        self.add_ref(name, ReferenceKind::Type, node, resolution);
        match resolution {
            Resolution::Type(ty) => TypeLookup::Found(Ty::Type(ty)),
            Resolution::Table(table) => TypeLookup::Found(Ty::Table(table)),
            Resolution::Unresolved => TypeLookup::Missing,
            Resolution::Column { .. }
            | Resolution::Source(_)
            | Resolution::Cte(_)
            | Resolution::Function(_)
            | Resolution::Param(_)
            | Resolution::Window(_)
            | Resolution::JsonPath(_)
            | Resolution::Savepoint(_)
            | Resolution::Cursor(_)
            | Resolution::PreparedTransaction(_)
            | Resolution::PreparedStatement(_)
            | Resolution::Channel(_)
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
            | Resolution::Unknown
            | Resolution::Ambiguous => TypeLookup::Unknown,
        }
    }

    fn builtin_type(&mut self, name: &str) -> Option<Ty> {
        let name = self.intern(name);
        self.find_type_in(self.pg_catalog, name).map(Ty::Type)
    }

    fn literal_type(&mut self, literal: &ast::Literal) -> Option<Ty> {
        let name = match literal.kind()? {
            ast::LitKind::IntNumber(_) => match literal.integer_value() {
                Some(value) if i32::try_from(&value).is_ok() => "int4",
                Some(value) if i64::try_from(&value).is_ok() => "int8",
                Some(_) => "numeric",
                None => "int4",
            },
            ast::LitKind::NumericNumber(_) => "numeric",
            ast::LitKind::String(_)
            | ast::LitKind::DollarQuotedString(_)
            | ast::LitKind::EscString(_)
            | ast::LitKind::NationalString(_)
            | ast::LitKind::UnicodeEscString(_) => "text",
            ast::LitKind::BitString(_) | ast::LitKind::ByteString(_) => "bit",
            ast::LitKind::True(_) | ast::LitKind::False(_) => "bool",
            ast::LitKind::Default(_)
            | ast::LitKind::Null(_)
            | ast::LitKind::Off(_)
            | ast::LitKind::On(_)
            | ast::LitKind::PositionalParam(_) => return None,
        };
        self.builtin_type(name)
    }

    fn lower_func_type(&mut self, func_type: Option<ast::FuncType>) -> Option<Ty> {
        self.lower_func_type_lookup(func_type).ty()
    }

    fn lower_func_type_lookup(&mut self, func_type: Option<ast::FuncType>) -> TypeLookup {
        match func_type {
            Some(ast::FuncType::Type(ty)) => self.lower_type_lookup(&ty, IfMissing::Error),
            Some(ast::FuncType::PercentType(percent_type)) => {
                match self.percent_type(&percent_type) {
                    Some(ty) => TypeLookup::Found(ty),
                    None => TypeLookup::Unknown,
                }
            }
            None => TypeLookup::Unknown,
        }
    }

    fn percent_type(&mut self, percent_type: &ast::PercentType) -> Option<Ty> {
        let path = percent_type.path_ref()?;
        let column = self
            .lower_qualified_column_ref(&path, TableLookup::Any)
            .column?;
        let ty = self.index.columns[column].ty?;
        if percent_type.array_token().is_some() || percent_type.array_bounds().next().is_some() {
            Some(ty.array())
        } else {
            Some(ty)
        }
    }

    fn lower_type_path_ref(
        &mut self,
        path: &ast::PathRef,
        if_missing: IfMissing,
        kind: Option<TypeKind>,
    ) -> Option<TypeId> {
        let segment = path.segment()?;
        let name = self.intern(&segment.text());
        let schema = self.lower_schema_qualifier(path.qualifier());
        let found = self
            .search_path(schema, Lookup::Type)
            .find_map(|schema| self.find_type_in(schema, name));
        let resolution = match found {
            Some(ty) => Resolution::Type(ty),
            None => match if_missing {
                IfMissing::Ignore => Resolution::Unknown,
                IfMissing::Error => Resolution::Unresolved,
            },
        };
        self.add_ref(name, ReferenceKind::Type, segment.syntax(), resolution);
        let ty = found?;
        if let Some(kind) = kind
            && self.index.types[ty].kind != kind
        {
            let error = SemaError::WrongTypeKind {
                name,
                expected: kind,
            };
            self.push_error(segment.syntax(), error);
            return None;
        }
        Some(ty)
    }

    fn drop_type(&mut self, ty: TypeId) {
        let in_transaction = self.in_transaction();
        for id in self.companion_types(ty) {
            if self.dropped_types.insert(id) && in_transaction {
                self.undo.push(Undo::DropType(id));
            }
        }
    }

    fn find_type(&self, schema: Option<Name>, name: Name) -> Resolution {
        for schema in self.search_path(schema, Lookup::Type) {
            if let Some(ty) = self.find_type_in(schema, name) {
                return Resolution::Type(ty);
            }
            if let Some(table) = self.find_table(Some(schema), name) {
                return Resolution::Table(table);
            }
        }
        Resolution::Unresolved
    }

    fn find_type_in(&self, schema: Name, name: Name) -> Option<TypeId> {
        self.index.types.named(schema, name).find(|id| {
            let ty = &self.index.types[*id];
            self.visible(ty.schema, ty.ptr.file_id) && !self.dropped_types.contains(id)
        })
    }

    fn lower_create_function(&mut self, create_function: &ast::CreateFunction) {
        let refs = self.index.references.next_idx();
        let Some(path) = create_function.name().and_then(|name| name.path()) else {
            return;
        };
        let params = self.alloc_params(create_function.param_list());
        let Some(id) = self.alloc_function(
            FunctionKind::Function,
            IfExists::from_or_replace(create_function.or_replace().as_ref()),
            create_function.syntax(),
            &path,
            params,
            create_function.ret_type().as_ref(),
        ) else {
            return;
        };
        if create_function.option_list().is_some_and(|options| {
            options
                .options()
                .any(|option| matches!(option, ast::FuncOption::WindowFuncOption(_)))
        }) {
            self.index.functions.set_window(id);
        }
        self.lower_routine_body(
            id,
            refs,
            create_function.option_list(),
            create_function.body(),
            create_function.sql_body(),
        );
    }

    fn lower_create_procedure(&mut self, create_procedure: &ast::CreateProcedure) {
        let refs = self.index.references.next_idx();
        let Some(path) = create_procedure.name().and_then(|name| name.path()) else {
            return;
        };
        let params = self.alloc_params(create_procedure.param_list());
        let Some(id) = self.alloc_function(
            FunctionKind::Procedure,
            IfExists::from_or_replace(create_procedure.or_replace().as_ref()),
            create_procedure.syntax(),
            &path,
            params,
            None,
        ) else {
            return;
        };
        self.lower_routine_body(
            id,
            refs,
            create_procedure.option_list(),
            create_procedure.body(),
            create_procedure.sql_body(),
        );
    }

    fn lower_routine_body(
        &mut self,
        id: FunctionId,
        refs: RefId,
        options: Option<ast::FuncOptionList>,
        body: Option<ast::RoutineBody>,
        sql_body: Option<SqlBody>,
    ) {
        let search_path = options
            .and_then(|options| self.lower_func_options(&options))
            .unwrap_or_else(|| self.search_path.clone());
        let outer = self.function.replace(id);
        let has_body = body.is_some();
        if let Some(body) = body {
            let range = body.syntax().text_range();
            self.with_search_path(&search_path, range, |this| match body {
                ast::RoutineBody::ReturnStmt(return_stmt) => this.lower_return_stmt(&return_stmt),
                ast::RoutineBody::AtomicBody(body) => {
                    for stmt in body.routine_body_stmts() {
                        match stmt {
                            ast::RoutineBodyStmt::ReturnStmt(return_stmt) => {
                                this.lower_return_stmt(&return_stmt);
                            }
                            ast::RoutineBodyStmt::Stmt(stmt) => this.lower_stmt(&stmt),
                        }
                    }
                }
            });
        }
        self.depend_on_refs(refs, IdxRange::single(id), Object::Function);
        if !has_body && let Some(sql_body) = sql_body {
            let range = sql_body.source_range(TextRange::up_to(TextSize::of(sql_body.text())));
            self.with_search_path(&search_path, range, |this| this.lower_sql_body(sql_body));
        }
        self.function = outer;
    }

    fn with_search_path(
        &mut self,
        search_path: &[Name],
        range: TextRange,
        f: impl FnOnce(&mut Self),
    ) {
        let outer_session = std::mem::replace(&mut self.session_search_path, search_path.into());
        let outer = std::mem::replace(&mut self.search_path, search_path.into());
        self.record_search_path(range.start());
        f(self);
        self.search_path = outer;
        self.session_search_path = outer_session;
        self.record_search_path(range.end());
    }

    fn lower_sql_body(&mut self, sql: SqlBody) {
        let tree = sql.tree();
        let body = self.index.bodies.alloc(Body {
            parent: self.file_id,
            sql,
        });
        let outer = std::mem::replace(&mut self.file_id, IrFileId::Body(body));
        let outer_transaction = self.transaction.take();
        let outer_savepoints = std::mem::take(&mut self.savepoints);
        let outer_undo = std::mem::take(&mut self.undo);
        for stmt in tree.stmts() {
            self.lower_stmt(&stmt);
        }
        self.file_id = outer;
        self.transaction = outer_transaction;
        self.savepoints = outer_savepoints;
        self.undo = outer_undo;
    }

    fn lower_set_config_param(
        &mut self,
        param: &ast::SetConfigParam,
    ) -> Option<SmallVec<[Name; 4]>> {
        if !is_search_path(param.config_parameter_ref()) {
            return None;
        }
        self.lower_search_path(param.config_assignment()?)
    }

    fn lower_set(&mut self, set: &ast::Set) {
        let scope = match set.set_scope() {
            Some(ast::SetScope::LocalScope(_)) => SearchPathScope::Transaction,
            Some(ast::SetScope::SessionScope(_)) | None => SearchPathScope::Session,
        };
        let path = match set.set_target() {
            Some(ast::SetTarget::SetConfig(config))
                if is_search_path(config.config_parameter_ref()) =>
            {
                config
                    .config_assignment()
                    .and_then(|assignment| self.lower_search_path(assignment))
            }
            Some(ast::SetTarget::SetSchemaValue(value)) => value.literal().and_then(|literal| {
                let schema = literal.string_value()?;
                Some(smallvec![
                    self.lower_search_path_schema(literal.syntax(), &schema),
                ])
            }),
            Some(ast::SetTarget::SetCatalog(catalog)) => {
                self.push_error(catalog.syntax(), SemaError::CannotChangeDatabase);
                None
            }
            Some(
                ast::SetTarget::SetConfig(_)
                | ast::SetTarget::SetTimeZone(_)
                | ast::SetTarget::SetXmlOption(_),
            )
            | None => None,
        };
        if let Some(path) = path {
            self.set_search_path(path, scope);
        }
    }

    fn lower_reset(&mut self, reset: &ast::Reset) {
        let reset_search_path = match reset.reset_target() {
            Some(ast::ResetTarget::All(_)) => true,
            Some(ast::ResetTarget::ConfigParameterRef(param)) => is_search_path(Some(param)),
            Some(
                ast::ResetTarget::ResetTimeZone(_) | ast::ResetTarget::ResetTransactionIsolation(_),
            )
            | None => false,
        };
        if reset_search_path {
            self.set_search_path(self.default_search_path(), SearchPathScope::Session);
        }
    }

    fn lower_search_path(
        &mut self,
        assignment: ast::ConfigAssignment,
    ) -> Option<SmallVec<[Name; 4]>> {
        let ast::ConfigAssignment::ToConfigValue(value) = assignment else {
            return None;
        };
        if value.default_token().is_some() {
            return Some(self.default_search_path());
        }
        let mut path = smallvec![];
        for value in value.config_values() {
            let (node, schema) = match value {
                ast::ConfigValue::ConfigValueName(name) => (name.syntax().clone(), name.text()),
                ast::ConfigValue::Literal(literal) => match literal.string_value() {
                    Some(schema) => (literal.syntax().clone(), schema),
                    None => continue,
                },
                ast::ConfigValue::PrefixExpr(_) => continue,
            };
            if !schema.is_empty() {
                path.push(self.lower_search_path_schema(&node, &schema));
            }
        }
        Some(path)
    }

    fn lower_search_path_schema(&mut self, node: &SyntaxNode, text: &str) -> Name {
        let name = self.intern_schema(text);
        let resolution = self
            .find_schema(name)
            .map_or(Resolution::Unknown, Resolution::Schema);
        self.add_ref(name, ReferenceKind::Schema, node, resolution);
        name
    }

    fn lower_set_config(&mut self, call: &ast::CallExpr) {
        let mut args = call
            .arg_list()
            .into_iter()
            .flat_map(|list| list.args())
            .map(|arg| arg.expr().and_then(|expr| expr.as_literal()));
        let (Some(Some(param)), Some(Some(value)), Some(Some(local)), None) =
            (args.next(), args.next(), args.next(), args.next())
        else {
            return;
        };
        if !param
            .string_value()
            .is_some_and(|param| param.eq_ignore_ascii_case("search_path"))
        {
            return;
        }
        let scope = match local.kind() {
            Some(ast::LitKind::True(_)) => SearchPathScope::Transaction,
            Some(ast::LitKind::False(_)) => SearchPathScope::Session,
            Some(
                ast::LitKind::BitString(_)
                | ast::LitKind::ByteString(_)
                | ast::LitKind::Default(_)
                | ast::LitKind::DollarQuotedString(_)
                | ast::LitKind::EscString(_)
                | ast::LitKind::IntNumber(_)
                | ast::LitKind::NationalString(_)
                | ast::LitKind::Null(_)
                | ast::LitKind::NumericNumber(_)
                | ast::LitKind::Off(_)
                | ast::LitKind::On(_)
                | ast::LitKind::PositionalParam(_)
                | ast::LitKind::String(_)
                | ast::LitKind::UnicodeEscString(_),
            )
            | None => return,
        };
        let Some(path) = value
            .string_value()
            .and_then(|value| postgres::split_identifier_string(&value, ','))
        else {
            return;
        };
        let path = path
            .iter()
            .map(|identifier| self.intern_schema(&identifier.name))
            .collect();
        self.set_search_path(path, scope);
    }

    fn set_search_path(&mut self, path: SmallVec<[Name; 4]>, scope: SearchPathScope) {
        if let SearchPathScope::Transaction = scope {
            if self.transaction.is_some() {
                self.search_path = path;
            }
            return;
        }
        self.session_search_path.clone_from(&path);
        self.search_path = path;
    }

    fn reset_search_path(&mut self) {
        self.search_path = self.default_search_path();
        self.session_search_path = self.default_search_path();
    }

    fn default_search_path(&self) -> SmallVec<[Name; 4]> {
        smallvec![self.dollar_user, self.public]
    }

    fn intern(&mut self, text: &str) -> Name {
        self.index.names.intern(postgres::truncate_identifier(text))
    }

    fn intern_schema(&mut self, schema: &str) -> Name {
        if is_temp_schema_alias(schema) {
            return self.pg_temp;
        }
        self.intern(schema)
    }

    fn creation_schema(&mut self, path: &ast::Path) -> Name {
        self.lower_schema_qualifier(path.qualifier())
            .unwrap_or_else(|| self.default_creation_schema())
    }

    fn lower_schema_qualifier(&mut self, qualifier: Option<ast::PathRef>) -> Option<Name> {
        let segment = qualifier?.segment()?;
        Some(self.lower_schema_ref(segment.syntax(), &segment.text()))
    }

    fn lower_set_schema_ref(&mut self, schema_ref: &ast::SchemaRef) -> Option<Name> {
        let name = self.lower_schema_ref(schema_ref.syntax(), &schema_ref.text());
        self.find_schema(name).map(|_| name)
    }

    fn lower_schema_ref(&mut self, node: &SyntaxNode, text: &str) -> Name {
        let name = self.intern_schema(text);
        let resolution = self
            .find_schema(name)
            .map_or(Resolution::Unresolved, Resolution::Schema);
        self.add_ref(name, ReferenceKind::Schema, node, resolution);
        name
    }

    fn find_schema(&self, name: Name) -> Option<SchemaId> {
        self.index
            .schemas
            .get(name)
            .find(|id| !self.dropped_schemas.contains(id))
    }

    fn default_creation_schema(&self) -> Name {
        self.search_path
            .iter()
            .copied()
            .find(|schema| *schema != self.dollar_user && *schema != self.pg_temp)
            .unwrap_or(self.public)
    }

    fn same_signature(
        &self,
        a: (FunctionKind, IdxRange<Param>),
        b: (FunctionKind, IdxRange<Param>),
    ) -> bool {
        let signature_types = |(kind, params): (FunctionKind, IdxRange<Param>)| {
            params
                .iter()
                .map(|id| &self.index.params[id])
                .filter(move |param| {
                    kind == FunctionKind::Procedure || param.mode != ParamMode::Out
                })
                .map(|param| param.ty)
        };
        let mut a = signature_types(a);
        let mut b = signature_types(b);
        loop {
            match (a.next(), b.next()) {
                (None, None) => return true,
                (Some(Some(a)), Some(Some(b))) if a == b => {}
                _ => return false,
            }
        }
    }

    fn lower_return_stmt(&mut self, return_stmt: &ast::ReturnStmt) {
        if let Some(expr) = return_stmt.expr() {
            self.lower_expr(&expr);
        }
    }

    fn param_def(&mut self, param: &ast::Param) -> Param {
        let param_name = param.name();
        let mode = match param.mode() {
            None | Some(ast::ParamMode::ParamIn(_)) => ParamMode::In,
            Some(ast::ParamMode::ParamOut(_)) => ParamMode::Out,
            Some(ast::ParamMode::ParamInOut(_)) => ParamMode::InOut,
            Some(ast::ParamMode::ParamVariadic(_)) => ParamMode::Variadic,
        };
        let ty = self.lower_func_type(param.func_type());
        let default = param.param_default();
        if let Some(expr) = default.as_ref().and_then(|default| default.expr()) {
            self.lower_expr_in(ExprContext::Default, &expr);
        }
        Param {
            name: param_name.as_ref().map(|name| self.intern(&name.text())),
            mode,
            ty,
            has_default: default.is_some(),
            ptr: self.ptr(param.syntax()),
            name_ptr: param_name.map(|name| self.ast_ptr(&name)),
        }
    }

    fn out_param_columns(&mut self, params: IdxRange<Param>) -> Option<Columns> {
        let range = self.index.columns.alloc_many(
            params
                .iter()
                .map(|id| &self.index.params[id])
                .filter(|param| matches!(param.mode, ParamMode::Out | ParamMode::InOut))
                .map(|param| Column {
                    name: param.name,
                    ptr: param
                        .name_ptr
                        .map_or(param.ptr, |ptr| ptr.syntax_node_ptr()),
                    ty: param.ty,
                    merge: None,
                }),
        );
        (!range.is_empty()).then_some(Columns {
            range,
            complete: true,
        })
    }

    fn lower_query(&mut self, query: ast::SelectVariant) -> QueryId {
        self.lower_cte_query(query, None)
    }

    fn lower_cte_query(
        &mut self,
        query: ast::SelectVariant,
        recursive_cte: Option<CteId>,
    ) -> QueryId {
        self.with_expr_context(ExprContext::Unrestricted, |this| {
            this.lower_cte_query_inner(query, recursive_cte)
        })
    }

    fn lower_cte_query_inner(
        &mut self,
        query: ast::SelectVariant,
        recursive_cte: Option<CteId>,
    ) -> QueryId {
        match query {
            ast::SelectVariant::Select(select) => self.lower_simple_select(&select),
            ast::SelectVariant::SelectInto(select) => self.lower_select_into(&select),
            ast::SelectVariant::ParenSelect(paren) => {
                self.lower_paren_select(&paren, recursive_cte)
            }
            ast::SelectVariant::CompoundSelect(compound) => {
                self.lower_compound(&compound, recursive_cte)
            }
            ast::SelectVariant::Values(values) => self.lower_values(&values),
            ast::SelectVariant::Table(table) => self.lower_table_query(&table),
        }
    }

    fn lower_simple_select(&mut self, select: &ast::Select) -> QueryId {
        let in_compound = select.in_compound();
        self.lower_select(
            self.ptr(select.syntax()),
            SelectParts {
                with_clause: select.with_clause().filter(|_| !in_compound),
                select_clause: select.select_clause(),
                from_clause: select.from_clause(),
                where_clause: select.where_clause(),
                group_by_clause: select.group_by_clause(),
                having_clause: select.having_clause(),
                window_clause: select.window_clause(),
                tail_clauses: select.tail_clauses(),
            },
        )
    }

    fn lower_select_into(&mut self, select: &ast::SelectInto) -> QueryId {
        self.lower_select(
            self.ptr(select.syntax()),
            SelectParts {
                with_clause: select.with_clause(),
                select_clause: select.select_clause(),
                from_clause: select.from_clause(),
                where_clause: select.where_clause(),
                group_by_clause: select.group_by_clause(),
                having_clause: select.having_clause(),
                window_clause: select.window_clause(),
                tail_clauses: select.tail_clauses(),
            },
        )
    }

    fn lower_paren_select(
        &mut self,
        paren: &ast::ParenSelect,
        recursive_cte: Option<CteId>,
    ) -> QueryId {
        let mark = self.ctes.len();
        if let Some(with_clause) = paren.with_clause() {
            self.lower_with(&with_clause);
        }
        let id = match paren.select() {
            Some(inner) => self.lower_cte_query(inner, recursive_cte),
            None => self.alloc_query(QueryKind::Select, paren.syntax()),
        };
        self.lower_tail_clauses_for_outputs(id, paren.tail_clauses());
        self.ctes.truncate(mark);
        id
    }

    fn lower_select(&mut self, ptr: InIrFile<SyntaxNodePtr>, parts: SelectParts) -> QueryId {
        let mark = self.ctes.len();
        if let Some(with_clause) = parts.with_clause {
            self.lower_with(&with_clause);
        }

        let id = self.alloc_query_ptr(QueryKind::Select, ptr);
        self.push_scope(id);

        for item in parts.from_clause.iter().flat_map(|clause| clause.items()) {
            self.lower_from_list_item(item);
        }
        self.set_visible((0..self.index.queries[id].sources.len()).into());
        let windows_mark = self.windows.len();
        self.scope_mut().windows = (windows_mark..windows_mark).into();
        if let Some(window_clause) = parts.window_clause {
            self.lower_window_clause(&window_clause);
        }

        let distinct_on = parts
            .select_clause
            .as_ref()
            .and_then(|select_clause| select_clause.select_quantifier())
            .and_then(|quantifier| match quantifier {
                ast::SelectQuantifier::DistinctClause(distinct) => distinct.distinct_on(),
                ast::SelectQuantifier::All(_) => None,
            });
        let columns = self.lower_target_list(
            id,
            parts
                .select_clause
                .and_then(|select_clause| select_clause.target_list()),
        );
        self.index.queries[id].columns = columns;
        for expr in distinct_on.iter().flat_map(ast::DistinctOn::exprs) {
            self.lower_expr_with_outputs(&expr, &columns, OutputClause::DistinctOn);
        }

        if let Some(expr) = parts.where_clause.and_then(|clause| clause.expr()) {
            self.lower_expr_in(ExprContext::Where, &expr);
        }
        if let Some(list) = parts
            .group_by_clause
            .and_then(|clause| clause.group_by_list())
        {
            self.with_expr_context(ExprContext::GroupBy, |this| {
                for group_by in list.group_bys() {
                    this.lower_group_by(&group_by, &columns);
                }
            });
        }
        if let Some(expr) = parts.having_clause.and_then(|clause| clause.expr()) {
            self.lower_expr_in(ExprContext::Having, &expr);
        }
        for clause in parts.tail_clauses {
            self.lower_tail_clause(clause, &columns);
        }

        self.scopes.pop();
        self.windows.truncate(windows_mark);
        self.ctes.truncate(mark);
        id
    }

    fn lower_compound(
        &mut self,
        compound: &ast::CompoundSelect,
        recursive_cte: Option<CteId>,
    ) -> QueryId {
        let mark = self.ctes.len();
        if !compound.is_nested()
            && let Some(with_clause) = compound.leftmost_with_clause()
        {
            self.lower_with(&with_clause);
        }
        let lhs = compound.lhs().map(|lhs| self.lower_query(lhs));
        if let (Some(cte), Some(lhs)) = (recursive_cte, lhs)
            && matches!(compound.op(), Some(ast::CompoundOp::Union(_)))
        {
            let columns = self.index.queries[lhs].columns;
            self.index.ctes[cte].columns = columns;
            self.index
                .set_column_owner(columns.range, ColumnOwner::Cte(cte));
        }
        if let Some(rhs) = compound.rhs() {
            let rhs_query = self.lower_query(rhs.clone());
            if let Some(lhs) = lhs {
                self.check_compound_arity(compound, lhs, rhs_query, &rhs);
            }
        }
        let id = self.alloc_query(QueryKind::Compound, compound.syntax());
        if let Some(lhs) = lhs {
            self.index.queries[id].columns = self.index.queries[lhs].columns;
        } else {
            self.index.queries[id].columns.complete = false;
        }
        self.lower_tail_clauses_for_outputs(id, compound.tail_clauses());
        self.ctes.truncate(mark);
        id
    }

    fn check_compound_arity(
        &mut self,
        compound: &ast::CompoundSelect,
        lhs: QueryId,
        rhs: QueryId,
        rhs_node: &ast::SelectVariant,
    ) {
        let lhs = self.index.queries[lhs].columns;
        let rhs = self.index.queries[rhs].columns;
        if !lhs.complete || !rhs.complete || lhs.range.len() == rhs.range.len() {
            return;
        }
        let op = match compound.op() {
            Some(ast::CompoundOp::Union(_)) => "UNION",
            Some(ast::CompoundOp::Intersect(_)) => "INTERSECT",
            Some(ast::CompoundOp::Except(_)) => "EXCEPT",
            None => return,
        };
        self.push_error(rhs_node.syntax(), SemaError::CompoundColumnCount { op });
    }

    fn lower_values(&mut self, values: &ast::Values) -> QueryId {
        let mark = self.ctes.len();
        if let Some(with_clause) = values.with_clause() {
            self.lower_with(&with_clause);
        }
        let first_len = values
            .row_list()
            .and_then(|list| list.rows().next())
            .map_or(0, |row| row.exprs().count());
        let mut columns = Vec::with_capacity(first_len);
        for (row_index, row) in values
            .row_list()
            .into_iter()
            .flat_map(|list| list.rows())
            .enumerate()
        {
            if row_index > 0 && row.exprs().count() != first_len {
                self.push_error(row.syntax(), SemaError::ValuesLengthMismatch);
            }
            for (column_index, expr) in row.exprs().enumerate() {
                let ty = self.lower_expr_in(ExprContext::Values, &expr);
                if row_index == 0 {
                    let name = self.intern(&format!("column{}", column_index + 1));
                    columns.push(Column {
                        name: Some(name),
                        ptr: self.ptr(expr.syntax()),
                        ty,
                        merge: None,
                    });
                }
            }
        }
        let id = self.alloc_query(QueryKind::Values, values.syntax());
        self.index.queries[id].columns.range = self.index.columns.alloc_many(columns);
        self.lower_tail_clauses_for_outputs(id, values.tail_clauses());
        self.ctes.truncate(mark);
        id
    }

    fn lower_table_query(&mut self, table: &ast::Table) -> QueryId {
        let mark = self.ctes.len();
        if let Some(with_clause) = table.with_clause() {
            self.lower_with(&with_clause);
        }
        let id = self.alloc_query(QueryKind::Table, table.syntax());
        if let Some(path) = table
            .relation_name()
            .and_then(|name| name.relation_name_ref())
            .and_then(|name| name.path_ref())
        {
            let RelationSource {
                name,
                kind,
                columns,
            } = self.lower_relation_ref(&path);
            let source = self.index.sources.alloc(Source {
                name,
                kind,
                ptr: self.ptr(table.syntax()),
                alias_ptr: None,
                columns,
            });
            let query = &mut self.index.queries[id];
            query.sources = vec![source];
            query.columns = columns;
        }
        self.lower_tail_clauses_for_outputs(id, table.tail_clauses());
        self.ctes.truncate(mark);
        id
    }

    fn lower_delete(&mut self, delete: &ast::Delete) -> QueryId {
        let mark = self.ctes.len();
        if let Some(with_clause) = delete.with_clause() {
            self.lower_with(&with_clause);
        }

        let id = self.alloc_query(QueryKind::Delete, delete.syntax());
        self.push_scope(id);

        let target = delete.relation_name().map(|relation_name| {
            self.push_target_source(
                relation_name
                    .relation_name_ref()
                    .and_then(|name| name.path_ref()),
                delete.alias().and_then(|alias| alias.name()),
                relation_name.syntax(),
            )
        });
        if let Some(for_portion_of) = delete.for_portion_of() {
            self.lower_for_portion_of(target, &for_portion_of);
        }
        let target_sources = Range::from(0..self.scope_sources_len());
        self.push_hidden(target_sources);
        for item in delete
            .using_clause()
            .into_iter()
            .flat_map(|clause| clause.items())
        {
            self.lower_from_list_item(item);
        }
        self.pop_hidden();
        self.set_visible((0..self.index.queries[id].sources.len()).into());

        match delete.where_clause_or_current_of() {
            Some(ast::WhereClauseOrCurrentOf::WhereClause(where_clause)) => {
                if let Some(expr) = where_clause.expr() {
                    self.lower_expr_in(ExprContext::Where, &expr);
                }
            }
            Some(ast::WhereClauseOrCurrentOf::WhereCurrentOf(current_of)) => {
                if let Some(cursor_ref) = current_of.cursor_ref() {
                    self.lower_cursor_ref(&cursor_ref);
                }
            }
            None => {}
        }
        if let Some(returning_clause) = delete.returning_clause() {
            let columns = self.lower_returning(id, target, &returning_clause);
            self.index.queries[id].columns = columns;
        }

        self.scopes.pop();
        self.ctes.truncate(mark);
        id
    }

    fn lower_update(&mut self, update: &ast::Update) -> QueryId {
        let mark = self.ctes.len();
        if let Some(with_clause) = update.with_clause() {
            self.lower_with(&with_clause);
        }

        let id = self.alloc_query(QueryKind::Update, update.syntax());
        self.push_scope(id);

        let target = update.relation_name().map(|relation_name| {
            self.push_target_source(
                relation_name
                    .relation_name_ref()
                    .and_then(|name| name.path_ref()),
                update.alias().and_then(|alias| alias.name()),
                relation_name.syntax(),
            )
        });
        if let Some(for_portion_of) = update.for_portion_of() {
            self.lower_for_portion_of(target, &for_portion_of);
        }
        let target_sources = Range::from(0..self.scope_sources_len());
        self.push_hidden(target_sources);
        for item in update
            .from_clause()
            .into_iter()
            .flat_map(|clause| clause.items())
        {
            self.lower_from_list_item(item);
        }
        self.pop_hidden();
        self.set_visible((0..self.index.queries[id].sources.len()).into());

        if let Some(set_clause) = update.set_clause() {
            self.lower_set_clause(target, &set_clause);
        }
        match update.where_clause_or_current_of() {
            Some(ast::WhereClauseOrCurrentOf::WhereClause(where_clause)) => {
                if let Some(expr) = where_clause.expr() {
                    self.lower_expr_in(ExprContext::Where, &expr);
                }
            }
            Some(ast::WhereClauseOrCurrentOf::WhereCurrentOf(current_of)) => {
                if let Some(cursor_ref) = current_of.cursor_ref() {
                    self.lower_cursor_ref(&cursor_ref);
                }
            }
            None => {}
        }
        if let Some(returning_clause) = update.returning_clause() {
            let columns = self.lower_returning(id, target, &returning_clause);
            self.index.queries[id].columns = columns;
        }

        self.scopes.pop();
        self.ctes.truncate(mark);
        id
    }

    fn lower_merge(&mut self, merge: &ast::Merge) -> QueryId {
        let mark = self.ctes.len();
        if let Some(with_clause) = merge.with_clause() {
            self.lower_with(&with_clause);
        }

        let id = self.alloc_query(QueryKind::Merge, merge.syntax());
        self.push_scope(id);

        let target = merge.table_relation_name().map(|relation_name| {
            self.push_target_source(
                relation_name
                    .table_name_ref()
                    .and_then(|name| name.path_ref()),
                merge.alias().and_then(|alias| alias.name()),
                relation_name.syntax(),
            )
        });
        let target_sources = Range::from(0..self.scope_sources_len());
        let using_on = merge.using_on_clause();
        if let Some(item) = using_on
            .as_ref()
            .and_then(|using_on| using_on.from_list_item())
        {
            self.push_hidden(target_sources);
            self.lower_from_list_item(item);
            self.pop_hidden();
        }
        let len = self.scope_sources_len();
        let source_sources = Range::from(target_sources.end..len);
        self.set_visible((0..len).into());

        if let Some(expr) = using_on
            .and_then(|using_on| using_on.on_clause())
            .and_then(|on_clause| on_clause.expr())
        {
            self.lower_expr_in(ExprContext::JoinCondition, &expr);
        }

        for when_clause in merge.merge_when_clauses() {
            let (hidden, condition, action) = match when_clause {
                ast::MergeWhenClause::MergeWhenMatched(matched) => {
                    (None, matched.merge_condition(), matched.merge_action())
                }
                ast::MergeWhenClause::MergeWhenNotMatchedSource(not_matched) => (
                    Some(source_sources),
                    not_matched.merge_condition(),
                    not_matched.merge_action(),
                ),
                ast::MergeWhenClause::MergeWhenNotMatchedTarget(not_matched) => (
                    Some(target_sources),
                    not_matched.merge_condition(),
                    not_matched.merge_action(),
                ),
            };
            if let Some(hidden) = hidden {
                self.push_hidden(hidden);
            }
            if let Some(expr) = condition.and_then(|condition| condition.expr()) {
                self.lower_expr_in(ExprContext::MergeWhen, &expr);
            }
            if let Some(action) = action {
                self.lower_merge_action(target, &action);
            }
            if hidden.is_some() {
                self.pop_hidden();
            }
        }

        if let Some(returning_clause) = merge.returning_clause() {
            let columns = self.lower_returning(id, target, &returning_clause);
            self.index.queries[id].columns = columns;
        }

        self.scopes.pop();
        self.ctes.truncate(mark);
        id
    }

    fn lower_merge_action(&mut self, target: Option<SourceId>, action: &ast::MergeAction) {
        match action {
            ast::MergeAction::MergeInsert(insert) => {
                if let Some(target) = target {
                    for column_target in insert
                        .column_target_list()
                        .into_iter()
                        .flat_map(|list| list.column_targets())
                    {
                        self.lower_column_target(ExprContext::Insert, target, &column_target);
                    }
                }
                for expr in insert
                    .values()
                    .and_then(|values| values.row_list())
                    .into_iter()
                    .flat_map(|list| list.rows())
                    .flat_map(|row| row.exprs())
                {
                    self.lower_expr_in(ExprContext::Values, &expr);
                }
            }
            ast::MergeAction::MergeUpdate(update) => {
                if let Some(set_clause) = update.set_clause() {
                    self.lower_set_clause(target, &set_clause);
                }
            }
            ast::MergeAction::MergeDelete(_) | ast::MergeAction::MergeDoNothing(_) => {}
        }
    }

    fn lower_insert(&mut self, insert: &ast::Insert) -> QueryId {
        let mark = self.ctes.len();
        if let Some(with_clause) = insert.with_clause() {
            self.lower_with(&with_clause);
        }

        let id = self.alloc_query(QueryKind::Insert, insert.syntax());
        self.push_scope(id);

        let source = match insert.insert_source() {
            Some(ast::InsertSource::SelectVariant(query)) => {
                Some((self.lower_query(query.clone()), query))
            }
            Some(ast::InsertSource::DefaultValues(_)) | None => None,
        };

        let target = insert.relation_name_ref().map(|relation_name_ref| {
            let node = relation_name_ref.syntax().clone();
            self.push_target_source(
                relation_name_ref.path_ref(),
                insert.alias().and_then(|alias| alias.name()),
                &node,
            )
        });
        if let Some(target) = target {
            let mut seen = smallvec![];
            for column_target in insert
                .column_target_list()
                .into_iter()
                .flat_map(|list| list.column_targets())
            {
                if let Some(name) = self.duplicate_target(&mut seen, &column_target) {
                    let error = SemaError::ColumnSpecifiedMoreThanOnce { name };
                    self.push_error(column_target.syntax(), error);
                }
                self.lower_column_target(ExprContext::Insert, target, &column_target);
            }
        }
        if let Some((query, source)) = &source {
            self.check_insert_arity(insert, target, *query, source);
        }
        self.set_visible((0..self.index.queries[id].sources.len()).into());

        if let Some(on_conflict) = insert.on_conflict_clause() {
            self.lower_on_conflict(id, target, &on_conflict);
        }
        if let Some(returning_clause) = insert.returning_clause() {
            let columns = self.lower_returning(id, target, &returning_clause);
            self.index.queries[id].columns = columns;
        }

        self.scopes.pop();
        self.ctes.truncate(mark);
        id
    }

    fn check_insert_arity(
        &mut self,
        insert: &ast::Insert,
        target: Option<SourceId>,
        query: QueryId,
        source: &ast::SelectVariant,
    ) {
        let column_targets = insert.column_target_list();
        let target_len = match &column_targets {
            Some(list) => list.column_targets().count(),
            None => {
                let Some(target) = target else {
                    return;
                };
                let columns = self.index.sources[target].columns;
                if !columns.complete {
                    return;
                }
                columns.range.len()
            }
        };
        let columns = self.index.queries[query].columns;
        if !columns.complete {
            return;
        }
        let source_len = columns.range.len();
        if source_len > target_len {
            let source_range = source.syntax().text_range();
            let ptr = columns
                .range
                .iter()
                .nth(target_len)
                .map(|id| self.index.columns[id].ptr)
                .filter(|ptr| {
                    ptr.file_id == self.file_id
                        && source_range.contains_range(ptr.value.text_range())
                })
                .unwrap_or_else(|| self.ptr(source.syntax()));
            self.push_error_ptr(ptr, SemaError::InsertMoreExpressionsThanTargets);
        } else if source_len < target_len
            && let Some(column_target) =
                column_targets.and_then(|list| list.column_targets().nth(source_len))
        {
            let row_hint = source_len == 1
                && single_source_expr(source).is_some_and(|expr| {
                    matches!(expr, ast::Expr::TupleExpr(tuple) if tuple.exprs().count() == target_len)
                });
            self.push_error(
                column_target.syntax(),
                SemaError::InsertMoreTargetsThanExpressions { row_hint },
            );
        }
    }

    fn lower_on_conflict(
        &mut self,
        query: QueryId,
        target: Option<SourceId>,
        on_conflict: &ast::OnConflictClause,
    ) {
        match on_conflict.conflict_target() {
            Some(ast::ConflictTarget::ConflictOnIndex(on_index)) => {
                for item in on_index
                    .conflict_index_item_list()
                    .into_iter()
                    .flat_map(|list| list.conflict_index_items())
                {
                    if let Some(expr) = item.expr() {
                        self.lower_expr_in(ExprContext::IndexExpression, &expr);
                    }
                    if let Some(path) = item.op_class_ref().and_then(|op_class| op_class.path_ref())
                    {
                        self.lower_operator_class_ref(&path, None, IfMissing::Error);
                    }
                }
                if let Some(expr) = on_index.where_clause().and_then(|clause| clause.expr()) {
                    self.lower_expr_in(ExprContext::IndexPredicate, &expr);
                }
            }
            Some(ast::ConflictTarget::ConflictOnConstraint(on_constraint)) => {
                let table = target.and_then(|target| match self.index.sources[target].kind {
                    SourceKind::Table(table) => Some(table),
                    SourceKind::Cte(_)
                    | SourceKind::External
                    | SourceKind::Subquery(_)
                    | SourceKind::Function(_)
                    | SourceKind::Join
                    | SourceKind::UsingAlias
                    | SourceKind::Returning(_)
                    | SourceKind::Excluded(_)
                    | SourceKind::RuleRelation(_)
                    | SourceKind::ElementVariable
                    | SourceKind::Other => None,
                });
                if let Some(name_ref) = on_constraint.constraint_name_ref() {
                    self.lower_constraint_ref(
                        table.map(ConstraintOwner::Table),
                        &name_ref,
                        IfMissing::Error,
                    );
                }
            }
            None => {}
        }

        let Some(action) = on_conflict.conflict_action() else {
            return;
        };
        let len = self.index.queries[query].sources.len();
        if let Some(target) = target {
            let columns = self.index.sources[target].columns;
            let name = self.intern("excluded");
            self.push_source(Source {
                name: Some(name),
                kind: SourceKind::Excluded(target),
                ptr: self.ptr(on_conflict.syntax()),
                alias_ptr: None,
                columns,
            });
            self.set_visible((0..len + 1).into());
        }
        let where_clause = match action {
            ast::ConflictAction::ConflictDoNothing(_) => None,
            ast::ConflictAction::ConflictDoSelect(do_select) => {
                if let Some(locking) = do_select.locking_clause() {
                    self.set_visible((0..len).into());
                    self.lower_locking_clause(&locking);
                    if target.is_some() {
                        self.set_visible((0..len + 1).into());
                    }
                }
                do_select.where_clause()
            }
            ast::ConflictAction::ConflictDoUpdateSet(do_update) => {
                if let Some(set_clause) = do_update.set_clause() {
                    self.lower_set_clause(target, &set_clause);
                }
                do_update.where_clause()
            }
        };
        if let Some(expr) = where_clause.and_then(|clause| clause.expr()) {
            self.lower_expr_in(ExprContext::Where, &expr);
        }
        self.index.queries[query].sources.truncate(len);
        self.set_visible((0..len).into());
    }

    fn lower_set_clause(&mut self, target: Option<SourceId>, set_clause: &ast::SetClause) {
        let mut seen = smallvec![];
        for set_column in set_clause
            .set_column_list()
            .into_iter()
            .flat_map(|list| list.set_columns())
        {
            match set_column {
                ast::SetColumn::SetSingleColumn(single) => {
                    if let Some(column_target) = single.column_target() {
                        self.check_duplicate_assignment(&mut seen, &column_target);
                        if let Some(target) = target {
                            self.lower_column_target(ExprContext::Update, target, &column_target);
                        }
                    }
                    if let Some(expr) = single.set_expr().and_then(|set_expr| set_expr.expr()) {
                        self.lower_expr_in(ExprContext::Update, &expr);
                    }
                }
                ast::SetColumn::SetMultipleColumns(multiple) => {
                    for column_target in multiple
                        .column_target_list()
                        .into_iter()
                        .flat_map(|list| list.column_targets())
                    {
                        self.check_duplicate_assignment(&mut seen, &column_target);
                        if let Some(target) = target {
                            self.lower_column_target(ExprContext::Update, target, &column_target);
                        }
                    }
                    if let Some(paren_select) = multiple.paren_select() {
                        self.lower_query(ast::SelectVariant::ParenSelect(paren_select));
                    }
                    for expr in multiple
                        .set_expr_list()
                        .into_iter()
                        .flat_map(|list| list.set_exprs())
                        .filter_map(|set_expr| set_expr.expr())
                    {
                        self.lower_expr_in(ExprContext::Update, &expr);
                    }
                }
            }
        }
    }

    fn check_duplicate_assignment(
        &mut self,
        seen: &mut SmallVec<[(Name, bool); 8]>,
        column_target: &ast::ColumnTarget,
    ) {
        if let Some(name) = self.duplicate_target(seen, column_target) {
            self.push_error(
                column_target.syntax(),
                SemaError::DuplicateAssignment { name },
            );
        }
    }

    fn duplicate_target(
        &mut self,
        seen: &mut SmallVec<[(Name, bool); 8]>,
        column_target: &ast::ColumnTarget,
    ) -> Option<Name> {
        let name = self.intern(&column_target.name()?.text());
        let partial = column_target.accessors().next().is_some();
        let duplicate = seen
            .iter()
            .any(|&(prev, prev_partial)| prev == name && !(partial && prev_partial));
        seen.push((name, partial));
        duplicate.then_some(name)
    }

    fn lower_for_portion_of(
        &mut self,
        target: Option<SourceId>,
        for_portion_of: &ast::ForPortionOf,
    ) {
        if let (Some(target), Some(column_ref)) = (target, for_portion_of.column_name_ref()) {
            self.add_target_column_ref(target, &column_ref);
        }
        match for_portion_of.range() {
            Some(ast::PortionTarget::PortionFromTo(from_to)) => {
                for expr in from_to.from().into_iter().chain(from_to.to()) {
                    self.lower_expr(&expr);
                }
            }
            Some(ast::PortionTarget::PortionRange(range)) => {
                if let Some(expr) = range.expr() {
                    self.lower_expr(&expr);
                }
            }
            None => {}
        }
    }

    fn add_target_column_ref(
        &mut self,
        target: SourceId,
        column_ref: &impl NameLike,
    ) -> Option<Ty> {
        let name = self.intern(&column_ref.text());
        let (resolution, ty) = match self.find_source_column(target, name) {
            Some(column) => (
                Resolution::Column {
                    source: Some(target),
                    column,
                },
                self.index.columns[column].ty,
            ),
            None if self.index.sources[target].columns.complete => (Resolution::Unresolved, None),
            None => (Resolution::Unknown, None),
        };
        self.add_ref(name, ReferenceKind::Column, column_ref.syntax(), resolution);
        ty
    }

    fn lower_column_target(
        &mut self,
        context: ExprContext,
        target: SourceId,
        column_target: &ast::ColumnTarget,
    ) {
        let Some(column_ref) = column_target.name() else {
            return;
        };
        let mut ty = self.add_target_column_ref(target, &column_ref);
        let mut subscript = None;
        for accessor in column_target.accessors() {
            match accessor {
                ast::Accessor::IndexAccessor(index) => {
                    subscript.get_or_insert(false);
                    if let Some(expr) = index.index() {
                        self.lower_expr_in(context, &expr);
                    }
                }
                ast::Accessor::SliceAccessor(slice) => {
                    subscript = Some(true);
                    for expr in slice.start().into_iter().chain(slice.end()) {
                        self.lower_expr_in(context, &expr);
                    }
                }
                ast::Accessor::FieldAccessor(field) => {
                    if let Some(slice) = subscript.take() {
                        ty = ty.and_then(|ty| self.subscripted(ty, slice));
                    }
                    ty = field
                        .composite_field_ref()
                        .and_then(|field_ref| self.add_field_ref(ty, &field_ref));
                }
            }
        }
    }

    fn add_field_ref(&mut self, ty: Option<Ty>, field_ref: &impl NameLike) -> Option<Ty> {
        if let Some(Ty::Array(_)) = ty {
            let name = self.intern(&field_ref.text());
            self.add_ref(
                name,
                ReferenceKind::Column,
                field_ref.syntax(),
                Resolution::Unresolved,
            );
            return None;
        }
        let columns = ty.and_then(|ty| self.ty_columns(ty).copied());
        self.add_columns_field_ref(columns, field_ref)
    }

    fn add_columns_field_ref(
        &mut self,
        columns: Option<Columns>,
        field_ref: &impl NameLike,
    ) -> Option<Ty> {
        let name = self.intern(&field_ref.text());
        let resolution = match columns {
            Some(columns) => match self.find_column_in(columns.range, name) {
                Some(column) => Resolution::Column {
                    source: None,
                    column,
                },
                None if columns.complete => Resolution::Unresolved,
                None => Resolution::Unknown,
            },
            None => Resolution::Unknown,
        };
        self.add_ref(name, ReferenceKind::Column, field_ref.syntax(), resolution);
        if let Resolution::Column { column, .. } = resolution {
            self.index.columns[column].ty
        } else {
            None
        }
    }

    fn ty_columns(&self, ty: Ty) -> Option<&Columns> {
        match ty {
            Ty::Type(ty) => self.index.types[ty].columns.as_ref(),
            Ty::Table(table) => Some(&self.index.tables[table].columns),
            Ty::Cte(cte) => Some(&self.index.ctes[cte].columns),
            Ty::Array(_) => None,
        }
    }

    fn push_target_source(
        &mut self,
        path: Option<ast::PathRef>,
        table_alias: Option<ast::TableAlias>,
        node: &SyntaxNode,
    ) -> SourceId {
        let RelationSource {
            mut name,
            kind,
            columns,
        } = match path {
            Some(path) => self.lower_relation_path(&path, RelationLookup::OnlyTable),
            None => self.other_relation_source(),
        };
        if let Some(table_alias) = &table_alias {
            name = Some(self.intern(&table_alias.text()));
        }
        self.push_source(Source {
            name,
            kind,
            ptr: self.ptr(node),
            alias_ptr: table_alias.map(|table_alias| self.ast_ptr(&table_alias)),
            columns,
        });
        *self.index.queries[self.scope().query]
            .sources
            .last()
            .expect("source was just pushed")
    }

    fn lower_returning(
        &mut self,
        query: QueryId,
        target: Option<SourceId>,
        returning_clause: &ast::ReturningClause,
    ) -> Columns {
        if let Some(target) = target {
            let mut old = None;
            let mut new = None;
            for option in returning_clause
                .returning_option_list()
                .into_iter()
                .flat_map(|list| list.returning_options())
            {
                match option {
                    ast::ReturningOption::ReturningOld(returning_old) => {
                        old = returning_old
                            .name()
                            .map(|name| (self.ptr(returning_old.syntax()), name));
                    }
                    ast::ReturningOption::ReturningNew(returning_new) => {
                        new = returning_new
                            .name()
                            .map(|name| (self.ptr(returning_new.syntax()), name));
                    }
                }
            }
            self.push_returning_source(target, returning_clause, old, "old");
            self.push_returning_source(target, returning_clause, new, "new");
            self.set_visible((0..self.index.queries[query].sources.len()).into());
        }
        self.with_expr_context(ExprContext::Returning, |this| {
            this.lower_target_list(query, returning_clause.target_list())
        })
    }

    fn push_returning_source(
        &mut self,
        target: SourceId,
        returning_clause: &ast::ReturningClause,
        alias: Option<(InIrFile<SyntaxNodePtr>, ast::TableAlias)>,
        default: &str,
    ) {
        let (name, ptr, alias_ptr) = match alias {
            Some((ptr, table_alias)) => (
                self.intern(&table_alias.text()),
                ptr,
                Some(self.ast_ptr(&table_alias)),
            ),
            None => (
                self.intern(default),
                self.ptr(returning_clause.syntax()),
                None,
            ),
        };
        let columns = self.index.sources[target].columns;
        self.push_source(Source {
            name: Some(name),
            kind: SourceKind::Returning(target),
            ptr,
            alias_ptr,
            columns,
        });
    }

    fn lower_with(&mut self, with_clause: &ast::WithClause) {
        self.check_duplicate_ctes(with_clause);
        if with_clause.recursive_token().is_some() {
            self.lower_recursive_with(with_clause);
            return;
        }
        for with_table in with_clause.with_tables() {
            let Some(id) = self.alloc_cte(&with_table) else {
                continue;
            };
            self.lower_cte_body(id, &with_table, CteKind::NonRecursive);
            self.ctes.push(id);
        }
    }

    fn lower_recursive_with(&mut self, with_clause: &ast::WithClause) {
        let mark = self.pending_ctes.len();
        let start = self.ctes.len();
        for with_table in with_clause.with_tables() {
            if let Some(id) = self.alloc_cte(&with_table) {
                self.ctes.push(id);
            }
        }
        let ctes = self.ctes.len();
        let with_tables = with_clause
            .with_tables()
            .filter(|with_table| with_table.name().is_some());
        for (with_table, &id) in with_tables.zip(&self.ctes[start..]) {
            self.pending_ctes.push(PendingCte {
                id,
                with_table,
                scopes: self.scopes.len(),
                ctes,
            });
        }
        while self.pending_ctes.len() > mark {
            let pending = self.pending_ctes.remove(mark);
            self.lower_pending_cte(&pending);
        }
    }

    fn check_duplicate_ctes(&mut self, with_clause: &ast::WithClause) {
        let mut seen: SmallVec<[Name; 8]> = smallvec![];
        for cte_name in with_clause
            .with_tables()
            .filter_map(|with_table| with_table.name())
        {
            let name = self.intern(&cte_name.text());
            if seen.contains(&name) {
                let error = SemaError::DuplicateCte { name };
                self.push_error(cte_name.syntax(), error);
            }
            seen.push(name);
        }
    }

    fn lower_pending_cte(&mut self, pending: &PendingCte) {
        let scopes = self.scopes.split_off(pending.scopes);
        let ctes = self.ctes.split_off(pending.ctes);
        self.lower_cte_body(pending.id, &pending.with_table, CteKind::Recursive);
        self.scopes.extend(scopes);
        self.ctes.extend(ctes);
    }

    fn alloc_cte(&mut self, with_table: &ast::WithTable) -> Option<CteId> {
        let cte_name = with_table.name()?;
        let name = self.intern(&cte_name.text());
        let columns = match with_table.column_list() {
            Some(list) => self.lower_column_list(&list),
            None => self.unknown_columns(),
        };
        let id = self.index.ctes.alloc(Cte {
            name,
            ptr: self.ast_ptr(with_table),
            name_ptr: self.ast_ptr(&cte_name),
            query: None,
            columns,
        });
        self.index
            .set_column_owner(columns.range, ColumnOwner::Cte(id));
        self.record_comment(with_table.syntax(), Item::Cte(id));
        self.add_item(cte_name.syntax(), Item::Cte(id));
        Some(id)
    }

    fn lower_cte_body(&mut self, id: CteId, with_table: &ast::WithTable, kind: CteKind) {
        let column_list = with_table
            .column_list()
            .is_some()
            .then(|| self.index.ctes[id].columns);
        if let Some(query) = with_table.query() {
            let recursive_cte = match kind {
                CteKind::Recursive => column_list.is_none().then_some(id),
                CteKind::NonRecursive => None,
            };
            let query = self.lower_with_query(&query, recursive_cte);
            let query_columns = self.index.queries[query].columns;
            let columns = match &column_list {
                Some(list) => self.apply_column_list(
                    list,
                    &query_columns,
                    ColumnListOwner::Cte(self.index.ctes[id].name),
                ),
                None => query_columns,
            };
            let cte = &mut self.index.ctes[id];
            cte.query = Some(query);
            cte.columns = columns;
            self.index
                .set_column_owner(columns.range, ColumnOwner::Cte(id));
        }
        self.lower_search_cycle(id, with_table);
    }

    fn lower_search_cycle(&mut self, cte: CteId, with_table: &ast::WithTable) {
        let search = with_table.search_clause();
        let cycle = with_table.cycle_clause();
        if search.is_none() && cycle.is_none() {
            return;
        }
        let columns = self.index.ctes[cte].columns;
        let start = self.index.columns.next_idx();
        if let Some(search) = search {
            for column_ref in search
                .columns()
                .into_iter()
                .flat_map(|columns| columns.column_name_refs())
            {
                self.lower_column_ref_in(&columns, &column_ref);
            }
            if let Some(column_ref) = search.set_column().and_then(|set| set.column_name_ref()) {
                let ty = match search.search_order() {
                    Some(ast::SearchOrder::BreadthFirst(_)) => self.builtin_type("record"),
                    Some(ast::SearchOrder::DepthFirst(_)) => {
                        self.builtin_type("record").map(Ty::array)
                    }
                    None => None,
                };
                self.alloc_column_def_ref(&column_ref, ty);
            }
        }
        if let Some(cycle) = cycle {
            for column_ref in cycle
                .columns()
                .into_iter()
                .flat_map(|columns| columns.column_name_refs())
            {
                self.lower_column_ref_in(&columns, &column_ref);
            }
            if let Some(set) = cycle.set_column() {
                let ty = match set.column_to() {
                    Some(column_to) => {
                        let ty = column_to.expr().and_then(|expr| self.lower_expr(&expr));
                        if let Some(expr) = column_to.default().and_then(|default| default.expr()) {
                            self.lower_expr(&expr);
                        }
                        ty
                    }
                    None => self.builtin_type("bool"),
                };
                if let Some(column_ref) = set.column_name_ref() {
                    self.alloc_column_def_ref(&column_ref, ty);
                }
            }
            if let Some(column_ref) = cycle.path().and_then(|path| path.column_name_ref()) {
                let ty = self.builtin_type("record").map(Ty::array);
                self.alloc_column_def_ref(&column_ref, ty);
            }
        }
        let added = self.index.columns.range_from(start);
        let range = self
            .index
            .columns
            .alloc_copies(columns.range.iter().chain(added));
        self.index.ctes[cte].columns = Columns {
            range,
            complete: columns.complete,
        };
        self.index.set_column_owner(range, ColumnOwner::Cte(cte));
    }

    fn lower_with_query(
        &mut self,
        query: &ast::WithQuery,
        recursive_cte: Option<CteId>,
    ) -> QueryId {
        self.with_expr_context(ExprContext::Unrestricted, |this| {
            this.lower_with_query_inner(query, recursive_cte)
        })
    }

    fn lower_with_query_inner(
        &mut self,
        query: &ast::WithQuery,
        recursive_cte: Option<CteId>,
    ) -> QueryId {
        let select = match query.clone() {
            ast::WithQuery::Delete(delete) => return self.lower_delete(&delete),
            ast::WithQuery::Select(select) => ast::SelectVariant::Select(select),
            ast::WithQuery::CompoundSelect(compound) => {
                ast::SelectVariant::CompoundSelect(compound)
            }
            ast::WithQuery::ParenSelect(paren) => ast::SelectVariant::ParenSelect(paren),
            ast::WithQuery::Table(table) => ast::SelectVariant::Table(table),
            ast::WithQuery::Values(values) => ast::SelectVariant::Values(values),
            ast::WithQuery::Insert(insert) => return self.lower_insert(&insert),
            ast::WithQuery::Update(update) => return self.lower_update(&update),
            ast::WithQuery::Merge(merge) => return self.lower_merge(&merge),
        };
        self.lower_cte_query(select, recursive_cte)
    }

    fn lower_column_list(&mut self, list: &ast::ColumnList) -> Columns {
        self.lower_column_names(list.column_names())
    }

    fn lower_column_names(
        &mut self,
        column_names: impl Iterator<Item = ast::ColumnName>,
    ) -> Columns {
        let start = self.index.columns.next_idx();
        for column_name in column_names {
            let column = self.column_def(&column_name);
            self.alloc_column_def(column);
        }
        Columns {
            range: self.index.columns.range_from(start),
            complete: true,
        }
    }

    fn lower_from_list_item(&mut self, item: ast::FromListItem) {
        match item {
            ast::FromListItem::FromItem(from_item) => {
                self.lower_from_item(&from_item);
            }
            ast::FromListItem::JoinExpr(join_expr) => {
                let start = self.scope_sources_len();
                if let Some(lhs) = join_expr.from_list_item() {
                    self.lower_from_list_item(lhs);
                }
                let Some(join) = join_expr.join() else {
                    return;
                };
                let mid = self.scope_sources_len();
                let kind = match join.join_type() {
                    Some(ast::JoinType::JoinLeft(_)) => JoinKind::Left,
                    Some(ast::JoinType::JoinRight(_)) => JoinKind::Right,
                    Some(ast::JoinType::JoinFull(_)) => JoinKind::Full,
                    Some(ast::JoinType::JoinInner(_) | ast::JoinType::JoinCross(_)) | None => {
                        JoinKind::Inner
                    }
                };
                let hides_lhs = matches!(kind, JoinKind::Right | JoinKind::Full);
                if hides_lhs {
                    self.push_hidden((start..mid).into());
                }
                if let Some(rhs) = join.from_list_item() {
                    self.lower_from_list_item(rhs);
                }
                if hides_lhs {
                    self.pop_hidden();
                }
                if join.natural_token().is_some() {
                    self.lower_natural_join(start, mid, kind);
                }
                if let Some(condition) = join.join_condition() {
                    self.with_expr_context(ExprContext::JoinCondition, |this| {
                        this.lower_join_condition(condition, start, mid, kind);
                    });
                }
            }
        }
    }

    fn lower_tablesample(&mut self, tablesample: &ast::TablesampleClause) {
        let Some(call) = tablesample.call_expr() else {
            return;
        };
        for arg in call.arg_list().iter().flat_map(|list| list.args()) {
            if let Some(expr) = arg.func_arg_expr().and_then(|expr| expr.expr()) {
                self.lower_expr_in(ExprContext::FromFunction, &expr);
            }
        }
        if let Some(expr) = tablesample
            .repeatable_clause()
            .and_then(|repeatable| repeatable.expr())
        {
            self.lower_expr_in(ExprContext::FromFunction, &expr);
        }
        let (schema, name_ref) = match call.expr() {
            Some(ast::Expr::NameRef(name_ref)) => (None, name_ref),
            Some(ast::Expr::FieldExpr(field_expr)) => {
                let (Some(ast::Expr::NameRef(schema)), Some(field)) =
                    (field_expr.base(), field_expr.field())
                else {
                    return;
                };
                (Some(schema), field)
            }
            Some(
                ast::Expr::ArrayExpr(_)
                | ast::Expr::BetweenExpr(_)
                | ast::Expr::BinExpr(_)
                | ast::Expr::CallExpr(_)
                | ast::Expr::CaseExpr(_)
                | ast::Expr::CastExpr(_)
                | ast::Expr::Collate(_)
                | ast::Expr::IndexExpr(_)
                | ast::Expr::Literal(_)
                | ast::Expr::ParenExpr(_)
                | ast::Expr::PostfixExpr(_)
                | ast::Expr::PrefixExpr(_)
                | ast::Expr::SliceExpr(_)
                | ast::Expr::TupleExpr(_),
            )
            | None => return,
        };
        let schema = schema.map(|schema| self.lower_schema_ref(schema.syntax(), &schema.text()));
        let name = self.intern(&name_ref.text());
        let internal = self.builtin_type("internal");
        let function = self.find_function_by(schema, name, |function| {
            self.has_arg_types(function, &[internal])
        });
        let resolution = function.map_or(Resolution::Unresolved, Resolution::Function);
        self.add_ref(
            name,
            ReferenceKind::Function(RoutineKind::Function),
            name_ref.syntax(),
            resolution,
        );
    }

    fn lower_from_item(&mut self, from_item: &ast::FromItem) {
        let RelationSource {
            mut name,
            kind,
            mut columns,
        } = match from_item {
            ast::FromItem::RelationFromItem(relation) => {
                if let Some(tablesample) = relation.tablesample_clause() {
                    self.lower_tablesample(&tablesample);
                }
                match relation
                    .relation_name_ref()
                    .and_then(|name| name.path_ref())
                {
                    Some(path) => self.lower_relation_ref(&path),
                    None => self.other_relation_source(),
                }
            }
            ast::FromItem::ParenFromItem(paren) => {
                let lateral = paren.lateral_token().is_some();
                let query =
                    if let Some(paren_select) = paren.paren_select() {
                        Some(self.lower_from_query(
                            ast::SelectVariant::ParenSelect(paren_select),
                            lateral,
                        ))
                    } else if let Some(paren_expr) = paren.paren_expr() {
                        if let Some(item) = paren_expr.from_list_item() {
                            let start = self.scope_sources_len();
                            self.lower_from_list_item(item);
                            if let Some(alias) = paren.alias() {
                                self.lower_join_alias(from_item, &alias, start);
                            }
                            return;
                        }
                        paren_expr
                            .select_variant()
                            .map(|query| self.lower_from_query(query, lateral))
                    } else {
                        None
                    };
                match query {
                    Some(query) => RelationSource {
                        name: None,
                        kind: SourceKind::Subquery(query),
                        columns: self.index.queries[query].columns,
                    },
                    None => self.other_relation_source(),
                }
            }
            ast::FromItem::FunctionFromItem(function_item) => {
                let alias_columns = from_item.alias().and_then(|alias| alias.columns());
                let scalar_alias = from_item
                    .alias()
                    .filter(|_| alias_columns.is_none())
                    .and_then(|alias| alias.name());
                let has_column_defs =
                    matches!(alias_columns, Some(ast::FromAliasColumns::ColumnDefList(_)));
                let FunctionSource {
                    callee,
                    function,
                    columns,
                } = match function_item.call_expr() {
                    Some(call) => {
                        self.lower_function_source(&call, scalar_alias.as_ref(), has_column_defs)
                    }
                    None => FunctionSource {
                        callee: None,
                        function: None,
                        columns: self.unknown_columns(),
                    },
                };
                RelationSource {
                    name: callee.map(|callee| callee.name),
                    kind: function.map_or(SourceKind::Other, SourceKind::Function),
                    columns,
                }
            }
            ast::FromItem::JsonTableFromItem(json_table_item) => {
                let mut columns = vec![];
                if let Some(json_table) = json_table_item.json_table() {
                    self.with_from_function(|this| {
                        if let Some(expr) = json_table.expr() {
                            this.lower_expr(&expr);
                        }
                        this.lower_json_passing(json_table.json_passing_clause());
                        let paths_mark = this.json_paths.len();
                        if let Some(clause) = json_table.json_path_name_clause() {
                            this.lower_json_path_name_clause(&clause);
                        }
                        if let Some(list) = json_table.json_table_column_list() {
                            this.json_table_columns(&list, &mut columns);
                        }
                        if let Some(plan_clause) = json_table.json_table_plan_clause() {
                            for plan in plan_clause.json_table_plans() {
                                this.lower_json_table_plan(&plan, paths_mark);
                            }
                        }
                        this.json_paths.truncate(paths_mark);
                    });
                }
                let columns = Columns {
                    range: self.alloc_column_defs(columns),
                    complete: true,
                };
                RelationSource {
                    name: None,
                    kind: SourceKind::Other,
                    columns,
                }
            }
            ast::FromItem::ExprFromItem(expr_item) => {
                self.with_from_function(|this| {
                    if let Some(call) = expr_item.call_expr() {
                        this.lower_call(&call);
                    }
                    if let Some(cast) = expr_item.cast_expr() {
                        this.lower_expr(&ast::Expr::CastExpr(cast));
                    }
                });
                self.other_relation_source()
            }
            ast::FromItem::GraphTableFromItem(graph_table_item) => {
                let columns = match graph_table_item.graph_table_fn() {
                    Some(graph_table) => {
                        self.with_lateral_sources(|this| this.lower_graph_table(&graph_table))
                    }
                    None => self.unknown_columns(),
                };
                RelationSource {
                    name: None,
                    kind: SourceKind::Other,
                    columns,
                }
            }
            ast::FromItem::RowsFromItem(rows_from_item) => RelationSource {
                name: None,
                kind: SourceKind::Other,
                columns: self.lower_rows_from_columns(rows_from_item),
            },
            ast::FromItem::XmlTableFromItem(xml_table_item) => {
                let columns = match xml_table_item.xml_table() {
                    Some(xml_table) => {
                        self.with_from_function(|this| this.lower_xml_table(&xml_table))
                    }
                    None => self.unknown_columns(),
                };
                RelationSource {
                    name: None,
                    kind: SourceKind::Other,
                    columns,
                }
            }
        };

        if let Some(with_ordinality) = from_item.with_ordinality() {
            columns = self.with_ordinality_column(&columns, &with_ordinality);
        }

        let mut alias_ptr = None;
        if let Some(alias) = from_item.alias() {
            if let Some(table_alias) = alias.name() {
                name = Some(self.intern(&table_alias.text()));
                alias_ptr = Some(self.ast_ptr(&table_alias));
            }
            match alias.columns() {
                Some(ast::FromAliasColumns::ColumnList(list)) => {
                    columns = self.rename_columns(
                        list.column_names(),
                        &columns,
                        ColumnListOwner::Table(name),
                    );
                }
                Some(ast::FromAliasColumns::ColumnDefList(list)) => {
                    columns = self.column_def_list_columns(&list);
                }
                None => {}
            }
        }

        self.check_duplicate_source(name, kind, alias_ptr.is_some(), from_item);
        self.push_source(Source {
            name,
            kind,
            ptr: self.ptr(from_item.syntax()),
            alias_ptr,
            columns,
        });
    }

    fn check_duplicate_source(
        &mut self,
        name: Option<Name>,
        kind: SourceKind,
        aliased: bool,
        from_item: &ast::FromItem,
    ) {
        let Some(name) = name else {
            return;
        };
        let query = self.scope().query;
        let sources = &self.index.queries[query].sources;
        let duplicate = sources.iter().any(|id| {
            let prev = &self.index.sources[*id];
            if prev.name != Some(name) || prev.kind.is_qualified_only() {
                return false;
            }
            let different_unaliased_tables = match (prev.kind, kind) {
                (SourceKind::Table(a), SourceKind::Table(b)) => {
                    a != b && prev.alias_ptr.is_none() && !aliased
                }
                _ => false,
            };
            !different_unaliased_tables
        });
        if duplicate {
            let node = match from_item.alias().and_then(|alias| alias.name()) {
                Some(alias) => alias.syntax().clone(),
                None => from_item.syntax().clone(),
            };
            let error = SemaError::DuplicateSource { name };
            self.push_error(&node, error);
        }
    }

    fn lower_from_query(&mut self, query: ast::SelectVariant, lateral: bool) -> QueryId {
        if lateral {
            self.with_lateral_sources(|this| this.lower_query(query))
        } else {
            self.lower_query(query)
        }
    }

    fn lower_join_alias(
        &mut self,
        from_item: &ast::FromItem,
        alias: &ast::FromAlias,
        start: usize,
    ) {
        let scope = self
            .scopes
            .last()
            .expect("from items are lowered inside a select scope");
        let index = &mut self.index;
        let from_sources = &index.queries[scope.query].sources;
        let columns_start = index.columns.next_idx();
        JoinColumns {
            join_merges: &scope.merges,
            from_sources,
            source_arena: &index.sources,
            column_arena: &mut index.columns,
        }
        .expand_from(start, |column_arena, id| {
            column_arena.alloc(column_arena[id]);
        });
        let complete = from_sources[start..]
            .iter()
            .all(|&id| index.sources[id].columns.complete);
        index.queries[scope.query].sources.truncate(start);
        self.scope_mut()
            .merges
            .retain(|merge| merge.sources.start < start);
        let mut columns = Columns {
            range: self.index.columns.range_from(columns_start),
            complete,
        };
        let table_alias = alias.name();
        let name = table_alias
            .as_ref()
            .map(|table_alias| self.intern(&table_alias.text()));
        if let Some(alias_columns) = alias.columns() {
            columns = self.rename_columns(
                alias_columns.column_names(),
                &columns,
                ColumnListOwner::Join(name),
            );
        }
        self.check_duplicate_source(name, SourceKind::Join, true, from_item);
        let source = Source {
            name,
            kind: SourceKind::Join,
            ptr: self.ptr(from_item.syntax()),
            alias_ptr: table_alias.map(|table_alias| self.ast_ptr(&table_alias)),
            columns,
        };
        self.push_source(source);
    }

    fn push_source(&mut self, source: Source) {
        let alias_ptr = source.alias_ptr;
        let id = self.index.sources.alloc(source);
        if let Some(alias_ptr) = alias_ptr {
            self.add_item_range(alias_ptr.file_range(), Item::Source(id));
        }
        let query = self.scope().query;
        self.index.queries[query].sources.push(id);
    }

    fn push_scope(&mut self, query: QueryId) {
        let scope = self.index.scopes.alloc(Scope {
            query,
            parent: self.scopes.last().map(|scope| scope.scope),
            visible: Range::default(),
            hidden: vec![],
        });
        self.scopes.push(QueryScope {
            query,
            scope,
            merges: vec![],
            windows: Range::default(),
        });
    }

    fn scope(&self) -> &QueryScope {
        self.scopes
            .last()
            .expect("from items are lowered inside a select scope")
    }

    fn scope_mut(&mut self) -> &mut QueryScope {
        self.scopes
            .last_mut()
            .expect("from items are lowered inside a select scope")
    }

    fn scope_sources_len(&self) -> usize {
        self.index.queries[self.scope().query].sources.len()
    }

    fn with_from_function<R>(&mut self, f: impl FnOnce(&mut Self) -> R) -> R {
        self.with_lateral_sources(|this| this.with_expr_context(ExprContext::FromFunction, f))
    }

    fn with_lateral_sources<R>(&mut self, f: impl FnOnce(&mut Self) -> R) -> R {
        let len = self.scope_sources_len();
        self.with_visible_sources((0..len).into(), f)
    }

    fn with_visible_sources<R>(
        &mut self,
        visible: Range<usize>,
        f: impl FnOnce(&mut Self) -> R,
    ) -> R {
        let previous = self.scope().scope;
        self.set_visible(visible);
        let result = f(self);
        self.scope_mut().scope = previous;
        result
    }

    fn update_scope(&mut self, f: impl FnOnce(&mut Scope)) {
        let mut scope = self.index.scopes[self.scope().scope].clone();
        f(&mut scope);
        self.scope_mut().scope = self.index.scopes.alloc(scope);
    }

    fn set_visible(&mut self, visible: Range<usize>) {
        self.update_scope(|scope| scope.visible = visible);
    }

    fn push_hidden(&mut self, hidden: Range<usize>) {
        self.update_scope(|scope| scope.hidden.push(hidden));
    }

    fn pop_hidden(&mut self) {
        self.update_scope(|scope| {
            scope.hidden.pop();
        });
    }

    fn visible_sources(&self, scope: &QueryScope) -> impl Iterator<Item = SourceId> + '_ {
        let sources = &self.index.queries[scope.query].sources;
        self.index.scopes[scope.scope]
            .visible_indices()
            .map(|index| sources[index])
    }

    fn json_table_columns(&mut self, list: &ast::JsonTableColumnList, columns: &mut Vec<Column>) {
        for column in list.json_table_columns() {
            match column {
                ast::JsonTableColumn::JsonTableExistsColumn(column) => {
                    let ty = column.ty().and_then(|ty| self.lower_type(&ty));
                    self.lower_json_behavior(
                        column
                            .json_on_error_clause()
                            .and_then(|clause| clause.json_behavior()),
                    );
                    if let Some(column_name) = column.column_name() {
                        columns.push(Column {
                            ty,
                            ..self.column_def(&column_name)
                        });
                    }
                }
                ast::JsonTableColumn::JsonTableOrdinalityColumn(column) => {
                    if let Some(column_name) = column.column_name() {
                        columns.push(self.column_def(&column_name));
                    }
                }
                ast::JsonTableColumn::JsonTableValueColumn(column) => {
                    let ty = column.ty().and_then(|ty| self.lower_type(&ty));
                    self.lower_json_behavior(
                        column
                            .json_on_empty_clause()
                            .and_then(|clause| clause.json_behavior()),
                    );
                    self.lower_json_behavior(
                        column
                            .json_on_error_clause()
                            .and_then(|clause| clause.json_behavior()),
                    );
                    if let Some(column_name) = column.column_name() {
                        columns.push(Column {
                            ty,
                            ..self.column_def(&column_name)
                        });
                    }
                }
                ast::JsonTableColumn::JsonTableNestedColumn(nested) => {
                    if let Some(clause) = nested.json_path_name_clause() {
                        self.lower_json_path_name_clause(&clause);
                    }
                    if let Some(list) = nested.json_table_column_list() {
                        self.json_table_columns(&list, columns);
                    }
                }
            }
        }
    }

    fn lower_json_path_name_clause(&mut self, clause: &ast::JsonPathNameClause) {
        let Some(path_name) = clause.json_path_name() else {
            return;
        };
        let name = self.intern(&path_name.text());
        let id = self.index.json_paths.alloc(JsonPath {
            name,
            name_ptr: self.ast_ptr(&path_name),
        });
        self.add_item(path_name.syntax(), Item::JsonPath(id));
        self.json_paths.push(id);
    }

    fn find_json_path(&self, paths_mark: usize, name: Name) -> Option<JsonPathId> {
        self.json_paths[paths_mark..]
            .iter()
            .copied()
            .find(|&path| self.index.json_paths[path].name == name)
    }

    fn lower_json_table_plan(&mut self, plan: &ast::JsonTablePlan, paths_mark: usize) {
        match plan {
            ast::JsonTablePlan::JsonPathNameRef(name_ref) => {
                let name = self.intern(&name_ref.text());
                let resolution = self
                    .find_json_path(paths_mark, name)
                    .map_or(Resolution::Unresolved, Resolution::JsonPath);
                self.add_ref(name, ReferenceKind::JsonPath, name_ref.syntax(), resolution);
            }
            ast::JsonTablePlan::JsonTablePlanChoice(_) => {}
            ast::JsonTablePlan::JsonTablePlanJoin(join) => {
                if let Some(lhs) = join.lhs() {
                    self.lower_json_table_plan(&lhs, paths_mark);
                }
                if let Some(rhs) = join.rhs() {
                    self.lower_json_table_plan(&rhs, paths_mark);
                }
            }
            ast::JsonTablePlan::ParenJsonTablePlan(paren) => {
                if let Some(plan) = paren.json_table_plan() {
                    self.lower_json_table_plan(&plan, paths_mark);
                }
            }
        }
    }

    fn column_def_list_columns(&mut self, list: &ast::ColumnDefList) -> Columns {
        let start = self.index.columns.next_idx();
        for column_def in list.column_defs() {
            let ty = column_def.ty().and_then(|ty| self.lower_type(&ty));
            if let Some(collate) = column_def.collate() {
                self.lower_collate(&collate);
            }
            let Some(column_name) = column_def.name() else {
                continue;
            };
            let column = Column {
                ty,
                ..self.column_def(&column_name)
            };
            self.alloc_column_def(column);
        }
        Columns {
            range: self.index.columns.range_from(start),
            complete: true,
        }
    }

    fn check_column_list_len(&mut self, list: &Columns, columns: &Columns, owner: ColumnListOwner) {
        let available = columns.range.len();
        let specified = list.range.len();
        if !columns.complete || specified <= available {
            return;
        }
        let Some(extra) = list.range.iter().nth(available) else {
            return;
        };
        let too_long = |what, name| SemaError::ColumnListTooLong {
            what,
            name,
            available,
            specified,
        };
        let error = match owner {
            ColumnListOwner::Cte(name) => too_long("WITH query", name),
            ColumnListOwner::Table(Some(name)) => too_long("table", name),
            ColumnListOwner::Join(Some(name)) => too_long("join expression", name),
            ColumnListOwner::Table(None) | ColumnListOwner::Join(None) => return,
            ColumnListOwner::View => SemaError::ViewTooManyColumnNames,
            ColumnListOwner::CreateAs => SemaError::CreateAsTooManyColumnNames,
        };
        let ptr = self.index.columns[extra].ptr;
        self.push_error_ptr(ptr, error);
    }

    fn rename_columns(
        &mut self,
        column_names: impl Iterator<Item = ast::ColumnName>,
        columns: &Columns,
        owner: ColumnListOwner,
    ) -> Columns {
        let list = self.lower_column_names(column_names);
        self.apply_column_list(&list, columns, owner)
    }

    fn apply_column_list(
        &mut self,
        list: &Columns,
        columns: &Columns,
        owner: ColumnListOwner,
    ) -> Columns {
        self.check_column_list_len(list, columns, owner);
        for (column, query_column) in list.range.iter().zip(columns.range) {
            self.index.columns[column].ty = self.index.columns[query_column].ty;
        }
        Columns {
            range: self.index.columns.alloc_copies(
                list.range
                    .iter()
                    .chain(columns.range.iter().skip(list.range.len())),
            ),
            complete: columns.complete,
        }
    }

    fn lower_relation_ref(&mut self, path: &ast::PathRef) -> RelationSource {
        self.lower_relation_path(path, RelationLookup::Any)
    }

    fn lower_relation_path(
        &mut self,
        path: &ast::PathRef,
        lookup: RelationLookup,
    ) -> RelationSource {
        let Some(segment) = path.segment() else {
            return self.other_relation_source();
        };
        let name = self.intern(&segment.text());
        let schema = self.lower_schema_qualifier(path.qualifier());

        let cte = (matches!(lookup, RelationLookup::Any) && schema.is_none())
            .then(|| self.find_cte(name))
            .flatten();
        if let Some(cte) = cte
            && let Some(index) = self
                .pending_ctes
                .iter()
                .position(|pending| pending.id == cte)
        {
            let pending = self.pending_ctes.remove(index);
            self.lower_pending_cte(&pending);
        }
        let (kind, resolution, columns) = if let Some(cte) = cte {
            (
                SourceKind::Cte(cte),
                Resolution::Cte(cte),
                self.index.ctes[cte].columns,
            )
        } else if let Some(table) = self.find_table(schema, name) {
            (
                SourceKind::Table(table),
                Resolution::Table(table),
                self.index.tables[table].columns,
            )
        } else {
            (
                SourceKind::External,
                Resolution::Unresolved,
                self.unknown_columns(),
            )
        };
        self.add_ref(name, ReferenceKind::Table, segment.syntax(), resolution);
        RelationSource {
            name: Some(name),
            kind,
            columns,
        }
    }

    fn lower_rows_from_columns(&mut self, rows_from_item: &ast::RowsFromItem) -> Columns {
        let mut items: SmallVec<[Columns; 4]> = smallvec![];
        for arg in rows_from_item.rows_from_args() {
            let columns = match (arg.call_expr(), arg.column_def_list()) {
                (Some(call), Some(list)) => {
                    self.with_from_function(|this| this.lower_call(&call));
                    self.column_def_list_columns(&list)
                }
                (Some(call), None) => self.lower_function_source(&call, None, false).columns,
                (None, Some(list)) => self.column_def_list_columns(&list),
                (None, None) => self.unknown_columns(),
            };
            items.push(columns);
        }
        Columns {
            range: self
                .index
                .columns
                .alloc_copies(items.iter().flat_map(|columns| columns.range)),
            complete: items.iter().all(|columns| columns.complete),
        }
    }

    fn lower_function_source(
        &mut self,
        call: &ast::CallExpr,
        scalar_alias: Option<&ast::TableAlias>,
        has_column_defs: bool,
    ) -> FunctionSource {
        if !has_column_defs && let Some(name_ref) = expanded_unnest(call) {
            return self.lower_expanded_unnest(call, &name_ref);
        }
        let LoweredCall { args, callee } =
            self.with_from_function(|this| this.lower_call_with_args(call));
        let Some(Resolution::Function(function)) = callee.map(|callee| callee.resolution) else {
            return FunctionSource {
                callee,
                function: None,
                columns: self.unknown_columns(),
            };
        };
        let columns = self
            .function_row_columns(function, &args)
            .unwrap_or_else(|| self.scalar_function_columns(function, &args, call, scalar_alias));
        FunctionSource {
            callee,
            function: Some(function),
            columns,
        }
    }

    fn lower_expanded_unnest(
        &mut self,
        call: &ast::CallExpr,
        name_ref: &ast::NameRef,
    ) -> FunctionSource {
        let CallArgs { args, .. } =
            self.with_from_function(|this| this.lower_call_args(call.arg_list()));
        let name = self.intern("unnest");
        let mut resolutions: SmallVec<[Resolution; 4]> = smallvec![];
        let mut items: SmallVec<[Columns; 4]> = smallvec![];
        for arg in &args {
            let arg = std::slice::from_ref(arg);
            let resolution =
                self.find_routine(Some(self.pg_catalog), name, arg, FunctionKind::Function);
            let columns = match resolution {
                Resolution::Function(function) => self
                    .function_row_columns(function, arg)
                    .unwrap_or_else(|| self.scalar_function_columns(function, arg, call, None)),
                _ => self.unknown_columns(),
            };
            resolutions.push(resolution);
            items.push(columns);
        }
        let resolution = resolutions
            .iter()
            .copied()
            .find(|resolution| !matches!(resolution, Resolution::Function(_)))
            .or_else(|| resolutions.first().copied())
            .unwrap_or(Resolution::Unknown);
        self.add_ref(
            name,
            ReferenceKind::Function(RoutineKind::Function),
            name_ref.syntax(),
            resolution,
        );
        let columns = Columns {
            range: self
                .index
                .columns
                .alloc_copies(items.iter().flat_map(|columns| columns.range)),
            complete: items.iter().all(|columns| columns.complete),
        };
        FunctionSource {
            callee: Some(Callee { name, resolution }),
            function: None,
            columns,
        }
    }

    fn scalar_function_columns(
        &mut self,
        function: FunctionId,
        args: &[CallArg],
        call: &ast::CallExpr,
        scalar_alias: Option<&ast::TableAlias>,
    ) -> Columns {
        let record = self.builtin_type("record");
        let ret = self.index.functions[function].ret;
        if ret.is_none() || ret == record {
            return self.unknown_columns();
        }
        let (name, ptr) = match (scalar_alias, call.expr()) {
            (Some(alias), _) => (self.intern(&alias.text()), self.ptr(alias.syntax())),
            (None, Some(expr)) => (self.index.functions[function].name, self.ptr(expr.syntax())),
            (None, None) => (self.index.functions[function].name, self.ptr(call.syntax())),
        };
        let ty = self.call_type(function, args);
        let range = IdxRange::single(self.index.columns.alloc(Column {
            name: Some(name),
            ptr,
            ty,
            merge: None,
        }));
        Columns {
            range,
            complete: true,
        }
    }

    fn function_row_columns(&self, function: FunctionId, args: &[CallArg]) -> Option<Columns> {
        self.index.functions[function].columns.or_else(|| {
            self.call_type(function, args)
                .and_then(|ty| self.ty_columns(ty).copied())
        })
    }

    fn with_ordinality_column(
        &mut self,
        columns: &Columns,
        with_ordinality: &ast::WithOrdinality,
    ) -> Columns {
        let start = self.index.columns.next_idx();
        self.index.columns.alloc_copies(columns.range);
        let name = self.intern("ordinality");
        let ty = self.builtin_type("int8");
        self.index.columns.alloc(Column {
            name: Some(name),
            ptr: self.ptr(with_ordinality.syntax()),
            ty,
            merge: None,
        });
        Columns {
            range: self.index.columns.range_from(start),
            complete: columns.complete,
        }
    }

    fn other_relation_source(&mut self) -> RelationSource {
        RelationSource {
            name: None,
            kind: SourceKind::Other,
            columns: self.unknown_columns(),
        }
    }

    fn drop_table_ref(&mut self, path: &ast::PathRef, if_missing: IfMissing, lookup: TableLookup) {
        if let Some(table) = self.lower_table_ref(path, if_missing, lookup) {
            self.drop_table(table);
        }
    }

    fn lower_table_ref(
        &mut self,
        path: &ast::PathRef,
        if_missing: IfMissing,
        lookup: TableLookup,
    ) -> Option<TableId> {
        self.resolve_table_ref(path, if_missing, lookup, false)
    }

    fn lower_drop_table_ref(
        &mut self,
        path: &ast::PathRef,
        if_missing: IfMissing,
        lookup: TableLookup,
    ) -> Option<TableId> {
        self.resolve_table_ref(path, if_missing, lookup, true)
    }

    fn resolve_table_ref(
        &mut self,
        path: &ast::PathRef,
        if_missing: IfMissing,
        lookup: TableLookup,
        drop: bool,
    ) -> Option<TableId> {
        let segment = path.segment()?;
        let name = self.intern(&segment.text());
        let schema = self.lower_schema_qualifier(path.qualifier());
        let found = self.find_table(schema, name);
        let resolution = match found {
            Some(table) => Resolution::Table(table),
            None => match if_missing {
                IfMissing::Ignore => Resolution::Unknown,
                IfMissing::Error => Resolution::Unresolved,
            },
        };
        self.add_ref(name, ReferenceKind::Table, segment.syntax(), resolution);
        let table = found?;
        if !lookup.accepts(self.index.tables[table].kind) {
            self.report_wrong_relation_kind(table, lookup, segment.syntax(), drop);
            return None;
        }
        Some(table)
    }

    fn report_wrong_relation_kind(
        &mut self,
        table: TableId,
        lookup: TableLookup,
        node: &SyntaxNode,
        drop: bool,
    ) {
        let Table { kind, name, .. } = self.index.tables[table];
        let error = match lookup {
            TableLookup::CopyTo | TableLookup::CopyFrom => SemaError::CannotCopy {
                name,
                copy_from: matches!(lookup, TableLookup::CopyTo),
                kind,
            },
            TableLookup::Table
            | TableLookup::ForeignTable
            | TableLookup::TableOrForeignTable
            | TableLookup::TableOrView
            | TableLookup::TableOrMaterializedView
            | TableLookup::Any
            | TableLookup::View
            | TableLookup::MaterializedView
            | TableLookup::Sequence
            | TableLookup::PropertyGraph
            | TableLookup::Truncate => SemaError::WrongRelationKind {
                name,
                expected: lookup,
                kind,
                drop,
            },
        };
        self.push_error(node, error);
    }

    fn lower_alter_table(&mut self, alter_table: &ast::AlterTable) {
        if let Some(all_in_tablespace) = alter_table.all_in_tablespace() {
            self.lower_all_in_tablespace(&all_in_tablespace);
            return;
        }
        let relation_name = alter_table.table_relation_name();
        let Some(path) = relation_name
            .as_ref()
            .and_then(|name| name.table_name_ref())
            .and_then(|name| name.path_ref())
        else {
            return;
        };
        let table = self.lower_table_ref(
            &path,
            IfMissing::from_if_exists(alter_table.if_exists().as_ref()),
            TableLookup::TableOrForeignTable,
        );
        let recurse = relation_name.is_some_and(|name| name.only_token().is_none());
        self.lower_alter_table_actions(table, recurse, alter_table.actions(), alter_table.syntax());
    }

    fn lower_alter_foreign_table(&mut self, alter_table: &ast::AlterForeignTable) {
        let relation_name = alter_table.table_relation_name();
        let Some(path) = relation_name
            .as_ref()
            .and_then(|name| name.table_name_ref())
            .and_then(|name| name.path_ref())
        else {
            return;
        };
        let table = self.lower_table_ref(
            &path,
            IfMissing::from_if_exists(alter_table.if_exists().as_ref()),
            TableLookup::ForeignTable,
        );
        let recurse = relation_name.is_some_and(|name| name.only_token().is_none());
        self.lower_alter_table_actions(table, recurse, alter_table.actions(), alter_table.syntax());
    }

    fn lower_drop_foreign_table(&mut self, drop_table: &ast::DropForeignTable) {
        self.drop_relation_refs(
            drop_table
                .table_name_refs()
                .filter_map(|table| table.path_ref()),
            IfMissing::from_if_exists(drop_table.if_exists().as_ref()),
            TableLookup::ForeignTable,
            drop_table.drop_behavior().as_ref(),
        );
    }

    fn lower_trigger_target(&mut self, table: Option<TableId>, target: &ast::TriggerTarget) {
        match target {
            ast::TriggerTarget::TriggerRef(trigger_ref) => {
                self.lower_trigger_ref(table, trigger_ref, IfMissing::Error);
            }
            ast::TriggerTarget::All(_) | ast::TriggerTarget::User(_) => {}
        }
    }

    fn lower_alter_table_actions(
        &mut self,
        table: Option<TableId>,
        recurse: bool,
        actions: impl Iterator<Item = ast::AlterTableAction> + Clone,
        node: &SyntaxNode,
    ) {
        let mut dropped = vec![];
        for action in actions.clone() {
            let drop_column = match action {
                ast::AlterTableAction::DropColumn(drop_column) => drop_column,
                ast::AlterTableAction::SetTablespace(set_tablespace) => {
                    self.lower_set_tablespace(&set_tablespace);
                    continue;
                }
                ast::AlterTableAction::ClusterOn(cluster_on) => {
                    if let Some(path) = cluster_on.index_ref().and_then(|index| index.path_ref()) {
                        self.lower_index_ref(&path, IfMissing::Error);
                    }
                    continue;
                }
                ast::AlterTableAction::ReplicaIdentity(replica_identity) => {
                    match replica_identity.replica_identity_option() {
                        Some(ast::ReplicaIdentityOption::UsingIndexName(using)) => {
                            if let Some(path) = using.index_ref().and_then(|index| index.path_ref())
                            {
                                self.lower_index_ref(&path, IfMissing::Error);
                            }
                        }
                        Some(
                            ast::ReplicaIdentityOption::ReplicaIdentityDefault(_)
                            | ast::ReplicaIdentityOption::ReplicaIdentityFull(_)
                            | ast::ReplicaIdentityOption::ReplicaIdentityNothing(_),
                        )
                        | None => {}
                    }
                    continue;
                }
                ast::AlterTableAction::AddColumn(_)
                | ast::AlterTableAction::AddConstraint(_)
                | ast::AlterTableAction::AlterColumn(_) => continue,
                ast::AlterTableAction::DropConstraint(drop_constraint) => {
                    if let Some(name_ref) = drop_constraint.constraint_name_ref()
                        && let Some(constraint) = self.lower_constraint_ref(
                            table.map(ConstraintOwner::Table),
                            &name_ref,
                            IfMissing::from_if_exists(drop_constraint.if_exists().as_ref()),
                        )
                    {
                        self.drop_constraint(constraint);
                    }
                    continue;
                }
                ast::AlterTableAction::AlterConstraint(alter_constraint) => {
                    if let Some(name_ref) = alter_constraint.constraint_name_ref() {
                        self.lower_constraint_ref(
                            table.map(ConstraintOwner::Table),
                            &name_ref,
                            IfMissing::Error,
                        );
                    }
                    continue;
                }
                ast::AlterTableAction::ValidateConstraint(validate_constraint) => {
                    if let Some(name_ref) = validate_constraint.constraint_name_ref() {
                        self.lower_constraint_ref(
                            table.map(ConstraintOwner::Table),
                            &name_ref,
                            IfMissing::Error,
                        );
                    }
                    continue;
                }
                ast::AlterTableAction::DisableRule(disable_rule) => {
                    if let Some(rule_ref) = disable_rule.rule_ref() {
                        self.lower_rule_ref(table, &rule_ref, IfMissing::Error);
                    }
                    continue;
                }
                ast::AlterTableAction::EnableRule(enable_rule) => {
                    if let Some(rule_ref) = enable_rule.rule_ref() {
                        self.lower_rule_ref(table, &rule_ref, IfMissing::Error);
                    }
                    continue;
                }
                ast::AlterTableAction::EnableAlwaysRule(enable_rule) => {
                    if let Some(rule_ref) = enable_rule.rule_ref() {
                        self.lower_rule_ref(table, &rule_ref, IfMissing::Error);
                    }
                    continue;
                }
                ast::AlterTableAction::EnableReplicaRule(enable_rule) => {
                    if let Some(rule_ref) = enable_rule.rule_ref() {
                        self.lower_rule_ref(table, &rule_ref, IfMissing::Error);
                    }
                    continue;
                }
                ast::AlterTableAction::OwnerTo(owner_to) => {
                    self.lower_owner_to(&owner_to);
                    continue;
                }
                ast::AlterTableAction::SetAccessMethod(set_access_method) => {
                    if let Some(access_method_ref) = set_access_method.access_method_ref() {
                        self.lower_access_method_ref(
                            &access_method_ref,
                            Some(AccessMethodKind::Table),
                            IfMissing::Error,
                        );
                    }
                    continue;
                }
                ast::AlterTableAction::InheritTable(inherit) => {
                    if let Some(path) = inherit.table_name_ref().and_then(|name| name.path_ref())
                        && let Some(parent) = self.lower_table_ref(
                            &path,
                            IfMissing::Error,
                            TableLookup::TableOrForeignTable,
                        )
                        && let Some(table) = table
                    {
                        self.add_inheritance(parent, table, InheritanceKind::Inherits);
                    }
                    continue;
                }
                ast::AlterTableAction::NoInheritTable(no_inherit) => {
                    if let Some(path) = no_inherit.table_name_ref().and_then(|name| name.path_ref())
                        && let Some(parent) = self.lower_table_ref(
                            &path,
                            IfMissing::Error,
                            TableLookup::TableOrForeignTable,
                        )
                        && let Some(table) = table
                    {
                        self.remove_inheritance(parent, table, InheritanceKind::Inherits);
                    }
                    continue;
                }
                ast::AlterTableAction::OfType(of_type) => {
                    let ty = of_type.ty().and_then(|ty| self.lower_type(&ty));
                    if let Some(table) = table
                        && let Some(Ty::Type(ty)) = ty
                    {
                        self.set_table_of_type(table, Some(ty));
                    }
                    continue;
                }
                ast::AlterTableAction::NotOf(_) => {
                    if let Some(table) = table {
                        self.set_table_of_type(table, None);
                    }
                    continue;
                }
                ast::AlterTableAction::MergePartitions(merge) => {
                    self.lower_merge_partitions(table, &merge);
                    continue;
                }
                ast::AlterTableAction::SplitPartition(split) => {
                    self.lower_split_partition(table, &split);
                    continue;
                }
                ast::AlterTableAction::AttachPartition(attach) => {
                    if let Some(path) = attach.table_name_ref().and_then(|name| name.path_ref())
                        && let Some(partition) = self.lower_table_ref(
                            &path,
                            IfMissing::Error,
                            TableLookup::TableOrForeignTable,
                        )
                        && let Some(table) = table
                    {
                        self.add_inheritance(table, partition, InheritanceKind::Partition);
                    }
                    if let Some(partition_type) = attach.partition_type() {
                        self.lower_partition_type(&partition_type);
                    }
                    continue;
                }
                ast::AlterTableAction::DetachPartition(detach) => {
                    if let Some(path) = detach.table_name_ref().and_then(|name| name.path_ref())
                        && let Some(partition) = self.lower_table_ref(
                            &path,
                            IfMissing::Error,
                            TableLookup::TableOrForeignTable,
                        )
                        && let Some(table) = table
                    {
                        self.remove_inheritance(table, partition, InheritanceKind::Partition);
                    }
                    continue;
                }
                ast::AlterTableAction::RenameColumn(rename) => {
                    if let Some(column_ref) = rename.column_name_ref() {
                        let name = self.lower_constraint_column_ref(table, &column_ref);
                        if let Some(table) = table
                            && let Some(new_name) = rename.column_name()
                        {
                            self.rename_table_column(table, name, &new_name);
                        }
                    }
                    continue;
                }
                ast::AlterTableAction::TableRenameTo(rename) => {
                    if let Some(table) = table
                        && let Some(path) = rename.table_name().and_then(|name| name.path())
                    {
                        self.rename_table(table, &path);
                    }
                    continue;
                }
                ast::AlterTableAction::SetSchema(set_schema) => {
                    self.set_table_schema(table, &set_schema);
                    continue;
                }
                ast::AlterTableAction::DisableTrigger(disable_trigger) => {
                    if let Some(target) = disable_trigger.trigger_target() {
                        self.lower_trigger_target(table, &target);
                    }
                    continue;
                }
                ast::AlterTableAction::EnableTrigger(enable_trigger) => {
                    if let Some(target) = enable_trigger.trigger_target() {
                        self.lower_trigger_target(table, &target);
                    }
                    continue;
                }
                ast::AlterTableAction::EnableAlwaysTrigger(enable_trigger) => {
                    if let Some(trigger_ref) = enable_trigger.trigger_ref() {
                        self.lower_trigger_ref(table, &trigger_ref, IfMissing::Error);
                    }
                    continue;
                }
                ast::AlterTableAction::EnableReplicaTrigger(enable_trigger) => {
                    if let Some(trigger_ref) = enable_trigger.trigger_ref() {
                        self.lower_trigger_ref(table, &trigger_ref, IfMissing::Error);
                    }
                    continue;
                }
                ast::AlterTableAction::RenameConstraint(rename) => {
                    if let Some(name_ref) = rename.constraint_name_ref()
                        && let Some(constraint) = self.lower_constraint_ref(
                            table.map(ConstraintOwner::Table),
                            &name_ref,
                            IfMissing::Error,
                        )
                        && let Some(name) = rename.constraint_name()
                    {
                        self.rename_constraint(constraint, &name, node);
                    }
                    continue;
                }
                ast::AlterTableAction::AlterOptionList(_)
                | ast::AlterTableAction::DisableRls(_)
                | ast::AlterTableAction::EnableRls(_)
                | ast::AlterTableAction::ForceRls(_)
                | ast::AlterTableAction::NoForceRls(_)
                | ast::AlterTableAction::OptionItemList(_)
                | ast::AlterTableAction::ResetOptions(_)
                | ast::AlterTableAction::SetLogged(_)
                | ast::AlterTableAction::SetOptions(_)
                | ast::AlterTableAction::SetUnlogged(_)
                | ast::AlterTableAction::SetWithoutCluster(_)
                | ast::AlterTableAction::SetWithoutOids(_) => continue,
            };
            let Some(column_ref) = drop_column.column_name_ref() else {
                continue;
            };
            let name = self.intern(&column_ref.text());
            let column = table.and_then(|table| {
                self.index.tables[table]
                    .columns
                    .range
                    .iter()
                    .find(|column| {
                        self.index.columns[*column].name == Some(name) && !dropped.contains(column)
                    })
            });
            let resolution = match column {
                Some(column) => Resolution::Column {
                    source: None,
                    column,
                },
                None if table.is_none() || drop_column.if_exists().is_some() => Resolution::Unknown,
                None => Resolution::Unresolved,
            };
            self.add_ref(name, ReferenceKind::Column, column_ref.syntax(), resolution);
            if let Some(table) = table
                && column.is_some()
            {
                self.set_column_expr(table, name, None);
                self.drop_column_sequences(table, name);
            }
            dropped.extend(column);
        }

        if !dropped.is_empty()
            && let Some(table) = table
            && self.index.tables.is_local(table)
        {
            let columns = self.index.tables[table].columns;
            let range = self.index.columns.alloc_copies(
                columns
                    .range
                    .iter()
                    .filter(|column| !dropped.contains(column)),
            );
            if self.in_transaction() {
                self.undo.push(Undo::TableColumns(table, columns));
            }
            self.index
                .tables
                .set_columns(table, Columns { range, ..columns }, &self.index.columns);
            if recurse {
                for &column in &dropped {
                    self.drop_inherited_column(table, self.index.columns[column]);
                }
            }
        }

        for action in actions.clone() {
            if let ast::AlterTableAction::AlterColumn(alter_column) = action
                && alter_column.is_set_type()
            {
                self.lower_alter_column(table, &alter_column, node);
            }
        }

        for action in actions.clone() {
            if let ast::AlterTableAction::AddColumn(add_column) = action {
                self.lower_add_column(table, &add_column, node);
            }
        }

        for action in actions {
            if let ast::AlterTableAction::AddConstraint(add_constraint) = &action
                && let Some(constraint) = add_constraint.constraint()
            {
                self.with_table_scope(QueryKind::Constraint, table, node, |this| {
                    this.lower_constraint(table, None, &constraint);
                });
            }
            if let ast::AlterTableAction::AlterColumn(alter_column) = &action
                && !alter_column.is_set_type()
            {
                self.lower_alter_column(table, alter_column, node);
            }
        }
    }

    fn lower_merge_partitions(&mut self, parent: Option<TableId>, merge: &ast::MergePartitions) {
        for path in merge
            .table_name_ref_list()
            .iter()
            .flat_map(|list| list.table_name_refs())
            .filter_map(|name| name.path_ref())
        {
            self.drop_table_ref(&path, IfMissing::Error, TableLookup::Table);
        }
        let Some(path) = merge.table_name().and_then(|name| name.path()) else {
            return;
        };
        let columns = self.partition_columns(parent);
        if let Some(partition) = self.alloc_table(
            TableKind::Table,
            Persistence::Permanent,
            IfExists::Error,
            merge.syntax(),
            &path,
            columns,
        ) {
            self.index.tables.set_constraints_complete(partition, false);
            if let Some(parent) = parent {
                self.add_inheritance(parent, partition, InheritanceKind::Partition);
            }
        }
    }

    fn lower_split_partition(&mut self, parent: Option<TableId>, split: &ast::SplitPartition) {
        if let Some(path) = split.table_name_ref().and_then(|name| name.path_ref()) {
            self.drop_table_ref(&path, IfMissing::Error, TableLookup::Table);
        }
        for partition in split
            .partition_list()
            .iter()
            .flat_map(|list| list.partitions())
        {
            if let Some(partition_type) = partition.partition_type() {
                self.lower_partition_type(&partition_type);
            }
            let Some(path) = partition.table_name_ref().and_then(|name| name.path_ref()) else {
                continue;
            };
            let Some(segment) = path.segment() else {
                continue;
            };
            let name = self.intern(&segment.text());
            let schema = self
                .lower_schema_qualifier(path.qualifier())
                .unwrap_or_else(|| self.default_creation_schema());
            let columns = self.partition_columns(parent);
            let id = self.alloc_table_in(
                TableKind::Table,
                IfExists::Error,
                partition.syntax(),
                schema,
                name,
                segment.syntax(),
                columns,
            );
            self.index.tables.set_constraints_complete(id, false);
            if let Some(parent) = parent {
                self.add_inheritance(parent, id, InheritanceKind::Partition);
            }
        }
    }

    fn partition_columns(&mut self, parent: Option<TableId>) -> Columns {
        match parent {
            Some(parent) => {
                let columns = self.index.tables[parent].columns;
                Columns {
                    range: self.index.columns.alloc_copies(columns.range),
                    complete: columns.complete,
                }
            }
            None => self.unknown_columns(),
        }
    }

    fn lower_alter_column(
        &mut self,
        table: Option<TableId>,
        alter_column: &ast::AlterColumn,
        node: &SyntaxNode,
    ) {
        let Some(column_ref) = alter_column.column_name_ref() else {
            return;
        };
        let column = self.lower_constraint_column_ref(table, &column_ref);
        match alter_column.option() {
            Some(ast::AlterColumnOption::SetType(set_type)) => {
                let ty = set_type.ty().and_then(|ty| self.lower_type(&ty));
                if let Some(collate) = set_type.collate() {
                    self.lower_collate(&collate);
                }
                if let Some(expr) = set_type.using_expr().and_then(|using| using.expr()) {
                    self.with_table_scope(QueryKind::Constraint, table, node, |this| {
                        this.lower_expr_in(ExprContext::Transform, &expr);
                    });
                }
                if let Some(table) = table
                    && self.index.tables.is_local(table)
                {
                    self.set_table_column_type(table, column, ty);
                    self.set_inherited_column_type(table, column, ty);
                }
            }
            Some(ast::AlterColumnOption::SetDefault(set_default)) => {
                if let Some(expr) = set_default.expr() {
                    self.lower_column_expr(table, Some(column), &expr, ColumnExprKind::Default);
                }
            }
            Some(ast::AlterColumnOption::SetExpression(set_expression)) => {
                if let Some(expr) = set_expression.expr() {
                    self.with_table_scope(QueryKind::Constraint, table, node, |this| {
                        this.lower_column_expr(
                            table,
                            Some(column),
                            &expr,
                            ColumnExprKind::Generated,
                        );
                    });
                }
            }
            Some(
                ast::AlterColumnOption::DropDefault(_) | ast::AlterColumnOption::DropExpression(_),
            ) => {
                if let Some(table) = table {
                    self.set_column_expr(table, column, None);
                }
            }
            Some(ast::AlterColumnOption::SetNotNull(set_not_null)) => {
                if let Some(table) = table {
                    self.add_not_null_constraint(table, column, set_not_null.syntax(), None);
                }
            }
            Some(ast::AlterColumnOption::DropNotNull(_)) => {
                let Some(table) = table else {
                    return;
                };
                let kind = ConstraintKind::NotNull { column };
                let constraint = self
                    .index
                    .constraints
                    .owned_by(ConstraintOwner::Table(table))
                    .rev()
                    .find(|id| {
                        self.index.constraints[*id].kind == kind
                            && !self.dropped_constraints.contains(id)
                    });
                if let Some(constraint) = constraint {
                    self.drop_constraint(constraint);
                }
            }
            Some(ast::AlterColumnOption::AddGenerated(add_generated)) => {
                self.lower_identity(
                    table,
                    Some(column),
                    add_generated.sequence_option_list(),
                    add_generated.syntax(),
                );
            }
            Some(ast::AlterColumnOption::SetSequenceOption(set_option)) => {
                if let Some(option) = set_option.sequence_option() {
                    self.lower_sequence_option(&option);
                }
            }
            Some(ast::AlterColumnOption::SetGeneratedOptions(options)) => {
                for option in options.set_generated_options() {
                    match option {
                        ast::SetGeneratedOption::SetSequenceOption(set_option) => {
                            if let Some(option) = set_option.sequence_option() {
                                self.lower_sequence_option(&option);
                            }
                        }
                        ast::SetGeneratedOption::Restart(_)
                        | ast::SetGeneratedOption::SetGenerated(_) => {}
                    }
                }
            }
            Some(ast::AlterColumnOption::DropIdentity(drop_identity)) => {
                let Some(table) = table else {
                    return;
                };
                let sequence = self
                    .index
                    .tables
                    .owned_sequences(table)
                    .into_iter()
                    .find(|id| {
                        self.index.tables[*id]
                            .sequence_owner
                            .is_some_and(|owner| owner.column == column && owner.identity)
                            && !self.dropped_tables.contains(id)
                    });
                match sequence {
                    Some(sequence) => self.drop_table(sequence),
                    None if drop_identity.if_exists().is_none()
                        && self.index.tables[table].constraints_complete
                        && self.columns_contain(self.index.tables[table].columns.range, column) =>
                    {
                        let error = SemaError::NotIdentityColumn {
                            column,
                            relation: self.index.tables[table].name,
                        };
                        self.push_error(column_ref.syntax(), error);
                    }
                    None => {}
                }
            }
            Some(
                ast::AlterColumnOption::Inherit(_)
                | ast::AlterColumnOption::NoInherit(_)
                | ast::AlterColumnOption::ResetOptions(_)
                | ast::AlterColumnOption::Restart(_)
                | ast::AlterColumnOption::SetCompression(_)
                | ast::AlterColumnOption::SetGenerated(_)
                | ast::AlterColumnOption::SetOptions(_)
                | ast::AlterColumnOption::SetOptionsList(_)
                | ast::AlterColumnOption::SetStatistics(_)
                | ast::AlterColumnOption::SetStorage(_),
            )
            | None => {}
        }
    }

    fn lower_add_column(
        &mut self,
        table: Option<TableId>,
        add_column: &ast::AddColumn,
        node: &SyntaxNode,
    ) {
        let ty = add_column.ty().and_then(|ty| self.lower_column_type(&ty));
        if let Some(collate) = add_column.collate() {
            self.lower_collate(&collate);
        }
        let column_name = add_column.column_name();
        let name = column_name
            .as_ref()
            .map(|column_name| self.intern(&column_name.text()));
        if let Some(table) = table
            && let Some(column_name) = &column_name
            && let Some(name) = name
            && self.index.tables.is_local(table)
        {
            let columns = self.index.tables[table].columns;
            if self.columns_contain(columns.range, name) {
                if add_column.if_not_exists().is_none() {
                    self.report_duplicate_column(
                        self.index.tables[table].name,
                        column_name.syntax(),
                        name,
                    );
                }
            } else {
                let start = self.index.columns.next_idx();
                self.index.columns.alloc_copies(columns.range);
                let column = Column {
                    ty,
                    ..self.column_def(column_name)
                };
                self.alloc_column_def(column);
                self.set_table_columns(table, self.index.columns.range_from(start));
                self.add_inherited_column(table, column);
                if let Some(ty) = add_column.ty() {
                    self.lower_serial_column(table, name, &ty, add_column.syntax());
                }
            }
        }
        if add_column.constraints().next().is_none() {
            return;
        }
        if let (Some(table), Some(column_name)) = (table, name) {
            self.check_column_nullability(table, column_name, add_column.constraints());
        }
        self.with_table_scope(QueryKind::Constraint, table, node, |this| {
            for constraint in add_column.constraints() {
                this.lower_constraint(table, name, &constraint);
            }
        });
    }

    fn is_live_child(&self, child: TableId) -> bool {
        !self.dropped_tables.contains(&child) && self.index.tables.is_local(child)
    }

    fn add_inherited_column(&mut self, parent: TableId, column: Column) {
        let Some(name) = column.name else {
            return;
        };
        let mut i = 0;
        while let Some(&inheritance) = self.inheritances.children(parent).get(i) {
            i += 1;
            if !self.is_live_child(inheritance.child) {
                continue;
            }
            let child = inheritance.child;
            let columns = self.index.tables[child].columns;
            if self.columns_contain(columns.range, name) {
                continue;
            }
            let start = self.index.columns.next_idx();
            self.index.columns.alloc_copies(columns.range);
            self.index.columns.alloc(column);
            self.set_table_columns(child, self.index.columns.range_from(start));
            self.add_inherited_column(child, column);
        }
    }

    fn drop_inherited_column(&mut self, parent: TableId, column: Column) {
        let Some(name) = column.name else {
            return;
        };
        let mut i = 0;
        while let Some(&inheritance) = self.inheritances.children(parent).get(i) {
            i += 1;
            if !self.is_live_child(inheritance.child) {
                continue;
            }
            let child = inheritance.child;
            let columns = self.index.tables[child].columns;
            let Some(inherited) = self.find_column_in(columns.range, name) else {
                continue;
            };
            if self.index.columns[inherited].ptr != column.ptr {
                continue;
            }
            if let Some(other) = self.other_parent_column(child, parent, name) {
                let ptr = self.index.columns[other].ptr;
                self.repoint_inherited_column(child, column, ptr);
                continue;
            }
            let range = self
                .index
                .columns
                .alloc_copies(columns.range.iter().filter(|id| *id != inherited));
            self.set_table_columns(child, range);
            self.set_column_expr(child, name, None);
            self.drop_column_sequences(child, name);
            self.drop_inherited_column(child, column);
        }
    }

    fn other_parent_column(&self, child: TableId, parent: TableId, name: Name) -> Option<ColumnId> {
        self.inheritances
            .parents(child)
            .iter()
            .find_map(|inheritance| {
                if inheritance.parent == parent || self.dropped_tables.contains(&inheritance.parent)
                {
                    return None;
                }
                self.find_column_in(self.index.tables[inheritance.parent].columns.range, name)
            })
    }

    fn repoint_inherited_column(
        &mut self,
        table: TableId,
        column: Column,
        ptr: InIrFile<SyntaxNodePtr>,
    ) {
        let columns = self.index.tables[table].columns;
        let range = self.copy_columns(columns.range, |columns, id| {
            let mut copy = columns[id];
            if copy.name == column.name && copy.ptr == column.ptr {
                copy.ptr = ptr;
            }
            Some(copy)
        });
        self.set_table_columns(table, range);
        let mut i = 0;
        while let Some(&inheritance) = self.inheritances.children(table).get(i) {
            i += 1;
            if self.is_live_child(inheritance.child) {
                self.repoint_inherited_column(inheritance.child, column, ptr);
            }
        }
    }

    fn set_inherited_column_type(&mut self, parent: TableId, name: Name, ty: Option<Ty>) {
        let mut i = 0;
        while let Some(&inheritance) = self.inheritances.children(parent).get(i) {
            i += 1;
            if !self.is_live_child(inheritance.child) {
                continue;
            }
            let child = inheritance.child;
            if !self.columns_contain(self.index.tables[child].columns.range, name) {
                continue;
            }
            self.set_table_column_type(child, name, ty);
            self.set_inherited_column_type(child, name, ty);
        }
    }

    fn lower_create_index(&mut self, create_index: &ast::CreateIndex) {
        let refs = self.index.references.next_idx();
        let start = self.index.indexes.next_idx();
        let access_method = match create_index.using_method() {
            Some(using_method) => self.lower_using_method(&using_method, AccessMethodKind::Index),
            None => {
                let btree = self.intern("btree");
                self.find_access_method(btree)
            }
        };
        if let Some(tablespace) = create_index.tablespace_clause() {
            self.lower_tablespace_clause(&tablespace);
        }
        let query = self.alloc_query(QueryKind::Index, create_index.syntax());
        self.push_scope(query);
        let relation_name = create_index.table_relation_name();
        let path = relation_name
            .as_ref()
            .and_then(|name| name.table_name_ref())
            .and_then(|name| name.path_ref());
        let table = relation_name.map(|relation_name| {
            let source = self.push_target_source(path.clone(), None, relation_name.syntax());
            match self.index.sources[source].kind {
                SourceKind::Table(table) => Some(table),
                SourceKind::Cte(_)
                | SourceKind::External
                | SourceKind::Subquery(_)
                | SourceKind::Function(_)
                | SourceKind::Join
                | SourceKind::UsingAlias
                | SourceKind::Returning(_)
                | SourceKind::Excluded(_)
                | SourceKind::RuleRelation(_)
                | SourceKind::ElementVariable
                | SourceKind::Other => None,
            }
        });
        let table = table.flatten();
        self.set_visible((0..self.scope_sources_len()).into());
        for list in create_index.partition_item_list().into_iter().chain(
            create_index
                .index_include_clause()
                .and_then(|include| include.partition_item_list()),
        ) {
            for item in list.partition_items() {
                if let Some(path) = item.op_class_ref().and_then(|op_class| op_class.path_ref()) {
                    self.lower_operator_class_ref(&path, access_method, IfMissing::Error);
                }
                if let Some(expr) = item.expr() {
                    self.lower_expr_in(ExprContext::IndexExpression, &expr);
                }
            }
        }
        if let Some(expr) = create_index.where_clause().and_then(|clause| clause.expr()) {
            self.lower_expr_in(ExprContext::IndexPredicate, &expr);
        }
        self.scopes.pop();

        if let Some(table) = table
            && let Some(table_segment) = path.as_ref().and_then(|path| path.segment())
        {
            let Table { kind, name, .. } = self.index.tables[table];
            match kind {
                TableKind::Table | TableKind::MaterializedView => {}
                TableKind::ForeignTable
                | TableKind::View
                | TableKind::Sequence
                | TableKind::PropertyGraph => {
                    self.push_error(
                        table_segment.syntax(),
                        SemaError::CannotCreateIndex { name, kind },
                    );
                    return;
                }
            }
        }
        let Some(segment) = create_index
            .index()
            .and_then(|index| index.path())
            .and_then(|path| path.segment())
        else {
            return;
        };
        let schema = match table {
            Some(table) => self.index.tables[table].schema,
            None => self
                .lower_schema_qualifier(path.and_then(|path| path.qualifier()))
                .unwrap_or_else(|| self.default_creation_schema()),
        };
        self.alloc_index(
            IfExists::from_if_not_exists(create_index.if_not_exists().as_ref()),
            create_index.syntax(),
            &segment,
            schema,
            table,
        );
        self.depend_on_refs(refs, self.index.indexes.range_from(start), Object::Index);
    }

    fn lower_trigger_function(&mut self, callee: &ast::Expr, kind: TriggerFunctionKind) {
        if let Some(Callee {
            name,
            resolution: Resolution::Function(function),
        }) = self.lower_callee(callee, &[], None, CalleeKind::Plain)
            && let Some(ret) = self.index.functions[function].ret
            && let Some(expected) = self.builtin_type(kind.return_type())
            && ret != expected
        {
            self.push_error(callee.syntax(), SemaError::TriggerReturnType { name, kind });
        }
    }

    fn lower_create_trigger(&mut self, create_trigger: &ast::CreateTrigger) {
        let refs = self.index.references.next_idx();
        let start = self.index.triggers.next_idx();
        let query = self.alloc_query(QueryKind::Trigger, create_trigger.syntax());
        self.push_scope(query);
        let mut table = None;
        if let Some(on_relation) = create_trigger.on_relation() {
            let RelationSource { kind, columns, .. } = match on_relation
                .relation_name_ref()
                .and_then(|name| name.path_ref())
            {
                Some(path) => self.lower_relation_path(&path, RelationLookup::OnlyTable),
                None => self.other_relation_source(),
            };
            if let SourceKind::Table(id) = kind {
                table = Some(id);
            }
            for name in ["old", "new"] {
                let name = self.intern(name);
                self.push_source(Source {
                    name: Some(name),
                    kind,
                    ptr: self.ptr(on_relation.syntax()),
                    alias_ptr: None,
                    columns,
                });
            }
            for event in create_trigger
                .trigger_event_list()
                .into_iter()
                .flat_map(|list| list.trigger_events())
            {
                match event {
                    ast::TriggerEvent::TriggerEventUpdate(update) => {
                        for column_ref in update.column_name_refs() {
                            self.lower_column_ref_in(&columns, &column_ref);
                        }
                    }
                    ast::TriggerEvent::TriggerEventDelete(_)
                    | ast::TriggerEvent::TriggerEventInsert(_)
                    | ast::TriggerEvent::TriggerEventTruncate(_) => {}
                }
            }
        }
        self.set_visible((0..self.scope_sources_len()).into());
        if let Some(expr) = create_trigger
            .when_condition()
            .and_then(|condition| condition.expr())
        {
            self.lower_expr_in(ExprContext::TriggerWhen, &expr);
        }
        self.scopes.pop();

        if let Some(path) = create_trigger
            .from_table()
            .and_then(|from_table| from_table.table_name_ref())
            .and_then(|name| name.path_ref())
        {
            self.lower_relation_path(&path, RelationLookup::OnlyTable);
        }
        if let Some(callee) = create_trigger.call_expr().and_then(|call| call.expr()) {
            self.lower_trigger_function(&callee, TriggerFunctionKind::Trigger);
        }

        let constraint = create_trigger.constraint_token().is_some();
        if constraint && let Some(or_replace) = create_trigger.or_replace() {
            self.push_error(or_replace.syntax(), SemaError::ReplaceConstraintTrigger);
        }
        if let (Some(table), Some(name)) = (table, create_trigger.trigger()) {
            self.alloc_trigger(
                IfExists::from_or_replace(create_trigger.or_replace().as_ref()),
                create_trigger.syntax(),
                &name,
                table,
                constraint,
            );
        }
        self.depend_on_refs(refs, self.index.triggers.range_from(start), Object::Trigger);
    }

    fn alloc_trigger(
        &mut self,
        if_exists: IfExists,
        node: &SyntaxNode,
        name_node: &impl NameLike,
        table: TableId,
        constraint: bool,
    ) {
        let name = self.intern(&name_node.text());
        if let Some(existing) = self.find_trigger(table, name) {
            match if_exists {
                IfExists::Error | IfExists::Ignore => {
                    self.report_duplicate_trigger(name_node.syntax(), name, table);
                }
                IfExists::Replace if self.index.triggers[existing].constraint => {
                    let error = SemaError::ConstraintTrigger {
                        name,
                        relation: self.index.tables[table].name,
                    };
                    self.push_error(name_node.syntax(), error);
                }
                IfExists::Replace => self.drop_trigger(existing),
            }
        }
        let id = self.index.triggers.alloc(Trigger {
            name,
            table,
            constraint,
            ptr: self.ptr(node),
            name_ptr: self.ptr(name_node.syntax()),
        });
        self.record_comment(node, Item::Trigger(id));
        self.add_item(name_node.syntax(), Item::Trigger(id));
    }

    fn report_duplicate_trigger(&mut self, name_node: &SyntaxNode, name: Name, table: TableId) {
        let table = self.index.tables[table].name;
        self.push_error(name_node, SemaError::DuplicateTrigger { name, table });
    }

    fn lower_on_relation(
        &mut self,
        on_relation: &ast::OnRelation,
        if_missing: IfMissing,
    ) -> Option<TableId> {
        self.lower_relation_name_ref(&on_relation.relation_name_ref()?, if_missing)
    }

    fn lower_relation_name_ref(
        &mut self,
        relation_name_ref: &ast::RelationNameRef,
        if_missing: IfMissing,
    ) -> Option<TableId> {
        let path = relation_name_ref.path_ref()?;
        let segment = path.segment()?;
        let name = self.intern(&segment.text());
        let schema = self.lower_schema_qualifier(path.qualifier());
        let table = self.find_table(schema, name);
        let resolution = match (table, if_missing) {
            (Some(table), _) => Resolution::Table(table),
            (None, IfMissing::Ignore) => Resolution::Unknown,
            (None, IfMissing::Error) => Resolution::Unresolved,
        };
        self.add_ref(name, ReferenceKind::Table, segment.syntax(), resolution);
        table
    }

    fn lower_trigger_ref(
        &mut self,
        table: Option<TableId>,
        trigger_ref: &ast::TriggerRef,
        if_missing: IfMissing,
    ) -> Option<TriggerId> {
        let name = self.intern(&trigger_ref.text());
        let trigger = table.and_then(|table| self.find_trigger(table, name));
        let resolution = match (trigger, table, if_missing) {
            (Some(trigger), _, _) => Resolution::Trigger(trigger),
            (None, Some(_), IfMissing::Error) => Resolution::Unresolved,
            (None, Some(_), IfMissing::Ignore) | (None, None, _) => Resolution::Unknown,
        };
        self.add_ref(
            name,
            ReferenceKind::Trigger,
            trigger_ref.syntax(),
            resolution,
        );
        trigger
    }

    fn lower_drop_trigger(&mut self, drop_trigger: &ast::DropTrigger) {
        let if_missing = IfMissing::from_if_exists(drop_trigger.if_exists().as_ref());
        let table = drop_trigger
            .on_relation()
            .and_then(|on_relation| self.lower_on_relation(&on_relation, if_missing));
        if let Some(trigger_ref) = drop_trigger.trigger_ref()
            && let Some(trigger) = self.lower_trigger_ref(table, &trigger_ref, if_missing)
        {
            self.drop_trigger(trigger);
        }
    }

    fn lower_alter_trigger(&mut self, alter_trigger: &ast::AlterTrigger) {
        let table = alter_trigger
            .on_relation()
            .and_then(|on_relation| self.lower_on_relation(&on_relation, IfMissing::Error));
        let trigger = alter_trigger
            .trigger_ref()
            .and_then(|trigger_ref| self.lower_trigger_ref(table, &trigger_ref, IfMissing::Error));
        match alter_trigger.action() {
            Some(ast::AlterTriggerAction::TriggerRenameTo(rename)) => {
                if let Some(trigger) = trigger
                    && let Some(name) = rename.trigger()
                {
                    let Trigger {
                        table, constraint, ..
                    } = self.index.triggers[trigger];
                    let new_name = self.intern(&name.text());
                    if self.find_trigger(table, new_name).is_some() {
                        self.report_duplicate_trigger(name.syntax(), new_name, table);
                        return;
                    }
                    let moved = self.index.triggers.next_idx();
                    self.drop_trigger(trigger);
                    self.alloc_trigger(
                        IfExists::Error,
                        alter_trigger.syntax(),
                        &name,
                        table,
                        constraint,
                    );
                    self.record_moves(
                        Object::Trigger(trigger),
                        self.index.triggers.range_from(moved),
                        Object::Trigger,
                    );
                }
            }
            Some(ast::AlterTriggerAction::DependsOnExtension(depends)) => {
                if let Some(extension) = depends.extension_ref() {
                    self.lower_extension_ref(&extension, IfMissing::Error);
                }
            }
            Some(ast::AlterTriggerAction::NoDependsOnExtension(depends)) => {
                if let Some(extension) = depends.extension_ref() {
                    self.lower_extension_ref(&extension, IfMissing::Error);
                }
            }
            None => {}
        }
    }

    fn find_trigger(&self, table: TableId, name: Name) -> Option<TriggerId> {
        self.index
            .triggers
            .get(table)
            .find(|id| self.index.triggers[*id].name == name && !self.dropped_triggers.contains(id))
    }

    fn lower_create_rule(&mut self, create_rule: &ast::CreateRule) {
        let refs = self.index.references.next_idx();
        let start = self.index.rules.next_idx();
        let query = self.alloc_query(QueryKind::Rule, create_rule.syntax());
        self.push_scope(query);
        let mut table = None;
        if let Some(rule_on) = create_rule.rule_on() {
            let RelationSource { kind, columns, .. } =
                match rule_on.relation_name_ref().and_then(|name| name.path_ref()) {
                    Some(path) => self.lower_relation_path(&path, RelationLookup::OnlyTable),
                    None => self.other_relation_source(),
                };
            if let SourceKind::Table(id) = kind {
                table = Some(id);
            }
            let names = [self.intern("old"), self.intern("new")];
            for name in names {
                self.push_source(Source {
                    name: Some(name),
                    kind,
                    ptr: self.ptr(rule_on.syntax()),
                    alias_ptr: None,
                    columns,
                });
            }
            self.set_visible((0..2).into());
            if let Some(expr) = rule_on.where_clause().and_then(|clause| clause.expr()) {
                self.lower_expr_in(ExprContext::Where, &expr);
            }
            for (index, name) in names.into_iter().enumerate() {
                let target = self.index.queries[query].sources[index];
                self.push_source(Source {
                    name: Some(name),
                    kind: SourceKind::RuleRelation(target),
                    ptr: self.ptr(rule_on.syntax()),
                    alias_ptr: None,
                    columns,
                });
            }
            self.set_visible((2..4).into());
        }
        match create_rule
            .rule_do()
            .and_then(|rule_do| rule_do.rule_action())
        {
            Some(ast::RuleAction::RuleStmt(stmt)) => self.lower_rule_stmt(stmt),
            Some(ast::RuleAction::RuleStmtList(list)) => {
                for stmt in list.rule_stmts() {
                    self.lower_rule_stmt(stmt);
                }
            }
            Some(ast::RuleAction::Nothing(_)) | None => {}
        }
        self.scopes.pop();

        let event = create_rule
            .rule_on()
            .and_then(|rule_on| rule_on.rule_event())
            .and_then(|event| event.kind())
            .map(|kind| match kind {
                ast::RuleEventKind::Select => RuleEvent::Select,
                ast::RuleEventKind::Insert => RuleEvent::Insert,
                ast::RuleEventKind::Update => RuleEvent::Update,
                ast::RuleEventKind::Delete => RuleEvent::Delete,
            });
        if let (Some(table), Some(name), Some(event)) = (table, create_rule.rule(), event) {
            self.alloc_rule(
                IfExists::from_or_replace(create_rule.or_replace().as_ref()),
                create_rule.syntax(),
                &name,
                table,
                event,
            );
        }
        self.depend_on_refs(refs, self.index.rules.range_from(start), Object::Rule);
    }

    fn lower_rule_stmt(&mut self, stmt: ast::RuleStmt) {
        match stmt {
            ast::RuleStmt::Delete(delete) => {
                self.lower_delete(&delete);
            }
            ast::RuleStmt::Insert(insert) => {
                self.lower_insert(&insert);
            }
            ast::RuleStmt::Update(update) => {
                self.lower_update(&update);
            }
            ast::RuleStmt::SelectVariant(query) => {
                self.lower_query(query);
            }
            ast::RuleStmt::Notify(notify) => self.lower_notify(&notify),
        }
    }

    fn alloc_rule(
        &mut self,
        if_exists: IfExists,
        node: &SyntaxNode,
        name_node: &impl NameLike,
        table: TableId,
        event: RuleEvent,
    ) {
        let name = self.intern(&name_node.text());
        if let Some(existing) = self.find_rule(table, name) {
            match if_exists {
                IfExists::Error | IfExists::Ignore => {
                    self.report_duplicate_rule(name_node.syntax(), name, table);
                }
                IfExists::Replace => self.drop_rule(existing),
            }
        }
        let id = self.index.rules.alloc(Rule {
            name,
            table,
            event,
            ptr: self.ptr(node),
            name_ptr: self.ptr(name_node.syntax()),
        });
        self.record_comment(node, Item::Rule(id));
        self.add_item(name_node.syntax(), Item::Rule(id));
    }

    fn alloc_return_rule(&mut self, table: TableId) {
        let Table { ptr, name_ptr, .. } = self.index.tables[table];
        let name = self.intern("_RETURN");
        self.index.rules.alloc(Rule {
            name,
            table,
            event: RuleEvent::Select,
            ptr,
            name_ptr,
        });
    }

    fn report_duplicate_rule(&mut self, name_node: &SyntaxNode, name: Name, table: TableId) {
        let table = self.index.tables[table].name;
        self.push_error(name_node, SemaError::DuplicateRule { name, table });
    }

    fn lower_rule_ref(
        &mut self,
        table: Option<TableId>,
        rule_ref: &ast::RuleRef,
        if_missing: IfMissing,
    ) -> Option<RuleId> {
        let name = self.intern(&rule_ref.text());
        let rule = table.and_then(|table| self.find_rule(table, name));
        let resolution = match (rule, table, if_missing) {
            (Some(rule), _, _) => Resolution::Rule(rule),
            (None, Some(_), IfMissing::Error) => Resolution::Unresolved,
            (None, Some(_), IfMissing::Ignore) | (None, None, _) => Resolution::Unknown,
        };
        self.add_ref(name, ReferenceKind::Rule, rule_ref.syntax(), resolution);
        rule
    }

    fn lower_drop_rule(&mut self, drop_rule: &ast::DropRule) {
        let if_missing = IfMissing::from_if_exists(drop_rule.if_exists().as_ref());
        let table = drop_rule
            .on_relation()
            .and_then(|on_relation| self.lower_on_relation(&on_relation, if_missing));
        let Some(rule_ref) = drop_rule.rule_ref() else {
            return;
        };
        let Some(rule) = self.lower_rule_ref(table, &rule_ref, if_missing) else {
            return;
        };
        let Rule {
            name, table, event, ..
        } = self.index.rules[rule];
        let Table {
            kind, name: table, ..
        } = self.index.tables[table];
        if event == RuleEvent::Select
            && matches!(kind, TableKind::View | TableKind::MaterializedView)
        {
            let error = SemaError::DropReturnRule { name, kind, table };
            self.push_error(rule_ref.syntax(), error);
            return;
        }
        self.drop_rule(rule);
    }

    fn lower_alter_rule(&mut self, alter_rule: &ast::AlterRule) {
        let table = alter_rule
            .on_relation()
            .and_then(|on_relation| self.lower_on_relation(&on_relation, IfMissing::Error));
        let rule = alter_rule
            .rule_ref()
            .and_then(|rule_ref| self.lower_rule_ref(table, &rule_ref, IfMissing::Error));
        if let Some(rule) = rule
            && let Some(name) = alter_rule.rule_rename_to().and_then(|rename| rename.rule())
        {
            let Rule { table, event, .. } = self.index.rules[rule];
            if event == RuleEvent::Select {
                self.push_error(name.syntax(), SemaError::RenameSelectRule);
                return;
            }
            let new_name = self.intern(&name.text());
            if self.find_rule(table, new_name).is_some() {
                self.report_duplicate_rule(name.syntax(), new_name, table);
                return;
            }
            let moved = self.index.rules.next_idx();
            self.drop_rule(rule);
            self.alloc_rule(IfExists::Error, alter_rule.syntax(), &name, table, event);
            self.record_moves(
                Object::Rule(rule),
                self.index.rules.range_from(moved),
                Object::Rule,
            );
        }
    }

    fn find_rule(&self, table: TableId, name: Name) -> Option<RuleId> {
        self.index
            .rules
            .get(table)
            .find(|id| self.index.rules[*id].name == name && !self.dropped_rules.contains(id))
    }

    fn lower_alter_index(&mut self, alter_index: &ast::AlterIndex) {
        if let Some(all_in_tablespace) = alter_index.all_in_tablespace() {
            self.lower_all_in_tablespace(&all_in_tablespace);
            return;
        }
        let index = alter_index
            .index_ref()
            .and_then(|index| index.path_ref())
            .and_then(|path| {
                self.lower_index_ref(
                    &path,
                    IfMissing::from_if_exists(alter_index.if_exists().as_ref()),
                )
            });
        match alter_index.action() {
            Some(ast::AlterIndexAction::AttachIndexPartition(attach)) => {
                if let Some(path) = attach.index_ref().and_then(|index| index.path_ref()) {
                    self.lower_index_ref(&path, IfMissing::Error);
                }
            }
            Some(ast::AlterIndexAction::SetTablespace(set_tablespace)) => {
                self.lower_set_tablespace(&set_tablespace);
            }
            Some(ast::AlterIndexAction::DependsOnExtension(depends)) => {
                if let Some(extension) = depends.extension_ref() {
                    self.lower_extension_ref(&extension, IfMissing::Error);
                }
            }
            Some(ast::AlterIndexAction::NoDependsOnExtension(depends)) => {
                if let Some(extension) = depends.extension_ref() {
                    self.lower_extension_ref(&extension, IfMissing::Error);
                }
            }
            Some(ast::AlterIndexAction::IndexRenameTo(rename)) => {
                if let Some(index) = index
                    && let Some(path) = rename.index().and_then(|index| index.path())
                {
                    self.rename_index(index, &path, alter_index.syntax());
                }
            }
            Some(
                ast::AlterIndexAction::AlterSetStatistics(_)
                | ast::AlterIndexAction::ResetOptions(_)
                | ast::AlterIndexAction::SetOptions(_),
            )
            | None => {}
        }
    }

    fn lower_drop_index(&mut self, drop_index: &ast::DropIndex) {
        let if_missing = IfMissing::from_if_exists(drop_index.if_exists().as_ref());
        for path in drop_index.index_refs().filter_map(|index| index.path_ref()) {
            if let Some(index) = self.lower_index_ref(&path, if_missing) {
                self.drop_index(index);
            }
        }
    }

    fn lower_reindex(&mut self, reindex: &ast::Reindex) {
        for option in reindex
            .reindex_option_list()
            .into_iter()
            .flat_map(|list| list.reindex_options())
        {
            match option {
                ast::ReindexOption::ReindexOptionTablespace(option) => {
                    if let Some(tablespace) = option.tablespace_ref() {
                        self.lower_tablespace_ref(&tablespace, IfMissing::Error);
                    }
                }
                ast::ReindexOption::ReindexOptionConcurrently(_)
                | ast::ReindexOption::ReindexOptionVerbose(_) => {}
            }
        }
        match reindex.reindex_target() {
            Some(ast::ReindexTarget::ReindexTargetIndex(target)) => {
                if let Some(path) = target.index_ref().and_then(|index| index.path_ref()) {
                    self.lower_index_ref(&path, IfMissing::Error);
                }
            }
            Some(ast::ReindexTarget::ReindexTargetTable(target)) => {
                if let Some(path) = target.table_name_ref().and_then(|table| table.path_ref()) {
                    self.lower_table_ref(&path, IfMissing::Error, TableLookup::Table);
                }
            }
            Some(ast::ReindexTarget::ReindexTargetDatabase(target)) => {
                if let Some(database) = target.database_ref() {
                    self.lower_database_ref(&database, IfMissing::Error);
                }
            }
            Some(ast::ReindexTarget::ReindexTargetSystem(target)) => {
                if let Some(database) = target.database_ref() {
                    self.lower_database_ref(&database, IfMissing::Error);
                }
            }
            Some(ast::ReindexTarget::ReindexTargetSchema(target)) => {
                if let Some(schema_ref) = target.schema_ref() {
                    self.lower_schema_ref_id(&schema_ref, IfMissing::Error);
                }
            }
            None => {}
        }
    }

    fn lower_alter_large_object(&mut self, alter_large_object: &ast::AlterLargeObject) {
        if let Some(role_ref) = alter_large_object.role_ref() {
            self.lower_role_ref(&role_ref, IfMissing::Error);
        }
    }

    fn lower_alter_default_privileges(
        &mut self,
        alter_default_privileges: &ast::AlterDefaultPrivileges,
    ) {
        for role_ref in alter_default_privileges
            .for_roles()
            .and_then(|for_roles| for_roles.role_ref_list())
            .iter()
            .flat_map(|list| list.role_refs())
        {
            self.lower_role_ref(&role_ref, IfMissing::Error);
        }
        for schema_ref in alter_default_privileges
            .in_schemas()
            .iter()
            .flat_map(|in_schemas| in_schemas.schema_refs())
        {
            self.lower_schema_ref_id(&schema_ref, IfMissing::Error);
        }
        let grantees = match alter_default_privileges.action() {
            Some(ast::AlterDefaultPrivilegesAction::GrantDefaultPrivileges(grant)) => {
                grant.role_ref_list()
            }
            Some(ast::AlterDefaultPrivilegesAction::RevokeDefaultPrivileges(revoke)) => {
                revoke.role_ref_list()
            }
            None => None,
        };
        for role_ref in grantees.iter().flat_map(|list| list.role_refs()) {
            self.lower_grantee_role_ref(&role_ref);
        }
    }

    fn lower_grant(&mut self, grant: &ast::Grant) {
        self.lower_privileges(
            grant.privileges(),
            grant.on_privilege_objects_clause(),
            grant.role_ref_list(),
            grant.granted_by_clause(),
        );
    }

    fn lower_revoke(&mut self, revoke: &ast::Revoke) {
        self.lower_privileges(
            revoke.privileges(),
            revoke.on_privilege_objects_clause(),
            revoke.role_ref_list(),
            revoke.granted_by_clause(),
        );
    }

    fn lower_privileges(
        &mut self,
        privileges: Option<ast::Privileges>,
        on_clause: Option<ast::OnPrivilegeObjectsClause>,
        grantees: Option<ast::RoleRefList>,
        granted_by: Option<ast::GrantedByClause>,
    ) {
        match on_clause.and_then(|clause| clause.privilege_objects()) {
            Some(objects) => {
                let table = self.lower_privilege_objects(&objects);
                match privileges {
                    Some(ast::Privileges::AllPrivileges(all)) => {
                        if let Some(list) = all.column_ref_list() {
                            self.lower_privilege_columns(table, &list);
                        }
                    }
                    Some(ast::Privileges::RevokeCommandList(commands)) => {
                        for list in commands
                            .revoke_commands()
                            .filter_map(|command| command.column_ref_list())
                        {
                            self.lower_privilege_columns(table, &list);
                        }
                    }
                    None => {}
                }
            }
            None => {
                if let Some(ast::Privileges::RevokeCommandList(commands)) = privileges {
                    for role_ref in commands
                        .revoke_commands()
                        .filter_map(|command| command.role_ref())
                    {
                        self.lower_role_ref(&role_ref, IfMissing::Error);
                    }
                }
            }
        }
        for role_ref in grantees.into_iter().flat_map(|list| list.role_refs()) {
            self.lower_grantee_role_ref(&role_ref);
        }
        if let Some(role_ref) = granted_by.and_then(|clause| clause.role_ref()) {
            self.lower_role_ref(&role_ref, IfMissing::Error);
        }
    }

    fn lower_privilege_columns(&mut self, table: Option<TableId>, list: &ast::ColumnRefList) {
        for column_ref in list.column_name_refs() {
            self.lower_constraint_column_ref(table, &column_ref);
        }
    }

    fn lower_privilege_objects(&mut self, objects: &ast::PrivilegeObjects) -> Option<TableId> {
        let if_missing = IfMissing::Error;
        match objects {
            ast::PrivilegeObjects::PrivilegeAllFunctionsInSchema(objects) => {
                for schema_ref in objects.schema_refs() {
                    self.lower_schema_ref_id(&schema_ref, if_missing);
                }
            }
            ast::PrivilegeObjects::PrivilegeAllProceduresInSchema(objects) => {
                for schema_ref in objects.schema_refs() {
                    self.lower_schema_ref_id(&schema_ref, if_missing);
                }
            }
            ast::PrivilegeObjects::PrivilegeAllRoutinesInSchema(objects) => {
                for schema_ref in objects.schema_refs() {
                    self.lower_schema_ref_id(&schema_ref, if_missing);
                }
            }
            ast::PrivilegeObjects::PrivilegeAllSequencesInSchema(objects) => {
                for schema_ref in objects.schema_refs() {
                    self.lower_schema_ref_id(&schema_ref, if_missing);
                }
            }
            ast::PrivilegeObjects::PrivilegeAllTablesInSchema(objects) => {
                for schema_ref in objects.schema_refs() {
                    self.lower_schema_ref_id(&schema_ref, if_missing);
                }
            }
            ast::PrivilegeObjects::PrivilegeDatabase(objects) => {
                for database in objects.database_refs() {
                    self.lower_database_ref(&database, if_missing);
                }
            }
            ast::PrivilegeObjects::PrivilegeDefault(objects) => {
                return self.lower_privilege_relations(objects.relation_name_refs());
            }
            ast::PrivilegeObjects::PrivilegeTable(objects) => {
                return self.lower_privilege_relations(objects.relation_name_refs());
            }
            ast::PrivilegeObjects::PrivilegeDomain(objects) => {
                for path in objects.domain_refs().filter_map(|name| name.path_ref()) {
                    self.lower_type_path_ref(&path, if_missing, Some(TypeKind::Domain));
                }
            }
            ast::PrivilegeObjects::PrivilegeType(objects) => {
                for path in objects.type_name_refs().filter_map(|name| name.path_ref()) {
                    self.lower_type_path_ref(&path, if_missing, None);
                }
            }
            ast::PrivilegeObjects::PrivilegeForeignDataWrapper(objects) => {
                for wrapper_ref in objects.foreign_data_wrapper_refs() {
                    self.lower_foreign_data_wrapper_ref(&wrapper_ref, if_missing);
                }
            }
            ast::PrivilegeObjects::PrivilegeForeignServer(objects) => {
                for server_ref in objects.server_refs() {
                    self.lower_server_ref(&server_ref, if_missing);
                }
            }
            ast::PrivilegeObjects::PrivilegeFunction(objects) => {
                for sig in objects
                    .function_sig_list()
                    .iter()
                    .flat_map(|list| list.function_sigs())
                {
                    if let Some(path) = sig.function_name_ref().and_then(|name| name.path_ref()) {
                        self.lower_routine_sig(
                            &path,
                            sig.param_list(),
                            RoutineKind::Function,
                            if_missing,
                        );
                    }
                }
            }
            ast::PrivilegeObjects::PrivilegeProcedure(objects) => {
                for sig in objects
                    .procedure_sig_list()
                    .iter()
                    .flat_map(|list| list.procedure_sigs())
                {
                    if let Some(path) = sig.procedure_name_ref().and_then(|name| name.path_ref()) {
                        self.lower_routine_sig(
                            &path,
                            sig.param_list(),
                            RoutineKind::Procedure,
                            if_missing,
                        );
                    }
                }
            }
            ast::PrivilegeObjects::PrivilegeRoutine(objects) => {
                for sig in objects
                    .routine_sig_list()
                    .iter()
                    .flat_map(|list| list.routine_sigs())
                {
                    if let Some(path) = sig.routine_name_ref().and_then(|name| name.path_ref()) {
                        self.lower_routine_sig(
                            &path,
                            sig.param_list(),
                            RoutineKind::Routine,
                            if_missing,
                        );
                    }
                }
            }
            ast::PrivilegeObjects::PrivilegeLanguage(objects) => {
                for language_ref in objects.language_refs() {
                    self.lower_language_ref(&language_ref, if_missing);
                }
            }
            ast::PrivilegeObjects::PrivilegeLargeObject(_)
            | ast::PrivilegeObjects::PrivilegeParameter(_) => {}
            ast::PrivilegeObjects::PrivilegePropertyGraph(objects) => {
                for path in objects
                    .property_graph_refs()
                    .filter_map(|name| name.path_ref())
                {
                    self.lower_table_ref(&path, if_missing, TableLookup::PropertyGraph);
                }
            }
            ast::PrivilegeObjects::PrivilegeSchema(objects) => {
                for schema_ref in objects.schema_refs() {
                    self.lower_schema_ref_id(&schema_ref, if_missing);
                }
            }
            ast::PrivilegeObjects::PrivilegeSequence(objects) => {
                for path in objects.sequence_refs().filter_map(|name| name.path_ref()) {
                    self.lower_table_ref(&path, if_missing, TableLookup::Sequence);
                }
            }
            ast::PrivilegeObjects::PrivilegeTablespace(objects) => {
                for tablespace in objects.tablespace_refs() {
                    self.lower_tablespace_ref(&tablespace, if_missing);
                }
            }
        }
        None
    }

    fn lower_privilege_relations(
        &mut self,
        relations: ast::AstChildren<ast::RelationNameRef>,
    ) -> Option<TableId> {
        let mut first = None;
        for (i, relation) in relations.enumerate() {
            let table = self.lower_relation_name_ref(&relation, IfMissing::Error);
            if i == 0 {
                first = table;
            }
        }
        first
    }

    fn lower_security_label(&mut self, security_label: &ast::SecurityLabel) {
        let Some(object) = security_label.security_label_object() else {
            return;
        };
        self.lower_object_ref(&object.into());
    }

    fn lower_comment_on(&mut self, comment_on: &ast::CommentOn) {
        if let Some(object) = comment_on.comment_object() {
            self.lower_object_ref(&object);
        }
    }

    fn lower_object_ref(&mut self, object: &ast::CommentObject) -> Option<Object> {
        let if_missing = IfMissing::Error;
        match object {
            ast::CommentObject::ObjectAccessMethod(object) => {
                let access_method_ref = object.access_method_ref()?;
                self.lower_access_method_ref(&access_method_ref, None, if_missing)
                    .map(Object::AccessMethod)
            }
            ast::CommentObject::ObjectAggregate(object) => {
                let aggregate = object.aggregate()?;
                let path = aggregate.path_ref()?;
                self.lower_routine_sig(
                    &path,
                    aggregate.param_list(),
                    RoutineKind::Aggregate,
                    if_missing,
                )
                .map(Object::Function)
            }
            ast::CommentObject::ObjectCast(object) => {
                let cast_sig = object.cast_sig()?;
                let types = self.lower_cast_sig(&cast_sig, if_missing)?;
                self.find_cast(types).map(Object::Cast)
            }
            ast::CommentObject::ObjectCollation(object) => {
                let path = object.collation_ref()?.path_ref()?;
                self.lower_collation_ref(&path, if_missing)
                    .map(Object::Collation)
            }
            ast::CommentObject::ObjectColumn(object) => {
                if let Some(path) = object.name().and_then(|name| name.path_ref()) {
                    self.lower_qualified_column_ref(&path, TableLookup::Any);
                }
                None
            }
            ast::CommentObject::ObjectConstraint(object) => {
                let owner = if let Some(path) = object.domain_ref().and_then(|name| name.path_ref())
                {
                    self.lower_type_path_ref(&path, if_missing, Some(TypeKind::Domain))
                        .map(ConstraintOwner::Domain)
                } else if let Some(path) = object.table_name_ref().and_then(|name| name.path_ref())
                {
                    self.lower_table_ref(&path, if_missing, TableLookup::TableOrForeignTable)
                        .map(ConstraintOwner::Table)
                } else {
                    None
                };
                let name_ref = object.constraint_name_ref()?;
                self.lower_constraint_ref(owner, &name_ref, if_missing)
                    .map(Object::Constraint)
            }
            ast::CommentObject::ObjectConversion(object) => {
                let path = object.conversion_ref()?.path_ref()?;
                self.lower_conversion_ref(&path, if_missing)
                    .map(Object::Conversion)
            }
            ast::CommentObject::ObjectDatabase(object) => {
                if let Some(database) = object.database_ref() {
                    self.lower_database_ref(&database, if_missing);
                }
                None
            }
            ast::CommentObject::ObjectDomain(object) => {
                let path = object.domain_ref()?.path_ref()?;
                self.lower_type_path_ref(&path, if_missing, Some(TypeKind::Domain))
                    .map(Object::Type)
            }
            ast::CommentObject::ObjectEventTrigger(object) => {
                let event_trigger_ref = object.event_trigger_ref()?;
                self.lower_event_trigger_ref(&event_trigger_ref, if_missing)
                    .map(Object::EventTrigger)
            }
            ast::CommentObject::ObjectExtension(object) => {
                let extension = object.extension_ref()?;
                self.lower_extension_ref(&extension, if_missing)
                    .map(Object::Extension)
            }
            ast::CommentObject::ObjectForeignDataWrapper(object) => {
                let wrapper_ref = object.foreign_data_wrapper_ref()?;
                self.lower_foreign_data_wrapper_ref(&wrapper_ref, if_missing)
                    .map(Object::ForeignDataWrapper)
            }
            ast::CommentObject::ObjectForeignTable(object) => {
                let path = object.table_name_ref()?.path_ref()?;
                self.lower_table_ref(&path, if_missing, TableLookup::ForeignTable)
                    .map(Object::Table)
            }
            ast::CommentObject::ObjectFunction(object) => {
                let sig = object.function_sig()?;
                let path = sig.function_name_ref()?.path_ref()?;
                self.lower_routine_sig(&path, sig.param_list(), RoutineKind::Function, if_missing)
                    .map(Object::Function)
            }
            ast::CommentObject::ObjectIndex(object) => {
                let path = object.index_ref()?.path_ref()?;
                self.lower_index_ref(&path, if_missing).map(Object::Index)
            }
            ast::CommentObject::ObjectLanguage(object) => {
                let language_ref = object.language_ref()?;
                self.lower_language_ref(&language_ref, if_missing)
                    .map(Object::Language)
            }
            ast::CommentObject::ObjectLargeObject(_) => None,
            ast::CommentObject::ObjectMaterializedView(object) => {
                let path = object.view_ref()?.path_ref()?;
                self.lower_table_ref(&path, if_missing, TableLookup::MaterializedView)
                    .map(Object::Table)
            }
            ast::CommentObject::ObjectOperator(object) => {
                let left = self.lower_operand(object.lhs(), if_missing);
                let right = self.lower_operand(object.rhs(), if_missing);
                let op = object.op()?;
                self.lower_operator_ref(&op, left, right, if_missing)
                    .map(Object::Operator)
            }
            ast::CommentObject::ObjectOperatorClass(object) => {
                let access_method = object.using_method().and_then(|using_method| {
                    self.lower_using_method(&using_method, AccessMethodKind::Index)
                });
                let path = object.op_class_ref()?.path_ref()?;
                self.lower_operator_class_ref(&path, access_method, if_missing)
                    .map(Object::OperatorClass)
            }
            ast::CommentObject::ObjectOperatorFamily(object) => {
                let access_method = object.using_method().and_then(|using_method| {
                    self.lower_using_method(&using_method, AccessMethodKind::Index)
                });
                let path = object.op_family_ref()?.path_ref()?;
                self.lower_operator_family_ref(&path, access_method, if_missing)
                    .map(Object::OperatorFamily)
            }
            ast::CommentObject::ObjectPolicy(object) => {
                let table = object
                    .table_name_ref()
                    .and_then(|name| name.path_ref())
                    .and_then(|path| self.lower_table_ref(&path, if_missing, TableLookup::Table));
                let policy_ref = object.policy_ref()?;
                self.lower_policy_ref(table, &policy_ref, if_missing)
                    .map(Object::Policy)
            }
            ast::CommentObject::ObjectProcedure(object) => {
                let sig = object.procedure_sig()?;
                let path = sig.procedure_name_ref()?.path_ref()?;
                self.lower_routine_sig(&path, sig.param_list(), RoutineKind::Procedure, if_missing)
                    .map(Object::Function)
            }
            ast::CommentObject::ObjectPropertyGraph(object) => {
                let path = object.property_graph_ref()?.path_ref()?;
                self.lower_table_ref(&path, if_missing, TableLookup::PropertyGraph)
                    .map(Object::Table)
            }
            ast::CommentObject::ObjectPublication(object) => {
                let publication_ref = object.publication_ref()?;
                self.lower_publication_ref(&publication_ref, if_missing)
                    .map(Object::Publication)
            }
            ast::CommentObject::ObjectRole(object) => {
                if let Some(role_ref) = object.role_ref() {
                    self.lower_role_ref(&role_ref, if_missing);
                }
                None
            }
            ast::CommentObject::ObjectRoutine(object) => {
                let sig = object.routine_sig()?;
                let path = sig.routine_name_ref()?.path_ref()?;
                self.lower_routine_sig(&path, sig.param_list(), RoutineKind::Routine, if_missing)
                    .map(Object::Function)
            }
            ast::CommentObject::ObjectRule(object) => {
                let table = object
                    .relation_name_ref()
                    .and_then(|name| self.lower_relation_name_ref(&name, if_missing));
                let rule_ref = object.rule_ref()?;
                self.lower_rule_ref(table, &rule_ref, if_missing)
                    .map(Object::Rule)
            }
            ast::CommentObject::ObjectSchema(object) => {
                let schema_ref = object.schema_ref()?;
                self.lower_schema_ref_id(&schema_ref, if_missing)
                    .map(Object::Schema)
            }
            ast::CommentObject::ObjectSequence(object) => {
                let path = object.sequence_ref()?.path_ref()?;
                self.lower_table_ref(&path, if_missing, TableLookup::Sequence)
                    .map(Object::Table)
            }
            ast::CommentObject::ObjectServer(object) => {
                let server_ref = object.server_ref()?;
                self.lower_server_ref(&server_ref, if_missing)
                    .map(Object::Server)
            }
            ast::CommentObject::ObjectStatistics(object) => {
                let path = object.statistics_ref()?.path_ref()?;
                self.lower_statistics_ref(&path, if_missing)
                    .map(Object::Statistics)
            }
            ast::CommentObject::ObjectSubscription(object) => {
                if let Some(subscription_ref) = object.subscription_ref() {
                    self.lower_subscription_ref(&subscription_ref, if_missing);
                }
                None
            }
            ast::CommentObject::ObjectTable(object) => {
                let path = object.table_name_ref()?.path_ref()?;
                self.lower_table_ref(&path, if_missing, TableLookup::Table)
                    .map(Object::Table)
            }
            ast::CommentObject::ObjectTablespace(object) => {
                if let Some(tablespace) = object.tablespace_ref() {
                    self.lower_tablespace_ref(&tablespace, if_missing);
                }
                None
            }
            ast::CommentObject::ObjectTextSearchConfig(object) => {
                if let Some(path) = object
                    .text_search_configuration_ref()
                    .and_then(|name| name.path_ref())
                {
                    return self
                        .lower_text_search_ref(TextSearchKind::Configuration, &path, if_missing)
                        .map(Object::TextSearch);
                }
                None
            }
            ast::CommentObject::ObjectTextSearchDict(object) => {
                if let Some(path) = object
                    .text_search_dictionary_ref()
                    .and_then(|name| name.path_ref())
                {
                    return self
                        .lower_text_search_ref(TextSearchKind::Dictionary, &path, if_missing)
                        .map(Object::TextSearch);
                }
                None
            }
            ast::CommentObject::ObjectTextSearchParser(object) => {
                if let Some(path) = object
                    .text_search_parser_ref()
                    .and_then(|name| name.path_ref())
                {
                    return self
                        .lower_text_search_ref(TextSearchKind::Parser, &path, if_missing)
                        .map(Object::TextSearch);
                }
                None
            }
            ast::CommentObject::ObjectTextSearchTemplate(object) => {
                if let Some(path) = object
                    .text_search_template_ref()
                    .and_then(|name| name.path_ref())
                {
                    return self
                        .lower_text_search_ref(TextSearchKind::Template, &path, if_missing)
                        .map(Object::TextSearch);
                }
                None
            }
            ast::CommentObject::ObjectTransform(object) => self
                .lower_transform_ref(
                    object.syntax(),
                    object.ty().as_ref(),
                    object.language_ref().as_ref(),
                    if_missing,
                )
                .map(Object::Transform),
            ast::CommentObject::ObjectTrigger(object) => {
                let table = object
                    .relation_name_ref()
                    .and_then(|name| self.lower_relation_name_ref(&name, if_missing));
                let trigger_ref = object.trigger_ref()?;
                self.lower_trigger_ref(table, &trigger_ref, if_missing)
                    .map(Object::Trigger)
            }
            ast::CommentObject::ObjectType(object) => {
                let path = object.type_name_ref()?.path_ref()?;
                self.lower_type_path_ref(&path, if_missing, None)
                    .map(Object::Type)
            }
            ast::CommentObject::ObjectView(object) => {
                let path = object.view_ref()?.path_ref()?;
                self.lower_table_ref(&path, if_missing, TableLookup::View)
                    .map(Object::Table)
            }
        }
    }

    fn lower_copy(&mut self, copy: &ast::Copy) {
        if let Some(query) = copy.copy_query().and_then(|query| query.preparable_stmt()) {
            self.lower_preparable_stmt(&query);
        }
        let Some(copy_table) = copy.copy_table() else {
            return;
        };
        let lookup = match copy.copy_direction() {
            Some(ast::CopyDirection::CopyTo(_)) => TableLookup::CopyTo,
            Some(ast::CopyDirection::CopyFrom(_)) | None => TableLookup::CopyFrom,
        };
        let table = copy_table
            .table_name_ref()
            .and_then(|name| name.path_ref())
            .and_then(|path| self.lower_table_ref(&path, IfMissing::Error, lookup));
        for column_ref in copy_table
            .column_ref_list()
            .iter()
            .flat_map(|list| list.column_name_refs())
        {
            self.lower_constraint_column_ref(table, &column_ref);
        }
        for column_ref in copy
            .copy_legacy_options()
            .flat_map(|option| option.column_name_refs())
        {
            self.lower_constraint_column_ref(table, &column_ref);
        }
        for option in copy
            .copy_option_list()
            .iter()
            .flat_map(|list| list.copy_options())
        {
            let Some(key) = option.copy_option_key() else {
                continue;
            };
            if !matches!(
                key.text().as_str(),
                "force_quote" | "force_not_null" | "force_null"
            ) {
                continue;
            }
            for name in option
                .copy_option_value()
                .and_then(|value| value.copy_option_arg_list())
                .iter()
                .flat_map(|list| list.copy_option_args())
                .filter_map(|arg| arg.copy_option_value_name())
            {
                self.lower_table_column_name(table, &name);
            }
        }
        if let Some(expr) = copy.where_clause().and_then(|clause| clause.expr()) {
            self.with_table_scope(QueryKind::Copy, table, copy.syntax(), |this| {
                this.lower_expr_in(ExprContext::CopyWhere, &expr);
            });
        }
    }

    fn lower_truncate(&mut self, truncate: &ast::Truncate) {
        for path in truncate
            .table_list()
            .iter()
            .flat_map(|list| list.table_relation_names())
            .filter_map(|name| name.table_name_ref()?.path_ref())
        {
            self.lower_table_ref(&path, IfMissing::Error, TableLookup::Truncate);
        }
    }

    fn lower_lock(&mut self, lock: &ast::Lock) {
        for path in lock
            .relation_list()
            .iter()
            .flat_map(|list| list.relation_names())
            .filter_map(|name| name.relation_name_ref()?.path_ref())
        {
            self.lower_table_ref(&path, IfMissing::Error, TableLookup::TableOrView);
        }
    }

    fn lower_analyze(&mut self, analyze: &ast::Analyze) {
        if let Some(list) = analyze.table_and_columns_list() {
            self.lower_table_and_columns_list(&list, TableLookup::Any);
        }
    }

    fn lower_vacuum(&mut self, vacuum: &ast::Vacuum) {
        if let Some(list) = vacuum.table_and_columns_list() {
            self.lower_table_and_columns_list(&list, TableLookup::Any);
        }
    }

    fn lower_repack(&mut self, repack: &ast::Repack) {
        if let Some(list) = repack.table_and_columns_list() {
            self.lower_table_and_columns_list(&list, TableLookup::TableOrMaterializedView);
        }
        if let Some(path) = repack
            .using_index()
            .and_then(|using| using.index_ref())
            .and_then(|index| index.path_ref())
        {
            self.lower_index_ref(&path, IfMissing::Error);
        }
    }

    fn lower_table_and_columns_list(
        &mut self,
        list: &ast::TableAndColumnsList,
        lookup: TableLookup,
    ) {
        for table_and_columns in list.table_and_columnss() {
            let table = table_and_columns
                .table_relation_name()
                .and_then(|name| name.table_name_ref()?.path_ref())
                .and_then(|path| self.lower_table_ref(&path, IfMissing::Error, lookup));
            for column_ref in table_and_columns
                .column_ref_list()
                .iter()
                .flat_map(|list| list.column_name_refs())
            {
                self.lower_constraint_column_ref(table, &column_ref);
            }
        }
    }

    fn lower_cluster(&mut self, cluster: &ast::Cluster) {
        if let Some(legacy) = cluster.cluster_legacy() {
            if let Some(path) = legacy.index_ref().and_then(|index| index.path_ref()) {
                self.lower_index_ref(&path, IfMissing::Error);
            }
            if let Some(path) = legacy
                .on_path()
                .and_then(|on_path| on_path.table_name_ref())
                .and_then(|table| table.path_ref())
            {
                self.lower_table_ref(&path, IfMissing::Error, TableLookup::Table);
            }
        }
        if let Some(path) = cluster.table_name_ref().and_then(|table| table.path_ref()) {
            self.lower_table_ref(&path, IfMissing::Error, TableLookup::Table);
        }
        if let Some(path) = cluster
            .cluster_using_index()
            .and_then(|using| using.index_ref())
            .and_then(|index| index.path_ref())
        {
            self.lower_index_ref(&path, IfMissing::Error);
        }
    }

    fn lower_index_ref(&mut self, path: &ast::PathRef, if_missing: IfMissing) -> Option<IndexId> {
        let segment = path.segment()?;
        let name = self.intern(&segment.text());
        let schema = self.lower_schema_qualifier(path.qualifier());
        let index = self
            .search_path(schema, Lookup::Relation)
            .find_map(|schema| self.find_index_in(schema, name));
        let resolution = match index {
            Some(index) => Resolution::Index(index),
            None => match if_missing {
                IfMissing::Ignore => Resolution::Unknown,
                IfMissing::Error => Resolution::Unresolved,
            },
        };
        self.add_ref(name, ReferenceKind::Index, segment.syntax(), resolution);
        index
    }

    fn lower_create_collation(&mut self, create_collation: &ast::CreateCollation) {
        match create_collation.definition() {
            Some(ast::CreateCollationDefinition::CollationFrom(from)) => {
                if let Some(path) = from
                    .collation_ref()
                    .and_then(|collation| collation.path_ref())
                {
                    self.lower_collation_ref(&path, IfMissing::Error);
                }
            }
            Some(ast::CreateCollationDefinition::AttributeList(_)) | None => {}
        }
        if let Some(path) = create_collation
            .collation()
            .and_then(|collation| collation.path())
        {
            self.alloc_collation(
                IfExists::from_if_not_exists(create_collation.if_not_exists().as_ref()),
                create_collation.syntax(),
                &path,
            );
        }
    }

    fn move_collation(
        &mut self,
        collation: CollationId,
        node: &SyntaxNode,
        schema: Name,
        name: Name,
        name_ptr: InIrFile<AstPtr<ast::PathSegment>>,
    ) -> CollationId {
        let moved = self.index.collations.next_idx();
        self.drop_collation(collation);
        let id = self.index.collations.alloc(Collation {
            schema,
            name,
            ptr: self.ptr(node),
            name_ptr,
        });
        self.record_moves(
            Object::Collation(collation),
            self.index.collations.range_from(moved),
            Object::Collation,
        );
        id
    }

    fn lower_alter_collation(&mut self, alter_collation: &ast::AlterCollation) {
        let collation = alter_collation
            .collation_ref()
            .and_then(|collation| collation.path_ref())
            .and_then(|path| self.lower_collation_ref(&path, IfMissing::Error));
        let node = alter_collation.syntax();
        match alter_collation.action() {
            Some(ast::AlterCollationAction::OwnerTo(owner_to)) => self.lower_owner_to(&owner_to),
            Some(ast::AlterCollationAction::CollationRenameTo(rename)) => {
                if let Some(collation) = collation
                    && let Some(segment) = rename
                        .collation()
                        .and_then(|collation| collation.path())
                        .and_then(|path| path.segment())
                {
                    let schema = self.index.collations[collation].schema;
                    let name = self.intern(&segment.text());
                    if self.find_collation_in(schema, name).is_some() {
                        self.report_duplicate(ObjectKind::Collation, segment.syntax(), name);
                        return;
                    }
                    let name_ptr = self.ast_ptr(&segment);
                    let id = self.move_collation(collation, node, schema, name, name_ptr);
                    self.add_item(segment.syntax(), Item::Collation(id));
                }
            }
            Some(ast::AlterCollationAction::SetSchema(set_schema)) => {
                let Some(schema_ref) = set_schema.schema_ref() else {
                    return;
                };
                let Some(schema) = self.lower_set_schema_ref(&schema_ref) else {
                    return;
                };
                if let Some(collation) = collation {
                    let Collation { name, name_ptr, .. } = self.index.collations[collation];
                    if self.find_collation_in(schema, name).is_some() {
                        let error = SemaError::DuplicateInSchema {
                            object: ObjectKind::Collation,
                            name,
                            schema,
                        };
                        self.push_error(schema_ref.syntax(), error);
                        return;
                    }
                    self.move_collation(collation, node, schema, name, name_ptr);
                }
            }
            Some(ast::AlterCollationAction::RefreshVersion(_)) | None => {}
        }
    }

    fn lower_drop_collation(&mut self, drop_collation: &ast::DropCollation) {
        let if_missing = IfMissing::from_if_exists(drop_collation.if_exists().as_ref());
        for path in drop_collation
            .collation_refs()
            .filter_map(|collation| collation.path_ref())
        {
            if let Some(collation) = self.lower_collation_ref(&path, if_missing) {
                self.drop_collation(collation);
            }
        }
    }

    fn lower_collate(&mut self, collate: &ast::Collate) {
        if let Some(path) = collate
            .collation_ref()
            .and_then(|collation| collation.path_ref())
        {
            self.lower_collation_ref(&path, IfMissing::Error);
        }
    }

    fn lower_collation_ref(
        &mut self,
        path: &ast::PathRef,
        if_missing: IfMissing,
    ) -> Option<CollationId> {
        let segment = path.segment()?;
        let name = self.intern(&segment.text());
        let schema = self.lower_schema_qualifier(path.qualifier());
        let collation = self
            .search_path(schema, Lookup::Collation)
            .find_map(|schema| self.find_collation_in(schema, name));
        let resolution = match collation {
            Some(collation) => Resolution::Collation(collation),
            None => match if_missing {
                IfMissing::Ignore => Resolution::Unknown,
                IfMissing::Error => Resolution::Unresolved,
            },
        };
        self.add_ref(name, ReferenceKind::Collation, segment.syntax(), resolution);
        collation
    }

    fn find_collation_in(&self, schema: Name, name: Name) -> Option<CollationId> {
        self.index.collations.named(schema, name).find(|id| {
            let collation = &self.index.collations[*id];
            self.visible(collation.schema, collation.ptr.file_id)
                && !self.dropped_collations.contains(id)
        })
    }

    fn with_table_scope(
        &mut self,
        kind: QueryKind,
        table: Option<TableId>,
        node: &SyntaxNode,
        f: impl FnOnce(&mut Self),
    ) {
        let query = self.alloc_query(kind, node);
        self.push_scope(query);
        let (name, kind, columns) = match table {
            Some(table) => {
                let table_def = &self.index.tables[table];
                (
                    Some(table_def.name),
                    SourceKind::Table(table),
                    table_def.columns,
                )
            }
            None => (None, SourceKind::External, self.unknown_columns()),
        };
        self.push_source(Source {
            name,
            kind,
            ptr: self.ptr(node),
            alias_ptr: None,
            columns,
        });
        self.set_visible((0..1).into());
        f(self);
        self.scopes.pop();
    }

    fn check_column_nullability(
        &mut self,
        table: TableId,
        column: Name,
        constraints: impl Iterator<Item = ast::Constraint>,
    ) {
        let mut seen_nullable = false;
        let mut not_null = false;
        let mut needs_not_null = false;
        for constraint in constraints {
            let conflict = match &constraint {
                ast::Constraint::NullConstraint(_) => {
                    let conflict = seen_nullable && not_null || needs_not_null;
                    seen_nullable = true;
                    not_null = false;
                    conflict
                }
                ast::Constraint::NotNullConstraint(_) => {
                    let conflict = seen_nullable && !not_null;
                    if !not_null {
                        seen_nullable = true;
                        not_null = true;
                        needs_not_null = false;
                    }
                    conflict
                }
                ast::Constraint::PrimaryKeyConstraint(_) => {
                    needs_not_null = true;
                    seen_nullable && !not_null
                }
                ast::Constraint::GeneratedConstraint(generated) => {
                    let identity = matches!(
                        generated.generated_as(),
                        Some(ast::GeneratedAs::GeneratedIdentity(_))
                    );
                    needs_not_null |= identity && !seen_nullable;
                    identity && seen_nullable && !not_null
                }
                ast::Constraint::CheckConstraint(_)
                | ast::Constraint::DefaultConstraint(_)
                | ast::Constraint::ExcludeConstraint(_)
                | ast::Constraint::ForeignKeyConstraint(_)
                | ast::Constraint::ReferencesConstraint(_)
                | ast::Constraint::UniqueConstraint(_) => false,
            };
            if conflict {
                let table = self.index.tables[table].name;
                self.push_error(
                    constraint.syntax(),
                    SemaError::ConflictingNullability { column, table },
                );
                return;
            }
        }
    }

    fn lower_constraint(
        &mut self,
        table: Option<TableId>,
        column: Option<Name>,
        constraint: &ast::Constraint,
    ) {
        match constraint {
            ast::Constraint::CheckConstraint(check) => {
                self.lower_check_constraint(table, check);
            }
            ast::Constraint::DefaultConstraint(default) => {
                self.lower_default_constraint(table, column, default);
            }
            ast::Constraint::ExcludeConstraint(exclude) => {
                self.lower_exclude_constraint(table, exclude);
            }
            ast::Constraint::ForeignKeyConstraint(foreign_key) => {
                self.lower_foreign_key_constraint(table, foreign_key);
            }
            ast::Constraint::GeneratedConstraint(generated) => {
                self.lower_generated_constraint(table, column, generated);
            }
            ast::Constraint::NotNullConstraint(not_null) => {
                self.lower_not_null_constraint(table, column, not_null);
            }
            ast::Constraint::NullConstraint(_) => {}
            ast::Constraint::PrimaryKeyConstraint(primary_key) => {
                self.lower_primary_key_constraint(table, column, primary_key);
            }
            ast::Constraint::ReferencesConstraint(references) => {
                self.lower_references_constraint(table, column, references);
            }
            ast::Constraint::UniqueConstraint(unique) => {
                self.lower_unique_constraint(table, column, unique);
            }
        }
    }

    fn lower_check_constraint(&mut self, table: Option<TableId>, check: &ast::CheckConstraint) {
        let start = self.index.references.next_idx();
        if let Some(expr) = check.expr() {
            self.lower_expr_in(ExprContext::CheckConstraint, &expr);
        }
        let Some(table) = table else {
            return;
        };
        let mut column = None;
        let mut single = true;
        for id in self.index.references.range_from(start) {
            let reference = &self.index.references[id];
            if let Resolution::Column { .. } = reference.resolution {
                match column {
                    None => column = Some(reference.name),
                    Some(name) if name == reference.name => {}
                    Some(_) => single = false,
                }
            }
        }
        let column = column.filter(|_| single);
        let addition = column.map(|column| self.index.names.resolve(column).to_owned());
        let name = match check
            .constraint_name_clause()
            .and_then(|clause| clause.constraint_name())
        {
            Some(name) => ConstraintName::Explicit(name),
            None => ConstraintName::Generated(self.choose_constraint_name(
                ConstraintOwner::Table(table),
                addition.as_deref(),
                "check",
            )),
        };
        let constraint = self.alloc_constraint(
            ConstraintKind::Check,
            ConstraintOwner::Table(table),
            check.syntax(),
            name,
            None,
        );
        self.depend_on_refs(start, IdxRange::single(constraint), Object::Constraint);
    }

    fn lower_default_constraint(
        &mut self,
        table: Option<TableId>,
        column: Option<Name>,
        default: &ast::DefaultConstraint,
    ) {
        if let Some(expr) = default.expr() {
            self.lower_column_expr(table, column, &expr, ColumnExprKind::Default);
        }
    }

    fn lower_generated_constraint(
        &mut self,
        table: Option<TableId>,
        column: Option<Name>,
        generated: &ast::GeneratedConstraint,
    ) {
        match generated.generated_as() {
            Some(ast::GeneratedAs::GeneratedStored(stored)) => {
                if let Some(expr) = stored.expr() {
                    self.lower_column_expr(table, column, &expr, ColumnExprKind::Generated);
                }
            }
            Some(ast::GeneratedAs::GeneratedIdentity(identity)) => {
                self.lower_identity(
                    table,
                    column,
                    identity.sequence_option_list(),
                    identity.syntax(),
                );
            }
            None => {}
        }
    }

    fn lower_not_null_constraint(
        &mut self,
        table: Option<TableId>,
        column: Option<Name>,
        not_null: &ast::NotNullConstraint,
    ) {
        let column = match not_null.column_name_ref() {
            Some(column_ref) => Some(self.lower_constraint_column_ref(table, &column_ref)),
            None => column,
        };
        let (Some(table), Some(column)) = (table, column) else {
            return;
        };
        let name = not_null
            .constraint_name_clause()
            .and_then(|clause| clause.constraint_name());
        self.add_not_null_constraint(table, column, not_null.syntax(), name);
    }

    fn add_not_null_constraint(
        &mut self,
        table: TableId,
        column: Name,
        node: &SyntaxNode,
        name: Option<ast::ConstraintName>,
    ) {
        let kind = ConstraintKind::NotNull { column };
        if self
            .index
            .constraints
            .owned_by(ConstraintOwner::Table(table))
            .any(|id| {
                self.index.constraints[id].kind == kind && !self.dropped_constraints.contains(&id)
            })
        {
            return;
        }
        let name = match name {
            Some(name) => ConstraintName::Explicit(name),
            None => {
                let addition = self.index.names.resolve(column).to_owned();
                ConstraintName::Generated(self.choose_constraint_name(
                    ConstraintOwner::Table(table),
                    Some(&addition),
                    "not_null",
                ))
            }
        };
        self.alloc_constraint(kind, ConstraintOwner::Table(table), node, name, None);
    }

    fn lower_primary_key_constraint(
        &mut self,
        table: Option<TableId>,
        column: Option<Name>,
        primary_key: &ast::PrimaryKeyConstraint,
    ) {
        let IndexColumns { columns, key_len } =
            self.lower_index_parameters(table, column, primary_key.index_parameters());
        let index = primary_key
            .using_index()
            .and_then(|using| self.lower_using_index(&using));
        let Some(table) = table else {
            return;
        };
        if self.has_primary_key(table) {
            let table = self.index.tables[table].name;
            self.push_error(
                primary_key.syntax(),
                SemaError::MultiplePrimaryKeys { table },
            );
        }
        let name = primary_key
            .constraint_name_clause()
            .and_then(|clause| clause.constraint_name());
        self.alloc_index_constraint(
            ConstraintKind::PrimaryKey,
            table,
            primary_key.syntax(),
            name,
            index,
            None,
            "pkey",
        );
        for column in &columns[..key_len] {
            self.add_not_null_constraint(table, *column, primary_key.syntax(), None);
        }
    }

    fn has_primary_key(&self, table: TableId) -> bool {
        self.find_primary_key(table).is_some()
            || self.inheritances.parents(table).iter().any(|inheritance| {
                inheritance.kind == InheritanceKind::Partition
                    && self.has_primary_key(inheritance.parent)
            })
    }

    fn find_primary_key(&self, table: TableId) -> Option<ConstraintId> {
        self.index
            .constraints
            .owned_by(ConstraintOwner::Table(table))
            .find(|id| {
                self.index.constraints[*id].kind == ConstraintKind::PrimaryKey
                    && !self.dropped_constraints.contains(id)
            })
    }

    fn lower_unique_constraint(
        &mut self,
        table: Option<TableId>,
        column: Option<Name>,
        unique: &ast::UniqueConstraint,
    ) {
        let IndexColumns { columns, .. } =
            self.lower_index_parameters(table, column, unique.index_parameters());
        let index = unique
            .using_index()
            .and_then(|using| self.lower_using_index(&using));
        let Some(table) = table else {
            return;
        };
        let name = unique
            .constraint_name_clause()
            .and_then(|clause| clause.constraint_name());
        let addition = postgres::join_names(
            columns
                .iter()
                .map(|column| self.index.names.resolve(*column)),
        );
        self.alloc_index_constraint(
            ConstraintKind::Unique,
            table,
            unique.syntax(),
            name,
            index,
            Some(&addition),
            "key",
        );
    }

    fn lower_exclusion_operator_ref(&mut self, op: &ast::Op, ty: Option<Ty>) {
        let path = op.path_ref();
        let schema = self.lower_schema_qualifier(path.as_ref().and_then(|path| path.qualifier()));
        let range = op.operator_range();
        let text = op.operator_text();
        let name = self.intern(&text);
        let operand = Some(ty.map_or(ArgType::Untyped, ArgType::Known));
        let resolution = self.select_operator(schema, name, operand, operand);
        self.add_ref_at(name, ReferenceKind::Operator, range, resolution);
    }

    fn lower_bin_operator(&mut self, op: &ast::BinOp, left: ArgType, right: ArgType) -> Option<Ty> {
        let left = Some(left);
        let right = Some(right);
        match op {
            ast::BinOp::Caret(token)
            | ast::BinOp::Eq(token)
            | ast::BinOp::Gteq(token)
            | ast::BinOp::LAngle(token)
            | ast::BinOp::Lteq(token)
            | ast::BinOp::Minus(token)
            | ast::BinOp::Neq(token)
            | ast::BinOp::Neqb(token)
            | ast::BinOp::Percent(token)
            | ast::BinOp::Plus(token)
            | ast::BinOp::RAngle(token)
            | ast::BinOp::Slash(token)
            | ast::BinOp::Star(token) => {
                let text = match token.text() {
                    "!=" => "<>",
                    text => text,
                };
                self.add_expr_operator_ref(token.text_range(), None, text, left, right)
            }
            ast::BinOp::CustomOp(custom_op) => self.lower_custom_op(custom_op, left, right),
            ast::BinOp::OperatorCall(operator_call) => {
                self.lower_operator_call(operator_call, left, right)
            }
            ast::BinOp::Like(token) => {
                self.add_expr_operator_ref(token.text_range(), None, "~~", left, right)
            }
            ast::BinOp::Ilike(token) => {
                self.add_expr_operator_ref(token.text_range(), None, "~~*", left, right)
            }
            ast::BinOp::In(token) => {
                self.add_expr_operator_ref(token.text_range(), None, "=", left, right)
            }
            ast::BinOp::NotLike(not_like) => {
                let range = not_like.syntax().text_range();
                self.add_expr_operator_ref(range, None, "!~~", left, right)
            }
            ast::BinOp::NotIlike(not_ilike) => {
                let range = not_ilike.syntax().text_range();
                self.add_expr_operator_ref(range, None, "!~~*", left, right)
            }
            ast::BinOp::SimilarTo(similar_to) => {
                let range = similar_to.syntax().text_range();
                self.add_expr_operator_ref(range, None, "~", left, right)
            }
            ast::BinOp::NotSimilarTo(not_similar_to) => {
                let range = not_similar_to.syntax().text_range();
                self.add_expr_operator_ref(range, None, "!~", left, right)
            }
            ast::BinOp::NotIn(not_in) => {
                let range = not_in.syntax().text_range();
                self.add_expr_operator_ref(range, None, "<>", left, right)
            }
            ast::BinOp::IsDistinctFrom(is_distinct_from) => {
                let range = is_distinct_from.syntax().text_range();
                self.add_expr_operator_ref(range, None, "=", left, right)
            }
            ast::BinOp::IsNotDistinctFrom(is_not_distinct_from) => {
                let range = is_not_distinct_from.syntax().text_range();
                self.add_expr_operator_ref(range, None, "=", left, right)
            }
            ast::BinOp::And(_)
            | ast::BinOp::AtTimeZone(_)
            | ast::BinOp::ColonColon(_)
            | ast::BinOp::ColonEq(_)
            | ast::BinOp::Escape(_)
            | ast::BinOp::FatArrow(_)
            | ast::BinOp::Is(_)
            | ast::BinOp::IsNot(_)
            | ast::BinOp::Or(_)
            | ast::BinOp::Overlaps(_) => None,
        }
    }

    fn lower_bin_rhs(
        &mut self,
        op: Option<&ast::BinOp>,
        lhs: Option<&ast::Expr>,
        rhs: &ast::Expr,
        left: ArgType,
    ) -> ArgType {
        let similar = matches!(
            op,
            Some(ast::BinOp::SimilarTo(_) | ast::BinOp::NotSimilarTo(_))
        );
        let pattern = matches!(
            op,
            Some(
                ast::BinOp::Like(_)
                    | ast::BinOp::Ilike(_)
                    | ast::BinOp::NotLike(_)
                    | ast::BinOp::NotIlike(_)
            )
        );
        if let ast::Expr::BinExpr(escape) = rhs
            && let Some(ast::BinOp::Escape(escape_token)) = escape.op()
            && (similar || pattern)
        {
            let function = if similar {
                "similar_to_escape"
            } else {
                "like_escape"
            };
            let args = [
                self.lower_positional_arg(escape.lhs()),
                self.lower_positional_arg(escape.rhs()),
            ];
            return self
                .lower_system_call(function, &args, escape_token.text_range())
                .map_or(ArgType::Untyped, ArgType::Known);
        }
        if let Some(ast::BinOp::In(_) | ast::BinOp::NotIn(_)) = op
            && let ast::Expr::TupleExpr(tuple) = rhs
        {
            return self.lower_in_list(tuple, left);
        }
        if let ast::Expr::CallExpr(call) = rhs
            && let Some(operand) = self.lower_quantified_operand(call, row_width(lhs).unwrap_or(1))
        {
            return operand;
        }
        if let ast::Expr::ParenExpr(paren) = rhs
            && let Some(query) = paren.select_variant()
            && let Some(width) = sublink_width(op, lhs)
        {
            let query = self.lower_compared_sublink(query, width);
            return expr_arg_type(Some(rhs), self.single_column_type(query));
        }
        let arg = expr_arg_type(Some(rhs), self.lower_expr(rhs));
        if similar {
            return self
                .system_call_type("similar_to_escape", &[CallArg::positional(arg)])
                .map_or(ArgType::Untyped, ArgType::Known);
        }
        arg
    }

    fn check_row_lengths(&mut self, op: &ast::BinOp, lhs: &ast::TupleExpr, rhs: &ast::Expr) {
        match op {
            ast::BinOp::In(_) | ast::BinOp::NotIn(_) => {
                if let ast::Expr::TupleExpr(list) = rhs {
                    for expr in list.exprs() {
                        if let ast::Expr::TupleExpr(row) = expr {
                            self.check_row_length(lhs, &row);
                        }
                    }
                }
            }
            ast::BinOp::Caret(_)
            | ast::BinOp::CustomOp(_)
            | ast::BinOp::Eq(_)
            | ast::BinOp::Gteq(_)
            | ast::BinOp::Ilike(_)
            | ast::BinOp::IsDistinctFrom(_)
            | ast::BinOp::IsNotDistinctFrom(_)
            | ast::BinOp::LAngle(_)
            | ast::BinOp::Like(_)
            | ast::BinOp::Lteq(_)
            | ast::BinOp::Minus(_)
            | ast::BinOp::Neq(_)
            | ast::BinOp::Neqb(_)
            | ast::BinOp::NotIlike(_)
            | ast::BinOp::NotLike(_)
            | ast::BinOp::NotSimilarTo(_)
            | ast::BinOp::OperatorCall(_)
            | ast::BinOp::Percent(_)
            | ast::BinOp::Plus(_)
            | ast::BinOp::RAngle(_)
            | ast::BinOp::SimilarTo(_)
            | ast::BinOp::Slash(_)
            | ast::BinOp::Star(_) => {
                if let ast::Expr::TupleExpr(rhs) = rhs {
                    self.check_row_length(lhs, rhs);
                }
            }
            ast::BinOp::And(_)
            | ast::BinOp::AtTimeZone(_)
            | ast::BinOp::ColonColon(_)
            | ast::BinOp::ColonEq(_)
            | ast::BinOp::Escape(_)
            | ast::BinOp::FatArrow(_)
            | ast::BinOp::Is(_)
            | ast::BinOp::IsNot(_)
            | ast::BinOp::Or(_)
            | ast::BinOp::Overlaps(_) => {}
        }
    }

    fn check_row_length(&mut self, lhs: &ast::TupleExpr, rhs: &ast::TupleExpr) {
        if lhs.exprs().count() != rhs.exprs().count() {
            self.push_error(rhs.syntax(), SemaError::RowLengthMismatch);
        }
    }

    fn lower_in_list(&mut self, tuple: &ast::TupleExpr, left: ArgType) -> ArgType {
        let mut common = match left {
            ArgType::Known(ty) => Some(ty),
            ArgType::UnknownLiteral | ArgType::Untyped => None,
        };
        let mut untyped = left == ArgType::Untyped;
        let mut all_unknown = true;
        for expr in tuple.exprs() {
            let ty = self.lower_expr(&expr);
            match expr_arg_type(Some(&expr), ty) {
                ArgType::Known(ty) => {
                    all_unknown = false;
                    common = match common {
                        Some(common) => self.overloads().select_common_type(common, ty),
                        None if untyped => None,
                        None => Some(ty),
                    };
                    untyped |= common.is_none();
                }
                ArgType::UnknownLiteral => {}
                ArgType::Untyped => {
                    all_unknown = false;
                    untyped = true;
                }
            }
        }
        if untyped {
            return ArgType::Untyped;
        }
        if all_unknown {
            return ArgType::UnknownLiteral;
        }
        common.map_or(ArgType::Untyped, ArgType::Known)
    }

    fn lower_quantified_operand(&mut self, call: &ast::CallExpr, width: usize) -> Option<ArgType> {
        let (expr, query) = if let Some(any_fn) = call.any_fn() {
            (any_fn.expr(), any_fn.select_variant())
        } else if let Some(some_fn) = call.some_fn() {
            (some_fn.expr(), some_fn.select_variant())
        } else {
            let all_fn = call.all_fn()?;
            (all_fn.expr(), all_fn.select_variant())
        };
        if let Some(query) = query {
            let query = self.lower_compared_sublink(query, width);
            return Some(
                self.single_column_type(query)
                    .map_or(ArgType::Untyped, ArgType::Known),
            );
        }
        let Some(expr) = expr else {
            return Some(ArgType::Untyped);
        };
        let ty = self.lower_expr(&expr);
        if is_unknown_literal_expr(&expr) {
            return Some(ArgType::UnknownLiteral);
        }
        Some(
            ty.and_then(|ty| self.index.base_type(ty))
                .and_then(|base| self.index.element_type(base))
                .map_or(ArgType::Untyped, ArgType::Known),
        )
    }

    fn single_column_type(&self, query: QueryId) -> Option<Ty> {
        let columns = self.index.queries[query].columns.range;
        if columns.len() != 1 {
            return None;
        }
        self.index.columns[columns.start()].ty
    }

    fn lower_prefix_operator(&mut self, op: &ast::PrefixOp, operand: ArgType) -> Option<Ty> {
        let right = Some(operand);
        match op {
            ast::PrefixOp::Minus(token) | ast::PrefixOp::Plus(token) => {
                self.add_expr_operator_ref(token.text_range(), None, token.text(), None, right)
            }
            ast::PrefixOp::CustomOp(custom_op) => self.lower_custom_op(custom_op, None, right),
            ast::PrefixOp::OperatorCall(operator_call) => {
                self.lower_operator_call(operator_call, None, right)
            }
            ast::PrefixOp::Not(_) => None,
        }
    }

    fn lower_custom_op(
        &mut self,
        custom_op: &ast::CustomOp,
        left: Option<ArgType>,
        right: Option<ArgType>,
    ) -> Option<Ty> {
        let range = custom_op.syntax().text_range();
        let text = custom_op.operator_text();
        self.add_expr_operator_ref(range, None, &text, left, right)
    }

    fn lower_operator_call(
        &mut self,
        operator_call: &ast::OperatorCall,
        left: Option<ArgType>,
        right: Option<ArgType>,
    ) -> Option<Ty> {
        let op = operator_call.op()?;
        let path = op.path_ref();
        let schema = self.lower_schema_qualifier(path.as_ref().and_then(|path| path.qualifier()));
        let range = op.operator_range();
        let text = op.operator_text();
        self.add_expr_operator_ref(range, schema, &text, left, right)
    }

    fn add_expr_operator_ref(
        &mut self,
        range: TextRange,
        schema: Option<Name>,
        text: &str,
        left: Option<ArgType>,
        right: Option<ArgType>,
    ) -> Option<Ty> {
        let name = self.intern(text);
        let resolution = self.select_operator(schema, name, left, right);
        self.add_ref_at(name, ReferenceKind::Operator, range, resolution);
        let Resolution::Operator(operator) = resolution else {
            return None;
        };
        let function = self.index.operators[operator].function?;
        let args: SmallVec<[CallArg; 2]> = [left, right]
            .into_iter()
            .flatten()
            .map(|ty| CallArg {
                ty,
                ..CallArg::default()
            })
            .collect();
        self.call_type(function, &args)
    }

    fn lower_exclude_constraint(
        &mut self,
        table: Option<TableId>,
        exclude: &ast::ExcludeConstraint,
    ) {
        let refs = self.index.references.next_idx();
        let access_method = match exclude
            .constraint_index_method()
            .and_then(|method| method.access_method_ref())
        {
            Some(access_method_ref) => self.lower_access_method_ref(
                &access_method_ref,
                Some(AccessMethodKind::Index),
                IfMissing::Error,
            ),
            None => {
                let btree = self.intern("btree");
                self.find_access_method(btree)
            }
        };
        if let Some(tablespace) = exclude.constraint_index_tablespace() {
            self.lower_constraint_index_tablespace(&tablespace);
        }
        let mut addition = String::new();
        for exclusion in exclude
            .constraint_exclusion_list()
            .into_iter()
            .flat_map(|list| list.constraint_exclusions())
        {
            let Some(item) = exclusion.partition_item() else {
                continue;
            };
            if let Some(path) = item.op_class_ref().and_then(|op_class| op_class.path_ref()) {
                self.lower_operator_class_ref(&path, access_method, IfMissing::Error);
            }
            let Some(expr) = item.expr() else {
                continue;
            };
            let ty = self.lower_expr_in(ExprContext::IndexExpression, &expr);
            let op = exclusion
                .op()
                .or_else(|| exclusion.operator_call().and_then(|call| call.op()));
            if let Some(op) = op {
                self.lower_exclusion_operator_ref(&op, ty);
            }
            if !addition.is_empty() {
                addition.push('_');
            }
            let column = match &expr {
                ast::Expr::Collate(collate) => collate.expr(),
                _ => Some(expr),
            };
            match &column {
                Some(ast::Expr::NameRef(name_ref)) => {
                    addition.push_str(&name_ref.text());
                }
                _ => addition.push_str("expr"),
            }
        }
        for column_ref in exclude
            .constraint_include_clause()
            .and_then(|include| include.column_ref_list())
            .into_iter()
            .flat_map(|list| list.column_name_refs())
        {
            let column = self.lower_constraint_column_ref(table, &column_ref);
            if !addition.is_empty() {
                addition.push('_');
            }
            addition.push_str(self.index.names.resolve(column));
        }
        if let Some(expr) = exclude
            .where_condition_clause()
            .and_then(|clause| clause.expr())
        {
            self.lower_expr_in(ExprContext::IndexPredicate, &expr);
        }
        let Some(table) = table else {
            return;
        };
        let name = exclude
            .constraint_name_clause()
            .and_then(|clause| clause.constraint_name());
        let constraint = self.alloc_index_constraint(
            ConstraintKind::Exclude,
            table,
            exclude.syntax(),
            name,
            None,
            Some(&addition),
            "excl",
        );
        self.depend_on_refs(refs, IdxRange::single(constraint), Object::Constraint);
    }

    fn lower_foreign_key_constraint(
        &mut self,
        table: Option<TableId>,
        foreign_key: &ast::ForeignKeyConstraint,
    ) {
        let refs = self.index.references.next_idx();
        let columns = self.lower_foreign_key_columns(table, foreign_key.from_columns());
        let referenced = foreign_key
            .table_name_ref()
            .and_then(|name| name.path_ref())
            .and_then(|path| self.lower_table_ref(&path, IfMissing::Error, TableLookup::Table));
        self.lower_foreign_key_columns(referenced, foreign_key.to_columns());
        self.lower_referential_actions(table, foreign_key.referential_actions());
        let Some(table) = table else {
            return;
        };
        let name = match foreign_key
            .constraint_name_clause()
            .and_then(|clause| clause.constraint_name())
        {
            Some(name) => ConstraintName::Explicit(name),
            None => {
                let addition = postgres::join_names(
                    columns
                        .iter()
                        .map(|column| self.index.names.resolve(*column)),
                );
                ConstraintName::Generated(self.choose_constraint_name(
                    ConstraintOwner::Table(table),
                    Some(&addition),
                    "fkey",
                ))
            }
        };
        let constraint = self.alloc_constraint(
            ConstraintKind::ForeignKey,
            ConstraintOwner::Table(table),
            foreign_key.syntax(),
            name,
            None,
        );
        self.depend_on_refs(refs, IdxRange::single(constraint), Object::Constraint);
    }

    fn lower_references_constraint(
        &mut self,
        table: Option<TableId>,
        column: Option<Name>,
        references: &ast::ReferencesConstraint,
    ) {
        let refs = self.index.references.next_idx();
        let referenced = references
            .table()
            .and_then(|name| name.path_ref())
            .and_then(|path| self.lower_table_ref(&path, IfMissing::Error, TableLookup::Table));
        if let Some(column_ref) = references.column() {
            self.lower_constraint_column_ref(referenced, &column_ref);
        }
        self.lower_referential_actions(table, references.referential_actions());
        let Some(table) = table else {
            return;
        };
        let name = match references
            .constraint_name_clause()
            .and_then(|clause| clause.constraint_name())
        {
            Some(name) => ConstraintName::Explicit(name),
            None => {
                let addition = column.map(|column| self.index.names.resolve(column).to_owned());
                ConstraintName::Generated(self.choose_constraint_name(
                    ConstraintOwner::Table(table),
                    addition.as_deref(),
                    "fkey",
                ))
            }
        };
        let constraint = self.alloc_constraint(
            ConstraintKind::ForeignKey,
            ConstraintOwner::Table(table),
            references.syntax(),
            name,
            None,
        );
        self.depend_on_refs(refs, IdxRange::single(constraint), Object::Constraint);
    }

    fn lower_foreign_key_columns(
        &mut self,
        table: Option<TableId>,
        list: Option<ast::ForeignKeyColumnList>,
    ) -> SmallVec<[Name; 4]> {
        let mut columns = SmallVec::new();
        let Some(list) = list else {
            return columns;
        };
        for column_ref in list
            .column_name_refs()
            .chain(list.period_column().and_then(|period| period.name()))
        {
            columns.push(self.lower_constraint_column_ref(table, &column_ref));
        }
        columns
    }

    fn lower_referential_actions(
        &mut self,
        table: Option<TableId>,
        actions: ast::AstChildren<ast::ReferentialAction>,
    ) {
        for action in actions {
            let ref_action = match action {
                ast::ReferentialAction::OnDeleteAction(action) => action.ref_action(),
                ast::ReferentialAction::OnUpdateAction(action) => action.ref_action(),
            };
            let list = match ref_action {
                Some(ast::RefAction::SetNullColumns(set_null)) => set_null.column_ref_list(),
                Some(ast::RefAction::SetDefaultColumns(set_default)) => {
                    set_default.column_ref_list()
                }
                Some(
                    ast::RefAction::Cascade(_)
                    | ast::RefAction::NoAction(_)
                    | ast::RefAction::Restrict(_),
                )
                | None => None,
            };
            for column_ref in list.into_iter().flat_map(|list| list.column_name_refs()) {
                self.lower_constraint_column_ref(table, &column_ref);
            }
        }
    }

    fn lower_index_parameters(
        &mut self,
        table: Option<TableId>,
        column: Option<Name>,
        params: Option<ast::IndexParameters>,
    ) -> IndexColumns {
        let mut columns: SmallVec<[Name; 4]> = column.into_iter().collect();
        let Some(params) = params else {
            let key_len = columns.len();
            return IndexColumns { columns, key_len };
        };
        if let Some(tablespace) = params.constraint_index_tablespace() {
            self.lower_constraint_index_tablespace(&tablespace);
        }
        for column_ref in params
            .column_list()
            .into_iter()
            .flat_map(|list| list.column_name_refs())
        {
            columns.push(self.lower_constraint_column_ref(table, &column_ref));
        }
        let key_len = columns.len();
        for column_ref in params
            .constraint_include_clause()
            .and_then(|include| include.column_ref_list())
            .into_iter()
            .flat_map(|list| list.column_name_refs())
        {
            columns.push(self.lower_constraint_column_ref(table, &column_ref));
        }
        IndexColumns { columns, key_len }
    }

    fn lower_using_index(&mut self, using: &ast::UsingIndexName) -> Option<IndexId> {
        let path = using.index_ref()?.path_ref()?;
        self.lower_index_ref(&path, IfMissing::Error)
    }

    fn lower_constraint_column_ref(
        &mut self,
        table: Option<TableId>,
        column_ref: &ast::ColumnNameRef,
    ) -> Name {
        self.lower_table_column_name(table, column_ref)
    }

    fn lower_table_column_name(&mut self, table: Option<TableId>, node: &impl NameLike) -> Name {
        match table {
            Some(table) => {
                let columns = self.index.tables[table].columns;
                self.lower_column_name_in(&columns, node)
            }
            None => {
                let name = self.intern(&node.text());
                self.add_ref(
                    name,
                    ReferenceKind::Column,
                    node.syntax(),
                    Resolution::Unknown,
                );
                name
            }
        }
    }

    fn lower_column_ref_in(&mut self, columns: &Columns, column_ref: &ast::ColumnNameRef) -> Name {
        self.lower_column_name_in(columns, column_ref)
    }

    fn lower_column_name_in(&mut self, columns: &Columns, node: &impl NameLike) -> Name {
        let name = self.intern(&node.text());
        let resolution = match self.find_column_in(columns.range, name) {
            Some(column) => Resolution::Column {
                source: None,
                column,
            },
            None if columns.complete => Resolution::Unresolved,
            None => Resolution::Unknown,
        };
        self.add_ref(name, ReferenceKind::Column, node.syntax(), resolution);
        name
    }

    fn choose_constraint_name(
        &mut self,
        owner: ConstraintOwner,
        addition: Option<&str>,
        label: &str,
    ) -> Name {
        let schema = self.constraint_owner_schema(owner);
        let name = postgres::choose_constraint_name(
            self.index.names.resolve(self.constraint_owner_name(owner)),
            addition,
            label,
            |name| self.constraint_name_exists(schema, name),
        );
        self.intern(&name)
    }

    fn relation_name_exists(&self, schema: Name, name: &str) -> bool {
        self.index
            .names
            .lookup(name)
            .is_some_and(|name| self.relation_exists(schema, name))
    }

    fn relation_exists(&self, schema: Name, name: Name) -> bool {
        self.find_index_in(schema, name).is_some() || self.find_table(Some(schema), name).is_some()
    }

    fn constraint_name_exists(&self, schema: Name, name: &str) -> bool {
        self.index.names.lookup(name).is_some_and(|name| {
            self.index.constraints.named(name).any(|id| {
                let owner = self.index.constraints[id].owner;
                self.constraint_owner_schema(owner) == schema
                    && !self.dropped_constraints.contains(&id)
                    && !self.constraint_owner_dropped(owner)
            })
        })
    }

    fn lower_constraint_ref(
        &mut self,
        owner: Option<ConstraintOwner>,
        name_ref: &ast::ConstraintNameRef,
        if_missing: IfMissing,
    ) -> Option<ConstraintId> {
        let segment = name_ref.path_ref()?.segment()?;
        let name = self.intern(&segment.text());
        let constraint = owner.and_then(|owner| self.find_constraint(owner, name));
        let resolution = match (constraint, owner) {
            (Some(constraint), _) => Resolution::Constraint(constraint),
            (None, Some(owner)) if self.constraints_complete(owner) => match if_missing {
                IfMissing::Ignore => Resolution::Unknown,
                IfMissing::Error => Resolution::Unresolved,
            },
            (None, _) => Resolution::Unknown,
        };
        self.add_ref(
            name,
            ReferenceKind::Constraint,
            segment.syntax(),
            resolution,
        );
        constraint
    }

    fn constraint_owner_schema(&self, owner: ConstraintOwner) -> Name {
        match owner {
            ConstraintOwner::Table(table) => self.index.tables[table].schema,
            ConstraintOwner::Domain(domain) => self.index.types[domain].schema,
        }
    }

    fn constraint_owner_name(&self, owner: ConstraintOwner) -> Name {
        match owner {
            ConstraintOwner::Table(table) => self.index.tables[table].name,
            ConstraintOwner::Domain(domain) => self.index.types[domain].name,
        }
    }

    fn constraint_owner_dropped(&self, owner: ConstraintOwner) -> bool {
        match owner {
            ConstraintOwner::Table(table) => self.dropped_tables.contains(&table),
            ConstraintOwner::Domain(domain) => self.dropped_types.contains(&domain),
        }
    }

    fn constraints_complete(&self, owner: ConstraintOwner) -> bool {
        match owner {
            ConstraintOwner::Table(table) => self.index.tables[table].constraints_complete,
            ConstraintOwner::Domain(domain) => {
                self.index.types[domain].ptr.file_id != FileId::Builtins.into()
            }
        }
    }

    fn find_constraint(&self, owner: ConstraintOwner, name: Name) -> Option<ConstraintId> {
        self.index.constraints.owned_by(owner).find(|id| {
            self.index.constraints[*id].name == name && !self.dropped_constraints.contains(id)
        })
    }

    fn lower_set_constraints(&mut self, set_constraints: &ast::SetConstraints) {
        for path in set_constraints
            .constraint_name_refs()
            .filter_map(|name_ref| name_ref.path_ref())
        {
            let Some(segment) = path.segment() else {
                continue;
            };
            let name = self.intern(&segment.text());
            let schema = self.lower_schema_qualifier(path.qualifier());
            let constraint = self
                .search_path(schema, Lookup::Relation)
                .find_map(|schema| self.find_constraint_in(schema, name));
            let resolution = constraint.map_or(Resolution::Unresolved, Resolution::Constraint);
            self.add_ref(
                name,
                ReferenceKind::Constraint,
                segment.syntax(),
                resolution,
            );
        }
    }

    fn find_constraint_in(&self, schema: Name, name: Name) -> Option<ConstraintId> {
        self.index.constraints.named(name).find(|id| {
            let ConstraintOwner::Table(table_id) = self.index.constraints[*id].owner else {
                return false;
            };
            let table = &self.index.tables[table_id];
            table.schema == schema
                && self.visible(table.schema, table.ptr.file_id)
                && !self.dropped_constraints.contains(id)
                && !self.dropped_tables.contains(&table_id)
        })
    }

    fn find_index_in(&self, schema: Name, name: Name) -> Option<IndexId> {
        self.index.indexes.named(schema, name).find(|id| {
            let index = &self.index.indexes[*id];
            self.visible(index.schema, index.ptr.file_id)
                && !self.dropped_indexes.contains(id)
                && index
                    .table
                    .is_none_or(|table| !self.dropped_tables.contains(&table))
        })
    }

    fn find_table(&self, schema: Option<Name>, name: Name) -> Option<TableId> {
        self.search_path(schema, Lookup::Relation)
            .find_map(|schema| {
                self.index.tables.named(schema, name).find(|id| {
                    let table = &self.index.tables[*id];
                    self.visible(table.schema, table.ptr.file_id)
                        && !self.dropped_tables.contains(id)
                        && table
                            .sequence_owner
                            .is_none_or(|owner| !self.dropped_tables.contains(&owner.table))
                })
            })
    }

    fn search_path(&self, schema: Option<Name>, lookup: Lookup) -> impl Iterator<Item = Name> {
        let implicit = schema.is_none();
        let implicit_temp =
            (implicit && lookup.searches_temp() && !self.search_path.contains(&self.pg_temp))
                .then_some(self.pg_temp);
        let implicit_catalog =
            (implicit && !self.search_path.contains(&self.pg_catalog)).then_some(self.pg_catalog);
        schema
            .into_iter()
            .chain(implicit_temp)
            .chain(implicit_catalog)
            .chain(self.search_path.iter().copied().filter(move |schema| {
                implicit
                    && *schema != self.dollar_user
                    && (lookup.searches_temp() || *schema != self.pg_temp)
            }))
    }

    fn visible(&self, schema: Name, file_id: IrFileId) -> bool {
        schema != self.pg_temp || self.index.file(file_id) == self.index.file(self.file_id)
    }

    fn lower_join_condition(
        &mut self,
        condition: ast::JoinCondition,
        start: usize,
        mid: usize,
        kind: JoinKind,
    ) {
        let visible = (start..self.scope_sources_len()).into();
        self.with_visible_sources(visible, |this| match condition {
            ast::JoinCondition::OnClause(on_clause) => {
                if let Some(expr) = on_clause.expr() {
                    this.lower_expr(&expr);
                }
            }
            ast::JoinCondition::JoinUsingClause(using_clause) => {
                this.lower_join_using(&using_clause, start, mid, kind);
            }
        });
    }

    fn lower_join_using(
        &mut self,
        using_clause: &ast::JoinUsingClause,
        start: usize,
        mid: usize,
        kind: JoinKind,
    ) {
        let alias = using_clause.alias().and_then(|alias| alias.name());
        let mut columns = vec![];
        let mut complete = true;
        for column_ref in using_clause
            .column_ref_list()
            .into_iter()
            .flat_map(|list| list.column_name_refs())
        {
            let name = self.intern(&column_ref.text());
            let MergedColumn { resolution, inputs } = self.merge_column(name, start, mid, kind);
            let input_resolution = match inputs {
                Some(inputs) => Resolution::Column {
                    source: None,
                    column: self.merge_inputs_column(inputs),
                },
                None => resolution,
            };
            self.add_ref(
                name,
                ReferenceKind::Column,
                column_ref.syntax(),
                input_resolution,
            );
            match resolution {
                Resolution::Column { column, .. } => columns.push(column),
                Resolution::Source(_)
                | Resolution::Cte(_)
                | Resolution::Table(_)
                | Resolution::Function(_)
                | Resolution::Param(_)
                | Resolution::Type(_)
                | Resolution::Window(_)
                | Resolution::JsonPath(_)
                | Resolution::Savepoint(_)
                | Resolution::Cursor(_)
                | Resolution::PreparedTransaction(_)
                | Resolution::PreparedStatement(_)
                | Resolution::Channel(_)
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
                | Resolution::Unknown
                | Resolution::Unresolved
                | Resolution::Ambiguous => complete = false,
            }
        }
        let Some(alias) = alias else {
            return;
        };
        let source = Source {
            name: Some(self.intern(&alias.text())),
            kind: SourceKind::UsingAlias,
            ptr: self.ptr(using_clause.syntax()),
            alias_ptr: Some(self.ast_ptr(&alias)),
            columns: Columns {
                range: self.index.columns.alloc_copies(columns),
                complete,
            },
        };
        self.push_source(source);
    }

    fn lower_natural_join(&mut self, start: usize, mid: usize, kind: JoinKind) {
        let query = self.scope().query;
        let right_sources = || {
            self.index.queries[query].sources[mid..]
                .iter()
                .map(|&source| &self.index.sources[source])
                .filter(|source| source.kind != SourceKind::UsingAlias)
        };
        let len = right_sources()
            .map(|source| source.columns.range.len())
            .sum();
        let mut right = FxHashSet::with_capacity_and_hasher(len, FxBuildHasher);
        right.extend(
            right_sources()
                .flat_map(|source| source.columns.range)
                .filter_map(|column| self.index.columns[column].name),
        );
        for index in start..mid {
            let source = &self.index.sources[self.index.queries[query].sources[index]];
            if source.kind == SourceKind::UsingAlias {
                continue;
            }
            for column in source.columns.range {
                if let Some(name) = self.index.columns[column].name
                    && right.remove(&name)
                {
                    self.merge_column(name, start, mid, kind);
                }
            }
        }
    }

    fn merge_inputs_column(&mut self, MergeInputs { left, right }: MergeInputs) -> ColumnId {
        let left_columns = self.index.merged_columns(left);
        let right_columns = self.index.merged_columns(right);
        let columns = self.alloc_merge_columns(left_columns, right_columns);
        let merge = self.index.merges.alloc(Merge {
            columns,
            plain: false,
        });
        self.index.columns.alloc(Column {
            merge: Some(merge),
            ..self.index.columns[left]
        })
    }

    fn alloc_merge_columns(
        &mut self,
        left: IdxRange<Column>,
        right: IdxRange<Column>,
    ) -> IdxRange<Column> {
        let inputs = left.iter().chain(right);
        let columns = self.index.columns.alloc_copies(inputs.clone());
        for (input, column) in inputs.zip(columns) {
            let owner = self.index.column_owner(input);
            if owner != ColumnOwner::Anonymous {
                self.index.column_owners.insert(column, owner);
            }
        }
        columns
    }

    fn merge_column(
        &mut self,
        name: Name,
        start: usize,
        mid: usize,
        kind: JoinKind,
    ) -> MergedColumn {
        let end = self.scope_sources_len();
        let left = self.lookup_column(self.scope(), start..mid, name);
        let right = self.lookup_column(self.scope(), mid..end, name);
        match (left, right) {
            (
                ColumnLookup::Found(Resolution::Column { column: left, .. }),
                ColumnLookup::Found(Resolution::Column { column: right, .. }),
            ) => {
                let left_ty = self.index.columns[left].ty;
                let right_ty = self.index.columns[right].ty;
                let ty = match (left_ty, right_ty) {
                    (Some(left_ty), Some(right_ty)) => self
                        .overloads()
                        .select_common_type(left_ty, right_ty)
                        .or(Some(left_ty)),
                    (left_ty, right_ty) => left_ty.or(right_ty),
                };
                let left_plain = self.is_plain_column(left) && (left_ty.is_none() || left_ty == ty);
                let right_plain =
                    self.is_plain_column(right) && (right_ty.is_none() || right_ty == ty);
                let side = match kind {
                    JoinKind::Inner if !left_plain && right_plain => MergeSide::Right,
                    JoinKind::Inner | JoinKind::Left => MergeSide::Left,
                    JoinKind::Right => MergeSide::Right,
                    JoinKind::Full => MergeSide::Both,
                };
                let left_columns = self.index.merged_columns(left);
                let right_columns = self.index.merged_columns(right);
                let merge = match side {
                    MergeSide::Left => Merge {
                        columns: left_columns,
                        plain: left_plain,
                    },
                    MergeSide::Right => Merge {
                        columns: right_columns,
                        plain: right_plain,
                    },
                    MergeSide::Both => Merge {
                        columns: self.alloc_merge_columns(left_columns, right_columns),
                        plain: false,
                    },
                };
                let merge = self.index.merges.alloc(merge);
                let column = self.index.columns.alloc(Column {
                    ty,
                    merge: Some(merge),
                    ..self.index.columns[left]
                });
                self.scope_mut().merges.push(ScopeMerge {
                    name,
                    column,
                    sources: (start..end).into(),
                });
                MergedColumn {
                    resolution: Resolution::Column {
                        source: None,
                        column,
                    },
                    inputs: Some(MergeInputs { left, right }),
                }
            }
            (ColumnLookup::Found(resolution), _) | (_, ColumnLookup::Found(resolution)) => {
                MergedColumn {
                    resolution,
                    inputs: None,
                }
            }
            (ColumnLookup::Ambiguous, _) | (_, ColumnLookup::Ambiguous) => MergedColumn {
                resolution: Resolution::Ambiguous,
                inputs: None,
            },
            (ColumnLookup::Missing { complete: false }, _)
            | (_, ColumnLookup::Missing { complete: false }) => MergedColumn {
                resolution: Resolution::Unknown,
                inputs: None,
            },
            (ColumnLookup::Missing { .. }, ColumnLookup::Missing { .. }) => MergedColumn {
                resolution: Resolution::Unresolved,
                inputs: None,
            },
        }
    }

    fn is_plain_column(&self, column: ColumnId) -> bool {
        self.index.columns[column]
            .merge
            .is_none_or(|merge| self.index.merges[merge].plain)
    }

    fn lower_target_list(&mut self, query: QueryId, list: Option<ast::TargetList>) -> Columns {
        let mut columns =
            Vec::with_capacity(list.as_ref().map_or(0, |list| list.targets().count()));
        let mut explicit = vec![];
        let mut complete = true;
        for target in list.into_iter().flat_map(|list| list.targets()) {
            if let Some(star) = target.star_token() {
                let start = self.index.star_columns.next_idx();
                let join_merges = match self.scopes.last() {
                    Some(scope) if scope.query == query => &scope.merges[..],
                    Some(_) | None => &[],
                };
                let index = &mut self.index;
                let from_sources = &index.queries[query].sources;
                JoinColumns {
                    join_merges,
                    from_sources,
                    source_arena: &index.sources,
                    column_arena: &mut index.columns,
                }
                .expand_from(0, |column_arena, id| {
                    columns.push(column_arena[id]);
                    index.star_columns.alloc(id);
                });
                complete &= from_sources.iter().all(|&id| {
                    let source = &index.sources[id];
                    source.kind.is_qualified_only() || source.columns.complete
                });
                self.add_star(&star, start);
                continue;
            }
            let Some(expr) = target.expr() else {
                continue;
            };
            if let ast::Expr::FieldExpr(field_expr) = &expr
                && let Some(star) = field_expr.star_token()
            {
                let Some(expanded) = self.lower_field_star(field_expr, &star).columns else {
                    complete = false;
                    continue;
                };
                columns.extend(expanded.range.iter().map(|id| self.index.columns[id]));
                complete &= expanded.complete;
                continue;
            }
            let ty = self.lower_expr(&expr);
            let Some((column_name, node)) = ColumnName::from_target(&target) else {
                continue;
            };
            let name = column_name.to_string().map(|name| self.intern(&name));
            if target.as_name().is_some() {
                explicit.push(columns.len());
            }
            columns.push(Column {
                name,
                ptr: self.ptr(&node),
                ty,
                merge: None,
            });
        }
        let range = self.index.columns.alloc_many(columns);
        let start = range.start().into_raw().into_u32() as usize;
        for offset in explicit {
            let id = nth_idx(start + offset);
            let ptr = self.index.columns[id].ptr;
            self.add_item_range(ptr.file_range(), Item::Column(id));
        }
        Columns { range, complete }
    }

    fn lower_field_star(&mut self, field_expr: &ast::FieldExpr, star: &SyntaxToken) -> FieldStar {
        let (ty, columns) = match self.lower_field_base(field_expr) {
            Some(FieldBase::Source(Some(source))) => (
                self.index.source_type(source),
                Some(self.index.sources[source].columns),
            ),
            Some(FieldBase::Expr(Some(ty))) => (Some(ty), self.ty_columns(ty).copied()),
            Some(FieldBase::Columns { columns, ty }) => (ty, Some(columns)),
            Some(FieldBase::Source(None) | FieldBase::Expr(None)) | None => (None, None),
        };
        if let Some(columns) = columns {
            let start = self.index.star_columns.next_idx();
            self.index.star_columns.alloc_many(columns.range);
            self.add_star(star, start);
        }
        FieldStar { ty, columns }
    }

    fn add_star(&mut self, star: &SyntaxToken, start: Idx<ColumnId>) {
        let columns = self.index.star_columns.range_from(start);
        let id = self.index.stars.alloc(Star { columns });
        self.add_token_item(star, Item::Star(id));
    }

    fn lower_tail_clauses_for_outputs(
        &mut self,
        query: QueryId,
        clauses: impl Iterator<Item = ast::SelectTailClause>,
    ) {
        let columns = self.index.queries[query].columns;
        for clause in clauses {
            self.lower_tail_clause(clause, &columns);
        }
    }

    fn lower_tail_clause(&mut self, clause: ast::SelectTailClause, outputs: &Columns) {
        match clause {
            ast::SelectTailClause::OrderByClause(order_by) => {
                for sort_by in order_by
                    .sort_by_list()
                    .into_iter()
                    .flat_map(|list| list.sort_bys())
                {
                    if let Some(expr) = sort_by.expr() {
                        self.lower_expr_with_outputs(&expr, outputs, OutputClause::OrderBy);
                    }
                }
            }
            ast::SelectTailClause::LimitClause(limit) => {
                if let Some(ast::LimitValue::Expr(expr)) = limit.limit_value() {
                    self.lower_expr_in(ExprContext::Limit, &expr);
                }
            }
            ast::SelectTailClause::OffsetClause(offset) => {
                if let Some(expr) = offset.expr() {
                    self.lower_expr_in(ExprContext::Offset, &expr);
                }
            }
            ast::SelectTailClause::FetchClause(fetch) => {
                if let Some(expr) = fetch.expr() {
                    self.lower_expr_in(ExprContext::Limit, &expr);
                }
            }
            ast::SelectTailClause::LockingClause(locking) => {
                self.lower_locking_clause(&locking);
            }
        }
    }

    fn lower_locking_clause(&mut self, locking: &ast::LockingClause) {
        for expr in locking
            .locking_of()
            .into_iter()
            .flat_map(|locking_of| locking_of.exprs())
        {
            let ast::Expr::NameRef(name_ref) = expr else {
                self.lower_expr(&expr);
                continue;
            };
            let Some(name) = self.name_ref_name(&name_ref) else {
                continue;
            };
            let resolution = self
                .find_visible_source(name)
                .map_or(Resolution::Unresolved, Resolution::Source);
            self.add_ref(name, ReferenceKind::Relation, name_ref.syntax(), resolution);
        }
    }

    fn lower_expr_with_outputs(
        &mut self,
        expr: &ast::Expr,
        outputs: &Columns,
        clause: OutputClause,
    ) {
        if let ast::Expr::NameRef(name_ref) = expr {
            self.lower_output_name(name_ref, outputs, clause);
        } else if let ast::Expr::Literal(literal) = expr
            && let Some(ast::LitKind::IntNumber(token)) = literal.kind()
        {
            self.lower_output_ordinal(literal, &token, outputs, clause);
        } else if let ast::Expr::ParenExpr(paren) = expr
            && paren.select_variant().is_none()
            && let Some(inner) = paren.expr()
        {
            self.lower_expr_with_outputs(&inner, outputs, clause);
        } else {
            self.lower_expr(expr);
        }
    }

    fn lower_output_ordinal(
        &mut self,
        literal: &ast::Literal,
        token: &SyntaxToken,
        outputs: &Columns,
        clause: OutputClause,
    ) {
        let Some(position) = literal
            .integer_value()
            .and_then(|value| usize::try_from(value).ok())
        else {
            return;
        };
        if !outputs.complete {
            return;
        }
        let Some(column) = position
            .checked_sub(1)
            .and_then(|index| outputs.range.iter().nth(index))
        else {
            let error = SemaError::InvalidOrdinal { clause, position };
            self.push_error(literal.syntax(), error);
            return;
        };
        let name = self.intern(token.text());
        self.add_ref(
            name,
            ReferenceKind::Column,
            literal.syntax(),
            Resolution::Column {
                source: None,
                column,
            },
        );
    }

    fn lower_output_name(
        &mut self,
        name_ref: &ast::NameRef,
        outputs: &Columns,
        clause: OutputClause,
    ) {
        let Some(name) = self.name_ref_name(name_ref) else {
            return;
        };
        let output = self
            .find_column_in(outputs.range, name)
            .map(|column| Resolution::Column {
                source: None,
                column,
            });
        let resolution = match (clause, output) {
            (OutputClause::OrderBy | OutputClause::DistinctOn, Some(output)) => output,
            (OutputClause::GroupBy, Some(output)) => match self.resolve_name(name) {
                NameResolution {
                    resolution: Resolution::Unresolved,
                    ..
                } => output,
                NameResolution { kind, resolution } => {
                    self.add_ref(name, kind, name_ref.syntax(), resolution);
                    return;
                }
            },
            (_, None) => {
                let NameResolution { kind, resolution } = self.resolve_name(name);
                self.add_ref(name, kind, name_ref.syntax(), resolution);
                return;
            }
        };
        self.add_ref(name, ReferenceKind::Column, name_ref.syntax(), resolution);
    }

    fn lower_expr(&mut self, expr: &ast::Expr) -> Option<Ty> {
        match expr {
            ast::Expr::NameRef(name_ref) => {
                if let Some(function) = name_ref
                    .sql_value_function()
                    .and_then(sql_value_function_name)
                {
                    return self.lower_system_call(function, &[], name_ref.syntax().text_range());
                }
                let name = self.name_ref_name(name_ref)?;
                let NameResolution { kind, resolution } = self.resolve_name(name);
                self.add_ref(name, kind, name_ref.syntax(), resolution);
                self.check_default_column_ref(name_ref.syntax(), resolution);
                return self.resolution_type(resolution);
            }
            ast::Expr::FieldExpr(field_expr) => {
                if let Some(star) = field_expr.star_token() {
                    return self.lower_field_star(field_expr, &star).ty;
                }
                if let Some(resolution) = self.lower_param_field(field_expr) {
                    return self.resolution_type(resolution);
                }
                return self.lower_field_expr(field_expr);
            }
            ast::Expr::CallExpr(call) => {
                let LoweredCall { args, callee } = self.lower_call_with_args(call);
                return match callee?.resolution {
                    Resolution::Function(function) => self.call_type(function, &args),
                    resolution => self.resolution_type(resolution),
                };
            }
            ast::Expr::Literal(literal) => {
                if let Some(resolution) = self.lower_positional_param(literal) {
                    return self.resolution_type(resolution);
                }
                return self.literal_type(literal);
            }
            ast::Expr::ArrayExpr(array) => {
                let mut element = None;
                for expr in array.exprs() {
                    let ty = self.lower_expr(&expr);
                    element = element.or(ty);
                }
                if let Some(query) = array.select_variant() {
                    let query = self.lower_scalar_sublink(query);
                    element = element.or_else(|| {
                        let columns = self.index.queries[query].columns;
                        columns
                            .range
                            .iter()
                            .next()
                            .and_then(|column| self.index.columns[column].ty)
                    });
                }
                return element.map(Ty::array);
            }
            ast::Expr::BetweenExpr(between) => {
                self.lower_exprs(
                    [between.target(), between.start(), between.end()]
                        .into_iter()
                        .flatten(),
                );
                if let Some(ast::Expr::TupleExpr(target)) = between.target() {
                    for bound in [between.start(), between.end()].into_iter().flatten() {
                        if let ast::Expr::TupleExpr(bound) = bound {
                            self.check_row_length(&target, &bound);
                        }
                    }
                }
            }
            ast::Expr::BinExpr(bin) => {
                let op = bin.op();
                if let Some(ast::BinOp::AtTimeZone(at_time_zone)) = &op {
                    let expr = self.lower_positional_arg(bin.lhs());
                    let zone = self.lower_positional_arg(bin.rhs());
                    return self.lower_system_call(
                        "timezone",
                        &[zone, expr],
                        at_time_zone.syntax().text_range(),
                    );
                }
                if let Some(ast::BinOp::Overlaps(overlaps)) = &op {
                    return self.lower_overlaps(bin, overlaps);
                }
                let lhs = bin.lhs();
                let rhs = bin.rhs();
                let lhs_ty = lhs.as_ref().and_then(|lhs| self.lower_expr(lhs));
                let left = expr_arg_type(lhs.as_ref(), lhs_ty);
                let right = match &rhs {
                    Some(rhs) => self.lower_bin_rhs(op.as_ref(), lhs.as_ref(), rhs, left),
                    None => ArgType::Untyped,
                };
                let rhs_ty = match right {
                    ArgType::Known(ty) => Some(ty),
                    ArgType::UnknownLiteral | ArgType::Untyped => None,
                };
                if let (Some(op), Some(ast::Expr::TupleExpr(lhs)), Some(rhs)) = (&op, &lhs, &rhs) {
                    self.check_row_lengths(op, lhs, rhs);
                }
                let ty = op
                    .as_ref()
                    .and_then(|op| self.lower_bin_operator(op, left, right));
                if op.is_some_and(|op| is_comparison(&op)) {
                    if let Some(rhs) = &rhs {
                        self.lower_enum_comparison(lhs_ty, rhs);
                    }
                    if let Some(lhs) = &lhs {
                        self.lower_enum_comparison(rhs_ty, lhs);
                    }
                }
                return ty;
            }
            ast::Expr::CaseExpr(case) => {
                self.lower_case_keywords(case);
                self.lower_exprs(case.expr());
                for when in case
                    .when_clause_list()
                    .into_iter()
                    .flat_map(|list| list.when_clauses())
                {
                    self.lower_exprs([when.condition(), when.then()].into_iter().flatten());
                }
                self.lower_exprs(case.else_clause().and_then(|clause| clause.expr()));
            }
            ast::Expr::CastExpr(cast) => {
                self.lower_exprs(cast.expr());
                self.lower_exprs(cast.literal().map(ast::Expr::Literal));
                let ty = self.lower_type(&cast.ty()?);
                let literal = cast
                    .literal()
                    .or_else(|| cast.expr().and_then(|expr| expr.as_literal()));
                if let Some(enum_ty) = self.enum_type(ty)
                    && let Some(literal) = literal
                {
                    self.lower_enum_value(enum_ty, &literal);
                }
                return ty;
            }
            ast::Expr::Collate(collate) => {
                let ty = collate.expr().and_then(|expr| self.lower_expr(&expr));
                self.lower_collate(collate);
                return ty;
            }
            ast::Expr::IndexExpr(index) => {
                let ty = index.base().and_then(|base| self.lower_expr(&base));
                self.lower_exprs(index.index());
                return ty.and_then(|ty| self.subscripted(ty, false));
            }
            ast::Expr::ParenExpr(paren) => {
                let ty = paren.expr().and_then(|expr| self.lower_expr(&expr));
                if let Some(query) = paren.select_variant() {
                    let query = self.lower_scalar_sublink(query);
                    return self.single_column_type(query);
                }
                return ty;
            }
            ast::Expr::PostfixExpr(postfix) => match postfix.op() {
                Some(ast::PostfixOp::AtLocal(at_local)) => {
                    let expr = self.lower_positional_arg(postfix.expr());
                    return self.lower_system_call(
                        "timezone",
                        &[expr],
                        at_local.syntax().text_range(),
                    );
                }
                Some(ast::PostfixOp::IsNormalized(is_normalized)) => {
                    return self.lower_is_normalized(
                        postfix.expr(),
                        is_normalized.unicode_normal_form().is_some(),
                        is_normalized.syntax(),
                    );
                }
                Some(ast::PostfixOp::IsNotNormalized(is_not_normalized)) => {
                    return self.lower_is_normalized(
                        postfix.expr(),
                        is_not_normalized.unicode_normal_form().is_some(),
                        is_not_normalized.syntax(),
                    );
                }
                Some(
                    ast::PostfixOp::IsJson(_)
                    | ast::PostfixOp::IsJsonArray(_)
                    | ast::PostfixOp::IsJsonObject(_)
                    | ast::PostfixOp::IsJsonScalar(_)
                    | ast::PostfixOp::IsJsonValue(_)
                    | ast::PostfixOp::IsNotJson(_)
                    | ast::PostfixOp::IsNotJsonArray(_)
                    | ast::PostfixOp::IsNotJsonObject(_)
                    | ast::PostfixOp::IsNotJsonScalar(_)
                    | ast::PostfixOp::IsNotJsonValue(_)
                    | ast::PostfixOp::IsNull(_)
                    | ast::PostfixOp::NotNull(_),
                )
                | None => self.lower_exprs(postfix.expr()),
            },
            ast::Expr::PrefixExpr(prefix) => {
                let expr = prefix.expr();
                let ty = expr.as_ref().and_then(|expr| self.lower_expr(expr));
                let op = prefix.op()?;
                return self.lower_prefix_operator(&op, expr_arg_type(expr.as_ref(), ty));
            }
            ast::Expr::SliceExpr(slice) => {
                let ty = slice.base().and_then(|base| self.lower_expr(&base));
                self.lower_exprs([slice.start(), slice.end()].into_iter().flatten());
                return ty.and_then(|ty| self.subscripted(ty, true));
            }
            ast::Expr::TupleExpr(tuple) => {
                self.lower_exprs(tuple.exprs());
                return self.builtin_type("record");
            }
        }
        None
    }

    fn resolution_type(&self, resolution: Resolution) -> Option<Ty> {
        match resolution {
            Resolution::Column { column, .. } => self.index.columns[column].ty,
            Resolution::Source(source) => self.index.source_type(source),
            Resolution::Param(param) => self.index.params[param].ty,
            Resolution::Cte(_)
            | Resolution::Table(_)
            | Resolution::Function(_)
            | Resolution::Type(_)
            | Resolution::Window(_)
            | Resolution::JsonPath(_)
            | Resolution::Savepoint(_)
            | Resolution::Cursor(_)
            | Resolution::PreparedTransaction(_)
            | Resolution::PreparedStatement(_)
            | Resolution::Channel(_)
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
            | Resolution::Unknown
            | Resolution::Unresolved
            | Resolution::Ambiguous => None,
        }
    }

    fn lower_exprs(&mut self, exprs: impl IntoIterator<Item = ast::Expr>) {
        for expr in exprs {
            self.lower_expr(&expr);
        }
    }

    fn lower_func_arg_expr(&mut self, arg: &ast::FuncArgExpr) -> Option<Ty> {
        self.lower_expr(&arg.expr()?)
    }

    fn lower_order_by(&mut self, order_by: Option<ast::OrderByClause>) {
        for sort_by in order_by
            .and_then(|clause| clause.sort_by_list())
            .into_iter()
            .flat_map(|list| list.sort_bys())
        {
            self.lower_exprs(sort_by.expr());
        }
    }

    fn lower_group_by(&mut self, group_by: &ast::GroupBy, outputs: &Columns) {
        match group_by {
            ast::GroupBy::GroupingExpr(grouping) => {
                if let Some(expr) = grouping.expr() {
                    self.lower_expr_with_outputs(&expr, outputs, OutputClause::GroupBy);
                }
            }
            ast::GroupBy::GroupingRollup(rollup) => {
                for expr in rollup.exprs() {
                    self.lower_expr_with_outputs(&expr, outputs, OutputClause::GroupBy);
                }
            }
            ast::GroupBy::GroupingCube(cube) => {
                for expr in cube.exprs() {
                    self.lower_expr_with_outputs(&expr, outputs, OutputClause::GroupBy);
                }
            }
            ast::GroupBy::GroupingSets(sets) => {
                for group_by in sets.group_bys() {
                    self.lower_group_by(&group_by, outputs);
                }
            }
        }
    }

    fn lower_window_clause(&mut self, window_clause: &ast::WindowClause) {
        for window_def in window_clause.window_defs() {
            let clauses = window_def
                .window_spec()
                .map(|window_spec| {
                    self.lower_window_spec(&window_spec, WindowSpecKind::WindowClause)
                })
                .unwrap_or_default();
            let Some(window) = window_def.window() else {
                continue;
            };
            let name = self.intern(&window.text());
            if self.find_window(name).is_some() {
                self.push_error(window.syntax(), SemaError::DuplicateWindow { name });
            }
            let id = self.index.windows.alloc(Window {
                name,
                ptr: self.ast_ptr(&window_def),
                name_ptr: self.ast_ptr(&window),
                clauses,
            });
            self.record_comment(window_def.syntax(), Item::Window(id));
            self.add_item(window.syntax(), Item::Window(id));
            self.windows.push(id);
            self.scope_mut().windows.end = self.windows.len();
        }
    }

    fn find_window(&self, name: Name) -> Option<WindowId> {
        let scope = self.scopes.last()?;
        self.windows[scope.windows]
            .iter()
            .copied()
            .find(|&window| self.index.windows[window].name == name)
    }

    fn lower_window_ref(&mut self, window_ref: &ast::WindowRef) -> Option<WindowId> {
        let name = self.intern(&window_ref.text());
        let window = self.find_window(name);
        let resolution = window.map_or(Resolution::Unresolved, Resolution::Window);
        self.add_ref(name, ReferenceKind::Window, window_ref.syntax(), resolution);
        window
    }

    fn lower_window_spec(
        &mut self,
        window_spec: &ast::WindowSpec,
        kind: WindowSpecKind,
    ) -> WindowClauses {
        let mut clauses = WindowClauses {
            partition_by: window_spec.partition_by_clause().is_some(),
            order_by: window_spec.order_by_clause().is_some(),
            frame: window_spec.frame_clause().is_some(),
        };
        if let Some(window_ref) = window_spec.window_ref()
            && let Some(base) = self.lower_window_ref(&window_ref)
        {
            let Window {
                name,
                clauses: base_clauses,
                ..
            } = self.index.windows[base];
            let error = if clauses.partition_by {
                Some(SemaError::WindowOverride {
                    name,
                    clause: WindowClause::PartitionBy,
                })
            } else if clauses.order_by && base_clauses.order_by {
                Some(SemaError::WindowOverride {
                    name,
                    clause: WindowClause::OrderBy,
                })
            } else if base_clauses.frame {
                Some(SemaError::WindowFrameCopy {
                    name,
                    hint: kind == WindowSpecKind::OverClause && !clauses.order_by && !clauses.frame,
                })
            } else {
                None
            };
            if let Some(error) = error {
                self.push_error(window_ref.syntax(), error);
            }
            clauses.partition_by |= base_clauses.partition_by;
            clauses.order_by |= base_clauses.order_by;
        }
        self.with_expr_context(ExprContext::WindowDefinition, |this| {
            if let Some(partition_by) = window_spec.partition_by_clause() {
                this.lower_exprs(partition_by.exprs());
            }
            this.lower_order_by(window_spec.order_by_clause());
        });
        let Some(frame_clause) = window_spec.frame_clause() else {
            return clauses;
        };
        let context = match frame_clause.frame_units() {
            Some(ast::FrameUnits::FrameRange(_)) => ExprContext::WindowRange,
            Some(ast::FrameUnits::FrameRows(_)) => ExprContext::WindowRows,
            Some(ast::FrameUnits::FrameGroups(_)) => ExprContext::WindowGroups,
            None => ExprContext::WindowDefinition,
        };
        self.with_expr_context(context, |this| match frame_clause.frame_extent() {
            Some(ast::FrameExtent::FrameBetween(between)) => {
                for bound in [between.start(), between.end()].into_iter().flatten() {
                    this.lower_frame_bound(&bound);
                }
            }
            Some(ast::FrameExtent::FrameBound(bound)) => this.lower_frame_bound(&bound),
            None => {}
        });
        clauses
    }

    fn lower_frame_bound(&mut self, bound: &ast::FrameBound) {
        match bound {
            ast::FrameBound::ExprPreceding(preceding) => self.lower_exprs(preceding.expr()),
            ast::FrameBound::ExprFollowing(following) => self.lower_exprs(following.expr()),
            ast::FrameBound::CurrentRow(_)
            | ast::FrameBound::UnboundedPreceding(_)
            | ast::FrameBound::UnboundedFollowing(_) => {}
        }
    }

    fn lower_special_fn(&mut self, call: &ast::CallExpr) -> Option<LoweredCall> {
        if let Some(graph_table) = call.graph_table_fn() {
            self.lower_graph_table(&graph_table);
        }
        if let Some(all_fn) = call.all_fn() {
            self.lower_exprs(all_fn.expr());
            self.lower_opt_sublink(all_fn.select_variant());
        }
        if let Some(any_fn) = call.any_fn() {
            self.lower_exprs(any_fn.expr());
            self.lower_opt_sublink(any_fn.select_variant());
        }
        if let Some(some_fn) = call.some_fn() {
            self.lower_exprs(some_fn.expr());
            self.lower_opt_sublink(some_fn.select_variant());
        }
        if let Some(exists_fn) = call.exists_fn() {
            self.lower_opt_sublink(exists_fn.select_variant());
        }
        if let Some(collation_for_fn) = call.collation_for_fn() {
            let arg = self.lower_positional_arg(collation_for_fn.expr());
            let (Some(collation), Some(for_token)) = (
                collation_for_fn.collation_token(),
                collation_for_fn.for_token(),
            ) else {
                return None;
            };
            let range = collation.text_range().cover(for_token.text_range());
            return Some(self.lower_special_call("pg_collation_for", smallvec![arg], range));
        }
        if let Some(extract_fn) = call.extract_fn() {
            let field = CallArg {
                ty: ArgType::UnknownLiteral,
                ..CallArg::default()
            };
            let expr = self.lower_positional_arg(extract_fn.expr());
            let token = extract_fn.extract_token()?;
            return Some(self.lower_special_call(
                "extract",
                smallvec![field, expr],
                token.text_range(),
            ));
        }
        if let Some(position_fn) = call.position_fn() {
            let substring = self.lower_positional_arg(position_fn.pos());
            let string = self.lower_positional_arg(position_fn.string());
            let token = position_fn.position_token()?;
            return Some(self.lower_special_call(
                "position",
                smallvec![string, substring],
                token.text_range(),
            ));
        }
        if let Some(overlay_fn) = call.overlay_fn() {
            let args = match overlay_fn.overlay_args() {
                Some(ast::OverlayArgs::OverlayExprs(exprs)) => {
                    self.lower_func_arg_exprs(exprs.func_arg_exprs())
                }
                Some(ast::OverlayArgs::OverlayPlacing(placing)) => {
                    let mut args = smallvec![
                        self.lower_positional_arg(placing.string()),
                        self.lower_positional_arg(placing.placing()),
                        self.lower_positional_arg(placing.from()),
                    ];
                    if let Some(count) = placing.for_() {
                        args.push(self.lower_positional_arg(Some(count)));
                    }
                    args
                }
                None => smallvec![],
            };
            let token = overlay_fn.overlay_token()?;
            return Some(self.lower_special_call("overlay", args, token.text_range()));
        }
        if let Some(substring_fn) = call.substring_fn() {
            let args = match substring_fn.substring_args() {
                Some(ast::SubstringArgs::SubstringExprs(exprs)) => {
                    self.lower_func_arg_exprs(exprs.func_arg_exprs())
                }
                Some(ast::SubstringArgs::SubstringForFrom(args)) => {
                    let string = self.lower_positional_arg(args.string());
                    let count = self.lower_positional_arg(args.count());
                    let start = match args.start() {
                        Some(start) => self.lower_positional_arg(Some(start)),
                        None => CallArg {
                            ty: self
                                .builtin_type("int4")
                                .map_or(ArgType::Untyped, ArgType::Known),
                            ..CallArg::default()
                        },
                    };
                    smallvec![string, start, count]
                }
                Some(ast::SubstringArgs::SubstringFromFor(args)) => {
                    let mut lowered = smallvec![
                        self.lower_positional_arg(args.string()),
                        self.lower_positional_arg(args.start()),
                    ];
                    if let Some(count) = args.count() {
                        lowered.push(self.lower_positional_arg(Some(count)));
                    }
                    lowered
                }
                Some(ast::SubstringArgs::SubstringSimilarEscape(args)) => smallvec![
                    self.lower_positional_arg(args.string()),
                    self.lower_positional_arg(args.pattern()),
                    self.lower_positional_arg(args.escape()),
                ],
                None => smallvec![],
            };
            let token = substring_fn.substring_token()?;
            return Some(self.lower_special_call("substring", args, token.text_range()));
        }
        if let Some(trim_fn) = call.trim_fn() {
            let function = match trim_fn.trim_side() {
                Some(ast::TrimSide::TrimBoth(_)) | None => "btrim",
                Some(ast::TrimSide::TrimLeading(_)) => "ltrim",
                Some(ast::TrimSide::TrimTrailing(_)) => "rtrim",
            };
            let args = match trim_fn.trim_args() {
                Some(ast::TrimArgs::TrimExprFrom(args)) => {
                    let mut exprs = args.exprs();
                    let characters = exprs.next();
                    let mut lowered: SmallVec<[CallArg; 4]> = exprs
                        .map(|expr| self.lower_positional_arg(Some(expr)))
                        .collect();
                    lowered.push(self.lower_positional_arg(characters));
                    lowered
                }
                Some(ast::TrimArgs::TrimExprs(args)) => args
                    .exprs()
                    .map(|expr| self.lower_positional_arg(Some(expr)))
                    .collect(),
                Some(ast::TrimArgs::TrimFrom(args)) => args
                    .exprs()
                    .map(|expr| self.lower_positional_arg(Some(expr)))
                    .collect(),
                None => smallvec![],
            };
            let token = trim_fn.trim_token()?;
            return Some(self.lower_special_call(function, args, token.text_range()));
        }
        if let Some(json_array_agg) = call.json_array_agg_fn() {
            self.lower_exprs(
                json_array_agg
                    .json_value_expr()
                    .and_then(|value| value.expr()),
            );
            self.lower_order_by(json_array_agg.order_by_clause());
            if let Some(returning) = json_array_agg.json_returning_clause() {
                self.lower_json_returning(&returning);
            }
        }
        if let Some(json_array) = call.json_array_fn() {
            for format in json_array.json_expr_formats() {
                self.lower_exprs(format.expr());
            }
            for format in json_array.json_select_formats() {
                self.lower_opt_sublink(format.select_variant());
            }
            if let Some(returning) = json_array.json_returning_clause() {
                self.lower_json_returning(&returning);
            }
        }
        if let Some(json_exists) = call.json_exists_fn() {
            self.lower_exprs(
                [json_exists.document(), json_exists.path()]
                    .into_iter()
                    .flatten(),
            );
            self.lower_json_passing(json_exists.json_passing_clause());
            self.lower_json_behavior(
                json_exists
                    .json_on_error_clause()
                    .and_then(|clause| clause.json_behavior()),
            );
        }
        if let Some(json_fn) = call.json_fn() {
            self.lower_exprs(json_fn.expr());
        }
        if let Some(json_object_agg) = call.json_object_agg_fn() {
            if let Some(key_value) = json_object_agg.json_key_value() {
                self.lower_json_key_value(&key_value);
            }
            if let Some(returning) = json_object_agg.json_returning_clause() {
                self.lower_json_returning(&returning);
            }
        }
        if let Some(json_object) = call.json_object_fn() {
            for arg in json_object.func_arg_exprs() {
                self.lower_func_arg_expr(&arg);
            }
            for key_value in json_object.json_key_values() {
                self.lower_json_key_value(&key_value);
            }
            if let Some(returning) = json_object.json_returning_clause() {
                self.lower_json_returning(&returning);
            }
        }
        if let Some(json_query) = call.json_query_fn() {
            self.lower_exprs(
                [json_query.document(), json_query.path()]
                    .into_iter()
                    .flatten(),
            );
            self.lower_json_passing(json_query.json_passing_clause());
            self.lower_json_behavior(
                json_query
                    .json_on_empty_clause()
                    .and_then(|clause| clause.json_behavior()),
            );
            self.lower_json_behavior(
                json_query
                    .json_on_error_clause()
                    .and_then(|clause| clause.json_behavior()),
            );
            if let Some(returning) = json_query.json_returning_clause() {
                self.lower_json_returning(&returning);
            }
        }
        if let Some(json_value) = call.json_value_fn() {
            self.lower_exprs(
                [json_value.document(), json_value.path()]
                    .into_iter()
                    .flatten(),
            );
            self.lower_json_passing(json_value.json_passing_clause());
            self.lower_json_behavior(
                json_value
                    .json_on_empty_clause()
                    .and_then(|clause| clause.json_behavior()),
            );
            self.lower_json_behavior(
                json_value
                    .json_on_error_clause()
                    .and_then(|clause| clause.json_behavior()),
            );
            if let Some(returning) = json_value.json_returning_clause() {
                self.lower_json_returning(&returning);
            }
        }
        if let Some(json_scalar) = call.json_scalar_fn() {
            self.lower_exprs(json_scalar.expr());
        }
        if let Some(json_serialize) = call.json_serialize_fn() {
            self.lower_exprs(json_serialize.expr());
            if let Some(returning) = json_serialize.json_returning_clause() {
                self.lower_json_returning(&returning);
            }
        }
        if let Some(xml_element) = call.xml_element_fn() {
            for attr in xml_element
                .expr_as_xml_attr_list()
                .into_iter()
                .flat_map(|list| list.expr_as_xml_attrs())
            {
                self.lower_exprs(attr.expr());
            }
            self.lower_exprs(xml_element.exprs());
        }
        if let Some(xml_exists) = call.xml_exists_fn() {
            let passing = xml_exists.xml_row_passing_clause();
            let path =
                self.lower_positional_arg(passing.as_ref().and_then(|passing| passing.row()));
            let document = self.lower_positional_arg(
                passing
                    .and_then(|passing| passing.xml_passing_doc())
                    .and_then(|doc| doc.expr()),
            );
            let token = xml_exists.xmlexists_token()?;
            return Some(self.lower_special_call(
                "xmlexists",
                smallvec![path, document],
                token.text_range(),
            ));
        }
        if let Some(xml_forest) = call.xml_forest_fn() {
            for tag in xml_forest
                .expr_as_element_tag_list()
                .into_iter()
                .flat_map(|list| list.expr_as_element_tags())
            {
                self.lower_exprs(tag.expr());
            }
        }
        if let Some(xml_parse) = call.xml_parse_fn() {
            self.lower_exprs(xml_parse.expr());
        }
        if let Some(xml_pi) = call.xml_pi_fn() {
            self.lower_exprs(xml_pi.expr());
        }
        if let Some(xml_root) = call.xml_root_fn() {
            self.lower_exprs(xml_root.expr());
        }
        if let Some(xml_serialize) = call.xml_serialize_fn() {
            self.lower_exprs(xml_serialize.expr());
            if let Some(ty) = xml_serialize.ty() {
                self.lower_type(&ty);
            }
        }
        None
    }

    fn lower_json_returning(&mut self, returning: &ast::JsonReturningClause) {
        if let Some(ty) = returning.ty() {
            self.lower_type(&ty);
        }
    }

    fn lower_opt_sublink(&mut self, query: Option<ast::SelectVariant>) {
        if let Some(query) = query {
            self.lower_sublink(query);
        }
    }

    fn lower_scalar_sublink(&mut self, query: ast::SelectVariant) -> QueryId {
        let ptr = self.ptr(query.syntax());
        let query = self.lower_sublink(query);
        let columns = self.index.queries[query].columns;
        if columns.complete && columns.range.len() != 1 {
            self.push_error_ptr(ptr, SemaError::SubqueryNotSingleColumn);
        }
        query
    }

    fn lower_compared_sublink(&mut self, query: ast::SelectVariant, width: usize) -> QueryId {
        let ptr = self.ptr(query.syntax());
        let query = self.lower_sublink(query);
        let columns = self.index.queries[query].columns;
        if !columns.complete {
            return query;
        }
        let error = match columns.range.len().cmp(&width) {
            Ordering::Greater => Some(SemaError::SubqueryTooManyColumns),
            Ordering::Less => Some(SemaError::SubqueryTooFewColumns),
            Ordering::Equal => None,
        };
        if let Some(error) = error {
            self.push_error_ptr(ptr, error);
        }
        query
    }

    fn lower_sublink(&mut self, query: ast::SelectVariant) -> QueryId {
        let context = self.expr_context;
        if context.subquery_label().is_some() {
            self.push_error(query.syntax(), SemaError::MisplacedSubquery { context });
        }
        self.lower_query(query)
    }

    fn lower_json_key_value(&mut self, key_value: &ast::JsonKeyValue) {
        self.lower_exprs(key_value.expr());
        self.lower_exprs(key_value.json_value_expr().and_then(|value| value.expr()));
    }

    fn lower_json_passing(&mut self, passing: Option<ast::JsonPassingClause>) {
        for arg in passing
            .into_iter()
            .flat_map(|clause| clause.json_passing_args())
        {
            self.lower_exprs(arg.expr());
        }
    }

    fn lower_json_behavior(&mut self, behavior: Option<ast::JsonBehavior>) {
        if let Some(ast::JsonBehavior::JsonBehaviorDefault(default)) = behavior {
            self.lower_exprs(default.expr());
        }
    }

    fn lower_xml_row_passing(&mut self, passing: Option<ast::XmlRowPassingClause>) {
        let Some(passing) = passing else {
            return;
        };
        self.lower_exprs(passing.row());
        self.lower_exprs(passing.xml_passing_doc().and_then(|doc| doc.expr()));
    }

    fn lower_xml_table(&mut self, xml_table: &ast::XmlTable) -> Columns {
        for namespace in xml_table
            .xml_namespace_list()
            .into_iter()
            .flat_map(|list| list.xml_namespaces())
        {
            self.lower_exprs(namespace.expr());
        }
        self.lower_xml_row_passing(xml_table.xml_row_passing_clause());
        let xml_columns = xml_table
            .xml_table_column_list()
            .into_iter()
            .flat_map(|list| list.xml_table_columns());
        for option in xml_columns
            .clone()
            .filter_map(|column| column.xml_column_option_list())
            .flat_map(|list| list.xml_column_options())
        {
            match option {
                ast::XmlColumnOption::OptionDefault(default) => self.lower_exprs(default.expr()),
                ast::XmlColumnOption::OptionPath(path) => self.lower_exprs(path.expr()),
                ast::XmlColumnOption::OptionIdent(_)
                | ast::XmlColumnOption::OptionNotNull(_)
                | ast::XmlColumnOption::OptionNull(_) => {}
            }
        }
        let start = self.index.columns.next_idx();
        for column in xml_columns {
            let ty = match (column.ty(), column.ordinality_token()) {
                (Some(ty), _) => self.lower_type(&ty),
                (None, Some(_)) => self.builtin_type("int4"),
                (None, None) => None,
            };
            if let Some(column_name) = column.column_name() {
                let column = Column {
                    ty,
                    ..self.column_def(&column_name)
                };
                self.alloc_column_def(column);
            }
        }
        Columns {
            range: self.index.columns.range_from(start),
            complete: true,
        }
    }

    fn lower_call(&mut self, call: &ast::CallExpr) -> Option<Callee> {
        self.lower_call_with_args(call).callee
    }

    fn lower_call_with_args(&mut self, call: &ast::CallExpr) -> LoweredCall {
        let refs = self.index.references.next_idx();
        let aggregate_calls = self.aggregate_calls.len();
        let callee = call.expr();
        let row_source = self.row_source_arg(call);
        let CallArgs {
            mut args,
            named: named_args,
        } = self.lower_call_args(call.arg_list());
        let within = call.within_clause();
        let direct_args = args.len();
        for sort_by in within
            .as_ref()
            .and_then(|clause| clause.order_by_clause())
            .and_then(|clause| clause.sort_by_list())
            .into_iter()
            .flat_map(|list| list.sort_bys())
        {
            let ty = match sort_by.expr() {
                Some(expr) => {
                    let ty = self.lower_expr(&expr);
                    match ty {
                        _ if is_unknown_literal_expr(&expr) => ArgType::UnknownLiteral,
                        Some(ty) => ArgType::Known(ty),
                        None => ArgType::Untyped,
                    }
                }
                None => ArgType::Untyped,
            };
            args.push(CallArg {
                name: None,
                ty,
                variadic: false,
            });
        }
        let kind = if within.is_some() {
            CalleeKind::OrderedSet
        } else {
            CalleeKind::Plain
        };
        let target = callee
            .as_ref()
            .and_then(|callee| self.lower_callee(callee, &args, row_source, kind));
        args.truncate(direct_args);
        let function = match target.map(|target| target.resolution) {
            Some(Resolution::Function(function)) => Some(function),
            Some(
                Resolution::Column { .. }
                | Resolution::Source(_)
                | Resolution::Cte(_)
                | Resolution::Table(_)
                | Resolution::Param(_)
                | Resolution::Type(_)
                | Resolution::Window(_)
                | Resolution::JsonPath(_)
                | Resolution::Savepoint(_)
                | Resolution::Cursor(_)
                | Resolution::PreparedTransaction(_)
                | Resolution::PreparedStatement(_)
                | Resolution::Channel(_)
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
                | Resolution::Unknown
                | Resolution::Unresolved
                | Resolution::Ambiguous,
            )
            | None => None,
        };
        if let Some(function) = function
            && let Some(callee) = &callee
            && let Some(element) = self
                .overloads()
                .missing_array_type(&self.index.functions[function], &args)
        {
            let ty = self.index.ty_name(element);
            self.push_error(callee.syntax(), SemaError::MissingArrayType { ty });
        }
        if self.function.is_none()
            && let Some(function) = function
            && self.is_set_config(function)
        {
            self.lower_set_config(call);
        }
        if let Some(function) = function
            && self.is_sequence_fn(function)
            && let Some(arg) = call
                .arg_list()
                .and_then(|list| list.args().next())
                .and_then(|arg| arg.expr())
        {
            self.lower_sequence_arg(&arg);
        }
        self.lower_named_args(function, named_args);
        self.check_aggregate_call(call, function, refs, aggregate_calls);
        if let Some(expr) = call.filter_clause().and_then(|clause| clause.expr()) {
            self.lower_expr_in(ExprContext::Filter, &expr);
        }
        if call.over_clause().is_some()
            && let Some(arg_list) = call.arg_list()
        {
            if let Some(ast::AllOrDistinct::Distinct(distinct)) = arg_list.all_or_distinct() {
                self.push_error(distinct.syntax(), SemaError::WindowDistinct);
            } else if let Some(order_by) = arg_list.args().find_map(|arg| arg.order_by_clause()) {
                self.push_error(order_by.syntax(), SemaError::WindowAggregateOrderBy);
            }
        }
        match call.over_clause().and_then(|clause| clause.over_target()) {
            Some(ast::OverTarget::OverWindowSpec(spec)) => {
                if let Some(window_spec) = spec.window_spec() {
                    self.lower_window_spec(&window_spec, WindowSpecKind::OverClause);
                }
            }
            Some(ast::OverTarget::WindowRef(window_ref)) => {
                self.lower_window_ref(&window_ref);
            }
            None => {}
        }
        self.lower_special_fn(call).unwrap_or(LoweredCall {
            args,
            callee: target,
        })
    }

    fn check_aggregate_call(
        &mut self,
        call: &ast::CallExpr,
        function: Option<FunctionId>,
        refs: RefId,
        aggregate_calls: usize,
    ) {
        if call.over_clause().is_none()
            && let Some(function) = function
            && let Function {
                window: true, name, ..
            } = self.index.functions[function]
        {
            self.push_error(call.syntax(), SemaError::WindowWithoutOver { name });
        }
        let kind = if call.over_clause().is_some() {
            AggregateCallKind::Window
        } else if function
            .is_some_and(|function| self.index.functions[function].kind == FunctionKind::Aggregate)
        {
            AggregateCallKind::Aggregate
        } else {
            return;
        };
        if kind == AggregateCallKind::Aggregate && !self.aggregate_is_local(refs) {
            return;
        }
        let ptr = self.ptr(call.syntax());
        let context = self.expr_context;
        let misplaced = match kind {
            AggregateCallKind::Aggregate => {
                (!context.allows_aggregates()).then_some(SemaError::MisplacedAggregate { context })
            }
            AggregateCallKind::Window => {
                (!context.allows_windows()).then_some(SemaError::MisplacedWindow { context })
            }
        };
        let depth = self.scopes.len();
        if let Some(error) = misplaced {
            self.push_error_ptr(ptr, error);
        } else {
            for index in aggregate_calls..self.aggregate_calls.len() {
                let inner = self.aggregate_calls[index];
                if inner.depth != depth {
                    continue;
                }
                let error = match (kind, inner.kind) {
                    (AggregateCallKind::Aggregate, AggregateCallKind::Aggregate) => {
                        SemaError::NestedAggregate
                    }
                    (AggregateCallKind::Aggregate, AggregateCallKind::Window) => {
                        SemaError::AggregateContainsWindow
                    }
                    (AggregateCallKind::Window, AggregateCallKind::Window) => {
                        SemaError::NestedWindow
                    }
                    (AggregateCallKind::Window, AggregateCallKind::Aggregate) => continue,
                };
                self.push_error_ptr(inner.ptr, error);
                self.aggregate_calls.truncate(aggregate_calls);
                return;
            }
        }
        self.aggregate_calls.truncate(aggregate_calls);
        self.aggregate_calls
            .push(AggregateCall { kind, depth, ptr });
    }

    fn aggregate_is_local(&self, refs: RefId) -> bool {
        let Some(scope) = self.scopes.last() else {
            return true;
        };
        let sources = &self.index.queries[scope.query].sources;
        let mut outer = false;
        for id in self.index.references.range_from(refs) {
            let reference = &self.index.references[id];
            if reference.scope != Some(scope.scope) {
                continue;
            }
            match reference.resolution {
                Resolution::Column {
                    source: Some(source),
                    ..
                }
                | Resolution::Source(source) => {
                    if sources.contains(&source) {
                        return true;
                    }
                    outer = true;
                }
                Resolution::Column { source: None, .. } => return true,
                Resolution::Unresolved | Resolution::Unknown | Resolution::Ambiguous
                    if reference.kind == ReferenceKind::Column =>
                {
                    outer = true;
                }
                _ => {}
            }
        }
        !outer
    }

    fn with_expr_context<T>(&mut self, context: ExprContext, f: impl FnOnce(&mut Self) -> T) -> T {
        let prev = std::mem::replace(&mut self.expr_context, context);
        let result = f(self);
        self.expr_context = prev;
        result
    }

    fn lower_expr_in(&mut self, context: ExprContext, expr: &ast::Expr) -> Option<Ty> {
        self.with_expr_context(context, |this| this.lower_expr(expr))
    }

    fn lower_call_args(&mut self, arg_list: Option<ast::ArgList>) -> CallArgs {
        let mut args: SmallVec<[CallArg; 4]> = SmallVec::new();
        let mut named_args = vec![];
        for arg in arg_list.into_iter().flat_map(|list| list.args()) {
            let arg_expr = arg.func_arg_expr();
            let param_name_ref = match &arg_expr {
                Some(ast::FuncArgExpr::NamedArg(named_arg)) => named_arg.name(),
                Some(ast::FuncArgExpr::Expr(_)) | None => None,
            };
            let name = param_name_ref.map(|param_name_ref| {
                let name = self.intern(&param_name_ref.text());
                named_args.push(NamedArg {
                    name,
                    name_ref: param_name_ref,
                });
                name
            });
            let ty = match arg_expr.as_ref() {
                Some(arg_expr) => {
                    let ty = self.lower_func_arg_expr(arg_expr);
                    match ty {
                        _ if is_unknown_literal(arg_expr) => ArgType::UnknownLiteral,
                        Some(ty) => ArgType::Known(ty),
                        None => ArgType::Untyped,
                    }
                }
                None => ArgType::Untyped,
            };
            self.lower_order_by(arg.order_by_clause());
            args.push(CallArg {
                name,
                ty,
                variadic: arg.variadic_token().is_some(),
            });
        }
        CallArgs {
            args,
            named: named_args,
        }
    }

    fn lower_named_args(&mut self, function: Option<FunctionId>, named_args: Vec<NamedArg>) {
        for NamedArg {
            name,
            name_ref: param_name_ref,
        } in named_args
        {
            let resolution = function
                .and_then(|function| self.find_function_param(function, name))
                .map_or(Resolution::Unknown, Resolution::Param);
            self.add_ref(
                name,
                ReferenceKind::Param,
                param_name_ref.syntax(),
                resolution,
            );
        }
    }

    fn lower_callee(
        &mut self,
        callee: &ast::Expr,
        args: &[CallArg],
        row_source: Option<SourceId>,
        kind: CalleeKind,
    ) -> Option<Callee> {
        let (schema, name_ref) = match callee {
            ast::Expr::NameRef(name_ref) => (None, name_ref.clone()),
            ast::Expr::FieldExpr(field_expr) => match (field_expr.base(), field_expr.field()) {
                (Some(ast::Expr::NameRef(schema)), Some(field)) => (Some(schema), field),
                (Some(ast::Expr::FieldExpr(qualifier)), Some(field)) => {
                    let (Some(ast::Expr::NameRef(_)), Some(schema)) =
                        (qualifier.base(), qualifier.field())
                    else {
                        return None;
                    };
                    (Some(schema), field)
                }
                (Some(ast::Expr::NameRef(_) | ast::Expr::FieldExpr(_)), None)
                | (
                    Some(
                        ast::Expr::ArrayExpr(_)
                        | ast::Expr::BetweenExpr(_)
                        | ast::Expr::BinExpr(_)
                        | ast::Expr::CallExpr(_)
                        | ast::Expr::CaseExpr(_)
                        | ast::Expr::CastExpr(_)
                        | ast::Expr::Collate(_)
                        | ast::Expr::IndexExpr(_)
                        | ast::Expr::Literal(_)
                        | ast::Expr::ParenExpr(_)
                        | ast::Expr::PostfixExpr(_)
                        | ast::Expr::PrefixExpr(_)
                        | ast::Expr::SliceExpr(_)
                        | ast::Expr::TupleExpr(_),
                    )
                    | None,
                    Some(_) | None,
                ) => return None,
            },
            ast::Expr::ArrayExpr(_)
            | ast::Expr::BetweenExpr(_)
            | ast::Expr::BinExpr(_)
            | ast::Expr::CallExpr(_)
            | ast::Expr::CaseExpr(_)
            | ast::Expr::CastExpr(_)
            | ast::Expr::Collate(_)
            | ast::Expr::IndexExpr(_)
            | ast::Expr::Literal(_)
            | ast::Expr::ParenExpr(_)
            | ast::Expr::PostfixExpr(_)
            | ast::Expr::PrefixExpr(_)
            | ast::Expr::SliceExpr(_)
            | ast::Expr::TupleExpr(_) => return None,
        };
        if schema.is_none() && name_ref.is_function_like_keyword() {
            return None;
        }
        let name = self.name_ref_name(&name_ref)?;
        let schema = schema
            .filter(|schema| schema.sql_value_function().is_none() && !schema.text().is_empty())
            .map(|schema| self.lower_schema_ref(schema.syntax(), &schema.text()));
        let row_source = row_source.filter(|_| schema.is_none());
        let row_function = row_source.and_then(|source| self.find_row_function(source, name));
        let row_column = row_source.and_then(|source| self.find_source_column(source, name));
        let (kind, resolution) = match (row_function, row_column) {
            (Some(function), _) => (
                ReferenceKind::Function(RoutineKind::Function),
                Resolution::Function(function),
            ),
            (None, Some(column)) => (
                ReferenceKind::Column,
                Resolution::Column {
                    source: row_source,
                    column,
                },
            ),
            (None, None) => {
                let incomplete_row =
                    row_source.is_some_and(|source| !self.index.sources[source].columns.complete);
                let resolution = match self.find_routine(schema, name, args, FunctionKind::Function)
                {
                    Resolution::Unresolved if kind == CalleeKind::OrderedSet || incomplete_row => {
                        Resolution::Unknown
                    }
                    resolution => resolution,
                };
                (ReferenceKind::Function(RoutineKind::Function), resolution)
            }
        };
        self.add_ref(name, kind, name_ref.syntax(), resolution);
        Some(Callee { name, resolution })
    }

    fn row_source_arg(&mut self, call: &ast::CallExpr) -> Option<SourceId> {
        let mut args = call.arg_list()?.args();
        let (Some(arg), None) = (args.next(), args.next()) else {
            return None;
        };
        let name_ref = arg.expr()?.as_name_ref()?;
        let name = self.name_ref_name(&name_ref)?;
        match self.resolve_column(name) {
            Resolution::Column { .. } | Resolution::Ambiguous => None,
            Resolution::Source(_)
            | Resolution::Cte(_)
            | Resolution::Table(_)
            | Resolution::Function(_)
            | Resolution::Param(_)
            | Resolution::Type(_)
            | Resolution::Window(_)
            | Resolution::JsonPath(_)
            | Resolution::Savepoint(_)
            | Resolution::Cursor(_)
            | Resolution::PreparedTransaction(_)
            | Resolution::PreparedStatement(_)
            | Resolution::Channel(_)
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
            | Resolution::Unknown
            | Resolution::Unresolved => self.resolve_source(name),
        }
    }

    fn lower_func_arg_exprs(
        &mut self,
        args: impl Iterator<Item = ast::FuncArgExpr>,
    ) -> SmallVec<[CallArg; 4]> {
        args.map(|arg| {
            let ty = match self.lower_func_arg_expr(&arg) {
                _ if is_unknown_literal(&arg) => ArgType::UnknownLiteral,
                Some(ty) => ArgType::Known(ty),
                None => ArgType::Untyped,
            };
            CallArg {
                name: None,
                ty,
                variadic: false,
            }
        })
        .collect()
    }

    fn lower_overlaps(&mut self, bin: &ast::BinExpr, overlaps: &SyntaxToken) -> Option<Ty> {
        let mut args: SmallVec<[CallArg; 4]> = smallvec![];
        for side in [bin.lhs(), bin.rhs()] {
            if let Some(ast::Expr::TupleExpr(tuple)) = &side {
                for expr in tuple.exprs() {
                    args.push(self.lower_positional_arg(Some(expr)));
                }
            } else {
                self.lower_exprs(side);
            }
        }
        self.lower_system_call("overlaps", &args, overlaps.text_range())
    }

    fn lower_is_normalized(
        &mut self,
        expr: Option<ast::Expr>,
        has_form: bool,
        node: &SyntaxNode,
    ) -> Option<Ty> {
        let expr = self.lower_positional_arg(expr);
        let range = node.text_range();
        if has_form {
            let form = CallArg {
                ty: ArgType::UnknownLiteral,
                ..CallArg::default()
            };
            self.lower_system_call("is_normalized", &[expr, form], range)
        } else {
            self.lower_system_call("is_normalized", &[expr], range)
        }
    }

    fn lower_positional_arg(&mut self, expr: Option<ast::Expr>) -> CallArg {
        let Some(expr) = expr else {
            return CallArg::default();
        };
        let ty = match self.lower_expr(&expr) {
            _ if is_unknown_literal_expr(&expr) => ArgType::UnknownLiteral,
            Some(ty) => ArgType::Known(ty),
            None => ArgType::Untyped,
        };
        CallArg {
            name: None,
            ty,
            variadic: false,
        }
    }

    fn lower_system_call(
        &mut self,
        function: &str,
        args: &[CallArg],
        range: TextRange,
    ) -> Option<Ty> {
        let callee = self.lower_system_function_ref(function, args, range);
        let Resolution::Function(function) = callee.resolution else {
            return None;
        };
        self.call_type(function, args)
    }

    fn lower_system_function_ref(
        &mut self,
        function: &str,
        args: &[CallArg],
        range: TextRange,
    ) -> Callee {
        let name = self.intern(function);
        let resolution =
            self.find_routine(Some(self.pg_catalog), name, args, FunctionKind::Function);
        self.add_ref_at(
            name,
            ReferenceKind::Function(RoutineKind::Function),
            range,
            resolution,
        );
        Callee { name, resolution }
    }

    fn system_call_type(&mut self, function: &str, args: &[CallArg]) -> Option<Ty> {
        let name = self.intern(function);
        let Resolution::Function(function) =
            self.find_routine(Some(self.pg_catalog), name, args, FunctionKind::Function)
        else {
            return None;
        };
        self.call_type(function, args)
    }

    fn lower_special_call(
        &mut self,
        function: &str,
        args: SmallVec<[CallArg; 4]>,
        range: TextRange,
    ) -> LoweredCall {
        let callee = self.lower_system_function_ref(function, &args, range);
        LoweredCall {
            args,
            callee: Some(callee),
        }
    }

    fn find_routine(
        &self,
        schema: Option<Name>,
        name: Name,
        args: &[CallArg],
        kind: FunctionKind,
    ) -> Resolution {
        let candidates = self
            .search_path(schema, Lookup::Function)
            .flat_map(|schema| {
                self.index.functions.named(schema, name).filter(move |id| {
                    let function = &self.index.functions[*id];
                    (function.kind == FunctionKind::Procedure) == (kind == FunctionKind::Procedure)
                        && self.visible(function.schema, function.ptr.file_id)
                        && !self.dropped_functions.contains(id)
                })
            });
        self.overloads().select(candidates, args)
    }

    fn overloads(&self) -> Overloads<'_> {
        Overloads::new(
            &self.index,
            self.pg_catalog,
            self.builtin_types,
            &self.dropped_casts,
        )
    }

    fn call_type(&self, function: FunctionId, args: &[CallArg]) -> Option<Ty> {
        self.overloads()
            .return_type(&self.index.functions[function], args)
    }

    fn find_row_function(&self, source: SourceId, name: Name) -> Option<FunctionId> {
        let SourceKind::Table(table) = self.index.sources[source].kind else {
            return None;
        };
        self.find_function_by(None, name, |function| {
            function.kind != FunctionKind::Procedure
                && self
                    .overloads()
                    .accepts_args(function, &[CallArg::default()])
                && self.takes_row_of(function, table)
        })
    }

    fn takes_row_of(&self, function: &Function, table: TableId) -> bool {
        function
            .params
            .iter()
            .map(|id| &self.index.params[id])
            .find(|param| param.mode != ParamMode::Out)
            .is_some_and(|param| param.ty == Some(Ty::Table(table)))
    }

    fn find_function_by(
        &self,
        schema: Option<Name>,
        name: Name,
        accepts: impl Fn(&Function) -> bool,
    ) -> Option<FunctionId> {
        self.search_path(schema, Lookup::Function)
            .find_map(|schema| {
                self.index.functions.named(schema, name).find(|id| {
                    let function = &self.index.functions[*id];
                    self.visible(function.schema, function.ptr.file_id)
                        && !self.dropped_functions.contains(id)
                        && accepts(function)
                })
            })
    }

    fn is_sequence_fn(&self, function: FunctionId) -> bool {
        let function = &self.index.functions[function];
        function.schema == self.pg_catalog
            && matches!(
                self.index.names.resolve(function.name),
                "nextval" | "currval" | "setval"
            )
    }

    fn lower_sequence_arg(&mut self, arg: &ast::Expr) {
        let Some(literal) = regclass_literal(arg) else {
            return;
        };
        let Some(decoded) = literal.decoded_value() else {
            return;
        };
        let Some(parts) = postgres::split_identifier_string(decoded.text(), '.') else {
            return;
        };
        let (schema, name) = match parts.as_slice() {
            [name] => (None, name),
            [schema, name] | [_, schema, name] => (Some(schema), name),
            _ => return,
        };
        let schema = schema.map(|schema_part| {
            let schema = self.intern_schema(&schema_part.name);
            let resolution = self
                .find_schema(schema)
                .map_or(Resolution::Unresolved, Resolution::Schema);
            self.add_ref_at(
                schema,
                ReferenceKind::Schema,
                decoded.source_range(schema_part.range),
                resolution,
            );
            schema
        });
        let name_range = name.range;
        let name = self.intern(&name.name);
        let resolution = self
            .find_table(schema, name)
            .filter(|table| self.index.tables[*table].kind == TableKind::Sequence)
            .map_or(Resolution::Unresolved, Resolution::Table);
        self.add_ref_at(
            name,
            ReferenceKind::Table,
            decoded.source_range(name_range),
            resolution,
        );
    }

    fn is_set_config(&self, function: FunctionId) -> bool {
        let function = &self.index.functions[function];
        function.schema == self.pg_catalog
            && self.index.names.resolve(function.name) == "set_config"
    }

    fn find_source_column(&self, source: SourceId, name: Name) -> Option<ColumnId> {
        self.find_column_in(self.index.sources[source].columns.range, name)
    }

    fn lower_param_field(&mut self, field_expr: &ast::FieldExpr) -> Option<Resolution> {
        let function = self.function?;
        let (Some(ast::Expr::NameRef(qualifier)), Some(field)) =
            (field_expr.base(), field_expr.field())
        else {
            return None;
        };
        let qualifier_name = self.name_ref_name(&qualifier)?;
        let field_name = self.name_ref_name(&field)?;
        if qualifier_name != self.index.functions[function].name
            || self.resolve_source(qualifier_name).is_some()
        {
            return None;
        }
        let resolution = self
            .find_param(field_name)
            .map_or(Resolution::Unresolved, Resolution::Param);
        self.add_ref(
            qualifier_name,
            ReferenceKind::Function(RoutineKind::Function),
            qualifier.syntax(),
            Resolution::Function(function),
        );
        self.add_ref(field_name, ReferenceKind::Param, field.syntax(), resolution);
        Some(resolution)
    }

    fn lower_positional_param(&mut self, literal: &ast::Literal) -> Option<Resolution> {
        let index = literal.positional_param_index()?;
        let resolution = match self.prepare_params {
            Some(params) => params
                .iter()
                .nth(index)
                .map_or(Resolution::Unknown, Resolution::Param),
            None => self.index.functions[self.function?]
                .params
                .iter()
                .filter(|id| self.index.params[*id].mode != ParamMode::Out)
                .nth(index)
                .map_or(Resolution::Unresolved, Resolution::Param),
        };
        let Some(ast::LitKind::PositionalParam(token)) = literal.kind() else {
            return None;
        };
        let name = self.intern(token.text());
        self.add_ref(name, ReferenceKind::Param, literal.syntax(), resolution);
        Some(resolution)
    }

    fn find_param(&self, name: Name) -> Option<ParamId> {
        self.find_function_param(self.function?, name)
    }

    fn find_function_param(&self, function: FunctionId, name: Name) -> Option<ParamId> {
        self.index.functions[function]
            .params
            .iter()
            .find(|id| self.index.params[*id].name == Some(name))
    }

    fn resolve_name(&self, name: Name) -> NameResolution {
        let (resolution, hidden) = match self.lookup_scoped_column(name) {
            ScopedColumn::Resolved(resolution) => (resolution, false),
            ScopedColumn::Hidden => (Resolution::Unresolved, true),
        };
        if !hidden && matches!(resolution, Resolution::Unresolved | Resolution::Unknown) {
            if let Some(source) = self.resolve_source(name) {
                return NameResolution {
                    kind: ReferenceKind::Relation,
                    resolution: Resolution::Source(source),
                };
            }
            if let Some(param) = self.find_param(name) {
                return NameResolution {
                    kind: ReferenceKind::Param,
                    resolution: Resolution::Param(param),
                };
            }
        }
        NameResolution {
            kind: ReferenceKind::Column,
            resolution,
        }
    }

    fn lower_field_expr(&mut self, field_expr: &ast::FieldExpr) -> Option<Ty> {
        let source = match self.lower_field_base(field_expr)? {
            FieldBase::Source(Some(source)) => source,
            FieldBase::Source(None) => {
                let field = field_expr.field()?;
                let name = self.name_ref_name(&field)?;
                self.add_ref(
                    name,
                    ReferenceKind::Column,
                    field.syntax(),
                    Resolution::Unknown,
                );
                return None;
            }
            FieldBase::Expr(ty) => {
                let field = field_expr.field()?;
                return self.add_field_ref(ty, &field);
            }
            FieldBase::Columns { columns, .. } => {
                let field = field_expr.field()?;
                return self.add_columns_field_ref(Some(columns), &field);
            }
        };
        let field = field_expr.field()?;
        let name = self.name_ref_name(&field)?;
        let (kind, resolution) = match self.find_source_column(source, name) {
            Some(_)
                if self.has_duplicate_column(self.index.sources[source].columns.range, name) =>
            {
                (ReferenceKind::Column, Resolution::Ambiguous)
            }
            Some(column) => (
                ReferenceKind::Column,
                Resolution::Column {
                    source: Some(source),
                    column,
                },
            ),
            None => match self.find_row_function(source, name) {
                Some(function) => (
                    ReferenceKind::Function(RoutineKind::Function),
                    Resolution::Function(function),
                ),
                None if self.index.sources[source].columns.complete => {
                    (ReferenceKind::Column, Resolution::Unresolved)
                }
                None => (ReferenceKind::Column, Resolution::Unknown),
            },
        };
        self.add_ref(name, kind, field.syntax(), resolution);
        self.check_default_column_ref(field_expr.syntax(), resolution);
        self.resolution_type(resolution)
    }

    fn check_default_column_ref(&mut self, node: &SyntaxNode, resolution: Resolution) {
        if self.expr_context == ExprContext::Default
            && matches!(
                resolution,
                Resolution::Column { .. } | Resolution::Source(_)
            )
        {
            self.push_error(node, SemaError::DefaultColumnReference);
        }
    }

    fn lower_field_base(&mut self, field_expr: &ast::FieldExpr) -> Option<FieldBase> {
        let base = field_expr.base()?;
        if let Some(call) = call_field_base(&base) {
            return Some(self.lower_call_field_base(&call));
        }
        let qualifier = match &base {
            ast::Expr::NameRef(name_ref) => Some(name_ref.clone()),
            ast::Expr::FieldExpr(inner) => match inner.base() {
                Some(ast::Expr::NameRef(schema)) => {
                    self.lower_schema_ref(schema.syntax(), &schema.text());
                    inner.field()
                }
                Some(ast::Expr::FieldExpr(qualified_schema))
                    if matches!(qualified_schema.base(), Some(ast::Expr::NameRef(_)))
                        && qualified_schema.field().is_some() =>
                {
                    if let Some(ast::Expr::NameRef(database)) = qualified_schema.base()
                        && let Some(name) = self.name_ref_name(&database)
                    {
                        self.add_database_ref(name, database.syntax(), IfMissing::Error);
                    }
                    if let Some(schema) = qualified_schema.field() {
                        self.lower_schema_ref(schema.syntax(), &schema.text());
                    }
                    inner.field()
                }
                Some(inner_base) if inner_base.is_name_path() => return None,
                Some(_) | None => None,
            },
            ast::Expr::ArrayExpr(_)
            | ast::Expr::BetweenExpr(_)
            | ast::Expr::BinExpr(_)
            | ast::Expr::CallExpr(_)
            | ast::Expr::CaseExpr(_)
            | ast::Expr::CastExpr(_)
            | ast::Expr::Collate(_)
            | ast::Expr::IndexExpr(_)
            | ast::Expr::Literal(_)
            | ast::Expr::ParenExpr(_)
            | ast::Expr::PostfixExpr(_)
            | ast::Expr::PrefixExpr(_)
            | ast::Expr::SliceExpr(_)
            | ast::Expr::TupleExpr(_) => None,
        };
        Some(match qualifier {
            Some(qualifier) => FieldBase::Source(self.lower_qualifier(&qualifier)),
            None => FieldBase::Expr(self.lower_expr(&base)),
        })
    }

    fn lower_call_field_base(&mut self, call: &ast::CallExpr) -> FieldBase {
        let LoweredCall { args, callee } = self.lower_call_with_args(call);
        let Some(Resolution::Function(function)) = callee.map(|callee| callee.resolution) else {
            return FieldBase::Expr(
                callee.and_then(|callee| self.resolution_type(callee.resolution)),
            );
        };
        let ty = self.call_type(function, &args);
        match self.index.functions[function].columns {
            Some(columns) => FieldBase::Columns { columns, ty },
            None => FieldBase::Expr(ty),
        }
    }

    fn lower_qualifier(&mut self, qualifier: &ast::NameRef) -> Option<SourceId> {
        let name = self.name_ref_name(qualifier)?;
        let source = self.resolve_source(name);
        let resolution = source.map_or(Resolution::Unresolved, Resolution::Source);
        self.add_ref(
            name,
            ReferenceKind::Relation,
            qualifier.syntax(),
            resolution,
        );
        source
    }

    fn find_cte(&self, name: Name) -> Option<CteId> {
        self.ctes
            .iter()
            .rev()
            .copied()
            .find(|cte| self.index.ctes[*cte].name == name)
    }

    fn find_visible_source(&self, name: Name) -> Option<SourceId> {
        self.visible_sources(self.scope())
            .find(|id| self.index.sources[*id].name == Some(name))
    }

    fn find_query_source(&self, query: QueryId, name: Name) -> Option<SourceId> {
        self.index.queries[query]
            .sources
            .iter()
            .copied()
            .find(|id| self.index.sources[*id].name == Some(name))
    }

    fn resolve_source(&self, name: Name) -> Option<SourceId> {
        self.scopes.iter().rev().find_map(|scope| {
            self.visible_sources(scope)
                .find(|id| self.index.sources[*id].name == Some(name))
        })
    }

    fn resolve_column(&self, name: Name) -> Resolution {
        match self.lookup_scoped_column(name) {
            ScopedColumn::Resolved(resolution) => resolution,
            ScopedColumn::Hidden => Resolution::Unresolved,
        }
    }

    fn lookup_scoped_column(&self, name: Name) -> ScopedColumn {
        for scope in self.scopes.iter().rev() {
            let indices = self.index.scopes[scope.scope].visible_indices();
            match self.lookup_column(scope, indices, name) {
                ColumnLookup::Found(resolution) => return ScopedColumn::Resolved(resolution),
                ColumnLookup::Ambiguous => return ScopedColumn::Resolved(Resolution::Ambiguous),
                ColumnLookup::Missing { complete: false } => {
                    return ScopedColumn::Resolved(Resolution::Unknown);
                }
                ColumnLookup::Missing { complete: true } => {}
            }
            if self.hidden_source_has_column(scope, name) {
                return ScopedColumn::Hidden;
            }
        }
        ScopedColumn::Resolved(Resolution::Unresolved)
    }

    fn hidden_source_has_column(&self, scope: &QueryScope, name: Name) -> bool {
        let sources = &self.index.queries[scope.query].sources;
        self.index.scopes[scope.scope]
            .hidden_indices()
            .map(|index| &self.index.sources[sources[index]])
            .filter(|source| !source.kind.is_qualified_only())
            .any(|source| self.columns_contain(source.columns.range, name))
    }

    fn lookup_column(
        &self,
        scope: &QueryScope,
        indices: impl Iterator<Item = usize>,
        name: Name,
    ) -> ColumnLookup {
        let sources = &self.index.queries[scope.query].sources;
        let mut found: Option<(usize, SourceId, ColumnId)> = None;
        let mut last = 0;
        let mut complete = true;
        for index in indices {
            let source = &self.index.sources[sources[index]];
            if source.kind.is_qualified_only() {
                continue;
            }
            let column = self.find_column_in(source.columns.range, name);
            match column {
                Some(column) => {
                    if self.has_duplicate_column(source.columns.range, name) {
                        return ColumnLookup::Ambiguous;
                    }
                    if found.is_none() {
                        found = Some((index, sources[index], column));
                    }
                    last = index;
                }
                None => complete &= source.columns.complete,
            }
        }
        let Some((first, source, column)) = found else {
            return ColumnLookup::Missing { complete };
        };
        if first == last {
            return ColumnLookup::Found(Resolution::Column {
                source: Some(source),
                column,
            });
        }
        match scope.merges.iter().rev().find(|merge| {
            merge.name == name && merge.sources.start <= first && last < merge.sources.end
        }) {
            Some(merge) => ColumnLookup::Found(Resolution::Column {
                source: None,
                column: merge.column,
            }),
            None => ColumnLookup::Ambiguous,
        }
    }

    fn name_ref_name(&mut self, name_ref: &ast::NameRef) -> Option<Name> {
        if name_ref.sql_value_function().is_some() {
            return None;
        }
        let text = name_ref.text();
        if text.is_empty() {
            return None;
        }
        Some(self.intern(&text))
    }

    fn column_def(&mut self, column_name: &ast::ColumnName) -> Column {
        Column {
            name: Some(self.intern(&column_name.text())),
            ptr: self.ptr(column_name.syntax()),
            ty: None,
            merge: None,
        }
    }

    fn alloc_schema(
        &mut self,
        if_exists: IfExists,
        create_schema: &ast::CreateSchema,
        name_node: &SyntaxNode,
        name: Name,
    ) -> Option<SchemaId> {
        if self.find_schema(name).is_some() {
            if if_exists == IfExists::Ignore {
                return None;
            }
            self.report_duplicate(ObjectKind::Schema, name_node, name);
        }
        let id = self.index.schemas.alloc(Schema {
            name,
            ptr: self.ast_ptr(create_schema),
            name_ptr: self.ptr(name_node),
        });
        self.record_comment(create_schema.syntax(), Item::Schema(id));
        self.add_item(name_node, Item::Schema(id));
        Some(id)
    }

    fn alloc_tablespace(
        &mut self,
        create_tablespace: &ast::CreateTablespace,
        tablespace: &ast::Tablespace,
    ) -> TablespaceId {
        let name = self.intern(&tablespace.text());
        if self.find_tablespace(name).is_some() {
            self.report_duplicate(ObjectKind::Tablespace, tablespace.syntax(), name);
        }
        let id = self.index.tablespaces.alloc(Tablespace {
            name,
            ptr: self.ast_ptr(create_tablespace),
            name_ptr: self.ast_ptr(tablespace),
        });
        self.record_comment(create_tablespace.syntax(), Item::Tablespace(id));
        self.add_item(tablespace.syntax(), Item::Tablespace(id));
        id
    }

    fn alloc_database(
        &mut self,
        create_database: &ast::CreateDatabase,
        database: &ast::Database,
    ) -> DatabaseId {
        let name = self.intern(&database.text());
        if self.find_database(name).is_some() {
            self.report_duplicate(ObjectKind::Database, database.syntax(), name);
        }
        let id = self.index.databases.alloc(Database {
            name,
            ptr: self.ast_ptr(create_database),
            name_ptr: self.ast_ptr(database),
        });
        self.record_comment(create_database.syntax(), Item::Database(id));
        self.add_item(database.syntax(), Item::Database(id));
        id
    }

    fn alloc_extension(
        &mut self,
        if_exists: IfExists,
        create_extension: &ast::CreateExtension,
        extension: &ast::Extension,
        schema: Name,
    ) -> Option<ExtensionId> {
        let name = self.intern(&extension.text());
        if self.find_extension(name).is_some() {
            if if_exists == IfExists::Ignore {
                return None;
            }
            self.report_duplicate(ObjectKind::Extension, extension.syntax(), name);
        }
        let id = self.index.extensions.alloc(Extension {
            schema,
            name,
            ptr: self.ast_ptr(create_extension),
            name_ptr: self.ast_ptr(extension),
        });
        self.record_comment(create_extension.syntax(), Item::Extension(id));
        self.add_item(extension.syntax(), Item::Extension(id));
        Some(id)
    }

    fn alloc_index(
        &mut self,
        if_exists: IfExists,
        node: &SyntaxNode,
        segment: &ast::PathSegment,
        schema: Name,
        table: Option<TableId>,
    ) -> Option<IndexId> {
        let name = self.intern(&segment.text());
        if self.relation_exists(schema, name) {
            if if_exists == IfExists::Ignore {
                return None;
            }
            self.report_duplicate(ObjectKind::Relation, segment.syntax(), name);
        }
        let id = self.index.indexes.alloc(Index {
            schema,
            name,
            table,
            ptr: self.ptr(node),
            name_ptr: self.ptr(segment.syntax()),
        });
        self.record_comment(node, Item::Index(id));
        self.add_item(segment.syntax(), Item::Index(id));
        Some(id)
    }

    fn alloc_collation(
        &mut self,
        if_exists: IfExists,
        node: &SyntaxNode,
        path: &ast::Path,
    ) -> Option<CollationId> {
        let segment = path.segment()?;
        let name = self.intern(&segment.text());
        let schema = self.creation_schema(path);
        if self.find_collation_in(schema, name).is_some() {
            if if_exists == IfExists::Ignore {
                return None;
            }
            self.report_duplicate(ObjectKind::Collation, segment.syntax(), name);
        }
        let id = self.index.collations.alloc(Collation {
            schema,
            name,
            ptr: self.ptr(node),
            name_ptr: self.ast_ptr(&segment),
        });
        self.record_comment(node, Item::Collation(id));
        self.add_item(segment.syntax(), Item::Collation(id));
        Some(id)
    }

    fn alloc_table(
        &mut self,
        kind: TableKind,
        persistence: Persistence,
        if_exists: IfExists,
        node: &SyntaxNode,
        path: &ast::Path,
        columns: Columns,
    ) -> Option<TableId> {
        let segment = path.segment()?;
        let name = self.intern(&segment.text());
        let schema = match path.qualifier() {
            None if persistence == Persistence::Temp => self.pg_temp,
            _ => self.creation_schema(path),
        };
        Some(self.alloc_table_in(
            kind,
            if_exists,
            node,
            schema,
            name,
            segment.syntax(),
            columns,
        ))
    }

    fn alloc_table_in(
        &mut self,
        kind: TableKind,
        if_exists: IfExists,
        node: &SyntaxNode,
        schema: Name,
        name: Name,
        name_node: &SyntaxNode,
        columns: Columns,
    ) -> TableId {
        let replaced = self.find_table(Some(schema), name).filter(|table| {
            if_exists == IfExists::Replace && self.index.tables[*table].kind == kind
        });
        if if_exists != IfExists::Ignore && replaced.is_none() {
            if self.relation_exists(schema, name) {
                self.report_duplicate(ObjectKind::Relation, name_node, name);
            } else if self.find_type_in(schema, name).is_some() {
                self.report_duplicate(ObjectKind::Type, name_node, name);
            }
        }
        let id = self.index.tables.alloc(
            Table {
                kind,
                schema,
                name,
                ptr: self.ptr(node),
                name_ptr: self.ptr(name_node),
                columns,
                constraints_complete: true,
                sequence_owner: None,
                of_type: None,
            },
            &self.index.columns,
        );
        self.index
            .set_column_owner(columns.range, ColumnOwner::Table(id));
        if let Some(replaced) = replaced {
            self.drop_table(replaced);
            self.dependencies
                .add_move(Object::Table(replaced), Object::Table(id));
        }
        self.record_comment(node, Item::Table(id));
        self.add_item(name_node, Item::Table(id));
        id
    }

    fn alloc_multirange_type(&mut self, range_id: TypeId, multirange_name: MultirangeName) {
        let range = &self.index.types[range_id];
        let (ptr, name_ptr, range_schema) = (range.ptr, range.name_ptr, range.schema);
        let range_name = self.index.names.resolve(range.name);
        let (schema, name) = match multirange_name {
            MultirangeName::Path(path) => match path.segment() {
                Some(segment) => {
                    let schema = self
                        .lower_schema_qualifier(path.qualifier())
                        .unwrap_or(range_schema);
                    (schema, segment.text())
                }
                None => (
                    range_schema,
                    postgres::make_multirange_type_name(range_name),
                ),
            },
            MultirangeName::String(name) => (range_schema, name),
            MultirangeName::Default => (
                range_schema,
                postgres::make_multirange_type_name(range_name),
            ),
        };
        let name = self.intern(&name);
        self.index.types.alloc(
            Type {
                kind: TypeKind::Multirange,
                schema,
                name,
                ptr,
                name_ptr,
                shell_name_ptr: None,
                columns: None,
                base: None,
                element: None,
                subtype: None,
                range: Some(range_id),
                subscript: None,
                subscript_result: None,
                slice_result: None,
                category: None,
                preferred: false,
            },
            &self.index.columns,
        );
    }

    fn subscripted(&self, ty: Ty, slice: bool) -> Option<Ty> {
        let ty = self.index.base_type(ty)?;
        match ty {
            Ty::Array(element) => Some(if slice { ty } else { element.into() }),
            Ty::Type(id) => {
                let ty = &self.index.types[id];
                if slice {
                    ty.slice_result
                } else {
                    ty.subscript_result
                }
            }
            Ty::Table(_) | Ty::Cte(_) => None,
        }
    }

    fn alloc_type(
        &mut self,
        kind: TypeKind,
        node: &SyntaxNode,
        path: &ast::Path,
        columns: Option<Columns>,
    ) -> Option<TypeId> {
        let segment = path.segment()?;
        let name = self.intern(&segment.text());
        let schema = self.creation_schema(path);
        if self.find_type_in(schema, name).is_some()
            || self.find_table(Some(schema), name).is_some()
        {
            self.report_duplicate(ObjectKind::Type, segment.syntax(), name);
        }
        let id = self.index.types.alloc(
            Type {
                kind,
                schema,
                name,
                ptr: self.ptr(node),
                name_ptr: self.ast_ptr(&segment),
                shell_name_ptr: None,
                columns,
                base: None,
                element: None,
                subtype: None,
                range: None,
                subscript: None,
                subscript_result: None,
                slice_result: None,
                category: None,
                preferred: false,
            },
            &self.index.columns,
        );
        if let Some(columns) = columns {
            self.index
                .set_column_owner(columns.range, ColumnOwner::Type(id));
        }
        self.record_comment(node, Item::Type(id));
        self.add_item(segment.syntax(), Item::Type(id));
        Some(id)
    }

    fn alloc_function(
        &mut self,
        kind: FunctionKind,
        if_exists: IfExists,
        node: &SyntaxNode,
        path: &ast::Path,
        FunctionParams {
            params,
            direct_args,
        }: FunctionParams,
        ret_type: Option<&ast::RetType>,
    ) -> Option<FunctionId> {
        let segment = path.segment()?;
        let name = self.intern(&segment.text());
        let schema = self.creation_schema(path);

        let mut ret = None;
        let mut table_columns = None;
        if let Some(ret_type) = ret_type {
            ret = self.lower_func_type(ret_type.func_type());
            if let Some(list) = ret_type.return_table_arg_list() {
                let start = self.index.columns.next_idx();
                for arg in list.args() {
                    let ty = self.lower_func_type(arg.func_type());
                    if let Some(name) = arg.name() {
                        let column = Column {
                            ty,
                            ..self.column_def(&name)
                        };
                        self.alloc_column_def(column);
                    }
                }
                table_columns = Some(Columns {
                    range: self.index.columns.range_from(start),
                    complete: true,
                });
            }
        }
        let mut replaced = None;
        if let Some(existing) = self.find_function_by(Some(schema), name, |function| {
            self.same_signature((function.kind, function.params), (kind, params))
        }) {
            match if_exists {
                IfExists::Error => {
                    let error = SemaError::DuplicateFunction { name, schema: None };
                    self.push_error(segment.syntax(), error);
                }
                IfExists::Ignore => {}
                IfExists::Replace => {
                    self.drop_function(existing);
                    replaced = Some(existing);
                }
            }
        }

        let columns = match table_columns {
            Some(columns) => Some(columns),
            None => self.out_param_columns(params),
        };
        if ret.is_none()
            && kind != FunctionKind::Procedure
            && let Some(columns) = columns
        {
            ret = match columns.range.len() {
                1 => self.index.columns[columns.range.start()].ty,
                _ => self.builtin_type("record"),
            };
        }

        let id = self.index.functions.alloc(Function {
            kind,
            window: false,
            schema,
            name,
            ptr: self.ptr(node),
            name_ptr: self.ast_ptr(&segment),
            params,
            direct_args,
            ret,
            columns,
        });
        self.index.set_param_function(params, id);
        if let Some(columns) = columns {
            self.index
                .set_column_owner(columns.range, ColumnOwner::Function(id));
        }
        if let Some(replaced) = replaced {
            self.dependencies.add_replacement(replaced, id);
        }
        self.record_comment(node, Item::Function(id));
        self.add_item(segment.syntax(), Item::Function(id));
        Some(id)
    }

    fn alloc_params(&mut self, param_list: Option<ast::ParamList>) -> FunctionParams {
        let start = self.index.params.next_idx();
        let order_by = param_list
            .as_ref()
            .and_then(|list| list.aggregate_order_by());
        let mut direct_args = None;
        for param in param_list.into_iter().flat_map(|list| list.params()) {
            self.alloc_param_def(start, &param);
        }
        if let Some(order_by) = order_by {
            direct_args = Some(self.index.params.range_from(start).len());
            for param in order_by.params() {
                self.alloc_param_def(start, &param);
            }
        }
        FunctionParams {
            params: self.index.params.range_from(start),
            direct_args,
        }
    }

    fn alloc_basetype_param(&mut self, attributes: Option<&ast::AttributeList>) -> IdxRange<Param> {
        let start = self.index.params.next_idx();
        let basetype = attributes
            .into_iter()
            .flat_map(|list| list.attribute_options())
            .find(|option| attribute_name(option).as_deref() == Some("basetype"));
        if let Some(ty) = basetype
            .and_then(|option| option.attribute_value())
            .and_then(|value| value.ty())
        {
            let lowered = self.lower_type(&ty);
            if lowered.is_some() && lowered == self.builtin_type("any") {
                return self.index.params.range_from(start);
            }
            self.index.params.alloc(Param {
                name: None,
                mode: ParamMode::In,
                ty: lowered,
                has_default: false,
                ptr: self.ptr(ty.syntax()),
                name_ptr: None,
            });
        }
        self.index.params.range_from(start)
    }

    fn alloc_param_def(&mut self, start: ParamId, param_node: &ast::Param) {
        let param = self.param_def(param_node);
        if let Some(name) = param.name
            && let Some(param_name) = param_node.name()
            && self.index.params.range_from(start).iter().any(|id| {
                let prev = &self.index.params[id];
                prev.name == Some(name) && params_conflict(prev.mode, param.mode)
            })
        {
            self.push_error(param_name.syntax(), SemaError::DuplicateParam { name });
        }
        let name_ptr = param.name_ptr;
        let id = self.index.params.alloc(param);
        self.record_comment(param_node.syntax(), Item::Param(id));
        if let Some(name_ptr) = name_ptr {
            self.add_item_range(name_ptr.file_range(), Item::Param(id));
        }
    }

    fn alloc_column_def_ref(&mut self, column_ref: &ast::ColumnNameRef, ty: Option<Ty>) {
        let column = Column {
            name: Some(self.intern(&column_ref.text())),
            ptr: self.ptr(column_ref.syntax()),
            ty,
            merge: None,
        };
        self.alloc_column_def(column);
    }

    fn alloc_index_constraint(
        &mut self,
        kind: ConstraintKind,
        table: TableId,
        node: &SyntaxNode,
        name: Option<ast::ConstraintName>,
        using_index: Option<IndexId>,
        addition: Option<&str>,
        label: &str,
    ) -> ConstraintId {
        let schema = self.index.tables[table].schema;
        if let (Some(name), None) = (&name, using_index) {
            let name_id = self.intern(&name.text());
            if self
                .find_constraint(ConstraintOwner::Table(table), name_id)
                .is_none()
                && self.relation_exists(schema, name_id)
            {
                self.report_duplicate(ObjectKind::Relation, name.syntax(), name_id);
            }
        }
        let name = match (name, using_index) {
            (Some(name), _) => ConstraintName::Explicit(name),
            (None, Some(index)) => ConstraintName::Generated(self.index.indexes[index].name),
            (None, None) => {
                let name = postgres::choose_relation_name(
                    self.index.names.resolve(self.index.tables[table].name),
                    addition,
                    label,
                    |name| self.relation_name_exists(schema, name),
                    |name| self.constraint_name_exists(schema, name),
                );
                ConstraintName::Generated(self.intern(&name))
            }
        };
        let constraint_id =
            self.alloc_constraint(kind, ConstraintOwner::Table(table), node, name, using_index);
        if using_index.is_some() {
            return constraint_id;
        }
        let constraint = &self.index.constraints[constraint_id];
        let index = self.index.indexes.alloc(Index {
            schema,
            name: constraint.name,
            table: Some(table),
            ptr: constraint.ptr,
            name_ptr: constraint.name_ptr,
        });
        self.index.constraints.set_index(constraint_id, Some(index));
        constraint_id
    }

    fn report_duplicate_constraint(
        &mut self,
        owner: ConstraintOwner,
        name_node: &SyntaxNode,
        name: Name,
    ) {
        let error = SemaError::DuplicateConstraint {
            name,
            owner: self.constraint_owner_name(owner),
            domain: matches!(owner, ConstraintOwner::Domain(_)),
        };
        self.push_error(name_node, error);
    }

    fn alloc_constraint(
        &mut self,
        kind: ConstraintKind,
        owner: ConstraintOwner,
        node: &SyntaxNode,
        name: ConstraintName,
        index: Option<IndexId>,
    ) -> ConstraintId {
        let (name_node, name_id) = match name {
            ConstraintName::Explicit(name) => {
                let name_id = self.intern(&name.text());
                if self.find_constraint(owner, name_id).is_some() {
                    self.report_duplicate_constraint(owner, name.syntax(), name_id);
                }
                (Some(name), name_id)
            }
            ConstraintName::Generated(name) => (None, name),
        };
        let id = self.index.constraints.alloc(Constraint {
            kind,
            name: name_id,
            owner,
            index,
            ptr: self.ptr(node),
            name_ptr: self.ptr(name_node.as_ref().map_or(node, |name| name.syntax())),
        });
        self.record_comment(node, Item::Constraint(id));
        if let Some(name) = name_node {
            self.add_item(name.syntax(), Item::Constraint(id));
        }
        id
    }

    fn alloc_column_def(&mut self, column: Column) -> ColumnId {
        let id = self.index.columns.alloc(column);
        self.add_item_range(column.ptr.file_range(), Item::Column(id));
        id
    }

    fn alloc_column_defs(&mut self, columns: Vec<Column>) -> IdxRange<Column> {
        let range = self.index.columns.alloc_many(columns);
        for id in range {
            let ptr = self.index.columns[id].ptr;
            self.add_item_range(ptr.file_range(), Item::Column(id));
        }
        range
    }

    fn unknown_columns(&mut self) -> Columns {
        Columns {
            range: self.index.columns.alloc_many([]),
            complete: false,
        }
    }

    fn alloc_query(&mut self, kind: QueryKind, node: &SyntaxNode) -> QueryId {
        self.alloc_query_ptr(kind, self.ptr(node))
    }

    fn alloc_query_ptr(&mut self, kind: QueryKind, ptr: InIrFile<SyntaxNodePtr>) -> QueryId {
        let columns = Columns {
            range: self.index.columns.alloc_many([]),
            complete: true,
        };
        self.index.queries.alloc(Query {
            kind,
            ptr,
            parent: self.scopes.last().map(|scope| scope.query),
            sources: vec![],
            columns,
        })
    }

    fn add_ref(
        &mut self,
        name: Name,
        kind: ReferenceKind,
        node: &SyntaxNode,
        resolution: Resolution,
    ) -> RefId {
        self.add_ref_at(name, kind, node.text_range(), resolution)
    }

    fn add_ref_at(
        &mut self,
        name: Name,
        kind: ReferenceKind,
        range: TextRange,
        resolution: Resolution,
    ) -> RefId {
        let range = InIrFile::new(self.file_id, range);
        let id = self.index.references.alloc(Reference {
            name,
            kind,
            range,
            resolution,
            scope: self.scopes.last().map(|scope| scope.scope),
        });
        self.add_item_range(range, Item::Reference(id));
        id
    }

    fn record_comment(&mut self, node: &SyntaxNode, item: Item) {
        if let Some(comment) = preceding_comment(node) {
            self.index.comments.insert(item, comment.into_boxed_str());
        }
    }

    fn copy_comment(&mut self, from: Item, to: Item) {
        match self.index.comment(from).map(Box::from) {
            Some(comment) => {
                self.index.comments.insert(to, comment);
            }
            None => {
                self.index.comments.remove(&to);
            }
        }
    }

    fn add_item(&mut self, node: &SyntaxNode, item: Item) {
        self.add_item_range(self.ptr(node).file_range(), item);
    }

    fn lower_case_keywords(&mut self, case: &ast::CaseExpr) {
        let Some(case_token) = case.case_token() else {
            return;
        };
        let id = self.index.cases.alloc(Case {
            case_range: InIrFile::new(self.file_id, case_token.text_range()),
        });
        let when_tokens = case
            .when_clause_list()
            .into_iter()
            .flat_map(|list| list.when_clauses())
            .filter_map(|when| when.when_token());
        let else_token = case.else_clause().and_then(|clause| clause.else_token());
        for token in when_tokens.chain(else_token).chain(case.end_token()) {
            self.add_token_item(&token, Item::CaseKeyword(id));
        }
    }

    fn add_token_item(&mut self, token: &SyntaxToken, item: Item) {
        self.add_item_range(InIrFile::new(self.file_id, token.text_range()), item);
    }

    fn record_search_path(&mut self, offset: TextSize) {
        let range = self
            .index
            .range(InIrFile::new(self.file_id, TextRange::empty(offset)));
        if self.index.search_paths.last().is_some_and(|change| {
            change.file_id == range.file_id && *change.value.path == *self.search_path
        }) {
            return;
        }
        self.index
            .search_paths
            .push(range.map(|range| SearchPathChange {
                start: range.start(),
                path: self.search_path.as_slice().into(),
            }));
    }

    fn add_item_range(&mut self, range: IrFileRange, item: Item) {
        let range = self.index.range(range);
        self.index
            .items
            .push(range.map(|range| ItemRange { range, item }));
    }

    fn report_duplicate(&mut self, object: ObjectKind, node: &SyntaxNode, name: Name) {
        self.push_error(node, SemaError::Duplicate { object, name });
    }

    fn push_error(&mut self, node: &SyntaxNode, error: SemaError) {
        self.push_error_ptr(self.ptr(node), error);
    }

    fn push_error_ptr(&mut self, ptr: InIrFile<SyntaxNodePtr>, error: SemaError) {
        let range = self.index.range(ptr.file_range());
        self.index.errors.push(range, error);
    }

    fn ptr(&self, node: &SyntaxNode) -> InIrFile<SyntaxNodePtr> {
        InIrFile::new(self.file_id, SyntaxNodePtr::new(node))
    }

    fn ast_ptr<N: AstNode>(&self, node: &N) -> InIrFile<AstPtr<N>> {
        InIrFile::new(self.file_id, AstPtr::new(node))
    }
}

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
    fn allows_aggregates(self) -> bool {
        matches!(
            self,
            ExprContext::Unrestricted | ExprContext::Having | ExprContext::WindowDefinition
        )
    }

    fn allows_windows(self) -> bool {
        self == ExprContext::Unrestricted
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum AggregateCallKind {
    Aggregate,
    Window,
}

#[derive(Clone, Copy)]
struct AggregateCall {
    kind: AggregateCallKind,
    depth: usize,
    ptr: InIrFile<SyntaxNodePtr>,
}

#[derive(Clone, Copy)]
enum ColumnListOwner {
    Cte(Name),
    Table(Option<Name>),
    Join(Option<Name>),
    View,
    CreateAs,
}

#[derive(Clone, Copy)]
enum SubscriptOption {
    Unset,
    Untyped,
    Handler(Subscript),
}

impl SubscriptOption {
    fn subscript(self) -> Option<Subscript> {
        match self {
            SubscriptOption::Handler(subscript) => Some(subscript),
            SubscriptOption::Unset | SubscriptOption::Untyped => None,
        }
    }
}

struct BaseTypeOptions {
    element: Option<Ty>,
    subscript: SubscriptOption,
    category: TypeCategory,
    preferred: bool,
}

fn params_conflict(a: ParamMode, b: ParamMode) -> bool {
    let input_only = |mode| matches!(mode, ParamMode::In | ParamMode::Variadic);
    !(input_only(a) && b == ParamMode::Out || a == ParamMode::Out && input_only(b))
}

fn split_ident(text: &str) -> Option<IdentPart<'_>> {
    let Some(quoted) = text.strip_prefix('"') else {
        let end = text.find('.').unwrap_or(text.len());
        return Some(IdentPart {
            ident: Cow::Borrowed(&text[..end]),
            rest: &text[end..],
        });
    };
    let mut chars = quoted.char_indices();
    while let Some((index, char)) = chars.next() {
        if char != '"' {
            continue;
        }
        if quoted[index + 1..].starts_with('"') {
            chars.next();
            continue;
        }
        let ident = &quoted[..index];
        let ident = if ident.contains(r#""""#) {
            Cow::Owned(ident.replace(r#""""#, r#"""#))
        } else {
            Cow::Borrowed(ident)
        };
        return Some(IdentPart {
            ident,
            rest: &quoted[index + 1..],
        });
    }
    None
}

fn regclass_literal(expr: &ast::Expr) -> Option<ast::Literal> {
    if let Some(literal) = expr.as_literal() {
        return Some(literal);
    }
    if let ast::Expr::CastExpr(cast) = expr
        && cast.ty().as_ref().is_some_and(is_regclass)
    {
        return cast.expr()?.as_literal();
    }
    None
}

fn is_unknown_literal(arg: &ast::FuncArgExpr) -> bool {
    arg.expr()
        .is_some_and(|expr| is_unknown_literal_expr(&expr))
}

fn expr_arg_type(expr: Option<&ast::Expr>, ty: Option<Ty>) -> ArgType {
    match ty {
        _ if expr.is_some_and(is_unknown_literal_expr) => ArgType::UnknownLiteral,
        Some(ty) => ArgType::Known(ty),
        None => ArgType::Untyped,
    }
}

fn is_unknown_literal_expr(expr: &ast::Expr) -> bool {
    let Some(literal) = expr.unparen().as_literal() else {
        return false;
    };
    matches!(
        literal.kind(),
        Some(
            ast::LitKind::String(_)
                | ast::LitKind::DollarQuotedString(_)
                | ast::LitKind::EscString(_)
                | ast::LitKind::UnicodeEscString(_)
                | ast::LitKind::Null(_)
        )
    )
}

fn expanded_unnest(call: &ast::CallExpr) -> Option<ast::NameRef> {
    let name_ref = call.expr()?.as_name_ref()?;
    let arg_list = call.arg_list()?;
    if call.over_clause().is_some()
        || call.filter_clause().is_some()
        || call.within_clause().is_some()
        || arg_list.star_token().is_some()
        || matches!(
            arg_list.all_or_distinct(),
            Some(ast::AllOrDistinct::Distinct(_))
        )
    {
        return None;
    }
    let mut count = 0;
    for arg in arg_list.args() {
        if arg.variadic_token().is_some()
            || arg.order_by_clause().is_some()
            || matches!(arg.func_arg_expr(), Some(ast::FuncArgExpr::NamedArg(_)))
        {
            return None;
        }
        count += 1;
    }
    (count > 1 && name_ref.text() == "unnest").then_some(name_ref)
}

fn is_regclass(ty: &ast::Type) -> bool {
    let Some(path) = ty.path_ref() else {
        return false;
    };
    path.segment()
        .is_some_and(|segment| segment.text() == "regclass")
        && path.qualifier().is_none_or(|qualifier| {
            qualifier
                .unqualified_segment()
                .is_some_and(|segment| segment.text() == "pg_catalog")
        })
}

enum ConstraintName {
    Explicit(ast::ConstraintName),
    Generated(Name),
}

pub(crate) fn is_public_role(name_ref: &ast::RoleNameRef) -> bool {
    name_ref.text() == "public"
}

fn attribute_name(option: &ast::AttributeOption) -> Option<String> {
    let mut name = option.name()?.text();
    name.make_ascii_lowercase();
    Some(name)
}

fn graph_element_live(
    elements: &GraphElementIndex,
    dropped_elements: &FxHashSet<GraphElementId>,
    dropped_tables: &FxHashSet<TableId>,
    id: GraphElementId,
) -> bool {
    let element = &elements[id];
    !dropped_elements.contains(&id)
        && element
            .table
            .is_none_or(|table| !dropped_tables.contains(&table))
        && [element.source, element.destination]
            .into_iter()
            .flatten()
            .all(|vertex| graph_element_live(elements, dropped_elements, dropped_tables, vertex))
}

fn visit_path_primaries(pattern: &ast::PathPattern, f: &mut impl FnMut(&ast::PathPrimary)) {
    for primary in pattern
        .path_factors()
        .filter_map(|factor| factor.path_primary())
    {
        f(&primary);
        if let ast::PathPrimary::ParenGraphPattern(paren) = &primary
            && let Some(inner) = paren.path_pattern()
        {
            visit_path_primaries(&inner, f);
        }
    }
}

fn graph_element_kind(vertex_or_edge: Option<&ast::VertexOrEdge>) -> GraphElementKind {
    match vertex_or_edge.and_then(|vertex_or_edge| vertex_or_edge.kind()) {
        Some(ast::VertexOrEdgeKind::Edge) => GraphElementKind::Edge,
        Some(ast::VertexOrEdgeKind::Vertex) | None => GraphElementKind::Vertex,
    }
}

fn set_object_schema(index: &mut IndexData, object: Object, schema: Name) -> bool {
    match object {
        Object::Table(table) => index.tables.set_schema(table, schema),
        Object::Index(idx) => index.indexes.set_schema(idx, schema),
        Object::Type(ty) => index.types.set_schema(ty, schema),
        Object::Function(function) => index.functions.set_schema(function, schema),
        Object::Operator(operator) => index.operators.set_schema(operator, schema),
        Object::Collation(collation) => index.collations.set_schema(collation, schema),
        Object::Conversion(conversion) => index.conversions.set_schema(conversion, schema),
        Object::OperatorFamily(family) => index.operator_families.set_schema(family, schema),
        Object::OperatorClass(class) => index.operator_classes.set_schema(class, schema),
        Object::Statistics(statistics) => index.statistics.set_schema(statistics, schema),
        Object::Extension(extension) => index.extensions.set_schema(extension, schema),
        Object::TextSearch(text_search) => index.text_search.set_schema(text_search, schema),
        Object::Trigger(_)
        | Object::EventTrigger(_)
        | Object::Constraint(_)
        | Object::Policy(_)
        | Object::Rule(_)
        | Object::Cast(_)
        | Object::Transform(_)
        | Object::AccessMethod(_)
        | Object::Language(_)
        | Object::ForeignDataWrapper(_)
        | Object::ColumnExpr(_)
        | Object::OperatorFamilyMember(_)
        | Object::Schema(_)
        | Object::Server(_)
        | Object::Publication(_) => false,
    }
}

fn rename_schema_objects(index: &mut IndexData, from: Name, to: Name) {
    index.tables.rename_schema(from, to);
    index.extensions.rename_schema(from, to);
    index.indexes.rename_schema(from, to);
    index.types.rename_schema(from, to);
    index.functions.rename_schema(from, to);
    index.operators.rename_schema(from, to);
    index.collations.rename_schema(from, to);
    index.conversions.rename_schema(from, to);
    index.operator_families.rename_schema(from, to);
    index.operator_classes.rename_schema(from, to);
    index.statistics.rename_schema(from, to);
    index.text_search.rename_schema(from, to);
}

fn attribute_path(option: &ast::AttributeOption) -> Option<ast::PathRef> {
    option.attribute_value()?.ty()?.path_ref()
}

fn same_access_method(a: Option<AccessMethodId>, b: Option<AccessMethodId>) -> bool {
    match (a, b) {
        (Some(a), Some(b)) => a == b,
        (None, _) | (_, None) => true,
    }
}

fn is_search_path(param: Option<ast::ConfigParameterRef>) -> bool {
    param
        .and_then(|param| param.path_ref())
        .and_then(|path| path.unqualified_segment())
        .is_some_and(|segment| segment.text() == "search_path")
}

fn is_temp_schema_alias(schema: &str) -> bool {
    schema
        .strip_prefix("pg_temp_")
        .is_some_and(|n| !n.is_empty() && n.bytes().all(|b| b.is_ascii_digit()))
}

fn sql_value_function_name(function: ast::SqlValueFunction) -> Option<&'static str> {
    match function {
        ast::SqlValueFunction::CurrentCatalog => Some("current_database"),
        ast::SqlValueFunction::CurrentRole
        | ast::SqlValueFunction::CurrentUser
        | ast::SqlValueFunction::User => Some("current_user"),
        ast::SqlValueFunction::CurrentSchema => Some("current_schema"),
        ast::SqlValueFunction::SessionUser => Some("session_user"),
        ast::SqlValueFunction::SystemUser => Some("system_user"),
        ast::SqlValueFunction::CurrentTimestamp => Some("now"),
        ast::SqlValueFunction::CurrentDate
        | ast::SqlValueFunction::CurrentTime
        | ast::SqlValueFunction::Localtime
        | ast::SqlValueFunction::Localtimestamp => None,
    }
}

fn nth_idx<T>(index: usize) -> la_arena::Idx<T> {
    la_arena::Idx::from_raw(la_arena::RawIdx::from_u32(u32::try_from(index).unwrap()))
}

fn member_number(literal: Option<&ast::Literal>) -> Option<u16> {
    u16::try_from(literal?.integer_value()?).ok()
}

fn row_width(expr: Option<&ast::Expr>) -> Option<usize> {
    match expr {
        Some(ast::Expr::TupleExpr(tuple)) => Some(tuple.exprs().count()),
        _ => None,
    }
}

fn sublink_width(op: Option<&ast::BinOp>, lhs: Option<&ast::Expr>) -> Option<usize> {
    match op? {
        ast::BinOp::In(_) | ast::BinOp::NotIn(_) => Some(row_width(lhs).unwrap_or(1)),
        ast::BinOp::Caret(_)
        | ast::BinOp::CustomOp(_)
        | ast::BinOp::Eq(_)
        | ast::BinOp::Gteq(_)
        | ast::BinOp::Ilike(_)
        | ast::BinOp::LAngle(_)
        | ast::BinOp::Like(_)
        | ast::BinOp::Lteq(_)
        | ast::BinOp::Minus(_)
        | ast::BinOp::Neq(_)
        | ast::BinOp::Neqb(_)
        | ast::BinOp::NotIlike(_)
        | ast::BinOp::NotLike(_)
        | ast::BinOp::NotSimilarTo(_)
        | ast::BinOp::OperatorCall(_)
        | ast::BinOp::Percent(_)
        | ast::BinOp::Plus(_)
        | ast::BinOp::RAngle(_)
        | ast::BinOp::SimilarTo(_)
        | ast::BinOp::Slash(_)
        | ast::BinOp::Star(_) => row_width(lhs),
        ast::BinOp::And(_)
        | ast::BinOp::AtTimeZone(_)
        | ast::BinOp::ColonColon(_)
        | ast::BinOp::ColonEq(_)
        | ast::BinOp::Escape(_)
        | ast::BinOp::FatArrow(_)
        | ast::BinOp::Is(_)
        | ast::BinOp::IsDistinctFrom(_)
        | ast::BinOp::IsNot(_)
        | ast::BinOp::IsNotDistinctFrom(_)
        | ast::BinOp::Or(_)
        | ast::BinOp::Overlaps(_) => None,
    }
}

fn is_comparison(op: &ast::BinOp) -> bool {
    match op {
        ast::BinOp::Eq(_)
        | ast::BinOp::Neq(_)
        | ast::BinOp::Neqb(_)
        | ast::BinOp::LAngle(_)
        | ast::BinOp::RAngle(_)
        | ast::BinOp::Lteq(_)
        | ast::BinOp::Gteq(_)
        | ast::BinOp::IsDistinctFrom(_)
        | ast::BinOp::IsNotDistinctFrom(_)
        | ast::BinOp::In(_)
        | ast::BinOp::NotIn(_) => true,
        ast::BinOp::And(_)
        | ast::BinOp::AtTimeZone(_)
        | ast::BinOp::Caret(_)
        | ast::BinOp::ColonColon(_)
        | ast::BinOp::ColonEq(_)
        | ast::BinOp::CustomOp(_)
        | ast::BinOp::Escape(_)
        | ast::BinOp::FatArrow(_)
        | ast::BinOp::Ilike(_)
        | ast::BinOp::Is(_)
        | ast::BinOp::IsNot(_)
        | ast::BinOp::Like(_)
        | ast::BinOp::Minus(_)
        | ast::BinOp::NotIlike(_)
        | ast::BinOp::NotLike(_)
        | ast::BinOp::NotSimilarTo(_)
        | ast::BinOp::OperatorCall(_)
        | ast::BinOp::Or(_)
        | ast::BinOp::Overlaps(_)
        | ast::BinOp::Percent(_)
        | ast::BinOp::Plus(_)
        | ast::BinOp::SimilarTo(_)
        | ast::BinOp::Slash(_)
        | ast::BinOp::Star(_) => false,
    }
}

fn subscript_results(
    ty: TypeId,
    element: Option<Ty>,
    subscript: Option<Subscript>,
) -> SubscriptResults {
    match subscript {
        Some(Subscript::Array) => SubscriptResults {
            result: element,
            slice: element.map(Ty::array),
        },
        Some(Subscript::RawArray) => SubscriptResults {
            result: element,
            slice: Some(Ty::Type(ty)),
        },
        None => SubscriptResults {
            result: None,
            slice: None,
        },
    }
}

fn call_field_base(base: &ast::Expr) -> Option<ast::CallExpr> {
    if let ast::Expr::CallExpr(call) = base.unparen() {
        Some(call)
    } else {
        None
    }
}

fn single_source_expr(source: &ast::SelectVariant) -> Option<ast::Expr> {
    match source {
        ast::SelectVariant::Values(values) => values.row_list()?.rows().next()?.exprs().next(),
        ast::SelectVariant::Select(select) => select
            .select_clause()?
            .target_list()?
            .targets()
            .next()?
            .expr(),
        ast::SelectVariant::SelectInto(_)
        | ast::SelectVariant::ParenSelect(_)
        | ast::SelectVariant::CompoundSelect(_)
        | ast::SelectVariant::Table(_) => None,
    }
}

fn like_includes_indexes(like: &ast::LikeClause) -> bool {
    let mut includes = false;
    for option in like.like_options() {
        let (property, including) = match option {
            ast::LikeOption::IncludingProperty(property) => (property.table_property(), true),
            ast::LikeOption::ExcludingProperty(property) => (property.table_property(), false),
        };
        match property {
            Some(ast::TableProperty::PropertyAll(_) | ast::TableProperty::PropertyIndexes(_)) => {
                includes = including;
            }
            Some(
                ast::TableProperty::PropertyComments(_)
                | ast::TableProperty::PropertyCompression(_)
                | ast::TableProperty::PropertyConstraints(_)
                | ast::TableProperty::PropertyDefaults(_)
                | ast::TableProperty::PropertyGenerated(_)
                | ast::TableProperty::PropertyIdentity(_)
                | ast::TableProperty::PropertyStatistics(_)
                | ast::TableProperty::PropertyStorage(_),
            )
            | None => {}
        }
    }
    includes
}
