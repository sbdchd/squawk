use anyhow::Result;
use camino::Utf8PathBuf;
use convert_case::{Case, Casing};

use crate::NewRuleArgs;
use std::{env, fs};

fn make_lint(name: &str) -> String {
    let rule_name_snake = name.to_case(Case::Snake);
    let rule_name_pascal = name.to_case(Case::Pascal);
    format!(
        r###"
use squawk_syntax::{{
    ast::{{self, AstNode, HasModuleItem}},
    Parse, SourceFile,
}};

use crate::{{Linter, Violation, Rule}};

pub(crate) fn {rule_name_snake}(ctx: &mut Linter, parse: &Parse<SourceFile>) {{
    let file = parse.tree();
    for item in file.items() {{
        match item {{
            // TODO: update to the item you want to check
            ast::Item::CreateTable(create_table) => {{
                ctx.report(Violation::new(
                    Rule::{rule_name_pascal},
                    "todo".to_string(),
                    create_table.syntax().text_range(),
                    "todo or none".to_string(),
                ));
                todo!();
            }}
            _ => (),
        }}
    }}
}}

#[cfg(test)]
mod test {{
    use insta::assert_debug_snapshot;

    use crate::{{Linter, Rule}};
    use squawk_syntax::SourceFile;

    #[test]
    fn err() {{
        let sql = r#"
        -- TODO: err sql
        "#;
        let file = SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::{rule_name_pascal}]);
        let errors = linter.lint(file, sql);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors);
    }}

    #[test]
    fn ok() {{
        let sql = r#"
        -- TODO: ok sql
        "#;
        let file = SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::{rule_name_pascal}]);
        let errors = linter.lint(file, sql);
        assert_eq!(errors.len(), 0);
    }}
}}
"###
    )
}

fn root_path() -> Utf8PathBuf {
    let binding = Utf8PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    Utf8PathBuf::from(binding.parent().unwrap().parent().unwrap())
}

fn create_rule_file(name: &str) -> Result<()> {
    let name = name.to_case(Case::Snake);
    let root = root_path();
    let lint_path = root.join(format!("crates/squawk_linter/src/rules/{}.rs", name));

    if fs::exists(&lint_path)? {
        println!("skipping rule file creation, it already exists {lint_path}");
        return Ok(());
    }

    let lint_data = make_lint(&name);
    fs::write(&lint_path, lint_data)?;
    println!("created rule file {lint_path}");
    Ok(())
}

fn update_lib(name: &str) -> Result<()> {
    let root = root_path();
    let lib = root.join("crates/squawk_linter/src/lib.rs");
    let mut file_content = fs::read_to_string(&lib)?;

    let name_snake = name.to_case(Case::Snake);
    let name_kebab = name.to_case(Case::Kebab);
    let name_pascal = name.to_case(Case::Pascal);

    let replacements = [
        (
            "// xtask:new-rule:rule-import",
            format!(
                "use rules::{name_snake};
",
            ),
        ),
        (
            "// xtask:new-rule:error-name",
            format!(
                r#"#[serde(rename = "{name_kebab}")]
    {name_pascal},
    "#
            ),
        ),
        (
            "// xtask:new-rule:str-name",
            format!(
                r#""{name_kebab}" => Ok(Rule::{name_pascal}),
            "#
            ),
        ),
        (
            "// xtask:new-rule:variant-to-name",
            format!(
                r#"Rule::{name_pascal} => "{name_kebab}",
            "#,
            ),
        ),
        (
            "// xtask:new-rule:rule-call",
            format!(
                r#"if self.rules.contains(&Rule::{name_pascal}) {{
            {name_snake}(self, &file);
        }}
        "#
            ),
        ),
    ];

    for (marker, replacement) in replacements {
        file_content = file_content.replace(marker, &(replacement + marker))
    }

    fs::write(&lib, file_content)?;

    Ok(())
}

fn update_rules_mod(name: &str) -> Result<()> {
    let root = root_path();
    let lib = root.join("crates/squawk_linter/src/rules/mod.rs");
    let mut file_content = fs::read_to_string(&lib)?;

    let name_snake = name.to_case(Case::Snake);

    let replacements = [
        (
            "// xtask:new-rule:mod-decl",
            format!("pub(crate) mod {name_snake};\n"),
        ),
        (
            "// xtask:new-rule:export",
            format!("pub(crate) use {name_snake}::{name_snake};\n",),
        ),
    ];

    for (marker, replacement) in replacements {
        file_content = file_content.replace(marker, &(replacement + marker))
    }

    fs::write(&lib, file_content)?;

    Ok(())
}

pub(crate) fn new_lint(args: NewRuleArgs) -> Result<()> {
    let name = args.name;

    create_rule_file(&name)?;
    update_lib(&name)?;
    update_rules_mod(&name)?;

    docs_create_rule(&name)?;
    docs_update_page_index(&name)?;
    docs_update_sidebar(&name)?;

    Ok(())
}

fn make_doc(name: &str) -> String {
    let rule_name_kebab = name.to_case(Case::Kebab);
    format!(
        r###"
---
id: {rule_name_kebab}
title: {rule_name_kebab}
---

## problem

<!-- TODO -->

## solution

<!-- TODO -->

## links

<!-- TODO -->
"###
    )
}

fn docs_create_rule(name: &str) -> Result<()> {
    let root = root_path();
    let docs = root.join("docs");
    let name_kebab = name.to_case(Case::Kebab);
    let rule_doc_path = docs.join(format!("docs/{name_kebab}.md"));
    if fs::exists(&rule_doc_path)? {
        println!("skipping rule doc file creation, it already exists {rule_doc_path}");
        return Ok(());
    }
    let doc = make_doc(name);
    fs::write(&rule_doc_path, doc)?;
    println!("created rule doc {rule_doc_path}");
    Ok(())
}

fn docs_update_page_index(name: &str) -> Result<()> {
    let name_kebab = name.to_case(Case::Kebab);
    let root = root_path();
    let rule_sidebars = root.join("docs/src/pages/index.js");
    let mut file_content = fs::read_to_string(&rule_sidebars)?;

    let replacements = [(
        "// xtask:new-rule:rule-doc-meta",
        format!(
            r#"{{
    name: "{name_kebab}",
    tags: ["TODO"],
    description: "TODO",
  }},
  "#
        ),
    )];

    for (marker, replacement) in replacements {
        file_content = file_content.replace(marker, &(replacement + marker))
    }

    fs::write(&rule_sidebars, file_content)?;
    Ok(())
}

fn docs_update_sidebar(name: &str) -> Result<()> {
    let name_kebab = name.to_case(Case::Kebab);
    let root = root_path();
    let rule_sidebars = root.join("docs/sidebars.js");
    let mut file_content = fs::read_to_string(&rule_sidebars)?;

    let replacements = [(
        "// xtask:new-rule:error-name",
        format!(
            r#""{name_kebab}",
      "#
        ),
    )];

    for (marker, replacement) in replacements {
        file_content = file_content.replace(marker, &(replacement + marker))
    }

    fs::write(&rule_sidebars, file_content)?;
    Ok(())
}
