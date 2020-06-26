//! Manages game controllers

use ggez::{event::KeyCode, nalgebra::Vector2};

/// Returns a vector oriented toward the related direction.
pub fn key_direction(keycode: KeyCode) -> Vector2<f32> {
    let (dx, dy) = match keycode {
        KeyCode::Up => (0.0, -1.0),
        KeyCode::Down => (0.0, 1.0),
        KeyCode::Left => (-1.0, 0.0),
        KeyCode::Right => (1.0, 0.0),
        _ => (0.0, 0.0),
    };

    Vector2::new(dx, dy)
}
