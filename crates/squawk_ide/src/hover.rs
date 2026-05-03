use crate::ast_nav;
use crate::builtins::builtins_file;
use crate::collect;
use crate::column_name::ColumnName;
use crate::comments::preceding_comment;
use crate::db::{File, bind, parse};
use crate::infer::infer_type_from_expr;
use crate::location::{Location, LocationKind};
use crate::name;
use crate::offsets::token_from_offset;
use crate::symbols::{Name, Schema};
use crate::{goto_definition, resolve};
use rowan::TextSize;
use salsa::Database as Db;
use squawk_syntax::SyntaxNode;
use squawk_syntax::SyntaxNodePtr;
use squawk_syntax::{
    SyntaxKind,
    ast::{self, AstNode},
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Hover {
    pub snippet: String,
    pub comment: Option<String>,
}

impl Hover {
    fn snippet(snippet: impl Into<String>) -> Hover {
        Hover {
            snippet: snippet.into(),
            comment: None,
        }
    }

    fn new(snippet: impl Into<String>, comment: impl Into<String>) -> Hover {
        Hover {
            snippet: snippet.into(),
            comment: Some(comment.into()),
        }
    }

    pub fn markdown(&self) -> String {
        let snippet = &self.snippet;
        let mut out = format!(
            "
```sql
{snippet}
```
"
        );

        if let Some(comment) = &self.comment {
            out.push_str(&format!(
                "---
{comment}
"
            ))
        }

        out
    }
}

fn merge_hovers(hovers: Vec<Hover>) -> Option<Hover> {
    if hovers.is_empty() {
        return None;
    }

    if hovers.len() == 1 {
        return Some(hovers[0].clone());
    }

    Some(Hover::snippet(
        hovers
            .into_iter()
            .map(|hover| hover.snippet)
            .collect::<Vec<_>>()
            .join("\n"),
    ))
}

fn hover_with_preceding_comment(snippet: impl Into<String>, node: &SyntaxNode) -> Hover {
    let snippet = snippet.into();
    if let Some(comment) = preceding_comment(node) {
        return Hover::new(snippet, comment);
    }
    Hover::snippet(snippet)
}

fn hover_column_with_preceding_comment(snippet: impl Into<String>, def_node: &SyntaxNode) -> Hover {
    let snippet = snippet.into();
    if let Some(definition_node) = def_node
        .ancestors()
        .find_map(|node| ast::Column::cast(node.clone()))
    {
        return hover_with_preceding_comment(snippet, definition_node.syntax());
    }
    Hover::snippet(snippet)
}

#[salsa::tracked]
pub fn hover(db: &dyn Db, file: File, offset: TextSize) -> Option<Hover> {
    let token = token_from_offset(db, file, offset)?;
    let parent = token.parent()?;

    if token.kind() == SyntaxKind::STAR {
        if let Some(field_expr) = ast::FieldExpr::cast(parent.clone())
            && field_expr.star_token().is_some()
            && let Some(result) = hover_qualified_star(db, file, field_expr)
        {
            return Some(result);
        }

        if let Some(arg_list) = ast::ArgList::cast(parent.clone())
            && let Some(result) = hover_unqualified_star_in_arg_list(db, file, arg_list)
        {
            return Some(result);
        }

        if let Some(target) = ast::Target::cast(parent.clone())
            && target.star_token().is_some()
            && let Some(result) = hover_unqualified_star(db, file, target)
        {
            return Some(result);
        }
        return None;
    }

    if ast::NameRef::can_cast(parent.kind()) {
        return hover_name_ref(db, file, offset);
    }

    if let Some(name) = ast::Name::cast(parent) {
        return hover_name(db, file, name);
    }

    None
}

fn hover_name(db: &dyn Db, file: File, name: ast::Name) -> Option<Hover> {
    let def = Location::from_node(file, name.syntax())?;
    match def.kind {
        LocationKind::Aggregate => hover_aggregate(db, def),
        LocationKind::CaseExpr | LocationKind::CommitBegin | LocationKind::CommitEnd => None,
        LocationKind::Channel => hover_channel(db, def),
        LocationKind::Column => hover_name_column(db, def),
        LocationKind::Cursor => hover_cursor(db, def),
        LocationKind::Database => hover_database(db, def),
        LocationKind::EventTrigger => hover_event_trigger(db, def),
        LocationKind::Extension => hover_extension(db, def),
        LocationKind::Function => hover_function(db, def),
        LocationKind::Index => hover_index(db, def),
        LocationKind::NamedArgParameter => hover_named_arg_parameter(db, def),
        LocationKind::Policy => hover_policy(db, def),
        LocationKind::PreparedStatement => hover_prepared_statement(db, def),
        LocationKind::Procedure => hover_procedure(db, def),
        LocationKind::PropertyGraph => hover_property_graph(db, def),
        LocationKind::Role => hover_role(db, def),
        LocationKind::Schema => hover_schema(db, def),
        LocationKind::Sequence => hover_sequence(db, def),
        LocationKind::Server => hover_server(db, def),
        LocationKind::Table => hover_table(db, def),
        LocationKind::Tablespace => hover_tablespace(db, def),
        LocationKind::View => {
            if let Some(hover) = format_create_view(db, def) {
                return Some(hover);
            }
            hover_table(db, def)
        }
        LocationKind::Trigger => hover_trigger(db, def),
        LocationKind::Type => hover_type(db, def),
        LocationKind::Window => hover_window(db, def),
    }
}

fn hover_name_column(db: &dyn Db, def: Location) -> Option<Hover> {
    if let Some(result) = hover_composite_type_field(db, def) {
        return Some(result);
    }

    let def_node = def.to_node(db)?;
    if let Some(column) = def_node.parent().and_then(ast::Column::cast)
        && let Some(create_table) = def_node.ancestors().find_map(ast::CreateTableLike::cast)
    {
        return hover_column_definition(db, def.file, create_table, column);
    }

    if def_node
        .ancestors()
        .any(|ancestor| ast::ColumnList::can_cast(ancestor.kind()))
        && let Some(create_view) = def_node.ancestors().find_map(ast::CreateViewLike::cast)
    {
        return format_view_column(db, def.file, &create_view, &def_node);
    }

    None
}

fn hover_name_ref(db: &dyn Db, file: File, offset: TextSize) -> Option<Hover> {
    // We can get multiple in the case of using
    //
    // select * from t join u using (id);
    //
    let definitions = goto_definition::goto_definition(db, file, offset);
    let def = *definitions.first()?;
    match def.kind {
        LocationKind::Aggregate => hover_aggregate(db, def),
        LocationKind::CaseExpr | LocationKind::CommitBegin | LocationKind::CommitEnd => None,
        LocationKind::Channel => hover_channel(db, def),
        LocationKind::Column => {
            if let Some(result) = hover_composite_type_field(db, def) {
                return Some(result);
            }
            if let Some(result) = hover_column(db, &definitions) {
                return Some(result);
            }
            // If no column, try as function (handles field-style function calls like `t.b`)
            if let Some(result) = hover_function(db, def) {
                return Some(result);
            }
            // Finally try as table (handles case like `select t from t;` where t is the table)
            hover_table(db, def)
        }
        LocationKind::Cursor => hover_cursor(db, def),
        LocationKind::Database => hover_database(db, def),
        LocationKind::EventTrigger => hover_event_trigger(db, def),
        LocationKind::Extension => hover_extension(db, def),
        LocationKind::Function => {
            if let Some(result) = hover_function(db, def) {
                return Some(result);
            }
            if let Some(result) = hover_routine(db, def) {
                return Some(result);
            }
            hover_column(db, &definitions)
        }
        LocationKind::Index => hover_index(db, def),
        LocationKind::NamedArgParameter => hover_named_arg_parameter(db, def),
        LocationKind::Policy => hover_policy(db, def),
        LocationKind::PreparedStatement => hover_prepared_statement(db, def),
        LocationKind::Procedure => hover_procedure(db, def),
        LocationKind::PropertyGraph => hover_property_graph(db, def),
        LocationKind::Role => hover_role(db, def),
        LocationKind::Schema => hover_schema(db, def),
        LocationKind::Sequence => hover_sequence(db, def),
        LocationKind::Server => hover_server(db, def),
        LocationKind::Table | LocationKind::View => hover_table(db, def),
        LocationKind::Tablespace => hover_tablespace(db, def),
        LocationKind::Trigger => hover_trigger(db, def),
        LocationKind::Type => hover_type(db, def),
        LocationKind::Window => hover_window(db, def),
    }
}

struct ColumnHover;
impl ColumnHover {
    fn table_column(table_name: &str, column_name: &str) -> String {
        format!("column {table_name}.{column_name}")
    }

    fn table_column_type(table_name: &str, column_name: &str, ty: &str) -> String {
        format!("column {table_name}.{column_name} {ty}")
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

fn hover_column(db: &dyn Db, definitions: &[Location]) -> Option<Hover> {
    let results: Vec<Hover> = definitions
        .iter()
        .filter_map(|def| format_hover_for_column_ptr(db, *def))
        .collect();

    merge_hovers(results)
}

fn format_hover_for_column_ptr(db: &dyn Db, def: Location) -> Option<Hover> {
    let def_node = &def.to_node(db)?;
    match ast_nav::parent_source(def_node)? {
        ast_nav::ParentSouce::WithTable(with_table) => {
            let cte_name = with_table.name()?;
            let column_name = collect::column_name_from_node(def_node)?;
            let table_name = Name::from_node(&cte_name);
            let ty = collect::with_table_columns_with_types(db, def.file, with_table)
                .into_iter()
                .find(|(name, _)| *name == column_name)
                .and_then(|(_, ty)| ty);
            return Some(hover_column_with_preceding_comment(
                match ty {
                    Some(ty) => ColumnHover::table_column_type(
                        &table_name.to_string(),
                        &column_name.to_string(),
                        &ty.to_string(),
                    ),
                    None => {
                        ColumnHover::table_column(&table_name.to_string(), &column_name.to_string())
                    }
                },
                def_node,
            ));
        }
        ast_nav::ParentSouce::ParenSelect(paren_select) => {
            // Qualified access like `t.a`
            let table_name = subquery_alias_name(&paren_select);

            // Unqualified access like `a` from `select a from (select 1 a)`
            let column_name = collect::column_name_from_node(def_node)?;

            let ty = collect::paren_select_columns_with_types(db, def.file, &paren_select)
                .into_iter()
                .find(|(name, _)| *name == column_name)
                .and_then(|(_, ty)| ty)?;
            if let Some(table_name) = table_name {
                Some(hover_column_with_preceding_comment(
                    ColumnHover::table_column_type(
                        &table_name.to_string(),
                        &column_name.to_string(),
                        &ty.to_string(),
                    ),
                    def_node,
                ))
            } else {
                Some(hover_column_with_preceding_comment(
                    ColumnHover::anon_column_type(&column_name.to_string(), &ty.to_string()),
                    def_node,
                ))
            }
        }
        // create view v(a) as select 1;
        // select a from v;
        //        ^
        ast_nav::ParentSouce::CreateView(create_view) => {
            let column_name = collect::column_name_from_node(def_node)?;
            let path = create_view.path()?;
            let (schema, view_name) = resolve::resolve_view_info(db, def.file, &path)?;
            let ty = collect::view_like_columns_with_types(db, def.file, &create_view)
                .into_iter()
                .find(|(name, _)| *name == column_name)
                .and_then(|(_, ty)| ty);
            return Some(hover_column_with_preceding_comment(
                match ty {
                    Some(ty) => ColumnHover::schema_table_column_type(
                        &schema.to_string(),
                        &view_name,
                        &column_name.to_string(),
                        &ty.to_string(),
                    ),
                    None => ColumnHover::schema_table_column(
                        &schema.to_string(),
                        &view_name,
                        &column_name.to_string(),
                    ),
                },
                def_node,
            ));
        }
        ast_nav::ParentSouce::Alias(alias) => {
            let alias_name = alias.name()?;
            alias.column_list()?;
            let from_item = alias.syntax().ancestors().find_map(ast::FromItem::cast)?;
            let table_name = Name::from_node(&alias_name);
            let column_name = Name::from_string(def_node.text().to_string());
            let ty = collect::columns_for_star_from_alias(db, def.file, &from_item, &alias)
                .into_iter()
                .find(|(name, _)| *name == column_name)
                .and_then(|(_, ty)| ty);
            return Some(hover_column_with_preceding_comment(
                match ty {
                    Some(ty) => ColumnHover::table_column_type(
                        &table_name.to_string(),
                        &column_name.to_string(),
                        &ty.to_string(),
                    ),
                    None => {
                        ColumnHover::table_column(&table_name.to_string(), &column_name.to_string())
                    }
                },
                def_node,
            ));
        }
        ast_nav::ParentSouce::CreateTableAs(create_table_as) => {
            let column_name = collect::column_name_from_node(def_node)?;
            let path = create_table_as.path()?;
            let (schema, table_name) = resolve::resolve_table_info(db, def.file, &path)?;
            let ty = collect::create_table_as_columns_with_types(db, def.file, &create_table_as)
                .into_iter()
                .find(|(name, _)| *name == column_name)
                .and_then(|(_, ty)| ty);
            return Some(hover_column_with_preceding_comment(
                match ty {
                    Some(ty) => ColumnHover::schema_table_column_type(
                        &schema.to_string(),
                        &table_name,
                        &column_name.to_string(),
                        &ty.to_string(),
                    ),
                    None => ColumnHover::schema_table_column(
                        &schema.to_string(),
                        &table_name,
                        &column_name.to_string(),
                    ),
                },
                def_node,
            ));
        }
        ast_nav::ParentSouce::CreateTable(create_table) => {
            let column = def_node.ancestors().find_map(ast::Column::cast)?;
            let column_name = column.name()?;
            let ty = column.ty()?;
            let path = create_table.path()?;
            let (schema, table_name) = resolve::resolve_table_info(db, def.file, &path)?;

            return Some(hover_column_with_preceding_comment(
                ColumnHover::schema_table_column_type(
                    &schema.to_string(),
                    &table_name,
                    &Name::from_node(&column_name).to_string(),
                    &ty.syntax().text().to_string(),
                ),
                def_node,
            ));
        }
    }
}

fn hover_composite_type_field(db: &dyn Db, def: Location) -> Option<Hover> {
    let column = def.to_node(db)?.ancestors().find_map(ast::Column::cast)?;
    let field_name = column.name()?.syntax().text().to_string();
    let ty = column.ty()?;

    let create_type = column
        .syntax()
        .ancestors()
        .find_map(ast::CreateType::cast)?;
    let type_path = create_type.path()?;
    let (schema, type_name) = resolve::resolve_type_info(db, def.file, &type_path)?;

    Some(hover_with_preceding_comment(
        format!(
            "field {}.{}.{} {}",
            schema,
            type_name,
            field_name,
            ty.syntax().text()
        ),
        column.syntax(),
    ))
}

fn hover_column_definition(
    db: &dyn Db,
    file: File,
    create_table: impl ast::HasCreateTable,
    column: ast::Column,
) -> Option<Hover> {
    let column_name = column.name()?.syntax().text().to_string();
    let ty = column.ty()?;
    let path = create_table.path()?;
    let (schema, table_name) = resolve::resolve_table_info(db, file, &path)?;
    let ty = ty.syntax().text().to_string();
    Some(hover_with_preceding_comment(
        ColumnHover::schema_table_column_type(&schema.to_string(), &table_name, &column_name, &ty),
        column.syntax(),
    ))
}

fn format_table_source(db: &dyn Db, file: File, source: ast_nav::ParentSouce) -> Option<Hover> {
    match source {
        ast_nav::ParentSouce::Alias(alias) => format_alias_with_column_list(db, file, alias),
        ast_nav::ParentSouce::WithTable(with_table) => format_with_table(with_table),
        ast_nav::ParentSouce::CreateView(create_view) => {
            format_create_view_like(db, file, create_view)
        }
        ast_nav::ParentSouce::CreateTable(create_table) => {
            format_create_table(db, file, create_table)
        }
        ast_nav::ParentSouce::CreateTableAs(create_table_as) => {
            format_create_table_as(db, file, create_table_as)
        }
        ast_nav::ParentSouce::ParenSelect(paren_select) => format_paren_select(paren_select),
    }
}

fn hover_table(db: &dyn Db, def: Location) -> Option<Hover> {
    let source = ast_nav::parent_source(&def.to_node(db)?)?;
    format_table_source(db, def.file, source)
}

fn format_alias_with_column_list(db: &dyn Db, file: File, alias: ast::Alias) -> Option<Hover> {
    let alias_name = alias.name()?;
    let name = Name::from_node(&alias_name);

    let Some(column_list) = alias.column_list() else {
        let name = Name::from_node(&alias.name()?);
        let from_item = alias.syntax().ancestors().find_map(ast::FromItem::cast)?;
        let paren_select = from_item.paren_select()?;
        return format_subquery_table(name, paren_select);
    };

    let mut columns: Vec<Name> = column_list
        .columns()
        .filter_map(|column| {
            column
                .name()
                .map(|column_name| Name::from_node(&column_name))
        })
        .collect();

    if let Some(from_item) = alias.syntax().ancestors().find_map(ast::FromItem::cast)
        && let Some(table_ptr) = resolve::table_ptr_from_from_item(db, file, &from_item)
    {
        let base_columns = collect::star_column_names(db, file, &table_ptr);
        for column in base_columns.iter().skip(columns.len()) {
            columns.push(column.clone());
        }
    }

    let columns = columns
        .iter()
        .map(|column| column.to_string())
        .collect::<Vec<_>>()
        .join(", ");
    Some(Hover::snippet(format!("table {}({})", name, columns)))
}

fn hover_qualified_star(db: &dyn Db, file: File, field_expr: ast::FieldExpr) -> Option<Hover> {
    let table_ptr = qualified_star_table_ptr(db, file, field_expr)?;
    hover_qualified_star_columns(db, file, &table_ptr)
}

fn hover_unqualified_star(db: &dyn Db, file: File, target: ast::Target) -> Option<Hover> {
    let mut results = vec![];
    for file in [file, builtins_file(db)] {
        results = hover_unqualified_star_with_binder(db, file, &target);
        if results.is_empty() && target_has_schema_qualified_from_item(&target) {
            continue;
        } else {
            break;
        }
    }
    merge_hovers(results)
}

fn hover_unqualified_star_with_binder(db: &dyn Db, file: File, target: &ast::Target) -> Vec<Hover> {
    let mut results = vec![];

    if let Some(table_ptrs) = unqualified_star_table_ptrs(db, file, target) {
        for table_ptr in table_ptrs {
            if let Some(columns) = hover_qualified_star_columns(db, file, &table_ptr) {
                results.push(columns);
            }
        }
    }

    results
}

fn target_has_schema_qualified_from_item(target: &ast::Target) -> bool {
    let Some(select) = target.syntax().ancestors().find_map(ast::Select::cast) else {
        return false;
    };
    let Some(from_clause) = select.from_clause() else {
        return false;
    };

    for from_item in from_clause.from_items() {
        if from_item.field_expr().is_some() {
            return true;
        }
    }

    false
}

fn hover_unqualified_star_in_arg_list(
    db: &dyn Db,
    file: File,
    arg_list: ast::ArgList,
) -> Option<Hover> {
    let table_ptrs = unqualified_star_in_arg_list_ptrs(db, file, &arg_list)?;
    let mut results = vec![];
    for table_ptr in table_ptrs {
        if let Some(columns) = hover_qualified_star_columns(db, file, &table_ptr) {
            results.push(columns);
        }
    }

    merge_hovers(results)
}

fn format_subquery_table(name: Name, paren_select: ast::ParenSelect) -> Option<Hover> {
    let name = name.to_string();
    let query = paren_select.syntax().text().to_string();
    Some(Hover::snippet(format!("subquery {} as {}", name, query)))
}

fn hover_qualified_star_columns(
    db: &dyn Db,
    file: File,
    table_ptr: &squawk_syntax::SyntaxNodePtr,
) -> Option<Hover> {
    let source_file = parse(db, file).tree();
    let root = source_file.syntax();
    let table_name_node = table_ptr.to_node(root);

    match ast_nav::parent_source(&table_name_node)? {
        ast_nav::ParentSouce::Alias(alias) => {
            hover_qualified_star_columns_from_alias(db, file, &alias)
        }
        ast_nav::ParentSouce::WithTable(with_table) => {
            hover_qualified_star_columns_from_cte(db, file, with_table)
        }
        ast_nav::ParentSouce::CreateTable(create_table) => {
            hover_qualified_star_columns_from_table(db, file, create_table)
        }
        ast_nav::ParentSouce::CreateTableAs(create_table_as) => {
            hover_qualified_star_columns_from_table_as(db, file, &create_table_as)
        }
        ast_nav::ParentSouce::CreateView(create_view) => {
            hover_qualified_star_columns_from_view_like(db, file, &create_view)
        }
        ast_nav::ParentSouce::ParenSelect(paren_select) => {
            hover_qualified_star_columns_from_subquery(db, file, &paren_select)
        }
    }
}

fn hover_qualified_star_columns_from_alias(
    db: &dyn Db,
    file: File,
    alias: &ast::Alias,
) -> Option<Hover> {
    let alias_name = Name::from_node(&alias.name()?);
    alias.column_list()?;
    let from_item = alias.syntax().ancestors().find_map(ast::FromItem::cast)?;
    let columns = collect::columns_for_star_from_alias(db, file, &from_item, alias);

    if columns.is_empty() {
        return None;
    }

    let results: Vec<Hover> = columns
        .into_iter()
        .map(|(column_name, ty)| {
            Hover::snippet(match ty {
                Some(ty) => ColumnHover::table_column_type(
                    &alias_name.to_string(),
                    &column_name.to_string(),
                    &ty.to_string(),
                ),
                None => {
                    ColumnHover::table_column(&alias_name.to_string(), &column_name.to_string())
                }
            })
        })
        .collect();

    merge_hovers(results)
}

fn hover_qualified_star_columns_from_table(
    db: &dyn Db,
    file: File,
    create_table: impl ast::HasCreateTable,
) -> Option<Hover> {
    let path = create_table.path()?;
    let (schema, table_name) = resolve::resolve_table_info(db, file, &path)?;
    let schema = schema.to_string();
    let results: Vec<Hover> = collect::table_columns(db, file, &create_table)
        .into_iter()
        .filter_map(|(column_name, ty)| {
            let ty = ty?;
            Some(Hover::snippet(ColumnHover::schema_table_column_type(
                &schema,
                &table_name,
                &column_name.to_string(),
                &ty.to_string(),
            )))
        })
        .collect();

    merge_hovers(results)
}

fn hover_qualified_star_columns_from_table_as(
    db: &dyn Db,
    file: File,
    create_table_as: &ast::CreateTableAs,
) -> Option<Hover> {
    let path = create_table_as.path()?;
    let (schema, table_name) = resolve::resolve_table_info(db, file, &path)?;
    let schema_str = schema.to_string();

    let columns = collect::create_table_as_columns_with_types(db, file, create_table_as);
    let results: Vec<Hover> = columns
        .into_iter()
        .map(|(column_name, ty)| {
            if let Some(ty) = ty {
                return Hover::snippet(ColumnHover::schema_table_column_type(
                    &schema_str,
                    &table_name,
                    &column_name.to_string(),
                    &ty.to_string(),
                ));
            }
            Hover::snippet(ColumnHover::schema_table_column(
                &schema_str,
                &table_name,
                &column_name.to_string(),
            ))
        })
        .collect();

    merge_hovers(results)
}

fn hover_qualified_star_columns_from_cte(
    db: &dyn Db,
    file: File,
    with_table: ast::WithTable,
) -> Option<Hover> {
    let cte_name = Name::from_node(&with_table.name()?);
    let cte_name = cte_name.to_string();
    let columns = collect::with_table_columns_with_types(db, file, with_table);
    let results: Vec<Hover> = columns
        .into_iter()
        .map(|(column_name, ty)| {
            let column_name = column_name.to_string();
            if let Some(ty) = ty {
                return Hover::snippet(ColumnHover::table_column_type(
                    &cte_name,
                    &column_name,
                    &ty.to_string(),
                ));
            }

            Hover::snippet(ColumnHover::table_column(&cte_name, &column_name))
        })
        .collect();

    merge_hovers(results)
}

fn hover_qualified_star_columns_from_view_like(
    db: &dyn Db,
    file: File,
    create_view: &ast::CreateViewLike,
) -> Option<Hover> {
    let path = create_view.path()?;
    let (schema, view_name) = resolve::resolve_view_info(db, file, &path)?;

    let schema_str = schema.to_string();
    let columns = collect::view_like_columns_with_types(db, file, create_view);
    let results: Vec<Hover> = columns
        .into_iter()
        .map(|(column_name, ty)| {
            if let Some(ty) = ty {
                return Hover::snippet(ColumnHover::schema_table_column_type(
                    &schema_str,
                    &view_name,
                    &column_name.to_string(),
                    &ty.to_string(),
                ));
            }

            Hover::snippet(ColumnHover::schema_table_column(
                &schema_str,
                &view_name,
                &column_name.to_string(),
            ))
        })
        .collect();

    merge_hovers(results)
}

fn hover_qualified_star_columns_from_subquery(
    db: &dyn Db,
    file: File,
    paren_select: &ast::ParenSelect,
) -> Option<Hover> {
    let select_variant = paren_select.select()?;

    if let Some(select) = ast_nav::select_from_variant(select_variant) {
        let target_list = select.select_clause()?.target_list()?;

        let mut results = vec![];
        let subquery_alias = subquery_alias_name(paren_select);

        for target in target_list.targets() {
            if target.star_token().is_some() {
                let table_ptrs = unqualified_star_table_ptrs(db, file, &target)?;
                for table_ptr in table_ptrs {
                    if let Some(columns) = hover_qualified_star_columns(db, file, &table_ptr) {
                        results.push(columns)
                    }
                }
                continue;
            }

            if let Some(result) =
                hover_subquery_target_column(db, file, &target, subquery_alias.as_ref())
            {
                results.push(result);
            }
        }

        return merge_hovers(results);
    }

    let subquery_alias = subquery_alias_name(paren_select);
    let results: Vec<Hover> = collect::paren_select_columns_with_types(db, file, paren_select)
        .into_iter()
        .map(|(column_name, ty)| {
            if let Some(alias) = &subquery_alias {
                return Hover::snippet(ColumnHover::table_column(
                    &alias.to_string(),
                    &column_name.to_string(),
                ));
            }
            if let Some(ty) = ty {
                return Hover::snippet(ColumnHover::anon_column_type(
                    &column_name.to_string(),
                    &ty.to_string(),
                ));
            }
            Hover::snippet(ColumnHover::anon_column(&column_name.to_string()))
        })
        .collect();

    merge_hovers(results)
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
    db: &dyn Db,
    file: File,
    target: &ast::Target,
    subquery_alias: Option<&Name>,
) -> Option<Hover> {
    if let Some(alias) = subquery_alias
        && let Some((col_name, _node)) = ColumnName::from_target(target.clone())
        && let Some(col_name) = col_name.to_string()
    {
        let ty = target.expr().and_then(|e| infer_type_from_expr(&e));
        return Some(Hover::snippet(match ty {
            Some(ty) => {
                ColumnHover::table_column_type(&alias.to_string(), &col_name, &ty.to_string())
            }
            None => ColumnHover::table_column(&alias.to_string(), &col_name),
        }));
    }

    let result = match target.expr()? {
        ast::Expr::NameRef(name_ref) => hover(db, file, name_ref.syntax().text_range().start()),
        ast::Expr::FieldExpr(field_expr) => {
            let field = field_expr.field()?;
            hover(db, file, field.syntax().text_range().start())
        }
        _ => None,
    };

    if result.is_some() {
        return result;
    }

    if let Some((col_name, _node)) = ColumnName::from_target(target.clone())
        && let Some(col_name) = col_name.to_string()
    {
        let ty = target.expr().and_then(|e| infer_type_from_expr(&e));
        return Some(Hover::snippet(match ty {
            Some(ty) => ColumnHover::anon_column_type(&col_name, &ty.to_string()),
            None => ColumnHover::anon_column(&col_name),
        }));
    }

    None
}

fn hover_index(db: &dyn Db, def: Location) -> Option<Hover> {
    let create_index = def
        .to_node(db)?
        .ancestors()
        .find_map(ast::CreateIndex::cast)?;
    format_create_index(db, def.file, create_index)
}

fn hover_sequence(db: &dyn Db, def: Location) -> Option<Hover> {
    let create_sequence = def
        .to_node(db)?
        .ancestors()
        .find_map(ast::CreateSequence::cast)?;
    format_create_sequence(db, def.file, create_sequence)
}

fn hover_trigger(db: &dyn Db, def: Location) -> Option<Hover> {
    let create_trigger = def
        .to_node(db)?
        .ancestors()
        .find_map(ast::CreateTrigger::cast)?;
    format_create_trigger(db, def.file, create_trigger)
}

fn hover_policy(db: &dyn Db, def: Location) -> Option<Hover> {
    let create_policy = def
        .to_node(db)?
        .ancestors()
        .find_map(ast::CreatePolicy::cast)?;
    format_create_policy(db, def.file, create_policy)
}

fn hover_property_graph(db: &dyn Db, def: Location) -> Option<Hover> {
    let create_property_graph = def
        .to_node(db)?
        .ancestors()
        .find_map(ast::CreatePropertyGraph::cast)?;
    format_create_property_graph(db, def.file, create_property_graph)
}

fn hover_event_trigger(db: &dyn Db, def: Location) -> Option<Hover> {
    let create_event_trigger = def
        .to_node(db)?
        .ancestors()
        .find_map(ast::CreateEventTrigger::cast)?;

    format_create_event_trigger(create_event_trigger)
}

fn hover_tablespace(db: &dyn Db, def: Location) -> Option<Hover> {
    let def_node = def.to_node(db)?;
    if let Some(create_tablespace) = def_node.ancestors().find_map(ast::CreateTablespace::cast) {
        return format_create_tablespace(create_tablespace);
    }
    Some(Hover::snippet(format!("tablespace {}", def_node.text())))
}

fn hover_database(db: &dyn Db, def: Location) -> Option<Hover> {
    let def_node = def.to_node(db)?;
    if let Some(create_database) = def_node.ancestors().find_map(ast::CreateDatabase::cast) {
        return format_create_database(create_database);
    }
    Some(Hover::snippet(format!("database {}", def_node.text())))
}

fn hover_server(db: &dyn Db, def: Location) -> Option<Hover> {
    let def_node = def.to_node(db)?;
    if let Some(create_server) = def_node.ancestors().find_map(ast::CreateServer::cast) {
        return format_create_server(create_server);
    }
    Some(Hover::snippet(format!("server {}", def_node.text())))
}

fn hover_extension(db: &dyn Db, def: Location) -> Option<Hover> {
    let def_node = def.to_node(db)?;
    if let Some(create_extension) = def_node.ancestors().find_map(ast::CreateExtension::cast) {
        return format_create_extension(create_extension);
    }
    Some(Hover::snippet(format!("extension {}", def_node.text())))
}

fn hover_role(db: &dyn Db, def: Location) -> Option<Hover> {
    let def_node = def.to_node(db)?;
    if let Some(create_role) = def_node.ancestors().find_map(ast::CreateRole::cast) {
        return format_create_role(create_role);
    }
    Some(Hover::snippet(format!("role {}", def_node.text())))
}

fn hover_cursor(db: &dyn Db, def: Location) -> Option<Hover> {
    let declare = def.to_node(db)?.ancestors().find_map(ast::Declare::cast)?;
    format_declare_cursor(declare)
}

fn hover_prepared_statement(db: &dyn Db, def: Location) -> Option<Hover> {
    let prepare = def.to_node(db)?.ancestors().find_map(ast::Prepare::cast)?;
    format_prepare(prepare)
}

fn hover_channel(db: &dyn Db, def: Location) -> Option<Hover> {
    let listen = def.to_node(db)?.ancestors().find_map(ast::Listen::cast)?;
    format_listen(listen)
}

fn hover_window(db: &dyn Db, def: Location) -> Option<Hover> {
    let window_def = def
        .to_node(db)?
        .ancestors()
        .find_map(ast::WindowDef::cast)?;

    Some(Hover::snippet(format!(
        "window {}",
        window_def.syntax().text()
    )))
}

fn hover_type(db: &dyn Db, def: Location) -> Option<Hover> {
    let create_type = def
        .to_node(db)?
        .ancestors()
        .find_map(ast::CreateType::cast)?;
    format_create_type(db, def.file, create_type)
}

fn format_declare_cursor(declare: ast::Declare) -> Option<Hover> {
    let name = declare.name()?;
    let query = declare.query()?;
    Some(Hover::snippet(format!(
        "cursor {} for {}",
        name.syntax().text(),
        query.syntax().text()
    )))
}

fn format_prepare(prepare: ast::Prepare) -> Option<Hover> {
    let name = prepare.name()?;
    let stmt = prepare.preparable_stmt()?;
    Some(Hover::snippet(format!(
        "prepare {} as {}",
        name.syntax().text(),
        stmt.syntax().text()
    )))
}

fn format_listen(listen: ast::Listen) -> Option<Hover> {
    let name = listen.name()?;
    Some(Hover::snippet(format!("listen {}", name.syntax().text())))
}

fn format_create_table(
    db: &dyn Db,
    file: File,
    create_table: impl ast::HasCreateTable,
) -> Option<Hover> {
    let path = create_table.path()?;
    let (schema, table_name) = resolve::resolve_table_info(db, file, &path)?;
    let schema = schema.to_string();
    let args = create_table.table_arg_list()?.syntax().text().to_string();

    let foreign = if create_table.syntax().kind() == SyntaxKind::CREATE_FOREIGN_TABLE {
        "foreign "
    } else {
        ""
    };

    Some(Hover::snippet(format!(
        "{foreign}table {schema}.{table_name}{args}"
    )))
}

fn format_create_table_as(
    db: &dyn Db,
    file: File,
    create_table_as: ast::CreateTableAs,
) -> Option<Hover> {
    let path = create_table_as.path()?;
    let (schema, table_name) = resolve::resolve_table_info(db, file, &path)?;
    let query = create_table_as.query()?.syntax().text().to_string();
    Some(Hover::snippet(format!(
        "table {}.{} as {}",
        schema, table_name, query
    )))
}

fn format_create_view(db: &dyn Db, def: Location) -> Option<Hover> {
    let create_view = ast::CreateViewLike::cast(def.to_node(db)?)?;
    format_create_view_like(db, def.file, create_view)
}

fn format_create_view_like(
    db: &dyn Db,
    file: File,
    create_view: ast::CreateViewLike,
) -> Option<Hover> {
    let path = create_view.path()?;
    // TODO: we use this to infer the schema, we should either rename this or
    // create a different function
    let (schema, view_name) = resolve::resolve_view_info(db, file, &path)?;
    let schema = schema.to_string();

    let column_list = create_view
        .column_list()
        .map(|cl| cl.syntax().text().to_string())
        .unwrap_or_default();

    let query = create_view.query()?.syntax().text().to_string();

    let view_kind = if create_view.syntax().kind() == SyntaxKind::CREATE_MATERIALIZED_VIEW {
        "materialized view"
    } else {
        "view"
    };

    Some(Hover::snippet(format!(
        "{view_kind} {}.{}{} as {}",
        schema, view_name, column_list, query
    )))
}

fn format_view_column(
    db: &dyn Db,
    file: File,
    create_view: &ast::CreateViewLike,
    def_node: &SyntaxNode,
) -> Option<Hover> {
    let path = create_view.path()?;
    let (schema, view_name) = resolve::resolve_view_info(db, file, &path)?;
    let column_name = Name::from_string(def_node.to_string());
    let ty = collect::view_like_columns_with_types(db, file, create_view)
        .into_iter()
        .find(|(name, _)| *name == column_name)
        .and_then(|(_, ty)| ty);
    Some(hover_column_with_preceding_comment(
        match ty {
            Some(ty) => ColumnHover::schema_table_column_type(
                &schema.to_string(),
                &view_name,
                &column_name.to_string(),
                &ty.to_string(),
            ),
            None => ColumnHover::schema_table_column(
                &schema.to_string(),
                &view_name,
                &column_name.to_string(),
            ),
        },
        def_node,
    ))
}

fn format_with_table(with_table: ast::WithTable) -> Option<Hover> {
    let name = with_table.name()?.syntax().text().to_string();
    let query = with_table.query()?.syntax().text().to_string();
    Some(Hover::snippet(format!("with {} as ({})", name, query)))
}

fn format_paren_select(paren_select: ast::ParenSelect) -> Option<Hover> {
    let query = paren_select.select()?.syntax().text().to_string();
    Some(Hover::snippet(format!("({})", query)))
}

fn format_create_index(db: &dyn Db, file: File, create_index: ast::CreateIndex) -> Option<Hover> {
    let index_name = create_index.name()?.syntax().text().to_string();

    let index_schema = index_schema(db, file, create_index.clone())?;

    let path = create_index.relation_name()?.path()?;
    let (table_schema, table_name) = resolve::resolve_table_info(db, file, &path)?;

    let partition_item_list = create_index.partition_item_list()?;
    let columns = partition_item_list.syntax().text().to_string();

    Some(Hover::snippet(format!(
        "index {}.{} on {}.{}{}",
        index_schema, index_name, table_schema, table_name, columns
    )))
}

fn format_create_sequence(
    db: &dyn Db,
    file: File,
    create_sequence: ast::CreateSequence,
) -> Option<Hover> {
    let path = create_sequence.path()?;
    let (schema, sequence_name) = resolve::resolve_sequence_info(db, file, &path)?;

    Some(Hover::snippet(format!(
        "sequence {}.{}",
        schema, sequence_name
    )))
}

fn format_create_trigger(
    db: &dyn Db,
    file: File,
    create_trigger: ast::CreateTrigger,
) -> Option<Hover> {
    let trigger_name = create_trigger.name()?.syntax().text().to_string();
    let on_table_path = create_trigger.on_table()?.path()?;

    let (schema, table_name) = resolve::resolve_table_info(db, file, &on_table_path)?;
    Some(Hover::snippet(format!(
        "trigger {}.{} on {}.{}",
        schema, trigger_name, schema, table_name
    )))
}

fn format_create_policy(
    db: &dyn Db,
    file: File,
    create_policy: ast::CreatePolicy,
) -> Option<Hover> {
    let policy_name = create_policy.name()?.syntax().text().to_string();
    let on_table_path = create_policy.on_table()?.path()?;

    let (schema, table_name) = resolve::resolve_table_info(db, file, &on_table_path)?;
    Some(Hover::snippet(format!(
        "policy {}.{} on {}.{}",
        schema, policy_name, schema, table_name
    )))
}

fn format_create_property_graph(
    db: &dyn Db,
    file: File,
    create_property_graph: ast::CreatePropertyGraph,
) -> Option<Hover> {
    let path = create_property_graph.path()?;
    let (schema, name) = resolve::resolve_property_graph_info(db, file, &path)?;
    Some(Hover::snippet(format!(
        "property graph {}.{}",
        schema, name
    )))
}

fn format_create_event_trigger(create_event_trigger: ast::CreateEventTrigger) -> Option<Hover> {
    let name = create_event_trigger.name()?.syntax().text().to_string();
    Some(Hover::snippet(format!("event trigger {}", name)))
}

fn format_create_tablespace(create_tablespace: ast::CreateTablespace) -> Option<Hover> {
    let name = create_tablespace.name()?.syntax().text().to_string();
    Some(Hover::snippet(format!("tablespace {}", name)))
}

fn format_create_database(create_database: ast::CreateDatabase) -> Option<Hover> {
    let name = create_database.name()?.syntax().text().to_string();
    Some(Hover::snippet(format!("database {}", name)))
}

fn format_create_server(create_server: ast::CreateServer) -> Option<Hover> {
    let name = create_server.name()?.syntax().text().to_string();
    Some(Hover::snippet(format!("server {}", name)))
}

fn format_create_extension(create_extension: ast::CreateExtension) -> Option<Hover> {
    let name = create_extension.name()?.syntax().text().to_string();
    Some(Hover::snippet(format!("extension {}", name)))
}

fn format_create_role(create_role: ast::CreateRole) -> Option<Hover> {
    let name = create_role.name()?.syntax().text().to_string();
    Some(Hover::snippet(format!("role {}", name)))
}

fn index_schema(db: &dyn Db, file: File, create_index: ast::CreateIndex) -> Option<String> {
    let position = create_index.syntax().text_range().start();
    bind(db, file)
        .search_path_at(position)
        .first()
        .map(|s| s.to_string())
}

fn format_create_type(db: &dyn Db, file: File, create_type: ast::CreateType) -> Option<Hover> {
    let path = create_type.path()?;
    let (schema, type_name) = resolve::resolve_type_info(db, file, &path)?;

    let snippet = if let Some(variant_list) = create_type.variant_list() {
        let variants = variant_list.syntax().text().to_string();
        format!("type {}.{} as enum {}", schema, type_name, variants)
    } else if let Some(column_list) = create_type.column_list() {
        let columns = column_list.syntax().text().to_string();
        format!("type {}.{} as {}", schema, type_name, columns)
    } else if let Some(attribute_list) = create_type.attribute_list() {
        let attributes = attribute_list.syntax().text().to_string();
        format!("type {}.{} {}", schema, type_name, attributes)
    } else {
        format!("type {}.{}", schema, type_name)
    };

    Some(hover_with_preceding_comment(snippet, create_type.syntax()))
}

fn hover_schema(db: &dyn Db, def: Location) -> Option<Hover> {
    let create_schema = def
        .to_node(db)?
        .ancestors()
        .find_map(ast::CreateSchema::cast)?;
    format_create_schema(create_schema)
}

fn create_schema_name(create_schema: ast::CreateSchema) -> Option<String> {
    if let Some(schema_name) = create_schema.name() {
        return Some(schema_name.syntax().text().to_string());
    }

    create_schema
        .role()
        .and_then(|r| r.name())
        .map(|n| n.syntax().text().to_string())
}

fn format_create_schema(create_schema: ast::CreateSchema) -> Option<Hover> {
    let schema_name = create_schema_name(create_schema)?;
    Some(Hover::snippet(format!("schema {}", schema_name)))
}

fn hover_function(db: &dyn Db, def: Location) -> Option<Hover> {
    let create_function = def
        .to_node(db)?
        .ancestors()
        .find_map(ast::CreateFunction::cast)?;
    format_create_function(db, def.file, create_function)
}

fn hover_named_arg_parameter(db: &dyn Db, def: Location) -> Option<Hover> {
    let def_node = def.to_node(db)?;
    let param = def_node.ancestors().find_map(ast::Param::cast)?;
    let param_name = param.name().map(|name| Name::from_node(&name))?;
    let param_type = param.ty().map(|ty| ty.syntax().text().to_string());

    for ancestor in def_node.ancestors() {
        if let Some(create_function) = ast::CreateFunction::cast(ancestor.clone()) {
            let path = create_function.path()?;
            let (schema, function_name) = resolve::resolve_function_info(db, def.file, &path)?;
            return Some(format_param_hover(
                schema,
                function_name,
                param_name,
                param_type,
            ));
        }
        if let Some(create_procedure) = ast::CreateProcedure::cast(ancestor.clone()) {
            let path = create_procedure.path()?;
            let (schema, procedure_name) = resolve::resolve_procedure_info(db, def.file, &path)?;
            return Some(format_param_hover(
                schema,
                procedure_name,
                param_name,
                param_type,
            ));
        }
        if let Some(create_aggregate) = ast::CreateAggregate::cast(ancestor) {
            let path = create_aggregate.path()?;
            let (schema, aggregate_name) = resolve::resolve_aggregate_info(db, def.file, &path)?;
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
) -> Hover {
    if let Some(param_type) = param_type {
        return Hover::snippet(format!(
            "parameter {}.{}.{} {}",
            schema, routine_name, param_name, param_type
        ));
    }

    Hover::snippet(format!(
        "parameter {}.{}.{}",
        schema, routine_name, param_name
    ))
}

fn format_create_function(
    db: &dyn Db,
    file: File,
    create_function: ast::CreateFunction,
) -> Option<Hover> {
    let path = create_function.path()?;
    let (schema, function_name) = resolve::resolve_function_info(db, file, &path)?;

    let params = create_function.param_list()?.syntax().text().to_string();
    let return_type = create_function.ret_type()?.syntax().text().to_string();
    let snippet = format!(
        "function {}.{}{} {}",
        schema, function_name, params, return_type
    );

    Some(hover_with_preceding_comment(
        snippet,
        create_function.syntax(),
    ))
}

fn hover_aggregate(db: &dyn Db, def: Location) -> Option<Hover> {
    let create_aggregate = def
        .to_node(db)?
        .ancestors()
        .find_map(ast::CreateAggregate::cast)?;
    format_create_aggregate(db, def.file, create_aggregate)
}

fn format_create_aggregate(
    db: &dyn Db,
    file: File,
    create_aggregate: ast::CreateAggregate,
) -> Option<Hover> {
    let path = create_aggregate.path()?;
    let (schema, aggregate_name) = resolve::resolve_aggregate_info(db, file, &path)?;

    let param_list = create_aggregate.param_list()?;
    let params = param_list.syntax().text().to_string();

    Some(Hover::snippet(format!(
        "aggregate {}.{}{}",
        schema, aggregate_name, params
    )))
}

fn hover_procedure(db: &dyn Db, def: Location) -> Option<Hover> {
    let create_procedure = def
        .to_node(db)?
        .ancestors()
        .find_map(ast::CreateProcedure::cast)?;
    format_create_procedure(db, def.file, create_procedure)
}

fn format_create_procedure(
    db: &dyn Db,
    file: File,
    create_procedure: ast::CreateProcedure,
) -> Option<Hover> {
    let path = create_procedure.path()?;
    let (schema, procedure_name) = resolve::resolve_procedure_info(db, file, &path)?;

    let param_list = create_procedure.param_list()?;
    let params = param_list.syntax().text().to_string();

    Some(Hover::snippet(format!(
        "procedure {}.{}{}",
        schema, procedure_name, params
    )))
}

fn hover_routine(db: &dyn Db, def: Location) -> Option<Hover> {
    for ancestor in def.to_node(db)?.ancestors() {
        if let Some(create_function) = ast::CreateFunction::cast(ancestor.clone()) {
            return format_create_function(db, def.file, create_function);
        }
        if let Some(create_aggregate) = ast::CreateAggregate::cast(ancestor.clone()) {
            return format_create_aggregate(db, def.file, create_aggregate);
        }
        if let Some(create_procedure) = ast::CreateProcedure::cast(ancestor) {
            return format_create_procedure(db, def.file, create_procedure);
        }
    }

    None
}

fn qualified_star_table_ptr(
    db: &dyn Db,
    file: File,
    field_expr: ast::FieldExpr,
) -> Option<SyntaxNodePtr> {
    let table_name = resolve::qualified_star_table_name(&field_expr)?;
    let position = field_expr.syntax().text_range().start();
    let target = field_expr
        .syntax()
        .ancestors()
        .find_map(ast::Target::cast)?;

    let path = match ast_nav::target_parent_query(target)? {
        ast_nav::ParentQuery::Select(select) => {
            let from_clause = select.from_clause()?;
            let from_item = resolve::find_from_item_in_from_clause(&from_clause, &table_name)?;

            if let Some(alias) = from_item.alias()
                && alias.column_list().is_some()
            {
                return Some(SyntaxNodePtr::new(alias.syntax()));
            }

            let (schema, table_name) = name::schema_and_table_from_from_item(&from_item)?;

            let name_ref = from_item.name_ref();
            if let Some((table_like_ptr, _kind)) = resolve::resolve_table_like(
                db,
                file,
                name_ref.as_ref(),
                &table_name,
                schema.as_ref(),
                position,
            ) {
                return Some(table_like_ptr);
            }

            return None;
        }
        ast_nav::ParentQuery::Update(update) => update.relation_name()?.path()?,
        ast_nav::ParentQuery::Delete(delete) => delete.relation_name()?.path()?,
        ast_nav::ParentQuery::Insert(insert) => insert.path()?,
        ast_nav::ParentQuery::Merge(merge) => merge.relation_name()?.path()?,
    };

    table_or_view_or_cte_ptrs(db, file, position, &path)?
        .into_iter()
        .next()
}

fn table_or_view_or_cte_ptrs(
    db: &dyn Db,
    file: File,
    position: TextSize,
    path: &ast::Path,
) -> Option<Vec<SyntaxNodePtr>> {
    let (schema, table_name) = name::schema_and_name_path(path)?;
    let mut results = vec![];
    let name_ref = path.segment().and_then(|x| x.name_ref());

    if let Some((table_like_ptr, _kind)) = resolve::resolve_table_like(
        db,
        file,
        name_ref.as_ref(),
        &table_name,
        schema.as_ref(),
        position,
    ) {
        results.push(table_like_ptr);
    }

    if results.is_empty() {
        return None;
    }
    Some(results)
}

fn unqualified_star_table_ptrs(
    db: &dyn Db,
    file: File,
    target: &ast::Target,
) -> Option<Vec<SyntaxNodePtr>> {
    target.star_token()?;

    let path = match ast_nav::target_parent_query(target.clone())? {
        ast_nav::ParentQuery::Select(select) => {
            let from_clause = select.from_clause()?;
            let results = resolve::table_ptrs_from_clause(db, file, &from_clause);
            if results.is_empty() {
                return None;
            }
            return Some(results);
        }
        ast_nav::ParentQuery::Update(update) => update.relation_name()?.path(),
        ast_nav::ParentQuery::Insert(insert) => insert.path(),
        ast_nav::ParentQuery::Delete(delete) => delete.relation_name()?.path(),
        ast_nav::ParentQuery::Merge(merge) => merge.relation_name()?.path(),
    }?;

    let position = target.syntax().text_range().start();
    table_or_view_or_cte_ptrs(db, file, position, &path)
}

fn unqualified_star_in_arg_list_ptrs(
    db: &dyn Db,
    file: File,
    arg_list: &ast::ArgList,
) -> Option<Vec<SyntaxNodePtr>> {
    let from_clause = arg_list
        .syntax()
        .ancestors()
        .find_map(ast::Select::cast)?
        .from_clause()?;
    let results = resolve::table_ptrs_from_clause(db, file, &from_clause);

    if results.is_empty() {
        return None;
    }

    Some(results)
}

#[cfg(test)]
mod test {
    use crate::db::{Database, File};
    use crate::hover::hover;
    use crate::test_utils::Fixture;
    use annotate_snippets::{AnnotationKind, Level, Renderer, Snippet, renderer::DecorStyle};
    use insta::assert_snapshot;

    #[track_caller]
    fn check_hover(sql: &str) -> String {
        check_hover_(sql).expect("should find hover information")
    }

    #[track_caller]
    fn check_hover_(sql: &str) -> Option<String> {
        let db = Database::default();
        let fixture = Fixture::new(sql);
        let marker = fixture.marker();
        let offset = marker.offset_before();
        let hover_span = marker.range();
        let sql = fixture.sql();
        let file = File::new(&db, sql.into());
        assert_eq!(crate::db::parse(&db, file).errors(), vec![]);

        if let Some(type_info) = hover(&db, file, offset) {
            let title = format!("hover: {}", type_info.snippet);
            let group = Level::INFO.primary_title(&title).element(
                Snippet::source(sql)
                    .fold(true)
                    .annotation(AnnotationKind::Context.span(hover_span).label("hover")),
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

    #[track_caller]
    fn check_hover_info(sql: &str) -> super::Hover {
        let db = Database::default();
        let fixture = Fixture::new(sql);
        let offset = fixture.marker().offset_before();
        let sql = fixture.sql();
        let file = File::new(&db, sql.into());
        assert_eq!(crate::db::parse(&db, file).errors(), vec![]);

        hover(&db, file, offset).expect("should find hover information")
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
    fn hover_function_extracts_preceding_comment() {
        let hover = check_hover_info(
            "
-- this is a doc comment
-- for foo
create function foo() returns int as $$ select 1 $$ language sql;
select foo$0();
",
        );
        assert_snapshot!(hover.markdown(), @"
        ```sql
        function public.foo() returns int
        ```
        ---
        this is a doc comment
        for foo
        ");
    }

    #[test]
    fn hover_type_extracts_preceding_comment() {
        let hover = check_hover_info(
            "
-- this is a doc comment
-- for foo
create type foo as enum ('a', 'b');
select 1::foo$0;
",
        );
        assert_snapshot!(hover.markdown(), @"
        ```sql
        type public.foo as enum ('a', 'b')
        ```
        ---
        this is a doc comment
        for foo
        ");
    }

    #[test]
    fn hover_bigint_extracts_preceding_comment_from_int8_definition() {
        let hover = check_hover_info(
            "
-- 64-bit integer
create type pg_catalog.int8;
select 1::bigint$0;
",
        );
        assert_snapshot!(hover.markdown(), @"
        ```sql
        type pg_catalog.int8
        ```
        ---
        64-bit integer
        ");
    }

    #[test]
    fn hover_text_type() {
        let hover = check_hover_info(
            "
-- variable-length string, no limit specified
--
-- size: -1, align: 4
create type pg_catalog.text;
select '1'::text$0;
",
        );
        assert_snapshot!(hover.markdown(), @"
        ```sql
        type pg_catalog.text
        ```
        ---
        variable-length string, no limit specified
        size: -1, align: 4
        ");
    }

    #[test]
    fn hover_column_extracts_preceding_comment() {
        let hover = check_hover_info(
            "
create table users(
  -- email address
  email text
);
select email$0 from users;
",
        );
        assert_snapshot!(hover.markdown(), @"
        ```sql
        column public.users.email text
        ```
        ---
        email address
        ");
    }

    #[test]
    fn hover_create_table_column_extracts_preceding_comment() {
        let hover = check_hover_info(
            "
create table users(
  -- email address
  email$0 text
);
",
        );
        assert_snapshot!(hover.markdown(), @"
        ```sql
        column public.users.email text
        ```
        ---
        email address
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
    fn hover_on_builtin_function_call() {
        assert_snapshot!(check_hover("
select now$0();
"), @r"
        hover: function pg_catalog.now() returns timestamp with time zone
          ╭▸ 
        2 │ select now();
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
"), @"
        hover: column t.a integer
          ╭▸ 
        2 │ select t.a from (select 1 a) t;
          ╰╴         ─ hover
        ");
    }

    #[test]
    fn hover_on_subquery_unqualified_column_ref_with_alias() {
        assert_snapshot!(check_hover("
select a$0 from (select 1 a) t;
"), @"
        hover: column t.a integer
          ╭▸ 
        2 │ select a from (select 1 a) t;
          ╰╴       ─ hover
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
    fn hover_on_select_cte_table_as_column() {
        assert_snapshot!(check_hover("
with t as (select 1 a, 2 b, 3 c)
select t$0 from t;
"), @r"
        hover: with t as (select 1 a, 2 b, 3 c)
          ╭▸ 
        3 │ select t from t;
          ╰╴       ─ hover
        ");
    }

    #[test]
    fn hover_on_cte_column() {
        assert_snapshot!(check_hover("
with t as (select 1 a)
select a$0 from t;
"), @"
        hover: column t.a integer
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
"), @"
        hover: column t.b integer
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
"), @"
        hover: column t.a integer
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
"), @"
        hover: column y.a integer
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
"), @"
        hover: column t.column1 integer
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
"), @"
        hover: column t.column2 integer
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
"), @"
        hover: column t.column1 integer
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
"), @"
        hover: column t.column1 integer
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
"), @"
        hover: column u.id integer
              column u.b integer
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
"), @"
        hover: column t.column1 integer
              column t.column2 integer
          ╭▸ 
        3 │ select t.* from t;
          ╰╴         ─ hover
        ");
    }

    #[test]
    fn hover_on_cte_table_alias_with_column_list() {
        assert_snapshot!(check_hover("
with t as (select 1 a, 2 b, 3 c)
select u$0.x, u.y from t as u(x, y);
"), @"
        hover: table u(x, y, c)
          ╭▸ 
        3 │ select u.x, u.y from t as u(x, y);
          ╰╴       ─ hover
        ");
    }

    #[test]
    fn hover_on_cte_table_alias_with_column_list_column_ref() {
        assert_snapshot!(check_hover("
with t as (select 1 a, 2 b, 3 c)
select u.x$0 from t as u(x, y);
"), @"
        hover: column u.x integer
          ╭▸ 
        3 │ select u.x from t as u(x, y);
          ╰╴         ─ hover
        ");
    }

    #[test]
    fn hover_on_cte_table_alias_with_column_list_table_ref() {
        assert_snapshot!(check_hover("
with t as (select 1 a, 2 b, 3 c)
select u$0 from t as u(x, y);
"), @"
        hover: table u(x, y, c)
          ╭▸ 
        3 │ select u from t as u(x, y);
          ╰╴       ─ hover
        ");
    }

    #[test]
    fn hover_on_subquery_alias_with_column_list_table_ref() {
        assert_snapshot!(check_hover("
with t as (select 1 a, 2 b, 3 c)
select z$0 from (select * from t) as z(x, y);
"), @"
        hover: table z(x, y, c)
          ╭▸ 
        3 │ select z from (select * from t) as z(x, y);
          ╰╴       ─ hover
        ");
    }

    #[test]
    fn hover_on_subquery_nested_paren_alias_with_column_list_table_ref() {
        assert_snapshot!(check_hover("
with t as (select 1 a, 2 b, 3 c)
select z$0 from ((select * from t)) as z(x, y);
"), @"
        hover: table z(x, y, c)
          ╭▸ 
        3 │ select z from ((select * from t)) as z(x, y);
          ╰╴       ─ hover
        ");
    }

    #[test]
    fn hover_on_cte_table_alias_with_partial_column_list_star() {
        assert_snapshot!(check_hover("
with t as (select 1 a, 2 b, 3 c)
select *$0 from t u(x, y);
"), @"
        hover: column u.x integer
              column u.y integer
              column u.c integer
          ╭▸ 
        3 │ select * from t u(x, y);
          ╰╴       ─ hover
        ");
    }

    #[test]
    fn hover_on_cte_table_alias_with_partial_column_list_star_from_information_schema() {
        assert_snapshot!(check_hover("
with t as (select * from information_schema.sql_features)
select *$0 from t u(x);
"), @"
        hover: column u.x character_data
              column u.feature_name character_data
              column u.sub_feature_id character_data
              column u.sub_feature_name character_data
              column u.is_supported yes_or_no
              column u.is_verified_by character_data
              column u.comments character_data
          ╭▸ 
        3 │ select * from t u(x);
          ╰╴       ─ hover
        ");
    }

    #[test]
    fn hover_cte_builtin_information_schema() {
        assert_snapshot!(check_hover("
with t as (select * from information_schema.sql_features) 
select *$0 from t;
"), @"
        hover: column t.feature_id character_data
              column t.feature_name character_data
              column t.sub_feature_id character_data
              column t.sub_feature_name character_data
              column t.is_supported yes_or_no
              column t.is_verified_by character_data
              column t.comments character_data
          ╭▸ 
        3 │ select * from t;
          ╰╴       ─ hover
        ");
    }

    #[test]
    fn hover_on_cte_table_alias_with_partial_column_list_qualified_star() {
        assert_snapshot!(check_hover("
with t as (select 1 a, 2 b, 3 c)
select u.*$0 from t u(x, y);
"), @"
        hover: column u.x integer
              column u.y integer
              column u.c integer
          ╭▸ 
        3 │ select u.* from t u(x, y);
          ╰╴         ─ hover
        ");
    }

    #[test]
    fn hover_on_star_from_cte_empty_select() {
        assert!(
            check_hover_(
                "
with t as (select)
select *$0 from t;
",
            )
            .is_none()
        );
    }

    #[test]
    fn hover_on_star_with_subquery_from_cte() {
        assert_snapshot!(check_hover("
with u as (select 1 id, 2 b)
select *$0 from (select *, *, * from u);
"), @"
        hover: column u.id integer
              column u.b integer
              column u.id integer
              column u.b integer
              column u.id integer
              column u.b integer
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
    fn hover_on_star_with_subquery_from_table_statement() {
        assert_snapshot!(check_hover("
with t as (select 1 a, 2 b)
select *$0 from (table t);
"), @"
        hover: column a integer
              column b integer
          ╭▸ 
        3 │ select * from (table t);
          ╰╴       ─ hover
        ");
    }

    #[test]
    fn hover_on_star_from_information_schema_table() {
        assert_snapshot!(check_hover("
select *$0 from information_schema.sql_features;
"), @"
        hover: column information_schema.sql_features.feature_id character_data
              column information_schema.sql_features.feature_name character_data
              column information_schema.sql_features.sub_feature_id character_data
              column information_schema.sql_features.sub_feature_name character_data
              column information_schema.sql_features.is_supported yes_or_no
              column information_schema.sql_features.is_verified_by character_data
              column information_schema.sql_features.comments character_data
          ╭▸ 
        2 │ select * from information_schema.sql_features;
          ╰╴       ─ hover
        ");
    }

    #[test]
    fn hover_on_star_with_subquery_literal() {
        assert_snapshot!(check_hover("
select *$0 from (select 1);
"), @"
        hover: column ?column? integer
          ╭▸ 
        2 │ select * from (select 1);
          ╰╴       ─ hover
        ");
    }

    #[test]
    fn hover_on_star_with_subquery_literal_with_alias() {
        assert_snapshot!(check_hover("
select *$0 from (select 1) as sub;
"), @"
        hover: column sub.?column? integer
          ╭▸ 
        2 │ select * from (select 1) as sub;
          ╰╴       ─ hover
        ");
    }

    #[test]
    fn hover_on_view_inferred_column_name() {
        assert_snapshot!(check_hover(r#"
create view v as select 1;
select "?column?"$0 from v;
"#), @r#"
        hover: column public.v.?column? integer
          ╭▸ 
        3 │ select "?column?" from v;
          ╰╴                ─ hover
        "#);
    }

    #[test]
    fn hover_on_cte_inferred_column_name() {
        assert_snapshot!(check_hover(r#"
with x as (select 1)
select "?column?"$0 from x;
"#), @r#"
        hover: column x.?column? integer
          ╭▸ 
        3 │ select "?column?" from x;
          ╰╴                ─ hover
        "#);
    }

    #[test]
    fn hover_on_create_table_as_inferred_column_name() {
        assert_snapshot!(check_hover(r#"
create table t as select 1;
select "?column?"$0 from t;
"#), @r#"
        hover: column public.t.?column? integer
          ╭▸ 
        3 │ select "?column?" from t;
          ╰╴                ─ hover
        "#);
    }

    #[test]
    fn hover_on_paren_select_inferred_column_name() {
        assert_snapshot!(check_hover(r#"
select "?column?"$0 from (select 1);
"#), @r#"
        hover: column ?column? integer
          ╭▸ 
        2 │ select "?column?" from (select 1);
          ╰╴                ─ hover
        "#);
    }

    #[test]
    fn hover_on_paren_select_aliased_inferred_column_name() {
        assert_snapshot!(check_hover(r#"
select sub."?column?"$0 from (select 1) sub;
"#), @r#"
        hover: column sub.?column? integer
          ╭▸ 
        2 │ select sub."?column?" from (select 1) sub;
          ╰╴                    ─ hover
        "#);
    }

    #[test]
    fn hover_on_view_qualified_star() {
        assert_snapshot!(check_hover("
create view v as select 1 id, 2 b;
select v.*$0 from v;
"), @"
        hover: column public.v.id integer
              column public.v.b integer
          ╭▸ 
        3 │ select v.* from v;
          ╰╴         ─ hover
        ");
    }

    #[test]
    fn hover_on_materialized_view_qualified_star() {
        assert_snapshot!(check_hover("
  create materialized view v as select 1 id, 2 b;
  select v.*$0 from v;
  "), @"
        hover: column public.v.id integer
              column public.v.b integer
          ╭▸ 
        3 │   select v.* from v;
          ╰╴           ─ hover
        ");
    }

    #[test]
    fn hover_on_view_qualified_star_with_column_list() {
        assert_snapshot!(check_hover("
create view v (x, y) as select 1, 2, 3;
select v.*$0 from v;
"), @"
        hover: column public.v.x integer
              column public.v.y integer
              column public.v.?column? integer
          ╭▸ 
        3 │ select v.* from v;
          ╰╴         ─ hover
        ");
    }

    #[test]
    fn hover_on_materialized_view_qualified_star_with_column_list() {
        assert_snapshot!(check_hover("
create materialized view mv (x, y) as select 1, 2, 3;
select mv.*$0 from mv;
"), @"
        hover: column public.mv.x integer
              column public.mv.y integer
              column public.mv.?column? integer
          ╭▸ 
        3 │ select mv.* from mv;
          ╰╴          ─ hover
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
"), @"
        hover: column new_data.email text
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
"), @"
        hover: column new_data.id integer
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
"), @"
        hover: column public.v.col1 integer
          ╭▸ 
        2 │ create view v(col1) as select 1;
          ╰╴                 ─ hover
        ");
    }

    #[test]
    fn hover_on_create_view_create_table_select_col() {
        assert_snapshot!(check_hover("
create table t(a bigint); 
create view v as
  select a from t;
select a$0 from v;
"), @"
        hover: column public.v.a bigint
          ╭▸ 
        5 │ select a from v;
          ╰╴       ─ hover
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
"), @"
        hover: column public.v.a integer
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
"), @"
        hover: column public.v.b integer
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
"), @"
        hover: column public.v.a integer
          ╭▸ 
        3 │ select a, b from v;
          ╰╴       ─ hover
        ");
    }

    #[test]
    fn hover_on_create_table_as_column() {
        assert_snapshot!(check_hover("
create table t as select 1 a;
select a$0 from t;
"), @"
        hover: column public.t.a integer
          ╭▸ 
        3 │ select a from t;
          ╰╴       ─ hover
        ");
    }

    #[test]
    fn hover_on_create_table_as_table() {
        assert_snapshot!(check_hover("
create table t as select 1 a;
select a from t$0;
"), @"
        hover: table public.t as select 1 a
          ╭▸ 
        3 │ select a from t;
          ╰╴              ─ hover
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
        hover: column merged.x int
              column merged.y int
           ╭▸ 
        16 │ select * from merged;
           ╰╴       ─ hover
        ");
    }

    #[test]
    fn hover_cte_insert_returning_aliased_column() {
        assert_snapshot!(check_hover("
create table t(a int, b int);
with inserted as (
  insert into t values (1, 2)
  returning a as x, b as y
)
select x$0 from inserted;
"), @r"
        hover: column inserted.x int
          ╭▸ 
        7 │ select x from inserted;
          ╰╴       ─ hover
        ");
    }

    #[test]
    fn hover_cte_update_returning_aliased_column() {
        assert_snapshot!(check_hover("
create table t(a int, b int);
with updated as (
  update t set a = 42
  returning a as x, b as y
)
select x$0 from updated;
"), @r"
        hover: column updated.x int
          ╭▸ 
        7 │ select x from updated;
          ╰╴       ─ hover
        ");
    }

    #[test]
    fn hover_cte_delete_returning_aliased_column() {
        assert_snapshot!(check_hover("
create table t(a int, b int);
with deleted as (
  delete from t
  returning a as x, b as y
)
select x$0 from deleted;
"), @r"
        hover: column deleted.x int
          ╭▸ 
        7 │ select x from deleted;
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
    fn hover_select_window_def_reuse() {
        assert_snapshot!(check_hover("
create table tbl (
  id bigint primary key,
  group_col text not null,
  update_date date not null,
  value text
);
select
  id,
  group_col,
  row_number() over w as rn,
  lag(value) over w$0 as prev_value
from tbl
window w as (
  partition by group_col
  order by update_date desc
);
"), @r"
hover: window w as (
        partition by group_col
        order by update_date desc
      )
   ╭▸ 
12 │   lag(value) over w as prev_value
   ╰╴                  ─ hover
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
    fn hover_create_table_inherits_builtin_star() {
        assert_snapshot!(check_hover_info("
create table t ()
inherits (information_schema.sql_features);
select *$0 from t;
").snippet, @"
        column public.t.feature_id character_data
        column public.t.feature_name character_data
        column public.t.sub_feature_id character_data
        column public.t.sub_feature_name character_data
        column public.t.is_supported yes_or_no
        column public.t.is_verified_by character_data
        column public.t.comments character_data
        ");
    }

    #[test]
    fn hover_create_table_like_builtin_star() {
        assert_snapshot!(check_hover_info("
create table t (like information_schema.sql_features);
select *$0 from t;
").snippet, @"
        column public.t.feature_id character_data
        column public.t.feature_name character_data
        column public.t.sub_feature_id character_data
        column public.t.sub_feature_name character_data
        column public.t.is_supported yes_or_no
        column public.t.is_verified_by character_data
        column public.t.comments character_data
        ");
    }

    #[test]
    fn hover_create_table_inherits_create_table_as_star() {
        assert_snapshot!(check_hover_info("
create table parent as select 1 a, 'x'::text b;
create table child (c int) inherits (parent);
select *$0 from child;
").snippet, @"
        column public.child.a integer
        column public.child.b text
        column public.child.c int
        ");
    }

    #[test]
    fn hover_create_table_like_select_into_star() {
        assert_snapshot!(check_hover_info("
select 1 a, 'x'::text b into parent;
create table child (like parent);
select *$0 from child;
").snippet, @"
        column public.child.a integer
        column public.child.b text
        ");
    }

    #[test]
    fn hover_create_table_like_view_star() {
        assert_snapshot!(check_hover_info("
create view parent as select 1 a, 'x'::text b;
create table child (like parent);
select *$0 from child;
").snippet, @"
        column public.child.a integer
        column public.child.b text
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
    fn hover_create_table_inherits_builtin_column() {
        assert_snapshot!(check_hover("
create table t ()
inherits (information_schema.sql_features);
select feature_name$0 from t;
"), @"
        hover: column information_schema.sql_features.feature_name information_schema.character_data
          ╭▸ 
        4 │ select feature_name from t;
          ╰╴                  ─ hover
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
"), @"
        hover: listen updates
          ╭▸ 
        3 │ unlisten updates;
          ╰╴               ─ hover
        ");
    }

    #[test]
    fn hover_property_graph_on_create() {
        assert_snapshot!(check_hover("
create property graph foo.ba$0r vertex tables (t key (a) no properties);
"), @"
        hover: property graph foo.bar
          ╭▸ 
        2 │ create property graph foo.bar vertex tables (t key (a) no properties);
          ╰╴                           ─ hover
        ");
    }

    #[test]
    fn hover_property_graph_on_drop() {
        assert_snapshot!(check_hover("
create property graph foo.bar vertex tables (t key (a) no properties);
drop property graph foo.ba$0r;
"), @"
        hover: property graph foo.bar
          ╭▸ 
        3 │ drop property graph foo.bar;
          ╰╴                         ─ hover
        ");
    }

    #[test]
    fn hover_property_graph_on_alter() {
        assert_snapshot!(check_hover("
create property graph foo.bar vertex tables (t key (a) no properties);
alter property graph foo.ba$0r rename to baz;
"), @"
        hover: property graph foo.bar
          ╭▸ 
        3 │ alter property graph foo.bar rename to baz;
          ╰╴                          ─ hover
        ");
    }
}
