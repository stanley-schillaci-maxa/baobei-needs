//! A little game made with Bevy.

// Clippy configuration
#![deny(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    clippy::clippy::missing_docs_in_private_items
)]
#![allow(clippy::needless_pass_by_value, clippy::clippy::cast_precision_loss)]

mod collisions;
mod constants;
mod controllers;
mod scenes;

use bevy::{input::system::exit_on_esc_system, prelude::*};

use collisions::{CollisionPlugin, Position};
use constants::{SPEED, WINDOW_HEIGHT, WINDOW_WIDTH};
use controllers::{ControllerPlugin, DirectionEvent};
use scenes::SceneLoaderPlugin;

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "Baobei needs".to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            ..WindowDescriptor::default()
        })
        .add_plugins(DefaultPlugins)
        .register_component::<Didi>()
        .register_component::<Furniture>()
        .register_component::<Baobei>()
        .add_plugin(ControllerPlugin)
        .add_plugin(CollisionPlugin)
        .add_plugin(SceneLoaderPlugin)
        .add_startup_system(setup_camera.system())
        .add_system_to_stage(stage::FIRST, exit_on_esc_system.system())
        .add_system_to_stage(stage::UPDATE, movement_system.system())
        .add_system_to_stage(stage::POST_UPDATE, drawing_system.system())
        .run();
}

/// The player
#[derive(Properties, Default)]
struct Didi;
/// The baobei to take care of
#[derive(Properties, Default)]
struct Baobei;
/// Furniture containing items
#[derive(Properties, Default)]
struct Furniture;

/// Spawn the camera.
fn setup_camera(mut commands: Commands) {
    let mut camera_2d = Camera2dComponents::default();
    camera_2d.transform.translation +=
        Vec3::new(WINDOW_WIDTH as f32 / 2.0, WINDOW_HEIGHT as f32 / 2.0, 0.0);

    commands.spawn(camera_2d);
}

/// Moves Didi toward the direction sent by controllers.
fn movement_system(
    time: Res<Time>,
    mut direction_event_reader: Local<EventReader<DirectionEvent>>,
    direction_events: Res<Events<DirectionEvent>>,
    mut query: Query<(&Didi, &mut Position)>,
) {
    for event in direction_event_reader.iter(&direction_events) {
        for (_didi, mut position) in query.iter_mut() {
            position.0 += event.direction * time.delta_seconds * SPEED;
        }
    }
}

/// Update position of the sprite with the position of the entity
fn drawing_system(position: Changed<Position>, mut transform: Mut<Transform>) {
    transform.translation = position.0;
}
