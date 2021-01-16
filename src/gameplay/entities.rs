//! Systems spawning entities of the game.

use bevy::prelude::*;
use rand::random;

use crate::{
    collisions::{BoxCollider, Movement, Position, TriggerArea},
    constants::{WINDOW_HEIGHT, WINDOW_WIDTH},
};

use super::{
    items::{AskedItem, AskingItem, Item, ItemProducer},
    materials::GameplayMaterials,
    Baobei, Didi,
};

/// Stores entities in the gameplay phase
pub struct GameData {
    /// Entity of didi
    pub didi_entity: Entity,
    /// Entity of baobei
    pub baobei_entity: Entity,
}

/// Spawn the camera.
pub fn setup_camera(commands: &mut Commands) {
    let mut camera_2d = Camera2dBundle::default();
    camera_2d.transform.translation += Vec3::new(WINDOW_WIDTH / 2.0, WINDOW_HEIGHT / 2.0, 0.0);

    commands.spawn(camera_2d);
}

/// Spawn the entity for Didi, the player and Baobei.
pub fn spawn_didi_and_baobei(commands: &mut Commands, materials: Res<GameplayMaterials>) {
    let position = Position(Vec3::new(640.0, 260.0, 0.0));
    let transform = Transform::from_scale(Vec3::new(0.3, 0.3, 0.0));
    let collider = BoxCollider {
        size: Vec2::new(75.0, 50.0),
        offset: Vec3::new(0.0, -10.0, 0.0),
    };

    let didi_entity = commands
        .spawn((Didi, position, collider, Movement::default()))
        .with_bundle(SpriteBundle {
            material: materials.didi_sprite.clone(),
            transform,
            ..SpriteBundle::default()
        })
        .current_entity()
        .unwrap();

    let asked_item = random::<Item>();

    let baobei_entity = commands
        .spawn((
            Baobei,
            Position(Vec3::new(1050.0, 150.0, 85.0)),
            TriggerArea::new(150.0, 150.0),
            AskingItem(asked_item),
        ))
        .with_bundle(SpriteBundle {
            material: materials.baobei_sprite.clone(),
            transform,
            ..SpriteBundle::default()
        })
        .with_children(|parent| {
            parent
                .spawn(SpriteBundle {
                    material: materials.item_sprite_for(asked_item),
                    transform: Transform {
                        translation: Vec3::new(0.0, 475.0, 0.0),
                        scale: Vec3::new(1.5, 1.5, 0.0),
                        ..Transform::default()
                    },
                    ..SpriteBundle::default()
                })
                .with(AskedItem);
        })
        .current_entity()
        .unwrap();

    commands.insert_resource(GameData {
        didi_entity,
        baobei_entity,
    });
}

/// Spawn furniture in the.
pub fn spawn_furniture(commands: &mut Commands, materials: Res<GameplayMaterials>) {
    commands
        // Sink
        .spawn(SpriteBundle {
            material: materials.sink_sprite.clone(),
            transform: Transform::from_scale(Vec3::new(0.3, 0.3, 0.0)),
            ..SpriteBundle::default()
        })
        .with_bundle((
            Position(Vec3::new(1050.0, 500.0, 0.0)),
            BoxCollider {
                size: Vec2::new(220.0, 40.0),
                offset: Vec3::new(0.0, 10.0, 0.0),
            },
        ))
        // Kitchen
        .spawn(SpriteBundle {
            material: materials.kitchen_sprite.clone(),
            transform: Transform::from_scale(Vec3::new(0.5, 0.5, 0.0)),
            ..SpriteBundle::default()
        })
        .with_bundle((
            Position(Vec3::new(300.0, 540.0, 0.0)),
            BoxCollider::new(400.0, 100.0),
        ))
        // Fridge
        .spawn(SpriteBundle {
            material: materials.fridge_sprite.clone(),
            transform: Transform::from_scale(Vec3::new(0.35, 0.35, 0.0)),
            ..SpriteBundle::default()
        })
        .with_bundle((
            Position(Vec3::new(720.0, 540.0, 0.0)),
            BoxCollider::new(100.0, 100.0),
        ))
        // Couch
        .spawn(SpriteBundle {
            material: materials.couch_sprite.clone(),
            transform: Transform::from_scale(Vec3::new(0.4, 0.4, 0.0)),
            ..SpriteBundle::default()
        })
        .with_bundle((
            Position(Vec3::new(1000.0, 150.0, 0.0)),
            BoxCollider {
                size: Vec2::new(300.0, 40.0),
                offset: Vec3::new(10.0, 15.0, 0.0),
            },
        ))
        // Table
        .spawn(SpriteBundle {
            material: materials.table_sprite.clone(),
            transform: Transform::from_scale(Vec3::new(0.4, 0.4, 0.0)),
            ..SpriteBundle::default()
        })
        .with_bundle((
            Position(Vec3::new(300.0, 200.0, 0.0)),
            BoxCollider {
                size: Vec2::new(300.0, 40.0),
                offset: Vec3::new(0.0, 25.0, 0.0),
            },
        ));
}

/// Spawn item producers.
pub fn spawn_item_producers(commands: &mut Commands) {
    commands
        .spawn((
            ItemProducer(Item::WaterGlass),
            Position(Vec3::new(1050.0, 500.0, 0.0)),
            TriggerArea::new(230.0, 50.0),
        ))
        .spawn((
            ItemProducer(Item::Chips),
            Position(Vec3::new(210.0, 480.0, 0.0)),
            TriggerArea::new(75.0, 75.0),
        ))
        .spawn((
            ItemProducer(Item::IceCream),
            Position(Vec3::new(720.0, 540.0, 0.0)),
            TriggerArea::new(175.0, 175.0),
        ));
}

/// Spawn boarders of the room, avoiding the user to go out of the screen.
pub fn spawn_boarders(commands: &mut Commands) {
    /// Gap between the screen limit and the available space.
    const GAP: f32 = 50.0;

    commands
        // Top
        .spawn((
            Position(Vec3::new(WINDOW_WIDTH / 2.0, 510.0 + GAP, 0.0)),
            BoxCollider::new(WINDOW_WIDTH, GAP),
        ))
        // Bottom
        .spawn((
            Position(Vec3::new(WINDOW_WIDTH / 2.0, GAP / 2.0, 0.0)),
            BoxCollider::new(WINDOW_WIDTH, GAP),
        ))
        // Left
        .spawn((
            Position(Vec3::new(GAP / 2.0, WINDOW_HEIGHT / 2.0, 0.0)),
            BoxCollider::new(GAP, WINDOW_HEIGHT),
        ))
        // Right
        .spawn((
            Position(Vec3::new(
                WINDOW_WIDTH - GAP / 2.0,
                WINDOW_HEIGHT / 2.0,
                0.0,
            )),
            BoxCollider::new(GAP, WINDOW_HEIGHT),
        ));
}
