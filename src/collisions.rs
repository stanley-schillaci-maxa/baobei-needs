//! Components and systems permitting to move and collide entities.

use std::{
    cmp::max,
    cmp::min,
    collections::{HashMap, HashSet},
    hash::{Hash, Hasher},
};

use bevy::{prelude::*, sprite::collide_aabb::collide};

use crate::constants::{GameState, STAGE};

/// Plugin managing contact collisions
pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<ContactEvent>()
            .register_type::<Position>()
            .register_type::<BoxCollider>()
            .init_resource::<ColliderViewers>()
            .on_state_update(STAGE, GameState::InGame, collision_system.system())
            .on_state_update(
                STAGE,
                GameState::InGame,
                add_collider_viewer_system.system(),
            )
            .on_state_update(
                STAGE,
                GameState::InGame,
                update_collider_viewers_system.system(),
            );
    }
}

/// Position
#[derive(Clone, Copy, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct Position(pub Vec3);

#[derive(Debug, Default, Reflect)]
#[reflect(Component)]
/// 2D Collider in a shape of a rectangle
pub struct BoxCollider {
    /// The width and height of the box.
    pub size: Vec2,
}

impl BoxCollider {
    /// Creates a box collider with the given size.
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            size: Vec2::new(x, y),
        }
    }
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
    commands: &mut Commands,
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

/// Adds to entities with a `SpritLoader` the related `SpriteBundle`.
fn add_collider_viewer_system(
    commands: &mut Commands,
    mut viewers: ResMut<ColliderViewers>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<(Entity, &BoxCollider, &Position), Changed<BoxCollider>>,
) {
    for (entity, box_collider, pos) in query.iter() {
        let color_handle = materials.add(Color::rgba(0.3, 1.0, 0.3, 0.3).into());

        viewers.0.entry(entity).or_insert_with(|| {
            let mut viewer_pos = Position(pos.0);
            viewer_pos.0.y = pos.0.y - 1.0; // Move the collision viewer forward

            commands
                .spawn((ColliderViewer, viewer_pos))
                .with_bundle(SpriteBundle {
                    material: color_handle,
                    sprite: Sprite::new(box_collider.size),
                    ..SpriteBundle::default()
                })
                .current_entity()
                .unwrap()
        });
    }
}

/// Adds to entities with a `SpritLoader` the related `SpriteBundle`.
fn update_collider_viewers_system(
    viewers: Res<ColliderViewers>,
    query: Query<(Entity, &BoxCollider, &Position), Changed<Position>>,
    mut viewer_query: Query<(&ColliderViewer, &mut Position)>,
) {
    for (entity, _, pos) in query.iter() {
        if let Some((_, mut viewer_pos)) = viewers
            .0
            .get(&entity)
            .and_then(|&viewer| viewer_query.get_mut(viewer).ok())
        {
            viewer_pos.0 = pos.0;
            viewer_pos.0.y = pos.0.y - 1.0; // Move the collision viewer forward
        }
    }
}
