use std::{num::NonZeroUsize, sync::Arc};

use crossbeam_channel::Sender;
use lsp_server::Message;
use lsp_types::Url;
use rustc_hash::FxHashMap;
use salsa::Setter;
use squawk_ide::db::{Database, File};
use squawk_thread::TaskPool;

pub(crate) trait System {
    fn db(&self) -> &Database;
    fn file(&self, uri: &Url) -> Option<File>;
}

pub(crate) trait MutableSystem: System {
    fn set(&mut self, uri: Url, content: String);
    fn remove(&mut self, uri: &Url);
}

pub(crate) struct Snapshot {
    db: Database,
    files: Arc<FxHashMap<Url, File>>,
}

impl System for Snapshot {
    fn db(&self) -> &Database {
        &self.db
    }

    fn file(&self, uri: &Url) -> Option<File> {
        self.files.get(uri).copied()
    }
}

pub(super) struct GlobalState {
    db: Database,
    files: Arc<FxHashMap<Url, File>>,
    pub(crate) sender: Sender<Message>,
    pub(crate) task_pool: TaskPool<Message>,
}

impl GlobalState {
    pub(super) fn new(sender: Sender<Message>, threads: NonZeroUsize) -> Self {
        Self {
            db: Database::default(),
            files: Arc::new(FxHashMap::default()),
            task_pool: TaskPool::new_with_threads(sender.clone(), threads),
            sender,
        }
    }

    pub(crate) fn snapshot(&self) -> Snapshot {
        Snapshot {
            db: self.db.clone(),
            files: self.files.clone(),
        }
    }
}

impl System for GlobalState {
    fn db(&self) -> &Database {
        &self.db
    }

    fn file(&self, uri: &Url) -> Option<File> {
        self.files.get(uri).copied()
    }
}

impl MutableSystem for GlobalState {
    fn set(&mut self, uri: Url, content: String) {
        if let Some(file) = self.files.get(&uri).copied() {
            file.set_content(&mut self.db).to(content.into());
        } else {
            let file = File::new(&self.db, content.into());
            Arc::make_mut(&mut self.files).insert(uri, file);
        }
    }

    fn remove(&mut self, uri: &Url) {
        Arc::make_mut(&mut self.files).remove(uri);
    }
}
