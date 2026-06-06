use squawk_syntax::{
    Parse, SourceFile,
    ast::{self, AstNode},
};

use crate::{Linter, Rule, Violation};

pub(crate) fn require_table_schema(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    let file = parse.tree();
    for stmt in file.stmts() {
        match stmt {
            ast::Stmt::CreateTable(create_table) => {
                if matches!(create_table.persistence(), Some(ast::Persistence::Temp(_))) {
                    continue;
                }
                check_path(ctx, create_table.path());
            }
            ast::Stmt::CreateTableAs(create_table_as) => {
                if matches!(
                    create_table_as.persistence(),
                    Some(ast::Persistence::Temp(_))
                ) {
                    continue;
                }
                check_path(ctx, create_table_as.path());
            }
            ast::Stmt::AlterTable(alter_table) => {
                check_path(ctx, alter_table.relation_name().and_then(|r| r.path()));
            }
            ast::Stmt::DropTable(drop_table) => {
                for path in drop_table.paths() {
                    check_path(ctx, Some(path));
                }
            }
            _ => (),
        }
    }
}

fn check_path(ctx: &mut Linter, path: Option<ast::Path>) {
    if let Some(path) = path
        && path.qualifier().is_none()
    {
        ctx.report(Violation::for_node(
            Rule::RequireTableSchema,
            "Table name is not schema-qualified. Use schema.table (e.g., public.my_table).".into(),
            path.syntax(),
        ));
    }
}

#[cfg(test)]
mod test {
    use insta::assert_snapshot;

    use crate::Rule;
    use crate::test_utils::{lint_errors, lint_ok};

    #[test]
    fn create_table_err() {
        let sql = r#"
CREATE TABLE my_table (id int);
"#;
        assert_snapshot!(lint_errors(sql, Rule::RequireTableSchema));
    }

    #[test]
    fn create_table_ok() {
        let sql = r#"
CREATE TABLE public.my_table (id int);
"#;
        lint_ok(sql, Rule::RequireTableSchema);
    }

    #[test]
    fn alter_table_err() {
        let sql = r#"
ALTER TABLE my_table ADD COLUMN name text;
"#;
        assert_snapshot!(lint_errors(sql, Rule::RequireTableSchema));
    }

    #[test]
    fn alter_table_ok() {
        let sql = r#"
ALTER TABLE public.my_table ADD COLUMN name text;
"#;
        lint_ok(sql, Rule::RequireTableSchema);
    }

    #[test]
    fn drop_table_err() {
        let sql = r#"
DROP TABLE my_table;
"#;
        assert_snapshot!(lint_errors(sql, Rule::RequireTableSchema));
    }

    #[test]
    fn drop_table_ok() {
        let sql = r#"
DROP TABLE public.my_table;
"#;
        lint_ok(sql, Rule::RequireTableSchema);
    }

    #[test]
    fn create_table_as_err() {
        let sql = r#"
CREATE TABLE my_table AS SELECT 1;
"#;
        assert_snapshot!(lint_errors(sql, Rule::RequireTableSchema));
    }

    #[test]
    fn create_table_as_ok() {
        let sql = r#"
CREATE TABLE public.my_table AS SELECT 1;
"#;
        lint_ok(sql, Rule::RequireTableSchema);
    }

    #[test]
    fn create_temp_table_ok() {
        let sql = r#"
CREATE TEMP TABLE temp_mapping_table (id int4) ON COMMIT DROP;
CREATE TEMPORARY TABLE other_temp (id int4);
"#;
        lint_ok(sql, Rule::RequireTableSchema);
    }

    #[test]
    fn create_temp_table_as_ok() {
        let sql = r#"
CREATE TEMP TABLE temp_mapping_table AS SELECT 1;
"#;
        lint_ok(sql, Rule::RequireTableSchema);
    }
}
