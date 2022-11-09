use crate::message::{Message, MessageType};

pub struct Channel {
    pub name: String,
    pub users: Vec<u64>,
    pub messages: Vec<Message>,
    pub max_messages: usize,
    pub backup_messages: bool,
}

impl Channel {
    pub fn new(name: String) -> Channel {
        Channel {
            name,
            users: Vec::new(),
            messages: Vec::new(),
            max_messages: 100,
            backup_messages: true,
        }
    }

    pub fn add_user(&mut self, user: u64) {
        self.users.push(user);
    }

    pub fn remove_user(&mut self, user: u64) {
        self.users.retain(|&x| x != user);
    }

    pub fn add_message(&mut self, message: Message) {
        if message.message_type != MessageType::Message {
            return;
        }

        // if we have 100 messages, remove and backup the first one
        if self.messages.len() == 100 {
            let message = self.messages.remove(0);
            self.save_message(message);
        }

        if self.messages.len() == 100 {
            // remove the oldest message
            let last_message = self.messages.remove(0);
            if self.backup_messages {
                self.save_message(last_message);
            }
        }

        self.messages.push(message);
    }

    pub fn refresh_messages(&mut self) {
        self.messages = Vec::new();
    }

    pub fn backup(&mut self) {
        // save messages to file using bson
        let data = bson::to_vec(&self.messages).unwrap();

        // save to file
        std::fs::write(format!("data/channels/{}.bson", self.name), data).unwrap();

        // clear messages for memory management
        self.refresh_messages();
    }

    pub fn save_message(&mut self, message: Message) {
        // load messages from file using bson
        let data = std::fs::read(format!("data/channels/{}.bson", self.name)).unwrap();
        let mut messages: Vec<Message> = bson::from_slice(&data).unwrap();

        // add message to messages
        messages.push(message);

        // save messages to file using bson
        let data = bson::to_vec(&messages).unwrap();

        // save to file
        std::fs::write(format!("data/channels/{}.bson", self.name), data).unwrap();
    }
}

pub fn get_default_channels() -> Vec<Channel> {
    let mut channels = Vec::new();

    channels.push(Channel::new("general".to_string()));
    channels.push(Channel::new("random".to_string()));

    channels
}
