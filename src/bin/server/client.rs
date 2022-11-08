use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};

use common::user::User;
use tokio::net::TcpStream;
use tokio_util::codec::{BytesCodec, Framed};

use crate::server::{Server, Rx};

pub struct Client {
    pub id: u64,
    pub username: String,
    pub socket: Framed<TcpStream, BytesCodec>,
    pub rx: Rx,
    pub is_logged_in: bool,
}

impl Client {
    pub async fn new(
        server: Arc<Mutex<Server>>,
        socket: Framed<TcpStream, BytesCodec>,
        user: User,
    ) -> std::io::Result<Client> {
        let addr = socket.get_ref().peer_addr()?;

        let (tx, rx) = mpsc::unbounded_channel();

        let client = Client {
            id: user.id,
            username: user.username,
            socket,
            rx,
            is_logged_in: true,
        };

        server.lock().await.add_client(addr, tx);

        Ok(client)
    }

    pub fn get_user(&self) -> User {
        User::create_all(self.id, self.username.clone())
    }
}
