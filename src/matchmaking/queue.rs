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

    pub fn find_match(&mut self, count: usize) -> Option<Vec<User>> {
        if self.queue.size() < count {
            return None;
        }

        let mut players: Vec<User> = vec![];
        for _ in 0..count {
            if let Ok(user) = self.queue.remove() {
                players.push(user);
            }
        }

        Some(players)
    }
}
