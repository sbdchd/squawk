/// Loosely based on TypeScript's binder
/// see: typescript-go/internal/binder/binder.go
use la_arena::Arena;
use rowan::{TextRange, TextSize};
use smallvec::SmallVec;
use squawk_syntax::{SyntaxNodePtr, ast, ast::AstNode};

use crate::literals::literal_string_value;
use crate::name::schema_and_func_name;
use crate::scope::Scope;
use crate::symbols::{Name, Schema, Symbol, SymbolKind};

type Schemas = SmallVec<[Schema; 3]>;

pub(crate) struct ResolvedSchemas {
    list: Schemas,
    unqualified: bool,
}

impl ResolvedSchemas {
    pub(crate) fn list(&self) -> &[Schema] {
        &self.list
    }

    pub(crate) fn unqualified(&self) -> bool {
        self.unqualified
    }

    pub(crate) fn from_single(schema: Schema) -> Self {
        Self {
            list: Schemas::from_iter([schema]),
            unqualified: false,
        }
    }
}

#[derive(Clone, PartialEq)]
struct SearchPathChange {
    position: TextSize,
    search_path: Vec<Schema>,
}

#[derive(Clone, PartialEq)]
pub(crate) struct Binder {
    scope: Scope,
    symbols: Arena<Symbol>,
    search_path_changes: Vec<SearchPathChange>,
    // When binding objects nested inside `create schema ...`, they default to
    // the schema being created instead of the current search path.
    default_schema_override: Option<Schema>,
    // If we have a `create schema foo` command then commands nested inside that
    // get `foo` for their schema.
    schema_regions: Vec<(TextRange, Schema)>,
}

impl Binder {
    fn new() -> Self {
        Binder {
            scope: Scope::default(),
            symbols: Arena::new(),
            search_path_changes: vec![SearchPathChange {
                position: TextSize::from(0),
                search_path: vec![
                    Schema::new("public"),
                    Schema::new("pg_temp"),
                    Schema::new("pg_catalog"),
                ],
            }],
            default_schema_override: None,
            schema_regions: vec![],
        }
    }

    pub(crate) fn lookup(&self, name: &Name, kind: SymbolKind) -> Option<SyntaxNodePtr> {
        let symbols = self.scope.get(name)?;
        let symbol_id = symbols.iter().copied().find(|id| {
            let symbol = &self.symbols[*id];
            symbol.kind == kind
        })?;
        Some(self.symbols[symbol_id].ptr)
    }

    pub(crate) fn resolved_schemas(
        &self,
        position: TextSize,
        schema: Option<&Schema>,
    ) -> ResolvedSchemas {
        let unqualified = schema.is_none();
        let list = match schema {
            Some(s) => Schemas::from_iter([s.clone()]),
            None => self.resolution_search_path(position),
        };
        ResolvedSchemas { list, unqualified }
    }

    fn resolution_search_path(&self, position: TextSize) -> Schemas {
        let pg_temp = Schema::new("pg_temp");
        let search_path = self.search_path_at(position);
        let mut list = Schemas::new();
        // Usually empty unless someone is using `create schema`
        if let Some((_, schema)) = self
            .schema_regions
            .iter()
            .find(|(range, _)| range.contains(position))
        {
            list.push(schema.clone());
        }
        if search_path.contains(&pg_temp) {
            list.push(pg_temp.clone());
        }
        list.extend(search_path.iter().filter(|s| **s != pg_temp).cloned());
        list
    }

    pub(crate) fn lookup_with(
        &self,
        name: &Name,
        kind: SymbolKind,
        schemas: &ResolvedSchemas,
    ) -> Option<SyntaxNodePtr> {
        let symbols = self.scope.get(name)?;
        for search_schema in schemas.list() {
            if let Some(symbol_id) = symbols.iter().copied().find(|id| {
                let symbol = &self.symbols[*id];
                symbol.kind == kind && symbol.schema.as_ref() == Some(search_schema)
            }) {
                return Some(self.symbols[symbol_id].ptr);
            }
        }
        None
    }

    pub(crate) fn lookup_with_params(
        &self,
        name: &Name,
        kind: SymbolKind,
        schemas: &ResolvedSchemas,
        params: Option<&[Name]>,
    ) -> Option<SyntaxNodePtr> {
        let symbols = self.scope.get(name)?;
        for search_schema in schemas.list() {
            if let Some(symbol_id) = symbols.iter().copied().find(|id| {
                let symbol = &self.symbols[*id];
                let params_match = match (&symbol.params, params) {
                    (Some(sym_params), Some(req_params)) => sym_params.as_slice() == req_params,
                    (None, None) => true,
                    (_, None) => true,
                    _ => false,
                };
                symbol.kind == kind && symbol.schema.as_ref() == Some(search_schema) && params_match
            }) {
                return Some(self.symbols[symbol_id].ptr);
            }
        }
        None
    }

    pub(crate) fn lookup_with_table(
        &self,
        name: &Name,
        kind: SymbolKind,
        schemas: &ResolvedSchemas,
        table: &Option<Name>,
    ) -> Option<SyntaxNodePtr> {
        let symbols = self.scope.get(name)?;
        for search_schema in schemas.list() {
            if let Some(symbol_id) = symbols.iter().copied().find(|id| {
                let symbol = &self.symbols[*id];
                symbol.kind == kind
                    && symbol.schema.as_ref() == Some(search_schema)
                    && &symbol.table == table
            }) {
                return Some(self.symbols[symbol_id].ptr);
            }
        }
        None
    }

    pub(crate) fn lookup_info(
        &self,
        name: &Name,
        kind: SymbolKind,
        schemas: &ResolvedSchemas,
    ) -> Option<(Schema, String)> {
        let symbols = self.scope.get(name)?;
        for search_schema in schemas.list() {
            if let Some(symbol_id) = symbols.iter().copied().find(|id| {
                let symbol = &self.symbols[*id];
                symbol.kind == kind && symbol.schema.as_ref() == Some(search_schema)
            }) {
                let symbol = &self.symbols[symbol_id];
                return Some((symbol.schema.clone()?, name.to_string()));
            }
        }
        None
    }

    fn default_schema(&self) -> Option<Schema> {
        if let Some(schema) = &self.default_schema_override {
            return Some(schema.clone());
        }
        self.current_search_path().first().cloned()
    }

    fn current_search_path(&self) -> &[Schema] {
        &self
            .search_path_changes
            .last()
            .expect("search_path_changes should never be empty")
            .search_path
    }

    pub(crate) fn search_path_at(&self, position: TextSize) -> &[Schema] {
        // We're assuming people don't actually use `set search_path` that much,
        // so linear search is fine
        for change in self.search_path_changes.iter().rev() {
            if change.position <= position {
                return &change.search_path;
            }
        }
        // default search path
        &self.search_path_changes[0].search_path
    }

    pub(crate) fn all_symbols_by_kind(
        &self,
        kind: SymbolKind,
        schema: Option<&Schema>,
    ) -> Vec<&Name> {
        let mut names = vec![];
        for (name, symbol_ids) in &self.scope.entries {
            for symbol_id in symbol_ids {
                let symbol = &self.symbols[*symbol_id];
                if symbol.kind == kind
                    && schema.is_none_or(|schema| symbol.schema.as_ref() == Some(schema))
                {
                    names.push(name);
                    break;
                }
            }
        }
        names
    }

    pub(crate) fn functions_with_single_param(&self, param_type: &Name) -> Vec<&Name> {
        let mut names = vec![];
        for (name, symbol_ids) in &self.scope.entries {
            for symbol_id in symbol_ids {
                let symbol = &self.symbols[*symbol_id];
                if symbol.kind == SymbolKind::Function
                    && symbol
                        .params
                        .as_ref()
                        .is_some_and(|p| p.len() == 1 && &p[0] == param_type)
                {
                    names.push(name);
                    break;
                }
            }
        }
        names
    }
}

pub(crate) fn bind(file: &ast::SourceFile) -> Binder {
    let mut binder = Binder::new();

    bind_file(&mut binder, file);

    binder
}

fn bind_file(b: &mut Binder, file: &ast::SourceFile) {
    for stmt in file.stmts() {
        bind_stmt(b, stmt);
    }
}

fn bind_stmt(b: &mut Binder, stmt: ast::Stmt) {
    match stmt {
        ast::Stmt::AlterDomain(alter_domain) => bind_alter_domain(b, alter_domain),
        ast::Stmt::AlterTable(alter_table) => bind_alter_table(b, alter_table),
        ast::Stmt::CreateTable(create_table) => bind_create_table(b, create_table),
        ast::Stmt::CreateTableAs(create_table_as) => bind_create_table_as(b, create_table_as),
        ast::Stmt::SelectInto(select_into) => bind_select_into(b, select_into),
        ast::Stmt::CreateForeignTable(create_foreign_table) => {
            bind_create_table(b, create_foreign_table)
        }
        ast::Stmt::CreateIndex(create_index) => bind_create_index(b, create_index),
        ast::Stmt::CreateFunction(create_function) => bind_create_function(b, create_function),
        ast::Stmt::CreateAggregate(create_aggregate) => bind_create_aggregate(b, create_aggregate),
        ast::Stmt::CreateProcedure(create_procedure) => bind_create_procedure(b, create_procedure),
        ast::Stmt::CreateSchema(create_schema) => bind_create_schema(b, create_schema),
        ast::Stmt::CreateType(create_type) => bind_create_type(b, create_type),
        ast::Stmt::CreateDomain(create_domain) => bind_create_domain(b, create_domain),
        ast::Stmt::CreateView(create_view) => bind_create_view(b, create_view),
        ast::Stmt::CreateMaterializedView(create_view) => {
            bind_create_materialized_view(b, create_view)
        }
        ast::Stmt::CreateSequence(create_sequence) => bind_create_sequence(b, create_sequence),
        ast::Stmt::CreateStatistics(create_statistics) => {
            bind_create_statistics(b, create_statistics)
        }
        ast::Stmt::CreateTrigger(create_trigger) => bind_create_trigger(b, create_trigger),
        ast::Stmt::CreateEventTrigger(create_event_trigger) => {
            bind_create_event_trigger(b, create_event_trigger)
        }
        ast::Stmt::CreateTablespace(create_tablespace) => {
            bind_create_tablespace(b, create_tablespace)
        }
        ast::Stmt::CreateDatabase(create_database) => bind_create_database(b, create_database),
        ast::Stmt::CreateServer(create_server) => bind_create_server(b, create_server),
        ast::Stmt::CreateForeignDataWrapper(create_fdw) => {
            bind_create_foreign_data_wrapper(b, create_fdw)
        }
        ast::Stmt::CreatePublication(create_publication) => {
            bind_create_publication(b, create_publication)
        }
        ast::Stmt::CreateSubscription(create_subscription) => {
            bind_create_subscription(b, create_subscription)
        }
        ast::Stmt::CreateLanguage(create_language) => bind_create_language(b, create_language),
        ast::Stmt::CreateCollation(create_collation) => bind_create_collation(b, create_collation),
        ast::Stmt::CreateConversion(create_conversion) => {
            bind_create_conversion(b, create_conversion)
        }
        ast::Stmt::CreateExtension(create_extension) => bind_create_extension(b, create_extension),
        ast::Stmt::CreateAccessMethod(create_access_method) => {
            bind_create_access_method(b, create_access_method)
        }
        ast::Stmt::CreateOperator(create_operator) => bind_create_operator(b, create_operator),
        ast::Stmt::CreateOperatorFamily(create_operator_family) => {
            bind_create_operator_family(b, create_operator_family)
        }
        ast::Stmt::CreateOperatorClass(create_operator_class) => {
            bind_create_operator_class(b, create_operator_class)
        }
        ast::Stmt::CreateTextSearchDictionary(create_text_search_dictionary) => {
            bind_create_text_search_dictionary(b, create_text_search_dictionary)
        }
        ast::Stmt::CreateTextSearchConfiguration(create_text_search_configuration) => {
            bind_create_text_search_configuration(b, create_text_search_configuration)
        }
        ast::Stmt::CreateTextSearchParser(create_text_search_parser) => {
            bind_create_text_search_parser(b, create_text_search_parser)
        }
        ast::Stmt::CreateTextSearchTemplate(create_text_search_template) => {
            bind_create_text_search_template(b, create_text_search_template)
        }
        ast::Stmt::CreateRole(create_role) => bind_create_role(b, create_role.role()),
        ast::Stmt::CreateUser(create_user) => bind_create_role(b, create_user.role()),
        ast::Stmt::CreateGroup(create_group) => bind_create_role(b, create_group.role()),
        ast::Stmt::Declare(declare) => bind_declare_cursor(b, declare),
        ast::Stmt::Prepare(prepare) => bind_prepare(b, prepare),
        ast::Stmt::Listen(listen) => bind_listen(b, listen),
        ast::Stmt::SavepointCreate(savepoint) => bind_savepoint(b, savepoint),
        ast::Stmt::Select(select) => bind_select(b, select),
        ast::Stmt::Set(set) => bind_set(b, set),
        ast::Stmt::CreatePolicy(create_policy) => bind_create_policy(b, create_policy),
        ast::Stmt::CreateRule(create_rule) => bind_create_rule(b, create_rule),
        ast::Stmt::CreatePropertyGraph(create_property_graph) => {
            bind_create_property_graph(b, create_property_graph)
        }
        _ => (),
    }
}

fn bind_create_table(b: &mut Binder, create_table: impl ast::HasCreateTable) {
    let Some(path) = create_table.table_name().and_then(|table| table.path()) else {
        return;
    };
    let Some(table_name) = item_name(&path) else {
        return;
    };
    let name_ptr = path_to_ptr(&path);
    let is_temp = create_table
        .persistence()
        .is_some_and(|p| matches!(p, ast::Persistence::Temp(_)));
    let Some(schema) = schema_name(b, &path, is_temp) else {
        return;
    };

    let table_id = b.symbols.alloc(Symbol {
        kind: SymbolKind::Table,
        ptr: name_ptr,
        schema: Some(schema.clone()),
        params: None,
        table: None,
    });

    let type_id = b.symbols.alloc(Symbol {
        kind: SymbolKind::Type,
        ptr: name_ptr,
        schema: Some(schema.clone()),
        params: None,
        table: None,
    });

    b.scope.insert(table_name.clone(), table_id);
    b.scope.insert(table_name.clone(), type_id);
    bind_create_table_constraints(b, &create_table, &schema, &table_name);
}

fn bind_create_table_constraints(
    b: &mut Binder,
    create_table: &impl ast::HasCreateTable,
    schema: &Schema,
    table_name: &Name,
) {
    let Some(table_arg_list) = create_table.table_arg_list() else {
        return;
    };

    for arg in table_arg_list.args() {
        match arg {
            ast::TableArg::Column(column) => {
                for constraint in column.constraints() {
                    if let Some(constraint_name) = constraint.constraint_name() {
                        bind_constraint_name_node(b, constraint_name, schema, table_name);
                    }
                }
            }
            ast::TableArg::TableConstraint(constraint) => {
                if let Some(constraint_name) = constraint.constraint_name() {
                    bind_constraint_name_node(b, constraint_name, schema, table_name);
                }
            }
            ast::TableArg::LikeClause(_) => (),
        }
    }
}

fn bind_create_table_as(b: &mut Binder, create_table_as: ast::CreateTableAs) {
    let Some(path) = create_table_as.table_name().and_then(|table| table.path()) else {
        return;
    };
    let Some(table_name) = item_name(&path) else {
        return;
    };
    let name_ptr = path_to_ptr(&path);
    let is_temp = create_table_as
        .persistence()
        .is_some_and(|p| matches!(p, ast::Persistence::Temp(_)));
    let Some(schema) = schema_name(b, &path, is_temp) else {
        return;
    };

    let table_id = b.symbols.alloc(Symbol {
        kind: SymbolKind::Table,
        ptr: name_ptr,
        schema: Some(schema.clone()),
        params: None,
        table: None,
    });

    let type_id = b.symbols.alloc(Symbol {
        kind: SymbolKind::Type,
        ptr: name_ptr,
        schema: Some(schema),
        params: None,
        table: None,
    });

    b.scope.insert(table_name.clone(), table_id);
    b.scope.insert(table_name, type_id);
}

fn bind_select_into(b: &mut Binder, select_into: ast::SelectInto) {
    let Some(into_clause) = select_into.into_clause() else {
        return;
    };
    let Some(path) = into_clause.table_name().and_then(|table| table.path()) else {
        return;
    };
    let Some(table_name) = item_name(&path) else {
        return;
    };
    let name_ptr = path_to_ptr(&path);
    let is_temp = into_clause
        .persistence()
        .is_some_and(|p| matches!(p, ast::Persistence::Temp(_)));
    let Some(schema) = schema_name(b, &path, is_temp) else {
        return;
    };

    let table_id = b.symbols.alloc(Symbol {
        kind: SymbolKind::Table,
        ptr: name_ptr,
        schema: Some(schema.clone()),
        params: None,
        table: None,
    });

    let type_id = b.symbols.alloc(Symbol {
        kind: SymbolKind::Type,
        ptr: name_ptr,
        schema: Some(schema),
        params: None,
        table: None,
    });

    b.scope.insert(table_name.clone(), table_id);
    b.scope.insert(table_name, type_id);
}

fn bind_create_index(b: &mut Binder, create_index: ast::CreateIndex) {
    let Some(path) = create_index.index().and_then(|index| index.path()) else {
        return;
    };

    let Some(index_name) = item_name(&path) else {
        return;
    };
    let name_ptr = path_to_ptr(&path);

    let schema = match create_index
        .table_relation_name()
        .and_then(|relation| relation.table_name_ref())
        .and_then(|table| table.path_ref())
    {
        Some(table_path) => schema_name_ref(b, &table_path, false),
        None => b.default_schema(),
    };
    let Some(schema) = schema else {
        return;
    };

    let index_id = b.symbols.alloc(Symbol {
        kind: SymbolKind::Index,
        ptr: name_ptr,
        schema: Some(schema),
        params: None,
        table: None,
    });

    b.scope.insert(index_name, index_id);
}

fn bind_create_function(b: &mut Binder, create_function: ast::CreateFunction) {
    let Some(path) = create_function.name().and_then(|name| name.path()) else {
        return;
    };

    let Some(function_name) = item_name(&path) else {
        return;
    };

    let name_ptr = path_to_ptr(&path);

    let Some(schema) = schema_name(b, &path, false) else {
        return;
    };

    let params = extract_param_signature(create_function.param_list());

    let function_id = b.symbols.alloc(Symbol {
        kind: SymbolKind::Function,
        ptr: name_ptr,
        schema: Some(schema),
        params,
        table: None,
    });

    b.scope.insert(function_name, function_id);

    bind_routine_body_search_path(b, create_function.option_list());
}

fn bind_create_aggregate(b: &mut Binder, create_aggregate: ast::CreateAggregate) {
    let Some(path) = create_aggregate
        .aggregate_name()
        .and_then(|name| name.path())
    else {
        return;
    };

    let Some(aggregate_name) = item_name(&path) else {
        return;
    };

    let name_ptr = path_to_ptr(&path);

    let Some(schema) = schema_name(b, &path, false) else {
        return;
    };

    let params = extract_param_signature(create_aggregate.param_list());

    let aggregate_id = b.symbols.alloc(Symbol {
        kind: SymbolKind::Aggregate,
        ptr: name_ptr,
        schema: Some(schema),
        params,
        table: None,
    });

    b.scope.insert(aggregate_name, aggregate_id);
}

fn bind_create_procedure(b: &mut Binder, create_procedure: ast::CreateProcedure) {
    let Some(path) = create_procedure.name().and_then(|name| name.path()) else {
        return;
    };

    let Some(procedure_name) = item_name(&path) else {
        return;
    };

    let name_ptr = path_to_ptr(&path);

    let Some(schema) = schema_name(b, &path, false) else {
        return;
    };

    let params = extract_param_signature(create_procedure.param_list());

    let procedure_id = b.symbols.alloc(Symbol {
        kind: SymbolKind::Procedure,
        ptr: name_ptr,
        schema: Some(schema),
        params,
        table: None,
    });

    b.scope.insert(procedure_name, procedure_id);

    bind_routine_body_search_path(b, create_procedure.option_list());
}

fn bind_create_schema(b: &mut Binder, create_schema: ast::CreateSchema) {
    let Some(schema_name_node) = create_schema.schema_name() else {
        return;
    };
    let schema_name = Name::from_string(schema_name_node.text().to_string());
    let name_ptr = SyntaxNodePtr::new(&schema_name_node);

    let schema = Schema(schema_name.clone());

    let schema_id = b.symbols.alloc(Symbol {
        kind: SymbolKind::Schema,
        ptr: name_ptr,
        schema: Some(schema.clone()),
        params: None,
        table: None,
    });

    b.scope.insert(schema_name, schema_id);

    b.schema_regions
        .push((create_schema.syntax().text_range(), schema.clone()));

    let prev_override = b.default_schema_override.replace(schema);
    for element in create_schema.schema_elements() {
        bind_schema_element(b, element);
    }
    b.default_schema_override = prev_override;
}

fn bind_schema_element(b: &mut Binder, element: ast::SchemaElement) {
    match element {
        ast::SchemaElement::CreateIndex(create_index) => bind_create_index(b, create_index),
        ast::SchemaElement::CreateSequence(create_sequence) => {
            bind_create_sequence(b, create_sequence)
        }
        ast::SchemaElement::CreateTable(create_table) => bind_create_table(b, create_table),
        ast::SchemaElement::CreateTrigger(create_trigger) => bind_create_trigger(b, create_trigger),
        ast::SchemaElement::CreateView(create_view) => bind_create_view(b, create_view),
        ast::SchemaElement::Grant(_) => (),
    }
}

fn bind_create_type(b: &mut Binder, create_type: ast::CreateType) {
    let Some(path) = create_type
        .type_name()
        .and_then(|type_name| type_name.path())
    else {
        return;
    };

    let Some(type_name) = item_name(&path) else {
        return;
    };

    let name_ptr = path_to_ptr(&path);

    let Some(schema) = schema_name(b, &path, false) else {
        return;
    };

    let type_id = b.symbols.alloc(Symbol {
        kind: SymbolKind::Type,
        ptr: name_ptr,
        schema: Some(schema.clone()),
        params: None,
        table: None,
    });

    b.scope.insert(type_name.clone(), type_id);

    if let Some(ast::CreateTypeKind::RangeType(range_type)) = create_type.kind() {
        if let Some((multirange_name, multirange_ptr, multirange_schema)) =
            multirange_type_from_range(b, &range_type, type_name, schema, name_ptr)
        {
            let multirange_id = b.symbols.alloc(Symbol {
                kind: SymbolKind::Type,
                ptr: multirange_ptr,
                schema: Some(multirange_schema),
                params: None,
                table: None,
            });
            b.scope.insert(multirange_name, multirange_id);
        }
    }
}

fn bind_create_domain(b: &mut Binder, create_domain: ast::CreateDomain) {
    let Some(path) = create_domain.domain().and_then(|domain| domain.path()) else {
        return;
    };

    let Some(domain_name) = item_name(&path) else {
        return;
    };

    let name_ptr = path_to_ptr(&path);

    let Some(schema) = schema_name(b, &path, false) else {
        return;
    };

    let type_id = b.symbols.alloc(Symbol {
        kind: SymbolKind::Type,
        ptr: name_ptr,
        schema: Some(schema.clone()),
        params: None,
        table: None,
    });

    b.scope.insert(domain_name.clone(), type_id);

    for constraint in create_domain.constraints() {
        if let Some(constraint_name) = constraint.constraint_name() {
            bind_constraint_name_node(b, constraint_name, &schema, &domain_name);
        }
    }
}

fn bind_alter_table(b: &mut Binder, alter_table: ast::AlterTable) {
    let Some(path) = alter_table
        .table_relation_name()
        .and_then(|relation| relation.table_name_ref())
        .and_then(|table| table.path_ref())
    else {
        return;
    };
    let Some(table_name) = item_name_ref(&path) else {
        return;
    };
    let Some(schema) = schema_name_ref(b, &path, false) else {
        return;
    };

    for action in alter_table.actions() {
        match action {
            ast::AlterTableAction::AddColumn(add_column) => {
                for constraint in add_column.constraints() {
                    if let Some(constraint_name) = constraint.constraint_name() {
                        bind_constraint_name_node(b, constraint_name, &schema, &table_name);
                    }
                }
            }
            ast::AlterTableAction::AddConstraint(add_constraint) => {
                if let Some(constraint) = add_constraint.constraint()
                    && let Some(constraint_name) = constraint.constraint_name()
                {
                    bind_constraint_name_node(b, constraint_name, &schema, &table_name);
                }
            }
            ast::AlterTableAction::RenameConstraint(rename_constraint) => {
                if let Some(constraint_name) = rename_constraint.constraint_name() {
                    bind_constraint_name_node(b, constraint_name, &schema, &table_name);
                }
            }
            _ => (),
        }
    }
}

fn bind_alter_domain(b: &mut Binder, alter_domain: ast::AlterDomain) {
    let Some(path) = alter_domain
        .domain_ref()
        .and_then(|domain| domain.path_ref())
    else {
        return;
    };
    let Some(domain_name) = item_name_ref(&path) else {
        return;
    };
    let Some(schema) = schema_name_ref(b, &path, false) else {
        return;
    };

    match alter_domain.action() {
        Some(ast::AlterDomainAction::AddConstraint(add_constraint)) => {
            if let Some(constraint) = add_constraint.constraint()
                && let Some(constraint_name) = constraint.constraint_name()
            {
                bind_constraint_name_node(b, constraint_name, &schema, &domain_name);
            }
        }
        Some(ast::AlterDomainAction::RenameConstraint(rename_constraint)) => {
            if let Some(constraint_name) = rename_constraint.constraint_name() {
                bind_constraint_name_node(b, constraint_name, &schema, &domain_name);
            }
        }
        _ => (),
    }
}

fn bind_constraint_name_node(
    b: &mut Binder,
    constraint_name: ast::ConstraintName,
    schema: &Schema,
    owner_name: &Name,
) {
    let name_ptr = SyntaxNodePtr::new(constraint_name.syntax());
    let constraint_name = Name::from_node(&constraint_name);
    let constraint_id = b.symbols.alloc(Symbol {
        kind: SymbolKind::Constraint,
        ptr: name_ptr,
        schema: Some(schema.clone()),
        params: None,
        table: Some(owner_name.clone()),
    });

    b.scope.insert(constraint_name, constraint_id);
}

fn multirange_type_from_range(
    b: &Binder,
    range_type: &ast::RangeType,
    type_name: Name,
    schema: Schema,
    fallback_ptr: SyntaxNodePtr,
) -> Option<(Name, SyntaxNodePtr, Schema)> {
    if let Some(attribute_list) = range_type.attribute_list() {
        let multirange_key = Name::from_string("multirange_type_name");
        for option in attribute_list.attribute_options() {
            let Some(name) = option.name() else {
                continue;
            };
            if Name::from_node(&name) != multirange_key {
                continue;
            }
            if let Some(attribute_value) = option.attribute_value() {
                if let Some(literal) = attribute_value.literal()
                    && let Some(string_value) = extract_string_literal(&literal)
                {
                    let multirange_name = Name::from_string(string_value);
                    return Some((multirange_name, fallback_ptr, schema));
                }
                if let Some(ast::Type::PathType(path_type)) = attribute_value.ty()
                    && let Some(path) = path_type.path_ref()
                    && let Some(multirange_name) = item_name_ref(&path)
                {
                    let multirange_schema = if path.qualifier().is_some() {
                        schema_name_ref(b, &path, false)?
                    } else {
                        schema
                    };
                    return Some((multirange_name, fallback_ptr, multirange_schema));
                }
            }
        }
    }

    let multirange_name = derive_multirange_name(type_name);
    Some((multirange_name, fallback_ptr, schema))
}

// from postgres docs:
// > If the range type name contains the substring range, then the multirange type
// > name is formed by replacement of the range substring with multirange in the
// > range type name.
// > Otherwise, the multirange type name is formed by appending a
// > _multirange suffix to the range type name.
fn derive_multirange_name(range_name: Name) -> Name {
    let range_text = range_name.0.as_str();
    if range_text.contains("range") {
        Name::from_string(range_text.replacen("range", "multirange", 1))
    } else {
        Name::from_string(format!("{range_text}_multirange"))
    }
}

fn bind_create_view(b: &mut Binder, create_view: ast::CreateView) {
    let Some(path) = create_view.view().and_then(|view| view.path()) else {
        return;
    };

    let Some(view_name) = item_name(&path) else {
        return;
    };

    let name_ptr = path_to_ptr(&path);
    let is_temp = create_view
        .persistence()
        .is_some_and(|p| matches!(p, ast::Persistence::Temp(_)));

    let Some(schema) = schema_name(b, &path, is_temp) else {
        return;
    };

    let view_id = b.symbols.alloc(Symbol {
        kind: SymbolKind::View,
        ptr: name_ptr,
        schema: Some(schema),
        params: None,
        table: None,
    });

    b.scope.insert(view_name, view_id);
}

// TODO: combine with create_view
fn bind_create_materialized_view(b: &mut Binder, create_view: ast::CreateMaterializedView) {
    let Some(path) = create_view.view().and_then(|view| view.path()) else {
        return;
    };

    let Some(view_name) = item_name(&path) else {
        return;
    };

    let name_ptr = path_to_ptr(&path);

    let Some(schema) = schema_name(b, &path, false) else {
        return;
    };

    let view_id = b.symbols.alloc(Symbol {
        kind: SymbolKind::View,
        ptr: name_ptr,
        schema: Some(schema),
        params: None,
        table: None,
    });

    b.scope.insert(view_name, view_id);
}

fn bind_create_sequence(b: &mut Binder, create_sequence: ast::CreateSequence) {
    let Some(path) = create_sequence
        .sequence()
        .and_then(|sequence| sequence.path())
    else {
        return;
    };

    let Some(sequence_name) = item_name(&path) else {
        return;
    };

    let name_ptr = path_to_ptr(&path);
    let is_temp = create_sequence
        .persistence()
        .is_some_and(|p| matches!(p, ast::Persistence::Temp(_)));

    let Some(schema) = schema_name(b, &path, is_temp) else {
        return;
    };

    let sequence_id = b.symbols.alloc(Symbol {
        kind: SymbolKind::Sequence,
        ptr: name_ptr,
        schema: Some(schema),
        params: None,
        table: None,
    });

    b.scope.insert(sequence_name, sequence_id);
}

fn bind_create_statistics(b: &mut Binder, create_statistics: ast::CreateStatistics) {
    let Some(path) = create_statistics
        .statistics()
        .and_then(|statistics| statistics.path())
    else {
        return;
    };

    let Some(statistics_name) = item_name(&path) else {
        return;
    };

    let name_ptr = path_to_ptr(&path);

    let Some(schema) = schema_name(b, &path, false) else {
        return;
    };

    let statistics_id = b.symbols.alloc(Symbol {
        kind: SymbolKind::Statistics,
        ptr: name_ptr,
        schema: Some(schema),
        params: None,
        table: None,
    });

    b.scope.insert(statistics_name, statistics_id);
}

fn bind_create_trigger(b: &mut Binder, create_trigger: ast::CreateTrigger) {
    let Some(trigger) = create_trigger.trigger() else {
        return;
    };

    let trigger_name = Name::from_node(&trigger);
    let name_ptr = SyntaxNodePtr::new(trigger.syntax());

    let Some(table_path) = create_trigger
        .on_relation()
        .and_then(|on| on.relation_name_ref())
        .and_then(|relation| relation.path_ref())
    else {
        return;
    };

    let Some(table_name) = item_name_ref(&table_path) else {
        return;
    };

    let Some(schema) = schema_name_ref(b, &table_path, false) else {
        return;
    };

    let trigger_id = b.symbols.alloc(Symbol {
        kind: SymbolKind::Trigger,
        ptr: name_ptr,
        schema: Some(schema),
        params: None,
        table: Some(table_name),
    });

    b.scope.insert(trigger_name, trigger_id);
}

fn bind_create_policy(b: &mut Binder, create_policy: ast::CreatePolicy) {
    let Some(policy) = create_policy.policy() else {
        return;
    };

    let policy_name = Name::from_node(&policy);
    let name_ptr = SyntaxNodePtr::new(policy.syntax());

    let Some(table_path) = create_policy
        .on_table()
        .and_then(|on| on.table_name_ref())
        .and_then(|table| table.path_ref())
    else {
        return;
    };

    let Some(table_name) = item_name_ref(&table_path) else {
        return;
    };

    let Some(schema) = schema_name_ref(b, &table_path, false) else {
        return;
    };

    let policy_id = b.symbols.alloc(Symbol {
        kind: SymbolKind::Policy,
        ptr: name_ptr,
        schema: Some(schema),
        params: None,
        table: Some(table_name),
    });

    b.scope.insert(policy_name, policy_id);
}

fn bind_create_rule(b: &mut Binder, create_rule: ast::CreateRule) {
    let Some(rule) = create_rule.rule() else {
        return;
    };

    let rule_name = Name::from_node(&rule);
    let name_ptr = SyntaxNodePtr::new(rule.syntax());

    let Some(table_path) = create_rule
        .rule_on()
        .and_then(|on| on.relation_name_ref())
        .and_then(|relation| relation.path_ref())
    else {
        return;
    };

    let Some(table_name) = item_name_ref(&table_path) else {
        return;
    };

    let Some(schema) = schema_name_ref(b, &table_path, false) else {
        return;
    };

    let rule_id = b.symbols.alloc(Symbol {
        kind: SymbolKind::Rule,
        ptr: name_ptr,
        schema: Some(schema),
        params: None,
        table: Some(table_name),
    });

    b.scope.insert(rule_name, rule_id);
}

fn bind_create_property_graph(b: &mut Binder, create_property_graph: ast::CreatePropertyGraph) {
    let Some(path) = create_property_graph
        .property_graph()
        .and_then(|property_graph| property_graph.path())
    else {
        return;
    };
    let Some(property_graph_name) = item_name(&path) else {
        return;
    };
    let name_ptr = path_to_ptr(&path);
    let Some(schema) = schema_name(b, &path, false) else {
        return;
    };

    let property_graph_id = b.symbols.alloc(Symbol {
        kind: SymbolKind::PropertyGraph,
        ptr: name_ptr,
        schema: Some(schema),
        params: None,
        table: None,
    });

    b.scope.insert(property_graph_name, property_graph_id);
}

fn bind_create_event_trigger(b: &mut Binder, create_event_trigger: ast::CreateEventTrigger) {
    let Some(event_trigger) = create_event_trigger.event_trigger() else {
        return;
    };

    let event_trigger_name = Name::from_node(&event_trigger);
    let name_ptr = SyntaxNodePtr::new(event_trigger.syntax());

    let event_trigger_id = b.symbols.alloc(Symbol {
        kind: SymbolKind::EventTrigger,
        ptr: name_ptr,
        schema: None,
        params: None,
        table: None,
    });

    b.scope.insert(event_trigger_name, event_trigger_id);
}

fn bind_create_tablespace(b: &mut Binder, create_tablespace: ast::CreateTablespace) {
    let Some(tablespace) = create_tablespace.tablespace() else {
        return;
    };

    let tablespace_name = Name::from_node(&tablespace);
    let name_ptr = SyntaxNodePtr::new(tablespace.syntax());

    let tablespace_id = b.symbols.alloc(Symbol {
        kind: SymbolKind::Tablespace,
        ptr: name_ptr,
        schema: None,
        params: None,
        table: None,
    });

    b.scope.insert(tablespace_name, tablespace_id);
}

fn bind_create_database(b: &mut Binder, create_database: ast::CreateDatabase) {
    let Some(database) = create_database.database() else {
        return;
    };

    let database_name = Name::from_node(&database);
    let name_ptr = SyntaxNodePtr::new(database.syntax());

    let database_id = b.symbols.alloc(Symbol {
        kind: SymbolKind::Database,
        ptr: name_ptr,
        schema: None,
        params: None,
        table: None,
    });

    b.scope.insert(database_name, database_id);
}

fn bind_create_server(b: &mut Binder, create_server: ast::CreateServer) {
    let Some(server) = create_server.server() else {
        return;
    };

    let server_name = Name::from_node(&server);
    let name_ptr = SyntaxNodePtr::new(server.syntax());

    let server_id = b.symbols.alloc(Symbol {
        kind: SymbolKind::Server,
        ptr: name_ptr,
        schema: None,
        params: None,
        table: None,
    });

    b.scope.insert(server_name, server_id);
}

fn bind_create_foreign_data_wrapper(b: &mut Binder, create_fdw: ast::CreateForeignDataWrapper) {
    let Some(foreign_data_wrapper) = create_fdw.foreign_data_wrapper() else {
        return;
    };

    let fdw_name = Name::from_node(&foreign_data_wrapper);
    let name_ptr = SyntaxNodePtr::new(foreign_data_wrapper.syntax());

    let fdw_id = b.symbols.alloc(Symbol {
        kind: SymbolKind::ForeignDataWrapper,
        ptr: name_ptr,
        schema: None,
        params: None,
        table: None,
    });

    b.scope.insert(fdw_name, fdw_id);
}

fn bind_create_publication(b: &mut Binder, create_publication: ast::CreatePublication) {
    let Some(publication) = create_publication.publication() else {
        return;
    };

    let publication_name = Name::from_node(&publication);
    let name_ptr = SyntaxNodePtr::new(publication.syntax());

    let publication_id = b.symbols.alloc(Symbol {
        kind: SymbolKind::Publication,
        ptr: name_ptr,
        schema: None,
        params: None,
        table: None,
    });

    b.scope.insert(publication_name, publication_id);
}

fn bind_create_subscription(b: &mut Binder, create_subscription: ast::CreateSubscription) {
    let Some(subscription) = create_subscription.subscription() else {
        return;
    };

    let subscription_name = Name::from_node(&subscription);
    let name_ptr = SyntaxNodePtr::new(subscription.syntax());

    let subscription_id = b.symbols.alloc(Symbol {
        kind: SymbolKind::Subscription,
        ptr: name_ptr,
        schema: None,
        params: None,
        table: None,
    });

    b.scope.insert(subscription_name, subscription_id);
}

fn bind_create_language(b: &mut Binder, create_language: ast::CreateLanguage) {
    let Some(language) = create_language.language() else {
        return;
    };

    let language_name = Name::from_node(&language);
    let name_ptr = SyntaxNodePtr::new(language.syntax());

    let language_id = b.symbols.alloc(Symbol {
        kind: SymbolKind::Language,
        ptr: name_ptr,
        schema: None,
        params: None,
        table: None,
    });

    b.scope.insert(language_name, language_id);
}

fn bind_create_collation(b: &mut Binder, create_collation: ast::CreateCollation) {
    let Some(path) = create_collation
        .collation()
        .and_then(|collation| collation.path())
    else {
        return;
    };

    let Some(collation_name) = item_name(&path) else {
        return;
    };

    let Some(schema) = schema_name(b, &path, false) else {
        return;
    };

    let collation_id = b.symbols.alloc(Symbol {
        kind: SymbolKind::Collation,
        ptr: path_to_ptr(&path),
        schema: Some(schema),
        params: None,
        table: None,
    });

    b.scope.insert(collation_name, collation_id);
}

fn bind_create_conversion(b: &mut Binder, create_conversion: ast::CreateConversion) {
    let Some(path) = create_conversion
        .conversion()
        .and_then(|conversion| conversion.path())
    else {
        return;
    };

    let Some(conversion_name) = item_name(&path) else {
        return;
    };

    let Some(schema) = schema_name(b, &path, false) else {
        return;
    };

    let conversion_id = b.symbols.alloc(Symbol {
        kind: SymbolKind::Conversion,
        ptr: path_to_ptr(&path),
        schema: Some(schema),
        params: None,
        table: None,
    });

    b.scope.insert(conversion_name, conversion_id);
}

fn bind_create_access_method(b: &mut Binder, create_access_method: ast::CreateAccessMethod) {
    let Some(access_method) = create_access_method.access_method() else {
        return;
    };

    let access_method_name = Name::from_node(&access_method);
    let name_ptr = SyntaxNodePtr::new(access_method.syntax());

    let access_method_id = b.symbols.alloc(Symbol {
        kind: SymbolKind::AccessMethod,
        ptr: name_ptr,
        schema: None,
        params: None,
        table: None,
    });

    b.scope.insert(access_method_name, access_method_id);
}

fn bind_create_operator(b: &mut Binder, create_operator: ast::CreateOperator) {
    let Some(op) = create_operator.op() else {
        return;
    };

    let Some(custom_op) = op.custom_op() else {
        return;
    };

    let operator_name = Name::from_string(custom_op.syntax().text().to_string());

    let path = op.syntax().children().find_map(ast::Path::cast);
    let schema = match &path {
        Some(path) => schema_name(b, path, false),
        None => b.default_schema(),
    };

    let operator_id = b.symbols.alloc(Symbol {
        kind: SymbolKind::Operator,
        ptr: SyntaxNodePtr::new(op.syntax()),
        schema,
        params: None,
        table: None,
    });

    b.scope.insert(operator_name, operator_id);
}

fn bind_create_operator_family(b: &mut Binder, create_operator_family: ast::CreateOperatorFamily) {
    let Some(path) = create_operator_family
        .op_family_name()
        .and_then(|name| name.path())
    else {
        return;
    };

    let Some(operator_family_name) = item_name(&path) else {
        return;
    };

    let Some(schema) = schema_name(b, &path, false) else {
        return;
    };

    let operator_family_id = b.symbols.alloc(Symbol {
        kind: SymbolKind::OperatorFamily,
        ptr: path_to_ptr(&path),
        schema: Some(schema),
        params: None,
        table: None,
    });

    b.scope.insert(operator_family_name, operator_family_id);
}

fn bind_create_text_search_dictionary(
    b: &mut Binder,
    create_text_search_dictionary: ast::CreateTextSearchDictionary,
) {
    let Some(path) = create_text_search_dictionary
        .text_search_dictionary()
        .and_then(|dictionary| dictionary.path())
    else {
        return;
    };

    let Some(dictionary_name) = item_name(&path) else {
        return;
    };

    let Some(schema) = schema_name(b, &path, false) else {
        return;
    };

    let dictionary_id = b.symbols.alloc(Symbol {
        kind: SymbolKind::TextSearchDictionary,
        ptr: path_to_ptr(&path),
        schema: Some(schema),
        params: None,
        table: None,
    });

    b.scope.insert(dictionary_name, dictionary_id);
}

fn bind_create_text_search_configuration(
    b: &mut Binder,
    create_text_search_configuration: ast::CreateTextSearchConfiguration,
) {
    let Some(path) = create_text_search_configuration
        .text_search_configuration()
        .and_then(|configuration| configuration.path())
    else {
        return;
    };

    let Some(configuration_name) = item_name(&path) else {
        return;
    };

    let Some(schema) = schema_name(b, &path, false) else {
        return;
    };

    let configuration_id = b.symbols.alloc(Symbol {
        kind: SymbolKind::TextSearchConfiguration,
        ptr: path_to_ptr(&path),
        schema: Some(schema),
        params: None,
        table: None,
    });

    b.scope.insert(configuration_name, configuration_id);
}

fn bind_create_text_search_parser(
    b: &mut Binder,
    create_text_search_parser: ast::CreateTextSearchParser,
) {
    let Some(path) = create_text_search_parser
        .text_search_parser()
        .and_then(|parser| parser.path())
    else {
        return;
    };

    let Some(parser_name) = item_name(&path) else {
        return;
    };

    let Some(schema) = schema_name(b, &path, false) else {
        return;
    };

    let parser_id = b.symbols.alloc(Symbol {
        kind: SymbolKind::TextSearchParser,
        ptr: path_to_ptr(&path),
        schema: Some(schema),
        params: None,
        table: None,
    });

    b.scope.insert(parser_name, parser_id);
}

fn bind_create_text_search_template(
    b: &mut Binder,
    create_text_search_template: ast::CreateTextSearchTemplate,
) {
    let Some(path) = create_text_search_template
        .text_search_template()
        .and_then(|template| template.path())
    else {
        return;
    };

    let Some(template_name) = item_name(&path) else {
        return;
    };

    let Some(schema) = schema_name(b, &path, false) else {
        return;
    };

    let template_id = b.symbols.alloc(Symbol {
        kind: SymbolKind::TextSearchTemplate,
        ptr: path_to_ptr(&path),
        schema: Some(schema),
        params: None,
        table: None,
    });

    b.scope.insert(template_name, template_id);
}

fn bind_create_operator_class(b: &mut Binder, create_operator_class: ast::CreateOperatorClass) {
    let Some(path) = create_operator_class
        .op_class_name()
        .and_then(|name| name.path())
    else {
        return;
    };

    let Some(operator_class_name) = item_name(&path) else {
        return;
    };

    let Some(schema) = schema_name(b, &path, false) else {
        return;
    };

    let operator_class_id = b.symbols.alloc(Symbol {
        kind: SymbolKind::OperatorClass,
        ptr: path_to_ptr(&path),
        schema: Some(schema),
        params: None,
        table: None,
    });

    b.scope.insert(operator_class_name, operator_class_id);
}

fn bind_create_extension(b: &mut Binder, create_extension: ast::CreateExtension) {
    let Some(extension) = create_extension.extension() else {
        return;
    };

    let extension_name = Name::from_node(&extension);
    let name_ptr = SyntaxNodePtr::new(extension.syntax());

    let extension_id = b.symbols.alloc(Symbol {
        kind: SymbolKind::Extension,
        ptr: name_ptr,
        schema: None,
        params: None,
        table: None,
    });

    b.scope.insert(extension_name, extension_id);
}

fn bind_create_role(b: &mut Binder, role: Option<ast::Role>) {
    let Some(role) = role else {
        return;
    };

    let role_name = Name::from_node(&role);
    let name_ptr = SyntaxNodePtr::new(role.syntax());

    let role_id = b.symbols.alloc(Symbol {
        kind: SymbolKind::Role,
        ptr: name_ptr,
        schema: None,
        params: None,
        table: None,
    });

    b.scope.insert(role_name, role_id);
}

fn bind_declare_cursor(b: &mut Binder, declare: ast::Declare) {
    let Some(cursor) = declare.cursor() else {
        return;
    };

    let cursor_name = Name::from_node(&cursor);
    let name_ptr = SyntaxNodePtr::new(cursor.syntax());

    let cursor_id = b.symbols.alloc(Symbol {
        kind: SymbolKind::Cursor,
        ptr: name_ptr,
        schema: None,
        params: None,
        table: None,
    });

    b.scope.insert(cursor_name, cursor_id);
}

fn bind_prepare(b: &mut Binder, prepare: ast::Prepare) {
    let Some(statement) = prepare.name() else {
        return;
    };

    let statement_name = Name::from_node(&statement);
    let name_ptr = SyntaxNodePtr::new(statement.syntax());

    let statement_id = b.symbols.alloc(Symbol {
        kind: SymbolKind::PreparedStatement,
        ptr: name_ptr,
        schema: None,
        params: None,
        table: None,
    });

    b.scope.insert(statement_name, statement_id);
}

fn bind_listen(b: &mut Binder, listen: ast::Listen) {
    let Some(channel) = listen.channel() else {
        return;
    };

    let channel_name = Name::from_node(&channel);
    let name_ptr = SyntaxNodePtr::new(channel.syntax());

    let channel_id = b.symbols.alloc(Symbol {
        kind: SymbolKind::Channel,
        ptr: name_ptr,
        schema: None,
        params: None,
        table: None,
    });

    b.scope.insert(channel_name, channel_id);
}

fn bind_savepoint(b: &mut Binder, savepoint: ast::SavepointCreate) {
    let Some(savepoint) = savepoint.savepoint() else {
        return;
    };

    let savepoint_name = Name::from_node(&savepoint);
    let name_ptr = SyntaxNodePtr::new(savepoint.syntax());

    let savepoint_id = b.symbols.alloc(Symbol {
        kind: SymbolKind::Savepoint,
        ptr: name_ptr,
        schema: None,
        params: None,
        table: None,
    });

    b.scope.insert(savepoint_name, savepoint_id);
}

fn item_name(path: &ast::Path) -> Option<Name> {
    Some(Name::from_node(&path.segment()?.name()?))
}

fn item_name_ref(path: &ast::PathRef) -> Option<Name> {
    Some(Name::from_node(&path.segment()?.name_ref()?))
}

fn path_to_ptr(path: &ast::Path) -> SyntaxNodePtr {
    path.segment()
        .and_then(|segment| segment.name())
        .map_or_else(
            || SyntaxNodePtr::new(path.syntax()),
            |name| SyntaxNodePtr::new(name.syntax()),
        )
}

fn schema_name(b: &Binder, path: &ast::Path, is_temp: bool) -> Option<Schema> {
    schema_name_from_qualifier(b, path.qualifier(), is_temp)
}

fn schema_name_ref(b: &Binder, path: &ast::PathRef, is_temp: bool) -> Option<Schema> {
    schema_name_from_qualifier(b, path.qualifier(), is_temp)
}

fn schema_name_from_qualifier(
    b: &Binder,
    qualifier: Option<ast::PathRef>,
    is_temp: bool,
) -> Option<Schema> {
    if let Some(name_ref) = qualifier
        .and_then(|q| q.segment())
        .and_then(|s| s.name_ref())
    {
        return Some(Schema(Name::from_node(&name_ref)));
    }

    if is_temp {
        return Some(Schema::new("pg_temp"));
    }

    b.default_schema()
}

fn bind_routine_body_search_path(b: &mut Binder, option_list: Option<ast::FuncOptionList>) {
    let Some(option_list) = option_list else {
        return;
    };

    let mut search_path = None;
    let mut body_range = None;
    for option in option_list.options() {
        match option {
            ast::FuncOption::SetFuncOption(set_func_option) => {
                if let Some(set_config_param) = set_func_option.set_config_param() {
                    search_path = search_path_from_set_config_param(&set_config_param);
                }
            }
            ast::FuncOption::BeginFuncOptionList(begin_func_option_list) => {
                body_range = Some(begin_func_option_list.syntax().text_range());
            }
            _ => (),
        }
    }

    let (Some(search_path), Some(body_range)) = (search_path, body_range) else {
        return;
    };

    let previous_search_path = b.current_search_path().to_vec();
    let search_path = match search_path {
        SearchPathOverride::Explicit(search_path) => search_path,
        SearchPathOverride::FromCurrent => previous_search_path.clone(),
    };
    b.search_path_changes.push(SearchPathChange {
        position: body_range.start(),
        search_path,
    });
    b.search_path_changes.push(SearchPathChange {
        position: body_range.end(),
        search_path: previous_search_path,
    });
}

enum SearchPathOverride {
    Explicit(Vec<Schema>),
    FromCurrent,
}

fn search_path_from_set_config_param(
    set_config_param: &ast::SetConfigParam,
) -> Option<SearchPathOverride> {
    let path = set_config_param.path_ref()?;
    if path.qualifier().is_some() {
        return None;
    }

    let segment = path.segment()?;
    let param_name = segment.name_ref()?.syntax().text().to_string();
    if !param_name.eq_ignore_ascii_case("search_path") {
        return None;
    }

    if set_config_param.current_token().is_some() {
        return Some(SearchPathOverride::FromCurrent);
    }

    if set_config_param.default_token().is_some() {
        return Some(SearchPathOverride::Explicit(vec![
            Schema::new("public"),
            Schema::new("pg_temp"),
            Schema::new("pg_catalog"),
        ]));
    }

    let mut search_path = vec![];
    for literal in set_config_param.literals() {
        if let Some(string_value) = extract_string_literal(&literal)
            && !string_value.is_empty()
        {
            search_path.push(Schema::new(string_value));
        }
    }
    for name_ref in set_config_param.name_refs() {
        search_path.push(Schema::new(name_ref.syntax().text().to_string()));
    }

    Some(SearchPathOverride::Explicit(search_path))
}

fn bind_set(b: &mut Binder, set: ast::Set) {
    let position = set.syntax().text_range().start();

    match set.set_target() {
        // `set schema` is an alternative to `set search_path`
        Some(ast::SetTarget::SetSchemaValue(set_schema)) => {
            if let Some(literal) = set_schema.literal()
                && let Some(string_value) = extract_string_literal(&literal)
            {
                b.search_path_changes.push(SearchPathChange {
                    position,
                    search_path: vec![Schema::new(string_value)],
                });
            }
        }
        Some(ast::SetTarget::SetConfig(set_config)) => bind_set_config(b, set_config, position),
        _ => (),
    }
}

fn bind_set_config(b: &mut Binder, set_config: ast::SetConfig, position: TextSize) {
    let Some(path) = set_config.path_ref() else {
        return;
    };

    if path.qualifier().is_some() {
        return;
    }

    let Some(segment) = path.segment() else {
        return;
    };

    let param_name = if let Some(name_ref) = segment.name_ref() {
        name_ref.syntax().text().to_string()
    } else {
        return;
    };

    if !param_name.eq_ignore_ascii_case("search_path") {
        return;
    }

    // `set search_path`
    if set_config.default_token().is_some() {
        b.search_path_changes.push(SearchPathChange {
            position,
            search_path: vec![
                Schema::new("public"),
                Schema::new("pg_temp"),
                Schema::new("pg_catalog"),
            ],
        });
    } else {
        let mut search_path = vec![];
        for config_value in set_config.config_values() {
            match config_value {
                ast::ConfigValue::Literal(literal) => {
                    if let Some(string_value) = extract_string_literal(&literal) {
                        // You can unset the search path via `set search_path = ''`
                        // so we want to skip over these, otherwise we'll
                        // have a schema of value `''` which isn't valid.
                        if !string_value.is_empty() {
                            search_path.push(Schema::new(string_value));
                        }
                    }
                }
                ast::ConfigValue::NameRef(name_ref) => {
                    let schema_name = name_ref.syntax().text().to_string();
                    search_path.push(Schema::new(schema_name));
                }
            }
        }
        b.search_path_changes.push(SearchPathChange {
            position,
            search_path,
        });
    }
}

fn bind_select(b: &mut Binder, select: ast::Select) {
    let position = select.syntax().text_range().start();
    bind_select_set_config(b, &select, position);
}

// `select set_config('search_path', 'foo, public', false)` is the functional
// equivalent of `set search_path to foo, public`.
fn bind_select_set_config(b: &mut Binder, select: &ast::Select, position: TextSize) {
    if select.from_clause().is_some() {
        return;
    }

    let mut targets = select
        .select_clause()
        .and_then(|select_clause| select_clause.target_list())
        .into_iter()
        .flat_map(|target_list| target_list.targets());

    let Some(target) = targets.next() else {
        return;
    };
    if targets.next().is_some() {
        return;
    }

    let Some(ast::Expr::CallExpr(call_expr)) = target.expr() else {
        return;
    };

    let Some((schema, func_name)) = schema_and_func_name(&call_expr) else {
        return;
    };
    if func_name != Name::from_string("set_config") {
        return;
    }
    if let Some(schema) = &schema
        && *schema != Schema::new("pg_catalog")
    {
        return;
    }

    let schemas = b.resolved_schemas(position, schema.as_ref());
    if let Some((resolved_schema, _)) = b.lookup_info(&func_name, SymbolKind::Function, &schemas)
        && resolved_schema != Schema::new("pg_catalog")
    {
        return;
    }

    let mut args = call_expr.arg_list().into_iter().flat_map(|al| al.args());

    let Some(ast::Expr::Literal(setting_name_literal)) = args.next().and_then(|a| a.expr()) else {
        return;
    };
    let Some(setting_name) = literal_string_value(&setting_name_literal) else {
        return;
    };
    if !setting_name.eq_ignore_ascii_case("search_path") {
        return;
    }

    let Some(ast::Expr::Literal(new_value_literal)) = args.next().and_then(|a| a.expr()) else {
        return;
    };
    let Some(new_value) = literal_string_value(&new_value_literal) else {
        return;
    };

    let search_path = new_value
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(Schema::new)
        .collect();

    b.search_path_changes.push(SearchPathChange {
        position,
        search_path,
    });
}

pub(crate) fn extract_string_literal(literal: &ast::Literal) -> Option<String> {
    let text = literal.syntax().text().to_string();

    if text.starts_with('\'') && text.ends_with('\'') && text.len() >= 2 {
        Some(text[1..text.len() - 1].to_string())
    } else {
        None
    }
}

fn extract_param_signature(param_list: Option<ast::ParamList>) -> Option<Vec<Name>> {
    let param_list = param_list?;
    let mut params = vec![];
    for param in param_list.params() {
        if let Some(ty) = param.ty()
            && let ast::Type::PathType(path_type) = ty
            && let Some(path) = path_type.path_ref()
            && let Some(segment) = path.segment()
            && let Some(name_ref) = segment.name_ref()
        {
            params.push(Name::from_node(&name_ref));
        }
    }
    (!params.is_empty()).then_some(params)
}
