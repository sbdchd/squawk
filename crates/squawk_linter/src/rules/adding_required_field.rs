use squawk_syntax::{
    Parse, SourceFile,
    ast::{self, AstNode},
};

use crate::visitors::is_serial_type;
use crate::{Linter, Rule, Violation, transactions::TablesCreatedInTransaction};

pub(crate) fn adding_required_field(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    let file = parse.tree();
    let mut tables_created = TablesCreatedInTransaction::new(ctx.settings.assume_in_transaction);
    for stmt in file.stmts() {
        tables_created.update(&stmt);
        if let ast::Stmt::AlterTable(alter_table) = stmt {
            if let Some(table_name) = alter_table
                .table_relation_name()
                .and_then(|relation| relation.table_name_ref())
                .and_then(|table| table.path_ref())
                && tables_created.contains(&table_name)
            {
                continue;
            }
            for action in alter_table.actions() {
                if let ast::AlterTableAction::AddColumn(add_column) = action {
                    if has_generated_constrait(add_column.constraints()) {
                        continue;
                    }
                    if add_column.ty().is_some_and(|ty| is_serial_type(&ty)) {
                        continue;
                    }
                    if has_not_null_and_no_default_constraint(add_column.constraints()) {
                        ctx.report(Violation::for_node(
                            Rule::AddingRequiredField,
                            "adding a new column that is `not null` and has no default value to an existing table effectively makes it required".into(),
                            add_column.syntax(),
                        ).help("make the field nullable or add a non-volatile default"));
                    }
                }
            }
        }
    }
}

fn has_generated_constrait(constraints: ast::AstChildren<ast::Constraint>) -> bool {
    for c in constraints {
        if let ast::Constraint::GeneratedConstraint(_) = c {
            return true;
        }
    }
    false
}

fn has_not_null_and_no_default_constraint(constraints: ast::AstChildren<ast::Constraint>) -> bool {
    let mut has_not_null = false;
    let mut has_default = false;
    for c in constraints {
        match c {
            ast::Constraint::NotNullConstraint(_) => {
                has_not_null = true;
            }
            ast::Constraint::DefaultConstraint(_) => {
                has_default = true;
            }
            _ => (),
        }
    }
    has_not_null && !has_default
}

#[cfg(test)]
mod test {
    use insta::assert_snapshot;

    use crate::Rule;
    use crate::test_utils::{lint_errors, lint_ok};

    #[test]
    fn nullable_ok() {
        let sql = r#"
ALTER TABLE "recipe" ADD COLUMN "public" boolean;
  "#;
        lint_ok(sql, Rule::AddingRequiredField);
    }

    #[test]
    fn not_null_with_default() {
        let sql = r#"
ALTER TABLE "recipe" ADD COLUMN "public" boolean NOT NULL DEFAULT true;
  "#;
        lint_ok(sql, Rule::AddingRequiredField);
    }

    #[test]
    fn not_null_without_default() {
        let sql = r#"
ALTER TABLE "recipe" ADD COLUMN "public" boolean NOT NULL;
  "#;
        assert_snapshot!(lint_errors(sql, Rule::AddingRequiredField));
    }

    #[test]
    fn generated_stored_not_null_ok() {
        let sql = r#"
ALTER TABLE foo
    ADD COLUMN bar numeric GENERATED ALWAYS AS (bar + baz) STORED NOT NULL;
  "#;
        lint_ok(sql, Rule::AddingRequiredField);
    }

    #[test]
    fn generated_stored_ok() {
        let sql = r#"
 ALTER TABLE foo
    ADD COLUMN bar numeric GENERATED ALWAYS AS (bar + baz) STORED ;
  "#;
        lint_ok(sql, Rule::AddingRequiredField);
    }

    #[test]
    fn serial_not_null_ok() {
        let sql = r#"
alter table t add column e bigserial not null;
  "#;
        lint_ok(sql, Rule::AddingRequiredField);
    }

    #[test]
    fn table_created_in_transaction_ok() {
        let sql = r#"
begin;
create table t (id bigint);
alter table t add column c bigint not null;
commit;
  "#;
        lint_ok(sql, Rule::AddingRequiredField);
    }
}
