//! Manages game controllers such as Keyboard and Gamepad

use ggez::{
    event::{Axis, Button, KeyCode},
    nalgebra::Vector2,
};
use std::collections::HashSet;

/// Direction vector between 0 and 1
pub type Direction = Vector2<f32>;

/// State of the keyboard
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

    /// Releases a key
    pub fn release_key(self: &mut Self, keycode: KeyCode) {
        self.pressed_keys.remove(&keycode);
    }

    /// Returns a vector corresponding to the direction indicated by arrow keys.
    pub fn arrow_direction(self: &Self) -> Direction {
        self.pressed_keys
            .iter()
            .map(|&keycode| match keycode {
                KeyCode::Up => Direction::new(0.0, -1.0),
                KeyCode::Down => Direction::new(0.0, 1.0),
                KeyCode::Left => Direction::new(-1.0, 0.0),
                KeyCode::Right => Direction::new(1.0, 0.0),
                _ => Direction::new(0.0, 0.0),
            })
            .sum()
    }
}

/// State of the Gamepad
pub struct Gamepad {
    /// Set of buttons that are being pressed.
    pressed_buttons: HashSet<Button>,
    /// Direction the left stick point to.
    pub left_stick: Direction,
    /// Direction the right stick point to.
    pub right_stick: Direction,
}

impl Gamepad {
    /// Creates a keyboard
    pub fn new() -> Self {
        Self {
            pressed_buttons: HashSet::new(),
            left_stick: Direction::new(0.0, 0.0),
            right_stick: Direction::new(0.0, 0.0),
        }
    }

    /// Presses a button.
    pub fn press_button(self: &mut Self, button: Button) {
        self.pressed_buttons.insert(button);
    }

    /// Releases a button.
    pub fn release_button(self: &mut Self, button: Button) {
        self.pressed_buttons.remove(&button);
    }

    /// Update the stick corresponding to the axis toward the related direction.
    pub fn move_axis(self: &mut Self, axis: Axis, value: f32) {
        match axis {
            Axis::LeftStickX => self.left_stick.x = value,
            Axis::LeftStickY => self.left_stick.y = -value,
            Axis::RightStickX => self.right_stick.x = value,
            Axis::RightStickY => self.right_stick.y = -value,
            _ => {}
        }
    }
}
