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

pub struct JarvisMarch<'f> {
    facade: &'f dyn Facade,
    /// Input points that will be wrapped in the generated hull
    points: Vec<Vertex>,
    program: Program,
    /// Buffer object that stores all the points
    points_buffer: VertexBuffer<Vertex>,
    /// Buffer object that stores the points that form the hull
    hull_buffer: VertexBuffer<Vertex>,
}

impl<'f> Drawable for JarvisMarch<'f> {
    fn draw(&self, target: &mut Frame) {
        self.draw_points(target);
        self.draw_hull(target);
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

impl<'f> Configurable for JarvisMarch<'f> {
    fn name(&self) -> &'static str {
        "Jarvis march"
    }

    fn configure(&mut self, ui: &imgui::Ui) {
        if ui.button(imgui::im_str!("Clear Points"), [0.0, 0.0]) {
            self.clear();
        }
    }
}

impl<'f> JarvisMarch<'f> {
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
        self.points.push(Vertex::new(point));
        self.points_buffer = VertexBuffer::new(self.facade, &self.points).unwrap(); // Regenerate the buffer

        let input = self.points.iter().map(|p| &p.position); // Prepare input for the march algorithm
        let hull = Self::march(input)
                            .into_iter()
                            .map(|idx| self.points[idx])
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

    /// Find the point that is the furthest on the left.
    /// If two points are on the same vertical line, the one that has the lower coordinates is returned.
    /// Returns a tuple that contains the index of the point found and a reference to its position in the input iterator.
    fn leftmost_point<'a, I>(points: I) -> (usize, &'a Vec2)
    where I: ExactSizeIterator<Item = &'a Vec2> + Clone {
        assert_ne!(points.len(), 0);

        let first = *points.clone().peekable().peek().unwrap();
        points
            .enumerate()
            .scan((0, first), |leftmost, p| {
                let pos = p.1;
                let lpos = leftmost.1;
                if pos.x < lpos.x || (math::cmp_f32(pos.x, lpos.x) && pos.y < lpos.y) {
                    *leftmost = p;
                }
                Some(*leftmost)
            })
            .last()
            .unwrap()
    }

    /// Returns a `Vec` of the indices of the points in the
    /// specified iterator `points` that form the convex hull.
    pub fn march<'a, I>(points: I) -> Vec<usize>
    where I: ExactSizeIterator<Item = &'a Vec2> + Clone {
        let mut hull = Vec::new();

        if points.len() < 2 {
            return hull;
        }

        let leftmost = Self::leftmost_point(points.clone());
        let mut hull_point = leftmost; // Start with the leftmost point
        let mut peekable = points.clone().peekable();

        loop {
            hull.push(hull_point.0);
            let mut best = (0, *peekable.peek().unwrap()); // The current best point candidate for the hull

            for checked in points.clone().enumerate() {
                let hullpoint_to_checked = checked.1 - hull_point.1;
                let hullpoint_to_best = best.1 - hull_point.1;
                let angle = hullpoint_to_checked.signed_angle(hullpoint_to_best);
                if best.0 == hull_point.0 || angle < 0.0 {
                    best = checked; // We found a better candidate
                }
            }
            hull_point = best; // Add the point we found to the hull

            if hull_point.0 == leftmost.0 { // Wrapped around all points, we're done
                break;
            }
        }

        hull
    }
}
