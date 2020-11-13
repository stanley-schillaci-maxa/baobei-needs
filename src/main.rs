//! A little game

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
/// The baobei to take care of
struct Baobei;
/// Furniture containing items
struct Furniture;

/// Spawn the camera, Didi and Baobei.
fn setup_entities(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let color_handle = materials.add(Color::rgba(0.3, 1.0, 0.3, 0.3).into());

    let box_collider_sprite = |size: Vec2| SpriteComponents {
        material: color_handle.clone(),
        sprite: Sprite::new(size),
        ..SpriteComponents::default()
    };

    let mut camera_2d = Camera2dComponents::default();
    camera_2d.transform.translation +=
        Vec3::new(WINDOW_WIDTH as f32 / 2.0, WINDOW_HEIGHT as f32 / 2.0, 0.0);

    commands.spawn(camera_2d);

    let couch_position = Position((1000.0, 150.0, 0.0).into());
    let couch_size = Vec2::new(100.0, 100.0);

    commands
        .spawn((Furniture, couch_position, BoxCollider { size: couch_size }))
        .with_bundle(SpriteComponents {
            material: materials.add(asset_server.load("couch.png").into()),
            transform: Transform::from_scale(Vec3::new(0.4, 0.4, 0.0)),
            ..SpriteComponents::default()
        })
        .spawn((Furniture, couch_position))
        .with_bundle(box_collider_sprite(couch_size));

    let fridge_position = Position((720.0, 540.0, 0.0).into());
    let fridge_size = Vec2::new(100.0, 100.0);

    commands
        .spawn((
            Furniture,
            fridge_position,
            BoxCollider { size: fridge_size },
        ))
        .with_bundle(SpriteComponents {
            material: materials.add(asset_server.load("fridge.png").into()),
            transform: Transform::from_scale(Vec3::new(0.35, 0.35, 0.0)),
            ..SpriteComponents::default()
        })
        .spawn((Furniture, fridge_position))
        .with_bundle(box_collider_sprite(fridge_size));

    let kitchen_position = Position((300.0, 540.0, 0.0).into());
    let kitchen_size = Vec2::new(100.0, 100.0);

    commands
        .spawn((
            Furniture,
            kitchen_position,
            BoxCollider { size: kitchen_size },
        ))
        .with_bundle(SpriteComponents {
            material: materials.add(asset_server.load("kitchen.png").into()),
            transform: Transform::from_scale(Vec3::new(0.5, 0.5, 0.0)),
            ..SpriteComponents::default()
        })
        .spawn((Furniture, kitchen_position))
        .with_bundle(box_collider_sprite(kitchen_size));

    let sink_position = Position((1050.0, 450.0, 0.0).into());
    let sink_size = Vec2::new(100.0, 100.0);

    commands
        .spawn((Furniture, sink_position, BoxCollider { size: sink_size }))
        .with_bundle(SpriteComponents {
            material: materials.add(asset_server.load("sink.png").into()),
            transform: Transform::from_scale(Vec3::new(0.3, 0.3, 0.0)),
            ..SpriteComponents::default()
        })
        .spawn((Furniture, sink_position))
        .with_bundle(box_collider_sprite(sink_size));

    let size = Vec2::new(100.0, 20.0);

    let baobei_position = Position((1050.00, 250.0, 0.0).into());

    commands
        .spawn((Baobei, baobei_position, BoxCollider { size }))
        .with_bundle(SpriteComponents {
            material: materials.add(asset_server.load("baobei.png").into()),
            transform: Transform::from_scale(Vec3::new(0.3, 0.3, 0.0)),
            ..SpriteComponents::default()
        })
        .spawn((Baobei, baobei_position))
        .with_bundle(box_collider_sprite(size));

    let didi_position = Position((640.00, 260.0, 0.0).into());

    commands
        .spawn((Didi, didi_position, BoxCollider { size }))
        .with_bundle(SpriteComponents {
            material: materials.add(asset_server.load("didi.png").into()),
            transform: Transform::from_scale(Vec3::new(0.3, 0.3, 0.0)),
            ..SpriteComponents::default()
        })
        .spawn((Didi, didi_position))
        .with_bundle(box_collider_sprite(size));
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
