use squawk_syntax::ast::{self, AstNode};

use crate::{Linter, Rule, Violation};

pub(super) fn check_table_action(ctx: &mut Linter, action: &ast::AlterTableAction) {
    if let ast::AlterTableAction::AlterColumn(column) = action {
        if let Some(option) = column.option() {
            match option {
                ast::AlterColumnOption::Restart(node) => ctx.report(Violation::for_node(
                    Rule::BanAlterSequenceValues, "Restarting an identity sequence can change values generated for existing clients.".into(), node.syntax())),
                ast::AlterColumnOption::SetSequenceOption(node) => ctx.report(Violation::for_node(
                    Rule::BanAlterSequenceValues, "Changing an identity sequence option can change values generated for existing clients.".into(), node.syntax())),
                ast::AlterColumnOption::SetGeneratedOptions(options) => {
                    for option in options.set_generated_options() {
                        if matches!(option, ast::SetGeneratedOption::Restart(_) | ast::SetGeneratedOption::SetSequenceOption(_)) {
                            ctx.report(Violation::for_node(Rule::BanAlterSequenceValues,
                                "Changing an identity sequence can change values generated for existing clients.".into(), option.syntax()));
                        }
                    }
                }
                _ => (),
            }
        }
    }
}

pub(super) fn check_sequence(ctx: &mut Linter, seq: &ast::AlterSequence) {
    for action in seq.actions() {
        if let ast::AlterSequenceAction::SequenceOption(option) = action {
            if matches!(
                option,
                ast::SequenceOption::OptionOwnedBy(_)
                    | ast::SequenceOption::OptionLogged(_)
                    | ast::SequenceOption::OptionUnlogged(_)
                    | ast::SequenceOption::OptionSequenceName(_)
            ) {
                continue;
            }
            ctx.report(Violation::for_node(
                Rule::BanAlterSequenceValues,
                "Changing a sequence option can change values generated for existing clients."
                    .into(),
                option.syntax(),
            ));
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        Rule,
        test_utils::{lint_errors, lint_ok},
    };

    #[test]
    fn sequence_values() {
        assert_eq!(
            lint_errors(
                "ALTER SEQUENCE ids RESTART WITH 1 INCREMENT BY 2;",
                Rule::BanAlterSequenceValues
            )
            .matches("warning[ban-alter-sequence-values]")
            .count(),
            2
        );
        lint_ok(
            "CREATE SEQUENCE ids; ALTER SEQUENCE ids RESTART WITH 1;",
            Rule::BanAlterSequenceValues,
        );
        lint_ok(
            "ALTER SEQUENCE ids OWNER TO app; ALTER SEQUENCE ids OWNED BY t.id; ALTER SEQUENCE ids SET LOGGED; ALTER SEQUENCE ids SET UNLOGGED;",
            Rule::BanAlterSequenceValues,
        );
        assert_eq!(
            lint_errors(
                "ALTER TABLE t ALTER COLUMN id RESTART WITH 5;",
                Rule::BanAlterSequenceValues
            )
            .matches("warning[ban-alter-sequence-values]")
            .count(),
            1
        );
    }
}
