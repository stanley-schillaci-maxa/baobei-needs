//! System managing changes in entity movement.

use crate::{
    controllers::Direction,
    events::{EventsExt, Subscriber},
    physics::Velocity,
};

use legion::{prelude::*, storage::Tag};

/// Event appearing when a movement happen.
pub struct MovementEvent(pub Direction);

/// System that updates the velocity of tagged entities toward incoming directions.
pub struct MovementSystem<TTag: Tag> {
    /// Tag of the filtered entity
    tag: TTag,
    /// Speed used to update the velocity
    speed: f32,
    /// Subscription to movement events
    events: Subscriber<MovementEvent>,
}

impl<TTag: Tag> MovementSystem<TTag> {
    /// Creates a `MovementSystem` for moving the tagged entity.
    pub fn new(world: &mut World, tag: TTag, speed: f32) -> Self {
        Self {
            tag,
            speed,
            events: world.subscribe_events::<MovementEvent>(),
        }
    }

    /// Updates the velocity of the tagged entity
    pub fn update(&self, world: &mut World) {
        let new_velocity = self
            .events
            .iter_events(world)
            .last()
            .map(|movement| self.speed * movement.0);

        if let Some(new_velocity) = new_velocity {
            Write::<Velocity>::query()
                .filter(tag_value(&self.tag))
                .iter_mut(world)
                .for_each(|mut velocity| *velocity = new_velocity)
        }
    }
}
