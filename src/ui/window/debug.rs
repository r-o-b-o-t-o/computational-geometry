use super::Window;

use std::time::{ Instant, Duration };

use imgui::{ im_str, Ui, Condition };

pub struct Debug {
    current_time: Instant,
    last_time: Instant,
    last_framerate_measure: Instant,
    delta_time: Duration,
    frames_count: u32,
    framerate: u32,
}

impl Debug {
    fn update_framerate(&mut self) {
        const FRAMERATE_UPDATE_STEP: Duration = Duration::from_millis(100);

        self.current_time = Instant::now();
        self.delta_time = self.current_time - self.last_time;
        self.last_time = self.current_time;
        self.frames_count += 1;

        if self.current_time - self.last_framerate_measure >= FRAMERATE_UPDATE_STEP {
            self.framerate = (self.frames_count as f32 / (self.current_time - self.last_framerate_measure).as_secs_f32()).round() as u32;
            self.frames_count = 0;
            self.last_framerate_measure = self.current_time;
        }
    }
}

impl Default for Debug {
    fn default() -> Self {
        let now = Instant::now();

        Self {
            current_time: now,
            last_time: now,
            last_framerate_measure: now,
            delta_time: Duration::default(),
            frames_count: 0,
            framerate: 0,
        }
    }
}

impl Window for Debug {
    fn draw(&mut self, ui: &Ui, window: &winit::Window) {
        let win_size = match window.get_inner_size() {
            Some(dim) => dim,
            None => return,
        };

        self.update_framerate();

        imgui::Window::new(im_str!("Debug"))
                    .resizable(false)
                    .always_auto_resize(true)
                    .position([win_size.width as f32 - 32.0, 16.0], Condition::FirstUseEver)
                    .position_pivot([1.0, 0.0])
                    .build(&ui, || {
                        ui.text(im_str!("Framerate: {} fps", self.framerate));
                    });
    }
}
