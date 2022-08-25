/// Possible buttons, including both keyboard and mouse buttons
// TODO actually thoughtfully enumerate all of these instead of
// typing a few I thought of off the top of my head
#[derive(Debug)]
pub enum Button {
    /// An unknown button input
    Unknown,
    MouseLeft,
    MouseRight,
    KeyW,
    KeyA,
    KeyS,
    KeyD,
    KeySpace,
    KeyEsc,
    KeyReturn,
}

/// Possible states of a button (can be either pressed or released)
pub enum ButtonState {
    Pressed,
    Released
}

pub struct ModifierKeys {
    ctrl: bool,
    shift: bool,
    alt: bool,
    meta: bool,
}

pub enum Event {
    /// The mouse moved to a new position ((x, y) in pixels)
    MousePos(f64, f64),

    /// The mouse wheel scrolled (TODO What are the arguments, if any?)
    Scroll(/* TODO */),

    /// A button changed state.
    /// 
    /// Covers both mouse and keyboard buttons. ButtonStatus is the new
    /// status of the button (either pressed or released), and ModifierKeys
    /// is a list of the active modifier keys when the event fired.
    Button(Button, ButtonState, ModifierKeys),

    /// The window resized ((new width, new height) in pixels)
    Resize(f64, f64),
}

/// A trait for the ability to handle an event
/// 
/// Makes no guarantees about *what* the implementer does with the
/// event - just that it is prepared to do *something* (or intentionally
/// nothing) with it
pub trait EventHandler {
    fn handle_event(&mut self, event: &Event);
}