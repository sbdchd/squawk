use rowan::TextSize;
use salsa::Database as Db;
use salsa::Storage;
use squawk_line_index::{LineIndex, find_newline};
use squawk_syntax::{Parse, SourceFile};
use std::sync::Arc;

use crate::completion::COMPLETION_MARKER;
pub use crate::extensions::BundledExtension;
use crate::file::InFile;
use crate::semantic_index::SemanticIndex;
use crate::semantic_index_builder::SemanticIndexBuilder;

#[salsa::input(debug)]
#[derive(PartialOrd, Ord)]
pub struct File {
    #[returns(ref)]
    pub content: Arc<str>,
}

#[salsa::interned(no_lifetime, debug)]
#[derive(PartialOrd, Ord)]
pub struct CompletionFile {
    pub file: File,
    pub offset: TextSize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, salsa::Update)]
pub enum FileId {
    File(File),
    Completion(CompletionFile),
    Builtins,
    Extension(BundledExtension),
}

impl From<File> for FileId {
    fn from(file: File) -> Self {
        FileId::File(file)
    }
}

impl FileId {
    pub fn original_file(self, db: &dyn Db) -> File {
        match self {
            FileId::File(file) => file,
            FileId::Completion(completion) => completion.file(db),
            FileId::Builtins => crate::builtins::builtins_file(db),
            FileId::Extension(extension) => extension_file(db, extension),
        }
    }
}

pub fn parse(db: &dyn Db, file: impl Into<FileId>) -> Parse<SourceFile> {
    match file.into() {
        FileId::File(file) => parse_file(db, file),
        FileId::Completion(completion) => parse_completion_file(db, completion),
        FileId::Builtins => parse_file(db, crate::builtins::builtins_file(db)),
        FileId::Extension(extension) => parse_file(db, extension_file(db, extension)),
    }
}

#[salsa::interned(no_lifetime)]
struct ExtensionFile {
    extension: BundledExtension,
}

fn extension_file(db: &dyn Db, extension: BundledExtension) -> File {
    extension_source(db, ExtensionFile::new(db, extension))
}

#[salsa::tracked]
fn extension_source(db: &dyn Db, file: ExtensionFile) -> File {
    File::new(db, file.extension(db).sql().into())
}

#[salsa::tracked]
fn parse_file(db: &dyn Db, file: File) -> Parse<SourceFile> {
    SourceFile::parse(file.content(db))
}

#[salsa::tracked]
fn parse_completion_file(db: &dyn Db, completion: CompletionFile) -> Parse<SourceFile> {
    let mut sql = completion.file(db).content(db).to_string();
    let offset = usize::from(completion.offset(db)).min(sql.len());
    sql.insert_str(offset, COMPLETION_MARKER);
    SourceFile::parse(&sql)
}

struct CachedIndex {
    index: Arc<SemanticIndex>,
}

impl PartialEq for CachedIndex {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.index, &other.index)
    }
}

pub(crate) fn semantic_index(db: &dyn Db, file: FileId) -> &SemanticIndex {
    match file {
        FileId::File(file) => &file_index(db, file).index,
        FileId::Completion(file) => &completion_index(db, file).index,
        FileId::Builtins | FileId::Extension(_) => unreachable!(),
    }
}

fn build_index(db: &dyn Db, file: FileId) -> CachedIndex {
    let mut builder = SemanticIndexBuilder::new();
    builder.add_file(InFile::new(file, &parse(db, file).tree()));
    CachedIndex {
        index: Arc::new(builder.build()),
    }
}

#[salsa::tracked(returns(ref), no_eq)]
fn file_index(db: &dyn Db, file: File) -> CachedIndex {
    build_index(db, file.into())
}

#[salsa::tracked(returns(ref), no_eq)]
fn completion_index(db: &dyn Db, file: CompletionFile) -> CachedIndex {
    build_index(db, FileId::Completion(file))
}

#[salsa::tracked(returns(ref))]
pub fn line_index(db: &dyn Db, file: File) -> LineIndex {
    LineIndex::new(file.content(db))
}

pub(crate) fn line_ending(db: &dyn Db, file: impl Into<FileId>) -> &'static str {
    find_newline(file.into().original_file(db).content(db))
        .map(|(_, line_ending)| line_ending)
        .unwrap_or_default()
        .as_str()
}

#[salsa::db]
#[derive(Clone, Default)]
pub struct Database {
    storage: Storage<Self>,
}

impl salsa::Database for Database {}

#[cfg(test)]
mod tests;
