use std::sync::{Arc, Mutex};

use crossbeam::channel::{unbounded, Receiver, Sender};

#[derive(Clone)]
pub struct EventTopic<T> {
    subscribed: Arc<Mutex<Vec<Sender<T>>>>,
}

impl<T> EventTopic<T>
where
    T: Clone,
{
    pub fn new() -> Self {
        Self {
            subscribed: Arc::new(Mutex::new(vec![])),
        }
    }

    pub fn subscribe(&self) -> Receiver<T> {
        let mut subs = self.subscribed.lock().unwrap();
        let (s, r) = unbounded();
        subs.push(s);
        r
    }

    pub fn invoke(&self, message: T) {
        let subs = self.subscribed.lock().unwrap();
        for sub in subs.iter() {
            sub.send(message.clone());
        }
    }
}
