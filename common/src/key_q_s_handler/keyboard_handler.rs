use crate::event_handler::event::InputEventTrait;
use crate::event_handler::keyboard_handler::keyboard_input::SingleKeyboardInput;

#[derive(Debug)]
pub enum Action {
    Quit,
    ChangeFigure,
}

pub struct KeyboardHandler {
    actions: Vec<SingleKeyboardInput>,
}

impl KeyboardHandler {
    pub fn new() -> KeyboardHandler {
        KeyboardHandler {
            actions: vec![
                SingleKeyboardInput::new("Q".to_string()),
                SingleKeyboardInput::new("S".to_string()),
            ],
        }
    }

    pub fn input(&mut self, event: &winit::event::WindowEvent) {
        self.actions
            .iter_mut()
            .for_each(|action| action.update(event));
    }

    pub fn get_action(&mut self) -> Option<Action> {
        match self.actions.iter().find(|action| action.in_progress()) {
            None => None,
            Some(action) => match action.key.as_str() {
                "q" => Some(Action::Quit),
                "s" => Some(Action::ChangeFigure),
                _ => None,
            },
        }
    }

    pub fn clear(&mut self) {
        self.actions.iter_mut().for_each(|action| action.clear());
    }
}
