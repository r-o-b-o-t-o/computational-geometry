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

use cgmath::Matrix4;
use cgmath::Vector4;
use cgmath::SquareMatrix;

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

        if ui.button(imgui::im_str!("Edge Flipping"), [0.0, 0.0]) {
            self.flip_edges();
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

    pub fn flip_edges(&mut self) {
        self.points_buffer = VertexBuffer::new(self.facade, &self.points).unwrap(); // Regenerate the vertex buffer

        let mut positions = self.points
                                    .iter()
                                    .map(|v| v.position)
                                    .collect::<Vec<_>>();
        //let start_time = Instant::now();
        let mut indices = Self::triangulate(&mut positions);
        //dbg!(&indices);
        if !indices.is_empty() {
            Self::edge_flipping(&mut indices, &positions);

            //dbg!(&indices);
        }
        //self.exec_time = Some(Instant::now() - start_time);
        
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
        //println!("flipped edges");
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

    pub fn get_triangles(indices: &mut Vec<usize>) -> Vec<(usize, usize, usize)> {
        let mut triangles = vec![];
        for i in (0..indices.len()).step_by(3) {
            triangles.push((indices[i], indices[i+1], indices[i+2]));
        }
        triangles
    }

    pub fn determinant(triangle1: &(usize, usize, usize), triangle2: &(usize, usize, usize), pos: &[Vec2]) -> f32 {
        let mut vec = vec![triangle1.0, triangle1.1, triangle1.2, triangle2.0, triangle2.1, triangle2.2];
        vec.dedup();

        let mat = Matrix4::from_cols(
            Vector4::new(pos[vec[0]].x,pos[vec[1]].x,pos[vec[2]].x,pos[vec[3]].x),
            Vector4::new(pos[vec[0]].y,pos[vec[1]].y,pos[vec[2]].y,pos[vec[3]].y),
            Vector4::new(pos[vec[0]].x.powf(2.0) + pos[vec[0]].y.powf(2.0),pos[vec[1]].x.powf(2.0) + pos[vec[1]].y.powf(2.0),pos[vec[2]].x.powf(2.0) + pos[vec[2]].y.powf(2.0),pos[vec[3]].x.powf(2.0) + pos[vec[3]].y.powf(2.0)),
            Vector4::new(1.0_f32, 1.0_f32,1.0_f32,1.0_f32)
        );

        mat.determinant()
    }
    
    pub fn indice_in_triangle(i: usize, triangle: &(usize, usize, usize)) -> bool {
        i == triangle.0 || i == triangle.1 || i == triangle.2
    }

    pub fn get_opposite(triangle1: &(usize, usize, usize), triangle2: &(usize, usize, usize)) -> (usize, usize, usize, usize) {
        let vec = vec![triangle1.0, triangle1.1, triangle1.2, triangle2.0, triangle2.1, triangle2.2];
        let mut single = vec.clone();
        single.dedup();
        //let (mut opposite1, mut opposite2, mut edge1, mut edge2) = (0, 0, 0, 0);
        let mut edges = vec![];
        let mut opposites = vec![];
        //dbg!(&single);
        for s in single{
            if vec.iter().filter(|&n| *n == s).count() > 1 {
                opposites.push(s);
            } else {
                edges.push(s);
            }
        }
        (opposites[0], opposites[1], edges[0], edges[1])
    }

    pub fn prod_vec(a: Vec2, b: Vec2, c: Vec2) -> f32 {
        (b.x - a.x) * (c.y - a.y) - (c.x - a.x) * (b.y - a.y)
    }

    pub fn has_sim_edge(triangle1: &(usize, usize, usize), triangle2: &(usize, usize, usize)) -> bool {
        let vec1 = vec![triangle1.0, triangle1.1, triangle1.2];
        let vec2 = vec![triangle2.0, triangle2.1, triangle2.2];

        let mut p1 = false;
        let mut p2 = false;
        for i in vec1{
            if vec2.iter().filter(|&n| *n == i).count() > 0 {
                if p1 {
                    p2 = true;
                } else {
                    p1 = true;
                }
            }
        }

        p1 & p2
    }

    pub fn edge_flipping(indices: &mut Vec<usize>, points: &[Vec2]) {
        let mut triangles = Self::get_triangles(indices);
        let mut res = vec![];
        //dbg!(&indices);
        //dbg!(&triangles);

        //dbg!(&pos);
        //let mut i = pos.len();
        while !triangles.is_empty() {
            //i-=1;
            let mut to_push = None;
            let triangle = &mut triangles.pop().unwrap();
            for other_triangle in &mut triangles {
                if *triangle == *other_triangle {
                    continue;
                }
                // if is_ccw > 0, its ccw, if is_ccw = 0 then collinear, else cw
                let is_ccw = Self::prod_vec(points[triangle.0], points[triangle.1], points[triangle.2]);
                if Self::has_sim_edge(triangle, other_triangle) &&
                ((is_ccw > 0.00001 && Self::determinant(triangle, other_triangle, &points) > 0.00001) ||
                 (is_ccw < -0.00001 && Self::determinant(triangle, other_triangle, &points) < -0.00001)) {
                    //println!("Flipping edge");
                    let (opposite1, opposite2, edge1, edge2) = Self::get_opposite(triangle, other_triangle);
                    //*triangle = (opposite1, edge1, opposite2);
                    *triangle = if is_ccw > 0.0 { 
                        (edge1, opposite1, edge2)
                    } else {
                        (edge2, opposite1, edge1)
                    };
                    //*other_triangle = (opposite1, opposite2, edge2);
                    *other_triangle = if is_ccw > 0.0 { 
                        (edge1, edge2, opposite2)
                    } else {
                        (opposite2, edge2, edge1)
                    };
                    //*other_triangle = (edge1, edge2, opposite2);
                    //newTri1 = [iOpposite1, edge[0], iOpposite2]
                    //newTri2 = [iOpposite1, iOpposite2, edge[1]]
                    to_push = Some(*triangle);
                    break;
                }
            }
            if to_push == None {
                res.insert(0,triangle.2);
                res.insert(0,triangle.1);
                res.insert(0,triangle.0);
            } else {
                triangles.push(to_push.unwrap());
            }
            //indices[pos[i]] = triangle.0;
            //indices[pos[i]+1] = triangle.1;
            //indices[pos[i]+2] = triangle.2;

            //dbg!(&res);
        }

        *indices = res;
        //dbg!(&indices);

        // ve: get_edges();
        // while (!ve.is_empty()) {
        //      edge = ve.pop();
        //      if !delauney_critera(edge) {
        //          [0, 1, 2]
        //          [2, 1, 3]
        //           ->
        //          [0, 1, 3] || [0, 1, 3]
        //          [0, 3, 2] || [0, 3, 2]
        //      }
        // }
    }
}
