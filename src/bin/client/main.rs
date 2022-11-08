use std::{error::Error, io};

mod client;
extern crate common;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // ask for the server address
    let mut server_addr = String::new();
    println!("Enter the server address: ");
    io::stdin()
        .read_line(&mut server_addr)
        .expect("Failed to read line");
    let server_addr = server_addr.trim();
    // convert to SocketAddr
    let server_addr = server_addr.parse().expect("Failed to parse server address");

    // ask for the username
    let mut username = String::new();
    println!("Enter your username: ");
    io::stdin()
        .read_line(&mut username)
        .expect("Failed to read line");
    let username = username.trim().to_string();

    // connect to the server
    client::connect(server_addr, username).await;

    Ok(())
}
