use std::result;

use crossbeam::channel::Receiver;
use uuid::Uuid;

use crate::{
    gameplay::game_datalayer::GameDatalayer,
    new_matchmaking::{
        datalayer::{OnMatchStatusChange, OnNewMatch, User},
        rpc_datalayer::RpcMatchmakingDatalayer,
    },
    runner::messages::{
        MatchStatusChange, PotentialMatchUpdate, QueueUpResponse, ServerPushUpdate,
    },
};

use super::messages::{JoinLobbyRequest, JoinLobbyResponse, ServerMessage, UserMessage};

enum ClientState {
    Unactive,
    WaitingForMatch,
    WaitingForMatchApproval,
    InMatchLobby,
    InMatch,
}

pub struct ClientSession<'a, TGameDatalayer> {
    id: Option<String>,
    current_match: Option<String>,
    state: ClientState,

    matchmaking: &'a RpcMatchmakingDatalayer,
    on_new_match: Receiver<OnNewMatch>,
    on_match_change: Receiver<OnMatchStatusChange>,
    gameplay: &'a TGameDatalayer,
}

impl<'a, TGameDatalayer> ClientSession<'a, TGameDatalayer>
where
    TGameDatalayer: GameDatalayer,
{
    pub fn new(matchmaking: &'a RpcMatchmakingDatalayer, gameplay: &'a TGameDatalayer) -> Self {
        let on_new_match = matchmaking.events.on_new_match.subscribe();
        let on_match_change = matchmaking.events.on_match_change.subscribe();

        Self {
            id: None,
            current_match: None,
            state: ClientState::Unactive,
            matchmaking,
            on_new_match,
            on_match_change,
            gameplay,
        }
    }

    pub fn kill_session(&mut self) {
        if let Some(id) = &self.id {
            _ = self.matchmaking.remove_from_queue(id.to_owned());
            if let Some(match_id) = &self.current_match {
                _ = self
                    .matchmaking
                    .remove_from_match(match_id.to_owned(), id.to_owned());
            }
        }
    }

    pub fn process_message(&mut self, message: UserMessage) -> ServerMessage {
        let (message, state) = match (&message, &self.state) {
            (UserMessage::NoUpdates, ClientState::Unactive) => nothing(),
            (UserMessage::NoUpdates, ClientState::WaitingForMatch) => self.waiting_for_match(),
            (UserMessage::NoUpdates, ClientState::WaitingForMatchApproval) => {
                self.update_on_match_change()
            }
            (UserMessage::NoUpdates, ClientState::InMatchLobby) => self.update_on_match_change(),
            (UserMessage::NoUpdates, ClientState::InMatch) => todo!(), // not handled at the moment
            (UserMessage::QueueUpRequest(request), ClientState::Unactive) => {
                self.queue_up_request(request)
            }
            (UserMessage::QueueUpRequest(_), ClientState::WaitingForMatch)
            | (UserMessage::QueueUpRequest(_), ClientState::WaitingForMatchApproval)
            | (UserMessage::QueueUpRequest(_), ClientState::InMatchLobby)
            | (UserMessage::QueueUpRequest(_), ClientState::InMatch) => nothing(),
            (UserMessage::JoinLobbyRequest(_), ClientState::Unactive) => nothing(),
            (UserMessage::JoinLobbyRequest(_), ClientState::WaitingForMatch) => nothing(),
            (UserMessage::JoinLobbyRequest(request), ClientState::WaitingForMatchApproval) => {
                self.join_lobby(request)
            } // magic
            (UserMessage::JoinLobbyRequest(_), ClientState::InMatchLobby) => nothing(),
            (UserMessage::JoinLobbyRequest(_), ClientState::InMatch) => nothing(),
        };

        if let Some(s) = state {
            self.state = s;
        }

        message
    }

    fn update_on_match_change(&self) -> (ServerMessage, Option<ClientState>) {
        if let Ok(change) = self.on_match_change.try_recv() {
            println!("match changed, should notify to client, {:?}", change);
            let (message, state) = match change {
                OnMatchStatusChange::OnTimeout(_) => {
                    let m = MatchStatusChange {
                        start: false,
                        end_reason: String::from("timeout"),
                    };
                    let state = ClientState::Unactive;
                    (m, state)
                }
                OnMatchStatusChange::OnDeath(_) => {
                    let m = MatchStatusChange {
                        start: false,
                        end_reason: String::from("death"),
                    };
                    let state = ClientState::Unactive;
                    (m, state)
                }
                OnMatchStatusChange::OnStart(_) => {
                    let m = MatchStatusChange {
                        start: true,
                        end_reason: String::from(""),
                    };
                    let state = ClientState::InMatch;
                    (m, state)
                }
            };

            return (
                ServerMessage::ServerPushUpdate(Some(ServerPushUpdate::MatchStatusChange(message))),
                Some(state),
            );
        }

        nothing()
    }

    fn waiting_for_match(&mut self) -> (ServerMessage, Option<ClientState>) {
        let my_id = match &self.id {
            Some(i) => i,
            None => return nothing(),
        };

        loop {
            let message = match self.on_new_match.try_recv() {
                Ok(m) => m,
                Err(_) => break,
            };

            let match_for_me = message.players.iter().find(|id| id == &my_id).is_some();
            if !match_for_me {
                continue;
            }

            self.current_match = Some(message.match_id.clone());
            return (
                ServerMessage::ServerPushUpdate(Some(ServerPushUpdate::PotentialMatchUpdate(
                    PotentialMatchUpdate {
                        opoonents_ids: message.players,
                        match_id: message.match_id,
                    },
                ))),
                Some(ClientState::WaitingForMatchApproval),
            );
        }

        nothing()
    }

    fn queue_up_request(
        &mut self,
        request: &super::messages::QueueUpRequest,
    ) -> (ServerMessage, Option<ClientState>) {
        let id = Uuid::new_v4();
        let user = User {
            id: id.to_string(),
            nickname: request.nickname.to_owned(),
        };

        match self.matchmaking.register(user) {
            Ok(_) => {
                self.id = Some(id.to_string());
                return (
                    ServerMessage::QueueUpResponse(Ok(QueueUpResponse { id: id.to_string() })),
                    Some(ClientState::WaitingForMatch),
                );
            }
            Err(_) => {
                return (
                    ServerMessage::QueueUpResponse(Err(String::from("could not register user"))),
                    None,
                )
            }
        };
    }

    fn join_lobby(&self, request: &JoinLobbyRequest) -> (ServerMessage, Option<ClientState>) {
        let user_id = match &self.id {
            Some(i) => i,
            None => {
                return (
                    ServerMessage::JoinLobbyResponse(Err(String::from(
                        "user state says this user is not registered",
                    ))),
                    None,
                );
            }
        };
        let match_id = match &self.current_match {
            Some(i) => i,
            None => {
                return (
                    ServerMessage::JoinLobbyResponse(Err(String::from(
                        "user state says this user is not part of a match",
                    ))),
                    None,
                );
            }
        };
        if match_id != &request.match_id {
            return (
                ServerMessage::JoinLobbyResponse(Err(String::from(
                    "user state says this user is part of a diffrent match",
                ))),
                None,
            );
        }

        let result = self
            .matchmaking
            .mark_player_as_ready(match_id.to_owned(), user_id.to_owned())
            .map(|_| JoinLobbyResponse {
                match_id: match_id.to_owned(),
            })
            .map_err(|e| e.to_string());

        (
            ServerMessage::JoinLobbyResponse(result),
            Some(ClientState::InMatchLobby),
        )
    }
}

fn nothing() -> (ServerMessage, Option<ClientState>) {
    (ServerMessage::ServerPushUpdate(None), None)
}
