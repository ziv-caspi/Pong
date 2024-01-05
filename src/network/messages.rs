use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UserMessage {
    QueueUpRequest(QueueUpRequest),
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ServerMessage {
    QueueUpResponse(Result<QueueUpResponse, String>),
    PotentialMatchUpdate(Result<PotentialMatchUpdate, String>),
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
