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
            .add_system_to_stage(stage::EVENT, collision_system.system());
    }
}

/// Rectangle collider, ie. the width and height.
pub struct BoxCollider(pub Vec2);

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
use itertools::Itertools;

/// Compare positions of box colliders and emit contact events.
pub fn collision_system(
    mut commands: Commands,
    mut contact_events: ResMut<Events<ContactEvent>>,
    query: Query<(Entity, &Transform, &BoxCollider)>,
    contacts: Query<(&Contact, Entity)>,
) {
    let next_contacts: HashSet<_> = query
        .iter()
        .combinations_with_replacement(2)
        .filter_map(|pair| {
            let (entity_a, transform_a, collider_a) = pair[0];
            let (entity_b, transform_b, collider_b) = pair[1];

            let pos_a = transform_a.translation;
            let size_a = transform_a.scale.truncate() * collider_a.0;

            let pos_b = transform_b.translation;
            let size_b = transform_b.scale.truncate() * collider_b.0;

            collide(pos_a, size_a, pos_b, size_b).map(|_| Contact(entity_a, entity_b))
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
