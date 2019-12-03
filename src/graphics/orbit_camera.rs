use crate::math::{ self, Vec2, Vec3 };

use cgmath::{ Matrix4, Point3 };
use glium::glutin::{ Event, WindowEvent, DeviceEvent, MouseScrollDelta };

pub struct OrbitCamera {
    pub target: Vec3,
    pub sensitivity: Vec2,
    pub rotate: bool,
    pub zoom: bool,

    is_rotating: bool,

    projection_matrix: Matrix4<f32>,
    distance: f32,
    phi: f32,
    theta: f32,
}

impl OrbitCamera {
    pub fn new(distance: f32, target: Vec3, fov: f32, aspect_ratio: f32, near: f32, far: f32) -> Self {
        let fov: cgmath::Rad<f32> = cgmath::Deg { 0: fov }.into();

        Self {
            target,
            sensitivity: Vec2::new(0.01, 0.01),
            rotate: true,
            zoom: true,

            is_rotating: false,

            projection_matrix: cgmath::perspective(fov, aspect_ratio, near, far),
            distance,
            phi: 90.0_f32.to_radians(),
            theta: 0.0,
        }
    }

    pub fn get_position(&self) -> Vec3 {
        let mut pos = Vec3::new(
            self.theta.cos() * self.phi.cos(),
            self.theta.sin(),
            self.theta.cos() * self.phi.sin(),
        );
        pos.0 *= self.distance;
        pos
    }

    pub fn get_view_matrix(&self) -> Matrix4<f32> {
        let pos = self.get_position().0;
        let pos = Point3::new(pos.x, pos.y, pos.z);

        let target = self.target.0;
        let target = Point3::new(target.x, target.y, target.z);

        let up = Vec3::new(0.0, 1.0, 0.0).0;

        Matrix4::<f32>::look_at(pos, target, up)
    }

    pub fn get_projection_matrix(&self) -> &Matrix4<f32> {
        &self.projection_matrix
    }

    pub fn zoom(&mut self, offset: f32) {
        self.distance -= offset;
        self.distance = math::clamp(self.distance, 0.5, 50.0);
    }

    pub fn handle_events(&mut self, _window: &winit::Window, event: &winit::Event, io: &imgui::Io) {
        if let Event::WindowEvent { event, .. } = event {
            if let WindowEvent::MouseInput { button, state, .. } = event {
                if !io.want_capture_mouse && // Ignore clicks when the cursor is over an ImGui window
                    button == &winit::MouseButton::Left && self.rotate {

                    self.is_rotating = state == &winit::ElementState::Pressed;
                }
            }

            if let WindowEvent::MouseWheel { delta, .. } = event {
                if let MouseScrollDelta::LineDelta { 0: _scroll_x, 1: scroll_y } = delta {
                    if !io.want_capture_mouse && self.zoom {
                        self.zoom(*scroll_y)
                    }
                }
            }
        }

        if let Event::DeviceEvent { event, .. } = event {
            if let DeviceEvent::MouseMotion { delta } = event {
                if self.rotate && self.is_rotating {
                    self.phi += delta.0 as f32 * self.sensitivity.x;
                    self.theta += delta.1 as f32 * self.sensitivity.y;
                    self.theta = math::clamp(self.theta, -89.0_f32.to_radians(), 89.0_f32.to_radians());
                }
            }
        }
    }
}
