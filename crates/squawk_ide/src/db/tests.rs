use super::semantic_index;
use crate::db::parse;
use crate::db::{CompletionFile, Database, File, FileId};
use crate::file::InFile;
use crate::goto_definition::goto_definition;
use crate::semantic_index_builder::SemanticIndexBuilder;
use rowan::TextSize;
use salsa::Setter;

#[test]
fn source_file_identity() {
    let db = Database::default();
    let definition = File::new(&db, "create table t (a int);".into());
    let reference = File::new(&db, "select a from t;".into());
    let mut builder = SemanticIndexBuilder::new();
    builder.add_file(InFile::new(definition, &parse(&db, definition).tree()));
    builder.add_file(InFile::new(reference, &parse(&db, reference).tree()));
    let index = builder.build();
    let definitions = crate::goto_definition::goto_definition_with_index(
        &index,
        &parse(&db, reference).tree(),
        InFile::new(reference, TextSize::from(7)),
    );
    assert_eq!(definitions.len(), 1);
    assert_eq!(definitions[0].file_id, definition.into());
    assert_eq!(&definition.content(&db)[definitions[0].value], "a");
}

#[test]
fn completion_file_identity() {
    let db = Database::default();
    let file = File::new(&db, "create table t (a int); select a from t; ".into());
    let completion = CompletionFile::new(&db, file, TextSize::from(41));
    let completion = FileId::Completion(completion);
    let definitions = goto_definition(&db, InFile::new(completion, TextSize::from(31)));
    assert_eq!(definitions.len(), 1);
    assert_eq!(definitions[0].file_id, completion);
    assert_eq!(&file.content(&db)[definitions[0].value], "a");
    assert!(!std::ptr::eq(
        semantic_index(&db, file.into()),
        semantic_index(&db, completion),
    ));
}

#[test]
fn extension_target() {
    let db = Database::default();
    let file = File::new(&db, "create extension vector; select null::vector;".into());
    let definitions = goto_definition(&db, InFile::new(file, TextSize::from(38)));
    assert!(!definitions.is_empty());
    for target in definitions {
        assert!(matches!(target.file_id, FileId::Extension(_)));
        let extension = target.file_id.original_file(&db);
        assert_eq!(&extension.content(&db)[target.value], "vector");
        assert_eq!(target.file_id.original_file(&db), extension);
    }
}

#[test]
fn embedded_sql_target() {
    let db = Database::default();
    let sql = "create function f(a int) returns int language sql as 'select a';";
    let file = File::new(&db, sql.into());
    let definitions = goto_definition(&db, InFile::new(file, TextSize::from(61)));
    assert_eq!(definitions.len(), 1);
    assert_eq!(definitions[0].file_id, file.into());
    assert_eq!(&sql[definitions[0].value], "a");
    assert_eq!(definitions[0].value.start(), TextSize::from(18));
}

#[test]
fn reuses_index_until_file_changes() {
    let mut db = Database::default();
    let file = File::new(&db, "create table t (a int); select a from t;".into());
    assert!(std::ptr::eq(
        semantic_index(&db, file.into()),
        semantic_index(&db, file.into()),
    ));
    let position = InFile::new(file, TextSize::from(31));
    let definitions = goto_definition(&db, position);
    assert_eq!(definitions.len(), 1);
    assert_eq!(definitions[0].file_id, file.into());
    assert_eq!(&file.content(&db)[definitions[0].value], "a");
    file.set_content(&mut db)
        .to("create table t (b int); select a from t;".into());
    assert!(goto_definition(&db, position).is_empty());
}
