mod ban_add_column;
mod ban_add_composite_attribute;
mod ban_add_enum_value;
mod ban_alter_sequence_values;
mod ban_detach_inheritance;
mod ban_new_write_restriction;

use rustc_hash::FxHashSet;
use squawk_syntax::{Parse, SourceFile, ast};

use crate::{Linter, Rule, name::Name};

fn object_name(path: &ast::Path) -> Vec<Name> {
    let mut name = path.qualifier().map(object_name_ref).unwrap_or_default();
    if let Some(segment) = path.segment() {
        name.push(Name::from_node(&segment));
    }
    name
}

fn object_name_ref(path: ast::PathRef) -> Vec<Name> {
    let mut name = std::iter::successors(Some(path), |path| path.qualifier())
        .filter_map(|path| path.segment())
        .map(|segment| Name::from_node(&segment))
        .collect::<Vec<_>>();
    name.reverse();
    name
}

// Only unconditional CREATE statements establish that an object is new to this file.
pub(crate) fn existing_object_compatibility(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    let mut tables = FxHashSet::default();
    let mut types = FxHashSet::default();
    let mut sequences = FxHashSet::default();

    for stmt in parse.tree().stmts() {
        match stmt {
            ast::Stmt::CreateTable(table) => {
                if table.if_not_exists().is_none()
                    && let Some(name) = table.table_name().and_then(|n| n.path())
                {
                    tables.insert(object_name(&name));
                }
            }
            ast::Stmt::CreateType(ty) => {
                if let Some(name) = ty.type_name().and_then(|n| n.path()) {
                    types.insert(object_name(&name));
                }
            }
            ast::Stmt::CreateSequence(seq) => {
                if seq.if_not_exists().is_none()
                    && let Some(name) = seq.sequence().and_then(|n| n.path())
                {
                    sequences.insert(object_name(&name));
                }
            }
            ast::Stmt::AlterTable(table) => {
                let new_table = table
                    .table_relation_name()
                    .and_then(|n| n.table_name_ref())
                    .and_then(|n| n.path_ref())
                    .is_some_and(|n| tables.contains(&object_name_ref(n)));
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
                    .is_none_or(|n| !tables.contains(&object_name_ref(n)))
                {
                    ban_new_write_restriction::check_index(ctx, &index);
                }
            }
            ast::Stmt::AlterType(ty) => {
                let new_type = ty
                    .type_name_ref()
                    .and_then(|n| n.path_ref())
                    .is_some_and(|n| types.contains(&object_name_ref(n)));
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
                    .is_some_and(|n| sequences.contains(&object_name_ref(n)));
                if !new_sequence {
                    ban_alter_sequence_values::check_sequence(ctx, &seq);
                }
            }
            _ => (),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Rule, test_utils::lint_errors};
    use insta::assert_snapshot;

    #[test]
    fn object_names_are_normalized() {
        let sql = r#"CREATE TABLE Public.Users (id int);
ALTER TABLE public.users ADD COLUMN name text;
ALTER TABLE "public"."users" ADD COLUMN email text;
ALTER TABLE public."Users" ADD COLUMN age int;"#;
        assert_snapshot!(lint_errors(sql, Rule::BanAddColumn), @r#"
        warning[ban-add-column]: Adding a column changes the shape of rows existing clients receive and can break positional inserts.
          ╭▸ 
        4 │ ALTER TABLE public."Users" ADD COLUMN age int;
          ╰╴                           ━━━━━━━━━━━━━━━━━━
        "#);
    }
}
