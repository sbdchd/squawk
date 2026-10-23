use std::sync::LazyLock;

use rustc_hash::FxHashMap;
use squawk_syntax::ast;

use crate::arena::LayeredArena;
use crate::builtins::BUILTINS_SQL;
use crate::constraint_index::ConstraintIndex;
use crate::extension_index::ExtensionIndex;
use crate::file::{FileId, InFile};
use crate::index_index::IndexIndex;
use crate::interner::Interner;
use crate::keyed_index::{
    AccessMethodIndex, CastIndex, DatabaseIndex, EventTriggerIndex, ForeignDataWrapperIndex,
    GraphElementIndex, GraphLabelIndex, GraphPropertyIndex, LanguageIndex,
    OperatorFamilyMemberIndex, PolicyIndex, PublicationIndex, RoleIndex, RuleIndex, SchemaIndex,
    ServerIndex, SubscriptionIndex, TablespaceIndex, TriggerIndex,
};
use crate::lower::Lowerer;
use crate::qualified_index::{
    CollationIndex, ConversionIndex, FunctionIndex, OperatorClassIndex, OperatorFamilyIndex,
    OperatorIndex, TextSearchIndex,
};
use crate::semantic_index::{Errors, IndexData, SemanticIndex};
use crate::statistics_index::StatisticsIndex;
use crate::table_index::TableIndex;
use crate::type_index::TypeIndex;

pub(crate) fn builtins() -> &'static SemanticIndex {
    static BUILTINS: LazyLock<SemanticIndex> = LazyLock::new(|| {
        let parse = ast::SourceFile::parse(BUILTINS_SQL);
        let mut builder = SemanticIndexBuilder::with_index(IndexData::empty());
        builder.add_file(InFile::new(FileId::Builtins, &parse.tree()));
        builder.build()
    });
    &BUILTINS
}

pub(crate) struct SemanticIndexBuilder {
    lowerer: Lowerer,
}

impl SemanticIndexBuilder {
    pub(crate) fn new() -> SemanticIndexBuilder {
        SemanticIndexBuilder::with_index(IndexData::on_top_of(builtins()))
    }

    fn with_index(index: IndexData) -> SemanticIndexBuilder {
        SemanticIndexBuilder {
            lowerer: Lowerer::new(index),
        }
    }

    pub(crate) fn add_file(&mut self, file: InFile<&ast::SourceFile>) -> &mut SemanticIndexBuilder {
        self.lowerer.lower_file(file);
        self
    }

    pub(crate) fn build(self) -> SemanticIndex {
        let mut index = self.lowerer.finish();
        index
            .items
            .sort_by_key(|item| (item.file_id, item.value.range.start()));
        index
            .search_paths
            .sort_by_key(|change| (change.file_id, change.value.start));
        shrink_to_fit(&mut index);
        SemanticIndex::from(index)
    }
}

impl IndexData {
    fn empty() -> IndexData {
        IndexData {
            names: Interner::default(),
            tables: TableIndex::default(),
            types: TypeIndex::default(),
            functions: FunctionIndex::default(),
            params: LayeredArena::default(),
            ctes: LayeredArena::default(),
            queries: LayeredArena::default(),
            scopes: LayeredArena::default(),
            sources: LayeredArena::default(),
            columns: LayeredArena::default(),
            column_owners: FxHashMap::default(),
            merges: LayeredArena::default(),
            references: LayeredArena::default(),
            bodies: LayeredArena::default(),
            windows: LayeredArena::default(),
            json_paths: LayeredArena::default(),
            indexes: IndexIndex::default(),
            constraints: ConstraintIndex::default(),
            triggers: TriggerIndex::default(),
            rules: RuleIndex::default(),
            roles: RoleIndex::default(),
            policies: PolicyIndex::default(),
            access_methods: AccessMethodIndex::default(),
            event_triggers: EventTriggerIndex::default(),
            languages: LanguageIndex::default(),
            foreign_data_wrappers: ForeignDataWrapperIndex::default(),
            servers: ServerIndex::default(),
            conversions: ConversionIndex::default(),
            operator_families: OperatorFamilyIndex::default(),
            operator_family_members: OperatorFamilyMemberIndex::default(),
            operator_classes: OperatorClassIndex::default(),
            statistics: StatisticsIndex::default(),
            text_search: TextSearchIndex::default(),
            publications: PublicationIndex::default(),
            subscriptions: SubscriptionIndex::default(),
            graph_elements: GraphElementIndex::default(),
            graph_labels: GraphLabelIndex::default(),
            enum_labels: LayeredArena::default(),
            graph_properties: GraphPropertyIndex::default(),
            collations: CollationIndex::default(),
            schemas: SchemaIndex::default(),
            tablespaces: TablespaceIndex::default(),
            databases: DatabaseIndex::default(),
            extensions: ExtensionIndex::default(),
            operators: OperatorIndex::default(),
            casts: CastIndex::default(),
            transforms: LayeredArena::default(),
            savepoints: LayeredArena::default(),
            cursors: LayeredArena::default(),
            transactions: LayeredArena::default(),
            prepared_transactions: LayeredArena::default(),
            prepared_statements: LayeredArena::default(),
            channels: LayeredArena::default(),
            cases: LayeredArena::default(),
            stars: LayeredArena::default(),
            star_columns: LayeredArena::default(),
            builtins: None,
            items: vec![],
            search_paths: vec![],
            errors: Errors::default(),
            renames: FxHashMap::default(),
            param_functions: FxHashMap::default(),
            comments: FxHashMap::default(),
            type_enum_labels: FxHashMap::default(),
        }
    }

    fn on_top_of(base: &'static SemanticIndex) -> IndexData {
        IndexData {
            names: Interner::on_top_of(&base.names),
            tables: TableIndex::on_top_of(&base.tables),
            types: TypeIndex::on_top_of(&base.types),
            functions: FunctionIndex::on_top_of(&base.functions),
            params: LayeredArena::on_top_of(&base.params),
            ctes: LayeredArena::on_top_of(&base.ctes),
            queries: LayeredArena::on_top_of(&base.queries),
            scopes: LayeredArena::on_top_of(&base.scopes),
            sources: LayeredArena::on_top_of(&base.sources),
            columns: LayeredArena::on_top_of(&base.columns),
            column_owners: FxHashMap::default(),
            merges: LayeredArena::on_top_of(&base.merges),
            references: LayeredArena::on_top_of(&base.references),
            bodies: LayeredArena::on_top_of(&base.bodies),
            windows: LayeredArena::on_top_of(&base.windows),
            json_paths: LayeredArena::on_top_of(&base.json_paths),
            indexes: IndexIndex::on_top_of(&base.indexes),
            constraints: ConstraintIndex::on_top_of(&base.constraints),
            triggers: TriggerIndex::on_top_of(&base.triggers),
            rules: RuleIndex::on_top_of(&base.rules),
            roles: RoleIndex::on_top_of(&base.roles),
            policies: PolicyIndex::on_top_of(&base.policies),
            access_methods: AccessMethodIndex::on_top_of(&base.access_methods),
            event_triggers: EventTriggerIndex::on_top_of(&base.event_triggers),
            languages: LanguageIndex::on_top_of(&base.languages),
            foreign_data_wrappers: ForeignDataWrapperIndex::on_top_of(&base.foreign_data_wrappers),
            servers: ServerIndex::on_top_of(&base.servers),
            conversions: ConversionIndex::on_top_of(&base.conversions),
            operator_families: OperatorFamilyIndex::on_top_of(&base.operator_families),
            operator_family_members: OperatorFamilyMemberIndex::on_top_of(
                &base.operator_family_members,
            ),
            operator_classes: OperatorClassIndex::on_top_of(&base.operator_classes),
            statistics: StatisticsIndex::on_top_of(&base.statistics),
            text_search: TextSearchIndex::on_top_of(&base.text_search),
            publications: PublicationIndex::on_top_of(&base.publications),
            subscriptions: SubscriptionIndex::on_top_of(&base.subscriptions),
            graph_elements: GraphElementIndex::on_top_of(&base.graph_elements),
            graph_labels: GraphLabelIndex::on_top_of(&base.graph_labels),
            enum_labels: LayeredArena::on_top_of(&base.enum_labels),
            graph_properties: GraphPropertyIndex::on_top_of(&base.graph_properties),
            collations: CollationIndex::on_top_of(&base.collations),
            schemas: SchemaIndex::on_top_of(&base.schemas),
            tablespaces: TablespaceIndex::on_top_of(&base.tablespaces),
            databases: DatabaseIndex::on_top_of(&base.databases),
            extensions: ExtensionIndex::on_top_of(&base.extensions),
            operators: OperatorIndex::on_top_of(&base.operators),
            casts: CastIndex::on_top_of(&base.casts),
            transforms: LayeredArena::on_top_of(&base.transforms),
            savepoints: LayeredArena::on_top_of(&base.savepoints),
            cursors: LayeredArena::on_top_of(&base.cursors),
            transactions: LayeredArena::on_top_of(&base.transactions),
            prepared_transactions: LayeredArena::on_top_of(&base.prepared_transactions),
            prepared_statements: LayeredArena::on_top_of(&base.prepared_statements),
            channels: LayeredArena::on_top_of(&base.channels),
            cases: LayeredArena::on_top_of(&base.cases),
            stars: LayeredArena::on_top_of(&base.stars),
            star_columns: LayeredArena::on_top_of(&base.star_columns),
            builtins: Some(base),
            items: vec![],
            search_paths: vec![],
            errors: Errors::default(),
            renames: FxHashMap::default(),
            param_functions: FxHashMap::default(),
            comments: FxHashMap::default(),
            type_enum_labels: FxHashMap::default(),
        }
    }
}

fn shrink_to_fit(index: &mut IndexData) {
    let IndexData {
        names,
        tables,
        types,
        functions,
        params,
        ctes,
        queries,
        scopes,
        sources,
        columns,
        column_owners,
        merges,
        references,
        bodies,
        windows,
        json_paths,
        indexes,
        constraints,
        triggers,
        rules,
        roles,
        policies,
        access_methods,
        event_triggers,
        languages,
        foreign_data_wrappers,
        servers,
        conversions,
        operator_families,
        operator_family_members,
        operator_classes,
        statistics,
        text_search,
        publications,
        subscriptions,
        graph_elements,
        graph_labels,
        enum_labels,
        graph_properties,
        collations,
        schemas,
        tablespaces,
        databases,
        extensions,
        operators,
        casts,
        transforms,
        savepoints,
        cursors,
        transactions,
        prepared_transactions,
        prepared_statements,
        channels,
        cases,
        stars,
        star_columns,
        builtins: _,
        items,
        search_paths,
        errors,
        renames,
        param_functions,
        comments,
        type_enum_labels,
    } = index;
    names.shrink_to_fit();
    tables.shrink_to_fit();
    types.shrink_to_fit();
    functions.shrink_to_fit();
    params.shrink_to_fit();
    ctes.shrink_to_fit();
    for query in queries.iter_local_mut() {
        query.sources.shrink_to_fit();
    }
    queries.shrink_to_fit();
    for scope in scopes.iter_local_mut() {
        scope.hidden.shrink_to_fit();
    }
    scopes.shrink_to_fit();
    sources.shrink_to_fit();
    columns.shrink_to_fit();
    column_owners.shrink_to_fit();
    merges.shrink_to_fit();
    references.shrink_to_fit();
    bodies.shrink_to_fit();
    windows.shrink_to_fit();
    json_paths.shrink_to_fit();
    indexes.shrink_to_fit();
    constraints.shrink_to_fit();
    triggers.shrink_to_fit();
    rules.shrink_to_fit();
    roles.shrink_to_fit();
    policies.shrink_to_fit();
    access_methods.shrink_to_fit();
    event_triggers.shrink_to_fit();
    languages.shrink_to_fit();
    foreign_data_wrappers.shrink_to_fit();
    servers.shrink_to_fit();
    conversions.shrink_to_fit();
    operator_families.shrink_to_fit();
    operator_family_members.shrink_to_fit();
    operator_classes.shrink_to_fit();
    statistics.shrink_to_fit();
    text_search.shrink_to_fit();
    publications.shrink_to_fit();
    subscriptions.shrink_to_fit();
    graph_elements.shrink_to_fit();
    graph_labels.shrink_to_fit();
    enum_labels.shrink_to_fit();
    graph_properties.shrink_to_fit();
    collations.shrink_to_fit();
    schemas.shrink_to_fit();
    tablespaces.shrink_to_fit();
    databases.shrink_to_fit();
    extensions.shrink_to_fit();
    operators.shrink_to_fit();
    casts.shrink_to_fit();
    transforms.shrink_to_fit();
    savepoints.shrink_to_fit();
    cursors.shrink_to_fit();
    transactions.shrink_to_fit();
    prepared_transactions.shrink_to_fit();
    prepared_statements.shrink_to_fit();
    channels.shrink_to_fit();
    cases.shrink_to_fit();
    stars.shrink_to_fit();
    star_columns.shrink_to_fit();
    items.shrink_to_fit();
    search_paths.shrink_to_fit();
    errors.shrink_to_fit();
    for versions in renames.values_mut() {
        versions.shrink_to_fit();
    }
    renames.shrink_to_fit();
    param_functions.shrink_to_fit();
    comments.shrink_to_fit();
    for labels in type_enum_labels.values_mut() {
        labels.shrink_to_fit();
    }
    type_enum_labels.shrink_to_fit();
}
