//! The main game state

use crate::{constants::SPEED, controllers::Keyboard, physics, rendering};
use event::{Axis, GamepadId, KeyCode, KeyMods};
use ggez::event;
use ggez::graphics;
use ggez::{nalgebra::Vector2, timer, Context, GameResult};
use graphics::{DrawParam, Image};
use legion::prelude::*;
use physics::{Position, Velocity};
use rendering::Rendering;

/// Game state for the main game play.
pub struct GamePlay {
    /// ECS world containing entities.
    world: World,
    /// The keyboard the player is using.
    keyboard: Keyboard,
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
                    param: DrawParam::new().scale(Vector2::new(0.3, 0.3)),
                },
            )],
        );
        world.insert(
            (Entity::Baobei,),
            vec![(
                Position::new(300.0, 300.0),
                Rendering {
                    sprite: Image::new(ctx, "/baobei.png")?,
                    param: DrawParam::new().scale(Vector2::new(0.3, 0.3)),
                },
            )],
        );

        let state = Self {
            world,
            keyboard: Keyboard::new(),
        };

        Ok(state)
    }

    /// Updates velocity of Didi depending on the key pressed
    fn update_key_velocity(self: &mut Self) {
        let direction = self.keyboard.arrow_direction();
        self.update_didi_velocity(|_| direction * SPEED);
    }

    /// Updates Didi's velocity to the one returned by the closure
    fn update_didi_velocity(self: &mut Self, new_velocity: impl Fn(Velocity) -> Velocity) {
        let query = Write::<Velocity>::query().filter(tag_value(&Entity::Didi));

        for mut velocity in query.iter_mut(&mut self.world) {
            *velocity = new_velocity(*velocity);
        }
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
        self.keyboard.press_key(keycode);
        self.update_key_velocity();
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _: KeyMods) {
        self.keyboard.unpress_key(keycode);
        self.update_key_velocity();
    }

    fn gamepad_axis_event(&mut self, _ctx: &mut Context, axis: Axis, value: f32, _: GamepadId) {
        self.update_didi_velocity(|velocity| match axis {
            Axis::LeftStickX => Velocity::new(value * SPEED, velocity.y),
            Axis::LeftStickY => Velocity::new(velocity.x, -value * SPEED),
            _ => velocity,
        });
    }
}
