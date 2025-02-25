use common::run;
use crate::state_app::StateApplication;

mod lib;
mod state;
mod state_app;
mod keyboard_handler;

fn main() {
    pollster::block_on(run(StateApplication::new()));
}
