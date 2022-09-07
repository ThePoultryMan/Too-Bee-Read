pub mod book_search;
pub mod window;

pub trait PopUp {
    fn show(&mut self, ctx: &egui::Context, open: &mut bool);

    fn is_visible(&self) -> bool;
}
