pub mod debug;
pub use debug::Debug;

pub mod algorithms;
pub use algorithms::Algorithms;

pub trait Window {
    fn draw(&mut self, target: &mut glium::Frame, ui: &imgui::Ui, window: &winit::Window);
    fn handle_events(&mut self, _window: &winit::Window, _event: &winit::Event, _io: &imgui::Io) { }
}
