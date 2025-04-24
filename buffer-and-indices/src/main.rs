use common::key_q_s_handler::qs_state_app::SQStateApplication;
use common::run;
use crate::state::State;

mod state;

mod vertex_layout;

fn main() {
    pollster::block_on(run(SQStateApplication::<State>::new()));
}
