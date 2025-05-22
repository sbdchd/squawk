// based on https://github.com/rust-lang/rust-analyzer/blob/96a253112ca280270a476297fe433e19b2da35e3/xtask/src/codegen/grammar.rs#L1C1-L1C1
// Permission is hereby granted, free of charge, to any
// person obtaining a copy of this software and associated
// documentation files (the "Software"), to deal in the
// Software without restriction, including without
// limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following
// conditions:
//
// The above copyright notice and this permission notice
// shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
// ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
// TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
// PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
// SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

use std::collections::{HashMap, HashSet};

use anyhow::{bail, Context, Result};
use convert_case::{Case, Casing};
use enum_iterator::{all, Sequence};
use proc_macro2::{Punct, Spacing};
use quote::{format_ident, quote};
use ungrammar::{Grammar, Rule};
use xshell::{cmd, Shell};

use crate::path::project_root;

fn ensure_rustfmt(sh: &Shell) {
    let version = cmd!(sh, "rustup run stable rustfmt --version")
        .read()
        .unwrap_or_default();
    if !version.contains("stable") {
        panic!(
            "Failed to run rustfmt from toolchain 'stable'. \
                 Please run `rustup component add rustfmt --toolchain stable` to install it.",
        );
    }
}

fn reformat(text: String) -> String {
    let sh = Shell::new().unwrap();
    ensure_rustfmt(&sh);
    let mut stdout = cmd!(sh, "rustup run stable rustfmt")
        .stdin(text)
        .read()
        .unwrap();
    if !stdout.ends_with('\n') {
        stdout.push('\n');
    }
    stdout
}

pub(crate) fn codegen() -> Result<()> {
    let postgres_grammar = project_root().join("crates/squawk_syntax/src/postgresql.ungram");

    let grammar: Grammar = std::fs::read_to_string(postgres_grammar)?.parse()?;
    let ast_src = lower(&grammar);

    let ast_tokens = generate_tokens(&ast_src.tokens);
    let ast_tokens_file = project_root().join("crates/squawk_syntax/src/ast/generated/tokens.rs");
    std::fs::write(ast_tokens_file, ast_tokens).context("problem writing generated tokens")?;

    let ast_nodes = generate_nodes(&ast_src.nodes, &ast_src.enums);
    let ast_nodes_file = project_root().join("crates/squawk_syntax/src/ast/generated/nodes.rs");
    std::fs::write(ast_nodes_file, ast_nodes).context("problem writing generated nodes")?;

    let token_sets = generate_token_sets()?;
    let token_sets_file = project_root().join("crates/squawk_parser/src/generated/token_sets.rs");
    std::fs::write(token_sets_file, token_sets).context("problem writing generated token sets")?;

    let pg_keywords = parse_header()?
        .into_iter()
        .map(|(key, _)| key)
        .collect::<Vec<_>>();

    let kinds = generate_kind_src(&ast_src.nodes, &ast_src.enums, &grammar, pg_keywords);

    let syntax_kinds = generate_syntax_kinds(kinds)?;
    let syntax_kinds_file =
        project_root().join("crates/squawk_parser/src/generated/syntax_kind.rs");
    std::fs::write(syntax_kinds_file, syntax_kinds).context("problem writing syntax kinds")?;

    Ok(())
}

#[derive(Copy, Clone, Debug)]
pub(crate) struct KindsSrc {
    pub(crate) punct: &'static [(&'static str, &'static str)],
    pub(crate) keywords: &'static [&'static str],
    pub(crate) contextual_keywords: &'static [&'static str],
    pub(crate) literals: &'static [&'static str],
    pub(crate) tokens: &'static [&'static str],
    pub(crate) nodes: &'static [&'static str],
}

const TOKENS: &[&str] = &["ERROR", "WHITESPACE", "COMMENT"];

const EOF: &str = "EOF";

/// The punctuations of the language.
const PUNCT: &[(&str, &str)] = &[
    // KEEP THE DOLLAR AT THE TOP ITS SPECIAL
    ("$", "DOLLAR"),
    (";", "SEMICOLON"),
    (",", "COMMA"),
    ("(", "L_PAREN"),
    (")", "R_PAREN"),
    // ("{", "L_CURLY"),
    // ("}", "R_CURLY"),
    ("[", "L_BRACK"),
    ("]", "R_BRACK"),
    ("<", "L_ANGLE"),
    (">", "R_ANGLE"),
    ("@", "AT"),
    ("#", "POUND"),
    ("~", "TILDE"),
    ("?", "QUESTION"),
    ("&", "AMP"),
    ("|", "PIPE"),
    ("+", "PLUS"),
    ("*", "STAR"),
    ("/", "SLASH"),
    ("^", "CARET"),
    ("%", "PERCENT"),
    ("_", "UNDERSCORE"),
    (".", "DOT"),
    // ("..", "DOT2"),
    // ("...", "DOT3"),
    // ("..=", "DOT2EQ"),
    (":", "COLON"),
    // ("::", "COLON_COLON"),
    ("=", "EQ"),
    // ("==", "EQ2"),
    // ("=>", "FAT_ARROW"),
    ("!", "BANG"),
    // ("!=", "NEQ"),
    ("-", "MINUS"),
    // ("->", "THIN_ARROW"),
    // ("<=", "LTEQ"),
    // (">=", "GTEQ"),
    // ("+=", "PLUSEQ"),
    // ("-=", "MINUSEQ"),
    // ("|=", "PIPEEQ"),
    // ("&=", "AMPEQ"),
    // ("^=", "CARETEQ"),
    // ("/=", "SLASHEQ"),
    // ("*=", "STAREQ"),
    // ("%=", "PERCENTEQ"),
    // ("&&", "AMP2"),
    // ("||", "PIPE2"),
    // ("<<", "SHL"),
    // (">>", "SHR"),
    // ("<<=", "SHLEQ"),
    // (">>=", "SHREQ"),
    ("`", "BACKTICK"),
];

const RESERVED: &[&str] = &[
    // "abstract", "become", "box", "do", "final", "macro", "override", "priv", "typeof", "unsized",
    // "virtual", "yield", "try",
];
const CONTEXTUAL_RESERVED: &[&str] = &[];

fn generate_kind_src(
    nodes: &[AstNodeSrc],
    enums: &[AstEnumSrc],
    grammar: &ungrammar::Grammar,
    pg_keywords: Vec<String>,
) -> KindsSrc {
    let mut keywords: Vec<&_> = Vec::new();
    let mut contextual_keywords: Vec<&_> = Vec::new();
    let mut tokens: Vec<&_> = TOKENS.to_vec();
    let mut literals: Vec<&_> = Vec::new();
    let mut used_puncts = vec![false; PUNCT.len()];
    // Mark $ as used
    used_puncts[0] = true;
    grammar.tokens().for_each(|token| {
        let name = &*grammar[token].name;
        if name == EOF {
            return;
        }
        match name.split_at(1) {
            ("@", lit) if !lit.is_empty() => {
                literals.push(String::leak(lit.to_case(Case::UpperSnake)));
            }
            ("#", token) if !token.is_empty() => {
                tokens.push(String::leak(token.to_case(Case::UpperSnake)));
            }
            ("?", kw) if !kw.is_empty() => {
                contextual_keywords.push(String::leak(kw.to_owned()));
            }
            _ if name.chars().all(|c| c.is_alphabetic() || c == '_') => {
                keywords.push(String::leak(name.to_owned()));
            }
            _ => {
                let idx = PUNCT
                    .iter()
                    .position(|(punct, _)| punct == &name)
                    .unwrap_or_else(|| panic!("Grammar references unknown punctuation {name:?}"));
                used_puncts[idx] = true;
            }
        }
    });
    PUNCT
        .iter()
        .zip(used_puncts)
        .filter(|(_, used)| !used)
        .for_each(|((punct, _), _)| {
            if *punct != "_" {
                panic!("Punctuation {punct:?} is not used in grammar");
            }
        });
    keywords.extend(RESERVED.iter().copied());
    keywords.extend(pg_keywords.into_iter().map(|s| &*s.leak()));
    keywords.sort();
    keywords.dedup();
    contextual_keywords.extend(CONTEXTUAL_RESERVED.iter().copied());
    contextual_keywords.sort();
    contextual_keywords.dedup();

    // we leak things here for simplicity, that way we don't have to deal with lifetimes
    // The execution is a one shot job so thats fine
    let nodes = nodes
        .iter()
        .map(|it| &it.name)
        .chain(enums.iter().map(|it| &it.name))
        .map(|it| it.to_case(Case::UpperSnake))
        .map(String::leak)
        .map(|it| &*it)
        .collect();
    let nodes = Vec::leak(nodes);
    nodes.sort();
    let keywords = Vec::leak(keywords);
    let contextual_keywords = Vec::leak(contextual_keywords);
    let literals = Vec::leak(literals);
    literals.sort();
    let tokens = Vec::leak(tokens);
    tokens.sort();

    KindsSrc {
        punct: PUNCT,
        nodes,
        keywords,
        contextual_keywords,
        literals,
        tokens,
    }
}

const LITERALS: [(&'static str, &'static str); 12] = [
    // TODO: get these from the grammar
    ("FLOAT_NUMBER", "`1.0`"),
    ("INT_NUMBER", "`1`"),
    ("STRING", "`'foo'`"),
    (
        "BYTE_STRING",
        r#"
`X'1FF'`, `U&'d\0061t\+000061'`

see: <https://www.postgresql.org/docs/16/sql-syntax-lexical.html#SQL-SYNTAX-STRINGS-UESCAPE>
see: <https://www.postgresql.org/docs/16/sql-syntax-lexical.html#SQL-SYNTAX-BIT-STRINGS>
"#,
    ),
    (
        "BIT_STRING",
        r#"
`X'1FF'`, `U&'d\0061t\+000061'`

see: <https://www.postgresql.org/docs/16/sql-syntax-lexical.html#SQL-SYNTAX-STRINGS-UESCAPE>
see: <https://www.postgresql.org/docs/16/sql-syntax-lexical.html#SQL-SYNTAX-BIT-STRINGS>
"#,
    ),
    (
        "DOLLAR_QUOTED_STRING",
        r#"

`$$Dianne's horse$$`

see: <https://www.postgresql.org/docs/16/sql-syntax-lexical.html#SQL-SYNTAX-DOLLAR-QUOTING>

        "#,
    ),
    (
        "ESC_STRING",
        r#"
`E'foo'`
                                                                         
see: <https://www.postgresql.org/docs/16/sql-syntax-lexical.html>
"#,
    ),
    ("PARAM", "`$1`"),
    // not in grammar?
    (
        "COMMENT",
        r#"
`-- foo`
or
`/* foo */`

see: <https://www.postgresql.org/docs/17/sql-syntax-lexical.html#SQL-SYNTAX-COMMENTS>

"#,
    ),
    ("IDENT", "`foo`"),
    ("ERROR", ""),
    ("WHITESPACE", ""),
];

const SYMBOLS: [&'static str; 25] = [
    ";", ",", "(", ")", "[", "]", "<", ">", "@", "#", "~", "?", "&", "|", "+", "*", "/", "^", "%",
    ".", ":", "`", "!", "-", "=",
];

fn generate_syntax_kinds(grammar: KindsSrc) -> Result<String> {
    // TODO: we should have a check to make sure each keyword is used in the grammar once the grammar is ready
    //     let keywords_ = parse_header()?;

    //     let mut keywords = keywords_
    //         .iter()
    //         .map(|(key, _value)| key)
    //         .collect::<Vec<_>>();
    //     keywords.sort();

    //     let keywords = keywords_
    //         .iter()
    //         .map(|(key, _value)| {
    //             let ident = format_ident!("{}_KW", key.to_case(Case::UpperSnake));
    //             let doc_string = format!(r#"`{key}`"#);
    //             quote! {
    //                 #[doc = #doc_string]
    //                 #ident
    //             }
    //         })
    //         .collect::<Vec<_>>();

    //     let symbols = SYMBOLS
    //         .iter()
    //         .map(|s| {
    //             let ident = format_ident!(
    //                 "{}",
    //                 token_to_name(s).unwrap_or(s).to_case(Case::UpperSnake)
    //             );
    //             let doc_string = if *s == "`" {
    //                 r#"`` ` ``"#.to_string()
    //             } else {
    //                 format!(r#"`{s}`"#)
    //             };
    //             quote! {
    //                 #[doc = #doc_string]
    //                 #ident
    //             }
    //         })
    //         .collect::<Vec<_>>();

    //     let literals = LITERALS
    //         .iter()
    //         .map(|(x, doc)| {
    //             let ident = format_ident!("{}", x.to_case(Case::UpperSnake));
    //             let doc_string = format!(r#"{}"#, doc.trim());
    //             quote! {
    //                 #[doc = #doc_string]
    //                 #ident
    //             }
    //         })
    //         .collect::<Vec<_>>();

    let conditions = grammar
        .keywords
        .iter()
        .enumerate()
        .map(|(i, keyword)| {
            let kind_ident = format_ident!("{}_KW", keyword.to_case(Case::UpperSnake));
            if i == 0 {
                quote! {
                    if ident.eq_ignore_ascii_case(#keyword) {
                        SyntaxKind::#kind_ident
                    }
                }
            } else {
                quote! {
                    else if ident.eq_ignore_ascii_case(#keyword) {
                        SyntaxKind::#kind_ident
                    }
                }
            }
        })
        .collect::<Vec<_>>();

    //     let nodes = "
    // LTEQ
    // GTEQ
    // CUSTOM_OP
    // COLON_COLON
    // COLON_EQ
    // NEQ
    // NEQB
    // FAT_ARROW
    // ARG_LIST
    // ARG
    // PARAM_LIST
    // COLLATE
    // TARGET_LIST
    // TARGET
    // ARRAY_EXPR
    // IS_NULL
    // IS_NOT
    // IS_NOT_DISTINCT_FROM
    // OPERATOR_CALL
    // AT_TIME_ZONE
    // SIMILAR_TO
    // IS_DISTINCT_FROM
    // NOT_LIKE
    // NOT_IN
    // BIN_EXPR
    // POSTFIX_EXPR
    // CALL_EXPR
    // BETWEEN_EXPR
    // CAST_EXPR
    // CASE_EXPR
    // ALIAS
    // FIELD_EXPR
    // INDEX_EXPR
    // LITERAL
    // NAME
    // NAMED_ARG
    // JSON_KEY_VALUE
    // PAREN_EXPR
    // PATH
    // PATH_SEGMENT
    // PATH_TYPE
    // CHAR_TYPE
    // BIT_TYPE
    // PERCENT_TYPE
    // DOUBLE_TYPE
    // TIME_TYPE
    // INTERVAL_TYPE
    // ARRAY_TYPE
    // PERCENT_TYPE_CLAUSE
    // WITH_TIMEZONE
    // WITHOUT_TIMEZONE
    // PREFIX_EXPR
    // COLUMN
    // SOURCE_FILE
    // RET_TYPE
    // STMT
    // ALTER_AGGREGATE_STMT
    // ALTER_COLLATION_STMT
    // ALTER_CONVERSION_STMT
    // ALTER_DATABASE_STMT
    // ALTER_DEFAULT_PRIVILEGES_STMT
    // ALTER_DOMAIN_STMT
    // ALTER_EVENT_TRIGGER_STMT
    // ALTER_EXTENSION_STMT
    // ALTER_FOREIGN_DATA_WRAPPER_STMT
    // ALTER_FOREIGN_TABLE_STMT
    // ALTER_FUNCTION_STMT
    // ALTER_GROUP_STMT
    // ALTER_INDEX_STMT
    // ALTER_LANGUAGE_STMT
    // ALTER_LARGE_OBJECT_STMT
    // ALTER_MATERIALIZED_VIEW_STMT
    // ALTER_OPERATOR_STMT
    // ALTER_OPERATOR_CLASS_STMT
    // ALTER_OPERATOR_FAMILY_STMT
    // ALTER_POLICY_STMT
    // ALTER_PROCEDURE_STMT
    // ALTER_PUBLICATION_STMT
    // ALTER_ROLE_STMT
    // ALTER_ROUTINE_STMT
    // ALTER_RULE_STMT
    // ALTER_SCHEMA_STMT
    // ALTER_SEQUENCE_STMT
    // ALTER_SERVER_STMT
    // ALTER_STATISTICS_STMT
    // ALTER_SUBSCRIPTION_STMT
    // ALTER_SYSTEM_STMT
    // ALTER_TABLESPACE_STMT
    // ALTER_TEXT_SEARCH_CONFIGURATION_STMT
    // ALTER_TEXT_SEARCH_DICTIONARY_STMT
    // ALTER_TEXT_SEARCH_PARSER_STMT
    // ALTER_TEXT_SEARCH_TEMPLATE_STMT
    // ALTER_TRIGGER_STMT
    // ALTER_TYPE_STMT
    // ALTER_USER_STMT
    // ALTER_USER_MAPPING_STMT
    // ALTER_VIEW_STMT
    // ANALYZE_STMT
    // CLUSTER_STMT
    // COMMENT_STMT
    // COMMIT
    // CREATE_EXTENSION_STMT
    // CREATE_ACCESS_METHOD_STMT
    // CREATE_AGGREGATE_STMT
    // CREATE_CAST_STMT
    // CREATE_COLLATION_STMT
    // CREATE_CONVERSION_STMT
    // CREATE_DATABASE_STMT
    // CREATE_DOMAIN_STMT
    // CREATE_EVENT_TRIGGER_STMT
    // CREATE_FOREIGN_DATA_WRAPPER_STMT
    // CREATE_FOREIGN_TABLE_STMT
    // CREATE_GROUP_STMT
    // CREATE_LANGUAGE_STMT
    // CREATE_MATERIALIZED_VIEW_STMT
    // CREATE_OPERATOR_STMT
    // CREATE_OPERATOR_CLASS_STMT
    // CREATE_OPERATOR_FAMILY_STMT
    // CREATE_POLICY_STMT
    // CREATE_PROCEDURE_STMT
    // CREATE_PUBLICATION_STMT
    // CREATE_ROLE_STMT
    // CREATE_RULE_STMT
    // CREATE_SEQUENCE_STMT
    // CREATE_SERVER_STMT
    // CREATE_STATISTICS_STMT
    // CREATE_SUBSCRIPTION_STMT
    // CREATE_TABLE_AS_STMT
    // CREATE_TABLESPACE_STMT
    // CREATE_TEXT_SEARCH_CONFIGURATION_STMT
    // CREATE_TEXT_SEARCH_DICTIONARY_STMT
    // CREATE_TEXT_SEARCH_PARSER_STMT
    // CREATE_TEXT_SEARCH_TEMPLATE_STMT
    // CREATE_TRANSFORM_STMT
    // CREATE_INDEX_STMT
    // CREATE_TYPE_STMT
    // CREATE_TRIGGER_STMT
    // CREATE_FUNCTION_STMT
    // PARAM_IN
    // PARAM_OUT
    // PARAM_IN_OUT
    // PARAM_VARIADIC
    // BEGIN_FUNC_OPTION
    // RETURN_FUNC_OPTION
    // AS_FUNC_OPTION
    // SET_FUNC_OPTION
    // SUPPORT_FUNC_OPTION
    // ROWS_FUNC_OPTION
    // COST_FUNC_OPTION
    // PARALLEL_FUNC_OPTION
    // SECURITY_FUNC_OPTION
    // STRICT_FUNC_OPTION
    // LEAKPROOF_FUNC_OPTION
    // RESET_FUNC_OPTION
    // VOLATILITY_FUNC_OPTION
    // WINDOW_FUNC_OPTION
    // TRANSFORM_FUNC_OPTION
    // LANGUAGE_FUNC_OPTION
    // PARAM_DEFAULT
    // FUNC_OPTION_LIST
    // IF_EXISTS
    // IF_NOT_EXISTS
    // OR_REPLACE
    // DROP_INDEX_STMT
    // DROP_TRIGGER_STMT
    // BEGIN
    // SHOW_STMT
    // SET_STMT
    // PREPARE_TRANSACTION_STMT
    // DROP_DATABASE_STMT
    // DROP_TYPE_STMT
    // CALL_STMT
    // TRUNCATE
    // MOVE_STMT
    // FETCH_STMT
    // DECLARE_STMT
    // DO_STMT
    // DISCARD_STMT
    // RESET_STMT
    // LISTEN_STMT
    // LOAD_STMT
    // DEALLOCATE_STMT
    // CHECKPOINT_STMT
    // PREPARE_STMT
    // UNLISTEN_STMT
    // NOTIFY_STMT
    // CLOSE_STMT
    // VACUUM_STMT
    // COPY_STMT
    // DELETE_STMT
    // MERGE_STMT
    // LOCK_STMT
    // EXPLAIN_STMT
    // DROP_USER_STMT
    // DROP_TRANSFORM_STMT
    // DROP_TEXT_SEARCH_TEMPLATE_STMT
    // DROP_TEXT_SEARCH_PARSER_STMT
    // DROP_TEXT_SEARCH_DICT_STMT
    // DROP_TEXT_SEARCH_CONFIG_STMT
    // DROP_TABLESPACE_STMT
    // DROP_SUBSCRIPTION_STMT
    // DROP_STATISTICS_STMT
    // DROP_SERVER_STMT
    // DROP_SEQUENCE_STMT
    // DROP_RULE_STMT
    // DROP_ROUTINE_STMT
    // DROP_ROLE_STMT
    // DROP_PUBLICATION_STMT
    // DROP_PROCEDURE_STMT
    // DROP_POLICY_STMT
    // DROP_OWNED_STMT
    // DROP_OPERATOR_FAMILY_STMT
    // DROP_OPERATOR_CLASS_STMT
    // DROP_MATERIALIZED_VIEW_STMT
    // DROP_OPERATOR_STMT
    // DROP_LANGUAGE_STMT
    // DROP_GROUP_STMT
    // DROP_FUNCTION_STMT
    // DROP_FOREIGN_TABLE_STMT
    // DROP_FOREIGN_DATA_WRAPPER_STMT
    // DROP_EXTENSION_STMT
    // DROP_EVENT_TRIGGER_STMT
    // DROP_DOMAIN_STMT
    // DROP_CONVERSION_STMT
    // DROP_COLLATION_STMT
    // DROP_CAST_STMT
    // DROP_AGGREGATE_STMT
    // DROP_ACCESS_METHOD_STMT
    // DROP_USER_MAPPING_STMT
    // IMPORT_FOREIGN_SCHEMA
    // EXECUTE_STMT
    // CREATE_VIEW_STMT
    // SAVEPOINT_STMT
    // RELEASE_SAVEPOINT_STMT
    // DROP_SCHEMA_STMT
    // DROP_VIEW_STMT
    // REINDEX_STMT
    // UPDATE_STMT
    // ROLLBACK_STMT
    // INSERT_STMT
    // CREATE_SCHEMA_STMT
    // SELECT
    // TABLE_STMT
    // VALUES
    // SELECT_INTO_STMT
    // SECURITY_LABEL_STMT
    // REVOKE_STMT
    // GRANT_STMT
    // REFRESH_STMT
    // REASSIGN_STMT
    // SET_SESSION_AUTH_STMT
    // CREATE_USER_MAPPING_STMT
    // CREATE_USER_STMT
    // SET_ROLE_STMT
    // SET_CONSTRAINTS_STMT
    // SET_TRANSACTION_STMT
    // INTO_CLAUSE
    // COMPOUND_SELECT
    // DROP_TABLE
    // JOIN
    // CREATE_TABLE
    // ALTER_TABLE
    // WINDOW_DEF
    // JSON_VALUE_EXPR
    // JSON_FORMAT_CLAUSE
    // JSON_RETURNING_CLAUSE
    // JSON_QUOTES_CLAUSE
    // JSON_WRAPPER_BEHAVIOR_CLAUSE
    // JSON_BEHAVIOR_CLAUSE
    // JSON_PASSING_CLAUSE
    // JSON_ON_ERROR_CLAUSE
    // JSON_NULL_CLAUSE
    // JSON_KEYS_UNIQUE_CLAUSE
    // SELECT_CLAUSE
    // LIKE_CLAUSE
    // REFERENCES_CONSTRAINT
    // PRIMARY_KEY_CONSTRAINT
    // FOREIGN_KEY_CONSTRAINT
    // EXCLUDE_CONSTRAINT
    // UNIQUE_CONSTRAINT
    // GENERATED_CONSTRAINT
    // DEFAULT_CONSTRAINT
    // CHECK_CONSTRAINT
    // NULL_CONSTRAINT
    // NOT_NULL_CONSTRAINT
    // INDEX_PARAMS
    // CONSTRAINT_INDEX_TABLESPACE
    // CONSTRAINT_STORAGE_PARAMS
    // CONSTRAINT_INCLUDE_CLAUSE
    // CONSTRAINT_WHERE_CLAUSE
    // CONSTRAINT_INDEX_METHOD
    // CONSTRAINT_EXCLUSIONS
    // DEFERRABLE_CONSTRAINT_OPTION
    // NOT_DEFERRABLE_CONSTRAINT_OPTION
    // INITIALLY_DEFERRED_CONSTRAINT_OPTION
    // INITIALLY_IMMEDIATE_CONSTRAINT_OPTION
    // CONSTRAINT_OPTION_LIST
    // SEQUENCE_OPTION_LIST
    // USING_INDEX
    // VALIDATE_CONSTRAINT
    // REPLICA_IDENTITY
    // OF_TYPE
    // NOT_OF
    // FORCE_RLS
    // NO_FORCE_RLS
    // INHERIT
    // NO_INHERIT
    // ENABLE_TRIGGER
    // ENABLE_REPLICA_TRIGGER
    // ENABLE_REPLICA_RULE
    // ENABLE_ALWAYS_TRIGGER
    // ENABLE_ALWAYS_RULE
    // ENABLE_RULE
    // ENABLE_RLS
    // DISABLE_TRIGGER
    // DISABLE_RLS
    // DISABLE_RULE
    // CLUSTER_ON
    // OWNER_TO
    // DETACH_PARTITION
    // DROP_CONSTRAINT
    // DROP_COLUMN
    // ADD_CONSTRAINT
    // ADD_COLUMN
    // ATTACH_PARTITION
    // TABLE_LIST
    // SET_SCHEMA
    // SET_TABLESPACE
    // SET_WITHOUT_CLUSTER
    // SET_WITHOUT_OIDS
    // SET_ACCESS_METHOD
    // SET_LOGGED
    // SET_UNLOGGED
    // SET_STORAGE_PARAMS
    // RESET_STORAGE_PARAMS
    // RENAME_TABLE
    // RENAME_CONSTRAINT
    // RENAME_COLUMN
    // RENAME_TO
    // NOT_VALID
    // ALTER_CONSTRAINT
    // ALTER_COLUMN
    // DROP_DEFAULT
    // DROP_EXPRESSION
    // DROP_IDENTITY
    // DROP_NOT_NULL
    // RESTART
    // ADD_GENERATED
    // RESET_OPTIONS
    // SET_TYPE
    // SET_GENERATED_OPTIONS
    // SET_GENERATED
    // SET_SEQUENCE_OPTION
    // SET_DEFAULT
    // SET_EXPRESSION
    // SET_STATISTICS
    // SET_OPTIONS
    // SET_OPTIONS_LIST
    // SET_STORAGE
    // SET_COMPRESSION
    // SET_NOT_NULL
    // TABLE_ARGS
    // COLUMN_LIST
    // WHEN_CLAUSE
    // USING_CLAUSE
    // WITHIN_CLAUSE
    // FILTER_CLAUSE
    // OVER_CLAUSE
    // DISTINCT_CLAUSE
    // WITH_TABLE
    // WITH_CLAUSE
    // FROM_CLAUSE
    // WHERE_CLAUSE
    // GROUP_BY_CLAUSE
    // HAVING_CLAUSE
    // WINDOW_CLAUSE
    // LIMIT_CLAUSE
    // OFFSET_CLAUSE
    // ORDER_BY_CLAUSE
    // LOCKING_CLAUSE
    // TUPLE_EXPR
    // NAME_REF
    // "
    //     .trim()
    //     .lines();

    //     let nodes = nodes.map(|x| format_ident!("{}", x)).collect::<Vec<_>>();

    let (single_byte_tokens_values, single_byte_tokens): (Vec<_>, Vec<_>) = grammar
        .punct
        .iter()
        .filter(|(token, _name)| token.len() == 1)
        .map(|(token, name)| (token.chars().next().unwrap(), format_ident!("{}", name)))
        .unzip();

    let punctuation_values = grammar.punct.iter().map(|(token, _name)| {
        if "{}[]()".contains(token) {
            let c = token.chars().next().unwrap();
            quote! { #c }
        } else {
            let cs = token.chars().map(|c| Punct::new(c, Spacing::Joint));
            quote! { #(#cs)* }
        }
    });
    let punctuation = grammar
        .punct
        .iter()
        .map(|(_token, name)| format_ident!("{}", name))
        .collect::<Vec<_>>();

    let x = |&name| match name {
        "Self" => format_ident!("SELF_TYPE_KW"),
        name => format_ident!("{}_KW", name.to_case(Case::UpperSnake)),
    };
    let full_keywords_values = grammar.keywords;
    let full_keywords = full_keywords_values.iter().map(x);

    let contextual_keywords_values = &grammar.contextual_keywords;
    let contextual_keywords = contextual_keywords_values.iter().map(x);

    let all_keywords_values = grammar
        .keywords
        .iter()
        .chain(grammar.contextual_keywords.iter())
        .copied()
        .collect::<Vec<_>>();
    let all_keywords_idents = all_keywords_values.iter().map(|kw| format_ident!("{}", kw));
    let all_keywords = all_keywords_values.iter().map(x).collect::<Vec<_>>();

    let literals = grammar
        .literals
        .iter()
        .map(|name| format_ident!("{}", name))
        .collect::<Vec<_>>();

    let tokens = grammar
        .tokens
        .iter()
        .map(|name| format_ident!("{}", name))
        .collect::<Vec<_>>();

    let nodes = grammar
        .nodes
        .iter()
        .map(|name| format_ident!("{}", name))
        .collect::<Vec<_>>();

    Ok(reformat(reformat(
        quote! {
            #![allow(bad_style, missing_docs, clippy::upper_case_acronyms)]

            #[doc = r"The kind of syntax node, e.g. `IDENT`, `SELECT_KW`, or `WHERE_CLAUSE`. Needs to be compatible with [`rowan::SyntaxKind`]"]
            #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
            #[repr(u16)]
            pub enum SyntaxKind {
                #[doc(hidden)]
                TOMBSTONE,
                #[doc(hidden)]
                EOF,

                // #[space_hack]
                // #(#symbols,)*

                // #[space_hack]
                // #(#keywords,)*

                // #[space_hack]
                // #(#literals,)*

                // #[space_hack]
                // #(#nodes,)*
                #(#punctuation,)*
                #(#all_keywords,)*
                #(#literals,)*
                #(#tokens,)*
                #(#nodes,)*

                #[space_hack]
                #[doc(hidden)]
                __LAST,
            }

            #[space_hack]
            impl SyntaxKind {
                pub(crate) fn from_keyword(ident: &str) -> Option<SyntaxKind> {
                    let kw = #(#conditions)* else {
                        return None;
                    };
                    Some(kw)
                }
            }
        }
        .to_string(),
    ).replace("#[space_hack]", "")))
}

/// related:
///   - [postgres/src/backend/utils/adt/misc.c](https://github.com/postgres/postgres/blob/08691ea958c2646b6aadefff878539eb0b860bb0/src/backend/utils/adt/misc.c#L452-L467/)
///   - [postgres docs: sql keywords appendix](https://www.postgresql.org/docs/17/sql-keywords-appendix.html)
///
/// The header file isn't enough though because `json_scalar` can be a function
/// name, but `between` cannot be
///
/// The Postgres parser special cases certain calls like `json_scalar`:
/// <https://github.com/postgres/postgres/blob/028b4b21df26fee67b3ce75c6f14fcfd3c7cf2ee/src/backend/parser/gram.y#L15684C8-L16145>
///
/// | Category     | Column | Table | Function | Type |
/// |--------------|--------|-------|----------|------|
/// | Unreserved   | Y      | Y     | Y        | Y    |
/// | Reserved     | N      | N     | N        | N    |
/// | ColName      | Y      | Y     | N        | Y    |
/// | TypeFuncName | N      | N     | Y        | Y    |
///
#[derive(Clone, Copy)]
enum KeywordCategory {
    Unreserved,
    Reserved,
    ColName,
    TypeFuncName,
}

enum KeywordLabel {
    As,
    Bare,
}

struct KeywordMeta {
    category: KeywordCategory,
    label: KeywordLabel,
}

fn parse_header() -> Result<HashMap<String, KeywordMeta>> {
    let kwlist_file = project_root().join("postgres/kwlist.h");
    let data = std::fs::read_to_string(kwlist_file).context("Failed to read kwlist.h")?;

    let mut keywords = HashMap::new();

    for line in data.lines() {
        if line.starts_with("PG_KEYWORD") {
            let line = line
                .split(&['(', ')'])
                .nth(1)
                .context("Invalid kwlist.h structure")?;

            let row_items: Vec<&str> = line.split(',').collect();

            match row_items[..] {
                [name, _value, category, is_bare_label] => {
                    let label = match is_bare_label.trim() {
                        "AS_LABEL" => KeywordLabel::As,
                        "BARE_LABEL" => KeywordLabel::Bare,
                        unexpected => anyhow::bail!("Unexpected label: {}", unexpected),
                    };

                    let category = match category.trim() {
                        "UNRESERVED_KEYWORD" => KeywordCategory::Unreserved,
                        "RESERVED_KEYWORD" => KeywordCategory::Reserved,
                        "COL_NAME_KEYWORD" => KeywordCategory::ColName,
                        "TYPE_FUNC_NAME_KEYWORD" => KeywordCategory::TypeFuncName,
                        unexpected => anyhow::bail!("Unexpected category: {}", unexpected),
                    };

                    let meta = KeywordMeta { category, label };
                    let name = name.trim().replace('\"', "");
                    keywords.insert(name, meta);
                }
                _ => bail!("Problem reading kwlist.h row"),
            }
        }
    }

    Ok(keywords)
}

#[derive(Sequence, PartialEq)]
enum KWType {
    ColumnTable,
    Type,
}

impl std::fmt::Display for KWType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            KWType::ColumnTable => "COLUMN_OR_TABLE_KEYWORDS",
            KWType::Type => "TYPE_KEYWORDS",
        })
    }
}

fn keyword_allowed(cat: KeywordCategory, kw_type: KWType) -> bool {
    match cat {
        KeywordCategory::Unreserved => match kw_type {
            KWType::ColumnTable => true,
            KWType::Type => true,
        },
        KeywordCategory::Reserved => match kw_type {
            KWType::ColumnTable => false,
            KWType::Type => false,
        },
        KeywordCategory::ColName => match kw_type {
            KWType::ColumnTable => true,
            KWType::Type => true,
        },
        KeywordCategory::TypeFuncName => match kw_type {
            KWType::ColumnTable => false,
            KWType::Type => true,
        },
    }
}

fn generate_token_sets() -> Result<String> {
    let keywords = parse_header()?;
    let mut bare_label_keywords = keywords
        .iter()
        .filter(|(_key, value)| match value.label {
            KeywordLabel::As => false,
            KeywordLabel::Bare => true,
        })
        .map(|(key, _value)| format_ident!("{}_KW", key.to_case(Case::UpperSnake)))
        .collect::<Vec<_>>();
    bare_label_keywords.sort();

    let mut unreserved_keywords = keywords
        .iter()
        .filter(|(_key, value)| matches!(value.category, KeywordCategory::Unreserved))
        .map(|(key, _value)| format_ident!("{}_KW", key.to_case(Case::UpperSnake)))
        .collect::<Vec<_>>();
    unreserved_keywords.sort();

    let mut reserved_keywords = keywords
        .iter()
        .filter(|(_key, value)| matches!(value.category, KeywordCategory::Reserved))
        .map(|(key, _value)| format_ident!("{}_KW", key.to_case(Case::UpperSnake)))
        .collect::<Vec<_>>();
    reserved_keywords.sort();

    let mut all_keywords = keywords
        .iter()
        .map(|(key, _value)| format_ident!("{}_KW", key.to_case(Case::UpperSnake)))
        .collect::<Vec<_>>();
    all_keywords.sort();

    let mut col_table_tokens = HashSet::new();
    let mut type_tokens = HashSet::new();
    for (key, meta) in &keywords {
        for variant in all::<KWType>() {
            match variant {
                KWType::ColumnTable => {
                    if keyword_allowed(meta.category, variant) {
                        col_table_tokens.insert(key);
                    }
                }
                KWType::Type => {
                    if keyword_allowed(meta.category, variant) {
                        type_tokens.insert(key);
                    }
                }
            }
        }
    }

    let mut column_or_table_keywords = col_table_tokens
        .iter()
        .map(|key| format_ident!("{}_KW", key.to_case(Case::UpperSnake)))
        .collect::<Vec<_>>();
    column_or_table_keywords.sort();

    let mut type_keywords = type_tokens
        .iter()
        .map(|key| format_ident!("{}_KW", key.to_case(Case::UpperSnake)))
        .collect::<Vec<_>>();
    type_keywords.sort();

    Ok(reformat(
        quote! {
            use crate::syntax_kind::SyntaxKind;
            use crate::token_set::TokenSet;

            pub(crate) const COLUMN_OR_TABLE_KEYWORDS: TokenSet = TokenSet::new(&[
                #(SyntaxKind::#column_or_table_keywords),*
            ]);

            pub(crate) const TYPE_KEYWORDS: TokenSet = TokenSet::new(&[
                #(SyntaxKind::#type_keywords),*
            ]);

            pub(crate) const ALL_KEYWORDS: TokenSet = TokenSet::new(&[
                #(SyntaxKind::#all_keywords),*
            ]);

            pub(crate) const BARE_LABEL_KEYWORDS: TokenSet = TokenSet::new(&[
                #(SyntaxKind::#bare_label_keywords),*
            ]);

            pub(crate) const UNRESERVED_KEYWORDS: TokenSet = TokenSet::new(&[
                #(SyntaxKind::#unreserved_keywords),*
            ]);

            pub(crate) const RESERVED_KEYWORDS: TokenSet = TokenSet::new(&[
                #(SyntaxKind::#reserved_keywords),*
            ]);
        }
        .to_string(),
    )
    .replace("pub(crate)", "\npub(crate)"))
}

#[derive(Debug, Default)]
struct AstSrc {
    tokens: Vec<(&'static str, &'static str)>,
    nodes: Vec<AstNodeSrc>,
    enums: Vec<AstEnumSrc>,
}

#[derive(Debug)]
struct AstNodeSrc {
    name: String,
    fields: Vec<Field>,
}

#[derive(Debug, Eq, PartialEq)]
enum Field {
    Token(String),
    Node {
        name: String,
        ty: String,
        cardinality: Cardinality,
    },
}

fn token_to_name(tk: &str) -> Option<&'static str> {
    let name = match tk {
        ";" => "semicolon",
        "'{'" => "l_curly",
        "'}'" => "r_curly",
        "'('" => "l_paren",
        "(" => "l_paren",
        "')'" => "r_paren",
        "ident" => "ident",
        ")" => "r_paren",
        "'['" => "l_brack",
        "[" => "l_brack",
        "']'" => "r_brack",
        "]" => "r_brack",
        "<" => "l_angle",
        ">" => "r_angle",
        "=" => "eq",
        "!" => "bang",
        "*" => "star",
        "&" => "amp",
        "+" => "plus",
        "/" => "slash",
        "^" => "caret",
        "`" => "backtick",
        "-" => "minus",
        "_" => "underscore",
        "." => "dot",
        "=>" => "fat_arrow",
        "@" => "at",
        "%" => "percent",
        ":" => "colon",
        "::" => "coloncolon",
        "#" => "pound",
        "?" => "question",
        "," => "comma",
        "|" => "pipe",
        "~" => "tilde",
        _ => return None,
    };
    Some(name)
}

impl Field {
    fn is_many(&self) -> bool {
        matches!(
            self,
            Field::Node {
                cardinality: Cardinality::Many,
                ..
            }
        )
    }
    fn token_kind(&self) -> Option<proc_macro2::Ident> {
        match self {
            Field::Token(token) => match token_to_name(token) {
                Some(token) => Some(format_ident!("{}", token.to_case(Case::UpperSnake))),
                None => Some(format_ident!("{}_KW", token.to_case(Case::UpperSnake))),
            },
            _ => None,
        }
    }
    fn method_name(&self) -> String {
        match self {
            Field::Token(name) => {
                let name = token_to_name(name).unwrap_or(name);
                format!("{name}_token",)
            }
            Field::Node { name, .. } => {
                if name == "type" {
                    String::from("ty")
                } else {
                    name.to_owned()
                }
            }
        }
    }
    fn ty(&self) -> proc_macro2::Ident {
        match self {
            Field::Token(_) => format_ident!("SyntaxToken"),
            Field::Node { ty, .. } => format_ident!("{}", ty),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Cardinality {
    Optional,
    Many,
}

#[derive(Debug)]
struct AstEnumSrc {
    name: String,
    variants: Vec<String>,
}

fn lower(grammar: &Grammar) -> AstSrc {
    let tokens = vec![("Null", "NULL_KW"), ("String", "STRING")];
    let mut res = AstSrc {
        tokens,
        ..Default::default()
    };

    let grammar_nodes = grammar.iter().collect::<Vec<_>>();
    for node in grammar_nodes {
        let name = grammar[node].name.clone();
        let rule = &grammar[node].rule;
        match lower_enum(&grammar, rule) {
            Some(variants) => {
                let enum_src = AstEnumSrc {
                    // doc: Vec::new(),
                    name,
                    // traits: Vec::new(),
                    variants,
                };
                res.enums.push(enum_src);
            }
            None => {
                let mut fields = Vec::new();
                lower_rule(&mut fields, &grammar, None, rule);
                res.nodes.push(AstNodeSrc {
                    // doc: Vec::new(),
                    name,
                    // traits: Vec::new(),
                    fields,
                });
            }
        }
    }

    deduplicate_fields(&mut res);
    res.nodes.sort_by_key(|it| it.name.clone());
    res.enums.sort_by_key(|it| it.name.clone());
    res.tokens.sort();
    res.nodes.iter_mut().for_each(|it| {
        // it.traits.sort();
        it.fields.sort_by_key(|it| match it {
            Field::Token(name) => (true, name.clone()),
            Field::Node { name, .. } => (false, name.clone()),
        });
    });
    res.enums.iter_mut().for_each(|it| {
        // it.traits.sort();
        it.variants.sort();
    });

    res
}

fn deduplicate_fields(ast: &mut AstSrc) {
    for node in &mut ast.nodes {
        let mut i = 0;
        'outer: while i < node.fields.len() {
            for j in 0..i {
                let f1 = &node.fields[i];
                let f2 = &node.fields[j];
                if f1 == f2 {
                    node.fields.remove(i);
                    continue 'outer;
                }
            }
            i += 1;
        }
    }
}

fn lower_enum(grammar: &Grammar, rule: &Rule) -> Option<Vec<String>> {
    let alternatives = match rule {
        Rule::Alt(it) => it,
        _ => return None,
    };
    let mut variants = Vec::new();
    for alternative in alternatives {
        match alternative {
            Rule::Node(it) => variants.push(grammar[*it].name.clone()),
            Rule::Token(it) if grammar[*it].name == ";" => (),
            _ => return None,
        }
    }
    Some(variants)
}

fn lower_rule(acc: &mut Vec<Field>, grammar: &Grammar, label: Option<&String>, rule: &Rule) {
    if lower_separated_list(acc, grammar, label, rule) {
        return;
    }

    match rule {
        Rule::Node(node) => {
            let ty = grammar[*node].name.clone();
            let name = label.cloned().unwrap_or_else(|| ty.to_case(Case::Snake));
            let field = Field::Node {
                name,
                ty,
                cardinality: Cardinality::Optional,
            };
            acc.push(field);
        }
        Rule::Token(token) => {
            assert!(label.is_none());
            let mut name = clean_token_name(&grammar[*token].name);
            if "[]{}()".contains(&name) {
                name = format!("'{name}'");
            }
            let field = Field::Token(name);
            acc.push(field);
        }
        Rule::Rep(inner) => {
            let rule = &**inner;
            if let Rule::Node(node) = rule {
                let ty = grammar[*node].name.clone();
                let name = label
                    .cloned()
                    .unwrap_or_else(|| pluralize(&ty.to_case(Case::Snake)));
                let field = Field::Node {
                    name,
                    ty,
                    cardinality: Cardinality::Many,
                };
                acc.push(field);
                return;
            }
            panic!("unhandled rule: {rule:?}. Instead of `'foo' T (',', T)`, write `'foo' (T (',', T))`");
        }
        Rule::Labeled { label: l, rule } => {
            assert!(label.is_none());
            let manually_implemented = matches!(
                l.as_str(),
                "lhs"
                    | "rhs"
                    | "then_branch"
                    | "else_branch"
                    | "start"
                    | "end"
                    | "op"
                    | "index"
                    | "base"
                    | "value"
                    | "trait"
                    | "self_ty"
                    | "iterable"
                    | "condition"
                    | "args"
                    | "body"
            );
            if manually_implemented {
                return;
            }
            lower_rule(acc, grammar, Some(l), rule);
        }
        Rule::Seq(rules) | Rule::Alt(rules) => {
            for rule in rules {
                lower_rule(acc, grammar, label, rule)
            }
        }
        Rule::Opt(rule) => lower_rule(acc, grammar, label, rule),
    }
}

fn clean_token_name(name: &str) -> String {
    let cleaned = name.trim_start_matches(['@', '#', '?']);
    if cleaned.is_empty() {
        name.to_owned()
    } else {
        cleaned.to_owned()
    }
}

fn pluralize(s: &str) -> String {
    format!("{s}s")
}

// (T (',' T)* ','?)
fn lower_separated_list(
    acc: &mut Vec<Field>,
    grammar: &Grammar,
    label: Option<&String>,
    rule: &Rule,
) -> bool {
    let rule = match rule {
        Rule::Seq(it) => it,
        _ => return false,
    };
    let (node, repeat, trailing_sep) = match rule.as_slice() {
        [Rule::Node(node), Rule::Rep(repeat), Rule::Opt(trailing_sep)] => {
            (node, repeat, Some(trailing_sep))
        }
        [Rule::Node(node), Rule::Rep(repeat)] => (node, repeat, None),
        _ => return false,
    };
    let repeat = match &**repeat {
        Rule::Seq(it) => it,
        _ => return false,
    };
    if !matches!(
        repeat.as_slice(),
        [comma, Rule::Node(n)]
            if trailing_sep.map_or(true, |it| comma == &**it) && n == node
    ) {
        return false;
    }
    let ty = grammar[*node].name.clone();
    let name = label
        .cloned()
        .unwrap_or_else(|| pluralize(&ty.to_case(Case::Snake)));
    let field = Field::Node {
        name,
        ty,
        cardinality: Cardinality::Many,
    };
    acc.push(field);
    true
}

fn generate_nodes(nodes: &[AstNodeSrc], enums: &[AstEnumSrc]) -> String {
    let (nodes, node_boilerplate_impls): (Vec<_>, Vec<_>) = nodes
        .iter()
        .map(|node| {
            let name = format_ident!("{}", node.name.to_case(Case::Pascal));
            let token = format_ident!("{}", node.name.to_case(Case::UpperSnake));
            let methods = node.fields.iter().map(|field| {
                let method_name = format_ident!("{}", field.method_name());
                let ty = field.ty();

                if field.is_many() {
                    quote! {
                        #[inline]
                        pub fn #method_name(&self) -> AstChildren<#ty> {
                            support::children(&self.syntax)
                        }
                    }
                } else if let Some(token_kind) = field.token_kind() {
                    quote! {
                        #[inline]
                        pub fn #method_name(&self) -> Option<#ty> {
                            support::token(&self.syntax, SyntaxKind::#token_kind)
                        }
                    }
                } else {
                    quote! {
                        #[inline]
                        pub fn #method_name(&self) -> Option<#ty> {
                            support::child(&self.syntax)
                        }
                    }
                }
            });
            (
                quote! {
                    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
                    pub struct #name {
                        pub(crate) syntax: SyntaxNode,
                    }
                    impl #name {
                        #(#methods)*
                    }
                },
                quote! {
                    impl AstNode for #name {
                        #[inline]
                        fn can_cast(kind: SyntaxKind) -> bool {
                            kind == SyntaxKind::#token
                        }
                        #[inline]
                        fn cast(syntax: SyntaxNode) -> Option<Self> {
                            if Self::can_cast(syntax.kind()) {
                                Some(Self { syntax })
                            } else {
                                None
                            }
                        }
                        #[inline]
                        fn syntax(&self) -> &SyntaxNode {
                            &self.syntax
                        }
                    }
                },
            )
        })
        .unzip();

    let (enums, enums_boilierplate_impls): (Vec<_>, Vec<_>) = enums
        .iter()
        .map(|e| {
            let enum_ = e;
            let variants = enum_
                .variants
                .iter()
                .map(|x| format_ident!("{}", x))
                .collect::<Vec<_>>();
            let kinds = variants
                .iter()
                .map(|name| format_ident!("{}", name.to_string().to_case(Case::UpperSnake)))
                .collect::<Vec<_>>();
            let name = format_ident!("{}", enum_.name);
            (
                quote! {
                  #[derive(Debug, Clone, PartialEq, Eq, Hash)]
                  pub enum #name {
                    #(#variants(#variants),)*
                  }
                },
                quote! {
                  impl AstNode for #name {
                    #[inline]
                    fn can_cast(kind: SyntaxKind) -> bool {
                        matches!(kind, #(SyntaxKind::#kinds)|*)
                    }
                    #[inline]
                    fn cast(syntax: SyntaxNode) -> Option<Self> {
                        let res = match syntax.kind() {
                            #(
                            SyntaxKind::#kinds => #name::#variants(#variants { syntax }),
                            )*
                            _ => return None,
                        };
                        Some(res)
                    }
                    #[inline]
                    fn syntax(&self) -> &SyntaxNode {
                        match self {
                            #(
                            #name::#variants(it) => &it.syntax,
                            )*
                        }
                    }
                  }
                  #(
                      impl From<#variants> for #name {
                          #[inline]
                          fn from(node: #variants) -> #name {
                              #name::#variants(node)
                          }
                      }
                  )*
                },
            )
        })
        .unzip();

    let file = quote! {
        use crate::ast::AstNode;
        use crate::syntax_node::SyntaxNode;
        use crate::syntax_node::SyntaxToken;
        use crate::ast::{support, AstChildren};
        use crate::SyntaxKind;
        #(#nodes)*
        #(#enums)*

        #(#node_boilerplate_impls)*
        #(#enums_boilierplate_impls)*
    };
    reformat(file.to_string()).replace("#[derive", "\n#[derive")
}

fn generate_tokens(tokens: &[(&'static str, &'static str)]) -> String {
    let tokens = tokens.iter().map(|(name, kind)| {
        let name = format_ident!("{}", name.to_case(Case::Pascal));
        let kind = format_ident!("{}", kind.to_case(Case::Constant));
        quote! {
            #[derive(Debug, Clone, PartialEq, Eq, Hash)]
            pub struct #name {
                pub(crate) syntax: SyntaxToken,
            }
            impl std::fmt::Display for #name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    std::fmt::Display::fmt(&self.syntax, f)
                }
            }
            impl AstToken for #name {
                fn can_cast(kind: SyntaxKind) -> bool {
                    kind == SyntaxKind::#kind
                }
                fn cast(syntax: SyntaxToken) -> Option<Self> {
                    if Self::can_cast(syntax.kind()) {
                        Some(Self { syntax })
                    } else {
                        None
                    }
                }
                fn syntax(&self) -> &SyntaxToken {
                    &self.syntax
                }
            }
        }
    });

    let file = quote! {
        use std::{fmt, hash};

        use crate::{SyntaxKind, SyntaxToken, ast::AstToken};

        #(#tokens)*
    };

    reformat(file.to_string()).replace("#[derive", "\n#[derive")
}
