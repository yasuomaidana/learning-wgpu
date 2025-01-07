use winit::event::{ElementState, MouseButton, WindowEvent};

#[derive(Clone, Debug)]
pub struct EventCommand {
    event_chain: Vec<WindowEvent>,
    escape_event: Option<WindowEvent>,
}

impl EventCommand {
    pub fn new(event_chain: Vec<WindowEvent>, escape_event: Option<WindowEvent>) -> Self {
        EventCommand {
            event_chain,
            escape_event,
        }
    }

    pub fn equal(
        &self,
        other: &EventCommand,
        comparator: fn(&WindowEvent, &WindowEvent) -> bool,
    ) -> bool {
        let other_commands = &other.event_chain;
        let other_escape = other.escape_event.as_ref();
        self.compare(other_commands, other_escape, comparator)
            .unwrap_or(false)
    }

    pub fn partial_equal(
        &self,
        commands: &Vec<WindowEvent>,
        escape_event: &Option<WindowEvent>,
        comparator: fn(&WindowEvent, &WindowEvent) -> bool,
    ) -> bool {
        match escape_event {
            None => match self.event_chain.len() == commands.len() {
                true => self
                    .event_chain
                    .iter()
                    .zip(commands.iter())
                    .all(|(a, b)| comparator(a, b)),
                false => false,
            },
            Some(_) => false,
        }
    }

    pub fn set_last_event(&mut self, last_event: WindowEvent) {
        let last_index = &self.event_chain.len() - 1;
        self.event_chain[last_index] = last_event;
    }

    pub fn get_last_event(&self) -> Option<&WindowEvent> {
        self.event_chain.last()
    }

    pub fn compare(
        &self,
        read_commands: &Vec<WindowEvent>,
        current_command: Option<&WindowEvent>,
        comparator: fn(&WindowEvent, &WindowEvent) -> bool,
    ) -> Option<bool> {
        let mut equal_chain = None;

        if read_commands.len() > 0 && read_commands.len() < self.event_chain.len() {
            if read_commands
                .iter()
                .zip(self.event_chain.iter())
                .all(|(a, b)| comparator(a, b))
            {
                equal_chain = Some(false);
                return equal_chain;
            }
        } else if read_commands.len() == self.event_chain.len() {
            if read_commands
                .iter()
                .zip(self.event_chain.iter())
                .all(|(a, b)| comparator(a, b))
            {
                equal_chain = Some(true);
            }
        }

        match equal_chain {
            None => None,
            Some(false) => Some(false),
            Some(true) => match &self.escape_event {
                None => Some(true),
                Some(escape_event) => match current_command {
                    None => Some(false),
                    Some(current_command) => match comparator(escape_event, current_command) {
                        true => Some(true),
                        false => None,
                    },
                },
            },
        }
    }
}

pub fn mouse_button_event_generator(button: MouseButton, state: ElementState) -> WindowEvent {
    WindowEvent::MouseInput {
        button,
        state,
        device_id: winit::event::DeviceId::dummy(),
    }
}

pub fn pressure_event_generator() -> WindowEvent {
    WindowEvent::TouchpadPressure {
        device_id: winit::event::DeviceId::dummy(),
        pressure: 0.0,
        stage: 0,
    }
}

pub fn default_compare_events(command_1: &WindowEvent, command_2: &WindowEvent) -> bool {
    match command_1 {
        WindowEvent::MouseInput {
            button: button_1,
            state: state_1,
            ..
        } => match command_2 {
            WindowEvent::MouseInput {
                button: button_2,
                state: state_2,
                ..
            } => button_1 == button_2 && state_1 == state_2,
            _ => false,
        },
        WindowEvent::TouchpadPressure { .. } => match command_2 {
            WindowEvent::TouchpadPressure { .. } => true,
            _ => false,
        },
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use crate::event_handler::event_command::{
        default_compare_events, mouse_button_event_generator, EventCommand,
    };
    use winit::event::{ElementState, MouseButton};

    #[test]
    fn test_click_command() {
        let command = EventCommand::new(
            vec![mouse_button_event_generator(
                MouseButton::Left,
                ElementState::Pressed,
            )],
            Some(mouse_button_event_generator(
                MouseButton::Left,
                ElementState::Released,
            )),
        );
        let read_commands = vec![mouse_button_event_generator(
            MouseButton::Left,
            ElementState::Pressed,
        )];
        let current_command =
            mouse_button_event_generator(MouseButton::Left, ElementState::Released);
        let result = command.compare(
            &read_commands,
            Some(&current_command),
            default_compare_events,
        );
        assert!(result.unwrap());
    }

    #[test]
    fn test_incomplete_command_comparison() {
        let command = EventCommand::new(
            vec![
                mouse_button_event_generator(MouseButton::Left, ElementState::Pressed),
                mouse_button_event_generator(MouseButton::Right, ElementState::Pressed),
            ],
            Some(mouse_button_event_generator(
                MouseButton::Left,
                ElementState::Released,
            )),
        );
        let read_commands = vec![mouse_button_event_generator(
            MouseButton::Left,
            ElementState::Pressed,
        )];
        let current_command =
            mouse_button_event_generator(MouseButton::Left, ElementState::Released);
        let result = command.compare(
            &read_commands,
            Some(&current_command),
            default_compare_events,
        );
        assert!(!result.unwrap());
    }

    #[test]
    fn test_different_command_comparison() {
        let command = EventCommand::new(
            vec![
                mouse_button_event_generator(MouseButton::Left, ElementState::Pressed),
                mouse_button_event_generator(MouseButton::Right, ElementState::Pressed),
            ],
            Some(mouse_button_event_generator(
                MouseButton::Left,
                ElementState::Released,
            )),
        );
        let read_commands = vec![
            mouse_button_event_generator(MouseButton::Left, ElementState::Pressed),
            mouse_button_event_generator(MouseButton::Middle, ElementState::Pressed),
        ];
        let current_command =
            mouse_button_event_generator(MouseButton::Left, ElementState::Released);
        let result = command.compare(
            &read_commands,
            Some(&current_command),
            default_compare_events,
        );
        assert!(result.is_none());
    }

    #[test]
    fn test_non_related_command() {
        let command = EventCommand::new(
            vec![
                mouse_button_event_generator(MouseButton::Left, ElementState::Pressed),
                mouse_button_event_generator(MouseButton::Right, ElementState::Pressed),
            ],
            Some(mouse_button_event_generator(
                MouseButton::Left,
                ElementState::Released,
            )),
        );
        let read_commands = vec![mouse_button_event_generator(
            MouseButton::Middle,
            ElementState::Pressed,
        )];
        let current_command =
            mouse_button_event_generator(MouseButton::Left, ElementState::Released);
        let result = command.compare(
            &read_commands,
            Some(&current_command),
            default_compare_events,
        );
        assert!(result.is_none());
    }
}
