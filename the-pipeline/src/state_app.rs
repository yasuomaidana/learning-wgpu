use crate::state::State;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};
use crate::event_handler::event_handler::EventHandler;

pub struct StateApplication<'a> {
    state: Option<State<'a>>,
    event_handler: EventHandler,
}

impl<'a> StateApplication<'a> {
    pub fn new() -> StateApplication<'a> {
        StateApplication {
            state: None,
            event_handler: EventHandler::new(),
        }
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
        let read_input = self.event_handler.input(event.clone());
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
                    self.state.as_mut().unwrap().render().unwrap();
                }
                _ => {}
            }
        }
        let current_stored = self.event_handler.get_current_event();

        if let Some(current) = current_stored {
            let redraw = self.state.as_mut().unwrap().input(current);
            if redraw {
                self.state.as_mut().unwrap().update();
            }
        }

        if read_input {
            self.event_handler.clear();
        }
    }
}
