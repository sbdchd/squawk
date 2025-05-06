use std::collections::HashSet;

use rowan::{NodeOrToken, TextRange, TextSize};
use squawk_syntax::{SyntaxKind, SyntaxNode, SyntaxToken};

use crate::{Linter, Rule, Violation};

#[derive(Debug)]
pub struct Ignore {
    pub range: TextRange,
    pub violation_names: HashSet<Rule>,
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

const IGNORE_TEXT: &str = "squawk-ignore";

fn ignore_rule_names(token: &SyntaxToken) -> Option<(&str, TextRange)> {
    if let Some((comment_body, range)) = comment_body(token) {
        let without_start = comment_body.trim_start();
        let trim_start_size = comment_body.len() - without_start.len();
        let trimmed_comment = without_start.trim_end();
        let trim_end_size = without_start.len() - trimmed_comment.len();

        if let Some(without_prefix) = trimmed_comment.strip_prefix(IGNORE_TEXT) {
            let range = TextRange::new(
                range.start() + TextSize::new((trim_start_size + IGNORE_TEXT.len()) as u32),
                range.end() - TextSize::new(trim_end_size as u32),
            );
            return Some((without_prefix, range));
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
                if let Some((rule_names, range)) = ignore_rule_names(&token) {
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

                            ctx.report(Violation::new(
                                Rule::UnusedIgnore,
                                format!("unknown name {trimmed}"),
                                range,
                                None,
                            ));
                        }

                        offset += x.len() + 1;
                    }
                    ctx.ignore(Ignore {
                        range,
                        violation_names: set,
                    });
                }
            }
            _ => (),
        }
    }
}

#[cfg(test)]
mod test {

    use crate::{find_ignores, Linter, Rule};

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
        let errors = linter.lint(parse, sql);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn starting_line_aka_zero() {
        let mut linter = Linter::with_all_rules();
        let sql = r#"alter table t add column c char;"#;

        let parse = squawk_syntax::SourceFile::parse(sql);
        let errors = linter.lint(parse, sql);
        assert_eq!(errors.len(), 1);
    }
}
