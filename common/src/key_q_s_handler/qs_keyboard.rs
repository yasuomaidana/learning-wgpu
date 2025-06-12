use crate::event_handler::event::InputEventTrait;
use crate::event_handler::keyboard_handler::keyboard_input::SingleKeyboardInput;
use crate::keyboard_handler::{DefaultKeyboardHandlerMethods, KeysHandler};
use keyboard_handler::DefaultKeyboardHandler;

#[derive(Debug)]
pub enum Action {
    Quit,
    ChangeFigure,
}

#[derive(DefaultKeyboardHandler)]
#[action_type(Action)]
pub struct QSKeyboardHandler {
    actions: Vec<SingleKeyboardInput>,
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
