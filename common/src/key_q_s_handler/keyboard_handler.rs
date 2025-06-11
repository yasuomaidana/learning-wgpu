use winit::event::WindowEvent;

/// Trait providing default methods for keyboard handlers.
///
/// Types implementing this trait must define an associated `Action` type,
/// and provide methods to handle input and clear state.
pub trait DefaultKeyboardHandlerMethods {
    type Action;
    // self refers to the instance of the type (like this in other languages).
    // In method signatures, &mut self means the method takes a mutable
    // reference to the instance.
    //
    //
    // Self (with a capital S) refers to the implementing type of the trait.
    // In type `Action;`, `Self::Action` means the associated type Action defined
    // for the type that implements the trait.
    fn input(&mut self, event: &WindowEvent);
    fn clear(&mut self);
}

/// Trait for handling keyboard input and mapping it to actions.
/// Types implementing this trait must define an associated `Action` type,
/// provide a constructor, and a method to retrieve the next action.
pub trait KeysHandler: DefaultKeyboardHandlerMethods {
    type Action;
    fn new() -> Self;
    // Here, `<Self as KeysHandler>::Action` explicitly refers to the Action
    // associated type from the KeysHandler trait, not from any supertrait
    // (like DefaultKeyboardHandlerMethods).
    //
    // This is necessary because both traits define an Action associated type,
    // and Rust needs to know which one you mean.
    // Using <Self as KeysHandler>::Action removes ambiguity and ensures you get
    // the correct associated type from the intended trait
    fn get_action(&mut self) -> Option<<Self as KeysHandler>::Action>;
}
