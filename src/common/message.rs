use std::io;
use std::net::TcpStream;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;

use crate::id::create_id;
use crate::id::IdType;
use serde::{Deserialize, Serialize};
use crate::crypt;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MessageType {
    Unknown,           // will always have empty payload, used when given garbage data
    Message,           // both client -> server and server -> client, used to send a message
    ConnectionReceive, // server -> client, used to send a connection message
    Login, // client -> server, used to login
    Connect, // client -> self, used to connect to the server
}

impl PartialEq for MessageType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (MessageType::Unknown, MessageType::Unknown) => true,
            (MessageType::Message, MessageType::Message) => true,
            (MessageType::ConnectionReceive, MessageType::ConnectionReceive) => true,
            (MessageType::Login, MessageType::Login) => true,
            (MessageType::Connect, MessageType::Connect) => true,
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

    pub fn to_bytes(&self) -> Vec<u8> {
        rmp_serde::to_vec(self).unwrap()
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Message {
        // try to deserialize the bson, if it fails, return an unknown message
        match rmp_serde::from_slice(&bytes) {
            Ok(message) => message,
            Err(_) => Message::new(MessageType::Unknown, vec![]),
        }
    }

    pub fn length(&self) -> usize {
        self.to_bytes().len()
    }
}

pub trait Payload {
    fn to_bytes(&self) -> Vec<u8>;
    fn from_bytes(bytes: Vec<u8>) -> Self;
    fn encrypt(&mut self, key: Vec<u8>);
    fn decrypt(&mut self, key: Vec<u8>);
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessagePayload {
    pub username: String,
    pub message: Vec<u8>,
}

impl MessagePayload {
    pub fn new(username: String, message: Vec<u8>) -> MessagePayload {
        MessagePayload {
            username,
            message,
        }
    }
}

impl Payload for MessagePayload {
    fn to_bytes(&self) -> Vec<u8> {
        rmp_serde::to_vec(self).unwrap()
    }

    fn from_bytes(bytes: Vec<u8>) -> MessagePayload {
        rmp_serde::from_slice(&bytes).unwrap_or_else(
            |_| MessagePayload::new("".to_string(), "00000000".as_bytes().to_vec())
        )
    }

    fn encrypt(&mut self, key: Vec<u8>) {
        self.message = crypt::encrypt_data(self.message.clone(), key);
    }

    fn decrypt(&mut self, key: Vec<u8>) {
        self.message = crypt::decrypt_data(self.message.clone(), key);
    }
}