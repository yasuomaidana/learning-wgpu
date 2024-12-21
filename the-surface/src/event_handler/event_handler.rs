use winit::event::{MouseButton, WindowEvent};

#[derive(Debug, Clone, Default)]
pub struct EventHandler {
    previous_event: Option<WindowEvent>,
    current_event: Option<WindowEvent>,
}

impl EventHandler {
    pub fn new() -> EventHandler {
        EventHandler {
            previous_event: None,
            current_event: None,
        }
    }

    pub fn input(&mut self, event: WindowEvent) -> bool {
        match &self.previous_event {
            None => {
                match event {
                    WindowEvent::MouseInput { button, .. } => match button {
                        MouseButton::Left => {
                            self.previous_event = Some(event);
                            println!("Left mouse button clicked in!");
                        }
                        MouseButton::Right => {}
                        _ => {}
                    },
                    _ => {}
                }
                false
            }
            Some(previous) => match previous {
                // Previous event was a mouse input event
                WindowEvent::MouseInput {
                    button: prev_mouse_input,
                    ..
                } => match prev_mouse_input {
                    // Previous event was a left mouse button click
                    MouseButton::Left => match event {
                        WindowEvent::TouchpadPressure { pressure, .. } => {
                            if pressure > 0.0 {
                                self.current_event = Some(event);
                            }
                            false
                        }
                        WindowEvent::MouseInput { button, .. } => match button {
                            MouseButton::Left => {
                                println!("Left mouse clicked out 😀!");
                                true
                            }
                            _ => false,
                        },
                        _ => false,
                    },
                    // MouseButton::Right => {
                    //
                    // }
                    _ => false,
                },
                _ => false,
            },
        }
    }

    pub fn clear(&mut self) {
        self.previous_event = None;
        self.current_event = None;
    }

    pub fn get_current_event(&self) -> Option<&WindowEvent> {
        self.current_event.as_ref()
    }
}
