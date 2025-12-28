use rowan::{TextRange, TextSize};
use squawk_syntax::{
    SyntaxNodePtr,
    ast::{self, AstNode},
};

use crate::binder::Binder;
pub(crate) use crate::symbols::Schema;
use crate::symbols::{Name, SymbolKind};

#[derive(Debug)]
enum NameRefContext {
    DropTable,
    Table,
    DropIndex,
    DropFunction,
    CreateIndex,
    CreateIndexColumn,
    SelectFunctionCall,
    SelectFromTable,
    SelectColumn,
    InsertTable,
    InsertColumn,
    DeleteTable,
    DeleteWhereColumn,
}

pub(crate) fn resolve_name_ref(binder: &Binder, name_ref: &ast::NameRef) -> Option<SyntaxNodePtr> {
    let context = classify_name_ref_context(name_ref)?;

    match context {
        NameRefContext::DropTable
        | NameRefContext::Table
        | NameRefContext::CreateIndex
        | NameRefContext::InsertTable
        | NameRefContext::DeleteTable => {
            let path = find_containing_path(name_ref)?;
            let table_name = extract_table_name(&path)?;
            let schema = extract_schema_name(&path);
            let position = name_ref.syntax().text_range().start();
            resolve_table(binder, &table_name, &schema, position)
        }
        NameRefContext::SelectFromTable => {
            let table_name = Name::new(name_ref.syntax().text().to_string());
            let schema = if let Some(parent) = name_ref.syntax().parent()
                && let Some(field_expr) = ast::FieldExpr::cast(parent)
                && let Some(base) = field_expr.base()
                && let Some(schema_name_ref) = ast::NameRef::cast(base.syntax().clone())
            {
                let schema_text = schema_name_ref.syntax().text().to_string();
                Some(Schema(Name::new(schema_text)))
            } else {
                None
            };
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
        NameRefContext::DropFunction => {
            let path = find_containing_path(name_ref)?;
            let function_name = extract_table_name(&path)?;
            let schema = extract_schema_name(&path);
            let position = name_ref.syntax().text_range().start();
            resolve_function(binder, &function_name, &schema, position)
        }
        NameRefContext::SelectFunctionCall => {
            let schema = if let Some(parent_node) = name_ref.syntax().parent()
                && let Some(field_expr) = ast::FieldExpr::cast(parent_node)
            {
                let base = field_expr.base()?;
                let schema_name_ref = ast::NameRef::cast(base.syntax().clone())?;
                let schema_text = schema_name_ref.syntax().text().to_string();
                Some(Schema(Name::new(schema_text)))
            } else {
                None
            };
            let function_name = Name::new(name_ref.syntax().text().to_string());
            let position = name_ref.syntax().text_range().start();
            resolve_function(binder, &function_name, &schema, position)
        }
        NameRefContext::CreateIndexColumn => resolve_create_index_column(binder, name_ref),
        NameRefContext::SelectColumn => resolve_select_column(binder, name_ref),
        NameRefContext::InsertColumn => resolve_insert_column(binder, name_ref),
        NameRefContext::DeleteWhereColumn => resolve_delete_where_column(binder, name_ref),
    }
}

fn classify_name_ref_context(name_ref: &ast::NameRef) -> Option<NameRefContext> {
    let mut in_partition_item = false;
    let mut in_call_expr = false;
    let mut in_column_list = false;
    let mut in_where_clause = false;
    let mut in_from_clause = false;
    let mut in_target_list = false;

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
        if ast::DropFunction::can_cast(ancestor.kind()) {
            return Some(NameRefContext::DropFunction);
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
        if ast::CallExpr::can_cast(ancestor.kind()) {
            in_call_expr = true;
        }
        if ast::FromClause::can_cast(ancestor.kind()) {
            in_from_clause = true;
        }
        if ast::TargetList::can_cast(ancestor.kind()) {
            in_target_list = true;
        }
        if ast::Select::can_cast(ancestor.kind()) {
            if in_call_expr {
                return Some(NameRefContext::SelectFunctionCall);
            }
            if in_from_clause {
                return Some(NameRefContext::SelectFromTable);
            }
            if in_target_list {
                return Some(NameRefContext::SelectColumn);
            }
        }
        if ast::ColumnList::can_cast(ancestor.kind()) {
            in_column_list = true;
        }
        if ast::Insert::can_cast(ancestor.kind()) {
            if in_column_list {
                return Some(NameRefContext::InsertColumn);
            }
            return Some(NameRefContext::InsertTable);
        }
        if ast::WhereClause::can_cast(ancestor.kind()) {
            in_where_clause = true;
        }
        if ast::Delete::can_cast(ancestor.kind()) {
            if in_where_clause {
                return Some(NameRefContext::DeleteWhereColumn);
            }
            return Some(NameRefContext::DeleteTable);
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
    resolve_for_kind(binder, table_name, schema, position, SymbolKind::Table)
}

fn resolve_index(
    binder: &Binder,
    index_name: &Name,
    schema: &Option<Schema>,
    position: TextSize,
) -> Option<SyntaxNodePtr> {
    resolve_for_kind(binder, index_name, schema, position, SymbolKind::Index)
}

fn resolve_for_kind(
    binder: &Binder,
    name: &Name,
    schema: &Option<Schema>,
    position: TextSize,
    kind: SymbolKind,
) -> Option<SyntaxNodePtr> {
    let symbols = binder.scopes[binder.root_scope()].get(name)?;

    if let Some(schema) = schema {
        let symbol_id = symbols.iter().copied().find(|id| {
            let symbol = &binder.symbols[*id];
            symbol.kind == kind && &symbol.schema == schema
        })?;
        return Some(binder.symbols[symbol_id].ptr);
    } else {
        let search_path = binder.search_path_at(position);
        for search_schema in search_path {
            if let Some(symbol_id) = symbols.iter().copied().find(|id| {
                let symbol = &binder.symbols[*id];
                symbol.kind == kind && &symbol.schema == search_schema
            }) {
                return Some(binder.symbols[symbol_id].ptr);
            }
        }
    }
    None
}

fn resolve_function(
    binder: &Binder,
    function_name: &Name,
    schema: &Option<Schema>,
    position: TextSize,
) -> Option<SyntaxNodePtr> {
    resolve_for_kind(
        binder,
        function_name,
        schema,
        position,
        SymbolKind::Function,
    )
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

    let root = &name_ref.syntax().ancestors().last()?;
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

fn resolve_insert_column(binder: &Binder, name_ref: &ast::NameRef) -> Option<SyntaxNodePtr> {
    let column_name = Name::new(name_ref.syntax().text().to_string());

    let insert = name_ref.syntax().ancestors().find_map(ast::Insert::cast)?;
    let path = insert.path()?;

    let table_name = extract_table_name(&path)?;
    let schema = extract_schema_name(&path);
    let position = name_ref.syntax().text_range().start();

    let table_ptr = resolve_table(binder, &table_name, &schema, position)?;

    let root = &name_ref.syntax().ancestors().last()?;
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

fn resolve_select_column(binder: &Binder, name_ref: &ast::NameRef) -> Option<SyntaxNodePtr> {
    let column_name = Name::new(name_ref.syntax().text().to_string());

    let select = name_ref.syntax().ancestors().find_map(ast::Select::cast)?;
    let from_clause = select.from_clause()?;
    let from_item = from_clause.from_items().next()?;

    let (table_name, schema) = if let Some(name_ref_node) = from_item.name_ref() {
        (Name::new(name_ref_node.syntax().text().to_string()), None)
    } else {
        let field_expr = from_item.field_expr()?;
        let table_name = Name::new(field_expr.field()?.syntax().text().to_string());
        let schema_name_ref = match field_expr.base()? {
            ast::Expr::NameRef(name_ref) => name_ref,
            _ => return None,
        };
        let schema = Schema(Name::new(schema_name_ref.syntax().text().to_string()));
        (table_name, Some(schema))
    };

    let position = name_ref.syntax().text_range().start();
    let table_ptr = resolve_table(binder, &table_name, &schema, position)?;

    let root = &name_ref.syntax().ancestors().last()?;
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

fn resolve_delete_where_column(binder: &Binder, name_ref: &ast::NameRef) -> Option<SyntaxNodePtr> {
    let column_name = Name::new(name_ref.syntax().text().to_string());

    let delete = name_ref.syntax().ancestors().find_map(ast::Delete::cast)?;
    let relation_name = delete.relation_name()?;
    let path = relation_name.path()?;

    let table_name = extract_table_name(&path)?;
    let schema = extract_schema_name(&path);
    let position = name_ref.syntax().text_range().start();

    let table_ptr = resolve_table(binder, &table_name, &schema, position)?;

    let root = &name_ref.syntax().ancestors().last()?;
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

pub(crate) fn extract_column_name(col: &ast::Column) -> Option<Name> {
    let text = if let Some(name_ref) = col.name_ref() {
        name_ref.syntax().text().to_string()
    } else {
        let name = col.name()?;
        name.syntax().text().to_string()
    };
    Some(Name::new(text))
}

pub(crate) fn find_column_in_table(
    table_arg_list: &ast::TableArgList,
    col_name: &Name,
) -> Option<TextRange> {
    table_arg_list.args().find_map(|arg| {
        if let ast::TableArg::Column(column) = arg
            && let Some(name) = column.name()
            && Name::new(name.syntax().text().to_string()) == *col_name
        {
            Some(name.syntax().text_range())
        } else {
            None
        }
    })
}

pub(crate) fn resolve_insert_table_columns(
    file: &ast::SourceFile,
    binder: &Binder,
    insert: &ast::Insert,
) -> Option<ast::TableArgList> {
    let path = insert.path()?;
    let table_name = extract_table_name(&path)?;
    let schema = extract_schema_name(&path);
    let position = insert.syntax().text_range().start();

    let table_ptr = resolve_table(binder, &table_name, &schema, position)?;
    let root = file.syntax();
    let table_name_node = table_ptr.to_node(root);

    let create_table = table_name_node
        .ancestors()
        .find_map(ast::CreateTable::cast)?;

    create_table.table_arg_list()
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
