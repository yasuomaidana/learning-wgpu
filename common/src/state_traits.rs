use winit::dpi::PhysicalSize;
use winit::window::Window;

pub trait DefaultResizeWindowMethods {
    fn resize(&mut self, new_size: PhysicalSize<u32>);
    fn window(&self) -> &Window;
}

pub trait DefaultAppStateMethods: DefaultResizeWindowMethods {
    fn update(&mut self);
}
