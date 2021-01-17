//! Systems for displaying colliders and trigger areas in the screen.

use std::collections::HashMap;

use bevy::prelude::*;

use crate::{
    constants::{GameState, STAGE},
    drawing::UIObject,
};

use super::{BoxCollider, Position, TriggerArea};

/// Plugin managing contact collisions
pub struct DebugCollisionPlugin;

impl Plugin for DebugCollisionPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<CollisionMaterials>()
            .init_resource::<ColliderViewers>()
            .on_state_update(
                STAGE,
                GameState::InGame,
                add_collider_viewer_system.system(),
            )
            .on_state_update(
                STAGE,
                GameState::InGame,
                update_collider_viewers_system.system(),
            )
            .on_state_update(
                STAGE,
                GameState::InGame,
                refresh_collider_viewers_system.system(),
            );
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
struct DebugViewer;

/// Stores a map of collider viewers: `{ entity_with_collider => viewer_entities }`
#[derive(Default)]
struct ColliderViewers(HashMap<Entity, Vec<Entity>>);

/// Creates a viewer entity for all colliders and trigger areas without one.
fn add_collider_viewer_system(
    commands: &mut Commands,
    mut viewers: ResMut<ColliderViewers>,
    materials: ResMut<CollisionMaterials>,
    non_viewed_colliders: Query<(Entity, &BoxCollider, &Position), Without<ViewedCollider>>,
    non_viewed_trigger_areas: Query<(Entity, &TriggerArea, &Position), Without<ViewedTriggerArea>>,
) {
    for (entity, collider, pos) in non_viewed_colliders.iter() {
        commands.insert_one(entity, ViewedCollider);

        let viewer = spawn_viewer(
            commands,
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
        commands.insert_one(entity, ViewedTriggerArea);

        let viewer = spawn_viewer(
            commands,
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
        .spawn((DebugViewer, pos, UIObject))
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

/// Refresh the position of collision viewers when the player presses `D`.
fn refresh_collider_viewers_system(
    commands: &mut Commands,
    keyboard_input: Res<Input<KeyCode>>,
    all_viewers: Res<ColliderViewers>,
    collider_positions: Query<&Position, Without<DebugViewer>>,
    mut viewer_positions: Query<&mut Position, With<DebugViewer>>,
    box_colliders: Query<&BoxCollider>,
) {
    if keyboard_input.just_pressed(KeyCode::D) {
        for (collider, viewers) in &all_viewers.0 {
            for viewer in viewers {
                if let Ok(pos) = collider_positions.get(*collider) {
                    if let Ok(mut viewer_pos) = viewer_positions.get_mut(*viewer) {
                        let offset = box_colliders
                            .get(*collider)
                            .map(|col| col.offset)
                            .unwrap_or_default();

                        *viewer_pos = forwarded_position(pos.0 + offset);
                    }
                } else {
                    commands.despawn(*viewer);
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
