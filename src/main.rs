mod api;
mod app;

use app::window;

#[tokio::main]
async fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Too Bee Read",
        native_options,
        Box::new(|cc| Box::new(window::App::new(cc))),
    );
}
