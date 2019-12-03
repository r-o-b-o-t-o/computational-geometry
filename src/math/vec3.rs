#[derive(Copy, Clone, Debug)]
pub struct Vec3(pub cgmath::Vector3::<f32>);

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            0: cgmath::vec3(x, y, z)
        }
    }
}

unsafe impl glium::vertex::Attribute for Vec3 {
    fn get_type() -> glium::vertex::AttributeType {
        glium::vertex::AttributeType::F32F32F32
    }
}

impl super::ToArray for Vec3 {
    type Output = [f32; 3];
    fn to_array(&self) -> Self::Output {
        [ self.0.x, self.0.y, self.0.z ]
    }
}

impl Into<Vec3> for cgmath::Vector3<f32> {
    fn into(self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }
}
