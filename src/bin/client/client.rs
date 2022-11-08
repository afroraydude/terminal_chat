use bytes::Bytes;
use common::user::User;
use futures::{future, Sink, SinkExt, Stream, StreamExt};
use std::{error::Error, io, net::SocketAddr};
use tokio::net::TcpStream;
use tokio_util::codec::{BytesCodec, FramedRead, FramedWrite};

struct Client {
    user: User,
    channels: Vec<String>,
}

pub async fn connect(addr: SocketAddr, username: String) -> Result<(), Box<dyn Error>> {
    Ok(())
}
