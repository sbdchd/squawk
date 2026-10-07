use squawk_syntax::ast::{self, AstNode};

use crate::{Linter, Rule, Violation};

pub(super) fn check(ctx: &mut Linter, action: &ast::AlterTableAction) {
    if let ast::AlterTableAction::AddColumn(column) = action {
        ctx.report(Violation::for_node(
            Rule::BanAddColumn,
            "Adding a column changes the shape of rows existing clients receive and can break positional inserts.".into(),
            column.syntax(),
        ));
    }
}

#[cfg(test)]
mod test {
    use crate::{
        Rule,
        test_utils::{lint_errors, lint_ok},
    };

    #[test]
    fn add_column() {
        assert_eq!(
            lint_errors("ALTER TABLE t ADD COLUMN c int;", Rule::BanAddColumn)
                .matches("warning[ban-add-column]")
                .count(),
            1
        );
        assert_eq!(
            lint_errors(
                "ALTER FOREIGN TABLE ft ADD COLUMN c int;",
                Rule::BanAddColumn
            )
            .matches("warning[ban-add-column]")
            .count(),
            1
        );
        lint_ok(
            "CREATE TABLE t (id int); ALTER TABLE t ADD COLUMN c int;",
            Rule::BanAddColumn,
        );
        assert_eq!(
            lint_errors(
                "CREATE TABLE IF NOT EXISTS t (id int); ALTER TABLE t ADD COLUMN c int;",
                Rule::BanAddColumn
            )
            .matches("warning[ban-add-column]")
            .count(),
            1
        );
    }
}
