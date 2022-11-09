use bytes::Bytes;
use common::message::{Message, MessageType, MessagePayload, Payload};
use common::user::User;
use log::{info, error, debug};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, Mutex};
use tokio_stream::StreamExt;
use tokio_util::codec::{Framed, BytesCodec};

use futures::SinkExt;
use std::env;
use std::error::Error;
use std::net::SocketAddr;
use std::sync::Arc;

use simplelog::*;

use server::Server;

mod client;
mod server;
extern crate common;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    SimpleLogger::init(LevelFilter::Debug, Config::default()).unwrap();

    let mut server = Server::default();
    let state = Arc::new(Mutex::new(server));

    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:1234".to_string());

    println!("Listening on: {}", addr);

    // Bind a TCP listener to the socket address.
    //
    // Note that this is the Tokio TcpListener, which is fully async.
    let listener = TcpListener::bind(&addr).await?;

    loop {
        // Asynchronously wait for an inbound TcpStream.
        let (stream, addr) = listener.accept().await?;

        // Clone a handle to the `Shared` state for the new connection.
        let state = Arc::clone(&state);

        // Spawn our handler to be run asynchronously.
        tokio::spawn(async move {
            info!("new client: {}", addr);
            match handle_connection(state, stream, addr).await {
                Err(e) => {
                    error!("failed to process connection: {}", e);
                }
                _ => (),
            }
        });
    }
}

async fn handle_connection(
    server: Arc<Mutex<Server>>,
    mut stream: TcpStream,
    addr: SocketAddr,
) -> Result<(), Box<dyn Error>> {
    let mut bytes = Framed::new(stream, BytesCodec::new());

    let conn_message = Message::new(MessageType::ConnectionReceive, vec![]).to_bson();
    bytes.send(Bytes::from(conn_message)).await?;

    // get the login message
    let login_message = match bytes.next().await {
        Some(Ok(bytes)) => bytes,
        Some(Err(e)) => {
            error!("Error: {}", e);
            return Ok(());
        }
        None => {
            info!("Client disconnected");
            return Ok(());
        }
    };

    // deserialize the login message
    let login_message = Message::from_bson(login_message.to_vec());
    if login_message.message_type != MessageType::Login {
        debug!("Client sent invalid message type");
        debug!("Expected: Login");
        return Ok(());
    }

    // create a new client
    let mut client = client::Client::new(server.clone(), bytes).await?;

    // get the user from the login message
    let user = User::from_bson(login_message.payload);
    if user.username == "Unknown" {
        debug!("Client sent invalid username");
        return Ok(());
    } else {
        debug!("Client logged in as {}", user.username);
        let mut state = server.lock().await;
        let message_payload = format!("{} has joined the server", user.username);
        let message_payload = MessagePayload::new("SERVER".to_string(), message_payload).to_bson();
        let message = Message::new(MessageType::Message, message_payload);
        state.broadcast(addr, message.to_bson()).await;
    }

    loop {
        tokio::select! {
            Some(bytes) = client.rx.recv() => {
                debug!("Sending message to client {}", client.addr().to_string());
                client.send(bytes).await?;
            }
            result = client.bytes.next() => {
                match result {
                    Some(Ok(bytes)) => {
                        let message = Message::from_bson(bytes.to_vec());
                        match message.message_type {
                            MessageType::Message => {
                                let mut state = server.lock().await;
                                state.broadcast(addr, message.to_bson()).await;
                            }
                            _ => {
                                debug!("Client sent invalid message type");
                                break;
                            }
                        }
                    }
                    Some(Err(e)) => {
                        error!("Error: {}", e);
                        break;
                    }
                    None => {
                        break;
                    }
                }
            }
        }
    }

    {
        let mut state = server.lock().await;
        state.remove_client(addr);

        let msg = format!("User has left the chat");
        let message = Message::new(MessageType::Message, MessagePayload::new("SERVER".to_string(), msg).to_bson());
        state.broadcast(addr, message.to_bson()).await;
    }


    Ok(())
}
