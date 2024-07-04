use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientOffer {
    pub to: String,
    pub sdp: String,
    pub message_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientAnswer {
    pub to: String,
    pub sdp: String,
    pub message_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum ClienMessage {
    Answer(ClientAnswer),
    Offer(ClientOffer),
    GetMyID,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerOffer {
    pub from: String,
    pub to: String,
    pub sdp: String,
    pub message_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerAnswer {
    pub from: String,
    pub to: String,
    pub sdp: String,
    pub message_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ID {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum ServerMessage {
    Answer(ServerAnswer),
    Offer(ServerOffer),
    ID(ID),
}