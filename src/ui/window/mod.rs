pub mod debug;
pub use debug::Debug;

pub trait Window {
    fn draw(&mut self, ui: &imgui::Ui, window: &winit::Window);
}
