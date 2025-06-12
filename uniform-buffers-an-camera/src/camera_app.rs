use crate::camera_controller::CameraController;
use crate::keyboard_handler::{Action, CameraKeyboardHandler};
use crate::state::State;
use common::keyboard_handler::{DefaultKeyboardHandlerMethods, KeysHandler};
use common::state_traits::DefaultResizeWindowMethods;
use common::state_traits::{AppState, DefaultAppStateMethods};
use common::{default_event, implement_resumed};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::Window;
use winit::window::WindowId;

pub struct CameraStateApplication<'a> {
    state: Option<State<'a>>,
    event_handler: CameraKeyboardHandler,
    window_name: String,
    camera_controller: CameraController,
}

impl<'a> CameraStateApplication<'a> {
    pub fn new(window_name: String) -> CameraStateApplication<'a> {
        CameraStateApplication {
            state: None,
            event_handler: CameraKeyboardHandler::new(),
            window_name,
            camera_controller: CameraController::new(0.1, 0.1),
        }
    }
}

impl<'a> ApplicationHandler for CameraStateApplication<'a> {
    implement_resumed!();

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let read_input = default_event!(self, event_loop, event, window_id);
        match read_input {
            None => {}
            Some(action) => {
                match action {
                    Action::Quit => event_loop.exit(),
                    other => {
                        self.camera_controller
                            .update(other, &mut self.state.as_mut().unwrap().camera); // _ => {}
                        self.state.as_mut().unwrap().update();
                    }
                }
                self.event_handler.clear();
            }
        }
    }
}
