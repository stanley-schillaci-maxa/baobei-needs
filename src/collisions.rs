//! Components and systems permitting to move and collide entities.

use legion::prelude::*;

use crate::events::Event;
use crate::physics::{Position, Velocity};
use ncollide2d::nalgebra::{Isometry2, Vector2};
use ncollide2d::query;
use ncollide2d::shape::Cuboid;
use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
    hash::{Hash, Hasher},
    iter::once,
};

/// Id tag of a entity that can be collided
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Id(pub usize);

/// Rectangle of
pub struct BoxCollider(pub Cuboid<f32>);

impl BoxCollider {
    /// Creates a `BoxCollider` with the given `width` and `height`.
    pub fn new(width: f32, height: f32) -> Self {
        Self(Cuboid::new(Vector2::new(width, height)))
    }
}

/// Represents a collision between two entities
#[derive(Clone, Copy, Debug, Eq)]
pub struct Collision(Id, Id);

impl PartialEq for Collision {
    fn eq(&self, other: &Self) -> bool {
        let Self(Id(a_1), Id(b_1)) = self;
        let Self(Id(a_2), Id(b_2)) = other;

        let same = a_1 == a_2 && b_1 == b_2;
        let inverted = a_1 == b_2 && b_1 == a_2;

        same || inverted
    }
}

impl Hash for Collision {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let Self(Id(a), Id(b)) = self;

        (min(a, b), max(a, b)).hash(state);
    }
}

/// Event appearing when entities collides.
#[derive(Clone, Debug, PartialEq)]
pub enum CollisionEvent {
    /// A collision is happening.
    Entering(Collision),
    /// A collision is finished.
    Exiting(Collision),
}

/// Emit collision events in the world.
///
/// For each new collision:
/// - emits event entities containing the entering collisions
/// - creates entities representing collisions
///
/// For each finished collision:
/// - emits event entities containing exiting collisions
/// - removes entities representing collisions
///
/// TODO: Refactor to be more maintainable
///
pub fn update(dt: f32, world: &mut World) {
    let moving_colliders = <(
        Read<Position>,
        Read<Velocity>,
        Read<BoxCollider>,
        Tagged<Id>,
    )>::query();

    let other_colliders = <(Read<Position>, Read<BoxCollider>, Tagged<Id>)>::query();

    let next_collisions: HashSet<_> = moving_colliders
        .iter(world)
        .flat_map(|(pos_1, vel_1, col_1, id_1)| {
            other_colliders
                .iter(world)
                .filter(move |(_, _, id_2)| id_1.0 != id_2.0)
                .filter_map(move |(pos_2, col_2, id_2)| {
                    let contact = query::contact(
                        &isometry_from(*pos_1 + (*vel_1 * dt)),
                        &col_1.0,
                        &isometry_from(*pos_2),
                        &col_2.0,
                        0.0,
                    );

                    if contact.is_some() {
                        Some(Collision(*id_1, *id_2))
                    } else {
                        None
                    }
                })
        })
        .collect();

    let prev_entities: HashMap<_, _> = Read::<Collision>::query()
        .iter_entities(world)
        .map(|(entity, col)| (*col, entity))
        .collect();

    let prev_collisions: HashSet<_> = prev_entities.keys().copied().collect();

    for &new_collision in next_collisions.difference(&prev_collisions) {
        dbg!(new_collision);
        world.insert((Event,), once((CollisionEvent::Entering(new_collision),)));
        world.insert((), once((new_collision,)));
    }

    for finished_collision in prev_collisions.difference(&next_collisions) {
        dbg!(finished_collision);

        world.insert(
            (Event,),
            once((CollisionEvent::Exiting(*finished_collision),)),
        );

        if let Some(&entity) = prev_entities.get(finished_collision) {
            world.delete(entity);
        }
    }
}

/// Converts a position to an isometry from the origin.
fn isometry_from(pos: Position) -> Isometry2<f32> {
    let translation = Vector2::new(pos.x, pos.y);
    Isometry2::new(translation, 0.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_publish_collision_events() {
        let mut world = Universe::new().create_world();

        let moving_entity = (
            Position::new(0.0, 0.0),
            Velocity::new(1.0, 0.0),
            BoxCollider::new(1.0, 1.0),
        );
        let collided_entity = (Position::new(2.0, 0.0), BoxCollider::new(1.0, 1.0));

        world.insert((Id(1),), vec![moving_entity]);
        world.insert((Id(2),), vec![collided_entity]);

        update(1.0, &mut world);

        let collisions: Vec<_> = Read::<Collision>::query()
            .iter(&world)
            .map(|col| *col)
            .collect();

        assert_eq!(collisions, vec![Collision(Id(1), Id(2))]);

        let query = Read::<CollisionEvent>::query();
        let mut collision_events = query.iter(&world);

        assert_eq!(
            *collision_events.next().unwrap(),
            CollisionEvent::Entering(Collision(Id(1), Id(2)))
        );
    }
}
