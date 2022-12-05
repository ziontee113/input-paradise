use self::{raw_keyboard_event::RawKeyboardEvent, raw_mouse_event::RawMouseEvent};

mod raw_keyboard_event;
mod raw_mouse_event;

pub enum RawInputEvent {
    Keyboard(RawKeyboardEvent),
    Mouse(RawMouseEvent),
}
