use winit::event::WindowEvent;

#[derive(Clone, Debug, Default)]
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

    pub fn compare(&self, read_commands: &Vec<WindowEvent>, current_command: &WindowEvent) -> bool {
        if self.event_chain.len() != read_commands.len() { 
            return false;
        }
        let equal_chain = self
            .event_chain
            .iter()
            .zip(read_commands.iter())
            .all(|(command_1, command_2)| compare_events(command_1, command_2));
        match &self.escape_event { 
            Some(escape_event) => equal_chain && compare_events(escape_event, current_command),
            None => equal_chain,
        }
    }
}

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

#[cfg(test)]
mod tests {
    
    use winit::event::{DeviceId, ElementState, MouseButton, WindowEvent};
    use crate::event_handler::event_command::EventCommand;

    #[test]
    fn test_click_command() {
        let command = EventCommand::new(
            vec![WindowEvent::MouseInput {
                button: MouseButton::Left,
                state: ElementState::Pressed,
                device_id: DeviceId::dummy()
            }],
            Some(WindowEvent::MouseInput {
                button: MouseButton::Left,
                state: ElementState::Released,
                device_id: DeviceId::dummy()
            }),
        );
        let read_commands = vec![WindowEvent::MouseInput {
            button: MouseButton::Left,
            state: ElementState::Pressed,
            device_id: DeviceId::dummy()
        }];
        let current_command = WindowEvent::MouseInput {
            button: MouseButton::Left,
            state: ElementState::Released,
            device_id: DeviceId::dummy()
        };
        assert!(command.compare(&read_commands, &current_command));
    }
}