use super::{PendingPlayersQueue, User};
use anyhow::Result;
use uuid::Uuid;

pub struct QueueApi<Q> {
    pub queue: Q,
}

impl<Q> QueueApi<Q>
where
    Q: PendingPlayersQueue,
{
    pub fn register_to_queue(&self, nickname: &str) -> Result<User> {
        let id = Uuid::new_v4();

        let user = User {
            id: id.to_string(),
            nickname: String::from(nickname),
        };

        let _ = self.queue.add(user.clone());

        Ok(user)
    }
}
