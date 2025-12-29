use rowan::TextRange;
use squawk_syntax::ast::{self, AstNode};

use crate::binder;
use crate::resolve::{resolve_function_info, resolve_table_info};

pub enum DocumentSymbolKind {
    Table,
    Function,
}

pub struct DocumentSymbol {
    pub name: String,
    pub detail: Option<String>,
    pub kind: DocumentSymbolKind,
    pub range: TextRange,
    pub selection_range: TextRange,
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
            _ => {}
        }
    }

    symbols
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

    let range = create_table.syntax().text_range();
    let selection_range = name_node.syntax().text_range();

    Some(DocumentSymbol {
        name,
        detail: None,
        kind: DocumentSymbolKind::Table,
        range,
        selection_range,
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

    let range = create_function.syntax().text_range();
    let selection_range = name_node.syntax().text_range();

    Some(DocumentSymbol {
        name,
        detail: None,
        kind: DocumentSymbolKind::Function,
        range,
        selection_range,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use annotate_snippets::{AnnotationKind, Level, Renderer, Snippet, renderer::DecorStyle};
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

        let mut groups = vec![];
        for symbol in symbols {
            let kind = match symbol.kind {
                DocumentSymbolKind::Table => "table",
                DocumentSymbolKind::Function => "function",
            };
            let title = format!("{}: {}", kind, symbol.name);
            let group = Level::INFO.primary_title(title).element(
                Snippet::source(sql)
                    .fold(true)
                    .annotation(
                        AnnotationKind::Primary
                            .span(symbol.selection_range.into())
                            .label("name"),
                    )
                    .annotation(
                        AnnotationKind::Context
                            .span(symbol.range.into())
                            .label("select range"),
                    ),
            );
            groups.push(group);
        }

        let renderer = Renderer::plain().decor_style(DecorStyle::Unicode);
        renderer.render(&groups).to_string()
    }

    #[test]
    fn create_table() {
        assert_snapshot!(symbols("create table users (id int);"), @r"
        info: table: public.users
          ╭▸ 
        1 │ create table users (id int);
          │ ┬────────────┯━━━━─────────
          │ │            │
          │ │            name
          ╰╴select range
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
          │ │               name
          ╰╴select range
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
          │ │            name
          │ select range
          ╰╴
        info: table: public.posts
          ╭▸ 
        3 │ create table posts (id int);
          │ ┬────────────┯━━━━─────────
          │ │            │
          │ │            name
          ╰╴select range
        info: function: public.get_user
          ╭▸ 
        4 │ create function get_user(user_id int) returns void as $$ select 1; $$ language sql;
          │ ┬───────────────┯━━━━━━━──────────────────────────────────────────────────────────
          │ │               │
          │ │               name
          ╰╴select range
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
          │ │                   name
          │ select range
          ╰╴
        info: function: my_schema.hello
          ╭▸ 
        3 │ create function my_schema.hello() returns void as $$ select 1; $$ language sql;
          │ ┬─────────────────────────┯━━━━───────────────────────────────────────────────
          │ │                         │
          │ │                         name
          ╰╴select range
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
}
