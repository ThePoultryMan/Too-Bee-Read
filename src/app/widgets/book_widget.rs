use egui::{NumExt, TextStyle};

use crate::api;

const BOOK_MARGINS: f32 = 7.5;

pub struct BookWidget {
    title: egui::WidgetText,
    author_text: egui::WidgetText,
}

impl BookWidget {
    pub fn new(volume: api::VolumeInfo) -> Self {
        BookWidget {
            title: volume.get_title().into(),
            author_text: {
                let mut author_text = String::new();
                for author in volume.get_authors() {
                    author_text = author_text + &author + ", ";
                }
                for _ in 0..2 {
                    author_text.remove(author_text.len() - 1);
                }

                author_text.into()
            },
        }
    }
}

impl egui::Widget for BookWidget {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let BookWidget { title, author_text } = self;

        let spacing = ui.spacing();
        let (title, mut desired_size) = {
            let wrap_width = ui.available_width();
            let title = title.into_galley(ui, None, wrap_width, TextStyle::Body);

            let desired_size = title.size();

            (Some(title), desired_size)
        };
        let author_text = {
            let wrap_width = ui.available_width();
            let authors = author_text.into_galley(ui, None, wrap_width, TextStyle::Body);

            Some(authors)
        };

        desired_size = desired_size.at_least(egui::Vec2::splat(spacing.interact_size.y));
        desired_size += egui::Vec2::splat(BOOK_MARGINS);
        let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());

        if ui.is_rect_visible(rect) {
            let visuals = ui.style().interact(&response);
            if let Some(title) = title {
                let title_pos = egui::pos2(
                    rect.left_top().x + BOOK_MARGINS,
                    rect.left_top().y + BOOK_MARGINS,
                );
                title
                    .clone()
                    .paint_with_visuals(ui.painter(), title_pos, visuals);

                if let Some(author_text) = author_text {
                    let author_pos = egui::pos2(
                        rect.left_top().x + BOOK_MARGINS,
                        rect.left_top().y + BOOK_MARGINS + title.size().y,
                    );
                    author_text.paint_with_visuals(ui.painter(), author_pos, visuals);
                }
            }
        }

        response
    }
}
