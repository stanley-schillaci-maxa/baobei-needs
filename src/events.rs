//! Types and helpers for interacting with appearing events.

use legion::prelude::*;

/// Static tag to assign event entities with.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Event;

/// Removes all events entities from the world.
pub fn clear_all(world: &mut World) {
    let event_entities: Vec<_> = Tagged::<Event>::query()
        .iter_entities(world)
        .map(|(entity, _)| entity)
        .collect();

    for entity in event_entities {
        world.delete(entity);
    }
}
