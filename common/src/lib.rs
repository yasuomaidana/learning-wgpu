use winit::application::ApplicationHandler;

pub mod event_handler;
pub mod key_q_s_handler;
pub mod pipeline_builder;
pub mod state_builder;
pub mod state_traits;
pub mod texture_builder;
pub mod vertex_layout;
pub mod keyboard_handler;

pub async fn run<T: ApplicationHandler + 'static>(app: T) {
    let event_loop = winit::event_loop::EventLoop::new();
    let mut window_state = app;
    let _ = event_loop.unwrap().run_app(&mut window_state);
}
