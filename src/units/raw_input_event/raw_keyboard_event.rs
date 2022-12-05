use std::time::SystemTime;

use crate::units::key_identifier::KeyIdentifier;

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum KeyState {
    Down,
    Up,
    Hold,
    Uninitiated,
}

impl From<i32> for KeyState {
    fn from(value: i32) -> Self {
        match value {
            0 => KeyState::Up,
            1 => KeyState::Down,
            2 => KeyState::Hold,
            -1 => KeyState::Uninitiated,
            _ => unreachable!(),
        }
    }
}

pub struct RawKeyboardEvent {
    key: KeyIdentifier,
    state: KeyState,
    timestamp: SystemTime,
}

impl RawKeyboardEvent {
    pub fn new(key: KeyIdentifier, state: KeyState, timestamp: SystemTime) -> Self {
        Self {
            key,
            state,
            timestamp,
        }
    }
}
