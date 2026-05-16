use ::line_index::LineIndex;
use salsa::Database as Db;
#[cfg(test)]
use salsa::Setter;
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
    [Some(file), include_builtins(db).then(|| builtins_file(db))]
        .into_iter()
        .flatten()
}

#[salsa::tracked]
pub(crate) fn bind(db: &dyn Db, file: File) -> Binder {
    let result = parse(db, file);
    let source_file = result.tree();
    binder::bind(&source_file)
}

#[salsa::input(singleton)]
pub(crate) struct Config {
    // currently only used for improve test runtime by skipping builtins
    pub(crate) include_builtins: bool,
}

#[salsa::tracked]
pub(crate) fn include_builtins(db: &dyn Db) -> bool {
    Config::get(db).include_builtins(db)
}

#[salsa::db]
#[derive(Clone)]
pub struct Database {
    storage: Storage<Self>,
}

impl Default for Database {
    fn default() -> Self {
        let db = Self {
            storage: Storage::default(),
        };
        Config::new(&db, true);
        db
    }
}

#[cfg(test)]
pub(crate) fn set_include_builtins(db: &mut dyn Db, include_builtins: bool) {
    Config::get(db)
        .set_include_builtins(db)
        .to(include_builtins);
}

impl salsa::Database for Database {}
