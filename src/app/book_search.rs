use serde::{Deserialize, Serialize};

use super::PopUp;

#[derive(Deserialize, Serialize)]
pub struct BookSearch {
    enabled: bool,
    search_text: String,
}

impl Default for BookSearch {
    fn default() -> Self {
        Self {
            enabled: false,
            search_text: "".to_owned(),
        }
    }
}

impl PopUp for BookSearch {
    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        egui::Window::new("Add book to your Too Bee Read List")
            .open(open)
            .resizable(true)
            .show(ctx, |ui| {
                ui.add(egui::TextEdit::singleline(&mut self.search_text).hint_text("Enter a book title..."));
            });
        self.enabled = true;
    }

    fn is_visible(&self) -> bool {
        self.enabled
    }
}
