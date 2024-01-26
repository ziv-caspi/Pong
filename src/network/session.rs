use crossbeam::channel::Receiver;

use crate::{
    matchmaking::{queue_up_api::QueueApi, rpc_queue::RpcQueue},
    network::messages::{PotentialMatchUpdate, QueueUpResponse, ServerPushUpdate},
};

use super::messages::{ServerMessage, UserMessage};

enum ClientState {
    Unactive,
    WaitingForMatch,
    WaintingForMatchApproval,
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
        let (message, state) = match message {
            UserMessage::QueueUpRequest(request) => self.queue_up_request(request),
            UserMessage::NoUpdates => self.no_updates(),
        };

        if let Some(s) = state {
            self.state = s;
        }

        message
    }

    fn no_updates(&self) -> (ServerMessage, Option<ClientState>) {
        match self.state {
            ClientState::Unactive => (ServerMessage::ServerPushUpdate(None), None),
            ClientState::WaitingForMatch => {
                let my_id = match &self.id {
                    Some(i) => i,
                    None => return (ServerMessage::ServerPushUpdate(None), None),
                };

                if let Some(m) = get_last_message(&self.queue_api.new_match_reciever) {
                    let match_for_me = m.player_ids.iter().find(|id| id == &my_id).is_some();
                    if !match_for_me {
                        return (ServerMessage::ServerPushUpdate(None), None);
                    }
                    return (
                        ServerMessage::ServerPushUpdate(Some(
                            ServerPushUpdate::PotentialMatchUpdate(PotentialMatchUpdate {
                                opoonents_ids: m.player_ids,
                            }),
                        )),
                        Some(ClientState::WaintingForMatchApproval),
                    );
                }

                (ServerMessage::ServerPushUpdate(None), None)
            }
            ClientState::WaintingForMatchApproval => todo!(),
            ClientState::InMatch => todo!(),
        }
    }

    fn queue_up_request(
        &self,
        request: super::messages::QueueUpRequest,
    ) -> (ServerMessage, Option<ClientState>) {
        let result = match self.queue_api.register_to_queue(&request.nickname) {
            Ok(user) => Ok(QueueUpResponse { id: user.id }),
            Err(e) => Err(e.to_string()),
        };

        let state = match result {
            Ok(_) => Some(ClientState::WaitingForMatch),
            Err(_) => None,
        };

        (ServerMessage::QueueUpResponse(result), state)
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
