use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UserMessage {
    NoUpdates,
    QueueUpRequest(QueueUpRequest),
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ServerMessage {
    QueueUpResponse(Result<QueueUpResponse, String>),
    ServerPushUpdate(Option<ServerPushUpdate>),
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ServerPushUpdate {
    PotentialMatchUpdate(PotentialMatchUpdate),
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct QueueUpRequest {
    pub nickname: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct QueueUpResponse {
    pub id: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PotentialMatchUpdate {
    pub opoonents_ids: Vec<String>,
}
