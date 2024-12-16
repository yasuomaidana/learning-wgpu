use crate::state_app::StateApplication;

mod state;
mod state_app;

pub async fn run(){
    let event_loop = winit::event_loop::EventLoop::new();
    let mut window_state = StateApplication::new();
    let _ = event_loop.unwrap().run_app(&mut window_state);
}

fn main() {
    pollster::block_on(run());
}
