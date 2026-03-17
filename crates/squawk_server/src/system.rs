use lsp_types::Url;
use salsa::Setter;
use squawk_ide::db::{Database, File};
use std::collections::HashMap;

pub(crate) struct Document {
    pub(crate) content: String,
}

pub(crate) trait System {
    fn db(&self) -> &Database;
    fn file(&self, uri: &Url) -> Option<File>;
    fn set(&mut self, uri: Url, doc: Document);
    fn remove(&mut self, uri: &Url);
}

pub(super) struct GlobalState {
    pub db: Database,
    files: HashMap<Url, File>,
}

impl GlobalState {
    pub(super) fn new() -> Self {
        Self {
            db: Database::default(),
            files: HashMap::new(),
        }
    }
}

impl System for GlobalState {
    fn db(&self) -> &Database {
        return &self.db;
    }

    fn file(&self, uri: &Url) -> Option<File> {
        self.files.get(uri).copied()
    }

    fn set(&mut self, uri: Url, doc: Document) {
        if let Some(file) = self.files.get(&uri).copied() {
            file.set_content(&mut self.db).to(doc.content.into());
        } else {
            let file = File::new(&self.db, doc.content.into());
            self.files.insert(uri, file);
        }
    }

    fn remove(&mut self, uri: &Url) {
        self.files.remove(uri);
    }
}
