use bson::serde_helpers::serialize_u32_as_timestamp;
use common::{user::User, channel::Channel, crypt};
use rand_core::OsRng;
use x25519_dalek::{StaticSecret, PublicKey};

pub struct Client {
    user: User,
    channels: Vec<Channel>,
    secret: Vec<u8>,
    shared_key: Vec<u8>,
}

impl Client {
    pub fn new(user: User, secret: StaticSecret) -> Self {
        Self {
            user,
            channels: Vec::new(),
            secret: crypt::serialize_private_key(secret),
            shared_key: Vec::new(),
        }
    }

    pub fn set_shared_key(&mut self, shared_key: Vec<u8>) {
        self.shared_key = shared_key;
    }

    pub fn get_shared_key(&self) -> Vec<u8> {
        self.shared_key.clone()
    }
}