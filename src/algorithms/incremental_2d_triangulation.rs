use std::{
    cmp::Ordering,
    time::{ Duration, Instant },
};

use crate::{
    graphics,
    math::{ self, Vec2 },
    ui::window::algorithms::{ Drawable, Configurable },
};

use glium::{
    index, Surface, Frame, Program, VertexBuffer, IndexBuffer, DrawParameters, PolygonMode,
    backend::Facade,
    glutin::{ Event, WindowEvent },
};

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    position: Vec2,
}

impl Vertex {
    pub fn new(position: Vec2) -> Self {
        Self {
            position,
        }
    }
}

implement_vertex!(Vertex, position);

pub struct Incremental2dTriangulation<'f> {
    facade: &'f dyn Facade,
    /// Input points that will be triangulated
    points: Vec<Vertex>,
    program: Program,
    /// Buffer object that stores all the points
    points_buffer: VertexBuffer<Vertex>,
    triangles_buffer: IndexBuffer<u32>,
    exec_time: Option<Duration>,
}

impl<'f> Drawable for Incremental2dTriangulation<'f> {
    fn draw(&self, target: &mut Frame) {
        self.draw_triangles(target);
        self.draw_points(target);
    }

    fn handle_events(&mut self, window: &winit::Window, event: &winit::Event, io: &imgui::Io) {
        if let Event::WindowEvent { event, .. } = event {
            if let WindowEvent::MouseInput { button, state, .. } = event {
                if !io.want_capture_mouse && // Ignore clicks when the cursor is over an ImGui window
                    button == &winit::MouseButton::Left && state == &winit::ElementState::Pressed {

                    // Add a point on click
                    let coords = graphics::window_pos_to_normalized(io.mouse_pos.into(), window);
                    self.add_point(coords);
                }
            }
        }
    }
}

impl<'f> Configurable for Incremental2dTriangulation<'f> {
    fn name(&self) -> &'static str {
        "Incremental 2D triangulation"
    }

    fn configure(&mut self, ui: &imgui::Ui) {
        ui.text(imgui::im_str!("{} vertices, {} triangles", self.points.len(), self.triangles_buffer.get_size() / 3 / std::mem::size_of::<u32>()));

        if ui.button(imgui::im_str!("Clear Points"), [0.0, 0.0]) {
            self.clear();
        }

        if let Some(exec_time) = self.exec_time {
            ui.text(imgui::im_str!("Execution time: {} µs", exec_time.as_micros()));
        }
    }
}

impl<'f> Incremental2dTriangulation<'f> {
    pub fn new(facade: &'f dyn Facade) -> Self {
        let vs = graphics::SHADERS._2d_vs;
        let fs = graphics::SHADERS.basic_fs;
        let program = Program::from_source(facade, vs, fs, None)
                                    .expect("Could not compile shaders");

        Self {
            facade,
            points: Vec::new(),
            program,
            points_buffer: VertexBuffer::empty(facade, 0).unwrap(), // Start without any point
            triangles_buffer: IndexBuffer::empty(facade, index::PrimitiveType::TrianglesList, 0).unwrap(),
            exec_time: None,
        }
    }

    fn draw_points(&self, target: &mut Frame) {
        let indices = index::NoIndices(index::PrimitiveType::Points);
        let uniforms = uniform! {
            color: [ 1.0_f32, 1.0_f32, 1.0_f32 ],
        };
        let mut draw_params = DrawParameters::default();
        draw_params.point_size = Some(8.0);
        target.draw(&self.points_buffer, &indices, &self.program, &uniforms, &draw_params).expect("Draw failure");
    }

    fn draw_triangles(&self, target: &mut Frame) {
        let uniforms = uniform! {
            color: [ 0.0_f32, 0.2_f32, 1.0_f32 ],
        };
        let mut draw_params = DrawParameters::default();
        draw_params.polygon_mode = PolygonMode::Line;
        target.draw(&self.points_buffer, &self.triangles_buffer, &self.program, &uniforms, &draw_params).expect("Draw failure");
    }

    /// Add a point to the input set of points.
    pub fn add_point(&mut self, point: Vec2) {
        self.points.push(Vertex::new(point));
        self.points_buffer = VertexBuffer::new(self.facade, &self.points).unwrap(); // Regenerate the vertex buffer

        let mut positions = self.points
                                    .iter()
                                    .map(|v| v.position)
                                    .collect::<Vec<_>>();
        let start_time = Instant::now();
        let indices = Self::triangulate(&mut positions);
        self.exec_time = Some(Instant::now() - start_time);

        // Convert our positions back to vertices, since the triangulate function sorts the input data (positions),
        // we need to change the order of our vertices vector as well so we recreate it from the positions
        self.points = positions
                            .iter()
                            .map(|p| Vertex { position: *p })
                            .collect();
        self.points_buffer = VertexBuffer::new(self.facade, &self.points).unwrap(); // Regenerate the vertex buffer

        // Convert usize indices to u32s
        let indices = indices.iter()
                                .map(|&idx| idx as u32)
                                .collect::<Vec<_>>();
        self.triangles_buffer = IndexBuffer::new(self.facade, index::PrimitiveType::TrianglesList, &indices).unwrap();
    }

    pub fn random_points(&mut self, n: usize) {
        let x_min = -0.8; // 0.8 to prevent getting too close to the edges of the window
        let x_max = -x_min;
        let y_min = x_min;
        let y_max = -y_min;

        for _ in 0..n {
            self.add_point(Vec2::random_range(x_min, x_max, y_min, y_max));
        }
    }

    /// Removes all the points.
    pub fn clear(&mut self) {
        self.points.clear();
        self.points_buffer = VertexBuffer::empty(self.facade, 0).unwrap();
        self.triangles_buffer = IndexBuffer::empty(self.facade, index::PrimitiveType::TrianglesList, 0).unwrap();
    }

    /// Sorts points by increasing x coordinates, and by increasing y coordinates if two points are on the same vertical line
    fn sort(points: &mut Vec<Vec2>) {
        points.sort_by(|a, b| {
            if a.x < b.x || (math::cmp_f32(a.x, b.x) && a.y < b.y) {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });
    }

    pub fn triangulate(points: &mut Vec<Vec2>) -> Vec<usize> {
        let n = points.len();
        if n < 3 {
            return Vec::new();
        }

        Self::sort(points);
        let mut indices = Vec::new();

        let mut polygon = vec![points[0]];
        let mut first_idx = 0;
        let mut last_collinear = Vec2::default();
        for i in 1..n {
            let a = points[i - 1];
            let b = points[i];
            let s = &a - &b;

            if i > 1 && !last_collinear.collinear(s) {
                first_idx = i;
                break;
            }
            last_collinear = s;
            polygon.push(b);
        }

        for i in first_idx..n {
            // Refresh the convex hull
            let hull_indices = super::JarvisMarch::march(polygon.iter());
            let convex_hull = hull_indices.iter()
                                    .map(|&idx| polygon[idx])
                                    .collect::<Vec<_>>();

            let mut added_to_polygon = false;
            let hull_size = convex_hull.len();
            for j in 0..hull_size {
                let idx_b = hull_indices[j];
                let idx_a = if j + 1 < hull_size {
                    hull_indices[j + 1]
                } else {
                    hull_indices[0]
                };
                let idx_c = i;
                let a = polygon[idx_a];
                let b = polygon[idx_b];
                let c = points[idx_c];
                if Vec2::cw(a, b, c) {
                    indices.push(idx_a);
                    indices.push(idx_b);
                    indices.push(idx_c);
                    if !added_to_polygon {
                        polygon.push(c);
                        added_to_polygon = true;
                    }
                }
            }
        }

        indices
    }

    pub fn get_triangles(indices: &mut Vec<usize>) -> (Vec<(usize, usize, usize)>, Vec<usize>) {
        let mut triangles = vec![];
        let mut pos = vec![];
        for i in (0..indices.len()).step_by(3) {
            triangles.push((indices[i], indices[i+1], indices[i+2]));
            pos.push(i);
        }
        (triangles, pos)
    }

    pub fn determinant() -> i32 {
        0
    }
    
    pub fn indice_in_triangle(i: usize, triangle: &(usize, usize, usize)) -> bool {
        i == triangle.0 || i == triangle.1 || i == triangle.2
    }

    pub fn get_opposite(triangle1: &(usize, usize, usize), triangle2: &(usize, usize, usize)) -> (usize, usize, usize, usize) {
        let (mut opposite1, mut opposite2, mut edge1, mut edge2) = (0, 0, 0, 0);
        if Self::indice_in_triangle(triangle1.0, triangle2) {
            opposite1 = triangle1.0;
        }
        else {
            edge1 = triangle1.0;
        }
        if Self::indice_in_triangle(triangle1.1, triangle2) {
            if opposite1 == 0 {
                opposite1 = triangle1.1;
            }
            else {
                opposite2 = triangle1.1;
            }
        }
        else {
            if edge1 == 0 {
                edge1 = triangle1.1;
            }
            else {
                edge2 = triangle1.1;
            }
        }
        if Self::indice_in_triangle(triangle1.2, triangle2) {
            if opposite1 == 0 {
                opposite1 = triangle1.2;
            }
            else {
                opposite2 = triangle1.2;
            }
        }
        else {
            if edge1 == 0 {
                edge1 = triangle1.2;
            }
            else {
                edge2 = triangle1.2;
            }
        }
        (opposite1, opposite2, edge1, edge2)
    }

    pub fn edge_flipping(indices: &mut Vec<usize>, points: &Vec<Vec2>) {
        let (triangles, pos) = Self::get_triangles(indices);
        for triangle in &mut triangles {
            for other_triangle in &mut triangles {
                if triangle == other_triangle {
                    continue;
                }
                if Self::determinant() > 0 {
                    let (opposite1, opposite2, edge1, edge2) = Self::get_opposite(triangle, other_triangle);
                    triangle = &mut (opposite1, edge1, opposite2);
                    other_triangle = &mut (opposite1, opposite2, edge2);
                    //newTri1 = [iOpposite1, edge[0], iOpposite2]
                    //newTri2 = [iOpposite1, iOpposite2, edge[1]]
                }
            }
        }
        // ve: get_edges();
        // while (!ve.is_empty()) {
        //      edge = ve.pop();
        //      if !delauney_critera(edge) {
        //          [0, 1, 2]
        //          [2, 1, 3]
        //           ->
        //          [0, 1, 3]
        //          [0, 3, 2]
        //      }
        // }
    }
}
