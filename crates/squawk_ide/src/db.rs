use rowan::TextSize;
use salsa::{Setter, Storage};
use squawk_line_index::{LineIndex, find_newline};
use squawk_sema::extensions::BundledExtension;
use squawk_sema::file::{FileId, FileIndex, InFile};
use squawk_sema::semantic_index::SemanticIndex;
use squawk_sema::semantic_index_builder::SemanticIndexBuilder;
use squawk_syntax::{Parse, SourceFile};
use std::sync::Arc;

use crate::builtins::builtins_file;
use crate::completion::COMPLETION_MARKER;

#[salsa::input(debug)]
#[derive(PartialOrd, Ord)]
pub struct File {
    #[returns(ref)]
    pub content: Arc<str>,
    pub file_id: FileId,
}

#[salsa::interned(no_lifetime, debug)]
#[derive(PartialOrd, Ord)]
pub struct CompletionFile {
    pub file: File,
    pub offset: TextSize,
}

#[salsa::db]
pub trait Db: salsa::Database {
    fn file(&self, file: FileIndex) -> File;
}

pub fn original_file(db: &dyn Db, file_id: FileId) -> File {
    match file_id {
        FileId::File(file) | FileId::Completion { file, .. } => db.file(file),
        FileId::Builtins => builtins_file(db),
        FileId::Extension(extension) => extension_file(db, extension),
    }
}

pub fn parse(db: &dyn Db, file_id: FileId) -> Parse<SourceFile> {
    match file_id {
        FileId::File(file) => parse_file(db, db.file(file)),
        FileId::Completion { file, offset } => {
            parse_completion_file(db, CompletionFile::new(db, db.file(file), offset))
        }
        FileId::Builtins => parse_file(db, builtins_file(db)),
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
    let extension = file.extension(db);
    File::new(db, extension.sql().into(), FileId::Extension(extension))
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

pub fn semantic_index(db: &dyn Db, file_id: FileId) -> &SemanticIndex {
    match file_id {
        FileId::File(file) => &file_index(db, db.file(file)).index,
        FileId::Completion { file, offset } => {
            &completion_index(db, CompletionFile::new(db, db.file(file), offset)).index
        }
        FileId::Builtins | FileId::Extension(_) => unreachable!(),
    }
}

fn build_index(db: &dyn Db, file_id: FileId) -> CachedIndex {
    let mut builder = SemanticIndexBuilder::new();
    builder.add_file(InFile::new(file_id, &parse(db, file_id).tree()));
    CachedIndex {
        index: Arc::new(builder.build()),
    }
}

#[salsa::tracked(returns(ref), no_eq)]
fn file_index(db: &dyn Db, file: File) -> CachedIndex {
    build_index(db, file.file_id(db))
}

#[salsa::tracked(returns(ref), no_eq)]
fn completion_index(db: &dyn Db, completion: CompletionFile) -> CachedIndex {
    let FileId::File(file) = completion.file(db).file_id(db) else {
        unreachable!()
    };
    build_index(
        db,
        FileId::Completion {
            file,
            offset: completion.offset(db),
        },
    )
}

pub fn line_index(db: &dyn Db, file_id: FileId) -> &LineIndex {
    #[salsa::interned(no_lifetime)]
    struct InternedFileId {
        id: FileId,
    }

    #[salsa::tracked(returns(ref))]
    fn line_index(db: &dyn Db, file_id: InternedFileId) -> LineIndex {
        LineIndex::new(original_file(db, file_id.id(db)).content(db))
    }

    line_index(db, InternedFileId::new(db, file_id))
}

pub fn line_ending(db: &dyn Db, file_id: FileId) -> &'static str {
    find_newline(original_file(db, file_id).content(db))
        .map(|(_, line_ending)| line_ending)
        .unwrap_or_default()
        .as_str()
}

#[salsa::db]
#[derive(Clone, Default)]
pub struct Database {
    storage: Storage<Self>,
    files: Vec<File>,
}

impl Database {
    pub fn add_file(&mut self, content: Arc<str>) -> FileIndex {
        let index = FileIndex::from_raw(u32::try_from(self.files.len()).unwrap());
        let file = File::new(self, content, FileId::File(index));
        self.files.push(file);
        index
    }

    pub fn set_file_content(&mut self, file: FileIndex, content: Arc<str>) {
        let file = self.files[file.index()];
        file.set_content(self).to(content);
    }
}

#[salsa::db]
impl salsa::Database for Database {}

#[salsa::db]
impl Db for Database {
    fn file(&self, file: FileIndex) -> File {
        self.files[file.index()]
    }
}

#[cfg(test)]
mod tests;
