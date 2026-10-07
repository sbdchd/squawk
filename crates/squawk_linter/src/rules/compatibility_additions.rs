mod ban_add_column;
mod ban_add_composite_attribute;
mod ban_add_enum_value;
mod ban_alter_sequence_values;
mod ban_detach_inheritance;
mod ban_new_write_restriction;

use rustc_hash::FxHashSet;
use squawk_syntax::{
    Parse, SourceFile,
    ast::{self, AstNode},
};

use crate::{Linter, Rule};

// Only unconditional CREATE statements establish that an object is new to this file.
pub(crate) fn compatibility_additions(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    let mut tables = FxHashSet::default();
    let mut types = FxHashSet::default();
    let mut sequences = FxHashSet::default();

    for stmt in parse.tree().stmts() {
        match stmt {
            ast::Stmt::CreateTable(table) => {
                if table.if_not_exists().is_none() {
                    if let Some(name) = table.table_name().and_then(|n| n.path()) {
                        tables.insert(name.syntax().to_string());
                    }
                }
            }
            ast::Stmt::CreateType(ty) => {
                if let Some(name) = ty.type_name().and_then(|n| n.path()) {
                    types.insert(name.syntax().to_string());
                }
            }
            ast::Stmt::CreateSequence(seq) => {
                if seq.if_not_exists().is_none() {
                    if let Some(name) = seq.sequence().and_then(|n| n.path()) {
                        sequences.insert(name.syntax().to_string());
                    }
                }
            }
            ast::Stmt::AlterTable(table) => {
                let new_table = table
                    .table_relation_name()
                    .and_then(|n| n.table_name_ref())
                    .and_then(|n| n.path_ref())
                    .is_some_and(|n| tables.contains(&n.syntax().to_string()));
                for action in table.actions() {
                    if !new_table {
                        if ctx.rules.contains(&Rule::BanAddColumn) {
                            ban_add_column::check(ctx, &action);
                        }
                        if ctx.rules.contains(&Rule::BanNewWriteRestriction) {
                            ban_new_write_restriction::check_table_action(ctx, &action);
                        }
                        if ctx.rules.contains(&Rule::BanAlterSequenceValues) {
                            ban_alter_sequence_values::check_table_action(ctx, &action);
                        }
                        if ctx.rules.contains(&Rule::BanDetachInheritance) {
                            ban_detach_inheritance::check(ctx, &action);
                        }
                    }
                }
            }
            ast::Stmt::AlterForeignTable(table) => {
                for action in table.actions() {
                    if ctx.rules.contains(&Rule::BanAddColumn) {
                        ban_add_column::check(ctx, &action);
                    }
                    if ctx.rules.contains(&Rule::BanNewWriteRestriction) {
                        ban_new_write_restriction::check_foreign_table_action(ctx, &action);
                    }
                }
            }
            ast::Stmt::AlterDomain(domain) if ctx.rules.contains(&Rule::BanNewWriteRestriction) => {
                ban_new_write_restriction::check_domain(ctx, &domain);
            }
            ast::Stmt::CreateIndex(index) if ctx.rules.contains(&Rule::BanNewWriteRestriction) => {
                if index
                    .table_relation_name()
                    .and_then(|n| n.table_name_ref())
                    .and_then(|n| n.path_ref())
                    .is_none_or(|n| !tables.contains(&n.syntax().to_string()))
                {
                    ban_new_write_restriction::check_index(ctx, &index);
                }
            }
            ast::Stmt::AlterType(ty) => {
                let new_type = ty
                    .type_name_ref()
                    .and_then(|n| n.path_ref())
                    .is_some_and(|n| types.contains(&n.syntax().to_string()));
                if !new_type {
                    if ctx.rules.contains(&Rule::BanAddEnumValue) {
                        ban_add_enum_value::check(ctx, &ty);
                    }
                    if ctx.rules.contains(&Rule::BanAddCompositeAttribute) {
                        ban_add_composite_attribute::check(ctx, &ty);
                    }
                }
            }
            ast::Stmt::AlterSequence(seq) if ctx.rules.contains(&Rule::BanAlterSequenceValues) => {
                let new_sequence = seq
                    .sequence_ref()
                    .and_then(|n| n.path_ref())
                    .is_some_and(|n| sequences.contains(&n.syntax().to_string()));
                if !new_sequence {
                    ban_alter_sequence_values::check_sequence(ctx, &seq);
                }
            }
            _ => (),
        }
    }
}
