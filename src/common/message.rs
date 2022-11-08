use crate::id::create_id;
use crate::id::IdType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MessageType {
    Unknown,           // will always have empty payload, used when given garbage data
    Message,           // both client -> server and server -> client, used to send a message
    ConnectionReceive, // server -> client, used to send a connection message
    Login,             // client -> server, used to login
}

impl PartialEq for MessageType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (MessageType::Unknown, MessageType::Unknown) => true,
            (MessageType::Message, MessageType::Message) => true,
            (MessageType::ConnectionReceive, MessageType::ConnectionReceive) => true,
            (MessageType::Login, MessageType::Login) => true,
            _ => false,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub id: u64,
    pub message_type: MessageType,
    pub payload: Vec<u8>,
}

impl Message {
    pub fn new(message_type: MessageType, payload: Vec<u8>) -> Message {
        let id = create_id(IdType::Message);
        Message {
            id,
            message_type,
            payload,
        }
    }

    pub fn create_all(id: u64, message_type: MessageType, payload: Vec<u8>) -> Message {
        Message {
            id,
            message_type,
            payload,
        }
    }

    pub fn change_payload(&mut self, payload: Vec<u8>) {
        self.payload = payload;
    }

    pub fn to_bson(&self) -> Vec<u8> {
        bson::to_vec(self).unwrap()
    }

    pub fn from_bson(bson: Vec<u8>) -> Message {
        // try to deserialize the bson, if it fails, return an unknown message
        match bson::from_slice(&bson) {
            Ok(message) => message,
            Err(_) => Message::new(MessageType::Unknown, vec![]),
        }
    }

    pub fn length(&self) -> usize {
        self.to_bson().len()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AcceptPayload {
    pub original_type: MessageType,
    pub original_id: u64,
    payload: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RejectPayload {
    pub original_type: MessageType,
    pub original_id: u64,
    pub reason: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerJoinPayload {
    pub user_id: u64,
    pub username: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChannelJoinPayload {
    pub channel_id: u64,
    pub channel_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChannelLeavePayload {
    pub channel_id: u64,
    pub channel_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserInfoPayload {
    pub user_id: u64,
    pub username: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageSendPayload {
    pub channel_name: String,
    pub user_id: u64,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AcknowledgePayload {
    pub original_type: MessageType,
    pub original_id: u64,
}

pub trait Payload {
    fn to_bson(&self) -> Vec<u8>;
    fn from_bson(bson: Vec<u8>) -> Self;
}

impl Payload for AcceptPayload {
    fn to_bson(&self) -> Vec<u8> {
        bson::to_vec(self).unwrap()
    }

    fn from_bson(bson: Vec<u8>) -> AcceptPayload {
        bson::from_slice(&bson).unwrap()
    }
}

impl Payload for RejectPayload {
    fn to_bson(&self) -> Vec<u8> {
        bson::to_vec(self).unwrap()
    }

    fn from_bson(bson: Vec<u8>) -> RejectPayload {
        bson::from_slice(&bson).unwrap()
    }
}

impl Payload for ServerJoinPayload {
    fn to_bson(&self) -> Vec<u8> {
        bson::to_vec(self).unwrap()
    }

    fn from_bson(bson: Vec<u8>) -> ServerJoinPayload {
        bson::from_slice(&bson).unwrap()
    }
}

impl Payload for ChannelJoinPayload {
    fn to_bson(&self) -> Vec<u8> {
        bson::to_vec(self).unwrap()
    }

    fn from_bson(bson: Vec<u8>) -> ChannelJoinPayload {
        bson::from_slice(&bson).unwrap()
    }
}

impl Payload for ChannelLeavePayload {
    fn to_bson(&self) -> Vec<u8> {
        bson::to_vec(self).unwrap()
    }

    fn from_bson(bson: Vec<u8>) -> ChannelLeavePayload {
        bson::from_slice(&bson).unwrap()
    }
}

impl Payload for UserInfoPayload {
    fn to_bson(&self) -> Vec<u8> {
        bson::to_vec(self).unwrap()
    }

    fn from_bson(bson: Vec<u8>) -> UserInfoPayload {
        bson::from_slice(&bson).unwrap()
    }
}

impl Payload for MessageSendPayload {
    fn to_bson(&self) -> Vec<u8> {
        bson::to_vec(self).unwrap()
    }

    fn from_bson(bson: Vec<u8>) -> MessageSendPayload {
        bson::from_slice(&bson).unwrap()
    }
}

impl Payload for AcknowledgePayload {
    fn to_bson(&self) -> Vec<u8> {
        bson::to_vec(self).unwrap()
    }

    fn from_bson(bson: Vec<u8>) -> AcknowledgePayload {
        bson::from_slice(&bson).unwrap()
    }
}
