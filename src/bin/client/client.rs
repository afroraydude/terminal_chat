use bytes::Bytes;
use common::{user::User, message::Message};
use futures::{future, Sink, SinkExt, Stream, StreamExt, channel::mpsc::Sender};
use std::{error::Error, io, net::SocketAddr};
use tokio::net::TcpStream;
use tokio_util::codec::{BytesCodec, FramedRead, FramedWrite};

pub struct Client {
    user: User,
    channels: Vec<String>,
}

impl Client {
    pub fn new(user: User) -> Self {
        Self {
            user,
            channels: Vec::new()
        }
    }
}