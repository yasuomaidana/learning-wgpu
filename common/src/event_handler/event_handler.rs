use crate::event_handler::event_command::{default_compare_events, EventCommand};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use winit::event::WindowEvent;

#[derive(Debug, Clone)]
pub struct EventHandler<T> {
    supported_commands: Vec<EventCommand>,
    compare_events: fn(&T, &T) -> bool,
    accumulated_events: Vec<WindowEvent>,
    current_event: Option<T>,
}

impl<T: Clone> EventHandler<T> {
    pub fn new(
        supported_commands: Vec<EventCommand>,
        compare_events: Option<fn(&T, &T) -> bool>,
    ) -> Self {
        EventHandler {
            supported_commands,
            compare_events: compare_events.unwrap_or(default_compare_events),
            accumulated_events: Vec::new(),
            current_event: None,
        }
    }
    pub fn clear(&mut self) {
        self.accumulated_events.clear();
        self.current_event = None;
    }

    pub fn get_current_event(&self) -> Option<&T> {
        self.current_event.as_ref()
    }

    pub fn input(&mut self, event: WindowEvent) -> Option<bool> {
        let likely_commands =
            self.supported_commands.par_iter().filter_map(|command| {
                match command.compare(&self.accumulated_events, &event, self.compare_events) {
                    Some(_) => Some(command),
                    None => None,
                }
            });

        None
    }
}
