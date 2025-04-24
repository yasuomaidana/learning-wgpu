use winit::dpi::PhysicalSize;
use winit::window::Window;

pub trait DefaultAppStateMethods {
    fn resize(&mut self, new_size: PhysicalSize<u32>);
    fn update(&mut self);
    fn window(&self) -> &Window;
}
