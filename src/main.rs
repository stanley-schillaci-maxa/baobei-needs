//! A little game

// Clippy configuration
#![deny(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    clippy::clippy::missing_docs_in_private_items
)]
#![allow(clippy::needless_pass_by_value)]

mod collisions;
mod constants;
mod controllers;

use bevy::{input::system::exit_on_esc_system, prelude::*};

use collisions::{BoxCollider, CollisionPlugin, Position};
use constants::{SPEED, WINDOW_HEIGHT, WINDOW_WIDTH};
use controllers::{ControllerPlugin, DirectionEvent};

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "Baobei needs".to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            ..WindowDescriptor::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(ControllerPlugin)
        .add_plugin(CollisionPlugin)
        .add_startup_system(setup_entities.system())
        .add_system_to_stage(stage::FIRST, exit_on_esc_system.system())
        .add_system_to_stage(stage::UPDATE, movement_system.system())
        .add_system_to_stage(stage::POST_UPDATE, drawing_system.system())
        .run();
}

/// The player
struct Didi;
/// The baobei to take care of.
struct Baobei;

/// Spawn the camera, Didi and Baobei.
fn setup_entities(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let didi_texture_handle = asset_server.load("didi.png");
    let baobei_texture_handle = asset_server.load("baobei.png");

    let size = Vec2::new(100.0, 20.0);

    let color_handle = materials.add(Color::rgb(0.3, 1.0, 0.3).into());

    let box_collider_sprite = || SpriteComponents {
        material: color_handle.clone(),
        sprite: Sprite::new(size),
        ..SpriteComponents::default()
    };

    commands.spawn(Camera2dComponents::default());

    let didi_position = Position((-00.0, 200.0, 0.0).into());
    let baobei_position = Position((200.0, -200.0, 0.0).into());

    commands
        .spawn((Didi, didi_position, BoxCollider { size }))
        .with_bundle(SpriteComponents {
            material: materials.add(didi_texture_handle.into()),
            transform: Transform::from_scale(Vec3::new(0.3, 0.3, 0.0)),
            ..SpriteComponents::default()
        })
        .spawn((Didi, didi_position))
        .with_bundle(box_collider_sprite());

    commands
        .spawn((Baobei, baobei_position, BoxCollider { size }))
        .with_bundle(SpriteComponents {
            material: materials.add(baobei_texture_handle.into()),
            transform: Transform::from_scale(Vec3::new(0.3, 0.3, 0.0)),
            ..SpriteComponents::default()
        })
        .spawn((Baobei, baobei_position))
        .with_bundle(box_collider_sprite());
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
