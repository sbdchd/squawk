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

use std::collections::HashSet;

use anyhow::{Context, Result};
use convert_case::{Case, Casing};
use quote::{format_ident, quote};
use ungrammar::{Grammar, Rule};
use xshell::{Shell, cmd};

use crate::{
    keywords::{KeywordKinds, keyword_kinds},
    path::project_root,
};

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
    let mut stdout = cmd!(sh, "rustup run stable rustfmt --edition=2024")
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

    let keyword_kinds = keyword_kinds()?;

    let token_sets = generate_token_sets(&keyword_kinds)?;
    let token_sets_file = project_root().join("crates/squawk_parser/src/generated/token_sets.rs");
    std::fs::write(token_sets_file, token_sets).context("problem writing generated token sets")?;

    let kinds = generate_kind_src(&ast_src.nodes, &grammar, keyword_kinds.all_keywords);

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
    (":", "COLON"),
    ("=", "EQ"),
    ("!", "BANG"),
    ("-", "MINUS"),
    ("`", "BACKTICK"),
];

fn generate_kind_src(
    nodes: &[AstNodeSrc],
    grammar: &ungrammar::Grammar,
    pg_keywords: Vec<String>,
) -> KindsSrc {
    let mut keywords: Vec<&_> = Vec::new();
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
            _ if name.chars().all(|c| c.is_alphabetic() || c == '_') => {
                // pass
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
    keywords.extend(pg_keywords.into_iter().map(|s| &*s.leak()));
    keywords.sort();
    keywords.dedup();

    // we leak things here for simplicity, that way we don't have to deal with lifetimes
    // The execution is a one shot job so thats fine
    let nodes = nodes
        .iter()
        .map(|it| &it.name)
        // We don't include enums since they don't compare against a specific syntax kind
        // .chain(enums.iter().map(|it| &it.name))
        .map(|it| it.to_case(Case::UpperSnake))
        .map(String::leak)
        .map(|it| &*it)
        .collect();
    let nodes = Vec::leak(nodes);
    nodes.sort();
    let keywords = Vec::leak(keywords);
    let literals = Vec::leak(literals);
    literals.sort();
    let tokens = Vec::leak(tokens);
    tokens.sort();

    KindsSrc {
        punct: PUNCT,
        nodes,
        keywords,
        literals,
        tokens,
    }
}

fn generate_syntax_kinds(grammar: KindsSrc) -> Result<String> {
    // TODO: we should have a check to make sure each keyword is used in the grammar once the grammar is ready
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

    let punctuation = grammar
        .punct
        .iter()
        .map(|(_token, name)| format_ident!("{}", name))
        .collect::<Vec<_>>();

    let x = |&name| match name {
        "Self" => format_ident!("SELF_TYPE_KW"),
        name => format_ident!("{}_KW", name.to_case(Case::UpperSnake)),
    };

    let all_keywords_values = grammar.keywords.to_vec();
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

fn generate_token_sets(keyword_kinds: &KeywordKinds) -> Result<String> {
    let column_or_table_keywords = keyword_kinds
        .col_table_keywords
        .iter()
        .map(|key| format_ident!("{}_KW", key.to_case(Case::UpperSnake)))
        .collect::<Vec<_>>();

    let type_keywords = keyword_kinds
        .type_keywords
        .iter()
        .map(|key| format_ident!("{}_KW", key.to_case(Case::UpperSnake)))
        .collect::<Vec<_>>();

    let all_keywords = &keyword_kinds
        .all_keywords
        .iter()
        .map(|key| format_ident!("{}_KW", key.to_case(Case::UpperSnake)))
        .collect::<Vec<_>>();
    let bare_label_keywords = &keyword_kinds
        .bare_label_keywords
        .iter()
        .map(|key| format_ident!("{}_KW", key.to_case(Case::UpperSnake)))
        .collect::<Vec<_>>();
    let unreserved_keywords = &keyword_kinds
        .unreserved_keywords
        .iter()
        .map(|key| format_ident!("{}_KW", key.to_case(Case::UpperSnake)))
        .collect::<Vec<_>>();
    let reserved_keywords = &keyword_kinds
        .reserved_keywords
        .iter()
        .map(|key| format_ident!("{}_KW", key.to_case(Case::UpperSnake)))
        .collect::<Vec<_>>();

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
        match lower_enum(grammar, rule) {
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
                lower_rule(&mut fields, grammar, None, rule);
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
    let Rule::Alt(alternatives) = rule else {
        return None;
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
            panic!(
                "unhandled rule: {rule:?}. Instead of `'foo' T (',', T)`, write `'foo' (T (',', T))`"
            );
        }
        Rule::Labeled { label: l, rule } => {
            assert!(label.is_none());
            let manually_implemented = matches!(l.as_str(), "value" | "lhs" | "rhs");
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
    let Rule::Seq(rule) = rule else {
        return false;
    };
    let (node, repeat, trailing_sep) = match rule.as_slice() {
        [Rule::Node(node), Rule::Rep(repeat), Rule::Opt(trailing_sep)] => {
            (node, repeat, Some(trailing_sep))
        }
        [Rule::Node(node), Rule::Rep(repeat)] => (node, repeat, None),
        _ => return false,
    };
    let Rule::Seq(repeat) = &**repeat else {
        return false;
    };
    if !matches!(
        repeat.as_slice(),
        [comma, Rule::Node(n)]
            if trailing_sep.is_none_or(|it| comma == &**it) && n == node
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

    let enum_nodes = enums.iter().map(|x| &x.name).collect::<HashSet<_>>();

    let (enums, enums_boilierplate_impls): (Vec<_>, Vec<_>) = enums
        .iter()
        .map(|e| {
            let cast_variants = e
                .variants
                .iter()
                .filter(|x| enum_nodes.contains(x))
                .map(|x| format_ident!("{}", x))
                .collect::<Vec<_>>();
            let variants = e
                .variants
                .iter()
                .filter(|x| !enum_nodes.contains(x))
                .map(|x| format_ident!("{}", x))
                .collect::<Vec<_>>();
            let kinds = variants
                .iter()
                .map(|name| format_ident!("{}", name.to_string().to_case(Case::UpperSnake)))
                .collect::<Vec<_>>();
            let name = format_ident!("{}", e.name);

            (
                quote! {
                  #[derive(Debug, Clone, PartialEq, Eq, Hash)]
                  pub enum #name {
                    #(#variants(#variants),)*
                    #(#cast_variants(#cast_variants),)*
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
                            _ => {
                                #(
                                    if let Some(result) = #cast_variants::cast(syntax) {
                                        return Some(#name::#cast_variants(result));
                                    }
                                )*
                                return None;
                            }
                        };
                        Some(res)
                    }
                    #[inline]
                    fn syntax(&self) -> &SyntaxNode {
                        match self {
                            #(
                            #name::#variants(it) => &it.syntax,
                            )*
                            #(
                            #name::#cast_variants(it) => it.syntax(),
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
        use crate::{SyntaxKind, SyntaxToken, ast::AstToken};

        #(#tokens)*
    };

    reformat(file.to_string()).replace("#[derive", "\n#[derive")
}
