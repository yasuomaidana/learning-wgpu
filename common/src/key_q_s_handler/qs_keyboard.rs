use crate::event_handler::event::InputEventTrait;
use crate::event_handler::keyboard_handler::keyboard_input::SingleKeyboardInput;
use crate::key_q_s_handler::keyboard_handler::{DefaultKeyboardHandlerMethods, KeysHandler};

#[derive(Debug)]
pub enum Action {
    Quit,
    ChangeFigure,
}

pub struct QSKeyboardHandler {
    actions: Vec<SingleKeyboardInput>,
}

impl DefaultKeyboardHandlerMethods for QSKeyboardHandler {
    type Action = Action;

    fn input(&mut self, event: &winit::event::WindowEvent) {
        self.actions
            .iter_mut()
            .for_each(|action| action.update(event));
    }

    fn clear(&mut self) {
        self.actions.iter_mut().for_each(|action| action.clear());
    }
}

impl KeysHandler for QSKeyboardHandler {
    type Action = Action;
    fn new() -> QSKeyboardHandler {
        QSKeyboardHandler {
            actions: vec![
                SingleKeyboardInput::new("Q".to_string()),
                SingleKeyboardInput::new("S".to_string()),
            ],
        }
    }

    fn get_action(&mut self) -> Option<Action> {
        match self.actions.iter().find(|action| action.in_progress()) {
            None => None,
            Some(action) => match action.key.as_str() {
                "q" => Some(Action::Quit),
                "s" => Some(Action::ChangeFigure),
                _ => None,
            },
        }
    }
}
