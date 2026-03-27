// Taken from:
// https://github.com/rust-lang/rust-analyzer/blob/2efc80078029894eec0699f62ec8d5c1a56af763/crates/rust-analyzer/src/task_pool.rs#L6C19-L6C19
//! A thin wrapper around [`crate::Pool`] which threads a sender through spawned jobs.

use std::num::NonZeroUsize;

use crossbeam_channel::Sender;

use crate::{Pool, ThreadIntent};

pub struct TaskPool<T> {
    sender: Sender<T>,
    pool: Pool,
}

impl<T> TaskPool<T> {
    pub fn new_with_threads(sender: Sender<T>, threads: NonZeroUsize) -> TaskPool<T> {
        TaskPool {
            sender,
            pool: Pool::new(threads),
        }
    }

    pub fn spawn<F>(&mut self, intent: ThreadIntent, task: F)
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        self.pool.spawn(intent, {
            let sender = self.sender.clone();
            move || sender.send(task()).unwrap()
        })
    }

    pub fn spawn_with_sender<F>(&mut self, intent: ThreadIntent, task: F)
    where
        F: FnOnce(Sender<T>) + Send + 'static,
        T: Send + 'static,
    {
        self.pool.spawn(intent, {
            let sender = self.sender.clone();
            move || task(sender)
        })
    }

    pub fn len(&self) -> usize {
        self.pool.len()
    }

    pub fn is_empty(&self) -> bool {
        self.pool.len() == 0
    }
}
