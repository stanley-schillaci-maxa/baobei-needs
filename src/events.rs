//! Types and helpers for interacting with appearing events.

use legion::prelude::*;

/// Static tag to assign event entities with.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Event;

/// Extends World with methods for managing event entities.
pub trait EventsExt {
    /// Removes all events entities from the world.
    fn clear_events(&mut self);
}

impl EventsExt for World {
    fn clear_events(&mut self) {
        let event_entities: Vec<_> = Tagged::<Event>::query()
            .iter_entities(self)
            .map(|(entity, _)| entity)
            .collect();

        for entity in event_entities {
            self.delete(entity);
        }
    }
}
