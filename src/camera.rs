use cgmath::prelude::*;
use cgmath::vec3;
use cgmath::Matrix4;
use cgmath::Point3;
use cgmath::Vector3;

pub struct Camera {
    position: Point3<f32>,
    front: Vector3<f32>,
    up: Vector3<f32>,
    pub field_of_view: f32,
    yaw: f32,
    pitch: f32,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            position: Point3::new(0., 0., 3.),
            front: vec3(0., 0., -1.),
            up: vec3(0., 1., 0.),
            field_of_view: 45.,
            yaw: -89.,
            pitch: 0.,
        }
    }
    pub fn move_forward(&mut self, speed: f32) {
        self.position += speed * self.front
    }

    pub fn move_right(&mut self, speed: f32) {
        self.position += self.front.cross(self.up).normalize() * speed;
    }

    pub fn zoom_in(&mut self, speed: f32) {
        self.field_of_view = (self.field_of_view - speed).min(45.).max(1.);
    }

    pub fn rotate(&mut self, xoffset: f32, yoffset: f32) {
        self.yaw += xoffset;
        self.pitch = (self.pitch + yoffset).min(89.).max(-89.);

        self.front = vec3(
            self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            self.pitch.to_radians().sin(),
            self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
        )
        .normalize();
    }

    pub fn view(&self) -> Matrix4<f32> {
        Matrix4::look_at(self.position, self.position + self.front, self.up)
    }
}
