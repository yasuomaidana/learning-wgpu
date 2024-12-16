use crate::state::State;
use winit::application::ApplicationHandler;
use winit::event::{MouseButton, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};

pub struct StateApplication<'a> {
    state: Option<State<'a>>,
}

impl<'a> StateApplication<'a> {
    pub fn new() -> StateApplication<'a> {
        StateApplication { state: None }
    }
}

impl ApplicationHandler for StateApplication<'_> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(Window::default_attributes().with_title("Hello, World!"))
            .expect("Failed to create window");
        self.state = Some(State::new(window));
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        
        let read_input = self.state.as_mut().unwrap().input(&event);
        let window = self.state.as_ref().unwrap().window();
        
        if window.id() == window_id && !read_input {
            match event {
                WindowEvent::CloseRequested => {
                    event_loop.exit();
                }
                WindowEvent::Resized(physical_size) => {
                    self.state.as_mut().unwrap().resize(physical_size);
                }
                WindowEvent::RedrawRequested => {
                    self.state.as_mut().unwrap().update();
                    self.state.as_mut().unwrap().render().unwrap();
                }
                WindowEvent::MouseInput { button, .. } => {
                    match button {
                        MouseButton::Left => {
                            println!("Left mouse button clicked!");
                        }
                        MouseButton::Right => {
                            println!("Right mouse button clicked!");
                        }
                        // MouseButton::Middle => {}
                        // MouseButton::Back => {}
                        // MouseButton::Forward => {}
                        MouseButton::Other(_) => {}
                        _ => {}
                    } 
                }
                _ => {}
            }
            
        }
        if read_input { 
            self.state.as_mut().unwrap().update();
            self.state.as_mut().unwrap().render().unwrap();
        } 
    }
}
