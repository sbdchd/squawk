use rowan::TextSize;
use squawk_syntax::{
    SyntaxNodePtr,
    ast::{self, AstNode},
};

use crate::binder::Binder;
pub(crate) use crate::symbols::Schema;
use crate::symbols::{Name, SymbolKind};
use squawk_syntax::SyntaxNode;

#[derive(Debug)]
enum NameRefContext {
    DropTable,
    Table,
    DropIndex,
    CreateIndex,
    CreateIndexColumn,
}

pub(crate) fn resolve_name_ref(binder: &Binder, name_ref: &ast::NameRef) -> Option<SyntaxNodePtr> {
    let context = classify_name_ref_context(name_ref)?;

    match context {
        NameRefContext::DropTable | NameRefContext::Table | NameRefContext::CreateIndex => {
            let path = find_containing_path(name_ref)?;
            let table_name = extract_table_name(&path)?;
            let schema = extract_schema_name(&path);
            let position = name_ref.syntax().text_range().start();
            resolve_table(binder, &table_name, &schema, position)
        }
        NameRefContext::DropIndex => {
            let path = find_containing_path(name_ref)?;
            let index_name = extract_table_name(&path)?;
            let schema = extract_schema_name(&path);
            let position = name_ref.syntax().text_range().start();
            resolve_index(binder, &index_name, &schema, position)
        }
        NameRefContext::CreateIndexColumn => resolve_create_index_column(binder, name_ref),
    }
}

fn classify_name_ref_context(name_ref: &ast::NameRef) -> Option<NameRefContext> {
    let mut in_partition_item = false;

    for ancestor in name_ref.syntax().ancestors() {
        if ast::DropTable::can_cast(ancestor.kind()) {
            return Some(NameRefContext::DropTable);
        }
        if ast::Table::can_cast(ancestor.kind()) {
            return Some(NameRefContext::Table);
        }
        if ast::DropIndex::can_cast(ancestor.kind()) {
            return Some(NameRefContext::DropIndex);
        }
        if ast::PartitionItem::can_cast(ancestor.kind()) {
            in_partition_item = true;
        }
        if ast::CreateIndex::can_cast(ancestor.kind()) {
            if in_partition_item {
                return Some(NameRefContext::CreateIndexColumn);
            }
            return Some(NameRefContext::CreateIndex);
        }
    }

    None
}

fn resolve_table(
    binder: &Binder,
    table_name: &Name,
    schema: &Option<Schema>,
    position: TextSize,
) -> Option<SyntaxNodePtr> {
    let symbols = binder.scopes[binder.root_scope()].get(table_name)?;

    if let Some(schema) = schema {
        let symbol_id = symbols.iter().copied().find(|id| {
            let symbol = &binder.symbols[*id];
            symbol.kind == SymbolKind::Table && &symbol.schema == schema
        })?;
        return Some(binder.symbols[symbol_id].ptr);
    } else {
        let search_path = binder.search_path_at(position);
        for search_schema in search_path {
            if let Some(symbol_id) = symbols.iter().copied().find(|id| {
                let symbol = &binder.symbols[*id];
                symbol.kind == SymbolKind::Table && &symbol.schema == search_schema
            }) {
                return Some(binder.symbols[symbol_id].ptr);
            }
        }
    }
    None
}

fn resolve_index(
    binder: &Binder,
    index_name: &Name,
    schema: &Option<Schema>,
    position: TextSize,
) -> Option<SyntaxNodePtr> {
    let symbols = binder.scopes[binder.root_scope()].get(index_name)?;

    if let Some(schema) = schema {
        let symbol_id = symbols.iter().copied().find(|id| {
            let symbol = &binder.symbols[*id];
            symbol.kind == SymbolKind::Index && &symbol.schema == schema
        })?;
        return Some(binder.symbols[symbol_id].ptr);
    } else {
        let search_path = binder.search_path_at(position);
        for search_schema in search_path {
            if let Some(symbol_id) = symbols.iter().copied().find(|id| {
                let symbol = &binder.symbols[*id];
                symbol.kind == SymbolKind::Index && &symbol.schema == search_schema
            }) {
                return Some(binder.symbols[symbol_id].ptr);
            }
        }
    }
    None
}

fn resolve_create_index_column(binder: &Binder, name_ref: &ast::NameRef) -> Option<SyntaxNodePtr> {
    let column_name = Name::new(name_ref.syntax().text().to_string());

    let create_index = name_ref
        .syntax()
        .ancestors()
        .find_map(ast::CreateIndex::cast)?;
    let relation_name = create_index.relation_name()?;
    let path = relation_name.path()?;

    let table_name = extract_table_name(&path)?;
    let schema = extract_schema_name(&path);
    let position = name_ref.syntax().text_range().start();

    let table_ptr = resolve_table(binder, &table_name, &schema, position)?;

    let root: &SyntaxNode = &name_ref.syntax().ancestors().last()?;
    let table_name_node = table_ptr.to_node(root);

    let create_table = table_name_node
        .ancestors()
        .find_map(ast::CreateTable::cast)?;

    let table_arg_list = create_table.table_arg_list()?;

    for arg in table_arg_list.args() {
        if let ast::TableArg::Column(column) = arg
            && let Some(col_name) = column.name()
            && Name::new(col_name.syntax().text().to_string()) == column_name
        {
            return Some(SyntaxNodePtr::new(col_name.syntax()));
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

pub(crate) fn resolve_table_info(binder: &Binder, path: &ast::Path) -> Option<(Schema, String)> {
    let table_name_str = extract_table_name_from_path(path)?;
    let schema = extract_schema_from_path(path);

    let table_name_normalized = Name::new(table_name_str.clone());
    let symbols = binder.scopes[binder.root_scope()].get(&table_name_normalized)?;

    if let Some(schema_name) = schema {
        let schema_normalized = Schema::new(schema_name);
        let symbol_id = symbols.iter().copied().find(|id| {
            let symbol = &binder.symbols[*id];
            symbol.kind == SymbolKind::Table && symbol.schema == schema_normalized
        })?;
        let symbol = &binder.symbols[symbol_id];
        return Some((symbol.schema.clone(), table_name_str));
    } else {
        let position = path.syntax().text_range().start();
        let search_path = binder.search_path_at(position);
        for search_schema in search_path {
            if let Some(symbol_id) = symbols.iter().copied().find(|id| {
                let symbol = &binder.symbols[*id];
                symbol.kind == SymbolKind::Table && &symbol.schema == search_schema
            }) {
                let symbol = &binder.symbols[symbol_id];
                return Some((symbol.schema.clone(), table_name_str));
            }
        }
    }
    None
}

fn extract_table_name_from_path(path: &ast::Path) -> Option<String> {
    let segment = path.segment()?;
    if let Some(name_ref) = segment.name_ref() {
        return Some(name_ref.syntax().text().to_string());
    }
    if let Some(name) = segment.name() {
        return Some(name.syntax().text().to_string());
    }
    None
}

fn extract_schema_from_path(path: &ast::Path) -> Option<String> {
    path.qualifier()
        .and_then(|q| q.segment())
        .and_then(|s| s.name_ref())
        .map(|name_ref| name_ref.syntax().text().to_string())
}
