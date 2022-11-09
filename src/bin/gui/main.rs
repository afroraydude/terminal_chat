use std::{io::{Write, self, BufReader}, fs::File, path};

use chat::ChatApp;
use common::user::User;

mod chat;
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

fn main() {
  let user = setup();

  let mut app = chat::ChatApp::new(user);

  eframe::run_native(
    "Chat",
    eframe::NativeOptions::default(),
    Box::new(|_ctx| Box::new(app)),
  )
}