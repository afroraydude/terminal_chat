use std::{sync::Arc, net::SocketAddr};
use bytes::Bytes;
use futures::SinkExt;
use log::debug;
use tokio::sync::{mpsc, Mutex};

use common::user::User;
use tokio::net::TcpStream;
use tokio_util::codec::{BytesCodec, Framed};
use x25519_dalek::{PublicKey, StaticSecret};

use crate::server::{Server, Rx};

pub struct Client {
    pub bytes: Framed<TcpStream, BytesCodec>,
    pub rx: Rx,
    pub shared_key: Vec<u8>,
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
            shared_key: Vec::new(),
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

    pub fn create_shared_key(&mut self, secret_key: StaticSecret, public_key: PublicKey) {
        self.shared_key = secret_key.diffie_hellman(&public_key).as_bytes().to_vec();
    }
}
