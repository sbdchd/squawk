use rowan::TextSize;
use squawk_syntax::ast::{self, AstNode};
use squawk_syntax::{SyntaxKind, SyntaxToken};
use std::collections::HashSet;

use crate::binder;
use crate::resolve;
use crate::symbols::{Name, Schema, SymbolKind};
use crate::tokens::is_string_or_comment;

pub fn completion(file: &ast::SourceFile, offset: TextSize) -> Vec<CompletionItem> {
    let Some(token) = token_at_offset(file, offset) else {
        // empty file
        return default_completions();
    };
    // We don't support completions inside comments since we don't have doc
    // comments a la JSDoc.
    // And we don't have string literal types so we bail out early for strings too.
    if is_string_or_comment(token.kind()) {
        return vec![];
    }

    match completion_context(&token) {
        CompletionContext::TableOnly => table_completions(file, &token),
        CompletionContext::Default => default_completions(),
        CompletionContext::SelectClause(select_clause) => {
            select_completions(file, select_clause, &token)
        }
    }
}

fn select_completions(
    file: &ast::SourceFile,
    select_clause: ast::SelectClause,
    token: &SyntaxToken,
) -> Vec<CompletionItem> {
    let binder = binder::bind(file);
    let mut completions = vec![];
    let schema = schema_qualifier_at_token(token);
    let functions = binder.all_symbols_by_kind(SymbolKind::Function, schema.as_ref());
    completions.extend(functions.into_iter().map(|name| CompletionItem {
        label: format!("{name}()"),
        kind: CompletionItemKind::Function,
        detail: None,
        insert_text: None,
        insert_text_format: None,
        trigger_completion_after_insert: false,
    }));

    let tables = binder.all_symbols_by_kind(SymbolKind::Table, schema.as_ref());
    completions.extend(tables.into_iter().map(|name| CompletionItem {
        label: name.to_string(),
        kind: CompletionItemKind::Table,
        detail: None,
        insert_text: None,
        insert_text_format: None,
        trigger_completion_after_insert: false,
    }));

    if schema.is_none() {
        completions.extend(schema_completions(&binder));
    }

    if let Some(parent) = select_clause.syntax().parent()
        && let Some(select) = ast::Select::cast(parent)
        && let Some(from_clause) = select.from_clause()
    {
        for table_ptr in resolve::table_ptrs_from_clause(&binder, &from_clause) {
            if let Some(create_table) = table_ptr
                .to_node(file.syntax())
                .ancestors()
                .find_map(ast::CreateTableLike::cast)
            {
                let columns = resolve::collect_table_columns(&binder, file.syntax(), &create_table);
                completions.extend(columns.into_iter().filter_map(|column| {
                    let name = column.name()?;
                    Some(CompletionItem {
                        label: crate::symbols::Name::from_node(&name).to_string(),
                        kind: CompletionItemKind::Column,
                        detail: None,
                        insert_text: None,
                        insert_text_format: None,
                        trigger_completion_after_insert: false,
                    })
                }));
            }
        }
    }

    return completions;
}

fn schema_completions(binder: &binder::Binder) -> Vec<CompletionItem> {
    let default_schemas =
        ["public", "pg_temp", "pg_catalog", "pg_toast", "postgres"].map(Name::from_string);
    let mut names = HashSet::from(default_schemas);
    for name in binder.all_symbols_by_kind(SymbolKind::Schema, None) {
        names.insert(name.clone());
    }

    names
        .into_iter()
        .map(|name| CompletionItem {
            label: name.to_string(),
            kind: CompletionItemKind::Schema,
            detail: None,
            insert_text: None,
            insert_text_format: None,
            trigger_completion_after_insert: false,
        })
        .collect()
}

fn table_completions(file: &ast::SourceFile, token: &SyntaxToken) -> Vec<CompletionItem> {
    let binder = binder::bind(file);
    // We're in a TRUNCATE or TABLE statement, return table names
    let tables = binder.all_symbols_by_kind(SymbolKind::Table, None);
    let mut completions: Vec<CompletionItem> = tables
        .into_iter()
        .map(|name| CompletionItem {
            label: name.to_string(),
            kind: CompletionItemKind::Table,
            detail: None,
            insert_text: None,
            insert_text_format: None,
            trigger_completion_after_insert: false,
        })
        .collect();

    if schema_qualifier_at_token(token).is_none() {
        completions.extend(schema_completions(&binder));
    }

    completions
}

enum CompletionContext {
    TableOnly,
    Default,
    SelectClause(ast::SelectClause),
}

fn completion_context(token: &SyntaxToken) -> CompletionContext {
    if let Some(node) = token.parent() {
        for a in node.ancestors() {
            if ast::Truncate::can_cast(a.kind()) || ast::Table::can_cast(a.kind()) {
                return CompletionContext::TableOnly;
            }
            if let Some(select_clause) = ast::SelectClause::cast(a.clone()) {
                return CompletionContext::SelectClause(select_clause);
            }
        }
    }
    CompletionContext::Default
}

fn token_at_offset(file: &ast::SourceFile, offset: TextSize) -> Option<SyntaxToken> {
    let Some(mut token) = file.syntax().token_at_offset(offset).left_biased() else {
        // empty file - definitely at top level
        return None;
    };
    while token.kind() == SyntaxKind::WHITESPACE {
        if let Some(tk) = token.prev_token() {
            token = tk;
        }
    }
    Some(token)
}

fn schema_qualifier_at_token(token: &SyntaxToken) -> Option<Schema> {
    let schema_token = if token.kind() == SyntaxKind::DOT {
        token.prev_token()
    } else if token.kind() == SyntaxKind::IDENT
        && let Some(prev) = token.prev_token()
        && prev.kind() == SyntaxKind::DOT
    {
        prev.prev_token()
    } else {
        None
    };

    schema_token
        .filter(|tk| tk.kind() == SyntaxKind::IDENT)
        .map(|tk| Schema(Name::from_string(tk.text().to_string())))
}

fn default_completions() -> Vec<CompletionItem> {
    ["select", "table", "truncate"]
        .map(|stmt| CompletionItem {
            label: stmt.to_owned(),
            kind: CompletionItemKind::Keyword,
            detail: None,
            insert_text: Some(format!("{stmt} $0;")),
            insert_text_format: Some(CompletionInsertTextFormat::Snippet),
            trigger_completion_after_insert: true,
        })
        .into_iter()
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompletionItemKind {
    Keyword,
    Table,
    Column,
    Function,
    Schema,
    Type,
    Snippet,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompletionInsertTextFormat {
    PlainText,
    Snippet,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletionItem {
    pub label: String,
    pub kind: CompletionItemKind,
    pub detail: Option<String>,
    pub insert_text: Option<String>,
    pub insert_text_format: Option<CompletionInsertTextFormat>,
    pub trigger_completion_after_insert: bool,
}

#[cfg(test)]
mod tests {
    use super::completion;
    use crate::test_utils::fixture;
    use insta::assert_snapshot;
    use squawk_syntax::ast;
    use tabled::builder::Builder;
    use tabled::settings::Style;

    fn completions(sql: &str) -> String {
        let (offset, sql) = fixture(sql);
        let parse = ast::SourceFile::parse(&sql);
        let file = parse.tree();
        let items = completion(&file, offset);
        assert!(
            !items.is_empty(),
            "No completions found. If this was intended, use `completions_not_found` instead."
        );
        format_items(items)
    }

    fn completions_not_found(sql: &str) {
        let (offset, sql) = fixture(sql);
        let parse = ast::SourceFile::parse(&sql);
        let file = parse.tree();
        let items = completion(&file, offset);
        assert_eq!(
            items,
            vec![],
            "Completions found. If this was unintended, use `completions` instead."
        )
    }

    fn format_items(items: Vec<super::CompletionItem>) -> String {
        let mut rows: Vec<Vec<String>> = items
            .into_iter()
            .map(|item| {
                vec![
                    item.label,
                    format!("{:?}", item.kind),
                    item.detail.unwrap_or_default(),
                    item.insert_text.unwrap_or_default(),
                ]
            })
            .collect();

        rows.sort();

        let mut builder = Builder::default();
        builder.push_record(["label", "kind", "detail", "insert_text"]);
        for row in rows {
            builder.push_record(row);
        }

        let mut table = builder.build();
        table.with(Style::psql());
        table.to_string()
    }

    #[test]
    fn completion_at_start() {
        assert_snapshot!(completions("$0"), @r"
         label    | kind    | detail | insert_text  
        ----------+---------+--------+--------------
         select   | Keyword |        | select $0;   
         table    | Keyword |        | table $0;    
         truncate | Keyword |        | truncate $0;
        ");
    }

    #[test]
    fn completion_at_top_level() {
        assert_snapshot!(completions("
create table t(a int);
$0
"), @r"
         label    | kind    | detail | insert_text  
        ----------+---------+--------+--------------
         select   | Keyword |        | select $0;   
         table    | Keyword |        | table $0;    
         truncate | Keyword |        | truncate $0;
        ");
    }

    #[test]
    fn completion_in_string() {
        completions_not_found("select '$0';");
    }

    #[test]
    fn completion_in_comment() {
        completions_not_found("-- $0 ");
    }

    #[test]
    fn completion_after_truncate() {
        assert_snapshot!(completions("
create table users (id int);
truncate $0;
"), @r"
         label      | kind   | detail | insert_text 
        ------------+--------+--------+-------------
         pg_catalog | Schema |        |             
         pg_temp    | Schema |        |             
         pg_toast   | Schema |        |             
         postgres   | Schema |        |             
         public     | Schema |        |             
         users      | Table  |        |
        ");
    }

    #[test]
    fn completion_table_at_top_level() {
        assert_snapshot!(completions("$0"), @r"
         label    | kind    | detail | insert_text  
        ----------+---------+--------+--------------
         select   | Keyword |        | select $0;   
         table    | Keyword |        | table $0;    
         truncate | Keyword |        | truncate $0;
        ");
    }

    #[test]
    fn completion_table_nested() {
        assert_snapshot!(completions("select * from ($0)"), @r"
         label    | kind    | detail | insert_text  
        ----------+---------+--------+--------------
         select   | Keyword |        | select $0;   
         table    | Keyword |        | table $0;    
         truncate | Keyword |        | truncate $0;
        ");
    }

    #[test]
    fn completion_after_table() {
        assert_snapshot!(completions("
create table users (id int);
table $0;
"), @r"
         label      | kind   | detail | insert_text 
        ------------+--------+--------+-------------
         pg_catalog | Schema |        |             
         pg_temp    | Schema |        |             
         pg_toast   | Schema |        |             
         postgres   | Schema |        |             
         public     | Schema |        |             
         users      | Table  |        |
        ");
    }

    #[test]
    fn completion_after_select() {
        assert_snapshot!(completions("
create table t(a text, b int);
create function f() returns text as 'select 1::text' language sql;
select $0 from t;
"), @r"
         label      | kind     | detail | insert_text 
        ------------+----------+--------+-------------
         a          | Column   |        |             
         b          | Column   |        |             
         f()        | Function |        |             
         pg_catalog | Schema   |        |             
         pg_temp    | Schema   |        |             
         pg_toast   | Schema   |        |             
         postgres   | Schema   |        |             
         public     | Schema   |        |             
         t          | Table    |        |
        ");
    }

    #[test]
    fn completion_with_schema_qualifier() {
        assert_snapshot!(completions("
create function f() returns int8 as 'select 1' language sql;
create function foo.b() returns int8 as 'select 2' language sql;
select public.$0;
"), @r"
         label | kind     | detail | insert_text 
        -------+----------+--------+-------------
         f()   | Function |        |
        ");
    }
}
