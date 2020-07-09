//! System managing changes in entity movement.

use crate::{controllers::Direction, physics::Velocity};
use crossbeam_channel::Receiver;
use legion::{prelude::*, storage::Tag};

/// System that updates the velocity of tagged entities toward incoming directions.
pub struct Movement<TTag: Tag> {
    /// Tag of the filtered entity
    pub tag: TTag,
    /// Subscription to directions changes
    pub directions: Receiver<Direction>,
    /// Speed used to update the velocity
    pub speed: f32,
}

impl<TTag: Tag> Movement<TTag> {
    /// Updates the velocity of the tagged entity
    pub fn update(&self, world: &mut World) {
        if let Some(direction) = self.directions.try_iter().last() {
            let new_velocity = self.speed * direction;

            Write::<Velocity>::query()
                .filter(tag_value(&self.tag))
                .iter_mut(world)
                .for_each(|mut velocity| *velocity = new_velocity)
        }
    }
}
