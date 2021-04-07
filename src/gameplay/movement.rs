//! Systems managing movement of the player

use bevy::prelude::*;

use crate::{collisions::Movement, constants::SPEED, controllers::DirectionEvent};

use super::Didi;

/// Moves Didi toward the direction sent by controllers.
pub fn movement_system(
    time: Res<Time>,
    mut direction_events: EventReader<DirectionEvent>,
    mut query: Query<&mut Movement, With<Didi>>,
) {
    for event in direction_events.iter() {
        for mut movement in query.iter_mut() {
            movement.0 = event.direction * time.delta_seconds() * SPEED;
        }
    }
}
