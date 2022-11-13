use common::{user::User,  channel::Channel};
use x25519_dalek::{StaticSecret, PublicKey};
use rand_core::os::OsRng;

pub struct Client {
    user: User,
    channels: Vec<Channel>,
    secret: Vec<u8>
}

impl Client {
    pub fn new(user: User) -> Self {
        Self {
            user,
            channels: Vec::new(),
            secret: Vec::new()
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

    pub fn create_public_key(secret: StaticSecret) -> Vec<u8> {
        let public_key = PublicKey::from(&secret);
        public_key.as_bytes().to_vec()
    }
}