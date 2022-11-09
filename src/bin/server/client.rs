use std::{sync::Arc, net::SocketAddr};
use bytes::Bytes;
use futures::SinkExt;
use log::debug;
use tokio::sync::{mpsc, Mutex};

use common::user::User;
use tokio::net::TcpStream;
use tokio_util::codec::{BytesCodec, Framed};

use crate::server::{Server, Rx};

pub struct Client {
    pub bytes: Framed<TcpStream, BytesCodec>,
    pub rx: Rx,
}

impl Client {
    pub async fn new(
        server: Arc<Mutex<Server>>,
        bytes: Framed<TcpStream, BytesCodec>,
    ) -> std::io::Result<Client> {
        let addr = bytes.get_ref().peer_addr()?;

        let (tx, rx) = mpsc::unbounded_channel();

        let client = Client {
            bytes,
            rx,
        };

        server.lock().await.add_client(addr, tx);

        Ok(client)
    }

    pub fn addr(&self) -> SocketAddr {
        self.bytes.get_ref().peer_addr().unwrap()
    }

    pub async fn send(&mut self, msg: Vec<u8>) -> std::io::Result<()> {
        let test = self.bytes.send(Bytes::from(msg)).await;
        debug!("Sending message to client {}", self.addr().to_string());
        if let Err(e) = test {
            debug!("Error sending message to client {}: {}", self.addr().to_string(), e);
        }

        Ok(())
    }
}
