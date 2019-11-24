use super::Window;

use crate::algorithms::*;

use glium::{
    Frame,
    backend::Facade,
};
use imgui::{ im_str, Ui, Io, Condition, ImString, ImStr };

pub trait Drawable {
    fn draw(&self, target: &mut Frame);
    fn handle_events(&mut self, _window: &winit::Window, _event: &winit::Event, _io: &Io) { }
}

pub trait Configurable {
    /// The name that will be shown in the algorithms dropdown list.
    fn name(&self) -> &'static str;
    /// Called by the window to configure the algorithm when it is selected,
    /// use the passed `ui` object to display an algorithm-specific user interface.
    fn configure(&mut self, _ui: &Ui) { }
}

trait A: Drawable + Configurable { }
impl<T> A for T where T: Drawable + Configurable { }

/// A window containing a dropdown list of all available algorithms.
/// The currently selected algorithm will be rendered to the window.
pub struct Algorithms<'f> {
    algs: Vec<Box<dyn A + 'f>>,
    selected: usize,
}

impl<'f> Algorithms<'f> {
    pub fn new(facade: &'f dyn Facade) -> Self {
        Self {
            algs: vec![
                Box::new(JarvisMarch::new(facade)),
                Box::new(GrahamScan::new(facade)),
                Box::new(Incremental2dTriangulation::new(facade)),
            ],
            selected: 0,
        }
    }
}

impl<'f> Window for Algorithms<'f> {
    fn draw(&mut self, target: &mut glium::Frame, ui: &Ui, _window: &winit::Window) {
        if self.algs.is_empty() {
            return;
        }

        // Draw the selected algorithm
        self.algs[self.selected].draw(target);

        imgui::Window::new(im_str!("Algorithms"))
                    .resizable(true)
                    .size([200.0, 100.0], Condition::FirstUseEver)
                    .position([16.0, 16.0], Condition::FirstUseEver)
                    .build(&ui, || {
                        // Create a vector containing the names of the available algorithms
                        let items = self.algs
                                            .iter()
                                            .map(|alg| ImString::from(alg.name().to_owned()))
                                            .collect::<Vec<_>>();
                        let items = items.iter()
                                            .map(|s| s.as_ref())
                                            .collect::<Vec<&ImStr>>();
                        imgui::ComboBox::new(im_str!(""))
                                    .build_simple_string(ui, &mut self.selected, &items[..]);

                        for _ in 0..3 {
                            ui.spacing();
                        }
                        self.algs[self.selected].configure(ui);
                    });
    }

    fn handle_events(&mut self, window: &winit::Window, event: &winit::Event, io: &Io) {
        self.algs[self.selected].handle_events(window, event, io);
    }
}
