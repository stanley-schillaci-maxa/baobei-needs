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
            .init_resource::<ColliderViewers>()
            .add_system_to_stage(stage::EVENT, collision_system.system())
            .add_system_to_stage(stage::PRE_UPDATE, add_collider_viewer_system.system())
            .add_system_to_stage(stage::POST_UPDATE, update_collider_viewers_system.system());
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

/// Component tagging an entity as a collider viewer
struct ColliderViewer;

/// Lobby containing connected gamepads.
#[derive(Default)]
struct ColliderViewers(HashMap<Entity, Entity>);

/// Adds to entities with a `SpritLoader` the related `SpriteComponents`.
fn add_collider_viewer_system(
    mut commands: Commands,
    mut viewers: ResMut<ColliderViewers>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<(Entity, Changed<BoxCollider>, &Position)>,
) {
    for (entity, box_collider, pos) in query.iter() {
        let color_handle = materials.add(Color::rgba(0.3, 1.0, 0.3, 0.3).into());

        viewers.0.entry(entity).or_insert_with(|| {
            commands
                .spawn((ColliderViewer, Position(pos.0)))
                .with_bundle(SpriteComponents {
                    material: color_handle,
                    sprite: Sprite::new(box_collider.size),
                    ..SpriteComponents::default()
                })
                .current_entity()
                .unwrap()
        });
    }
}

/// Adds to entities with a `SpritLoader` the related `SpriteComponents`.
fn update_collider_viewers_system(
    viewers: Res<ColliderViewers>,
    query: Query<(Entity, &BoxCollider, Changed<Position>)>,
    mut viewer_query: Query<(&ColliderViewer, Mut<Position>)>,
) {
    for (entity, _, pos) in query.iter() {
        if let Some((_, mut viewer_pos)) = viewers
            .0
            .get(&entity)
            .and_then(|&viewer| viewer_query.get_mut(viewer).ok())
        {
            viewer_pos.0 = pos.0
        }
    }
}
