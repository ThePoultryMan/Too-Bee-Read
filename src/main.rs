mod api;
mod app;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Too Bee Read",
        native_options,
        Box::new(|cc| Box::new(app::App::new(cc))),
    );
}
