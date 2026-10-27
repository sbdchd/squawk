mod access_methods;
mod aggregates;
mod builtins;
mod casts;
mod collations;
mod comments;
mod composite_types;
mod constraints;
mod conversions;
mod copy;
mod create_table_as;
mod ctes;
mod cursors;
mod databases;
mod delete;
mod dependencies;
mod domains;
mod enums;
mod event_triggers;
mod explain;
mod extensions;
mod field_selection;
mod foreign_data;
mod function_overloads;
mod functions;
mod indexes;
mod inheritance;
mod insert;
mod items;
mod joins;
mod languages;
mod merge;
mod operator_classes;
mod operators;
mod ordinals;
mod partitions;
mod policies;
mod polymorphic_types;
mod prepared_statements;
mod property_graphs;
mod publications;
mod ranges;
mod roles;
mod rules;
mod schemas;
mod search_path;
mod select;
mod sequences;
mod statistics;
mod subqueries;
mod subscripts;
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

use annotate_snippets::{AnnotationKind, Level, Patch, Renderer, Snippet, renderer::DecorStyle};
use squawk_syntax::SourceFile;

use crate::file::{FileId, InFile};
use crate::semantic_index::SemanticIndex;
use crate::semantic_index_builder::SemanticIndexBuilder;

#[track_caller]
fn build_index(sql: &str) -> (FileId, SemanticIndex) {
    let parse = SourceFile::parse(sql);
    assert_eq!(parse.errors(), vec![]);
    let mut builder = SemanticIndexBuilder::new();
    let db = crate::db::Database::default();
    let file: FileId = crate::db::File::new(&db, sql.into()).into();
    builder.add_file(InFile::new(file, &parse.tree()));
    (file, builder.build())
}

#[track_caller]
fn check(sql: &str) -> String {
    let (_, index) = build_index(sql);

    let renderer = Renderer::plain().decor_style(DecorStyle::Unicode);
    let diagnostics = index.diagnostics();
    if diagnostics.is_empty() {
        return "no diagnostics".to_string();
    }
    diagnostics
        .iter()
        .map(|diagnostic| {
            let range = diagnostic.range.value.start().into()..diagnostic.range.value.end().into();
            let snippet = Snippet::source(sql)
                .fold(true)
                .annotation(AnnotationKind::Primary.span(range));
            let mut group = Level::ERROR
                .primary_title(diagnostic.message.as_ref())
                .element(snippet);
            if let Some(hint) = &diagnostic.hint {
                group = group.element(Level::HELP.with_name("hint").message(hint.as_ref()));
            }
            let Some(suggestion) = &diagnostic.suggestion else {
                return renderer.render(&[group]);
            };
            let patches = suggestion.fix.edits.iter().map(|edit| {
                let range = edit.text_range.start().into()..edit.text_range.end().into();
                Patch::new(range, edit.text.as_deref().unwrap_or_default())
            });
            let fix_group = Level::HELP
                .secondary_title(suggestion.message.as_ref())
                .element(Snippet::source(sql).fold(true).patches(patches));
            renderer.render(&[group, fix_group])
        })
        .collect::<Vec<_>>()
        .join("\n")
}
