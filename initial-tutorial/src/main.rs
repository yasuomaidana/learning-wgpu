mod app;

use winit::event_loop::EventLoop;
use crate::app::App;

pub async fn run() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut app = App::default();
    let _ = event_loop.unwrap().run_app(&mut app);
}

fn main() {
    pollster::block_on(run());
}
