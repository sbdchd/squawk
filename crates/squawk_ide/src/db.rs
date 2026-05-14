use ::line_index::LineIndex;
use salsa::Database as Db;
use salsa::Storage;
use squawk_syntax::{Parse, SourceFile};
use std::sync::Arc;

use crate::binder;
use crate::binder::Binder;
use crate::builtins::builtins_file;

#[salsa::input]
pub struct File {
    #[returns(ref)]
    pub content: Arc<str>,
}

#[salsa::tracked]
pub fn parse(db: &dyn Db, file: File) -> Parse<SourceFile> {
    SourceFile::parse(file.content(db))
}

#[salsa::tracked]
pub fn line_index(db: &dyn Db, file: File) -> LineIndex {
    LineIndex::new(file.content(db))
}

#[inline]
pub(crate) fn list_files(db: &dyn Db, file: File) -> impl Iterator<Item = File> {
    [file, builtins_file(db)].into_iter()
}

#[salsa::tracked]
pub fn bind(db: &dyn Db, file: File) -> Binder {
    let result = parse(db, file);
    let source_file = result.tree();
    binder::bind(&source_file)
}

#[salsa::db]
#[derive(Clone, Default)]
pub struct Database {
    storage: Storage<Self>,
}

impl salsa::Database for Database {}
