use bson::serde_helpers::serialize_u32_as_timestamp;
use common::{user::User,  channel::Channel};
use rand_core::os::OsRng;
use x25519_dalek::{StaticSecret, PublicKey};

pub struct Client {
    user: User,
    channels: Vec<Channel>,
    secret: Vec<u8>
}

impl Client {
    pub fn new(user: User, secret: StaticSecret) -> Self {
        Self {
            user,
            channels: Vec::new(),
            secret: Client::serialize_secret(secret)
        }
    }

    pub fn create_secret() -> StaticSecret {
        StaticSecret::new(OsRng)
    }

    pub fn serialize_secret(secret: StaticSecret) -> Vec<u8> {
        secret.to_bytes().to_vec()
    }

    pub fn deserialize_secret(secret: Vec<u8>) -> StaticSecret {
        let mut secret_bytes = [0u8; 32];
        for (i, byte) in secret.iter().enumerate() {
            secret_bytes[i] = *byte;
        }
        StaticSecret::from(secret_bytes)
    }
}