use crate::state::State;
use common::key_q_s_handler::qs_state_app::SQStateApplication;
use common::run;

mod state;

mod vertex_layout;

fn main() {
    pollster::block_on(run(SQStateApplication::<State>::new(
        "Buffer and Indices".to_string(),
    )));
}
