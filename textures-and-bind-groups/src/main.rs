mod state;

use crate::state::State;
use common::key_q_s_handler::qs_state_app::SQStateApplication;
use common::run;
use utils::resource_manager::{check_resource, download_resource};

fn main() {
    let resource_path = "./textures-and-bind-groups/src/happy-tree.png";
    if !check_resource(resource_path) {
        download_resource(
            "https://sotrh.github.io/learn-wgpu/assets/img/happy-tree.bdff8a19.png",
            resource_path,
        )
        .expect("Failed to download resource");
    }

    pollster::block_on(run(SQStateApplication::<State>::new(
        "Textures and Bind Groups".to_string(),
    )));
}
