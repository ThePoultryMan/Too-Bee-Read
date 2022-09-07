use serde::{Deserialize, Serialize};

use super::PopUp;

#[derive(Deserialize, Serialize)]
pub struct BookSearch {
    pub enabled: bool,
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
    fn show(&mut self, ctx: &egui::Context) {
        let Self {
            enabled: _,
            search_text,
        } = self;

        egui::Window::new("Add book to your Too Bee Read List")
            .resizable(true)
            .show(ctx, |ui| {
                self.create_ui(ui);
            });
        self.enabled = true;
    }

    fn create_ui(&mut self, ui: &mut egui::Ui) {
        let search_input = ui.text_edit_singleline(&mut self.search_text);
    }

    fn enable(&mut self, enable: bool) {
        self.enabled = enable;
    }

    fn is_visible(&self) -> bool {
        self.enabled
    }
}
