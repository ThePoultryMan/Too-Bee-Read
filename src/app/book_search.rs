use pollster::FutureExt;
use serde::{Deserialize, Serialize};

use crate::api;

use super::PopUp;

const PADDING: f32 = 5.0;

#[derive(Deserialize, Serialize, Clone)]
pub struct BookSearch {
    enabled: bool,
    search_text: String,
    result: Option<api::Volume>,
}

impl BookSearch {
    fn set_result(&mut self, result: api::Volume) {
        self.result = Some(result);
    }

    fn _clear_result(&mut self) {
        self.result = None;
    }

    pub fn get_result(&mut self) -> api::Volume {
        self.result.clone().unwrap()
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    fn render_book_result(&self, ui: &mut egui::Ui, volume: api::VolumeInfo) {
        ui.add_space(PADDING);
        ui.label(volume.get_title());
        ui.add_space(PADDING);
        ui.label("Authors:");
        for author in volume.get_authors() {
            ui.label(author);
        }
    }
}

impl Default for BookSearch {
    fn default() -> Self {
        Self {
            enabled: false,
            search_text: "".to_owned(),
            result: None,
        }
    }
}

impl PopUp for BookSearch {
    fn show(&mut self, ctx: &egui::Context) {
        let Self {
            enabled: _,
            search_text: _,
            result: _,
        } = self;

        egui::Window::new("Add book to your Too Bee Read List")
            .resizable(true)
            .show(ctx, |ui| {
                self.create_ui(ui);
            });
        self.enabled = true;
    }

    fn create_ui(&mut self, ui: &mut egui::Ui) {
        let _search_input = ui.text_edit_singleline(&mut self.search_text);

        if ui
            .input_mut()
            .consume_key(egui::Modifiers::NONE, egui::Key::Enter)
        {
            self.set_result(api::search_for_book(&self.search_text).block_on().unwrap());
        }

        egui::containers::ScrollArea::vertical().show(ui, |ui| {
            if self.is_visible() && self.has_result() {
                for book_result in self.get_result().get_items() {
                    self.render_book_result(ui, book_result.get_volume_info());
                }
            }
        });
    }

    fn enable(&mut self, enable: bool) {
        self.enabled = enable;
    }

    fn is_visible(&self) -> bool {
        self.enabled
    }
}
