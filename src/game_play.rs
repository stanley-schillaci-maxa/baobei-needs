//! The main game state

use crate::{
    collisions::{self, BoxCollider, Id},
    constants::SPEED,
    controllers::{Direction, Gamepad, Keyboard},
    events,
    movement::Movement,
    physics,
    publisher::Publisher,
    rendering,
};
use event::{Axis, Button, GamepadId, KeyCode, KeyMods};
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
    /// The gamepad the player is using.
    gamepad: Gamepad,
    /// System that moves didi toward the input direction.
    didi_movement: Movement<Entity>,
    /// Publisher of movement events
    movement_publisher: Publisher<Direction>,
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
            (Entity::Didi, Id(1)),
            vec![(
                Position::origin(),
                Velocity::new(0.0, 0.0),
                BoxCollider::new(50.0, 20.0),
                Rendering {
                    sprite: Image::new(ctx, "/didi.png")?,
                    param: DrawParam::new().scale(Vector2::new(0.3, 0.3)),
                    order: 2,
                },
            )],
        );
        world.insert(
            (Entity::Baobei, Id(2)),
            vec![(
                Position::new(300.0, 300.0),
                BoxCollider::new(50.0, 20.0),
                Rendering {
                    sprite: Image::new(ctx, "/baobei.png")?,
                    param: DrawParam::new().scale(Vector2::new(0.3, 0.3)),
                    order: 1,
                },
            )],
        );

        let mut movement_publisher = Publisher::new();

        let didi_movement = Movement {
            tag: Entity::Didi,
            speed: SPEED,
            directions: movement_publisher.subscribe(),
        };

        let state = Self {
            world,
            keyboard: Keyboard::new(),
            gamepad: Gamepad::new(),
            didi_movement,
            movement_publisher,
        };

        Ok(state)
    }
}

impl event::EventHandler for GamePlay {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let world = &mut self.world;
        let dt = timer::delta(ctx).as_secs_f32();

        collisions::update(dt, world);
        physics::update(dt, world);
        rendering::update(world);
        self.didi_movement.update(world);

        events::clear_all(world);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);

        rendering::draw(ctx, &self.world)?;
        rendering::draw_colliders(ctx, &self.world)?;

        graphics::present(ctx)
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: KeyCode, _: KeyMods, _: bool) {
        if keycode == KeyCode::Escape {
            event::quit(ctx);
        }
        self.keyboard.press_key(keycode);
        self.movement_publisher
            .publish(self.keyboard.arrow_direction());
    }

    fn key_up_event(&mut self, _: &mut Context, keycode: KeyCode, _: KeyMods) {
        self.keyboard.release_key(keycode);
        self.movement_publisher
            .publish(self.keyboard.arrow_direction());
    }

    fn gamepad_axis_event(&mut self, _: &mut Context, axis: Axis, value: f32, _: GamepadId) {
        self.gamepad.move_axis(axis, value);
        self.movement_publisher.publish(self.gamepad.left_stick);
    }

    fn gamepad_button_down_event(&mut self, _: &mut Context, button: Button, _: GamepadId) {
        self.gamepad.press_button(button);
    }

    fn gamepad_button_up_event(&mut self, _: &mut Context, button: Button, _: GamepadId) {
        self.gamepad.release_button(button);
    }
}
