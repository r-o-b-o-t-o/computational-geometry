pub mod shaders;
pub use shaders::SHADERS;

pub mod orbit_camera;
pub use orbit_camera::OrbitCamera;

pub mod axes;

use crate::math::Vec2;
use glium::{
    Display, Version, Api, Profile,
    glutin::dpi::LogicalSize, backend::Facade, vertex::VertexBufferAny,
};

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

pub fn load_obj(facade: &dyn Facade, path: &std::path::Path) -> (VertexBufferAny, f32) {
    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 3],
        normal: [f32; 3],
        color_diffuse: [f32; 3],
        color_specular: [f32; 4],
    }

    implement_vertex!(Vertex, position, normal, color_diffuse, color_specular);

    let mut min_pos = [std::f32::INFINITY; 3];
    let mut max_pos = [std::f32::NEG_INFINITY; 3];
    let mut vertex_data = Vec::new();
    match tobj::load_obj(path) {
        Ok((models, mats)) => {
            // Just upload the first object in the group
            for model in &models {
                let mesh = &model.mesh;
                for idx in &mesh.indices {
                    let i = *idx as usize;
                    let pos = [
                        mesh.positions[3 * i],
                        mesh.positions[3 * i + 1],
                        mesh.positions[3 * i + 2],
                    ];
                    let normal = if !mesh.normals.is_empty() {
                        [
                            mesh.normals[3 * i],
                            mesh.normals[3 * i + 1],
                            mesh.normals[3 * i + 2],
                        ]
                    } else {
                        [0.0, 0.0, 0.0]
                    };
                    let (color_diffuse, color_specular) = match mesh.material_id {
                        Some(i) => (
                            mats[i].diffuse,
                            [
                                mats[i].specular[0],
                                mats[i].specular[1],
                                mats[i].specular[2],
                                mats[i].shininess,
                            ],
                        ),
                        None => ([0.8, 0.8, 0.8], [0.15, 0.15, 0.15, 15.0]),
                    };
                    vertex_data.push(Vertex {
                        position: pos,
                        normal,
                        color_diffuse,
                        color_specular,
                    });
                    // Update our min/max pos so we can figure out the bounding box of the object to view it
                    for i in 0..3 {
                        min_pos[i] = f32::min(min_pos[i], pos[i]);
                        max_pos[i] = f32::max(max_pos[i], pos[i]);
                    }
                }
            }
        },
        Err(e) => panic!("Loading of {:?} failed: {:?}", path, e),
    }
    // Compute scale factor to fit the model with a [-1, 1] bounding box
    let diagonal_len = 6.0;
    let current_len = f32::powf(max_pos[0] - min_pos[0], 2.0)
                    + f32::powf(max_pos[1] - min_pos[1], 2.0)
                    + f32::powf(max_pos[2] - min_pos[2], 2.0);
    let scale = f32::sqrt(diagonal_len / current_len);
    (
        glium::vertex::VertexBuffer::new(facade, &vertex_data)
            .unwrap()
            .into_vertex_buffer_any(),
        scale,
    )
}
