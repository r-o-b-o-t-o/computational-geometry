use crate::{
    graphics::OrbitCamera, ui::window::algorithms::Drawable,
    math::{ ToArray, Vec3 },
};

use glium::{
    Surface, Frame, Program,
    backend::Facade, vertex::VertexBufferAny,
};
use cgmath::SquareMatrix;

pub struct Axes {
    buffer: VertexBufferAny,
    program: Program,
    cam: OrbitCamera,
}

impl Axes {
    pub fn new(facade: &dyn Facade) -> Self {
        let vs = super::SHADERS.obj_vs;
        let fs = super::SHADERS.obj_fs;
        let program = Program::from_source(facade, vs, fs, None)
                                    .expect("Could not compile shaders");

        let (vertex_buffer, _scale) = super::load_obj(facade, &std::path::PathBuf::from("models/axes.obj"));

        let mut cam = OrbitCamera::new(6.5, Vec3::new(0.0, 0.0, 0.0), 45.0, 1.0, 0.1, 50.0);
        cam.zoom = false;

        Self {
            buffer: vertex_buffer,
            program,
            cam,
        }
    }
}

impl Drawable for Axes {
    fn draw(&self, target: &mut Frame) {
        let uniforms = uniform! {
            model: cgmath::Matrix4::<f32>::identity().to_array(),
            projection: self.cam.get_projection_matrix().to_array(),
            view: self.cam.get_view_matrix().to_array(),
        };

        let width = 128;
        let height = width;
        let offset = 32;

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            viewport: Some(glium::Rect {
                left: target.get_dimensions().0 - width - offset,
                bottom: offset,
                width,
                height,
            }),
            .. Default::default()
        };

        target.draw(&self.buffer,
                    &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                    &self.program, &uniforms, &params).unwrap();
    }

    fn handle_events(&mut self, window: &winit::Window, event: &winit::Event, io: &imgui::Io) {
        self.cam.handle_events(window, event, io);
    }
}
