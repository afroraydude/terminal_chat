use bytes::Bytes;
use common::message::{Message, MessageType};
use common::user::User;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, Mutex};
use tokio_stream::StreamExt;
use tokio_util::codec::{Framed, LinesCodec};

use futures::SinkExt;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::io::{self, Read};
use std::net::SocketAddr;
use std::sync::Arc;

use server::Server;
use tokio_util::codec::*;

mod client;
mod server;
extern crate common;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut server = Server::default();
    let state = Arc::new(Mutex::new(server));

    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:1234".to_string());

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
            println!("new client: {}", addr);
            match handle_connection(state, stream, addr).await {
                Err(e) => {
                    println!("failed to process connection: {}", e);
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
            println!("Error: {}", e);
            return Ok(());
        }
        None => {
            println!("Client disconnected");
            return Ok(());
        }
    };

    // deserialize the login message
    let login_message = Message::from_bson(login_message.to_vec());
    if login_message.message_type != MessageType::Login {
        println!("Client sent invalid message type");
        return Ok(());
    }

    // get the user from the login message
    let user = User::from_bson(login_message.payload);
    if user.username == "Unknown" {
        println!("Client sent invalid username");
        return Ok(());
    }

    // create a new client
    let mut client = client::Client::new(server.clone(), bytes, user).await?;

    loop {
        tokio::select! {
            result = client.socket.next() => {
                match result {
                    Some(Ok(bytes)) => {
                        let message = Message::from_bson(bytes.to_vec());
                        match message.message_type {
                            MessageType::Message => {
                                let message = message.payload;
                                let mut state = server.lock().await;
                                state.broadcast(message).await;
                            }
                            _ => {
                                println!("Client sent invalid message type");
                                break;
                            }
                        }
                    }
                    Some(Err(e)) => {
                        println!("Error: {}", e);
                        break;
                    }
                    None => {
                        println!("Client disconnected");
                        break;
                    }
                }
            }
        }
    }
    Ok(())
}
