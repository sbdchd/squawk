use rowan::TextSize;
use squawk_syntax::ast::{self, AstNode};

use crate::tokens::is_string_or_comment;

pub fn completion(file: &ast::SourceFile, offset: TextSize) -> Vec<CompletionItem> {
    let Some(token) = file.syntax().token_at_offset(offset).right_biased() else {
        // empty file
        return top_level_completions();
    };

    // We don't support completions inside comments since we don't have doc
    // comments a la JSDoc.
    // And we don't have string literal types so we bail out early for strings too.
    if is_string_or_comment(token.kind()) {
        return vec![];
    }

    top_level_completions()
}

fn top_level_completions() -> Vec<CompletionItem> {
    ["select", "table"]
        .map(|x| CompletionItem::keyword(x.to_owned()))
        .to_vec()
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
}

impl CompletionItem {
    fn keyword(text: String) -> CompletionItem {
        CompletionItem {
            label: text,
            kind: CompletionItemKind::Keyword,
            detail: None,
            insert_text: None,
            insert_text_format: None,
        }
    }
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
                ]
            })
            .collect();

        rows.sort();

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
         label  | kind    | detail 
        --------+---------+--------
         select | Keyword |        
         table  | Keyword |
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
}
