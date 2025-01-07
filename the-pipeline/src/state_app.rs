use crate::state::State;
use common::event_handler::event_command::{
    default_compare_events, mouse_button_event_generator, pressure_event_generator, EventCommand,
};
use common::event_handler::event_handler::EventHandler;
use winit::application::ApplicationHandler;
use winit::event::{ElementState, MouseButton, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};

pub struct StateApplication<'a> {
    state: Option<State<'a>>,
    event_handler: EventHandler,
}

enum PipelineCommands {
    LeftClickCommand(EventCommand),
}

fn get_pipeline_command(event_command: Option<EventCommand>) -> Option<PipelineCommands> {
    let left_pressure_command = EventCommand::new(
        vec![
            mouse_button_event_generator(MouseButton::Left, ElementState::Pressed),
            pressure_event_generator(),
        ],
        Some(mouse_button_event_generator(
            MouseButton::Left,
            ElementState::Released,
        )),
    );
    match event_command {
        None => None,
        Some(event_command) => match event_command {
            event_command
                if event_command.equal(&left_pressure_command, default_compare_events) =>
            {
                Some(PipelineCommands::LeftClickCommand(event_command))
            }
            _ => None,
        },
    }
}

impl<'a> StateApplication<'a> {
    pub fn new() -> StateApplication<'a> {
        StateApplication {
            state: None,
            event_handler: EventHandler::new(
                vec![EventCommand::new(
                    vec![
                        mouse_button_event_generator(MouseButton::Left, ElementState::Pressed),
                        pressure_event_generator(),
                    ],
                    Some(mouse_button_event_generator(
                        MouseButton::Left,
                        ElementState::Released,
                    )),
                )],
                None,
            ),
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
        let read_input = self.event_handler.input(event.clone());
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

        let current_stored = get_pipeline_command(self.event_handler.get_partial_command());

        if let Some(current) = current_stored {
            match current {
                PipelineCommands::LeftClickCommand(event_command) => {
                    let left_pressure = event_command.get_last_event().unwrap();
                    let redraw = self.state.as_mut().unwrap().input(&left_pressure);
                    if redraw {
                        self.state.as_mut().unwrap().update();
                    }
                }
            }
        }

        if read_input.map(|x| x == true).unwrap_or(false) {
            self.event_handler.clear();
        }
    }
}
