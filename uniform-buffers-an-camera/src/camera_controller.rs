use crate::camera::Camera;
use crate::keyboard_handler::Action;
use cgmath::{InnerSpace, Vector3};

pub struct CameraController {
    pub(crate) velocity: f32,
    pub(crate) acceleration: f32,
}

impl CameraController {
    pub fn new(velocity: f32, acceleration: f32) -> Self {
        CameraController {
            velocity,
            acceleration,
        }
    }
}

fn get_forward_vector(camera: &Camera) -> Vector3<f32> {
    let target = camera.target;
    let position = camera.eye;
    target - position
}

impl CameraController {
    pub fn update(&mut self, input: Action, camera: &mut Camera) {
        match input {
            Action::Up => self.radial_movement(camera, true),
            Action::Down => self.radial_movement(camera, false),
            Action::Left => self.tangential_movement(camera, false),
            Action::Right => self.tangential_movement(camera, true),
            Action::Accelerate => self.velocity += self.acceleration,
            Action::Decelerate => {
                if self.velocity > 0.0 {
                    self.velocity -= self.acceleration;
                }
            }
            _ => {}
        }
    }
    fn radial_movement(&self, camera: &mut Camera, forward: bool) {
        let f = get_forward_vector(camera);
        let f_norm = f.normalize();
        let displacement = if forward && f.magnitude() > self.velocity {
            f_norm * self.velocity
        } else {
            -f_norm * self.velocity
        };
        camera.eye += displacement;
    }
    fn tangential_movement(&self, camera: &mut Camera, forward: bool) {
        let f = get_forward_vector(camera);
        let s = f.cross(camera.up).normalize();

        let displacement_direction = if forward {
            f + s * self.velocity
        } else {
            f - s * self.velocity
        };

        let displacement = displacement_direction.normalize() * f.magnitude();
        camera.eye = camera.target - displacement;
    }
}
