//! Components and systems permitting to move and collide entities.

use std::{
    cmp::max,
    cmp::min,
    collections::{HashMap, HashSet},
    hash::{Hash, Hasher},
};

use bevy::{prelude::*, sprite::collide_aabb::collide};
use debug_collisions::DebugCollisionPlugin;

use crate::constants::GameState;

mod debug_collisions;

/// Plugin managing contact collisions
pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<ContactEvent>()
            .register_type::<Position>()
            .register_type::<BoxCollider>()
            .add_system_set(
                SystemSet::on_update(GameState::InGame)
                    .with_system(collision_system.system())
                    .with_system(trigger_area_system.system()),
            )
            .add_plugin(DebugCollisionPlugin);
    }
}

/// Absolute position of the entity.
#[derive(Clone, Copy, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct Position(pub Vec3);

/// Delta of the next movement the entity will do move to
#[derive(Debug, Reflect, Default)]
#[reflect(Component)]
pub struct Movement(pub Vec3);

/// Collider in a shape of a rectangle
#[derive(Clone, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct BoxCollider {
    /// The width and height of the box.
    pub size: Vec2,
    /// Offset of the collider with the position.
    pub offset: Vec3,
}

impl BoxCollider {
    /// Creates a box collider with the given size and no offset.
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            size: Vec2::new(width, height),
            offset: Vec3::new(0.0, 0.0, 0.0),
        }
    }
}

/// A rectangle area that can be contacted without collision.
#[derive(Clone, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct TriggerArea {
    /// The width and height of the box.
    pub size: Vec2,
}

impl TriggerArea {
    /// Creates a box collider with the given size.
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            size: Vec2::new(width, height),
        }
    }
}

/// Represents a contact between two entities
#[derive(Clone, Copy, Debug, Eq)]
pub struct Contact(pub Entity, pub Entity);

impl PartialEq for Contact {
    fn eq(&self, other: &Self) -> bool {
        let Self(a_1, b_1) = self;
        let Self(a_2, b_2) = other;

        let same = a_1 == a_2 && b_1 == b_2;
        let inverted = a_1 == b_2 && b_1 == a_2;

        same || inverted
    }
}

impl Hash for Contact {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let Self(a, b) = self;

        (min(a, b), max(a, b)).hash(state);
    }
}

/// Event appearing when entities collides.
#[derive(Clone, Debug, PartialEq)]
pub enum ContactEvent {
    /// A contact is happening.
    Started(Contact),
    /// A contact is finished.
    Stopped(Contact),
}

/// Moves the position of moving entities depending on their movement.
/// If the entity collides with another collider, then the movement will not be made.
///
/// The collision is checked for both the X and Y axises, and in case of
/// diagonal movement, one axis can still be moved.
pub fn collision_system(
    mut moving_colliders: Query<(&mut Position, &BoxCollider, &mut Movement)>,
    other_colliders: Query<(&Position, &BoxCollider), Without<Movement>>,
) {
    for (mut pos_a, col_a, mut mov_a) in moving_colliders.iter_mut() {
        let will_not_collide = |next_pos_a: Vec3| {
            other_colliders.iter().all(|(pos_b, col_b)| {
                collide(
                    next_pos_a + col_a.offset,
                    col_a.size,
                    pos_b.0 + col_b.offset,
                    col_b.size,
                )
                .is_none()
            })
        };

        if will_not_collide(pos_a.0 + mov_a.0 * Vec3::unit_x()) {
            pos_a.0.x += mov_a.0.x;
        }
        if will_not_collide(pos_a.0 + mov_a.0 * Vec3::unit_y()) {
            pos_a.0.y += mov_a.0.y;
        }

        *mov_a = Movement::default();
    }
}

/// Compares positions of box colliders with trigger areas and emit trigger
/// events.
pub fn trigger_area_system(
    mut commands: Commands,
    mut contact_events: EventWriter<ContactEvent>,
    moving_colliders: Query<(Entity, &Position, &BoxCollider), With<Movement>>,
    trigger_areas: Query<(Entity, &Position, &TriggerArea)>,
    contacts: Query<(&Contact, Entity)>,
) {
    let mut next_contacts: HashSet<Contact> = HashSet::new();

    for (entity_a, pos_a, col_a) in moving_colliders.iter() {
        for (entity_b, pos_b, area_b) in trigger_areas.iter() {
            if collide(pos_a.0, col_a.size, pos_b.0, area_b.size).is_some() {
                next_contacts.insert(Contact(entity_a, entity_b));
            }
        }
    }

    let prev_entities: HashMap<_, _> = contacts.iter().map(|(&c, e)| (c, e)).collect();
    let prev_contacts: HashSet<_> = prev_entities.keys().copied().collect();

    for &started_contact in next_contacts.difference(&prev_contacts) {
        debug!("Started contact: {:?}", started_contact);

        contact_events.send(ContactEvent::Started(started_contact));
        commands.spawn().insert(started_contact);
    }

    for stopped_contact in prev_contacts.difference(&next_contacts) {
        debug!("Stopped contact: {:?}", stopped_contact);

        contact_events.send(ContactEvent::Stopped(*stopped_contact));
        if let Some(&entity) = prev_entities.get(stopped_contact) {
            commands.entity(entity).despawn();
        }
    }
}
