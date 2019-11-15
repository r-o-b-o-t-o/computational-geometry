use crate::{
    graphics,
    math::{ self, Vec2 },
    ui::window::algorithms::{ Drawable, Configurable },
};

use glium::{
    index, Surface, Frame, Program, VertexBuffer, DrawParameters,
    backend::Facade,
    glutin::{ Event, WindowEvent },
};

use std::cmp::Ordering;

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    position: Vec2,
}

impl Vertex {
    pub fn new(position: Vec2) -> Self {
        Self {
            position
        }
    }
}

implement_vertex!(Vertex, position);

pub struct GrahamScan<'f> {
    facade: &'f dyn Facade,
    /// Input points that will be wrapped in the generated hull
    points: Vec<Vertex>,
    program: Program,
    /// Buffer object that stores all the points
    points_buffer: VertexBuffer<Vertex>,
    /// Buffer object that stores the points that form the hull
    hull_buffer: VertexBuffer<Vertex>,
}

impl<'f> Drawable for GrahamScan<'f> {
    fn draw(&self, target: &mut Frame) {
        self.draw_points(target);
        self.draw_hull(target);
    }

    fn handle_events(&mut self, window: &winit::Window, event: &winit::Event, io: &imgui::Io) {
        if let Event::WindowEvent { event, .. } = event {
            if let WindowEvent::MouseInput { button, state, .. } = event {
                if !io.want_capture_mouse && // Ignore clicks when the cursor is over an ImGui window
                    button == &winit::MouseButton::Left && state == &winit::ElementState::Pressed {

                    // Add a point when the window is clicked
                    let coords = graphics::window_pos_to_normalized(io.mouse_pos.into(), window);
                    self.add_point(coords);
                }
            }
        }
    }
}

impl<'f> Configurable for GrahamScan<'f> {
    fn name(&self) -> &'static str {
        "Graham scan"
    }

    fn configure(&mut self, ui: &imgui::Ui) {
        if ui.button(imgui::im_str!("Clear Points"), [0.0, 0.0]) {
            self.clear();
        }
    }
}

impl<'f> GrahamScan<'f> {
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
            hull_buffer: VertexBuffer::empty(facade, 0).unwrap(), // Same for the hull
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

    fn draw_hull(&self, target: &mut Frame) {
        let indices = index::NoIndices(index::PrimitiveType::LineLoop);
        let uniforms = uniform! {
            color: [ 0.0, 0.8_f32, 0.0 ],
        };
        let draw_params = DrawParameters::default();
        target.draw(&self.hull_buffer, &indices, &self.program, &uniforms, &draw_params).expect("Draw failure");

        let indices = index::NoIndices(index::PrimitiveType::Points);
        let uniforms = uniform! {
            color: [ 0.0, 0.0, 0.8_f32 ],
        };
        target.draw(&self.hull_buffer, &indices, &self.program, &uniforms, &draw_params).expect("Draw failure");
    }

    /// Add an input point that will be used to compute the convex hull.
    pub fn add_point(&mut self, point: Vec2) {
        self.points.push(Vertex {
            position: point,
        });
        self.points_buffer = VertexBuffer::new(self.facade, &self.points).unwrap(); // Regenerate the buffer

        let hull = Self::scan(&self.points.iter().map(|p| p.position).collect::<Vec<_>>())
                            .into_iter()
                            .map(Vertex::new)
                            .collect::<Vec<_>>();
        self.hull_buffer = VertexBuffer::new(self.facade, &hull).unwrap(); // Regenerate the hull buffer from result
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
        self.hull_buffer = VertexBuffer::empty(self.facade, 0).unwrap();
    }

    /// Find the point that is the furthest on the bottom (first) and left.
    /// If two points are on the same horizontal line, the one that has the lower x coordinates is returned.
    /// Returns a tuple that contains the index of the point found and a reference to its position in the input iterator.
    fn bottommost_point<'a, I>(points: I) -> (usize, &'a Vec2)
    where I: ExactSizeIterator<Item = &'a Vec2> + Clone {
        assert_ne!(points.len(), 0);

        let first = *points.clone().peekable().peek().unwrap();
        points
            .enumerate()
            .scan((0, first), |leftmost, p| {
                let pos = p.1;
                let lpos = leftmost.1;
                if pos.y < lpos.y || math::cmp_f32(pos.y, lpos.y) && pos.x < lpos.x {
                    *leftmost = p;
                }
                Some(*leftmost)
            })
            .last()
            .unwrap()

    }

    pub fn prod_vec(a: Vec2, b: Vec2, c: Vec2) -> f32 {
        (b.x - a.x) * (c.y - a.y) - (c.x - a.x) * (b.y - a.y)
    }

    /// Returns a `Vec` of points
    /// that form the convex hull.
    pub fn scan(points:  &[Vec2]) -> Vec<Vec2> {
        let mut hull = Vec::new();

        if points.len() < 2 {
            return hull;
        }

        let bottommost = Self::bottommost_point(points.iter());
        let mut points_clone = points.to_owned();

        points_clone.remove(bottommost.0);

        points_clone.sort_by(|a, b| {
            let vec1 = a - bottommost.1;
            let vec2 = b - bottommost.1;
            if vec1.signed_angle(vec2) < 0.0 {
                Ordering::Greater
            } else if vec1.signed_angle(vec2) == 0.0 {
                Ordering::Equal
            } else {
                Ordering::Less
            }
        });

        hull.push(*bottommost.1);

        for point in points_clone {
            while hull.len() > 1 && Self::prod_vec(hull[hull.len()-2], *hull.last().unwrap(), point) < 0.0 {
                hull.pop();
            }
            hull.push(point);
        }
        hull
    }
}
