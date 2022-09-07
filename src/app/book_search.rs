use egui::Color32;
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
        ui.horizontal(|ui| {
            let title_text = egui::RichText::new(volume.get_title())
            .color(Color32::WHITE)
            .size(19.0);
            ui.label(title_text);
            ui.add_space(PADDING * 2.0);
            if ui.button("Add to TBR").clicked() {
                todo!();
            };
        });
        ui.add_space(PADDING);
        ui.colored_label(Color32::WHITE, "Authors:");
        ui.horizontal(|ui| {
            for author in volume.get_authors() {
                if volume
                    .get_authors()
                    .iter()
                    .position(|a| a == &author)
                    .unwrap_or(0 as usize)
                    + 1
                    != volume.get_authors().len()
                {
                    ui.label(author + ",");
                } else {
                    ui.label(author);
                }
            }
        });
        ui.separator();
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

        ui.add_space(PADDING);
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
