use crate::camera::Camera;
use crate::keyboard_handler::Action;
use cgmath::{InnerSpace, Vector3};

/// Controls the camera movement by managing velocity and acceleration.
///
/// - `velocity`: The current speed of the camera.
/// - `acceleration`: The rate at which the camera's velocity changes.
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

/// Returns the forward vector of the camera, calculated as the difference
/// between the camera's target and eye (position).
fn get_forward_vector(camera: &Camera) -> Vector3<f32> {
    let target = camera.target;
    let position = camera.eye;
    target - position
}

impl CameraController {
    /// Updates the camera's position and velocity based on the given input action.
    ///
    /// # Arguments
    ///
    /// * `input` - The action to process (e.g., movement or speed change).
    /// * `camera` - The mutable reference to the camera to be updated.
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

    /// Moves the camera radially along its forward vector.
    ///
    /// # Arguments
    ///
    /// * `camera` - The mutable reference to the camera to update.
    /// * `forward` - If true, moves the camera forward; otherwise, moves backward.
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

    /// Moves the camera tangentially around the target, simulating left/right orbiting.
    ///
    /// # Arguments
    ///
    /// * `camera` - The mutable reference to the camera to update.
    /// * `forward` - If true, orbits right; otherwise, orbits left.
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
