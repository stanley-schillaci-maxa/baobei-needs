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

use collisions::{BoxCollider, CollisionPlugin};
use constants::SPEED;
use controllers::{ControllerPlugin, DirectionEvent};

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "Baobei needs".to_string(),
            ..WindowDescriptor::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(ControllerPlugin)
        .add_plugin(CollisionPlugin)
        .add_startup_system(setup_entities.system())
        .add_system_to_stage(stage::FIRST, exit_on_esc_system.system())
        .add_system_to_stage(stage::UPDATE, movement_system.system())
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

    let size = Vec2::new(200.0, 200.0);

    let color_handle = materials.add(Color::rgb(0.3, 1.0, 0.3).into());

    let add_box_collider_sprite = |parent: &mut ChildBuilder| {
        parent.spawn(SpriteComponents {
            material: color_handle.clone(),
            sprite: Sprite::new(size),
            ..SpriteComponents::default()
        });
    };

    commands
        .spawn(Camera2dComponents::default())
        .spawn((Didi, BoxCollider(size)))
        .with_bundle(SpriteComponents {
            material: materials.add(didi_texture_handle.into()),
            transform: Transform::from_scale(Vec3::new(0.3, 0.3, 0.0)),
            ..SpriteComponents::default()
        })
        .with_children(add_box_collider_sprite)
        .spawn((Baobei, BoxCollider(size)))
        .with_bundle(SpriteComponents {
            material: materials.add(baobei_texture_handle.into()),
            transform: Transform {
                scale: Vec3::new(0.3, 0.3, 0.0),
                translation: Vec3::new(20.0, 20.0, 1.0),
                ..Transform::default()
            },
            ..SpriteComponents::default()
        })
        .with_children(add_box_collider_sprite);
}

/// Moves Didi toward the direction sent by controllers.
fn movement_system(
    time: Res<Time>,
    mut direction_event_reader: Local<EventReader<DirectionEvent>>,
    direction_events: Res<Events<DirectionEvent>>,
    mut query: Query<(&Didi, &mut Transform)>,
) {
    for event in direction_event_reader.iter(&direction_events) {
        for (_didi, mut transform) in query.iter_mut() {
            transform.translation += event.direction * time.delta_seconds * SPEED;
        }
    }
}
