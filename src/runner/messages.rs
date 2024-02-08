use serde_derive::{Deserialize, Serialize};

use crate::new_matchmaking::datalayer::OnMatchStatusChange;

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UserMessage {
    NoUpdates,
    QueueUpRequest(QueueUpRequest),
    JoinLobbyRequest(JoinLobbyRequest),
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ServerMessage {
    QueueUpResponse(Result<QueueUpResponse, String>),
    JoinLobbyResponse(Result<JoinLobbyResponse, String>),
    ServerPushUpdate(Option<ServerPushUpdate>),
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ServerPushUpdate {
    PotentialMatchUpdate(PotentialMatchUpdate),
    MatchStatusChange(MatchStatusChange),
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct QueueUpRequest {
    pub nickname: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JoinLobbyRequest {
    pub match_id: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct QueueUpResponse {
    pub id: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JoinLobbyResponse {
    pub match_id: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PotentialMatchUpdate {
    pub match__id: String,
    pub opoonents_ids: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MatchStatusChange {
    pub start: bool,
    pub end_reason: String,
}
