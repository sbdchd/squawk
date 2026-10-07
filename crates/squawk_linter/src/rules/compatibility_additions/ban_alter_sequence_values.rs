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
