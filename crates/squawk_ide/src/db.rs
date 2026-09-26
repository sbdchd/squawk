use rowan::{TextRange, TextSize};
use salsa::Database as Db;
#[cfg(test)]
use salsa::Setter;
use salsa::Storage;
use smallvec::{SmallVec, smallvec};
use squawk_line_index::{LineIndex, find_newline};
use squawk_syntax::ast::{self, AstNode};
use squawk_syntax::sql_body::SqlBody;
use squawk_syntax::{Parse, SourceFile, SyntaxNode, SyntaxNodePtr};
use std::sync::Arc;

use crate::binder;
use crate::binder::{Binder, ResolvedSchemas};
use crate::builtins::builtins_file;
use crate::completion::COMPLETION_MARKER;
use crate::file::InFile;
use crate::name::{AsName, Name, Schema};
use crate::symbols::SymbolKind;

#[salsa::input(debug)]
pub struct File {
    #[returns(ref)]
    pub content: Arc<str>,
}

#[salsa::interned(no_lifetime, debug)]
pub struct EmbeddedFile {
    pub parent: FileId,
    pub literal: TextRange,
}

#[salsa::interned(no_lifetime, debug)]
pub struct CompletionFile {
    pub file: File,
    pub offset: TextSize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, salsa::Update)]
pub enum FileId {
    File(File),
    Embedded(EmbeddedFile),
    Completion(CompletionFile),
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
            FileId::Embedded(embedded) => embedded.parent(db).original_file(db),
            FileId::Completion(completion) => completion.file(db),
        }
    }
}

pub fn parse(db: &dyn Db, file: impl Into<FileId>) -> Parse<SourceFile> {
    match file.into() {
        FileId::File(file) => parse_file(db, file),
        FileId::Embedded(embedded) => match embedded_body(db, embedded) {
            Some(body) => body.to_parse(),
            None => SourceFile::parse(""),
        },
        FileId::Completion(completion) => parse_completion_file(db, completion),
    }
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

pub(crate) fn embedded_literal(db: &dyn Db, embedded: EmbeddedFile) -> Option<ast::Literal> {
    let tree = parse(db, embedded.parent(db)).tree();
    let range = embedded.literal(db);
    tree.syntax()
        .covering_element(range)
        .ancestors()
        .find_map(ast::Literal::cast)
        .filter(|literal| literal.syntax().text_range() == range)
}

pub(crate) fn ancestors_with_embedded(
    db: &dyn Db,
    node: InFile<SyntaxNode>,
) -> impl Iterator<Item = InFile<SyntaxNode>> {
    std::iter::successors(Some(node), move |current| {
        if let Some(parent) = current.value.parent() {
            return Some(InFile::new(current.file_id, parent));
        }
        let FileId::Embedded(embedded) = current.file_id else {
            return None;
        };
        let literal = embedded_literal(db, embedded)?;
        Some(InFile::new(embedded.parent(db), literal.syntax().clone()))
    })
}

#[salsa::tracked(returns(ref))]
pub(crate) fn embedded_body(db: &dyn Db, embedded: EmbeddedFile) -> Option<SqlBody> {
    embedded_literal(db, embedded)?.sql_body()
}

#[salsa::tracked]
pub fn line_index(db: &dyn Db, file: File) -> LineIndex {
    LineIndex::new(file.content(db))
}

pub(crate) fn line_ending(db: &dyn Db, file: impl Into<FileId>) -> &'static str {
    find_newline(file.into().original_file(db).content(db))
        .map(|(_, line_ending)| line_ending)
        .unwrap_or_default()
        .as_str()
}

pub(crate) fn list_files(db: &dyn Db, file: impl Into<FileId>) -> impl Iterator<Item = FileId> {
    let file = file.into();
    let mut files: SmallVec<[FileId; 3]> = smallvec![file];
    let mut current = file;
    while let FileId::Embedded(embedded) = current {
        current = embedded.parent(db);
        files.push(current);
    }
    let builtins = FileId::from(builtins_file(db));
    if include_builtins(db) && file != builtins {
        files.push(builtins);
    }
    files.into_iter()
}

impl InFile<SyntaxNodePtr> {
    pub(crate) fn to_node(self, db: &dyn Db) -> SyntaxNode {
        self.value.to_node(parse(db, self.file_id).tree().syntax())
    }
}

pub(crate) struct Binders<'db> {
    db: &'db dyn Db,
    file: FileId,
}

pub(crate) fn binders(db: &dyn Db, file: impl Into<FileId>) -> Binders<'_> {
    Binders {
        db,
        file: file.into(),
    }
}

impl Binders<'_> {
    pub(crate) fn resolved_schemas(
        &self,
        position: TextSize,
        schema: Option<&Schema>,
    ) -> ResolvedSchemas {
        bind(self.db, self.file).resolved_schemas(position, schema)
    }

    pub(crate) fn find<T>(&self, lookup: impl Fn(&Binder) -> Option<T>) -> Option<InFile<T>> {
        list_files(self.db, self.file)
            .find_map(|file| lookup(&bind(self.db, file)).map(|value| InFile::new(file, value)))
    }

    pub(crate) fn lookup<N: AsName + ?Sized>(
        &self,
        name: &N,
        kind: SymbolKind,
    ) -> Option<InFile<SyntaxNodePtr>> {
        self.find(|binder| binder.lookup(name, kind))
    }

    pub(crate) fn lookup_with<N: AsName + ?Sized>(
        &self,
        name: &N,
        kind: SymbolKind,
        schemas: &ResolvedSchemas,
    ) -> Option<InFile<SyntaxNodePtr>> {
        self.find(|binder| binder.lookup_with(name, kind, schemas))
    }

    pub(crate) fn lookup_with_params<N: AsName + ?Sized>(
        &self,
        name: &N,
        kind: SymbolKind,
        schemas: &ResolvedSchemas,
        params: Option<&[Name]>,
    ) -> Option<InFile<SyntaxNodePtr>> {
        self.find(|binder| binder.lookup_with_params(name, kind, schemas, params))
    }

    pub(crate) fn lookup_with_table<N: AsName + ?Sized>(
        &self,
        name: &N,
        kind: SymbolKind,
        schemas: &ResolvedSchemas,
        table: &Option<Name>,
    ) -> Option<InFile<SyntaxNodePtr>> {
        self.find(|binder| binder.lookup_with_table(name, kind, schemas, table))
    }

    pub(crate) fn lookup_info<N: AsName + ?Sized>(
        &self,
        name: &N,
        kind: SymbolKind,
        schemas: &ResolvedSchemas,
    ) -> Option<(Schema, String)> {
        self.find(|binder| binder.lookup_info(name, kind, schemas))
            .map(|info| info.value)
    }
}

pub(crate) fn bind(db: &dyn Db, file: impl Into<FileId>) -> Binder {
    match file.into() {
        FileId::File(file) => bind_file(db, file),
        FileId::Embedded(embedded) => bind_embedded(db, embedded),
        FileId::Completion(completion) => bind_completion_file(db, completion),
    }
}

#[salsa::tracked]
fn bind_file(db: &dyn Db, file: File) -> Binder {
    let result = parse(db, file);
    let source_file = result.tree();
    binder::bind(&source_file)
}

#[salsa::tracked]
fn bind_completion_file(db: &dyn Db, completion: CompletionFile) -> Binder {
    let source_file = parse(db, FileId::Completion(completion)).tree();
    binder::bind(&source_file)
}

#[salsa::tracked]
fn bind_embedded(db: &dyn Db, embedded: EmbeddedFile) -> Binder {
    let parent = bind(db, embedded.parent(db));
    let search_path = parent.search_path_at(embedded.literal(db).start()).to_vec();
    let source_file = parse(db, FileId::Embedded(embedded)).tree();
    binder::bind_with_search_path(&source_file, search_path)
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
