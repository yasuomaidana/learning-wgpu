use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize, event::*,
    event_loop::ActiveEventLoop,
    window::{Window, WindowId}
};

pub struct State<'a> {
    instance: wgpu::Instance,
    surface: wgpu::Surface<'a>,
}

impl<'a> State<'a> {
    pub async fn new(window: Arc<Window>) -> State<'a> {
        let instance = wgpu::Instance::default();
        let surface = instance.create_surface(Arc::clone(&window)).unwrap();

        Self {
            instance,
            surface,
        }
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        let w = new_size.width;
        let h = new_size.height;
        println!("State resize {w} {h}");
    }

    pub fn draw(&self) {
        println!("State draw")
    }
}

#[derive(Default)]
pub struct App<'a> {
    window: Option<Arc<Window>>,
    state: Option<State<'a>>,
}

impl ApplicationHandler for App<'_> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        println!("App resumed");
        if self.window.is_none() {
            let window = Arc::new(event_loop.create_window(Window::default_attributes()).unwrap());
            window.set_title("Learning WPU");
            self.window = Some(window.clone());

            let state = pollster::block_on(State::new(window.clone()));
            self.state = Some(state);
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        if id != self.window.as_ref().unwrap().id() {
            return;
        }

        match event {
            WindowEvent::CloseRequested => {
                println!("Close requested");
                event_loop.exit()
            },
            WindowEvent::Resized(physical_size) => {
                println!("Resize requested");
                self.state.as_mut().unwrap().resize(physical_size);
            },
            WindowEvent::RedrawRequested => {
                println!("Redraw requested");
                self.state.as_ref().unwrap().draw();
            },
            WindowEvent::CursorLeft { .. } => {
                println!("Cursor left");
            },
            WindowEvent::KeyboardInput { device_id, event, is_synthetic } => {

                let a= match event.text {
                    Some(a) => a.to_string(),
                    None => "".to_string(),
                };
                if a == "q" {
                    event_loop.exit();
                }
                else if a =="m" {
                    self.window.as_ref().unwrap().set_cursor_visible(false);
                    self.window.as_ref().unwrap().set_title("Mouse hidden");
                }
                else if a =="M" {
                    self.window.as_ref().unwrap().set_cursor_visible(true);
                }
                println!("Keyboard input {a}");
            },
            _ => {},
        }
    }

    fn suspended(&mut self, event_loop: &ActiveEventLoop) {

        println!("App suspended");
    }

    fn exiting(&mut self, event_loop: &ActiveEventLoop) {
        println!("App exiting");
    }

}
