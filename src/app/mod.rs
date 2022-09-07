pub mod book_search;
pub mod window;

pub trait PopUp {
    fn show(&mut self, ctx: &egui::Context);

    fn create_ui(&mut self, ui: &mut egui::Ui);

    fn enable(&mut self, enable: bool);

    fn is_visible(&self) -> bool;
}
