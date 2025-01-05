use winit::event::WindowEvent;

#[derive(Debug, Clone, Default)]
pub struct EventHandler<T> {
    event_command: EventCommand,
    current_event: Option<T>,
}

#[derive(Clone, Debug, Default)]
pub struct EventCommand {
    event_chain: Vec<WindowEvent>,
    escape_event: Option<WindowEvent>,
}

impl EventCommand {
    pub fn new(escape_event:Option<WindowEvent>) -> Self {
        EventCommand {
            event_chain: vec![],
            escape_event
        }
    }
    pub fn add_event(&mut self, event: WindowEvent) -> Option<bool> {
        match &self.escape_event { 
            Some(escape_event) => {
                if event == *escape_event {
                    Some(true)
                } else {
                    self.event_chain.push(event);
                    None
                }
            }
            None => {
                self.event_chain.push(event);
                None
            }
        }
    }
    
}

impl<T:Clone> EventHandler<T> {
    
    pub fn new() -> Self{
        EventHandler {
            event_command: EventCommand::new(None),
            current_event: None,
        }
    }
    pub fn clear(&mut self) {
        self.event_command.event_chain.clear();
        self.current_event = None;
    }
    
    pub fn get_current_event(&self) -> Option<&T> {
        self.current_event.as_ref()
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;
    

    #[test]
    fn test_get_current_event() {
        let mut handler: EventHandler<i32> = EventHandler::new();
        handler.current_event = Some(42);
        assert_eq!(handler.get_current_event(), Some(&42));
        handler.clear();
        assert!(handler.get_current_event().is_none());
    }
    
    #[test]
    fn test_command_add_event() {
        let mut command = EventCommand::new(Some(WindowEvent::CloseRequested));
        let result = command.add_event(WindowEvent::CloseRequested);
        assert_eq!(result, Some(true));
        let result = command.add_event(WindowEvent::Resized(Default::default()));
        assert!(result.is_none());
        assert_eq!(command.event_chain.len(), 1);
    }
}