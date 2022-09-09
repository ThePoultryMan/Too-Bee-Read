use egui::{NumExt, TextStyle};

use crate::api;

pub struct BookWidget {
    title: egui::WidgetText,
}

impl BookWidget {
    pub fn new(volume: api::VolumeInfo) -> Self {
        BookWidget {
            title: volume.get_title().into(),
        }
    }
}

impl egui::Widget for BookWidget {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let BookWidget { title } = self;

        let spacing = ui.spacing();
        let (title, mut desired_size) = {
            let wrap_width = ui.available_width();
            let title = title.into_galley(ui, None, wrap_width, TextStyle::Body);

            let desired_size = title.size();

            (Some(title), desired_size)
        };

        desired_size = desired_size.at_least(egui::Vec2::splat(spacing.interact_size.y));
        let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());

        if ui.is_rect_visible(rect) {
            let visuals = ui.style().interact(&response);
            if let Some(title) = title {
                let text_pos = egui::pos2(rect.min.x, rect.center().y - 0.5 * title.size().y);
                title.paint_with_visuals(ui.painter(), text_pos, visuals);
            }
        }

        response
    }
}
