use common::{
    channel::{self, Channel},
    message::{Message, MessagePayload, Payload},
    user::User,
};
use egui::Layout;

pub struct ChatApp {
    pub user: User,
    pub messages: Vec<Message>,
    pub next_message: String,
}

impl ChatApp {
    pub fn new(user: User) -> Self {
        Self {
            user,
            messages: Vec::new(),
            next_message: String::new(),
        }
    }
}

impl Default for ChatApp {
    fn default() -> Self {
        Self {
            user: User::new("Unknown".to_string()),
            messages: Vec::new(),
            next_message: String::new(),
        }
    }
}

impl eframe::App for ChatApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
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
