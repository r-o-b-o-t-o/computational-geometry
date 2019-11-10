use super::window::Window;

use glium::{ Display, Frame };
use imgui::{
    FontConfig, FontSource,
    Context as ImContext,
};
use winit::Event;
use imgui_glium_renderer::Renderer as ImRenderer;
use imgui_winit_support::{ HiDpiMode, WinitPlatform };

pub struct Manager<'a> {
    imgui: ImContext,
    platform: WinitPlatform,
    imgui_renderer: ImRenderer,
    windows: Vec<Box<dyn Window + 'a>>,
}

impl<'a> Manager<'a> {
    pub fn new(display: &Display) -> Self {
        let mut imgui = ImContext::create();
        imgui.set_ini_filename(None);
        imgui.set_log_filename(None);

        if let Some(backend) = super::clipboard::init() {
            imgui.set_clipboard_backend(Box::new(backend));
        } else {
            eprintln!("Failed to initialize clipboard");
        }

        let gl_window = display.gl_window();
        let window = gl_window.window();
        let mut platform = WinitPlatform::init(&mut imgui);
        platform.attach_window(imgui.io_mut(), &window, HiDpiMode::Rounded);

        let hidpi_factor = platform.hidpi_factor();
        let font_size = (13.0 * hidpi_factor) as f32;
        imgui.fonts().add_font(&[
            FontSource::DefaultFontData {
                config: Some(FontConfig {
                    size_pixels: font_size,
                    ..FontConfig::default()
                }),
            },
        ]);
        imgui.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;

        let imgui_renderer = ImRenderer::init(&mut imgui, display)
                                    .expect("Could not initialize ImGui renderer");

        Self {
            imgui,
            platform,
            imgui_renderer,
            windows: Vec::new(),
        }
    }

    pub fn handle_events(&mut self, window: &winit::Window, event: &Event) {
        self.platform.handle_event(self.imgui.io_mut(), window, event);

        for ui_win in self.windows.iter_mut() {
            ui_win.handle_events(window, event, self.imgui.io());
        }
    }

    pub fn draw(&mut self, window: &winit::Window, target: &mut Frame) {
        let io = self.imgui.io_mut();
        self.platform.prepare_frame(io, window).expect("Could not start ImGui frame");
        let ui = self.imgui.frame();

        for ui_win in self.windows.iter_mut() {
            ui_win.draw(target, &ui, window);
        }

        self.platform.prepare_render(&ui, &window);
        self.imgui_renderer.render(target, ui.render()).expect("Could not render ImGui");
    }

    pub fn add_window<W: 'a>(&mut self, window: W)
    where W: Window {
        self.windows.push(Box::new(window));
    }

    pub fn imgui_io(&self) -> &imgui::Io {
        self.imgui.io()
    }
}
