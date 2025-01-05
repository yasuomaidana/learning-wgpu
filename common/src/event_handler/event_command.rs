use winit::event::WindowEvent;

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
        comparator: fn(&WindowEvent, &WindowEvent) -> Option<bool>,
    ) -> Option<bool> {
        let equal_chain = self
            .event_chain
            .iter()
            .zip(read_commands.iter())
            .find_map(|(command, read_command)| comparator(command, read_command));
        
        match equal_chain {
            None => None,
            Some(false) => Some(false),
            Some(true) => match &self.escape_event {
                None => Some(true),
                Some(escape_event) => comparator(&escape_event, current_command),
            },
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::event_handler::event_command::EventCommand;
    use winit::event::{DeviceId, ElementState, MouseButton, WindowEvent};

    fn compare_events(command_1: &WindowEvent, command_2: &WindowEvent) -> Option<bool> {
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
                } => Some(button_1 == button_2 && state_1 == state_2),
                _ => None,
            },
            _ => None,
        }
    }

    #[test]
    fn test_click_command() {
        let command = EventCommand::new(
            vec![WindowEvent::MouseInput {
                button: MouseButton::Left,
                state: ElementState::Pressed,
                device_id: DeviceId::dummy(),
            }],
            Some(WindowEvent::MouseInput {
                button: MouseButton::Left,
                state: ElementState::Released,
                device_id: DeviceId::dummy(),
            }),
        );
        let read_commands = vec![WindowEvent::MouseInput {
            button: MouseButton::Left,
            state: ElementState::Pressed,
            device_id: DeviceId::dummy(),
        }];
        let current_command = WindowEvent::MouseInput {
            button: MouseButton::Left,
            state: ElementState::Released,
            device_id: DeviceId::dummy(),
        };
        let result = command.compare(&read_commands, &current_command, compare_events);
        assert!(result.unwrap());
    }
}
