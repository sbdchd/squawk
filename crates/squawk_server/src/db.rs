use ::line_index::LineIndex;
use salsa::Database as Db;
use salsa::Storage;
use squawk_syntax::{Parse, SourceFile};

#[salsa::input]
pub struct File {
    #[returns(ref)]
    pub content: String,
    pub version: i32,
}

#[salsa::tracked]
pub fn parse(db: &dyn Db, file: File) -> Parse<SourceFile> {
    SourceFile::parse(file.content(db))
}

#[salsa::tracked]
pub fn line_index(db: &dyn Db, file: File) -> LineIndex {
    LineIndex::new(file.content(db))
}

#[salsa::db]
#[derive(Default)]
pub struct Database {
    storage: Storage<Self>,
}

impl salsa::Database for Database {}
