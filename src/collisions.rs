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
        app.init_resource::<CollisionMaterials>()
            .add_event::<ContactEvent>()
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

/// Colors of the button.
struct CollisionMaterials {
    /// Debug color for the `BoxCollider`
    collider: Handle<ColorMaterial>,
    /// Debug color for the `TriggerArea`
    trigger_area: Handle<ColorMaterial>,
}

impl FromResources for CollisionMaterials {
    fn from_resources(resources: &Resources) -> Self {
        let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
        Self {
            collider: materials.add(Color::rgba(0.3, 1.0, 0.3, 0.3).into()),
            trigger_area: materials.add(Color::rgba(0.3, 0.3, 1.0, 0.3).into()),
        }
    }
}

/// Component tagging an entity that there is a debugger view for its collider.  
struct ViewedCollider;
/// Component tagging an entity that there is a debugger view for its trigger area.  
struct ViewedTriggerArea;

/// Component tagging an entity as a collider viewer
struct Viewer;

/// Stores a map of collider viewers: `{ entity_with_collider => viewer_entities }`
#[derive(Default)]
struct ColliderViewers(HashMap<Entity, Vec<Entity>>);

/// Adds to entities with a `SpritLoader` the related `SpriteBundle`.
fn add_collider_viewer_system(
    commands: &mut Commands,
    mut viewers: ResMut<ColliderViewers>,
    materials: ResMut<CollisionMaterials>,
    non_viewed_colliders: Query<(Entity, &BoxCollider, &Position), Without<ViewedCollider>>,
    non_viewed_trigger_areas: Query<(Entity, &TriggerArea, &Position), Without<ViewedTriggerArea>>,
) {
    let viewer_pos_from = |pos: &Position| {
        let mut viewer_pos = Position(pos.0);
        viewer_pos.0.y = pos.0.y - 1.0; // Move the viewer forward
        viewer_pos
    };

    for (entity, collider, pos) in non_viewed_colliders.iter() {
        commands.insert_one(entity, ViewedCollider);

        let viewer = spawn_viewer(
            commands,
            viewer_pos_from(pos),
            collider.size,
            materials.collider.clone(),
        );

        viewers
            .0
            .entry(entity)
            .or_insert_with(Vec::new)
            .push(viewer);
    }
    for (entity, trigger_area, pos) in non_viewed_trigger_areas.iter() {
        commands.insert_one(entity, ViewedTriggerArea);

        let viewer = spawn_viewer(
            commands,
            viewer_pos_from(pos),
            trigger_area.size,
            materials.trigger_area.clone(),
        );

        viewers
            .0
            .entry(entity)
            .or_insert_with(Vec::new)
            .push(viewer);
    }
}

/// Spawns a viewer at the given position and size and returns the entity.  
fn spawn_viewer(
    commands: &mut Commands,
    pos: Position,
    size: Vec2,
    color: Handle<ColorMaterial>,
) -> Entity {
    commands
        .spawn((Viewer, pos))
        .with_bundle(SpriteBundle {
            material: color,
            sprite: Sprite::new(size),
            ..SpriteBundle::default()
        })
        .current_entity()
        .unwrap()
}

/// Query filter of a entity with a moved collider.
type MovedCollider = (
    Changed<Position>,
    Without<Viewer>,
    Or<(With<ViewedCollider>, With<ViewedTriggerArea>)>,
);

/// Adds to entities with a `SpritLoader` the related `SpriteBundle`.
fn update_collider_viewers_system(
    all_viewers: Res<ColliderViewers>,
    moved_colliders: Query<(Entity, &Position), MovedCollider>,
    mut viewer_query: Query<&mut Position, With<Viewer>>,
) {
    for (entity, pos) in moved_colliders.iter() {
        if let Some(viewers) = all_viewers.0.get(&entity) {
            for viewer in viewers {
                if let Ok(mut viewer_pos) = viewer_query.get_mut(*viewer) {
                    viewer_pos.0 = pos.0;
                    viewer_pos.0.y = pos.0.y - 1.0; // Move the collision viewer forward
                }
            }
        }
    }
}
