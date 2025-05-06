use rowan::TextRange;
use squawk_syntax::{
    ast::{self, AstNode, HasModuleItem},
    Parse, SourceFile,
};

use crate::{Linter, Rule, Violation};

pub(crate) fn ban_create_domain_with_constraint(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    let file = parse.tree();
    for item in file.items() {
        if let ast::Item::CreateDomain(domain) = item {
            let range =
                domain
                    .constraints()
                    .map(|c| c.syntax().text_range())
                    .fold(None, |prev, cur| match prev {
                        None => Some(cur),
                        Some(prev) => {
                            let new_start = prev.start().min(cur.start());
                            let new_end = prev.end().max(cur.end());
                            Some(TextRange::new(new_start, new_end))
                        }
                    });
            if let Some(range) = range {
                ctx.report(Violation::new(
                Rule::BanCreateDomainWithConstraint,
                    "Domains with constraints have poor support for online migrations. Use table and column constraints instead.".into(),
                    range,
                    None,
                ))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use insta::assert_debug_snapshot;

    use crate::{Linter, Rule};

    #[test]
    fn err() {
        let sql = r#"
CREATE DOMAIN domain_name_3 AS NUMERIC(15,5) CHECK (value > 0);
        "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::BanCreateDomainWithConstraint]);
        let errors = linter.lint(file, sql);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors);
    }

    #[test]
    fn err_with_multiple_constraints() {
        // checking that we highlight all the constraints in our range
        let sql = r#"
create domain d as t check (value > 0) not null;
        "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::BanCreateDomainWithConstraint]);
        let errors = linter.lint(file, sql);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors);
    }

    #[test]
    fn ok() {
        // creating without a constraint is okay
        let sql = r#"
CREATE DOMAIN domain_name_1 AS TEXT;
CREATE DOMAIN domain_name_2 AS CHARACTER VARYING;
        "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::BanCreateDomainWithConstraint]);
        let errors = linter.lint(file, sql);
        assert_eq!(errors.len(), 0);
    }
}
