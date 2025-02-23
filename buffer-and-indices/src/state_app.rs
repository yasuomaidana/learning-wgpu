use crate::state::State;

// use crate::event_handler::EventHandler;
use common::event_handler::button_click::button_click::{ButtonClickEvent, ButtonEvent};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};

pub struct StateApplication<'a> {
    state: Option<State<'a>>,
    // event_handler: EventHandler,
}

impl<'a> StateApplication<'a> {
    pub fn new() -> StateApplication<'a> {
        StateApplication {
            state: None,
            // event_handler: EventHandler::new(),
        }
    }
}

impl ApplicationHandler for StateApplication<'_> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(Window::default_attributes().with_title("The pipeline"))
            .expect("Failed to create window");
        self.state = Some(State::new(window));
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        // let read_input = self.event_handler.input(event.clone());
        let read_input: Option<bool> = None;
        let window = self.state.as_ref().unwrap().window();

        if window.id() == window_id && read_input.is_none() {
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
        // if let Some(button_event) = read_input {
        //     
        // }
    }
}
