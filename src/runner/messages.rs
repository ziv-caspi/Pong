use serde_derive::{Deserialize, Serialize};

use crate::gameplay::game::OnGameStateUpdate;

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UserMessage {
    NoUpdates,
    QueueUpRequest(QueueUpRequest),
    JoinLobbyRequest(JoinLobbyRequest),
    MovePlayerRequest(MovePlayerRequest),
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ServerMessage {
    QueueUpResponse(Result<QueueUpResponse, String>),
    JoinLobbyResponse(Result<JoinLobbyResponse, String>),
    ServerPushUpdate(Option<ServerPushUpdate>),
    MovePlayerResponse(Result<MovePlayerResponse, String>),
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ServerPushUpdate {
    PotentialMatchUpdate(PotentialMatchUpdate),
    MatchStatusChange(MatchStatusChange),
    GameStateChange(OnGameStateUpdate),
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
pub struct MovePlayerRequest {
    pub match_id: String,
    pub y_delta: i32,
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
    pub match_id: String,
    pub opoonents_ids: Vec<PotentialPlayer>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PotentialPlayer {
    pub id: String,
    pub nickname: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum MatchStatusChange {
    Stop(String),
    Start(String, String),
    PlayerReady(String),
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MovePlayerResponse {
    pub match_id: String,
}
