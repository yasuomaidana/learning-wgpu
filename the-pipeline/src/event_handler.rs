use common::event_handler::button_click::button_click::{
    ButtonClick, ButtonClickEvent, ButtonEvent,
};
// use winit::dpi::PhysicalPosition;
// use winit::event::WindowEvent::CursorMoved;
use winit::event::{MouseButton, WindowEvent};

pub struct EventHandler {
    left_click: ButtonClick,
    right_click: ButtonClick,
    // cursor_position: PhysicalPosition<f64>,
}

impl EventHandler {
    pub fn new() -> EventHandler {
        EventHandler {
            left_click: ButtonClick::new(MouseButton::Left),
            right_click: ButtonClick::new(MouseButton::Right),
            // cursor_position: PhysicalPosition::new(0.0, 0.0),
        }
    }

    pub fn input(&mut self, event: WindowEvent) -> Option<ButtonEvent> {
        self.left_click.update(&event);
        self.right_click.update(&event);
        // match event {
        //     CursorMoved { position, .. } => {
        //         self.cursor_position = position;
        //     }
        //     _ => {}
        // }
        if self.left_click.get_button_event().is_some() {
            self.left_click.get_button_event()
        } else if self.right_click.get_button_event().is_some() {
            self.right_click.get_button_event()
        } else {
            None
        }
    }

    pub fn clear(&mut self) {
        if self.left_click.finished() {
            self.left_click.clear();
        }
        if self.right_click.finished() {
            self.right_click.clear();
        }
    }
}
