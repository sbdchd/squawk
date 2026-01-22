use rowan::TextSize;
use squawk_syntax::ast::{self, AstNode};
use squawk_syntax::{SyntaxKind, SyntaxToken};

use crate::binder;
use crate::resolve;
use crate::symbols::{Name, Schema, SymbolKind};
use crate::tokens::is_string_or_comment;

const COMPLETION_MARKER: &str = "squawkCompletionMarker";

pub fn completion(file: &ast::SourceFile, offset: TextSize) -> Vec<CompletionItem> {
    let file = file_with_completion_marker(file, offset);
    let Some(token) = token_at_offset(&file, offset) else {
        // empty file
        return default_completions();
    };
    // We don't support completions inside comments since we don't have doc
    // comments a la JSDoc.
    // And we don't support enums aka string literal types yet so we bail out
    // early for strings as well
    if is_string_or_comment(token.kind()) {
        return vec![];
    }

    match completion_context(&token) {
        CompletionContext::TableOnly => table_completions(&file, &token),
        CompletionContext::Default => default_completions(),
        CompletionContext::SelectClause(select_clause) => {
            select_completions(&file, select_clause, &token)
        }
        CompletionContext::DeleteClauses(delete) => {
            delete_clauses_completions(&file, &delete, &token)
        }
        CompletionContext::DeleteExpr(delete) => delete_expr_completions(&file, &delete, &token),
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
    let position = token.text_range().start();
    completions.extend(functions.into_iter().map(|name| CompletionItem {
        label: format!("{name}()"),
        kind: CompletionItemKind::Function,
        detail: function_detail(&binder, file, name, &schema, position),
        insert_text: None,
        insert_text_format: None,
        trigger_completion_after_insert: false,
        sort_text: None,
    }));

    let tables = binder.all_symbols_by_kind(SymbolKind::Table, schema.as_ref());
    completions.extend(tables.into_iter().map(|name| CompletionItem {
        label: name.to_string(),
        kind: CompletionItemKind::Table,
        detail: None,
        insert_text: None,
        insert_text_format: None,
        trigger_completion_after_insert: false,
        sort_text: None,
    }));

    if schema.is_none() {
        completions.extend(schema_completions(&binder));
    }

    if let Some(parent) = select_clause.syntax().parent()
        && let Some(select) = ast::Select::cast(parent)
        && let Some(from_clause) = select.from_clause()
    {
        for table_ptr in resolve::table_ptrs_from_clause(&binder, &from_clause) {
            let table_node = table_ptr.to_node(file.syntax());
            match resolve::find_table_source(&table_node) {
                Some(resolve::TableSource::CreateTable(create_table)) => {
                    let columns =
                        resolve::collect_table_columns(&binder, file.syntax(), &create_table);
                    completions.extend(columns.into_iter().filter_map(|column| {
                        let name = column.name()?;
                        let detail = column.ty().map(|t| t.syntax().text().to_string());
                        Some(CompletionItem {
                            label: Name::from_node(&name).to_string(),
                            kind: CompletionItemKind::Column,
                            detail,
                            insert_text: None,
                            insert_text_format: None,
                            trigger_completion_after_insert: false,
                            sort_text: None,
                        })
                    }));
                }
                Some(resolve::TableSource::WithTable(with_table)) => {
                    let columns = resolve::collect_with_table_columns_with_types(&with_table);
                    completions.extend(columns.into_iter().map(|(name, ty)| CompletionItem {
                        label: name.to_string(),
                        kind: CompletionItemKind::Column,
                        detail: ty.map(|t| t.to_string()),
                        insert_text: None,
                        insert_text_format: None,
                        trigger_completion_after_insert: false,
                        sort_text: None,
                    }));
                }
                Some(resolve::TableSource::CreateView(create_view)) => {
                    let columns = resolve::collect_view_columns_with_types(&create_view);
                    completions.extend(columns.into_iter().map(|(name, ty)| CompletionItem {
                        label: name.to_string(),
                        kind: CompletionItemKind::Column,
                        detail: ty.map(|t| t.to_string()),
                        insert_text: None,
                        insert_text_format: None,
                        trigger_completion_after_insert: false,
                        sort_text: None,
                    }));
                }
                Some(resolve::TableSource::CreateMaterializedView(create_materialized_view)) => {
                    let columns = resolve::collect_materialized_view_columns_with_types(
                        &create_materialized_view,
                    );
                    completions.extend(columns.into_iter().map(|(name, ty)| CompletionItem {
                        label: name.to_string(),
                        kind: CompletionItemKind::Column,
                        detail: ty.map(|t| t.to_string()),
                        insert_text: None,
                        insert_text_format: None,
                        trigger_completion_after_insert: false,
                        sort_text: None,
                    }));
                }
                Some(resolve::TableSource::ParenSelect(paren_select)) => {
                    let columns = resolve::collect_paren_select_columns_with_types(
                        &binder,
                        file.syntax(),
                        &paren_select,
                    );
                    completions.extend(columns.into_iter().map(|(name, ty)| CompletionItem {
                        label: name.to_string(),
                        kind: CompletionItemKind::Column,
                        detail: ty.map(|t| t.to_string()),
                        insert_text: None,
                        insert_text_format: None,
                        trigger_completion_after_insert: false,
                        sort_text: None,
                    }));
                }
                None => {}
            }
        }
    }

    return completions;
}

fn schema_completions(binder: &binder::Binder) -> Vec<CompletionItem> {
    let builtin_schemas = [
        "public",
        "pg_catalog",
        "pg_temp",
        "pg_toast",
        "information_schema",
    ];
    let mut completions: Vec<CompletionItem> = builtin_schemas
        .into_iter()
        .enumerate()
        .map(|(i, name)| CompletionItem {
            label: name.to_string(),
            kind: CompletionItemKind::Schema,
            detail: None,
            insert_text: None,
            insert_text_format: None,
            trigger_completion_after_insert: false,
            sort_text: Some(format!("{i}")),
        })
        .collect();

    for name in binder.all_symbols_by_kind(SymbolKind::Schema, None) {
        completions.push(CompletionItem {
            label: name.to_string(),
            kind: CompletionItemKind::Schema,
            detail: None,
            insert_text: None,
            insert_text_format: None,
            trigger_completion_after_insert: false,
            sort_text: None,
        });
    }

    completions
}

fn table_completions(file: &ast::SourceFile, token: &SyntaxToken) -> Vec<CompletionItem> {
    let binder = binder::bind(file);
    let schema = schema_qualifier_at_token(token);
    let tables = binder.all_symbols_by_kind(SymbolKind::Table, schema.as_ref());
    let mut completions: Vec<CompletionItem> = tables
        .into_iter()
        .map(|name| CompletionItem {
            label: name.to_string(),
            kind: CompletionItemKind::Table,
            detail: None,
            insert_text: None,
            insert_text_format: None,
            trigger_completion_after_insert: false,
            sort_text: None,
        })
        .collect();

    if schema.is_none() {
        completions.extend(schema_completions(&binder));
    }

    completions
}

fn delete_clauses_completions(
    file: &ast::SourceFile,
    delete: &ast::Delete,
    token: &SyntaxToken,
) -> Vec<CompletionItem> {
    let mut completions = vec![];

    // `delete from $0`
    if token.kind() == SyntaxKind::FROM_KW {
        return table_completions(file, token);
    }

    if delete.using_clause().is_none() {
        completions.push(CompletionItem {
            label: "using".to_owned(),
            kind: CompletionItemKind::Keyword,
            detail: None,
            insert_text: Some("using $0".to_owned()),
            insert_text_format: Some(CompletionInsertTextFormat::Snippet),
            trigger_completion_after_insert: true,
            sort_text: None,
        });
    }

    if delete.where_clause().is_none() {
        completions.push(CompletionItem {
            label: "where".to_owned(),
            kind: CompletionItemKind::Keyword,
            detail: None,
            insert_text: Some("where $0".to_owned()),
            insert_text_format: Some(CompletionInsertTextFormat::Snippet),
            trigger_completion_after_insert: true,
            sort_text: None,
        });
    }

    if delete.returning_clause().is_none() {
        completions.push(CompletionItem {
            label: "returning".to_owned(),
            kind: CompletionItemKind::Keyword,
            detail: None,
            insert_text: Some("returning $0".to_owned()),
            insert_text_format: Some(CompletionInsertTextFormat::Snippet),
            trigger_completion_after_insert: true,
            sort_text: None,
        });
    }

    completions
}

fn delete_expr_completions(
    file: &ast::SourceFile,
    delete: &ast::Delete,
    token: &SyntaxToken,
) -> Vec<CompletionItem> {
    let binder = binder::bind(file);
    let mut completions = vec![];

    let Some(path) = delete.relation_name().and_then(|r| r.path()) else {
        return completions;
    };

    let Some(delete_table_name) = resolve::extract_table_name(&path) else {
        return completions;
    };

    let has_table_qualifier = qualifier_at_token(token).is_some_and(|q| q == delete_table_name);
    let schema = schema_qualifier_at_token(token);
    let position = token.text_range().start();

    if has_table_qualifier {
        let functions = binder.functions_with_single_param(&delete_table_name);
        completions.extend(functions.into_iter().map(|name| CompletionItem {
            label: name.to_string(),
            kind: CompletionItemKind::Function,
            detail: function_detail(&binder, file, name, &schema, position),
            insert_text: None,
            insert_text_format: None,
            trigger_completion_after_insert: false,
            sort_text: None,
        }));
    } else {
        let functions = binder.all_symbols_by_kind(SymbolKind::Function, None);
        completions.extend(functions.into_iter().map(|name| CompletionItem {
            label: format!("{name}()"),
            kind: CompletionItemKind::Function,
            detail: function_detail(&binder, file, name, &schema, position),
            insert_text: None,
            insert_text_format: None,
            trigger_completion_after_insert: false,
            sort_text: None,
        }));

        completions.push(CompletionItem {
            label: delete_table_name.to_string(),
            kind: CompletionItemKind::Table,
            detail: None,
            insert_text: None,
            insert_text_format: None,
            trigger_completion_after_insert: false,
            sort_text: None,
        });
    }

    let schema = resolve::extract_schema_name(&path);
    if let Some(table_ptr) =
        binder.lookup_with(&delete_table_name, SymbolKind::Table, position, &schema)
        && let Some(create_table) = table_ptr
            .to_node(file.syntax())
            .ancestors()
            .find_map(ast::CreateTableLike::cast)
    {
        let columns = resolve::collect_table_columns(&binder, file.syntax(), &create_table);
        completions.extend(columns.into_iter().filter_map(|column| {
            let name = column.name()?;
            let detail = column.ty().map(|t| t.syntax().text().to_string());
            Some(CompletionItem {
                label: Name::from_node(&name).to_string(),
                kind: CompletionItemKind::Column,
                detail,
                insert_text: None,
                insert_text_format: None,
                trigger_completion_after_insert: false,
                sort_text: None,
            })
        }));
    }

    completions
}

fn qualifier_at_token(token: &SyntaxToken) -> Option<Name> {
    let qualifier_token = if token.kind() == SyntaxKind::DOT {
        token.prev_token()
    } else if token.kind() == SyntaxKind::IDENT
        && let Some(prev) = token.prev_token()
        && prev.kind() == SyntaxKind::DOT
    {
        prev.prev_token()
    } else {
        None
    };

    qualifier_token
        .filter(|tk| tk.kind() == SyntaxKind::IDENT)
        .map(|tk| Name::from_string(tk.text().to_string()))
}

#[derive(Debug)]
enum CompletionContext {
    TableOnly,
    Default,
    SelectClause(ast::SelectClause),
    DeleteClauses(ast::Delete),
    DeleteExpr(ast::Delete),
}

fn completion_context(token: &SyntaxToken) -> CompletionContext {
    if let Some(node) = token.parent() {
        let mut inside_delete_clause = false;
        for a in node.ancestors() {
            if ast::Truncate::can_cast(a.kind()) || ast::Table::can_cast(a.kind()) {
                return CompletionContext::TableOnly;
            }
            if ast::WhereClause::can_cast(a.kind())
                || ast::UsingClause::can_cast(a.kind())
                || ast::ReturningClause::can_cast(a.kind())
            {
                inside_delete_clause = true;
            }
            if let Some(delete) = ast::Delete::cast(a.clone()) {
                if inside_delete_clause {
                    return CompletionContext::DeleteExpr(delete);
                }
                if delete.relation_name().is_some() {
                    return CompletionContext::DeleteClauses(delete);
                }
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

// In order to make completions, we do something similar to rust analyzer by
// inserting an ident to make the parse tree parse in more cases.
// Rust analyzer does fancier things for this, which we can investigate later.
//
// This helps us support `select t. from t`, which parses as `select t.from t`.
// If we insert the ident we get, `select t.c from t`.
fn file_with_completion_marker(file: &ast::SourceFile, offset: TextSize) -> ast::SourceFile {
    let mut sql = file.syntax().text().to_string();
    let offset = u32::from(offset) as usize;
    let offset = offset.min(sql.len());
    sql.insert_str(offset, COMPLETION_MARKER);
    ast::SourceFile::parse(&sql).tree()
}

fn schema_qualifier_at_token(token: &SyntaxToken) -> Option<Schema> {
    qualifier_at_token(token).map(Schema)
}

fn function_detail(
    binder: &binder::Binder,
    file: &ast::SourceFile,
    function_name: &Name,
    schema: &Option<Schema>,
    position: TextSize,
) -> Option<String> {
    let create_function = binder
        .lookup_with(function_name, SymbolKind::Function, position, schema)?
        .to_node(file.syntax())
        .ancestors()
        .find_map(ast::CreateFunction::cast)?;
    let path = create_function.path()?;
    let (schema, function_name) = resolve::resolve_function_info(binder, &path)?;

    let param_list = create_function.param_list()?;
    let params = param_list.syntax().text().to_string();

    let ret_type = create_function.ret_type()?;
    let return_type = ret_type.syntax().text().to_string();

    Some(format!(
        "{}.{}{} {}",
        schema, function_name, params, return_type
    ))
}

fn default_completions() -> Vec<CompletionItem> {
    ["delete from", "select", "table", "truncate"]
        .map(|stmt| CompletionItem {
            label: stmt.to_owned(),
            kind: CompletionItemKind::Keyword,
            detail: None,
            insert_text: Some(format!("{stmt} $0;")),
            insert_text_format: Some(CompletionInsertTextFormat::Snippet),
            trigger_completion_after_insert: true,
            sort_text: None,
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

impl CompletionItemKind {
    fn sort_prefix(self) -> &'static str {
        match self {
            Self::Column => "0",
            Self::Keyword => "1",
            Self::Table => "1",
            Self::Type => "1",
            Self::Snippet => "1",
            Self::Function => "2",
            Self::Schema => "9",
        }
    }
}

impl CompletionItem {
    pub fn sort_text(&self) -> String {
        let prefix = self.kind.sort_prefix();
        let suffix = self.sort_text.as_ref().unwrap_or(&self.label);
        format!("{prefix}_{suffix}")
    }
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
    pub sort_text: Option<String>,
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

    fn format_items(mut items: Vec<super::CompletionItem>) -> String {
        items.sort_by_key(|a| a.sort_text());

        let rows: Vec<Vec<String>> = items
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
         label       | kind    | detail | insert_text     
        -------------+---------+--------+-----------------
         delete from | Keyword |        | delete from $0; 
         select      | Keyword |        | select $0;      
         table       | Keyword |        | table $0;       
         truncate    | Keyword |        | truncate $0;
        ");
    }

    #[test]
    fn completion_at_top_level() {
        assert_snapshot!(completions("
create table t(a int);
$0
"), @r"
         label       | kind    | detail | insert_text     
        -------------+---------+--------+-----------------
         delete from | Keyword |        | delete from $0; 
         select      | Keyword |        | select $0;      
         table       | Keyword |        | table $0;       
         truncate    | Keyword |        | truncate $0;
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
         label              | kind   | detail | insert_text 
        --------------------+--------+--------+-------------
         users              | Table  |        |             
         public             | Schema |        |             
         pg_catalog         | Schema |        |             
         pg_temp            | Schema |        |             
         pg_toast           | Schema |        |             
         information_schema | Schema |        |
        ");
    }

    #[test]
    fn completion_table_at_top_level() {
        assert_snapshot!(completions("$0"), @r"
         label       | kind    | detail | insert_text     
        -------------+---------+--------+-----------------
         delete from | Keyword |        | delete from $0; 
         select      | Keyword |        | select $0;      
         table       | Keyword |        | table $0;       
         truncate    | Keyword |        | truncate $0;
        ");
    }

    #[test]
    fn completion_table_nested() {
        assert_snapshot!(completions("select * from ($0)"), @r"
         label       | kind    | detail | insert_text     
        -------------+---------+--------+-----------------
         delete from | Keyword |        | delete from $0; 
         select      | Keyword |        | select $0;      
         table       | Keyword |        | table $0;       
         truncate    | Keyword |        | truncate $0;
        ");
    }

    #[test]
    fn completion_after_table() {
        assert_snapshot!(completions("
create table users (id int);
table $0;
"), @r"
         label              | kind   | detail | insert_text 
        --------------------+--------+--------+-------------
         users              | Table  |        |             
         public             | Schema |        |             
         pg_catalog         | Schema |        |             
         pg_temp            | Schema |        |             
         pg_toast           | Schema |        |             
         information_schema | Schema |        |
        ");
    }

    #[test]
    fn completion_after_select() {
        assert_snapshot!(completions("
create table t(a text, b int);
create function f() returns text as 'select 1::text' language sql;
select $0 from t;
"), @r"
         label              | kind     | detail                  | insert_text 
        --------------------+----------+-------------------------+-------------
         a                  | Column   | text                    |             
         b                  | Column   | int                     |             
         t                  | Table    |                         |             
         f()                | Function | public.f() returns text |             
         public             | Schema   |                         |             
         pg_catalog         | Schema   |                         |             
         pg_temp            | Schema   |                         |             
         pg_toast           | Schema   |                         |             
         information_schema | Schema   |                         |
        ");
    }

    #[test]
    fn completion_select_table_qualified() {
        assert_snapshot!(completions("
create table t (c int);
select t.$0 from t;
"), @r"
         label | kind   | detail | insert_text 
        -------+--------+--------+-------------
         c     | Column | int    |             
        ");
    }

    #[test]
    fn completion_after_select_with_cte() {
        assert_snapshot!(completions("
with t as (select 1 a)
select $0 from t;
"), @r"
         label              | kind   | detail  | insert_text 
        --------------------+--------+---------+-------------
         a                  | Column | integer |             
         public             | Schema |         |             
         pg_catalog         | Schema |         |             
         pg_temp            | Schema |         |             
         pg_toast           | Schema |         |             
         information_schema | Schema |         |
        ");
    }

    #[test]
    fn completion_values_cte() {
        assert_snapshot!(completions("
with t as (values (1, 'foo', false))
select $0 from t;
"), @r"
         label              | kind   | detail  | insert_text 
        --------------------+--------+---------+-------------
         column1            | Column | integer |             
         column2            | Column | text    |             
         column3            | Column | boolean |             
         public             | Schema |         |             
         pg_catalog         | Schema |         |             
         pg_temp            | Schema |         |             
         pg_toast           | Schema |         |             
         information_schema | Schema |         |
        ");
    }

    #[test]
    fn completion_values_subquery() {
        assert_snapshot!(completions("
select $0 from (values (1, 'foo', 1.5, false));
"), @r"
         label              | kind   | detail  | insert_text 
        --------------------+--------+---------+-------------
         column1            | Column | integer |             
         column2            | Column | text    |             
         column3            | Column | numeric |             
         column4            | Column | boolean |             
         public             | Schema |         |             
         pg_catalog         | Schema |         |             
         pg_temp            | Schema |         |             
         pg_toast           | Schema |         |             
         information_schema | Schema |         |
        ");
    }

    #[test]
    fn completion_with_schema_qualifier() {
        assert_snapshot!(completions("
create function f() returns int8 as 'select 1' language sql;
create function foo.b() returns int8 as 'select 2' language sql;
select public.$0;
"), @r"
         label | kind     | detail                  | insert_text 
        -------+----------+-------------------------+-------------
         f()   | Function | public.f() returns int8 |
        ");
    }

    #[test]
    fn completion_truncate_with_schema_qualifier() {
        assert_snapshot!(completions("
create table users (id int);
truncate public.$0;
"), @r"
         label | kind  | detail | insert_text 
        -------+-------+--------+-------------
         users | Table |        |
        ");
    }

    #[test]
    fn completion_after_delete_from() {
        assert_snapshot!(completions("
create table users (id int);
delete from $0;
"), @r"
         label              | kind   | detail | insert_text 
        --------------------+--------+--------+-------------
         users              | Table  |        |             
         public             | Schema |        |             
         pg_catalog         | Schema |        |             
         pg_temp            | Schema |        |             
         pg_toast           | Schema |        |             
         information_schema | Schema |        |
        ");
    }

    #[test]
    fn completion_delete_clauses() {
        assert_snapshot!(completions("
create table t (id int);
delete from t $0;
"), @r"
         label     | kind    | detail | insert_text  
        -----------+---------+--------+--------------
         returning | Keyword |        | returning $0 
         using     | Keyword |        | using $0     
         where     | Keyword |        | where $0
        ");
    }

    #[test]
    fn completion_delete_where_expr() {
        assert_snapshot!(completions("
create table t (id int, name text);
create function is_active() returns bool as 'select true' language sql;
delete from t where $0;
"), @r"
         label       | kind     | detail                          | insert_text 
        -------------+----------+---------------------------------+-------------
         id          | Column   | int                             |             
         name        | Column   | text                            |             
         t           | Table    |                                 |             
         is_active() | Function | public.is_active() returns bool |
        ")
    }

    #[test]
    fn completion_delete_returning_expr() {
        assert_snapshot!(completions("
create table t (id int, name text);
delete from t returning $0;
"), @r"
         label | kind   | detail | insert_text 
        -------+--------+--------+-------------
         id    | Column | int    |             
         name  | Column | text   |             
         t     | Table  |        |
        ");
    }

    #[test]
    fn completion_delete_where_qualified() {
        assert_snapshot!(completions("
-- different type than the table, so we shouldn't show this
create function b(diff_type) returns int8
  as 'select 1'
  language sql;
create function f(t) returns int8
  as 'select 1'
  language sql;
create table t (a int, b text);
delete from t where t.$0;
"), @r"
         label | kind     | detail | insert_text 
        -------+----------+--------+-------------
         a     | Column   | int    |             
         b     | Column   | text   |             
         f     | Function |        |
        ");
    }
}
