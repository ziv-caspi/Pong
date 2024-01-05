use super::{queue::InMemoryPendingPlayersQueue, PendingPlayersQueue, User};
use anyhow::anyhow;
use std::sync::{Arc, Mutex};

// This is for if you would want to use a mutex to share the queue between threads

#[derive(Clone)]
pub struct MutexQueue {
    impl_lock: Arc<Mutex<InMemoryPendingPlayersQueue>>,
}

impl MutexQueue {
    fn new() -> Self {
        let inner = InMemoryPendingPlayersQueue::new();
        Self {
            impl_lock: Arc::new(Mutex::new(inner)),
        }
    }
}

impl PendingPlayersQueue for MutexQueue {
    fn add(&self, user: User) -> anyhow::Result<()> {
        let mut queue = self
            .impl_lock
            .try_lock()
            .or(Err(anyhow!("could not get lock")))?;

        match queue.add(user) {
            true => Ok(()),
            false => Err(anyhow!("could not add to queue")),
        }
    }
}
