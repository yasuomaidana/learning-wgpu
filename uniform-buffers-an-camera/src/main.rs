use common::key_q_s_handler::qs_state_app::SQStateApplication;
use common::run;
use crate::state::State;

mod camera;
mod state;

fn main() {
    pollster::block_on(run(SQStateApplication::<State>::new(
        "Uniform Buffers and 3D Camera".to_string(),
    )));
}
