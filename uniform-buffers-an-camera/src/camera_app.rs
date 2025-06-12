use crate::keyboard_handler::{Action, CameraKeyboardHandler};
use crate::state::State;
use appstate_macros::DefaultApp;
use common::keyboard_handler::{DefaultKeyboardHandlerMethods, KeysHandler};
use common::state_traits::DefaultResizeWindowMethods;
use common::state_traits::{AppState, DefaultAppStateMethods};
use common::{default_event, implement_resumed};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::Window;
use winit::window::WindowId;

#[derive(DefaultApp)]
pub struct CameraStateApplication<'a> {
    state: Option<State<'a>>,
    event_handler: CameraKeyboardHandler,
    window_name: String,
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
                    Action::Down => {
                        self.state.as_mut().unwrap().update();
                    }
                    Action::Quit => event_loop.exit(),
                    _ => {}
                }
                self.event_handler.clear();
            }
        }
    }
}
