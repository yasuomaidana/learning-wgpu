use crate::event_handler::event::InputEventTrait;
use winit::event::{KeyEvent, WindowEvent};

pub struct SingleKeyboardInput {
    pub key: String,
    started: bool,
    finished: bool,
}

impl SingleKeyboardInput {
    pub fn new(key: String) -> Self {
        let key = key.to_lowercase();
        SingleKeyboardInput {
            key,
            started: false,
            finished: false,
        }
    }
}

impl InputEventTrait<'_, String> for SingleKeyboardInput {
    fn finished(&self) -> bool {
        self.finished
    }

    fn update(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::KeyboardInput { event, .. } => {
                let key = event
                    .text
                    .as_ref()
                    .map(|c| c.to_string())
                    .unwrap_or("".to_string()).to_lowercase();
                
                if key == self.key {
                    match event {
                        KeyEvent { state, .. } => match state {
                            winit::event::ElementState::Pressed => {
                                self.started = true;
                            }
                            winit::event::ElementState::Released => {
                                self.finished = true;
                            }
                        },
                    }
                }
            }
            _ => {}
        }
    }

    fn in_progress(&self) -> bool {
        self.started && !self.finished
    }

    fn get_event(&self) -> Result<Option<String>, String> {
        if !self.started {
            Ok(None)
        } else {
            Ok(Some(self.key.clone()))
        }
    }

    fn clear(&mut self) {
        self.started = false;
        self.finished = false;
    }
}
