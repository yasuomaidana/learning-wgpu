use crate::event_handler::event_command::{default_compare_events, EventCommand};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use winit::event::WindowEvent;

#[derive(Debug, Clone)]
pub struct EventHandler {
    supported_commands: Vec<EventCommand>,
    compare_events: fn(&WindowEvent, &WindowEvent) -> bool,
    accumulated_events: Vec<WindowEvent>,
    current_command: Option<EventCommand>,
    last_event: Option<WindowEvent>,
}

impl EventHandler {
    pub fn new(
        supported_commands: Vec<EventCommand>,
        compare_events: Option<fn(&WindowEvent, &WindowEvent) -> bool>,
    ) -> Self {
        EventHandler {
            supported_commands,
            compare_events: compare_events.unwrap_or(default_compare_events),
            accumulated_events: Vec::new(),
            current_command: None,
            last_event: None,
        }
    }
    pub fn clear(&mut self) {
        self.accumulated_events.clear();
        self.current_command = None;
    }

    pub fn get_partial_command(&self) -> Option<EventCommand> {
        let last_event = match &self.last_event {
            Some(event) => match self.accumulated_events.last() {
                None => None,
                Some(last_event) => {
                    if !(self.compare_events)(last_event, event) {
                        Some(event.clone())
                    } else {
                        None
                    }
                }
            },
            None => return None,
        };

        let base_command = self
            .supported_commands
            .par_iter()
            .find_any(|command| {
                command.partial_equal(&self.accumulated_events, &last_event, self.compare_events)
            })
            .cloned();
        match base_command {
            Some(mut command) => {
                command.set_last_event(self.last_event.clone()?);
                Some(command)
            }
            None => None,
        }
    }

    pub fn get_current_command(&self) -> Option<EventCommand> {
        self.current_command.clone()
    }

    fn likely_commands(&self, escape_event: Option<&WindowEvent>) -> Vec<bool> {
        self.supported_commands
            .par_iter()
            .filter_map(|command| {
                command.compare(&self.accumulated_events, escape_event, self.compare_events)
            })
            .collect::<Vec<bool>>()
    }

    pub fn input(&mut self, event: WindowEvent) -> Option<bool> {
        let likely_commands = self.likely_commands(Some(&event));
        let partial_command = self.get_partial_command();
        if likely_commands.is_empty() && partial_command.is_none() {
            self.accumulated_events.push(event);
            if self.likely_commands(None).is_empty() {
                self.accumulated_events.clear();
                self.current_command = None;
                return None;
            }
            {
                Some(false)
            }
        } else {
            let finished = likely_commands.iter().any(|&x| x);
            if finished {
                let mut event_command = self
                    .supported_commands
                    .par_iter()
                    .find_any(|command| {
                        command.compare(&self.accumulated_events, Some(&event), self.compare_events)
                            == Some(true)
                    })
                    .unwrap()
                    .clone();
                event_command.set_last_event(self.last_event.clone()?);
                self.current_command = Some(event_command);
                Some(true)
            } else {
                let last_event = self.accumulated_events.last();

                match last_event {
                    Some(last_event) => {
                        if !(self.compare_events)(last_event, &event) {
                            self.accumulated_events.push(event.clone());
                        }
                    }
                    None => {
                        self.accumulated_events.push(event.clone());
                    }
                }
                self.last_event = Some(event.clone());
                Some(false)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::event_handler::event_command::{
        default_compare_events, mouse_button_event_generator, pressure_event_generator,
        EventCommand,
    };
    use crate::event_handler::event_handler::EventHandler;
    use winit::event::{ElementState, MouseButton, WindowEvent};

    fn create_pressure_command_handler() -> EventHandler {
        let command = EventCommand::new(
            vec![
                mouse_button_event_generator(MouseButton::Left, ElementState::Pressed),
                pressure_event_generator(),
            ],
            Some(mouse_button_event_generator(
                MouseButton::Left,
                ElementState::Released,
            )),
        );
        EventHandler::new(vec![command], None)
    }
    #[test]
    fn test_event_handler() {
        let mut event_handler = create_pressure_command_handler();
        let pressing_left_button =
            mouse_button_event_generator(MouseButton::Left, ElementState::Pressed);
        let pressing_left_button = event_handler.input(pressing_left_button);
        assert!(pressing_left_button.is_some());
        let pressure_event = WindowEvent::TouchpadPressure {
            device_id: winit::event::DeviceId::dummy(),
            pressure: 2.0,
            stage: 0,
        };
        let pressure_event = event_handler.input(pressure_event);
        assert!(pressure_event.is_some());
        assert!(!pressure_event.unwrap());
        let button_event = mouse_button_event_generator(MouseButton::Left, ElementState::Released);
        let pressing_left_button = event_handler.input(button_event);
        assert!(pressing_left_button.is_some());
        assert!(pressing_left_button.unwrap());
        let last_command = event_handler.get_current_command();
        assert!(last_command.is_some());
        let last_command = last_command.unwrap();
        let last_event = last_command.get_last_event().unwrap();
        let pressure = match last_event {
            WindowEvent::TouchpadPressure { pressure, .. } => pressure,
            _ => &0.0,
        };
        assert_eq!(pressure, &2.0);
    }

    #[test]
    fn test_incomplete_event_handler() {
        let mut event_handler = create_pressure_command_handler();
        let pressing_left_button =
            mouse_button_event_generator(MouseButton::Left, ElementState::Pressed);
        let pressing_left_button = event_handler.input(pressing_left_button);
        assert!(pressing_left_button.is_some());
        let pressure_event = WindowEvent::TouchpadPressure {
            device_id: winit::event::DeviceId::dummy(),
            pressure: 2.0,
            stage: 0,
        };
        let pressure_event = event_handler.input(pressure_event);
        assert!(pressure_event.is_some());
        assert!(!pressure_event.unwrap());
        let button_event =
            mouse_button_event_generator(MouseButton::Middle, ElementState::Released);
        let pressing_left_button = event_handler.input(button_event);
        assert!(pressing_left_button.is_none());
    }

    fn assert_updating_pressure(pressure: f32, event_handler: &mut EventHandler) {
        let pressure_event = WindowEvent::TouchpadPressure {
            device_id: winit::event::DeviceId::dummy(),
            pressure,
            stage: 0,
        };
        let pressure_event = event_handler.input(pressure_event);
        assert!(pressure_event.is_some());
        let partial_command = event_handler.get_partial_command();
        assert!(partial_command.is_some());
        let partial_command = partial_command.unwrap();
        let last_event = partial_command.get_last_event().unwrap();
        assert!(
            matches!(last_event, WindowEvent::TouchpadPressure { pressure, .. } if pressure == pressure)
        );
    }

    #[test]
    fn test_updating_value() {
        let mut event_handler = create_pressure_command_handler();
        let pressing_left_button =
            mouse_button_event_generator(MouseButton::Left, ElementState::Pressed);
        let pressing_left_button = event_handler.input(pressing_left_button);
        assert!(pressing_left_button.is_some());

        assert_updating_pressure(2.0, &mut event_handler);
        assert_updating_pressure(3.0, &mut event_handler);
    }

    #[test]
    fn test_invalid_event_handler() {
        let mut event_handler = create_pressure_command_handler();
        let button_event = mouse_button_event_generator(MouseButton::Left, ElementState::Released);
        let pressing_left_button = event_handler.input(button_event);
        assert!(pressing_left_button.is_none());
    }

    enum Command {
        LeftPressure(EventCommand),
        RightPressure(EventCommand),
    }

    #[test]
    fn test_get_enum_command() {
        let commands = vec![
            Command::LeftPressure(EventCommand::new(
                vec![
                    mouse_button_event_generator(MouseButton::Left, ElementState::Pressed),
                    pressure_event_generator(),
                ],
                Some(mouse_button_event_generator(
                    MouseButton::Left,
                    ElementState::Released,
                )),
            )),
            Command::RightPressure(EventCommand::new(
                vec![
                    mouse_button_event_generator(MouseButton::Right, ElementState::Pressed),
                    pressure_event_generator(),
                ],
                Some(mouse_button_event_generator(
                    MouseButton::Right,
                    ElementState::Released,
                )),
            )),
        ];

        let mut event_handler = EventHandler::new(
            commands
                .iter()
                .map(|command| match command {
                    Command::LeftPressure(event_command) => event_command.clone(),
                    Command::RightPressure(event_command) => event_command.clone(),
                })
                .collect(),
            None,
        );
        let pressing_left_button =
            mouse_button_event_generator(MouseButton::Left, ElementState::Pressed);

        let partial_command = event_handler.get_partial_command();
        assert!(partial_command.is_none());

        let pressing_left_button = event_handler.input(pressing_left_button);
        assert!(pressing_left_button.is_some());

        let partial_command = event_handler.get_partial_command();
        assert!(partial_command.is_none());

        let pressure_event = WindowEvent::TouchpadPressure {
            device_id: winit::event::DeviceId::dummy(),
            pressure: 2.0,
            stage: 0,
        };
        let pressure_event = event_handler.input(pressure_event);
        assert!(pressure_event.is_some());
        assert!(!pressure_event.unwrap());

        let partial_command = event_handler.get_partial_command();
        assert!(partial_command.is_some());

        let partial_command = partial_command.unwrap();
        let last_event = partial_command.get_last_event().unwrap();
        assert!(
            matches!(last_event, WindowEvent::TouchpadPressure { pressure, .. } if pressure == &2.0)
        );

        let button_event = mouse_button_event_generator(MouseButton::Left, ElementState::Released);
        let pressing_left_button = event_handler.input(button_event);
        assert!(pressing_left_button.is_some());
        assert!(pressing_left_button.unwrap());
        let last_command = event_handler.get_current_command();
        assert!(last_command.is_some());
        let last_command = last_command.unwrap();

        let last_command_enum = commands
            .iter()
            .find_map(|command| match command {
                Command::LeftPressure(event_command)
                    if event_command.equal(&last_command, default_compare_events) =>
                {
                    Some(Command::LeftPressure(last_command.clone()))
                }
                Command::RightPressure(event_command)
                    if event_command.equal(&last_command, default_compare_events) =>
                {
                    Some(Command::RightPressure(last_command.clone()))
                }
                _ => None,
            })
            .unwrap();

        assert!(matches!(last_command_enum, Command::LeftPressure(_)));

        match last_command_enum {
            Command::LeftPressure(command) => {
                let pressure = match command.get_last_event().unwrap() {
                    WindowEvent::TouchpadPressure { pressure, .. } => pressure,
                    _ => &0.0,
                };
                assert_eq!(pressure, &2.0);
            }
            Command::RightPressure(_command) => {}
        }
    }
}
