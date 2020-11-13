//! Components and systems permitting to move and collide entities.

use std::{
    cmp::max,
    cmp::min,
    collections::{HashMap, HashSet},
    hash::{Hash, Hasher},
};

use bevy::{math::Vec2, prelude::*, sprite::collide_aabb::collide};

/// Plugin managing contact collisions
pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<ContactEvent>()
            .register_component::<Position>()
            .register_component::<BoxCollider>()
            .add_system_to_stage(stage::EVENT, collision_system.system());
    }
}

/// Position
#[derive(Clone, Copy, Debug, Default, Properties)]
pub struct Position(pub Vec3);

#[derive(Debug, Default, Properties)]
/// 2D Collider in a shape of a rectangle
pub struct BoxCollider {
    /// The width and height of the box.
    pub size: Vec2,
}

/// Represents a contact between two entities
#[derive(Clone, Copy, Debug, Eq)]
pub struct Contact(Entity, Entity);

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

/// Compare positions of box colliders and emit contact events.
pub fn collision_system(
    mut commands: Commands,
    mut contact_events: ResMut<Events<ContactEvent>>,
    query: Query<(Entity, &Position, &BoxCollider)>,
    contacts: Query<(&Contact, Entity)>,
) {
    use itertools::Itertools;

    let next_contacts: HashSet<_> = query
        .iter()
        .combinations_with_replacement(2)
        .filter_map(|pair| {
            let (entity_a, pos_a, collider_a) = pair[0];
            let (entity_b, pos_b, collider_b) = pair[1];

            let size_a = collider_a.size;
            let size_b = collider_b.size;

            collide(pos_a.0, size_a, pos_b.0, size_b).map(|_| Contact(entity_a, entity_b))
        })
        .collect();

    let prev_entities: HashMap<_, _> = contacts.iter().map(|(&c, e)| (c, e)).collect();
    let prev_contacts: HashSet<_> = prev_entities.keys().copied().collect();

    for &started_contact in next_contacts.difference(&prev_contacts) {
        dbg!(started_contact);

        contact_events.send(ContactEvent::Started(started_contact));
        commands.spawn((started_contact,));
    }

    for stopped_contact in prev_contacts.difference(&next_contacts) {
        dbg!(stopped_contact);

        contact_events.send(ContactEvent::Stopped(*stopped_contact));
        if let Some(&entity) = prev_entities.get(stopped_contact) {
            commands.despawn(entity);
        }
    }
}
