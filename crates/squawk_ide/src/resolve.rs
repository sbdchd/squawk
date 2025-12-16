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

pub(crate) fn resolve_name_ref(
    binder: &Binder,
    name_ref: &ast::NameRef,
) -> Option<SyntaxNodePtr> {
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

fn resolve_table(binder: &Binder, table_name: &Name, schema: &Schema) -> Option<SyntaxNodePtr> {
    let symbol_id = binder.scopes[binder.root_scope()]
        .get(table_name)?
        .iter()
        .copied()
        .find(|id| {
            let symbol = &binder.symbols[*id];
            symbol.kind == SymbolKind::Table && &symbol.schema == schema
        })?;
    Some(binder.symbols[symbol_id].ptr)
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

fn extract_schema_name(path: &ast::Path) -> Schema {
    let Some(qualifier) = path.qualifier() else {
        return Schema::Public;
    };
    let Some(segment) = qualifier.segment() else {
        return Schema::Public;
    };
    let Some(name_ref) = segment.name_ref() else {
        return Schema::Public;
    };
    Schema::from_name(Name::new(name_ref.syntax().text().to_string()))
}
