mod app;

use crate::app::App;
use winit::event_loop::EventLoop;

pub async fn run() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut app = App::default();
    let _ = event_loop.unwrap().run_app(&mut app);
}

fn main() {
    pollster::block_on(run());
}
