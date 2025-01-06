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
}
