use common::{channel::Channel, user::User};
use std::{collections::HashMap, net::SocketAddr};
use tokio::sync::mpsc;

use crate::client::Client;

pub type Tx = mpsc::UnboundedSender<Vec<u8>>;
pub type Rx = mpsc::UnboundedReceiver<Vec<u8>>;

pub struct Server {
    channels: HashMap<String, Channel>,
    users: Vec<User>,
    clients: HashMap<SocketAddr, Tx>,
}

impl Server {
    pub fn add_channel(&mut self, channel: Channel) {
        self.channels.insert(channel.name.clone(), channel);
    }

    pub fn remove_channel(&mut self, channel: Channel) {
        self.channels.remove(&channel.name);
    }

    pub fn add_user(&mut self, user: User) {
        self.users.push(user);
    }

    pub fn remove_user(&mut self, user: User) {
        self.users.retain(|u| u.id != user.id);
    }

    pub fn add_client(&mut self, addr: SocketAddr, tx: Tx) {
        self.clients.insert(addr, tx);
    }

    pub fn remove_client(&mut self, addr: SocketAddr) {
        self.clients.remove(&addr);
    }

    pub async fn broadcast(&mut self, msg: Vec<u8>) {
        for tx in self.clients.values() {
            let _ = tx.send(msg.clone());
        }
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
            users: Vec::new(),
            clients: HashMap::new(),
        }
    }
}
