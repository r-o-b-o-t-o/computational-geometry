use crate::{
    graphics,
    math::{ Vec3, ToArray },
    ui::window::algorithms::{ Drawable, Configurable },
};

use std::time::{ Duration, /*Instant*/ };

use glium::{
    index, Surface, Frame, Program, VertexBuffer, IndexBuffer, DrawParameters, Depth, DepthTest,
    backend::Facade,
};
use cgmath::{ SquareMatrix, InnerSpace };

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    position: Vec3,
    normal: Vec3,
}

impl Vertex {
    pub fn new(position: Vec3, normal: Vec3) -> Self {
        Self {
            position,
            normal,
        }
    }
}

pub enum Color {
    White,
    Red,
    Blue,
    Purple,
}

pub struct GraphVertex {
    pub idx: u32,
    pub color: Color,
}

pub struct GraphEdge {
    pub idx: (u32, u32),
    pub color: Color,
}

pub struct GraphFace {
    pub idx: (u32, u32, u32),
    pub color: Color,
}

impl GraphFace {
    pub fn normal(&self, vertices: &Vec<Vertex>) -> Vec3 {
        let p1 = vertices[self.idx.0 as usize].position.0;
        let p2 = vertices[self.idx.1 as usize].position.0;
        let p3 = vertices[self.idx.2 as usize].position.0;
        (p2 - p1).cross(p3 - p1).into()
    }
}

#[derive(Default)]
pub struct Graph {
    pub vertices: Vec<GraphVertex>,
    pub edges: Vec<GraphEdge>,
    pub faces: Vec<GraphFace>,
}

implement_vertex!(Vertex, position, normal);

pub struct ConvexHull<'f> {
    facade: &'f dyn Facade,
    cam: graphics::OrbitCamera,
    axes: graphics::axes::Axes,
    /// Input points that will be wrapped in the generated hull
    points: Vec<Vertex>,
    points_program: Program,
    faces_program: Program,
    /// Buffer object that stores all the points
    points_buffer: VertexBuffer<Vertex>,
    faces_indices: IndexBuffer<u32>,
    exec_time: Option<Duration>,
    rotate_model: bool,
    is_rotating: bool,
    model_matrix: cgmath::Matrix4<f32>,
}

impl<'f> Drawable for ConvexHull<'f> {
    fn draw(&self, target: &mut Frame) {
        self.draw_points(target);
        self.draw_faces(target);
        self.axes.draw(target);
    }

    fn handle_events(&mut self, window: &winit::Window, event: &winit::Event, io: &imgui::Io) {
        self.cam.handle_events(window, event, io);
        if !self.rotate_model {
            self.axes.handle_events(window, event, io);
        }

        if let winit::Event::WindowEvent { event, .. } = event {
            if let winit::WindowEvent::MouseInput { button, state, .. } = event {
                if !io.want_capture_mouse && // Ignore clicks when the cursor is over an ImGui window
                    button == &winit::MouseButton::Left && self.rotate_model {

                    self.is_rotating = state == &winit::ElementState::Pressed;
                }
            }
        }

        if let winit::Event::DeviceEvent { event, .. } = event {
            if let winit::DeviceEvent::MouseMotion { delta } = event {
                if self.rotate_model && self.is_rotating {
                    self.model_matrix = self.model_matrix * cgmath::Matrix4::<f32>::from_angle_y(cgmath::Rad { 0: delta.0 as f32 * 0.01 });
                    self.model_matrix = self.model_matrix * cgmath::Matrix4::<f32>::from_angle_x(cgmath::Rad { 0: delta.1 as f32 * 0.01 });
                }
            }
        }
    }
}

impl<'f> Configurable for ConvexHull<'f> {
    fn name(&self) -> &'static str {
        "3D Convex hull"
    }

    fn configure(&mut self, ui: &imgui::Ui) {
        ui.text(imgui::im_str!("{} vertices", self.points.len()));

        if ui.checkbox(imgui::im_str!("Rotate model"), &mut self.rotate_model) {
            self.cam.zoom = !self.rotate_model;
            self.cam.rotate = !self.rotate_model;
        }

        if let Some(exec_time) = self.exec_time {
            ui.text(imgui::im_str!("Execution time: {} µs", exec_time.as_micros()));
        }
    }
}

impl<'f> ConvexHull<'f> {
    pub fn new(facade: &'f dyn Facade) -> Self {
        let vs = graphics::SHADERS._3d_vs;
        let fs = graphics::SHADERS.basic_fs;
        let points_program = Program::from_source(facade, vs, fs, None)
                                    .expect("Could not compile shaders");

        let vs = graphics::SHADERS._3d_vs;
        let fs = graphics::SHADERS.phong_fs;
        let faces_program = Program::from_source(facade, vs, fs, None)
                                    .expect("Could not compile shaders");

        let mut points = vec! [
            Vertex {position: Vec3::new(-0.5, 0.0, 0.0,), normal: Vec3::new(0.0, 0.0, 0.0)},
            Vertex {position: Vec3::new(0.0, 0.0, 0.5,), normal: Vec3::new(0.0, 0.0, 0.0)},
            Vertex {position: Vec3::new(0.5, 0.0, 0.0,), normal: Vec3::new(0.0, 0.0, 0.0)},
            Vertex {position: Vec3::new(0.0, 0.5, 0.0,), normal: Vec3::new(0.0, 0.0, 0.0)},
            Vertex {position: Vec3::new(0.9, 0.6, 0.2,), normal: Vec3::new(0.0, 0.0, 0.0)},
        ];
        let graph = Self::hull(&points);
        let faces = graph.faces.iter().fold(vec![], |mut vec, indices| {
            vec.push(indices.idx.0);
            vec.push(indices.idx.1);
            vec.push(indices.idx.2);
            vec
        });
        Self::compute_normals(&mut points, &graph);
        let points_buffer = VertexBuffer::new(facade, &points).unwrap();
        let faces_indices = IndexBuffer::new(facade, glium::index::PrimitiveType::TrianglesList, &faces).unwrap();

        Self {
            facade,
            cam: graphics::OrbitCamera::new(10.0, Vec3::new(0.0, 0.0, 0.0), 45.0, 1024.0 / 768.0, 0.1, 100.0),
            axes: graphics::axes::Axes::new(facade),
            points,
            points_program,
            faces_program,
            points_buffer,
            faces_indices,
            exec_time: None,
            rotate_model: false,
            is_rotating: false,
            model_matrix: cgmath::Matrix4::<f32>::identity(),
        }
    }

    fn draw_points(&self, target: &mut Frame) {
        let indices = index::NoIndices(index::PrimitiveType::Points);
        let uniforms = uniform! {
            model: cgmath::Matrix4::<f32>::identity().to_array(),
            view: self.cam.get_view_matrix().to_array(),
            projection: self.cam.get_projection_matrix().to_array(),
            color: [ 0.0_f32, 0.2_f32, 1.0_f32 ],
        };
        let mut draw_params = DrawParameters::default();
        draw_params.point_size = Some(8.0);
        draw_params.depth = Depth {
            write: true,
            test: DepthTest::IfLess,
            .. Default::default()
        };
        target.draw(&self.points_buffer, &indices, &self.points_program, &uniforms, &draw_params).expect("Draw failure");
    }

    fn draw_faces(&self, target: &mut Frame) {
        let uniforms = uniform! {
            model: self.model_matrix.to_array(),
            view: self.cam.get_view_matrix().to_array(),
            projection: self.cam.get_projection_matrix().to_array(),
            viewPosition: self.cam.get_position().to_array(),
            materialAmbient: [ 1.0_f32, 1.0_f32, 1.0_f32 ],
            materialDiffuse: [ 1.0_f32, 1.0_f32, 1.0_f32 ],
            materialSpecular: [ 0.5_f32, 0.5_f32, 0.5_f32 ],
            materialShininess: 32.0_f32,
            lightPosition: [ 5.0_f32, 1.0_f32, 0.0_f32 ],
            lightAmbient: [ 0.2_f32, 0.2_f32, 0.2_f32 ],
            lightDiffuse: [ 0.75_f32, 0.75_f32, 0.75_f32 ],
            lightSpecular: [ 0.6_f32, 0.6_f32, 0.6_f32 ],
        };
        let mut draw_params = DrawParameters::default();
        draw_params.depth = Depth {
            write: true,
            test: DepthTest::IfLess,
            .. Default::default()
        };
        target.draw(&self.points_buffer, &self.faces_indices, &self.faces_program, &uniforms, &draw_params).expect("Draw failure");
    }

    pub fn hull(points: &Vec<Vertex>) -> Graph {
        let mut graph = Graph::default();

        let n = points.len();
        if n < 4 {
            return graph;
        }
        let iter = points.iter().skip(4);

        // Initial tetrahedron
        graph.vertices.push(GraphVertex { idx: 0, color: Color::White });
        graph.vertices.push(GraphVertex { idx: 1, color: Color::White });
        graph.vertices.push(GraphVertex { idx: 2, color: Color::White });
        graph.vertices.push(GraphVertex { idx: 3, color: Color::White });
        graph.edges.push(GraphEdge { idx: (0, 1), color: Color::White });
        graph.edges.push(GraphEdge { idx: (0, 2), color: Color::White });
        graph.edges.push(GraphEdge { idx: (0, 3), color: Color::White });
        graph.edges.push(GraphEdge { idx: (1, 2), color: Color::White });
        graph.edges.push(GraphEdge { idx: (1, 3), color: Color::White });
        graph.edges.push(GraphEdge { idx: (2, 3), color: Color::White });
        graph.faces.push(GraphFace { idx:(0, 1, 2), color: Color::White });
        graph.faces.push(GraphFace { idx:(0, 1, 3), color: Color::White });
        graph.faces.push(GraphFace { idx:(0, 2, 3), color: Color::White });
        graph.faces.push(GraphFace { idx:(1, 2, 3), color: Color::White });
        let b = points[graph.vertices[0].idx as usize].position.0;

        for _point_to_add in iter {
            for face in graph.faces.iter() {
                let mut normal = face.normal(points);
                let p1 = points[face.idx.0 as usize].position.0;
                if normal.0.dot(b - p1) > 0.0 {
                    normal.0 *= -1.0;
                }
            }
        }

        graph
    }

    fn compute_normals(vertices: &mut Vec<Vertex>, graph: &Graph) {
        for v in vertices.iter_mut() {
            v.normal = Vec3::new(0.0, 0.0, 0.0);
        }

        for tri in graph.faces.iter() {
            let tri = vec![tri.idx.0, tri.idx.1, tri.idx.2];
            let p1 = vertices[tri[0] as usize].position.0;
            let p2 = vertices[tri[1] as usize].position.0;
            let p3 = vertices[tri[2] as usize].position.0;
            for vert_idx in tri {
                let vert = &mut vertices[vert_idx as usize];
                let face_normal = (p2 - p1).cross(p3 - p1);
                vert.normal = (vert.normal.0 + face_normal).into();
            }
        }

        for v in vertices.iter_mut() {
            if v.normal.0 == Vec3::new(0.0, 0.0, 0.0).0 {
                continue;
            }
            v.normal = v.normal.0.normalize().into();
        }
    }

    pub fn clear(&mut self) {
        self.points.clear();
        self.points_buffer = VertexBuffer::empty(self.facade, 0).unwrap();
        self.faces_indices = IndexBuffer::empty(self.facade, glium::index::PrimitiveType::TrianglesList, 0).unwrap();
    }
}
