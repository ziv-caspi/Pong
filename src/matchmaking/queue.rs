use super::User;
use queues::{queue, IsQueue, Queue};

pub struct InMemoryPendingPlayersQueue {
    queue: Queue<User>,
}

impl InMemoryPendingPlayersQueue {
    pub fn new() -> Self {
        InMemoryPendingPlayersQueue { queue: queue![] }
    }

    pub fn add(&mut self, user: User) -> bool {
        let result = self.queue.add(user).is_ok();
        println!("added user. size: {:?}", self.queue.size());
        result
    }
}
