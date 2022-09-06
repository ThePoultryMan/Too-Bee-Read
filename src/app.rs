use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct App {
    username: String,
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
            username: "Username".to_owned(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let Self { username } = self;

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("Home", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.close();
                    }
                })
            });
        });
    }
}
