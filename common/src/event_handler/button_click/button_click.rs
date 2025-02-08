use crate::event_handler::event_command::{default_compare_events, mouse_button_event_generator};
use std::collections::VecDeque;
use winit::event::{ElementState, MouseButton, WindowEvent};

const QUEUE_SIZE: usize = 10;

pub trait ButtonClickEvent {
    fn finished(&self) -> bool;
    fn update(&mut self, event: &WindowEvent);
    fn in_progress(&self) -> bool;
    fn get_button_event(&self) -> Option<ButtonEvent>;
    fn clear(&mut self);
}

pub enum ButtonEvent<'a> {
    LeftClick(&'a ButtonClick),
    RightClick(&'a ButtonClick),
    OtherClick,
}

#[derive(Debug)]
pub struct ButtonClick {
    starting_event: WindowEvent,
    started: bool,
    pressure_values: VecDeque<f32>,
    finishing_event: WindowEvent,
    finished: bool,
}

impl ButtonClick {
    pub fn new(mouse_button: MouseButton) -> ButtonClick {
        let starting_event = mouse_button_event_generator(mouse_button, ElementState::Pressed);
        let finishing_event = mouse_button_event_generator(mouse_button, ElementState::Released);
        ButtonClick {
            starting_event,
            started: false,
            pressure_values: VecDeque::new(),
            finishing_event,
            finished: false,
        }
    }
    pub fn get_pressure(&self) -> f32 {
        if self.pressure_values.is_empty() {
            0.0
        } else if self.pressure_values.len() == 1 {
            self.pressure_values[0]
        } else {
            let mut min_change = f32::MAX;
            let mut last_value = self.pressure_values[0];
            let mut ret_value = last_value;
            for value in self.pressure_values.iter().skip(1) {
                let change = (value - last_value).abs();
                if change < min_change {
                    min_change = change;
                    ret_value = *value;
                }
                last_value = *value;
            }
            return ret_value;
        }
    }
}

fn validate_value<T>(queue: &mut VecDeque<T>, value: T) {
    if queue.len() > QUEUE_SIZE {
        queue.pop_front();
    }
    queue.push_back(value);
}
impl ButtonClickEvent for ButtonClick {
    fn finished(&self) -> bool {
        self.finished
    }

    fn update(&mut self, event: &WindowEvent) {
        if !self.started && default_compare_events(event, &self.starting_event) {
            self.started = true;
        } else if self.started && default_compare_events(event, &self.finishing_event) {
            self.finished = true;
        } else {
            match event {
                WindowEvent::TouchpadPressure { pressure, .. } => {
                    validate_value(&mut self.pressure_values, *pressure);
                }
                _ => {}
            }
        }
    }

    fn in_progress(&self) -> bool {
        self.started && !self.finished
    }

    fn get_button_event(&self) -> Option<ButtonEvent> {
        if !self.started {
            None
        } else {
            match self.starting_event {
                WindowEvent::MouseInput { button, .. } => match button {
                    MouseButton::Left => Some(ButtonEvent::LeftClick(self)),
                    MouseButton::Right => Some(ButtonEvent::RightClick(self)),
                    _ => panic!("Button not supported"),
                },
                _ => panic!("Button not supported"),
            }
        }
    }

    fn clear(&mut self) {
        self.started = false;
        self.finished = false;
        self.pressure_values.clear();
    }
}
