use std::{thread, time::Duration};

use crate::utils::events::EventTopic;

use super::{PendingPlayersQueue, User};
use anyhow::Result;
use crossbeam::channel::{unbounded, Receiver};
use uuid::Uuid;

#[derive(Clone)]
pub struct OnNewMatch {
    pub player_ids: Vec<String>,
}

pub struct QueueApi<Q> {
    new_match_event: EventTopic<OnNewMatch>,
    pub new_match_reciever: Receiver<OnNewMatch>,
    pub queue: Q,
}

impl<Q> QueueApi<Q>
where
    Q: PendingPlayersQueue,
{
    pub fn new(queue: Q, new_match: EventTopic<OnNewMatch>) -> Self {
        Self {
            queue,
            new_match_reciever: new_match.subscribe(),
            new_match_event: new_match,
        }
    }

    pub fn register_to_queue(&self, nickname: &str) -> Result<User> {
        let id = Uuid::new_v4();

        let user = User {
            id: id.to_string(),
            nickname: String::from(nickname),
        };

        let _ = self.queue.add(user.clone());

        Ok(user)
    }

    fn find_matches(&self) {
        loop {
            if let Ok(result) = self.queue.find_match() {
                if let Some(m) = result {
                    let ids: Vec<String> = m.iter().map(|user| user.id.to_owned()).collect();
                    self.new_match_event.invoke(OnNewMatch { player_ids: ids });
                }
            }
        }
    }
}

pub struct MatchMaker<Queue> {
    queue: Queue,
    new_match_event: EventTopic<OnNewMatch>,
}

impl<Queue> MatchMaker<Queue>
where
    Queue: PendingPlayersQueue,
{
    pub fn new(queue: Queue, topic: EventTopic<OnNewMatch>) -> Self {
        Self {
            queue,
            new_match_event: topic,
        }
    }

    pub fn find_matches(&self) {
        loop {
            if let Ok(result) = self.queue.find_match() {
                if let Some(m) = result {
                    let ids: Vec<String> = m.iter().map(|user| user.id.to_owned()).collect();
                    self.new_match_event.invoke(OnNewMatch { player_ids: ids });
                }
            } else {
                thread::sleep(Duration::from_millis(100));
            }
        }
    }
}
