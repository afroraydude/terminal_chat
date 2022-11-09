use bytes::Bytes;
use common::{user::User,  channel::Channel};

pub struct Client {
    user: User,
    channels: Vec<Channel>,
}

impl Client {
    pub fn new(user: User) -> Self {
        Self {
            user,
            channels: Vec::new()
        }
    }
}