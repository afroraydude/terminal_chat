use common::{
    channel::{self, Channel},
    message::{Message, MessagePayload, Payload},
    user::User,
};
use egui::Layout;
use std::sync::mpsc::{Sender, Receiver, self};
use tokio::sync::mpsc::UnboundedSender;

pub struct ChatApp {
    pub user: User,
    pub messages: Vec<Message>,
    pub next_message: String,
    pub tx: UnboundedSender<Message>,
    pub rx: mpsc::Receiver<Message>,
}

impl ChatApp {
    pub fn new(user: User, tx: UnboundedSender<Message>, rx: mpsc::Receiver<Message>) -> Self {
        Self {
            user,
            messages: Vec::new(),
            next_message: String::new(),
            tx,
            rx
        }
    }

    pub fn update(&mut self) {
        // check for new messages
        while let Ok(message) = self.rx.try_recv() {
            self.messages.push(message);
        }
    }
}

impl eframe::App for ChatApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.update();
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Chat");
            ui.separator();
            /*
            ui.label("Channels");
            ui.separator();
            // show the channels
            ui.horizontal(|ui| {
                for channel in &self.channels {
                    ui.label(channel.name.clone());
                }
            });

            ui.separator();
            */
            ui.label("Messages");
            ui.separator();
            // put input at the bottom
            ui.with_layout(Layout::bottom_up(egui::Align::TOP), |ui| {
                // show the input
                ui.horizontal(|ui| {
                    ui.text_edit_singleline(&mut self.next_message);
                    if ui.button("Send").clicked() {
                        // send the message
                        let payload = MessagePayload::new(
                            self.user.clone().username,
                            self.next_message.clone(),
                        )
                        .to_bson();
                        let message = Message::new(common::message::MessageType::Message, payload);
                        self.tx.send(message.clone()).unwrap();
                        self.messages.push(message);
                        // TODO: send message to server
                        self.next_message = String::new();
                        
                    }
                });
                // show the messages
                self.messages.reverse();
                for message in self.messages.iter() {
                    // convert payload to messagepayload
                    let payload = MessagePayload::from_bson(message.payload.clone());
                    ui.label(format!(
                        "[{}]{}: {}",
                        common::id::to_timestamp_string(message.id),
                        payload.username,
                        payload.message
                    ));
                }
                self.messages.reverse();
            });
        });
    }
}
