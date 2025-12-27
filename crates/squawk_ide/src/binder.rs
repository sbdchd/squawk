/// Loosely based on TypeScript's binder
/// see: typescript-go/internal/binder/binder.go
use la_arena::Arena;
use rowan::TextSize;
use squawk_syntax::{SyntaxNodePtr, ast, ast::AstNode};

use crate::scope::{Scope, ScopeId};
use crate::symbols::{Name, Schema, Symbol, SymbolKind};

pub(crate) struct SearchPathChange {
    position: TextSize,
    search_path: Vec<Schema>,
}

pub(crate) struct Binder {
    pub(crate) scopes: Arena<Scope>,
    pub(crate) symbols: Arena<Symbol>,
    pub(crate) search_path_changes: Vec<SearchPathChange>,
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
                search_path: vec![Schema::new("public"), Schema::new("pg_temp")],
            }],
        }
    }

    pub(crate) fn root_scope(&self) -> ScopeId {
        self.scopes
            .iter()
            .next()
            .map(|(id, _)| id)
            .expect("root scope must exist")
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
        ast::Stmt::CreateIndex(create_index) => bind_create_index(b, create_index),
        ast::Stmt::CreateFunction(create_function) => bind_create_function(b, create_function),
        ast::Stmt::Set(set) => bind_set(b, set),
        _ => {}
    }
}

fn bind_create_table(b: &mut Binder, create_table: ast::CreateTable) {
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
        schema,
    });

    let root = b.root_scope();
    b.scopes[root].insert(table_name, table_id);
}

fn bind_create_index(b: &mut Binder, create_index: ast::CreateIndex) {
    let Some(name) = create_index.name() else {
        return;
    };

    let index_name = Name::new(name.syntax().text().to_string());
    let name_ptr = SyntaxNodePtr::new(name.syntax());

    let Some(schema) = b.current_search_path().first().cloned() else {
        return;
    };

    let index_id = b.symbols.alloc(Symbol {
        kind: SymbolKind::Index,
        ptr: name_ptr,
        schema,
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

    let function_id = b.symbols.alloc(Symbol {
        kind: SymbolKind::Function,
        ptr: name_ptr,
        schema,
    });

    let root = b.root_scope();
    b.scopes[root].insert(function_name, function_id);
}

fn item_name(path: &ast::Path) -> Option<Name> {
    let segment = path.segment()?;

    if let Some(name) = segment.name() {
        return Some(Name::new(name.syntax().text().to_string()));
    }
    if let Some(name) = segment.name_ref() {
        return Some(Name::new(name.syntax().text().to_string()));
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
        return Some(Schema(Name::new(name_ref.syntax().text().to_string())));
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
            search_path: vec![Schema::new("public"), Schema::new("pg_temp")],
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

fn extract_string_literal(literal: &ast::Literal) -> Option<String> {
    let text = literal.syntax().text().to_string();

    if text.starts_with('\'') && text.ends_with('\'') && text.len() >= 2 {
        Some(text[1..text.len() - 1].to_string())
    } else {
        None
    }
}
