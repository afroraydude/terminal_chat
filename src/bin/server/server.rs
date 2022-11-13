use common::{channel::Channel, user::User};
use log::{debug, error};
use x25519_dalek::{PublicKey, StaticSecret};
use std::{collections::HashMap, net::SocketAddr};
use tokio::sync::mpsc;

use crate::client::Client;

pub type Tx = mpsc::UnboundedSender<Vec<u8>>;
pub type Rx = mpsc::UnboundedReceiver<Vec<u8>>;

pub struct Server {
    channels: HashMap<String, Channel>,
    clients: HashMap<SocketAddr, Tx>,
    shared_keys: HashMap<SocketAddr, Vec<u8>>,
    private_key: Vec<u8>,
}

impl Server {
    pub fn add_channel(&mut self, channel: Channel) {
        self.channels.insert(channel.name.clone(), channel);
    }

    pub fn remove_channel(&mut self, channel: Channel) {
        self.channels.remove(&channel.name);
    }

    pub fn add_client(&mut self, addr: SocketAddr, tx: Tx) {
        self.clients.insert(addr, tx);
    }

    pub fn remove_client(&mut self, addr: SocketAddr) {
        self.clients.remove(&addr);
    }

    pub async fn broadcast(&mut self, sender: SocketAddr, msg: Vec<u8>) {
        for (addr, tx) in self.clients.iter_mut() {
            if *addr == sender {
                continue;
            }
            debug!("Sending message to client {}", addr.to_string());

            if let Err(e) = tx.send(msg.clone().into()) {
                error!("Error sending message to client {}: {}", addr.to_string(), e);
            }
        }
    }

    pub fn create_private_key() -> Vec<u8> {
        let secret_key = StaticSecret::new(rand_core::os::OsRng);
        secret_key.to_bytes().to_vec()
    }

    pub fn create_shared_key(&mut self, addr: SocketAddr, pub_key: PublicKey) {
        let mut key_bytes = [0u8; 32];
        key_bytes.copy_from_slice(pub_key.as_bytes());
        let private_key = x25519_dalek::StaticSecret::from(key_bytes);
        let shared_key = private_key.diffie_hellman(&pub_key).as_bytes().to_vec();
        self.shared_keys.insert(addr, shared_key);
    }
}

impl Default for Server {
    fn default() -> Server {
        let mut default_channels: HashMap<String, Channel> = HashMap::new();

        let default_channel: Channel = Channel::new("general".to_string());
        let default_channel2: Channel = Channel::new("random".to_string());

        default_channels.insert("general".to_string(), default_channel);
        default_channels.insert("random".to_string(), default_channel2);

        Server {
            channels: default_channels,
            clients: HashMap::new(),
            shared_keys: HashMap::new(),
            private_key: Server::create_private_key(),
        }
    }
}
