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
                // show the messages
                for message in self.messages.iter() {
                    // convert payload to messagepayload
                    let payload = MessagePayload::from_bytes(message.payload.clone());
                    ui.label(format!(
                        "[{}]{}: {}",
                        common::id::to_formatted_timestamp(message.id, "%H:%M:%S"),
                        payload.username,
                        payload.message
                    ));
                }
            });
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
                        .to_bytes();
                        let message = Message::new(common::message::MessageType::Message, payload);
                        self.tx.send(message.clone()).unwrap();
                        self.messages.push(message);
                        // TODO: send message to server
                        self.next_message = String::new();
                        
                    }
                });

            });
        });
    }
}
