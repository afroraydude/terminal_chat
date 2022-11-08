use std::{error::Error, io::{self, Write, BufReader}, net::SocketAddr, fs::File, path};

use common::user::User;

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
    // get the user either from a file or from the user
    let user = setup();

    
    // ask for the server address
    let mut server_addr = String::new();
    println!("Enter the server address: ");
    io::stdin()
        .read_line(&mut server_addr)
        .expect("Failed to read line");
    let server_addr = server_addr.trim();
    // convert to SocketAddr
    let server_addr: SocketAddr = server_addr.parse().expect("Failed to parse server address");

    let (tx, rx) = mpsc::channel(1);

    Ok(())
}
