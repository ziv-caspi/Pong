use crossbeam::channel::Receiver;

use crate::{
    matchmaking::{queue_up_api::QueueApi, rpc_queue::RpcQueue},
    network::messages::{PotentialMatchUpdate, QueueUpResponse, ServerPushUpdate},
};

use super::messages::{ServerMessage, UserMessage};

enum ClientState {
    Unactive,
    WaitingForMatch,
    InMatch,
}

pub struct ClientSession<'a> {
    id: Option<String>,
    state: ClientState,
    queue_api: &'a QueueApi<RpcQueue>,
}

impl<'a> ClientSession<'a> {
    pub fn new(queue_api: &'a QueueApi<RpcQueue>) -> Self {
        Self {
            id: None,
            state: ClientState::Unactive,
            queue_api,
        }
    }

    pub fn process_message(&mut self, message: UserMessage) -> ServerMessage {
        match message {
            UserMessage::QueueUpRequest(request) => return self.queue_up_request(request),
            UserMessage::NoUpdates => return self.no_updates(),
        };
    }

    fn no_updates(&mut self) -> ServerMessage {
        match self.state {
            ClientState::Unactive => ServerMessage::ServerPushUpdate(None),
            ClientState::WaitingForMatch => {
                let my_id = match &self.id {
                    Some(i) => i,
                    None => return ServerMessage::ServerPushUpdate(None),
                };

                if let Some(m) = get_last_message(&self.queue_api.new_match_reciever) {
                    let match_for_me = m.player_ids.iter().find(|id| id == &my_id).is_some();
                    if !match_for_me {
                        return ServerMessage::ServerPushUpdate(None);
                    }
                    self.state = ClientState::InMatch;
                    return ServerMessage::ServerPushUpdate(Some(
                        ServerPushUpdate::PotentialMatchUpdate(PotentialMatchUpdate {
                            opoonents_ids: m.player_ids,
                        }),
                    ));
                }
                ServerMessage::ServerPushUpdate(None)
            }
            ClientState::InMatch => ServerMessage::ServerPushUpdate(None),
        }
    }

    fn queue_up_request(&mut self, request: super::messages::QueueUpRequest) -> ServerMessage {
        let result = match self.queue_api.register_to_queue(&request.nickname) {
            Ok(user) => {
                self.state = ClientState::WaitingForMatch;
                Ok(QueueUpResponse { id: user.id })
            }
            Err(e) => Err(e.to_string()),
        };
        ServerMessage::QueueUpResponse(result)
    }
}

fn get_last_message<T>(reciever: &Receiver<T>) -> Option<T> {
    if reciever.is_empty() {
        return None;
    }

    match reciever.recv() {
        Ok(m) => return Some(m),
        Err(_) => return None,
    }
}
