//! Systems and functions managing the display of things in the screen.

use bevy::prelude::*;

use crate::{collisions::Position, constants::WINDOW_HEIGHT};

/// Plugin the drawing things on the screen.
pub struct DrawingPlugin;

impl Plugin for DrawingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_to_stage(stage::UPDATE, drawing_position_system.system());
    }
}

/// Updates position of the sprite with the position of the entity
fn drawing_position_system(mut query: Query<(&Position, &mut Transform), Without<Parent>>) {
    for (position, mut transform) in query.iter_mut() {
        /// Limit position in which the displayed sprite is still visible.
        const Z_LIMIT: f32 = 1000.0;

        transform.translation = position.0;

        // Scale the z index between 0 and 1000 depending on the y index.
        transform.translation.z = Z_LIMIT - position.0.y * Z_LIMIT / WINDOW_HEIGHT;

        // Move up the entities in the air.
        transform.translation.y += position.0.z;
    }
}
