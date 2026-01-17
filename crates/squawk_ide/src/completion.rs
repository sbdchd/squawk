use rowan::TextSize;
use squawk_syntax::ast::{self, AstNode};
use squawk_syntax::{SyntaxKind, SyntaxToken};

use crate::binder;
use crate::resolve;
use crate::symbols::SymbolKind;
use crate::tokens::is_string_or_comment;

pub fn completion(file: &ast::SourceFile, offset: TextSize) -> Vec<CompletionItem> {
    let Some(token) = token_at_offset(file, offset) else {
        // empty file
        return default_completions(true);
    };
    // We don't support completions inside comments since we don't have doc
    // comments a la JSDoc.
    // And we don't have string literal types so we bail out early for strings too.
    if is_string_or_comment(token.kind()) {
        return vec![];
    }

    let binder = binder::bind(file);
    match completion_context(token) {
        CompletionContext::TableOnly => table_completions(&binder),
        CompletionContext::Default(is_nested) => default_completions(!is_nested),
        CompletionContext::SelectClause(select_clause) => {
            select_completions(binder, file, select_clause)
        }
    }
}

fn select_completions(
    binder: binder::Binder,
    file: &ast::SourceFile,
    select_clause: ast::SelectClause,
) -> Vec<CompletionItem> {
    let mut completions = vec![];
    let functions = binder.all_symbols_by_kind(SymbolKind::Function);
    completions.extend(functions.into_iter().map(|name| CompletionItem {
        label: name.to_string(),
        kind: CompletionItemKind::Function,
        detail: None,
        insert_text: None,
        insert_text_format: None,
        trigger_completion_after_insert: false,
    }));

    let tables = binder.all_symbols_by_kind(SymbolKind::Table);
    completions.extend(tables.into_iter().map(|name| CompletionItem {
        label: name.to_string(),
        kind: CompletionItemKind::Table,
        detail: None,
        insert_text: None,
        insert_text_format: None,
        trigger_completion_after_insert: false,
    }));

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

fn table_completions(binder: &binder::Binder) -> Vec<CompletionItem> {
    // We're in a TRUNCATE or TABLE statement, return table names
    let tables = binder.all_symbols_by_kind(SymbolKind::Table);
    tables
        .into_iter()
        .map(|name| CompletionItem {
            label: name.to_string(),
            kind: CompletionItemKind::Table,
            detail: None,
            insert_text: None,
            insert_text_format: None,
            trigger_completion_after_insert: false,
        })
        .collect()
}

enum CompletionContext {
    TableOnly,
    Default(bool),
    SelectClause(ast::SelectClause),
}

fn completion_context(token: SyntaxToken) -> CompletionContext {
    let mut node = token.parent();
    let mut is_nested = false;
    let mut kind = None;
    while let Some(current_node) = node {
        if ast::Stmt::can_cast(current_node.kind())
            && current_node
                .parent()
                .is_some_and(|x| x.kind() == SyntaxKind::SOURCE_FILE)
        {
            is_nested = true;
        }
        if ast::Truncate::can_cast(current_node.kind()) || ast::Table::can_cast(current_node.kind())
        {
            if kind.is_none() {
                kind = Some(CompletionContext::TableOnly)
            };
        }
        if let Some(select_clause) = ast::SelectClause::cast(current_node.clone()) {
            if kind.is_none() {
                kind = Some(CompletionContext::SelectClause(select_clause))
            };
        }
        node = current_node.parent();
    }
    kind.unwrap_or_else(|| CompletionContext::Default(is_nested))
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

fn default_completions(at_top_level: bool) -> Vec<CompletionItem> {
    let select_insert_text = if at_top_level {
        "select $0;"
    } else {
        "select $0"
    };

    let table_insert_text = if at_top_level {
        "table $0;"
    } else {
        "table $0"
    };

    let mut completions = vec![
        CompletionItem {
            label: "select".to_owned(),
            kind: CompletionItemKind::Keyword,
            detail: None,
            insert_text: Some(select_insert_text.to_owned()),
            insert_text_format: Some(CompletionInsertTextFormat::Snippet),
            trigger_completion_after_insert: false,
        },
        CompletionItem {
            label: "table".to_owned(),
            kind: CompletionItemKind::Keyword,
            detail: None,
            insert_text: Some(table_insert_text.to_owned()),
            insert_text_format: Some(CompletionInsertTextFormat::Snippet),
            trigger_completion_after_insert: true,
        },
    ];

    if at_top_level {
        completions.push(CompletionItem {
            label: "truncate".to_owned(),
            kind: CompletionItemKind::Keyword,
            detail: None,
            insert_text: Some("truncate $0;".to_owned()),
            insert_text_format: Some(CompletionInsertTextFormat::Snippet),
            trigger_completion_after_insert: true,
        });
    }

    completions
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
         label | kind  | detail | insert_text 
        -------+-------+--------+-------------
         users | Table |        |
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
         label  | kind    | detail | insert_text 
        --------+---------+--------+-------------
         select | Keyword |        | select $0   
         table  | Keyword |        | table $0
        ");
    }

    #[test]
    fn completion_after_table() {
        assert_snapshot!(completions("
create table users (id int);
table $0;
"), @r"
         label | kind  | detail | insert_text 
        -------+-------+--------+-------------
         users | Table |        |
        ");
    }

    #[test]
    fn completion_after_select() {
        assert_snapshot!(completions("
create table t(a text, b int);
create function f() returns text as 'select 1::text' language sql;
select $0 from t;
"), @r"
         label | kind     | detail | insert_text 
        -------+----------+--------+-------------
         a     | Column   |        |             
         b     | Column   |        |             
         f     | Function |        |             
         t     | Table    |        |
        ");
    }
}
