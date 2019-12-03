#[macro_use]
extern crate glium;

pub mod ui;
pub mod math;
pub mod tests;
pub mod graphics;
pub mod algorithms;

use glium::{
    Surface, Display,
    glutin::{
        EventsLoop, WindowBuilder, ContextBuilder, Event, WindowEvent, GlProfile,
        dpi::LogicalSize,
    },
};

fn main() {
    let mut events_loop = EventsLoop::new();
    let wb = WindowBuilder::new()
                        .with_title("Computational Geometry")
                        .with_dimensions(LogicalSize::new(1024.0, 768.0));
    let cb = ContextBuilder::new()
                        .with_gl_profile(GlProfile::Core)
                        .with_vsync(false)
                        .with_double_buffer(Some(true))
                        .with_hardware_acceleration(Some(true));
    let display = Display::new(wb, cb, &events_loop).expect("Could not create glium display");
    graphics::print_api_info(&display);

    // Create Dear ImGui windows
    let mut ui_mngr = ui::init(&display);
    ui_mngr.add_window(ui::window::Debug::default());
    ui_mngr.add_window(ui::window::Algorithms::new(&display));

    let gl_window = display.gl_window();
    let window = gl_window.window();

    let mut run = true;
    while run {
        events_loop.poll_events(|event| {
            ui_mngr.handle_events(window, &event);

            let io = ui_mngr.imgui_io();
            if let Event::WindowEvent { event, .. } = event {
                match &event {
                    WindowEvent::CloseRequested => run = false,
                    WindowEvent::KeyboardInput { input, .. } => {
                        if !io.want_capture_keyboard {
                            if let Some(key) = input.virtual_keycode {
                                if key == winit::VirtualKeyCode::Escape {
                                    run = false;
                                }
                            }
                        }
                    },
                    _ => {},
                }
            }
        });

        let mut target = display.draw();
        target.clear_color_srgb_and_depth((0.12, 0.12, 0.12, 1.0), 1.0);
        ui_mngr.draw(&window, &mut target);
        target.finish().expect("Could not swap buffers");
    }
}
