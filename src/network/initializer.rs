use std::thread;

use crate::{
    matchmaking::{
        queue_up_api::{MatchMaker, OnNewMatch, QueueApi},
        rpc_queue::RpcQueue,
    },
    utils::events::EventTopic,
};

pub struct Initializer {
    queue: RpcQueue,
    match_events: EventTopic<OnNewMatch>,
}

impl Initializer {
    pub fn init() -> Initializer {
        let queue = RpcQueue::new();
        let new_match_topic = EventTopic::new();

        let matchmaker = MatchMaker::new(queue.clone(), new_match_topic.clone());
        thread::spawn(move || {
            matchmaker.find_matches();
        });

        Initializer {
            queue,
            match_events: new_match_topic,
        }
    }

    pub fn get_queue_api(&mut self) -> QueueApi<RpcQueue> {
        let reciever = self.match_events.subscribe();

        QueueApi::new(self.queue.clone(), self.match_events.clone())
    }
}
