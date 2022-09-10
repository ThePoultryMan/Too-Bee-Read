use serde::{Deserialize, Serialize};

use super::{book_search::BookSearch, widgets, PopUp};
use crate::save;

#[derive(Deserialize, Serialize, Clone)]
pub struct App {
    book_search_window: BookSearch,
    save_data: save::SaveData,
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
            save_data: save::get_save(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut dummy_self = self;
        if dummy_self.book_search_window.save_changed {
            dummy_self.save_data = save::get_save();
            dummy_self.book_search_window.save_changed = false;
        }
        let Self {
            book_search_window,
            save_data,
        } = dummy_self;

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("Add", |ui| {
                    if ui.button("By name...").clicked() {
                        book_search_window.enable(true);
                    }
                })
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if save_data.has_books() {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.horizontal_wrapped(|ui| {
                        for book_data in save_data.get_books() {
                            ui.add(widgets::BookWidget::new(book_data.get_volume_info()));
                        }
                    });
                });
            } else {
                ui.label("You have no books! Click 'Add' in the menu bar to add a book.");
            }
        });

        if book_search_window.is_visible() {
            book_search_window.show(ctx);
        }
    }
}
