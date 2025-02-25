use winit::event::WindowEvent;

pub trait EventHandler<T> {
    fn finished(&self) -> bool;
    fn update(&mut self, event: &WindowEvent);
    fn in_progress(&self) -> bool;
    fn get_event(&self) -> Result<Option<T>, String>;
    fn clear(&mut self);
}
