use winit::dpi::PhysicalSize;
use winit::window::Window;

pub trait DefaultResizeWindowMethods {
    fn resize(&mut self, new_size: PhysicalSize<u32>);
    fn window(&self) -> &Window;
}

pub trait DefaultAppStateMethods: DefaultResizeWindowMethods {
    fn update(&mut self);
}

pub trait AppState: DefaultAppStateMethods {
    fn new(window: Window) -> Self;
    fn render(&mut self) -> Result<(), wgpu::SurfaceError>;
}

#[macro_export]
macro_rules! implement_resumed {
    () => {
        fn resumed(&mut self, event_loop: &ActiveEventLoop) {
            let window = event_loop
                .create_window(Window::default_attributes().with_title(self.window_name.as_str()))
                .expect("Failed to create window");
            self.state = Some(AppState::new(window));
        }
    };
}
#[macro_export]
macro_rules! default_event {
    ($self:ident, $event_loop:ident, $event:ident) => {
        match $event {
            WindowEvent::CloseRequested => {
                $event_loop.exit();
            }
            WindowEvent::Resized(physical_size) => {
                $self.state.as_mut().unwrap().resize(physical_size);
            }
            WindowEvent::RedrawRequested => {
                $self.state.as_mut().unwrap().render().unwrap();
            }
            _ => {}
        }
    };
    ($self:ident,$event_loop:ident,$event:ident, $window_id:ident) => {{
        $self.event_handler.input(&$event);
        let read_input = $self.event_handler.get_action();
        let window = $self.state.as_ref().unwrap().window();
        if window.id() == $window_id {
            default_event!($self, $event_loop, $event);
        }
        read_input
    }};
}
