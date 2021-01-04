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
            .on_state_update(STAGE, GameState::InGame, trigger_area_system.system())
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

/// Absolute position of the entity.
#[derive(Clone, Copy, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct Position(pub Vec3);

/// Delta of the next movement the entity will do move to.
#[derive(Default)]
pub struct Movement(pub Vec3);

/// Collider in a shape of a rectangle
#[derive(Clone, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct BoxCollider {
    /// The width and height of the box.
    pub size: Vec2,
}

impl BoxCollider {
    /// Creates a box collider with the given size.
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            size: Vec2::new(width, height),
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

/// Moves the position of moving entities depending on their movement.
/// If the entity collides with another collider, then it will be stopped.
pub fn collision_system(
    mut moving_colliders: Query<(&mut Position, &BoxCollider, &mut Movement)>,
    other_colliders: Query<(&Position, &BoxCollider), Without<Movement>>,
) {
    for (mut pos_a, col_a, mut mov_a) in moving_colliders.iter_mut() {
        let next_pos_a = pos_a.0 + mov_a.0;

        let no_collision = other_colliders
            .iter()
            .all(|(pos_b, col_b)| collide(next_pos_a, col_a.size, pos_b.0, col_b.size).is_none());

        if no_collision {
            pos_a.0 = next_pos_a;
        }
        *mov_a = Movement::default();
    }
}

/// Compares positions of box colliders with trigger areas and emit trigger
/// events.
pub fn trigger_area_system(
    commands: &mut Commands,
    mut contact_events: ResMut<Events<ContactEvent>>,
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

/// Component tagging an entity that there is a debugger view for its collider.  
struct ViewedCollider;
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
    non_viewed_colliders: Query<(Entity, &BoxCollider, &Position), Without<ViewedCollider>>,
) {
    for (entity, box_collider, pos) in non_viewed_colliders.iter() {
        let color_handle = materials.add(Color::rgba(0.3, 1.0, 0.3, 0.3).into());

        commands.insert_one(entity, ViewedCollider);

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

/// Query filter of a entity with a moved collider.
type MovedCollider = (Changed<Position>, With<ViewedCollider>);

/// Adds to entities with a `SpritLoader` the related `SpriteBundle`.
fn update_collider_viewers_system(
    viewers: Res<ColliderViewers>,
    moved_colliders: Query<(Entity, &BoxCollider, &Position), MovedCollider>,
    mut viewer_query: Query<(&ColliderViewer, &mut Position)>,
) {
    for (entity, _, pos) in moved_colliders.iter() {
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
