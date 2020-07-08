//! Components and systems permitting to move and collide entities.

use legion::prelude::*;

use ggez::nalgebra::{Point2, Vector2};

/// Position of an entity
pub type Position = Point2<f32>;

/// Velocity of an entity
pub type Velocity = Vector2<f32>;

/// Moves positions of entities depending on their velocity.
pub fn update(dt: f32, world: &mut World) {
    <(Write<Position>, Read<Velocity>)>::query()
        .iter_mut(world)
        .for_each(|(mut position, velocity)| *position += *velocity * dt)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_updates_position_using_velocity_and_delta_time() {
        let mut world = Universe::new().create_world();

        let entities = vec![
            (Position::new(0.0, 0.0), Velocity::new(0.0, 0.0)),
            (Position::new(1.0, 1.0), Velocity::new(1.0, 2.0)),
            (Position::new(2.0, 3.0), Velocity::new(2.0, 1.0)),
        ];
        world.insert((), entities);

        update(0.5, &mut world);

        // Assert updated positions
        let query = Read::<Position>::query();

        let mut pos = query.iter(&world);
        assert_eq!(*pos.next().unwrap(), Position::new(0.0, 0.0));
        assert_eq!(*pos.next().unwrap(), Position::new(1.5, 2.0));
        assert_eq!(*pos.next().unwrap(), Position::new(3.0, 3.5));
        assert_eq!(pos.next(), None);
    }
}
