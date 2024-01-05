use crate::matchmaking::{queue_up_api::QueueApi, rpc_queue::RpcQueue};

pub struct Initializer {
    queue: RpcQueue,
}

impl Initializer {
    pub fn init() -> Initializer {
        let queue = RpcQueue::new();

        Initializer { queue }
    }

    pub fn get_queue_api(&self) -> QueueApi<RpcQueue> {
        QueueApi {
            queue: self.queue.clone(),
        }
    }
}
