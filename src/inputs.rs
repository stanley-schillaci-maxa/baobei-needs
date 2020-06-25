//! Manages game controllers

use ggez::{nalgebra::Vector2, event::{Axis, KeyCode}};

/// Return a vector oriented toward the related direction.
pub fn key_direction(keycode: KeyCode) -> Vector2<f32> {
    match keycode {
        KeyCode::Up => Vector2::new(0.0, -1.0),
        KeyCode::Down => Vector2::new(0.0, 1.0),
        KeyCode::Left => Vector2::new(-1.0, 0.0),
        KeyCode::Right => Vector2::new(1.0, 0.0),
        _ => Vector2::new(0.0, 0.0),
    }
}

/// Return a vector oriented toward the related direction.
pub fn axis_direction(axis: Axis) -> Vector2<f32> {
    match axis {
        Axis::LeftStickX => Vector2::new(1.0, 0.0),
        Axis::LeftStickY => Vector2::new(0.0, -1.0),
        _ => Vector2::new(0.0, 0.0),
    }
}
