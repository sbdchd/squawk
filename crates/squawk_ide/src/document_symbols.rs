use rowan::TextRange;
use squawk_syntax::ast::{self, AstNode};

use crate::binder::{self, extract_string_literal};
use crate::resolve::{
    resolve_aggregate_info, resolve_function_info, resolve_procedure_info, resolve_sequence_info,
    resolve_table_info, resolve_type_info, resolve_view_info,
};

#[derive(Debug)]
pub enum DocumentSymbolKind {
    Schema,
    Table,
    View,
    MaterializedView,
    Function,
    Aggregate,
    Procedure,
    EventTrigger,
    Role,
    Policy,
    Type,
    Enum,
    Index,
    Domain,
    Sequence,
    Trigger,
    Tablespace,
    Database,
    Server,
    Extension,
    Column,
    Variant,
    Cursor,
    PreparedStatement,
    Channel,
}

#[derive(Debug)]
pub struct DocumentSymbol {
    pub name: String,
    pub detail: Option<String>,
    pub kind: DocumentSymbolKind,
    /// Range used for determining when cursor is inside the symbol for showing
    /// in the UI
    pub full_range: TextRange,
    /// Range selected when symbol is selected
    pub focus_range: TextRange,
    pub children: Vec<DocumentSymbol>,
}

pub fn document_symbols(file: &ast::SourceFile) -> Vec<DocumentSymbol> {
    let binder = binder::bind(file);
    let mut symbols = vec![];

    for stmt in file.stmts() {
        match stmt {
            ast::Stmt::CreateSchema(create_schema) => {
                if let Some(symbol) = create_schema_symbol(create_schema) {
                    symbols.push(symbol);
                }
            }
            ast::Stmt::CreateTable(create_table) => {
                if let Some(symbol) = create_table_symbol(&binder, create_table) {
                    symbols.push(symbol);
                }
            }
            ast::Stmt::CreateTableAs(create_table_as) => {
                if let Some(symbol) = create_table_as_symbol(&binder, create_table_as) {
                    symbols.push(symbol);
                }
            }
            ast::Stmt::CreateForeignTable(create_foreign_table) => {
                if let Some(symbol) = create_table_symbol(&binder, create_foreign_table) {
                    symbols.push(symbol);
                }
            }
            ast::Stmt::CreateFunction(create_function) => {
                if let Some(symbol) = create_function_symbol(&binder, create_function) {
                    symbols.push(symbol);
                }
            }
            ast::Stmt::CreateAggregate(create_aggregate) => {
                if let Some(symbol) = create_aggregate_symbol(&binder, create_aggregate) {
                    symbols.push(symbol);
                }
            }
            ast::Stmt::CreateProcedure(create_procedure) => {
                if let Some(symbol) = create_procedure_symbol(&binder, create_procedure) {
                    symbols.push(symbol);
                }
            }
            ast::Stmt::CreateIndex(create_index) => {
                if let Some(symbol) = create_index_symbol(create_index) {
                    symbols.push(symbol);
                }
            }
            ast::Stmt::CreateDomain(create_domain) => {
                if let Some(symbol) = create_domain_symbol(&binder, create_domain) {
                    symbols.push(symbol);
                }
            }
            ast::Stmt::CreateSequence(create_sequence) => {
                if let Some(symbol) = create_sequence_symbol(&binder, create_sequence) {
                    symbols.push(symbol);
                }
            }
            ast::Stmt::CreateTrigger(create_trigger) => {
                if let Some(symbol) = create_trigger_symbol(create_trigger) {
                    symbols.push(symbol);
                }
            }
            ast::Stmt::CreateEventTrigger(create_event_trigger) => {
                if let Some(symbol) = create_event_trigger_symbol(create_event_trigger) {
                    symbols.push(symbol);
                }
            }
            ast::Stmt::CreateTablespace(create_tablespace) => {
                if let Some(symbol) = create_tablespace_symbol(create_tablespace) {
                    symbols.push(symbol);
                }
            }
            ast::Stmt::CreateDatabase(create_database) => {
                if let Some(symbol) = create_database_symbol(create_database) {
                    symbols.push(symbol);
                }
            }
            ast::Stmt::CreateServer(create_server) => {
                if let Some(symbol) = create_server_symbol(create_server) {
                    symbols.push(symbol);
                }
            }
            ast::Stmt::CreateExtension(create_extension) => {
                if let Some(symbol) = create_extension_symbol(create_extension) {
                    symbols.push(symbol);
                }
            }
            ast::Stmt::CreateRole(create_role) => {
                if let Some(symbol) = create_role_symbol(create_role) {
                    symbols.push(symbol);
                }
            }
            ast::Stmt::CreatePolicy(create_policy) => {
                if let Some(symbol) = create_policy_symbol(create_policy) {
                    symbols.push(symbol);
                }
            }
            ast::Stmt::CreateType(create_type) => {
                if let Some(symbol) = create_type_symbol(&binder, create_type) {
                    symbols.push(symbol);
                }
            }
            ast::Stmt::CreateView(create_view) => {
                if let Some(symbol) = create_view_symbol(&binder, create_view) {
                    symbols.push(symbol);
                }
            }
            ast::Stmt::CreateMaterializedView(create_view) => {
                if let Some(symbol) = create_materialized_view_symbol(&binder, create_view) {
                    symbols.push(symbol);
                }
            }
            ast::Stmt::Declare(declare) => {
                if let Some(symbol) = create_declare_cursor_symbol(declare) {
                    symbols.push(symbol);
                }
            }
            ast::Stmt::Prepare(prepare) => {
                if let Some(symbol) = create_prepare_symbol(prepare) {
                    symbols.push(symbol);
                }
            }
            ast::Stmt::Select(select) => {
                symbols.extend(cte_table_symbols(select));
            }
            ast::Stmt::SelectInto(select_into) => {
                symbols.extend(cte_table_symbols(select_into));
            }
            ast::Stmt::Insert(insert) => {
                symbols.extend(cte_table_symbols(insert));
            }
            ast::Stmt::Update(update) => {
                symbols.extend(cte_table_symbols(update));
            }
            ast::Stmt::Delete(delete) => {
                symbols.extend(cte_table_symbols(delete));
            }
            ast::Stmt::Listen(listen) => {
                if let Some(symbol) = create_listen_symbol(listen) {
                    symbols.push(symbol);
                }
            }
            ast::Stmt::Notify(notify) => {
                if let Some(symbol) = create_notify_symbol(notify) {
                    symbols.push(symbol);
                }
            }
            ast::Stmt::Unlisten(unlisten) => {
                if let Some(symbol) = create_unlisten_symbol(unlisten) {
                    symbols.push(symbol);
                }
            }

            _ => {}
        }
    }

    symbols
}

fn cte_table_symbols(stmt: impl ast::HasWithClause) -> Vec<DocumentSymbol> {
    let Some(with_clause) = stmt.with_clause() else {
        return vec![];
    };

    with_clause
        .with_tables()
        .filter_map(create_cte_table_symbol)
        .collect()
}

fn create_cte_table_symbol(with_table: ast::WithTable) -> Option<DocumentSymbol> {
    let name_node = with_table.name()?;
    let name = name_node.syntax().text().to_string();

    let full_range = with_table.syntax().text_range();
    let focus_range = name_node.syntax().text_range();

    symbols_from_column_list(
        with_table.column_list(),
        name,
        full_range,
        focus_range,
        DocumentSymbolKind::Table,
    )
}

fn create_schema_symbol(create_schema: ast::CreateSchema) -> Option<DocumentSymbol> {
    let (name, focus_range) = if let Some(name_node) = create_schema.name() {
        (
            name_node.syntax().text().to_string(),
            name_node.syntax().text_range(),
        )
    } else if let Some(name) = create_schema.role().and_then(|r| r.name()) {
        (name.syntax().text().to_string(), name.syntax().text_range())
    } else {
        return None;
    };

    let full_range = create_schema.syntax().text_range();

    Some(DocumentSymbol {
        name,
        detail: None,
        kind: DocumentSymbolKind::Schema,
        full_range,
        focus_range,
        children: vec![],
    })
}

fn create_table_symbol(
    binder: &binder::Binder,
    create_table: impl ast::HasCreateTable,
) -> Option<DocumentSymbol> {
    let path = create_table.path()?;
    let segment = path.segment()?;
    let name_node = segment.name()?;

    let (schema, table_name) = resolve_table_info(binder, &path)?;
    let name = format!("{}.{}", schema.0, table_name);

    let full_range = create_table.syntax().text_range();
    let focus_range = name_node.syntax().text_range();

    let mut children = vec![];
    if let Some(table_arg_list) = create_table.table_arg_list() {
        for arg in table_arg_list.args() {
            if let ast::TableArg::Column(column) = arg
                && let Some(column_symbol) = create_column_symbol(column)
            {
                children.push(column_symbol);
            }
        }
    }

    Some(DocumentSymbol {
        name,
        detail: None,
        kind: DocumentSymbolKind::Table,
        full_range,
        focus_range,
        children,
    })
}

fn create_table_as_symbol(
    binder: &binder::Binder,
    create_table_as: ast::CreateTableAs,
) -> Option<DocumentSymbol> {
    let path = create_table_as.path()?;
    let segment = path.segment()?;
    let name_node = if let Some(name) = segment.name() {
        name.syntax().clone()
    } else {
        return None;
    };

    let (schema, table_name) = resolve_table_info(binder, &path)?;
    let name = format!("{}.{}", schema.0, table_name);

    let full_range = create_table_as.syntax().text_range();
    let focus_range = name_node.text_range();

    Some(DocumentSymbol {
        name,
        detail: None,
        kind: DocumentSymbolKind::Table,
        full_range,
        focus_range,
        // TODO: infer the column names, we need the same for views without
        // explicit column lists
        children: vec![],
    })
}

fn create_view_symbol(
    binder: &binder::Binder,
    create_view: ast::CreateView,
) -> Option<DocumentSymbol> {
    let path = create_view.path()?;
    let segment = path.segment()?;
    let name_node = segment.name()?;

    let (schema, view_name) = resolve_view_info(binder, &path)?;
    let name = format!("{}.{}", schema.0, view_name);

    let full_range = create_view.syntax().text_range();
    let focus_range = name_node.syntax().text_range();

    symbols_from_column_list(
        create_view.column_list(),
        name,
        full_range,
        focus_range,
        DocumentSymbolKind::View,
    )
}

fn symbols_from_column_list(
    column_list: Option<ast::ColumnList>,
    name: String,
    full_range: TextRange,
    focus_range: TextRange,
    kind: DocumentSymbolKind,
) -> Option<DocumentSymbol> {
    let mut children = vec![];
    if let Some(column_list) = column_list {
        for column in column_list.columns() {
            if let Some(column_symbol) = create_column_symbol(column) {
                children.push(column_symbol);
            }
        }
    }

    Some(DocumentSymbol {
        name,
        detail: None,
        kind,
        full_range,
        focus_range,
        children,
    })
}

// TODO: combine with create_view_symbol
fn create_materialized_view_symbol(
    binder: &binder::Binder,
    create_view: ast::CreateMaterializedView,
) -> Option<DocumentSymbol> {
    let path = create_view.path()?;
    let segment = path.segment()?;
    let name_node = segment.name()?;

    let (schema, view_name) = resolve_view_info(binder, &path)?;
    let name = format!("{}.{}", schema.0, view_name);

    let full_range = create_view.syntax().text_range();
    let focus_range = name_node.syntax().text_range();

    symbols_from_column_list(
        create_view.column_list(),
        name,
        full_range,
        focus_range,
        DocumentSymbolKind::MaterializedView,
    )
}

fn create_function_symbol(
    binder: &binder::Binder,
    create_function: ast::CreateFunction,
) -> Option<DocumentSymbol> {
    let path = create_function.path()?;
    let segment = path.segment()?;
    let name_node = segment.name()?;

    let (schema, function_name) = resolve_function_info(binder, &path)?;
    let name = format!("{}.{}", schema.0, function_name);

    let full_range = create_function.syntax().text_range();
    let focus_range = name_node.syntax().text_range();

    Some(DocumentSymbol {
        name,
        detail: None,
        kind: DocumentSymbolKind::Function,
        full_range,
        focus_range,
        children: vec![],
    })
}

fn create_aggregate_symbol(
    binder: &binder::Binder,
    create_aggregate: ast::CreateAggregate,
) -> Option<DocumentSymbol> {
    let path = create_aggregate.path()?;
    let segment = path.segment()?;
    let name_node = segment.name()?;

    let (schema, aggregate_name) = resolve_aggregate_info(binder, &path)?;
    let name = format!("{}.{}", schema.0, aggregate_name);

    let full_range = create_aggregate.syntax().text_range();
    let focus_range = name_node.syntax().text_range();

    Some(DocumentSymbol {
        name,
        detail: None,
        kind: DocumentSymbolKind::Aggregate,
        full_range,
        focus_range,
        children: vec![],
    })
}

fn create_procedure_symbol(
    binder: &binder::Binder,
    create_procedure: ast::CreateProcedure,
) -> Option<DocumentSymbol> {
    let path = create_procedure.path()?;
    let segment = path.segment()?;
    let name_node = segment.name()?;

    let (schema, procedure_name) = resolve_procedure_info(binder, &path)?;
    let name = format!("{}.{}", schema.0, procedure_name);

    let full_range = create_procedure.syntax().text_range();
    let focus_range = name_node.syntax().text_range();

    Some(DocumentSymbol {
        name,
        detail: None,
        kind: DocumentSymbolKind::Procedure,
        full_range,
        focus_range,
        children: vec![],
    })
}

fn create_index_symbol(create_index: ast::CreateIndex) -> Option<DocumentSymbol> {
    let name_node = create_index.name()?;
    let name = name_node.syntax().text().to_string();

    let full_range = create_index.syntax().text_range();
    let focus_range = name_node.syntax().text_range();

    Some(DocumentSymbol {
        name,
        detail: None,
        kind: DocumentSymbolKind::Index,
        full_range,
        focus_range,
        children: vec![],
    })
}

fn create_domain_symbol(
    binder: &binder::Binder,
    create_domain: ast::CreateDomain,
) -> Option<DocumentSymbol> {
    let path = create_domain.path()?;
    let segment = path.segment()?;
    let name_node = segment.name()?;

    let (schema, domain_name) = resolve_type_info(binder, &path)?;
    let name = format!("{}.{}", schema.0, domain_name);

    let full_range = create_domain.syntax().text_range();
    let focus_range = name_node.syntax().text_range();

    Some(DocumentSymbol {
        name,
        detail: None,
        kind: DocumentSymbolKind::Domain,
        full_range,
        focus_range,
        children: vec![],
    })
}

fn create_sequence_symbol(
    binder: &binder::Binder,
    create_sequence: ast::CreateSequence,
) -> Option<DocumentSymbol> {
    let path = create_sequence.path()?;
    let segment = path.segment()?;
    let name_node = segment.name()?;

    let (schema, sequence_name) = resolve_sequence_info(binder, &path)?;
    let name = format!("{}.{}", schema.0, sequence_name);

    let full_range = create_sequence.syntax().text_range();
    let focus_range = name_node.syntax().text_range();

    Some(DocumentSymbol {
        name,
        detail: None,
        kind: DocumentSymbolKind::Sequence,
        full_range,
        focus_range,
        children: vec![],
    })
}

fn create_trigger_symbol(create_trigger: ast::CreateTrigger) -> Option<DocumentSymbol> {
    let name_node = create_trigger.name()?;
    let name = name_node.syntax().text().to_string();

    let full_range = create_trigger.syntax().text_range();
    let focus_range = name_node.syntax().text_range();

    Some(DocumentSymbol {
        name,
        detail: None,
        kind: DocumentSymbolKind::Trigger,
        full_range,
        focus_range,
        children: vec![],
    })
}

fn create_event_trigger_symbol(
    create_event_trigger: ast::CreateEventTrigger,
) -> Option<DocumentSymbol> {
    let name_node = create_event_trigger.name()?;
    let name = name_node.syntax().text().to_string();

    let full_range = create_event_trigger.syntax().text_range();
    let focus_range = name_node.syntax().text_range();

    Some(DocumentSymbol {
        name,
        detail: None,
        kind: DocumentSymbolKind::EventTrigger,
        full_range,
        focus_range,
        children: vec![],
    })
}

fn create_tablespace_symbol(create_tablespace: ast::CreateTablespace) -> Option<DocumentSymbol> {
    let name_node = create_tablespace.name()?;
    let name = name_node.syntax().text().to_string();

    let full_range = create_tablespace.syntax().text_range();
    let focus_range = name_node.syntax().text_range();

    Some(DocumentSymbol {
        name,
        detail: None,
        kind: DocumentSymbolKind::Tablespace,
        full_range,
        focus_range,
        children: vec![],
    })
}

fn create_database_symbol(create_database: ast::CreateDatabase) -> Option<DocumentSymbol> {
    let name_node = create_database.name()?;
    let name = name_node.syntax().text().to_string();

    let full_range = create_database.syntax().text_range();
    let focus_range = name_node.syntax().text_range();

    Some(DocumentSymbol {
        name,
        detail: None,
        kind: DocumentSymbolKind::Database,
        full_range,
        focus_range,
        children: vec![],
    })
}

fn create_server_symbol(create_server: ast::CreateServer) -> Option<DocumentSymbol> {
    let name_node = create_server.name()?;
    let name = name_node.syntax().text().to_string();

    let full_range = create_server.syntax().text_range();
    let focus_range = name_node.syntax().text_range();

    Some(DocumentSymbol {
        name,
        detail: None,
        kind: DocumentSymbolKind::Server,
        full_range,
        focus_range,
        children: vec![],
    })
}

fn create_extension_symbol(create_extension: ast::CreateExtension) -> Option<DocumentSymbol> {
    let name_node = create_extension.name()?;
    let name = name_node.syntax().text().to_string();

    let full_range = create_extension.syntax().text_range();
    let focus_range = name_node.syntax().text_range();

    Some(DocumentSymbol {
        name,
        detail: None,
        kind: DocumentSymbolKind::Extension,
        full_range,
        focus_range,
        children: vec![],
    })
}

fn create_role_symbol(create_role: ast::CreateRole) -> Option<DocumentSymbol> {
    let name_node = create_role.name()?;
    let name = name_node.syntax().text().to_string();

    let full_range = create_role.syntax().text_range();
    let focus_range = name_node.syntax().text_range();

    Some(DocumentSymbol {
        name,
        detail: None,
        kind: DocumentSymbolKind::Role,
        full_range,
        focus_range,
        children: vec![],
    })
}

fn create_policy_symbol(create_policy: ast::CreatePolicy) -> Option<DocumentSymbol> {
    let name_node = create_policy.name()?;
    let name = name_node.syntax().text().to_string();

    let full_range = create_policy.syntax().text_range();
    let focus_range = name_node.syntax().text_range();

    Some(DocumentSymbol {
        name,
        detail: None,
        kind: DocumentSymbolKind::Policy,
        full_range,
        focus_range,
        children: vec![],
    })
}

fn create_type_symbol(
    binder: &binder::Binder,
    create_type: ast::CreateType,
) -> Option<DocumentSymbol> {
    let path = create_type.path()?;
    let segment = path.segment()?;
    let name_node = segment.name()?;

    let (schema, type_name) = resolve_type_info(binder, &path)?;
    let name = format!("{}.{}", schema.0, type_name);

    let full_range = create_type.syntax().text_range();
    let focus_range = name_node.syntax().text_range();

    let mut children = vec![];
    if let Some(variant_list) = create_type.variant_list() {
        for variant in variant_list.variants() {
            if let Some(variant_symbol) = create_variant_symbol(variant) {
                children.push(variant_symbol);
            }
        }
    } else if let Some(column_list) = create_type.column_list() {
        for column in column_list.columns() {
            if let Some(column_symbol) = create_column_symbol(column) {
                children.push(column_symbol);
            }
        }
    }

    Some(DocumentSymbol {
        name,
        detail: None,
        kind: if create_type.variant_list().is_some() {
            DocumentSymbolKind::Enum
        } else {
            DocumentSymbolKind::Type
        },
        full_range,
        focus_range,
        children,
    })
}

fn create_column_symbol(column: ast::Column) -> Option<DocumentSymbol> {
    let name_node = column.name()?;
    let name = name_node.syntax().text().to_string();

    let detail = column.ty().map(|t| t.syntax().text().to_string());

    let full_range = column.syntax().text_range();
    let focus_range = name_node.syntax().text_range();

    Some(DocumentSymbol {
        name,
        detail,
        kind: DocumentSymbolKind::Column,
        full_range,
        focus_range,
        children: vec![],
    })
}

fn create_variant_symbol(variant: ast::Variant) -> Option<DocumentSymbol> {
    let literal = variant.literal()?;
    let name = extract_string_literal(&literal)?;

    let full_range = variant.syntax().text_range();
    let focus_range = literal.syntax().text_range();

    Some(DocumentSymbol {
        name,
        detail: None,
        kind: DocumentSymbolKind::Variant,
        full_range,
        focus_range,
        children: vec![],
    })
}

fn create_declare_cursor_symbol(declare: ast::Declare) -> Option<DocumentSymbol> {
    let name_node = declare.name()?;
    let name = name_node.syntax().text().to_string();

    let full_range = declare.syntax().text_range();
    let focus_range = name_node.syntax().text_range();

    Some(DocumentSymbol {
        name,
        detail: None,
        kind: DocumentSymbolKind::Cursor,
        full_range,
        focus_range,
        children: vec![],
    })
}

fn create_prepare_symbol(prepare: ast::Prepare) -> Option<DocumentSymbol> {
    let name_node = prepare.name()?;
    let name = name_node.syntax().text().to_string();

    let full_range = prepare.syntax().text_range();
    let focus_range = name_node.syntax().text_range();

    Some(DocumentSymbol {
        name,
        detail: None,
        kind: DocumentSymbolKind::PreparedStatement,
        full_range,
        focus_range,
        children: vec![],
    })
}

fn create_listen_symbol(listen: ast::Listen) -> Option<DocumentSymbol> {
    let name_node = listen.name()?;
    let name = name_node.syntax().text().to_string();

    let full_range = listen.syntax().text_range();
    let focus_range = name_node.syntax().text_range();

    Some(DocumentSymbol {
        name,
        detail: Some("listen".to_string()),
        kind: DocumentSymbolKind::Channel,
        full_range,
        focus_range,
        children: vec![],
    })
}

fn create_notify_symbol(notify: ast::Notify) -> Option<DocumentSymbol> {
    let name_node = notify.name_ref()?;
    let name = name_node.syntax().text().to_string();

    let full_range = notify.syntax().text_range();
    let focus_range = name_node.syntax().text_range();

    Some(DocumentSymbol {
        name,
        detail: Some("notify".to_string()),
        kind: DocumentSymbolKind::Channel,
        full_range,
        focus_range,
        children: vec![],
    })
}

fn create_unlisten_symbol(unlisten: ast::Unlisten) -> Option<DocumentSymbol> {
    let name_node = unlisten.name_ref()?;
    let name = name_node.syntax().text().to_string();

    let full_range = unlisten.syntax().text_range();
    let focus_range = name_node.syntax().text_range();

    Some(DocumentSymbol {
        name,
        detail: Some("unlisten".to_string()),
        kind: DocumentSymbolKind::Channel,
        full_range,
        focus_range,
        children: vec![],
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use annotate_snippets::{
        AnnotationKind, Group, Level, Renderer, Snippet, renderer::DecorStyle,
    };
    use insta::assert_snapshot;

    fn symbols_not_found(sql: &str) {
        let parse = ast::SourceFile::parse(sql);
        let file = parse.tree();
        let symbols = document_symbols(&file);
        if !symbols.is_empty() {
            panic!("Symbols found. If this is expected, use `symbols` instead.")
        }
    }

    fn symbols(sql: &str) -> String {
        let parse = ast::SourceFile::parse(sql);
        let file = parse.tree();
        let symbols = document_symbols(&file);
        if symbols.is_empty() {
            panic!("No symbols found. If this is expected, use `symbols_not_found` instead.")
        }

        let mut output = vec![];
        for symbol in symbols {
            let group = symbol_to_group(&symbol, sql);
            output.push(group);
        }
        Renderer::plain()
            .decor_style(DecorStyle::Unicode)
            .render(&output)
            .to_string()
    }

    fn symbol_to_group<'a>(symbol: &DocumentSymbol, sql: &'a str) -> Group<'a> {
        let kind = match symbol.kind {
            DocumentSymbolKind::Schema => "schema",
            DocumentSymbolKind::Table => "table",
            DocumentSymbolKind::View => "view",
            DocumentSymbolKind::MaterializedView => "materialized view",
            DocumentSymbolKind::Function => "function",
            DocumentSymbolKind::Aggregate => "aggregate",
            DocumentSymbolKind::Procedure => "procedure",
            DocumentSymbolKind::EventTrigger => "event trigger",
            DocumentSymbolKind::Role => "role",
            DocumentSymbolKind::Policy => "policy",
            DocumentSymbolKind::Type => "type",
            DocumentSymbolKind::Enum => "enum",
            DocumentSymbolKind::Index => "index",
            DocumentSymbolKind::Domain => "domain",
            DocumentSymbolKind::Sequence => "sequence",
            DocumentSymbolKind::Trigger => "trigger",
            DocumentSymbolKind::Tablespace => "tablespace",
            DocumentSymbolKind::Database => "database",
            DocumentSymbolKind::Server => "server",
            DocumentSymbolKind::Extension => "extension",
            DocumentSymbolKind::Column => "column",
            DocumentSymbolKind::Variant => "variant",
            DocumentSymbolKind::Cursor => "cursor",
            DocumentSymbolKind::PreparedStatement => "prepared statement",
            DocumentSymbolKind::Channel => "channel",
        };

        let title = if let Some(detail) = &symbol.detail {
            format!("{}: {} {}", kind, symbol.name, detail)
        } else {
            format!("{}: {}", kind, symbol.name)
        };

        let snippet = Snippet::source(sql)
            .fold(true)
            .annotation(
                AnnotationKind::Primary
                    .span(symbol.focus_range.into())
                    .label("focus range"),
            )
            .annotation(
                AnnotationKind::Context
                    .span(symbol.full_range.into())
                    .label("full range"),
            );

        let mut group = Level::INFO.primary_title(title.clone()).element(snippet);

        if !symbol.children.is_empty() {
            let child_labels: Vec<String> = symbol
                .children
                .iter()
                .map(|child| {
                    let kind = match child.kind {
                        DocumentSymbolKind::Column => "column",
                        DocumentSymbolKind::Variant => "variant",
                        _ => unreachable!("only columns and variants can be children"),
                    };
                    if let Some(detail) = &child.detail {
                        format!("{}: {} {}", kind, child.name, detail)
                    } else {
                        format!("{}: {}", kind, child.name)
                    }
                })
                .collect();

            let mut children_snippet = Snippet::source(sql).fold(true);

            for (i, child) in symbol.children.iter().enumerate() {
                children_snippet = children_snippet
                    .annotation(
                        AnnotationKind::Context
                            .span(child.full_range.into())
                            .label(format!("full range for `{}`", child_labels[i].clone())),
                    )
                    .annotation(
                        AnnotationKind::Primary
                            .span(child.focus_range.into())
                            .label("focus range"),
                    );
            }

            group = group.element(children_snippet);
        }

        group
    }

    #[test]
    fn create_table() {
        assert_snapshot!(symbols("
create table users (
  id int,
  email citext
);"), @r"
        info: table: public.users
          ╭▸ 
        2 │   create table users (
          │   │            ━━━━━ focus range
          │ ┌─┘
          │ │
        3 │ │   id int,
        4 │ │   email citext
        5 │ │ );
          │ └─┘ full range
          │
          ⸬  
        3 │     id int,
          │     ┯━────
          │     │
          │     full range for `column: id int`
          │     focus range
        4 │     email citext
          │     ┯━━━━───────
          │     │
          │     full range for `column: email citext`
          ╰╴    focus range
        ");
    }

    #[test]
    fn create_table_as() {
        assert_snapshot!(symbols("
create table t as select 1 a;
"), @r"
        info: table: public.t
          ╭▸ 
        2 │ create table t as select 1 a;
          │ ┬────────────┯──────────────
          │ │            │
          │ │            focus range
          ╰╴full range
        ");
    }

    #[test]
    fn create_schema() {
        assert_snapshot!(symbols("
create schema foo;
"), @r"
        info: schema: foo
          ╭▸ 
        2 │ create schema foo;
          │ ┬─────────────┯━━
          │ │             │
          │ │             focus range
          ╰╴full range
        ");
    }

    #[test]
    fn create_schema_authorization() {
        assert_snapshot!(symbols("
create schema authorization foo;
"), @r"
        info: schema: foo
          ╭▸ 
        2 │ create schema authorization foo;
          │ ┬───────────────────────────┯━━
          │ │                           │
          │ │                           focus range
          ╰╴full range
        ");
    }

    #[test]
    fn listen_notify_unlisten() {
        assert_snapshot!(symbols("
listen updates;
notify updates;
unlisten updates;
unlisten *;
"), @r"
        info: channel: updates listen
          ╭▸ 
        2 │ listen updates;
          │ ┬──────┯━━━━━━
          │ │      │
          │ │      focus range
          │ full range
          ╰╴
        info: channel: updates notify
          ╭▸ 
        3 │ notify updates;
          │ ┬──────┯━━━━━━
          │ │      │
          │ │      focus range
          ╰╴full range
        info: channel: updates unlisten
          ╭▸ 
        4 │ unlisten updates;
          │ ┬────────┯━━━━━━
          │ │        │
          │ │        focus range
          ╰╴full range
        ");
    }

    #[test]
    fn create_function() {
        assert_snapshot!(
            symbols("create function hello() returns void as $$ select 1; $$ language sql;"),
            @r"
        info: function: public.hello
          ╭▸ 
        1 │ create function hello() returns void as $$ select 1; $$ language sql;
          │ ┬───────────────┯━━━━───────────────────────────────────────────────
          │ │               │
          │ │               focus range
          ╰╴full range
        "
        );
    }

    #[test]
    fn create_materialized_view() {
        assert_snapshot!(
            symbols("create materialized view reports as select 1;"),
            @r"
        info: materialized view: public.reports
          ╭▸ 
        1 │ create materialized view reports as select 1;
          │ ┬────────────────────────┯━━━━━━────────────
          │ │                        │
          │ │                        focus range
          ╰╴full range
        "
        );
    }

    #[test]
    fn create_aggregate() {
        assert_snapshot!(
            symbols("create aggregate myavg(int) (sfunc = int4_avg_accum, stype = _int8);"),
            @r"
        info: aggregate: public.myavg
          ╭▸ 
        1 │ create aggregate myavg(int) (sfunc = int4_avg_accum, stype = _int8);
          │ ┬────────────────┯━━━━─────────────────────────────────────────────
          │ │                │
          │ │                focus range
          ╰╴full range
        "
        );
    }

    #[test]
    fn create_procedure() {
        assert_snapshot!(
            symbols("create procedure hello() language sql as $$ select 1; $$;"),
            @r"
        info: procedure: public.hello
          ╭▸ 
        1 │ create procedure hello() language sql as $$ select 1; $$;
          │ ┬────────────────┯━━━━──────────────────────────────────
          │ │                │
          │ │                focus range
          ╰╴full range
        "
        );
    }

    #[test]
    fn create_index() {
        assert_snapshot!(symbols("
create index idx_users_email on users (email);
"), @r"
        info: index: idx_users_email
          ╭▸ 
        2 │ create index idx_users_email on users (email);
          │ ┬────────────┯━━━━━━━━━━━━━━─────────────────
          │ │            │
          │ │            focus range
          ╰╴full range
        ");
    }

    #[test]
    fn create_domain() {
        assert_snapshot!(
            symbols("create domain email_addr as text;"),
            @r"
        info: domain: public.email_addr
          ╭▸ 
        1 │ create domain email_addr as text;
          │ ┬─────────────┯━━━━━━━━━────────
          │ │             │
          │ │             focus range
          ╰╴full range
        "
        );
    }

    #[test]
    fn create_sequence() {
        assert_snapshot!(
            symbols("create sequence user_id_seq;"),
            @r"
        info: sequence: public.user_id_seq
          ╭▸ 
        1 │ create sequence user_id_seq;
          │ ┬───────────────┯━━━━━━━━━━
          │ │               │
          │ │               focus range
          ╰╴full range
        "
        );
    }

    #[test]
    fn create_trigger() {
        assert_snapshot!(symbols("
create trigger update_timestamp
  before update on users
  execute function update_modified_column();
"), @r"
        info: trigger: update_timestamp
          ╭▸ 
        2 │   create trigger update_timestamp
          │   │              ━━━━━━━━━━━━━━━━ focus range
          │ ┌─┘
          │ │
        3 │ │   before update on users
        4 │ │   execute function update_modified_column();
          ╰╴└───────────────────────────────────────────┘ full range
        ");
    }

    #[test]
    fn create_event_trigger() {
        assert_snapshot!(
            symbols("create event trigger et on ddl_command_start execute function f();"),
            @r"
        info: event trigger: et
          ╭▸ 
        1 │ create event trigger et on ddl_command_start execute function f();
          │ ┬────────────────────┯━──────────────────────────────────────────
          │ │                    │
          │ │                    focus range
          ╰╴full range
        "
        );
    }

    #[test]
    fn create_tablespace() {
        assert_snapshot!(symbols("
create tablespace dbspace location '/data/dbs';
"), @r"
        info: tablespace: dbspace
          ╭▸ 
        2 │ create tablespace dbspace location '/data/dbs';
          │ ┬─────────────────┯━━━━━━─────────────────────
          │ │                 │
          │ │                 focus range
          ╰╴full range
        ");
    }

    #[test]
    fn create_database() {
        assert_snapshot!(
            symbols("create database mydb;"),
            @r"
        info: database: mydb
          ╭▸ 
        1 │ create database mydb;
          │ ┬───────────────┯━━━
          │ │               │
          │ │               focus range
          ╰╴full range
        "
        );
    }

    #[test]
    fn create_server() {
        assert_snapshot!(symbols("
create server myserver foreign data wrapper postgres_fdw;
"), @r"
        info: server: myserver
          ╭▸ 
        2 │ create server myserver foreign data wrapper postgres_fdw;
          │ ┬─────────────┯━━━━━━━──────────────────────────────────
          │ │             │
          │ │             focus range
          ╰╴full range
        ");
    }

    #[test]
    fn create_extension() {
        assert_snapshot!(
            symbols("create extension pgcrypto;"),
            @r"
        info: extension: pgcrypto
          ╭▸ 
        1 │ create extension pgcrypto;
          │ ┬────────────────┯━━━━━━━
          │ │                │
          │ │                focus range
          ╰╴full range
        "
        );
    }

    #[test]
    fn create_role() {
        assert_snapshot!(symbols("
create role reader;
"), @r"
        info: role: reader
          ╭▸ 
        2 │ create role reader;
          │ ┬───────────┯━━━━━
          │ │           │
          │ │           focus range
          ╰╴full range
        ");
    }

    #[test]
    fn create_policy() {
        assert_snapshot!(symbols("
create policy allow_read on t;
"), @r"
        info: policy: allow_read
          ╭▸ 
        2 │ create policy allow_read on t;
          │ ┬─────────────┯━━━━━━━━━─────
          │ │             │
          │ │             focus range
          ╰╴full range
        ");
    }

    #[test]
    fn multiple_symbols() {
        assert_snapshot!(symbols("
create table users (id int);
create table posts (id int);
create function get_user(user_id int) returns void as $$ select 1; $$ language sql;
"), @r"
        info: table: public.users
          ╭▸ 
        2 │ create table users (id int);
          │ ┬────────────┯━━━━─────────
          │ │            │
          │ │            focus range
          │ full range
          │
          ⸬  
        2 │ create table users (id int);
          │                     ┯━────
          │                     │
          │                     full range for `column: id int`
          │                     focus range
          ╰╴
        info: table: public.posts
          ╭▸ 
        3 │ create table posts (id int);
          │ ┬────────────┯━━━━─────────
          │ │            │
          │ │            focus range
          │ full range
          │
          ⸬  
        3 │ create table posts (id int);
          │                     ┯━────
          │                     │
          │                     full range for `column: id int`
          ╰╴                    focus range
        info: function: public.get_user
          ╭▸ 
        4 │ create function get_user(user_id int) returns void as $$ select 1; $$ language sql;
          │ ┬───────────────┯━━━━━━━──────────────────────────────────────────────────────────
          │ │               │
          │ │               focus range
          ╰╴full range
        ");
    }

    #[test]
    fn qualified_names() {
        assert_snapshot!(symbols("
create table public.users (id int);
create function my_schema.hello() returns void as $$ select 1; $$ language sql;
"), @r"
        info: table: public.users
          ╭▸ 
        2 │ create table public.users (id int);
          │ ┬───────────────────┯━━━━─────────
          │ │                   │
          │ │                   focus range
          │ full range
          │
          ⸬  
        2 │ create table public.users (id int);
          │                            ┯━────
          │                            │
          │                            full range for `column: id int`
          │                            focus range
          ╰╴
        info: function: my_schema.hello
          ╭▸ 
        3 │ create function my_schema.hello() returns void as $$ select 1; $$ language sql;
          │ ┬─────────────────────────┯━━━━───────────────────────────────────────────────
          │ │                         │
          │ │                         focus range
          ╰╴full range
        ");
    }

    #[test]
    fn create_type() {
        assert_snapshot!(
            symbols("create type status as enum ('active', 'inactive');"),
            @r"
        info: enum: public.status
          ╭▸ 
        1 │ create type status as enum ('active', 'inactive');
          │ ┬───────────┯━━━━━───────────────────────────────
          │ │           │
          │ │           focus range
          │ full range
          │
          ⸬  
        1 │ create type status as enum ('active', 'inactive');
          │                             ┯━━━━━━━  ┯━━━━━━━━━
          │                             │         │
          │                             │         full range for `variant: inactive`
          │                             │         focus range
          │                             full range for `variant: active`
          ╰╴                            focus range
        "
        );
    }

    #[test]
    fn create_type_composite() {
        assert_snapshot!(
            symbols("create type person as (name text, age int);"),
            @r"
        info: type: public.person
          ╭▸ 
        1 │ create type person as (name text, age int);
          │ ┬───────────┯━━━━━────────────────────────
          │ │           │
          │ │           focus range
          │ full range
          │
          ⸬  
        1 │ create type person as (name text, age int);
          │                        ┯━━━─────  ┯━━────
          │                        │          │
          │                        │          full range for `column: age int`
          │                        │          focus range
          │                        full range for `column: name text`
          ╰╴                       focus range
        "
        );
    }

    #[test]
    fn create_type_composite_multiple_columns() {
        assert_snapshot!(
            symbols("create type address as (street text, city text, zip varchar(10));"),
            @r"
        info: type: public.address
          ╭▸ 
        1 │ create type address as (street text, city text, zip varchar(10));
          │ ┬───────────┯━━━━━━─────────────────────────────────────────────
          │ │           │
          │ │           focus range
          │ full range
          │
          ⸬  
        1 │ create type address as (street text, city text, zip varchar(10));
          │                         ┯━━━━━─────  ┯━━━─────  ┯━━────────────
          │                         │            │          │
          │                         │            │          full range for `column: zip varchar(10)`
          │                         │            │          focus range
          │                         │            full range for `column: city text`
          │                         │            focus range
          │                         full range for `column: street text`
          ╰╴                        focus range
        "
        );
    }

    #[test]
    fn create_type_with_schema() {
        assert_snapshot!(
            symbols("create type myschema.status as enum ('active', 'inactive');"),
            @r"
        info: enum: myschema.status
          ╭▸ 
        1 │ create type myschema.status as enum ('active', 'inactive');
          │ ┬────────────────────┯━━━━━───────────────────────────────
          │ │                    │
          │ │                    focus range
          │ full range
          │
          ⸬  
        1 │ create type myschema.status as enum ('active', 'inactive');
          │                                      ┯━━━━━━━  ┯━━━━━━━━━
          │                                      │         │
          │                                      │         full range for `variant: inactive`
          │                                      │         focus range
          │                                      full range for `variant: active`
          ╰╴                                     focus range
        "
        );
    }

    #[test]
    fn create_type_enum_multiple_variants() {
        assert_snapshot!(
            symbols("create type priority as enum ('low', 'medium', 'high', 'urgent');"),
            @r"
        info: enum: public.priority
          ╭▸ 
        1 │ create type priority as enum ('low', 'medium', 'high', 'urgent');
          │ ┬───────────┯━━━━━━━────────────────────────────────────────────
          │ │           │
          │ │           focus range
          │ full range
          │
          ⸬  
        1 │ create type priority as enum ('low', 'medium', 'high', 'urgent');
          │                               ┯━━━━  ┯━━━━━━━  ┯━━━━━  ┯━━━━━━━
          │                               │      │         │       │
          │                               │      │         │       full range for `variant: urgent`
          │                               │      │         │       focus range
          │                               │      │         full range for `variant: high`
          │                               │      │         focus range
          │                               │      full range for `variant: medium`
          │                               │      focus range
          │                               full range for `variant: low`
          ╰╴                              focus range
        "
        );
    }

    #[test]
    fn declare_cursor() {
        assert_snapshot!(symbols("
declare c scroll cursor for select * from t;
"), @r"
        info: cursor: c
          ╭▸ 
        2 │ declare c scroll cursor for select * from t;
          │ ┬───────┯──────────────────────────────────
          │ │       │
          │ │       focus range
          ╰╴full range
        ");
    }

    #[test]
    fn prepare_statement() {
        assert_snapshot!(symbols("
prepare stmt as select 1;
"), @r"
        info: prepared statement: stmt
          ╭▸ 
        2 │ prepare stmt as select 1;
          │ ┬───────┯━━━────────────
          │ │       │
          │ │       focus range
          ╰╴full range
        ");
    }

    #[test]
    fn empty_file() {
        symbols_not_found("")
    }

    #[test]
    fn non_create_statements() {
        symbols_not_found("select * from users;")
    }

    #[test]
    fn cte_table() {
        assert_snapshot!(
            symbols("
with recent_users as (
  select id, email as user_email
  from users
)
select * from recent_users;
"),
            @r"
        info: table: recent_users
          ╭▸ 
        2 │   with recent_users as (
          │        │━━━━━━━━━━━
          │        │
          │ ┌──────focus range
          │ │
        3 │ │   select id, email as user_email
        4 │ │   from users
        5 │ │ )
          ╰╴└─┘ full range
        "
        );
    }

    #[test]
    fn cte_table_with_column_list() {
        assert_snapshot!(
            symbols("
with t(a, b, c) as (
  select 1, 2, 3
)
select * from t;
"),
            @r"
        info: table: t
          ╭▸ 
        2 │   with t(a, b, c) as (
          │        ━ focus range
          │ ┌──────┘
          │ │
        3 │ │   select 1, 2, 3
        4 │ │ )
          │ └─┘ full range
          │
          ⸬  
        2 │   with t(a, b, c) as (
          │          ┯  ┯  ┯
          │          │  │  │
          │          │  │  full range for `column: c`
          │          │  │  focus range
          │          │  full range for `column: b`
          │          │  focus range
          │          full range for `column: a`
          ╰╴         focus range
        "
        );
    }

    #[test]
    fn create_foreign_table() {
        assert_snapshot!(symbols("
create foreign table films (
  code char(5),
  title varchar(40)
) server film_server;
"), @r"
        info: table: public.films
          ╭▸ 
        2 │   create foreign table films (
          │   │                    ━━━━━ focus range
          │ ┌─┘
          │ │
        3 │ │   code char(5),
        4 │ │   title varchar(40)
        5 │ │ ) server film_server;
          │ └────────────────────┘ full range
          │
          ⸬  
        3 │     code char(5),
          │     ┯━━━────────
          │     │
          │     full range for `column: code char(5)`
          │     focus range
        4 │     title varchar(40)
          │     ┯━━━━────────────
          │     │
          │     full range for `column: title varchar(40)`
          ╰╴    focus range
        ");
    }
}
