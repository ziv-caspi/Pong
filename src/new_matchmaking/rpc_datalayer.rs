use super::datalayer::{DataLayerEvents, MatchmakingDataLayer, OnNewMatch, User};
use crate::utils::{
    events::{self, EventTopic},
    rpc::{GenericInnerRpcHandler, RpcMessage, ThreadSafeAPI},
};
use anyhow::{bail, Result};

#[derive(Clone)]
enum Method {
    Register(RpcMessage<User, Option<String>>), // (user) -> Option<ErrorMessage>
    RemoveFromQueue(RpcMessage<String, Option<String>>), // (player_id) -> Option<ErrorMessage>
    LookForMatches(RpcMessage<(), ()>),
    MarkPlayerAsReady(RpcMessage<(String, String), Option<String>>), // (match_id, player_id) -> -> Option<ErrorMessage>
    RemoveFromMatch(RpcMessage<(String, String), Option<String>>), // ((match_id, player_id) -> -> Option<ErrorMessage>
}

#[derive(Clone)]
pub struct RpcMatchmakingDatalayer {
    pub events: DataLayerEvents,
    api: ThreadSafeAPI<Method>,
}

impl RpcMatchmakingDatalayer {
    pub fn new() -> Self {
        let datalayer = MatchmakingDataLayer::new();
        let events = datalayer.events.clone();
        let inner = GenericInnerRpcHandler::new(datalayer, Box::new(Self::handle_request));

        let queue = RpcMatchmakingDatalayer {
            api: ThreadSafeAPI::new(inner.general_channel.sender.clone()),
            events,
        };

        inner.start();
        queue
    }

    fn handle_request(datalayer: &mut MatchmakingDataLayer, method: Method) {
        match method {
            Method::Register(m) => {
                m.response_channel
                    .send(result_to_string(datalayer.register(m.message)));
            }
            Method::RemoveFromQueue(m) => {
                m.response_channel
                    .send(result_to_string(datalayer.remove_from_queue(m.message)));
            }
            Method::LookForMatches(m) => {
                datalayer.look_for_matches();
                m.response_channel.send(());
            }
            Method::MarkPlayerAsReady(m) => {
                m.response_channel.send(result_to_string(
                    datalayer.mark_player_as_ready(m.message.0, m.message.1),
                ));
            }
            Method::RemoveFromMatch(m) => {
                m.response_channel.send(result_to_string(
                    datalayer.remove_from_match(m.message.0, m.message.1),
                ));
            }
        };
    }

    pub fn register(&self, user: User) -> Result<()> {
        let (message, reciever) = RpcMessage::new(user);
        let response = self.api.call(Method::Register(message), reciever)?;
        if let Some(err) = response {
            bail!(err);
        }

        Ok(())
    }

    pub fn remove_from_queue(&self, user_id: String) -> Result<()> {
        let (message, reciever) = RpcMessage::new(user_id);
        let response = self.api.call(Method::RemoveFromQueue(message), reciever)?;
        if let Some(err) = response {
            bail!(err);
        }

        Ok(())
    }

    pub fn look_for_matches(&self) -> Result<()> {
        let (message, reciever) = RpcMessage::new(());
        self.api.call(Method::LookForMatches(message), reciever)
    }

    pub fn mark_player_as_ready(&self, match_id: String, user_id: String) -> Result<()> {
        let (message, reciever) = RpcMessage::new((match_id, user_id));
        let response = self
            .api
            .call(Method::MarkPlayerAsReady(message), reciever)?;
        if let Some(err) = response {
            bail!(err);
        }

        Ok(())
    }

    pub fn remove_from_match(&self, match_id: String, user_id: String) -> Result<()> {
        let (message, reciever) = RpcMessage::new((match_id, user_id));
        let response = self.api.call(Method::RemoveFromMatch(message), reciever)?;
        if let Some(err) = response {
            bail!(err);
        }

        Ok(())
    }
}

fn result_to_string<T>(res: Result<T>) -> Option<String> {
    match res {
        Ok(_) => None,
        Err(e) => Some(e.to_string()),
    }
}
