use std::fmt::Write;
use std::iter;

use rowan::TextSize;
use rustc_hash::FxHashSet;
use salsa::Database as Db;
use squawk_syntax::{
    SyntaxKind, SyntaxNode, SyntaxNodePtr, SyntaxToken,
    ast::{self, AstNode},
};

use crate::db::{CompletionFile, FileId, parse, semantic_index};
use crate::file::{FileRange, InFile, InIrFile, IrFileId};
use crate::hover::function_signature;
use crate::interner::Name;
use crate::ir::{ColumnId, FunctionId, FunctionKind, Item, ParamMode, ScopeId, SourceId, Ty};
use crate::semantic_index::SemanticIndex;

pub(crate) const COMPLETION_MARKER: &str = "squawkCompletionMarker";

const STATEMENTS: &[&str] = &["delete from", "select", "table", "truncate"];
const SELECT_CLAUSES: &[&str] = &[
    "except",
    "fetch",
    "for",
    "group by",
    "having",
    "intersect",
    "limit",
    "offset",
    "order by",
    "union",
    "where",
    "window",
];
const DELETE_CLAUSES: &[&str] = &["returning", "using", "where"];

pub fn completion(db: &dyn Db, position: InFile<TextSize>) -> Vec<CompletionItem> {
    let file = position.file_id.original_file(db);
    let file_id = FileId::Completion(CompletionFile::new(db, file, position.value));
    let tree = parse(db, file_id).tree();
    let Some(token) = tree
        .syntax()
        .token_at_offset(position.value)
        .right_biased()
        .filter(|token| {
            token.kind() == SyntaxKind::IDENT && token.text().contains(COMPLETION_MARKER)
        })
    else {
        return vec![];
    };
    let Some(context) = CompletionContext::new(&token) else {
        return vec![];
    };
    let index = semantic_index(db, file_id);
    let scope = scope_at(index, InFile::new(file_id, token.text_range()));
    let mut completions = Completions {
        db,
        index,
        search_path: index.names.lookup("public"),
        items: vec![],
    };
    match context {
        CompletionContext::Statement => completions.snippets(STATEMENTS),
        CompletionContext::Table { schema: None } => {
            completions.tables_in_search_path();
            completions.schemas();
        }
        CompletionContext::Table {
            schema: Some(schema),
        } => {
            if let Some(schema) = index.names.lookup(&schema) {
                completions.tables_in(schema);
            }
        }
        CompletionContext::SelectClause => completions.snippets(SELECT_CLAUSES),
        CompletionContext::DeleteClause => completions.snippets(DELETE_CLAUSES),
        CompletionContext::Limit => {
            completions.keyword("all");
            completions.functions_in_search_path();
        }
        CompletionContext::Offset => completions.functions_in_search_path(),
        CompletionContext::Expr {
            qualifier: Some(qualifier),
            ..
        } => {
            let Some(qualifier) = index.names.lookup(&qualifier) else {
                return vec![];
            };
            if let Some(source) = visible_sources(index, scope)
                .find(|&source| index.sources[source].name == Some(qualifier))
            {
                completions.columns(&[source]);
                completions.field_functions(source);
            } else {
                completions.tables_in(qualifier);
                completions.functions_in(qualifier);
            }
        }
        CompletionContext::Expr {
            clause,
            qualifier: None,
        } => {
            let sources: Vec<SourceId> = visible_sources(index, scope)
                .filter(|&source| !index.sources[source].kind.is_qualified_only())
                .collect();
            completions.columns(&sources);
            completions.functions_in_search_path();
            match clause {
                Clause::Target { has_from } => {
                    completions.tables_in_search_path();
                    if !has_from {
                        completions.snippets(&["from"]);
                        completions.snippets(SELECT_CLAUSES);
                    }
                    completions.schemas();
                }
                Clause::Other => completions.source_names(&sources),
            }
        }
    }
    completions.items
}

#[derive(Clone, Copy)]
enum Clause {
    Target { has_from: bool },
    Other,
}

enum CompletionContext {
    Statement,
    Table {
        schema: Option<String>,
    },
    SelectClause,
    DeleteClause,
    Limit,
    Offset,
    Expr {
        clause: Clause,
        qualifier: Option<String>,
    },
}

impl CompletionContext {
    fn new(token: &SyntaxToken) -> Option<CompletionContext> {
        let parent = token.parent()?;
        match parent.kind() {
            SyntaxKind::ERROR if parent.parent()?.kind() == SyntaxKind::SOURCE_FILE => {
                Some(CompletionContext::Statement)
            }
            SyntaxKind::PATH_SEGMENT_REF => Self::table(&parent),
            SyntaxKind::TABLE_ALIAS => Self::alias(&parent),
            SyntaxKind::NAME_REF => Self::expr(&ast::NameRef::cast(parent)?),
            _ => None,
        }
    }

    fn table(segment: &SyntaxNode) -> Option<CompletionContext> {
        let path = ast::PathRef::cast(segment.parent()?)?;
        let owner = path.syntax().parent()?;
        if !matches!(
            owner.kind(),
            SyntaxKind::TABLE_NAME_REF | SyntaxKind::RELATION_NAME_REF
        ) {
            return None;
        }
        let Some(qualifier) = path.qualifier() else {
            let in_parens = owner
                .parent()
                .filter(|node| node.kind() == SyntaxKind::RELATION_FROM_ITEM)
                .and_then(|node| node.parent())
                .is_some_and(|node| node.kind() == SyntaxKind::PAREN_EXPR);
            if in_parens {
                return Some(CompletionContext::Statement);
            }
            return Some(CompletionContext::Table { schema: None });
        };
        if qualifier.qualifier().is_some() {
            return None;
        }
        Some(CompletionContext::Table {
            schema: Some(qualifier.segment()?.text()),
        })
    }

    fn alias(alias: &SyntaxNode) -> Option<CompletionContext> {
        let parent = alias.parent()?;
        let has_as = parent
            .children_with_tokens()
            .chain(alias.children_with_tokens())
            .any(|element| element.kind() == SyntaxKind::AS_KW);
        if has_as {
            return None;
        }
        match parent.kind() {
            SyntaxKind::FROM_ALIAS => Some(CompletionContext::SelectClause),
            SyntaxKind::OPTIONAL_AS_ALIAS if parent.parent()?.kind() == SyntaxKind::DELETE => {
                Some(CompletionContext::DeleteClause)
            }
            _ => None,
        }
    }

    fn expr(name_ref: &ast::NameRef) -> Option<CompletionContext> {
        let mut node = name_ref.syntax().clone();
        let mut qualifier = None;
        if let Some(field_expr) = node.parent().and_then(ast::FieldExpr::cast)
            && field_expr.field().as_ref() == Some(name_ref)
        {
            let ast::Expr::NameRef(base) = field_expr.base()? else {
                return None;
            };
            qualifier = Some(base.text());
            node = field_expr.syntax().clone();
        }
        node.ancestors().find_map(|node| {
            let clause = match node.kind() {
                SyntaxKind::TARGET_LIST => match node
                    .parent()
                    .filter(|node| node.kind() == SyntaxKind::SELECT_CLAUSE)
                    .and_then(|node| node.parent())
                    .and_then(ast::Select::cast)
                {
                    Some(select) => Clause::Target {
                        has_from: select.from_clause().is_some(),
                    },
                    None => Clause::Other,
                },
                SyntaxKind::LIMIT_CLAUSE if qualifier.is_none() => {
                    return Some(CompletionContext::Limit);
                }
                SyntaxKind::OFFSET_CLAUSE if qualifier.is_none() => {
                    return Some(CompletionContext::Offset);
                }
                SyntaxKind::WHERE_CLAUSE
                | SyntaxKind::GROUP_BY_CLAUSE
                | SyntaxKind::HAVING_CLAUSE
                | SyntaxKind::ON_CLAUSE
                | SyntaxKind::ORDER_BY_CLAUSE
                | SyntaxKind::LIMIT_CLAUSE
                | SyntaxKind::OFFSET_CLAUSE => Clause::Other,
                _ => return None,
            };
            Some(CompletionContext::Expr {
                clause,
                qualifier: qualifier.clone(),
            })
        })
    }
}

fn scope_at(index: &SemanticIndex, range: FileRange) -> Option<ScopeId> {
    index.items_at_range(range).find_map(|item| match item {
        Item::Reference(reference) => index.references[reference].scope,
        _ => None,
    })
}

fn visible_sources(
    index: &SemanticIndex,
    scope: Option<ScopeId>,
) -> impl Iterator<Item = SourceId> + '_ {
    iter::successors(scope, |&scope| index.scopes[scope].parent).flat_map(|scope| {
        let scope = &index.scopes[scope];
        let sources = &index.queries[scope.query].sources;
        scope.visible_indices().map(|index| sources[index])
    })
}

struct Completions<'a> {
    db: &'a dyn Db,
    index: &'a SemanticIndex,
    search_path: Option<Name>,
    items: Vec<CompletionItem>,
}

impl Completions<'_> {
    fn push(&mut self, label: String, kind: CompletionItemKind) -> &mut CompletionItem {
        self.items.push(CompletionItem {
            label,
            kind,
            detail: None,
            insert_text: None,
            insert_text_format: None,
            trigger_completion_after_insert: false,
            sort_key: None,
        });
        self.items.last_mut().unwrap()
    }

    fn snippets(&mut self, labels: &[&str]) {
        for label in labels {
            let item = self.push(label.to_string(), CompletionItemKind::Snippet);
            item.insert_text = Some(format!("{label} $0"));
            item.insert_text_format = Some(CompletionInsertTextFormat::Snippet);
            item.trigger_completion_after_insert = true;
        }
    }

    fn keyword(&mut self, keyword: &str) {
        self.push(keyword.to_string(), CompletionItemKind::Keyword);
    }

    fn schemas(&mut self) {
        let index = self.index;
        for (_, schema) in index.schemas.iter() {
            let name = index.names.resolve(schema.name);
            let rank = match name {
                "public" => 0,
                "information_schema" => 3,
                name if name.starts_with("pg_") => 2,
                _ => 1,
            };
            let item = self.push(name.to_string(), CompletionItemKind::Schema);
            item.sort_key = Some(format!("{rank}_{name}"));
        }
    }

    fn tables_in_search_path(&mut self) {
        if let Some(schema) = self.search_path {
            self.tables_in(schema);
        }
    }

    fn tables_in(&mut self, schema: Name) {
        let index = self.index;
        for table in index.tables.in_schema(schema) {
            let name = index.names.resolve(index.tables[table].name);
            self.push(name.to_string(), CompletionItemKind::Table);
        }
    }

    fn functions_in_search_path(&mut self) {
        if let Some(schema) = self.search_path {
            self.functions_in(schema);
        }
    }

    fn functions_in(&mut self, schema: Name) {
        let index = self.index;
        for function in index.functions.in_schema(schema) {
            let data = &index.functions[function];
            if data.kind == FunctionKind::Procedure {
                continue;
            }
            let name = index.names.resolve(data.name);
            let has_args = data.params.iter().any(|param| {
                matches!(
                    index.params[param].mode,
                    ParamMode::In | ParamMode::InOut | ParamMode::Variadic
                )
            });
            let detail = self.function_detail(function);
            let item = self.push(format!("{name}()"), CompletionItemKind::Function);
            item.detail = Some(detail);
            if has_args {
                item.insert_text = Some(format!("{name}($0)"));
                item.insert_text_format = Some(CompletionInsertTextFormat::Snippet);
            }
        }
    }

    fn field_functions(&mut self, source: SourceId) {
        let index = self.index;
        let Some(ty) = index.source_type(source) else {
            return;
        };
        let Some(schema) = self.search_path else {
            return;
        };
        for function in index.functions.in_schema(schema) {
            if takes_only(index, function, ty) {
                let name = index.names.resolve(index.functions[function].name);
                self.push(name.to_string(), CompletionItemKind::Function);
            }
        }
    }

    fn source_names(&mut self, sources: &[SourceId]) {
        let index = self.index;
        let mut seen = FxHashSet::default();
        for &source in sources {
            if let Some(name) = index.sources[source].name
                && seen.insert(name)
            {
                self.push(
                    index.names.resolve(name).to_string(),
                    CompletionItemKind::Table,
                );
            }
        }
    }

    fn columns(&mut self, sources: &[SourceId]) {
        let index = self.index;
        let mut seen = FxHashSet::default();
        let mut position = 0;
        for &source in sources {
            for column in index.sources[source].columns.range {
                let Some(name) = index.columns[column].name else {
                    continue;
                };
                if !seen.insert(name) {
                    continue;
                }
                let detail = self.column_type(column);
                let builtin = matches!(
                    index.file(index.columns[column].ptr.file_id),
                    FileId::Builtins | FileId::Extension(_)
                );
                let item = self.push(
                    index.names.resolve(name).to_string(),
                    CompletionItemKind::Column,
                );
                item.detail = detail;
                if !builtin {
                    item.sort_key = Some(format!("{position:04}"));
                }
                position += 1;
            }
        }
    }

    fn node(&self, ptr: InIrFile<SyntaxNodePtr>) -> Option<SyntaxNode> {
        let IrFileId::File(file_id) = ptr.file_id else {
            return None;
        };
        ptr.value
            .try_to_node(parse(self.db, file_id).tree().syntax())
    }

    fn function_detail(&self, function: FunctionId) -> String {
        let index = self.index;
        let data = &index.functions[function];
        let Some(create) = self.node(data.ptr).and_then(ast::CreateFunction::cast) else {
            return function_signature(index, function);
        };
        let mut detail = format!(
            "{}.{}",
            index.names.resolve(data.schema),
            index.names.resolve(data.name)
        );
        match create.param_list() {
            Some(params) => write!(detail, "{}", params.syntax().text()).unwrap(),
            None => detail.push_str("()"),
        }
        if let Some(ret) = create.ret_type() {
            write!(detail, " {}", ret.syntax().text()).unwrap();
        }
        detail
    }

    fn column_type(&self, column: ColumnId) -> Option<String> {
        let data = &self.index.columns[column];
        if let Some(ty) = self
            .node(data.ptr)
            .and_then(|node| node.parent())
            .and_then(ast::Column::cast)
            .and_then(|column| column.ty())
        {
            return Some(match &ty {
                ast::Type::PathType(path)
                    if path.arg_list().is_none()
                        && let Some(segment) = path.path_ref().and_then(|path| path.segment()) =>
                {
                    segment.text()
                }
                _ => ty.syntax().text().to_string(),
            });
        }
        Some(self.index.type_name(data.ty?).to_string())
    }
}

fn takes_only(index: &SemanticIndex, function: FunctionId, ty: Ty) -> bool {
    let mut params = index.functions[function].params.iter();
    params
        .next()
        .is_some_and(|param| index.params[param].ty == Some(ty))
        && params.next().is_none()
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
    Operator,
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
            Self::Operator => "8",
            Self::Schema => "9",
        }
    }
}

impl CompletionItem {
    pub fn sort_text(&self) -> String {
        let prefix = self.kind.sort_prefix();
        let suffix = self.sort_key.as_ref().unwrap_or(&self.label);
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
    pub sort_key: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::completion;

    use crate::test_utils::Fixture;
    use insta::assert_snapshot;
    use tabled::builder::Builder;
    use tabled::settings::Style;

    #[must_use]
    fn completions(sql: &str) -> String {
        let fixture = Fixture::new_allow_errors(sql);
        let offset = fixture.marker().offset();
        let items = completion(fixture.db(), offset);
        assert!(
            !items.is_empty(),
            "No completions found. If this was intended, use `completions_not_found` instead."
        );
        format_items(items)
    }

    fn completions_not_found(sql: &str) {
        let fixture = Fixture::new_allow_errors(sql);
        let offset = fixture.marker().offset();
        let items = completion(fixture.db(), offset);
        assert_eq!(
            items,
            vec![],
            "Completions found. If this was unintended, use `completions` instead."
        );
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
                ]
            })
            .collect();

        let mut builder = Builder::default();
        builder.push_record(["label", "kind", "detail"]);
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
         label       | kind    | detail 
        -------------+---------+--------
         delete from | Snippet |        
         select      | Snippet |        
         table       | Snippet |        
         truncate    | Snippet |
        ");
    }

    #[test]
    fn completion_at_top_level() {
        assert_snapshot!(completions("
create table t(a int);
$0
"), @r"
         label       | kind    | detail 
        -------------+---------+--------
         delete from | Snippet |        
         select      | Snippet |        
         table       | Snippet |        
         truncate    | Snippet |
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
         label              | kind   | detail 
        --------------------+--------+--------
         users              | Table  |        
         public             | Schema |        
         pg_catalog         | Schema |        
         pg_temp            | Schema |        
         pg_toast           | Schema |        
         information_schema | Schema |
        ");
    }

    #[test]
    fn completion_table_nested() {
        assert_snapshot!(completions("select * from ($0)"), @r"
         label       | kind    | detail 
        -------------+---------+--------
         delete from | Snippet |        
         select      | Snippet |        
         table       | Snippet |        
         truncate    | Snippet |
        ");
    }

    #[test]
    fn completion_after_table() {
        assert_snapshot!(completions("
create table users (id int);
table $0;
"), @r"
         label              | kind   | detail 
        --------------------+--------+--------
         users              | Table  |        
         public             | Schema |        
         pg_catalog         | Schema |        
         pg_temp            | Schema |        
         pg_toast           | Schema |        
         information_schema | Schema |
        ");
    }

    #[test]
    fn completion_select_without_from() {
        assert_snapshot!(completions("
create table t (a int);
select $0;
"), @r"
         label              | kind    | detail 
        --------------------+---------+--------
         except             | Snippet |        
         fetch              | Snippet |        
         for                | Snippet |        
         from               | Snippet |        
         group by           | Snippet |        
         having             | Snippet |        
         intersect          | Snippet |        
         limit              | Snippet |        
         offset             | Snippet |        
         order by           | Snippet |        
         t                  | Table   |        
         union              | Snippet |        
         where              | Snippet |        
         window             | Snippet |        
         public             | Schema  |        
         pg_catalog         | Schema  |        
         pg_temp            | Schema  |        
         pg_toast           | Schema  |        
         information_schema | Schema  |
        ");
    }

    #[test]
    fn completion_after_select() {
        assert_snapshot!(completions("
create table t(a text, b int);
create function f() returns text as 'select 1::text' language sql;
select $0 from t;
"), @r"
         label              | kind     | detail                  
        --------------------+----------+-------------------------
         a                  | Column   | text                    
         b                  | Column   | int                     
         t                  | Table    |                         
         f()                | Function | public.f() returns text 
         public             | Schema   |                         
         pg_catalog         | Schema   |                         
         pg_temp            | Schema   |                         
         pg_toast           | Schema   |                         
         information_schema | Schema   |
        ");
    }

    #[test]
    fn completion_after_select_create_table_inherits_builtin() {
        assert_snapshot!(completions("
-- include-builtins
create table t ()
inherits (information_schema.sql_features);
select $0 from t;
"), @"
         label              | kind   | detail         
        --------------------+--------+----------------
         comments           | Column | character_data 
         feature_id         | Column | character_data 
         feature_name       | Column | character_data 
         is_supported       | Column | yes_or_no      
         is_verified_by     | Column | character_data 
         sub_feature_id     | Column | character_data 
         sub_feature_name   | Column | character_data 
         t                  | Table  |                
         public             | Schema |                
         pg_catalog         | Schema |                
         pg_temp            | Schema |                
         pg_toast           | Schema |                
         information_schema | Schema |
        ");
    }

    #[test]
    fn completion_after_select_create_table_inherits_create_table_as() {
        assert_snapshot!(completions("
create table parent as select 1 a, 'x'::text b;
create table child (c int) inherits (parent);
select $0 from child;
"), @"
         label              | kind   | detail 
        --------------------+--------+--------
         a                  | Column | int4   
         b                  | Column | text   
         c                  | Column | int    
         child              | Table  |        
         parent             | Table  |        
         public             | Schema |        
         pg_catalog         | Schema |        
         pg_temp            | Schema |        
         pg_toast           | Schema |        
         information_schema | Schema |
        ");
    }

    #[test]
    fn completion_after_select_create_table_like_select_into() {
        assert_snapshot!(completions("
select 1 a, 'x'::text b into parent;
create table child (like parent);
select $0 from child;
"), @"
         label              | kind   | detail 
        --------------------+--------+--------
         a                  | Column | int4   
         b                  | Column | text   
         child              | Table  |        
         parent             | Table  |        
         public             | Schema |        
         pg_catalog         | Schema |        
         pg_temp            | Schema |        
         pg_toast           | Schema |        
         information_schema | Schema |
        ");
    }

    #[test]
    fn completion_in_where_clause_with_join() {
        assert_snapshot!(completions("
create table t (a int);
create table u (b int);
select * from t join u on t.a = u.b where $0;
"), @"
         label | kind   | detail 
        -------+--------+--------
         a     | Column | int    
         b     | Column | int    
         t     | Table  |        
         u     | Table  |
        ");
    }

    #[test]
    fn completion_after_select_select_into() {
        assert_snapshot!(completions("
select 1 a, 'x'::text b into t;
select $0 from t;
"), @"
         label              | kind   | detail 
        --------------------+--------+--------
         a                  | Column | int4   
         b                  | Column | text   
         t                  | Table  |        
         public             | Schema |        
         pg_catalog         | Schema |        
         pg_temp            | Schema |        
         pg_toast           | Schema |        
         information_schema | Schema |
        ");
    }

    #[test]
    fn completion_select_table_qualified() {
        assert_snapshot!(completions("
create table t (c int);
select t.$0 from t;
"), @"
         label | kind   | detail 
        -------+--------+--------
         c     | Column | int
        ");
    }

    #[test]
    fn completion_after_select_with_cte() {
        assert_snapshot!(completions("
with t as (select 1 a)
select $0 from t;
"), @"
         label              | kind   | detail 
        --------------------+--------+--------
         a                  | Column | int4   
         public             | Schema |        
         pg_catalog         | Schema |        
         pg_temp            | Schema |        
         pg_toast           | Schema |        
         information_schema | Schema |
        ");
    }

    #[test]
    fn completion_after_select_with_cte_alias_column_list() {
        assert_snapshot!(completions("
with t as (select 1 a, 2 b, 3 c)
select $0 from t as u(x, y);
"), @"
         label              | kind   | detail 
        --------------------+--------+--------
         x                  | Column | int4   
         y                  | Column | int4   
         c                  | Column | int4   
         public             | Schema |        
         pg_catalog         | Schema |        
         pg_temp            | Schema |        
         pg_toast           | Schema |        
         information_schema | Schema |
        ");
    }

    #[test]
    fn completion_values_cte() {
        assert_snapshot!(completions("
with t as (values (1, 'foo', false))
select $0 from t;
"), @"
         label              | kind   | detail 
        --------------------+--------+--------
         column1            | Column | int4   
         column2            | Column | text   
         column3            | Column | bool   
         public             | Schema |        
         pg_catalog         | Schema |        
         pg_temp            | Schema |        
         pg_toast           | Schema |        
         information_schema | Schema |
        ");
    }

    #[test]
    fn completion_values_subquery() {
        assert_snapshot!(completions("
select $0 from (values (1, 'foo', 1.5, false));
"), @"
         label              | kind   | detail  
        --------------------+--------+---------
         column1            | Column | int4    
         column2            | Column | text    
         column3            | Column | numeric 
         column4            | Column | bool    
         public             | Schema |         
         pg_catalog         | Schema |         
         pg_temp            | Schema |         
         pg_toast           | Schema |         
         information_schema | Schema |
        ");
    }

    #[test]
    fn completion_with_schema_qualifier() {
        assert_snapshot!(completions("
create function f() returns int8 as 'select 1' language sql;
create schema foo;
create function foo.b() returns int8 as 'select 2' language sql;
select public.$0;
"), @r"
         label | kind     | detail                  
        -------+----------+-------------------------
         f()   | Function | public.f() returns int8
        ");
    }

    #[test]
    fn completion_truncate_with_schema_qualifier() {
        assert_snapshot!(completions("
create table users (id int);
truncate public.$0;
"), @r"
         label | kind  | detail 
        -------+-------+--------
         users | Table |
        ");
    }

    #[test]
    fn completion_after_delete_from() {
        assert_snapshot!(completions("
create table users (id int);
delete from $0;
"), @r"
         label              | kind   | detail 
        --------------------+--------+--------
         users              | Table  |        
         public             | Schema |        
         pg_catalog         | Schema |        
         pg_temp            | Schema |        
         pg_toast           | Schema |        
         information_schema | Schema |
        ");
    }

    #[test]
    fn completion_delete_clauses() {
        assert_snapshot!(completions("
create table t (id int);
delete from t $0;
"), @r"
         label     | kind    | detail 
        -----------+---------+--------
         returning | Snippet |        
         using     | Snippet |        
         where     | Snippet |
        ");
    }

    #[test]
    fn completion_delete_where_expr() {
        assert_snapshot!(completions("
create table t (id int, name text);
create function is_active() returns bool as 'select true' language sql;
delete from t where $0;
"), @r"
         label       | kind     | detail                          
        -------------+----------+---------------------------------
         id          | Column   | int                             
         name        | Column   | text                            
         t           | Table    |                                 
         is_active() | Function | public.is_active() returns bool
        ");
    }

    #[test]
    fn completion_delete_returning_expr() {
        assert_snapshot!(completions("
create table t (id int, name text);
delete from t returning $0;
"), @r"
         label | kind   | detail 
        -------+--------+--------
         id    | Column | int    
         name  | Column | text   
         t     | Table  |
        ");
    }

    #[test]
    fn completion_delete_where_qualified() {
        assert_snapshot!(completions("
create table t (a int, b text);
create type diff_type as (x int);
-- different type than the table, so we shouldn't show this
create function b(diff_type) returns int8
  as 'select 1'
  language sql;
create function f(t) returns int8
  as 'select 1'
  language sql;
delete from t where t.$0;
"), @r"
         label | kind     | detail 
        -------+----------+--------
         a     | Column   | int    
         b     | Column   | text   
         f     | Function |
        ");
    }

    #[test]
    fn completion_select_clauses() {
        assert_snapshot!(completions("
with t as (select 1 a)
select a from t $0;
"), @r"
         label     | kind    | detail 
        -----------+---------+--------
         except    | Snippet |        
         fetch     | Snippet |        
         for       | Snippet |        
         group by  | Snippet |        
         having    | Snippet |        
         intersect | Snippet |        
         limit     | Snippet |        
         offset    | Snippet |        
         order by  | Snippet |        
         union     | Snippet |        
         where     | Snippet |        
         window    | Snippet |
        ");
    }

    #[test]
    fn completion_select_clauses_simple() {
        assert_snapshot!(completions("
create table t (a int);
select 1 from t $0;
"), @r"
         label     | kind    | detail 
        -----------+---------+--------
         except    | Snippet |        
         fetch     | Snippet |        
         for       | Snippet |        
         group by  | Snippet |        
         having    | Snippet |        
         intersect | Snippet |        
         limit     | Snippet |        
         offset    | Snippet |        
         order by  | Snippet |        
         union     | Snippet |        
         where     | Snippet |        
         window    | Snippet |
        ");
    }

    #[test]
    fn completion_select_group_by_expr() {
        assert_snapshot!(completions("
with t as (select 1 a)
select a from t group by $0;
"), @"
         label | kind   | detail 
        -------+--------+--------
         a     | Column | int4   
         t     | Table  |
        ");
    }

    #[test]
    fn completion_select_where_expr() {
        assert_snapshot!(completions("
create table t (id int, name text);
select * from t where $0;
"), @r"
         label | kind   | detail 
        -------+--------+--------
         id    | Column | int    
         name  | Column | text   
         t     | Table  |
        ");
    }

    #[test]
    fn completion_select_limit() {
        assert_snapshot!(completions("
create table t (a int);
create function get_limit() returns int as 'select 10' language sql;
select 1 from t limit $0;
"), @r"
         label       | kind     | detail                         
        -------------+----------+--------------------------------
         all         | Keyword  |                                
         get_limit() | Function | public.get_limit() returns int
        ");
    }

    #[test]
    fn completion_select_offset() {
        assert_snapshot!(completions("
create table t (a int);
create function get_offset() returns int as 'select 10' language sql;
select 1 from t offset $0;
"), @r"
         label        | kind     | detail                          
        --------------+----------+---------------------------------
         get_offset() | Function | public.get_offset() returns int
        ");
    }

    #[test]
    fn completion_lateral_subquery_target() {
        assert_snapshot!(completions("
create table t (a int);
select * from t, lateral (select $0) s;
"), @"
         label              | kind    | detail 
        --------------------+---------+--------
         a                  | Column  | int    
         except             | Snippet |        
         fetch              | Snippet |        
         for                | Snippet |        
         from               | Snippet |        
         group by           | Snippet |        
         having             | Snippet |        
         intersect          | Snippet |        
         limit              | Snippet |        
         offset             | Snippet |        
         order by           | Snippet |        
         t                  | Table   |        
         union              | Snippet |        
         where              | Snippet |        
         window             | Snippet |        
         public             | Schema  |        
         pg_catalog         | Schema  |        
         pg_temp            | Schema  |        
         pg_toast           | Schema  |        
         information_schema | Schema  |
        ");
    }

    #[test]
    fn completion_from_subquery_target() {
        assert_snapshot!(completions("
create table t (a int);
select * from t, (select $0) s;
"), @"
         label              | kind    | detail 
        --------------------+---------+--------
         except             | Snippet |        
         fetch              | Snippet |        
         for                | Snippet |        
         from               | Snippet |        
         group by           | Snippet |        
         having             | Snippet |        
         intersect          | Snippet |        
         limit              | Snippet |        
         offset             | Snippet |        
         order by           | Snippet |        
         t                  | Table   |        
         union              | Snippet |        
         where              | Snippet |        
         window             | Snippet |        
         public             | Schema  |        
         pg_catalog         | Schema  |        
         pg_temp            | Schema  |        
         pg_toast           | Schema  |        
         information_schema | Schema  |
        ");
    }

    #[test]
    fn completion_correlated_subquery_target() {
        assert_snapshot!(completions("
create table t (a int);
select * from t where exists (select $0);
"), @"
         label              | kind    | detail 
        --------------------+---------+--------
         a                  | Column  | int    
         except             | Snippet |        
         fetch              | Snippet |        
         for                | Snippet |        
         from               | Snippet |        
         group by           | Snippet |        
         having             | Snippet |        
         intersect          | Snippet |        
         limit              | Snippet |        
         offset             | Snippet |        
         order by           | Snippet |        
         t                  | Table   |        
         union              | Snippet |        
         where              | Snippet |        
         window             | Snippet |        
         public             | Schema  |        
         pg_catalog         | Schema  |        
         pg_temp            | Schema  |        
         pg_toast           | Schema  |        
         information_schema | Schema  |
        ");
    }

    #[test]
    fn completion_lateral_subquery_later_source() {
        assert_snapshot!(completions("
create table t (a int);
create table u (b int);
select * from t, lateral (select $0 from u) s, u v;
"), @"
         label              | kind   | detail 
        --------------------+--------+--------
         b                  | Column | int    
         a                  | Column | int    
         t                  | Table  |        
         u                  | Table  |        
         public             | Schema |        
         pg_catalog         | Schema |        
         pg_temp            | Schema |        
         pg_toast           | Schema |        
         information_schema | Schema |
        ");
    }

    #[test]
    fn completion_join_on_later_source() {
        assert_snapshot!(completions("
create table t (a int);
create table u (b int);
create table v (c int);
select * from t join u on $0 join v on true;
"), @"
         label | kind   | detail 
        -------+--------+--------
         a     | Column | int    
         b     | Column | int    
         t     | Table  |        
         u     | Table  |
        ");
    }
}
