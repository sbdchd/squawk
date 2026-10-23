mod access_methods;
mod aggregates;
mod case_expr;
mod casts;
mod channels;
mod collations;
mod composite_types;
mod constraints;
mod conversions;
mod copy;
mod create_table_as;
mod ctes;
mod cursor_position;
mod cursors;
mod databases;
mod delete;
mod domains;
mod enums;
mod event_triggers;
mod extensions;
mod field_selection;
mod foreign_data;
mod function_overloads;
mod functions;
mod indexes;
mod inheritance;
mod insert;
mod joins;
mod keyword_functions;
mod languages;
mod merge;
mod operator_classes;
mod operator_overload;
mod operators;
mod ordinals;
mod partitions;
mod policies;
mod prepared_statements;
mod property_graphs;
mod publications;
mod ranges;
mod roles;
mod rules;
mod schemas;
mod select;
mod sequences;
mod statistics;
mod table_changes;
mod table_functions;
mod tables;
mod tablespaces;
mod text_search;
mod transactions;
mod transforms;
mod triggers;
mod types;
mod update;
mod views;
mod windows;

use crate::test_utils::render_goto;

use crate::goto_definition::goto_definition_with_index as goto_definition;

#[track_caller]
fn goto(sql: &str) -> String {
    goto_files(&[sql])
}

#[track_caller]
fn goto_files(files: &[&str]) -> String {
    render_goto(files, "goto definition", goto_definition)
        .unwrap_or_else(|| "no definition".to_string())
}
