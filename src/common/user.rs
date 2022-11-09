use crate::id::{create_id, IdType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: u64,
    pub username: String,
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl User {
    pub fn new(username: String) -> User {
        let id = create_id(IdType::User);
        User { id, username }
    }

    pub fn create_all(id: u64, username: String) -> User {
        User { id, username }
    }

    pub fn change_username(&mut self, username: String) {
        self.username = username;
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        rmp_serde::to_vec(self).unwrap()
    }

    pub fn from_bytes(bson: Vec<u8>) -> User {
        // try to deserialize the bson, if it fails, return an unknown user
        match rmp_serde::from_slice(&bson) {
            Ok(user) => user,
            Err(_) => User::new("Unknown".to_string()),
        }
    }
}

impl Clone for User {
    fn clone(&self) -> Self {
        User::create_all(self.id, self.username.clone())
    }
}