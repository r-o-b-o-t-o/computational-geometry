use crate::math::{ self, Vec2 };

use glium::{
    index, Surface, Frame, Program, VertexBuffer, DrawParameters,
    backend::Facade,
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

impl<'f> JarvisMarch<'f> {
    pub fn new(facade: &'f dyn Facade) -> Self {
        let program = Program::from_source(facade, include_str!("../../shaders/2d.vs.glsl"), include_str!("../../shaders/basic.fs.glsl"), None).expect("Could not compile shaders");

        Self {
            facade,
            points: Vec::new(),
            program,
            points_buffer: VertexBuffer::empty(facade, 0).unwrap(),
            hull_buffer: VertexBuffer::empty(facade, 0).unwrap(),
        }
    }

    pub fn draw(&self, target: &mut Frame) {
        self.draw_points(target);
        self.draw_hull(target);
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
        self.march();
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

    fn leftmost_point(&self) -> (usize, &Vertex) {
        assert!(!self.points.is_empty());

        self.points
                .iter()
                .enumerate()
                .scan((0, &self.points[0]), |leftmost, p| {
                    let pos = p.1.position;
                    let lpos = leftmost.1.position;
                    if pos.x < lpos.x || math::cmp_f32(pos.x, lpos.x) && pos.y < lpos.y {
                        *leftmost = p;
                    }
                    Some(*leftmost)
                })
                .last()
                .unwrap()
    }

    fn march(&mut self) {
        if self.points.len() < 2 {
            return;
        }

        let mut hull = Vec::new();
        let leftmost = self.leftmost_point();
        let mut hull_point = leftmost;

        loop {
            hull.push(*hull_point.1);
            let mut best = (0, &self.points[0]);

            for checked in self.points.iter().enumerate() {
                let hullpoint_to_checked = &checked.1.position - &hull_point.1.position;
                let hullpoint_to_best = &best.1.position - &hull_point.1.position;
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

        self.hull_buffer = VertexBuffer::new(self.facade, &hull).unwrap();
    }
}
