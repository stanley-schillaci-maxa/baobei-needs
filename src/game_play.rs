//! The main game state

use crate::{
    inputs::{update_velocity_gamepad_axis, update_velocity_key_down, update_velocity_key_up},
    physics, rendering,
};
use event::{Axis, GamepadId, KeyCode, KeyMods};
use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::{timer, Context, GameResult};
use graphics::{DrawParam, Image};
use legion::prelude::*;
use physics::{Position, Velocity};
use rendering::Rendering;

/// Game state for the main game play.
pub struct GamePlay {
    /// ECS world
    world: World,
}

/// Tags for the main entities.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Entity {
    /// The player.
    Didi,
    /// The baobei to take care of.
    Baobei,
}

impl GamePlay {
    /// Create the world with Didi and Baobei
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let mut world = Universe::new().create_world();

        world.insert(
            (Entity::Didi,),
            vec![(
                Position::origin(),
                Velocity::new(0.0, 0.0),
                Rendering {
                    sprite: Image::new(ctx, "/didi.png")?,
                    param: DrawParam::new().scale(na::Vector2::new(0.5, 0.5)),
                },
            )],
        );
        world.insert(
            (Entity::Baobei,),
            vec![(
                Position::new(300.0, 300.0),
                Rendering {
                    sprite: Image::new(ctx, "/baobei.png")?,
                    param: DrawParam::new().scale(na::Vector2::new(0.5, 0.5)),
                },
            )],
        );

        Ok(Self { world })
    }
}

impl event::EventHandler for GamePlay {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let world = &mut self.world;
        let dt = timer::delta(ctx).as_secs_f32();

        physics::update(dt, world);
        rendering::update(world);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);

        rendering::draw(ctx, &self.world)?;

        graphics::present(ctx)
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: KeyCode, _: KeyMods, _: bool) {
        if keycode == KeyCode::Escape {
            event::quit(ctx);
        }

        let query = Write::<Velocity>::query().filter(tag_value(&Entity::Didi));

        for mut velocity in query.iter_mut(&mut self.world) {
            update_velocity_key_down(velocity.as_mut(), keycode);
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _: KeyMods) {
        let query = Write::<Velocity>::query().filter(tag_value(&Entity::Didi));

        for mut velocity in query.iter_mut(&mut self.world) {
            update_velocity_key_up(velocity.as_mut(), keycode);
        }
    }

    fn gamepad_axis_event(&mut self, _ctx: &mut Context, axis: Axis, value: f32, _: GamepadId) {
        let query = Write::<Velocity>::query().filter(tag_value(&Entity::Didi));

        for mut velocity in query.iter_mut(&mut self.world) {
            update_velocity_gamepad_axis(velocity.as_mut(), axis, value);
        }
    }
}
