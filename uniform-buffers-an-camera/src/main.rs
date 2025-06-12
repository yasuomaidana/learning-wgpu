use crate::camera_app::CameraStateApplication;
use common::run;

mod camera;
mod camera_app;
mod camera_controller;
mod keyboard_handler;
mod state;

fn main() {
    pollster::block_on(run(CameraStateApplication::new(
        "Uniform Buffers and 3D Camera".to_string(),
    )));
}
