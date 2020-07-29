//! System managing changes in entity movement.

use crate::{controllers::Direction, physics::Velocity};
use crossbeam_channel::Receiver;
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
    events: Receiver<Event>,
}

impl<TTag: Tag> MovementSystem<TTag> {
    /// Creates a `MovementSystem` for moving the tagged entity.
    pub fn new(world: &mut World, tag: TTag, speed: f32) -> Self {
        let (sender, receiver) = crossbeam_channel::unbounded();
        world.subscribe(sender, component::<MovementEvent>());

        Self {
            tag,
            speed,
            events: receiver,
        }
    }

    /// Updates the velocity of the tagged entity
    pub fn update(&self, world: &mut World) {
        if let Some(Event::EntityInserted(entity, _)) = self.events.try_iter().last() {
            let new_velocity = match world.get_component::<MovementEvent>(entity) {
                Some(movement) => self.speed * movement.0,
                None => Velocity::new(0.0, 0.0),
            };

            Write::<Velocity>::query()
                .filter(tag_value(&self.tag))
                .iter_mut(world)
                .for_each(|mut velocity| *velocity = new_velocity)
        }
    }
}
