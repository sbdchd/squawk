use squawk_syntax::{
    ast::{self, AstNode, HasModuleItem},
    Parse, SourceFile,
};

use crate::{Linter, Rule, Violation};

pub(crate) fn ban_alter_domain_with_add_constraint(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    let file = parse.tree();
    for item in file.items() {
        if let ast::Item::AlterDomain(alter_domain) = item {
            let actions = alter_domain.actions();
            for action in actions {
                if let ast::AlterDomainAction::AddConstraint(add_constraint) = action {
                    ctx.report(Violation::new(
                    Rule::BanAlterDomainWithAddConstraint,
                        "Domains with constraints have poor support for online migrations. Use table and column constraints instead.".into(),
                        add_constraint.syntax().text_range(),
                        None,
                    ))
                }
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
     ALTER DOMAIN domain_name ADD CONSTRAINT constraint_name CHECK (value > 0);
        "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::BanAlterDomainWithAddConstraint]);
        let errors = linter.lint(file, sql);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors);
    }

    #[test]
    fn ok() {
        let sql = r#"
     ALTER DOMAIN domain_name_2 SET NOT NULL;
     ALTER DOMAIN domain_name_3 DROP CONSTRAINT other_domain_name;
     ALTER DOMAIN domain_name_4 RENAME CONSTRAINT constraint_name TO other_constraint_name;
     ALTER DOMAIN domain_name_5 RENAME TO other_domain_name;
     ALTER DOMAIN domain_name_6 VALIDATE CONSTRAINT constraint_name;
     ALTER DOMAIN domain_name_7 OWNER TO you;
     ALTER DOMAIN domain_name_8 SET SCHEMA foo;
        "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::BanAlterDomainWithAddConstraint]);
        let errors = linter.lint(file, sql);
        assert_eq!(errors.len(), 0);
    }
}
