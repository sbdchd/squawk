use anyhow::{Context, Result};
use convert_case::{Case, Casing};

use crate::NewLintArgs;
use std::{
    fs,
    path::{Path, PathBuf},
};

fn make_lint(name: &str) -> String {
    format!(
        r###"
use squawk_syntax::{{
    ast::{{self, AstNode, HasModuleItem}},
    Parse, SourceFile,
}};

use crate::{{Linter, Violation, ErrorCode}};

pub(crate) fn {rule_name}(ctx: &mut Linter, parse: &Parse<SourceFile>) {{
    let file = parse.tree();
    for item in file.items() {{
        match item {{
            // TODO:
            _ => (),
        }}
    }}
}}

#[cfg(test)]
mod test {{
    use insta::assert_debug_snapshot;

    use crate::{{Linter, Rule, ErrorCode}};
    use squawk_syntax::SourceFile;

    #[test]
    fn err() {{
        let sql = r#"
        -- TODO: err sql
        "#;
        let file = SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::{rule_name_pascal}]);
        let errors = linter.lint(file, sql);
        assert_ne!(linter.errors.len(), 0);
        assert_debug_snapshot!(linter.errors);
    }}

    #[test]
    fn ok() {{
        let sql = r#"
        -- TODO: ok sql
        "#;
        let file = SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::{rule_name_pascal}]);
        let errors = linter.lint(file, sql);
        assert_eq!(linter.errors.len(), 0);
    }}
}}
"###,
        rule_name = name,
        rule_name_pascal = name.to_case(Case::Pascal),
    )
}

fn crates_path() -> Result<PathBuf> {
    let current_file = Path::new(file!());
    let path = current_file
        .ancestors()
        // crates/xtask/src/new_lint
        //                  ^ from
        // ^ to
        .nth(3)
        .context("couldn't get root of crate")
        .unwrap();
    Ok(PathBuf::from(path))
}

fn create_rule_file(name: &str) -> Result<()> {
    let crates = crates_path()?;
    let lint_path = crates.join(format!("squawk_linter/src/rules/{}.rs", name));

    if fs::exists(&lint_path)? {
        println!("skipping file creation, it already exists");
        return Ok(());
    }

    let lint_data = make_lint(name);
    fs::write(&lint_path, lint_data)?;
    println!("created file");
    Ok(())
}

fn update_lib(name: &str) -> Result<()> {
    let crates = crates_path()?;
    let lib = crates.join("squawk_linter/src/lib.rs");
    let mut file_content = fs::read_to_string(&lib)?;

    let name_snake = name.to_case(Case::Snake);
    let name_kebab = name.to_case(Case::Kebab);
    let name_pascal = name.to_case(Case::Pascal);

    let replacements = [
        (
            "// xtask:new-lint:rule-import",
            format!(
                "use rules::{name_snake};
",
                name_snake = name_snake,
            ),
        ),
        (
            "// xtask:new-lint:error-name",
            format!(
                r#"#[serde(rename = "{name_kebab}")]
    {name_pascal},
    "#,
                name_kebab = name_kebab,
                name_pascal = name_pascal,
            ),
        ),
        (
            "// xtask:new-lint:str-name",
            format!(
                r#""{name_kebab}" => Ok(ErrorCode::{name_pascal}),
    "#,
                name_kebab = name_kebab,
                name_pascal = name_pascal,
            ),
        ),
        (
            "// xtask:new-lint:name",
            format!(
                r#"{name_pascal},
    "#,
                name_pascal = name_pascal,
            ),
        ),
        (
            "// xtask:new-lint:rule-call",
            format!(
                r#"if self.rules.contains(&Rule::{name_pascal}) {{
            {name_snake}(self, &file);
        }}
        "#,
                name_pascal = name_pascal,
                name_snake = name_snake,
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
    let crates = crates_path()?;
    let lib = crates.join("squawk_linter/src/rules/mod.rs");
    let mut file_content = fs::read_to_string(&lib)?;

    let name_snake = name.to_case(Case::Snake);

    let replacements = [
        (
            "// xtask:new-lint:mod-decl",
            format!("pub(crate) mod {name_snake};\n", name_snake = name_snake),
        ),
        (
            "// xtask:new-lint:export",
            format!(
                "pub(crate) use {name_snake}::{name_snake};\n",
                name_snake = name_snake
            ),
        ),
    ];

    for (marker, replacement) in replacements {
        file_content = file_content.replace(marker, &(replacement + marker))
    }

    fs::write(&lib, file_content)?;

    Ok(())
}

pub(crate) fn new_lint(args: NewLintArgs) -> Result<()> {
    let name = args.name;

    create_rule_file(&name)?;

    update_lib(&name)?;

    update_rules_mod(&name)?;

    Ok(())
}
