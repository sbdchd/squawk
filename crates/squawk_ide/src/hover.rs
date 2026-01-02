use crate::offsets::token_from_offset;
use crate::resolve;
use crate::{binder, symbols::Name};
use rowan::TextSize;
use squawk_syntax::ast::{self, AstNode};

pub fn hover(file: &ast::SourceFile, offset: TextSize) -> Option<String> {
    let token = token_from_offset(file, offset)?;
    let parent = token.parent()?;

    let binder = binder::bind(file);

    // TODO: can we use the classify_name_ref_context function from goto def here?
    if let Some(name_ref) = ast::NameRef::cast(parent.clone()) {
        if is_column_ref(&name_ref) {
            return hover_column(file, &name_ref, &binder);
        }

        if is_type_ref(&name_ref) {
            return hover_type(file, &name_ref, &binder);
        }

        if is_select_column(&name_ref) {
            // Try hover as column first
            if let Some(result) = hover_column(file, &name_ref, &binder) {
                return Some(result);
            }
            // If no column, try as function (handles field-style function calls like `t.b`)
            if let Some(result) = hover_function(file, &name_ref, &binder) {
                return Some(result);
            }
            // Finally try as table (handles case like `select t from t;` where t is the table)
            return hover_table(file, &name_ref, &binder);
        }

        if is_table_ref(&name_ref) {
            return hover_table(file, &name_ref, &binder);
        }

        if is_select_from_table(&name_ref) {
            return hover_table(file, &name_ref, &binder);
        }

        if is_update_from_table(&name_ref) {
            return hover_table(file, &name_ref, &binder);
        }

        if is_index_ref(&name_ref) {
            return hover_index(file, &name_ref, &binder);
        }

        if is_function_ref(&name_ref) {
            return hover_function(file, &name_ref, &binder);
        }

        if is_aggregate_ref(&name_ref) {
            return hover_aggregate(file, &name_ref, &binder);
        }

        if is_procedure_ref(&name_ref) {
            return hover_procedure(file, &name_ref, &binder);
        }

        if is_routine_ref(&name_ref) {
            return hover_routine(file, &name_ref, &binder);
        }

        if is_call_procedure(&name_ref) {
            return hover_procedure(file, &name_ref, &binder);
        }

        if is_select_function_call(&name_ref) {
            // Try function first, but fall back to column if no function found
            // (handles function-call-style column access like `select a(t)`)
            if let Some(result) = hover_function(file, &name_ref, &binder) {
                return Some(result);
            }
            return hover_column(file, &name_ref, &binder);
        }

        if is_schema_ref(&name_ref) {
            return hover_schema(file, &name_ref, &binder);
        }
    }

    if let Some(name) = ast::Name::cast(parent) {
        if let Some(column) = name.syntax().parent().and_then(ast::Column::cast)
            && let Some(create_table) = column.syntax().ancestors().find_map(ast::CreateTable::cast)
        {
            return hover_column_definition(&create_table, &column, &binder);
        }

        if let Some(create_table) = name.syntax().ancestors().find_map(ast::CreateTable::cast) {
            return format_create_table(&create_table, &binder);
        }

        if let Some(with_table) = name.syntax().parent().and_then(ast::WithTable::cast) {
            return format_with_table(&with_table);
        }

        if let Some(create_index) = name.syntax().ancestors().find_map(ast::CreateIndex::cast) {
            return format_create_index(&create_index, &binder);
        }

        if let Some(create_type) = name.syntax().ancestors().find_map(ast::CreateType::cast) {
            return format_create_type(&create_type, &binder);
        }

        if let Some(create_function) = name
            .syntax()
            .ancestors()
            .find_map(ast::CreateFunction::cast)
        {
            return format_create_function(&create_function, &binder);
        }

        if let Some(create_aggregate) = name
            .syntax()
            .ancestors()
            .find_map(ast::CreateAggregate::cast)
        {
            return format_create_aggregate(&create_aggregate, &binder);
        }

        if let Some(create_procedure) = name
            .syntax()
            .ancestors()
            .find_map(ast::CreateProcedure::cast)
        {
            return format_create_procedure(&create_procedure, &binder);
        }

        if let Some(create_schema) = name.syntax().ancestors().find_map(ast::CreateSchema::cast) {
            return format_create_schema(&create_schema);
        }

        // create view t(x) as select 1;
        //               ^
        if let Some(column_list) = name.syntax().ancestors().find_map(ast::ColumnList::cast)
            && let Some(create_view) = column_list
                .syntax()
                .ancestors()
                .find_map(ast::CreateView::cast)
        {
            return format_view_column(&create_view, Name::from_node(&name), &binder);
        }

        // create view t as select 1;
        //             ^
        if let Some(create_view) = name.syntax().ancestors().find_map(ast::CreateView::cast) {
            return format_create_view(&create_view, &binder);
        }
    }

    None
}

fn format_column(
    schema: &str,
    table_name: &str,
    column_name: &str,
    ty: &impl std::fmt::Display,
) -> String {
    format!("column {schema}.{table_name}.{column_name} {ty}")
}

fn hover_column(
    file: &ast::SourceFile,
    name_ref: &ast::NameRef,
    binder: &binder::Binder,
) -> Option<String> {
    let column_ptr = resolve::resolve_name_ref(binder, name_ref)?;

    let root = file.syntax();
    let column_name_node = column_ptr.to_node(root);

    if let Some(with_table) = column_name_node.ancestors().find_map(ast::WithTable::cast) {
        let cte_name = with_table.name()?.syntax().text().to_string();
        let column_name = if column_name_node
            .ancestors()
            .any(|a| ast::Values::can_cast(a.kind()))
        {
            Name::from_node(name_ref)
        } else {
            Name::from_string(column_name_node.text().to_string())
        };
        return Some(format!("column {}.{}", cte_name, column_name));
    }

    // create view v(a) as select 1;
    // select a from v;
    //        ^
    if let Some(create_view) = column_name_node.ancestors().find_map(ast::CreateView::cast)
        && let Some(column_name) =
            ast::Name::cast(column_name_node.clone()).map(|name| Name::from_node(&name))
    {
        return format_view_column(&create_view, column_name, binder);
    }

    let column = column_name_node.ancestors().find_map(ast::Column::cast)?;
    let column_name = column.name()?.syntax().text().to_string();
    let ty = column.ty()?;

    let create_table = column
        .syntax()
        .ancestors()
        .find_map(ast::CreateTable::cast)?;
    let path = create_table.path()?;
    let table_name = path.segment()?.name()?.syntax().text().to_string();

    let schema = if let Some(qualifier) = path.qualifier() {
        qualifier.syntax().text().to_string()
    } else {
        table_schema(&create_table, binder)?
    };

    Some(format_column(
        &schema,
        &table_name,
        &column_name,
        &ty.syntax().text(),
    ))
}

fn hover_column_definition(
    create_table: &ast::CreateTable,
    column: &ast::Column,
    binder: &binder::Binder,
) -> Option<String> {
    let column_name = column.name()?.syntax().text().to_string();
    let ty = column.ty()?;
    let path = create_table.path()?;
    let table_name = path.segment()?.name()?.syntax().text().to_string();

    let schema = if let Some(qualifier) = path.qualifier() {
        qualifier.syntax().text().to_string()
    } else {
        table_schema(create_table, binder)?
    };

    Some(format_column(
        &schema,
        &table_name,
        &column_name,
        &ty.syntax().text(),
    ))
}

fn hover_table(
    file: &ast::SourceFile,
    name_ref: &ast::NameRef,
    binder: &binder::Binder,
) -> Option<String> {
    let table_ptr = resolve::resolve_name_ref(binder, name_ref)?;

    let root = file.syntax();
    let table_name_node = table_ptr.to_node(root);

    if let Some(with_table) = table_name_node.ancestors().find_map(ast::WithTable::cast) {
        return format_with_table(&with_table);
    }

    // create view v as select 1 a;
    // select a from v;
    //               ^
    if let Some(create_view) = table_name_node.ancestors().find_map(ast::CreateView::cast) {
        return format_create_view(&create_view, binder);
    }

    let create_table = table_name_node
        .ancestors()
        .find_map(ast::CreateTable::cast)?;

    format_create_table(&create_table, binder)
}

fn hover_index(
    file: &ast::SourceFile,
    name_ref: &ast::NameRef,
    binder: &binder::Binder,
) -> Option<String> {
    let index_ptr = resolve::resolve_name_ref(binder, name_ref)?;

    let root = file.syntax();
    let index_name_node = index_ptr.to_node(root);

    let create_index = index_name_node
        .ancestors()
        .find_map(ast::CreateIndex::cast)?;

    format_create_index(&create_index, binder)
}

fn hover_type(
    file: &ast::SourceFile,
    name_ref: &ast::NameRef,
    binder: &binder::Binder,
) -> Option<String> {
    let type_ptr = resolve::resolve_name_ref(binder, name_ref)?;

    let root = file.syntax();
    let type_name_node = type_ptr.to_node(root);

    let create_type = type_name_node.ancestors().find_map(ast::CreateType::cast)?;

    format_create_type(&create_type, binder)
}

// Insert inferred schema into the create table hover info
fn format_create_table(create_table: &ast::CreateTable, binder: &binder::Binder) -> Option<String> {
    let path = create_table.path()?;
    let segment = path.segment()?;
    let table_name = segment.name()?.syntax().text().to_string();

    let schema = if let Some(qualifier) = path.qualifier() {
        qualifier.syntax().text().to_string()
    } else {
        table_schema(create_table, binder)?
    };

    let args = create_table.table_arg_list()?.syntax().text().to_string();

    Some(format!("table {}.{}{}", schema, table_name, args))
}

fn format_create_view(create_view: &ast::CreateView, binder: &binder::Binder) -> Option<String> {
    let path = create_view.path()?;
    let segment = path.segment()?;
    let view_name = segment.name()?.syntax().text().to_string();

    let schema = if let Some(qualifier) = path.qualifier() {
        qualifier.syntax().text().to_string()
    } else {
        view_schema(create_view, binder)?
    };

    let column_list = create_view
        .column_list()
        .map(|cl| cl.syntax().text().to_string())
        .unwrap_or_default();

    let query = create_view.query()?.syntax().text().to_string();

    Some(format!(
        "view {}.{}{} as {}",
        schema, view_name, column_list, query
    ))
}

fn format_view_column(
    create_view: &ast::CreateView,
    column_name: Name,
    binder: &binder::Binder,
) -> Option<String> {
    let path = create_view.path()?;
    let segment = path.segment()?;
    let view_name = Name::from_node(&segment.name()?);

    let schema = if let Some(qualifier) = path.qualifier() {
        Name::from_string(qualifier.syntax().text().to_string())
    } else {
        Name::from_string(view_schema(create_view, binder)?)
    };

    Some(format!("column {}.{}.{}", schema, view_name, column_name))
}

fn format_with_table(with_table: &ast::WithTable) -> Option<String> {
    let name = with_table.name()?.syntax().text().to_string();
    let query = with_table.query()?.syntax().text().to_string();
    Some(format!("with {} as ({})", name, query))
}

fn table_schema(create_table: &ast::CreateTable, binder: &binder::Binder) -> Option<String> {
    let is_temp = create_table.temp_token().is_some() || create_table.temporary_token().is_some();
    if is_temp {
        return Some("pg_temp".to_string());
    }

    let position = create_table.syntax().text_range().start();
    let search_path = binder.search_path_at(position);
    search_path.first().map(|s| s.to_string())
}

fn view_schema(create_view: &ast::CreateView, binder: &binder::Binder) -> Option<String> {
    let is_temp = create_view.temp_token().is_some() || create_view.temporary_token().is_some();
    if is_temp {
        return Some("pg_temp".to_string());
    }

    let position = create_view.syntax().text_range().start();
    let search_path = binder.search_path_at(position);
    search_path.first().map(|s| s.to_string())
}

fn format_create_index(create_index: &ast::CreateIndex, binder: &binder::Binder) -> Option<String> {
    let index_name = create_index.name()?.syntax().text().to_string();

    let index_schema = index_schema(create_index, binder)?;

    let relation_name = create_index.relation_name()?;
    let path = relation_name.path()?;
    let (table_schema, table_name) = resolve::resolve_table_info(binder, &path)?;

    let partition_item_list = create_index.partition_item_list()?;
    let columns = partition_item_list.syntax().text().to_string();

    Some(format!(
        "index {}.{} on {}.{}{}",
        index_schema, index_name, table_schema, table_name, columns
    ))
}

fn index_schema(create_index: &ast::CreateIndex, binder: &binder::Binder) -> Option<String> {
    let position = create_index.syntax().text_range().start();
    let search_path = binder.search_path_at(position);
    search_path.first().map(|s| s.to_string())
}

fn format_create_type(create_type: &ast::CreateType, binder: &binder::Binder) -> Option<String> {
    let path = create_type.path()?;
    let segment = path.segment()?;
    let type_name = segment.name()?.syntax().text().to_string();

    let schema = if let Some(qualifier) = path.qualifier() {
        qualifier.syntax().text().to_string()
    } else {
        type_schema(create_type, binder)?
    };

    if let Some(variant_list) = create_type.variant_list() {
        let variants = variant_list.syntax().text().to_string();
        return Some(format!(
            "type {}.{} as enum {}",
            schema, type_name, variants
        ));
    }

    if let Some(column_list) = create_type.column_list() {
        let columns = column_list.syntax().text().to_string();
        return Some(format!("type {}.{} as {}", schema, type_name, columns));
    }

    if let Some(attribute_list) = create_type.attribute_list() {
        let attributes = attribute_list.syntax().text().to_string();
        return Some(format!("type {}.{} {}", schema, type_name, attributes));
    }

    Some(format!("type {}.{}", schema, type_name))
}

fn type_schema(create_type: &ast::CreateType, binder: &binder::Binder) -> Option<String> {
    let position = create_type.syntax().text_range().start();
    let search_path = binder.search_path_at(position);
    search_path.first().map(|s| s.to_string())
}

fn is_column_ref(name_ref: &ast::NameRef) -> bool {
    let mut in_partition_item = false;
    let mut in_column_list = false;
    let mut in_where_clause = false;
    let mut in_set_clause = false;

    for ancestor in name_ref.syntax().ancestors() {
        if ast::PartitionItem::can_cast(ancestor.kind()) {
            in_partition_item = true;
        }
        if ast::CreateIndex::can_cast(ancestor.kind()) {
            return in_partition_item;
        }
        if ast::ColumnList::can_cast(ancestor.kind()) {
            in_column_list = true;
        }
        if ast::Insert::can_cast(ancestor.kind()) {
            return in_column_list;
        }
        if ast::WhereClause::can_cast(ancestor.kind()) {
            in_where_clause = true;
        }
        if ast::SetClause::can_cast(ancestor.kind()) {
            in_set_clause = true;
        }
        if ast::Delete::can_cast(ancestor.kind()) {
            return in_where_clause;
        }
        if ast::Update::can_cast(ancestor.kind()) {
            return in_where_clause || in_set_clause;
        }
    }
    false
}

fn is_table_ref(name_ref: &ast::NameRef) -> bool {
    let mut in_partition_item = false;
    let mut in_column_list = false;
    let mut in_where_clause = false;
    let mut in_set_clause = false;
    let mut in_from_clause = false;

    for ancestor in name_ref.syntax().ancestors() {
        if ast::DropTable::can_cast(ancestor.kind()) {
            return true;
        }
        if ast::DropView::can_cast(ancestor.kind()) {
            return true;
        }
        if ast::Table::can_cast(ancestor.kind()) {
            return true;
        }
        if ast::ColumnList::can_cast(ancestor.kind()) {
            in_column_list = true;
        }
        if ast::Insert::can_cast(ancestor.kind()) {
            return !in_column_list;
        }
        if ast::WhereClause::can_cast(ancestor.kind()) {
            in_where_clause = true;
        }
        if ast::SetClause::can_cast(ancestor.kind()) {
            in_set_clause = true;
        }
        if ast::FromClause::can_cast(ancestor.kind()) {
            in_from_clause = true;
        }
        if ast::Delete::can_cast(ancestor.kind()) {
            return !in_where_clause;
        }
        if ast::Update::can_cast(ancestor.kind()) {
            return !in_where_clause && !in_set_clause && !in_from_clause;
        }
        if ast::DropIndex::can_cast(ancestor.kind()) {
            return false;
        }
        if ast::PartitionItem::can_cast(ancestor.kind()) {
            in_partition_item = true;
        }
        if ast::CreateIndex::can_cast(ancestor.kind()) {
            return !in_partition_item;
        }
    }
    false
}

fn is_index_ref(name_ref: &ast::NameRef) -> bool {
    for ancestor in name_ref.syntax().ancestors() {
        if ast::DropIndex::can_cast(ancestor.kind()) {
            return true;
        }
    }
    false
}

fn is_type_ref(name_ref: &ast::NameRef) -> bool {
    let mut in_type = false;
    for ancestor in name_ref.syntax().ancestors() {
        if ast::PathType::can_cast(ancestor.kind()) || ast::ExprType::can_cast(ancestor.kind()) {
            in_type = true;
        }
        if ast::DropType::can_cast(ancestor.kind()) {
            return true;
        }
        if ast::CastExpr::can_cast(ancestor.kind()) && in_type {
            return true;
        }
    }
    false
}

fn is_function_ref(name_ref: &ast::NameRef) -> bool {
    for ancestor in name_ref.syntax().ancestors() {
        if ast::DropFunction::can_cast(ancestor.kind()) {
            return true;
        }
    }
    false
}

fn is_aggregate_ref(name_ref: &ast::NameRef) -> bool {
    for ancestor in name_ref.syntax().ancestors() {
        if ast::DropAggregate::can_cast(ancestor.kind()) {
            return true;
        }
    }
    false
}

fn is_procedure_ref(name_ref: &ast::NameRef) -> bool {
    for ancestor in name_ref.syntax().ancestors() {
        if ast::DropProcedure::can_cast(ancestor.kind()) {
            return true;
        }
    }
    false
}

fn is_routine_ref(name_ref: &ast::NameRef) -> bool {
    for ancestor in name_ref.syntax().ancestors() {
        if ast::DropRoutine::can_cast(ancestor.kind()) {
            return true;
        }
    }
    false
}

fn is_call_procedure(name_ref: &ast::NameRef) -> bool {
    for ancestor in name_ref.syntax().ancestors() {
        if ast::Call::can_cast(ancestor.kind()) {
            return true;
        }
    }
    false
}

fn is_select_function_call(name_ref: &ast::NameRef) -> bool {
    let mut in_call_expr = false;
    let mut in_arg_list = false;

    for ancestor in name_ref.syntax().ancestors() {
        if ast::ArgList::can_cast(ancestor.kind()) {
            in_arg_list = true;
        }
        if ast::CallExpr::can_cast(ancestor.kind()) {
            in_call_expr = true;
        }
        if ast::Select::can_cast(ancestor.kind()) && in_call_expr && !in_arg_list {
            return true;
        }
    }
    false
}

fn is_select_from_table(name_ref: &ast::NameRef) -> bool {
    let mut in_from_clause = false;

    for ancestor in name_ref.syntax().ancestors() {
        if ast::FromClause::can_cast(ancestor.kind()) {
            in_from_clause = true;
        }
        if ast::Select::can_cast(ancestor.kind()) && in_from_clause {
            return true;
        }
    }
    false
}

fn is_update_from_table(name_ref: &ast::NameRef) -> bool {
    let mut in_from_clause = false;

    for ancestor in name_ref.syntax().ancestors() {
        if ast::FromClause::can_cast(ancestor.kind()) {
            in_from_clause = true;
        }
        if ast::Update::can_cast(ancestor.kind()) && in_from_clause {
            return true;
        }
    }
    false
}

fn is_select_column(name_ref: &ast::NameRef) -> bool {
    let mut in_call_expr = false;
    let mut in_arg_list = false;
    let mut in_from_clause = false;

    for ancestor in name_ref.syntax().ancestors() {
        if ast::ArgList::can_cast(ancestor.kind()) {
            in_arg_list = true;
        }
        if ast::CallExpr::can_cast(ancestor.kind()) {
            in_call_expr = true;
        }
        if ast::FromClause::can_cast(ancestor.kind()) {
            in_from_clause = true;
        }
        if ast::Select::can_cast(ancestor.kind()) {
            // if we're inside a call expr but not inside an arg list, this is a function call
            if in_call_expr && !in_arg_list {
                return false;
            }
            // if we're in FROM clause, this is a table reference, not a column
            if in_from_clause {
                return false;
            }
            // anything else in SELECT (target list, WHERE, ORDER BY, etc.) is a column
            return true;
        }
    }
    false
}

fn is_schema_ref(name_ref: &ast::NameRef) -> bool {
    for ancestor in name_ref.syntax().ancestors() {
        if ast::DropSchema::can_cast(ancestor.kind()) {
            return true;
        }
    }
    false
}

fn hover_schema(
    file: &ast::SourceFile,
    name_ref: &ast::NameRef,
    binder: &binder::Binder,
) -> Option<String> {
    let schema_ptr = resolve::resolve_name_ref(binder, name_ref)?;

    let root = file.syntax();
    let schema_name_node = schema_ptr.to_node(root);

    let create_schema = ast::CreateSchema::cast(schema_name_node.parent()?)?;

    format_create_schema(&create_schema)
}

fn format_create_schema(create_schema: &ast::CreateSchema) -> Option<String> {
    let schema_name = create_schema.name()?.syntax().text().to_string();
    Some(format!("schema {}", schema_name))
}

fn hover_function(
    file: &ast::SourceFile,
    name_ref: &ast::NameRef,
    binder: &binder::Binder,
) -> Option<String> {
    let function_ptr = resolve::resolve_name_ref(binder, name_ref)?;

    let root = file.syntax();
    let function_name_node = function_ptr.to_node(root);

    let create_function = function_name_node
        .ancestors()
        .find_map(ast::CreateFunction::cast)?;

    format_create_function(&create_function, binder)
}

fn format_create_function(
    create_function: &ast::CreateFunction,
    binder: &binder::Binder,
) -> Option<String> {
    let path = create_function.path()?;
    let segment = path.segment()?;
    let name = segment.name()?;
    let function_name = name.syntax().text().to_string();

    let schema = if let Some(qualifier) = path.qualifier() {
        qualifier.syntax().text().to_string()
    } else {
        function_schema(create_function, binder)?
    };

    let param_list = create_function.param_list()?;
    let params = param_list.syntax().text().to_string();

    let ret_type = create_function.ret_type()?;
    let return_type = ret_type.syntax().text().to_string();

    Some(format!(
        "function {}.{}{} {}",
        schema, function_name, params, return_type
    ))
}

fn function_schema(
    create_function: &ast::CreateFunction,
    binder: &binder::Binder,
) -> Option<String> {
    let position = create_function.syntax().text_range().start();
    let search_path = binder.search_path_at(position);
    search_path.first().map(|s| s.to_string())
}

fn hover_aggregate(
    file: &ast::SourceFile,
    name_ref: &ast::NameRef,
    binder: &binder::Binder,
) -> Option<String> {
    let aggregate_ptr = resolve::resolve_name_ref(binder, name_ref)?;

    let root = file.syntax();
    let aggregate_name_node = aggregate_ptr.to_node(root);

    let create_aggregate = aggregate_name_node
        .ancestors()
        .find_map(ast::CreateAggregate::cast)?;

    format_create_aggregate(&create_aggregate, binder)
}

fn format_create_aggregate(
    create_aggregate: &ast::CreateAggregate,
    binder: &binder::Binder,
) -> Option<String> {
    let path = create_aggregate.path()?;
    let segment = path.segment()?;
    let name = segment.name()?;
    let aggregate_name = name.syntax().text().to_string();

    let schema = if let Some(qualifier) = path.qualifier() {
        qualifier.syntax().text().to_string()
    } else {
        aggregate_schema(create_aggregate, binder)?
    };

    let param_list = create_aggregate.param_list()?;
    let params = param_list.syntax().text().to_string();

    Some(format!("aggregate {}.{}{}", schema, aggregate_name, params))
}

fn aggregate_schema(
    create_aggregate: &ast::CreateAggregate,
    binder: &binder::Binder,
) -> Option<String> {
    let position = create_aggregate.syntax().text_range().start();
    let search_path = binder.search_path_at(position);
    search_path.first().map(|s| s.to_string())
}

fn hover_procedure(
    file: &ast::SourceFile,
    name_ref: &ast::NameRef,
    binder: &binder::Binder,
) -> Option<String> {
    let procedure_ptr = resolve::resolve_name_ref(binder, name_ref)?;

    let root = file.syntax();
    let procedure_name_node = procedure_ptr.to_node(root);

    let create_procedure = procedure_name_node
        .ancestors()
        .find_map(ast::CreateProcedure::cast)?;

    format_create_procedure(&create_procedure, binder)
}

fn format_create_procedure(
    create_procedure: &ast::CreateProcedure,
    binder: &binder::Binder,
) -> Option<String> {
    let path = create_procedure.path()?;
    let segment = path.segment()?;
    let name = segment.name()?;
    let procedure_name = name.syntax().text().to_string();

    let schema = if let Some(qualifier) = path.qualifier() {
        qualifier.syntax().text().to_string()
    } else {
        procedure_schema(create_procedure, binder)?
    };

    let param_list = create_procedure.param_list()?;
    let params = param_list.syntax().text().to_string();

    Some(format!("procedure {}.{}{}", schema, procedure_name, params))
}

fn procedure_schema(
    create_procedure: &ast::CreateProcedure,
    binder: &binder::Binder,
) -> Option<String> {
    let position = create_procedure.syntax().text_range().start();
    let search_path = binder.search_path_at(position);
    search_path.first().map(|s| s.to_string())
}

fn hover_routine(
    file: &ast::SourceFile,
    name_ref: &ast::NameRef,
    binder: &binder::Binder,
) -> Option<String> {
    let routine_ptr = resolve::resolve_name_ref(binder, name_ref)?;

    let root = file.syntax();
    let routine_name_node = routine_ptr.to_node(root);

    if let Some(create_function) = routine_name_node
        .ancestors()
        .find_map(ast::CreateFunction::cast)
    {
        return format_create_function(&create_function, binder);
    }

    if let Some(create_aggregate) = routine_name_node
        .ancestors()
        .find_map(ast::CreateAggregate::cast)
    {
        return format_create_aggregate(&create_aggregate, binder);
    }

    if let Some(create_procedure) = routine_name_node
        .ancestors()
        .find_map(ast::CreateProcedure::cast)
    {
        return format_create_procedure(&create_procedure, binder);
    }

    None
}

#[cfg(test)]
mod test {
    use crate::hover::hover;
    use crate::test_utils::fixture;
    use annotate_snippets::{AnnotationKind, Level, Renderer, Snippet, renderer::DecorStyle};
    use insta::assert_snapshot;
    use squawk_syntax::ast;

    #[track_caller]
    fn check_hover(sql: &str) -> String {
        check_hover_(sql).expect("should find hover information")
    }

    #[track_caller]
    fn check_hover_(sql: &str) -> Option<String> {
        let (mut offset, sql) = fixture(sql);
        offset = offset.checked_sub(1.into()).unwrap_or_default();
        let parse = ast::SourceFile::parse(&sql);
        assert_eq!(parse.errors(), vec![]);
        let file: ast::SourceFile = parse.tree();

        if let Some(type_info) = hover(&file, offset) {
            let offset_usize: usize = offset.into();
            let title = format!("hover: {}", type_info);
            let group = Level::INFO.primary_title(&title).element(
                Snippet::source(&sql).fold(true).annotation(
                    AnnotationKind::Context
                        .span(offset_usize..offset_usize + 1)
                        .label("hover"),
                ),
            );
            let renderer = Renderer::plain().decor_style(DecorStyle::Unicode);
            return Some(
                renderer
                    .render(&[group])
                    .to_string()
                    // neater
                    .replace("info: hover:", "hover:"),
            );
        }
        None
    }

    #[test]
    fn hover_column_in_create_index() {
        assert_snapshot!(check_hover("
create table users(id int, email text);
create index idx_email on users(email$0);
"), @r"
        hover: column public.users.email text
          ╭▸ 
        3 │ create index idx_email on users(email);
          ╰╴                                    ─ hover
        ");
    }

    #[test]
    fn hover_column_int_type() {
        assert_snapshot!(check_hover("
create table users(id int, email text);
create index idx_id on users(id$0);
"), @r"
        hover: column public.users.id int
          ╭▸ 
        3 │ create index idx_id on users(id);
          ╰╴                              ─ hover
        ");
    }

    #[test]
    fn hover_column_with_schema() {
        assert_snapshot!(check_hover("
create table public.users(id int, email text);
create index idx_email on public.users(email$0);
"), @r"
        hover: column public.users.email text
          ╭▸ 
        3 │ create index idx_email on public.users(email);
          ╰╴                                           ─ hover
        ");
    }

    #[test]
    fn hover_column_temp_table() {
        assert_snapshot!(check_hover("
create temp table users(id int, email text);
create index idx_email on users(email$0);
"), @r"
        hover: column pg_temp.users.email text
          ╭▸ 
        3 │ create index idx_email on users(email);
          ╰╴                                    ─ hover
        ");
    }

    #[test]
    fn hover_column_multiple_columns() {
        assert_snapshot!(check_hover("
create table users(id int, email text, name varchar(100));
create index idx_users on users(id, email$0, name);
"), @r"
        hover: column public.users.email text
          ╭▸ 
        3 │ create index idx_users on users(id, email, name);
          ╰╴                                        ─ hover
        ");
    }

    #[test]
    fn hover_column_varchar() {
        assert_snapshot!(check_hover("
create table users(id int, name varchar(100));
create index idx_name on users(name$0);
"), @r"
        hover: column public.users.name varchar(100)
          ╭▸ 
        3 │ create index idx_name on users(name);
          ╰╴                                  ─ hover
        ");
    }

    #[test]
    fn hover_column_bigint() {
        assert_snapshot!(check_hover("
create table metrics(value bigint);
create index idx_value on metrics(value$0);
"), @r"
        hover: column public.metrics.value bigint
          ╭▸ 
        3 │ create index idx_value on metrics(value);
          ╰╴                                      ─ hover
        ");
    }

    #[test]
    fn hover_column_timestamp() {
        assert_snapshot!(check_hover("
create table events(created_at timestamp with time zone);
create index idx_created on events(created_at$0);
"), @r"
        hover: column public.events.created_at timestamp with time zone
          ╭▸ 
        3 │ create index idx_created on events(created_at);
          ╰╴                                            ─ hover
        ");
    }

    #[test]
    fn hover_column_with_search_path() {
        assert_snapshot!(check_hover(r#"
set search_path to myschema;
create table myschema.users(id int, email text);
create index idx_email on users(email$0);
"#), @r"
        hover: column myschema.users.email text
          ╭▸ 
        4 │ create index idx_email on users(email);
          ╰╴                                    ─ hover
        ");
    }

    #[test]
    fn hover_column_explicit_schema_overrides_search_path() {
        assert_snapshot!(check_hover(r#"
set search_path to myschema;
create table public.users(id int, email text);
create table myschema.users(value bigint);
create index idx_email on public.users(email$0);
"#), @r"
        hover: column public.users.email text
          ╭▸ 
        5 │ create index idx_email on public.users(email);
          ╰╴                                           ─ hover
        ");
    }

    #[test]
    fn hover_on_table_name() {
        assert_snapshot!(check_hover("
create table t(id int);
create index idx on t$0(id);
"), @r"
        hover: table public.t(id int)
          ╭▸ 
        3 │ create index idx on t(id);
          ╰╴                    ─ hover
        ");
    }

    #[test]
    fn hover_on_index_name_in_create() {
        assert_snapshot!(check_hover("
create table users(id int);
create index idx$0 on users(id);
"), @r"
        hover: index public.idx on public.users(id)
          ╭▸ 
        3 │ create index idx on users(id);
          ╰╴               ─ hover
        ");
    }

    #[test]
    fn hover_table_in_create_index() {
        assert_snapshot!(check_hover("
create table users(id int, email text);
create index idx_email on users$0(email);
"), @r"
        hover: table public.users(id int, email text)
          ╭▸ 
        3 │ create index idx_email on users(email);
          ╰╴                              ─ hover
        ");
    }

    #[test]
    fn hover_table_with_schema() {
        assert_snapshot!(check_hover("
create table public.users(id int, email text);
create index idx on public.users$0(id);
"), @r"
        hover: table public.users(id int, email text)
          ╭▸ 
        3 │ create index idx on public.users(id);
          ╰╴                               ─ hover
        ");
    }

    #[test]
    fn hover_table_temp() {
        assert_snapshot!(check_hover("
create temp table users(id int, email text);
create index idx on users$0(id);
"), @r"
        hover: table pg_temp.users(id int, email text)
          ╭▸ 
        3 │ create index idx on users(id);
          ╰╴                        ─ hover
        ");
    }

    #[test]
    fn hover_table_multiline() {
        assert_snapshot!(check_hover("
create table users(
    id int,
    email text,
    name varchar(100)
);
create index idx on users$0(id);
"), @r"
        hover: table public.users(
                  id int,
                  email text,
                  name varchar(100)
              )
          ╭▸ 
        7 │ create index idx on users(id);
          ╰╴                        ─ hover
        ");
    }

    #[test]
    fn hover_table_with_search_path() {
        assert_snapshot!(check_hover(r#"
set search_path to myschema;
create table users(id int, email text);
create index idx on users$0(id);
"#), @r"
        hover: table myschema.users(id int, email text)
          ╭▸ 
        4 │ create index idx on users(id);
          ╰╴                        ─ hover
        ");
    }

    #[test]
    fn hover_table_search_path_at_definition() {
        assert_snapshot!(check_hover(r#"
set search_path to myschema;
create table users(id int, email text);
set search_path to myschema, otherschema;
create index idx on users$0(id);
"#), @r"
        hover: table myschema.users(id int, email text)
          ╭▸ 
        5 │ create index idx on users(id);
          ╰╴                        ─ hover
        ");
    }

    #[test]
    fn hover_on_create_table_definition() {
        assert_snapshot!(check_hover("
create table t$0(x bigint);
"), @r"
        hover: table public.t(x bigint)
          ╭▸ 
        2 │ create table t(x bigint);
          ╰╴             ─ hover
        ");
    }

    #[test]
    fn hover_on_create_table_definition_with_schema() {
        assert_snapshot!(check_hover("
create table myschema.users$0(id int);
"), @r"
        hover: table myschema.users(id int)
          ╭▸ 
        2 │ create table myschema.users(id int);
          ╰╴                          ─ hover
        ");
    }

    #[test]
    fn hover_on_create_temp_table_definition() {
        assert_snapshot!(check_hover("
create temp table t$0(x bigint);
"), @r"
        hover: table pg_temp.t(x bigint)
          ╭▸ 
        2 │ create temp table t(x bigint);
          ╰╴                  ─ hover
        ");
    }

    #[test]
    fn hover_on_column_in_create_table() {
        assert_snapshot!(check_hover("
create table t(id$0 int);
"), @r"
        hover: column public.t.id int
          ╭▸ 
        2 │ create table t(id int);
          ╰╴                ─ hover
        ");
    }

    #[test]
    fn hover_on_column_in_create_table_with_schema() {
        assert_snapshot!(check_hover("
create table myschema.users(id$0 int, name text);
"), @r"
        hover: column myschema.users.id int
          ╭▸ 
        2 │ create table myschema.users(id int, name text);
          ╰╴                             ─ hover
        ");
    }

    #[test]
    fn hover_on_column_in_temp_table() {
        assert_snapshot!(check_hover("
create temp table t(x$0 bigint);
"), @r"
        hover: column pg_temp.t.x bigint
          ╭▸ 
        2 │ create temp table t(x bigint);
          ╰╴                    ─ hover
        ");
    }

    #[test]
    fn hover_on_multiple_columns() {
        assert_snapshot!(check_hover("
create table t(id int, email$0 text, name varchar(100));
"), @r"
        hover: column public.t.email text
          ╭▸ 
        2 │ create table t(id int, email text, name varchar(100));
          ╰╴                           ─ hover
        ");
    }

    #[test]
    fn hover_on_drop_table() {
        assert_snapshot!(check_hover("
create table users(id int, email text);
drop table users$0;
"), @r"
        hover: table public.users(id int, email text)
          ╭▸ 
        3 │ drop table users;
          ╰╴               ─ hover
        ");
    }

    #[test]
    fn hover_on_drop_table_with_schema() {
        assert_snapshot!(check_hover("
create table myschema.users(id int);
drop table myschema.users$0;
"), @r"
        hover: table myschema.users(id int)
          ╭▸ 
        3 │ drop table myschema.users;
          ╰╴                        ─ hover
        ");
    }

    #[test]
    fn hover_on_drop_temp_table() {
        assert_snapshot!(check_hover("
create temp table t(x bigint);
drop table t$0;
"), @r"
        hover: table pg_temp.t(x bigint)
          ╭▸ 
        3 │ drop table t;
          ╰╴           ─ hover
        ");
    }

    #[test]
    fn hover_on_create_index_definition() {
        assert_snapshot!(check_hover("
create table t(x bigint);
create index idx$0 on t(x);
"), @r"
        hover: index public.idx on public.t(x)
          ╭▸ 
        3 │ create index idx on t(x);
          ╰╴               ─ hover
        ");
    }

    #[test]
    fn hover_on_drop_index() {
        assert_snapshot!(check_hover("
create table t(x bigint);
create index idx_x on t(x);
drop index idx_x$0;
"), @r"
        hover: index public.idx_x on public.t(x)
          ╭▸ 
        4 │ drop index idx_x;
          ╰╴               ─ hover
        ");
    }

    #[test]
    fn hover_on_create_type_definition() {
        assert_snapshot!(check_hover("
create type status$0 as enum ('active', 'inactive');
"), @r"
        hover: type public.status as enum ('active', 'inactive')
          ╭▸ 
        2 │ create type status as enum ('active', 'inactive');
          ╰╴                 ─ hover
        ");
    }

    #[test]
    fn hover_on_create_type_definition_with_schema() {
        assert_snapshot!(check_hover("
create type myschema.status$0 as enum ('active', 'inactive');
"), @r"
        hover: type myschema.status as enum ('active', 'inactive')
          ╭▸ 
        2 │ create type myschema.status as enum ('active', 'inactive');
          ╰╴                          ─ hover
        ");
    }

    #[test]
    fn hover_on_drop_type() {
        assert_snapshot!(check_hover("
create type status as enum ('active', 'inactive');
drop type status$0;
"), @r"
        hover: type public.status as enum ('active', 'inactive')
          ╭▸ 
        3 │ drop type status;
          ╰╴               ─ hover
        ");
    }

    #[test]
    fn hover_on_drop_type_with_schema() {
        assert_snapshot!(check_hover("
create type myschema.status as enum ('active', 'inactive');
drop type myschema.status$0;
"), @r"
        hover: type myschema.status as enum ('active', 'inactive')
          ╭▸ 
        3 │ drop type myschema.status;
          ╰╴                        ─ hover
        ");
    }

    #[test]
    fn hover_on_create_type_composite() {
        assert_snapshot!(check_hover("
create type person$0 as (name text, age int);
"), @r"
        hover: type public.person as (name text, age int)
          ╭▸ 
        2 │ create type person as (name text, age int);
          ╰╴                 ─ hover
        ");
    }

    #[test]
    fn hover_on_drop_type_composite() {
        assert_snapshot!(check_hover("
create type person as (name text, age int);
drop type person$0;
"), @r"
        hover: type public.person as (name text, age int)
          ╭▸ 
        3 │ drop type person;
          ╰╴               ─ hover
        ");
    }

    #[test]
    fn hover_on_create_type_range() {
        assert_snapshot!(check_hover("
create type int4_range$0 as range (subtype = int4);
"), @r"
        hover: type public.int4_range (subtype = int4)
          ╭▸ 
        2 │ create type int4_range as range (subtype = int4);
          ╰╴                     ─ hover
        ");
    }

    #[test]
    fn hover_on_drop_type_range() {
        assert_snapshot!(check_hover("
create type int4_range as range (subtype = int4);
drop type int4_range$0;
"), @r"
        hover: type public.int4_range (subtype = int4)
          ╭▸ 
        3 │ drop type int4_range;
          ╰╴                   ─ hover
        ");
    }

    #[test]
    fn hover_on_cast_operator() {
        assert_snapshot!(check_hover("
create type foo as enum ('a', 'b');
select x::foo$0;
"), @r"
        hover: type public.foo as enum ('a', 'b')
          ╭▸ 
        3 │ select x::foo;
          ╰╴            ─ hover
        ");
    }

    #[test]
    fn hover_on_cast_function() {
        assert_snapshot!(check_hover("
create type bar as enum ('x', 'y');
select cast(x as bar$0);
"), @r"
        hover: type public.bar as enum ('x', 'y')
          ╭▸ 
        3 │ select cast(x as bar);
          ╰╴                   ─ hover
        ");
    }

    #[test]
    fn hover_on_cast_with_schema() {
        assert_snapshot!(check_hover("
create type myschema.baz as enum ('m', 'n');
select x::myschema.baz$0;
"), @r"
        hover: type myschema.baz as enum ('m', 'n')
          ╭▸ 
        3 │ select x::myschema.baz;
          ╰╴                     ─ hover
        ");
    }

    #[test]
    fn hover_on_drop_function() {
        assert_snapshot!(check_hover("
create function foo() returns int as $$ select 1 $$ language sql;
drop function foo$0();
"), @r"
        hover: function public.foo() returns int
          ╭▸ 
        3 │ drop function foo();
          ╰╴                ─ hover
        ");
    }

    #[test]
    fn hover_on_drop_function_with_schema() {
        assert_snapshot!(check_hover("
create function myschema.foo() returns int as $$ select 1 $$ language sql;
drop function myschema.foo$0();
"), @r"
        hover: function myschema.foo() returns int
          ╭▸ 
        3 │ drop function myschema.foo();
          ╰╴                         ─ hover
        ");
    }

    #[test]
    fn hover_on_create_function_definition() {
        assert_snapshot!(check_hover("
create function foo$0() returns int as $$ select 1 $$ language sql;
"), @r"
        hover: function public.foo() returns int
          ╭▸ 
        2 │ create function foo() returns int as $$ select 1 $$ language sql;
          ╰╴                  ─ hover
        ");
    }

    #[test]
    fn hover_on_create_function_with_explicit_schema() {
        assert_snapshot!(check_hover("
create function myschema.foo$0() returns int as $$ select 1 $$ language sql;
"), @r"
        hover: function myschema.foo() returns int
          ╭▸ 
        2 │ create function myschema.foo() returns int as $$ select 1 $$ language sql;
          ╰╴                           ─ hover
        ");
    }

    #[test]
    fn hover_on_drop_function_with_search_path() {
        assert_snapshot!(check_hover(r#"
set search_path to myschema;
create function foo() returns int as $$ select 1 $$ language sql;
drop function foo$0();
"#), @r"
        hover: function myschema.foo() returns int
          ╭▸ 
        4 │ drop function foo();
          ╰╴                ─ hover
        ");
    }

    #[test]
    fn hover_on_drop_function_overloaded() {
        assert_snapshot!(check_hover("
create function add(complex) returns complex as $$ select null $$ language sql;
create function add(bigint) returns bigint as $$ select 1 $$ language sql;
drop function add$0(complex);
"), @r"
        hover: function public.add(complex) returns complex
          ╭▸ 
        4 │ drop function add(complex);
          ╰╴                ─ hover
        ");
    }

    #[test]
    fn hover_on_drop_function_second_overload() {
        assert_snapshot!(check_hover("
create function add(complex) returns complex as $$ select null $$ language sql;
create function add(bigint) returns bigint as $$ select 1 $$ language sql;
drop function add$0(bigint);
"), @r"
        hover: function public.add(bigint) returns bigint
          ╭▸ 
        4 │ drop function add(bigint);
          ╰╴                ─ hover
        ");
    }

    #[test]
    fn hover_on_drop_aggregate() {
        assert_snapshot!(check_hover("
create aggregate myavg(int) (sfunc = int4_avg_accum, stype = _int8);
drop aggregate myavg$0(int);
"), @r"
        hover: aggregate public.myavg(int)
          ╭▸ 
        3 │ drop aggregate myavg(int);
          ╰╴                   ─ hover
        ");
    }

    #[test]
    fn hover_on_drop_aggregate_with_schema() {
        assert_snapshot!(check_hover("
create aggregate myschema.myavg(int) (sfunc = int4_avg_accum, stype = _int8);
drop aggregate myschema.myavg$0(int);
"), @r"
        hover: aggregate myschema.myavg(int)
          ╭▸ 
        3 │ drop aggregate myschema.myavg(int);
          ╰╴                            ─ hover
        ");
    }

    #[test]
    fn hover_on_create_aggregate_definition() {
        assert_snapshot!(check_hover("
create aggregate myavg$0(int) (sfunc = int4_avg_accum, stype = _int8);
"), @r"
        hover: aggregate public.myavg(int)
          ╭▸ 
        2 │ create aggregate myavg(int) (sfunc = int4_avg_accum, stype = _int8);
          ╰╴                     ─ hover
        ");
    }

    #[test]
    fn hover_on_drop_aggregate_with_search_path() {
        assert_snapshot!(check_hover(r#"
set search_path to myschema;
create aggregate myavg(int) (sfunc = int4_avg_accum, stype = _int8);
drop aggregate myavg$0(int);
"#), @r"
        hover: aggregate myschema.myavg(int)
          ╭▸ 
        4 │ drop aggregate myavg(int);
          ╰╴                   ─ hover
        ");
    }

    #[test]
    fn hover_on_drop_aggregate_overloaded() {
        assert_snapshot!(check_hover("
create aggregate sum(complex) (sfunc = complex_add, stype = complex, initcond = '(0,0)');
create aggregate sum(bigint) (sfunc = bigint_add, stype = bigint, initcond = '0');
drop aggregate sum$0(complex);
"), @r"
        hover: aggregate public.sum(complex)
          ╭▸ 
        4 │ drop aggregate sum(complex);
          ╰╴                 ─ hover
        ");
    }

    #[test]
    fn hover_on_drop_aggregate_second_overload() {
        assert_snapshot!(check_hover("
create aggregate sum(complex) (sfunc = complex_add, stype = complex, initcond = '(0,0)');
create aggregate sum(bigint) (sfunc = bigint_add, stype = bigint, initcond = '0');
drop aggregate sum$0(bigint);
"), @r"
        hover: aggregate public.sum(bigint)
          ╭▸ 
        4 │ drop aggregate sum(bigint);
          ╰╴                 ─ hover
        ");
    }

    #[test]
    fn hover_on_select_function_call() {
        assert_snapshot!(check_hover("
create function foo() returns int as $$ select 1 $$ language sql;
select foo$0();
"), @r"
        hover: function public.foo() returns int
          ╭▸ 
        3 │ select foo();
          ╰╴         ─ hover
        ");
    }

    #[test]
    fn hover_on_select_function_call_with_schema() {
        assert_snapshot!(check_hover("
create function public.foo() returns int as $$ select 1 $$ language sql;
select public.foo$0();
"), @r"
        hover: function public.foo() returns int
          ╭▸ 
        3 │ select public.foo();
          ╰╴                ─ hover
        ");
    }

    #[test]
    fn hover_on_select_function_call_with_search_path() {
        assert_snapshot!(check_hover(r#"
set search_path to myschema;
create function foo() returns int as $$ select 1 $$ language sql;
select foo$0();
"#), @r"
        hover: function myschema.foo() returns int
          ╭▸ 
        4 │ select foo();
          ╰╴         ─ hover
        ");
    }

    #[test]
    fn hover_on_select_function_call_with_params() {
        assert_snapshot!(check_hover("
create function add(a int, b int) returns int as $$ select a + b $$ language sql;
select add$0(1, 2);
"), @r"
        hover: function public.add(a int, b int) returns int
          ╭▸ 
        3 │ select add(1, 2);
          ╰╴         ─ hover
        ");
    }

    #[test]
    fn hover_on_function_call_style_column_access() {
        assert_snapshot!(check_hover("
create table t(a int, b int);
select a$0(t) from t;
"), @r"
        hover: column public.t.a int
          ╭▸ 
        3 │ select a(t) from t;
          ╰╴       ─ hover
        ");
    }

    #[test]
    fn hover_on_function_call_style_column_access_with_function_precedence() {
        assert_snapshot!(check_hover("
create table t(a int, b int);
create function b(t) returns int as 'select 1' LANGUAGE sql;
select b$0(t) from t;
"), @r"
        hover: function public.b(t) returns int
          ╭▸ 
        4 │ select b(t) from t;
          ╰╴       ─ hover
        ");
    }

    #[test]
    fn hover_on_function_call_style_table_arg() {
        assert_snapshot!(check_hover("
create table t(a int, b int);
select a(t$0) from t;
"), @r"
        hover: table public.t(a int, b int)
          ╭▸ 
        3 │ select a(t) from t;
          ╰╴         ─ hover
        ");
    }

    #[test]
    fn hover_on_function_call_style_table_arg_with_function() {
        assert_snapshot!(check_hover("
create table t(a int, b int);
create function b(t) returns int as 'select 1' LANGUAGE sql;
select b(t$0) from t;
"), @r"
        hover: table public.t(a int, b int)
          ╭▸ 
        4 │ select b(t) from t;
          ╰╴         ─ hover
        ");
    }

    #[test]
    fn hover_on_function_call_style_table_arg_in_where() {
        assert_snapshot!(check_hover("
create table t(a int);
select * from t where a(t$0) > 2;
"), @r"
        hover: table public.t(a int)
          ╭▸ 
        3 │ select * from t where a(t) > 2;
          ╰╴                        ─ hover
        ");
    }

    #[test]
    fn hover_on_qualified_table_ref_in_where() {
        assert_snapshot!(check_hover("
create table t(a int);
create function b(t) returns int as 'select 1' language sql;
select * from t where t$0.b > 2;
"), @r"
        hover: table public.t(a int)
          ╭▸ 
        4 │ select * from t where t.b > 2;
          ╰╴                      ─ hover
        ");
    }

    #[test]
    fn hover_on_field_style_function_call() {
        assert_snapshot!(check_hover("
create table t(a int);
create function b(t) returns int as 'select 1' language sql;
select t.b$0 from t;
"), @r"
        hover: function public.b(t) returns int
          ╭▸ 
        4 │ select t.b from t;
          ╰╴         ─ hover
        ");
    }

    #[test]
    fn hover_on_field_style_function_call_column_precedence() {
        assert_snapshot!(check_hover("
create table t(a int, b int);
create function b(t) returns int as 'select 1' language sql;
select t.b$0 from t;
"), @r"
        hover: column public.t.b int
          ╭▸ 
        4 │ select t.b from t;
          ╰╴         ─ hover
        ");
    }

    #[test]
    fn hover_on_field_style_function_call_table_ref() {
        assert_snapshot!(check_hover("
create table t(a int);
create function b(t) returns int as 'select 1' language sql;
select t$0.b from t;
"), @r"
        hover: table public.t(a int)
          ╭▸ 
        4 │ select t.b from t;
          ╰╴       ─ hover
        ");
    }

    #[test]
    fn hover_on_select_from_table() {
        assert_snapshot!(check_hover("
create table users(id int, email text);
select * from users$0;
"), @r"
        hover: table public.users(id int, email text)
          ╭▸ 
        3 │ select * from users;
          ╰╴                  ─ hover
        ");
    }

    #[test]
    fn hover_on_select_from_table_with_schema() {
        assert_snapshot!(check_hover("
create table public.users(id int, email text);
select * from public.users$0;
"), @r"
        hover: table public.users(id int, email text)
          ╭▸ 
        3 │ select * from public.users;
          ╰╴                         ─ hover
        ");
    }

    #[test]
    fn hover_on_select_from_table_with_search_path() {
        assert_snapshot!(check_hover("
set search_path to foo;
create table foo.users(id int, email text);
select * from users$0;
"), @r"
        hover: table foo.users(id int, email text)
          ╭▸ 
        4 │ select * from users;
          ╰╴                  ─ hover
        ");
    }

    #[test]
    fn hover_on_select_from_temp_table() {
        assert_snapshot!(check_hover("
create temp table users(id int, email text);
select * from users$0;
"), @r"
        hover: table pg_temp.users(id int, email text)
          ╭▸ 
        3 │ select * from users;
          ╰╴                  ─ hover
        ");
    }

    #[test]
    fn hover_on_select_from_multiline_table() {
        assert_snapshot!(check_hover("
create table users(
    id int,
    email text,
    name varchar(100)
);
select * from users$0;
"), @r"
        hover: table public.users(
                  id int,
                  email text,
                  name varchar(100)
              )
          ╭▸ 
        7 │ select * from users;
          ╰╴                  ─ hover
        ");
    }

    #[test]
    fn hover_on_select_column() {
        assert_snapshot!(check_hover("
create table users(id int, email text);
select id$0 from users;
"), @r"
        hover: column public.users.id int
          ╭▸ 
        3 │ select id from users;
          ╰╴        ─ hover
        ");
    }

    #[test]
    fn hover_on_select_column_second() {
        assert_snapshot!(check_hover("
create table users(id int, email text);
select id, email$0 from users;
"), @r"
        hover: column public.users.email text
          ╭▸ 
        3 │ select id, email from users;
          ╰╴               ─ hover
        ");
    }

    #[test]
    fn hover_on_select_column_with_schema() {
        assert_snapshot!(check_hover("
create table public.users(id int, email text);
select email$0 from public.users;
"), @r"
        hover: column public.users.email text
          ╭▸ 
        3 │ select email from public.users;
          ╰╴           ─ hover
        ");
    }

    #[test]
    fn hover_on_select_column_with_search_path() {
        assert_snapshot!(check_hover("
set search_path to foo;
create table foo.users(id int, email text);
select id$0 from users;
"), @r"
        hover: column foo.users.id int
          ╭▸ 
        4 │ select id from users;
          ╰╴        ─ hover
        ");
    }

    #[test]
    fn hover_on_insert_table() {
        assert_snapshot!(check_hover("
create table users(id int, email text);
insert into users$0(id, email) values (1, 'test');
"), @r"
        hover: table public.users(id int, email text)
          ╭▸ 
        3 │ insert into users(id, email) values (1, 'test');
          ╰╴                ─ hover
        ");
    }

    #[test]
    fn hover_on_insert_table_with_schema() {
        assert_snapshot!(check_hover("
create table public.users(id int, email text);
insert into public.users$0(id, email) values (1, 'test');
"), @r"
        hover: table public.users(id int, email text)
          ╭▸ 
        3 │ insert into public.users(id, email) values (1, 'test');
          ╰╴                       ─ hover
        ");
    }

    #[test]
    fn hover_on_insert_column() {
        assert_snapshot!(check_hover("
create table users(id int, email text);
insert into users(id$0, email) values (1, 'test');
"), @r"
        hover: column public.users.id int
          ╭▸ 
        3 │ insert into users(id, email) values (1, 'test');
          ╰╴                   ─ hover
        ");
    }

    #[test]
    fn hover_on_insert_column_second() {
        assert_snapshot!(check_hover("
create table users(id int, email text);
insert into users(id, email$0) values (1, 'test');
"), @r"
        hover: column public.users.email text
          ╭▸ 
        3 │ insert into users(id, email) values (1, 'test');
          ╰╴                          ─ hover
        ");
    }

    #[test]
    fn hover_on_insert_column_with_schema() {
        assert_snapshot!(check_hover("
create table public.users(id int, email text);
insert into public.users(email$0) values ('test');
"), @r"
        hover: column public.users.email text
          ╭▸ 
        3 │ insert into public.users(email) values ('test');
          ╰╴                             ─ hover
        ");
    }

    #[test]
    fn hover_on_delete_table() {
        assert_snapshot!(check_hover("
create table users(id int, email text);
delete from users$0 where id = 1;
"), @r"
        hover: table public.users(id int, email text)
          ╭▸ 
        3 │ delete from users where id = 1;
          ╰╴                ─ hover
        ");
    }

    #[test]
    fn hover_on_delete_table_with_schema() {
        assert_snapshot!(check_hover("
create table public.users(id int, email text);
delete from public.users$0 where id = 1;
"), @r"
        hover: table public.users(id int, email text)
          ╭▸ 
        3 │ delete from public.users where id = 1;
          ╰╴                       ─ hover
        ");
    }

    #[test]
    fn hover_on_delete_where_column() {
        assert_snapshot!(check_hover("
create table users(id int, email text);
delete from users where id$0 = 1;
"), @r"
        hover: column public.users.id int
          ╭▸ 
        3 │ delete from users where id = 1;
          ╰╴                         ─ hover
        ");
    }

    #[test]
    fn hover_on_delete_where_column_second() {
        assert_snapshot!(check_hover("
create table users(id int, email text, active boolean);
delete from users where id = 1 and email$0 = 'test';
"), @r"
        hover: column public.users.email text
          ╭▸ 
        3 │ delete from users where id = 1 and email = 'test';
          ╰╴                                       ─ hover
        ");
    }

    #[test]
    fn hover_on_delete_where_column_with_schema() {
        assert_snapshot!(check_hover("
create table public.users(id int, email text);
delete from public.users where email$0 = 'test';
"), @r"
        hover: column public.users.email text
          ╭▸ 
        3 │ delete from public.users where email = 'test';
          ╰╴                                   ─ hover
        ");
    }

    #[test]
    fn hover_on_select_table_as_column() {
        assert_snapshot!(check_hover("
create table t(x bigint, y bigint);
select t$0 from t;
"), @r"
        hover: table public.t(x bigint, y bigint)
          ╭▸ 
        3 │ select t from t;
          ╰╴       ─ hover
        ");
    }

    #[test]
    fn hover_on_select_table_as_column_with_schema() {
        assert_snapshot!(check_hover("
create table public.t(x bigint, y bigint);
select t$0 from public.t;
"), @r"
        hover: table public.t(x bigint, y bigint)
          ╭▸ 
        3 │ select t from public.t;
          ╰╴       ─ hover
        ");
    }

    #[test]
    fn hover_on_select_table_as_column_with_search_path() {
        assert_snapshot!(check_hover("
set search_path to foo;
create table foo.users(id int, email text);
select users$0 from users;
"), @r"
        hover: table foo.users(id int, email text)
          ╭▸ 
        4 │ select users from users;
          ╰╴           ─ hover
        ");
    }

    #[test]
    fn hover_on_select_column_with_same_name_as_table() {
        assert_snapshot!(check_hover("
create table t(t int);
select t$0 from t;
"), @r"
        hover: column public.t.t int
          ╭▸ 
        3 │ select t from t;
          ╰╴       ─ hover
        ");
    }

    #[test]
    fn hover_on_create_schema() {
        assert_snapshot!(check_hover("
create schema foo$0;
"), @r"
        hover: schema foo
          ╭▸ 
        2 │ create schema foo;
          ╰╴                ─ hover
        ");
    }

    #[test]
    fn hover_on_drop_schema() {
        assert_snapshot!(check_hover("
create schema foo;
drop schema foo$0;
"), @r"
        hover: schema foo
          ╭▸ 
        3 │ drop schema foo;
          ╰╴              ─ hover
        ");
    }

    #[test]
    fn hover_on_schema_after_definition() {
        assert_snapshot!(check_hover("
drop schema foo$0;
create schema foo;
"), @r"
        hover: schema foo
          ╭▸ 
        2 │ drop schema foo;
          ╰╴              ─ hover
        ");
    }

    #[test]
    fn hover_on_cte_table() {
        assert_snapshot!(check_hover("
with t as (select 1 a)
select a from t$0;
"), @r"
        hover: with t as (select 1 a)
          ╭▸ 
        3 │ select a from t;
          ╰╴              ─ hover
        ");
    }

    #[test]
    fn hover_on_cte_column() {
        assert_snapshot!(check_hover("
with t as (select 1 a)
select a$0 from t;
"), @r"
        hover: column t.a
          ╭▸ 
        3 │ select a from t;
          ╰╴       ─ hover
        ");
    }

    #[test]
    fn hover_on_cte_with_multiple_columns() {
        assert_snapshot!(check_hover("
with t as (select 1 a, 2 b)
select b$0 from t;
"), @r"
        hover: column t.b
          ╭▸ 
        3 │ select b from t;
          ╰╴       ─ hover
        ");
    }

    #[test]
    fn hover_on_cte_with_column_list() {
        assert_snapshot!(check_hover("
with t(a) as (select 1)
select a$0 from t;
"), @r"
        hover: column t.a
          ╭▸ 
        3 │ select a from t;
          ╰╴       ─ hover
        ");
    }

    #[test]
    fn hover_on_nested_cte() {
        assert_snapshot!(check_hover("
with x as (select 1 a),
     y as (select a from x)
select a$0 from y;
"), @r"
        hover: column y.a
          ╭▸ 
        4 │ select a from y;
          ╰╴       ─ hover
        ");
    }

    #[test]
    fn hover_on_cte_shadowing_table_with_star() {
        assert_snapshot!(check_hover("
create table t(a bigint);
with t as (select * from t)
select a$0 from t;
"), @r"
        hover: column public.t.a bigint
          ╭▸ 
        4 │ select a from t;
          ╰╴       ─ hover
        ");
    }

    #[test]
    fn hover_on_cte_definition() {
        assert_snapshot!(check_hover("
with t$0 as (select 1 a)
select a from t;
"), @r"
        hover: with t as (select 1 a)
          ╭▸ 
        2 │ with t as (select 1 a)
          ╰╴     ─ hover
        ");
    }

    #[test]
    fn hover_on_cte_values_column1() {
        assert_snapshot!(check_hover("
with t as (
    values (1, 2), (3, 4)
)
select column1$0, column2 from t;
"), @r"
        hover: column t.column1
          ╭▸ 
        5 │ select column1, column2 from t;
          ╰╴             ─ hover
        ");
    }

    #[test]
    fn hover_on_cte_values_column2() {
        assert_snapshot!(check_hover("
with t as (
    values (1, 2), (3, 4)
)
select column1, column2$0 from t;
"), @r"
        hover: column t.column2
          ╭▸ 
        5 │ select column1, column2 from t;
          ╰╴                      ─ hover
        ");
    }

    #[test]
    fn hover_on_cte_values_single_column() {
        assert_snapshot!(check_hover("
with t as (
    values (1), (2), (3)
)
select column1$0 from t;
"), @r"
        hover: column t.column1
          ╭▸ 
        5 │ select column1 from t;
          ╰╴             ─ hover
        ");
    }

    #[test]
    fn hover_on_cte_values_uppercase_column_names() {
        assert_snapshot!(check_hover("
with t as (
    values (1, 2), (3, 4)
)
select COLUMN1$0, COLUMN2 from t;
"), @r"
        hover: column t.column1
          ╭▸ 
        5 │ select COLUMN1, COLUMN2 from t;
          ╰╴             ─ hover
        ");
    }

    #[test]
    fn hover_on_drop_procedure() {
        assert_snapshot!(check_hover("
create procedure foo() language sql as $$ select 1 $$;
drop procedure foo$0();
"), @r"
        hover: procedure public.foo()
          ╭▸ 
        3 │ drop procedure foo();
          ╰╴                 ─ hover
        ");
    }

    #[test]
    fn hover_on_drop_procedure_with_schema() {
        assert_snapshot!(check_hover("
create procedure myschema.foo() language sql as $$ select 1 $$;
drop procedure myschema.foo$0();
"), @r"
        hover: procedure myschema.foo()
          ╭▸ 
        3 │ drop procedure myschema.foo();
          ╰╴                          ─ hover
        ");
    }

    #[test]
    fn hover_on_create_procedure_definition() {
        assert_snapshot!(check_hover("
create procedure foo$0() language sql as $$ select 1 $$;
"), @r"
        hover: procedure public.foo()
          ╭▸ 
        2 │ create procedure foo() language sql as $$ select 1 $$;
          ╰╴                   ─ hover
        ");
    }

    #[test]
    fn hover_on_create_procedure_with_explicit_schema() {
        assert_snapshot!(check_hover("
create procedure myschema.foo$0() language sql as $$ select 1 $$;
"), @r"
        hover: procedure myschema.foo()
          ╭▸ 
        2 │ create procedure myschema.foo() language sql as $$ select 1 $$;
          ╰╴                            ─ hover
        ");
    }

    #[test]
    fn hover_on_drop_procedure_with_search_path() {
        assert_snapshot!(check_hover(r#"
set search_path to myschema;
create procedure foo() language sql as $$ select 1 $$;
drop procedure foo$0();
"#), @r"
        hover: procedure myschema.foo()
          ╭▸ 
        4 │ drop procedure foo();
          ╰╴                 ─ hover
        ");
    }

    #[test]
    fn hover_on_drop_procedure_overloaded() {
        assert_snapshot!(check_hover("
create procedure add(complex) language sql as $$ select null $$;
create procedure add(bigint) language sql as $$ select 1 $$;
drop procedure add$0(complex);
"), @r"
        hover: procedure public.add(complex)
          ╭▸ 
        4 │ drop procedure add(complex);
          ╰╴                 ─ hover
        ");
    }

    #[test]
    fn hover_on_drop_procedure_second_overload() {
        assert_snapshot!(check_hover("
create procedure add(complex) language sql as $$ select null $$;
create procedure add(bigint) language sql as $$ select 1 $$;
drop procedure add$0(bigint);
"), @r"
        hover: procedure public.add(bigint)
          ╭▸ 
        4 │ drop procedure add(bigint);
          ╰╴                 ─ hover
        ");
    }

    #[test]
    fn hover_on_call_procedure() {
        assert_snapshot!(check_hover("
create procedure foo() language sql as $$ select 1 $$;
call foo$0();
"), @r"
        hover: procedure public.foo()
          ╭▸ 
        3 │ call foo();
          ╰╴       ─ hover
        ");
    }

    #[test]
    fn hover_on_call_procedure_with_schema() {
        assert_snapshot!(check_hover("
create procedure public.foo() language sql as $$ select 1 $$;
call public.foo$0();
"), @r"
        hover: procedure public.foo()
          ╭▸ 
        3 │ call public.foo();
          ╰╴              ─ hover
        ");
    }

    #[test]
    fn hover_on_call_procedure_with_search_path() {
        assert_snapshot!(check_hover(r#"
set search_path to myschema;
create procedure foo() language sql as $$ select 1 $$;
call foo$0();
"#), @r"
        hover: procedure myschema.foo()
          ╭▸ 
        4 │ call foo();
          ╰╴       ─ hover
        ");
    }

    #[test]
    fn hover_on_call_procedure_with_params() {
        assert_snapshot!(check_hover("
create procedure add(a int, b int) language sql as $$ select a + b $$;
call add$0(1, 2);
"), @r"
        hover: procedure public.add(a int, b int)
          ╭▸ 
        3 │ call add(1, 2);
          ╰╴       ─ hover
        ");
    }

    #[test]
    fn hover_on_drop_routine_function() {
        assert_snapshot!(check_hover("
create function foo() returns int as $$ select 1 $$ language sql;
drop routine foo$0();
"), @r"
        hover: function public.foo() returns int
          ╭▸ 
        3 │ drop routine foo();
          ╰╴               ─ hover
        ");
    }

    #[test]
    fn hover_on_drop_routine_aggregate() {
        assert_snapshot!(check_hover("
create aggregate myavg(int) (sfunc = int4_avg_accum, stype = _int8);
drop routine myavg$0(int);
"), @r"
        hover: aggregate public.myavg(int)
          ╭▸ 
        3 │ drop routine myavg(int);
          ╰╴                 ─ hover
        ");
    }

    #[test]
    fn hover_on_drop_routine_procedure() {
        assert_snapshot!(check_hover("
create procedure foo() language sql as $$ select 1 $$;
drop routine foo$0();
"), @r"
        hover: procedure public.foo()
          ╭▸ 
        3 │ drop routine foo();
          ╰╴               ─ hover
        ");
    }

    #[test]
    fn hover_on_drop_routine_with_schema() {
        assert_snapshot!(check_hover("
set search_path to public;
create function foo() returns int as $$ select 1 $$ language sql;
drop routine public.foo$0();
"), @r"
        hover: function public.foo() returns int
          ╭▸ 
        4 │ drop routine public.foo();
          ╰╴                      ─ hover
        ");
    }

    #[test]
    fn hover_on_drop_routine_with_search_path() {
        assert_snapshot!(check_hover(r#"
set search_path to myschema;
create function foo() returns int as $$ select 1 $$ language sql;
drop routine foo$0();
"#), @r"
        hover: function myschema.foo() returns int
          ╭▸ 
        4 │ drop routine foo();
          ╰╴               ─ hover
        ");
    }

    #[test]
    fn hover_on_drop_routine_overloaded() {
        assert_snapshot!(check_hover("
create function add(complex) returns complex as $$ select null $$ language sql;
create function add(bigint) returns bigint as $$ select 1 $$ language sql;
drop routine add$0(complex);
"), @r"
        hover: function public.add(complex) returns complex
          ╭▸ 
        4 │ drop routine add(complex);
          ╰╴               ─ hover
        ");
    }

    #[test]
    fn hover_on_drop_routine_prefers_function_over_procedure() {
        assert_snapshot!(check_hover("
create function foo() returns int as $$ select 1 $$ language sql;
create procedure foo() language sql as $$ select 1 $$;
drop routine foo$0();
"), @r"
        hover: function public.foo() returns int
          ╭▸ 
        4 │ drop routine foo();
          ╰╴               ─ hover
        ");
    }

    #[test]
    fn hover_on_drop_routine_prefers_aggregate_over_procedure() {
        assert_snapshot!(check_hover("
create aggregate foo(int) (sfunc = int4_avg_accum, stype = _int8);
create procedure foo(int) language sql as $$ select 1 $$;
drop routine foo$0(int);
"), @r"
        hover: aggregate public.foo(int)
          ╭▸ 
        4 │ drop routine foo(int);
          ╰╴               ─ hover
        ");
    }

    #[test]
    fn hover_on_update_table() {
        assert_snapshot!(check_hover("
create table users(id int, email text);
update users$0 set email = 'new@example.com';
"), @r"
        hover: table public.users(id int, email text)
          ╭▸ 
        3 │ update users set email = 'new@example.com';
          ╰╴           ─ hover
        ");
    }

    #[test]
    fn hover_on_update_table_with_schema() {
        assert_snapshot!(check_hover("
create table public.users(id int, email text);
update public.users$0 set email = 'new@example.com';
"), @r"
        hover: table public.users(id int, email text)
          ╭▸ 
        3 │ update public.users set email = 'new@example.com';
          ╰╴                  ─ hover
        ");
    }

    #[test]
    fn hover_on_update_set_column() {
        assert_snapshot!(check_hover("
create table users(id int, email text);
update users set email$0 = 'new@example.com' where id = 1;
"), @r"
        hover: column public.users.email text
          ╭▸ 
        3 │ update users set email = 'new@example.com' where id = 1;
          ╰╴                     ─ hover
        ");
    }

    #[test]
    fn hover_on_update_set_column_with_schema() {
        assert_snapshot!(check_hover("
create table public.users(id int, email text);
update public.users set email$0 = 'new@example.com' where id = 1;
"), @r"
        hover: column public.users.email text
          ╭▸ 
        3 │ update public.users set email = 'new@example.com' where id = 1;
          ╰╴                            ─ hover
        ");
    }

    #[test]
    fn hover_on_update_where_column() {
        assert_snapshot!(check_hover("
create table users(id int, email text);
update users set email = 'new@example.com' where id$0 = 1;
"), @r"
        hover: column public.users.id int
          ╭▸ 
        3 │ update users set email = 'new@example.com' where id = 1;
          ╰╴                                                  ─ hover
        ");
    }

    #[test]
    fn hover_on_update_where_column_with_schema() {
        assert_snapshot!(check_hover("
create table public.users(id int, email text);
update public.users set email = 'new@example.com' where id$0 = 1;
"), @r"
        hover: column public.users.id int
          ╭▸ 
        3 │ update public.users set email = 'new@example.com' where id = 1;
          ╰╴                                                         ─ hover
        ");
    }

    #[test]
    fn hover_on_update_from_table() {
        assert_snapshot!(check_hover("
create table users(id int, email text);
create table messages(id int, user_id int, email text);
update users set email = messages.email from messages$0 where users.id = messages.user_id;
"), @r"
        hover: table public.messages(id int, user_id int, email text)
          ╭▸ 
        4 │ update users set email = messages.email from messages where users.id = messages.user_id;
          ╰╴                                                    ─ hover
        ");
    }

    #[test]
    fn hover_on_update_from_table_with_schema() {
        assert_snapshot!(check_hover("
create table users(id int, email text);
create table public.messages(id int, user_id int, email text);
update users set email = messages.email from public.messages$0 where users.id = messages.user_id;
"), @r"
        hover: table public.messages(id int, user_id int, email text)
          ╭▸ 
        4 │ update users set email = messages.email from public.messages where users.id = messages.user_id;
          ╰╴                                                           ─ hover
        ");
    }

    #[test]
    fn hover_on_update_with_cte_table() {
        assert_snapshot!(check_hover("
create table users(id int, email text);
with new_data as (
    select 1 as id, 'new@example.com' as email
)
update users set email = new_data.email from new_data$0 where users.id = new_data.id;
"), @r"
        hover: with new_data as (select 1 as id, 'new@example.com' as email)
          ╭▸ 
        6 │ update users set email = new_data.email from new_data where users.id = new_data.id;
          ╰╴                                                    ─ hover
        ");
    }

    #[test]
    fn hover_on_update_with_cte_column_in_set() {
        assert_snapshot!(check_hover("
create table users(id int, email text);
with new_data as (
    select 1 as id, 'new@example.com' as email
)
update users set email = new_data.email$0 from new_data where users.id = new_data.id;
"), @r"
        hover: column new_data.email
          ╭▸ 
        6 │ update users set email = new_data.email from new_data where users.id = new_data.id;
          ╰╴                                      ─ hover
        ");
    }

    #[test]
    fn hover_on_update_with_cte_column_in_where() {
        assert_snapshot!(check_hover("
create table users(id int, email text);
with new_data as (
    select 1 as id, 'new@example.com' as email
)
update users set email = new_data.email from new_data where new_data.id$0 = users.id;
"), @r"
        hover: column new_data.id
          ╭▸ 
        6 │ update users set email = new_data.email from new_data where new_data.id = users.id;
          ╰╴                                                                      ─ hover
        ");
    }

    #[test]
    fn hover_on_create_view_definition() {
        assert_snapshot!(check_hover("
create view v$0 as select 1;
"), @r"
        hover: view public.v as select 1
          ╭▸ 
        2 │ create view v as select 1;
          ╰╴            ─ hover
        ");
    }

    #[test]
    fn hover_on_create_view_definition_with_schema() {
        assert_snapshot!(check_hover("
create view myschema.v$0 as select 1;
"), @r"
        hover: view myschema.v as select 1
          ╭▸ 
        2 │ create view myschema.v as select 1;
          ╰╴                     ─ hover
        ");
    }

    #[test]
    fn hover_on_create_temp_view_definition() {
        assert_snapshot!(check_hover("
create temp view v$0 as select 1;
"), @r"
        hover: view pg_temp.v as select 1
          ╭▸ 
        2 │ create temp view v as select 1;
          ╰╴                 ─ hover
        ");
    }

    #[test]
    fn hover_on_create_view_with_column_list() {
        assert_snapshot!(check_hover("
create view v(col1$0) as select 1;
"), @r"
        hover: column public.v.col1
          ╭▸ 
        2 │ create view v(col1) as select 1;
          ╰╴                 ─ hover
        ");
    }

    #[test]
    fn hover_on_select_from_view() {
        assert_snapshot!(check_hover("
create view v as select 1;
select * from v$0;
"), @r"
        hover: view public.v as select 1
          ╭▸ 
        3 │ select * from v;
          ╰╴              ─ hover
        ");
    }

    #[test]
    fn hover_on_select_column_from_view_column_list() {
        assert_snapshot!(check_hover("
create view v(a) as select 1;
select a$0 from v;
"), @r"
        hover: column public.v.a
          ╭▸ 
        3 │ select a from v;
          ╰╴       ─ hover
        ");
    }

    #[test]
    fn hover_on_select_column_from_view_column_list_overrides_target() {
        assert_snapshot!(check_hover("
create view v(a) as select 1, 2 b;
select a, b$0 from v;
"), @r"
        hover: column public.v.b
          ╭▸ 
        3 │ select a, b from v;
          ╰╴          ─ hover
        ");
    }

    #[test]
    fn hover_on_select_column_from_view_target_list() {
        assert_snapshot!(check_hover("
create view v as select 1 a, 2 b;
select a$0, b from v;
"), @r"
        hover: column public.v.a
          ╭▸ 
        3 │ select a, b from v;
          ╰╴       ─ hover
        ");
    }

    #[test]
    fn hover_on_select_from_view_with_schema() {
        assert_snapshot!(check_hover("
create view myschema.v as select 1;
select * from myschema.v$0;
"), @r"
        hover: view myschema.v as select 1
          ╭▸ 
        3 │ select * from myschema.v;
          ╰╴                       ─ hover
        ");
    }

    #[test]
    fn hover_on_drop_view() {
        assert_snapshot!(check_hover("
create view v as select 1;
drop view v$0;
"), @r"
        hover: view public.v as select 1
          ╭▸ 
        3 │ drop view v;
          ╰╴          ─ hover
        ");
    }
}
