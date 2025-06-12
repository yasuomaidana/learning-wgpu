use common::event_handler::event::InputEventTrait;
use common::event_handler::keyboard_handler::keyboard_input::SingleKeyboardInput;
use common::keyboard_handler::{DefaultKeyboardHandlerMethods, KeysHandler};
use keyboard_handler::DefaultKeyboardHandler;

#[derive(Debug)]
pub enum Action {
    Quit,
    Up,
    Down,
    Left,
    Right,
    Accelerate,
    Decelerate,
}

#[derive(DefaultKeyboardHandler)]
#[action_type(Action)]
pub struct CameraKeyboardHandler {
    actions: Vec<SingleKeyboardInput>,
}

impl KeysHandler for CameraKeyboardHandler {
    type Action = Action;

    fn new() -> Self {
        CameraKeyboardHandler {
            actions: vec![
                SingleKeyboardInput::new("Q".to_string()),
                SingleKeyboardInput::new("W".to_string()),
                SingleKeyboardInput::new("S".to_string()),
                SingleKeyboardInput::new("A".to_string()),
                SingleKeyboardInput::new("D".to_string()),
                SingleKeyboardInput::new("Z".to_string()),
                SingleKeyboardInput::new("X".to_string()),
            ],
        }
    }

    fn get_action(&mut self) -> Option<<Self as KeysHandler>::Action> {
        match self.actions.iter().find(|action| action.in_progress()) {
            None => None,
            Some(action) => match action.key.as_str() {
                "q" => Some(Action::Quit),
                "w" => Some(Action::Up),
                "s" => Some(Action::Down),
                "a" => Some(Action::Left),
                "d" => Some(Action::Right),
                "z" => Some(Action::Accelerate),
                "x" => Some(Action::Decelerate),
                _ => None,
            },
        }
    }
}
