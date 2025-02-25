use winit::event::WindowEvent;

pub trait InputEventTrait<'a, T> {
    fn finished(&self) -> bool;
    fn update(&mut self, event: &WindowEvent);
    fn in_progress(&self) -> bool;
    fn get_event(&'a self) -> Result<Option<T>, String>;
    fn clear(&mut self);
}
