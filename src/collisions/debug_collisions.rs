//! Systems for displaying colliders and trigger areas in the screen.

use std::collections::HashMap;

use bevy::prelude::*;

use crate::{constants::GameState, drawing::UiObject};

use super::{BoxCollider, Position, TriggerArea};

/// Plugin for displaying colliders and trigger areas.
pub struct DebugCollisionPlugin;

impl Plugin for DebugCollisionPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<ColliderMaterials>()
            .init_resource::<ColliderViewers>()
            .add_system_set(
                SystemSet::on_update(GameState::InGame)
                    .with_system(add_collider_viewer_system.system())
                    .with_system(update_collider_viewers_system.system()),
            );
    }
}

/// Colors of the colliders.
struct ColliderMaterials {
    /// Debug color for the `BoxCollider`
    collider: Handle<ColorMaterial>,
    /// Debug color for the `TriggerArea`
    trigger_area: Handle<ColorMaterial>,
}

impl FromWorld for ColliderMaterials {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();

        Self {
            collider: materials.add(Color::rgba(0.3, 1.0, 0.3, 0.25).into()),
            trigger_area: materials.add(Color::rgba(0.3, 0.3, 1.0, 0.25).into()),
        }
    }
}

/// Component tagging an entity that there is a debugger view for its collider.  
struct ViewedCollider;
/// Component tagging an entity that there is a debugger view for its trigger area.  
struct ViewedTriggerArea;

/// Component tagging an entity as a collider viewer
struct DebugViewer;

/// Stores a map of collider viewers: `{ entity_with_collider => viewer_entities }`
#[derive(Default)]
struct ColliderViewers(HashMap<Entity, Vec<Entity>>);

/// Creates a viewer entity for all colliders and trigger areas without one.
fn add_collider_viewer_system(
    mut commands: Commands,
    mut viewers: ResMut<ColliderViewers>,
    materials: ResMut<ColliderMaterials>,
    non_viewed_colliders: Query<(Entity, &BoxCollider, &Position), Without<ViewedCollider>>,
    non_viewed_trigger_areas: Query<(Entity, &TriggerArea, &Position), Without<ViewedTriggerArea>>,
) {
    for (entity, collider, pos) in non_viewed_colliders.iter() {
        commands.entity(entity).insert(ViewedCollider);

        let viewer = spawn_viewer(
            &mut commands,
            forwarded_position(pos.0 + collider.offset),
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
        commands.entity(entity).insert(ViewedTriggerArea);

        let viewer = spawn_viewer(
            &mut commands,
            forwarded_position(pos.0),
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
        .spawn_bundle((DebugViewer, pos, UiObject))
        .insert_bundle(SpriteBundle {
            material: color,
            sprite: Sprite::new(size),
            ..SpriteBundle::default()
        })
        .id()
}

/// Query filter of a entity with a moved collider.
type MovedCollider = (
    Changed<Position>,
    Without<DebugViewer>,
    Or<(With<ViewedCollider>, With<ViewedTriggerArea>)>,
);

/// Adds to entities with a `SpritLoader` the related `SpriteBundle`.
fn update_collider_viewers_system(
    all_viewers: Res<ColliderViewers>,
    moved_colliders: Query<(Entity, &Position), MovedCollider>,
    box_colliders: Query<&BoxCollider>,
    mut viewer_query: Query<&mut Position, With<DebugViewer>>,
) {
    for (entity, pos) in moved_colliders.iter() {
        if let Some(viewers) = all_viewers.0.get(&entity) {
            for viewer in viewers {
                if let Ok(mut viewer_pos) = viewer_query.get_mut(*viewer) {
                    let offset = box_colliders
                        .get(entity)
                        .map(|col| col.offset)
                        .unwrap_or_default();

                    *viewer_pos = forwarded_position(pos.0 + offset);
                }
            }
        }
    }
}

/// Returns a the given position placed a little bit forward.
fn forwarded_position(pos: Vec3) -> Position {
    let mut new_pos = Position(pos);
    new_pos.0.y = pos.y - 1.0;
    new_pos.0.z = 0.0; // ignore the z position of the entity when colliding it
    new_pos
}
