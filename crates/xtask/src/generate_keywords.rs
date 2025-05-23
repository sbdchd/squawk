use crate::path_util::cwd_to_workspace_root;
use anyhow::{Context, Ok, Result};
use enum_iterator::{all, Sequence};
use std::collections::{HashMap, HashSet};

struct KeywordMeta {
    category: KeywordCategory,
    label: KeywordLabel,
}

enum KeywordLabel {
    As,
    Bare,
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

pub(crate) fn generate_keywords() -> Result<()> {
    let keywords = parse_header()?;

    update_syntax_kind(&keywords)
}

fn update_syntax_kind(keywords: &HashMap<String, KeywordMeta>) -> Result<()> {
    let path = "crates/squawk_parser/src/syntax_kind.rs";

    let data = std::fs::read_to_string(path).context("opening syntax_kind.rs")?;

    let mut keys: Vec<_> = keywords.keys().collect();
    keys.sort();

    let keywords_start = "// keywords";
    let keywords_end = "// literals";
    let mut in_keywords = false;

    let from_kw_start = "pub(crate) fn from_keyword";
    let from_kw_end = "} else {";
    let mut in_from_keyword = false;
    let mut is_first_from_keyword_case = true;

    let token_set_start = "// Generated TokenSet start";
    let token_set_end = "// Generated TokenSet end";
    let mut in_token_sets = false;

    let mut allowed_col_table_tokens = HashSet::new();
    let mut allowed_type_tokens = HashSet::new();
    let mut bare_label_keywords = keywords
        .iter()
        .filter(|(_key, value)| match value.label {
            KeywordLabel::As => false,
            KeywordLabel::Bare => true,
        })
        .map(|(key, _value)| key)
        .collect::<Vec<_>>();
    bare_label_keywords.sort();

    let mut unreserved_keywords = keywords
        .iter()
        .filter(|(_key, value)| matches!(value.category, KeywordCategory::Unreserved))
        .map(|(key, _value)| key)
        .collect::<Vec<_>>();
    unreserved_keywords.sort();

    let mut reserved_keywords = keywords
        .iter()
        .filter(|(_key, value)| matches!(value.category, KeywordCategory::Reserved))
        .map(|(key, _value)| key)
        .collect::<Vec<_>>();
    reserved_keywords.sort();

    let mut all_keywords = keywords.iter().map(|(key, _value)| key).collect::<Vec<_>>();
    all_keywords.sort();

    for (key, meta) in keywords {
        for variant in all::<KWType>() {
            match variant {
                KWType::ColumnTable => {
                    if keyword_allowed(meta.category, variant) {
                        allowed_col_table_tokens.insert(key);
                    }
                }
                KWType::Type => {
                    if keyword_allowed(meta.category, variant) {
                        allowed_type_tokens.insert(key);
                    }
                }
            }
        }
    }

    let mut out = vec![];

    for line in data.lines() {
        if line.contains(keywords_end) {
            for kw in &keys {
                // /// `column`
                // COLUMN_KW,
                let comment = format!("    /// `{}`\n", kw);
                let ident = format!("    {},", kw.to_uppercase() + "_KW");
                out.push(comment + &ident);
            }
            out.push("".to_string());

            in_keywords = false;
        } else if line.contains(from_kw_end) {
            let mut keys: Vec<_> = keywords.keys().collect();
            keys.sort();
            for kw in keys {
                // } else if ident.eq_ignore_ascii_case("when") {
                //     SyntaxKind::WHEN_KW
                let cond_op = if is_first_from_keyword_case {
                    "let kw = if"
                } else {
                    "} else if"
                };

                let cond = format!(
                    r#"        {} ident.eq_ignore_ascii_case("{}") {{"#,
                    cond_op, kw
                ) + "\n";
                let ident = format!("            SyntaxKind::{}", kw.to_uppercase() + "_KW");
                out.push(cond + &ident);

                is_first_from_keyword_case = false;
            }

            in_from_keyword = false;
        } else if line.contains(token_set_end) {
            for variant in all::<KWType>() {
                out.push(format!(
                    "pub(crate) const {}: TokenSet = TokenSet::new(&[",
                    variant
                ));
                let mut tokens = match variant {
                    KWType::ColumnTable => &allowed_col_table_tokens,
                    KWType::Type => &allowed_type_tokens,
                }
                .iter()
                .collect::<Vec<_>>();

                tokens.sort();

                for tk in tokens {
                    out.push(format!("    SyntaxKind::{},", tk.to_uppercase() + "_KW"));
                }
                out.push("]);".to_string());
                out.push("".to_string());
            }

            // all keywords
            {
                out.push("pub(crate) const ALL_KEYWORDS: TokenSet = TokenSet::new(&[".to_string());
                let tokens = &all_keywords;
                for tk in tokens {
                    out.push(format!("    SyntaxKind::{},", tk.to_uppercase() + "_KW"));
                }
                out.push("]);".to_string());
                out.push("".to_string());
            }

            {
                out.push(
                    "pub(crate) const BARE_LABEL_KEYWORDS: TokenSet = TokenSet::new(&[".to_string(),
                );
                for tk in &bare_label_keywords {
                    out.push(format!("    SyntaxKind::{},", tk.to_uppercase() + "_KW"));
                }
                out.push("]);".to_string());
                out.push("".to_string());
            }

            {
                out.push(
                    "pub(crate) const UNRESERVED_KEYWORDS: TokenSet = TokenSet::new(&[".to_string(),
                );
                let tokens = &unreserved_keywords;
                for tk in tokens {
                    out.push(format!("    SyntaxKind::{},", tk.to_uppercase() + "_KW"));
                }
                out.push("]);".to_string());
                out.push("".to_string());
            }

            {
                out.push(
                    "pub(crate) const RESERVED_KEYWORDS: TokenSet = TokenSet::new(&[".to_string(),
                );
                let tokens = &reserved_keywords;
                for tk in tokens {
                    out.push(format!("    SyntaxKind::{},", tk.to_uppercase() + "_KW"));
                }
                out.push("]);".to_string());
                out.push("".to_string());
            }

            out.push(line.to_string());
        }
        if !in_keywords && !in_from_keyword && !in_token_sets {
            out.push(line.to_string());
        }
        if line.contains(keywords_start) {
            in_keywords = true;
        } else if line.contains(from_kw_start) {
            in_from_keyword = true;
        } else if line.contains(token_set_start) {
            in_token_sets = true;
        }
    }

    std::fs::write(path, out.join("\n") + "\n").context("writing to syntax_kind.rs")
}

fn parse_header() -> Result<HashMap<String, KeywordMeta>> {
    cwd_to_workspace_root().context("Failed to cwd to root")?;

    let data = std::fs::read_to_string("postgres/kwlist.h").context("Failed to read kwlist.h")?;

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
                _ => anyhow::bail!("Problem reading kwlist.h row"),
            }
        }
    }

    Ok(keywords)
}
