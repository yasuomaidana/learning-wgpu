use crate::state::State;

use crate::keyboard_handler::{Action, KeyboardHandler};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};

pub struct StateApplication<'a> {
    state: Option<State<'a>>,
    event_handler: KeyboardHandler,
}

impl<'a> StateApplication<'a> {
    pub fn new() -> StateApplication<'a> {
        StateApplication {
            state: None,
            event_handler: KeyboardHandler::new(),
        }
    }
}

impl ApplicationHandler for StateApplication<'_> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(Window::default_attributes().with_title("Buffer and indices"))
            .expect("Failed to create window");
        self.state = Some(State::new(window));
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        self.event_handler.input(&event);
        let read_input = self.event_handler.get_action();
        let window = self.state.as_ref().unwrap().window();
        let quit = read_input.as_ref().map_or(false, |action| match action {
            Action::Quit => true,
            _ => false,
        });
        if window.id() == window_id || quit {
            if quit {
                event_loop.exit();
            }
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
    }
}
