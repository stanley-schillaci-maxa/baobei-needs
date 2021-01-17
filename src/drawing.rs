//! Systems and functions managing the display of things in the screen.

use bevy::prelude::*;

use crate::{collisions::Position, constants::WINDOW_HEIGHT};

/// Plugin the drawing things on the screen.
pub struct DrawingPlugin;

impl Plugin for DrawingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_to_stage(stage::UPDATE, update_game_object_position_system.system())
            .add_system_to_stage(stage::UPDATE, update_ui_objects_position_system.system());
    }
}

/// Component meaning that the entity will be drawn in the foreground as a UI object.
pub struct UIObject;

/// Limit value in which the displayed sprite is visible.
/// z = 0 => background, z = 1000 => foreground
const Z_LIMIT: f32 = 1000.0;

/// Updates transform of game objects following their game position.
fn update_game_object_position_system(
    mut game_objects: Query<(&Position, &mut Transform), Without<(Parent, UIObject)>>,
) {
    for (position, mut transform) in game_objects.iter_mut() {
        transform.translation = position.0;

        // Scale the z index between 0 and 1000 depending on the y index.
        // 0 = background, 1000 = foreground
        transform.translation.z = Z_LIMIT - position.0.y * Z_LIMIT / WINDOW_HEIGHT;

        // Move up the entities in the air.
        transform.translation.y += position.0.z;
    }
}

/// Updates transform of UI objects following their position.
fn update_ui_objects_position_system(
    mut ui_objects: Query<(&Position, &mut Transform), With<UIObject>>,
) {
    for (position, mut transform) in ui_objects.iter_mut() {
        transform.translation = position.0;
        transform.translation.z = Z_LIMIT - 1.0;
    }
}
