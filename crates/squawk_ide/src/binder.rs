/// Loosely based on TypeScript's binder
/// see: typescript-go/internal/binder/binder.go
use la_arena::Arena;
use squawk_syntax::{SyntaxNodePtr, ast, ast::AstNode};

use crate::scope::{Scope, ScopeId};
use crate::symbols::{Name, Schema, Symbol, SymbolKind};

pub(crate) struct Binder {
    pub(crate) scopes: Arena<Scope>,
    pub(crate) symbols: Arena<Symbol>,
}

impl Binder {
    fn new() -> Self {
        let mut scopes = Arena::new();
        let _root_scope = scopes.alloc(Scope::with_parent(None));
        Binder {
            scopes,
            symbols: Arena::new(),
        }
    }

    pub(crate) fn root_scope(&self) -> ScopeId {
        self.scopes
            .iter()
            .next()
            .map(|(id, _)| id)
            .expect("root scope must exist")
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
    if let ast::Stmt::CreateTable(create_table) = stmt {
        bind_create_table(b, create_table)
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
    let schema = schema_name(&path, is_temp);

    let table_id = b.symbols.alloc(Symbol {
        kind: SymbolKind::Table,
        ptr: name_ptr,
        schema,
    });

    let root = b.root_scope();
    b.scopes[root].insert(table_name, table_id);
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

fn schema_name(path: &ast::Path, is_temp: bool) -> Schema {
    let default_schema = if is_temp { "pg_temp" } else { "public" };

    let Some(segment) = path.qualifier().and_then(|q| q.segment()) else {
        return Schema::new(default_schema);
    };

    let schema_name = if let Some(name) = segment.name() {
        Name::new(name.syntax().text().to_string())
    } else if let Some(name_ref) = segment.name_ref() {
        Name::new(name_ref.syntax().text().to_string())
    } else {
        return Schema::new(default_schema);
    };

    Schema(schema_name)
}
