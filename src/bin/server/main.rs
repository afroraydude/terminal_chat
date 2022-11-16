extern crate common;

use std::env;
use std::error::Error;
use std::net::SocketAddr;
use std::sync::Arc;

use bytes::Bytes;
use futures::SinkExt;
use log::{debug, error, info};
use simplelog::*;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, Mutex};
use tokio_stream::StreamExt;
use tokio_util::codec::{BytesCodec, Framed};
use common::crypt;

use common::message::{Message, MessagePayload, MessageType, Payload};
use common::user::User;
use server::Server;

mod client;
mod server;

fn print_logo() {
    // load logo from file
    let logo = "Yuttari";
    println!("{}", logo);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    SimpleLogger::init(LevelFilter::Debug, Config::default()).unwrap();

    print_logo();

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

    let priv_key = crypt::deserialize_private_key(server.lock().await.get_private_key());

    let pub_key = crypt::create_public_key(priv_key.clone());

    let conn_message = Message::new(MessageType::ConnectionReceive, crypt::serialize_public_key(pub_key)).to_bytes();

    let priv_key = vec![0u8; 0];
    let pub_key = vec![0u8; 0];

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
    let login_message = Message::from_bytes(login_message.to_vec());
    if login_message.message_type != MessageType::Login {
        debug!("Client sent invalid message type");
        debug!("Expected: Login");
        return Ok(());
    }

    // create a new client
    let mut client = client::Client::new(server.clone(), bytes).await?;

    // get the user from the login message
    let user = User::from_bytes(login_message.payload);
    if user.username == "Unknown" {
        debug!("Client sent invalid username");
        return Ok(());
    } else {
        debug!("Client logged in as {}", user.username);
        let pub_key = User::deserialize_public_key(user.public_key);
        let mut state = server.lock().await;
        let private_key = crypt::deserialize_private_key(state.get_private_key());
        let shared_key = crypt::create_shared_key(private_key, pub_key);
        state.add_shared_key(addr, shared_key);
        debug!("Shared key created, {:?}", state.get_shared_key(addr));
        let message_payload = format!("{} has joined the server", user.username);
        let message_payload = MessagePayload::new("SERVER".to_string(), message_payload).to_bytes();
        let message = Message::new(MessageType::Message, message_payload);
        state.broadcast(None, message).await;
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
                        let message = Message::from_bytes(bytes.to_vec());
                        match message.message_type {
                            MessageType::Message => {
                                let mut state = server.lock().await;
                                state.broadcast(Some(addr), message).await;
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

    let mut state = server.lock().await;
    state.remove_client(addr);

    let msg = format!("User has left the chat");
    let message = Message::new(MessageType::Message, MessagePayload::new("SERVER".to_string(), msg).to_bytes());
    state.broadcast(None, message).await;


    Ok(())
}
