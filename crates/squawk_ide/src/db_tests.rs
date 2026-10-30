use squawk_sema::file::FileId;
use squawk_sema::file::InFile;
use squawk_sema::semantic_index_builder::SemanticIndexBuilder;

use crate::goto_definition::goto_definition;
use rowan::TextSize;
use squawk_ide_db::{Database, original_file, parse, semantic_index};

#[test]
fn source_file_identity() {
    let mut db = Database::default();
    let definition: FileId = db.add_file("create table t (a int);".into()).into();
    let reference: FileId = db.add_file("select a from t;".into()).into();
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
    assert_eq!(definitions[0].file_id, definition);
    assert_eq!(
        &original_file(&db, definition).content(&db)[definitions[0].value],
        "a"
    );
}

#[test]
fn completion_file_identity() {
    let mut db = Database::default();
    let file = db.add_file("create table t (a int); select a from t; ".into());
    let completion = FileId::Completion {
        file,
        offset: TextSize::from(41),
    };
    let definitions = goto_definition(&db, InFile::new(completion, TextSize::from(31)));
    assert_eq!(definitions.len(), 1);
    assert_eq!(definitions[0].file_id, completion);
    assert_eq!(
        &original_file(&db, file.into()).content(&db)[definitions[0].value],
        "a"
    );
    assert!(!std::ptr::eq(
        semantic_index(&db, file.into()),
        semantic_index(&db, completion),
    ));
}

#[test]
fn extension_target() {
    let mut db = Database::default();
    let file = db.add_file("create extension vector; select null::vector;".into());
    let definitions = goto_definition(&db, InFile::new(file, TextSize::from(38)));
    assert!(!definitions.is_empty());
    for target in definitions {
        assert!(matches!(target.file_id, FileId::Extension(_)));
        let extension = original_file(&db, target.file_id);
        assert_eq!(&extension.content(&db)[target.value], "vector");
        assert_eq!(original_file(&db, target.file_id), extension);
    }
}

#[test]
fn embedded_sql_target() {
    let mut db = Database::default();
    let sql = "create function f(a int) returns int language sql as 'select a';";
    let file = db.add_file(sql.into());
    let definitions = goto_definition(&db, InFile::new(file, TextSize::from(61)));
    assert_eq!(definitions.len(), 1);
    assert_eq!(definitions[0].file_id, file.into());
    assert_eq!(&sql[definitions[0].value], "a");
    assert_eq!(definitions[0].value.start(), TextSize::from(18));
}

#[test]
fn reuses_index_until_file_changes() {
    let mut db = Database::default();
    let file = db.add_file("create table t (a int); select a from t;".into());
    assert!(std::ptr::eq(
        semantic_index(&db, file.into()),
        semantic_index(&db, file.into()),
    ));
    let position = InFile::new(file, TextSize::from(31));
    let definitions = goto_definition(&db, position);
    assert_eq!(definitions.len(), 1);
    assert_eq!(definitions[0].file_id, file.into());
    assert_eq!(
        &original_file(&db, file.into()).content(&db)[definitions[0].value],
        "a"
    );
    db.set_file_content(file, "create table t (b int); select a from t;".into());
    assert!(goto_definition(&db, position).is_empty());
}
