mod state;
mod vertex_layout;
mod state_app;
mod keyboard_handler;

use common::run;
use utils::resource_manager::{check_resource, download_resource};
use crate::state_app::StateApplication;

fn main() {
    let resource_path = "./textures-and-bind-groups/src/happy-tree.png";
    if !check_resource(resource_path) {
        download_resource("https://sotrh.github.io/learn-wgpu/assets/img/happy-tree.bdff8a19.png", resource_path).expect("Failed to download resource");
    }
    pollster::block_on(run(StateApplication::new()));
}
