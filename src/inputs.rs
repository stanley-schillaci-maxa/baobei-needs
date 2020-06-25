//! Manages game controllers

use crate::{constants::SPEED, physics::Velocity};
use ggez::event::{Axis, KeyCode};

/// Increases the velocity depending on the key direction
pub fn update_velocity_key_down(velocity: &mut Velocity, keycode: KeyCode) {
    match keycode {
        KeyCode::Up => velocity.y = -SPEED,
        KeyCode::Down => velocity.y = SPEED,
        KeyCode::Left => velocity.x = -SPEED,
        KeyCode::Right => velocity.x = SPEED,
        _ => (),
    }
}

/// Decreases the velocity depending on the key direction.
pub fn update_velocity_key_up(velocity: &mut Velocity, keycode: KeyCode) {
    match keycode {
        KeyCode::Up | KeyCode::Down => velocity.y = 0.0,
        KeyCode::Left | KeyCode::Right => velocity.x = 0.0,
        _ => (),
    }
}

/// Sets the velocity to the axis direction.
pub fn update_velocity_gamepad_axis(velocity: &mut Velocity, axis: Axis, value: f32) {
    match axis {
        Axis::LeftStickX => velocity.x = value * SPEED,
        Axis::LeftStickY => velocity.y = -value * SPEED,
        _ => (),
    }
}
