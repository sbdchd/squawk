/// Loosely based on TypeScript's binder
/// see: typescript-go/internal/binder/binder.go
use la_arena::Arena;
use rowan::TextSize;
use squawk_syntax::{SyntaxNodePtr, ast, ast::AstNode};

use crate::scope::{Scope, ScopeId};
use crate::symbols::{Name, Schema, Symbol, SymbolKind};

struct SearchPathChange {
    position: TextSize,
    search_path: Vec<Schema>,
}

pub(crate) struct Binder {
    // TODO: doesn't seem like we need this with our resolve setup
    scopes: Arena<Scope>,
    symbols: Arena<Symbol>,
    search_path_changes: Vec<SearchPathChange>,
}

impl Binder {
    fn new() -> Self {
        let mut scopes = Arena::new();
        let _root_scope = scopes.alloc(Scope::with_parent(None));
        Binder {
            scopes,
            symbols: Arena::new(),
            search_path_changes: vec![SearchPathChange {
                position: TextSize::from(0),
                search_path: vec![
                    Schema::new("public"),
                    Schema::new("pg_temp"),
                    Schema::new("pg_catalog"),
                ],
            }],
        }
    }

    fn root_scope(&self) -> ScopeId {
        self.scopes
            .iter()
            .next()
            .map(|(id, _)| id)
            .expect("root scope must exist")
    }

    pub(crate) fn lookup(&self, name: &Name, kind: SymbolKind) -> Option<SyntaxNodePtr> {
        let symbols = self.scopes[self.root_scope()].get(name)?;
        let symbol_id = symbols.iter().copied().find(|id| {
            let symbol = &self.symbols[*id];
            symbol.kind == kind
        })?;
        Some(self.symbols[symbol_id].ptr)
    }

    pub(crate) fn lookup_with(
        &self,
        name: &Name,
        kind: SymbolKind,
        position: TextSize,
        schema: &Option<Schema>,
    ) -> Option<SyntaxNodePtr> {
        let symbols = self.scopes[self.root_scope()].get(name)?;

        let search_paths = match schema {
            Some(s) => std::slice::from_ref(s),
            None => self.search_path_at(position),
        };

        for search_schema in search_paths {
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
        position: TextSize,
        schema: &Option<Schema>,
        params: Option<&[Name]>,
    ) -> Option<SyntaxNodePtr> {
        let symbols = self.scopes[self.root_scope()].get(name)?;

        let search_paths = match schema {
            Some(s) => std::slice::from_ref(s),
            None => self.search_path_at(position),
        };

        for search_schema in search_paths {
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
        position: TextSize,
        schema: &Option<Schema>,
        table: &Option<Name>,
    ) -> Option<SyntaxNodePtr> {
        let symbols = self.scopes[self.root_scope()].get(name)?;

        let search_paths = match schema {
            Some(s) => std::slice::from_ref(s),
            None => self.search_path_at(position),
        };

        for search_schema in search_paths {
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
        name_str: String,
        schema: &Option<String>,
        kind: SymbolKind,
        position: TextSize,
    ) -> Option<(Schema, String)> {
        let name_normalized = Name::from_string(name_str.clone());
        let symbols = self.scopes[self.root_scope()].get(&name_normalized)?;

        let search_paths = match schema {
            Some(schema_name) => &[Schema::new(schema_name)],
            None => self.search_path_at(position),
        };

        for search_schema in search_paths {
            if let Some(symbol_id) = symbols.iter().copied().find(|id| {
                let symbol = &self.symbols[*id];
                symbol.kind == kind && symbol.schema.as_ref() == Some(search_schema)
            }) {
                let symbol = &self.symbols[symbol_id];
                return Some((symbol.schema.clone()?, name_str));
            }
        }
        None
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
        let root_scope = self.root_scope();
        let scope = &self.scopes[root_scope];

        let mut names = vec![];
        for (name, symbol_ids) in &scope.entries {
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
        let root_scope = self.root_scope();
        let scope = &self.scopes[root_scope];

        let mut names = vec![];
        for (name, symbol_ids) in &scope.entries {
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
        ast::Stmt::CreateTable(create_table) => bind_create_table(b, create_table),
        ast::Stmt::CreateTableAs(create_table_as) => bind_create_table_as(b, create_table_as),
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
        ast::Stmt::CreateTrigger(create_trigger) => bind_create_trigger(b, create_trigger),
        ast::Stmt::CreateEventTrigger(create_event_trigger) => {
            bind_create_event_trigger(b, create_event_trigger)
        }
        ast::Stmt::CreateTablespace(create_tablespace) => {
            bind_create_tablespace(b, create_tablespace)
        }
        ast::Stmt::CreateDatabase(create_database) => bind_create_database(b, create_database),
        ast::Stmt::CreateServer(create_server) => bind_create_server(b, create_server),
        ast::Stmt::CreateExtension(create_extension) => bind_create_extension(b, create_extension),
        ast::Stmt::CreateRole(create_role) => bind_create_role(b, create_role),
        ast::Stmt::Declare(declare) => bind_declare_cursor(b, declare),
        ast::Stmt::Prepare(prepare) => bind_prepare(b, prepare),
        ast::Stmt::Listen(listen) => bind_listen(b, listen),
        ast::Stmt::Set(set) => bind_set(b, set),
        ast::Stmt::CreatePolicy(create_policy) => bind_create_policy(b, create_policy),
        _ => {}
    }
}

fn bind_create_table(b: &mut Binder, create_table: impl ast::HasCreateTable) {
    let Some(path) = create_table.path() else {
        return;
    };
    let Some(table_name) = item_name(&path) else {
        return;
    };
    let name_ptr = path_to_ptr(&path);
    let is_temp = create_table.temp_token().is_some() || create_table.temporary_token().is_some();
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

    let root = b.root_scope();
    b.scopes[root].insert(table_name.clone(), table_id);
    b.scopes[root].insert(table_name, type_id);
}

fn bind_create_table_as(b: &mut Binder, create_table_as: ast::CreateTableAs) {
    let Some(path) = create_table_as.path() else {
        return;
    };
    let Some(table_name) = item_name(&path) else {
        return;
    };
    let name_ptr = path_to_ptr(&path);
    let is_temp =
        create_table_as.temp_token().is_some() || create_table_as.temporary_token().is_some();
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

    let root = b.root_scope();
    b.scopes[root].insert(table_name.clone(), table_id);
    b.scopes[root].insert(table_name, type_id);
}

fn bind_create_index(b: &mut Binder, create_index: ast::CreateIndex) {
    let Some(name) = create_index.name() else {
        return;
    };

    let index_name = Name::from_node(&name);
    let name_ptr = SyntaxNodePtr::new(name.syntax());

    let Some(schema) = b.current_search_path().first().cloned() else {
        return;
    };

    let index_id = b.symbols.alloc(Symbol {
        kind: SymbolKind::Index,
        ptr: name_ptr,
        schema: Some(schema),
        params: None,
        table: None,
    });

    let root = b.root_scope();
    b.scopes[root].insert(index_name, index_id);
}

fn bind_create_function(b: &mut Binder, create_function: ast::CreateFunction) {
    let Some(path) = create_function.path() else {
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

    let root = b.root_scope();
    b.scopes[root].insert(function_name, function_id);
}

fn bind_create_aggregate(b: &mut Binder, create_aggregate: ast::CreateAggregate) {
    let Some(path) = create_aggregate.path() else {
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

    let root = b.root_scope();
    b.scopes[root].insert(aggregate_name, aggregate_id);
}

fn bind_create_procedure(b: &mut Binder, create_procedure: ast::CreateProcedure) {
    let Some(path) = create_procedure.path() else {
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

    let root = b.root_scope();
    b.scopes[root].insert(procedure_name, procedure_id);
}

fn bind_create_schema(b: &mut Binder, create_schema: ast::CreateSchema) {
    let (schema_name, name_ptr) = if let Some(schema_name_node) = create_schema.name() {
        let schema_name = Name::from_node(&schema_name_node);
        let name_ptr = SyntaxNodePtr::new(schema_name_node.syntax());
        (schema_name, name_ptr)
    } else if let Some(name) = create_schema.role().and_then(|role| role.name()) {
        let schema_name = Name::from_node(&name);
        let name_ptr = SyntaxNodePtr::new(name.syntax());
        (schema_name, name_ptr)
    } else {
        return;
    };

    let schema_id = b.symbols.alloc(Symbol {
        kind: SymbolKind::Schema,
        ptr: name_ptr,
        schema: Some(Schema(schema_name.clone())),
        params: None,
        table: None,
    });

    let root = b.root_scope();
    b.scopes[root].insert(schema_name, schema_id);
}

fn bind_create_type(b: &mut Binder, create_type: ast::CreateType) {
    let Some(path) = create_type.path() else {
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

    let root = b.root_scope();
    b.scopes[root].insert(type_name.clone(), type_id);

    if create_type.range_token().is_some() {
        if let Some((multirange_name, multirange_ptr, multirange_schema)) =
            multirange_type_from_range(b, create_type, type_name, schema, name_ptr)
        {
            let multirange_id = b.symbols.alloc(Symbol {
                kind: SymbolKind::Type,
                ptr: multirange_ptr,
                schema: Some(multirange_schema),
                params: None,
                table: None,
            });
            b.scopes[root].insert(multirange_name, multirange_id);
        }
    }
}

fn bind_create_domain(b: &mut Binder, create_domain: ast::CreateDomain) {
    let Some(path) = create_domain.path() else {
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
        schema: Some(schema),
        params: None,
        table: None,
    });

    let root = b.root_scope();
    b.scopes[root].insert(domain_name, type_id);
}

fn multirange_type_from_range(
    b: &Binder,
    create_type: ast::CreateType,
    type_name: Name,
    schema: Schema,
    fallback_ptr: SyntaxNodePtr,
) -> Option<(Name, SyntaxNodePtr, Schema)> {
    if let Some(attribute_list) = create_type.attribute_list() {
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
                    && let Some(path) = path_type.path()
                    && let Some(multirange_name) = item_name(&path)
                {
                    let multirange_schema = if path.qualifier().is_some() {
                        schema_name(b, &path, false)?
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
    let Some(path) = create_view.path() else {
        return;
    };

    let Some(view_name) = item_name(&path) else {
        return;
    };

    let name_ptr = path_to_ptr(&path);
    let is_temp = create_view.temp_token().is_some() || create_view.temporary_token().is_some();

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

    let root = b.root_scope();
    b.scopes[root].insert(view_name, view_id);
}

// TODO: combine with create_view
fn bind_create_materialized_view(b: &mut Binder, create_view: ast::CreateMaterializedView) {
    let Some(path) = create_view.path() else {
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

    let root = b.root_scope();
    b.scopes[root].insert(view_name, view_id);
}

fn bind_create_sequence(b: &mut Binder, create_sequence: ast::CreateSequence) {
    let Some(path) = create_sequence.path() else {
        return;
    };

    let Some(sequence_name) = item_name(&path) else {
        return;
    };

    let name_ptr = path_to_ptr(&path);
    let is_temp =
        create_sequence.temp_token().is_some() || create_sequence.temporary_token().is_some();

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

    let root = b.root_scope();
    b.scopes[root].insert(sequence_name, sequence_id);
}

fn bind_create_trigger(b: &mut Binder, create_trigger: ast::CreateTrigger) {
    let Some(name) = create_trigger.name() else {
        return;
    };

    let trigger_name = Name::from_node(&name);
    let name_ptr = SyntaxNodePtr::new(name.syntax());

    let Some(table_path) = create_trigger.on_table().and_then(|on| on.path()) else {
        return;
    };

    let Some(table_name) = item_name(&table_path) else {
        return;
    };

    let Some(schema) = schema_name(b, &table_path, false) else {
        return;
    };

    let trigger_id = b.symbols.alloc(Symbol {
        kind: SymbolKind::Trigger,
        ptr: name_ptr,
        schema: Some(schema),
        params: None,
        table: Some(table_name),
    });

    let root = b.root_scope();
    b.scopes[root].insert(trigger_name, trigger_id);
}

fn bind_create_policy(b: &mut Binder, create_policy: ast::CreatePolicy) {
    let Some(name) = create_policy.name() else {
        return;
    };

    let policy_name = Name::from_node(&name);
    let name_ptr = SyntaxNodePtr::new(name.syntax());

    let Some(table_path) = create_policy.on_table().and_then(|on| on.path()) else {
        return;
    };

    let Some(table_name) = item_name(&table_path) else {
        return;
    };

    let Some(schema) = schema_name(b, &table_path, false) else {
        return;
    };

    let policy_id = b.symbols.alloc(Symbol {
        kind: SymbolKind::Policy,
        ptr: name_ptr,
        schema: Some(schema),
        params: None,
        table: Some(table_name),
    });

    let root = b.root_scope();
    b.scopes[root].insert(policy_name, policy_id);
}

fn bind_create_event_trigger(b: &mut Binder, create_event_trigger: ast::CreateEventTrigger) {
    let Some(name) = create_event_trigger.name() else {
        return;
    };

    let event_trigger_name = Name::from_node(&name);
    let name_ptr = SyntaxNodePtr::new(name.syntax());

    let event_trigger_id = b.symbols.alloc(Symbol {
        kind: SymbolKind::EventTrigger,
        ptr: name_ptr,
        schema: None,
        params: None,
        table: None,
    });

    let root = b.root_scope();
    b.scopes[root].insert(event_trigger_name, event_trigger_id);
}

fn bind_create_tablespace(b: &mut Binder, create_tablespace: ast::CreateTablespace) {
    let Some(name) = create_tablespace.name() else {
        return;
    };

    let tablespace_name = Name::from_node(&name);
    let name_ptr = SyntaxNodePtr::new(name.syntax());

    let tablespace_id = b.symbols.alloc(Symbol {
        kind: SymbolKind::Tablespace,
        ptr: name_ptr,
        schema: None,
        params: None,
        table: None,
    });

    let root = b.root_scope();
    b.scopes[root].insert(tablespace_name, tablespace_id);
}

fn bind_create_database(b: &mut Binder, create_database: ast::CreateDatabase) {
    let Some(name) = create_database.name() else {
        return;
    };

    let database_name = Name::from_node(&name);
    let name_ptr = SyntaxNodePtr::new(name.syntax());

    let database_id = b.symbols.alloc(Symbol {
        kind: SymbolKind::Database,
        ptr: name_ptr,
        schema: None,
        params: None,
        table: None,
    });

    let root = b.root_scope();
    b.scopes[root].insert(database_name, database_id);
}

fn bind_create_server(b: &mut Binder, create_server: ast::CreateServer) {
    let Some(name) = create_server.name() else {
        return;
    };

    let server_name = Name::from_node(&name);
    let name_ptr = SyntaxNodePtr::new(name.syntax());

    let server_id = b.symbols.alloc(Symbol {
        kind: SymbolKind::Server,
        ptr: name_ptr,
        schema: None,
        params: None,
        table: None,
    });

    let root = b.root_scope();
    b.scopes[root].insert(server_name, server_id);
}

fn bind_create_extension(b: &mut Binder, create_extension: ast::CreateExtension) {
    let Some(name) = create_extension.name() else {
        return;
    };

    let extension_name = Name::from_node(&name);
    let name_ptr = SyntaxNodePtr::new(name.syntax());

    let extension_id = b.symbols.alloc(Symbol {
        kind: SymbolKind::Extension,
        ptr: name_ptr,
        schema: None,
        params: None,
        table: None,
    });

    let root = b.root_scope();
    b.scopes[root].insert(extension_name, extension_id);
}

fn bind_create_role(b: &mut Binder, create_role: ast::CreateRole) {
    let Some(name) = create_role.name() else {
        return;
    };

    let role_name = Name::from_node(&name);
    let name_ptr = SyntaxNodePtr::new(name.syntax());

    let role_id = b.symbols.alloc(Symbol {
        kind: SymbolKind::Role,
        ptr: name_ptr,
        schema: None,
        params: None,
        table: None,
    });

    let root = b.root_scope();
    b.scopes[root].insert(role_name, role_id);
}

fn bind_declare_cursor(b: &mut Binder, declare: ast::Declare) {
    let Some(name) = declare.name() else {
        return;
    };

    let cursor_name = Name::from_node(&name);
    let name_ptr = SyntaxNodePtr::new(name.syntax());

    let cursor_id = b.symbols.alloc(Symbol {
        kind: SymbolKind::Cursor,
        ptr: name_ptr,
        schema: None,
        params: None,
        table: None,
    });

    let root = b.root_scope();
    b.scopes[root].insert(cursor_name, cursor_id);
}

fn bind_prepare(b: &mut Binder, prepare: ast::Prepare) {
    let Some(name) = prepare.name() else {
        return;
    };

    let statement_name = Name::from_node(&name);
    let name_ptr = SyntaxNodePtr::new(name.syntax());

    let statement_id = b.symbols.alloc(Symbol {
        kind: SymbolKind::PreparedStatement,
        ptr: name_ptr,
        schema: None,
        params: None,
        table: None,
    });

    let root = b.root_scope();
    b.scopes[root].insert(statement_name, statement_id);
}

fn bind_listen(b: &mut Binder, listen: ast::Listen) {
    let Some(name) = listen.name() else {
        return;
    };

    let channel_name = Name::from_node(&name);
    let name_ptr = SyntaxNodePtr::new(name.syntax());

    let channel_id = b.symbols.alloc(Symbol {
        kind: SymbolKind::Channel,
        ptr: name_ptr,
        schema: None,
        params: None,
        table: None,
    });

    let root = b.root_scope();
    b.scopes[root].insert(channel_name, channel_id);
}

fn item_name(path: &ast::Path) -> Option<Name> {
    let segment = path.segment()?;

    if let Some(name) = segment.name() {
        return Some(Name::from_node(&name));
    }
    if let Some(name) = segment.name_ref() {
        return Some(Name::from_node(&name));
    }

    None
}

fn path_to_ptr(path: &ast::Path) -> SyntaxNodePtr {
    if let Some(segment) = path.segment() {
        if let Some(name) = segment.name() {
            return SyntaxNodePtr::new(name.syntax());
        }
        if let Some(name_ref) = segment.name_ref() {
            return SyntaxNodePtr::new(name_ref.syntax());
        }
    }
    SyntaxNodePtr::new(path.syntax())
}

fn schema_name(b: &Binder, path: &ast::Path, is_temp: bool) -> Option<Schema> {
    if let Some(name_ref) = path
        .qualifier()
        .and_then(|q| q.segment())
        .and_then(|s| s.name_ref())
    {
        return Some(Schema(Name::from_node(&name_ref)));
    }

    if is_temp {
        return Some(Schema::new("pg_temp"));
    }

    b.current_search_path().first().cloned()
}

fn bind_set(b: &mut Binder, set: ast::Set) {
    let position = set.syntax().text_range().start();

    // `set schema` is an alternative to `set search_path`
    if set.schema_token().is_some() {
        if let Some(literal) = set.literal()
            && let Some(string_value) = extract_string_literal(&literal)
        {
            b.search_path_changes.push(SearchPathChange {
                position,
                search_path: vec![Schema::new(string_value)],
            });
        }
        return;
    }

    let Some(path) = set.path() else { return };

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
    if set.default_token().is_some() {
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
        for config_value in set.config_values() {
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
            && let Some(path) = path_type.path()
            && let Some(segment) = path.segment()
            && let Some(name_ref) = segment.name_ref()
        {
            params.push(Name::from_node(&name_ref));
        }
    }
    (!params.is_empty()).then_some(params)
}
