extern crate common;

use std::{
    env,
    error::Error,
    fs::File,
    io::{self, BufReader, Write},
    net::SocketAddr,
    path, sync::mpsc,
};

use bytes::Bytes;
use futures::SinkExt;
use log::{debug, LevelFilter};
use simplelog::{Config, SimpleLogger};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver};
use tokio_stream::StreamExt;
use tokio_util::codec::{BytesCodec, Framed, FramedRead, FramedWrite};

use chat::ChatApp;
use common::{id, message::{MessagePayload, MessageType, Payload}, message::Message, user::User};

mod chat;
mod setup;

fn setup() -> User {
    // if file exists, read from file
    if path::Path::new("me.dat").exists() {
        let file = File::open("me.dat").unwrap();
        let reader = BufReader::new(file);
        let user: User = rmp_serde::from_read(reader).unwrap();
        return user;
    }

    // else, create new user
    let app = setup::Setup::default();
    eframe::run_native(
        "Setup",
        eframe::NativeOptions {
            initial_window_size: Some(egui::Vec2::new(300.0, 100.0)),
            ..Default::default()
        },
        Box::new(|_ctx| Box::new(app)),
    );

    // load the user from the file
    let file = File::open("me.dat").unwrap();
    let reader = BufReader::new(file);
    let user: User = rmp_serde::from_read(reader).unwrap_or_else(
        |e| panic!("Failed to load user from file: {}. Check your permissions maybe?", e),
    );

    user
}

#[tokio::main]
async fn main() {
    SimpleLogger::init(LevelFilter::Debug, Config::default()).unwrap();


    let user = setup();

    let (tx, rx) = unbounded_channel();

    let (tx2, rx2) = mpsc::channel();

    let mut app = chat::ChatApp::new(user.clone(), tx, rx2);

    app.create_secret(); // create the secret key for the user

    // spawn the connect task
    tokio::spawn(async move {
        match connect(user, rx, tx2).await {
            Ok(_) => {
                println!("Disconnected");
                // exit the program
                std::process::exit(0);
            },
            Err(e) => {
                println!("Error: {}", e);
                // exit the program
                std::process::exit(1);
            },
        }
    });

    eframe::run_native(
        "Chat",
        eframe::NativeOptions::default(),
        Box::new(|_ctx| Box::new(app)),
    );
}

async fn connect(
    user: User,
    mut to_server_rx: UnboundedReceiver<Message>,
    tx: mpsc::Sender<Message>,
) -> Result<(), Box<dyn Error>> {

    // wait for the user to enter the server address from the gui
    let raw_bytes = to_server_rx.recv().await.unwrap().payload;
    // convert the bytes to a string
    let addr = String::from_utf8(raw_bytes.to_vec()).unwrap();
    // convert the string to a SocketAddr
    let addr = addr.parse::<SocketAddr>().unwrap();

    let mut stream = TcpStream::connect(addr).await?;
    let (reader, writer) = stream.split();
    let mut sink = FramedWrite::new(writer, BytesCodec::new());
    let mut stream = FramedRead::new(reader, BytesCodec::new());

    loop {
        tokio::select! {
        msg = stream.next() => {
          match msg {
            Some(Ok(bytes)) => {
              debug!("Received bytes: {:?}", bytes.len());
              // convert from bytes to message
              let message = Message::from_bytes(bytes.to_vec());
              // send the message to the rx channel
              match message.message_type {
                MessageType::Message => {
                  debug!("Received message: {:?}", message);
                  tx.send(message).unwrap();
                },
                MessageType::ConnectionReceive => {
                  // send login message
                  let login_message = Message::new(MessageType::Login, user.clone().to_bytes());
                  sink.send(Bytes::from(login_message.to_bytes())).await?;
                },
                _ => {
                  log::error!("Invalid message type");
                }
              }
            }
            Some(Err(e)) => {
              log::error!("Error: {}", e);
              break;
            }
            None => {
              log::error!("Connection closed");
              break;
            }
          }
        }
        msg = to_server_rx.recv() => {
            match msg {
                Some(message) => {
                debug!("Sending message: {:?}", message);
                sink.send(Bytes::from(message.to_bytes())).await?;
                }
                None => {
                log::error!("Connection closed");
                break;
                }
            }
        }
      }
    }

    Ok(())
}
