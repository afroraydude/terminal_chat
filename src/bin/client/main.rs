use std::{error::Error, io::{self, Write, BufReader}, net::SocketAddr, fs::File, path, thread};

use client::Client;
use common::{user::User, message::{self, MessagePayload, Payload}};
use futures::{channel::mpsc, future::InspectOk, StreamExt};
use log::debug;
use tokio_util::codec::{Framed, BytesCodec, FramedWrite, FramedRead};
use bytes::{Bytes, BytesMut};
use common::message::{Message, MessageType};
use tokio::net::{TcpListener, TcpStream};

use futures::SinkExt;
use std::env;
use std::sync::Arc;

use simplelog::*;

mod client;
extern crate common;

fn setup() -> User {
    // if file exists, read from file
    if path::Path::new("me.dat").exists() {
        let file = File::open("me.dat").unwrap();
        let reader = BufReader::new(file);
        // convert from bson to user
        let user: User = bson::from_reader(reader).unwrap();
        return user;
    }

    // else, create new user

    // ask for the username
    let mut username = String::new();
    println!("Enter your username: ");
    io::stdin()
        .read_line(&mut username)
        .expect("Failed to read line");
    let username = username.trim().to_string();

    let user = User::new(username);

    // save the user to a file
    let mut file = std::fs::File::create("me.dat").unwrap();

    file.write_all(&user.to_bson()).unwrap();

    user
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    SimpleLogger::init(LevelFilter::Debug, Config::default()).unwrap();

    // get the user either from a file or from the user
    let user = setup();

    let mut client = Client::new(user.clone());

    let stdin = FramedRead::new(tokio::io::stdin(), BytesCodec::new());
    let mut stdin = stdin.map(|i| i.map(BytesMut::freeze));
    let mut stdout = FramedWrite::new(tokio::io::stdout(), BytesCodec::new());

    let mut stream = TcpStream::connect("127.0.0.1:1234").await?;
    let (reader, writer) = stream.split();
    let mut sink = FramedWrite::new(writer, BytesCodec::new());
    let mut stream = FramedRead::new(reader, BytesCodec::new());

    loop {
        tokio::select! {
            msg = stream.next() => {
                if let Some(Ok(msg)) = msg {
                    let raw: Vec<u8> = msg.to_vec();

                    let message = Message::from_bson(raw.clone());

                    match message.message_type {
                        MessageType::Message => {
                            let message_content = String::from_utf8(message.payload).unwrap();

                            // get message payload
                            let message_payload: MessagePayload = MessagePayload::from_bson(message_content.as_bytes().to_vec());

                            // get the message
                            println!("{}: {}", message_payload.username, message_payload.message);
                        },
                        MessageType::ConnectionReceive => {
                            let login_message = Message::new(MessageType::Login, user.clone().to_bson());
                            sink.send(Bytes::from(login_message.to_bson())).await?;
                        },
                        MessageType::Unknown => {
                            debug!("Received unknown message type");
                        },
                        _ => {
                            debug!("received a message of type {:?}", message.message_type);
                        }
                    }
                } else {
                    break
                }
            },
            input = stdin.next() => {
                if let Some(Ok(input)) = input {
                    let input = String::from_utf8(input.to_vec()).unwrap();
                    // remove the newline
                    let input = input.trim().to_string();
                    
                    let payload = MessagePayload::new(user.clone().username, input);

                    let message = Message::new(MessageType::Message, payload.to_bson());
                    let message = message.to_bson();
                    let message = Bytes::from(message);

                    sink.send(message).await?;
                } else {
                    break;
                }
            }
            // on close
            else => {
                break;
            }
        }
    }

    Ok(())
}