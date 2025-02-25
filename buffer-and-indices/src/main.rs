use crate::state_app::StateApplication;
use common::run;

mod keyboard_handler;
mod state;
mod state_app;
mod vertex_layout;

fn main() {
    pollster::block_on(run(StateApplication::new()));
}
