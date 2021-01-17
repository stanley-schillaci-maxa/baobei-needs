//! Systems and functions managing the display of things in the screen.

use bevy::prelude::*;

use crate::{
    collisions::Position,
    constants::{WINDOW_HEIGHT, WINDOW_WIDTH},
};

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
///
/// TODO: Scales the translation and sprite scale with the window size.
/// This needs the scale as a component and center positions in the middle
/// of the window
fn update_game_object_position_system(
    // windows: Res<Windows>,
    mut game_objects: Query<(&Position, &mut Transform), Without<(Parent, UIObject)>>,
) {
    // TODO: scale depending on window size
    // let window = windows.get_primary().unwrap();
    // dbg!(window.width(), window.height());
    // window_size_ratio(window);

    for (position, mut transform) in game_objects.iter_mut() {
        transform.translation = position.0;

        // Scale the z index depending on the y index.
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

/// Returns the ratio between the current size and the initial one.
fn window_size_ratio(window: Window) -> Vec3 {
    Vec3::new(
        window.width() / WINDOW_WIDTH,
        window.height() / WINDOW_HEIGHT,
        1.0,
    )
}
