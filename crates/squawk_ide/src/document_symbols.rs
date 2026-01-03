use rowan::TextRange;
use squawk_syntax::ast::{self, AstNode};

use crate::binder::{self, extract_string_literal};
use crate::resolve::{
    resolve_aggregate_info, resolve_function_info, resolve_materialized_view_info,
    resolve_procedure_info, resolve_table_info, resolve_type_info, resolve_view_info,
};

#[derive(Debug)]
pub enum DocumentSymbolKind {
    Table,
    View,
    MaterializedView,
    Function,
    Aggregate,
    Procedure,
    Type,
    Enum,
    Column,
    Variant,
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
            ast::Stmt::CreateTable(create_table) => {
                if let Some(symbol) = create_table_symbol(&binder, create_table) {
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

    let mut children = vec![];
    if let Some(column_list) = with_table.column_list() {
        for column in column_list.columns() {
            if let Some(column_symbol) = create_column_symbol(column) {
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

fn create_table_symbol(
    binder: &binder::Binder,
    create_table: ast::CreateTable,
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

    let mut children = vec![];
    if let Some(column_list) = create_view.column_list() {
        for column in column_list.columns() {
            if let Some(column_symbol) = create_column_symbol(column) {
                children.push(column_symbol);
            }
        }
    }

    Some(DocumentSymbol {
        name,
        detail: None,
        kind: DocumentSymbolKind::View,
        full_range,
        focus_range,
        children,
    })
}

fn create_materialized_view_symbol(
    binder: &binder::Binder,
    create_view: ast::CreateMaterializedView,
) -> Option<DocumentSymbol> {
    let path = create_view.path()?;
    let segment = path.segment()?;
    let name_node = segment.name()?;

    let (schema, view_name) = resolve_materialized_view_info(binder, &path)?;
    let name = format!("{}.{}", schema.0, view_name);

    let full_range = create_view.syntax().text_range();
    let focus_range = name_node.syntax().text_range();

    let mut children = vec![];
    if let Some(column_list) = create_view.column_list() {
        for column in column_list.columns() {
            if let Some(column_symbol) = create_column_symbol(column) {
                children.push(column_symbol);
            }
        }
    }

    Some(DocumentSymbol {
        name,
        detail: None,
        kind: DocumentSymbolKind::MaterializedView,
        full_range,
        focus_range,
        children,
    })
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
            DocumentSymbolKind::Table => "table",
            DocumentSymbolKind::View => "view",
            DocumentSymbolKind::MaterializedView => "materialized view",
            DocumentSymbolKind::Function => "function",
            DocumentSymbolKind::Aggregate => "aggregate",
            DocumentSymbolKind::Procedure => "procedure",
            DocumentSymbolKind::Type => "type",
            DocumentSymbolKind::Enum => "enum",
            DocumentSymbolKind::Column => "column",
            DocumentSymbolKind::Variant => "variant",
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
}
