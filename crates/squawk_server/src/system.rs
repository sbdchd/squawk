use lsp_types::Url;
use rustc_hash::FxHashMap;
use salsa::Setter;
use squawk_ide::db::{Database, File};

pub(crate) trait System {
    fn db(&self) -> &Database;
    fn file(&self, uri: &Url) -> Option<File>;
    fn set(&mut self, uri: Url, content: String);
    fn remove(&mut self, uri: &Url);
}

pub(super) struct GlobalState {
    pub db: Database,
    files: FxHashMap<Url, File>,
}

impl GlobalState {
    pub(super) fn new() -> Self {
        Self {
            db: Database::default(),
            files: FxHashMap::default(),
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

    fn set(&mut self, uri: Url, content: String) {
        if let Some(file) = self.files.get(&uri).copied() {
            file.set_content(&mut self.db).to(content.into());
        } else {
            let file = File::new(&self.db, content.into());
            self.files.insert(uri, file);
        }
    }

    fn remove(&mut self, uri: &Url) {
        self.files.remove(uri);
    }
}
