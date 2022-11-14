use common::{
    channel::{self, Channel},
    message::{Message, MessagePayload, Payload},
    user::User,
};
use egui::Layout;
use rand_core::OsRng;
use x25519_dalek::{StaticSecret, PublicKey};
use std::sync::mpsc::{Sender, Receiver, self};
use tokio::sync::mpsc::UnboundedSender;

pub struct ChatApp {
    pub user: User,
    pub messages: Vec<Message>,
    pub next_message: String,
    pub tx: UnboundedSender<Message>,
    pub rx: mpsc::Receiver<Message>,
    secret: Vec<u8>,
    shared_key: Vec<u8>,
    setup: bool,
}

impl ChatApp {
    pub fn new(user: User, tx: UnboundedSender<Message>, rx: mpsc::Receiver<Message>) -> Self {
        Self {
            user,
            messages: Vec::new(),
            next_message: String::new(),
            tx,
            rx,
            secret: Vec::new(),
            shared_key: Vec::new(),
            setup: false,
        }
    }

    pub fn update(&mut self) {
        // check for new messages
        while let Ok(message) = self.rx.try_recv() {
            self.messages.push(message);
        }
    }

    pub fn update_main_app(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.update();
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Chat");
                ui.end_row();
                ui.label("Username: ");
                ui.label(self.user.username.clone());
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Messages");
            ui.separator();
            // put input at the bottom
            egui::containers::ScrollArea::vertical().show(ui, |ui| {
                self.messages.reverse();
                ui.with_layout(Layout::top_down_justified(egui::Align::TOP), |ui| {
                    // show the messages
                    for message in self.messages.iter() {
                        // convert payload to messagepayload
                        let payload = MessagePayload::from_bytes(message.payload.clone());
                        ui.label(format!(
                            "[{}] {}: {}",
                            common::id::to_formatted_timestamp(message.id, "%H:%M:%S"),
                            payload.username,
                            payload.message
                        ));
                    }

                    ui.end_row();
                });
                self.messages.reverse();
                // add spacing
                ui.label("");
            });
        });
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.next_message);
                if ui.button("Send").clicked() {
                    // send the message
                    let payload = MessagePayload::new(
                        self.user.clone().username,
                        self.next_message.clone(),
                    )
                        .to_bytes();
                    let message = Message::new(common::message::MessageType::Message, payload);
                    self.tx.send(message.clone()).unwrap();
                    self.messages.push(message);
                    // TODO: send message to server
                    self.next_message = String::new();

                }
            });
            ui.horizontal(|ui| {
                ui.label("Status");
                ui.end_row();
                ui.label("Connected");
            });
        });
    }

    pub fn set_secret(&mut self, secret: Vec<u8>) {
        self.secret = secret;
    }

    pub fn set_shared_key(&mut self, shared_key: Vec<u8>) {
        self.shared_key = shared_key;
    }

    pub fn get_shared_key(&self) -> Vec<u8> {
        self.shared_key.clone()
    }
}

impl eframe::App for ChatApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if self.setup {
            self.update_main_app(ctx, frame);
        } else {
            // ask for socket address
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.heading("Welcome to Chat!");
                ui.add(egui::Label::new("Enter the server to connect to:"));
                ui.add(egui::TextEdit::singleline(&mut self.next_message));
                if ui.button("Done").clicked() {
                    self.setup = true;
                    // send message to tokio thread
                    let message = Message::new(
                        common::message::MessageType::Connect,
                        self.next_message.clone().into_bytes(),
                    );
                    self.tx.send(message).unwrap();
                    self.next_message = String::new();
                }
            });
        }
    }
}
