use rowan::{NodeOrToken, TextRange};
use salsa::Database as Db;
use squawk_syntax::{
    SyntaxKind,
    ast::{self, AstNode},
};

use crate::db::{File, parse};

/// A semantic token with its position and classification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticToken {
    pub range: TextRange,
    pub token_type: SemanticTokenType,
    pub modifiers: Option<SemanticTokenModifier>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum SemanticTokenModifier {
    Definition = 0,
    Readonly,
    Documentation,
}

/// Semantic token types supported by the language server.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SemanticTokenType {
    Keyword,
    String,
    Bool,
    Number,
    Function,
    Operator,
    Punctuation,
    Name,
    NameRef,
    Comment,
    Type,
    Parameter,
    PositionalParam,
}

#[salsa::tracked]
pub fn semantic_tokens(
    db: &dyn Db,
    file: File,
    range_to_highlight: Option<TextRange>,
) -> Vec<SemanticToken> {
    let parse = parse(db, file);
    let tree = parse.tree();
    let root = tree.syntax();

    // Determine the root based on the given range.
    let (root, range_to_highlight) = {
        let source_file = root;
        match range_to_highlight {
            Some(range) => {
                let node = match source_file.covering_element(range) {
                    NodeOrToken::Node(it) => it,
                    NodeOrToken::Token(it) => it.parent().unwrap_or_else(|| source_file.clone()),
                };
                (node, range)
            }
            None => (source_file.clone(), source_file.text_range()),
        }
    };

    let mut out = vec![];

    // Taken from: https://github.com/rust-lang/rust-analyzer/blob/2efc80078029894eec0699f62ec8d5c1a56af763/crates/ide/src/syntax_highlighting.rs#L267C21-L267C21
    let preorder = root.preorder_with_tokens();
    for event in preorder {
        use rowan::WalkEvent::{Enter, Leave};

        let range = match &event {
            Enter(it) | Leave(it) => it.text_range(),
        };

        // Element outside of the viewport, no need to highlight
        if range_to_highlight.intersect(range).is_none() {
            continue;
        }

        match event {
            Enter(NodeOrToken::Node(node)) => {
                if let Some(target) = ast::Target::cast(node) {
                    if let Some(as_name) = target.as_name() {
                        if let Some(name) = as_name.name() {
                            let range = name.syntax().text_range();
                            out.push(SemanticToken {
                                range,
                                token_type: SemanticTokenType::Name,
                                modifiers: None,
                            });
                        }
                    }
                };
            }
            Enter(NodeOrToken::Token(token)) => {
                if token.kind() == SyntaxKind::WHITESPACE {
                    continue;
                }
                if token.kind() == SyntaxKind::POSITIONAL_PARAM {
                    out.push(SemanticToken {
                        range: token.text_range(),
                        token_type: SemanticTokenType::PositionalParam,
                        modifiers: None,
                    })
                }
            }
            Leave(_) => {}
        }
    }
    out
}

#[cfg(test)]
mod test {
    use crate::db::{Database, File};
    use insta::assert_snapshot;
    use std::fmt::Write;

    fn semantic_tokens(sql: &str) -> String {
        let db = Database::default();
        let file = File::new(&db, sql.to_string().into());
        let tokens = super::semantic_tokens(&db, file, None);

        let mut result = String::new();
        for token in tokens {
            let start: usize = token.range.start().into();
            let end: usize = token.range.end().into();
            let token_text = &sql[start..end];
            // TODO:
            let modifiers_text = "";
            writeln!(
                result,
                "{:?} @ {}..{}: {:?}{}",
                token_text, start, end, token.token_type, modifiers_text
            )
            .unwrap();
        }
        result
    }

    #[test]
    fn create_function() {
        assert_snapshot!(semantic_tokens("
create function add(a int, b int) returns int
  as 'select $1 + $2'
  language sql;
"), @"");
    }

    #[test]
    fn select_keywords() {
        assert_snapshot!(semantic_tokens("
select 1 and, 2 select;
"), @r#"
        "and" @ 10..13: Name
        "select" @ 17..23: Name
        "#)
    }

    #[test]
    fn positional_param() {
        assert_snapshot!(semantic_tokens("
select $1, $2;
"), @r#"
        "$1" @ 8..10: PositionalParam
        "$2" @ 12..14: PositionalParam
        "#)
    }
}
