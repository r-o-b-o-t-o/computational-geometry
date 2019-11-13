pub mod shaders;
pub use shaders::SHADERS;

use crate::math::Vec2;
use glium::glutin::dpi::LogicalSize;
use glium::{ Display, Version, Api, Profile };

pub fn print_api_info(display: &Display) {
    let version = *display.get_opengl_version();
    let api = match version {
        Version(Api::Gl, _, _) => "OpenGL",
        Version(Api::GlEs, _, _) => "OpenGL ES"
    };
    println!("{} context version: {}", api, display.get_opengl_version_string());

    print!("{} context flags:", api);
    if display.is_forward_compatible() {
        print!(" forward-compatible");
    }
    if display.is_debug() {
        print!(" debug");
    }
    if display.is_robust() {
        print!(" robustness");
    }
    println!();

    if version >= Version(Api::Gl, 3, 2) {
        let profile_mask = match display.get_opengl_profile() {
            Some(Profile::Core) => "core",
            Some(Profile::Compatibility) => "compatibility",
            None => "unknown",
        };
        println!("{} profile mask: {}", api, profile_mask);
    }

    let robustness = if display.is_context_loss_possible() {
        "lose"
    } else {
        "none"
    };
    println!("{} robustness strategy: {}", api, robustness);

    println!("{} context renderer: {}", api, display.get_opengl_renderer_string());
    println!("{} context vendor: {}", api, display.get_opengl_vendor_string());
}

/// Takes a position in window space coordinates and converts it to coordinates clamped between -1.0 and 1.0.
pub fn window_pos_to_normalized(mut pos: Vec2, window: &winit::Window) -> Vec2 {
    let size = window.get_inner_size().unwrap_or_else(|| LogicalSize::new(1.0, 1.0));
    let size = Vec2::new(size.width as f32, size.height as f32);
    pos.y = size.y - pos.y;
    let half = &size / 2.0;
    &(&pos - &half) / &half
}
