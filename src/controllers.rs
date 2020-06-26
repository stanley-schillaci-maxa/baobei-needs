//! Manages game controllers such as Keyboard and Gamepad

use ggez::{event::KeyCode, nalgebra::Vector2};
use std::collections::HashSet;

/// Contains keyboard information
pub struct Keyboard {
    /// Set of keys that are being pressed.
    pressed_keys: HashSet<KeyCode>,
}

impl Keyboard {
    /// Creates a keyboard
    pub fn new() -> Self {
        Self {
            pressed_keys: HashSet::new(),
        }
    }

    /// Presses a key
    pub fn press_key(self: &mut Self, keycode: KeyCode) {
        self.pressed_keys.insert(keycode);
    }
    /// Unpresses a key
    pub fn unpress_key(self: &mut Self, keycode: KeyCode) {
        self.pressed_keys.remove(&keycode);
    }

    /// Returns a vector corresponding to the direction indicated by arrow keys.
    pub fn arrow_direction(self: &Self) -> Vector2<f32> {
        self.pressed_keys
            .iter()
            .map(|&keycode| match keycode {
                KeyCode::Up => Vector2::new(0.0, -1.0),
                KeyCode::Down => Vector2::new(0.0, 1.0),
                KeyCode::Left => Vector2::new(-1.0, 0.0),
                KeyCode::Right => Vector2::new(1.0, 0.0),
                _ => Vector2::new(0.0, 0.0),
            })
            .sum()
    }
}
