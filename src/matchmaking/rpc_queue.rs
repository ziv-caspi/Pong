use super::{queue::InMemoryPendingPlayersQueue, PendingPlayersQueue, User};
use crate::utils::rpc::ThreadSafeAPI;
use crate::utils::rpc::{GenericInnerRpcHandler, RpcMessage};
use anyhow::anyhow;

// This is for if you want to use channels and an RPC API to use the queue between threads

#[derive(Clone)]
enum QueueMethod {
    Add(RpcMessage<User, bool>),
}

#[derive(Clone)]
pub struct RpcQueue {
    api: ThreadSafeAPI<QueueMethod>,
}

impl RpcQueue {
    pub fn new() -> Self {
        let inner = GenericInnerRpcHandler::new(
            InMemoryPendingPlayersQueue::new(),
            Box::new(Self::handle_request),
        );

        let queue = RpcQueue {
            api: ThreadSafeAPI::new(inner.general_channel.sender.clone()),
        };

        inner.start();
        queue
    }

    fn handle_request(queue: &mut InMemoryPendingPlayersQueue, method: QueueMethod) {
        match method {
            QueueMethod::Add(requset) => {
                _ = requset.response_channel.send(queue.add(requset.message));
            }
        }
    }
}

impl PendingPlayersQueue for RpcQueue {
    fn add(&self, user: User) -> anyhow::Result<()> {
        let (message, recv) = RpcMessage::<User, bool>::new(user);
        let response = self.api.call(QueueMethod::Add(message), recv)?;
        match response {
            true => Ok(()),
            false => Err(anyhow!("could not add")),
        }
    }
}
