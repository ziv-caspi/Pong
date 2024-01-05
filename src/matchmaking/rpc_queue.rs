use super::{queue::InMemoryPendingPlayersQueue, PendingPlayersQueue, User};
use crate::utils::rpc::ThreadSafeAPI;
use crate::utils::rpc::{Channel, GenericInnerRpcHandler, RpcMessage, RpcMethod};
use anyhow::anyhow;
use anyhow::Result;
use crossbeam::channel::{unbounded, Receiver, Sender};
use std::{thread, time::Duration};
// This is for if you want to use channels and an RPC API to use the queue between threads

// could maybe be a macro, cant think of a generic solution right now
struct InnerRpcHandler {
    queue: InMemoryPendingPlayersQueue,
    add_channel: Channel<RpcMessage<User, bool>>,
}

impl InnerRpcHandler {
    fn new() -> Self {
        let queue = InMemoryPendingPlayersQueue::new();
        let (add_s, add_r) = unbounded();
        InnerRpcHandler {
            queue: queue,
            add_channel: Channel {
                reciever: add_r,
                sender: add_s,
            },
        }
    }

    fn start(self) {
        // here you could do threadpool stuff if you wanted to
        thread::spawn(|| self.handle_requests());
    }

    fn handle_requests(mut self) {
        loop {
            let request = self.add_channel.reciever.recv().unwrap();
            _ = request
                .response_channel
                .send(self.queue.add(request.message));
        }
    }
}

#[derive(Clone)]
pub struct RpcQueue {
    add: RpcMethod<User, bool>,
}

impl RpcQueue {
    pub fn new() -> Self {
        let inner = InnerRpcHandler::new();
        let queue = RpcQueue {
            add: RpcMethod {
                sender: inner.add_channel.sender.clone(),
            },
        };

        inner.start();
        queue
    }
}

impl PendingPlayersQueue for RpcQueue {
    fn add(&self, user: User) -> anyhow::Result<()> {
        self.add.invoke(user, Duration::from_secs(3))?;
        Ok(())
    }
}

// new idea

#[derive(Clone)]
enum QueueMethod {
    Add(RpcMessage<User, bool>),
}

#[derive(Clone)]
struct NewRpcQueue {
    api: ThreadSafeAPI<QueueMethod>, // could probably use RpcMethod of some sort
}

impl NewRpcQueue {
    pub fn new() -> Self {
        let inner =
            GenericInnerRpcHandler::new(InMemoryPendingPlayersQueue::new(), Box::new(Self::handle));

        let queue = NewRpcQueue {
            api: ThreadSafeAPI::new(inner.general_channel.sender.clone()),
        };

        inner.start();
        queue
    }

    fn handle(queue: &mut InMemoryPendingPlayersQueue, method: QueueMethod) {
        match method {
            QueueMethod::Add(requset) => {
                _ = requset.response_channel.send(queue.add(requset.message));
            }
        }
    }
}

impl PendingPlayersQueue for NewRpcQueue {
    fn add(&self, user: User) -> anyhow::Result<()> {
        let (message, recv) = RpcMessage::<User, bool>::new(user);
        let response = self.api.call(QueueMethod::Add(message), recv)?;
        match response {
            true => Ok(()),
            false => Err(anyhow!("could not add")),
        }
    }
}
