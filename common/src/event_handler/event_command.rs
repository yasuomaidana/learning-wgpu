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

    pub fn compare(
        &self,
        read_commands: &Vec<WindowEvent>,
        current_command: &WindowEvent,
        comparator: fn(&WindowEvent, &WindowEvent) -> bool,
    ) -> Option<bool> {
        let mut equal_chain = None;

        for (i, chain_event) in self.event_chain.iter().enumerate() {
            if i >= read_commands.len() {
                break;
            }
            if comparator(chain_event, &read_commands[i]) {
                equal_chain = Some(true);
            } else {
                if equal_chain == Some(true) {
                    equal_chain = Some(false);
                } else {
                    equal_chain = None;
                }
                break;
            }
        }

        match equal_chain {
            None => None,
            Some(false) => Some(false),
            Some(true) => match &self.escape_event {
                None => Some(self.event_chain.len() == read_commands.len()),
                Some(escape_event) => Some(
                    comparator(escape_event, current_command)
                        && self.event_chain.len() == read_commands.len(),
                ),
            },
        }
    }
}

fn mouse_button_event_generator(button: MouseButton, state: ElementState) -> WindowEvent {
    WindowEvent::MouseInput {
        button,
        state,
        device_id: winit::event::DeviceId::dummy(),
    }
}

#[cfg(test)]
mod tests {

    use crate::event_handler::event_command::{mouse_button_event_generator, EventCommand};
    use winit::event::{ElementState, MouseButton, WindowEvent};

    fn compare_events(command_1: &WindowEvent, command_2: &WindowEvent) -> bool {
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
            _ => false,
        }
    }

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
        let result = command.compare(&read_commands, &current_command, compare_events);
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
        let result = command.compare(&read_commands, &current_command, compare_events);
        assert!(!result.unwrap());
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
        let result = command.compare(&read_commands, &current_command, compare_events);
        assert!(result.is_none());
    }
}
