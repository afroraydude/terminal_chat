use common::{channel::Channel, crypt, user::User};
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

    pub fn add_shared_key(&mut self, addr: SocketAddr, shared_key: Vec<u8>) {
        self.shared_keys.insert(addr, shared_key);
    }

    pub fn get_private_key(&self) -> Vec<u8> {
        self.private_key.clone()
    }

    pub fn get_shared_key(&self, addr: SocketAddr) -> Vec<u8> {
        self.shared_keys.get(&addr).unwrap().clone()
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
            private_key: crypt::serialize_private_key(crypt::create_private_key())
        }
    }
}
