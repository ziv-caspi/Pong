use anyhow::Result;
pub mod mutex_queue;
mod queue;
pub mod queue_up_api;
pub mod rpc_queue;
pub trait PendingPlayersQueue: Clone {
    fn add(&self, user: User) -> Result<()>;
    fn find_match(&self) -> Result<Option<Vec<User>>>;
}

#[derive(Clone)]
pub struct User {
    pub id: String,
    pub nickname: String,
}
