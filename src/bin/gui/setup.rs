use std::io::Write;
use common::user::User;

pub struct Setup {
    username: String,
    pub user: User,
}

impl Clone for Setup {
    fn clone(&self) -> Self {
        Self {
            username: self.username.clone(),
            user: self.user.clone(),
        }
    }
}

impl Setup {
    fn setup(&mut self) {
        let user = User::new(self.username.clone());

        // save the user to a file
        let mut file = std::fs::File::create("me.dat").unwrap();

        file.write_all(&user.to_bytes()).unwrap();

        self.user = user;
    }

    fn get_user(&self) -> User {
        self.user.clone()
    }
}

impl Default for Setup {
    fn default() -> Self {
        Self {
            username: String::new(),
            user: User::new("".to_string()),
        }
    }
}

impl eframe::App for Setup {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Welcome to Yuttari!");
            ui.add(egui::Label::new("Enter your username:"));
            ui.add(egui::TextEdit::singleline(&mut self.username));
            if ui.button("Done").clicked() {
                self.setup();
                // close the window
                frame.close();
            }
        });
    }
}