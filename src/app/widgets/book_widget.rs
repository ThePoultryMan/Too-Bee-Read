use egui::{Color32, TextStyle};

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

        let _spacing = ui.spacing();
        let (title, title_size) = {
            let wrap_width = ui.available_width();
            let title = title.into_galley(ui, None, wrap_width, TextStyle::Body);
            let title_size = title.size();

            (Some(title), title_size)
        };
        let (author_text, author_size) = {
            let wrap_width = ui.available_width();
            let authors = author_text.into_galley(ui, None, wrap_width, TextStyle::Body);
            let author_size = authors.size();

            (Some(authors), author_size)
        };

        let mut desired_size = egui::Vec2::new(0.0, 0.0);
        desired_size.x = title_size.x.max(author_size.x) + BOOK_MARGINS * 2.0;
        desired_size.y = title_size.y + author_size.y + BOOK_MARGINS * 2.0;
        let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());

        if ui.is_rect_visible(rect) {
            let visuals = ui.style().interact(&response);

            // Surrounding Box
            ui.painter().add(egui::epaint::RectShape {
                rect: rect,
                rounding: visuals.rounding,
                fill: Color32::from_gray(37),
                stroke: visuals.bg_stroke,
            });

            // Text
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
