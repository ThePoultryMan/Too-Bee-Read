use egui::{Color32, FontId};

pub fn new(ui: &mut egui::Ui, title: &str) -> egui::Response {
    let text_width = ui.painter().layout_no_wrap(title.to_owned(), FontId::default(), Color32::WHITE).rect.size().x;
    let desired_size = ui.spacing().interact_size.y * egui::vec2(text_width / 2.0, 7.0);

    let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());

    
        ui.painter().text(
            rect.left_top(),
            egui::Align2::LEFT_TOP,
            title,
            FontId::default(),
            Color32::WHITE,
        );
    

    response
}
