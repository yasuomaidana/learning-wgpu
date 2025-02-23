use common::run;
use crate::state_app::StateApplication;

mod lib;
mod state;
mod state_app;

fn main() {
    pollster::block_on(run(StateApplication::new()));
}
