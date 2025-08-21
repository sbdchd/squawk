use std::collections::HashSet;

use rowan::{NodeOrToken, TextRange, TextSize};
use squawk_syntax::{SyntaxKind, SyntaxNode, SyntaxToken};

use crate::{Linter, Rule, Violation};

#[derive(Debug)]
pub enum IgnoreKind {
    File,
    Line,
}

#[derive(Debug)]
pub struct Ignore {
    pub range: TextRange,
    pub violation_names: HashSet<Rule>,
    pub kind: IgnoreKind,
}

fn comment_body(token: &SyntaxToken) -> Option<(&str, TextRange)> {
    let range = token.text_range();
    if token.kind() == SyntaxKind::COMMENT {
        let text = token.text();
        if let Some(trimmed) = text.strip_prefix("--") {
            if let Some(start) = range.start().checked_add(2.into()) {
                let end = range.end();
                let updated_range = TextRange::new(start, end);
                return Some((trimmed, updated_range));
            }
        }
        if let Some(trimmed) = text.strip_prefix("/*").and_then(|x| x.strip_suffix("*/")) {
            if let Some(start) = range.start().checked_add(2.into()) {
                if let Some(end) = range.end().checked_sub(2.into()) {
                    let updated_range = TextRange::new(start, end);
                    return Some((trimmed, updated_range));
                }
            }
        }
    }
    None
}

// TODO: maybe in a future version we can rename this to squawk-ignore-line
pub const IGNORE_LINE_TEXT: &str = "squawk-ignore";
pub const IGNORE_FILE_TEXT: &str = "squawk-ignore-file";

pub fn ignore_rule_info(token: &SyntaxToken) -> Option<(&str, TextRange, IgnoreKind)> {
    if let Some((comment_body, range)) = comment_body(token) {
        let without_start = comment_body.trim_start();
        let trim_start_size = comment_body.len() - without_start.len();
        let trimmed_comment = without_start.trim_end();
        let trim_end_size = without_start.len() - trimmed_comment.len();

        for (prefix, kind) in [
            (IGNORE_FILE_TEXT, IgnoreKind::File),
            (IGNORE_LINE_TEXT, IgnoreKind::Line),
        ] {
            if let Some(without_prefix) = trimmed_comment.strip_prefix(prefix) {
                let range = TextRange::new(
                    range.start() + TextSize::new((trim_start_size + prefix.len()) as u32),
                    range.end() - TextSize::new(trim_end_size as u32),
                );
                return Some((without_prefix, range, kind));
            }
        }
    }
    None
}

pub(crate) fn find_ignores(ctx: &mut Linter, file: &SyntaxNode) {
    for event in file.preorder_with_tokens() {
        match event {
            rowan::WalkEvent::Enter(NodeOrToken::Token(token))
                if token.kind() == SyntaxKind::COMMENT =>
            {
                if let Some((rule_names, range, kind)) = ignore_rule_info(&token) {
                    let mut set = HashSet::new();
                    let mut offset = 0usize;

                    // we need to keep track of our offset and report specific
                    // ranges for any unknown names we encounter, which makes
                    // this more complicated
                    for x in rule_names.split(",") {
                        if x.is_empty() {
                            continue;
                        }
                        if let Ok(violation_name) = Rule::try_from(x.trim()) {
                            set.insert(violation_name);
                        } else {
                            let without_start = x.trim_start();
                            let trim_start_size = x.len() - without_start.len();
                            let trimmed = without_start.trim_end();

                            let range = range.checked_add(TextSize::new(offset as u32)).unwrap();

                            let start = range.start() + TextSize::new(trim_start_size as u32);
                            let end = start + TextSize::new(trimmed.len() as u32);
                            let range = TextRange::new(start, end);

                            ctx.report(Violation::for_range(
                                Rule::UnusedIgnore,
                                format!("unknown name {trimmed}"),
                                range,
                            ));
                        }

                        offset += x.len() + 1;
                    }
                    ctx.ignore(Ignore {
                        range,
                        violation_names: set,
                        kind,
                    });
                }
            }
            _ => (),
        }
    }
}

#[cfg(test)]
mod test {

    use insta::assert_debug_snapshot;

    use super::IgnoreKind;
    use crate::{Linter, Rule, find_ignores};

    #[test]
    fn single_ignore() {
        let sql = r#"
-- squawk-ignore ban-drop-column
alter table t drop column c cascade;
        "#;
        let parse = squawk_syntax::SourceFile::parse(sql);

        let mut linter = Linter::from([]);
        find_ignores(&mut linter, &parse.syntax_node());

        assert_eq!(linter.ignores.len(), 1);
        let ignore = &linter.ignores[0];
        assert!(ignore.violation_names.contains(&Rule::BanDropColumn));
    }

    #[test]
    fn multiple_sql_comments_with_ignore_is_ok() {
        let sql = "
-- fooo bar
-- buzz
-- squawk-ignore prefer-robust-stmts
create table x();

select 1;
";

        let parse = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::with_all_rules();
        find_ignores(&mut linter, &parse.syntax_node());

        assert_eq!(linter.ignores.len(), 1);
        let ignore = &linter.ignores[0];
        assert!(
            ignore.violation_names.contains(&Rule::PreferRobustStmts),
            "Make sure we picked up the ignore"
        );

        let errors = linter.lint(&parse, sql);

        assert_eq!(
            errors,
            vec![],
            "We shouldn't have any errors because we have the ignore setup"
        );
    }

    #[test]
    fn single_ignore_c_style_comment() {
        let sql = r#"
/* squawk-ignore ban-drop-column */
alter table t drop column c cascade;
        "#;
        let parse = squawk_syntax::SourceFile::parse(sql);

        let mut linter = Linter::from([]);

        find_ignores(&mut linter, &parse.syntax_node());

        assert_eq!(linter.ignores.len(), 1);
        let ignore = &linter.ignores[0];
        assert!(ignore.violation_names.contains(&Rule::BanDropColumn));
    }

    #[test]
    fn multi_ignore() {
        let sql = r#"
-- squawk-ignore ban-drop-column, renaming-column,ban-drop-database
alter table t drop column c cascade;
        "#;
        let parse = squawk_syntax::SourceFile::parse(sql);

        let mut linter = Linter::from([]);

        find_ignores(&mut linter, &parse.syntax_node());

        assert_eq!(linter.ignores.len(), 1);
        let ignore = &linter.ignores[0];
        assert!(ignore.violation_names.contains(&Rule::BanDropColumn));
        assert!(ignore.violation_names.contains(&Rule::RenamingColumn));
        assert!(ignore.violation_names.contains(&Rule::BanDropDatabase));
    }

    #[test]
    fn multi_ignore_c_style_comment() {
        let sql = r#"
/* squawk-ignore ban-drop-column, renaming-column,ban-drop-database */
alter table t drop column c cascade;
        "#;
        let parse = squawk_syntax::SourceFile::parse(sql);

        let mut linter = Linter::from([]);

        find_ignores(&mut linter, &parse.syntax_node());

        assert_eq!(linter.ignores.len(), 1);
        let ignore = &linter.ignores[0];
        assert!(ignore.violation_names.contains(&Rule::BanDropColumn));
        assert!(ignore.violation_names.contains(&Rule::RenamingColumn));
        assert!(ignore.violation_names.contains(&Rule::BanDropDatabase));
    }

    #[test]
    fn ignore_multiple_stmts() {
        let mut linter = Linter::with_all_rules();
        let sql = r#"
-- squawk-ignore ban-char-field,prefer-robust-stmts
alter table t add column c char;

ALTER TABLE foo
-- squawk-ignore adding-field-with-default,prefer-robust-stmts
ADD COLUMN bar numeric GENERATED 
  ALWAYS AS (bar + baz) STORED;

-- squawk-ignore prefer-robust-stmts
create table users (
);
"#;

        let parse = squawk_syntax::SourceFile::parse(sql);
        let errors = linter.lint(&parse, sql);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn starting_line_aka_zero() {
        let mut linter = Linter::with_all_rules();
        let sql = r#"alter table t add column c char;"#;

        let parse = squawk_syntax::SourceFile::parse(sql);
        let errors = linter.lint(&parse, sql);
        assert_eq!(errors.len(), 1);
    }

    #[test]
    fn regression_unknown_name() {
        let mut linter = Linter::with_all_rules();
        let sql = r#"
-- squawk-ignore prefer-robust-stmts
create table test_table (
  -- squawk-ignore prefer-timestamp-tz
  created_at timestamp default current_timestamp,
  other_field text
);
        "#;

        let parse = squawk_syntax::SourceFile::parse(sql);
        let errors = linter.lint(&parse, sql);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn file_single_rule() {
        let sql = r#"
-- squawk-ignore-file ban-drop-column
alter table t drop column c cascade;
        "#;
        let parse = squawk_syntax::SourceFile::parse(sql);

        let mut linter = Linter::from([]);
        find_ignores(&mut linter, &parse.syntax_node());

        assert_eq!(linter.ignores.len(), 1);
        let ignore = &linter.ignores[0];
        assert!(ignore.violation_names.contains(&Rule::BanDropColumn));
        assert!(matches!(ignore.kind, IgnoreKind::File));
    }

    #[test]
    fn file_ignore_with_all_rules() {
        let sql = r#"
-- squawk-ignore-file
alter table t drop column c cascade;
        "#;
        let parse = squawk_syntax::SourceFile::parse(sql);

        let mut linter = Linter::from([]);
        find_ignores(&mut linter, &parse.syntax_node());

        assert_eq!(linter.ignores.len(), 1);
        let ignore = &linter.ignores[0];
        assert!(matches!(ignore.kind, IgnoreKind::File));
        assert!(ignore.violation_names.is_empty());

        let errors: Vec<_> = linter
            .lint(&parse, sql)
            .into_iter()
            .map(|x| x.code)
            .collect();
        assert!(errors.is_empty());
    }

    #[test]
    fn file_ignore_with_multiple_rules() {
        let sql = r#"
-- squawk-ignore-file ban-drop-column, renaming-column
alter table t drop column c cascade;
        "#;
        let parse = squawk_syntax::SourceFile::parse(sql);

        let mut linter = Linter::from([]);
        find_ignores(&mut linter, &parse.syntax_node());

        assert_eq!(linter.ignores.len(), 1);
        let ignore = &linter.ignores[0];
        assert!(ignore.violation_names.contains(&Rule::BanDropColumn));
        assert!(ignore.violation_names.contains(&Rule::RenamingColumn));
        assert!(matches!(ignore.kind, IgnoreKind::File));
    }

    #[test]
    fn file_ignore_anywhere_works() {
        let sql = r#"
alter table t add column x int;
-- squawk-ignore-file ban-drop-column
alter table t drop column c cascade;
        "#;
        let parse = squawk_syntax::SourceFile::parse(sql);

        let mut linter = Linter::from([]);
        find_ignores(&mut linter, &parse.syntax_node());

        assert_eq!(linter.ignores.len(), 1);
        let ignore = &linter.ignores[0];
        assert!(ignore.violation_names.contains(&Rule::BanDropColumn));
        assert!(matches!(ignore.kind, IgnoreKind::File));
    }

    #[test]
    fn file_ignore_c_style_comment() {
        let sql = r#"
/* squawk-ignore-file ban-drop-column */
alter table t drop column c cascade;
        "#;
        let parse = squawk_syntax::SourceFile::parse(sql);

        let mut linter = Linter::from([]);
        find_ignores(&mut linter, &parse.syntax_node());

        assert_eq!(linter.ignores.len(), 1);
        let ignore = &linter.ignores[0];
        assert!(ignore.violation_names.contains(&Rule::BanDropColumn));
        assert!(matches!(ignore.kind, IgnoreKind::File));
    }

    #[test]
    fn file_level_only_ignores_specific_rules() {
        let mut linter = Linter::with_all_rules();
        let sql = r#"
-- squawk-ignore-file ban-drop-column
alter table t drop column c cascade;
alter table t2 drop column c2 cascade;
        "#;

        let parse = squawk_syntax::SourceFile::parse(sql);
        let errors: Vec<_> = linter
            .lint(&parse, sql)
            .into_iter()
            .map(|x| x.code)
            .collect();

        assert_debug_snapshot!(errors, @r"
        [
            PreferRobustStmts,
            PreferRobustStmts,
        ]
        ");
    }

    #[test]
    fn file_ignore_at_end_of_file_is_fine() {
        let mut linter = Linter::with_all_rules();
        let sql = r#"
alter table t drop column c cascade;
alter table t2 drop column c2 cascade;
-- squawk-ignore-file ban-drop-column
        "#;

        let parse = squawk_syntax::SourceFile::parse(sql);
        let errors: Vec<_> = linter
            .lint(&parse, sql)
            .into_iter()
            .map(|x| x.code)
            .collect();

        assert_debug_snapshot!(errors, @r"
        [
            PreferRobustStmts,
            PreferRobustStmts,
        ]
        ");
    }
}
