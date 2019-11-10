use super::Window;

use crate::algorithms::JarvisMarch;

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
    fn name(&self) -> &'static str;
    fn configure(&mut self, _ui: &Ui) { }
}

trait A: Drawable + Configurable { }
impl<T> A for T where T: Drawable + Configurable { }

pub struct Algorithms<'f> {
    algs: Vec<Box<dyn A + 'f>>,
    selected: usize,
}

impl<'f> Algorithms<'f> {
    pub fn new(facade: &'f dyn Facade) -> Self {
        Self {
            algs: vec![
                Box::new(JarvisMarch::new(facade)),
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

        self.algs[self.selected].draw(target);

        imgui::Window::new(im_str!("Algorithms"))
                    .resizable(true)
                    .size([200.0, 100.0], Condition::FirstUseEver)
                    .position([16.0, 16.0], Condition::FirstUseEver)
                    .build(&ui, || {
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
