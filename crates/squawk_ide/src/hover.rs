use crate::classify::{NameClass, NameRefClass, classify_name, classify_name_ref};
use crate::column_name::ColumnName;
use crate::offsets::token_from_offset;
use crate::resolve;
use crate::{
    binder,
    symbols::{Name, Schema},
};
use rowan::TextSize;
use squawk_syntax::SyntaxNode;
use squawk_syntax::{
    SyntaxKind,
    ast::{self, AstNode},
};

pub fn hover(file: &ast::SourceFile, offset: TextSize) -> Option<String> {
    let token = token_from_offset(file, offset)?;
    let parent = token.parent()?;

    let root = file.syntax();
    let binder = binder::bind(file);

    if token.kind() == SyntaxKind::STAR {
        if let Some(field_expr) = ast::FieldExpr::cast(parent.clone())
            && field_expr.star_token().is_some()
            && let Some(result) = hover_qualified_star(root, &field_expr, &binder)
        {
            return Some(result);
        }

        if let Some(arg_list) = ast::ArgList::cast(parent.clone())
            && let Some(result) = hover_unqualified_star_in_arg_list(root, &arg_list, &binder)
        {
            return Some(result);
        }

        if let Some(target) = ast::Target::cast(parent.clone())
            && target.star_token().is_some()
            && let Some(result) = hover_unqualified_star(root, &target, &binder)
        {
            return Some(result);
        }
    }

    if let Some(name_ref) = ast::NameRef::cast(parent.clone()) {
        let context = classify_name_ref(&name_ref)?;
        match context {
            NameRefClass::CreateIndexColumn
            | NameRefClass::InsertColumn
            | NameRefClass::DeleteColumn
            | NameRefClass::UpdateColumn
            | NameRefClass::MergeColumn
            | NameRefClass::ConstraintColumn
            | NameRefClass::JoinUsingColumn
            | NameRefClass::ForeignKeyColumn
            | NameRefClass::AlterColumn
            | NameRefClass::QualifiedColumn => {
                return hover_column(root, &name_ref, &binder);
            }
            NameRefClass::Type => {
                return hover_type(root, &name_ref, &binder);
            }
            NameRefClass::CompositeTypeField => {
                return hover_composite_type_field(root, &name_ref, &binder);
            }
            NameRefClass::SelectColumn
            | NameRefClass::SelectQualifiedColumn
            | NameRefClass::PolicyColumn => {
                // Try hover as column first
                if let Some(result) = hover_column(root, &name_ref, &binder) {
                    return Some(result);
                }
                // If no column, try as function (handles field-style function calls like `t.b`)
                if let Some(result) = hover_function(root, &name_ref, &binder) {
                    return Some(result);
                }
                // Finally try as table (handles case like `select t from t;` where t is the table)
                return hover_table(root, &name_ref, &binder);
            }
            NameRefClass::DeleteQualifiedColumnTable
            | NameRefClass::ForeignKeyTable
            | NameRefClass::FromTable
            | NameRefClass::InsertQualifiedColumnTable
            | NameRefClass::LikeTable
            | NameRefClass::MergeQualifiedColumnTable
            | NameRefClass::PolicyQualifiedColumnTable
            | NameRefClass::SelectQualifiedColumnTable
            | NameRefClass::Table
            | NameRefClass::UpdateQualifiedColumnTable
            | NameRefClass::View => {
                return hover_table(root, &name_ref, &binder);
            }
            NameRefClass::Sequence => return hover_sequence(root, &name_ref, &binder),
            NameRefClass::Trigger => return hover_trigger(root, &name_ref, &binder),
            NameRefClass::Policy => {
                return hover_policy(root, &name_ref, &binder);
            }
            NameRefClass::EventTrigger => {
                return hover_event_trigger(root, &name_ref, &binder);
            }
            NameRefClass::Database => {
                return hover_database(root, &name_ref, &binder);
            }
            NameRefClass::Server => {
                return hover_server(root, &name_ref, &binder);
            }
            NameRefClass::Extension => {
                return hover_extension(root, &name_ref, &binder);
            }
            NameRefClass::Role => {
                return hover_role(root, &name_ref, &binder);
            }
            NameRefClass::Tablespace => return hover_tablespace(root, &name_ref, &binder),
            NameRefClass::Index => {
                return hover_index(root, &name_ref, &binder);
            }
            NameRefClass::Function | NameRefClass::FunctionCall | NameRefClass::FunctionName => {
                return hover_function(root, &name_ref, &binder);
            }
            NameRefClass::Aggregate => return hover_aggregate(root, &name_ref, &binder),
            NameRefClass::Procedure | NameRefClass::CallProcedure | NameRefClass::ProcedureCall => {
                return hover_procedure(root, &name_ref, &binder);
            }
            NameRefClass::Routine => return hover_routine(root, &name_ref, &binder),
            NameRefClass::SelectFunctionCall => {
                // Try function first, but fall back to column if no function found
                // (handles function-call-style column access like `select a(t)`)
                if let Some(result) = hover_function(root, &name_ref, &binder) {
                    return Some(result);
                }
                return hover_column(root, &name_ref, &binder);
            }
            NameRefClass::Schema => {
                return hover_schema(root, &name_ref, &binder);
            }
            NameRefClass::NamedArgParameter => {
                return hover_named_arg_parameter(root, &name_ref, &binder);
            }
            NameRefClass::Cursor => {
                return hover_cursor(root, &name_ref, &binder);
            }
            NameRefClass::PreparedStatement => {
                return hover_prepared_statement(root, &name_ref, &binder);
            }
            NameRefClass::Channel => {
                return hover_channel(root, &name_ref, &binder);
            }
        }
    }

    if let Some(name) = ast::Name::cast(parent) {
        let context = classify_name(&name)?;
        match context {
            NameClass::ColumnDefinition {
                create_table,
                column,
            } => return hover_column_definition(&create_table, &column, &binder),
            NameClass::CreateTable(create_table) => {
                return format_create_table(&create_table, &binder);
            }
            NameClass::WithTable(with_table) => return format_with_table(&with_table),
            NameClass::CreateIndex(create_index) => {
                return format_create_index(&create_index, &binder);
            }
            NameClass::CreateSequence(create_sequence) => {
                return format_create_sequence(&create_sequence, &binder);
            }
            NameClass::CreateTrigger(create_trigger) => {
                return format_create_trigger(&create_trigger, &binder);
            }
            NameClass::CreateEventTrigger(create_event_trigger) => {
                return format_create_event_trigger(&create_event_trigger);
            }
            NameClass::CreateTablespace(create_tablespace) => {
                return format_create_tablespace(&create_tablespace);
            }
            NameClass::CreateDatabase(create_database) => {
                return format_create_database(&create_database);
            }
            NameClass::CreateServer(create_server) => {
                return format_create_server(&create_server);
            }
            NameClass::CreateExtension(create_extension) => {
                return format_create_extension(&create_extension);
            }
            NameClass::CreateRole(create_role) => {
                return format_create_role(&create_role);
            }
            NameClass::CreateType(create_type) => {
                return format_create_type(&create_type, &binder);
            }
            NameClass::CreateFunction(create_function) => {
                return format_create_function(&create_function, &binder);
            }
            NameClass::CreateAggregate(create_aggregate) => {
                return format_create_aggregate(&create_aggregate, &binder);
            }
            NameClass::CreateProcedure(create_procedure) => {
                return format_create_procedure(&create_procedure, &binder);
            }
            NameClass::CreateSchema(create_schema) => {
                return format_create_schema(&create_schema);
            }
            NameClass::ViewColumnList { create_view, name } => {
                return format_view_column(&create_view, Name::from_node(&name), &binder);
            }
            NameClass::CreateView(create_view) => {
                return format_create_view(&create_view, &binder);
            }
            NameClass::DeclareCursor(declare) => {
                return format_declare_cursor(&declare);
            }
            NameClass::PrepareStatement(prepare) => {
                return format_prepare(&prepare);
            }
            NameClass::Listen(listen) => {
                return format_listen(&listen);
            }
        }
    }

    None
}

struct ColumnHover {}
impl ColumnHover {
    fn table_column(table_name: &str, column_name: &str) -> String {
        format!("column {table_name}.{column_name}")
    }
    fn schema_table_column_type(
        schema: &str,
        table_name: &str,
        column_name: &str,
        ty: &str,
    ) -> String {
        format!("column {schema}.{table_name}.{column_name} {ty}")
    }
    fn schema_table_column(schema: &str, table_name: &str, column_name: &str) -> String {
        format!("column {schema}.{table_name}.{column_name}")
    }

    fn anon_column(col_name: &str) -> String {
        format!("column {}", col_name)
    }
    fn anon_column_type(col_name: &str, ty: &str) -> String {
        format!("column {} {}", col_name, ty)
    }
}

fn hover_column(
    root: &SyntaxNode,
    name_ref: &ast::NameRef,
    binder: &binder::Binder,
) -> Option<String> {
    let column_ptrs = resolve::resolve_name_ref_ptrs(binder, root, name_ref)?;

    let results: Vec<String> = column_ptrs
        .iter()
        .filter_map(|column_ptr| {
            let column_name_node = column_ptr.to_node(root);
            format_hover_for_column_node(binder, root, &column_name_node, name_ref)
        })
        .collect();

    if results.is_empty() {
        return None;
    }

    Some(results.join("\n"))
}

fn format_hover_for_column_node(
    binder: &binder::Binder,
    root: &SyntaxNode,
    column_name_node: &squawk_syntax::SyntaxNode,
    name_ref: &ast::NameRef,
) -> Option<String> {
    for a in column_name_node.ancestors() {
        if let Some(with_table) = ast::WithTable::cast(a.clone()) {
            let cte_name = with_table.name()?;
            let column_name = if column_name_node
                .ancestors()
                .any(|a| ast::Values::can_cast(a.kind()))
            {
                Name::from_node(name_ref)
            } else {
                Name::from_string(column_name_node.text().to_string())
            };
            let table_name = Name::from_node(&cte_name);
            return Some(ColumnHover::table_column(
                &table_name.to_string(),
                &column_name.to_string(),
            ));
        }
        if let Some(paren_select) = ast::ParenSelect::cast(a.clone()) {
            // Qualified access like `t.a`
            if let Some(field_expr) = name_ref.syntax().parent().and_then(ast::FieldExpr::cast)
                && let Some(base) = field_expr.base()
                && let ast::Expr::NameRef(table_name_ref) = base
            {
                let table_name = Name::from_node(&table_name_ref);
                let column_name = Name::from_string(column_name_node.text().to_string());
                return Some(ColumnHover::table_column(
                    &table_name.to_string(),
                    &column_name.to_string(),
                ));
            }
            // Unqualified access like `a` from `select a from (select 1 a)`
            // For VALUES, use name_ref since column_name_node is the expression
            let column_name = if column_name_node
                .ancestors()
                .any(|a| ast::Values::can_cast(a.kind()))
            {
                Name::from_node(name_ref)
            } else {
                Name::from_string(column_name_node.text().to_string())
            };
            let ty = resolve::collect_paren_select_columns_with_types(binder, root, &paren_select)
                .into_iter()
                .find(|(name, _)| *name == column_name)
                .and_then(|(_, ty)| ty)?;
            return Some(ColumnHover::anon_column_type(
                &column_name.to_string(),
                &ty.to_string(),
            ));
        }

        // create view v(a) as select 1;
        // select a from v;
        //        ^
        if let Some(create_view) = ast::CreateView::cast(a.clone())
            && let Some(column_name) =
                ast::Name::cast(column_name_node.clone()).map(|name| Name::from_node(&name))
        {
            return format_view_column(&create_view, column_name, binder);
        }

        if let Some(create_table_as) = ast::CreateTableAs::cast(a.clone()) {
            let column_name = if let Some(name) = ast::Name::cast(column_name_node.clone()) {
                Name::from_node(&name)
            } else {
                continue;
            };
            let path = create_table_as.path()?;
            let (schema, table_name) = resolve::resolve_table_info(binder, &path)?;
            return Some(ColumnHover::schema_table_column(
                &schema.to_string(),
                &table_name,
                &column_name.to_string(),
            ));
        }
    }

    let column = column_name_node.ancestors().find_map(ast::Column::cast)?;
    let column_name = column.name()?;
    let ty = column.ty()?;

    let create_table = column
        .syntax()
        .ancestors()
        .find_map(ast::CreateTableLike::cast)?;
    let path = create_table.path()?;
    let (schema, table_name) = resolve::resolve_table_info(binder, &path)?;

    let schema = schema.to_string();
    let column_name = Name::from_node(&column_name);
    let ty = &ty.syntax().text().to_string();
    Some(ColumnHover::schema_table_column_type(
        &schema,
        &table_name,
        &column_name.to_string(),
        ty,
    ))
}

fn hover_composite_type_field(
    root: &SyntaxNode,
    name_ref: &ast::NameRef,
    binder: &binder::Binder,
) -> Option<String> {
    let field_ptr = resolve::resolve_name_ref_ptrs(binder, root, name_ref)?
        .into_iter()
        .next()?;
    let field_name_node = field_ptr.to_node(root);

    let column = field_name_node.ancestors().find_map(ast::Column::cast)?;
    let field_name = column.name()?.syntax().text().to_string();
    let ty = column.ty()?;

    let create_type = column
        .syntax()
        .ancestors()
        .find_map(ast::CreateType::cast)?;
    let type_path = create_type.path()?;
    let (schema, type_name) = resolve::resolve_type_info(binder, &type_path)?;

    Some(format!(
        "field {}.{}.{} {}",
        schema,
        type_name,
        field_name,
        ty.syntax().text()
    ))
}

fn hover_column_definition(
    create_table: &impl ast::HasCreateTable,
    column: &ast::Column,
    binder: &binder::Binder,
) -> Option<String> {
    let column_name = column.name()?.syntax().text().to_string();
    let ty = column.ty()?;
    let path = create_table.path()?;
    let (schema, table_name) = resolve::resolve_table_info(binder, &path)?;
    let ty = ty.syntax().text().to_string();
    Some(ColumnHover::schema_table_column_type(
        &schema.to_string(),
        &table_name,
        &column_name,
        &ty,
    ))
}

fn hover_table(
    root: &SyntaxNode,
    name_ref: &ast::NameRef,
    binder: &binder::Binder,
) -> Option<String> {
    if let Some(result) = hover_subquery_table(name_ref) {
        return Some(result);
    }

    let table_ptr = resolve::resolve_name_ref_ptrs(binder, root, name_ref)?
        .into_iter()
        .next()?;

    hover_table_from_ptr(root, &table_ptr, binder)
}

fn hover_table_from_ptr(
    root: &SyntaxNode,
    table_ptr: &squawk_syntax::SyntaxNodePtr,
    binder: &binder::Binder,
) -> Option<String> {
    let table_name_node = table_ptr.to_node(root);

    match resolve::find_table_source(&table_name_node)? {
        resolve::TableSource::WithTable(with_table) => format_with_table(&with_table),
        resolve::TableSource::CreateView(create_view) => format_create_view(&create_view, binder),
        resolve::TableSource::CreateMaterializedView(create_materialized_view) => {
            format_create_materialized_view(&create_materialized_view, binder)
        }
        resolve::TableSource::CreateTable(create_table) => {
            format_create_table(&create_table, binder)
        }
        resolve::TableSource::ParenSelect(paren_select) => format_paren_select(&paren_select),
    }
}

fn hover_qualified_star(
    root: &SyntaxNode,
    field_expr: &ast::FieldExpr,
    binder: &binder::Binder,
) -> Option<String> {
    let table_ptr = resolve::resolve_qualified_star_table_ptr(binder, field_expr)?;
    hover_qualified_star_columns(root, &table_ptr, binder)
}

fn hover_unqualified_star(
    root: &SyntaxNode,
    target: &ast::Target,
    binder: &binder::Binder,
) -> Option<String> {
    let table_ptrs = resolve::resolve_unqualified_star_table_ptrs(binder, target)?;
    let mut results = vec![];
    for table_ptr in table_ptrs {
        if let Some(columns) = hover_qualified_star_columns(root, &table_ptr, binder) {
            results.push(columns);
        }
    }

    if results.is_empty() {
        return None;
    }

    Some(results.join("\n"))
}

fn hover_unqualified_star_in_arg_list(
    root: &SyntaxNode,
    arg_list: &ast::ArgList,
    binder: &binder::Binder,
) -> Option<String> {
    let table_ptrs = resolve::resolve_unqualified_star_in_arg_list_ptrs(binder, arg_list)?;
    let mut results = vec![];
    for table_ptr in table_ptrs {
        if let Some(columns) = hover_qualified_star_columns(root, &table_ptr, binder) {
            results.push(columns);
        }
    }

    if results.is_empty() {
        return None;
    }

    Some(results.join("\n"))
}

fn hover_subquery_table(name_ref: &ast::NameRef) -> Option<String> {
    let select = name_ref.syntax().ancestors().find_map(ast::Select::cast)?;
    let from_clause = select.from_clause()?;
    let qualifier = Name::from_node(name_ref);
    let from_item = resolve::find_from_item_in_from_clause(&from_clause, &qualifier)?;
    let paren_select = from_item.paren_select()?;
    format_subquery_table(name_ref, &paren_select)
}

fn format_subquery_table(
    name_ref: &ast::NameRef,
    paren_select: &ast::ParenSelect,
) -> Option<String> {
    let name = name_ref.syntax().text().to_string();
    let query = paren_select.syntax().text().to_string();
    Some(format!("subquery {} as {}", name, query))
}

fn hover_qualified_star_columns(
    root: &SyntaxNode,
    table_ptr: &squawk_syntax::SyntaxNodePtr,
    binder: &binder::Binder,
) -> Option<String> {
    let table_name_node = table_ptr.to_node(root);

    if let Some(paren_select) = ast::ParenSelect::cast(table_name_node.clone()) {
        return hover_qualified_star_columns_from_subquery(root, &paren_select, binder);
    }

    match resolve::find_table_source(&table_name_node)? {
        resolve::TableSource::WithTable(with_table) => {
            hover_qualified_star_columns_from_cte(&with_table)
        }
        resolve::TableSource::CreateTable(create_table) => {
            hover_qualified_star_columns_from_table(root, &create_table, binder)
        }
        resolve::TableSource::CreateView(create_view) => {
            hover_qualified_star_columns_from_view(&create_view, binder)
        }
        resolve::TableSource::CreateMaterializedView(create_materialized_view) => {
            hover_qualified_star_columns_from_materialized_view(&create_materialized_view, binder)
        }
        resolve::TableSource::ParenSelect(paren_select) => {
            hover_qualified_star_columns_from_subquery(root, &paren_select, binder)
        }
    }
}

fn hover_qualified_star_columns_from_table(
    root: &SyntaxNode,
    create_table: &impl ast::HasCreateTable,
    binder: &binder::Binder,
) -> Option<String> {
    let path = create_table.path()?;
    let (schema, table_name) = resolve::resolve_table_info(binder, &path)?;
    let schema = schema.to_string();
    let results: Vec<String> = resolve::collect_table_columns(binder, root, create_table)
        .into_iter()
        .filter_map(|column| {
            let column_name = Name::from_node(&column.name()?);
            let ty = column.ty()?;
            let ty = &ty.syntax().text().to_string();
            Some(ColumnHover::schema_table_column_type(
                &schema,
                &table_name,
                &column_name.to_string(),
                ty,
            ))
        })
        .collect();

    if results.is_empty() {
        return None;
    }

    Some(results.join("\n"))
}

fn hover_qualified_star_columns_from_cte(with_table: &ast::WithTable) -> Option<String> {
    let cte_name = Name::from_node(&with_table.name()?);
    let column_names = resolve::collect_with_table_column_names(with_table);
    let results: Vec<String> = column_names
        .iter()
        .map(|column_name| {
            ColumnHover::table_column(&cte_name.to_string(), &column_name.to_string())
        })
        .collect();

    if results.is_empty() {
        return None;
    }

    Some(results.join("\n"))
}

fn hover_qualified_star_columns_from_view(
    create_view: &ast::CreateView,
    binder: &binder::Binder,
) -> Option<String> {
    let path = create_view.path()?;
    let (schema, view_name) = resolve::resolve_view_info(binder, &path)?;

    let schema_str = schema.to_string();
    let column_names = resolve::collect_view_column_names(create_view);
    let results: Vec<String> = column_names
        .iter()
        .map(|column_name| {
            ColumnHover::schema_table_column(&schema_str, &view_name, &column_name.to_string())
        })
        .collect();

    if results.is_empty() {
        return None;
    }

    Some(results.join("\n"))
}

fn hover_qualified_star_columns_from_materialized_view(
    create_materialized_view: &ast::CreateMaterializedView,
    binder: &binder::Binder,
) -> Option<String> {
    let path = create_materialized_view.path()?;
    let (schema, view_name) = resolve::resolve_view_info(binder, &path)?;

    let schema_str = schema.to_string();
    let column_names = resolve::collect_materialized_view_column_names(create_materialized_view);
    let results: Vec<String> = column_names
        .iter()
        .map(|column_name| {
            ColumnHover::schema_table_column(&schema_str, &view_name, &column_name.to_string())
        })
        .collect();

    if results.is_empty() {
        return None;
    }

    Some(results.join("\n"))
}

fn hover_qualified_star_columns_from_subquery(
    root: &SyntaxNode,
    paren_select: &ast::ParenSelect,
    binder: &binder::Binder,
) -> Option<String> {
    let ast::SelectVariant::Select(select) = paren_select.select()? else {
        return None;
    };

    let select_clause = select.select_clause()?;
    let target_list = select_clause.target_list()?;

    let mut results = vec![];
    let subquery_alias = subquery_alias_name(paren_select);

    for target in target_list.targets() {
        if target.star_token().is_some() {
            let table_ptrs = resolve::resolve_unqualified_star_table_ptrs(binder, &target)?;
            for table_ptr in table_ptrs {
                if let Some(columns) = hover_qualified_star_columns(root, &table_ptr, binder) {
                    results.push(columns)
                }
            }
            continue;
        }

        if let Some(result) =
            hover_subquery_target_column(root, &target, subquery_alias.as_ref(), binder)
        {
            results.push(result);
        }
    }

    if results.is_empty() {
        return None;
    }

    Some(results.join("\n"))
}

fn subquery_alias_name(paren_select: &ast::ParenSelect) -> Option<Name> {
    let from_item = paren_select
        .syntax()
        .ancestors()
        .find_map(ast::FromItem::cast)?;
    let alias_name = from_item.alias()?.name()?;
    Some(Name::from_node(&alias_name))
}

fn hover_subquery_target_column(
    root: &SyntaxNode,
    target: &ast::Target,
    subquery_alias: Option<&Name>,
    binder: &binder::Binder,
) -> Option<String> {
    if let Some(alias) = subquery_alias
        && let Some((col_name, _node)) = ColumnName::from_target(target.clone())
        && let Some(col_name) = col_name.to_string()
    {
        return Some(ColumnHover::table_column(&alias.to_string(), &col_name));
    }

    let result = match target.expr()? {
        ast::Expr::NameRef(name_ref) => hover_column(root, &name_ref, binder),
        ast::Expr::FieldExpr(field_expr) => {
            let field = field_expr.field()?;
            hover_column(root, &field, binder)
        }
        _ => None,
    };

    if result.is_some() {
        return result;
    }

    if let Some((col_name, _node)) = ColumnName::from_target(target.clone())
        && let Some(col_name) = col_name.to_string()
    {
        return Some(ColumnHover::anon_column(&col_name));
    }

    None
}

fn hover_index(
    root: &SyntaxNode,
    name_ref: &ast::NameRef,
    binder: &binder::Binder,
) -> Option<String> {
    let index_ptr = resolve::resolve_name_ref_ptrs(binder, root, name_ref)?
        .into_iter()
        .next()?;

    let index_name_node = index_ptr.to_node(root);

    let create_index = index_name_node
        .ancestors()
        .find_map(ast::CreateIndex::cast)?;

    format_create_index(&create_index, binder)
}

fn hover_sequence(
    root: &SyntaxNode,
    name_ref: &ast::NameRef,
    binder: &binder::Binder,
) -> Option<String> {
    let sequence_ptr = resolve::resolve_name_ref_ptrs(binder, root, name_ref)?
        .into_iter()
        .next()?;

    let sequence_name_node = sequence_ptr.to_node(root);

    let create_sequence = sequence_name_node
        .ancestors()
        .find_map(ast::CreateSequence::cast)?;

    format_create_sequence(&create_sequence, binder)
}

fn hover_trigger(
    root: &SyntaxNode,
    name_ref: &ast::NameRef,
    binder: &binder::Binder,
) -> Option<String> {
    let trigger_ptr = resolve::resolve_name_ref_ptrs(binder, root, name_ref)?
        .into_iter()
        .next()?;

    let trigger_name_node = trigger_ptr.to_node(root);

    let create_trigger = trigger_name_node
        .ancestors()
        .find_map(ast::CreateTrigger::cast)?;

    format_create_trigger(&create_trigger, binder)
}

fn hover_policy(
    root: &SyntaxNode,
    name_ref: &ast::NameRef,
    binder: &binder::Binder,
) -> Option<String> {
    let policy_ptr = resolve::resolve_name_ref_ptrs(binder, root, name_ref)?
        .into_iter()
        .next()?;

    let policy_name_node = policy_ptr.to_node(root);

    let create_policy = policy_name_node
        .ancestors()
        .find_map(ast::CreatePolicy::cast)?;

    format_create_policy(&create_policy, binder)
}

fn hover_event_trigger(
    root: &SyntaxNode,
    name_ref: &ast::NameRef,
    binder: &binder::Binder,
) -> Option<String> {
    let event_trigger_ptr = resolve::resolve_name_ref_ptrs(binder, root, name_ref)?
        .into_iter()
        .next()?;

    let event_trigger_name_node = event_trigger_ptr.to_node(root);

    let create_event_trigger = event_trigger_name_node
        .ancestors()
        .find_map(ast::CreateEventTrigger::cast)?;

    format_create_event_trigger(&create_event_trigger)
}

fn hover_tablespace(
    root: &SyntaxNode,
    name_ref: &ast::NameRef,
    binder: &binder::Binder,
) -> Option<String> {
    let tablespace_ptr = resolve::resolve_name_ref_ptrs(binder, root, name_ref)?
        .into_iter()
        .next()?;
    let tablespace_name_node = tablespace_ptr.to_node(root);
    Some(format!("tablespace {}", tablespace_name_node.text()))
}

fn hover_database(
    root: &SyntaxNode,
    name_ref: &ast::NameRef,
    binder: &binder::Binder,
) -> Option<String> {
    let database_ptr = resolve::resolve_name_ref_ptrs(binder, root, name_ref)?
        .into_iter()
        .next()?;
    let database_name_node = database_ptr.to_node(root);
    Some(format!("database {}", database_name_node.text()))
}

fn hover_server(
    root: &SyntaxNode,
    name_ref: &ast::NameRef,
    binder: &binder::Binder,
) -> Option<String> {
    let server_ptr = resolve::resolve_name_ref_ptrs(binder, root, name_ref)?
        .into_iter()
        .next()?;
    let server_name_node = server_ptr.to_node(root);
    Some(format!("server {}", server_name_node.text()))
}

fn hover_extension(
    root: &SyntaxNode,
    name_ref: &ast::NameRef,
    binder: &binder::Binder,
) -> Option<String> {
    let extension_ptr = resolve::resolve_name_ref_ptrs(binder, root, name_ref)?
        .into_iter()
        .next()?;
    let extension_name_node = extension_ptr.to_node(root);
    Some(format!("extension {}", extension_name_node.text()))
}

fn hover_role(
    root: &SyntaxNode,
    name_ref: &ast::NameRef,
    binder: &binder::Binder,
) -> Option<String> {
    let role_ptr = resolve::resolve_name_ref_ptrs(binder, root, name_ref)?
        .into_iter()
        .next()?;
    let role_name_node = role_ptr.to_node(root);
    Some(format!("role {}", role_name_node.text()))
}

fn hover_cursor(
    root: &SyntaxNode,
    name_ref: &ast::NameRef,
    binder: &binder::Binder,
) -> Option<String> {
    let cursor_ptr = resolve::resolve_name_ref_ptrs(binder, root, name_ref)?
        .into_iter()
        .next()?;
    let cursor_name_node = cursor_ptr.to_node(root);
    let declare = cursor_name_node.ancestors().find_map(ast::Declare::cast)?;
    format_declare_cursor(&declare)
}

fn hover_prepared_statement(
    root: &SyntaxNode,
    name_ref: &ast::NameRef,
    binder: &binder::Binder,
) -> Option<String> {
    let statement_ptr = resolve::resolve_name_ref_ptrs(binder, root, name_ref)?
        .into_iter()
        .next()?;
    let statement_name_node = statement_ptr.to_node(root);
    let prepare = statement_name_node
        .ancestors()
        .find_map(ast::Prepare::cast)?;
    format_prepare(&prepare)
}

fn hover_channel(
    root: &SyntaxNode,
    name_ref: &ast::NameRef,
    binder: &binder::Binder,
) -> Option<String> {
    let channel_ptr = resolve::resolve_name_ref_ptrs(binder, root, name_ref)?
        .into_iter()
        .next()?;
    let channel_name_node = channel_ptr.to_node(root);
    let listen = channel_name_node.ancestors().find_map(ast::Listen::cast)?;
    format_listen(&listen)
}

fn hover_type(
    root: &SyntaxNode,
    name_ref: &ast::NameRef,
    binder: &binder::Binder,
) -> Option<String> {
    let type_ptr = resolve::resolve_name_ref_ptrs(binder, root, name_ref)?
        .into_iter()
        .next()?;

    let type_name_node = type_ptr.to_node(root);

    let create_type = type_name_node.ancestors().find_map(ast::CreateType::cast)?;

    format_create_type(&create_type, binder)
}

fn format_declare_cursor(declare: &ast::Declare) -> Option<String> {
    let name = declare.name()?;
    let query = declare.query()?;
    Some(format!(
        "cursor {} for {}",
        name.syntax().text(),
        query.syntax().text()
    ))
}

fn format_prepare(prepare: &ast::Prepare) -> Option<String> {
    let name = prepare.name()?;
    let stmt = prepare.preparable_stmt()?;
    Some(format!(
        "prepare {} as {}",
        name.syntax().text(),
        stmt.syntax().text()
    ))
}

fn format_listen(listen: &ast::Listen) -> Option<String> {
    let name = listen.name()?;
    Some(format!("listen {}", name.syntax().text()))
}

fn format_create_table(
    create_table: &impl ast::HasCreateTable,
    binder: &binder::Binder,
) -> Option<String> {
    let path = create_table.path()?;
    let (schema, table_name) = resolve::resolve_table_info(binder, &path)?;
    let schema = schema.to_string();
    let args = create_table.table_arg_list()?.syntax().text().to_string();

    let foreign = if create_table.syntax().kind() == SyntaxKind::CREATE_FOREIGN_TABLE {
        "foreign "
    } else {
        ""
    };

    Some(format!("{foreign}table {schema}.{table_name}{args}"))
}

fn format_create_view(create_view: &ast::CreateView, binder: &binder::Binder) -> Option<String> {
    let path = create_view.path()?;
    let (schema, view_name) = resolve::resolve_view_info(binder, &path)?;
    let schema = schema.to_string();

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

fn format_create_materialized_view(
    create_materialized_view: &ast::CreateMaterializedView,
    binder: &binder::Binder,
) -> Option<String> {
    let path = create_materialized_view.path()?;
    let (schema, view_name) = resolve::resolve_view_info(binder, &path)?;
    let schema = schema.to_string();

    let column_list = create_materialized_view
        .column_list()
        .map(|cl| cl.syntax().text().to_string())
        .unwrap_or_default();

    let query = create_materialized_view
        .query()?
        .syntax()
        .text()
        .to_string();

    Some(format!(
        "materialized view {}.{}{} as {}",
        schema, view_name, column_list, query
    ))
}

fn format_view_column(
    create_view: &ast::CreateView,
    column_name: Name,
    binder: &binder::Binder,
) -> Option<String> {
    let path = create_view.path()?;
    let (schema, view_name) = resolve::resolve_view_info(binder, &path)?;
    Some(ColumnHover::schema_table_column(
        &schema.to_string(),
        &view_name,
        &column_name.to_string(),
    ))
}

fn format_with_table(with_table: &ast::WithTable) -> Option<String> {
    let name = with_table.name()?.syntax().text().to_string();
    let query = with_table.query()?.syntax().text().to_string();
    Some(format!("with {} as ({})", name, query))
}

fn format_paren_select(paren_select: &ast::ParenSelect) -> Option<String> {
    let query = paren_select.select()?.syntax().text().to_string();
    Some(format!("({})", query))
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

fn format_create_sequence(
    create_sequence: &ast::CreateSequence,
    binder: &binder::Binder,
) -> Option<String> {
    let path = create_sequence.path()?;
    let (schema, sequence_name) = resolve::resolve_sequence_info(binder, &path)?;

    Some(format!("sequence {}.{}", schema, sequence_name))
}

fn format_create_trigger(
    create_trigger: &ast::CreateTrigger,
    binder: &binder::Binder,
) -> Option<String> {
    let trigger_name = create_trigger.name()?.syntax().text().to_string();
    let on_table_path = create_trigger.on_table()?.path()?;

    let (schema, table_name) = resolve::resolve_table_info(binder, &on_table_path)?;
    Some(format!(
        "trigger {}.{} on {}.{}",
        schema, trigger_name, schema, table_name
    ))
}

fn format_create_policy(
    create_policy: &ast::CreatePolicy,
    binder: &binder::Binder,
) -> Option<String> {
    let policy_name = create_policy.name()?.syntax().text().to_string();
    let on_table_path = create_policy.on_table()?.path()?;

    let (schema, table_name) = resolve::resolve_table_info(binder, &on_table_path)?;
    Some(format!(
        "policy {}.{} on {}.{}",
        schema, policy_name, schema, table_name
    ))
}

fn format_create_event_trigger(create_event_trigger: &ast::CreateEventTrigger) -> Option<String> {
    let name = create_event_trigger.name()?.syntax().text().to_string();
    Some(format!("event trigger {}", name))
}

fn format_create_tablespace(create_tablespace: &ast::CreateTablespace) -> Option<String> {
    let name = create_tablespace.name()?.syntax().text().to_string();
    Some(format!("tablespace {}", name))
}

fn format_create_database(create_database: &ast::CreateDatabase) -> Option<String> {
    let name = create_database.name()?.syntax().text().to_string();
    Some(format!("database {}", name))
}

fn format_create_server(create_server: &ast::CreateServer) -> Option<String> {
    let name = create_server.name()?.syntax().text().to_string();
    Some(format!("server {}", name))
}

fn format_create_extension(create_extension: &ast::CreateExtension) -> Option<String> {
    let name = create_extension.name()?.syntax().text().to_string();
    Some(format!("extension {}", name))
}

fn format_create_role(create_role: &ast::CreateRole) -> Option<String> {
    let name = create_role.name()?.syntax().text().to_string();
    Some(format!("role {}", name))
}

fn index_schema(create_index: &ast::CreateIndex, binder: &binder::Binder) -> Option<String> {
    let position = create_index.syntax().text_range().start();
    let search_path = binder.search_path_at(position);
    search_path.first().map(|s| s.to_string())
}

fn format_create_type(create_type: &ast::CreateType, binder: &binder::Binder) -> Option<String> {
    let path = create_type.path()?;
    let (schema, type_name) = resolve::resolve_type_info(binder, &path)?;

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

fn hover_schema(
    root: &SyntaxNode,
    name_ref: &ast::NameRef,
    binder: &binder::Binder,
) -> Option<String> {
    let schema_ptr = resolve::resolve_name_ref_ptrs(binder, root, name_ref)?
        .into_iter()
        .next()?;

    let schema_name_node = schema_ptr.to_node(root);

    let create_schema = schema_name_node
        .ancestors()
        .find_map(ast::CreateSchema::cast)?;

    format_create_schema(&create_schema)
}

fn create_schema_name(create_schema: &ast::CreateSchema) -> Option<String> {
    if let Some(schema_name) = create_schema.name() {
        return Some(schema_name.syntax().text().to_string());
    }

    create_schema
        .role()
        .and_then(|r| r.name())
        .map(|n| n.syntax().text().to_string())
}

fn format_create_schema(create_schema: &ast::CreateSchema) -> Option<String> {
    let schema_name = create_schema_name(create_schema)?;
    Some(format!("schema {}", schema_name))
}

fn hover_function(
    root: &SyntaxNode,
    name_ref: &ast::NameRef,
    binder: &binder::Binder,
) -> Option<String> {
    let function_ptr = resolve::resolve_name_ref_ptrs(binder, root, name_ref)?
        .into_iter()
        .next()?;

    let function_name_node = function_ptr.to_node(root);

    let create_function = function_name_node
        .ancestors()
        .find_map(ast::CreateFunction::cast)?;

    format_create_function(&create_function, binder)
}

fn hover_named_arg_parameter(
    root: &SyntaxNode,
    name_ref: &ast::NameRef,
    binder: &binder::Binder,
) -> Option<String> {
    let param_ptr = resolve::resolve_name_ref_ptrs(binder, root, name_ref)?
        .into_iter()
        .next()?;
    let param_name_node = param_ptr.to_node(root);
    let param = param_name_node.ancestors().find_map(ast::Param::cast)?;
    let param_name = param.name().map(|name| Name::from_node(&name))?;
    let param_type = param.ty().map(|ty| ty.syntax().text().to_string());

    for ancestor in param_name_node.ancestors() {
        if let Some(create_function) = ast::CreateFunction::cast(ancestor.clone()) {
            let path = create_function.path()?;
            let (schema, function_name) = resolve::resolve_function_info(binder, &path)?;
            return Some(format_param_hover(
                schema,
                function_name,
                param_name,
                param_type,
            ));
        }
        if let Some(create_procedure) = ast::CreateProcedure::cast(ancestor.clone()) {
            let path = create_procedure.path()?;
            let (schema, procedure_name) = resolve::resolve_procedure_info(binder, &path)?;
            return Some(format_param_hover(
                schema,
                procedure_name,
                param_name,
                param_type,
            ));
        }
        if let Some(create_aggregate) = ast::CreateAggregate::cast(ancestor) {
            let path = create_aggregate.path()?;
            let (schema, aggregate_name) = resolve::resolve_aggregate_info(binder, &path)?;
            return Some(format_param_hover(
                schema,
                aggregate_name,
                param_name,
                param_type,
            ));
        }
    }

    None
}

fn format_param_hover(
    schema: Schema,
    routine_name: String,
    param_name: Name,
    param_type: Option<String>,
) -> String {
    if let Some(param_type) = param_type {
        return format!(
            "parameter {}.{}.{} {}",
            schema, routine_name, param_name, param_type
        );
    }

    format!("parameter {}.{}.{}", schema, routine_name, param_name)
}

fn format_create_function(
    create_function: &ast::CreateFunction,
    binder: &binder::Binder,
) -> Option<String> {
    let path = create_function.path()?;
    let (schema, function_name) = resolve::resolve_function_info(binder, &path)?;

    let param_list = create_function.param_list()?;
    let params = param_list.syntax().text().to_string();

    let ret_type = create_function.ret_type()?;
    let return_type = ret_type.syntax().text().to_string();

    Some(format!(
        "function {}.{}{} {}",
        schema, function_name, params, return_type
    ))
}

fn hover_aggregate(
    root: &SyntaxNode,
    name_ref: &ast::NameRef,
    binder: &binder::Binder,
) -> Option<String> {
    let aggregate_ptr = resolve::resolve_name_ref_ptrs(binder, root, name_ref)?
        .into_iter()
        .next()?;

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
    let (schema, aggregate_name) = resolve::resolve_aggregate_info(binder, &path)?;

    let param_list = create_aggregate.param_list()?;
    let params = param_list.syntax().text().to_string();

    Some(format!("aggregate {}.{}{}", schema, aggregate_name, params))
}

fn hover_procedure(
    root: &SyntaxNode,
    name_ref: &ast::NameRef,
    binder: &binder::Binder,
) -> Option<String> {
    let procedure_ptr = resolve::resolve_name_ref_ptrs(binder, root, name_ref)?
        .into_iter()
        .next()?;

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
    let (schema, procedure_name) = resolve::resolve_procedure_info(binder, &path)?;

    let param_list = create_procedure.param_list()?;
    let params = param_list.syntax().text().to_string();

    Some(format!("procedure {}.{}{}", schema, procedure_name, params))
}

fn hover_routine(
    root: &SyntaxNode,
    name_ref: &ast::NameRef,
    binder: &binder::Binder,
) -> Option<String> {
    let routine_ptr = resolve::resolve_name_ref_ptrs(binder, root, name_ref)?
        .into_iter()
        .next()?;
    let routine_name = routine_ptr.to_node(root);

    for a in routine_name.ancestors() {
        if let Some(create_function) = ast::CreateFunction::cast(a.clone()) {
            return format_create_function(&create_function, binder);
        }
        if let Some(create_aggregate) = ast::CreateAggregate::cast(a.clone()) {
            return format_create_aggregate(&create_aggregate, binder);
        }
        if let Some(create_procedure) = ast::CreateProcedure::cast(a) {
            return format_create_procedure(&create_procedure, binder);
        }
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
    fn hover_on_named_arg_param() {
        assert_snapshot!(check_hover("
create function foo(bar_param int) returns int as $$ select 1 $$ language sql;
select foo(bar_param$0 := 5);
"), @r"
        hover: parameter public.foo.bar_param int
          ╭▸ 
        3 │ select foo(bar_param := 5);
          ╰╴                   ─ hover
        ");
    }

    #[test]
    fn hover_on_named_arg_param_schema_qualified() {
        assert_snapshot!(check_hover("
create schema s;
create function s.foo(my_param int) returns int as $$ select 1 $$ language sql;
select s.foo(my_param$0 := 10);
"), @r"
        hover: parameter s.foo.my_param int
          ╭▸ 
        4 │ select s.foo(my_param := 10);
          ╰╴                    ─ hover
        ");
    }

    #[test]
    fn hover_on_named_arg_param_procedure() {
        assert_snapshot!(check_hover("
create procedure proc(param_x int) as 'select 1' language sql;
call proc(param_x$0 := 42);
"), @r"
        hover: parameter public.proc.param_x int
          ╭▸ 
        3 │ call proc(param_x := 42);
          ╰╴                ─ hover
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
    fn hover_on_subquery_qualified_table_ref() {
        assert_snapshot!(check_hover("
select t$0.a from (select 1 a) t;
"), @r"
        hover: subquery t as (select 1 a)
          ╭▸ 
        2 │ select t.a from (select 1 a) t;
          ╰╴       ─ hover
        ");
    }

    #[test]
    fn hover_on_subquery_qualified_column_ref() {
        assert_snapshot!(check_hover("
select t.a$0 from (select 1 a) t;
"), @r"
        hover: column t.a
          ╭▸ 
        2 │ select t.a from (select 1 a) t;
          ╰╴         ─ hover
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
    fn hover_on_select_qualified_star() {
        assert_snapshot!(check_hover("
create table u(id int, b int);
select u.*$0 from u;
"), @r"
        hover: column public.u.id int
              column public.u.b int
          ╭▸ 
        3 │ select u.* from u;
          ╰╴         ─ hover
        ");
    }

    #[test]
    fn hover_on_select_unqualified_star() {
        assert_snapshot!(check_hover("
create table u(id int, b int);
select *$0 from u;
"), @r"
        hover: column public.u.id int
              column public.u.b int
          ╭▸ 
        3 │ select * from u;
          ╰╴       ─ hover
        ");
    }

    #[test]
    fn hover_on_select_count_star() {
        assert_snapshot!(check_hover("
create table u(id int, b int);
select count(*$0) from u;
"), @r"
        hover: column public.u.id int
              column public.u.b int
          ╭▸ 
        3 │ select count(*) from u;
          ╰╴             ─ hover
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
    fn hover_on_create_schema_authorization() {
        assert_snapshot!(check_hover("
create schema authorization foo$0;
"), @r"
        hover: schema foo
          ╭▸ 
        2 │ create schema authorization foo;
          ╰╴                              ─ hover
        ");
    }

    #[test]
    fn hover_on_drop_schema_authorization() {
        assert_snapshot!(check_hover("
create schema authorization foo;
drop schema foo$0;
"), @r"
        hover: schema foo
          ╭▸ 
        3 │ drop schema foo;
          ╰╴              ─ hover
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
    fn hover_on_subquery_column() {
        assert_snapshot!(check_hover("
select a$0 from (select 1 a);
"), @r"
        hover: column a integer
          ╭▸ 
        2 │ select a from (select 1 a);
          ╰╴       ─ hover
        ");
    }

    #[test]
    fn hover_on_subquery_values_column() {
        assert_snapshot!(check_hover("
select column1$0 from (values (1, 'foo'));
"), @r"
        hover: column column1 integer
          ╭▸ 
        2 │ select column1 from (values (1, 'foo'));
          ╰╴             ─ hover
        ");
    }

    #[test]
    fn hover_on_cte_qualified_star() {
        assert_snapshot!(check_hover("
with u as (select 1 id, 2 b)
select u.*$0 from u;
"), @r"
        hover: column u.id
              column u.b
          ╭▸ 
        3 │ select u.* from u;
          ╰╴         ─ hover
        ");
    }

    #[test]
    fn hover_on_cte_values_qualified_star() {
        assert_snapshot!(check_hover("
with t as (values (1, 2), (3, 4))
select t.*$0 from t;
"), @r"
        hover: column t.column1
              column t.column2
          ╭▸ 
        3 │ select t.* from t;
          ╰╴         ─ hover
        ");
    }

    #[test]
    fn hover_on_star_with_subquery_from_cte() {
        assert_snapshot!(check_hover("
with u as (select 1 id, 2 b)
select *$0 from (select *, *, * from u);
"), @r"
        hover: column u.id
              column u.b
              column u.id
              column u.b
              column u.id
              column u.b
          ╭▸ 
        3 │ select * from (select *, *, * from u);
          ╰╴       ─ hover
        ");
    }

    #[test]
    fn hover_on_star_with_subquery_from_table() {
        assert_snapshot!(check_hover("
create table t(a int, b int);
select *$0 from (select a from t);
"), @r"
        hover: column public.t.a int
          ╭▸ 
        3 │ select * from (select a from t);
          ╰╴       ─ hover
        ");
    }

    #[test]
    fn hover_on_star_with_subquery_literal() {
        assert_snapshot!(check_hover("
select *$0 from (select 1);
"), @r"
        hover: column ?column?
          ╭▸ 
        2 │ select * from (select 1);
          ╰╴       ─ hover
        ");
    }

    #[test]
    fn hover_on_star_with_subquery_literal_with_alias() {
        assert_snapshot!(check_hover("
select *$0 from (select 1) as sub;
"), @r"
        hover: column sub.?column?
          ╭▸ 
        2 │ select * from (select 1) as sub;
          ╰╴       ─ hover
        ");
    }

    #[test]
    fn hover_on_view_qualified_star() {
        assert_snapshot!(check_hover("
create view v as select 1 id, 2 b;
select v.*$0 from v;
"), @r"
        hover: column public.v.id
              column public.v.b
          ╭▸ 
        3 │ select v.* from v;
          ╰╴         ─ hover
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
    fn hover_on_select_column_from_create_table_as() {
        assert_snapshot!(check_hover("
create table t as select 1 a;
select a$0 from t;
"), @r"
        hover: column public.t.a
          ╭▸ 
        3 │ select a from t;
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

    #[test]
    fn hover_composite_type_field() {
        assert_snapshot!(check_hover("
create type person_info as (name varchar(50), age int);
with team as (
    select 1 as id, ('Alice', 30)::person_info as member
)
select (member).name$0, (member).age from team;
"), @r"
        hover: field public.person_info.name varchar(50)
          ╭▸ 
        6 │ select (member).name, (member).age from team;
          ╰╴                   ─ hover
        ");
    }

    #[test]
    fn hover_composite_type_field_age() {
        assert_snapshot!(check_hover("
create type person_info as (name varchar(50), age int);
with team as (
    select 1 as id, ('Alice', 30)::person_info as member
)
select (member).name, (member).age$0 from team;
"), @r"
        hover: field public.person_info.age int
          ╭▸ 
        6 │ select (member).name, (member).age from team;
          ╰╴                                 ─ hover
        ");
    }

    #[test]
    fn hover_composite_type_field_nested_parens() {
        assert_snapshot!(check_hover("
create type person_info as (name varchar(50), age int);
with team as (
    select 1 as id, ('Alice', 30)::person_info as member
)
select ((((member))).name$0) from team;
"), @r"
        hover: field public.person_info.name varchar(50)
          ╭▸ 
        6 │ select ((((member))).name) from team;
          ╰╴                        ─ hover
        ");
    }

    #[test]
    fn hover_on_join_using_column() {
        assert_snapshot!(check_hover("
create table t(id int);
create table u(id int);
select * from t join u using (id$0);
"), @r"
        hover: column public.t.id int
              column public.u.id int
          ╭▸ 
        4 │ select * from t join u using (id);
          ╰╴                               ─ hover
        ");
    }

    #[test]
    fn hover_on_truncate_table() {
        assert_snapshot!(check_hover("
create table users(id int, email text);
truncate table users$0;
"), @r"
        hover: table public.users(id int, email text)
          ╭▸ 
        3 │ truncate table users;
          ╰╴                   ─ hover
        ");
    }

    #[test]
    fn hover_on_truncate_table_without_table_keyword() {
        assert_snapshot!(check_hover("
create table users(id int, email text);
truncate users$0;
"), @r"
        hover: table public.users(id int, email text)
          ╭▸ 
        3 │ truncate users;
          ╰╴             ─ hover
        ");
    }

    #[test]
    fn hover_on_lock_table() {
        assert_snapshot!(check_hover("
create table users(id int, email text);
lock table users$0;
"), @r"
        hover: table public.users(id int, email text)
          ╭▸ 
        3 │ lock table users;
          ╰╴               ─ hover
        ");
    }

    #[test]
    fn hover_on_lock_table_without_table_keyword() {
        assert_snapshot!(check_hover("
create table users(id int, email text);
lock users$0;
"), @r"
        hover: table public.users(id int, email text)
          ╭▸ 
        3 │ lock users;
          ╰╴         ─ hover
        ");
    }

    #[test]
    fn hover_on_vacuum_table() {
        assert_snapshot!(check_hover("
create table users(id int, email text);
vacuum users$0;
"), @r"
        hover: table public.users(id int, email text)
          ╭▸ 
        3 │ vacuum users;
          ╰╴           ─ hover
        ");
    }

    #[test]
    fn hover_on_vacuum_with_analyze() {
        assert_snapshot!(check_hover("
create table users(id int, email text);
vacuum analyze users$0;
"), @r"
        hover: table public.users(id int, email text)
          ╭▸ 
        3 │ vacuum analyze users;
          ╰╴                   ─ hover
        ");
    }

    #[test]
    fn hover_on_alter_table() {
        assert_snapshot!(check_hover("
create table users(id int, email text);
alter table users$0 alter email set not null;
"), @r"
        hover: table public.users(id int, email text)
          ╭▸ 
        3 │ alter table users alter email set not null;
          ╰╴                ─ hover
        ");
    }

    #[test]
    fn hover_on_alter_table_column() {
        assert_snapshot!(check_hover("
create table users(id int, email text);
alter table users alter email$0 set not null;
"), @r"
        hover: column public.users.email text
          ╭▸ 
        3 │ alter table users alter email set not null;
          ╰╴                            ─ hover
        ");
    }

    #[test]
    fn hover_on_refresh_materialized_view() {
        assert_snapshot!(check_hover("
create materialized view mv as select 1;
refresh materialized view mv$0;
"), @r"
        hover: materialized view public.mv as select 1
          ╭▸ 
        3 │ refresh materialized view mv;
          ╰╴                           ─ hover
        ");
    }

    #[test]
    fn hover_on_reindex_table() {
        assert_snapshot!(check_hover("
create table users(id int);
reindex table users$0;
"), @r"
        hover: table public.users(id int)
          ╭▸ 
        3 │ reindex table users;
          ╰╴                  ─ hover
        ");
    }

    #[test]
    fn hover_on_reindex_index() {
        assert_snapshot!(check_hover("
create table t(c int);
create index idx on t(c);
reindex index idx$0;
"), @r"
        hover: index public.idx on public.t(c)
          ╭▸ 
        4 │ reindex index idx;
          ╰╴                ─ hover
        ");
    }

    #[test]
    fn hover_merge_returning_star_from_cte() {
        assert_snapshot!(check_hover("
create table t(a int, b int);
with u(x, y) as (
  select 1, 2
),
merged as (
  merge into t
    using u
      on t.a = u.x
  when matched then
    do nothing
  when not matched then
    do nothing
  returning a as x, b as y
)
select *$0 from merged;
"), @r"
        hover: column merged.x
              column merged.y
           ╭▸ 
        16 │ select * from merged;
           ╰╴       ─ hover
        ");
    }

    #[test]
    fn hover_update_returning_star() {
        assert_snapshot!(check_hover("
create table t(a int, b int);
update t set a = 1
returning *$0;
"), @r"
        hover: column public.t.a int
              column public.t.b int
          ╭▸ 
        4 │ returning *;
          ╰╴          ─ hover
        ");
    }

    #[test]
    fn hover_insert_returning_star() {
        assert_snapshot!(check_hover("
create table t(a int, b int);
insert into t values (1, 2)
returning *$0;
"), @r"
        hover: column public.t.a int
              column public.t.b int
          ╭▸ 
        4 │ returning *;
          ╰╴          ─ hover
        ");
    }

    #[test]
    fn hover_delete_returning_star() {
        assert_snapshot!(check_hover("
create table t(a int, b int);
delete from t
returning *$0;
"), @r"
        hover: column public.t.a int
              column public.t.b int
          ╭▸ 
        4 │ returning *;
          ╰╴          ─ hover
        ");
    }

    #[test]
    fn hover_merge_returning_star() {
        assert_snapshot!(check_hover("
create table t(a int, b int);
merge into t
  using (select 1 as x, 2 as y) u
    on t.a = u.x
  when matched then
    do nothing
returning *$0;
"), @r"
        hover: column public.t.a int
              column public.t.b int
          ╭▸ 
        8 │ returning *;
          ╰╴          ─ hover
        ");
    }

    #[test]
    fn hover_merge_returning_qualified_star_old() {
        assert_snapshot!(check_hover("
create table t(a int, b int);
merge into t
  using (select 1 as x, 2 as y) u
    on t.a = u.x
  when matched then
    update set a = 99
returning old$0.*;
"), @r"
        hover: table public.t(a int, b int)
          ╭▸ 
        8 │ returning old.*;
          ╰╴            ─ hover
        ");
    }

    #[test]
    fn hover_merge_returning_qualified_star_new() {
        assert_snapshot!(check_hover("
create table t(a int, b int);
merge into t
  using (select 1 as x, 2 as y) u
    on t.a = u.x
  when matched then
    update set a = 99
returning new$0.*;
"), @r"
        hover: table public.t(a int, b int)
          ╭▸ 
        8 │ returning new.*;
          ╰╴            ─ hover
        ");
    }

    #[test]
    fn hover_merge_returning_qualified_star_table() {
        assert_snapshot!(check_hover("
create table t(a int, b int);
merge into t
  using (select 1 as x, 2 as y) u
    on t.a = u.x
  when matched then
    update set a = 99
returning t$0.*;
"), @r"
        hover: table public.t(a int, b int)
          ╭▸ 
        8 │ returning t.*;
          ╰╴          ─ hover
        ");
    }

    #[test]
    fn hover_merge_returning_qualified_star_old_on_star() {
        assert_snapshot!(check_hover("
create table t(a int, b int);
merge into t
  using (select 1 as x, 2 as y) u
    on t.a = u.x
  when matched then
    update set a = 99
returning old.*$0;
"), @r"
        hover: column public.t.a int
              column public.t.b int
          ╭▸ 
        8 │ returning old.*;
          ╰╴              ─ hover
        ");
    }

    #[test]
    fn hover_merge_returning_qualified_star_new_on_star() {
        assert_snapshot!(check_hover("
create table t(a int, b int);
merge into t
  using (select 1 as x, 2 as y) u
    on t.a = u.x
  when matched then
    update set a = 99
returning new.*$0;
"), @r"
        hover: column public.t.a int
              column public.t.b int
          ╭▸ 
        8 │ returning new.*;
          ╰╴              ─ hover
        ");
    }

    #[test]
    fn hover_merge_returning_qualified_star_table_on_star() {
        assert_snapshot!(check_hover("
create table t(a int, b int);
merge into t
  using (select 1 as x, 2 as y) u
    on t.a = u.x
  when matched then
    update set a = 99
returning t.*$0;
"), @r"
        hover: column public.t.a int
              column public.t.b int
          ╭▸ 
        8 │ returning t.*;
          ╰╴            ─ hover
        ");
    }

    #[test]
    fn hover_partition_table_column() {
        assert_snapshot!(check_hover("
create table part (
  a int,
  inserted_at timestamptz not null default now()
) partition by range (inserted_at);
create table part_2026_01_02 partition of part
    for values from ('2026-01-02') to ('2026-01-03');
select a$0 from part_2026_01_02;
"), @r"
        hover: column public.part.a int
          ╭▸ 
        8 │ select a from part_2026_01_02;
          ╰╴       ─ hover
        ");
    }

    #[test]
    fn hover_create_table_like_multi_star() {
        assert_snapshot!(check_hover("
create table t(a int, b int);
create table u(x int, y int);
create table k(like t, like u, c int);
select *$0 from k;
"), @r"
        hover: column public.k.a int
              column public.k.b int
              column public.k.x int
              column public.k.y int
              column public.k.c int
          ╭▸ 
        5 │ select * from k;
          ╰╴       ─ hover
        ");
    }

    #[test]
    fn hover_create_table_inherits_star() {
        assert_snapshot!(check_hover("
create table t (
  a int, b text
);
create table u (
  c int
) inherits (t);
select *$0 from u;
"), @r"
        hover: column public.u.a int
              column public.u.b text
              column public.u.c int
          ╭▸ 
        8 │ select * from u;
          ╰╴       ─ hover
        ");
    }

    #[test]
    fn hover_create_table_inherits_column() {
        assert_snapshot!(check_hover("
create table t (
  a int, b text
);
create table u (
  c int
) inherits (t);
select a$0 from u;
"), @r"
        hover: column public.t.a int
          ╭▸ 
        8 │ select a from u;
          ╰╴       ─ hover
        ");
    }

    #[test]
    fn hover_create_table_inherits_local_column() {
        assert_snapshot!(check_hover("
create table t (
  a int, b text
);
create table u (
  c int
) inherits (t);
select c$0 from u;
"), @r"
        hover: column public.u.c int
          ╭▸ 
        8 │ select c from u;
          ╰╴       ─ hover
        ");
    }

    #[test]
    fn hover_create_table_inherits_multiple_parents() {
        assert_snapshot!(check_hover("
create table t1 (
  a int
);
create table t2 (
  b text
);
create table u (
  c int
) inherits (t1, t2);
select b$0 from u;
"), @r"
        hover: column public.t2.b text
           ╭▸ 
        11 │ select b from u;
           ╰╴       ─ hover
        ");
    }

    #[test]
    fn hover_create_foreign_table_inherits_column() {
        assert_snapshot!(check_hover("
create server myserver foreign data wrapper postgres_fdw;
create table t (
  a int, b text
);
create foreign table u (
  c int
) inherits (t) server myserver;
select a$0 from u;
"), @r"
        hover: column public.t.a int
          ╭▸ 
        9 │ select a from u;
          ╰╴       ─ hover
        ");
    }

    #[test]
    fn hover_extension_on_create() {
        assert_snapshot!(check_hover("
create extension my$0ext;
"), @r"
        hover: extension myext
          ╭▸ 
        2 │ create extension myext;
          ╰╴                  ─ hover
        ");
    }

    #[test]
    fn hover_extension_on_drop() {
        assert_snapshot!(check_hover("
create extension myext;
drop extension my$0ext;
"), @r"
        hover: extension myext
          ╭▸ 
        3 │ drop extension myext;
          ╰╴                ─ hover
        ");
    }

    #[test]
    fn hover_extension_on_alter() {
        assert_snapshot!(check_hover("
create extension myext;
alter extension my$0ext update to '2.0';
"), @r"
        hover: extension myext
          ╭▸ 
        3 │ alter extension myext update to '2.0';
          ╰╴                 ─ hover
        ");
    }

    #[test]
    fn hover_role_on_create() {
        assert_snapshot!(check_hover("
create role read$0er;
"), @r"
        hover: role reader
          ╭▸ 
        2 │ create role reader;
          ╰╴               ─ hover
        ");
    }

    #[test]
    fn hover_role_on_alter() {
        assert_snapshot!(check_hover("
create role reader;
alter role read$0er rename to writer;
"), @r"
        hover: role reader
          ╭▸ 
        3 │ alter role reader rename to writer;
          ╰╴              ─ hover
        ");
    }

    #[test]
    fn hover_role_on_drop() {
        assert_snapshot!(check_hover("
create role reader;
drop role read$0er;
"), @r"
        hover: role reader
          ╭▸ 
        3 │ drop role reader;
          ╰╴             ─ hover
        ");
    }

    #[test]
    fn hover_role_on_set() {
        assert_snapshot!(check_hover("
create role reader;
set role read$0er;
"), @r"
        hover: role reader
          ╭▸ 
        3 │ set role reader;
          ╰╴            ─ hover
        ");
    }

    #[test]
    fn hover_role_on_create_tablespace_owner() {
        assert_snapshot!(check_hover("
create role reader;
create tablespace t owner read$0er location 'foo';
"), @r"
        hover: role reader
          ╭▸ 
        3 │ create tablespace t owner reader location 'foo';
          ╰╴                             ─ hover
        ");
    }

    #[test]
    fn hover_on_fetch_cursor() {
        assert_snapshot!(check_hover("
declare c scroll cursor for select * from t;
fetch forward 5 from c$0;
"), @r"
        hover: cursor c for select * from t
          ╭▸ 
        3 │ fetch forward 5 from c;
          ╰╴                     ─ hover
        ");
    }

    #[test]
    fn hover_on_close_cursor() {
        assert_snapshot!(check_hover("
declare c scroll cursor for select * from t;
close c$0;
"), @r"
        hover: cursor c for select * from t
          ╭▸ 
        3 │ close c;
          ╰╴      ─ hover
        ");
    }

    #[test]
    fn hover_on_move_cursor() {
        assert_snapshot!(check_hover("
declare c scroll cursor for select * from t;
move forward 10 from c$0;
"), @r"
        hover: cursor c for select * from t
          ╭▸ 
        3 │ move forward 10 from c;
          ╰╴                     ─ hover
        ");
    }

    #[test]
    fn hover_on_prepare_statement() {
        assert_snapshot!(check_hover("
prepare stmt$0 as select 1;
"), @r"
        hover: prepare stmt as select 1
          ╭▸ 
        2 │ prepare stmt as select 1;
          ╰╴           ─ hover
        ");
    }

    #[test]
    fn hover_on_execute_prepared_statement() {
        assert_snapshot!(check_hover("
prepare stmt as select 1;
execute stmt$0;
"), @r"
        hover: prepare stmt as select 1
          ╭▸ 
        3 │ execute stmt;
          ╰╴           ─ hover
        ");
    }

    #[test]
    fn hover_on_deallocate_prepared_statement() {
        assert_snapshot!(check_hover("
prepare stmt as select 1;
deallocate stmt$0;
"), @r"
        hover: prepare stmt as select 1
          ╭▸ 
        3 │ deallocate stmt;
          ╰╴              ─ hover
        ");
    }

    #[test]
    fn hover_on_listen_definition() {
        assert_snapshot!(check_hover("
listen updates$0;
"), @r"
        hover: listen updates
          ╭▸ 
        2 │ listen updates;
          ╰╴             ─ hover
        ");
    }

    #[test]
    fn hover_on_notify_channel() {
        assert_snapshot!(check_hover("
listen updates;
notify updates$0;
"), @r"
        hover: listen updates
          ╭▸ 
        3 │ notify updates;
          ╰╴             ─ hover
        ");
    }

    #[test]
    fn hover_on_unlisten_channel() {
        assert_snapshot!(check_hover("
listen updates;
unlisten updates$0;
"), @r"
        hover: listen updates
          ╭▸ 
        3 │ unlisten updates;
          ╰╴               ─ hover
        ");
    }
}
