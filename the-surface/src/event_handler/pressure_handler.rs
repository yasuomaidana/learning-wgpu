use winit::event::WindowEvent;

pub struct PressureHandler {
    previous_event: Option<WindowEvent>,
    pressure: f64,
}

impl PressureHandler {
    pub fn new() -> PressureHandler {
        PressureHandler {
            previous_event: None,
            pressure: 0.1,
        }
    }

    pub fn handle_pressure(&mut self, event: &WindowEvent) -> bool {
        let previous_event = self.previous_event.as_ref();
        match previous_event {
            Some(WindowEvent::MouseInput { button, .. }) => match button {
                winit::event::MouseButton::Left => match event {
                    WindowEvent::TouchpadPressure { pressure, .. } => {
                        self.pressure = *pressure as f64;
                        false
                    }
                    WindowEvent::MouseInput { button, .. } => match button {
                        winit::event::MouseButton::Left => { 
                            self.previous_event = None;
                            true
                        },
                        _ => {
                            self.pressure = 0.1;
                            false
                        }
                    },
                    _ => {
                        self.pressure = 0.1;
                        false
                    }
                },
                _ => {
                    false
                }
            },
            _ => {
                match event {
                    WindowEvent::MouseInput { button,.. } => {
                        match button { 
                            winit::event::MouseButton::Left => {
                                self.previous_event = Some(event.clone());
                                false
                            }
                            _ => {
                                false
                            }
                        }
                    }
                    _ => {
                        false
                    }
                }
            }
        }
    }

    pub fn pressure(&self) -> f64 {
        self.pressure
    }
}
