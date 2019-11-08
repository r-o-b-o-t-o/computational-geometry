mod clipboard;
pub mod window;
pub mod manager;

use glium::Display;

pub fn init(display: &Display) -> manager::Manager {
    manager::Manager::new(display)
}
