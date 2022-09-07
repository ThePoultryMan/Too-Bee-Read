use super::{book_search::BookSearch, PopUp};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct App {
    book_search_window: BookSearch,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            book_search_window: BookSearch::default(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { book_search_window } = self;

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("Add", |ui| {
                    if ui.button("By name...").clicked() {
                        book_search_window.enable(true);
                    }
                })
            });
        });

        egui::CentralPanel::default().show(ctx, |_ui| {});

        if book_search_window.is_visible() {
            book_search_window.show(ctx);
        }
    }
}
