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

implement_vertex!(Vertex, position);

pub struct JarvisMarch<'f> {
    facade: &'f dyn Facade,
    points: Vec<Vertex>,
    program: Program,
    points_buffer: VertexBuffer<Vertex>,
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
                if !io.want_capture_mouse && button == &winit::MouseButton::Left && state == &winit::ElementState::Pressed {
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
            points_buffer: VertexBuffer::empty(facade, 0).unwrap(),
            hull_buffer: VertexBuffer::empty(facade, 0).unwrap(),
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

    pub fn add_point(&mut self, point: Vec2) {
        self.points.push(Vertex {
            position: point,
        });
        self.points_buffer = VertexBuffer::new(self.facade, &self.points).unwrap();

        let input = self.points.iter().map(|p| &p.position);
        let hull = Self::march(input)
                            .into_iter()
                            .map(|position| Vertex { position })
                            .collect::<Vec<_>>();
        self.hull_buffer = VertexBuffer::new(self.facade, &hull).unwrap();
    }

    pub fn random_points(&mut self, n: usize) {
        let x_min = -0.8;
        let x_max = -x_min;
        let y_min = x_min;
        let y_max = -y_min;

        for _ in 0..n {
            self.add_point(Vec2::random_range(x_min, x_max, y_min, y_max));
        }
    }

    pub fn clear(&mut self) {
        self.points.clear();
        self.points_buffer = VertexBuffer::empty(self.facade, 0).unwrap();
        self.hull_buffer = VertexBuffer::empty(self.facade, 0).unwrap();
    }

    fn leftmost_point<'a, I>(points: I) -> (usize, &'a Vec2)
    where I: ExactSizeIterator<Item = &'a Vec2> + Clone {
        assert_ne!(points.len(), 0);

        let first = *points.clone().peekable().peek().unwrap();
        points
            .enumerate()
            .scan((0, first), |leftmost, p| {
                let pos = p.1;
                let lpos = leftmost.1;
                if pos.x < lpos.x || math::cmp_f32(pos.x, lpos.x) && pos.y < lpos.y {
                    *leftmost = p;
                }
                Some(*leftmost)
            })
            .last()
            .unwrap()
    }

    pub fn march<'a, I>(points: I) -> Vec<Vec2>
    where I: ExactSizeIterator<Item = &'a Vec2> + Clone {
        let mut hull = Vec::new();

        if points.len() < 2 {
            return hull;
        }

        let leftmost = Self::leftmost_point(points.clone());
        let mut hull_point = leftmost;
        let mut peekable = points.clone().peekable();

        loop {
            hull.push(*hull_point.1);
            let mut best = (0, *peekable.peek().unwrap());

            for checked in points.clone().enumerate() {
                let hullpoint_to_checked = checked.1 - hull_point.1;
                let hullpoint_to_best = best.1 - hull_point.1;
                let angle = hullpoint_to_checked.signed_angle(hullpoint_to_best);
                if best.0 == hull_point.0 || angle < 0.0 {
                    best = checked;
                }
            }
            hull_point = best;

            if hull_point.0 == leftmost.0 {
                break;
            }
        }

        hull
    }
}
