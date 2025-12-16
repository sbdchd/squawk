use squawk_syntax::{
    SyntaxNodePtr,
    ast::{self, AstNode},
};

use crate::binder::Binder;
use crate::symbols::{Name, Schema, SymbolKind};

#[derive(Debug)]
enum NameRefContext {
    DropTable,
}

pub(crate) fn resolve_name_ref(binder: &Binder, name_ref: &ast::NameRef) -> Option<SyntaxNodePtr> {
    let context = classify_name_ref_context(name_ref)?;

    match context {
        NameRefContext::DropTable => {
            let path = find_containing_path(name_ref)?;
            let table_name = extract_table_name(&path)?;
            let schema = extract_schema_name(&path);
            resolve_table(binder, &table_name, &schema)
        }
    }
}

fn classify_name_ref_context(name_ref: &ast::NameRef) -> Option<NameRefContext> {
    for ancestor in name_ref.syntax().ancestors() {
        if ast::DropTable::can_cast(ancestor.kind()) {
            return Some(NameRefContext::DropTable);
        }
    }

    None
}

fn resolve_table(
    binder: &Binder,
    table_name: &Name,
    schema: &Option<Schema>,
) -> Option<SyntaxNodePtr> {
    let symbols = binder.scopes[binder.root_scope()].get(table_name)?;

    if let Some(schema) = schema {
        let symbol_id = symbols.iter().copied().find(|id| {
            let symbol = &binder.symbols[*id];
            symbol.kind == SymbolKind::Table && &symbol.schema == schema
        })?;
        return Some(binder.symbols[symbol_id].ptr);
    } else {
        for search_schema in [Schema::new("pg_temp"), Schema::new("public")] {
            if let Some(symbol_id) = symbols.iter().copied().find(|id| {
                let symbol = &binder.symbols[*id];
                symbol.kind == SymbolKind::Table && symbol.schema == search_schema
            }) {
                return Some(binder.symbols[symbol_id].ptr);
            }
        }
    }
    None
}

fn find_containing_path(name_ref: &ast::NameRef) -> Option<ast::Path> {
    for ancestor in name_ref.syntax().ancestors() {
        if let Some(path) = ast::Path::cast(ancestor) {
            return Some(path);
        }
    }
    None
}

fn extract_table_name(path: &ast::Path) -> Option<Name> {
    let segment = path.segment()?;
    let name_ref = segment.name_ref()?;
    Some(Name::new(name_ref.syntax().text().to_string()))
}

fn extract_schema_name(path: &ast::Path) -> Option<Schema> {
    path.qualifier()
        .and_then(|q| q.segment())
        .and_then(|s| s.name_ref())
        .map(|name_ref| Schema(Name::new(name_ref.syntax().text().to_string())))
}
