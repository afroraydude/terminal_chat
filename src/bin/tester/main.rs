extern crate common;

use std::io::Write;
use common::message::{Message, MessageType};
use log::{debug, LevelFilter};
use simplelog::{SimpleLogger, Config};

fn main() {
    SimpleLogger::init(LevelFilter::Debug, Config::default()).unwrap();
    // loop through input
    loop {
        // get input
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input = input.trim().parse().unwrap();
        // create a message from the input
        let message = Message::new(MessageType::Message, input.as_bytes().to_vec());

        // print debug "Message sent. Message ID: {id}, Message Type: {type}, Message Length: {length}"
        debug!("Message sent. Message ID: {}, Message Type: {:?}, Message Length: {}", message.id, message.message_type, message.length());

        // test the id by getting its timestamp
        debug!("Message timestamp: {}", common::id::to_timestamp(message.id));
        // convert timestamp (unix epoch) to human readable
        debug!("Message timestamp (human readable): {}", common::id::to_timestamp_string(message.id));
    }
}