mod api;
mod app;
mod save;

use std::path::Path;

use app::window;

#[tokio::main]
async fn main() {
    let native_options = eframe::NativeOptions::default();
    if !Path::new("./data/save_data.json").exists() {
        save::SaveData::default().write_save();
    }

    eframe::run_native(
        "Too Bee Read",
        native_options,
        Box::new(|cc| Box::new(window::App::new(cc))),
    );
}
